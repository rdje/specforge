# R15C-R15G-LEARNING-PLANE-BACKFILL: own + audit the learning / eval / corpus lanes (in-progress)

## Metadata

- Tree ID: `R15C-R15G-LEARNING-PLANE-BACKFILL`
- Status: `active`
- Roadmap lane: `R15c-R15g`
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Backfill task-tree ownership + a meticulous audit for ROADMAP milestones
**R15c–R15g** — the convergent-learning / evaluation / corpus lanes that are
**In Progress** (not yet complete). Driven by `ROADMAP-TASKTREE-COVERAGE.4`.
Each milestone is an owned leaf recording (a) the **delivered-so-far** surface
audited against the code, (b) its **remaining scope** (open completion
criteria), and (c) book coverage. Because these lanes are open, this tree stays
`active` and its leaves stay `in_progress` — closing them requires the ROADMAP
completion criteria to be met, which is future work, each under this tree (or a
dedicated child tree) when pursued.

## Non-Goals

- No code change — audit + ownership only; any *advance* of an open completion
  criterion becomes its own normal task-tree (no code change without one).
- NOT duplicating the ROADMAP's long `done:`/`remaining:` lists — link to the
  §section and record the audited delivering surface + the open-scope summary.

## Acceptance Criteria

- R15c–R15g each owned by a leaf with: audited delivered surface (symbols/
  commands/fixtures verified present), remaining-scope summary, book coverage;
  status `in_progress` (lanes are open). Registered in `docs/TASK_TREE.md`.
- The umbrella leaf `ROADMAP-TASKTREE-COVERAGE.4` (ownership+audit) closes; this
  tree remains `active` tracking the open lanes.

## Task Tree

- ID: `R15C-R15G-LEARNING-PLANE-BACKFILL`
  Status: `active`
  Goal: own + audit R15c–R15g (in-progress lanes)
  Children: `.1`, `.2`, `.3`, `.4`, `.5`

- ID: `R15C-R15G-LEARNING-PLANE-BACKFILL.1`
  Status: `in_progress`
  Goal: own + audit **R15c — KG-guided multimodal rescans** (ROADMAP §R15c)
  Verification: >
    AUDIT passed (`2026-05-31`) — DELIVERED surface present: `EvidenceIR`
    `signal_semantic_hints: Vec<SignalSemanticHintRecord>` (mined from table
    `SignalDescription`, prose `SourceFact`, alias-grounded prose, grounded
    visual captions + VLM timing annotations; explicit-id stripping +
    clause-local per-signal windows before role inference); `SemanticIR`/
    `IntentIR` carry `semantic_tags`, `semantic_observations`,
    `semantic_candidates`, `semantic_arbitration`, `resolved_semantic_role`,
    `semantic_grounding_strength`, `semantic_consensus`, + `alias_dependent`
    marking; handshake-completion derivation consults semantic tags before
    literal-name fallback (contested arbitration blocks fallback; fallback-only
    roles surface as `semantic_resolved_role_without_consensus` residual +
    IntentIR assumption); `nlp-enrich` refreshes hints on alias/backannotation
    change; `validate` reports decisive-vs-contested arbitration + alias-
    dependent handshake completion. (All symbols in `ir/evidence.rs` +
    `ir/semantic.rs` + `ir/intent.rs`.) Book: `domain/actor-connectivity.md`.
    REMAINING (open completion criteria): make anchored rescans over
    tables/prose/figures a *first-class convergent-loop workstream* with
    new-fact-vs-duplicate convergence accounting; recover more weakly-labeled
    detail tables / polarity / timing / value facts from known anchors. Status
    `in_progress` (lane open). Each advance → its own owning tree.
    **Advances landed:** `R15C-CONVERGENCE-REPORT` (`2026-05-31`, CLOSED) —
    delivered the *convergence-accounting* half: the loop now emits a typed
    `EvidenceConvergenceReport` (passes, genuinely-new deduped facts/pass,
    converged-vs-capped), persisted + surfaced by `validate`. Still open: the
    *accuracy* half — anchored prose/figure rescans that recover MORE facts,
    measured against that now-visible metric.
  Commit: `see Commit Log`

