# RUST_CODEBASE_ANALYSIS
## Purpose
- maintain a live, deep-dive analysis of the Rust codebase
- record the current architecture, risks, subsystem boundaries, and recommended implementation direction
- remain useful even while only the early IR stages are implemented

## Session update (2026-04-22 signal-leading clock benchmark lock)
- Continued from commit `c55b858`, still hardening the temporal proof surface rather than widening the parser, semantic model, or validation planner.
- The signal-leading clock family was already directly proved in unit coverage after the previous slices:
  - word-form `HCLK rising edge` / `HCLK falling edge`
  - token-form `HCLK posedge` / `HCLK negedge`
- But that family was still missing from the tracked KG-quality benchmark corpus.
- This slice adds `crates/specforge/test_data/kg_quality/signal_leading_clock_timing_gold/`, a compact tracked fixture that locks all four signal-leading local-clock forms through both `SemanticIR` and `IntentIR`, with `temporal_rules_missing_clock_grounding = 0`.
- The tracked KG-quality suite grows by one fixture family because this is a corpus-surface hardening slice, not just another unit-level proof refinement.
- Focused tracked-fixture coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.
- The tracked KG-quality suite is now `106` fixtures.

## Session update (2026-04-22 signal-leading clock lexical hardening)
- Continued from commit `d237f12`, still tightening the temporal proof surface rather than widening the parser, semantic model, or tracked fixture corpus.
- The signal-leading clock family was already symmetric in edge direction after the previous slice:
  - `HCLK rising edge` and `HCLK falling edge` at the semantic layer
  - `HCLK posedge` and `HCLK negedge` at the validator layer
- But the direct proof was still lexically split across those two lanes.
- This slice makes the family lexically self-contained in both places:
  - `crates/specforge/src/ir/semantic.rs` now also proves token-form `HCLK posedge` / `HCLK negedge` beside the existing word-form pair
  - `crates/specforge/src/commands/validate.rs` now also proves word-form `HCLK rising edge` / `HCLK falling edge` beside the existing token-form pair
- The tracked KG-quality suite stays flat because this is reliability hardening inside an existing temporal family, not a new benchmark family or capability row.
- Focused semantic and validator coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 signal-leading clock symmetry hardening)
- Continued from commit `235197f`, still tightening the temporal proof surface rather than widening the parser, semantic model, or tracked fixture corpus.
- The signal-leading clock family already had direct proof on the rising side:
  - `HCLK rising edge` at the semantic layer
  - `HCLK posedge` at the validator layer
- But the falling-side twins were still only implied by shared parser logic.
- This slice makes the family internally symmetric in its direct proof lanes:
  - `crates/specforge/src/ir/semantic.rs` now proves `PWAKEUP must be asserted on HCLK falling edge.` preserves `clock_signal = HCLK` and `edge = falling` beside the existing rising-side rule
  - `crates/specforge/src/commands/validate.rs` now proves `PWAKEUP must be asserted on HCLK negedge.` stays fully grounded beside the existing `HCLK posedge` rule
- The tracked KG-quality suite stays flat because this is reliability hardening inside an existing temporal family, not a new benchmark family or capability row.
- Focused semantic and validator coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.

## Executive summary
- the repository now contains a single active `specforge` crate and CLI with an executable surface of:
  - `inspect`
  - `doctor`
  - `converge`
  - `ingest`
  - `evidence`
  - `semantic`
  - `intent`
  - `adapt`
  - `enrich`
  - `validate`
  - `kg-bench`
  - `project-validation`
  - `rescan-plan`
  - `learn-priors`
  - `corpus-kb`
  - `clean`
  - `nlp-enrich`
- the canonical product boundary remains `IntentIR`, not `.fsm`
- the staged pipeline is operational through `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- a whole-pipeline fixed-point entrypoint now exists via `specforge converge`, which reuses persisted artifacts, defaults to Ollama VLM + NLP Level 3, and stops when the cross-stage knowledge snapshot is stable
- `SourceIR` now captures structured Docling output, typed content elements, structured tables, visual assets, and document-profile metadata, and its PDF normalization path stages into `normalized.staging/` before atomically replacing `normalized/` so stale source-side leftovers do not accumulate across reruns
- `EvidenceIR` now synthesizes typed declarations and records from tables, preserves typed NLP outputs, persists alias-learning state, extracts actor-signal relation triples from prose and signal-description tables, runs a monotone convergence loop so discovered enum facts and explicit active-level polarity prose can unlock additional signal constraints without hardcoded protocol-specific value lists, keeps polarity disagreement explicit through typed conflict records instead of only via a neutralized fallback, preserves provenance from table-synthesized signal declarations back to originating `SourceIR` table ids, and exposes that bridge through the `table_signal_declaration_provenance` validation metric
- `SemanticIR` now lifts that evidence into interfaces, canonical signal records with table-support ids, explicit interface-signal conflict records for conflicting direction/width evidence, actor-relative port/connectivity records, explicit signal-connectivity conflict records for unresolved multi-producer ambiguity, system/reset/init records, control/state records, timing/register records, and filtered NLP constraints, with VLM observations merged into the semantic surface
- `IntentIR` now carries forward the canonical signal/control/system/state/register/timing surface plus the actor-relative KG needed for honest downstream lowering
- `specforge validate` now reports `table_signal_declaration_provenance` for `EvidenceIR` plus `with_table_support` for `SemanticIR` and `IntentIR`, while `specforge kg-bench` can assert exact EvidenceIR table-signal provenance records, direct EvidenceIR provenance counts, exact canonical table support, validation finding payloads, validation finding related IDs, direct graph-direction conflicted signal-name sets, actor-level graph-direction conflict provenance records, actor-aware graph-direction conflict related ids, direct graph-direction coverage-gap related ids, and direct compat-direction lag related ids via a narrow flat-hint-clearing patch lane, making both evidence-stage table-declaration provenance and validation-backed caution/rescan surfaces visible without replacing exact per-signal provenance checks
- `kg-bench` can now also assert `validation.source` expectations directly, so tracked fixtures can lock SourceIR validation findings and metrics instead of leaving source-stage replay surfaces unit-test-only
- source-stage missing VLM enrichment now joins the replay plane too: `validate` still reports timing/state diagram asset ids when `SourceIR` carries no VLM enrichment, but now also emits source-stage VLM-gap-specific rescan guidance so `project-validation` can route those exact asset ids through a narrower `enrich -> validate` loop on the current `SourceIR` artifact instead of leaving them as passive ingest-side debt
- actor-port gaps now join that canonical replay plane too: `validate` still reports relation-only actor graph remnants when `SemanticIR` or `IntentIR` carries `actor_signal_relations` but no `actor_ports`, but now also emits actor-port-gap-specific rescan guidance so `project-validation` can route those exact `asr_*` relation ids through the bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` lane instead of leaving them as passive graph debt
- graph-direction coverage in validation and benchmark expectations is now a little stricter too: same-actor self-conflicts no longer earn graph-direction credit merely because non-`unknown` actor ports exist somewhere for that signal, validation reports those self-conflicts explicitly instead of only showing a silent coverage drop, and `project-validation` now routes those preserved actor-aware contradictions into the bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` replay lane instead of leaving them as passive warnings
- evidence-stage missing VLM observations now join the replay plane too: `validate` still reports visual evidence ids when `EvidenceIR` carries no timing/state observations, but now also emits VLM-gap-specific rescan guidance so `project-validation` can route those exact visual ids through a source-side `enrich -> evidence -> validate` loop instead of leaving them as passive enrichment debt
- evidence-stage structural-KG gaps now join the replay plane too: `validate` still reports stranded behavioral ids when `EvidenceIR` carries signal constraints or conditional rules without any actor-signal graph, but now also emits structural-gap-specific rescan guidance so `project-validation` can route those exact behavioral ids through a narrower `nlp-enrich -> validate` loop on the current `EvidenceIR` artifact instead of leaving them as passive graph debt
- evidence-stage normative residuals now join the replay plane too: `validate` still reports partially structured normative statement ids as an honesty surface, but now also emits residual-specific rescan guidance so `project-validation` can route those exact statement ids through a narrower `nlp-enrich -> validate` loop on the current `EvidenceIR` artifact instead of leaving them as passive extraction debt
- evidence-stage signal-polarity conflicts now join the replay plane too: `validate` still reports the warning-level active-level disagreement, but now also emits conflict-id-specific rescan guidance so `project-validation` can route preserved `polarity_conflict_*` ids through a narrower `nlp-enrich -> validate` loop on the current `EvidenceIR` artifact instead of leaving them as passive upstream warnings
- evidence-stage negative-knowledge caution now joins the replay plane too: `validate` still reports prior-match caution at `EvidenceIR`, but `project-validation` now routes the exact caution-target ids through a bounded `SourceIR -> EvidenceIR -> validate` lane instead of leaving evidence-stage caution as a planner dead end or pretending it already requires canonical-stage rebuilds
- carried signal-semantic conflicts now join that same replay plane too: `validate` still reports the warning-level semantic-role disagreement, but now also emits conflict-id-specific rescan guidance so `project-validation` can route preserved `semantic_conflict_*` ids through the same bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` loop instead of relying only on the broader signal-name arbitration surface
- the current `.fsm` adapter slice is real and intentionally narrow: it can emit honest `?dt:name`, `?fsm:name`, and `?top:name` outputs when the canonical facts are explicit enough
- the enrichment, convergence, validation, benchmark, targeted-rescan, first prior-learning, and first corpus-knowledge toolchain is also real: `specforge enrich`, `specforge nlp-enrich`, `specforge converge`, `specforge validate`, `specforge project-validation`, `specforge rescan-plan`, `specforge kg-bench`, `specforge learn-priors`, and `specforge corpus-kb` are wired into the CLI and exercised by the workspace tests
- generated artifact hygiene is now part of the executable surface too: `specforge clean` dry-runs rebuildable heavyweight `generated/source_ir/*/normalized` bundles by default, can execute those deletions explicitly, can remove full per-document generated stage trees, and now also supports a first-class `--scope all-generated` sweep for intentionally discarding the entire local `generated/` root
- `project-validation` and `rescan-plan` now form a schema-v2 targeted-rescan loop: recommendations carry typed replay inputs, structured local command hints, dry-run-by-default execution, before/after validation snapshots, execution summaries, promotion-gate descriptors, and an explicit no-canonical-mutation boundary
- that replay surface now spans both evidence-local and canonical-local loops: evidence-stage normative residuals, signal-polarity conflicts, or signal-semantic conflicts can drive a direct `nlp-enrich -> validate` plan on the current `EvidenceIR`, while canonical-stage gaps still use the bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` rebuild lanes
- negative-knowledge caution now spans stage-appropriate replay scopes too: evidence-stage caution uses `SourceIR -> EvidenceIR -> validate`, while semantic/intent caution still use the deeper downstream rebuild lanes
- `CorpusMemory` schema v5 now carries actor-taxonomy, semantic-phrase, semantic-modality-reliability, temporal-phrase, table-shape, visual-motif, and negative-knowledge prior families; current consumers remain advisory and locally grounded rather than fact-authoring, and the negative-knowledge surface now includes signal-polarity-conflict archetypes beside semantic, temporal, interface, connectivity, and residual caution patterns
- the first `R15g` corpus knowledge-base plane now exists too: tracked `corpus_kb/` pages can preserve reviewable cross-document synthesis, and `specforge corpus-kb` refreshes managed validation-finding, KG fixture-result, dedicated semantic/truthfulness pattern, typed-prior-memory, table/visual/state-machine/timing/infrastructure/protocol-family pages, and review-only prior-candidate pages plus a schema-versioned readiness manifest from validation report sidecars / benchmark fixture outcomes without mutating canonical IR or typed priors; the KG fixture-result path now uses quiet fixture-local validation and projects fixture-family summaries so coverage is visible beyond aggregate pass/fail counts
- the tracked KG-quality benchmark surface currently contains 105 fixtures, including signal-table inventory authority and table-provenance coverage, direct signal-inventory exclusion coverage for document/integration vocabulary false positives, direct signal-connectivity conflict shape coverage for multi-producer graph conflicts, direct signal-semantic conflict shape coverage for multimodal disagreement, direct interface-signal conflict shape coverage for direction/width disagreement, direct signal-polarity conflict shape coverage for active-level prose/table disagreement, direct source-stage missing-VLM-enrichment replay-guidance coverage, direct actor-port-gap replay-guidance coverage, direct evidence-stage missing-VLM replay-guidance coverage, direct evidence-stage structural-KG replay-guidance coverage, direct polarity-conflict replay-guidance coverage, direct evidence-stage normative-residual replay-guidance coverage, direct prior-guided polarity-conflict caution coverage, direct resolved signal-polarity shape coverage for active-high/active-low recovery, direct resolved semantic-role shape coverage for valid-like/ready-like recovery, direct semantic-grounding strength coverage for single-source/cross-modality evidence quality, direct temporal-conflict shape coverage for the negative-knowledge caution path, direct temporal-rule shape coverage for representative AXI/APB/AHB timing fixtures including APB address/protection wait-state stability, APB write-control stability, APB response stability, AHB control stability, AHB transfer/lock stability, AHB exclusive/security stability, AHB response stability, AHB write-data stability, AXI write response, write-response ID stability, address/response USER sideband stability, read address, read-address ID stability, address QoS/region sideband stability, data USER sideband stability, read-address control sideband stability, write-address ID stability, write-address control sideband stability, read-address sideband stability, read data, read-data ID stability, read-data response stability, read-data last stability, write data, write-data last stability, sideband stability, write-address sideband stability, generic `clock edge(s) of <clock>` timing, and trailing shorthand-edge timing, direct relative-clause actor-noise rejection and coordinated active-drive object recovery, direct clock/reset infrastructure-topology coverage, protocol-PDF clock/reset contract-scope negative coverage, generic clock/reset advice negative coverage, same-actor graph-direction self-conflict coverage, compat-direction lag despite graph recovery coverage, VLM timing spurious-annotation/sample-index-label rejection, motion-only timing-annotation rejection, waveform-motion state rejection, VLM state-machine label-noise, undeclared-transition, duplicate-initial, multiple-initial, and missing-initial coverage, active-low VLM timing polarity-equivalence coverage, and collective, mixed clause-local, and detached mixed-polarity negative non-reset control polarity coverage

## Session update (2026-04-21 unit-first diagram-position clock grounding)
- Continued from commit `d3ea3a0`, extending the explicit clock-tick model into the remaining unit-first diagram-position grounding gap without widening timing guesses.
- `explicit_clock_signal_from_text()` now recognizes unit-first diagram-position forms with trailing local clock names such as `tick T3 of HCLK`, `posedge T4 of HCLK`, and `rising edge T5 of HCLK`, preserving the local `clock_signal` instead of leaving those rules half grounded.
- The exact bounded `cycle_window` path was already present; this slice completes the grounding story without changing the window semantics.
- Focused parser, semantic, validator, tracked KG fixture, docs CI, full local CI, and whitespace checks all passed for this slice.
- The current local CI baseline is `461` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite remains `103` fixtures.

## Session update (2026-04-21 generic clock-edge-of-clock regression lock)
- Continued from commit `35960c7`, tightening a quality seam without widening the temporal model.
- Generic `clock edge(s) of <clock>` phrasing such as `clock edge T4 of HCLK` and `within 2 clock edges of HCLK` was already supported by the bounded parser helpers and already described in the docs, but it was not yet regression-locked through the semantic and validation layers the way neighboring temporal phrasing families were.
- This slice keeps the model unchanged and makes the contract explicit:
  - parser coverage now proves those forms still recover the expected bounded `cycle_window`
  - semantic coverage now proves `clock_signal = HCLK`, `edge = rising`, and the bounded window survive together into typed temporal rules
  - validator coverage now proves that same phrasing no longer relies on undocumented helper behavior and stays out of both missing-clock and missing-window warning paths
- Focused parser, semantic, validator, tracked KG fixture, docs CI, full local CI, and whitespace checks passed for this slice.
- The current local CI baseline is `467` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite remains `103` fixtures.

## Session update (2026-04-21 generic clock-edge-of-clock tracked fixture coverage)
- Continued from commit `3fc7e1a`, raising the same temporal family into the tracked benchmark corpus instead of leaving it only in unit coverage.
- Added `crates/specforge/test_data/kg_quality/clock_edge_of_clock_timing_gold/`, a minimal tracked fixture that locks both:
  - exact diagram-style generic clock-edge timing: `clock edge T4 of HCLK`
  - bounded generic clock-edge-of-clock timing: `within 2 clock edges of HCLK`
- That fixture now proves, at the KG benchmark level, that both `SemanticIR` and `IntentIR` preserve:
  - `clock_signal = HCLK`
  - `edge = rising`
  - the expected exact or bounded `cycle_window`
- The validation expectations in the same fixture also prove the grounded shape directly instead of only relying on rule presence.
- Focused benchmark coverage and focused `clock_edge_of_clock` unit coverage passed for this slice.
- The current local CI baseline remains `467` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite now contains `104` fixtures.

## Session update (2026-04-22 trailing edge-word timing symmetry lock)
- Continued from commit `9db08cf`, tightening the same trailing shorthand-edge benchmark family again without widening the temporal model or adding a new fixture family.
- The prior slice already benchmark-locked token shorthand symmetry:
  - `within 2 posedges of HCLK`
  - `within 2 negedges of HCLK`
- But the closely parallel word-based rising-edge form was still only implied by local parser and semantic logic:
  - `within 2 rising edges of HCLK`
- This slice makes that neighboring form explicit at all three useful levels:
  - direct semantic regression now proves `PSEL must be asserted within 2 rising edges of HCLK.` preserves `clock_signal = HCLK`, `edge = rising`, and `cycle_window.max_cycles = 2`
  - direct validator regression now proves the resulting intent rule stays fully grounded instead of surfacing missing-clock or missing-window debt
  - the existing tracked fixture `trailing_shorthand_edge_timing_gold` now carries the same bounded word-edge rule so the phrase is benchmark-locked end to end through `SemanticIR`, `IntentIR`, and validation
- The tracked benchmark surface size remains `105` fixtures because this is a hardening pass inside an existing family, not a new family.
- Focused temporal tests, tracked KG fixtures, docs CI, full local CI, and whitespace checks all passed for this slice.
- The current full local CI baseline is now `471` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 trailing falling edge-word symmetry lock)
- Continued from commit `3859295`, keeping the same trailing shorthand-edge family honest without widening truth or creating a new fixture family.
- The prior slice benchmark-locked the word-based rising-side phrase:
  - `within 2 rising edges of HCLK`
- But the falling-side twin was still only implied by generic parser helpers and neighboring token-form coverage:
  - `within 2 falling edges of HCLK`
- This slice makes that twin explicit at the same useful levels:
  - the explicit local-clock extraction regression for trailing `of <clock>` edge phrasing now covers `within 2 falling edges of HCLK`
  - direct semantic coverage now proves `PWRITE must be asserted within 2 falling edges of HCLK.` preserves `clock_signal = HCLK`, `edge = falling`, and `cycle_window.max_cycles = 2`
- the existing validator regression for trailing word-edge timing now proves both rising and falling word-edge rules stay fully grounded together
- the existing tracked fixture `trailing_shorthand_edge_timing_gold` now also carries the falling-side word-edge rule, so both `SemanticIR` and `IntentIR` lock the full five-rule temporal family end to end
- The tracked benchmark surface size remains `105` fixtures because this is another hardening pass inside an existing family, not a new family.
- Focused temporal tests, tracked KG fixtures, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline is now `472` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 ordinal falling edge-word symmetry lock)
- Continued from commit `9cf8a28`, still tightening the same trailing shorthand-edge family rather than widening the temporal model or creating a new fixture family.
- The prior slice closed the bounded falling-side word-edge gap:
  - `within 2 falling edges of HCLK`
- But the exact ordinal family was still asymmetric:
  - `the third rising edge of HCLK` already had direct semantic and validator proof
  - `the third falling edge of HCLK` did not
- This slice makes that exact falling-side twin explicit at the same useful levels:
  - the trailing `of <clock>` local-clock extraction regression now covers `the third falling edge of HCLK`
  - the existing ordinal semantic regression now proves both `third rising edge` and `third falling edge` rules survive typed lowering with `cycle_window = 3..3`
  - the existing explicit-clock validator regression now proves both exact ordinal word-edge rules stay fully grounded together
- the existing tracked fixture `trailing_shorthand_edge_timing_gold` now also carries `PWAKEUP must be asserted on the third falling edge of HCLK.`, so both `SemanticIR` and `IntentIR` lock the full six-rule temporal family end to end
- The tracked benchmark surface size remains `105` fixtures because this is another hardening pass inside the same fixture family.
- Focused extraction, semantic, validator, tracked KG fixture checks, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `472` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 word-edge validator family completion)
- Continued from commit `7ec2155`, still hardening the temporal proof surface rather than widening the temporal model or adding benchmark corpus.
- The spelled-out trailing word-edge family was already supported and benchmark-locked, but its direct validator proof was still split:
  - exact `third rising edge` / `third falling edge` lived in the broader `explicit_clock_text` regression
  - bounded `within 2 rising edges` / `within 2 falling edges` lived in the `trailing_of_word_edge` regression
- This slice makes that validator story self-contained:
  - `validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_word_edge_text` now also carries the exact rising/falling word-edge pair
  - the same direct validator lane now proves all four trailing word-edge corners together: exact rising, exact falling, bounded rising, and bounded falling
- The tracked benchmark surface size remains `105` fixtures because this is validator hardening inside an existing temporal family, not a new corpus addition.
- Focused validator coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 exact shorthand-token validator symmetry lock)
- Continued from commit `2f20610`, still hardening the same trailing shorthand-edge family rather than widening the temporal model or adding new benchmark corpus.
- The previous slice closed the benchmark and semantic asymmetry for the exact shorthand-token pair:
  - `the third posedge of HCLK`
  - `the third negedge of HCLK`
- After that, the direct intent-stage validator lane was still slightly uneven:
  - bounded `posedges` / `negedges` were both covered
  - exact `negedge` was covered
  - exact `posedge` was only indirectly protected by the tracked fixture and nearby semantic regression
- This slice closes that remaining validator asymmetry:
  - the existing `validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_shorthand_edge_text` regression now also carries `PGRANT must be asserted on the third posedge of HCLK.`
  - the same direct validator lane now proves all four shorthand-token corners together: bounded rising, bounded falling, exact rising, and exact falling
- The tracked benchmark surface size remains `105` fixtures because this is validator hardening inside an existing temporal family, not a new corpus addition.
- Focused validator coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `473` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 exact falling shorthand-token benchmark lock)
- Continued from commit `628c124`, still deepening the existing trailing shorthand-edge family rather than widening the temporal model or creating a new benchmark family.
- The previous slice benchmark-locked the exact ordinal word-edge pair:
  - `the third rising edge of HCLK`
  - `the third falling edge of HCLK`
- After that, one exact shorthand-token asymmetry remained:
  - `the third posedge of HCLK` was already fixture-locked
  - `the third negedge of HCLK` was still only implied by nearby parser and semantic logic
- This slice closes that last exact shorthand-token gap:
  - the trailing `of <clock>` local-clock extraction regression now also covers `the third negedge of HCLK`
  - a new direct semantic regression now proves `PLOCK must be asserted on the third negedge of HCLK.` preserves `clock_signal = HCLK`, `edge = falling`, and `cycle_window = 3..3`
  - the existing trailing shorthand-edge validator regression now proves the exact falling token form stays grounded alongside the already-locked bounded token pair
  - the existing tracked fixture `trailing_shorthand_edge_timing_gold` now carries `PLOCK must be asserted on the third negedge of HCLK.`
- The tracked benchmark surface size remains `105` fixtures because this is another hardening pass inside the same fixture family.
- Focused extraction, semantic, validator, tracked KG fixture checks, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline is now `473` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-22 ordinal rising edge-word benchmark lock)
- Continued from commit `cb24e3b`, still tightening the same trailing shorthand-edge family rather than widening the temporal model or spawning a new benchmark family.
- The prior slice benchmark-locked the exact falling-side ordinal word-edge form:
  - `the third falling edge of HCLK`
- After that, the exact word-based ordinal pair was symmetric in direct semantic and validator proof, but still asymmetric at the tracked fixture level:
  - `the third rising edge of HCLK` remained unit-locked
  - `the third falling edge of HCLK` was fixture-locked
- This slice closes that last benchmark asymmetry:
  - the trailing `of <clock>` local-clock extraction regression now also covers `the third rising edge of HCLK`
  - the existing tracked fixture `trailing_shorthand_edge_timing_gold` now carries `PGRANT must be asserted on the third rising edge of HCLK.`
  - both `SemanticIR` and `IntentIR` now lock the full seven-rule temporal family end to end, including exact ordinal word-edge rules on both sides
- The tracked benchmark surface size remains `105` fixtures because this is another hardening pass inside the same fixture family.
- Focused extraction, ordinal semantic/validator checks, tracked KG fixture checks, docs CI, full local CI, and whitespace checks passed for this slice.
- The current full local CI baseline remains `472` Rust tests plus warning-deny rustdoc and the mdBook build.
## Session update (2026-04-21 trailing shorthand-edge tracked fixture coverage)
- Continued from commit `f7f9921`, raising the neighboring trailing shorthand-edge family into the tracked benchmark corpus instead of leaving it only in unit coverage.
- The first benchmark pass exposed a real semantic bug rather than just a missing fixture:
  - `within 2 negedges of HCLK` already preserved the local `clock_signal` and bounded `cycle_window`
  - but `explicit_clock_edge_from_text()` still only recognized singular `posedge` / `negedge`
  - so plural shorthand-edge timing could silently fall back to `edge = rising`
- `crates/specforge/src/ir/semantic.rs` now fixes that boundary directly by recognizing plural `posedges` and `negedges`, and a focused regression now proves `within 2 negedges of HCLK` keeps `ClockEdge::Falling`.
- Added `crates/specforge/test_data/kg_quality/trailing_shorthand_edge_timing_gold/`, a minimal tracked fixture that locks both:
  - exact shorthand-edge timing: `the third posedge of HCLK`
  - bounded shorthand-edge timing: `within 2 negedges of HCLK`
- That fixture now proves, at the KG benchmark level, that both `SemanticIR` and `IntentIR` preserve:
  - `clock_signal = HCLK`
  - the explicit edge kind for both rules
  - the expected exact or bounded `cycle_window`
- The validation expectations in the same fixture also prove the grounded shape directly instead of only relying on rule presence.
- Focused benchmark coverage, focused `trailing_of_shorthand_edge` unit coverage, docs CI, full local CI, and whitespace checks passed for this slice.
- The current local CI baseline is now `468` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite now contains `105` fixtures.

## Session update (2026-04-21 plural shorthand-edge symmetry lock)
- Continued from commit `58d0333`, tightening the same temporal family without widening the model or adding a new benchmark family.
- The prior slice fixed plural shorthand-edge semantics in code and directly proved the falling-side plural form, but the rising-side plural twin was still only implied by shared detector logic.
- This slice makes that symmetry explicit:
  - `trailing_shorthand_edge_timing_gold` now also carries `PENABLE must be asserted within 2 posedges of HCLK.`
  - focused semantic regression now proves plural rising shorthand preserves `clock_signal = HCLK`, `edge = rising`, and `cycle_window.max_cycles = 2`
  - validator coverage now proves plural rising and plural falling shorthand rules stay grounded together in one intent-stage report
- The tracked benchmark surface size is unchanged at `105` fixtures because this is a hardening pass inside an existing fixture family, not a new benchmark family.
- The current local CI baseline is now `469` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 trailing-`of <clock>` shorthand-edge grounding)
- Continued from commit `6477b08`, closing another half-grounded temporal corner without widening truth.
- `extract_cycle_window_from_text()` and `explicit_clock_edge_from_text()` already knew how to recover bounded windows and explicit edge kind from phrases like `the third posedge of HCLK` or `within 2 negedges of HCLK`, but `explicit_clock_signal_from_text()` could still drop the locally named clock unless the phrase used the diagram-position `... T4 of HCLK` family.
- This slice keeps the fix narrow:
  - only explicit trailing `of <known clock>` shorthand-edge phrasing is added
  - the widening is limited to edge families already accepted as typed timing language
  - there is still no arbitrary clock guessing from free-form shorthand edge text
- Focused parser, semantic, validator, tracked KG fixture, docs CI, full local CI, and whitespace checks passed for this slice.
- The current local CI baseline is `464` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite remains `103` fixtures.

## Session update (2026-04-21 named diagram-style generic-edge positions)
- Continued from commit `d7557f8`, extending the explicit clock-tick model into the remaining named diagram-style generic-edge gap without widening arbitrary edge guessing.
- `extract_cycle_window_from_text_with_known_signals()` now recognizes exact diagram-position forms such as `HCLK edge T3`, `edge T3 of HCLK`, and `clock edge T4 of HCLK`, recovering `cycle_window = N..N` through a bounded known-signal-aware helper instead of leaving those phrases as half-parsed prose.
- `explicit_clock_signal_from_text()` now reuses that same bounded helper, so named diagram-style generic-edge phrases preserve the local `clock_signal` instead of only recovering a numeric window.
- Focused parser, semantic, validator, tracked KG fixture, docs CI, full local CI, and whitespace checks all passed for this slice.
- The current local CI baseline is `458` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite remains `103` fixtures.

## Session update (2026-04-21 evidence-stage negative-knowledge replay planning)
- Continued from commit `29d97ef`, closing a planner-consumption gap without widening truth.
- `validate` already emitted `evidence_negative_knowledge_rescan_guidance` for learned caution matches at `EvidenceIR`; this slice makes `project-validation` consume that finding instead of silently dropping it.
- The bounded replay contract stays upstream and honest about stage ownership:
  - replay inputs are `source_ir` plus `evidence_ir`
  - command hints are `rebuild_evidence_ir` then `validate_current_artifact`
  - the action text now says "related evidence conflict or residual ids" so caution does not imply downstream canonical rebuilds that the current stage does not justify
- Focused planner coverage, full tracked KG-bench validation, docs CI, full local CI, and whitespace checking passed for this slice.
- The current local CI baseline is `429` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite remains `103` fixtures.

## Session update (2026-04-21 actor-port-gap replay guidance)
- Continued from commit `5505790`, extending the canonical replay surface to another explicit graph-completeness gap rather than widening truth.
- `validate` still reports `semantic_actor_ports_missing` / `intent_actor_ports_missing` as explicit knowledge-graph errors, but now also emits actor-port-gap-specific rescan guidance with the exact `asr_*` relation ids that still lack actor-relative port synthesis.
- `project-validation` now treats that gap as a first-class replay target and routes it through the bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` lane instead of leaving relation-only graph remnants as passive canonical debt.
- `kg-bench` now accepts `semantic_ir_patch.clear_actor_ports`, and the new tracked fixture `actor_port_gap_surface_negative` locks the replay surface against a relation-present, actor-port-missing canonical artifact.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, full local CI, and whitespace checking passed for this slice.
- The current local CI baseline is `428` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite now contains `103` fixtures.

