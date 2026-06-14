# LLM-PRIMARY-PROMOTION: promote the LLM-primary constraint surface into the canonical pipeline

## Metadata

- Tree ID: `LLM-PRIMARY-PROMOTION`
- Status: `active` (`.1`–`.4` DONE `2026-06-10`; **owner AUTHORIZED the FLIP `2026-06-14`** —
  `.5` is un-gated and ready to execute, deferred to a fresh session for signoff sharpness;
  R1–R4 lever candidates recorded in the packet)
- Roadmap lane: `R15e`/`R16` (extraction quality / production-readiness)
- Created: `2026-06-10`
- Parent context: `EXTRACTION-QUALITY-GAUGE.0`'s standing gauge made the gap VISIBLE: the
  canonical persisted artifacts still carry the **Pattern** constraint surface — APB 4/14
  not-entailed (28.6%) but AHB 9/15 (60%), degraded CHI 9/13 (69%), AXI 91/100 (91%) all
  majority-erroneous — while the cleaned **LLM-primary** surface (`extract-constraints-llm`,
  built `.5`→`.8`→`.3a`→`.3b`→`.4`: typed subjects, grounded conditions, condition-only-subject
  gate, permissive-frame gate, value recovery, provenance-merging dedup) measured
  **P=R=F1=1.000 ×3 + 16/16 doc-level gold recall** on the wire docs — but only ever on /tmp
  redirected measurement copies. Promotion = making the clean surface the one the canonical
  pipeline actually carries, without breaking anything the Pattern surface currently feeds.

## `.1` design (probed `2026-06-10`, docs-only)

### Probed integration facts (all verified in-code this session)

1. **Converge's monotone invariant forbids an in-loop replace.** `KnowledgeSnapshot` counts
   `EvidenceSnapshot.signal_constraints` into `total_fact_count`, and `run_convergence` errors
   on any pass-to-pass shrink ("pipeline knowledge shrank"). Replacing AXI's 102 Pattern
   records with ~50 clean ones inside the loop trips the guard (and replace-vs-rebuild would
   oscillate against `nlp-enrich`, which re-adds per pass). → Promotion must run
   **post-stability**, like the rescan step and the quality gauge.
2. **Downstream consumers of `signal_constraints`** (must be rebuilt + re-verified after a
   replace): `SemanticIr::build` carries the surface (declared-signal filtered) AND feeds it to
   the typed temporal layer; `IntentIr` carries it onward; the ISF adapter renders from it;
   `learn-priors` harvests temporal phrase priors from it (advisory plane);
   `corpus_cluster`/`CorpusMemory` fingerprints bucket its count (advisory). `validate` and the
   NLI gauge measure it.
3. **The LLM-primary extractor's recall universe is the Pattern surface's sentence set** —
   `extract-constraints-llm` iterates the distinct `source_text` of the EXISTING constraints
   (one LLM call per sentence). Promotion is therefore a *refinement* of what Pattern found,
   not a new discovery pass: doc-level recall within that universe measured 16/16, but a
   sentence Pattern never captured stays uncaptured (honest scope bound; widening the sentence
   universe is future work, NOT this tree).
4. **Provider-off must stay Pattern.** `extract-constraints-llm --provider skip` is a strict
   no-op; kg-bench fixtures and CI run provider-free and keep building the Pattern surface.
   Promotion only ever happens when a text provider actually ran — the deterministic
   provider-free pipeline is untouched by construction.
5. **The gauge must measure the PROMOTED surface**: converge ordering becomes
   stable → rescan step → (optional) promote + downstream rebuild → extraction-quality gauge →
   summary. (`EXTRACTION-QUALITY-GAUGE.0` deliberately placed the gauge last; promotion slots
   in before it.)

### Decision (recorded)

