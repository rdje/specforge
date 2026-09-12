#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(FB_CROAK decode encode);
use File::Basename qw(dirname);
use File::Copy qw(copy);
use File::Path qw(make_path remove_tree);
use File::Spec;
use FindBin qw($Bin);
use JSON::PP;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

# PRODUCTION-GRAPH-CENSUS-PIN.3 — the self-test line used to print the literal `15/15`, which was
# derived from nothing and so could not be checked against the suite: delete a case and it still said
# 15. The count is now taken by the assertions themselves (`assert_valid`/`assert_invalid` below) and
# compared against this declaration, so a case removed — or one added and not declared — fails the
# check instead of silently moving a number nobody reads. `$passed/$total` is the shape four sibling
# checks already use; the declaration lives beside the suite it counts rather than in a contract file.
our $SELF_TEST_PASSED = 0;
our $SELF_TEST_EXPECTED = 15;

my $mode = 'check';
my $root = File::Spec->catdir($Bin, '..');
my $relative_contract = 'doctrine/live_document_size/corpus_kb.json';
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
die "corpus-kb-currentness: unsafe contract path\n"
    if !safe_relative_path($relative_contract, 512);

if ($mode eq 'self-test') {
    run_self_test($base, $relative_contract);
    die "corpus-kb-currentness: self-test ran $SELF_TEST_PASSED assertions, "
        . "declaration expects $SELF_TEST_EXPECTED — re-derive the declaration beside the suite\n"
        if $SELF_TEST_PASSED != $SELF_TEST_EXPECTED;
    print "corpus-kb-currentness: self-test $SELF_TEST_PASSED/$SELF_TEST_EXPECTED passed.\n";
    exit 0;
}

my $contract = load_json(absolute($base, $relative_contract));
my @errors = validate_contract($base, $contract, $relative_contract, undef);
if (@errors) {
    print STDERR "corpus-kb-currentness: $_\n" for @errors;
    exit 1;
}

if ($mode eq 'report') {
    my $kg = $contract->{inputs}{kg_fixtures};
    my $outputs = $contract->{outputs};
    print JSON::PP->new->canonical->encode({
        schema_version => $contract->{schema_version},
        owner => $contract->{owner},
        reviewed_validation_snapshot => $contract->{inputs}{validation}{snapshot_path},
        fixture_root => $kg->{root},
        fixture_files => $kg->{file_count},
        fixtures => $kg->{fixture_count},
        passed => $contract->{producer}{expected_passed_count},
        managed_markdown_outputs => scalar(@{$outputs->{markdown}}),
        managed_json_outputs => 1,
        producer_regions => scalar(@{$contract->{producer}{regions}}),
        human_regions_preserved => JSON::PP::true,
        canonical_ir_mutation_allowed => JSON::PP::false,
        prior_memory_mutation_allowed => JSON::PP::false,
    }) . "\n";
} else {
    print "corpus-kb-currentness: reviewed validation, 156/156 KG fixtures, ten managed Markdown blocks, paired JSON, human regions, and producer seams are current.\n";
}

