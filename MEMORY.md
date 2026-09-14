# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.2f` CENSUS DONE `2026-09-15` (PROBE/DOC) — still
  blocked, and the leaf's recorded trade was wrong about BOTH sides.**
  `python3 scripts/measure_bit_range_width_cells.py` (self-test 9/9) over all 78 SourceIR/EvidenceIR
  pairs: `[hi:lo]` appears in **167 rows / 15 `signal_description` tables / 4 documents**, splitting by
  the table's own header into two scopes needing opposite answers.
  * **SIGNAL scope** (`Signal name` header): 9 tables, 130 rows, 81 already declared, **49 not** — of
    which **17 are `Unused`** and **32 are real MMU-700 wires**.
  * **GENERIC scope** (`Name`/`Field` header): 6 tables, 37 rows, **0 declared, all 37 newly minted**,
    every one a REGISTER FIELD (eMMC CID fields, `LTI_PORT_RESOURCE_LIMIT0..7`, `CIDR0..3`).
  **Counting both populations together for the first time** — the 32 newly admitted PLUS the 47 `.4d`
  handed over (already declared, would finally get a readable width) — the prize is **79 real**; the
  cost is **54 phantoms unscoped (59%)** or **17 scoped by the `Signal name` header (82%)**.
  **One unblocker candidate measured and REFUSED**: *a name cell repeating inside its own table*
  selects 237 rows / 37 texts and catches `1`(65)/`Output`(27)/`Input`(19)/`Unused`(16) — but also
  **real** `AxPROT`(8), `CXSCNTL`(7), `BRESP`(4), `RRESP`(4), `CXSDATA`(3), four `ar*_m`. The `Unused`
  prerequisite stands. `.2f` also needs the width-COLUMN choice fixed, not only bit-range parsing.
- Earlier: **`.4d`** answered NO a third time (24% `.1d`, 1-in-8 `.4b`, this) — an identity with no
  readable attribute is not a signal; teach the notation rather than drop the attribute. **`.4b`**
  made the refusal COUNTED AND NAMED; **`.4e`** shipped `replay-declarations` (`TOOLBOX.md` §5.6) —
  78 documents, **0 skipped**, 3,196 opened / 2,927 read / 269 refused / 94 unrecovered.
- Next action: **`SIGNAL-DECLARATION-ROW-DROP.2d`** (census the actor-taxonomy blast radius before
  touching `builtin_actor_taxonomy_role_in_text`); then `TEXT-LAYER-IDENTIFIER-SPLIT.1` (live VLM);
  `EXTRACTION-QUALITY-GAUGE.3j` (unsized). `.2f` is blocked on the `Unused` row and `.4c` is parked
  behind the owner-gated `(width 1)` emitter default (`KG-ISF-COMPLETENESS.2a`).
- In-flight uncommitted: none. No background job outstanding.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED), none due at 265; directive 16
  gates it on full CI. **Corpus CURRENT — 27/27 at semantic and intent, retention exactly 24.**
  `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of 3,000 lines; **`DOCTRINE_ENFORCEMENT.md`
  line 394 is 845 of the 1,024-byte `line_bytes_each` ceiling** — route new detail to §10 prose.
  **A new Rust file costs five registrations**: `commands/mod.rs`, `cli.rs`+`lib.rs` dispatch,
  `module_inventory.tsv`, `CLI_SURFACE_REGISTRY` in `converge.rs`, and the `inventory_files` literal
  in `tools/production-genericity-graph/src/lib.rs`; a function reading raw evidence also needs an
  `information_flow_boundary.tsv` row. **A book insert re-pins line-anchored regions in three claim
  registries — relocate BY CONTENT (find the line whose stored sha matches), never rehash in place** —
  and moves the `shipped_behavior` aggregate. Standing hazards live in fact cards:
  **[[a-dropped-declaration-row-is-usually-not-a-signal]]**,
  **[[declaration-replay-reads-the-legacy-stratum]]** (a census over `generated/` mixes strata — 51 of
  78 are legacy), **[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]** (name the
  BINARY PROFILE — `release` everywhere), **[[live-surface-edit-bookkeeping-chain]]**. Cap: 50 lines.
