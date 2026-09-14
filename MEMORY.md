# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`TEXT-LAYER-IDENTIFIER-SPLIT.2` CLOSED `2026-09-14` (PROBE/DOC) — NO RULE, and the census
  found something the leaf did not go looking for: THE SAME CELL SHAPE WANTS TWO DIFFERENT JOINS.**
  `a opcode` is `a_opcode` (underscore); `t PERIOD` is `tPERIOD` (concatenation). Both are a one-letter lead
  plus a word, and nothing in the cell says which. Over 573 boundary tables, in the column the reader reads:
  tier A (UPPER-CASE continuation) **9 cells / 2 reachable**; tier B (any word) **126 / 2**, of which **81 are
  TileLink and 0 reachable**; 1,677 tier-B matches outside it are the false-positive surface.
- **The document's own spelling is an exact discriminator**: asking whether the document writes the
  CONCATENATION picks **2 of 126 and is right about both** (eMMC `table_0221`, which DECLARES) and refuses all
  81 TileLink cells, because TileLink never writes `aopcode` — zero false positives against a same-shape
  counter-population of 81. **It still does not ship, for `.0`'s reason**: both cells are in one LEGACY table,
  so a rule changes 0 declarations in the current stratum. `MAX_LEAD_CHARACTERS = 2` keeps `AWSIZE, ARSIZE` /
  `HSELx a` / `ARMPAM [10:0]` / `Duty Cycle` untouched, verified against the tokenizer. Instrument:
  `scripts/measure_subscript_split_name_cells.py`; card `[[a-split-name-cell-does-not-say-which-join-it-wants]]`.
- **Fourth demonstration of the book-census blind spot, logged into `CLAIM-VERIFICATION-ADOPTION.9`** (which
  already owns it): this slice's book paragraph publishes "126 name cells" and the frozen census still reports
  346/346 and passes, because `cells` is not in the closed noun list. `.9`'s step (1) is now measured: **527**
  book lines carry a `<number> <word>` clause it cannot see, but most of those words are noise (`and` 25) —
  the real nouns are `constraints` 9, `tables` 7, `rows` 6, `behaviors` 6, `statements` 6, `bits`/`cycles`/
  `invariants` 5, `cells`. A curated ~9-noun addition is bounded work.
- Earlier today: **`CORPUS-CHAIN-CURRENCY` is EXHAUSTED** (`.0`-`.9` closed). `.8` gave the three corpus-replay
  entrypoints ONE binary predicate at the **release** profile (`--total` 18m45s → 1m59.2s; `check_chain_currency.sh`
  28m00s → 12m38.2s; ≈47 min of CI-tier doctrine → **14m37s**, verdicts byte-identical). `.9` turned the per-stage
  TOTAL probe ON (gate **4m13.0s → 5m30.1s**) and added **self-test 21**, which reads the SHIPPED DEFAULT.
- Next action: **pick a new tree.** Candidates: `TEXT-LAYER-IDENTIFIER-SPLIT.1` (needs a live VLM over three
  persisted figures; TileLink is unrebuildable, so demonstrate on the assets), `SIGNAL-DECLARATION-ROW-DROP.4b`
  /`.4c`/`.2d`/`.2f`, `CLAIM-VERIFICATION-ADOPTION.9` (now measured and ready), the unsized `.3j`. `.3k.9`
  stays `DO NOT SHIP YET`.
- In-flight uncommitted: none after this commit. No background job outstanding.
- Blockers: none. Push cadence **400** (director directive `2026-09-13`, FIXED), so none due at 260; directive
  16 gates it on full CI. **Corpus CURRENT — 27/27 accepted at semantic and intent, retention exactly 24.**
  `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of its 3,000-line ceiling; **`DOCTRINE_ENFORCEMENT.md`
  line 394 is 845 of the 1,024-byte `line_bytes_each` ceiling** — route new detail to §10 prose, never into that
  table cell. Standing hazards, each with a fact card: **say which BINARY PROFILE a probe cost was measured with**
  (now `release` everywhere). **A test scoring a table against a catalog THAT TABLE FED is circular.** **A control
  that passes its own configuration tests the MECHANISM, never the shipped default.** **A leaf that changes a
  READER moves every document it does not rebuild.** **`specforge validate` is NOT idempotent.** **A new fact card
  may not reuse a question key another card answers — and a new FINDING may not reuse a leaf another tree owns.**
  **An insert into a book chapter re-pins line-anchored regions in ALL THREE of `published_assertions`/
  `book_quantitative_claims`/`current_claim_census`, and a bulk digest refresh must NEVER touch a node carrying
  `start_line`** (`[[live-surface-edit-bookkeeping-chain]]`). **A new `scripts/` file must be STAGED.** **The
  doctrine driver runs no cargo gate**; `evidence.rs` tests are in **`-p specforge-core`**. Cap: 50 lines.
