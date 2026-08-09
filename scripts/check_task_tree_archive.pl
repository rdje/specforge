#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(encode_utf8);
use File::Basename qw(dirname);
use File::Path qw(make_path remove_tree);
use File::Spec;
use JSON::PP;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root;
my $contract_rel = 'doctrine/live_document_size/task_tree_archive.json';
my $report = 0;
my $self_test = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--contract') {
        $contract_rel = shift @ARGV // usage();
    } elsif ($arg eq '--check') {
        # Check is the default operation; retain the flag as a stable public command.
    } elsif ($arg eq '--report') {
        $report = 1;
    } elsif ($arg eq '--self-test') {
        $self_test = 1;
    } else {
        usage();
    }
}

my $project_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'))
    // die "task-tree-archive: cannot resolve repository root\n";
if ($self_test) {
    run_self_test($project_root);
    exit 0;
}

$root //= $project_root;
$root = abs_path($root) // die "task-tree-archive: root does not exist\n";
my ($errors, $result) = validate_tree($root, $contract_rel);
if (@$errors) {
    print STDERR "task-tree-archive: $_\n" for @$errors;
    print STDERR "task-tree-archive: FAILED with ", scalar(@$errors), " violation(s).\n";
    exit 1;
}
print STDERR "task-tree-archive: warning: $_\n" for @{$result->{warnings} // []};

if ($report) {
    print JSON::PP->new->canonical(1)->encode($result), "\n";
} else {
    print "task-tree-archive: $result->{migration_state} source identity and bounded route are valid.\n";
}
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--contract PATH] [--check|--report|--self-test]\n";
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

sub raw_literal {
    my ($value) = @_;
    return '' if !defined $value;
    return utf8::is_utf8($value) ? encode_utf8($value) : $value;
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
    my $device = (stat($path))[0];
    push @$errors, "$label is off the repository volume: $relative"
        if !defined($root_device) || !defined($device) || $root_device != $device;
    open my $fh, '<:raw', $path or do {
        push @$errors, "cannot read $label '$relative': $!";
        return;
    };
    local $/;
    my $raw = <$fh> // '';
    close $fh or push @$errors, "cannot close $label '$relative': $!";
    return $raw;
}

sub metrics {
    my ($raw) = @_;
    my $bytes = length($raw);
    my $lines = () = $raw =~ /\n/g;
    $lines++ if $bytes && $raw !~ /\n\z/;
    my $line_bytes = 0;
    for my $line (split /\n/, $raw, -1) {
        $line =~ s/\r\z//;
        my $length = length($line);
        $line_bytes = $length if $length > $line_bytes;
    }
    return {lines => $lines, bytes => $bytes, line_bytes => $line_bytes};
}

sub occurrences {
    my ($raw, $literal) = @_;
    $literal = raw_literal($literal);
    return 0 if $literal eq '';
    my ($count, $offset) = (0, 0);
    while (1) {
        my $position = index($raw, $literal, $offset);
        last if $position < 0;
        $count++;
        $offset = $position + length($literal);
    }
    return $count;
}

