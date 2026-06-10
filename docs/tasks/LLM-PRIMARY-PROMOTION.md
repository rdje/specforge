# LLM-PRIMARY-PROMOTION: promote the LLM-primary constraint surface into the canonical pipeline

## Metadata

- Tree ID: `LLM-PRIMARY-PROMOTION`
- Status: `active` (`.1` design + `.2` opt-in stage DONE `2026-06-10`; `.3` frontier)
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
- ID: `LLM-PRIMARY-PROMOTION.3` · Status: `pending` · Goal: wire-doc end-to-end verification
  on promoted canonical artifacts — eval 1.000 + 16/16 held, temporal/relations re-scored
  (WIRE-BASED-100 intact), serial-class no-regression sweep, gauge before/after recorded
  per document.
- ID: `LLM-PRIMARY-PROMOTION.4` · Status: `pending` · Goal: corpus sweep + tracked validation
  snapshot refresh + the default-flip decision packet (owner-visible: per-doc gauge deltas,
  gold-gate status, recommendation).

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `.3` | `pending` | AHB/AXI promoted-artifact gold gates + serial no-regression sweep |
| 2 | `.4` | `pending` | Corpus evidence for the default-flip decision |

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
