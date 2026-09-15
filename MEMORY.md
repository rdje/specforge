# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.2g` CLOSED `2026-09-15` (CODE) — a cell that states a
  FLOW is not an actor name.** `infer_signal_direction_from_actor_text` now declines any cell carrying
  a `FLOW_ARROW_FORMS`/`FLOW_ARROW_DISQUALIFIERS` marker, ahead of even the literal `input`/`output`
  substring readings, so the flow reader judges it. **ALL 78 documents rebuild BYTE-IDENTICAL**
  (`evidence --dry-run` before/after, 24 accepted / 54 refused both sides); core lib **1539**
  (1537 + 2 controls), `specforge` 472, fmt + clippy clean.
  **The build corrected `.2d`: "latent" was only half true.** The CHAIN moves nothing (**0 of 476**
  actor-text-answered rows are flow cells) but the FUNCTION was already wrong — RED observed,
  `infer_signal_direction_from_actor_text("Interconnect → Slave", DestinationLike, None)` =
  **`Some("input")`**, a real Avalon `table_0014` cell. **A column header, not a rule, kept it out of
  an artifact** (Avalon heads it `Direction`, so `source_col`/`dest_col` are `None`). Measured and NOT
  acted on: `synthesize_trapped_row_signal_declarations` claims to mint "under the body-row path's own
  rules" but has **no flow-arrow arm** — 0 of 663 trapped rows carry a marker.
- Next action: **`SIGNAL-DECLARATION-ROW-DROP.2h.1`** — read the unread literal direction column.
  `.2h.0` adjudicated and the number MOVED: **124 / 15 / 6 is WITHDRAWN, it is 106 rows / 13 tables /
  4 documents** (18 rows were admitted on `O`, which LTI `table_0081` and AXI-Stream `table_0015` use
  for *Optional*; abbreviations measured 0 true / 18 false and are gone). **94 of 106 are
  unconditional**; `.2h.1` must still answer whether the name-column rotation override covers
  CoreSight TMC `table_0074` (direction first, name LAST, 6 real ATB wires) and count HBM2
  `table_0076` as **2, not 6** (prose name column). Then `.2f` (blocked on `Unused`),
  `TEXT-LAYER-IDENTIFIER-SPLIT.1`, `EXTRACTION-QUALITY-GAUGE.3j`; `.4c` behind `KG-ISF-COMPLETENESS.2a`.
- Also ready, measured this session, both in `CLAIM-VERIFICATION-ADOPTION`: **`.12`** (the per-slice
  region re-pin instrument — built in scratch and used 5×, relocates every `line_range_sha256` region
  BY CONTENT across all three registries) and a NEW leaf for a stale carried count — `claims.jsonl`
  `mdbook-quantitative-census-frozen` `durability.stale_check.stdout_contains` says **"309 adjudicated
  region(s)"**, the checker reports **464**, no gate executes `stale_check`, and that claim's own
  assertion forbids carrying a per-commit counter.
- In-flight uncommitted: none. No background job outstanding.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED), none due at 267; directive 16
  gates it on full CI. **Corpus CURRENT — 27/27 at semantic and intent, retention exactly 24.**
  `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of 3,000 lines; **`DOCTRINE_ENFORCEMENT.md`
  line 394 is 845 of the 1,024-byte `line_bytes_each` ceiling** — route new detail to §10 prose.
  **A new Rust file costs five registrations**: `commands/mod.rs`, `cli.rs`+`lib.rs` dispatch,
  `module_inventory.tsv`, `CLI_SURFACE_REGISTRY` (`converge.rs`), `inventory_files`
  (`tools/production-genericity-graph`); a raw-evidence reader also needs an
  `information_flow_boundary.tsv` row. **A book insert re-pins line-anchored regions in three claim
  registries — relocate BY CONTENT, never rehash in place** — and moves the `shipped_behavior`
  aggregate. Standing hazards live in fact cards: **[[actor-taxonomy-grows-in-pairs-not-terms]]**,
  **[[a-single-index-bit-cell-is-a-width-of-one]]**, **[[a-dropped-declaration-row-is-usually-not-a-signal]]**,
  **[[declaration-replay-reads-the-legacy-stratum]]** (51 of 78 are legacy),
  **[[prior-phrase-utf8-byte-as-char]]** (a corpus `evidence --dry-run` sweep is ~6 min, not hung),
  **[[live-surface-edit-bookkeeping-chain]]**. Cap: 50 lines.
