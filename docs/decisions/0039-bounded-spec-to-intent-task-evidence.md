---
id: bounded-spec-to-intent-task-evidence
title: SPEC-TO-INTENT-ALIGNMENT keeps executable ownership in a bounded root over semantic evidence and exact provenance
date: 2026-08-14
status: accepted
scope: documentation, continuity, task-tree, active-work, archive, retrieval, trajectory
evidence: docs/research/spec-to-intent-task-evidence-containment-census.md; docs/tasks/SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT.md; crates/specforge/src/ir/trajectory.rs; scripts/check_active_task_evidence.pl
answers:
  - "Which three authorities govern the bounded alignment task migration?"
  - "What normalized parent and state precedence does the alignment task migration use?"
  - "How does ADR 0039 preserve trajectory owner lookup at the stable task path?"
  - "Where will behavioral qualification evidence live after alignment task migration?"
  - "Why can a completed post-migration alignment evidence part not be marked sealed yet?"
  - "What are the local bounds for the partitioned alignment task evidence?"
  - "What is the root-last failure transaction for alignment evidence migration?"
  - "Why is the exact alignment source kept in both a capsule and marked semantic payloads?"
---

# ADR 0039: SPEC-TO-INTENT-ALIGNMENT keeps executable ownership in a bounded root over semantic evidence and exact provenance

## Context

`docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md` is active at 2,049 lines / 278,178 bytes, only 350 bytes below the
unchanged `task_evidence` health target and ceiling. Its next product leaf, `.6d.ii.f`, needs enough room for a
behavioral contract, results, verification, and chronology. An ordinary append cannot preserve that evidence
without crossing the boundary.

The target-specific census closes every source byte in 21 semantic regions. It finds 58 formal task IDs, 45
frontier rows, 40 ID-bearing leaf commits, 31 stable-path consumers, and 92 identifier consumers. The stable path
has one constraint absent from the first active-task migration: production
`trajectory.rs::validate_task_owner` opens the Markdown and requires the exact owner form ``- ID: `<task_id>` ``.
Historical tests and current controller artifacts exercise `.5a`, `.6e`, `.7`, `.8`, and `.9`. Moving the owner
registry only to a nested index would change product behavior and break existing readers.

Three current-facing source statements are stale. The root child list stops at `.5` although `.6`–`.9` are
formal nodes; `.6` omits `.6e` although its ID, goal, roadmap position, and frontier row place it in that lane;
and the Open Questions/Blockers prose still describes the completed `.4`/`.5b` boundary. Literal history must
remain exact, but the bounded current authority cannot continue presenting those statements as current truth.

ADR 0019 permits a target-specific active hybrid. The census shows that its checker already admits this target's
21 regions, 58 routes, seven legacy groups, 64,899-byte largest payload, and 1,605-byte longest source line
through contract data. The migration therefore needs a new contract and composed invocation, not a new product
reader or a widened portable cap.

## Decision

Adopt three distinct authorities:

1. **Current and executable ownership authority:** keep `docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md` as a bounded
   active root. It owns identity, normalized parent/state summaries, the current frontier and blocker, bounded
   recent verification/commit evidence, the route to detail, and all 58 exact compact ``- ID: `<task_id>` `` declarations.
   Those declarations preserve `validate_task_owner`; detailed goals and checklists do not accumulate here.
2. **Browsable task authority:** create `docs/tasks/spec-to-intent-alignment/INDEX.md`, `manifest.json`, and
   bounded semantic parts. The index routes every canonical ID exactly once. One owning active part carries a
   current leaf's detailed contract, checklist, rationale, evidence, and verification; legacy marked payloads
   remain immutable.
3. **Literal provenance:** copy the locked source byte-for-byte to
   `docs/archive/tasks/spec-to-intent-alignment/source-through-2026-08-14.md`. The capsule never accepts future
   writes. It proves the original order and exact contradictions independently of the semantic projection.

The root links the index. The index links every part, the manifest, and capsule. Current work is visible at the
stable path; a canonical task reaches its primary detail in at most two bounded hops; exact legacy bytes are
available through the deliberately large archive terminal.

## Exact semantic partition

The 21 contiguous source regions are assigned exactly once to seven legacy parts. Each marked payload remains
byte-identical even when noncontiguous regions are grouped by meaning.

