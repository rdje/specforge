# Expected outcomes — sf-isf-optional-signal

A regression test for this capability can assert:

1. `./bin/fsmgen --strict --check --json sources/fsmgen-input/absent-by-default.isf` exits 0 with
   `success: true`, and the generated module declares no `AWMECID` port.
2. With `(MECID_WIDTH 16)` the generated module is byte-identical to the one for `expected/baseline-good.isf`
   (captured result: `expected/baseline-good.strict-check.json`).
3. `sources/fsmgen-input/present-when.isf` (`PROT_Present` 0) generates no `AWPROT` port and no assignment to
   it; with `PROT_Present` 1 the module equals the one generated without the `(present …)` clause.
4. `sources/fsmgen-input/tie-off.isf` exits 0; a guard reading `AWSNOOP` is evaluated with `AWSNOOP` = 0.
5. A zero-width port that is read and has no `(absent-value …)`, a `(present P)` naming an undeclared or
   non-scalar `P`, an `(absent-value V)` wider than the port, and an `(absent-value …)` on an output each exit
   non-zero under `--strict` with a diagnostic naming the port.
