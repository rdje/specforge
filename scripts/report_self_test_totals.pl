#!/usr/bin/env perl
# scripts/report_self_test_totals.pl — the producer for every published self-test case count.
#
# CLAIM-VERIFICATION-ADOPTION.7.3. A shell check's self-test count is a CURRENT ACTIONABLE ASSERTION
# the moment a document states it, and three of them were stale under a fully green gate:
# `TOOLBOX.md` §7.2a and §7.2a-i both said "sixteen fail-closed cases" for checks that had reached
# **22** and **21**, and `DOCTRINE_ENFORCEMENT.md` §10 repeated the first. Nothing observed it,
# because nothing bound the prose to the script.
#
# ── Why this producer reads the source instead of running the self-test ────────────────────────
# The obvious producer is the self-test itself. It is the wrong one HERE, for a measured reason:
# `scripts/rebuild_stage_cascade.sh --self-test` builds the `specforge` binary and takes **43.6 s**,
# and `check_published_assertions.pl` runs every derived producer on every gate run. Binding a doc
# sentence to a 43-second build would have moved the cost of the fix onto every commit.
#
# Reading the DECLARED total is sound because the declaration is self-guarding. Each script compares
# `passed` against that same constant and fails when they differ, so a case added without bumping the
# constant goes RED in the script's own self-test, and a constant bumped without adding a case goes
# RED there too. This producer therefore re-derives the number from its authority, and the authority
# re-derives it from the cases. The two legs compose; neither is a digest over the other
# (`CLAIM_VERIFICATION.md` §2 — a check and the thing it checks must not share a parent).
#
# ── Fail-closed, because a producer that shrugs is worse than none ─────────────────────────────
# A script that HAS a `--self-test` mode but whose total this reader cannot extract is an error, not
# an omission: silently dropping it would let a published count point at nothing while the report
# stays valid JSON. `--check` exits nonzero and names the script.
#
# Usage:
#   perl scripts/report_self_test_totals.pl            # one JSON object on stdout
#   perl scripts/report_self_test_totals.pl --check    # same, plus fail-closed on an unreadable total
#   perl scripts/report_self_test_totals.pl --self-test # this reader's own controls

use strict;
use warnings;
use File::Basename qw(dirname);
use File::Spec;
use File::Temp qw(tempdir);

my $LABEL = 'self-test-totals';
my $ROOT  = File::Spec->rel2abs(File::Spec->catdir(dirname(__FILE__), File::Spec->updir));

# The scripts whose self-test count a document may publish. A key is a stable slug, so a published
# assertion's `field` never has to carry a path with dots and slashes in it.
my %SCRIPTS = (
    proof_seal_currency  => 'scripts/check_proof_seal_currency.sh',
    chain_currency       => 'scripts/check_chain_currency.sh',
    rebuild_stage_cascade => 'scripts/rebuild_stage_cascade.sh',
);

# The three shapes a check in this repository declares its total in. Every one is the constant the
# script's own pass/fail comparison uses, never a count of matches — counting the cases here would be
# a second implementation of the thing being checked.
my @PATTERNS = (
    qr/\btotal=(\d+)\b/,                       # local work passed=0 total=21
    qr/self-test\s+\$passed\/(\d+)\b/,         # fail_note "self-test $passed/22 passed"
    qr/self-test\s+(\d+)\/\1\s+passed/,        # note 'self-test 14/14 passed.'
);

sub declared_total {
    my ($abs) = @_;
    open my $fh, '<:raw', $abs or return (undef, "cannot read $abs");
    my $text = do { local $/; <$fh> };
    close $fh;
    return (undef, 'declares no --self-test mode') if $text !~ /--self-test/;
    my %seen;
    for my $pattern (@PATTERNS) {
        while ($text =~ /$pattern/g) {
            $seen{$1} = 1;
        }
    }
    my @totals = sort { $a <=> $b } keys %seen;
    return (undef, 'no declared self-test total found') if !@totals;
    return (undef, 'declares more than one self-test total: ' . join(', ', @totals)) if @totals > 1;
    return ($totals[0], undef);
}