- **Opt-in first, default later.** `.2` ships promotion behind an explicit converge flag
  (e.g. `--promote-constraints-llm`, default OFF), printing the gauge so the before/after is
  measured per document. Flipping the default is a separate later decision (own slice, owner
  visibility) once the corpus sweep (`.4`) shows the gauges improving and the gold gates
  holding — consistent with the repo's promotion-review culture: favorable deltas are
  demonstrated, never assumed.
- Promotion is **manifest-recorded** (a `constraints.llm_primary` extractor entry), so the
  fingerprint/`corpus-cluster` plane sees which documents carry the promoted surface — no
  silent surface swaps.
- The replaced surface keeps `message_field_constraints` routing exactly as
  `EXTRACTION-QUALITY-GAUGE.FIELD.4` shipped it (field obligations land field-scoped).

### Gold gates (every slice must keep these green)

- Wire-doc eval on the PROMOTED canonical artifacts (not /tmp copies): P=R=F1=1.000 ×3 +
  16/16 doc-level recall must hold end-to-end.
- WIRE-BASED-100 (constraints/relations/temporal) stays 100% — relations are untouched by
  construction; the temporal layer is re-verified after the Semantic rebuild.
- Serial-class docs (SWD/CAN/SWP/SMBus/I2S): no regression (their gold covers
  frame/operations/FSM/timing — re-run to prove no side effects).
- kg-bench full pass; provider-free CI byte-stable (promotion never fires without a provider).

## Task Tree

- ID: `LLM-PRIMARY-PROMOTION` · Status: `active, awaiting owner decision` · Children: `.1`–`.5`
- ID: `LLM-PRIMARY-PROMOTION.1` · Status: `done` (`2026-06-10`, design + in-code probes,
  docs-only) · Goal: ground the promotion design in the actual converge invariant, downstream
  consumer set, recall universe, and provider-off semantics BEFORE coding (probe-first method).
  Findings + decisions recorded above.
- ID: `LLM-PRIMARY-PROMOTION.2` · Status: `done` (`2026-06-10`) · Goal: the opt-in converge
  promotion stage — post-stability `extract-constraints-llm` over the final EvidenceIR
  (flag-gated, manifest-recorded), one downstream rebuild (Semantic → Intent → adapter), gauge
  measured AFTER promotion, summary reports both the promotion delta and the gauge.
  **Shipped:** `promote_constraints` extracted as the shared core in
  `commands/extract_constraints_llm.rs` (the command is now a thin printing wrapper; behavior
  identical) + TWO new surface effects on every replace: manifest-recorded
  `constraints.llm_primary` (via new `ExtractionManifest::record_surface_manifest`, same
  replace-per-surface semantics as `record` — test-locked) and the persisted NLI gauge dropped
  (its measured ids are definitively gone — test-locked). `ConvergeArgs.promote_constraints_llm`
  (default OFF); flag + `--nlp-provider skip` = explicit EARLY error (an opt-in that silently
  does nothing is worse — test-locked); `maybe_promote_constraints` at the stable branch AFTER
  the rescan step and BEFORE the gauge (the standing gauge measures the PROMOTED surface);
  summary prints `constraint_promotion:` delta. +3 tests (lib 1537→1540).
  **Verification (live, `2026-06-10`, APB end-to-end** `converge --vlm-provider skip
  --nlp-provider ollama --promote-constraints-llm` on the git-tracked corpus PDF,
  DOCLING_DEVICE=cpu): stabilized in 2 passes (Pattern+NLP3 = 18 constraints) → promotion
  **18 → 21 grounded → 21 kept** over 15 distinct sentences, 0 field constraints → downstream
  rebuilt → gauge on the promoted surface **5/21 not-entailed (23.8%)** vs 28.6% on the old
  Pattern artifact. HONEST FINDING: on APB the promoted surface GROWS (21 > 18) — the `.8`
  validity-requirement recovery reads facts the pattern grammar mis-read; the shrink shape
  (102→~50) is the dense-spec class, and the monotone-guard placement argument holds
  regardless of direction. **Gold gate HELD on the promoted CANONICAL artifact:**
  `eval-extraction seed_apb --provider skip` → signal_constraint **P=R=F1=1.000** (6/6),
  WIRE-BASED-100 filtered relations **1.000**, doc-level fact recall **6/6**, conformal
  empirical_error **0.000**. Book: converge flags row + "Promoting the LLM-primary constraint
  surface" subsection (real output transcript) + `extract-constraints-llm` section updated
  (no longer research-only; manifest + gauge-drop semantics); README bullet. KM
  [[llm-primary-promotion-stage]]. AHB/AXI + serial no-regression sweep = `.3` as planned.
