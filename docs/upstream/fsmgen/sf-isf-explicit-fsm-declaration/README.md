# WITHDRAWN — premature feature request (ISF already expresses FSM intent)

- Status: **WITHDRAWN** `2026-06-07` (same day as raised). Do not action.
- Reason for withdrawal: filed without thoroughly checking current FSMGen — the `subs/fsmgen` submodule
  was **312 commits stale**, and the ISF model was read incompletely. Owner correction: "did you
  thoroughly check what FSMGEN has to offer? … Only then can you make an informed decision to issue a
  feature request"; "focus on ISF (.isf), not .fsm"; "SPECFORGE shall not bother cycle-scheduling, it
  should leave that to FSMGEN. FSMGEN role is to lower ISF (.isf) to FSM (.fsm)."

## What the original request claimed (incorrectly)

That ISF (IAL1) has "no first-class construct to declare a given explicit FSM (named states + labeled
transitions)", and therefore an ISF feature is needed to model the JTAG/SWD TAP FSM.

## Why it is wrong (after updating the submodule + reading the current ISF book/contract/spec)

1. **Division of labor.** SpecForge captures *intent* and emits `.isf` (IAL1); it does **not** do
   cycle-scheduling or state synthesis. **FSMGen lowers `.isf` → `.fsm`** and owns scheduling/state
   synthesis. So "the generated `.fsm` must have exactly the 16 TAP states" is FSMGen's lowering concern,
   not a SpecForge/ISF expressibility gap.
2. **ISF already has the FSM-intent constructs.** The shipped IAL1 surface includes **enums** (state-name
   members), **`switch`** on enum/aggregate selectors, **`set`** scalar assignment, and bounded
   **`while`/`until`** loops (which project through `transaction_loops` with decision/body/exit states).
   These compose into the standard explicit-FSM idiom: an enum `state` variable + `switch (state)` +
   `set state = next` inside a perpetual loop — exactly how a state machine is authored in any
   intent/RTL language. "No register vocabulary; the scheduler decides storage" means the enum state
   variable IS the elegant state representation. This is not a hack.
3. **IAL2 (reserved, not shipped) is for *reusable protocol-level intent objects* ("APB read
   transaction", "AXI burst")** — a convenience layer, not a prerequisite for expressing an FSM.

## Conclusion — PROVEN, no feature request warranted

Empirically verified against the current FSMGen (`subs/fsmgen` @ `d31b0b91`): a **6-state JTAG TAP-DR FSM**
(Run-Test/Idle → Select-DR → Capture-DR → Shift-DR → Exit1-DR → Update-DR, with the correct TMS-driven
edges) lowers cleanly — `./bin/fsmgen --strict --check --json` → `success: true` — using only shipped ISF
constructs:

```lisp
(storage (var tap (width 3) (reset 0)))            ;; state register
(transaction step (on start)
  (switch tap (0 (select tap tms 1 0)) (1 (select tap tms 1 2)) ...) ;; per-state input-driven transitions
  (complete done))
(rule tick start (trigger step))                    ;; perpetual recurrence
```

So `SWD-SERIAL-EXTRACTION.5` will express the TAP FSM with this idiom; SpecForge captures the intent and
**FSMGen lowers `.isf` → `.fsm`** (SpecForge does not cycle-schedule). If — and only if — a future, richer
FSM construct genuinely requires a hack on the **current** FSMGen, a substantiated request will be re-filed
with a concrete failing `.isf` (per "parser acceptance ≠ support").

Lessons: `feedback_isf_no_hacks` (no hacks) and `feedback_verify_fsmgen_before_fr` — **update `subs/fsmgen`
and EMPIRICALLY test (`--strict --check --json`) before asserting any FSMGen capability gap**. See KM
`[[isf-fsm-via-switch-select]]`.
