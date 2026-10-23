#!/usr/bin/env perl
# scripts/check_corpus_frontier_census.pl — the CORPUS-FRONTIER doctrine.
#
# CORPUS-COVERAGE.4.1. "How much of the corpus is still unrefreshed" must be a DERIVED quantity that
# a declaration is checked against, never a number carried from slice to slice.
#
# Why this exists. The `.2` refresh program tracked its own remaining work as prose that each slice
# decremented by one — 27, 26, 25, ... 6, 5 — across twenty-two consecutive refreshes. A decrement
# cannot detect an error at its base, so a denominator adjustment recorded at `.2.29` (for the
# project's own `ingest README.md` artifact, a cohort member that later left `generated/`) outlived
# what it described and silently shortened the queue. `nvme_base_specification_2_0a_2021_07_26` — a
# 454-page specification with no exception recorded against it anywhere — simply stopped being
# counted. `CORPUS-COVERAGE.4.0` re-derived the census and restored it; this check is what stops the
# same class of drift returning.
#
# Archetype: DERIVE-AND-DIFF (DOCTRINE_ENFORCEMENT.md §3). The census is recomputed from persisted
# artifacts and compared with an exact declaration, so a disagreement is a failure rather than a
# discrepancy nobody is looking for.
#
# The census derives the cohort from `generated/source_ir/<key>/source_ir.json` and checks it against
# an exact, root-neutral lifecycle partition:
#   cohort     documents whose `source.requested_path` is NOT under a declared excluded prefix.
#              `corpus/` is the tracked in-repo gold/eval corpus — copied into the repository, never
#              part of the host-library refresh program.
#   refreshed  DECLARED completed document keys.
#   remaining  DECLARED unfinished document keys.
#
# The four checks, each catching a distinct failure:
#   1. IDENTITY      cohort == refreshed UNION remaining, the sets are disjoint, and all three counts
#                    match `expected`. Catches a count or partition that drifted in either direction.
#   2. MEMBERSHIP    every declared key exists in the cohort; a remaining key has NO retained bundle;
#                    every retained cohort key is declared refreshed.
#   3. OMISSION      every cohort member occurs in exactly one lifecycle set. This catches the original
#                    dropped-document defect without treating a workstation path as refresh evidence.
#   4. PROSE         the root task file states exactly the declared counts. The frontier a human or
#                    agent reads and the census the artifacts support cannot say different things.
#
# A source path deliberately does not participate in lifecycle classification. The library may move and
# stale references may be repaired without falsely completing a current-binary refresh. No document,
# vendor, protocol, workstation root, or volume name participates in executable logic (ADR 0006).
#
# Skips LOUDLY when the corpus root is absent: a fresh clone and a hosted CI runner have no
# `generated/`, and the doctrine does not govern them. Silence would read as a pass.
#
# Cost: a bounded prefix read of each SourceIR (the `source` object is the third top-level key), so
# roughly ten milliseconds for the whole corpus — gate-tier, unlike its CHAIN-CURRENCY sibling which
# needs a build and a full replay.
#
# Knobs: SPECFORGE_CORPUS_FRONTIER_CONTRACT=<file> to point at another declaration.
#        SPECFORGE_CORPUS_FRONTIER_ROOT=<dir>      to point at another corpus root.
#        Both are used by --self-test to exercise the core against fixtures.
#
# Native since RUSTDOC-AND-XTASK.14.a in `cargo xtask check corpus-frontier --census`
# (xtask/src/checks/corpus_frontier.rs). This shim keeps the path the records cite, its argument loop, its usage
# line and the SPECFORGE_CORPUS_FRONTIER_CONTRACT default; SPECFORGE_CORPUS_FRONTIER_ROOT passes through to the
# native check, which reads it as the script did.
use strict;
use warnings;
use Cwd qw(getcwd);
use File::Basename qw(dirname);

my $root;
my $contract_rel = $ENV{SPECFORGE_CORPUS_FRONTIER_CONTRACT} // 'doctrine/corpus_frontier/census.json';
my @modes;

while (@ARGV) {
    my $arg = shift @ARGV;
    if    ($arg eq '--root')      { $root = shift @ARGV // usage() }
    elsif ($arg eq '--contract')  { $contract_rel = shift @ARGV // usage() }
    elsif ($arg eq '--self-test') { push @modes, '--self-test' }
    elsif ($arg eq '--report')    { push @modes, '--report' }
    elsif ($arg eq '--check')     { }
    else                          { usage() }
}
my $repository = repository_root();
$root //= $repository;
my $caller = getcwd();

chdir $repository or die "corpus-frontier-census: cannot enter the repository root $repository: $!\n";
exec { 'cargo' } qw(cargo xtask check corpus-frontier), '--census', '--caller-dir', $caller, '--root', $root,
    '--contract', $contract_rel, @modes;
die "corpus-frontier-census: cannot run cargo xtask: $!\n";

sub usage {
    print STDERR "usage: check_corpus_frontier_census.pl [--root DIR] [--contract FILE] "
        . "[--check|--report|--self-test]\n";
    exit 2;
}

sub repository_root {
    my $dir = dirname(__FILE__) . '/..';
    my $abs = eval { require Cwd; Cwd::abs_path($dir) } // $dir;
    return $abs;
}
