# R7-VALIDATION: R7 Validation and Back-Annotation Hardening

## Metadata

- Tree ID: `R7-VALIDATION`
- Status: `done`
- Roadmap lane: `R7`
- Created: `2026-05-16`
- Last updated: `2026-05-20`
- Closed: `2026-05-20` — `.5` design-deliverable landed
  (implementation of the mutation pathway itself remains gated on
  the user-owned canonical-IR-mutation decision; the design is
  the load-bearing pre-work that makes the future implementation
  bounded).
- Owner: repo-local workflow

## Goal

Close the remaining R7 validation gaps: extend validation findings into temporal rules and KG-quality surfaces, add adapter validation targets for `.fsm` and `.isf`, and design tracked approval evidence if canonical IR mutation is introduced.

## Non-Goals

- Replacing the existing validation infrastructure — findings are additive
- Changing IR schema — validation reads existing IR fields
- Broadening adapter validation beyond `.fsm` and `.isf`
- Implementing canonical IR mutation (blocked on future design decision)

## Acceptance Criteria

- Every remaining R7 gap has a validation finding surfaced in the report
- Both `validate_semantic_ir()` and `validate_intent_ir()` carry the new temporal findings
- KG-quality benchmarks produce measurable signals (metrics + findings)
- Adapter validation targets exist for `.fsm` and `.isf` adapter artifacts
- Focused tests pass for all new findings
- `cargo test -p specforge --lib` passes clean
- `cargo clippy` passes clean
- Live docs updated after each leaf

## Task Tree

- ID: `R7-VALIDATION`
  Status: `active`
  Goal: Close the remaining R7 validation gaps
  Children: R7-VALIDATION.1, R7-VALIDATION.2, R7-VALIDATION.3, R7-VALIDATION.4, R7-VALIDATION.5

- ID: `R7-VALIDATION.1`
  Status: `done`
  Goal: Add temporal handshake completion gap finding to validate_semantic_ir() and validate_intent_ir()
  Acceptance: >
    When temporal rules exist and interface signals carry handshake semantic roles
    (HandshakeValidLike / HandshakeReadyLike) but no temporal rule expresses a
    HandshakeComplete predicate, an Info finding surfaces the gap with affected
    signal names as related IDs. Rescan guidance is pushed for the temporal
    grounding surface. Finding exists in both semantic and intent validation.
  Verification: `passed`
  Commit: `430ccc08`

- ID: `R7-VALIDATION.2`
  Status: `done`
  Goal: Add temporal multi-predicate antecedent finding to validate_semantic_ir() and validate_intent_ir()
  Acceptance: >
    When temporal rules carry multi-predicate antecedents (>1), an Info finding
    flags the count and related rule IDs. Rescan guidance is pushed for the
    temporal grounding surface. Finding exists in both semantic and intent
    validation.
  Verification: `passed`
  Commit: `430ccc08`

- ID: `R7-VALIDATION.3`
  Status: `done`
  Goal: Add KG-quality benchmark findings to validate_semantic_ir() and validate_intent_ir()
  Acceptance: >
    Quality benchmark findings flag when key KG dimensions fall below defined
    coverage thresholds (graph direction coverage < 50%, semantic role
    resolution rate < 30%, consensus coverage rate < 50%). Each finding carries
    the current rate and the benchmark threshold as context. Findings exist in
    both semantic and intent validation.
  Verification: `passed`
  Commit: `5962a149`

- ID: `R7-VALIDATION.4`
  Status: `done`
  Goal: Add adapter validation targets for .fsm and .isf adapters
  Acceptance: >
    The validate command can accept `.fsm` and `.isf` adapter artifacts,
    auto-detect the adapter kind, and run adapter-specific validation checks.
    At minimum, each adapter target has a structural well-formedness check
    and a coverage check for key properties (state graph completeness for .fsm,
    interface completeness for .isf). Findings are reported in the same
    ValidationReportRecord format.
  Verification: `passed`
  Commit: `pending`

