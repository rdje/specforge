#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(FB_CROAK decode encode);
use File::Basename qw(basename dirname);
use File::Copy qw(copy);
use File::Path qw(make_path remove_tree);
use File::Spec;
use FindBin qw($Bin);
use JSON::PP;

my $mode = 'check';
my $root = File::Spec->catdir($Bin, '..');
my $relative_contract = 'doctrine/live_document_size/source_pdf_registry.json';

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        @ARGV or die "--root requires a value\n";
        $root = shift @ARGV;
    } elsif ($arg eq '--contract') {
        @ARGV or die "--contract requires a value\n";
        $relative_contract = shift @ARGV;
    } else {
        die "Usage: $0 [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n";
    }
}

my $base = canonical_root($root);
die "source-pdf-registry-currentness: unsafe contract path\n"
    if !safe_relative_path($relative_contract, 512);

if ($mode eq 'self-test') {
    run_self_test($base, $relative_contract);
    print "source-pdf-registry-currentness: self-test 13/13 passed.\n";
    exit 0;
}

my $contract = load_json(absolute($base, $relative_contract));
my @errors = validate_contract($base, $contract, $relative_contract, undef);
if (@errors) {
    print STDERR "source-pdf-registry-currentness: $_\n" for @errors;
    exit 1;
}

if ($mode eq 'report') {
    my $registry = $contract->{registry};
    my $tracked = $contract->{tracked_corpus};
    my $producer = $contract->{derivation};
    print JSON::PP->new->canonical->encode({
        schema_version => $contract->{schema_version},
        owner => $contract->{owner},
        registry => {
            path => $registry->{path},
            rows => $registry->{row_count},
            lines => $registry->{lines},
            bytes => $registry->{bytes},
            line_bytes => $registry->{line_bytes},
            sha256 => $registry->{sha256},
        },
        tracked_corpus => {
            root => $tracked->{root},
            pdfs => $tracked->{file_count},
            git_index_is_authoritative => JSON::PP::true,
            host_local_or_generated_is_authoritative => JSON::PP::false,
        },
        derivation_regions => scalar(@{$producer->{regions}}),
        derivation_seams => scalar(@{$producer->{seams}}),
    }) . "\n";
} else {
    print "source-pdf-registry-currentness: 22 tracked PDFs, registry rows, derived keys, paths, signatures, and producer seams are current.\n";
}

