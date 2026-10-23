#!/usr/bin/env perl
use strict;
use warnings;

# scripts/check_published_assertions.pl — CLAIM-VERIFICATION-ADOPTION.7.1.
#
# The gate CLAIM-VERIFICATION-ADOPTION.7.0 froze. A digest-bound live document proves only that a region has
# not changed; it never proves that the numbers inside it still re-derive. Ten recorded instances in
# docs/tasks/CLAIM-VERIFICATION-ADOPTION.md went stale under a fully green gate for exactly that reason, and
# five consecutive rounds of prose correction were each invalidated by their own transaction.
#
# This checker binds a published value in a governed region to a producer field and RE-EXECUTES the producer.
# Every element below exists because a specific recorded instance defeated the alternative:
#   * execute-and-compare        — instances 1-6 and 10: values false while every digest control was green.
#   * four outcomes, no fifth    — `derived` / `gated` / `authored` / `dated`. There is deliberately no outcome
#                                  for "a trajectory shows it has held" (instance 8 disproved that licence).
#   * population derived on read — three commits in a row moved their own published population by describing it,
#                                  so a stored surface list is wrong at the moment it lands.
#   * excludes_self              — .11's ownership screen went green for the three surfaces it named BECAUSE it
#                                  named them: a check the act of writing satisfies shares a parent with the
#                                  thing it checks.
#   * cross-surface disagreement — instance 5: two surfaces published different values for one quantity and
#                                  nothing noticed. Costs one comparison and no producer run.
#   * membership as enumeration  — a set claim's size is not its content; instances 4, 5 and .11a are set claims
#                                  published without their enumeration or with a stale one.
# Mechanism claims are deliberately OUT of scope (.10): no checker can decide whether two accounts predict the
# same observation. Records may carry `adjudicated_against` to name the prior ruling a mechanism was checked
# against — shape, not truth.
#
# Native since RUSTDOC-AND-XTASK.13.e in `cargo xtask check published-assertions`
# (xtask/src/checks/published_assertions.rs). This shim keeps the doctrine's registered path, its argument handling
# and its root resolution, so the usage line and a missing root exit exactly as they did, then runs the native check
# from the repository root, where the `cargo xtask` alias resolves.

use strict;
use warnings;
use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

my $LABEL = 'published-assertions';
my $script_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my $root;
my $contract_rel = 'doctrine/claim_verification/published_assertions.jsonl';
my $mode = 'check';

while (@ARGV) {
    my $arg = shift @ARGV;
    if    ($arg eq '--root')      { $root = shift @ARGV // usage(); }
    elsif ($arg eq '--contract')  { $contract_rel = shift @ARGV // usage(); }
    elsif ($arg eq '--check')     { $mode = 'check'; }
    elsif ($arg eq '--report')    { $mode = 'report'; }
    elsif ($arg eq '--produce')   { $mode = 'produce'; }
    elsif ($arg eq '--self-test') { $mode = 'self-test'; }
    else { usage(); }
}
$root //= $script_root;
$root = abs_path($root) // die "$LABEL: repository root does not exist\n";

chdir $script_root or die "$LABEL: cannot enter the repository root $script_root: $!\n";
exec { 'cargo' } qw(cargo xtask check published-assertions), '--root', $root, '--contract', $contract_rel, "--$mode";
die "$LABEL: cannot run cargo xtask: $!\n";

sub usage {
    print STDERR "Usage: scripts/check_published_assertions.pl [--root DIR] [--contract PATH] "
        . "[--check|--report|--produce|--self-test]\n";
    exit 2;
}
