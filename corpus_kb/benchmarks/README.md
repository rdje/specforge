# Benchmark Result Pages

This page family records repeatable benchmark outcomes from tracked KG-quality fixtures.

The goal is to make fixture drift and fixture-family coverage visible to future implementation without turning benchmark pass/fail status into canonical document truth.

Pages in this family can feed:

- fixture-family review
- regression debugging
- future corpus-KB synthesis notes
- prior-candidate and rescan-target ideas

The aggregate benchmark page can also feed dedicated fixture-family pages under `tables/`, `visuals/`, `state_machines/`, `timing/`, `infra/`, and `protocols/`.
Those pages are still benchmark projections, not canonical truth stores.

They cannot approve canonical IR mutation, suppress local validation findings, or replace the `specforge kg-bench` executable gate.
