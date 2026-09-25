# Expected outcomes — sf-isf-strict-unknown-port-clause

This is a question, so either answer closes it. A regression test can then assert one of:

1. **Unknown clauses are refused:** `./bin/fsmgen --strict --check --json sources/fsmgen-input/bogus-clause.isf`
   exits non-zero with a diagnostic naming `bogus-clause` on port `AWPROT`; `baseline-good.isf` still passes.
2. **Unknown clauses are accepted and ignored by design:** the book (13a or 13k *Interface ports*) says so, and all
   three inputs keep passing with identical generated modules.
