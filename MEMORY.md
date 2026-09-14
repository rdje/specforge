# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.4e` CLOSED `2026-09-14` (CODE/DOC) — `replay-declarations`
  reads the LEGACY stratum, so the declaration reader is measurable on 78 documents instead of 27.**
  The sibling of `replay-constraints` one stage down (`TOOLBOX.md` §5.6): the declaration surface is a
  pure function of `extracted_statements` — no `SourceIr`, no proof — so `specforge replay-declarations
  --evidence-root generated/evidence_ir` runs the REAL reader over every artifact. **78 scanned, 0
  SKIPPED**: 3,196 sentences opened as `Signal <name> …`, **2,927 read**, **269 refused** (186
  `width_text_unread`, 80 `no_direction_and_no_width`, 3 `name_not_an_identifier`), **94 unrecovered
  identities** over 24 documents, MMU-700 holding 49. The persisted-artifact census says 75 for the
  same corpus and **74 of those are legacy** — two populations, never summed.
  `unrecovered` is recomputed from the replay's OWN read set, never joined against the persisted
  catalog (the stale thing being replaced). **Cross-check**: on the current stratum the one document
  with an unrecovered `width_text_unread` is AXI-L, and `.4b`'s producer packet fires on exactly AXI-L.
  **0 of 27 artifacts move** (additive; no rebuild, no seal touched). **RED**: dropping the
  `!read_names.contains(name)` filter fails
  `declaration_replay_does_not_call_a_signal_lost_when_another_statement_declares_it`.
  Registered `diagnostic.declaration_replay` / `non_authoritative_root` / `diagnostics_only` —
  `boundary_rows` **143 → 144**, `non_authoritative_regions` **8 → 9**, the SECOND deliberate move of
  that boundary after `EXTRACTION-QUALITY-GAUGE.3k.6`.
- Earlier: **`.4b`** made the refusal COUNTED AND NAMED (`semantic_unreadable_declaration_width`),
  reporting only the `width_text_unread` arm because the other two are English prose opening with the
  word "signal" (**1 real in 8** vs **1 in 1**); AXI-L rebuilt, `.isf` byte-identical.
- Next action: **`SIGNAL-DECLARATION-ROW-DROP.4d`**, now sizeable — should a refused declaration's
  identity and DIRECTION survive? The adjudication set is the **76** `width_text_unread` refusals with
  an unrecovered identity (MMU-700 51, Avalon 8, GICv3 6, AXI-H 5, CXS 2, four singletons). Prior
  rulings on the same question scored **24%** (`.1d`) and **1-in-8** (`.4b`). Then `.4c`, `.2f`, `.2d`;
  `TEXT-LAYER-IDENTIFIER-SPLIT.1`; `EXTRACTION-QUALITY-GAUGE.3j` (unsized).
- In-flight uncommitted: none. No background job outstanding.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED), none due at 263; directive 16
  gates it on full CI. **Corpus CURRENT — 27/27 at semantic and intent, retention exactly 24.**
  `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of 3,000 lines; **`DOCTRINE_ENFORCEMENT.md`
  line 394 is 845 of the 1,024-byte `line_bytes_each` ceiling** — route new detail to §10 prose.
  **A new Rust file costs five registrations**: `commands/mod.rs`, `cli.rs`+`lib.rs` dispatch,
  `module_inventory.tsv`, the `CLI_SURFACE_REGISTRY` in `converge.rs`, and the `inventory_files`
  literal in `tools/production-genericity-graph/src/lib.rs`; a function that reads raw evidence also
  needs an `information_flow_boundary.tsv` row. Standing hazards live in fact cards; the ones that bite
  most often are **[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]** (name the BINARY
  PROFILE — `release` everywhere), **[[live-surface-edit-bookkeeping-chain]]** (a book insert re-pins
  line-anchored regions in all three claim registries AND moves the `shipped_behavior` aggregate),
  **[[a-dropped-declaration-row-is-usually-not-a-signal]]**, and
  **[[declaration-replay-reads-the-legacy-stratum]]** (a census over `generated/` mixes strata — 51 of
  78 are legacy). Cap: 50 lines.
