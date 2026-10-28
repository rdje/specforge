#!/usr/bin/env perl
# Corpus KB currentness: native implementation
# xtask/src/checks/corpus_kb_currentness.rs (RUSTDOC-AND-XTASK.15.q).
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

binmode STDOUT, ':encoding(UTF-8)';
binmode STDERR, ':encoding(UTF-8)';

my $mode = 'check';
my $root = File::Spec->catdir(dirname(abs_path($0)), '..');
my $relative_contract = 'doctrine/live_document_size/corpus_kb.json';
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--check') {
        $mode = 'check';
    } elsif ($arg eq '--report') {
        $mode = 'report';
    } elsif ($arg eq '--self-test') {
        $mode = 'self-test';
    } elsif ($arg eq '--root') {
        @ARGV or die "--root requires a value\n";
        $root = shift @ARGV;
    } elsif ($arg eq '--contract') {
        @ARGV or die "--contract requires a value\n";
        $relative_contract = shift @ARGV;
    } else {
        die "Usage: $0 [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n";
    }
}

my $base = canonical_root($root);
die "corpus-kb-currentness: unsafe contract path\n"
    if !safe_relative_path($relative_contract, 512);
my @command = qw(cargo xtask check corpus-kb-currentness);
push @command, '--root', $base, '--contract', $relative_contract, "--$mode";
$ENV{SPECFORGE_SHIM_PATH} = $0;
chdir $base or die "cannot enter $base: $!\n";
exec { $command[0] } @command;

sub canonical_root {
    my ($path) = @_;
    my $absolute = abs_path($path);
    die "corpus-kb-currentness: repository root does not exist\n" if !defined $absolute || !-d $absolute;
    return $absolute;
}

sub safe_relative_path {
    my ($path, $max_bytes) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '' || length($path) > $max_bytes;
    return 0 if File::Spec->file_name_is_absolute($path) || $path =~ /(?:\A|\/)\.\.(?:\/|\z)/;
    return 0 if $path =~ /\0/ || $path =~ m{//};
    return 1;
}
