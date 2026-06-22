# FSMGEN-REFRESH-INTEGRATE-5: refresh the FSMGen pin to the SHIPPED declarative storage fields + un-gate DOC-INTENT-TAXONOMY.4a.ii

## Metadata

- Tree ID: `FSMGEN-REFRESH-INTEGRATE-5`
- Status: `done` (CLOSED `2026-06-22`) — `.1` refresh + verify + un-gate DONE
- Roadmap lane: `R6` (× `R15`/`R16` — the `.isf` contract canary serves the north-star bar #6)
- Created: `2026-06-22`
- Last updated: `2026-06-22`
- Owner: repo-local workflow
- Owner directive (`2026-06-22`): "FSMGEN just pushed. Update FSMGEN submodule then read [the declarative-storage-fields
  spec/book sections]." The fifth FSMGen refresh cycle: FSMGen **shipped** the field-structured-storage construct that
  SpecForge's `DOC-INTENT-TAXONOMY.4a` FR requested (accepted in `FSMGEN-REFRESH-INTEGRATE-4`).

## Goal

Bump the pinned `subs/fsmgen` from `5ce0335c5` → `d327129b7` (FSMGen's `origin/main` tip carrying
`ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1` contract-select + `.2` ship-scalar-storage-fields), **empirically verify** the
emitted `.isf` is still strict-valid on the new binary (the contract canary, `[[feedback_verify_fsmgen_before_fr]]`),
and integrate the now-SHIPPED construct: mark the FR SHIPPED, **un-gate `DOC-INTENT-TAXONOMY.4a.ii`** (the Gap-A
field-structured emit is now buildable), and record the precise ISF grammar + verification surface + fail-closed rules
so the next session can implement `.4a.ii` faithfully.

## What FSMGen shipped (the construct SpecForge can now emit)

Reference (pin `d327129b7`): `subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md:637`,
`docs/book/src/13a-actor-interface.md:468` ("Declarative Storage Fields"),
`docs/book/src/13k-isf-feature-support-matrix.md:42` (shipped), report key
`docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md:3450`.

```lisp
(storage
  (var control (width 8) (reset 161)
    (fields
      (field mode   (bits 7 5) (access rw) (reset 5) (enum (IDLE 0) (RUN 5)))
      (field prio   (bits 4 2) (access rw))
      (field enable (bits 0 0) (access rw) (reset 1) (enum (OFF 0) (ON 1))))))
```

Rules that bound a faithful SpecForge `.4a.ii` emit (all FSMGen-enforced, fail-closed):

- **Metadata-only / schedule-safe.** The register still lowers as one N-bit word; the scheduled `.fsm` is
  **byte-identical** to the same source WITHOUT `(fields …)`. So adding fields cannot change scheduling/HDL — the
  emit is safe to add behind the existing `(storage (var …))`.
- `(field NAME (bits HI LO) …)`: `NAME` a unique HDL identifier (sanitize the recovered `field_name`); ranges literal +
  inclusive; **gaps allowed**, but **overlaps or ranges outside the parent width fail closed** → reuse the existing
  `classify_register_reset` / `register_field_extent` tiling gate (`isf_ir.rs`) to admit only in-width non-overlapping
  fields.
- Optional `(access ro|rw|wo|w1c|w0c|rc|rs|warl|wpri|reserved)` — metadata only; **unsupported access tokens fail
  closed** → normalize `RegisterFieldRecord.access_type` to this vocabulary, omit when it does not map (honest).
- Optional field `(reset V)` **must match the corresponding bit slice of the explicit parent `(reset V)`** → only emit a
  field reset when the parent reset is composed (it is composed from the same per-field resets, so it matches by
  construction); omit field resets when the parent reset is omitted (symbolic/partial) — honest, never guessed.
- Inline `(enum (NAME VALUE)…)` accepted **when values fit the field width** → map `enumerated_values`; drop members
  that exceed the field width.
- **Verification surface:** the accepted map is published as `inferred_storage[].fields[]: name, msb, lsb, width,
  access, reset, enum` in the FSMGen report — the `.4a.ii` NO-REGRESSION oracle (assert the emitted fields round-trip).
- Deferred by FSMGen (NOT yet available): actor `(enums …)` references, typed storage fields, banks, aggregate
  carriers, **packet/flit layouts** (so Gap B `.4b` still has no ISF carrier), access enforcement, generated register
  models, parent reset derivation.

## Non-Goals

- Not implementing `.4a.ii` here (that is the next session's CODE slice; this leaf is the refresh + un-gate + design
  capture). Not implementing `.4a.i` either (it remains a valid honest-residual slice, but `.4a.ii` now supersedes it
  as the faithful synthesis path — see Decisions).
- Not modifying `subs/fsmgen` (read-only reference).
- Gap B (message-field packet/flit structures) stays gated — FSMGen explicitly deferred packet/flit layouts.

## Acceptance Criteria

- Gitlink bumped to `d327129b7`; working tree clean.
- The 6 `*_passes_fsmgen_strict_validation` canaries pass on the new binary; `run_ci.sh` green; `kg-bench 156/156`.
- The FR is marked SHIPPED; `DOC-INTENT-TAXONOMY.4a.ii` is un-gated with the precise emit grammar + fail-closed rules +
  verification surface recorded; README pin + KM + live docs updated.