sub validate_contract {
    my ($base, $contract, $relative_contract, $tracked_override) = @_;
    my @errors;
    if (ref($contract) ne 'HASH') {
        return ('contract root must be an object');
    }
    problem(\@errors, 'unsupported schema_version')
        if !defined($contract->{schema_version}) || $contract->{schema_version} != 1;
    problem(\@errors, 'owner differs from the task-owned authority')
        if ($contract->{owner} // '') ne 'LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g.ii';
    for my $name (qw(registry tracked_corpus derivation controls)) {
        problem(\@errors, "$name must be an object") if ref($contract->{$name}) ne 'HASH';
    }
    return @errors if @errors;

    my $registry = $contract->{registry};
    my $tracked = $contract->{tracked_corpus};
    my $derivation = $contract->{derivation};
    my $controls = $contract->{controls};

    for my $field (qw(max_contract_bytes max_registry_rows max_required_literals max_path_bytes max_class_bytes max_producer_regions max_producer_seams)) {
        problem(\@errors, "controls.$field must be a positive integer")
            if !positive_integer($controls->{$field});
    }
    return @errors if @errors;

    my $contract_path = absolute($base, $relative_contract);
    my $contract_raw = read_raw($contract_path);
    problem(\@errors, 'contract exceeds controls.max_contract_bytes')
        if length($contract_raw) > $controls->{max_contract_bytes};

    for my $field (qw(path h1 table_header table_separator sha256)) {
        problem(\@errors, "registry.$field must be a non-empty scalar")
            if !defined($registry->{$field}) || ref($registry->{$field}) || $registry->{$field} eq '';
    }
    for my $field (qw(lines bytes line_bytes row_count)) {
        problem(\@errors, "registry.$field must be a positive integer")
            if !positive_integer($registry->{$field});
    }
    problem(\@errors, 'registry.required_literals must be a non-empty array')
        if ref($registry->{required_literals}) ne 'ARRAY' || !@{$registry->{required_literals} // []};
    problem(\@errors, 'registry.required_literals exceeds its control bound')
        if ref($registry->{required_literals}) eq 'ARRAY'
        && @{$registry->{required_literals}} > $controls->{max_required_literals};
    return @errors if @errors;

    problem(\@errors, 'unsafe registry path')
        if !safe_relative_path($registry->{path}, $controls->{max_path_bytes});
    my $registry_path = absolute($base, $registry->{path});
    if (!-f $registry_path || -l $registry_path) {
        problem(\@errors, 'registry path is missing, non-regular, or a symlink');
        return @errors;
    }
    problem(\@errors, 'registry path escapes the repository root')
        if !canonical_path_is_within($base, $registry_path);

    my $registry_raw = read_raw($registry_path);
    compare_identity($registry_raw, $registry, 'registry', \@errors);
    my $registry_text = decode_utf8($registry_raw, 'registry');
    my @lines = split /\n/, $registry_text, -1;
    problem(\@errors, 'registry H1 differs') if ($lines[0] // '') ne $registry->{h1};
    for my $literal (@{$registry->{required_literals}}) {
        problem(\@errors, "registry required literal is invalid")
            if !defined($literal) || ref($literal) || $literal eq '';
        problem(\@errors, "registry required literal is missing: $literal")
            if defined($literal) && !ref($literal) && index($registry_text, $literal) < 0;
    }

    my ($rows, $parse_errors) = parse_registry($registry_text, $registry);
    push @errors, @$parse_errors;
    problem(\@errors, 'registry row count differs') if @$rows != $registry->{row_count};
    problem(\@errors, 'registry row count exceeds its control bound')
        if @$rows > $controls->{max_registry_rows};

    my $errors_before_tracked_shape = scalar @errors;
    for my $field (qw(root extension file_signature)) {
        problem(\@errors, "tracked_corpus.$field must be a non-empty scalar")
            if !defined($tracked->{$field}) || ref($tracked->{$field}) || $tracked->{$field} eq '';
    }
    problem(\@errors, 'tracked_corpus.file_count must be a positive integer')
        if !positive_integer($tracked->{file_count});
    problem(\@errors, 'tracked_corpus.git_index_is_authoritative must be true')
        if !json_true($tracked->{git_index_is_authoritative});
    problem(\@errors, 'tracked_corpus.host_local_or_generated_is_authoritative must be false')
        if !json_false($tracked->{host_local_or_generated_is_authoritative});
    return @errors if @errors > $errors_before_tracked_shape;

    problem(\@errors, 'unsafe tracked corpus root')
        if !safe_relative_path($tracked->{root}, $controls->{max_path_bytes});
    problem(\@errors, 'tracked corpus extension must be .pdf')
        if lc($tracked->{extension}) ne '.pdf';

    my @tracked_paths;
    if (defined $tracked_override) {
        @tracked_paths = @$tracked_override;
    } else {
        my ($paths, $git_error) = git_tracked_pdfs($base, $tracked->{root}, $tracked->{extension});
        problem(\@errors, $git_error) if $git_error ne '';
        @tracked_paths = @$paths;
    }
    @tracked_paths = sort @tracked_paths;
    problem(\@errors, 'tracked PDF count differs') if @tracked_paths != $tracked->{file_count};
    problem(\@errors, 'registry/tracked file count contract differs')
        if $registry->{row_count} != $tracked->{file_count};

    my %tracked_paths;
    for my $path (@tracked_paths) {
        problem(\@errors, "unsafe tracked PDF path: $path")
            if !safe_relative_path($path, $controls->{max_path_bytes});
        problem(\@errors, "tracked PDF is outside corpus root: $path")
            if index($path, "$tracked->{root}/") != 0;
        problem(\@errors, "tracked corpus member does not end in $tracked->{extension}: $path")
            if lc(substr($path, -length($tracked->{extension}))) ne lc($tracked->{extension});
        problem(\@errors, "duplicate tracked PDF path: $path") if $tracked_paths{$path}++;
    }

    my (%row_paths, %row_keys);
    for my $row (@$rows) {
        my $path = $row->{path};
        my $key = $row->{document_key};
        my $class = $row->{class};
        my $directory = $row->{directory};

        problem(\@errors, "unsafe registry path: $path")
            if !safe_relative_path($path, $controls->{max_path_bytes});
        problem(\@errors, "registry path is outside corpus root: $path")
            if index($path, "$tracked->{root}/") != 0;
        problem(\@errors, "registry class is empty or overlong for $path")
            if $class eq '' || utf8_bytes($class) > $controls->{max_class_bytes};
        problem(\@errors, "duplicate document_key: $key") if $row_keys{$key}++;
        problem(\@errors, "duplicate registry path: $path") if $row_paths{$path}++;
        problem(\@errors, "registry path is not a tracked corpus PDF: $path")
            if !$tracked_paths{$path};

        my $derived = derive_document_key($path, $tracked->{extension});
        problem(\@errors, "document_key differs from code-derived key for $path: $key != $derived")
            if $key ne $derived;

        my $expected_directory = corpus_directory($path, $tracked->{root});
        problem(\@errors, "corpus directory differs for $path: $directory != $expected_directory")
            if $directory ne $expected_directory;

        my $absolute_path = absolute($base, $path);
        if (!-f $absolute_path || -l $absolute_path) {
            problem(\@errors, "registered PDF is missing, non-regular, or a symlink: $path");
            next;
        }
        if (!canonical_path_is_within($base, $absolute_path)) {
            problem(\@errors, "registered PDF escapes the repository root: $path");
            next;
        }
        my $signature = read_prefix($absolute_path, utf8_bytes($tracked->{file_signature}));
        problem(\@errors, "registered file does not start with $tracked->{file_signature}: $path")
            if $signature ne $tracked->{file_signature};
    }
    for my $path (@tracked_paths) {
        problem(\@errors, "tracked PDF is missing a registry row: $path") if !$row_paths{$path};
    }

    problem(\@errors, 'derivation.source_path must be a safe repository-relative path')
        if !safe_relative_path($derivation->{source_path} // '', $controls->{max_path_bytes});
    problem(\@errors, 'derivation.regions must be a non-empty array')
        if ref($derivation->{regions}) ne 'ARRAY' || !@{$derivation->{regions} // []};
    problem(\@errors, 'derivation.seams must be a non-empty array')
        if ref($derivation->{seams}) ne 'ARRAY' || !@{$derivation->{seams} // []};
    problem(\@errors, 'derivation.regions exceeds its control bound')
        if ref($derivation->{regions}) eq 'ARRAY'
        && @{$derivation->{regions}} > $controls->{max_producer_regions};
    problem(\@errors, 'derivation.seams exceeds its control bound')
        if ref($derivation->{seams}) eq 'ARRAY'
        && @{$derivation->{seams}} > $controls->{max_producer_seams};
    return @errors if @errors;

    my $source_path = absolute($base, $derivation->{source_path});
    if (!-f $source_path || -l $source_path) {
        problem(\@errors, 'derivation source is missing, non-regular, or a symlink');
        return @errors;
    }
    my $source_raw = read_raw($source_path);
    my %region_names;
    for my $region (@{$derivation->{regions}}) {
        if (ref($region) ne 'HASH') {
            problem(\@errors, 'derivation region must be an object');
            next;
        }
        my $name = $region->{name} // '';
        my $signature = $region->{signature} // '';
        problem(\@errors, "duplicate or empty derivation region name: $name")
            if $name eq '' || $region_names{$name}++;
        my ($raw, $region_error) = extract_braced_region($source_raw, $signature);
        if ($region_error ne '') {
            problem(\@errors, "derivation region $name: $region_error");
            next;
        }
        compare_identity($raw, $region, "derivation region $name", \@errors);
    }
    for my $seam (@{$derivation->{seams}}) {
        if (ref($seam) ne 'HASH' || !defined($seam->{literal}) || ref($seam->{literal})
            || !positive_integer($seam->{occurrences})) {
            problem(\@errors, 'derivation seam is malformed');
            next;
        }
        my $actual = literal_occurrences($source_raw, $seam->{literal});
        problem(\@errors, "derivation seam occurrence differs: $seam->{literal}")
            if $actual != $seam->{occurrences};
    }

    return @errors;
}

sub parse_registry {
    my ($text, $registry) = @_;
    my @errors;
    my @lines = split /\n/, $text, -1;
    my @header_indices = grep { $lines[$_] eq $registry->{table_header} } 0 .. $#lines;
    if (@header_indices != 1) {
        push @errors, 'registry must contain exactly one declared table header';
        return ([], \@errors);
    }
    my $index = $header_indices[0];
    if (($lines[$index + 1] // '') ne $registry->{table_separator}) {
        push @errors, 'registry table separator differs';
        return ([], \@errors);
    }
    my @rows;
    for my $line_index ($index + 2 .. $#lines) {
        my $line = $lines[$line_index];
        last if $line !~ /^\|/;
        if ($line =~ /^\| `([^`]+)` \| ([^|]+?) \| `([^`]+)` \| `([^`]*)` \|$/) {
            my $class = $2;
            $class =~ s/^\s+|\s+$//g;
            push @rows, {
                document_key => $1,
                class => $class,
                path => $3,
                directory => $4,
                line => $line_index + 1,
            };
        } else {
            push @errors, 'malformed registry row at line ' . ($line_index + 1);
        }
    }
    return (\@rows, \@errors);
}

sub git_tracked_pdfs {
    my ($base, $corpus_root, $extension) = @_;
    open my $git, '-|', 'git', '-C', $base, 'ls-files', '-z', '--', $corpus_root
        or return ([], "cannot run git ls-files: $!");
    local $/;
    my $raw = <$git> // '';
    if (!close $git) {
        return ([], 'git ls-files failed');
    }
    my @paths;
    for my $raw_path (split /\0/, $raw) {
        next if $raw_path eq '';
        my $path;
        eval { $path = decode('UTF-8', $raw_path, FB_CROAK); 1 }
            or return ([], 'git ls-files emitted a non-UTF-8 path');
        next if length($path) < length($extension);
        next if lc(substr($path, -length($extension))) ne lc($extension);
        push @paths, $path;
    }
    return (\@paths, '');
}

sub derive_document_key {
    my ($path, $extension) = @_;
    my $name = basename($path);
    $name =~ s/\Q$extension\E\z//i
        or return 'source';
    my $key = '';
    my $last_was_separator = 0;
    for my $character (split //, $name) {
        if ($character =~ /[A-Za-z0-9]/) {
            $key .= lc($character);
            $last_was_separator = 0;
        } elsif (!$last_was_separator && $key ne '') {
            $key .= '_';
            $last_was_separator = 1;
        }
    }
    $key =~ s/^_+|_+$//g;
    return $key eq '' ? 'source' : $key;
}

sub corpus_directory {
    my ($path, $root) = @_;
    my $relative = $path;
    $relative =~ s/^\Q$root\E\///;
    $relative =~ s/[^\/]+\z//;
    return $relative;
}

sub extract_braced_region {
    my ($raw, $signature) = @_;
    return ('', 'signature is empty') if !defined($signature) || $signature eq '';
    my $count = literal_occurrences($raw, $signature);
    return ('', "signature occurrence count is $count") if $count != 1;
    my $start = index($raw, $signature);
    my $open = index($raw, '{', $start);
    return ('', 'opening brace is missing') if $open < 0;
    my $depth = 0;
    for my $position ($open .. length($raw) - 1) {
        my $character = substr($raw, $position, 1);
        $depth++ if $character eq '{';
        if ($character eq '}') {
            $depth--;
            return (substr($raw, $start, $position - $start + 1), '') if $depth == 0;
        }
    }
    return ('', 'closing brace is missing');
}

sub run_self_test {
    my ($base, $relative_contract) = @_;
    my $fixture_root = absolute($base, "generated/.source-pdf-registry-currentness-test.$$");
    my $guard = absolute($base, 'generated/.source-pdf-registry-currentness-test.');
    die "source-pdf-registry-currentness self-test: unsafe fixture root\n"
        if index($fixture_root, $guard) != 0;
    my $preclean_error = cleanup_fixture_root($fixture_root);
    die "source-pdf-registry-currentness self-test: pre-clean failed: $preclean_error\n"
        if $preclean_error ne '';
    make_path($fixture_root);

    my $source_contract = load_json(absolute($base, $relative_contract));
    my $registry_raw = read_raw(absolute($base, $source_contract->{registry}{path}));
    my ($source_rows, $source_errors) = parse_registry(
        decode_utf8($registry_raw, 'self-test source registry'),
        $source_contract->{registry},
    );
    die "source-pdf-registry-currentness self-test: cannot parse source registry: "
        . join('; ', @$source_errors) . "\n" if @$source_errors;
    my @source_tracked = map { $_->{path} } @$source_rows;

    my @cases = (
        ['valid tracked registry', 1, '', sub {}],
        ['registry identity drift', 0, 'registry bytes differs', sub {
            my ($fixture, $contract, $tracked) = @_;
            append_raw(absolute($fixture, $contract->{registry}{path}), "\nDRIFT\n");
        }],
        ['registry row omitted', 0, 'tracked PDF is missing a registry row', sub {
            my ($fixture, $contract, $tracked) = @_;
            remove_first_registry_row(absolute($fixture, $contract->{registry}{path}));
        }],
        ['tracked PDF omitted from registry', 0, 'tracked PDF is missing a registry row', sub {
            my ($fixture, $contract, $tracked) = @_;
            my $path = 'corpus/fixture/extra/Extra-Spec.pdf';
            write_raw(absolute($fixture, $path), "%PDF-1.7\n");
            push @$tracked, $path;
        }],
        ['registry path not tracked', 0, 'registry path is not a tracked corpus PDF', sub {
            my ($fixture, $contract, $tracked) = @_;
            shift @$tracked;
        }],
        ['document key drift', 0, 'document_key differs from code-derived key', sub {
            my ($fixture, $contract, $tracked) = @_;
            my $rows = fixture_rows($fixture, $contract);
            replace_once(
                absolute($fixture, $contract->{registry}{path}),
                "| `$rows->[0]{document_key}` |",
                '| `wrong_key` |',
            );
        }],
        ['duplicate document key', 0, 'duplicate document_key', sub {
            my ($fixture, $contract, $tracked) = @_;
            my $rows = fixture_rows($fixture, $contract);
            replace_once(
                absolute($fixture, $contract->{registry}{path}),
                "| `$rows->[1]{document_key}` |",
                "| `$rows->[0]{document_key}` |",
            );
        }],
        ['duplicate registry path', 0, 'duplicate registry path', sub {
            my ($fixture, $contract, $tracked) = @_;
            my $rows = fixture_rows($fixture, $contract);
            replace_once(
                absolute($fixture, $contract->{registry}{path}),
                "`$rows->[1]{path}`",
                "`$rows->[0]{path}`",
            );
        }],
        ['corpus directory drift', 0, 'corpus directory differs', sub {
            my ($fixture, $contract, $tracked) = @_;
            my $rows = fixture_rows($fixture, $contract);
            replace_once(
                absolute($fixture, $contract->{registry}{path}),
                "`$rows->[0]{directory}` |",
                '`wrong/directory/` |',
            );
        }],
        ['unsafe registry path', 0, 'unsafe registry path', sub {
            my ($fixture, $contract, $tracked) = @_;
            my $rows = fixture_rows($fixture, $contract);
            replace_once(
                absolute($fixture, $contract->{registry}{path}),
                "`$rows->[0]{path}`",
                '`../outside.pdf`',
            );
        }],
        ['invalid PDF signature', 0, 'registered file does not start with', sub {
            my ($fixture, $contract, $tracked) = @_;
            write_raw(absolute($fixture, $tracked->[0]), "NOT-A-PDF\n");
        }],
        ['producer derivation drift', 0, 'derivation region document_key', sub {
            my ($fixture, $contract, $tracked) = @_;
            replace_once(
                absolute($fixture, $contract->{derivation}{source_path}),
                'if ch.is_ascii_alphanumeric() {',
                'if ch.is_alphanumeric() {',
            );
        }],
        ['unknown schema', 0, 'unsupported schema_version', sub {
            my ($fixture, $contract, $tracked) = @_;
            $contract->{schema_version} = 2;
            write_contract($fixture, $relative_contract, $contract);
        }],
    );

    my $passed = 0;
    my $test_error = '';
    eval {
        for my $index (0 .. $#cases) {
            my ($name, $expected_ok, $expected_error, $mutate) = @{$cases[$index]};
            my $fixture = File::Spec->catdir($fixture_root, sprintf('case-%02d', $index + 1));
            seed_fixture($base, $fixture, $relative_contract, $source_contract, $source_rows);
            my $contract = load_json(absolute($fixture, $relative_contract));
            my @tracked = @source_tracked;
            $mutate->($fixture, $contract, \@tracked);
            $contract = load_json(absolute($fixture, $relative_contract));
            my @case_errors = validate_contract(
                $fixture,
                $contract,
                $relative_contract,
                \@tracked,
            );
            my $actual_ok = @case_errors ? 0 : 1;
            if ($actual_ok != $expected_ok) {
                die "source-pdf-registry-currentness self-test '$name' expected $expected_ok, got $actual_ok: "
                    . join('; ', @case_errors) . "\n";
            }
            if (!$expected_ok && index(join('; ', @case_errors), $expected_error) < 0) {
                die "source-pdf-registry-currentness self-test '$name' did not emit '$expected_error': "
                    . join('; ', @case_errors) . "\n";
            }
            $passed++;
        }
        1;
    } or $test_error = $@ || 'unknown self-test failure';

    my $cleanup_error = cleanup_fixture_root($fixture_root);
    die "source-pdf-registry-currentness self-test: cleanup failed: $cleanup_error\n"
        if $cleanup_error ne '';
    die $test_error if $test_error ne '';
    die "source-pdf-registry-currentness self-test: expected 13 cases, passed $passed\n"
        if $passed != 13;
}

sub seed_fixture {
    my ($base, $fixture, $relative_contract, $contract, $rows) = @_;
    for my $path (
        $relative_contract,
        $contract->{registry}{path},
        $contract->{derivation}{source_path},
    ) {
        my $destination = absolute($fixture, $path);
        make_path(dirname($destination));
        copy(absolute($base, $path), $destination)
            or die "source-pdf-registry-currentness self-test: copy $path failed: $!\n";
    }
    for my $row (@$rows) {
        write_raw(absolute($fixture, $row->{path}), "%PDF-1.7\n");
    }
}

sub fixture_rows {
    my ($fixture, $contract) = @_;
    my $raw = read_raw(absolute($fixture, $contract->{registry}{path}));
    my ($rows, $errors) = parse_registry(decode_utf8($raw, 'fixture registry'), $contract->{registry});
    die "source-pdf-registry-currentness self-test: fixture parse failed: "
        . join('; ', @$errors) . "\n" if @$errors;
    return $rows;
}

sub remove_first_registry_row {
    my ($path) = @_;
    my $raw = read_raw($path);
    my $count = ($raw =~ s/^\| `[^\n]+\n//m);
    die "source-pdf-registry-currentness self-test: row removal count is $count\n" if $count != 1;
    write_raw($path, $raw);
}

sub replace_once {
    my ($path, $from, $to) = @_;
    my $raw = read_raw($path);
    my $count = literal_occurrences($raw, $from);
    die "source-pdf-registry-currentness self-test: replacement '$from' count is $count\n"
        if $count != 1;
    substr($raw, index($raw, $from), length($from), $to);
    write_raw($path, $raw);
}

sub append_raw {
    my ($path, $suffix) = @_;
    open my $file, '>>:raw', $path
        or die "source-pdf-registry-currentness self-test: cannot append $path: $!\n";
    print {$file} $suffix;
    close $file or die "source-pdf-registry-currentness self-test: cannot close $path: $!\n";
}

sub write_contract {
    my ($base, $relative_contract, $contract) = @_;
    write_raw(
        absolute($base, $relative_contract),
        JSON::PP->new->canonical->pretty->encode($contract),
    );
}

sub cleanup_fixture_root {
    my ($fixture_root) = @_;
    return '' if !-e $fixture_root;
    my $errors;
    remove_tree($fixture_root, {error => \$errors});
    my @details;
    for my $entry (@{$errors // []}) {
        for my $path (sort keys %$entry) {
            push @details, "$path: $entry->{$path}";
        }
    }
    push @details, "$fixture_root still exists" if -e $fixture_root;
    return join('; ', @details);
}

sub compare_identity {
    my ($raw, $declared, $label, $errors) = @_;
    my $actual = metrics($raw);
    for my $field (qw(lines bytes line_bytes sha256)) {
        problem($errors, "$label $field differs")
            if !defined($declared->{$field}) || $actual->{$field} ne $declared->{$field};
    }
}

sub metrics {
    my ($raw) = @_;
    my $lines = () = $raw =~ /\n/g;
    my $line_bytes = 0;
    for my $line (split /\n/, $raw, -1) {
        $line_bytes = length($line) if length($line) > $line_bytes;
    }
    return {
        lines => $lines,
        bytes => length($raw),
        line_bytes => $line_bytes,
        sha256 => sha256_hex($raw),
    };
}

sub literal_occurrences {
    my ($haystack, $needle) = @_;
    return 0 if !defined($needle) || $needle eq '';
    my $count = 0;
    my $offset = 0;
    while (1) {
        my $position = index($haystack, $needle, $offset);
        last if $position < 0;
        $count++;
        $offset = $position + length($needle);
    }
    return $count;
}

sub canonical_root {
    my ($path) = @_;
    my $absolute = abs_path($path);
    die "source-pdf-registry-currentness: repository root does not exist: $path\n"
        if !defined($absolute) || !-d $absolute;
    return $absolute;
}

sub canonical_path_is_within {
    my ($base, $path) = @_;
    my $canonical = abs_path($path);
    return 0 if !defined($canonical);
    return $canonical eq $base || index($canonical, "$base/") == 0;
}

sub safe_relative_path {
    my ($path, $max_bytes) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '';
    return 0 if utf8_bytes($path) > $max_bytes;
    return 0 if File::Spec->file_name_is_absolute($path) || $path =~ /[\\\r\n\0`|]/;
    my @parts = split m{/}, $path, -1;
    return 0 if grep { $_ eq '' || $_ eq '.' || $_ eq '..' } @parts;
    return 1;
}

sub absolute {
    my ($base, $relative) = @_;
    return File::Spec->catfile($base, split m{/}, $relative);
}

sub load_json {
    my ($path) = @_;
    my $raw = read_raw($path);
    return JSON::PP->new->utf8->decode($raw);
}

sub read_raw {
    my ($path) = @_;
    open my $file, '<:raw', $path
        or die "source-pdf-registry-currentness: cannot read $path: $!\n";
    local $/;
    my $raw = <$file> // '';
    close $file or die "source-pdf-registry-currentness: cannot close $path: $!\n";
    return $raw;
}

sub read_prefix {
    my ($path, $length) = @_;
    open my $file, '<:raw', $path
        or die "source-pdf-registry-currentness: cannot read $path: $!\n";
    my $raw = '';
    my $read = read($file, $raw, $length);
    close $file or die "source-pdf-registry-currentness: cannot close $path: $!\n";
    return defined($read) ? $raw : '';
}

sub write_raw {
    my ($path, $raw) = @_;
    make_path(dirname($path));
    open my $file, '>:raw', $path
        or die "source-pdf-registry-currentness self-test: cannot write $path: $!\n";
    print {$file} $raw;
    close $file or die "source-pdf-registry-currentness self-test: cannot close $path: $!\n";
}

sub decode_utf8 {
    my ($raw, $label) = @_;
    my $decoded;
    eval { $decoded = decode('UTF-8', $raw, FB_CROAK); 1 }
        or die "source-pdf-registry-currentness: $label is not valid UTF-8\n";
    return $decoded;
}

sub utf8_bytes {
    my ($value) = @_;
    return length(encode('UTF-8', $value));
}

sub positive_integer {
    my ($value) = @_;
    return defined($value) && !ref($value) && $value =~ /\A[1-9][0-9]*\z/;
}

sub json_true {
    my ($value) = @_;
    return defined($value) && ref($value) && "$value" eq '1';
}

sub json_false {
    my ($value) = @_;
    return defined($value) && ref($value) && "$value" eq '0';
}

sub problem {
    my ($errors, $message) = @_;
    push @$errors, $message;
}
