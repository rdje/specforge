# Expected behavior

- `sources/fsmgen-input/undeclared-width1.isf` under `--strict --check --json`: non-zero exit,
  `success:false`, and a diagnostic that names `VALID` as an undeclared operand read by rules `r_a` and
  `r_b` (book 10, *Direct Generation Errors*: "undeclared internal operands").
- `sources/fsmgen-input/undeclared-mixed.isf`: the same undeclared-operand diagnostic, reported before any
  width is inferred, so no width-mismatch diagnostic appears for `r_b`.
- `declared-baseline.isf` (copied here) stays accepted: `declared-baseline.strict-check.json` is the
  captured `success:true` result. It is the input the author should have written, and the only one of
  the three whose generated module may carry an `input VALID` port.
- No generation path adds a module port that the actor's `(interface ...)` does not declare.
