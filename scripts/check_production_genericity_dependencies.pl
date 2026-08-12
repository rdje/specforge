#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Path qw(make_path remove_tree);
use File::Spec;
use FindBin qw($Bin);
use Getopt::Long qw(GetOptions);

my $root;
my $self_test = 0;
GetOptions(
    'root=s'    => \$root,
    'self-test' => \$self_test,
) or die "Usage: $0 [--root DIR] [--self-test]\n";

sub slurp {
    my ($path) = @_;
    open my $handle, '<', $path or die "cannot read $path: $!\n";
    local $/;
    my $text = <$handle>;
    close $handle or die "cannot close $path: $!\n";
    return $text;
}

sub write_text {
    my ($path, $text) = @_;
    my (undef, $directory) = File::Spec->splitpath($path);
    make_path($directory);
    open my $handle, '>', $path or die "cannot write $path: $!\n";
    print {$handle} $text;
    close $handle or die "cannot close $path: $!\n";
}

sub dependency_names {
    my ($manifest) = @_;
    my $text = slurp($manifest);
    my ($body) = $text =~ /^\[dependencies\]\s*\n(.*?)(?=^\[|\z)/ms;
    return () if !defined $body;
    return map { /^\s*([A-Za-z0-9_-]+)\s*=/ ? $1 : () } split /\n/, $body;
}

sub workspace_members {
    my ($manifest) = @_;
    my $text = slurp($manifest);
    my ($body) = $text =~ /^members\s*=\s*\[(.*?)\]/ms;
    return () if !defined $body;
    return $body =~ /"([^"]+)"/g;
}

sub exact_set_problems {
    my ($label, $actual, $expected) = @_;
    my %actual = map { $_ => 1 } @{$actual};
    my %expected = map { $_ => 1 } @{$expected};
    my @problems;
    push @problems, "$label is missing '$_'" for sort grep { !$actual{$_} } keys %expected;
    push @problems, "$label has unexpected member '$_'" for sort grep { !$expected{$_} } keys %actual;
    push @problems, "$label contains duplicate entries"
        if scalar(@{$actual}) != scalar(keys %actual);
    return @problems;
}

sub inspect_boundary {
    my ($project_root) = @_;
    my @problems;

    my @expected_members = qw(crates/specforge crates/specforge-core crates/specforge-conformance);
    my @members = workspace_members(File::Spec->catfile($project_root, 'Cargo.toml'));
    push @problems, exact_set_problems('workspace', \@members, \@expected_members);

    my %deps;
    my %manifest_text;
    for my $package (qw(specforge specforge-core specforge-conformance)) {
        my $manifest = File::Spec->catfile($project_root, 'crates', $package, 'Cargo.toml');
        $manifest_text{$package} = slurp($manifest);
        my @names = dependency_names($manifest);
        $deps{$package} = { map { $_ => 1 } @names };
    }

    push @problems, 'specforge-core must not depend on specforge-conformance'
        if $deps{'specforge-core'}{'specforge-conformance'};
    push @problems, 'specforge-core manifest names or aliases specforge-conformance'
        if $manifest_text{'specforge-core'} =~ /specforge[_-]conformance/;
    push @problems, 'specforge-core must not depend on the specforge application'
        if $deps{'specforge-core'}{'specforge'};
    push @problems, 'specforge-conformance must depend on specforge-core'
        if !$deps{'specforge-conformance'}{'specforge-core'};
    push @problems, 'specforge-conformance must not depend on the specforge application'
        if $deps{'specforge-conformance'}{'specforge'};
    push @problems, 'specforge application must depend on specforge-core'
        if !$deps{'specforge'}{'specforge-core'};
    push @problems, 'specforge application must depend on specforge-conformance'
        if !$deps{'specforge'}{'specforge-conformance'};

    my $core_root = slurp(
        File::Spec->catfile($project_root, qw(crates specforge-core src lib.rs))
    );
    push @problems, 'core package root references the conformance package'
        if $core_root =~ /specforge[_-]conformance/;
    push @problems, 'core package root directly includes a conformance/oracle source module'
        if $core_root =~ m{(?:eval\.rs|ir/(?:completeness|source_to_intent_eval|source_to_intent_replay|trajectory)\.rs|test_support/trajectory_snapshot\.rs)};

    my $core_ir_registry = slurp(
        File::Spec->catfile($project_root, qw(crates specforge src ir mod.rs))
    );
    push @problems, 'core IR registry declares a conformance/oracle module'
        if $core_ir_registry =~ /^\s*pub\s+mod\s+(?:completeness|source_to_intent_eval|source_to_intent_replay|trajectory)\s*;/m;

    return (\@problems, \%deps);
}

