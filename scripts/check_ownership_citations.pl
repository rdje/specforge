#!/usr/bin/env perl
# check_ownership_citations.pl — refuse a current-owner citation that names a closed work unit.
#
# LIVE-DOCUMENT-PRESSURE-HEADROOM.15. Eight times a tracked document named a `done` or `superseded`
# tree as the current owner of live work, and the last one happened under full attention immediately
# after the class was documented. Review demonstrably does not hold this invariant, so a gate must.
# It proves that every work unit cited inside a declared ownership region is classified, and that every
# citation classified as a CURRENT OWNER names a unit whose own `Status:` line is still open; it cannot
# prove that the open owner is the RIGHT owner.
#
# Native since RUSTDOC-AND-XTASK.13.d in `cargo xtask check ownership-citations`
# (xtask/src/checks/ownership_citations.rs). This shim keeps the doctrine's registered path and its argument
# handling, so the usage line names it as invoked and a refused argument or a missing root exits exactly as it
# did; it then runs the native check from the repository root, where the `cargo xtask` alias resolves.
use strict;
use warnings;
use FindBin qw($Bin);
use File::Spec;
use Cwd qw(abs_path);

# The script's own output layers, kept so a `die` below exits with the same status the script's did (Perl's
# `die` exits with `$!`, which these leave set).
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

chdir $project_root or die "ownership-citations: cannot enter the repository root $project_root: $!\n";
exec { 'cargo' } qw(cargo xtask check ownership-citations), '--root', $root, '--registry', $registry_rel, "--$mode";
die "ownership-citations: cannot run cargo xtask: $!\n";
