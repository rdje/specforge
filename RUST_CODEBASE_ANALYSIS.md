# RUST_CODEBASE_ANALYSIS
## Purpose
- maintain a live, deep-dive analysis of the Rust codebase
- record the current architecture, risks, subsystem boundaries, and recommended implementation direction
- remain useful even while only the early IR stages are implemented

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
  - `nlp-enrich`
- the canonical product boundary remains `IntentIR`, not `.fsm`
- the staged pipeline is operational through `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- a whole-pipeline fixed-point entrypoint now exists via `specforge converge`, which reuses persisted artifacts, defaults to Ollama VLM + NLP Level 3, and stops when the cross-stage knowledge snapshot is stable
- `SourceIR` now captures structured Docling output, typed content elements, structured tables, visual assets, and document-profile metadata
- `EvidenceIR` now synthesizes typed declarations and records from tables, preserves typed NLP outputs, persists alias-learning state, extracts actor-signal relation triples from prose and signal-description tables, runs a monotone convergence loop so discovered enum facts and explicit active-level polarity prose can unlock additional signal constraints without hardcoded protocol-specific value lists, and now keeps polarity disagreement explicit through typed conflict records instead of only via a neutralized fallback
- `SemanticIR` now lifts that evidence into interfaces, explicit interface-signal conflict records for conflicting direction/width evidence, actor-relative port/connectivity records, explicit signal-connectivity conflict records for unresolved multi-producer ambiguity, system/reset/init records, control/state records, timing/register records, and filtered NLP constraints, with VLM observations merged into the semantic surface
- `IntentIR` now carries forward the canonical signal/control/system/state/register/timing surface plus the actor-relative KG needed for honest downstream lowering
- the current `.fsm` adapter slice is real and intentionally narrow: it can emit honest `?dt:name`, `?fsm:name`, and `?top:name` outputs when the canonical facts are explicit enough
- the enrichment, convergence, validation, benchmark, targeted-rescan, first prior-learning, and first corpus-knowledge toolchain is also real: `specforge enrich`, `specforge nlp-enrich`, `specforge converge`, `specforge validate`, `specforge project-validation`, `specforge rescan-plan`, `specforge kg-bench`, `specforge learn-priors`, and `specforge corpus-kb` are wired into the CLI and exercised by the workspace tests
- `project-validation` and `rescan-plan` now form a schema-v2 targeted-rescan loop: recommendations carry typed replay inputs, structured local command hints, dry-run-by-default execution, before/after validation snapshots, execution summaries, promotion-gate descriptors, and an explicit no-canonical-mutation boundary
- `CorpusMemory` schema v5 now carries actor-taxonomy, semantic-phrase, semantic-modality-reliability, temporal-phrase, table-shape, visual-motif, and negative-knowledge prior families; current consumers remain advisory and locally grounded rather than fact-authoring
- the first `R15g` corpus knowledge-base plane now exists too: tracked `corpus_kb/` pages can preserve reviewable cross-document synthesis, and `specforge corpus-kb` refreshes managed validation-finding, KG fixture-result, dedicated semantic/truthfulness pattern, typed-prior-memory, table/visual/state-machine/timing/infrastructure/protocol-family pages, and review-only prior-candidate pages plus a schema-versioned readiness manifest from validation report sidecars / benchmark fixture outcomes without mutating canonical IR or typed priors; the KG fixture-result path now uses quiet fixture-local validation and projects fixture-family summaries so coverage is visible beyond aggregate pass/fail counts
- the tracked KG-quality benchmark surface currently contains 68 fixtures, including direct signal-connectivity conflict shape coverage for multi-producer graph conflicts, direct signal-semantic conflict shape coverage for multimodal disagreement, direct interface-signal conflict shape coverage for direction/width disagreement, direct signal-polarity conflict shape coverage for active-level prose/table disagreement, direct resolved signal-polarity shape coverage for active-high/active-low recovery, direct resolved semantic-role shape coverage for valid-like/ready-like recovery, direct semantic-grounding strength coverage for single-source/cross-modality evidence quality, direct temporal-conflict shape coverage for the negative-knowledge caution path, direct temporal-rule shape coverage for representative AXI/APB/AHB timing fixtures including APB write-control stability, APB response stability, AHB control stability, AHB response stability, AHB write-data stability, and AXI write response, read address, read data, write data, and sideband stability, direct clock/reset infrastructure-topology coverage, VLM timing spurious-annotation/sample-index-label rejection, motion-only timing-annotation rejection, waveform-motion state rejection, VLM state-machine label-noise, undeclared-transition, duplicate-initial, multiple-initial, and missing-initial coverage, active-low VLM timing polarity-equivalence coverage, and collective, mixed clause-local, and detached mixed-polarity negative non-reset control polarity coverage
- the local runtime boundary is now operationally stronger too: `specforge doctor` reports Docling readiness, the default Ollama loopback readiness, and LM Studio fallback readiness directly, repo-local `.venv-docling` auto-discovery is supported, and the backend now probes versioned Python candidates like `python3.11` before giving up on fresh ingest
- GitHub Actions CI is part of the repo baseline and still runs `cargo fmt --all --check`, warning-deny Clippy, warning-deny Rust tests, warning-deny rustdoc, and the mdBook build when launched manually, but automatic `push` / `pull_request` triggers are temporarily paused to conserve account Actions minutes
- that CI path still has a single checked-in entrypoint at `scripts/run_ci.sh`, and the GitHub workflow calls that script directly so local and hosted Rust validation do not drift apart
- the remaining dominant gaps are semantic-truthfulness gaps: finishing the remaining graph-first consumers, deepening the temporal-rule layer into richer actor-relative and contradiction-aware clocked semantics, KG-guided rescans, evidence arbitration, benchmark-quality evaluation, and deepening the now-started `R15g` corpus knowledge base beside the already-live typed prior-memory plane; adapter expansion is now horizon work
- the workspace currently validates through `bash scripts/run_ci.sh`, which runs Rust formatting, Clippy with `-D warnings`, Rust tests with `RUSTFLAGS="-D warnings"`, rustdoc with `RUSTDOCFLAGS="-D warnings"`, and the mdBook docs build; after the AHB write-data stability KG fixture slice the full local CI path reports clean Clippy, `313` passing Rust tests, clean Rust API docs, and a successful mdBook build

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
- the temporal-rule layer now also recovers bounded `cycle_window` latency from structured prose/timing text, including idiomatic one-cycle phrases like `next cycle`, `next tick`, and `next rising edge`, and exposes that coverage in validation
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
  - a representative APB write-control stability fixture that proves `PWRITE`, `PWDATA`, and `PSTRB` stay actor-grounded stable requester outputs through wait-state constraints without false handshake completion
  - a representative APB response stability fixture that proves `PRDATA` and `PSLVERR` stay actor-grounded stable completer outputs through completed-access constraints with real handshake completion
  - a representative AHB control stability fixture that proves `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, and `HPROT` stay actor-grounded stable manager outputs through wait-state constraints without false handshake completion
  - a representative AHB response stability fixture that proves `HRDATA` and `HRESP` stay actor-grounded stable subordinate outputs through wait-state constraints without false handshake completion
  - a representative AHB write-data stability fixture that proves `HWDATA` stays actor-grounded stable as a manager output through three-predicate write wait-state constraints without false handshake completion
  - a representative AXI-style gold fixture that proves width-only channel tables plus prose drive/sample relations recover actor-relative direction, signal inventory, request/accept semantics, and typed handshake completion without any table direction column
  - a representative AXI-style timing gold fixture that proves width-only channel tables plus prose actor relations can also recover next-cycle timing, actor-grounded temporal predicates, and handshake completion together
  - a representative AXI write-response timing fixture that proves the same path on `BVALID` / `BREADY` / `BRESP`, including response-valid timing and response-payload stability
  - a representative AXI read-address timing fixture that proves the same path on `ARVALID` / `ARREADY` / `ARADDR` / `ARLEN`, including read-address-ready timing and read-address payload stability
  - a representative AXI read-data timing fixture that proves the same path on `RVALID` / `RREADY` / `RDATA` / `RRESP`, including read-data-valid timing and read-data payload stability
  - a representative AXI write-data timing fixture that proves the same path on `WVALID` / `WREADY` / `WDATA` / `WSTRB`, including write-data-ready timing and write-data payload stability
  - a representative AXI sideband stability fixture that proves `ARLEN` and `WSTRB` remain actor-grounded sideband obligations under their controlling ready/valid handshakes
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
- current Rust test count observed through that path after the current slice: 313 library tests, 0 binary tests, and 0 doc tests, all passing under `RUSTFLAGS="-D warnings"` after clean `cargo clippy --manifest-path Cargo.toml --all-targets -- -D warnings` and `RUSTDOCFLAGS="-D warnings" cargo doc --manifest-path Cargo.toml --no-deps` passes
- current tracked KG-quality fixture count: 68
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
  - passed; Rust test suite reported 313 passed tests under `RUSTFLAGS="-D warnings"`, 0 failures, 0 binary tests, 0 doc tests, rustdoc completed under `RUSTDOCFLAGS="-D warnings"`, and the mdBook build completed successfully

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
