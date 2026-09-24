# Expected behavior

This is a feature request: FSMGen currently behaves as its public contract documents. The expected
behavior below is what SPECFORGE asks the new capability to deliver; the full request, with the measured
need and the proposed acceptance tests, is in `../README.md` sections 5 and 6.

- `baseline-good.isf` (here and in `sources/fsmgen-input/`) is accepted today
  (`baseline-good.strict-check.json`: `success:true`). It states the strobe width as the concrete `4`,
  which is correct only while `DATA_WIDTH` is 32 — the dependency the specification states is lost.
- `sources/fsmgen-input/derived-param.isf` should be accepted with `WSTRB` resolving to 4 bits at the
  default `DATA_WIDTH` 32, and re-derive to `DATA_WIDTH / 8` when `DATA_WIDTH` is overridden.
- `sources/fsmgen-input/derived-ceil.isf` should be accepted with `ADDRCHK` resolving to 4 bits at
  `ADDR_WIDTH` 32 (ceiling division), or FSMGen should document the operator that replaces it.
- `derived-constant.isf` and `derived-inline.isf` are other spellings authors try; accepting them is
  optional (README R4), but whichever are refused should keep a diagnostic that names the supported form.
