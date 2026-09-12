#!/usr/bin/env perl
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(encode_utf8);
use Fcntl qw(O_CREAT O_EXCL O_WRONLY);
use File::Basename qw(dirname);
use File::Path qw(make_path remove_tree);
use File::Spec;
use IO::Select;
use IPC::Open3;
use JSON::PP;
use Symbol qw(gensym);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

# Nested exact evidence may be as wide as the repository's direct task-evidence
# surface, while each task-specific contract remains free to set a tighter cap.
my $TASK_EVIDENCE_LINE_BYTES_CAP = 6_400;
# Heading, generated-artifact warning, landing pointer, and table header of a rendered route
# catalog part; a structurally full part is this many lines plus its declared routes per part.
my $ROUTE_PART_FIXED_LINES = 10;
my $DEFAULT_CONTRACT_REL = 'doctrine/live_document_size/active_task_evidence.json';
my $root;
my $contract_rel = $DEFAULT_CONTRACT_REL;
my $report = 0;
my $self_test = 0;
my $write = 0;
my $migrate_template_rel;

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
    } elsif ($arg eq '--write') {
        $write = 1;
    } elsif ($arg eq '--migrate') {
        $migrate_template_rel = shift @ARGV // usage();
    } else {
        usage();
    }
}

my $project_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'))
    // die "active-task-evidence: cannot resolve repository root\n";
usage() if defined($migrate_template_rel) && ($self_test || $report || $write);
usage() if $write && ($self_test || $report);
if ($self_test) {
    run_self_test($project_root);
    exit 0;
}

$root //= $project_root;
$root = abs_path($root) // die "active-task-evidence: root does not exist\n";
if (defined $migrate_template_rel) {
    materialize_migration($root, $contract_rel, $migrate_template_rel, 0);
    exit 0;
}
if ($write) {
    materialize_derived($root, $contract_rel, 0);
    exit 0;
}
my ($errors, $result) = validate_tree($root, $contract_rel);
if (@$errors) {
    print STDERR "active-task-evidence: $_\n" for @$errors;
    print STDERR "active-task-evidence: FAILED with ", scalar(@$errors), " violation(s).\n";
    exit 1;
}
print STDERR "active-task-evidence: warning: $_\n" for @{$result->{warnings} // []};
if ($report) {
    print JSON::PP->new->canonical(1)->encode($result), "\n";
} else {
    print "active-task-evidence: $result->{migration_state}/$result->{input_state} contract is valid.\n";
}
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--contract PATH] [--check|--report|--write|--self-test|--migrate ROOT_TEMPLATE]\n";
}

