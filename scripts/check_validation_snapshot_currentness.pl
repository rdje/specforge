#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(decode encode FB_CROAK);
use File::Basename qw(dirname);
use File::Path qw(make_path remove_tree);
use File::Spec;
use FindBin qw($Bin);
use JSON::PP;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root = abs_path(File::Spec->catdir($Bin, '..'));
my $contract_path = 'doctrine/live_document_size/validation_snapshot.json';
my $mode = 'check';
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        die "validation-snapshot-currentness: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "validation-snapshot-currentness: root does not exist\n" if !defined $root;
    } elsif ($arg eq '--contract') {
        die "validation-snapshot-currentness: --contract requires a path\n" if !@ARGV;
        $contract_path = shift @ARGV;
    } else {
        die "Usage: $0 [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n";
    }
}

if ($mode eq 'self-test') {
    run_self_test($root, $contract_path);
    print "validation-snapshot-currentness: self-test 10/10 passed.\n";
    exit 0;
}

my $contract_absolute = absolute($root, $contract_path);
my $contract = load_json($contract_absolute);
my @errors = validate_contract($root, $contract, 0);
if (@errors) {
    print STDERR "validation-snapshot-currentness: $_\n" for @errors;
    exit 1;
}

if ($mode eq 'report') {
    my $report = {
        schema_version => $contract->{schema_version},
        owner => $contract->{owner},
        reviewed_commit => $contract->{review_boundary}{source_commit},
        snapshot => {
            path => $contract->{snapshot}{path},
            sha256 => $contract->{snapshot}{sha256},
            lines => $contract->{snapshot}{lines},
            bytes => $contract->{snapshot}{bytes},
            line_bytes => $contract->{snapshot}{line_bytes},
        },
        artifacts => scalar(@{$contract->{artifacts}}),
        recommendations => $contract->{rescan_projection}{recommendations},
        execution_summaries => $contract->{rescan_projection}{execution_summaries},
        producer_regions => scalar(@{$contract->{producer}{regions}}),
        ambient_local_artifacts_authoritative => JSON::PP::false,
    };
    print JSON::PP->new->canonical->encode($report), "\n";
} else {
    print "validation-snapshot-currentness: reviewed boundary, four report identities, producer regions, and live projection are current.\n";
}