sub reject_unknown {
    my ($object, $label, $errors, @allowed) = @_;
    if (ref($object) ne 'HASH') {
        push @$errors, "$label must be an object";
        return;
    }
    my %allowed = map { $_ => 1 } @allowed;
    push @$errors, "$label has unknown field '$_'" for grep { !$allowed{$_} } sort keys %$object;
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

sub required_literals {
    my ($value, $label, $errors) = @_;
    if (ref($value) ne 'ARRAY' || !@$value) {
        push @$errors, "$label must be a non-empty array";
        return [];
    }
    if (@$value > 32) {
        push @$errors, "$label exceeds 32 items";
    }
    my %seen;
    for my $literal (@$value) {
        if (!defined($literal) || ref($literal) || $literal eq '') {
            push @$errors, "$label contains an empty or non-scalar item";
            next;
        }
        push @$errors, "$label contains a scalar longer than 512 bytes"
            if length(raw_literal($literal)) > 512;
        push @$errors, "$label contains a duplicate literal '$literal'" if $seen{$literal}++;
    }
    return $value;
}

sub validate_metric_object {
    my ($object, $label, $errors) = @_;
    reject_unknown($object, $label, $errors, qw(lines bytes line_bytes));
    my %values;
    $values{$_} = required_positive_integer($object, $_, $label, $errors)
        for qw(lines bytes line_bytes);
    return \%values;
}

sub compare_metrics {
    my ($actual, $expected, $label, $errors) = @_;
    for my $dimension (qw(lines bytes line_bytes)) {
        next if !defined $expected->{$dimension};
        push @$errors, "$label $dimension is $actual->{$dimension}, expected $expected->{$dimension}"
            if $actual->{$dimension} != $expected->{$dimension};
    }
}

sub enforce_limits {
    my ($actual, $limits, $label, $errors) = @_;
    for my $dimension (qw(lines bytes line_bytes)) {
        next if !defined $limits->{$dimension};
        push @$errors, "$label exceeds $dimension ceiling $limits->{$dimension}"
            if $actual->{$dimension} > $limits->{$dimension};
    }
}

sub section_body {
    my ($raw, $heading, $label, $errors) = @_;
    my $marker = "## $heading";
    my $count = occurrences($raw, $marker);
    if ($count != 1) {
        push @$errors, "$label must contain exactly one '$marker' heading (found $count)";
        return '';
    }
    my $quoted = quotemeta(raw_literal($marker));
    return $raw =~ /^$quoted\r?\n(.*?)(?=^## |\z)/ms ? $1 : '';
}

sub normalize_link {
    my ($index_path, $target) = @_;
    return if !defined($target) || $target eq '' || $target =~ /[\x00-\x1f\x7f\\]/;
    return if $target =~ m{\A(?:[a-z][a-z0-9+.-]*:|/|#)}i;
    $target =~ s/[?#].*\z//;
    my @parts = (split(m{/}, dirname($index_path)), split(m{/}, $target));
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

sub resolved_link_counts {
    my ($raw, $source_path, $label, $errors) = @_;
    my %counts;
    while ($raw =~ /\[[^\]\r\n]*\]\(([^)\r\n]+)\)/g) {
        my $resolved = normalize_link($source_path, $1);
        if (!defined $resolved || !safe_relative_path($resolved)) {
            push @$errors, "$label contains an unsafe local link target '$1'";
            next;
        }
        $counts{$resolved}++;
    }
    return \%counts;
}

sub read_json_object {
    my ($base, $relative, $label, $max_bytes, $errors) = @_;
    my $raw = read_regular($base, $relative, $label, $errors);
    return if !defined $raw;
    push @$errors, "$label exceeds $max_bytes bytes" if length($raw) > $max_bytes;
    my $decoded = eval { decode_json($raw) };
    if ($@ || ref($decoded) ne 'HASH') {
        push @$errors, "$label is not one valid JSON object";
        return;
    }
    return ($decoded, $raw);
}

sub validate_contract_schema {
    my ($contract, $errors) = @_;
    reject_unknown(
        $contract,
        'contract',
        $errors,
        qw(schema_version migration_state current_path archive source source_requirements current_root archive_index manifest_limits verifier),
    );
    push @$errors, 'contract schema_version must be 1'
        if !defined($contract->{schema_version}) || ref($contract->{schema_version})
        || $contract->{schema_version} !~ /\A\d+\z/ || $contract->{schema_version} != 1;
    my $state = required_scalar($contract, 'migration_state', 'contract', $errors);
    push @$errors, "contract has invalid migration_state '$state'"
        if defined($state) && $state ne 'source_locked' && $state ne 'migrated';

    my $current = required_scalar($contract, 'current_path', 'contract', $errors);
    push @$errors, 'contract current_path is unsafe' if defined($current) && !safe_relative_path($current);
    my $verifier = required_scalar($contract, 'verifier', 'contract', $errors);
    push @$errors, "contract verifier must be 'perl scripts/check_task_tree_archive.pl --check'"
        if defined($verifier) && $verifier ne 'perl scripts/check_task_tree_archive.pl --check';

    my $archive = $contract->{archive};
    reject_unknown($archive, 'contract archive', $errors, qw(index manifest source_capsule));
    my %paths;
    for my $field (qw(index manifest source_capsule)) {
        my $path = required_scalar($archive, $field, 'contract archive', $errors);
        push @$errors, "contract archive $field path is unsafe"
            if defined($path) && !safe_relative_path($path);
        push @$errors, "contract path '$path' is declared more than once" if defined($path) && $paths{$path}++;
    }
    push @$errors, "contract path '$current' is declared more than once"
        if defined($current) && $paths{$current}++;

    my $source = $contract->{source};
    reject_unknown($source, 'contract source', $errors, qw(sha256 metrics));
    my $sha = required_scalar($source, 'sha256', 'contract source', $errors);
    push @$errors, 'contract source sha256 must be 64 lowercase hexadecimal characters'
        if defined($sha) && $sha !~ /\A[0-9a-f]{64}\z/;
    validate_metric_object($source->{metrics}, 'contract source metrics', $errors);
    required_literals($contract->{source_requirements}, 'contract source_requirements', $errors);

    for my $field (qw(current_root archive_index)) {
        my $spec = $contract->{$field};
        reject_unknown($spec, "contract $field", $errors, qw(health_targets enforcement_ceilings milestones required_literals forbidden_literals));
        my $health = validate_metric_object($spec->{health_targets}, "contract $field health_targets", $errors);
        my $ceilings = validate_metric_object($spec->{enforcement_ceilings}, "contract $field enforcement_ceilings", $errors);
        for my $dimension (qw(lines bytes line_bytes)) {
            next if !defined($health->{$dimension}) || !defined($ceilings->{$dimension});
            push @$errors, "contract $field health $dimension exceeds its ceiling"
                if $health->{$dimension} > $ceilings->{$dimension};
        }
        my $milestones = $spec->{milestones};
        reject_unknown($milestones, "contract $field milestones", $errors, qw(warning_pct rollover_pct));
        my $warning = required_positive_integer($milestones, 'warning_pct', "contract $field milestones", $errors);
        my $rollover = required_positive_integer($milestones, 'rollover_pct', "contract $field milestones", $errors);
        push @$errors, "contract $field milestones must satisfy warning_pct < rollover_pct < 100"
            if defined($warning) && defined($rollover)
            && ($warning >= $rollover || $rollover >= 100);
        required_literals($spec->{required_literals}, "contract $field required_literals", $errors);
        required_literals($spec->{forbidden_literals}, "contract $field forbidden_literals", $errors);
    }
    my $manifest_limits = $contract->{manifest_limits};
    reject_unknown($manifest_limits, 'contract manifest_limits', $errors, qw(bytes line_bytes scalar_bytes));
    required_positive_integer($manifest_limits, $_, 'contract manifest_limits', $errors)
        for qw(bytes line_bytes scalar_bytes);
}

sub validate_required_presence {
    my ($raw, $literals, $label, $errors) = @_;
    for my $literal (@$literals) {
        next if !defined($literal) || ref($literal) || $literal eq '';
        push @$errors, "$label lacks required literal '$literal'" if !occurrences($raw, $literal);
    }
}

sub validate_forbidden_absence {
    my ($raw, $literals, $label, $errors) = @_;
    for my $literal (@$literals) {
        next if !defined($literal) || ref($literal) || $literal eq '';
        push @$errors, "$label contains forbidden literal '$literal'" if occurrences($raw, $literal);
    }
}

sub pressure_warnings {
    my ($actual, $spec, $label) = @_;
    my @warnings;
    my $targets = $spec->{health_targets};
    my $milestones = $spec->{milestones};
    return \@warnings if ref($targets) ne 'HASH' || ref($milestones) ne 'HASH';
    my $warning = $milestones->{warning_pct};
    my $rollover = $milestones->{rollover_pct};
    return \@warnings if !defined($warning) || !defined($rollover)
        || ref($warning) || ref($rollover) || $warning !~ /\A\d+\z/ || $rollover !~ /\A\d+\z/;
    for my $dimension (qw(lines bytes line_bytes)) {
        next if !defined($targets->{$dimension}) || ref($targets->{$dimension})
            || $targets->{$dimension} !~ /\A\d+\z/ || !$targets->{$dimension};
        my $percent = 100 * $actual->{$dimension} / $targets->{$dimension};
        if ($percent >= $rollover) {
            push @warnings, sprintf('%s %s is at or above rollover (%.1f%%)', $label, $dimension, $percent);
        } elsif ($percent >= $warning) {
            push @warnings, sprintf('%s %s is at or above warning (%.1f%%)', $label, $dimension, $percent);
        }
    }
    return \@warnings;
}

sub validate_source {
    my ($base, $path, $contract, $errors) = @_;
    my $raw = read_regular($base, $path, 'source authority', $errors);
    return if !defined $raw;
    my $actual = metrics($raw);
    my $expected = $contract->{source}{metrics};
    compare_metrics($actual, $expected, 'source authority', $errors) if ref($expected) eq 'HASH';
    my $sha = sha256_hex($raw);
    push @$errors, "source authority sha256 is $sha, expected $contract->{source}{sha256}"
        if defined($contract->{source}{sha256}) && $sha ne $contract->{source}{sha256};
    validate_required_presence($raw, $contract->{source_requirements}, 'source authority', $errors)
        if ref($contract->{source_requirements}) eq 'ARRAY';
    return ($raw, $actual, $sha);
}

sub validate_migrated_manifest {
    my ($base, $contract, $errors) = @_;
    my $limits = $contract->{manifest_limits};
    my ($manifest, $raw) = read_json_object(
        $base,
        $contract->{archive}{manifest},
        'archive manifest',
        $limits->{bytes} // 1,
        $errors,
    );
    return if !defined $manifest;
    my $manifest_metrics = metrics($raw);
    push @$errors, "archive manifest exceeds line_bytes ceiling $limits->{line_bytes}"
        if $manifest_metrics->{line_bytes} > ($limits->{line_bytes} // 0);
    reject_unknown(
        $manifest,
        'archive manifest',
        $errors,
        qw(schema_version current_path source_capsule source_sha256 source_metrics source_boundary_commit sealed_date reason verifier),
    );
    push @$errors, 'archive manifest schema_version must be 1'
        if !defined($manifest->{schema_version}) || ref($manifest->{schema_version}) || $manifest->{schema_version} != 1;
    for my $field (qw(current_path source_capsule source_sha256 source_boundary_commit sealed_date reason verifier)) {
        my $value = required_scalar($manifest, $field, 'archive manifest', $errors);
        push @$errors, "archive manifest scalar '$field' exceeds $limits->{scalar_bytes} bytes"
            if defined($value) && length(raw_literal($value)) > ($limits->{scalar_bytes} // 0);
    }
    push @$errors, 'archive manifest current_path disagrees with contract'
        if defined($manifest->{current_path}) && $manifest->{current_path} ne $contract->{current_path};
    push @$errors, 'archive manifest source_capsule disagrees with contract'
        if defined($manifest->{source_capsule}) && $manifest->{source_capsule} ne $contract->{archive}{source_capsule};
    push @$errors, 'archive manifest source_sha256 disagrees with contract'
        if defined($manifest->{source_sha256}) && $manifest->{source_sha256} ne $contract->{source}{sha256};
    push @$errors, 'archive manifest verifier disagrees with contract'
        if defined($manifest->{verifier}) && $manifest->{verifier} ne $contract->{verifier};
    push @$errors, 'archive manifest source_boundary_commit must be a full lowercase Git object id'
        if defined($manifest->{source_boundary_commit}) && $manifest->{source_boundary_commit} !~ /\A[0-9a-f]{40}\z/;
    my $manifest_source_metrics = validate_metric_object(
        $manifest->{source_metrics},
        'archive manifest source_metrics',
        $errors,
    );
    compare_metrics($manifest_source_metrics, $contract->{source}{metrics}, 'archive manifest source_metrics', $errors)
        if ref($contract->{source}{metrics}) eq 'HASH';
    return ($manifest, $manifest_metrics);
}

sub validate_tree {
    my ($base, $relative_contract) = @_;
    my @errors;
    if (!safe_relative_path($relative_contract)) {
        push @errors, 'contract path is absolute, escaping, or malformed';
        return (\@errors, {});
    }
    my ($contract) = read_json_object($base, $relative_contract, 'contract', 65_536, \@errors);
    return (\@errors, {}) if !defined $contract;
    validate_contract_schema($contract, \@errors);

    my $state = $contract->{migration_state} // '';
    my $current = $contract->{current_path};
    my $source_path = $state eq 'migrated' ? $contract->{archive}{source_capsule} : $current;
    my (undef, $source_metrics, $source_sha) = validate_source($base, $source_path, $contract, \@errors)
        if safe_relative_path($source_path);

    my %result = (
        migration_state => $state,
        source_path => $source_path,
        source_sha256 => $source_sha // '',
        source_metrics => $source_metrics // {},
        warnings => [],
    );

    if ($state eq 'source_locked') {
        for my $field (qw(index manifest source_capsule)) {
            my $path = $contract->{archive}{$field};
            next if !safe_relative_path($path);
            my $absolute = absolute($base, $path);
            push @errors, "source_locked state has premature archive $field at '$path'"
                if -e $absolute || -l $absolute;
        }
    } elsif ($state eq 'migrated') {
        my $current_raw = read_regular($base, $current, 'bounded current root', \@errors);
        if (defined $current_raw) {
            my $current_metrics = metrics($current_raw);
            $result{current_metrics} = $current_metrics;
            push @{$result{warnings}}, @{pressure_warnings($current_metrics, $contract->{current_root}, 'bounded current root')};
            enforce_limits(
                $current_metrics,
                $contract->{current_root}{enforcement_ceilings},
                'bounded current root',
                \@errors,
            );
            validate_required_presence(
                $current_raw,
                $contract->{current_root}{required_literals},
                'bounded current root',
                \@errors,
            );
            validate_forbidden_absence(
                $current_raw,
                $contract->{current_root}{forbidden_literals},
                'bounded current root',
                \@errors,
            );
            my $metadata = section_body($current_raw, 'Metadata', 'bounded current root', \@errors);
            push @errors, 'bounded current root metadata status is not done'
                if $metadata !~ /^- Status: `done`\s*$/m;
            my $frontier = section_body($current_raw, 'Current Frontier', 'bounded current root', \@errors);
            push @errors, 'bounded current root does not declare an empty frontier'
                if index($frontier, 'No active frontier.') < 0;
            my $verification = section_body($current_raw, 'Verification Log', 'bounded current root', \@errors);
            push @errors, 'bounded current root lacks the verification capture-boundary header'
                if index($verification, '| Date | Leaf | Checks | Result |') < 0;
            my $root_links = resolved_link_counts($current_raw, $current, 'bounded current root', \@errors);
            my $index_path = $contract->{archive}{index};
            push @errors, "bounded current root must link archive index exactly once (found "
                . ($root_links->{$index_path} // 0) . ')'
                if ($root_links->{$index_path} // 0) != 1;
        }

        my $index_path = $contract->{archive}{index};
        my $index_raw = read_regular($base, $index_path, 'archive index', \@errors);
        if (defined $index_raw) {
            my $index_metrics = metrics($index_raw);
            $result{index_metrics} = $index_metrics;
            push @{$result{warnings}}, @{pressure_warnings($index_metrics, $contract->{archive_index}, 'archive index')};
            enforce_limits(
                $index_metrics,
                $contract->{archive_index}{enforcement_ceilings},
                'archive index',
                \@errors,
            );
            validate_required_presence(
                $index_raw,
                $contract->{archive_index}{required_literals},
                'archive index',
                \@errors,
            );
            validate_forbidden_absence(
                $index_raw,
                $contract->{archive_index}{forbidden_literals},
                'archive index',
                \@errors,
            );
            my $links = resolved_link_counts($index_raw, $index_path, 'archive index', \@errors);
            my @expected = ($current, $contract->{archive}{source_capsule}, $contract->{archive}{manifest});
            for my $path (@expected) {
                push @errors, "archive index must link '$path' exactly once (found "
                    . ($links->{$path} // 0) . ')'
                    if ($links->{$path} // 0) != 1;
            }
            # The explicit loop avoids accidental acceptance of unrelated local destinations.
            for my $path (keys %$links) {
                next if grep { $path eq $_ } @expected;
                push @errors, "archive index contains unexpected local link '$path'";
            }
        }
        my (undef, $manifest_metrics) = validate_migrated_manifest($base, $contract, \@errors);
        $result{manifest_metrics} = $manifest_metrics if defined $manifest_metrics;
    }

    return (\@errors, \%result);
}

sub write_raw {
    my ($base, $relative, $raw) = @_;
    my $path = absolute($base, $relative);
    make_path(dirname($path));
    open my $fh, '>:raw', $path or die "task-tree-archive self-test: cannot write $relative: $!\n";
    print {$fh} $raw;
    close $fh or die "task-tree-archive self-test: cannot close $relative: $!\n";
}

sub fixture_contract {
    my ($state, $source_raw) = @_;
    my $source_metrics = metrics($source_raw);
    return {
        schema_version => 1,
        migration_state => $state,
        current_path => 'docs/tasks/PROGRAM.md',
        archive => {
            index => 'docs/archive/tasks/program/INDEX.md',
            manifest => 'docs/archive/tasks/program/manifest.json',
            source_capsule => 'docs/archive/tasks/program/source-through-2026-08-09.md',
        },
        source => {sha256 => sha256_hex($source_raw), metrics => $source_metrics},
        source_requirements => ['# PROGRAM: fixture', '## Verification Log', '| Date | Leaf | Checks | Result |'],
        current_root => {
            health_targets => {lines => 64, bytes => 4096, line_bytes => 256},
            enforcement_ceilings => {lines => 96, bytes => 6144, line_bytes => 512},
            milestones => {warning_pct => 80, rollover_pct => 90},
            required_literals => ['# PROGRAM: fixture', '## Verification Log', '| Date | Leaf | Checks | Result |', 'No active frontier.'],
            forbidden_literals => ['Status: `in_progress`', 'Status: `pending`'],
        },
        archive_index => {
            health_targets => {lines => 32, bytes => 2048, line_bytes => 256},
            enforcement_ceilings => {lines => 48, bytes => 4096, line_bytes => 512},
            milestones => {warning_pct => 80, rollover_pct => 90},
            required_literals => ['# PROGRAM archive', '## Verification'],
            forbidden_literals => ['Status: `in_progress`'],
        },
        manifest_limits => {bytes => 8192, line_bytes => 512, scalar_bytes => 512},
        verifier => 'perl scripts/check_task_tree_archive.pl --check',
    };
}

sub source_fixture {
    return <<'SOURCE';
# PROGRAM: fixture

## Metadata

- Status: `in_progress`

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
SOURCE
}

sub current_fixture {
    return <<'CURRENT';
# PROGRAM: fixture

## Metadata

- Status: `done`

## Goal and outcome

Closed.

## Current Frontier

No active frontier.

## Archive and retrieval

[Complete history](../archive/tasks/program/INDEX.md)

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| 2026-08-09 | close | all | green |
CURRENT
}

sub index_fixture {
    return <<'INDEX';
# PROGRAM archive

- [Bounded current root](../../../tasks/PROGRAM.md)
- [Exact source](source-through-2026-08-09.md)
- [Manifest](manifest.json)

## Verification

Run the declared verifier.
INDEX
}

sub manifest_fixture {
    my ($contract) = @_;
    return {
        schema_version => 1,
        current_path => $contract->{current_path},
        source_capsule => $contract->{archive}{source_capsule},
        source_sha256 => $contract->{source}{sha256},
        source_metrics => $contract->{source}{metrics},
        source_boundary_commit => ('a' x 40),
        sealed_date => '2026-08-09',
        reason => 'fixture migration',
        verifier => $contract->{verifier},
    };
}

sub seed_fixture {
    my ($base, $state, $mutator) = @_;
    my $source_raw = source_fixture();
    my $contract = fixture_contract($state, $source_raw);
    write_raw($base, $contract->{current_path}, $state eq 'migrated' ? current_fixture() : $source_raw);
    if ($state eq 'migrated') {
        write_raw($base, $contract->{archive}{source_capsule}, $source_raw);
        write_raw($base, $contract->{archive}{index}, index_fixture());
        write_raw(
            $base,
            $contract->{archive}{manifest},
            JSON::PP->new->canonical(1)->pretty(1)->encode(manifest_fixture($contract)),
        );
    }
    $mutator->($base, $contract) if defined $mutator;
    write_raw(
        $base,
        'doctrine/live_document_size/task_tree_archive.json',
        JSON::PP->new->canonical(1)->pretty(1)->encode($contract),
    );
}

sub run_self_test {
    my ($base) = @_;
    my $generated = File::Spec->catdir($base, 'generated');
    make_path($generated);
    my @cases = (
        ['source_locked positive', 'source_locked', undef, undef],
        ['source hash drift', 'source_locked', sub { $_[1]{source}{sha256} = '0' x 64 }, qr/sha256/],
        ['source metric drift', 'source_locked', sub { $_[1]{source}{metrics}{bytes}++ }, qr/source authority bytes/],
        ['source marker missing', 'source_locked', sub { $_[1]{source_requirements}[0] = 'missing marker' }, qr/lacks required literal/],
        ['premature archive', 'source_locked', sub { write_raw($_[0], $_[1]{archive}{index}, "premature\n") }, qr/premature archive index/],
        ['unsafe source path', 'source_locked', sub { $_[1]{current_path} = '../escape.md' }, qr/current_path is unsafe/],
        ['migrated positive', 'migrated', undef, undef],
        ['capsule mutation', 'migrated', sub { write_raw($_[0], $_[1]{archive}{source_capsule}, source_fixture() . "changed\n") }, qr/source authority/],
        ['active current status', 'migrated', sub { my $raw = current_fixture(); $raw =~ s/`done`/`in_progress`/; write_raw($_[0], $_[1]{current_path}, $raw) }, qr/metadata status is not done/],
        ['missing root route', 'migrated', sub { my $raw = current_fixture(); $raw =~ s!\Q[Complete history](../archive/tasks/program/INDEX.md)\E!history unavailable!; write_raw($_[0], $_[1]{current_path}, $raw) }, qr/must link archive index exactly once/],
        ['incomplete index', 'migrated', sub { my $raw = index_fixture(); $raw =~ s/^- \[Manifest\].*\n//m; write_raw($_[0], $_[1]{archive}{index}, $raw) }, qr/must link 'docs\/archive\/tasks\/program\/manifest.json' exactly once/],
        ['manifest identity drift', 'migrated', sub { my $manifest = manifest_fixture($_[1]); $manifest->{source_sha256} = '0' x 64; write_raw($_[0], $_[1]{archive}{manifest}, JSON::PP->new->canonical(1)->pretty(1)->encode($manifest)) }, qr/source_sha256 disagrees/],
        ['invalid provenance', 'migrated', sub { my $manifest = manifest_fixture($_[1]); $manifest->{source_boundary_commit} = 'short'; write_raw($_[0], $_[1]{archive}{manifest}, JSON::PP->new->canonical(1)->pretty(1)->encode($manifest)) }, qr/source_boundary_commit/],
        ['current overflow', 'migrated', sub { $_[1]{current_root}{enforcement_ceilings}{bytes} = 10 }, qr/bounded current root exceeds bytes ceiling/],
        ['index overflow', 'migrated', sub { $_[1]{archive_index}{enforcement_ceilings}{lines} = 1 }, qr/archive index exceeds lines ceiling/],
    );

    my $passed = 0;
    for my $index (0 .. $#cases) {
        my ($name, $state, $mutator, $expected) = @{$cases[$index]};
        my $fixture = File::Spec->catdir($generated, ".task-tree-archive-self-test.$$.$index");
        remove_tree($fixture) if -e $fixture;
        make_path($fixture);
        seed_fixture($fixture, $state, $mutator);
        my ($errors) = validate_tree($fixture, 'doctrine/live_document_size/task_tree_archive.json');
        my $joined = join "\n", @$errors;
        if (!defined $expected) {
            die "task-tree-archive self-test '$name' unexpectedly failed:\n$joined\n" if @$errors;
        } else {
            die "task-tree-archive self-test '$name' unexpectedly passed\n" if !@$errors;
            die "task-tree-archive self-test '$name' missed expected diagnostic $expected:\n$joined\n"
                if $joined !~ $expected;
        }
        remove_tree($fixture);
        $passed++;
    }
    print "task-tree-archive self-test: $passed/$passed source/route/identity/boundary cases pass.\n";
}
