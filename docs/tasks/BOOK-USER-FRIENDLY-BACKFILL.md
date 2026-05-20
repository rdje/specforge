# BOOK-USER-FRIENDLY-BACKFILL: upgrade existing book subsections to the user-friendly standard

## Metadata

- Tree ID: `BOOK-USER-FRIENDLY-BACKFILL`
- Status: `active`
- Roadmap lane: `R0` (live-doc continuity / accuracy)
- Created: `2026-05-20`
- Last updated: `2026-05-20`
- Owner: repo-local workflow

## Goal

Bring every existing per-tree book method-doc subsection up to the
**user-friendly thorough** standard established `2026-05-20` (per
the `BOOK-METHOD-DOC.md` Decisions entry + the
`feedback-book-user-friendly` memory). The standard, recorded
verbatim from the user's direction:

> *"the user shall be able to understand everything; user-friendly;
> engage not scare; explain things so they understand; we want them
> to understand if we want them to use specforge."*

The worked template is the `R7-VALIDATION.5` subsection in
`docs/book/src/quality/validation.md` (rewritten `2026-05-20`).
Every concept-introducing subsection that pre-dates that rewrite
needs to meet the same bar.

## Non-Goals

- **Not** rewriting subsections that are already meeting the
  standard (e.g. the R7-VALIDATION.5 template itself).
- **Not** rewriting one-paragraph closure summaries that don't
  introduce new concepts (e.g. *"R6-SOURCE-HARDENING closed
  zero-coverage assertion gaps on SourceIR via mutation testing"*
  is concept-light and concise; it does what it needs to do).
  Per the AUDIT-DOC-RECONCILE doctrine, *accurate* is non-negotiable;
  *more-words* is not a goal when fewer is honest.
- **Not** modifying the task-tree files — those remain the spec
  authority; the book is the human-facing explanation.
- **Not** widening any tree's claims — book language describes what
  the code/design actually does, never more.

## Acceptance Criteria

- Every concept-introducing book subsection (the ones that
  introduce new typed records, new doctrines, new pipeline
  surfaces, etc.) meets the standard:
  - **thorough** — mirrors every concept the tree introduces,
    not just the typed names;
  - **accurate** — language describes what the design / code does;
  - **user-friendly** — explains *why* before *what*; introduces
    typed names alongside plain-language summaries; walks through
    worked examples; frames safety properties as user benefits;
  - **engage-not-scare** — opens with the user's problem, not the
    type lattice; uses second-person; short paragraphs; code
    blocks sparingly.
- `scripts/run_docs_ci.sh` green per leaf (mdBook builds).
- Every leaf via `COMMIT.md`.

## Scope audit (`.1` output, 2026-05-20)

Existing book subsections, classified by whether they need the
full user-friendly upgrade (concept-introducing) or are
acceptable as-is (concise closure summary):

