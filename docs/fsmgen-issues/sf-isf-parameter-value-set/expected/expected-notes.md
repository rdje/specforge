# Expected outcomes — sf-isf-parameter-value-set

A regression test for this capability can assert:

1. `./bin/fsmgen --strict --check --json sources/fsmgen-input/value-set.isf` exits 0 with `success: true`; so do
   `range.isf` and `recommended.isf`; each generates the same module as `expected/baseline-good.isf` (captured
   result: `expected/baseline-good.strict-check.json`).
2. `sources/fsmgen-input/illegal-default.isf` exits non-zero under `--strict` with a diagnostic that names
   `DATA_WIDTH`, the value `24`, and the declared set.
3. A use-site or activation override of `DATA_WIDTH` to a value outside the set exits non-zero; to a value in it,
   zero.
4. `actor_params[]` for `DATA_WIDTH` carries the declared set.
5. A default outside a `(recommended …)` range produces a warning, not an error.
