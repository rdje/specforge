#!/usr/bin/env perl
# scripts/check_persisted_artifact_paths.pl — the persisted-artifact path contract (ARTIFACT-PATH-PORTABILITY),
# native since RUSTDOC-AND-XTASK.14.g in `cargo xtask check persisted-artifact-paths`
# (xtask/src/checks/persisted_artifact_paths.rs), which documents the scan, the producer contract, the self-test and
# the legacy-root migration. PROJECT-DATA-LOCALITY runs the native self-test and check in-process.
#
# This shim keeps the tool's registered path, its argument loop and usage refusal, its output layers (so a refusal
# exits with the status Perl's `die` leaves) and the root resolution, then passes the native tool a normalized
# command line from the repository root, where the `cargo xtask` alias resolves.
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use File::Spec;
use FindBin qw($Bin);

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $repository = abs_path(File::Spec->catdir($Bin, '..'));
my $root = $repository;
my $mode = 'check';
my $legacy_root;
my @external_source_roots;
my $execute = 0;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        die "persisted-artifact-paths: --root requires a path\n" if !@ARGV;
        $root = abs_path(shift @ARGV);
        die "persisted-artifact-paths: root does not exist\n" if !defined $root;
    } elsif ($arg eq '--migrate-legacy-root') {
        die "persisted-artifact-paths: --migrate-legacy-root requires a path\n" if !@ARGV;
        $legacy_root = shift @ARGV;
        $mode = 'migrate';
    } elsif ($arg eq '--external-source-root') {
        die "persisted-artifact-paths: --external-source-root requires a path\n" if !@ARGV;
        push @external_source_roots, shift @ARGV;
    } elsif ($arg eq '--execute') {
        $execute = 1;
    } else {
        die usage();
    }
}

die "persisted-artifact-paths: --execute requires --migrate-legacy-root\n"
    if $execute && $mode ne 'migrate';

my @native = ('--mode', $mode, '--root', $root);
push @native, '--legacy', $legacy_root if defined $legacy_root;
push @native, '--external', $_ for @external_source_roots;
push @native, '--execute' if $execute;
chdir $repository or die "persisted-artifact-paths: cannot enter the repository root $repository: $!\n";
exec { 'cargo' } qw(cargo xtask check persisted-artifact-paths), @native;
die "persisted-artifact-paths: cannot run cargo: $!\n";

sub usage {
    return <<'USAGE';
Usage: scripts/check_persisted_artifact_paths.pl [--check|--report|--self-test]
       scripts/check_persisted_artifact_paths.pl --migrate-legacy-root ABSOLUTE_PATH
           --external-source-root ABSOLUTE_PATH [--external-source-root ...] [--execute]
USAGE
}
