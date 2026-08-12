# FSMGEN-REFRESH-INTEGRATE-8: refresh the FSMGen pin and audit the new upstream delta

## Metadata

- Tree ID: `FSMGEN-REFRESH-INTEGRATE-8`
- Status: `done`
- Roadmap lane: `R6` (× `R15`/`R16` — FSMGen strict is the executable-intent boundary)
- Created: `2026-08-12`
- Last updated: `2026-08-12`
- Owner: repo-local workflow
- Owner directive (`2026-08-12`): "Whenever you have time, please update FSMGEN submosule as he recently pushed."

## Goal

Advance the read-only `subs/fsmgen` gitlink from the currently pinned
`a51dcdad0a7e752e638abfe3ab414f7f3911889d` to the freshly fetched `origin/main` tip, audit every
intervening upstream commit and the resulting downstream-integration contract, verify SpecForge's strict
executable-intent boundary against the exact new binary, and integrate only product or documentation truth
established by that audit.

## Non-Goals

- Do not modify or commit FSMGen source from this repository; the submodule is an upstream read-only authority.
- Do not infer adoptable behavior from commit subjects alone; inspect changed contract, handoff, book, tests,
  and implementation.
- Do not weaken SpecForge strict canaries or generated expectations to conceal an incompatible contract change.
- Do not mix the pending SpecForge EvidenceIR-neutrality implementation into this dependency refresh.

## Acceptance Criteria

- The fetched `origin/main` identity, ancestry, exact commit range, subjects, and changed-file inventory are recorded.
- `subs/fsmgen` is detached at the exact upstream tip, its worktree is clean, and only the parent gitlink changes.
- Every relevant changed FSMGen contract, handoff, book, release, test, and implementation surface is read; every
  adoptable, incompatible, or no-action conclusion has exact upstream evidence.
- Focused real-binary strict canaries pass on the new pin; broader SpecForge gates run in proportion to the delta.
- Required integration remains generic and source-grounded; no document, vendor, protocol, or pin exception is added.
- FSMGen pin copies, Knowledge Map, mdBook, task/live state, and derived-state checks agree with the index gitlink.
- The required `CHANGES.md` entry's induced rollover uses the registered root-last transaction: exact dry run,
  whole records only, no bound change, older archive members byte-identical, and a warning-safe live root.
- The slice is committed through `COMMIT.md` with no submodule-local change or project-data residue.

## Task Tree

- ID: `FSMGEN-REFRESH-INTEGRATE-8` · Status: `done` · Goal: refresh cycle 8 — pin, audit, verify,
  integrate, and close. Children: `.1`.
- ID: `FSMGEN-REFRESH-INTEGRATE-8.1` · Status: `done` · Goal: fetch and inspect the exact upstream
  range, bump the gitlink, run the strict executable-intent canaries, reconcile durable pin/contract truth,
  and close the refresh.

## Acceptance Checklist (enforced) — `FSMGEN-REFRESH-INTEGRATE-8.1`

- [x] **REPRODUCE / MEASURE** — fetch `origin/main`; record old/new full revisions, ancestry, commit count,
  subjects, dates, and changed paths before drawing a contract conclusion.
- [x] **ROOT CAUSE (WHY + WHERE)** — classify the upstream delta from authoritative contract/book/code sources
  and identify exactly which SpecForge integration seam, if any, is affected.
- [x] **ADDRESSED (verified)** — pin the parent gitlink to the fetched tip and implement only required
  compatibility or current-truth integration established by the audit.
- [x] **NO REGRESSION** — keep the submodule worktree clean; run focused FSMGen strict canaries, declared-pin
  currentness, doctrines, mdBook, and the broader CI gate warranted by the executable dependency update.
- [x] **GENERICITY (ADR 0006)** — add no document/vendor/protocol exceptions or pin-specific product logic.
- [x] **LOCKSTEP** — task tree, index, Knowledge Map, mdBook, live docs, resume pointer, and every declared
  FSMGen pin copy agree with the stage-zero gitlink.

## Current Frontier

Complete. After this leaf commits cleanly, return to roadmap-owned `SPEC-TO-INTENT-ALIGNMENT.6d.ii.c`.

## Decisions

- Entry gitlink target: `a51dcdad0a7e752e638abfe3ab414f7f3911889d`.
- Create a new refresh tree rather than reopening the closed historical cycle 7. The upstream tip and product
  meaning remain deliberately unasserted until fetched and inspected.
