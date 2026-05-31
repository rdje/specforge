# REGISTER-MAP-CLASSIFIER-PRECISION: stop the over-eager `register_map` table classification

## Metadata

- Tree ID: `REGISTER-MAP-CLASSIFIER-PRECISION`
- Status: `active`
- Roadmap lane: `R8` (SourceIR Tier-1 capture — table classification)
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Fix a **systemic precision bug** in the structured-table classifier surfaced by
the `CORPUS-HARDENING` campaign: the register-map heuristic in
`ir/source/docling_backend.rs` (`classify_table_kind`) classifies a table as
`register_map` on a **bare address-keyword header match**
(`has_addr_col` over `["offset","address","addr","base"]`, line ~306–308),
which mis-fires on feature matrices, tables of contents, data-frame layouts, and
address-assignment tables — inflating `register_records` with non-registers.

## Evidence (real corpus, `2026-05-31`)

Actual tables classified `register_map` (headers shown):
- **I2C UM10204** (4): `Feature | Configuration…` + `I3C v1.0 | I3C Basic…`
  (feature matrices) and `Target address | R/W bit | Description` ×2 (Table 4,
  the reserved-address *assignment* table) — **none are registers**. The tiling
  detector flagged the overlap on the address table (the original symptom).
- **eMMC JESD84-B50** (18): `Page | Scope… | Normative reference…` (a **table of
  contents**) and `Start | Stuff Bytes | … | Address | Block Count | …` (RPMB
  **data-frame** layouts) — the "Address" column triggers `has_addr_col`.
- **AMBA APB** (1): `PNSE | PPROT[1] | Physical address space` — a PPROT
  **encoding** table; "Physical address space" matches "address".

Root cause: `if has_addr_col or (has_access_col and has_name_col): return
"register_map"` — the `has_addr_col` **alone** branch is too weak. A register
map needs an address/offset column **together with register-field structure**
(bit ranges, field names, or access), which none of the false positives have.

## Non-Goals

- NOT loosening anything — purely tightening precision; must NOT reduce recall of
  **real** register maps (the discriminating fix must be regression-verified
  against the corpus, not guessed).
- NOT changing the downstream register synthesis — only the `table_kind` gate.

## Acceptance Criteria

- `classify_table_kind` requires register-field structure alongside an
  address/offset column (e.g. an address/offset col AND (bits/field col OR access
  col); keep the existing `access AND name` branch).
- **Corpus regression-verified** (the load-bearing requirement): re-run Docling
  on I2C / eMMC / APB (and ideally AHB/AXI) and confirm (a) the false-positive
  tables above are NO LONGER `register_map`, and (b) genuine register maps still
  classify (real `register_records` preserved — verify by inspecting the surviving
  tables' headers, not just the count).
- A `validate` re-run shows the I2C tiling overlap finding gone (the address
  table is no longer a register); full `scripts/run_ci.sh` green; book note if a
  user-facing behavior/section changes.

## Task Tree

- ID: `REGISTER-MAP-CLASSIFIER-PRECISION`
  Status: `active`
  Goal: tighten the register-map table classifier; corpus-regression-verified
  Children: `.1`, `.2`, `.3`

- ID: `REGISTER-MAP-CLASSIFIER-PRECISION.1`
  Status: `done`
  Goal: own + diagnose (this file) with real-corpus evidence; register.
  Verification: >
    passed (`2026-05-31`) — diagnosed via `CORPUS-HARDENING`: the bare
    `has_addr_col` branch in `classify_table_kind` (`docling_backend.rs` ~308)
    mis-classifies feature matrices / TOCs / data-frame layouts / address-
    assignment tables as `register_map` across I2C/eMMC/APB (evidence above).
    Owned + registered.
  Commit: `see Commit Log`

- ID: `REGISTER-MAP-CLASSIFIER-PRECISION.2`
  Status: `pending`
  Goal: >
    Investigate the headers of the GENUINE register maps in the corpus (the
    real eMMC/APB/AHB/AXI register tables, distinct from the false positives
    above) so the tightening provably preserves them; design the precise
    predicate (address/offset + bits/field/access structure).
  Acceptance: discriminating predicate validated against real register tables' headers.
  Verification: pending
  Commit: pending

- ID: `REGISTER-MAP-CLASSIFIER-PRECISION.3`
  Status: `pending`
  Goal: >
    Implement the tightened `classify_table_kind`; re-ingest I2C/eMMC/APB(+AHB/AXI)
    and verify the false positives drop while real register maps survive; re-validate
    (I2C tiling overlap gone); full CI; book note; close. Update the
    `CORPUS-HARDENING` ledger with corrected register counts.
  Acceptance: precision fix landed + corpus-regression-verified; CI green; tree CLOSED.
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why |
| --- | --- | --- | --- |
| 1 | `.1` | `done` | owned + diagnosed with real-corpus evidence |
| 2 | `.2` | `pending` | confirm the discriminating predicate against REAL register headers — next |
| 3 | `.3` | `pending` | implement + corpus-regression-verify + close |

## Decisions

- `2026-05-31`: regression-verify against the corpus (re-ingest), not just a unit
  test — the Python classifier is empirical and a wrong tightening could silently
  drop real registers (a recall regression). Quality > speed: confirm the real
  register headers first (`.2`) before changing the predicate (`.3`).

## Open Questions

- Do the genuine eMMC register tables (CSD/EXT_CSD field definitions) carry
  bits/field/access headers (so the tightening keeps them)? — resolve in `.2`.
- Should the feature-matrix / TOC mis-classifications be fixed here too, or only
  the register-map over-trigger? (Likely just register-map precision here; the
  others are separate `table_kind` precision items.)

## Blockers

- None (Docling + corpus ready).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `.1` | systemic over-eager `has_addr_col` diagnosed with I2C/eMMC/APB evidence; owned + registered | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `REGISTER-MAP-CLASSIFIER-PRECISION.1` | `REGISTER-MAP-CLASSIFIER-PRECISION.1 — own + diagnose the over-eager register-map classifier` | docs-only; found by CORPUS-HARDENING |

## Changelog

- `2026-05-31`: Created — own a systemic table-classifier precision bug surfaced
  by `CORPUS-HARDENING` (bare `has_addr_col` mis-classifies feature matrices /
  TOCs / data-frames / address tables as `register_map` across I2C/eMMC/APB).
  Fix is precision-tightening, **corpus-regression-verified** (quality > speed —
  confirm real register headers in `.2` before changing the predicate in `.3`).
