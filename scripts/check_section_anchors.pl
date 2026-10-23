#!/usr/bin/env perl
# scripts/check_section_anchors.pl — SECTION-ANCHORS doctrine (structural archetype), registered by
# LIVE-DOCUMENT-PRESSURE-HEADROOM.4f and native since RUSTDOC-AND-XTASK.13.a in `cargo xtask check section-anchors`
# (xtask/src/checks/section_anchors.rs): every qualified cross-document section reference, the repository's
# `<path>.md` §`<section>` form, resolves to a heading that exists in the file it names, so a content-preserving
# move cannot silently break a route.
#
# This shim keeps the doctrine's registered path and arguments (--check|--report|--self-test, --root DIR). It runs
# the native check from the repository root, where the `cargo xtask` alias resolves, so it first makes a --root
# directory absolute against the caller's working directory.

use strict;
use warnings;

use File::Spec;
use FindBin qw($Bin);

my $root = File::Spec->catdir($Bin, '..');
my @arguments = @ARGV;
for my $index (0 .. $#arguments - 1) {
    $arguments[$index + 1] = File::Spec->rel2abs($arguments[$index + 1]) if $arguments[$index] eq '--root';
}
chdir $root or die "section-anchors: cannot enter the repository root $root: $!\n";
exec { 'cargo' } qw(cargo xtask check section-anchors), @arguments;
die "section-anchors: cannot run cargo xtask: $!\n";