- ID: `LLM-PRIMARY-PROMOTION.3` · Status: `done` (`2026-06-10`) · Goal: wire-doc end-to-end
  verification on promoted canonical artifacts — eval 1.000 + 16/16 held, temporal/relations
  re-scored (WIRE-BASED-100 intact), serial-class no-regression sweep, gauge before/after
  recorded per document. **HONEST VERDICT: the gates CLEARED APB+AHB and CAUGHT a real
  promotion recall defect on AXI — exactly what gold gates exist for. AXI was REVERTED.**
  - Promoted the persisted CANONICAL AHB/AXI artifacts via the standalone command (same
    shared `promote_constraints` core the converge stage uses; converge integration itself was
    `.2`-proven end-to-end on APB): AHB 15→12 kept and AXI 102→54→50 kept — **reproducing the
    earlier /tmp redirected-copy measurements EXACTLY** (`.3b`: 15→12; `.4`: 54→50 w/ 4
    merged). Gauges on promoted surfaces: **AHB 60.0% → 33.3%** not-entailed, **AXI 91.0% →
    36.0%**.
  - **Gold gates:** seed_ahb P=R=F1=1.000 (6/6) + filtered relations 1.000 + doc recall 6/6 ✓;
    seed_axi constraints 1.000 (4/4) + filtered relations 1.000 ✓; seed_ahb_temporal 1.000
    (4/4) ✓; seed_apb_temporal 1.000 (3/3) ✓; seed_swd 1.000 (untouched serial control) ✓;
    **seed_axi_temporal FAILED on the promoted artifact: 1/3** — the SYSCOREQ/SYSCOACK
    reset-condition temporal rules lost.
  - **Per-item root cause (probed live, temp 0, ×2 identical):** the source sentence is the
    coordinated-subject "SYSCOREQ and SYSCOACK must be deasserted when ARESETn is asserted.";
    the model emits BOTH records but **misspells the first subject `SYCOREQ`** (one `S`
    dropped); entity typing then correctly rejects the undeclared token — the grounding gate
    worked as designed (no fabricated subject), recall lost to a one-character model typo.
    The surviving SYSCOACK record is correct. NEW defect class: **model-misspelled subject on
    an otherwise-grounded proposal** → fix leaf `.3a`.
  - **Revert executed + verified:** AXI evidence rebuilt from persisted SourceIR (zero Nlp
    fact-provenance → clean full supersede), semantic+intent rebuilt; seed_axi 1.000 AND
    seed_axi_temporal 1.000 (3/3) RESTORED — also confirming causality (the dropped SYSCOREQ
    constraint fed the temporal rules). Gauge re-measured on the restored Pattern surface so
    the standing report stays honest. APB + AHB keep their promoted surfaces (all their gates
    green).
