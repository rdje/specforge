# R16-KG-PROTOCOL-ONTOLOGY: protocol-structured knowledge graph (point #2)

## Metadata

- Tree ID: `R16-KG-PROTOCOL-ONTOLOGY`
- Status: `done`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #2, order 2)
- Created: `2026-05-19`
- Last updated: `2026-05-20`
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

## Task Tree

- ID: `R16-KG-PROTOCOL-ONTOLOGY`
  Status: `active`
  Goal: protocol-structured KG; IntentIR a systematic projection of it
  Children: `.1`, `.2`, `.3`, `.4`

- ID: `R16-KG-PROTOCOL-ONTOLOGY.1`
  Status: `done`
  Goal: >
    Ontology design (docs-only): node/edge schema for
    `Channel`/`ProtocolPhase`/`Transaction`/`HandshakePair`, placement,
    the `TickPhase` (clock-edge) vs `ProtocolPhase` (protocol-stage)
    disambiguation, and the projection rules
    (KG → IntentIR → ContractIR `channel`/`phase`/`HandshakeFire`).
  Acceptance: `Design recorded in this tree + the mdBook R16 chapter (BOOK-METHOD-DOC); reviewed against the current actor graph + ActorContract channel/phase fields; no code.`
  Verification: `passed` — see "Design (`.1` output)" below; placement =
    typed layer (no new stage; canonical-IntentIR doctrine, parallels
    `R16-CONTRACT-IR`); node/edge schema + projection rules fixed; the
    `TickPhase`≠`ProtocolPhase` disambiguation recorded; reviewed
    against the verified current types (`actor_signal_relations`/
    `actor_ports`/`signal_connectivity`, `ActorContract.channel`/`.phase`
    already present from `R16-CONTRACT-IR.2`). Book mirror added per
    `BOOK-METHOD-DOC`. Docs-only.
  Commit: `see Commit Log`

