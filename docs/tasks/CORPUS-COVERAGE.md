# CORPUS-COVERAGE: build every ingested doc through to IntentIR/.isf + keep downstream stages non-stale

## Metadata

- Tree ID: `CORPUS-COVERAGE`
- Status: `active` (`.0` build-out + `.1` stage-staleness validator both done `2026-06-17`; standing frontier = re-ingest the 57 normalized-missing docs, RAM-gated/host-local-source)
- Roadmap lane: `R15e`/`R16` (corpus digestion — the owner's substantive gap #2)
- Created: `2026-06-17`
- Owner directive: `2026-06-17` — after the owner rejected the "buildable frontier exhausted" framing
  (SpecForge is NOT complete, `[[feedback_not_complete_attack_substantive_gaps]]`), attack three substantive
  north-star gaps; this tree owns **#2 corpus coverage** ("only 36 of 79 ingested docs reach IntentIR/.isf").

## The point

A document that stops at EvidenceIR contributes nothing to the canonical IntentIR/.isf surface the whole
tool exists to produce. The pre-existing local corpus had **only 36 of 79** ingested docs carried through to
IntentIR — the other 42 sat at evidence-only (operational: a sweep rebuilt evidence without cascading
downstream; the per-stage commands do not auto-cascade, only `converge` rebuilds the whole chain). Building
semantic→intent needs only the already-persisted `evidence_ir.json`, **not** the heavyweight `normalized/`
bundle, so the evidence-only docs are cheap-buildable with no re-ingest.

## `.0` — corpus build-out + coverage census (DONE `2026-06-17`)

Read-only census + a deterministic, RAM-safe build-out (release binary; no LLM, no Docling).

**Census (before):** of 79 ingested source docs — evidence **78**, semantic/intent/isf **36/36/36**; 42
evidence-only; 3 stale-intent (`tilelink_1_8_0` ev 40/int 0, `um10204` i2c 17/0, `wbspec` 1/0 — the
`KG-ISF-COMPLETENESS.3` staleness class); 57 of 79 lack the `normalized/` bundle.

**Build-out:** rebuilt the 42 evidence-only + 3 stale docs `semantic`→`intent`→`adapt --target isf`
(excluding the 4 WIRE-BASED-100 gold docs, which already build and are gated). **Result: 42/42 build OK, 0
failures**, RAM steady 77% throughout.

**Census (after):** evidence **78**, semantic **78**, intent **78**, isf **75**. **0 stale-intent
remaining**; the 3 stale docs recovered their relations (tilelink_1_8_0 → 40, i2c → 17, wbspec → 1). The 3
intent-without-isf docs (`risc_v_debug`, coresight `den0068` BSA, GIC arch `ihi0069_g`) **block honestly**
(`adapt` reports `no behavioral content (temporal/conditional rules, signal constraints, or control
blocks)`) — register/architecture docs with nothing behavioral to lower, not a failure. Intent relation
distribution across 78: 30 at 0 (the honest-absence register/guide class from `KG-ISF-COMPLETENESS.3`), 18 at
1–10, 17 at 11–50, 13 at 50+.

**Finding:** the staged pipeline is now validated end-to-end on the **entire** local corpus (78/78 to intent,
0 errors). The generated tree is git-ignored local cache, so the durable deliverables are this measurement +
the two follow-ups below. The 57 docs lacking a `normalized/` bundle cannot have their EVIDENCE rebuilt
without re-ingest (Docling + source PDF, RAM-gated) — a standing frontier gated on host-local source
re-provisioning (`[[feedback_source_pdfs_in_repo]]`).

## Task Tree

- ID: `CORPUS-COVERAGE` · Status: `active` · Children: `.0` (build-out + census, done), `.1` (stage-staleness
  validator, next)
- ID: `CORPUS-COVERAGE.0` · Status: `done` (`2026-06-17`) · Goal: build every evidence-only doc through to
  IntentIR/.isf and census the result. Done: 36→78 intent / 36→75 isf, 0 build failures, 0 stale remaining,
  3 isf honest-blocks. Verification above.
- ID: `CORPUS-COVERAGE.1` · Status: `done` (`2026-06-17`, CODE) · Goal: a generic **stage-staleness
  detector** in `validate` so a downstream artifact silently dropping intent (the `tilelink` 39→0 relation
  class) is SURFACED, not hidden — directly serves "the KG must be COMPLETE." **DONE:** `validate
  <intent-ir>` / `<semantic-ir>` now loads the upstream artifact (via the carried `semantic_ir_path` /
  `evidence_ir_path` — `validate` already does this for graph-aware findings) and emits a `stage_staleness`
  **Warning** (`intent_stale_relations_dropped` / `semantic_stale_relations_dropped`) when the downstream
  carries 0 `actor_signal_relations` while the upstream carries some. **False-positive-free** because the
  agent-identity gates (consolidation/split/phantom-drop) NEVER empty a non-empty relation set — a 0-vs-N
  split is staleness, not gating; the I/O is paid only when the downstream is empty (a `let`-chain
  short-circuit), and is skipped when the upstream is not on disk (detached copy). Pure decision helper
  `stage_staleness_relation_finding` (+3 unit tests: 0-vs-39 fires, 39/17-vs-N silent, 0-vs-0 honest-absence
  silent). **Verified live** (release binary, temp-CWD to avoid the WRITE-PATH GOTCHA): POSITIVE fires on a
  synthetic stale tilelink (0 vs real semantic 39), NEGATIVE silent on `nvme` (0-vs-0 honest absence — the
  critical no-false-positive case) and on healthy tilelink (39). ADR-0006 (universal/structural, no name
  list). WIRE-BASED-100 unaffected by construction (validate-only additive finding; wire docs carry non-empty
  relations → silent; extraction/IR content untouched); `run_ci.sh` GREEN, lib 1660 passed (+3); `kg-bench`
  156/156. Book `quality/validation.md`; KM `[[stage-staleness-validate-detector]]`.
- Frontier (standing): re-ingest the 57 `normalized/`-missing docs (Docling + source PDF, RAM-gated,
  host-local source re-provisioning) to enable their EVIDENCE rebuild.

## Changelog

- `2026-06-17`: `.1` stage-staleness validator DONE (CODE). `validate <intent-ir>`/`<semantic-ir>` now emits a
  `stage_staleness` Warning when the downstream carries 0 `actor_signal_relations` while its upstream (loaded
  via the carried path) carries some — false-positive-free (gating never empties a non-empty set), I/O paid
  only when empty (`let`-chain), skipped when the upstream is off-disk. Pure helper +3 unit tests; live-verified
  (positive fires, nvme/healthy silent); ADR-0006; `run_ci.sh` green (lib 1660, +3); `kg-bench` 156/156;
  WIRE-BASED-100 unaffected by construction. Book `quality/validation.md`; KM `stage-staleness-validate-detector`.
- `2026-06-17`: Created on the owner's substantive-gap-#2 directive. `.0` build-out + census DONE — corpus
  IntentIR coverage 36→78 (42 evidence-only docs + 3 stale built, 0 failures, 0 stale remaining; 75/78 isf
  with 3 honest behavioral-content blocks); RAM steady 77%; deterministic, no LLM/Docling. Frontier → `.1`
  stage-staleness `validate` detector + the gated re-ingest of the 57 normalized-missing docs.
