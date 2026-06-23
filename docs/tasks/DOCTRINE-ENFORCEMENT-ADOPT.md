# DOCTRINE-ENFORCEMENT-ADOPT: adopt the portable Doctrine-Enforcement architecture (4th standard)

## Metadata

- Tree ID: `DOCTRINE-ENFORCEMENT-ADOPT`
- Status: `done` (`2026-06-22` — all leaves complete: `.0` framework + `.1` native task-acceptance check + SpecForge `TOOLBOX.md` + `.2` user-facing mdBook chapter + KM card)
- Roadmap lane: `process / continuity` (sibling of `MEMORY_ARCHITECTURE.md` enforcement; cross-cutting)
- Created: `2026-06-22`
- Last updated: `2026-06-22`
- Owner: repo-local workflow
- Owner directive (`2026-06-22`): "When the ramp up is complete please adopt this doctrine enforcement
  system" → `DOCTRINE_ENFORCEMENT.md` (the portable, project-agnostic standard that turns every written
  doctrine into a mechanically-gated check: a `check-script contract` + a `registry/driver` + the same
  E1→E4 defense-in-depth `MEMORY_ARCHITECTURE.md` §9 uses). It is explicitly the **4th** portable
  architecture this project carries, alongside task-trees, memory-architecture, and the knowledge-map.

## Goal

Adopt the Doctrine-Enforcement architecture in SpecForge so that **every mechanizable doctrine is
enforced by one registered driver** (`scripts/check_doctrines.sh`), gated locally by `.githooks/pre-commit`
(E3) and by `scripts/run_ci.sh` / CI (E4). Concretely:

1. land the portable **standard** (`DOCTRINE_ENFORCEMENT.md`) and the **registry+driver**
   (`scripts/check_doctrines.sh`) that runs every `check_*.sh`, reports per-doctrine PASS/FAIL, and
   meta-checks that every registered enforcer exists + is executable (no dangling promise);
2. **register SpecForge's already-existing structural checks** (memory-architecture, knowledge-map)
   into the driver and route the existing pre-commit + `run_ci.sh` through it (unify, do not duplicate);
3. mechanize SpecForge's **flagship doctrine** — *no code change without an owning task-tree leaf first*
   (`docs/decisions/0003`, owner-restated repeatedly) — as a native evidence check
   (`scripts/check_task_acceptance.sh` + `TOOLBOX.md` acceptance-checklist template), so a Rust code
   change that lacks a staged owning task-tree leaf carrying an evidence-backed acceptance checklist
   **cannot commit**;
4. keep the mdBook (the user-facing surface) in lockstep with a chapter documenting the enforcement.

## Non-Goals

- Not weakening or replacing the existing memory-architecture / knowledge-map gates — this tree
  **wraps** them in one driver and registers them (they remain the single source of truth for their
  own invariant).
