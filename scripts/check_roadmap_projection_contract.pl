#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use Encode qw(encode_utf8);
use File::Basename qw(basename dirname);
use File::Spec;
use JSON::PP;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root;
my $contract_rel = 'doctrine/live_document_size/roadmap_projection.json';
my $report = 0;
my $self_test = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--contract') {
        $contract_rel = shift @ARGV // usage();
    } elsif ($arg eq '--check') {
        # Check is the default operation; retain the flag for a stable reverify command.
    } elsif ($arg eq '--report') {
        $report = 1;
    } elsif ($arg eq '--self-test') {
        $self_test = 1;
    } else {
        usage();
    }
}

run_self_test() if $self_test;
exit 0 if $self_test;

$root //= File::Spec->catdir(dirname(abs_path($0)), '..');
$root = abs_path($root) // die "roadmap-projection: repository root does not exist\n";

my @errors;
my $json = JSON::PP->new->canonical(1);
my $contract = read_contract();
my $report_record = {};
validate_contract($contract) if defined $contract;

if (@errors) {
    print STDERR "roadmap-projection: $_\n" for @errors;
    print STDERR "roadmap-projection: FAILED with ", scalar(@errors), " violation(s).\n";
    exit 1;
}

if ($report) {
    print $json->encode($report_record), "\n";
} else {
    print "roadmap-projection: source identity, 23-workstream coverage, reader contract, and "
        . "$contract->{migration_state} lifecycle satisfy the bounded current/history design.\n";
}
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--contract PATH] [--check|--report|--self-test]\n";
}

sub problem {
    my ($message) = @_;
    push @errors, $message;
}

sub absolute {
    my ($relative) = @_;
    return File::Spec->catfile($root, split m{/}, $relative);
}

