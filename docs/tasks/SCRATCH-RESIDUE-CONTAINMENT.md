# SCRATCH-RESIDUE-CONTAINMENT: reclaim repository scratch that nothing can reach

## Metadata

- Tree ID: `SCRATCH-RESIDUE-CONTAINMENT`
- Status: `active` (`.0`, `.3`, `.5`-`.14`, `.16`-`.18`, `.20`-`.21` complete; `.1`, `.2`, `.4`, `.15`, `.19`
  pending)
- Roadmap lane: repository durability and portability (sibling of `SOURCE-IR-REPRODUCIBILITY`)
- Created: `2026-08-27`
- Last updated: `2026-10-27`
- Owner: repo-local workflow

## Goal

Make "repository scratch is reclaimed when nothing can reach it" a **decided and evidenced** property rather
than a periodic guess. The artifact-cleanup directive asks for a sweep roughly every day and permits deletion
only when it is provably safe; the reachability question is what makes it provable.

## Non-Goals

- Do not delete a root that a tracked declaration names, or that the retained-evidence chain can reach. The
  behavioral holdout resolves retained artifacts by walking `retained_evidence_path` links, so a root can be
  load-bearing without any tracked file naming it directly.
- Do not delete the evidence behind a published signoff on a sweep's own authority. That is a decision (`.1`).
- Do not touch stage artifacts under `generated/`; their lifecycle is owned by `specforge clean` and
  `CORPUS-CHAIN-CURRENCY`. **Narrowed `2026-08-29` (`.3`), because this Non-Goal's premise was measured and
  is false for one class.** It assumed everything under `generated/` has an owner. Test-fixture roots left by
  `scripts/test_live_document_size.pl` have none: `specforge clean`'s three scopes are `source-normalized`,
  `document`, and `all-generated` (verified in `crates/specforge/src/commands/clean.rs` and `clean --help`),
  the first two are keyed on document roots and reach nothing else, and the third discards the whole corpus —
  so there is no proportionate route. `CORPUS-CHAIN-CURRENCY` walks persisted stage artifacts and never sees
  them. The narrowing is deliberate and bounded: unreachable **test-fixture** roots under `generated/` are in
  scope; stage artifacts remain out.

## Reachability model (`.0`, `2026-08-27`, complete)

A scratch root under `.project-data/tmp/` is reachable if either holds:

1. **Named** — a Git-tracked file names it. `git grep -n "\.project-data/tmp/[A-Za-z0-9._-]"` is the complete
   enumeration.
2. **Chained** — `ultimate_retained_artifact_output_root`
   (`crates/specforge-conformance/src/behavioral_genericity.rs`) walks up to eight `retained_evidence_path`
   links from `doctrine/production_genericity/behavioral_holdout_evidence.json`, and resolves retained
   attempt artifacts under `<ultimate root>/attempts/<relation>/<document_key>`. Every root on that walk is
   load-bearing even though no tracked file names it.

Traced at this revision, the chain is one hop: the doctrine aggregate names
`.project-data/tmp/spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`, whose own
`retained_evidence_path` is absent, so that root is the ultimate artifact store. Its declared
`retained_evidence_sha256` matches the file on disk exactly.

| Root | Size | Files | Reachability | Disposition |
| --- | ---: | ---: | --- | --- |
| `spec-to-intent-f-iii-heldout` | 7,408.8 MiB | 14,039 | named + chain terminus; supplies `attempts/` | retain |
| `spec-to-intent-f-iii-heldout-qualified-r5` | 6,718.1 MiB | 13,839 | unnamed, but is the run behind the committed aggregate at `2cdcd131` | `.1` decides |
| `spec-to-intent-external-sources` | 6.1 MiB | 8 | named by `CHANGES.md` and the `.8d` carrier record | retain |
| `signal-catalog-capture-gap-1` | 0.1 MiB | 10 | its tool is named by `docs/tasks/SIGNOFF-BURNDOWN.md` | retain |
| `source-ir-census-1` | 1,940.2 MiB | 4,929 | unreachable; `SOURCE-IR-REPRODUCIBILITY.1` result published | reclaimed |
| `srir-repeat-a`, `srir-repeat-b` | 42.0 MiB | 140 | unreachable; the `.1` repeat control, published | reclaimed |
| `spec-to-intent-f-iii-heldout-corrected` | 132.6 MiB | 47 | superseded chain link at `5dd1302a`; nothing chains through it | reclaimed |
| `spec-to-intent-f-iii-heldout-qualified` | 137.2 MiB | 48 | superseded chain link at `5dd1302a` | reclaimed |
| `spec-to-intent-f-iii-heldout-qualified-r2` | 137.2 MiB | 49 | superseded chain link at `5dd1302a` | reclaimed |
| `spec-to-intent-f-iii-heldout-qualified-r3` | 137.2 MiB | 48 | superseded chain link at `5dd1302a` | reclaimed |
| `spec-to-intent-f-iii-i2c-alpha-diagnostic` | 38.1 MiB | 12 | unreachable; no holdout evidence, no tracked name | reclaimed |
| `spec-to-intent-f-iii-i2c-alpha-diagnostic-r2` | 42.7 MiB | 13 | unreachable; no holdout evidence, no tracked name | reclaimed |
| `source-ir-census-plan.json`, `source-ir-census-external-source-map.json` | 0.1 MiB | 2 | unreachable runtime inputs, deterministically regenerable | reclaimed |
| `.tmpxvj63h` | 0.0 MiB | 2 | stray temporary | reclaimed |
| `pre-reingest-snapshot-2026-09-19` | 184.5 MiB | 21 | named by `CORPUS-CHAIN-CURRENCY` and `RETAINED-BUNDLE-POPULATION-FROZEN` (`.6`) | retain |
| `tmp.dAMoRvjZ1W` | 276.9 MiB | 32 | unreachable; a `2026-09-11` seal-restore pre-write snapshot, restore long committed (`.6`) | reclaimed |
| `gate-cost`, `kg-5iv-a`, `fcc_plan.json`, `fcc_plan.err`, `planned.json` | 0.0 MiB | 6 | unreachable `2026-08-31`/`09-01` probe outputs (`.6`) | reclaimed |
| `req5`, `req6`, `req15`, `req16`, `req19`, `req20` | 0.1 MiB | 29 | named by the `env.txt` of the FSMGen packets under `docs/upstream/fsmgen/` (`.8`) | retain |
| `req21` | 0.0 MiB | 9 | unnamed; the reset-polarity probe cases of `EXTRACTION-REPRESENTATION-GAPS.21`, which re-runs them at the next pin (`.8`) | retain while `.21` is open |
| `reqkit`, and the `*.sh`/`*.py` in `erg7` | 0.0 MiB | 12 | unnamed; the packet-building and chain-replay scripts the open `EXTRACTION-REPRESENTATION-GAPS` leaves reuse (`.8`) | retain |
| `erg7` data (`snap*`, `new*`, `probe*`, `wire`, `*.out`, `axi_adapt_dry.txt`), `probe` | 2,029.4 MiB | 364 | unreachable; snapshots and drafts of the committed `EXTRACTION-REPRESENTATION-GAPS.7.b`/`.22`/`.25.a` and the signed-off packets (`.8`) | `.9`: removal refused by the session's permission policy |

The four superseded `5dd1302a` links are safe precisely because the chain is traced rather than assumed:
`-qualified-r5` links directly to `-heldout`, not through them, so no walk can reach them. `torchinductor` and
`xcrun_db` are OS/toolchain caches the environment recreates and are left in place.