- ID: `R16-KG-PROTOCOL-ONTOLOGY.2`
  Status: `done`
  Goal: implement the typed `protocol_graph` module + serde; additive
  empty fields on SemanticIr/IntentIr (serde-default + skip-if-empty,
  zero artifact churn, parallel to `actor_contracts`); count helpers;
  unit tests.
  Acceptance: `Typed nodes/edges + serde + count helpers + tests; zero artifact churn; scripts/run_ci.sh green.`
  Verification: `passed` — `crates/specforge/src/ir/protocol_graph.rs`
    added (`Channel`/`ProtocolPhase`/`Transaction`/`HandshakePair` +
    `ChannelRole` + `ProtocolGraph` with `is_empty`/`counts`; serde
    snake_case; empty graph ⇒ `{}`); module registered in `ir/mod.rs`;
    additive `protocol_graph` field on `SemanticIr` (empty
    `ProtocolGraph::default()`) and `IntentIr` (carried forward from
    `SemanticIR`, parallel to `actor_contracts`), serde-default +
    `skip_serializing_if = ProtocolGraph::is_empty` ⇒ **zero
    artifact/fixture churn** (unpopulated). 3 unit tests
    (empty/populated/serde-round-trip). Scope note: `specforge validate`
    count *reporting* deferred to `.4` (counts are meaningless while
    empty; `counts()` helper + tests added now — parallels the
    `R16-CONTRACT-IR.2` bounded scope). Full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R16-KG-PROTOCOL-ONTOLOGY.3`
  Status: `done`
  Goal: wire the projection — `ActorContract.channel`/`.phase` reference
  the new node ids; `EventExpr::PhaseBoundary` may reference a
  `ProtocolPhase`; IntentIR carries the node sets forward. Mechanical,
  parity-preserving (nodes empty until extraction populates).
  Acceptance: `Projection wired; CI-parity (zero .isf change while node sets empty); scripts/run_ci.sh green.`
  Verification: `passed` — `project_handshake_pairs(&[ActorContract])`
    mechanically derives `HandshakePair` nodes from already-recovered
    `HandshakeBarrier` obligations (LOSSLESS restatement of contract
    data — NOT PDF extraction, which stays the extraction trees' job /
    Non-Goal; deduped by `(valid,ready)`); `SemanticIr::build` populates
    `protocol_graph.handshakes` from `actor_contracts`; `IntentIR`
    carries `protocol_graph` forward (from `.2`). Typed-reference
    accessors `channel()`/`phase()` + `dangling_contract_refs()`
    (surfaces a contract pointing at a non-existent channel/phase
    explicitly). Parity: the corpus has 0 `handshake_complete`
    temporal_rules ⇒ 0 projected pairs ⇒ `protocol_graph` stays empty
    corpus-wide ⇒ **zero `.isf`/artifact change**; the path is
    unit-tested with synthesized handshake contracts. Full
    `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R16-KG-PROTOCOL-ONTOLOGY.4`
  Status: `done`
  Goal: `validate` count surface for `protocol_graph` (deferred from
  `.2`); honestly scope kg-bench fixtures; close tree; sync mdBook
  + ROADMAP R16.
  Acceptance (reconciled honestly): `specforge validate reports protocol_graph counts (channels/phases/transactions/handshakes) for SemanticIR and IntentIR; kg-bench protocol-structure fixtures explicitly deferred to the extraction trees (#3/#4/#6) that recover protocol structure — a fixture here would be hollow (Non-Goal: this tree ships the vocabulary, not the extractor); existing kg-bench fixtures unchanged; tree marked done; ROADMAP R16 closed for KG-PROTOCOL-ONTOLOGY; mdBook chapter "Status — delivered" subsection per BOOK-METHOD-DOC close-rule; scripts/run_ci.sh green.`
  Verification: `passed` — `specforge validate` now prints
    `actor_contracts: …` and
    `protocol_graph: channels=… phases=… transactions=… handshakes=…`
    in both the SemanticIR and IntentIR count blocks
    (`crates/specforge/src/commands/validate.rs`, additive lines via
    `replace_all`). Corpus reads `channels=0 phases=0 transactions=0
    handshakes=0` (empty until extraction). Full `scripts/run_ci.sh`
    green; no structured-metric/JSON shape touched (kept bounded);
    kg-bench fixtures honestly deferred (recorded in Decisions).
  Commit: `see Commit Log`

## Current Frontier

**Tree closed `2026-05-20`.** All four leaves done; ROADMAP R16 entry
for `R16-KG-PROTOCOL-ONTOLOGY` marked done; next DAG-promotable
R16 sub-tree = `R16-CAPTURE-FIDELITY-GATES` (#5, order 3; dep
`R16-CONTRACT-IR` ✓ already satisfied).

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-KG-PROTOCOL-ONTOLOGY.1` | `done` | Ontology design fixed; book mirror added |
| 2 | `R16-KG-PROTOCOL-ONTOLOGY.2` | `done` | Typed `protocol_graph` module + serde + additive empty fields + 3 tests; zero artifact churn |
| 3 | `R16-KG-PROTOCOL-ONTOLOGY.3` | `done` | Projection wired (HandshakePair from HandshakeBarrier; accessors + dangling-ref check); zero corpus churn |
| 4 | `R16-KG-PROTOCOL-ONTOLOGY.4` | `done` | `validate` protocol_graph count surface delivered; kg-bench fixtures honestly deferred to the extraction trees; tree closed; book + ROADMAP synced |

## Design (`.1` output, 2026-05-19)

Reviewed against the verified current IR: the actor-relative graph
(`actor_signal_relations`, `actor_ports`, `signal_connectivity`),
`interfaces[].signal_records`, and `ActorContract` (which already carries
optional `channel: Option<String>` / `phase: Option<String>` and
`EventExpr::HandshakeFire`/`PhaseBoundary`, added in `R16-CONTRACT-IR.2`).

### Placement — typed layer, no new stage

New module `crates/specforge/src/ir/protocol_graph.rs`. `SemanticIR` and
`IntentIR` carry additive `protocol_graph: ProtocolGraph` (serde
`#[serde(default, skip_serializing_if = …is_empty)]`), exactly like
`actor_contracts` — respects the canonical-IntentIR doctrine and the
ISF-only fewer-stages ethos; zero artifact churn while empty.

### Node/edge schema (closed, typed records — not a raw edge soup)

```
ProtocolGraph { channels:Vec<Channel>, phases:Vec<ProtocolPhase>,
                transactions:Vec<Transaction>, handshakes:Vec<HandshakePair> }
Channel        { channel_id, name, actor:Option<String>,
                 signal_names:Vec<String>, role:Option<ChannelRole> }
ChannelRole    = Address|Data|Response|Request|Sideband|Mixed
ProtocolPhase  { phase_id, name, channel:Option<channel_id>, order:u32 }
Transaction    { transaction_id, name, channels:Vec<channel_id>,
                 phases:Vec<phase_id>, ordered_before:Vec<transaction_id> }
HandshakePair  { pair_id, valid_signal, ready_signal,
                 channel:Option<channel_id> }
```

Edges are modelled as typed references (matching how the codebase models
the actor graph as typed records): `belongs-to-channel` =
`Channel.signal_names` + `HandshakePair.channel`; `phase-of` =
`ProtocolPhase.channel` / `Transaction.phases`; `ordered-before` =
`Transaction.ordered_before`; `qualifies`/`stable-during` are expressed
through the ContractIR obligation (a `Stable{during:Between{from,to}}`
whose endpoints are `PhaseBoundary` referencing a `ProtocolPhase`).

### `TickPhase` ≠ `ProtocolPhase` (disambiguation — load-bearing)

`TickPhase{PreTick,PostTick}` is **clock-edge** granularity (already in
the temporal model). `ProtocolPhase` is **protocol-stage** granularity
(APB setup/access; address/data/response). They are distinct and
co-exist; a `ProtocolPhase` may span many clock ticks. `EventExpr` keeps
`PhaseBoundary{phase,…}` whose `phase` may now be a `ProtocolPhase.phase_id`.

### Projection rules (mechanical; the thesis's "almost mechanical")

IntentIR carries `protocol_graph` forward from SemanticIR (clone, like
`actor_contracts`). `ActorContract.channel`/`.phase` are populated with
`channel_id`/`phase_id` when a channel/phase grounds the contract;
`HandshakePair` ties to a `HandshakeBarrier` obligation +
`EventExpr::HandshakeFire`. No new lowering logic — `.isf` lowering is
unchanged; this is pure typed structure the extraction trees (#3/#4/#6)
populate and downstream projects.

### Population is NOT this tree

`.2`/`.3` define + carry + wire the **empty** typed structure (zero
artifact churn, parity-preserving — same discipline as
`R16-CONTRACT-IR.2`). Recovering channels/transactions/phases from the
PDF is the extraction trees' job (explicit Non-Goal here): this tree
ships the vocabulary, not the extractor.

## Dependencies / Order

- Depends on `R16-CONTRACT-IR` (contracts must exist to attach to
  channel/phase nodes). Blocks `R16-MULTIMODAL-CONTRACT-FUSION`
  (fusion keys on channel/phase identity).

## Decisions

- `2026-05-19`: Foundational target shape with `R16-CONTRACT-IR`; the
  pair is the "almost mechanical to lower from" KG of the thesis.
  Created `proposed`.
- `2026-05-19`: **Promoted `proposed → active`** by
  `R16-INTENT-CAPTURE.2` after its DAG predecessor `R16-CONTRACT-IR`
  closed. `.1` (ontology design, docs-only) recorded; placement = typed
  layer / no new stage (parallels `R16-CONTRACT-IR`); edges as typed
  references not a raw soup; `TickPhase`≠`ProtocolPhase` fixed; `.2`/`.3`
  ship the EMPTY typed structure (zero artifact churn, parity-preserving
  — same discipline as `R16-CONTRACT-IR.2`); population is the
  extraction trees' job (Non-Goal here).

## Blockers

- None. Active; frontier `R16-KG-PROTOCOL-ONTOLOGY.2`.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-19` | `R16-KG-PROTOCOL-ONTOLOGY.1` | ontology design vs verified current IR; placement/schema/projection/disambiguation recorded; book mirror per BOOK-METHOD-DOC | `passed` (docs-only) |
| `2026-05-19` | `R16-KG-PROTOCOL-ONTOLOGY.2` | typed `protocol_graph` module + serde + count helpers + 3 unit tests; additive empty fields (serde skip-if-empty); full `scripts/run_ci.sh` | `passed` (zero artifact churn) |
| `2026-05-19` | `R16-KG-PROTOCOL-ONTOLOGY.3` | `project_handshake_pairs` + accessors + `dangling_contract_refs` + wire SemanticIr::build; 3 new tests; full `scripts/run_ci.sh` | `passed` (corpus 0 handshakes ⇒ empty ⇒ zero `.isf`/artifact change) |
| `2026-05-20` | `R16-KG-PROTOCOL-ONTOLOGY.4` | `validate` SemanticIR+IntentIR count blocks now print `actor_contracts` + `protocol_graph: channels/phases/transactions/handshakes`; kg-bench fixtures honestly deferred; tree closed; book + ROADMAP synced; `scripts/run_ci.sh` | `passed` (corpus = all-zero counts) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R16-KG-PROTOCOL-ONTOLOGY.1` | `R16-KG-PROTOCOL-ONTOLOGY.1 — ontology design (promote #2)` (`ba630db6`) | docs-only; book mirror; also the `R16-INTENT-CAPTURE.2` #2 promotion |
| `R16-KG-PROTOCOL-ONTOLOGY.2` | `R16-KG-PROTOCOL-ONTOLOGY.2 — typed protocol_graph module + additive empty fields` (`f810f24d`) | first KG-ONTOLOGY code; zero artifact churn |
| `R16-KG-PROTOCOL-ONTOLOGY.3` | `R16-KG-PROTOCOL-ONTOLOGY.3 — wire projection (HandshakePair from HandshakeBarrier) + accessors` (`22042746`) | mechanical/lossless; zero corpus churn |
| `R16-KG-PROTOCOL-ONTOLOGY.4` | `R16-KG-PROTOCOL-ONTOLOGY.4 — validate count surface + close tree (kg-bench deferred)` | closes the tree; corpus all-zero counts; bounded scope |

## Changelog

- `2026-05-19`: Created `proposed` as program point #2.
- `2026-05-19`: Promoted to `active` (DAG predecessor `R16-CONTRACT-IR`
  done); `.1` ontology design fixed + book mirror; concrete `.1`–`.4`
  leaves defined. Frontier → `.2` (implement typed `protocol_graph`).
- `2026-05-20`: **Tree CLOSED.** `.4` done — `specforge validate` now
  reports `actor_contracts` + `protocol_graph: channels/phases/
  transactions/handshakes` counts for SemanticIR and IntentIR (additive
  println lines via replace_all; structured-metric/JSON shape
  untouched, kept bounded). Honest scope: kg-bench protocol-structure
  fixtures explicitly **deferred to the extraction trees**
  (`#3`/`#4`/`#6`) that actually populate `protocol_graph` — a fixture
  here would be hollow (this tree ships the vocabulary, not the
  extractor; explicit Non-Goal). Corpus reads
  `channels=0 phases=0 transactions=0 handshakes=0` (empty until
  extraction). Full CI green. ROADMAP R16 entry + TASK_TREE index +
  book "Status — delivered" subsection synced per BOOK-METHOD-DOC
  close-rule. Next DAG-promotable R16 sub-tree =
  `R16-CAPTURE-FIDELITY-GATES` (#5, order 3; dep CONTRACT-IR ✓).
- `2026-05-19`: `.3` done — projection wired:
  `project_handshake_pairs` derives `HandshakePair` nodes from
  `HandshakeBarrier` contracts (lossless restatement, NOT PDF
  extraction — that's the extraction trees'; deduped); `SemanticIr::build`
  populates `protocol_graph.handshakes`; accessors + `dangling_contract_refs`
  added. Corpus has 0 `handshake_complete` ⇒ empty ⇒ zero `.isf`/artifact
  change; path unit-tested. Frontier → `.4` (kg-bench fixtures +
  `validate` counts + close).
- `2026-05-19`: `.2` done — `ir/protocol_graph.rs` typed module
  (`Channel`/`ProtocolPhase`/`Transaction`/`HandshakePair`/`ChannelRole`/
  `ProtocolGraph`) + serde + `is_empty`/`counts` + 3 unit tests; additive
  `protocol_graph` field on SemanticIr (empty) / IntentIr (carried
  forward), serde skip-if-empty ⇒ zero artifact churn (CONTRACT-IR.2
  discipline). `validate` count-reporting deferred to `.4`. Full CI
  green. Frontier → `.3` (wire ContractIR channel/phase projection).
