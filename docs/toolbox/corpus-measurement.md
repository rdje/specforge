# SpecForge toolbox — cross-document & corpus measurement (§6)

> A part of SpecForge's diagnostic toolbox. The standing directive, the enforcement and
> acceptance-checklist contract, the quick chooser, the first-reach tools (§1-§4), and the
> diagnosis protocols stay in the landing, [`TOOLBOX.md`](../../TOOLBOX.md). Section numbering
> is continuous with it, so a citation like `TOOLBOX.md` §7.7 still names this entry.

## 6. Cross-document & corpus measurement

### 6.1 `scripts/measure_isf_completeness.py`
- **WHAT:** read-only per-surface ISF-lowering ledger over all persisted docs (registers/fields/message-
  fields/constraints/rules/transactions → lowered/partial/true-gap/honest-residual), plus per-doc intent
  category labels. The reproducer behind `DOC-INTENT-TAXONOMY.2`.
- **WHEN:** "what fraction of each document's intent reaches `.isf`, and what is honest-absence vs a true
  gap"; prioritizing a per-category lever.
- **HOW:** `python3 scripts/measure_isf_completeness.py` (reads `generated/`, writes no canonical state).

### 6.2 `corpus-cluster [--threshold <0.0-1.0>]`
- **WHAT:** clusters documents by their ADR-0006-safe structural fingerprint (count-buckets + fired
  extractor strategies), surfacing emergent families + each family's fired-extractor union — no vendor
  names (the shared structure IS the key).
- **WHEN:** "which documents share an extraction shape / form a family"; finding a family to attack.
- **HOW:** `cargo run --manifest-path Cargo.toml -- corpus-cluster --evidence-root generated/evidence_ir`

### 6.3 `learn-priors <intent-ir>...` / `corpus-kb`
- **WHAT:** `learn-priors` builds the advisory typed `CorpusMemory` prior store; `corpus-kb` refreshes the
  tracked corpus knowledge-base pages from reviewable evidence. Both are advisory, never mutate canonical IR.
- **WHEN:** inspecting / refreshing the cross-document learning plane.

### 6.4 `scripts/check_source_pdf_registry_currentness.pl`
- **WHAT:** read-only exact-membership oracle for the durable source corpus: Git-indexed PDFs below
  `corpus/` ↔ registry rows, code-derived document keys, parent directories, PDF signatures, and pinned
  `SourceIR` derivation seams.
- **WHEN:** adding, removing, or renaming a tracked source PDF; changing filename-to-key code; auditing
  whether a fresh clone has every reproducible source named exactly once.
- **HOW:** `perl scripts/check_source_pdf_registry_currentness.pl --report` (13 fail-closed cases run
  unconditionally through `LIVE-DOC-SIZE`; host-local libraries and `generated/` are not inputs).

### 6.5 `scripts/check_corpus_kb_currentness.pl`

- **WHAT:** read-only dependency/output oracle for the corpus KB's eleven managed Markdown regions and
  paired prior-candidate JSON. It binds the reviewed validation snapshot, all Git-indexed KG fixture
  inputs, exact managed and human-side regions, and the Rust producer seams.
- **WHEN:** changing a KG fixture, validation review boundary, corpus-KB producer, managed page, or
  prior-candidate projection; auditing that human synthesis survived a refresh.
- **HOW:** `perl scripts/check_corpus_kb_currentness.pl --report` (15 fail-closed cases run
  unconditionally through `LIVE-DOC-SIZE`; canonical IR and typed prior memory are forbidden outputs).

### 6.6 `scripts/check_derived_state_contracts.pl`

- **WHAT:** the neutral exact-field authority gate. It reads the bounded
  `doctrine/live_document_size/derived_state_contracts.jsonl` registry, proves that every declared path
  belongs to its current governed surface or an explicit repository-local control role, locates exact literal
  primary/secondary markers, distinguishes derive-on-read, verified copies, authored intent, and immutable
  evidence, and executes every declared copy verifier.
- **WHEN:** adding or changing a current-state version, hash, count, projection, resume field, selected next
  action, or revision-bound measurement; use it before deciding that a convenient copy is trustworthy.