| Part | Source lines | Legacy payload bytes | Role |
| --- | --- | ---: | --- |
| `program-foundation.md` | 1–129, 512–679, 716–726, 878–1019 | 42,814 | identity, `.0`–`.5b`, decisions, old questions/blocker, early acceptance |
| `qualification-and-repair.md` | 130–225, 1020–1205 | 29,722 | `.6`–`.6d.i` contracts and acceptance |
| `genericity-remediation.md` | 226–290, 1206–1357 | 23,360 | `.6d.ii`–`.d.iv` contracts and acceptance |
| `structural-proof.md` | 291–426, 680–715, 1358–1665 | 61,529 | `.e` contracts, doctrine contract, proof acceptance |
| `current-and-future.md` | 427–511 | 9,818 | `.f`–`.9` legacy contracts and ordered frontier |
| `verification.md` | 727–877 | 64,899 | chronological verification ledger |
| `commit-and-chronology.md` | 1666–2049 | 46,036 | commit log and changelog |

Create an eighth active part, `behavioral-qualification.md`, during migration. It is initially an explicitly
reserved, unrouted scaffold; creating it does not activate `.6d.ii.f`. The closing audit populates it with the
normalized pending `.f` contract and moves `.f`'s one primary route from its legacy payload to this current part
while the exact old contract stays available in the legacy payload and capsule. The following product commit can
then activate `.f` through an ordinary root-plus-owning-part transaction instead of changing topology and product
state at once.

## Current-state precedence

The compact root may normalize current state but may not rewrite historical payloads. Apply these rules in order:

1. An ID-bearing completion commit plus checked acceptance/verification evidence makes a leaf `done`, even when
   earlier prose says pending or in progress.
2. A container is `done` only when all declared children are terminal; otherwise it remains `in_progress`.
3. The normalized root children are `.0`–`.9`. The normalized `.6` children are `.6a`–`.6e`, because `.6e`'s
   formal ID, goal, roadmap position, and frontier establish that parent relation.
4. `.6d` and `.6d.ii` remain `in_progress`; `.6d.ii.e` is done; `.6d.ii.f` is pending and blocked only by this
   containment program until its closing audit releases it. `.6e`, `.7`, `.8`, and `.9` remain pending.
5. A missing dependency or unresolved conflict blocks eligibility. Ambiguity never promotes a leaf.
6. Stale Open Questions/Blockers text remains byte-identical in the capsule and marked legacy payloads but has no
   current-state authority.

After migration and before the closing audit, the stable root names containment as `.f`'s only blocker. The
closing audit removes that blocker and makes `.f` the one eligible product leaf without marking it active.

## Routes and writer transaction

The manifest and index route all 58 canonical IDs exactly once. A legacy or structural route includes an exact
source literal in its owning marked payload. A post-migration route names the current owning part and must occur
in that part. The stable root independently declares all 58 IDs for executable ownership; that duplicate literal
registry is intentional and is not the detail-routing authority.

Every post-migration task change is one commit-level transaction:

1. update the stable root's normalized state, current frontier/blocker, bounded recent evidence, and compact owner
   registry when membership changes;
2. update exactly one owning active part with the detailed current leaf contract or evidence;
3. update index and manifest only when routing, membership, state, or measured metrics change;
4. stage root and owning part together for product work so task acceptance and trajectory validation see one
   coherent state; and
5. run the target contract, all doctrines, and `COMMIT.md` before committing.

Legacy payloads and the exact capsule never change. At the first warning, the next write must split or rotate the
active part at an existing child/container boundary; the split must complete before rollover. A completed
post-migration part remains contract state `active` for now, while closure is expressed by root/frontier/route
authority and a new continuation part receives future writes. `TASK-PART-SEAL-REACHABILITY.0` separately owns
the currently impossible `active` → `sealed` transition. This ADR does not claim an unreachable state.

## Local limits

These target-derived limits admit the measured root registry and every migrated payload below mandatory rollover,
while remaining within the neutral checker's portable caps. Warning is 80%; rollover is 90%.

