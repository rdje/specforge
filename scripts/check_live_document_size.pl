#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use Digest::SHA qw(sha256_hex);
use File::Basename qw(dirname);
use File::Find qw(find);
use File::Spec;
use JSON::PP;

my $root;
my $registry_rel = 'doctrine/live_document_size/surfaces.jsonl';
my $routes_rel = 'doctrine/readme_entrypoint/routed_destinations.tsv';
my $authorities_rel = 'doctrine/live_document_size/ceiling_increase_authorities.jsonl';
my $report = 0;
my $no_history = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--routes') {
        $routes_rel = shift @ARGV // usage();
    } elsif ($arg eq '--authorities') {
        $authorities_rel = shift @ARGV // usage();
    } elsif ($arg eq '--report') {
        $report = 1;
    } elsif ($arg eq '--no-history') {
        $no_history = 1;
    } else {
        usage();
    }
}

$root //= File::Spec->catdir(dirname(abs_path($0)), '..');
$root = abs_path($root) // die "live-document-size: repository root does not exist\n";

my @errors;
my @warnings;
my $json = JSON::PP->new->canonical(1);
my @dimensions = qw(files lines_each bytes_each lines_total bytes_total line_bytes_each);
my %valid_lifecycle = map { $_ => 1 } qw(
  bounded_snapshot rolling_ledger partitioned_canonical generated_projection
  archive_terminal frozen_legacy maintained_reference
);
my %valid_state = map { $_ => 1 } qw(normal warning_debt rollover_debt transition_debt frozen terminal);

my ($registry_meta, $surfaces) = read_jsonl_registry(
    absolute($registry_rel),
    'surface registry',
    128,
    131_072,
    8_192,
);
my ($authority_meta, $authorities) = read_jsonl_registry(
    absolute($authorities_rel),
    'ceiling-authority registry',
    128,
    65_536,
    8_192,
);

my @paths = markdown_paths();
my %path_seen = map { $_ => 1 } @paths;
my %surface_by_id;
my %matches_by_surface;

for my $surface (@$surfaces) {
    my $id = required_scalar($surface, 'surface_id', 'surface record');
    next if !defined $id;
    if ($surface_by_id{$id}) {
        problem("duplicate surface_id '$id'");
        next;
    }
    $surface_by_id{$id} = $surface;
    my $lifecycle = required_scalar($surface, 'lifecycle', "surface '$id'");
    problem("surface '$id' has unknown lifecycle '$lifecycle'")
        if defined($lifecycle) && !$valid_lifecycle{$lifecycle};
    my $state = required_scalar($surface, 'state', "surface '$id'");
    problem("surface '$id' has unknown state '$state'")
        if defined($state) && !$valid_state{$state};
    required_scalar($surface, 'owner', "surface '$id'");
    required_scalar($surface, 'locator', "surface '$id'");

    my $targets = $surface->{targets};
    if (ref($targets) ne 'ARRAY' || !@$targets) {
        problem("surface '$id' must declare a non-empty targets array");
        next;
    }
    my @regexes;
    for my $target (@$targets) {
        if (!defined($target) || ref($target) || $target eq '') {
            problem("surface '$id' has an empty or non-scalar target");
            next;
        }
        if (!safe_relative_pattern($target)) {
            problem("surface '$id' target '$target' is absolute, parent-relative, or off-root");
            next;
        }
        push @regexes, glob_regex($target);
    }
    my @matched = grep {
        my $path = $_;
        scalar grep { $path =~ $_ } @regexes;
    } @paths;
    if (!@matched) {
        problem("surface '$id' matches no Markdown path");
    }
    $matches_by_surface{$id} = \@matched;
}

for my $path (@paths) {
    my @owners;
    for my $id (sort keys %matches_by_surface) {
        push @owners, $id if grep { $_ eq $path } @{ $matches_by_surface{$id} };
    }
    problem("tracked Markdown '$path' is not classified") if !@owners;
    problem("tracked Markdown '$path' is classified more than once: " . join(', ', @owners))
        if @owners > 1;
}

