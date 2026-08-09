#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(decode encode FB_CROAK);
use File::Basename qw(basename dirname);
use File::Path qw(make_path remove_tree);
use File::Spec;
use JSON::PP;

my $root = abs_path(File::Spec->catdir(dirname(__FILE__), '..'));
my $contract_path = 'doctrine/live_document_size/fsmgen_feedback.json';
my $mode = 'check';

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        @ARGV or usage();
        $root = abs_path(shift @ARGV) // die "feedback-protocol: invalid --root\n";
    } elsif ($arg eq '--contract') {
        @ARGV or usage();
        $contract_path = shift @ARGV;
    } elsif ($arg eq '--check' || $arg eq '--report' || $arg eq '--self-test') {
        $mode = substr($arg, 2);
    } else {
        usage();
    }
}

sub usage {
    die "Usage: scripts/check_fsmgen_feedback_protocol.pl [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n";
}

sub absolute {
    my ($base, $path) = @_;
    return File::Spec->catfile($base, split m{/}, $path);
}

sub safe_relative_path {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '';
    return 0 if $path =~ m{\A/} || $path =~ /\\/ || $path =~ /\0/;
    return 0 if $path =~ m{(?:\A|/)\.{1,2}(?:/|\z)};
    return 0 if $path =~ m{//};
    return 1;
}

sub read_raw {
    my ($path) = @_;
    open my $fh, '<:raw', $path or die "feedback-protocol: cannot read $path: $!\n";
    local $/;
    return <$fh>;
}

sub decoded {
    my ($raw, $label) = @_;
    return eval { decode('UTF-8', $raw, FB_CROAK) }
        // die "feedback-protocol: $label is not valid UTF-8\n";
}

sub raw_lines {
    my ($raw) = @_;
    my @lines = ($raw =~ /[^\n]*\n|[^\n]+\z/g);
    return @lines;
}

sub metrics {
    my ($raw) = @_;
    my @lines = raw_lines($raw);
    my $maximum = 0;
    for my $line (@lines) {
        $line =~ s/\n\z//;
        $line =~ s/\r\z//;
        my $length = do { use bytes; length($line) };
        $maximum = $length if $length > $maximum;
    }
    return {
        lines      => scalar(@lines),
        bytes      => do { use bytes; length($raw) },
        line_bytes => $maximum,
        sha256     => sha256_hex($raw),
    };
}

sub line_span {
    my ($raw, $start, $end) = @_;
    my @lines = raw_lines($raw);
    return undef if !defined($start) || !defined($end) || $start < 1 || $end < $start || $end > @lines;
    return join '', @lines[$start - 1 .. $end - 1];
}

sub count_literal {
    my ($text, $literal) = @_;
    return 0 if !defined($literal) || $literal eq '';
    my $count = 0;
    my $offset = 0;
    while (($offset = index($text, $literal, $offset)) >= 0) {
        $count++;
        $offset += length($literal) || 1;
    }
    return $count;
}

sub problem {
    my ($errors, $message) = @_;
    push @$errors, $message;
}

sub required_hash {
    my ($value, $label, $errors) = @_;
    if (ref($value) ne 'HASH') {
        problem($errors, "$label must be an object");
        return {};
    }
    return $value;
}

sub required_array {
    my ($value, $label, $errors) = @_;
    if (ref($value) ne 'ARRAY') {
        problem($errors, "$label must be an array");
        return [];
    }
    return $value;
}

sub validate_safe_path {
    my ($path, $label, $errors) = @_;
    problem($errors, "$label must be one safe repository-relative path") if !safe_relative_path($path);
}

sub validate_metric_contract {
    my ($actual, $expected, $label, $errors, $with_sha) = @_;
    for my $key (qw(lines bytes line_bytes)) {
        if (!defined($expected->{$key}) || ref($expected->{$key}) || $expected->{$key} !~ /\A\d+\z/) {
            problem($errors, "$label has invalid $key");
            next;
        }
        problem($errors, "$label $key differs: $actual->{$key} != $expected->{$key}")
            if $actual->{$key} != $expected->{$key};
    }
    if ($with_sha) {
        if (!defined($expected->{sha256}) || ref($expected->{sha256}) || $expected->{sha256} !~ /\A[0-9a-f]{64}\z/) {
            problem($errors, "$label has invalid sha256");
        } elsif ($actual->{sha256} ne $expected->{sha256}) {
            problem($errors, "$label sha256 differs: $actual->{sha256} != $expected->{sha256}");
        }
    }
}

sub load_json {
    my ($path) = @_;
    my $raw = read_raw($path);
    return eval { JSON::PP->new->utf8->decode($raw) }
        // die "feedback-protocol: invalid JSON in $path: $@\n";
}

sub validate_paths {
    my ($contract, $errors) = @_;
    my $source = required_hash($contract->{source}, 'source', $errors);
    validate_safe_path($source->{path}, 'source.path', $errors);
    my $archive = required_hash($contract->{archive}, 'archive', $errors);
    validate_safe_path($archive->{index}, 'archive.index', $errors);
    validate_safe_path($archive->{manifest}, 'archive.manifest', $errors);
    validate_safe_path($archive->{source_capsule}, 'archive.source_capsule', $errors);

    my %seen;
    for my $record (@{required_array($contract->{records}, 'records', $errors)}) {
        next if ref($record) ne 'HASH';
        my $id = $record->{id} // '';
        problem($errors, "record has invalid id '$id'") if $id !~ /\A[a-z0-9]+(?:-[a-z0-9]+)*\z/;
        problem($errors, "record id '$id' is duplicated") if $seen{$id}++;
        for my $evidence (@{required_array($record->{evidence}, "record '$id' evidence", $errors)}) {
            next if ref($evidence) ne 'HASH';
            validate_safe_path($evidence->{path}, "record '$id' evidence path", $errors);
        }
    }
    my $consumers = required_hash($contract->{consumers}, 'consumers', $errors);
    for my $path (@{required_array($consumers->{known_paths}, 'consumers.known_paths', $errors)}) {
        validate_safe_path($path, 'consumer path', $errors);
    }
    for my $item (@{required_array($consumers->{required_literals}, 'consumers.required_literals', $errors)}) {
        validate_safe_path($item->{path}, 'consumer literal path', $errors) if ref($item) eq 'HASH';
    }
    for my $finding (@{required_array($contract->{current_truth_findings}, 'current_truth_findings', $errors)}) {
        validate_safe_path($finding->{evidence_path}, 'finding evidence path', $errors) if ref($finding) eq 'HASH';
    }
}

sub validate_source {
    my ($base, $contract, $errors) = @_;
    my $source = required_hash($contract->{source}, 'source', $errors);
    my $archive = required_hash($contract->{archive}, 'archive', $errors);
    my $state = $contract->{migration_state} // '';
    my $authority = $state eq 'migrated' ? $archive->{source_capsule} : $source->{path};
    return if !safe_relative_path($authority);
    my $absolute = absolute($base, $authority);
    if (!-f $absolute) {
        problem($errors, "source authority '$authority' is missing");
        return;
    }
    my $raw = read_raw($absolute);
    my $text = eval { decoded($raw, $authority) };
    if (!defined $text) {
        problem($errors, "source authority '$authority' is not valid UTF-8");
        return;
    }
    my $actual = metrics($raw);
    validate_metric_contract($actual, $source, "source authority '$authority'", $errors, 1);

    my $regions = required_array($source->{regions}, 'source.regions', $errors);
    my $expected_start = 1;
    my $reconstructed = '';
    my %region_ids;
    for my $region (@$regions) {
        if (ref($region) ne 'HASH') {
            problem($errors, 'source region must be an object');
            next;
        }
        my $id = $region->{id} // '';
        problem($errors, "source region id '$id' is invalid") if $id !~ /\A[a-z0-9_]+\z/;
        problem($errors, "source region id '$id' is duplicated") if $region_ids{$id}++;
        problem($errors, "source region '$id' must start at line $expected_start")
            if ($region->{start_line} // 0) != $expected_start;
        my $chunk = line_span($raw, $region->{start_line}, $region->{end_line});
        if (!defined $chunk) {
            problem($errors, "source region '$id' has an invalid line span");
            next;
        }
        my $chunk_metrics = metrics($chunk);
        validate_metric_contract($chunk_metrics, $region, "source region '$id'", $errors, 1);
        $reconstructed .= $chunk;
        $expected_start = ($region->{end_line} // 0) + 1;
    }
    problem($errors, 'source regions do not reconstruct the exact source') if $reconstructed ne $raw;

    my %record_ids;
    for my $record (@{required_array($contract->{records}, 'records', $errors)}) {
        next if ref($record) ne 'HASH';
        my $id = $record->{id} // '<unknown>';
        my $chunk = line_span($raw, $record->{start_line}, $record->{end_line});
        if (!defined $chunk) {
            problem($errors, "record '$id' has an invalid line span");
            next;
        }
        my $chunk_text = eval { decoded($chunk, "record '$id'") } // '';
        validate_metric_contract(metrics($chunk), $record, "record '$id'", $errors, 1);
        my $heading = $record->{heading} // '';
        problem($errors, "record '$id' heading does not begin its source span")
            if $chunk_text !~ /\A\Q$heading\E(?:\n|\z)/;
        my $status_literal = $record->{source_status_literal} // '';
        problem($errors, "record '$id' source status literal is missing")
            if $status_literal eq '' || index($chunk_text, $status_literal) < 0;
        for my $field (qw(kind direction status)) {
            my $value = $record->{$field} // '';
            problem($errors, "record '$id' has invalid $field") if $value !~ /\A[a-z][a-z0-9_]*\z/;
        }
        for my $evidence (@{required_array($record->{evidence}, "record '$id' evidence", $errors)}) {
            next if ref($evidence) ne 'HASH' || !safe_relative_path($evidence->{path});
            my $path = absolute($base, $evidence->{path});
            if (!-f $path) {
                problem($errors, "record '$id' evidence '$evidence->{path}' is missing");
                next;
            }
            my $evidence_text = eval { decoded(read_raw($path), $evidence->{path}) } // '';
            my $literal = $evidence->{literal} // '';
            problem($errors, "record '$id' evidence literal is missing from '$evidence->{path}'")
                if $literal eq '' || index($evidence_text, $literal) < 0;
        }
    }

    for my $finding (@{required_array($contract->{current_truth_findings}, 'current_truth_findings', $errors)}) {
        next if ref($finding) ne 'HASH';
        my $id = $finding->{id} // '<unknown>';
        my $source_literal = $finding->{source_literal} // '';
        problem($errors, "finding '$id' source literal is absent")
            if $source_literal eq '' || index($text, $source_literal) < 0;
        next if !safe_relative_path($finding->{evidence_path});
        my $path = absolute($base, $finding->{evidence_path});
        if (!-f $path) {
            problem($errors, "finding '$id' evidence path is missing");
            next;
        }
        my $evidence_text = eval { decoded(read_raw($path), $finding->{evidence_path}) } // '';
        my $literal = $finding->{evidence_literal} // '';
        problem($errors, "finding '$id' evidence literal is absent")
            if $literal eq '' || index($evidence_text, $literal) < 0;
    }
}

sub validate_consumers {
    my ($base, $contract, $errors) = @_;
    my $consumers = required_hash($contract->{consumers}, 'consumers', $errors);
    my %seen;
    for my $path (@{required_array($consumers->{known_paths}, 'consumers.known_paths', $errors)}) {
        next if !safe_relative_path($path);
        problem($errors, "consumer '$path' is duplicated") if $seen{$path}++;
        my $absolute = absolute($base, $path);
        if (!-f $absolute) {
            problem($errors, "consumer '$path' is missing");
            next;
        }
        my $text = eval { decoded(read_raw($absolute), $path) } // '';
        problem($errors, "consumer '$path' no longer names docs/FSMGEN_FEEDBACK.md")
            if index($text, 'docs/FSMGEN_FEEDBACK.md') < 0;
    }
    for my $item (@{required_array($consumers->{required_literals}, 'consumers.required_literals', $errors)}) {
        next if ref($item) ne 'HASH' || !safe_relative_path($item->{path});
        my $absolute = absolute($base, $item->{path});
        if (!-f $absolute) {
            problem($errors, "required consumer '$item->{path}' is missing");
            next;
        }
        my $text = eval { decoded(read_raw($absolute), $item->{path}) } // '';
        my $literal = $item->{literal} // '';
        problem($errors, "required consumer literal is missing from '$item->{path}'")
            if $literal eq '' || index($text, $literal) < 0;
    }
}

sub between_markers {
    my ($text, $start, $end, $label, $errors) = @_;
    my $start_count = count_literal($text, $start);
    my $end_count = count_literal($text, $end);
    problem($errors, "$label start marker must occur exactly once") if $start_count != 1;
    problem($errors, "$label end marker must occur exactly once") if $end_count != 1;
    return '' if $start_count != 1 || $end_count != 1;
    my $start_at = index($text, $start) + length($start);
    my $end_at = index($text, $end);
    if ($end_at < $start_at) {
        problem($errors, "$label markers are reversed");
        return '';
    }
    return substr($text, $start_at, $end_at - $start_at);
}

sub direction_display {
    my ($direction) = @_;
    return 'SpecForge → FSMGen' if $direction eq 'specforge_to_fsmgen';
    return 'FSMGen → SpecForge' if $direction eq 'fsmgen_to_specforge';
    return $direction;
}

sub validate_current_root {
    my ($base, $contract, $errors) = @_;
    my $source = required_hash($contract->{source}, 'source', $errors);
    my $current = required_hash($contract->{current_root}, 'current_root', $errors);
    return if !safe_relative_path($source->{path});
    my $path = absolute($base, $source->{path});
    if (!-f $path) {
        problem($errors, "current feedback root '$source->{path}' is missing");
        return;
    }
    my $raw = read_raw($path);
    my $text = eval { decoded($raw, $source->{path}) } // '';
    my $actual = metrics($raw);
    my $limits = required_hash($current->{enforcement_ceilings}, 'current_root.enforcement_ceilings', $errors);
    for my $key (qw(lines bytes line_bytes)) {
        problem($errors, "current feedback root exceeds $key ceiling: $actual->{$key} > $limits->{$key}")
            if defined($limits->{$key}) && $actual->{$key} > $limits->{$key};
    }
    my ($first) = split /\n/, $text, 2;
    problem($errors, 'current feedback root H1 differs') if $first ne ($current->{h1} // '');
    my @h2 = ($text =~ /^## (.+)$/mg);
    my @expected_h2 = @{required_array($current->{required_h2_order}, 'current_root.required_h2_order', $errors)};
    problem($errors, 'current feedback root H2 order differs')
        if join("\n", @h2) ne join("\n", @expected_h2);
    for my $literal (@{required_array($current->{required_links}, 'current_root.required_links', $errors)},
                     @{required_array($current->{required_literals}, 'current_root.required_literals', $errors)}) {
        problem($errors, "current feedback root lacks required literal '$literal'")
            if count_literal($text, $literal) < 1;
    }
    for my $literal (@{required_array($current->{forbidden_literals}, 'current_root.forbidden_literals', $errors)}) {
        problem($errors, "current feedback root retains forbidden historical literal '$literal'")
            if count_literal($text, $literal) > 0;
    }

    my $open = between_markers(
        $text, $current->{open_start_marker} // '', $current->{open_end_marker} // '',
        'open correspondence', $errors,
    );
    my @open_ids = ($open =~ /^### ([a-z0-9]+(?:-[a-z0-9]+)*)\b/mg);
    my @expected_open = @{required_array($current->{open_record_ids}, 'current_root.open_record_ids', $errors)};
    problem($errors, 'current feedback open-record ids differ')
        if join("\n", @open_ids) ne join("\n", @expected_open);
    my $open_limits = required_hash($current->{open_record_limits}, 'current_root.open_record_limits', $errors);
    problem($errors, 'current feedback has too many open records')
        if defined($open_limits->{records}) && @open_ids > $open_limits->{records};
    my @open_chunks = split /(?=^### [a-z0-9]+(?:-[a-z0-9]+)*\b)/m, $open;
    @open_chunks = grep { /^### /m } @open_chunks;
    my %allowed_direction = map { $_ => 1 }
        @{required_array($current->{open_direction_values}, 'current_root.open_direction_values', $errors)};
    my %allowed_kind = map { $_ => 1 }
        @{required_array($current->{open_kind_values}, 'current_root.open_kind_values', $errors)};
    my %allowed_status = map { $_ => 1 }
        @{required_array($current->{open_status_values}, 'current_root.open_status_values', $errors)};
    for my $chunk (@open_chunks) {
        my ($id) = $chunk =~ /^### ([a-z0-9]+(?:-[a-z0-9]+)*)\b/m;
        my $chunk_metrics = metrics(encode('UTF-8', $chunk));
        for my $pair ([lines => 'lines_each'], [bytes => 'bytes_each'], [line_bytes => 'line_bytes_each']) {
            my ($actual_key, $limit_key) = @$pair;
            problem($errors, "open record '$id' exceeds $limit_key")
                if defined($open_limits->{$limit_key}) && $chunk_metrics->{$actual_key} > $open_limits->{$limit_key};
        }
        my %fields;
        while ($chunk =~ /^- ([A-Za-z]+):\s+`?([^`\n]+?)`?\s*$/mg) {
            $fields{$1} = $2;
        }
        for my $field (@{required_array($current->{open_required_fields}, 'current_root.open_required_fields', $errors)}) {
            problem($errors, "open record '$id' lacks $field") if !defined($fields{$field}) || $fields{$field} eq '';
        }
        problem($errors, "open record '$id' has unknown Direction")
            if defined($fields{Direction}) && !$allowed_direction{$fields{Direction}};
        problem($errors, "open record '$id' has unknown Kind")
            if defined($fields{Kind}) && !$allowed_kind{$fields{Kind}};
        problem($errors, "open record '$id' has unknown Status")
            if defined($fields{Status}) && !$allowed_status{$fields{Status}};
    }

    my $register = between_markers(
        $text, $current->{register_start_marker} // '', $current->{register_end_marker} // '',
        'closed correspondence register', $errors,
    );
    my @actual_rows;
    while ($register =~ /^\| ([a-z0-9]+(?:-[a-z0-9]+)*) \| ([a-z][a-z0-9_]*) \| ([^|]+?) \|/mg) {
        push @actual_rows, [$1, $2, $3];
    }
    my @expected_rows = map {
        [$_->{id} // '', $_->{status} // '', direction_display($_->{direction} // '')]
    } @{required_array($contract->{records}, 'records', $errors)};
    problem($errors, 'current feedback closed-register row count differs') if @actual_rows != @expected_rows;
    for my $i (0 .. $#expected_rows) {
        last if $i > $#actual_rows;
        problem($errors, "current feedback closed-register row $i differs")
            if join("\t", @{$actual_rows[$i]}) ne join("\t", @{$expected_rows[$i]});
    }
}

sub validate_archive {
    my ($base, $contract, $errors) = @_;
    my $source = required_hash($contract->{source}, 'source', $errors);
    my $archive = required_hash($contract->{archive}, 'archive', $errors);
    return if !safe_relative_path($archive->{index}) || !safe_relative_path($archive->{manifest});
    my $index_path = absolute($base, $archive->{index});
    if (!-f $index_path) {
        problem($errors, "feedback archive index '$archive->{index}' is missing");
    } else {
        my $raw = read_raw($index_path);
        my $text = eval { decoded($raw, $archive->{index}) } // '';
        my $actual = metrics($raw);
        my $limits = required_hash($archive->{index_limits}, 'archive.index_limits', $errors);
        for my $key (qw(lines bytes line_bytes)) {
            problem($errors, "feedback archive index exceeds $key ceiling")
                if defined($limits->{$key}) && $actual->{$key} > $limits->{$key};
        }
        for my $literal ($source->{path}, basename($archive->{source_capsule}), basename($archive->{manifest})) {
            problem($errors, "feedback archive index lacks '$literal'") if index($text, $literal) < 0;
        }
    }

    my $manifest_path = absolute($base, $archive->{manifest});
    if (!-f $manifest_path) {
        problem($errors, "feedback archive manifest '$archive->{manifest}' is missing");
        return;
    }
    my $manifest = eval { load_json($manifest_path) };
    if (!$manifest || ref($manifest) ne 'HASH') {
        problem($errors, 'feedback archive manifest is invalid JSON');
        return;
    }
    my %expected = (
        schema_version => 1,
        current_path   => $source->{path},
        source_capsule => $archive->{source_capsule},
        source_sha256  => $source->{sha256},
        sealed_date    => $archive->{sealed_date},
        reason         => $archive->{reason},
        verifier       => $archive->{verifier},
    );
    for my $key (sort keys %expected) {
        problem($errors, "feedback archive manifest $key differs")
            if !defined($manifest->{$key}) || ref($manifest->{$key}) || $manifest->{$key} ne $expected{$key};
    }
    my $manifest_metrics = required_hash($manifest->{source_metrics}, 'manifest.source_metrics', $errors);
    for my $key (qw(lines bytes line_bytes)) {
        problem($errors, "feedback archive manifest source_metrics.$key differs")
            if !defined($manifest_metrics->{$key}) || $manifest_metrics->{$key} != $source->{$key};
    }
    my %allowed = map { $_ => 1 } (keys(%expected), 'source_metrics');
    for my $key (keys %$manifest) {
        problem($errors, "feedback archive manifest has unknown field '$key'") if !$allowed{$key};
    }
}

sub validate_contract {
    my ($base, $contract) = @_;
    my @errors;
    if (ref($contract) ne 'HASH') {
        return ('contract root must be an object');
    }
    problem(\@errors, 'schema_version must be 1') if ($contract->{schema_version} // 0) != 1;
    my $state = $contract->{migration_state} // '';
    problem(\@errors, "migration_state '$state' is invalid") if $state ne 'planned' && $state ne 'migrated';
    validate_paths($contract, \@errors);
    validate_source($base, $contract, \@errors);
    validate_consumers($base, $contract, \@errors);
    if ($state eq 'migrated') {
        validate_current_root($base, $contract, \@errors);
        validate_archive($base, $contract, \@errors);
    }
    return @errors;
}

sub write_raw {
    my ($path, $raw) = @_;
    make_path(dirname($path));
    open my $fh, '>:raw', $path or die "feedback-protocol self-test: cannot write $path: $!\n";
    $raw = encode('UTF-8', $raw) if utf8::is_utf8($raw);
    print {$fh} $raw;
    close $fh or die "feedback-protocol self-test: cannot close $path: $!\n";
}

sub clone_json {
    my ($value) = @_;
    return JSON::PP->new->decode(JSON::PP->new->canonical->encode($value));
}

sub render_current_root {
    my ($contract) = @_;
    my $current = $contract->{current_root};
    my $required_literals = $current->{required_literals};
    ref($required_literals) eq 'ARRAY'
        or die "feedback-protocol self-test: current_root.required_literals must be an array\n";
    my @out = ($current->{h1}, '');
    push @out, '## Current channel', '',
        'The stable channel is `docs/FSMGEN_FEEDBACK.md`; detailed history is in docs/archive/fsmgen-feedback/INDEX.md.', '';
    push @out, '## Current downstream boundary', '',
        "SPECFORGE's single adapter target is now `.isf`.",
        @$required_literals, '';
    push @out, '## Open correspondence', '', $current->{open_start_marker},
        '- None at the sealed source boundary.', $current->{open_end_marker}, '';
    push @out, '## Closed correspondence register', '', $current->{register_start_marker},
        '| Record | Status | Direction | History |',
        '| --- | --- | --- | --- |';
    for my $record (@{$contract->{records}}) {
        push @out, sprintf('| %s | %s | %s | history |',
            $record->{id}, $record->{status}, direction_display($record->{direction}));
    }
    push @out, $current->{register_end_marker}, '';
    push @out, '## Feedback record format', '',
        'New records are task-owned, evidence-linked, explicitly directed, and status-bearing. See TOOLBOX.md.', '';
    push @out, '## History and retrieval', '',
        '- docs/archive/fsmgen-feedback/INDEX.md',
        '- subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md',
        '- docs/catalogs/fsmgen-issue-packets.md',
        '- TOOLBOX.md', '';
    return join("\n", @out);
}

sub seed_fixture {
    my ($fixture, $contract, $source_raw) = @_;
    my %content;
    for my $record (@{$contract->{records}}) {
        for my $evidence (@{$record->{evidence}}) {
            $content{$evidence->{path}} .= $evidence->{literal} . "\n";
        }
    }
    for my $finding (@{$contract->{current_truth_findings}}) {
        next if $finding->{evidence_path} eq $contract->{source}{path};
        $content{$finding->{evidence_path}} .= $finding->{evidence_literal} . "\n";
    }
    for my $path (@{$contract->{consumers}{known_paths}}) {
        $content{$path} .= "docs/FSMGEN_FEEDBACK.md\n";
    }
    for my $item (@{$contract->{consumers}{required_literals}}) {
        $content{$item->{path}} .= $item->{literal} . "\n";
    }
    for my $path (keys %content) {
        write_raw(absolute($fixture, $path), $content{$path});
    }
    write_raw(absolute($fixture, $contract->{source}{path}), $source_raw);
}

sub seed_migrated_files {
    my ($fixture, $contract, $source_raw) = @_;
    write_raw(absolute($fixture, $contract->{archive}{source_capsule}), $source_raw);
    write_raw(absolute($fixture, $contract->{source}{path}), render_current_root($contract));
    my $index = "# Archive\n\n- $contract->{source}{path}\n- "
        . basename($contract->{archive}{source_capsule}) . "\n- "
        . basename($contract->{archive}{manifest}) . "\n";
    write_raw(absolute($fixture, $contract->{archive}{index}), $index);
    my $manifest = {
        schema_version => 1,
        current_path => $contract->{source}{path},
        source_capsule => $contract->{archive}{source_capsule},
        source_sha256 => $contract->{source}{sha256},
        source_metrics => {
            lines => $contract->{source}{lines},
            bytes => $contract->{source}{bytes},
            line_bytes => $contract->{source}{line_bytes},
        },
        sealed_date => $contract->{archive}{sealed_date},
        reason => $contract->{archive}{reason},
        verifier => $contract->{archive}{verifier},
    };
    write_raw(
        absolute($fixture, $contract->{archive}{manifest}),
        JSON::PP->new->canonical->pretty->encode($manifest),
    );
}

sub run_self_test {
    my ($base, $contract) = @_;
    my $fixture = absolute($base, "generated/.fsmgen-feedback-protocol-test.$$" );
    remove_tree($fixture) if -e $fixture;
    make_path($fixture);
    my $state = $contract->{migration_state};
    my $authority = $state eq 'migrated' ? $contract->{archive}{source_capsule} : $contract->{source}{path};
    my $source_raw = read_raw(absolute($base, $authority));
    my $planned = clone_json($contract);
    $planned->{migration_state} = 'planned';
    seed_fixture($fixture, $planned, $source_raw);

    my @cases;
    push @cases, ['valid planned contract', $planned, 1, undef];
    push @cases, ['source identity drift', $planned, 0, sub {
        my $path = absolute($fixture, $planned->{source}{path});
        write_raw($path, $source_raw . "drift\n");
    }];
    push @cases, ['missing response evidence', $planned, 0, sub {
        my $path = absolute($fixture, $planned->{records}[0]{evidence}[0]{path});
        write_raw($path, "unrelated\n");
    }];
    my $bad_heading = clone_json($planned);
    $bad_heading->{records}[0]{heading} = '## Missing heading';
    push @cases, ['record heading drift', $bad_heading, 0, undef];
    my $unsafe = clone_json($planned);
    $unsafe->{archive}{index} = '../outside.md';
    push @cases, ['unsafe archive path', $unsafe, 0, undef];

    my $passed = 0;
    for my $case (@cases) {
        remove_tree($fixture);
        make_path($fixture);
        seed_fixture($fixture, $planned, $source_raw);
        $case->[3]->() if $case->[3];
        my @errors = validate_contract($fixture, $case->[1]);
        my $ok = $case->[2] ? !@errors : !!@errors;
        if (!$ok) {
            remove_tree($fixture);
            die "feedback-protocol self-test '$case->[0]' failed: " . join('; ', @errors) . "\n";
        }
        $passed++;
    }

    my $changed_literals = clone_json($planned);
    $changed_literals->{current_root}{required_literals} = [
        'fixture-declared adapter route',
        'fixture-declared downstream boundary',
    ];
    remove_tree($fixture);
    make_path($fixture);
    seed_fixture($fixture, $changed_literals, $source_raw);
    my @changed_literal_errors = validate_contract($fixture, $changed_literals);
    if (@changed_literal_errors) {
        remove_tree($fixture);
        die "feedback-protocol self-test 'declared current literals drive fixture rendering' failed: "
            . join('; ', @changed_literal_errors) . "\n";
    }
    $passed++;

    my $migrated = clone_json($planned);
    $migrated->{migration_state} = 'migrated';
    my @migrated_cases = (
        ['valid migrated contract', 1, undef],
        ['duplicate open marker', 0, sub {
            my $path = absolute($fixture, $migrated->{source}{path});
            my $raw = read_raw($path);
            write_raw($path, $raw . $migrated->{current_root}{open_start_marker} . "\n");
        }],
        ['missing closed-register row', 0, sub {
            my $path = absolute($fixture, $migrated->{source}{path});
            my $raw = read_raw($path);
            $raw =~ s/^\| enum-type-clarity \|.*\n//m;
            write_raw($path, $raw);
        }],
        ['forbidden historical leakage', 0, sub {
            my $path = absolute($fixture, $migrated->{source}{path});
            my $raw = read_raw($path);
            write_raw($path, $raw . $migrated->{current_root}{forbidden_literals}[0] . "\n");
        }],
        ['manifest identity drift', 0, sub {
            my $path = absolute($fixture, $migrated->{archive}{manifest});
            my $manifest = load_json($path);
            $manifest->{source_sha256} = '0' x 64;
            write_raw($path, JSON::PP->new->canonical->pretty->encode($manifest));
        }],
    );
    for my $case (@migrated_cases) {
        remove_tree($fixture);
        make_path($fixture);
        seed_fixture($fixture, $migrated, $source_raw);
        seed_migrated_files($fixture, $migrated, $source_raw);
        $case->[2]->() if $case->[2];
        my @errors = validate_contract($fixture, $migrated);
        my $ok = $case->[1] ? !@errors : !!@errors;
        if (!$ok) {
            remove_tree($fixture);
            die "feedback-protocol self-test '$case->[0]' failed: " . join('; ', @errors) . "\n";
        }
        $passed++;
    }
    remove_tree($fixture);
    die "feedback-protocol self-test residue remains at $fixture\n" if -e $fixture;
    print "feedback-protocol: self-test $passed/11 passed.\n";
}

my $contract_absolute = absolute($root, $contract_path);
die "feedback-protocol: contract '$contract_path' is missing\n" if !-f $contract_absolute;
my $contract = load_json($contract_absolute);

if ($mode eq 'self-test') {
    run_self_test($root, $contract);
    exit 0;
}

my @errors = validate_contract($root, $contract);
if (@errors) {
    print STDERR "feedback-protocol: $_\n" for @errors;
    print STDERR "feedback-protocol: FAILED with " . scalar(@errors) . " violation(s).\n";
    exit 1;
}

if ($mode eq 'report') {
    my $source = $contract->{source};
    my %status;
    $status{$_->{status}}++ for @{$contract->{records}};
    my $report = {
        migration_state => $contract->{migration_state},
        source => {
            lines => $source->{lines}, bytes => $source->{bytes}, line_bytes => $source->{line_bytes},
            sha256 => $source->{sha256},
        },
        regions => $source->{regions},
        records => scalar(@{$contract->{records}}),
        open_records => scalar(@{$contract->{current_root}{open_record_ids}}),
        status_counts => \%status,
        current_truth_findings => scalar(@{$contract->{current_truth_findings}}),
        consumers => scalar(@{$contract->{consumers}{known_paths}}),
        archive => $contract->{archive},
        current_root_limits => $contract->{current_root}{enforcement_ceilings},
    };
    if ($contract->{migration_state} eq 'migrated') {
        $report->{current_root} = metrics(read_raw(absolute($root, $source->{path})));
    }
    print JSON::PP->new->canonical->encode($report), "\n";
} else {
    print "feedback-protocol: exact source, six closed exchanges, evidence, consumers, and $contract->{migration_state} lifecycle satisfy the bounded channel design.\n";
}
