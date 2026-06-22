# FSMGEN-REFRESH-INTEGRATE-4: refresh the FSMGen pin + integrate FSMGen's answer to the field-structured-storage FR

## Metadata

- Tree ID: `FSMGEN-REFRESH-INTEGRATE-4`
- Status: `done` (CLOSED `2026-06-22`) — `.1` refresh + verify + FR-answer integration DONE
- Roadmap lane: `R6` (× `R15`/`R16` — the `.isf` contract canary serves the north-star bar #6)
- Created: `2026-06-22`
- Last updated: `2026-06-22`
- Owner: repo-local workflow
- Owner directive (`2026-06-22`): "FSMGEN provided an answer to your request, please update FSMGEN's submodule then read
  `docs/SPECFORGE_FEEDBACK_RESPONSE.md`." The fourth FSMGen refresh cycle, triggered by FSMGen's response to the
  `DOC-INTENT-TAXONOMY.4a` field-structured-storage feature request.

## Goal

Bump the pinned `subs/fsmgen` reference from `030f8c273` → `5ce0335c5` (FSMGen's `origin/main` tip after it answered the
FR), **empirically verify** the emitted `.isf` is still strict-valid on the new binary (the contract canary — never
trust commit subjects, `[[feedback_fsmgen_contract]]` / `[[feedback_verify_fsmgen_before_fr]]`), assess the contract
delta across the 106 intervening commits, and integrate FSMGen's field-structured-storage answer into SpecForge's
tracked feedback + the owning `DOC-INTENT-TAXONOMY.4a` thread.

## Non-Goals

- Not modifying `subs/fsmgen` from this repo (read-only reference; if upstream misbehaves, file a local bug report).
- Not implementing the field-structured-storage emit (`DOC-INTENT-TAXONOMY.4a.ii`) — FSMGen accepted the FR but it is
  **not shipped**; the construct is gated on FSMGen's own `ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1` audit.
- Not adopting any of the 106 new commits as a SpecForge behavior change (they are FSMGen-internal IAL2 feature work +
  doctrine-adoption + the FR response doc; FSMGen states the response changes no parser/scheduler/syntax).

## Acceptance Criteria

- The gitlink is bumped to `5ce0335c5` and the working tree is clean.
- The 6 `*_passes_fsmgen_strict_validation` canaries pass on the new binary (emitted `.isf` still strict-valid — no
  contract break); `run_ci.sh` green; `kg-bench 156/156`.
- The contract delta is assessed and recorded (every form SpecForge emits is still accepted).
- FSMGen's FR answer is integrated: the `2026-06-22` FR in `docs/FSMGEN_FEEDBACK.md` is marked RESOLVED with the answer;
  `DOC-INTENT-TAXONOMY.4a.ii` is confirmed gated (accepted-not-shipped) and `.4a.i` confirmed FSMGen-endorsed.
- Committed through `COMMIT.md`; README pin reference + live docs + KM updated; doctrine gates green.

## Task Tree

- ID: `FSMGEN-REFRESH-INTEGRATE-4` · Status: `done` (CLOSED `2026-06-22`) · Goal: refresh cycle 4 — pin bump + verify +
  integrate the FR answer. Children: `.1` (bump + verify + integrate, this slice).
- ID: `FSMGEN-REFRESH-INTEGRATE-4.1` · Status: `done` (`2026-06-22`) · Goal: bump `030f8c273 → 5ce0335c5`;
  empirically verify the strict-validation canaries + `run_ci.sh` on the new binary; assess the contract delta (106
  commits); integrate FSMGen's field-structured-storage answer. See the Acceptance Checklist below.

## Acceptance Checklist (enforced) — `FSMGEN-REFRESH-INTEGRATE-4.1`

- [x] **REPRODUCE / MEASURE** — baseline pin `030f8c273` (`ISF-SPECFORGE-PHASE-MEMBERSHIP-RESPONSE.2`); `git fetch` shows
  106 new commits to `origin/main` (`030f8c273..5ce0335c5`), top two = `ISF-FIELD-STRUCTURED-STORAGE-RESPONSE.1/.2`
  (FSMGen's answer to the `DOC-INTENT-TAXONOMY.4a` FR). Rest = IAL2-FEATURE-COMPLETENESS-FRONTIER.* (FSMGen-internal) +
  DOCTRINE-ENFORCEMENT-ADOPTION.1.
- [x] **ROOT CAUSE (WHY + WHERE)** — n/a as a bug; this is an integration. The contract-delta question (does any emitted
  `.isf` form break on the new binary?) is answered by the 6 `*_passes_fsmgen_strict_validation` canaries
  (`crates/specforge/src/ir/adapters.rs:544`, `crates/specforge/src/ir/isf_ir.rs` ×5) which emit `.isf` and run the real
  `subs/fsmgen/bin/fsmgen --strict --check`. FSMGen's response states it changes no parser/scheduler/lowerer/syntax.
- [x] **ADDRESSED (verified)** — bumped the gitlink to `5ce0335c5`; the 6 canaries pass on the new binary; FR answer
  integrated (FR RESOLVED in `docs/FSMGEN_FEEDBACK.md`; `DOC-INTENT-TAXONOMY.4a.ii` gating confirmed).
- [x] **NO REGRESSION** — `*_passes_fsmgen_strict_validation` ×6 PASS; `run_ci.sh` green; `kg-bench 156/156`;
  `cargo fmt`/`clippy -D warnings`/test green. The emitted `.isf` is unchanged (no emitter code touched) → WIRE-BASED-100
  orthogonal.
- [x] **GENERICITY (ADR 0006)** — n/a (submodule pin + docs); no runtime extraction logic changed.
- [x] **LOCKSTEP** — `README.md` pin reference (`030f8c273 → 5ce0335c5`), `docs/FSMGEN_FEEDBACK.md` (FR RESOLVED),
  `docs/tasks/DOC-INTENT-TAXONOMY.md` (`.4a.ii` gating + `.4a.i` endorsement), `docs/TASK_TREE.md`, KM card, and
  `CHANGES.md` / `DEVELOPMENT_NOTES.md` / `LIVE_ACHIEVEMENT_STATUS.md` / `MEMORY.md` updated this slice.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `FSMGEN-REFRESH-INTEGRATE-4.1` | `done` (`2026-06-22`) | Bump + verify + FR-answer integration DONE; all gates green on `5ce0335c5`. **Tree CLOSED.** |

## Decisions

- `2026-06-22`: FSMGen **accepted** the field-structured-storage FR as a real ISF representational gap and valid future
  direction, but it is **not shipped** — the next step is FSMGen's own `ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1`
  readiness/contract audit. SpecForge's sanctioned near-term posture (FSMGen's own words): keep recovered register/CSR +
  packet/flit field maps in IntentIR metadata/residuals, keep emitting opaque storage, and do NOT fabricate via
  `set-field`/`extract`/fake drives/comments. ⇒ `DOC-INTENT-TAXONOMY.4a.i` (adapter honest residual) is the
  FSMGen-endorsed near-term move; `.4a.ii` (field-structured emit) stays gated on FSMGen shipping the construct.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-22` | `FSMGEN-REFRESH-INTEGRATE-4.1` | bump `030f8c273→5ce0335c5`; `*_passes_fsmgen_strict_validation` ×6 PASS; `run_ci.sh` green (1696 tests / 0 failed, rustdoc, mdBook); `kg-bench 156/156`; emitted `.isf` unchanged → WIRE-BASED-100 orthogonal | PASS |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-REFRESH-INTEGRATE-4.1` | `FSMGEN-REFRESH-INTEGRATE-4.1 — bump subs/fsmgen 030f8c273→5ce0335c5 + integrate the accepted field-structured-storage FR answer` | bump + verify + FR-answer integration; tree CLOSED |

## Changelog

- `2026-06-22`: Created on the owner directive to update the FSMGen submodule and read its response. `.1` in progress:
  bump `030f8c273 → 5ce0335c5`, verify the strict-validation canaries on the new binary, integrate FSMGen's
  field-structured-storage FR answer (accepted, not shipped → gated; `.4a.i` endorsed).
</content>
