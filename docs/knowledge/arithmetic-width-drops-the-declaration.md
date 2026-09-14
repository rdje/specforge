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
  - "does SemanticIR record a declaration it refused for an unreadable width"
  - "what is the semantic_unreadable_declaration_width residual packet"
  - "how many declarations does the SemanticIR reader refuse corpus-wide"
  - "why are the 75 missing declared signals almost all legacy"
  - "is a sentence opening with the word signal and no attribute a lost declaration"
date: 2026-09-14
status: current
tags: [semantic-ir, signal-catalog, declaration, grounding-filter, axi, signal-declaration-row-drop, extraction-quality-gauge]
evidence: crates/specforge/src/ir/semantic.rs (parse_explicit_signal_declaration, the `if index != tokens.len() { return None; }` arm; parse_width_expression; width_expression_length; build_interfaces; declared_signal_names; the signal_constraints grounding partition); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.4, .4a, .4b); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3k.7)
reverify: "python3 scripts/measure_declared_signals_missing_from_semantic.py — expect 75 missing across 10 documents after SIGNAL-DECLARATION-ROW-DROP.4a; the AXI-L line must NOT show WSTRB. Stratum: 9 of those 10 carry `\"schema_version\": 1` SemanticIR and `specforge semantic <doc>/evidence_ir.json --dry-run` refuses each with `EvidenceIR schema version 2 is legacy/proofless`, so 74 of the 75 are latent, not current. Refusal visibility: over the 27 chain-current documents exactly one carries `semantic_unreadable_declaration_width` (AXI-L, naming RUSERCHK) — `jq -r \".residual_decisions[].packet_id\" generated/semantic_ir/*/semantic_ir.json | sort | uniq -c`."
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

## `.4b`: the refusal is recorded, and the population is 74/75 LEGACY

**The census mixes strata, and no node had said so.** Nine of its ten documents carry
`"schema_version": 1` SemanticIR — an older binary's output that the current chain cannot reproduce,
because `specforge semantic <doc>/evidence_ir.json --dry-run` refuses each of the nine with
`EvidenceIR schema version 2 is legacy/proofless and inspection-only`. **The chain-current residue is
1**: AXI-L's `RUSERCHK`. MMU-700's 47 is real and *latent* — it becomes observable when MMU-700 is
re-ingested, not before. Any recall number read off this census is a statement about persisted
artifacts, not about what today's reader does.

**`read_explicit_signal_declaration` now states which refusal arm fired**, and
`unreadable_declaration_residual_packet` carries one of the three as a
`semantic_unreadable_declaration_width` residual: the declaration stated an attribute and was then
refused because its width text could not be consumed to the end of the sentence, and the identity it
names reaches no interface record anywhere in the document.

**Only that one arm is reported, and the boundary was measured.** Over the 27 chain-current documents
the reader refuses 11 sentences — 8 `no_direction_and_no_width`, 2 `name_not_an_identifier`, 1
`width_text_unread`. Adjudicated against their source statements, all 10 in the first two arms are
English prose that opens with the word "signal": *"Signal names MUST adhere to the rules of the
native tool"*, *"Signal arrays are identified by a name followed by a set of parenthesis"*. Reporting
all three arms publishes `names`, `arrays`, `direction`, `is` and `at` as lost wires — **1 real
identity in 8**; reporting the one arm is **1 in 1**. The split is structural, not tuned: EvidenceIR
synthesizes a declaration only from a row that yielded at least one attribute (`.0`'s `_ => continue`
arm drops the rest before any statement exists), so a `Signal …` sentence with neither a direction nor
a width cannot be a declaration this pipeline produced.

AXI-L rebuilt `semantic → validate → intent → validate → adapt`: `residual_decisions` **0 → 1**,
catalog unchanged at 296, `signal_constraints` unchanged at 56, and the emitted `.isf` `source_text`
**byte-identical** at 31,691 bytes / 296 signals / 138 rules. Whether a refused declaration's identity
and direction should SURVIVE its unreadable width is `SIGNAL-DECLARATION-ROW-DROP.4d`.

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
