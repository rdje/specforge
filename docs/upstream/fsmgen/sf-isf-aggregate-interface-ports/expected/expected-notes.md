# Expected outcomes — sf-isf-aggregate-interface-ports

A regression test for this capability can assert:

1. `sources/fsmgen-input/record-port.isf` exits 0 under `--strict --check --json`; the generated HDL declares
   `lmp_in` 16 bits wide, and the semantic report names its type `lmp_t` with fields `subtype` (4 bits) and
   `payload` (12 bits) in a documented order.
2. `sources/fsmgen-input/record-port-field-guard.isf` exits 0; the generated guard tests the four `subtype` bits
   of `lmp_in`.
3. A port typed with an undeclared record exits non-zero under `--strict` with a diagnostic naming the port and
   the type.
4. `expected/baseline-good.isf` keeps its current result (captured: `expected/baseline-good.strict-check.json`).
