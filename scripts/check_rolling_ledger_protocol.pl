#!/usr/bin/env perl
# Rolling-ledger protocol: native implementation
# xtask/src/checks/rolling_ledger_protocol.rs (RUSTDOC-AND-XTASK.15.w).
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root;
my $registry_rel = 'doctrine/live_document_size/rolling_ledgers.jsonl';
my $surfaces_rel = 'doctrine/live_document_size/surfaces.jsonl';
my $report = 0;
my $self_test = 0;
my $emit_id;
my $output_rel;
my $rollover_plan_rel;
my $apply_rollover = 0;

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
    } elsif ($arg eq '--self-test') {
        $self_test = 1;
    } elsif ($arg eq '--emit-planned') {
        $emit_id = shift @ARGV // usage();
    } elsif ($arg eq '--output') {
        $output_rel = shift @ARGV // usage();
    } elsif ($arg eq '--rollover-plan') {
        $rollover_plan_rel = shift @ARGV // usage();
    } elsif ($arg eq '--apply-rollover') {
        $apply_rollover = 1;
    } else {
        usage();
    }
}

my $project_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
$root //= $project_root;
$root = abs_path($root) // die "rolling-ledger: repository root does not exist\n";
usage() if $apply_rollover && !defined($rollover_plan_rel);
usage() if defined($rollover_plan_rel) && ($report || $self_test || defined($emit_id) || defined($output_rel));

my @command = qw(cargo xtask check rolling-ledger-protocol);
push @command, '--root', $root, '--registry', $registry_rel, '--surfaces', $surfaces_rel;
push @command, '--report' if $report;
push @command, '--self-test' if $self_test;
push @command, '--emit-planned', $emit_id if defined $emit_id;
push @command, '--output', $output_rel if defined $output_rel;
push @command, '--rollover-plan', $rollover_plan_rel if defined $rollover_plan_rel;
push @command, '--apply-rollover' if $apply_rollover;
$ENV{SPECFORGE_SHIM_PATH} = $0;
chdir $project_root or die "cannot enter $project_root: $!\n";
exec { $command[0] } @command;

sub usage {
    die "Usage: $0 [--root DIR] [--registry PATH] [--surfaces PATH] [--report] [--self-test] "
        . "[--emit-planned LEDGER_ID --output SOURCE_PATH] "
        . "[--rollover-plan PATH [--apply-rollover]]\n";
}
