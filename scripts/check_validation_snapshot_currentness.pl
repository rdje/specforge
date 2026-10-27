#!/usr/bin/env perl
# Validation snapshot currentness: native implementation
# xtask/src/checks/validation_snapshot_currentness.rs (RUSTDOC-AND-XTASK.15.n).
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $project_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my $root = $project_root;
my $contract_path = 'doctrine/live_document_size/validation_snapshot.json';
my $mode = 'check';
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        die "validation-snapshot-currentness: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "validation-snapshot-currentness: root does not exist\n" if !defined $root;
    } elsif ($arg eq '--contract') {
        die "validation-snapshot-currentness: --contract requires a path\n" if !@ARGV;
        $contract_path = shift @ARGV;
    } else {
        die "Usage: $0 [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n";
    }
}

my @command = qw(cargo xtask check validation-snapshot-currentness);
push @command, '--root', $root, '--contract', $contract_path, "--$mode";
$ENV{SPECFORGE_SHIM_PATH} = $0;
chdir $project_root or die "cannot enter $project_root: $!\n";
exec { $command[0] } @command;
