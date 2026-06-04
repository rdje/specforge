# FSMGEN-ASSERT-MIGRATE: re-pin to 43b29f5c + migrate `(contract … eventually …)` → `(assert (monitor …))`

## Metadata

- Tree ID: `FSMGEN-ASSERT-MIGRATE`
- Status: `done` (CLOSED `2026-06-04` — re-pinned `43b29f5c` + emission migrated; `.1`–`.2`)
- Roadmap lane: `R6` (FSMGen handoff / ISF adapter)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: **consequence of FSMGen's response** (user: "update FSMGEN submodule … and do
  whatever other action you have to do like FSMGEN-REFRESH-INTEGRATE"). FSMGen answered the
  2026-06-04 LTL/MTL-in-ISF suggestion (`subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`,
  2026-06-04 section): **yes, already shipped** as a generalization — the standalone
  `(contract … (eventually s (within N)))` clause was **removed** and replaced by a compositional
  temporal-property language in the `(assert/assume/cover …)` verification family (their decisions
  `0008`/`0009`, book `13d-control-flow.md`). The user is separately asking FSMGen to add the two
  deltas (`stable`, `min > 1`) "and more", so SpecForge's stability / non-trivial-min obligations
  correctly **stay residual** until FSMGen ships them — no migration for those now.

## What breaks, and the fix (assessed empirically against the new binary)

Re-pinning `subs/fsmgen` `c0b7eaa7 → 43b29f5c` makes SpecForge's emitted
`(contract <n> (eventually <s> (within <N>)))` (`isf_ir.rs:450`) invalid: the new strict checker
reports *"Transaction '…': unsupported '(contract ...)' clause in transaction body"* (verified by
running `bin/fsmgen --strict --check` from the new pin on a real SpecForge `.isf`). The test
`bounded_contract_passes_fsmgen_strict_validation` fails on the new pin.

**Fix (empirically strict-valid on 43b29f5c):** lower the bounded-eventually to the
verification-family monitor form — `(assert (monitor (within <signal> <within>)))` — per FSMGen's
1:1 mapping (the old `eventually` is its bottom row). Confirmed: that form returns
`success=true, diagnostics=[]` from the new binary; the SpecForge-side contract `name` is dropped
(the assert is anonymous). **`(stage …)` still validates unchanged** (re-checked) — only the
`(contract …)` form moves.

## Scope (bounded)

- Re-pin `subs/fsmgen` to `43b29f5c` (commit the submodule bump **with** the migration so CI is
  consistent on the new pin).
- `isf_ir.rs`: change the emission `(contract … (eventually …))` → `(assert (monitor (within s N)))`;
  update the comments that describe the old clause; update the string-shape test and the
  end-to-end `has_contract` check (`adapters.rs`) to the new form. The strict-check tests then
  pass against the new pin (the real re-validation).
- Retire `TEMPORAL-RULE-SVA-RENDER` (deferred SpecForge-side SVA export) — **superseded**: the
  FSMGen-native path exists and is now used.
- Book + KM note (the `(eventually)` clause is gone; bounded-eventually → `(assert (monitor …))`).

## Non-Goals

- NOT migrating `stable` / `min > 1` obligations (they stay residual until FSMGen ships the deltas
  the user is requesting).
- NOT the broader "lower every temporal-rule shape into the full `(assert …)` family" (only the
  one currently-emitted bounded-eventually form is moved; richer lowering is a future tree once
  the deltas land).
- NOT re-validating non-temporal ISF surfaces beyond running full CI against the new pin.

## Acceptance Criteria

- `.1` assessment recorded (this file); registered in `docs/TASK_TREE.md`.
- `.2`: emission migrated; comments/tests/`adapters.rs` updated; `subs/fsmgen` re-pinned to
  `43b29f5c`; full `scripts/run_ci.sh` GREEN **on the new pin** (the strict-check tests pass with
  the new form); `TEMPORAL-RULE-SVA-RENDER` retired; book + KM updated; tree CLOSED.

## Task Tree

- ID: `FSMGEN-ASSERT-MIGRATE`
  Status: `active`
  Children: `.1` (assess) · `.2` (migrate + re-pin + re-validate + retire SVA + close)

- ID: `FSMGEN-ASSERT-MIGRATE.1`
  Status: `done`
  Goal: read FSMGen's response; empirically assess the breakage + the migration target against
    the new binary; scope the change.
  Acceptance: assessment recorded; registered.
  Verification: passed (`2026-06-04`) — confirmed via `bin/fsmgen --strict --check` on the new
    pin: old `(contract … eventually …)` → rejected ("unsupported (contract ...) clause"); new
    `(assert (monitor (within s N)))` → `success=true`; `(stage …)` still valid. Only one emission
    form moves.
  Commit: `see Commit Log`

