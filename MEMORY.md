# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.4d` CLOSED `2026-09-15` (PROBE/DOC) — NO for the THIRD
  time, and the biggest sub-population was not even this leaf's.** The question: should a declaration
  refused for an unreadable WIDTH keep its identity and its already-parsed direction? All **69**
  distinct refused identities adjudicated against their source rows via `replay-declarations`.
  * **47 are MMU-700's LTI observation interface and the document STATES their width.** `table_0259`
    (*"Table B-6: LTI TBU observation interface signals"*) + `table_0260` share
    `SIGNALGRP<n> | Bits | Signal name | SIGQUAL<n> 4'b{MSB..LSB} | …`; the row is
    `0 | [125:110] | latlbloc | 3'b000 , lavalid | 1`. **The reader takes the FOURTH column as the
    width and never reads the SECOND** — `[125:110]` is 16 bits, one cell away. Handed to **`.2f`**,
    now owning 47 real widths (not 15) + the column-choice defect; `Unused` is row 1 of both tables.
  * **18 are not signals**: Avalon section headings, AXI-H transaction names, GICv3 peripheral-ID
    register FIELDS, eMMC `NOTE`, and `group` from prose (which reaches this arm only because
    `output` parses as a direction). 20 of the 47 also carry spellings the document never writes
    (`LCVALID_0..7` from `lcvalid[7:0]`).
  * **4 real losses**, three malformed in the source. The fourth (`CXSACTIVEREQ is width 1 bit`) got
    its own measurement: `width <N> bit(s)` is **9 refusals in ONE document**, 7 of 8 identities
    already read elsewhere → recovers **1** corpus-wide, **0** current. Measured and REFUSED.
  **Rule now on three measurements (24% `.1d`, 1-in-8 `.4b`, this): an identity with no readable
  attribute is not a signal; teach the notation rather than drop the attribute.**
- Earlier: **`.4b`** made the refusal COUNTED AND NAMED (`semantic_unreadable_declaration_width`, one
  arm only); **`.4e`** shipped `replay-declarations` (`TOOLBOX.md` §5.6) — 78 documents, **0 skipped**,
  3,196 opened / 2,927 read / 269 refused / 94 unrecovered, which made `.4d` decidable at all.
- Next action: **`SIGNAL-DECLARATION-ROW-DROP.2f`** — 47 real MMU-700 widths + the width-column
  defect, still blocked on `PROSE-NAME-CELL-DECLARATION` deciding the `Unused` row. Then `.2d`;
  `TEXT-LAYER-IDENTIFIER-SPLIT.1`; `EXTRACTION-QUALITY-GAUGE.3j` (unsized). `.4c` stays parked behind
  the owner-gated `(width 1)` emitter default (`KG-ISF-COMPLETENESS.2a`).
- In-flight uncommitted: none. No background job outstanding.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED), none due at 264; directive 16
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
