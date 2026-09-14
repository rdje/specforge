# SIGNAL-DECLARATION-ROW-DROP: the authoritative declaration reader silently discards 18.3% of the rows it was given

## Metadata

- Tree ID: `SIGNAL-DECLARATION-ROW-DROP`
- Status: `active` (`2026-09-14`; `.0`/`.1`/`.1a`-`.1e`/`.2a`/`.2b`/`.2e`/`.3`/`.4a`/`.4b` closed; `.2c` deferred; `.2d`/`.2f`/`.4c`/`.4d` open)
- Roadmap lane: `R2` (extraction correctness / wire recall)
- Created: `2026-09-11`
- Last updated: `2026-09-14`
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

- ID: `SIGNAL-DECLARATION-ROW-DROP` · Status: `active` (`2026-09-14`) · Children: `.0`, `.1` (`.1a`–`.1e`), `.2` (`.2a`–`.2f`), `.3`, `.4` (`.4a`–`.4d`)

- ID: `SIGNAL-DECLARATION-ROW-DROP.2` · Status: `active` (`2026-09-11`) · Children: `.2a`, `.2b`, `.2c`, `.2d`, `.2e`, `.2f`
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

- ID: `SIGNAL-DECLARATION-ROW-DROP.1c` · Status: `done` (`2026-09-11`) · Goal: **make the canonical probe
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
  **Decided by measurement: the total sweep is 19m02s, so it is CI tier and the gate keeps sampling —
  but the gate now says it samples.** The sampled run is 4 probes; the total run is 108 (27 documents ×
  4 non-terminal stages). Both modes live in one script because the census, the stratum, the seal
  predicate and the loader probe must not fork; `scripts/check_proof_seal_total.sh` exists only because
  the driver's registry names one argument-less executable per row.
  Non-goal: making the gate tier total. A pre-commit hook that costs 19 minutes is a hook that gets
  bypassed, which is a worse outcome than a sample that admits what it is.
  Prerequisite: `.1b`.
  Verification: see the `.1c` checklist below.
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


## Acceptance Checklist — `.1c` (enforced)
- [x] **REPRODUCE / MEASURE** — the gate's own output, before: `source-ir — 27/27 persisted and sealed,
  1 distinct seal(s); 1 accepted, 0 refused`. One probe stood for 27 documents at every stage, and
  `.1b` measured that exact configuration answering 23 accepted / 4 refused. The script's header
  argued this "is not a sample: the census is what establishes representativeness"; the census
  establishes representativeness **of the seal**, which is a digest over the ruleset, and the loader
  also verifies a per-document replay topology the seal cannot express.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_proof_seal_currency.sh`: the probe list was
  `awk '!seen[$2]++'` over the seal census, so exactly one artifact per distinct seal was ever asked.
  `CLAIM_VERIFICATION.md` §2 row 4 names the class — a per-item assertion checked against per-container
  data, which reproduces perfectly while getting it wrong.
- [x] **ADDRESSED (verified)** — `--total` probes every in-scope artifact at every non-terminal stage;
  the gate tier keeps sampling and now **reports that it samples**, with the sample size and the class
  it cannot see. Measured on this corpus: `source-ir 24 of 27 accepted, 0 refused, 3 with no verdict`
  (the three held-out normalized bundles, each named), `evidence/semantic/intent 27 of 27 accepted`,
  exit 0, **19m02s** — which is what puts it at CI tier. **Observed RED, on the exact defect shape:**
  self-test 17 runs a loader that accepts `doc_a` and refuses `doc_c`, its **same-seal** neighbour, and
  asserts the sampled mode MISSES it; self-test 18 asserts `--total` fails and names `doc_c`; self-test
  19 asserts a probe whose own input is absent yields NO VERDICT rather than an acceptance. Without the
  change, 18 cannot pass.
- [x] **NO REGRESSION** — `bash scripts/check_proof_seal_currency.sh --self-test` **19/19 passed**
  (16 pre-existing + 3 new); `./scripts/check_proof_seal_total.sh --self-test` reaches the same 19
  rather than a weaker copy. `bash scripts/check_doctrines.sh` ALL 13 executed PASS (**15 registered**,
  up from 14); `bash scripts/check_project_data_locality.sh` PASS — the wrapper carries the same
  repository-root data contract, which it failed until it did. No Rust source is touched, so no score,
  artifact, or gold can move; `cargo test` 1414 / 472 unchanged from `.2b`.
- [x] **GENERICITY (ADR 0006)** — shell tooling only; no rule, vocabulary, or document text.
- [x] **LOCKSTEP** — `DOCTRINE_ENFORCEMENT.md` §10 gains the `PROOF-SEAL-TOTAL` row and its
  `PROOF-SEAL-CURRENCY` row stops claiming representativeness;
  `docs/book/src/reference/doctrine-enforcement.md` corrects the same claim in prose — it said *"a total
  census, not a sample … representativeness is measured rather than assumed"* — and gains the two-tier
  explanation plus a fourth property for the no-verdict case. Fact card
  `[[evidence-rule-field-content-stales-every-proof]]` updated with the repair. **Producer sub-clause:
  the sampled probe is not deleted**, it is relabelled, so no text describes behaviour that has gone away.


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

- ID: `SIGNAL-DECLARATION-ROW-DROP.2c` · Status: `deferred` (`2026-09-11`) · Goal: **read an enumerated
  legal-width cell** (`8, 16, 32, 64, 128, 256, 512, 1024`) as a width *set* rather than a parse failure.
  **Deferred before implementation, on its own adjudication, and the reason is worth more than the
  leaf.** The census found 7 such cells. Looking at them:

  - **4 of the 7 are not signal widths.** They are the `Bus Width` column of eMMC `table_0020`, a
    bus-mode matrix (`Mode Name | Data Rate | IO Voltage | Bus Width | Frequency | Max Data Transfer`)
    that SourceIR typed `signal_description`; its name column holds mode names. Teaching the reader
    this notation would have declared `Backwards`, `High`, `High` and `HS200` as signals. That table
    already mints `HS400` — its one row whose width cell is a single value and therefore parses — so
    the change would have taken it from one phantom to five. Opened as
    `[[PROSE-NAME-CELL-DECLARATION]]`, which is the real defect here.
  - **The remaining 3 are real** (Avalon `readdata`, `writedata`, `byteenable`) and, after `.2b`, all
    three already declare with a direction. What is missing is width fidelity for three rows in one
    legacy document.
  - **The representation question is not cheap.** `WidthHint` lives in `ir/source.rs` with exactly
    `Numeric(u32)` and `Parametric(String)`; a set is neither, and a new variant crosses SourceIR →
    SemanticIR → IntentIR → `.isf`. Worse, `parse_width_token` (`ir/semantic.rs`) strips a trailing
    comma from a width token, so a sentence reading `width 8, 16, 32` **parses as `Numeric(8)`** — the
    obvious spelling silently fabricates a width. A pipe-joined token (`8|16|32`) parses as nothing,
    which is safe but carries no information.

  **Consequence of deferring, stated:** three Avalon rows keep a direction and no width; the document
  is legacy proofless, so no persisted chain, gold, or score is affected. Reopen once
  `PROSE-NAME-CELL-DECLARATION` has decided the row question, or if a document appears whose
  enumerated widths belong to real signals in a measurable chain — at which point the representation
  decision is a decision record, not a leaf.
  Prerequisite: `.2b`.
  Verification: read-only adjudication of all 7 cells; `jesd84_b50…/evidence_ir.json`
  `table_signal_declaration_provenance` confirms `HS400` is declared from `table_0020`.
  Commit: see log.

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

