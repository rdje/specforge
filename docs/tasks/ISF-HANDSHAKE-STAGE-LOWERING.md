# ISF-HANDSHAKE-STAGE-LOWERING: lower HandshakeComplete temporal_rules to `(stage …)`

## Metadata

- Tree ID: `ISF-HANDSHAKE-STAGE-LOWERING`
- Status: `superseded` → `R16-CONTRACT-IR`
- Superseded: `2026-05-19` — folded into `R16-CONTRACT-IR`:
  `HandshakeComplete` → `HandshakeBarrier` obligation → `(stage p
  (ready r)(valid v))` is part of the ContractIR design (`.1`) and its
  lowering (`R16-CONTRACT-IR.3`). No standalone work; no regression
  (current behavior = residual, unchanged until ContractIR lands).
- Roadmap lane: `R15b` (temporal lowering completeness — follow-on)
- Created: `2026-05-18`
- Last updated: `2026-05-18`
- Owner: repo-local workflow

## Why this exists (captured, not yet active)

`ISF-TEMPORAL-LOWERING` `.2.1`/`.2.3` decision #2 mapped
`HandshakeComplete` temporal_rules to **explicit residual decisions**
because the pinned FSMGEN binary strict-REJECTED `(stage … (ready
r)(valid v))` ("unsupported subclause 'ready'"). FSMGEN has since fixed
that (`d4d6dfab ISF-SPECFORGE-REPORTED-STAGE-CONTRACT-BUGS.2`, pin
`9bfb9a20`, verified in `FSMGEN-SUBMODULE-BUMP.1`): a transaction-scoped
`(stage …)` is now strict-accepted. So `HandshakeComplete` rules — today
preserved as `isf_temporal_unrepresentable_*` residuals — could instead
lower to a real `(stage <phase> (ready <ready>)(valid <valid>))`,
reducing residual loss and carrying more recovered intent into `.isf`.

This is a deliberate **behavior change** that reverses a shipped
decision, so per the task-tree + `fsmgen-contract-authority` doctrine it
gets its own tree and its own verification gate rather than being folded
silently into the closed temporal tree or the submodule bump.

## Non-Goals

- Doing this opportunistically without re-confirming `(stage …)` strict
  acceptance against the *currently pinned* binary at implementation time.
- Changing the windowed-contract or value-rule dispositions (those are
  done and correct).

## Acceptance Criteria (when activated)

- `classify_temporal_rule` gains a `Stage` disposition: a
  `HandshakeComplete{valid_signal, ready_signal}` consequent whose both
  signals are declared → a synthetic
  `(transaction txn_temporal_<id> (on start) (stage <id> (ready
  <ready>)(valid <valid>)) (complete done))`; otherwise still residual
  (undeclared signal, etc. — never fabricated).
- Re-verified empirically against the pinned binary (not commit
  subjects): the emitted stage transaction passes `fsmgen --strict
  --check --json`; a self-conflict-free shape is used (the
  `FSMGEN-SUBMODULE-BUMP.1` audit showed a stage on an already-driven
  signal triggers `isf_priority_mixed_timing_conflict`).
- Metric reconciliation still holds (`.2.4` invariant): counts ==
  emitted; residual count drops by the number of now-lowered handshakes.
- Unit + live-corpus + end-to-end regression (mirror
  `ISF-TEMPORAL-LOWERING.3`); all fsmgen-binary tests via
  `crate::ir::run_fsmgen_strict_check` (the serialized helper).
- `scripts/run_ci.sh` green; live docs + mdBook ISF chapter updated;
  every leaf via `COMMIT.md`.

## Task Tree

- ID: `ISF-HANDSHAKE-STAGE-LOWERING`
  Status: `proposed`
  Goal: `HandshakeComplete temporal_rules → (stage …); residual loss reduced; fsmgen-strict valid.`
  Children: defined when promoted to `active`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | — | `proposed` | Not active — awaiting promotion (PNT order or explicit user direction) |

## Decisions

- `2026-05-18`: Captured as `proposed`, NOT `active`. The closed
  `ISF-TEMPORAL-LOWERING` tree's residual behavior is correct as shipped;
  this is an enhancement enabled by the upstream fix, owned separately so
  the reversal of decision #2 is explicit and verification-gated.

## Open Questions

- Phase/edge: `HandshakeComplete` carries a `TickPhase`; does the FSMGEN
  `(stage …)` form need/accept any phase qualifier, or is the bare
  `(stage n (ready r)(valid v))` sufficient? Resolve empirically at
  implementation time against the then-pinned binary.

## Blockers

- None (proposed; not on any active frontier).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-18` | — | captured as proposed | n/a |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| (tree) | `FSMGEN-SUBMODULE-BUMP.3 — reconcile …` | created as `proposed` follow-up |

## Changelog

- `2026-05-18`: Created `proposed` by `FSMGEN-SUBMODULE-BUMP.3` — the
  upstream `(stage …)` fix (pin `9bfb9a20`) unlocks lowering
  `HandshakeComplete` temporal_rules to `(stage …)` instead of residual;
  scoped here with a verification gate, not auto-done.
