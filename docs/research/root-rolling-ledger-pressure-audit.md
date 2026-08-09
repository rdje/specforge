# Root rolling-ledger pressure independent closure audit

Owning leaf: `ROOT-ROLLING-LEDGER-PRESSURE.2` (AUDIT/DOC). This audit independently reproduces the committed
`.1` result before the closure records are added. It changes no checker, archive member, manifest, index, registry,
product artifact, threshold, or ceiling.

## Audited boundary and workspace

- Commit: `10d182ffa0fe44e1053cf769e0ca4b94e00318eb`.
- Workspace: a fresh `git clone --no-local` beneath `generated/audits/` on the repository filesystem.
- Device identity: source repository and audit clone both report device `16777240`.
- Git integrity: the clone starts and ends with an empty `git status --short`; `git diff --exit-code` and
  `git fsck --full` pass.
- Gitlink prerequisite: plain Git clone leaves `subs/fsmgen` uninitialized. The first complete doctrine run
  therefore fails only because five feedback evidence paths are absent. Initializing the exact committed gitlink
  `d327129b718ab29fc889db026c19257b0f7fcc49` from the existing same-volume checkout makes all six doctrines pass.
  No network or off-volume cache is used.

The gitlink observation is a clone bootstrap prerequisite, not ledger drift. Focused reconstruction, generic
pressure, mdBook current truth, and the Knowledge Map pass both before and after gitlink initialization.

## Committed result reproduced

The 35 focused cases pass in the clone. Normal protocol validation reconstructs every root, retained migration
suffix, segment, capsule, reciprocal manifest edge, and ordered index route. Generic live-document validation
measures the committed roots exactly as follows:

| Ledger | Records | Lines | Bytes | Max line | Warning disposition |
| --- | ---: | ---: | ---: | ---: | --- |
| changes | 87 | 1,246 | 188,183 | 1,629 | below all warnings |
| development-notes | 62 | 1,294 | 175,215 | 1,401 | below all warnings |
| live-achievement-status | 59 | 101 | 82,836 | 4,824 | below all warnings |
| rust-codebase-analysis | 55 | 1,064 | 89,706 | 369 | below all warnings |

All six doctrines pass after the declared gitlink is populated. The unrelated pre-existing warnings for the fact
index, knowledge-card collection, roadmap, and validation snapshot are unchanged and remain outside this tree.

## Future append reachability

The audit next prepends one concise, valid, uncommitted record through each grammar: a change H3, development H2,
status top-level bullet, and Rust H2. The focused protocol still reconstructs every historical byte and preserves
the complete status trailer. The generic checker reports:

| Ledger | Records | Lines | Bytes | Max line | Result |
| --- | ---: | ---: | ---: | ---: | --- |
| changes | 88 | 1,250 | 188,355 | 1,629 | below warning |
| development-notes | 63 | 1,298 | 175,402 | 1,401 | below warning |
| live-achievement-status | 60 | 102 | 83,067 | 4,824 | below warning |
| rust-codebase-analysis | 56 | 1,068 | 89,864 | 369 | below warning |

The fixtures are then removed exactly. A final clean diff/status, object check, gitlink identity, and
`find generated -name '.rolling-ledger*'` residue census pass. The audit clone and its empty parent are deleted;
the durable evidence is this report plus the task-tree verification log.

## Closure disposition

- **Reproduce/measure:** exact committed root metrics, archive topology, git objects, device identity, and
  prerequisite gitlink identity reproduce from a fresh repository-local clone.
- **Root cause:** the only initial doctrine failure is the standard uninitialized-submodule state of a plain clone;
  populating the exact committed gitlink closes it without changing repository truth.
- **Addressed:** the generic writer, four installed cuts, health-authority binding, rollback behavior, complete
  retrieval chains, and normal/future-append paths all validate independently.
- **No regression:** the clone finishes clean with no generated transaction residue and is removed. The closure
  commit owns final live-doc synchronization and the main-tree full gates.