- ID: `FSMGEN-ASSERT-MIGRATE.2`
  Status: `done`
  Goal: migrate emission + tests/comments/`adapters.rs`; re-pin; re-validate (full CI on new pin);
    retire `TEMPORAL-RULE-SVA-RENDER`; book + KM; close.
  Acceptance: as above.
  Verification: passed (`2026-06-04`) — `isf_ir.rs` emission `(contract … (eventually …))` →
    `(assert (monitor (within <signal> <within>)))`; dropped the now-unused `IsfContract.name`
    field (struct + both construction sites; the disposition `name` still labels the synthetic
    txn); updated the string-shape test (renamed `render_emits_assert_monitor_bounded_eventually`),
    the `adapters.rs` end-to-end check (`has_assert_monitor`), and the descriptive comments + the
    strict-check assert message. Re-pinned `subs/fsmgen` `c0b7eaa7 → 43b29f5c`. **Full
    `scripts/run_ci.sh` GREEN on the new pin (1239)** — the fsmgen-binary strict-check tests
    (`run_fsmgen_strict_check`) pass against `43b29f5c` with the new form, which IS the
    re-validation. Retired `TEMPORAL-RULE-SVA-RENDER` (superseded). Book subsection in
    `pipeline/isf-adapter.md` (+ updated the temporal-lowering section); KM card
    `fsmgen-temporal-isf-form`; feedback-log resolution note. Tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FSMGEN-ASSERT-MIGRATE.1` | `done` | assessed (breakage + migration target empirically confirmed) |
| 2 | `FSMGEN-ASSERT-MIGRATE.2` | `done` | emission migrated + re-pinned `43b29f5c` + re-validated (CI green 1239) + SVA retired → **CLOSED** |

**Tree CLOSED `2026-06-04`.** `subs/fsmgen` re-pinned to `43b29f5c`; the bounded-eventually now
emits `(assert (monitor (within s N)))` (the removed `(contract … eventually …)` clause);
fsmgen-binary strict-check re-validated on the new pin; `TEMPORAL-RULE-SVA-RENDER` retired as
superseded. `stable` / `min>1` stay residual pending FSMGen primitives.

## Decisions

- `2026-06-04`: lower the bounded-eventually to `(assert (monitor (within s N)))` (FSMGen's
  shipped verification-family form), drop the SpecForge-side contract name, re-pin to `43b29f5c`,
  and retire the SpecForge-side SVA export as superseded. `stable`/`min>1` stay residual pending
  FSMGen support (user is requesting it).

## Blockers

- None — migration target empirically validated against the new binary.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-04` | `.1` | new-pin strict-check: old `(contract eventually)` rejected, new `(assert (monitor (within s N)))` accepted, `(stage …)` still valid; breakage localized to one emission | `passed` |
| `2026-06-04` | `.2` | emission migrated + `IsfContract.name` removed + tests/comments/`adapters.rs` updated; `subs/fsmgen` re-pinned `43b29f5c`; full CI GREEN 1239 on the new pin (fsmgen-binary strict-check re-validated); SVA retired; book + KM `fsmgen-temporal-isf-form` | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-ASSERT-MIGRATE.1` | `FSMGEN-ASSERT-MIGRATE.1 — assess the verification-family re-pin (contract→assert-monitor)` | docs-only |
| `FSMGEN-ASSERT-MIGRATE.2` | `FSMGEN-ASSERT-MIGRATE.2 — migrate bounded-eventually emission to (assert (monitor …)) + re-pin fsmgen 43b29f5c; retire SVA; close` | CI green 1239 on new pin |

## Changelog

- `2026-06-04`: Created — FSMGen generalized the LTL/MTL suggestion (verification family;
  `(contract … eventually …)` removed at `43b29f5c`). Migrate SpecForge's bounded-eventually
  emission to `(assert (monitor (within s N)))`, re-pin, retire the deferred SVA export.
- `2026-06-04`: **Tree CLOSED.** `.2` migrated the emission + removed `IsfContract.name`, re-pinned
  `subs/fsmgen` to `43b29f5c`, re-validated against the new binary (CI green 1239), retired
  `TEMPORAL-RULE-SVA-RENDER`, and added the book subsection + KM card `fsmgen-temporal-isf-form` +
  the feedback-log resolution note.
