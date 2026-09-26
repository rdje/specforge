# Expected outcomes — sf-isf-storage-register-access

A regression test for this capability can assert:

1. `./bin/fsmgen --strict --check --json sources/fsmgen-input/register-access.isf` exits 0 with `success: true`;
   `--emit-schedule-json` reports the `inferred_storage[]` entry `status` with `"access": "ro"`; the scheduled `.fsm`
   and generated HDL equal those of `expected/baseline-good.isf` (captured result:
   `expected/baseline-good.strict-check.json`).
2. `sources/fsmgen-input/register-access-with-fields.isf` exits 0; the `control` entry carries `"access": "rw"`
   and the same `fields` list as for `expected/baseline-good.isf`.
3. `sources/fsmgen-input/unknown-token.isf` exits non-zero under `--strict`, with a diagnostic naming storage
   `status` and the token `sometimes`.
4. `expected/baseline-good.isf` and `sources/fsmgen-input/whole-word-field.isf` keep their current results and
   schedule reports (`observed/stdout/*-schedule.stdout`).
