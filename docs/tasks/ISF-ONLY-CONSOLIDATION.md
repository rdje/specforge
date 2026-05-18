# ISF-ONLY-CONSOLIDATION: Drop HDL + `.fsm` adapters; SpecForge emits only `.isf`

## Metadata

- Tree ID: `ISF-ONLY-CONSOLIDATION`
- Status: `done`
- Roadmap lane: `R6` (adapter layer — this redefines the adapter strategy)
- Created: `2026-05-18`
- Last updated: `2026-05-18`
- Owner: repo-local workflow

## Goal

Make `.isf` SpecForge's single adapter target. Remove the HDL adapter
surface (SystemVerilog / Verilog / VHDL) and the entire `.fsm` adapter
subsystem from code, tests, fixtures, and documentation. After this tree,
SpecForge lowers `IntentIR` to `.isf` only; FSMGen owns everything
downstream of `.isf` (scheduling, `.fsm`, HDL).

## Background / decision

User decision (2026-05-18): now that the typed `.isf` adapter exists
(`R6-ISF-ADAPTER`), there is no objective reason for SpecForge to keep
HDL adapters or its own `.fsm` adapter. SpecForge's canonical downstream
contract is `IntentIR → .isf → FSMGen`. This supersedes the prior
"`.fsm` active, `.isf` planned" adapter strategy and closes the recorded
`R6-ISF-ADAPTER` open question about lingering HDL `AdapterTarget`
variants by removing them outright (and `.fsm` with them).

Scope reality (surveyed 2026-05-18):
- `crates/specforge/src/ir/adapters.rs` is 28,119 lines: ~1,977 FSM-symbol
  lines, ~62 ISF-symbol lines, 163 `#[test]` fns (the large majority FSM).
- `AdapterTarget` = `{Fsm, Isf, SystemVerilog, Verilog, Vhdl}`;
  `AdapterArtifact` carries `fsm: Option<FsmAdapterArtifact>` and
  `isf: Option<IsfAdapterArtifact>`; `IrStage` = `{… FsmAdapter, IsfAdapter}`.
- FSM coupling also in `cli.rs`, `ir/source.rs`, `ir/evidence.rs`,
  `ir/mod.rs`, `commands/validate.rs`, `commands/project_validation.rs`
  (~25 IrStage match arms), `commands/adapt.rs`, `commands/converge.rs`.
- 153 `crates/specforge/test_data/kg_quality` fixtures (mixed: some are
  FSM-adapter-specific, many are KG/semantic-truthfulness and stay).
- Pervasive `.fsm` content in the mdBook and in README / ROADMAP /
  INTENTIR_SPEC / USER_GUIDE / docs/FSMGEN_FEEDBACK.md.
- `subs/fsmgen` submodule and the `isf_output_passes_fsmgen_strict_validation`
  test STAY — FSMGen is the downstream consumer of `.isf`.

## Non-Goals

- Do not change `.isf` adapter behavior or the `IsfIr` typed model.
- Do not remove the `subs/fsmgen` submodule or the ISF↔FSMGen strict test.
- Do not rewrite history; `R6-FSM-ADAPTER` (closed) is kept as historical
  record, marked `superseded` by this tree.
- Do not delete KG/semantic-truthfulness fixtures that are not FSM-adapter
  specific — only retire genuinely FSM-adapter-only fixtures.
- Do not regress signoff: `scripts/run_ci.sh` must be green at every
  code/fixture-touching leaf.

## Acceptance Criteria

- `AdapterTarget` and `AdapterTargetArg` expose only `isf`.
- `AdapterArtifact` no longer has an `fsm` payload; `IrStage` has no
  `FsmAdapter`; no `FsmAdapterArtifact` / `validate_fsm_adapter` /
  `fsm_adapter_fingerprint` / FSM lowering remains.
- `adapters.rs` (or its ISF successor module) contains only ISF + shared
  adapter scaffolding; the FSM lowering bulk and FSM tests are gone.
