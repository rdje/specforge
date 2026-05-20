# R16-MULTIMODAL-CONTRACT-FUSION: cross-modal evidence → one contract (point #3)

## Metadata

- Tree ID: `R16-MULTIMODAL-CONTRACT-FUSION`
- Status: `active`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #3, order 4)
- Created: `2026-05-19`
- Last updated: `2026-05-20`
- Owner: repo-local workflow

## Goal

Today `temporal_rules` are built ~per-sentence/row, so a single
obligation spread across prose §3.1 + a timing table in §3.4 + Figure
3-2 + an exception in §3.5 is never assembled into one object — recall
is silently lost at the join. Add a **contract-assembly** phase that
clusters multimodal evidence keyed by `(actor, channel/signal-group,
phase)` into one ContractIR object with: full multi-modal provenance, a
typed merge, and an explicit **disagreement surface** (sources that
contradict become a residual/repair packet, not a silent pick).

## Non-Goals

- Not a new extractor — it fuses candidates produced by existing
  extraction + #4/#6 into the typed contract; it does not invent
  obligations no source licenses.

## Acceptance Criteria

- A fusion pass clusters by `(actor, channel/group, phase)` (using the
  `R16-KG-PROTOCOL-ONTOLOGY` identity), merges into one ContractIR
  contract with provenance, and surfaces inter-source disagreement as an
  explicit residual; measured by `R16-CAPTURE-FIDELITY-GATES`.
- Recall improvement demonstrated on the real corpus vs. the
  pre-fusion baseline; `scripts/run_ci.sh` green per leaf.

## Task Tree

- ID: `R16-MULTIMODAL-CONTRACT-FUSION.1`
  Status: `done`
  Goal: fusion-key + typed-merge algebra + disagreement-policy +
  provenance-fingerprint design (docs-only, parallels prior R16 `.1`s).
  Acceptance: `Design recorded in this tree + mirrored in mdBook per BOOK-METHOD-DOC; docs-only; scripts/run_docs_ci.sh green.`
  Verification: `passed` — see "Design (`.1` output)" section below;
    book mirror; `mdbook build` green.
  Commit: `see Commit Log`

- ID: `R16-MULTIMODAL-CONTRACT-FUSION.2`
  Status: `done`
  Goal: implement typed `fusion` module: `FusionKey` +
  `fusion_key(&ActorContract) -> FusionKey` +
  `merge_cluster(&[ActorContract]) -> ActorContract` deterministic;
  agreement merges preserve provenance, disagreements route to
  `Residual{reason="disagreement: …"}`.
  Acceptance: `Typed module + merge primitive + agreement/disagreement tests; no producer wiring; zero artifact churn; scripts/run_ci.sh green.`
  Verification: `passed` — `crates/specforge/src/ir/fusion.rs` added:
    `FusionKey { actor, channel, phase, obligation_kind, primary_signal }`
    (Hash+Eq for grouping); `obligation_kind(&Obligation)` (9
    variants) + `fusion_key(&ActorContract)`; `merge_cluster` is size-1
    identity OR deterministic merge — kind/obligation/guard
    disagreements collected, `disagreement_fields` sorted+deduped, set
    `lowering = Residual{reason="disagreement: …"}` on any diff;
    provenance preserved on both paths (union
    `supporting_statement_ids`, `Mixed` modality when sources differ,
    delimited `source_text`, minimum `automation_confidence` via
    `min_confidence`); `contract_id = "fused:<id1>+<id2>+…"`. 6 unit
    tests covering FusionKey discrimination (actor / primary signal),
    size-1 identity, agreement-merge provenance shape, single-field
    disagreement, multi-field sorted-dedup disagreement, and the
    `obligation_kind` round-trip. Module registered in `ir/mod.rs`.
    Producer wiring deferred to `.3` (`SemanticIr`/`IntentIr` schemas
    unchanged ⇒ zero artifact/fixture churn). Clippy-clean
    (`#[allow(clippy::too_many_arguments)]` on the test helper).
    Full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R16-MULTIMODAL-CONTRACT-FUSION.3`
  Status: `pending`
  Goal: wire producer in `SemanticIr::build` — cluster
  `actor_contracts` by `fusion_key`, merge clusters of size > 1
  (deterministic), replace originals with the fused contract;
  disagreement → `Residual{reason}` (honesty doctrine, parallel to
  `R16-CAPTURE-FIDELITY-GATES.3`). IntentIR carries the fused
  contracts (already done in CONTRACT-IR.3). Corpus parity: today
  `actor_name`/`channel`/`phase` are unpopulated for most contracts
  (extraction trees will populate), so clusters are size 1 ⇒ fusion is
  identity ⇒ zero artifact churn. The producer becomes load-bearing
  when extraction lands.
  Acceptance: `Producer wired; per-leaf agreement/disagreement clustering tested; corpus parity preserved (zero artifact churn today; honest residual on synthetic disagreement); scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

