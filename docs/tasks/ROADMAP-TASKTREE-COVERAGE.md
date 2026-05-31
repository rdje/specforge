# ROADMAP-TASKTREE-COVERAGE: every roadmap milestone task-tree-owned + audited + locked to code & mdBook

## Metadata

- Tree ID: `ROADMAP-TASKTREE-COVERAGE`
- Status: `done`
- Roadmap lane: `R0` (governance / continuity doctrine)
- Created: `2026-05-31`
- Last updated: `2026-05-31`
- Owner: repo-local workflow

## Goal

Enforce the task-tree doctrine across the WHOLE roadmap (user directive
`2026-05-31`): every roadmap milestone/phase must be a task-tree, including the
ones delivered **before** the task-tree system existed — and for those, the
tree must carry a thorough, accurate, meticulous **audit** of what the codebase
actually delivered, annotated back into the tree. Also: create the missing
`docs/TASK_TREE_README.md`, and establish/verify the standing lock that
**ROADMAP ↔ codebase ↔ mdBook never drift** (the mdBook must always reflect
what the code does).

This survives session loss/crash: every milestone's delivery + audit + book
coverage becomes a durable, machine-tracked record.

## Coverage gap (measured `2026-05-31`)

Roadmap milestones: R0–R16 + R15b–g (23 total). Existing task-trees already
OWN these lanes: **R0** (SIGNOFF-REMEDIATION, AUDIT-DOC-RECONCILE,
AUDIT-PROVIDER-FRAMING-RECONCILE, BOOK-METHOD-DOC, BOOK-USER-FRIENDLY-BACKFILL,
FSMGEN-ISSUE-REPORTING, FSMGEN-SUBMODULE-BUMP, LLM-TEXT-TRANSPORT-DEDUP),
**R6** (PROVENANCE-HARDENING + R6-*-HARDENING + R6-ISF-ADAPTER +
ISF-ONLY-CONSOLIDATION + ISF-ONLY-IR-PRUNE + ISF-* + FSMGEN-REFRESH-INTEGRATE),
**R7** (R7-VALIDATION), **R14** (R14-SIGNAL-RESOLVE), **R15**
(R15-GRAPH-DIRECTION-MIGRATION), **R15b** (ISF-TEMPORAL-LOWERING), **R16**
(R16-INTENT-CAPTURE + 6 sub-trees + R16-MODULE-HARDENING + CVE-PROSE-EXTRACTION).

**UNCOVERED milestones (need a backfill tree + audit):**
- **R1, R2, R3, R4, R5** — foundational (Done; predate the system).
- **R8, R9, R10, R11, R12, R13** — (Done / R9 Mostly Done; predate the system).
- **R15c, R15d, R15e, R15f, R15g** — (In Progress; learning/eval/corpus lanes).

(16 milestones.) Each gets a `R<NN>-BACKFILL`-style tree whose status reflects
the roadmap (`done` / `in_progress`) and whose Verification Log records the
**audit**: what code/commands/tests deliver it, and whether the mdBook covers
it (drift flagged + reconciled).

## Non-Goals

- NOT re-implementing or changing delivered milestones — this is audit +
  documentation/ownership backfill only. Any *fix* a milestone's audit
  uncovers becomes its own normal task-tree (no code change without one).
- NOT duplicating the roadmap into the trees — each backfill tree links to its
  ROADMAP section and records only the audit + ownership + book-coverage status.

## Acceptance Criteria

- `docs/TASK_TREE_README.md` created (task-tree system overview + the doctrine:
  no code change without an owning tree; every activity tracked; past-work
  audited into trees; ROADMAP ↔ code ↔ mdBook locked).
- Each of the 16 uncovered milestones has an owning task-tree registered in
  `docs/TASK_TREE.md`, with an audit (delivering code/commands/tests + book
  coverage) in its Verification Log; status matches the roadmap.
- A standing alignment check recorded (ROADMAP ↔ codebase ↔ mdBook); any drift
  found is reconciled (text describes the code — `AUDIT-DOC-RECONCILE` doctrine)
  or scoped as a follow-up tree.
