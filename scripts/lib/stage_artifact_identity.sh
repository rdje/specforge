#!/usr/bin/env bash
# scripts/lib/stage_artifact_identity.sh — the ONE stage-artifact content-identity predicate.
#
# SOURCE-IR-REPRODUCIBILITY.15. Two callers need to answer "do these two stage artifacts carry the
# same content?": `scripts/check_chain_currency.sh` (the CHAIN-CURRENCY doctrine oracle, which asks
# it of a persisted artifact against a dry-run replay) and `scripts/rebuild_stage_cascade.sh` (the
# seal-restoration remedy, which asks it of a pre-write snapshot against the rebuilt artifact).
#
# `.11` established the lesson this file exists to obey: a second copy of a predicate is a predicate
# that can drift, and a remedy whose comparator disagrees with the gate's comparator would prove
# nothing. So the definition lives here once and both callers source it. Authority is unchanged —
# CHAIN-CURRENCY remains the currency ORACLE; the cascade only uses the same predicate to NAME which
# sections moved, so a delta can be attributed per ADR 0025 decision 1 rather than absorbed.
#
# Sourcing contract: this file defines functions only. It sets no options, changes no directory, and
# writes nothing, so a caller's `set` flags and traps survive it.
# Bash-3.2-safe, matching every entrypoint that sources it.

# ── The comparison core ─────────────────────────────────────────────────────
# compare_stage_artifact <persisted.json> <replay.json>
# Exit 0 when both carry the same content identity. On a difference, print the differing top-level
# sections as `name(persisted-cardinality->replay-cardinality)` and exit 1.
compare_stage_artifact() {
  perl - "$1" "$2" <<'PERL'
use strict;
use warnings;
use JSON::PP;

my ($persisted_path, $replay_path) = @ARGV;

sub load_artifact {
    my ($path) = @_;
    open my $handle, '<:raw', $path or die "cannot read $path: $!\n";
    local $/;
    my $text = <$handle>;
    close $handle;
    my $value = JSON::PP->new->utf8->decode($text);
    die "$path is not a JSON object\n" if ref $value ne 'HASH';
    # Back-annotated by `specforge validate` after the stage ran; the product's *_ir_fingerprint
    # helpers clear the same field before hashing, so it is not part of content identity.
    delete @{$value}{qw(validation_reports proof_context proof_ledger)};
    return $value;
}

sub cardinality {
    my ($artifact, $key) = @_;
    return 'absent' if !exists $artifact->{$key};
    my $field = $artifact->{$key};
    return scalar(@$field) if ref $field eq 'ARRAY';
    return scalar(keys %$field) if ref $field eq 'HASH';
    return 'scalar';
}

my $persisted = load_artifact($persisted_path);
my $replay    = load_artifact($replay_path);
my $encoder   = JSON::PP->new->utf8->canonical->allow_nonref;

my %sections;
$sections{$_} = 1 for (keys %$persisted, keys %$replay);

my @differing;
for my $key (sort keys %sections) {
    my $lhs = exists $persisted->{$key} ? $encoder->encode($persisted->{$key}) : undef;
    my $rhs = exists $replay->{$key}    ? $encoder->encode($replay->{$key})    : undef;
    next if !defined $lhs && !defined $rhs;
    next if defined $lhs && defined $rhs && $lhs eq $rhs;
    push @differing,
      sprintf('%s(%s->%s)', $key, cardinality($persisted, $key), cardinality($replay, $key));
}

exit 0 if !@differing;
print join(' ', @differing), "\n";
exit 1;
PERL
}

# compare_emitted_isf <replay-adapter.json> <persisted-artifact-root>
# Mirrors `AdapterArtifact::write_to_disk` + `reconcile_emitted_isf_files`: the persisted root holds
# exactly the `.isf` the replayed adapter licenses, with exactly its rendered text — no stale
# sibling, no missing emission, no drifted body. Prints the breach and exits 1 on any difference.
compare_emitted_isf() {
  perl - "$1" "$2" <<'PERL'
use strict;
use warnings;
use File::Basename qw(basename);
use File::Spec;
use JSON::PP;

my ($adapter_path, $artifact_root) = @ARGV;

open my $handle, '<:raw', $adapter_path or die "cannot read $adapter_path: $!\n";
local $/;
my $adapter = JSON::PP->new->utf8->decode(<$handle>);
close $handle;

my $isf          = $adapter->{isf};
my $renderable   = ($isf && $isf->{is_renderable}) ? 1 : 0;
my $emitted_path = $adapter->{artifact_layout}{emitted_target_path};

my $expected_name = ($renderable && defined $emitted_path) ? basename($emitted_path) : undef;
my $expected_text = $renderable ? $isf->{source_text} : undef;

opendir my $dir, $artifact_root or die "cannot read $artifact_root: $!\n";
my @present = sort grep { /\.isf\z/ } readdir $dir;
closedir $dir;

my @breaches;
if (!defined $expected_name) {
    push @breaches, "stale emitted .isf present but the current adapter renders none: @present"
      if @present;
} else {
    my @unexpected = grep { $_ ne $expected_name } @present;
    push @breaches, "unlicensed .isf sibling(s) retained: @unexpected" if @unexpected;

    my $expected_file = File::Spec->catfile($artifact_root, $expected_name);
    if (!-f $expected_file) {
        push @breaches, "the current adapter emits $expected_name but it is not persisted";
    } else {
        open my $emitted, '<:raw', $expected_file or die "cannot read $expected_file: $!\n";
        my $persisted_text = <$emitted>;
        close $emitted;
        $persisted_text = '' if !defined $persisted_text;
        my $rendered = defined $expected_text ? $expected_text : '';
        utf8::encode($rendered) if utf8::is_utf8($rendered);
        push @breaches, "$expected_name body differs from the rendered isf.source_text"
          if $persisted_text ne $rendered;
    }
}

exit 0 if !@breaches;
print join('; ', @breaches), "\n";
exit 1;
PERL
}
