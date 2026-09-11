# SIGNAL-DECLARATION-ROW-DROP: the authoritative declaration reader silently discards 18.3% of the rows it was given

## Metadata

- Tree ID: `SIGNAL-DECLARATION-ROW-DROP`
- Status: `active` (`2026-09-11`; `.0`/`.1`/`.1a`/`.1b`/`.2a`/`.2b` closed; `.2` split into `.2a`-`.2d`; `.1c`/`.2c`/`.2d`/`.3` open)
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

- ID: `SIGNAL-DECLARATION-ROW-DROP` · Status: `active` (`2026-09-11`) · Children: `.0`, `.1`, `.1a`, `.1b`, `.1c`, `.2`, `.3`

- ID: `SIGNAL-DECLARATION-ROW-DROP.2` · Status: `active` (`2026-09-11`) · Children: `.2a`, `.2b`, `.2c`, `.2d`
  · Goal: unchanged — read the notations the census names, as grammars. **Split before implementation**
  after the corpus population was measured: the two notations are independent changes with different
  payoffs (the arrow recovers rows; the enumerated width only sharpens rows the arrow already
  recovered), adjudicating the arrow's own selection turned up a precondition that had to land first,
  and the 49 rows that fail closed name a taxonomy question with a much wider blast radius than either
  notation. Each child ships its own corpus count and adjudicated sample.

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
  **Correction (`2026-09-11`, `.1b`): "gate-tier is green" understated this residual badly.** Until the
  rebuild ran, 4 of the 27 proof-carrying documents — AXI, APB, AHB and ADIv6, i.e. every wire-bearing
  one — could not be loaded by the current binary at all, so `eval-extraction` refused every gold and
  the WIRE-BASED-100 hard gate was unmeasurable. The deferral was not "a stale seal to tidy up before
  pushing"; it was the project's primary oracle off the air. Executed and re-derived in `.1b`.
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

- ID: `SIGNAL-DECLARATION-ROW-DROP.1b` · Status: `done` (`2026-09-11`) · Goal: **execute the cascade `.1`
  deferred, and correct what its "gate-tier is green" meant.** `.1` named the residual honestly — *"the
  persisted corpus still carries pre-slice content … the rebuild is owned by this leaf and must run
  before the next push"* — and then reported the tree as green. It was not green in the sense a reader
  would take: at `f2966d42` the product's own read-only canonical probe
  (`specforge semantic <persisted evidence_ir.json> --dry-run`) was **refused for 4 of the 27**
  proof-carrying documents, and the four were exactly the wire-bearing ones — AXI `ihi0022_l`, APB
  `ihi0024_e`, AHB `ihi0033_c`, ADIv6 `ihi0074_a`. `eval-extraction`, the WIRE-BASED-100 scoring
  oracle, therefore refused **every gold**, and had been doing so since `48def695`.

  **Root cause — a second staling mechanism, not the one `.1` guarded against.** `extraction_manifest`
  is one of the 39 `EVIDENCE_RULE_FIELDS`, so it is a proof-bearing claim field. `.1` put the
  accounting *inside* it precisely to avoid adding a 40th field and restamping the stage ruleset
  digest, and at that it succeeded: the seal is unchanged and the count is still 39. But the proof
  binds more than the ruleset. `public_field_values()` feeds `replay_bytes`; `replay_bytes` is the
  output of the registered derivation `evidence.current-replay`; and that derivation is the sole input
  of every per-claim derivation `evidence.claim.<field>.root`. **Changing the content of an existing
  rule field moves `inputs_sha256` for every claim in the artifact**, so every persisted proof goes
  stale — a different route to the same place, invisible to a seal comparison. It bit only four
  documents because the accounting is empty for a document whose tables never reach the body-row
  reader, and an absent field deserializes to that same empty value.

  **The repair.** The three wire golds' normalized bundles were restored from
  `generated/preserved/WIRE-BASED-100.10/{apb,ahb,axi}-normalized-bundle-held-out/`, the four chains
  rebuilt stage-major with one `validate` per artifact strictly upstream-to-downstream, and the
  bundles returned — `diff -r` clean against the preserved copies before removal, declared population
  still exactly 24 (`[[retained-bundle-population-is-frozen]]`). The cascade **remedy** script could
  not be used: it refuses any content delta by design (ADR 0025 decision 1) and this delta is real and
  deliberate, so the stages were run directly with the delta attributed here.
  Non-goal: teaching `rebuild_stage_cascade.sh` to absorb an attributed delta — that weakens the one
  control that makes a remedy safe, and it needs a decision record, not a leaf.
  Prerequisite: `.1`.
  Verification: see the `.1b` checklist below.
  Commit: see log.

