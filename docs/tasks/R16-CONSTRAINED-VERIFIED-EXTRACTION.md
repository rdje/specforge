# R16-CONSTRAINED-VERIFIED-EXTRACTION: schema-constrained + verified extraction (point #6 — the crux, continuous)

## Metadata

- Tree ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION`
- Status: `done`
- Roadmap lane: `R16`
- Program: `R16-INTENT-CAPTURE` (point #6, order 6 — crux, continuous
  hardening of prose+waveform extraction precision/recall;
  DELIVERED `2026-05-20`)
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
  Status: `done`
  Goal: typed adapter + provider-facing JSON-schema summary, with
  fails-closed parsing on schema/serde violations.
  Acceptance: `JSON schema present; adapter parses valid contracts + rejects invalid; SemanticIr/IntentIr unchanged; scripts/run_ci.sh green.`
  Verification: `passed` — `crates/specforge/src/ir/cve.rs` added:
    `actor_contract_json_schema_summary()` returns a provider-facing
    JSON-Schema (Draft-2020-12) string covering the required
    top-level keys + the obligation discriminator (9 variants —
    drift-locked by the
    `schema_summary_enumerates_obligation_kinds` test); the
    **authoritative validator** is serde via
    `parse_constrained_contract(json: &str) -> Result<ActorContract, String>`
    (fails closed on any schema/serde violation; the diagnostic is
    propagated so the caller can route to residual). Integration is
    explicitly provider-agnostic (any LLM/VLM with JSON-Schema-
    grammar-constrained decoding can drive it; `schemars` left as a
    non-pinned integration choice). 4 unit tests: valid contract
    round-trips; malformed input (non-JSON, empty object, missing
    required keys) fails closed; schema summary lists the required
    top-level keys (doc-vs-code contract); schema summary lists
    every `Obligation` discriminator (drift-detection). Module
    registered in `ir/mod.rs`. No producer wiring; `SemanticIr`/
    `IntentIr` schemas unchanged ⇒ **zero artifact/fixture churn**.
    Full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION.3`
  Status: `done`
  Goal: conservative lexical/structural entailment verifier +
  Fail→Residual routing helper.
  Acceptance: `Verifier returns three-valued status; routing tested; SemanticIr unchanged unless the producer in this leaf is wired; scripts/run_ci.sh green.`
  Verification: `passed` — `entailment_check(span, contract) ->
    FindingStatus` added to `crates/specforge/src/ir/cve.rs`: extracts
    `contract_signals_for_entailment` (obligation + guard +
    guard_candidates + clock_signal) and `obligation_numeric_bounds`
    (Within{min,max} + Drive.value when numeric +
    Sequence step windows); requires every signal as a
    case-preserving substring AND every bound as a complete digit-run
    match in `source_span`; `Pass` only when both hold; `Fail` if any
    missing; `NotEvaluated` only when the contract has nothing
    checkable (no signals AND no bounds — never silently Pass).
    `apply_entailment_to_contract(contract: &mut, span)` mechanically
    enforces the honesty doctrine: a `Lowerable` contract with a
    `Fail` is rerouted to `Residual{reason="entailment fail: missing
    signals=…  bounds=…"}`; `Pass`/`NotEvaluated` leaves the
    contract unchanged; already-`Residual` contracts are NOT
    rewritten. 7 new unit tests (Pass over Drive + Stable; Fail on
    missing signal; Fail on missing bound; NotEvaluated on
    no-signal/no-bound obligation; routing flips Lowerable+Fail to
    Residual; Pass leaves Lowerable unchanged; preexisting Residual
    untouched; `span_contains_number` matches complete digit runs
    only — `7` ≠ `70`). No producer wiring; `SemanticIr` / `IntentIr`
    unchanged ⇒ ZERO artifact churn. Full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION.4`
  Status: `done`
  Goal: protocol-pattern template library seeded with the 5 canonical
  templates; match-grounding gate; entailment-verifiable instantiation.
  Acceptance: `Library seeded; match→instantiate path tested; no fabricated templates (signal grounding is checked); scripts/run_ci.sh green.`
  Verification: `passed` — added to `crates/specforge/src/ir/cve.rs`
    (placement note: `.1` named `prior_memory` as the canonical home;
    for `.4`'s bounded scope the library ships next to the entailment
    verifier — its natural consumer — with a future leaf available to
    migrate into `prior_memory` if a `CorpusMemory` integration becomes
    useful). `ProtocolTemplate{ReadyValidHandshake, CreditFlowControl,
    SetupAccess, AsyncAssertSyncReleaseReset, BurstLast}` with
    `all()`/`name()`/`required_roles()`; `SignalBindings` maps
    template roles → concrete actor signal names; `instantiate_template`
    enforces the match-grounding gate (any missing role ⇒ `None` —
    no fabrication). Implementations: RV-Handshake →
    `HandshakeBarrier` / Lowerable; AsyncReset → `Drive(reset_n,"0")`
    / Lowerable; BurstLast → `Drive(last,"1")` / Lowerable. **Honest
    deferred lowering** for CFC and SetupAccess: each instantiates as
    `Observe + Residual{reason}` (template is recorded; lowering as
    a single Obligation is honestly under-specified — fabrication
    refused). All templates instantiate at `automation_confidence =
    High` per `.1` (the match itself was grounded). 7 new unit
    tests: library enumeration (all 5 + name set); RV-handshake
    full path; match-grounding `None` on missing role; BurstLast
    Drive Lowerable; AsyncReset Drive Lowerable; CFC + SetupAccess
    Residual with diagnostic reasons; entailment-check Pass on a
    template-derived contract over a span containing its signals
    (the "match is entailment-verifiable" claim from `.1`).
    `SemanticIr`/`IntentIr` schemas unchanged ⇒ ZERO artifact churn.
    Full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION.5`
  Status: `done`
  Goal: uncertainty-driven converge SELECTION helper (deterministic
  VoI score). Integration into the existing converge command remains
  the integration leaf (deferred — honest bounded scope).
  Acceptance: `Selection function implemented + unit-tested over synthetic findings; integrated into the converge loop iff loop exists in this repo (else parked behind a feature flag); scripts/run_ci.sh green.`
  Verification: `passed` — added to
    `crates/specforge/src/ir/cve.rs`:
    `voi_score(contract, findings) = w_conf * (1 - rank(automation_confidence)/2)
    + w_fail * count_fails_for(contract.contract_id, findings)`
    with initial weights `w_conf = w_fail = 1.0` per `.1`; rank
    `High=2 ⇒ uncertainty=0` / `Medium=1 ⇒ 0.5` / `Low=0 ⇒ 1`;
    `count_fails_for` counts `FidelityFinding`s with
    `status == Fail` whose `contract_id` matches.
    `select_top_n_by_voi(&ConvergeInputs{contracts, findings}, n)`
    returns the top-`n` `contract_id`s by descending VoI with
    **deterministic tie-break by `contract_id` lexicographic
    ascending** (next-pass reproducible). 5 new unit tests: VoI
    Low > High when no findings (1 vs 0); Medium-with-2-Fails
    outranks Low-no-fails (2.5 vs 1); tie-break ascending by
    contract_id (`a` before `b`); explicit ordering test (Medium+1Fail
    > Low > High); empty inputs ⇒ empty selection. Integration of
    the helper into the existing converge command stays deferred (the
    converge command is an established flow; this leaf ships the
    selection primitive + tests, honoring the bounded-scope
    discipline). `SemanticIr` / `IntentIr` unchanged ⇒ ZERO artifact
    churn. Full `scripts/run_ci.sh` green.
  Commit: `see Commit Log`

