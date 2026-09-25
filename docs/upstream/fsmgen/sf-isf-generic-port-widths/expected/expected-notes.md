# Expected outcomes — sf-isf-generic-port-widths

A regression test for this capability can assert:

1. The SystemVerilog generated for `sources/fsmgen-input/parameter-width.isf` declares `WDATA` with a range that
   depends on `DATA_WIDTH` (for example `[DATA_WIDTH-1:0]`), and neither `WDATA`, `WDATA_next` nor its reset value
   contains the literal `31` or `32'h`.
2. A testbench instantiating that module with `#(.DATA_WIDTH(64))` elaborates a 64-bit `WDATA`, and its behaviour
   matches the module generated for `sources/fsmgen-input/parameter-width-64.isf`.
3. The module generated for `expected/baseline-good.isf` is unchanged (captured check result:
   `expected/baseline-good.strict-check.json`).
4. A library use-site override of `DATA_WIDTH` generates a child whose port has the overridden width.
