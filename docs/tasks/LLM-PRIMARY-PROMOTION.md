# LLM-PRIMARY-PROMOTION: promote the LLM-primary constraint surface into the canonical pipeline

## Metadata

- Tree ID: `LLM-PRIMARY-PROMOTION`
- Status: `active` (`.1` design DONE `2026-06-10`; `.2` frontier)
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
- ID: `LLM-PRIMARY-PROMOTION.2` · Status: `pending` · Goal: the opt-in converge promotion
  stage — post-stability `extract-constraints-llm` over the final EvidenceIR (flag-gated,
  manifest-recorded), one downstream rebuild (Semantic → Intent → adapter), gauge measured
  AFTER promotion, summary reports both the promotion delta and the gauge.
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
| 1 | `.2` | `pending` | Design locked; the opt-in stage is the bounded first code slice |
| 2 | `.3` | `pending` | Gold gates on the real canonical artifacts |
| 3 | `.4` | `pending` | Corpus evidence for the default-flip decision |

## Changelog

- `2026-06-10`: Created from `EXTRACTION-QUALITY-GAUGE.0`'s key live finding (the canonical
  artifacts still carry the Pattern surface). `.1` design DONE — probed the converge monotone
  invariant (post-stability placement is forced), enumerated the downstream consumer set,
  recorded the refinement-not-discovery recall bound, and locked the opt-in-first /
  manifest-recorded / gauge-after-promotion decisions. See
  [[extraction-quality-gauge-standing]].