## Session update (2026-04-21 source-stage VLM replay guidance)
- Continued from commit `f18d02a`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `source_vlm_enrichment_missing` as an explicit honesty warning, but now also emits `source_vlm_enrichment_missing_surface_rescan_guidance` with the exact timing/state diagram asset ids that still lack VLM enrichment.
- `project-validation` now treats that source-stage finding as a first-class replay target and emits a SourceIR-local `enrich -> validate` plan against the current `SourceIR`, rather than leaving it as passive visual-enrichment debt or forcing a downstream symptom-oriented replay.
- `kg-bench` now accepts `validation.source` expectations too, and the new tracked fixture `source_vlm_enrichment_surface_negative` locks that source-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, full local CI, and whitespace checking passed for this slice.
- The current local CI baseline is `426` Rust tests plus warning-deny rustdoc and the mdBook build, and the tracked KG-quality suite now contains `102` fixtures.

## Session update (2026-04-21 evidence-stage missing VLM replay guidance)
- Continued from commit `fca8c73`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `evidence_missing_vlm_observations` as an explicit honesty warning, but now also emits `evidence_missing_vlm_observations_surface_rescan_guidance` with the exact visual evidence ids that still lack timing/state extraction.
- `project-validation` now treats that evidence-stage finding as a first-class replay target and emits a source-side `enrich -> evidence -> validate` plan against the current `SourceIR`, rather than forcing an NLP-only lane when the missing information is visual rather than textual.
- The new tracked fixture `evidence_missing_vlm_observations_surface_negative` now locks that evidence-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, and full local CI passed for this slice; the current local CI baseline is `424` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 evidence-stage structural-KG replay guidance)
- Continued from commit `cb3c08b`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `evidence_structural_kg_missing` as an explicit honesty warning, but now also emits `evidence_structural_kg_missing_surface_rescan_guidance` with the exact stranded behavioral ids.
- `project-validation` now treats that evidence-stage finding as a first-class replay target and emits a narrower `nlp-enrich -> validate` plan against the current `EvidenceIR` artifact, rather than forcing a downstream rebuild lane when the missing graph grounding has not yet left the evidence stage.
- The new tracked fixture `evidence_structural_kg_missing_surface_negative` now locks that evidence-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, and full local CI passed for this slice; the current local CI baseline is `422` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 evidence-stage normative residual replay guidance)
- Continued from commit `02f862e`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `EvidenceIR` normative residual statement ids as explicit honesty signals, but now also emits `evidence_normative_residual_surface_rescan_guidance` with those exact residual statement ids.
- `project-validation` now treats that evidence-stage finding as a first-class replay target and emits a narrower `nlp-enrich -> validate` plan against the current `EvidenceIR` artifact, rather than forcing a downstream rebuild lane when the under-structuring has not yet left the evidence stage.
- The new tracked fixture `evidence_normative_residual_surface_negative` now locks that evidence-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, and full local CI passed for this slice; the current local CI baseline is `421` Rust tests plus warning-deny rustdoc and the mdBook build.
- active drive/read actor extraction now trims relative clauses before selecting subject actors, scans the active verb's current object clause for coordinated signals, and shared actor-term hygiene rejects descriptive phrases such as `mixture of`; the staged KG fixture locks `interconnect` as producer and `Manager` as consumer for both `ARCHUNKEN` and `RCHUNKV` while preserving zero connectivity conflicts
- direct `.fsm` graph-backed direction recovery is now target-actor-aware: standalone direct roots can select one actor that graph-drives every render-critical output target, so external actors that drive inputs or read outputs no longer make an otherwise clear target actor unusable; after that target actor is selected, explicit control reads can recover target-actor input directions for local inventory signals across DT and true FSM roots without mutating canonical `IntentIR`; standalone explicit module roots now apply the same bounded read-side recovery inside module-local inventories and preserve contradictory graph/control-read evidence as unresolved; top-composition analysis also preserves top-link-recovered boundary directions in blocked adapter artifacts, collapses contradictory top-boundary plus duplicate top-port direction/width evidence to unresolved state, and can recover existing child-module input/output port directions from explicit child-link endpoints before module renderability analysis while preserving contradictory topology as unresolved; direct and explicit-module roots can also recover existing clock/reset signal input/1-bit shape from canonical system-contract facts while preserving contradictory local shape as unresolved
- adapter-local signal inventories now also separate graph-derived direction from compatibility/system-contract direction explicitly: `graph_direction_hint` plus sticky graph-conflict tracking feeds renderability first, compatibility direction only fills when graph evidence is absent, and explicit graph disagreement remains blocking instead of silently falling back to flat hints

