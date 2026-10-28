#!/usr/bin/env perl
# Active-task evidence: native implementation
# xtask/src/checks/active_task_evidence.rs (RUSTDOC-AND-XTASK.15.x).
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root;
my $contract_rel = 'doctrine/live_document_size/active_task_evidence.json';
my $report = 0;
my $self_test = 0;
my $write = 0;
my $migrate_template_rel;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--contract') {
        $contract_rel = shift @ARGV // usage();
    } elsif ($arg eq '--check') {
    } elsif ($arg eq '--report') {
        $report = 1;
    } elsif ($arg eq '--self-test') {
        $self_test = 1;
    } elsif ($arg eq '--write') {
        $write = 1;
    } elsif ($arg eq '--migrate') {
        $migrate_template_rel = shift @ARGV // usage();
    } else {
        usage();
    }
}

my $project_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'))
    // die "active-task-evidence: cannot resolve repository root\n";
usage() if defined($migrate_template_rel) && ($self_test || $report || $write);
usage() if $write && ($self_test || $report);

my @command = qw(cargo xtask check active-task-evidence);
if ($self_test) {
    push @command, '--self-test';
} else {
    $root //= $project_root;
    $root = abs_path($root) // die "active-task-evidence: root does not exist\n";
    push @command, '--root', $root, '--contract', $contract_rel;
    push @command, '--report' if $report;
    push @command, '--write' if $write;
    push @command, '--migrate', $migrate_template_rel if defined $migrate_template_rel;
}
$ENV{SPECFORGE_SHIM_PATH} = $0;
chdir $project_root or die "cannot enter $project_root: $!\n";
exec { $command[0] } @command;

sub usage {
    die "Usage: $0 [--root DIR] [--contract PATH] [--check|--report|--write|--self-test|--migrate ROOT_TEMPLATE]\n";
}