- ID: `R15C-R15G-LEARNING-PLANE-BACKFILL.2`
  Status: `in_progress`
  Goal: own + audit **R15d — Evidence arbitration & cross-modality conflict resolution** (ROADMAP §R15d)
  Verification: >
    AUDIT passed (`2026-05-31`) — DELIVERED surface present: typed conflict
    records persisted + carried + validation-reported across EvidenceIR →
    SemanticIR → IntentIR — `signal_polarity_conflicts:
    Vec<SignalPolarityConflictRecord>` (prose vs signal-table active-level
    disagreement; clause-local polarity parsing beyond reset),
    `signal_semantic_conflicts` (incompatible role evidence),
    `interface_signal_conflicts` (direction/width disagreement, sticky collapse),
    `signal_connectivity_conflicts` (multiple structural producers). `validate`
    reports + flags each at the relevant stage(s); polarity-only co-mentions are
    treated as record enrichment (no duplicated canonical records). (Symbols in
    `ir/evidence.rs` + `ir/semantic.rs` + `ir/intent.rs`.) Book:
    `quality/validation.md` + `domain/actor-connectivity.md`. REMAINING (open):
    broaden the typed arbitration surface to *all* unresolved multimodal
    disagreements incl. contradictory direction/timing/value across full
    APB/AHB/AXI corpora; the AI-hypothesis-must-earn-promotion rule is in force
    but completeness across modalities is ongoing. Status `in_progress`.
  Commit: `see Commit Log`

- ID: `R15C-R15G-LEARNING-PLANE-BACKFILL.3`
  Status: `in_progress`
  Goal: own + audit **R15e — KG-quality evaluation & benchmark hardening** (ROADMAP §R15e)
  Verification: >
    AUDIT passed (`2026-05-31`) — DELIVERED surface present: `specforge
    kg-bench` command (`commands/kg_bench.rs`) runs tracked fixtures through
    SourceIR→EvidenceIR→SemanticIR→IntentIR; fixture pack under
    `crates/specforge/test_data/kg_quality/` (**153** fixture dirs audited) —
    gold + negative + residual-quality + stage-patched fixtures spanning
    actor-port recovery, name-only-role rejection, multi-producer conflict
    surfacing, contested/alias-dependent handshake, cross-modality
    grounding/conflict, VLM timing-note role discipline, active-low equivalence,
    and AMBA/APB/AHB/AXI source-column / requester-completer / stability /
    width-only / timing gold fixtures. Book: `quality/kg-bench.md`. REMAINING
    (open): keep curating gold/negative fixtures so roadmap progress is driven
    by accuracy + false-positive control (not one scalar); deepen explicit
    residual-quality / conflict-surfacing / bounded-hypothesis-rejection checks.
    Status `in_progress` (an evergreen evaluation lane).
  Commit: `see Commit Log`

- ID: `R15C-R15G-LEARNING-PLANE-BACKFILL.4`
  Status: `in_progress`
  Goal: own + audit **R15f — Cross-document extraction learning plane** (ROADMAP §R15f)
  Verification: >
    AUDIT passed (`2026-05-31`) — DELIVERED surface present: typed `CorpusMemory`
    store (`ir/prior_memory.rs`); `specforge learn-priors <intent_ir>...`
    (`commands/learn_priors.rs`) builds `generated/prior_memory/corpus_memory.json`
    under a conservative, validated-IntentIR-only, advisory-only update policy.
    Harvested prior families: actor-taxonomy, semantic-role phrase, semantic
    modality-reliability, temporal-language phrase, table-shape (live 4-doc AMBA
    run: 16/5/4/266/99). FOUR+ bounded prior-consumption paths, each provenance-
    safe (cannot author canonical facts without fresh local grounding):
    `evidence`/`converge` actor-taxonomy + semantic-phrase consumers; SemanticIR
    temporal-window + semantic-modality-reliability consumers; EvidenceIR
    table-kind consumer (only when local kind is `unknown`). `kg-bench` stages
    fixture-owned `CorpusMemory` proving prior-guided recovery improves an unseen
    local phrase without cross-document fact leakage. Book:
    `quality/corpus-memory.md`. REMAINING (per ROADMAP `remaining:`): grow
    protocol-family scoping beyond AMBA inference w/o brittle logic; deepen
    negative-knowledge rescan selection; add new typed prior families only with
    full schema+harvester+consumer+guard coverage. Status `in_progress`.
  Commit: `see Commit Log`