- `docs/TASK_TREE.md`'s Active Task Trees table lists all backfill trees;
  `scripts/run_ci.sh` green (incl. mdBook) at each leaf that touches the book.

## Task Tree

- ID: `ROADMAP-TASKTREE-COVERAGE`
  Status: `done`
  Goal: whole-roadmap task-tree ownership + audit + alignment lock
  Children: `.1`, `.2`, `.3`, `.4`, `.5`

- ID: `ROADMAP-TASKTREE-COVERAGE.1`
  Status: `done`
  Goal: >
    Create `docs/TASK_TREE_README.md` (system + doctrine); record the measured
    coverage gap (this file); register the umbrella in `docs/TASK_TREE.md`.
    Docs-only.
  Acceptance: README created; gap recorded; umbrella registered; committed.
  Verification: >
    passed (`2026-05-31`) — created `docs/TASK_TREE_README.md` (system overview
    + the 5-point doctrine: no code change without an owning tree; every
    activity tracked; whole roadmap tracked; past work audited into trees;
    ROADMAP↔code↔mdBook locked with zero drift). Measured + recorded the
    coverage gap (existing trees cover lanes R0/R6/R7/R14/R15/R15b/R16; 16
    uncovered milestones: R1–R5, R8–R13, R15c–g). Registered the umbrella in
    `docs/TASK_TREE.md` Active Task Trees + cross-linked the README from its
    header. Docs-only (no code/book; CI invariant).
  Commit: `see Commit Log`

- ID: `ROADMAP-TASKTREE-COVERAGE.2`
  Status: `done`
  Goal: >
    Backfill + audit the foundational milestones **R1–R5**: read each ROADMAP
    section, audit the delivering code/tests, create each owning tree
    (status `done`, audit in Verification Log), confirm book coverage / flag
    drift. Register in the index.
  Acceptance: R1–R5 each owned by an audited tree; book-coverage noted; CI green.
  Verification: >
    passed (`2026-05-31`) — created `R1-R5-FOUNDATION-BACKFILL` owning R1–R5 as
    audited leaves. AUDIT confirmed all delivered: R1 (crate=`specforge`,
    IntentIR-canonical, `.isf`-only), R2 (`ir/source`+docling+`ingest`/`clean`/
    `doctor`, 12 tests), R3 (`ir/evidence`+command, 120), R4 (`ir/semantic`+
    command, 381), R5 (`ir/intent`+command, 52); all book-covered
    (`pipeline/*` + `runtime-and-doctor`); ROADMAP §R1–R5 criteria met; further
    hardened by the R6-*-HARDENING trees. Registered in the index. Docs-only.
  Commit: `see Commit Log`

- ID: `ROADMAP-TASKTREE-COVERAGE.3`
  Status: `done`
  Goal: >
    Backfill + audit **R8–R13** (same method). Register in the index.
  Acceptance: R8–R13 each owned by an audited tree; book-coverage noted; CI green.
  Verification: >
    passed (`2026-05-31`) — created `R8-R13-EXTRACTION-BACKFILL` owning R8–R13
    as audited leaves. AUDIT confirmed all delivered: R8 (`ir/source` Tier-1
    capture — structured_tables/content_elements/document_sections/
    document_profile/table_kind), R9 (`ir/evidence` synthesize_* + monotone
    `converge_evidence_extractions` + `NormativeStatement` + polarity +
    provenance — Mostly Done, residual = robustness hardening), R10 (`DiagramKind`
    + `enrich` VLM → `VisualObservation` → `semantic` merge; Ollama+qwen2.5vl
    default), R11 (`nlp_enrich` run/prompt/alias/Layers A–E; Done 2026-04-03),
    R12 (`extract_alias_phrase` filter + AMBA direction parsing + APB94/AHB94/
    AXI85 baselines), R13 (`RelationKind`/`ActorSignalRelation` +
    `extract_actor_signal_relations`); all book-covered (`pipeline/sourceir`,
    `pipeline/evidenceir`, `quality/validation`, `domain/actor-connectivity`,
    `runtime-and-doctor`, `commands/overview`); ROADMAP §R8–R13 criteria met.
    Registered in the index. Docs-only (CI invariant).
  Commit: `see Commit Log`