| Surface | Health targets | Inclusive ceilings |
| --- | --- | --- |
| stable active root | 256 lines / 24,576 bytes / 512 max-line bytes | 384 / 36,864 / 1,024 |
| bounded index | 128 lines / 16,384 bytes / 384 max-line bytes | 192 / 24,576 / 768 |
| semantic parts | 16 files; 640 lines / 86,016 bytes / 2,048 max-line bytes each; 6,400 lines / 786,432 bytes aggregate | 24 files; 896 / 98,304 / 2,560 each; 9,600 / 1,179,648 aggregate |
| exact source capsule | exact 2,049 lines / 278,178 bytes / 1,605 max-line bytes | exact same values |

The manifest is closed at 65,536 bytes, 1,024 maximum record/scalar bytes, 24 parts, 32 source regions, and 128
routes. The eight initial parts and 21 regions fit. No existing target, warning, rollover, or ceiling changes.
Dedicated root-adjacent index, part-collection, and archive-terminal surfaces classify the new Markdown exactly
once; the generic `task_evidence` collection continues to classify the stable root.

## Migration and restoration

1. `.2.1` adds a target contract in `topology_declared`/source-locked state, a third composed checker invocation,
   all local limits and required owner literals, destination-absence checks, and focused malformed-state cases.
2. `.2.2` pins the final clean boundary commit/blob, stage-zero and working bytes, all 21 exact region
   hashes/metrics, 58 routes, and eight destinations while every destination remains absent.
3. `.3` renders and reviews the bounded root, stages on the repository volume, writes the capsule, semantic parts,
   index, manifest, and migrated contract, then replaces the stable root last. It validates the complete result.
4. `.4` audits from a fresh repository-local reader/clone equivalent: source reconstruction, capsule and Git
   identity, root owner lookup, all routes and limits, a positive pending `.f` root-plus-part update, mdBook and
   retrieval truth, and the full CI/doctrine gate. Only then does it release `.f`.

The migration writer records that all destinations were absent before it starts. On any write or final-check
failure, it restores the exact source root and contract bytes and removes only destinations whose prior absence
proved migration ownership. It then reruns source-locked validation. No temporary workspace, output, cache, or
log may leave the repository volume. Ordinary Git commits remain the recovery unit after migration.

## Rejected alternatives

- **Semantic parts without an exact capsule:** marked payloads could reconstruct the source, but every part and
  marker would become jointly load-bearing for provenance. The capsule is a simpler independent identity proof.
- **Chronological partitions only:** they fit verification and changelog tails but cannot route goals, acceptance,
  decisions, and cross-phase structure by task meaning.
- **Move trajectory ownership to the nested index:** this changes product code solely for documentation layout
  even though 58 compact declarations fit the bounded root.
- **Reuse the terminal task-tree archive topology:** the alignment program is active and must retain a current
  frontier plus a writable task authority.
- **Declare completed new parts sealed:** the current checker makes that transition unreachable. Pretending it is
  available would encode a transaction no compliant commit can perform.

## Consequences

- `.6d.ii.f` remains behaviorally unchanged and unactivated throughout containment.
- Existing stable-path and executable owner readers keep working without fallback or product-code changes.
- Current state becomes bounded and explicit while all 278,178 source bytes remain independently recoverable and
  directly browsable by meaning.
- The first behavioral product commit writes only current state and its already-existing owning part.
- Neutral enforcement grows through target data and composition; portable caps do not widen.

## Links

- General active topology: [`0019-bounded-active-task-root-and-semantic-evidence-parts.md`](0019-bounded-active-task-root-and-semantic-evidence-parts.md)
- Corpus adoption: [`0024-corpus-task-bounded-active-root-and-evidence-parts.md`](0024-corpus-task-bounded-active-root-and-evidence-parts.md)
- Census: [`spec-to-intent-task-evidence-containment-census.md`](../research/spec-to-intent-task-evidence-containment-census.md)
- Owning task: [`SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT.md`](../tasks/SPEC-TO-INTENT-TASK-EVIDENCE-CONTAINMENT.md)
- Product program: [`SPEC-TO-INTENT-ALIGNMENT.md`](../tasks/SPEC-TO-INTENT-ALIGNMENT.md)
- Seal reachability: [`TASK-PART-SEAL-REACHABILITY.md`](../tasks/TASK-PART-SEAL-REACHABILITY.md)
