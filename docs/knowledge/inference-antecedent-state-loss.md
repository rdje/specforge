---
id: inference-antecedent-state-loss
title: The current APB canonical miss is an independently explicit inference-antecedent state loss
answers:
  - "why is APB PSEL asserted missing from the current reviewed population"
  - "where is the sole source to EvidenceIR canonical loss"
  - "why must PSEL not inherit VALID from the which means consequence"
  - "what does SPEC-TO-INTENT-ALIGNMENT.7 repair"
  - "what is the first task in canonical recovery"
  - "how can one compound sentence contain two independent signal facts"
  - "why does the APB recovery keep PSEL separate from PSELX"
  - "does suffix spelling authorize a PSEL to PSELX alias"
  - "why does the APB missing fact remain must_be_asserted instead of HIGH"
  - "how does production recover explicit inference antecedent state"
date: 2026-08-16
status: current
tags: [spec-to-intent-alignment, evidence-ir, signal-constraint, apb, canonical-recall]
evidence: crates/specforge/test_data/source_to_intent_vertical/current_result_snapshot.json; doctrine/spec_to_intent/canonical_recovery_contract.json; scripts/validate_canonical_recovery_contract.py; crates/specforge/src/ir/evidence.rs; docs/tasks/spec-to-intent-alignment/canonical-recovery.md
reverify: "python3 -B scripts/validate_canonical_recovery_contract.py --check && python3 -B scripts/validate_canonical_recovery_contract.py --self-test && cargo test --offline -p specforge-core --lib canonical_inference_antecedent_recovery && bash scripts/check_chain_currency.sh --check"
---

The current reviewed result has one canonical false negative and one unexplained SourceIR-to-EvidenceIR drop:
APB `PSEL|must_be_asserted|<missing>`. The complete source region is already captured. Its compound sentence says that
`PSEL` is asserted and, after `which means`, that `PADDR`, `PWRITE`, and `PWDATA` must be valid.

The consequence path in `extract_signal_constraints` intentionally calls `consequent_after_inference_marker`.
Its existing precision control proves that `PSEL` is not a subject of the later validity obligation. That behavior
is correct: emitting `PSEL|must_be_value|VALID` would borrow the suffix's value and fabricate the wrong fact.

The causal loss is the missing companion interpretation. No independent producer preserves the prefix's own
explicit copular state `PSEL is asserted` after the consequence is isolated. `SPEC-TO-INTENT-ALIGNMENT.7` is
therefore not permission to widen consequence subjects.

The exact identity premise is also bounded. The document's existing table-derived catalog declares `PSELX`,
but the reviewed span itself uses the appositive `select signal, PSEL` and the frozen oracle names `PSEL`. ADR
0037 forbids deriving an alias from the `X` suffix. `.7a` therefore treats the same-clause appositive as a local
declaration of opaque `PSEL`, retains `PSELX` as a distinct declaration, and adds a negative control proving that
bare `PSEL` with only `PSELX` declared emits nothing. This supersedes the historical indexed-family behavior;
see [[indexed-signal-family-canonicalization]].

The selected evidence also establishes no active-high or active-low polarity. The former gold key refined
`asserted` to `HIGH` without authority, contradicting the production polarity post-pass, which deliberately
keeps `MustBeAsserted` symbolic when polarity is unknown. `.7a` corrects the reviewed dataset, retrospective
snapshot, and current snapshot to the polarity-neutral key. The same one fact is still absent, so 39/0/1,
provenance, and conservation counts do not move; HIGH and LOW are now explicit forbidden outputs.

`.7a` froze that source-grounded sibling-producer grammar, polarity and ambiguity controls, affected-chain
scope, and population replay contract. `.7b` now implements it inside the normative constraint family: existing
pattern and consequence paths keep their order, the sibling appends only an independently explicit antecedent
state, the shared polarity pass refines it, and only sibling-introduced semantic duplicates are removed. The
production test executes all 20 frozen cases directly, and a complete synthetic build recovers the exact
polarity-neutral `PSEL` fact while preserving the three consequence facts and forbidding PSEL/PSELX value
borrowing.

The ruleset change rotated proof identity, so `.7b` rebuilt all 24 retained chains through the ISF adapter.
Every measurable stage is current with zero stale and the 192-file comparison has zero public JSON or emitted-
ISF delta. Those results prove the production boundary and fixed-input artifact currency; they do not rewrite
the revision-pinned reviewed score. The first independent replay proves this SourceIR-to-EvidenceIR loss is
closed and moves the remaining loss to SemanticIR's record-grounding boundary. See
[[inference-antecedent-local-grounding-stops-at-semantic-layer-d]]; `.7c.i` closes that exact carry boundary
without catalog widening, and `.7c.ii` publishes clean comparable 40/0/0 closure across all reviewed stages.
