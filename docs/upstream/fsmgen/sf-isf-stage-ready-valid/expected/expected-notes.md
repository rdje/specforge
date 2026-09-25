# Expected behavior

## Spec basis

`subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md` §11.8 "Stages,
Contracts, Latency" documents the stage as:

```lisp
(stage phase_name
  (ready ready_signal)
  (valid valid_signal))
```

and states: "Current shipped stage kind is `ready_valid_barrier`."

A downstream tool reading the spec therefore expects a transaction
containing `(stage s_demo (ready REQ) (valid ACK))` to be accepted by the
shipped `./bin/fsmgen --strict --check --json`.

## Known-good counterpart

`baseline-good.isf` is the same artifact as the failing case except the
`(stage …)` clause is replaced by an accepted `(contract …)` clause; it is
derived from a real SPECFORGE-emitted `.isf` that passes strict cleanly.
`baseline-good.strict-check.json` is its captured result (`success: true`,
`diagnostic_count: 0`). It demonstrates the surrounding transaction is
otherwise strict-valid, so the rejection is provably caused by the
`(stage … (ready …)(valid …))` clause and nothing else.

## Resolution is FSMGen's call

This is a doc-vs-shipped-strict contradiction. Either:

- the strict checker should accept the spec-documented
  `(stage name (ready r) (valid v))` `ready_valid_barrier`, or
- §11.8 should be corrected to no longer present `(stage …)` as a shipped
  supported form.

SPECFORGE does not emit `(stage …)`; ISF temporal `HandshakeComplete`
obligations are preserved as explicit SPECFORGE residual decisions instead.
This bundle is filed so the spec and the shipped checker stop contradicting
each other. Secondary observation: the strict rejection exits 255 with no
JSON on stdout even though `--json` was requested.
