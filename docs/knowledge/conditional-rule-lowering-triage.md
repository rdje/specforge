---
id: conditional-rule-lowering-triage
title: Conditional-rule lowering triage (DOC-INTENT-TAXONOMY.4e) — the conditional_rules ISF-lowering shortfall is HONEST RESIDUAL, not a gap (73% prose/undeclared/placeholder; the 27% "lever" bucket is 161/164 bare deontic modals, only 3/603 carry a concrete obligation); no ISF lever, no FR — only an upstream extraction-quality opportunity
answers:
  - "is the conditional_rules ISF-lowering shortfall a real gap or honest residual (HONEST RESIDUAL — 73% prose/undeclared/placeholder; the rest are bare deontic modals with no concrete obligation; no buildable ISF lever, no FR)"
  - "why do many conditional_rules not lower to an ISF (rule) (they name no signal obligation, an undeclared signal, a placeholder action, or only a bare modal shall/must with no concrete value/level — lowering would fabricate the obligation)"
  - "how many conditional_rules are concretely lowerable (only ~3 of 603 across 9 representative docs carry a concrete value/level cue; 161/164 declared-consequent candidates are bare modals shall/must/shall not)"
  - "should SpecForge build a conditional-rule lowering lever or file an FSMGen FR (NO — the adapter already lowers the 516 cleanly-grounded conditional obligations corpus-wide; the shortfall is honest residual; the only upside is upstream extraction quality, not an ISF construct)"
  - "what is the only improvement path for conditional rules (upstream EXTRACTION — extract-constraints-llm / EXTRACTION-QUALITY-GAUGE recovering the concrete obligation from the conditional's source_text for the deontic-modal bucket; then it lowers via the existing (rule) path with no new ISF construct — lower-leverage than register/structure/topology)"
  - "what is DOC-INTENT-TAXONOMY .2 Result 3 verdict (closed by .4e: the rule-lowering shortfall is dominated by conditional_rules that are honest residual; signal_constraints + temporal_rules lower well; not an ISF-completeness gap)"
  - "is the DOC-INTENT-TAXONOMY .2 per-category scorecard measurement complete (YES after .4e — Gap A lowered .4a.ii, Gap B carrier .4b gated, cat-3 .4c->.4c.i, cat-4 .4d->.4d.i, conditional rules .4e honest residual; remaining work is CODE not measurement)"
date: 2026-06-23
tags: [doc-intent-taxonomy, conditional-rules, isf-adapter, honest-residual, extraction-quality, isf-no-hacks, scoring-rigor, adr-0006, measured, decision-packet]
evidence: generated/intent_ir/*/intent_ir.json conditional_rules (9 representative docs, 603 rules — A no-consequent 392 / B undeclared 14 / C placeholder 33 / D declared+action 164 of which 161 bare modal, 3 concrete); declared-signal inventory = interfaces[].signals + interfaces[].signal_records[].signal_name + actor_ports[].signal_name; docs/research/conditional-rule-lowering-triage.md; docs/research/document-intent-isf-completeness.md (Result 3)
reverify: "python3 over generated/intent_ir/<key>/intent_ir.json conditional_rules: classify by consequent_signal present? declared? action placeholder/modal/concrete. Declared set = interfaces[].signals + interfaces[].signal_records[].signal_name + actor_ports[].signal_name. Expect ~65% no-consequent + ~8% undeclared/placeholder + ~27% declared-consequent of which ~98% bare modal (shall/must), only ~3/603 concrete. Conclusion: honest residual, no ISF lever, no FR; only upstream extraction-quality upside (extract-constraints-llm). Docs-only leaf -> golds/kg-bench orthogonal. Related: [[behavior-temporal-lowering-broader-corpus]], [[document-intent-isf-completeness]], [[cat4-isa-csr-lowering-decision]], [[cat3-topology-isf-lowering-decision]]."
---

`DOC-INTENT-TAXONOMY.4e` is the per-item triage of `.2` Result 3 — the `conditional_rules` ISF-lowering
shortfall — separating honest residual from a real buildable lever before any fraction is called a gap
([[feedback_scoring_rigor]]). Read-only, docs-only.

**Measured (603 conditional_rules across 9 representative docs, all 4 buildable categories).** Classified each
against the document's declared-signal inventory + consequent quality: **A** no consequent signal (prose
condition) = 392 (65%); **B** consequent names an undeclared signal = 14 (2%); **C** declared consequent but
placeholder action = 33 (6%); **D** declared consequent + non-placeholder action = 164 (27%). The D bucket is
**not** a clean lever: 161 of 164 are bare deontic modals (`shall` 45 / `must` 35 / `shall not` 14 /
`shall be cleared` 10 / `must not` 7 / …) with no concrete obligation, and **only 3 of 603 carry a concrete
value/level cue**. Lowering a bare `shall`/`must` against a declared signal would require synthesizing the
value the document states only in prose — fabrication, forbidden by [[feedback_isf_no_hacks]].

**Decision.** The conditional-rule shortfall is **honest residual, not an ISF-completeness gap** — no buildable
ISF lever and no FSMGen FR. The adapter already lowers the cleanly-grounded conditional obligations (516
corpus-wide, [[behavior-temporal-lowering-broader-corpus]]); `signal_constraints` + `temporal_rules` lower
well. The only upside is **upstream extraction quality** (the grounded constraint extractor recovering the
concrete obligation from the deontic-modal conditionals' `source_text`), owned by the extraction-quality
program, distinct from `.4` ISF lowering and lower-leverage than register/structure/topology — recorded as a
cross-reference, **not** minted as a `.4` gap (a 0.5%-concrete residual is not a gap). With `.4e` closed, the
`.2` per-category scorecard's measurement phase is complete; the remaining `.4` work is CODE (`.4d.i`,
`.4c.i`, `.4b`), not measurement. See [[document-intent-isf-completeness]].