- Not adding heavy oracle checks (a full `run_ci.sh` / `kg-bench` build) to the **pre-commit** path —
  those stay the strongest leg run via `run_ci.sh` / CI (the §9 honest-limit: pre-commit runs only the
  cheap structural + evidence checks; the deterministic oracle re-run is CI's job).
- Not gating docs-only / scripts-only / mdBook-only commits as "code changes" — the task-acceptance
  evidence check governs **Rust behavioral code** (`crates/**/*.rs`, `crates/**/test_data/**`) and
  exempts the rest, so it never false-blocks unrelated continuity work (incl. this tree's own
  scripts+docs adoption commits).
- No chip/vendor/protocol-name lists anywhere (ADR 0006) — irrelevant here (process infra), but the
  doctrine still holds for any check that ever reads extraction output.

## Acceptance Criteria

- `DOCTRINE_ENFORCEMENT.md` present at the repo root (the portable standard, with SpecForge's live
  registry table in §10).
- `scripts/check_doctrines.sh` runs every registered check, prints a per-doctrine report, exits
  nonzero on any breach, and its meta-check fails on a missing/non-executable enforcer.
- The driver registers, at minimum: `MEMORY-ARCH`, `KNOWLEDGE-MAP`, `TASK-ACCEPTANCE`.
- `.githooks/pre-commit` and `scripts/run_ci.sh` both invoke the driver (E3 + E4).
- A staged Rust code change with no owning task-tree leaf / no evidence-backed acceptance checklist is
  **blocked** by the driver (demonstrated); a docs/scripts-only commit is **exempt** (demonstrated).
- Discovery pointers (`README.md` doc map, `AGENTS.md`, `CLAUDE.md`) name `DOCTRINE_ENFORCEMENT.md` and
  `TOOLBOX.md`; a layer-C decision record captures the adoption.
- The mdBook carries a user-facing chapter on the enforcement system (book-method-doc close rule).
- Every completed leaf is committed through `COMMIT.md`; memory-arch + knowledge-map + (new) driver
  gates green; `kg-bench` / golds orthogonal (no Rust extraction code is touched by this tree).

## Task Tree

- ID: `DOCTRINE-ENFORCEMENT-ADOPT` · Status: `done` (`2026-06-22`) · Goal: adopt the 4th portable architecture ·
  Children: `.0` (framework + register existing + wire) DONE, `.1` (native task-acceptance check + TOOLBOX) DONE,
  `.2` (mdBook chapter + live-doc sync + KM card) DONE.
- ID: `DOCTRINE-ENFORCEMENT-ADOPT.0` · Status: `done` (`2026-06-22`, framework, no Rust) · Goal: land `DOCTRINE_ENFORCEMENT.md`
  (standard) + `scripts/check_doctrines.sh` (registry+driver) registering the EXISTING
  `check_memory_architecture.sh` + `check_knowledge_map.sh`; route `.githooks/pre-commit` and
  `scripts/run_ci.sh` through the driver (preserving the knowledge-map regen+stage step); add decision
  record `0006`; add discovery pointers. Acceptance: `bash scripts/check_doctrines.sh` green with both
  registered checks PASS + the meta-check; pre-commit + CI invoke it; no Rust touched → golds/`kg-bench`
  orthogonal.
- ID: `DOCTRINE-ENFORCEMENT-ADOPT.1` · Status: `done` (`2026-06-22`, scripts+docs, no Rust) · Goal: mechanize SpecForge's flagship doctrine
  — add `scripts/check_task_acceptance.sh` (a staged Rust code change must have a staged owning
  `docs/tasks/*.md` leaf whose acceptance checklist has ROOT CAUSE / ADDRESSED / NO REGRESSION ticked +
  backed by SpecForge tool signatures: `kg-bench 156/156`, `WIRE-BASED-100 1.000`, `run_ci`/`cargo test`,
  byte-identical, validate/adapt findings) + `TOOLBOX.md` (SpecForge's diagnostic surface + the
  acceptance-checklist template); register `TASK-ACCEPTANCE` in the driver; add `TOOLBOX.md` to the
  discovery pointers. Acceptance: a synthetic staged Rust change with no checklist is blocked; a
  docs/scripts-only change is exempt; driver green on this adoption commit (scripts+docs only).
- ID: `DOCTRINE-ENFORCEMENT-ADOPT.2` · Status: `done` (`2026-06-22`, closing leaf) · Goal: add the user-facing mdBook chapter
  documenting the doctrine-enforcement system (what it is, the registry/driver, the E1→E4 gates, the
  acceptance checklist, how to run `check_doctrines.sh`), wired into `SUMMARY.md`; write the
  knowledge-map fact card; sync `LIVE_ACHIEVEMENT_STATUS.md` / `CHANGES.md` / `DEVELOPMENT_NOTES.md` /
  `MEMORY.md`. Closing leaf → book-method-doc rule applies. Acceptance: `mdbook build` green; KM
  derive-and-diff green; driver green.
- ID: `DOCTRINE-ENFORCEMENT-ADOPT.3` · Status: `done` (`2026-06-23`, post-closure bugfix follow-up; scripts-only) ·
  Goal: **fix a deterministic false-NEGATIVE in `scripts/check_task_acceptance.sh` that blocked a legitimate
  code commit.** Surfaced while committing `KG-ISF-COMPLETENESS.2a.iv`: the gate reported the (genuinely present
  + ticked + evidence-backed) ROOT CAUSE / ADDRESSED boxes as "MISSING". **Root cause (WHY+WHERE):** the
  `checked()`/`unchecked()` helpers + the two signature greps ran `printf '%s\n' "$leaf_text" | grep -Eq…` under
  the script's `set -o pipefail`. When `leaf_text` (the concatenated staged task leaves) grows past the ~64 KB
  pipe buffer AND a `grep -q` matches EARLY, grep exits and closes the pipe, `printf` then dies with **SIGPIPE
  (141)**, and `pipefail` propagates that 141 as the pipeline's status — so `checked()` returns non-zero
  (a FALSE "no match"). It is non-interactive-only (an interactive shell ignores SIGPIPE, which is why a
  manual REPL test passed) and size-gated (it only began false-blocking once the `docs/tasks/*.md` leaves grew
  large — `KG-ISF-COMPLETENESS.md` + `CORPUS-COVERAGE.md` are now 151 KB concatenated). Proven: a non-interactive
  `printf "$big" | grep -Eiq <pat>` → exit `141` while `grep -Eiq <pat> <<<"$big"` → exit `0` on the same input.
  **Fix:** replace the four large-`leaf_text` `printf … | grep` pipes with bash here-strings
  (`grep … <<<"$leaf_text"`) — no pipe, no SIGPIPE; behaviour otherwise identical; bash-3.2-safe (the small
  `staged_tasks` filename grep at the top is left as-is — small input, `|| true`-guarded, no `-q` early exit).
  **Acceptance:** the fixed check returns `0` robustly (6/6 runs) on the `KG-ISF-COMPLETENESS.2a.iv` staged set;
  `bash scripts/check_doctrines.sh` 3/3 PASS; `bash -n` syntax-clean. Scripts-only → exempt from the
  task-acceptance checklist requirement (decision `2026-06-22`), so this commit is not self-blocked. The
  deterministic oracles (`kg-bench`/golds/`run_ci.sh`) are unaffected (this is the gate's own logic).

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `DOCTRINE-ENFORCEMENT-ADOPT.0` | `done` (`2026-06-22`) | Standard + driver landed; `MEMORY-ARCH` + `KNOWLEDGE-MAP` registered; pre-commit + `run_ci.sh` route through the driver (2/2 PASS). |
| — | `DOCTRINE-ENFORCEMENT-ADOPT.1` | `done` (`2026-06-22`) | `scripts/check_task_acceptance.sh` (bash-3.2-safe) + SpecForge `TOOLBOX.md`; `TASK-ACCEPTANCE` registered → driver 3/3 PASS; all 5 gate paths tested (exempt / block-no-leaf / pass / block-unticked / block-unbacked). |
| — | `DOCTRINE-ENFORCEMENT-ADOPT.2` | `done` (`2026-06-22`) | mdBook chapter `reference/doctrine-enforcement.md` (in `SUMMARY.md`; `mdbook build` green) + KM card `doctrine-enforcement-adoption` (map 112→113) + live-doc sync. Tree CLOSED. |
| — | `DOCTRINE-ENFORCEMENT-ADOPT.3` | `done` (`2026-06-23`) | Post-closure bugfix: `check_task_acceptance.sh` `printf\|grep -q` under `pipefail` SIGPIPE'd on a >64 KB `leaf_text` with an early match → FALSE "box MISSING", which blocked `KG-ISF-COMPLETENESS.2a.iv`. Fixed with here-strings (no pipe → no SIGPIPE); 6/6 robust pass; scripts-only (checklist-exempt). |

**Tree complete (`2026-06-22`; `.3` bugfix follow-up `2026-06-23`).** All leaves `done`; the doctrine-enforcement
architecture is adopted, the driver gates pre-commit + CI, and the system is documented in the mdBook + KM. The
`.3` follow-up hardened `check_task_acceptance.sh` against a `pipefail`/SIGPIPE false-negative that only surfaced
once the staged task leaves grew large (the gate had passed earlier, smaller commits).

## Decisions

- `2026-06-22`: Adopt `DOCTRINE_ENFORCEMENT.md` as the **4th portable architecture** (sibling of
  `MEMORY_ARCHITECTURE.md`). SpecForge already had E1–E4 for memory-arch + knowledge-map; this tree adds
  the **unifying registry+driver** and a **native evidence check** for the task-tree-ownership doctrine.
  Recorded as decision record `0006`.
- `2026-06-22`: The pre-commit driver runs only **cheap** checks (memory-arch structural, knowledge-map
  derive-and-diff, task-acceptance evidence-presence). The **deterministic oracles** (golds / `kg-bench`
  156/156 / `run_ci.sh`) are the strongest leg and stay on the `run_ci.sh` / CI path (the §9 honest
  limit — leg-3 oracle re-run is CI's job; hosted CI is currently manual-only, an honest gap stated in
  the standard, not hidden).
- `2026-06-22`: "Code change" for the task-acceptance gate = `crates/**/*.rs` + `crates/**/test_data/**`
  (behavioral Rust + gold fixtures). Shell/hook/docs/mdBook changes are **exempt** — they carry their
  own gates and gating them would false-block continuity work, including this tree's own adoption commits.

## Open Questions

- A future native **ADR-0006 structural check** (flag NEW hardcoded chip/signal-name string literals in
  extraction code) is a strong candidate but risks false-positives on legitimate test fixtures /
  vocabulary; deferred to a later leaf so v1 ships zero false-blocking. Tracked here, not blocking.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-22` | `DOCTRINE-ENFORCEMENT-ADOPT.0` | `bash scripts/check_doctrines.sh` (driver) → 2/2 doctrines PASS (`MEMORY-ARCH`, `KNOWLEDGE-MAP`) + meta-check; knowledge-map regen+diff in sync; no Rust → WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal by construction | PASS (committed `810b510b`; pre-commit hook ran the new driver) |
| `2026-06-22` | `DOCTRINE-ENFORCEMENT-ADOPT.1` | `scripts/check_task_acceptance.sh` 5-path behavior test (exempt / block-no-leaf / pass / block-unticked / block-unbacked, all as expected, throwaway staged files fully reverted); `bash -n` syntax-clean + bash-3.2-safe (no `mapfile`); `bash scripts/check_doctrines.sh` → 3/3 PASS; no Rust → golds/`kg-bench` orthogonal | PASS (committed `a9c8d415`) |
| `2026-06-22` | `DOCTRINE-ENFORCEMENT-ADOPT.2` | `bash scripts/run_docs_ci.sh` (mdbook build) green with the new `reference/doctrine-enforcement.md` chapter; KM regen 112→113 facts + `bash scripts/check_doctrines.sh` → 3/3 PASS (KM in sync); no Rust → golds/`kg-bench` orthogonal | PASS |
| `2026-06-23` | `DOCTRINE-ENFORCEMENT-ADOPT.3` | reproduced the bug (`printf "$big" \| grep -Eiq` → exit 141 under non-interactive `pipefail`; `grep -Eiq <<<"$big"` → exit 0) on the 151 KB staged `leaf_text`; applied here-string fix to the 4 large-`leaf_text` greps; `bash -n` clean; fixed `check_task_acceptance.sh` returns 0 6/6 on the `KG-ISF-COMPLETENESS.2a.iv` staged set; `bash scripts/check_doctrines.sh` → 3/3 PASS; scripts-only → golds/`kg-bench` orthogonal | PASS |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `DOCTRINE-ENFORCEMENT-ADOPT.0` | `DOCTRINE-ENFORCEMENT-ADOPT.0 — adopt the portable Doctrine-Enforcement architecture (driver + register existing checks + wire gates)` | committed `810b510b`; framework, no Rust |
| `DOCTRINE-ENFORCEMENT-ADOPT.1` | `DOCTRINE-ENFORCEMENT-ADOPT.1 — SpecForge-native task-acceptance evidence check + SpecForge TOOLBOX.md` | committed `a9c8d415`; scripts+docs, no Rust |
| `DOCTRINE-ENFORCEMENT-ADOPT.2` | `DOCTRINE-ENFORCEMENT-ADOPT.2 — user-facing mdBook chapter + KM card (book-method-doc close)` | closing leaf; docs-only; hash filled at next checkpoint |
| `DOCTRINE-ENFORCEMENT-ADOPT.3` | `DOCTRINE-ENFORCEMENT-ADOPT.3 — fix check_task_acceptance.sh pipefail/SIGPIPE false-negative (here-strings)` | post-closure bugfix; scripts+docs; committed this session |

## Changelog

- `2026-06-22`: `.2` user-facing mdBook chapter + KM card DONE (closing leaf, docs-only) — **tree CLOSED**.
  Added `docs/book/src/reference/doctrine-enforcement.md` (why-before-what: why "trust me" + silent drift
  fail, the doctrine=rule+check idea, the 3 check kinds, the 3 registered doctrines, the acceptance
  checklist + "earned not ticked", the E1→E4 layering with honest limits, how to run/extend, how it was
  verified) wired into `docs/book/src/SUMMARY.md` under Reference; `mdbook build` green. Wrote KM fact
  card `docs/knowledge/doctrine-enforcement-adoption.md` (map 112→113 facts, 802 question keys; driver KM
  check in sync). Synced live docs. Book-method-doc close satisfied. No Rust → golds/`kg-bench` orthogonal.
- `2026-06-22`: `.1` native task-acceptance check + SpecForge `TOOLBOX.md` DONE (scripts+docs, no Rust).
  Added `scripts/check_task_acceptance.sh` — a staged Rust code change (`crates/**/*.rs`,
  `crates/**/test_data/**`) must have a staged owning `docs/tasks/*.md` leaf whose acceptance checklist
  carries ROOT CAUSE / ADDRESSED / NO REGRESSION, ticked + backed by SpecForge tool signatures
  (`validate`/`adapt` findings, `kg-bench 156/156`, `WIRE-BASED-100 1.000`, byte-identical, `run_ci`);
  docs/scripts/mdBook changes are EXEMPT (own gates). Bash-3.2-safe (no `mapfile`), `SPECFORGE_TASK_
  ACCEPTANCE_RANGE`/`_WAIVER` knobs. Wrote SpecForge's own `TOOLBOX.md` (the owner's clarification:
  catalog SpecForge's OWN debug tools — `doctor`/`inspect`/`validate`/`adapt --target isf`/`kg-bench`/
  WIRE-BASED-100 golds/`--dry-run` byte-identical/FSMGen `--strict --check`/`nli-verify`/`grits-consensus`/
  `measure_isf_completeness.py`/`run_ci.sh`, plus the acceptance-checklist template + 3 diagnostic
  protocols). Registered `TASK-ACCEPTANCE` in the driver (3/3 PASS). All 5 gate paths tested + reverted;
  `bash -n` clean. Frontier → `.2` mdBook chapter.
- `2026-06-22`: `.0` framework DONE (no Rust). Landed `DOCTRINE_ENFORCEMENT.md` (portable standard;
  §10 = SpecForge's live registry) + `scripts/check_doctrines.sh` (registry+driver with a meta-check),
  registered the two EXISTING structural checks (`MEMORY-ARCH`, `KNOWLEDGE-MAP`), and routed
  `.githooks/pre-commit` (E3) + `scripts/run_ci.sh` (E4) through the driver while preserving the
  knowledge-map regen+stage step. Added decision record `0006` + discovery pointers (`README.md` doc
  map, `AGENTS.md`, `CLAUDE.md`). Driver verified standalone before wiring (2/2 PASS). No
  extraction/emitter Rust → golds + `kg-bench` orthogonal by construction. Frontier → `.1`.
- `2026-06-22`: Created on the owner's directive to adopt `DOCTRINE_ENFORCEMENT.md` (the 4th portable
  architecture). Decomposed into `.0` (framework + register existing checks + wire gates), `.1` (native
  task-acceptance evidence check + `TOOLBOX.md`), `.2` (mdBook chapter + live-doc sync + KM card).
  Frontier → `.0`.
