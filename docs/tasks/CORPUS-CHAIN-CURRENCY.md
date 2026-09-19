# CORPUS-CHAIN-CURRENCY: prove, not assume, that every persisted chain matches the current binary

## Metadata

- Tree ID: `CORPUS-CHAIN-CURRENCY`
- Status: `active` (`2026-09-20`; `.0`-`.9` complete and the corpus still CURRENT. `.10`/`.10a`/`.10b`/`.10c` closed the re-ingest question; `.11` is open — the 51 UNMEASURABLE chains are still being read as evidence)
- Roadmap lane: `R15e`/`R16` corpus digestion (sibling of `CORPUS-COVERAGE`)
- Created: `2026-08-10`
- Last updated: `2026-09-20`
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
  **CORRECTED `2026-09-14` by `.7`: these timings were taken with `target/release/specforge`, and the
  check builds and probes with `target/debug/specforge`.** A debug probe is ~14 s where a release probe
  is 1.2 s, so the activated gate measures **13m01s**, not 66 s. The conclusion that the scope is a
  per-stage question survives; the number that made it look affordable at gate tier does not, and the
  profile is now `.8`.
  Verification: read-only. Distinct-key counts computed from the persisted `proof_ledger`s
  (`RegisteredDerivation` premises, full and `.root`-only); per-stage probe cost timed with `time` over
  every current-stratum artifact; both refusals reproduced in that run.
  Commit: `CORPUS-CHAIN-CURRENCY.5`

- ID: `CORPUS-CHAIN-CURRENCY.6` · Status: `done` (`2026-09-14`, CODE — **mechanism shipped INERT**) ·
  Children: `.7` · Goal: **probe the semantic and intent stages TOTALLY at gate tier; keep source-ir
  and evidence sampled.**
  Shipped: `probe_scope_for <stage>` makes the probe scope a per-stage decision, `--total` still forces
  total everywhere, and the summary lines report per stage which scope was used. 66 seconds buys the
  property the sampled tier is documented as wanting and cannot currently deliver.
  **It ships with the stage set EMPTY, and that is the leaf's real finding.** Turning it on was tried
  and measured: the gate then FAILS on the live corpus, naming APB-e and I2C at both stages — correctly,
  because both artifacts genuinely are refused by their own consumers. A gate that fails closed over a
  known-broken corpus is right and is also **unlandable**: it blocks every commit until the repair, and
  the repair is blocked twice over — APB-e by `SIGNAL-DECLARATION-ROW-DROP.4c` (rebuilding it today
  publishes that regression into a wire-gold chain) and I2C by a reclaimed normalized bundle (it needs a
  re-ingest, not a replay).
  So the mechanism lands with its controls and the activation waits for the repair, rather than the
  contract being widened to accommodate a failure. **Activation is one constant**
  (`TOTAL_PROBE_STAGES` → `'semantic intent'`), and both halves are already asserted.
  **The self-tests are where this leaf's weight is.** Self-test 17 — "the sampled probe MISSES a
  divergent same-seal document" — is restated **per stage** rather than deleted: its stub now refuses
  only at `source_ir/`, so it still proves the blindness the sampled tier pays for. New **17b** proves
  the other half at a TOTAL stage with the same seal and the same stub shape, and the mini corpus gained
  a semantic stage so the per-stage decision is exercisable at all. Observed RED: with the stage set
  emptied, 17b fails with *"a TOTAL stage passed over a document its own loader refuses"*.
  Measured: self-tests **20/20**; the default gate run is **15.9 s** and green; with the stage set
  activated it is ~66 s longer and RED on exactly the two documents `.4`/`.5` found.
  Verification: see the acceptance checklist below.
  Commit: `CORPUS-CHAIN-CURRENCY.6`

- ID: `CORPUS-CHAIN-CURRENCY.7` · Status: `done` (`2026-09-14`, DATA/DOC) · Children: `.8` · **Both
  documents repaired and the corpus is CURRENT; the activation is held for a reason that has nothing to
  do with the corpus.**
  **Step 2 — APB-e rebuilt.** See "`.7` step 2" below.
  **Step 3 — I2C rebuilt, and this step's own premise was wrong.** `.4`, `.6` and this node all recorded
  that I2C's normalized bundle was *reclaimed* and that it therefore needed a RE-INGEST rather than a
  replay. Reading `doctrine/chain_currency/retained_bundles.json` shows **I2C is one of the declared 24
  retained bundles**, and its bundle is on disk. The chain-currency sweep had said so all along — it
  listed I2C under `evidence: 24 replayed … 1 stale`, i.e. *replayed and content-stale*, not under the
  54 unmeasurable. The repair was a plain chain rebuild, not a re-ingest, and the retention declaration
  never moved.
  Rebuilt in the documented order with exactly one `validate` per artifact, upstream-first:
  `evidence` → `validate` → `semantic` → `validate` → `intent` → `validate` → `adapt --target isf`,
  with a `--dry-run` before each write to prove the loader accepts the upstream. Pre-rebuild artifacts
  at `generated/preserved/CORPUS-CHAIN-CURRENCY.7/pre-rebuild-i2c/` (8 files, `1a3a49a2…50cf`).
  **What it publishes is a precision WIN, not a repair of damage**: `signal_constraints` **9 → 3** and
  `fact_provenance` 21 → 15. The eight removed records are exactly the shape the `.3k` series refused
  and I2C never received — *"Every byte put on the USDA line must be eight bits long"* minted
  `USDA must_be_high`; *"The UFm I²C-bus is a 2-wire push-pull serial bus that operates from DC to
  5 MHz"* minted `USCL must_be_stable`; *"If the data line (SDA) is stuck LOW…"* minted
  `SDA must_be_low` from a fault condition. All four stages then replay CONTENT SAME.
  **Step 4 — measured, and held.** With the stage set forced on, the probe reports **27 of 27 accepted
  at semantic and at intent, exit 0**: the corpus obstacle is gone. The activation is still held,
  because switching it on measured **13m01s** against 15.9 s for the sampled default — and the reason
  is a binary this leaf had not looked at. That is `.8`.
  Verification: see the acceptance checklist below.
  Commit: `CORPUS-CHAIN-CURRENCY.7`