- ID: `LLM-PRIMARY-PROMOTION.3a` · Status: `done` (`2026-06-10`; CODE + live proof of the
  snap; the gate battery then exposed a SECOND, separate gap → `.3b`) · Goal:
  **model-misspelled-subject recovery** — a deterministic, document-grounded backstop.
  **ROOT-CAUSE CORRECTION (supersedes the `.3` reading "entity typing rightly rejects"):**
  production typing DEFERS to the LLM judge for an undeclared non-structural token, so the
  `SYCOREQ` typo *typed as Signal and grounded* — a phantom-name record that died silently at
  the SemanticIR declared-signal filter. The per-item audit killed the first cut AGAIN: a
  snap hooked on *typing failure* never fires in production (green unit tests, failed live
  gate); the shipped trigger is **absence-from-sentence** — this extractor's subjects are
  quotes, so a proposed subject with zero identifier-boundary occurrences in its own source
  sentence is suspect per se. Shipped (`ir/constraint_extract_llm.rs`):
  `snap_subject_to_sentence_token` (+ `is_snap_candidate_token` ≥4 chars/leading-upper/≤1
  lowercase; `within_one_edit_ignore_case`; exactly ONE candidate that types as Signal/Field
  or no snap) hooked in `ground_constraint_typed` BEFORE typing; a subject occurring in its
  sentence is never rewritten. +5 tests incl. the deferring-judge production-shape regression
  (lib 1545); fmt/clippy/rustdoc green. **Live proof (clean protocol: Pattern baseline
  restored → promote): the promoted AXI artifact now carries `SYSCOREQ must_be_deasserted
  when ARESETn is asserted` under the DOCUMENT's spelling** (pre-fix: the `SYCOREQ` phantom).
  **But the temporal gate STILL fails (tp=1 fp=2): both gold reset rules are missed — INCLUDING
  the always-correctly-spelled SYSCOACK — so the residual gap is NOT the typo: the SemanticIR
  temporal derivation does not reproduce the gold rule shape (`MANAGER/SUBORDINATE drives
  SYSCO* → LOW post_tick of rising ARESETN ASSERTED`) from the LLM-primary record shape,
  while it did from the Pattern surface.** → `.3b`. AXI REVERTED again (Pattern baseline;
  both gates re-verified 1.000; 102 constraints). Measurement-protocol lesson recorded: an
  already-promoted artifact is NOT a valid promotion input — always restore the Pattern
  baseline first (one invalid intermediate run caught + discarded).
