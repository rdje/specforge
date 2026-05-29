# ISF-SYMBOL-SURFACE-EMIT: emit the built-but-discarded `(constants)`/`(types)`/`(enums)` ISF surface

## Metadata

- Tree ID: `ISF-SYMBOL-SURFACE-EMIT`
- Status: `active`
- Roadmap lane: `R6` (`.isf` adapter)
- Created: `2026-05-29`
- Last updated: `2026-05-29`
- Owner: repo-local workflow

## Goal

`isf_ir.rs` already *builds* `IsfConstant`/`IsfTypeDef`/`IsfEnum` from
`IntentIR.symbol_definitions` (one `(type NAME (bits ceil(log2(members))))`
+ one `(enums (NAME …))` per enum symbol; `(constants (NAME VALUE))` per
constant) — but `render()` **discards them**, while the adapter artifact
still reports `constant_count`/`enum_count`. That is a **count-vs-emission
truthfulness gap** plus a free, contract-supported emission widening
(surfaced by the `FSMGEN-REFRESH-INTEGRATE.2` assessment).

The blocking question — the actor-local `(types)` ↔ `(enums)` same-name
relationship — was **answered by FSMGen** in upstream `c0b7eaa7`
(`docs/SPECFORGE_FEEDBACK_RESPONSE.md`, locked by `t/1378`):

- `(enums (NAME …))` is **not** a `(type NAME)` alias.
- To use an enum name as a width-bearing type, **co-declare** both
  `(types (type NAME (bits k)))` and `(enums (NAME …))` — accepted,
  required, **not** a redeclaration conflict; `k = ceil(log2(members))`
  is an accepted choice (the width is not cross-validated).
- Unreferenced actor-local `(types)`/`(enums)`/`(constants)` are valid.

This **validates SpecForge's existing build** (it already emits both, with
`k = ceil(log2(members))`) and corrects the earlier "enums-standalone"
reading. So the feature needs **no build change** — only `render()` must
emit the already-built surface, with conservative constant-value handling.

## Non-Goals

- No change to how the symbol surface is *built* (the build is correct per
  the FSMGen contract); only `render()` emission + a value-safety guard.
- No fabrication: constant values that are not safe scalar literals are
  not emitted as-is (residual/skip), per residual-honesty.

## Acceptance Criteria

- `subs/fsmgen` pinned to include FSMGen's answer (`c0b7eaa7`); full CI
  green on the new pin.
- `render()` emits `(constants …)` / `(types …)` / `(enums …)` for the
  built surface; the emitted `.isf` passes FSMGen `--strict`.
- The `constant_count`/`enum_count` the artifact reports now correspond to
  emitted content (honesty gap closed) — or any non-emittable value is
  honestly excluded with a recorded reason.
- Tests lock the emitted grammar + a FSMGen-strict e2e; `BOOK-METHOD-DOC`
  close-rule subsection added.

## Task Tree

- ID: `ISF-SYMBOL-SURFACE-EMIT`
  Status: `active`
  Goal: emit the built-but-discarded symbol surface, contract-correctly
  Children: `ISF-SYMBOL-SURFACE-EMIT.1`, `.2`, `.3`

- ID: `ISF-SYMBOL-SURFACE-EMIT.1`
  Status: `done`
  Goal: >
    Bump `subs/fsmgen` `88a7af9c` → `c0b7eaa7` (the clean enum-type-clarity
    tree-done boundary that delivers FSMGen's answer; before the in-flight
    loop-`do` tree). Verify full CI green on the new binary; reconcile the
    README pin; mark the clarity request RESOLVED in `docs/FSMGEN_FEEDBACK.md`.
  Acceptance: `gitlink updated + staged; scripts/run_ci.sh green (esp. fsmgen-strict ISF test); README pin + FSMGEN_FEEDBACK reconciled.`
  Verification: >
    passed (`2026-05-29`) — gitlink `88a7af9c → c0b7eaa7` staged (the answer
    boundary; `t/1378` present, working tree clean). Full `scripts/run_ci.sh`
    green on the new binary: `1148 passed/0`, with
    `isf_output_passes_fsmgen_strict_validation` +
    `isf_temporal_rules_reach_isf_end_to_end` both `ok` (SpecForge's emitted
    `.isf` still strict-valid; the enum-type clarity tree is doc/test-only,
    no binary change). README pin reconciled `88a7af9c → c0b7eaa7`; the
    clarity request marked RESOLVED in `docs/FSMGEN_FEEDBACK.md`.
  Commit: `see Commit Log`

- ID: `ISF-SYMBOL-SURFACE-EMIT.2`
  Status: `pending`
  Goal: >
    Emit the symbol surface in `render()` per the clarified contract: per
    enum symbol, emit both `(types (type NAME (bits k)))` and
    `(enums (NAME (M V)…))`; emit `(constants (NAME VALUE))` only when the
    value is a safe scalar literal (residual/skip otherwise). Add a
    render-lock test + a FSMGen-strict e2e; reconcile the validate
    count-vs-emission honesty gap.
  Acceptance: `emitted .isf carries the surface and passes FSMGen --strict; reported counts correspond to emitted content; tests green; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

- ID: `ISF-SYMBOL-SURFACE-EMIT.3`
  Status: `pending`
  Goal: close the tree; `BOOK-METHOD-DOC` close-rule subsection; reconcile live docs.
  Acceptance: `tree closed; book subsection added; TASK_TREE index + live docs synced.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ISF-SYMBOL-SURFACE-EMIT.1` | `done` | bumped `subs/fsmgen` → `c0b7eaa7`; answer ingested; CI green; README + FSMGEN_FEEDBACK reconciled |
| → | `ISF-SYMBOL-SURFACE-EMIT.2` | `pending` | **Real frontier** — emit `(constants)`/`(types)`/`(enums)` in `render()` + strict-verify + close the count-vs-emission honesty gap |
| 3 | `ISF-SYMBOL-SURFACE-EMIT.3` | `pending` | close |

## Decisions

- `2026-05-29`: promoted once FSMGen answered the type↔enum clarity request
  (`c0b7eaa7`). Pin target is `c0b7eaa7` (the enum-type tree's `done`
  boundary) rather than the moving `origin/main` tip (`107ca400`, mid an
  unrelated in-flight `loop-do` tree) — a clean, stable point that carries
  the answer. Re-fetch immediately before pinning in case FSMGen revises.

## Open Questions

- None blocking — the contract question is answered.

## Blockers

- None (unblocked by `c0b7eaa7`).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-29` | `ISF-SYMBOL-SURFACE-EMIT.1` | gitlink `88a7af9c → c0b7eaa7`; full `scripts/run_ci.sh` on the new binary (`1148/0`; fsmgen-strict ISF + temporal e2e both `ok`); README + FSMGEN_FEEDBACK reconciled | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-SYMBOL-SURFACE-EMIT.1` | `ISF-SYMBOL-SURFACE-EMIT.1 — bump subs/fsmgen 88a7af9c -> c0b7eaa7 (ingest enum-type answer); verify; mark request RESOLVED` | submodule pin change (tracked); answer ingested; `.isf` still strict-valid |

## Changelog

- `2026-05-29`: Created + promoted `active` — FSMGen answered the type↔enum
  clarity request (`c0b7eaa7`), unblocking emission of the built-but-
  discarded `(constants)`/`(types)`/`(enums)` surface.