- ID: `SIGNAL-DECLARATION-ROW-DROP.4` · Status: `active` (opened `2026-09-13` by
  `EXTRACTION-QUALITY-GAUGE.3k.7`; split the same day) · Children: `.4a`, `.4b` (`.4d`), `.4c` · Goal: **the same silent drop one stage later — a declaration the
  SemanticIR reader cannot finish parsing is discarded whole, direction included.**
  This tree's `.0`–`.3` are about `synthesize_signal_declarations_from_table`, the EVIDENCE-stage
  reader. `parse_explicit_signal_declaration` in `crates/specforge/src/ir/semantic.rs` is the reader
  one stage down, and it ends with `if index != tokens.len() { return None; }`: a declaration whose
  WIDTH it cannot consume to the end yields nothing at all, so a direction it has already parsed is
  thrown away with it. There is no residual, no counter and no validation entry — the same silence
  `.0` measured, at a different function.
  **The consequence is not confined to the catalog.** `declared_signal_names` is built from the
  interface records this parser produces, and the semantic grounding filter demotes every
  `signal_constraint` whose subject is not in that set. So a dropped declaration silently deletes
  every obligation the document states about that signal.
  **Measured `2026-09-13` over all 77 documents with both artifacts: 83 signals are declared in
  EvidenceIR and absent from the SemanticIR interface catalog, across 10 documents** — MMU-700 47,
  AXI-H 9, AXI-L 9, GICv3 7, Avalon 5, CHI 2, ATB/CXS/LTI/eMMC 1 each. The population splits in two
  and the strata need different answers:
  * **17 carry a legitimate arithmetic width the parser stops short of** — AXI's
    `Signal WSTRB is output width DATA_WIDTH / 8.`, `WPOISON`/`RPOISON`
    (`ceil(DATA_WIDTH / 64)`), `RUSER` (`USER_DATA_WIDTH + USER_RESP_WIDTH`), the four `*IDCHK`
    (`ceil((ID_W_WIDTH + int(Unique_ID_Support))/8)`). These are real losses: `WSTRB` is one of AXI's
    core wires, its parity companion `WSTRBCHK` IS in the catalog, and AXI's own
    *"An attached Subordinate must have its WSTRB input tied HIGH"* obligation is demoted to a
    residual because of it.
  * **66 carry a width the source row never stated as one** — MMU-700's
    `Signal LAADDR is width 3'b000 , lavalid`, an upstream waveform/table misread. Refusing these may
    well be CORRECT; refusing them **silently** is the defect either way, and their real fix is
    upstream of this function.
  **Ordering AMENDED `2026-09-13`, before either child was implemented.** This node said to do the
  accounting first, because it was *"a precondition: it immediately tells the corpus which stratum
  each of the 83 is in"*. That is no longer true: `scripts/measure_declared_signals_missing_from_
  semantic.py` already splits the strata from persisted artifacts, offline, so the precondition is
  satisfied without changing any producer. The recall half is therefore first, and the accounting
  half keeps the population the recall half deliberately does not take.
  **And the recall half is much narrower than "arithmetic widths are unsupported."**
  `parse_width_token` ALREADY reads `DATA_WIDTH/8` as a parametric width. What it cannot do is span
  whitespace: `parse_optional_width_hint` consumes exactly ONE whitespace token, so
  `width DATA_WIDTH / 8` leaves `/` and `8` unconsumed and the whole declaration is discarded by the
  `index != tokens.len()` guard. The defect is a tokenization boundary, not a missing grammar.
  Prerequisite: none. Verification: per child.
  Commit: n/a (split)
- ID: `SIGNAL-DECLARATION-ROW-DROP.4a` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.4`) · Goal:
  **a width expression may span whitespace, and trailing prose is not a reason to discard the
  declaration in front of it.** The width branch reads a well-formed arithmetic expression — numbers,
  parametric identifiers, `+ - * /` and balanced parentheses, including a call form such as
  `ceil(…)` — over the tokens after `width`, and the declaration is admitted when one is found.
  **Two conditions keep it honest, and each is there because a real corpus declaration needs it:**
  the expression must end at a whitespace-token BOUNDARY, so MMU-700's
  `Signal LAADDR is width 3'b000 , lavalid` cannot be read as the number `3`; and trailing material is
  tolerated only when the expression is STRUCTURED (it contains an operator or a parenthesis), so ATB's
  ingest-mangled `width log 2 (DATA_WIDTH) -` is not read as a width of `log`.
  **Expected population, to be re-derived with the real reader: 14 of the 17 arithmetic-width
  declarations** — AXI-L 8 of 9, AXI-H 4 of 4, CHI 2 of 2 — leaving three whose expression is
  MALFORMED in the source (`ceil((USER_DATA_WIDTH USER_RESP_WIDTH)/8)` has no operator,
  `ceil((LTI_SSID_WIDTH +` is truncated, `log 2 (DATA_WIDTH) -` is mangled) to `.4b`.
  Non-goal: admitting a declaration whose width cannot be read at all — that is `.4b`, it has a
  different population (the 66 plus these three), and it is a different question.
  **MEASURED with the real reader `2026-09-13`, and the prediction held: 14 of the 17 read.** AXI-L 8
  of 9, AXI-H 4 of 4, CHI 2 of 2; the three refused are exactly the malformed ones. **Only ONE
  document's artifact can move**, and that is a separate fact worth its own sentence: of the 78
  documents, only 26 carry a current-schema EvidenceIR the semantic stage will accept, and AXI-H, CHI,
  ATB and LTI are not among them — their recovery is real in the reader and latent in the corpus until
  re-ingest. The corpus census therefore falls **83 → 75**, not 83 → 69.
  **AXI-L rebuilt from the semantic stage (this is not an evidence-stage change, so no normalized
  bundle is involved): catalog 288 → 296**, gaining `ARIDCHK`, `AWIDCHK`, `BIDCHK`, `RIDCHK`,
  `RPOISON`, `RUSER`, `WPOISON` and **`WSTRB`**. SemanticIR and IntentIR `signal_constraints`
  **55 → 56** and `residual_decisions` **1 → 0** — the one record un-demoted is
  *"An attached Subordinate must have its WSTRB input tied HIGH"*, the REAL obligation that
  `EXTRACTION-QUALITY-GAUGE.3k.7` found was being demoted beside the fabrication it removed. The
  emitted `.isf` goes **288 → 296 signals and 135 → 138 rules**. Nothing is removed anywhere.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `SIGNAL-DECLARATION-ROW-DROP.4a`
