# FSMGEN-MIN-WINDOW-CONFIRM: answer FSMGen's `min > 1` window question

## Metadata

- Tree ID: `FSMGEN-MIN-WINDOW-CONFIRM`
- Status: `done` (CLOSED `2026-06-04` — answer filed in the feedback channel; FSMGen's action)
- Roadmap lane: `R6` (FSMGen handoff / temporal semantics)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: **FSMGen has a question** (user relayed). FSMGen's
  `SPECFORGE_FEEDBACK_RESPONSE.md` "2026-06-04 (follow-up)" section: `(stable …)` shipped
  (`6700fbb4`); the `min > 1` window slice is **proposed but gated on one confirmation** —
  *"are SPECFORGE's mined `cycle_window` bounds always integer literals with `MIN >= 1`, or can
  `MIN` be `0`? That determines whether `(within B 0 MAX)` should be accepted or redirected to
  the monitor form."*

## The answer (verified against SpecForge's code)

`cycle_window = { min_cycles: Option<u32>, max_cycles: Option<u32> }` (`ir/semantic.rs`):

- **Always integer literals** — concrete non-negative integers parsed from cycle counts
  (`parse_cycle_count_value`); no symbolic/parameter form (`None` = bound unstated).
- **`MIN = 0` occurs in the model but is never emitted as a `|-> ##` consequent.** Two sources:
  the degenerate same-cycle `[0,0]` window (`semantic.rs:8803`/`8883`) — already a **residual**
  (the `(within 0)` guard) — and a literal `0`-to-`N` range (`min=0, max=N`), which is
  semantically the anchored `F[0,N]` = FSMGen's `(monitor (within S N))` form.
- **Emission commitment:** SpecForge will emit `(assert (=> <ante> (within <cons> MIN MAX)))`
  **only with `1 <= MIN <= MAX`**; `[0,0]` → residual, `[0,N]` → the `(monitor (within S N))`
  form.

**Recommendation to FSMGen:** lock `(within B MIN MAX)` to `1 <= MIN <= MAX` — SpecForge will
not emit `(within B 0 MAX)`, so redirecting/rejecting `MIN = 0` (FSMGen's instinct) is correct.

## Scope

Docs-only — a dated answer in `docs/FSMGEN_FEEDBACK.md` (the suggestion/answer channel). No
code change; no submodule re-pin (the integration — re-pin + migrate `stable` + the eventual
`min > 1` lowering — is a future tree once FSMGen ships `min > 1`).

## Acceptance Criteria

- Answer filed in `docs/FSMGEN_FEEDBACK.md` (integer-literal bounds; `MIN >= 1` guarantee;
  `[0,0]`→residual, `[0,N]`→monitor; lock the form to `1 <= MIN <= MAX`). Registered. CLOSED.

## Task Tree

- ID: `FSMGEN-MIN-WINDOW-CONFIRM`
  Status: `done`
  Goal: verify SpecForge's `cycle_window` min semantics; answer FSMGen's gating question in the
    feedback channel.
  Acceptance: as above.
  Verification: passed (`2026-06-04`) — verified `min_cycles: Option<u32>` (integer literals);
    the only `Some(0)` sites are the same-cycle `[0,0]` window (already residual); a `0`-to-`N`
    range maps to the anchored monitor form; SpecForge guarantees `MIN >= 1` for the
    `(within B MIN MAX)` consequent. Answer filed in `docs/FSMGEN_FEEDBACK.md`. No code change /
    no re-pin. Tree CLOSED — the ball is in FSMGen's court (it will ship `min > 1` on this
    confirmation).

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FSMGEN-MIN-WINDOW-CONFIRM` | `done` | answer filed; **CLOSED** |

**Tree CLOSED `2026-06-04`.** FSMGen can lock `(within B MIN MAX)` to `1 <= MIN <= MAX`. The
follow-on integration (re-pin past `stable` + `min > 1`, migrate stability obligations off
residuals, expand the bounded lowering) is a future owned tree once FSMGen ships `min > 1`.

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-MIN-WINDOW-CONFIRM` | `FSMGEN-MIN-WINDOW-CONFIRM — answer FSMGen's min>1 window question (integer literals; MIN>=1 guarantee); close` | docs-only |

## Changelog

- `2026-06-04`: Created + CLOSED — answered FSMGen's gating question for the `min > 1` window
  slice: bounds are integer literals; SpecForge guarantees `MIN >= 1` for the `(within B MIN
  MAX)` consequent (`[0,0]`→residual, `[0,N]`→monitor). FSMGen can lock `1 <= MIN <= MAX`.