- Committed through `COMMIT.md`; doctrine gates green.

## Task Tree

- ID: `FSMGEN-REFRESH-INTEGRATE-5` · Status: `done` · Goal: refresh cycle 5 — bump to the shipped feature + un-gate
  `.4a.ii`. Children: `.1` (bump + verify + integrate, this slice).
- ID: `FSMGEN-REFRESH-INTEGRATE-5.1` · Status: `done` (`2026-06-22`) · Goal: bump `5ce0335c5 → d327129b7`; verify the
  strict canaries + `run_ci.sh`; integrate the SHIPPED construct (un-gate `.4a.ii`, capture grammar). See checklist.

## Acceptance Checklist (enforced) — `FSMGEN-REFRESH-INTEGRATE-5.1`

- [x] **REPRODUCE / MEASURE** — baseline pin `5ce0335c5` (FR accepted-not-shipped). `git fetch` shows +2 commits to
  `origin/main` (`5ce0335c5..d327129b7`): `ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1` (contract select) + `.2` (ship scalar
  storage fields). Matrix line 42 now marks `(fields FIELD...)` SHIPPED for actor-owned scalar storage.
- [x] **ROOT CAUSE (WHY + WHERE)** — n/a as a bug; an integration. The contract-delta question (does any emitted `.isf`
  form break on the new binary?) is answered by the 6 `*_passes_fsmgen_strict_validation` canaries
  (`crates/specforge/src/ir/adapters.rs:544`, `isf_ir.rs` ×5). The new feature is additive + metadata-only (FSMGen: the
  scheduled `.fsm` is byte-identical with vs without `(fields …)`).
- [x] **ADDRESSED (verified)** — bumped the gitlink to `d327129b7`; the 6 canaries pass on the new binary; the FR is
  marked SHIPPED in `docs/FSMGEN_FEEDBACK.md`; `DOC-INTENT-TAXONOMY.4a.ii` un-gated with the grammar/rules/verification
  surface captured (above + in the DOC-INTENT-TAXONOMY tree).
- [x] **NO REGRESSION** — `*_passes_fsmgen_strict_validation` ×6 PASS; `run_ci.sh` green; `kg-bench 156/156`. No SpecForge
  Rust code changed → emitted `.isf` byte-identical, WIRE-BASED-100 orthogonal.
- [x] **GENERICITY (ADR 0006)** — n/a (submodule pin + docs); no runtime extraction logic changed.
- [x] **LOCKSTEP** — `README.md` pin (`5ce0335c5 → d327129b7`), `docs/FSMGEN_FEEDBACK.md` (FR SHIPPED),
  `docs/tasks/DOC-INTENT-TAXONOMY.md` (`.4a.ii` un-gated + grammar), `docs/TASK_TREE.md`, KM card, and `CHANGES.md` /
  `DEVELOPMENT_NOTES.md` / `LIVE_ACHIEVEMENT_STATUS.md` / `MEMORY.md` updated this slice.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `FSMGEN-REFRESH-INTEGRATE-5.1` | `done` (`2026-06-22`) | Bump + verify + un-gate DONE; gates green on `d327129b7`. **Tree CLOSED.** Next work is `DOC-INTENT-TAXONOMY.4a.ii` (now buildable). |

## Decisions

- `2026-06-22`: FSMGen **shipped** declarative scalar-storage fields (`(var … (fields (field …)))`), exactly the
  construct SpecForge's `.4a` FR requested. ⇒ `DOC-INTENT-TAXONOMY.4a.ii` (Gap-A field-structured emit) is **un-gated and
  is now the highest-leverage buildable lever** — it faithfully synthesizes the register programming model (12,638
  bit-fields / 32 docs that currently reach `.isf` zero times). It **supersedes** `.4a.i` (the adapter honest residual):
  with a real lowering target, the faithful move is to EMIT the fields, not just record that they were dropped — though
  `.4a.i` remains a valid cheap fallback if `.4a.ii` proves larger than one slice. Gap B (`.4b`, packet/flit structures)
  stays gated — FSMGen explicitly deferred packet/flit layouts.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-22` | `FSMGEN-REFRESH-INTEGRATE-5.1` | bump `5ce0335c5→d327129b7`; `*_passes_fsmgen_strict_validation` ×6; `run_ci.sh`; `kg-bench 156/156`; emitted `.isf` unchanged → WIRE-BASED-100 orthogonal | (recorded at commit) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-REFRESH-INTEGRATE-5.1` | `FSMGEN-REFRESH-INTEGRATE-5.1 — bump subs/fsmgen 5ce0335c5→d327129b7 (shipped storage fields) + un-gate DOC-INTENT-TAXONOMY.4a.ii` | bump + verify + un-gate; tree CLOSED |

## Changelog

- `2026-06-22`: Created on the owner directive to update the FSMGen submodule after FSMGen shipped declarative storage
  fields. `.1` DONE: bump `5ce0335c5 → d327129b7`, verify the strict canaries on the new binary, un-gate
  `DOC-INTENT-TAXONOMY.4a.ii` and capture the precise emit grammar + fail-closed rules + the `inferred_storage[].fields[]`
  verification surface for the next session.
</content>
