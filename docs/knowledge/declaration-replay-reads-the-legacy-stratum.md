---
id: declaration-replay-reads-the-legacy-stratum
title: The declaration reader can be replayed over any persisted EvidenceIR, including the 51 the semantic stage refuses — so the current reader's behaviour is measurable on the whole corpus, not on 27 documents
answers:
  - "how do I see what today's SemanticIR declaration reader does with a legacy document"
  - "what is replay-declarations"
  - "can the declaration reader be run over a legacy or proofless EvidenceIR"
  - "how many declarations does the current reader refuse corpus-wide"
  - "why is a missing-signal census over generated/ not a statement about the current reader"
  - "which refusal arm of the declaration reader is a real loss"
  - "how is unrecovered computed in replay-declarations"
  - "how many Signal statements does the corpus carry"
date: 2026-09-14
status: current
tags: [semantic-ir, declaration, replay, legacy-stratum, signal-declaration-row-drop, measurement, toolbox]
evidence: crates/specforge/src/ir/semantic.rs (replay_persisted_signal_declarations, read_explicit_signal_declaration, DeclarationReplayReport); crates/specforge/src/commands/replay_declarations.rs; TOOLBOX.md §5.6; docs/book/src/commands/quality-and-learning.md; docs/tasks/SIGNAL-DECLARATION-ROW-DROP.md (.4b, .4d, .4e)
reverify: "./target/release/specforge replay-declarations --evidence-root generated/evidence_ir — expect 78 documents scanned, 0 skipped, 3196 opened, 2927 read, 269 refused (186 width_text_unread / 80 no_direction_and_no_width / 3 name_not_an_identifier) and 94 unrecovered across 24 documents. Contrast: python3 scripts/measure_declared_signals_missing_from_semantic.py reports 75, of which 74 are legacy."
---

**The problem this retires.** SemanticIR's catalog is built by `parse_explicit_signal_declaration`
reading each `Signal <name> is <direction> width <W>.` statement, and that catalog decides which
obligations are promoted at all. But `specforge semantic` refuses a legacy/proofless EvidenceIR
(`EvidenceIR schema version 2 is legacy/proofless and inspection-only`) for **51 of the 78** persisted
documents, so their persisted catalog is an older binary's output. A census over `generated/`
therefore measures what SpecForge PUBLISHED, not what today's reader does — the same trap
`[[persisted-census-measures-published-not-current]]` names for the constraint producer, at a
different function.

**The instrument.** `replay-declarations` (`SIGNAL-DECLARATION-ROW-DROP.4e`, `TOOLBOX.md` §5.6) is the
declaration-reader sibling of `replay-constraints`. The declaration surface is a pure function of the
artifact's own `extracted_statements` — no `SourceIr`, no proof context — so it replays offline for
**every** document. It calls the REAL reader; a re-implementation would answer a question about itself
(`CLAIM_VERIFICATION.md` §2).

**Two reporting rules that are load-bearing, not cosmetic.**

* A sentence that never opened as `Signal <name> …` is **not counted**. It is not an event, and
  counting it would bury the real refusals in prose.
* `unrecovered` is recomputed from **this replay's own read declarations**, never joined against the
  persisted SemanticIR catalog. A specification commonly declares the same wire twice, so a refusal
  whose identity another statement declares has lost nothing — and joining against the stored catalog
  would answer a question about the binary that wrote it.

**Corpus population, `2026-09-14`.** 78 documents replayed, **0 skipped**; 3,196 sentences opened as
declarations, 2,927 read, **269 refused** — 186 `width_text_unread`, 80 `no_direction_and_no_width`,
3 `name_not_an_identifier` — and **94 unrecovered identities** across 24 documents, MMU-700 alone
holding 49. The persisted-artifact census reports 75 for the same corpus, and **74 of those 75 are
legacy**: the two numbers are different populations and must never be summed or substituted.

**The arms are not interchangeable** (`SIGNAL-DECLARATION-ROW-DROP.4b`). `no_direction_and_no_width`
is nearly always English prose opening with the word "signal" — *"Signal names MUST adhere to the
rules of the native tool"* — because the evidence stage only synthesizes a declaration from a row that
yielded an attribute. `width_text_unread` is the arm where a real declaration was lost.

See `[[arithmetic-width-drops-the-declaration]]`,
`[[persisted-census-measures-published-not-current]]`,
`[[corpus-canonical-currency-and-ownership]]`,
`[[legacy-source-classifications-are-neutralized-on-load]]`.