sub safe_relative {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '';
    return 0 if $path =~ m{\A/} || $path =~ /\\/ || $path =~ /[\x00-\x1f\x7f]/;
    return 0 if $path =~ m{(?:\A|/)\.\.(?:/|\z)} || $path =~ m{//};
    return 0 if $path =~ m{(?:\A|/)\.(?:/|\z)};
    return 1;
}

sub slurp {
    my ($path, $label) = @_;
    open my $fh, '<:raw', $path or do {
        problem("cannot read $label: $!");
        return;
    };
    local $/;
    my $bytes = <$fh>;
    close $fh;
    return $bytes;
}

sub raw_literal {
    my ($value) = @_;
    return '' if !defined $value;
    return utf8::is_utf8($value) ? encode_utf8($value) : $value;
}

sub read_contract {
    problem("contract path '$contract_rel' is unsafe") if !safe_relative($contract_rel);
    return if !safe_relative($contract_rel);
    my $path = absolute($contract_rel);
    if (!-f $path) {
        problem("contract is missing: $contract_rel");
        return;
    }
    problem('contract exceeds the 65536-byte control-plane cap') if -s $path > 65_536;
    my $bytes = slurp($path, "contract '$contract_rel'");
    return if !defined $bytes;
    my $decoded = eval { decode_json($bytes) };
    if (!$decoded || ref($decoded) ne 'HASH') {
        problem("contract is not one JSON object: $@");
        return;
    }
    return $decoded;
}

sub reject_unknown {
    my ($object, $label, @allowed) = @_;
    if (ref($object) ne 'HASH') {
        problem("$label must be an object");
        return;
    }
    my %allowed = map { $_ => 1 } @allowed;
    problem("$label has unknown field '$_'") for grep { !$allowed{$_} } keys %$object;
}

sub required_string {
    my ($object, $field, $label) = @_;
    if (ref($object) ne 'HASH' || !defined($object->{$field}) || ref($object->{$field})
        || $object->{$field} eq '') {
        problem("$label lacks nonempty string '$field'");
        return;
    }
    return $object->{$field};
}

sub required_positive_integer {
    my ($object, $field, $label) = @_;
    if (ref($object) ne 'HASH' || !defined($object->{$field}) || ref($object->{$field})
        || $object->{$field} !~ /\A\d+\z/ || $object->{$field} < 1) {
        problem("$label lacks positive integer '$field'");
        return;
    }
    return 0 + $object->{$field};
}

sub require_array {
    my ($object, $field, $label) = @_;
    if (ref($object) ne 'HASH' || ref($object->{$field}) ne 'ARRAY') {
        problem("$label '$field' must be an array");
        return [];
    }
    return $object->{$field};
}

sub validate_contract {
    my ($c) = @_;
    reject_unknown($c, 'contract', qw(
      schema_version owner migration_state source layout drift_findings current_root archive consumers
      rollover_policy rollovers
    ));
    problem('contract schema_version must be 1')
        if !defined($c->{schema_version}) || ref($c->{schema_version}) || $c->{schema_version} != 1;
    required_string($c, 'owner', 'contract');
    my $state = required_string($c, 'migration_state', 'contract') // '';
    problem("unknown migration_state '$state'") if $state ne 'planned' && $state ne 'migrated';

    my $source = $c->{source};
    reject_unknown($source, 'source', qw(path sha256 lines bytes line_bytes regions));
    my $source_path = required_string($source, 'path', 'source') // '';
    validate_safe_markdown($source_path, 'source path');
    validate_hash(required_string($source, 'sha256', 'source'), 'source sha256');
    required_positive_integer($source, $_, 'source') for qw(lines bytes line_bytes);

    my $layout = $c->{layout};
    reject_unknown($layout, 'layout', qw(h1 h2_order workstream_ids workstream_owners));
    required_string($layout, 'h1', 'layout');
    my $h2_order = require_array($layout, 'h2_order', 'layout');
    my $ids = require_array($layout, 'workstream_ids', 'layout');
    my $owners = require_array($layout, 'workstream_owners', 'layout');
    validate_unique_strings($h2_order, 'layout h2_order');
    validate_workstream_ids($ids);
    validate_owners($ids, $owners);

    my $regions = require_array($source, 'regions', 'source');
    validate_regions_schema($regions, $source->{lines});
    validate_drift_findings($c->{drift_findings});
    validate_current_root_schema($c->{current_root});
    validate_archive_schema($c->{archive});
    validate_rollover_policy_schema($c->{rollover_policy});
    problem($_) for rollover_schema_errors(
        $c->{rollovers}, $state, $c->{rollover_policy}, $c->{archive}, $c->{current_root});
    validate_consumers($c->{consumers});

    if (safe_relative($source_path) && !-f absolute($source_path)) {
        problem("source path '$source_path' is missing");
        return;
    }
    return if !safe_relative($source_path);
    my $archive = $c->{archive};
    my $capsule = ref($archive) eq 'HASH' ? ($archive->{source_capsule} // '') : '';
    my $frozen_path = $state eq 'migrated' ? $capsule : $source_path;
    if (!safe_relative($frozen_path) || !-f absolute($frozen_path)) {
        problem("$state identity source '$frozen_path' is missing or unsafe");
        return;
    }
    my $frozen = slurp(absolute($frozen_path), "roadmap identity source '$frozen_path'");
    return if !defined $frozen;
    validate_exact_source($frozen, $source);
    problem($_) for source_layout_errors($frozen, $layout);
    validate_regions_content($frozen, $regions);
    validate_drift_evidence($frozen, $c->{drift_findings});

    my $done = done_block_metrics($frozen);
    $report_record = {
        migration_state => $state,
        source => { %{ metrics($frozen) }, sha256 => sha256_hex($frozen) },
        regions => $regions,
        workstreams => scalar(@$ids),
        done_blocks => $done,
        drift_findings => scalar(@{ ref($c->{drift_findings}) eq 'ARRAY' ? $c->{drift_findings} : [] }),
        current_root_limits => $c->{current_root}{enforcement_ceilings},
        archive => $c->{archive},
        rollovers => rollover_summary($c->{rollovers}),
    };

    if ($state eq 'migrated') {
        my $live = slurp(absolute($source_path), "current roadmap '$source_path'");
        if (defined $live) {
            problem($_) for live_root_errors($live, $c->{current_root}, $layout, $owners);
            problem($_) for section_limit_errors($live, $c->{current_root});
            print STDERR "roadmap-projection: WARNING $_\n"
                for section_pressure_warnings($live, $c->{current_root});
            $report_record->{current_root} = metrics($live);
            $report_record->{sections} = [ section_line_counts($live) ];
        }
        validate_archive($c, $frozen);
        validate_rollovers($c);
    }
}

sub validate_safe_markdown {
    my ($path, $label) = @_;
    problem("$label '$path' is not a safe repository-relative Markdown path")
        if !safe_relative($path) || $path !~ /\.md\z/;
}

sub validate_hash {
    my ($hash, $label) = @_;
    problem("$label must be a lowercase SHA-256")
        if !defined($hash) || ref($hash) || $hash !~ /\A[0-9a-f]{64}\z/;
}

sub validate_unique_strings {
    my ($items, $label) = @_;
    my %seen;
    for my $value (@$items) {
        problem("$label contains a non-string value") if !defined($value) || ref($value) || $value eq '';
        next if !defined($value) || ref($value);
        problem("$label contains duplicate '$value'") if $seen{$value}++;
    }
}

sub validate_workstream_ids {
    my ($ids) = @_;
    validate_unique_strings($ids, 'layout workstream_ids');
    problem('layout must contain exactly 23 roadmap workstream ids') if @$ids != 23;
    problem("invalid roadmap workstream id '$_'")
        for grep { !defined($_) || ref($_) || $_ !~ /\AR(?:\d+|15[b-g])\z/ } @$ids;
}

sub validate_owners {
    my ($ids, $owners) = @_;
    problem('workstream_owners count must equal workstream_ids count') if @$owners != @$ids;
    my $catalog_path = absolute('docs/TASK_TREE.md');
    my $catalog = -f $catalog_path ? slurp($catalog_path, 'task catalog') : undef;
    for my $index (0 .. $#$owners) {
        my $owner = $owners->[$index];
        reject_unknown($owner, "workstream owner $index", qw(id initial_status path));
        next if ref($owner) ne 'HASH';
        my $id = required_string($owner, 'id', "workstream owner $index") // '';
        required_string($owner, 'initial_status', "workstream owner $index");
        my $path = required_string($owner, 'path', "workstream owner $index") // '';
        problem("workstream owner order mismatch at $index: expected '$ids->[$index]', found '$id'")
            if defined($ids->[$index]) && $id ne $ids->[$index];
        validate_safe_markdown($path, "owner path for '$id'");
        problem("owner path '$path' is missing") if safe_relative($path) && !-f absolute($path);
        if (defined($catalog) && $path =~ m{\Adocs/tasks/(.+)\z}) {
            problem("owner path '$path' is absent from the complete task catalog")
                if index($catalog, "(tasks/$1)") < 0;
        }
    }
}

sub validate_regions_schema {
    my ($regions, $source_lines) = @_;
    my $next_line = 1;
    my %seen;
    for my $region (@$regions) {
        reject_unknown($region, 'source region', qw(id start_line end_line lines bytes line_bytes sha256));
        next if ref($region) ne 'HASH';
        my $id = required_string($region, 'id', 'source region') // '';
        problem("duplicate source region '$id'") if $seen{$id}++;
        my $start = required_positive_integer($region, 'start_line', "source region '$id'");
        my $end = required_positive_integer($region, 'end_line', "source region '$id'");
        my $lines = required_positive_integer($region, 'lines', "source region '$id'");
        required_positive_integer($region, $_, "source region '$id'") for qw(bytes line_bytes);
        validate_hash(required_string($region, 'sha256', "source region '$id'"), "source region '$id' sha256");
        next if !defined($start) || !defined($end) || !defined($lines);
        problem("source region '$id' must start at line $next_line") if $start != $next_line;
        problem("source region '$id' line count does not match its inclusive bounds")
            if $end < $start || $lines != $end - $start + 1;
        $next_line = $end + 1;
    }
    problem('source regions must partition every source line exactly once')
        if defined($source_lines) && !ref($source_lines) && $next_line != $source_lines + 1;
}

sub validate_drift_findings {
    my ($findings) = @_;
    if (ref($findings) ne 'ARRAY') {
        problem('drift_findings must be an array');
        return;
    }
    my %seen;
    for my $finding (@$findings) {
        reject_unknown($finding, 'drift finding', qw(id source_literal evidence_path evidence_literal));
        next if ref($finding) ne 'HASH';
        my $id = required_string($finding, 'id', 'drift finding') // '';
        problem("duplicate drift finding '$id'") if $seen{$id}++;
        required_string($finding, $_, "drift finding '$id'") for qw(source_literal evidence_path evidence_literal);
        validate_safe_markdown($finding->{evidence_path} // '', "drift evidence path for '$id'");
    }
}

sub validate_current_root_schema {
    my ($current) = @_;
    reject_unknown($current, 'current_root', qw(
      required_h2_order workstream_start_marker workstream_end_marker required_links forbidden_literals
      health_targets enforcement_ceilings section_limits
    ));
    return if ref($current) ne 'HASH';
    validate_unique_strings(require_array($current, 'required_h2_order', 'current_root'), 'current_root h2 order');
    validate_unique_strings(require_array($current, 'required_links', 'current_root'), 'current_root required links');
    validate_unique_strings(require_array($current, 'forbidden_literals', 'current_root'), 'current_root forbidden literals');
    required_string($current, $_, 'current_root') for qw(workstream_start_marker workstream_end_marker);
    for my $kind (qw(health_targets enforcement_ceilings)) {
        my $limits = $current->{$kind};
        reject_unknown($limits, "current_root $kind", qw(lines bytes line_bytes));
        required_positive_integer($limits, $_, "current_root $kind") for qw(lines bytes line_bytes);
    }
    if (ref($current->{health_targets}) eq 'HASH' && ref($current->{enforcement_ceilings}) eq 'HASH') {
        for my $axis (qw(lines bytes line_bytes)) {
            next if !defined($current->{health_targets}{$axis}) || !defined($current->{enforcement_ceilings}{$axis});
            problem("current_root health $axis exceeds enforcement ceiling")
                if $current->{health_targets}{$axis} > $current->{enforcement_ceilings}{$axis};
        }
    }
    problem($_) for section_limit_schema_errors($current);
}

# A per-section bound is what makes the file bound survivable: the root's last overflow was one section
# growing by a sentence per closed leaf, and a file-level ceiling can only report that after the fact and
# without naming the cause. The declared sections must therefore be exactly the required H2 set, and no
# legal combination of them may exceed the reviewed working set (ADR 0029's rule applied inward).
sub section_limit_schema_errors {
    my ($current) = @_;
    my @found;
    return @found if ref($current) ne 'HASH';
    my $limits = $current->{section_limits};
    if (ref($limits) ne 'HASH') {
        push @found, 'current_root section_limits must be an object';
        return @found;
    }
    my %allowed = map { $_ => 1 } qw(warning_pct sections);
    push @found, "current_root section_limits has unknown field '$_'"
        for sort grep { !$allowed{$_} } keys %$limits;
    my $warning = $limits->{warning_pct};
    push @found, 'current_root section_limits warning_pct must be an integer percentage below 100'
        if !defined($warning) || ref($warning) || $warning !~ /\A\d+\z/ || $warning < 1 || $warning >= 100;
    my $sections = $limits->{sections};
    if (ref($sections) ne 'ARRAY' || !@$sections) {
        push @found, 'current_root section_limits sections must be a nonempty array';
        return @found;
    }
    my @expected = @{ ref($current->{required_h2_order}) eq 'ARRAY' ? $current->{required_h2_order} : [] };
    my $total = 0;
    for my $index (0 .. $#$sections) {
        my $section = $sections->[$index];
        if (ref($section) ne 'HASH') {
            push @found, "current_root section $index must be an object";
            next;
        }
        my %section_allowed = map { $_ => 1 } qw(heading max_lines remedy);
        push @found, "current_root section $index has unknown field '$_'"
            for sort grep { !$section_allowed{$_} } keys %$section;
        my $heading = $section->{heading};
        if (!defined($heading) || ref($heading) || $heading eq '') {
            push @found, "current_root section $index lacks a heading";
            $heading = "index $index";
        }
        my $max = $section->{max_lines};
        if (!defined($max) || ref($max) || $max !~ /\A\d+\z/ || $max < 1) {
            push @found, "current_root section '$heading' lacks a positive 'max_lines'";
        } else {
            $total += $max;
        }
        push @found, "current_root section '$heading' lacks a nonempty 'remedy'"
            if !defined($section->{remedy}) || ref($section->{remedy}) || $section->{remedy} eq '';
    }
    my @declared = map { ref($_) eq 'HASH' ? ($_->{heading} // '') : '' } @$sections;
    push @found, 'current_root section_limits must declare exactly the required H2 sections in order'
        if join("\0", @declared) ne join("\0", @expected);
    my $health = ref($current->{health_targets}) eq 'HASH' ? $current->{health_targets}{lines} : undef;
    if (defined($health) && !ref($health) && $health =~ /\A\d+\z/) {
        my $largest = $total + scalar(@$sections);
        push @found, "current_root sections sum to $largest legal lines above the $health-line health target"
            if $largest > $health;
    }
    return @found;
}

# Section lines are the heading plus its body up to the next H2, with trailing blank separators trimmed so
# the count does not depend on how many blank lines an author left behind.
sub section_line_counts {
    my ($bytes) = @_;
    my @counts;
    my ($heading, @body);
    my $flush = sub {
        return if !defined $heading;
        pop @body while @body && $body[-1] =~ /\A\s*\z/;
        push @counts, { heading => $heading, lines => scalar(@body) + 1 };
        @body = ();
    };
    for my $line (physical_lines($bytes)) {
        if ($line =~ /\A## (?!\#)([^\r\n]+)\r?\n?\z/) {
            $flush->();
            $heading = $1;
            next;
        }
        push @body, $line if defined $heading;
    }
    $flush->();
    return @counts;
}

sub section_limit_errors {
    my ($bytes, $current) = @_;
    my @found;
    my $limits = ref($current) eq 'HASH' ? $current->{section_limits} : undef;
    return @found if ref($limits) ne 'HASH' || ref($limits->{sections}) ne 'ARRAY';
    my @measured = section_line_counts($bytes);
    my %bound = map { ref($_) eq 'HASH' ? ($_->{heading} // '' => $_) : () } @{ $limits->{sections} };
    for my $section (@measured) {
        my $declared = $bound{ $section->{heading} };
        if (!$declared) {
            push @found, "current roadmap section '$section->{heading}' has no declared line bound";
            next;
        }
        my $max = $declared->{max_lines};
        next if !defined($max) || ref($max) || $max !~ /\A\d+\z/;
        push @found, "current roadmap section '$section->{heading}' is $section->{lines} lines over its "
            . "$max-line bound; remedy: " . ($declared->{remedy} // 'undeclared')
            if $section->{lines} > $max;
    }
    return @found;
}

sub section_pressure_warnings {
    my ($bytes, $current) = @_;
    my @found;
    my $limits = ref($current) eq 'HASH' ? $current->{section_limits} : undef;
    return @found if ref($limits) ne 'HASH' || ref($limits->{sections}) ne 'ARRAY';
    my $warning = $limits->{warning_pct};
    return @found if !defined($warning) || ref($warning) || $warning !~ /\A\d+\z/;
    my %bound = map { ref($_) eq 'HASH' ? ($_->{heading} // '' => $_) : () } @{ $limits->{sections} };
    for my $section (section_line_counts($bytes)) {
        my $declared = $bound{ $section->{heading} } or next;
        my $max = $declared->{max_lines};
        next if !defined($max) || ref($max) || $max !~ /\A\d+\z/ || $max == 0;
        my $percent = 100 * $section->{lines} / $max;
        next if $percent < $warning || $section->{lines} > $max;
        push @found, sprintf("section '%s' is at %.1f%% of its %d-line bound; remedy: %s",
            $section->{heading}, $percent, $max, $declared->{remedy} // 'undeclared');
    }
    return @found;
}

sub validate_archive_schema {
    my ($archive) = @_;
    reject_unknown($archive, 'archive', qw(
      index manifest source_capsule sealed_date reason verifier index_limits
    ));
    return if ref($archive) ne 'HASH';
    for my $field (qw(index source_capsule)) {
        my $path = required_string($archive, $field, 'archive') // '';
        validate_safe_markdown($path, "archive $field");
    }
    my $manifest = required_string($archive, 'manifest', 'archive') // '';
    problem("archive manifest '$manifest' must be a safe repository-relative JSON path")
        if !safe_relative($manifest) || $manifest !~ /\.json\z/;
    required_string($archive, $_, 'archive') for qw(sealed_date reason verifier);
    my $limits = $archive->{index_limits};
    reject_unknown($limits, 'archive index_limits', qw(lines bytes line_bytes));
    required_positive_integer($limits, $_, 'archive index_limits') for qw(lines bytes line_bytes);
}

sub validate_rollover_policy_schema {
    my ($policy) = @_;
    reject_unknown($policy, 'rollover_policy', qw(max_capsules warning_capsules remedy));
    return if ref($policy) ne 'HASH';
    my $max = required_positive_integer($policy, 'max_capsules', 'rollover_policy');
    my $warning = required_positive_integer($policy, 'warning_capsules', 'rollover_policy');
    required_string($policy, 'remedy', 'rollover_policy');
    problem('rollover_policy warning_capsules must stay below max_capsules')
        if defined($max) && defined($warning) && $warning >= $max;
}

# Pure schema rules for the sealed-rollover series. A rollover retires the bounded current root, so
# every capsule must have been legal when it was sealed: an entry above the current-root ceiling would
# mean the surface breached its own bound before the rollover, not that the rollover recovered from it.
sub rollover_schema_errors {
    my ($rollovers, $state, $policy, $archive, $current) = @_;
    my @found;
    if (ref($rollovers) ne 'ARRAY') {
        push @found, 'rollovers must be an array';
        return @found;
    }
    my $max = ref($policy) eq 'HASH' ? $policy->{max_capsules} : undef;
    $max = 16 if !defined($max) || ref($max) || $max !~ /\A\d+\z/;
    my $pre_migration = ref($archive) eq 'HASH' ? ($archive->{source_capsule} // '') : '';
    my $ceilings = ref($current) eq 'HASH' ? $current->{enforcement_ceilings} : undef;
    push @found, 'planned migration_state may not declare a sealed rollover'
        if $state ne 'migrated' && @$rollovers;
    push @found, "rollovers exceed the declared maximum of $max capsules" if @$rollovers > $max;
    my (%seen_id, %seen_capsule);
    my $previous_date = '';
    for my $index (0 .. $#$rollovers) {
        my $record = $rollovers->[$index];
        if (ref($record) ne 'HASH') {
            push @found, "rollover $index must be an object";
            next;
        }
        my %allowed = map { $_ => 1 } qw(id capsule sha256 lines bytes line_bytes sealed_date reason);
        push @found, "rollover $index has unknown field '$_'"
            for sort grep { !$allowed{$_} } keys %$record;
        my $id = $record->{id};
        if (!defined($id) || ref($id) || $id !~ /\A[a-z0-9][a-z0-9-]*\z/) {
            push @found, "rollover $index lacks a lowercase identifier";
            $id = "index $index";
        } else {
            push @found, "duplicate rollover id '$id'" if $seen_id{$id}++;
        }
        my $capsule = $record->{capsule} // '';
        if (!safe_relative($capsule) || $capsule !~ /\.md\z/) {
            push @found, "rollover '$id' capsule is not a safe repository-relative Markdown path";
        } else {
            push @found, "duplicate rollover capsule '$capsule'" if $seen_capsule{$capsule}++;
            push @found, "rollover '$id' may not reuse the pre-migration source capsule"
                if $pre_migration ne '' && $capsule eq $pre_migration;
        }
        push @found, "rollover '$id' sha256 must be a lowercase SHA-256"
            if !defined($record->{sha256}) || ref($record->{sha256}) || $record->{sha256} !~ /\A[0-9a-f]{64}\z/;
        for my $axis (qw(lines bytes line_bytes)) {
            my $value = $record->{$axis};
            if (!defined($value) || ref($value) || $value !~ /\A\d+\z/ || $value < 1) {
                push @found, "rollover '$id' lacks positive integer '$axis'";
                next;
            }
            next if ref($ceilings) ne 'HASH' || !defined($ceilings->{$axis});
            push @found, "rollover '$id' $axis exceeds the current-root enforcement ceiling"
                if $value > $ceilings->{$axis};
        }
        my $date = $record->{sealed_date} // '';
        if (ref($date) || $date !~ /\A\d{4}-\d{2}-\d{2}\z/) {
            push @found, "rollover '$id' lacks an ISO sealed_date";
        } else {
            push @found, "rollover '$id' seals before its predecessor" if $date lt $previous_date;
            $previous_date = $date;
        }
        push @found, "rollover '$id' lacks a nonempty 'reason'"
            if !defined($record->{reason}) || ref($record->{reason}) || $record->{reason} eq '';
    }
    return @found;
}

# Pure identity rules: each declared capsule must exist byte-for-byte as declared and be reachable
# from the bounded archive index. $reader maps a repository-relative path to raw bytes or undef.
sub rollover_content_errors {
    my ($rollovers, $reader, $index_bytes) = @_;
    my @found;
    return () if ref($rollovers) ne 'ARRAY';
    for my $record (@$rollovers) {
        next if ref($record) ne 'HASH';
        my $id = $record->{id} // '?';
        my $capsule = $record->{capsule} // '';
        next if !safe_relative($capsule);
        my $bytes = $reader->($capsule);
        if (!defined $bytes) {
            push @found, "rollover '$id' capsule '$capsule' is missing";
            next;
        }
        my $actual = metrics($bytes);
        for my $axis (qw(lines bytes line_bytes)) {
            push @found, "rollover '$id' $axis mismatch: expected $record->{$axis}, found $actual->{$axis}"
                if defined($record->{$axis}) && $actual->{$axis} != $record->{$axis};
        }
        push @found, "rollover '$id' capsule bytes do not match its sealed SHA-256"
            if sha256_hex($bytes) ne ($record->{sha256} // '');
        push @found, "rollover '$id' capsule is not reachable from the roadmap archive index"
            if defined($index_bytes) && index($index_bytes, basename($capsule)) < 0;
    }
    return @found;
}

sub rollover_summary {
    my ($rollovers) = @_;
    return [] if ref($rollovers) ne 'ARRAY';
    return [ map { ref($_) eq 'HASH'
        ? { id => $_->{id}, sealed_date => $_->{sealed_date}, lines => $_->{lines}, bytes => $_->{bytes} }
        : {} } @$rollovers ];
}

sub validate_rollovers {
    my ($c) = @_;
    my $rollovers = $c->{rollovers};
    return if ref($rollovers) ne 'ARRAY';
    my $index_rel = ref($c->{archive}) eq 'HASH' ? ($c->{archive}{index} // '') : '';
    my $index_bytes;
    $index_bytes = slurp(absolute($index_rel), "roadmap archive index '$index_rel'")
        if safe_relative($index_rel) && -f absolute($index_rel);
    my $reader = sub {
        my ($relative) = @_;
        my $path = absolute($relative);
        return undef if !-f $path;
        open my $fh, '<:raw', $path or return undef;
        local $/;
        my $bytes = <$fh>;
        close $fh;
        return $bytes;
    };
    problem($_) for rollover_content_errors($rollovers, $reader, $index_bytes);

    my $policy = $c->{rollover_policy};
    return if ref($policy) ne 'HASH';
    my $warning = $policy->{warning_capsules};
    return if !defined($warning) || ref($warning) || $warning !~ /\A\d+\z/;
    return if @$rollovers < $warning;
    printf STDERR "roadmap-projection: WARNING sealed rollovers are at %d of %s capsules; remedy: %s\n",
        scalar(@$rollovers), $policy->{max_capsules} // '?', $policy->{remedy} // 'undeclared';
}

sub validate_consumers {
    my ($consumers) = @_;
    reject_unknown($consumers, 'consumers', qw(readers writers required_literals));
    return if ref($consumers) ne 'HASH';
    for my $field (qw(readers writers)) {
        my $paths = require_array($consumers, $field, 'consumers');
        validate_unique_strings($paths, "consumers $field");
        for my $path (@$paths) {
            next if !defined($path) || ref($path);
            problem("consumer path '$path' is unsafe") if !safe_relative($path);
            problem("consumer path '$path' is missing") if safe_relative($path) && !-f absolute($path);
        }
    }
    my $literals = require_array($consumers, 'required_literals', 'consumers');
    for my $entry (@$literals) {
        reject_unknown($entry, 'consumer literal', qw(path literal));
        next if ref($entry) ne 'HASH';
        my $path = required_string($entry, 'path', 'consumer literal') // '';
        my $literal = required_string($entry, 'literal', "consumer literal '$path'");
        if (!safe_relative($path) || !-f absolute($path)) {
            problem("consumer literal path '$path' is missing or unsafe");
            next;
        }
        my $bytes = slurp(absolute($path), "consumer '$path'");
        problem("consumer '$path' lacks required literal '$literal'")
            if defined($bytes) && defined($literal) && index($bytes, raw_literal($literal)) < 0;
    }
}

sub validate_exact_source {
    my ($bytes, $source) = @_;
    my $actual = metrics($bytes);
    for my $axis (qw(lines bytes line_bytes)) {
        problem("source $axis mismatch: expected $source->{$axis}, found $actual->{$axis}")
            if defined($source->{$axis}) && $actual->{$axis} != $source->{$axis};
    }
    problem('source SHA-256 does not match the pinned pre-migration identity')
        if sha256_hex($bytes) ne ($source->{sha256} // '');
}

sub validate_regions_content {
    my ($bytes, $regions) = @_;
    my @lines = physical_lines($bytes);
    for my $region (@$regions) {
        next if ref($region) ne 'HASH';
        my $start = $region->{start_line};
        my $end = $region->{end_line};
        next if !defined($start) || !defined($end) || $start < 1 || $end > @lines || $end < $start;
        my $slice = join('', @lines[$start - 1 .. $end - 1]);
        my $actual = metrics($slice);
        my $id = $region->{id} // '?';
        for my $axis (qw(lines bytes line_bytes)) {
            problem("source region '$id' $axis mismatch")
                if defined($region->{$axis}) && $actual->{$axis} != $region->{$axis};
        }
        problem("source region '$id' SHA-256 mismatch")
            if sha256_hex($slice) ne ($region->{sha256} // '');
    }
}

sub validate_drift_evidence {
    my ($source, $findings) = @_;
    return if ref($findings) ne 'ARRAY';
    for my $finding (@$findings) {
        next if ref($finding) ne 'HASH';
        my $id = $finding->{id} // '?';
        my $literal = $finding->{source_literal} // '';
        problem("drift finding '$id' is no longer present in the pinned source")
            if $literal ne '' && index($source, raw_literal($literal)) < 0;
        my $path = $finding->{evidence_path} // '';
        next if !safe_relative($path) || !-f absolute($path);
        my $evidence = slurp(absolute($path), "drift evidence '$path'");
        my $expected = $finding->{evidence_literal} // '';
        problem("drift finding '$id' lacks its task-tree evidence literal")
            if defined($evidence) && $expected ne '' && index($evidence, raw_literal($expected)) < 0;
    }
}

sub source_layout_errors {
    my ($bytes, $layout) = @_;
    my @found_errors;
    return ('layout must be an object') if ref($layout) ne 'HASH';
    my ($first_line) = $bytes =~ /\A([^\r\n]*)/;
    push @found_errors, "source H1 mismatch" if ($first_line // '') ne ($layout->{h1} // '');
    my @h2 = $bytes =~ /^## (?!#)([^\r\n]+)\r?$/mg;
    my @expected_h2 = @{ ref($layout->{h2_order}) eq 'ARRAY' ? $layout->{h2_order} : [] };
    push @found_errors, 'source H2 order/membership mismatch' if join("\0", @h2) ne join("\0", @expected_h2);
    my $records = workstream_records($bytes);
    my @ids = map { $_->{id} } @$records;
    my @expected_ids = @{ ref($layout->{workstream_ids}) eq 'ARRAY' ? $layout->{workstream_ids} : [] };
    push @found_errors, 'source workstream order/membership mismatch'
        if join("\0", @ids) ne join("\0", @expected_ids);
    for my $record (@$records) {
        my $count = () = $record->{bytes} =~ /^- status:/mg;
        push @found_errors, "workstream '$record->{id}' must contain exactly one top-level status"
            if $count != 1;
    }
    return @found_errors;
}

sub workstream_records {
    my ($bytes) = @_;
    my @headings;
    while ($bytes =~ /^(#{2,3}) ([^\r\n]+)\r?$/mg) {
        push @headings, { level => length($1), title => $2, offset => $-[0] };
    }
    my @records;
    for my $index (0 .. $#headings) {
        my $heading = $headings[$index];
        next if $heading->{level} != 3 || $heading->{title} !~ /\A(R(?:\d+|15[b-g]))(?:\s|\z)/;
        my $end = $index == $#headings ? length($bytes) : $headings[$index + 1]{offset};
        push @records, {
            id => $1,
            bytes => substr($bytes, $heading->{offset}, $end - $heading->{offset}),
        };
    }
    return \@records;
}

sub live_root_errors {
    my ($bytes, $current, $layout, $owners) = @_;
    my @found_errors;
    return ('current_root must be an object') if ref($current) ne 'HASH';
    my $actual = metrics($bytes);
    my $ceilings = $current->{enforcement_ceilings};
    if (ref($ceilings) eq 'HASH') {
        for my $axis (qw(lines bytes line_bytes)) {
            push @found_errors, "current roadmap exceeds $axis ceiling"
                if defined($ceilings->{$axis}) && $actual->{$axis} > $ceilings->{$axis};
        }
    }
    my ($first_line) = $bytes =~ /\A([^\r\n]*)/;
    push @found_errors, 'current roadmap H1 mismatch' if ($first_line // '') ne ($layout->{h1} // '');
    my @h2 = $bytes =~ /^## (?!#)([^\r\n]+)\r?$/mg;
    my @expected_h2 = @{ ref($current->{required_h2_order}) eq 'ARRAY' ? $current->{required_h2_order} : [] };
    push @found_errors, 'current roadmap H2 order/membership mismatch'
        if join("\0", @h2) ne join("\0", @expected_h2);
    for my $literal (@{ ref($current->{required_links}) eq 'ARRAY' ? $current->{required_links} : [] }) {
        push @found_errors, "current roadmap lacks required route '$literal'" if index($bytes, $literal) < 0;
    }
    for my $literal (@{ ref($current->{forbidden_literals}) eq 'ARRAY' ? $current->{forbidden_literals} : [] }) {
        push @found_errors, "current roadmap contains forbidden chronology literal '$literal'"
            if index($bytes, $literal) >= 0;
    }
    my $start = $current->{workstream_start_marker} // '';
    my $end = $current->{workstream_end_marker} // '';
    my $start_at = index($bytes, $start);
    my $end_at = index($bytes, $end);
    push @found_errors, 'current roadmap workstream markers are missing or out of order'
        if $start eq '' || $end eq '' || $start_at < 0 || $end_at <= $start_at;
    push @found_errors, 'current roadmap workstream start marker is not unique'
        if $start_at >= 0 && index($bytes, $start, $start_at + length($start)) >= 0;
    push @found_errors, 'current roadmap workstream end marker is not unique'
        if $end_at >= 0 && index($bytes, $end, $end_at + length($end)) >= 0;
    if ($start_at >= 0 && $end_at > $start_at) {
        my $region = substr($bytes, $start_at + length($start), $end_at - $start_at - length($start));
        my @rows;
        my %row_text;
        while ($region =~ /^\|\s*(R(?:\d+|15[b-g]))\s*\|([^\r\n]*)$/mg) {
            push @rows, $1;
            $row_text{$1} = $2 if !exists $row_text{$1};
        }
        my @expected = @{ ref($layout->{workstream_ids}) eq 'ARRAY' ? $layout->{workstream_ids} : [] };
        push @found_errors, 'current roadmap workstream table order/membership mismatch'
            if join("\0", @rows) ne join("\0", @expected);
        for my $owner (@{ ref($owners) eq 'ARRAY' ? $owners : [] }) {
            next if ref($owner) ne 'HASH';
            my $id = $owner->{id} // '';
            my $status = $owner->{initial_status} // '';
            my $path = $owner->{path} // '';
            push @found_errors, "current roadmap workstream '$id' lacks initial status '$status'"
                if !exists($row_text{$id}) || $row_text{$id} !~ /^\s*\Q$status\E\s*\|/;
            push @found_errors, "current roadmap workstream '$id' lacks its owning task-tree route"
                if !exists($row_text{$id}) || index($row_text{$id}, "($path)") < 0;
        }
    }
    return @found_errors;
}

sub validate_archive {
    my ($c, $frozen) = @_;
    my $archive = $c->{archive};
    return if ref($archive) ne 'HASH';
    my $index_rel = $archive->{index} // '';
    my $manifest_rel = $archive->{manifest} // '';
    for my $path ($index_rel, $manifest_rel) {
        problem("migrated archive path '$path' is missing or unsafe")
            if !safe_relative($path) || !-f absolute($path);
    }
    return if !safe_relative($index_rel) || !-f absolute($index_rel)
        || !safe_relative($manifest_rel) || !-f absolute($manifest_rel);

    my $index = slurp(absolute($index_rel), "roadmap archive index '$index_rel'");
    if (defined $index) {
        my $actual = metrics($index);
        my $limits = $archive->{index_limits};
        if (ref($limits) eq 'HASH') {
            for my $axis (qw(lines bytes line_bytes)) {
                problem("roadmap archive index exceeds $axis limit")
                    if defined($limits->{$axis}) && $actual->{$axis} > $limits->{$axis};
            }
        }
        my $capsule_name = basename($archive->{source_capsule} // '');
        problem('roadmap archive index lacks the exact source capsule route')
            if $capsule_name eq '' || index($index, $capsule_name) < 0;
        problem('roadmap archive index lacks the current ROADMAP.md route') if index($index, 'ROADMAP.md') < 0;
    }

    my $manifest_bytes = slurp(absolute($manifest_rel), "roadmap archive manifest '$manifest_rel'");
    return if !defined $manifest_bytes;
    problem('roadmap archive manifest exceeds 8192 bytes') if length($manifest_bytes) > 8_192;
    my $manifest = eval { decode_json($manifest_bytes) };
    if (!$manifest || ref($manifest) ne 'HASH') {
        problem("roadmap archive manifest is not one JSON object: $@");
        return;
    }
    reject_unknown($manifest, 'roadmap archive manifest', qw(
      schema_version current_path source_capsule source_sha256 source_metrics sealed_date reason verifier
    ));
    my $source = $c->{source};
    my %expected = (
        schema_version => 1,
        current_path => $source->{path},
        source_capsule => $archive->{source_capsule},
        source_sha256 => $source->{sha256},
        sealed_date => $archive->{sealed_date},
        reason => $archive->{reason},
        verifier => $archive->{verifier},
    );
    for my $field (keys %expected) {
        problem("roadmap archive manifest '$field' mismatch")
            if !defined($manifest->{$field}) || ref($manifest->{$field}) || $manifest->{$field} ne "$expected{$field}";
    }
    if (ref($manifest->{source_metrics}) ne 'HASH') {
        problem('roadmap archive manifest source_metrics must be an object');
    } else {
        reject_unknown($manifest->{source_metrics}, 'roadmap archive source_metrics', qw(lines bytes line_bytes));
        for my $axis (qw(lines bytes line_bytes)) {
            problem("roadmap archive manifest source_metrics.$axis mismatch")
                if !defined($manifest->{source_metrics}{$axis})
                || ref($manifest->{source_metrics}{$axis})
                || $manifest->{source_metrics}{$axis} != $source->{$axis};
        }
    }
    problem('roadmap source capsule bytes do not match the manifest identity')
        if sha256_hex($frozen) ne ($manifest->{source_sha256} // '');
}

sub metrics {
    my ($bytes) = @_;
    my $lines = () = $bytes =~ /\n/g;
    $lines++ if length($bytes) && $bytes !~ /\n\z/;
    my $line_bytes = 0;
    for my $line (split /\n/, $bytes, -1) {
        $line =~ s/\r\z//;
        $line_bytes = length($line) if length($line) > $line_bytes;
    }
    return { lines => $lines, bytes => length($bytes), line_bytes => $line_bytes };
}

sub physical_lines {
    my ($bytes) = @_;
    my @lines = $bytes =~ /.*?(?:\n|\z)/gs;
    pop @lines if @lines && $lines[-1] eq '';
    return @lines;
}

sub done_block_metrics {
    my ($bytes) = @_;
    my $inside_workstreams = 0;
    my $block = '';
    my $done_bytes = '';
    for my $line (physical_lines($bytes)) {
        $inside_workstreams = 1 if $line =~ /^### R0(?:\s|\r?$)/;
        last if $line =~ /^### Applied task trees since/;
        next if !$inside_workstreams;
        $block = '' if $line =~ /^### /;
        if ($line =~ /^- ([^:\r\n]+):/) {
            $block = $1;
        }
        $done_bytes .= $line if $block eq 'done';
    }
    return metrics($done_bytes);
}

sub run_self_test {
    my $layout = {
        h1 => '# ROADMAP',
        h2_order => ['Objective', 'Major workstreams'],
        workstream_ids => ['R0', 'R1'],
    };
    my $valid_source = <<'SOURCE';
# ROADMAP
## Objective
- current
## Major workstreams
### R0 Bootstrap
- status: Done
### R1 Pipeline
- status: Active
SOURCE
    my $owners = [
        { id => 'R0', initial_status => 'Done', path => 'docs/tasks/R0.md' },
        { id => 'R1', initial_status => 'Active', path => 'docs/tasks/R1.md' },
    ];
    my $current = {
        required_h2_order => ['Objective', 'Workstream status', 'History and execution'],
        workstream_start_marker => '<!-- roadmap_workstreams:start -->',
        workstream_end_marker => '<!-- roadmap_workstreams:end -->',
        required_links => ['docs/TASK_TREE.md', 'docs/archive/roadmap/INDEX.md', 'CHANGES.md'],
        forbidden_literals => ['- done:'],
        enforcement_ceilings => { lines => 30, bytes => 4096, line_bytes => 256 },
    };
    my $valid_live = <<'LIVE';
# ROADMAP
## Objective
- current
## Workstream status
<!-- roadmap_workstreams:start -->
| Workstream | Status | Owner |
| --- | --- | --- |
| R0 | Done | [owner](docs/tasks/R0.md) |
| R1 | Active | [owner](docs/tasks/R1.md) |
<!-- roadmap_workstreams:end -->
## History and execution
- docs/TASK_TREE.md
- docs/archive/roadmap/INDEX.md
- CHANGES.md
LIVE
    my @cases;
    push @cases, ['valid source layout', !source_layout_errors($valid_source, $layout)];
    (my $duplicate = $valid_source) =~ s/### R1 Pipeline/### R0 Pipeline/;
    push @cases, ['duplicate source workstream rejected', scalar(source_layout_errors($duplicate, $layout)) > 0];
    (my $reordered = $valid_source) =~ s/### R0 Bootstrap\n- status: Done\n### R1 Pipeline\n- status: Active/### R1 Pipeline\n- status: Active\n### R0 Bootstrap\n- status: Done/;
    push @cases, ['reordered source workstream rejected', scalar(source_layout_errors($reordered, $layout)) > 0];
    (my $missing_status = $valid_source) =~ s/- status: Active\n//;
    push @cases, ['missing source status rejected', scalar(source_layout_errors($missing_status, $layout)) > 0];
    push @cases, ['valid bounded current root', !live_root_errors($valid_live, $current, $layout, $owners)];
    (my $missing_row = $valid_live) =~ s/^\| R1 .*\n//m;
    push @cases, ['missing current workstream rejected', scalar(live_root_errors($missing_row, $current, $layout, $owners)) > 0];
    (my $duplicate_row = $valid_live) =~ s/(^\| R1 .*\n)/$1$1/m;
    push @cases, ['duplicate current workstream rejected', scalar(live_root_errors($duplicate_row, $current, $layout, $owners)) > 0];
    (my $chronology = $valid_live) =~ s/## History and execution/\- done: old delivery\n## History and execution/;
    push @cases, ['current chronology rejected', scalar(live_root_errors($chronology, $current, $layout, $owners)) > 0];
    push @cases, ['unsafe archive path rejected', !safe_relative('../ROADMAP.md')];

    my $policy = { max_capsules => 3, warning_capsules => 2, remedy => 'seal the series' };
    my $archive = { source_capsule => 'docs/archive/roadmap/source-through-2026-08-08.md' };
    my $capsule_bytes = "# ROADMAP\n## Objective\n- retired\n";
    my $capsule_digest = sha256_hex($capsule_bytes);
    my $capsule_metrics = metrics($capsule_bytes);
    my $sealed = sub {
        my (%override) = @_;
        return {
            id => 'root-through-2026-08-11',
            capsule => 'docs/archive/roadmap/root-through-2026-08-11.md',
            sha256 => $capsule_digest,
            lines => $capsule_metrics->{lines},
            bytes => $capsule_metrics->{bytes},
            line_bytes => $capsule_metrics->{line_bytes},
            sealed_date => '2026-08-11',
            reason => 'chronology accretion reached the ceiling',
            %override,
        };
    };
    my $schema_errors = sub {
        my ($rollovers, $state) = @_;
        my @found = rollover_schema_errors($rollovers, $state // 'migrated', $policy, $archive, $current);
        return scalar(@found);
    };
    push @cases, ['valid sealed rollover accepted', !$schema_errors->([$sealed->()])];
    push @cases, ['empty rollover series accepted', !$schema_errors->([])];
    push @cases, ['non-array rollovers rejected', $schema_errors->({}) > 0];
    push @cases, ['unknown rollover field rejected', $schema_errors->([$sealed->(note => 'x')]) > 0];
    push @cases, ['non-identifier rollover id rejected', $schema_errors->([$sealed->(id => 'Root 1')]) > 0];
    push @cases, ['duplicate rollover id rejected',
        $schema_errors->([$sealed->(), $sealed->(capsule => 'docs/archive/roadmap/root-through-2026-08-12.md')]) > 0];
    push @cases, ['duplicate rollover capsule rejected',
        $schema_errors->([$sealed->(), $sealed->(id => 'root-through-2026-08-12')]) > 0];
    push @cases, ['pre-migration capsule reuse rejected',
        $schema_errors->([$sealed->(capsule => $archive->{source_capsule})]) > 0];
    push @cases, ['unsafe rollover capsule rejected',
        $schema_errors->([$sealed->(capsule => '../ROADMAP.md')]) > 0];
    push @cases, ['non-Markdown rollover capsule rejected',
        $schema_errors->([$sealed->(capsule => 'docs/archive/roadmap/root.txt')]) > 0];
    push @cases, ['out-of-order rollover dates rejected', $schema_errors->([
        $sealed->(),
        $sealed->(id => 'root-through-2026-08-09', capsule => 'docs/archive/roadmap/root-through-2026-08-09.md',
            sealed_date => '2026-08-09'),
    ]) > 0];
    push @cases, ['malformed rollover date rejected', $schema_errors->([$sealed->(sealed_date => '11-08-2026')]) > 0];
    push @cases, ['missing rollover reason rejected', $schema_errors->([$sealed->(reason => '')]) > 0];
    push @cases, ['rollover above the current-root ceiling rejected',
        $schema_errors->([$sealed->(lines => $current->{enforcement_ceilings}{lines} + 1)]) > 0];
    push @cases, ['rollover count above the declared maximum rejected', $schema_errors->([
        $sealed->(),
        $sealed->(id => 'root-through-2026-08-12', capsule => 'docs/archive/roadmap/root-through-2026-08-12.md',
            sealed_date => '2026-08-12'),
        $sealed->(id => 'root-through-2026-08-13', capsule => 'docs/archive/roadmap/root-through-2026-08-13.md',
            sealed_date => '2026-08-13'),
        $sealed->(id => 'root-through-2026-08-14', capsule => 'docs/archive/roadmap/root-through-2026-08-14.md',
            sealed_date => '2026-08-14'),
    ]) > 0];
    push @cases, ['rollover declared in planned state rejected', $schema_errors->([$sealed->()], 'planned') > 0];

    my $stored = { 'docs/archive/roadmap/root-through-2026-08-11.md' => $capsule_bytes };
    my $reader = sub { return $stored->{ $_[0] } };
    my $sealed_index = "# Roadmap archive\n[current](../../../ROADMAP.md)\n"
        . "[retired](root-through-2026-08-11.md)\n";
    my $content_errors = sub {
        my ($rollovers, $index_text) = @_;
        my @found = rollover_content_errors($rollovers, $reader, $index_text // $sealed_index);
        return scalar(@found);
    };
    push @cases, ['sealed capsule identity accepted', !$content_errors->([$sealed->()])];
    push @cases, ['missing capsule rejected',
        $content_errors->([$sealed->(capsule => 'docs/archive/roadmap/root-through-2026-08-12.md')]) > 0];
    push @cases, ['capsule digest mismatch rejected', $content_errors->([$sealed->(sha256 => '0' x 64)]) > 0];
    push @cases, ['capsule line-count mismatch rejected',
        $content_errors->([$sealed->(lines => $capsule_metrics->{lines} + 1)]) > 0];
    push @cases, ['capsule byte-count mismatch rejected',
        $content_errors->([$sealed->(bytes => $capsule_metrics->{bytes} + 1)]) > 0];
    push @cases, ['capsule absent from the archive index rejected',
        $content_errors->([$sealed->()], "# Roadmap archive\n[current](../../../ROADMAP.md)\n") > 0];

    my $bounded = sub {
        my (@sections) = @_;
        return {
            %$current,
            health_targets => { lines => 30, bytes => 4096, line_bytes => 256 },
            section_limits => { warning_pct => 80, sections => \@sections },
        };
    };
    my $remedy = 'route the detail to its owning task tree';
    my @declared = (
        { heading => 'Objective', max_lines => 4, remedy => $remedy },
        { heading => 'Workstream status', max_lines => 10, remedy => $remedy },
        { heading => 'History and execution', max_lines => 8, remedy => $remedy },
    );
    my $schema_of = sub {
        my (@sections) = @_;
        my @found = section_limit_schema_errors($bounded->(@sections));
        return scalar(@found);
    };
    my $bounds_of = sub {
        my ($text, @sections) = @_;
        my @found = section_limit_errors($text, $bounded->(@sections));
        return scalar(@found);
    };
    push @cases, ['valid section bounds accepted', !$schema_of->(@declared)];
    push @cases, ['missing section_limits rejected',
        scalar(section_limit_schema_errors({ %$current, required_h2_order => $current->{required_h2_order} })) > 0];
    push @cases, ['section declaration missing a required H2 rejected', $schema_of->(@declared[0, 1]) > 0];
    push @cases, ['section declaration in the wrong order rejected', $schema_of->(@declared[1, 0, 2]) > 0];
    push @cases, ['undeclared extra section rejected',
        $schema_of->(@declared, { heading => 'Appendix', max_lines => 4, remedy => $remedy }) > 0];
    push @cases, ['section without max_lines rejected',
        $schema_of->({ heading => 'Objective', remedy => $remedy }, @declared[1, 2]) > 0];
    push @cases, ['section without a remedy rejected',
        $schema_of->({ heading => 'Objective', max_lines => 4 }, @declared[1, 2]) > 0];
    push @cases, ['section with an unknown field rejected',
        $schema_of->({ %{ $declared[0] }, owner => 'x' }, @declared[1, 2]) > 0];
    push @cases, ['section bounds summing above the health target rejected',
        $schema_of->({ %{ $declared[0] }, max_lines => 20 }, @declared[1, 2]) > 0];
    push @cases, ['section measurement counts the heading and trims trailing blanks',
        join('|', map { "$_->{heading}=$_->{lines}" } section_line_counts($valid_live))
            eq 'Objective=2|Workstream status=7|History and execution=4'];
    push @cases, ['section within its bound accepted', !$bounds_of->($valid_live, @declared)];
    push @cases, ['section exactly at its bound accepted',
        !$bounds_of->($valid_live, { %{ $declared[0] }, max_lines => 2 }, @declared[1, 2])];
    push @cases, ['section one line over its bound rejected',
        $bounds_of->($valid_live, { %{ $declared[0] }, max_lines => 1 }, @declared[1, 2]) > 0];
    (my $accreted = $valid_live) =~ s/^- current\n/- current\n- and then this leaf closed\n/m;
    push @cases, ['prose accretion inside one section rejected',
        $bounds_of->($accreted, { %{ $declared[0] }, max_lines => 2 }, @declared[1, 2]) > 0];
    push @cases, ['section present in the root but undeclared rejected',
        $bounds_of->($valid_live, @declared[0, 1]) > 0];
    my $pressure = sub {
        my ($text, @sections) = @_;
        my @found = section_pressure_warnings($text, $bounded->(@sections));
        return scalar(@found);
    };
    push @cases, ['section below the warning percentage stays quiet', !$pressure->($valid_live, @declared)];
    push @cases, ['section at the warning percentage reports its remedy',
        $pressure->($valid_live, { %{ $declared[0] }, max_lines => 2 }, @declared[1, 2]) == 1];
    push @cases, ['an over-bound section warns as an error, not a warning',
        !$pressure->($valid_live, { %{ $declared[0] }, max_lines => 1 }, @declared[1, 2])];

    my $passed = 0;
    for my $case (@cases) {
        die "roadmap-projection: self-test failed: $case->[0]\n" if !$case->[1];
        $passed++;
    }
    print "roadmap-projection: self-test $passed/", scalar(@cases), " passed.\n";
}