- `specforge adapt`/`converge` accept only `--target isf`; no HDL/FSM CLI.
- `kg-bench` + `corpus-kb` green with the retained fixture set.
- mdBook builds; FSM pages removed/rewritten; ISF chapter is the canonical
  adapter chapter.
- README / ROADMAP / INTENTIR_SPEC / USER_GUIDE / LIVE_ACHIEVEMENT_STATUS /
  RUST_CODEBASE_ANALYSIS / MEMORY reflect the ISF-only adapter strategy.
- Every leaf committed through `COMMIT.md`; `scripts/run_ci.sh` green.

## Task Tree

- ID: `ISF-ONLY-CONSOLIDATION`
  Status: `done`
  Goal: `SpecForge emits only .isf; HDL and .fsm fully removed.`
  Children: `.1`, `.2`, `.3`, `.4`, `.5`, `.6`, `.7`

- ID: `ISF-ONLY-CONSOLIDATION.1`
  Status: `done`
  Goal: >
    Adopt and record the ISF-only adapter strategy in the canonical
    docs before code changes: README objective, INTENTIR_SPEC adapter
    section, ROADMAP R6, USER_GUIDE, docs/FSMGEN_FEEDBACK scope note;
    mark `R6-FSM-ADAPTER` `superseded` in TASK_TREE.md; record the
    decision. Docs only.
  Acceptance: `Canonical docs state .isf is the sole adapter target and FSMGen owns downstream; R6-FSM-ADAPTER marked superseded; no code change.`
  Verification: `passed` — README/INTENTIR_SPEC/ROADMAP(R6+objective+order+R7 target)/USER_GUIDE/FSMGEN_FEEDBACK now ISF-only; R6-FSM-ADAPTER superseded in TASK_TREE.md + file; docs-only (no .rs changed)
  Commit: `ISF-ONLY-CONSOLIDATION.1 — adopt ISF-only adapter strategy in canonical docs`

- ID: `ISF-ONLY-CONSOLIDATION.2`
  Status: `done`
  Goal: >
    RE-SCOPED 2026-05-18 (user decision: keep ISF in `adapters.rs`,
    delete FSM in place — no physical module move). Make the 3 ISF
    adapter tests self-contained so the FSM bulk and FSM-only test
    helpers can be deleted in `.4` without breaking ISF: drop their
    dependence on the FSM-only fixtures `build_explicit_symbolic_dt_intent_ir`
    and `build_explicit_fsm_intent_ir`, routing them through the generic
    `build_intent_ir_from_markdown` pipeline helper, relocated next to the
    ISF tests so `.4`'s deletion boundary is unambiguous.
  Acceptance: `The 3 ISF tests depend only on the generic markdown→IntentIR pipeline helper (no FSM-only fixtures); all 3 pass incl. fsmgen --strict --check --json; scripts/run_ci.sh green.`
  Verification: `passed` — 3 ISF tests green via generic pipeline + self-contained spec, fsmgen strict green; full scripts/run_ci.sh green; test-only change
  Commit: `ISF-ONLY-CONSOLIDATION.2 — decouple ISF tests from FSM-only fixtures`

- ID: `ISF-ONLY-CONSOLIDATION.3`
  Status: `done`
  Goal: >
    RE-SCOPED 2026-05-18: HDL-only removal. Drop
    `AdapterTarget::{SystemVerilog,Verilog,Vhdl}` and the matching
    `AdapterTargetArg` variants + their `Err(FeatureNotYetImplemented)`
    dispatch arms in `cli.rs` / `adapters.rs`. This compiles independently
    of the FSM removal (nothing else references the HDL variants), so it
    is a clean small slice ahead of the atomic FSM removal.
  Acceptance: `No SystemVerilog/Verilog/Vhdl variants remain; .fsm/.isf still build; scripts/run_ci.sh green.`
  Verification: `passed` — no HDL refs remain; crate compiles standalone; full scripts/run_ci.sh green
  Commit: `ISF-ONLY-CONSOLIDATION.3 — remove HDL (SystemVerilog/Verilog/VHDL) adapter surface`