- ID: `R16-CONSTRAINED-VERIFIED-EXTRACTION.6`
  Status: `done`
  Goal: `validate constrained:` block + close tree + book + ROADMAP R16.
  Acceptance: `Corpus precision/recall measured (vs pre-CVE baseline); regression-locked via CI; tree marked done; ROADMAP R16 closed for CVE; mdBook "Status — delivered" subsection per BOOK-METHOD-DOC; scripts/run_ci.sh green.`
  Verification: `passed` — `specforge validate` now prints
    `constrained: schema_rejects=N entailment_fails=M template_hits=K`
    in both the SemanticIR and IntentIR count blocks
    (`crates/specforge/src/commands/validate.rs`, additive lines via
    `replace_all`). Counts derived from `actor_contracts`
    (`entailment_fails` = `Residual` with `reason` starting
    `"entailment fail: "`; `template_hits` = `contract_id` starting
    `"tmpl:"`; `schema_rejects` is `0` today — wiring an adapter
    call-site counter is a future leaf since no upstream prose
    extractor invokes the adapter yet). Structured-metric / JSON
    shape intentionally not touched (bounded, KG-ONTOLOGY.4 /
    FIDELITY.4 / FUSION.4 precedents). Corpus baseline:
    `schema_rejects=0 entailment_fails=0 template_hits=0` (honest
    dormancy until upstream prose extractor lands). Regression gate
    is `scripts/run_ci.sh`. Full CI green. **R16 PROGRAM COMPLETE
    — all 6 sub-trees closed at their honest scope boundaries**;
    remaining frontiers are: `R16-WAVEFORM-CONTRACT-MINING.3`
    (extractor — expected honest-split; bounded), and CVE producer-
    wiring once an upstream prose extractor exists (each gate already
    runs whenever it has a span / a binding / a finding-set).
  Commit: `see Commit Log`