- ID: `R16-MULTIMODAL-CONTRACT-FUSION.4`
  Status: `pending`
  Goal: `validate fusion:` count surface (`groups_merged`,
  `disagreements`) for SemanticIR + IntentIR; close tree + book +
  ROADMAP R16.
  Acceptance: `validate prints fusion block; corpus baseline locked (today: groups_merged=0, disagreements=0 — honest dormancy until extraction populates multi-source candidates); tree marked done; ROADMAP R16 closed for MULTIMODAL-CONTRACT-FUSION; mdBook "Status — delivered" subsection per BOOK-METHOD-DOC; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-MULTIMODAL-CONTRACT-FUSION.1` | `done` | Fusion design fixed; book mirror added |
| 2 | `R16-MULTIMODAL-CONTRACT-FUSION.2` | `done` | Typed `fusion` module + `FusionKey` + `merge_cluster` (agree/disagree) + 6 tests; zero artifact churn |
| 3 | `R16-MULTIMODAL-CONTRACT-FUSION.3` | `pending` | **Next** — producer wiring in `SemanticIr::build` before `apply_fidelity_gates` |
| 4 | `R16-MULTIMODAL-CONTRACT-FUSION.4` | `pending` | Corpus report + close |

## Design (`.1` output, 2026-05-20)

The pipeline today builds `actor_contracts` one-per-`temporal_rule`,
so an obligation distributed across prose §3.1 + a timing table §3.4 +
Figure 3-2 + an exception in §3.5 lives as 4 separate contracts; the
contract-level recall is lost at the join. This tree adds a
**deterministic fusion phase** that clusters multimodal candidates by
a typed `FusionKey` into one `ActorContract`, with provenance union,
`Mixed` modality, and an explicit **disagreement → Residual** route
that mechanically refuses to silently pick one source over another.
Design parallels prior R16 `.1`s: typed layer, no new stage, additive
producer in `SemanticIr::build`, honest doctrine enforcement.

### Placement — typed layer, no new stage

A new `crates/specforge/src/ir/fusion.rs` defines `FusionKey` and the
merge primitive. The fused outcome is still an `ActorContract` (no
schema change to `ActorContract` itself); fusion is reflected through
`provenance` (`modality = Mixed`, `supporting_statement_ids` = union,
`source_text` = delimited concatenation) and, on disagreement, the
fused contract's `lowering = Residual{reason="disagreement: …"}`.
`.2`/`.3` ship the primitive and the producer; corpus has size-1
clusters today ⇒ zero artifact churn (the
`R16-CAPTURE-FIDELITY-GATES.2`/`.3` discipline).

### `FusionKey`

```rust
pub struct FusionKey {
    pub actor: Option<String>,       // ActorContract.actor_name
    pub channel: Option<String>,     // ActorContract.channel (populated by extraction trees)
    pub phase: Option<String>,       // ActorContract.phase   (populated by extraction trees)
    pub obligation_kind: &'static str, // "drive" / "stable" / "handshake_barrier" / ...
    pub primary_signal: Option<String>, // obligation's primary signal
}
```

Two contracts cluster iff their `FusionKey`s are equal. `None`/`None`
counts as equal (so weakly-grounded contracts can still cluster on the
other fields). Cross-actor contracts never cluster (correct — they
target different actors).

### Agreement merge (deterministic)

Two contracts in the same cluster *agree* iff their `obligation` value
is structurally equal AND their `guard` and `kind` agree.
The merge yields one `ActorContract`:
- `obligation`, `guard`, `kind`, `clock_signal`, `edge`, `channel`,
  `phase` — taken from the common value;
- `guard_candidates` — union, preserving source order, dedup;
- `provenance.modality = Mixed` when modalities differ, else common;
- `provenance.supporting_statement_ids` — union (preserving source order, dedup);
- `provenance.source_text` — delimited concatenation
  (`"<a> | <b>"`);
- `lowering` — taken from the common value when all agree; else
  `Residual{reason="disagreement: lowering"}`;
- `automation_confidence` — minimum (conservative).

### Disagreement detection

Two contracts in the same cluster *disagree* iff their
`obligation` / `guard` / `kind` are not structurally equal. The
fused contract carries the **union** of all sources' provenance and
sets `lowering = Residual{reason = "disagreement: <obligation|guard|kind diff>"}`
— the honesty doctrine, mechanically enforced (never a silent pick).
This makes "two sources contradict" a first-class IR observation.

### Producer wiring (`.3`)

`SemanticIr::build` clusters `actor_contracts` by `fusion_key`, merges
each cluster of size > 1 via `merge_cluster`, replaces the originals
with the merged contract. Clusters of size 1 are unchanged. The
fidelity producer (`apply_fidelity_gates`, from
`R16-CAPTURE-FIDELITY-GATES.3`) runs **after** fusion so the gates see
the fused contracts. IntentIR carries forward (no schema change
needed).

