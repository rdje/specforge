#!/usr/bin/env perl
# scripts/check_production_genericity_inventory.pl — PRODUCTION-GENERICITY's INVENTORY component, native since
# RUSTDOC-AND-XTASK.15.b in `cargo xtask check production-genericity-inventory`
# (xtask/src/checks/production_genericity_inventory.rs), which documents the module and claim-family inventories
# and the self-test. This shim keeps the registered path, its Getopt::Long front end and the root resolution, so its
# refusals are as they were, and runs the native component from the repository root, where the `cargo xtask` alias
# resolves.
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Spec;
use FindBin qw($Bin);
use Getopt::Long qw(GetOptions);

my $root;
my $self_test = 0;
GetOptions(
    'root=s'    => \$root,
    'self-test' => \$self_test,
) or die "Usage: $0 [--root DIR] [--self-test]\n";

my $repository = abs_path(File::Spec->catdir($Bin, '..'));
$root //= $repository;
$root = abs_path($root) // die "cannot resolve project root\n";

my @native = ('--root', $root);
push @native, '--self-test' if $self_test;
chdir $repository or die "cannot enter the repository root $repository: $!\n";
exec { 'cargo' } qw(cargo xtask check production-genericity-inventory), @native;
die "cannot run cargo: $!\n";