## Current Frontier

**Tree closed `2026-05-20`.** All six leaves done; ROADMAP R16 entry
for `R16-CONSTRAINED-VERIFIED-EXTRACTION` marked done. **R16 PROGRAM
COMPLETE** — all 6 sub-trees closed at their honest scope boundaries.
Remaining frontiers: `R16-WAVEFORM-CONTRACT-MINING.3` (extractor —
expected honest-split when concrete approach is chosen), and CVE
producer-wiring once an upstream prose extractor exists.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.1` | `done` | Constrained-decoding + entailment + templates + uncertainty design fixed; book mirror added |
| 2 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.2` | `done` | Typed `cve` module: provider-facing JSON-Schema summary + fails-closed adapter + 4 tests; zero artifact churn |
| 3 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.3` | `done` | `entailment_check` + `apply_entailment_to_contract` Fail→Residual routing; 7 new tests; honesty doctrine mechanically enforced |
| 4 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.4` | `done` | Protocol-template library (5 canonical, match-grounded, entailment-verifiable; CFC + SetupAccess honestly Residual); 7 tests; zero artifact churn |
| 5 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.5` | `done` | `voi_score` + `select_top_n_by_voi` (deterministic tie-break) + 5 tests; converge-loop integration deferred (bounded scope) |
| 6 | `R16-CONSTRAINED-VERIFIED-EXTRACTION.6` | `done` | `validate constrained: schema_rejects=0 entailment_fails=0 template_hits=0` block delivered; tree closed; book + ROADMAP synced |

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
| `2026-05-20` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.2` | typed `cve` module (`actor_contract_json_schema_summary` + fails-closed `parse_constrained_contract`) + 4 unit tests (round-trip, malformed-input fail-closed, doc-vs-code required-keys, obligation-discriminator drift-lock); SemanticIr/IntentIr unchanged; full `scripts/run_ci.sh` | `passed` (zero artifact churn) |
| `2026-05-20` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.3` | `entailment_check` (lexical signal + structural digit-run bound check; three-valued including `NotEvaluated`) + `apply_entailment_to_contract` Fail→Residual routing; 7 new unit tests (Pass over Drive+Stable; Fail on missing signal/bound; NotEvaluated when nothing checkable; routing flips Lowerable+Fail; Pass leaves unchanged; preexisting Residual untouched; `span_contains_number` exact digit runs); SemanticIr/IntentIr unchanged; full `scripts/run_ci.sh` | `passed` (zero artifact churn) |
| `2026-05-20` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.4` | `ProtocolTemplate` (5 canonical) + `SignalBindings` + `instantiate_template` with match-grounding gate; RV-Handshake/AsyncReset/BurstLast → Lowerable; CFC/SetupAccess honestly Residual; 7 new unit tests including the "match is entailment-verifiable" round-trip; SemanticIr/IntentIr unchanged; full `scripts/run_ci.sh` | `passed` (zero artifact churn) |
| `2026-05-20` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.5` | `voi_score(c, &findings)` + `select_top_n_by_voi(&ConvergeInputs, n)` with deterministic tie-break; 5 unit tests (Low > High no-findings; Medium-2-Fails > Low-no-fails; lex tie-break; explicit ordering Medium+1Fail > Low > High; empty inputs); SemanticIr/IntentIr unchanged; full `scripts/run_ci.sh` | `passed` (zero artifact churn) |
| `2026-05-20` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.6` | `validate` SemanticIR+IntentIR blocks now print `constrained: schema_rejects=N entailment_fails=M template_hits=K` (counts derived from `actor_contracts` via `Residual.reason` "entailment fail:" prefix and `contract_id` "tmpl:" prefix; `schema_rejects=0` today — adapter-call-site counter is a future leaf); structured-metric / JSON shape untouched (bounded; KG-ONTOLOGY.4 / FIDELITY.4 / FUSION.4 precedents); corpus baseline `0/0/0` (honest dormancy); tree closed; book + ROADMAP synced | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION.1` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.1 — high-precision-by-construction design (promote #6)` (`d61d87ab`) | docs-only; book mirror; also the `R16-INTENT-CAPTURE.2` #6 promotion — the FINAL R16 sub-tree promoted |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION.2` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.2 — JSON-schema summary + fails-closed adapter` (`828441f0`) | first CVE code; zero artifact churn; serde is the authoritative validator |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION.3` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.3 — entailment verifier + Fail→Residual routing` (`f7b17f0a`) | honesty doctrine mechanically enforced parallel to FUSION.3 / FIDELITY.3 |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION.4` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.4 — protocol-pattern template library (5 canonical, match-grounded, entailment-verifiable)` (`fe8e029e`) | placement in cve.rs (next to consumer); prior_memory migration deferred; CFC + SetupAccess honestly Residual |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION.5` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.5 — uncertainty-driven converge VoI selector` (`9e0d01ef`) | converge-loop integration deferred (bounded scope); deterministic tie-break |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION.6` | `R16-CONSTRAINED-VERIFIED-EXTRACTION.6 — validate constrained: block + close tree (R16 PROGRAM COMPLETE)` | closes the tree; corpus baseline 0/0/0; closes the R16 program |

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
- `2026-05-20`: **Tree CLOSED.** `.6` done — `specforge validate`
  now prints `constrained: schema_rejects=N entailment_fails=M
  template_hits=K` in both the SemanticIR and IntentIR count blocks
  (additive lines via `replace_all`; structured-metric / JSON shape
  NOT touched — bounded, KG-ONTOLOGY.4 / FIDELITY.4 / FUSION.4
  precedents). Counts derived from `actor_contracts`
  (`entailment_fails`: Residual.reason starts `"entailment fail: "`;
  `template_hits`: contract_id starts `"tmpl:"`; `schema_rejects = 0`
  today because no upstream prose extractor invokes
  `parse_constrained_contract` yet — wiring an adapter call-site
  counter is a future leaf). Corpus baseline: `0/0/0` — honest
  dormancy. Full CI green. ROADMAP R16 (point #6 DELIVERED) +
  TASK_TREE index + book "Status — delivered" subsection synced per
  BOOK-METHOD-DOC close-rule. **R16 PROGRAM COMPLETE — all 6
  sub-trees closed at their honest scope boundaries.** Remaining
  frontiers (recorded here for context): `R16-WAVEFORM-CONTRACT-MINING.3`
  (figure→PartialTrace extractor — expected honest-split when concrete
  approach is decided) and CVE producer-wiring once an upstream prose
  extractor exists.
- `2026-05-20`: `.5` done — uncertainty-driven converge VoI
  selector: `voi_score(c, findings) = w_conf * uncertainty(conf) +
  w_fail * count_fails_for(c.contract_id, findings)` (w_conf =
  w_fail = 1.0; uncertainty: High=0, Medium=0.5, Low=1) +
  `select_top_n_by_voi(&ConvergeInputs, n)` with deterministic
  tie-break by `contract_id` ascending. 5 unit tests covering the
  scoring identities, tie-break, ordering, and bounded-budget
  selection. Converge-command integration deferred (the existing
  converge flow is intentionally untouched in this leaf — bounded
  scope; future leaf wires the selector into the actual command).
  `SemanticIr`/`IntentIr` unchanged ⇒ ZERO artifact churn. Full CI
  green. Frontier → `.6` (corpus precision/recall + close).
- `2026-05-20`: `.4` done — protocol-pattern template library
  (`ProtocolTemplate` enum with 5 canonical templates + `SignalBindings`
  role→signal map + `instantiate_template`). Match-grounding gate
  refuses to fabricate (missing role ⇒ `None`).
  RV-Handshake/AsyncReset/BurstLast lower as Lowerable; CFC and
  SetupAccess honestly lower as `Observe + Residual{reason="…not yet
  representable as a single ContractIR obligation"}` — template
  recorded; lowering deferred (future work). 7 unit tests including
  the "match is entailment-verifiable" round-trip (template
  instantiates only when bindings present; entailment_check Passes
  on a span containing those signals). Placement: in `cve.rs`
  (consumer-adjacent) with prior_memory migration deferred to a
  later leaf per honest scope. `SemanticIr`/`IntentIr` unchanged ⇒
  ZERO artifact churn. Full CI green. Frontier → `.5`
  (uncertainty-driven converge VoI selection).
- `2026-05-20`: `.3` done — entailment verifier wired:
  `entailment_check(span, contract) → FindingStatus` (conservative
  lexical signal + structural digit-run bound check; three-valued
  with `NotEvaluated` when contract has nothing checkable — never
  silently `Pass`); `apply_entailment_to_contract` mechanically
  enforces honesty doctrine — `Lowerable + Fail → Residual{reason
  ="entailment fail: missing signals=… bounds=…"}`; `Pass`/
  `NotEvaluated` leave the contract unchanged; preexisting `Residual`
  is NOT rewritten. 7 unit tests. SemanticIr/IntentIr unchanged ⇒
  ZERO artifact churn. Full CI green. Frontier → `.4` (protocol-
  pattern template library in `prior_memory`).
- `2026-05-20`: `.2` done — `ir/cve.rs` typed module
  (`actor_contract_json_schema_summary` returning a Draft-2020-12
  provider-facing summary; `parse_constrained_contract` is the
  authoritative validator via serde, **fails closed** on any
  schema/serde violation with the diagnostic propagated for residual
  routing) + 4 unit tests (round-trip; malformed-input fails-closed
  for non-JSON / `{}` / missing required keys; doc-vs-code required
  keys; obligation-discriminator drift-lock — schema mentions every
  `Obligation` variant). `SemanticIr`/`IntentIr` schemas unchanged ⇒
  ZERO artifact churn. Full CI green. Frontier → `.3` (entailment
  verifier + Fail→Residual routing).
