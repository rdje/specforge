# CORPUS-CHAIN-CURRENCY: prove, not assume, that every persisted chain matches the current binary

## Metadata

- Tree ID: `CORPUS-CHAIN-CURRENCY`
- Status: `active`
- Roadmap lane: `R15e`/`R16` corpus digestion (sibling of `CORPUS-COVERAGE`)
- Created: `2026-08-10`
- Last updated: `2026-08-10`
- Owner: repo-local workflow

## Goal

Make "the persisted corpus chain reflects the current binary" a **measured, gated** property instead of an
assumption. `CORPUS-COVERAGE`'s stated goal is to keep completed chains non-stale, and its `.1` stage-staleness
lane is marked complete — but nothing re-checks currency after a shared extractor changes. A repair leaf rebuilds
only the documents its own change touches, so every other completed document silently drifts one code delta at a
time.

## Non-Goals

- Do not re-ingest documents; currency measurement must be read-only or use `--dry-run` replay only.
- Do not silently refresh a document outside its owning `CORPUS-COVERAGE` leaf.
- Do not treat a non-rebuildable document (no normalized bundle) as current merely because it cannot be replayed.

## Reproduction and measurement (`2026-08-10`)

Discovered while completing `CORPUS-COVERAGE.2.50a`. Rebuilding the three documents that repair affected showed
their pre-existing chains were not reproducible by the **pre-repair** binary, so the rebuild also carried every
delta accumulated since each document's own refresh. Measured for USB 3.2 (`refresh #33`): timing constraints
102 → 74, actors 18 → 16, behaviors 2,705 → 866, all before the `.2.50a` subtraction.

A read-only `evidence --dry-run` replay of every rebuildable document against its persisted artifact then measured
the standing drift:

| Population | Count | Result |
| --- | ---: | --- |
| Documents replayable at the **evidence** stage (normalized bundle present) | 22 | replayable read-only |
| Persisted **EvidenceIR** already current | 19 | no section differs |
| Persisted **EvidenceIR** would change on rebuild | 3 | timing surfaces only |
| Documents not replayable at the evidence stage | 58 | **not measurable** without re-ingest |

Scope correction (`.1`, `2026-08-10`): this census covered the **evidence stage only**. `.1` later measured the
whole chain and found the semantic, intent, and adapter stages badly stale corpus-wide — and established that
those stages need no normalized bundle, so "58 non-rebuildable documents" is true of the evidence stage alone,
not of the chain.

The three drifted documents are `ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification`
(timings 6 → 0), `opencapi_25gbps_phy_signaling_spec_1_0` (70 → 55), and `opencapi_discovery_configuration_v201`
(6 → 0) — residue of the `.2.47a`/`.2.48a` timing-authority repairs, which rebuilt only the documents they had
measured as affected.

The first of those was the **SWD wire gold document**, so the WIRE-BASED-100 gold eval was scoring an artifact the
current binary would not produce. `eval-extraction` reads persisted artifacts by design, so "the gold is 1.000" and
"the current code scores 1.000" were two claims and only the first was gated. `.0` closed that gap for this
document: after the rebuild every scored SWD surface still reports 1.000, so both claims now hold.

## Decisions (ADR 0025 — accepted `2026-08-10`)

Every question this tree opened is decided from the measurement above, not from preference.
See [`docs/decisions/0025-persisted-chain-currency-is-measured-not-assumed.md`](../decisions/0025-persisted-chain-currency-is-measured-not-assumed.md).

1. **Rebuild to currency; prove isolation by replay.** A leaf that changes a shared extractor runs the exhaustive
   baseline-versus-change `--dry-run` replay, then rebuilds every rebuildable document whose replay differs from
   its persisted chain — not only the ones its own change moved — attributing each delta to this change or to a
   named earlier leaf. Isolation comes from the old-versus-new comparison at fixed inputs, so withholding a
   rebuild never sharpens evidence; it only leaves the corpus wrong. The replay costs about four seconds.
2. **Currency is a gated doctrine, fail-closed on a present corpus.** `CHAIN-CURRENCY` replays every rebuildable
   document and fails when a persisted chain differs from what the current code produces. It skips loudly when
   `generated/` is absent, because a fresh clone and hosted CI have no corpus. At roughly 36 seconds of debug
   replay plus a cargo freshness build it is a CI-tier doctrine (`DOCTRINE_ENFORCEMENT.md` §4.7), and it always
   reports the unmeasurable population so partial coverage can never read as full.