## Session update (2026-04-21 evidence-stage polarity conflict replay guidance)
- Continued from commit `50f3ada`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `EvidenceIR` `signal_polarity_conflicts` as explicit honesty warnings, but now also emits `evidence_signal_polarity_conflict_surface_rescan_guidance` with the exact `polarity_conflict_*` ids.
- `project-validation` now treats that evidence-stage finding as a first-class replay target and emits a narrower `nlp-enrich -> validate` plan against the current `EvidenceIR` artifact, rather than forcing a downstream rebuild lane when the disagreement has not yet left the evidence stage.
- The tracked fixtures `control_polarity_conflict_negative` and `negative_knowledge_prior_guided_polarity_conflict_caution_gold` now lock that evidence-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, docs CI, and full local CI passed for this slice; the current local CI baseline is `420` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 evidence-stage semantic conflict replay guidance)
- Continued from commit `dbce48e`, extending the replay surface one level earlier in the pipeline rather than widening canonical truth.
- `validate` still reports `EvidenceIR` `signal_semantic_conflicts` as explicit honesty warnings, but now also emits `evidence_signal_semantic_conflict_surface_rescan_guidance` with the exact `semantic_conflict_*` ids.
- `project-validation` now treats that evidence-stage finding as a first-class replay target and emits a narrower `nlp-enrich -> validate` plan against the current `EvidenceIR` artifact, rather than forcing a downstream rebuild lane when the disagreement has not yet left the evidence stage.
- The tracked fixtures `cross_modality_semantic_conflict_negative` and `negative_knowledge_prior_guided_semantic_conflict_caution_gold` now lock that evidence-stage replay surface.
- Focused validator coverage, focused planner coverage, full tracked KG-bench validation, and full local CI passed for this slice; the current local CI baseline is `419` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 signal-semantic conflict replay guidance)
- Continued from commit `3687576`, extending the schema-v2 replay surface rather than widening canonical truth.
- `validate` still reports carried `signal_semantic_conflicts` as explicit honesty warnings, but now also emits `semantic_signal_semantic_conflict_surface_rescan_guidance` / `intent_signal_semantic_conflict_surface_rescan_guidance` with the exact `semantic_conflict_*` ids.
- `project-validation` now treats those findings as first-class evidence-local replay targets, reusing the existing bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` plan shape and action summaries instead of leaving typed semantic-role contradiction as passive review debt or relying only on the higher-level arbitration-by-signal lane.
- The tracked fixtures `cross_modality_semantic_conflict_negative` and `negative_knowledge_prior_guided_semantic_conflict_caution_gold` now lock that replay surface with and without prior-memory caution.
- Focused validator tests, focused planner tests, full tracked KG-bench validation, and full local CI passed for this slice; the current local CI baseline is `418` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-21 graph-direction self-conflict replay guidance)
- Continued from commit `a94c73f`, extending the schema-v2 replay surface rather than widening canonical truth.
- `validate` still reports same-actor `graph_direction_conflicts` as explicit honesty warnings, but now also emits `semantic_graph_direction_conflict_surface_rescan_guidance` / `intent_graph_direction_conflict_surface_rescan_guidance` with the exact actor-aware conflict ids.
- `project-validation` now treats those findings as first-class evidence-local replay targets, reusing the existing bounded `EvidenceIR -> SemanticIR -> IntentIR? -> validate` plan shape and action summaries instead of leaving graph-direction contradiction as passive review debt.
- The tracked fixture `graph_direction_same_actor_conflict_negative` now locks that replay surface end to end, so the benchmark proves both the honesty boundary and the follow-up contract.
- Focused planner tests, focused validator tests, full tracked KG-bench validation, and full local CI passed for this slice; the current local CI baseline is `416` Rust tests plus warning-deny rustdoc and the mdBook build.

## Session update (2026-04-18 KG-bench graph-direction conflict fixture lane)
- Continued from commit `99808e9`, turning the new validation honesty surface into a tracked benchmark contract.
- `kg-bench` now accepts a narrow `semantic_ir_patch.actor_ports_append` surface, so fixtures can append canonical actor-port records after normal semantic build and then still flow through the standard `SemanticIR -> IntentIR -> validate` path.
- The new tracked fixture `graph_direction_same_actor_conflict_negative` uses that patch lane to append a conflicting `Completer -> PREADY` input port and proves:
  - graph-direction signal inclusion/exclusion stays `PADDR` yes / `PREADY` no
  - `SemanticIR` and `IntentIR` both report `graph_direction_conflicts: 1`
  - both stages emit the graph-direction conflict warning with `PREADY` as a related id
- This is intentionally a narrow benchmark escape hatch, not a general semantic patching framework. The point is to express canonical honesty regressions that normal source-only synthesis does not naturally preserve, while still exercising the ordinary stage builders and validators.
- Focused harness-unit validation, focused tracked-fixture validation, full `90/90` tracked KG-bench validation, and full local CI with `357` Rust tests plus rustdoc and the mdBook build passed for this slice.

## Session update (2026-04-19 compat-direction lag related IDs)
- Continued from commit `792c60e`, tightening the sibling compatibility-surface warning now that graph-direction conflict vs coverage semantics are cleaner.
- Validation already knew when graph evidence had recovered signal direction while flat compatibility `direction_hint` lagged behind; this slice makes that surface reviewable instead of count-only.
- `semantic_compat_direction_hints_incomplete` now emits missing flat-hint signal names directly in `related_ids`.
- `intent_compat_direction_hints_lag_graph` now emits `related_ids` only for graph-backed declared signals with missing flat hints, which keeps the finding aligned with its actual meaning instead of counting every missing compat hint on the page.
- `kg-bench` now supports a narrow `semantic_ir_patch.clear_signal_direction_hints` patch lane so tracked fixtures can express "graph truth survived, flat compat lagged" without mutating actor ports or general semantic structure.
- The new tracked fixture `compat_direction_hints_lag_graph_negative` locks that graph-first truth boundary end to end, and the corpus-KB projection now reports `92/92` tracked fixtures with `51/51` semantic/truthfulness fixtures.
- Focused validator coverage, focused harness coverage, the tracked fixture run, full local CI with `361` Rust tests plus rustdoc and the mdBook build, and `corpus-kb --kg-fixtures-root` refresh all passed for this slice.

## Session update (2026-04-18 corpus-KB benchmark refresh to 90 fixtures)
- Continued from commit `1ae25f8`, synchronizing the review-facing corpus knowledge-base plane with the newly expanded tracked benchmark suite.
- Re-ran `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality`, which refreshed the managed benchmark and pattern pages without changing canonical IR, validation behavior, or benchmark execution semantics.
- The benchmark projection now reports `90/90` passing fixtures, and the pattern page now reports `49` fixtures across the semantic/truthfulness families.
- `graph_direction_same_actor_conflict_negative` now appears explicitly in those managed review surfaces under:
  - `actor connectivity`
  - `truthfulness negatives and cautions`
- This matters as continuity infrastructure, not only as vanity bookkeeping: future sessions and human reviewers now see the real tracked suite size and the new graph-direction conflict coverage in the persistent corpus-KB plane instead of an outdated `89/89` snapshot.

## Session update (2026-04-18 graph-direction conflict visibility)
- Continued from commit `4fa5e31`, keeping the stricter graph-direction truth boundary but improving observability.
- The validator already refused to credit same-actor self-conflicts as graph-resolved direction; this slice makes that decision explicit through a shared summary helper that returns both resolved signal names and conflicted signal names.
- `SemanticIR` and `IntentIR` validation now both expose:
  - a `graph_direction_conflicts` metric
  - a printed coverage-line conflict count
  - a warning finding that names the conflicted signal ids and explains why graph coverage remains unresolved
- This stays on the reporting side of the boundary:
  - no canonical `SemanticIR` or `IntentIR` facts are rewritten
  - no conflict auto-resolution is attempted
  - the goal is truthful diagnostics, not silent healing
- Focused graph-direction validation tests, the existing `kg-bench` conflict guard regression, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 graph-direction coverage conflict guard)
- Continued from commit `2e7a49b`, tightening the graph-first evaluation surface rather than widening adapters again.
- `validate::graph_direction_signal_names()` now excludes signals when the same actor claims contradictory directions for that same signal, so graph-direction coverage no longer over-credits locally self-conflicted actor-port evidence.
- `kg-bench` now calls that same validator helper, keeping benchmark expectations aligned with validation semantics instead of letting fixture coverage drift onto a looser interpretation.
- This is deliberately narrower than forcing the actor-relative model back into one flat direction bit:
  - different actors can still contribute different directions for the same signal
  - structural multi-producer ambiguity is still surfaced separately through connectivity conflicts
  - only same-actor self-contradiction loses graph-direction credit here
- Focused `validate` tests, focused `kg-bench` tests, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter graph-backed direction surface split)
- Continued from commit `42b5adc`, landing the next concrete `R15` adapter consumer split rather than another analysis-only refresh.
- `FsmSignalCandidate` now carries `graph_direction_hint` and `graph_direction_hint_conflicted`, so adapter-local graph overlays no longer have to reuse the compatibility-facing `direction_hint` field.
- Actor-port, child-link topology, module-control-read, and direct-control-read overlays now all register through that graph-backed surface, while flat interface/system-contract shape remains available as compatibility evidence.
- Renderability now reads direction with an explicit three-state rule:
  - unambiguous graph direction wins
  - absent graph evidence may fall back to compatibility direction
  - conflicted graph evidence stays unresolved and blocks lowering even if a flat compatibility hint exists
- That last behavior is deliberate honesty, not pessimism: if the actor/topology/control-read graph disagrees internally, the adapter should not let a flat hint erase the conflict and pretend the canonical world model is already coherent.
- Focused adapter validation, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter duplicate top-port width conflict collapse)
- Continued from commit `6282875`, hardening duplicate explicit top-port width handling.
- Duplicate top-port declarations still block lowering, but their width hints now merge through sticky `TopPortWidthEvidence` instead of overwriting earlier width evidence in blocked artifacts.
- Added `top_composition_keeps_duplicate_top_port_width_conflict_unresolved`, where `drive_data` is declared twice as a top output with widths `8` and `16`.
- Expected behavior is blocked `.fsm` lowering, no emitted target text, resolved output direction, and unresolved width in both the top candidate and selected top signal inventory.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.
- the local runtime boundary is now operationally stronger too: `specforge doctor` reports Docling readiness, the default Ollama loopback readiness, and LM Studio fallback readiness directly, repo-local `.venv-docling` auto-discovery is supported, and the backend now probes versioned Python candidates like `python3.11` before giving up on fresh ingest
- GitHub Actions CI is part of the repo baseline and still runs `cargo fmt --all --check`, warning-deny Clippy, warning-deny Rust tests, warning-deny rustdoc, and the mdBook build when launched manually, but automatic `push` / `pull_request` triggers are temporarily paused to conserve account Actions minutes
- that CI path still has a single checked-in entrypoint at `scripts/run_ci.sh`, and the GitHub workflow calls that script directly so local and hosted Rust validation do not drift apart
- the remaining dominant gaps are semantic-truthfulness gaps: finishing the remaining graph-first consumers, deepening the temporal-rule layer into richer actor-relative and contradiction-aware clocked semantics, KG-guided rescans, evidence arbitration, benchmark-quality evaluation, and deepening the now-started `R15g` corpus knowledge base beside the already-live typed prior-memory plane; adapter expansion is now horizon work
- the workspace currently validates through `bash scripts/run_ci.sh`, which runs Rust formatting, Clippy with `-D warnings`, Rust tests with `RUSTFLAGS="-D warnings"`, rustdoc with `RUSTDOCFLAGS="-D warnings"`, and the mdBook docs build; after the signal-polarity conflict replay/caution slice the full local CI path reports clean formatting, clean Clippy, `414` passing Rust tests, clean Rust API docs, and a successful mdBook build

## Session update (2026-04-21 polarity-conflict replay and caution)
- Continued from commit `5501cd9`, extending the replay-oriented validation lane to the remaining carried active-level disagreement surface instead of leaving polarity as a warning-only outlier.
- `validate` now emits `semantic_signal_polarity_conflict_surface_rescan_guidance` / `intent_signal_polarity_conflict_surface_rescan_guidance` alongside the existing carried polarity-conflict warnings, using the preserved `polarity_conflict_*` ids directly.
- `project-validation` now classifies those findings into the same bounded local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane used by the nearby conflict and evidence-strength surfaces, with action text aimed at collapsing the related conflict ids toward one locally corroborated active-level interpretation.
- Negative-knowledge caution learned to cover this family too:
  - `NegativeKnowledgeKind` now includes `signal_polarity_conflict`
  - `learn-priors` harvests the normalized polarity-disagreement shape from validated `IntentIR`
  - `validate` matches that learned caution in `EvidenceIR`, `SemanticIR`, and `IntentIR` without mutating current-document truth
- The tracked benchmark surface now locks both sides:
  - `control_polarity_conflict_negative` requires the semantic/intent replay-guidance findings
  - `negative_knowledge_prior_guided_polarity_conflict_caution_gold` proves evidence-stage caution plus semantic/intent replay guidance and negative-knowledge findings for the same preserved conflict id
- Focused polarity-validation coverage, focused polarity prior-learning coverage, focused project-validation coverage, full tracked `kg-bench`, docs CI, full local CI with `414` Rust tests plus rustdoc and mdBook, and `git diff --check` passed for this slice.

## Session update (2026-04-18 adapter duplicate top-port direction conflict collapse)
- Continued from commit `283215f`, hardening duplicate explicit top-port declaration handling.
- Duplicate top-port declarations still block lowering, but their direction hints now merge through sticky `TopPortDirectionEvidence` instead of overwriting earlier evidence in the direction map.
- Added `top_composition_keeps_duplicate_top_port_direction_conflict_unresolved`, where `drive_data` is declared as both top output and top input.
- Expected behavior is blocked `.fsm` lowering, no emitted target text, and unresolved `drive_data` direction in both the top candidate and selected top signal inventory.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter top-port direction conflict collapse)
- Continued from commit `3f99161`, hardening explicit top-boundary direction analysis.
- Added sticky `TopPortDirectionEvidence` so contradictory top declaration/topology evidence blocks lowering and collapses the resolved top port direction to `None` instead of leaving a stale earlier hint visible.
- Added `top_composition_keeps_conflicting_top_port_direction_unresolved`, where `drive_data` is declared as a top output but used as a top-link source, implying top input.
- Expected behavior is blocked `.fsm` lowering, no emitted target text, and unresolved `drive_data` direction in both the top candidate and selected top signal inventory.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 graph-direction conflict finding provenance)
- Continued from commit `08de26a`, aligning validation payloads with the sharper graph-conflict truth surface already present in the benchmark harness.
- Same-actor graph-direction conflict findings now emit stable actor-aware related ids such as `graph_direction_conflict:actor_completer:PREADY` instead of only the raw conflicted signal name.
- The signal-count metric stays intentionally unchanged: `graph_direction_conflicts` still counts conflicted signals, while the warning payload now carries the actor-level provenance needed for targeted review and rescan planning.
- Updated the tracked `graph_direction_same_actor_conflict_negative` fixture and focused validation regressions so both `SemanticIR` and `IntentIR` lock the actor-aware related-id payload directly.
- Formatting, three focused regressions, the focused tracked fixture run, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 kg-bench graph-direction conflict provenance)
- Continued from commit `c17dda7`, tightening the tracked graph-direction truth contract without changing canonical behavior.
- Added `graph_direction_conflicts_include` to canonical-stage `kg-bench` expectations so fixtures can assert same-actor self-conflicts as typed actor-plus-signal records.
- The harness derives those records from the same graph-direction coverage summary used for resolved/conflicted signal-name coverage, keeping the conflict detector centralized instead of duplicating logic in the benchmark harness.
- Updated `graph_direction_same_actor_conflict_negative` to lock the specific culprit explicitly: `Completer` self-conflicts on `PREADY`.
- Formatting, two focused regressions, the focused tracked fixture run, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 kg-bench graph-direction conflict expectations)
- Continued from commit `f8359fa`, tightening the tracked graph-direction honesty contract without changing canonical semantics.
- Added `graph_direction_conflicted_signal_names_include` and `graph_direction_conflicted_signal_names_exclude` to canonical-stage `kg-bench` expectations.
- The harness computes that set from the same graph-direction coverage summary used by validation, so fixtures can assert exactly which signals were withheld from resolved graph-direction credit because of same-actor self-conflicts.
- Updated `graph_direction_same_actor_conflict_negative` to prove the split explicitly: `PADDR` remains resolved while `PREADY` appears only in the conflicted set.
- Formatting, the focused harness test, the focused tracked fixture run, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter system-contract signal conflict guard)
- Continued from commit `3d417ea`, adding adversarial regression coverage for clock/reset signal recovery from canonical system-contract facts.
- The new test marks `clk` as a direct interface output while the canonical system contract still says `clk` is the clock.
- Expected behavior is sticky unresolved direction: `clk` keeps `system_contract_signal` provenance, `direction_hint` collapses to `None`, the adapter emits the existing system-contract residual, and no `.fsm` text is emitted.
- This protects the system-contract recovery boundary: clock/reset facts may fill absent adapter shape, but contradictory local signal shape still requires upstream correction.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter system-contract signal recovery)
- Continued from commit `2bd0034`, moving clock/reset adapter renderability away from flat interface shape hints when canonical system-contract facts are already present.
- Added `overlay_system_contract_signal_inventory`, which applies to direct roots and explicit module roots and overlays existing local clock/reset inventory entries as input, 1-bit `system_contract_signal` evidence.
- The overlay is bounded: it cannot create undeclared clock/reset ports, does not mutate canonical `IntentIR`, and still relies on sticky merge behavior to keep contradictory evidence unresolved.
- Added focused regressions for standalone sequential DT and standalone explicit module roots where flat `clk` / `rst_n` direction and width hints are cleared but the system contract remains intact.
- Formatting, both focused system-contract recovery tests, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter child-link topology conflict guard)
- Continued from commit `6d05173`, adding adversarial regression coverage for child-module direction recovery from explicit top-link topology.
- The new test clears flat module-local directions, then makes topology evidence disagree about `producer_core.output_data`: one link uses it as a child source, another as a child target.
- Expected behavior is sticky unresolved direction: `producer_core.output_data` keeps `module_topology_link` provenance, `direction_hint` collapses to `None`, the producer module blocks with the existing missing-direction diagnostic, and no `.fsm` text is emitted.
- This protects the topology recovery boundary: explicit links may fill absent child port roles, but contradictory topology still requires upstream correction.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter child-link topology direction recovery)
- Continued from commit `e03d5f3`, moving another top-composition consumer away from flat module-local `direction_hint` dependence.
- Added a topology-local child-port direction projection from explicit top links: child link sources imply module outputs, and child link targets imply module inputs.
- The overlay is bounded to already-inventory module signals and uses the sticky merge path, so it cannot invent ports, cannot mutate canonical `IntentIR`, and cannot force a winner when flat, actor-port, control-read, or topology evidence conflicts.
- Added `top_composition_recovers_child_directions_from_link_topology`, which clears module-local direction hints and supplies no actor ports; explicit topology alone recovers `producer_core.output_data`, `consumer_core.input_data`, and `consumer_core.result_data` well enough to emit the top.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter explicit-module read conflict guard)
- Continued from commit `e7b41e4`, adding adversarial regression coverage for the new explicit-module `module_control_input` recovery.
- The regression clears flat module-local directions, then makes `controller` actor-port evidence claim `DATA_IN` is an output while the module body reads `DATA_IN` in state-body assignments.
- Expected behavior is sticky unresolved direction: `DATA_IN` keeps both `actor_port` and `module_control_input` evidence categories, `direction_hint` collapses to `None`, the module renderability blocks with the existing missing-direction diagnostic, and no `.fsm` text is emitted.
- This protects the recovery boundary: module-local reads may fill absent input roles, but contradictory evidence still requires upstream correction.
- Formatting, the focused new adapter test, the full adapter module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter explicit-module control-read recovery)
- Continued from commit `ee2f1b4`, moving from coverage-only true-FSM direct-root protection into a production adapter improvement for explicit module roots.
- Refactored output-target and control-read collection into reusable slice helpers shared by direct roots and explicit module candidates.
- Added `overlay_module_control_input_inventory`, which marks already-inventory module read signals as `module_control_input` inputs when they are not output/init targets.
- This keeps the recovery bounded: it does not add new ports, does not choose external actor perspectives, and does not mutate canonical `IntentIR`.
- Added a standalone explicit `controller` module regression where flat module-local directions are cleared, the module actor only provides clock/reset inputs plus `ACC` / `TRACE` outputs, and external actors drive `DATA_IN`, `GO`, and `DONE`.
- The selected root renders as `?fsm:controller`, with `DATA_IN`, `GO`, and `DONE` recovered from state-body assignments, transition guards, and standalone control blocks.
- Formatting, the focused new adapter test, the full adapter test module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter structured-FSM graph-read coverage)
- Continued from commit `b888297`, adding regression coverage for an existing true-FSM target-actor/control-read recovery path.
- The new structured-FSM test clears flat direct-interface direction hints and provides graph evidence where `controller` owns `ACC` / `TRACE`, reads `clk` / `rst_n`, external `environment` drives `DATA_IN` / `GO` / `DONE`, and `monitor` reads the outputs.
- The adapter must select `controller` from produced output targets, then recover `DATA_IN`, `GO`, and `DONE` as `direct_control_input` from state-body assignments, transition guards, and standalone control blocks.
- The rendered root remains a true `?fsm:explicit_fsm`; the regression checks emitted sequential assignment, guard, and standalone trace syntax.
- This is intentionally coverage-only: production already had the recovery path, but the true-FSM branch was not directly protected against future drift back to flat `direction_hint` dependence.
- Formatting, the focused new adapter test, the full adapter test module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 adapter blocked-top direction retention)
- Continued from commit `4c79c0b`, targeting the remaining adapter-side `direction_hint` cleanup work rather than widening backend scope.
- `analyze_top_renderability` now returns recovered top ports separately from the optional renderable top root, so link-topology direction recovery can survive even when top composition remains blocked for another reason.
- This closes a small but important artifact-quality gap: a width-only top port whose role is recoverable from `source -> top_port` or `top_port -> target` topology now stays recovered in `FsmTopCandidate.ports` and selected top signal inventory even if no `.fsm` target text is emitted.
- Added a regression with `consumer.result_data -> result_data` and an intentionally missing child module. The adapter stays blocked on `missing_module`, but the recovered `result_data` output direction remains visible for downstream review.
- Focused validation, the full adapter test module, docs CI, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-18 KG validation-finding payload expectations)
- Continued from commit `9e83612`, adding exact validation-finding payload expectations to the `kg-bench` fixture schema.
- `validation.<stage>.findings_include` can now match finding id, severity, category, summary substring, related ids that must be present, and related ids that must be absent.
- The five negative-knowledge prior-guided caution fixtures now prove their validation prior-match and rescan-guidance findings point at the exact local conflict/residual IDs that triggered the learned caution: `semantic_conflict_0001`, `temporal_conflict_0001`, `semantic_actor_boundary_inference`, `signal_connectivity_conflict_0001`, and `interface_signal_conflict_0001` / `interface_signal_conflict_0002`.
- Added a focused harness self-test for missing validation finding related IDs.
- Focused negative-knowledge fixture validation, `cargo test -p specforge kg_bench`, full tracked `kg-bench`, corpus-KB refresh, docs CI, full local CI, and `git diff --check` passed for this slice.

## Session update (2026-04-18 post-push continuity baseline sync)
- Continued from pushed commit `4dfb6b9`, updating live continuity state rather than Rust code.
- The post-push baseline is clean and aligned with `origin/main`; the previous KG table-support diagnostic batch is no longer in-flight.
- No architecture, extractor, validator, fixture, adapter, or mdBook behavior changed in this slice.
- The full local CI result from the pushed baseline remains the current behavior proof: formatting, warning-deny Clippy, `328` passing Rust tests, warning-deny rustdoc, and mdBook build.
- Docs-only validation for the continuity sync passed through docs CI, whitespace checking, and the README sentinel check.

## Session update (2026-04-18 KG IntentIR table-support missing-signal diagnostic coverage)
- Continued from commit `849e14a`, adding the IntentIR mirror for absent-signal `signal_supporting_table_ids_include` diagnostics.
- `kg_bench_reports_missing_intent_table_support_failure` expects `MISSING_INTENT_SIGNAL` table support in a one-row signal-table fixture that actually recovers `XREQ`.
- The focused `table_support_failure` filter now runs all four canonical table-support diagnostic cases: wrong SemanticIR support id, wrong IntentIR support id, missing SemanticIR signal, and missing IntentIR signal.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG canonical table-support missing-signal diagnostic coverage)
- Continued from commit `5cdc36e`, tightening the canonical `signal_supporting_table_ids_include` missing-signal diagnostic.
- The diagnostic now reports the exact `signal_supporting_table_ids_include[<signal>]` field and the actual canonical signal set when a fixture expects table support for a signal absent from the canonical inventory.
- Added `kg_bench_reports_missing_canonical_table_support_failure`, which expects `MISSING_SIGNAL` table support in a one-row signal-table fixture that actually recovers `XREQ`.
- The focused `table_support_failure` filter now runs the wrong SemanticIR support-id diagnostic, wrong IntentIR support-id diagnostic, and missing canonical signal diagnostic together.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG IntentIR table-support diagnostic coverage)
- Continued from commit `c2bbd8f`, adding focused negative diagnostic coverage for canonical `signal_supporting_table_ids_include` expectations at the `IntentIR` stage.
- `kg_bench_reports_intent_table_support_failure` reuses the structured one-row signal-table fixture helper, expects `XREQ` to carry `missing_intent_signal_table`, and confirms the failure reports the fixture, `intent` stage, `signal_supporting_table_ids_include[XREQ]`, missing table id, and actual `table_protocol_signal_description` support set.
- This complements the previous SemanticIR diagnostic test so the canonical table-provenance carry-through path is now guarded at both canonical stages.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG canonical table-support diagnostic coverage)
- Continued from commit `ae4ea54`, adding focused negative diagnostic coverage for canonical `signal_supporting_table_ids_include` expectations.
- `kg_bench_reports_canonical_table_support_failure` reuses the structured one-row signal-table fixture helper, expects `XREQ` to carry `missing_signal_table`, and confirms the failure reports the fixture, `signal_supporting_table_ids_include[XREQ]`, the missing table id, and the actual `table_protocol_signal_description` support set.
- The one-row signal-table fixture helper now accepts full expectation objects so EvidenceIR and canonical-stage diagnostic tests share the same realistic `SourceIR` table patch without duplicating setup.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG EvidenceIR table-provenance count diagnostic coverage)
- Continued from commit `69351aa`, adding focused negative diagnostic coverage for the EvidenceIR table-provenance count expectation.
- `kg_bench_reports_evidence_table_provenance_count_failure` uses the shared one-row signal-table fixture helper, expects `table_signal_declaration_provenance_count: 0`, and confirms the resulting failure reports the fixture, expectation field, expected count, and actual count.
- This completes the immediate EvidenceIR table-provenance expectation diagnostic trio: count mismatch, missing signal/table provenance record, and mismatched synthesized statement text.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG EvidenceIR table-provenance count expectation)
- Continued from commit `cc7e533`, adding a direct EvidenceIR provenance count expectation to `specforge kg-bench`.
- `EvidenceStageExpectations.table_signal_declaration_provenance_count` checks `EvidenceIr.table_signal_declaration_provenance.len()` directly.
- `table_misclassification_field_table_negative` now asserts count `0`, proving the deliberately misclassified `Bits | Name | Description` table does not create fake table-backed signal declarations or provenance.
- The fixture also asserts the persisted EvidenceIR validation metric `table_signal_declaration_provenance: 0`, keeping exact staged expectations and validator output aligned.
- Focused validation passed for the strengthened field-table negative fixture; full tracked `kg-bench`, `corpus-kb`, docs CI, full local CI, and `git diff --check` also passed for this slice.

## Session update (2026-04-17 KG EvidenceIR missing-provenance diagnostic coverage)
- Continued from commit `5ae15c4`, adding the sibling negative branch for the EvidenceIR table-provenance expectation path.
- `kg_bench_reports_missing_evidence_table_provenance_failure` builds a temporary fixture with a real structured signal table, then expects `XREQ` provenance from a deliberately wrong table id.
- The test proves missing `table_signal_declaration_provenance_include` records report the fixture name, expectation field, missing expected table id, and actual EvidenceIR table id.
- The two EvidenceIR provenance diagnostic tests now share a one-row signal-table fixture writer, reducing duplicated JSON patch setup while preserving realistic staged fixture behavior.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 KG EvidenceIR table-provenance diagnostic coverage)
- Continued from commit `487389e`, adding focused negative coverage for the new EvidenceIR table-provenance fixture expectation path.
- `kg_bench_reports_evidence_table_provenance_statement_failure` builds a temporary fixture with a real structured signal table and deliberately expects the wrong synthesized statement text for `XREQ`.
- The first focused run exposed that statement-text mismatch diagnostics did not name `table_signal_declaration_provenance_include`.
- The diagnostic now names that field for statement-text mismatches as well as missing provenance records, so fixture failures point at the relevant schema field.
- Focused validation, full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 EvidenceIR table provenance KG expectations)
- Continued from commit `43653a1`, adding exact EvidenceIR table-provenance fixture expectations rather than another aggregate metric.
- `specforge kg-bench` now accepts an `evidence` expectation section with `table_signal_declaration_provenance_include` entries.
- Each entry checks a required `signal_name` / `table_id` pair and can additionally verify the synthesized `statement_text` reached by the provenance record's `statement_id`.
- `signal_table_inventory_authority_negative` now proves that `XREQ`, `XACK`, and `PAYLOAD` each carry exact EvidenceIR provenance from `table_protocol_signal_description` before the canonical `supporting_table_ids` checks run.
- The focused fixture initially caught an incorrect `PAYLOAD` expectation of width `32`; the real synthesized statement is `Signal PAYLOAD is output width DATA_WIDTH.`, reflecting the table patch.
- Focused and full tracked `kg-bench`, `corpus-kb`, docs CI, and full local CI passed for this slice.

## Session update (2026-04-17 EvidenceIR table-signal provenance validation metric)
- Continued from commit `7d8e6e2`, adding EvidenceIR-side validation visibility for the table-signal provenance bridge.
- `specforge validate` now prints and persists `table_signal_declaration_provenance` for `EvidenceIR` by counting `EvidenceIr.table_signal_declaration_provenance`.
- The new focused validator regression builds markdown plus a structured `Signal | Direction | Width | Description` table and proves `XREQ`, `XACK`, and `PAYLOAD` each produce a provenance record linked to `table_protocol_signal_description`.
- `signal_table_inventory_authority_negative` now asserts the EvidenceIR validation metric value `3`, complementing the existing SemanticIR / IntentIR `with_table_support` metrics and exact canonical table-support expectations.
- This remains a visibility and regression metric only: it does not author truth, repair missing provenance, or promote facts.
- Focused validation passed for the new unit test and focused KG fixture; full tracked `kg-bench`, `corpus-kb`, docs CI, full local CI, and `git diff --check` also passed.

## Session update (2026-04-17 validator table-support unit coverage)
- Continued from commit `8d447e3`, adding direct unit coverage for the table-support validation metric rather than widening production behavior.
- `validate_semantic_and_intent_ir_count_signal_table_support` builds a real staged pipeline from a structured `Signal | Direction | Width | Description` table and checks that `SemanticIR` and `IntentIR` validation both report `with_table_support: 3`.
- This test complements `signal_table_inventory_authority_negative`: the fixture locks exact per-signal provenance and false-positive exclusion, while the unit test locks the validator metric over the canonical stages.
- Focused validation and full local CI passed; the suite now reports `320` Rust tests plus the mdBook build.

## Session update (2026-04-17 signal-table support validation metric)
- Continued from commit `2014352`, adding a narrow validator coverage metric over the already-committed table-provenance surface.
- `SemanticIR` validation now reports `with_table_support` by counting interface signal records with non-empty `supporting_table_ids`.
- `IntentIR` validation now reports `with_table_support` over declared non-`Low` signal records, matching the declared-signal coverage family.
- `signal_table_inventory_authority_negative` now asserts `with_table_support: 3` for both canonical stages while still requiring exact `signal_supporting_table_ids_include` expectations for `XREQ`, `XACK`, and `PAYLOAD`.
- Focused KG-bench validation, full `89/89` KG-bench validation, corpus-KB refresh, docs CI, full local CI, and `git diff --check` passed for this slice.

## Session update (2026-04-17 signal-table provenance carry-through)
- Continued from commit `e96ebe1`, adding a small IR provenance bridge plus KG expectation support.
- `EvidenceIR` now records `table_signal_declaration_provenance` when a structured signal-description table synthesizes a formal signal declaration statement.
- `SemanticIR` now resolves those synthesized statement ids back to table ids and stores them on `InterfaceSignalRecord.supporting_table_ids`; `IntentIR` carries the same canonical interface signal records forward.
- `specforge kg-bench` now supports `signal_supporting_table_ids_include` expectations for semantic and intent stages.
- `signal_table_inventory_authority_negative` now proves that `XREQ`, `XACK`, and `PAYLOAD` are not only recovered from the structured table with direction hints, but also retain `table_protocol_signal_description` as table support.
- Focused KG-bench validation, full `89/89` KG-bench validation, corpus-KB refresh, test compilation, docs CI, full local CI, and `git diff --check` passed for this slice.

## Session update (2026-04-17 signal-table inventory authority fixture)
- Continued from commit `664f2a5`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/signal_table_inventory_authority_negative`.
- The fixture encodes the protocol-PDF reality that real signal inventory usually comes from dedicated signal/interface tables.
- Its markdown source contains only document/integration context terms, while a structured `Signal | Direction | Width | Description` table supplies `XREQ`, `XACK`, and `PAYLOAD`.
- The fixture requires all three table signals to survive through `SemanticIR` and `IntentIR` with table-derived directions, and excludes `PDF`, `RTL`, `IP`, `VIP`, `PLL`, `DFT`, `CDC`, `CTS`, `ECO`, `SoC`, and `SOC` from canonical signal inventory.
- Focused and full KG-bench validation passed with `89/89` fixtures; `corpus-kb` refreshed table extraction/hygiene coverage to `9/9` and truthfulness-negative/caution coverage to `36/36`; full local CI passed with `319` Rust tests plus the mdBook build.

## Session update (2026-04-17 KG signal-inventory exclusion expectations)
- Continued from commit `4aa4183`, adding a narrow test-harness feature in `crates/specforge/src/commands/kg_bench.rs`.
- `CanonicalStageExpectations` now supports `signal_names_exclude`, letting fixtures assert that specific terms are absent from the canonical interface signal inventory at `SemanticIR` and `IntentIR`.
- This complements the existing `signal_names_include` check and makes negative signal-inventory truthfulness executable instead of relying on absence-by-inspection.
- The first hardened fixture is `clock_reset_contract_scope_negative`, which now excludes `PDF`, `RTL`, `IP`, `VIP`, `PLL`, `PLLs`, `DFT`, `SoC`, and `SOC` while still requiring `ACLK` and `ARESETN`.
- Focused and full KG-bench validation passed for the hardened fixture path, `corpus-kb` refreshed the tracked `88/88` fixture projections with `0` failures, and full local CI passed with `319` Rust tests plus the mdBook build.

## Session update (2026-04-17 clock/reset contract-scope KG fixture)
- Continued from commit `5d49cdb`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/clock_reset_contract_scope_negative`.
- The fixture makes the protocol-document scope boundary executable: the current document can define `ACLK` / `ARESETN` contract semantics for RTL and verification IP while explicitly saying final physical clock/reset tree construction belongs to the integrating SoC team.
- It preserves `ACLK` and active-low asynchronous `ARESETN` as first-class infrastructure signals, while requiring zero ordinary actor ports and zero concrete topology records through both `SemanticIR` and `IntentIR`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `88/88`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed benchmark, infrastructure, and semantic/truthfulness pattern projections from the `88`-fixture suite with `0` failures; infrastructure semantics coverage is `5/5`, infrastructure/polarity page coverage is `10/10`, and truthfulness-negative/caution coverage is `35/35`.
- Full local CI passed through formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 clock/reset protocol-document scope)
- Continued from commit `988e9bc`, with no Rust production-code change.
- The user clarified the intended domain boundary: AMBA, Intel, and similar protocol/chip-interface PDFs are contract documents for RTL designers and VIP creators, not complete sources for physical clock/reset tree construction.
- SPECFORGE should recover interface-visible clock/reset contract semantics from those PDFs: clock/reset identity, polarity, reset kind, asynchronous assertion / synchronous release discipline, and protocol-boundary timing obligations.
- Physical clock/reset trees remain team-specific SoC integration artifacts that depend on local clock generators, reset controllers, power domains, CDC/RDC policy, DFT/scan constraints, CTS, floorplan, and methodology.
- Future implementation should treat complete clock/reset tree synthesis as out of scope for protocol PDFs. Explicit topology phrases can remain bounded intent-level `infrastructure_topology` hints, but they are not signoff-quality physical tree recipes.
- Docs CI, full local CI with `319` Rust tests, and `git diff --check` passed for this documentation-only scope slice.

## Session update (2026-04-17 clock/reset generic-advice negative KG fixture)
- Continued from commit `4912ad7`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/clock_reset_generic_advice_negative`.
- The fixture preserves `ACLK` and active-low asynchronous `ARESETN` as first-class infrastructure signals while proving generic clock/reset doctrine does not create concrete `infrastructure_topology` records.
- The negative evidence covers glitch-avoidance clock-gate policy, no-glue reset-tree advice, possible synchronizer usage, and asynchronous assertion / synchronous release discipline.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `87/87`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed benchmark, infrastructure, and semantic/truthfulness pattern projections from the `87`-fixture suite with `0` failures; infrastructure semantics coverage is `4/4`, infrastructure/polarity page coverage is `9/9`, and truthfulness-negative/caution coverage is `34/34`.
- Full local CI passed through formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 FSMGEN response captured)
- FSMGEN responded to SPECFORGE's feedback in `/Users/richarddje/Documents/github/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`, observed at FSMGEN commit `7475f07`.
- SPECFORGE captured that response in `docs/FSMGEN_FEEDBACK.md` and `ROADMAP.md` so `.fsm` adapter planning can rely on the accepted sync contract.
- The concrete adapter posture is now explicit: target strict-mode canonical `.fsm`, block compatibility syntax by default, use FSMGEN's mdBook and regression corpus/support-accounting sources until machine-readable surfaces exist, and plan future adapter validation around capability manifests, stable diagnostic codes, check-only JSON, and normalized semantic JSON export.
- Longer-term `.fsm` language features remain valuable only when FSMGEN can parse, validate, normalize, document, support-account, and honestly lower or preserve them as checked metadata.

## Session update (2026-04-17 FSMGEN IntentIR-aligned feedback)
- `docs/FSMGEN_FEEDBACK.md` now frames SPECFORGE's feedback as `.fsm` language co-evolution, not only adapter validation tooling.
- The same handoff now introduces SPECFORGE explicitly as a Rust staged-IR toolchain whose product boundary is backend-independent `IntentIR`, so FSMGEN can read the feedback without prior project context.
- The added feature suggestions target natural `IntentIR` lowering: first-class system contracts, actor-relative ports, interface/channel grouping, temporal/stability contracts, semantic signal roles, generated assumptions/residual/provenance metadata, explicit direct-module root shape, and contract-aware composition.
- The support/tooling requests remain, but are now secondary to the language goal: capability manifest, JSON diagnostics, normalized AST/IR export, stable diagnostic codes, and adapter-facing examples make the richer `.fsm` contract executable.
- This preserves the SPECFORGE boundary: `IntentIR` remains canonical, but FSMGEN can evolve `.fsm` so fewer justified canonical facts are lost or blocked at adapter time.