sub validate_contract {
    my ($base, $contract, $relative_contract, $tracked_override) = @_;
    my @errors;
    return ('contract root must be an object') if ref($contract) ne 'HASH';
    problem(\@errors, 'schema_version must be 1')
        if ($contract->{schema_version} // 0) != 1;
    problem(\@errors, 'owner differs from the task-owned authority')
        if ($contract->{owner} // '') ne 'LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5i';
    for my $field (qw(inputs outputs producer controls)) {
        problem(\@errors, "$field must be an object") if ref($contract->{$field}) ne 'HASH';
    }
    return @errors if @errors;

    my $controls = $contract->{controls};
    for my $field (qw(max_contract_bytes max_markdown_outputs max_producer_regions max_path_bytes max_marker_bytes max_fixture_files max_fixture_bytes)) {
        problem(\@errors, "controls.$field must be a positive integer")
            if !positive_integer($controls->{$field});
    }
    return @errors if @errors;
    my $contract_raw = read_raw(absolute($base, $relative_contract));
    problem(\@errors, 'contract exceeds controls.max_contract_bytes')
        if length($contract_raw) > $controls->{max_contract_bytes};

    validate_validation_input($base, $contract->{inputs}{validation}, $contract->{outputs}, $controls, \@errors);
    validate_kg_input($base, $contract->{inputs}{kg_fixtures}, $controls, $tracked_override, \@errors);
    validate_outputs($base, $contract->{outputs}, $controls, \@errors);
    validate_producer($base, $contract->{producer}, $controls, \@errors);
    return @errors;
}

sub validate_validation_input {
    my ($base, $validation, $outputs, $controls, $errors) = @_;
    if (ref($validation) ne 'HASH') {
        problem($errors, 'inputs.validation must be an object');
        return;
    }
    for my $field (qw(snapshot_path authority_contract_path authority_contract_sha256)) {
        problem($errors, "inputs.validation.$field must be a non-empty scalar")
            if !defined($validation->{$field}) || ref($validation->{$field}) || $validation->{$field} eq '';
    }
    problem($errors, 'reviewed validation snapshot must be authoritative')
        if !json_true($validation->{reviewed_snapshot_is_authoritative});
    problem($errors, 'ambient validation reports must not be authoritative')
        if !json_false($validation->{ambient_reports_are_authoritative});
    return if @$errors;
    for my $field (qw(snapshot_path authority_contract_path)) {
        problem($errors, "unsafe inputs.validation.$field")
            if !safe_relative_path($validation->{$field}, $controls->{max_path_bytes});
    }
    problem($errors, 'invalid validation authority contract hash')
        if !sha_value($validation->{authority_contract_sha256});
    return if @$errors;

    my $authority_path = absolute($base, $validation->{authority_contract_path});
    if (!regular_within($base, $authority_path)) {
        problem($errors, 'validation authority contract is missing, unsafe, or a symlink');
        return;
    }
    my $authority_raw = read_raw($authority_path);
    problem($errors, 'validation authority contract identity differs')
        if sha256_hex($authority_raw) ne $validation->{authority_contract_sha256};
    my $authority = eval { JSON::PP->new->utf8->decode($authority_raw) };
    problem($errors, 'validation authority contract is malformed JSON') if $@ || ref($authority) ne 'HASH';
    return if @$errors;
    problem($errors, 'validation authority does not declare reviewed status')
        if ($authority->{review_boundary}{status} // '') ne 'reviewed';
    problem($errors, 'validation authority permits ambient local artifacts')
        if !json_false($authority->{review_boundary}{local_artifacts_authoritative});
    problem($errors, 'validation snapshot path differs from its authority contract')
        if ($authority->{snapshot}{path} // '') ne $validation->{snapshot_path};

    my $snapshot_path = absolute($base, $validation->{snapshot_path});
    if (!regular_within($base, $snapshot_path)) {
        problem($errors, 'reviewed validation snapshot is missing, unsafe, or a symlink');
        return;
    }
    my $snapshot_raw = read_raw($snapshot_path);
    problem($errors, 'reviewed validation snapshot identity differs from its authority contract')
        if sha256_hex($snapshot_raw) ne ($authority->{snapshot}{sha256} // '');

    my ($validation_output) = grep { ($_->{kind} // '') eq 'reviewed_validation' }
        @{ref($outputs->{markdown}) eq 'ARRAY' ? $outputs->{markdown} : []};
    if (!$validation_output) {
        problem($errors, 'one reviewed_validation managed output is required');
        return;
    }
    my $page_path = absolute($base, $validation_output->{path});
    return if !regular_within($base, $page_path);
    my ($actual) = extract_managed(read_raw($page_path), $validation_output->{start_marker}, $validation_output->{end_marker});
    my ($expected, $render_error) = render_reviewed_validation_block($snapshot_raw);
    problem($errors, $render_error) if $render_error ne '';
    problem($errors, 'reviewed validation managed block differs from VALIDATION_SNAPSHOT.md')
        if defined($actual) && defined($expected) && $actual ne $expected;
}

sub validate_kg_input {
    my ($base, $kg, $controls, $tracked_override, $errors) = @_;
    if (ref($kg) ne 'HASH') {
        problem($errors, 'inputs.kg_fixtures must be an object');
        return;
    }
    for my $field (qw(file_count fixture_count bytes)) {
        problem($errors, "inputs.kg_fixtures.$field must be a positive integer")
            if !positive_integer($kg->{$field});
    }
    problem($errors, 'inputs.kg_fixtures.aggregate_sha256 is invalid')
        if !sha_value($kg->{aggregate_sha256} // '');
    problem($errors, 'KG fixture Git index must be authoritative')
        if !json_true($kg->{git_index_is_authoritative});
    problem($errors, 'unsafe KG fixture root')
        if !safe_relative_path($kg->{root} // '', $controls->{max_path_bytes});
    return if @$errors;
    problem($errors, 'KG fixture file count exceeds control bound')
        if $kg->{file_count} > $controls->{max_fixture_files};
    problem($errors, 'KG fixture bytes exceed control bound')
        if $kg->{bytes} > $controls->{max_fixture_bytes};

    my @paths;
    if (defined $tracked_override) {
        @paths = @$tracked_override;
    } else {
        my ($paths, $git_error) = git_tracked_paths($base, $kg->{root});
        problem($errors, $git_error) if $git_error ne '';
        @paths = @$paths;
    }
    @paths = sort @paths;
    problem($errors, 'tracked KG fixture file count differs') if @paths != $kg->{file_count};
    my ($aggregate, $bytes, $fixtures) = ('', 0, 0);
    my %seen;
    for my $path (@paths) {
        problem($errors, "unsafe tracked KG fixture path: $path")
            if !safe_relative_path($path, $controls->{max_path_bytes});
        problem($errors, "tracked KG fixture path is outside the declared root: $path")
            if index($path, "$kg->{root}/") != 0;
        problem($errors, "duplicate tracked KG fixture path: $path") if $seen{$path}++;
        my $absolute = absolute($base, $path);
        if (!regular_within($base, $absolute)) {
            problem($errors, "tracked KG fixture file is missing, unsafe, or a symlink: $path");
            next;
        }
        my $raw = read_raw($absolute);
        $aggregate .= $path . "\0" . sha256_hex($raw) . "\n";
        $bytes += length($raw);
        $fixtures++ if $path =~ m{/fixture\.json\z};
    }
    problem($errors, 'tracked KG fixture count differs') if $fixtures != $kg->{fixture_count};
    problem($errors, 'tracked KG fixture byte total differs') if $bytes != $kg->{bytes};
    problem($errors, 'tracked KG fixture aggregate identity differs')
        if sha256_hex($aggregate) ne $kg->{aggregate_sha256};
}

sub validate_outputs {
    my ($base, $outputs, $controls, $errors) = @_;
    if (ref($outputs) ne 'HASH') {
        problem($errors, 'outputs must be an object');
        return;
    }
    my $markdown = $outputs->{markdown};
    if (ref($markdown) ne 'ARRAY' || !@$markdown) {
        problem($errors, 'outputs.markdown must be a non-empty array');
        return;
    }
    problem($errors, 'exactly ten managed Markdown outputs are required') if @$markdown != 10;
    problem($errors, 'managed Markdown output count exceeds control bound')
        if @$markdown > $controls->{max_markdown_outputs};
    my (%ids, %paths);
    for my $entry (@$markdown) {
        if (ref($entry) ne 'HASH') {
            problem($errors, 'managed Markdown output entry must be an object');
            next;
        }
        for my $field (qw(id path kind start_marker end_marker managed_sha256 human_prefix_sha256 human_suffix_sha256)) {
            problem($errors, "managed Markdown $field must be a non-empty scalar")
                if !defined($entry->{$field}) || ref($entry->{$field}) || $entry->{$field} eq '';
        }
        for my $field (qw(managed_lines managed_bytes)) {
            problem($errors, "managed Markdown $field must be a positive integer")
                if !positive_integer($entry->{$field});
        }
        next if @$errors;
        problem($errors, "duplicate managed Markdown id: $entry->{id}") if $ids{$entry->{id}}++;
        problem($errors, "duplicate managed Markdown path: $entry->{path}") if $paths{$entry->{path}}++;
        problem($errors, "unsafe managed Markdown path: $entry->{path}")
            if !safe_relative_path($entry->{path}, $controls->{max_path_bytes});
        for my $marker_field (qw(start_marker end_marker)) {
            problem($errors, "managed Markdown $marker_field is overlong: $entry->{path}")
                if utf8_bytes($entry->{$marker_field}) > $controls->{max_marker_bytes};
        }
        for my $sha_field (qw(managed_sha256 human_prefix_sha256 human_suffix_sha256)) {
            problem($errors, "managed Markdown $sha_field is invalid: $entry->{path}")
                if !sha_value($entry->{$sha_field});
        }
        my $path = absolute($base, $entry->{path});
        if (!regular_within($base, $path)) {
            problem($errors, "managed Markdown output is missing, unsafe, or a symlink: $entry->{path}");
            next;
        }
        my $raw = read_raw($path);
        my ($managed, $prefix, $suffix, $marker_error) =
            extract_managed($raw, $entry->{start_marker}, $entry->{end_marker});
        if ($marker_error ne '') {
            problem($errors, "$entry->{path}: $marker_error");
            next;
        }
        problem($errors, "$entry->{path}: managed output identity differs")
            if sha256_hex($managed) ne $entry->{managed_sha256};
        problem($errors, "$entry->{path}: managed output line count differs")
            if line_count($managed) != $entry->{managed_lines};
        problem($errors, "$entry->{path}: managed output byte count differs")
            if length($managed) != $entry->{managed_bytes};
        problem($errors, "$entry->{path}: human prefix identity differs")
            if sha256_hex($prefix) ne $entry->{human_prefix_sha256};
        problem($errors, "$entry->{path}: human suffix identity differs")
            if sha256_hex($suffix) ne $entry->{human_suffix_sha256};
        if (($entry->{kind} // '') eq 'kg_aggregate') {
            problem($errors, 'aggregate KG output does not declare 156 fixtures')
                if $managed !~ /^- fixtures_total: `156`$/m;
            problem($errors, 'aggregate KG output does not declare 156 passed fixtures')
                if $managed !~ /^- fixtures_passed: `156`$/m;
            problem($errors, 'aggregate KG output does not declare zero failures')
                if $managed !~ /^- fixtures_failed: `0`$/m;
        }
    }

    my $json = $outputs->{json};
    if (ref($json) ne 'HASH') {
        problem($errors, 'outputs.json must be an object');
        return;
    }
    problem($errors, 'unsafe managed JSON path')
        if !safe_relative_path($json->{path} // '', $controls->{max_path_bytes});
    for my $field (qw(lines bytes candidate_count)) {
        problem($errors, "outputs.json.$field must be a positive integer")
            if !positive_integer($json->{$field});
    }
    problem($errors, 'outputs.json.sha256 is invalid') if !sha_value($json->{sha256} // '');
    return if @$errors;
    my $json_path = absolute($base, $json->{path});
    if (!regular_within($base, $json_path)) {
        problem($errors, 'managed JSON output is missing, unsafe, or a symlink');
        return;
    }
    my $raw = read_raw($json_path);
    problem($errors, 'managed JSON identity differs') if sha256_hex($raw) ne $json->{sha256};
    problem($errors, 'managed JSON line count differs') if line_count($raw) != $json->{lines};
    problem($errors, 'managed JSON byte count differs') if length($raw) != $json->{bytes};
    my $decoded = eval { JSON::PP->new->utf8->decode($raw) };
    problem($errors, 'managed JSON is malformed') if $@ || ref($decoded) ne 'HASH';
    return if @$errors;
    problem($errors, 'managed JSON candidate count differs')
        if ref($decoded->{candidates}) ne 'ARRAY' || @{$decoded->{candidates}} != $json->{candidate_count};
    problem($errors, 'managed JSON promotion status differs')
        if ($decoded->{promotion_status} // '') ne ($json->{promotion_status} // '');
}

sub validate_producer {
    my ($base, $producer, $controls, $errors) = @_;
    if (ref($producer) ne 'HASH') {
        problem($errors, 'producer must be an object');
        return;
    }
    my $regions = $producer->{regions};
    if (ref($regions) ne 'ARRAY' || !@$regions) {
        problem($errors, 'producer.regions must be a non-empty array');
        return;
    }
    problem($errors, 'producer region count exceeds control bound')
        if @$regions > $controls->{max_producer_regions};
    problem($errors, 'producer verification command differs')
        if ($producer->{verification_command} // '') ne 'cargo run --manifest-path Cargo.toml -- kg-bench';
    problem($errors, 'producer expected fixture/pass counts differ')
        if ($producer->{expected_fixture_count} // 0) != 156
        || ($producer->{expected_passed_count} // 0) != 156;
    problem($errors, 'producer must forbid canonical IR mutation')
        if !json_false($producer->{canonical_ir_mutation_allowed});
    problem($errors, 'producer must forbid prior-memory mutation')
        if !json_false($producer->{prior_memory_mutation_allowed});
    my %ids;
    for my $region (@$regions) {
        for my $field (qw(id path start_literal end_before_literal sha256)) {
            problem($errors, "producer region $field must be a non-empty scalar")
                if !defined($region->{$field}) || ref($region->{$field}) || $region->{$field} eq '';
        }
        for my $field (qw(lines bytes)) {
            problem($errors, "producer region $field must be a positive integer")
                if !positive_integer($region->{$field});
        }
        next if @$errors;
        problem($errors, "duplicate producer region id: $region->{id}") if $ids{$region->{id}}++;
        problem($errors, "unsafe producer path: $region->{path}")
            if !safe_relative_path($region->{path}, $controls->{max_path_bytes});
        problem($errors, "invalid producer region hash: $region->{id}")
            if !sha_value($region->{sha256});
        my $path = absolute($base, $region->{path});
        if (!regular_within($base, $path)) {
            problem($errors, "producer source is missing, unsafe, or a symlink: $region->{path}");
            next;
        }
        my $raw = read_raw($path);
        my $start = index($raw, $region->{start_literal});
        my $end = $start < 0 ? -1 : index($raw, $region->{end_before_literal}, $start);
        if ($start < 0 || $end < 0 || $end <= $start) {
            problem($errors, "producer region boundaries differ: $region->{id}");
            next;
        }
        problem($errors, "producer region start is ambiguous: $region->{id}")
            if index($raw, $region->{start_literal}, $start + 1) >= 0;
        my $body = substr($raw, $start, $end - $start);
        problem($errors, "producer region identity differs: $region->{id}")
            if sha256_hex($body) ne $region->{sha256};
        problem($errors, "producer region line count differs: $region->{id}")
            if line_count($body) != $region->{lines};
        problem($errors, "producer region byte count differs: $region->{id}")
            if length($body) != $region->{bytes};
    }
}

sub render_reviewed_validation_block {
    my ($raw) = @_;
    my $text = eval { decode('UTF-8', $raw, FB_CROAK) };
    return (undef, 'reviewed validation snapshot is not valid UTF-8') if $@;
    my (undef, $projected) = split /^## Projected Artifacts\n/m, $text, 2;
    return (undef, 'reviewed validation snapshot has no projected artifact section')
        if !defined($projected) || $projected !~ s/\A### //;
    my @chunks = split /\n### /, $projected;
    return (undef, 'reviewed validation snapshot has no artifact records') if !@chunks;
    my $output = "<!-- corpus_kb_validation_findings:start -->\n";
    $output .= "<!-- This reviewed block is refreshed from `VALIDATION_SNAPSHOT.md` by `specforge corpus-kb --validation-snapshot`. -->\n\n";
    my %keys;
    for my $chunk (@chunks) {
        my ($heading) = split /\n/, $chunk, 2;
        my ($stage) = $heading =~ / \(([^()]+)\)\z/;
        return (undef, "reviewed validation snapshot has malformed heading `$heading`") if !defined $stage;
        my ($key) = $chunk =~ /^- document_key: `([^`]+)`$/m;
        my ($artifact) = $chunk =~ /^- artifact_path: `([^`]+)`$/m;
        my ($fingerprint) = $chunk =~ /^- artifact_fingerprint: `([^`]+)`$/m;
        my ($score) = $chunk =~ /^- score: `([^`]+)`$/m;
        my ($summary) = $chunk =~ /^- summary: (.+)$/m;
        return (undef, 'reviewed validation snapshot artifact field is missing')
            if !defined($key) || !defined($artifact) || !defined($fingerprint)
            || !defined($score) || !defined($summary);
        return (undef, "reviewed validation snapshot repeats document_key `$key`") if $keys{$key}++;
        my ($finding_text) = $chunk =~ /^- findings:\n((?:  - .+\n?)+)/m;
        return (undef, "reviewed validation snapshot artifact `$key` has no findings")
            if !defined $finding_text;
        my @findings = ($finding_text =~ /^  - (.+)$/mg);
        return (undef, "reviewed validation snapshot artifact `$key` has no findings") if !@findings;
        $summary =~ s/\|/\\|/g;
        $output .= "### $key\n";
        $output .= "- artifact_path: `$artifact`\n";
        $output .= "- stage: `$stage`\n";
        $output .= "- artifact_fingerprint: `$fingerprint`\n";
        $output .= "- score: `$score`\n";
        $output .= "- summary: $summary\n";
        $output .= "- findings:\n";
        for my $finding (@findings) {
            $finding =~ s/\|/\\|/g;
            $output .= "  - $finding\n";
        }
        $output .= "\n";
    }
    $output .= '<!-- corpus_kb_validation_findings:end -->';
    return (encode('UTF-8', $output), '');
}

sub extract_managed {
    my ($raw, $start_marker, $end_marker) = @_;
    return (undef, undef, undef, 'markers must be non-empty')
        if !defined($start_marker) || !defined($end_marker) || $start_marker eq '' || $end_marker eq '';
    my $start = index($raw, $start_marker);
    return (undef, undef, undef, 'start marker is missing') if $start < 0;
    return (undef, undef, undef, 'start marker is duplicated')
        if index($raw, $start_marker, $start + 1) >= 0;
    my $end_start = index($raw, $end_marker, $start + length($start_marker));
    return (undef, undef, undef, 'end marker is missing or precedes the start marker') if $end_start < 0;
    return (undef, undef, undef, 'end marker is duplicated')
        if index($raw, $end_marker, $end_start + 1) >= 0;
    my $end = $end_start + length($end_marker);
    return (
        substr($raw, $start, $end - $start),
        substr($raw, 0, $start),
        substr($raw, $end),
        '',
    );
}

sub git_tracked_paths {
    my ($base, $root) = @_;
    my @paths;
    my $pid = open my $git, '-|', 'git', '-C', $base, 'ls-files', '-z', '--', $root;
    return ([], 'cannot run git ls-files for KG fixture membership') if !defined $pid;
    local $/ = "\0";
    while (my $path = <$git>) {
        $path =~ s/\0\z//;
        push @paths, $path if $path ne '';
    }
    close $git;
    return ([], 'git ls-files failed for KG fixture membership') if $? != 0;
    return (\@paths, '');
}

sub run_self_test {
    my ($base, $relative_contract) = @_;
    my $source_contract = load_json(absolute($base, $relative_contract));
    my ($tracked, $git_error) = git_tracked_paths($base, $source_contract->{inputs}{kg_fixtures}{root});
    die "corpus-kb-currentness self-test: $git_error\n" if $git_error ne '';
    my $temp_relative = "generated/.corpus-kb-currentness-self-test-$$";
    my $temp = absolute($base, $temp_relative);
    remove_tree($temp) if -e $temp;
    make_path($temp);
    my %paths = map { $_ => 1 } @$tracked;
    $paths{$relative_contract} = 1;
    my $validation = $source_contract->{inputs}{validation};
    $paths{$validation->{snapshot_path}} = 1;
    $paths{$validation->{authority_contract_path}} = 1;
    $paths{$_->{path}} = 1 for @{$source_contract->{outputs}{markdown}};
    $paths{$source_contract->{outputs}{json}{path}} = 1;
    $paths{$_->{path}} = 1 for @{$source_contract->{producer}{regions}};
    for my $path (sort keys %paths) {
        copy_relative($base, $temp, $path);
    }

    my $contract = load_json(absolute($temp, $relative_contract));
    assert_valid('baseline', $temp, $contract, $relative_contract, $tracked);

    my $bad = clone_json($contract);
    $bad->{schema_version} = 99;
    assert_invalid('schema', $temp, $bad, $relative_contract, $tracked);

    $bad = clone_json($contract);
    $bad->{outputs}{markdown}[0]{path} = '../escape.md';
    assert_invalid('unsafe path', $temp, $bad, $relative_contract, $tracked);

    my $page_path = $contract->{outputs}{markdown}[0]{path};
    my $page_raw = read_raw(absolute($temp, $page_path));
    my $start = $contract->{outputs}{markdown}[0]{start_marker};
    write_raw(absolute($temp, $page_path), do { my $x = $page_raw; $x =~ s/\Q$start\E//; $x });
    assert_invalid('missing marker', $temp, $contract, $relative_contract, $tracked);
    write_raw(absolute($temp, $page_path), $page_raw);

    write_raw(absolute($temp, $page_path), $page_raw . $start);
    assert_invalid('duplicate marker', $temp, $contract, $relative_contract, $tracked);
    write_raw(absolute($temp, $page_path), $page_raw);

    my $managed_mutation = $page_raw;
    $managed_mutation =~ s/score:/score-drift:/;
    write_raw(absolute($temp, $page_path), $managed_mutation);
    assert_invalid('managed output drift', $temp, $contract, $relative_contract, $tracked);
    write_raw(absolute($temp, $page_path), $page_raw);

    write_raw(absolute($temp, $page_path), "human drift\n" . $page_raw);
    assert_invalid('human prefix drift', $temp, $contract, $relative_contract, $tracked);
    write_raw(absolute($temp, $page_path), $page_raw);

    my $json_path = $contract->{outputs}{json}{path};
    my $json_raw = read_raw(absolute($temp, $json_path));
    write_raw(absolute($temp, $json_path), $json_raw . " ");
    assert_invalid('managed JSON drift', $temp, $contract, $relative_contract, $tracked);
    write_raw(absolute($temp, $json_path), $json_raw);

    my @missing = @$tracked;
    pop @missing;
    assert_invalid('missing tracked fixture', $temp, $contract, $relative_contract, \@missing);

    my $extra_path = $contract->{inputs}{kg_fixtures}{root} . '/self_test_extra/fixture.json';
    write_raw_with_parent(absolute($temp, $extra_path), "{}\n");
    my @extra = (@$tracked, $extra_path);
    assert_invalid('extra tracked fixture', $temp, $contract, $relative_contract, \@extra);

    my $fixture_path = $tracked->[0];
    my $fixture_raw = read_raw(absolute($temp, $fixture_path));
    write_raw(absolute($temp, $fixture_path), $fixture_raw . " ");
    assert_invalid('fixture content drift', $temp, $contract, $relative_contract, $tracked);
    write_raw(absolute($temp, $fixture_path), $fixture_raw);

    my $authority_path = $validation->{authority_contract_path};
    my $authority_raw = read_raw(absolute($temp, $authority_path));
    write_raw(absolute($temp, $authority_path), $authority_raw . " ");
    assert_invalid('validation authority drift', $temp, $contract, $relative_contract, $tracked);
    write_raw(absolute($temp, $authority_path), $authority_raw);

    my $producer_path = $contract->{producer}{regions}[0]{path};
    my $producer_raw = read_raw(absolute($temp, $producer_path));
    my $producer_start = $contract->{producer}{regions}[0]{start_literal};
    my $producer_drift = $producer_raw;
    $producer_drift =~ s/\Q$producer_start\E/$producer_start\/\/ drift/;
    write_raw(absolute($temp, $producer_path), $producer_drift);
    assert_invalid('producer region drift', $temp, $contract, $relative_contract, $tracked);
    write_raw(absolute($temp, $producer_path), $producer_raw);

    $bad = clone_json($contract);
    $bad->{inputs}{kg_fixtures}{fixture_count}++;
    assert_invalid('fixture count contract drift', $temp, $bad, $relative_contract, $tracked);

    $bad = clone_json($contract);
    $bad->{producer}{canonical_ir_mutation_allowed} = JSON::PP::true;
    assert_invalid('canonical mutation permission', $temp, $bad, $relative_contract, $tracked);

    remove_tree($temp);
    die "corpus-kb-currentness self-test: fixture residue remains\n" if -e $temp;
}

sub assert_valid {
    my ($name, @args) = @_;
    my @errors = validate_contract(@args);
    die "corpus-kb-currentness self-test `$name` unexpectedly failed: @errors\n" if @errors;
    $SELF_TEST_PASSED++;
}

sub assert_invalid {
    my ($name, @args) = @_;
    my @errors = validate_contract(@args);
    die "corpus-kb-currentness self-test `$name` unexpectedly passed\n" if !@errors;
    $SELF_TEST_PASSED++;
}

sub copy_relative {
    my ($source_root, $target_root, $relative) = @_;
    my $source = absolute($source_root, $relative);
    my $target = absolute($target_root, $relative);
    make_path(dirname($target));
    copy($source, $target) or die "copy $relative failed: $!\n";
}

sub clone_json {
    my ($value) = @_;
    return JSON::PP->new->decode(JSON::PP->new->canonical->encode($value));
}

sub canonical_root {
    my ($path) = @_;
    my $absolute = abs_path($path);
    die "corpus-kb-currentness: repository root does not exist\n" if !defined $absolute || !-d $absolute;
    return $absolute;
}

sub absolute {
    my ($base, $relative) = @_;
    return File::Spec->catfile($base, split m{/}, $relative);
}

sub safe_relative_path {
    my ($path, $max_bytes) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '' || utf8_bytes($path) > $max_bytes;
    return 0 if File::Spec->file_name_is_absolute($path) || $path =~ /(?:\A|\/)\.\.(?:\/|\z)/;
    return 0 if $path =~ /\0/ || $path =~ m{//};
    return 1;
}

sub regular_within {
    my ($base, $path) = @_;
    return 0 if !-f $path || -l $path;
    my $canonical = abs_path($path);
    return defined($canonical) && ($canonical eq $base || index($canonical, "$base/") == 0);
}

sub load_json {
    my ($path) = @_;
    my $raw = read_raw($path);
    my $decoded = eval { JSON::PP->new->utf8->decode($raw) };
    die "corpus-kb-currentness: malformed JSON at $path: $@\n" if $@;
    return $decoded;
}

sub read_raw {
    my ($path) = @_;
    open my $handle, '<:raw', $path or die "corpus-kb-currentness: cannot read $path: $!\n";
    local $/;
    my $raw = <$handle>;
    close $handle or die "corpus-kb-currentness: cannot close $path: $!\n";
    return $raw;
}

sub write_raw {
    my ($path, $raw) = @_;
    open my $handle, '>:raw', $path or die "corpus-kb-currentness self-test: cannot write $path: $!\n";
    print {$handle} $raw;
    close $handle or die "corpus-kb-currentness self-test: cannot close $path: $!\n";
}

sub write_raw_with_parent {
    my ($path, $raw) = @_;
    make_path(dirname($path));
    write_raw($path, $raw);
}

sub positive_integer {
    my ($value) = @_;
    return defined($value) && !ref($value) && $value =~ /^\d+$/ && $value > 0;
}

sub sha_value {
    my ($value) = @_;
    return defined($value) && !ref($value) && $value =~ /^[0-9a-f]{64}$/;
}

sub json_true {
    my ($value) = @_;
    return JSON::PP::is_bool($value) && $value;
}

sub json_false {
    my ($value) = @_;
    return JSON::PP::is_bool($value) && !$value;
}

sub utf8_bytes {
    my ($value) = @_;
    return length(encode('UTF-8', $value));
}

sub line_count {
    my ($raw) = @_;
    my $lines = () = $raw =~ /\n/g;
    $lines++ if $raw ne '' && $raw !~ /\n\z/;
    return $lines;
}

sub problem {
    my ($errors, $message) = @_;
    push @$errors, $message;
}