- ID: `ISF-ONLY-CONSOLIDATION.4`
  Status: `done`
  Goal: >
    RE-SCOPED 2026-05-18: the atomic FSM removal. `AdapterTarget::Fsm`,
    `AdapterArtifact.fsm`, `FsmAdapterArtifact`, `build_fsm_adapter_artifact`
    + the FSM lowering bulk, all FSM `#[test]` fns + FSM-only fixtures in
    `adapters.rs`, `validate_fsm_adapter` / `persist_fsm_adapter_validation`
    / `fsm_adapter_fingerprint` + dispatch, `IrStage::FsmAdapter`, and FSM
    handling in `commands/adapt.rs` / `commands/converge.rs` /
    `project_validation.rs`. These are mutually dependent (the
    `AdapterArtifact.fsm` field, the `AdapterTarget::Fsm` variant, and the
    FSM code/tests cannot be removed in separately-compiling pieces), so
    `.4` is ONE atomic compile-coherent slice — it cannot split into
    independently green sub-commits. `build_intent_ir_from_markdown` is
    generic and MUST be retained (ISF tests depend on it).
  Acceptance: `No FSM lowering/validation/test/dispatch code or AdapterTarget::Fsm/IrStage::FsmAdapter remains; ISF tests + fsmgen strict still green; scripts/run_ci.sh green.`
  Verification: `passed` — adapters.rs 28113→575; no FSM symbols remain; 1040 lib tests (−161 FSM); fsmgen strict green; full scripts/run_ci.sh green
  Commit: `ISF-ONLY-CONSOLIDATION.4 — atomic FSM removal (adapters.rs 28113->575)`

- ID: `ISF-ONLY-CONSOLIDATION.5`
  Status: `done`
  Goal: >
    Triage `test_data/kg_quality`: retire genuinely FSM-adapter-only
    fixtures, keep KG/semantic-truthfulness fixtures, and keep
    `kg-bench` + `corpus-kb` green and their tracked projections clean.
    AUDIT OUTCOME 2026-05-18: zero of the 153 fixtures reference
    `fsm` / `.fsm` / `adapter` / `renderab` / `root_kind` / `FsmAdapter`;
    `kg_bench.rs` carries no FSM-adapter assertions; `kg-bench` +
    `corpus-kb` were green in the `.4` CI with the fixtures unchanged.
    The kg_quality fixtures exercise the KG/semantic-truthfulness IR
    pipeline (`SourceIR→EvidenceIR→SemanticIR→IntentIR` + validation),
    not the (now-removed) FSM adapter — which had its own tests in
    `adapters.rs`. Nothing to retire; no fixture/code change. Audit-only.
  Acceptance: `Audit shows no FSM-adapter-specific fixtures exist; kg-bench + corpus-kb remain green (verified in .4 CI); no fixture/code change required.`
  Verification: `passed` — grep audit (0/153 FSM-referencing) + kg-bench/corpus-kb green in `.4` CI; docs-only audit record
  Commit: `ISF-ONLY-CONSOLIDATION.5 — kg fixture audit (no FSM-adapter fixtures; nothing to retire)`

- ID: `ISF-ONLY-CONSOLIDATION.6`
  Status: `done`
  Goal: >
    mdBook sweep: remove or rewrite FSM-centric pages/sections across
    `docs/book/src` (pipeline, commands, domain, quality, getting-started,
    introduction, SUMMARY); the ISF chapter becomes the canonical adapter
    chapter. `bash scripts/run_docs_ci.sh` green.
  Acceptance: `No stale .fsm user-facing content; ISF is the canonical adapter chapter; docs CI green. May split.`
  Verification: `passed` — 11 pages swept; only legit FSMGen-downstream/state-machine-extraction `.fsm` mentions remain; full scripts/run_ci.sh + mdBook green (re-run from correct cwd after a background-cwd flake)
  Commit: `ISF-ONLY-CONSOLIDATION.6 — mdBook FSM sweep (ISF-only book)`

