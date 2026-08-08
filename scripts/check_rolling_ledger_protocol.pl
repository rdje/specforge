#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(decode_utf8);
use File::Basename qw(dirname);
use File::Spec;
use JSON::PP;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root;
my $registry_rel = 'doctrine/live_document_size/rolling_ledgers.jsonl';
my $report = 0;
my $self_test = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--report') {
        $report = 1;
    } elsif ($arg eq '--self-test') {
        $self_test = 1;
    } else {
        usage();
    }
}

$root //= File::Spec->catdir(dirname(abs_path($0)), '..');
$root = abs_path($root) // die "rolling-ledger: repository root does not exist\n";

my @errors;
run_self_test() if $self_test;
exit 0 if $self_test;

my $json = JSON::PP->new->canonical(1);
my %valid_grammar = map { $_ => 1 } qw(changes_mixed_v1 h2_records_v1 current_snapshot_bullets_v1);
my %valid_state = map { $_ => 1 } qw(planned migrated);

my ($meta, $ledgers) = read_registry(absolute($registry_rel));
my %seen;
for my $ledger (@$ledgers) {
    validate_ledger_schema($ledger);
    my $id = scalar_field($ledger, 'ledger_id', 'ledger record');
    next if !defined $id;
    problem("duplicate ledger_id '$id'") if $seen{$id}++;
    problem("ledger_id '$id' has an invalid identifier shape")
        if $id !~ /\A[a-z0-9][a-z0-9._-]*\z/;
    validate_ledger($ledger, $id);
}

if (@errors) {
    print STDERR "rolling-ledger: $_\n" for @errors;
    print STDERR "rolling-ledger: FAILED with ", scalar(@errors), " violation(s).\n";
    exit 1;
}

print "rolling-ledger: ", scalar(@$ledgers),
    " ledgers satisfy the lossless live-window/archive protocol.\n" if !$report;
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--registry PATH] [--report] [--self-test]\n";
}

sub absolute {
    my ($relative) = @_;
    return File::Spec->catfile($root, split m{/}, $relative);
}

sub problem {
    my ($message) = @_;
    push @errors, $message;
}

