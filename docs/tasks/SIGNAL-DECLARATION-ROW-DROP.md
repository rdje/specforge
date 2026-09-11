# SIGNAL-DECLARATION-ROW-DROP: the authoritative declaration reader silently discards 18.3% of the rows it was given

## Metadata

- Tree ID: `SIGNAL-DECLARATION-ROW-DROP`
- Status: `active` (`2026-09-11`; `.0` census closed, `.1`–`.3` open)
- Roadmap lane: `R2` (extraction correctness / wire recall)
- Created: `2026-09-11`
- Last updated: `2026-09-11`
- Owner: repo-local workflow

## Goal

`synthesize_signal_declarations_from_table` is the path doctrine designates as **authoritative** for
signal declarations — table declarations win over everything derived. It ends its `(direction, width)`
match with:

```rust
_ => continue, // No direction AND no width — not enough info to synthesize
```

A row that reaches that arm is dropped with no record: no declaration, no residual, no counter, no
validation report entry. Measured over every persisted artifact, **482 of 2,637 rows — 18.3% —** are
discarded this way, from tables SpecForge itself typed `signal_description` with a clean
single-identifier name cell.

Make a row the reader cannot interpret **visible** rather than silent, and recover the interpretable
majority of the 482.

## How it was found

`WIRE-BASED-100.10f` opened on the belief that `synthesize_directions_from_relations` — the one path
that turns a relation into a declaration, against the SemanticIR authority doctrine — could not be
deleted, because eight genuine Avalon signals (`READDATA`, `WRITEDATA`, `ADDRESS`, `BURSTCOUNT`,
`BYTEENABLE`, `CHANNEL`, `DATA`, `ERROR`) reach EvidenceIR through it and nothing else. Two
deterministic discriminators were tried against the junk it also mints (`Manager`, `TCU`, `ATB`); the
leaf was about to conclude that only a bounded entity-typing tier could separate them.

The premise was false. Avalon's `table_0012` is typed `signal_description`, its first column is
`Signal Role`, and `readdata` and `writedata` are rows in it. They are dropped by the arm above,
because Avalon writes direction as the arrow form `Slave → Master` (which
`infer_signal_direction_from_actor_text` does not read) and writes `readdata`'s width as the
enumerated set `8, 16, 32, 64, 128, 256, 512, 1024` (which `infer_signal_table_row_width_hint` does
not read). Neither is inferable, so the row vanishes. Corroboration: **15 of Avalon's 26 surviving
declarations are width-only**, so the arrow form is never understood for any of them.

## Why no gate saw it

- **The scores cannot see it.** AXI drops **103** rows across its two specifications while scoring
  `1.000` on every aspect, because its gold names facts the surviving declarations already carry. This
  is the standing hazard — *a green score is evidence only about the facts its gold names* — with a
  three-digit number attached.
- **The chain-currency gate cannot see it.** A dropped row is not drift: the persisted artifact is
  exactly what the current binary produces. Replay is green precisely because the loss is deterministic.
- **Nothing counts the denominator.** The reader never reports rows-in versus declarations-out, so
  there is no ratio for any check to bound.

## The measured population

Per document, `signal_description` rows with a single-identifier name cell, versus declarations
actually emitted from those tables:

| document | rows | declared | dropped |
| --- | ---: | ---: | ---: |
| Cortex-A76 TRM | 302 | 195 | 107 |
| AMBA AXI/ACE (`ihi0022_h_c`) | 603 | 525 | 78 |
| AMD IOMMU | 59 | 0 | 59 |
| GIC-600 TRM | 80 | 25 | 55 |
| HBM2 DRAM (JESD235A) | 32 | 0 | 32 |
| AMBA AXI (`ihi0022_l`) | 376 | 351 | 25 |
| AMBA ATP | 25 | 0 | 25 |
| MMU-700 TRM | 392 | 368 | 24 |
| AMBA CHI C2C | 15 | 0 | 15 |
| AMBA CXS | 44 | 30 | 14 |
| **corpus** | **2,637** | **2,155** | **482 (18.3%)** |

Four documents lose **every** row (AMD IOMMU, HBM2, ATP, CHI C2C) — a whole-document failure mode, not
a long tail.

## Non-Goals

- Do not lower the bar to "declare every row". A row with no direction and no width may genuinely carry
  neither; the fix is to make that visible and to read the notations we currently fail to read, not to
  emit unfounded declarations.
- Do not add document-, vendor- or protocol-specific notation handling (ADR 0006). The arrow form and
  the enumerated-width form are *grammars*, admissible only as grammars.
- Do not treat this tree as licence to delete `synthesize_directions_from_relations`; that deletion is
  `WIRE-BASED-100.10f`'s, and it unblocks only once `.1` demonstrates the rows come back.

