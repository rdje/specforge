# Expected behavior

## Spec basis

`subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md` §11.8 "Stages,
Contracts, Latency" documents the temporal contract as:

```lisp
(contract name
  (eventually signal within N))
```

with the flat `within N`, and states: "Current shipped temporal contract kind
is `bounded_eventually`."

A downstream tool reading the spec therefore expects the flat
`(contract name (eventually signal within N))` to be accepted by the shipped
`./bin/fsmgen --strict --check --json`.

## Known-good counterpart

`baseline-good.isf` is the same artifact as the failing case except the
contract clause uses the nested `(eventually ADDRESS (within 4))`. It is
itself derived from a real SPECFORGE-emitted `.isf` that passes strict
cleanly. `baseline-good.strict-check.json` is its captured result
(`success: true`, `diagnostic_count: 0`).

The failing case (`sources/fsmgen-input/f1-contract-eventually-flat.isf`)
differs from `baseline-good.isf` by exactly one line — flat `within 4`
instead of nested `(within 4)`.

## Resolution is FSMGen's call

This is a doc-vs-shipped-strict contradiction. Either:

- the strict checker should accept the spec-documented flat
  `(eventually signal within N)`, or
- §11.8 should be corrected to print only the nested
  `(eventually signal (within N))` form.

SPECFORGE already emits the nested form (it is what strict accepts) and is not
blocked; this bundle is filed so the spec and the shipped checker stop
contradicting each other for the next downstream consumer. A secondary, minor
observation: the strict rejection exits 255 with empty stdout even though
`--json` was requested, so the failure is not expressible through the
documented JSON check surface.
