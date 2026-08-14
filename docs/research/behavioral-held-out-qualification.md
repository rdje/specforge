# Behavioral held-out qualification — corrected oracle result

- Date: `2026-08-14`
- Owner: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii`
- Production revision: `5dd1302a7fbb21519d9b428f177f7a681c9639df`
- Frozen prospective population: 17 identity-disjoint documents
- Machine evidence: `doctrine/production_genericity/behavioral_holdout_evidence.json`
- Evidence SHA-256: `fbc8331bac463bc400c079873406e5ffe9388a73c513a4811a6499edab5a4e06`

## Qualification result

The corrected conformance oracle closes all 51 declared attempts with no invalid run. It qualifies the two
full-PDF relations over all 17 prospective documents and exposes one real normalized-text alpha failure:

| Relation | Pass | Fail | Unmeasurable | Invalid | Descriptive 95% Wilson interval |
| --- | ---: | ---: | ---: | ---: | ---: |
| unchanged PDF replay | 17 | 0 | 0 | 0 | `0.815682–1.000000` |
| adversarial document identity | 17 | 0 | 0 | 0 | `0.815682–1.000000` |
| normalized-text symbol alpha | 0 | 1 | 16 | 0 | `0.000000–0.793451` |

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

## Production finding

The valid six-signal I2C alpha pair fails below SourceIR. SourceIR is invariant with 27/27 proof claims. EvidenceIR
changes by 292 undeclared paths and loses two claims (`2,166 → 2,164`). The delta propagates through SemanticIR
(`2,628 → 2,618`, 3,152 paths), IntentIR (`3,121 → 3,101`, 4,599 paths), and ISF (`3,228 → 3,201`, 4,398 paths).
Across all five stages the pair compares 266,194 leaves and has 11,170 baseline versus 11,111 transformed proof
claims. This is real production symbol-spelling/ordering coupling, not an oracle or transform defect.

The exact failing attempt report is SHA-256
`2d933561a424f6e2ac386d9400666f7789be3f4f0f2687110dc3708cf335fce4`. The aggregate records all six expected
and observed rename deltas and classifies the attempt `undeclared_semantic_delta`.

## Evidence and leakage integrity

The aggregate contains exactly 17 documents, 51 attempts, and 90 relation/stratum rows. It accounts for 824,172
baseline proof claims, 824,113 transformed proof claims, 19,745,620 compared leaves, and all six expected and
observed deltas. Execution provenance is explicit: 34 full-capture reports were retained-report revalidated,
16 alpha rows were eligibility preflights, and the eligible I2C alpha pair was a fresh five-stage pipeline run.

The retained chain is fail-closed. Every aggregate link is digest-checked; every carried report digest is checked;
every referenced source and stage artifact remains repository-relative and on the repository filesystem; and all
five persisted artifact types are reloaded through their current typed proof validators. The initial failed
aggregate remains durable in Git history. Held-out labels, outcomes, uncertainty, execution modes, and thresholds
remain conformance-only and cannot steer canonical extraction.

The checker reconstructs split identity, novelty, attempt coverage, execution provenance, aggregate counts,
stratum denominators, dispositions, detail records, and Wilson limits. Its 17/17 mutation suite passes. Final
production-genericity signoff remains deferred: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii.a` owns remediation of the
measured I2C symbol-alpha coupling before whole-population replay.