- ID: `LLM-PRIMARY-PROMOTION.3b` · Status: `done` (`2026-06-10`, CODE + live battery) · Goal:
  **temporal-derivation parity for promoted constraints** — probe-first per-item diff, then fix
  the honestly deficient layer.
  **Probe verdict (all three candidate factors checked, one guilty):** antecedent recovery has
  parity (`parse_temporal_condition_predicates` strips a leading `when ` — both surfaces parse
  "ARESETn is asserted" → `ARESETN ASSERTED pre_tick`); actor grounding has parity (the
  `Manager`/`Subordinate` drive predicates come from `unique_producer_by_signal` over
  `signal_connectivity`, untouched by promotion); **the guilty factor is consequent value
  collapse: the build path polarity-refines `MustBeDeasserted`→`MustBeLow` via
  `apply_signal_polarity_to_constraints` (SYSCOREQ/SYSCOACK are persisted `active_high` from
  `table_0238`) BEFORE persisting, while the post-build promotion replace skipped that
  refinement entirely — so the promoted shape fed the temporal layer a symbolic `DEASSERTED`
  the gold (and the rest of the pipeline) knows as `LOW`.** Synthetic probe demonstrated the
  chain deterministically: a /tmp evidence copy with ONLY the two reset kinds set to
  `must_be_deasserted` derives rules identical to gold except value `DEASSERTED`≠`LOW`; the
  canonical Pattern artifact derives `LOW`. The temporal layer is doctrine-correct (symbolic
  stays symbolic when polarity is ungrounded) — the deficiency was the broken build-path
  INVARIANT at the replace point, which also skewed every other kind consumer (NLI gauge claim
  text, ISF adapter).
  **Shipped:** `evidence::apply_persisted_polarity_to_constraints` (`pub(crate)` wrapper:
  resolved map from persisted `signal_polarities` records → the same build-path refinement) +
  the call in `promote_constraints` BEFORE dedup (the canonical dedup key sees the refined
  kind). +1 record-matrix test (deasserted+AH→LOW, deasserted+AL→HIGH, asserted+AL→LOW,
  ungrounded stays symbolic, value-kinds untouched); lib 1546. APB/AHB promoted artifacts
  scanned: ZERO latent unrefined-with-grounded-polarity records — no re-promotion needed.
  **Live battery (clean protocol: AXI verified at Pattern baseline first):** promotion
  102→54→50 (reproducing `.3`/`.4` exactly); promoted records carry `SYSCOREQ`/`SYSCOACK`
  `must_be_low` ("when ARESETn is asserted", statement_4690); downstream rebuilt;
  **seed_axi_temporal 3/3 P=R=F1=1.000 on the promoted CANONICAL artifact — the `.4` unblock.**
  Full battery green: seed_axi 4/4 + filtered relations 1.000; seed_apb 6/6 / seed_ahb 6/6 +
  filtered relations 1.000; seed_apb_temporal 3/3; seed_ahb_temporal 4/4; seed_swd 1.000 ×4
  surfaces; seed_i2c filtered 1.000; kg-bench 154/154. Gauge re-measured + persisted on the
  promoted surface: 24/50 not-entailed (48.0%) vs 91.0% Pattern. HONEST gauge note: the
  refinement trades NLI-gauge optics for gold-gate correctness — the NLI judge marks
  "SYSCOACK must be LOW" not-entailed against source "must be deasserted" because it lacks the
  document's polarity grounding; the per-item-verified gold gates outrank the heuristic gauge
  (`feedback_scoring_rigor`). PRE-EXISTING residual observed (NOT this slice, untouched
  artifacts + untouched eval code): seed_nvme/seed_riscv register evals read 0 because the
  strict register tuple has never matched — 0.000 at gold-authoring time BY DOCUMENTED DESIGN
  (`PDF-VARIANT-DIGESTION.4a.2`/`.4a.3`: mnemonic-vs-long-name representation; graphic-borne
  bits), with the honest measurement in the dedicated register-field views, which have since
  IMPROVED (NVMe field-name recall 28/29 = 0.966, 42/42 real register names, 201/201 bit
  extents). CORRECTED `2026-06-10` — the first reading ("`.3c` shifted statement-id anchors")
  was an aggregate-only misread of the battery output; nothing regressed.
  **Canonical state: APB + AHB + AXI all PROMOTED with all gates green.**
- ID: `LLM-PRIMARY-PROMOTION.4` · Status: `done` (`2026-06-10`, measurements + docs — no code)
  · Goal: corpus sweep + tracked validation snapshot refresh + the default-flip decision
  packet. **The packet is below (§ Default-flip decision packet); the flip itself is the
  owner's call (`.5`).** Sweep protocol: canonical artifacts received only the standing
  Pattern-gauge measurement (`nli-verify`, the EXTRACTION-QUALITY-GAUGE.0 design); promotion
  ran exclusively on REDIRECTED /tmp copies (`artifact_layout` patched) — zero canonical
  mutation outside the three gold-gated wire docs. 12 docs swept across classes (dense/mid
  AMBA, register, memory, capability, serial, TRM, tiny-surface). Tracked snapshot refreshed
  (`project-validation` over the four wire IntentIRs) — the prior snapshot was TWO MONTHS
  stale (2026-04-10); composite scores dropped (e.g. untouched AXI-Stream 90→67) because the
  CURRENT validator sees far more surfaces than April's — validator-version drift, proven by
  the untouched-artifact control, NOT a promotion effect; scores are not comparable across
  validator versions.

## Default-flip decision packet (`.4`, owner-visible — decision pending at `.5`)

**Question:** should `converge --promote-constraints-llm` become the DEFAULT for live-NLP
converge runs (provider-free runs always stay Pattern by construction)?

**Per-doc gauge evidence (Pattern → promoted, % = not-entailed of labeled; records before→after):**

