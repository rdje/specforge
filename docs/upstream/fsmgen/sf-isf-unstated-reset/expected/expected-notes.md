# Expected outcomes — sf-isf-unstated-reset

This is a question; the outcomes below are what an answer lets a regression test assert, whichever form FSMGen
names:

1. The book names one form for "the specification states no reset pin", and `sources/fsmgen-input/omitted-reset.isf`
   or `sources/fsmgen-input/clock-domain-no-reset.isf` (or a new form, if one is added) passes `--strict --check`
   in that form.
2. That form's report says the reset is unstated (or chosen at instantiation), so it differs from
   `expected/baseline-good.isf`'s report in a documented field, not only in the input path.
3. If the clock-only form is the answer, the book states the generated FSM's initial state without a reset, and a
   test can observe it.
4. `expected/baseline-good.isf` keeps its current result (captured: `expected/baseline-good.strict-check.json`)
   and semantic report (`observed/stdout/baseline-good-semantic.stdout`).
