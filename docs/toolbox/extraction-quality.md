# SpecForge toolbox — extraction quality (§5)

> A part of SpecForge's diagnostic toolbox. The standing directive, the enforcement and
> acceptance-checklist contract, the quick chooser, the first-reach tools (§1-§4), and the
> diagnosis protocols stay in the landing, [`TOOLBOX.md`](../../TOOLBOX.md). Section numbering
> is continuous with it, so a citation like `TOOLBOX.md` §7.7 still names this entry.

## 5. Extraction quality (deeper measurement)

### 5.1 `nli-verify <evidence-ir>`
- **WHAT:** entailment-checks the extracted `signal_constraints` against the document's own sentences and
  persists an `extraction_quality_gauge` (entailed / not-entailed / abstained + the not-entailed ids).
- **WHEN:** "how trustworthy is this document's constraint extraction"; measuring the LLM-primary promotion.
- **HOW:** `cargo run --manifest-path Cargo.toml -- nli-verify generated/evidence_ir/<key>/evidence_ir.json`
- **OUTPUT:** the gauge counts + the exact not-entailed constraint ids (also reported by `validate`).

### 5.2 `eval-extraction <dataset>`
- **WHAT:** scores extraction against a labeled dataset (precision/recall over the labeled items).
- **WHEN:** measuring an extractor change against a held-out labeled set.
- **HOW:** `cargo run --manifest-path Cargo.toml -- eval-extraction <dataset> --evidence-root generated/evidence_ir`

### 5.3 `audit-extraction <source-ir> [--sample <n>] [--seed <s>]`
- **WHAT:** samples extracted items for a reproducible spot audit (seeded).
- **WHEN:** a quick honesty spot-check of a document's extracted surface.
- **HOW:** `cargo run --manifest-path Cargo.toml -- audit-extraction generated/source_ir/<key>/source_ir.json --sample 20 --seed 0`

### 5.5 `replay-constraints <evidence-ir>` / `--evidence-root <root>`
- **WHAT:** re-runs the REAL deterministic constraint producer over a persisted artifact's own
  `extracted_statements` and reports, per published record, whether today's code still mints it — and
  when it does not, which gate stands in the way.
- **WHEN:** **before sizing any extractor change.** A count over `generated/` measures what SpecForge
  PUBLISHED; only 24 of 78 documents keep a normalized bundle, so the rest are frozen at the generation
  that wrote them and can carry records the current code would never produce. `EXTRACTION-QUALITY-GAUGE.3k.1`
  sized itself on four published records and found the current extractor reproduces none of them.
- **HOW:** `cargo run --manifest-path Cargo.toml -- replay-constraints generated/evidence_ir/<key>/evidence_ir.json`
  or `-- replay-constraints --evidence-root generated/evidence_ir` for the corpus totals. Read-only: no
  provider, no write, and it reads the legacy/proofless stratum the canonical loader refuses.
- **OUTPUT:** `reproduced` / `not_reproduced` with the refusing gate per record; `granted_declarations`
  (published subjects the artifact no longer declares, granted one so their records still get a trial);
  a named skip for every artifact that would not load.
- **READ THE VERDICT ASYMMETRICALLY:** "not reproduced" is sound, because the replay runs a widened
  catalog and a wider catalog can only admit more subjects. `unpersisted_replay_records` is not a drift
  measure — the build applies convergence stages this replay does not.
- **CHECK WHICH STRATA IT JUDGED** (`EXTRACTION-QUALITY-GAUGE.3k.2g`). All three deterministic
  producers are replayed — the statement path, the dynamic path and the table-row path — but the row
  path needs the document's own `SourceIr`, and specifically one whose typed classifications survived
  loading. A legacy artifact is loaded with every `table_kind` neutralized to `Unknown`, so a pass
  keyed on `SignalDescription` selects nothing and returns an empty result that reads like "this
  document states none". The report therefore prints `row_stratum_judged_documents` and
  `row_stratum_unjudged_documents`, and the second number is a population you cannot measure, not a
  population you measured as zero (`[[legacy-source-classifications-are-neutralized-on-load]]`).
  Prior guidance is not applied either, so a table promoted to `SignalDescription` only by corpus
  memory is invisible here.

### 5.6 `replay-declarations <evidence-ir>` / `--evidence-root <root>`
- **WHAT:** re-runs the REAL SemanticIR declaration reader (`read_explicit_signal_declaration`) over a
  persisted artifact's own `Signal <name> …` statements and reports what it READS, what it REFUSES and
  under which of its three arms, and which refused identities no read declaration in the same document
  mints.
- **WHEN:** **before sizing any declaration-reader change, and whenever a catalog looks short.** The
  sibling of §5.5, for the same measured reason one stage down: `specforge semantic` refuses 51 of the
  78 persisted EvidenceIRs as legacy/proofless, so their SemanticIR catalog is an older binary's output
  and a census over it says nothing about today's reader. `SIGNAL-DECLARATION-ROW-DROP.4b` measured the
  cost of not having this: of the 75 signals declared in EvidenceIR and missing from the persisted
  catalog, **74 are in documents the current chain cannot reproduce**.
- **HOW:** `cargo run --manifest-path Cargo.toml -- replay-declarations generated/evidence_ir/<key>/evidence_ir.json`
  or `-- replay-declarations --evidence-root generated/evidence_ir` for the corpus totals; `--json` for
  the full report. Read-only: no provider, no write, and it reads the legacy/proofless stratum.
- **OUTPUT:** `opened` (sentences that opened as a declaration — one that never did is not an event and
  is not counted), `read`, `refused` split by arm (`name_not_an_identifier`,
  `no_direction_and_no_width`, `width_text_unread`), and `unrecovered`; each refusal prints its
  statement id and the sentence the reader saw.
- **`unrecovered` IS COMPUTED FROM THIS REPLAY, NEVER FROM THE PERSISTED CATALOG.** A refusal whose
  identity another statement declares successfully has lost nothing, and joining against the persisted
  SemanticIR would answer a question about the binary that wrote it. Corpus population on
  `2026-09-14`: 78 documents replayed, **0 skipped**, 3,196 sentences opened, 2,927 read, **269
  refused** (186 `width_text_unread`, 80 `no_direction_and_no_width`, 3 `name_not_an_identifier`) and
  **94 unrecovered identities** across 24 documents, MMU-700 alone holding 49.
- **THE ARMS ARE NOT INTERCHANGEABLE** (`SIGNAL-DECLARATION-ROW-DROP.4b`). A `no_direction_and_no_width`
  refusal is usually English prose that opens with the word "signal" — *"Signal names MUST adhere to
  the rules of the native tool"* — because the evidence stage only ever synthesizes a declaration from
  a row that yielded an attribute. `width_text_unread` is the arm where a real declaration was lost.

### 5.4 `grits-consensus <witnesses-json>`
- **WHAT:** scores how faithfully Docling read a document's TABLES against a CROSS-TOOL consensus gold
  (docling vs pdfplumber's geometric read vs qwen2.5vl's vision read), never against itself; flags
  split cells for adjudication (`--adjudicate-out <queue>`).
- **WHEN:** a suspected table mis-read (merged words, dropped cells) — the loop caught a real docling
  word-merge (`forAPB5` where the page prints `for APB5`).
- **HOW:** offline `.venv-eval` path (`scripts/grits_cross_tool.py` emits witnesses); see
  `docs/book/src/commands/quality-and-learning.md`.