- ID: `SIGNAL-DECLARATION-ROW-DROP.1c` · Status: `pending` · Goal: **make the canonical probe
  per-document, because "one probe per distinct seal" is a sample of the property that actually
  broke.** `scripts/check_proof_seal_currency.sh` states its own argument in its header: the probe is
  *"REPRESENTATIVE, one per distinct seal per stage. That is not a sample: the census is what
  establishes representativeness."* `.1b` falsifies that argument by measurement. All 27 evidence
  artifacts carried **one** distinct seal, so exactly one probe ran — and 23 of the 27 accepted while
  4 refused. The seal is homogeneous *because it does not depend on artifact content*; the loader also
  verifies a per-document replay topology, which does. One probe per seal can therefore never see a
  content-driven divergence, and it is `CLAIM_VERIFICATION.md` §2's fourth row exactly: a per-item
  assertion checked against per-container data, which reproduces perfectly while getting it wrong.
  Decide the tier honestly: measure a full per-document sweep first, and if it costs more than the
  gate tier can carry, move the total probe to CI and leave a cheap prefilter at the gate — but never
  leave a sampled probe *describing itself* as total.
  Prerequisite: `.1b`.
  Verification: pending
  Commit: pending

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

- ID: `SIGNAL-DECLARATION-ROW-DROP.2a` · Status: `done` (`2026-09-11`) · Goal: **a template row is not
  a declaration.** Adjudicating `.2b`'s selection before implementing it — the discipline this tree
  exists to enforce — turned up a precondition. Of the 7 currently-dropped rows the flow-arrow grammar
  would newly admit, **2 are template metavariables**: Avalon `table_0031` documents a tristate conduit
  as `<name> _in` / `<name> _out` / `<name> _outen`, *"the input signal of a logical tristate signal"*.
  The reader declares them, because `signal_names_in_name_cell` trims the leading token of its
  non-identifier characters *before* judging it, and that trim is exactly what turns `<name>` into the
  ordinary identifier `name`. Shipping `.2b` first would have minted that phantom **twice with opposite
  senses**, one per template row — a 28.6% false-positive rate in its own newly-admitted set.
  **The rule:** a leading name token wrapped in a matched bracket pair (`<>`, `()`, `[]`, `{}`) around a
  non-empty body is a metavariable; the row declares nothing and is recorded with its own reason
  `name_is_a_placeholder`, name cell verbatim. Shape only — the delimiters decide, never the word
  between them, so there is no placeholder vocabulary to maintain (ADR 0006).
  **Ordering is load-bearing and deliberate:** the metavariable test runs AFTER the identifier test, so
  a bracketed token that was never an identifier (`[15:8]` under a `Bits` header, GIC-600's
  `[<domain>_]mbistaddr[variable:0]`) keeps the reason it already had. Without that ordering the change
  would relabel 7 rows corpus-wide whose fate does not move, churning the accounting for nothing.
  Non-goal: changing `is_hardware_signal_token` or `signal_names_in_name_cell` — both are used well
  outside this reader and the trim itself is not the defect; judging the trimmed token was.
  Prerequisite: `.1`.
  Verification: see the `.2a` checklist below.
  Commit: see log.

