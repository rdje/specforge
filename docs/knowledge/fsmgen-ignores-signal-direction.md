---
id: fsmgen-ignores-signal-direction
title: FSMGen --strict --check does NOT validate or use a signal's declared direction (input/output) — driving a declared input is not even a strict error; and a storage/interface (width W) accepts a symbolic name only if it resolves to a declared constant/param (else fails closed). Proven empirically (binary probe) + by contract (book).
answers:
  - "does FSMGen --strict --check use or validate signal direction (input vs output)"
  - "is it a strict error to drive a signal declared (input ...) in an .isf rule"
  - "does the emitted .isf signal direction affect FSMGen downstream correctness"
  - "does FSMGen accept a symbolic (width PARAM) or only a concrete integer width"
  - "why is the .isf direction default (output) FSMGen-neutral / not a faithful-lowering gap"
  - "can specforge emit a symbolic signal width like (width ADDR_WIDTH) to the .isf"
date: 2026-06-17
tags: [fsmgen, isf, isf-adapter, contract, signals, direction, width, kg-isf-completeness, measured, empirical]
evidence: "subs/fsmgen/docs/book/src/13a-actor-interface.md:369-402 (width resolves a declared constant/param to a positive integer, else fails closed; unknown symbolic names + arbitrary expressions fail closed); subs/fsmgen/docs/book/src/13g-rules.md (set port expr has no direction constraint; only pulse target requires an output); docs/research/isf-lowering-fidelity-measurement.md §6; docs/tasks/KG-ISF-COMPLETENESS.md (.2a/.2a.i)"
reverify: "Empirical, on the smallest wire .isf (SWD agent.isf, 13 signals all output/width-1): subs/fsmgen/bin/fsmgen --strict --check --json -> success:true. Flip a driven (output SWDIO (width 1)) to (input SWDIO ...) keeping its (drive) -> STILL success:true (direction NOT enforced). Change (output TCK (width 1)) to (width DATA_WIDTH) [undefined] -> success:false (symbolic-undefined fails closed); to (width 32) [concrete] -> success:true. AXI/AHB wire .isf carry a PRE-EXISTING rule-lowering diagnostic (constraint_33 (port expr) / HAUSER conflicting-rule-writes), unrelated to widths."
---

**Measured `2026-06-17` (`KG-ISF-COMPLETENESS.2a` trigger — empirical binary probe + FSMGen book contract, cross-checked).** Resolves whether the emitted `.isf`'s defaulted signal direction/width is a faithful-lowering gap.

**Direction is NOT used.** FSMGen `--strict --check` neither validates nor consumes a signal's declared
direction. Empirically, flipping a *driven* `(output SWDIO (width 1))` to `(input SWDIO …)` while keeping
its `(drive …)` still returns `success:true` — driving a declared input is not even a strict error. Book:
`(set port expr)` (13g-rules.md) places no direction constraint; only `(pulse target)` requires an actor
output; `(domain NAME)` on a port is "ownership metadata only, not permission" (13a:377-380). So the
emitter defaulting unknown direction to `output` (the deliberate `R6-ISF-ADAPTER.4` policy) is
**FSMGen-neutral** — it does not affect downstream correctness. Emitting a *faithful* direction would only
help a human reader (FSMGen would not catch a wrong one), and it is relationship-relative (every signal is
both an input and an output across actors) given the single flat module — so it is an owner-philosophy
choice, not a technical lowering gap.

**Width must resolve to a positive integer.** A `(width W)` accepts a concrete integer literal OR a
symbolic name that **resolves** to a declared actor constant / scalar parameter default (13a:369-402).
Empirically: `(width 32)` → `success:true`; an *undefined* `(width DATA_WIDTH)` → `success:false` (fails
closed); unknown symbolic names and arbitrary expressions fail closed. So to emit a symbolic width
faithfully the adapter must ALSO emit a `(constant …)`/param declaring it — a concrete grounded width is
the simpler faithful improvement, and it is what `KG-ISF-COMPLETENESS.2a.i` emits.

**Consequence for the north star.** The alarming "~98% of signals defaulted to `(output … (width 1))`" is
mostly *not* a faithful-lowering gap: direction is FSMGen-irrelevant, and only 69 signals corpus-wide carry
a concrete grounded width > 1 the emitter was defaulting to 1 (`.2a.i` recovers those). See
`[[isf-lowering-fidelity-gauge]]`, `[[project_kg_isf_completeness]]`, `[[feedback_isf_no_hacks]]`,
`[[feedback_verify_fsmgen_before_fr]]`.
