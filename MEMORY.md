# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.2f` CENSUS DONE `2026-09-15` (PROBE/DOC), and its FIRST
  CUT WAS WRONG — corrected `2026-09-15` under audit.** `.2f`'s own spec says *"`[hi:lo]` is
  `hi - lo + 1`"* and the census matched it, so both missed the **single-index form `[n]`** — how these
  tables write every 1-bit wire (`0 | [0] | lavalid`, `1 | [122] | laogv`, `2 | [37] | lrctag`).
  Reading only the ranged form keeps the wide buses and drops **every valid/qualifier signal**.
  **Caught by asking whether two published numbers COMPOSE**, not by re-reading: `.4d` handed `.2f` 47
  rows claiming a bit rule would give them a width — ranged-only, just **20 of 48** were even in a bit
  cell; with `[n]`, **46 of 48**. `.2f` must specify BOTH forms or ship the same hole.
  Corrected (`--self-test 12/12`): **279 bit cells / 15 tables / 4 documents**.
  * **SIGNAL scope** (`Signal name` header): 9 tables, 241 rows, 146 declared, **95 not** — **19
    `Unused`**, **76 real** MMU-700 wires.
  * **GENERIC scope** (`Name`/`Field`): 6 tables, 38 rows, **0 declared, all 38 newly minted**, every
    one a REGISTER FIELD (eMMC CID fields, `LTI_PORT_RESOURCE_LIMIT0..7`, `CIDR0..3`).
  **Prize 122 real** (76 newly admitted + 46 already-declared gaining a width; disjoint). Cost **19
  scoped by the `Signal name` header (87%)**, **57 unscoped (68%)**. Still BLOCKED on `Unused`: the
  *repeated-name* candidate refuses real signals — 237 rows / 37 texts, catching `1`(65)/`Output`(27)/
  `Input`(19)/`Unused`(16) but also `AxPROT`(8), `CXSCNTL`(7), `BRESP`/`RRESP`(4), `CXSDATA`(3).
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
  **[[a-single-index-bit-cell-is-a-width-of-one]]** (a notation census that reads one spelling is
  biased, not merely incomplete), **[[a-dropped-declaration-row-is-usually-not-a-signal]]**,
  **[[declaration-replay-reads-the-legacy-stratum]]** (a census over `generated/` mixes strata — 51 of
  78 are legacy), **[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]** (name the
  BINARY PROFILE — `release` everywhere), **[[live-surface-edit-bookkeeping-chain]]**. Cap: 50 lines.