- ID: `ROADMAP-TASKTREE-COVERAGE.4`
  Status: `done`
  Goal: >
    Backfill + audit the learning/eval/corpus lanes **R15c–R15g** (same
    method; these are `In Progress`, so trees stay `active`/`in_progress` with
    the delivered-so-far audit + remaining scope). Register in the index.
  Acceptance: R15c–R15g each owned by an audited tree; remaining scope captured; CI green.
  Verification: >
    passed (`2026-05-31`) — created `R15C-R15G-LEARNING-PLANE-BACKFILL` (stays
    `active`; one `in_progress` leaf per open milestone). AUDIT confirmed the
    delivered surface present per milestone: R15c (semantic-hint/observation/
    candidate/arbitration/consensus surfaces across evidence+semantic+intent +
    nlp-enrich refresh + validate arbitration reporting), R15d (polarity/
    semantic/interface/connectivity conflict records carried + validate-reported
    across all 3 stages), R15e (`kg-bench` command + **153** `kg_quality`
    fixtures), R15f (`prior_memory.rs` `CorpusMemory` + `learn-priors` + 5 prior
    families + ≥4 provenance-safe consumers), R15g (`corpus_kb/` root with **11**
    page families + README/SCHEMA + `corpus-kb` command). Each leaf also records
    the REMAINING open completion criteria; all book-covered (`quality/kg-bench`,
    `quality/corpus-memory`, `quality/corpus-kb`, `domain/actor-connectivity`,
    `quality/validation`). Registered in the index. Docs-only (CI invariant).
  Commit: `see Commit Log`

- ID: `ROADMAP-TASKTREE-COVERAGE.5`
  Status: `done`
  Goal: >
    ROADMAP ↔ codebase ↔ mdBook alignment lock: verify every milestone's
    capability is reflected in the mdBook (no drift); reconcile or scope-out
    any gap; record the standing no-drift discipline (cross-ref
    `AUDIT-DOC-RECONCILE` + `BOOK-METHOD-DOC`). Close the umbrella.
  Acceptance: alignment verified/recorded; drift reconciled or tree-scoped; umbrella CLOSED; full `scripts/run_ci.sh` green.
  Verification: >
    passed (`2026-05-31`) — alignment swept across all milestones R0–R16 +
    R15b–g (every milestone has an owning tree + a book chapter). Two real
    drifts found and reconciled against the code (code-is-truth,
    `AUDIT-DOC-RECONCILE`): (1) COMMAND-SURFACE drift — the two newest commands
    `extract-contracts` + `signal-resolve` were in their capability chapters but
    ABSENT from the command reference; added both to `commands/overview.md` (exact
    flags from `cli.rs`) + a user-friendly section in
    `commands/quality-and-learning.md`; the book now lists all **19** CLI
    commands. (2) CAPABILITY drift — `quality/validation.md` claimed `specforge
    validate` runs an "eight-finding `.fsm` adapter" validator; the code's
    `AdapterTargetArg` has ONLY `Isf` and `validate.rs` auto-detects only
    `IrStage::IsfAdapter` → `validate_isf_adapter` (no `.fsm` validator exists);
    rewrote the section to the real `.isf`-only **six-finding** surface (payload
    presence / schema freshness / renderability / signal inventory / behavior /
    residual visibility) + fixed the "`.fsm`, `.isf`, or future targets" bullet.
    Confirmed all other book `.fsm` mentions are correct (they document the
    `ISF-ONLY-CONSOLIDATION`/`PRUNE` removal or the FSMGen downstream boundary).
    Provider framing confirmed correct (Ollama + qwen2.5vl named as the
    production default; the past "provider doesn't exist" framing remains only as
    the `documentation-scope.md` reconciliation record). Standing no-drift
    discipline recorded (this leaf + `AUDIT-DOC-RECONCILE` + `BOOK-METHOD-DOC`
    close-rule + `COMMIT.md`). Full `scripts/run_ci.sh` GREEN (1169 tests, 0
    failed; rustdoc warnings-denied + mdBook built). Umbrella CLOSED.
  Commit: `see Commit Log`

