# CORPUS-CHAIN-CURRENCY: prove, not assume, that every persisted chain matches the current binary

## Metadata

- Tree ID: `CORPUS-CHAIN-CURRENCY`
- Status: `active` (`2026-09-14`; `.0`-`.5` complete, **`.6` open** — probe semantic and intent totally at gate tier, 66 s)
- Roadmap lane: `R15e`/`R16` corpus digestion (sibling of `CORPUS-COVERAGE`)
- Created: `2026-08-10`
- Last updated: `2026-09-14`
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
- ID: `CORPUS-CHAIN-CURRENCY.3` · Status: `done` (`2026-08-10`, DATA/DOC) · Goal: close the drift `.1` measured.
  Rebuilt `semantic` → `intent` → `adapt --target isf` for all 79 chains from their unchanged persisted
  EvidenceIR (2m32s, zero failures) and re-validated the same 90 artifacts that carried validation state, so the
  reported population neither grew nor shrank. No re-ingest: only the evidence stage reads a normalized bundle.
  The two non-corpus scratch chains were removed with `clean --scope document` — `readme` (its promoted markdown
  *is* the live `README.md`, so every `README_POLICY` edit would redden a corpus gate) and `spec` (residue whose
  source was a deleted `.project-data/tmp` file, therefore permanently unmeasurable). The gate is now GREEN.
- ID: `CORPUS-CHAIN-CURRENCY.2` · Status: `done` (`2026-08-10`, CODE/DATA/DOC) · Goal: make bundle retention
  real — stop routine `clean --scope source-normalized` in the refresh routine, state the retention rule in the
  book's generated-artifacts chapter, and record each refresh's bundle as retained so the measurable population
  grows by one per refresh. Shipped: retention is **declared and gated**, not conventional. The new schema-closed
  `doctrine/chain_currency/retained_bundles.json` names the exact 22 retained document keys, and
  `check_chain_currency.sh` gained a second leg that compares that declaration with the bundles on disk. It fails
  closed both ways — a declared bundle that vanished is an unauthorised reclamation, a bundle no leaf declared is
  a refresh that never recorded what it kept — so the measurable population can only grow deliberately, one
  refresh at a time. Deliberate reclamation stays possible as a `reclamations` record naming its owning leaf,
  date, and reason. Six new fail-closed self-test cases take the check to 16/16. `TOOLBOX.md` §7.6 now tells a
  refresh **never** to run `clean --scope source-normalized` as routine, and the book's generated-artifacts
  chapter states the retention rule, its cost, and the declaration.