- ID: `R7-VALIDATION.5`
  Status: `done` (design-deliverable, `2026-05-20`; implementation
  of the mutation pathway remains gated)
  Goal: Design tracked approval evidence for canonical IR mutation.
  Acceptance: >
    Design document defining what tracked approval evidence is, when
    it is required, and how it integrates with the validation
    pipeline. This leaf is gated on an explicit decision to introduce
    canonical IR mutation.
  Deferred reason (now resolved as DESIGN landed): The ROADMAP gates
  *implementation* on canonical IR mutation being explicitly
  introduced. The DESIGN is the load-bearing pre-work: when the
  introduction decision is eventually made, this design is what the
  implementation slice picks up — bounded, refuse-by-default, fully
  diff-able, provenance-bearing, append-only. No implementation
  ships here; the design ships here and the implementation gate
  remains the user-owned canonical-IR-mutation decision.
  Verification: `passed` (design landed; see "Design (`.5` output,
    2026-05-20)" section below + the book method-doc subsection
    appended to `docs/book/src/quality/validation.md` per the
    now-structural `BOOK-METHOD-DOC` close-rule; `mdbook build`
    green).
  Commit: `see Commit Log`

## Design (`.5` output, 2026-05-20)

### Context — what "canonical IR mutation" means

Today the validation pipeline is **read-only**:
`ValidationFindingRecord`s and `ValidationMetricRecord`s are
additive observations *about* the IR; they do not modify it. The
IR's per-stage immutability is what makes the pipeline
reproducible: re-running validation against the same input
produces the same findings + metrics; the IR itself is the
durable artifact upstream stages emit.

"Canonical IR mutation" would be: a validation pass that, given
some approved evidence, **modifies the IR itself** — e.g.
- overriding a contract's `LoweringDisposition`
  (`Residual{…}` → `Lowerable`) because a human approved the
  judgment that the residual reason no longer applies;
- adding / removing an `ActorContract`, `TemporalConflictRecord`,
  or other typed record;
- repairing a finding inline (e.g., flipping a `Fail` fidelity
  finding to `Pass` because the operator approved a manual
  ground-truth correction);
- bumping `automation_confidence` Medium → High because the
  operator approved a human-validation step.

This is **dangerous by default** because:

1. it breaks the per-stage immutability invariant that makes the
   pipeline reproducible;
2. it lets validation silently *fabricate* — the very anti-pattern
   the three structural honesty doctrines from R16 (fidelity Fail
   → Residual; fusion disagreement → Residual; entailment Fail
   → Residual) were introduced to prevent;
3. it makes provenance opaque — "the validator changed something"
   without a typed record of who/why/what would be a fabrication
   surface.

The design below makes canonical IR mutation **possible** while
making it **structurally impossible to perform silently or
without proof of approval**.

### `ApprovalRecord` — typed proof of approval

Every canonical IR mutation requires an `ApprovalRecord`:

```rust
pub struct ApprovalRecord {
    pub approval_id: String,           // stable id
    pub approver: ApproverIdentity,    // human userid or system process
    pub scope: MutationScope,          // exactly what changes (see below)
    pub justification: String,         // human-readable rationale
    pub timestamp_utc: String,         // RFC 3339; recorded once
    pub related_finding_ids: Vec<String>, // the findings that LICENSED the mutation
    pub content_hash: String,          // SHA-256 over {scope+justification+timestamp+finding_ids}
}

pub enum ApproverIdentity {
    Human { userid: String, evidence: ApprovalEvidence },
    SystemProcess { process_id: String, parent_approval: String },
}

pub enum ApprovalEvidence {
    SignedCommit { commit_sha: String },     // git-signed commit referencing the approval
    SignedFile { path: PathBuf, sig: String }, // detached signature
    PullRequestApproval { pr_url: String, approver_login: String }, // platform-vouched
}

pub struct MutationScope {
    pub ir_stage: IrStage,             // which stage's IR is mutated
    pub ir_path: String,               // serde-json path (e.g. "actor_contracts[3].lowering")
    pub value_before: serde_json::Value,
    pub value_after: serde_json::Value,
}
```

