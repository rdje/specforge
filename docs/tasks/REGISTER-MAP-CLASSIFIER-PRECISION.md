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
  Status: `done`
  Goal: >
    Investigate the headers of the GENUINE register maps in the corpus (the
    real eMMC/APB/AHB/AXI register tables, distinct from the false positives
    above) so the tightening provably preserves them; design the precise
    predicate (address/offset + bits/field/access structure).
  Acceptance: discriminating predicate validated against real register tables' headers.
  Verification: >
    passed (`2026-05-31`) — analyzed every `register_map` table across I2C/eMMC/
    APB. **Header-keyword tightening is insufficient** (the cleanest finding):
    the I2C address table header `Target address | R/W bit | Description` trips
    both a bit-check ("bit") AND an access-check ("r/w") via "R/W bit", so
    "address + bits/access" would still keep it; eMMC TOC rows trip a bit-check
    via the `[177]` page-reference regex; coincidental "type"/"scope" trip access
    checks. The ONLY genuine register table in the sample is eMMC
    `name | field | bit | type | … | hs_ctrl_rel` (EXT_CSD field table) — explicit
    field+bit+type columns + real field names. **Conclusion: the robust
    discriminator must inspect BODY content** (≥2 rows with a bit-range cell —
    `\[\d+:\d+\]` / `\d+:\d+` — AND/OR an access-type token — `RO|RW|WO|RC|W1C|
    W1S` — and a field-name cell), mirroring the classifier's existing
    encoding-detection body inspection (lines ~294–303), NOT header keywords
    alone. (Note: feature-matrix + TOC + data-frame mis-classifications are the
    same over-trigger; fixing register-map precision via body structure also
    removes those from `register_map`.)
  Commit: `see Commit Log`

- ID: `REGISTER-MAP-CLASSIFIER-PRECISION.3`
  Status: `done`
  Goal: >
    Implement a body-structure register-map predicate in `classify_table_kind`
    (gate the address/access header branches on genuine register STRUCTURE —
    a colon bit-range or a standalone access-type token in headers/body).
  Acceptance: structure gate landed + corpus-regression-verified on the clear cases (FP↓, real registers kept); CI green.
  Verification: >
    passed (`2026-05-31`) — gated the `register_map` branches in
    `classify_table_kind` (`docling_backend.rs`) on `has_register_structure`: a
    colon bit-range (`\[?\d+:\d+\]?`, excludes single `[177]` page refs) or a
    standalone access-type token (`ro|rw|wo|rc|w1c|…`) in headers or body cells.
    **Verified by fresh re-ingest:** I2C `register_map` 4→0 (the 2 feature
    matrices now correctly `feature_matrix`, the 2 address tables `unknown`) and
    APB 1→0 (PPROT encoding); I2C re-validate: `register_records` 1→0,
    **`register_field_overlaps` 1→0 — the original tiling-overlap symptom is
    GONE**. **No regression** — the gate removes only tables with *no* register
    structure; genuine registers (incl. eMMC Table 143 `7:3`) carry bit-ranges/
    access tokens and are kept. fmt/clippy clean; full `scripts/run_ci.sh` green.
    **Honest scope:** this eliminates header-keyword-only false positives. It
    does NOT fix false positives that carry *genuine* bit-ranges but aren't
    registers (eMMC RPMB data frames `[511:316]`, register-name TOC indexes
    `[159:157]`) — structurally register-like; deferred to `.4`.
  Commit: `see Commit Log`

- ID: `REGISTER-MAP-CLASSIFIER-PRECISION.4`
  Status: `pending`
  Goal: >
    Exclude the bit-range-bearing non-registers a structure gate can't separate:
    eMMC RPMB **data-frame** layouts (column vocab `stuff bytes`/`nonce`/`block
    count`/`key (mac)`; captions `… Data Frame`/`Packet`/`Response`) and
    **tables of contents** (dotted-leader cells `....`, page-index structure).
    Force a CLEAN eMMC re-ingest (the cached `source_ir` did not re-classify),
    verify the RPMB/TOC FPs drop while genuine EXT_CSD register/field tables
    survive, refresh the `CORPUS-HARDENING` eMMC ledger counts; book note; close.
  Acceptance: RPMB/TOC FPs excluded + genuine eMMC registers kept (clean re-ingest); CI green; tree CLOSED.
  Verification: pending
  Commit: pending

## Current Frontier

| Order | Leaf | Status | Why |
| --- | --- | --- | --- |
| 1 | `.1` | `done` | owned + diagnosed with real-corpus evidence |
| 2 | `.2` | `done` | discriminator must be BODY-structure (header keywords fooled by "R/W bit"/`[177]`); design recorded |
| 3 | `.3` | `done` | structure gate landed + verified (I2C 4→0, APB 1→0, tiling symptom fixed, no regression); CI green |
| 4 | `.4` | `pending` | eMMC RPMB/TOC bit-range-bearing FPs (caption/column-vocab exclusion) — next |

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
| `2026-05-31` | `.2` | analyzed all register_map tables; header keywords proven insufficient ("R/W bit"/`[177]`/"type" fool bit/access checks); robust discriminator = BODY structure (bit-range + access-token + field-name rows); `.3` design recorded | `passed` |
| `2026-05-31` | `.3` | structure-gate landed in `classify_table_kind`; fresh re-ingest: I2C register_map 4→0, APB 1→0, I2C tiling overlap 1→0 (symptom fixed); no regression (genuine bit-range/access tables kept); fmt/clippy clean; full CI green (1181/0) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `REGISTER-MAP-CLASSIFIER-PRECISION.1` | `REGISTER-MAP-CLASSIFIER-PRECISION.1 — own + diagnose the over-eager register-map classifier` | docs-only; found by CORPUS-HARDENING |
| `REGISTER-MAP-CLASSIFIER-PRECISION.2` | `REGISTER-MAP-CLASSIFIER-PRECISION.2 — discriminator must be body-structure (header keywords fooled)` | docs-only; design grounded in real corpus |
| `REGISTER-MAP-CLASSIFIER-PRECISION.3` | `REGISTER-MAP-CLASSIFIER-PRECISION.3 — gate register_map on body-structure; fix I2C/APB false positives` | code (`docling_backend.rs`); verified + CI green; eMMC RPMB/TOC → `.4` |

## Changelog

- `2026-05-31`: Created — own a systemic table-classifier precision bug surfaced
  by `CORPUS-HARDENING` (bare `has_addr_col` mis-classifies feature matrices /
  TOCs / data-frames / address tables as `register_map` across I2C/eMMC/APB).
  Fix is precision-tightening, **corpus-regression-verified** (quality > speed —
  confirm real register headers in `.2` before changing the predicate in `.3`).
