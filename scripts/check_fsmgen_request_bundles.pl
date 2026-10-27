#!/usr/bin/env perl
# FSMGEN-REQUESTS.0 — enforce the signoff standard of docs/upstream/fsmgen/PROCESS.md on every request
# bundle: layout, summary fields, request.json, signoff boxes, verification on the pin and upstream,
# ownership (task leaf + open feedback record), and portability (no machine-local absolute path).
# ADR 0054 (UPSTREAM-CHANNEL-UNIFICATION.2): a question is a packet too, and the ledger indexes the packets —
# every outgoing open record has exactly one packet, cites its README, and agrees with it on kind and status.
# Native implementation: xtask/src/checks/fsmgen_request_bundles.rs (RUSTDOC-AND-XTASK.15.g).
# Run from anywhere: perl scripts/check_fsmgen_request_bundles.pl [--check|--self-test] [--root PROJECT_ROOT]
use strict;
use warnings;
use Cwd qw(abs_path);
use File::Basename qw(dirname);

my $root = abs_path(dirname(__FILE__) . '/..');
my $self_test = 0;
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        next;
    } elsif ($arg eq '--self-test') {
        $self_test = 1;
    } elsif ($arg eq '--root') {
        $root = abs_path(shift(@ARGV) // die usage());
    } else {
        die usage();
    }
}
sub usage { "Usage: $0 [--check|--self-test] [--root PROJECT_ROOT]\n" }

$root = '/' if !defined($root) || $root eq '';
my @command = qw(cargo xtask check fsmgen-request-bundles);
push @command, '--root', $root;
push @command, '--self-test' if $self_test;
my $cargo_root = -d $root ? $root : abs_path(dirname(__FILE__) . '/..');
chdir $cargo_root or die "cannot enter $cargo_root: $!\n";
exec { $command[0] } @command;
