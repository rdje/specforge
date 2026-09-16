#!/usr/bin/env perl
# check_ownership_citations.pl — refuse a current-owner citation that names a closed work unit.
#
# LIVE-DOCUMENT-PRESSURE-HEADROOM.15. Eight times a tracked document named a `done` or `superseded`
# tree as the current owner of live work, and the last one happened under full attention immediately
# after the class was documented. Review demonstrably does not hold this invariant, so a gate must.
#
# What it proves, and what it does not. It proves that every work unit cited inside a declared
# ownership region is classified, and that every citation classified as a CURRENT OWNER names a unit
# whose own `Status:` line is still open. It cannot prove that the open owner is the RIGHT owner —
# that stays a review judgement. The completeness leg is what makes it more than a spot check: a new
# citation added to a declared region with no classification record fails closed, which is exactly how
# the eighth instance entered `ROADMAP.md` unseen.
use strict;
use warnings;
use FindBin qw($Bin);
use File::Spec;
use Cwd qw(abs_path);
use JSON::PP;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $registry_rel = 'doctrine/ownership_citations/citations.jsonl';
my $mode = 'check';
my $root;
while (@ARGV) {
    my $arg = shift @ARGV;
    if    ($arg eq '--check')     { $mode = 'check' }
    elsif ($arg eq '--report')    { $mode = 'report' }
    elsif ($arg eq '--self-test') { $mode = 'self-test' }
    elsif ($arg eq '--root')      { $root = shift @ARGV // usage() }
    elsif ($arg eq '--registry')  { $registry_rel = shift @ARGV // usage() }
    else                          { usage() }
}
sub usage { die "Usage: $0 [--root DIR] [--registry PATH] [--check|--report|--self-test]\n" }

my $project_root = abs_path(File::Spec->catdir($Bin, '..'))
    // die "ownership-citations: cannot resolve repository root\n";
$root //= $project_root;
$root = abs_path($root) // die "ownership-citations: root does not exist\n";

my @errors;
sub problem { push @errors, $_[0] }
sub absolute { return File::Spec->catfile($root, split m{/}, $_[0]) }
sub safe_relative {
    my ($path) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '';
    return 0 if $path =~ m{\A/} || $path =~ /\\/ || $path =~ /[\x00-\x1f\x7f]/;
    return 0 if $path =~ m{(?:\A|/)\.\.?(?:/|\z)} || $path =~ m{//};
    return 1;
}

if ($mode eq 'self-test') { run_self_test(); exit 0 }

my ($regions, $citations) = read_registry(absolute($registry_rel));
my %classified;   # "path\0unit" => record
for my $record (@$citations) {
    my $key = join "\0", $record->{path}, $record->{unit};
    problem("duplicate citation record for '$record->{unit}' in '$record->{path}'")
        if exists $classified{$key};
    $classified{$key} = $record;
}

my %observed;
my $region_count = 0;
my $citation_count = 0;
for my $region (@$regions) {
    my ($path, $marker) = @{$region}{qw(path marker)};
    my $text = slurp($path) // next;
    my @units = region_units($text, $path, $marker);
    $region_count++;
    for my $unit (@units) {
        $citation_count++;
        my $key = join "\0", $path, $unit;
        $observed{$key} = 1;
        my $record = $classified{$key};
        if (!$record) {
            problem("'$path' cites '$unit' inside '$marker' with no classification record; "
                . 'classify it as current_owner or historical');
            next;
        }
        next if $record->{kind} eq 'historical';
        my $status = unit_status($unit);
        if (!defined $status) {
            problem("'$path' names '$unit' as a current owner, but no such work unit exists");
        } elsif ($status eq 'done' || $status eq 'superseded') {
            problem("'$path' names '$unit' as a current owner, but its own Status is '$status'");
        }
    }
}
for my $key (sort keys %classified) {
    next if $observed{$key};
    my ($path, $unit) = split /\0/, $key, 2;
    problem("citation record for '$unit' in '$path' matches no citation in a declared region");
}

if (@errors) {
    print STDERR "ownership-citations: $_\n" for @errors;
    print STDERR "ownership-citations: FAILED with ", scalar(@errors), " violation(s).\n";
    exit 1;
}
if ($mode eq 'report') {
    my $current = grep { $_->{kind} eq 'current_owner' } @$citations;
    print JSON::PP->new->canonical(1)->encode({
        regions => $region_count, citations => $citation_count,
        current_owner => $current, historical => scalar(@$citations) - $current,
    }), "\n";
} else {
    print "ownership-citations: $citation_count cited work units across $region_count "
        . "declared ownership regions resolve to open owners or a stated history.\n";
}
exit 0;

sub slurp {
    my ($relative) = @_;
    if (!safe_relative($relative)) { problem("declared path '$relative' is unsafe"); return }
    my $absolute = absolute($relative);
    if (!-f $absolute) { problem("declared path '$relative' is missing"); return }
    open my $fh, '<:encoding(UTF-8)', $absolute or do {
        problem("cannot read '$relative': $!");
        return;
    };
    local $/;
    my $text = <$fh> // '';
    close $fh;
    return $text;
}

# A region is delimited by an HTML comment pair, the same shape ROADMAP.md already uses for its
# generated workstream table, so the boundary is visible in the document it governs.
sub region_units {
    my ($text, $path, $marker) = @_;
    my $open = "<!-- $marker:start -->";
    my $close = "<!-- $marker:end -->";
    # A document must be able to NAME its own mechanism: the leaf that registered this doctrine quotes
    # `<!-- current_owners:start -->` inside backticks while sitting in a declared region, and counting
    # that quotation as a second marker refused the very prose describing the marker. Only an unquoted
    # occurrence opens or closes a region.
    my $opens = () = $text =~ /(?<!`)\Q$open\E/g;
    my $closes = () = $text =~ /(?<!`)\Q$close\E/g;
    if ($opens != 1 || $closes != 1) {
        problem("'$path' must contain exactly one '$marker' start and end marker; "
            . "found $opens and $closes");
        return ();
    }
    my ($body) = $text =~ /(?<!`)\Q$open\E(.*?)(?<!`)\Q$close\E/s;
    if (!defined $body) {
        problem("'$path' has its '$marker' end marker before its start marker");
        return ();
    }
    my %seen;
    my @units;
    # A citation is a link into docs/tasks/, or a backticked work-unit id whose tree file exists.
    while ($body =~ m{\]\(([^)]*?)docs/tasks/([A-Za-z0-9\-]+)\.md\)}g) {
        my $unit = $2;
        push @units, $unit if !$seen{$unit}++;
    }
    while ($body =~ /`([A-Z][A-Za-z0-9\-]*(?:\.[0-9a-z][0-9a-z.]*)?)`/g) {
        my $unit = $1;
        my ($tree) = split /\./, $unit, 2;
        next if !-f absolute("docs/tasks/$tree.md");
        push @units, $unit if !$seen{$unit}++;
    }
    return sort @units;
}

# The authority is the cited unit's own Status line, never a mention of it elsewhere.
sub unit_status {
    my ($unit) = @_;
    my ($tree, $leaf) = split /\./, $unit, 2;
    my $text = eval { read_task($tree) };
    return undef if !defined $text;
    # One tree in the tracked population writes `- Status: **`active`** - ...`, so the emphasis markers
    # are part of the real grammar, not a typo to normalise away: a checker that only accepted the bare
    # form would have reported that tree as non-existent and invited an edit to satisfy the regex.
    if (defined $leaf) {
        return $1 if $text =~ /^- ID: `\Q$unit\E`\s*\n\s*Status: [*_]{0,2}`([a-z_]+)`/m;
        return undef;
    }
    return $1 if $text =~ /^- Status: [*_]{0,2}`([a-z_]+)`/m;
    return undef;
}

my %task_cache;
sub read_task {
    my ($tree) = @_;
    return $task_cache{$tree} if exists $task_cache{$tree};
    my $absolute = absolute("docs/tasks/$tree.md");
    if (!-f $absolute) { $task_cache{$tree} = undef; return undef }
    open my $fh, '<:encoding(UTF-8)', $absolute or do { $task_cache{$tree} = undef; return undef };
    local $/;
    my $text = <$fh> // '';
    close $fh;
    $task_cache{$tree} = $text;
    return $text;
}

sub read_registry {
    my ($absolute) = @_;
    my (@regions, @citations);
    if (!-f $absolute) { problem("citation registry is missing: $registry_rel"); return (\@regions, \@citations) }
    open my $fh, '<:encoding(UTF-8)', $absolute or do {
        problem("cannot read citation registry: $!");
        return (\@regions, \@citations);
    };
    my $line_number = 0;
    while (my $line = <$fh>) {
        $line_number++;
        chomp $line;
        next if $line =~ /\A\s*\z/;
        my $record = eval { JSON::PP->new->utf8(0)->decode($line) };
        if ($@ || ref($record) ne 'HASH') {
            problem("citation registry line $line_number is not one JSON object");
            next;
        }
        my $kind = $record->{record_type} // '';
        if ($kind eq 'registry') {
            problem('the registry header must be the first record') if $line_number != 1;
            next;
        }
        if ($kind eq 'region') {
            problem("region record at line $line_number needs a safe path")
                if !safe_relative($record->{path});
            problem("region record at line $line_number needs a marker")
                if !defined($record->{marker}) || ref($record->{marker})
                || $record->{marker} !~ /\A[a-z][a-z0-9_]*\z/;
            push @regions, $record;
            next;
        }
        if ($kind eq 'citation') {
            problem("citation record at line $line_number needs a safe path")
                if !safe_relative($record->{path});
            problem("citation record at line $line_number needs a unit")
                if !defined($record->{unit}) || ref($record->{unit}) || $record->{unit} eq '';
            my $classification = $record->{kind} // '';
            problem("citation record at line $line_number must be current_owner or historical")
                if $classification ne 'current_owner' && $classification ne 'historical';
            problem("historical citation at line $line_number must state a reason")
                if $classification eq 'historical'
                && (!defined($record->{reason}) || ref($record->{reason}) || $record->{reason} eq '');
            push @citations, $record;
            next;
        }
        problem("citation registry line $line_number has unknown record_type '$kind'");
    }
    close $fh;
    problem('the citation registry declares no ownership region') if !@regions;
    return (\@regions, \@citations);
}

sub run_self_test {
    my $checks = 0;
    my @failures;
    my $sample = <<'MARKDOWN';
intro
<!-- current_owners:start -->
- owned by [`OPEN-TREE`](docs/tasks/OPEN-TREE.md) and `CLOSED-TREE` before it
<!-- current_owners:end -->
tail [`OUTSIDE`](docs/tasks/OUTSIDE.md)
MARKDOWN

    @errors = ();
    my @units = region_units($sample, 'fixture.md', 'current_owners');
    push @failures, 'a link outside the region was counted' if grep { $_ eq 'OUTSIDE' } @units;
    push @failures, 'the linked unit inside the region was missed' if !grep { $_ eq 'OPEN-TREE' } @units;
    $checks++;

    @errors = ();
    region_units("no markers here\n", 'fixture.md', 'current_owners');
    push @failures, 'a document with no marker pair was accepted'
        if !grep { /must contain exactly one/ } @errors;
    $checks++;

    @errors = ();
    my $quoting = $sample;
    $quoting =~ s/\Qtail \E/tail `<!-- current_owners:start -->` and `<!-- current_owners:end -->` /;
    my @quoted_units = region_units($quoting, 'fixture.md', 'current_owners');
    push @failures, 'a backtick-quoted marker was counted as a real one'
        if grep { /must contain exactly one/ } @errors;
    push @failures, 'quoting the marker changed the extracted citations'
        if join(',', @quoted_units) ne join(',', @units);
    $checks++;

    @errors = ();
    region_units($sample . $sample, 'fixture.md', 'current_owners');
    push @failures, 'a duplicated marker pair was accepted'
        if !grep { /must contain exactly one/ } @errors;
    $checks++;

    @errors = ();
    region_units("<!-- current_owners:end -->\n<!-- current_owners:start -->\n", 'fixture.md', 'current_owners');
    push @failures, 'an inverted marker pair was accepted'
        if !grep { /end marker before its start marker/ } @errors;
    $checks++;

    @errors = ();
    my $bad_kind = read_registry_from_text(
        qq({"record_type":"region","path":"a.md","marker":"current_owners"}\n)
        . qq({"record_type":"citation","path":"a.md","unit":"X","kind":"maybe"}\n));
    push @failures, 'an unknown classification was accepted'
        if !grep { /must be current_owner or historical/ } @errors;
    $checks++;

    @errors = ();
    read_registry_from_text(
        qq({"record_type":"region","path":"a.md","marker":"current_owners"}\n)
        . qq({"record_type":"citation","path":"a.md","unit":"X","kind":"historical"}\n));
    push @failures, 'a historical classification with no reason was accepted'
        if !grep { /must state a reason/ } @errors;
    $checks++;

    @errors = ();
    read_registry_from_text(qq({"record_type":"citation","path":"a.md","unit":"X","kind":"current_owner"}\n));
    push @failures, 'a registry with no declared region was accepted'
        if !grep { /declares no ownership region/ } @errors;
    $checks++;

    # Grammar cases pinned from the tracked population rather than invented.
    my %shapes = (
        'plain tree status'     => "# T\n\n- Status: `active`\n",
        'emphasised tree status' => "# T\n\n- Status: **`active`** - UN-PARKED `2026-06-05`\n",
        'annotated leaf status' => "# T\n\n- ID: `T.1`\n  Status: `done` (`2026-08-31`, PROBE/DOC)\n",
    );
    for my $name (sort keys %shapes) {
        my $unit = $name eq 'annotated leaf status' ? 'T.1' : 'T';
        my $parsed = $name eq 'annotated leaf status'
            ? ($shapes{$name} =~ /^- ID: `\Q$unit\E`\s*\n\s*Status: [*_]{0,2}`([a-z_]+)`/m ? $1 : undef)
            : ($shapes{$name} =~ /^- Status: [*_]{0,2}`([a-z_]+)`/m ? $1 : undef);
        push @failures, "the $name shape did not parse" if !defined $parsed;
        $checks++;
    }

    @errors = ();
    die "ownership-citations self-test: $_\n" for @failures;
    my $expected = 11;
    die "ownership-citations self-test: ran $checks checks, declaration expects $expected; "
        . "re-derive the declaration beside the suite\n"
        if $checks != $expected;
    print "ownership-citations: self-test $checks/$expected grammar and registry cases pass.\n";
}

sub read_registry_from_text {
    my ($text) = @_;
    my $temporary = File::Spec->catfile($root, 'generated', ".ownership-citations-self-test.$$");
    mkdir File::Spec->catdir($root, 'generated');
    open my $fh, '>:encoding(UTF-8)', $temporary or die "self-test: cannot write fixture: $!\n";
    print {$fh} $text;
    close $fh;
    my @result = read_registry($temporary);
    unlink $temporary;
    return @result;
}