- ID: `CORPUS-CHAIN-CURRENCY.8` · Status: `done` (`2026-09-14`, CODE/DOC) · Children: `.9` · **The
  profile moved to `release`, and the three things that made the trade look hard were all measured
  false.** See the measurement table under "`.8` — both profiles, cold and warm" below.
  **The cold objection does not exist.** A cold release build of this workspace — empty target tree —
  is **36.1 s**, not minutes. The dependency set is six crates, so there is no cold cliff to pay for;
  cold debug is 21.0 s, and the *release* tree is the smaller one at **234 MB** against debug's 1.1 GB.
  **And it is not paid on a clean checkout or a CI runner at all.** All three corpus-replay entrypoints
  skip on an absent `generated/` **before** they reach the build — measured at **0.031 s with no cargo
  invocation** — because the corpus is untracked and those machines have none. The leaf's own premise,
  "a cold release build … would land on every clean checkout and CI runner", was wrong twice.
  **What actually decides it is that a build is a FIXED cost and a probe is a PER-DOCUMENT one**, so the
  debug argument inverts with the probe count rather than being right or wrong in general. It was right
  at four probes and wrong at fifty-four. The crossover is small: a release build costs ~27 s more than a
  debug one after a real core edit, and a single large-document probe saves ~50 s, so **two large
  documents pay for the build**.
  **The verdict does not move, and that was measured rather than argued.** Both profiles report 27 of 27
  accepted at semantic and at intent in the same working tree; AXI's **43,419,318-byte** `intent
  --dry-run` is byte-identical between them (same SHA-256), so is Wishbone's 10,496,700-byte `semantic
  --dry-run`, and a legacy artifact's refusal is the same string. That is what the code predicts:
  verification is digest comparison and ordered-map lookup (`verify_ledger`/`validate_claim`,
  `crates/specforge/src/ir/derivation.rs`), the workspace has **no** `cfg(debug_assertions)`, and the
  only two `debug_assert!`s on the proof path assert COMPILE-TIME CONSTANTS. Debug assertions and
  overflow panics stay `cargo test`'s job, in the dev profile, at CI.
  **The scope is three entrypoints, not one, and that is the `.11` contract rather than a widening.**
  `check_chain_currency.sh`, `check_proof_seal_currency.sh` and `rebuild_stage_cascade.sh` all replay the
  persisted corpus against the current build, and each carried its own copy of `cargo build … --bin
  specforge` + `target/debug/specforge`. A gate and a remedy that disagree about **which loader answers
  for them** leave a debt no compliant work can clear — the reason the seal read and the chain table
  already live in `scripts/lib/proof_seal_scan.sh`. The binary was the third shared predicate and the
  only one still copied out. It is now `scripts/lib/corpus_replay_binary.sh`, which owns the profile, the
  measurement table, and the no-verdict-change evidence.
  **A rejected third option, measured rather than dismissed**: `profile.dev.opt-level=1` plus
  `profile.dev.package."*".opt-level=3` probes AXI in **7.40 s** — within 14% of release — while keeping
  debug assertions on. It is not taken here because it changes the build profile of the whole workspace
  (every `cargo test`, `clippy` and hook invocation; 12.6 s incremental against debug's 5.3 s; a 1.5 GB
  tree) to buy an assertion class this check does not exercise. It is recorded under `.8` rather than
  discarded, because it is the right answer to a **different** question — workspace-wide dev build
  speed — and the measurement is done.
  Verification: see the acceptance checklist below.
  Commit: `CORPUS-CHAIN-CURRENCY.8`

- ID: `CORPUS-CHAIN-CURRENCY.9` · Status: `done` (`2026-09-14`, CODE/DOC) · **The per-stage TOTAL probe
  is ON, and the tier decision was measured on the whole gate rather than argued from a component.**
  `TOTAL_PROBE_STAGES` now defaults to `'semantic intent'`. Measured before and after on the same tree:
  the gate-tier driver **4m13.0s → 5m30.1s**, with a second post-change sample at 5m24.5s, so **+72-77 s,
  about +29%**; this check itself **7.4 s →
  1m12.7s**. Both obstacles `.6` named are gone and both were closed by measurement — the corpus by
  `.7` (27 of 27 accepted at semantic and at intent), the cost by `.8` (the same sweep was 12m57.9s at
  the debug profile).
  **What that minute-and-a-bit buys is a class the sampled tier is documented as unable to see, and its price
  has already been paid once in this repository.** At `48def695` all 27 evidence artifacts carried one
  seal, so one probe ran and the gate reported green while the canonical loader refused **4 of the
  27 — every wire-bearing specification in the corpus — for three commits**, with the scoring oracle
  reading them (`SIGNAL-DECLARATION-ROW-DROP.1b`). A 1-in-27 sample cannot see that. Source-ir and
  evidence stay sampled, because their probes replay extraction and are the expensive ones; the check
  still prints that it is blind there, and `--total` (1m59.2s, CI tier) still closes it.
  **The leaf's real addition is self-test 21, because nothing tested the DEFAULT.** `.6`'s controls 17
  and 17b prove the *mechanism* — 17b passes the stage set in explicitly — so the shipped value itself
  was unguarded, and an edit that quietly emptied it would have restored a 1-in-27 sample under a green
  gate. Case 21 passes no override and requires the default to catch a divergent same-seal document at
  `semantic`. Observed RED: with `SPECFORGE_PROOF_SEAL_TOTAL_STAGES=''` the suite reports **20/21**,
  failing on exactly *"self-test 21: the DEFAULT stage set passed over a document its own loader
  refuses"* and on nothing else.
  **Two stale self-descriptions in the script were corrected rather than left standing**: its "what it
  proves" header still claimed one probe per distinct seal "is not a sample" — an argument the same
  file refutes eighty lines later — and the fully-sampled summary still told the reader the tier
  "ships INERT until `CORPUS-CHAIN-CURRENCY.6` activates it". That branch now says the stage set must
  have been emptied for this run and that the check is knowingly blind.
  Verification: see the acceptance checklist below.
  Commit: `CORPUS-CHAIN-CURRENCY.9`

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

## `.7` step 2 — APB-e rebuilt (`2026-09-14`)

The first of the two drifted documents is repaired. Its EvidenceIR was already current and
byte-unchanged, so the rebuild started at `semantic` and ran the documented order with **exactly one
`validate` per artifact, strictly upstream-first**: `semantic` → `validate` → `intent` → `validate` →
`adapt --target isf`. Pre-rebuild artifacts are held at
`generated/preserved/CORPUS-CHAIN-CURRENCY.7/pre-rebuild/` (6 files, digest
`8d1b9e13f605d81c45525433d7f26547c02f120c06cd6bafdfc3e7d5c588791f`).

**What moved, and it is exactly what `.4` predicted and `SIGNAL-DECLARATION-ROW-DROP.4c` adjudicated:**

| section | before | after |
| --- | ---: | ---: |
| `interface_signal_conflicts` | 0 | **1** (`PADDRCHK`, `width_mismatch`, `ADDR_WIDTH/8` against `ceil(ADDR_WIDTH/8)`) |
| `actor_ports` | 64 | 64 — two records lose `width_hint` |
| `interfaces` | 35 | 35 — one record changes |
| `signal_connectivity` | 32 | 32 — one record changes |

**What did NOT move, verified rather than assumed:**

- the emitted `.isf` `source_text` is **byte-identical** (4,029 bytes before and after), and
  `adapter.json` has **no** moved section outside the proof surfaces — which is the measurement
  `SIGNAL-DECLARATION-ROW-DROP.4c` was corrected by, now confirmed through the real rebuild rather than
  by reading the pre-rebuild artifact;
- the APB wire gold is unchanged: `signal_constraint` 1.000 and, source-tolerant + filtered,
  `actor_signal_relation` 1.000, before and after;
- retention is exactly the declared **24** bundles;
- the default (inert) `PROOF-SEAL-CURRENCY` gate passes.

**The repair is measured at the gate that found it.** With `SPECFORGE_PROOF_SEAL_TOTAL_STAGES='semantic
intent'`, the probe goes from **25 of 27** accepted at each of those stages to **26 of 27**, and the
single remaining refusal at both is `um10204…i2c` — step 3.

Every stage now replays clean for this document: `semantic --dry-run`, `intent --dry-run` and
`adapt --dry-run` each reproduce the persisted artifact exactly.

**One thing left as-is and stated rather than hidden**: `generated/adapters/isf/<apb-e>/validation_report.json`
predates the rebuild. `specforge validate` accepts IR artifacts and not an adapter, the documented
rebuild order ends at `adapt`, and the chain-currency comparison excludes `validation_reports` by
construction — so nothing is stale by any gate's definition, but the file is older than the artifact
beside it and a future leaf should decide whether an adapter's report has an owner.

## `.7` step 3 — I2C rebuilt, and this tree's own premise corrected (`2026-09-14`)

Three of this tree's nodes recorded that I2C's normalized bundle was **reclaimed**, so that it needed a
re-ingest rather than a replay, and that the re-ingest would move the retention declaration. **All three
were wrong.** `doctrine/chain_currency/retained_bundles.json` lists I2C among the declared **24 retained
bundles**, and the bundle is on disk. The sweep had said so in its own output — I2C appears under
`evidence: 24 replayed, 23 current, 1 stale`, i.e. replayed and CONTENT-stale, not among the 54
unmeasurable. The error was reading the summary line's parenthetical ("normalized bundle reclaimed")
as if it applied to the failing document rather than to the 54 it names.

The repair was therefore a plain chain rebuild, in the documented order with exactly one `validate` per
artifact, upstream-first: `evidence` → `validate` → `semantic` → `validate` → `intent` → `validate` →
`adapt --target isf`, with a `--dry-run` before each write to prove the loader accepts its upstream.
Pre-rebuild artifacts at `generated/preserved/CORPUS-CHAIN-CURRENCY.7/pre-rebuild-i2c/` (8 files, digest
`1a3a49a2f827e3847e9133249e5f2034d283cbd76aff8c07f231509aac0850cf`).

