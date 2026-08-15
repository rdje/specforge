#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;
use JSON::PP;

my $root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my $registry_rel = 'doctrine/live_document_size/surfaces.jsonl';
my $mode = 'check';

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = abs_path(shift @ARGV // '') // die "workflow-capacity: root does not exist\n";
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--self-test') {
        run_self_test();
        exit 0;
    } else {
        usage();
    }
}

safe_relative($registry_rel) or die "workflow-capacity: registry path is unsafe\n";
my ($meta, @surfaces) = read_jsonl(absolute($registry_rel));
my @matches = grep { ($_->{surface_id} // '') eq 'workflow_standards' } @surfaces;
die "workflow-capacity: workflow_standards must occur exactly once\n" if @matches != 1;
my $surface = $matches[0];
my $targets = $surface->{targets};
die "workflow-capacity: workflow_standards targets must be a nonempty array\n"
    if ref($targets) ne 'ARRAY' || !@$targets;

my %seen;
my %daily;
for my $path (@$targets) {
    die "workflow-capacity: unsafe or non-Markdown target '$path'\n"
        if !safe_relative($path) || $path !~ /\.md\z/ || $path =~ /[*?]/;
    die "workflow-capacity: duplicate target '$path'\n" if $seen{$path}++;
    git_capture('ls-files', '--error-unmatch', '--', $path);
    my $history = git_capture(
        'log', '--diff-filter=A', '--follow', '--date=short', '--format=%ad', '--', $path,
    );
    my @dates = grep { /\A\d{4}-\d{2}-\d{2}\z/ } split /\n/, $history;
    die "workflow-capacity: target '$path' has no Git creation date\n" if !@dates;
    $daily{$dates[-1]}++;
}

my $current = scalar @$targets;
my $peak = 0;
for my $date (keys %daily) {
    $peak = $daily{$date} if $daily{$date} > $peak;
}
my $milestones = $surface->{milestones};
die "workflow-capacity: milestones must be an object\n" if ref($milestones) ne 'HASH';
my $warning = positive_integer($milestones->{warning_pct}, 'warning_pct');
my $rollover = positive_integer($milestones->{rollover_pct}, 'rollover_pct');
die "workflow-capacity: milestones must satisfy warning < rollover < 100\n"
    if $warning >= $rollover || $rollover >= 100;

my $portable_cap = positive_integer($meta->{max_array_items}, 'registry max_array_items');
my $derived = derive_capacity($current, $peak, $warning, $rollover, $portable_cap);
my $health = validate_band($surface->{health_targets}, 'health_targets');
my $ceiling = validate_band($surface->{enforcement_ceilings}, 'enforcement_ceilings');
die "workflow-capacity: health and ceiling file capacities must agree\n"
    if $health->{files} != $ceiling->{files};
die "workflow-capacity: declared capacity $ceiling->{files} differs from derived minimum $derived\n"
    if $mode eq 'check' && $ceiling->{files} != $derived;

my $result = {
    surface_id => 'workflow_standards',
    current_files => $current,
    daily_additions => { map { $_ => $daily{$_} } sort keys %daily },
    peak_active_day => $peak,
    warning_pct => $warning,
    rollover_pct => $rollover,
    derived_files => $derived,
    declared_files => $ceiling->{files},
    current_utilization_pct => sprintf('%.1f', 100 * $current / $ceiling->{files}),
    one_peak_utilization_pct => sprintf('%.1f', 100 * ($current + $peak) / $ceiling->{files}),
    portable_explicit_target_cap => $portable_cap,
};

if ($mode eq 'report') {
    print JSON::PP->new->canonical(1)->encode($result), "\n";
} else {
    print "workflow-capacity: $current current + peak $peak derive $derived files; "
        . "$result->{current_utilization_pct}% current / "
        . "$result->{one_peak_utilization_pct}% after one peak.\n";
}

sub usage {
    die "Usage: $0 [--check|--report|--self-test] [--root DIR] [--registry PATH]\n";
}

sub absolute {
    my ($relative) = @_;
    return File::Spec->catfile($root, split m{/}, $relative);
}

sub safe_relative {
    my ($path) = @_;
    return defined($path) && !ref($path) && $path ne '' && $path !~ m{\A/}
        && $path !~ /\\/ && $path !~ m{(?:\A|/)\.\.(?:/|\z)} && $path !~ /[\x00-\x1f]/;
}

sub read_jsonl {
    my ($path) = @_;
    open my $fh, '<:raw', $path or die "workflow-capacity: cannot read registry: $!\n";
    my @records;
    my $json = JSON::PP->new->utf8(1);
    while (my $line = <$fh>) {
        next if $line =~ /^\s*\z/;
        my $record = eval { $json->decode($line) };
        die "workflow-capacity: registry contains invalid JSONL\n"
            if $@ || ref($record) ne 'HASH';
        push @records, $record;
    }
    close $fh or die "workflow-capacity: cannot close registry: $!\n";
    die "workflow-capacity: registry is empty\n" if !@records;
    my $meta = shift @records;
    return ($meta, @records);
}

sub git_capture {
    my (@args) = @_;
    open my $fh, '-|', 'git', '-C', $root, @args
        or die "workflow-capacity: cannot execute git @args: $!\n";
    local $/;
    my $output = <$fh> // '';
    close $fh or die "workflow-capacity: git @args failed\n";
    return $output;
}

sub positive_integer {
    my ($value, $label) = @_;
    die "workflow-capacity: $label must be a positive integer\n"
        if !defined($value) || ref($value) || $value !~ /\A[1-9][0-9]*\z/;
    return 0 + $value;
}

sub derive_capacity {
    my ($current, $peak, $warning, $rollover, $cap) = @_;
    die "workflow-capacity: derivation inputs must be positive\n"
        if $current < 1 || $peak < 1 || $warning < 1 || $rollover < 1 || $cap < 1;
    for my $files (1 .. $cap) {
        return $files
            if 100 * $current < $warning * $files
            && 100 * ($current + $peak) < $rollover * $files;
    }
    die "workflow-capacity: no warning-safe profile fits portable cap $cap\n";
}

sub validate_band {
    my ($band, $label) = @_;
    die "workflow-capacity: $label must be an object\n" if ref($band) ne 'HASH';
    my %values;
    $values{$_} = positive_integer($band->{$_}, "$label.$_")
        for qw(files lines_each bytes_each lines_total bytes_total line_bytes_each);
    die "workflow-capacity: $label lines_total is not files times lines_each\n"
        if $values{lines_total} != $values{files} * $values{lines_each};
    die "workflow-capacity: $label bytes_total is not files times bytes_each\n"
        if $values{bytes_total} != $values{files} * $values{bytes_each};
    return \%values;
}

sub run_self_test {
    die "workflow-capacity self-test: 14 + 4 must derive 21\n"
        if derive_capacity(14, 4, 80, 90, 32) != 21;
    die "workflow-capacity self-test: exact 90% must not pass\n"
        if 100 * (14 + 4) < 90 * 20;
    die "workflow-capacity self-test: small profile drift\n"
        if derive_capacity(2, 1, 80, 90, 8) != 4;
    my $overflow = eval { derive_capacity(14, 4, 80, 90, 20); 1 };
    die "workflow-capacity self-test: portable-cap overflow did not fail\n" if $overflow;
    my $bad_band = eval {
        validate_band({
            files => 21, lines_each => 700, bytes_each => 65_536,
            lines_total => 14_699, bytes_total => 1_376_256, line_bytes_each => 1_024,
        }, 'fixture');
        1;
    };
    die "workflow-capacity self-test: aggregate drift did not fail\n" if $bad_band;
    print "workflow-capacity: 5/5 derivation, strict-boundary, hard-cap, and aggregate cases pass.\n";
}
