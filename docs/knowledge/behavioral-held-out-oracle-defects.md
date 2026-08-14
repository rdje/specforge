---
id: behavioral-held-out-oracle-defects
title: The initial behavioral holdout exposed two conformance-oracle defects, not production signoff evidence
answers:
  - "why did the initial prospective behavioral held-out run fail"
  - "what did real filenames expose in adversarial identity comparison"
  - "why are held-out alpha failures not yet production name-coupling evidence"
  - "which held-out document has an eligible opaque alpha catalog"
  - "what were the initial 17-document held-out relation outcomes"
date: 2026-08-14
status: current
tags: [genericity, held-out, metamorphic-testing, conformance, alpha-renaming]
evidence: docs/research/behavioral-held-out-initial-run.md; doctrine/production_genericity/behavioral_holdout_evidence.json; docs/tasks/spec-to-intent-alignment/behavioral-qualification.md; crates/specforge-conformance/src/behavioral_genericity.rs; scripts/check_behavioral_genericity_contract.py
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

The active `.f.iii` leaf owns narrow oracle repair and requalification. Initial machine evidence stays in Git
history, and neither held-out outcomes nor thresholds enter production.
