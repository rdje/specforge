# BOOK-USER-FRIENDLY-BACKFILL: upgrade existing book subsections to the user-friendly standard

## Metadata

- Tree ID: `BOOK-USER-FRIENDLY-BACKFILL`
- Status: `done`
- Roadmap lane: `R0` (live-doc continuity / accuracy)
- Created: `2026-05-20`
- Last updated: `2026-05-20`
- Closed: `2026-05-20`
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
  Status: `done` (`2026-05-20`; all 6 R16-family subsections
  rewritten user-friendly via the per-subsection sub-leaves
  `.2.a`–`.2.f`)
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
    Status: `done` (`2026-05-20`)
    Goal: rewrite the `R16-WAVEFORM-CONTRACT-MINING` subsection
    (including the `.3.1 FigureRegion` sub-design).
    Acceptance: `Subsection meets standard; mdBook green.`
    Verification: `passed` — opens with the user's problem
      (timing diagrams = the densest temporal-intent source;
      VLM extractor was defensive); one-sentence mental model
      (PartialTrace + generalizer + round-trip verifier =
      fabrication-prevented mining); typed `PartialTrace`
      walked through with what each evidence kind contributes
      to extractor-observability; four conservative
      generalization rules with explicit honesty defaults
      (bare LaneEdge → Observe+Residual; cap on single-source
      confidence); round-trip verifier framed as **third
      structural-honesty enforcement** (with FIDELITY.3 and
      FUSION.3 — four-doctrine framing); cross-check
      delegated to FUSION explained; `.3.1` FigureRegion
      input contract design walked through (corpus survey
      finding; VisualAsset extension; typed
      `FigureAnnotation` enum); `.3.2` adapter mapping
      walked through (lane→edges+spans; Unknown breaks runs
      without recording edges; Unknown annotation demotes
      confidence); corpus baseline `waveform: figure_contracts=0
      verifier_fail_residuals=0` as honest dormancy;
      **negative-fixture coverage** explicitly walked through
      as the load-bearing safety claim with three named unit
      tests; four user-facing guarantees framed as benefits;
      Status delivered leaf-by-leaf with honest-split
      framing. mdBook green.
    Commit: `see Commit Log`

  - ID: `BOOK-USER-FRIENDLY-BACKFILL.2.f`
    Status: `done` (`2026-05-20`)
    Goal: rewrite the `R16-CONSTRAINED-VERIFIED-EXTRACTION`
    subsection (closes the R16 family in the book).
    Acceptance: `Subsection meets standard; mdBook green.`
    Verification: `passed` — opens with the user's problem
      (LLM/VLM extractor producing JSON that *looks* like
      ActorContract; three concrete failure modes: malformed
      shape, claimed-but-not-licensed signal, heuristic
      template match); one-sentence mental model;
      `parse_constrained_contract` walked through with
      serde-is-the-validator + drift-lock + provider-agnostic
      properties; `entailment_check` walked through with the
      conservative initial impl + the **fourth structural
      honesty doctrine** routing rule that completes the
      four-doctrine framing (along with FIDELITY.3, FUSION.3,
      and WAVEFORM.3's verifier); template library walked
      through with match-grounding rule + honestly-residual
      lowering for CFC + SetupAccess (extending the
      WAVEFORM.2 bare-edge doctrine pattern);
      `voi_score`/`select_top_n_by_voi` walked through with
      reproducibility framing; corpus baseline
      `constrained: schema_rejects=0 entailment_fails=0
      template_hits=0` as honest dormancy; four user-facing
      guarantees framed as benefits; **Status closes with R16
      PROGRAM COMPLETE recap** — all six trees enumerated
      with their one-sentence summaries, four-doctrine table
      (with the tree each enforcement landed in), structural
      fabrication-prevention end-to-end framing. mdBook green.
    Commit: `see Commit Log`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.3`
  Status: `done` (`2026-05-20`)
  Goal: light upgrade of the two `reference/*` doctrine
  subsections (`AUDIT-DOC-RECONCILE` →
  `reference/documentation-scope.md`; `SIGNOFF-REMEDIATION` →
  `reference/live-docs.md`) — frame each doctrine as a user
  benefit, with concrete examples of what the doctrine prevents.
  Acceptance: `Both subsections framed in user terms; the doctrine each tree established is presented as a property the user can rely on; mdBook green.`
  Verification: `passed` — both subsections rewritten with
    the same user-friendly structure: user-facing guarantee
    up top (in blockquote); why-this-isn't-free section
    with a concrete drift-or-bar-slip example; what-this-
    tree-fixed concretely; how-the-doctrine-is-enforced-now
    section explaining the structural enforcement;
    what-this-buys-you-as-a-SpecForge-user section framing
    the doctrine as a property the user can rely on. mdBook
    green.
  Commit: `see Commit Log`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.4`
  Status: `done` (`2026-05-20`)
  Goal: light upgrade of the `R7-VALIDATION` close-out summary in
  `quality/validation.md` so the per-leaf findings/metrics are
  framed as *what the user gets out of running validation today*,
  not a leaf-by-leaf changelog.
  Acceptance: `R7-VALIDATION close-out summary reads as a user guide to today's validation surface; mdBook green.`
  Verification: `passed` — rewrote the subsection as a
    user-facing guide to what `specforge validate` produces:
    user-facing guarantee in blockquote (typed report with
    findings + metrics + additive-empty-by-design
    `applied_mutations`); explanation of Info/Warning/Error
    severities + the read-only contract; the four delivered
    surfaces (temporal handshake completion gap, temporal
    multi-predicate antecedent flag, KG-quality benchmark
    findings with the conservative 50% floor, adapter
    validation auto-detection) each framed in terms of what
    the user sees and what it means; four user-facing
    benefits framed as properties; signoff discipline
    pointer. mdBook green.
  Commit: `see Commit Log`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.5`
  Status: `done` (`2026-05-20`)
  Goal: light upgrade of `R15-GRAPH-DIRECTION-MIGRATION` in
  `domain/actor-connectivity.md` — frame "actor-relative graph
  as the source of truth for signal direction" as the user-facing
  property, not a migration story.
  Acceptance: `R15 subsection presents actor-relative graph semantics as a property the user benefits from; mdBook green.`
  Verification: `passed` — opens with the tautology
    framing ("if you ask SpecForge whether a signal is an
    input or an output, there's exactly one correct way to
    find out: ask the actor-relative graph from the
    perspective of the actor you care about"); user-facing
    guarantee in blockquote; **why-this-isn't-trivial section
    explains protocol-direction-is-inherently-perspective-
    relative** — same wire = output for manager + input for
    subordinate; flat direction_hint silently drifts when
    upstream + downstream use different conventions;
    actor-relative graph fixes this by making the perspective
    explicit; what-this-tree-did concretely (adapter +
    validation + semantic migrated); compatibility surface
    kept; three user-facing benefits framed as properties
    (no perspective bugs, read-IR-by-perspective, external
    tooling can still consume direction_hint). mdBook green.
  Commit: `see Commit Log`

- ID: `BOOK-USER-FRIENDLY-BACKFILL.6`
  Status: `done` (`2026-05-20`)
  Goal: close the tree. Final verification: re-read every upgraded
  subsection cold; confirm a new user could understand the
  introduced concepts from the book alone; record this audit pass
  in the Verification Log; refresh the BOOK-METHOD-DOC Decisions
  entry with a cross-reference to this tree so the standard is
  fully self-referential.
  Acceptance: `Cold-read audit confirms each upgraded subsection is user-comprehensible; BOOK-METHOD-DOC Decisions cross-references this tree; tree marked done.`
  Verification: `passed` — every upgraded subsection
    (`.2.a`–`.2.f` R16-family in
    `direction/temporal-intent-capture.md`; `.3`
    AUDIT-DOC-RECONCILE in
    `reference/documentation-scope.md`; `.3`
    SIGNOFF-REMEDIATION in `reference/live-docs.md`; `.4`
    R7-VALIDATION close-out in `quality/validation.md`;
    `.5` R15 in `domain/actor-connectivity.md`) applies the
    same structural pattern: user-facing guarantee in
    blockquote up top → why this isn't free / trivial /
    a-given (with a concrete failure-mode example) →
    what-this-tree-fixed concretely → how-the-doctrine-is-
    enforced-now (when there is a doctrine) →
    what-this-buys-the-SpecForge-user. The R7-VALIDATION.5
    book subsection in `docs/book/src/quality/validation.md`
    is the canonical worked template (rewritten earlier
    today); the BOOK-METHOD-DOC Decisions entry is updated
    to cross-reference this tree as the worked backfill that
    proved the standard across the existing surface. Honest
    Non-Goal recorded: concise 1-paragraph closure summaries
    (ISF/FSMGen 7 trees + R6 hardening 6 trees +
    PROVENANCE-HARDENING) are kept as-is — accurate brevity
    is the right answer for non-concept-introducing
    subsections; more-words for more-words' sake is not
    the standard. mdBook green throughout.
  Commit: `see Commit Log`

## Current Frontier

**Tree closed `2026-05-20`.** All six leaves done; ten upgraded
subsections across five book chapters meet the user-friendly
standard; BOOK-METHOD-DOC Decisions cross-references this tree as
the worked backfill that proved the standard across the existing
surface. The standard now applies to every future close-leaf
structurally (per the BOOK-METHOD-DOC close-rule encoded in
`docs/TASK_TREE.md` Completion Rules + `COMMIT.md` Required
Commit Workflow).

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `BOOK-USER-FRIENDLY-BACKFILL.1` | `done` | Scope audit recorded |
| 2 | `BOOK-USER-FRIENDLY-BACKFILL.2` | `done` | All 6 R16-family subsections rewritten user-friendly — `2026-05-20` |
| 2.a | `BOOK-USER-FRIENDLY-BACKFILL.2.a` | `done` | `R16-CONTRACT-IR` rewritten user-friendly — `2026-05-20` |
| 2.b | `BOOK-USER-FRIENDLY-BACKFILL.2.b` | `done` | `R16-KG-PROTOCOL-ONTOLOGY` rewritten user-friendly — `2026-05-20` |
| 2.c | `BOOK-USER-FRIENDLY-BACKFILL.2.c` | `done` | `R16-CAPTURE-FIDELITY-GATES` rewritten user-friendly — `2026-05-20` |
| 2.d | `BOOK-USER-FRIENDLY-BACKFILL.2.d` | `done` | `R16-MULTIMODAL-CONTRACT-FUSION` rewritten user-friendly — `2026-05-20` |
| 2.e | `BOOK-USER-FRIENDLY-BACKFILL.2.e` | `done` | `R16-WAVEFORM-CONTRACT-MINING` rewritten user-friendly — `2026-05-20` |
| 2.f | `BOOK-USER-FRIENDLY-BACKFILL.2.f` | `done` | `R16-CONSTRAINED-VERIFIED-EXTRACTION` rewritten user-friendly — `2026-05-20` |
| 3 | `BOOK-USER-FRIENDLY-BACKFILL.3` | `done` | reference/* doctrines rewritten user-friendly — `2026-05-20` |
| 4 | `BOOK-USER-FRIENDLY-BACKFILL.4` | `done` | R7-VALIDATION close-out summary upgraded — `2026-05-20` |
| 5 | `BOOK-USER-FRIENDLY-BACKFILL.5` | `done` | R15 actor-relative-graph framing rewritten — `2026-05-20` |
| 6 | `BOOK-USER-FRIENDLY-BACKFILL.6` | `done` | Tree closed `2026-05-20`; BOOK-METHOD-DOC Decisions cross-referenced |

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
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.2.e` | `R16-WAVEFORM-CONTRACT-MINING` subsection rewritten user-friendly (opens with timing diagrams as densest temporal-intent source + defensive VLM extractor problem; one-sentence mental model PartialTrace + generalizer + round-trip verifier = fabrication-prevented mining; typed `PartialTrace` walked through with evidence-kind structure; four conservative generalization rules with explicit honesty defaults; **round-trip verifier framed as third structural-honesty enforcement** alongside FIDELITY.3 + FUSION.3 + CVE.3 = four-doctrine framing; cross-check delegated to FUSION; `.3.1` FigureRegion input contract design walked through with corpus-survey finding; `.3.2` adapter mapping walked through; corpus baseline 0/0 as honest dormancy; negative-fixture coverage as the load-bearing safety claim with three named unit tests; 4 user-facing guarantees framed as benefits); mdBook green | `passed` |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.2.f` | `R16-CONSTRAINED-VERIFIED-EXTRACTION` subsection rewritten user-friendly (opens with the LLM/VLM extractor producing JSON that *looks* like ActorContract; three concrete failure modes; one-sentence mental model; `parse_constrained_contract` with serde-is-validator + drift-lock + provider-agnostic properties; `entailment_check` + the **fourth structural honesty doctrine** completing four-doctrine framing (FIDELITY.3 + FUSION.3 + WAVEFORM.3 + CVE.3); template library with match-grounding + honestly-residual lowering for CFC/SetupAccess; `voi_score`/`select_top_n_by_voi` with reproducibility framing; corpus baseline 0/0/0 as honest dormancy; 4 user-facing guarantees; **Status closes with R16 PROGRAM COMPLETE recap** + four-doctrine table); mdBook green | `passed` |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.2` | All 6 R16-family subsections rewritten user-friendly via per-subsection sub-leaves `.2.a`–`.2.f`; per-tree task-tree files unchanged; mdBook green | `passed` |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.3` | AUDIT-DOC-RECONCILE + SIGNOFF-REMEDIATION subsections rewritten with consistent user-friendly structure: user-facing guarantee in blockquote up top; why-this-isn't-free with concrete drift-or-bar-slip example; what-this-tree-fixed concretely; how-the-doctrine-is-enforced-now (structural enforcement via Completion Rules + Required Commit Workflow); what-this-buys-the-SpecForge-user; mdBook green | `passed` |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.4` | R7-VALIDATION close-out summary rewritten as a user-facing guide ("what `specforge validate` does for you today") rather than a leaf-by-leaf changelog; user-facing guarantee in blockquote; severity meanings + read-only contract; four delivered surfaces (handshake gap, multi-predicate antecedent, KG-quality benchmarks with 50% floor, adapter auto-detection) each framed in terms of what the user sees + what it means; four benefits framed as properties; signoff discipline pointer; mdBook green | `passed` |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.5` | R15-GRAPH-DIRECTION-MIGRATION subsection rewritten: opens with the tautology framing ("ask the actor-relative graph from the perspective of the actor you care about"); user-facing guarantee in blockquote; **why-this-isn't-trivial section explains protocol-direction = perspective-relative** (manager-output = subordinate-input); flat direction_hint drift example; actor-relative-graph fix; what-this-tree-did concretely (adapter + validation + semantic migrated); compatibility surface kept; 3 user-facing benefits as properties (no perspective bugs, read-IR-by-perspective, external direction_hint consumers still work); mdBook green | `passed` |
| `2026-05-20` | `BOOK-USER-FRIENDLY-BACKFILL.6` | tree closed; cross-referenced this tree on the BOOK-METHOD-DOC Decisions entry so the standard is fully self-referential (the worked template + the backfill instance both recorded); honest Non-Goal preserved (concise 1-paragraph closure summaries kept as-is); mdBook green | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `BOOK-USER-FRIENDLY-BACKFILL.1` | `BOOK-USER-FRIENDLY-BACKFILL.1 — create tree + scope audit + chapter classification` (`6f6839ec`) | docs-only; the new tree's own scaffolding |
| `BOOK-USER-FRIENDLY-BACKFILL.2.a` | `BOOK-USER-FRIENDLY-BACKFILL.2.a — rewrite R16-CONTRACT-IR book subsection user-friendly` (`3cc840ef`) | docs-only; per the standard; mdBook green |
| `BOOK-USER-FRIENDLY-BACKFILL.2.b` | `BOOK-USER-FRIENDLY-BACKFILL.2.b — rewrite R16-KG-PROTOCOL-ONTOLOGY book subsection user-friendly` (`40d8e2cb`) | docs-only; per the standard; mdBook green |
| `BOOK-USER-FRIENDLY-BACKFILL.2.c` | `BOOK-USER-FRIENDLY-BACKFILL.2.c — rewrite R16-CAPTURE-FIDELITY-GATES book subsection user-friendly` (`26cac946`) | docs-only; per the standard; mdBook green |
| `BOOK-USER-FRIENDLY-BACKFILL.2.d` | `BOOK-USER-FRIENDLY-BACKFILL.2.d — rewrite R16-MULTIMODAL-CONTRACT-FUSION book subsection user-friendly` (`db9d8fe6`) | docs-only; per the standard; mdBook green |
| `BOOK-USER-FRIENDLY-BACKFILL.2.e` | `BOOK-USER-FRIENDLY-BACKFILL.2.e — rewrite R16-WAVEFORM-CONTRACT-MINING book subsection user-friendly (incl. .3.1 FigureRegion)` (`c75ac736`) | docs-only; per the standard; mdBook green |
| `BOOK-USER-FRIENDLY-BACKFILL.2.f` | `BOOK-USER-FRIENDLY-BACKFILL.2.f — rewrite R16-CONSTRAINED-VERIFIED-EXTRACTION book subsection user-friendly (closes BUF.2; R16 PROGRAM COMPLETE recap)` (`e36a00c3`) | docs-only; per the standard; mdBook green; **closes the R16 family book backfill** |
| `BOOK-USER-FRIENDLY-BACKFILL.3` | `BOOK-USER-FRIENDLY-BACKFILL.3 — rewrite reference/* doctrine subsections (AUDIT-DOC-RECONCILE + SIGNOFF-REMEDIATION) user-friendly` (`96bef553`) | docs-only; per the standard; mdBook green |
| `BOOK-USER-FRIENDLY-BACKFILL.4` | `BOOK-USER-FRIENDLY-BACKFILL.4 — rewrite R7-VALIDATION close-out summary as user-facing validate guide` (`e0638d30`) | docs-only; per the standard; mdBook green |
| `BOOK-USER-FRIENDLY-BACKFILL.5` | `BOOK-USER-FRIENDLY-BACKFILL.5 — rewrite R15-GRAPH-DIRECTION-MIGRATION as actor-relative-graph user-facing property` (`56c03ce7`) | docs-only; per the standard; mdBook green; perspective-relativity rationale explained |
| `BOOK-USER-FRIENDLY-BACKFILL.6` | `BOOK-USER-FRIENDLY-BACKFILL.6 — close tree + cross-reference BOOK-METHOD-DOC Decisions` | docs-only; tree closed; standard now self-referential (worked template + backfill instance) |

## Changelog

- `2026-05-20`: Created and promoted `active` same day the
  user-friendly standard was established. `.1` scope audit
  recorded; frontier → `.2` (R16 family upgrade).
- `2026-05-20`: **Tree CLOSED.** `.6` done — cross-referenced
  this tree on the BOOK-METHOD-DOC Decisions entry so the
  user-friendly standard is now **self-referential**: the
  worked template (`R7-VALIDATION.5` book subsection in
  `docs/book/src/quality/validation.md`) AND the backfill
  instance (this tree's per-subsection upgrades of every
  concept-introducing pre-2026-05-20 subsection) are both
  recorded as the canonical examples. The honest Non-Goal
  is preserved: concise 1-paragraph closure summaries
  (ISF/FSMGen 7 trees + R6 hardening 6 trees +
  PROVENANCE-HARDENING) stay as-is — accurate brevity is
  the right answer for non-concept-introducing subsections.
  Final scoreboard: **10 concept-introducing subsections
  upgraded across 5 book chapters** (R16 family ×6 in
  `direction/temporal-intent-capture.md`;
  AUDIT-DOC-RECONCILE in `reference/documentation-scope.md`;
  SIGNOFF-REMEDIATION in `reference/live-docs.md`;
  R7-VALIDATION close-out in `quality/validation.md`;
  R15-GRAPH-DIRECTION-MIGRATION in
  `domain/actor-connectivity.md`). mdBook green throughout
  every leaf.
- `2026-05-20`: `.5` done — `R15-GRAPH-DIRECTION-MIGRATION`
  subsection in `docs/book/src/domain/actor-connectivity.md`
  rewritten as an actor-relative-graph user-facing property
  (not a migration story). Opens with the tautology framing
  *"if you ask SpecForge whether a signal is an input or an
  output, there's exactly one correct way to find out: ask
  the actor-relative graph from the perspective of the
  actor you care about"*; user-facing guarantee in
  blockquote; **why-this-isn't-trivial section explains
  that protocol direction is inherently perspective-
  relative** (same wire = output for manager + input for
  subordinate), the flat-direction_hint drift mechanism
  (some upstream stage sets it from one perspective; some
  downstream consumer reads it from another; silent
  reversal for a subset of signals), and the
  actor-relative-graph fix (perspective made explicit per
  edge); what-this-tree-did concretely (adapter +
  validation + semantic stages migrated to graph lookup);
  compatibility surface kept (direction_hint stays for
  external consumers); three user-facing benefits framed
  as properties (no perspective bugs; you can read the IR
  by perspective; external tooling can still consume
  direction_hint). Frontier → `.6` (close tree: cold-read
  audit + cross-reference BOOK-METHOD-DOC Decisions entry
  for self-referential standard).
- `2026-05-20`: `.4` done — `R7-VALIDATION` close-out
  summary in `docs/book/src/quality/validation.md` rewritten
  as a user-facing guide ("what `specforge validate` does
  for you today") rather than a leaf-by-leaf changelog.
  Pattern: user-facing guarantee in blockquote (typed report
  with findings + metrics + additive-empty-by-design
  `applied_mutations`); Info/Warning/Error severity
  meanings + the read-only-by-default contract; the four
  delivered surfaces (temporal handshake completion gap,
  temporal multi-predicate antecedent flag, KG-quality
  benchmark findings with the conservative 50% floor,
  adapter validation auto-detection) each framed in terms
  of what the user sees and what it means; four
  user-facing benefits framed as properties; pointer back
  to the SIGNOFF-REMEDIATION doctrine. Frontier → `.5`
  (R15 actor-graph framing).
- `2026-05-20`: `.3` done — both `reference/*` doctrine
  subsections rewritten with the same user-friendly
  structure:
  - **`AUDIT-DOC-RECONCILE`** in
    `docs/book/src/reference/documentation-scope.md` —
    user-facing guarantee in blockquote ("every claim in the
    book / ROADMAP / README either describes what the code
    does, or it's recorded as wrong and gets fixed"); why
    this isn't free (with a concrete doc-drift example);
    what this tree fixed (ROADMAP R15 + stale HDL refs); how
    the doctrine is enforced now (via BOOK-METHOD-DOC
    close-rule + run_docs_ci.sh); what it buys the SpecForge
    user (you can read the book and trust it).
  - **`SIGNOFF-REMEDIATION`** in
    `docs/book/src/reference/live-docs.md` — user-facing
    guarantee in blockquote ("`scripts/run_ci.sh` on `main`
    is always green; if you branch, your starting point is
    clean"); why this isn't free (three concrete failure
    modes when CI bar slips); what this tree fixed
    (25-clippy-errors restoration with idiomatic fixes only +
    narrow-recorded `#[allow]`s where genuinely
    inapplicable + zero production behaviour change); how
    the doctrine is enforced now (via COMMIT.md Required
    Commit Workflow); what it buys the SpecForge user
    (clean starting point, every leaf shipped under a green
    gate, idiomatic clippy precedent).
  Both subsections share the structural pattern: user-facing
  guarantee → why-not-free → what-was-fixed → how-it's-
  enforced-now → what-this-buys-you. Frontier → `.4`
  (R7-VALIDATION close-out summary in `quality/validation.md`
  — frame the per-leaf findings/metrics as what the user
  gets out of running validation today).
- `2026-05-20`: **`.2` (parent) DONE — all 6 R16-family
  subsections rewritten user-friendly via the per-subsection
  sub-leaves.** `.2.f` done — `R16-CONSTRAINED-VERIFIED-EXTRACTION`
  subsection rewritten user-friendly (opens with the LLM/VLM
  extractor producing JSON that *looks* like ActorContract;
  three concrete failure modes spelled out: malformed shape,
  claimed-but-not-licensed signal, heuristic template match;
  one-sentence mental model; `parse_constrained_contract`
  walked through with serde-is-the-validator +
  obligation-discriminator drift-lock + provider-agnostic
  properties; `entailment_check` walked through with the
  conservative initial impl + apply_entailment_to_contract +
  **the fourth structural honesty doctrine** routing rule
  that completes the four-doctrine framing alongside
  FIDELITY.3 + FUSION.3 + WAVEFORM.3; template library walked
  through with match-grounding rule + honestly-residual
  lowering for CFC + SetupAccess (the doctrine pattern from
  WAVEFORM.2's bare-edge case extending naturally);
  `voi_score` + `select_top_n_by_voi` walked through with
  reproducibility framing for next-pass selection; corpus
  baseline `constrained: schema_rejects=0 entailment_fails=0
  template_hits=0` as honest dormancy; four user-facing
  guarantees framed as benefits; **Status closes with R16
  PROGRAM COMPLETE recap** — all six trees enumerated with
  one-sentence summaries, the four-doctrine table mapping
  each enforcement to its source tree, structural-
  fabrication-prevention-end-to-end framing). Frontier →
  `.3` (reference/* doctrine subsections —
  AUDIT-DOC-RECONCILE in `reference/documentation-scope.md`
  + SIGNOFF-REMEDIATION in `reference/live-docs.md`).
- `2026-05-20`: `.2.e` done — `R16-WAVEFORM-CONTRACT-MINING`
  subsection rewritten user-friendly (opens with timing
  diagrams = densest temporal-intent source + the defensive
  VLM extractor problem; one-sentence mental model walks the
  reader through the PartialTrace + generalizer + round-trip
  verifier triple as "fabrication-prevented mining"; typed
  `PartialTrace` walked through with the rationale for the
  per-evidence-kind structure (extractor reliability is
  observable per dimension); four conservative generalization
  rules explained with explicit honesty defaults (bare edge →
  Observe+Residual; under-determined delay → Residual;
  single-source confidence capped at Medium); round-trip
  verifier framed as the **third structural-honesty
  enforcement** alongside FIDELITY.3 + FUSION.3 (and
  CVE.3 as the fourth) — the four-doctrine framing makes
  fabrication mechanically impossible end-to-end; cross-check
  delegation to FUSION; `.3.1` FigureRegion input contract
  walked through with corpus-survey finding (no raw
  PDFs/SVGs in tree); `.3.2` adapter mapping walked through
  field-by-field with explicit honest-dormancy notes
  (Unknown breaks runs without recording edges; Unknown
  annotation demotes confidence); corpus baseline `waveform:
  figure_contracts=0 verifier_fail_residuals=0` as honest
  dormancy; negative-fixture coverage explicitly walked
  through as the load-bearing safety claim with three named
  unit tests; four user-facing guarantees framed as benefits;
  Status delivered leaf-by-leaf with honest-split framing).
  Frontier → `.2.f` (R16-CONSTRAINED-VERIFIED-EXTRACTION —
  the closing R16 subsection; mirrors the R7-VALIDATION.5
  template).
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