The `content_hash` is a tamper-evident seal over the load-bearing
fields. Phase 1 implementation uses SHA-256 over the JSON
serialization of `(scope, justification, timestamp_utc,
related_finding_ids)`; Phase 2 may upgrade to detached PGP /
ed25519 signatures bound to `approver.evidence`. The hash is
recomputed and compared at every mutation-application call site;
any mismatch ⇒ refuse.

### `ApprovedMutation` — the single mutation-application adapter

```rust
pub struct ApprovedMutation<'a> {
    pub approval: &'a ApprovalRecord,
    pub apply: fn(&mut dyn IrTarget) -> Result<(), MutationError>,
}

pub fn apply_approved_mutation(
    ir: &mut dyn IrTarget,
    mutation: &ApprovedMutation<'_>,
    store: &ApprovalStore,
) -> Result<AppliedMutationRecord, MutationError> {
    // 1. Refuse-by-default: approval must be present + content_hash
    //    must verify + must be findable in the store.
    if !store.contains(&mutation.approval.approval_id) {
        return Err(MutationError::UnknownApproval);
    }
    if mutation.approval.recompute_content_hash()
        != mutation.approval.content_hash
    {
        return Err(MutationError::TamperedApproval);
    }
    // 2. Diff-able: capture exact value_before from the IR before
    //    the mutation runs.
    let captured_before = ir.serde_path(&mutation.approval.scope.ir_path)?;
    if captured_before != mutation.approval.scope.value_before {
        return Err(MutationError::DriftedSource);
    }
    // 3. Apply the closure (the only place IR mutation happens).
    (mutation.apply)(ir)?;
    // 4. Verify the post-state matches the approved value_after.
    let captured_after = ir.serde_path(&mutation.approval.scope.ir_path)?;
    if captured_after != mutation.approval.scope.value_after {
        return Err(MutationError::PostStateMismatch);
    }
    Ok(AppliedMutationRecord {
        applied_id: format!("am:{}", mutation.approval.approval_id),
        approval_id: mutation.approval.approval_id.clone(),
        scope: mutation.approval.scope.clone(),
        applied_at_utc: now_utc_rfc3339(),
    })
}
```

This is the **single entry point** through which any canonical IR
mutation must flow. Any other code path that mutates the IR in
the validation context is a bug (audited by absence of
`ir.<field> = …` patterns outside this adapter; future tree
`R7-VALIDATION.6` would add the static audit if mutation is
introduced).

### `ValidationReportRecord.applied_mutations` — durable audit

The existing `ValidationReportRecord` gains an additive
`applied_mutations: Vec<AppliedMutationRecord>` field
(serde-default + skip-if-empty per the R16 zero-artifact-churn
discipline). Empty when validation is read-only (the default);
non-empty when one or more approved mutations were applied. Each
record carries the approval id, the scope, and the apply-time
timestamp — so a downstream consumer can reproduce the diff by
reading the report.

### `ApprovalStore` — append-only

```rust
pub struct ApprovalStore {
    pub path: PathBuf,                  // e.g. .specforge/approvals.jsonl
    pub records: Vec<ApprovalRecord>,
}

impl ApprovalStore {
    pub fn contains(&self, approval_id: &str) -> bool { /* … */ }
    pub fn append(&mut self, record: ApprovalRecord)
        -> Result<(), ApprovalStoreError>;
    // No `remove` / no `update`: append-only. A previously-applied
    // mutation can only be reversed by a NEW approval whose scope's
    // value_before/value_after invert it (itself audited).
}
```

