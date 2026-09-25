# Expected outcomes — sf-isf-transaction-phase-groups

A regression test for this capability can assert:

1. `./bin/fsmgen --strict --check --json sources/fsmgen-input/phase-groups.isf` exits 0 with `success: true`; the
   generated module equals the one for `expected/baseline-good.isf` (captured check result:
   `expected/baseline-good.strict-check.json`); the schedule or semantic JSON lists transaction `write_transfer`
   with phases `setup` (`PSEL`, `PADDR`, `PWRITE`, `PWDATA`) and `access` (`PSEL`, `PENABLE`, `PREADY`).
2. `sources/fsmgen-input/phase-groups-unknown-signal.isf` exits non-zero under `--strict` with a diagnostic naming
   `NOT_A_PORT`.
3. A member whose stated role contradicts its port's direction, and a duplicate phase name, each exit non-zero
   under `--strict`.
