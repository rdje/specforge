# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.2g` CLOSED `2026-09-15` (CODE) — a cell that states a
  FLOW is not an actor name.** `infer_signal_direction_from_actor_text` now declines any cell carrying
  a `FLOW_ARROW_FORMS`/`FLOW_ARROW_DISQUALIFIERS` marker, ahead of even the literal `input`/`output`
  substring readings, so the flow reader judges it. **ALL 78 rebuild BYTE-IDENTICAL** (`evidence
  --dry-run` before/after); core lib **1539**, `specforge` 472, fmt + clippy clean.
  **The build corrected `.2d`: "latent" was only half true.** The CHAIN moves nothing (**0 of 476**
  actor-text rows are flow cells) but the FUNCTION was already wrong — RED observed,
  `infer_signal_direction_from_actor_text("Interconnect → Slave", DestinationLike, None)` =
  **`Some("input")`**, a real Avalon `table_0014` cell: **a column header, not a rule, kept it out of
  an artifact.** Measured and NOT acted on: `synthesize_trapped_row_signal_declarations` has **no
  flow-arrow arm** despite its doc — 0 of 663 trapped rows carry a marker.
- Next action: **`SIGNAL-DECLARATION-ROW-DROP.2h.1`** — read the unread literal direction column.
  `.2h.0` adjudicated and the number MOVED: **124 / 15 / 6 is WITHDRAWN, it is 106 rows / 13 tables /
  4 documents** (18 rows were admitted on `O`, which LTI `table_0081` and AXI-Stream `table_0015` use
  for *Optional*; abbreviations measured 0 true / 18 false and are gone). **94 of 106 unconditional.
  SIZED: all 4 documents are LEGACY PROOFLESS — 0 persisted artifacts move, and 0 of the 24
  rebuildable documents carry such a column, so a byte-identical corpus passes TRIVIALLY and is NOT
  evidence; the oracle is an in-crate control over the corpus cell forms.** TMC `table_0074` is MIXED
  not rotated (6 rows name-LAST, 1 name-first) so the whole-table offset cannot fix it; HBM2
  `table_0076` yields **2, not 6**. Then `.2f` (blocked on `Unused`),
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
