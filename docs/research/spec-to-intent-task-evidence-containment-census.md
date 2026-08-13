# Spec-to-intent active task-evidence containment census

- Date: `2026-08-14`
- Owner: `SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT.1.1`
- Target: `docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md`
- Mode: read-only target census; no migration, source correction, or product change

## Result

The target combines a stable program contract, 58 formal task identities, a 45-row frontier, accumulated
decisions, one large verification ledger, 41 acceptance/closure sections, a commit table, and a long chronology. At
2,049 lines / 278,178 bytes it has only 350 bytes below the unchanged 278,528-byte `task_evidence` target and
ceiling. Behavioral leaf `.6d.ii.f` cannot be activated and durably verified by another ordinary append.

The source is more internally consistent than the first active-task containment target, but three current-state
surfaces are stale: the root `Children` list stops at `.5` although `.6`–`.9` are formal nodes and frontier rows;
`.6` omits `.6e` from its child list even though the numbered lane and frontier treat it as the next fabrication
repair; and the global Open Questions/Blockers text still describes the completed `.4`/`.5b` boundary. Current
truth is otherwise unambiguous: structural `.6d.ii.e` is done, `.6d.ii.f` is the next product leaf, and this
containment prerequisite is its only temporary blocker.

The measurements favor a target-aware form of the ADR 0019 hybrid: a bounded stable root, bounded semantic
parts and route index, and an exact source capsule. This target adds a decisive root constraint. Production
trajectory validation opens the stable Markdown and searches exact `- ID:` owner declarations, so all 58 formal
IDs must remain directly declared in the stable root unless product behavior changes. Fifty-eight compact
declarations plus current state fit comfortably within the existing portable root caps; changing the product
reader is unnecessary and outside containment scope.

## Exact source boundary

Leaf `.0` committed the ownership while leaving the target identical to its last changing commit:

| Property | Value |
| --- | --- |
| Last target-changing commit | `112bc3338c6c4a0dbd522add29e86706ff03c645` (`2026-08-14`) |
| Containment ownership boundary | `5032b378d41f2ce5ca496fff5296dcbe431d1431` |
| Path-touching commits | 40 |
| Lines / bytes / maximum content-line bytes | 2,049 / 278,178 / 1,605 |
| SHA-256 | `e70892a5c6acbe794bea6a9dd4484a90481e31d0aba64d8b8e8dcaaeae90a26c` |
| Git blob | `66ae9b6cde51476d5b45a180c756f0f9463d3632` |
| Existing target/ceiling distance | 350 bytes |

`git diff --exit-code 112bc333..HEAD -- docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md` and the working-tree target
diff remain empty throughout this census.

## Exhaustive semantic source census

These non-overlapping regions account for all 2,049 lines and 278,178 bytes. Boundaries follow existing headings
and task-phase declarations rather than arbitrary equal-size cuts.

| Lines | Count | Bytes | Max line bytes | Role |
| --- | ---: | ---: | ---: | --- |
| 1–43 | 43 | 2,290 | 110 | stable identity, metadata, goal, non-goals, acceptance, task-tree heading |
| 44–129 | 86 | 10,027 | 461 | formal nodes `.0`–`.5b` |
| 130–225 | 96 | 13,317 | 669 | formal nodes `.6`–`.6d.i` |
| 226–290 | 65 | 9,800 | 608 | formal nodes `.6d.ii`–`.6d.ii.d.iv` |
| 291–426 | 136 | 29,957 | 1,605 | formal structural nodes `.6d.ii.e`–`.e.vii` |
| 427–461 | 35 | 2,575 | 422 | behavioral and remaining roadmap nodes `.f`–`.9` |
| 462–511 | 50 | 7,243 | 251 | complete ordered current frontier |
| 512–679 | 168 | 17,342 | 115 | accumulated decisions |
| 680–715 | 36 | 3,336 | 115 | structural doctrine execution contract |
| 716–721 | 6 | 294 | 111 | stale `.4`/`.5` open question |
| 722–726 | 5 | 228 | 107 | stale `.5b` blocker statement |
| 727–877 | 151 | 64,899 | 780 | chronological verification ledger |
| 878–1019 | 142 | 12,633 | 115 | acceptance `.2`–`.5b` |
| 1020–1205 | 186 | 16,405 | 118 | acceptance `.6a`–`.6d.i` |
| 1206–1357 | 152 | 13,560 | 117 | acceptance for identity/genericity remediation |
| 1358–1419 | 62 | 5,774 | 117 | proof-architecture foundation acceptance |
| 1420–1555 | 136 | 12,713 | 118 | five-stage proof-migration acceptance |
| 1556–1652 | 97 | 8,746 | 130 | compiled doctrine and adversarial qualification acceptance |
| 1653–1665 | 13 | 1,003 | 116 | final structural qualification acceptance |
| 1666–1712 | 47 | 11,016 | 364 | commit log |
| 1713–2049 | 337 | 35,020 | 119 | chronology/changelog |
| **Total** | **2,049** | **278,178** | **1,605** | exact source closure |

