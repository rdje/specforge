#!/usr/bin/env perl
# Knowledge Map shard contract: native implementation
# xtask/src/checks/knowledge_map_shard_contract.rs (RUSTDOC-AND-XTASK.15.m).
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Glob qw(bsd_glob GLOB_NOSORT);
use File::Spec;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $project_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my $root = $project_root;
my $contract_rel = 'doctrine/knowledge_map/shard_contract.json';
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
        die "knowledge-map-shards: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "knowledge-map-shards: root does not exist\n" if !defined $root;
    } elsif ($arg eq '--contract') {
        die "knowledge-map-shards: --contract requires a repository-relative path\n" if !@ARGV;
        $contract_rel = shift @ARGV;
    } else {
        die "Usage: $0 [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n";
    }
}

my @command = qw(cargo xtask check knowledge-map-shard-contract);
push @command, '--root', $root, '--contract', $contract_rel, "--$mode";
$ENV{SPECFORGE_SHIM_PATH} = $0;
chdir $project_root or die "cannot enter $project_root: $!\n";
exec { $command[0] } @command;