## Acceptance Checklist — `.1b` (enforced)
- [x] **REPRODUCE / MEASURE** — over the 27 proof-carrying documents at the pre-repair corpus,
  `specforge semantic generated/evidence_ir/<doc>/evidence_ir.json --dry-run` (the read-only canonical
  probe, chosen because `validate` mutates): **23 accepted, 4 refused** with
  `EvidenceIR proof verification failed: registered derivation 'evidence.claim.schema_version.root'
  output or input topology is stale`. The four: `ihi0022_l` (AXI), `ihi0024_e` (APB), `ihi0033_c`
  (AHB), `ihi0074_a` (ADIv6). `specforge eval-extraction … --provider skip` consequently refused all
  eight gold datasets. Bisected across three commits by rebuilding each and re-running the oracle:
  `4697fdf0` scores, `71a2b3a0` (`.0`) scores, `48def695` (`.1`) refuses. (The probe ran on a binary
  carrying the unlanded `.2b`; that is sound here because `.2b`'s EvidenceIR output is byte-identical
  to `f2966d42`'s for all 27 documents — measured — and the probe compares the persisted artifact
  against exactly that replay.)
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: `extraction_manifest` is
  one of the 39 `EVIDENCE_RULE_FIELDS`, so `public_field_values()` (`evidence.rs:1249`) puts it in
  `replay_bytes`; `replay_bytes` is the output of the registered derivation `evidence.current-replay`
  (`evidence.rs:1441`), which is the sole input of every `evidence.claim.<field>.root`
  (`evidence.rs:1454`); and `verify_premise` (`crates/specforge/src/ir/derivation.rs:2149`) rejects a
  premise whose recorded `output_sha256`/`inputs_sha256` no longer match the current replay. So adding
  a sub-field to an existing rule field stales every claim in the artifact **without touching the
  ruleset seal** — a second staling mechanism beside the one `.1` guarded against. Empty for a
  document whose tables never reach the body-row reader, which is why 23 of 27 were unaffected.
- [x] **ADDRESSED (verified)** — three held-out bundles restored from
  `generated/preserved/WIRE-BASED-100.10/`, four chains rebuilt stage-major
  (`evidence`→`validate`→`semantic`→`validate`→`intent`→`validate`→`adapt --target isf`→`validate`,
  one validate per artifact, strictly upstream-to-downstream), bundles returned with `diff -r` clean
  and the declared population still 24. Measured against a pre-rebuild snapshot held at
  `generated/preserved/SIGNAL-DECLARATION-ROW-DROP.1b/pre-rebuild/`: across all four documents the
  **only** differing top-level section is `extraction_manifest` at the EvidenceIR stage, and within it
  only `declaration_row_accounting`; `semantic_ir`, `intent_ir` and the `.isf` adapter are
  content-identical, and the emitted `.isf` is unchanged under `compare_emitted_isf` — the cascade's
  own predicate, borrowed so the repair cannot certify itself with a weaker comparison than the gate
  would make. Exactly the attribution ADR 0025 decision 1 requires. After the rebuild the probe
  accepts **27 of 27**. The recovered denominators: APB 9 tables / 61 rows / 60 emitted / 1 dropped;
  AHB 12 / 95 / 79 / 16; AXI 76 / 464 / 468 / 33; ADIv6 4 / 24 / 3 / 21.
