# R16-CONSTRAINED-VERIFIED-EXTRACTION: schema-constrained + verified extraction (point #6 — the crux, continuous)

## Metadata

- Tree ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION`
- Status: `active`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #6, order 6 — crux, continuous
  hardening of prose+waveform extraction precision/recall)
- Created: `2026-05-19`
- Last updated: `2026-05-20`
- Owner: repo-local workflow

## Goal

Make extraction high-precision by construction (the program thesis: prose
+ timing-diagram → typed KG accuracy is the crux):

1. **Schema-constrained extraction** — LLM/VLM emit **directly into the
   ContractIR schema** via grammar/JSON-schema-constrained decoding (no
   free-text post-parse).
2. **Entailment verifier** — a second pass checks the exact source span
   (sentence / table cell / figure region) actually *licenses* the
   extracted contract; failed entailment ⇒ residual, never fabricate.
   This makes the residual-honesty doctrine *enforceable*.
3. **Protocol-pattern template library** — canonical temporal templates
   (ready/valid, credit flow control, setup/access, async-assert/
   sync-release reset, burst+last) seeded into prior memory; matched
   templates instantiate at high confidence and concentrate uncertainty
   on the genuinely novel residue.
4. **Uncertainty-driven converge** — each VLM/NLP pass spends on the
   lowest-confidence, highest-impact contracts (value-of-information),
   not blanket rescans.

## Non-Goals

- Not a model/runtime swap — provider-agnostic, reuses existing
  enrich/nlp-enrich/converge infrastructure.
- The verifier may not "soften" a contract to pass; it accepts or routes
  to residual.

## Acceptance Criteria

- Constrained decoding into ContractIR + an entailment-verifier gate +
  a template library in prior memory + uncertainty-driven converge pass
  selection; precision/recall measured by
  `R16-CAPTURE-FIDELITY-GATES` on the real corpus vs. baseline.
- `scripts/run_ci.sh` green per leaf; every leaf via `COMMIT.md`.

## Task Tree

- ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION.1`
  Status: `done`
  Goal: schema-constrained-decoding + entailment-verifier + template-
  library + uncertainty-driven-converge design (docs-only, parallels
  prior R16 `.1`s; the typed primitives ship in `.2`–`.5`).
  Acceptance: `Design recorded in this tree + mirrored in mdBook per BOOK-METHOD-DOC; docs-only; scripts/run_docs_ci.sh green.`
  Verification: `passed` — see "Design (`.1` output)" below; book
    mirror; `mdbook build` green.
  Commit: `see Commit Log`

- ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION.2`
  Status: `pending`
  Goal: derive a JSON-schema for `ActorContract` (the constrained
  target) from the typed Rust definition (via existing serde-derived
  shape or a hand-mirrored schema with a serde round-trip test); add
  a typed adapter `parse_constrained_contract(json: &str) -> Result<ActorContract>`
  that fails closed on schema violations. Adapter is provider-agnostic
  (any LLM/VLM that supports JSON-schema-constrained decoding can
  drive it; the integration is not pinned here).
  Acceptance: `JSON schema present; adapter parses valid contracts + rejects invalid; SemanticIr/IntentIr unchanged; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

- ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION.3`
  Status: `pending`
  Goal: entailment verifier — given (source_span, ActorContract),
  returns Pass / Fail / NotEvaluated. Initial implementation is a
  conservative lexical/structural check (every signal name in the
  contract appears in the span; every numeric bound in the obligation
  matches a number in the span); future iterations may consult an
  LLM-as-judge gated behind the verifier API. A `Fail` reroutes the
  contract to `Residual{reason="entailment fail: …"}` — honesty
  doctrine mechanically enforced (parallels FUSION.3 / FIDELITY.3
  routing).
  Acceptance: `Verifier returns three-valued status; routing tested; SemanticIr unchanged unless the producer in this leaf is wired; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

- ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION.4`
  Status: `pending`
  Goal: protocol-pattern template library seeded into `prior_memory`
  (canonical templates: ready/valid, credit flow control, setup/access,
  async-assert/sync-release reset, burst+last). Matched templates
  instantiate at high confidence; the *match* is itself entailment-
  verifiable (template's promised signals appear in the source).
  Acceptance: `Library seeded; match→instantiate path tested; no fabricated templates (signal grounding is checked); scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

- ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION.5`
  Status: `pending`
  Goal: uncertainty-driven converge — at converge time, pass selection
  prioritizes contracts with `automation_confidence` low AND high
  fidelity-`Fail`-rate; computed as a simple
  value-of-information score over current `fidelity_findings` +
  `automation_confidence`. Bounded budget per pass.
  Acceptance: `Selection function implemented + unit-tested over synthetic findings; integrated into the converge loop iff loop exists in this repo (else parked behind a feature flag); scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

- ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION.6`
  Status: `pending`
  Goal: corpus precision/recall measurement via
  `R16-CAPTURE-FIDELITY-GATES`; baseline-lock; close tree + book +
  ROADMAP R16.
  Acceptance: `Corpus precision/recall measured (vs pre-CVE baseline); regression-locked via CI; tree marked done; ROADMAP R16 closed for CVE; mdBook "Status — delivered" subsection per BOOK-METHOD-DOC; scripts/run_ci.sh green.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.1` | `done` | Constrained-decoding + entailment + templates + uncertainty design fixed; book mirror added |
| 2 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.2` | `pending` | **Next** — derive a JSON schema for ActorContract + typed adapter |
| 3 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.3` | `pending` | Entailment verifier + routing |
| 4 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.4` | `pending` | Protocol-pattern template library in prior_memory |
| 5 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.5` | `pending` | Uncertainty-driven converge selection |
| 6 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.6` | `pending` | Corpus precision/recall + close |

## Design (`.1` output, 2026-05-20)

Closes the program by making extraction **high-precision by
construction**: the prior trees (#1–#5) shaped the typed target,
the protocol graph, the fidelity gates, the fusion of multi-source
candidates, and the figure-mining primitive. This tree turns the
remaining lever — keeping every captured contract honest — into
structural rather than authorial enforcement.

### Placement — typed layer, no new stage

Parallels prior R16 trees. New module
`crates/specforge/src/ir/cve.rs` houses the
constrained-decoding adapter (`.2`), the entailment verifier (`.3`),
the template-library types (`.4`), and the uncertainty selection
helper (`.5`). The prior-memory schema absorbs the template library
in `.4` (`prior_memory.rs` is the existing home for repeated
priors). No new IR stage; `SemanticIr`/`IntentIr` schemas unchanged
through `.2`/`.3`/`.4`/`.5`; the corpus stays unchanged until the
producers in `.3`/`.4`/`.5` legitimately reshape contracts via the
already-enforced honesty doctrines.

### Constrained decoding (`.2`)

Derive a JSON schema for `ActorContract` from its serde-derived
shape (or hand-mirrored with a serde round-trip oracle test).
Provider-agnostic adapter
`parse_constrained_contract(json: &str) -> Result<ActorContract>`
fails closed on schema violations — invalid JSON is a `Result::Err`,
not a fabricated contract. Any LLM/VLM that supports JSON-schema-
constrained decoding (or grammar-constrained, e.g. GBNF) can drive
the adapter; the integration is **not pinned** here so the design
survives provider churn.

### Entailment verifier (`.3`)

Given `(source_span: &str, contract: &ActorContract)` returns
`FindingStatus` (the same three-valued type used elsewhere). Initial
implementation is conservative lexical/structural:

- every signal referenced by `contract` (via existing
  `contract_signals` helper) must appear in `source_span` (case-
  preserving substring match);
- every numeric bound in the obligation (e.g. `Within.max`,
  `Drive.value`) must match a number actually present in the span
  (regex `\d+`);
- otherwise `Pass`. (Future iterations may consult an LLM-as-judge
  gated behind the same API — but never to "soften" a Fail; only to
  upgrade `NotEvaluated` to `Pass` with a separate
  `automation_confidence` adjustment.)

A `Fail` reroutes the contract to
`Residual{reason="entailment fail: <details>"}` — honesty doctrine,
mechanically enforced (parallels `R16-MULTIMODAL-CONTRACT-FUSION.3`
disagreement routing and `R16-CAPTURE-FIDELITY-GATES.3`
Fail-on-Lowerable routing). The verifier never "softens" a contract
to pass; it accepts or routes.

### Template library (`.4`)

Canonical protocol templates seeded into `prior_memory`:
- ready/valid handshake (`AWVALID/AWREADY` pattern; produces a
  `HandshakeBarrier` contract);
- credit flow control (counter-bound `Eventually`);
- setup/access (APB-style phase ordering);
- async-assert / sync-release reset (a `Drive` then an
  `Eventually{Within}` release);
- burst + last (a `Sequence` ending in a `Drive` on `LAST`).

Match is signal-name-keyed against the actor's declared boundary;
matched templates instantiate at `automation_confidence = High`
**only when the template's promised signals are all present** —
otherwise demote to `Medium`. The match itself is
entailment-verifiable (`.3`).

### Uncertainty-driven converge (`.5`)

Pass selection prioritizes a small budget of contracts via a
value-of-information score:

```
voi(c) = w_conf * (1 - rank(c.automation_confidence) / 2)
       + w_fail * count_fail_findings(c)
```

with `w_conf` and `w_fail` constants (initial: 1.0 each). A pass
re-extracts the top-N by `voi`. Bounded budget per pass; deterministic
ordering for reproducibility.

### Cross-tree positioning

- **Schema source** = `R16-CONTRACT-IR` (#1).
- **Objective metric** = `R16-CAPTURE-FIDELITY-GATES` (#5/order-3).
- **Cross-modal consumer** = `R16-MULTIMODAL-CONTRACT-FUSION` (#3).
- **Sibling crux** = `R16-WAVEFORM-CONTRACT-MINING` (#4) — this
  tree is the prose/text counterpart; the entailment verifier here
  is the "round-trip" oracle for prose, parallel to the
  trace-replay verifier for figures (`verify_contract_against_trace`).

### Honest dormancy / Non-Goals

- Not a model/runtime swap — provider-agnostic, reuses existing
  enrich/nlp-enrich/converge infrastructure when those points come
  online.
- Until an upstream prose extractor produces candidates and feeds the
  adapter, the producer-side of `.3`/`.5` is dormant on the corpus
  (the verifier runs whenever it has a span; the template library is
  always available; the schema adapter is always usable). `.6` is
  where measurement becomes meaningful end-to-end.

### Report shape (`.6`, `validate`)

Additive lines in the SemanticIR + IntentIR count blocks:

```
  constrained: schema_rejects=… entailment_fails=… template_hits=…
```

Structured-metric / JSON shape intentionally not touched (bounded;
KG-ONTOLOGY.4 / FIDELITY.4 / FUSION.4 precedents).

## Decisions

- `2026-05-19`: Continuous precision/recall hardening on the crux;
  enforces the residual-honesty doctrine via the verifier. Created
  `proposed`, last in order (depends on schema + metric).
- `2026-05-20`: **Promoted `proposed → active`** by
  `R16-INTENT-CAPTURE.2` after DAG predecessors closed
  (`R16-CONTRACT-IR` ✓, `R16-CAPTURE-FIDELITY-GATES` ✓). `.1` design
  fixed (docs-only): typed layer / no new stage (parallels prior R16
  trees); schema-constrained adapter is **provider-agnostic** (not
  pinned to any LLM/VLM); entailment verifier is the prose-side
  round-trip oracle paired with WAVEFORM's
  `verify_contract_against_trace`; template library lives in
  `prior_memory`; uncertainty-driven converge uses a simple value-of-
  information score; **honesty doctrine MECHANICALLY enforced** at
  every gate (Fail → Residual, never "softening"). Honest dormancy
  through `.2`/`.3`/`.4`/`.5` (no producer churn on the corpus until
  an upstream prose extractor feeds the adapter); `.6` is where
  measurement becomes meaningful end-to-end. Book mirror per
  BOOK-METHOD-DOC.

## Blockers

- None. Active; frontier `R16-CONSTRAINED-VERIFIED-EXTRACTION.2`.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-20` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.1` | constrained-decoding / entailment / templates / uncertainty / honesty enforcement / dormancy framing recorded; book mirror per BOOK-METHOD-DOC; mdBook builds | `passed` (docs-only) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION.1` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.1 — high-precision-by-construction design (promote #6)` | docs-only; book mirror; also the `R16-INTENT-CAPTURE.2` #6 promotion — the FINAL R16 sub-tree promoted |

## Dependencies / Order

- Depends on `R16-CONTRACT-IR` (#1, the schema to constrain into) and
  `R16-CAPTURE-FIDELITY-GATES` (#5/order-3, its objective function).
  Feeds `R16-MULTIMODAL-CONTRACT-FUSION` (#3, already closed and
  ready to consume cross-modal candidates). Pairs with
  `R16-WAVEFORM-CONTRACT-MINING` (#4) — this tree is the prose-side
  counterpart to that tree's figure-side mining.

## Changelog

- `2026-05-19`: Created `proposed` as program point #6, ordered 6th.
- `2026-05-20`: **Promoted `proposed → active`** by
  `R16-INTENT-CAPTURE.2` (DAG predecessors closed); `.1` design
  fixed + book mirror; concrete `.1`–`.6` leaves defined. This is the
  FINAL R16 sub-tree promoted (all 6 program points now active or
  closed). Frontier → `.2` (JSON-schema adapter for ActorContract).
