# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (the memory system) and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 30 commits; artifact cleanup ≥ every 24h).
- Durable cross-cutting facts: `docs/decisions/` (e.g. Docling CPU device 0001,
  LLM/VLM provider default 0002).

## Current state (OVERWRITE this block each update — do not append)
- latest_commit: `fe696b75` — "TEMPORAL-RULE-LTL-RENDER.2 — render temporal rules as standard LTL/MTL …; close tree" (this `AMBIGUITY-PHRASE-DETECTOR.1` design commit pending → becomes 21 ahead of pushed `d92a73e3`; **PUSH due-ish, threshold ~30**). NOTE: CI green baseline = 1223 tests. KM live (`KNOWLEDGE_MAP.md`, 4 facts/20 keys) — grep before re-deriving; card durable facts.
- active_work_unit: `AMBIGUITY-PHRASE-DETECTOR` — `.1` design DONE (grounded gap from the LITERATURE-GROUNDING backlog: flag-only weak-phrase/ambiguity detector over EvidenceIR statements → `validate` finding+metric; NASA ARM/Berry-Kamsties; modal verbs excluded; extraction-neutral). SIX trees CLOSED `2026-06-02`/`-04`.
- next_action: PNT — `AMBIGUITY-PHRASE-DETECTOR.2`: new `crates/specforge/src/ir/ambiguity.rs` (`pub mod ambiguity;` in `ir/mod.rs`): `WEAK_PHRASES: &[&str]` lexicon (NASA ARM weak phrases + "implementation-defined"/"vendor-specific"; NO modal verbs) + `weak_phrase_findings(&[ExtractedStatement]) -> Vec<WeakPhraseFinding{statement_id, phrase}>` (case-insensitive `.to_ascii_lowercase().contains`); unit tests (flagged stmt, clean stmt, case-insensitive, modal-only NOT flagged). Wire into `validate_evidence_ir` (`commands/validate.rs:2379`): summary line + `finding("evidence_ambiguous_statements", Info, …)` (cap list take(8)) + `metric("ambiguous_statements", count)`. Book note (`quality/validation.md`). KM card. Full CI (fix any validate test asserting an exact metric set). Close. Then backlog: `.isf`→PSL/SVA export (FSMGen contract first); conformal calibration; NLI/Dempster. **PUSH due-ish (21 ahead).**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none.