- [x] **NO REGRESSION** — the wire golds are **re-scored, not assumed**, and every published value
  re-derives. `eval-extraction --provider skip`, WIRE-BASED-100 block: APB `signal_constraint` 1.000
  (tp=6 fp=0 fn=0) / `actor_signal_relation` 1.000 (tp=5 fp=0 fn=0) / `temporal_rule` 1.000 (tp=3);
  AHB 1.000 (tp=6) / 1.000 (tp=6) / 1.000 (tp=4); AXI 1.000 (tp=3) / 1.000 (tp=6) / 1.000 (tp=3); SWD
  `signal_constraint` 1.000 (tp=1) / `actor_signal_relation` 1.000 (tp=1), and its derivation gold
  `protocol_operation` 1.000 (tp=4), `protocol_state` tp=8 fn=5, `interface_edge_timing` 1.000 (tp=1),
  `serial_frame_field` 0 of 11 — which is the published **SWD 13/29** exactly (0+4+8+1 of 11+4+13+1).
  `kg-bench` **156/156**. `bash scripts/check_chain_currency.sh` over the rebuilt corpus: *"every
  measurable persisted artifact is exactly what the current binary produces"* — evidence 24 replayed /
  24 current / 0 stale, semantic 27/27/0, intent 27/27/0, isf-adapter 27/27/0, retention **24 bundles
  on disk, exactly the declared retained set**. The three wire golds stay `UNMEASURABLE` at the
  evidence stage by construction, because their bundles were returned.
  `cargo test --offline -p specforge-core --lib` 1410 passed / `-p specforge --lib` 472 passed;
  `cargo fmt --all --check` clean; `cargo clippy --offline --all-targets -- -D warnings` exit 0;
  `bash scripts/check_doctrines.sh` all gate-tier PASS.
- [x] **GENERICITY (ADR 0006)** — no rule, vocabulary, or production decision changed; this leaf runs
  the product's own stages over its own corpus. The four rebuilt documents are named as corpus
  members, not as inputs to any rule.
- [x] **LOCKSTEP** — no user-visible behaviour changed, so the book is unchanged by the producer
  sub-clause; the durable findings are the fact card
  `[[evidence-rule-field-content-stales-every-proof]]` and `.1`'s residual note,
  corrected in place above rather than quietly replaced. The blind spot that hid it is opened as
  `.1c` rather than left as a remark.


