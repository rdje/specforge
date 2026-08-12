#!/usr/bin/env perl
use strict;
use warnings;

use Cwd qw(abs_path);
use File::Find qw(find);
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

sub slurp_lines {
    my ($path) = @_;
    open my $handle, '<', $path or die "cannot read $path: $!\n";
    my @lines = <$handle>;
    close $handle or die "cannot close $path: $!\n";
    chomp @lines;
    return @lines;
}

sub relative_path {
    my ($base, $path) = @_;
    my $relative = File::Spec->abs2rel($path, $base);
    $relative =~ s{\\}{/}g;
    return $relative;
}

sub exact_set_differences {
    my ($actual, $declared) = @_;
    my @missing = sort grep { !exists $declared->{$_} } keys %{$actual};
    my @stale   = sort grep { !exists $actual->{$_} } keys %{$declared};
    return (\@missing, \@stale);
}

sub inspect_inventory {
    my ($project_root) = @_;
    my @problems;

    my $module_path = File::Spec->catfile(
        $project_root, qw(doctrine production_genericity module_inventory.tsv)
    );
    my @module_lines = slurp_lines($module_path);
    my $module_header = shift @module_lines // '';
    push @problems, 'module inventory header is not the schema-1 contract'
        if $module_header ne "path\tcurrent_plane\ttarget_plane\tprimary_lane\tdisposition";

    my %declared_modules;
    my %allowed_current_planes = map { $_ => 1 } qw(
        application conformance core_provider mixed core_foundation core core_grammar
        core_schema core_learning core_verifier core_registry core_inference core_lowering
        core_capture test_support
    );
    my %allowed_target_planes = map { $_ => 1 } qw(
        application conformance core_provider application+conformance core+conformance core core_rule
    );
    for my $line_number (2 .. @module_lines + 1) {
        my $line = $module_lines[$line_number - 2];
        my @fields = split /\t/, $line, -1;
        if (@fields != 5 || grep { $_ eq '' } @fields) {
            push @problems, "module inventory line $line_number must contain five non-empty TSV fields";
            next;
        }
        my ($path, $current_plane, $target_plane, $lane, undef) = @fields;
        push @problems, "module inventory line $line_number has a non-repository Rust path '$path'"
            if $path !~ m{^crates/specforge/src/(?:[A-Za-z0-9_./-]+)\.rs$}
                || $path =~ m{(?:^|/)\.\.(?:/|$)};
        push @problems, "module inventory line $line_number has invalid primary lane '$lane'"
            if $lane !~ /^e\.(?:ii|iii|iv)$/;
        push @problems,
            "module inventory line $line_number has unknown current plane '$current_plane'"
            if !$allowed_current_planes{$current_plane};
        push @problems,
            "module inventory line $line_number has unknown target plane '$target_plane'"
            if !$allowed_target_planes{$target_plane};
        if (exists $declared_modules{$path}) {
            push @problems, "module inventory duplicates '$path'";
        } else {
            $declared_modules{$path} = 1;
        }
    }

    my %actual_modules;
    my $source_root = File::Spec->catdir($project_root, qw(crates specforge src));
    find(
        {
            no_chdir => 1,
            wanted   => sub {
                return if !-f $_ || $_ !~ /\.rs\z/;
                $actual_modules{relative_path($project_root, $File::Find::name)} = 1;
            },
        },
        $source_root,
    );
    my ($missing_modules, $stale_modules) =
        exact_set_differences(\%actual_modules, \%declared_modules);
    push @problems, map { "Rust module is unclassified: $_" } @{$missing_modules};
    push @problems, map { "module inventory path is stale: $_" } @{$stale_modules};

    my $claim_path = File::Spec->catfile(
        $project_root, qw(doctrine production_genericity claim_family_inventory.tsv)
    );
    my @claim_lines = slurp_lines($claim_path);
    my $claim_header = shift @claim_lines // '';
    push @problems, 'claim-family inventory header is not the schema-1 contract'
        if $claim_header ne
        "family_id\tstage\ttop_level_fields\tcurrent_authority\ttarget_proof_class\tprimary_lane";

    my %allowed_stages = map { $_ => 1 } qw(source_ir evidence_ir semantic_ir intent_ir isf_adapter);
    my %declared_families;
    my %declared_claim_fields;
    for my $line_number (2 .. @claim_lines + 1) {
        my $line = $claim_lines[$line_number - 2];
        my @fields = split /\t/, $line, -1;
        if (@fields != 6 || grep { $_ eq '' } @fields) {
            push @problems, "claim-family inventory line $line_number must contain six non-empty TSV fields";
            next;
        }
        my ($family_id, $stage, $field_list, undef, undef, $lane) = @fields;
        push @problems, "claim-family inventory line $line_number has invalid family id '$family_id'"
            if $family_id !~ /^[a-z][a-z0-9_]*\.[a-z][a-z0-9_]*$/;
        push @problems, "claim-family inventory duplicates '$family_id'"
            if $declared_families{$family_id}++;
        push @problems, "claim-family inventory line $line_number has unknown stage '$stage'"
            if !$allowed_stages{$stage};
        my $expected_family_prefix = $stage eq 'isf_adapter' ? 'adapter' : $stage;
        $expected_family_prefix =~ s/_ir\z//;
        push @problems,
            "claim-family inventory line $line_number family '$family_id' does not match stage '$stage'"
            if $family_id !~ /^\Q$expected_family_prefix\E\./;
        push @problems, "claim-family inventory line $line_number has invalid primary lane '$lane'"
            if $lane !~ /^e\.(?:ii|iii|iv)$/;
        for my $field (split /,/, $field_list, -1) {
            if ($field !~ /^[a-z][a-z0-9_]*$/) {
                push @problems,
                    "claim-family inventory line $line_number has invalid field '$field'";
                next;
            }
            my $qualified = "$stage.$field";
            if (exists $declared_claim_fields{$qualified}) {
                push @problems,
                    "claim field '$qualified' appears in both '$declared_claim_fields{$qualified}' and '$family_id'";
            } else {
                $declared_claim_fields{$qualified} = $family_id;
            }
        }
    }

    my %struct_files = (
        source_ir    => [qw(crates specforge src ir source.rs), 'SourceIr'],
        evidence_ir  => [qw(crates specforge src ir evidence.rs), 'EvidenceIr'],
        semantic_ir  => [qw(crates specforge src ir semantic.rs), 'SemanticIr'],
        intent_ir    => [qw(crates specforge src ir intent.rs), 'IntentIr'],
        isf_adapter  => [qw(crates specforge src ir adapters.rs), 'AdapterArtifact'],
    );
    my %actual_claim_fields;
    for my $stage (sort keys %struct_files) {
        my ($path_parts, $struct_name);
        my @parts = @{$struct_files{$stage}};
        $struct_name = pop @parts;
        my $path = File::Spec->catfile($project_root, @parts);
        my @lines = slurp_lines($path);
        my $inside = 0;
        my $found = 0;
        for my $line (@lines) {
            if (!$inside && $line =~ /^pub struct \Q$struct_name\E \{\s*$/) {
                $inside = 1;
                $found = 1;
                next;
            }
            next if !$inside;
            last if $line =~ /^}\s*$/;
            if ($line =~ /^\s+pub ([A-Za-z][A-Za-z0-9_]*):/) {
                $actual_claim_fields{"$stage.$1"} = 1;
            }
        }
        push @problems, "could not locate top-level struct $struct_name in " . relative_path($project_root, $path)
            if !$found;
    }
    my ($missing_claims, $stale_claims) =
        exact_set_differences(\%actual_claim_fields, \%declared_claim_fields);
    push @problems, map { "top-level artifact field is unclassified: $_" } @{$missing_claims};
    push @problems, map { "claim-family inventory field is stale: $_" } @{$stale_claims};

    return (\@problems, scalar keys %actual_modules, scalar keys %declared_families,
        scalar keys %actual_claim_fields);
}

