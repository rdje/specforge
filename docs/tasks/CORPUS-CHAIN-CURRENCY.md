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
| Rebuildable documents (normalized bundle present) | 22 | replayable read-only |
| Persisted chain already current | 19 | no section differs |
| Persisted chain would change on rebuild | 3 | timing surfaces only |
| Non-rebuildable documents | 58 | **not measurable** without re-ingest |

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
  and validated through all four stages, taking the rebuildable population to **22/22 current**.
  All 27 records the rebuild removed are prose cells misread as timing parameters — a JTAG/SWD ACK-response table
  (`parameter_name: "Read"`, `unit: "Capture read data."`), OpenCAPI crosstalk rows whose `max_value` repeats the
  description, and OpenCAPI configuration-limit rows. Nothing real was lost and nothing was added.
- ID: `CORPUS-CHAIN-CURRENCY.1` · Status: `open` · Goal: ship `scripts/check_chain_currency.sh` per the §4 check
  contract and register it as the `CHAIN-CURRENCY` doctrine. It must replay every rebuildable document against its
  persisted chain, fail closed on any difference, skip explicitly when `generated/` is absent, build through cargo
  so freshness is decided by the toolchain rather than an mtime heuristic, and always print the unmeasurable count.
  Add the `DOCTRINE_ENFORCEMENT.md` §10 row in lockstep.
- ID: `CORPUS-CHAIN-CURRENCY.2` · Status: `open` · Goal: make bundle retention real — stop routine
  `clean --scope source-normalized` in the refresh routine, state the retention rule in the book's generated-
  artifacts chapter, and record each refresh's bundle as retained so the measurable population grows by one per
  refresh.

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

## Verification Log

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-10` | read-only currency census | 22 rebuildable documents replayed; 19 current, three drifted on timing surfaces; 58 unmeasurable |
| `2026-08-10` | drift closed | three documents rebuilt and validated; census re-run reports 22/22 current; 27 removed records all prose-as-timing |
| `2026-08-10` | no regression | KG 156/156; 57/57 FSMGen strict; SWD gold surfaces all 1.000 after rebuilding its document |

## Commit Log

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-CHAIN-CURRENCY` ownership | `8475900d` — `CORPUS-CHAIN-CURRENCY — track measured persisted-chain currency` |
| `CORPUS-CHAIN-CURRENCY.0` completion | `CORPUS-CHAIN-CURRENCY.0 — decide currency policy and close measured drift` |