## Acceptance Checklist — `.2a` (enforced)
- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_declaration_row_notations.py` over all 78
  persisted `source_ir.json`: **12** name cells in `signal_description` tables whose leading token is
  wrapped in a matched bracket pair, of which **5 currently survive the identifier test** and can
  therefore reach a declaration — `<name> _in`, `<name> _out`, `<name> _outen` (Avalon `table_0031`),
  `<any>` (`table_0030`), `(varies)` (AMD IOMMU `table_0314`). The other 7 (GIC-600's
  `[<domain>_]mbistaddr[variable:0]…`, CHI's `[15:8]`/`[7:2]`/`[1:0]`) already fail the identifier test.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `synthesize_signal_declarations`: `signal_names_in_name_cell` takes the first whitespace token and
  `trim_matches`-es every non-identifier character off both ends, so `<name>` reaches
  `is_hardware_signal_token` as `name` and passes. The defect is not the trim — it is judging the
  trimmed token. Confirmed by asserting both halves in the control:
  `signal_names_in_name_cell("<name> _outen")[0] == "name"` and `is_hardware_signal_token("name")`.
- [x] **ADDRESSED (verified)** — measured before→after through the product's own reader, by running
  the corpus fixtures with the new guard removed and then restored, rather than predicting the verdict.
  **Before:** Avalon `table_0031` emits `statement_0001` declaring signal **`name`** (2 further rows
  dropped `no_direction_and_no_width`); `table_0030` emits `statement_0001` declaring signal **`any`**,
  0 drops. **After:** both tables emit nothing; 3 and 1 rows respectively recorded
  `name_is_a_placeholder`, with `rows_considered == declarations_emitted + dropped_rows.len()` still
  closing. **Two phantom declarations removed.** The guard is observed RED: with it removed, both
  `the_corpus_template_rows_declare_no_signal` and
  `a_bracketed_metavariable_name_cell_declares_no_signal` fail, the latter listing `basename` five
  times — one per bracket pair the grammar recognises.
- [x] **NO REGRESSION** — **byte-identical `EvidenceIR` for every rebuildable document**: all 24
  proof-carrying specifications re-run through `specforge evidence --dry-run` before and after, and
  `cmp` reports **0 of 24 changed**. No gold can move because no artifact moves. (The 3 remaining
  proof-carrying documents — AXI, APB, AHB — cannot be re-run at all: their `normalized/` bundles do
  not exist and the population is frozen, `[[retained-bundle-population-is-frozen]]`.) The two
  documents this leaf does change are legacy proofless, so no persisted chain and no seal is touched.
  `cargo test --offline -p specforge-core --lib` **1410 passed, 0 failed** (1406 + 4 new controls);
  `cargo test --offline -p specforge --lib` 472 passed; `cargo fmt --all --check` clean;
  `cargo clippy --offline --all-targets -- -D warnings` exit 0 (run in this session, per `.1a`);
  `bash scripts/check_doctrines.sh` all gate-tier PASS.
- [x] **GENERICITY (ADR 0006)** — the rule is four bracket pairs and a non-empty body; no word, vendor,
  protocol, or document name enters it, and there is no placeholder vocabulary to maintain. The
  corpus-verbatim rows live only in a test fixture, which is where document-specific text is allowed.
  The census script replicates the taxonomy for classification only and says so in its own header —
  it is not, and may not be cited as, a check on the Rust rule (`CLAIM_VERIFICATION.md` §2).
- [x] **LOCKSTEP** — `docs/book/src/pipeline/evidenceir.md` gains *"A template row is not a
  declaration"*, continuing `.0`'s failure-mode section; its two dated counts are registered in
  `book_quantitative_claims.jsonl` as `excluded / dated_boundary_evidence`, and the frozen candidate
  expectation moves 335 → 337 to cover them rather than leaving them ungoverned. **Producer
  sub-clause: no production rule was deleted or replaced** — the rule is additive and every existing
  drop reason still means what the book says it means.


- ID: `SIGNAL-DECLARATION-ROW-DROP.2b` · Status: `done` (`2026-09-11`) · Goal: **read the flow-arrow direction
  grammar** — a direction-bearing cell that states the signal's flow (`<driver> → <receiver>`) rather
  than its port sense. Both sides must resolve through the existing actor-role taxonomy and must
  **agree** (left read as a source, right read as a destination, same port sense), and exactly one
  right-flow arrow must be present; a reverse or bidirectional marker, two flows in one cell, or one
  unresolved side fails closed.
  **Population already measured** (`python3 scripts/measure_declaration_row_notations.py`, 78 persisted
  SourceIR): **83** arrow cells in direction-bearing columns of `signal_description` tables, in exactly
  2 documents and 13 distinct cell forms — every one enumerable, so the sample is the population.
  **18 admit** (11 `Master → Slave` → `output`, 7 `Slave → Master` → `input`), **65 fail closed** — 16
  two-flow cells (`ITS →Distributor Distributor →ITS`, genuinely bidirectional groups) and 49 whose
  actor names are outside the builtin taxonomy. Of the 18, **7 are rows that produce nothing today**
  and 11 currently emit a width-only declaration that would gain a direction. After `.2a`, 5 of those 7
  are real wires (`address`, `byteenable`, `readdata`, `writedata`, `burstcount`) and 2 are the
  metavariable rows `.2a` now refuses.
  **The `.2` prediction, graded before implementation — half of it is already falsified.** It predicted
  the arrow form "moves the four all-zero documents off zero". It does not: AMD IOMMU, HBM2, ATP and
  CHI C2C contain **no arrow cell at all** in any signal-description table. It also predicted recovery
  of "Avalon's eight signals"; the admitted set covers **5 of the 8** — `CHANNEL`, `DATA` and `ERROR`
  are not among them, so `WIRE-BASED-100.10f` must re-measure rather than assume its unblock. The
  surviving half of the prediction holds and then some: 18 of 482 is **3.7%**, nowhere near a majority.
  Non-goal: extending the actor taxonomy (that is `.2d`); entity typing; anything an LLM decides.
  Non-goal: reading a LEFTWARD arrow as a flow. It is the same relation with its operands swapped, but
  no corpus direction cell uses one and the single family that writes `←` writes it as *assignment*
  (`ATVALID ← 0`), so reading it as flow would ship a rule with no population behind it. Disqualified
  explicitly rather than left to fall through.
  Prerequisite: `.2a`.
  Verification: see the `.2b` checklist below.
  Commit: see log.

- ID: `SIGNAL-DECLARATION-ROW-DROP.2c` · Status: `pending` · Goal: **read an enumerated legal-width
  cell** (`8, 16, 32, 64, 128, 256, 512, 1024`) as a width *set* rather than a parse failure.
  Population: **7** cells in 2 documents (`8, 16, 32, 64, 128, 256, 512, 1024` ×2, `2, 4, 8, 16, 32,
  64, 128`, `4, 8` ×2, `1, 4, 8`, `1,4, 8`). This leaf carries an unresolved policy choice and that is
  why it is separate: `WidthHint` has `Numeric(u32)` and `Parametric(String)`, and a set is neither.
  Picking a member fabricates; carrying the verbatim text puts commas into a declaration sentence that
  SemanticIR parses. Decide the representation first, in the leaf, before writing code.
  Note the payoff is narrow: after `.2b` every one of these rows already has a direction, so `.2c` adds
  width fidelity, not recall. Non-goal: recovering rows — that is `.2b`'s.
  Prerequisite: `.2b`.
  Verification: pending
  Commit: pending

- ID: `SIGNAL-DECLARATION-ROW-DROP.2d` · Status: `pending` · Goal: **decide what to do about the 49
  arrow rows whose actors the taxonomy does not know.** `builtin_actor_taxonomy_role_in_text` knows
  four requester-like and six completer-like terms. Every GIC-600 arrow row names an actor outside that
  set (`Distributor`, `Redistributor`, `ITS`, `SPI Collator`, `Wake Request`, `Remote chip`), as do 9
  Avalon rows (`Source → Sink`, `Interconnect → Slave`). These are real flows the reader can see and
  cannot close. Extending the taxonomy is NOT a local change: that function also decides section-heading
  direction and prose relation direction, so any new term moves populations this tree has not measured.
  Census the blast radius of each candidate term corpus-wide before adding any, or decide that a flow
  between two document-named actors can be read relative to the table's own subject without a taxonomy
  at all. Non-goal: adding terms because they look obvious.
  Prerequisite: `.2b`.
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

## Acceptance Checklist — `.2b` (enforced)
- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_declaration_row_notations.py` over 78 persisted
  SourceIR: **83** arrow cells in direction-bearing columns of `signal_description` tables, 2 documents,
  13 distinct forms. Through the reader, on Avalon `table_0012` carried verbatim as a fixture, the
  before state is **1 declaration and 2 dropped rows**: `Signal debugaccess is width 1.` plus
  `readdata` and `writedata` recorded `no_direction_and_no_width` — measured by running the control
  with the new chain step removed, not predicted.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `synthesize_signal_declarations`: the direction chain read an explicit column only for the literal
  substrings `output`/`input`, then a source/destination column through
  `infer_signal_direction_from_actor_text`, whose `builtin_actor_taxonomy_role_in_text` returns `None`
  for `Master → Slave` because `normalize_actor_term` turns the arrow into whitespace and the cell then
  contains a requester term **and** a completer term. So a cell that states the flow perfectly clearly
  resolved to no direction, and with the width also unreadable the row hit `_ => continue`.