sub write_text {
    my ($path, $text) = @_;
    my (undef, $directory) = File::Spec->splitpath($path);
    make_path($directory);
    open my $handle, '>', $path or die "cannot write $path: $!\n";
    print {$handle} $text;
    close $handle or die "cannot close $path: $!\n";
}

sub self_test_fixture {
    my ($fixture) = @_;
    my @structs = (
        [source_ir   => 'source.rs',   'SourceIr',        'alpha'],
        [evidence_ir => 'evidence.rs', 'EvidenceIr',      'beta'],
        [semantic_ir => 'semantic.rs', 'SemanticIr',      'gamma'],
        [intent_ir   => 'intent.rs',   'IntentIr',        'delta'],
        [isf_adapter => 'adapters.rs', 'AdapterArtifact', 'epsilon'],
    );
    my @module_rows;
    my @claim_rows;
    for my $entry (@structs) {
        my ($stage, $file, $name, $field) = @{$entry};
        my $family_prefix = $stage eq 'isf_adapter' ? 'adapter' : $stage;
        $family_prefix =~ s/_ir\z//;
        my $relative = "crates/specforge/src/ir/$file";
        write_text(File::Spec->catfile($fixture, split m{/}, $relative),
            "pub struct $name {\n    pub $field: String,\n}\n");
        push @module_rows, "$relative\tcore\tcore\te.iii\tfixture";
        push @claim_rows,
            "$family_prefix.family\t$stage\t$field\tfixture\tintegrity_attestation\te.iii";
    }
    write_text(
        File::Spec->catfile($fixture, qw(doctrine production_genericity module_inventory.tsv)),
        "path\tcurrent_plane\ttarget_plane\tprimary_lane\tdisposition\n"
            . join("\n", @module_rows) . "\n",
    );
    write_text(
        File::Spec->catfile($fixture, qw(doctrine production_genericity claim_family_inventory.tsv)),
        "family_id\tstage\ttop_level_fields\tcurrent_authority\ttarget_proof_class\tprimary_lane\n"
            . join("\n", @claim_rows) . "\n",
    );
}

