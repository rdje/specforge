# EVIDENCE-MATERIALIZE-IDEMPOTENCY: don't accumulate stale facts on re-build

## Metadata

- Tree ID: `EVIDENCE-MATERIALIZE-IDEMPOTENCY`
- Status: `done` (CLOSED `2026-06-06`; `.1`)
- Roadmap lane: `R16`/`R15e` (extraction infra)
- Created: `2026-06-06`
- Parent context: backlog Actionable/discrete follow-up #2. `CONSTRAINT-EXTRACTION-V2.4` root-caused a
  scary re-measurement (13 → 21 constraints with duplicate ids) to the `evidence` command's
  **materialize-merge** (`carry_forward_existing_knowledge` → `merge_signal_constraints`): re-running
  over a stale on-disk artifact carried its old deterministic facts forward.

## Fix

The merge exists to **preserve LLM enrichments** (`nlp_enrich`/`signal-resolve`, tagged
`ExtractorTier::Nlp`) across re-builds. So: in `carry_forward_existing_knowledge`, if the existing
artifact carries **no Nlp-tier facts**, the fresh deterministic build fully supersedes it — return
early (skip the whole carry-forward). That makes the common re-measurement case (a pure-deterministic
artifact, re-built after an extractor change) **idempotent**, while a genuinely-enriched artifact
still merges to keep its LLM facts.

(Scope note: an artifact that mixes LLM facts AND stale deterministic facts would still carry the
stale deterministic ones — a full provenance-keyed merge, carrying forward *only* Nlp-tagged keys, is
the further refinement. Deferred; the observed/common gotcha is the pure-deterministic case, now fixed.)

## Verification

Passed (`2026-06-06`) — +1 integration test (`carry_forward_skips_a_pure_deterministic_existing_artifact`:
build → inject a stale constraint + persist (no Nlp provenance) → re-build → the stale constraint is
NOT carried forward). Full `scripts/run_ci.sh` GREEN (1274→1275). No regression (the genuinely-enriched
merge path is unchanged).

## Task Tree

- ID: `EVIDENCE-MATERIALIZE-IDEMPOTENCY` · Status: `done` · Children: `.1`
- ID: `EVIDENCE-MATERIALIZE-IDEMPOTENCY.1` · Status: `done` · Goal: skip carry-forward when the
  existing artifact has no LLM facts. Verification above.

## Changelog

- `2026-06-06`: Created + CLOSED — idempotency guard in `carry_forward_existing_knowledge`. (Backlog
  Actionable/discrete #2.)
