# REGISTER-CLASSIFIER-ENCODING-FP: stop DVM/encoding cross-reference tables being mis-read as register maps

## Metadata

- Tree ID: `REGISTER-CLASSIFIER-ENCODING-FP`
- Status: `done` (CLOSED)
- Roadmap lane: `R12`/`R15e` (ingest precision / completeness)
- Created: `2026-06-01`
- Owner: repo-local workflow
- Parent context: `CORPUS-HARDENING` follow-up candidate ("1 CHI register overlap (review)").
  Sibling of the closed `REGISTER-MAP-CLASSIFIER-PRECISION` tree (same detector→fix loop).

## Discovery (the virtuous loop in action)

The register-tiling closure-invariant detector flagged `register_field_overlaps: 1`
on the real CHI spec (`ihi0050_g … amba_chi`) — `2:0 overlap: bit [1]`. Diagnosing
the artifact (read-only) showed the "register" is a **phantom**:

```
reg_table_0170_000  register_name "2:0"  offset "2:0"
  fields: "3"(1:1) "4"(1:1) "5"(1:1) "6"(1:1)   ← four fields all claim bit [1]
```

8 of CHI's 20 `register_records` are phantoms named after bit ranges
(`2:0`, `13:11`, `21:14`, `37:22`, `39:38`, `48:46`, `55:51`, `63:56`), all from
two source tables:

- `table_0170` — caption **"Table B8.10: Security field encodings for each DVMType"**
- `table_0171` — its continuation ("Continued from previous page", caption null)

These are **DVM (Distributed Virtual Memory) field-encoding cross-reference
tables**, not register maps. The detector is correct (it found a real
contradiction); the *classifier* is wrong (it mis-typed an encoding table as a
register map and emitted phantom registers). A true precision bug.

### Root cause (`classify_table_kind`, embedded Python in `docling_backend.rs`)

1. `has_addr_col` trips on the **substring "addr"** inside the header
   `"X in REQ.Addr[x] DAT.Data[x]"` (a bit-position cross-reference, not an offset
   column).
2. `has_register_structure` trips on the body cell `"2:0"` (a colon bit range).
3. The **content-based encoding detector misses it**: it scans the first column
   for `SIGNAL[N]` bit-field refs or binary/hex literals, but the first column
   here holds **bare** positions (`2:0`, `3`, `4`, …) — no `\w+[..]` prefix, no
   literal — so `encoding_hits = 0`.
4. With the encoding path missed and the register gate tripped, the table falls
   through to `register_map`.

## Goal

Make `classify_table_kind` recognize encoding cross-reference tables as
`encoding` (their true kind) **before** the register-map gate, so the DVM tables
(and similar) no longer emit phantom bit-range-named registers — with **zero
regression** to the registers/signals/encodings already classified correctly
across the whole corpus.

## Design

Insert an **encoding cross-reference positive** after the existing content-based
encoding detection and before the register-map gate:

- `caption_is_encoding` = caption contains `"encoding"` and not `caption_is_payload`
  — author intent (catches `table_0170`, captioned "… field encodings …").
- `header_is_xref` = any header matches the `\bx in\b` cross-reference idiom
  (catches `table_0171`, the caption-less continuation whose header is
  `"X in REQ.Addr[x] …"` / `"X in SNP.Addr[x]"`).
- If either holds → return `"encoding"`.

Ordering rationale: the signal-description caption/header checks already run
first, so a genuine signal table that merely mentions "encoding" is unaffected;
this new positive only intercepts tables that would otherwise reach the
register-map gate.

## Non-Goals

- NOT touching the register-record extractor or any downstream Rust — the fix is
  one classifier decision; downstream simply stops seeing these as register maps.
- NOT a general encoding-table rewrite — minimal, corpus-validated gate only.

## Validation plan

`.venv-docling` is absent and the corpus tree moved, so full Docling re-ingest is
not feasible right now. Instead validate the **classifier decision directly** on
the real stored table inputs (the exact unit under change): a Python harness
extracts `classify_table_kind` (old vs new) and runs both over **every
`structured_tables` entry in every `generated/source_ir/*/source_ir.json`**.

Pass criteria:
- `table_0170` + `table_0171` (CHI) flip `register_map → encoding` (8 phantom
  registers eliminated at the source).
- **No other table** changes classification (zero regression) — or every change
  is reviewed and confirmed a true correction.

## Acceptance Criteria

- Classifier patched (encoding cross-reference positive before register gate);
  corpus-wide old-vs-new diff shows the two CHI tables flip to `encoding` and no
  unreviewed regressions; fmt + clippy clean; full CI green; book note; tree CLOSED.

## Task Tree

- ID: `REGISTER-CLASSIFIER-ENCODING-FP`
  Status: `done`
  Goal: stop DVM/encoding cross-reference tables emitting phantom registers
  Children: `.1`, `.2`