A seven-part semantic grouping is already feasible without choosing it: program foundation 42,814 bytes;
qualification `.6a`–`.6d.i` 29,722; genericity remediation 23,360; structural proof 61,529; current/future legacy
9,818; verification 64,899; and commit/chronology 46,036. Each stays below the checker's 98,304-byte portable
part ceiling. The 1,605-byte source maximum requires a target-derived part width above ADR 0019's first-target
1,024-byte choice but below the existing generic 6,400-byte portable cap; no global ceiling needs to move.

## Current-state and route census

The source contains 58 unique formal `- ID:` declarations: the root plus 57 containers/leaves. Git history has
40 unique ID-bearing leaf commit subjects, the commit table has the same completed-leaf role, and the frontier
has 45 ordered executable/history rows. These sets intentionally differ: containers and pending leaves have no
completion commit, while the frontier omits container-only rows.

Current-state normalization must preserve these facts and correct only their presentation:

- Root metadata correctly says the program is active.
- `.6d.ii.e` and `.e.vii` are done; `.6d.ii.f` is pending and is the next product step after containment.
- Root `Children: .0 … .5` predates the formal `.6`–`.9` nodes and cannot remain the current topology authority.
- `.6` declares only `.6a`–`.6d`, while the `.6e` id, goal, and frontier row place the remaining fabrication
  repair on the same numbered lane. `.1.2` must settle the normalized parent relation from roadmap/task evidence.
- The Open Questions section asks what `.4`/`.5` should rank, although `.5b` and the complete `.6` history
  already answered it.
- The Blockers section says “None for `.5b`”; it is preserved as literal history but is not a current global
  blocker statement.

The exact source capsule must preserve every old byte and contradiction. The bounded root must separately state
current authority and route every one of the 58 canonical IDs without renumbering.

## Reader inventory

### Stable-path consumers

At committed census input `5032b378`, 31 tracked files outside the target contain its exact path. None appends a
fragment or query suffix, so the stable root route can remain unchanged.

| Consumer class | Files | Required contract |
| --- | ---: | --- |
| Roadmap | 1 | stable program route |
| Rust/test-support plus controller JSON | 4 | stable path and task-ID membership |
| mdBook | 1 | stable explanatory route |
| Decision records | 4 | stable evidence route |
| Knowledge Map fact cards | 20 | stable evidence route |
| Containment owner | 1 | exact target authority |

The Rust consumer is load-bearing. `crates/specforge/src/ir/trajectory.rs::validate_task_owner` canonicalizes the
repository-relative path, reads the file, and requires `contents.contains("- ID: `<task_id>`")`. Unit tests use
historical owner `.5a`; test support and current controller artifacts use `.6e`, `.7`, `.8`, and `.9`. Therefore
a migrated root that routes IDs only through a nested index would break current code/tests even though Markdown
navigation still worked. Keeping all 58 exact declaration literals in the bounded root preserves behavior.

### Identifier consumers and generic readers

Ninety-two tracked files outside the target contain `SPEC-TO-INTENT-ALIGNMENT`; 11 are immutable rolling-ledger
segments, leaving 81 current references. The current set includes 17 crate/test-data files, six mdBook chapters,
six decisions, 21 fact cards, 11 research files, eight task trees, three doctrine inputs, three generated question
shards, five live ledgers, the roadmap, and the derived task catalog. Most require stable IDs, not monolithic
layout.

Generic structural readers add these constraints:

- Startup/PNT reaches the direct-child stable root through `MEMORY.md` and `docs/TASK_TREE.md`.
- `check_task_tree_catalog.pl` scans direct `docs/tasks/*.md`, so the root must remain there with matching H1/id
  and active metadata.
- `check_roadmap_projection_contract.pl` relies on that catalog membership.
- `check_task_acceptance.sh` accepts staged task Markdown at any depth, so a product commit can stage both root
  and owning semantic part; the writer contract should require both.
