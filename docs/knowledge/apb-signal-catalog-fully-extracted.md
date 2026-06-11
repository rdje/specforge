---
id: apb-signal-catalog-fully-extracted
title: APB signal catalog is fully extracted (35/35); tables 0016/0017/0018 are duplicate views, not misses
answers:
  - "is the APB signal catalog extracted"
  - "are the APB signals PCLK PADDR PWDATA the parity-check PADDRCHK extracted"
  - "why do APB tables 0016 0017 0018 produce no signal records"
  - "what is the completeness gauge over-counting on APB"
  - "are APB tables 0016 0017 0018 a real catalog miss"
  - "where do the APB signal declarations come from (which table)"
  - "what does signal_table_covered_by_inventory do"
  - "is the APB PSTRB must be LOW constraint extracted"
  - "why is statement_0223 still a normative statement"
  - "what field holds the constrained signal name (signal_name vs subject_signal)"
  - "what does uncaptured_normative_statement_ids do"
  - "what are APB's remaining completeness candidate misses"
date: 2026-06-06
tags: [extraction, completeness, apb, region-accounting, wire-based-100]
evidence: docs/tasks/WIRE-BASED-100.md; crates/specforge/src/ir/completeness.rs (unexplained_intent_bearing_tables, signal_table_covered_by_inventory)
reverify: ./target/debug/specforge validate generated/evidence_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/evidence_ir.json 2>/dev/null | sed -n '/Region Accounting/,/convergence:/p'
---

The AMBA APB signal catalog is **already 100% extracted — 35/35 distinct signals, including all 14
parity-check `*CHK` signals** — from the well-aligned `table_0004`/`0005` ("APB signal descriptions",
`Signal|Source|Width|Description`) and `table_0014` ("Check signal descriptions"). Do **not** re-derive
"the signal catalog is unextracted" — that was a `WIRE-BASED-100.3` misdiagnosis, corrected by `.3a`.

`table_0016`/`0017`/`0018` are **redundant duplicate presentations** of those same signals (the AMBA
version matrix `Signal|Width|…|APB5|APB4|APB3|APB2`), badly column-mangled by docling — the body is
cyclically rotated so the `Signal` column lands LAST. They produce **zero** direct records, but every
signal they carry is already in the inventory, so they are **not** a catalog miss.

The completeness gauge previously over-counted them as `unexplained_intent_bearing_tables`. `.3a` fixed it:
`signal_table_covered_by_inventory` (in `ir/completeness.rs`) marks a `SignalDescription` table covered when
every signal in its densest-by-distinct-count signal-name column is already declared (content-based column
detection survives the rotation; distinct count beats a repeated `Property` column). Strict — one unknown
signal keeps the table flagged. APB `validate` candidate_misses **5 → 3** (0016/0017 covered; `0018` stayed
flagged until `PDF-VARIANT-DIGESTION.12a` taught the coverage to read the header-trapped data rows too —
see [[header-trapped-signal-table-recovery]]; **APB now reads 0/9 unexplained tables**, candidate_misses 13,
all prose residuals). No fabrication: the inventory (`collect_known_signal_names`) is read, never written.
See `[[conformal-tier-agreement-degenerate]]` for the related Pattern×Nlp disjoint-tier finding on APB.

**`.3b` sibling — captured normative statements aren't residuals.** APB `statement_0223` "the Requester
must drive all bits of PSTRB LOW" **IS** extracted as a constraint (`dyn_sigcon_0015` = `PSTRB
must_be_low`, via the *dynamic* path) — do not re-derive "PSTRB is a recall gap" (that was a field-name
mistake: the `SignalConstraintRecord` field is **`subject_signal`**, NOT `signal_name`). The statement
keeps class `NormativeStatement`, so the completeness gauge used to count it as a prose residual. `.3b`
fixed it: `completeness::uncaptured_normative_statement_ids` counts a normative statement as a residual
only when no typed record (signal_constraint/conditional_rule) cites its `statement_id`. After `.3a`+`.3b`,
APB candidate_misses were **2**, both genuine: `table_0018` (docling-garbled) + `statement_0370` (EDC
end-to-end — a non-wire system requirement, correctly left unstructured). `PDF-VARIANT-DIGESTION.12a`
(2026-06-11) then closed the `table_0018` line: its signals were trapped in `header_rows`, and the
trapped-row-aware coverage now recognizes them as already-declared duplicates
([[header-trapped-signal-table-recovery]]) — APB's unexplained-table count is **0**.