- ID: `ISF-ONLY-CONSOLIDATION.7`
  Status: `done`
  Goal: >
    Final continuity reconcile (README/ROADMAP/INTENTIR_SPEC/
    LIVE_ACHIEVEMENT_STATUS/RUST_CODEBASE_ANALYSIS/MEMORY/CHANGES/
    DEVELOPMENT_NOTES), close the tree, push the completed batch per the
    COMMIT.md batch rule.
  Acceptance: `All live docs consistent with ISF-only; tree done; final scripts/run_ci.sh gate green; batch pushed.`
  Verification: `passed` — README/ROADMAP(R6+R15 note)/INTENTIR_SPEC/live docs reconciled to ISF-only; final scripts/run_ci.sh green; batch pushed
  Commit: `ISF-ONLY-CONSOLIDATION.7 — reconcile docs, close tree, push batch`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `ISF-ONLY-CONSOLIDATION.1` | `done` | Strategy recorded in canonical docs |
| 2 | `ISF-ONLY-CONSOLIDATION.2` | `done` | ISF tests decoupled from FSM-only fixtures |
| 3 | `ISF-ONLY-CONSOLIDATION.3` | `done` | HDL surface removed |
| 4 | `ISF-ONLY-CONSOLIDATION.4` | `done` | Atomic FSM removal complete; CI green |
| 5 | `ISF-ONLY-CONSOLIDATION.5` | `done` | Audit: no FSM-adapter fixtures; nothing to retire |
| 6 | `ISF-ONLY-CONSOLIDATION.6` | `done` | mdBook swept to ISF-only; CI green |
| 7 | `ISF-ONLY-CONSOLIDATION.7` | `done` | Docs reconciled; tree closed; batch pushed |

No executable leaves remain. Tree closed 2026-05-18.

## Decisions

- `2026-05-18`: `.isf` is SpecForge's sole adapter target; FSMGen owns
  scheduling/`.fsm`/HDL downstream. Supersedes the `.fsm`-active strategy.
- `2026-05-18`: ISF is extracted to its own module first (`.2`) so the
  FSM bulk can be deleted as a block rather than untangled in place.
- `2026-05-18`: `.3`/`.4` re-scoped. `AdapterTarget::Fsm`,
  `AdapterArtifact.fsm`, `FsmAdapterArtifact`, and the FSM code/tests are
  mutually dependent and cannot land in separately-compiling pieces, so the
  original "narrow types (`.3`) then delete bulk (`.4`)" split is infeasible
  under per-commit signoff. `.3` is re-scoped to HDL-only (compiles
  standalone); `.4` is the single atomic FSM removal (cannot split into
  independently green sub-commits — signoff requires each commit compile +
  CI green, and any partial FSM removal leaves the tree non-compiling).
- `2026-05-18` (revises the module-extraction item, user decision): do NOT
  physically move ISF. Keep ISF in `adapters.rs` and delete the FSM blocks in place;
  `adapters.rs` naturally becomes the ISF adapter file (optional rename
  deferred to `.7`). Lower churn/risk than a cross-module move on a 28K-line
  file. `.2` is therefore re-scoped to only de-coupling the ISF tests from
  FSM-only fixtures so `.4` can delete FSM test infra safely.
- `2026-05-18`: `R6-FSM-ADAPTER` stays as closed history, marked
  `superseded` by this tree (no history rewrite).
- `2026-05-18`: `subs/fsmgen` + the ISF↔FSMGen strict test are retained;
  FSMGen is the downstream `.isf` consumer.

## Open Questions

- `.4`/`.6` sizes: `adapters.rs` FSM bulk and the mdBook sweep will very
  likely each split into sub-leaves; exact decomposition decided when the
  leaf is reached, per the TASK_TREE splitting rules.
- `.5`: precise fixture classification (FSM-adapter-only vs KG-truthfulness)
  is determined by inspecting each fixture during `.5`, not pre-judged.

## Blockers

