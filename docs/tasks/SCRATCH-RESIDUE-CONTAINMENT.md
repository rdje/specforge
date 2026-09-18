# SCRATCH-RESIDUE-CONTAINMENT: reclaim repository scratch that nothing can reach

## Metadata

- Tree ID: `SCRATCH-RESIDUE-CONTAINMENT`
- Status: `active` (`.0`, `.3`, `.5` complete; `.1`, `.2`, `.4` pending)
- Roadmap lane: repository durability and portability (sibling of `SOURCE-IR-REPRODUCIBILITY`)
- Created: `2026-08-27`
- Last updated: `2026-09-18`
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
  shape exactly.
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

## Open Questions

- Should a scratch root that is only chain-reachable be required to carry its own declaration, so reachability
  is readable from tracked files rather than only by executing the resolver?

## Blockers

- None.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-29` | `.3` fixture-residue reclamation | reachability both legs negative (`git grep` returns only the producer and two prose lines; no chain leaves `.project-data/tmp`). Reclaimed **317 directories / 7,630 files / 32 MB** — the standing **138 / 3,449 / 15 MB** plus this leaf's own controls — with an empty name census after. Producer re-verified clean: **84/84, exit 0, leak 0**. Mechanism measured rather than inferred: killing the producer at a known live count leaks every live fixture under **SIGKILL (15/15, 15/15, 20/20)** and **SIGTERM (15/15, 15/15, 20/20)**; killing only the wrapper and orphaning the producer leaks **0**; a harness timeout reproduced it incidentally at **50**. SIGINT stays unmeasured — a background child inherits `SIG_IGN` for it. Gate exposure confirmed: `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILED with 20 `cannot read canonical artifact` lines when a fixture run deleted them mid-walk |
| `2026-08-27` | `.0` reachability sweep and reclamation | every root under `.project-data/tmp/` classified by named-or-chained reachability; the retained-evidence chain traced to one hop with the declared `retained_evidence_sha256` matching on disk; 12 unreachable paths removed — 5,290 files / 2,669,876 KiB (2.55 GiB) — with an empty residue census, and all four retained roots byte-count and file-count identical before and after |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.3` | `SCRATCH-RESIDUE-CONTAINMENT.3 — own and reclaim the fixture residue under generated/` | narrow the `generated/` Non-Goal on a measured falsification, reclaim 317 roots / 32 MB, measure the kill shape that creates them, and route prevention to `.4` |
| `.0` | `SCRATCH-RESIDUE-CONTAINMENT.0 — reclaim the scratch nothing can reach` | establish the named-or-chained reachability model, reclaim 2.55 GiB provably unreachable, and route the two chain-bearing holdout roots to `.1` |

## Update protocol

Each child updates this file's node state, verification log, and commit log in its own commit. A re-run of the
sweep updates the reachability table rather than appending a second one.