- `check_live_document_size.pl` needs each nested part/index/capsule classified exactly once before migration.
- `COMMIT.md` requires node status, evidence, frontier, and commit attribution to update atomically per slice.

## Writer inventory

No executable rewrites the target. Its 40 path-touching commits use the manual task transaction: choose/split a
leaf, update state, implement and verify, add task evidence and commit attribution, then commit the task with
affected live surfaces. `docs/TASK_TREE.md` is regenerated only when root catalog identity/status changes.

The migrated writer transaction must remove destination ambiguity:

1. update the stable root's bounded current state, all compact ID declarations, and frontier;
2. update exactly one owning semantic part with the detailed leaf contract/checklist/evidence;
3. update index/manifest only when routing, membership, state, or metrics change;
4. stage root and part together for product work so executable owner validation and task acceptance both see it;
5. never write the exact capsule or a completed legacy payload.

## Reuse measurement for the neutral checker

`scripts/check_active_task_evidence.pl` is parameterized by `--contract`; the live-size driver already invokes
it with independent PDF and corpus contracts. Its portable caps admit 24 parts, 32 source regions, 128 routes,
98,304 bytes per part, and 6,400 bytes per part line. This target's proposed 21 regions, 58 routes, seven semantic
groups, 64,899-byte largest group, and 1,605-byte widest line fit without parser or cap changes.

Adoption still requires a new target contract, a third driver invocation, target-specific live surfaces, and
focused fixtures for the executable-ID/root constraint. `.1.2` decides those authorities before `.2.1` changes
the checker composition or writes a destination.

## Reconstruction and migration invariants

1. The exact 2,049-line / 278,178-byte source, SHA-256, Git blob, and 40-commit boundary remain independently
   retrievable.
2. The stable direct-child path retains the exact tree identity, active status, goal, and a truthful bounded
   current frontier.
3. All 58 exact `- ID:` declarations remain in the stable root so production task-owner validation and
   historical/current tests remain byte-semantically compatible.
4. Every goal, acceptance clause, decision, measurement, verification record, checklist, commit, and chronology
   entry remains directly reachable in bounded hops, while literal contradictions survive only as history.
5. The route index covers all 58 IDs exactly once and distinguishes container, completed, current, and pending
   roles without inventing completion commits.
6. Target-derived part bounds admit the existing 1,605-byte line but remain below the generic 6,400-byte cap;
   every live collection has fixed file/aggregate bounds and a usable rotation/closure rule.
7. The task catalog, roadmap projection, trajectory owner validator, task-acceptance gate, live-document coverage,
   mdBook, Knowledge Map, and `COMMIT.md` workflow continue without heuristic fallback.
8. Migration workspaces, manifests, and outputs remain repository-relative and on the repository volume.
9. Migration changes documentation architecture only; it does not activate `.f`, alter product behavior, or
   claim behavioral genericity.

## Candidate topologies for `.1.2`

### A. Target-aware hybrid root, semantic parts, and exact capsule

Keep the stable root as compact current authority and as the 58-line executable owner registry. Add a bounded
index/manifest, group exact legacy regions into semantic parts, and preserve the complete committed source as an
immutable capsule. Future work updates root plus one active semantic part atomically. This is the best measured
fit and reuses the neutral checker through data, but `.1.2` must fix exact regions, parent precedence, limits,
part rotation, and root rendering before implementation.

### B. Semantic parts with no exact capsule

Reconstruct the source only from marked part payloads. This avoids duplicate provenance but makes every part and
marker jointly responsible for literal recovery and complicates restoration. It offers no benefit for the
stable-root executable-ID requirement.

### C. Chronological segments plus a current root

Roll verification and changelog rows by date. This fits the two append-heavy tails but cannot route task goals,
acceptance checklists, decisions, or cross-phase structure by itself; it would still need semantic parts.

### D. Move task-owner validation to a nested index

Teach production to resolve IDs through a new route file. This can make the root smaller, but changes product
code solely to accommodate documentation layout and expands the migration blast radius. The measured 58-line
root registry already fits, so this is unnecessary.

The census recommends candidate A and leaves selection to `.1.2`'s ADR.

## Reverification

Run from the repository root:

```sh
wc -l -c docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md
shasum -a 256 docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md
git hash-object docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md
git diff --exit-code 112bc333..HEAD -- docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md
git log --format='%H' -- docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md | wc -l
git grep -l -F 'docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md' 5032b378 -- ':!docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md'
git grep -l -F 'SPEC-TO-INTENT-ALIGNMENT' 5032b378 -- ':!docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md'
```