## Acceptance Criteria

- The reader reports its own denominator: rows considered, declarations emitted, rows dropped, per
  table, on the persisted artifact — so the 18.3% is a number a gate can bound rather than a number
  only a census can find.
- Every notation this tree teaches the reader ships with a corpus-wide count of what it newly admits
  **and a sample adjudicated for false positives**, per the standing finding that a cheap structural
  rule over-fires until its selection is inspected.
- No gold score moves down; the wire golds (APB/AHB/AXI at `1.000`) are re-scored, not assumed.
- `scripts/check_doctrines.sh` green; no ceiling, milestone, or contract widened.

## Task Tree

- ID: `SIGNAL-DECLARATION-ROW-DROP` · Status: `active` (`2026-09-11`) · Children: `.0`, `.1`, `.2`, `.3`

- ID: `SIGNAL-DECLARATION-ROW-DROP.0` · Status: `done` (`2026-09-11`) · Goal: **measure the drop before
  proposing a fix.** Establish whether the Avalon case is a document quirk or a corpus-wide loss.
  Result: corpus-wide, **482 of 2,637 rows (18.3%)**, across the population tabulated above; four
  documents lose 100% of their rows. The denominator counts only `signal_description` tables whose name
  cell is a single identifier, so it is a **lower bound** — multi-name cells (`read read_n`) and
  bracketed forms (`response [1:0]`) are excluded and may lose more.
  **Falsified in passing:** Avalon declares `READDATAVALI`, `WRITERESPONS` and `BEGINBURSTTR`, all
  exactly 12 characters, which looked like a 12-character truncation in our code. The corpus-wide
  declared-name length histogram over 2,689 names is smooth with **no spike at 12**, so the truncation
  is Avalon's PDF text layer (the cell reads `readdatavali d readdatavali d _ n`), not SpecForge.
  **Also falsified:** 12 of Avalon's 26 declarations looked like section-heading rows
  (`FUNDAMENTAL`, `PIPELINE`, `BURST`), suggesting a systemic heading-as-signal bug; measured
  corpus-wide it is **6 names across 2 documents (0.2%)** — real, Avalon-concentrated, and far too
  small to be this tree's subject.
  Non-goal: any code change; any notation work.
  Prerequisite: none.
  Verification: read-only over all 78 persisted `source_ir.json` + `evidence_ir.json` pairs; no file
  written, no artifact mutated.
  Commit: see log.

- ID: `SIGNAL-DECLARATION-ROW-DROP.1` · Status: `pending` · Goal: **make the drop visible, before making it
  smaller.** Give the reader a per-table accounting — rows considered, emitted, dropped, with the
  dropped rows' name cells retained — persisted where a check can read it. The order matters: a silent
  loss that gets quietly smaller is still a silent loss, and every notation added in `.2` needs this
  denominator to be measured against.
  Non-goal: reading any new notation; that is `.2`.
  Prerequisite: `.0`.
  Verification: pending
  Commit: pending

- ID: `SIGNAL-DECLARATION-ROW-DROP.2` · Status: `pending` · Goal: **read the two notations the census
  names**, as grammars and not as vendor forms — directional arrow (`A → B`, and its ASCII spellings)
  and enumerated legal widths (`8, 16, 32, …` → a width set, not a parse failure). Each ships with the
  corpus-wide count of rows it newly admits and an adjudicated sample, per the Acceptance Criteria.
  **Prediction, stated before implementation:** the arrow form recovers Avalon's eight signals *with
  directions* and moves the four all-zero documents off zero; it does **not** account for the majority
  of the 482, because Cortex-A76's 107 and AXI's 103 are unlikely to share Avalon's notation. If the
  arrow form alone closes more than half the 482, this prediction is wrong and the reason gets recorded.
  Non-goal: entity typing; anything an LLM decides.
  Prerequisite: `.1`.
  Verification: pending
  Commit: pending

- ID: `SIGNAL-DECLARATION-ROW-DROP.3` · Status: `pending` · Goal: **the declared spelling must be the
  document's spelling.** `known_signals` carries `READDATA`; Avalon writes `readdata` 75 times and
  `READDATA` zero times. Under ADR 0037 case carries no alias authority, so emitting a case-variant the
  source never wrote is minting an identifier rather than grounding one. Find where the uppercase
  spelling enters, and ground it. Census first: how many declared names corpus-wide are spelled in a
  case the source document never uses.
  Non-goal: normalising case for comparison — matching case-insensitively is correct and stays; this is
  about the spelling that gets *emitted*.
  Prerequisite: none (independent of `.1`/`.2`).
  Verification: pending
  Commit: pending
