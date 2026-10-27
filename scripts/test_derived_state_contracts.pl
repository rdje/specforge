#!/usr/bin/env perl
# The 47 derived-state contract checks: native implementation
# xtask/src/checks/derived_state_contract_tests.rs (RUSTDOC-AND-XTASK.15.l).
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

my $root = abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
my $quiet = 0;
if (@ARGV) {
    die "Usage: $0 [--quiet]\n" if @ARGV != 1 || $ARGV[0] ne '--quiet';
    $quiet = 1;
}

my @command = qw(cargo xtask check derived-state-contract-tests);
push @command, '--quiet' if $quiet;
chdir $root or die "cannot enter $root: $!\n";
exec { $command[0] } @command;