- None. (Execution authorization beyond task-tree creation is a separate
  user decision; this tree records the work regardless.)

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-18` | `ISF-ONLY-CONSOLIDATION.1` | docs-only diff audit (no `.rs` changed) | `passed` |
| `2026-05-18` | `ISF-ONLY-CONSOLIDATION.2` | 3 ISF tests + fsmgen strict; full `scripts/run_ci.sh` | `passed` |
| `2026-05-18` | `ISF-ONLY-CONSOLIDATION.3` | no HDL refs; standalone compile; full `scripts/run_ci.sh` | `passed` |
| `2026-05-18` | `ISF-ONLY-CONSOLIDATION.4` | adapters.rs 28113→575; 1040 lib tests; fsmgen strict; full `scripts/run_ci.sh` | `passed` |
| `2026-05-18` | `ISF-ONLY-CONSOLIDATION.5` | grep audit 0/153 FSM-referencing; kg-bench/corpus-kb green in `.4` CI | `passed` |
| `2026-05-18` | `ISF-ONLY-CONSOLIDATION.6` | 11 mdBook pages swept; full `scripts/run_ci.sh` + mdBook green | `passed` |
| `2026-05-18` | `ISF-ONLY-CONSOLIDATION.7` | README/ROADMAP/live docs reconciled; final `scripts/run_ci.sh` gate; batch pushed | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `ISF-ONLY-CONSOLIDATION.1` | `ISF-ONLY-CONSOLIDATION.1 — adopt ISF-only adapter strategy in canonical docs` | Docs only; ROADMAP R6 spliced (~279 FSM-criteria lines removed) |
| `ISF-ONLY-CONSOLIDATION.2` | `ISF-ONLY-CONSOLIDATION.2 — decouple ISF tests from FSM-only fixtures` | Test-only; 3 ISF tests via generic pipeline; re-scoped (in-place FSM deletion) |
| `ISF-ONLY-CONSOLIDATION.3` | `ISF-ONLY-CONSOLIDATION.3 — remove HDL (SystemVerilog/Verilog/VHDL) adapter surface` | HDL-only; compiles standalone |
| `ISF-ONLY-CONSOLIDATION.4` | `ISF-ONLY-CONSOLIDATION.4 — atomic FSM removal (adapters.rs 28113->575)` | Largest slice; −27,964 lines; 1040 lib tests |
| `ISF-ONLY-CONSOLIDATION.5` | `ISF-ONLY-CONSOLIDATION.5 — kg fixture audit (no FSM-adapter fixtures; nothing to retire)` | Audit-only; 0/153 FSM-referencing |
| `ISF-ONLY-CONSOLIDATION.6` | `ISF-ONLY-CONSOLIDATION.6 — mdBook FSM sweep (ISF-only book)` | 11 pages; generated-artifacts 521→221, actor-connectivity 326→238 |
| `ISF-ONLY-CONSOLIDATION.7` | `ISF-ONLY-CONSOLIDATION.7 — reconcile docs, close tree, push batch` | README/ROADMAP reconciled; tree closed; batch pushed |

## Changelog

- `2026-05-18`: Created task tree. Owns the user-authorized ISF-only
  consolidation: remove HDL + the entire `.fsm` adapter; SpecForge emits
  only `.isf`. Scope surveyed; decomposed into 7 leaves (`.4`/`.6`
  expected to split further during execution).
- `2026-05-18`: Closed tree. All 7 leaves done — `.1` doc strategy,
  `.2` ISF tests decoupled, `.3` HDL removal, `.4` atomic FSM removal
  (adapters.rs 28,113→575; −27,964 lines; 1040 lib tests), `.5` kg
  fixture audit (nothing to retire), `.6` mdBook sweep, `.7` reconcile
  + close + push. `.2`/`.3`/`.4` were re-scoped vs the original plan
  (keep ISF in adapters.rs, delete FSM in place; `.4` is one atomic
  non-splittable slice) — recorded in Decisions. SpecForge now emits
  only `.isf`; FSMGen owns scheduling/`.fsm`/HDL downstream;
  `subs/fsmgen` + the ISF↔FSMGen strict test retained.