- ID: `SIGNAL-DECLARATION-ROW-DROP.4b` · Status: `done` (`2026-09-14`, CODE; opened `2026-09-13` by
  `.4`) · Children: `.4d` · Goal:
  **a declaration whose width cannot be read at all is counted and named, not dropped in silence.**
  The refusal itself is not the defect — `Signal LAADDR is width 3'b000 , lavalid` comes from a table
  that is not a signal description, and refusing it is very likely CORRECT. The defect is that the
  refusal leaves no record: no declaration, no residual, no counter, no validation entry, exactly as
  `.0` measured for the evidence-stage reader one stage up.
  **Split before implementation, and the reason is a measured correction to this node's own premise.**
  It inherited a residue of "69", and the SECOND question — should a refused declaration's DIRECTION
  survive? — was to be answered in the same leaf. That question is now `.4d`, because its population
  is not what either node assumed.

  **The residue this leaf inherited is 74/75 LEGACY, and no node had said so.** `.4a` re-derived the
  corpus census at 75 and named four documents (AXI-H, CHI, ATB, LTI) whose recovery is latent because
  their EvidenceIR is not current-schema. Reading the persisted SemanticIR schema of all ten documents
  in that census says the same about six more: **nine of the ten carry `schema_version: 1`**, so their
  catalog is an older binary's output that the current chain cannot reproduce. Re-derived through the
  product rather than asserted — `specforge semantic <doc>/evidence_ir.json --dry-run` refuses all
  nine with `EvidenceIR schema version 2 is legacy/proofless and inspection-only`. The
  **current-stratum residue is 1**: AXI-L's `RUSERCHK`, whose stated width
  `ceil((USER_DATA_WIDTH USER_RESP_WIDTH)/8)` is malformed (no operator between the two parameters)
  and which `.4a` deliberately refuses. MMU-700's 47 is real and it is **latent**, not current: it
  becomes observable only when MMU-700 is re-ingested.
  **Shipped.** `read_explicit_signal_declaration` is the same parse stating which of its three refusal
  arms fired, and `unreadable_declaration_residual_packet` carries ONE of them —
  `semantic_unreadable_declaration_width` — naming every identity that was refused for an unreadable
  width and reaches no interface record. **Which arm to report was measured, not chosen.** Over the 27
  chain-current documents the reader refuses **11** sentences: 8 `no_direction_and_no_width`, 2
  `name_not_an_identifier`, 1 `width_text_unread`. Adjudicated against their source statements, **all
  10 in the first two arms are English prose that opens with the word "signal"** — *"Signal names MUST
  adhere to the rules of the native tool"*, *"Signal arrays are identified by a name followed by a set
  of parenthesis"* — so reporting all three arms publishes `names`, `arrays`, `direction`, `is` and
  `at` as lost wires: **1 real identity in 8**, against **1 in 1** for the single arm. The split is
  structural rather than tuned: `.0`'s `_ => continue` arm drops a row with no attribute before any
  statement exists, so a `Signal …` sentence carrying neither a direction nor a width cannot be a
  declaration this pipeline synthesized.
  Prerequisite: `.4a`.
  Verification: see the acceptance checklist below.
  Commit: `SIGNAL-DECLARATION-ROW-DROP.4b`

- ID: `SIGNAL-DECLARATION-ROW-DROP.4d` · Status: `pending` (opened `2026-09-14` by `.4b`) · Goal:
  **should a refused declaration's IDENTITY and DIRECTION survive its unreadable width?** `.4b` made
  the refusal visible and deliberately did not answer this: `parse_explicit_signal_declaration`
  discards a direction it has already parsed along with the width it cannot read, and admitting those
  identities would move MMU-700's catalog by 47 and un-demote every constraint about them.
  **The population this leaf must adjudicate is LATENT, and that is the first thing to size.** `.4b`
  re-derived the stratum: 9 of the census's 10 documents carry legacy SemanticIR the current chain
  refuses, so today the whole current-stratum population is **one** identity (AXI-L `RUSERCHK`) — far
  too small to decide a rule on. The 74 latent ones are only observable through the real reader in one
  of two ways: re-ingest, or a read-only replay that runs the current reader over a persisted
  artifact's own statements, the way `replay-constraints` already does for the constraint producer
  (`TOOLBOX.md` §5.5). **Build the replay before deciding**, because the alternative — reading the
  legacy artifacts and reasoning about what the reader would do — is the mirror the doctrine refuses.
  **The prior ruling to beat:** `.1d` asked the same question at the row level one stage up and
  answered NO at **24% precision**, and `.4b`'s own adjudication of the no-attribute arms scored
  **1 in 8**. An affirmative answer here has to name what makes the width-unread arm different.
  Non-goal: admitting an identity because MMU-700's 47 look like real LTI wires; four of them
  (`LMACTIVE_7`, `LMASKCLOSE_7`, `LMOPENACK_7`, `LMOPENREQ_7`) carry an index-7 suffix with no
  siblings, which is the shape of an expansion artifact rather than a declaration.
  Prerequisite: `.4b`.
  Verification: the latent population read by the REAL reader (not a mirror), adjudicated per
  document; observed RED; the chain rebuilt for every document whose artifacts move.
  Commit: pending