sub read_registry {
    my ($path) = @_;
    if (!-f $path) {
        problem("registry is missing: $registry_rel");
        return ({}, []);
    }
    my $file_bytes = -s $path;
    problem('registry exceeds portable hard byte cap 65536') if $file_bytes > 65_536;
    open my $fh, '<:raw', $path or do {
        problem("cannot read registry: $!");
        return ({}, []);
    };
    my @records;
    my @lengths;
    my $line_number = 0;
    while (my $line = <$fh>) {
        $line_number++;
        chomp $line;
        $line =~ s/\r\z//;
        next if $line eq '';
        problem("registry line $line_number exceeds portable raw-record cap 8192")
            if length($line) > 8_192;
        my $record = eval { decode_json($line) };
        if (!$record || ref($record) ne 'HASH') {
            problem("registry line $line_number is not one JSON object: $@");
            next;
        }
        push @records, $record;
        push @lengths, length($line);
    }
    close $fh;
    if (!@records) {
        problem('registry is empty');
        return ({}, []);
    }
    my $control = shift @records;
    reject_unknown($control, 'registry control', qw(
      record_type schema_version max_records max_bytes max_record_bytes max_array_items max_scalar_bytes
    ));
    problem('first record must have record_type=registry')
        if ($control->{record_type} // '') ne 'registry';
    for my $field (qw(schema_version max_records max_bytes max_record_bytes max_array_items max_scalar_bytes)) {
        problem("registry control lacks numeric '$field'")
            if !defined($control->{$field}) || ref($control->{$field}) || $control->{$field} !~ /\A\d+\z/;
    }
    problem('registry schema_version must be 1')
        if defined($control->{schema_version}) && $control->{schema_version} != 1;
    my %hard = (
        max_records => 32,
        max_bytes => 65_536,
        max_record_bytes => 8_192,
        max_array_items => 32,
        max_scalar_bytes => 1_024,
    );
    for my $field (keys %hard) {
        next if !defined $control->{$field} || ref($control->{$field});
        problem("registry '$field' must be positive") if $control->{$field} == 0;
        problem("registry '$field' exceeds portable hard cap $hard{$field}")
            if $control->{$field} > $hard{$field};
    }
    problem('registry has more records than max_records')
        if defined($control->{max_records}) && @records > $control->{max_records};
    problem('registry exceeds max_bytes')
        if defined($control->{max_bytes}) && $file_bytes > $control->{max_bytes};
    if (defined $control->{max_record_bytes}) {
        problem('registry contains a record above max_record_bytes')
            if grep { $_ > $control->{max_record_bytes} } @lengths;
    }
    if (defined($control->{max_array_items}) && defined($control->{max_scalar_bytes})) {
        validate_value_bounds($_, 'registry record', $control->{max_array_items}, $control->{max_scalar_bytes})
            for @records;
    }
    return ($control, \@records);
}

sub validate_ledger_schema {
    my ($ledger) = @_;
    reject_unknown($ledger, 'ledger record', qw(
      ledger_id source owner order migration_state grammar planned_live live_limits archive consumers measurement
    ));
    reject_unknown($ledger->{grammar}, 'grammar', qw(
      kind skipped_h2 start_marker end_marker detached_titles trailer_markers
    )) if ref($ledger->{grammar}) eq 'HASH';
    reject_unknown($ledger->{planned_live}, 'planned_live', qw(
      prefix_records promote_titles records lines bytes line_bytes first_archived_sha256
    )) if ref($ledger->{planned_live}) eq 'HASH';
    reject_unknown($ledger->{live_limits}, 'live_limits', qw(records lines bytes line_bytes warning_pct rollover_pct))
        if ref($ledger->{live_limits}) eq 'HASH';
    reject_unknown($ledger->{archive}, 'archive', qw(
      manifest index directory source_capsule retrieval verifier overlap
    )) if ref($ledger->{archive}) eq 'HASH';
    reject_unknown($ledger->{consumers}, 'consumers', qw(readers writers required_literals))
        if ref($ledger->{consumers}) eq 'HASH';
    reject_unknown($ledger->{measurement}, 'measurement', qw(records lines bytes line_bytes sha256))
        if ref($ledger->{measurement}) eq 'HASH';
    if (ref($ledger->{consumers}) eq 'HASH' && ref($ledger->{consumers}{required_literals}) eq 'ARRAY') {
        for my $literal (@{ $ledger->{consumers}{required_literals} }) {
            reject_unknown($literal, 'required literal', qw(path literal)) if ref($literal) eq 'HASH';
        }
    }
}

sub validate_ledger {
    my ($ledger, $id) = @_;
    for my $field (qw(source owner order migration_state)) {
        scalar_field($ledger, $field, "ledger '$id'");
    }
    problem("ledger '$id' order must be newest_first") if ($ledger->{order} // '') ne 'newest_first';
    problem("ledger '$id' has unknown migration_state '$ledger->{migration_state}'")
        if defined($ledger->{migration_state}) && !$valid_state{$ledger->{migration_state}};
    my $source = $ledger->{source} // '';
    problem("ledger '$id' source '$source' is not one safe Markdown path")
        if !safe_relative($source) || $source !~ /\.md\z/;
    problem("ledger '$id' source '$source' is missing")
        if safe_relative($source) && !-f absolute($source);

    my $grammar = $ledger->{grammar};
    if (ref($grammar) ne 'HASH') {
        problem("ledger '$id' grammar must be an object");
        return;
    }
    my $kind = scalar_field($grammar, 'kind', "ledger '$id' grammar");
    problem("ledger '$id' has unknown grammar '$kind'")
        if defined($kind) && !$valid_grammar{$kind};
    validate_grammar_contract($grammar, $id, $kind // '');

    my $state = $ledger->{migration_state} // '';
    my $capsule = ref($ledger->{archive}) eq 'HASH' ? ($ledger->{archive}{source_capsule} // '') : '';
    my $measurement_path = $state eq 'migrated' ? $capsule : $source;
    if ($state eq 'migrated' && (!safe_relative($capsule) || !-f absolute($capsule))) {
        problem("ledger '$id' migrated source capsule '$capsule' is missing or unsafe");
        return;
    }
    return if !safe_relative($measurement_path) || !-f absolute($measurement_path);
    my $frozen_bytes = slurp(absolute($measurement_path), "ledger '$id' measurement source");
    return if !defined $frozen_bytes;
    my $parsed = parse_ledger($frozen_bytes, $grammar, $id);
    return if !defined $parsed;
    problem("ledger '$id' parser does not reconstruct its source byte-for-byte")
        if reconstruct($parsed) ne $frozen_bytes;
    my $metrics = metrics($frozen_bytes);
    $metrics->{records} = scalar @{ $parsed->{records} };
    validate_exact_metrics($ledger->{measurement}, $metrics, sha256_hex($frozen_bytes), "ledger '$id' measurement");

    my ($selected, $selection_errors) = select_live_records($parsed->{records}, $ledger->{planned_live}, $id);
    problem($_) for @$selection_errors;
    my $planned_bytes = $parsed->{prologue} . join('', map { $_->{bytes} } @$selected) . $parsed->{trailer};
    my $planned_metrics = metrics($planned_bytes);
    $planned_metrics->{records} = scalar @$selected;
    validate_planned($ledger->{planned_live}, $planned_metrics, $parsed->{records}, $selected, $id);
    validate_limits($ledger->{live_limits}, $planned_metrics, $id, 'planned survivor');
    validate_archive_contract($ledger->{archive}, $id, $state, $source, $metrics, $parsed, $grammar);
    validate_consumers($ledger->{consumers}, $id);

    if ($state eq 'migrated') {
        my $live_bytes = slurp(absolute($source), "ledger '$id' live source");
        if (defined $live_bytes) {
            my $live = parse_ledger($live_bytes, $grammar, "$id live");
            if (defined $live) {
                problem("ledger '$id' live parser does not reconstruct byte-for-byte")
                    if reconstruct($live) ne $live_bytes;
                my $live_metrics = metrics($live_bytes);
                $live_metrics->{records} = scalar @{ $live->{records} };
                validate_limits($ledger->{live_limits}, $live_metrics, $id, 'live source');
                validate_retained_suffix($live->{records}, $selected, $id);
            }
        }
    }

    if ($report) {
        my %selected_ordinals = map { $_->{ordinal} => 1 } @$selected;
        my ($first_archived) = grep { !$selected_ordinals{$_->{ordinal}} } @{ $parsed->{records} };
        print $json->encode({
            ledger_id => $id,
            migration_state => $state,
            source => { %$metrics, sha256 => sha256_hex($frozen_bytes) },
            planned_live => $planned_metrics,
            archived_records => $metrics->{records} - $planned_metrics->{records},
            first_archived_sha256 => sha256_hex($first_archived->{bytes}),
        }), "\n";
    }
}

sub validate_grammar_contract {
    my ($grammar, $id, $kind) = @_;
    for my $array_field (qw(detached_titles trailer_markers)) {
        problem("ledger '$id' grammar '$array_field' must be an array")
            if ref($grammar->{$array_field}) ne 'ARRAY';
    }
    my $skipped = $grammar->{skipped_h2};
    problem("ledger '$id' grammar skipped_h2 must be a nonnegative integer")
        if !defined($skipped) || ref($skipped) || $skipped !~ /\A\d+\z/;
    if ($kind eq 'current_snapshot_bullets_v1') {
        scalar_field($grammar, 'start_marker', "ledger '$id' grammar");
        scalar_field($grammar, 'end_marker', "ledger '$id' grammar");
    } else {
        problem("ledger '$id' non-snapshot grammar must not declare range markers")
            if defined($grammar->{start_marker}) || defined($grammar->{end_marker});
    }
    if ($kind ne 'changes_mixed_v1' && ref($grammar->{detached_titles}) eq 'ARRAY'
        && @{ $grammar->{detached_titles} }) {
        problem("ledger '$id' detached titles are valid only for changes_mixed_v1");
    }
}

sub parse_ledger {
    my ($bytes, $grammar, $label) = @_;
    my $kind = $grammar->{kind} // '';
    return parse_changes($bytes, $grammar, $label) if $kind eq 'changes_mixed_v1';
    return parse_h2($bytes, $grammar, $label) if $kind eq 'h2_records_v1';
    return parse_snapshot($bytes, $grammar, $label) if $kind eq 'current_snapshot_bullets_v1';
    problem("ledger '$label' cannot parse unknown grammar '$kind'");
    return;
}

sub parse_changes {
    my ($bytes, $grammar, $label) = @_;
    my %detached = map { $_ => 1 } @{ $grammar->{detached_titles} // [] };
    my @starts;
    my $seen_h2 = 0;
    while ($bytes =~ /\G(.*?(?:\n|\z))/gcs) {
        my $line = $1;
        my $offset = pos($bytes) - length($line);
        if ($line =~ /^## (?!#)([^\r\n]+)/) {
            $seen_h2 = 1;
            push @starts, [$offset, '## ' . decode_utf8($1)];
        } elsif (!$seen_h2 && $line =~ /^### ([^\r\n]+)/) {
            push @starts, [$offset, '### ' . decode_utf8($1)];
        } elsif ($seen_h2 && $line =~ /^(### [^\r\n]+)/) {
            my $title = decode_utf8($1);
            push @starts, [$offset, $title] if $detached{$title};
        }
    }
    if (!@starts || $starts[0][0] != 0) {
        problem("ledger '$label' changes grammar requires a record at byte zero");
        return;
    }
    return build_parts($bytes, '', '', \@starts, length($bytes));
}

sub parse_h2 {
    my ($bytes, $grammar, $label) = @_;
    my @all;
    while ($bytes =~ /^## (?!#)([^\r\n]+)/mg) {
        push @all, [$-[0], '## ' . decode_utf8($1)];
    }
    my $skip = $grammar->{skipped_h2} // 0;
    if (@all <= $skip) {
        problem("ledger '$label' h2 grammar found no record after $skip skipped heading(s)");
        return;
    }
    my @starts = @all[$skip .. $#all];
    my $prologue = substr($bytes, 0, $starts[0][0]);
    return build_parts($bytes, $prologue, '', \@starts, length($bytes));
}

sub parse_snapshot {
    my ($bytes, $grammar, $label) = @_;
    my $start = $grammar->{start_marker} // '';
    my $end = $grammar->{end_marker} // '';
    my $start_at = index($bytes, $start);
    my $end_at = index($bytes, $end);
    if ($start_at < 0 || $end_at < 0 || $end_at <= $start_at + length($start)) {
        problem("ledger '$label' snapshot markers are missing, duplicated, or out of order");
        return;
    }
    problem("ledger '$label' snapshot start marker is not unique")
        if index($bytes, $start, $start_at + 1) >= 0;
    problem("ledger '$label' snapshot end marker is not unique")
        if index($bytes, $end, $end_at + 1) >= 0;
    my $records_at = $start_at + length($start);
    my $region = substr($bytes, $records_at, $end_at - $records_at);
    my @starts;
    while ($region =~ /\G([^\n]*(?:\n|\z))/gc) {
        my $line = $1;
        last if $line eq '';
        my $offset = $records_at + pos($region) - length($line);
        next if $line =~ /^\r?\n\z/;
        if ($line !~ /^- /) {
            problem("ledger '$label' snapshot record region contains a non-bullet line");
            return;
        }
        (my $title = $line) =~ s/[\r\n]+\z//;
        push @starts, [$offset, decode_utf8($title)];
    }
    if (!@starts) {
        problem("ledger '$label' snapshot has no bullet records");
        return;
    }
    my $prologue = substr($bytes, 0, $records_at);
    my $trailer = substr($bytes, $end_at);
    for my $marker (@{ $grammar->{trailer_markers} // [] }) {
        problem("ledger '$label' trailer lacks required marker '$marker'") if index($trailer, $marker) < 0;
    }
    return build_parts($bytes, $prologue, $trailer, \@starts, $end_at);
}

sub build_parts {
    my ($bytes, $prologue, $trailer, $starts, $record_end) = @_;
    my @records;
    for my $index (0 .. $#$starts) {
        my ($start, $title) = @{ $starts->[$index] };
        my $end = $index == $#$starts ? $record_end : $starts->[$index + 1][0];
        push @records, {
            ordinal => $index + 1,
            title => $title,
            bytes => substr($bytes, $start, $end - $start),
        };
    }
    return { prologue => $prologue, records => \@records, trailer => $trailer };
}

sub reconstruct {
    my ($parsed) = @_;
    return $parsed->{prologue} . join('', map { $_->{bytes} } @{ $parsed->{records} }) . $parsed->{trailer};
}

sub select_live_records {
    my ($records, $plan, $id) = @_;
    my @selection_errors;
    if (ref($plan) ne 'HASH') {
        return ([], ["ledger '$id' planned_live must be an object"]);
    }
    my $prefix = $plan->{prefix_records};
    if (!defined($prefix) || ref($prefix) || $prefix !~ /\A\d+\z/ || $prefix < 1 || $prefix >= @$records) {
        return ([], ["ledger '$id' prefix_records must be positive and leave nonempty archive history"]);
    }
    my @selected = @$records[0 .. $prefix - 1];
    my %selected_ordinal = map { $_->{ordinal} => 1 } @selected;
    my $promote = $plan->{promote_titles};
    if (ref($promote) ne 'ARRAY') {
        push @selection_errors, "ledger '$id' promote_titles must be an array";
        return (\@selected, \@selection_errors);
    }
    for my $title (@$promote) {
        my @matches = grep { $_->{title} eq $title } @$records;
        if (@matches != 1) {
            push @selection_errors, "ledger '$id' promoted title '$title' matches " . scalar(@matches) . ' records, expected one';
            next;
        }
        next if $selected_ordinal{$matches[0]{ordinal}}++;
        push @selected, $matches[0];
    }
    return (\@selected, \@selection_errors);
}

sub validate_planned {
    my ($plan, $actual, $records, $selected, $id) = @_;
    return if ref($plan) ne 'HASH';
    for my $field (qw(records lines bytes line_bytes)) {
        problem("ledger '$id' planned_live '$field' must be a nonnegative integer")
            if !defined($plan->{$field}) || ref($plan->{$field}) || $plan->{$field} !~ /\A\d+\z/;
        problem("ledger '$id' planned_live '$field' drift: actual $actual->{$field}, expected $plan->{$field}")
            if defined($plan->{$field}) && !ref($plan->{$field}) && $plan->{$field} =~ /\A\d+\z/
            && $actual->{$field} != $plan->{$field};
    }
    my %selected = map { $_->{ordinal} => 1 } @$selected;
    my ($first_archived) = grep { !$selected{$_->{ordinal}} } @$records;
    my $expected_sha = $plan->{first_archived_sha256};
    problem("ledger '$id' planned_live lacks first_archived_sha256")
        if !defined($expected_sha) || ref($expected_sha) || $expected_sha !~ /\A[0-9a-f]{64}\z/;
    my $actual_sha = $first_archived ? sha256_hex($first_archived->{bytes}) : '';
    problem("ledger '$id' first archived record identity drift: actual '$actual_sha', expected '$expected_sha'")
        if defined($expected_sha) && !ref($expected_sha) && $expected_sha =~ /\A[0-9a-f]{64}\z/
        && $actual_sha ne $expected_sha;
}

sub validate_exact_metrics {
    my ($expected, $actual, $sha, $label) = @_;
    if (ref($expected) ne 'HASH') {
        problem("$label must be an object");
        return;
    }
    for my $field (qw(records lines bytes line_bytes)) {
        problem("$label '$field' must be a nonnegative integer")
            if !defined($expected->{$field}) || ref($expected->{$field}) || $expected->{$field} !~ /\A\d+\z/;
        problem("$label '$field' drift: actual $actual->{$field}, expected $expected->{$field}")
            if defined($expected->{$field}) && !ref($expected->{$field}) && $expected->{$field} =~ /\A\d+\z/
            && $actual->{$field} != $expected->{$field};
    }
    problem("$label sha256 drift: actual $sha, expected " . ($expected->{sha256} // '<missing>'))
        if ($expected->{sha256} // '') ne $sha;
}

sub validate_limits {
    my ($limits, $actual, $id, $label) = @_;
    if (ref($limits) ne 'HASH') {
        problem("ledger '$id' live_limits must be an object");
        return;
    }
    for my $field (qw(records lines bytes line_bytes warning_pct rollover_pct)) {
        problem("ledger '$id' live_limits '$field' must be a positive integer")
            if !defined($limits->{$field}) || ref($limits->{$field}) || $limits->{$field} !~ /\A\d+\z/ || $limits->{$field} < 1;
    }
    problem("ledger '$id' warning_pct must be below rollover_pct")
        if ($limits->{warning_pct} // 100) >= ($limits->{rollover_pct} // 0);
    problem("ledger '$id' rollover_pct must be below 100") if ($limits->{rollover_pct} // 100) >= 100;
    for my $field (qw(records lines bytes line_bytes)) {
        next if !defined($limits->{$field}) || ref($limits->{$field}) || $limits->{$field} !~ /\A\d+\z/;
        problem("ledger '$id' $label exceeds live $field limit: $actual->{$field} > $limits->{$field}")
            if $actual->{$field} > $limits->{$field};
        if ($label eq 'planned survivor') {
            problem("ledger '$id' planned survivor starts at or above the $limits->{warning_pct}% warning for $field")
                if $actual->{$field} * 100 >= $limits->{$field} * $limits->{warning_pct};
        } elsif ($label eq 'live source') {
            problem("ledger '$id' live source reached the $limits->{rollover_pct}% rollover threshold for $field")
                if $actual->{$field} * 100 >= $limits->{$field} * $limits->{rollover_pct};
        }
    }
}

sub validate_archive_contract {
    my ($archive, $id, $state, $source, $metrics, $parsed, $grammar) = @_;
    if (ref($archive) ne 'HASH') {
        problem("ledger '$id' archive must be an object");
        return;
    }
    for my $field (qw(manifest index directory source_capsule verifier)) {
        my $path = scalar_field($archive, $field, "ledger '$id' archive");
        problem("ledger '$id' archive '$field' path '$path' is unsafe")
            if defined($path) && !safe_relative($path);
    }
    my $retrieval = scalar_field($archive, 'retrieval', "ledger '$id' archive");
    problem("ledger '$id' retrieval must invoke scripts/check_rolling_ledger_protocol.pl --report")
        if defined($retrieval) && $retrieval ne 'perl scripts/check_rolling_ledger_protocol.pl --report';
    problem("ledger '$id' archive verifier must be scripts/check_rolling_ledger_protocol.pl")
        if ($archive->{verifier} // '') ne 'scripts/check_rolling_ledger_protocol.pl';
    problem("ledger '$id' archive overlap must declare source_capsule_overlaps_live_window")
        if ($archive->{overlap} // '') ne 'source_capsule_overlaps_live_window';
    my $directory = $archive->{directory} // '';
    for my $field (qw(source_capsule)) {
        my $path = $archive->{$field} // '';
        problem("ledger '$id' archive $field must be below '$directory/'")
            if $directory ne '' && index($path, "$directory/") != 0;
    }
    return if $state ne 'migrated';
    for my $field (qw(manifest index source_capsule verifier)) {
        my $path = $archive->{$field} // '';
        problem("ledger '$id' migrated archive '$path' is missing")
            if safe_relative($path) && !-f absolute($path);
    }
    validate_archive_manifest(
        $archive->{manifest}, $id, $source, $archive->{source_capsule}, $metrics, $parsed, $grammar
    );
    validate_archive_index($archive->{index}, $id, $source, $archive->{source_capsule});
}

sub validate_archive_manifest {
    my ($manifest_rel, $id, $source, $capsule, $metrics, $parsed, $grammar) = @_;
    return if !safe_relative($manifest_rel) || !-f absolute($manifest_rel);
    my $manifest_path = absolute($manifest_rel);
    my $file_bytes = -s $manifest_path;
    problem("ledger '$id' archive manifest exceeds portable hard byte cap 65536")
        if $file_bytes > 65_536;
    open my $fh, '<:raw', $manifest_path or do {
        problem("ledger '$id' cannot read archive manifest '$manifest_rel'");
        return;
    };
    my @objects;
    my @raw_lengths;
    my $line_number = 0;
    while (my $line = <$fh>) {
        $line_number++;
        chomp $line;
        $line =~ s/\r\z//;
        next if $line eq '';
        problem("ledger '$id' archive manifest line $line_number exceeds portable record cap 4096")
            if length($line) > 4_096;
        my $object = eval { decode_json($line) };
        if (!$object || ref($object) ne 'HASH') {
            problem("ledger '$id' archive manifest line $line_number is not one JSON object: $@");
            next;
        }
        push @objects, $object;
        push @raw_lengths, length($line);
    }
    close $fh;
    if (!@objects) {
        problem("ledger '$id' archive manifest is empty");
        return;
    }
    my $control = shift @objects;
    reject_unknown($control, 'archive manifest control', qw(
      record_type schema_version max_records max_bytes max_record_bytes max_scalar_bytes
    ));
    problem("ledger '$id' archive manifest control has unknown record_type")
        if ($control->{record_type} // '') ne 'archive_manifest';
    my %hard = (
        schema_version => 1,
        max_records => 64,
        max_bytes => 65_536,
        max_record_bytes => 4_096,
        max_scalar_bytes => 1_024,
    );
    for my $field (keys %hard) {
        problem("ledger '$id' archive manifest control lacks numeric '$field'")
            if !defined($control->{$field}) || ref($control->{$field}) || $control->{$field} !~ /\A\d+\z/;
    }
    problem("ledger '$id' archive manifest schema_version must be 1")
        if defined($control->{schema_version}) && $control->{schema_version} != 1;
    for my $field (qw(max_records max_bytes max_record_bytes max_scalar_bytes)) {
        next if !defined($control->{$field}) || ref($control->{$field});
        problem("ledger '$id' archive manifest '$field' must be positive") if $control->{$field} == 0;
        problem("ledger '$id' archive manifest '$field' exceeds portable hard cap $hard{$field}")
            if $control->{$field} > $hard{$field};
    }
    problem("ledger '$id' archive manifest exceeds its max_records")
        if defined($control->{max_records}) && @objects > $control->{max_records};
    problem("ledger '$id' archive manifest exceeds its max_bytes")
        if defined($control->{max_bytes}) && $file_bytes > $control->{max_bytes};
    problem("ledger '$id' archive manifest contains a record above max_record_bytes")
        if defined($control->{max_record_bytes})
        && grep { $_ > $control->{max_record_bytes} } @raw_lengths;
    validate_value_bounds($_, 'archive manifest record', 1, $control->{max_scalar_bytes})
        for grep { defined $control->{max_scalar_bytes} } @objects;

    my %path_seen;
    my %segment_seen;
    for my $entry (@objects) {
        my $type = $entry->{record_type} // '';
        my @common = qw(
          record_type ledger_id path sealed_on reason sha256 records lines bytes line_bytes
          first_record_sha256 last_record_sha256 verifier
        );
        if ($type eq 'source_capsule') {
            reject_unknown($entry, 'source capsule manifest record', @common, qw(source));
        } elsif ($type eq 'sealed_segment') {
            reject_unknown($entry, 'sealed segment manifest record', @common,
                qw(segment_id predecessor successor));
            my $segment_id = scalar_field($entry, 'segment_id', 'sealed segment manifest record');
            problem("archive manifest duplicate segment_id '$segment_id'")
                if defined($segment_id) && $segment_seen{$segment_id}++;
        } else {
            problem("ledger '$id' archive manifest has unknown record_type '$type'");
            next;
        }
        for my $field (qw(ledger_id path sealed_on reason sha256 first_record_sha256 last_record_sha256 verifier)) {
            scalar_field($entry, $field, "$type manifest record");
        }
        for my $field (qw(records lines bytes line_bytes)) {
            problem("$type manifest record '$field' must be a nonnegative integer")
                if !defined($entry->{$field}) || ref($entry->{$field}) || $entry->{$field} !~ /\A\d+\z/;
        }
        my $path = $entry->{path} // '';
        problem("archive manifest path '$path' is unsafe") if !safe_relative($path);
        problem("archive manifest path '$path' is duplicated") if $path_seen{$path}++;
        problem("archive manifest path '$path' is missing") if safe_relative($path) && !-f absolute($path);
        problem("$type manifest record lacks ISO sealing date")
            if ($entry->{sealed_on} // '') !~ /\A\d{4}-\d{2}-\d{2}\z/;
        problem("$type manifest record verifier mismatch")
            if ($entry->{verifier} // '') ne 'scripts/check_rolling_ledger_protocol.pl';
        for my $field (qw(sha256 first_record_sha256 last_record_sha256)) {
            problem("$type manifest record '$field' is not SHA-256")
                if ($entry->{$field} // '') !~ /\A[0-9a-f]{64}\z/;
        }
        next if !safe_relative($path) || !-f absolute($path);
        my $entry_bytes = slurp(absolute($path), "$type '$path'");
        next if !defined $entry_bytes;
        my $entry_metrics = metrics($entry_bytes);
        problem("$type manifest record sha256 mismatch for '$path'")
            if ($entry->{sha256} // '') ne sha256_hex($entry_bytes);
        for my $field (qw(lines bytes line_bytes)) {
            problem("$type manifest record '$field' mismatch for '$path'")
                if ($entry->{$field} // -1) != $entry_metrics->{$field};
        }
    }

    my @matches = grep {
        ($_->{record_type} // '') eq 'source_capsule' && ($_->{ledger_id} // '') eq $id
    } @objects;
    if (@matches != 1) {
        problem("ledger '$id' archive manifest must contain exactly one source_capsule record");
        return;
    }
    my $entry = $matches[0];
    for my $pair ([source => $source], [path => $capsule], [verifier => 'scripts/check_rolling_ledger_protocol.pl']) {
        problem("ledger '$id' archive manifest '$pair->[0]' mismatch")
            if ($entry->{$pair->[0]} // '') ne $pair->[1];
    }
    for my $field (qw(records lines bytes line_bytes)) {
        problem("ledger '$id' archive manifest '$field' mismatch")
            if ($entry->{$field} // -1) != $metrics->{$field};
    }
    my $capsule_bytes = slurp(absolute($capsule), "ledger '$id' source capsule");
    problem("ledger '$id' archive manifest sha256 mismatch")
        if defined($capsule_bytes) && ($entry->{sha256} // '') ne sha256_hex($capsule_bytes);
    problem("ledger '$id' archive manifest first record identity mismatch")
        if ($entry->{first_record_sha256} // '') ne sha256_hex($parsed->{records}[0]{bytes});
    problem("ledger '$id' archive manifest last record identity mismatch")
        if ($entry->{last_record_sha256} // '') ne sha256_hex($parsed->{records}[-1]{bytes});

    for my $segment (grep {
        ($_->{record_type} // '') eq 'sealed_segment' && ($_->{ledger_id} // '') eq $id
    } @objects) {
        my $path = $segment->{path} // '';
        next if !safe_relative($path) || !-f absolute($path);
        my $segment_bytes = slurp(absolute($path), "ledger '$id' sealed segment");
        next if !defined $segment_bytes;
        my $segment_parsed = parse_segment($segment_bytes, $grammar, "$id segment");
        next if !defined $segment_parsed;
        problem("ledger '$id' sealed segment does not reconstruct byte-for-byte")
            if join('', map { $_->{bytes} } @$segment_parsed) ne $segment_bytes;
        problem("ledger '$id' sealed segment record count mismatch")
            if ($segment->{records} // -1) != @$segment_parsed;
        problem("ledger '$id' sealed segment first record identity mismatch")
            if ($segment->{first_record_sha256} // '') ne sha256_hex($segment_parsed->[0]{bytes});
        problem("ledger '$id' sealed segment last record identity mismatch")
            if ($segment->{last_record_sha256} // '') ne sha256_hex($segment_parsed->[-1]{bytes});
    }
}

sub parse_segment {
    my ($bytes, $grammar, $label) = @_;
    my $kind = $grammar->{kind} // '';
    if ($kind eq 'changes_mixed_v1') {
        my $parsed = parse_changes($bytes, $grammar, $label);
        return $parsed ? $parsed->{records} : undef;
    }
    if ($kind eq 'h2_records_v1') {
        my %segment_grammar = %$grammar;
        $segment_grammar{skipped_h2} = 0;
        my $parsed = parse_h2($bytes, \%segment_grammar, $label);
        if ($parsed && $parsed->{prologue} ne '') {
            problem("ledger '$label' sealed segment has non-record prologue bytes");
            return;
        }
        return $parsed ? $parsed->{records} : undef;
    }
    if ($kind eq 'current_snapshot_bullets_v1') {
        my @starts;
        while ($bytes =~ /\G([^\n]*(?:\n|\z))/gc) {
            my $line = $1;
            last if $line eq '';
            next if $line =~ /^\r?\n\z/ && pos($bytes) == length($bytes);
            if ($line !~ /^- /) {
                problem("ledger '$label' sealed status segment contains a non-bullet line");
                return;
            }
            my $offset = pos($bytes) - length($line);
            (my $title = $line) =~ s/[\r\n]+\z//;
            push @starts, [$offset, decode_utf8($title)];
        }
        if (!@starts) {
            problem("ledger '$label' sealed status segment has no records");
            return;
        }
        return build_parts($bytes, '', '', \@starts, length($bytes))->{records};
    }
    problem("ledger '$label' sealed segment has unknown grammar '$kind'");
    return;
}

sub validate_archive_index {
    my ($index_rel, $id, $source, $capsule) = @_;
    return if !safe_relative($index_rel) || !-f absolute($index_rel);
    my $bytes = slurp(absolute($index_rel), "ledger '$id' archive index");
    return if !defined $bytes;
    for my $needle ($source, $capsule, $id) {
        problem("ledger '$id' archive index lacks '$needle'") if index($bytes, $needle) < 0;
    }
}

sub validate_consumers {
    my ($consumers, $id) = @_;
    if (ref($consumers) ne 'HASH') {
        problem("ledger '$id' consumers must be an object");
        return;
    }
    for my $kind (qw(readers writers)) {
        my $paths = $consumers->{$kind};
        if (ref($paths) ne 'ARRAY' || !@$paths) {
            problem("ledger '$id' consumers '$kind' must be a non-empty array");
            next;
        }
        for my $path (@$paths) {
            problem("ledger '$id' $kind path '$path' is missing or unsafe")
                if !safe_relative($path) || !-e absolute($path);
        }
    }
    my $literals = $consumers->{required_literals};
    if (ref($literals) ne 'ARRAY') {
        problem("ledger '$id' required_literals must be an array");
        return;
    }
    for my $entry (@$literals) {
        if (ref($entry) ne 'HASH') {
            problem("ledger '$id' required literal must be an object");
            next;
        }
        my $path = $entry->{path} // '';
        my $literal = $entry->{literal} // '';
        if (!safe_relative($path) || !-f absolute($path)) {
            problem("ledger '$id' required-literal path '$path' is missing or unsafe");
            next;
        }
        my $bytes = slurp(absolute($path), "ledger '$id' required-literal path");
        problem("ledger '$id' required literal '$literal' is absent from '$path'")
            if defined($bytes) && index($bytes, $literal) < 0;
    }
}

sub validate_retained_suffix {
    my ($live, $selected, $id) = @_;
    if (@$live < @$selected) {
        problem("ledger '$id' live source has fewer records than its retained migration window");
        return;
    }
    my $offset = @$live - @$selected;
    for my $index (0 .. $#$selected) {
        problem("ledger '$id' retained record identity/order changed at survivor index " . ($index + 1))
            if sha256_hex($live->[$offset + $index]{bytes}) ne sha256_hex($selected->[$index]{bytes});
    }
}

sub metrics {
    my ($bytes) = @_;
    my $lines = () = $bytes =~ /\n/g;
    $lines++ if length($bytes) && $bytes !~ /\n\z/;
    my $max = 0;
    for my $line (split /\n/, $bytes, -1) {
        $line =~ s/\r\z//;
        $max = length($line) if length($line) > $max;
    }
    return { lines => $lines, bytes => length($bytes), line_bytes => $max };
}

sub slurp {
    my ($path, $label) = @_;
    open my $fh, '<:raw', $path or do {
        problem("cannot read $label: $!");
        return;
    };
    local $/;
    my $bytes = <$fh> // '';
    close $fh;
    return $bytes;
}

sub scalar_field {
    my ($object, $field, $label) = @_;
    if (!defined($object->{$field}) || ref($object->{$field}) || $object->{$field} eq '') {
        problem("$label lacks scalar '$field'");
        return;
    }
    return $object->{$field};
}

sub safe_relative {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '' || $path =~ m{\A/};
    return 0 if $path =~ m{(?:\A|/)\.\.(?:/|\z)} || $path =~ m{(?:\A|/)\.(?:/|\z)};
    return 0 if $path =~ /[\x00\r\n]/;
    return 1;
}

sub reject_unknown {
    my ($object, $label, @allowed) = @_;
    if (ref($object) ne 'HASH') {
        problem("$label must be an object");
        return;
    }
    my %allowed = map { $_ => 1 } @allowed;
    problem("$label has unknown field '$_'") for grep { !$allowed{$_} } sort keys %$object;
}

sub validate_value_bounds {
    my ($value, $label, $max_array, $max_scalar) = @_;
    if (ref($value) eq 'HASH') {
        validate_value_bounds($value->{$_}, "$label.$_", $max_array, $max_scalar) for keys %$value;
    } elsif (ref($value) eq 'ARRAY') {
        problem("$label exceeds max_array_items") if @$value > $max_array;
        validate_value_bounds($_, "$label\[\]", $max_array, $max_scalar) for @$value;
    } elsif (defined($value) && length("$value") > $max_scalar) {
        problem("$label exceeds max_scalar_bytes");
    }
}

sub run_self_test {
    my @failures;
    my $checks = 0;
    my @saved_errors = @errors;
    @errors = ();
    my @cases = (
        [
            'changes',
            "### new\nA\n## old\nB\n### detached\nC\n",
            { kind => 'changes_mixed_v1', skipped_h2 => 0,
              detached_titles => ['### detached'], trailer_markers => [] },
            3,
        ],
        [
            'development',
            "# notes\n## recent\nA\n## old\nB\n",
            { kind => 'h2_records_v1', skipped_h2 => 0,
              detached_titles => [], trailer_markers => [] },
            2,
        ],
        [
            'analysis',
            "# analysis\n## Purpose\nP\n## recent\nA\n## old\nB\n",
            { kind => 'h2_records_v1', skipped_h2 => 1,
              detached_titles => [], trailer_markers => [] },
            2,
        ],
        [
            'snapshot',
            "# status\n## Current snapshot\n- new\n- old\n## Gap\n<!-- start -->\n<!-- end -->\n",
            { kind => 'current_snapshot_bullets_v1', skipped_h2 => 0,
              start_marker => "## Current snapshot\n", end_marker => "## Gap\n",
              detached_titles => [], trailer_markers => ['<!-- start -->', '<!-- end -->'] },
            2,
        ],
    );
    for my $case (@cases) {
        $checks++;
        my ($name, $bytes, $grammar, $count) = @$case;
        my $parsed = parse_ledger($bytes, $grammar, "self-test $name");
        push @failures, "$name parser returned no result" if !defined $parsed;
        next if !defined $parsed;
        push @failures, "$name record count drift" if @{ $parsed->{records} } != $count;
        push @failures, "$name reconstruction drift" if reconstruct($parsed) ne $bytes;
    }
    my $bad = parse_ledger(
        "# status\n## Current snapshot\nnot a bullet\n## Gap\n",
        { kind => 'current_snapshot_bullets_v1', skipped_h2 => 0,
          start_marker => "## Current snapshot\n", end_marker => "## Gap\n",
          detached_titles => [], trailer_markers => [] },
        'self-test malformed snapshot',
    );
    push @failures, 'malformed snapshot was accepted' if defined $bad;
    push @failures, 'malformed snapshot produced no diagnostic' if !@errors;
    $checks++;

    @errors = ();
    my @sample_records = (
        { ordinal => 1, title => 'first', bytes => "## first\n" },
        { ordinal => 2, title => 'second', bytes => "## second\n" },
    );
    my (undef, $selection_errors) = select_live_records(
        \@sample_records, { prefix_records => 2, promote_titles => [] }, 'self-test full prefix'
    );
    push @failures, 'a live selection leaving no archive history was accepted' if !@$selection_errors;
    $checks++;

    @errors = ();
    validate_limits(
        { records => 10, lines => 10, bytes => 100, line_bytes => 10,
          warning_pct => 80, rollover_pct => 90 },
        { records => 8, lines => 1, bytes => 1, line_bytes => 1 },
        'self-test warning', 'planned survivor',
    );
    push @failures, 'a planned survivor at its warning threshold was accepted' if !@errors;
    $checks++;

    @errors = ();
    validate_retained_suffix(
        [{ ordinal => 1, title => 'changed', bytes => "## changed\n" }],
        [{ ordinal => 1, title => 'first', bytes => "## first\n" }],
        'self-test suffix',
    );
    push @failures, 'a changed retained record suffix was accepted' if !@errors;
    $checks++;

    @errors = ();
    reject_unknown({ known => 1, surprise => 1 }, 'self-test schema', qw(known));
    push @failures, 'an unknown schema field was accepted' if !@errors;
    $checks++;

    push @failures, 'an escaping path was accepted' if safe_relative('../archive.md');
    $checks++;
    @errors = @saved_errors;
    die "rolling-ledger self-test: $_\n" for @failures;
    print "rolling-ledger: $checks parser/control self-tests pass.\n";
}
