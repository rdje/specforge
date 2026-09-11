# SIGNAL-DECLARATION-ROW-DROP: the authoritative declaration reader silently discards 18.3% of the rows it was given

## Metadata

- Tree ID: `SIGNAL-DECLARATION-ROW-DROP`
- Status: `active` (`2026-09-11`; `.0` census + `.1` instrument closed, `.1a` correction closed, `.2`/`.3` open)
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

- ID: `SIGNAL-DECLARATION-ROW-DROP` · Status: `active` (`2026-09-11`) · Children: `.0`, `.1`, `.1a`, `.2`, `.3`

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

- ID: `SIGNAL-DECLARATION-ROW-DROP.1` · Status: `done` (`2026-09-11`) · Goal: **make the drop visible,
  before making it smaller.** Give the reader a per-table accounting — rows considered, emitted,
  dropped, with the dropped rows' name cells retained — persisted where a check can read it. The order
  matters: a silent loss that gets quietly smaller is still a silent loss, and every notation added in
  `.2` needs this denominator to be measured against.

  **Implemented on the extraction manifest, and the reason is load-bearing.** The build records one
  `TableDeclarationRowAccounting { table_id, rows_considered, declarations_emitted, dropped_rows }` per
  table the body-row reader was handed, and one `DroppedDeclarationRow { name_cell, reason }` for every
  row that produced nothing, published as `extraction_manifest.declaration_row_accounting`. The reason
  vocabulary is `no_name_cell` / `name_not_an_identifier` / `no_direction_and_no_width` — each a
  property of the row's own shape, so it is grammar and names no document, vendor or protocol
  (ADR 0006). The name cell is retained **verbatim**, not as the derived token, so a drop can be
  adjudicated from the artifact alone without re-reading the source PDF. A table is accounted for
  whenever it offered a row, including tables that emitted everything they were given: an accounting
  kept only for lossy tables would make the ratio unreadable, because its own denominator would depend
  on the loss.
  **The first implementation put it on `EvidenceIr` as a registered evidence field, and that was
  wrong twice over.** Architecturally, a counter describing what the reader was handed is producer
  telemetry, not extracted evidence — the extraction manifest is precisely the "which extractors fired
  and what each contributed" surface, and the accounting belongs there. Mechanically, a **new
  registered rule field restamps the EvidenceIR stage ruleset digest and un-seals every persisted
  artifact at and below the stage**, and the re-seal cannot complete: `rebuild_stage_cascade.sh --write`
  rebuilt 24 of 27 and failed on AXI, APB and AHB because their `normalized/` bundles do not exist, and
  the retained-bundle population is frozen at exactly 24 by three independent doctrine mechanisms
  (`[[retained-bundle-population-is-frozen]]`). Measured both ways: with the registered field,
  `check_proof_seal_currency.sh` refuses the persisted seal at evidence, semantic and intent; with the
  same data on the manifest it reports *"the persisted corpus carries seals the current build
  accepts"*. Adding an evidence rule field is therefore not merely expensive right now — **it is
  structurally unlandable**, and that is worth knowing independently of this leaf.
  **Scope, stated precisely.** This instruments the body-row path (`synthesize_signal_declarations`).
  The additive trapped-row recovery path is a different population and is deliberately not counted.
  **The two denominators are not the same number, and must not be compared naively.** `.0` counted
  `signal_description` body rows whose name cell is a single identifier, read from persisted SourceIR —
  482 of 2,637 (18.3%). The runtime accounting counts **every** body row of every table this reader is
  handed, so its denominator is wider by construction. A per-document rate from one is not comparable
  to the corpus rate from the other; `.2` must quote the runtime number on both sides of its change.
  **Named residual (CI-tier, before push).** The persisted corpus still carries pre-slice content: the
  new manifest field is absent from the 27 stored artifacts, so `CHAIN-CURRENCY` will report ADIv6's
  EvidenceIR as content-changed. The delta is **attributed here** — `declaration_row_accounting`
  appears on the extraction manifest, nothing else moves — which is exactly the attribution ADR 0025
  decision 1 requires before the cascade may be re-run. Gate-tier is green; the rebuild is owned by
  this leaf and must run before the next push.
  Non-goal: reading any new notation; that is `.2`. Non-goal: failing a build on the ratio — `.1`
  publishes the denominator, and what bound to enforce is a decision that needs `.2`'s recovery first.
  Prerequisite: `.0`.
  Verification: see the Acceptance Checklist below.
  Commit: see log.

