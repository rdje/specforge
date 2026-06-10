# LLM-PRIMARY-PROMOTION: promote the LLM-primary constraint surface into the canonical pipeline

## Metadata

- Tree ID: `LLM-PRIMARY-PROMOTION`
- Status: `active` (`.1`–`.3a` DONE `2026-06-10`; `.3b` frontier — APB/AHB promoted +
  gate-cleared, AXI reverted: the typo snap is live-proven but the temporal-derivation
  parity gap blocks AXI promotion)
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

- ID: `LLM-PRIMARY-PROMOTION` · Status: `active` · Children: `.1`–`.4`
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
- ID: `LLM-PRIMARY-PROMOTION.3b` · Status: `pending` · Goal: **temporal-derivation parity for
  promoted constraints** — probe-first: diff per-item how the SemanticIR temporal layer
  derives the two gold reset rules from the PATTERN surface (which passes) vs the LLM-primary
  surface (which loses both, even with correct subjects/conditions: `must_be_deasserted` +
  "when ARESETn is asserted"). Candidate factors to probe, not guess: consequent value
  collapse (DEASSERTED→LOW needs grounded polarity), antecedent recovery from the
  condition_text vs the Pattern records' statement linkage, supporting-statement ids feeding
  actor grounding (`C:ads|MANAGER|…`). Then fix in whichever layer is honestly deficient and
  re-run the FULL battery (seed_axi_temporal 3/3 on the promoted artifact = the `.4` unblock).
- ID: `LLM-PRIMARY-PROMOTION.4` · Status: `pending` · Goal: corpus sweep + tracked validation
  snapshot refresh + the default-flip decision packet (owner-visible: per-doc gauge deltas,
  gold-gate status, recommendation).

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `.3b` | `pending` | Temporal-derivation parity: why the gold reset rules derive from the Pattern surface but not the LLM-primary one (probe per-item, fails even for correctly-spelled SYSCOACK) |
| 2 | `.4` | `pending` | Corpus evidence for the default-flip decision (needs `.3b` green on AXI) |

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