## Session update (2026-04-17 FSMGEN reference sync)
- `subs/fsmgen` is now fast-forwarded from `57f00e5` to `955f2bb` for `.fsm` adapter reconnaissance.
- The updated FSMGEN reference includes its own live mdBook at `subs/fsmgen/docs/book/`, so SPECFORGE can consult both code and progressive user-facing documentation when widening `.fsm` lowering.
- The adapter-relevant FSMGEN direction is strict-mode/support-accounting, typed diagnostics, richer aggregate/type/package semantics, composition/toplink typing, and structural forward-IR separation.
- The SPECFORGE implementation stance stays unchanged: `IntentIR` remains canonical, `.fsm` is a downstream target, and FSMGEN is a read-only contextual reference from this repository.
- `docs/FSMGEN_FEEDBACK.md` is now the tracked handoff document for FSMGEN-facing feedback from SPECFORGE.
- Recommended FSMGEN feedback for future adapter leverage: machine-readable capability manifest, JSON check/diagnostic mode, normalized AST/IR export, richer reset/clock metadata, adapter-facing example corpus, and continued strict-mode-first support accounting.
- A local FSMGEN `./bin/ci-regression` run was intentionally stopped after the scope was clarified; it had reached `t/274-package-aggregate-values.t` with all reported tests green, so this sync is not recorded as a full FSMGEN validation pass.

## Session update (2026-04-17 direct `.fsm` control-read input recovery)
- Continued from commit `1a8c424`, completing the immediate pair to target-actor selection: after a direct root selects a target actor from graph-owned outputs, explicit control reads can now recover that target actor's input directions.
- `crates/specforge/src/ir/adapters.rs` now collects read references from DT guards/actions, rich control selectors/predicates/actions, and state-transition guards, then overlays `direct_control_input` only for local inventory signals that are not direct output targets.
- The new regression proves `DATA_IN` can be recovered as a target input from `DATA_OUT = DATA_IN` and a `DATA_IN` guard even when the structural KG only says an external `environment` drives `DATA_IN`.
- Existing standalone direct ambiguity, conflict, unrelated-actor, and output-owner tests remain green, keeping the recovery adapter-local and bounded.
- Full local CI passed through formatting, warning-deny Clippy, `319` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 direct `.fsm` target-actor selection)
- Continued from commit `14d6496`, deepening the `R15` graph-first adapter path rather than adding another flat direction-hint workaround.
- `crates/specforge/src/ir/adapters.rs` now selects a standalone direct target actor from graph-backed output-target ownership before falling back to the older one-actor direct context rule.
- The new regression clears flat direct-interface directions and proves the adapter still lowers when `controller` drives all direct outputs while an external `environment` drives an input and `monitor` reads the outputs.
- Existing standalone direct ambiguity, direction-conflict, width-conflict, unrelated-actor, and unambiguous-context tests remain green, keeping the widening bounded.
- Full local CI passed through formatting, warning-deny Clippy, `318` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 coordinated active-read coverage)
- Continued from commit `4cb4b53`, strengthening the read/sample side of the active-object extraction path.
- `relative_clause_actor_noise_negative` now uses one coordinated consumer sentence: `The Manager samples ARCHUNKEN and RCHUNKV.`
- Added focused regressions proving coordinated active-read clauses recover every sampled object and do not cross guard markers such as `when RVALID is HIGH`.
- Focused KG-bench validation still passes for `relative_clause_actor_noise_negative`, proving the staged `SemanticIR` / `IntentIR` graph carries both `Manager reads ARCHUNKEN` and `Manager reads RCHUNKV`.
- Full tracked KG-bench validation reports `86/86`, and full local CI passed through formatting, warning-deny Clippy, `317` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 relative-clause actor-noise KG fixture)
- Continued from commit `019f536`, with a production-code follow-on in `crates/specforge/src/ir/evidence.rs` plus a new tracked KG fixture under `crates/specforge/test_data/kg_quality/relative_clause_actor_noise_negative`.
- `relative_clause_actor_noise_negative` locks AXI-style chunking prose where an `interconnect` with a relative clause can drive both `ARCHUNKEN` and `RCHUNKV`.
- The fixture asserts graph direction, actor-signal relations, actor ports, zero signal-connectivity conflicts, and validation metrics through both `SemanticIR` and `IntentIR`.
- The first fixture run exposed that coordinated active-drive objects after the same verb only recovered the first signal; active object parsing now scans the verb's current clause so both `ARCHUNKEN` and `RCHUNKV` inherit the same head actor.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `86/86`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed benchmark and semantic/truthfulness corpus-KB pages from the `86`-fixture suite with `0` failures; actor-connectivity coverage is `12/12` and truthfulness-negative/caution coverage is `33/33`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `315` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 relative-clause actor extraction hygiene)
- Continued from commit `02d4be1`, with production-code changes in `crates/specforge/src/ir/evidence.rs` and `crates/specforge/src/ir/prior_memory.rs`.
- `extract_subject_phrase()` now limits active-drive subject parsing to the current sentence and strips relative clauses before choosing the candidate actor.
- `is_meaningful_actor_term()` rejects `mixture` / `mixture of`, so descriptive support phrases cannot become protocol actors or learned actor-taxonomy terms.
- Added a regression for AXI chunking prose proving `interconnect` remains the `ARCHUNKEN` actor and `mixture of` is not promoted.
- Rebuilt local AXI `EvidenceIR`, `SemanticIR`, and `IntentIR` from the existing generated `SourceIR` and validated the result; the live score remains `85/100 GOOD`, the fake `mixture of` producer is gone, and the remaining `ARCHUNKEN` conflict is `Manager` versus `interconnect`.
- Focused actor-extraction tests passed, and full local CI passed through formatting, warning-deny Clippy, `314` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 AHB exclusive/security wait-state stability KG fixture)
- Continued from commit `264be3e`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/ahb_exclusive_security_stability_gold`.
- `ahb_exclusive_security_stability_gold` locks AHB wait-state exclusive/security stability for `HEXCL`, `HNONSEC`, and `HEXOKAY`.
- The fixture uses `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables, proving `HEXCL`, `HNONSEC`, `HSEL`, `HREADY`, and `HEXOKAY` become graph-backed Manager/Subordinate actor relations and actor-relative ports.
- `HEXCL` and `HNONSEC` stability are Manager-grounded while `HEXOKAY` stability is Subordinate-grounded, all under `HREADY LOW` and `HSEL HIGH`, with no handshake-completion predicate.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `85/85`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `85`-fixture suite with `0` failures; AMBA-family coverage is `36/36` and temporal-family coverage is `42/42`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 AHB transfer/lock wait-state stability KG fixture)
- Continued from commit `64bc735`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/ahb_transfer_lock_stability_gold`.
- `ahb_transfer_lock_stability_gold` locks AHB wait-state transfer/lock stability for `HTRANS` and `HMASTLOCK`.
- The fixture uses `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables, proving `HTRANS`, `HMASTLOCK`, `HSEL`, and `HREADY` become graph-backed Manager/Subordinate actor relations and actor-relative ports.
- `HTRANS` and `HMASTLOCK` stability are Manager-grounded under `HREADY LOW` and `HSEL HIGH`, with no handshake-completion predicate, so stalled transfer/control obligations stay distinct from completed transfers.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `84/84`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `84`-fixture suite with `0` failures; AMBA-family coverage is `35/35` and temporal-family coverage is `41/41`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 APB address/protection wait-state stability KG fixture)
- Continued from commit `62fdca5`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/apb_address_protection_stability_gold`.
- `apb_address_protection_stability_gold` locks APB wait-state address/protection stability for `PADDR` and `PPROT`.
- The fixture uses a `Signal | Source | Width | Description` table plus prose actor relations, proving `PSEL`, `PENABLE`, `PREADY`, `PADDR`, and `PPROT` become graph-backed Requester/Completer actor relations and actor-relative ports.
- `PSEL` and `PREADY` resolve to valid-like / ready-like roles from local table descriptions; `PADDR` and `PPROT` stability are Requester-grounded under `PSEL HIGH`, `PENABLE HIGH`, and `PREADY LOW`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `83/83`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `83`-fixture suite with `0` failures; AMBA-family coverage is `34/34` and temporal-family coverage is `40/40`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 AXI address/response USER sideband stability KG fixture)
- Continued from commit `ac0e91b`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_address_response_user_sideband_stability_gold`.
- `axi_address_response_user_sideband_stability_gold` locks AXI address-channel and write-response `USER` sideband stability for `AWUSER`, `ARUSER`, and `BUSER`.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `AWVALID`, `AWREADY`, `AWUSER`, `ARVALID`, `ARREADY`, `ARUSER`, `BVALID`, `BREADY`, and `BUSER` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `AWVALID` / `AWREADY`, `ARVALID` / `ARREADY`, and `BVALID` / `BREADY` resolve to valid-like / ready-like roles from local table descriptions; `AWUSER` and `ARUSER` stability are Manager-grounded, while `BUSER` stability is Subordinate-grounded.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `82/82`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `82`-fixture suite with `0` failures; AMBA-family coverage is `33/33` and temporal-family coverage is `39/39`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-17 AXI data USER sideband stability KG fixture)
- Continued from commit `672c3b2`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_data_user_sideband_stability_gold`.
- `axi_data_user_sideband_stability_gold` locks AXI data-channel `USER` sideband stability for `WUSER` and `RUSER`.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `WVALID`, `WREADY`, `WUSER`, `RVALID`, `RREADY`, and `RUSER` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `WVALID` / `WREADY` and `RVALID` / `RREADY` resolve to valid-like / ready-like roles from local table descriptions; `WUSER` stability is Manager-grounded, while `RUSER` stability is Subordinate-grounded.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `81/81`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `81`-fixture suite with `0` failures; AMBA-family coverage is `32/32` and temporal-family coverage is `38/38`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI address QoS/region sideband stability KG fixture)
- Continued from commit `59881b3`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_address_qos_region_sideband_stability_gold`.
- `axi_address_qos_region_sideband_stability_gold` locks paired AXI address-channel QoS/region stability for `AWQOS`, `AWREGION`, `ARQOS`, and `ARREGION`.
- The fixture uses one width-only `Name | Width | Description` table plus prose actor relations, proving both write-address and read-address sideband sets become graph-backed Manager/Subordinate actor relations and actor-relative ports without direction columns.
- `AWVALID` / `AWREADY` and `ARVALID` / `ARREADY` resolve to valid-like / ready-like roles from local table descriptions, and each QoS/region stability rule requires the matching `HandshakeComplete(...)` predicate plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `80/80`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `80`-fixture suite with `0` failures; AMBA-family coverage is `31/31` and temporal-family coverage is `37/37`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-address control sideband stability KG fixture)
- Continued from commit `997da1a`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_address_control_sideband_stability_gold`.
- `axi_read_address_control_sideband_stability_gold` locks AXI read-address control sideband stability for `ARPROT`, `ARCACHE`, and `ARLOCK` when `ARVALID` and `ARREADY` complete the read-address channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `ARVALID`, `ARREADY`, `ARPROT`, `ARCACHE`, and `ARLOCK` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `ARVALID` and `ARREADY` resolve to valid-like / ready-like roles from local table descriptions, and each control-sideband stability rule requires `HandshakeComplete(ARVALID, ARREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `79/79`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `79`-fixture suite with `0` failures; AMBA-family coverage is `30/30` and temporal-family coverage is `36/36`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-address control sideband stability KG fixture)
- Continued from commit `b3c0820`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_address_control_sideband_stability_gold`.
- `axi_write_address_control_sideband_stability_gold` locks AXI write-address control sideband stability for `AWPROT`, `AWCACHE`, and `AWLOCK` when `AWVALID` and `AWREADY` complete the write-address channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `AWVALID`, `AWREADY`, `AWPROT`, `AWCACHE`, and `AWLOCK` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `AWVALID` and `AWREADY` resolve to valid-like / ready-like roles from local table descriptions, and each control-sideband stability rule requires `HandshakeComplete(AWVALID, AWREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `78/78`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `78`-fixture suite with `0` failures; AMBA-family coverage is `29/29` and temporal-family coverage is `35/35`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-address ID stability KG fixture)
- Continued from commit `b9f69d5`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_address_id_stability_gold`.
- `axi_read_address_id_stability_gold` locks AXI read-address ID stability for `ARID` when `ARVALID` and `ARREADY` complete the read-address channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `ARVALID`, `ARREADY`, and `ARID` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `ARVALID` and `ARREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `ARID` stability rule requires `HandshakeComplete(ARVALID, ARREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `77/77`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `77`-fixture suite with `0` failures; AMBA-family coverage is `28/28` and temporal-family coverage is `34/34`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-address ID stability KG fixture)
- Continued from commit `883366f`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_address_id_stability_gold`.
- `axi_write_address_id_stability_gold` locks AXI write-address ID stability for `AWID` when `AWVALID` and `AWREADY` complete the write-address channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `AWVALID`, `AWREADY`, and `AWID` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `AWVALID` and `AWREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `AWID` stability rule requires `HandshakeComplete(AWVALID, AWREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `76/76`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `76`-fixture suite with `0` failures; AMBA-family coverage is `27/27` and temporal-family coverage is `33/33`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-data ID stability KG fixture)
- Continued from commit `cf2fc42`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_data_id_stability_gold`.
- `axi_read_data_id_stability_gold` locks AXI read-data ID stability for `RID` when `RVALID` and `RREADY` complete the read-data channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `RVALID`, `RREADY`, and `RID` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `RVALID` and `RREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `RID` stability rule requires `HandshakeComplete(RVALID, RREADY)` plus a Subordinate-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `75/75`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `75`-fixture suite with `0` failures; AMBA-family coverage is `26/26` and temporal-family coverage is `32/32`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-response ID stability KG fixture)
- Continued from commit `11592f5`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_response_id_stability_gold`.
- `axi_write_response_id_stability_gold` locks AXI write-response ID stability for `BID` when `BVALID` and `BREADY` complete the write-response channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `BVALID`, `BREADY`, and `BID` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `BVALID` and `BREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `BID` stability rule requires `HandshakeComplete(BVALID, BREADY)` plus a Subordinate-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `74/74`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `74`-fixture suite with `0` failures; AMBA-family coverage is `25/25` and temporal-family coverage is `31/31`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-data response stability KG fixture)
- Continued from commit `14e53ed`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_data_response_stability_gold`.
- `axi_read_data_response_stability_gold` locks AXI read-data response stability for `RRESP` when `RVALID` and `RREADY` complete the read-data channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `RVALID`, `RREADY`, and `RRESP` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `RVALID` and `RREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `RRESP` stability rule requires `HandshakeComplete(RVALID, RREADY)` plus a Subordinate-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `73/73`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `73`-fixture suite with `0` failures; AMBA-family coverage is `24/24` and temporal-family coverage is `30/30`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-address sideband stability KG fixture)
- Continued from commit `fec0137`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_address_sideband_stability_gold`.
- `axi_read_address_sideband_stability_gold` locks AXI read-address sideband stability for `ARSIZE` and `ARBURST` when `ARVALID` and `ARREADY` complete the read-address channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `ARVALID`, `ARREADY`, `ARSIZE`, and `ARBURST` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `ARVALID` and `ARREADY` resolve to valid-like / ready-like roles from local table descriptions, and both sideband stability rules require `HandshakeComplete(ARVALID, ARREADY)` plus Manager-grounded `actor_maintains_signal_stable` consequents.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `72/72`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `72`-fixture suite with `0` failures; AMBA-family coverage is `23/23` and temporal-family coverage is `29/29`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-data last stability KG fixture)
- Continued from commit `d21252a`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_data_last_stability_gold`.
- `axi_write_data_last_stability_gold` locks AXI write-data last-beat stability for `WLAST` when `WVALID` and `WREADY` complete the write-data channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `WVALID`, `WREADY`, and `WLAST` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `WVALID` and `WREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `WLAST` stability rule requires `HandshakeComplete(WVALID, WREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `71/71`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `71`-fixture suite with `0` failures; AMBA-family coverage is `22/22` and temporal-family coverage is `28/28`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-data last stability KG fixture)
- Continued from commit `b0b20c1`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_data_last_stability_gold`.
- `axi_read_data_last_stability_gold` locks AXI read-data last-beat stability for `RLAST` when `RVALID` and `RREADY` complete the read-data channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `RVALID`, `RREADY`, and `RLAST` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `RVALID` and `RREADY` resolve to valid-like / ready-like roles from local table descriptions, and the `RLAST` stability rule requires `HandshakeComplete(RVALID, RREADY)` plus a Subordinate-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `70/70`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `70`-fixture suite with `0` failures; AMBA-family coverage is `21/21` and temporal-family coverage is `27/27`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-address sideband stability KG fixture)
- Continued from commit `746bf5d`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_address_sideband_stability_gold`.
- `axi_write_address_sideband_stability_gold` locks AXI write-address sideband stability for `AWLEN`, `AWSIZE`, and `AWBURST` when `AWVALID` and `AWREADY` complete the channel handshake.
- The fixture uses a width-only `Name | Width | Description` table plus prose actor relations, proving `AWVALID`, `AWREADY`, `AWLEN`, `AWSIZE`, and `AWBURST` become graph-backed Manager/Subordinate actor relations and actor-relative ports without a direction column.
- `AWVALID` and `AWREADY` resolve to valid-like / ready-like roles from local table descriptions, and all three sideband stability rules require `HandshakeComplete(AWVALID, AWREADY)` plus a Manager-grounded `actor_maintains_signal_stable` consequent.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `69/69`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `69`-fixture suite with `0` failures; AMBA-family coverage is `20/20` and temporal-family coverage is `26/26`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AHB write-data stability KG fixture)
- Continued from commit `598ed3f`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/ahb_write_data_stability_gold`.
- `ahb_write_data_stability_gold` locks AHB wait-state write-data stability for `HWDATA` when `HREADY` is low, `HSEL` is high, and `HWRITE` is high.
- The fixture asserts `HSEL`, `HWRITE`, `HWDATA`, and `HREADY` as graph-backed Manager/Subordinate actor relations and actor-relative ports using `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables.
- The temporal rule is actor-grounded to a `Manager` stability obligation and intentionally asserts `temporal_rules_with_handshake_completion = 0`, proving three-predicate write wait-state stability does not become a false completed-handshake predicate.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `68/68`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `68`-fixture suite with `0` failures; AMBA-family coverage is `19/19` and temporal-family coverage is `25/25`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AHB response stability KG fixture)
- Continued from commit `bbde6db`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/ahb_response_stability_gold`.
- `ahb_response_stability_gold` locks AHB wait-state response stability for `HRDATA` and `HRESP` when `HREADY` is low and `HSEL` is high.
- The fixture asserts `HSEL`, `HREADY`, `HRDATA`, and `HRESP` as graph-backed Manager/Subordinate actor relations and actor-relative ports using `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables.
- The two temporal rules are actor-grounded to `Subordinate` stability obligations and intentionally assert `temporal_rules_with_handshake_completion = 0`, proving response-side wait-state stability does not become a false completed-handshake predicate.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `67/67`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `67`-fixture suite with `0` failures; AMBA-family coverage is `18/18` and temporal-family coverage is `24/24`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AHB control stability KG fixture)
- Continued from commit `6801c19`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/ahb_control_stability_gold`.
- `ahb_control_stability_gold` locks AHB wait-state address/control stability for `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, and `HPROT` when `HREADY` is low and `HSEL` is high.
- The fixture asserts `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, `HPROT`, `HSEL`, and `HREADY` as graph-backed Manager/Subordinate actor relations and actor-relative ports using `Manager signals` / `Subordinate signals` section context plus `Destination`-column tables.
- The five temporal rules are actor-grounded to `Manager` stability obligations and intentionally assert `temporal_rules_with_handshake_completion = 0`, proving wait-state stability does not become a false completed-handshake predicate.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `66/66`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `66`-fixture suite with `0` failures; AMBA-family coverage is `17/17` and temporal-family coverage is `23/23`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 APB response stability KG fixture)
- Continued from commit `e9310c8`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/apb_response_stability_gold`.
- `apb_response_stability_gold` locks APB completed-access response stability for `PRDATA` and `PSLVERR` when `PSEL`, `PENABLE`, and `PREADY` are high.
- The fixture asserts `PSEL`, `PENABLE`, `PREADY`, `PRDATA`, and `PSLVERR` as graph-backed actor relations and actor-relative ports, with `PSEL` / `PREADY` resolved to valid-like / ready-like roles from table descriptions.
- The two temporal rules are actor-grounded to `Completer` stability obligations and require `HandshakeComplete(PSEL, PREADY)`, giving a contrast to the prior `PREADY LOW` wait-state fixture that intentionally had zero handshake-completion predicates.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `65/65`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `65`-fixture suite with `0` failures; AMBA-family coverage is `16/16` and temporal-family coverage is `22/22`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 APB write-control stability KG fixture)
- Continued from commit `599e577`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/apb_write_control_stability_gold`.
- `apb_write_control_stability_gold` locks APB wait-state stability for `PWRITE`, `PWDATA`, and `PSTRB` when `PSEL` and `PENABLE` are high and `PREADY` is low.
- The fixture asserts `PSEL`, `PENABLE`, `PREADY`, `PWRITE`, `PWDATA`, and `PSTRB` as graph-backed actor relations and actor-relative ports, with `PSEL` / `PREADY` resolved to valid-like / ready-like roles from table descriptions.
- The three temporal rules are actor-grounded to `Requester` stability obligations and intentionally assert `temporal_rules_with_handshake_completion = 0`, proving the wait-state guard does not become a false completed handshake.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `64/64`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `64`-fixture suite with `0` failures; AMBA-family coverage is `15/15` and temporal-family coverage is `21/21`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI sideband stability KG fixture)
- Continued from commit `ccfd99e`, with `30` Rust source files and `58,564` lines under `crates/specforge/src` after tightening the corpus-KB fixture-family classifier.
- `axi_sideband_stability_gold` locks sideband stability across two AXI channel fragments: `ARLEN` must remain stable when `ARVALID` / `ARREADY` complete, and `WSTRB` must remain stable when `WVALID` / `WREADY` complete.
- The fixture asserts graph-backed actor relations and actor ports for `ARVALID`, `ARREADY`, `ARLEN`, `WVALID`, `WREADY`, and `WSTRB`, resolved valid-like / ready-like semantic roles for the controlling handshakes, single-source semantic grounding, and actor-grounded stability consequents for `Manager` at both `SemanticIR` and `IntentIR`.
- `corpus_kb` fixture-family labeling now treats `stability` fixture names as temporal semantics, so sideband hold coverage appears on both the AMBA protocol page and the timing motif page.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `63/63`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `63`-fixture suite with `0` failures; AMBA-family coverage is `14/14` and temporal-family coverage is `20/20`.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-address timing KG fixture)
- Continued from commit `242caca`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_address_timing_gold`.
- `axi_read_address_timing_gold` locks an AXI read-address channel with a width-only signal table plus prose actor relations: `ARVALID`, `ARADDR`, and `ARLEN` are driven by `Manager`, `ARREADY` is driven by `Subordinate`, and reciprocal reads/samples remain graph-visible.
- The fixture asserts resolved valid-like / ready-like semantic roles, single-source semantic grounding, actor-grounded next-cycle `ARREADY` assertion, and actor-grounded `ARADDR` stability under `ARVALID` / `ARREADY` handshake completion at both `SemanticIR` and `IntentIR`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `62/62`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `62`-fixture suite with `0` failures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI write-data timing KG fixture)
- Continued from commit `f8c4faa`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_data_timing_gold`.
- `axi_write_data_timing_gold` locks an AXI write-data channel with a width-only signal table plus prose actor relations: `WVALID`, `WDATA`, and `WSTRB` are driven by `Manager`, `WREADY` is driven by `Subordinate`, and reciprocal reads/samples remain graph-visible.
- The fixture asserts resolved valid-like / ready-like semantic roles, single-source semantic grounding, actor-grounded next-cycle `WREADY` assertion, and actor-grounded `WDATA` stability under `WVALID` / `WREADY` handshake completion at both `SemanticIR` and `IntentIR`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `61/61`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `61`-fixture suite with `0` failures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-16 AXI read-data timing KG fixture)
- Continued from commit `3414a6d`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_read_data_timing_gold`.
- `axi_read_data_timing_gold` locks an AXI read-data channel with a width-only signal table plus prose actor relations: `RVALID`, `RDATA`, and `RRESP` are driven by `Subordinate`, `RREADY` is driven by `Manager`, and reciprocal reads/samples remain graph-visible.
- The fixture asserts resolved valid-like / ready-like semantic roles, single-source semantic grounding, actor-grounded next-cycle `RVALID` assertion, and actor-grounded `RDATA` stability under `RVALID` / `RREADY` handshake completion at both `SemanticIR` and `IntentIR`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `60/60`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `60`-fixture suite with `0` failures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-15 AXI write-response timing KG fixture)
- Continued from commit `bc110c3`, with no Rust production-code change and a new tracked KG fixture under `crates/specforge/test_data/kg_quality/axi_write_response_timing_gold`.
- `axi_write_response_timing_gold` locks an AXI write-response channel with a width-only signal table plus prose actor relations: `BVALID` and `BRESP` are driven by `Subordinate`, `BREADY` is driven by `Manager`, and reciprocal reads/samples remain graph-visible.
- The fixture asserts resolved valid-like / ready-like semantic roles, single-source semantic grounding, actor-grounded next-cycle `BVALID` assertion, and actor-grounded `BRESP` stability under `BVALID` / `BREADY` handshake completion at both `SemanticIR` and `IntentIR`.
- Focused KG-bench validation passed for the new fixture; the full tracked KG suite now reports `59/59`.
- `specforge corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality` refreshed the benchmark, AMBA-family, and timing corpus-KB pages from the `59`-fixture suite with `0` failures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-14 GitHub Actions manual-only cost control)
- Continued from commit `6cd5170` after the KG-bench semantic-grounding strength expectation slice.
- `.github/workflows/ci.yml` now exposes only `workflow_dispatch`, temporarily removing automatic `push` / `pull_request` triggers to conserve account GitHub Actions minutes.
- The hosted workflow still delegates to `./scripts/run_ci.sh`, so manual GitHub runs and local validation continue to exercise the same formatting, Clippy, warning-deny Rust test, warning-deny rustdoc, and mdBook gate.
- Trigger inspection confirms the workflow has no active automatic `push` or `pull_request` trigger keys.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-14 KG-bench semantic-grounding strength expectations)
- Continued from commit `ac7e3f4`, with `30` Rust source files and `58,559` lines under `crates/specforge/src` after extending KG-bench semantic-grounding strength expectations.
- `kg-bench` can now assert canonical `InterfaceSignalRecord.semantic_grounding_strength` directly at the `SemanticIR` and `IntentIR` stages through `semantic_grounding_strengths_include`.
- Strengthened `cross_modality_semantic_grounding_gold`, `vlm_timing_semantic_grounding_gold`, `visual_semantic_prior_guided_caption_gold`, and `apb_requester_completer_handshake_gold` so `XREQ`, `XACK`, `PSEL`, and `PREADY` lock exact single-source / cross-modality canonical grounding strength rather than only aggregate validation counters.
- Focused KG-bench validation passed for all four strengthened semantic-grounding fixtures; the full tracked KG suite reports `58/58` passing fixtures.
- Focused `kg_bench` Rust target passed with the tracked fixture suite reporting `58/58`, and full local CI passed through `bash scripts/run_ci.sh` with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-14 KG-bench resolved semantic-role shape expectations)
- Continued from commit `d85a061`, with `30` Rust source files and `58,530` lines under `crates/specforge/src` after extending KG-bench resolved semantic-role expectations.
- `kg-bench` can now assert canonical `InterfaceSignalRecord.resolved_semantic_role` directly at the `SemanticIR` and `IntentIR` stages through `resolved_semantic_roles_include`.
- Strengthened `cross_modality_semantic_grounding_gold`, `vlm_timing_semantic_grounding_gold`, `visual_semantic_prior_guided_caption_gold`, and `apb_requester_completer_handshake_gold` so `XREQ`, `XACK`, `PSEL`, and `PREADY` lock exact valid-like / ready-like canonical role shape rather than only aggregate role-presence metrics.
- Focused KG-bench validation passed for all four strengthened semantic-role fixtures; the full tracked KG suite reports `58/58` passing fixtures.
- Focused `kg_bench` Rust target passed with the tracked fixture suite reporting `58/58`, and full local CI passed through `bash scripts/run_ci.sh` with formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-14 KG-bench resolved signal-polarity shape expectations)
- Continued from commit `9cb41c8`, with `30` Rust source files and `58,500` lines under `crates/specforge/src` after extending KG-bench resolved signal-polarity expectations.
- `kg-bench` can now assert canonical `InterfaceSignalRecord.resolved_polarity` directly at the `SemanticIR` and `IntentIR` stages through `signal_polarities_include`.
- Strengthened `non_reset_control_polarity_gold`, `multi_control_polarity_gold`, and `mixed_control_polarity_gold` so `CS_N`, `WE_N`, and `ENABLE` lock exact active-low / active-high canonical polarity rather than only aggregate `with_resolved_polarity` metrics.
- Focused KG-bench validation passed for all three strengthened polarity fixtures; the full tracked KG suite reports `58/58` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-14 KG-bench signal-polarity conflict shape expectations)
- Continued from commit `010d9e2`, with `30` Rust source files and `58,471` lines under `crates/specforge/src` after extending KG-bench signal-polarity conflict expectations.
- `kg-bench` can now assert canonical `signal_polarity_conflicts` directly at the `SemanticIR` and `IntentIR` stages using partial matches over conflict signal, optional conflict id, observation polarity, observation source kind, supporting statement ids, and supporting table ids.
- Added `control_polarity_conflict_negative`, proving `PRESETN` active-high prose evidence and active-low signal-description-table evidence remain visible as a carried polarity conflict rather than forcing a winner.
- Focused KG-bench validation passed for the new polarity-conflict fixture; the full tracked KG suite reports `58/58` passing fixtures.
- The corpus-KB benchmark, pattern, and infrastructure/polarity fixture pages refreshed from the `58`-fixture suite with `0` failures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench interface-signal conflict shape expectations)
- Continued from commit `e1e629e`, with `30` Rust source files and `58,377` lines under `crates/specforge/src` after extending KG-bench interface-signal conflict expectations.
- `kg-bench` can now assert canonical `interface_signal_conflicts` directly at the `SemanticIR` and `IntentIR` stages using partial matches over conflict signal, optional conflict id, conflict kind, and included observation values plus supporting statement ids.
- Strengthened `negative_knowledge_prior_guided_interface_conflict_caution_gold` so the `DATA` conflict is asserted as a `direction_mismatch` over `input` / `output` and a `width_mismatch` over `8` / `16`, including under prior-memory caution.
- Focused KG-bench validation passed for the strengthened interface-conflict fixture; the full tracked KG suite reports `57/57` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench signal-connectivity conflict shape expectations)
- Continued from commit `5b56ce3`, with `30` Rust source files and `58,291` lines under `crates/specforge/src` after extending KG-bench signal-connectivity conflict expectations.
- `kg-bench` can now assert canonical `signal_connectivity_conflicts` directly at the `SemanticIR` and `IntentIR` stages using partial matches over conflict signal, optional conflict id, conflict kind, conflicting actor ids/names, and supporting statement ids.
- Strengthened `multi_producer_conflict_negative` and `negative_knowledge_prior_guided_connectivity_conflict_caution_gold` so the `PREADY` graph conflict is asserted as a `multiple_producers` conflict involving `Completer` and `Monitor`, including under prior-memory caution.
- Focused KG-bench validation passed for both strengthened connectivity-conflict fixtures; the full tracked KG suite reports `57/57` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench signal-semantic conflict shape expectations)
- Continued from commit `92344fa`, with `30` Rust source files and `58,215` lines under `crates/specforge/src` after extending KG-bench signal-semantic conflict expectations.
- `kg-bench` can now assert canonical `signal_semantic_conflicts` directly at the `SemanticIR` and `IntentIR` stages using partial matches over conflict signal, optional conflict id, and included observations by semantic tags, source kind, source text, and supporting statement/table/visual evidence ids.
- Strengthened `visual_sources_semantic_conflict_negative` so the `XCTRL` multimodal disagreement is asserted as visual-caption `handshake_valid_like` evidence versus VLM timing-diagram annotation `handshake_ready_like` evidence, with arbitration still non-decisive.
- Focused KG-bench validation passed for the strengthened visual semantic-conflict fixture; the full tracked KG suite reports `57/57` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench temporal-conflict shape expectations)
- Continued from commit `7490cb5`, with `30` Rust source files and `58,104` lines under `crates/specforge/src` after extending KG-bench temporal-conflict expectations.
- `kg-bench` can now assert canonical `temporal_conflicts` directly at the `SemanticIR` and `IntentIR` stages using partial matches over signal name, phase, clock signal, edge, cycle window, antecedents, conflicting values, supporting rule ids, and supporting statement ids.
- Strengthened `negative_knowledge_prior_guided_temporal_conflict_caution_gold` so the current-document `PREADY` post-tick `HIGH` / `LOW` contradiction under `HREADY LOW` stays asserted as typed conflict shape even when a matching negative-knowledge prior adds caution/rescan/corroboration guidance.
- Focused KG-bench validation passed for the strengthened temporal-conflict fixture; the full tracked KG suite reports `57/57` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench temporal-rule shape expectations)
- Continued from commit `f99956e`, with `30` Rust source files and `58,010` lines under `crates/specforge/src` after extending KG-bench temporal-rule expectations.
- `kg-bench` can now assert canonical `temporal_rules` directly at the `SemanticIR` and `IntentIR` stages using partial matches over source text, clock signal, edge, cycle window, supporting statement ids, antecedents, and consequents.
- Strengthened `axi_next_cycle_timing_gold`, `apb_setup_access_timing_gold`, and `ahb_wait_state_timing_gold` so representative AXI/APB/AHB timing fixtures now lock typed actor-grounded consequents, compound antecedents, handshake-completion predicates, and one-cycle windows directly.
- Focused KG-bench validation passed for all three strengthened timing fixtures; the full tracked KG suite reports `57/57` passing fixtures.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 KG-bench clock/reset infrastructure topology)
- Continued from commit `37cd89a`, with `30` Rust source files and `57,931` lines under `crates/specforge/src` after extending KG-bench infrastructure expectations.
- `kg-bench` can now assert canonical `infrastructure_signals` and `infrastructure_topology` records directly at the `SemanticIR` and `IntentIR` stages.
- Added `clock_reset_topology_gold`, proving explicit current-document clock-gate, reset-synchronizer, and reset-tree topology survives canonically while generic topology advice stays non-authoring and ordinary actor ports remain absent.
- The tracked KG-quality suite now reports `57/57` passing fixtures, and the corpus-KB benchmark plus infrastructure/polarity pages were refreshed from that suite.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 VLM timing motion-only annotation filtering)
- Continued from commit `7ce78ea`, with `30` Rust source files and `57,712` lines under `crates/specforge/src` after adding the VLM timing annotation filter.
- `SemanticIR` now treats non-quantitative waveform-motion prose in VLM timing `annotations[]` as annotation markup rather than timing law when no timing/constraint indicators or numeric/cycle anchors are present.
- The new `vlm_timing_motion_annotation_negative` KG fixture failed before the parser fix with two unexpected timing constraints, and now passes with zero timing constraints while preserving one concrete signal constraint / temporal rule from the document-grounded `HIGH` sample.
- Positive VLM annotation coverage still passes: fenced setup/hold-style timing annotations and direct VLM timing-note semantic grounding remain accepted.
- The tracked KG-quality suite now reports `56/56` passing fixtures, and the corpus-KB benchmark/timing/visual/pattern pages were refreshed from that suite.
- Full local CI passed through `bash scripts/run_ci.sh`, including formatting, warning-deny Clippy, `313` Rust tests under warning denial, warning-deny rustdoc, and the mdBook build.