- ID: `SIGNAL-DECLARATION-ROW-DROP.1d` · Status: `done` (`2026-09-14`, PROBE/DOC) · Children: `.1e` ·
  **Answered at the row level, and the answer is NO: 24% precision.** The question was whether a
  declaration must carry an ATTRIBUTE to carry an IDENTITY, after
  `SIGNAL-CATALOG-CAPTURE-GAP.6` found a class with no attribute to read.
  **The population is the reader's own accounting, not a scan** (`scripts/measure_dropped_row_offerings.py`):
  `.1` made the drop countable at runtime, so a dropped row here is one the real reader really dropped,
  with the reason it recorded. That accounting exists only in documents rebuilt since `.1` — **4 of 27**
  — and gives **71 dropped rows, 100% joined** back to their source row: 59 `no_direction_and_no_width`
  and 12 `name_not_an_identifier`. `.0`'s corpus-wide 482 is a scan of all 78 artifacts and is a
  different population; the two are never summed.

  | table | rows | what it is | admitting the identity |
  | --- | ---: | --- | --- |
  | AXI `table_0092` | 4 | *adjudicated `real` here and **CORRECTED by `.1e`**: the cells are the `Ax` METAVARIABLE (`AxLEN`, `AxSIZE`), `AXLEN` is undeclared while `AWLEN`/`ARLEN` are, and the caption reads "Signals that should be the same in an exclusive sequence"* | **phantom** |
  | ADIv6 `table_0058` | 5 | pin equivalence — `SWDIOTMS \| SWDIO \| TMS` | **real** |
  | ADIv6 `table_0039` | 3 | `signal \| programmers' model`, rotated — `CDBGPWRUPREQ` … | **real** |
  | ADIv6 `table_0041` | 2 | `DBGTDO`, `DBGTRSTn` — JTAG-DP signals | **real** |
  | AXI `table_0199` | 10 | a PAS ENCODING — `Secure`, `Root`, `Realm`, `SA` | phantom |
  | ADIv6 `table_0108` | 10 | a garbled body — name cells are `Out`, `TDI Out`, `TDO In` | phantom |
  | AHB `table_0014` | 8 | the HBURST ENCODING — `SINGLE`, `INCR`, `WRAP4` | phantom |
  | AHB `table_0019` | 6 | the HPROT ENCODING — `Opcode fetch`, `Data access` | phantom |
  | AXI `table_0265` | 6 | a LEGEND — `Y`, `YM`, `YS`, `O`, `NS` mean mandatory/optional | phantom |
  | AXI `table_0183`/`0184` | 5 | PARAMETER tables — `LOOP_W_WIDTH`, `USER_REQ_WIDTH` as wires | phantom |

  **10 real against 49 phantom** (`.1d` first counted 14/45; `.1e`'s grounding test caught AXI
  `table_0092`'s four rows as metavariables and the count is corrected here rather than left standing).
  A rule that admits an identity with no attribute would mint 49 new phantoms in the current stratum — width parameters, encoding names, and a legend — which is
  precisely what `.2a`'s placeholder refusal and `ACTOR-NOUN-RELATION-DECLARATION.1`'s orthography rule
  exist to stop. The refusal stands.
  **What the census also shows is where the answer lives.** Every one of the ten tables is uniformly
  real or uniformly phantom — not one has a mix. **The decision is a property of the TABLE, not of the
  row**, which is the same conclusion `PROSE-NAME-CELL-DECLARATION.0` reached about eMMC's bus-mode
  matrix from the other direction. Split to `.1e`, which then corrected one of this node's own verdicts.
  Verification: `python3 scripts/measure_dropped_row_offerings.py`, read-only, 71/71 joined; every row
  printed with its headers and its full source row, and adjudicated in the table above.
  Commit: `SIGNAL-DECLARATION-ROW-DROP.1d`

- ID: `SIGNAL-DECLARATION-ROW-DROP.1e` · Status: `done` (`2026-09-14`, PROBE/DOC) · **Three table-level
  discriminators tried, three refuted — and the third one corrected `.1d`'s own adjudication before it
  was refuted.** The `.1` branch closes here: the refusal is correct and its residue is small and named.
  **Two shape tests, refuted against the ten tables:**
  - *every column header is a name header* selects only ADIv6 `table_0058`. AXI `table_0092`'s headers
    are literal signal names (`AxID`, `AxADDR`) and match no name keyword at all;
  - *every body cell is an identifier* admits AXI `table_0265`, whose cells are `Y | Mandatory |
    Mandatory` — `Mandatory` is a lone word the identifier test accepts.
  **The grounding test — are the table's cells drawn from the document's own declared catalog? — earned
  its keep by falsifying a hand verdict.** AXI `table_0092` scored **0.00** against a 297-name catalog,
  which looked like a false negative until the names were checked: `AXLEN`, `AXSIZE` and `AXPROT` are
  **undeclared** while `AWLEN`, `ARLEN`, `AWSIZE`, `ARSIZE` and `AWPROT` are all declared, and the
  caption is *"Table A6.5: Signals that should be the same in an exclusive sequence"*. The cells are
  AXI's `Ax` metavariable for a signal FAMILY, not a wire name; admitting them would mint `AXLEN`, which
  the document never writes. **`.1d`'s count is corrected above from 14/45 to 10 real / 49 phantom —
  17% precision, not 24%.**
  **And then the test itself is refuted, for a stated methodological reason.** Scored over every
  current-stratum boundary table, it looks strong — 28 of 58 at ≥ 0.50 — but that number is
  **circular**: a table that DECLARED its own signals scores against a catalog it fed, so it trivially
  reaches 1.00 (AXI `table_0246`–`0249` at 0.97–1.00). Excluding declaring tables leaves the population
  a rule would actually act on, and it is tiny and bimodal:

  | grounded ratio | non-declaring boundary tables (≥ 4 identifier cells) |
  | --- | ---: |
  | 0.00 | 9 |
  | < 0.25 | 1 — ADIv6 `table_0108` at 0.12, the garbled body, phantom |
  | 0.25 – 0.50 | **0** |
  | 0.50 – 0.75 | **1** — ADIv6 `table_0058` at 0.55, the pin-equivalence table, REAL |
  | ≥ 0.75 | 0 |

  Any threshold in `(0.12, 0.55]` selects exactly **one table, five rows, no false positive**. That is a
  clean separation and it is **one instance** — a threshold justified by a single positive is a rule
  fitted to one table, which this tree has refused three times already.
  **The test is also structurally blind to the best candidates.** ADIv6 `table_0041` holds real signals
  and is excluded from the measurement entirely, because it PARTIALLY declares: a table that fed the
  catalog cannot be scored against it. The partially-declaring tables are exactly where recall would
  come from, and grounding cannot see them.
  **What remains, stated so it is not re-derived**: ~10 recoverable rows in three ADIv6 tables, against
  49 phantoms, with no vocabulary-free table-level test yet separating them. Reopen only with a
  discriminator that survives all ten tables AND is not circular.
  Verification: read-only. Grounding ratios computed from each document's own declared catalog
  (`Signal X is …` statements plus `table_signal_declaration_provenance`); the metavariable finding
  confirmed by checking `AXLEN`/`AWLEN`/`ARLEN` membership directly and by the table's caption.
  Commit: `SIGNAL-DECLARATION-ROW-DROP.1e`

- ID: `SIGNAL-DECLARATION-ROW-DROP.4c` · Status: `pending` (opened `2026-09-14` by the chain-currency
  sweep; **its opening premise was corrected the same day by tracing the conflict to its tables and the
  signal to its `.isf`**) · Goal: **a width conflict costs the signal its width in SemanticIR, and the
  emitter then writes a width the document never stated.**
  **What the leaf opened on, and what is actually true.** It opened reading `.4a` as a regression: APB-e
  states `PADDRCHK`'s width three times, `.4a` made the footnote-marked form readable, and the interface
  reader turned two spellings into a `width_mismatch` and dropped the width. Traced to their tables, the
  two observations are **not two readings of one statement**:

  ```text
  table_0014  Table 5-1 Check signal descriptions
              PADDRCHK | PADDR | ceil(ADDR_WIDTH/8) a | 1-8 | PSEL
  table_0017  Signal | Width | Property | APB5 | APB4 | APB3 | APB2   (body rotated by one)
              ADDR_WIDTH/8 | Check_Type | C | N | N | N | PADDRCHK
  ```

  The document states `ceil(ADDR_WIDTH/8)` in the table that describes check signals and `ADDR_WIDTH/8`
  in its version matrix, and those are **not the same width** for an `ADDR_WIDTH` that is not a multiple
  of 8. **The conflict is real and reporting it is right**; `.4a` is not a regression and this leaf no
  longer claims it is.
  **The defect is what happens after the conflict.** A conflicted signal loses its `width_hint` from
  every `actor_ports`, `interfaces` and `signal_connectivity` record, so the SemanticIR product boundary
  carries no width for a signal the document does state a width for — twice.
  **Blast radius, measured rather than asserted, and smaller than the leaf first said**: the emitted
  `.isf` does **not** move. `PADDRCHK` already ships as `(output PADDRCHK (width 1))` today, and so do
  **31 of APB-e's 32** emitted signals; the one current-stratum conflict, AHB `HBURST` (`3` against
  `HBURST_WIDTH`), likewise ships as `(output HBURST (width 1))` for a signal the document states as 3
  bits. The width-1 default is a far larger, pre-existing defect that this leaf does not own.
  **The hold is therefore LIFTED, and the rebuild has since HAPPENED and confirmed the prediction.**
  `CORPUS-CHAIN-CURRENCY.7` rebuilt APB-e on `2026-09-14`: `interface_signal_conflicts` 0 → 1, two
  `actor_ports` records lose `width_hint`, and the emitted `.isf` `source_text` is **byte-identical at
  4,029 bytes** — so the measurement this node was corrected by is now confirmed through the real chain
  rather than by reading the pre-rebuild artifact. The APB wire gold is unchanged.
  What remains to decide: whether a contested width should be carried with its provenance and a
  contested flag, or refused outright — and that only matters once the emitter stops defaulting to 1, so
  size it against that question rather than alone. Corpus population: **1** current-stratum
  `width_mismatch` (AHB `HBURST`) and 48 legacy, of which the LTI and AXI-H ones are garbage widths from
  the rotation defect (`LAVALID`, `RESETn`, `V` as "widths") rather than real disagreements.
  Prerequisite: none. Verification: the corpus population above, adjudicated; the emitter's width
  behaviour established before any change, because a width restored into a `(width 1)` emitter is
  invisible.
  Commit: pending