| Subsection | File | Class | Action |
| --- | --- | --- | --- |
| `R16-CONTRACT-IR` — how it is implemented and verified + Status delivered | `direction/temporal-intent-capture.md` | concept-introducing (typed `ActorContract` / `Obligation` algebra) | upgrade (`.2`) |
| `R16-KG-PROTOCOL-ONTOLOGY` — how it is implemented and verified + Status | `direction/temporal-intent-capture.md` | concept-introducing (`Channel` / `ProtocolPhase` / `Transaction` / `HandshakePair`) | upgrade (`.2`) |
| `R16-CAPTURE-FIDELITY-GATES` — how it is implemented and verified + Status | `direction/temporal-intent-capture.md` | concept-introducing (6 gates / 3-valued `FindingStatus` / `FidelitySummary` / honesty routing) | upgrade (`.2`) |
| `R16-MULTIMODAL-CONTRACT-FUSION` — how it is implemented and verified + Status | `direction/temporal-intent-capture.md` | concept-introducing (`FusionKey` / `merge_cluster` / disagreement routing) | upgrade (`.2`) |
| `R16-WAVEFORM-CONTRACT-MINING` — how it is implemented and verified + `.3.1` + Status | `direction/temporal-intent-capture.md` | concept-introducing (`PartialTrace` / generalizer rules / round-trip verifier / `FigureRegion` / adapter) | upgrade (`.2`) |
| `R16-CONSTRAINED-VERIFIED-EXTRACTION` — how it is implemented and verified + Status R16 program complete | `direction/temporal-intent-capture.md` | concept-introducing (`parse_constrained_contract` / `entailment_check` / `ProtocolTemplate` / VoI selector) | upgrade (`.2`) |
| ISF / FSMGen 7-tree backfill | `pipeline/isf-adapter.md` | concise closure summaries; no new concepts beyond the .isf adapter chapter's own coverage | keep as-is |
| `AUDIT-DOC-RECONCILE` | `reference/documentation-scope.md` | doctrine-introducing (text-describes-code) | light upgrade — frame the doctrine in user terms (`.3`) |
| `SIGNOFF-REMEDIATION` | `reference/live-docs.md` | doctrine-introducing (signoff is non-negotiable; idiomatic-fixes-only) | light upgrade — frame the doctrine in user terms (`.3`) |
| R6 hardening trees (6 of them) | `pipeline/*` + `quality/corpus-memory.md` | concise closure summaries (mutation-testing methodology) | keep as-is |
| `PROVENANCE-HARDENING` | `quality/validation.md` | concise closure summary | keep as-is |
| `R7-VALIDATION` close-out summary | `quality/validation.md` | mostly concise but introduces the per-leaf finding-and-metric structure | light upgrade (`.4`) |
| `R7-VALIDATION.5` design | `quality/validation.md` | **already the worked template** | keep |
| `R15-GRAPH-DIRECTION-MIGRATION` | `domain/actor-connectivity.md` | doctrine-introducing (actor-relative graph as source of truth) | light upgrade — frame in user terms (`.5`) |

Honest scope total: **6 R16 family subsections** (`.2`) +
**2 doctrine subsections** in `reference/*` (`.3`) +
**1 closure summary** that introduces structure (`.4`) +
**1 doctrine subsection** in `domain/*` (`.5`). Concise closure
summaries (ISF/FSMGen + R6 + PROVENANCE) are left as-is per the
Non-Goal above — accurate is the bar; more-words is not.

## Task Tree

