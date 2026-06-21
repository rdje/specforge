# CORPUS-COVERAGE: build every ingested doc through to IntentIR/.isf + keep downstream stages non-stale

## Metadata

- Tree ID: `CORPUS-COVERAGE`
- Status: `active` (`.0` build-out + `.1` stage-staleness validator both done `2026-06-17`; `.2` re-ingest batch ACTIVE `2026-06-21` — owner re-provisioned the host-local library, so the 57 normalized-missing docs are now re-ingestable to refresh their STALE EvidenceIR with the current binary)
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
  validator, done), `.2` (host-local re-ingest batch, active)
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
- ID: `CORPUS-COVERAGE.2` · Status: `active` (`2026-06-21`) · Goal: **RAM-guarded re-ingest of the 57
  normalized-missing docs with the CURRENT binary**, now that the owner has re-provisioned the host-local
  spec library. **Provisioning (owner-chosen `2026-06-21`):** instead of copying ~150 MB of PDFs into tracked
  `corpus/` (permanent git bloat), the library is reached through a **git-ignored symlink**
  `.cache/local-references/chipdoc → <owner host-local chipdoc git repo>` (`.cache/` added to `.gitignore`;
  the owner's absolute library path is therefore never recorded in any tracked file — `[[feedback_source_pdfs_in_repo]]`
  — and tracked docs cite only the repo-relative `.cache/local-references/chipdoc/...` path). The owner confirms
  chipdoc is permanent. The 22 gold/measured docs stay copied in `corpus/` for the reproducible
  WIRE-BASED-100/eval path; this symlink serves the bulk coverage re-ingest only.
  **The substantive win (not mere "reach .isf"):** all 57 already reached IntentIR via `.0`, but their
  EvidenceIR is STALE — built before the `.10a`–`.10g` register/message-field families, the `.12a`/`.12b`
  presence records, and the `.2a`–`.2m` transaction recognition landed. Re-ingest → `evidence` → `semantic` →
  `intent` → `adapt --target isf` with the current binary surfaces all that new typed intent → more complete
  KG/IntentIR → more faithful `.isf`. **Method:** PNT, one doc per slice (smallest/highest-value AMBA &
  interconnect PROTOCOL specs first — CXS/GFB/ACC/ATP/TileLink/LPI/DTI/CHI-C2C — then the register/TRM/ISA
  docs), `DOCLING_DEVICE=cpu` (`[[project_docling_mps_cpu]]`), the built-in `.4a` RAM guard active (clean
  abort at ≥85% used), Ollama kept idle, RAM+swap monitored between docs (`[[feedback_ram_ceiling_monitor]]`),
  commit per `COMMIT.md` after each doc. No fabrication / ADR-0006 unchanged (this is a re-run of existing
  deterministic extractors, not new code); WIRE-BASED-100 + register/wire golds + `kg-bench` stay green
  (orthogonal — the 4 gold docs are not re-ingested). Record per-doc before/after typed-surface deltas here.
- Frontier (active): `CORPUS-COVERAGE.2` — re-ingest the 57 normalized-missing docs from the
  `.cache/local-references/chipdoc` symlink, prioritized protocol-specs-first, one doc per slice.

## `.2` re-ingest log (per doc — current-binary refresh; generated/ is git-ignored, so this table is the durable trace)

Columns: pages · key new typed surfaces the refresh added (vs the STALE pre-`.10`/`.12`/`.2` evidence) · `.isf` render + FSMGen `--strict --check` diagnostics.