sub run_self_tests {
    my ($project_root) = @_;
    my $fixture = File::Spec->catdir(
        $project_root, 'generated', "production-genericity-inventory-tests.$$"
    );
    die "unsafe self-test path\n"
        if $fixture !~ m{/generated/production-genericity-inventory-tests\.[0-9]+\z};
    remove_tree($fixture) if -e $fixture;
    my $failure;
    eval {
        self_test_fixture($fixture);

        my ($clean) = inspect_inventory($fixture);
        die "self-test clean fixture failed: @{$clean}\n" if @{$clean};

        write_text(
            File::Spec->catfile($fixture, qw(crates specforge src ir unclassified.rs)),
            "pub struct Unclassified;\n",
        );
        my ($unclassified) = inspect_inventory($fixture);
        die "self-test did not reject an unclassified module\n"
            if !grep { /Rust module is unclassified/ } @{$unclassified};
        unlink File::Spec->catfile($fixture, qw(crates specforge src ir unclassified.rs));

        my $source = File::Spec->catfile($fixture, qw(crates specforge src ir source.rs));
        write_text($source, "pub struct SourceIr {\n    pub alpha: String,\n    pub new_field: String,\n}\n");
        my ($field_drift) = inspect_inventory($fixture);
        die "self-test did not reject an unclassified artifact field\n"
            if !grep { /top-level artifact field is unclassified/ } @{$field_drift};
    };
    $failure = $@;
    remove_tree($fixture) if -e $fixture;
    die "self-test residue remains at $fixture\n" if -e $fixture;
    die $failure if $failure;
    print "production-genericity-inventory self-test: 3/3 pass\n";
}

$root //= abs_path(File::Spec->catdir($Bin, '..'));
$root = abs_path($root) // die "cannot resolve project root\n";

if ($self_test) {
    run_self_tests($root);
    exit 0;
}

my ($problems, $module_count, $family_count, $field_count) = inspect_inventory($root);
if (@{$problems}) {
    print STDERR "production-genericity-inventory: $_\n" for @{$problems};
    exit 1;
}
print "production-genericity-inventory: $module_count modules, $family_count claim families, "
    . "$field_count top-level artifact fields are classified exactly once\n";