## Planned children

- ID: `SCRATCH-RESIDUE-CONTAINMENT.0`
  State: `done` (`2026-08-27`)
  Goal: establish the reachability model and reclaim everything provably unreachable
  Acceptance: every root under `.project-data/tmp/` is classified by named-or-chained reachability with its
  size and file count stated; only unreachable roots are removed; a residue census proves each removed path is
  gone and each retained path is intact, including the declared `retained_evidence_sha256`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.1`
  State: `pending`
  Goal: decide the disposition of the two chain-bearing behavioral-holdout roots
  Acceptance: a decision on `spec-to-intent-f-iii-heldout` (7.4 GiB, the retained artifact store a future
  qualification reuse would read) and `spec-to-intent-f-iii-heldout-qualified-r5` (6.7 GiB, the run behind the
  committed aggregate) that states what a reuse or re-derivation would need, and either retains them under an
  explicit declaration or reclaims them with a stated regeneration route and cost
  Prerequisite: `SCRATCH-RESIDUE-CONTAINMENT.0`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.2`
  State: `pending`
  Goal: make the reachability sweep executable instead of manual
  Acceptance: a checker enumerates scratch roots, resolves named and chained reachability, and reports
  unreachable roots with their size; a RED control proves a chain-reachable root is never reported unreachable
  Prerequisite: `SCRATCH-RESIDUE-CONTAINMENT.1`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.3`
  State: `done` (`2026-08-29`)
  Goal: own and reclaim the fixture residue under `generated/` that no owner reaches
  Acceptance: routed here from `SOURCE-IR-REPRODUCIBILITY.16`'s resume pointer, which recorded the class as
  unowned. Both legs of `.0`'s reachability model are answered for it, the reclamation is censused before and
  after, and the producer is proved unaffected — not merely assumed to be.
  **Reachability.** Named: `git grep live-document-size-tests` returns only the producer script and two prose
  lines describing the residue itself; no tracked file names any individual root. Chained: the retained-evidence
  walk resolves under `.project-data/tmp`, never `generated/`, so no chain reaches them. Both legs negative.
  **Census.** Standing residue at `2026-08-29`: **138 directories, 3,449 files, 15 MB**, in five mtime clusters
  (53/53/30/1/1), holding 2,080 `.md`, 276 `.sh`, 276 `.jsonl`, 138 `.json`, 138 `.log`, 138 `.tsv`, 138 `.txt`
  and 10 fixture Git repositories. Reclaimed together with this leaf's own experiment output: **317 directories,
  7,630 files, 32 MB removed, 0 remaining** by name census. The producer then re-ran clean: **84/84 pass, exit
  0, leak 0**.
  **The mechanism is now measured, not inferred.** `.15` recorded "a kill bypasses File::Temp's END cleanup" as
  a hypothesis and forbade recording a trigger without measuring one. Measured here by direct control, killing
  the producer at a known live-fixture count: **SIGKILL leaks every live fixture (15/15, 15/15, 20/20)** and
  **SIGTERM leaks every live fixture (15/15, 15/15, 20/20)**. The suite installs no `%SIG` handler and relies
  solely on `tempdir(CLEANUP => 1)`, whose cleanup is an END block a signal death never reaches.
  A **third, incidental reproduction** arrived unasked: this session's own 2-minute harness timeout killed a
  verification run mid-suite and leaked **50**.
  **The asymmetry that explains why this residue is rare.** Killing the *wrapper*
  (`scripts/check_live_document_size.sh`) and leaving the producer orphaned leaks **0** — the orphan runs to
  normal exit and cleans up. So an interrupted gate does not normally leak; only a signal that reaches the perl
  process itself does. The observed cluster sizes (53/53/30/1/1), all below the run's 84-fixture peak, fit that
  shape exactly. **(The 84 is correct as of this node's date and has since drifted: the suite is 116 checks
  with a measured 114-fixture peak as of `2026-09-21`. `.4` restated the 84 as current and drew an invalid
  inference from it — re-measure the peak, or count runs by the once-per-run verifier store instead.)**
  **Honestly unmeasured:** SIGINT, the Ctrl-C shape. A background child of a non-interactive shell inherits
  `SIG_IGN` for SIGINT, so the probe could not deliver it and the one apparent `leaked=0` reading for SIGINT is
  an artifact of the harness, not a property of the suite. It is not published as a result.
  **Gate exposure, corrected from the resume pointer.** `.16`'s pointer said "no gate sees them". No gate
  *judges* them, but `scripts/check_persisted_artifact_paths.pl` — run by `PROJECT-DATA-LOCALITY` — walks every
  `*.json` under `generated/` and classified their 138 as `other`. Observed `2026-08-29`: a fixture run
  concurrent with the gate deleted them mid-walk and the gate FAILED with 20 `cannot read canonical artifact`
  lines. That is a live nondeterministic-failure surface, not a tidiness issue, and it is why this class needed
  an owner rather than a note
  Prerequisite: `SCRATCH-RESIDUE-CONTAINMENT.0`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.4`
  State: `pending`
  Goal: stop the fixture residue being created, or give it a proportionate reclamation route
  Acceptance: `.3` reclaimed the standing residue and measured how it appears; it did not stop it recurring.
  Two candidate remedies, and the leaf must decide between them on evidence rather than take both: make the
  producer signal-safe (a `%SIG` handler for INT/TERM that runs the same cleanup END would), or give the
  fixture root a reclamation scope that does not discard the corpus. A RED control must prove the chosen
  remedy actually survives the kill shape `.3` measured, and the gate-walk exposure must be closed or
  explicitly accepted with a reason
  **Recurrence rate measured `2026-09-18`, which `.3` could not supply because it had just reclaimed.**
  Twenty days after `.3` reclaimed 317 directories, the standing residue was **175 directories / 18 MB**
  across both producers — `live-document-size-tests.*` and `derived-state-authority-tests.*` — with mtimes
  spanning `2026-09-01` to `2026-09-15` and nothing since. Reclaimed again by name census: **0 remaining**,
  and both producers then re-ran clean — `1..113` and `1..25`, leak **0** — so the producers are confirmed
  unaffected a second time. Roughly **9 directories a day** while sessions are active, and the residue is
  now known to accumulate from BOTH producers rather than the one `.3` named.
  **What that rate does to the choice between the two remedies, stated so the leaf starts from it rather
  than re-deriving it.** A `%SIG` handler cannot catch **SIGKILL**, and SIGKILL is one of the two shapes
  `.3` measured leaking 15/15 — and it is the shape a harness timeout produces, which is how `.3`'s own
  incidental 50-fixture leak happened. So the signal-safe remedy provably cannot close the measured
  population on its own, and the leaf should say so explicitly rather than ship a partial fix as a whole
  one. A reclamation route closes nothing automatically. A third shape neither option names — the producer
  reclaiming **stale siblings of its own prefix at startup** — is immune to every signal by construction,
  because it does not ask the dying process to do anything; its cost is that it deletes under `generated/`
  while `scripts/check_persisted_artifact_paths.pl` may be walking, which is the same nondeterministic gate
  failure `.3` observed and is the exposure this leaf must close either way.
  **Third reclamation `2026-09-21`, and it REFUTES the rate model above.** Three days after the
  `2026-09-18` census took the residue to zero, the standing population was **104 directories / 2,980
  files / 1.28 MB** — but **every mtime falls on a single day, `2026-09-20`**, and nothing since. So
  this is not "roughly 9 directories a day"; it is **bursty**.
  **CORRECTION, same day, and the correction is the useful part.** This record first argued "101 of
  the 104 exceed the suite's own 84-fixture peak, so that day carried at least two killed runs". The
  **84 was `.3`'s number and `.3` is dated**: the suite has grown to 116 checks, and its peak
  concurrent live fixtures measured **114** today (counted by polling `generated/` during a real run,
  which also re-confirmed leak 0 on normal exit). **101 < 114, so a single killed run explains every
  one of them** and the inference was invalid — an instance of this repository's own standing rule
  that a handed-down number must be re-derived, not restated.
  **The right instrument was already in the residue.** `live-document-size-verifiers.*` is created by
  a memoised `$verifier_store ||= tempdir(...)` (`scripts/test_live_document_size.pl:61`), so there is
  **exactly one per process**. Three of them were standing, so `2026-09-20` carried **at least three**
  killed runs, and 101 fixture directories spread over them averages ~34 — comfortably inside the 114
  peak and consistent with runs dying at different points. **Count runs with the once-per-run
  artifact, never by dividing a fixture count by a remembered peak.**
  The distinction is decisive for the choice this leaf owes: a steady rate would argue for a
  reclamation scope, a burst from kills argues for making the producer survive the kill — except that
  SIGKILL, which a harness timeout sends, defeats a `%SIG` handler, so the burst shape actually
  strengthens the **third** option above, the producer reclaiming stale siblings of its own prefix at
  startup.
  **A THIRD producer prefix, which this leaf does not name.** `live-document-size-verifiers.*`
  (3 of the 104) is created at `scripts/test_live_document_size.pl:61` by the same
  `tempdir(..., CLEANUP => 1)` idiom and leaks the same way. `.4` names two producers; there are
  three, and the remedy must be chosen per IDIOM rather than per prefix or the next one is missed.
  **Producers proved unaffected a third time**: reclaimed by name census to **0 remaining**, then
  `perl scripts/test_live_document_size.pl` re-ran **116/116, exit 0, leak 0** (the suite was 113
  checks at the second measurement, so the declaration has grown with the surface).
  Prerequisite: `SCRATCH-RESIDUE-CONTAINMENT.3`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.5`
  State: `done` (`2026-08-31`)
  Goal: make the agent-harness scratch conflict discoverable before an agent writes off-volume, not after
  Acceptance: this tree has been about scratch the repository CREATES. The complementary hazard is scratch an
  agent is INSTRUCTED to create somewhere else: an interactive harness can hand a session a scratchpad
  directory outside the repository volume and tell it to use that for all temporary files, which is exactly
  what `PROJECT_DATA_LOCALITY.md` forbids ("must never default to `/private/tmp`, `/tmp`, user-home caches, or
  another off-volume location"). No gate can see this — the files never enter the repository, so
  `check_project_data_locality` has nothing to walk, and the residue is invisible to the census `.3` built.
  The only workable control is retrieval: the fact must be findable in the Knowledge Map at the moment an
  agent is deciding where to put a temporary file, which is before it has any reason to open
  `PROJECT_DATA_LOCALITY.md`. Acceptance is a fact card answering the question in the words an agent would
  actually use, no bound or gate changes, and no claim that this is mechanically enforced — it is not
  **Observed live, which is why it is worth recording (`2026-08-31`).** The `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e`
  session was handed an off-volume scratchpad by its harness and wrote three partition-verification files there
  before catching the conflict; they were moved to `.project-data/tmp/` and the off-volume copies deleted, with
  an empty residue check. The failure mode is not carelessness — the harness instruction is explicit and
  arrives before the repository's own policy is read, so ordinary compliance with one rule breaks the other.
  Verification: `fact card project-scratch-location added, answering where a temporary file goes and naming
  .project-data/tmp/ as the repository-volume root (gitignored except .gitkeep, per PROJECT_DATA_LOCALITY.md's
  temporary-workspaces row); off-volume residue from the observing session removed and re-censused empty; no
  surface bound, gate, or registry dimension changed`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.5 — record the off-volume harness-scratchpad hazard where retrieval finds it`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.6`
  State: `done` (`2026-09-23`)
  Goal: the directive's daily sweep, re-run under `.0`'s model, plus the incremental-cache class it names
  Acceptance: every current root under `.project-data/tmp/` classified named-or-chained with size and file count;
  only unreachable roots removed; residue census proves them gone and every retained root intact
  **Scratch roots.** The chain is unchanged: the doctrine aggregate names `spec-to-intent-f-iii-heldout`, whose
  own evidence file carries no `retained_evidence_path`, so it remains the one-hop terminus. Named roots were
  re-enumerated with `.0`'s `git grep`. Seven roots were new since `.0`: `pre-reingest-snapshot-2026-09-19` is
  named by two task trees and is retained; `tmp.dAMoRvjZ1W` (276.9 MiB / 32 files, a seal-restore pre-write
  snapshot of `2026-09-11`) and five 0.0 MiB probe outputs are unreachable and are reclaimed. The table above is
  updated in place, per the protocol.
  **Incremental compilation cache — the directive names `target/debug/incremental`.** rustc loads only the newest
  finalized session of each unit and garbage-collects the rest on that unit's next build; a superseded session
  is therefore unreachable by construction. 90 unit directories held 53 superseded finalized sessions and one
  orphaned `-working` session (an interrupted build from the crashed session). Kept the newest finalized
  session of every unit (89) and removed the other 54 with their locks: **5.63 GiB freed, 12 GiB -> 7.1 GiB**,
  with no rebuild cost for any unit that still had a finalized session. The guard refused to run while any
  `cargo`/`rustc`/`clippy-driver` process had its working directory in this repository (two process-name
  matches were other projects' builds, and a command-line match was the guard's own shell — hence matching on
  the executable name and its cwd, never on the command line). **Ordering, stated:** the cache pruning ran before
  this leaf was written; it touched no tracked file, and this leaf records it rather than presenting it as
  planned. `target/release` and `target/debug/deps` hold no `.bin`/`.log` files.
  Verification: `residue census after deletion: the six reclaimed roots are absent; spec-to-intent-f-iii-heldout,
  -qualified-r5, -external-sources, signal-catalog-capture-gap-1 and pre-reingest-snapshot-2026-09-19 present
  with their sizes unchanged; target/debug/incremental 7.1 GiB with one finalized session per unit`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.6 — the daily sweep, and the cache sessions rustc can no longer reach`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.7`
  State: `done` (`2026-09-24`)
  Goal: the session-startup cleanup directive gets its one-entry record file, and the sweep it triggers runs
  Acceptance: `docs/ARTIFACT_CLEANUP.md` exists, holds exactly one overwrite-only entry (date + one-line summary),
  and points here for the rules; the sweep follows `.0`'s reachability model and `.6`'s incremental-cache rule
  **Registered.** The file is a `bounded_snapshot` surface (`artifact_cleanup_record` in
  `doctrine/live_document_size/surfaces.jsonl`, 16 lines / 2,048 bytes, beside `active_resume`); `LIVE-DOC-SIZE`
  refused it unclassified on the first commit attempt, then `CLAIM-VERIFICATION` required its census disposition:
  `excluded` as `operational_maintenance_record` (it publishes no product claim), denominator 56 -> 57. Digests re-pinned.
  **Why now.** The directive tells a session to sweep when that file is older than 24 hours *or missing*. It was
  missing — `.6`'s sweep (`2026-09-23 16:11`) was recorded only in this tree — so the rule fired about ten hours
  after the last sweep. The file now carries the date, so the next session can apply the 24-hour rule directly.
  **Scratch roots.** No root is new since `.6`. The chain is unchanged: the doctrine aggregate names
  `spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`, whose sha256 matches the declared
  `retained_evidence_sha256` (`4a05ec3d...`) and which carries no `retained_evidence_path`. Nothing reclaimed.
  **Incremental cache.** 90 unit directories held 22 superseded finalized sessions and no `-working` session.
  Removed the 22 with their locks, keeping the newest finalized session of every unit: **2,831,972 KiB (2.70 GiB)
  freed, 9.0 -> 7.1 GiB**. One unit directory (`specforge_core-0900plsb923ab`) was already empty before the sweep
  (mtime `2026-09-23 16:04`) and was left for rustc to manage. The three running `cargo`/`rustc` processes had
  their working directory in another repository, checked by cwd rather than by command line.
  `target/release`, `target/debug/incremental` and `target/debug/deps` hold no `.bin`/`.log` files.
  Verification: `all 22 removed session paths absent; every other unit has exactly one finalized session; retained
  roots present; chain sha256 re-derived equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.7 — the cleanup record the startup directive reads, and the sweep it fired`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.8`
  State: `done` (`2026-09-25`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-09-24 02:10 +0200`; the session started `2026-09-25 16:39 +0200`, 38 hours later.
  **Scratch roots.** The chain is unchanged: the doctrine aggregate names
  `spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`, whose sha256 re-derives equal to the declared
  `4a05ec3d...` and which carries no `retained_evidence_path`. Ten roots are new since `.7`, all from the
  `EXTRACTION-REPRESENTATION-GAPS` lane. Six are named by the packets' `env.txt` and are retained; `req21` holds
  the probe cases the open `.21` re-runs at the next pin, and `reqkit` plus the scripts in `erg7` are the lane's
  tooling, so both are kept although unnamed. `erg7`'s data and the `probe` drafts (2,029.4 MiB, 364 files) are
  unreachable, but their removal was refused by the session's permission policy, so they stay in place and `.9`
  owns them.
  **Incremental cache.** 111 unit directories held 49 superseded finalized sessions, no `-working` session and 75
  zero-byte locks with no session. Session order by mtime and by the name's base-36 timestamp agreed on every
  unit. Removed the 49 with their locks and the 75 orphan locks, keeping the newest finalized session of every
  unit: **13,976,156 -> 9,075,712 KiB (13.3 -> 8.7 GiB)**, measured before and after. (The 49 sessions sum to
  5.6 GiB one by one, more than was freed: 7,573 of the 9,388 files left there have more than one link, so a
  removed session frees only the inodes nothing else links.) The one
  running `cargo` process had its working directory in another repository. `target/release` and
  `target/debug/deps` hold no `.bin`/`.log` files, and the `.bin` files under `target/debug/incremental` are the
  retained sessions' own caches.
  Verification: `every unit has exactly one session and no orphan lock; retained roots present; chain sha256
  re-derived equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.8 — the startup sweep: 4.7 GiB of rustc sessions, and the scratch it may not delete`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.9`
  State: `done` (`2026-09-28`; the owner granted the deletion on `2026-09-28`)
  Goal: remove the unreachable `erg7` data and `probe` drafts `.8` classified (2,029.4 MiB, 364 files)
  Acceptance: `.8`'s deletion was refused by the session's permission policy as irreversible local destruction,
  so it waits for the owner to allow it or to remove the paths themselves. The paths are in `.8`'s table row; the
  scripts in `erg7` stay. Re-check reachability (`git grep`, the chain) on the day it runs, then census.
  **Done.** Re-checked the day it ran: no tracked file names `erg7` or `probe`, and the retained-evidence chain still
  ends at `spec-to-intent-f-iii-heldout`. Removed exactly `.8`'s classes — `erg7`'s `snap*`, `new*`, `probe*`, `wire`,
  `*.out` and `axi_adapt_dry.txt`, and the `probe` root: **32 paths, 364 files, 2,028.6 MiB** (the same 364 files `.8`
  counted; the byte total now excludes the scripts). `erg7` keeps its 9 scripts (`*.sh`, `*.py`, 36 KiB).
  Verification: `every removed path absent; erg7 holds only its 9 scripts; probe absent; chain sha256 re-derived
  4a05ec3d…, equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.9 — the owner-approved deletion: erg7's 2.0 GiB of unreachable scratch`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.10`
  State: `done` (`2026-09-26`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-09-25 16:55 +0200`; the sweep ran at `2026-09-26 17:03 +0200`.
  **Scratch roots.** 19 roots from the `EXTRACTION-REPRESENTATION-GAPS` lane (`erg7c`, `erg7d`, `erg10`, `erg10e`,
  `erg10g`, `erg10h`, `erg11a`, `erg17`, `erg24b`, `erg25c`, `erg26`, `erg26b`, `erg29`-`erg35`) hold the per-slice
  snapshots of committed leaves whose chains were rebuilt and verified current. `git grep` for each root's path
  returned nothing today and none is in the chain, so their data subdirectories went: **38 directories and 2 stray
  `evidence.rs` copies at the top level (`erg17_block.rs`, `erg17_evidence.rs.bak`), 1,192 files, 10,490.7 MiB**.
  Each root's top-level logs and scripts stay, as `.8` kept `erg7`'s. Retained: `spec-to-intent-f-iii-heldout`
  (the chain), `-qualified-r5` (`.1` decides), `erg7`'s data (`.9`, the owner's permission), and the named `erg3`,
  `erg3a`, `erg24`, `erg27`, `pre-reingest-snapshot-2026-09-19`, the `req*` packets and `reqkit`.
  **Incremental cache.** 112 units held 39 superseded finalized sessions, no `-working` session and 150 locks with no
  session; mtime order and the names' base-36 timestamps agreed on every unit and no `cargo` or `rustc` ran. Removed
  the 39 and the 150, keeping each unit's newest session: **13,408,680 -> 9,259,044 KiB (12.8 -> 8.8 GiB)**.
  `target/release` and `target/debug/deps` hold no `.bin`/`.log` file.
  Verification: `every removal path absent; every unit has exactly one session and no orphan lock; chain sha256
  re-derived 4a05ec3d…, equal; retained roots present`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.10 — the daily sweep: 10.2 GiB of per-slice snapshots and 4.0 GiB of rustc sessions`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.11`
  State: `done` (`2026-09-28`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-09-26 17:03 +0200`; the sweep ran at `2026-09-28 11:08 +0200`.
  **Scratch roots.** The chain is still one hop to `spec-to-intent-f-iii-heldout` (evidence `4a05ec3d…`). 23 roots are
  unnamed and off the chain: the 19 `.10` swept, plus `erg38`, `fsmreq6` and `egf9` from committed leaves since, and
  `erg7e` (nothing in it matched). In them the sweep (`.project-data/tmp/src11/sweep.py`, plan then `--apply`) removed every data
  subdirectory and, beyond `.10`, the top-level copies of old `specforge` executables and of `*.rs`/`*.bak` sources,
  all reproducible from Git: **53 paths, 114 files, 632.8 MiB**. Logs, scripts (a `*probe*.rs` stays) and small
  outputs stay. Retained: the chain terminus, `-qualified-r5` (`.1`), `erg7`'s data and `probe` (`.9`), `reqkit`,
  `req21` (`EXTRACTION-REPRESENTATION-GAPS.21` open), and every named root (`egf7`, `erg3`, `erg3a`, `erg24`, `erg27`,
  `pre-reingest-snapshot-2026-09-19`, the `req*` packets, `signal-catalog-capture-gap-1`,
  `spec-to-intent-external-sources`).
  **Incremental cache.** No process held `target/debug/incremental` open (the one running `cargo` belonged to another
  repository). 112 units: 22 superseded finalized sessions and 44 locks with no session, no `-working` session, and the
  base-36 name timestamps agreed with mtime order on every unit. Removed them, keeping each unit's newest session:
  **11,548,200 -> 9,237,196 KiB (11.0 -> 8.8 GiB)**. `target/release` and `target/debug/deps` hold no `.bin`/`.log`.
  Verification: `every removal path absent; the re-plan is empty on both scripts; every retained root present; chain
  sha256 re-derived 4a05ec3d…, equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.11 — the daily sweep: 632.8 MiB of scratch and 2.2 GiB of rustc sessions`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.12`
  State: `done` (`2026-09-29`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-09-28 11:08 +0200`; the sweep ran at `2026-09-29 11:40 +0200`, after the slice in
  flight at the 24-hour mark had committed.
  **Scratch roots.** The chain is still one hop: the doctrine aggregate names
  `spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`, whose sha256 equals the declared `4a05ec3d…` and
  which names no further hop. The sweep (`.project-data/tmp/src12/sweep.py`, plan then `--apply`) now classifies every
  root itself: 7 retained (the chain terminus, `-qualified-r5` `.1`, `erg7` `.9`, `reqkit`, `req21` while
  `EXTRACTION-REPRESENTATION-GAPS.21` is open, a tool cache, the sweep's own directory), 18 named, and 31 unreachable.
  In the unreachable roots it removed every data subdirectory and top-level executable or source copy: **27 paths, 143
  files, 926.2 MiB**. Logs, scripts and small outputs stay. `erg3cii` is named by `.3.c.ii`'s record, which cites a
  patch saved there, and is retained whole.
  **Incremental cache.** The one running `cargo` had its working directory in another repository. 112 units: 25
  superseded finalized sessions and 57 locks with no session; no `-working` session; every unit's base-36 name
  timestamp agreed with mtime order. Removed them, keeping each unit's newest session: **11,498,616 -> 9,297,964 KiB
  (11.0 -> 8.9 GiB)**. `target/release` and `target/debug/deps` hold no `.bin`/`.log`.
  Verification: `every removal path absent; the re-plan is empty on both scripts; every retained root present; chain
  sha256 re-derived 4a05ec3d…, equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.12 — the daily sweep: 926.2 MiB of scratch and 2.1 GiB of rustc sessions`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.13`
  State: `done` (`2026-10-03`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-09-29 11:40 +0200`; the sweep ran at `2026-10-03 01:27 +0200`, at session start
  with a clean tree.
  **Scratch roots.** The chain is still one hop: the doctrine aggregate names
  `spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`, whose sha256 equals the declared `4a05ec3d…` and
  which names no further hop. `.12`'s sweep, copied to `.project-data/tmp/src13/sweep.py` with only its own directory
  substituted in the retained set (plan, then `--apply`), classifies 7 retained, 20 named (`src12` joined, cited by
  `.12`'s record) and 42 unreachable roots. In the unreachable roots it removed every data subdirectory and top-level
  source or backup copy that had accumulated since `.12`: **16 paths, 135 files, 143.8 MiB**, 141.1 MiB of it the
  per-slice snapshots under `erg3cvii`. Logs, scripts and small outputs stay. No executable copy was found.
  **Incremental cache.** No `cargo` was running. 112 units: 22 superseded finalized sessions and 44 locks with no
  session; no `-working` session; every unit's base-36 name timestamp agreed with mtime order. Removed them, keeping
  each unit's newest session: **12,291,900 -> 9,447,364 KiB (11.7 -> 9.0 GiB)**. `target/release` and
  `target/debug/deps` hold no `.bin`/`.log`.
  Verification: `every removal path absent; the re-plan is empty on both scripts; every retained root present; chain
  sha256 re-derived 4a05ec3d…, equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.13 — the daily sweep: 143.8 MiB of scratch and 3.0 GiB of rustc sessions`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.14`
  State: `done` (`2026-10-19`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-10-03 01:27 +0200`; the sweep ran at `2026-10-19 22:59 +0200`, at session start
  with a clean tree.
  **Scratch roots.** The chain is still one hop: the doctrine aggregate names
  `spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`, whose sha256 equals the declared `4a05ec3d…` and
  which names no further hop. `.13`'s sweep, copied to `.project-data/tmp/src14/sweep.py` with only its own directory
  substituted in the retained set, classifies 7 retained, 21 named (`src13` joined, cited by `.13`'s record) and 42
  unreachable roots, and plans **0 paths**: no root is new since `.13` and nothing has accumulated in the unreachable
  ones. `generated/` holds no `live-document-size-tests.*`, `-verifiers.*` or `derived-state-authority-tests.*` fixture
  root (`.3`/`.4`'s class). `req21` stays while `EXTRACTION-REPRESENTATION-GAPS.21` is open.
  **Incremental cache.** The running `cargo` and `rustc` had their working directory in another repository. 112 units:
  18 superseded finalized sessions and 36 locks with no session; no `-working` session; every unit's base-36 name
  timestamp agreed with mtime order. Removed them, keeping each unit's newest session: **11,064,684 -> 9,441,428 KiB
  (10.6 -> 9.0 GiB)**; the plan summed 2.39 GiB, and hard links shared with the retained sessions explain the
  difference, as `.8` measured. `target/release` and `target/debug/deps` hold no `.bin`/`.log`.
  Verification: `every removal path absent; the re-plan is empty on both scripts; every retained root present; chain
  sha256 re-derived 4a05ec3d…, equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.14 — the daily sweep: no scratch to reclaim, and 1.5 GiB of rustc sessions`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.15`
  State: `pending` (`2026-10-20`; found by `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.b`)
  Goal: **every self-test fixture root under `generated/` is in the residue census, and no producer leaks one.**
  `.14` recorded "no fixture residue under `generated/`", but its census named only `.3`/`.4`'s three prefixes
  (`live-document-size-tests.*`, `-verifiers.*`, `derived-state-authority-tests.*`). **Reproduce:**
  `ls -d generated/.*self-test* generated/.knowledge-map-check.*` lists **31 roots / 4,170 files / 26,908 KiB**
  from six producers, mtimes `2026-08-08` to `2026-09-29`: `scripts/check_corpus_kb_currentness.pl` 10,
  `check_book_quantitative_claims.pl` 6, `check_claim_verification.pl` 5, `check_fact_card_catalog.pl` 5,
  `check_current_claim_census.pl` 4, `knowledge-map/scripts/check_knowledge_map.sh` 1. The Perl self-tests remove
  their fixture only after every case passes (`check_book_quantitative_claims.pl`'s `run_self_test` reaches
  `remove_tree` last), so a failed case or a killed run leaves it; the knowledge-map check traps `EXIT HUP INT TERM`
  and leaks only under SIGKILL. **Impact:** the residue grows unseen, and a root under `generated/` deleted
  mid-walk is `.3`'s nondeterministic gate failure.
  Acceptance: widen the sweep's census to every dot-prefixed fixture root under `generated/` (derive the prefixes
  from the producers, not a list), reclaim the 31 after the reachability check, and give each producer `.4`'s chosen
  remedy (cleanup on every exit path, or stale-sibling reclamation at startup) with a RED control per producer.

- ID: `SCRATCH-RESIDUE-CONTAINMENT.16`
  State: `done` (`2026-10-21`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-10-19 22:59 +0200`; the sweep ran at `2026-10-21 00:07 +0200`, at session
  start with a clean tree.
  **Scratch roots.** The chain is still one hop to `spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`,
  sha256 `4a05ec3d…`, equal to the declaration. `.14`'s sweep, copied to `.project-data/tmp/src16/sweep.py` with
  only its own directory substituted in the retained set, classifies 7 retained, 30 named (`src14` and the eight
  `erg-*` roots that task records cite joined) and 56 unreachable roots. It removed **60 paths / 1,107 files /
  7,803.9 MiB** that 11 roots accumulated since `.14`: 36 data directories (7,638.8 MiB), 9 executables (141.9 MiB) and 15
  source/backup copies (22.6 MiB); logs and scripts stay, so `erg-3diibiii`'s dry-run, seed and bookkeeping helpers
  are intact. `.project-data/tmp` went **36,423,504 -> 28,430,788 KiB**. The 31 fixture roots under `generated/`
  are `.15`'s, still pending, and were not touched.
  **Incremental cache.** No `cargo` or `rustc` was running. 127 units: 35 superseded finalized sessions and 75 locks
  with no session; no `-working` session; every unit's base-36 name timestamp agreed with mtime order. Removed them,
  keeping each unit's newest session: **13,788,464 -> 10,282,456 KiB (13.1 -> 9.8 GiB)**; the plan summed 4.05 GiB
  and hard links shared with retained sessions explain the difference. `target/release` and `target/debug/deps`
  hold no `.bin`/`.log`.
  Verification: `every removal path absent; the re-plan is empty on both scripts; every retained root present; chain
  sha256 re-derived 4a05ec3d…, equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.16 — the daily sweep: 7.6 GiB of scratch and 3.3 GiB of rustc sessions`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.17`
  State: `done` (`2026-10-22`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-10-21 00:07 +0200`; the sweep ran at `2026-10-22 07:25 +0200`, between commits
  with a clean tree. **Scratch roots.** The chain is still one hop to
  `spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`, sha256 `4a05ec3d…`, equal to the declaration.
  `.16`'s sweep, copied to `.project-data/tmp/src17/sweep.py`, retains three in-flight roots beside the declared
  ones: `fsmgen-c51e22712` (the upstream export `FSMGEN-REQUESTS.15` needs), `erg-3civa` (the HEAD seed baseline)
  and `erg-3civb`. It classifies 10 retained, 31 named and the rest unreachable, and removed **118 paths / 3,237
  files / 22,556.8 MiB** from the unreachable roots' data directories, executables and source copies; logs and
  scripts stay, so the bookkeeping, dry-run and oracle helpers are intact. `.project-data/tmp` went **52,203,704 ->
  29,100,740 KiB**. **Incremental cache.** No `cargo` or `rustc` ran in this repository (one ran in another
  repository, with its own `target/`). 128 units: 22 superseded finalized sessions and 47 locks with no session, no
  `-working` session; removed them, keeping each unit's newest session: **13,809,268 -> 10,761,124 KiB**.
  **`generated/`.** A gate run this session killed mid-way left 196 fixture roots (`live-document-size-tests.*` 126,
  `derived-state-tests.*` 46, `derived-state-authority-tests.*` 24, `live-document-size-verifiers.*` 1, all created
  that hour); they were removed. `.15`'s dot-prefixed roots are 33 now, two more since its census
  (`corpus-kb-currentness`, `2026-10-21`), and stay `.15`'s.
  Verification: `every removal path absent; the re-plan is empty on both scripts; every retained root present; chain
  sha256 re-derived 4a05ec3d…, equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.17 — the daily sweep: 22.0 GiB of scratch and 2.9 GiB of rustc sessions`
  **Corrected `2026-10-22`.** No leaf `FSMGEN-REQUESTS.15` exists: `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b` first
  named one for port sub-field layouts and withdrew it the same day (`d0d342c8c`, ADR 0033 §4). The export served that
  draft and `FSMGEN-REFRESH-INTEGRATE-10`'s strict census of upstream `c51e22712`, and is removed since.

- ID: `SCRATCH-RESIDUE-CONTAINMENT.18`
  State: `done` (`2026-10-23`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-10-22 07:25 +0200`; the sweep ran at `2026-10-23 07:42 +0200`, between commits
  with a clean tree. **Scratch roots.** The chain is still one hop to
  `spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`, sha256 `4a05ec3d…`, equal to the declaration.
  `.17`'s sweep, copied to `.project-data/tmp/src18/sweep.py`, retains one in-flight root beside the declared ones:
  `erg-65a`, whose `seeds_new` is the HEAD seed baseline. `.17`'s three in-flight roots are no longer in flight
  (`fsmgen-c51e22712` was removed with `.17`'s correction; `erg-3civa` and `erg-3civb` served closed leaves). It
  classifies 8 retained, 43 named and the rest unreachable, and removed **45 paths / 890 files / 6,920.3 MiB** from
  the unreachable roots' data directories, executables and source copies; logs and scripts stay, so the bookkeeping,
  dry-run and oracle helpers are intact. Inside `erg-65a` the cascade and SourceIR snapshots of the committed slice
  `a6ab3ebd4` and its two executable copies went too (**5 paths / 203 files / 1,706.6 MiB**), keeping `seeds_new`
  and the logs. `.project-data/tmp` went **41,245,824 -> 32,410,576 KiB**. **Incremental cache.** No `cargo` or
  `rustc` was running. 128 units: 19 superseded finalized sessions and 53 locks with no session, no `-working`
  session; removed them, keeping each unit's newest session: **13,717,980 -> 10,936,396 KiB**. **`generated/`.** No
  killed-run fixture roots; `.15`'s dot-prefixed roots are 33, unchanged, and stay `.15`'s.
  Verification: `every removal path absent; the re-plan is empty on both scripts; every retained root present; chain
  sha256 re-derived 4a05ec3d…, equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.18 — the daily sweep: 8.4 GiB of scratch and 2.7 GiB of rustc sessions`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.19`
  State: `pending` (`2026-10-23`; found by `RUSTDOC-AND-XTASK.14.a`)
  Goal: **no generator's working file is ever tracked.** **Reproduce:** `git ls-files docs/knowledge-map/` lists three
  files `knowledge-map/scripts/gen_knowledge_map.sh` writes as its working set and removes on exit
  (`.questions-identity.72627.tmp` 5,918 bytes, `.questions-part.72627.tmp` 7 lines, `.questions-rows.72627.tmp`
  509 lines), all three added by `be58b45bf` (`EXTRACTION-REPRESENTATION-GAPS.39`, `2026-09-29 04:20:04 +0200`); their
  mtimes are `04:19`, so run 72627 was still alive or had been killed when `git add -A` staged the tree. The
  generator writes them beside the shards it publishes (`$output_shard_dir/.${KM_SHARD_PREFIX}*.$pid.tmp`) and its
  `trap cleanup EXIT HUP INT TERM` cannot run under SIGKILL, and nothing refuses a tracked `*.tmp`. **Impact:** a
  stale copy of the question rows travels with every clone, and the next leaked run is committed the same way.
  Acceptance: remove the three from the index and disk; give the generator a remedy `.4` names (stale-sibling
  reclamation at startup, or working files outside the tracked shard directory), fixed in the kit in place as
  `RUSTDOC-AND-XTASK.16` rules; and a mechanical refusal of a tracked generator working file, with a RED control.

- ID: `SCRATCH-RESIDUE-CONTAINMENT.20`
  State: `done` (`2026-10-24`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-10-23 07:42 +0200`; the sweep ran at `2026-10-24 07:42 +0200`, between commits
  with a clean tree. **Scratch roots.** The chain is still one hop to
  `spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`, sha256 `4a05ec3d…`, equal to the declaration.
  `.18`'s sweep, copied to `.project-data/tmp/src20/sweep.py`, no longer retains `erg-65a`: the HEAD seed baseline
  moved with `EXTRACTION-REPRESENTATION-GAPS.65.b` and `.67.a` (both `2026-10-23`, after `.18`) to
  `erg-67a/seeds_new`, the one path the sweep now protects inside an otherwise unreachable root; only tooling commits
  have landed since `.67.a`, so it is current. It classifies 7 retained and the rest named or unreachable, and
  removed **9 paths / 295 files / 2,115.7 MiB**: `erg-65a`'s superseded seeds, and the cascade, SourceIR and binary
  snapshots of the committed slices `.65.b` (`erg-65b`) and `.67.a` (`erg-67a`); logs and scripts stay.
  `.project-data/tmp` went **36,201,780 -> 34,034,800 KiB**. **Incremental cache.** No `cargo` or `rustc` ran in
  this repository. 171 units: 57 superseded finalized sessions and 126 locks with no session, no `-working` session;
  removed them, keeping each unit's newest session: **13,834,436 -> 11,879,304 KiB**. `target/release` and
  `target/debug/deps` hold no `.bin`/`.log`. **`generated/`.** A run killed at `2026-10-23 16:33` left three fixture
  roots (`live-document-size-tests.*` 2, `live-document-size-verifiers.*` 1; 47 files), removed; `.15`'s
  dot-prefixed roots are 33, unchanged, and stay `.15`'s.
  Verification: `every removal path absent; the re-plan is empty on both scripts; every retained root present, the
  seed baseline's 12 files intact; chain sha256 re-derived 4a05ec3d…, equal`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.20 — the daily sweep: 2.1 GiB of scratch and 1.9 GiB of rustc sessions`

- ID: `SCRATCH-RESIDUE-CONTAINMENT.21`
  State: `done` (`2026-10-27`)
  Goal: the startup sweep the directive fires when `docs/ARTIFACT_CLEANUP.md` is more than 24 hours old
  Acceptance: every root under `.project-data/tmp/` classified named-or-chained; only unreachable scratch and
  superseded rustc sessions removed; residue census proves them gone and every retained root intact
  **Why now.** The record read `2026-10-24 07:42 +0200`; the sweep ran at `2026-10-27 16:47 +0100`, after
  `RUSTDOC-AND-XTASK.15.1` committed with a clean tree. **Scratch roots.** The chain is still one hop to
  `spec-to-intent-f-iii-heldout/behavioral_holdout_evidence.json`, sha256 `4a05ec3d…`, equal to the declaration.
  `.20`'s sweep, copied to `.project-data/tmp/src21/sweep.py`, protects `erg-67a/seeds_new` and the declared
  retentions; its plan is **0 paths / 0 files / 0.0 MiB**, and the re-plan is empty. No scratch was removed;
  logs, scripts, named roots and all 12 seed-baseline files remain intact. **Incremental cache.** The one running
  `cargo` had its working directory in another repository. 171 units held three superseded finalized sessions and
  12 orphan locks, no `-working` session and no timestamp/mtime disagreement. Removed them, keeping each unit's
  newest session: **12,010,128 -> 11,907,228 KiB (102,900 KiB freed)**; the 0.27 GiB plan exceeds the freed bytes
  because sessions share hard links. `target/release` and `target/debug/deps` hold no `.bin`/`.log`.
  **`generated/`.** No killed-run fixture roots; `.15`'s 33 dot-prefixed roots are unchanged and stay `.15`'s.
  Verification: `both re-plans empty; every retained root present; seed baseline's 12 files intact; chain sha256
  re-derived 4a05ec3d…, equal; fixture and stray-artifact censuses empty`
  Commit: `SCRATCH-RESIDUE-CONTAINMENT.21 — the daily sweep: no scratch to reclaim, and 100.5 MiB of rustc sessions`

## Open Questions

- Should a scratch root that is only chain-reachable be required to carry its own declaration, so reachability
  is readable from tracked files rather than only by executing the resolver?

## Blockers

- None.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-10-27` | `.21` daily sweep | chain sha256 equal; scratch plan 0 paths and no fixture residue; 3 superseded rustc sessions and 12 orphan locks removed (12,010,128 -> 11,907,228 KiB); both re-plans empty; retained roots and all 12 seed-baseline files intact |
| `2026-10-24` | `.20` daily sweep | chain sha256 equal; 9 paths / 295 files / 2,115.7 MiB removed (a superseded seed baseline and two committed slices' snapshots), the HEAD seed baseline protected; 57 superseded rustc sessions and 126 orphan locks removed (13.2 -> 11.3 GiB); three killed-run fixture roots under `generated/` removed |
| `2026-10-23` | `.18` daily sweep | chain sha256 equal; 45 paths / 890 files / 6,920.3 MiB of unreachable scratch removed, logs and scripts kept, one in-flight root retained, whose committed slice's snapshots also went (5 paths / 203 files / 1,706.6 MiB); 19 superseded rustc sessions and 53 orphan locks removed (13.1 -> 10.4 GiB); no fixture residue under `generated/` |
| `2026-10-22` | `.17` daily sweep | chain sha256 equal; 118 paths / 3,237 files / 22,556.8 MiB of unreachable scratch removed, logs and scripts kept, three in-flight roots retained; 22 superseded rustc sessions and 47 orphan locks removed (13.2 -> 10.3 GiB); 196 fixture roots a killed gate run left under `generated/` removed |
| `2026-10-21` | `.16` daily sweep | chain sha256 equal; 56 unreachable roots, 11 of which accumulated scratch since `.14`; their data directories and executable/source copies removed (60 paths, 1,107 files, 7,803.9 MiB), logs and scripts kept; 35 superseded rustc sessions and 75 orphan locks removed (13.1 -> 9.8 GiB); every retained root present |
| `2026-10-19` | `.14` daily sweep | chain sha256 equal; 42 unreachable roots, nothing new to reclaim (plan 0 paths); no fixture residue under `generated/`; 18 superseded rustc sessions and 36 orphan locks removed (10.6 -> 9.0 GiB); every retained root present |
| `2026-10-03` | `.13` daily sweep | chain sha256 equal; 42 unreachable roots' data subdirectories and top-level source/backup copies removed (16 paths, 135 files, 143.8 MiB), logs and scripts kept; 22 superseded rustc sessions and 44 orphan locks removed (11.7 -> 9.0 GiB); every retained root present |
| `2026-09-29` | `.12` daily sweep | chain sha256 equal; 31 unreachable roots' data subdirectories and top-level executable/source copies removed (27 paths, 143 files, 926.2 MiB), logs and scripts kept; 25 superseded rustc sessions and 57 orphan locks removed (11.0 -> 8.9 GiB); every retained root present |
| `2026-09-28` | `.9` owner-approved deletion | reachability re-checked (no tracked name, chain unchanged); `erg7`'s data and `probe` removed (32 paths, 364 files, 2,028.6 MiB), its 9 scripts kept; chain sha256 equal |
| `2026-09-28` | `.11` daily sweep | chain sha256 equal; 23 unreachable roots' data subdirectories and top-level executable/source copies removed (53 paths, 114 files, 632.8 MiB), logs and scripts kept; 22 superseded rustc sessions and 44 orphan locks removed (11.0 -> 8.8 GiB); every retained root present |
| `2026-09-26` | `.10` daily sweep | chain sha256 equal; 19 unreachable lane roots' snapshot data and 2 stray copies removed (1,192 files / 10,490.7 MiB), their logs and scripts kept; 39 superseded rustc sessions and 150 orphan locks removed (12.8 -> 8.8 GiB); residue census clean |
| `2026-09-25` | `.8` startup sweep | chain sha256 equal; 10 new scratch roots classified (6 named and 2 in use retained; `erg7`'s scripts kept, its data and `probe` unreachable); 49 superseded rustc sessions and 75 orphan locks removed (13.3 -> 8.7 GiB); 2,029.4 MiB of unreachable scratch left for `.9` because its removal was refused; residue census clean |
| `2026-09-24` | `.7` startup sweep | record file created; no scratch root new or unreachable; chain sha256 equal; 22 superseded rustc incremental sessions removed (2.70 GiB); residue census clean |
| `2026-09-23` | `.6` daily sweep | chain unchanged; 6 unreachable scratch roots reclaimed (276.9 MiB / 38 files), 1 new named root retained; 54 superseded rustc incremental sessions removed (5.63 GiB); residue census clean |
| `2026-08-29` | `.3` fixture-residue reclamation | reachability both legs negative (`git grep` returns only the producer and two prose lines; no chain leaves `.project-data/tmp`). Reclaimed **317 directories / 7,630 files / 32 MB** — the standing **138 / 3,449 / 15 MB** plus this leaf's own controls — with an empty name census after. Producer re-verified clean: **84/84, exit 0, leak 0**. Mechanism measured rather than inferred: killing the producer at a known live count leaks every live fixture under **SIGKILL (15/15, 15/15, 20/20)** and **SIGTERM (15/15, 15/15, 20/20)**; killing only the wrapper and orphaning the producer leaks **0**; a harness timeout reproduced it incidentally at **50**. SIGINT stays unmeasured — a background child inherits `SIG_IGN` for it. Gate exposure confirmed: `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILED with 20 `cannot read canonical artifact` lines when a fixture run deleted them mid-walk |
| `2026-08-27` | `.0` reachability sweep and reclamation | every root under `.project-data/tmp/` classified by named-or-chained reachability; the retained-evidence chain traced to one hop with the declared `retained_evidence_sha256` matching on disk; 12 unreachable paths removed — 5,290 files / 2,669,876 KiB (2.55 GiB) — with an empty residue census, and all four retained roots byte-count and file-count identical before and after |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.21` | `SCRATCH-RESIDUE-CONTAINMENT.21 — the daily sweep: no scratch to reclaim, and 100.5 MiB of rustc sessions` | re-run the sweep; no unreachable scratch or fixture residue; reclaim superseded incremental sessions and orphan locks |
| `.20` | `SCRATCH-RESIDUE-CONTAINMENT.20 — the daily sweep: 2.1 GiB of scratch and 1.9 GiB of rustc sessions` | re-run the sweep; reclaim committed slices' snapshots, a superseded seed baseline, superseded incremental sessions and a killed run's fixtures |
| `.18` | `SCRATCH-RESIDUE-CONTAINMENT.18 — the daily sweep: 8.4 GiB of scratch and 2.7 GiB of rustc sessions` | re-run the sweep; reclaim unreachable scratch, a committed slice's snapshots and superseded incremental sessions |
| `.17` | `SCRATCH-RESIDUE-CONTAINMENT.17 — the daily sweep: 22.0 GiB of scratch and 2.9 GiB of rustc sessions` | re-run the sweep; reclaim unreachable scratch, superseded incremental sessions and a killed run's fixtures |
| `.16` | `SCRATCH-RESIDUE-CONTAINMENT.16 — the daily sweep: 7.6 GiB of scratch and 3.3 GiB of rustc sessions` | re-run the sweep; reclaim unreachable scratch and superseded incremental sessions |
| `.14` | `SCRATCH-RESIDUE-CONTAINMENT.14 — the daily sweep: no scratch to reclaim, and 1.5 GiB of rustc sessions` | re-run the sweep; nothing unreachable has accumulated; reclaim superseded incremental sessions |
| `.13` | `SCRATCH-RESIDUE-CONTAINMENT.13 — the daily sweep: 143.8 MiB of scratch and 3.0 GiB of rustc sessions` | re-run the sweep; reclaim unreachable scratch and superseded incremental sessions |
| `.12` | `SCRATCH-RESIDUE-CONTAINMENT.12 — the daily sweep: 926.2 MiB of scratch and 2.1 GiB of rustc sessions` | re-run the sweep; classify every root in the script itself; reclaim unreachable scratch and superseded incremental sessions |
| `.9` | `SCRATCH-RESIDUE-CONTAINMENT.9 — the owner-approved deletion: erg7's 2.0 GiB of unreachable scratch` | remove the unreachable scratch `.8` could not delete without the owner's permission |
| `.11` | `SCRATCH-RESIDUE-CONTAINMENT.11 — the daily sweep: 632.8 MiB of scratch and 2.2 GiB of rustc sessions` | re-run the sweep; reclaim unreachable scratch, old executable and source copies, and superseded incremental sessions |
| `.10` | `SCRATCH-RESIDUE-CONTAINMENT.10 — the daily sweep: 10.2 GiB of per-slice snapshots and 4.0 GiB of rustc sessions` | re-run the sweep; reclaim the lane's per-slice snapshots and superseded incremental sessions |
| `.8` | `SCRATCH-RESIDUE-CONTAINMENT.8 — the startup sweep: 4.7 GiB of rustc sessions, and the scratch it may not delete` | re-run the sweep; reclaim 4.7 GiB of superseded incremental sessions; own the refused scratch deletion as `.9` |
| `.6` | `SCRATCH-RESIDUE-CONTAINMENT.6 — the daily sweep, and the cache sessions rustc can no longer reach` | re-run `.0`'s sweep; reclaim 276.9 MiB of unreachable scratch and 5.63 GiB of superseded incremental sessions |
| `.3` | `SCRATCH-RESIDUE-CONTAINMENT.3 — own and reclaim the fixture residue under generated/` | narrow the `generated/` Non-Goal on a measured falsification, reclaim 317 roots / 32 MB, measure the kill shape that creates them, and route prevention to `.4` |
| `.0` | `SCRATCH-RESIDUE-CONTAINMENT.0 — reclaim the scratch nothing can reach` | establish the named-or-chained reachability model, reclaim 2.55 GiB provably unreachable, and route the two chain-bearing holdout roots to `.1` |

## Update protocol

Each child updates this file's node state, verification log, and commit log in its own commit. A re-run of the
sweep updates the reachability table rather than appending a second one.
