#!/usr/bin/env perl
# Fact-card catalog: native implementation xtask/src/checks/fact_card_catalog.rs
# (RUSTDOC-AND-XTASK.15.v).
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use File::Spec;
use FindBin qw($Bin);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $project_root = abs_path(File::Spec->catdir($Bin, '..'))
    // die "fact-card-catalog: cannot resolve repository root\n";
my $root;
my $contract_rel = 'doctrine/live_document_size/fact_card_catalog.json';
my $mode = 'check';
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--write') {
        $mode = 'write';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--print-plan') {
        $mode = 'print-plan';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--contract') {
        $contract_rel = shift @ARGV // usage();
    } else {
        usage();
    }
}

my @command = qw(cargo xtask check fact-card-catalog);
if ($mode eq 'self-test') {
    push @command, '--self-test';
} else {
    $root //= $project_root;
    $root = abs_path($root) // die "fact-card-catalog: root does not exist\n";
    push @command, '--root', $root, '--contract', $contract_rel, "--$mode";
}
$ENV{SPECFORGE_SHIM_PATH} = $0;
chdir $project_root or die "cannot enter $project_root: $!\n";
exec { $command[0] } @command;

sub usage {
    die "Usage: $0 [--root DIR] [--contract PATH] "
        . "[--check|--write|--report|--print-plan|--self-test]\n";
}
