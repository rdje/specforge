# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- **`TEXT-LAYER-IDENTIFIER-SPLIT.2` CLOSED (PROBE/DOC) — NO RULE: THE SAME CELL SHAPE WANTS TWO DIFFERENT
  JOINS.** `a opcode` is `a_opcode` (underscore), `t PERIOD` is `tPERIOD` (concatenation), and nothing in the
  cell says which. Over 573 boundary tables, in the column the reader reads: tier A **9 / 2 reachable**, tier B
  **126 / 2** of which **81 are TileLink, 0 reachable**; 1,677 tier-B matches outside it are the false-positive
  surface. **The document's own spelling is an exact discriminator** — **2 of 126, right about both** (eMMC
  `table_0221`, which DECLARES), refusing all 81 TileLink cells. **Still no rule, for `.0`'s reason**: both are
  in one LEGACY table, so it changes 0 declarations in the current stratum. `MAX_LEAD_CHARACTERS = 2` keeps
  `AWSIZE, ARSIZE`/`HSELx a`/`Duty Cycle` untouched. `[[a-split-name-cell-does-not-say-which-join-it-wants]]`.
- Active unit: **`CLAIM-VERIFICATION-ADOPTION.7.3` CLOSED `2026-09-14` (CODE/DOC) — a SELF-TEST CASE COUNT is a published assertion and
  nothing bound one to its script; THREE were stale at once** under a green gate (TOOLBOX §7.2a/§7.2a-i said
  "sixteen" for checks at **22** and **21**; DOCTRINE_ENFORCEMENT repeated one). Correcting them was NOT the fix:
  `scripts/report_self_test_totals.pl` emits each script's DECLARED total and three `derived` assertions bind the
  TOOLBOX lines to it, so the count is published once. It reads the declaration, not the self-test, because
  `rebuild_stage_cascade.sh --self-test` builds the binary (43.6 s) and every derived producer runs on every
  gate; the declaration is self-guarding, so the legs compose. **RED observed**: `total=21`→`22` fails the gate
  naming both values. Reader self-test **5/5**.
- **Book-census blind spot: 4th demonstration + `.9`'s step (1) measured, into `.9` which owns it.** "126 name
  cells" is invisible because `cells` is not in the closed noun list; **527** book lines carry a `<number> <word>`
  clause it cannot see, mostly noise — the real nouns are `constraints` 9, `tables` 7, `rows`/`behaviors`/
  `statements` 6, `bits`/`cycles`/`invariants` 5, `cells`. **`.9` IS STILL OPEN — next to own.**
- Earlier: **`CORPUS-CHAIN-CURRENCY` EXHAUSTED** (`.0`-`.9`). `.8` gave the three corpus-replay entrypoints ONE
  binary predicate at the **release** profile (`--total` 18m45s → 1m59.2s; `check_chain_currency.sh` 28m00s →
  12m38.2s; ≈47 min of CI doctrine → **14m37s**, byte-identical). `.9` turned the per-stage TOTAL probe ON (gate
  **4m13.0s → 5m30.1s**) + **self-test 21**, which reads the SHIPPED DEFAULT.
- Next action: **`CLAIM-VERIFICATION-ADOPTION.9`** — measured and ready; widen the candidate grammar (curated
  nouns vs. a noun-free signal), adjudicate the new population, add a RED control. Alternatives:
  `TEXT-LAYER-IDENTIFIER-SPLIT.1` (live VLM over three persisted figures), `SIGNAL-DECLARATION-ROW-DROP.4b`/
  `.4c`/`.2d`/`.2f`, the unsized `.3j`. `.3k.9` stays `DO NOT SHIP YET`.
- In-flight uncommitted: none. No background job outstanding.
- Blockers: none. Push cadence **400** (directive `2026-09-13`, FIXED), none due at 261; directive 16 gates it
  on full CI. **Corpus CURRENT — 27/27 at semantic and intent, retention exactly 24.**
  `docs/tasks/EXTRACTION-QUALITY-GAUGE.md` is at 2,878 of 3,000 lines; **`DOCTRINE_ENFORCEMENT.md` line 394 is
  845 of the 1,024-byte `line_bytes_each` ceiling** — route new detail to §10 prose, never into that cell.
  Standing hazards live in fact cards, not here; the ones that bite most often are
  **[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]** (say which BINARY PROFILE a cost was
  measured with — `release` everywhere now), **[[live-surface-edit-bookkeeping-chain]]** (a book-chapter insert
  re-pins line-anchored regions in all three claim registries; a bulk digest refresh must NEVER touch a node
  carrying `start_line`), **[[a-dropped-declaration-row-is-usually-not-a-signal]]** (a test scoring a table
  against a catalog THAT TABLE FED is circular), and **a control that passes its own configuration tests the
  MECHANISM, never the shipped default** (`CORPUS-CHAIN-CURRENCY.9`). A new `scripts/` file must be STAGED; the
  doctrine driver runs no cargo gate; `evidence.rs` tests are in **`-p specforge-core`**. Cap: 50 lines.
