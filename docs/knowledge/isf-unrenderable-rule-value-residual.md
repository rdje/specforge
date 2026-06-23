---
id: isf-unrenderable-rule-value-residual
title: The `.isf` emitter drops a rule whose drive VALUE is not a renderable ISF value expression (free prose, not a `(port expr)`) and records an `isf_rule_value_<name>` residual — closing the last AMBA AXI+ACE `(port expr)` strict-FAIL so the renderable corpus is 70/70 FSMGen-strict-clean (KG-ISF-COMPLETENESS.2a.vi)
answers:
  - "why does an .isf rule get dropped when its drive value is prose / not a (port expr)"
  - "what is KG-ISF-COMPLETENESS.2a.vi (ISF rule-drive-value validity gate)"
  - "why did the AMBA AXI+ACE ihi0022_h_c manager.isf fail fsmgen strict with 'rule constraint_48 assignment actions require (port expr)'"
  - "what does drop_unrenderable_rule_values / unrenderable_rule_value_residual_packet do in ir/isf_ir.rs"
  - "how does the emitter decide a rule drive value is renderable (is_safe_isf_scalar_value — non-empty, whitespace-free; a prose value like 'the value that was presented on the ARLOOP signal' fails)"
  - "why not recover (port ARLOOP) from the loopback prose (the temporal 'was presented' loopback is not the current (port ARLOOP); recovering one would fabricate the timing — honest residual over fabrication)"
  - "is the whole renderable corpus FSMGen --strict clean now (yes — 70/70 current-emit .isf after .2a.vi; the ISF-emit strict-FAIL frontier is closed: Levers A/B/C/F + .2a.vi)"
  - "are other docs affected by the rule-drive-value gate (no — only ihi0022_h_c carries a prose-valued rule corpus-wide; every other emit is byte-identical)"
date: 2026-06-23
tags: [kg-isf-completeness, isf, emitter, rule-value, port-expr, fsmgen-strict, adr-0006, residual, fix, wire-gold]
evidence: crates/specforge/src/ir/isf_ir.rs (drop_unrenderable_rule_values — drop a rule whose any drive value fails is_safe_isf_scalar_value; unrenderable_rule_value_residual_packet → isf_rule_value_<name>; wired in from_intent_ir before the width/dedup passes); docs/tasks/KG-ISF-COMPLETENESS.md (.2a.vi node + acceptance checklist)
reverify: "RAM-safe, no VLM/Docling. cargo build -p specforge. AXI+ACE: specforge adapt generated/intent_ir/ihi0022_h_c_2021_01_amba_axi_and_ace_protocol_specification/intent_ir.json --target isf; perl subs/fsmgen/bin/fsmgen --strict --check --json generated/adapters/isf/ihi0022_h_c_*/manager.isf -> success=true 0 diagnostics; adapter.json residual_decisions carry isf_rule_value_constraint_48/_49/tinv_sc_llm_sigcon_0061/_0062 (the RLOOP/BLOOP loopback prose). Scope: grep -nE '^\\s*\\([A-Z][A-Za-z0-9_]*\\s+[^)]*\\s[^)]*\\)$' over all generated/adapters/isf/**/*.isf (excluding head keywords) finds prose-valued drives ONLY in ihi0022_h_c (4 lines). Full sweep: clean generated/adapters/isf, re-emit all, fsmgen --strict each -> 70/70 PASS. run_ci.sh GREEN (lib 1709); kg-bench 156/156."
---

**Built `2026-06-23` (`KG-ISF-COMPLETENESS.2a.vi`, CODE).** The LAST open ISF-emit strict-FAIL (the spun-out
`ISF-VALUE-WIDTH-EMIT` `(port expr)` Non-Goal, surfaced as the lone remaining FAIL after Lever C `.2a.v`):
the combined AMBA AXI+ACE `ihi0022_h_c` `manager.isf` failed FSMGen `--strict --check` with
`Error: rule 'constraint_48' assignment actions require '(port expr)'`.

## Root cause (WHY + WHERE)
The rule's drive VALUE is free PROSE — `(RLOOP the value that was presented on the ARLOOP signal)` /
`(BLOOP … AWLOOP …)`, a loopback-tag obligation the extractor captured as a sentence rather than a literal.
FSMGen requires a rule assignment action's RHS to be a renderable value expression (`(port expr)` — a
literal, port reference, or expression), so a multi-word prose value breaks the whole `.isf`. The emitter
rendered rule drive values VERBATIM (`ir/isf_ir.rs`) with NO validity gate on the value (the existing gates
cover enum members `.2a.iv` and value width `ISF-VALUE-WIDTH-EMIT`, not the rule's own scalar).

## The fix (drop + honest residual)
New pass `drop_unrenderable_rule_values` (`isf_ir.rs`, before the width/dedup passes) drops a rule whose any
drive value fails `is_safe_isf_scalar_value` (the existing non-empty / whitespace-free scalar test) and
records an `isf_rule_value_<name>` residual. The prose value is unrecoverable — "the value that was
*presented* on ARLOOP" is a temporal loopback, NOT the current `(port ARLOOP)`, so recovering a `(port expr)`
would fabricate the timing — honest residual over fabrication (`[[feedback_isf_no_hacks]]`). ADR-0006: a
structural value-shape test, no chip/vendor/protocol name list. Every legitimate rule drive (a scalar
literal `0`/`1`/`0b01`, a width-cast `7'd125`, an enum symbol `VALID`) passes the gate.

## Measured impact — the renderable corpus is now 70/70 FSMGen-strict-clean
A read-only scan of all 70 current-emit `.isf` found EXACTLY 4 prose-valued rule drives, ALL in
`ihi0022_h_c` (`constraint_48`/`_49` + the `tinv_sc_llm_sigcon_0061`/`_0062` duplicates); ZERO other docs
carry a whitespace-bearing rule drive value, so the gate is corpus-safe by construction (every other emit
byte-identical). After the fix, `ihi0022_h_c` goes FAIL→PASS and a full FSMGen `--strict` sweep over the 70
current emits is **70/70 PASS** — the ISF-emit strict-FAIL frontier is fully CLOSED (Levers A `[[isf-value-width-operand-contract]]`, B module-name sanitization, C `[[isf-unconditional-rule-overlap-conflict]]`,
F `[[isf-enum-value-literal-emit-gate]]`, plus this value-validity gate). `kg-bench` 156/156; `run_ci.sh`
GREEN (lib 1709, +1 test); WIRE-BASED-100 orthogonal by construction (emitter-only — `eval-extraction` reads
the IR, never the `.isf`). `[[project_kg_isf_completeness]]` (bar #6) / `[[feedback_verify_fsmgen_before_fr]]`.

The upstream extraction-precision question — why a constraint's value was captured as prose — is a separate,
non-emitter lever (an `extract-constraints-llm` value-recovery concern), recorded as an honest residual not a
`.2a.vi` gap.
