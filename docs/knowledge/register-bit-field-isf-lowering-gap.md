---
id: register-bit-field-isf-lowering-gap
title: Register bit-field ISF lowering (DOC-INTENT-TAXONOMY.4a Gap A) — bit-field intent is fully captured + carried to IntentIR but dropped at ISF emit (isf_ir.rs:852) because ISF has no field-structured-storage construct; verified on FSMGen pin 030f8c273 → a verified FSMGen FR, not an emitter hack
answers:
  - "why do register bit-fields not appear in the emitted .isf (registers lower as opaque width-only storage vars)"
  - "where are register bit-fields dropped on the way to .isf (isf_ir.rs:852, IsfStorageVar { name, width, reset } — field metadata discarded)"
  - "is the register-bit-field ISF gap (Gap A) a SpecForge bug or a missing ISF abstraction (a missing ISF abstraction — fields reach IntentIR fully; ISF has no field-structured storage)"
  - "does the current FSMGen ISF support named bit-fields inside a storage var (NO — opaque (var NAME (width N)) only on pin 030f8c273; set-field/extract are runtime ops not a declaration)"
  - "what carries register bit-fields in SpecForge (RegisterFieldRecord in source.rs:414; IntentIr.register_records clone at intent.rs:193 — full metadata survives to IntentIR)"
  - "how many register bit-fields fail to lower to .isf (12,638 fields across 32 docs — the largest measurable intent-loss; DOC-INTENT-TAXONOMY.2)"
  - "why was an emitter-only fix for register bit-fields rejected (per-field vars fabricate/lose grouping; set-field/extract fabricate runtime behavior; comments are not intent — feedback_isf_no_hacks)"
  - "what is the FSMGen feature request for field-structured storage (declarative (var NAME (width N) (fields (field NAME (bits hi lo) (access ..) (reset ..) (enum ..)))); docs/FSMGEN_FEEDBACK.md 2026-06-22)"
  - "how is Gap A (register bit-fields) related to Gap B (message-field structures) — same missing ISF abstraction (named-field packed layout); Gap B also lacks an Evidence->Intent carrier (no message_field key in intent.rs)"
  - "did FSMGen accept the field-structured-storage FR (YES 2026-06-22 — accepted as a real ISF gap + valid future direction, accepted the proposed shape, but NOT shipped; gated on FSMGen's ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1; pin 5ce0335c5)"
  - "what can SpecForge do now about register bit-fields per FSMGen (keep field maps as IntentIR metadata/residuals, keep emitting opaque storage, fabricate nothing — .4a.i adapter honest residual is the FSMGen-endorsed near-term move; .4a.ii field-structured emit stays gated)"
date: 2026-06-22
tags: [doc-intent-taxonomy, isf-adapter, register, bit-field, storage, fsmgen-fr, isf-no-hacks, honest-residual, verify-fsmgen-before-fr, adr-0006, measured, gap-a]
evidence: crates/specforge/src/ir/source.rs:414 (RegisterFieldRecord full metadata); crates/specforge/src/ir/intent.rs:75/:193 (IntentIr.register_records full clone); crates/specforge/src/ir/isf_ir.rs:82/:834-852/:391 (IsfStorageVar build+render — fields discarded); subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md §8 (opaque (var) grammar); subs/fsmgen/docs/book/src/13k-isf-feature-support-matrix.md (set-field/extract are runtime ops); subs/fsmgen/docs/book/src/14-feature-backlog.md (field-structured storage absent); docs/FSMGEN_FEEDBACK.md (2026-06-22 FR); docs/research/register-bit-field-isf-lowering-design.md; docs/research/document-intent-isf-completeness.md (Gap A: 12,638/32 docs)
reverify: "Code path: grep -n 'IsfStorageVar' crates/specforge/src/ir/isf_ir.rs -> struct { name, width, reset } at ~:82, built ~:852 from r.fields used only for classify_register_reset; render ~:391 emits (var NAME (width N) [(reset V)]). grep -n message_field crates/specforge/src/ir/intent.rs -> zero matches (Gap B no carrier). FSMGen: grep -niE 'storage|var |field|struct|record' subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md subs/fsmgen/docs/book/src/14-feature-backlog.md -> (storage) is opaque width-only; no named-bit-field declaration; field-structured storage not in backlog (verified pin 030f8c273). Decision: FSMGen FR (docs/FSMGEN_FEEDBACK.md 2026-06-22), not an emitter hack (feedback_isf_no_hacks); empirical submodule verification done (feedback_verify_fsmgen_before_fr). Docs-only leaf -> golds/kg-bench orthogonal. Related: [[document-intent-isf-completeness]], [[register-reset-isf-emit]], [[isf-lowering-fidelity-gauge]]."
---