- A pruned fetch and independent live `ls-remote --symref` query agree on remote `HEAD`/`main` tip
  `c0d8b668db2527108d1c23c20d184400c51efea6`; the entry pin is its ancestor across exactly four commits.
- The four commits are `8dcab8c994575b953579826428e69ee12962f0f4` (replace broken mdBook action),
  `e746fc4ba8d0900aa8a84fdc3439b05c95c966d9` (restore AHB semantic corpus count),
  `2dcd29942b7cd0bb732ddec75e6fd025d8b9173c` (restore AHB alias corpus count), and
  `c0d8b668db2527108d1c23c20d184400c51efea6` (close terminal failure inventory).
- The exact 15-path / 223-insertion / 76-deletion diff changes hosted CI workflow/install support, FSMGen's AHB
  regression expectations, and upstream task/memory/knowledge evidence. It changes none of `docs/ISF_SPEC.md`,
  `docs/ISF_PUBLIC_INTERFACE_CONTRACT.md`, `docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md`, `docs/isf-spec/`,
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md`, mdBook ISF contract chapters, or parser/IR/lowering implementation.
- Classification: pin-only compatibility refresh. No SpecForge product code, emitted ISF, or conformance behavior
  should change; the seven real-binary strict canaries are the executable confirmation.
- Current gitlink target (abbreviated): `c0d8b668d`.
- The required change record moved `CHANGES.md` to 1,627 lines / 222,249 bytes, crossing the 1,620-line
  rollover signal. This leaf owns the induced, mechanically registered rollover rather than deleting the record,
  widening a bound, or pivoting to another task while dirty. Plan
  `docs/research/fsmgen-refresh-integrate-8.1-changes-rollover-plan.jsonl` pins the committed opening blob and
  seals 12 exact oldest post-migration records into `changes-0008`, leaving the current refresh record live.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-12` | `.1` baseline | parent index gitlink; submodule status/remote | pinned `a51dcdad0a7e752e638abfe3ab414f7f3911889d`; detached clean worktree; remote inspection pending |
| `2026-08-12` | `.1` remote audit | pruned fetch; independent `ls-remote --symref`; ancestry/range/log/path diff; relevant upstream bootstrap, contract, task, evidence, and code reads | live remote tip `c0d8b668db2527108d1c23c20d184400c51efea6`; exact four-commit fast-forward; 15 changed paths; downstream contract delta none |
| `2026-08-12` | `.1` focused | `cargo test -p specforge --lib fsmgen_strict -- --nocapture`; submodule status | 7 passed / 0 failed; submodule detached, clean, and exact at the live tip |
| `2026-08-12` | `.1` first full gate | `bash scripts/run_ci.sh` | correctly blocked at `LIVE-DOC-SIZE`: the feedback self-test fixture needed the current task evidence path, and the required change entry crossed the `CHANGES.md` rollover signal; all other doctrines including chain currency passed |
| `2026-08-12` | `.1` induced rollover | exact plan dry run, then `--apply-rollover`; standard ledger/feedback checks; archive diff | dry run exact and warning-safe; `changes-0008` seals 12 records / 271 lines / 25,138 bytes at SHA-256 `13225d9b…a23e`; live root is 93 records / 1,355 lines / 197,110 bytes; all seven older segments and the source capsule remain byte-identical |
| `2026-08-12` | `.1` final | `bash scripts/run_ci.sh` | PASS: all eight doctrines, formatting, warning-deny Clippy, 1,873 passed / six ignored / zero failed, rustdoc, mdBook test/build, and final project-data locality |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-REFRESH-INTEGRATE-8.1` | `FSMGEN-REFRESH-INTEGRATE-8.1 — pin c0d8b668d and verify compatibility` | four-commit pin-only fast-forward; no downstream contract/code delta; all gates green |

## Changelog

- `2026-08-12`: Created before fetching or changing the gitlink in response to the owner's explicit refresh request.
- `2026-08-12`: Fetched and independently verified the live remote; audited the exact four-commit delta, advanced
  the detached clean gitlink, classified the change as contract-neutral, and began final repository gates.
- `2026-08-12`: Closed `.1` and the tree after the induced ledger rollover and complete CI passed.