- ID: `SIGNAL-DECLARATION-ROW-DROP.1a` · Status: `done` (`2026-09-11`) · Goal: **repair the toolchain
  gate `.1` left red, and withdraw the claim that said otherwise.** `.1` ticked NO REGRESSION citing
  `cargo clippy --offline --all-targets -- -D warnings` clean. It is not: at `48def695` clippy fails
  with `field_reassign_with_default` on `crates/specforge/src/ir/evidence.rs:1905`, the line `.1`
  itself added (`extraction_manifest.declaration_row_accounting = …` after
  `ExtractionManifest::default()`), and `scripts/run_ci.sh:30` runs that exact command with
  `-D warnings`, so the branch could not have passed CI.

  **Why nothing caught it, which is the part worth keeping.** `scripts/check_doctrines.sh` runs no
  cargo gate at all — its registry is fourteen repository doctrines, and the Rust toolchain oracles
  (`fmt`, `clippy`, `test`) are named in its header as DETERMINISTIC-ORACLE doctrines that live in
  `scripts/run_ci.sh`, not in the driver. So the full doctrine report reads all-PASS over a tree whose
  build does not lint, and the pre-commit hook cannot see it either. The only thing standing between a
  red clippy and a merge is that someone actually runs it — which is exactly the class of check
  `CLAIM_VERIFICATION.md` §2 calls "a ticked acceptance box: catches a forgotten step, still permits
  the step being done wrong." Fact card `[[doctrine-driver-runs-no-cargo-gate]]`.
  Non-goal: adding a cargo gate to the doctrine driver — the tiering is deliberate (a pre-commit hook
  must stay fast) and changing it is `DOCTRINE_ENFORCEMENT.md`'s decision, not this tree's.
  Prerequisite: `.1`.
  Verification: see the `.1a` checklist below.
  Commit: see log.

## Acceptance Checklist (enforced)
- [x] **REPRODUCE / MEASURE** — `.0`'s corpus census: **482 of 2,637 signal-description rows (18.3%)**
  discarded with no declaration, no residual, no counter and no validation-report entry, over all 78
  persisted `source_ir.json` + `evidence_ir.json` pairs. Four documents lose every row; AXI loses 103
  across two editions while scoring `1.000` on every aspect.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `synthesize_signal_declarations`: the `(direction, width)` match ended `_ => continue, // No direction
  AND no width — not enough info to synthesize`. Nothing downstream of that arm records that a row
  existed, so the reader reported no denominator and no check could bound the ratio. Confirmed on
  Avalon `table_0012` (typed `signal_description`, first column `Signal Role`, rows `readdata` and
  `writedata`): direction is the arrow form `Slave → Master`, which
  `infer_signal_direction_from_actor_text` does not read, and `readdata`'s width is the enumerated set
  `8, 16, 32, 64, 128, 256, 512, 1024`, which `infer_signal_table_row_width_hint` does not read — so
  both rows hit the arm. Corroborated by 15 of Avalon's 26 surviving declarations being width-only.
- [x] **ADDRESSED (verified)** — every row the reader examines is now accounted for. Control
  `a_row_the_reader_cannot_interpret_is_counted_rather_than_discarded` pins the invariant
  `rows_considered == declarations_emitted + dropped_rows.len()` and asserts both drop reasons and the
  retained name cell. Measured on real documents: all **24** chain-current specifications re-run
  through the instrumented reader (`specforge evidence --dry-run`), and **0 of 24 produce a declaration
  without also producing an accounting record** — the instrument has no blind document. ADIv6
  (`ihi0074_a`), the one wire-bearing specification in that set, reports **4 tables, 24 rows
  considered, 3 declarations emitted, 21 dropped** — 20 `no_direction_and_no_width`, 1
  `name_not_an_identifier` — with the invariant `rows == emitted + dropped` holding. The retained name
  cells name **real wires**: `CDBGPWRUPREQ`, `CDBGPWRUPACK`, `CSYSPWRUPREQ`, `DBGTDO`, `DBGTRSTn`. That
  87.5% loss in a single document was previously invisible in every artifact, score and gate.
