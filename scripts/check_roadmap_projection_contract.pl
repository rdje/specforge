#!/usr/bin/env perl
# Roadmap projection contract: native implementation
# xtask/src/checks/roadmap_projection_contract.rs (RUSTDOC-AND-XTASK.15.s).
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root;
my $contract_rel = 'doctrine/live_document_size/roadmap_projection.json';
my $report = 0;
my $self_test = 0;
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
    } else {
        usage();
    }
}

my $project_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my @command = qw(cargo xtask check roadmap-projection-contract);
if ($self_test) {
    push @command, '--self-test';
} else {
    $root //= $project_root;
    $root = abs_path($root) // die "roadmap-projection: repository root does not exist\n";
    push @command, '--root', $root, '--contract', $contract_rel;
    push @command, '--report' if $report;
}
$ENV{SPECFORGE_SHIM_PATH} = $0;
chdir $project_root or die "cannot enter $project_root: $!\n";
exec { $command[0] } @command;

sub usage {
    die "Usage: $0 [--root DIR] [--contract PATH] [--check|--report|--self-test]\n";
}