| Doc (class) | Pattern | Promoted | Records |
| --- | --- | --- | --- |
| AXI L (wire, gold-gated, CANONICAL) | 91.0% | 48.0% | 102→50 |
| APB E (wire, gold-gated, CANONICAL) | 28.6% | 23.8% | 18→21 |
| AHB C (wire, gold-gated, CANONICAL) | 60.0% | 33.3% | 15→12 |
| DTI (dense AMBA) | 99.1% | 73.3% | 114→30 |
| AXI+ACE H.c (dense AMBA) | 80.9% | 44.4% | 94→63 |
| LTI (mid AMBA) | 90.2% | 65.7% | 41→35 |
| Low-power IF (mid AMBA) | 86.7% | 52.9% | 30→17 |
| GFB (mid AMBA) | 80.0% | 64.3% | 20→14 |
| CHI G (degraded ingest) | 69.2% | 16.7% | 13→6 |
| APB D (older version) | 30.8% | 21.1% | 13→19 |
| AXI-Stream (wire) | 100% | 87.5% | 13→8 |
| OpenCAPI 4.0 (capability) | 100% | 25.0% | 15→8 |
| HBM2 (memory) | 85.7% | 40.0% | 14→20 |
| NVMe 2.0a (register) | 88.5% | 50.0% | 26→2 |
| I2C (serial) | 72.7% | 0.0% | 11→2 |
| I2S (serial, 1 known-bad record) | 100% | n/a (0 records) | 1→0 |
| CoreSight SoC-600 0701 (TRM) | 50.0% | 100% | 2→2 |