**What it publishes is a precision win.** `signal_constraints` **9 → 3**, `fact_provenance` 21 → 15,
13 `conditional_rules` renumbered. The eight removed records are exactly the shape the
`EXTRACTION-QUALITY-GAUGE.3k` series refused and that I2C never received:

| removed record | the sentence it came from |
| --- | --- |
| `USDA must_be_high` | *"Every byte put on the USDA line must be eight bits long."* — the modal governs the length |
| `USCL must_be_stable` | *"The UFm I²C-bus is a 2-wire push-pull serial bus that operates from DC to 5 MHz."* — no obligation at all |
| `SDA must_be_low` | *"If the data line (SDA) is stuck LOW, the controller should send nine clock pulses."* — a fault condition |
| `SDA must_be_stable` | *"When SDA remains HIGH during this ninth clock pulse, this is defined as the Not Acknowledge…"* — a definition |
| `SCLH must_be_high` | *"After the not-acknowledge bit (A), and the SCLH line has been pulled-up to a HIGH…"* — a narration |

All four stages then replay CONTENT SAME.

## `.7` step 4 — the corpus obstacle is gone, and the cost one is not (`2026-09-14`)

With `SPECFORGE_PROOF_SEAL_TOTAL_STAGES='semantic intent'` the probe reports **27 of 27 accepted at
semantic and at intent, 0 refused, exit 0**. Both repairs are confirmed at the gate that found the
drifts, and the corpus is current at every probeable stage.

The activation is still held, and the reason is a binary this tree had not looked at. Switching the
stage set on measured **13m01s** against **15.9 s** for the sampled default — because
`check_proof_seal_currency.sh` builds and probes with **`target/debug/specforge`**, while `.5`'s 66 s
sizing was measured with `target/release/specforge`. A debug probe is ~14 s where a release probe is
1.2 s. `.5`'s number was right about the release binary and wrong about the gate, and that is corrected
in `.5`'s node, in the script's own header, and in the fact card rather than left standing.

Thirteen minutes per commit is not gate tier. The mechanism stays inert, the corpus stays clean, and
the profile question is `.8`.

## `.8` — both profiles, cold and warm (`2026-09-14`)

Measured on this machine at `8126a072`, over the 27-document proof-carrying stratum. Cold builds were
taken in throwaway `CARGO_TARGET_DIR` trees on the repository volume, so the working `target/` was never
destroyed; each was reclaimed after its measurement.

| measurement | debug | release |
| --- | ---: | ---: |
| cold build of `--bin specforge`, empty target tree | 21.0 s | **36.1 s** |
| that cold target tree | 1.1 GB | **234 MB** |
| warm no-op freshness check | 0.18 s | 0.06 s |
| rebuild after a real edit in `crates/specforge/src/ir/evidence.rs` | 5.3 s | 32.6 s |
| one `intent --dry-run`, AXI `semantic_ir.json` 39.7 MB | 63.1 s | **6.5 s** |
| one `intent --dry-run`, ADIv6 `semantic_ir.json` 38.1 MB | 52.4 s | **5.2 s** |
| one `intent --dry-run`, HBM `semantic_ir.json` 15.9 KB | 0.00 s | 0.00 s |
| `check_proof_seal_currency.sh`, sampled default (4 probes + the 5-stage census) | 15.9 s | **7.4 s** |
| the same with `TOTAL_PROBE_STAGES='semantic intent'` (54 probes) | **12m57.9s** | **69.8 s** |
| `check_proof_seal_currency.sh --total`, the `PROOF-SEAL-TOTAL` CI doctrine | 18m45s | **1m59.2s** |
| `check_chain_currency.sh`, the `CHAIN-CURRENCY` CI doctrine | 28m00s | **12m38.2s** |
| absent corpus: the skip that happens before the build | 0.031 s, no cargo | 0.031 s, no cargo |

**The two CI-tier doctrines together go from ≈ 47 minutes to 14m37s.** That is the whole of "`--all` did
not finish in 50 minutes", and it is now a coffee break. The gain is not uniform, and the shape of the
difference is the interesting part: `PROOF-SEAL-TOTAL` is 9.4× faster because every probe it makes is a
deserialize-and-verify, which is exactly what an unoptimized build is worst at; `CHAIN-CURRENCY` is 2.2×
because its evidence leg replays extraction from a normalized markdown bundle, where the work is text
processing and I/O rather than parsing a 40 MB artifact.

**One more thing this change fixes that is not about cost.** `TOOLBOX.md` already tells a reader to run
the CLI as `./target/release/specforge`, and the corpus's own provenance cards record the hash of "the
owning **release** binary" for the documents they describe. The persisted corpus was produced by release
builds while the two doctrines that interrogate it used debug ones. They now agree.

**The cost is concentrated in a handful of documents, not spread over the corpus.** A 15.9 KB artifact
probes in 0.00 s at both profiles; a 39.7 MB one costs 63.1 s at debug and 6.5 s at release. So the
thirteen minutes were never "27 documents × 14 s" — they were a few large artifacts deserialized by an
unoptimized build, which is also why the release gain is a clean order of magnitude rather than a
constant factor.

**Per-commit arithmetic, with the commit mix measured rather than assumed** (43 of the last 100 commits
touch `crates/**.rs`), for the check's own marginal cost:

| | sampled default | activated at semantic + intent |
| --- | ---: | ---: |
| debug | 18.3 s | ~13 min |
| release | 19.8 s | **~84 s** |

At the sampled default the two profiles are a wash — release buys 8.5 s of probe and gives back 27 s of
build on the 43% of commits that touch Rust. The whole reason to move is the activated tier, where it is
**9.3× cheaper**, and that is `.9`. `.8` therefore makes the gate very slightly more expensive on a Rust
commit and says so, rather than claiming a win it does not have yet.

## Acceptance Checklist (enforced) — `.6`

- [x] **REPRODUCE / MEASURE** — `.4` and `.5`: 1 distinct seal per stage across 27 artifacts, so the
  sampled probe is 1 in 27; it passed while `intent` refused APB-e. Total probes cost 30.3 s (semantic)
  + 35.5 s (intent) = 66 s and find every refusal the corpus has.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_proof_seal_currency.sh`: `PROBE_SCOPE` was a single
  global, so the scope decision could not follow the cost, which differs by an order of magnitude
  between the stages that replay extraction and the stages that only load and verify.
- [x] **ADDRESSED (verified)** — `probe_scope_for <stage>` plus per-stage reporting; `--total` still
  forces total everywhere. Ships with the stage set EMPTY and the reason stated in the script itself:
  activating it fails the gate on a corpus whose repair is blocked, so the mechanism lands and the
  activation waits. One constant turns it on.
- [x] **NO REGRESSION** — self-test **20/20**; the default gate run is **15.9 s** and green on the live
  corpus, and byte-for-byte the same verdict as before this change (all stages sampled). `bash -n`
  clean. No Rust change, so no cargo gate and no artifact moves.
- [x] **OBSERVED RED** — with `TOTAL_PROBE_STAGES` emptied, self-test **17b fails**: *"a TOTAL stage
  passed over a document its own loader refuses"*. With it activated against the live corpus the check
  fails naming APB-e and I2C at both stages, which is the behaviour `.7` will land.
- [x] **GENERICITY (ADR 0006)** — the scope is keyed on the stage name the chain already defines
  (`chain_stages`), not on any document, vendor or protocol.
- [x] **LOCKSTEP** — the script's own header carries the measurement, the refutation of the
  topology-key repair, and the activation instruction; `[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]`
  records the same numbers. No behaviour was deleted, so no document describes a behaviour that is gone.

## Acceptance Checklist (enforced) — `.8`

- [x] **REPRODUCE / MEASURE** — the whole table above, taken before anything was edited: both profiles,
  cold and warm, per-probe and end to end, plus the corpus-absent skip at 0.031 s with no cargo
  invocation and the commit mix at 43 of 100. `.7`'s 13m01s re-derives here as **12m57.9s**.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_proof_seal_currency.sh`, `check_chain_currency.sh`
  and `rebuild_stage_cascade.sh` each carried their own `cargo build … --bin specforge` +
  `${CARGO_TARGET_DIR:-$ROOT/target}/debug/specforge`. The debug choice was argued from BUILD cost —
  correctly, while a run was four probes — and nothing re-examined it when `.6` made a run fifty-four.
  The cost is `serde_json` + derived `Deserialize` over 38–84 MB artifacts in an unoptimized build:
  15.9 KB probes in 0.00 s at both profiles, 39.7 MB costs 63.1 s at debug and 6.5 s at release.
