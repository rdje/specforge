---
id: arithmetic-width-drops-the-declaration
title: A signal declaration whose width the SemanticIR parser cannot finish is discarded whole, and every obligation about that signal is demoted with it
answers:
  - "why is WSTRB missing from AXI's SemanticIR interface catalog while WSTRBCHK is in it"
  - "why did a signal declared in EvidenceIR never reach the semantic interface records"
  - "why is an AXI obligation about WSTRB demoted to a residual"
  - "what happens to a declaration whose width is an arithmetic expression"
  - "how many declared signals never reach the SemanticIR interface catalog"
  - "can a width be written as an arithmetic expression with spaces in it"
  - "why does DATA_WIDTH/8 work as a width but DATA_WIDTH / 8 not"
  - "does parse_explicit_signal_declaration keep the direction when it cannot read the width"
  - "why does removing a fabricated EvidenceIR constraint change nothing in SemanticIR"
  - "where is the declared-signal set that the semantic grounding filter uses built"
date: 2026-09-13
status: current
tags: [semantic-ir, signal-catalog, declaration, grounding-filter, axi, signal-declaration-row-drop, extraction-quality-gauge]
evidence: crates/specforge/src/ir/semantic.rs (parse_explicit_signal_declaration, the `if index != tokens.len() { return None; }` arm; parse_width_expression; width_expression_length; build_interfaces; declared_signal_names; the signal_constraints grounding partition); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.4, .4a, .4b); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3k.7)
reverify: "python3 scripts/measure_declared_signals_missing_from_semantic.py — expect 75 missing across 10 documents after SIGNAL-DECLARATION-ROW-DROP.4a; the AXI-L line must NOT show WSTRB"
---

`parse_explicit_signal_declaration` reads `Signal X is <direction> width <W>.` and ends with

```rust
if index != tokens.len() { return None; }
```

so a declaration it cannot consume **to the last token** yields `None` — not a partial record. The
direction it has already parsed goes with it. There is no residual, no counter and no validation
entry, which is the same silence `SIGNAL-DECLARATION-ROW-DROP.0` measured for the EvidenceIR-stage
reader, at a different function one stage later.

`build_interfaces` is the only producer of `InterfaceRecord::signal_records`; `declared_signal_names`
is built from those records; and the SemanticIR grounding partition keeps a `signal_constraint` only
when `declared_signal_names` contains its subject. **So an unreadable width does not merely lose a
width — it deletes the signal's identity, and every obligation the document states about it is
demoted into `residual_decisions`.**

## The measured population, and what `.4a` closed

Over all 78 documents carrying both an EvidenceIR and a SemanticIR (`2026-09-13`): **83 signals were
declared in EvidenceIR and absent from the SemanticIR interface catalog, across 10 documents** —
MMU-700 47, AXI-H 9, AXI-L 9, GICv3 7, Avalon 5, CHI 2, and one each in ATB, CXS, LTI and eMMC. Two
strata, needing different answers:

* **17 carry a legitimate arithmetic width, and the reader could not span whitespace.**
  `parse_width_token` already accepted the parametric width `DATA_WIDTH/8`; `parse_optional_width_hint`
  consumed exactly ONE whitespace token, so `width DATA_WIDTH / 8` left `/` and `8` over. The tell was
  in the catalog: AXI carries **`WSTRBCHK` but not `WSTRB`** — the parity companion has a plain width
  and the wire it checks does not. **`SIGNAL-DECLARATION-ROW-DROP.4a` reads the width as an
  expression** (numbers, parameters, `+ - * /`, balanced parentheses, the call form `ceil(…)`) and
  tolerates the prose a specification writes after it, under two conditions: the expression must end
  at a whitespace-token boundary, and trailing material is allowed only after a STRUCTURED expression.
  **14 of the 17 now read**; the census falls to **75** because only AXI-L's artifact is rebuildable
  (AXI-H, CHI, ATB and LTI are legacy/proofless and frozen until re-ingest).
* **66 carry a width the source row never stated as one** — MMU-700's
  `Signal LAADDR is width 3'b000 , lavalid`, an upstream misread. Refusing these is probably right;
  refusing them silently is the defect either way. With the three malformed arithmetic expressions
  `.4a` also refuses (`ceil((USER_DATA_WIDTH USER_RESP_WIDTH)/8)` has no operator,
  `ceil((LTI_SSID_WIDTH +` is truncated, `log 2 (DATA_WIDTH) -` is mangled) that residue is **69**,
  owned by `SIGNAL-DECLARATION-ROW-DROP.4b`.

## Why this is easy to mistake for something else

`EXTRACTION-QUALITY-GAUGE.3k.7` removed a fabricated `WSTRB must_be_value VALID` from AXI-L's
EvidenceIR and the SemanticIR constraint count did not move (53 → 53). That was not the fix failing:
the fabricated record was already being dropped here, together with the document's REAL
*"An attached Subordinate must have its WSTRB input tied HIGH"*. **A constraint-stage change measured
only at SemanticIR reads as a no-op for any signal in this set.** `.4a` closed the loop: AXI-L's
catalog went 288 → 296, that obligation was promoted, and its lone `residual_decisions` entry went to
zero.

Links: [[declaration-reader-drops-uninterpretable-rows]], [[semantic-grounding-filter-is-catalog-independent]],
[[axi-constraint-subject-must-be-declared]], [[declared-spelling-is-the-document-spelling]].
