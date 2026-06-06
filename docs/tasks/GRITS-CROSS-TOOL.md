# GRITS-CROSS-TOOL: a table-structure gold from independent-witness agreement

## Metadata

- Tree ID: `GRITS-CROSS-TOOL`
- Status: `active` (`.1` witness done; `.2` harness + LLM-vision witness next)
- Roadmap lane: `R16`/`R15e` (eval quality — unblock the gated GriTS metric)
- Created: `2026-06-06`
- Parent context: `TABLE-GRITS-CONFORMAL` built `grits_content` but it was "gated on data" (no
  table-structure gold). Owner discussion: derive gold from **independent witnesses**, not fake it.

## Policy (why a second/third tool, not a replacement)

You cannot grade an extractor against *itself* (circular). docling stays the production **extractor**;
the **gold** is the consensus of tools whose errors are *uncorrelated* with docling's. Independence is
the currency:
- **pdfplumber** — geometric / PDF content-stream (no ML). *Most orthogonal* to docling (ML). Strong
  on RULED tables, weak on borderless. The primary witness.
- **qwen2.5vl (LLM-vision)** — ML, from the rendered image. *Partially* correlated with docling (both
  ML), but strong on BORDERLESS/complex tables → a coverage-extender + tiebreaker (we already have it).

Scheme (weak-supervision / consensus, the κ idea applied cross-tool): all-agree → high-confidence
gold; the two witnesses agree *against* docling → docling's likely error (triangulated); all-disagree →
human review.

## `.1` — pdfplumber witness (DONE `2026-06-06`)

`scripts/pdfplumber_tables.py <pdf>` (run via a gitignored `.venv-eval`, since the system Python is
PEP-668-managed) → `{"tables":[{"page","rows":[[cell,…]]}]}`. **On the real APB: pdfplumber found 13
tables vs docling's 24.**

**Finding (the empirical case for the 3rd witness):** the two tools detect tables in largely
*different* places (pdfplumber on the ruled timing tables pp.20–36; docling on signal/revision tables
elsewhere) — so the **2-tool overlap is sparse**, and cross-tool agreement *alone* yields thin gold on
APB. The qwen2.5vl witness is needed to densify the agreement set (it sees the borderless tables
pdfplumber misses).

## `.2` — the harness (NEXT)

- A Rust path (command or eval fn) that: loads docling grids (SourceIR `structured_tables` →
  header+body rows), runs the pdfplumber witness, **aligns pages** (docling `page_0NNN` vs pdfplumber
  physical index — they did not line up 1:1 on APB; needs an offset/label mapping), matches tables,
  and runs the existing `grits_content` per matched pair → cross-tool score + the agreement gold.
- Add the **qwen2.5vl witness** (table-image crop → grid) as the 2nd witness → 3-way consensus gold.

## `.2` — full harness + qwen2.5vl witness (DONE `2026-06-06`) — the metric is UNBLOCKED

- **Rust (owns the metric):** `eval::witness_consensus(witnesses, min_agree) -> {gold, disagreements}`
  (a cell ≥`min_agree` witnesses agree on is silver gold; a split cell is human-flagged) +
  `grits_against_consensus(gold, prediction)`; `grits-consensus <witnesses_json>` command. Tested.
- **Python (owns witness extraction):** `scripts/grits_cross_tool.py` — pdfplumber + qwen2.5vl
  witnesses + docling (the SourceIR grids = the *prediction*), matched by physical page; emits the
  witness JSON.
- **Demonstrated on the real APB (docling vs qwen2.5vl GriTS):** `table_0001` **F1=0.917** (strong
  agreement → docling validated), `table_0002` **F1=0.034** (strong disagreement → flag), `table_0003`
  F1=0.229. **The metric discriminates** — real gold from an independent witness, no faking. The
  gated metric is unblocked.
- **Finding deepened:** pdfplumber (geometric) and docling (ML-semantic) disagree on *what a table is*
  (pdfplumber found nothing on the semantic-table pages 2/5/6 — it catches geometric grids docling
  ignores). So **qwen2.5vl is the apt *semantic* witness** (shares docling's table-notion);
  pdfplumber is the orthogonal *ruled-table* witness. Exactly the complementarity argued for.

## `.3` — refinements (DONE `2026-06-06`)

- **Content-based matching:** the orchestrator now gates each witness by cell-overlap with the
  docling table (`MIN_OVERLAP`) — a witness below the gate is a *mismatch*, never paired. So we always
  compare the SAME table, and the VLM is cached per page.
- **Adjudication queue (the human-flag workflow):** `WitnessConsensus.disagreements` now carries the
  COMPETING `(text, witness_count)` per split cell (most-supported first); `grits-consensus` prints
  `adjudicate (r,c): "a"×1 vs "b"×1`. **The adjudicator is an evidence-grounded *agent* (or a human)**
  — resolves each against the rendered source, never a correlated vote (bounded-LLM: witnesses
  propose, the source decides). Tested.
- **3-way consensus:** the machinery supports `min_agree=2` (demonstrated by the synthetic 2-witness
  test). *Real-APB* 3-way is sparse: with the content-gate on, pdfplumber and docling rarely extract
  the SAME table (they disagree on what a table *is*), so content-matched 2-witness tables are rare —
  itself the finding, not a gap.

## Adjudication loop — WIRED + exercised (`2026-06-06`)

The full closed loop, with the agent as the evidence-grounded judge:
1. `eval::gold_vs_prediction_mismatches(gold, prediction)` → the positions where docling disagrees
   with the consensus gold (each a candidate docling error). Tested.
2. `grits-consensus --adjudicate-out <queue.json>` → writes the disputed cells (`table_id, page,
   row, col, consensus_gold, docling`).
3. `scripts/grits_adjudicate.py <queue.json> <pdf> <dir>` → renders each disputed table's page so
   the cells can be ruled against the SOURCE; attaches an `image` path per cell.
4. **The agent reads the rendered image and rules** — never a correlated vote.

**Exercised on the real APB:** the queue surfaced `table_0001` cells `(r4,c3)`/`(r5,c3)` — docling
`"for​APB5"` vs the qwen2.5vl gold `"for APB5"`. Reading the rendered page-2 source: it prints
`"for APB5"` (space present) → **docling WRONG on both (a space-drop / word-merge artifact)**. The
cross-tool loop flagged a genuine docling extraction bug, and the agent confirmed it against ground
truth.

## Task Tree

- ID: `GRITS-CROSS-TOOL` · Status: `done` (metric unblocked + refined + adjudication loop wired;
  `.1`–`.3` done) · Children: `.1` `.2` `.3`
- ID: `GRITS-CROSS-TOOL.1` · Status: `done` · Goal: independent pdfplumber witness extractor.
  Delivered + run on APB (13 tables); finding recorded.
- ID: `GRITS-CROSS-TOOL.2` · Status: `done` · Goal: full harness (consensus machinery + command +
  orchestrator + qwen2.5vl witness). Demonstrated on APB; metric unblocked. Verification above.
- ID: `GRITS-CROSS-TOOL.3` · Status: `done` · Goal: content-based matching + adjudication queue
  (agent-as-judge) + 3-way consensus. Verification above.

## Changelog

- `2026-06-06`: Created. `.1` pdfplumber witness done (13 tables on APB); finding: 2-tool overlap is
  sparse → the LLM-vision 3rd witness is needed (validates the multi-witness design). `.2` harness next.
