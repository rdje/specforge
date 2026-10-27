#!/usr/bin/env perl
# Bounded task-tree catalog: native implementation xtask/src/checks/task_tree_catalog.rs
# (RUSTDOC-AND-XTASK.15.h). Open trees stay on the bounded landing; every real tree appears in exactly one part.
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use File::Spec;
use FindBin qw($Bin);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root = abs_path(File::Spec->catdir($Bin, '..'));
my $mode = 'check';
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--write') {
        $mode = 'write';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        die "task-tree-catalog: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "task-tree-catalog: root does not exist\n" if !defined $root;
    } else {
        die "Usage: $0 [--check|--write|--self-test] [--root PROJECT_ROOT]\n";
    }
}

my @command = qw(cargo xtask check task-tree-catalog);
push @command, '--root', $root if defined $root;
push @command, "--$mode";
my $cargo_root = -d $root ? $root : abs_path(File::Spec->catdir($Bin, '..'));
chdir $cargo_root or die "cannot enter $cargo_root: $!\n";
exec { $command[0] } @command;
