#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(decode_utf8);
use File::Basename qw(dirname);
use File::Path qw(make_path remove_tree);
use File::Spec;
use Fcntl qw(O_CREAT O_EXCL O_WRONLY);
use JSON::PP;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root;
my $registry_rel = 'doctrine/live_document_size/rolling_ledgers.jsonl';
my $surfaces_rel = 'doctrine/live_document_size/surfaces.jsonl';
my $report = 0;
my $self_test = 0;
my $emit_id;
my $output_rel;
my $rollover_plan_rel;
my $apply_rollover = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--surfaces') {
        $surfaces_rel = shift @ARGV // usage();
    } elsif ($arg eq '--report') {
        $report = 1;
    } elsif ($arg eq '--self-test') {
        $self_test = 1;
    } elsif ($arg eq '--emit-planned') {
        $emit_id = shift @ARGV // usage();
    } elsif ($arg eq '--output') {
        $output_rel = shift @ARGV // usage();
    } elsif ($arg eq '--rollover-plan') {
        $rollover_plan_rel = shift @ARGV // usage();
    } elsif ($arg eq '--apply-rollover') {
        $apply_rollover = 1;
    } else {
        usage();
    }
}

$root //= File::Spec->catdir(dirname(abs_path($0)), '..');
$root = abs_path($root) // die "rolling-ledger: repository root does not exist\n";
usage() if $apply_rollover && !defined($rollover_plan_rel);
usage() if defined($rollover_plan_rel) && ($report || $self_test || defined($emit_id) || defined($output_rel));

my @errors;
my %surface_by_source;
my %planned_rollover_ids;
my $post_apply_validation = 0;
my $json = JSON::PP->new->canonical(1);
run_self_test() if $self_test;
exit 0 if $self_test;

my %valid_grammar = map { $_ => 1 } qw(changes_mixed_v1 h2_records_v1 current_snapshot_bullets_v1);
my %valid_state = map { $_ => 1 } qw(planned migrated);
my %emissions;

my ($meta, $ledgers) = read_registry(absolute($registry_rel));
my $rollover_plan = defined($rollover_plan_rel) ? read_rollover_plan($rollover_plan_rel) : undef;
%planned_rollover_ids = map { ($_->{ledger_id} // '') => 1 } @{ $rollover_plan->{rows} // [] }
    if defined $rollover_plan;
read_surface_authority(absolute($surfaces_rel));
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
validate_archive_landings($meta, $ledgers);

emit_planned_view() if defined $emit_id && !@errors;
run_rollover_transaction($rollover_plan, $ledgers, $meta) if defined($rollover_plan) && !@errors;

if (@errors) {
    print STDERR "rolling-ledger: $_\n" for @errors;
    print STDERR "rolling-ledger: FAILED with ", scalar(@errors), " violation(s).\n";
    exit 1;
}

print "rolling-ledger: ", scalar(@$ledgers),
    " ledgers satisfy the lossless live-window/archive protocol.\n" if !$report;
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--registry PATH] [--surfaces PATH] [--report] [--self-test] "
        . "[--emit-planned LEDGER_ID --output SOURCE_PATH] "
        . "[--rollover-plan PATH [--apply-rollover]]\n";
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
      retired_paths
    ));
    problem('first record must have record_type=registry')
        if ($control->{record_type} // '') ne 'registry';
    for my $field (qw(schema_version max_records max_bytes max_record_bytes max_array_items max_scalar_bytes)) {
        problem("registry control lacks numeric '$field'")
            if !defined($control->{$field}) || ref($control->{$field}) || $control->{$field} !~ /\A\d+\z/;
    }
    problem('registry schema_version must be 2')
        if defined($control->{schema_version}) && $control->{schema_version} != 2;
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
    validate_retired_paths($control);
    return ($control, \@records);
}

