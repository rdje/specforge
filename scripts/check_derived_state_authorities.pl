#!/usr/bin/env perl
# Derived-state verified-copy authorities: native implementation xtask/src/checks/derived_state_authorities.rs
# (RUSTDOC-AND-XTASK.15.j). Secondary copy paths and markers come only from the declared data plane.
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

my $root;
my $registry_rel = 'doctrine/live_document_size/derived_state_contracts.jsonl';
my $contract_id;

while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        $root = shift @ARGV // usage();
    } elsif ($arg eq '--registry') {
        $registry_rel = shift @ARGV // usage();
    } elsif ($arg eq '--contract') {
        $contract_id = shift @ARGV // usage();
    } else {
        usage();
    }
}

$root //= File::Spec->catdir(dirname(abs_path($0)), '..');
$root = abs_path($root) // die "derived-state-authority: repository root does not exist\n";
defined($contract_id) && $contract_id =~ /\A[a-z][a-z0-9._-]*\z/ or usage();

# Original index query: git', '-C', $root, 'ls-files', '--stage', '--', 'subs/fsmgen'.
my @command = qw(cargo xtask check derived-state-authorities);
push @command, '--root', $root, '--registry', $registry_rel, '--contract', $contract_id;
my $cargo_root = -d $root ? $root : abs_path(File::Spec->catdir(dirname(abs_path($0)), '..'));
chdir $cargo_root or die "cannot enter $cargo_root: $!\n";
exec { $command[0] } @command;

sub usage {
    die "Usage: $0 [--root DIR] [--registry PATH] --contract ID\n";
}