- [x] **NO REGRESSION** — `cargo test --offline -p specforge-core --lib` **1406 passed, 0 failed**
  (1405 pre-existing + the new control); `cargo test --offline -p specforge --lib` 472 passed;
  `cargo fmt --all --check` clean. **Correction (`2026-09-11`, `.1a`): the clippy leg of this line did
  not re-derive.** `cargo clippy --offline --all-targets -- -D warnings` fails at `48def695` on the
  very line this leaf added, and CI runs exactly that command (`scripts/run_ci.sh:30`). The rest of
  the line stands; the clippy leg is withdrawn and repaired by `.1a`. The
  change is purely additive — no declaration is emitted or withheld that was not before, so no score
  can move. **No registration count moves**: `EVIDENCE_RULE_FIELDS` stays 39 and the genericity rule
  inventory stays 170, because the accounting is manifest telemetry rather than a new evidence rule —
  which is also why `check_proof_seal_currency.sh` reports the persisted corpus still sealed.
- [x] **GENERICITY (ADR 0006)** — the reason vocabulary is three properties of a row's own shape; no
  chip, vendor, or protocol name appears in the rule or the enum. The retained `name_cell` is document
  text carried as provenance, not as a rule input. Registered under the existing
  `evidence.declaration` claim family.
- [x] **LOCKSTEP** — book updated in `.0`'s commit (`pipeline/evidenceir.md` gains the missing
  failure-mode section, `quality/extraction-eval.md` gains the recall caveat); fact card
  `[[declaration-reader-drops-uninterpretable-rows]]`. **Producer sub-clause: no production rule was
  deleted or replaced** — the `_ => continue` arm still drops the same rows, it now records them — so
  no book text described behaviour that has gone away.

## Acceptance Checklist — `.1a` (enforced)
- [x] **REPRODUCE / MEASURE** — `git stash push --include-untracked` to reach `48def695` exactly, then
  `cargo clippy --offline --all-targets -- -D warnings`: `error: could not compile specforge-core (lib)`
  / `error: could not compile specforge-core (lib test)`, one violation. The same command is
  `scripts/run_ci.sh:30`. `bash scripts/check_doctrines.sh` over the same tree reports every doctrine
  PASS, so the red build is invisible to the enforcer.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs:1905` (at `48def695`):
  `let mut extraction_manifest = ExtractionManifest::default();` immediately followed by
  `extraction_manifest.declaration_row_accounting = table_declaration_row_accounting;` is
  `clippy::field_reassign_with_default`, denied by `-D warnings`. Introduced by `.1` —
  `git show 48def695 -- crates/specforge/src/ir/evidence.rs` shows the assignment as an added line
  under an unchanged `::default()`. The gap that let it land: `scripts/check_doctrines.sh` registers
  fourteen doctrines and no cargo invocation, so the clippy leg of a NO-REGRESSION box is attested by
  the author and by nothing else until `run_ci.sh` runs.
- [x] **ADDRESSED (verified)** — the manifest is built in one initializer
  (`declaration_row_accounting: …, ..ExtractionManifest::default()`), preserving the comment that says
  why the field is there. `cargo clippy --offline --all-targets -- -D warnings` now exits `0`.
- [x] **NO REGRESSION** — `cargo test --offline -p specforge-core --lib` **1406 passed, 0 failed** and
  `cargo test --offline -p specforge --lib` **472 passed** — identical to `.1`'s counts, as a
  construction-syntax change must be; `cargo fmt --all --check` clean; `bash scripts/check_doctrines.sh`
  all gate-tier PASS. No behaviour, no field, no registration count moves, so no gold can move.
- [x] **GENERICITY (ADR 0006)** — syntax only; no rule, vocabulary, or document text is touched.
- [x] **LOCKSTEP** — no user-visible behaviour changed, so the book is unchanged by the producer
  sub-clause; the durable finding is the fact card `[[doctrine-driver-runs-no-cargo-gate]]`, and `.1`'s
  withdrawn clippy leg is corrected in place above rather than quietly replaced.

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