`DOC-INTENT-TAXONOMY.2` measured **Gap A**: register **bit-fields** reach the emitted `.isf` zero times
(12,638 fields across 32 docs) while registers themselves lower 1:1 to opaque
`(storage (var NAME (width N) [(reset V)]))`. `.4a` localized and resolved the WHY.

**The bit-field intent is fully captured and carried — it is dropped only at ISF emit.**
`RegisterFieldRecord` (`source.rs:414`) carries the complete field map (name, `bits_high`/`bits_low`,
width, `access_type`, `reset_value`, `description`, `enumerated_values`), and IntentIR carries the registers
unchanged (`IntentIr.register_records`, a direct clone at `intent.rs:193`). The loss is a single emit-time
discard at `isf_ir.rs:852`: `IsfStorageVar { name, width, reset }` reads `r.fields` only to compose a
register-wide reset and then renders the opaque `(var …)` (`isf_ir.rs:391`) — every field's name, bit range,
access, description and enum is dropped. So there is **no SpecForge carry gap** for Gap A.

**The current ISF has no field-structured-storage construct** (empirically verified on `subs/fsmgen` pin
`030f8c273`): `(storage …)` declares only opaque width-only scalars (`(var NAME (width N))` / `(variable …)`
/ `(bank …)`); the shipped `set-field`/`when-field`/`extract`/`assemble` are **runtime** read-modify-write
operations on an opaque register, not a static field-map declaration; and named scalar bit-fields are not on
the FSMGen backlog. Emitting per-field vars, or runtime `extract`, would fabricate structure/behaviour the
document never states — rejected by `[[feedback_isf_no_hacks]]`.

**Decision:** Gap A is a genuine missing ISF abstraction, so the doctrine-correct action is a **verified
FSMGen feature request** for declarative field-structured storage (`docs/FSMGEN_FEEDBACK.md`, `2026-06-22`) —
the empirical submodule verification satisfies `[[feedback_verify_fsmgen_before_fr]]`. Gap B (message-field
structures, 1,220 fields) shares the same missing abstraction and additionally lacks an `Evidence→Intent`
carrier (no `message_field` in `intent.rs`). Until ISF carries it, the field map stays honest IntentIR
metadata; SpecForge never fabricates a structure. See [[document-intent-isf-completeness]] (the scorecard),
[[register-reset-isf-emit]] (the reset value SpecForge already lowers onto the opaque var), and
[[isf-lowering-fidelity-gauge]].

**FSMGen ANSWER (`2026-06-22`, ingested via `FSMGEN-REFRESH-INTEGRATE-4`, pin `030f8c273`→`5ce0335c5`;
`subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md` § "Declarative Field-Structured Storage").** FSMGen **ACCEPTED** the
FR as a real ISF representational gap + valid future direction, and accepted the exact proposed shape (a storage var
with an optional declarative field partition — per field: name, bit range, optional access/reset/enum/provenance; first
version = checked metadata with fail-closed validation). It is **NOT shipped** — gated on FSMGen's own
`ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1` readiness/contract audit. FSMGen **confirmed the no-hack stance** (no
`set-field`/`extract`/fake-drive/comment substitute — "would fabricate behavior"). SpecForge's sanctioned near-term
posture (FSMGen's words): keep recovered register/CSR + packet/flit field maps as IntentIR **metadata/residuals**, keep
emitting **opaque** storage, fabricate nothing. ⇒ `[[document-intent-isf-completeness]]` Gap A `.4a.i` (adapter honest
residual) is the FSMGen-endorsed near-term move; `.4a.ii` (field-structured emit) stays GATED on FSMGen shipping the
construct. The bump was verified strict-clean on `5ce0335c5` (`*_passes_fsmgen_strict_validation` ×6 + `run_ci.sh` +
`kg-bench 156/156`).
</content>