sub build_report {
    my ($root) = @_;
    my (%report, @errors);
    for my $key (sort keys %SCRIPTS) {
        my $rel = $SCRIPTS{$key};
        my $abs = File::Spec->catfile($root, $rel);
        if (!-f $abs) {
            push @errors, "$rel is registered here but absent";
            next;
        }
        my ($total, $why) = declared_total($abs);
        if (!defined $total) {
            push @errors, "$rel: $why";
            next;
        }
        $report{"${key}_self_test_total"} = $total + 0;
    }
    return (\%report, \@errors);
}

sub emit {
    my ($report) = @_;
    print '{';
    my @keys = sort keys %$report;
    for my $index (0 .. $#keys) {
        print ',' if $index;
        printf '"%s":%d', $keys[$index], $report->{$keys[$index]};
    }
    print "}\n";
}

sub run_self_test {
    my $passed = 0;
    my $total  = 5;
    my $work   = tempdir(CLEANUP => 1);

    my $write = sub {
        my ($name, $body) = @_;
        my $path = File::Spec->catfile($work, $name);
        open my $fh, '>:raw', $path or die "cannot write $path";
        print {$fh} $body;
        close $fh;
        return $path;
    };

    # 1) the `total=N` shape is read
    my ($value, $why) = declared_total($write->('a.sh', "--self-test\nlocal passed=0 total=21 out\n"));
    if (defined $value && $value == 21) { $passed++ }
    else { warn "$LABEL: self-test 1: total=N not read (" . ($why // 'wrong value') . ")\n" }

    # 2) the `self-test \$passed/N` shape is read
    ($value, $why) = declared_total($write->('b.sh', "--self-test\nfail_note \"self-test \$passed/22 passed\"\n"));
    if (defined $value && $value == 22) { $passed++ }
    else { warn "$LABEL: self-test 2: \$passed/N not read (" . ($why // 'wrong value') . ")\n" }

    # 3) the `self-test N/N passed` shape is read
    ($value, $why) = declared_total($write->('c.sh', "--self-test\nnote 'self-test 14/14 passed.'\n"));
    if (defined $value && $value == 14) { $passed++ }
    else { warn "$LABEL: self-test 3: N/N not read (" . ($why // 'wrong value') . ")\n" }

    # 4) TWO DIFFERENT totals in one script is a breach, not a pick — this is the case that keeps the
    #    reader from silently choosing one when a script is mid-edit.
    ($value, $why) = declared_total($write->('d.sh', "--self-test\ntotal=7\nnote 'self-test 9/9 passed.'\n"));
    if (!defined $value && ($why // '') =~ /more than one/) { $passed++ }
    else { warn "$LABEL: self-test 4: conflicting totals did not fail closed\n" }

    # 5) a self-test mode with NO readable total fails closed rather than being dropped
    ($value, $why) = declared_total($write->('e.sh', "--self-test\nrun_cases\n"));
    if (!defined $value && ($why // '') =~ /no declared self-test total/) { $passed++ }
    else { warn "$LABEL: self-test 5: an unreadable total did not fail closed\n" }

    if ($passed == $total) {
        print STDERR "$LABEL: self-test $total/$total passed.\n";
        return 0;
    }
    print STDERR "$LABEL: self-test $passed/$total passed\n";
    return 1;
}

my $mode = 'report';
for my $arg (@ARGV) {
    if    ($arg eq '--check')     { $mode = 'check' }
    elsif ($arg eq '--report')    { $mode = 'report' }
    elsif ($arg eq '--self-test') { $mode = 'self-test' }
    else {
        print STDERR "Usage: perl scripts/report_self_test_totals.pl [--report|--check|--self-test]\n";
        exit 2;
    }
}

exit run_self_test() if $mode eq 'self-test';

my ($report, $errors) = build_report($ROOT);
if (@$errors) {
    print STDERR "$LABEL: $_\n" for @$errors;
    print STDERR "$LABEL: FAILED with " . scalar(@$errors) . " violation(s).\n";
    exit 1 if $mode eq 'check';
    exit 1;
}
emit($report);
exit 0;
