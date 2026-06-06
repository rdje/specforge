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

## Task Tree

- ID: `GRITS-CROSS-TOOL` · Status: `active` · Children: `.1` `.2`
- ID: `GRITS-CROSS-TOOL.1` · Status: `done` · Goal: independent pdfplumber witness extractor.
  Delivered + run on APB (13 tables); finding recorded.
- ID: `GRITS-CROSS-TOOL.2` · Status: `pending` · Goal: page-alignment + table-matching + `grits_content`
  wiring + the qwen2.5vl 2nd witness (3-way consensus gold).

## Changelog

- `2026-06-06`: Created. `.1` pdfplumber witness done (13 tables on APB); finding: 2-tool overlap is
  sparse → the LLM-vision 3rd witness is needed (validates the multi-witness design). `.2` harness next.
