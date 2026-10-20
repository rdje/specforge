# Expected outcomes — sf-isf-byte-group-reduction

A regression test for this capability can assert:

1. `sources/fsmgen-input/reduction-over-slice.isf` exits 0 under `--strict --check --json`; the generated HDL
   computes `DATACHK0` as the inverted XOR of `DATA[7:0]` and `DATACHK1` as that of `DATA[15:8]`.
2. `sources/fsmgen-input/byte-parity-param-width.isf`, rewritten in the requested per-group form, exits 0; at
   `DATA_WIDTH = 16, DATA_BYTES = 2` the generated HDL computes bit *i* of `DATACHK` from `DATA[8i+7:8i]`, and at
   `DATA_WIDTH = 32, DATA_BYTES = 4` from each of the four bytes.
3. A per-group target whose width does not equal the operand's group count exits non-zero under `--strict` with a
   diagnostic naming the target and the operand.
4. `expected/baseline-good.isf` keeps its current result (captured: `expected/baseline-good.strict-check.json`).
