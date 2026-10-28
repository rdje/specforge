#!/usr/bin/env perl
# Source PDF registry currentness: native implementation
# xtask/src/checks/source_pdf_registry_currentness.rs (RUSTDOC-AND-XTASK.15.p).
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

my $mode = 'check';
my $root = File::Spec->catdir(dirname(abs_path($0)), '..');
my $relative_contract = 'doctrine/live_document_size/source_pdf_registry.json';
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
die "source-pdf-registry-currentness: unsafe contract path\n"
    if !safe_relative_path($relative_contract, 512);
my @command = qw(cargo xtask check source-pdf-registry-currentness);
push @command, '--root', $base, '--contract', $relative_contract, "--$mode";
$ENV{SPECFORGE_SHIM_PATH} = $0;
chdir $base or die "cannot enter $base: $!\n";
exec { $command[0] } @command;

sub canonical_root {
    my ($path) = @_;
    my $absolute = abs_path($path);
    die "source-pdf-registry-currentness: repository root does not exist: $path\n"
        if !defined($absolute) || !-d $absolute;
    return $absolute;
}

sub safe_relative_path {
    my ($path, $max_bytes) = @_;
    return 0 if !defined($path) || ref($path) || $path eq '';
    return 0 if length($path) > $max_bytes;
    return 0 if File::Spec->file_name_is_absolute($path) || $path =~ /[\\\r\n\0`|]/;
    my @parts = split m{/}, $path, -1;
    return 0 if grep { $_ eq '' || $_ eq '.' || $_ eq '..' } @parts;
    return 1;
}