3. **Normalized bundles are retained.** 22 bundles occupy 1.3 GB against 3.4 TB free; the full corpus extrapolates
   to about 4.7 GB, roughly 0.14% of available space. Retention is what makes a document replayable, so a refresh
   keeps its bundle and `clean --scope source-normalized` becomes deliberate, task-owned reclamation. The 58
   missing bundles are backfilled at each document's own refresh; no bulk re-ingest is scheduled, because
   re-ingesting the corpus is an hour-scale, RAM-sensitive mutation needing its own owned leaf.

## Task Tree

- ID: `CORPUS-CHAIN-CURRENCY.0` · Status: `done` (`2026-08-10`, PROBE/DATA/DOC) · Goal: measure the drift, decide
  the policy, and close the measured backlog. The read-only census measured 22 rebuildable documents (19 current,
  three drifted, 58 unmeasurable); ADR 0025 records the three decisions; the three drifted documents were rebuilt
  and validated through all four stages, taking the evidence-stage population to **22/22 current** (an
  evidence-stage claim only — `.1` measured the rest of the chain and found it stale; see the `.1` census).
  All 27 records the rebuild removed are prose cells misread as timing parameters — a JTAG/SWD ACK-response table
  (`parameter_name: "Read"`, `unit: "Capture read data."`), OpenCAPI crosstalk rows whose `max_value` repeats the
  description, and OpenCAPI configuration-limit rows. Nothing real was lost and nothing was added.
- ID: `CORPUS-CHAIN-CURRENCY.1` · Status: `done` (`2026-08-10`, CODE/DOC) · Goal: ship
  `scripts/check_chain_currency.sh` per the §4 check contract and register it as the `CHAIN-CURRENCY` doctrine.
  Shipped: the check replays `evidence`/`semantic`/`intent`/`adapt --target isf` with `--dry-run` from each
  **persisted** upstream artifact and additionally compares every emitted `.isf` with the replayed adapter's
  rendered `isf.source_text` (mirroring `write_to_disk` + `reconcile_emitted_isf_files`); it fails closed on any
  difference, skips loudly when the corpus root is absent, builds through cargo, and always prints the unmeasurable
  count. `--self-test` proves the comparison core fail-closed in 10 cases. The driver gained a **tier** column so a
  §4.7 CI-tier doctrine stays registered and meta-checked while the pre-commit hook keeps running only `gate` tier;
  a deferred doctrine prints as `DEFER`, never as absent. `run_ci.sh` now invokes `--all`.
- ID: `CORPUS-CHAIN-CURRENCY.3` · Status: `open` · Goal: close the drift `.1` measured (see the census below).
  Rebuild the downstream chain — `semantic` → `intent` → `adapt` — for every stale document, re-validate, and
  re-measure the FSMGen-strict emitted-`.isf` population, which this rebuild will move. No re-ingest is needed:
  only the evidence stage reads a normalized bundle, so all 79 downstream chains are rebuildable today. Decide the
  `readme` scratch chain's fate in the same leaf (refresh it or drop it) instead of letting a demo artifact
  permanently redden a corpus gate.
- ID: `CORPUS-CHAIN-CURRENCY.2` · Status: `open` · Goal: make bundle retention real — stop routine
  `clean --scope source-normalized` in the refresh routine, state the retention rule in the book's generated-
  artifacts chapter, and record each refresh's bundle as retained so the measurable population grows by one per
  refresh.

## Measured corpus census (`2026-08-10`, `.1`) — the drift `.3` closes

The first full-chain run of the shipped gate. `.0` measured only the **evidence** stage over the 22 documents
with a normalized bundle; the downstream stages had never been re-checked at all.

| Stage | Replayed | Current | Stale | Not persisted | Unmeasurable |
| --- | ---: | ---: | ---: | ---: | ---: |
| `evidence` | 23 | 22 | 1 | 0 | 57 |
| `semantic` | 79 | 14 | 65 | 1 | — |
| `intent` | 79 | 15 | 64 | 1 | — |
| `isf-adapter` | 79 | 65 | 14 | 1 | — |

Root cause of the bulk, read from the code rather than inferred: `CORPUS-COVERAGE.2.43a.i` and `.2.43b` retired
the generic section-phase and whole-statement-gate producers outright — `crates/specforge/src/ir/semantic.rs`
now assigns `let phases = Vec::new();` and `let gates = Vec::new();` unconditionally. Only documents refreshed
after those leaves carry post-retirement artifacts; the other 65 still hold pre-retirement gates/phases (for
example `gates(56->0) phases(22->0)` on the AArch64 external debug guide), and their IntentIR behaviors are
derived from them (`behaviors(78->0)`, `behaviors(523->126)`). Two smaller classes ride along: a
`protocol_states` schema field absent from older SemanticIRs, and per-extractor content deltas at unchanged
cardinality (`decomposition_candidates(14->14)` differing by one `supporting_statement_id`).