- ID: `CORPUS-CHAIN-CURRENCY.4` · Status: `done` (`2026-09-14`, PROBE/DOC) · Children: `.5` · **Both
  questions answered by running the thing: what is drifted, and what it costs to know.**
  **Cost, measured on this machine with a warm build:** `check_chain_currency.sh` **28m00s**;
  `check_proof_seal_currency.sh --total` **18m45s**. Together ≈ 47 minutes, which is what "`--all` did
  not finish in 50 minutes" actually was — a number this tree can now stop restating as a mystery.
  Neither is affordable per commit, both are affordable per push, and directive 16 already puts the
  full gate at the push boundary. **So CHAIN-CURRENCY stays CI-tier and this leaf does not widen it.**
  **Drift, measured:** evidence **23 of 24** current, semantic **25 of 27**, intent **25 of 27**,
  isf-adapter **25 of 27**; retention exactly the declared 24 bundles.

  | document | stage | what |
  | --- | --- | --- |
  | `um10204…i2c` | evidence | CONTENT stale — `signal_constraints` 9 → 3, `fact_provenance` 21 → 15, 13 `conditional_rules` renumbered; everything downstream then blocked |
  | `ihi0024_e…apb` | semantic | CONTENT stale — `interface_signal_conflicts` 0 → 1 and `PADDRCHK` loses its width; the persisted artifact is **refused** by `intent` and by `isf-adapter` |

  Both predate this leaf and both are attributed: I2C's delta reproduces at `1ada364a`, APB-e's at
  `956fbcce`, each from its own byte-unchanged upstream artifact. APB-e's is not a bookkeeping gap but a
  precision regression that has not shipped only because its document was never rebuilt — routed to
  `SIGNAL-DECLARATION-ROW-DROP.4c`, which must land before APB-e is rebuilt.

  **The real finding is the third one, and it is about the gate rather than the corpus.** A
  load-REFUSING artifact is far cheaper to detect than a content-stale one, and a gate-tier doctrine
  already exists for exactly that — `PROOF-SEAL-CURRENCY`, which probes the product's own canonical
  loader. It passed on every run today while `intent` refuses APB-e's SemanticIR. Not a bug: it probes
  **one representative per distinct seal per stage**, and the census says there is **1 distinct seal
  across all 27 artifacts at every stage**. Its sample size is therefore **1 in 27**, and its own
  `--total` mode — which probes all of them — catches both refusals in the same run, with the right
  diagnostic (*"this is NOT the stale-seal diagnostic … find out why the loader refuses this
  artifact"*).
  The seal key does not vary with what the loader actually verifies. That is `.5`.
  Verification: `bash scripts/check_chain_currency.sh` and
  `bash scripts/check_proof_seal_currency.sh --total`, both run detached to completion with `time`;
  per-document attribution by `semantic --dry-run` / `evidence --dry-run` at `956fbcce` and `1ada364a`
  against the persisted artifacts. Read-only throughout: both checks replay with `--dry-run` and write
  no artifact, and retention was 24 before and after.
  Commit: `CORPUS-CHAIN-CURRENCY.4`

- ID: `CORPUS-CHAIN-CURRENCY.5` · Status: `done` (`2026-09-14`, PROBE/DOC) · Children: `.6` ·
  **The key is not the lever, and the leaf's own proposal was refuted by the first measurement it
  made.** `.4` proposed folding the recorded derivation topology into the seal-census key so a refusing
  document would earn its own probe. Measured over the whole proof-carrying stratum, that key gives
  **27 distinct keys for 27 documents** at evidence, semantic and intent alike — and so does the
  narrower key over ROOT derivations only (39/89/139 root ids per stage, every one of them
  document-specific, because `output_sha256` and `inputs_sha256` are digests over the document's own
  content). A topology-bearing key *is* `--total`, which is 18m45s, exactly as this leaf's own warning
  said it must not become.
  **The lever is the per-stage TIER, and the measurement is unambiguous.** `--total`'s cost is not
  spread evenly: an accepted `intent --dry-run` takes **1.2 s** and a refusal **0.24 s**, because a
  refusal stops at the loader. A TOTAL probe — every one of the 27, no sampling — costs

  | stage probed totally | wall clock | refusals found |
  | --- | ---: | --- |
  | semantic (27 evidence artifacts) | **30.3 s** | I2C |
  | intent (27 semantic artifacts) | **35.5 s** | APB-e, I2C |
  | **both** | **66 s** | **every refusal the corpus currently has** |

  The 18m45s belongs to the **evidence** and **source-ir** probes, which replay extraction from the
  normalized bundle — a different order of work from loading a persisted artifact and verifying its
  proof. Sampling is right there and wrong at semantic and intent.
  Verification: read-only. Distinct-key counts computed from the persisted `proof_ledger`s
  (`RegisteredDerivation` premises, full and `.root`-only); per-stage probe cost timed with `time` over
  every current-stratum artifact; both refusals reproduced in that run.
  Commit: `CORPUS-CHAIN-CURRENCY.5`

- ID: `CORPUS-CHAIN-CURRENCY.6` · Status: `pending` (opened `2026-09-14` by `.5`) · Goal: **probe the
  semantic and intent stages TOTALLY at gate tier; keep source-ir and evidence sampled.** 66 seconds
  buys the property the sampled tier is documented as wanting and currently cannot deliver: no
  load-refusing artifact reaches a commit. Today's two would both have been caught on the commit that
  caused them rather than days later by a detached sweep.
  **State the cost rather than hide it**: the gate gains ~64 s (the two stages go from 1 probe each to
  27 each). Directive 16 asks ordinary commits to run the checks that prove the main functionality still
  behaves, and a wire-gold artifact its own consumer refuses is squarely that.
  The doctrine's self-tests are the load-bearing part of this leaf, not the tier constant: **self-test 17
  asserts that the sampled probe MISSES a divergent same-seal document**, which is the behaviour this
  change removes at two stages and keeps at the other two. It must become a per-stage assertion rather
  than be deleted, or the check will have lost the control that documents its own limit.
  Prerequisite: none. Verification: the new tier observed RED against today's two refusing artifacts and
  GREEN once they are rebuilt; the gate's runtime measured before and after; every existing self-test
  re-run, with 17 restated per stage; `PROOF-SEAL-TOTAL` left registered at CI tier and unchanged.
  Commit: pending

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

## Closing census (`2026-08-10`, `.3`) — the corpus after the rebuild

| Stage | Replayed | Current | Stale | Not persisted | Unmeasurable |
| --- | ---: | ---: | ---: | ---: | ---: |
| `evidence` | 22 | 22 | 0 | 0 | 56 |
| `semantic` | 78 | 78 | 0 | 0 | — |
| `intent` | 78 | 78 | 0 | 0 | — |
| `isf-adapter` | 78 | 78 | 0 | 0 | — |

**Product-status change: the emitted-`.isf` population is 57 → 44, and all 44 are FSMGen `--strict --check
--json` clean (0 diagnostics).** The cause is measured, not inferred: exactly 14 documents' SemanticIR
`interfaces[]` collapsed from a heuristic bulk (929, 737, 530, 480, 426, 211, 195, 135, 131, 121, 82, 78, 74, 7)
to **zero** under current authority, and all 14 now block on `no signals declared in interface` and emit
nothing — a 14/14 correspondence with zero exceptions. Each `.2.4x` refresh removed stale heuristic interfaces
for its **own** document only; every un-refreshed document kept its false interface set, and its `.isf`, until
this rebuild. The gate/phase retirements are exonerated: their own fact cards' "no adapter renderability value"
claim survives this measurement intact.

Honest limits of this measurement:

- The **per-document pre-rebuild emitted set was not snapshotted**, so the single offsetting gain implied by
  57 − 14 = 43 vs the measured 44 cannot be attributed to a named document. That state was in any case a
  mixture of code vintages and irreproducible by construction — which is the condition this doctrine ends.
  From now on the before-state is always reproducible, because the gate reproduces it.
- 56 documents remain **unmeasurable at the evidence stage** until their normalized bundles are backfilled at
  each document's own refresh (ADR 0025 decision 3). Every later stage is measured for the whole corpus.
- Two documents (`opencapi_25gbps_phy_mechanical_spec_v10`, `um11732_v3_2022_02_17_i2s_bus_specification`) have
  declared signals yet block on `no behavioral content`. That is the documented `R6-ISF-ADAPTER.4` policy —
  `assess_isf_renderability` counts only `temporal_rules`, `conditional_rules`, `signal_constraints`, and
  `control_blocks`, never `behaviors`/`constraints`/`temporal_invariants`. `um11732` was already fully current
  before the rebuild, proving the pattern pre-dates this leaf; it is a standing lowering-recall question, not a
  regression, and it is surfaced rather than closed here.

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

### Acceptance Checklist (enforced) — `CORPUS-CHAIN-CURRENCY.3`

- [x] **REPRODUCE / MEASURE** — the `.1` gate measured the drift (semantic 14/79, intent 15/79, isf-adapter
  65/79 current) before anything was rebuilt.
- [x] **ROOT CAUSE (WHY + WHERE)** — 14/14 correspondence between `interfaces(N->0)` in the `.1` semantic
  replay and `no signals declared in interface` in the rebuilt adapter; blocking policy read at
  `crates/specforge/src/ir/adapters.rs` `assess_isf_renderability`.
- [x] **ADDRESSED (verified)** — the closing census reports 22/22 evidence, 78/78 semantic, 78/78 intent, and
  78/78 isf-adapter current, with all 78 emitted-`.isf` checks passing; `check_chain_currency.sh` exits 0.
- [x] **NO REGRESSION** — no Rust changed, so `kg-bench` and the WIRE-BASED-100 golds are orthogonal by
  construction and every EvidenceIR is byte-unchanged (the rebuild never touched that stage); all 44 emitted
  ISFs pass `fsmgen --strict --check --json` with 0 diagnostics, and `scripts/check_doctrines.sh` is green.
- [x] **GENERICITY (ADR 0006)** — no code changed; the rebuild applies existing universal rules uniformly.
- [x] **LOCKSTEP** — this tree, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, the
  `corpus-wide-interface-authority-rebuild` fact card, and the resume pointer agree before commit.

### Acceptance Checklist (enforced) — `CORPUS-CHAIN-CURRENCY.2`

- [x] **REPRODUCE / MEASURE** — the retained set was measured, not assumed, before anything was declared: 22 of
  78 documents resolve an existing `promoted_markdown_path`, exactly matching the 22 `normalized/` directories
  on disk and the gate's own 56-unmeasurable count. Cost re-measured at 1.4 GB against 3.4 TB free.
- [x] **ROOT CAUSE (WHY + WHERE)** — measurability was a side effect of whatever cleanup happened to have been
  run: the `2026-07-05` bulk reclamation removed every bundle then existing, and nothing recorded or re-checked
  retention afterwards, so the population that `CHAIN-CURRENCY` can measure was untracked state. The refresh
  routine's own guidance (`TOOLBOX.md` §7.6) still presented `clean --scope source-normalized` neutrally.
- [x] **ADDRESSED (verified)** — the full gate reports `retention: 22 normalized bundle(s) on disk — exactly
  the declared retained set` alongside 22/78/78/78 current stages and exits 0. Fail-closed behaviour is proven
  against the real declaration, not only fixtures: dropping one measured key reports `opencapi_afu_address_
  space_usage is declared retained but its normalized bundle is absent — an unauthorised reclamation`, adding an
  undeclared key reports `retains a normalized bundle that no leaf declared`, and removing the document entirely
  reports `is no longer a corpus document`.
- [x] **NO REGRESSION** — the change is shell + data + docs; no Rust changed, so `kg-bench` and the
  WIRE-BASED-100 golds are orthogonal by construction. `--self-test` is 16/16, `scripts/check_doctrines.sh` is
  green, and the full four-stage census is byte-unchanged from `.3`.
- [x] **GENERICITY (ADR 0006)** — the retention leg names no chip, vendor, or protocol; document keys are data
  in a declaration file, and the check derives the measured set from the corpus tree and each SourceIR's own
  `promoted_markdown_path`.
- [x] **LOCKSTEP** — this tree, `DOCTRINE_ENFORCEMENT.md` §10, `TOOLBOX.md` §7.2a/§7.6, the book's
  generated-artifacts / doctrine-enforcement / SourceIR chapters, both retention fact cards, the corpus tree's
  corrected census, and the resume pointer agree before commit.

## Current Frontier

1. `CORPUS-CHAIN-CURRENCY.6` — raise the semantic and intent probes to TOTAL at gate tier (66 s
   measured), leaving source-ir and evidence sampled. The doctrine's self-test 17 — "the sampled probe
   MISSES a divergent same-seal document" — must become a per-stage assertion rather than be deleted.
2. Rebuilding the two drifted documents is **not** this tree's next step. I2C's rebuild is unblocked but
   its normalized bundle is reclaimed, so it needs a re-ingest rather than a replay; APB-e's is blocked
   by `SIGNAL-DECLARATION-ROW-DROP.4c`, and rebuilding it today would publish that regression into a
   wire-gold document's chain.

## Verification Log

- `2026-09-14` — `.5`. Read-only throughout. Distinct-key counts computed from every current-stratum
  artifact's persisted `proof_ledger`, over its `RegisteredDerivation` premises: the full
  `(derivation_id, output_sha256, inputs_sha256)` multiset gives **27 distinct keys for 27 documents** at
  evidence, semantic and intent; restricting to `.root` derivations (39 / 89 / 139 ids) gives the same
  27, because each root's digests are over that document's own content. Probe cost timed with `time`
  over all 27: `semantic --dry-run` **30.3 s** total (1 refusal, I2C), `intent --dry-run` **35.5 s**
  total (2 refusals, APB-e and I2C); a single accepted `intent --dry-run` is **1.2 s** and a refusal
  **0.24 s**. No artifact written, rebuilt or mutated.

- `2026-09-14` — `.4`. `bash scripts/check_chain_currency.sh` run detached to completion: **28m00s real**
  (26m49s user), exit 1, evidence 23/24 current + 54 unmeasurable, semantic 25/27, intent 25/27,
  isf-adapter 25/27, retention exactly 24 before and after.
  `bash scripts/check_proof_seal_currency.sh --total`: **18m44s real**, exit 1 — it catches BOTH refusals
  with the correct non-stale-seal diagnostic, and its census reports **1 distinct seal per stage across
  27 artifacts** at source-ir, evidence, semantic, intent and isf-adapter alike, which is what makes the
  sampled tier a 1-in-27 sample.
  Attribution, each from its own byte-unchanged upstream artifact: I2C's evidence delta reproduces at
  `1ada364a`; APB-e's semantic delta reproduces at `956fbcce`. Neither is this session's doing.
  Live confirmation of the gate gap: `specforge intent generated/semantic_ir/<apb-e>/semantic_ir.json
  --dry-run` is REFUSED by the current build while `PROOF-SEAL-CURRENCY` (sampled, gate tier) passes in
  the same working tree; AHB and AXI both load.
  Read-only: every replay is `--dry-run`, no artifact was written, rebuilt or mutated.

| Date | Boundary | Result |
| --- | --- | --- |
| `2026-08-10` | read-only currency census | 22 rebuildable documents replayed; 19 current, three drifted on timing surfaces; 58 unmeasurable |
| `2026-08-10` | drift closed | three documents rebuilt and validated; census re-run reports 22/22 current; 27 removed records all prose-as-timing |
| `2026-08-10` | no regression | KG 156/156; 57/57 FSMGen strict; SWD gold surfaces all 1.000 after rebuilding its document |
| `2026-08-10` | `CHAIN-CURRENCY` shipped | `--self-test` 10/10; driver reports 6 executed PASS / 7 registered with `DEFER CHAIN-CURRENCY`; full run `6m58s`, exit 1 |
| `2026-08-10` | full-chain census | evidence 22/23 current (57 unmeasurable); semantic 14/79; intent 15/79; isf-adapter 65/79 — the drift `.3` owns |
| `2026-08-10` | `.3` rebuild | 79 chains rebuilt in `2m32s`, 0 failures; 90 previously-validated artifacts re-validated, 0 failures |
| `2026-08-10` | `.3` closing census | 22/22 evidence · 78/78 semantic · 78/78 intent · 78/78 isf-adapter current; gate exits 0 |
| `2026-08-10` | `.3` emitted ISFs | 57 → 44 emitted; 44/44 FSMGen `--strict --check --json` clean; 14/14 loss correspondence with `interfaces(N->0)` |
| `2026-08-10` | `.2` retention declared | 22 retained keys measured from `promoted_markdown_path`, matching the 22 `normalized/` directories exactly; declaration written and gated |
| `2026-08-10` | `.2` gate re-run | 78 documents: 22/78/78/78 current, 78 emitted `.isf` bodies checked, `retention: 22 … exactly the declared retained set`, exit 0 |
| `2026-08-10` | `.2` fail-closed | `--self-test` 16/16; three live negatives against the real declaration (vanished bundle, undeclared bundle, dropped document) each named and rejected |
| `2026-08-10` | `.2` corpus-record audit | `.2.50`'s "184-file / 52,570,034-byte bundle" measured the document root; the bundle is 183 files / 51,754,156 bytes and the 815,878-byte difference is exactly its `source_ir.json`. Referential integrity confirmed complete — nothing lost. `.2.48`/`.2.49` records match their bundles exactly |

## Commit Log

- `.4` — `CORPUS-CHAIN-CURRENCY.4`.
- `.5` — `CORPUS-CHAIN-CURRENCY.5`.

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-CHAIN-CURRENCY` ownership | `8475900d` — `CORPUS-CHAIN-CURRENCY — track measured persisted-chain currency` |
| `CORPUS-CHAIN-CURRENCY.0` completion | `CORPUS-CHAIN-CURRENCY.0 — decide currency policy and close measured drift` |
| `CORPUS-CHAIN-CURRENCY.1` completion | `ddc2f798` — `CORPUS-CHAIN-CURRENCY.1 — gate persisted-chain currency as a CI-tier doctrine` |
| `CORPUS-CHAIN-CURRENCY.3` completion | `CORPUS-CHAIN-CURRENCY.3 — rebuild the corpus chain to currency` |
| `CORPUS-CHAIN-CURRENCY.2` completion | `CORPUS-CHAIN-CURRENCY.2 — declare and gate normalized-bundle retention` |
