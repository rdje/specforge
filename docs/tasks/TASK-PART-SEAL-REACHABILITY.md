# TASK-PART-SEAL-REACHABILITY: give a completed task-evidence part a reachable closed state

## Metadata

- Tree ID: `TASK-PART-SEAL-REACHABILITY`
- Status: `active`
- Roadmap lane: repository durability and portability
- Created: `2026-08-11`
- Last updated: `2026-08-11`
- Owner: repo-local workflow

## Goal

The partitioned task-evidence contract declares three part states — `legacy`, `active`, and `sealed`. A
post-migration part can only ever hold `active`. `legacy` is reserved for parts that own a region of the archived
source capsule, and `sealed` cannot be reached by any sequence of compliant commits. So a part that is finished,
closed to further writes, and should be immutable has no state that says so, and `CORPUS-COVERAGE`'s own Slice
Transaction sentence — "Completed legacy parts stay sealed" — describes a transition the gate refuses.

This is the same class `LIVE-DOC-STOP-RISK` closed for live documents: a declared bound or state whose only exit is
non-compliant. Give the closed state a compliant entrance, or retire it and say what replaces it.

## Non-Goals

- Do not weaken the sealed-part immutability check. Byte-identity against a named ancestor commit is the property
  worth keeping; only its *entrance* is broken.
- Do not widen `legacy` to cover post-migration parts. A legacy part is defined by owning an exact capsule source
  region, and that pin is what makes the pre-containment source retrievable.
- Do not hand-edit a part, contract, or manifest to fake a seal, and do not land one with `--no-verify`.
- Do not block or delay a corpus refresh on this tree. A completed part staying `active` is untidy, not unsafe.

## Reproduction and measurement (`2026-08-11`)

Measured against the real repository before anything was changed, then restored exactly. `refreshes-49-56` is a
post-migration part, complete at the `CORPUS-COVERAGE.2.50a` boundary, byte-identical to commit `c4f03838`
(blob `06fd9e5a67968f86989f9f3857bc7f1d71209902`). Both candidate one-commit seals fail:

| Candidate | Working-tree state | `check_active_task_evidence.pl --check` result |
| --- | --- | --- |
| A | contract part `state: sealed` with the real `sealed_commit`/`git_blob`; part body still says `- State: \`active\`` | `semantic part 'refreshes-49-56' lacks state literal '- State: \`sealed\`'` |
| B | contract part `state: sealed`; part body updated to `- State: \`sealed\`` with resealed sha/metrics | `sealed part 'refreshes-49-56' differs from its sealing commit` |

The two checks are individually correct and jointly unsatisfiable:

- `scripts/check_active_task_evidence.pl:956` derives the required body literal from the **contract's** state, so
  the body must read `sealed` the moment the contract does.
- `:968-971` requires the body to be byte-identical to `git show <sealed_commit>:<path>`, and `:973` requires that
  commit to be an ancestor of `HEAD` — so the sealing commit must already contain the `sealed` literal.

A commit cannot name its own hash, so the sealing commit would have to be an earlier one that already carried the
`sealed` literal — which candidate A proves could not itself have passed the gate. There is no two-commit path
either: the intermediate state *is* candidate A.

`seal_fixture_activity_part` (`:1686-1718`) confirms the intent rather than contradicting it. The self-test writes
the sealed body, commits it, and only *then* mutates the contract in the working tree — it never commits the
contract and body together, which is exactly the state a real change must land.

Restoration after the probe was exact: `git checkout --` on the three touched paths, `git status --short` empty,
and the contract re-validates.

## Acceptance Criteria

- A completed post-migration part can be marked closed by a sequence of commits that each pass
  `scripts/check_doctrines.sh` unmodified, with no `--no-verify` and no hand-faked hash.
- The immutability property survives: after closure, any edit to the part's bytes fails the gate.
- The chosen mechanism is proven by fail-closed self-test cases, not only by a live example.
- `CORPUS-COVERAGE`'s Slice Transaction wording and any other surface that promises sealing agree with what the
  contract can actually do.
- `refreshes-49-56` reaches its closed state, or the tree records why leaving it `active` is the correct end state.

## Task Tree

- ID: `TASK-PART-SEAL-REACHABILITY`
  Status: `active`
  Goal: a finished task-evidence part has a closed state its own gate will accept
  Children: `TASK-PART-SEAL-REACHABILITY.0`

- ID: `TASK-PART-SEAL-REACHABILITY.0`
  Status: `pending`
  Goal: decide the mechanism and record it as a decision before writing code. The candidates measured so far are
  (a) seal against the part's **last content-changing commit** rather than a commit that must contain the sealed
  literal, moving the state word out of the sealed bytes and into the contract alone; (b) a two-phase
  `sealing` → `sealed` transition where the first phase pins the body and the second pins the commit; and (c)
  retire `sealed`, keep post-migration parts `active`, and express closure through the contract's route/frontier
  data instead. Option (a) looks smallest, but it changes what "the sealed bytes" means and needs the state
  literal's purpose re-examined first.
  Acceptance: `an ADR records the chosen mechanism, the rejected alternatives with their measured failure, and the immutability property the change must preserve; no production code changes in this leaf`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `TASK-PART-SEAL-REACHABILITY.0` | `pending` | the mechanism is a contract-semantics decision; implementing before deciding would pick option (a) by accident |

## Decisions

- `2026-08-11`: found while selecting the write destination for `CORPUS-COVERAGE.2.51`. The refresh needed
  `refreshes-49-56` closed and a new active part opened; only the second half turned out to be reachable.
- `2026-08-11`: the corpus refresh proceeds with `refreshes-49-56` left in state `active`. That is honest — the
  gate has no way to say more — and the refresh records the reason inline so the state is not read as an
  oversight.

## Open Questions

- Is the `- State: \`x\`` body literal load-bearing for a human reader, or is it redundant with the contract? The
  answer decides between options (a) and (b).
- Does any other partitioned contract (`active_task_evidence.json`, `task_tree_archive.json`) declare a state with
  the same unreachable shape? The same checker serves the first of those.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-11` | ownership | ran both candidate seals against the live contract and restored exactly | candidate A `lacks state literal '- State: \`sealed\`'`; candidate B `differs from its sealing commit`; `git status --short` empty and the contract valid after restore |
| `2026-08-11` | ownership | read `scripts/check_active_task_evidence.pl:956`, `:968-973`, `:725-729`, `:1686-1718` | the body literal is derived from the contract state, the sealed body must equal an ancestor commit's blob, `legacy` requires a capsule source region, and the self-test reaches `sealed` only by never committing the contract and body together |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `TASK-PART-SEAL-REACHABILITY` | `TASK-PART-SEAL-REACHABILITY — track the unreachable sealed part state` | ownership and measured reproduction only |

## Changelog

- `2026-08-11`: created from a measured probe taken while choosing where `CORPUS-COVERAGE.2.51` should write.
