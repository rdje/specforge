# Expected outcomes — sf-isf-named-timers

A regression test for this capability can assert:

1. `sources/fsmgen-input/named-timers.isf`, in the spelling FSMGen selects, exits 0 under `--strict --check --json`;
   `--emit-schedule-json` lists the timers `pending_hp` (limit 1250) and `credit_hp` (limit 625000), each with a
   counter width and the places it is started and tested.
2. In a generated simulation of a body that starts both timers, the branch tested on `pending_hp`'s expiry is taken
   1250 cycles after its start while `credit_hp` keeps counting; neither sets `last_error`.
3. A reference to an undeclared timer, and a timer declared twice, each exit non-zero under `--strict` with a
   diagnostic naming the timer.
4. `expected/baseline-good.isf` keeps its current result (captured: `expected/baseline-good.strict-check.json`) and
   schedule report (`observed/stdout/baseline-good-schedule.stdout`), including its `pending_timeout` state.
