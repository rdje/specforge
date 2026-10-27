#!/usr/bin/env perl
# Derived-state field contracts: native implementation xtask/src/checks/derived_state_contracts.rs
# (RUSTDOC-AND-XTASK.15.k). Bounded registries, current-surface ownership, secondary copies and verifiers.
# Four classes: derive_on_read verified_copy authored_intent immutable_evidence.
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

my $root;
my $registry_rel = 'doctrine/live_document_size/derived_state_contracts.jsonl';
my $surfaces_rel = 'doctrine/live_document_size/surfaces.jsonl';
my $report = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--surfaces') {
        $surfaces_rel = shift @ARGV // usage();
    } elsif ($arg eq '--report') {
        $report = 1;
    } else {
        usage();
    }
}

$root //= File::Spec->catdir(dirname(abs_path($0)), '..');
$root = abs_path($root) // die "derived-state: repository root does not exist\n";
my @command = qw(cargo xtask check derived-state-contracts);
push @command, '--root', $root, '--registry', $registry_rel, '--surfaces', $surfaces_rel;
push @command, '--report' if $report;
my $cargo_root = -d $root ? $root : abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
chdir $cargo_root or die "cannot enter $cargo_root: $!\n";
exec { $command[0] } @command;

sub usage {
    die "Usage: $0 [--root DIR] [--registry PATH] [--surfaces PATH] [--report]\n";
}