sub read_surface_authority {
    my ($path) = @_;
    if (!-f $path) {
        problem("surface registry is missing: $surfaces_rel");
        return;
    }
    open my $fh, '<:raw', $path or do {
        problem("cannot read surface registry: $!");
        return;
    };
    my $line_number = 0;
    while (my $line = <$fh>) {
        $line_number++;
        chomp $line;
        $line =~ s/\r\z//;
        next if $line eq '';
        my $record = eval { decode_json($line) };
        if (!$record || ref($record) ne 'HASH') {
            problem("surface registry line $line_number is not one JSON object: $@");
            next;
        }
        next if ($record->{record_type} // '') eq 'registry';
        next if ($record->{lifecycle} // '') ne 'rolling_ledger';
        my $targets = $record->{targets};
        if (ref($targets) ne 'ARRAY' || @$targets != 1 || ref($targets->[0])) {
            problem("rolling-ledger surface at line $line_number must declare one scalar target");
            next;
        }
        my $source = $targets->[0];
        problem("rolling-ledger surface target '$source' is unsafe") if !safe_relative($source);
        problem("duplicate rolling-ledger surface authority for '$source'") if exists $surface_by_source{$source};
        $surface_by_source{$source} = $record;
    }
    close $fh;
}

sub read_rollover_plan {
    my ($relative) = @_;
    if (!safe_relative($relative) || $relative !~ /\.jsonl\z/) {
        problem("rollover plan path '$relative' is unsafe or not JSONL");
        return { control => {}, rows => [] };
    }
    my $path = absolute($relative);
    if (!-f $path) {
        problem("rollover plan is missing: $relative");
        return { control => {}, rows => [] };
    }
    open my $fh, '<:raw', $path or do {
        problem("cannot read rollover plan '$relative': $!");
        return { control => {}, rows => [] };
    };
    my (@objects, @raw_lengths);
    my $line_number = 0;
    while (my $line = <$fh>) {
        $line_number++;
        chomp $line;
        $line =~ s/\r\z//;
        next if $line eq '';
        my $object = eval { decode_json($line) };
        if (!$object || ref($object) ne 'HASH') {
            problem("rollover plan line $line_number is not one JSON object: $@");
            next;
        }
        push @objects, $object;
        push @raw_lengths, length($line);
    }
    close $fh;
    if (!@objects) {
        problem('rollover plan is empty');
        return { control => {}, rows => [] };
    }
    my $control = shift @objects;
    reject_unknown($control, 'rollover plan control', qw(
      record_type schema_version plan_id owner boundary_commit sealed_on order terminal_separator
      max_records max_bytes max_record_bytes max_scalar_bytes
    ));
    problem('rollover plan control record_type must be rolling_ledger_rollover_plan')
        if ($control->{record_type} // '') ne 'rolling_ledger_rollover_plan';
    problem('rollover plan schema_version must be 1') if ($control->{schema_version} // 0) != 1;
    scalar_field($control, $_, 'rollover plan control')
        for qw(plan_id owner boundary_commit sealed_on order terminal_separator);
    problem('rollover plan boundary_commit must be a full Git object id')
        if ($control->{boundary_commit} // '') !~ /\A[0-9a-f]{40}\z/;
    problem('rollover plan sealed_on must be an ISO date')
        if ($control->{sealed_on} // '') !~ /\A\d{4}-\d{2}-\d{2}\z/;
    problem('rollover plan order must be registry_order')
        if ($control->{order} // '') ne 'registry_order';
    problem('rollover plan terminal_separator must be canonical_single_newline')
        if ($control->{terminal_separator} // '') ne 'canonical_single_newline';
    for my $field (qw(max_records max_bytes max_record_bytes max_scalar_bytes)) {
        problem("rollover plan control lacks positive numeric '$field'")
            if !defined($control->{$field}) || ref($control->{$field})
            || $control->{$field} !~ /\A\d+\z/ || $control->{$field} < 1;
    }
    my %hard = (max_records => 16, max_bytes => 32_768, max_record_bytes => 4_096, max_scalar_bytes => 1_024);
    for my $field (keys %hard) {
        next if !defined($control->{$field}) || ref($control->{$field});
        problem("rollover plan '$field' exceeds portable hard cap $hard{$field}")
            if $control->{$field} > $hard{$field};
    }
    problem('rollover plan exceeds max_records')
        if defined($control->{max_records}) && @objects > $control->{max_records};
    problem('rollover plan exceeds max_bytes')
        if defined($control->{max_bytes}) && -s($path) > $control->{max_bytes};
    problem('rollover plan contains a record above max_record_bytes')
        if defined($control->{max_record_bytes}) && grep { $_ > $control->{max_record_bytes} } @raw_lengths;

    my %seen;
    for my $row (@objects) {
        reject_unknown($row, 'rollover plan row', qw(
          ledger_id source opening_sha256 opening_records post_migration_records
          retained_migration_suffix_records keep_opening_prefix_records segment_id segment_path successor
          records lines bytes line_bytes sha256 first_record_sha256 last_record_sha256
          opening_root_after_cut reason
        ));
        reject_unknown($row->{opening_root_after_cut}, 'rollover plan opening_root_after_cut',
            qw(records lines bytes line_bytes sha256));
        for my $field (qw(ledger_id source segment_id segment_path successor reason)) {
            scalar_field($row, $field, 'rollover plan row');
        }
        for my $field (qw(opening_sha256 sha256 first_record_sha256 last_record_sha256)) {
            problem("rollover plan row '$field' is not SHA-256")
                if ($row->{$field} // '') !~ /\A[0-9a-f]{64}\z/;
        }
        for my $field (qw(opening_records post_migration_records retained_migration_suffix_records
          keep_opening_prefix_records records lines bytes line_bytes)) {
            problem("rollover plan row '$field' must be a nonnegative integer")
                if !defined($row->{$field}) || ref($row->{$field}) || $row->{$field} !~ /\A\d+\z/;
        }
        my $opening = $row->{opening_root_after_cut};
        if (ref($opening) ne 'HASH') {
            problem('rollover plan row opening_root_after_cut must be an object');
        } else {
            for my $field (qw(records lines bytes line_bytes)) {
                problem("rollover plan opening_root_after_cut '$field' must be a nonnegative integer")
                    if !defined($opening->{$field}) || ref($opening->{$field}) || $opening->{$field} !~ /\A\d+\z/;
            }
            problem('rollover plan opening_root_after_cut sha256 is not SHA-256')
                if ($opening->{sha256} // '') !~ /\A[0-9a-f]{64}\z/;
        }
        my $id = $row->{ledger_id} // '';
        problem("rollover plan duplicates ledger_id '$id'") if $seen{$id}++;
        for my $field (qw(source segment_path successor)) {
            problem("rollover plan row '$field' path '$row->{$field}' is unsafe")
                if !safe_relative($row->{$field});
        }
        validate_value_bounds($row, 'rollover plan row', 8, $control->{max_scalar_bytes})
            if defined($control->{max_scalar_bytes});
    }
    return { relative => $relative, control => $control, rows => \@objects };
}

sub validate_retired_paths {
    my ($control) = @_;
    my $paths = $control->{retired_paths};
    if (ref($paths) ne 'ARRAY' || !@$paths) {
        problem('registry control retired_paths must be a non-empty array');
        return;
    }
    problem('registry control retired_paths exceeds max_array_items')
        if defined($control->{max_array_items}) && @$paths > $control->{max_array_items};
    validate_value_bounds($paths, 'registry control.retired_paths', $control->{max_array_items}, $control->{max_scalar_bytes})
        if defined($control->{max_array_items}) && defined($control->{max_scalar_bytes});
    my %seen;
    for my $path (@$paths) {
        problem("registry retired path '$path' is unsafe") if !safe_relative($path);
        problem("registry retired path '$path' is duplicated") if defined($path) && !ref($path) && $seen{$path}++;
        problem("registry retired path '$path' still exists")
            if safe_relative($path) && -e absolute($path);
    }
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
      landing manifest index directory source_capsule retrieval verifier overlap
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
    my $surface = validate_surface_binding($ledger, $id, $source);

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
    my $planned_bytes = render_live_view($parsed, $selected, $grammar);
    my $planned_metrics = metrics($planned_bytes);
    $planned_metrics->{records} = scalar @$selected;
    validate_planned($ledger->{planned_live}, $planned_metrics, $parsed->{records}, $selected, $id);
    validate_limits($ledger->{live_limits}, $surface, $planned_metrics, $id, 'planned survivor');
    validate_archive_contract($ledger->{archive}, $id, $state, $source, $metrics, $parsed, $grammar);
    validate_consumers($ledger->{consumers}, $id);
    $emissions{$id} = {
        bytes => $planned_bytes,
        source => $source,
        state => $state,
        capsule => $capsule,
        source_sha256 => $ledger->{measurement}{sha256} // '',
    };

    if ($state eq 'migrated') {
        my $live_bytes = slurp(absolute($source), "ledger '$id' live source");
        if (defined $live_bytes) {
            my $live = parse_ledger($live_bytes, $grammar, "$id live");
            if (defined $live) {
                problem("ledger '$id' live parser does not reconstruct byte-for-byte")
                    if reconstruct($live) ne $live_bytes;
                my $live_metrics = metrics($live_bytes);
                $live_metrics->{records} = scalar @{ $live->{records} };
                validate_limits($ledger->{live_limits}, $surface, $live_metrics, $id, 'live source');
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
            source_first_record_sha256 => sha256_hex($parsed->{records}[0]{bytes}),
            source_last_record_sha256 => sha256_hex($parsed->{records}[-1]{bytes}),
            planned_live => $planned_metrics,
            archived_records => $metrics->{records} - $planned_metrics->{records},
            first_archived_sha256 => sha256_hex($first_archived->{bytes}),
        }), "\n";
    }
}

sub emit_planned_view {
    problem('--emit-planned cannot be combined with --report or --self-test') if $report || $self_test;
    problem('--emit-planned requires --output') if !defined $output_rel;
    problem("emit ledger_id '$emit_id' is unknown") if !$emissions{$emit_id};
    return if @errors;
    my $emission = $emissions{$emit_id};
    problem("ledger '$emit_id' cannot emit after migration") if $emission->{state} ne 'planned';
    problem("ledger '$emit_id' output must be its declared source '$emission->{source}'")
        if $output_rel ne $emission->{source};
    problem("ledger '$emit_id' output '$output_rel' is unsafe") if !safe_relative($output_rel);
    my $capsule = $emission->{capsule};
    if (!safe_relative($capsule) || !-f absolute($capsule)) {
        problem("ledger '$emit_id' exact source capsule must exist before live emission");
    } else {
        my $capsule_bytes = slurp(absolute($capsule), "ledger '$emit_id' source capsule");
        problem("ledger '$emit_id' source capsule does not match the pinned source identity")
            if defined($capsule_bytes) && sha256_hex($capsule_bytes) ne $emission->{source_sha256};
    }
    return if @errors;

    my $output = absolute($output_rel);
    my $temporary = "$output.rolling-ledger-tmp.$$";
    sysopen my $fh, $temporary, O_WRONLY | O_CREAT | O_EXCL, 0644
        or die "rolling-ledger: cannot create same-directory temporary '$temporary': $!\n";
    binmode $fh, ':raw';
    print {$fh} $emission->{bytes}
        or die "rolling-ledger: cannot write same-directory temporary '$temporary': $!\n";
    close $fh or die "rolling-ledger: cannot close same-directory temporary '$temporary': $!\n";
    rename $temporary, $output
        or die "rolling-ledger: cannot atomically replace '$output_rel': $!\n";
    print "rolling-ledger: wrote planned whole-record view for '$emit_id' to '$output_rel'.\n";
}

sub run_rollover_transaction {
    my ($plan, $ledgers, $meta) = @_;
    my %ledger_by_id = map { ($_->{ledger_id} // '') => $_ } @$ledgers;
    my @registry_order = map { $_->{ledger_id} // '' } @$ledgers;
    my @plan_order = map { $_->{ledger_id} // '' } @{ $plan->{rows} };
    my @expected_order = grep { $planned_rollover_ids{$_} } @registry_order;
    problem('rollover plan rows differ from registry order')
        if join("\0", @plan_order) ne join("\0", @expected_order);
    my @prepared;
    for my $row (@{ $plan->{rows} }) {
        my $id = $row->{ledger_id} // '';
        my $ledger = $ledger_by_id{$id};
        if (ref($ledger) ne 'HASH') {
            problem("rollover plan names unknown ledger_id '$id'");
            next;
        }
        problem("rollover plan ledger '$id' is not migrated")
            if ($ledger->{migration_state} // '') ne 'migrated';
        my $entry = prepare_rollover_entry($plan->{control}, $row, $ledger);
        push @prepared, $entry if defined $entry;
    }
    return if @errors;

    if (!$apply_rollover) {
        for my $entry (@prepared) {
            print $json->encode({
                ledger_id => $entry->{id},
                segment => {
                    path => $entry->{segment_path},
                    %{ $entry->{segment_metrics} },
                    sha256 => sha256_hex($entry->{segment_bytes}),
                },
                resulting_live => {
                    %{ $entry->{root_metrics} },
                    sha256 => sha256_hex($entry->{root_bytes}),
                },
                future_prepends => $entry->{future_prepends},
            }), "\n";
        }
        print "rolling-ledger: rollover plan '$plan->{control}{plan_id}' dry-run is exact and warning-safe for "
            . scalar(@prepared) . " ledger(s).\n";
        return;
    }

    my $context = install_rollover_transaction($plan->{control}, \@prepared);
    return if !defined $context;

    $post_apply_validation = 1;
    @errors = ();
    %emissions = ();
    for my $ledger (@$ledgers) {
        my $id = $ledger->{ledger_id} // '';
        validate_ledger($ledger, $id);
    }
    validate_archive_landings($meta, $ledgers);
    if (@errors) {
        my @post_errors = @errors;
        my $rollback_ok = eval { rollback_rollover_transaction($context); 1 };
        @errors = @post_errors;
        problem("rollover transaction rollback failed: " . ($@ || 'unknown failure')) if !$rollback_ok;
        return;
    }
    cleanup_rollover_stage($context->{stage});
    print "rolling-ledger: applied warning-safe root-last rollover plan '$plan->{control}{plan_id}' for "
        . scalar(@prepared) . " ledger(s).\n";
}

sub prepare_rollover_entry {
    my ($control, $row, $ledger) = @_;
    my $id = $row->{ledger_id} // '';
    my $source = $ledger->{source} // '';
    problem("rollover plan ledger '$id' source differs from registry") if ($row->{source} // '') ne $source;
    my $archive = $ledger->{archive};
    return if ref($archive) ne 'HASH';
    my $directory = $archive->{directory} // '';
    my $segment_path = $row->{segment_path} // '';
    problem("rollover plan ledger '$id' segment path is outside '$directory/'")
        if $directory eq '' || index($segment_path, "$directory/") != 0;
    my ($sequence) = ($row->{segment_id} // '') =~ /-([0-9]{4})\z/;
    problem("rollover plan ledger '$id' segment id/path sequence is inconsistent")
        if !defined($sequence)
        || $segment_path !~ m{(?:\A|/)segment-\Q$sequence\E-\d{4}-\d{2}-\d{2}\.md\z};
    problem("rollover plan ledger '$id' segment destination already exists")
        if safe_relative($segment_path) && -e absolute($segment_path);

    my $live_bytes = slurp(absolute($source), "ledger '$id' rollover source");
    return if !defined $live_bytes;
    my $parsed = parse_ledger($live_bytes, $ledger->{grammar}, "$id rollover source");
    return if !defined $parsed;
    my @records = @{ $parsed->{records} };
    my $opening_records = $row->{opening_records} // 0;
    my $future_prepends = @records - $opening_records;
    if ($future_prepends < 0) {
        problem("rollover plan ledger '$id' has fewer records than its opening boundary");
        return;
    }
    my @opening = @records[$future_prepends .. $#records];
    my $opening_bytes = render_live_view($parsed, \@opening, $ledger->{grammar});
    problem("rollover plan ledger '$id' opening source identity drift")
        if sha256_hex($opening_bytes) ne ($row->{opening_sha256} // '');
    if (($control->{boundary_commit} // '') =~ /\A[0-9a-f]{40}\z/) {
        my $committed_opening = git_blob_bytes($control->{boundary_commit}, $source, $id);
        problem("rollover plan ledger '$id' committed opening blob differs from reconstructed boundary")
            if defined($committed_opening) && $committed_opening ne $opening_bytes;
    }
    problem("rollover plan ledger '$id' opening record decomposition drift")
        if ($row->{post_migration_records} // -1) + ($row->{retained_migration_suffix_records} // -1)
        != $opening_records;
    problem("rollover plan ledger '$id' cut does not consume the oldest opening post-migration records")
        if ($row->{keep_opening_prefix_records} // -1) + ($row->{records} // -1)
        != ($row->{post_migration_records} // -2);

    my $keep = $row->{keep_opening_prefix_records} // 0;
    my $cut = $row->{records} // 0;
    if ($keep < 0 || $cut < 1 || $keep + $cut > @opening) {
        problem("rollover plan ledger '$id' cut range is invalid");
        return;
    }
    my @segment_records = @opening[$keep .. $keep + $cut - 1];
    my $segment_bytes = join('', map { $_->{bytes} } @segment_records);
    $segment_bytes =~ s/\r?\n\r?\n\z/\n/;
    my $segment_metrics = metrics($segment_bytes);
    $segment_metrics->{records} = scalar @segment_records;
    validate_rollover_metrics($row, $segment_metrics, sha256_hex($segment_bytes),
        "rollover plan ledger '$id' segment");
    my $segment_parsed = parse_segment($segment_bytes, $ledger->{grammar}, "$id planned segment");
    if (defined($segment_parsed)) {
        problem("rollover plan ledger '$id' segment first-record identity drift")
            if sha256_hex($segment_parsed->[0]{bytes}) ne ($row->{first_record_sha256} // '');
        problem("rollover plan ledger '$id' segment last-record identity drift")
            if sha256_hex($segment_parsed->[-1]{bytes}) ne ($row->{last_record_sha256} // '');
    }

    my @opening_survivors = (
        ($keep ? @opening[0 .. $keep - 1] : ()),
        @opening[$keep + $cut .. $#opening],
    );
    my $opening_root_bytes = render_live_view($parsed, \@opening_survivors, $ledger->{grammar});
    my $opening_root_metrics = metrics($opening_root_bytes);
    $opening_root_metrics->{records} = scalar @opening_survivors;
    validate_rollover_metrics($row->{opening_root_after_cut}, $opening_root_metrics,
        sha256_hex($opening_root_bytes), "rollover plan ledger '$id' opening root after cut");

    my $start = $future_prepends + $keep;
    my @current_survivors = (
        ($start ? @records[0 .. $start - 1] : ()),
        @records[$start + $cut .. $#records],
    );
    my $root_bytes = render_live_view($parsed, \@current_survivors, $ledger->{grammar});
    my $root_metrics = metrics($root_bytes);
    $root_metrics->{records} = scalar @current_survivors;
    my $surface = $surface_by_source{$source};
    validate_warning_safe_survivor($ledger->{live_limits}, $surface, $root_metrics, $id);

    my $manifest = prepare_rollover_manifest($control, $row, $ledger, $segment_metrics);
    return if !defined $manifest;
    my @chain = @{ $manifest->{chain} };
    my $index_bytes = render_archive_index($id, $archive->{index}, $archive->{manifest}, \@chain, $manifest->{segments});
    validate_archive_index_content($index_bytes, $archive->{index}, $id, $archive->{manifest}, \@chain);

    return {
        id => $id,
        source => $source,
        segment_path => $segment_path,
        segment_bytes => $segment_bytes,
        segment_metrics => $segment_metrics,
        root_bytes => $root_bytes,
        root_metrics => $root_metrics,
        manifest_path => $archive->{manifest},
        manifest_bytes => $manifest->{bytes},
        index_path => $archive->{index},
        index_bytes => $index_bytes,
        future_prepends => $future_prepends,
    };
}

sub validate_rollover_metrics {
    my ($expected, $actual, $sha, $label) = @_;
    if (ref($expected) ne 'HASH') {
        problem("$label metrics must be an object");
        return;
    }
    for my $field (qw(records lines bytes line_bytes)) {
        problem("$label $field drift: actual $actual->{$field}, expected " . ($expected->{$field} // '<missing>'))
            if ($expected->{$field} // -1) != $actual->{$field};
    }
    problem("$label SHA-256 drift: actual $sha, expected " . ($expected->{sha256} // '<missing>'))
        if ($expected->{sha256} // '') ne $sha;
}

sub validate_warning_safe_survivor {
    my ($limits, $surface, $actual, $id) = @_;
    return if ref($limits) ne 'HASH' || ref($surface) ne 'HASH';
    my $health = $surface->{health_targets};
    my %health_field = (lines => 'lines_each', bytes => 'bytes_each', line_bytes => 'line_bytes_each');
    for my $field (qw(records lines bytes line_bytes)) {
        my $ceiling = $limits->{$field};
        problem("rollover plan ledger '$id' survivor exceeds $field ceiling")
            if defined($ceiling) && $actual->{$field} > $ceiling;
        my $target = $field eq 'records' ? $limits->{records}
            : ref($health) eq 'HASH' ? $health->{ $health_field{$field} } : undef;
        next if !defined($target) || !defined($limits->{warning_pct});
        problem("rollover plan ledger '$id' survivor is not below the $limits->{warning_pct}% warning for $field")
            if $actual->{$field} * 100 >= $target * $limits->{warning_pct};
    }
}

sub prepare_rollover_manifest {
    my ($plan_control, $row, $ledger, $segment_metrics) = @_;
    my $id = $row->{ledger_id};
    my $manifest_rel = $ledger->{archive}{manifest};
    open my $fh, '<:raw', absolute($manifest_rel) or do {
        problem("rollover plan ledger '$id' cannot read manifest '$manifest_rel'");
        return;
    };
    my (@raw, @objects);
    while (my $line = <$fh>) {
        chomp $line;
        $line =~ s/\r\z//;
        next if $line eq '';
        my $object = eval { decode_json($line) };
        if (!$object || ref($object) ne 'HASH') {
            problem("rollover plan ledger '$id' manifest contains invalid JSON");
            close $fh;
            return;
        }
        push @raw, $line;
        push @objects, $object;
    }
    close $fh;
    my $control = shift @objects;
    my $control_raw = shift @raw;
    my @segments = grep { ($_->{record_type} // '') eq 'sealed_segment' } @objects;
    my $prior_chain = validate_archive_chain(
        $id, $ledger->{source}, $ledger->{archive}{source_capsule}, [map { +{%$_} } @segments]
    );
    my $successor = $row->{successor};
    problem("rollover plan ledger '$id' successor is not the current newest segment")
        if !defined($prior_chain) || (@$prior_chain < 3) || $prior_chain->[1] ne $successor;
    my @successor_matches = grep {
        ($_->{record_type} // '') eq 'sealed_segment' && ($_->{path} // '') eq $successor
    } @objects;
    problem("rollover plan ledger '$id' successor matches " . scalar(@successor_matches) . ' manifest rows')
        if @successor_matches != 1;
    return if @successor_matches != 1;
    problem("rollover plan ledger '$id' successor is not attached to the live root")
        if ($successor_matches[0]{predecessor} // '') ne $ledger->{source};

    my @updated_raw;
    my @updated_objects;
    for my $index (0 .. $#objects) {
        my $object = { %{ $objects[$index] } };
        if (($object->{record_type} // '') eq 'sealed_segment' && ($object->{path} // '') eq $successor) {
            $object->{predecessor} = $row->{segment_path};
            push @updated_raw, $json->encode($object);
        } else {
            push @updated_raw, $raw[$index];
        }
        push @updated_objects, $object;
    }
    my $new_entry = {
        record_type => 'sealed_segment',
        ledger_id => $id,
        segment_id => $row->{segment_id},
        path => $row->{segment_path},
        sealed_on => $plan_control->{sealed_on},
        reason => $row->{reason},
        sha256 => $row->{sha256},
        records => $segment_metrics->{records},
        lines => $segment_metrics->{lines},
        bytes => $segment_metrics->{bytes},
        line_bytes => $segment_metrics->{line_bytes},
        first_record_sha256 => $row->{first_record_sha256},
        last_record_sha256 => $row->{last_record_sha256},
        verifier => 'scripts/check_rolling_ledger_protocol.pl',
        predecessor => $ledger->{source},
        successor => $successor,
    };
    push @updated_objects, $new_entry;
    push @updated_raw, $json->encode($new_entry);
    my $bytes = join("\n", $control_raw, @updated_raw) . "\n";
    my @lengths = map { length($_) } ($control_raw, @updated_raw);
    validate_archive_manifest_limits($control, length($bytes), \@updated_objects, \@lengths, $id);
    validate_value_bounds($_, 'archive manifest record', 1, $control->{max_scalar_bytes})
        for @updated_objects;
    my @updated_segments = grep { ($_->{record_type} // '') eq 'sealed_segment' } @updated_objects;
    my $chain = validate_archive_chain(
        $id, $ledger->{source}, $ledger->{archive}{source_capsule}, \@updated_segments
    );
    return { bytes => $bytes, prior_chain => $prior_chain, chain => $chain, segments => \@updated_segments };
}

sub render_archive_index {
    my ($id, $index_rel, $manifest_rel, $chain, $segments) = @_;
    my %segment_id = map { ($_->{path} // '') => ($_->{segment_id} // '') } @$segments;
    my @lines = (
        "# `$id` Rolling-Ledger Archive",
        '',
        'Complete newest-to-oldest chronology:',
        '',
    );
    for my $index (0 .. $#$chain) {
        my $path = $chain->[$index];
        my $label = $index == 0 ? 'Current root'
            : $index == $#$chain ? 'Immutable source capsule'
            : "Sealed segment `$segment_id{$path}`";
        push @lines, ($index + 1) . ". [$label](" . relative_markdown_path($index_rel, $path) . ')';
    }
    push @lines, '', 'Authority: [bounded JSONL manifest]('
        . relative_markdown_path($index_rel, $manifest_rel) . ').';
    return join("\n", @lines) . "\n";
}

sub relative_markdown_path {
    my ($document_rel, $target_rel) = @_;
    my $relative = File::Spec->abs2rel($target_rel, dirname($document_rel));
    $relative =~ s{\\}{/}g;
    return $relative;
}

sub install_rollover_transaction {
    my ($control, $prepared, $inject_failure) = @_;
    my $stage_rel = "generated/.rolling-ledger-transaction.$$";
    my $stage = absolute($stage_rel);
    if (-e $stage) {
        problem("rollover transaction stage already exists: $stage_rel");
        return;
    }
    make_path($stage) or do {
        problem("cannot create rollover transaction stage '$stage_rel'");
        return;
    };
    my $context = { stage => $stage, backups => {}, installed_segments => [] };
    my $ok = eval {
        my $root_device = (stat($root))[0];
        my $stage_device = (stat($stage))[0];
        die "staging directory is not on the repository filesystem\n"
            if !defined($root_device) || !defined($stage_device) || $root_device != $stage_device;
        for my $entry (@$prepared) {
            for my $pair (
                [$entry->{segment_path}, $entry->{segment_bytes}],
                [$entry->{manifest_path}, $entry->{manifest_bytes}],
                [$entry->{index_path}, $entry->{index_bytes}],
                [$entry->{source}, $entry->{root_bytes}],
            ) {
                my ($relative, $bytes) = @$pair;
                my $staged = File::Spec->catfile($stage, split m{/}, $relative);
                make_path(dirname($staged));
                write_new_file($staged, $bytes);
                my $staged_bytes = slurp_or_die($staged);
                die "staged output identity drift for '$relative'\n" if $staged_bytes ne $bytes;
            }
            for my $relative ($entry->{source}, $entry->{manifest_path}, $entry->{index_path}) {
                die "authority input '$relative' is missing before transaction\n" if !-f absolute($relative);
                $context->{backups}{$relative} = slurp_or_die(absolute($relative));
            }
        }
        for my $entry (@$prepared) {
            my $destination = absolute($entry->{segment_path});
            write_new_file($destination, $entry->{segment_bytes});
            push @{ $context->{installed_segments} }, $entry->{segment_path};
        }
        die "injected failure after segment installation\n"
            if defined($inject_failure) && $inject_failure eq 'after_segments';
        for my $field (qw(manifest index root)) {
            for my $entry (@$prepared) {
                my ($relative, $bytes) = $field eq 'manifest'
                    ? ($entry->{manifest_path}, $entry->{manifest_bytes})
                    : $field eq 'index'
                    ? ($entry->{index_path}, $entry->{index_bytes})
                    : ($entry->{source}, $entry->{root_bytes});
                atomic_replace($relative, $bytes);
            }
        }
        1;
    };
    if (!$ok) {
        my $failure = $@ || 'unknown transaction failure';
        my $rollback_ok = eval { rollback_rollover_transaction($context); 1 };
        problem("rollover transaction failed: $failure");
        problem("rollover transaction rollback failed: " . ($@ || 'unknown failure')) if !$rollback_ok;
        return;
    }
    return $context;
}

sub atomic_replace {
    my ($relative, $bytes) = @_;
    my $destination = absolute($relative);
    my $temporary = "$destination.rolling-ledger-tmp.$$";
    write_new_file($temporary, $bytes);
    if (!rename $temporary, $destination) {
        my $failure = $!;
        unlink $temporary;
        die "cannot atomically replace '$relative': $failure\n";
    }
}

sub write_new_file {
    my ($path, $bytes) = @_;
    sysopen my $fh, $path, O_WRONLY | O_CREAT | O_EXCL, 0644
        or die "cannot create '$path' exclusively: $!\n";
    binmode $fh, ':raw';
    if (!print {$fh} $bytes) {
        my $failure = $!;
        close $fh;
        unlink $path;
        die "cannot write '$path': $failure\n";
    }
    close $fh or do {
        my $failure = $!;
        unlink $path;
        die "cannot close '$path': $failure\n";
    };
}

sub slurp_or_die {
    my ($path) = @_;
    open my $fh, '<:raw', $path or die "cannot read '$path': $!\n";
    local $/;
    my $bytes = <$fh> // '';
    close $fh or die "cannot close '$path': $!\n";
    return $bytes;
}

sub git_blob_bytes {
    my ($commit, $relative, $id) = @_;
    open my $fh, '-|', 'git', '-C', $root, 'show', "$commit:$relative" or do {
        problem("rollover plan ledger '$id' cannot open committed boundary blob");
        return;
    };
    binmode $fh, ':raw';
    local $/;
    my $bytes = <$fh>;
    if (!close $fh) {
        problem("rollover plan ledger '$id' cannot read '$relative' at boundary '$commit'");
        return;
    }
    return $bytes // '';
}

sub rollback_rollover_transaction {
    my ($context) = @_;
    for my $relative (sort keys %{ $context->{backups} }) {
        atomic_replace($relative, $context->{backups}{$relative});
    }
    for my $relative (reverse @{ $context->{installed_segments} }) {
        my $path = absolute($relative);
        unlink $path or die "cannot remove rolled-back segment '$relative': $!\n" if -e $path;
    }
    cleanup_rollover_stage($context->{stage});
}

sub cleanup_rollover_stage {
    my ($stage) = @_;
    remove_tree($stage) if defined($stage) && -e $stage;
    die "rollover transaction stage survived cleanup: '$stage'\n" if defined($stage) && -e $stage;
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

sub render_live_view {
    my ($parsed, $selected, $grammar) = @_;
    my $bytes = $parsed->{prologue} . join('', map { $_->{bytes} } @$selected) . $parsed->{trailer};
    if (($grammar->{kind} // '') eq 'h2_records_v1' && $parsed->{trailer} eq '') {
        $bytes =~ s/\r?\n\r?\n\z/\n/;
    }
    return $bytes;
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

sub validate_surface_binding {
    my ($ledger, $id, $source) = @_;
    my $surface = $surface_by_source{$source};
    if (ref($surface) ne 'HASH') {
        problem("ledger '$id' lacks one generic rolling-ledger surface authority for '$source'");
        return;
    }
    problem("ledger '$id' generic surface locator must be file") if ($surface->{locator} // '') ne 'file';
    problem("ledger '$id' generic surface state must be normal") if ($surface->{state} // '') ne 'normal';
    problem("ledger '$id' generic surface verifier must be builtin:budget")
        if ($surface->{verifier} // '') ne 'builtin:budget';
    my $manifest = ref($ledger->{archive}) eq 'HASH' ? ($ledger->{archive}{manifest} // '') : '';
    problem("ledger '$id' generic surface archive manifest differs from the focused registry")
        if ($surface->{archive_manifest} // '') ne $manifest;
    for my $object_field (qw(health_targets enforcement_ceilings milestones)) {
        problem("ledger '$id' generic surface lacks '$object_field'")
            if ref($surface->{$object_field}) ne 'HASH';
    }
    my $limits = $ledger->{live_limits};
    if (ref($limits) eq 'HASH' && ref($surface->{enforcement_ceilings}) eq 'HASH') {
        my %mapping = (lines => 'lines_each', bytes => 'bytes_each', line_bytes => 'line_bytes_each');
        for my $field (sort keys %mapping) {
            my $surface_field = $mapping{$field};
            problem("ledger '$id' focused $field ceiling differs from generic $surface_field")
                if ($limits->{$field} // -1) != ($surface->{enforcement_ceilings}{$surface_field} // -2);
        }
    }
    if (ref($limits) eq 'HASH' && ref($surface->{milestones}) eq 'HASH') {
        for my $field (qw(warning_pct rollover_pct)) {
            problem("ledger '$id' focused $field differs from generic surface authority")
                if ($limits->{$field} // -1) != ($surface->{milestones}{$field} // -2);
        }
    }
    return $surface;
}

sub validate_limits {
    my ($limits, $surface, $actual, $id, $label) = @_;
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
    my $health = ref($surface) eq 'HASH' ? $surface->{health_targets} : undef;
    my %health_field = (lines => 'lines_each', bytes => 'bytes_each', line_bytes => 'line_bytes_each');
    for my $field (qw(records lines bytes line_bytes)) {
        next if !defined($limits->{$field}) || ref($limits->{$field}) || $limits->{$field} !~ /\A\d+\z/;
        problem("ledger '$id' $label exceeds live $field limit: $actual->{$field} > $limits->{$field}")
            if $actual->{$field} > $limits->{$field};
        my $target = $field eq 'records' ? $limits->{$field}
            : ref($health) eq 'HASH' ? $health->{ $health_field{$field} } : undef;
        if (!defined($target) || ref($target) || $target !~ /\A\d+\z/ || $target < 1) {
            problem("ledger '$id' lacks positive generic health target for $field");
            next;
        }
        if ($label eq 'planned survivor') {
            problem("ledger '$id' planned survivor starts at or above the $limits->{warning_pct}% warning for $field")
                if $actual->{$field} * 100 >= $target * $limits->{warning_pct};
        } elsif ($label eq 'live source') {
            problem("ledger '$id' live source reached the $limits->{rollover_pct}% rollover threshold for $field")
                if $actual->{$field} * 100 >= $target * $limits->{rollover_pct}
                && ($post_apply_validation || !$planned_rollover_ids{$id});
        }
    }
}

sub validate_archive_contract {
    my ($archive, $id, $state, $source, $metrics, $parsed, $grammar) = @_;
    if (ref($archive) ne 'HASH') {
        problem("ledger '$id' archive must be an object");
        return;
    }
    for my $field (qw(landing manifest index directory source_capsule verifier)) {
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
    for my $field (qw(manifest index source_capsule)) {
        my $path = $archive->{$field} // '';
        problem("ledger '$id' archive $field must be below '$directory/'")
            if $directory ne '' && index($path, "$directory/") != 0;
    }
    return if $state ne 'migrated';
    for my $field (qw(landing manifest index source_capsule verifier)) {
        my $path = $archive->{$field} // '';
        problem("ledger '$id' migrated archive '$path' is missing")
            if safe_relative($path) && !-f absolute($path);
    }
    my $chain = validate_archive_manifest(
        $archive->{manifest}, $id, $source, $archive->{source_capsule}, $metrics, $parsed, $grammar
    );
    validate_archive_index($archive->{index}, $id, $archive->{manifest}, $chain) if defined $chain;
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
    validate_archive_manifest_limits($control, $file_bytes, \@objects, \@raw_lengths, $id);
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
        validate_manifest_ledger_membership($entry, $id);
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

    my @segments = grep {
        ($_->{record_type} // '') eq 'sealed_segment' && ($_->{ledger_id} // '') eq $id
    } @objects;
    for my $segment (@segments) {
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
    return validate_archive_chain($id, $source, $capsule, \@segments);
}

sub validate_archive_manifest_limits {
    my ($control, $file_bytes, $objects, $raw_lengths, $id) = @_;
    problem("ledger '$id' archive manifest exceeds its max_records")
        if defined($control->{max_records}) && @$objects > $control->{max_records};
    problem("ledger '$id' archive manifest exceeds its max_bytes")
        if defined($control->{max_bytes}) && $file_bytes > $control->{max_bytes};
    problem("ledger '$id' archive manifest contains a record above max_record_bytes")
        if defined($control->{max_record_bytes})
        && grep { $_ > $control->{max_record_bytes} } @$raw_lengths;
}

sub validate_manifest_ledger_membership {
    my ($entry, $id) = @_;
    problem("ledger '$id' archive manifest contains foreign ledger_id '" . ($entry->{ledger_id} // '') . "'")
        if ($entry->{ledger_id} // '') ne $id;
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

sub validate_archive_chain {
    my ($id, $source, $capsule, $segments) = @_;
    my %by_path;
    my %by_predecessor;
    for my $segment (@$segments) {
        my $path = $segment->{path} // '';
        my $predecessor = scalar_field($segment, 'predecessor', 'sealed segment manifest record');
        my $successor = scalar_field($segment, 'successor', 'sealed segment manifest record');
        if ($path ne '' && exists $by_path{$path}) {
            problem("ledger '$id' segment path '$path' is duplicated in chronology");
        } elsif ($path ne '') {
            $by_path{$path} = $segment;
        }
        for my $pair ([predecessor => $predecessor], [successor => $successor]) {
            my ($field, $value) = @$pair;
            problem("ledger '$id' segment '$path' $field '$value' is unsafe")
                if defined($value) && !safe_relative($value);
        }
        if (defined $predecessor) {
            problem("ledger '$id' has duplicate chronology edge after '$predecessor'")
                if exists $by_predecessor{$predecessor};
            $by_predecessor{$predecessor} = $segment;
        }
    }

    my @ordered = ($source);
    my %visited;
    my $predecessor = $source;
    while (my $segment = $by_predecessor{$predecessor}) {
        my $path = $segment->{path} // '';
        if ($visited{$path}++) {
            problem("ledger '$id' archive chronology contains a cycle at '$path'");
            last;
        }
        push @ordered, $path;
        my $next = $by_predecessor{$path};
        my $expected_successor = $next ? ($next->{path} // '') : $capsule;
        problem("ledger '$id' segment '$path' successor mismatch: actual '"
            . ($segment->{successor} // '') . "', expected '$expected_successor'")
            if ($segment->{successor} // '') ne $expected_successor;
        $predecessor = $path;
    }
    push @ordered, $capsule;
    problem("ledger '$id' archive chronology does not start at live source '$source'")
        if @$segments && !exists $by_predecessor{$source};
    my $visited_count = scalar keys %visited;
    problem("ledger '$id' archive chronology is disconnected: visited $visited_count of " . scalar(@$segments) . ' segments')
        if $visited_count != @$segments;
    my %reported_cycle;
    for my $start (keys %by_path) {
        my %trail;
        my $cursor = $start;
        while (my $segment = $by_path{$cursor}) {
            if ($trail{$cursor}++) {
                problem("ledger '$id' archive chronology contains a cycle at '$cursor'")
                    if !$reported_cycle{$cursor}++;
                last;
            }
            my $successor = $segment->{successor} // '';
            last if !exists $by_path{$successor};
            $cursor = $successor;
        }
    }
    return \@ordered;
}

sub validate_archive_index {
    my ($index_rel, $id, $manifest_rel, $chain) = @_;
    return if !safe_relative($index_rel) || !-f absolute($index_rel);
    my $bytes = slurp(absolute($index_rel), "ledger '$id' archive index");
    return if !defined $bytes;
    validate_archive_index_content($bytes, $index_rel, $id, $manifest_rel, $chain);
}

sub validate_archive_index_content {
    my ($bytes, $index_rel, $id, $manifest_rel, $chain) = @_;
    my @links = markdown_link_paths($bytes, $index_rel);
    my %counts;
    $counts{$_}++ for @links;
    for my $path (@$chain, $manifest_rel) {
        problem("ledger '$id' archive index must link '$path' exactly once")
            if ($counts{$path} // 0) != 1;
    }
    my %chain_member = map { $_ => 1 } @$chain;
    my @actual_order = grep { $chain_member{$_} } @links;
    problem("ledger '$id' archive index member order differs from verified chronology")
        if join("\0", @actual_order) ne join("\0", @$chain);
    my %allowed = (%chain_member, $manifest_rel => 1);
    for my $path (@links) {
        problem("ledger '$id' archive index links undeclared member '$path'") if !$allowed{$path};
    }
}

sub validate_archive_landings {
    my ($control, $ledgers) = @_;
    my @migrated = grep { ($_->{migration_state} // '') eq 'migrated' } @$ledgers;
    return if !@migrated;
    my %landing_seen;
    my %index_seen;
    my %manifest_seen;
    my %directory_seen;
    my %retired = map { $_ => 1 } grep { defined($_) && !ref($_) } @{ $control->{retired_paths} // [] };
    my @ids;
    my @expected_links;
    for my $ledger (@migrated) {
        my $id = $ledger->{ledger_id} // '';
        my $archive = $ledger->{archive};
        next if ref($archive) ne 'HASH';
        my $landing = $archive->{landing} // '';
        $landing_seen{$landing}++ if $landing ne '';
        for my $pair ([index => \%index_seen], [manifest => \%manifest_seen], [directory => \%directory_seen]) {
            my ($field, $seen) = @$pair;
            my $path = $archive->{$field} // '';
            problem("ledger '$id' reuses archive $field '$path'") if $path ne '' && $seen->{$path}++;
            problem("ledger '$id' archive $field still uses retired path '$path'") if $retired{$path};
        }
        push @ids, $id;
        push @expected_links, $ledger->{source}, $archive->{index}, $archive->{manifest};
    }
    problem('migrated rolling ledgers must share exactly one archive landing') if keys(%landing_seen) != 1;
    my ($landing_rel) = keys %landing_seen;
    return if !defined($landing_rel) || !safe_relative($landing_rel) || !-f absolute($landing_rel);
    my $bytes = slurp(absolute($landing_rel), 'rolling-ledger archive landing');
    return if !defined $bytes;
    validate_archive_landing_content($bytes, $landing_rel, \@ids, \@expected_links);
}

sub validate_archive_landing_content {
    my ($bytes, $landing_rel, $ids, $expected_links) = @_;
    my @headings = $bytes =~ /^## `([^`]+)`\s*$/mg;
    problem('rolling-ledger archive landing ledger order/membership differs from the registry')
        if join("\0", @headings) ne join("\0", @$ids);
    my @links = markdown_link_paths($bytes, $landing_rel);
    my %counts;
    $counts{$_}++ for @links;
    my %allowed = map { $_ => 1 } @$expected_links;
    for my $path (@$expected_links) {
        problem("rolling-ledger archive landing must link '$path' exactly once")
            if ($counts{$path} // 0) != 1;
    }
    for my $path (@links) {
        problem("rolling-ledger archive landing links undeclared route '$path'") if !$allowed{$path};
    }
}

sub markdown_link_paths {
    my ($bytes, $document_rel) = @_;
    my $base = dirname($document_rel);
    my @paths;
    while ($bytes =~ /\]\(([^)]+)\)/g) {
        my $href = $1;
        $href =~ s/^<|>$//g;
        $href =~ s/#.*\z//;
        next if $href eq '' || $href =~ m{\A[a-z][a-z0-9+.-]*:}i || $href =~ m{\A/};
        my $path = normalize_relative_link($base, $href);
        push @paths, $path if defined $path;
    }
    return @paths;
}

sub normalize_relative_link {
    my ($base, $href) = @_;
    return if $href =~ /[\x00\r\n?]/;
    my @parts;
    for my $part (split m{/}, "$base/$href") {
        next if $part eq '' || $part eq '.';
        if ($part eq '..') {
            return if !@parts;
            pop @parts;
        } else {
            push @parts, $part;
        }
    }
    return join '/', @parts;
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
        my $live_bytes = $live->[$offset + $index]{bytes};
        my $selected_bytes = $selected->[$index]{bytes};
        my $matches = $live_bytes eq $selected_bytes;
        if (!$matches && $index == $#$selected) {
            my $without_successor_separator = $selected_bytes;
            $without_successor_separator =~ s/\r?\n\r?\n\z/\n/;
            $matches = $live_bytes eq $without_successor_separator;
        }
        problem("ledger '$id' retained record identity/order changed at survivor index " . ($index + 1))
            if !$matches;
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
        { health_targets => { lines_each => 10, bytes_each => 100, line_bytes_each => 10 } },
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

    my $source = 'CURRENT.md';
    my $capsule = 'docs/archive/rolling-ledgers/fixture/capsule.md';
    my @segments = (
        {
            path => 'docs/archive/rolling-ledgers/fixture/segment-0002.md',
            predecessor => $source,
            successor => 'docs/archive/rolling-ledgers/fixture/segment-0001.md',
        },
        {
            path => 'docs/archive/rolling-ledgers/fixture/segment-0001.md',
            predecessor => 'docs/archive/rolling-ledgers/fixture/segment-0002.md',
            successor => $capsule,
        },
    );
    @errors = ();
    my $chain = validate_archive_chain('fixture', $source, $capsule, [map { +{%$_} } @segments]);
    push @failures, 'a valid archive chronology failed' if @errors;
    push @failures, 'valid archive chronology order drifted'
        if join("\0", @$chain) ne join("\0", $source, (map { $_->{path} } @segments), $capsule);
    $checks++;

    @errors = ();
    my @missing_edge = map { +{%$_} } @segments;
    delete $missing_edge[0]{predecessor};
    validate_archive_chain('fixture', $source, $capsule, \@missing_edge);
    push @failures, 'a missing predecessor edge was accepted' if !@errors;
    $checks++;

    @errors = ();
    my @duplicate_edge = map { +{%$_} } @segments;
    $duplicate_edge[1]{predecessor} = $source;
    validate_archive_chain('fixture', $source, $capsule, \@duplicate_edge);
    push @failures, 'a duplicate predecessor edge was accepted' if !@errors;
    $checks++;

    @errors = ();
    my @broken_successor = map { +{%$_} } @segments;
    $broken_successor[0]{successor} = $capsule;
    validate_archive_chain('fixture', $source, $capsule, \@broken_successor);
    push @failures, 'a broken successor edge was accepted' if !@errors;
    $checks++;

    @errors = ();
    my @disconnected_cycle = map { +{%$_} } @segments;
    $disconnected_cycle[1]{predecessor} = $disconnected_cycle[1]{path};
    $disconnected_cycle[1]{successor} = $disconnected_cycle[1]{path};
    validate_archive_chain('fixture', $source, $capsule, \@disconnected_cycle);
    push @failures, 'a disconnected chronology cycle was accepted' if !@errors;
    $checks++;

    @errors = ();
    validate_manifest_ledger_membership({ ledger_id => 'foreign' }, 'fixture');
    push @failures, 'a foreign-ledger manifest record was accepted' if !@errors;
    $checks++;

    @errors = ();
    validate_archive_manifest_limits(
        { max_records => 1, max_bytes => 10, max_record_bytes => 4 },
        11,
        [{}, {}],
        [5, 1],
        'fixture',
    );
    push @failures, 'manifest record/byte limits were not enforced' if @errors < 3;
    $checks++;

    my $index_rel = 'docs/archive/rolling-ledgers/fixture/INDEX.md';
    my $manifest_rel = 'docs/archive/rolling-ledgers/fixture/manifest.jsonl';
    my $valid_index = join '',
        "[Current](../../../../CURRENT.md)\n",
        "[New segment](segment-0002.md)\n",
        "[Old segment](segment-0001.md)\n",
        "[Capsule](capsule.md)\n",
        "[Manifest](manifest.jsonl)\n";
    @errors = ();
    validate_archive_index_content($valid_index, $index_rel, 'fixture', $manifest_rel, $chain);
    push @failures, 'a complete ordered archive index failed' if @errors;
    $checks++;

    @errors = ();
    (my $missing_index_member = $valid_index) =~ s/^\[Old segment\].*\n//m;
    validate_archive_index_content($missing_index_member, $index_rel, 'fixture', $manifest_rel, $chain);
    push @failures, 'an archive index missing a member was accepted' if !@errors;
    $checks++;

    @errors = ();
    my $duplicate_index_member = $valid_index . "[Duplicate](segment-0001.md)\n";
    validate_archive_index_content($duplicate_index_member, $index_rel, 'fixture', $manifest_rel, $chain);
    push @failures, 'an archive index with a duplicate member was accepted' if !@errors;
    $checks++;

    @errors = ();
    (my $wrong_index_order = $valid_index) =~ s{(\[New segment\].*\n)(\[Old segment\].*\n)}{$2$1};
    validate_archive_index_content($wrong_index_order, $index_rel, 'fixture', $manifest_rel, $chain);
    push @failures, 'an archive index with wrong chronology order was accepted' if !@errors;
    $checks++;

    @errors = ();
    my $landing_rel = 'docs/archive/rolling-ledgers/INDEX.md';
    my @landing_links = ($source, $index_rel, $manifest_rel);
    my $valid_landing = "## `fixture`\n"
        . "[Current](../../../CURRENT.md)\n"
        . "[Index](fixture/INDEX.md)\n"
        . "[Manifest](fixture/manifest.jsonl)\n";
    validate_archive_landing_content($valid_landing, $landing_rel, ['fixture'], \@landing_links);
    push @failures, 'a complete archive landing failed' if @errors;
    $checks++;

    @errors = ();
    (my $missing_landing_route = $valid_landing) =~ s/^\[Manifest\].*\n//m;
    validate_archive_landing_content($missing_landing_route, $landing_rel, ['fixture'], \@landing_links);
    push @failures, 'an archive landing missing a route was accepted' if !@errors;
    $checks++;

    @errors = ();
    %planned_rollover_ids = ();
    $post_apply_validation = 0;
    my $health_surface = {
        health_targets => { lines_each => 100, bytes_each => 1_000, line_bytes_each => 100 },
    };
    validate_limits(
        { records => 20, lines => 200, bytes => 2_000, line_bytes => 200,
          warning_pct => 80, rollover_pct => 90 },
        $health_surface,
        { records => 1, lines => 90, bytes => 1, line_bytes => 1 },
        'self-test health denominator', 'live source',
    );
    push @failures, 'live rollover still used the enforcement ceiling as its denominator' if !@errors;
    $checks++;

    @errors = ();
    validate_warning_safe_survivor(
        { records => 20, lines => 200, bytes => 2_000, line_bytes => 200,
          warning_pct => 80, rollover_pct => 90 },
        $health_surface,
        { records => 15, lines => 80, bytes => 1, line_bytes => 1 },
        'self-test warning-safe survivor',
    );
    push @failures, 'a rollover survivor at a generic health warning was accepted' if !@errors;
    $checks++;

    my %saved_surfaces = %surface_by_source;
    %surface_by_source = (
        'CURRENT.md' => {
            locator => 'file', lifecycle => 'rolling_ledger', state => 'normal',
            verifier => 'builtin:budget', archive_manifest => 'archive/manifest.jsonl',
            health_targets => { lines_each => 100, bytes_each => 1_000, line_bytes_each => 100 },
            enforcement_ceilings => { lines_each => 200, bytes_each => 2_000, line_bytes_each => 200 },
            milestones => { warning_pct => 80, rollover_pct => 90 },
        },
    );
    my $binding_ledger = {
        live_limits => { records => 20, lines => 200, bytes => 2_000, line_bytes => 200,
            warning_pct => 80, rollover_pct => 90 },
        archive => { manifest => 'archive/manifest.jsonl' },
    };
    @errors = ();
    validate_surface_binding($binding_ledger, 'fixture', 'CURRENT.md');
    push @failures, 'matching focused/generic surface authorities failed' if @errors;
    $checks++;

    @errors = ();
    $surface_by_source{'CURRENT.md'}{enforcement_ceilings}{lines_each} = 201;
    validate_surface_binding($binding_ledger, 'fixture', 'CURRENT.md');
    push @failures, 'focused/generic ceiling drift was accepted' if !@errors;
    $checks++;
    %surface_by_source = %saved_surfaces;

    my ($rollover_checks, $rollover_failures) = run_rollover_writer_self_test();
    $checks += $rollover_checks;
    push @failures, @$rollover_failures;

    @errors = ();
    my $residue_rel = "generated/rolling-ledger-retired-self-test-$$";
    my $residue_path = absolute($residue_rel);
    sysopen my $residue_fh, $residue_path, O_CREAT | O_EXCL | O_WRONLY
        or die "rolling-ledger self-test: cannot create retired-path fixture: $!\n";
    print {$residue_fh} "retired\n";
    close $residue_fh;
    validate_retired_paths({ retired_paths => [$residue_rel], max_array_items => 1 });
    unlink $residue_path or die "rolling-ledger self-test: cannot remove retired-path fixture: $!\n";
    push @failures, 'a retired manifest residue was accepted' if !@errors;
    $checks++;

    @errors = @saved_errors;
    die "rolling-ledger self-test: $_\n" for @failures;
    print "rolling-ledger: $checks parser/control self-tests pass.\n";
}

sub run_rollover_writer_self_test {
    my @failures;
    my $checks = 0;
    my @saved_errors = @errors;
    my %saved_surfaces = %surface_by_source;
    my %saved_plan_ids = %planned_rollover_ids;
    my $saved_post_apply = $post_apply_validation;
    my $fixture_rel = "generated/.rolling-ledger-writer-self-test.$$";
    my $fixture = absolute($fixture_rel);
    my $transaction_stage = absolute("generated/.rolling-ledger-transaction.$$");
    my $unexpected;
    eval {
        die "rollover writer self-test fixture already exists\n" if -e $fixture || -e $transaction_stage;
        my $archive_rel = "$fixture_rel/archive";
        make_path(absolute($archive_rel));
        my $source_rel = "$fixture_rel/CURRENT.md";
        my $manifest_rel = "$archive_rel/manifest.jsonl";
        my $index_rel = "$archive_rel/INDEX.md";
        my $capsule_rel = "$archive_rel/capsule.md";
        my $old_segment_rel = "$archive_rel/segment-0001-2026-08-09.md";
        my $new_segment_rel = "$archive_rel/segment-0002-2026-08-09.md";
        my $grammar = {
            kind => 'h2_records_v1', skipped_h2 => 0,
            detached_titles => [], trailer_markers => [],
        };
        my $opening = "# Fixture\n\n"
            . "## newest\nA\n\n"
            . "## oldest new\nB\n\n"
            . "## retained\nC\n";
        my $current = "# Fixture\n\n"
            . "## future\nF\n\n"
            . "## newest\nA\n\n"
            . "## oldest new\nB\n\n"
            . "## retained\nC\n";
        my $opening_parsed = parse_ledger($opening, $grammar, 'rollover writer opening fixture');
        my @opening_records = @{ $opening_parsed->{records} };
        my $segment_bytes = $opening_records[1]{bytes};
        $segment_bytes =~ s/\r?\n\r?\n\z/\n/;
        my $segment_metrics = metrics($segment_bytes);
        $segment_metrics->{records} = 1;
        my $segment_parsed = parse_segment($segment_bytes, $grammar, 'rollover writer segment fixture');
        my @opening_survivors = ($opening_records[0], $opening_records[2]);
        my $opening_root = render_live_view($opening_parsed, \@opening_survivors, $grammar);
        my $opening_root_metrics = metrics($opening_root);
        $opening_root_metrics->{records} = 2;
        my $manifest_control = {
            record_type => 'archive_manifest', schema_version => 1,
            max_records => 8, max_bytes => 16_384, max_record_bytes => 2_048, max_scalar_bytes => 768,
        };
        my $capsule = {
            record_type => 'source_capsule', ledger_id => 'fixture', path => $capsule_rel,
        };
        my $old_segment = {
            record_type => 'sealed_segment', ledger_id => 'fixture', segment_id => 'fixture-0001',
            path => $old_segment_rel, predecessor => $source_rel, successor => $capsule_rel,
        };
        my $manifest_bytes = join("\n", map { $json->encode($_) }
            ($manifest_control, $capsule, $old_segment)) . "\n";
        my $index_bytes = "old index\n";
        write_new_file(absolute($source_rel), $current);
        write_new_file(absolute($manifest_rel), $manifest_bytes);
        write_new_file(absolute($index_rel), $index_bytes);
        write_new_file(absolute($capsule_rel), $opening);
        write_new_file(absolute($old_segment_rel), "## archived\nold\n");

        my $limits = {
            records => 16, lines => 200, bytes => 4_096, line_bytes => 200,
            warning_pct => 80, rollover_pct => 90,
        };
        %surface_by_source = (
            $source_rel => {
                health_targets => { lines_each => 100, bytes_each => 2_000, line_bytes_each => 100 },
                enforcement_ceilings => { lines_each => 200, bytes_each => 4_096, line_bytes_each => 200 },
                milestones => { warning_pct => 80, rollover_pct => 90 },
            },
        );
        %planned_rollover_ids = (fixture => 1);
        $post_apply_validation = 0;
        my $ledger = {
            ledger_id => 'fixture', source => $source_rel, grammar => $grammar, live_limits => $limits,
            archive => {
                directory => $archive_rel, manifest => $manifest_rel, index => $index_rel,
                source_capsule => $capsule_rel,
            },
        };
        my $control = { sealed_on => '2026-08-09', plan_id => 'fixture-plan' };
        my $row = {
            ledger_id => 'fixture', source => $source_rel,
            opening_sha256 => sha256_hex($opening), opening_records => 3,
            post_migration_records => 2, retained_migration_suffix_records => 1,
            keep_opening_prefix_records => 1, segment_id => 'fixture-0002',
            segment_path => $new_segment_rel, successor => $old_segment_rel,
            %$segment_metrics, sha256 => sha256_hex($segment_bytes),
            first_record_sha256 => sha256_hex($segment_parsed->[0]{bytes}),
            last_record_sha256 => sha256_hex($segment_parsed->[-1]{bytes}),
            opening_root_after_cut => {
                %$opening_root_metrics, sha256 => sha256_hex($opening_root),
            },
            reason => 'Fixture rollover transaction.',
        };

        @errors = ();
        my $prepared = prepare_rollover_entry($control, $row, $ledger);
        push @failures, 'a valid rollover plan entry failed preparation' if @errors || !defined $prepared;
        $checks++;

        @errors = ();
        my %bad_opening = (%$row, opening_sha256 => ('0' x 64));
        prepare_rollover_entry($control, \%bad_opening, $ledger);
        push @failures, 'opening-boundary identity drift was accepted' if !@errors;
        $checks++;

        @errors = ();
        my %bad_segment = (%$row, sha256 => ('0' x 64));
        prepare_rollover_entry($control, \%bad_segment, $ledger);
        push @failures, 'planned segment identity drift was accepted' if !@errors;
        $checks++;

        @errors = ();
        my %bad_successor = (%$row, successor => $capsule_rel);
        prepare_rollover_entry($control, \%bad_successor, $ledger);
        push @failures, 'a non-newest rollover successor was accepted' if !@errors;
        $checks++;

        @errors = ();
        my $failed_context = install_rollover_transaction($control, [$prepared], 'after_segments');
        push @failures, 'an injected post-segment transaction failure was accepted'
            if defined($failed_context) || !@errors;
        push @failures, 'failed transaction did not restore authority or remove residue'
            if slurp_or_die(absolute($source_rel)) ne $current
            || slurp_or_die(absolute($manifest_rel)) ne $manifest_bytes
            || slurp_or_die(absolute($index_rel)) ne $index_bytes
            || -e absolute($new_segment_rel) || -e $transaction_stage;
        $checks++;

        @errors = ();
        my $context = install_rollover_transaction($control, [$prepared]);
        push @failures, 'valid root-last rollover installation failed' if @errors || !defined $context;
        if (defined $context) {
            push @failures, 'installed rollover root differs from its prepared bytes'
                if slurp_or_die(absolute($source_rel)) ne $prepared->{root_bytes};
            push @failures, 'installed rollover segment differs from its prepared bytes'
                if slurp_or_die(absolute($new_segment_rel)) ne $prepared->{segment_bytes};
        }
        $checks++;

        if (defined $context) {
            rollback_rollover_transaction($context);
            push @failures, 'rollback did not restore the exact source root'
                if slurp_or_die(absolute($source_rel)) ne $current;
            push @failures, 'rollback did not restore the exact manifest'
                if slurp_or_die(absolute($manifest_rel)) ne $manifest_bytes;
            push @failures, 'rollback did not restore the exact index'
                if slurp_or_die(absolute($index_rel)) ne $index_bytes;
            push @failures, 'rollback left a planned segment or stage residue'
                if -e absolute($new_segment_rel) || -e $transaction_stage;
        }
        $checks++;
        1;
    } or $unexpected = $@ || 'unknown rollover writer self-test failure';

    remove_tree($transaction_stage) if -e $transaction_stage;
    remove_tree($fixture) if -e $fixture;
    push @failures, $unexpected if defined $unexpected;
    %surface_by_source = %saved_surfaces;
    %planned_rollover_ids = %saved_plan_ids;
    $post_apply_validation = $saved_post_apply;
    @errors = @saved_errors;
    return ($checks, \@failures);
}
