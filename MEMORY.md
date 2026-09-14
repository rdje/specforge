# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- **`TEXT-LAYER-IDENTIFIER-SPLIT.2` CLOSED (PROBE/DOC) — NO RULE: the same cell shape wants TWO DIFFERENT
  JOINS.** `a opcode` is `a_opcode`, `t PERIOD` is `tPERIOD`, and nothing in the cell says which. Name column
  over 573 tables: tier A **9 / 2 reachable**, tier B **126 / 2** of which **81 TileLink, 0 reachable**. The
  document's own spelling is exact — **2 of 126, right about both** — but both are in one LEGACY table (eMMC has
  no retained bundle, evidence schema 2 vs 3), so a rule changes 0 declarations. `[[a-split-name-cell-does-not-say-which-join-it-wants]]`.
- **`CLAIM-VERIFICATION-ADOPTION.7.3` CLOSED (CODE/DOC) — a SELF-TEST CASE COUNT is a published assertion;
  THREE were stale at once** under a green gate. `scripts/report_self_test_totals.pl` emits each script's
  DECLARED total and three `derived` assertions bind the TOOLBOX lines to it, so the count is published once.
  It reads the declaration, not the self-test (`rebuild_stage_cascade.sh --self-test` builds the binary, 43.6 s),
  and the declaration is self-guarding, so the legs compose. **RED**: `total=21`→`22` fails naming both values.
- Active unit: **`CLAIM-VERIFICATION-ADOPTION.9` CLOSED `2026-09-14` (CODE/DOC) — the book census can now SEE
  the numbers on the page.** Three sized changes: the four nouns with a RECORDED miss (13 lines; the
  speculative 19-noun set was measured and refused); **up to TWO words between numeral and noun**, the gap the
  noun list cannot close ("126 **name** cells") — 0 words catches 1 of 4 demonstrated misses, 2 catches **4 of
  4**, a third adds 12 and catches nothing; and a **trailing word boundary**, a precision bug found by measuring
  (`signals?` matched inside "PHY **Signal**ing"), removing exactly 5 false positives plus a list-marker strip.
  **The noun-free option was REFUTED as a substitute**: 119 lines vs 61, overlapping by only 7. Denominator
  **346 → 458** across 25 files, 458/458 adjudicated (112 dated, **1 `incomplete`** — kg-bench's live "156
  tracked fixtures" — 1 authored threshold), **2 records DELETED** that had adjudicated false positives, one a
  `3.` list index read as "3 shards". **RED**: reverting the gap fails new self-test case 20 by name. Residual,
  asserted as bounds: three intervening words, and spelled numerals.
- Earlier: **`CORPUS-CHAIN-CURRENCY` EXHAUSTED** (`.0`-`.9`). `.8` gave the three corpus-replay entrypoints ONE
  binary predicate at the **release** profile (`--total` 18m45s → 1m59.2s; `check_chain_currency.sh` 28m00s →
  12m38.2s; ≈47 min of CI doctrine → **14m37s**, byte-identical). `.9` turned the per-stage TOTAL probe ON (gate
  **4m13.0s → 5m30.1s**) + **self-test 21**, which reads the SHIPPED DEFAULT.
- Next action: **pick a tree** — `TEXT-LAYER-IDENTIFIER-SPLIT.1` (live VLM over three persisted figures),
  `SIGNAL-DECLARATION-ROW-DROP.4b`/`.4c`/`.2d`/`.2f`, the unsized `.3j`. `.3k.9` stays `DO NOT SHIP YET`.
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
