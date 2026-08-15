# Behavioral held-out qualification — corrected and production-remediated result

- Date: `2026-08-15`
- Owner: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii` with remediation closure in `.f.iii.a`
- Production revision: `2cdcd1311c71e308cedcd244b9a3bc4d6a215bad`
- Frozen prospective population: 17 identity-disjoint documents
- Machine evidence: `doctrine/production_genericity/behavioral_holdout_evidence.json`
- Evidence SHA-256: `06e2358c199ca80559811be0fee38c368892ec21a46603fe3648a12fb9cd68a1`

## Qualification result

The corrected conformance oracle and production remediation close all 51 declared attempts with no failed or
invalid run. The two full-PDF relations qualify all 17 prospective documents, and the one measurable normalized-
text alpha relation now passes:

| Relation | Pass | Fail | Unmeasurable | Invalid | Descriptive 95% Wilson interval |
| --- | ---: | ---: | ---: | ---: | ---: |
| unchanged PDF replay | 17 | 0 | 0 | 0 | `0.815682–1.000000` |
| adversarial document identity | 17 | 0 | 0 | 0 | `0.815682–1.000000` |
| normalized-text symbol alpha | 1 | 0 | 16 | 0 | `0.206549–1.000000` |

The interval denominator is pass plus fail. Invalid and unmeasurable attempts never enter it. These intervals
describe this frozen, non-random population only; they are not claims about specifications outside it. The
absent prospective `cpu-isa` and `register-ip` categories remain explicit zero-denominator strata.

## Oracle repairs

The adversarial-identity allowance now normalizes `stable_artifact_stem` only for that relation. The semantic-
role mutation control still rejects, and all 17 byte-identical PDFs pass the complete SourceIR→ISF comparison.

Alpha authority now comes only from typed signal declarations. It no longer admits actor, state, transition,
member, module, or ordinary role-bearing prose values. Fifteen non-vacuous documents have no independently typed
opaque signal surface and receive `eligible_symbol_surface_absent`; the OpenCAPI AFU row remains
`vacuous_baseline`. Neither disposition creates a favorable denominator.

The I2C document supplies six eligible declarations: `SDA`, `SCL`, `USCL`, `USDA`, `SDAH`, and `SCLH`. The
deterministic transform changes all 352 occurrences, is case-fold unique and bijective, and changes lexical
ordering. A derived-id pairing conflict initially classified this completed comparison as invalid. That was a
second comparator defect: ambiguity in downstream derived ids is a semantic delta, not proof that the source
rename recipe is non-bijective. The final comparator retains only independently bijective derived-id pairs and
leaves every ambiguous value visible to ordinary fail comparison.

## Original production finding

The valid six-signal I2C alpha pair originally failed below SourceIR. SourceIR was invariant with 27/27 proof claims. EvidenceIR
changes by 292 undeclared paths and loses two claims (`2,166 → 2,164`). The delta propagates through SemanticIR
(`2,628 → 2,618`, 3,152 paths), IntentIR (`3,121 → 3,101`, 4,599 paths), and ISF (`3,228 → 3,201`, 4,398 paths).
Across all five stages the pair compares 266,194 leaves and has 11,170 baseline versus 11,111 transformed proof
claims. This is real production symbol-spelling/ordering coupling, not an oracle or transform defect.

The exact failing attempt report is SHA-256
`2d933561a424f6e2ac386d9400666f7789be3f4f0f2687110dc3708cf335fce4`. The aggregate records all six expected
and observed rename deltas and classifies the attempt `undeclared_semantic_delta`.

## Final evidence and leakage integrity

The aggregate contains exactly 17 documents, 51 attempts, and 90 relation/stratum rows. It accounts for 824,172
baseline and transformed proof claims, 19,745,620 compared leaves, and all six expected and observed deltas.
Execution provenance is explicit: all 34 full-capture relations and the eligible I2C alpha relation are fresh
pipeline executions under production revision `2cdcd131`; the other 16 alpha rows are retained eligibility
preflights. No prior-revision full-capture report is revalidated or relabeled as current-revision evidence.

The retained chain is fail-closed. Every aggregate link is digest-checked; every carried report digest is checked;
every referenced source and stage artifact remains repository-relative and on the repository filesystem; and all
five persisted artifact types are reloaded through their current typed proof validators. The initial failed
aggregate remains durable in Git history. Held-out labels, outcomes, uncertainty, execution modes, and thresholds
remain conformance-only and cannot steer canonical extraction.

The checker reconstructs split identity, novelty, attempt coverage, execution provenance, aggregate counts,
stratum denominators, dispositions, detail records, and Wilson limits. Its 17/17 mutation suite passes and now
requires fresh execution for both full-capture relations. Final production-genericity signoff remains deferred:
`.f.iv` owns complete reviewed-population reconciliation before `.f.v` closure.

## Remediation diagnostic

The failure was not one heuristic. Four independently alpha-sensitive mechanisms composed:

1. EvidenceIR split grammar words on underscores. The transformed `USDA` alias therefore occupied several words
   inside a bounded drive-to-logic-value window, and the later `HIGH` no longer bound to the signal. Treating a
   complete underscore identifier as one grammar token restores the exact ninth signal constraint.
2. Actor-signal relation records were generated from a lexically sorted declaration catalog. The transform
   deliberately reverses lexical rank, so sequential relation ids attached to different normalized signals.
   Exact-first/unique-case-folded resolution is retained, but emission follows first source occurrence.
3. The decomposition-title grammar saw the familiar component `reset_n` inside a complete declared alias and
   minted a section candidate. Removing complete declared signal tokens before universal topic grammar preserves
   real independent `reset control` headings while making alias spelling inert.
4. Transaction signal sets, connectivity-conflict ordering/ordinals, and residual overlap presentation inherited
   raw `String` ordering. A shared source-occurrence order now governs these source-owned identities.

Focused positive and negative controls pass for all four boundaries. A worktree replay of the exact retained
pair now preserves equal stage record counts and all 11,170 baseline/transformed proof claims across 266,194
compared leaves. SourceIR and EvidenceIR compare cleanly. The remaining five SemanticIR paths, 78 IntentIR paths,
and one ISF path were comparator-owned representation: lowercased source-derived id references, set-valued
responsibilities, and the unordered top-level named-drive declaration block.

The production change moves exactly two retained current chains under fixed-input ADR 0025 replay. Arm Debug
Interface v6 reassigns six actor-signal ordinals across `SWCLK`/`SWDIO` and `TDI`/`TDO`; I2C reassigns two across
`SCL`/`SDA`. Matching provenance rows and downstream source-order/conflict/residual presentations follow those
corrected assignments. Every stage record count and proof-claim count remains exact, both adapters retain the same
blocked/no-file outcome, and the exhaustive gate reports 24/24 current with zero stale at all four replayed stages.

## Clean-revision closure

The comparator now projects renamed fragments to lowercase only when they occur inside source-derived identifiers;
standalone prose and declared signals retain their source spelling. It canonicalizes responsibility/reference sets
after symbol normalization and sorts only top-level ISF interface/named-drive declarations, never ordered
transaction steps. Non-bijective scalar and list-shaped derived ids remain visible semantic deltas.

An exact clean-revision I2C replay passes all five stages with 11,170/11,170 claims, 266,194 compared leaves, and
zero undeclared paths. The first aggregate publication attempt then correctly invalidated 34 old full-capture
reports: retained artifacts from a prior production revision cannot be relabeled as `2cdcd131`. The executor now
offers an explicit full-capture refresh mode that reruns unchanged/adversarial PDF relations while retaining only
the already-frozen alpha-eligibility authority. The resulting 51-attempt aggregate is 35 pass / zero fail / 16
unmeasurable / zero invalid and has SHA-256
`06e2358c199ca80559811be0fee38c368892ec21a46603fe3648a12fb9cd68a1`.
