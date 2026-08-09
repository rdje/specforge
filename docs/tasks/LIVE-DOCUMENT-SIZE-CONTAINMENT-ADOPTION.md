# LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION: bounded live docs, stable README, and same-volume project data

## Metadata

- Tree ID: `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION`
- Status: `done`
- Roadmap lane: process / continuity / repository portability
- Created: `2026-08-08`
- Closed: `2026-08-09`
- Owner: SpecForge maintainers through the repo-local workflow
- History lifecycle: bounded closed summary over one exact immutable source capsule

## Goal and outcome

SpecForge now enforces bounded, role-correct live documentation and repository-volume project-data locality
without discarding durable information. Current views stay concise; exact history is preserved behind bounded
indexes and manifests; generated projections are reproducible; current-state copies have named authorities; and
project-owned temporary/cache/build/runtime data derives from the repository root on the repository volume.

The program adopted project-owned `README_POLICY.md`, `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, and
`PROJECT_DATA_LOCALITY.md`; reduced `README.md` to a stable landing page; migrated four rolling ledgers, the
roadmap, FSMGen feedback, and this task tree losslessly; bounded all collection/projection navigation; made the
mdBook the public product-contract plane; and mechanized the contracts through `scripts/check_doctrines.sh`.
The current downstream handoff remains `docs/FSMGEN_FEEDBACK.md`.

Shared `~/.cargo` and `~/.rustup` remain explicit read/toolchain dependencies on the boot volume. Ambiguous
shared caches remain protected inputs; project-owned data does not default there or to system temporary roots.

## Owner directives satisfied

- README growth is bounded by a repository-owned policy, local targets, hard ceilings, and route checks.
- Every parent-tracked Markdown path is classified exactly once by the live-document registry.
- Historical ledger/task/roadmap/feedback evidence remains exact and directly retrievable.
- mdBook current behavior is code-bound and updated by impact rather than ceremonial co-staging.
- Project-owned temp, cache, dependency, model, test, log, and build paths derive from the current repository.
- The pinned FSMGen submodule and shared machine-global toolchain/cache inputs remain separate authorities.
- No existing ceiling was widened to hide pressure, and no unique evidence was trimmed for size.

## Completed activity map

| Activity | Result | Durable decision / authority |
| --- | --- | --- |
| `.0` | owned the donor review, complete local census, and migration/locality graph | this tree's exact capsule |
| `.1` | made `MEMORY.md` a bounded overwrite-only pointer and commit docs impact-routed | `MEMORY_ARCHITECTURE.md`, `COMMIT.md` |
| `.2` | bounded the README and closed reader/author routes | `README_POLICY.md`, ADR 0007 |
| `.3` | classified every Markdown surface and activated deterministic lifecycle enforcement | `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` |
| `.4` | migrated four append-growing ledgers through exact capsules and bounded live windows | ADR 0008 |
| `.5` | bounded catalogs, Knowledge Map, roadmap, feedback, mdBook, validation, source registry, and corpus KB | ADRs 0009–0015 |
| `.6` | enforced repository-derived project-data roots and completed old-volume cleanup | `PROJECT_DATA_LOCALITY.md` |
| `.7` | independently closed the original whole-program audit | exact capsule verification rows |
| `.8` | added explicit derive-on-read, verified-copy, authored-intent, and immutable-evidence contracts | derived-state registry |
| `.9` | partitioned the rolling-ledger archive route and enforced complete chronology | ADR 0017 |
| `.10` | sealed this completed task source and left this bounded closed root | ADR 0018 |

## Current Frontier

No active frontier.

The next continuity action is a new top-level task tree for active-tree containment before any further append to
`PDF-VARIANT-DIGESTION`; that active program was measured at 2,393 lines / 222,616 bytes, 207 bytes below its
warning. This closed tree must not be reopened.

## Key decisions and invariants

- Current truth, durable rationale, immutable evidence, generated projections, and append-growing chronology are
  different information roles and use different lifecycle contracts.
- A smaller current document is valid only when exact history, reader questions, and reconstruction routes remain
  provable. Git history alone is not the only retrieval plan.
- The task catalog reads the stable task H1 and metadata status; it never mirrors task frontiers or evidence.
- Terminal task containment is permitted only after all leaves close. Active trees require separately owned
  semantic partitioning and may not borrow a terminal summary or wider ceiling.
- `source_locked` pins the pre-migration task source and rejects premature archive files; `migrated` pins the same
  identity to the capsule and validates this closed root, its index, manifest, routes, milestones, and ceilings.
- Persisted project paths are repository-root-relative. Host-local or shared inputs are explicit exceptions and
  never become project-owned defaults.

Durable cross-cutting decisions are indexed in [decision records](../decisions/INDEX.md), including
[ADR 0007](../decisions/0007-live-document-containment-and-data-locality.md),
[ADR 0008](../decisions/0008-lossless-rolling-ledger-protocol.md),
[ADR 0017](../decisions/0017-partitioned-rolling-ledger-archive-route.md), and
[ADR 0018](../decisions/0018-terminal-task-tree-current-history-boundary.md).

## Archive and retrieval

[Complete detailed task history and its identity manifest](../archive/tasks/live-document-size-containment-adoption/INDEX.md)
is the authoritative route for every pre-migration activity, decision, checklist, verification row, commit record,
and changelog entry through `.10b.i`. The capsule is the byte-identical source committed at
`05d80d29bce25e7ae39c515d22789651a90785a2`; this root carries `.10b.ii` migration and closure evidence.

Verify the relationship from the repository root:

```sh
perl scripts/check_task_tree_archive.pl --check
```

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-09` | `.10b.i` | source-locked real report; 15 fail-closed cases; six doctrines; full CI | exact 2,538 lines / 242,172 bytes / max 746 / `f8e10e…96b68`; archive absent; 1,779 tests pass / five ignored |
| `2026-08-09` | `.10b.ii` | committed-source byte comparison; manifest/index/root/surface/reader migration; 15 cases | capsule equals `.10b.i` source byte-for-byte; bounded root and three-link index pass migrated enforcement |
| `2026-08-09` | `.10b.ii` | staged six doctrines; full CI; mdBook; catalogs/map; locality; diff and residue hygiene | green; complete program and `.10` closed without product, canonical artifact, shared input, or ceiling regression |

## Commit Log

| Unit | Commit subject or reference | Result |
| --- | --- | --- |
| `.0`–`.7` | exact subjects and evidence in the source capsule | original containment/locality program closed |
| `.8` | exact subjects and evidence in the source capsule | derived-state authority plane closed |
| `.9` | exact subjects and evidence in the source capsule | partitioned archive route closed |
| `.10a` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.10a — design the terminal task-evidence boundary` | ADR 0018 accepted |
| `.10b.i` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.10b.i — lock the terminal task source boundary` | exact committed source/verifier seam |
| `.10b.ii` | `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.10b.ii — migrate and close terminal task evidence` | exact capsule + bounded closed root |

## Changelog

- `2026-08-09`: `.10b.ii` copied the committed `.10b.i` source byte-for-byte, landed the bounded archive
  index/manifest and exact terminal surface, replaced the stable task path with this closed summary, migrated
  every reader, and closed `.10` plus the complete adoption program. New work must use a new task tree.