sub validate_contract {
    my ($base, $contract, $skip_git_history) = @_;
    my @errors;
    if (ref($contract) ne 'HASH') {
        return ('contract root must be an object');
    }
    unknown_fields($contract, 'contract', [qw(schema_version owner snapshot review_boundary producer artifacts rescan_projection live_projection)], \@errors);
    problem(\@errors, 'schema_version must be 1') if ($contract->{schema_version} // 0) != 1;
    scalar_string($contract->{owner}, 'owner', \@errors);

    my $snapshot = object($contract->{snapshot}, 'snapshot', \@errors);
    unknown_fields($snapshot, 'snapshot', [qw(path sha256 lines bytes line_bytes required_h2_order required_literals)], \@errors);
    my $snapshot_path = safe_path_value($snapshot->{path}, 'snapshot.path', \@errors);
    sha_value($snapshot->{sha256}, 'snapshot.sha256', \@errors);
    positive_integer($snapshot->{lines}, 'snapshot.lines', \@errors);
    positive_integer($snapshot->{bytes}, 'snapshot.bytes', \@errors);
    positive_integer($snapshot->{line_bytes}, 'snapshot.line_bytes', \@errors);
    my $required_h2 = string_array($snapshot->{required_h2_order}, 'snapshot.required_h2_order', \@errors, 8);
    my $required_literals = string_array($snapshot->{required_literals}, 'snapshot.required_literals', \@errors, 16);

    my $review = object($contract->{review_boundary}, 'review_boundary', \@errors);
    unknown_fields($review, 'review_boundary', [qw(source_commit source_snapshot_sha256 status evidence_path evidence_literal local_artifacts_authoritative)], \@errors);
    problem(\@errors, 'review_boundary.source_commit must be a full Git object id')
        if ($review->{source_commit} // '') !~ /^[0-9a-f]{40}$/;
    sha_value($review->{source_snapshot_sha256}, 'review_boundary.source_snapshot_sha256', \@errors);
    problem(\@errors, 'review_boundary.status must be reviewed') if ($review->{status} // '') ne 'reviewed';
    my $evidence_path = safe_path_value($review->{evidence_path}, 'review_boundary.evidence_path', \@errors);
    scalar_string($review->{evidence_literal}, 'review_boundary.evidence_literal', \@errors);
    problem(\@errors, 'ambient local artifacts must not be validation authority')
        if !JSON::PP::is_bool($review->{local_artifacts_authoritative})
        || $review->{local_artifacts_authoritative};

    my $producer = object($contract->{producer}, 'producer', \@errors);
    unknown_fields($producer, 'producer', [qw(path regions)], \@errors);
    my $producer_path = safe_path_value($producer->{path}, 'producer.path', \@errors);
    my $regions = object_array($producer->{regions}, 'producer.regions', \@errors, 8);
    my %region_ids;
    for my $i (0 .. $#$regions) {
        my $region = $regions->[$i];
        my $label = "producer.regions[$i]";
        unknown_fields($region, $label, [qw(id start_literal end_before_literal lines bytes sha256)], \@errors);
        my $id = scalar_string($region->{id}, "$label.id", \@errors);
        problem(\@errors, "producer region id '$id' is duplicated") if defined($id) && $region_ids{$id}++;
        scalar_string($region->{start_literal}, "$label.start_literal", \@errors);
        scalar_string($region->{end_before_literal}, "$label.end_before_literal", \@errors);
        positive_integer($region->{lines}, "$label.lines", \@errors);
        positive_integer($region->{bytes}, "$label.bytes", \@errors);
        sha_value($region->{sha256}, "$label.sha256", \@errors);
    }

    my $artifacts = object_array($contract->{artifacts}, 'artifacts', \@errors, 16);
    problem(\@errors, 'exactly four reviewed artifacts must be declared') if @$artifacts != 4;
    my (%keys, %paths, %fingerprints);
    for my $i (0 .. $#$artifacts) {
        my $artifact = $artifacts->[$i];
        my $label = "artifacts[$i]";
        unknown_fields($artifact, $label, [qw(document_key display_name stage artifact_path artifact_fingerprint score grade finding_count)], \@errors);
        my $key = scalar_string($artifact->{document_key}, "$label.document_key", \@errors);
        problem(\@errors, "$label.document_key is invalid")
            if defined($key) && $key !~ /^[a-z0-9]+(?:_[a-z0-9]+)*$/;
        problem(\@errors, "artifact document_key '$key' is duplicated") if defined($key) && $keys{$key}++;
        scalar_string($artifact->{display_name}, "$label.display_name", \@errors);
        problem(\@errors, "$label.stage must be intent_ir") if ($artifact->{stage} // '') ne 'intent_ir';
        my $path = safe_path_value($artifact->{artifact_path}, "$label.artifact_path", \@errors);
        problem(\@errors, "artifact path '$path' is duplicated") if defined($path) && $paths{$path}++;
        my $fingerprint = $artifact->{artifact_fingerprint} // '';
        problem(\@errors, "$label.artifact_fingerprint must be 16 lowercase hex digits")
            if $fingerprint !~ /^[0-9a-f]{16}$/;
        problem(\@errors, "artifact fingerprint '$fingerprint' is duplicated")
            if $fingerprint ne '' && $fingerprints{$fingerprint}++;
        bounded_integer($artifact->{score}, "$label.score", 0, 100, \@errors);
        problem(\@errors, "$label.grade is invalid")
            if ($artifact->{grade} // '') !~ /^(?:EXCELLENT|GOOD|ADEQUATE|NEEDS IMPROVEMENT)$/;
        positive_integer($artifact->{finding_count}, "$label.finding_count", \@errors);
    }

    my $rescan = object($contract->{rescan_projection}, 'rescan_projection', \@errors);
    unknown_fields($rescan, 'rescan_projection', [qw(recommendations execution_summaries live_queue_rows live_queue_remainder)], \@errors);
    positive_integer($rescan->{recommendations}, 'rescan_projection.recommendations', \@errors);
    nonnegative_integer($rescan->{execution_summaries}, 'rescan_projection.execution_summaries', \@errors);
    nonnegative_integer($rescan->{live_queue_rows}, 'rescan_projection.live_queue_rows', \@errors);
    nonnegative_integer($rescan->{live_queue_remainder}, 'rescan_projection.live_queue_remainder', \@errors);
    problem(\@errors, 'live queue rows plus remainder must equal recommendations')
        if defined($rescan->{recommendations}) && defined($rescan->{live_queue_rows})
        && defined($rescan->{live_queue_remainder})
        && $rescan->{live_queue_rows} + $rescan->{live_queue_remainder} != $rescan->{recommendations};

    my $live = object($contract->{live_projection}, 'live_projection', \@errors);
    unknown_fields($live, 'live_projection', [qw(path start_marker end_marker sha256 lines bytes required_literal)], \@errors);
    my $live_path = safe_path_value($live->{path}, 'live_projection.path', \@errors);
    scalar_string($live->{start_marker}, 'live_projection.start_marker', \@errors);
    scalar_string($live->{end_marker}, 'live_projection.end_marker', \@errors);
    sha_value($live->{sha256}, 'live_projection.sha256', \@errors);
    positive_integer($live->{lines}, 'live_projection.lines', \@errors);
    positive_integer($live->{bytes}, 'live_projection.bytes', \@errors);
    scalar_string($live->{required_literal}, 'live_projection.required_literal', \@errors);

    validate_snapshot($base, $snapshot_path, $snapshot, $artifacts, $rescan, $required_h2, $required_literals, \@errors)
        if defined $snapshot_path;
    validate_live_projection($base, $live_path, $live, $artifacts, $rescan, \@errors)
        if defined $live_path;
    validate_producer($base, $producer_path, $regions, \@errors) if defined $producer_path;
    validate_review_evidence($base, $evidence_path, $review, \@errors) if defined $evidence_path;
    validate_review_history($base, $snapshot_path, $review, \@errors)
        if !$skip_git_history && defined $snapshot_path;
    return @errors;
}

sub validate_snapshot {
    my ($base, $path, $declared, $artifacts, $rescan, $required_h2, $required_literals, $errors) = @_;
    my $absolute = absolute($base, $path);
    if (!-f $absolute) {
        problem($errors, "snapshot '$path' is missing");
        return;
    }
    my $raw = read_raw($absolute);
    my $text = decoded($raw, $path, $errors);
    return if !defined $text;
    compare_identity($raw, $declared, 'snapshot', $errors);
    my @h2 = ($text =~ /^## (.+)$/mg);
    problem($errors, 'snapshot H2 order differs') if join("\n", @h2) ne join("\n", @$required_h2);
    for my $literal (@$required_literals) {
        problem($errors, "snapshot lacks required literal '$literal'") if index($text, $literal) < 0;
    }
    problem($errors, 'snapshot artifact summary count differs')
        if $text !~ /^- Artifacts projected: \Q@{[scalar @$artifacts]}\E$/m;
    problem($errors, 'snapshot recommendation summary count differs')
        if $text !~ /^- Targeted rescan recommendations: \Q$rescan->{recommendations}\E$/m;
    problem($errors, 'snapshot execution-summary count differs')
        if $text !~ /^- Rescan execution summaries: \Q$rescan->{execution_summaries}\E(?:\s|$)/m;

    my ($recommendation_text, $projected_text) =
        $text =~ /^## Targeted Rescan Recommendations\n(.*?)^## Projected Artifacts\n(.*)\z/ms;
    if (!defined $recommendation_text || !defined $projected_text) {
        problem($errors, 'snapshot recommendation/projected sections are malformed');
        return;
    }
    my @recommendation_chunks = grep { /^### /m } split /(?=^### )/m, $recommendation_text;
    problem($errors, 'snapshot recommendation record count differs')
        if @recommendation_chunks != $rescan->{recommendations};
    my @artifact_chunks = grep { /^### /m } split /(?=^### )/m, $projected_text;
    problem($errors, 'snapshot projected artifact record count differs') if @artifact_chunks != @$artifacts;
    for my $i (0 .. $#$artifacts) {
        last if $i > $#artifact_chunks;
        my $artifact = $artifacts->[$i];
        my $chunk = $artifact_chunks[$i];
        my $heading = "### $artifact->{display_name} ($artifact->{stage})";
        problem($errors, "snapshot projected artifact $i heading differs") if $chunk !~ /^\Q$heading\E$/m;
        for my $pair (
            ['document_key', $artifact->{document_key}],
            ['artifact_path', $artifact->{artifact_path}],
            ['artifact_fingerprint', $artifact->{artifact_fingerprint}],
        ) {
            my ($field, $value) = @$pair;
            problem($errors, "snapshot projected artifact $i $field differs")
                if $chunk !~ /^- \Q$field\E: `\Q$value\E`$/m;
        }
        my $score = "$artifact->{score}/100 $artifact->{grade}";
        problem($errors, "snapshot projected artifact $i score differs")
            if $chunk !~ /^- score: `\Q$score\E`$/m;
        problem($errors, "snapshot projected artifact $i summary finding count differs")
            if $chunk !~ /^- summary: .* with \Q$artifact->{finding_count}\E finding\(s\)$/m;
        my @finding_lines = ($chunk =~ /^  - \[[^\n]+$/mg);
        problem($errors, "snapshot projected artifact $i finding-line count differs")
            if @finding_lines != $artifact->{finding_count};
    }
}

sub validate_live_projection {
    my ($base, $path, $declared, $artifacts, $rescan, $errors) = @_;
    my $absolute = absolute($base, $path);
    if (!-f $absolute) {
        problem($errors, "live projection path '$path' is missing");
        return;
    }
    my $raw = read_raw($absolute);
    my $text = decoded($raw, $path, $errors);
    return if !defined $text;
    my $start = $declared->{start_marker} // '';
    my $end = $declared->{end_marker} // '';
    problem($errors, 'live projection start marker must occur exactly once') if count_literal($text, $start) != 1;
    problem($errors, 'live projection end marker must occur exactly once') if count_literal($text, $end) != 1;
    my $framed_start = "$start\n";
    my $framed_end = "\n$end";
    my $from = index($text, $framed_start);
    my $to = index($text, $framed_end, $from + length($framed_start));
    if ($from < 0 || $to < 0) {
        problem($errors, 'live projection marker framing is malformed');
        return;
    }
    my $block = substr($text, $from + length($framed_start), $to - $from - length($framed_start));
    my $block_raw = encode('UTF-8', $block);
    compare_identity($block_raw, $declared, 'live projection', $errors, [qw(lines bytes sha256)]);
    problem($errors, 'live projection required label differs')
        if index($block, $declared->{required_literal} // '') < 0;
    for my $i (0 .. $#$artifacts) {
        my $artifact = $artifacts->[$i];
        my $row = "  - `$artifact->{display_name}` (`$artifact->{stage}`): `$artifact->{score}/100 $artifact->{grade}` from `$artifact->{artifact_path}`";
        problem($errors, "live projection artifact row $i differs") if count_literal($block, $row) != 1;
    }
    problem($errors, 'live projection execution-summary count differs')
        if $block !~ /^- Rescan execution summaries: \Q$rescan->{execution_summaries}\E(?:\s|$)/m;
    my ($queue) = $block =~ /^- Targeted rescan queue:\n(.*)\z/ms;
    if (!defined $queue) {
        problem($errors, 'live projection queue is missing');
    } else {
        my @rows = ($queue =~ /^  - `[^\n]+$/mg);
        problem($errors, 'live projection queue row count differs') if @rows != $rescan->{live_queue_rows};
        my $remainder = "  - ... and $rescan->{live_queue_remainder} more targeted rescan recommendation(s)";
        problem($errors, 'live projection queue remainder differs') if count_literal($queue, $remainder) != 1;
    }
}

sub validate_producer {
    my ($base, $path, $regions, $errors) = @_;
    my $absolute = absolute($base, $path);
    if (!-f $absolute) {
        problem($errors, "producer '$path' is missing");
        return;
    }
    my $raw = read_raw($absolute);
    my $text = decoded($raw, $path, $errors);
    return if !defined $text;
    for my $region (@$regions) {
        my $start = $region->{start_literal} // '';
        my $end = $region->{end_before_literal} // '';
        if (count_literal($text, $start) != 1 || count_literal($text, $end) != 1) {
            problem($errors, "producer region '$region->{id}' boundary differs");
            next;
        }
        my $from = index($text, $start);
        my $to = index($text, $end, $from + length($start));
        if ($from < 0 || $to <= $from) {
            problem($errors, "producer region '$region->{id}' is malformed");
            next;
        }
        my $region_raw = encode('UTF-8', substr($text, $from, $to - $from));
        compare_identity($region_raw, $region, "producer region '$region->{id}'", $errors, [qw(lines bytes sha256)]);
    }
}

sub validate_review_evidence {
    my ($base, $path, $review, $errors) = @_;
    my $absolute = absolute($base, $path);
    if (!-f $absolute) {
        problem($errors, "review evidence '$path' is missing");
        return;
    }
    my $text = decoded(read_raw($absolute), $path, $errors);
    return if !defined $text;
    problem($errors, 'review-gate evidence literal differs')
        if count_literal($text, $review->{evidence_literal} // '') != 1;
}

sub validate_review_history {
    my ($base, $snapshot_path, $review, $errors) = @_;
    my $commit = $review->{source_commit} // '';
    return if $commit !~ /^[0-9a-f]{40}$/ || !safe_relative_path($snapshot_path);
    my $spec = "$commit:$snapshot_path";
    open my $fh, '-|', 'git', '-C', $base, 'show', $spec or do {
        problem($errors, 'cannot open reviewed snapshot from Git');
        return;
    };
    binmode $fh, ':raw';
    local $/;
    my $raw = <$fh> // '';
    my $closed = close $fh;
    if (!$closed) {
        problem($errors, 'reviewed snapshot Git object is missing');
        return;
    }
    problem($errors, 'reviewed source snapshot identity differs')
        if sha256_hex($raw) ne ($review->{source_snapshot_sha256} // '');
}

sub compare_identity {
    my ($raw, $declared, $label, $errors, $keys) = @_;
    $keys //= [qw(lines bytes line_bytes sha256)];
    my $actual = metrics($raw);
    for my $key (@$keys) {
        problem($errors, "$label $key differs")
            if !defined($declared->{$key}) || $actual->{$key} ne $declared->{$key};
    }
}

sub metrics {
    my ($raw) = @_;
    my $lines = () = $raw =~ /\n/g;
    my $line_bytes = 0;
    for my $line (split /\n/, $raw, -1) {
        $line_bytes = length($line) if length($line) > $line_bytes;
    }
    return {lines => $lines, bytes => length($raw), line_bytes => $line_bytes, sha256 => sha256_hex($raw)};
}

sub run_self_test {
    my ($base, $relative_contract) = @_;
    my $fixture_root = absolute($base, "generated/.validation-snapshot-currentness-test.$$");
    die "validation-snapshot-currentness self-test: unsafe fixture root\n"
        if index($fixture_root, absolute($base, 'generated/.validation-snapshot-currentness-test.')) != 0;
    my $preclean_error = cleanup_fixture_root($fixture_root);
    die "validation-snapshot-currentness self-test: pre-clean failed: $preclean_error\n"
        if $preclean_error ne '';
    make_path($fixture_root);
    my $source_contract = load_json(absolute($base, $relative_contract));
    my @cases = (
        ['valid reviewed boundary', 1, sub {}],
        ['snapshot identity drift', 0, sub {
            my ($fixture, $contract) = @_;
            append_raw(absolute($fixture, $contract->{snapshot}{path}), "\nDRIFT\n");
        }],
        ['report fingerprint drift', 0, sub {
            my ($fixture, $contract) = @_;
            replace_once(absolute($fixture, $contract->{snapshot}{path}), $contract->{artifacts}[0]{artifact_fingerprint}, '0000000000000000');
        }],
        ['recommendation count drift', 0, sub {
            my ($fixture, $contract) = @_;
            replace_once(absolute($fixture, $contract->{snapshot}{path}), '- Targeted rescan recommendations: 29', '- Targeted rescan recommendations: 28');
        }],
        ['live projection drift', 0, sub {
            my ($fixture, $contract) = @_;
            replace_once(absolute($fixture, $contract->{live_projection}{path}), 'Last reviewed projected validation snapshot', 'Drifted validation snapshot');
        }],
        ['producer region drift', 0, sub {
            my ($fixture, $contract) = @_;
            replace_once(
                absolute($fixture, $contract->{producer}{path}),
                'let mut lines = vec!["- Last reviewed projected validation snapshot:".to_string()];',
                'let mut lines = vec!["- Drifted projected validation snapshot:".to_string()];',
            );
        }],
        ['review evidence drift', 0, sub {
            my ($fixture, $contract) = @_;
            replace_once(absolute($fixture, $contract->{review_boundary}{evidence_path}), $contract->{review_boundary}{evidence_literal}, 'review evidence removed');
        }],
        ['unsafe artifact path', 0, sub {
            my ($fixture, $contract) = @_;
            $contract->{artifacts}[0]{artifact_path} = '../outside.json';
            write_contract($fixture, $relative_contract, $contract);
        }],
        ['duplicate artifact identity', 0, sub {
            my ($fixture, $contract) = @_;
            $contract->{artifacts}[1] = JSON::PP->new->decode(JSON::PP->new->encode($contract->{artifacts}[0]));
            write_contract($fixture, $relative_contract, $contract);
        }],
        ['unknown schema', 0, sub {
            my ($fixture, $contract) = @_;
            $contract->{schema_version} = 2;
            write_contract($fixture, $relative_contract, $contract);
        }],
    );
    my $passed = 0;
    my $test_error = '';
    eval {
        for my $i (0 .. $#cases) {
            my ($name, $expected_ok, $mutate) = @{$cases[$i]};
            my $fixture = File::Spec->catdir($fixture_root, sprintf('case-%02d', $i + 1));
            seed_fixture($base, $fixture, $relative_contract, $source_contract);
            my $contract = load_json(absolute($fixture, $relative_contract));
            $mutate->($fixture, $contract);
            $contract = load_json(absolute($fixture, $relative_contract));
            my @errors = validate_contract($fixture, $contract, 1);
            my $actual_ok = @errors ? 0 : 1;
            if ($actual_ok != $expected_ok) {
                die "validation-snapshot-currentness self-test '$name' expected $expected_ok, got $actual_ok: "
                    . join('; ', @errors) . "\n";
            }
            $passed++;
        }
        1;
    } or $test_error = $@ || 'unknown self-test failure';
    my $cleanup_error = cleanup_fixture_root($fixture_root);
    die "validation-snapshot-currentness self-test: cleanup failed: $cleanup_error\n"
        if $cleanup_error ne '';
    die $test_error if $test_error ne '';
    die "validation-snapshot-currentness self-test: expected 10 cases, passed $passed\n" if $passed != 10;
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

sub seed_fixture {
    my ($base, $fixture, $relative_contract, $contract) = @_;
    my @paths = (
        $relative_contract,
        $contract->{snapshot}{path},
        $contract->{live_projection}{path},
        $contract->{producer}{path},
        $contract->{review_boundary}{evidence_path},
    );
    for my $path (@paths) {
        my $destination = absolute($fixture, $path);
        make_path(dirname($destination));
        write_raw($destination, read_raw(absolute($base, $path)));
    }
}

sub write_contract {
    my ($fixture, $relative_contract, $contract) = @_;
    write_raw(absolute($fixture, $relative_contract), JSON::PP->new->canonical->pretty->encode($contract));
}

sub replace_once {
    my ($path, $from, $to) = @_;
    my $raw = read_raw($path);
    my $count = count_literal($raw, $from);
    die "validation-snapshot-currentness self-test: replacement '$from' count is $count\n" if $count != 1;
    substr($raw, index($raw, $from), length($from), $to);
    write_raw($path, $raw);
}

sub append_raw {
    my ($path, $suffix) = @_;
    write_raw($path, read_raw($path) . $suffix);
}

sub object {
    my ($value, $label, $errors) = @_;
    if (ref($value) ne 'HASH') {
        problem($errors, "$label must be an object");
        return {};
    }
    return $value;
}

sub object_array {
    my ($value, $label, $errors, $max) = @_;
    if (ref($value) ne 'ARRAY') {
        problem($errors, "$label must be an array");
        return [];
    }
    problem($errors, "$label exceeds $max entries") if @$value > $max;
    for my $item (@$value) {
        problem($errors, "$label entries must be objects") if ref($item) ne 'HASH';
    }
    return [grep { ref($_) eq 'HASH' } @$value];
}

sub string_array {
    my ($value, $label, $errors, $max) = @_;
    if (ref($value) ne 'ARRAY') {
        problem($errors, "$label must be an array");
        return [];
    }
    problem($errors, "$label exceeds $max entries") if @$value > $max;
    for my $item (@$value) {
        problem($errors, "$label entries must be non-empty strings")
            if !defined($item) || ref($item) || $item eq '' || length(encode('UTF-8', $item)) > 512;
    }
    return [grep { defined($_) && !ref($_) && $_ ne '' } @$value];
}

sub scalar_string {
    my ($value, $label, $errors) = @_;
    if (!defined($value) || ref($value) || $value eq '' || length(encode('UTF-8', $value)) > 1024) {
        problem($errors, "$label must be a bounded non-empty string");
        return undef;
    }
    return $value;
}

sub safe_path_value {
    my ($value, $label, $errors) = @_;
    my $path = scalar_string($value, $label, $errors);
    return undef if !defined $path;
    if (!safe_relative_path($path)) {
        problem($errors, "$label must be a safe repository-relative path");
        return undef;
    }
    return $path;
}

sub safe_relative_path {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '' || length($path) > 512;
    return 0 if $path =~ m{^/} || $path =~ /\\/ || $path =~ /\0/;
    my @parts = split m{/}, $path, -1;
    return 0 if grep { $_ eq '' || $_ eq '.' || $_ eq '..' } @parts;
    return 1;
}

sub positive_integer {
    my ($value, $label, $errors) = @_;
    problem($errors, "$label must be a positive integer")
        if !defined($value) || ref($value) || $value !~ /^\d+$/ || $value < 1;
}

sub nonnegative_integer {
    my ($value, $label, $errors) = @_;
    problem($errors, "$label must be a nonnegative integer")
        if !defined($value) || ref($value) || $value !~ /^\d+$/;
}

sub bounded_integer {
    my ($value, $label, $min, $max, $errors) = @_;
    problem($errors, "$label must be an integer from $min through $max")
        if !defined($value) || ref($value) || $value !~ /^\d+$/ || $value < $min || $value > $max;
}

sub sha_value {
    my ($value, $label, $errors) = @_;
    problem($errors, "$label must be a SHA-256 hex digest") if ($value // '') !~ /^[0-9a-f]{64}$/;
}

sub unknown_fields {
    my ($object, $label, $allowed, $errors) = @_;
    return if ref($object) ne 'HASH';
    my %allowed = map { $_ => 1 } @$allowed;
    problem($errors, "$label has unknown field '$_'") for grep { !$allowed{$_} } keys %$object;
}

sub count_literal {
    my ($text, $literal) = @_;
    return 0 if !defined($literal) || $literal eq '';
    my ($count, $offset) = (0, 0);
    while (($offset = index($text, $literal, $offset)) >= 0) {
        $count++;
        $offset += length($literal);
    }
    return $count;
}

sub problem {
    my ($errors, $message) = @_;
    push @$errors, $message;
}

sub absolute {
    my ($base, $relative) = @_;
    return File::Spec->catfile($base, split m{/}, $relative);
}

sub read_raw {
    my ($path) = @_;
    open my $fh, '<:raw', $path or die "validation-snapshot-currentness: cannot read $path: $!\n";
    local $/;
    my $raw = <$fh> // '';
    close $fh;
    return $raw;
}

sub decoded {
    my ($raw, $label, $errors) = @_;
    my $text = eval { decode('UTF-8', $raw, FB_CROAK) };
    if (!defined $text) {
        problem($errors, "$label is not valid UTF-8");
        return undef;
    }
    return $text;
}

sub load_json {
    my ($path) = @_;
    my $raw = read_raw($path);
    die "validation-snapshot-currentness: contract exceeds 32,768 bytes\n" if length($raw) > 32_768;
    my $value = eval { JSON::PP->new->decode($raw) };
    die "validation-snapshot-currentness: invalid JSON at $path: $@\n" if !defined $value;
    return $value;
}

sub write_raw {
    my ($path, $raw) = @_;
    make_path(dirname($path));
    open my $fh, '>:raw', $path or die "validation-snapshot-currentness self-test: cannot write $path: $!\n";
    print {$fh} $raw;
    close $fh or die "validation-snapshot-currentness self-test: cannot close $path: $!\n";
}