my %metrics_by_surface;
my %executed_verifier;
for my $id (sort keys %surface_by_id) {
    my $surface = $surface_by_id{$id};
    my $paths = $matches_by_surface{$id} // [];
    my $metrics = measure_paths($paths);
    $metrics_by_surface{$id} = $metrics;
    validate_limits($surface, $metrics, $id);
    validate_lifecycle($surface, $paths, $metrics, $id, \%executed_verifier);
    if ($report) {
        print $json->encode({ surface_id => $id, metrics => $metrics }), "\n";
    }
}

validate_routes(absolute($routes_rel), \%path_seen, \%surface_by_id, \%matches_by_surface);
validate_ceiling_history($surfaces, $authorities) if !$no_history;

if (@errors) {
    print STDERR "live-document-size: $_\n" for @errors;
    print STDERR "live-document-size: FAILED with ", scalar(@errors), " violation(s).\n";
    exit 1;
}

print STDERR "live-document-size: warning: $_\n" for @warnings;
print "live-document-size: ", scalar(@paths), " Markdown files satisfy ",
    scalar(keys %surface_by_id), " governed surfaces.\n" if !$report;
exit 0;

sub usage {
    die "Usage: $0 [--root DIR] [--registry PATH] [--routes PATH] [--authorities PATH] [--report] [--no-history]\n";
}

sub absolute {
    my ($relative) = @_;
    return File::Spec->catfile($root, split m{/}, $relative);
}

sub problem {
    my ($message) = @_;
    push @errors, $message;
}

