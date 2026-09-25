# Expected outcomes — sf-isf-actor-level-assertion

A regression test for this capability can assert:

1. `./bin/fsmgen --strict --check --json sources/fsmgen-input/no-transaction-assert.isf` exits 0 with
   `success: true`, and the generated module contains one `assert property` on `posedge clk`, reset-gated, over
   `$rose(SCK) |-> $stable(WS)` (or an equivalent).
2. `sources/fsmgen-input/actor-level-assert.isf` generates the same `assert property` as `expected/baseline-good.isf`
   (captured check result: `expected/baseline-good.strict-check.json`; captured property:
   `observed/command-logs/baseline-good-assert-property.txt`).
3. `sources/fsmgen-input/actor-level-assume.isf` generates the corresponding `assume property`.
4. An actor-level check naming an undeclared signal exits non-zero under `--strict` with a diagnostic naming it.