- ID: `R15C-R15G-LEARNING-PLANE-BACKFILL.5`
  Status: `in_progress`
  Goal: own + audit **R15g — Corpus knowledge base plane** (ROADMAP §R15g)
  Verification: >
    AUDIT passed (`2026-05-31`) — DELIVERED surface present: tracked `corpus_kb/`
    root (**11** page families audited — incl. `protocols/`, `patterns/`,
    `tables/`, `visuals/`, `timing/`, `infra/`, `failures/`, plus
    `prior_candidates/`/`prior_memory/`/`benchmarks/`/`state_machines/`) with
    `README.md` + `SCHEMA.md`; `specforge corpus-kb <validation-report>...`
    (`commands/corpus_kb.rs`) refreshes the managed validation-finding
    projection in `corpus_kb/failures/validation-findings.md`, preserving
    human-authored synthesis outside the generated block and citing report
    sidecars by document key/path/fingerprint/finding-id. Book:
    `quality/corpus-kb.md`. REMAINING (open completion criteria): make ≥1 more
    page family auto-refreshable from validation/benchmark results without
    overwriting human synthesis; keep the corpus-KB→prior-candidate /
    benchmark-idea / rescan-target routing typed-and-validation-gated. Status
    `in_progress`.
  Commit: `see Commit Log`

## Current Frontier

This tree stays **`active`** — R15c–R15g are open ROADMAP lanes. Ownership +
audit is complete (the `.4` umbrella obligation); each leaf carries the audited
delivered surface + remaining scope. Advancing any leaf's open scope is a future
owned tree.

| Order | Leaf | Status | Why |
| --- | --- | --- | --- |
| 1 | `R15C-R15G-LEARNING-PLANE-BACKFILL.1` | `in_progress` | R15c owned + audited; rescan-loop completeness open |
| 2 | `R15C-R15G-LEARNING-PLANE-BACKFILL.2` | `in_progress` | R15d owned + audited; full-corpus arbitration open |
| 3 | `R15C-R15G-LEARNING-PLANE-BACKFILL.3` | `in_progress` | R15e owned + audited; evergreen fixture curation |
| 4 | `R15C-R15G-LEARNING-PLANE-BACKFILL.4` | `in_progress` | R15f owned + audited; protocol-scope / new families open |
| 5 | `R15C-R15G-LEARNING-PLANE-BACKFILL.5` | `in_progress` | R15g owned + audited; more auto-refresh families open |

## Decisions

- `2026-05-31`: phase-group backfill tree, one leaf per in-progress milestone.
  Unlike `R1-R5-FOUNDATION-BACKFILL` / `R8-R13-EXTRACTION-BACKFILL` (closed —
  their milestones are Done), this tree stays `active` because R15c–g are open;
  leaves stay `in_progress` and record both delivered surface AND remaining
  scope, so the open work is durably owned and survives session loss.

## Open Questions

- Per-milestone child trees vs advancing leaves in place: when an open
  completion criterion is actually worked, give it its own owning tree (no code
  change without one) and cross-link it here; this backfill tree tracks
  ownership + audit, not the implementation of the remaining scope.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `.1` R15c | semantic-hint/observation/candidate/arbitration/consensus surfaces in evidence+semantic+intent; nlp-enrich refresh; validate arbitration reporting; book `domain/actor-connectivity`; remaining = first-class rescan loop | `passed` |
| `2026-05-31` | `.2` R15d | polarity/semantic/interface/connectivity conflict records carried + validate-reported across 3 stages; book `quality/validation`+`domain/actor-connectivity`; remaining = full-corpus arbitration | `passed` |
| `2026-05-31` | `.3` R15e | `kg-bench` command + 153 `test_data/kg_quality` fixtures (gold/negative/residual/stage-patched); book `quality/kg-bench`; remaining = evergreen curation | `passed` |
| `2026-05-31` | `.4` R15f | `prior_memory.rs` CorpusMemory + `learn-priors` command + 5 prior families + ≥4 provenance-safe consumers + kg-bench staging; book `quality/corpus-memory`; remaining = protocol-scope / new families | `passed` |
| `2026-05-31` | `.5` R15g | `corpus_kb/` root (11 page families + README/SCHEMA) + `corpus-kb` command (validation-finding projection); book `quality/corpus-kb`; remaining = more auto-refresh families | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R15C-R15G-LEARNING-PLANE-BACKFILL.{1..5}` | `ROADMAP-TASKTREE-COVERAGE.4 — backfill+audit R15c–R15g learning/eval/corpus lanes` | audit/ownership only; lanes open → tree stays active |

## Changelog

- `2026-05-31`: Created (stays `active`) — owned + audited R15c–R15g (KG-guided
  multimodal rescans / cross-modality conflict arbitration / KG-quality
  benchmark hardening / cross-document learning plane / corpus knowledge base).
  Recorded the delivered surface (symbols/commands/fixtures verified present)
  AND the remaining open scope per leaf; all book-covered. Open lanes → leaves
  `in_progress`; advancing any remaining scope is a future owned tree.
  (`ROADMAP-TASKTREE-COVERAGE.4`.)