sub write_fixture {
    my ($fixture) = @_;
    write_text(
        File::Spec->catfile($fixture, 'Cargo.toml'),
        "[workspace]\nmembers = [\"crates/specforge\", \"crates/specforge-core\", \"crates/specforge-conformance\"]\n",
    );
    write_text(
        File::Spec->catfile($fixture, qw(crates specforge Cargo.toml)),
        "[package]\nname = \"specforge\"\n[dependencies]\nspecforge-core = { path = \"../specforge-core\" }\nspecforge-conformance = { path = \"../specforge-conformance\" }\n",
    );
    write_text(
        File::Spec->catfile($fixture, qw(crates specforge-core Cargo.toml)),
        "[package]\nname = \"specforge-core\"\n[dependencies]\nserde = \"1\"\n",
    );
    write_text(
        File::Spec->catfile($fixture, qw(crates specforge-conformance Cargo.toml)),
        "[package]\nname = \"specforge-conformance\"\n[dependencies]\nspecforge-core = { path = \"../specforge-core\" }\n",
    );
    write_text(
        File::Spec->catfile($fixture, qw(crates specforge-core src lib.rs)),
        "#[path = \"../../specforge/src/ir/mod.rs\"]\npub mod ir;\n",
    );
    write_text(
        File::Spec->catfile($fixture, qw(crates specforge src ir mod.rs)),
        "pub mod source;\n",
    );
}

sub run_self_tests {
    my ($project_root) = @_;
    my $fixture = File::Spec->catdir(
        $project_root, 'generated', "production-genericity-dependency-tests.$$"
    );
    die "unsafe self-test path\n"
        if $fixture !~ m{/generated/production-genericity-dependency-tests\.[0-9]+\z};
    remove_tree($fixture) if -e $fixture;
    my $failure;
    eval {
        write_fixture($fixture);
        my ($clean) = inspect_boundary($fixture);
        die "self-test clean fixture failed: @{$clean}\n" if @{$clean};

        my $core_manifest = File::Spec->catfile(
            $fixture, qw(crates specforge-core Cargo.toml)
        );
        write_text(
            $core_manifest,
            slurp($core_manifest)
                . "specforge-conformance = { path = \"../specforge-conformance\" }\n",
        );
        my ($reverse_dependency) = inspect_boundary($fixture);
        die "self-test did not reject the reverse package dependency\n"
            if !grep { /must not depend on specforge-conformance/ } @{$reverse_dependency};
        write_fixture($fixture);

        write_text(
            $core_manifest,
            slurp($core_manifest)
                . "oracle = { package = \"specforge-conformance\", path = \"../specforge-conformance\" }\n",
        );
        my ($aliased_reverse_dependency) = inspect_boundary($fixture);
        die "self-test did not reject an aliased reverse package dependency\n"
            if !grep { /manifest names or aliases specforge-conformance/ } @{$aliased_reverse_dependency};
        write_fixture($fixture);

        my $conformance_manifest = File::Spec->catfile(
            $fixture, qw(crates specforge-conformance Cargo.toml)
        );
        write_text(
            $conformance_manifest,
            slurp($conformance_manifest) . "specforge = { path = \"../specforge\" }\n",
        );
        my ($application_cycle) = inspect_boundary($fixture);
        die "self-test did not reject the conformance-to-application dependency\n"
            if !grep { /must not depend on the specforge application/ } @{$application_cycle};
        write_fixture($fixture);

        my $registry = File::Spec->catfile($fixture, qw(crates specforge src ir mod.rs));
        write_text($registry, slurp($registry) . "pub mod trajectory;\n");
        my ($oracle_in_core) = inspect_boundary($fixture);
        die "self-test did not reject a conformance module in the core registry\n"
            if !grep { /core IR registry declares/ } @{$oracle_in_core};
    };
    $failure = $@;
    remove_tree($fixture) if -e $fixture;
    die "self-test residue remains at $fixture\n" if -e $fixture;
    die $failure if $failure;
    print "production-genericity-dependencies self-test: 5/5 pass\n";
}

$root //= abs_path(File::Spec->catdir($Bin, '..'));
$root = abs_path($root) // die "cannot resolve project root\n";

if ($self_test) {
    run_self_tests($root);
    exit 0;
}

my ($problems, $deps) = inspect_boundary($root);
if (@{$problems}) {
    print STDERR "production-genericity-dependencies: $_\n" for @{$problems};
    exit 1;
}
print "production-genericity-dependencies: core -> conformance absent; "
    . "conformance -> core and application -> {core, conformance} present\n";