- ID: `SIGNAL-DECLARATION-ROW-DROP.2e` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.4b`'s re-derivation) · Children: `.2f` · Goal: **a decoy header substring turns an ALIGNED header into a whole-table column
  rotation, and every other column moves with it.**
  `.2a` gave this reader a content-based name-column override: when a different column holds decisively
  more distinct signal tokens than the header-named one, that column wins. It was written for a header
  row SHIFTED relative to the body, so it applies the difference as an OFFSET and rotates the width,
  direction, source and destination columns by the same amount.
  **MMU-700 `Table B-6: LTI TBU observation interface signals` is aligned, and it still rotates.** Its
  header is `SIGNALGRP<n> | Bits | Signal name | SIGQUAL<n> 4'b{MSB..LSB} | Number of cycles of delay`.
  The name-column scan takes the FIRST header containing `signal`, which is the DECOY `SIGNALGRP<n>` at
  column 0; the content override then correctly finds the real names in column 2 and rotates everything
  by +2 — so the width column moves off `Bits` (`[64:1]`) and onto `SIGQUAL<n>`, and all 17 rows
  declare `width 3'b000 , lavalid`. That is a FABRICATED width, and it is the whole of MMU-700's
  contribution to the 75 signals that never reach the SemanticIR catalog: 47 of the 75.
  **The discriminator is already in the data:** when the header cell AT the winning column is itself a
  name header, the header row is aligned and the first scan simply matched a decoy — so the name column
  moves and nothing rotates. A genuinely shifted header has no name keyword at the winning column,
  which is the case `.2a` measured (APB `table_0016` 18 against 5, AHB `table_0033` 19 against 4).
  **The second half was BUILT, MEASURED, and taken back out — which is the result.** With the width
  column back on `Bits`, the cell is a BIT RANGE (`[64:1]`), and reading `[hi:lo]` as `hi - lo + 1`
  does recover the rows: the real reader then declares `zetatlbloc` 16, `zetaid` 32, `zetaaddr` 64.
  It also declares **`Signal Unused is width 2.`** — the table's two `Unused` spacer rows become
  wires. They are dropped TODAY only by accident, because the rotated width cell was `-`. Trading 15
  recoveries for 2 phantoms is not a quality win in a document nothing can rebuild, and
  `inferred_name_is_an_ordinary_word` — the orthography rule that would refuse `Unused` — states in
  its own contract that it is **never** applied to a table declaration, *"where the document's own
  spelling is authoritative and this reader has no business overruling it"*. Inventing a second rule
  for one table is the mirror this tree keeps refusing. The bit-range width is therefore split out as
  `.2f` behind that prerequisite.
  **Shipped: the rotation narrowing alone, and it is a strict precision win.** MMU-700 goes from 17
  declarations carrying a FABRICATED width to 0 declarations and 17 rows counted as
  `NoDirectionAndNoWidth` — the loss made visible, which is `.1`'s whole thesis, instead of a width
  invented for it. Observed RED on the file at `HEAD`: the alpha-renamed table declared
  `Signal zetatlbloc is width 3'b000 , zetavalid.` three times over.
  **Measured over every rebuildable document: nothing moves.** `evidence --dry-run` over all
  **27** documents that have a normalized bundle (the 24 retained plus AHB/AXI-L/APB-E restored from
  the held-out set for the measurement, `diff -rq` verified and removed again, retention back to 24)
  — **0 of 27 change a single declaration**. The entire population of this defect is in frozen
  legacy artifacts, so the published effect is zero today and lands the moment MMU-700 is re-ingested.
  The class is demonstrable through the real reader, which is the footing `EXTRACTION-QUALITY-GAUGE.3k.1`
  established and `.3k.2e`/`.3k.2f`/`.3k.2k` shipped on; the risk is measured at zero, which is what
  separates it from `EXTRACTION-QUALITY-GAUGE.3k.9`, where a zero-effect fix would have moved the
  identity layer of 67 documents.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `SIGNAL-DECLARATION-ROW-DROP.2e`