- [x] **ADDRESSED (verified)** — `scripts/lib/corpus_replay_binary.sh` defines `corpus_replay_profile`
  and `corpus_replay_build` once; all three entrypoints source it and no longer name a profile. The
  sampled gate check goes **15.9 s → 7.4 s** and stays green with the identical verdict; the full
  gate-tier driver run is **4m44.7s**, all 14 gate doctrines PASS with both CI-tier ones DEFER.
- [x] **NO REGRESSION** — self-tests **20/20** (proof-seal), **22/22** (chain-currency), **14/14**
  (rebuild-cascade) — they pin the binary through their own stub env vars and are profile-independent
  by construction. `bash -n` clean on all three. **No verdict moves**: both profiles report 27 of 27
  accepted at semantic and at intent, and the artifacts are byte-identical — AXI `intent --dry-run`
  43,419,318 bytes at the same SHA-256, Wishbone `semantic --dry-run` 10,496,700 bytes, a legacy
  refusal the same string. **Both CI-tier oracles re-run end to end at the new profile and are green
  with their verdicts unchanged**: `check_proof_seal_currency.sh --total` **18m45s → 1m59.2s** (24/27
  accepted at source-ir with the three held-out bundles reported as no-verdict, 27/27 at evidence,
  semantic and intent), and `check_chain_currency.sh` **28m00s → 12m38.2s** (evidence 24/24 current,
  semantic 27/27, intent 27/27, isf-adapter 27/27, retention exactly the declared 24). No Rust changed,
  so `kg-bench` and the WIRE-BASED-100 golds are orthogonal by construction and no persisted artifact
  moved.
- [x] **OBSERVED RED** — staging was proved necessary rather than assumed: with the new lib present but
  untracked the gate FAILS with *"claim-verification: untracked producer-shaped path
  'scripts/lib/corpus_replay_binary.sh' exists under governed source roots"*, and passes once staged.
- [x] **GENERICITY (ADR 0006)** — a cargo profile name; no document, chip, vendor or protocol is named
  or read anywhere in the change.
- [x] **LOCKSTEP** — `scripts/lib/corpus_replay_binary.sh` carries the measurement and the
  no-verdict-change evidence; the proof-seal header's cost block now says which binary it was measured
  with and its inert-activation block is restated for `.9`; `TOOLBOX.md` §7.2a-i gains a WHICH-BUILD
  entry; `DOCTRINE_ENFORCEMENT.md` §10 gains the shared-predicate paragraph; the book's
  doctrine-enforcement chapter and
  `[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]` carry the new costs. No production
  rule was deleted, so no chapter describes a behaviour that is gone.
  **Two stale counts were found while doing this and corrected rather than left standing**: `TOOLBOX.md`
  said both `check_chain_currency.sh` and `check_proof_seal_currency.sh` had "sixteen fail-closed
  cases" (they have **22** and **20**), and `DOCTRINE_ENFORCEMENT.md` §10 repeated the chain-currency
  one. Nothing gates a self-test count, which is why all three drifted.

## Acceptance Checklist (enforced) — `.9`

