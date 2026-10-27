#!/usr/bin/env perl
# The 25 derived-state authority checks: native implementation
# xtask/src/checks/derived_state_authority_tests.rs (RUSTDOC-AND-XTASK.15.j).
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

my @command = qw(cargo xtask check derived-state-authority-tests);
push @command, '--quiet' if $quiet;
chdir $root or die "cannot enter $root: $!\n";
exec { $command[0] } @command;
