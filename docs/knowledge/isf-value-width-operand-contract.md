---
id: isf-value-width-operand-contract
title: FSMGen strict rejects an ISF value literal whose notation width ≠ the target signal width (OperandContract, no implicit truncation); the faithful fix is emit the grounded width + width-align the literal (W'…) or residualize — never truncate (ISF-VALUE-WIDTH-EMIT.1 measured GO 2026-06-21; .2 FIXED + verified 2026-06-21, tree CLOSED)
answers:
  - "why does the DTI / trace-bus .isf fail FSMGen --strict --check (OperandContract: a value literal wider than the declared signal width)"
  - "how does FSMGen decide a value literal's width (by notation digit count — 0x7D=8 bits, 0b00=2 bits — NOT by value; it requires an exact width-cast W'… match, no implicit truncation/extension; a bare decimal is unsized and fits any width)"
  - "where does the ISF emitter lower a rule value literal (ir/isf_ir.rs:1493-1495 render_isf_control_expression → ControlExpressionRecord::Literal{literal}=>literal.clone(); copied verbatim, no width reconciliation at the emit site isf_ir.rs:418-432)"
  - "why is ATID emitted (width 1) when the IntentIR grounds width 7 (the emitter's first-seen signal dedup isf_ir.rs:696-700 takes the first signal_records entry (w=None→1) and skips the later w=7 record; the .2a.i recovery only falls back to actor_ports, and ATID has none)"
  - "what is the faithful fix for an over-width ISF value literal (1: recover the signal's grounded width across ALL interface signal_records + actor_ports; 2: re-render the literal as a width-cast W'<radix><digits> when value<2^W, else residualize — never truncate; ADR-0006 numeric only)"
  - "is the DTI ATST 0B01 constraint a real obligation (no — mis-attribution: the source text 'ATTR_OVR.SHCFG must be 0b01' binds SHCFG's value to ATST, which is a value of the FLOW field; an upstream extraction bug spun out of ISF-VALUE-WIDTH-EMIT)"
  - "do the wire-gold .isf carry over-width value literals (only AXI ihi0022_l has one — AWCMO; and AXI already fails strict on the orthogonal (port expr) error; APB/AHB/SWD have none; WIRE-BASED-100 measures extraction F1 not .isf bytes so it is orthogonal)"
  - "which docs have the ISF value-width defect (4 docs / 13 clauses: DTI ATST ×3 [mis-attribution], AXI+ACE ARTAGOP/BTAGMATCH ×6 [width-2 under-emitted, masked by (port expr)], AXI-gold AWCMO ×1 [parametric AWCMO_WIDTH], trace-bus ATID ×3 [width-7 under-emitted — the clean lever])"
date: 2026-06-21
tags: [isf-value-width-emit, isf, operand-contract, width, value-literal, intent-ir, adr-0006, measured, fsmgen-contract, north-star, ir-isf-ir, honest-residual]
evidence: docs/research/isf-value-width-alignment-measurement.md (the full .1 measurement + FSMGen probe table + GO decision); crates/specforge/src/ir/isf_ir.rs:1493-1495 (render_isf_control_expression literal clone), :696-700 (first-seen signal dedup that drops the grounded width), :715-736 (.2a.i width recovery, actor_ports-only), :418-432 (rule-body emit, no width check); generated/adapters/isf/ihi0088_g_*/channel.isf (DTI ATST) + ihi0032_c_*/channel.isf (trace ATID); generated/intent_ir/ihi0088_g_*/intent_ir.json (dyn_sigcon_0014 ATST=0B01, source_text names SHCFG); subs/fsmgen OperandContractValidationSupport.pm (the pre-generation operand contract)
reverify: "RAM-SAFE, no cargo build (fsmgen is subs/fsmgen/bin/fsmgen, Perl). Confirm the failure: perl subs/fsmgen/bin/fsmgen --strict --check --json generated/adapters/isf/ihi0088_g_2024_06_amba_dti_protocol_specification/channel.isf 2>&1 | grep -v '^\\[WARNING\\]' | python3 -c 'import sys,json; d=json.load(sys.stdin); print([x.get(\"message\") for x in (d.get(\"diagnostics\") or [])][:1])' -> 'assignment to ATST uses RHS 2'b1 with incompatible width 2 for LHS width 1; the current contract blocks implicit truncation'. FSMGen literal-width semantics (minimal probe): mk(){ printf '(actor p\\n (clock C)\\n (reset (R sync active_high))\\n (watchdog 1)\\n (interface\\n  (output %s (width %s))\\n )\\n (rule r1\\n  (%s %s)\\n )\\n)\\n' \"$1\" \"$2\" \"$1\" \"$3\" > /tmp/p.isf; perl subs/fsmgen/bin/fsmgen --strict --check --json /tmp/p.isf 2>&1 | grep -v '^\\[WARNING\\]' | python3 -c 'import sys,json;print(json.load(sys.stdin).get(\"success\"))'; }; mk ATID 7 0x7D -> False (8 notation-bits != 7); mk ATID 7 \"7'd125\" -> True (width-cast aligned); mk ARTAGOP 2 0b00 -> True; mk ARTAGOP 1 0b00 -> False. Root cause: python3 - <<'PY' reads generated/intent_ir/ihi0032_c_2021_12_amba_trace_bus_protocol_specification/intent_ir.json -> ATID interface signal_records = [{w:None},{w:None},{w:7}] (first-seen wins -> width 1; the w:7 record is skipped); ATID actor_ports = [] PY"
---

**Established `2026-06-21`** (`ISF-VALUE-WIDTH-EMIT.1`, read-only over the 86 persisted
`generated/adapters/isf/*/*.isf` + FSMGen probes; report
`docs/research/isf-value-width-alignment-measurement.md`).

The ISF emitter copies a `(rule … (SIGNAL value))` value literal **verbatim** from the IntentIR
(`render_isf_control_expression`, `isf_ir.rs:1493-1495`) and never reconciles its bit-width against the
target signal's declared `(width N)`. FSMGen's strict `--check` (`OperandContractValidationSupport.pm`)
decides a literal's width by its **notation digit count** (`0x7D` = 8 bits, `0b00` = 2 bits — *not* by
value) and requires the RHS to be an *"explicit width-aligned source expression"* whose width **equals**
the LHS signal width, blocking implicit truncation. So `(ATST 0B01)` on a `(width 1)` signal renders
`2'b1` (width 2) onto width 1 and fails before HDL generation.