| # | doc (key) | pages | refreshed surfaces (after) | `.isf` | fsmgen `--strict` |
|---|---|---|---|---|---|
| 1 | `ihi0079` AMBA CXS | 54 | message_field_records 0→1; transactions →2 (recognition now fires); 11 signal ports emitted (CXSDATA/CXSVALID/CXSCRDGNT/…); stale evidence had message/temporal/presence fields ABSENT | renderable (`transmitter.isf`, 11 ports) | **0 diagnostics** ✓ |
| 2 | `ihi0083` AMBA GFB | 45 | transactions →5 (recognition now fires); relations 16 (held); signal_constraints 14→7 (current stricter declared-subject gate drops ungrounded); no msg/reg fields (GFB has none — honest) | renderable (`device.isf`, 7 ports) | **0 diagnostics** ✓ |
| 3 | `ihi0076` ACC | 90 | register_records 2→4 (the `.10g` section-heading register-field family fires — ACC is one of its 5 docs); rich `.isf` 133 ports (vs stale 2-register); 0 txns/relations (register/debug-channel arch — honest) | renderable (`agent.isf`, 133 ports) | **0 diagnostics** ✓ |
| 4 | `ihi0082` ATP | 84 | transactions →5 (recognition fires); relations 11→9 — the CLEAN deterministic re-ingest drops the garbled-VLM-fragment actors the `KG-ISF-COMPLETENESS.4` census flagged (`"RREADY is RBR"`), so ATP no longer needs the owner-gated VLM lever just to get a clean baseline; reg 4 held | renderable (`rate_parameter.isf`, 1 port) | **0 diagnostics** ✓ |
| 5 | `tilelink_1_7_1` TileLink 1.7.1 | 107 | relations 39→33 (current agent-identity consolidation gates `.1a`/`.1b` — which postdate the stale evidence — fold fragment actors); renderable per-channel `.isf` (`channel_b`); transactions 0 (TileLink's Get/Put op vocabulary not yet in the section-heading recognizer — honest) | renderable (`channel_b.isf`, 5 ports) | **0 diagnostics** ✓ |
| 6 | `tilelink_1_8_0` TileLink 1.8.0 | 111 | relations 40→34 (same `.1a`/`.1b` consolidation as 1.7.1); transactions 0 (op vocabulary — honest); per-channel `.isf` | renderable (`channel_b.isf`, 6 ports) | **0 diagnostics** ✓ |
| 7 | `ihi0068` AMBA LPI | 66 | transactions →1 (recognition fires); relations 9→8; constraints 17 held | renderable (`controller.isf`, 5 ports) | **1** — `isf_conflicting_rule_writes` on `PREQ` (overlapping rule writes pick different values, `rule_5`): the `ISF-RULE-CONFLICT-RESIDUAL` family (same as pre-existing AHB `HAUSER`/AXI `ASKSTOP`); honest residual, NOT introduced by this slice (emitter unchanged) → candidate for that tree |
| 8 | `ihi0088` AMBA DTI | 146 | **`message_field_records` 0→159 / 17 containers** (the `.10f` section-heading message-field family fires — the marquee re-ingest win; totally ABSENT in stale evidence); transactions →5; constraints 30→16 (stricter gate) | renderable (`channel.isf`, 294 ports) | **1 ERROR (strict FAIL)** — FSMGen HDL-gen: `assignment to 'ATST' uses RHS 2'b1 (width 2) for LHS width 1` (implicit truncation blocked). A real SpecForge ISF **width-alignment** emitter bug (emits a width-2 value to a 1-bit signal), EXPOSED not caused by this slice (emitter unchanged) → **spun-out CODE lever** (see below) |

| 9 | `ihi0098_b` AMBA CHI-C2C | 291 | **`message_field_records` 0→210 / 21 containers** (`.10b`/`.10d` families); register_records 76→81 (`.10g`); transactions →3; relations 4→0 (honest — coherency/packet protocol carries intent in fields/registers, not wire relations, per `KG-ISF-COMPLETENESS.3`) | renderable (`agent.isf`, 109 ports) | **0 diagnostics** ✓ (no width-mismatch → DTI's `ATST` bug is value-specific, not universal) |

**Spun-out finding (`.2` → future CODE lever) — ISF emitter width-alignment:** DTI's rich 294-port `.isf` (from the 159 `.10f` message fields) fails FSMGen `--strict --check` because a rule/drive lowers a width-2 literal (`2'b1`) onto the 1-bit signal `ATST` — the emitter does not width-align the emitted value to the declared signal width (FSMGen's OperandContract blocks implicit truncation). This is a genuine ISF lowering-fidelity bug (north-star bar #6), surfaced by re-ingest. It needs its OWN owned leaf (likely under the `ISF-*-EMIT` family), measurement-first, with WIRE-BASED-100 + register/wire golds + `kg-bench` gating — NOT done inside a re-ingest slice. Watch whether other enum/message-heavy docs (CHI-C2C, CCIX, NVMe-class) hit the same width-mismatch as the batch proceeds (strengthens the case + scopes the fix).

## Changelog

- `2026-06-21`: `.2` re-ingest batch OWNED + provisioning set up. Owner re-provisioned the host-local spec
  library (`chipdoc`, 88 PDFs); chosen mechanism = a git-ignored symlink `.cache/local-references/chipdoc`
  (no `corpus/` copy → no git bloat; absolute library path never tracked). `/.cache/` added to `.gitignore`.
  Frontier now ACTIVE (was standing/blocked): RAM-guarded per-doc re-ingest with the current binary to refresh
  the 57 docs' STALE EvidenceIR (unlock `.10`/`.12`/`.2` extractor families) and cascade to `.isf`,
  protocol-specs-first. Ownership slice — no extraction code change.
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