sub verifier_command_for {
    my ($relative_contract) = @_;
    return 'perl scripts/check_active_task_evidence.pl --check'
        if ($relative_contract // '') eq $DEFAULT_CONTRACT_REL;
    return "perl scripts/check_active_task_evidence.pl --contract $relative_contract --check";
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

sub occurrences {
    my ($raw, $literal) = @_;
    $literal = raw_scalar($literal);
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

sub task_token_occurrences {
    my ($raw, $literal) = @_;
    $literal = raw_scalar($literal);
    return 0 if $literal eq '';
    my ($count, $offset) = (0, 0);
    while (1) {
        my $position = index($raw, $literal, $offset);
        last if $position < 0;
        my $before = $position == 0 ? '' : substr($raw, $position - 1, 1);
        my $after_at = $position + length($literal);
        my $after = $after_at >= length($raw) ? '' : substr($raw, $after_at, 1);
        $count++ if $before !~ /[A-Za-z0-9.]/ && $after !~ /[A-Za-z0-9.]/;
        $offset = $position + length($literal);
    }
    return $count;
}

sub metrics {
    my ($raw) = @_;
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

sub split_source_lines {
    my ($raw) = @_;
    my @lines;
    while ($raw =~ /\G([^\n]*(?:\n|\z))/gc) {
        my $line = $1;
        last if $line eq '';
        push @lines, $line;
    }
    return \@lines;
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
    my $resolved = abs_path($path);
    push @$errors, "$label resolves outside the repository: $relative"
        if !defined($resolved) || ($resolved ne $base && index($resolved, "$base/") != 0);
    my $root_device = (stat($base))[0];
    my $file_device = (stat($path))[0];
    push @$errors, "$label is off the repository volume: $relative"
        if !defined($root_device) || !defined($file_device) || $root_device != $file_device;
    open my $fh, '<:raw', $path or do {
        push @$errors, "cannot read $label '$relative': $!";
        return;
    };
    local $/;
    my $raw = <$fh> // '';
    close $fh or push @$errors, "cannot close $label '$relative': $!";
    return $raw;
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
    my $value = ref($object) eq 'HASH' ? $object->{$field} : undef;
    if (!defined($value) || ref($value) || $value eq '') {
        push @$errors, "$label lacks non-empty scalar '$field'";
        return;
    }
    return $value;
}

sub positive_integer {
    my ($object, $field, $label, $errors) = @_;
    my $value = ref($object) eq 'HASH' ? $object->{$field} : undef;
    if (!defined($value) || ref($value) || $value !~ /\A\d+\z/ || $value < 1) {
        push @$errors, "$label lacks positive integer '$field'";
        return;
    }
    return 0 + $value;
}

sub scalar_array {
    my ($value, $label, $errors, $allow_empty) = @_;
    if (ref($value) ne 'ARRAY' || (!$allow_empty && !@$value)) {
        push @$errors, "$label must be " . ($allow_empty ? 'an array' : 'a non-empty array');
        return [];
    }
    my %seen;
    for my $item (@$value) {
        if (!defined($item) || ref($item) || $item eq '') {
            push @$errors, "$label contains an empty or non-scalar item";
            next;
        }
        push @$errors, "$label contains a duplicate '$item'" if $seen{$item}++;
    }
    return $value;
}

sub metric_object {
    my ($object, $label, $errors, @fields) = @_;
    reject_unknown($object, $label, $errors, @fields);
    my %values;
    $values{$_} = positive_integer($object, $_, $label, $errors) for @fields;
    return \%values;
}

sub compare_metrics {
    my ($actual, $expected, $label, $errors, @fields) = @_;
    for my $field (@fields) {
        next if !defined($expected->{$field});
        push @$errors, "$label $field is $actual->{$field}, expected $expected->{$field}"
            if $actual->{$field} != $expected->{$field};
    }
}

sub validate_limit_pair {
    my ($spec, $label, $errors, $fields, $extra) = @_;
    reject_unknown($spec, $label, $errors, qw(health_targets enforcement_ceilings milestones), @{$extra // []});
    my $health = metric_object($spec->{health_targets}, "$label health_targets", $errors, @$fields);
    my $ceilings = metric_object($spec->{enforcement_ceilings}, "$label enforcement_ceilings", $errors, @$fields);
    for my $field (@$fields) {
        next if !defined($health->{$field}) || !defined($ceilings->{$field});
        push @$errors, "$label health $field exceeds its enforcement ceiling"
            if $health->{$field} > $ceilings->{$field};
    }
    my $milestones = $spec->{milestones};
    reject_unknown($milestones, "$label milestones", $errors, qw(warning_pct rollover_pct));
    my $warning = positive_integer($milestones, 'warning_pct', "$label milestones", $errors);
    my $rollover = positive_integer($milestones, 'rollover_pct', "$label milestones", $errors);
    push @$errors, "$label milestones must satisfy warning_pct < rollover_pct < 100"
        if defined($warning) && defined($rollover) && ($warning >= $rollover || $rollover >= 100);
}

sub enforce_ceilings {
    my ($actual, $ceilings, $label, $errors, $mapping) = @_;
    for my $actual_field (sort keys %$mapping) {
        my $ceiling_field = $mapping->{$actual_field};
        next if !defined $ceilings->{$ceiling_field};
        push @$errors, "$label exceeds $ceiling_field ceiling $ceilings->{$ceiling_field}"
            if $actual->{$actual_field} > $ceilings->{$ceiling_field};
    }
}

sub warning_messages {
    my ($actual, $spec, $label, $mapping) = @_;
    my @warnings;
    my $targets = $spec->{health_targets};
    my $milestones = $spec->{milestones};
    return \@warnings if ref($targets) ne 'HASH' || ref($milestones) ne 'HASH';
    for my $actual_field (sort keys %$mapping) {
        my $target_field = $mapping->{$actual_field};
        next if !$targets->{$target_field};
        my $percent = 100 * $actual->{$actual_field} / $targets->{$target_field};
        if ($percent >= ($milestones->{rollover_pct} // 101)) {
            push @warnings, sprintf('%s %s is at or above rollover (%.1f%%)', $label, $target_field, $percent);
        } elsif ($percent >= ($milestones->{warning_pct} // 101)) {
            push @warnings, sprintf('%s %s is at or above warning (%.1f%%)', $label, $target_field, $percent);
        }
    }
    return \@warnings;
}

sub apply_live_pressure {
    my ($actual, $spec, $label, $mapping, $errors, $warnings) = @_;
    my $targets = $spec->{health_targets};
    my $milestones = $spec->{milestones};
    return if ref($targets) ne 'HASH' || ref($milestones) ne 'HASH';
    for my $actual_field (sort keys %$mapping) {
        my $target_field = $mapping->{$actual_field};
        next if !$targets->{$target_field};
        my $percent = 100 * $actual->{$actual_field} / $targets->{$target_field};
        if ($percent >= ($milestones->{rollover_pct} // 101)) {
            push @$errors, sprintf('%s %s is at or above mandatory rollover (%.1f%%)', $label, $target_field, $percent);
        } elsif ($percent >= ($milestones->{warning_pct} // 101)) {
            push @$warnings, sprintf('%s %s is at or above warning (%.1f%%)', $label, $target_field, $percent);
        }
    }
}

sub enforce_portable_caps {
    my ($values, $caps, $label, $errors) = @_;
    for my $field (sort keys %$caps) {
        next if !defined($values->{$field}) || ref($values->{$field});
        push @$errors, "$label $field exceeds portable cap $caps->{$field}"
            if $values->{$field} > $caps->{$field};
    }
}

sub validate_required_literals {
    my ($raw, $literals, $label, $errors) = @_;
    for my $literal (@$literals) {
        next if !defined($literal) || ref($literal) || $literal eq '';
        push @$errors, "$label lacks required literal '$literal'" if occurrences($raw, $literal) == 0;
    }
}

sub validate_forbidden_literals {
    my ($raw, $literals, $label, $errors) = @_;
    for my $literal (@$literals) {
        next if !defined($literal) || ref($literal) || $literal eq '';
        push @$errors, "$label contains forbidden literal '$literal'" if occurrences($raw, $literal);
    }
}

sub command_capture {
    my (@command) = @_;
    my $stdout = gensym;
    my $stderr = gensym;
    my $pid = eval { open3(undef, $stdout, $stderr, @command) };
    return ('', 255, "$@") if !$pid;
    binmode $stdout, ':raw';
    binmode $stderr, ':raw';
    my $stdout_fileno = fileno($stdout);
    my $selector = IO::Select->new($stdout, $stderr);
    my ($out, $err) = ('', '');
    while (my @ready = $selector->can_read) {
        for my $fh (@ready) {
            my $chunk = '';
            my $read = sysread($fh, $chunk, 8192);
            if (!defined($read) || $read == 0) {
                $selector->remove($fh);
                close $fh;
                next;
            }
            if (fileno($fh) == $stdout_fileno) {
                $out .= $chunk;
            } else {
                $err .= $chunk;
            }
        }
    }
    waitpid($pid, 0);
    return ($out, $? == -1 ? 255 : $? >> 8, $err);
}

sub git_capture {
    my ($base, $label, $errors, @args) = @_;
    my ($raw, $exit) = command_capture('git', '-C', $base, @args);
    push @$errors, "$label failed (git exit $exit)" if $exit != 0;
    return if $exit != 0;
    return $raw;
}

sub validate_git_boundary {
    my ($base, $contract, $source_raw, $errors) = @_;
    my $source = $contract->{source};
    my $commit = $source->{boundary_commit};
    my $path = $contract->{current_path};
    return if !defined($commit) || !safe_relative_path($path);

    my $resolved = git_capture($base, 'source boundary commit lookup', $errors, 'rev-parse', '--verify', "$commit^{commit}");
    if (defined $resolved) {
        $resolved =~ s/[\r\n]+\z//;
        push @$errors, "source boundary commit resolves to '$resolved', expected '$commit'" if $resolved ne $commit;
    }
    my $blob = git_capture($base, 'source boundary blob lookup', $errors, 'rev-parse', "$commit:$path");
    if (defined $blob) {
        $blob =~ s/[\r\n]+\z//;
        push @$errors, "source boundary blob is '$blob', expected '$source->{git_blob}'"
            if $blob ne ($source->{git_blob} // '');
    }
    my $committed_raw = git_capture($base, 'source boundary content read', $errors, 'show', "$commit:$path");
    push @$errors, 'source boundary Git content differs from the pinned source bytes'
        if defined($committed_raw) && defined($source_raw) && $committed_raw ne $source_raw;
    my (undef, $ancestor_exit) = command_capture('git', '-C', $base, 'merge-base', '--is-ancestor', $commit, 'HEAD');
    push @$errors, 'source boundary commit is not an ancestor of HEAD' if $ancestor_exit != 0;
}

sub boundary_route_ids {
    my ($base, $contract, $errors) = @_;
    my $source = $contract->{source};
    my $tree_id = $contract->{identity}{tree_id};
    my $path = $contract->{current_path};
    return [] if !defined($source->{boundary_commit}) || !defined($tree_id) || !safe_relative_path($path);
    my $subjects = git_capture(
        $base,
        'source boundary path history read',
        $errors,
        'log', '--format=%s', $source->{boundary_commit}, '--', $path,
    );
    return [] if !defined $subjects;
    my %ids;
    my $quoted = quotemeta($tree_id);
    while ($subjects =~ /\b($quoted(?:\.[A-Za-z0-9]+)*)\b/g) {
        $ids{$1} = 1;
    }
    return [sort keys %ids];
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

sub link_counts {
    my ($raw, $source_path, $label, $errors) = @_;
    my %counts;
    while ($raw =~ /\[[^\]\r\n]*\]\(([^)\r\n]+)\)/g) {
        my $resolved = normalize_link($source_path, $1);
        if (!defined($resolved) || !safe_relative_path($resolved)) {
            push @$errors, "$label contains unsafe local link '$1'";
            next;
        }
        $counts{$resolved}++;
    }
    return \%counts;
}

sub canonical_json {
    my ($value) = @_;
    return JSON::PP->new->canonical(1)->ascii(1)->encode($value);
}

sub deep_clone {
    my ($value) = @_;
    return decode_json(canonical_json($value));
}

sub validate_scalar_bounds {
    my ($value, $limit, $label, $errors) = @_;
    if (!ref($value)) {
        push @$errors, "$label contains a scalar above $limit bytes"
            if defined($value) && length(raw_scalar($value)) > $limit;
    } elsif (ref($value) eq 'ARRAY') {
        validate_scalar_bounds($_, $limit, $label, $errors) for @$value;
    } elsif (ref($value) eq 'HASH') {
        for my $key (keys %$value) {
            push @$errors, "$label contains a key above $limit bytes" if length(raw_scalar($key)) > $limit;
            validate_scalar_bounds($value->{$key}, $limit, $label, $errors);
        }
    }
}

sub validate_contract_schema {
    my ($contract, $relative_contract, $errors) = @_;
    reject_unknown(
        $contract,
        'contract',
        $errors,
        qw(schema_version contract_id migration_state input_state current_path identity current_frontier migration_metadata source source_requirements destinations regions leaf_routes route_basis route_catalog_state marker_prefix limits migrated_requirements verifier),
    );
    push @$errors, 'contract schema_version must be 1'
        if !defined($contract->{schema_version}) || ref($contract->{schema_version}) || $contract->{schema_version} != 1;
    required_scalar($contract, 'contract_id', 'contract', $errors);
    my $migration_state = required_scalar($contract, 'migration_state', 'contract', $errors);
    push @$errors, "contract has invalid migration_state '$migration_state'"
        if defined($migration_state) && $migration_state ne 'source_locked' && $migration_state ne 'migrated';
    my $input_state = required_scalar($contract, 'input_state', 'contract', $errors);
    push @$errors, "contract has invalid input_state '$input_state'"
        if defined($input_state) && $input_state ne 'topology_declared' && $input_state ne 'complete';
    push @$errors, 'migrated contract requires complete migration inputs'
        if ($migration_state // '') eq 'migrated' && ($input_state // '') ne 'complete';

    my $current = required_scalar($contract, 'current_path', 'contract', $errors);
    push @$errors, 'contract current_path is unsafe' if defined($current) && !safe_relative_path($current);
    my $verifier = required_scalar($contract, 'verifier', 'contract', $errors);
    my $expected_verifier = verifier_command_for($relative_contract);
    push @$errors, "contract verifier must be '$expected_verifier'"
        if defined($verifier) && $verifier ne $expected_verifier;
    my $route_basis = required_scalar($contract, 'route_basis', 'contract', $errors);
    push @$errors, "contract route_basis must be 'boundary_path_commit_subject_ids'"
        if defined($route_basis) && $route_basis ne 'boundary_path_commit_subject_ids';
    my $catalog_state = required_scalar($contract, 'route_catalog_state', 'contract', $errors);
    push @$errors, "contract has invalid route_catalog_state '$catalog_state'"
        if defined($catalog_state) && $catalog_state ne 'inline' && $catalog_state ne 'sharded';
    my $sharded = (($catalog_state // '') eq 'sharded') ? 1 : 0;
    my $marker_prefix = required_scalar($contract, 'marker_prefix', 'contract', $errors);
    push @$errors, 'contract marker_prefix must use lowercase letters, digits, and hyphens only'
        if defined($marker_prefix) && $marker_prefix !~ /\A[a-z0-9]+(?:-[a-z0-9]+)*\z/;

    my $identity = $contract->{identity};
    reject_unknown($identity, 'contract identity', $errors, qw(tree_id heading active_status_literal));
    required_scalar($identity, $_, 'contract identity', $errors) for qw(tree_id heading active_status_literal);
    my $frontier = $contract->{current_frontier};
    reject_unknown($frontier, 'contract current_frontier', $errors, qw(mode literal leaf_id part_id));
    my $frontier_mode = required_scalar($frontier, 'mode', 'contract current_frontier', $errors);
    push @$errors, "contract current_frontier has invalid mode '$frontier_mode'"
        if defined($frontier_mode) && $frontier_mode ne 'none' && $frontier_mode ne 'eligible';
    required_scalar($frontier, 'literal', 'contract current_frontier', $errors);
    if (($frontier_mode // '') eq 'eligible') {
        required_scalar($frontier, 'leaf_id', 'contract current_frontier', $errors);
        required_scalar($frontier, 'part_id', 'contract current_frontier', $errors);
    } else {
        push @$errors, 'contract no-frontier object must not declare leaf_id or part_id'
            if exists($frontier->{leaf_id}) || exists($frontier->{part_id});
    }

    my $migration_metadata = $contract->{migration_metadata};
    if (defined($migration_metadata) || ($migration_state // '') eq 'migrated') {
        reject_unknown($migration_metadata, 'contract migration_metadata', $errors, qw(migrated_on reason));
        my $migrated_on = required_scalar(
            $migration_metadata, 'migrated_on', 'contract migration_metadata', $errors,
        );
        push @$errors, 'contract migration_metadata migrated_on is not an ISO date'
            if defined($migrated_on) && $migrated_on !~ /\A\d{4}-\d{2}-\d{2}\z/;
        required_scalar($migration_metadata, 'reason', 'contract migration_metadata', $errors);
    }

    my $source = $contract->{source};
    reject_unknown($source, 'contract source', $errors, qw(boundary_commit git_blob sha256 metrics));
    for my $field (qw(boundary_commit git_blob sha256)) {
        my $value = required_scalar($source, $field, 'contract source', $errors);
        my $pattern = $field eq 'sha256' ? qr/\A[0-9a-f]{64}\z/ : qr/\A(?:[0-9a-f]{40}|[0-9a-f]{64})\z/;
        push @$errors, "contract source $field has invalid object identity" if defined($value) && $value !~ $pattern;
    }
    metric_object($source->{metrics}, 'contract source metrics', $errors, qw(lines bytes line_bytes));
    scalar_array($contract->{source_requirements}, 'contract source_requirements', $errors, 0);

    my $destinations = $contract->{destinations};
    reject_unknown(
        $destinations,
        'contract destinations',
        $errors,
        qw(collection_directory index manifest parts archive_directory source_capsule route_part_prefix),
    );
    my %paths;
    $paths{$current} = 1 if defined $current;
    for my $field (qw(collection_directory index manifest archive_directory source_capsule)) {
        my $path = required_scalar($destinations, $field, 'contract destinations', $errors);
        push @$errors, "contract destination $field path is unsafe"
            if defined($path) && !safe_relative_path($path);
        push @$errors, "contract path '$path' is declared more than once" if defined($path) && $paths{$path}++;
    }
    my $collection_directory = $destinations->{collection_directory} // '';
    my $archive_directory = $destinations->{archive_directory} // '';
    for my $field (qw(index manifest)) {
        my $path = $destinations->{$field} // '';
        push @$errors, "contract destination $field is not below collection_directory"
            if $collection_directory ne '' && index($path, "$collection_directory/") != 0;
    }
    push @$errors, 'contract source_capsule is not below archive_directory'
        if $archive_directory ne '' && index(($destinations->{source_capsule} // ''), "$archive_directory/") != 0;
    my $route_prefix;
    if (!$sharded) {
        push @$errors, 'inline route catalog must not declare a route_part_prefix'
            if exists $destinations->{route_part_prefix};
    } else {
        $route_prefix = required_scalar($destinations, 'route_part_prefix', 'contract destinations', $errors);
    }
    if (defined $route_prefix) {
        push @$errors, 'contract destination route_part_prefix is unsafe'
            if !safe_relative_path("${route_prefix}0001.md");
        push @$errors, 'contract destination route_part_prefix is not below collection_directory'
            if $collection_directory ne '' && index($route_prefix, "$collection_directory/") != 0;
        push @$errors, 'contract destination route_part_prefix must end with a hyphenated stem'
            if $route_prefix !~ m{/[a-z0-9]+(?:-[a-z0-9]+)*-\z};
    }

    my $parts = $destinations->{parts};
    if (ref($parts) ne 'ARRAY' || !@$parts) {
        push @$errors, 'contract destinations parts must be a non-empty array';
        $parts = [];
    }
    my (%part_ids, %part_paths, %part_states);
    for my $part (@$parts) {
        reject_unknown($part, 'contract part', $errors, qw(part_id path heading state sha256 metrics sealed_commit git_blob));
        my $part_id = required_scalar($part, 'part_id', 'contract part', $errors);
        my $path = required_scalar($part, 'path', 'contract part', $errors);
        required_scalar($part, 'heading', 'contract part', $errors);
        my $state = required_scalar($part, 'state', 'contract part', $errors);
        push @$errors, "contract part has invalid state '$state'"
            if defined($state) && $state ne 'legacy' && $state ne 'active' && $state ne 'sealed';
        push @$errors, "contract duplicate part_id '$part_id'" if defined($part_id) && $part_ids{$part_id}++;
        $part_states{$part_id} = $state if defined($part_id) && defined($state);
        push @$errors, "contract duplicate part path '$path'" if defined($path) && $part_paths{$path}++;
        push @$errors, "contract part path '$path' is unsafe or outside collection_directory"
            if defined($path) && (!safe_relative_path($path) || index($path, "$collection_directory/") != 0);
        push @$errors, "contract path '$path' is declared more than once" if defined($path) && $paths{$path}++;
        if (($migration_state // '') eq 'migrated') {
            my $sha = required_scalar($part, 'sha256', 'migrated contract part', $errors);
            push @$errors, "migrated contract part sha256 is invalid" if defined($sha) && $sha !~ /\A[0-9a-f]{64}\z/;
            metric_object($part->{metrics}, 'migrated contract part metrics', $errors, qw(lines bytes line_bytes));
        } else {
            push @$errors, 'source-locked part must not pin result sha256 or metrics'
                if exists($part->{sha256}) || exists($part->{metrics});
        }
        if (($state // '') eq 'sealed') {
            my $sealed_commit = required_scalar($part, 'sealed_commit', 'sealed contract part', $errors);
            my $git_blob = required_scalar($part, 'git_blob', 'sealed contract part', $errors);
            push @$errors, 'sealed contract part has invalid sealed_commit'
                if defined($sealed_commit) && $sealed_commit !~ /\A(?:[0-9a-f]{40}|[0-9a-f]{64})\z/;
            push @$errors, 'sealed contract part has invalid git_blob'
                if defined($git_blob) && $git_blob !~ /\A(?:[0-9a-f]{40}|[0-9a-f]{64})\z/;
        } elsif (exists($part->{sealed_commit}) || exists($part->{git_blob})) {
            push @$errors, 'non-sealed contract part must not declare sealed_commit or git_blob';
        }
    }

    my $limits = $contract->{limits};
    reject_unknown($limits, 'contract limits', $errors, qw(root index parts route_parts capsule_exact manifest));
    validate_limit_pair($limits->{root}, 'contract root limits', $errors, [qw(lines bytes line_bytes)]);
    validate_limit_pair($limits->{index}, 'contract index limits', $errors, [qw(lines bytes line_bytes)]);
    validate_limit_pair(
        $limits->{parts},
        'contract parts limits',
        $errors,
        [qw(files lines_each bytes_each line_bytes_each lines_total bytes_total)],
    );
    enforce_portable_caps(
        $limits->{root}{enforcement_ceilings},
        {lines => 384, bytes => 36_864, line_bytes => 1_024},
        'contract root enforcement ceiling',
        $errors,
    );
    enforce_portable_caps(
        $limits->{index}{enforcement_ceilings},
        {lines => 224, bytes => 24_576, line_bytes => 768},
        'contract index enforcement ceiling',
        $errors,
    );
    enforce_portable_caps(
        $limits->{parts}{enforcement_ceilings},
        {
            files => 24,
            lines_each => 896,
            bytes_each => 98_304,
            line_bytes_each => $TASK_EVIDENCE_LINE_BYTES_CAP,
            lines_total => 9_600,
            bytes_total => 1_179_648,
        },
        'contract parts enforcement ceiling',
        $errors,
    );
    my ($routes_per_part, $max_route_parts);
    if (!$sharded) {
        push @$errors, 'inline route catalog must not declare route_parts limits'
            if exists $limits->{route_parts};
    } else {
        validate_limit_pair(
            $limits->{route_parts},
            'contract route_parts limits',
            $errors,
            [qw(files lines_each bytes_each line_bytes_each lines_total bytes_total)],
            [qw(routes_per_part max_parts max_unverified_routes)],
        );
        $routes_per_part = positive_integer(
            $limits->{route_parts}, 'routes_per_part', 'contract route_parts limits', $errors,
        );
        $max_route_parts = positive_integer(
            $limits->{route_parts}, 'max_parts', 'contract route_parts limits', $errors,
        );
        my $max_unverified = $limits->{route_parts}{max_unverified_routes};
        push @$errors, 'contract route_parts limits lacks a non-negative max_unverified_routes'
            if !defined($max_unverified) || ref($max_unverified) || $max_unverified !~ /\A\d+\z/;
        enforce_portable_caps(
            $limits->{route_parts}{enforcement_ceilings},
            {
                files => 16,
                lines_each => 256,
                bytes_each => 32_768,
                line_bytes_each => 768,
                lines_total => 4_096,
                bytes_total => 524_288,
            },
            'contract route_parts enforcement ceiling',
            $errors,
        );
        enforce_portable_caps(
            {routes_per_part => $routes_per_part, max_parts => $max_route_parts},
            {routes_per_part => 128, max_parts => 16},
            'contract route_parts limit',
            $errors,
        );
        # A route catalog part must stay below its own warning band when structurally full, so a full part is
        # never a bound the writer can reach with no compliant remedy (LIVE-DOCUMENT-PRESSURE-HEADROOM.2c).
        if (defined($routes_per_part)
            && ref($limits->{route_parts}{health_targets}) eq 'HASH'
            && ref($limits->{route_parts}{milestones}) eq 'HASH') {
            my $target = $limits->{route_parts}{health_targets}{lines_each};
            my $warning = $limits->{route_parts}{milestones}{warning_pct};
            my $full = $routes_per_part + $ROUTE_PART_FIXED_LINES;
            push @$errors, "contract route_parts health lines_each admits no full part below its warning band"
                if defined($target) && defined($warning) && $target > 0
                && 100 * $full / $target >= $warning;
        }
        if (defined($max_route_parts) && defined($routes_per_part)) {
            push @$errors, 'contract route catalog capacity is below the declared leaf route count'
                if ref($contract->{leaf_routes}) eq 'ARRAY'
                && scalar(@{$contract->{leaf_routes}}) > $max_route_parts * $routes_per_part;
        }
    }
    my $capsule_exact = metric_object(
        $limits->{capsule_exact},
        'contract capsule_exact',
        $errors,
        qw(lines bytes line_bytes),
    );
    compare_metrics($capsule_exact, $source->{metrics}, 'contract capsule/source metric agreement', $errors, qw(lines bytes line_bytes))
        if ref($source->{metrics}) eq 'HASH';
    my $manifest_limits = $limits->{manifest};
    reject_unknown($manifest_limits, 'contract manifest limits', $errors, qw(bytes line_bytes scalar_bytes max_parts max_regions max_leaf_routes));
    positive_integer($manifest_limits, $_, 'contract manifest limits', $errors)
        for qw(bytes line_bytes scalar_bytes max_parts max_regions max_leaf_routes);
    enforce_portable_caps(
        $manifest_limits,
        {bytes => 65_536, line_bytes => 1_024, scalar_bytes => 1_024, max_parts => 24, max_regions => 32, max_leaf_routes => 128},
        'contract manifest limit',
        $errors,
    );
    push @$errors, 'contract part count exceeds max_parts'
        if @$parts > ($manifest_limits->{max_parts} // 0);

    my $regions = $contract->{regions};
    if (ref($regions) ne 'ARRAY' || !@$regions) {
        push @$errors, 'contract regions must be a non-empty array';
        $regions = [];
    }
    push @$errors, 'contract region count exceeds max_regions'
        if @$regions > ($manifest_limits->{max_regions} // 0);
    my (%region_ids, %part_region_count);
    my $expected_start = 1;
    for my $region (@$regions) {
        reject_unknown($region, 'contract region', $errors, qw(region_id start_line end_line part_id sha256 metrics));
        my $region_id = required_scalar($region, 'region_id', 'contract region', $errors);
        my $start = positive_integer($region, 'start_line', 'contract region', $errors);
        my $end = positive_integer($region, 'end_line', 'contract region', $errors);
        my $part_id = required_scalar($region, 'part_id', 'contract region', $errors);
        push @$errors, "contract duplicate region_id '$region_id'" if defined($region_id) && $region_ids{$region_id}++;
        push @$errors, "contract region '$region_id' references unknown part '$part_id'"
            if defined($part_id) && !$part_ids{$part_id};
        $part_region_count{$part_id}++ if defined($part_id);
        if (defined($start) && $start != $expected_start) {
            push @$errors, "contract regions are not contiguous at line $expected_start";
        }
        if (defined($start) && defined($end)) {
            push @$errors, "contract region '$region_id' ends before it starts" if $end < $start;
            $expected_start = $end + 1 if $end >= $start;
        }
        if (($input_state // '') eq 'complete') {
            my $sha = required_scalar($region, 'sha256', 'complete contract region', $errors);
            push @$errors, "complete contract region sha256 is invalid"
                if defined($sha) && $sha !~ /\A[0-9a-f]{64}\z/;
            my $region_metrics = metric_object($region->{metrics}, 'complete contract region metrics', $errors, qw(lines bytes line_bytes));
            push @$errors, "complete contract region '$region_id' line metric disagrees with span"
                if defined($start) && defined($end) && defined($region_metrics->{lines})
                && $region_metrics->{lines} != $end - $start + 1;
        } else {
            push @$errors, 'topology-declared region must not pin sha256 or metrics'
                if exists($region->{sha256}) || exists($region->{metrics});
        }
    }
    my $source_lines = ref($source->{metrics}) eq 'HASH' ? $source->{metrics}{lines} : undef;
    push @$errors, 'contract regions do not cover the complete source line range'
        if defined($source_lines) && $expected_start != $source_lines + 1;
    for my $part (@$parts) {
        next if ($part->{state} // '') ne 'legacy';
        push @$errors, "legacy part '$part->{part_id}' owns no source region"
            if !$part_region_count{$part->{part_id}};
    }

    my $routes = $contract->{leaf_routes};
    if (ref($routes) ne 'ARRAY') {
        push @$errors, 'contract leaf_routes must be an array';
        $routes = [];
    }
    push @$errors, 'contract leaf route count exceeds max_leaf_routes'
        if @$routes > ($manifest_limits->{max_leaf_routes} // 0);
    push @$errors, 'topology-declared contract must not contain leaf routes'
        if ($input_state // '') eq 'topology_declared' && @$routes;
    push @$errors, 'complete contract must contain leaf routes'
        if ($input_state // '') eq 'complete' && !@$routes;
    my (%route_ids, %route_parts);
    for my $route (@$routes) {
        reject_unknown($route, 'contract leaf route', $errors, qw(leaf_id part_id origin source_literal lifecycle));
        my $leaf_id = required_scalar($route, 'leaf_id', 'contract leaf route', $errors);
        my $part_id = required_scalar($route, 'part_id', 'contract leaf route', $errors);
        my $origin = required_scalar($route, 'origin', 'contract leaf route', $errors);
        push @$errors, "contract duplicate leaf route '$leaf_id'" if defined($leaf_id) && $route_ids{$leaf_id}++;
        $route_parts{$leaf_id} = $part_id if defined($leaf_id) && defined($part_id);
        push @$errors, "contract leaf route '$leaf_id' references unknown part '$part_id'"
            if defined($part_id) && !$part_ids{$part_id};
        push @$errors, "contract leaf route '$leaf_id' has invalid origin '$origin'"
            if defined($origin) && $origin ne 'legacy' && $origin ne 'structural' && $origin ne 'post_migration';
        if (!$sharded) {
            push @$errors, "inline contract leaf route '$leaf_id' must not declare a lifecycle"
                if exists $route->{lifecycle};
        } else {
            my $lifecycle = required_scalar($route, 'lifecycle', 'contract leaf route', $errors);
            push @$errors, "contract leaf route '$leaf_id' has invalid lifecycle '$lifecycle'"
                if defined($lifecycle) && $lifecycle ne 'open' && $lifecycle ne 'closed';
        }
        push @$errors, "source-locked contract cannot declare post-migration leaf route '$leaf_id'"
            if ($migration_state // '') eq 'source_locked' && ($origin // '') eq 'post_migration';
        if (($origin // '') eq 'legacy' || ($origin // '') eq 'structural') {
            my $literal = required_scalar($route, 'source_literal', "$origin contract leaf route", $errors);
            if (defined($literal) && defined($leaf_id)) {
                my $relative = $leaf_id;
                $relative =~ s/\A\Q$identity->{tree_id}\E//;
                push @$errors, "$origin leaf route '$leaf_id' source_literal is not its full or tree-relative id"
                    if $literal ne $leaf_id && ($relative eq '' || $literal ne $relative);
            }
        } elsif (exists($route->{source_literal})) {
            push @$errors, "post-migration leaf route '$leaf_id' must not declare source_literal";
        }
    }
    if (($frontier_mode // '') eq 'eligible') {
        push @$errors, 'eligible frontier leaf_id lacks a primary leaf route'
            if !$route_ids{$frontier->{leaf_id} // ''};
        push @$errors, 'eligible frontier part_id disagrees with its primary leaf route'
            if ($route_ids{$frontier->{leaf_id} // ''}
                && ($frontier->{part_id} // '') ne ($route_parts{$frontier->{leaf_id}} // ''));
        push @$errors, 'eligible frontier must route to an active semantic part'
            if ($part_states{$frontier->{part_id} // ''} // '') ne 'active';
    }

    my $migrated = $contract->{migrated_requirements};
    reject_unknown($migrated, 'contract migrated_requirements', $errors, qw(root_required_literals root_forbidden_literals index_required_literals));
    scalar_array($migrated->{root_required_literals}, 'contract root_required_literals', $errors, 0);
    scalar_array($migrated->{root_forbidden_literals}, 'contract root_forbidden_literals', $errors, 1);
    scalar_array($migrated->{index_required_literals}, 'contract index_required_literals', $errors, 0);
    my %root_owner_ids;
    for my $literal (@{$migrated->{root_required_literals} // []}) {
        next if ref($literal) || $literal !~ /\A- ID: `([^`]+)`\z/;
        my $owner_id = $1;
        push @$errors, "contract root owner '$owner_id' is outside tree '$identity->{tree_id}'"
            if $owner_id ne ($identity->{tree_id} // '')
            && index($owner_id, ($identity->{tree_id} // '') . '.') != 0;
        $root_owner_ids{$owner_id} = 1;
    }
    if (($input_state // '') eq 'complete' && keys %root_owner_ids) {
        push @$errors, "complete leaf routes omit declared root owner '$_'"
            for grep { !$route_ids{$_} } sort keys %root_owner_ids;
        push @$errors, "complete leaf route '$_' is absent from the declared root owner registry"
            for grep { !$root_owner_ids{$_} } sort keys %route_ids;
    }
}

sub validate_source_and_inputs {
    my ($base, $contract, $source_path, $locked, $errors, $result) = @_;
    my $source_raw = read_regular($base, $source_path, 'source authority', $errors);
    return if !defined $source_raw;
    my $source_metrics = metrics($source_raw);
    compare_metrics($source_metrics, $contract->{source}{metrics}, 'source authority', $errors, qw(lines bytes line_bytes));
    my $sha = sha256_hex($source_raw);
    push @$errors, "source authority sha256 is $sha, expected $contract->{source}{sha256}"
        if $sha ne ($contract->{source}{sha256} // '');
    validate_required_literals($source_raw, $contract->{source_requirements}, 'source authority', $errors);
    push @$errors, 'source authority heading does not match contract identity'
        if index($source_raw, raw_scalar($contract->{identity}{heading})) != 0;
    my $tree_literal = "- Tree ID: `$contract->{identity}{tree_id}`";
    push @$errors, 'source authority does not carry exactly one declared tree identity'
        if occurrences($source_raw, $tree_literal) != 1;
    push @$errors, 'source authority lacks the declared active-status literal'
        if occurrences($source_raw, $contract->{identity}{active_status_literal}) == 0;
    validate_git_boundary($base, $contract, $source_raw, $errors);
    if ($locked) {
        my $staged = git_capture($base, 'source Git-index lookup', $errors, 'ls-files', '--stage', '--', $contract->{current_path});
        if (defined $staged) {
            my @entries = grep { $_ ne '' } split /\n/, $staged;
            if (@entries != 1 || $entries[0] !~ /^\d+\s+([0-9a-f]{40}|[0-9a-f]{64})\s+0\t/) {
                push @$errors, 'source current path does not have exactly one stage-zero Git-index entry';
            } elsif ($1 ne ($contract->{source}{git_blob} // '')) {
                push @$errors, "source Git-index blob is '$1', expected '$contract->{source}{git_blob}'";
            }
        }
    }

    my $lines = split_source_lines($source_raw);
    my (%region_raw, %part_payload);
    for my $region (@{$contract->{regions}}) {
        next if ref($region) ne 'HASH';
        my ($start, $end) = @{$region}{qw(start_line end_line)};
        next if !defined($start) || !defined($end) || $start < 1 || $end > @$lines || $end < $start;
        my $raw = join '', @$lines[$start - 1 .. $end - 1];
        $region_raw{$region->{region_id}} = $raw;
        $part_payload{$region->{part_id}} .= $raw;
        if (($contract->{input_state} // '') eq 'complete') {
            my $actual = metrics($raw);
            compare_metrics($actual, $region->{metrics}, "source region '$region->{region_id}'", $errors, qw(lines bytes line_bytes));
            push @$errors, "source region '$region->{region_id}' sha256 mismatch"
                if sha256_hex($raw) ne ($region->{sha256} // '');
        }
    }
    my %planned;
    my $part_limits = $contract->{limits}{parts};
    for my $part (@{$contract->{destinations}{parts}}) {
        my $part_id = $part->{part_id};
        next if !defined $part_id || !exists $part_payload{$part_id};
        my $actual = metrics($part_payload{$part_id});
        $planned{$part_id} = $actual;
        my %map = (lines => 'lines_each', bytes => 'bytes_each', line_bytes => 'line_bytes_each');
        my $warnings = warning_messages($actual, $part_limits, "planned part '$part_id' payload", \%map);
        push @$errors, "planned part '$part_id' payload starts at or above its warning boundary: $_" for @$warnings;
        enforce_ceilings($actual, $part_limits->{enforcement_ceilings}, "planned part '$part_id' payload", $errors, \%map);
    }

    if (($contract->{input_state} // '') eq 'complete') {
        my $basis = boundary_route_ids($base, $contract, $errors);
        my %expected = map { $_ => 1 } @$basis;
        my %actual = map { ($_->{leaf_id} // '') => 1 }
            grep { ($_->{origin} // '') eq 'legacy' } @{$contract->{leaf_routes}};
        for my $id (sort keys %expected) {
            push @$errors, "complete leaf routes omit boundary-history id '$id'" if !$actual{$id};
        }
        for my $id (sort keys %actual) {
            push @$errors, "complete legacy leaf route '$id' is absent from boundary path commit subjects"
                if !$expected{$id};
        }
        my %part_by_id = map { $_->{part_id} => 1 } @{$contract->{destinations}{parts}};
        for my $route (@{$contract->{leaf_routes}}) {
            my $id = $route->{leaf_id} // '';
            my $part_id = $route->{part_id} // '';
            next if !$part_by_id{$part_id};
            if (($route->{origin} // '') eq 'legacy' || ($route->{origin} // '') eq 'structural') {
                my $origin = $route->{origin};
                my $literal = $route->{source_literal} // '';
                push @$errors, "$origin leaf route '$id' source literal '$literal' is absent from its primary part payload"
                    if task_token_occurrences($part_payload{$part_id} // '', $literal) == 0;
            }
        }
    }
    $result->{source_path} = $source_path;
    $result->{source_sha256} = $sha;
    $result->{source_metrics} = $source_metrics;
    $result->{planned_part_payload_metrics} = \%planned;
    return ($source_raw, \%region_raw);
}

sub validate_destinations_absent {
    my ($base, $contract, $errors) = @_;
    my $destinations = $contract->{destinations};
    my @declared = (
        ['collection_directory', $destinations->{collection_directory}],
        ['archive_directory', $destinations->{archive_directory}],
        ['index', $destinations->{index}],
        ['manifest', $destinations->{manifest}],
        ['source_capsule', $destinations->{source_capsule}],
        map { ["part '$_->{part_id}'", $_->{path}] } @{$destinations->{parts} // []},
    );
    for my $entry (@declared) {
        my ($label, $path) = @$entry;
        next if !safe_relative_path($path);
        my $absolute = absolute($base, $path);
        push @$errors, "source_locked state has premature $label destination at '$path'"
            if -e $absolute || -l $absolute;
    }
}

sub manifest_expected_subset {
    my ($contract) = @_;
    return {
        schema_version => 1,
        contract_id => $contract->{contract_id},
        migration_state => 'migrated',
        current_path => $contract->{current_path},
        index_path => $contract->{destinations}{index},
        source_capsule => $contract->{destinations}{source_capsule},
        source => deep_clone($contract->{source}),
        current_frontier => deep_clone($contract->{current_frontier}),
        marker_prefix => $contract->{marker_prefix},
        parts => deep_clone($contract->{destinations}{parts}),
        regions => deep_clone($contract->{regions}),
        leaf_routes => deep_clone($contract->{leaf_routes}),
        migrated_on => $contract->{migration_metadata}{migrated_on},
        reason => $contract->{migration_metadata}{reason},
        verifier => $contract->{verifier},
    };
}

sub validate_manifest {
    my ($base, $contract, $errors, $result) = @_;
    my $path = $contract->{destinations}{manifest};
    my $limits = $contract->{limits}{manifest};
    my ($manifest, $raw) = read_json_object($base, $path, 'active task manifest', $limits->{bytes}, $errors);
    return if !defined $manifest;
    my $manifest_metrics = metrics($raw);
    push @$errors, "active task manifest exceeds line_bytes ceiling $limits->{line_bytes}"
        if $manifest_metrics->{line_bytes} > $limits->{line_bytes};
    validate_scalar_bounds($manifest, $limits->{scalar_bytes}, 'active task manifest', $errors);
    reject_unknown(
        $manifest,
        'active task manifest',
        $errors,
        qw(schema_version contract_id migration_state current_path index_path source_capsule source current_frontier marker_prefix parts regions leaf_routes migrated_on reason verifier),
    );
    my $migrated_on = required_scalar($manifest, 'migrated_on', 'active task manifest', $errors);
    push @$errors, 'active task manifest migrated_on is not an ISO date'
        if defined($migrated_on) && $migrated_on !~ /\A\d{4}-\d{2}-\d{2}\z/;
    required_scalar($manifest, 'reason', 'active task manifest', $errors);
    my $expected = manifest_expected_subset($contract);
    for my $field (sort keys %$expected) {
        push @$errors, "active task manifest field '$field' disagrees with the contract"
            if canonical_json($manifest->{$field}) ne canonical_json($expected->{$field});
    }
    $result->{manifest_metrics} = $manifest_metrics;
}

sub validate_part_and_regions {
    my ($base, $contract, $part, $region_raw, $seen_regions, $errors, $part_raw) = @_;
    my $path = $part->{path};
    my $raw = read_regular($base, $path, "semantic part '$part->{part_id}'", $errors);
    return if !defined $raw;
    $part_raw->{$path} = $raw if ref($part_raw) eq 'HASH';
    my $actual = metrics($raw);
    compare_metrics($actual, $part->{metrics}, "semantic part '$part->{part_id}'", $errors, qw(lines bytes line_bytes));
    push @$errors, "semantic part '$part->{part_id}' sha256 mismatch"
        if sha256_hex($raw) ne ($part->{sha256} // '');
    push @$errors, "semantic part '$part->{part_id}' lacks its exact heading"
        if index($raw, raw_scalar($part->{heading})) != 0;
    my $state_literal = "- State: `$part->{state}`";
    push @$errors, "semantic part '$part->{part_id}' lacks state literal '$state_literal'"
        if occurrences($raw, $state_literal) != 1;
    if (($part->{state} // '') eq 'sealed') {
        my $commit = $part->{sealed_commit};
        my $blob = git_capture($base, "sealed part '$part->{part_id}' blob lookup", $errors,
            'rev-parse', "$commit:$path");
        if (defined $blob) {
            $blob =~ s/[\r\n]+\z//;
            push @$errors, "sealed part '$part->{part_id}' Git blob is '$blob', expected '$part->{git_blob}'"
                if $blob ne ($part->{git_blob} // '');
        }
        my $sealed_raw = git_capture($base, "sealed part '$part->{part_id}' content read", $errors,
            'show', "$commit:$path");
        push @$errors, "sealed part '$part->{part_id}' differs from its sealing commit"
            if defined($sealed_raw) && $sealed_raw ne $raw;
        my (undef, $ancestor_exit) = command_capture('git', '-C', $base, 'merge-base', '--is-ancestor', $commit, 'HEAD');
        push @$errors, "sealed part '$part->{part_id}' commit is not an ancestor of HEAD"
            if $ancestor_exit != 0;
    }
    for my $route (@{$contract->{leaf_routes}}) {
        next if ($route->{part_id} // '') ne ($part->{part_id} // '');
        next if ($route->{origin} // '') ne 'post_migration';
        push @$errors, "post-migration leaf route '$route->{leaf_id}' is absent from its primary part"
            if index($raw, raw_scalar($route->{leaf_id} // '')) < 0;
    }

    my $prefix = $contract->{marker_prefix};
    for my $region (@{$contract->{regions}}) {
        next if ($region->{part_id} // '') ne ($part->{part_id} // '');
        my $id = $region->{region_id};
        my $start = "<!-- $prefix:$id:start -->\n";
        my $end = "<!-- $prefix:$id:end -->";
        my $start_count = occurrences($raw, $start);
        my $end_count = occurrences($raw, $end);
        push @$errors, "semantic part '$part->{part_id}' must contain one start marker for region '$id'"
            if $start_count != 1;
        push @$errors, "semantic part '$part->{part_id}' must contain one end marker for region '$id'"
            if $end_count != 1;
        next if $start_count != 1 || $end_count != 1;
        my $start_at = index($raw, $start) + length($start);
        my $end_at = index($raw, $end, $start_at);
        if ($end_at < $start_at) {
            push @$errors, "semantic part '$part->{part_id}' region '$id' markers are reversed";
            next;
        }
        my $payload = substr($raw, $start_at, $end_at - $start_at);
        push @$errors, "semantic part '$part->{part_id}' region '$id' payload differs from source"
            if $payload ne ($region_raw->{$id} // '');
        $seen_regions->{$id}++;
    }
    return $actual;
}

sub writer_command_for {
    my ($contract) = @_;
    my $command = raw_scalar($contract->{verifier} // '');
    $command =~ s/--check\z/--write/;
    return $command;
}

sub route_catalog_is_sharded {
    my ($contract) = @_;
    return (($contract->{route_catalog_state} // '') eq 'sharded') ? 1 : 0;
}

sub route_part_paths {
    my ($contract) = @_;
    return () if !route_catalog_is_sharded($contract);
    my $prefix = $contract->{destinations}{route_part_prefix};
    my $per_part = $contract->{limits}{route_parts}{routes_per_part};
    return () if !defined($prefix) || ref($prefix) || !defined($per_part) || ref($per_part) || $per_part < 1;
    my $routes = ref($contract->{leaf_routes}) eq 'ARRAY' ? scalar(@{$contract->{leaf_routes}}) : 0;
    my $count = int(($routes + $per_part - 1) / $per_part);
    $count = 1 if $count < 1;
    return map { sprintf('%s%04d.md', $prefix, $_) } 1 .. $count;
}

# The routes a sharded landing carries. An inline landing still carries every declared route, so its
# index grows with the tree's lifetime; that is the shape ADR 0046 retires tree by tree.
sub landing_leaf_routes {
    my ($contract) = @_;
    return @{$contract->{leaf_routes} // []} if !route_catalog_is_sharded($contract);
    return grep { ($_->{lifecycle} // '') eq 'open' } @{$contract->{leaf_routes} // []};
}

# The lifecycle a leaf's own primary part records, or undef when that part declares no node for it.
# Ownership is read from the owner's own status line, never inferred from a mention
# (LIVE-DOCUMENT-PRESSURE-HEADROOM.7).
sub declared_node_statuses {
    my ($raw, $leaf_id) = @_;
    my @statuses;
    return \@statuses if !defined($raw) || !defined($leaf_id) || $leaf_id eq '';
    my $needle = raw_scalar("- ID: `$leaf_id`");
    while ($raw =~ /^\Q$needle\E[ \t]*\r?\n((?:[^\r\n]*\r?\n){0,2})/mg) {
        my $tail = $1 // '';
        push @statuses, ($tail =~ /^[ \t]+(?:State|Status): `([^`\r\n]+)`/m) ? $1 : undef;
    }
    return \@statuses;
}

sub render_index {
    my ($contract) = @_;
    my $path = $contract->{destinations}{index};
    my %part_by_id = map { $_->{part_id} => $_ } @{$contract->{destinations}{parts}};
    my $raw = "# $contract->{identity}{tree_id} task-evidence index\n\n";
    $raw .= '- [Current root](' . relative_markdown_link($path, $contract->{current_path}) . ")\n";
    $raw .= '- [Manifest](' . relative_markdown_link($path, $contract->{destinations}{manifest}) . ")\n\n";
    $raw .= "## Semantic parts\n\n";
    for my $part (@{$contract->{destinations}{parts}}) {
        my $label = part_display_label($part->{part_id});
        $raw .= "- [$label](" . relative_markdown_link($path, $part->{path}) . ")\n";
    }
    if (route_catalog_is_sharded($contract)) {
        $raw .= "\n## Open leaf routes\n\n";
        $raw .= "Every leaf a session can still act on \x{2014} one this tree does not record as `done` or\n";
        $raw .= "`superseded`. The complete route set for every leaf the tree has ever declared is in the\n";
        $raw .= "route catalog below, so this landing measures work in flight rather than project age.\n\n";
    } else {
        $raw .= "\n## Primary leaf routes\n\n";
    }
    $raw .= "| Leaf | Primary detail |\n| --- | --- |\n";
    for my $route (landing_leaf_routes($contract)) {
        my $part = $part_by_id{$route->{part_id} // ''} or next;
        my $label = part_display_label($route->{part_id});
        $raw .= "| `$route->{leaf_id}` | [$label]("
            . relative_markdown_link($path, $part->{path}) . ") |\n";
    }
    my @route_paths = route_part_paths($contract);
    if (@route_paths) {
        $raw .= "\n## Complete route catalog\n\n";
        for my $position (0 .. $#route_paths) {
            $raw .= sprintf(
                "- [Route catalog part %04d](%s)\n",
                $position + 1,
                relative_markdown_link($path, $route_paths[$position]),
            );
        }
    }
    $raw .= "\n## Exact provenance\n\n";
    $raw .= '- [Source](' . relative_markdown_link($path, $contract->{destinations}{source_capsule}) . ")\n\n";
    $raw .= "## Verification\n\nRun `$contract->{verifier}` from the repository root.\n";
    return raw_scalar($raw);
}

sub render_route_part {
    my ($contract, $position, $route_paths) = @_;
    my $path = $route_paths->[$position];
    my %part_by_id = map { $_->{part_id} => $_ } @{$contract->{destinations}{parts}};
    my $per_part = $contract->{limits}{route_parts}{routes_per_part};
    my @routes = @{$contract->{leaf_routes} // []};
    my $first = $position * $per_part;
    my $last = $first + $per_part - 1;
    $last = $#routes if $last > $#routes;
    my $raw = sprintf("# %s route catalog part %04d\n\n", $contract->{identity}{tree_id}, $position + 1);
    $raw .= "> **AUTO-GENERATED \x{2014} DO NOT EDIT.** Regenerate with\n";
    $raw .= '> `' . writer_command_for($contract) . "`.\n\n";
    $raw .= "Complete membership for this range, closed leaves included. The bounded landing is\n";
    $raw .= '[the task-evidence index](' . relative_markdown_link($path, $contract->{destinations}{index})
        . "); it carries the open leaves only.\n\n";
    $raw .= "| Leaf | Lifecycle | Primary detail |\n| --- | --- | --- |\n";
    for my $position_in_range ($first .. $last) {
        my $route = $routes[$position_in_range];
        my $part = $part_by_id{$route->{part_id} // ''} or next;
        my $label = part_display_label($route->{part_id});
        my $lifecycle = $route->{lifecycle} // '';
        $raw .= "| `$route->{leaf_id}` | `$lifecycle` | [$label]("
            . relative_markdown_link($path, $part->{path}) . ") |\n";
    }
    return raw_scalar($raw);
}

sub validate_route_parts {
    my ($base, $contract, $errors, $result) = @_;
    return if !route_catalog_is_sharded($contract);
    my @route_paths = route_part_paths($contract);
    my $spec = $contract->{limits}{route_parts};
    if (!@route_paths) {
        push @$errors, 'contract declares no derivable route catalog part';
        return;
    }
    push @$errors, 'route catalog part count exceeds its declared max_parts'
        if @route_paths > ($spec->{max_parts} // 0);
    my %declared_paths = (
        ($contract->{destinations}{index} // '') => 'index',
        ($contract->{destinations}{manifest} // '') => 'manifest',
        map { ($_->{path} // '') => "semantic part '$_->{part_id}'" } @{$contract->{destinations}{parts}},
    );
    my ($files, $lines_total, $bytes_total, $max_lines, $max_bytes, $max_line_bytes) = (0, 0, 0, 0, 0, 0);
    for my $position (0 .. $#route_paths) {
        my $path = $route_paths[$position];
        if ($declared_paths{$path}) {
            push @$errors, "route catalog part '$path' collides with the declared $declared_paths{$path}";
            next;
        }
        my $raw = read_regular($base, $path, "route catalog part '$path'", $errors);
        next if !defined $raw;
        push @$errors, "route catalog part '$path' differs from its derived form; regenerate with "
            . writer_command_for($contract)
            if $raw ne render_route_part($contract, $position, \@route_paths);
        my $actual = metrics($raw);
        $files++;
        $lines_total += $actual->{lines};
        $bytes_total += $actual->{bytes};
        $max_lines = $actual->{lines} if $actual->{lines} > $max_lines;
        $max_bytes = $actual->{bytes} if $actual->{bytes} > $max_bytes;
        $max_line_bytes = $actual->{line_bytes} if $actual->{line_bytes} > $max_line_bytes;
    }
    for my $stale (unplanned_route_part_paths($base, $contract, \@route_paths, $errors)) {
        push @$errors, "unplanned route catalog file '$stale' is not derived by the contract";
    }
    my $actual = {
        files => $files,
        lines_each => $max_lines,
        bytes_each => $max_bytes,
        line_bytes_each => $max_line_bytes,
        lines_total => $lines_total,
        bytes_total => $bytes_total,
    };
    my %identity_map = map { $_ => $_ } keys %$actual;
    enforce_ceilings($actual, $spec->{enforcement_ceilings}, 'route catalog collection', $errors, \%identity_map);
    apply_live_pressure($actual, $spec, 'route catalog collection', \%identity_map, $errors, $result->{warnings});
    $result->{route_part_collection_metrics} = $actual;
}

sub unplanned_route_part_paths {
    my ($base, $contract, $route_paths, $errors) = @_;
    my $prefix = $contract->{destinations}{route_part_prefix} // '';
    return () if $prefix eq '';
    my $directory = dirname($prefix);
    my $stem = $prefix;
    $stem =~ s{\A\Q$directory\E/}{};
    my %planned = map { $_ => 1 } @$route_paths;
    my $handle;
    if (!opendir($handle, absolute($base, $directory))) {
        push @$errors, "cannot read route catalog directory '$directory'";
        return ();
    }
    my @unplanned;
    for my $entry (sort readdir $handle) {
        next if $entry !~ /\A\Q$stem\E[^\/]*\.md\z/;
        my $candidate = "$directory/$entry";
        push @unplanned, $candidate if !$planned{$candidate};
    }
    closedir $handle;
    return @unplanned;
}

# The landing claims a lifecycle for every route; where the primary part declares that leaf as a node,
# the claim must agree with the part's own status line, so the claim is re-derived rather than asserted.
sub validate_route_lifecycles {
    my ($contract, $part_raw, $errors, $result) = @_;
    return if !route_catalog_is_sharded($contract);
    my %part_path = map { $_->{part_id} => $_->{path} } @{$contract->{destinations}{parts}};
    my $unverified = 0;
    for my $route (@{$contract->{leaf_routes} // []}) {
        my $path = $part_path{$route->{part_id} // ''} // '';
        my $statuses = declared_node_statuses($part_raw->{$path}, $route->{leaf_id});
        if (@$statuses > 1) {
            push @$errors, "leaf route '$route->{leaf_id}' is declared " . scalar(@$statuses)
                . " times in its primary part '$route->{part_id}'";
            next;
        }
        my $status = @$statuses ? $statuses->[0] : undef;
        if (!defined $status) {
            $unverified++;
            next;
        }
        my $observed = ($status eq 'done' || $status eq 'superseded') ? 'closed' : 'open';
        push @$errors, "leaf route '$route->{leaf_id}' declares lifecycle '"
            . ($route->{lifecycle} // '') . "' but its primary part records status '$status'"
            if $observed ne ($route->{lifecycle} // '');
    }
    my $allowed = $contract->{limits}{route_parts}{max_unverified_routes};
    push @$errors, "$unverified leaf routes declare a lifecycle their primary part does not corroborate, "
        . "above the declared max_unverified_routes $allowed"
        if defined($allowed) && !ref($allowed) && $unverified > $allowed;
    $result->{routes_without_declared_status} = $unverified;
}

sub validate_index {
    my ($base, $contract, $errors, $result) = @_;
    my $path = $contract->{destinations}{index};
    my $raw = read_regular($base, $path, 'active task index', $errors);
    return if !defined $raw;
    my $actual = metrics($raw);
    my $spec = $contract->{limits}{index};
    enforce_ceilings($actual, $spec->{enforcement_ceilings}, 'active task index', $errors,
        {lines => 'lines', bytes => 'bytes', line_bytes => 'line_bytes'});
    apply_live_pressure(
        $actual,
        $spec,
        'active task index',
        {lines => 'lines', bytes => 'bytes', line_bytes => 'line_bytes'},
        $errors,
        $result->{warnings},
    );
    validate_required_literals($raw, $contract->{migrated_requirements}{index_required_literals}, 'active task index', $errors);
    push @$errors, 'active task index differs from its derived form; regenerate with '
        . writer_command_for($contract)
        if $raw ne render_index($contract);

    my $links = link_counts($raw, $path, 'active task index', $errors);
    my %expected = (
        $contract->{current_path} => 1,
        $contract->{destinations}{manifest} => 1,
        $contract->{destinations}{source_capsule} => 1,
    );
    $expected{$_->{path}}++ for @{$contract->{destinations}{parts}};
    $expected{$_}++ for route_part_paths($contract);
    my (%part_path, %route_rows);
    $part_path{$_->{part_id}} = $_->{path} for @{$contract->{destinations}{parts}};
    while ($raw =~ /^\| `([^`]+)` \| \[[^\]\r\n]+\]\(([^)\r\n]+)\) \|\s*$/mg) {
        my ($leaf_id, $target) = ($1, $2);
        my $resolved = normalize_link($path, $target);
        push @$errors, "active task index route row for '$leaf_id' has unsafe target"
            if !defined($resolved) || !safe_relative_path($resolved);
        push @{$route_rows{$leaf_id}}, $resolved // '';
    }
    for my $route (@{$contract->{leaf_routes}}) {
        my $leaf_id = $route->{leaf_id};
        my $expected_path = $part_path{$route->{part_id}} // '';
        my $rows = $route_rows{$leaf_id} // [];
        if (route_catalog_is_sharded($contract) && ($route->{lifecycle} // '') ne 'open') {
            push @$errors, "active task index must not route closed leaf '$leaf_id'; "
                . 'a closed leaf belongs to the route catalog'
                if @$rows;
            next;
        }
        push @$errors, "active task index must route leaf '$leaf_id' exactly once"
            if @$rows != 1;
        push @$errors, "active task index leaf '$leaf_id' routes to '$rows->[0]', expected '$expected_path'"
            if @$rows == 1 && $rows->[0] ne $expected_path;
        $expected{$expected_path}++ if $expected_path ne '';
    }
    my %declared_routes = map { ($_->{leaf_id} // '') => 1 } @{$contract->{leaf_routes}};
    push @$errors, "active task index contains undeclared leaf route '$_'"
        for grep { !$declared_routes{$_} } sort keys %route_rows;
    for my $target (sort keys %expected) {
        push @$errors, "active task index links '$target' " . ($links->{$target} // 0)
            . " times, expected $expected{$target}"
            if ($links->{$target} // 0) != $expected{$target};
    }
    push @$errors, "active task index contains unexpected local link '$_'"
        for grep { !exists $expected{$_} } sort keys %$links;
    $result->{index_metrics} = $actual;
}

sub validate_migrated {
    my ($base, $contract, $source_raw, $region_raw, $errors, $result, $options) = @_;
    $options //= {};
    my $root_raw = read_regular($base, $contract->{current_path}, 'bounded active root', $errors);
    if (defined $root_raw) {
        my $actual = metrics($root_raw);
        my $spec = $contract->{limits}{root};
        enforce_ceilings($actual, $spec->{enforcement_ceilings}, 'bounded active root', $errors,
            {lines => 'lines', bytes => 'bytes', line_bytes => 'line_bytes'});
        apply_live_pressure(
            $actual,
            $spec,
            'bounded active root',
            {lines => 'lines', bytes => 'bytes', line_bytes => 'line_bytes'},
            $errors,
            $result->{warnings},
        );
        validate_required_literals($root_raw, $contract->{migrated_requirements}{root_required_literals}, 'bounded active root', $errors);
        validate_forbidden_literals($root_raw, $contract->{migrated_requirements}{root_forbidden_literals}, 'bounded active root', $errors);
        push @$errors, 'bounded active root heading does not match contract identity'
            if index($root_raw, raw_scalar($contract->{identity}{heading})) != 0;
        my $tree_literal = "- Tree ID: `$contract->{identity}{tree_id}`";
        push @$errors, 'bounded active root does not carry exactly one declared tree identity'
            if occurrences($root_raw, $tree_literal) != 1;
        push @$errors, 'bounded active root lacks the declared active-status literal'
            if occurrences($root_raw, $contract->{identity}{active_status_literal}) == 0;
        push @$errors, 'bounded active root does not carry exactly one current-frontier literal'
            if occurrences($root_raw, $contract->{current_frontier}{literal}) != 1;
        my $links = link_counts($root_raw, $contract->{current_path}, 'bounded active root', $errors);
        my $index = $contract->{destinations}{index};
        push @$errors, "bounded active root must link index '$index' exactly once"
            if ($links->{$index} // 0) != 1;
        $result->{root_metrics} = $actual;
    }

    validate_index($base, $contract, $errors, $result) if !$options->{skip_derived};
    validate_route_parts($base, $contract, $errors, $result) if !$options->{skip_derived};
    my %seen_regions;
    my %part_raw;
    my ($files, $lines_total, $bytes_total, $max_lines, $max_bytes, $max_line_bytes) = (0, 0, 0, 0, 0, 0);
    for my $part (@{$contract->{destinations}{parts}}) {
        my $actual = validate_part_and_regions(
            $base, $contract, $part, $region_raw, \%seen_regions, $errors, \%part_raw,
        );
        next if !defined $actual;
        $files++;
        $lines_total += $actual->{lines};
        $bytes_total += $actual->{bytes};
        $max_lines = $actual->{lines} if $actual->{lines} > $max_lines;
        $max_bytes = $actual->{bytes} if $actual->{bytes} > $max_bytes;
        $max_line_bytes = $actual->{line_bytes} if $actual->{line_bytes} > $max_line_bytes;
    }
    for my $region (@{$contract->{regions}}) {
        push @$errors, "source region '$region->{region_id}' occurs in " . ($seen_regions{$region->{region_id}} // 0)
            . ' semantic parts, expected exactly one'
            if ($seen_regions{$region->{region_id}} // 0) != 1;
    }
    my $part_actual = {
        files => $files,
        lines_each => $max_lines,
        bytes_each => $max_bytes,
        line_bytes_each => $max_line_bytes,
        lines_total => $lines_total,
        bytes_total => $bytes_total,
    };
    my $part_spec = $contract->{limits}{parts};
    my %identity_map = map { $_ => $_ } keys %$part_actual;
    enforce_ceilings($part_actual, $part_spec->{enforcement_ceilings}, 'semantic part collection', $errors, \%identity_map);
    apply_live_pressure(
        $part_actual,
        $part_spec,
        'semantic part collection',
        \%identity_map,
        $errors,
        $result->{warnings},
    );
    $result->{part_collection_metrics} = $part_actual;
    validate_route_lifecycles($contract, \%part_raw, $errors, $result);
    validate_manifest($base, $contract, $errors, $result) if !$options->{skip_derived};
}

sub validate_tree {
    my ($base, $relative_contract, $options) = @_;
    $options //= {};
    my @errors;
    if (!safe_relative_path($relative_contract)) {
        push @errors, 'contract path is absolute, escaping, or malformed';
        return (\@errors, {});
    }
    my ($contract) = read_json_object($base, $relative_contract, 'contract', 131_072, \@errors);
    return (\@errors, {}) if !defined $contract;
    validate_contract_schema($contract, $relative_contract, \@errors);
    my %result = (
        migration_state => $contract->{migration_state} // '',
        input_state => $contract->{input_state} // '',
        warnings => [],
    );
    my $locked = ($contract->{migration_state} // '') eq 'source_locked';
    my $source_path = $locked ? $contract->{current_path} : $contract->{destinations}{source_capsule};
    my ($source_raw, $region_raw) = validate_source_and_inputs(
        $base,
        $contract,
        $source_path,
        $locked,
        \@errors,
        \%result,
    );
    if ($locked) {
        validate_destinations_absent($base, $contract, \@errors);
    } elsif (($contract->{migration_state} // '') eq 'migrated' && defined $source_raw) {
        validate_migrated($base, $contract, $source_raw, $region_raw, \@errors, \%result, $options);
    }
    return (\@errors, \%result);
}

sub relative_markdown_link {
    my ($from_path, $to_path) = @_;
    my $relative = File::Spec->abs2rel($to_path, dirname($from_path));
    $relative =~ s{\\}{/}g;
    return $relative;
}

sub part_display_label {
    my ($part_id) = @_;
    my $label = join ' ', split /-/, ($part_id // '');
    $label =~ s/\A([a-z])/\U$1/;
    return $label;
}

sub render_semantic_part {
    my ($contract, $source_raw, $part) = @_;
    my $lines = split_source_lines($source_raw);
    my $raw = raw_scalar(
        "$part->{heading}\n\n- Part ID: `$part->{part_id}`\n- State: `$part->{state}`\n\n",
    );
    for my $region (@{$contract->{regions}}) {
        next if ($region->{part_id} // '') ne ($part->{part_id} // '');
        my $id = $region->{region_id};
        my $payload = join '', @$lines[$region->{start_line} - 1 .. $region->{end_line} - 1];
        $raw .= raw_scalar("<!-- $contract->{marker_prefix}:$id:start -->\n");
        $raw .= $payload;
        $raw .= raw_scalar("<!-- $contract->{marker_prefix}:$id:end -->\n\n");
    }
    # Preserve the blank separator between regions without emitting a blank
    # line after the final region.
    $raw =~ s/\n\n\z/\n/;
    return $raw;
}

sub validate_migration_root_template {
    my ($contract, $root_raw, $errors) = @_;
    my $actual = metrics($root_raw);
    my $spec = $contract->{limits}{root};
    enforce_ceilings(
        $actual,
        $spec->{enforcement_ceilings},
        'migration root template',
        $errors,
        {lines => 'lines', bytes => 'bytes', line_bytes => 'line_bytes'},
    );
    my @warnings;
    apply_live_pressure(
        $actual,
        $spec,
        'migration root template',
        {lines => 'lines', bytes => 'bytes', line_bytes => 'line_bytes'},
        $errors,
        \@warnings,
    );
    push @$errors, "migration root template begins under warning pressure: $_" for @warnings;
    validate_required_literals(
        $root_raw,
        $contract->{migrated_requirements}{root_required_literals},
        'migration root template',
        $errors,
    );
    validate_forbidden_literals(
        $root_raw,
        $contract->{migrated_requirements}{root_forbidden_literals},
        'migration root template',
        $errors,
    );
    push @$errors, 'migration root template heading does not match contract identity'
        if index($root_raw, raw_scalar($contract->{identity}{heading})) != 0;
    my $tree_literal = "- Tree ID: `$contract->{identity}{tree_id}`";
    push @$errors, 'migration root template does not carry exactly one declared tree identity'
        if occurrences($root_raw, $tree_literal) != 1;
    push @$errors, 'migration root template lacks the declared active-status literal'
        if occurrences($root_raw, $contract->{identity}{active_status_literal}) == 0;
    push @$errors, 'migration root template does not carry exactly one current-frontier literal'
        if occurrences($root_raw, $contract->{current_frontier}{literal}) != 1;
    my $links = link_counts($root_raw, $contract->{current_path}, 'migration root template', $errors);
    my $index = $contract->{destinations}{index};
    push @$errors, "migration root template must link index '$index' exactly once"
        if ($links->{$index} // 0) != 1;
    push @$errors, "migration root template contains unexpected local link '$_'"
        for grep { $_ ne $index } sort keys %$links;
    return $actual;
}

sub build_migration_outputs {
    my ($contract, $contract_rel, $source_raw, $root_raw) = @_;
    my $migrated = deep_clone($contract);
    $migrated->{migration_state} = 'migrated';
    my %outputs = ($migrated->{destinations}{source_capsule} => $source_raw);
    for my $part (@{$migrated->{destinations}{parts}}) {
        my $raw = render_semantic_part($migrated, $source_raw, $part);
        $part->{sha256} = sha256_hex($raw);
        $part->{metrics} = metrics($raw);
        $outputs{$part->{path}} = $raw;
    }
    $outputs{$migrated->{destinations}{index}} = render_index($migrated);
    my @route_paths = route_part_paths($migrated);
    $outputs{$route_paths[$_]} = render_route_part($migrated, $_, \@route_paths) for 0 .. $#route_paths;
    my $manifest = manifest_expected_subset($migrated);
    $outputs{$migrated->{destinations}{manifest}}
        = raw_scalar(JSON::PP->new->canonical(1)->pretty(1)->encode($manifest));
    $outputs{$contract_rel}
        = raw_scalar(JSON::PP->new->canonical(1)->pretty(1)->encode($migrated));
    $outputs{$migrated->{current_path}} = $root_raw;
    return ($migrated, \%outputs);
}

sub write_raw_atomic {
    my ($base, $relative, $raw) = @_;
    die "active-task-evidence migration: unsafe output path '$relative'\n" if !safe_relative_path($relative);
    my $path = absolute($base, $relative);
    make_path(dirname($path));
    my $temporary = "$path.active-task-evidence.$$";
    sysopen(my $fh, $temporary, O_CREAT | O_EXCL | O_WRONLY, 0600)
        or die "active-task-evidence migration: cannot create temporary for '$relative': $!\n";
    binmode $fh, ':raw';
    my $ok = eval {
        print {$fh} raw_scalar($raw) or die "cannot write: $!";
        close $fh or die "cannot close: $!";
        chmod 0644, $temporary or die "cannot chmod: $!";
        rename $temporary, $path or die "cannot rename: $!";
        1;
    };
    if (!$ok) {
        my $reason = $@ || 'unknown write failure';
        close $fh if defined(fileno($fh));
        unlink $temporary if -e $temporary || -l $temporary;
        die "active-task-evidence migration: $relative: $reason\n";
    }
}

sub materialize_migration {
    my ($base, $relative_contract, $template_rel, $quiet) = @_;
    die "active-task-evidence migration: root template path is unsafe\n"
        if !safe_relative_path($template_rel);
    my ($existing_errors) = validate_tree($base, $relative_contract);
    die "active-task-evidence migration: source preflight failed:\n"
        . join("\n", map { "- $_" } @$existing_errors) . "\n"
        if @$existing_errors;
    my @read_errors;
    my ($contract, $contract_raw) = read_json_object(
        $base, $relative_contract, 'migration contract', 131_072, \@read_errors,
    );
    my $source_raw = defined($contract)
        ? read_regular($base, $contract->{current_path}, 'migration source', \@read_errors)
        : undef;
    my $root_raw = read_regular($base, $template_rel, 'migration root template', \@read_errors);
    die "active-task-evidence migration: input read failed:\n"
        . join("\n", map { "- $_" } @read_errors) . "\n"
        if @read_errors || !defined($contract) || !defined($source_raw) || !defined($root_raw);
    die "active-task-evidence migration: contract must be source_locked/complete\n"
        if ($contract->{migration_state} // '') ne 'source_locked'
        || ($contract->{input_state} // '') ne 'complete';
    die "active-task-evidence migration: contract lacks migration_metadata\n"
        if ref($contract->{migration_metadata}) ne 'HASH';
    die "active-task-evidence migration: root template must not be a declared authority path\n"
        if $template_rel eq $relative_contract
        || $template_rel eq $contract->{current_path}
        || grep { $template_rel eq $_ } (
            $contract->{destinations}{index},
            $contract->{destinations}{manifest},
            $contract->{destinations}{source_capsule},
            map { $_->{path} } @{$contract->{destinations}{parts}},
        );
    my @template_errors;
    my $root_metrics = validate_migration_root_template($contract, $root_raw, \@template_errors);
    die "active-task-evidence migration: root template preflight failed:\n"
        . join("\n", map { "- $_" } @template_errors) . "\n"
        if @template_errors;

    my ($migrated, $outputs) = build_migration_outputs(
        $contract, $relative_contract, $source_raw, $root_raw,
    );
    my @write_order = (
        $migrated->{destinations}{source_capsule},
        (map { $_->{path} } @{$migrated->{destinations}{parts}}),
        route_part_paths($migrated),
        $migrated->{destinations}{index},
        $migrated->{destinations}{manifest},
        $relative_contract,
        $migrated->{current_path},
    );
    my $write_error;
    eval { write_raw_atomic($base, $_, $outputs->{$_}) for @write_order; 1 } or $write_error = $@;
    my ($migrated_errors) = $write_error ? ([]) : validate_tree($base, $relative_contract);
    if ($write_error || @$migrated_errors) {
        my $reason = $write_error || join("\n", map { "- $_" } @$migrated_errors);
        eval {
            write_raw_atomic($base, $contract->{current_path}, $source_raw);
            write_raw_atomic($base, $relative_contract, $contract_raw);
            remove_tree(absolute($base, $contract->{destinations}{collection_directory}));
            remove_tree(absolute($base, $contract->{destinations}{archive_directory}));
            1;
        } or $reason .= "\nrollback failed: $@";
        die "active-task-evidence migration: transaction failed and was rolled back:\n$reason\n";
    }
    my $part_count = scalar @{$migrated->{destinations}{parts}};
    print "active-task-evidence migration: wrote bounded root ($root_metrics->{lines} lines / "
        . "$root_metrics->{bytes} bytes), $part_count semantic parts, index, manifest, and exact source capsule.\n"
        if !$quiet;
}

# Regenerate every derived surface the contract owns - the bounded landing, the complete route
# catalog, and the manifest - as one transaction that restores the exact previous bytes on failure.
# Canonical evidence (root, semantic parts, capsule, contract) is never written here.
sub materialize_derived {
    my ($base, $relative_contract, $quiet) = @_;
    my ($preflight_errors) = validate_tree($base, $relative_contract, {skip_derived => 1});
    die "active-task-evidence writer: preflight failed:\n"
        . join("\n", map { "- $_" } @$preflight_errors) . "\n"
        if @$preflight_errors;
    my @read_errors;
    my ($contract) = read_json_object($base, $relative_contract, 'writer contract', 131_072, \@read_errors);
    die "active-task-evidence writer: contract read failed:\n" . join("\n", @read_errors) . "\n"
        if @read_errors || !defined $contract;
    die "active-task-evidence writer: contract must be migrated/complete\n"
        if ($contract->{migration_state} // '') ne 'migrated'
        || ($contract->{input_state} // '') ne 'complete';

    my @route_paths = route_part_paths($contract);
    die "active-task-evidence writer: sharded contract derives no route catalog part\n"
        if route_catalog_is_sharded($contract) && !@route_paths;
    my %outputs = ($contract->{destinations}{index} => render_index($contract));
    $outputs{$route_paths[$_]} = render_route_part($contract, $_, \@route_paths) for 0 .. $#route_paths;
    $outputs{$contract->{destinations}{manifest}} = raw_scalar(
        JSON::PP->new->canonical(1)->pretty(1)->encode(manifest_expected_subset($contract)),
    );
    my @scan_errors;
    my @stale = unplanned_route_part_paths($base, $contract, \@route_paths, \@scan_errors);
    die "active-task-evidence writer: cannot enumerate route catalog parts:\n"
        . join("\n", map { "- $_" } @scan_errors) . "\n"
        if @scan_errors;

    my %previous;
    for my $relative (sort(keys %outputs), @stale) {
        my @ignored;
        $previous{$relative} = read_regular($base, $relative, 'writer preimage', \@ignored);
    }
    my $write_error;
    eval {
        write_raw_atomic($base, $_, $outputs{$_}) for sort keys %outputs;
        for my $relative (@stale) {
            unlink absolute($base, $relative)
                or die "cannot remove stale route catalog part '$relative': $!\n";
        }
        1;
    } or $write_error = $@;
    my ($post_errors) = $write_error ? ([]) : validate_tree($base, $relative_contract);
    if ($write_error || @$post_errors) {
        my $reason = $write_error || join("\n", map { "- $_" } @$post_errors);
        eval {
            for my $relative (sort keys %previous) {
                if (defined $previous{$relative}) {
                    write_raw_atomic($base, $relative, $previous{$relative});
                } elsif (-e absolute($base, $relative)) {
                    unlink absolute($base, $relative);
                }
            }
            1;
        } or $reason .= "\nrollback failed: $@";
        die "active-task-evidence writer: transaction failed and was rolled back:\n$reason\n";
    }
    my $removed = scalar @stale;
    print 'active-task-evidence writer: wrote the bounded index ('
        . scalar(landing_leaf_routes($contract)) . ' routed of '
        . scalar(@{$contract->{leaf_routes}}) . ' leaf routes), '
        . scalar(@route_paths) . " route catalog part(s), and the manifest; removed $removed stale part(s).\n"
        if !$quiet;
}

sub write_raw {
    my ($base, $relative, $raw) = @_;
    my $path = absolute($base, $relative);
    make_path(dirname($path));
    open my $fh, '>:raw', $path or die "active-task-evidence self-test: cannot write $relative: $!\n";
    print {$fh} raw_scalar($raw);
    close $fh or die "active-task-evidence self-test: cannot close $relative: $!\n";
}

sub fixture_source {
    return raw_scalar(<<'SOURCE');
# PROGRAM: fixture
## Metadata
- Tree ID: `PROGRAM`
- Status: `active`
## Task tree — legacy
- ID: `PROGRAM`
- ID: `.1`
## Verification Log
| Date | Leaf | Checks | Result |
| 2026-08-09 | PROGRAM.1 | fixture | green |
SOURCE
}

sub fixture_contract {
    my ($input_state, $source_raw) = @_;
    my $source_metrics = metrics($source_raw);
    my @parts = (
        {part_id => 'foundation', path => 'docs/tasks/program/foundation.md', heading => '# PROGRAM — foundation', state => 'legacy'},
        {part_id => 'activity', path => 'docs/tasks/program/activity.md', heading => '# PROGRAM — activity', state => 'legacy'},
    );
    my @regions = (
        {region_id => 'identity', start_line => 1, end_line => 5, part_id => 'foundation'},
        {region_id => 'activity', start_line => 6, end_line => 7, part_id => 'activity'},
        {region_id => 'verification', start_line => 8, end_line => 10, part_id => 'foundation'},
    );
    my $contract = {
        schema_version => 1,
        contract_id => 'fixture-active-task',
        migration_state => 'source_locked',
        input_state => $input_state,
        current_path => 'docs/tasks/PROGRAM.md',
        identity => {
            tree_id => 'PROGRAM',
            heading => '# PROGRAM: fixture',
            active_status_literal => '- Status: `active`',
        },
        current_frontier => {mode => 'none', literal => 'No eligible frontier.'},
        migration_metadata => {migrated_on => '2026-08-09', reason => 'fixture migration'},
        source => {
            boundary_commit => '0' x 40,
            git_blob => '0' x 40,
            sha256 => sha256_hex($source_raw),
            metrics => $source_metrics,
        },
        source_requirements => ['# PROGRAM: fixture', '- Tree ID: `PROGRAM`', '## Verification Log'],
        destinations => {
            collection_directory => 'docs/tasks/program',
            index => 'docs/tasks/program/INDEX.md',
            manifest => 'docs/tasks/program/manifest.json',
            parts => \@parts,
            route_part_prefix => 'docs/tasks/program/routes-',
            archive_directory => 'docs/archive/tasks/program',
            source_capsule => 'docs/archive/tasks/program/source.md',
        },
        regions => \@regions,
        leaf_routes => [],
        route_basis => 'boundary_path_commit_subject_ids',
        route_catalog_state => 'sharded',
        marker_prefix => 'active-task-source-region',
        limits => {
            root => {
                health_targets => {lines => 64, bytes => 4096, line_bytes => 256},
                enforcement_ceilings => {lines => 96, bytes => 6144, line_bytes => 512},
                milestones => {warning_pct => 80, rollover_pct => 90},
            },
            index => {
                health_targets => {lines => 64, bytes => 4096, line_bytes => 256},
                enforcement_ceilings => {lines => 96, bytes => 6144, line_bytes => 512},
                milestones => {warning_pct => 80, rollover_pct => 90},
            },
            parts => {
                health_targets => {files => 8, lines_each => 64, bytes_each => 4096, line_bytes_each => 256, lines_total => 256, bytes_total => 16384},
                enforcement_ceilings => {files => 12, lines_each => 96, bytes_each => 6144, line_bytes_each => 512, lines_total => 512, bytes_total => 32768},
                milestones => {warning_pct => 80, rollover_pct => 90},
            },
            route_parts => {
                health_targets => {files => 4, lines_each => 32, bytes_each => 4096, line_bytes_each => 256, lines_total => 128, bytes_total => 16384},
                enforcement_ceilings => {files => 6, lines_each => 48, bytes_each => 6144, line_bytes_each => 384, lines_total => 192, bytes_total => 24576},
                milestones => {warning_pct => 80, rollover_pct => 90},
                routes_per_part => 8,
                max_parts => 4,
                max_unverified_routes => 2,
            },
            capsule_exact => $source_metrics,
            manifest => {bytes => 32768, line_bytes => 1024, scalar_bytes => 512, max_parts => 8, max_regions => 8, max_leaf_routes => 16},
        },
        migrated_requirements => {
            root_required_literals => ['# PROGRAM: fixture', '- Tree ID: `PROGRAM`', '- Status: `active`', '- ID: `PROGRAM`', '- ID: `PROGRAM.1`', '## Current Frontier', 'No eligible frontier.', '## Detailed task evidence', '## Verification Log', '## Commit Log'],
            root_forbidden_literals => ['- Status: `done`'],
            index_required_literals => ['# PROGRAM task-evidence index', '## Semantic parts', '## Open leaf routes', '## Complete route catalog', '## Exact provenance', '## Verification'],
        },
        verifier => 'perl scripts/check_active_task_evidence.pl --check',
    };
    return $contract;
}

sub region_bytes {
    my ($source_raw, $region) = @_;
    my $lines = split_source_lines($source_raw);
    return join '', @$lines[$region->{start_line} - 1 .. $region->{end_line} - 1];
}

sub complete_fixture_inputs {
    my ($contract, $source_raw) = @_;
    $contract->{input_state} = 'complete';
    for my $region (@{$contract->{regions}}) {
        my $raw = region_bytes($source_raw, $region);
        $region->{sha256} = sha256_hex($raw);
        $region->{metrics} = metrics($raw);
    }
    $contract->{leaf_routes} = [
        {leaf_id => 'PROGRAM.1', part_id => 'activity', origin => 'legacy', source_literal => '.1', lifecycle => 'closed'},
        {leaf_id => 'PROGRAM', part_id => 'foundation', origin => 'structural', source_literal => 'PROGRAM', lifecycle => 'open'},
    ];
}

sub fixture_part_raw {
    my ($contract, $source_raw, $part) = @_;
    my $raw = raw_scalar(
        "$part->{heading}\n\n- Part ID: `$part->{part_id}`\n- State: `$part->{state}`\n\n",
    );
    for my $region (@{$contract->{regions}}) {
        next if $region->{part_id} ne $part->{part_id};
        my $id = $region->{region_id};
        $raw .= raw_scalar("<!-- $contract->{marker_prefix}:$id:start -->\n");
        $raw .= region_bytes($source_raw, $region);
        $raw .= raw_scalar("<!-- $contract->{marker_prefix}:$id:end -->\n\n");
    }
    return $raw;
}

sub fixture_root {
    return <<'ROOT';
# PROGRAM: fixture

## Metadata

- Tree ID: `PROGRAM`
- Status: `active`

## Task owners

- ID: `PROGRAM`
- ID: `PROGRAM.1`

## Current Frontier

No eligible frontier.

## Detailed task evidence

[Complete task evidence](program/INDEX.md)

## Verification Log

Verified.

## Commit Log

Committed.
ROOT
}

sub convert_fixture_to_inline {
    my ($base, $contract) = @_;
    $contract->{route_catalog_state} = 'inline';
    delete $contract->{destinations}{route_part_prefix};
    delete $contract->{limits}{route_parts};
    delete $_->{lifecycle} for @{$contract->{leaf_routes}};
    $contract->{migrated_requirements}{index_required_literals} = [
        '# PROGRAM task-evidence index', '## Semantic parts', '## Primary leaf routes',
        '## Exact provenance', '## Verification',
    ];
    unlink absolute($base, 'docs/tasks/program/routes-0001.md');
    write_fixture_derived($base, $contract);
}

sub write_fixture_derived {
    my ($base, $contract) = @_;
    write_raw($base, $contract->{destinations}{index}, render_index($contract));
    my @route_paths = route_part_paths($contract);
    write_raw($base, $route_paths[$_], render_route_part($contract, $_, \@route_paths))
        for 0 .. $#route_paths;
    write_raw(
        $base,
        $contract->{destinations}{manifest},
        JSON::PP->new->canonical(1)->pretty(1)->encode(manifest_expected_subset($contract)),
    );
}

sub append_fixture_activity {
    my ($base, $contract, $node_status) = @_;
    $node_status //= 'pending';
    my $part = {
        part_id => 'activity-02',
        path => 'docs/tasks/program/activity-02.md',
        heading => '# PROGRAM — activity 02',
        state => 'active',
    };
    my $part_raw = raw_scalar(
        "$part->{heading}\n\n- Part ID: `$part->{part_id}`\n- State: `$part->{state}`\n\n"
        . "## PROGRAM.2\n\n- ID: `PROGRAM.2`\n  State: `$node_status`\n  Goal: new bounded activity\n",
    );
    $part->{sha256} = sha256_hex($part_raw);
    $part->{metrics} = metrics($part_raw);
    push @{$contract->{destinations}{parts}}, $part;
    push @{$contract->{leaf_routes}}, {
        leaf_id => 'PROGRAM.2',
        part_id => $part->{part_id},
        origin => 'post_migration',
        lifecycle => ($node_status eq 'done' || $node_status eq 'superseded') ? 'closed' : 'open',
    };
    $contract->{current_frontier} = {
        mode => 'eligible',
        literal => 'Eligible frontier: PROGRAM.2.',
        leaf_id => 'PROGRAM.2',
        part_id => $part->{part_id},
    };
    for my $literal (@{$contract->{migrated_requirements}{root_required_literals}}) {
        $literal = 'Eligible frontier: PROGRAM.2.' if $literal eq 'No eligible frontier.';
    }
    push @{$contract->{migrated_requirements}{root_required_literals}}, '- ID: `PROGRAM.2`';
    my $root_raw = fixture_root();
    $root_raw =~ s/- ID: `PROGRAM\.1`\n/- ID: `PROGRAM.1`\n- ID: `PROGRAM.2`\n/;
    $root_raw =~ s/No eligible frontier\./Eligible frontier: PROGRAM.2./;
    write_raw($base, $contract->{current_path}, $root_raw);
    write_raw($base, $part->{path}, $part_raw);
    write_fixture_derived($base, $contract);
}

sub init_fixture_git {
    my ($base, $contract, $source_raw) = @_;
    write_raw($base, $contract->{current_path}, $source_raw);
    system('git', '-C', $base, 'init', '-q') == 0 or die "active-task-evidence self-test: git init failed\n";
    system('git', '-C', $base, 'add', '--', $contract->{current_path}) == 0
        or die "active-task-evidence self-test: git add failed\n";
    system(
        'git', '-C', $base,
        '-c', 'user.name=Fixture',
        '-c', 'user.email=fixture@example.invalid',
        '-c', 'commit.gpgsign=false',
        'commit', '-qm', 'PROGRAM.1 — fixture source',
    ) == 0 or die "active-task-evidence self-test: git commit failed\n";
    my ($commit, $commit_exit) = command_capture('git', '-C', $base, 'rev-parse', 'HEAD');
    die "active-task-evidence self-test: git rev-parse failed\n" if $commit_exit;
    $commit =~ s/[\r\n]+\z//;
    my ($blob, $blob_exit) = command_capture('git', '-C', $base, 'rev-parse', "HEAD:$contract->{current_path}");
    die "active-task-evidence self-test: git blob lookup failed\n" if $blob_exit;
    $blob =~ s/[\r\n]+\z//;
    $contract->{source}{boundary_commit} = $commit;
    $contract->{source}{git_blob} = $blob;
}

sub seed_fixture {
    my ($base, $state, $input_state, $mutator, $fixture_contract_rel) = @_;
    $fixture_contract_rel //= $DEFAULT_CONTRACT_REL;
    my $source_raw = fixture_source();
    my $contract = fixture_contract($input_state, $source_raw);
    $contract->{verifier} = verifier_command_for($fixture_contract_rel);
    init_fixture_git($base, $contract, $source_raw);
    complete_fixture_inputs($contract, $source_raw) if $input_state eq 'complete';
    if ($state eq 'migrated') {
        $contract->{migration_state} = 'migrated';
        write_raw($base, $contract->{destinations}{source_capsule}, $source_raw);
        write_raw($base, $contract->{current_path}, fixture_root());
        for my $part (@{$contract->{destinations}{parts}}) {
            my $raw = raw_scalar(fixture_part_raw($contract, $source_raw, $part));
            $part->{sha256} = sha256_hex($raw);
            $part->{metrics} = metrics($raw);
            write_raw($base, $part->{path}, $raw);
        }
        write_fixture_derived($base, $contract);
    }
    $mutator->($base, $contract, $source_raw) if defined $mutator;
    write_raw(
        $base,
        $fixture_contract_rel,
        JSON::PP->new->canonical(1)->pretty(1)->encode($contract),
    );
}

sub seal_fixture_activity_part {
    my ($base, $contract, $source_raw) = @_;
    my $part = $contract->{destinations}{parts}[1];
    $part->{state} = 'sealed';
    my $raw = raw_scalar(fixture_part_raw($contract, $source_raw, $part));
    $part->{sha256} = sha256_hex($raw);
    $part->{metrics} = metrics($raw);
    write_raw($base, $part->{path}, $raw);
    system('git', '-C', $base, 'add', '--', $part->{path}) == 0
        or die "active-task-evidence self-test: sealed part git add failed\n";
    system(
        'git', '-C', $base,
        '-c', 'user.name=Fixture',
        '-c', 'user.email=fixture@example.invalid',
        '-c', 'commit.gpgsign=false',
        'commit', '-qm', 'PROGRAM.1 — seal fixture part',
    ) == 0 or die "active-task-evidence self-test: sealed part git commit failed\n";
    my ($commit, $commit_exit) = command_capture('git', '-C', $base, 'rev-parse', 'HEAD');
    die "active-task-evidence self-test: sealed commit lookup failed\n" if $commit_exit;
    $commit =~ s/[\r\n]+\z//;
    my ($blob, $blob_exit) = command_capture('git', '-C', $base, 'rev-parse', "HEAD:$part->{path}");
    die "active-task-evidence self-test: sealed blob lookup failed\n" if $blob_exit;
    $blob =~ s/[\r\n]+\z//;
    $part->{sealed_commit} = $commit;
    $part->{git_blob} = $blob;
    my $manifest = manifest_expected_subset($contract);
    $manifest->{migrated_on} = '2026-08-09';
    $manifest->{reason} = 'fixture migration';
    write_raw(
        $base,
        $contract->{destinations}{manifest},
        JSON::PP->new->canonical(1)->pretty(1)->encode($manifest),
    );
}

sub run_self_test {
    my ($base) = @_;
    my $generated = File::Spec->catdir($base, 'generated');
    make_path($generated);
    my @cases = (
        ['source-locked topology positive', 'source_locked', 'topology_declared', undef, undef],
        ['non-default contract verifier positive', 'source_locked', 'topology_declared', undef, undef, 'doctrine/live_document_size/fixture_task_evidence.json'],
        ['non-default contract verifier mismatch', 'source_locked', 'topology_declared', sub { $_[1]{verifier} = 'perl scripts/check_active_task_evidence.pl --check' }, qr/contract verifier must be/, 'doctrine/live_document_size/fixture_task_evidence.json'],
        ['source-locked complete positive', 'source_locked', 'complete', undef, undef],
        ['unknown contract field', 'source_locked', 'topology_declared', sub { $_[1]{unknown} = 1 }, qr/unknown field/],
        ['unsafe current path', 'source_locked', 'topology_declared', sub { $_[1]{current_path} = '../escape.md' }, qr/current_path is unsafe/],
        ['working source drift', 'source_locked', 'topology_declared', sub { write_raw($_[0], $_[1]{current_path}, $_[2] . "changed\n") }, qr/source authority/],
        ['boundary commit drift', 'source_locked', 'topology_declared', sub { $_[1]{source}{boundary_commit} = 'f' x 40 }, qr/boundary commit lookup/],
        ['Git blob drift', 'source_locked', 'topology_declared', sub { $_[1]{source}{git_blob} = 'f' x 40 }, qr/boundary blob/],
        ['region gap', 'source_locked', 'topology_declared', sub { $_[1]{regions}[1]{start_line}++ }, qr/not contiguous/],
        ['region unknown part', 'source_locked', 'topology_declared', sub { $_[1]{regions}[1]{part_id} = 'missing' }, qr/unknown part/],
        ['duplicate part path', 'source_locked', 'topology_declared', sub { $_[1]{destinations}{parts}[1]{path} = $_[1]{destinations}{parts}[0]{path} }, qr/duplicate part path/],
        ['invalid limit pair', 'source_locked', 'topology_declared', sub { $_[1]{limits}{root}{health_targets}{bytes} = 7000 }, qr/health bytes exceeds/],
        ['portable ceiling inflation', 'source_locked', 'topology_declared', sub { $_[1]{limits}{parts}{enforcement_ceilings}{files} = 25 }, qr/exceeds portable cap 24/],
        ['exact wide part ceiling positive', 'source_locked', 'topology_declared', sub { $_[1]{limits}{parts}{enforcement_ceilings}{line_bytes_each} = 6400 }, undef],
        ['exact wide part ceiling inflation', 'source_locked', 'topology_declared', sub { $_[1]{limits}{parts}{enforcement_ceilings}{line_bytes_each} = 6401 }, qr/exceeds portable cap 6400/],
        ['premature collection directory', 'source_locked', 'topology_declared', sub { make_path(absolute($_[0], $_[1]{destinations}{collection_directory})) }, qr/premature collection_directory/],
        ['premature archive directory', 'source_locked', 'topology_declared', sub { make_path(absolute($_[0], $_[1]{destinations}{archive_directory})) }, qr/premature archive_directory/],
        ['topology route leakage', 'source_locked', 'topology_declared', sub { $_[1]{leaf_routes} = [{leaf_id => 'PROGRAM.1', part_id => 'activity', origin => 'legacy'}] }, qr/must not contain leaf routes/],
        ['complete region digest missing', 'source_locked', 'complete', sub { delete $_[1]{regions}[0]{sha256} }, qr/lacks non-empty scalar 'sha256'/],
        ['complete route omission', 'source_locked', 'complete', sub { $_[1]{leaf_routes} = [] }, qr/must contain leaf routes|omit boundary-history/],
        ['complete route source literal missing', 'source_locked', 'complete', sub { delete $_[1]{leaf_routes}[0]{source_literal} }, qr/lacks non-empty scalar 'source_literal'/],
        ['complete route source literal invalid', 'source_locked', 'complete', sub { $_[1]{leaf_routes}[0]{source_literal} = 'PROGRAM' }, qr/not its full or tree-relative id/],
        ['complete route source literal absent', 'source_locked', 'complete', sub { $_[1]{leaf_routes}[0]{source_literal} = 'PROGRAM.1' }, qr/source literal 'PROGRAM\.1' is absent/],
        ['complete structural owner route omitted', 'source_locked', 'complete', sub { pop @{$_[1]{leaf_routes}} }, qr/omit declared root owner 'PROGRAM'/],
        ['complete undeclared structural route', 'source_locked', 'complete', sub { push @{$_[1]{leaf_routes}}, {leaf_id => 'PROGRAM.9', part_id => 'foundation', origin => 'structural', source_literal => 'PROGRAM'} }, qr/leaf route 'PROGRAM\.9' is absent from the declared root owner registry/],
        ['structural route source literal missing', 'source_locked', 'complete', sub { delete $_[1]{leaf_routes}[1]{source_literal} }, qr/structural contract leaf route lacks non-empty scalar 'source_literal'/],
        ['structural route source literal absent', 'source_locked', 'complete', sub { $_[1]{leaf_routes}[1]{leaf_id} = 'PROGRAM.9'; $_[1]{leaf_routes}[1]{source_literal} = '.9' }, qr/structural leaf route 'PROGRAM\.9' source literal '\.9' is absent/],
        ['premature post-migration route', 'source_locked', 'complete', sub { $_[1]{leaf_routes}[0]{origin} = 'post_migration' }, qr/cannot declare post-migration/],
        ['migrated positive', 'migrated', 'complete', undef, undef],
        ['capsule mutation', 'migrated', 'complete', sub { write_raw($_[0], $_[1]{destinations}{source_capsule}, $_[2] . "changed\n") }, qr/source authority/],
        ['root owner declaration missing', 'migrated', 'complete', sub { my $raw = fixture_root(); $raw =~ s/^- ID: `PROGRAM\.1`\n//m; write_raw($_[0], $_[1]{current_path}, $raw) }, qr/lacks required literal '- ID: `PROGRAM\.1`'/],
        ['root frontier missing', 'migrated', 'complete', sub { my $raw = fixture_root(); $raw =~ s/No eligible frontier\./Frontier unknown./; write_raw($_[0], $_[1]{current_path}, $raw) }, qr/frontier/],
        ['root mandatory rollover', 'migrated', 'complete', sub { $_[1]{limits}{root}{health_targets}{lines} = 20 }, qr/mandatory rollover/],
        ['index part route missing', 'migrated', 'complete', sub { my $raw = render_index($_[1]); $raw =~ s/^- \[Foundation\]\(foundation\.md\)\n//m; write_raw($_[0], $_[1]{destinations}{index}, $raw) }, qr/links 'docs\/tasks\/program\/foundation.md'/],
        ['index leaf misroute', 'migrated', 'complete', sub { my $raw = render_index($_[1]); $raw =~ s/\[Foundation\]\(foundation\.md\) \|/\[Activity](activity.md) |/; write_raw($_[0], $_[1]{destinations}{index}, $raw) }, qr/leaf 'PROGRAM' routes/],
        ['index derived drift', 'migrated', 'complete', sub { my $raw = render_index($_[1]); $raw =~ s/measures work in flight/measures nothing in particular/; write_raw($_[0], $_[1]{destinations}{index}, $raw) }, qr/index differs from its derived form/],
        ['index carries a closed leaf', 'migrated', 'complete', sub { my $raw = render_index($_[1]); my $row = '| `PROGRAM.1` | [Activity](activity.md) |'; $raw =~ s/^(\Q| `PROGRAM` | [Foundation](foundation.md) |\E\n)/$1$row\n/m; write_raw($_[0], $_[1]{destinations}{index}, $raw) }, qr/must not route closed leaf 'PROGRAM\.1'/],
        ['route catalog drift', 'migrated', 'complete', sub { my @paths = route_part_paths($_[1]); my $raw = render_route_part($_[1], 0, \@paths); $raw =~ s/`closed`/`open`/; write_raw($_[0], $paths[0], $raw) }, qr/route catalog part '[^']+' differs from its derived form/],
        ['unplanned route catalog part', 'migrated', 'complete', sub { write_raw($_[0], 'docs/tasks/program/routes-0002.md', "# stray\n") }, qr/unplanned route catalog file 'docs\/tasks\/program\/routes-0002.md'/],
        ['route catalog capacity below route count', 'migrated', 'complete', sub { $_[1]{limits}{route_parts}{max_parts} = 1; $_[1]{limits}{route_parts}{routes_per_part} = 1 }, qr/route catalog capacity is below the declared leaf route count/],
        ['route part health admits no full part', 'migrated', 'complete', sub { $_[1]{limits}{route_parts}{health_targets}{lines_each} = 20 }, qr/admits no full part below its warning band/],
        ['leaf route lifecycle missing', 'migrated', 'complete', sub { delete $_[1]{leaf_routes}[0]{lifecycle} }, qr/lacks non-empty scalar 'lifecycle'/],
        ['leaf route lifecycle invalid', 'migrated', 'complete', sub { $_[1]{leaf_routes}[0]{lifecycle} = 'retired' }, qr/has invalid lifecycle 'retired'/],
        ['uncorroborated routes exceed their ratchet', 'migrated', 'complete', sub { $_[1]{limits}{route_parts}{max_unverified_routes} = 1 }, qr/above the declared max_unverified_routes 1/],
        ['inline route catalog positive', 'migrated', 'complete', sub { convert_fixture_to_inline($_[0], $_[1]) }, undef],
        ['inline route catalog with a lifecycle', 'migrated', 'complete', sub { convert_fixture_to_inline($_[0], $_[1]); $_[1]{leaf_routes}[0]{lifecycle} = 'closed' }, qr/must not declare a lifecycle/],
        ['inline route catalog with a route prefix', 'migrated', 'complete', sub { convert_fixture_to_inline($_[0], $_[1]); $_[1]{destinations}{route_part_prefix} = 'docs/tasks/program/routes-' }, qr/must not declare a route_part_prefix/],
        ['unknown route catalog state', 'migrated', 'complete', sub { $_[1]{route_catalog_state} = 'partitioned' }, qr/invalid route_catalog_state 'partitioned'/],
        ['post-migration closed append positive', 'migrated', 'complete', sub { append_fixture_activity($_[0], $_[1], 'done') }, undef],
        ['leaf route lifecycle disagrees with its primary part', 'migrated', 'complete', sub { append_fixture_activity($_[0], $_[1], 'done'); $_[1]{leaf_routes}[-1]{lifecycle} = 'open'; write_fixture_derived($_[0], $_[1]) }, qr/declares lifecycle 'open' but its primary part records status 'done'/],
        ['part payload mutation', 'migrated', 'complete', sub { my $part = $_[1]{destinations}{parts}[1]; my $raw = fixture_part_raw($_[1], $_[2], $part); $raw =~ s/- ID: `\.1`/- ID: `.2`/; write_raw($_[0], $part->{path}, $raw) }, qr/semantic part|payload differs/],
        ['manifest identity drift', 'migrated', 'complete', sub { my $manifest = manifest_expected_subset($_[1]); $manifest->{migrated_on} = '2026-08-09'; $manifest->{reason} = 'fixture migration'; $manifest->{source}{sha256} = 'f' x 64; write_raw($_[0], $_[1]{destinations}{manifest}, JSON::PP->new->canonical(1)->pretty(1)->encode($manifest)) }, qr/manifest field 'source' disagrees/],
        ['manifest scalar overflow', 'migrated', 'complete', sub { my $manifest = manifest_expected_subset($_[1]); $manifest->{migrated_on} = '2026-08-09'; $manifest->{reason} = 'x' x 600; write_raw($_[0], $_[1]{destinations}{manifest}, JSON::PP->new->canonical(1)->pretty(1)->encode($manifest)) }, qr/scalar above 512 bytes/],
        ['post-migration active append positive', 'migrated', 'complete', sub { append_fixture_activity($_[0], $_[1]) }, undef],
        ['sealed part positive', 'migrated', 'complete', sub { seal_fixture_activity_part($_[0], $_[1], $_[2]) }, undef],
        ['sealed part mutation', 'migrated', 'complete', sub { seal_fixture_activity_part($_[0], $_[1], $_[2]); my $part = $_[1]{destinations}{parts}[1]; write_raw($_[0], $part->{path}, fixture_part_raw($_[1], $_[2], $part) . "changed\n") }, qr/differs from its sealing commit/],
    );

    my $passed = 0;
    for my $index (0 .. $#cases) {
        my ($name, $state, $input_state, $mutator, $expected, $fixture_contract_rel) = @{$cases[$index]};
        $fixture_contract_rel //= $DEFAULT_CONTRACT_REL;
        my $fixture = File::Spec->catdir($generated, ".active-task-evidence-self-test.$$.$index");
        remove_tree($fixture) if -e $fixture;
        make_path($fixture);
        my $case_failure;
        eval {
            seed_fixture($fixture, $state, $input_state, $mutator, $fixture_contract_rel);
            my ($errors) = validate_tree($fixture, $fixture_contract_rel);
            my $joined = join "\n", @$errors;
            if (!defined $expected) {
                die "active-task-evidence self-test '$name' unexpectedly failed:\n$joined\n" if @$errors;
            } else {
                die "active-task-evidence self-test '$name' unexpectedly passed\n" if !@$errors;
                die "active-task-evidence self-test '$name' missed expected diagnostic $expected:\n$joined\n"
                    if $joined !~ $expected;
            }
            1;
        } or $case_failure = $@ || "active-task-evidence self-test '$name' failed without a diagnostic\n";
        remove_tree($fixture);
        die $case_failure if defined $case_failure;
        $passed++;
    }

    my $writer_fixture = File::Spec->catdir($generated, ".active-task-evidence-writer-self-test.$$");
    remove_tree($writer_fixture) if -e $writer_fixture;
    make_path($writer_fixture);
    my $positive_failure;
    eval {
        seed_fixture($writer_fixture, 'source_locked', 'complete', undef);
        write_raw($writer_fixture, 'generated/root-template.md', fixture_root());
        materialize_migration(
            $writer_fixture,
            'doctrine/live_document_size/active_task_evidence.json',
            'generated/root-template.md',
            1,
        );
        my @hygiene_errors;
        my ($writer_contract) = read_json_object(
            $writer_fixture,
            'doctrine/live_document_size/active_task_evidence.json',
            'migration writer contract',
            131_072,
            \@hygiene_errors,
        );
        die "active-task-evidence self-test 'migration writer positive' could not read its contract:\n"
            . join("\n", @hygiene_errors) . "\n"
            if @hygiene_errors || !defined $writer_contract;
        for my $part (@{$writer_contract->{destinations}{parts}}) {
            my $raw = read_regular(
                $writer_fixture,
                $part->{path},
                "migration writer part '$part->{part_id}'",
                \@hygiene_errors,
            );
            die "active-task-evidence self-test 'migration writer positive' could not read semantic parts:\n"
                . join("\n", @hygiene_errors) . "\n"
                if @hygiene_errors || !defined $raw;
            die "active-task-evidence self-test 'migration writer positive' left a blank line at the end of '$part->{path}'\n"
                if $raw =~ /\n\n\z/;
        }
        my ($writer_errors, $writer_result) = validate_tree(
            $writer_fixture, 'doctrine/live_document_size/active_task_evidence.json',
        );
        die "active-task-evidence self-test 'migration writer positive' failed:\n"
            . join("\n", @$writer_errors) . "\n"
            if @$writer_errors || ($writer_result->{migration_state} // '') ne 'migrated';
        1;
    } or $positive_failure = $@
        || "active-task-evidence self-test 'migration writer positive' failed without a diagnostic\n";
    remove_tree($writer_fixture);
    die $positive_failure if defined $positive_failure;
    $passed++;

    make_path($writer_fixture);
    my $refusal_failure;
    eval {
        seed_fixture($writer_fixture, 'source_locked', 'complete', undef);
        my $invalid_root = fixture_root();
        $invalid_root =~ s/No eligible frontier\./Unknown frontier./;
        write_raw($writer_fixture, 'generated/root-template.md', $invalid_root);
        my $writer_failure = eval {
            materialize_migration(
                $writer_fixture,
                'doctrine/live_document_size/active_task_evidence.json',
                'generated/root-template.md',
                1,
            );
            '';
        };
        $writer_failure = $@ if $@;
        die "active-task-evidence self-test 'migration writer preflight refusal' unexpectedly passed\n"
            if !$writer_failure;
        die "active-task-evidence self-test 'migration writer preflight refusal' missed diagnostic\n"
            if $writer_failure !~ /root template preflight failed/;
        my ($rollback_errors, $rollback_result) = validate_tree(
            $writer_fixture, 'doctrine/live_document_size/active_task_evidence.json',
        );
        die "active-task-evidence self-test 'migration writer preflight refusal' changed source state\n"
            if @$rollback_errors || ($rollback_result->{migration_state} // '') ne 'source_locked';
        1;
    } or $refusal_failure = $@
        || "active-task-evidence self-test 'migration writer preflight refusal' failed without a diagnostic\n";
    remove_tree($writer_fixture);
    die $refusal_failure if defined $refusal_failure;
    $passed++;

    make_path($writer_fixture);
    my $derived_failure;
    eval {
        seed_fixture($writer_fixture, 'migrated', 'complete', undef);
        write_raw($writer_fixture, 'docs/tasks/program/INDEX.md', "# PROGRAM task-evidence index\n");
        write_raw($writer_fixture, 'docs/tasks/program/routes-0007.md', "# stray\n");
        materialize_derived($writer_fixture, $DEFAULT_CONTRACT_REL, 1);
        die "active-task-evidence self-test 'derived writer positive' left a stray route catalog part\n"
            if -e File::Spec->catfile($writer_fixture, 'docs', 'tasks', 'program', 'routes-0007.md');
        my ($derived_errors) = validate_tree($writer_fixture, $DEFAULT_CONTRACT_REL);
        die "active-task-evidence self-test 'derived writer positive' failed:\n"
            . join("\n", @$derived_errors) . "\n"
            if @$derived_errors;
        my $preimage = read_regular($writer_fixture, 'docs/tasks/program/routes-0001.md', 'derived writer part', \my @unused);
        write_raw($writer_fixture, 'docs/tasks/program/foundation.md', "corrupted\n");
        my $refused = eval { materialize_derived($writer_fixture, $DEFAULT_CONTRACT_REL, 1); '' };
        $refused = $@ if $@;
        die "active-task-evidence self-test 'derived writer refusal' unexpectedly passed\n" if !$refused;
        die "active-task-evidence self-test 'derived writer refusal' missed diagnostic\n"
            if $refused !~ /writer: preflight failed/;
        my $after = read_regular($writer_fixture, 'docs/tasks/program/routes-0001.md', 'derived writer part', \@unused);
        die "active-task-evidence self-test 'derived writer refusal' changed a derived surface\n"
            if ($after // '') ne ($preimage // '');
        1;
    } or $derived_failure = $@
        || "active-task-evidence self-test 'derived writer positive' failed without a diagnostic\n";
    remove_tree($writer_fixture);
    die $derived_failure if defined $derived_failure;
    $passed += 2;
    my $expected = 61;
    # PRODUCTION-GRAPH-CENSUS-PIN.3 — `$passed/$passed` can only ever print N/N: delete a case
    # and both sides drop together. The expected total is declared here, independently of the
    # suite, so a case removed — or one added and not declared — fails instead of moving a
    # number nothing compares.
    die "active-task-evidence self-test ran $passed assertions, declaration expects $expected — "
        . "re-derive the declaration beside the suite\n"
        if $passed != $expected;
    print "active-task-evidence self-test: $passed/$expected source/topology/route/payload/bound cases pass.\n";
}
