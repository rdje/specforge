---
id: fsmgen-temporal-isf-form
title: SpecForge emits bounded-eventually as (assert (monitor (within s N))) into .isf (fsmgen pin 43b29f5c)
answers:
  - "how does SpecForge emit temporal rules or a bounded-eventually into .isf"
  - "what ISF form does SpecForge use for a bounded-eventually contract"
  - "was the (contract ... eventually ...) ISF clause removed"
  - "what fsmgen pin does SpecForge target for temporal properties"
  - "why doesn't SpecForge emit (contract eventually) anymore"
date: 2026-06-04
tags: [fsmgen, isf, temporal, adapter]
evidence: crates/specforge/src/ir/isf_ir.rs; subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md
reverify: grep -n "assert (monitor (within" crates/specforge/src/ir/isf_ir.rs
---

SpecForge lowers a **windowed bounded-eventually** obligation to the FSMGen verification-family
monitor property **`(assert (monitor (within <signal> <N>)))`** (`isf_ir.rs` render). The former
`(contract <name> (eventually <signal> (within N)))` clause was **removed upstream** at fsmgen
pin **`43b29f5c`** (verification-family generalization; FSMGen decisions `0008`/`0009`; their
`SPECFORGE_FEEDBACK_RESPONSE.md` 2026-06-04 section) — emitting it now draws *"unsupported
'(contract ...)' clause"*. `subs/fsmgen` is pinned at `43b29f5c` (`FSMGEN-ASSERT-MIGRATE`).

`(stage …)` (ready/valid barrier) still validates unchanged. Non-windowed value/guard→drive →
`(rule …)`; un-representable obligations (HandshakeComplete, `stable`, `min > 1` windows) →
residual decisions — `stable` and `min > 1` await FSMGen primitives (being requested). The
emission is empirically strict-valid (the fsmgen-binary strict-check tests run the new binary
via `run_fsmgen_strict_check`). See `docs/tasks/FSMGEN-ASSERT-MIGRATE.md`,
`[[fsmgen-feedback-channel]]`, `[[isf-temporal-lowering-no-silent-drop]]`.
