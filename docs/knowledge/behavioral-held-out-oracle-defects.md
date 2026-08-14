---
id: behavioral-held-out-oracle-defects
title: Corrected behavioral holdout separates oracle defects from real I2C symbol-alpha coupling
answers:
  - "why did the initial prospective behavioral held-out run fail"
  - "what did real filenames expose in adversarial identity comparison"
  - "why are held-out alpha failures not yet production name-coupling evidence"
  - "which held-out document has an eligible opaque alpha catalog"
  - "what were the initial 17-document held-out relation outcomes"
  - "what are the corrected 17-document behavioral held-out outcomes"
  - "does I2C extraction remain invariant under opaque signal renaming"
  - "what caused the I2C symbol alpha production failure"
  - "why must opaque identifiers remain one grammar token"
  - "what stable order may production use for opaque signal collections"
  - "why do source derived ids need field aware alpha normalization"
  - "why is an ambiguous derived id pairing a fail rather than an invalid alpha transform"
date: 2026-08-14
status: current
tags: [genericity, held-out, metamorphic-testing, conformance, alpha-renaming]
evidence: docs/research/behavioral-held-out-initial-run.md; docs/research/behavioral-held-out-qualification.md; doctrine/production_genericity/behavioral_holdout_evidence.json; docs/tasks/spec-to-intent-alignment/behavioral-qualification.md; crates/specforge-conformance/src/behavioral_genericity.rs; scripts/check_behavioral_genericity_contract.py
reverify: "python3 -B scripts/check_behavioral_genericity_contract.py --self-test && cargo test --offline -p specforge-conformance behavioral_genericity::tests -- --nocapture"
---

The first frozen 17-document execution completed all 51 attempts. Unchanged PDF replay passed 17/17;
adversarial identity failed 17/17; alpha was 0 pass / 15 fail / one unmeasurable / one invalid. These are a
fail-closed diagnostic checkpoint, not behavioral genericity signoff.

Every adversarial failure is exactly the filename-derived `source.stable_artifact_stem` and its proof premise.
The contract already permits document/artifact identity to change; the comparator mapped document key and
filename but omitted this separately persisted derivative. The small calibration happened to use equal values
and could not expose the omission.

The alpha transform overreached before production ran. It combined explicit signal declarations with schema
actor/state/transition/member/base/module values and globally renamed matching source tokens. The heldout
population showed that this set includes meaningful role words such as `device`, `controller`, `source`,
`target`, `receiver`, and `host`. Their transformation changes intent, so downstream actor loss cannot diagnose
production name coupling. The I2C document is the only prospective row with an independently declared opaque
catalog: `SCL`, `SDA`, `USCL`, `USDA`, `SCLH`, and `SDAH`. Absence of such a catalog elsewhere must be reported
as unmeasurable, not pass, fail, or inferred authority.

The corrected oracle normalizes the filename-derived stem only for adversarial identity and derives alpha
authority only from typed signal declarations. The final prospective matrix is 34 pass / one fail / 16
unmeasurable / zero invalid: unchanged PDF and adversarial identity both pass 17/17; 15 rows have no eligible
opaque alpha surface, the OpenCAPI AFU row is vacuous, and the sole measurable I2C alpha pair fails.

The I2C transform bijectively renames six declared signals across 352 occurrences. SourceIR remains invariant,
but EvidenceIR loses two proof claims and has 292 undeclared paths; the difference propagates through SemanticIR,
IntentIR, and ISF. An initially ambiguous derived-id pairing is not transform invalidity: only the source rename
recipe determines transform bijection. The comparator now retains only unambiguous derived-id pairs and exposes
the remaining structure as an ordinary semantic fail.

The remediation localized four alpha-sensitive mechanisms. EvidenceIR's bounded verb/value grammar split on
underscores, so a long opaque alias became many grammar words and moved `HIGH` outside the accepted gap; keeping
underscore identifiers atomic restores the missing constraint and its claims. Actor-signal relations assigned
sequential ids after lexical signal sorting, so alpha renaming swapped otherwise identical edges; first source
occurrence now governs emission. SemanticIR's section-topic grammar interpreted `reset_n` inside a complete
declared alias as an independent `reset` topic; declared signal tokens are now removed before universal title
grammar runs. Transaction signal sets, connectivity-conflict ordinals, and residual overlap presentation also
used raw lexical spelling and now use first source occurrence.

After those production repairs, the exact pair has equal stage record counts and 11,170/11,170 cumulative proof
claims. The remaining diagnostic differences are conformance-normalization concerns: five source-derived
SemanticIR reference ids, 78 propagated id/set values, and one unordered top-level ISF named-drive block.
Generated ids use lowercase source projections, while ordinary prose and signal declarations preserve source
case; the comparator therefore needs field-aware projection. Set-valued responsibilities and declaration blocks
may be canonicalized only after symbol normalization. The existing many-to-one derived-id control remains the
fail-closed boundary: ambiguous structure must stay visible rather than receive positional pairing.

Initial machine evidence stays in Git history. Outcomes and thresholds never enter production.
`SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii.a` owns the measured production symbol-spelling coupling before complete
population replay.