## Session update (2026-04-13 R15g prior-candidate readiness manifest)
- Continued from commit `c2e6084`, with `30` Rust source files and `57,614` lines under `crates/specforge/src` after adding the prior-candidate readiness manifest path.
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes a tracked schema-versioned `corpus_kb/prior_candidates/kg-fixture-candidates.json` manifest beside the existing Markdown prior-candidate page.
- The manifest records candidate kind, target `CorpusMemory` schema, readiness, fixture counts, supporting/positive/guard fixture names, structured gate identifiers, and the explicit `review_only_no_corpus_memory_or_canonical_ir_mutation` promotion boundary.
- The Markdown prior-candidate page now mirrors this with a `Readiness Summary`, including `fixture_paired_review_ready` for paired prior fixture families and `caution_surface_review_ready` for caution-only negative-knowledge coverage.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, docs CI passed, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g typed prior-memory corpus KB page)
- Continued from commit `fd716eb`, with `30` Rust source files and `57,453` lines under `crates/specforge/src` after adding the typed prior-memory corpus-KB page family.
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/prior_memory/kg-fixtures.md` from the existing `typed prior memory` fixture-family label.
- The current tracked refresh projects `21/21` passing typed prior-memory fixtures spanning actor-taxonomy, semantic phrase, semantic modality-reliability, temporal phrase, table-shape, visual semantic, visual-motif, and caution-only negative-knowledge behavior.
- The page is review-only corpus synthesis. It preserves fixture provenance and cannot write `generated/prior_memory/corpus_memory.json`, mutate `CorpusMemory`, promote canonical IR, or approve prior records.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, docs CI passed, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g state-machine corpus KB page)
- Continued from commit `91492cc`, with `30` Rust source files and `57,433` lines under `crates/specforge/src` after adding the state-machine corpus-KB page family.
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/state_machines/kg-fixtures.md` from the existing `VLM state machines` fixture-family label.
- The current tracked refresh projects `5/5` passing state-machine fixtures: duplicate-initial merge, label-noise filtering, missing-initial warning, multiple-initial warning, and undeclared-transition endpoint filtering.
- The page is review-only corpus synthesis. It preserves fixture provenance and cannot promote VLM state-machine hypotheses into canonical IR, typed prior memory, validation scoring, or adapter lowering.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, docs CI passed, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g prior-candidate gate matrix)
- Continued from commit `2286f86`, with `30` Rust source files and `57,395` lines under `crates/specforge/src` after adding the prior-candidate gate review matrix.
- `specforge corpus-kb --kg-fixtures-root ...` now emits `review_scope: family_surface_not_individual_prior` in `corpus_kb/prior_candidates/kg-fixture-candidates.md`.
- The managed prior-candidate projection now includes a `Promotion Gate Review Matrix` with family-level schema, fixture, harvest, consumer, and non-mutation boundary gates for all seven `CorpusMemory` prior families.
- The matrix is deliberately review-only: it records visible implementation surfaces and still ends every row with `review_only_no_corpus_memory_or_canonical_ir_mutation`.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g semantic/truthfulness corpus KB patterns)
- Continued from commit `b3c05f5`, with `30` Rust source files and `57,326` lines under `crates/specforge/src` after adding the semantic/truthfulness pattern-page projection.
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/patterns/kg-fixtures.md` from the same tracked KG fixture run, selecting actor/connectivity, semantic role arbitration, negative-knowledge, truthfulness-negative/caution, and residual/caveat fixture labels.
- The current live refresh projects `42/42` passing semantic/truthfulness pattern fixtures with explicit fixture-path provenance while preserving human synthesis outside the managed block.
- The page is corpus synthesis only: it can guide review, debugging, benchmark design, and arbitration work, but it does not affect fixture execution, validation scoring, `CorpusMemory`, or canonical IR.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g prior-candidate corpus KB projection)
- Continued from commit `0f3166f`, with `30` Rust source files and `57,283` lines under `crates/specforge/src` after adding the prior-candidate projection path.
- `specforge corpus-kb --kg-fixtures-root ...` now refreshes `corpus_kb/prior_candidates/kg-fixture-candidates.md` from KG fixtures that already encode prior-guided gold/negative/caution behavior.
- The managed projection groups candidate kinds by target `CorpusMemory` schema surface, supporting fixture count, positive fixtures, guard/caution fixtures, and required gates while keeping `promotion_status: candidate_not_promoted_review_required`, `canonical_mutation_allowed: false`, and `corpus_memory_mutation_allowed: false`.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g dedicated corpus KB fixture-family pages)
- Continued from commit `8681b13`, with `30` Rust source files and `57,076` lines under `crates/specforge/src` after the multi-page corpus-KB refresh update.
- The KG fixture refresh now writes the aggregate benchmark page plus dedicated family pages under `corpus_kb/tables/`, `corpus_kb/visuals/`, `corpus_kb/timing/`, `corpus_kb/infra/`, and `corpus_kb/protocols/`.
- The current tracked refresh projects table fixtures `8/8`, visual fixtures `18/18`, timing fixtures `14/14`, infrastructure/polarity fixtures `6/6`, and AMBA-family fixtures `9/9`, all with fixture-path provenance and all remaining review-only corpus synthesis.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g KG fixture-family corpus KB summary)
- Continued from commit `62be333`, with `30` Rust source files and `56,860` lines under `crates/specforge/src` after the corpus-KB family-summary projection update.
- `specforge corpus-kb --kg-fixtures-root ...` now renders a managed fixture-family summary table before the per-fixture KG benchmark list, so the corpus KB exposes coverage across VLM timing/state-machine, actor connectivity, multimodal visual grounding, negative knowledge, polarity, AMBA-family protocol, semantic arbitration, table hygiene, temporal, truthfulness-negative, and typed-prior-memory families.
- The family summary is review-facing synthesis only. It can help humans and future sessions see what the tracked benchmark suite covers, but it does not change `kg-bench` execution, validation scoring, canonical IR, or `CorpusMemory`.
- Focused corpus-KB tests passed, the live corpus-KB refresh command passed with the tracked `55/55` fixture suite and `0` failures, and the full local CI wrapper passed with `313` Rust tests.

## Session update (2026-04-13 R15g KG fixture-result corpus KB projection)
- Continued from the README/PNT flow after commit `1229426`, with `30` Rust source files and `56,601` lines under `crates/specforge/src` after the CLI/benchmark seam update.
- `specforge corpus-kb` now accepts optional `--kg-fixtures-root <fixture-root>` plus repeated `--kg-fixture <fixture>` selectors, so R15g can refresh a tracked benchmark-result page without requiring validation report inputs.
- The new `corpus_kb/benchmarks/kg-fixtures.md` managed page records the current tracked KG fixture run: `55` total fixtures, `55` passed, `0` failed, with fixture paths preserved as provenance and human synthesis kept outside the managed block.
- `kg_bench` now exposes an internal fixture-outcome collection seam for this projection while preserving the standalone `specforge kg-bench` command behavior.
- The former noisy-output caveat on the KG fixture-result projection is now closed by the quiet fixture-local validation path; future cleanup can still improve fixture-family summarization, but projection output no longer dumps full validation reports.
- Focused `corpus_kb` and `kg_bench` tests passed, the KG fixture-result refresh command passed with `55/55` fixtures, and the full local CI wrapper passed with `311` Rust tests.

## Session update (2026-04-13 R15g quiet KG fixture validation path)
- Continued from commit `84500f5`, with `30` Rust source files and `56,671` lines under `crates/specforge/src` after the validator-output guard update.
- `validate::run_quiet()` now wraps the existing user-facing validator in a thread-local output guard, so internal callers can persist validation sidecars/backannotations without printing the full validation report.
- `kg-bench` now uses that quiet path for fixture-local validation expectations. This keeps `specforge kg-bench` and `specforge corpus-kb --kg-fixtures-root ...` focused on fixture outcomes while preserving the same validation semantics and artifacts.
- The output guard restores its previous state through `Drop`; focused validation passed for the guard, fixture-failure output, corpus-KB benchmark refresh, the live `55/55` corpus-KB refresh command, the full `kg_bench` test target, and the full local CI wrapper with `312` Rust tests.

## Session update (2026-04-13 R15g corpus knowledge-base bootstrap)
- Executed the README handoff through `SESSION_BOOTSTRAP.md`, reviewed the referenced live docs, and rechecked the active Rust surface: `30` Rust source files and `56,332` lines under `crates/specforge/src` after adding the new command module.
- `specforge corpus-kb <validation-report>...` is now wired into the CLI to refresh tracked corpus knowledge-base pages under `corpus_kb/` from reviewable validation report sidecars.
- The first `R15g` page family is `corpus_kb/failures/validation-findings.md`; its managed block is auto-refreshed from the four live AMBA validation reports while the human synthesis section remains outside the managed block.
- The corpus KB boundary is explicit in code and docs: it can inform humans, future LLM sessions, benchmark ideas, rescan design, and typed-prior candidates, but it cannot directly mutate canonical IR or become a prior without a separate validation-gated path.
- Focused `corpus_kb` tests passed, the refresh command passed on the four live validation report sidecars, and the full local CI wrapper passed with `307` Rust tests.

## Session update (2026-04-12 VLM timing signal bit-select annotation filtering)
- Recovered the interrupted README-bootstrap/PNT state after the crash, re-read the referenced continuity docs, and rechecked the active Rust surface: `29` Rust source files and `56,040` lines under `crates/specforge/src` before documentation edits.
- `SemanticIR` now extends the VLM timing spurious-annotation filter to standalone signal bit-select/range labels such as `XREQ[0]`, `XREQ<1>`, and `XREQ[3:0]`.
- The change is deliberately scoped to VLM timing `annotations[]` label noise. Grounded `signals[].values[]` observations for the same document signal still become typed `SignalConstraintRecord`s and temporal `SignalValue` predicates when the sampled value is concrete.
- The direct semantic regression and `vlm_timing_spurious_annotation_negative` KG fixture now cover this edge case while keeping the tracked fixture count at `55`; focused validation, targeted `kg-bench`, and the full local CI wrapper passed with `305` Rust tests.

## Session update (2026-04-12 README bootstrap refresh)
- Executed the README handoff through `SESSION_BOOTSTRAP.md`, re-read the referenced live docs, and compared the current Rust implementation surface against the documentation map.
- The active Rust surface now contains `29` Rust source files and `56,024` lines under `crates/specforge/src`, including the full command set wired through `cli.rs`, `commands/mod.rs`, and `lib.rs`: `inspect`, `doctor`, `converge`, `ingest`, `evidence`, `semantic`, `intent`, `adapt`, `enrich`, `validate`, `project-validation`, `rescan-plan`, `kg-bench`, `learn-priors`, and `nlp-enrich`.
- No Rust architecture drift requiring code changes was found, but the README implementation-path map and this analysis file's current testing subsection lagged the codebase; this refresh aligns them with the `76274f6` baseline, `55` tracked KG-quality fixtures, and the latest full local CI result of `305` passing Rust tests.

## Session update (2026-04-12 NLP alias markdown/list marker filtering)
- `extract_alias_phrase()` now rejects broader Form 2 alias subjects that begin with markdown bullets, block quotes, and ordered-list markers such as `*`, `+`, `>`, `1.`, and `2)` in addition to the existing `-`, `|`, and `#` guards.
- This keeps list/table formatting out of `signal_alias_map` while preserving ordinary prose alias learning for noun phrases like `address bus`.
- Focused validation, warning-deny Clippy, and the full local CI gate passed with `305` Rust tests for this slice.

## Session update (2026-04-12 VLM timing bracketed sample label filtering)
- `SemanticIR` now filters bracketed VLM timing annotation labels such as `D[0]`, `A[1]`, `DATA[3]`, and `ADDR[7]` as low-value waveform/sample markup instead of allowing them to become `TimingConstraintRecord`s.
- The existing `vlm_timing_spurious_annotation_negative` fixture and direct semantic regression now cover that bracketed label family while preserving real signal-value observations as typed temporal evidence.
- Focused validation, targeted `kg-bench` validation, and the full local CI gate passed with `305` Rust tests for this slice.

## Session update (2026-04-12 VLM timing compact sample label filtering)
- `SemanticIR` now filters compact VLM timing annotation labels such as `D0`, `A1`, `DATA0`, and `0xAA` as low-value waveform/sample markup instead of allowing them to become `TimingConstraintRecord`s.
- The existing `vlm_timing_spurious_annotation_negative` fixture and direct semantic regression now cover that compact label family while preserving real signal-value observations as typed temporal evidence.
- Focused validation, targeted `kg-bench` validation, and the full local CI gate passed with `305` Rust tests for this slice.

## Session update (2026-04-12 missing initial FSM state cardinality warning)
- Added `vlm_state_machine_missing_initial_negative`, proving VLM-authored `IDLE` / `BUSY` state evidence with no initial marker keeps the state graph and transition visible while validation reports unsafe initial-state cardinality at both semantic and intent stages.
- Added a direct explicit-state validation regression for the same zero-initial path and shared the staged IR build helper with the multiple-initial regression.
- Focused validation, targeted `kg-bench` validation, and the full local CI gate passed with `305` Rust tests for this slice.

## Session update (2026-04-12 initial FSM state cardinality warning)
- `specforge validate` now emits `semantic_state_machine_initial_cardinality` and `intent_state_machine_initial_cardinality` warnings when a canonical state graph exists but has zero or multiple initial states.
- Added `vlm_state_machine_multiple_initial_negative`, proving VLM-authored `IDLE` and `BUSY` multiple-initial evidence keeps the graph visible while validation flags the unsafe cardinality at both semantic and intent stages.
- Focused validation, targeted `kg-bench` validation, and the full local CI gate passed with 304 Rust tests for this slice.

## Session update (2026-04-12 initial FSM state validation metric)
- `specforge validate` now reports `initial_regular_states` for both `SemanticIR` and `IntentIR`, making FSM initial-state cardinality visible beside total state and transition counts.
- The tracked `vlm_state_machine_duplicate_initial_gold` fixture now asserts `initial_regular_states = 1` at both semantic and intent validation stages, so the duplicate-label merge path proves exactly one canonical initial state survives.
- Focused semantic validation, targeted `kg-bench` validation, and the full local CI gate passed with 303 Rust tests for this slice.

## Session update (2026-04-12 VLM state-machine duplicate initial markers)
- `SemanticIR` now merges duplicate VLM state-machine state labels by state name before adding canonical `RegularStateRecord`s, preserving `is_initial` if any duplicate carries it.
- `kg-bench` can now assert canonical initial-state names directly, and the tracked `vlm_state_machine_duplicate_initial_gold` fixture proves `IDLE` remains the single initial state when a duplicate later marks it as initial while `BUSY` stays non-initial.
- Focused validation passed for the semantic regression, targeted `kg-bench` validation passed for `vlm_state_machine_duplicate_initial_gold`, and the full local CI gate passed with 303 Rust tests.

## Session update (2026-04-11 VLM state-machine undeclared-transition filtering)
- `SemanticIR` now requires VLM state-machine transition `from` / `to` endpoints to reference state names accepted from the same `vlm_state_machine_extraction` observation before adding `StateTransitionRecord` entries.
- Added `vlm_state_machine_observation_rejects_undeclared_transition_endpoints` plus the tracked `vlm_state_machine_undeclared_transition_negative` fixture, proving `IDLE` / `BUSY` and `IDLE->BUSY` survive while undeclared identifier-shaped endpoints such as `DONE` and `RESET` are filtered.
- Focused validation passed for the semantic regression, targeted `kg-bench` validation passed for `vlm_state_machine_undeclared_transition_negative`, and the full local CI gate passed with 302 Rust tests.

