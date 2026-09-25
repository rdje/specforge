# Expected outcomes — sf-isf-known-value-predicate

A regression test for this capability can assert:

1. `./bin/fsmgen --strict --check --json sources/fsmgen-input/known.isf` exits 0 with `success: true`, and the
   generated module contains, under `` `ifndef SYNTHESIS ``, an `assert property` over
   `((PSEL & PENABLE)) |-> (!$isunknown(PADDR))` or an equivalent.
2. If `(isunknown …)` ships, `sources/fsmgen-input/isunknown.isf` generates the equivalent property.
3. `(known …)` used in a rule guard or as a drive value exits non-zero under `--strict` with a diagnostic.
4. The module for `expected/baseline-good.isf` is unchanged (captured check result:
   `expected/baseline-good.strict-check.json`).
