# FSMGEN-REFRESH-INTEGRATE: refresh the FSMGen submodule + assess adoptable ISF features

## Metadata

- Tree ID: `FSMGEN-REFRESH-INTEGRATE`
- Status: `active`
- Roadmap lane: `R6` (`.isf` adapter — FSMGen is the downstream `.isf` consumer)
- Created: `2026-05-29`
- Last updated: `2026-05-29`
- Owner: repo-local workflow

## Goal

Per explicit user direction (`2026-05-29`): update the pinned
`subs/fsmgen` submodule to current upstream, then read its handoff,
integration, and public-API-contract-stabilization documents plus its
mdBook thoroughly, and assess which new FSMGen features / enhancements
SpecForge's `.isf` adapter can adopt. FSMGen is the downstream consumer
of SpecForge's `.isf`; staying current keeps the emitted-`.isf` contract
honest and surfaces new lowering capability.

## Non-Goals

- Do NOT modify the `subs/fsmgen` submodule from this repository (it is a
  read-only upstream reference); upstream misbehavior is tracked locally
  via the `DOWNSTREAM_ISSUE_REPORTING.md` bundle protocol.
- Do NOT adopt a new FSMGen feature into the SpecForge adapter in this
  tree beyond what is verifiably safe; feature adoption that changes
  emitted `.isf` is scoped as its own follow-up tree (this tree assesses
  and recommends).
- Respect [[feedback-fsmgen-contract]]: parser acceptance ≠ support — the
  public-interface contract / integration spec is the authority, not
  empirical binary behavior alone.

## Acceptance Criteria

- `subs/fsmgen` pinned to current upstream `origin/main` tip; gitlink
  staged + committed; full `scripts/run_ci.sh` green on the new pin
  (critically `isf_output_passes_fsmgen_strict_validation` still accepts
  SpecForge's emitted `.isf`, or any divergence is understood + recorded).
- The handoff / integration / public-API-contract / mdBook docs are read
  thoroughly and a concrete feature-adoption assessment is recorded (what
  SpecForge can use now, what needs a follow-up tree, what stays out of
  scope).
- Stale pin references in tracked docs (README, RUST_CODEBASE_ANALYSIS)
  reconciled to the new pin.

## Task Tree

- ID: `FSMGEN-REFRESH-INTEGRATE`
  Status: `active`
  Goal: refresh the FSMGen submodule + assess adoptable ISF features
  Children: `FSMGEN-REFRESH-INTEGRATE.1`, `.2`, `.3`

- ID: `FSMGEN-REFRESH-INTEGRATE.1`
  Status: `done`
  Goal: >
    Bump `subs/fsmgen` `9bfb9a20` → upstream `origin/main` tip
    (`88a7af9c`, +637 commits); verify full `scripts/run_ci.sh` green on
    the new binary (esp. the fsmgen-strict ISF test); reconcile the stale
    pin reference in `README.md`.
  Acceptance: `gitlink updated + staged; scripts/run_ci.sh green; README pin reference corrected; any strict-behavior divergence recorded honestly.`
  Verification: >
    passed (`2026-05-29`) — gitlink `9bfb9a206 → 88a7af9c1` staged;
    submodule working tree clean; new `bin/fsmgen` compiles. Full
    `scripts/run_ci.sh` green on the new binary: `1142 passed; 0 failed`,
    and **`isf_output_passes_fsmgen_strict_validation`** +
    **`isf_temporal_rules_reach_isf_end_to_end`** both `ok` — i.e. the
    new fsmgen still accepts SpecForge's emitted nested
    `(contract … (eventually s (within N)))` + `ready`/`valid` `(stage)`.
    The updated integration spec confirms the nested form remains a
    supported compatibility alias (flat `within N` now preferred — a `.2`
    follow-up candidate, not a break). README pin reconciled
    `32aa318 → 88a7af9c`. (`RUST_CODEBASE_ANALYSIS.md` historical entries
    are dated point-in-time snapshots; the current 2026-05-29 entry +
    this tree carry the live pin — no rewrite of dated history.)
  Commit: `see Commit Log`

- ID: `FSMGEN-REFRESH-INTEGRATE.2`
  Status: `pending`
  Goal: >
    Read thoroughly the handoff (`SPECFORGE_FEEDBACK_RESPONSE.md`),
    integration (`ISF_DOWNSTREAM_INTEGRATION_SPEC.md`), public API
    contract (`ISF_PUBLIC_INTERFACE_CONTRACT.md`), and mdBook (esp.
    `13i-downstream-integration`, `13k-isf-feature-support-matrix`,
    `13h-lowering-reference`); record a concrete feature-adoption
    assessment for the SpecForge `.isf` adapter; update
    `DEVELOPMENT_NOTES.md` + `docs/FSMGEN_FEEDBACK.md`; scope any adoption
    as named follow-up trees.
  Acceptance: `assessment recorded with specific adoptable features (or an honest "nothing new safely adoptable yet"); follow-up trees named if warranted; respects the contract-is-authority doctrine.`
  Verification: `pending`
  Commit: `pending`

- ID: `FSMGEN-REFRESH-INTEGRATE.3`
  Status: `pending`
  Goal: close the tree; reconcile any remaining doc drift; record final CI evidence.
  Acceptance: `tree closed; TASK_TREE index synced; all touched live docs reconciled.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FSMGEN-REFRESH-INTEGRATE.1` | `done` | bumped `9bfb9a20 → 88a7af9c`; CI green on new binary; README pin reconciled |
| → | `FSMGEN-REFRESH-INTEGRATE.2` | `pending` | **Real frontier** — record the feature-adoption assessment + scope follow-up trees |
| 3 | `FSMGEN-REFRESH-INTEGRATE.3` | `pending` | close |

## Decisions

- `2026-05-29`: created by explicit user direction. Upstream is **+637
  commits** (`9bfb9a20 → 88a7af9c`) with extensive ISF work
  (`ISF-DOWNSTREAM-CONTRACT-HANDOFF-SYNC`, feature-support-matrix,
  cookbook, diagnostics, mdBook coverage). Bump to `origin/main` tip.
  Read the *updated* docs (checkout first, then read) so the assessment
  reflects current upstream, per the user's "update … when done read"
  sequence.

## Open Questions

- Whether the +637-commit bump changes fsmgen `--strict` behavior in a
  way that affects SpecForge's emitted nested `.isf` contract. Resolved
  empirically in `.1` (the strict ISF test is the canary) cross-checked
  against the public-interface contract in `.2`.

## Blockers

- None (network fetch succeeded; upstream tip identified).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-29` | `FSMGEN-REFRESH-INTEGRATE.1` | gitlink `9bfb9a20 → 88a7af9c` staged; full `scripts/run_ci.sh` on new binary (`1142 passed/0`; fsmgen-strict ISF + temporal e2e both `ok`); README pin reconciled | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-REFRESH-INTEGRATE.1` | `FSMGEN-REFRESH-INTEGRATE.1 — bump subs/fsmgen 9bfb9a20 -> 88a7af9c (+637); CI green; reconcile README pin` | tree created + first leaf; submodule pin change (tracked); SpecForge `.isf` still strict-valid |

## Changelog

- `2026-05-29`: Created by explicit user direction (update FSMGen
  submodule + read handoff/integration/contract/mdBook + assess
  adoptable features).