Decisively, the affected signals are emitted at `(width 1)` **even when the IntentIR grounds a wider
width**: the emitter's first-seen dedup (`isf_ir.rs:696-700`) takes the first `signal_records` entry
(often `width=None`→1) and skips a later record carrying the concrete width, and the `.2a.i` recovery
only consults `actor_ports` (ATID has none). So ATID emits width 1 despite a grounded width 7; ARTAGOP
despite width 2. The over-width literal is the *symptom* — **truncating the value would be dishonest**
(ATID `0x7D` = 125 is a genuine 7-bit trace-ID value).

**Faithful fix (GO, `.2`):** (1) recover the signal's grounded width across **all** interface
`signal_records` + `actor_ports`; (2) re-render the value literal as a width-cast `W'<radix><digits>` when
`value < 2^W`, else **residualize** the clause (the `[[register-reset-isf-emit]]` / ISF-RULE-CONFLICT
honest-residual pattern) — never truncate. Bare decimals are unsized and untouched. ADR-0006: numeric
parsing + width arithmetic only, no name list.

**Prevalence:** 4 docs / 13 clauses — DTI ATST ×3 (**upstream mis-attribution**: the source text's
`0b01` belongs to `ATTR_OVR.SHCFG`, ATST is a value of FLOW — spun out), AXI+ACE ARTAGOP/BTAGMATCH ×6
(width-2 under-emitted, but masked by the orthogonal `(port expr)` grammar failure), AXI-gold AWCMO ×1
(parametric `AWCMO_WIDTH` → residual), trace-bus ATID ×3 (width-7 under-emitted — the clean lever).
**Wire-gold blast radius:** only AXI `ihi0022_l` carries one (AWCMO) and it already fails strict on
`(port expr)`; WIRE-BASED-100 measures extraction F1, not `.isf` bytes, so the fix is orthogonal.

Governing north star: `[[project_kg_isf_completeness]]` (bar #6). Sibling emitter-fidelity tree:
`[[register-reset-isf-emit]]`. Orthogonal silent-drop gauge: `[[isf-lowering-fidelity-gauge]]`. FSMGen
direction/width neutrality: `[[fsmgen-ignores-signal-direction]]`.

**Resolved `2026-06-21` (`ISF-VALUE-WIDTH-EMIT.2`, CODE — tree CLOSED).** Both root causes are now fixed in
`ir/isf_ir.rs` exactly as the `.1` design specified, ADR-0006 numeric-only: (1) an `interface_widths` aggregate
(single unambiguous `WidthHint::Numeric > 1` across **all** `interfaces[].signal_records`, conflict→width-1)
chained ahead of the `.2a.i` `port_widths` in the width fallback, so a width in a non-first record is recovered;
(2) an `align_rule_drive_widths` post-pass (`parse_sized_literal` / `align_value_to_width` / `ValueAlign` /
`value_width_residual_packet`, before `dedup_conflicting_rules`) that re-renders an over-wide-but-fitting based
literal as `W'd<v>` or DROPS the rule with an `isf_value_width_*` residual. Verified on the real
`subs/fsmgen/bin/fsmgen --strict --check --json`: DTI `(ATST 1'd1)` and trace-bus `(output ATID (width 7))` +
`(ATID 7'd125)` now report `has_diagnostics: false` (the OperandContract value-width error is CLEARED); the 4
wire golds carry 0 NEW diagnostics (AHB/AXI keep their pre-existing orthogonal
`isf_conflicting_rule_writes`/`(port expr)` issues); `scripts/run_ci.sh` GREEN (suite 1682 passed / 0 failed);
`kg-bench` 156/156. To re-confirm the FIX (not the original failure), run the same `reverify` command above — it
now returns `[None]` (no diagnostics) on a freshly regenerated `.isf`.