sub read_jsonl_registry {
    my ($path, $label, $hard_records, $hard_bytes, $hard_record_bytes) = @_;
    if (!-f $path) {
        problem("$label is missing: " . relative_to_root($path));
        return ({}, []);
    }
    my $file_bytes = -s $path;
    problem("$label exceeds portable hard byte cap $hard_bytes") if $file_bytes > $hard_bytes;
    open my $fh, '<:raw', $path or do {
        problem("cannot read $label: $!");
        return ({}, []);
    };
    my @records;
    my @record_lengths;
    my $line_number = 0;
    while (my $line = <$fh>) {
        $line_number++;
        chomp $line;
        $line =~ s/\r\z//;
        next if $line eq '';
        problem("$label line $line_number exceeds portable raw-record cap $hard_record_bytes")
            if length($line) > $hard_record_bytes;
        my $record = eval { decode_json($line) };
        if (!$record || ref($record) ne 'HASH') {
            problem("$label line $line_number is not one JSON object: $@");
            next;
        }
        push @records, $record;
        push @record_lengths, length($line);
    }
    close $fh;
    if (!@records) {
        problem("$label is empty");
        return ({}, []);
    }
    my $meta = shift @records;
    problem("$label first record must have record_type=registry")
        if ($meta->{record_type} // '') ne 'registry';
    for my $field (qw(schema_version max_records max_bytes max_record_bytes)) {
        problem("$label registry record lacks numeric '$field'")
            if !defined($meta->{$field}) || ref($meta->{$field}) || $meta->{$field} !~ /^\d+$/;
    }
    if (defined $meta->{max_records}) {
        problem("$label declares max_records above portable hard cap") if $meta->{max_records} > $hard_records;
        problem("$label has more records than its declared max_records") if @records > $meta->{max_records};
    }
    if (defined $meta->{max_bytes}) {
        problem("$label declares max_bytes above portable hard cap") if $meta->{max_bytes} > $hard_bytes;
        problem("$label exceeds its declared max_bytes") if $file_bytes > $meta->{max_bytes};
    }
    if (defined $meta->{max_record_bytes}) {
        problem("$label declares max_record_bytes above portable hard cap")
            if $meta->{max_record_bytes} > $hard_record_bytes;
        for my $length (@record_lengths) {
            problem("$label contains a record above its declared max_record_bytes")
                if $length > $meta->{max_record_bytes};
        }
    }
    return ($meta, \@records);
}

sub markdown_paths {
    my @result;
    my $git_top = git_top();
    if ($git_top eq $root) {
        open my $fh, '-|', 'git', '-C', $root, 'ls-files', '-z', '--cached', '--', '*.md'
            or die "live-document-size: cannot enumerate Git Markdown paths\n";
        local $/ = "\0";
        while (my $path = <$fh>) {
            chomp $path;
            push @result, $path if $path ne '';
        }
        close $fh;
    } else {
        find(
            {
                wanted => sub {
                    return if -d $_;
                    return if $_ !~ /\.md\z/;
                    my $absolute = $File::Find::name;
                    my $relative = File::Spec->abs2rel($absolute, $root);
                    return if $relative =~ m{^(?:\.git|generated)(?:/|\z)};
                    $relative =~ s{\\}{/}g;
                    push @result, $relative;
                },
                no_chdir => 1,
            },
            $root,
        );
    }
    my %unique;
    return sort grep { !$unique{$_}++ } @result;
}

sub safe_relative_pattern {
    my ($path) = @_;
    return 0 if $path =~ m{\A/} || $path =~ m{\\};
    return 0 if $path =~ m{(?:\A|/)\.\.(?:/|\z)};
    return 0 if $path =~ /[\x00-\x1f]/;
    return 1;
}

sub glob_regex {
    my ($pattern) = @_;
    my $regex = '';
    my @chars = split //, $pattern;
    for (my $i = 0; $i < @chars; $i++) {
        if ($chars[$i] eq '*' && $i + 1 < @chars && $chars[$i + 1] eq '*') {
            $regex .= '.*';
            $i++;
        } elsif ($chars[$i] eq '*') {
            $regex .= '[^/]*';
        } elsif ($chars[$i] eq '?') {
            $regex .= '[^/]';
        } else {
            $regex .= quotemeta($chars[$i]);
        }
    }
    return qr/\A$regex\z/;
}

sub required_scalar {
    my ($record, $field, $label) = @_;
    if (!defined($record->{$field}) || ref($record->{$field}) || $record->{$field} eq '') {
        problem("$label lacks non-empty scalar '$field'");
        return undef;
    }
    return $record->{$field};
}

sub measure_paths {
    my ($paths) = @_;
    my %metrics = map { $_ => 0 } @dimensions;
    $metrics{files} = scalar @$paths;
    for my $path (@$paths) {
        my $absolute = absolute($path);
        if (!-f $absolute) {
            problem("classified Markdown '$path' is missing from the resulting tree");
            next;
        }
        open my $fh, '<:raw', $absolute or do {
            problem("cannot read classified Markdown '$path': $!");
            next;
        };
        local $/;
        my $content = <$fh> // '';
        close $fh;
        my $bytes = length($content);
        my $lines = ($content =~ tr/\n/\n/);
        my $max_line = 0;
        for my $line (split /\n/, $content, -1) {
            $line =~ s/\r\z//;
            $max_line = length($line) if length($line) > $max_line;
        }
        $metrics{bytes_total} += $bytes;
        $metrics{lines_total} += $lines;
        $metrics{bytes_each} = $bytes if $bytes > $metrics{bytes_each};
        $metrics{lines_each} = $lines if $lines > $metrics{lines_each};
        $metrics{line_bytes_each} = $max_line if $max_line > $metrics{line_bytes_each};
    }
    return \%metrics;
}

sub numeric_dimensions {
    my ($object, $label, $allow_null) = @_;
    if (ref($object) ne 'HASH') {
        problem("$label must be an object");
        return;
    }
    for my $dimension (@dimensions) {
        if (!exists $object->{$dimension}) {
            problem("$label lacks '$dimension'");
        } elsif (!defined $object->{$dimension}) {
            problem("$label may not set '$dimension' to null") if !$allow_null;
        } elsif (ref($object->{$dimension}) || $object->{$dimension} !~ /^\d+$/) {
            problem("$label '$dimension' must be a non-negative integer or allowed null");
        }
    }
}

sub validate_limits {
    my ($surface, $metrics, $id) = @_;
    my $allow_null = ($surface->{lifecycle} // '') eq 'maintained_reference';
    numeric_dimensions($surface->{health_targets}, "surface '$id' health_targets", $allow_null);
    numeric_dimensions($surface->{enforcement_ceilings}, "surface '$id' enforcement_ceilings", $allow_null);
    my $ceilings = $surface->{enforcement_ceilings};
    return if ref($ceilings) ne 'HASH';
    for my $dimension (@dimensions) {
        next if !defined $ceilings->{$dimension};
        problem("surface '$id' exceeds $dimension ceiling: $metrics->{$dimension} > $ceilings->{$dimension}")
            if $metrics->{$dimension} > $ceilings->{$dimension};
    }
    my $targets = $surface->{health_targets};
    my $milestones = $surface->{milestones};
    if (ref($targets) eq 'HASH' && ref($milestones) eq 'HASH') {
        my $warning = $milestones->{warning_pct};
        my $rollover = $milestones->{rollover_pct};
        if (!defined($warning) || !defined($rollover) || $warning !~ /^\d+$/ || $rollover !~ /^\d+$/ || $warning >= $rollover || $rollover >= 100) {
            problem("surface '$id' has invalid warning/rollover milestones");
        } else {
            for my $dimension (@dimensions) {
                next if !defined $targets->{$dimension} || $targets->{$dimension} == 0;
                my $percent = 100 * $metrics->{$dimension} / $targets->{$dimension};
                next if $dimension eq 'files' && $metrics->{$dimension} == $targets->{$dimension};
                next if ($surface->{locator} // '') eq 'file'
                    && ($dimension eq 'lines_total' || $dimension eq 'bytes_total');
                next if ($surface->{lifecycle} // '') eq 'frozen_legacy';
                next if ($surface->{state} // '') eq 'transition_debt';
                if ($percent >= $rollover) {
                    push @warnings, sprintf("surface '%s' %s is at or above rollover (%.1f%%)", $id, $dimension, $percent);
                } elsif ($percent >= $warning) {
                    push @warnings, sprintf("surface '%s' %s is at or above warning (%.1f%%)", $id, $dimension, $percent);
                }
            }
        }
    } else {
        problem("surface '$id' must declare milestones");
    }
    if (($surface->{state} // '') eq 'transition_debt') {
        numeric_dimensions($surface->{baseline}, "surface '$id' baseline", 0);
        my $transition = $surface->{transition};
        if (ref($transition) ne 'HASH' || !defined($transition->{owner}) || ref($transition->{owner}) || $transition->{owner} eq '') {
            problem("surface '$id' transition debt lacks an owner");
        } else {
            numeric_dimensions($transition->{max_growth}, "surface '$id' transition.max_growth", 0);
            if (ref($surface->{baseline}) eq 'HASH' && ref($transition->{max_growth}) eq 'HASH') {
                for my $dimension (@dimensions) {
                    next if !defined($surface->{baseline}{$dimension}) || !defined($transition->{max_growth}{$dimension});
                    my $allowed = $surface->{baseline}{$dimension} + $transition->{max_growth}{$dimension};
                    problem("surface '$id' exceeds immutable baseline plus transition allowance for $dimension: $metrics->{$dimension} > $allowed")
                        if $metrics->{$dimension} > $allowed;
                }
            }
        }
    }
}

sub validate_lifecycle {
    my ($surface, $paths, $metrics, $id, $executed) = @_;
    my $lifecycle = $surface->{lifecycle} // '';
    required_scalar($surface, 'verifier', "surface '$id'");
    if ($lifecycle eq 'bounded_snapshot') {
        problem("surface '$id' bounded_snapshot must contain exactly one file") if @$paths != 1;
    } elsif ($lifecycle eq 'partitioned_canonical') {
        validate_index($surface, $paths, $id);
    } elsif ($lifecycle eq 'generated_projection') {
        problem("surface '$id' generated_projection must contain exactly one file") if @$paths != 1;
        my $verifier = required_scalar($surface, 'freshness_verifier', "surface '$id'");
        if (defined $verifier) {
            if (!safe_relative_pattern($verifier) || $verifier =~ /[*?]/) {
                problem("surface '$id' freshness verifier must be one repository-relative executable path");
            } else {
                my $script = absolute($verifier);
                if (!-x $script) {
                    problem("surface '$id' freshness verifier '$verifier' is missing or non-executable");
                } elsif (!$executed->{$verifier}++) {
                    my $status = system { $script } $script;
                    problem("surface '$id' freshness verifier '$verifier' failed") if $status != 0;
                }
            }
        }
    } elsif ($lifecycle eq 'frozen_legacy') {
        problem("surface '$id' frozen_legacy must contain exactly one file") if @$paths != 1;
        my $expected = required_scalar($surface, 'sha256', "surface '$id'");
        if (defined($expected) && @$paths == 1) {
            open my $fh, '<:raw', absolute($paths->[0]) or do {
                problem("cannot hash frozen surface '$id'");
                return;
            };
            local $/;
            my $actual = sha256_hex(<$fh> // '');
            close $fh;
            problem("surface '$id' frozen hash changed") if $actual ne $expected;
        }
    } elsif ($lifecycle eq 'maintained_reference') {
        for my $dimension (qw(lines_each bytes_each line_bytes_each)) {
            problem("surface '$id' maintained_reference requires a numeric $dimension ceiling")
                if !defined $surface->{enforcement_ceilings}{$dimension};
        }
        for my $dimension (qw(files lines_total bytes_total)) {
            problem("surface '$id' maintained_reference must delegate $dimension to exact aggregate authority")
                if defined $surface->{enforcement_ceilings}{$dimension};
        }
        validate_index($surface, $paths, $id);
        my $contract = $surface->{reference_contract};
        if (ref($contract) ne 'HASH') {
            problem("surface '$id' maintained_reference lacks reference_contract");
            return;
        }
        my $mandatory = $contract->{mandatory_read};
        if (ref($mandatory) ne 'HASH') {
            problem("surface '$id' maintained_reference lacks mandatory_read contract");
        } else {
            my $path = $mandatory->{path} // '';
            my $file_metrics = measure_paths([$path]);
            problem("surface '$id' mandatory_read path is outside the surface")
                if !grep { $_ eq $path } @$paths;
            problem("surface '$id' mandatory_read exceeds line ceiling")
                if $file_metrics->{lines_total} > ($mandatory->{lines_ceiling} // -1);
            problem("surface '$id' mandatory_read exceeds byte ceiling")
                if $file_metrics->{bytes_total} > ($mandatory->{bytes_ceiling} // -1);
        }
        problem("surface '$id' max_navigation_depth must be 1")
            if ($contract->{max_navigation_depth} // 0) != 1;
        my $authority = $contract->{aggregate_change};
        if (ref($authority) ne 'HASH') {
            problem("surface '$id' lacks aggregate_change authority");
        } else {
            required_scalar($authority, 'authority_id', "surface '$id' aggregate authority");
            required_scalar($authority, 'owner', "surface '$id' aggregate authority");
            required_scalar($authority, 'rationale', "surface '$id' aggregate authority");
            for my $dimension (qw(files lines_total bytes_total)) {
                my $baseline = $authority->{baseline}{$dimension};
                my $delta = $authority->{delta}{$dimension};
                if (!defined($baseline) || $baseline !~ /^\d+$/ || !defined($delta) || $delta !~ /^-?\d+$/) {
                    problem("surface '$id' aggregate authority has invalid '$dimension' baseline/delta");
                    next;
                }
                my $expected = $baseline + $delta;
                problem("surface '$id' aggregate authority is stale for $dimension: actual $metrics->{$dimension}, expected $expected")
                    if $metrics->{$dimension} != $expected;
            }
        }
    } elsif ($lifecycle eq 'rolling_ledger') {
        problem("surface '$id' rolling_ledger must contain exactly one file") if @$paths != 1;
        problem("surface '$id' rolling ledger must remain transition_debt until it has a shard/archive manifest")
            if ($surface->{state} // '') ne 'transition_debt' && !defined($surface->{archive_manifest});
    } elsif ($lifecycle eq 'archive_terminal') {
        problem("surface '$id' archive terminal must have state=terminal")
            if ($surface->{state} // '') ne 'terminal';
        required_scalar($surface, 'archive_manifest', "surface '$id'");
    }
    validate_currency($surface, $id, $executed) if exists $surface->{currency};
}

sub validate_currency {
    my ($surface, $id, $executed) = @_;
    my $currency = $surface->{currency};
    if (ref($currency) ne 'HASH') {
        problem("surface '$id' currency contract must be an object");
        return;
    }
    my $status = $currency->{status} // '';
    my $verifier = $currency->{verifier} // '';
    if ($status eq 'transition_debt') {
        problem("surface '$id' currency debt lacks an owner")
            if !defined($currency->{owner}) || ref($currency->{owner}) || $currency->{owner} eq '';
        problem("surface '$id' currency debt verifier must use debt: identifier")
            if $verifier !~ /^debt:[a-z0-9][a-z0-9._-]*$/;
    } elsif ($status eq 'enforced') {
        if (!safe_relative_pattern($verifier) || $verifier =~ /[*?]/ || !-x absolute($verifier)) {
            problem("surface '$id' currency verifier '$verifier' is missing or non-executable");
        } elsif (!$executed->{$verifier}++) {
            my $script = absolute($verifier);
            my $exit = system { $script } $script;
            problem("surface '$id' currency verifier '$verifier' failed") if $exit != 0;
        }
    } else {
        problem("surface '$id' currency status must be transition_debt or enforced");
    }
}

sub validate_index {
    my ($surface, $paths, $id) = @_;
    my $index = required_scalar($surface, 'index', "surface '$id'");
    my $contract = $surface->{index_contract};
    if (ref($contract) ne 'HASH') {
        problem("surface '$id' lacks index_contract");
        return;
    }
    my $kind = $contract->{kind} // '';
    if ($kind eq 'query') {
        problem("surface '$id' query index must be git:query") if ($index // '') ne 'git:query';
        return;
    }
    if ($kind ne 'membership') {
        problem("surface '$id' has unknown index kind '$kind'");
        return;
    }
    return if !defined $index;
    problem("surface '$id' membership index '$index' is outside the surface")
        if !grep { $_ eq $index } @$paths;
    my $absolute_index = absolute($index);
    if (!-f $absolute_index) {
        problem("surface '$id' membership index '$index' is missing");
        return;
    }
    open my $fh, '<:raw', $absolute_index or do {
        problem("surface '$id' cannot read membership index '$index'");
        return;
    };
    local $/;
    my $content = <$fh> // '';
    close $fh;
    my %linked;
    while ($content =~ /\]\(([^)]+)\)/g) {
        my $href = $1;
        $href =~ s/#.*\z//;
        next if $href eq '' || $href =~ m{^[a-z]+://}i;
        my $candidate = abs_path(File::Spec->catfile(dirname($absolute_index), split m{/}, $href));
        next if !defined $candidate || index($candidate, "$root/") != 0;
        $linked{relative_to_root($candidate)} = 1;
    }
    for my $path (@$paths) {
        next if $path eq $index;
        problem("surface '$id' index '$index' does not link member '$path'") if !$linked{$path};
    }
}

sub validate_routes {
    my ($path, $path_seen, $surfaces, $matches) = @_;
    if (!-f $path) {
        problem('README route registry is missing');
        return;
    }
    open my $fh, '<:raw', $path or do {
        problem("cannot read README route registry: $!");
        return;
    };
    my $header = <$fh> // '';
    chomp $header;
    problem('README route registry schema is unknown')
        if $header ne "target\troute_kind\towner\tlifecycle\tpressure_control\tterminal";
    while (my $line = <$fh>) {
        chomp $line;
        my ($target) = split /\t/, $line, 2;
        next if !defined($target) || $target eq '' || $target =~ m{^(?:https://|git:)};
        if ($target =~ /\.md\z/) {
            problem("README route '$target' is not a classified Markdown surface") if !$path_seen->{$target};
        } elsif ($target =~ m{/\z}) {
            my $found = grep { index($_, $target) == 0 } keys %$path_seen;
            problem("README route collection '$target' contains no classified Markdown") if !$found;
        }
    }
    close $fh;
}

sub validate_ceiling_history {
    my ($current_surfaces, $authorities) = @_;
    my $git_top = git_top();
    return if $git_top ne $root;
    open my $tree_fh, '-|', 'git', '-C', $root, 'ls-tree', '-r', '--name-only', 'HEAD', '--', $registry_rel
        or return;
    my $tracked_registry = <$tree_fh> // '';
    close $tree_fh;
    return if $tracked_registry eq '';
    open my $fh, '-|', 'git', '-C', $root, 'show', "HEAD:$registry_rel" or return;
    local $/;
    my $previous_text = <$fh> // '';
    close $fh;
    return if $previous_text eq '';
    my @previous_records;
    for my $line (split /\n/, $previous_text) {
        next if $line eq '';
        my $record = eval { decode_json($line) };
        next if !$record || ref($record) ne 'HASH' || ($record->{record_type} // '') eq 'registry';
        push @previous_records, $record;
    }
    my %previous = map { ($_->{surface_id} // '') => $_ } @previous_records;
    my %authority_for = map { ($_->{surface_id} // '') => $_ } grep { ($_->{record_type} // '') eq 'increase' } @$authorities;
    for my $current (@$current_surfaces) {
        my $id = $current->{surface_id} // next;
        my $old = $previous{$id} // next;
        if (($old->{state} // '') eq 'transition_debt' && ($current->{state} // '') eq 'transition_debt') {
            my $old_baseline = $json->encode($old->{baseline} // {});
            my $new_baseline = $json->encode($current->{baseline} // {});
            problem("surface '$id' moved its immutable transition baseline") if $old_baseline ne $new_baseline;
        }
        my @increased;
        for my $dimension (@dimensions) {
            my $before = $old->{enforcement_ceilings}{$dimension};
            my $after = $current->{enforcement_ceilings}{$dimension};
            push @increased, $dimension
                if defined($before) && (!defined($after) || $after > $before);
        }
        next if !@increased;
        my $authority = $authority_for{$id};
        if (!$authority
            || $json->encode($authority->{old} // {}) ne $json->encode($old->{enforcement_ceilings} // {})
            || $json->encode($authority->{new} // {}) ne $json->encode($current->{enforcement_ceilings} // {})
            || !defined($authority->{work_unit}) || $authority->{work_unit} eq ''
            || !defined($authority->{owner}) || $authority->{owner} eq ''
            || !defined($authority->{rationale}) || $authority->{rationale} eq '') {
            problem("surface '$id' increased ceiling dimensions without exact authority: " . join(', ', @increased));
        }
    }
}

sub git_top {
    open my $fh, '-|', 'git', '-C', $root, 'rev-parse', '--show-toplevel' or return '';
    my $top = <$fh> // '';
    close $fh;
    chomp $top;
    return $top;
}

sub relative_to_root {
    my ($path) = @_;
    my $relative = File::Spec->abs2rel($path, $root);
    $relative =~ s{\\}{/}g;
    return $relative;
}