- [x] **ADDRESSED (verified)** — after: the same fixture emits **3 declarations and drops nothing** —
  `Signal readdata is input.`, `Signal writedata is output.`, `Signal debugaccess is output width 1.`
  The 18-cell admitted set is adjudicated in full and is **100% genuine direction statements**; the 65
  closed cells are adjudicated too (16 genuinely bidirectional, 49 a taxonomy gap owned by `.2d`).
  Controls: `the_corpus_flow_arrow_forms_admit_only_the_mirrored_ones` enumerates **all 13 corpus cell
  forms**, so the control population is the corpus population;
  `a_flow_arrow_needs_both_sides_to_resolve_and_agree` pins the mirror (one side unknown, and two sides
  of the same role, both fail closed); `a_flow_arrow_must_be_single_forward_and_unambiguous` pins every
  accepted spelling, every disqualifier, and the splitter's one-arrow rule.
- [x] **NO REGRESSION** — **the wire golds are re-scored, not assumed**, against the corpus `.1b`
  repaired: APB `signal_constraint` 1.000 (tp=6 fp=0 fn=0) / `actor_signal_relation` 1.000 (tp=5) /
  `temporal_rule` 1.000 (tp=3); AHB 1.000 (tp=6) / 1.000 (tp=6) / 1.000 (tp=4); AXI 1.000 (tp=3) /
  1.000 (tp=6) / 1.000 (tp=3); SWD 1.000 / 1.000 and its derivation gold unchanged at **13/29**.
  `kg-bench` **156/156**. **Byte-identical EvidenceIR for every proof-carrying document**: the 24
  rebuildable ones compared before/after through `evidence --dry-run` (`cmp`: 0 of 24 changed) and the
  four wire-bearing ones compared with their bundles temporarily restored (4 of 4 identical), so the
  change provably cannot move a score or a stored chain — the two documents it does affect are legacy
  proofless. All 27 artifacts still load. `cargo test --offline -p specforge-core --lib` **1414
  passed, 0 failed** (1410 + 4 new controls); `-p specforge --lib` 472 passed; `cargo fmt --all
  --check` clean; `cargo clippy --offline --all-targets -- -D warnings` exit 0;
  `bash scripts/check_doctrines.sh` all gate-tier PASS. CHAIN-CURRENCY is CI-tier: `.1b` ran it green
  over this exact corpus (*"every measurable persisted artifact is exactly what the current binary
  produces"*), and this leaf's producer output is byte-identical for all 27 proof-carrying documents,
  so the corpus cannot have moved; it is re-running as a confirmatory check and must be green before
  the next push.
- [x] **GENERICITY (ADR 0006)** — the rule is seven arrow spellings, seven disqualifying markers, and
  the role taxonomy that already existed; no document, vendor, or protocol name enters it. The 13
  corpus cell forms appear only in a test fixture. Registered under the existing `evidence.declaration`
  claim family; `EVIDENCE_RULE_FIELDS` stays 39 and the genericity rule inventory stays 170.
- [x] **LOCKSTEP** — `docs/book/src/pipeline/evidenceir.md` gains *"Direction written as a flow, not as
  a port sense"*; its one dated ratio is registered in `book_quantitative_claims.jsonl` as
  `excluded / dated_boundary_evidence` and the frozen candidate expectation moves 337 → 338 rather than
  leaving it ungoverned. Fact card `[[flow-arrow-direction-grammar]]`. **Producer sub-clause: no
  production rule was deleted or replaced** — the chain step is additive and every prior reading keeps
  priority over it.

## Current Frontier

Ordered; PNT selects the first eligible leaf.

1. `SIGNAL-DECLARATION-ROW-DROP.1c` — make the canonical proof probe per-document. First, because a
   sampled probe that calls itself total is how the corpus stayed broken for three commits.
2. `SIGNAL-DECLARATION-ROW-DROP.2c` — the enumerated width set. Decide the representation of a legal
   width set in the leaf before writing code; 7 cells in 2 documents.
3. `SIGNAL-DECLARATION-ROW-DROP.3` — the emitted spelling must be the document's spelling. Independent
   of `.2`; census first.
4. `SIGNAL-DECLARATION-ROW-DROP.2d` — the actor-taxonomy gap behind 49 fail-closed arrow rows. Census
   the blast radius before touching `builtin_actor_taxonomy_role_in_text`.