The gauge improves on **14 of 15** measurable docs. The two apparent counterexamples dissolve
per-item: I2S's single Pattern record was a genuine flagged mis-extraction (`SCK`) whose drop
is correct; CoreSight's promoted records are BETTER-attributed (Pattern blamed `ATB` for facts
about `araddr_m`/`awaddr_m` — promotion fixes the subjects) and the 2/2 flag is the judge
correctly catching a granularity overstatement (source says "some of the LOWER BITS are tied
LOW"; the constraint vocabulary has no bit-subrange slot — residual R4 below).

**Gold-gate status:** all three gated wire docs promoted on canonical artifacts with
constraints P=R=F1=1.000, filtered relations 1.000, temporal 3/3+4/4+3/3, WIRE-BASED-100
intact, kg-bench 154/154 (`.2`/`.3`/`.3b`).

**Recall cost, quantified per-item (not assumed):** for every swept doc, each gauge-ENTAILED
Pattern record was checked against the promoted surface. 10 flagged; per-item audit: 4 were
kept in equal-or-better form (HBM2's `CKE` gains two grounded conditions), 1 drop is correct
(the CoreSight misattribution), **3 are genuine signal-fact losses — all on the ungated
AXI+ACE doc** (`ACADDR must_not_change` from a coordinated stability sentence; `ARBURST INCR`
+ `ARLEN 0x00` from `| ARBURST | Burst type must be INCR. |` table-cell rows where the cell's
grammatical subject is "Burst type", not the signal), and 2 are NVMe capsule FIELDS
(`ELEN`/`RECFMT`, bit-ranges 31:16/41:40) that Pattern mis-typed as wires — their honest home
is field routing, not the signal surface (residual R3). Net: ~3 genuine losses across 12
swept docs, against Pattern error mass like DTI's 113/114.

**Residual levers surfaced by the sweep (bounded, each probe-first):**
- R1 table-cell-row subjects: recover the signal name from the row-leading cell when the
  sentence's grammatical subject is a description noun (the ARBURST/ARLEN class).
- R2 source-grounded condition recovery: when the model omits the condition, recover it from
  the sentence the same way `.8` recovers values (the AXI-Stream class — correct facts
  flagged for missing "during reset" / "for a transfer to occur" / "when TLAST is LOW").
- R3 register-class field routing: capsule/command field obligations on docs without a
  message-field catalog currently drop instead of routing (the NVMe `ELEN`/`RECFMT` class).
- R4 bit-subrange constraint vocabulary: "the lower N bits are tied LOW" has no typed slot
  (the CoreSight class) — candidate future kind.

**Recommendation:** FLIP the default for live-NLP converge runs. The gauge improves on every
honestly-measurable doc across all classes; the quantified recall cost is ~3 facts on one
ungated doc with recognizable shapes (R1/R2 recover them); the gold gates remain the
regression net on gated docs; the standing gauge keeps flagging the residue; provider-free
CI is untouched by construction. The alternative (stay opt-in until R1/R2 land) trades the
corpus-wide error-mass reduction for ~3 recoverable facts — a poor trade, but the flip is
deliberately the owner's call per the `.1` decision record. → `.5`.

- ID: `LLM-PRIMARY-PROMOTION.5` · Status: `pending` (OWNER-AUTHORIZED `2026-06-14` — un-gated;
  ready to execute) · Goal: execute the owner's default-flip decision = **FLIP**. Make
  `converge --promote-constraints-llm` the DEFAULT for live-NLP runs (provider-free runs stay
  Pattern by construction; keep the flag/an explicit opt-out). Then RE-VERIFY the gold gates on
  CANONICAL artifacts: wire-doc eval P=R=F1=1.000 ×3 + 16/16 doc recall, WIRE-BASED-100
  (constraints/relations/temporal), serial-class no-regression (SWD/CAN/SWP/SMBus/I2S),
  kg-bench, provider-free CI byte-stable. **PROTOCOL (2× burned): restore the Pattern baseline
  BEFORE any promotion measurement.** Needs live Ollama (qwen2.5:14b-instruct) for the converge
  gold-gate runs → RAM care: `ollama stop` before any ingest, watch RAM, autonomous-kill ≥85%
  used. R1–R4 levers stay future candidates (recover the ~3 ungated-doc recall losses).

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `.5` | `pending (OWNER-AUTHORIZED — FLIP)` | Owner authorized the FLIP `2026-06-14` ("do each in turn, you know what needs to be done"); `.4` packet recommended it. Execute the default flip + re-verify all gold gates on canonical artifacts. Deferred to a FRESH session for signoff sharpness (heavy: live 14B converge runs + full gold-gate battery). |

## Changelog

- `2026-06-10`: Created from `EXTRACTION-QUALITY-GAUGE.0`'s key live finding (the canonical
  artifacts still carry the Pattern surface). `.1` design DONE — probed the converge monotone
  invariant (post-stability placement is forced), enumerated the downstream consumer set,
  recorded the refinement-not-discovery recall bound, and locked the opt-in-first /
  manifest-recorded / gauge-after-promotion decisions. See
  [[extraction-quality-gauge-standing]].
- `2026-06-10`: `.2` DONE — the opt-in stage is live: `converge --promote-constraints-llm`
  replaces the Pattern surface post-stability via the shared `promote_constraints` core
  (manifest-recorded `constraints.llm_primary`; stale gauge dropped on replace; flag without a
  provider errors early; gauge measured after promotion). APB end-to-end: 18 → 21 kept, gauge
  28.6% → 23.8% not-entailed, eval gate held at P=R=F1=1.000 + 6/6 doc recall on the promoted
  canonical artifact. See [[llm-primary-promotion-stage]]. Next: `.3` (AHB/AXI + serial
  sweep).
- `2026-06-10`: `.3` DONE — the gold-gate battery on promoted CANONICAL artifacts: APB+AHB
  CLEARED (all gates 1.000; gauges 60.0%→33.3% AHB; /tmp measurements reproduced exactly),
  serial control intact; **AXI CAUGHT a real promotion recall defect** (seed_axi_temporal
  1/3 — the coordinated-subject sentence "SYSCOREQ and SYSCOACK must be deasserted when
  ARESETn is asserted." loses SYSCOREQ because the model misspells it `SYCOREQ` at temp 0,
  ×2 reproducible, and entity typing rightly rejects the undeclared token). AXI REVERTED
  (clean Pattern rebuild; both AXI gates re-verified 1.000; gauge re-measured on the restored
  surface). The defect is a new bounded class → `.3a` (document-grounded typo snap, edit
  distance 1, unambiguous-candidate-only). WIRE-BASED-100 stands intact on all canonical
  artifacts.
- `2026-06-10`: `.3a` DONE — the model-misspelled-subject snap is live (trigger =
  absence-from-sentence, NOT typing failure: production typing defers to the LLM judge and the
  typo grounded as a phantom name dying at the SemanticIR declared-signal filter — the audit
  killed the typing-failure first cut). Snap live-proven: promoted AXI carries SYSCOREQ under
  the document's spelling. The full battery then exposed `.3b`: the temporal derivation loses
  BOTH gold reset rules from the LLM-primary shape (even correctly-spelled SYSCOACK), so AXI
  stays reverted (gates re-verified 1.000) and the default-flip stays blocked. lib 1545. See
  [[model-misspelled-subject-snap]].
- `2026-06-10`: `.3b` DONE — temporal-derivation parity restored. Probe pinned the single
  guilty factor per-item (synthetic /tmp shape-diff: only the constraint kind differs →
  consequent `DEASSERTED`≠gold `LOW`): the post-build promotion replace skipped the build
  path's polarity refinement; SYSCOREQ/SYSCOACK are persisted `active_high`, so the build
  path collapses `must_be_deasserted`→`must_be_low` before any consumer reads the surface.
  Fix = `apply_persisted_polarity_to_constraints` (evidence.rs `pub(crate)` wrapper over the
  build-path refinement, fed by the artifact's persisted `signal_polarities`) called in
  `promote_constraints` before dedup. Antecedent parsing and actor grounding were probed and
  CLEARED (parity by construction). Live: AXI promoted 102→54→50, seed_axi_temporal 3/3 =
  1.000 on the promoted canonical artifact, full battery green (constraints 1.000 ×3, filtered
  relations 1.000, temporal 3/3+4/4+3/3, SWD/I2C intact, kg-bench 154/154); gauge 48.0%
  not-entailed on the promoted surface (vs 91.0% Pattern) with the honest NLI-judge-lacks-
  polarity note recorded. APB/AHB scanned for latent unrefined records: zero. **All three
  wire docs now carry the promoted surface on canonical artifacts; `.4` unblocked.** lib 1546.
  See [[llm-primary-promotion-stage]].
- `2026-06-10`: `.4` DONE — the corpus sweep (12 docs, redirected copies only; canonical
  untouched outside the gated wire docs), the per-item recall-cost quantification (~3 genuine
  losses, all on one ungated doc, recognizable shapes → levers R1/R2), the two-month-stale
  tracked snapshot refreshed (validator-version drift proven by the untouched AXI-Stream
  control 90→67 — scores are not comparable across validator versions), and the
  owner-visible default-flip decision packet written (recommendation: FLIP; the gauge
  improves on 14/15 measurable docs and the 15th dissolves per-item). `.5` = owner-gated
  flip execution. R1–R4 lever candidates recorded in the packet.
- `2026-06-14`: **OWNER DECISION — FLIP authorized.** The owner read the `.4` packet and
  directed proceeding ("Please whatever needs to be done… each of them in turn… you know what
  needs to be done"). `.5` is un-gated. Execution deferred to a FRESH session for signoff
  sharpness (the flip needs repeated live qwen2.5:14b converge runs + the full gold-gate
  battery on canonical artifacts), with the repo left handoff-ready — see `MEMORY.md` for the
  precise resume plan + RAM-safety protocol. No code changed in this handoff commit.