- ID: `SIGNAL-DECLARATION-ROW-DROP.2f` · Status: `pending` (opened `2026-09-13` by `.2e`) · Goal:
  **a bit range is a width, once a spacer row is not a signal.** `parse_table_width_hint_text` reads a
  number and a parametric string; `[125:110]` is neither, so a `Bits` column states a width this
  reader cannot use. `[hi:lo]` is `hi - lo + 1` — universal notation, no vocabulary — and it recovers
  MMU-700's 15 real observation-interface signals (`.2e` proved this through the real reader).
  **It cannot ship until a row whose name cell is `Unused` stops being a declaration**, or it trades
  15 recoveries for 2 phantoms. That question is not this leaf's to answer: the orthography rule that
  would settle it is contractually barred from the table path, and the row-shape question belongs to
  `PROSE-NAME-CELL-DECLARATION`. Measure the corpus population of bit-range width cells before
  implementing — a `Bits` column is common, and the blast radius is not this one table.
  Prerequisite: `PROSE-NAME-CELL-DECLARATION` deciding the non-name row cell, or an equivalent
  structural refusal with its own measured population.
  Verification: the corpus population of bit-range width cells measured with the real reader and
  adjudicated; observed RED; the chain rebuilt for every document whose artifacts move.
  Commit: pending

- ID: `SIGNAL-DECLARATION-ROW-DROP.3` · Status: `done` (`2026-09-11`) · Goal: **the declared spelling
  must be the document's spelling.** Under ADR 0037 case carries no alias authority, so emitting a
  case-variant the source never wrote mints an identifier rather than grounding one.

  **The census was run first, and it corrected this leaf's own premise.** Over all 78 persisted
  artifact pairs, joining each `table_signal_declaration_provenance` entry to every token its source
  document actually writes: **716 of 2,085 declarations in the LEGACY stratum** carry a spelling their
  document never uses — `ARESETN` for `ARESETn`, `PSELX` for `PSELx`, `NRESET` for `nRESET`,
  `AMEVCNTRN_EL0` for `AMEVCNTRn_EL0`, `STREAMID` for `StreamID`, and MMU-700 alone accounting for 488
  where the document writes `qactive_cg` and the artifact says `QACTIVE_CG`. In the **current
  proof-carrying stratum the count is 0 of 604**. The defect is historic: the legacy artifacts predate
  its removal, and `known_signals` — this leaf's named suspect — turns out to resolve a proposed
  spelling *back to* the declared one (`parse_nlp_relations`, `resolve_declared_signal_identifier`),
  which is the correct behaviour, not the source of the uppercase.
  **The current-artifact evidence is weak on its own and this leaf says so:** only 4 of the 27
  proof-carrying documents produce table declarations at all, and AXI is 462 of the 604 while writing
  its signals upper-case natively, so a case-folding emitter would be nearly invisible there. The
  authority is therefore the reader, not the artifacts — see the checklist.

  **What this leaf ships is a guard, not a fix**, and the distinction is stated rather than blurred:
  `a_declared_name_keeps_the_cell_s_own_spelling` passes on the unmodified tree. It is worth its place
  because the property is load-bearing twice over — the folded name is one ADR 0037 forbids, and
  `is_alpha_variant_placeholder` refuses a relation-derived placeholder by finding an **interior**
  lower-case position, so folding is how that control gets disarmed. It was observed RED against a
  reintroduced fold, returning exactly the legacy spellings.
  Non-goal: normalising case for comparison — matching case-insensitively is correct and stays.
  Non-goal: repairing the legacy artifacts. They are inspection-only and cannot be rebuilt; the 716 is
  a fact about stored files, not about the product.
  Prerequisite: none (independent of `.1`/`.2`).
  Verification: see the `.3` checklist below.
  Commit: see log.

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
  so the corpus cannot have moved. Confirmed after the commit: the re-run reports *"every measurable
  persisted artifact is exactly what the current binary produces"* — evidence 24/24/0,
  semantic·intent·isf-adapter 27/27/0, retention 24.
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

## Acceptance Checklist — `.3` (enforced)
- [x] **REPRODUCE / MEASURE** — read-only census over all 78 persisted `source_ir.json` +
  `evidence_ir.json` pairs: every `table_signal_declaration_provenance` name is looked up against the
  complete token set its own document writes (content elements plus every table header and body cell).
  **Legacy stratum: 716 of 2,085** declarations carry a spelling the document never writes, across 17
  documents. **Current proof-carrying stratum: 0 of 604**, across the 4 documents that declare.
- [x] **ROOT CAUSE (WHY + WHERE)** — none in the current producer, which is the finding. Every
  `to_ascii_uppercase` reachable from the declaration path is comparison-normalisation, not emission:
  `strip_signal_mentions_from_semantic_hint_text` (`evidence.rs:8967`), the column-scoring closure
  inside `synthesize_signal_declarations` (`evidence.rs:10451`), `extract_condition_clause`
  (`evidence.rs:9974`, condition text), `extract_enum_member_name` and
  `synthesize_encoding_declarations_for_enum` (enum members, a different surface). `known_signals`
  resolves a proposed spelling to the canonical declared one rather than folding it
  (`ir/nlp_relation_extract.rs:74`). The uppercase in the legacy artifacts entered through a producer
  that no longer exists.
- [x] **ADDRESSED (verified)** — control `a_declared_name_keeps_the_cell_s_own_spelling` asserts both
  the provenance names and the emitted sentences for five spellings the corpus actually uses:
  `ARESETn` (polarity), `PSELx` (final index marker), `AMEVCNTRn_EL0` (interior index marker),
  `qactive_cg` (an all-lower-case document), `StreamID` (neither convention). **Observed RED:**
  reintroducing a fold in `signal_names_in_name_cell`'s result makes it fail with exactly
  `["ARESETN", "PSELX", "AMEVCNTRN_EL0", "QACTIVE_CG", "STREAMID"]` — the legacy spellings, reproduced
  character for character, which is what makes this a guard against the real defect rather than a
  tautology.