## Current Frontier

**Umbrella CLOSED `2026-05-31`** — the whole roadmap is task-tree-owned +
audited, and ROADMAP ↔ codebase ↔ mdBook are aligned (two drifts reconciled,
full CI green). The only `active` descendant is
`R15C-R15G-LEARNING-PLANE-BACKFILL` (tracks the still-open R15c–g lanes);
advancing any of its remaining scope is a future owned tree.

| Order | Leaf | Status | Why |
| --- | --- | --- | --- |
| 1 | `ROADMAP-TASKTREE-COVERAGE.1` | `done` | README + gap record + umbrella registration landed |
| 2 | `ROADMAP-TASKTREE-COVERAGE.2` | `done` | R1–R5 owned + audited (`R1-R5-FOUNDATION-BACKFILL`) |
| 3 | `ROADMAP-TASKTREE-COVERAGE.3` | `done` | R8–R13 owned + audited (`R8-R13-EXTRACTION-BACKFILL`) |
| 4 | `ROADMAP-TASKTREE-COVERAGE.4` | `done` | R15c–R15g owned + audited (`R15C-R15G-LEARNING-PLANE-BACKFILL`, stays active) |
| 5 | `ROADMAP-TASKTREE-COVERAGE.5` | `done` | alignment lock done (2 drifts reconciled); umbrella CLOSED |

## Decisions

- `2026-05-31`: created on the user's standing doctrine directive — the whole
  roadmap must be task-tree-owned, past work audited into trees, and
  ROADMAP/code/mdBook locked with no drift. Grouped the 16 uncovered milestones
  into phase leaves (R1–R5 / R8–R13 / R15c–g) for reviewable slices; each
  milestone still gets its own owning tree file + audit.

## Open Questions

- Per-milestone tree granularity vs grouped: chose one owning tree per
  uncovered milestone (faithful to "every milestone/phase is a task-tree"),
  created in phase-grouped leaves. Revisit if the user prefers a single
  consolidated backfill ledger.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-31` | `ROADMAP-TASKTREE-COVERAGE.1` | `TASK_TREE_README.md` created (system + 5-point doctrine); coverage gap measured + recorded (16 uncovered milestones); umbrella registered in `docs/TASK_TREE.md` + README cross-linked; docs-only (CI invariant) | `passed` |
| `2026-05-31` | `ROADMAP-TASKTREE-COVERAGE.2` | `R1-R5-FOUNDATION-BACKFILL` created (R1–R5 as audited leaves); delivering code/tests/book verified per milestone; ROADMAP §R1–R5 criteria met; registered; docs-only | `passed` |
| `2026-05-31` | `ROADMAP-TASKTREE-COVERAGE.3` | `R8-R13-EXTRACTION-BACKFILL` created (R8–R13 as audited leaves); delivering symbols/commands + book pages verified per milestone (R9 Mostly-Done residual noted); ROADMAP §R8–R13 criteria met; registered; docs-only | `passed` |
| `2026-05-31` | `ROADMAP-TASKTREE-COVERAGE.4` | `R15C-R15G-LEARNING-PLANE-BACKFILL` created (stays `active`; R15c–g as `in_progress` leaves); delivered surface audited (semantic/conflict surfaces; `kg-bench` + 153 fixtures; `learn-priors`/CorpusMemory; `corpus-kb`/11 families) + remaining scope captured + book-covered; registered; docs-only | `passed` |
| `2026-05-31` | `ROADMAP-TASKTREE-COVERAGE.5` | alignment sweep R0–R16+R15b–g (every milestone owned + book-covered); 2 drifts reconciled code-is-truth — (1) added `extract-contracts`+`signal-resolve` to the book command reference (now 19/19); (2) rewrote the fictitious "8-finding `.fsm` adapter validator" claim to the real `.isf`-only 6-finding surface; standing no-drift discipline recorded; full `scripts/run_ci.sh` GREEN (1169/0); umbrella CLOSED | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ROADMAP-TASKTREE-COVERAGE.1` | `ROADMAP-TASKTREE-COVERAGE.1 — task-tree doctrine README + whole-roadmap coverage umbrella` | docs-only; sets up the backfill+audit+alignment program |
| `ROADMAP-TASKTREE-COVERAGE.2` | `ROADMAP-TASKTREE-COVERAGE.2 — backfill+audit R1–R5 foundation milestones` | created `R1-R5-FOUNDATION-BACKFILL`; all delivered + audited |
| `ROADMAP-TASKTREE-COVERAGE.3` | `ROADMAP-TASKTREE-COVERAGE.3 — backfill+audit R8–R13 extraction-SOTA milestones` | created `R8-R13-EXTRACTION-BACKFILL`; all delivered + audited (R9 residual noted) |
| `ROADMAP-TASKTREE-COVERAGE.4` | `ROADMAP-TASKTREE-COVERAGE.4 — backfill+audit R15c–R15g learning/eval/corpus lanes` | created `R15C-R15G-LEARNING-PLANE-BACKFILL` (stays active); delivered surface audited + remaining scope captured |
| `ROADMAP-TASKTREE-COVERAGE.5` | `ROADMAP-TASKTREE-COVERAGE.5 — ROADMAP↔code↔mdBook alignment lock + close umbrella` | 2 drifts reconciled (command surface + `.fsm`-validator claim); full CI green; umbrella CLOSED |

