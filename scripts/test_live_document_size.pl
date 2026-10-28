#!/usr/bin/env perl
# The 126 live-document lifecycle and control-plane checks: native implementation
# xtask/src/checks/live_document_size_tests.rs (RUSTDOC-AND-XTASK.15.u).
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

my @command = qw(cargo xtask check live-document-size-tests);
push @command, '--quiet' if $quiet;
chdir $root or die "cannot enter $root: $!\n";
exec { $command[0] } @command;