## Session update (2026-04-11 VLM state-machine label-noise filtering)
- `SemanticIR` now parses VLM state-machine `states[].name` labels and transition `from` / `to` endpoints through the canonical identifier parser before adding `RegularStateRecord` or `StateTransitionRecord` entries, so prose labels such as `IDLE state` and `ACCESS phase` stay out of canonical FSM state.
- `kg-bench` can now assert canonical state names and state-transition endpoint keys directly, and the tracked `vlm_state_machine_label_noise_negative` fixture locks that clean `IDLE` / `BUSY` evidence survives while noisy prose labels are filtered.
- Focused validation passed for the semantic regression and targeted `kg-bench` validation passed for `vlm_state_machine_label_noise_negative`; the full local CI gate passed with 301 Rust tests for this slice.

## Session update (2026-04-11 VLM timing compact edge spelling variants)
- `SemanticIR` now rejects abbreviated and compact VLM timing waveform-motion spellings such as `POS_EDGE`, `NEG_EDGE`, `risingedge`, `LOW2HIGH`, and `HIGH2LOW` before the generic symbolic-value fallback can promote them into false signal values.
- Strengthened `vlm_timing_diagram_observation_rejects_waveform_motion_states` and `vlm_timing_waveform_motion_negative` so the mixed waveform still produces exactly one VLM-authored signal constraint and temporal rule: the concrete `HIGH` sample.
- Focused validation passed for the semantic regression, targeted `kg-bench` validation passed for `vlm_timing_waveform_motion_negative`, and the full local CI gate passed with 300 Rust tests for this slice.

## Session update (2026-04-11 VLM timing waveform motion spelling variants)
- `SemanticIR` now normalizes VLM timing motion states across spaces, underscores, and hyphens before applying the waveform-motion filter, so identifier-shaped values such as `RISING_EDGE`, `LOW_TO_HIGH`, and `HIGH_TO_LOW` do not become false symbolic signal values.
- Strengthened `vlm_timing_diagram_observation_rejects_waveform_motion_states` and `vlm_timing_waveform_motion_negative` to cover those separator variants while keeping the concrete `HIGH` sample as the only surviving signal constraint and temporal rule.
- Focused validation passed for the semantic regression, targeted `kg-bench` validation passed for `vlm_timing_waveform_motion_negative`, and the full local CI gate passed with 300 Rust tests.

## Session update (2026-04-11 VLM timing waveform motion filtering)
- `SemanticIR` VLM timing signal-value lifting now filters waveform motion descriptors such as `rising`, `falling`, `stable`, `steady`, `unchanged`, and `toggle` before the generic symbolic-value fallback can promote them into false `MustBeValue` facts.
- Concrete VLM timing values still flow through the bounded path: `HIGH`, `LOW`, `ASSERTED`, `DEASSERTED`, `0`, and `1` remain eligible for typed signal constraints and temporal predicates when the signal name is document-grounded.
- Added `vlm_timing_diagram_observation_rejects_waveform_motion_states` plus tracked KG fixture `vlm_timing_waveform_motion_negative`, proving a mixed `rising` / `HIGH` / `stable` / `falling` / `UNCHANGED` waveform creates exactly one concrete signal constraint and temporal rule.
- Focused validation passed for the semantic regression, targeted `kg-bench` validation passed for `vlm_timing_waveform_motion_negative`, and the full local CI gate passed with 300 Rust tests.

## Session update (2026-04-11 SemanticIR sticky interface conflicts)
- `InterfaceSignalAccumulator` now tracks whether direction and width hints already conflicted, so `None` no longer ambiguously means both unknown and conflict-collapsed inside the canonical interface builder.
- Strengthened `surfaces_interface_signal_conflicts_for_conflicting_explicit_declarations` with an input/output/input width 8/16/8 declaration sequence; `DATA.direction_hint` and `DATA.width_hint` remain unresolved instead of self-healing to the repeated original value.
- Focused validation passed for the strengthened regression, the full `ir::semantic::tests` suite passed with 65 semantic tests, and the full local CI gate passed with 299 Rust tests.

## Session update (2026-04-11 actor-port width conflict stickiness)
- Added `standalone_dt_keeps_conflicting_actor_port_width_unresolved`, proving sticky adapter inventory conflicts cover numeric width as well as direction roles.
- The regression starts with canonical flat `DATA_OUT` width `8`, overlays graph-backed `DATA_OUT` width `16`, then repeats graph-backed width `8`; lowering must remain blocked with `DATA_OUT.width_hint == None` instead of resurrecting the original width.
- Focused validation passed for the new regression, the full `ir::adapters::tests` suite passed with 26 adapter tests, and the full local CI gate passed with 299 Rust tests.

## Session update (2026-04-11 actor-port conflict stickiness)
- `FsmSignalCandidate` inventory evidence now tracks whether a direction or width hint already conflicted, so `None` no longer ambiguously means both unknown and conflict-collapsed.
- This prevents later repeated graph/interface evidence from resurrecting a signal role after disagreement; once a direct actor-port overlay sees incompatible `DATA_OUT` directions, the candidate stays unresolved and lowering blocks.
- Added `standalone_dt_keeps_conflicting_actor_port_direction_unresolved`, proving repeated same-actor `DATA_OUT` evidence in the order `Output -> Input -> Output` remains blocked instead of self-healing to `Output`.
- Focused validation passed for the new regression, the full `ir::adapters::tests` suite passed with 25 adapter tests, and the full local CI gate passed with 298 Rust tests.

## Session update (2026-04-11 graph-backed sequential system directions)
- Added an adapter regression proving standalone sequential DT lowering remains renderable after clearing flat direct-interface direction hints when a single `controller` actor-port graph supplies directions for `clk`, `rst_n`, `DATA_IN`, and `ACC`.
- This locks the system-contract renderability path, not just ordinary data-port rendering: `clk` and `rst_n` are recovered as graph-backed inputs, `(+system ...)` still emits, and `fsm_adapter_system_contract` is not produced.
- Focused validation passed for `standalone_sequential_dt_recovers_system_directions_from_actor_ports`, the full `ir::adapters::tests` suite passed with 24 adapter tests, and the full local CI gate passed with 297 Rust tests.

## Session update (2026-04-11 mixed clause-local control polarity)
- `EvidenceIR` now has a bounded fallback that can split mixed active-level prose such as `CS_N is active LOW and ENABLE is active HIGH` into clause-local polarity observations.
- The fallback is intentionally strict: every mentioned known signal must be recovered, each recovered polarity clause must name exactly one known signal, and detached polarity wording such as `CS_N is active LOW and active HIGH` stays unresolved instead of borrowing an implicit subject.
- The whole-statement detector still returns no polarity for text containing both active-low and active-high cues; clause-local recovery is a separate guarded path rather than a broad global guess.
- `mixed_control_polarity_gold` locks the path through the KG benchmark surface with two declared canonical signal records, zero heuristic duplicates, two resolved polarities, and zero polarity/temporal conflicts.
- Focused validation passed for the mixed-clause positive case, the detached-polarity negative case, the detector guard, targeted `kg-bench mixed_control_polarity_gold`, the full 48-fixture KG benchmark suite, and the full local CI gate with 296 Rust tests.

## Session update (2026-04-11 detached mixed polarity negative fixture)
- `detached_mixed_control_polarity_negative` now locks the conservative mirror case for `CS_N is active LOW and active HIGH` through the tracked KG benchmark surface.
- The fixture proves the signal remains declared and constrained, but `with_resolved_polarity` stays `0` through `SemanticIR` and `IntentIR`, with zero heuristic duplicates, zero polarity conflicts, and zero temporal conflicts.
- Targeted `kg-bench detached_mixed_control_polarity_negative` passed, the full `kg_bench` test passed with 49 tracked fixtures, and the full local CI gate passed with 296 Rust tests.

## Session update (2026-04-11 collective control polarity)
- `EvidenceIR` now recovers unambiguous collective active-level prose such as `CS_N and WE_N are active LOW signals`, producing polarity observations for both declared controls and refining asserted/deasserted constraints after polarity is grounded.
- Mixed low/high compound prose remained unresolved in that slice until clause-local parsing could safely bind each signal to exactly one polarity phrase.
- `SemanticIR` now treats polarity-only co-mentions of already declared signals as enrichment of the authoritative records rather than low-confidence heuristic interface groups, preventing duplicate canonical signal records and inflated polarity metrics.
- `multi_control_polarity_gold` locks the path through the KG benchmark surface with two resolved polarities and zero polarity/temporal conflicts.
- Focused validation passed for the new evidence regressions, semantic duplicate-suppression regression, targeted `kg-bench multi_control_polarity_gold`, the full 47-fixture KG benchmark suite, and the full local CI gate with 295 Rust tests.

## Session update (2026-04-11 asserted-when-level control polarity)
- `EvidenceIR` polarity detection now recognizes explicit local phrases such as `asserted when LOW`, `LOW when asserted`, `asserted by driving LOW`, and their active-high mirrors
- the new path is intentionally evidence-grounded: it does not infer active-low from `_N` suffixes alone
- focused regressions prove the detector accepts asserted-when-level wording and that a non-reset control signal `CS_N` refines asserted/deasserted constraints to LOW/HIGH only after local prose says it is asserted when LOW
- `SemanticIR` now suppresses redundant one-signal heuristic interface candidates when that signal is already explicitly declared, preventing local polarity prose from double-counting the same canonical signal
- focused `cargo test --manifest-path Cargo.toml polarity -- --nocapture` passed with 21 polarity tests, `cargo test --manifest-path Cargo.toml kg_bench -- --nocapture` passed with all 46 KG fixtures, and `bash scripts/run_ci.sh` passed with 292 Rust tests plus docs for that slice

## Session update (2026-04-11 KG-bench graph direction expectations)
- `specforge kg-bench` canonical stage expectations now include `graph_direction_signal_names_include` and `graph_direction_signal_names_exclude`
- those expectations inspect canonical `actor_ports` for non-`unknown` direction evidence by signal name, which lets fixtures test graph-backed direction coverage directly without overloading flat `InterfaceSignalRecord.direction_hint`
- the existing `signal_directions_include` expectation remains compatibility-specific; this matters because actor-relative producer/consumer roles should not be flattened into a fake global direction when a fixture really means flat interface perspective
- `actor_ports_gold` now exercises the new surface for both `SemanticIR` and `IntentIR`
- the full local CI path passed with 288 Rust tests, warning-deny Clippy, warning-deny rustdoc, and mdBook build

