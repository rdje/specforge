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

my $CLAIM_HEADER = join "\t", qw(
    family_id stage top_level_fields current_authority target_proof_class primary_lane
);
my $RULE_HEADER = join "\t", qw(
    family_id stage rule_stem current_producer_entrypoints current_mutator_entrypoints
    premise_kinds symbol_capability alpha_obligation compatibility target_module schema_owner
    canonical_seams migration_leaf
);
my $BYPASS_HEADER = join "\t", qw(
    entrypoint stage current_mutation current_sink target_disposition migration_leaf
);

my %ALLOWED_PREMISE = map { $_ => 1 } qw(
    source_span table_cell visual_region upstream_claim registered_derivation
    grounded_model_proposal validated_prior universal_axiom
);
my %CAPABILITY_ALPHA = (
    symbol_blind       => 'byte_identical_non_symbol_output',
    exact_identity_only => 'identity_graph_invariant',
    grammar_introduces => 'introduced_symbols_preserve_origins',
    lossless_carry     => 'lossless_topology_invariant',
    merge_or_conflict  => 'merge_conflict_topology_invariant',
    residual           => 'residual_topology_invariant',
    target_lowering    => 'target_safe_renaming',
);
my %MIGRATION_LEAF = (
    source_ir   => '6d.ii.e.iv.ii',
    evidence_ir => '6d.ii.e.iv.iii',
    semantic_ir => '6d.ii.e.iv.iv',
    intent_ir   => '6d.ii.e.iv.v',
    isf_adapter => '6d.ii.e.iv.vi',
);

sub slurp {
    my ($path) = @_;
    open my $handle, '<', $path or die "cannot read $path: $!\n";
    local $/;
    my $text = <$handle>;
    close $handle or die "cannot close $path: $!\n";
    return $text;
}

sub slurp_lines {
    my ($path) = @_;
    my @lines = split /\n/, slurp($path), -1;
    pop @lines if @lines && $lines[-1] eq '';
    return @lines;
}

sub repository_path {
    my ($project_root, $relative) = @_;
    return if $relative =~ m{(?:^|/)\.\.(?:/|$)} || File::Spec->file_name_is_absolute($relative);
    return File::Spec->catfile($project_root, split m{/}, $relative);
}

sub function_name {
    my ($symbol) = @_;
    my @parts = split /::/, $symbol;
    return $parts[-1];
}

