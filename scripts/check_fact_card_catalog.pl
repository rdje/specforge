#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(decode_utf8 encode_utf8);
use Fcntl qw(O_CREAT O_EXCL O_WRONLY);
use File::Basename qw(dirname);
use File::Path qw(make_path remove_tree);
use File::Spec;
use FindBin qw($Bin);
use IO::Select;
use IPC::Open3;
use JSON::PP;
use Symbol qw(gensym);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my %VALID_STATUS = map { $_ => 1 } qw(current superseded deprecated);
my $DEFAULT_CONTRACT = 'doctrine/live_document_size/fact_card_catalog.json';
my $mode = 'check';
my $root;
my $contract_rel = $DEFAULT_CONTRACT;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--write') {
        $mode = 'write';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--print-plan') {
        $mode = 'print-plan';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--contract') {
        $contract_rel = shift @ARGV // usage();
    } else {
        usage();
    }
}

my $project_root = abs_path(File::Spec->catdir($Bin, '..'))
    // die "fact-card-catalog: cannot resolve repository root\n";
if ($mode eq 'self-test') {
    run_self_test($project_root);
    exit 0;
}

$root //= $project_root;
$root = abs_path($root) // die "fact-card-catalog: root does not exist\n";

if ($mode eq 'print-plan') {
    my ($errors, $context) = prepare_context($root, $contract_rel, 1);
    fail($errors) if @$errors;
    print JSON::PP->new->canonical(1)->pretty(1)->encode(
        planned_records($context->{projection}),
    );
    exit 0;
}

if ($mode eq 'write') {
    my ($errors, undef, $context) = validate_tree($root, $contract_rel, 1);
    fail($errors) if @$errors;
    write_projection($root, $context);
}

my ($errors, $result) = validate_tree($root, $contract_rel, 0);
fail($errors) if @$errors;
print STDERR "fact-card-catalog: warning: $_\n" for @{$result->{warnings}};
if ($mode eq 'report') {
    print JSON::PP->new->canonical(1)->encode($result), "\n";
} else {
    print "fact-card-catalog: $result->{migration_state} projection is valid for "
        . "$result->{card_count} canonical cards.\n";
}
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--contract PATH] "
        . "[--check|--write|--report|--print-plan|--self-test]\n";
}

sub fail {
    my ($errors) = @_;
    print STDERR "fact-card-catalog: $_\n" for @$errors;
    print STDERR "fact-card-catalog: FAILED with ", scalar(@$errors), " violation(s).\n";
    exit 1;
}

sub fixed_limits {
    return {
        max_cards => 336,
        cards_per_part => 56,
        max_parts => 6,
        max_id_bytes => 64,
        max_source_title_bytes => 1_024,
        max_title_cell_bytes => 112,
        max_answers => 64,
        max_answer_bytes => 2_048,
        max_legacy_row_bytes => 320,
        max_migrated_row_bytes => 333,
        landing => {
            health_targets => {lines => 224, bytes => 32_768, line_bytes => 256},
            enforcement_ceilings => {lines => 256, bytes => 32_768, line_bytes => 320},
        },
        # Aggregates are the file bound times the per-file bound (ADR 0029): a collection that may
        # never be deleted or rolled over must never refuse a corpus whose every file is legal.
        title_parts => {
            health_targets => {
                files => 6, lines_each => 80, bytes_each => 24_576,
                lines_total => 480, bytes_total => 147_456, line_bytes_each => 384,
            },
            enforcement_ceilings => {
                files => 6, lines_each => 96, bytes_each => 32_768,
                lines_total => 576, bytes_total => 196_608, line_bytes_each => 512,
            },
        },
        projection_ceiling => {files => 7, lines => 832, bytes => 229_376, line_bytes => 512},
    };
}

sub planned_output_limit {
    return fixed_limits()->{max_parts} + 1;
}

sub raw_scalar {
    my ($value) = @_;
    return '' if !defined $value;
    return utf8::is_utf8($value) ? encode_utf8($value) : $value;
}