The single stale `evidence` document is `readme`, the quick-start demo chain: its promoted markdown *is* the
live `README.md`, so `README_POLICY` edits made its persisted chain unreproducible from its own source.

The 14 stale adapters are computed from stale IntentIRs, so that number will move once `.3` rebuilds upstream.

This is exactly the silent drift ADR 0025 predicted, at a scale `.0` could not see, and it is why `.1` shipped a
gate rather than a one-off census.

### Acceptance Checklist (enforced) — `CORPUS-CHAIN-CURRENCY.0`

- [x] **REPRODUCE / MEASURE** — read-only `evidence --dry-run` census over all 22 rebuildable documents pinned 19
  current and three drifted before any artifact was touched.
- [x] **ROOT CAUSE (WHY + WHERE)** — a repair leaf rebuilds only the documents its own measurement names, so every
  other completed document absorbs later deltas silently; measured on USB 3.2 as timings 102 → 74, actors 18 → 16,
  behaviors 2,705 → 866 under the pre-repair binary.
- [x] **ADDRESSED (verified)** — the three drifted documents rebuilt and validated through four stages; the census
  re-run reports 22/22 current, 0 drifted.
- [x] **NO REGRESSION** — `kg-bench` 156/156; 57/57 emitted ISFs pass FSMGen `--strict --check`; the rebuilt SWD
  gold document keeps every scored surface at 1.000 (serial frame fields 11/11, SWD operations 4/4, protocol
  states 13/13, interface edge timings 1/1) with the known `CSYSPWRUPACK` residual unchanged.
- [x] **GENERICITY (ADR 0006)** — no code changed in this leaf; the rebuild applies existing universal rules.
- [x] **LOCKSTEP** — ADR 0025, this tree, the decision index, and the resume pointer agree before commit.

### Acceptance Checklist (enforced) — `CORPUS-CHAIN-CURRENCY.1`

- [x] **REPRODUCE / MEASURE** — `bash scripts/check_chain_currency.sh` ran the full corpus in `6m58s` and
  produced the four-stage census above from 80 persisted SourceIRs.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/semantic.rs:242` (`let phases = Vec::new();`,
  `CORPUS-COVERAGE.2.43a.i`) and `:249` (`let gates = Vec::new();`, `.2.43b`) retire both generic producers
  unconditionally, so every document not refreshed since those leaves carries pre-retirement artifacts.
- [x] **ADDRESSED (verified)** — the doctrine is registered and executes: `scripts/check_doctrines.sh` reports
  `ALL 6 executed doctrines PASS (7 registered, tier=gate)` with `DEFER CHAIN-CURRENCY`, and the CI-tier run
  exits 1 on the real breach rather than passing silently.
- [x] **NO REGRESSION** — no Rust changed (shell + docs only), so `kg-bench` and the WIRE-BASED-100 golds are
  orthogonal by construction; `scripts/check_doctrines.sh` is green and `check_chain_currency.sh --self-test`
  reports `10/10 passed`.
- [x] **GENERICITY (ADR 0006)** — the check names no document, chip, vendor, or protocol; it enumerates the
  corpus from the tree and reads stage names from the CLI only.
- [x] **LOCKSTEP** — this tree, `DOCTRINE_ENFORCEMENT.md` §5/§8/§10, `TOOLBOX.md` §7.2/§7.2a, the book's
  doctrine-enforcement chapter, the `chain-currency-doctrine` fact card, and the resume pointer agree.

## Verification Log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-10` | read-only currency census | 22 rebuildable documents replayed; 19 current, three drifted on timing surfaces; 58 unmeasurable |
| `2026-08-10` | drift closed | three documents rebuilt and validated; census re-run reports 22/22 current; 27 removed records all prose-as-timing |
| `2026-08-10` | no regression | KG 156/156; 57/57 FSMGen strict; SWD gold surfaces all 1.000 after rebuilding its document |
| `2026-08-10` | `CHAIN-CURRENCY` shipped | `--self-test` 10/10; driver reports 6 executed PASS / 7 registered with `DEFER CHAIN-CURRENCY`; full run `6m58s`, exit 1 |
| `2026-08-10` | full-chain census | evidence 22/23 current (57 unmeasurable); semantic 14/79; intent 15/79; isf-adapter 65/79 — the drift `.3` owns |

## Commit Log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-CHAIN-CURRENCY` ownership | `8475900d` — `CORPUS-CHAIN-CURRENCY — track measured persisted-chain currency` |
| `CORPUS-CHAIN-CURRENCY.0` completion | `CORPUS-CHAIN-CURRENCY.0 — decide currency policy and close measured drift` |
| `CORPUS-CHAIN-CURRENCY.1` completion | `CORPUS-CHAIN-CURRENCY.1 — gate persisted-chain currency as a CI-tier doctrine` |