sub inspect_entrypoints {
    my ($project_root, $cell, $label, $line_number, $problems, $counter) = @_;
    return if $cell eq '-';
    my %seen;
    for my $entry (split /;/, $cell, -1) {
        if ($entry !~ m{^(crates/specforge/src/[A-Za-z0-9_./-]+\.rs)#([A-Za-z][A-Za-z0-9_:]*)$}) {
            push @{$problems}, "rule inventory line $line_number has malformed $label entry '$entry'";
            next;
        }
        my ($relative, $symbol) = ($1, $2);
        push @{$problems}, "rule inventory line $line_number duplicates $label entry '$entry'"
            if $seen{$entry}++;
        my $path = repository_path($project_root, $relative);
        if (!defined $path || !-f $path) {
            push @{$problems}, "rule inventory line $line_number $label path is missing: $relative";
            next;
        }
        my $name = function_name($symbol);
        my $source = slurp($path);
        push @{$problems},
            "rule inventory line $line_number $label symbol '$symbol' is absent from $relative"
            if $source !~ /\bfn\s+\Q$name\E\s*(?:<[^;{}()]*>)?\s*\(/s;
        ${$counter}++;
    }
}

sub target_module_path {
    my ($project_root, $module) = @_;
    return if $module !~ /^crate::ir::([a-z][a-z0-9_]*)$/;
    my $name = $1;
    my $file = File::Spec->catfile($project_root, qw(crates specforge src ir), "$name.rs");
    return $file if -f $file;
    my $mod = File::Spec->catfile($project_root, qw(crates specforge src ir), $name, 'mod.rs');
    return $mod if -f $mod;
    return;
}

sub inspect_rules {
    my ($project_root) = @_;
    my @problems;

    my $claim_path = File::Spec->catfile(
        $project_root, qw(doctrine production_genericity claim_family_inventory.tsv)
    );
    my @claim_lines = slurp_lines($claim_path);
    my $claim_header = shift @claim_lines // '';
    push @problems, 'claim-family inventory header is not the schema-1 contract'
        if $claim_header ne $CLAIM_HEADER;

    my %claims;
    for my $line_number (2 .. @claim_lines + 1) {
        my @fields = split /\t/, $claim_lines[$line_number - 2], -1;
        if (@fields != 6 || grep { $_ eq '' } @fields) {
            push @problems, "claim-family inventory line $line_number is malformed";
            next;
        }
        my ($family, $stage, $field_list) = @fields;
        $claims{$family} = {
            stage  => $stage,
            fields => [split /,/, $field_list, -1],
        };
    }

    my $rule_path = File::Spec->catfile(
        $project_root, qw(doctrine production_genericity rule_family_inventory.tsv)
    );
    my @rule_lines = slurp_lines($rule_path);
    my $rule_header = shift @rule_lines // '';
    push @problems, 'rule-family inventory header is not the schema-1 contract'
        if $rule_header ne $RULE_HEADER;

    my %declared_families;
    my %expanded_rules;
    my $entrypoint_count = 0;
    my $seam_count = 0;
    for my $line_number (2 .. @rule_lines + 1) {
        my @fields = split /\t/, $rule_lines[$line_number - 2], -1;
        if (@fields != 13 || grep { $_ eq '' } @fields) {
            push @problems,
                "rule inventory line $line_number must contain 13 non-empty TSV fields";
            next;
        }
        my (
            $family, $stage, $rule_stem, $producers, $mutators, $premises, $capability,
            $alpha, $compatibility, $target_module, $schema_owner, $seams, $migration_leaf
        ) = @fields;

        push @problems, "rule inventory duplicates family '$family'"
            if $declared_families{$family}++;
        my $claim = $claims{$family};
        if (!$claim) {
            push @problems, "rule inventory has unknown family '$family'";
            next;
        }
        push @problems, "rule inventory family '$family' stage '$stage' disagrees with claim inventory"
            if $stage ne $claim->{stage};
        push @problems, "rule inventory family '$family' must use rule stem '$family'"
            if $rule_stem ne $family;
        push @problems, "rule inventory family '$family' has invalid rule stem '$rule_stem'"
            if $rule_stem !~ /^[a-z][a-z0-9_]*\.[a-z][a-z0-9_]*$/;
        push @problems, "rule inventory family '$family' has wrong migration leaf '$migration_leaf'"
            if !exists $MIGRATION_LEAF{$stage} || $migration_leaf ne $MIGRATION_LEAF{$stage};

        my %premise_seen;
        for my $premise (split /,/, $premises, -1) {
            push @problems, "rule inventory family '$family' has unknown premise '$premise'"
                if !$ALLOWED_PREMISE{$premise};
            push @problems, "rule inventory family '$family' duplicates premise '$premise'"
                if $premise_seen{$premise}++;
        }
        push @problems, "rule inventory family '$family' has unknown capability '$capability'"
            if !exists $CAPABILITY_ALPHA{$capability};
        push @problems,
            "rule inventory family '$family' capability '$capability' requires alpha obligation '"
                . ($CAPABILITY_ALPHA{$capability} // '<unknown>') . "'"
            if exists $CAPABILITY_ALPHA{$capability} && $alpha ne $CAPABILITY_ALPHA{$capability};
        push @problems, "rule inventory family '$family' has invalid compatibility '$compatibility'"
            if $compatibility ne 'current_only' && $compatibility ne 'lossless_carry';
        push @problems,
            "rule inventory family '$family' may use lossless compatibility only with lossless capability"
            if ($compatibility eq 'lossless_carry') != ($capability eq 'lossless_carry');

        my $target_path = target_module_path($project_root, $target_module);
        push @problems, "rule inventory family '$family' target module is absent: $target_module"
            if !defined $target_path;
        my $schema_path = repository_path($project_root, $schema_owner);
        push @problems, "rule inventory family '$family' schema owner is absent: $schema_owner"
            if !defined $schema_path || !-f $schema_path;

        inspect_entrypoints(
            $project_root, $producers, 'producer', $line_number, \@problems, \$entrypoint_count
        );
        inspect_entrypoints(
            $project_root, $mutators, 'mutator', $line_number, \@problems, \$entrypoint_count
        );
        inspect_entrypoints(
            $project_root, $seams, 'canonical seam', $line_number, \@problems, \$seam_count
        );

        for my $field (@{$claim->{fields}}) {
            my $rule_id = "$rule_stem.$field.v1";
            push @problems, "expanded rule id is invalid or overlong: $rule_id"
                if length($rule_id) > 128
                    || $rule_id !~ /^[a-z0-9_.-]+$/
                    || $rule_id !~ /\./;
            push @problems, "expanded rule id is duplicated: $rule_id"
                if $expanded_rules{$rule_id}++;
        }
    }

    for my $family (sort keys %claims) {
        push @problems, "claim family lacks a rule migration row: $family"
            if !$declared_families{$family};
    }
    for my $required (
        'crates/specforge/src/ir/source.rs#SourceIr::write_to_disk',
        'crates/specforge/src/ir/evidence.rs#EvidenceIr::write_to_disk',
        'crates/specforge/src/ir/semantic.rs#SemanticIr::write_to_disk',
        'crates/specforge/src/ir/intent.rs#IntentIr::write_to_disk',
        'crates/specforge/src/ir/adapters.rs#AdapterArtifact::write_to_disk',
        'crates/specforge/src/commands/validate.rs#write_backannotated_artifact',
    ) {
        push @problems, "canonical write seam is not inventoried: $required"
            if !grep { /(?:^|;)\Q$required\E(?:;|\z)/ } map { (split /\t/, $_, -1)[11] } @rule_lines;
    }

    my $bypass_path = File::Spec->catfile(
        $project_root, qw(doctrine production_genericity conformance_bypass_inventory.tsv)
    );
    my @bypass_lines = slurp_lines($bypass_path);
    my $bypass_header = shift @bypass_lines // '';
    push @problems, 'conformance-bypass inventory header is not the schema-1 contract'
        if $bypass_header ne $BYPASS_HEADER;
    my %declared_bypasses;
    my $bypass_entrypoint_count = 0;
    for my $line_number (2 .. @bypass_lines + 1) {
        my @fields = split /\t/, $bypass_lines[$line_number - 2], -1;
        if (@fields != 6 || grep { $_ eq '' } @fields) {
            push @problems,
                "conformance-bypass line $line_number must contain six non-empty TSV fields";
            next;
        }
        my ($entrypoint, $stage, undef, undef, $disposition, $migration_leaf) = @fields;
        my $key = "$entrypoint\t$stage";
        push @problems, "conformance-bypass inventory duplicates '$key'"
            if $declared_bypasses{$key}++;
        push @problems, "conformance-bypass line $line_number has unknown stage '$stage'"
            if !exists $MIGRATION_LEAF{$stage};
        push @problems, "conformance-bypass line $line_number has wrong migration leaf '$migration_leaf'"
            if exists $MIGRATION_LEAF{$stage} && $migration_leaf ne $MIGRATION_LEAF{$stage};
        push @problems, "conformance-bypass line $line_number lacks a fail-closed target disposition"
            if $disposition !~ /^(?:noncanonical typed overlay cannot call canonical write or (?:lowering|downstream production build)|verified persistence rebase preserves proof bytes and grants no semantic mutation)$/;
        inspect_entrypoints(
            $project_root, $entrypoint, 'conformance bypass', $line_number,
            \@problems, \$bypass_entrypoint_count
        );
    }
    my $kg_bench = File::Spec->catfile(
        $project_root, qw(crates specforge src commands kg_bench.rs)
    );
    if (-f $kg_bench) {
        my @required_bypasses = (
            "crates/specforge/src/commands/kg_bench.rs#run_fixture\tsource_ir",
            "crates/specforge/src/commands/kg_bench.rs#run_fixture\tevidence_ir",
            "crates/specforge/src/commands/kg_bench.rs#run_fixture\tsemantic_ir",
            "crates/specforge/src/commands/eval_extraction.rs#extract_on_copy\tevidence_ir",
        );
        push @problems, map { "known conformance bypass is unclassified: $_" }
            grep { !$declared_bypasses{$_} } @required_bypasses;
    }

    return (
        \@problems,
        scalar(keys %declared_families),
        scalar(keys %expanded_rules),
        $entrypoint_count,
        $seam_count,
        scalar(keys %declared_bypasses),
    );
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
    my @stages = (
        [source_ir   => source   => 'SourceIr'],
        [evidence_ir => evidence => 'EvidenceIr'],
        [semantic_ir => semantic => 'SemanticIr'],
        [intent_ir   => intent   => 'IntentIr'],
        [isf_adapter => adapter  => 'AdapterArtifact'],
    );
    my (@claim_rows, @rule_rows);
    for my $entry (@stages) {
        my ($stage, $prefix, $type) = @{$entry};
        my $file = $stage eq 'isf_adapter' ? 'adapters.rs' : "$prefix.rs";
        my $relative = "crates/specforge/src/ir/$file";
        write_text(
            File::Spec->catfile($fixture, split m{/}, $relative),
            "pub struct $type {\n    pub alpha: String,\n}\nimpl $type {\n    pub fn build() {}\n    pub fn write_to_disk() {}\n}\n",
        );
        my $family = "$prefix.family";
        push @claim_rows, "$family\t$stage\talpha\tfixture\tintegrity_attestation\te.iv";
        my $leaf = $MIGRATION_LEAF{$stage};
        push @rule_rows, join "\t",
            $family, $stage, $family, "$relative#$type\::build", '-', 'universal_axiom',
            'exact_identity_only', 'identity_graph_invariant', 'current_only',
            "crate::ir::" . ($stage eq 'isf_adapter' ? 'adapters' : $prefix), $relative,
            "$relative#$type\::write_to_disk", $leaf;
    }
    my $validate = 'crates/specforge/src/commands/validate.rs';
    write_text(
        File::Spec->catfile($fixture, split m{/}, $validate),
        "fn write_backannotated_artifact() {}\n",
    );
    $rule_rows[0] =~ s{\t6d\.ii\.e\.iv\.ii\z}{;$validate#write_backannotated_artifact\t6d.ii.e.iv.ii};
    write_text(
        File::Spec->catfile($fixture, qw(doctrine production_genericity claim_family_inventory.tsv)),
        "$CLAIM_HEADER\n" . join("\n", @claim_rows) . "\n",
    );
    write_text(
        File::Spec->catfile($fixture, qw(doctrine production_genericity rule_family_inventory.tsv)),
        "$RULE_HEADER\n" . join("\n", @rule_rows) . "\n",
    );
    write_text(
        File::Spec->catfile(
            $fixture, qw(doctrine production_genericity conformance_bypass_inventory.tsv)
        ),
        "$BYPASS_HEADER\n",
    );
}

sub run_self_tests {
    my ($project_root) = @_;
    my $fixture = File::Spec->catdir(
        $project_root, 'generated', "production-genericity-rule-tests.$$"
    );
    die "unsafe self-test path\n"
        if $fixture !~ m{/generated/production-genericity-rule-tests\.[0-9]+\z};
    remove_tree($fixture) if -e $fixture;
    my $failure;
    eval {
        self_test_fixture($fixture);
        my ($clean) = inspect_rules($fixture);
        die "clean fixture failed: @{$clean}\n" if @{$clean};

        my $rules = File::Spec->catfile(
            $fixture, qw(doctrine production_genericity rule_family_inventory.tsv)
        );
        my $original = slurp($rules);
        my @lines = split /\n/, $original;
        splice @lines, 2, 1;
        write_text($rules, join("\n", @lines) . "\n");
        my ($missing) = inspect_rules($fixture);
        die "missing-family mutation was admitted\n"
            if !grep { /lacks a rule migration row/ } @{$missing};

        write_text($rules, $original =~ s/#SourceIr::build/#SourceIr::absent/r);
        my ($absent) = inspect_rules($fixture);
        die "missing-entrypoint mutation was admitted\n"
            if !grep { /symbol 'SourceIr::absent' is absent/ } @{$absent};

        write_text($rules, $original =~ s/identity_graph_invariant/target_safe_renaming/r);
        my ($alpha) = inspect_rules($fixture);
        die "capability-alpha mutation was admitted\n"
            if !grep { /requires alpha obligation/ } @{$alpha};

        write_text($rules, $original =~ s/source\.family\tsource_ir\tsource\.family/source.family\tsource_ir\tevidence.family/r);
        my ($stem) = inspect_rules($fixture);
        die "rule-stem mutation was admitted\n"
            if !grep { /must use rule stem/ } @{$stem};

        write_text($rules, $original);
        my $bypasses = File::Spec->catfile(
            $fixture, qw(doctrine production_genericity conformance_bypass_inventory.tsv)
        );
        write_text(
            $bypasses,
            "$BYPASS_HEADER\n"
                . "crates/specforge/src/ir/source.rs#SourceIr::build\tsource_ir\tfixture mutation"
                . "\tfixture sink\tallows canonical write\t6d.ii.e.iv.ii\n",
        );
        my ($bypass) = inspect_rules($fixture);
        die "unsafe conformance-bypass disposition was admitted\n"
            if !grep { /lacks a fail-closed target disposition/ } @{$bypass};
    };
    $failure = $@;
    remove_tree($fixture) if -e $fixture;
    die "self-test residue remains at $fixture\n" if -e $fixture;
    die $failure if $failure;
    print "production-genericity-rules self-test: 6/6 pass\n";
}

$root //= abs_path(File::Spec->catdir($Bin, '..'));
$root = abs_path($root) // die "cannot resolve project root\n";

if ($self_test) {
    run_self_tests($root);
    exit 0;
}

my ($problems, $families, $rules, $entrypoints, $seams, $bypasses) = inspect_rules($root);
if (@{$problems}) {
    print STDERR "production-genericity-rules: $_\n" for @{$problems};
    exit 1;
}
print "production-genericity-rules: $families families expand to $rules field rules; "
    . "$entrypoints producer/mutator entrypoints, $seams canonical seams, and "
    . "$bypasses conformance-only bypass obligations resolve\n";
