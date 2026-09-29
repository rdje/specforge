# Expected outcomes — sf-isf-duration-time-units

A regression test for this capability can assert:

1. `sources/fsmgen-input/wait-in-ms.isf`, with its actor's clock frequency declared in the form FSMGen selects for
   R2, exits 0 under `--strict --check --json`; `--emit-schedule-json` reports for the wait the stated duration
   `12 ms`, the frequency `125000000` and the derived count `1500000`, and the scheduled `.fsm` implements it with a
   counter rather than one state per cycle.
2. `sources/fsmgen-input/watchdog-in-us.isf`, with the same declaration, exits 0; the await's watchdog limit is
   reported as `10 us` and `1250` cycles.
3. A duration in an actor that declares no frequency, and a duration with an unknown unit, each exit non-zero under
   `--strict` with a diagnostic naming the clause.
4. `expected/baseline-good.isf` keeps its current result (captured: `expected/baseline-good.strict-check.json`) and
   its schedule report (`observed/stdout/baseline-good-schedule.stdout`).