### Report shape (`.4`, `validate`)

Additive lines in the SemanticIR + IntentIR count blocks of `specforge
validate`:

```
  fusion: groups_merged=… disagreements=…
```

Structured-metric / JSON shape NOT touched (bounded, mirrors the
`R16-KG-PROTOCOL-ONTOLOGY.4` / `R16-CAPTURE-FIDELITY-GATES.4`
precedents).

### Non-Goals (recorded)

- Not a new extractor — fuses candidates extraction produces; never
  invents an obligation no source licenses.
- Recall-improvement *measurement* on the corpus is necessarily `0`
  today (no multi-source candidates yet — extraction is `#4`/`#6`).
  The corpus baseline at `.4` will read `groups_merged=0
  disagreements=0` — honest dormancy, not a faked Pass; the primitive
  is unit-tested with synthetic multi-modal candidates.

## Decisions

- `2026-05-19`: The join/aggregation step that converts statement-local
  facts into protocol-level intent; created `proposed`.
- `2026-05-20`: **Promoted `proposed → active`** by
  `R16-INTENT-CAPTURE.2` after all 3 DAG predecessors closed
  (`R16-CONTRACT-IR`, `R16-KG-PROTOCOL-ONTOLOGY`,
  `R16-CAPTURE-FIDELITY-GATES`). `.1` design fixed (docs-only): typed
  layer / no new stage (parallels prior R16 trees); `FusionKey`
  on (actor, channel, phase, obligation_kind, primary_signal); merge
  is provenance-union + `Mixed` modality + dedup; disagreement →
  `Residual{reason="disagreement: …"}` (honesty doctrine, mechanically
  enforced, parallel to `R16-CAPTURE-FIDELITY-GATES.3`); recall
  *measurement* is honestly dormant today (no multi-source candidates
  yet — extraction is `#4`/`#6`); structured-metric / JSON shape
  intentionally not touched in `.4` (bounded — KG-ONTOLOGY.4 /
  FIDELITY.4 precedents). Book mirror per BOOK-METHOD-DOC.

## Blockers

- None. Active; frontier `R16-MULTIMODAL-CONTRACT-FUSION.2`.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-20` | `R16-MULTIMODAL-CONTRACT-FUSION.1` | fusion-key / merge / disagreement-policy / report shape recorded; book mirror per BOOK-METHOD-DOC; mdBook builds | `passed` (docs-only) |
| `2026-05-20` | `R16-MULTIMODAL-CONTRACT-FUSION.2` | typed `fusion` module (`FusionKey`/`obligation_kind`/`fusion_key`/`merge_cluster`) + 6 unit tests; SemanticIr/IntentIr schemas unchanged; clippy-clean; full `scripts/run_ci.sh` | `passed` (zero artifact churn) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R16-MULTIMODAL-CONTRACT-FUSION.1` | `R16-MULTIMODAL-CONTRACT-FUSION.1 — fusion design (promote #3)` (`f50b2e1e`) | docs-only; book mirror; also the `R16-INTENT-CAPTURE.2` #3 promotion |
| `R16-MULTIMODAL-CONTRACT-FUSION.2` | `R16-MULTIMODAL-CONTRACT-FUSION.2 — typed fusion module + merge primitive + tests` | first FUSION code; zero artifact churn; producer wiring deferred to `.3` |

## Dependencies / Order

- Depends on `R16-CONTRACT-IR` (#1), `R16-KG-PROTOCOL-ONTOLOGY` (#2),
  `R16-CAPTURE-FIDELITY-GATES` (#5/order-3). Consumes candidates from
  `R16-WAVEFORM-CONTRACT-MINING` (#4) and
  `R16-CONSTRAINED-VERIFIED-EXTRACTION` (#6).

## Changelog

- `2026-05-19`: Created `proposed` as program point #3, ordered 4th.
- `2026-05-20`: Promoted to `active` (all 3 DAG predecessors closed);
  `.1` fusion design fixed + book mirror; concrete `.1`–`.4` leaves
  defined. Frontier → `.2` (implement typed `fusion` module).
- `2026-05-20`: `.2` done — `ir/fusion.rs` typed module
  (`FusionKey{actor,channel,phase,obligation_kind,primary_signal}` +
  `obligation_kind(9 variants)` + `fusion_key` + deterministic
  `merge_cluster` with agreement-merge / disagreement-routing +
  `min_confidence` helper) + 6 unit tests (key discrimination, size-1
  identity, agreement provenance shape, single-field disagreement,
  multi-field sorted-dedup disagreement, obligation_kind round-trip);
  `SemanticIr`/`IntentIr` schemas unchanged ⇒ ZERO artifact churn.
  Clippy-clean (`#[allow(clippy::too_many_arguments)]` on the test
  helper). Full CI green. Frontier → `.3` (wire producer in
  `SemanticIr::build` BEFORE `apply_fidelity_gates`).