## Session update (2026-04-11 bootstrap refresh)
- executed the README terminal instruction by reading `SESSION_BOOTSTRAP.md`, which expands the task into reading the referenced live docs, analyzing the Rust codebase, updating this analysis if necessary, and continuing from the roadmap
- reviewed the high-signal live docs (`README.md`, `SESSION_BOOTSTRAP.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `VALIDATION_SNAPSHOT.md`, `INTENTIR_SPEC.md`, `MEMORY.md`, `COMMIT.md`) and compared their claims against the current CLI/module surface
- observed the current Rust implementation under `crates/specforge/src`: 28 Rust files and about 53,858 lines across the active single-crate implementation
- corrected stale analysis claims around the CLI command list, `rescan-plan`, schema-v2 rescan execution, promotion-review boundaries, `CorpusMemory` schema v5 prior families, KG fixture count, and current test count
- re-ran the README bootstrap sanity pass after `35a8372` and found no new Rust architecture drift beyond continuity/doc hygiene: `MEMORY.md` needed the latest committed baseline and `USER_GUIDE.md` had stray example bullets under its root-doc list
- the codebase remains a real end-to-end staged IR pipeline with a growing learning/rescan plane, but it is not yet a universal chip-spec-PDF oracle; the honest next pressure remains graph-first semantics, temporal/clock-reset truthfulness, multimodal arbitration, KG-quality expansion, and corpus-level learning without cross-document fact leakage

## Session update (2026-04-11 dead-code warning cleanup)
- removed the last five dead-code warning sources instead of suppressing them
- the deleted adapter helpers were stale legacy paths for `DecisionTreeFragmentRecord` block/action validation and direct `DecisionTreeActionRecord` rendering; the active `.fsm` lowering path now validates and renders `ControlBlockRecord` / `ControlActionRecord` instead
- the deleted semantic helpers were empty register/timing builder stubs with no call sites; `SemanticIR` already carries register records from `EvidenceIR` and extends timing records with VLM timing extraction directly in `SemanticIr::build()`
- targeted validation with `cargo test --manifest-path Cargo.toml --lib` now reports 282 passing tests and no dead-code warning output

## Session update (2026-04-11 warning-deny CI)
- `scripts/run_ci.sh` now runs `cargo test --manifest-path Cargo.toml` with `RUSTFLAGS="-D warnings"`
- the warning-deny gate lives in the shared script, so local pre-push validation and manually launched GitHub Actions runs enforce the same baseline
- this turns the cleaned warning baseline into a regression guard rather than a one-time cleanup

## Session update (2026-04-11 Clippy gate)
- `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings` is now clean
- `scripts/run_ci.sh` now runs Clippy before the Rust test suite, and GitHub Actions installs the `clippy` component before calling the shared script
- mechanical Clippy findings were fixed directly; intentional broad IR plumbing remains localized behind `#[expect(...)]` attributes with reasons rather than global allows

## Session update (2026-04-11 rustdoc gate)
- `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml --no-deps` is now clean
- fixed a broken intra-doc link interpretation in the `ActorSignalRelation` docs by changing bracket-shaped `output_of[...]` / `input_of[...]` wording into code-formatted prose
- `scripts/run_ci.sh` now runs rustdoc before the mdBook build and composes caller-provided `RUSTDOCFLAGS` with `-D warnings`

## Session update (2026-04-11 active-low VLM deassertion fixture)
- added `vlm_timing_active_low_deassertion_equivalence_gold` to lock the reset-release mirror of the existing active-low VLM timing polarity fixture
- the fixture proves active-low `ARESETN` observed as both `deasserted` and `HIGH` creates two typed temporal rules and zero temporal/polarity conflicts
- targeted `kg-bench` validation for the new fixture passed

## Session update (2026-04-11 graph-backed `.fsm` module directions)
- moved the first `.fsm` adapter consumer onto target-actor-relative graph evidence: explicit module candidates now overlay matching `IntentIR.actor_ports` before renderability analysis
- this lets an explicit top composition lower when a child module's flat module-local `direction_hint` values lag but the actor-relative graph already resolves the module actor's input/output port roles
- the slice stays conservative: `Input` / `Output` graph directions can fill `.fsm` module port roles, but `InOut` / `Unknown` do not become fake directions, and conflicts with flat hints still collapse to unresolved state
- added regressions that clear flat child-module directions to prove graph-backed `producer_core` / `consumer_core` actor ports recover the needed child port directions, and that inject a conflicting graph direction to prove lowering stays blocked instead of silently overriding local evidence

## Session update (2026-04-11 graph-backed `.fsm` direct-root directions)
- moved the next `.fsm` adapter consumer onto bounded actor-relative graph evidence: standalone direct roots can now recover missing local signal directions from `IntentIR.actor_ports` when the actor-port graph relevant to the direct local signal inventory has exactly one renderable actor context
- the direct-root path is intentionally stricter than explicit module lowering because no module name exists to identify the target actor; mixed producer/consumer graph contexts stay blocked rather than guessing which perspective should define direct `.fsm` input/output roles
- the overlay only strengthens signals already present in the direct signal inventory, unlike explicit-module lowering where graph-only module ports can be useful child-module interface evidence; unrelated graph-only actor ports are ignored by the direct-root actor-context gate
- actor-port provenance merging now reconciles direction once per actor-port record, preventing a conflicting graph direction with multiple supporting ids from accidentally reintroducing a resolved direction after conflict collapse
- focused tests now prove unambiguous standalone recovery, unrelated graph-only actor-port ignoring, ambiguous mixed-actor blocking, and duplicate-provenance conflict blocking for top-composition graph evidence
- the full local CI path passed with 287 Rust tests, warning-deny Clippy, warning-deny rustdoc, and mdBook build

## Session update (2026-04-11 top-link-backed `.fsm` boundary directions)
- explicit top ports now preserve width-only declarations by making `ExplicitTopPortRecord.direction_hint` optional
- `.fsm` top-composition renderability can recover a missing top boundary direction from explicit link topology: top link source means top input, and top link target means top output
- the recovery is not actor-graph inference and does not invent boundary roles; unresolved top directions and conflicts between explicit direction and link topology still block
- focused tests prove width-only `result_data` survives into canonical top composition and renders as `result_data>8` only through the explicit `consumer.result_data -> result_data` link
- the full local CI path passed with 288 Rust tests, warning-deny Clippy, warning-deny rustdoc, and mdBook build

## Session update (2026-04-04)
- `specforge converge` now drives the persisted `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters` path as a fixed-point loop and stops when the materialized knowledge snapshot is stable
- `EvidenceIr::build()` now preserves compatible alias-learning state, NLP-upgraded statement classes, and structured NLP records across rebuilds so pass `N+1` does not forget pass `N`
- `EvidenceIr::build()` now uses `converge_evidence_extractions()` instead of a one-shot extraction tail, allowing new enum facts and polarity facts to feed later passes in the same build
- signal-anchored encoding rescans recover weakly labeled encoding tables without introducing a new hardcoded APB/AHB/AXI value list
- `SemanticIR` now tolerates raw JSON, fenced JSON, and prose-wrapped JSON in VLM timing/state observations, restoring timing/state lift from real Ollama outputs
- `SemanticIR` / `IntentIR` now preserve the structural KG downstream via `actor_signal_relations`, `actor_ports`, and `signal_connectivity`, so actor-aware evidence is no longer trapped in `EvidenceIR`
- `specforge validate` now scores semantic and intent direction coverage from the actor-relative graph first, exposing flat `direction_hint` lag separately instead of treating compatibility-hint absence as semantic failure
- `SemanticIR` / `IntentIR` now also carry a first typed `temporal_rules` layer derived from constraints and timing observations, and validation now reports missing clock/edge grounding for that surface
- the temporal-rule layer now also recovers bounded `cycle_window` latency from structured prose/timing text, including idiomatic one-cycle phrases like `next cycle`, `next tick`, `next clock edge`, `next rising edge`, `next falling edge`, `next posedge`, and `next negedge`, plus quantified tick/edge forms like `within 2 ticks`, `within 2 clock edges`, `within 2 HCLK edges`, `within 2 edges of HCLK`, and `after 3 falling edges`, unit-first diagram labels like `tick T3` / `posedge T4`, named diagram-style generic-edge labels like `HCLK edge T3` / `edge T3 of HCLK`, unit-first local-clock forms like `tick T3 of HCLK` / `posedge T4 of HCLK`, trailing-`of <clock>` shorthand-edge forms like `the third posedge of HCLK` / `within 2 negedges of HCLK`, and explicit later/ordinal forms like `two cycles later`, `on the third rising edge`, and `on the third edge of HCLK`; explicit shorthand edge phrases now also preserve the named edge itself for signal/conditional/timing temporal rules instead of silently falling back to the default clock edge, explicit local timing text like `rising edge of HCLK`, `HCLK rising edge`, `HCLK posedge`, `next HCLK edge`, `the third edge of HCLK`, `within 2 edges of HCLK`, `edge T3 of HCLK`, `tick T3 of HCLK`, `posedge T4 of HCLK`, `third posedge of HCLK`, or `2 negedges of HCLK` can now override the document default `clock_signal`, named local cycle/tick phrasing like `same ACLK cycle` now inherits the same bounded rising-edge temporal anchor already used for default-clock timing, and known-signal-aware named edge phrasing like `next HCLK edge`, `within 2 HCLK edges`, `within 2 edges of HCLK`, `edge T3 of HCLK`, `third posedge of HCLK`, or `on the third edge of HCLK` now recovers the same bounded windows as the more explicit edge families instead of getting stranded behind the inserted clock token
- the temporal-rule layer now also compresses grounded ready/valid completion guards into typed `HandshakeComplete` predicates, so protocol-native transfer events are no longer represented only as separate scalar value clauses
- `EvidenceIR` now also persists typed `signal_semantic_hints` mined from signal-description text, direct prose descriptions, and alias-grounded prose descriptions, and `SemanticIR` / `IntentIR` now carry those roles as per-signal `semantic_tags`, so handshake-role inference can use meaning-grounded descriptions before falling back to literal `VALID` / `READY` spellings
- that semantic-hint synthesis is now stricter too: prose/visual hint extraction strips explicit signal identifiers before tag inference, so declarations and captions no longer create semantic role consensus from the signal name token alone
- semantic-hint synthesis is now also less lossy for multi-signal regions: prose statements and visual captions can be decomposed into clause-local per-signal context windows, so one region can ground valid-like meaning for one signal and ready-like meaning for another without forcing whole-text single-target resolution
- that same targeting path is now also stricter about aliases: if a sentence/caption already names the signal explicitly, alias-grounding is suppressed for that same signal so one source region cannot inflate semantic support just because both the alias and the canonical signal name appear
- the canonical role surface is now more inspectable too: semantic candidates and consensus summaries carry an explicit `alias_dependent` flag, and validation reports alias-dependent resolved roles instead of forcing readers to infer that dependency from raw source-kind lists
- that honesty surface now reaches the temporal layer too: validation reports when typed `HandshakeComplete` predicates depend on alias-dependent semantic consensus, so temporal progress semantics no longer look equally grounded when they still rely only on alias mapping
- that weaker temporal grounding is now canonical too: `SemanticIR` emits an explicit residual packet for alias-dependent handshake completion, and `IntentIR` mirrors it as an assumption so the warning survives even without running the validator
- contradictory polarity now survives downstream as well: `signal_polarity_conflicts` are no longer trapped in `EvidenceIR`, and validation now reports them for `SemanticIR` and `IntentIR` too
- `EvidenceIR` now also mines `signal_semantic_hints` from grounded visual captions and VLM timing-diagram annotations, using the same robust fenced/prose-wrapped JSON recovery path as the semantic VLM lift and preserving explicit visual-evidence provenance on those hints
- `SemanticIR` / `IntentIR` no longer collapse that richer role evidence entirely into `semantic_tags`; they now also carry per-signal `semantic_observations`, preserving source kind and provenance in the canonical layers
- `SemanticIR` / `IntentIR` now also carry per-signal `semantic_candidates`, keeping competing role hypotheses visible in the canonical layers instead of flattening them into only winner-or-none state
- `SemanticIR` / `IntentIR` now also carry per-signal `semantic_arbitration`, preserving the current lead role, runner-up, evidence margin, and decisive-vs-contested state so arbitration remains inspectable without forcing unsafe winner selection
- the temporal handshake layer now respects that arbitration state too: contested semantic-role evidence blocks literal `VALID` / `READY` name fallback instead of being silently overridden by it
- that blocked heuristic path is now surfaced explicitly as semantic residual state and validation metrics/findings, so withheld handshake promotion is inspectable rather than only implicit
- `SemanticIR` / `IntentIR` now also resolve canonical per-signal semantic roles plus modality-aware `semantic_grounding_strength` from those observations, so downstream consumers can prefer provenance-backed role consensus and validation can distinguish single-source grounding, same-modality repetition, and cross-modality reinforcement
- fallback-only resolved semantic roles are no longer only validator-visible drift: `SemanticIR` now emits a dedicated residual packet when a role still lacks observation-backed consensus, and `IntentIR` carries the same provisional state as an explicit assumption
- handshake-role consumers are now stricter as well: typed handshake recovery only trusts observation-backed semantic consensus, and handshake-shaped signals with fallback-only provisional roles now block raw name fallback instead of quietly regaining protocol meaning through spelling alone
- `SemanticIR` / `IntentIR` now also carry explicit `semantic_consensus` summaries for observation-backed role meaning, including supporting source kinds, supporting observation count, and strongest supporting automation confidence, and validation now flags any resolved role still lacking that richer profile
- `specforge nlp-enrich` now refreshes `signal_semantic_hints` before writing updated `EvidenceIR`, so alias learning can immediately feed downstream semantic-role inference instead of waiting for a later rebuild path
- `EvidenceIR` now also persists typed `signal_semantic_conflicts` when the same signal accumulates incompatible valid-like and ready-like role evidence, and `specforge validate` reports that disagreement explicitly instead of hiding it inside a dual-tag ambiguity
- `SemanticIR` / `IntentIR` now also carry those `signal_semantic_conflicts`, and `specforge validate` now reports them there too, so semantic-role disagreement does not disappear once the pipeline leaves the evidence stage
- the temporal-rule layer now reuses unique KG producers to emit actor-grounded drive predicates for value-oriented rules, reconnecting temporal semantics back to the structural graph
- the temporal-rule layer now also emits actor-grounded stability predicates for stable/hold rules when the KG resolves a unique producer, so producer obligations are no longer flattened into signal-only invariants
- the temporal-rule layer now preserves compound `and` guards as multiple typed antecedents when each clause anchors to a known signal, so conjunctive protocol preconditions survive into `IntentIR`
- the temporal layer now also materializes typed `temporal_conflicts` records when contradictory value obligations target the same signal/phase under the same grounded context, which keeps disagreement explicit instead of silently flattening it away
- the convergent evidence loop now also extracts active-high/active-low signal polarity from `SignalDescription` tables and merges that with prose polarity conservatively before refining asserted/deasserted constraints
- `EvidenceIR` now also persists `signal_polarity_conflicts` when prose/table polarity disagree, and `specforge validate` reports those conflicts explicitly instead of leaving the disagreement visible only as a neutralized constraint kind
- `SemanticIR` / `IntentIR` now also persist `interface_signal_conflicts` when conflicting declarations disagree on signal direction or width, and `specforge validate` reports that interface-shape disagreement explicitly instead of only degrading the canonical hint to `None`
- `SemanticIR` / `IntentIR` now also persist `signal_connectivity_conflicts` when the structural KG resolves multiple producers for the same signal, and `specforge validate` reports that producer ambiguity explicitly instead of leaving it implicit in connectivity vectors
- `specforge validate` now writes deterministic stage-local `validation_report.json` sidecars and backannotates the current report into `validation_reports` on `SourceIR`, `EvidenceIR`, `SemanticIR`, and `IntentIR`
- `specforge project-validation <artifact>...` now refreshes `VALIDATION_SNAPSHOT.md` and the managed validation block in `LIVE_ACHIEVEMENT_STATUS.md` from persisted IR validation reports
- `specforge learn-priors <intent_ir>...` now builds the first local typed `CorpusMemory` prior store under `generated/prior_memory/corpus_memory.json`, harvesting only from validated `IntentIR` artifacts and keeping the learning plane advisory-only
- the first `R15f` slice currently learns:
  - actor-taxonomy priors from decisive actor-grounded handshake-role evidence plus conservative self-identifying actor vocabulary
  - semantic-role phrase priors from decisive, non-alias-dependent canonical semantic consensus plus preserved observation text
  - semantic modality-reliability priors from decisive, non-alias-dependent canonical semantic consensus plus supporting source kinds
  - temporal-language phrase priors from canonical `temporal_rules` plus validated canonical `signal_constraints` / `conditional_rules`
  - table-shape priors from validated structured-table header signatures chained through validated artifacts
- the latest live four-document AMBA prior-memory run across AXI/APB/AHB/AXI-Stream `IntentIR` artifacts currently yields `16` actor-taxonomy priors, `5` semantic phrase priors, `4` semantic modality-reliability priors, `266` temporal phrase priors, and `99` table-shape priors
- the learning plane now also rejects bogus payload/event actor terms like `control information` during both prior harvesting and prior lookup, so stale non-actor vocabulary cannot keep biasing future extraction after the underlying document-local bug is repaired
- that matters because the first unseen protocol run has now moved the semantic side of the learning plane from “architecturally landed but empty on real artifacts” to “materially populated by validated real-document consensus”
- the first bounded prior-consumption path is now landed too: `EvidenceIR` can consult the local `CorpusMemory` during `evidence` / `converge` and use actor-taxonomy priors to interpret explicit local actor labels in section headings and `Source` / `Destination` columns, and width-only section-guided signal tables can now recover structural `ActorSignalRelation::Drives` edges instead of only flat compatibility directions
- that matters because it replaces another brittle hardcoded-vocabulary heuristic with a typed reusable prior while still keeping canonical truth local; the prior can widen the meaning of a seen actor term like `Producer`, but it cannot invent a new actor, signal, or relation
- the second bounded prior-consumption path is now landed too: `EvidenceIR` can use semantic phrase priors to recover local signal-role hints from non-hardcoded grounded phrases, and that guidance now survives later `refresh_signal_semantic_hints()` calls because the consulted `prior_memory_path` is persisted into `EvidenceIR`
- the third bounded prior-consumption path is now landed too: `SemanticIR` can use temporal phrase priors to advisory-recover cycle windows from local timing text when the built-in parser cannot recover that timing window directly
- the fourth bounded prior-consumption path is now landed too: `SemanticIR` can use semantic modality-reliability priors to advisory-adjust local semantic arbitration when the current PDF already contains multiple competing locally grounded role candidates, while keeping the underlying semantic conflict explicit
- the benchmark surface now locks all four first bounded prior families:
  - actor-taxonomy priors: width-only `Issuer signals` / `Acceptor signals` sections stay directionless and graph-empty without prior memory and gain structural KG edges, actor ports, and canonical directions only when the matching actor-taxonomy priors are staged into the fixture
  - semantic phrase priors: `XACK can receive the transfer` stays semantically unresolved without prior memory and gains ready-like recovery only with a matching semantic prior
  - temporal phrase priors: `PREADY must be asserted one beat later` stays temporally unbounded without prior memory and gains a one-cycle `cycle_window` only with a matching temporal prior
  - semantic modality-reliability priors: locally conflicted role evidence like `XCTRL` stays contested without prior memory and becomes decisively resolved only when the matching modality-reliability prior is staged into the fixture
- the same semantic prior family is now benchmarked across another modality too: the unseen local visual-caption phrase `XACK can sink the transfer` stays unresolved without prior memory and gains ready-like recovery only when a matching `visual_caption` semantic prior is staged into the fixture
- the fourth bounded prior-consumption path is now landed too: `EvidenceIR` can advisory-recover a local table kind from a learned table-shape prior, but only when the current table is still `unknown`; explicit local `SourceIR` table kinds still win outright
- the benchmark surface now locks the first table-shape before/after truthfulness pairs too:
  - a locally `unknown` `Name | Direction | Width` table stays inert without prior memory and gains signal-description recovery only when a matching learned table-shape prior is staged into the fixture
  - a locally `unknown` `Parameter | Min | Max | Unit` table stays inert without prior memory and gains timing-parameter recovery only when a matching learned table-shape prior is staged into the fixture
- that matters because the learning plane can now strengthen local temporal understanding on unseen phrase shapes without letting prior memory create timing rules that are not already grounded in the current PDF
- the KG benchmark harness can now also stage a fixture-owned `CorpusMemory`, and the first tracked gold/negative pair proves that unseen local timing language only gains a `cycle_window` when a matching learned temporal prior is present
- the KG benchmark harness now also locks the same before/after truthfulness pattern for semantic priors: unseen local role language only gains canonical semantic recovery when a matching learned semantic prior is present
- the latest unseen protocol stress run is now AXI-Stream: it converges in `2` full pipeline iterations, validates at `90/100 EXCELLENT`, and the latest truthfulness fixes now recover both parity-check ownership and parity-check widths from the local `Check Signal / Signals Covered` semantics, bringing declared graph-direction and width coverage to `22/22`; `ACLK` / `ARESETN` now classify as infrastructure connectivity with canonical sourcing left in the system-contract surface, same-cycle timing language now lands as six explicit `0`-cycle temporal windows, assertion-vs-level temporal comparison is now polarity-aware, and the last carried interface-grouping residual is gone after heuristic grouping stopped treating width/table metadata as interface signals and explicit interfaces began subsuming smaller statement fragments
- resolved signal polarity now also survives directly on canonical `InterfaceSignalRecord`s and is visible in validator metrics as `with_resolved_polarity`; after the latest AHB infrastructure-interface fix, AXI/APB/AHB/AXI-Stream now all report `1`, and non-reset control coverage now includes asserted-when-level prose, unambiguous collective active-level prose, and safe mixed clause-local prose rather than only reset carry-through plumbing
- `specforge converge` now excludes downstream adapter residual work from `knowledge_fact_count`, so fewer adapter residual decisions do not falsely trip the monotone-knowledge guard
- `generated/` is now intentionally git-ignored and untracked, so local validation snapshots must be recorded in the live docs instead of relying on versioned artifacts
- latest local validation snapshot is now AXI `85/100 GOOD`, APB `90/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`; this refresh replaced a stale optimistic snapshot, the current tracked four-artifact projection now uses APB `IHI0024_E`, AHB remains in the excellent lane, and the latest AXI truthfulness fixes removed the last blocked-handshake residual, the false `AWAKEUP` / `CRVALID` semantic-role conflicts, the false `ACLK` / `ARESETN` interface-direction conflicts, and the bogus `Tie-off`-driven `BROADCAST*` missing-consumer warning while leaving AXI as the main remaining live quality outlier
- APB `IHI0024_D` was re-run from the original PDF through full `specforge converge` with Ollama VLM + NLP Level 3 and converged in 2 passes
- AXI `IHI0022_L` was re-run from the original PDF through full `specforge converge` with Ollama VLM + NLP Level 3, converged in 2 passes, and recovered timing to reach 94/100 EXCELLENT
- `extract_alias_phrase()` now rejects markdown/table/list marker prefixes such as `-`, `|`, `#`, `*`, `+`, `>`, `1.`, and `2)`, closing the last small R12 cleanup in the NLP alias-learning loop
- the documented README staged flow was re-executed on `README.md` through `inspect -> ingest -> evidence -> semantic -> intent -> adapt --dry-run`, confirming the current entry path still runs end-to-end
- the roadmap now explicitly treats adapter expansion as horizon work; the next structural gaps are making the actor-relative graph primary, broadening the new table/prose/alias-grounded role inference into richer multimodal grounding, adding deeper explicit temporal semantics, and hardening KG quality/evaluation
- the current architecture is still intentionally document-local, which is correct for truthfulness, but the next strategic expansion after the current semantic-truthfulness work should be a separate cross-document learning plane that stores reusable extraction priors rather than cross-document facts
- the local Rust test suite is now at `288/288` passing in the latest full CI run after the KG-bench graph-direction expectation surface landed
- `specforge kg-bench` now provides the first tracked KG-quality fixture harness under `crates/specforge/test_data/kg_quality`, including:
  - a gold actor-port recovery fixture
  - a negative name-only semantic noise fixture
  - a negative multi-producer conflict fixture that exercises validation findings
  - an actor-boundary residual-quality fixture
  - a stage-patched contested-handshake fixture that proves contested `XVALID`/`XACK` meaning blocks typed `HandshakeComplete` recovery
  - a stage-patched alias-dependent caveat fixture that proves typed handshake recovery can remain canonical while keeping alias-dependent residual and assumption state explicit
  - a stage-patched cross-modality grounding gold fixture that proves one role can be reinforced jointly by table evidence and visual-caption evidence
  - a stage-patched cross-modality conflict fixture that proves table-versus-visual disagreement stays canonically contested instead of collapsing into false cross-modality consensus
  - a stage-patched VLM timing-note gold fixture that proves a semantic hint can be grounded directly by `vlm_timing_diagram_extraction` and verified at the evidence stage instead of being inferred only from downstream visual-grounding side effects
  - a stage-patched VLM timing-note negative fixture that proves waveform/tick descriptions around `XVALID`-style names still produce timing extraction while semantic-role inference stays at zero
  - a stage-patched visual-source conflict fixture that proves caption semantics and VLM timing-note semantics can disagree while remaining visibly grounded and canonically contested
  - a first representative AMBA-style gold fixture that proves `Source`-column signal-description tables can recover driver-side actor ports plus semantic handshake meaning strongly enough to derive a typed `HandshakeComplete` guard from one constraint
  - a representative AMBA-style `Destination`-column gold fixture that proves receiver-side table rows survive canonically as `Reads` relations and actor-relative input ports
  - a representative APB-style gold fixture that proves `Requester` / `Completer` source roles recover driver-side actor relations, actor-relative ports, request/accept semantics, and typed handshake completion end-to-end
  - a representative APB-style timing gold fixture that proves `Requester` / `Completer` source roles can also recover setup/access timing, actor-grounded temporal predicates, multi-predicate guards, bounded next-cycle latency, and handshake completion together
  - a representative APB address/protection stability fixture that proves `PADDR` and `PPROT` stay actor-grounded stable Requester outputs through wait-state constraints without false handshake completion
  - a representative APB write-control stability fixture that proves `PWRITE`, `PWDATA`, and `PSTRB` stay actor-grounded stable requester outputs through wait-state constraints without false handshake completion
  - a representative APB response stability fixture that proves `PRDATA` and `PSLVERR` stay actor-grounded stable completer outputs through completed-access constraints with real handshake completion
  - a representative AHB control stability fixture that proves `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, and `HPROT` stay actor-grounded stable manager outputs through wait-state constraints without false handshake completion
  - a representative AHB transfer/lock stability fixture that proves `HTRANS` and `HMASTLOCK` stay actor-grounded stable manager outputs through wait-state constraints without false handshake completion
  - a representative AHB exclusive/security stability fixture that proves `HEXCL` and `HNONSEC` stay actor-grounded stable manager outputs while `HEXOKAY` stays an actor-grounded stable subordinate output through wait-state constraints without false handshake completion
  - a representative AHB response stability fixture that proves `HRDATA` and `HRESP` stay actor-grounded stable subordinate outputs through wait-state constraints without false handshake completion
  - a representative AHB write-data stability fixture that proves `HWDATA` stays actor-grounded stable as a manager output through three-predicate write wait-state constraints without false handshake completion
  - a representative AXI-style gold fixture that proves width-only channel tables plus prose drive/sample relations recover actor-relative direction, signal inventory, request/accept semantics, and typed handshake completion without any table direction column
  - a representative AXI-style timing gold fixture that proves width-only channel tables plus prose actor relations can also recover next-cycle timing, actor-grounded temporal predicates, and handshake completion together
  - a representative AXI write-response timing fixture that proves the same path on `BVALID` / `BREADY` / `BRESP`, including response-valid timing and response-payload stability
  - a representative AXI write-response ID stability fixture that proves `BID` remains an actor-grounded subordinate transaction-ID obligation under the controlling `BVALID` / `BREADY` handshake
  - a representative AXI read-address timing fixture that proves the same path on `ARVALID` / `ARREADY` / `ARADDR` / `ARLEN`, including read-address-ready timing and read-address payload stability
  - a representative AXI read-address ID stability fixture that proves `ARID` remains an actor-grounded manager transaction-ID obligation under the controlling `ARVALID` / `ARREADY` handshake
  - a representative AXI write-address ID stability fixture that proves `AWID` remains an actor-grounded manager transaction-ID obligation under the controlling `AWVALID` / `AWREADY` handshake
  - a representative AXI write-address control sideband stability fixture that proves `AWPROT`, `AWCACHE`, and `AWLOCK` remain actor-grounded manager control-sideband obligations under the controlling `AWVALID` / `AWREADY` handshake
  - a representative AXI read-address control sideband stability fixture that proves `ARPROT`, `ARCACHE`, and `ARLOCK` remain actor-grounded manager control-sideband obligations under the controlling `ARVALID` / `ARREADY` handshake
  - a representative AXI address QoS/region sideband stability fixture that proves `AWQOS`, `AWREGION`, `ARQOS`, and `ARREGION` remain actor-grounded manager sideband obligations under their matching address-channel handshakes
  - a representative AXI read-address sideband stability fixture that proves `ARSIZE` and `ARBURST` remain actor-grounded manager sideband obligations under the controlling `ARVALID` / `ARREADY` handshake
  - a representative AXI read-data timing fixture that proves the same path on `RVALID` / `RREADY` / `RDATA` / `RRESP`, including read-data-valid timing and read-data payload stability
  - a representative AXI read-data ID stability fixture that proves `RID` remains an actor-grounded subordinate transaction-ID obligation under the controlling `RVALID` / `RREADY` handshake
  - a representative AXI read-data response stability fixture that proves `RRESP` remains an actor-grounded subordinate response obligation under the controlling `RVALID` / `RREADY` handshake
  - a representative AXI read-data last stability fixture that proves `RLAST` remains an actor-grounded subordinate sideband obligation under the controlling `RVALID` / `RREADY` handshake
  - a representative AXI address/response USER sideband stability fixture that proves `AWUSER` and `ARUSER` remain actor-grounded manager sideband obligations while `BUSER` remains an actor-grounded subordinate sideband obligation under their matching channel handshakes
  - a representative AXI data USER sideband stability fixture that proves `WUSER` remains an actor-grounded manager sideband obligation and `RUSER` remains an actor-grounded subordinate sideband obligation under their matching data-channel handshakes
  - a representative AXI write-data timing fixture that proves the same path on `WVALID` / `WREADY` / `WDATA` / `WSTRB`, including write-data-ready timing and write-data payload stability
  - a representative AXI write-data last stability fixture that proves `WLAST` remains an actor-grounded manager sideband obligation under the controlling `WVALID` / `WREADY` handshake
  - a representative AXI sideband stability fixture that proves `ARLEN` and `WSTRB` remain actor-grounded sideband obligations under their controlling ready/valid handshakes
  - a representative AXI write-address sideband stability fixture that proves `AWLEN`, `AWSIZE`, and `AWBURST` remain actor-grounded sideband obligations under the controlling `AWVALID` / `AWREADY` handshake
  - a representative AHB-style gold fixture that proves `Manager signals` / `Subordinate signals` section context recovers per-signal direction and width correctly from staged `SourceIR`
  - a representative AHB-style timing gold fixture that proves the same section-heading and destination-column context can also recover wait-state timing, actor-grounded temporal predicates, bounded next-cycle latency, and multi-predicate guards together
  - the canonical parser now also accepts width-only synthesized declarations like `Signal AWVALID is width 1.`, which closes the specific AXI gap where table-grounded widths previously stopped at actor relations and never became real interface signal records
  - a bogus-actor-attribution negative fixture that proves `Clock` / `Reset` infrastructure rows in AMBA-style `Source` columns do not become protocol actors while the true requester/subordinate rows still survive canonically
  - a field-table misclassification negative fixture that proves a misclassified `Bits | Name | Description` table does not synthesize fake top-level signals or semantic roles from field names that merely look signal-like
  - a spurious-timing negative fixture that proves low-value VLM timing-diagram labels like `T0`, `Addr 1`, `Cycle 2`, `D0`, `A1`, `DATA0`, `0xAA`, `D[0]`, `A[1]`, `DATA[3]`, and `ADDR[7]` still count as timing-diagram extraction at the evidence stage but do not survive into `TimingConstraintRecord` or `TemporalRuleRecord`
  - a VLM waveform-motion negative fixture that proves `rising`, `stable`, `falling`, `UNCHANGED`, `RISING_EDGE`, `LOW_TO_HIGH`, `HIGH_TO_LOW`, `POS_EDGE`, `NEG_EDGE`, `risingedge`, `LOW2HIGH`, and `HIGH2LOW` timing states do not become symbolic signal values while a concrete `HIGH` sample still survives
  - a VLM state-machine label-noise negative fixture that proves prose/OCR labels like `IDLE state` and `ACCESS phase` do not become canonical FSM state names or transition endpoints while clean identifier evidence still survives
  - a VLM state-machine undeclared-transition negative fixture that proves identifier-shaped but undeclared endpoints such as `DONE` and `RESET` do not become canonical transition graph facts
  - a VLM state-machine duplicate-initial gold fixture that proves duplicate state labels merge and preserve an initial marker if any duplicate carries it
- the harness can now also assert canonical actor-signal relations directly, so tracked gold fixtures can lock `Drives` versus `Reads` truth instead of checking only actor-port projections or relation counts
- the harness can now also patch `SourceIR.document_sections` and assert per-signal canonical direction directly, which is important for protocol families like AHB where section-heading context still carries real directionality
- the harness now also asserts canonical semantic candidates and decisive-vs-contested semantic arbitration directly, which is a better `R15e` truthfulness check than inferring arbitration quality only from blocked fallback or validation side effects
- the harness now also asserts canonical state names, initial-state names, and transition endpoints directly, so VLM state-machine truthfulness can be locked in tracked fixtures instead of only in unit tests
- the harness now also asserts persisted validation metric values directly at the evidence, semantic, and intent stages and can patch `SourceIR` visual assets, which makes tracked cross-modality grounding and VLM-note provenance checks practical instead of leaving them to ad hoc unit tests
- table-based relation extraction itself is also less lossy now:
  - `Source` / `Driver` columns become `Drives`
  - `Destination` columns become `Reads`
  - direction/infrastructure placeholders like `input`, `Clock`, and `Reset` are filtered instead of being promoted into bogus actor names
- table-driven top-level signal synthesis is also less lossy now:
  - field-like `Bits | Name | Description` layouts are rejected even if they were misclassified upstream as `signal_description`
  - that guard now protects fake-signal leakage across declarations, semantic hints, and related table-driven inference paths
- timing-diagram lifting is also less noisy now:
  - label-only VLM annotations such as `T0`, `Addr 1`, and `Cycle 2` are treated as waveform labels, not as timing semantics
  - VLM timing signal-value states such as `rising`, `stable`, `falling`, `UNCHANGED`, `RISING_EDGE`, `LOW_TO_HIGH`, `HIGH_TO_LOW`, `POS_EDGE`, `NEG_EDGE`, `risingedge`, `LOW2HIGH`, and `HIGH2LOW` are treated as waveform motion descriptors, not as concrete symbolic signal values
  - the semantic lift now keeps the timing extraction visible upstream while refusing to fabricate canonical timing constraints from those low-value labels alone
- that negative multimodal fixture also locks an important nuance in the validation surface:
  - conflicting multimodal evidence should still count as visual grounding
  - but it must not count as resolved cross-modality grounding
- that means benchmark hardening is no longer purely roadmap text; the repo now has a real seed harness for false-positive control and residual-quality regression across AMBA, APB, AHB, and expanding APB/AHB/AXI protocol-grade gold suites, even though broader AXI and deeper structured-constraint suites are still ahead
- the harness is also more realistic now than a pure markdown fixture runner because tracked fixtures can patch `SourceIR` and `EvidenceIR` surfaces directly, which is a practical way to benchmark richer multimodal/structured semantics without needing a heavyweight source document for every regression

## Observed current state
### Repository contents directly observed
- `.git/`
- `.gitmodules`
- live documentation surface
- `INTENTIR_SPEC.md`
- `Cargo.toml`
- `Cargo.lock`
- `crates/specforge/Cargo.toml`
- `crates/specforge/src/main.rs`
- `crates/specforge/src/lib.rs`
- `crates/specforge/src/cli.rs`
- `crates/specforge/src/error.rs`
- `crates/specforge/src/commands/inspect.rs`
- `crates/specforge/src/commands/converge.rs`
- `crates/specforge/src/commands/ingest.rs`
- `crates/specforge/src/commands/evidence.rs`
- `crates/specforge/src/commands/semantic.rs`
- `crates/specforge/src/commands/intent.rs`
- `crates/specforge/src/commands/adapt.rs`
- `crates/specforge/src/commands/enrich.rs`
- `crates/specforge/src/commands/validate.rs`
- `crates/specforge/src/commands/project_validation.rs`
- `crates/specforge/src/commands/rescan_plan.rs`
- `crates/specforge/src/commands/kg_bench.rs`
- `crates/specforge/src/commands/learn_priors.rs`
- `crates/specforge/src/commands/nlp_enrich.rs`
- `crates/specforge/src/test_support.rs`
- `crates/specforge/src/ir/mod.rs`
- `crates/specforge/src/ir/source.rs`
- `crates/specforge/src/ir/source/docling_backend.rs`
- `crates/specforge/src/ir/evidence.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/src/ir/intent.rs`
- `crates/specforge/src/ir/adapters.rs`
- `crates/specforge/src/ir/prior_memory.rs`
- `subs/fsmgen/`

### Rust-specific contents still absent
- no dedicated `specforge-source` crate
- no dedicated `specforge-evidence` crate
- no dedicated `specforge-semantic` crate
- no dedicated `specforge-intent` crate
- no dedicated `specforge-adapters` crate
- no dedicated validation crate
- no separate Rust integration-test crate; the tracked KG-quality fixture harness lives inside the active crate under `crates/specforge/test_data/kg_quality`
- no additional real builders beyond the current `SourceIR`/`EvidenceIR`/`SemanticIR`/`IntentIR` slices and the first `.fsm` adapter slice

### Immediate implication
- the codebase is no longer mostly scaffolding; the main open problem is semantic truthfulness across the four IR layers, especially graph primacy, temporal semantics, multimodal rescans, and evidence arbitration
- the practical risk is now split across two partial migrations:
  - `SemanticIR` and `IntentIR` still expose both graph-native actor-relative records and legacy flat `direction_hint` fields; validation/scoring is now graph-first and explicit-module plus unambiguous standalone direct `.fsm` paths have graph-backed direction overlays, but some downstream compatibility and consumer paths still consult the flat hints directly
  - the new temporal-rule layer is real and now includes bounded cycle windows, compound antecedents, typed temporal conflicts, and first actor-grounded drive/stability predicates, but it still covers only a narrow slice of possible temporal/actor semantics and does not yet arbitrate broader cross-rule or cross-modality contradictions
- the continuity risk around untracked generated artifacts is lower now that validation snapshots can be re-projected into tracked docs deterministically, but the docs still depend on someone running the projection flow after meaningful validation runs

## What the tool needs to do
- build `SourceIR` from raw specifications and normalized artifacts
- build `EvidenceIR` from normalized markdown, page assets, figures, captions, and evidence extraction
- build `SemanticIR` from actors, interfaces, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
- build canonical `IntentIR` as a backend-independent intent model
- lower `IntentIR` through adapters such as `.fsm`, SystemVerilog, Verilog, and VHDL
- validate stage outputs and adapter outputs and back-annotate findings

## Current implemented architecture
### Root workspace
- `Cargo.toml`
  - workspace root
- `Cargo.lock`
  - dependency lockfile

### Active crate
- `crates/specforge`
  - single user-facing CLI crate and binary for the current slice

### Implemented module boundaries
- `src/main.rs`
  - binary entrypoint
- `src/lib.rs`
  - command dispatch and public module exports
- `src/cli.rs`
  - clap-based command model using the `specforge` binary name
- `src/error.rs`
  - typed error/result boundary
- `src/commands/inspect.rs`
  - deterministic source/path inspection command
- `src/commands/converge.rs`
  - fixed-point orchestration command over persisted `SourceIR`/`EvidenceIR`/`SemanticIR`/`IntentIR`/adapter artifacts
- `src/commands/ingest.rs`
  - `SourceIR` preview/materialization command
- `src/commands/evidence.rs`
  - `EvidenceIR` preview/materialization command
- `src/commands/semantic.rs`
  - `SemanticIR` preview/materialization command
- `src/commands/intent.rs`
  - `IntentIR` preview/materialization command
- `src/commands/adapt.rs`
  - `.fsm` adapter preview/materialization command
- `src/commands/enrich.rs`
  - VLM-backed visual enrichment command for `SourceIR`
- `src/commands/validate.rs`
  - stage-aware artifact validation and quality-scoring command
- `src/commands/project_validation.rs`
  - project-level validation snapshot projection and schema-v2 targeted rescan recommendation generation
- `src/commands/rescan_plan.rs`
  - dry-run-first consumer and whitelisted local executor for schema-v2 targeted rescan plans
- `src/commands/kg_bench.rs`
  - tracked KG-quality benchmark harness over staged IR artifacts and persisted validation findings
- `src/commands/learn_priors.rs`
  - local typed `CorpusMemory` harvesting command for advisory cross-document extraction priors
- `src/commands/nlp_enrich.rs`
  - LLM-backed NLP Level 3 enrichment command for `EvidenceIR`
- `src/test_support.rs`
  - shared process-global test synchronization utilities for env-var-mutating CLI tests
- `src/ir/mod.rs`
  - stage identifiers for `source_ir`, `evidence_ir`, `semantic_ir`, and `intent_ir`
- `src/ir/source.rs`
  - concrete `SourceIR` implementation, structured source types, `WidthHint`, and actor-signal relation type definitions
- `src/ir/source/docling_backend.rs`
  - external backend discovery, Docling command orchestration, and the embedded Python helper for structured PDF normalization
- `src/ir/evidence.rs`
  - concrete `EvidenceIR` builder, markdown parsing, caption/reference linking, table synthesis, typed NLP extraction, alias persistence, and actor-signal relation extraction
- `src/ir/semantic.rs`
  - concrete `SemanticIR` builder, semantic lifting heuristics, VLM merge logic, and residual-decision generation
- `src/ir/intent.rs`
  - concrete `IntentIR` builder, canonicalization heuristics, and residual-decision preservation
- `src/ir/adapters.rs`
  - typed adapter artifacts, `.fsm` lowering logic, renderability gating, and adapter-side residual-decision generation
- `src/ir/prior_memory.rs`
  - typed local prior-memory schema and helper logic for advisory cross-document extraction priors

## Assessment of current structure
### What is good
- the crate/binary identity now matches the repo direction
- the code no longer hardcodes `.fsm` as the conceptual endpoint
- a real typed `SourceIR` artifact exists instead of a handwritten ingest plan
- a real structured PDF normalization path now exists inside `SourceIR`, so the first stage is operational for both Markdown and PDF inputs
- a real typed `EvidenceIR` artifact now exists, so the staged pipeline is operational beyond raw source normalization
- a real typed `SemanticIR` artifact now exists, so the staged pipeline now reaches a backend-neutral semantic layer before the final canonicalization stage
- a real typed `IntentIR` artifact now exists, so the end-to-end source-to-intent pipeline is operational before adapter lowering
- the later stages have typed names and module homes, which reduces the risk of accidental backend-first growth
- adapter lowering is separated from the canonical IR stages
- the IR surface now carries page and visual manifests plus backend source references that later stages can ground against
- the repository now also contains a pinned local `fsmgen` checkout, which gives the next `.fsm` adapter slice a nearby reference implementation without changing the canonical `IntentIR` boundary
- that `fsmgen` checkout is now explicitly contextual and read-only from the `specforge` side; any observed upstream misbehavior should be captured as a local `FSMGEN-BUG-####` report instead of a submodule edit
- the first `.fsm` adapter slice already enforces honest renderability boundaries instead of fabricating target text from under-specified intent
- the canonical model now preserves typed signal inventory and backend-neutral guarded/action control fragments before the adapter boundary
- the canonical model now also preserves backend-neutral system contract and init-assignment records for explicit standalone sequential control, including first-class reset polarity/assertion/release/target semantics
- the canonical model now also preserves explicit regular-state and transition records for stateful lowering
- the canonical model now also preserves canonical symbol-definition sections and structured control blocks, including dedicated synchronous-reset and asynchronous-reset control roles
- the `.fsm` adapter can now emit a real standalone `?dt:name` file for explicit combinational and sequential DT cases, canonical symbol-definition sections, structured reset-role blocks, selector/test-node branches, compound-update shorthand, and a real structured `?fsm:name` file for explicit state-graph cases when those canonical facts are explicit enough

### What is still insufficient
- only the `.fsm` adapter is implemented today; SystemVerilog, Verilog, and VHDL adapters are still absent
- validation now backannotates persisted IR artifacts, writes stage-local sidecars, and can project the latest staged snapshot back into tracked docs
- the actor-signal relation graph now survives into `SemanticIR` / `IntentIR`, and the first explicit-module `.fsm` composition consumer can use matching `actor_ports` to recover child-module port directions, but legacy interface records still flatten some other downstream consumers onto actor-agnostic `direction_hint` values
- the previous dead-code warning baseline has been cleaned by deleting stale helpers rather than suppressing them
- the current renderable `.fsm` slices are intentionally narrow: they handle explicit standalone combinational/sequential DT control, canonical symbol-definition sections, structured reset-role blocks, selector/test-node branches, compound-update shorthand, explicit structured FSM-root cases, and explicit top-root composition, while broader unsupported selector/predicate shapes and non-FSM backends stay deferred

## Architectural recommendation
### Core architectural stance
- keep `IntentIR` as the canonical endpoint
- keep adapters downstream of `IntentIR`
- keep the internal system of record typed and stage-specific
- keep residual decisions explicit at every stage
- do not let convenience around one backend contaminate the stage-neutral model
- use structured parsing first and selective multimodal enrichment second, rather than collapsing the problem into markdown-only OCR or ungrounded VLM generation
- keep actor/signal relations first-class long enough that downstream adapter work does not have to rediscover them from flattened `input` / `output` hints

### Recommended growth path from the current codebase
#### Keep in the current crate for the next slices
- extend the current validation projection flow beyond staged IR artifacts into downstream adapter artifacts
- finish moving the downstream signal-direction model from compatibility flat hints to actor-relative semantics before serious SystemVerilog adapter work
- keep compatibility-level `?mod:name` / `?module:name` spellings outside the adapter root-kind model until a real backend-neutral direct-module distinction exists
- keep any new composition/control enrichment backend-neutral so the canonical model boundary stays intact
- keep the latest projected four-artifact snapshot visible as follow-on work lands: AXI `85/100 GOOD`, APB `90/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`, with AXI still the main quality outlier

#### Split into dedicated crates when pressure becomes real
- `specforge-source`
  - source registration, normalization, converter orchestration
- `specforge-evidence`
  - section anchors, evidence spans, statement extraction, relation extraction, and provenance
- `specforge-semantic`
  - actor and semantic lifting
- `specforge-intent`
  - canonical intent model and versioned serialization
- `specforge-adapters`
  - target-specific lowerings
- `specforge-validate`
  - validation, diagnostics, and back-annotation

## Mapping from staged architecture to the current modules
### SourceIR
- current primary ownership:
  - `src/commands/ingest.rs`
  - `src/ir/source.rs`

### EvidenceIR
- current declared ownership:
  - `src/commands/evidence.rs`
  - `src/ir/evidence.rs`
- current executable behavior:
  - builds `EvidenceIR` from ready `SourceIR`, promoted markdown, visual-asset manifests, and structured tables
  - synthesizes signal, enum, register, and timing evidence from structured tables
  - extracts structured signal constraints and conditional rules from classified sentences
  - extracts actor-signal relation triples from prose verb patterns and signal-description table role columns
  - re-enters a monotone convergence loop so signal anchors, discovered enum members, dynamic prose constraints, polarity refinement, and KG-derived direction synthesis can reinforce one another before hand-off to `SemanticIR`
  - persists signal-alias state so `specforge nlp-enrich` can tighten the evidence iteratively across passes

### SemanticIR
- current declared ownership:
  - `src/commands/semantic.rs`
  - `src/ir/semantic.rs`
- dependency:
  - requires real `EvidenceIR`
- current executable behavior:
  - builds `SemanticIR` from persisted `EvidenceIR`, deriving actors, actor-relative port/connectivity records, interfaces, typed signal records, backend-neutral control fragments, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions
  - merges VLM timing/state observations and filters NLP outputs through the declared-signal gate
  - now preserves `actor_signal_relations`, `actor_ports`, and `signal_connectivity`, but still keeps compatibility-level actor-agnostic `direction_hint` values on interface records

### IntentIR
- current declared ownership:
  - `src/commands/intent.rs`
  - `src/ir/intent.rs`
- dependency:
  - requires real `SemanticIR`
- current executable behavior:
  - builds `IntentIR` from persisted `SemanticIR`, deriving intent identity, actor responsibilities, interface inventory, backend-neutral control fragments, behaviors, constraints, assumptions, and residual decisions

### Adapters
- current declared ownership:
  - `src/commands/adapt.rs` — `.fsm` adapter preview/materialization command
  - `src/commands/enrich.rs` — VLM diagram enrichment command (Ollama/OpenAI/LM Studio)
  - `src/commands/validate.rs` — artifact health validation command with quality score
  - `src/commands/nlp_enrich.rs` — NLP Level 3 evidence-enrichment command
  - `src/ir/mod.rs`
- dependency:
  - requires stable `IntentIR`
- nearby reference implementation:
  - `subs/fsmgen/`
- local workflow rule:
  - treat `subs/fsmgen` as read-only contextual input
  - if upstream behavior looks wrong, file a local tracked bug report under `FSMGEN-BUG-####` rather than patching the submodule here
- current executable behavior:
  - builds a typed `.fsm` adapter artifact from persisted `IntentIR`
  - chooses `?dt:name` for explicit standalone DT cases, `?fsm:name` when explicit regular-state and transition records are present, and `?top:name` when explicit module/top composition facts are present
  - consumes canonical signal inventory, backend-neutral system/init records, backend-neutral control fragments, explicit regular-state/transition records, and explicit module/top composition facts when present
  - overlays explicit module signal inventory with matching `IntentIR.actor_ports` so graph-backed module actor directions can fill missing child-module port hints during `?top:name` renderability analysis
  - overlays standalone direct signal inventory with graph-backed actor-port directions only when all renderable actor-port evidence for signals already present in the local direct inventory points at one unambiguous actor
  - recovers missing top boundary port directions from explicit top-link source/target topology before rendering `?top:name`
  - emits a real standalone `?dt:name` file only when widths, directions, guarded/action blocks, and any required standalone sequential system/init facts are explicit enough to avoid semantic invention
  - emits a real structured `?fsm:name` file only when the state graph, transition targets, and state-body control are explicit enough to avoid semantic invention
  - emits a real explicit `?top:name` source document only when the top ports, child modules, and links are explicit enough to avoid semantic invention
- next real implementation target:
  - complete the transition to graph-first actor-relative direction semantics, then extend the same validation/reporting discipline into downstream non-FSM adapter work

## Major risks
### Risk: backend leakage into IntentIR
- if `.fsm` or RTL-specific assumptions creep back into the canonical model, the pivot fails even if the names remain correct
### Risk: actor-agnostic direction collapse
- if the current actor-signal relation graph is flattened too early into one-size-fits-all `input` / `output` hints, downstream adapters will encode the wrong actor perspective and hide the real structural knowledge the pipeline already extracted

### Risk: incomplete validation/back-annotation
- if validation findings never flow back into persisted artifacts and live docs, the pipeline will remain executable but harder to trust, compare, and iterate on

### Risk: markdown-only drift for PDFs
- if the real builder treats markdown as the only normalized representation, the system will silently lose figure, chart, and layout semantics before `EvidenceIR`

### Risk: ungrounded visual descriptions
- if multimodal descriptions are generated without stable links back to page regions, captions, and source references, later stages will be vulnerable to hallucinated evidence
### Risk: mixed Rust/Python backend seam
- the SourceIR PDF path now depends on a Rust-to-Python orchestration boundary and an external Docling runtime
- interpreter discovery, package installation, and first-run model downloads are operational concerns that must stay explicit in docs and tests
- this is acceptable temporarily, but should be closed soon so the first stage is truly operational for PDFs

## Testing implications
- current full local CI path: `bash scripts/run_ci.sh`
- current active Rust surface after the README/bootstrap refresh: `31` Rust source files and `62,689` total lines under `crates/specforge/src`
- current Rust test count observed through the canonical local CI path after the latest slice: 357 library tests, 0 binary tests, and 0 doc tests, all passing under warning-deny Clippy/rustdoc plus the mdBook build
- current tracked KG-quality fixture count: 90
- current tests cover:
  - source-kind detection
  - deterministic source key naming
  - `SourceIR` JSON materialization
  - directory residual decision emission
  - PDF normalization planning
  - PDF materialization through a stubbed backend override
  - Docling table-kind and diagram-kind classification
  - markdown-backed `EvidenceIR` construction
  - table-synthesized signal, enum, register, and timing evidence
  - anchored encoding-table rescans and dynamic value-constraint extraction
  - polarity refinement from active-low / active-high prose, including explicit asserted-when-level, collective active-level, safe mixed clause-local control wording, and detached mixed-polarity rejection
  - caption and figure-reference grounding into visual evidence
  - VLM observation injection (TimingDiagramExtraction, StateMachineExtraction from VisualAsset.note)
  - handshake-driven `SemanticIR` actor/interface/invariant extraction
  - structured-table signal direction+width extraction through EvidenceIR → SemanticIR
  - sticky SemanticIR interface direction/width conflict collapse when conflicting declarations repeat
  - parametric-width handling through the IR pipeline
  - VLM timing diagram annotation → TimingConstraintRecord in SemanticIR
  - VLM timing signal-value filtering for waveform motion states, separator variants, and compact edge spellings such as rising/stable/falling/RISING_EDGE/LOW_TO_HIGH/POS_EDGE/LOW2HIGH
  - VLM state machine extraction → duplicate-merged identifier-bounded RegularStateRecord plus same-observation declared-endpoint-gated StateTransitionRecord in SemanticIR
  - ambiguous visual-grounding residual decisions in `SemanticIR`
  - handshake-driven `IntentIR` identity/behavior/constraint/assumption construction
  - residual-decision preservation from `SemanticIR` into `IntentIR`
  - NLP Level 3 extraction, backannotation, and alias learning
  - markdown/table/list marker alias rejection for Form 2 alias learning
  - actor-signal relation extraction from prose and table roles
  - AMBA `Source` / `Driver` / `Destination` signal-direction handling
  - `.fsm` adapter renderability, including graph-backed top-composition child direction recovery, unambiguous standalone direct direction recovery, sticky actor-port direction/width conflict blocking, graph-backed sequential system-contract direction recovery, graph-only direct-context filtering, top-link boundary direction recovery, and conflict/ambiguity blocking
  - `specforge validate` for all four IR stages
  - project-level validation projection and schema-v2 rescan-plan generation
  - schema-v2 `rescan-plan` normalization, whitelisted execution, execution summaries, and no-promotion review gates
  - bounded prior-family harvest/consumption for actor taxonomy, semantic phrases, temporal phrases, table shapes, semantic modality reliability, visual motifs, and negative knowledge
  - KG-quality fixture execution across gold, negative, conflict, multimodal, prior-guided, infrastructure, and active-low VLM timing polarity-equivalence cases
- next tests should cover:
  - richer APB and AXI end-to-end fixtures for relation-driven direction coverage
  - actor-relative direction modeling once it lands in `SemanticIR` / `IntentIR`
  - wider `.fsm` renderability coverage and snapshot stability on protocol-heavy fixtures
  - future adapter targets beyond the current `.fsm` slice

## Latest validation completed in this refresh
- `bash scripts/run_ci.sh`
  - passed; Rust test suite reported 357 passed tests under warning-deny CI, 0 failures, 0 binary tests, 0 doc tests, rustdoc completed under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build completed successfully

## Session update (2026-04-19 README bootstrap analysis refresh)
- Re-executed the README handoff path through `SESSION_BOOTSTRAP.md`, reread the linked continuity and user-facing markdown surfaces, and resurveyed the active Rust crate layout directly from disk.
- The current Rust implementation now spans `31` source files and `62,689` lines under `crates/specforge/src`, with the tracked KG fixture suite at `90` and the canonical local CI path at `357` passing Rust tests plus warning-deny rustdoc and mdBook validation.
- The bootstrap pass did not reveal a new architectural pivot or an unlogged product-surface drift; the recent graph-direction validation work is already represented in the live docs.
- The meaningful action from this refresh is simply keeping the bootstrap analysis truthful, so future resumed sessions start from current numbers instead of stale ones.

## Session update (2026-04-18 README bootstrap analysis refresh)
- Re-executed the README handoff path through `SESSION_BOOTSTRAP.md`, `ROADMAP.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `MEMORY.md`, and `COMMIT.md`, then resurveyed the active Rust crate layout directly from disk.
- The current Rust implementation now spans `31` source files and `62,017` lines under `crates/specforge/src`, with the tracked KG fixture suite at `89` and the canonical local CI path at `351` passing Rust tests plus warning-deny rustdoc and mdBook validation.
- The bootstrap pass did not reveal a new architectural pivot, but it did reaffirm the most important remaining codebase-level gap from the roadmap: `R15` is still incomplete because several downstream adapter and validation seams continue to consume compatibility `direction_hint` rather than purely actor-relative graph semantics.
- That means the next meaningful implementation work should favor graph-first downstream direction consumers over more adapter-family breadth or superficial scoring tweaks.

## Earlier validation trail
- `cargo run --manifest-path Cargo.toml -p specforge -- --help`
  - passed and confirmed the current CLI surface includes `enrich`, `validate`, and `nlp-enrich`
- `cargo test --manifest-path Cargo.toml`
  - passed with 99 tests
- `cargo run -p specforge -- inspect README.md`
  - passed; confirmed the repo entry source is detected as markdown
- `cargo run -p specforge -- ingest README.md --dry-run`
  - passed; confirmed `SourceIR` planning for the README entrypoint
- `cargo run -p specforge -- ingest README.md`
  - passed; materialized `generated/source_ir/readme/source_ir.json`
- `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run`
  - passed; confirmed README-backed `EvidenceIR` preview
- `cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json`
  - passed; materialized `generated/evidence_ir/readme/evidence_ir.json` with 15 section anchors and 190 extracted statements
- `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run`
  - passed; confirmed README-backed `SemanticIR` preview
- `cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json`
  - passed; materialized `generated/semantic_ir/readme/semantic_ir.json` with 2 actors, 6 phases, 5 invariants, and 8 gates
- `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run`
  - passed; confirmed README-backed `IntentIR` preview
- `cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json`
  - passed; materialized `generated/intent_ir/readme/intent_ir.json` with 2 actors, 14 behaviors, 6 constraints, and 1 assumption
- `cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm --dry-run`
  - passed; produced the expected honest blocked `.fsm` adapter plan for the README-derived intent surface

## Current recommendation
- keep the current single-crate workspace for one more slice
- keep `IntentIR` canonical and resist any temptation to make `.fsm` the hidden endpoint again
- take the alias-marker cleanup as complete and treat it as evidence that the current multi-spec extraction stack is ready for the next slice
- finish promoting the signal model from compatibility hints to actor-relative direction semantics next, then extend the same validation/reporting discipline into downstream RTL adapter work
- do not treat NLP Level 3 as the missing piece anymore; the pipeline now has both Level 3 enrichment and convergent typed EvidenceIR reuse

## Session update (2026-04-18 VLM timing bus-label annotation rejection)
- Continued from commit `d7f7281`, hardening VLM timing truthfulness rather than broadening extraction authority.
- `is_generic_waveform_label_token()` now recognizes short bus/waveform labels such as `Burst`, `Packet`, `Frame`, `Transaction`, and `Txn` as the same low-value annotation family as `Addr`, `Cycle`, `D0`, `DATA[3]`, and `XREQ[3:0]`.
- This closes a realistic chip-spec PDF failure mode where timing-diagram annotation gutters label burst/transaction phases, but those labels are descriptive figure markup rather than timing constraints that should survive into canonical IR.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock the expanded annotation-noise set while preserving the grounded `signals[].values[]` temporal evidence path.

## Session update (2026-04-18 Compact VLM timing bus-label rejection)
- Continued from commit `619a773`, tightening the same timing truthfulness boundary for compact diagram layout.
- `is_compact_waveform_sample_label()` now recognizes `Burst1`, `Packet2`, `Frame3`, `Transaction4`, and `Txn5` as the compact equivalents of the already-filtered spaced bus labels.
- This closes the space-constrained waveform-gutter variant common in chip-spec PDFs, where bus-phase labels are squeezed into one token but still do not encode protocol timing law.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock both spaced and compact bus-label annotation noise while preserving the grounded temporal sample path.

## Session update (2026-04-18 Compact VLM phase/transfer label rejection)
- Continued from commit `867455e`, tightening the same compact-layout timing truthfulness boundary one step further.
- `is_compact_waveform_sample_label()` now recognizes `Phase1` and `Transfer2` as compact waveform-gutter labels rather than timing constraints.
- This closes the remaining compact-layout gap inside the existing generic waveform label vocabulary: `phase` and `transfer` were already filtered when tokenized, but not when layout collapsed them into a single token.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock compact phase/transfer labels alongside the existing spaced and compact annotation-noise families.

## Session update (2026-04-18 Multi-token VLM waveform-label rejection)
- Continued from commit `5996029`, tightening the same annotation-noise boundary for short multi-token waveform-gutter labels.
- `is_spurious_timing_annotation_label()` now rejects all-generic annotation groups up to four tokens instead of three.
- This closes a real leak where labels such as `Channel 1 Phase 2` or `Lane 0 Slot 1` could previously survive as `TimingConstraintRecord`s even though every token was already in the generic waveform-label vocabulary.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock these four-token markup labels alongside the shorter spaced and compact label-noise families.

## Session update (2026-04-18 Name-only VLM timing label rejection)
- Continued from commit `74a74de`, tightening the same timing truthfulness boundary for bare known-signal labels.
- `parse_timing_diagram_observation()` now rejects annotations that are only a known signal name, using the document-grounded `known_signal_names` set rather than generic waveform-label heuristics.
- This closes a separate leak where labels such as `XREQ` could previously survive as `TimingConstraintRecord`s simply because they were neither generic markup nor sentence-shaped timing statements.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock bare known-signal labels alongside the existing generic spaced, compact, and multi-token annotation-noise families.

## Session update (2026-04-18 Signal-value VLM timing label rejection)
- Continued from commit `53f5953`, tightening the same timing truthfulness boundary for known-signal sampled-value lane labels.
- `parse_timing_diagram_observation()` now rejects annotations that are exactly a known signal plus one simple sampled-value token such as `HIGH`, `LOW`, `asserted`, or `deasserted`.
- This closes the follow-on leak where labels such as `XREQ HIGH` could previously survive as `TimingConstraintRecord`s even though the structured `signals[].values[]` path already represents sampled VLM timing values more honestly.
- The direct semantic regression plus `vlm_timing_spurious_annotation_negative` now lock these two-token known-signal label variants alongside the existing name-only, generic spaced, compact, and multi-token annotation-noise families.
## Session update (2026-04-19 graph-direction conflict vs coverage-gap split)
- Continued from commit `f8164d0`, tightening the warning boundary inside validation rather than adding new canonical data.
- Same-actor graph-direction conflicts no longer also emit the generic `*_graph_direction_coverage_incomplete` finding.
- The split is now cleaner:
  - conflict findings mean graph evidence exists but disagrees
  - coverage-gap findings mean the graph still provides no usable direction for those signals
- Updated the tracked `graph_direction_same_actor_conflict_negative` fixture to assert `finding_ids_exclude` for the generic coverage-gap finding, so the benchmark surface now locks that non-overlap directly.
- Formatting, four focused validator regressions, two focused tracked fixture runs, full local CI, and whitespace checking passed for this slice.

## Session update (2026-04-19 graph-direction coverage finding related IDs)
- Continued from commit `eeb67a8`, tightening the graph-direction validation surface rather than widening extractor or adapter scope.
- `semantic_graph_direction_coverage_incomplete` and `intent_graph_direction_coverage_incomplete` now emit the missing signal names in `related_ids` instead of remaining count-only findings.
- The split with the previous conflict work stays deliberate:
  - coverage-gap findings remain signal-level because they answer which canonical signals still lack graph-derived direction
  - same-actor contradiction findings remain actor-aware because they answer which actor-signal pair caused the unresolved conflict
- Added focused validator regressions for both canonical stages plus a tracked `graph_direction_coverage_incomplete_negative` fixture.
- Refreshed the managed corpus-KB benchmark and pattern pages; the tracked truthfulness suite now reports `91/91` fixtures and the semantic/truthfulness pattern page reports `50` fixtures.
- Formatting, two focused validator regressions, the focused tracked fixture run, full local CI, corpus-KB refresh, and whitespace checking passed for this slice.