- ID: `REGISTER-CLASSIFIER-ENCODING-FP.1`
  Status: `done`
  Goal: own + diagnose (this file) — root-cause the CHI phantom registers to
    `classify_table_kind`; fix the discriminating signal; register. Docs-only.
  Verification: >
    passed (`2026-06-01`) — diagnosed via the CHI evidence_ir.json + source_ir.json:
    8 phantom bit-range-named registers from `table_0170`/`table_0171` (DVM field
    encodings, caption "… Security field encodings …" + continuation). Root cause
    pinned to `classify_table_kind` (`has_addr_col` substring "addr" + body bit-range
    + missed content-encoding). Fix designed (encoding cross-reference positive).
  Commit: `see Commit Log`

- ID: `REGISTER-CLASSIFIER-ENCODING-FP.2`
  Status: `done`
  Goal: >
    Implement the encoding cross-reference positive in `classify_table_kind`;
    validate corpus-wide (old-vs-new harness — CHI tables flip to `encoding`, no
    regression); fmt + clippy; full CI; book note; close.
  Acceptance: classifier patched; corpus diff clean; CI green; book; tree CLOSED.
  Verification: >
    passed (`2026-06-01`) — patched `classify_table_kind` (encoding cross-reference
    positive: `caption_is_encoding` ∨ `header_is_xref` → "encoding", before the
    register gate). Corpus-wide old-vs-new harness over **1,989 stored tables in
    18 corpus source_ir.json**: `table_0170` + `table_0171` flip `register_map →
    encoding` (all 8 CHI phantom registers eliminated; register_map total 9 → 7);
    signal_description unchanged (210 → 210); the other 32 changes are all
    `→ encoding` for caption-declared encoding tables (30 from `unknown`, 1 from a
    `feature_matrix` FP, 1 from a `timing_parameter` FP) — net precision gain, zero
    regression. fmt + clippy clean; full CI green (1196 tests). Book note in
    `pipeline/sourceir.md`.
  Commit: `see Commit Log`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `REGISTER-CLASSIFIER-ENCODING-FP.1` | `done` | owned + root-caused + fix designed |
| 2 | `REGISTER-CLASSIFIER-ENCODING-FP.2` | `done` | classifier patched; corpus-validated (8 phantoms gone, 0 regression); CI green; book |

Tree **CLOSED** (`2026-06-01`): the register-tiling closure-invariant detector
found a real CHI contradiction; the root cause (an encoding table mis-typed as a
register map) is fixed at the classifier and corpus-validated. The detector→
diagnose→owned-fix→corpus-regression-verified loop, again.

## Decisions

- `2026-06-01`: fix at the **classifier** (reclassify to `encoding`), not the
  register extractor — these tables are genuinely encoding tables; the right cut
  is at type assignment, and downstream needs no change.
- `2026-06-01`: dual signal (caption `"encoding"` + header `\bx in\b` idiom) —
  the caption catches `table_0170`, the header idiom catches the caption-less
  continuation `table_0171`. Consistent with the corpus-token style of the prior
  precision fix (TOC dotted-leaders, eMMC frame vocab).
- `2026-06-01`: validate via stored-table harness (no re-ingest available) — the
  classifier input is exactly the stored header/body/caption, so this is faithful.

## Open Questions

- The 11 unexplained CHI tables (timing_parameter / signal_description that
  produced 0 records) are a **separate** region-accounting finding — tracked under
  `CORPUS-HARDENING` follow-ups, not this tree.

## Blockers

- Full Docling re-ingest unavailable (`.venv-docling` absent; corpus tree moved) —
  mitigated by the stored-table harness, which validates the exact unit under change.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | root-caused CHI phantoms to `classify_table_kind`; fix designed; registered; docs-only | `passed` |
| `2026-06-01` | `.2` | classifier patched; corpus harness over 1,989 tables (CHI 2 tables register_map→encoding, 8 phantoms gone, register_map 9→7, signal_description 210→210, +32 true encoding corrections, 0 regression); fmt/clippy clean; full CI green (1196 tests); book note | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `REGISTER-CLASSIFIER-ENCODING-FP.1` | (folded into `.2` commit) | docs-only own+diagnose |
| `REGISTER-CLASSIFIER-ENCODING-FP.2` | `REGISTER-CLASSIFIER-ENCODING-FP.2 — reclassify DVM/encoding cross-reference tables as encoding (kill CHI phantom registers); close tree` | classifier + book + close |

## Changelog

- `2026-06-01`: Created — corpus-discovered CHI register-overlap FP root-caused to
  the table classifier mis-typing DVM field-encoding tables as register maps.
  Frontier → `.2`.
- `2026-06-01`: CLOSED — classifier patched + corpus-validated (8 phantoms gone,
  +32 true encoding corrections, 0 regression); CI green; book note.
