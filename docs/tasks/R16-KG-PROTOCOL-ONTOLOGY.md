# R16-KG-PROTOCOL-ONTOLOGY: protocol-structured knowledge graph (point #2)

## Metadata

- Tree ID: `R16-KG-PROTOCOL-ONTOLOGY`
- Status: `proposed`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #2, order 2)
- Created: `2026-05-19`
- Last updated: `2026-05-19`
- Owner: repo-local workflow

## Goal

Give the KG a protocol ontology. Today it has actors, signals,
`actor_ports`, `signal_connectivity`, and `TickPhase` (pre/post **clock
edge**) — but protocol intent is organized around **channels,
transactions, and protocol phases** (AXI AW/W/B/AR/R; APB setup/access;
burst/beat/last), none first-class. Add node types **`Channel`**,
**`Transaction`**, **`Phase`**, **`HandshakePair`** with edges
`qualifies`, `belongs-to-channel`, `phase-of`, `ordered-before`,
`stable-during`, so IntentIR is a systematic projection of a graph whose
shape mirrors the protocol — the "typed KG that turns mechanically into
IntentIR" the thesis requires.

## Non-Goals

- Not an extraction tree — it defines the structural vocabulary the
  extraction trees (#3/#4/#6) populate.
- Do not remove the existing actor/signal graph; extend it.

## Acceptance Criteria

- New node/edge types in SemanticIR/IntentIR KG with serde + validation
  counts; ContractIR contracts attach to `Channel`/`Phase`/`Transaction`
  nodes (not just bare signals).
- `kg-bench` extended with protocol-structure fixtures; no regression of
  existing `vlm_state_machine_*`/connectivity fixtures.
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`.

## Task Tree (proposed; expands at promotion)

- Children (sketch): `.1` ontology design (node/edge schema + projection
  rules, docs-only) → `.2` typed nodes/edges + serde + validate counts →
  `.3` bind ContractIR to channel/phase nodes → `.4` kg-bench fixtures +
  close

## Dependencies / Order

- Depends on `R16-CONTRACT-IR` (contracts must exist to attach to
  channel/phase nodes). Blocks `R16-MULTIMODAL-CONTRACT-FUSION`
  (fusion keys on channel/phase identity).

## Decisions

- `2026-05-19`: Foundational target shape with `R16-CONTRACT-IR`; the
  pair is the "almost mechanical to lower from" KG of the thesis.
  Created `proposed`.

## Blockers

- None (proposed; promotion after `R16-CONTRACT-IR`).

## Changelog

- `2026-05-19`: Created `proposed` as program point #2.
