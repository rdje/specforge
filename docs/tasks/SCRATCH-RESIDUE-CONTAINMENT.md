# SCRATCH-RESIDUE-CONTAINMENT: reclaim repository scratch that nothing can reach

## Metadata

- Tree ID: `SCRATCH-RESIDUE-CONTAINMENT`
- Status: `active` (`.0` complete; `.1` pending)
- Roadmap lane: repository durability and portability (sibling of `SOURCE-IR-REPRODUCIBILITY`)
- Created: `2026-08-27`
- Last updated: `2026-08-27`
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
- Do not touch `generated/`; its lifecycle is owned by `specforge clean` and `CORPUS-CHAIN-CURRENCY`.

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

## Open Questions

- Should a scratch root that is only chain-reachable be required to carry its own declaration, so reachability
  is readable from tracked files rather than only by executing the resolver?

## Blockers

- None.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-27` | `.0` reachability sweep and reclamation | every root under `.project-data/tmp/` classified by named-or-chained reachability; the retained-evidence chain traced to one hop with the declared `retained_evidence_sha256` matching on disk; 12 unreachable paths removed — 5,290 files / 2,669,876 KiB (2.55 GiB) — with an empty residue census, and all four retained roots byte-count and file-count identical before and after |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.0` | `SCRATCH-RESIDUE-CONTAINMENT.0 — reclaim the scratch nothing can reach` | establish the named-or-chained reachability model, reclaim 2.55 GiB provably unreachable, and route the two chain-bearing holdout roots to `.1` |

## Update protocol

Each child updates this file's node state, verification log, and commit log in its own commit. A re-run of the
sweep updates the reachability table rather than appending a second one.