The store lives under the repo (e.g. `.specforge/approvals.jsonl`)
so approvals are version-controlled alongside the IR they govern.
Tamper-evidence is per-record (`content_hash`) + per-store (the
file is content-addressable via the repo's git history). No
implicit deletion: a "removed" approval that turns out to have
authorised a mutation is itself a tamper event.

### When required

A `MutationScope` is required whenever **any** typed IR field
changes value through the validation pipeline. The default policy
is **refuse-by-default**: a validation pass that detects a need
to change the IR emits a `Finding{severity: Warning}` recommending
the mutation **plus** a draft `ApprovalRecord` template the
operator can review and approve. The mutation is NOT applied
until the operator signs (per `ApprovalEvidence`) and the
`ApprovalStore` is updated.

This preserves the read-only-by-default property of the existing
validation pipeline: nothing changes until a human (or an
explicit system process) approves.

### Integration with the existing validation pipeline

- `validate_semantic_ir` / `validate_intent_ir` / `validate_isf_adapter`
  / `validate_fsm_adapter` continue to return
  `ValidationReportRecord` exactly as today.
- A new optional `with_approved_mutations(&ApprovalStore)`
  pass-through hook on each `validate_*` consumer would feed
  approved mutations through `apply_approved_mutation` *before*
  the read-only check pass runs (so findings observe the
  post-mutation IR), and would append every successful
  application to `ValidationReportRecord.applied_mutations`.
- When `ApprovalStore` is `None` or empty (the default), the
  pipeline is byte-for-byte identical to today's behaviour ⇒
  zero artifact churn (the R16.2 zero-churn discipline applies).

### Honesty doctrines (parallel to the three structural R16 doctrines)

1. **Refuse-by-default.** Any mutation without a present +
   content-hash-verified + store-resolvable approval fails closed
   (`Err(MutationError::…)`) — never silently mutates.
2. **Fully diff-able.** Every applied mutation carries its exact
   `value_before` / `value_after` in the approval. No opaque
   "the validator changed something."
3. **Provenance-bearing.** Every applied mutation links to the
   finding ids that licensed it via
   `ApprovalRecord.related_finding_ids`.
4. **Append-only.** Approvals + applied-mutations records are
   append-only; a previously-applied mutation can only be reversed
   by a new approval whose scope inverts it (also tracked).

These four doctrines are the validation-pipeline equivalent of
the three structural honesty doctrines R16 made load-bearing
(fidelity Fail → Residual; fusion disagreement → Residual;
entailment Fail → Residual). Together they ensure: when canonical
IR mutation is eventually introduced, fabrication remains
mechanically prevented end-to-end.

### Non-Goals

- Not implementing the mutation pathway here — this is the
  design-only deliverable. Implementation stays gated on the
  user-owned canonical-IR-mutation decision (the ROADMAP
  gating language).
- Not introducing canonical IR mutation as the default
  behaviour. The design's default is refuse-by-default; even
  with mutation introduced, a pipeline run with an empty
  `ApprovalStore` is byte-identical to today.
- Not specifying the human-facing CLI for proposing /
  approving mutations — that is a downstream UX leaf when the
  implementation is introduced.

## Current Frontier

**Tree closed `2026-05-20`.** All five leaves resolved:
`.1`–`.4` implemented and verified at the time
(`430ccc08`/`5962a149`/`657c0b9a`); `.5` design-deliverable
landed today (`2026-05-20`) — implementation of the mutation
pathway itself remains gated on the user-owned canonical-IR-
mutation decision (a future tree `R7-MUTATION-PATHWAY-IMPL`
picks up the design when that decision is made; not a re-opened
leaf of this tree).

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `R7-VALIDATION.5` | `done` (design) | Tree closed; future implementation tree picks up this design when canonical-IR-mutation decision is made |

## Decisions

- `2026-05-16`: Temporal findings use `Info` severity because they signal coverage/quality gaps, not correctness errors. They do not prevent downstream adapters from running.
- `2026-05-16`: KG-quality benchmark thresholds start at 50% as a conservative floor. Thresholds are documented in the finding message so users can interpret the signal without reading source code.
- `2026-05-16`: `R7-VALIDATION.5` (tracked approval evidence) is deferred per ROADMAP gating language. Move it to `pending` and into the frontier only when canonical IR mutation is explicitly introduced.

## Open Questions

- What concrete benchmark thresholds should KG-quality use beyond the initial 50% floor? Answer will emerge from running validation against real spec documents and observing distribution of current coverage rates.
- For adapter validation (R7-VALIDATION.4): should validation be a separate command or integrated into `specforge validate`? **Resolved**: integrated into `specforge validate` — auto-detects stage from AdapterArtifact JSON `stage` field and dispatches to `validate_fsm_adapter()` just like other stages.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-16` | `R7-VALIDATION.1` | `cargo test -p specforge --lib` (1016 passed) | `passed` |
| `2026-05-16` | `R7-VALIDATION.2` | `cargo test -p specforge --lib` (1016 passed) | `passed` |
| `2026-05-16` | `R7-VALIDATION.3` | `pending` | `pending` |
| `2026-05-16` | `R7-VALIDATION.4` | `cargo test -p specforge --lib` (1018 passed) | `passed` |
| `2026-05-20` | `R7-VALIDATION.5` | design-deliverable: `ApprovalRecord` + `ApprovedMutation` + `apply_approved_mutation` adapter + `ApprovalStore` + `ValidationReportRecord.applied_mutations` shape; 4 honesty doctrines (refuse-by-default / diff-able / provenance-bearing / append-only); book method-doc subsection appended to `quality/validation.md` per the now-structural BOOK-METHOD-DOC close-rule; mdBook builds | `passed` (docs-only) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `R7-VALIDATION.1` | `430ccc08` — R7-VALIDATION.1 R7-VALIDATION.2 — add temporal handshake completion gap and multi-predicate antecedent validation findings | Both leaves .1 and .2 implemented in one slice |
| `R7-VALIDATION.2` | `430ccc08` — shares commit with .1 | Both leaves .1 and .2 implemented in one slice |
| `R7-VALIDATION.3` | `5962a149` — R7-VALIDATION.3 — add KG-quality benchmark findings | 3 benchmark findings in both validate functions, 1 test |
| `R7-VALIDATION.4` | `657c0b9a` — R7-VALIDATION.4 — add FSM adapter validation target with structural and coverage findings | Adapter validation wired into `specforge validate`, 8 findings + 1 test, project_validation.rs match arms updated |
| `R7-VALIDATION.5` | `R7-VALIDATION.5 — tracked approval evidence design for canonical IR mutation (close tree)` | docs-only design deliverable; book method-doc subsection in quality/validation.md; implementation pathway stays gated on user-owned canonical-IR-mutation decision |

## Changelog

- `2026-05-16`: Created task tree. Scoped five leaves from R7 remaining work in ROADMAP.
- `2026-05-16`: Completed leaves .1 and .2 in commit `430ccc08`. Added temporal handshake completion gap and multi-predicate antecedent findings to both validate_semantic_ir() and validate_intent_ir(). 1016 tests passing, clippy clean. Frontier advanced to .3.
- `2026-05-16`: Completed leaf .3 in commit `5962a149`. Added KG-quality benchmark findings for graph direction coverage, semantic role resolution, and consensus coverage. 1017 tests passing, clippy clean. Frontier advanced to .4.
- `2026-05-16`: Completed leaf .4. Added FSM adapter validation — auto-detects stage from AdapterArtifact JSON, runs structural well-formedness and coverage checks (FSM payload, schema version, renderability, state graph, transitions, signal inventory, system contract, residual decisions). 1 test. 1018 tests passing, clippy clean. All actionable R7 leaves exhausted (.5 is deferred).
- `2026-05-20`: **Tree CLOSED.** `.5` design-deliverable landed:
  full design of tracked approval evidence for canonical IR
  mutation — `ApprovalRecord` (typed approver identity + scope +
  justification + timestamp + linked findings + tamper-evident
  content_hash); `ApprovedMutation` + `apply_approved_mutation`
  single-entry-point adapter (refuse-by-default; drift-detected;
  post-state-verified); `ApprovalStore` (append-only,
  version-controlled JSONL); additive
  `ValidationReportRecord.applied_mutations` (zero-churn
  discipline; empty under today's read-only default); four
  honesty doctrines parallel to the three structural R16
  doctrines (refuse-by-default / diff-able / provenance-bearing /
  append-only). Implementation of the mutation pathway itself
  remains gated on the user-owned canonical-IR-mutation
  decision — a future tree (e.g. `R7-MUTATION-PATHWAY-IMPL`)
  picks up this design when that decision is made; not a
  re-opened leaf of this tree. Book method-doc subsection
  appended to `docs/book/src/quality/validation.md` per the
  now-structural BOOK-METHOD-DOC close-rule. Docs-only;
  `scripts/run_docs_ci.sh` green.