- **HOW:** `perl scripts/check_derived_state_contracts.pl --report`. The unconditional `LIVE-DOC-SIZE`
  path also runs 47 neutral fail-closed cases and 25 project-adapter cases. Only
  `scripts/check_derived_state_authorities.pl` knows the local Cargo/Rust and FSMGen-gitlink comparisons;
  it consumes declared copy roles and contains no secondary path fallback. The neutral checker knows no local
  field IDs or roles and does not infer fields from dates, numbers, hash shapes, or prose.

### 6.7 `scripts/measure_actor_taxonomy_blast_radius.py`

- **WHAT:** read-only census of what a NEW term in `builtin_actor_taxonomy_role_in_text` would move,
  across all four surfaces that read it — the direction cell of a signal row (S1), a section heading
  ending in ` signals`/` inputs`/… (S2), a relation-actor name in the per-document by-role map (S3),
  and `unique_complementary_reader_actor_name`, whose exactly-one-opposite-name condition makes S4
  **non-monotone**: a new term can take a set from one name to two and DESTROY the complementary
  `Reads` relations a document already mints. Verdicts are `gain` / `loss` / `flip` / `restage`, never
  a bare count.
- **WHEN:** **before adding any actor-role term, and before believing that a taxonomy gap is the reason
  a direction cell fails closed.** `SIGNAL-DECLARATION-ROW-DROP.2d` measured the cost of guessing: the
  obvious reading was "add the six names GIC-600 uses"; the census answered that six product names are
  ADR-0006-forbidden, that the one admissible pair (`source`/`sink`) is 60 % false positives and
  destroys a third document's relations, and that **a partial pair is worse than no pair** — one term
  of a flow's two endpoints makes the literal actor-text reading answer before the arrow reader and
  give the SAME direction to both senses of the link (3 pairs / 28 rows, measured).
- **HOW:** `python3 scripts/measure_actor_taxonomy_blast_radius.py` for the discovered candidates and
  their per-surface blast radius; `--vocabulary 'a=requester,b=completer'` to size a whole term SET at
  once, which is the only unit that means anything; `--term 'x=completer'` for one hypothesis; `--json`
  for the full report.
- **OUTPUT:** site populations per surface (S1 3,940 / S2 300 / S3 771 on `2026-09-15`, S1
  cross-checking §6.8's `body rows examined`); candidates DISCOVERED from the corpus, never listed, so
  the script carries no vendor vocabulary; per-term changed sites with every distinct text printed
  verbatim for adjudication; and under `--vocabulary`, **flow-sense collapses** — opposite flows between
  one actor pair that the chain gives the same direction, the one oracle here that needs no vocabulary
  of its own.
- **IT CLASSIFIES A POPULATION AND IS NOT A CHECK ON THE RUST.** It replicates the taxonomy it measures,
  so its agreement with the code carries no information (`CLAIM_VERIFICATION.md` §2); the independent
  leg is the in-crate control suite. Its table boundary (`table_kind == signal_description`) is an
  over-approximation of the producers' own gate, so every count is an upper bound.

### 6.8 `scripts/measure_declaration_row_notations.py`

- **WHAT:** read-only census of the notations that decide a signal-description row's fate in the
  authoritative declaration reader — bracketed metavariable name cells, flow-arrow direction cells,
  enumerated legal-width cells, and a COLUMN whose cells are the literal direction words in a table
  whose header carries no `direction` keyword — each distinct form printed verbatim with its count.
- **WHEN:** before teaching the reader any table notation, and to re-derive the `SIGNAL-DECLARATION-ROW-DROP`
  populations. Same disclaimer as §6.7: it classifies the population, it does not check the Rust.
- **HOW:** `python3 scripts/measure_declaration_row_notations.py` (`--json` for the report).
- **OUTPUT** (`2026-09-15`, 78 documents / 602 tables / 3,940 body rows): 12 bracketed name cells,
  83 flow-arrow cells (18 admitted, 16 two-sense link cells, 49 unresolved actors), 7 enumerated
  widths, and **106 rows in 13 tables** under an unread literal direction column — the last reported
  per TABLE, because the table is the unit of adjudication. That figure was **124 / 15 for one
  revision**: `.2h.0` read all 15 and found 18 rows admitted on the single letter `O`, which two
  protocol-VERSION presence matrices use for *Optional*. The abbreviated spellings are gone from the
  census — measured 0 true positives, 18 false.
