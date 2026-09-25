# RESOLUTION — sf-isf-stage-ready-valid (F2)

Status: **RESOLVED upstream** (2026-05-18) — with a precise caveat about
this bundle's minimized artifact.

FSMGEN fixed the `(stage … (ready r)(valid v))` "unsupported subclause
'ready'" strict rejection in commit `d4d6dfab
ISF-SPECFORGE-REPORTED-STAGE-CONTRACT-BUGS.2: accept ready-valid stages`
(tracked under FSMGEN's `ISF-SPECFORGE-REPORTED-STAGE-CONTRACT-BUGS` tree,
`a60cc1ab`). The "strict reject exits 255 with no JSON despite `--json`"
surface was fixed in `9bfb9a20 …STAGE-CONTRACT-BUGS.3`.

SPECFORGE bumped the `subs/fsmgen` pin `effe591d → 9bfb9a20`
(`FSMGEN-SUBMODULE-BUMP.1`) and **empirically verified** on the new binary:

- The reported construct is fixed. An isolated, non-self-conflicting
  `(transaction … (stage s (ready REQ)(valid ACK)) …)` now yields
  `success: true` on `9bfb9a20` (was "unsupported subclause 'ready'" on
  `effe591d`).
- Honest caveat on THIS bundle's minimized input: running
  `sources/fsmgen-input/f2-stage-ready-valid.isf` on `9bfb9a20` still
  returns `success: false`, but with a NEW and **correct** diagnostic
  `isf_priority_mixed_timing_conflict on ADDRESS`. The minimization
  injected the `(stage … (valid ADDRESS))` onto a corpus-derived head
  that already drives `ADDRESS` via `(rule rule_7 (ADDRESS 1))`; now that
  stages are processed, FSMGEN correctly flags that artifact's own
  mixed-timing self-conflict. This is correct FSMGEN behavior on a
  self-conflicting artifact — NOT a remaining stage bug. The reported
  bug (stage subclause unsupported) is genuinely fixed.

No SPECFORGE code change is required by this fix: SPECFORGE does not emit
`(stage …)` (it was dropped in `ISF-TEMPORAL-LOWERING.2.1`;
`HandshakeComplete` → residual in `.2.3`). The now-unlocked option to
lower `HandshakeComplete` temporal_rules to `(stage …)` is a scoped
follow-up (see `docs/FSMGEN_FEEDBACK.md` "RESOLVED upstream" and the
proposed `ISF-HANDSHAKE-STAGE-LOWERING` tree), not auto-enabled. This
bundle is retained as the archived reproduction; `observed/` is
intentionally NOT rewritten.
