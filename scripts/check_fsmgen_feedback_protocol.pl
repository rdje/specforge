#!/usr/bin/env perl
# FSMGen feedback protocol: native implementation
# xtask/src/checks/fsmgen_feedback_protocol.rs (RUSTDOC-AND-XTASK.15.r).
use strict;
use warnings;
use utf8;

use Cwd qw(abs_path);
use File::Basename qw(dirname);
use File::Spec;

my $project_root = abs_path(File::Spec->catdir(dirname(__FILE__), '..'));
my $root = $project_root;
my $contract_path = 'doctrine/live_document_size/fsmgen_feedback.json';
my $mode = 'check';
while (@ARGV) {
    my $arg = shift @ARGV;
    if ($arg eq '--root') {
        @ARGV or usage();
        $root = abs_path(shift @ARGV) // die "feedback-protocol: invalid --root\n";
    } elsif ($arg eq '--contract') {
        @ARGV or usage();
        $contract_path = shift @ARGV;
    } elsif ($arg eq '--check' || $arg eq '--report' || $arg eq '--self-test') {
        $mode = substr($arg, 2);
    } else {
        usage();
    }
}

my $contract_absolute = File::Spec->catfile($root, split(m{/}, $contract_path));
die "feedback-protocol: contract '$contract_path' is missing\n" if !-f $contract_absolute;
my @command = qw(cargo xtask check fsmgen-feedback-protocol);
push @command, '--root', $root, '--contract', $contract_path, "--$mode";
$ENV{SPECFORGE_SHIM_PATH} = $0;
chdir $project_root or die "cannot enter $project_root: $!\n";
exec { $command[0] } @command;

sub usage {
    die "Usage: scripts/check_fsmgen_feedback_protocol.pl [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n";
}
