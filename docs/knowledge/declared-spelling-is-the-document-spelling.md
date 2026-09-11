---
id: declared-spelling-is-the-document-spelling
title: 716 of 2085 legacy declarations carry a spelling their document never writes, and none of the current ones do — the uppercase came from a producer that no longer exists
answers:
  - "why does the persisted Avalon EvidenceIR say READDATA when the document writes readdata"
  - "does SpecForge uppercase declared signal names"
  - "where does the uppercase spelling in a declaration come from"
  - "is ARESETN in a persisted artifact a current defect"
  - "how many declared names are spelled in a case the document never uses"
  - "does known_signals fold a signal name's case"
  - "which to_ascii_uppercase calls in the evidence stage emit a name"
  - "why does folding a signal name's case matter beyond style"
  - "what disarms is_alpha_variant_placeholder"
  - "why is PSELx not affected by the interior lower-case rule"
date: 2026-09-11
status: current
tags: [evidence-ir, declarations, adr-0037, adr-0006, legacy-artifacts, signal-declaration-row-drop]
evidence: crates/specforge/src/ir/evidence.rs (signal_names_in_name_cell; synthesize_signal_declarations; is_alpha_variant_placeholder; a_declared_name_keeps_the_cell_s_own_spelling); crates/specforge/src/ir/nlp_relation_extract.rs (parse_nlp_relations); docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.3)
reverify: "join every table_signal_declaration_provenance name in generated/evidence_ir/*/evidence_ir.json against the token set of its own generated/source_ir/*/source_ir.json (content elements + all table cells); expect 716 of 2085 in the legacy stratum and 0 of 604 in the proof-carrying one; then cargo test -p specforge-core --lib a_declared_name_keeps_the_cell_s_own_spelling"
---

A persisted EvidenceIR that says `READDATA` for a document that writes `readdata` seventy-five times
looks like a live defect, and it is the obvious reading of the artifact. It is not one. **The
uppercase is in the legacy stratum only**, left by a producer that has since been removed.

Measured by joining every `table_signal_declaration_provenance` name against the complete token set
its own document writes:

| stratum | declarations | spelled in a case the document never uses |
| --- | ---: | ---: |
| legacy (inspection-only artifacts) | 2,085 | **716** across 17 documents |
| proof-carrying (current) | 604 | **0** across 4 documents |

The legacy examples name real conventions the fold destroys: `ARESETN` for `ARESETn` and `NRESET` for
`nRESET` (active-low polarity), `AMEVCNTRN_EL0` for `AMEVCNTRn_EL0` and `PSELX` for `PSELx` (an index
metavariable), `STREAMID` for `StreamID`, and 488 in MMU-700 alone where the document writes
`qactive_cg`.

**State the weakness of the current-side evidence, because it is real.** Only 4 of the 27
proof-carrying documents produce table declarations at all, and one of them is 462 of the 604 while
writing its signals upper-case natively — so a case-folding emitter would be nearly invisible in that
stratum. The artifacts are corroboration; the authority is the reader, pinned by
`a_declared_name_keeps_the_cell_s_own_spelling`, which asserts provenance names *and* emitted
sentences for five spellings the corpus actually uses, and which goes RED with exactly the legacy
spellings when a fold is reintroduced.

Nothing in the current evidence stage folds a name on the way out. Every reachable
`to_ascii_uppercase` is comparison-normalisation — the semantic-hint stripper, the column-scoring
closure inside `synthesize_signal_declarations`, condition-clause extraction — or belongs to the enum
surface rather than the signal one. `known_signals` is the opposite of a folder: `parse_nlp_relations`
resolves a model-proposed spelling **back to** the canonical declared one, and
`resolve_declared_signal_identifier` does the same for the deterministic path, failing closed on a
case-fold collision rather than picking arbitrarily.

## Why it is worth a control rather than a shrug

Two reasons, and the second is easy to miss. First, ADR 0037: case carries no alias authority, so an
emitted `ARESETN` is a **minted** identifier, not a grounded one. Second,
`is_alpha_variant_placeholder` refuses a relation-derived name by finding an **interior** lower-case
position in it (`[[alpha-variant-placeholder-is-not-a-wire]]`) — folding is precisely how that
false-positive control would be disarmed, two stages from where the folding happened. (`PSELx` is
unaffected by that rule either way: its `x` is final rather than interior, which is why the rule
requires `index + 1 < len`.)

The general shape is worth keeping separately from the case: **a persisted legacy artifact is evidence
about the producer that wrote it, never about the one running now.** Seventeen documents' worth of
`READDATA` is a fact about files, and reading it as a fact about the product is the mistake this card
exists to prevent. Compare `[[corpus-canonical-currency-and-ownership]]` for the same distinction
applied to coverage ratios.