- [x] **NO REGRESSION** — `cargo test --offline -p specforge-core --lib` **1415 passed, 0 failed**
  (1414 + this control); `-p specforge --lib` 472 passed; `cargo fmt --all --check` clean;
  `cargo clippy --offline --all-targets -- -D warnings` exit 0; `bash scripts/check_doctrines.sh` all
  gate-tier PASS. Test-only change — no production line moves, so no artifact, score, or gold can.
- [x] **GENERICITY (ADR 0006)** — the control is a fixture; the five spellings are document text in a
  test, which is where protocol-specific text is allowed. No production rule learns a name.
- [x] **LOCKSTEP** — no user-visible behaviour changed, so the book is unchanged by the producer
  sub-clause; the durable finding is the fact card `[[declared-spelling-is-the-document-spelling]]`,
  which records the 716/0 split so the legacy artifacts are not mistaken for current evidence.


## Acceptance Checklist — `.4a` (enforced)

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_declared_signals_missing_from_semantic.py`
  (self-test 7/7): **83 declared signals across 10 documents never reach the SemanticIR catalog**, 17
  of them with an arithmetic width. Each of the 17 was then run through the REAL parser and read
  against its source declaration: 14 are well-formed expressions, 3 are malformed in the document
  (`ceil((USER_DATA_WIDTH USER_RESP_WIDTH)/8)` has no operator, `ceil((LTI_SSID_WIDTH +` is truncated,
  `log 2 (DATA_WIDTH) -` is ingest-mangled).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs`:
  `parse_optional_width_hint` consumes exactly ONE whitespace token, and
  `parse_explicit_signal_declaration` then refuses the sentence with `index != tokens.len()`. So
  `Signal WSTRB is output width DATA_WIDTH / 8.` is discarded whole — identity and direction with it —
  while `DATA_WIDTH/8` written without spaces parses. Observed RED against the file at `HEAD`: that
  exact declaration returns `None`.
- [x] **ADDRESSED (verified)** — `width_expression_length` (recursive descent over numbers,
  parametric identifiers, `+ - * /`, balanced parentheses and the call form) and
  `parse_width_expression` (admits one only when it ends at a whitespace-token boundary, and is
  STRUCTURED if anything follows it). **14 of 17 read.** AXI-L rebuilt `semantic → validate → intent
  → validate → adapt`: catalog **288 → 296**, `signal_constraints` **55 → 56**,
  `residual_decisions` **1 → 0**, `.isf` **288 → 296 signals / 135 → 138 rules**. Corpus census
  **83 → 75**; the other recoveries are latent because AXI-H, CHI, ATB and LTI carry legacy/proofless
  EvidenceIR the semantic stage refuses. A dry run over all 78 documents confirms **AXI-L is the only
  artifact that moves**.
- [x] **NO REGRESSION** — wire golds `signal_constraint P=R=F1=1.000` with **fp=0** on APB, AHB, AXI
  and SWD, `temporal_rule 1.000` on AXI; `kg-bench` **156/156**; `cargo fmt --all --check` and
  `cargo clippy --offline --all-targets -D warnings` clean; the whole workspace suite green
  (`specforge-core` lib 1,521 → **1,526** passing, `specforge` 472, conformance 168,
  production-graph 8/8); `flow_census.json` re-derived and attributed (+2 functions, +11 decision
  sites, +5 helper edges). Nothing is removed from any artifact anywhere in the corpus.
- [x] **GENERICITY (ADR 0006)** — arithmetic-expression grammar only. A function call is recognised by
  its SHAPE, never by its name; no document, protocol, vendor or parameter vocabulary appears.
- [x] **LOCKSTEP** — the book's `evidence-failure-modes` chapter, which owns *"rows that are simply
  dropped"*, gains *"The same loss one stage later"* — the same failure at the SemanticIR reader, with
  the `WSTRB`/`WSTRBCHK` tell and the two conditions. The fact card
  `[[arithmetic-width-drops-the-declaration]]`, written by `EXTRACTION-QUALITY-GAUGE.3k.7` one commit
  earlier, is updated in place: its census, its `reverify` expectation and the residue owned by `.4b`.
  No production rule was deleted.

## Acceptance Checklist — `.2e` (enforced)

- [x] **REPRODUCE / MEASURE** — the 75 signals that never reach the SemanticIR catalog were split by
  document: **47 of them are one MMU-700 table**, and every one of those 47 is declared with the width
  `3'b000 , lavalid`, a string the document states about no signal. Read against source:
  `Table B-6` heads `SIGNALGRP<n> | Bits | Signal name | SIGQUAL<n> 4'b{MSB..LSB} | …` and its `Bits`
  column holds `[64:1]`, `[125:110]` — the real widths, one column away from the one the reader used.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `synthesize_signal_declarations`: the header scan takes the FIRST header containing `signal`, which
  is the decoy `SIGNALGRP<n>` at column 0; `.2a`'s content override then finds the real names in
  column 2 and applies the difference as an OFFSET, rotating width, direction, source and destination
  by +2. Observed RED on the file at `HEAD` with the table alpha-renamed: three declarations, each
  carrying the SIGQUAL cell as its width.
- [x] **ADDRESSED (verified)** — one shared `is_signal_name_column_header` decides both the header
  scan and, at the override's winning column, whether the header was aligned all along; an aligned
  header moves the name column and rotates nothing. The table now yields **0 declarations and 4 rows
  counted `NoDirectionAndNoWidth`** — the fabrication removed and the loss made visible, which is
  `.1`'s accounting doing exactly what it was built for. A second control proves the override still
  moves the NAME column: with a readable width in `Bits`, the same table declares
  `zetatlbloc` 16, `zetaid` 32, `zetaaddr` 64.
- [x] **NO REGRESSION** — `evidence --dry-run` over **all 27 documents that have a normalized bundle**
  (the 24 retained plus AHB, AXI-L and APB-E restored from the held-out set for the measurement,
  `diff -rq` byte-identical, removed again, retention back to **24**): **0 of 27 change a single
  declaration**. `wire_based_100_5h::rotated_signal_table_extracts_name_from_last_column` — the
  genuinely shifted AHB `table_0009` the override exists for — still passes, and its winning column's
  header is `Description`, which is what makes the two cases distinguishable. Wire golds
  `signal_constraint P=R=F1=1.000` with **fp=0** on APB, AHB and AXI; `kg-bench` **156/156**;
  `cargo fmt --all --check` and `cargo clippy --offline --all-targets -D warnings` clean; workspace
  suite green (`specforge-core` lib 1,526 → **1,529** passing); corpus replay unchanged at 192/119;
  `flow_census.json` re-derived and attributed.
- [x] **GENERICITY (ADR 0006)** — a header-keyword test this reader already had, asked a second time
  at a column the content scan chose. No document, protocol, vendor or signal vocabulary.
- [x] **LOCKSTEP** — no user-visible behaviour changes in any document the corpus can rebuild, so the
  book is unchanged; the durable finding is this leaf and `.2f`, which carries the measured reason the
  recall half is blocked. No production rule was deleted. No KM card: the fact is a defect that is now
  fixed, and the rule it establishes lives in the code's own contract.

## Acceptance Checklist — `.4b` (enforced)

- [x] **REPRODUCE / MEASURE** — `python3 scripts/measure_declared_signals_missing_from_semantic.py`
  (self-test 7/7): **75 declared signals across 10 documents** never reach the SemanticIR catalog. The
  reader that refuses them says nothing: no declaration, no residual, no counter, no validation entry.
  **The census's own stratum was then re-derived and it corrects the population**: nine of the ten
  documents carry `"schema_version": 1` SemanticIR, and
  `./target/release/specforge semantic generated/evidence_ir/<doc>/evidence_ir.json --dry-run` refuses
  all nine with `EvidenceIR schema version 2 is legacy/proofless and inspection-only`. **74 of the 75
  are latent; the chain-current residue is 1** (AXI-L `RUSERCHK`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs`,
  `parse_explicit_signal_declaration`: three `return None` arms — a name that is not an identifier, no
  direction and no width, and `index != tokens.len()` — are indistinguishable to every caller, so a
  refusal cannot be told from "this sentence was never a declaration". `build_interfaces` is the only
  producer of `InterfaceRecord::signal_records`, `declared_signal_names` is built from them, and the
  grounding partition keeps a `signal_constraint` only when that set holds its subject — so the silent
  refusal deletes the identity AND every obligation about it. Observed on AXI-L:
  `Signal RUSERCHK is width ceil((USER_DATA_WIDTH USER_RESP_WIDTH)/8).` returns `None` because the
  source expression has no operator between the two parameters.
- [x] **ADDRESSED (verified)** — `read_explicit_signal_declaration` returns
  `NotADeclaration | Refused(reason) | Read(_)`, and `unreadable_declaration_residual_packet` emits
  `semantic_unreadable_declaration_width` naming every `width_text_unread` identity that reaches no
  interface record. **Which arm to report was MEASURED**: over the 27 chain-current documents the
  reader refuses 11 sentences (8 `no_direction_and_no_width`, 2 `name_not_an_identifier`, 1
  `width_text_unread`), and adjudicating each against its source statement, all 10 in the first two
  arms are English prose opening with the word "signal" — *"Signal names MUST adhere to the rules of
  the native tool"*, *"Signal arrays are identified by a name followed by a set of parenthesis"*.
  Reporting all three arms names `names`, `arrays`, `direction`, `is` and `at` as lost wires — **1
  real identity in 8**; the single arm is **1 in 1**. Replay over all 27 with the shared
  CHAIN-CURRENCY predicate (`scripts/lib/stage_artifact_identity.sh`): **exactly 1 document moves**,
  `residual_decisions(0->1)`. AXI-L rebuilt `semantic → validate → intent → validate → adapt →
  validate`: catalog unchanged **296**, `signal_constraints` unchanged **56**, `residual_decisions`
  **0 → 1**, adapter residuals **30 → 31**, and the emitted `.isf` `source_text` **byte-identical** —
  31,691 bytes, sha256 `7b1b67fe…`, 296 signals, 138 rules.
- [x] **NO REGRESSION** — `kg-bench` **156/156**; wire golds re-scored from the persisted corpus,
  `signal_constraint P=R=F1=1.000 fp=0` on APB/AHB/AXI/SWD and `temporal_rule 1.000` on
  APB/AHB/AXI temporal; `scripts/check_chain_currency.sh` **evidence 24/24 current, semantic 27/27
  current, intent 27/27 current, isf-adapter 27/27 current, 0 stale**; `cargo fmt --all --check` and
  `cargo clippy --offline --all-targets -- -D warnings` clean; workspace suite green
  (`specforge-core` lib 1,531 → **1,535** passing, `specforge` 472, conformance 168,
  production-graph 8/8); `flow_census.json` re-derived and attributed (+3 functions, +7 decision
  sites, +40 helper edges, +1 semantic macro). **RED observed**: widening the packet filter to admit
  `NoDirectionAndNoWidth` fails `prose_opening_with_signal_is_not_a_lost_declaration` by name, and
  reverting restores green.
- [x] **GENERICITY (ADR 0006)** — the three refusal arms are properties of the sentence's own shape;
  no document, vendor, protocol or signal vocabulary appears in the enum, the filter or the packet.
  The named identities are document text carried as provenance, not as a rule input. No new
  registered field: `SEMANTIC_RULE_FIELDS` stays at its existing count because the packet lands on
  the already-registered `residual_decisions` surface, which is why the proof seal does not restamp.
- [x] **LOCKSTEP** — the book's `pipeline/evidence-failure-modes.md`, which `.4a` extended with *"The
  same loss one stage later"*, gains *"The refusal is no longer silent"*: the packet, the measured
  one-arm boundary with its 1-in-8 against 1-in-1, and the legacy/current stratum correction. Fact
  card `[[arithmetic-width-drops-the-declaration]]` updated in place with the `.4b` section, five new
  question keys and a `reverify` that now pins both the stratum refusal and the packet census.
  **Producer sub-clause: no production rule was deleted or replaced** — `parse_explicit_signal_
  declaration` refuses exactly the sentences it refused before, so no book text describes behaviour
  that has gone away.

## Current Frontier

Ordered; PNT selects the first eligible leaf.

0. `SIGNAL-DECLARATION-ROW-DROP.4d` — should a refused declaration's identity and direction survive an
   unreadable width? Needs the latent population read by the REAL reader first; the current stratum
   holds exactly one instance, and the two prior rulings on the same question scored 24% and 1-in-8.
1. `SIGNAL-DECLARATION-ROW-DROP.4c` — **no longer blocks a rebuild.** The two widths are stated in two
   different APB-e tables and genuinely differ, so `.4a` is right to report a conflict; what is wrong is
   that the conflict costs the SemanticIR width, and even that changes no emitted `.isf` because the
   signal already ships `(width 1)`. Size it against the emitter's width-1 default, not alone.
2. `SIGNAL-DECLARATION-ROW-DROP.2f` — a bit range is a width; blocked on a spacer row not being a
   signal.
3. `SIGNAL-DECLARATION-ROW-DROP.2d` — the actor-taxonomy gap behind 49 fail-closed arrow rows. Census
   the blast radius before touching `builtin_actor_taxonomy_role_in_text`.