sub safe_relative_path {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '';
    return 0 if $path =~ m{\A/} || $path =~ /\\/ || $path =~ /[\x00-\x1f\x7f]/;
    return 0 if $path =~ m{(?:\A|/)\.\.?(/|\z)} || $path =~ m{//};
    return 1;
}

sub absolute {
    my ($base, $relative) = @_;
    return File::Spec->catfile($base, split m{/}, $relative);
}

sub canonical_json {
    my ($value) = @_;
    return JSON::PP->new->canonical(1)->encode($value);
}

sub metrics {
    my ($raw) = @_;
    $raw = raw_scalar($raw);
    my $bytes = length($raw);
    my $lines = () = $raw =~ /\n/g;
    $lines++ if $bytes && $raw !~ /\n\z/;
    my $line_bytes = 0;
    for my $line (split /\n/, $raw, -1) {
        $line =~ s/\r\z//;
        $line_bytes = length($line) if length($line) > $line_bytes;
    }
    return {lines => $lines, bytes => $bytes, line_bytes => $line_bytes};
}

sub read_regular {
    my ($base, $relative, $label, $errors) = @_;
    if (!safe_relative_path($relative)) {
        push @$errors, "$label path is absolute, escaping, or malformed";
        return;
    }
    my $path = absolute($base, $relative);
    if (!-f $path || -l $path) {
        push @$errors, "$label is missing or not a regular non-symlink file: $relative";
        return;
    }
    my $root_device = (stat($base))[0];
    my $path_device = (stat($path))[0];
    push @$errors, "$label is off the repository volume: $relative"
        if !defined($root_device) || !defined($path_device) || $root_device != $path_device;
    open my $fh, '<:raw', $path or do {
        push @$errors, "cannot read $label '$relative': $!";
        return;
    };
    local $/;
    my $raw = <$fh> // '';
    close $fh or push @$errors, "cannot close $label '$relative': $!";
    return $raw;
}

sub read_json {
    my ($base, $relative, $label, $errors) = @_;
    my $raw = read_regular($base, $relative, $label, $errors);
    return if !defined $raw;
    my $value = eval { JSON::PP->new->utf8(1)->decode($raw) };
    if ($@ || ref($value) ne 'HASH') {
        push @$errors, "$label is not one valid JSON object";
        return;
    }
    return $value;
}

sub reject_unknown {
    my ($object, $label, $errors, @allowed) = @_;
    if (ref($object) ne 'HASH') {
        push @$errors, "$label must be an object";
        return;
    }
    my %allowed = map { $_ => 1 } @allowed;
    push @$errors, "$label has unknown field '$_'"
        for grep { !$allowed{$_} } sort keys %$object;
}

sub required_scalar {
    my ($object, $field, $label, $errors) = @_;
    if (ref($object) ne 'HASH' || !defined($object->{$field}) || ref($object->{$field})
        || $object->{$field} eq '') {
        push @$errors, "$label lacks non-empty scalar '$field'";
        return;
    }
    return $object->{$field};
}

sub required_positive_integer {
    my ($object, $field, $label, $errors) = @_;
    my $value = ref($object) eq 'HASH' ? $object->{$field} : undef;
    if (!defined($value) || ref($value) || $value !~ /\A\d+\z/ || $value < 1) {
        push @$errors, "$label lacks positive integer '$field'";
        return;
    }
    return 0 + $value;
}

sub validate_contract {
    my ($contract, $errors, $skip_planned) = @_;
    reject_unknown(
        $contract, 'contract', $errors,
        qw(schema_version migration_state surface_id part_surface_id paths legacy limits planned_outputs verifier),
    );
    push @$errors, 'schema_version must be 1'
        if !defined($contract->{schema_version}) || ref($contract->{schema_version})
        || $contract->{schema_version} != 1;
    my $state = required_scalar($contract, 'migration_state', 'contract', $errors) // '';
    push @$errors, "migration_state must be legacy_locked or migrated"
        if $state ne 'legacy_locked' && $state ne 'migrated';
    required_scalar($contract, 'surface_id', 'contract', $errors);
    required_scalar($contract, 'part_surface_id', 'contract', $errors);
    my $verifier = required_scalar($contract, 'verifier', 'contract', $errors) // '';
    push @$errors, 'verifier must name the stable fact-card checker command'
        if $verifier ne 'perl scripts/check_fact_card_catalog.pl --check';

    my $paths = $contract->{paths};
    reject_unknown(
        $paths, 'paths', $errors,
        qw(card_directory collection_readme landing part_directory part_prefix surface_registry question_contract),
    );
    my %path;
    $path{$_} = required_scalar($paths, $_, 'paths', $errors)
        for qw(card_directory collection_readme landing part_directory part_prefix surface_registry question_contract);
    for my $field (qw(card_directory collection_readme landing part_directory surface_registry question_contract)) {
        next if !defined $path{$field};
        push @$errors, "paths.$field is unsafe" if !safe_relative_path($path{$field});
    }
    if (defined $path{part_prefix}) {
        push @$errors, 'paths.part_prefix must be a lowercase safe filename prefix ending in hyphen'
            if $path{part_prefix} !~ /\A[a-z0-9][a-z0-9-]*-\z/;
    }
    if (defined($path{card_directory}) && defined($path{collection_readme})) {
        push @$errors, 'collection_readme must be the README directly inside card_directory'
            if $path{collection_readme} ne "$path{card_directory}/README.md";
    }
    if (defined($path{card_directory}) && defined($path{landing})) {
        push @$errors, 'landing must be INDEX.md directly inside card_directory'
            if $path{landing} ne "$path{card_directory}/INDEX.md";
    }
    push @$errors, 'card_directory and part_directory must be distinct'
        if defined($path{card_directory}) && defined($path{part_directory})
        && $path{card_directory} eq $path{part_directory};

    my $legacy = $contract->{legacy};
    reject_unknown(
        $legacy, 'legacy', $errors,
        qw(boundary_commit git_blob sha256 metrics card_count row_sha256),
    );
    for my $field (qw(boundary_commit git_blob sha256 row_sha256)) {
        my $value = required_scalar($legacy, $field, 'legacy', $errors);
        next if !defined $value;
        my $pattern = ($field eq 'boundary_commit' || $field eq 'git_blob')
            ? qr/\A(?:[0-9a-f]{40}|[0-9a-f]{64})\z/
            : qr/\A[0-9a-f]{64}\z/;
        push @$errors, "legacy.$field is not a lowercase object/digest id" if $value !~ $pattern;
    }
    required_positive_integer($legacy, 'card_count', 'legacy', $errors);
    validate_metrics_object($legacy->{metrics}, 'legacy.metrics', $errors);

    if (canonical_json($contract->{limits}) ne canonical_json(fixed_limits())) {
        push @$errors, 'limits differ from the fixed ADR 0020/0021/0022 profile';
    }
    validate_planned_schema($contract->{planned_outputs}, $errors) if !$skip_planned;
    return \%path;
}

sub validate_metrics_object {
    my ($object, $label, $errors) = @_;
    reject_unknown($object, $label, $errors, qw(lines bytes line_bytes));
    required_positive_integer($object, $_, $label, $errors) for qw(lines bytes line_bytes);
}

sub validate_planned_schema {
    my ($records, $errors) = @_;
    my $limit = planned_output_limit();
    if (ref($records) ne 'ARRAY' || !@$records || @$records > $limit) {
        push @$errors, "planned_outputs must contain one to $limit output records";
        return;
    }
    my %seen;
    for my $index (0 .. $#$records) {
        my $record = $records->[$index];
        my $label = "planned_outputs[$index]";
        reject_unknown($record, $label, $errors, qw(path role sha256 metrics));
        my $path = required_scalar($record, 'path', $label, $errors);
        my $role = required_scalar($record, 'role', $label, $errors);
        my $sha = required_scalar($record, 'sha256', $label, $errors);
        push @$errors, "$label path is unsafe" if defined($path) && !safe_relative_path($path);
        push @$errors, "$label has duplicate path '$path'" if defined($path) && $seen{$path}++;
        push @$errors, "$label role must be landing or title_part"
            if defined($role) && $role ne 'landing' && $role ne 'title_part';
        push @$errors, "$label sha256 is invalid"
            if defined($sha) && $sha !~ /\A[0-9a-f]{64}\z/;
        validate_metrics_object($record->{metrics}, "$label.metrics", $errors);
    }
}

sub validate_external_authorities {
    my ($base, $contract, $paths, $limits, $errors) = @_;
    my $record_slots;
    my $registry_raw = read_regular($base, $paths->{surface_registry}, 'surface registry', $errors);
    if (defined $registry_raw) {
        my @records;
        my $line_number = 0;
        for my $line (split /\n/, $registry_raw) {
            $line_number++;
            next if $line =~ /^\s*\z/;
            my $record = eval { JSON::PP->new->utf8(1)->decode($line) };
            if ($@ || ref($record) ne 'HASH') {
                push @$errors, "surface registry line $line_number is not one JSON object";
                next;
            }
            push @records, $record;
        }
        my @matches = grep {
            ($_->{surface_id} // '') eq ($contract->{surface_id} // '')
        } @records;
        if (@matches != 1) {
            push @$errors, "surface registry must contain surface_id '$contract->{surface_id}' exactly once";
        } else {
            my $surface = $matches[0];
            my $target = "$paths->{card_directory}/*.md";
            push @$errors, 'knowledge-card surface targets disagree with card_directory'
                if canonical_json($surface->{targets}) ne canonical_json([$target]);
            push @$errors, 'knowledge-card surface must remain partitioned_canonical/normal'
                if ($surface->{lifecycle} // '') ne 'partitioned_canonical'
                || ($surface->{state} // '') ne 'normal';
            push @$errors, 'knowledge-card surface index disagrees with landing'
                if ($surface->{index} // '') ne $paths->{landing};
            push @$errors, 'knowledge-card surface verifier disagrees with catalog checker'
                if ($surface->{verifier} // '') ne 'scripts/check_fact_card_catalog.pl';
            my $index_contract = ref($surface->{index_contract}) eq 'HASH'
                ? $surface->{index_contract} : {};
            push @$errors, 'knowledge-card surface index verifier must remain builtin:markdown_links'
                if ($index_contract->{verifier} // '') ne 'builtin:markdown_links';
            if (($contract->{migration_state} // '') eq 'migrated') {
                push @$errors, 'knowledge-card surface must route membership through the title parts'
                    if ($index_contract->{kind} // '') ne 'routed_membership'
                    || ($index_contract->{route_surface} // '') ne ($contract->{part_surface_id} // '');
            } else {
                push @$errors, 'knowledge-card surface must retain direct membership links'
                    if ($index_contract->{kind} // '') ne 'membership'
                    || exists $index_contract->{route_surface};
            }
            my $health_files = ref($surface->{health_targets}) eq 'HASH'
                ? $surface->{health_targets}{files} : undef;
            my $ceiling_files = ref($surface->{enforcement_ceilings}) eq 'HASH'
                ? $surface->{enforcement_ceilings}{files} : undef;
            push @$errors, 'knowledge-card surface file health/ceiling must remain 338'
                if !defined($health_files) || !defined($ceiling_files)
                || $health_files != 338 || $ceiling_files != 338;
            push @$errors, 'knowledge-card surface milestones must remain warning 80 / rollover 90'
                if ref($surface->{milestones}) ne 'HASH'
                || ($surface->{milestones}{warning_pct} // -1) != 80
                || ($surface->{milestones}{rollover_pct} // -1) != 90;
            push @$errors, 'max_cards is not derived from surface files minus README and INDEX'
                if defined($ceiling_files) && $limits->{max_cards} != $ceiling_files - 2;
        }

        my @part_matches = grep {
            ($_->{surface_id} // '') eq ($contract->{part_surface_id} // '')
        } @records;
        if (($contract->{migration_state} // '') eq 'legacy_locked') {
            push @$errors, "title-part surface '$contract->{part_surface_id}' must be absent while legacy_locked"
                if @part_matches;
        } elsif (@part_matches != 1) {
            push @$errors, "surface registry must contain title-part surface '$contract->{part_surface_id}' exactly once";
        } else {
            validate_part_surface($part_matches[0], $contract, $paths, $limits, $errors);
        }

        # The second fact writer. Every decision record but the collection index may carry an
        # `answers:` block, so its file ceiling is a fact-capacity authority too (ADR 0029).
        my @record_matches = grep { ($_->{surface_id} // '') eq 'decision_records' } @records;
        if (@record_matches != 1) {
            push @$errors, "surface registry must contain surface 'decision_records' exactly once";
        } else {
            my $files = ref($record_matches[0]{enforcement_ceilings}) eq 'HASH'
                ? $record_matches[0]{enforcement_ceilings}{files} : undef;
            if (!defined($files) || ref($files) || $files !~ /\A[1-9][0-9]*\z/ || $files < 2) {
                push @$errors, 'decision-record surface must declare a file ceiling above its index';
            } else {
                $record_slots = $files - 1;
            }
        }
    }

    my $question = read_json($base, $paths->{question_contract}, 'question projection contract', $errors);
    if (defined $question) {
        push @$errors, 'question projection fact_catalog disagrees with landing'
            if ($question->{fact_catalog} // '') ne $paths->{landing};
        my $max_facts = ref($question->{limits}) eq 'HASH' ? $question->{limits}{max_facts} : undef;
        my $derived_facts = defined($record_slots) ? $limits->{max_cards} + $record_slots : undef;
        push @$errors, 'question projection max_facts must fund every card slot plus every '
            . 'answers-bearing decision record'
            if !defined($max_facts) || ref($max_facts)
            || !defined($derived_facts) || $max_facts != $derived_facts;
    }
}

sub validate_part_surface {
    my ($surface, $contract, $paths, $limits, $errors) = @_;
    my $target = "$paths->{part_directory}/$paths->{part_prefix}*.md";
    push @$errors, 'title-part surface targets disagree with generated namespace'
        if canonical_json($surface->{targets}) ne canonical_json([$target]);
    push @$errors, 'title-part surface must be collection/generated_projection/normal'
        if ($surface->{locator} // '') ne 'collection'
        || ($surface->{lifecycle} // '') ne 'generated_projection'
        || ($surface->{state} // '') ne 'normal';
    push @$errors, 'title-part surface owner must remain knowledge-maintainers'
        if ($surface->{owner} // '') ne 'knowledge-maintainers';
    push @$errors, 'title-part surface health targets differ from the fixed profile'
        if canonical_json($surface->{health_targets})
        ne canonical_json($limits->{title_parts}{health_targets});
    push @$errors, 'title-part surface ceilings differ from the fixed profile'
        if canonical_json($surface->{enforcement_ceilings})
        ne canonical_json($limits->{title_parts}{enforcement_ceilings});
    push @$errors, 'title-part surface milestones must remain warning 80 / rollover 90'
        if ref($surface->{milestones}) ne 'HASH'
        || ($surface->{milestones}{warning_pct} // -1) != 80
        || ($surface->{milestones}{rollover_pct} // -1) != 90;
    push @$errors, 'title-part surface verifier must remain builtin:budget'
        if ($surface->{verifier} // '') ne 'builtin:budget';
    push @$errors, 'title-part surface canonical inputs disagree with canonical cards'
        if canonical_json($surface->{canonical_inputs})
        ne canonical_json(["$paths->{card_directory}/*.md"]);
    push @$errors, 'title-part surface freshness verifier disagrees with catalog checker'
        if ($surface->{freshness_verifier} // '') ne 'scripts/check_fact_card_catalog.pl';
    push @$errors, 'title-part surface index disagrees with stable landing'
        if ($surface->{index} // '') ne $paths->{landing};
    push @$errors, 'title-part surface must use external Markdown membership'
        if ref($surface->{index_contract}) ne 'HASH'
        || ($surface->{index_contract}{kind} // '') ne 'external_membership'
        || ($surface->{index_contract}{verifier} // '') ne 'builtin:markdown_links';
}

sub collect_cards {
    my ($base, $paths, $limits, $errors) = @_;
    my $relative = $paths->{card_directory};
    return [] if !defined $relative || !safe_relative_path($relative);
    my $directory = absolute($base, $relative);
    if (!-d $directory || -l $directory) {
        push @$errors, "card_directory is missing or not a regular directory: $relative";
        return [];
    }
    my $root_device = (stat($base))[0];
    my $directory_device = (stat($directory))[0];
    push @$errors, "card_directory is off the repository volume: $relative"
        if !defined($root_device) || !defined($directory_device) || $root_device != $directory_device;
    opendir my $dh, $directory or do {
        push @$errors, "cannot open card_directory '$relative': $!";
        return [];
    };
    my @names = sort grep { /\.md\z/ && $_ ne 'README.md' && $_ ne 'INDEX.md' } readdir $dh;
    closedir $dh or push @$errors, "cannot close card_directory '$relative': $!";
    push @$errors, 'catalog has no canonical fact cards' if !@names;
    push @$errors, "catalog has " . scalar(@names) . " cards; limit is $limits->{max_cards}"
        if @names > $limits->{max_cards};

    my @cards;
    for my $name (@names) {
        if ($name !~ /\A[a-z0-9][a-z0-9-]*\.md\z/) {
            push @$errors, "unsafe or reserved fact-card filename '$name'";
            next;
        }
        my $card_rel = "$relative/$name";
        my $raw = read_regular($base, $card_rel, "fact card '$name'", $errors);
        next if !defined $raw;
        my $card = eval { parse_card($name, decode_utf8($raw, 1), $limits) };
        if ($@ || !defined $card) {
            my $message = $@ || 'unknown parse failure';
            $message =~ s/\s+\z//;
            push @$errors, $message;
            next;
        }
        $card->{path} = $card_rel;
        push @cards, $card;
    }
    return \@cards;
}

sub parse_card {
    my ($name, $text, $limits) = @_;
    my ($front_matter) = $text =~ /\A---\r?\n(.*?)\r?\n---(?:\r?\n|\z)/s;
    die "$name lacks leading YAML front matter\n" if !defined $front_matter;

    my (%scalar, @answers, %seen);
    my $current_key = '';
    for my $line (split /\r?\n/, $front_matter) {
        if ($line =~ /^[ \t]+-[ \t]+(.+)\z/) {
            my $value = clean_scalar($1);
            if ($current_key eq 'answers') {
                die "$name has an empty answer\n" if $value eq '';
                push @answers, $value;
            }
            next;
        }
        next if $line =~ /^\s*\z/;
        my ($key, $rest) = $line =~ /\A([A-Za-z_][A-Za-z0-9_]*):[ \t]*(.*)\z/;
        die "$name has unsupported front-matter syntax: $line\n" if !defined $key;
        die "$name repeats front-matter key '$key'\n" if $seen{$key}++;
        $current_key = $key;
        if ($rest =~ /^\[(.*)\]\z/) {
            if ($key eq 'answers') {
                push @answers, map { clean_scalar($_) } split /,/, $1, -1;
            }
            $current_key = '';
        } elsif ($rest ne '') {
            $scalar{$key} = clean_scalar($rest);
            $current_key = '';
        }
    }

    my ($expected_id) = $name =~ /\A(.+)\.md\z/;
    my $id = $scalar{id} // '';
    die "$name lacks non-empty id\n" if $id eq '';
    die "$name id '$id' does not match filename '$expected_id'\n" if $id ne $expected_id;
    die "$name id exceeds $limits->{max_id_bytes} UTF-8 bytes\n"
        if length(encode_utf8($id)) > $limits->{max_id_bytes};

    my $title = $scalar{title} // '';
    die "$name lacks non-empty title\n" if $title eq '';
    die "$name title exceeds $limits->{max_source_title_bytes} UTF-8 bytes\n"
        if length(encode_utf8($title)) > $limits->{max_source_title_bytes};
    my $date = $scalar{date} // '';
    die "$name date must be YYYY-MM-DD\n"
        if $date !~ /\A\d{4}-(?:0[1-9]|1[0-2])-(?:0[1-9]|[12]\d|3[01])\z/;
    my $status = $scalar{status} // 'current';
    die "$name has unknown status '$status'\n" if !$VALID_STATUS{$status};

    @answers = grep { $_ ne '' } @answers;
    die "$name lacks a non-empty answers list\n" if !@answers;
    die "$name has " . scalar(@answers) . " answers; limit is $limits->{max_answers}\n"
        if @answers > $limits->{max_answers};
    for my $answer (@answers) {
        die "$name answer exceeds $limits->{max_answer_bytes} UTF-8 bytes\n"
            if length(encode_utf8($answer)) > $limits->{max_answer_bytes};
    }
    die "$name lacks evidence or reverify\n"
        if ($scalar{evidence} // '') eq '' && ($scalar{reverify} // '') eq '';
    return {id => $id, title => $title, date => $date, status => $status, name => $name};
}

sub clean_scalar {
    my ($value) = @_;
    $value =~ s/^[ \t]+//;
    $value =~ s/[ \t]+\z//;
    if ($value =~ /\A"(.*)"\z/s || $value =~ /\A'(.*)'\z/s) {
        $value = $1;
    }
    return $value;
}

sub compact_title {
    my ($title, $limit) = @_;
    $title =~ s/\\/\\\\/g;
    $title =~ s/\|/\\|/g;
    return $title if length(encode_utf8($title)) <= $limit;
    my $content_limit = $limit - length(encode_utf8('…'));
    my $short = '';
    for my $character (split //, $title) {
        last if length(encode_utf8($short . $character)) > $content_limit;
        $short .= $character;
    }
    $short =~ s/\s+\z//;
    return $short . '…';
}

sub legacy_row {
    my ($card, $limits) = @_;
    my $title = compact_title($card->{title}, $limits->{max_title_cell_bytes});
    return '| [' . $card->{id} . '](' . $card->{name} . ') | '
        . $card->{date} . ' | `' . $card->{status} . '` | ' . $title . ' |';
}

sub migrated_row {
    my ($card, $limits, $part_path) = @_;
    my $title = compact_title($card->{title}, $limits->{max_title_cell_bytes});
    my $target = relative_link($part_path, $card->{path});
    return '| [' . $card->{id} . '](' . $target . ') | '
        . $card->{date} . ' | `' . $card->{status} . '` | ' . $title . ' |';
}

sub render_legacy {
    my ($cards, $limits, $errors) = @_;
    my @lines = (
        '# Knowledge fact-card catalog',
        '',
        '> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with',
        '> `perl scripts/check_fact_card_catalog.pl --write`.',
        '',
        'This bounded human route lists every immediate fact card once. Search by question in the',
        '[generated Knowledge Map](../../KNOWLEDGE_MAP.md); browse durable rationale in the',
        '[decision index](../decisions/INDEX.md). Full titles, questions, evidence, and prose remain',
        'canonical in the linked cards.',
        '',
        'Collection guidance: [authoring and lifecycle README](README.md).',
        '',
        '| Fact | Established | Status | Title |',
        '| --- | --- | --- | --- |',
    );
    for my $card (@$cards) {
        my $row = legacy_row($card, $limits);
        push @$errors, "$card->{name} legacy row exceeds $limits->{max_legacy_row_bytes} UTF-8 bytes"
            if length(encode_utf8($row)) > $limits->{max_legacy_row_bytes};
        push @lines, $row;
    }
    return raw_scalar(join("\n", @lines) . "\n");
}

sub part_path {
    my ($paths, $number) = @_;
    return sprintf('%s/%s%04d.md', $paths->{part_directory}, $paths->{part_prefix}, $number);
}

sub render_projection {
    my ($cards, $paths, $limits, $errors) = @_;
    push @$errors, "card count exceeds migrated maximum $limits->{max_cards}"
        if @$cards > $limits->{max_cards};
    my $part_count = int((@$cards + $limits->{cards_per_part} - 1) / $limits->{cards_per_part});
    push @$errors, "projection requires $part_count parts; limit is $limits->{max_parts}"
        if $part_count > $limits->{max_parts};

    my @range_rows;
    for my $index (0 .. $part_count - 1) {
        my $start = $index * $limits->{cards_per_part};
        my $end = $start + $limits->{cards_per_part} - 1;
        $end = $#$cards if $end > $#$cards;
        my $path = part_path($paths, $index + 1);
        my $relative = relative_link($paths->{landing}, $path);
        push @range_rows, sprintf(
            '| [%04d](%s) | %d | `%s` | `%s` |',
            $index + 1, $relative, $end - $start + 1,
            $cards->[$start]{id}, $cards->[$end]{id},
        );
    }
    my @root = (
        '# Knowledge fact-card catalog',
        '> **AUTO-GENERATED — DO NOT EDIT.** Run `perl scripts/check_fact_card_catalog.pl --write`. '
            . '[README](README.md) · [questions](../../KNOWLEDGE_MAP.md) · '
            . '[decisions](../decisions/INDEX.md).',
        sprintf(
            '> %d fact card%s route through %d title part%s; find an id inside a range, then open that part.',
            scalar(@$cards), @$cards == 1 ? '' : 's',
            $part_count, $part_count == 1 ? '' : 's',
        ),
        '',
        '| Part | Cards | First id | Last id |',
        '| --- | ---: | --- | --- |',
        @range_rows,
    );

    my @outputs = ({path => $paths->{landing}, role => 'landing', raw => raw_scalar(join("\n", @root) . "\n")});
    for my $index (0 .. $part_count - 1) {
        my $start = $index * $limits->{cards_per_part};
        my $end = $start + $limits->{cards_per_part} - 1;
        $end = $#$cards if $end > $#$cards;
        my $output_path = part_path($paths, $index + 1);
        my @part = (
            sprintf('# Knowledge fact-card titles — part %04d', $index + 1),
            '',
            '> **AUTO-GENERATED — DO NOT EDIT.** Canonical facts remain in `docs/knowledge/`.',
            '> Return to the [bounded fact-card landing](' . relative_link($output_path, $paths->{landing}) . ').',
            '',
            '| Fact | Established | Status | Title |',
            '| --- | --- | --- | --- |',
        );
        for my $card (@$cards[$start .. $end]) {
            my $row = migrated_row($card, $limits, $output_path);
            push @$errors, "$card->{name} migrated row exceeds $limits->{max_migrated_row_bytes} UTF-8 bytes"
                if length(encode_utf8($row)) > $limits->{max_migrated_row_bytes};
            push @part, $row;
        }
        push @outputs, {
            path => $output_path,
            role => 'title_part',
            raw => raw_scalar(join("\n", @part) . "\n"),
        };
    }
    validate_projection_metrics(\@outputs, $limits, $errors);
    validate_expected_links(\@outputs, $cards, $paths, $errors);
    validate_landing_ranges($outputs[0]{raw}, $cards, $limits, $part_count, $errors);
    return \@outputs;
}

# Re-read the rendered router and prove it is range-complete against the canonical card list:
# one ordered row per title part, per-part counts that sum to the catalog, and inclusive
# first/last ids. This is what replaces the removed per-card landing line as the browse proof.
sub validate_landing_ranges {
    my ($raw, $cards, $limits, $part_count, $errors) = @_;
    my @rows;
    while ($raw =~ /^\| \[(\d{4})\]\([^)\r\n]+\) \| (\d+) \| `([^`\r\n]+)` \| `([^`\r\n]+)` \|$/mg) {
        push @rows, {number => $1, count => $2, first => $3, last => $4};
    }
    if (@rows != $part_count) {
        push @$errors, 'planned landing must carry exactly one range row per title part';
        return;
    }
    my $covered = 0;
    for my $index (0 .. $#rows) {
        my $row = $rows[$index];
        my $number = $index + 1;
        my $start = $index * $limits->{cards_per_part};
        my $end = $start + $limits->{cards_per_part} - 1;
        $end = $#$cards if $end > $#$cards;
        push @$errors, "planned landing range row $number is out of part order"
            if $row->{number} != $number;
        push @$errors, "planned landing range row $number miscounts its title part"
            if $row->{count} != $end - $start + 1;
        push @$errors, "planned landing range row $number names the wrong first id"
            if $row->{first} ne $cards->[$start]{id};
        push @$errors, "planned landing range row $number names the wrong last id"
            if $row->{last} ne $cards->[$end]{id};
        $covered += $row->{count};
    }
    push @$errors, 'planned landing ranges do not cover every card exactly once'
        if $covered != scalar(@$cards);
}

sub validate_projection_metrics {
    my ($outputs, $limits, $errors) = @_;
    return if !@$outputs;
    my $landing_metrics = metrics($outputs->[0]{raw});
    enforce_metrics(
        $landing_metrics,
        $limits->{landing}{enforcement_ceilings},
        'planned landing', $errors,
    );
    my @parts = @$outputs[1 .. $#$outputs];
    my $aggregate = aggregate_metrics(\@parts);
    enforce_collection_metrics(
        $aggregate,
        $limits->{title_parts}{enforcement_ceilings},
        'planned title parts', $errors,
    );
    my $projection = aggregate_metrics($outputs);
    my $ceiling = $limits->{projection_ceiling};
    for my $dimension (qw(files lines bytes line_bytes)) {
        push @$errors, "planned projection exceeds $dimension ceiling $ceiling->{$dimension}"
            if $projection->{$dimension} > $ceiling->{$dimension};
    }
}

sub aggregate_metrics {
    my ($outputs) = @_;
    my %result = (
        files => scalar(@$outputs), lines => 0, bytes => 0, line_bytes => 0,
        lines_each => 0, bytes_each => 0, line_bytes_each => 0,
        lines_total => 0, bytes_total => 0,
    );
    for my $output (@$outputs) {
        my $item = metrics($output->{raw});
        $result{lines} += $item->{lines};
        $result{bytes} += $item->{bytes};
        $result{line_bytes} = $item->{line_bytes} if $item->{line_bytes} > $result{line_bytes};
        $result{lines_each} = $item->{lines} if $item->{lines} > $result{lines_each};
        $result{bytes_each} = $item->{bytes} if $item->{bytes} > $result{bytes_each};
        $result{line_bytes_each} = $item->{line_bytes}
            if $item->{line_bytes} > $result{line_bytes_each};
        $result{lines_total} += $item->{lines};
        $result{bytes_total} += $item->{bytes};
    }
    return \%result;
}

sub enforce_metrics {
    my ($actual, $limits, $label, $errors) = @_;
    for my $dimension (qw(lines bytes line_bytes)) {
        push @$errors, "$label exceeds $dimension ceiling $limits->{$dimension}"
            if $actual->{$dimension} > $limits->{$dimension};
    }
}

sub enforce_collection_metrics {
    my ($actual, $limits, $label, $errors) = @_;
    for my $dimension (qw(files lines_each bytes_each lines_total bytes_total line_bytes_each)) {
        push @$errors, "$label exceeds $dimension ceiling $limits->{$dimension}"
            if $actual->{$dimension} > $limits->{$dimension};
    }
}

sub relative_link {
    my ($source_path, $target_path) = @_;
    my @source = split m{/}, dirname($source_path);
    my @target = split m{/}, $target_path;
    while (@source && @target && $source[0] eq $target[0]) {
        shift @source;
        shift @target;
    }
    return join '/', (('..') x scalar(@source), @target);
}

sub normalize_link {
    my ($source_path, $target) = @_;
    return if !defined($target) || $target eq '' || $target =~ /[\x00-\x1f\x7f\\]/;
    return if $target =~ m{\A(?:[a-z][a-z0-9+.-]*:|/|#)}i;
    $target =~ s/[?#].*\z//;
    my @parts = (split(m{/}, dirname($source_path)), split(m{/}, $target));
    my @normalized;
    for my $part (@parts) {
        next if $part eq '' || $part eq '.';
        if ($part eq '..') {
            return if !@normalized;
            pop @normalized;
        } else {
            push @normalized, $part;
        }
    }
    return join '/', @normalized;
}

sub resolved_links {
    my ($raw, $source_path, $label, $errors) = @_;
    my %counts;
    while ($raw =~ /\[[^\]\r\n]*\]\(([^)\r\n]+)\)/g) {
        my $resolved = normalize_link($source_path, $1);
        if (!defined($resolved) || !safe_relative_path($resolved)) {
            push @$errors, "$label has unsafe local link '$1'";
            next;
        }
        $counts{$resolved}++;
    }
    return \%counts;
}

sub validate_expected_links {
    my ($outputs, $cards, $paths, $errors) = @_;
    return if !@$outputs;
    my $root_links = resolved_links($outputs->[0]{raw}, $outputs->[0]{path}, 'planned landing', $errors);
    push @$errors, 'planned landing must link collection README exactly once'
        if ($root_links->{$paths->{collection_readme}} // 0) != 1;
    for my $card (@$cards) {
        push @$errors, "planned landing must route card '$card->{path}' through a title part, not link it"
            if $root_links->{$card->{path}};
    }
    for my $output (@$outputs[1 .. $#$outputs]) {
        push @$errors, "planned landing must link title part '$output->{path}' exactly once"
            if ($root_links->{$output->{path}} // 0) != 1;
    }
    my %routed;
    my $part_index = 0;
    for my $output (@$outputs[1 .. $#$outputs]) {
        my $links = resolved_links($output->{raw}, $output->{path}, "planned title part", $errors);
        my $start = $part_index * fixed_limits()->{cards_per_part};
        my $end = $start + fixed_limits()->{cards_per_part} - 1;
        $end = $#$cards if $end > $#$cards;
        for my $card (@$cards[$start .. $end]) {
            push @$errors, "planned title part '$output->{path}' must resolve card '$card->{path}' exactly once"
                if ($links->{$card->{path}} // 0) != 1;
        }
        $routed{$_} += $links->{$_} // 0 for map { $_->{path} } @$cards;
        $part_index++;
    }
    for my $card (@$cards) {
        push @$errors, "card '$card->{path}' is not reachable exactly once through the landing's title parts"
            if ($routed{$card->{path}} // 0) != 1;
    }
}

sub planned_records {
    my ($outputs) = @_;
    return [map {
        {
            path => $_->{path}, role => $_->{role},
            sha256 => sha256_hex($_->{raw}), metrics => metrics($_->{raw}),
        }
    } @$outputs];
}

sub compare_planned_records {
    my ($actual, $expected, $errors) = @_;
    return if ref($expected) ne 'ARRAY' || !@$expected;
    push @$errors, 'planned_outputs membership/order differs from deterministic rendering'
        if canonical_json($actual) ne canonical_json($expected);
}

sub legacy_rows {
    my ($raw) = @_;
    my @rows = $raw =~ /^(\| \[[^\r\n]+)$/mg;
    return \@rows;
}

sub row_digest {
    my ($rows) = @_;
    return sha256_hex(raw_scalar(join('', map { $_ . "\n" } @$rows)));
}

sub command_capture {
    my (@command) = @_;
    my ($stdin, $stdout);
    my $stderr = gensym;
    my $pid = eval { open3($stdin, $stdout, $stderr, @command) };
    return ('', 127, $@ || 'cannot execute command') if !$pid;
    close $stdin;
    my $selector = IO::Select->new($stdout, $stderr);
    my ($out, $err) = ('', '');
    while (my @ready = $selector->can_read) {
        for my $handle (@ready) {
            my $buffer = '';
            my $count = sysread($handle, $buffer, 8192);
            if (!defined($count) || $count == 0) {
                $selector->remove($handle);
                next;
            }
            if (fileno($handle) == fileno($stdout)) {
                $out .= $buffer;
            } else {
                $err .= $buffer;
            }
        }
    }
    waitpid($pid, 0);
    return ($out, $? >> 8, $err);
}

sub validate_git_boundary {
    my ($base, $contract, $paths, $errors) = @_;
    my $legacy = $contract->{legacy};
    my ($top, $top_exit, $top_err) = command_capture('git', '-C', $base, 'rev-parse', '--show-toplevel');
    if ($top_exit) {
        push @$errors, "cannot resolve Git root: $top_err";
        return;
    }
    $top =~ s/[\r\n]+\z//;
    my $resolved_top = abs_path($top) // $top;
    push @$errors, 'validation root is not the Git repository root' if $resolved_top ne $base;
    return if !defined($legacy->{boundary_commit}) || !defined($paths->{landing});

    my (undef, $commit_exit) = command_capture(
        'git', '-C', $base, 'cat-file', '-e', "$legacy->{boundary_commit}^{commit}",
    );
    if ($commit_exit) {
        push @$errors, 'legacy boundary commit lookup failed';
        return;
    }
    my (undef, $ancestor_exit) = command_capture(
        'git', '-C', $base, 'merge-base', '--is-ancestor', $legacy->{boundary_commit}, 'HEAD',
    );
    push @$errors, 'legacy boundary commit is not an ancestor of HEAD' if $ancestor_exit;
    my ($blob, $blob_exit, $blob_err) = command_capture(
        'git', '-C', $base, 'rev-parse', "$legacy->{boundary_commit}:$paths->{landing}",
    );
    if ($blob_exit) {
        push @$errors, "legacy boundary blob lookup failed: $blob_err";
        return;
    }
    $blob =~ s/[\r\n]+\z//;
    push @$errors, "legacy boundary blob '$blob' differs from declared '$legacy->{git_blob}'"
        if defined($legacy->{git_blob}) && $blob ne $legacy->{git_blob};
    my ($raw, $raw_exit, $raw_err) = command_capture('git', '-C', $base, 'cat-file', 'blob', $blob);
    if ($raw_exit) {
        push @$errors, "cannot read legacy boundary blob: $raw_err";
        return;
    }
    push @$errors, 'legacy boundary SHA-256 differs from contract'
        if defined($legacy->{sha256}) && sha256_hex($raw) ne $legacy->{sha256};
    compare_metrics(metrics($raw), $legacy->{metrics}, 'legacy boundary', $errors);
    my $rows = legacy_rows($raw);
    push @$errors, "legacy boundary row count is " . scalar(@$rows)
        . ", expected $legacy->{card_count}"
        if defined($legacy->{card_count}) && @$rows != $legacy->{card_count};
    push @$errors, 'legacy boundary row digest differs from contract'
        if defined($legacy->{row_sha256}) && row_digest($rows) ne $legacy->{row_sha256};
    return $raw;
}

sub compare_metrics {
    my ($actual, $expected, $label, $errors) = @_;
    return if ref($expected) ne 'HASH';
    for my $dimension (qw(lines bytes line_bytes)) {
        next if !defined $expected->{$dimension};
        push @$errors, "$label $dimension is $actual->{$dimension}, expected $expected->{$dimension}"
            if $actual->{$dimension} != $expected->{$dimension};
    }
}

sub validate_stage_zero {
    my ($base, $path, $expected_blob, $errors) = @_;
    my ($output, $exit, $stderr) = command_capture('git', '-C', $base, 'ls-files', '--stage', '--', $path);
    if ($exit) {
        push @$errors, "cannot inspect stage-zero landing: $stderr";
        return;
    }
    my @rows = grep { $_ ne '' } split /\r?\n/, $output;
    if (@rows != 1 || $rows[0] !~ /\A\d+ ([0-9a-f]{40}|[0-9a-f]{64}) 0\t\Q$path\E\z/) {
        push @$errors, 'landing must have exactly one stage-zero Git index entry';
        return;
    }
    push @$errors, "stage-zero landing blob '$1' differs from legacy '$expected_blob'"
        if defined($expected_blob) && $1 ne $expected_blob;
}

sub prepare_context {
    my ($base, $contract_path, $skip_planned) = @_;
    my @errors;
    if (!safe_relative_path($contract_path)) {
        push @errors, 'contract path is unsafe';
        return (\@errors, {});
    }
    my $contract = read_json($base, $contract_path, 'contract', \@errors);
    return (\@errors, {}) if !defined $contract;
    my $paths = validate_contract($contract, \@errors, $skip_planned);
    my $limits = fixed_limits();
    validate_external_authorities($base, $contract, $paths, $limits, \@errors);
    read_regular($base, $paths->{collection_readme}, 'collection README', \@errors)
        if defined $paths->{collection_readme};
    my $cards = collect_cards($base, $paths, $limits, \@errors);
    my $legacy = render_legacy($cards, $limits, \@errors);
    my $projection = render_projection($cards, $paths, $limits, \@errors);
    my $planned = planned_records($projection);
    compare_planned_records($planned, $contract->{planned_outputs}, \@errors) if !$skip_planned;
    return (\@errors, {
        contract => $contract, paths => $paths, limits => $limits,
        cards => $cards, legacy => $legacy, projection => $projection, planned => $planned,
    });
}

sub validate_tree {
    my ($base, $contract_path, $skip_outputs) = @_;
    my ($errors, $context) = prepare_context($base, $contract_path, 0);
    my @errors = @$errors;
    my $contract = $context->{contract};
    return (\@errors, {warnings => []}, $context) if ref($contract) ne 'HASH';
    my $paths = $context->{paths};
    my $boundary_raw = validate_git_boundary($base, $contract, $paths, \@errors);
    my $state = $contract->{migration_state} // '';
    my %result = (
        migration_state => $state,
        card_count => scalar(@{$context->{cards} // []}),
        planned_outputs => $context->{planned} // [],
        warnings => [],
    );

    if ($state eq 'legacy_locked') {
        validate_stage_zero($base, $paths->{landing}, $contract->{legacy}{git_blob}, \@errors)
            if defined $paths->{landing};
        my $current = read_regular($base, $paths->{landing}, 'legacy landing', \@errors);
        if (defined $current) {
            push @errors, 'working legacy landing differs from committed boundary'
                if defined($boundary_raw) && $current ne $boundary_raw;
            push @errors, 'working legacy landing differs from canonical card rendering'
                if defined($context->{legacy}) && $current ne $context->{legacy};
        }
        if (defined($paths->{part_directory}) && -e absolute($base, $paths->{part_directory})) {
            push @errors, "premature title-part directory exists: $paths->{part_directory}";
        }
    } elsif ($state eq 'migrated') {
        validate_migrated_outputs($base, $context, \@errors, $skip_outputs);
    }

    my $landing_metrics = metrics($context->{projection}[0]{raw});
    my ($landing_pressure_errors, $landing_pressure_warnings) = pressure_findings(
        $landing_metrics, $context->{limits}{landing}{health_targets}, 'planned landing',
    );
    push @errors, @$landing_pressure_errors;
    push @{$result{warnings}}, @$landing_pressure_warnings;
    my @parts = @{$context->{projection}}[1 .. $#{$context->{projection}}];
    my ($part_pressure_errors, $part_pressure_warnings) = collection_pressure_findings(
        aggregate_metrics(\@parts), $context->{limits}{title_parts}{health_targets}, 'planned title parts',
    );
    push @errors, @$part_pressure_errors;
    push @{$result{warnings}}, @$part_pressure_warnings;
    return (\@errors, \%result, $context);
}

sub validate_migrated_outputs {
    my ($base, $context, $errors, $skip_outputs) = @_;
    my $paths = $context->{paths};
    my %expected = map { $_->{path} => $_ } @{$context->{projection}};
    my $directory = absolute($base, $paths->{part_directory});
    if (!-e $directory) {
        push @$errors, "title-part directory is missing: $paths->{part_directory}" if !$skip_outputs;
    } elsif (!-d $directory || -l $directory) {
        push @$errors, "title-part directory is not a regular directory: $paths->{part_directory}";
    } else {
        opendir my $dh, $directory or push @$errors, "cannot open title-part directory: $!";
        if ($dh) {
            my @entries = sort grep { $_ ne '.' && $_ ne '..' } readdir $dh;
            closedir $dh;
            for my $entry (@entries) {
                my $relative = "$paths->{part_directory}/$entry";
                my $is_generated = $entry =~ /\A\Q$paths->{part_prefix}\E\d{4}\.md\z/;
                push @$errors, "unexpected title-part residue '$relative'" if !$is_generated;
                push @$errors, "stale title-part output '$relative'"
                    if !$skip_outputs && $is_generated && !$expected{$relative};
            }
        }
    }
    return if $skip_outputs;
    for my $output (@{$context->{projection}}) {
        my $actual = read_regular($base, $output->{path}, "migrated $output->{role}", $errors);
        next if !defined $actual;
        push @$errors, "migrated output differs from deterministic rendering: $output->{path}"
            if $actual ne $output->{raw};
    }
    my $current_rows = [map { legacy_row($_, $context->{limits}) } @{$context->{cards}}];
    if (@$current_rows == $context->{contract}{legacy}{card_count}
        && row_digest($current_rows) eq $context->{contract}{legacy}{row_sha256}) {
        compare_planned_records(
            planned_records($context->{projection}),
            $context->{contract}{planned_outputs},
            $errors,
        );
    }
}

sub pressure_findings {
    my ($actual, $targets, $label) = @_;
    my @errors;
    my @warnings;
    for my $dimension (qw(lines bytes line_bytes)) {
        my $percent = 100 * $actual->{$dimension} / $targets->{$dimension};
        if ($percent >= 90) {
            push @errors, sprintf('%s %s is at %.1f%% of health; mandatory rollover is required',
                $label, $dimension, $percent);
        } elsif ($percent >= 80) {
            push @warnings, sprintf('%s %s is at %.1f%% of health', $label, $dimension, $percent);
        }
    }
    return (\@errors, \@warnings);
}

sub collection_pressure_findings {
    my ($actual, $targets, $label) = @_;
    my @errors;
    my @warnings;
    for my $dimension (qw(files lines_each bytes_each lines_total bytes_total line_bytes_each)) {
        next if $dimension eq 'files' && $actual->{$dimension} == $targets->{$dimension};
        my $percent = 100 * $actual->{$dimension} / $targets->{$dimension};
        if ($percent >= 90) {
            push @errors, sprintf('%s %s is at %.1f%% of health; mandatory rollover is required',
                $label, $dimension, $percent);
        } elsif ($percent >= 80) {
            push @warnings, sprintf('%s %s is at %.1f%% of health', $label, $dimension, $percent);
        }
    }
    return (\@errors, \@warnings);
}

sub atomic_write {
    my ($path, $raw) = @_;
    make_path(dirname($path)) if !-d dirname($path);
    my $temporary = $path . '.fact-catalog.' . $$;
    sysopen my $fh, $temporary, O_WRONLY | O_CREAT | O_EXCL, 0644
        or die "fact-card-catalog: cannot create same-directory temporary '$temporary': $!\n";
    binmode $fh, ':raw';
    print {$fh} $raw or die "fact-card-catalog: cannot write '$temporary': $!\n";
    close $fh or die "fact-card-catalog: cannot close '$temporary': $!\n";
    rename $temporary, $path or die "fact-card-catalog: cannot replace '$path': $!\n";
}

sub write_projection {
    my ($base, $context, $quiet) = @_;
    my $state = $context->{contract}{migration_state};
    if ($state eq 'legacy_locked') {
        atomic_write(absolute($base, $context->{paths}{landing}), $context->{legacy});
        print "fact-card-catalog: wrote legacy monolithic catalog for "
            . scalar(@{$context->{cards}}) . " cards.\n" if !$quiet;
        return;
    }
    my $workspace_rel = "generated/.fact-card-catalog-write.$$";
    my $workspace = absolute($base, $workspace_rel);
    die "fact-card-catalog: unsafe existing write workspace\n" if -e $workspace;
    make_path($workspace);
    my $root_device = (stat($base))[0];
    my $workspace_device = (stat($workspace))[0];
    die "fact-card-catalog: write workspace is off the repository volume\n"
        if !defined($root_device) || !defined($workspace_device) || $root_device != $workspace_device;
    eval {
        for my $output (@{$context->{projection}}) {
            my $workspace_path = absolute($workspace, $output->{path});
            make_path(dirname($workspace_path));
            atomic_write($workspace_path, $output->{raw});
        }
        my @workspace_errors;
        for my $output (@{$context->{projection}}) {
            my $staged = read_regular(
                $workspace, $output->{path}, "staged $output->{role}", \@workspace_errors,
            );
            push @workspace_errors, "staged output differs from deterministic rendering: $output->{path}"
                if defined($staged) && $staged ne $output->{raw};
        }
        die join("\n", @workspace_errors) . "\n" if @workspace_errors;
        make_path(absolute($base, $context->{paths}{part_directory}));
        for my $output (@{$context->{projection}}[1 .. $#{$context->{projection}}]) {
            atomic_write(absolute($base, $output->{path}), $output->{raw});
        }
        my %expected = map { $_->{path} => 1 } @{$context->{projection}};
        my $directory = absolute($base, $context->{paths}{part_directory});
        opendir my $dh, $directory or die "cannot open title-part directory: $!\n";
        my @entries = sort grep { $_ ne '.' && $_ ne '..' } readdir $dh;
        closedir $dh;
        for my $entry (@entries) {
            next if $entry !~ /\A\Q$context->{paths}{part_prefix}\E\d{4}\.md\z/;
            my $relative = "$context->{paths}{part_directory}/$entry";
            next if $expected{$relative};
            unlink absolute($base, $relative)
                or die "cannot remove stale generated title part '$relative': $!\n";
        }
        atomic_write(absolute($base, $context->{paths}{landing}), $context->{projection}[0]{raw});
        1;
    } or do {
        my $error = $@ || 'unknown write failure';
        remove_tree($workspace) if -e $workspace;
        die $error;
    };
    remove_tree($workspace);
    print "fact-card-catalog: wrote bounded landing plus "
        . ($#{$context->{projection}}) . " title parts for "
        . scalar(@{$context->{cards}}) . " cards.\n" if !$quiet;
}

sub write_raw {
    my ($base, $relative, $raw) = @_;
    my $path = absolute($base, $relative);
    make_path(dirname($path));
    open my $fh, '>:raw', $path or die "fact-card-catalog self-test: cannot write $relative: $!\n";
    print {$fh} $raw;
    close $fh or die "fact-card-catalog self-test: cannot close $relative: $!\n";
}

sub fixture_card {
    my ($id, $title) = @_;
    return encode_utf8(<<"CARD");
---
id: $id
title: $title
answers:
  - "where is $id"
date: 2026-08-09
evidence: docs/example.md
---

Fixture fact.
CARD
}

sub self_test_paths {
    return {
        card_directory => 'docs/knowledge', collection_readme => 'docs/knowledge/README.md',
        landing => 'docs/knowledge/INDEX.md', part_directory => 'docs/knowledge-catalog',
        part_prefix => 'titles-',
    };
}

sub fixture_contract {
    my ($state) = @_;
    return {
        schema_version => 1,
        migration_state => $state,
        surface_id => 'knowledge_cards',
        part_surface_id => 'fact_card_titles',
        paths => {
            card_directory => 'docs/knowledge',
            collection_readme => 'docs/knowledge/README.md',
            landing => 'docs/knowledge/INDEX.md',
            part_directory => 'docs/knowledge-catalog',
            part_prefix => 'titles-',
            surface_registry => 'doctrine/live_document_size/surfaces.jsonl',
            question_contract => 'doctrine/knowledge_map/shard_contract.json',
        },
        legacy => {
            boundary_commit => '0' x 40,
            git_blob => '0' x 40,
            sha256 => '0' x 64,
            metrics => {lines => 1, bytes => 1, line_bytes => 1},
            card_count => 3,
            row_sha256 => '0' x 64,
        },
        limits => fixed_limits(),
        planned_outputs => [],
        verifier => 'perl scripts/check_fact_card_catalog.pl --check',
    };
}

sub fixture_surface {
    my ($files, $state) = @_;
    $state //= 'legacy_locked';
    my $index_contract = $state eq 'migrated'
        ? {
            kind => 'routed_membership', verifier => 'builtin:markdown_links',
            route_surface => 'fact_card_titles',
        }
        : {kind => 'membership', verifier => 'builtin:markdown_links'};
    return {
        surface_id => 'knowledge_cards', targets => ['docs/knowledge/*.md'], locator => 'file',
        lifecycle => 'partitioned_canonical', state => 'normal', owner => 'fixture',
        health_targets => {files => $files}, enforcement_ceilings => {files => $files},
        milestones => {warning_pct => 80, rollover_pct => 90},
        verifier => 'scripts/check_fact_card_catalog.pl', index => 'docs/knowledge/INDEX.md',
        index_contract => $index_contract,
    };
}

sub fixture_record_surface {
    my ($files) = @_;
    $files //= 44;
    return {
        surface_id => 'decision_records', targets => ['docs/decisions/*.md'], locator => 'collection',
        lifecycle => 'partitioned_canonical', state => 'normal', owner => 'fixture',
        health_targets => {files => $files}, enforcement_ceilings => {files => $files},
        milestones => {warning_pct => 80, rollover_pct => 90}, verifier => 'builtin:budget',
        index => 'docs/decisions/INDEX.md',
        index_contract => {kind => 'membership', verifier => 'builtin:markdown_links'},
    };
}

sub fixture_part_surface {
    my $limits = fixed_limits()->{title_parts};
    return {
        surface_id => 'fact_card_titles', targets => ['docs/knowledge-catalog/titles-*.md'],
        locator => 'collection', lifecycle => 'generated_projection', state => 'normal',
        owner => 'knowledge-maintainers', health_targets => $limits->{health_targets},
        enforcement_ceilings => $limits->{enforcement_ceilings},
        milestones => {warning_pct => 80, rollover_pct => 90}, verifier => 'builtin:budget',
        canonical_inputs => ['docs/knowledge/*.md'],
        freshness_verifier => 'scripts/check_fact_card_catalog.pl',
        index => 'docs/knowledge/INDEX.md',
        index_contract => {kind => 'external_membership', verifier => 'builtin:markdown_links'},
    };
}

sub init_fixture {
    my ($base, $state, $mutator) = @_;
    my $contract = fixture_contract($state);
    my @fixture_surfaces = (fixture_surface(338, $state), fixture_record_surface());
    push @fixture_surfaces, fixture_part_surface() if $state eq 'migrated';
    write_raw(
        $base, 'doctrine/live_document_size/surfaces.jsonl',
        join('', map { JSON::PP->new->canonical(1)->encode($_) . "\n" } @fixture_surfaces),
    );
    write_raw(
        $base, 'doctrine/knowledge_map/shard_contract.json',
        JSON::PP->new->canonical(1)->pretty(1)->encode({
            fact_catalog => 'docs/knowledge/INDEX.md', limits => {max_facts => 379},
        }),
    );
    write_raw($base, 'docs/knowledge/README.md', "# Cards\n");
    write_raw($base, 'docs/knowledge/alpha.md', fixture_card('alpha', 'Alpha title'));
    write_raw($base, 'docs/knowledge/beta.md', fixture_card('beta', 'Beta | title'));
    write_raw($base, 'docs/knowledge/gamma.md', fixture_card('gamma', 'Gamma — title'));
    my @errors;
    my $cards = collect_cards($base, $contract->{paths}, fixed_limits(), \@errors);
    die "fixture card parse failed: @errors\n" if @errors;
    my $legacy = render_legacy($cards, fixed_limits(), \@errors);
    die "fixture legacy render failed: @errors\n" if @errors;
    write_raw($base, $contract->{paths}{landing}, $legacy);
    system('git', '-C', $base, 'init', '-q') == 0
        or die "fact-card-catalog self-test: git init failed\n";
    system('git', '-C', $base, 'add', '--', 'docs/knowledge') == 0
        or die "fact-card-catalog self-test: git add failed\n";
    system(
        'git', '-C', $base, '-c', 'user.name=Fixture',
        '-c', 'user.email=fixture@example.invalid', '-c', 'commit.gpgsign=false',
        'commit', '-qm', 'fixture fact catalog boundary',
    ) == 0 or die "fact-card-catalog self-test: git commit failed\n";
    my ($commit, $commit_exit) = command_capture('git', '-C', $base, 'rev-parse', 'HEAD');
    die "fixture commit lookup failed\n" if $commit_exit;
    $commit =~ s/[\r\n]+\z//;
    my ($blob, $blob_exit) = command_capture(
        'git', '-C', $base, 'rev-parse', "HEAD:$contract->{paths}{landing}",
    );
    die "fixture blob lookup failed\n" if $blob_exit;
    $blob =~ s/[\r\n]+\z//;
    $contract->{legacy} = {
        boundary_commit => $commit,
        git_blob => $blob,
        sha256 => sha256_hex($legacy),
        metrics => metrics($legacy),
        card_count => scalar(@$cards),
        row_sha256 => row_digest(legacy_rows($legacy)),
    };
    my $projection = render_projection($cards, $contract->{paths}, fixed_limits(), \@errors);
    die "fixture projection render failed: @errors\n" if @errors;
    $contract->{planned_outputs} = planned_records($projection);
    if ($state eq 'migrated') {
        for my $output (@$projection) {
            write_raw($base, $output->{path}, $output->{raw});
        }
    }
    $mutator->($base, $contract, $legacy, $projection) if defined $mutator;
    write_raw(
        $base, $DEFAULT_CONTRACT,
        JSON::PP->new->canonical(1)->pretty(1)->encode($contract),
    );
}

sub run_self_test {
    my ($base) = @_;
    my $generated = File::Spec->catdir($base, 'generated');
    make_path($generated);
    my $limits = fixed_limits();

    # The profile is derived, not a set of coincident literals (ADR 0029): capacity is the part
    # quantum times the part count, and every aggregate is its file bound times its per-file bound,
    # so no legal corpus can be refused by a total no single file can see.
    die "fact-card-catalog parser self-test: max_cards is not the derived part capacity\n"
        if $limits->{max_cards} != $limits->{cards_per_part} * $limits->{max_parts};
    for my $band (qw(health_targets enforcement_ceilings)) {
        my $part = $limits->{title_parts}{$band};
        die "fact-card-catalog parser self-test: title-part $band files disagree with max_parts\n"
            if $part->{files} != $limits->{max_parts};
        die "fact-card-catalog parser self-test: title-part $band lines_total is not files x lines_each\n"
            if $part->{lines_total} != $part->{files} * $part->{lines_each};
        die "fact-card-catalog parser self-test: title-part $band bytes_total is not files x bytes_each\n"
            if $part->{bytes_total} != $part->{files} * $part->{bytes_each};
    }
    my $projection_ceiling = $limits->{projection_ceiling};
    my $landing_ceiling = $limits->{landing}{enforcement_ceilings};
    my $part_ceiling = $limits->{title_parts}{enforcement_ceilings};
    die "fact-card-catalog parser self-test: projection ceiling does not cover landing plus parts\n"
        if $projection_ceiling->{files} != planned_output_limit()
        || $projection_ceiling->{lines} != $landing_ceiling->{lines} + $part_ceiling->{lines_total}
        || $projection_ceiling->{bytes} != $landing_ceiling->{bytes} + $part_ceiling->{bytes_total};

    my $normal = decode_utf8(fixture_card('alpha', 'Alpha title'), 1);
    my $parsed = parse_card('alpha.md', $normal, $limits);
    die "fact-card-catalog parser self-test: default status failed\n"
        if $parsed->{status} ne 'current';
    my $pipe = parse_card('beta.md', decode_utf8(fixture_card('beta', 'Beta | title'), 1), $limits);
    die "fact-card-catalog parser self-test: Markdown pipe escaping failed\n"
        if legacy_row($pipe, $limits) !~ /Beta \\\| title/;
    my @parser_failures = (
        ['missing answers', $normal =~ s/^answers:.*?^date:/date:/msr, qr/answers/],
        ['filename mismatch', $normal, qr/does not match filename/],
        ['bad date', $normal =~ s/2026-08-09/2026-13-09/r, qr/YYYY-MM-DD/],
        ['missing evidence', $normal =~ s/^evidence:.*\n//mr, qr/evidence or reverify/],
        ['duplicate id', $normal =~ s/^title:/id: duplicate\ntitle:/mr, qr/repeats front-matter key/],
    );
    for my $case (@parser_failures) {
        my ($name, $raw, $expected) = @$case;
        my $filename = $name eq 'filename mismatch' ? 'other.md' : 'alpha.md';
        my $ok = eval { parse_card($filename, $raw, $limits); 1 };
        die "fact-card-catalog parser self-test '$name' unexpectedly passed\n" if $ok;
        die "fact-card-catalog parser self-test '$name' missed $expected: $@\n" if $@ !~ $expected;
    }
    my $long_title = 'x' x ($limits->{max_source_title_bytes} + 1);
    my $long_ok = eval {
        parse_card('alpha.md', $normal =~ s/Alpha title/$long_title/r, $limits); 1;
    };
    die "fact-card-catalog parser self-test: oversized title passed\n" if $long_ok;
    my @packing_cards = map {
        my $id = sprintf('fact-%03d', $_);
        {id => $id, name => "$id.md", path => "docs/knowledge/$id.md",
         title => "Fact $_", date => '2026-08-09', status => 'current'};
    } 1 .. 57;
    my @packing_errors;
    my $packing = render_projection(
        \@packing_cards,
        self_test_paths(), $limits, \@packing_errors,
    );
    die "fact-card-catalog parser self-test: 57-card packing failed: @packing_errors\n"
        if @packing_errors || @$packing != 3;
    my @overflow_cards = map {
        my $id = sprintf('fact-%03d', $_);
        {id => $id, name => "$id.md", path => "docs/knowledge/$id.md",
         title => "Fact $_", date => '2026-08-09', status => 'current'};
    } 1 .. 337;
    my @overflow_errors;
    render_projection(
        \@overflow_cards,
        self_test_paths(), $limits, \@overflow_errors,
    );
    die "fact-card-catalog parser self-test: 337-card ceiling did not fail closed\n"
        if join("\n", @overflow_errors) !~ /card count exceeds migrated maximum 336/;
    my @capacity_cards = map {
        my $prefix = sprintf('fact-%03d-', $_);
        my $id = $prefix . ('x' x ($limits->{max_id_bytes} - length($prefix)));
        {id => $id, name => "$id.md", path => "docs/knowledge/$id.md",
         title => 'T' x $limits->{max_title_cell_bytes}, date => '2026-08-09',
         status => 'superseded'};
    } 1 .. $limits->{max_cards};
    my @capacity_errors;
    my $capacity_projection = render_projection(
        \@capacity_cards,
        self_test_paths(), $limits, \@capacity_errors,
    );
    my ($capacity_landing_errors) = pressure_findings(
        metrics($capacity_projection->[0]{raw}), $limits->{landing}{health_targets},
        'capacity landing',
    );
    my @capacity_parts = @$capacity_projection[1 .. $#$capacity_projection];
    my ($capacity_part_errors) = collection_pressure_findings(
        aggregate_metrics(\@capacity_parts), $limits->{title_parts}{health_targets},
        'capacity title parts',
    );
    die "fact-card-catalog parser self-test: exact 336-card capacity crosses mandatory pressure: "
        . join('; ', @capacity_errors, @$capacity_landing_errors, @$capacity_part_errors) . "\n"
        if @capacity_errors || @$capacity_landing_errors || @$capacity_part_errors;

    # The landing is a router: its size follows the title-part count, never the card count.
    my @single_errors;
    my $single_projection = render_projection(
        [$capacity_cards[0]], self_test_paths(), $limits, \@single_errors,
    );
    die "fact-card-catalog parser self-test: single-card projection failed: @single_errors\n"
        if @single_errors;
    for my $case ([$single_projection, 1], [$packing, 2], [$capacity_projection, 6]) {
        my ($projection, $parts) = @$case;
        my $lines = metrics($projection->[0]{raw})->{lines};
        die "fact-card-catalog parser self-test: landing is $lines lines for $parts parts, "
            . "expected " . ($parts + 6) . "\n"
            if $lines != $parts + 6 || $#$projection != $parts;
    }
    my @landing_link_errors;
    my $capacity_landing_links = resolved_links(
        $capacity_projection->[0]{raw}, 'docs/knowledge/INDEX.md',
        'capacity landing', \@landing_link_errors,
    );
    die "fact-card-catalog parser self-test: capacity landing has unsafe links: @landing_link_errors\n"
        if @landing_link_errors;
    die "fact-card-catalog parser self-test: the router landing still links a card directly\n"
        if grep { $capacity_landing_links->{$_->{path}} } @capacity_cards;
    for my $case (
        ['miscounted part', sub { $_[0] =~ s/\| 56 \| /| 55 | /; $_[0] }, qr/miscounts its title part/],
        ['renamed first id', sub {
            $_[0] =~ s/\| `\Q$capacity_cards[0]{id}\E`/| `fact-000-renamed`/; $_[0];
        }, qr/names the wrong first id/],
        ['dropped range row', sub { $_[0] =~ s/^\| \[0006\][^\n]*\n//m; $_[0] }, qr/exactly one range row per title part/],
    ) {
        my ($name, $mutator, $expected) = @$case;
        my @range_errors;
        validate_landing_ranges(
            $mutator->($capacity_projection->[0]{raw}), \@capacity_cards, $limits, 6, \@range_errors,
        );
        die "fact-card-catalog parser self-test: range rule missed '$name': @range_errors\n"
            if join("\n", @range_errors) !~ $expected;
    }
    my ($rollover_errors, $rollover_warnings) = pressure_findings(
        {lines => 90, bytes => 80, line_bytes => 1},
        {lines => 100, bytes => 100, line_bytes => 100},
        'synthetic projection',
    );
    die "fact-card-catalog parser self-test: rollover/warning pressure split failed\n"
        if @$rollover_errors != 1 || @$rollover_warnings != 1
            || $rollover_errors->[0] !~ /mandatory rollover/
            || $rollover_warnings->[0] !~ /bytes is at 80\.0%/;
    my @cases = (
        ['legacy positive', 'legacy_locked', undef, undef],
        ['unknown contract field', 'legacy_locked', sub { $_[1]{unknown} = 1 }, qr/unknown field/],
        ['unsafe landing path', 'legacy_locked', sub { $_[1]{paths}{landing} = '../INDEX.md' }, qr/landing is unsafe/],
        ['fixed limit inflation', 'legacy_locked', sub { $_[1]{limits}{max_cards}++ }, qr/limits differ/],
        ['surface capacity drift', 'legacy_locked', sub { write_raw($_[0], 'doctrine/live_document_size/surfaces.jsonl', JSON::PP->new->canonical(1)->encode(fixture_surface(339)) . "\n") }, qr/file health\/ceiling must remain 338/],
        ['surface milestone drift', 'legacy_locked', sub { my $surface = fixture_surface(338); $surface->{milestones}{rollover_pct} = 91; write_raw($_[0], 'doctrine/live_document_size/surfaces.jsonl', JSON::PP->new->canonical(1)->encode($surface) . "\n") }, qr/milestones must remain warning 80/],
        ['premature title surface', 'legacy_locked', sub { write_raw($_[0], 'doctrine/live_document_size/surfaces.jsonl', JSON::PP->new->canonical(1)->encode(fixture_surface(338)) . "\n" . JSON::PP->new->canonical(1)->encode(fixture_part_surface()) . "\n") }, qr/must be absent while legacy_locked/],
        ['question capacity drift', 'legacy_locked', sub { write_raw($_[0], 'doctrine/knowledge_map/shard_contract.json', JSON::PP->new->canonical(1)->encode({fact_catalog => 'docs/knowledge/INDEX.md', limits => {max_facts => 380}})) }, qr/max_facts must fund/],
        ['decision-record capacity drift', 'legacy_locked', sub { write_raw($_[0], 'doctrine/live_document_size/surfaces.jsonl', JSON::PP->new->canonical(1)->encode(fixture_surface(338)) . "\n" . JSON::PP->new->canonical(1)->encode(fixture_record_surface(45)) . "\n") }, qr/max_facts must fund/],
        ['boundary commit drift', 'legacy_locked', sub { $_[1]{legacy}{boundary_commit} = 'f' x 40 }, qr/boundary commit lookup/],
        ['boundary blob drift', 'legacy_locked', sub { $_[1]{legacy}{git_blob} = 'f' x 40 }, qr/boundary blob/],
        ['boundary digest drift', 'legacy_locked', sub { $_[1]{legacy}{sha256} = 'f' x 64 }, qr/SHA-256/],
        ['boundary metric drift', 'legacy_locked', sub { $_[1]{legacy}{metrics}{bytes}++ }, qr/legacy boundary bytes/],
        ['boundary row drift', 'legacy_locked', sub { $_[1]{legacy}{row_sha256} = 'f' x 64 }, qr/row digest/],
        ['working landing drift', 'legacy_locked', sub { write_raw($_[0], $_[1]{paths}{landing}, $_[2] . "changed\n") }, qr/working legacy landing differs/],
        ['stage-zero drift', 'legacy_locked', sub { my ($base, $contract, $legacy) = @_; write_raw($base, $contract->{paths}{landing}, $legacy . "stage\n"); system('git', '-C', $base, 'add', '--', $contract->{paths}{landing}) == 0 or die 'fixture stage failed'; write_raw($base, $contract->{paths}{landing}, $legacy) }, qr/stage-zero landing blob/],
        ['canonical row drift', 'legacy_locked', sub { my ($base) = @_; write_raw($base, 'docs/knowledge/alpha.md', fixture_card('alpha', 'Changed title')) }, qr/canonical card rendering/],
        ['premature title directory', 'legacy_locked', sub { make_path(absolute($_[0], $_[1]{paths}{part_directory})) }, qr/premature title-part directory/],
        ['planned hash drift', 'legacy_locked', sub { $_[1]{planned_outputs}[0]{sha256} = 'f' x 64 }, qr/planned_outputs membership/],
        ['planned duplicate path', 'legacy_locked', sub { $_[1]{planned_outputs}[1]{path} = $_[1]{planned_outputs}[0]{path} }, qr/duplicate path|membership\/order/],
        ['planned output above derived limit', 'legacy_locked', sub { my $records = $_[1]{planned_outputs}; push @$records, {%{$records->[0]}} while @$records <= planned_output_limit() }, qr/planned_outputs must contain one to 7 output records/],
        ['migrated positive', 'migrated', undef, undef],
        ['missing migrated title surface', 'migrated', sub { write_raw($_[0], 'doctrine/live_document_size/surfaces.jsonl', JSON::PP->new->canonical(1)->encode(fixture_surface(338, 'migrated')) . "\n") }, qr/must contain title-part surface/],
        ['migrated title surface drift', 'migrated', sub { my $part = fixture_part_surface(); $part->{health_targets}{lines_each}++; write_raw($_[0], 'doctrine/live_document_size/surfaces.jsonl', JSON::PP->new->canonical(1)->encode(fixture_surface(338, 'migrated')) . "\n" . JSON::PP->new->canonical(1)->encode($part) . "\n") }, qr/title-part surface health targets differ/],
        ['migrated membership kind drift', 'migrated', sub { write_raw($_[0], 'doctrine/live_document_size/surfaces.jsonl', JSON::PP->new->canonical(1)->encode(fixture_surface(338, 'legacy_locked')) . "\n" . JSON::PP->new->canonical(1)->encode(fixture_part_surface()) . "\n") }, qr/must route membership through the title parts/],
        ['migrated route surface drift', 'migrated', sub { my $surface = fixture_surface(338, 'migrated'); $surface->{index_contract}{route_surface} = 'other_titles'; write_raw($_[0], 'doctrine/live_document_size/surfaces.jsonl', JSON::PP->new->canonical(1)->encode($surface) . "\n" . JSON::PP->new->canonical(1)->encode(fixture_part_surface()) . "\n") }, qr/must route membership through the title parts/],
        ['missing migrated part', 'migrated', sub { unlink absolute($_[0], $_[1]{planned_outputs}[1]{path}) }, qr/migrated title_part is missing/],
        ['stale migrated part', 'migrated', sub { write_raw($_[0], $_[1]{paths}{part_directory} . '/titles-9999.md', "stale\n") }, qr/stale title-part output/],
        ['unexpected part residue', 'migrated', sub { write_raw($_[0], $_[1]{paths}{part_directory} . '/notes.md', "residue\n") }, qr/unexpected title-part residue/],
        ['migrated landing drift', 'migrated', sub { write_raw($_[0], $_[1]{paths}{landing}, $_[3][0]{raw} . "changed\n") }, qr/migrated output differs/],
        ['migrated link drift', 'migrated', sub { my ($base, $contract, undef, $projection) = @_; my $raw = $projection->[1]{raw}; $raw =~ s{\.\./knowledge/alpha\.md}{alpha.md}; write_raw($base, $projection->[1]{path}, $raw) }, qr/migrated output differs/],
        ['migrated card addition without regeneration', 'migrated', sub { write_raw($_[0], 'docs/knowledge/delta.md', fixture_card('delta', 'Delta title')) }, qr/migrated output differs/],
    );

    my $passed = 26;
    for my $index (0 .. $#cases) {
        my ($name, $state, $mutator, $expected) = @{$cases[$index]};
        my $fixture = File::Spec->catdir($generated, ".fact-card-catalog-self-test.$$.$index");
        remove_tree($fixture) if -e $fixture;
        make_path($fixture);
        init_fixture($fixture, $state, $mutator);
        my ($errors) = validate_tree($fixture, $DEFAULT_CONTRACT, 0);
        my $joined = join "\n", @$errors;
        if (!defined $expected) {
            die "fact-card-catalog self-test '$name' unexpectedly failed:\n$joined\n" if @$errors;
        } else {
            die "fact-card-catalog self-test '$name' unexpectedly passed\n" if !@$errors;
            die "fact-card-catalog self-test '$name' missed expected diagnostic $expected:\n$joined\n"
                if $joined !~ $expected;
        }
        remove_tree($fixture);
        $passed++;
    }
    my $write_fixture = File::Spec->catdir($generated, ".fact-card-catalog-write-self-test.$$");
    remove_tree($write_fixture) if -e $write_fixture;
    make_path($write_fixture);
    init_fixture($write_fixture, 'legacy_locked', undef);
    my @write_errors;
    my $write_contract = read_json($write_fixture, $DEFAULT_CONTRACT, 'contract', \@write_errors);
    die "fact-card-catalog write self-test: cannot read contract: @write_errors\n" if @write_errors;
    $write_contract->{migration_state} = 'migrated';
    write_raw(
        $write_fixture, $DEFAULT_CONTRACT,
        JSON::PP->new->canonical(1)->pretty(1)->encode($write_contract),
    );
    write_raw(
        $write_fixture, 'doctrine/live_document_size/surfaces.jsonl',
        JSON::PP->new->canonical(1)->encode(fixture_surface(338, 'migrated')) . "\n"
            . JSON::PP->new->canonical(1)->encode(fixture_record_surface()) . "\n"
            . JSON::PP->new->canonical(1)->encode(fixture_part_surface()) . "\n",
    );
    write_raw($write_fixture, 'docs/knowledge-catalog/titles-9999.md', "stale\n");
    my ($prepare_errors, $write_context) = prepare_context($write_fixture, $DEFAULT_CONTRACT, 0);
    die "fact-card-catalog write self-test: preparation failed: @$prepare_errors\n" if @$prepare_errors;
    write_projection($write_fixture, $write_context, 1);
    my ($written_errors) = validate_tree($write_fixture, $DEFAULT_CONTRACT, 0);
    die "fact-card-catalog write self-test: migrated output failed: @$written_errors\n"
        if @$written_errors;
    die "fact-card-catalog write self-test: stale generated part survived\n"
        if -e absolute($write_fixture, 'docs/knowledge-catalog/titles-9999.md');
    remove_tree($write_fixture);
    $passed++;
    print "fact-card-catalog: $passed/$passed source/plan/route/residue/bound cases pass.\n";
}
