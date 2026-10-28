#!/usr/bin/env perl
# Live-document containment core: native implementation
# xtask/src/checks/live_document_size_core.rs (RUSTDOC-AND-XTASK.15.t).
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

my $root;
my $registry_rel = 'doctrine/live_document_size/surfaces.jsonl';
my $routes_rel = 'doctrine/readme_entrypoint/routed_destinations.tsv';
my $authorities_rel = 'doctrine/live_document_size/ceiling_increase_authorities.jsonl';
my $report = 0;
my $no_history = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--routes') {
        $routes_rel = shift @ARGV // usage();
    } elsif ($arg eq '--authorities') {
        $authorities_rel = shift @ARGV // usage();
    } elsif ($arg eq '--report') {
        $report = 1;
    } elsif ($arg eq '--no-history') {
        $no_history = 1;
    } else {
        usage();
    }
}

my $project_root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
$root //= $project_root;
$root = abs_path($root) // die "live-document-size: repository root does not exist\n";
my @command = qw(cargo xtask check live-document-size-core);
push @command, '--root', $root, '--registry', $registry_rel, '--routes', $routes_rel,
    '--authorities', $authorities_rel;
push @command, '--report' if $report;
push @command, '--no-history' if $no_history;
$ENV{SPECFORGE_SHIM_PATH} = $0;
chdir $project_root or die "cannot enter $project_root: $!\n";
exec { $command[0] } @command;

sub usage {
    die "Usage: $0 [--root DIR] [--registry PATH] [--routes PATH] [--authorities PATH] [--report] [--no-history]\n";
}