## Changelog

- `2026-05-31`: `.5` — ROADMAP↔code↔mdBook alignment lock. Swept R0–R16+R15b–g
  (every milestone owned + book-covered). Reconciled 2 real drifts code-is-truth:
  added `extract-contracts`+`signal-resolve` to the book command reference (now
  all 19 CLI commands listed) + a user-friendly section; rewrote the fictitious
  "8-finding `.fsm` adapter validator" claim in `quality/validation.md` to the
  real `.isf`-only 6-finding surface (code has only `AdapterTargetArg::Isf`).
  Recorded the standing no-drift discipline. Full `scripts/run_ci.sh` GREEN
  (1169/0). **Umbrella CLOSED** — whole roadmap is task-tree-owned + audited +
  aligned. (`R15C-R15G-LEARNING-PLANE-BACKFILL` stays `active` for the open
  R15c–g lanes.)
- `2026-05-31`: `.4` — created `R15C-R15G-LEARNING-PLANE-BACKFILL` (stays
  `active`) owning + auditing R15c–g (KG-guided rescans / cross-modality
  conflict arbitration / KG-quality benchmark / cross-document learning plane /
  corpus knowledge base); delivered surface verified present (semantic+conflict
  surfaces; `kg-bench`+153 fixtures; `learn-priors`/`CorpusMemory`; `corpus-kb`/
  11 families) + remaining open scope captured per leaf; all book-covered.
  Frontier → `.5` (alignment lock + close).
- `2026-05-31`: `.3` — created `R8-R13-EXTRACTION-BACKFILL` owning + auditing
  R8–R13 (extraction SOTA: Tier-1 capture / Tier-2 typed evidence / Tier-3 VLM /
  NLP-L3 / multi-spec validation / Tier-2 relations; all delivered + book-covered;
  R9 core landed with residual robustness-hardening noted). Frontier → `.4`
  (R15c–R15g).
- `2026-05-31`: `.2` — created `R1-R5-FOUNDATION-BACKFILL` owning + auditing
  R1–R5 (all delivered, tested, book-covered; ROADMAP criteria met). Frontier
  → `.3` (R8–R13).
- `2026-05-31`: `.1` — created `docs/TASK_TREE_README.md` (doctrine) + this
  umbrella + recorded the coverage gap (R1–R5, R8–R13, R15c–g uncovered);
  registered in the index. Frontier → `.2` (backfill+audit R1–R5).
- `2026-05-31`: Created — own the whole-roadmap task-tree-coverage + past-work
  audit + ROADMAP/code/mdBook alignment program (user doctrine directive).