- [x] **REPRODUCE / MEASURE** — the gate timed on the same tree immediately before the change:
  **4m13.0s**, ALL 15 executed doctrines PASS, with this check contributing 7.4 s and reporting a
  SAMPLED probe of 1 in 27 at semantic and at intent.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_proof_seal_currency.sh`: `TOTAL_PROBE_STAGES`
  defaulted to empty, so `probe_scope_for` returned `sample` at every stage. The blindness that buys is
  not hypothetical — `SIGNAL-DECLARATION-ROW-DROP.1b` is the case where it passed over 4 refused
  wire-gold documents for three commits under one seal.
- [x] **ADDRESSED (verified)** — the default is `'semantic intent'`; the live run now reports **TOTAL
  probe: 27 of 27 accepted** at both stages, SAMPLED at source-ir and evidence, exit 0. Measured cost:
  the check **1m12.7s**, the whole gate **5m30.1s** and **5m24.5s** on a second sample (+72-77 s, ~+29%),
  ALL 15 executed doctrines PASS on every run.
- [x] **NO REGRESSION** — self-tests **21/21**. Case 17 still proves the SAMPLED stages stay blind (its
  stub refuses only at `source_ir/`), so activation did not silently widen what the cheap stages claim.
  `bash -n` clean. No Rust changed, so `kg-bench` and the WIRE-BASED-100 golds are orthogonal by
  construction and no persisted artifact moved.
- [x] **OBSERVED RED** — `SPECFORGE_PROOF_SEAL_TOTAL_STAGES='' bash scripts/check_proof_seal_currency.sh
  --self-test` reports **20/21**, failing on *"self-test 21: the DEFAULT stage set passed over a
  document its own loader refuses"* and on nothing else. The new control fails for exactly the reason
  it exists and the twenty older ones are unaffected.
- [x] **GENERICITY (ADR 0006)** — the default names two stage ids the chain already defines
  (`chain_stages`); no document, chip, vendor or protocol appears.
- [x] **LOCKSTEP** — the script's own cost block, activation block, "what it proves" header and
  fully-sampled summary now describe what it does; `DOCTRINE_ENFORCEMENT.md`'s `PROOF-SEAL-CURRENCY`
  registry row, `TOOLBOX.md` §7.2a-i, the book's doctrine-enforcement chapter and
  `[[one-distinct-seal-makes-the-sampled-probe-a-one-in-27-sample]]` carry the activation and its cost.
  No production rule was deleted, so no chapter describes a behaviour that is gone.

- ID: `CORPUS-CHAIN-CURRENCY.10` · Status: `decided` (`2026-09-19`; execution is `.10a`) (opened `2026-09-19` by
  `EXTRACTION-GAP-FIX.5a`, which found it while sizing a producer change) · **Three proof-carrying
  documents cannot be re-derived at all, and they are three of the four wire-based golds.**
  **The measurement.** Of the 27 measured-stratum documents, **24 retain a normalized bundle and 3 do
  not**: `ihi0022_l_2025_08` (AXI), `ihi0024_e` (APB) and `ihi0033_c` (AHB). The retention declaration
  agrees and is not at fault — it declares `retained: 24, reclamations: 0`, and these three are simply not
  in it; they were ingested before `.2`'s retention rule and no bundle was ever kept. Observed directly:
  `specforge evidence generated/source_ir/ihi0024_e_.../source_ir.json --dry-run` fails with *"path does
  not exist: …/normalized/ihi0024_e_….md"*, while the same command on a bundle-retaining document
  succeeds in **0.5 s**.
  **Why that is an exposure rather than a tidiness note.** `EXTRACTION-GAP-FIX.5a` established by A/B that
  composing a reader into a registered evidence derivation invalidates every proof-carrying artifact until
  it is **rebuilt** — and rebuilding needs the bundle. So a change to the evidence producer silently
  removes AXI, APB and AHB from the measured stratum **permanently**, because the remedy the book offers
  (rebuild from the retained bundle) does not exist for them. Their only route back is a re-ingest from
  PDF, which is a different act with different risk: all three PDFs are present under `corpus/`, but
  re-ingesting rewrites the SourceIR that `WIRE-BASED-100` holds at `1.000` for exactly these three.
  **The shape of the problem, stated so the leaf is not mistaken for a cleanup.** The corpus is currently
  CURRENT, and `.0`–`.9` proved it. This leaf is not about drift; it is about **the cost of the next
  producer change**, which no surface states: 24 documents cost ~2 s each to rebuild through evidence and
  semantic, and 3 cost a full re-ingest plus an adjudicated regression risk on the project's own golds.
  **DECIDED `2026-09-19`: (a) re-ingest, and the other two options are closed on evidence, not preference.**
  **(c) does not exist and cannot be built for this case.** `source_proof_migrate` works because SourceIr
  re-derives its ledger *from its own retained capture* while touching no public content. EvidenceIR's
  equivalent capture **is** the normalized bundle, which is precisely what these three lack — so a
  proof-only re-seal would have to re-seal content the current producer would not produce. That is not a
  migration, it is a false attestation, and this repository refuses those by construction.
  **(b) forecloses the project's main improvement path.** `EXTRACTION-QUALITY-GAUGE.3j.3` measured that
  deterministic recall — a property of the evidence producer — is the binding constraint on extraction
  quality, and `EXTRACTION-GAP-FIX.5` located the bound inside `extract_signal_constraints` itself. Freezing
  the three means every future change to that producer is a decision about losing AXI, APB and AHB. A
  programme whose bottleneck is a component it may never change is not a programme.
  **(a) pays a cost that is coming anyway, at a moment of our choosing.** The re-ingest risk — that a fresh
  SourceIR moves the `WIRE-BASED-100` scores these three hold at `1.000` — does not go away by waiting; it
  only moves inside a future producer slice, where it would be conflated with that slice's own changes and
  impossible to attribute. Paying it now, isolated, with the golds re-verified in the same transaction, is
  one variable at a time instead of two.
  **The rollback exists, and it had to be made rather than assumed: `generated/` is git-ignored, so there
  is no version-control rollback for any of this.** Snapshot taken `2026-09-19` at
  `.project-data/tmp/pre-reingest-snapshot-2026-09-19/`, holding the complete `source_ir`, `evidence_ir`,
  `semantic_ir` and `intent_ir` chain for all three documents, verified byte-identical against the live
  tree: **21 files, 193,457,740 bytes, content digest `5a5dffa2865f67ad`**. Naming that path here is what
  makes it *reachable* under `SCRATCH-RESIDUE-CONTAINMENT.0`'s model, so the residue sweep will not take it.
  Re-verify with the same census before relying on it.
  **Execution order, which is the next slice and is deliberately not folded into this one.** Start with
  **APB**, the smallest of the three, because it answers the question that decides the other two: does a
  re-ingest reproduce the document well enough to leave the golds at `1.000`? Per document — re-ingest;
  rebuild the cascade (measured cheap: evidence `0.5 s`, semantic `0.8 s`); re-verify `WIRE-BASED-100`
  **before** touching the next document; update the retention declaration, which moves `retained: 24`
  toward 27 and will otherwise redden `CHAIN-CURRENCY` both ways; and re-verify the measured stratum is
  still 27. If APB's golds move, **stop and adjudicate** — that outcome is itself the answer, and it turns
  (a) back into a live question rather than a procedure.
  **CORRECTED `2026-09-19` by `.10a`, and the correction reverses this leaf's decision.** The premise above
  — that these three "cannot be re-derived at all" and that "their only route back is a re-ingest from PDF" —
  is false. The failing `evidence` command it rests on is real, but a bundle absent from the normalized root
  is not a bundle that does not exist: all three were re-ingested by `WIRE-BASED-100.9b`/`.9c`/`.9d`
  (`2026-09-10`) and again by `WIRE-BASED-100.10` (`2026-09-11`), and each run **held the bundle out** under
  `generated/preserved/` rather than declaring it, because the retention declaration is frozen
  (`RETAINED-BUNDLE-POPULATION-FROZEN`). `.10a` measured all three replaying **CONTENT SAME** from those
  held-out bundles in **0.30 s / 0.67 s / 3.28 s**. So option (a) is withdrawn: it would have paid a real
  gold-regression risk to produce an artifact already on this volume. The reasoning that closed (b) and (c)
  survives untouched — a proof-only re-seal still cannot exist, and freezing the three would still foreclose
  the producer work — but the live option was never (a). It is (d): **install the bundle already held**, which
  `RETAINED-BUNDLE-POPULATION-FROZEN.3` owns and which is blocked on a declaration wall, not on Docling.
  What this leaf got right and should keep credit for: the exposure is real, it was found by measurement
  rather than assumed, and the rollback it built is the right rollback for the remedy that replaces it.
  Prerequisite: none. Blocks: `EXTRACTION-GAP-FIX.5b`, whose ten records are all in AXI.
  Verification: the retention census, the byte-identical snapshot, and the stage-cost measurement
  Commit: `CORPUS-CHAIN-CURRENCY.10 — decide the re-ingest, and build the rollback that did not exist`

- ID: `CORPUS-CHAIN-CURRENCY.10a` · Status: `done` (`2026-09-19`, PROBE/DOC) · **The re-ingest was not
  executed, because executing it would have been wrong: `.10`'s premise is refuted. All three documents
  are already re-ingested, their bundles are on the repository volume, and every one of them replays
  CONTENT SAME in seconds.**
  This leaf owned "the execution and the judgement inside it". The judgement came first and it stopped the
  execution — a stronger stop than the one `.10` anticipated, and it arrived before the golds were ever at
  risk.
  **What `.10` asserted, and what is actually true.** `.10` decided (a) re-ingest on the premise that these
  three "cannot be re-derived at all" and that "their only route back is a re-ingest from PDF". The
  observation behind it was sound — `specforge evidence …` does fail with *"path does not exist:
  …/normalized/…md"* — but the inference from it was not. The bundle is not GONE; it is **HELD OUT**, and
  deliberately so. `WIRE-BASED-100.9b`/`.9c`/`.9d` re-ingested APB, AHB and AXI on `2026-09-10`, and
  `WIRE-BASED-100.10` re-ingested them again on `2026-09-11` — the run that wrote the SourceIR these three
  carry today. Each run produced the normalized bundle and then **parked it outside the normalized root**
  rather than declaring it, because declaring it reddens two gate-tier doctrines
  (`RETAINED-BUNDLE-POPULATION-FROZEN`). The bundles sit at
  `generated/preserved/WIRE-BASED-100.10/{apb,ahb,axi}-normalized-bundle-held-out/`, with APB additionally
  at `…/WIRE-BASED-100.9b/…` — byte-identical markdown across both preservation points
  (`f83d437d585d8ce13a1ff875c29dbfb0d70ef5c184318dfef0151b003b4aeb7c`, 70,357 bytes).
  **Measured `2026-09-19`, and it is the whole finding.** For each document: copy the held-out bundle to the
  normalized root the SourceIR declares, run `specforge evidence <source_ir.json> --dry-run` on the release
  build, strip the `*_json:` preamble the way `check_chain_currency.sh` does, compare with
  `compare_stage_artifact` against the persisted EvidenceIR, then remove the copy.

  | document | replay | elapsed | content identity vs persisted EvidenceIR |
  | --- | --- | ---: | --- |
  | `ihi0024_e` (APB) | exit 0 | **0.30 s** | **CONTENT SAME** |
  | `ihi0033_c` (AHB) | exit 0 | **0.67 s** | **CONTENT SAME** |
  | `ihi0022_l` (AXI) | exit 0 | **3.28 s** | **CONTENT SAME** |

  **4.25 s for all three**, against a three-document Docling re-ingest plus an adjudicated regression risk on
  the project's own golds. The copy was a copy and never a move, so the held-out originals were never at
  risk; the normalized root was removed after each probe and `git status` is clean.
  **So the exposure `.10` found is real, and its cause is misattributed.** These documents are not
  *unrebuildable*. They are **undeclared**. What is missing is not the artifact — it is permission to install
  the artifact where the producer reads it, and that permission is frozen by three live mechanisms
  (`len(retained_ids) != 24` in `scripts/validate_residual_actionability_contract.py:592` and
  `scripts/validate_canonical_recovery_contract.py:455`; `reclamations != []` in both; behavioral-population
  set equality in `scripts/check_behavioral_genericity_contract.py:1392`). That is not this tree's wall to
  take down, and it already has an owner.
  **CONTENT SAME is also what makes the correct remedy safe, and it is the reason the re-ingest was the
  riskier of the two.** `.10` accepted a gold-regression risk because a fresh SourceIR could move the
  `WIRE-BASED-100` scores these three hold at `1.000`. Installing a held-out bundle carries no such risk and
  the argument is closed rather than probabilistic: every downstream stage reads the persisted EvidenceIR,
  the replayed EvidenceIR is content-identical to it, so no downstream score has an input that moved. The
  re-ingest would have paid a real risk to obtain an artifact already in hand.
  **Routed out, not absorbed.** Installing the three bundles and declaring them is
  `RETAINED-BUNDLE-POPULATION-FROZEN.3`, which already owned exactly this act for APB and is widened here to
  all three on this leaf's evidence; it stays blocked on `.2` (what a newly retained key owes the frozen
  behavioral population) and `.1` (retire the redundant literal). The falsification and durability legs of
  the published claim are owned by `.10b`, which ships the probe as a tracked producer so
  `RETAINED-BUNDLE-POPULATION-FROZEN.3` can pre-flight its restore instead of discovering the answer by
  performing it.
  **The rollback snapshot stays.** `.project-data/tmp/pre-reingest-snapshot-2026-09-19/` was built for the
  re-ingest, and the restore it is being replaced by mutates the same chain, so it remains exactly the right
  rollback for `RETAINED-BUNDLE-POPULATION-FROZEN.3`. Census re-verified `2026-09-19`: **21 files,
  193,457,740 bytes** — unchanged from `.10`.
  Prerequisite: `.10`. Blocks: `EXTRACTION-GAP-FIX.5b` (still blocked, now on
  `RETAINED-BUNDLE-POPULATION-FROZEN.3` rather than on a re-ingest).
  Verification: the three-document probe table above, each probe restored to pre-state and `git status`
  clean after it; the held-out bundle inventory under `generated/preserved/`; the three freeze mechanisms
  re-read in current source; snapshot census re-verified at 21 files / 193,457,740 bytes.
  Published-claims: `wire-gold-bundles-are-held-out-not-lost` (`incomplete` at this leaf; lifted to
  `verified` by `.10b`, which also found the inventory is six bundles rather than three)
  Commit: `CORPUS-CHAIN-CURRENCY.10a — the re-ingest is refused: the bundles were never lost, only held out`

- ID: `CORPUS-CHAIN-CURRENCY.10b` · Status: `done` (`2026-09-19`, CODE/DOC) · **The probe is shipped, the
  claim is `verified`, and the inventory is twice what `.10a` thought it was.**
  `scripts/probe_held_out_bundle_replay.sh` (340 lines) now answers, on demand and from a tracked
  producer, the question that nearly cost a destructive re-ingest: is a bundle absent from the
  normalized root *lost*, or merely *held out*?
  **The finding `.10a` did not have: there are SIX held-out bundles, not three.** `.10a` probed the
  `WIRE-BASED-100.10` set by hand. The probe globs the whole preservation root and found
  `WIRE-BASED-100.9b`/`.9c`/`.9d` still hold their own copies of APB/AHB/AXI — **all six replay CONTENT
  SAME**, and the census shows each document's markdown digest is identical across both preservation
  points (APB `f83d437d…`, AHB `29103894…`, AXI `abdb221b…`). The rollback for
  `RETAINED-BUNDLE-POPULATION-FROZEN.3` is therefore redundant rather than single-copy, which is a
  strictly better position than this tree recorded a commit ago.

  | document | `.9x` bundle | `.10` bundle |
  | --- | ---: | ---: |
  | `ihi0024_e` (APB) | CONTENT SAME 0.29 s | CONTENT SAME 0.30 s |
  | `ihi0033_c` (AHB) | CONTENT SAME 0.66 s | CONTENT SAME 1.11 s |
  | `ihi0022_l` (AXI) | CONTENT SAME 3.42 s | CONTENT SAME 3.41 s |

  Whole run: **6 proved, 0 skipped, 0 failed**, and every `source_ir/` directory back to exactly
  `source_ir.json` afterwards with no scratch residue.
  **Two modes, because they are two different questions at two different prices.** The bare probe
  stages ~280 MB per preservation point and takes ~59 s — the right cost before a corpus mutation, the
  wrong cost at every commit. `--census` answers the affordable half in **0.108 s** without staging
  anything: which bundles exist, which document each declares, and whether that document still carries
  the persisted SourceIR/EvidenceIR a replay would read. That split is what let the claim reach
  `verified` at gate cost instead of adding a minute and 560 MB of copying to every commit.
  **Safety is the property, not the cleanup.** A staged bundle no leaf declared fails `CHAIN-CURRENCY`
  closed, so pre-state restoration must hold on success, on failure, on refusal and on interrupt — a
  `RETURN` trap plus a globally tracked staged path released on `INT`/`TERM`/`EXIT`. The bundle is
  copied, never moved. An already-installed bundle is **skipped, never clobbered**: overwriting a
  declared bundle is the one move here with no way back.
  **What review caught before it shipped, and it is the reason the self-test has accounting cases.**
  The first draft returned success for a skip and counted only failures, so a run in which every bundle
  was skipped would have printed *"3 probed, 0 failures"* — reading as proof while proving nothing.
  That is the class `CLAIM_VERIFICATION.md` §2 refuses. Proved/skipped/failed are now counted and
  reported separately, and the claim binds **proved**. Three further draft defects were fixed: whole-
  second timing that rendered APB's 0.30 s as `0s` (now `Time::HiRes`, because GNU `date +%s%N` is not
  on a stock macOS and these scripts are Bash-3.2-safe by declaration), a non-`local` leak, and an
  `ls`-parse replaced by a glob.
  **`--self-test` is 18/18 and it was made to go RED three times**, each perturbation reverted
  byte-identically: returning 0 for an installed bundle fails cases 11 and 14; dropping
  `probe_release_staged` from the `RETURN` trap fails cases 2 and 5; accepting an unresolvable bundle
  in the census fails case 18. A fourth attempt did **not** go red — and the reason is worth keeping:
  the perturbation had silently failed to apply. A perturbation that does not land looks exactly like a
  check that does not catch, so a RED observation is only evidence once the edit is confirmed present.
  Prerequisite: `.10a`. Blocks: nothing.
  Verification: `--self-test` 18/18 with three confirmed RED perturbations; `--census` 6/6 resolvable in
  0.108 s; bare probe 6 proved / 0 skipped / 0 failed in 58.6 s with pre-state restored and `git status`
  clean; `scripts/check_claim_verification.pl --check` green with the record at `verified`.
  Published-claims: `wire-gold-bundles-are-held-out-not-lost` (`verified`)
  Commit: `CORPUS-CHAIN-CURRENCY.10b — ship the oracle, and find six bundles where three were counted`

- ID: `CORPUS-CHAIN-CURRENCY.10c` · Status: `done` (`2026-09-19`, CODE/DOC) · **the held-out census exits 1 when it correctly
  finds zero, so the probe's own healthy end state fails forever.**
  `run_census()` in `scripts/probe_held_out_bundle_replay.sh` ends in
  `[ "$found" -gt 0 ] && [ "$found" -eq "$resolvable" ]`. The `-gt 0` guard was right while held-out
  bundles were expected to exist — it stops an empty census being mistaken for a proof. But
  `RETAINED-BUNDLE-POPULATION-FROZEN.3` installed and declared all three golds and removed the six
  preserved copies on `2026-09-19`, so **zero held-out bundles is now the correct, permanent steady
  state**, and the probe reports it as a failure. Measured: `--census` prints
  `census: 0 held-out bundle(s), 0 resolvable …` and exits **1**.
  **This is ADR 0050's pattern in another file** — an expectation true only of a transient state, wired
  as a live invariant. The remedy shape follows the ADR: separate "no bundles are held out" (healthy,
  exit 0, said plainly) from "bundles exist and some are unresolvable" (exit 1), so the oracle can
  report an empty population without calling it a fault, and still refuse to treat a skipped or
  unresolvable bundle as a proof.
  **Not currently red at any gate, which is why this is a leaf and not a stop.** Nothing in
  `scripts/check_doctrines.sh` or `scripts/run_ci.sh` invokes the probe; its only executed reference was
  `wire-gold-bundles-are-held-out-not-lost`, superseded by `.3`. So it is wrong in silence today — the
  same condition ADR 0049 warns about — and the next session to run the probe would read a red command
  as a corpus problem.
  Acceptance: `--census` exits 0 and states plainly that no bundles are held out when the preserved tree
  is empty; a bundle that exists but does not resolve still exits 1; both are controlled cases in
  `--self-test`, which stays green at its full count; the claim or fact card that cites the census is
  updated to the new expected line.
  Prerequisite: none. Opened `2026-09-19` by `RETAINED-BUNDLE-POPULATION-FROZEN.3`, which caused the
  end state that exposed it.
  **SHIPPED.** `run_census` now returns 0 on an empty preserved tree with a line that states the
  outcome without implying one — `census: no bundles are held out — nothing to replay, and nothing
  outstanding` — and the resolvable equality `[ "$found" -eq "$resolvable" ]` still governs every
  non-empty census unchanged. The relaxation is exactly one case wide: an empty population is no
  longer a fault, and nothing else moved.
  **The empty line deliberately proves nothing.** Case 20 asserts it mentions neither `resolvable` nor
  `CONTENT SAME`, because what made the old guard defensible was the fear of an empty census reading
  as a clean bill of health. Green is now reachable without being green-by-implication.
  Verification: `--self-test` **20/20** (was 18/18; cases 19 and 20 are new). Two perturbations with
  the producer restored byte-identically after each
  (`059a9dbb41332850f1001b1915bdccba9f9cd013a617a0459eef27e188a88e77`): restoring the pre-`.10c`
  semantics (`[ "$found" -gt 0 ] && …`, early return removed) fails **exactly cases 19 and 20** and
  nothing else, and replacing the resolvable equality with `true` fails **exactly case 18** — so the
  new behaviour and the refusal it must not weaken are each caught by their own binding. Live
  `--census` exits 0 on the now-empty preserved tree.
  Commit: `CORPUS-CHAIN-CURRENCY.10c — zero held-out bundles is the answer, not a fault`

- ID: `CORPUS-CHAIN-CURRENCY.11` · Status: `pending` (opened `2026-09-20` by
  `SIGNAL-DECLARATION-ROW-DROP.2h.2`) · Goal: **the 51 legacy chains are reported as UNMEASURABLE, and
  the repository keeps measuring them anyway.** `check_chain_currency.sh` is honest about the stratum
  it cannot replay — 27 replayed, 27 current, **51 UNMEASURABLE** — but a persisted
  `evidence_ir.json` exists for every one of the 78 documents, and censuses read them. `.2h.2` found
  the consequence with a named instance: `.2j.1`'s drift census takes its `declared` column out of
  those artifacts, and for CoreSight TMC `table_0074` the artifact records the declaration name
  `DATA` where the current reader emits `Data`. That artifact is therefore evidence about a binary
  nobody can name, and a census built on it is measuring history.
  **What makes it unmeasurable is specific and worth stating:** a legacy artifact stops at
  `EvidenceIr::build_unproved_from_source_ir`, whose normalization-status precondition it fails, so
  the product's own path cannot re-derive it. `synthesize_declarations_from_tables` and
  `synthesize_signal_declaration_seed` CAN be run against a serde-loaded `SourceIr` — `.2h.2` did
  exactly that to measure its own change — so the question is not whether the reader can be asked,
  but whether the answer may be compared with an artifact built by an unknown revision.
  Acceptance: a read-only measurement of how many of the 51 legacy `evidence_ir.json` differ from
  what the current reader produces, with the difference characterised (not merely counted) and an
  adjudicated sample; plus a decision, recorded here, on whether a legacy artifact may be cited as
  evidence about the reader at all, or only about itself. **Not** a re-ingest — that is
  `CORPUS-COVERAGE`'s, and the non-goals above forbid it here.
  Non-goal: rebuilding or re-ingesting any document; changing the currency oracle's verdict.
  Prerequisite: none.
  Verification: pending
  Commit: pending

## Current Frontier

1. **This tree's corpus question is answered and its remaining work is owned elsewhere.** `.0`–`.9`
   closed and the corpus is CURRENT (27 of 27 accepted at semantic and at intent, retention exactly the
   declared 24 bundles). `EXTRACTION-GAP-FIX.5a` found a real cost-of-change exposure in the 3 documents
   outside those 24 — AXI, APB and AHB, three of the four wire-based golds — `.10` decided to re-ingest
   them from PDF, **`.10a` refused that execution**, and `.10b` shipped the oracle that settles it.
   The three are not unrebuildable: **six** held-out bundles under
   `generated/preserved/WIRE-BASED-100.{9b,9c,9d,10}/` all replay **CONTENT SAME**, two independent
   copies per document with identical markdown digests.
2. **The remedy landed `2026-09-19`.** `RETAINED-BUNDLE-POPULATION-FROZEN.3` installed and declared all
   three golds: `retained` is 27 and `check_chain_currency.sh` reports **27 replayed / 27 current / 0
   stale** at every stage, with retention exactly the declared set. The six preserved copies are gone
   (583,434,736 bytes, residue census 0) and the held-out census is 0.
3. **`.10c` closed `2026-09-19`**: the census now answers 0 with exit 0, so the oracle `.10b` shipped
   survives the outcome it was built to enable.
4. **`CORPUS-CHAIN-CURRENCY.11` — ELIGIBLE, and it arrived the way `.10` did**: as a measurement that
   found something, not as a scheduled sweep. `SIGNAL-DECLARATION-ROW-DROP.2h.2` showed that the 51
   UNMEASURABLE chains are still being read — its own tree's censuses take a `declared` column out of
   them — and named one artifact that demonstrably differs from the current reader (`DATA` vs `Data`
   on CoreSight TMC `table_0074`). The oracle is honest; the consumers of those artifacts are not
   gated.
4. Rebuilding a drifted document is **not** this tree's next step: `.7` rebuilt both of them, APB-e and
   I2C, and every stage of both replays CONTENT SAME.

## Verification Log

- `2026-09-19` — `.10c`. `scripts/probe_held_out_bundle_replay.sh --self-test` **20/20**, up from 18/18
  with two new census cases. Perturbed twice with the producer restored byte-identically after each
  (`059a9dbb41332850f1001b1915bdccba9f9cd013a617a0459eef27e188a88e77`): restoring the pre-`.10c`
  `[ "$found" -gt 0 ] && …` semantics fails **exactly cases 19 and 20**, and replacing the resolvable
  equality with `true` fails **exactly case 18**. So the healthy-empty behaviour and the
  unresolvable-bundle refusal are each enforced by their own binding, and the relaxation is one case
  wide. Live `--census` against the now-empty preserved tree exits **0**:
  `census: no bundles are held out — nothing to replay, and nothing outstanding`.

- `2026-09-19` — `.10b`. `scripts/probe_held_out_bundle_replay.sh --self-test` **18/18**, and made to go
  RED three times with the producer restored byte-identically after each: skip returning 0 fails cases
  11 and 14, dropping `probe_release_staged` from the `RETURN` trap fails cases 2 and 5, and a census
  that accepts an unresolvable bundle fails case 18. `--census` resolves **6 of 6** bundles in 0.108 s
  without staging. The bare probe: **6 proved CONTENT SAME, 0 skipped, 0 failed** in 58.6 s — APB
  0.29/0.30 s, AHB 0.66/1.11 s, AXI 3.42/3.41 s across the `.9x` and `.10` preservation points — with
  every `source_ir/` directory restored to exactly `source_ir.json`, no scratch residue under
  `.project-data/tmp/`, and `git status` showing only the new script.
  `perl scripts/check_claim_verification.pl --check` green with
  `wire-gold-bundles-are-held-out-not-lost` at **`verified`** (8 claims, 14 source/control commands).
- `2026-09-19` — `.10a`, and it is a refutation rather than a confirmation. Each of the three golds probed
  by copying its held-out bundle to the normalized root its own SourceIR declares, replaying
  `specforge evidence <source_ir.json> --dry-run` on `target/release/specforge`, stripping the `*_json:`
  preamble exactly as `check_chain_currency.sh` does, and comparing against the persisted EvidenceIR with
  `compare_stage_artifact` from `scripts/lib/stage_artifact_identity.sh` — the gate's own comparator, not a
  second one. **APB exit 0 / 0.30 s / CONTENT SAME; AHB exit 0 / 0.67 s / CONTENT SAME; AXI exit 0 / 3.28 s /
  CONTENT SAME.** Every probe copied rather than moved, so no held-out original was exposed, and each removed
  its copy before the next began; `git status` clean and each `source_ir/` directory back to exactly
  `source_ir.json` afterwards. The three freeze mechanisms were re-read in current source rather than taken
  from the fact card: `validate_residual_actionability_contract.py:592`,
  `validate_canonical_recovery_contract.py:455`, `check_behavioral_genericity_contract.py:1392`. Rollback
  snapshot re-censused at 21 files / 193,457,740 bytes, unchanged.
- `2026-09-14` — `.9`, closing audit of its own class. Self-test 21 guards ONE default; the question it
  raises is whether any other self-tested doctrine check ships an unguarded one. Enumerated over every
  `check_*.sh` plus `rebuild_stage_cascade.sh` that has a `--self-test`, covering **both** expansion forms
  (`${VAR:-d}` and the `${VAR-d}` form the defect actually used — a first pass matched only the former and
  missed the very variable in question, which is why the enumeration is stated with its pattern). The
  complete population is seven: `SPECFORGE_PROOF_SEAL_TOTAL_STAGES` (`semantic intent` — the only one that
  changes what a check PROVES, and the one case 21 now reads), three `*_GENERATED_ROOT` and one
  `*_RETENTION_CONTRACT` whose defaults are PATHS exercised against the real corpus on every gate run, and
  three `*_BIN`/`*_WORK` overrides defaulting to empty, which is the ordinary path every real run takes.
  **So there is no second instance**: the behavioural default is the guarded one, and the rest are exercised
  by the gate itself rather than by a control.
- `2026-09-14` — `.9`. Gate timed before and after on the same tree: **4m13.0s → 5m30.1s**, a second
  post-change sample **5m24.5s**, ALL 15
  executed doctrines PASS both times; this check alone **7.4 s → 1m12.7s**, reporting TOTAL 27/27 at
  semantic and at intent and SAMPLED at source-ir and evidence. Self-tests **21/21**; observed RED at
  **20/21** with `SPECFORGE_PROOF_SEAL_TOTAL_STAGES=''`, failing only on the new case 21. Read-only:
  every probe is a `--dry-run` and no persisted artifact moved.
- `2026-09-14` — `.8`. Both profiles measured cold and warm before any edit, cold builds taken in
  throwaway `CARGO_TARGET_DIR` trees under `.project-data/tmp/` on the repository volume and reclaimed
  afterwards (5.1 GB, zero residue; `git status` clean across the whole measurement). Profile identity
  proved by byte comparison, not by argument: AXI `intent --dry-run` **43,419,318 bytes, identical
  SHA-256** between `target/debug/specforge` and `target/release/specforge`; Wishbone `semantic
  --dry-run` 10,496,700 bytes identical; a legacy artifact's refusal string identical. After the change:
  `check_proof_seal_currency.sh --self-test` **20/20**, `check_chain_currency.sh --self-test` **22/22**,
  `rebuild_stage_cascade.sh --self-test` **14/14**; `scripts/check_doctrines.sh` **4m44.7s, 14 gate
  doctrines PASS, 2 DEFER**; `check_proof_seal_currency.sh --total` **1m59.2s exit 0**;
  `check_chain_currency.sh` **12m38.2s exit 0** — evidence 24/24 current, semantic 27/27, intent 27/27,
  isf-adapter 27/27, retention exactly the declared 24 bundles. Read-only throughout: every probe is a
  `--dry-run`, and no persisted artifact moved.
- `2026-09-14` — `.7` steps 3 and 4. I2C rebuilt from its RETAINED bundle (snapshot
  `generated/preserved/CORPUS-CHAIN-CURRENCY.7/pre-rebuild-i2c/`, 8 files, `1a3a49a2…50cf`) in the
  documented order, one `validate` per artifact, a `--dry-run` before every write. Result:
  `signal_constraints` 9 → 3, `fact_provenance` 21 → 15, and `evidence`/`semantic`/`intent`/`adapt` all
  replay CONTENT SAME. With the stage set forced on, `check_proof_seal_currency.sh` reports **27 of 27
  accepted at semantic and at intent, exit 0**.
  **Cost correction, measured**: the activated run is **13m01s**; the sampled default is **15.99 s**;
  self-tests **20/20** either way. Root cause established by reading the script's own build step — it
  runs `cargo build --bin specforge` and probes `target/debug/specforge`, while `.5`'s 66 s sizing used
  `target/release/specforge` (1.2 s per accepted intent probe against ~14 s). The activation was
  reverted to inert in the same slice that measured it; the corpus repair stands.

- `2026-09-14` — `.7` step 2 (APB-e rebuild). Snapshot first:
  `generated/preserved/CORPUS-CHAIN-CURRENCY.7/pre-rebuild/`, 6 files, digest `8d1b9e13…8791f`.
  Order: `semantic` (write) → `validate` (once) → `intent` (write) → `validate` (once) →
  `adapt --target isf` (write); `intent --dry-run` was run against the rebuilt SemanticIR BEFORE
  writing intent, to prove the loader accepts it. Post-conditions, each measured: all three stages
  replay CONTENT SAME; the emitted `.isf` `source_text` byte-identical at 4,029 bytes; `adapter.json`
  with no moved section; `seed_apb` gold unchanged (1.000 / 1.000 filtered); retention 24; the default
  seal gate green; and with the tier activated the semantic and intent probes go 25/27 → **26/27**, I2C
  alone remaining.

- `2026-09-14` — `.6`. `bash scripts/check_proof_seal_currency.sh --self-test` **20/20**, including the
  new 17b. **Observed RED** by emptying `TOTAL_PROBE_STAGES` and re-running: 19/20, 17b failing with
  *"a TOTAL stage passed over a document its own loader refuses"* — so the control asserts the
  mechanism and not the default. Live corpus: the default run is green in **15.9 s**; with
  `SPECFORGE_PROOF_SEAL_TOTAL_STAGES='semantic intent'` it is RED, naming
  `ihi0024_e…apb` and `um10204…i2c` at semantic and at intent, with the non-stale-seal diagnostic.
  `bash -n` clean. Read-only: the check writes nothing (its own self-test 13 proves that end to end).

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
- `.6` — `CORPUS-CHAIN-CURRENCY.6`.
- `.7` step 2 — `CORPUS-CHAIN-CURRENCY.7` (APB-e rebuild).
- `.7` steps 3-4 — `CORPUS-CHAIN-CURRENCY.7` (I2C rebuild; activation measured and held).
- `.8` — `CORPUS-CHAIN-CURRENCY.8` (the corpus-replay binary profile).
- `.9` — `CORPUS-CHAIN-CURRENCY.9` (the per-stage TOTAL probe activated).
- `.10` — `CORPUS-CHAIN-CURRENCY.10` (the re-ingest decision, since reversed by `.10a`).
- `.10a` — `CORPUS-CHAIN-CURRENCY.10a` (the re-ingest refused; the bundles were held out, not lost).
- `.10b` — `CORPUS-CHAIN-CURRENCY.10b` (the oracle shipped; six bundles found where three were counted).

| Unit | Durable evidence |
| --- | --- |
| `CORPUS-CHAIN-CURRENCY` ownership | `8475900d` — `CORPUS-CHAIN-CURRENCY — track measured persisted-chain currency` |
| `CORPUS-CHAIN-CURRENCY.0` completion | `CORPUS-CHAIN-CURRENCY.0 — decide currency policy and close measured drift` |
| `CORPUS-CHAIN-CURRENCY.1` completion | `ddc2f798` — `CORPUS-CHAIN-CURRENCY.1 — gate persisted-chain currency as a CI-tier doctrine` |
| `CORPUS-CHAIN-CURRENCY.3` completion | `CORPUS-CHAIN-CURRENCY.3 — rebuild the corpus chain to currency` |
| `CORPUS-CHAIN-CURRENCY.2` completion | `CORPUS-CHAIN-CURRENCY.2 — declare and gate normalized-bundle retention` |