- ID: `BOOK-USER-FRIENDLY-BACKFILL.1`
  Status: `done` (`2026-05-20`)
  Goal: tree scaffolding + scope audit (recorded above).
  Acceptance: `Tree + scope audit recorded; chapters classified into upgrade-needed vs keep-as-is; mdBook still builds.`
  Verification: `passed` — see "Scope audit (.1 output)" above;
    `scripts/run_docs_ci.sh` green.
  Commit: `see Commit Log`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.2`
  Status: `in-progress` (honest-split into per-subsection sub-leaves
  for reviewable diffs; each R16 subsection is substantial)
  Goal: upgrade the 6 R16-family subsections in
  `docs/book/src/direction/temporal-intent-capture.md` to the
  user-friendly standard (use the `R7-VALIDATION.5` template).
  Each subsection introduces typed records / doctrines a new user
  needs to understand to use SpecForge; this is the highest-impact
  leaf for user adoption.
  Acceptance: `All 6 R16 family subsections rewritten to meet the user-friendly standard; mdBook green; per-tree task-tree files unchanged.`

  - ID: `BOOK-USER-FRIENDLY-BACKFILL.2.a`
    Status: `done` (`2026-05-20`)
    Goal: rewrite the `R16-CONTRACT-IR` subsection (program DAG
    root; introduces `ActorContract` + closed operator algebra
    + lowering disposition + the four user-facing guarantees).
    Acceptance: `Subsection meets standard; mdBook green.`
    Verification: `passed` — opens with the user's problem
      (the bound obligation that gets shredded into a
      bag-of-predicates); one-sentence mental model; typed
      `ActorContract` walked through field-by-field in plain
      language; closed `Obligation` / `EventExpr` / `Window`
      algebra enumerated with plain-language explanations;
      `Observe` framed as the "honest weak fact" doctrine;
      `.isf` lowering pathway walked through; four user-facing
      guarantees framed as benefits ("you can hold one
      contract in one place"; "you always know what got
      lowered and what didn't"; "you can trust the lowering
      to be reproducible"; "the honesty doctrine is
      structural"); Status — delivered section concise but
      complete, with honest constraints recorded (FSMGen
      ready-is-input requirement; `(stage …)` dormant on the
      current corpus). mdBook green.
    Commit: `see Commit Log`

  - ID: `BOOK-USER-FRIENDLY-BACKFILL.2.b`
    Status: `done` (`2026-05-20`)
    Goal: rewrite the `R16-KG-PROTOCOL-ONTOLOGY` subsection.
    Acceptance: `Subsection meets standard; mdBook green.`
    Verification: `passed` — opens with the user's problem
      (every protocol PDF is organised around channels +
      phases + transactions; the IR had none of those as
      first-class records); one-sentence mental model;
      typed `ProtocolGraph` walked through record-by-record
      (Channel/ProtocolPhase/Transaction/HandshakePair) in
      plain language with concrete protocol examples
      (AXI AW/W/B; APB setup/access; TileLink A..E); the
      `TickPhase ≠ ProtocolPhase` distinction explained
      with the two questions they each answer; the two
      population paths (mechanical projection from
      contracts vs PDF extraction) clearly separated
      with trust levels; accessors covered; what you see in
      the report today (the all-zero baseline as honest
      dormancy); three user-facing guarantees framed as
      benefits; Status delivered with the same dormancy
      signalling. mdBook green.
    Commit: `see Commit Log`

  - ID: `BOOK-USER-FRIENDLY-BACKFILL.2.c`
    Status: `done` (`2026-05-20`)
    Goal: rewrite the `R16-CAPTURE-FIDELITY-GATES` subsection.
    Acceptance: `Subsection meets standard; mdBook green.`
    Verification: `passed` — opens with the user's problem
      (how would we know if capture got better? and the
      missing structural barrier against silently lowering
      ungrounded contracts); one-sentence mental model;
      each of the six gates explained in plain language with
      what it checks and when it returns `NotEvaluated`;
      three-valued `FindingStatus` framed via why
      `NotEvaluated` is a real outcome (not Pass);
      `apply_fidelity_gates` producer walked through; the
      `Lowerable+Fail→Residual{"fidelity:<Gate>: <message>"}`
      routing rule called out explicitly; `FidelitySummary` +
      `score()` + `meets_threshold(1.0)` explained as
      "disciplined-honesty default"; corpus baseline
      `fail=0 score=1.000` read as live evidence of
      fidelity-honest contract producer; what
      `fidelity_failures (first 5)` looks like when
      `fail > 0`; four user-facing guarantees framed as
      benefits; Status delivered with the three-doctrines
      framing. mdBook green.
    Commit: `see Commit Log`

  - ID: `BOOK-USER-FRIENDLY-BACKFILL.2.d`
    Status: `done` (`2026-05-20`)
    Goal: rewrite the `R16-MULTIMODAL-CONTRACT-FUSION` subsection.
    Acceptance: `Subsection meets standard; mdBook green.`
    Verification: `passed` — opens with the user's problem
      (one AXI write-channel obligation in four places of the
      spec; before this tree, four disjoint contracts; two
      nasty failure modes: silent overwrite, lost
      contradiction); one-sentence mental model;
      `FusionKey` walked through with what each field
      contributes; agreement merge walked through
      field-by-field (provenance union, `Mixed` modality,
      delimited source_text, min confidence, `"fused:…"`
      id); disagreement routing framed as the **second
      structural honesty doctrine** (with FIDELITY.3 and
      CVE.3) — contradiction preserved in IR as first-class
      observation; `apply_fusion` runs before fidelity gates
      (ordering rationale called out); implementation notes
      on determinism + idempotence; corpus baseline
      `fusion: groups_merged=0 disagreements=0` as honest
      dormancy; four user-facing guarantees framed as
      benefits. mdBook green.
    Commit: `see Commit Log`

  - ID: `BOOK-USER-FRIENDLY-BACKFILL.2.e`
    Status: `pending`
    Goal: rewrite the `R16-WAVEFORM-CONTRACT-MINING` subsection
    (including the `.3.1 FigureRegion` sub-design).
    Acceptance: `Subsection meets standard; mdBook green.`
    Verification: `pending`
    Commit: `pending`

  - ID: `BOOK-USER-FRIENDLY-BACKFILL.2.f`
    Status: `pending`
    Goal: rewrite the `R16-CONSTRAINED-VERIFIED-EXTRACTION`
    subsection. The `R7-VALIDATION.5` template lives next door
    in `quality/validation.md` and shares conceptual ground
    (verification + residual routing) — keep the framing
    consistent.
    Acceptance: `Subsection meets standard; mdBook green.`
    Verification: `pending`
    Commit: `pending`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.3`
  Status: `pending`
  Goal: light upgrade of the two `reference/*` doctrine
  subsections (`AUDIT-DOC-RECONCILE` →
  `reference/documentation-scope.md`; `SIGNOFF-REMEDIATION` →
  `reference/live-docs.md`) — frame each doctrine as a user
  benefit, with concrete examples of what the doctrine prevents.
  Acceptance: `Both subsections framed in user terms; the doctrine each tree established is presented as a property the user can rely on; mdBook green.`
  Verification: `pending`
  Commit: `pending`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.4`
  Status: `pending`
  Goal: light upgrade of the `R7-VALIDATION` close-out summary in
  `quality/validation.md` so the per-leaf findings/metrics are
  framed as *what the user gets out of running validation today*,
  not a leaf-by-leaf changelog.
  Acceptance: `R7-VALIDATION close-out summary reads as a user guide to today's validation surface; mdBook green.`
  Verification: `pending`
  Commit: `pending`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.5`
  Status: `pending`
  Goal: light upgrade of `R15-GRAPH-DIRECTION-MIGRATION` in
  `domain/actor-connectivity.md` — frame "actor-relative graph
  as the source of truth for signal direction" as the user-facing
  property, not a migration story.
  Acceptance: `R15 subsection presents actor-relative graph semantics as a property the user benefits from; mdBook green.`
  Verification: `pending`
  Commit: `pending`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.6`
  Status: `pending`
  Goal: close the tree. Final verification: re-read every upgraded
  subsection cold; confirm a new user could understand the
  introduced concepts from the book alone; record this audit pass
  in the Verification Log; refresh the BOOK-METHOD-DOC Decisions
  entry with a cross-reference to this tree so the standard is
  fully self-referential.
  Acceptance: `Cold-read audit confirms each upgraded subsection is user-comprehensible; BOOK-METHOD-DOC Decisions cross-references this tree; tree marked done.`
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `BOOK-USER-FRIENDLY-BACKFILL.1` | `done` | Scope audit recorded |
| 2 | `BOOK-USER-FRIENDLY-BACKFILL.2` | `in-progress` | Honest-split into `.2.a`–`.2.f` (per-subsection) for reviewable diffs |
| 2.a | `BOOK-USER-FRIENDLY-BACKFILL.2.a` | `done` | `R16-CONTRACT-IR` rewritten user-friendly — `2026-05-20` |
| 2.b | `BOOK-USER-FRIENDLY-BACKFILL.2.b` | `done` | `R16-KG-PROTOCOL-ONTOLOGY` rewritten user-friendly — `2026-05-20` |
| 2.c | `BOOK-USER-FRIENDLY-BACKFILL.2.c` | `done` | `R16-CAPTURE-FIDELITY-GATES` rewritten user-friendly — `2026-05-20` |
| 2.d | `BOOK-USER-FRIENDLY-BACKFILL.2.d` | `done` | `R16-MULTIMODAL-CONTRACT-FUSION` rewritten user-friendly — `2026-05-20` |
| 2.e | `BOOK-USER-FRIENDLY-BACKFILL.2.e` | `pending` | **Next** — `R16-WAVEFORM-CONTRACT-MINING` rewrite |
| 2.f | `BOOK-USER-FRIENDLY-BACKFILL.2.f` | `pending` | `R16-CONSTRAINED-VERIFIED-EXTRACTION` rewrite |
| 3 | `BOOK-USER-FRIENDLY-BACKFILL.3` | `pending` | reference/* doctrine subsections |
| 4 | `BOOK-USER-FRIENDLY-BACKFILL.4` | `pending` | R7-VALIDATION close-out summary |
| 5 | `BOOK-USER-FRIENDLY-BACKFILL.5` | `pending` | R15 actor-relative-graph framing |
| 6 | `BOOK-USER-FRIENDLY-BACKFILL.6` | `pending` | Close tree |

## Dependencies / Order

- Depends on `BOOK-METHOD-DOC` (closed `2026-05-20`) for the
  standard the upgrades target, and on
  [[feedback-book-user-friendly]] for the user-friendly clarification.
- Independent of every other R0 / R6 / R7 / R15 / R16 tree —
  this is pure docs-quality work.

## Decisions

- `2026-05-20`: Created and immediately promoted `active`. The
  standard was established same day during the `R7-VALIDATION.5`
  book rewrite (the worked template). Spinning up this tree
  immediately is the right move because the standard now structurally
  applies via the BOOK-METHOD-DOC close-rule, and existing
  subsections that don't meet it are a real (per
  AUDIT-DOC-RECONCILE) doc-quality gap.
- `2026-05-20`: Non-Goal recorded — concise closure summaries
  (1-paragraph "tree X closed gap Y") that don't introduce new
  concepts are KEPT AS-IS. The standard is *user-friendly thorough*
  for concept-introducing subsections, not *more-words for
  more-words' sake*. Accurate brevity remains the right answer
  for the ISF/FSMGen/R6/PROVENANCE 1-paragraph summaries.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.1` | scope audit recorded; chapters classified; mdBook builds | `passed` (docs-only) |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.2.a` | `R16-CONTRACT-IR` subsection rewritten user-friendly (opens with the bound-obligation problem; one-sentence mental model; typed `ActorContract` walked through; closed `Obligation`/`EventExpr`/`Window` algebra in plain language; `Observe` honest-weak-fact framing; lowering pathway walked through; 4 user-facing guarantees framed as benefits; honest constraints on `(stage …)` dormancy recorded); mdBook green | `passed` |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.2.b` | `R16-KG-PROTOCOL-ONTOLOGY` subsection rewritten user-friendly (opens with the channels/phases/transactions-missing-from-IR problem; one-sentence mental model; `ProtocolGraph` walked through record-by-record with concrete protocol examples; `TickPhase ≠ ProtocolPhase` explained via the two questions they each answer; two population paths (mechanical projection vs PDF extraction) with trust levels; accessors covered; 3 user-facing guarantees framed as benefits; honest all-zero dormancy baseline signalled); mdBook green | `passed` |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.2.c` | `R16-CAPTURE-FIDELITY-GATES` subsection rewritten user-friendly (opens with "how would we know if capture got better?" + missing structural barrier against silent unground-lowering; one-sentence mental model; six gates explained in plain language; three-valued `FindingStatus` framed as why `NotEvaluated` ≠ `Pass`; `apply_fidelity_gates` + Lowerable+Fail→Residual{"fidelity:<Gate>: <message>"} routing rule called out; `FidelitySummary`/`score()`/`meets_threshold(1.0)` as disciplined-honesty default; corpus baseline `fail=0 score=1.000` as live evidence; `fidelity_failures (first 5)` shape shown; 4 user-facing guarantees framed as benefits; three-doctrines framing in Status); mdBook green | `passed` |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.2.d` | `R16-MULTIMODAL-CONTRACT-FUSION` subsection rewritten user-friendly (opens with the AXI write-channel obligation in 4 places of the spec; two nasty failure modes: silent overwrite + lost contradiction; one-sentence mental model; `FusionKey` field-by-field; agreement merge walked through with provenance union + Mixed + min confidence + "fused:…" id; disagreement routing as the second structural honesty doctrine; producer ordering before fidelity called out with rationale; determinism + idempotence notes; corpus baseline `groups_merged=0 disagreements=0` as honest dormancy; 4 user-facing guarantees framed as benefits); mdBook green | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `BOOK-USER-FRIENDLY-BACKFILL.1` | `BOOK-USER-FRIENDLY-BACKFILL.1 — create tree + scope audit + chapter classification` (`6f6839ec`) | docs-only; the new tree's own scaffolding |
| `BOOK-USER-FRIENDLY-BACKFILL.2.a` | `BOOK-USER-FRIENDLY-BACKFILL.2.a — rewrite R16-CONTRACT-IR book subsection user-friendly` (`3cc840ef`) | docs-only; per the standard; mdBook green |
| `BOOK-USER-FRIENDLY-BACKFILL.2.b` | `BOOK-USER-FRIENDLY-BACKFILL.2.b — rewrite R16-KG-PROTOCOL-ONTOLOGY book subsection user-friendly` (`40d8e2cb`) | docs-only; per the standard; mdBook green |
| `BOOK-USER-FRIENDLY-BACKFILL.2.c` | `BOOK-USER-FRIENDLY-BACKFILL.2.c — rewrite R16-CAPTURE-FIDELITY-GATES book subsection user-friendly` (`26cac946`) | docs-only; per the standard; mdBook green |
| `BOOK-USER-FRIENDLY-BACKFILL.2.d` | `BOOK-USER-FRIENDLY-BACKFILL.2.d — rewrite R16-MULTIMODAL-CONTRACT-FUSION book subsection user-friendly` | docs-only; per the standard; mdBook green |

## Changelog

- `2026-05-20`: Created and promoted `active` same day the
  user-friendly standard was established. `.1` scope audit
  recorded; frontier → `.2` (R16 family upgrade).
- `2026-05-20`: `.2.d` done — `R16-MULTIMODAL-CONTRACT-FUSION`
  subsection rewritten user-friendly (opens with the AXI
  write-channel obligation appearing in four places of the
  spec; two nasty failure modes: silent overwrite + lost
  contradiction; one-sentence mental model;
  `FusionKey` walked through with what each field
  contributes; agreement merge walked through
  field-by-field (provenance union, `Mixed` modality,
  delimited source_text, min confidence, `"fused:…"` id);
  disagreement routing framed as the **second structural
  honesty doctrine** (parallel to FIDELITY.3 and CVE.3);
  `apply_fusion` runs BEFORE fidelity gates — ordering
  rationale called out; implementation notes on determinism
  + idempotence; corpus baseline `fusion: groups_merged=0
  disagreements=0` as honest dormancy; four user-facing
  guarantees framed as benefits). Frontier → `.2.e`
  (R16-WAVEFORM-CONTRACT-MINING — most substantial; includes
  the `.3.1` FigureRegion sub-design).
- `2026-05-20`: `.2.c` done — `R16-CAPTURE-FIDELITY-GATES`
  subsection rewritten user-friendly (opens with "how would
  we know if capture got better?" + missing structural
  barrier against silent unground-lowering; one-sentence
  mental model; each of the six gates explained in plain
  language with what it checks and when it returns
  `NotEvaluated`; three-valued `FindingStatus` framed via
  why `NotEvaluated` is a real outcome not `Pass`;
  `apply_fidelity_gates` producer + the
  `Lowerable+Fail→Residual{"fidelity:<Gate>: <message>"}`
  routing rule called out explicitly; `FidelitySummary` +
  `score()` + `meets_threshold(1.0)` as "disciplined-honesty
  default"; corpus baseline `fail=0 score=1.000` as live
  evidence; `fidelity_failures (first 5)` shape shown; four
  user-facing guarantees framed as benefits; Status delivered
  with three-doctrines framing). Frontier → `.2.d`
  (R16-MULTIMODAL-CONTRACT-FUSION).
- `2026-05-20`: `.2.b` done — `R16-KG-PROTOCOL-ONTOLOGY`
  subsection rewritten user-friendly (opens with the
  channels/phases/transactions-missing-from-IR problem;
  one-sentence mental model; `ProtocolGraph` walked through
  record-by-record with concrete protocol examples (AXI
  AW/W/B; APB setup/access; TileLink A..E); the
  `TickPhase ≠ ProtocolPhase` distinction explained via the
  two questions they each answer; the two population paths
  (mechanical projection from contracts vs PDF extraction)
  cleanly separated with trust levels; accessors covered;
  what you see in the report today is the all-zero baseline
  as honest dormancy; three user-facing guarantees framed as
  benefits ("you can reason about protocol structure, not
  just signals"; "mechanical projection from contracts is
  lossless"; "today's pipeline is byte-identical"). Frontier
  → `.2.c` (R16-CAPTURE-FIDELITY-GATES).
- `2026-05-20`: `.2.a` done — `R16-CONTRACT-IR` subsection
  rewritten to the user-friendly standard (opens with the
  bound-obligation problem; one-sentence mental model; typed
  `ActorContract` walked through field-by-field in plain
  language; closed `Obligation` / `EventExpr` / `Window`
  algebra enumerated with plain-language explanations;
  `Observe` framed as the honest-weak-fact doctrine; `.isf`
  lowering pathway walked through; four user-facing
  guarantees framed as benefits; Status — delivered with
  honest constraints on `(stage …)` dormancy recorded).
  Honest-split `.2` into per-subsection sub-leaves
  (`.2.a`–`.2.f`) for reviewable diffs. Frontier → `.2.b`.
