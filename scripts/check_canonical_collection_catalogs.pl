#!/usr/bin/env perl
# Canonical collection catalogs: native implementation xtask/src/checks/canonical_collection_catalogs.rs
# (RUSTDOC-AND-XTASK.15.i). Bounded complete membership views over declared partitioned-canonical surfaces.
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use File::Spec;
use FindBin qw($Bin);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $root = abs_path(File::Spec->catdir($Bin, '..'));
my $config_rel = 'doctrine/live_document_size/canonical_catalogs.jsonl';
my $surface_registry_rel = 'doctrine/live_document_size/surfaces.jsonl';
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
        die "canonical-catalogs: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "canonical-catalogs: root does not exist\n" if !defined $root;
    } elsif ($arg eq '--config') {
        die "canonical-catalogs: --config requires a repository-relative path\n" if !@ARGV;
        $config_rel = shift @ARGV;
    } elsif ($arg eq '--surfaces') {
        die "canonical-catalogs: --surfaces requires a repository-relative path\n" if !@ARGV;
        $surface_registry_rel = shift @ARGV;
    } else {
        die "Usage: $0 [--check|--write|--self-test] [--root PROJECT_ROOT]"
          . " [--config PATH] [--surfaces PATH]\n";
    }
}

my @command = qw(cargo xtask check canonical-collection-catalogs);
push @command, '--root', $root if defined $root;
push @command, '--config', $config_rel, '--surfaces', $surface_registry_rel, "--$mode";
my $cargo_root = -d $root ? $root : abs_path(File::Spec->catdir($Bin, '..'));
chdir $cargo_root or die "cannot enter $cargo_root: $!\n";
exec { $command[0] } @command;
