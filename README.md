# SpecForge
This file is the single entry point for the project.
Use it first for the project objective, document navigation, and the current implementation map.

## Project objective
- build `specforge` as a staged Rust toolchain for extracting implementation-relevant intent from protocol, component, system, and software-interface specifications
- make the canonical deliverable a backend-independent `IntentIR`, serialized as JSON or a future equivalent interchange format
- treat `.fsm`, SystemVerilog, Verilog, and VHDL as adapter targets downstream of `IntentIR`, not as the core product boundary
- push automation as far as safely possible, while representing unresolved ambiguity as structured residual decision packets instead of ad hoc manual gaps
- preserve crash-safe continuity through live documentation so a new AI or LLM session can resume work quickly and correctly
- implement protocol semantics as a typed domain model plus evidence aggregation, using AI only as a bounded hypothesis generator rather than as an end-to-end black-box reader

## Canonical user-facing docs
- the canonical user-facing documentation surface now lives in the mdBook under `docs/book/`
- the mdBook should be treated as a live book that evolves alongside user-facing project changes, not as a static scaffold
- the mdBook is the public-facing documentation product for `specforge`: it should openly explain what the tool does, how it works, and why it is designed that way
- every meaningful user-facing aspect of the project should land in the book with its own section or chapter as the coverage grows
- source entry point: `docs/book/src/introduction.md`
- local build:
  - `mdbook build docs/book`
  - or the repo CI wrapper: `bash scripts/run_docs_ci.sh`
- the mdBook is structured in increasing depth:
  - architecture rationale
  - getting started
  - runtime and `doctor`
  - command workflow
  - IR pipeline semantics
  - domain model semantics
  - validation and learning semantics
  - validation, learning, and troubleshooting
- root markdown docs still exist, but they now serve continuity, roadmap, validation projection, and developer-state purposes rather than being the primary user-doc surface

## Current repository state
- the live-document surface has been pivoted around `IntentIR` as the canonical endpoint
- the Rust workspace and active CLI/crate identity are now `specforge`
- the current `specforge` CLI surface supports:
  - `inspect <path>`
  - `doctor [--strict]`
  - `converge <source> --target fsm [--rescan-plan <plan>]`
  - `ingest <source> --dry-run`
  - `ingest <source>`
  - `evidence <source-ir> --dry-run`
  - `evidence <source-ir>`
  - `semantic <evidence-ir> --dry-run`
  - `semantic <evidence-ir>`
  - `intent <semantic-ir> --dry-run`
  - `intent <semantic-ir>`
  - `adapt <intent-ir> --target fsm --dry-run`
  - `adapt <intent-ir> --target fsm`
  - `kg-bench`
  - `project-validation <artifact>...`
  - `rescan-plan [--execute]`
  - `learn-priors <intent-ir>...`
  - `corpus-kb [validation-report]... [--kg-fixtures-root <fixture-root>]`
  - `clean [--execute] [--scope <scope>]`
- `specforge ingest` now computes and materializes `SourceIR` at `generated/source_ir/<document_key>/source_ir.json`
- PDF re-ingest now stages normalization into `generated/source_ir/<document_key>/normalized.staging` and only swaps it into `normalized/` after backend success, so stale page/image leftovers from older runs do not accumulate and a failed rerun does not destroy the last good normalized bundle
- `specforge evidence` now computes and materializes `EvidenceIR` at `generated/evidence_ir/<document_key>/evidence_ir.json`
- `specforge semantic` now computes and materializes `SemanticIR` at `generated/semantic_ir/<document_key>/semantic_ir.json`
- `specforge intent` now computes and materializes `IntentIR` at `generated/intent_ir/<document_key>/intent_ir.json`
- `specforge adapt --target fsm` now computes and materializes typed adapter artifacts at `generated/adapters/fsm/<document_key>/adapter.json`
- `.fsm` adapter lowering is now partially graph-first: explicit module/top-composition paths recover child-module port directions from matching `IntentIR.actor_ports`, standalone direct roots can recover missing local-inventory directions when the direct actor context is unambiguous or when one target actor graph-drives all direct output targets despite external shared-signal actors, direct and explicit-module control reads can fill target-actor input directions after that target actor is selected across DT and true FSM roots, top-link topology can recover and retain top-boundary port directions even when another composition gate still blocks emission, repeated actor-port direction/width conflicts stay unresolved instead of self-healing by duplication, and standalone sequential DT roots now have regression coverage for graph-backed clock/reset system-contract direction recovery
- `specforge converge <source> --target fsm` now ingests once, runs Ollama-backed VLM figure enrichment and NLP Level 3 by default, rebuilds the downstream IR stages, and stops when the persisted knowledge snapshot is stable across passes; use `--vlm-provider skip` and/or `--nlp-provider skip` only when you explicitly want a narrower run, and use `--rescan-plan <plan>` plus optional `--execute-rescan-plan` only when you deliberately want the stabilized loop to inspect or execute replayable validation rescan hints
- the intended architecture is not "teach code to understand unrestricted English"; it is "teach the pipeline to recover typed protocol facts from multimodal evidence and validate them aggressively"
- temporal arbitration is now polarity-aware too: `ASSERTED` / `DEASSERTED` only collapse to `HIGH` / `LOW` when the current document actually grounds signal polarity
- resolved signal polarity now also lives directly on canonical `InterfaceSignalRecord` entries, with validator coverage reported as `with_resolved_polarity`
- clock/reset infrastructure now carries bounded explicit topology hints for current-document gated clock branches, reset synchronizer stages, and reset-tree targets without treating them as ordinary protocol edges or full physical tree proof
- generic clock/reset best-practice prose now has explicit negative benchmark coverage too: advice such as glitch avoidance, no glue logic on reset trees, possible synchronizer use, or async-assert/sync-release discipline can preserve first-class clock/reset infrastructure semantics, but it must not mint concrete topology records without current-document topology evidence
- protocol and chip-interface PDFs are treated as contract sources for RTL designers and verification-IP authors; SPECFORGE should recover boundary-visible clock/reset contract semantics from them, not infer SoC-specific physical clock/reset tree construction that belongs to the integrating chip team
- `specforge validate <artifact>` now writes a deterministic stage-local `validation_report.json` sidecar and backannotates the latest report into the artifact's `validation_reports` field
- `specforge project-validation <artifact>...` now validates the passed artifacts and refreshes the tracked validation snapshot docs from their persisted reports, including review-facing projection of any preserved rescan execution summaries
- `specforge project-validation` also writes local `generated/validation/rescan_plan.json` schema v2 recommendations with typed replay inputs, structured command hints, and `planned_not_executed` status for targeted rescan loops; the compact live-status queue now also projects the replay-input kind chain plus a concise action summary so operators can see replay scope without opening the raw JSON plan, evidence-stage signal-polarity conflicts now route to a bounded local `nlp-enrich -> validate` replay guidance keyed by exact `polarity_conflict_*` ids, evidence-stage signal-semantic conflicts now route to a bounded local `nlp-enrich -> validate` replay guidance keyed by exact `semantic_conflict_*` ids, non-decisive semantic-role arbitration findings now route to bounded local `nlp-enrich -> semantic -> intent? -> validate` replay guidance, protocol connectivity findings for missing producers or consumers now route through that same local replay lane, carried interface-signal conflicts now route through it too as conflict-id replay targets for unresolved direction/width disagreement, carried typed temporal conflicts now route through it too as conflict-id replay targets for contradictory timing obligations, carried signal-connectivity conflicts now route through it too as conflict-id replay targets, carried signal-polarity conflicts now route through it too as conflict-id replay targets for unresolved active-level disagreement, carried signal-semantic conflicts now route through it too as conflict-id replay targets for unresolved semantic-role disagreement, carried graph-direction self-conflicts now route through it too as actor-aware conflict-id replay targets for unresolved same-actor direction disagreement, and visual-motif corroboration findings now get an explicit `enrich_source_ir` hint before the downstream `EvidenceIR` rebuild and validation hints, with `--rescan-vlm-provider auto-local` preferring a ready local Ollama model and falling back to a ready local LM Studio model
- `specforge clean` now provides a first-class local artifact reclamation path: dry-run by default, `--execute` to delete, `--scope source-normalized` to reclaim heavyweight `generated/source_ir/<document_key>/normalized` bundles while preserving `source_ir.json`, `--scope document [--document-key <key>]` to remove full per-document generated stage trees, and `--scope all-generated` to sweep the whole local `generated/` root when you intentionally want a cold local rebuild
- `specforge rescan-plan` now reads that local schema-v2 plan, dry-runs pending targets by default, and executes only whitelisted in-process local enrichment/stage rebuild/validate hints when explicitly passed `--execute`, recording neutral before/after validation changed/no-change status plus an `execution_summary` with before/after validation snapshots, deltas, a review-required arbitration verdict when the validation surface changes, and an explicit `promotion_status` / `promotion_blockers` / `promotion_review` gate so even favorable deltas remain `not_promoted_review_required`, `human_review_required`, and `canonical_mutation_allowed: false` until current-document evidence review and an explicit future approval record approve them; the dry-run preview now prints `replay_inputs`, `recommended_action`, and `automation_status` before the command hints so a human can inspect replay scope directly, `--document-key <key>` can scope multi-document plans, and `converge --rescan-plan <plan>` reuses the same guarded executor after stability with an automatic current-document filter and an arbitration status rather than treating any changed artifact as automatically improved
- `promotion_review` is a review-requirement descriptor, not an approval artifact; future approval artifacts stay local/generated by default until an explicit canonical IR mutation workflow exists and deliberately defines tracked approval evidence
- `specforge kg-bench` now runs tracked KG-quality fixtures through the staged pipeline, so gold expectations, negative expectations, residual quality, conflict surfacing, and prior-guided caution behavior can be checked explicitly instead of relying only on aggregate scores
- `specforge kg-bench` can now assert canonical signal-inventory exclusions directly with `signal_names_exclude`, so fixtures can prove that document-scope or integration vocabulary such as `PDF`, `VIP`, `PLL`, `DFT`, and `SoC` does not become fake interface signals
- `specforge kg-bench` can now assert exact EvidenceIR table-signal provenance with `table_signal_declaration_provenance_include` plus direct provenance counts with `table_signal_declaration_provenance_count`, so fixtures can prove that table-synthesized declarations point at the intended source table and that false-positive table shapes produce zero provenance before canonical carry-through; the harness self-tests the count, missing-record, and statement-text mismatch diagnostics for this surface
- `specforge kg-bench` can now assert canonical signal table provenance directly with `signal_supporting_table_ids_include`, so fixtures can prove that recovered interface signals are backed by specific structured `SourceIR` tables rather than only by synthesized statement text; the harness self-tests `SemanticIR` and `IntentIR` wrong-support and absent expected-signal diagnostic paths
- `specforge validate` now reports `table_signal_declaration_provenance` for `EvidenceIR`, so users can see whether table-synthesized signal declarations still point back to their source table before canonical carry-through
- `specforge validate` now reports `with_table_support` for `SemanticIR` and `IntentIR`, so users can see how much of the canonical signal inventory still has structured-table provenance
- `specforge kg-bench` can now assert canonical `signal_connectivity_conflicts` directly, so multi-producer graph conflicts lock the conflicting signal, conflict kind, and actor set rather than only aggregate conflict counts
- `specforge kg-bench` can now assert canonical `signal_semantic_conflicts` directly, so multimodal disagreement fixtures lock the conflicting evidence shape rather than only aggregate conflict counts
- `specforge kg-bench` can now assert canonical `interface_signal_conflicts` directly, so interface declaration conflicts lock direction/width disagreement shape rather than only aggregate conflict counts
- `specforge kg-bench` can now assert canonical `signal_polarity_conflicts` directly, so active-level disagreements lock polarity/source-kind observation shape rather than only aggregate conflict counts
- `specforge kg-bench` can now assert canonical resolved signal polarity directly, so active-high/active-low recovery locks per-signal IR shape rather than only `with_resolved_polarity` counts
- `specforge kg-bench` can now assert canonical resolved semantic roles directly, so valid-like/ready-like recovery locks exact per-signal IR shape rather than only role-presence counts
- `specforge kg-bench` can now assert canonical semantic grounding strength directly, so single-source/multi-source/cross-modality grounding stays locked as per-signal IR shape rather than only validation counters
- `specforge kg-bench` can now assert canonical `temporal_rules` directly, so APB/AHB/AXI timing fixtures lock typed antecedents, consequents, cycle windows, clock/edge grounding, and actor-grounded predicates rather than only aggregate temporal metrics
- `specforge kg-bench` can now assert canonical `temporal_conflicts` directly, so contradiction fixtures lock the actual conflict signal, phase, context, values, and supporting evidence instead of only aggregate conflict counts
- `specforge kg-bench` can now assert canonical clock/reset `infrastructure_signals` and `infrastructure_topology` records directly, so infrastructure truthfulness is checked as typed IR rather than only through aggregate validation metrics
- the tracked KG fixture set now also includes a signal-table inventory-authority negative fixture: real protocol signals such as `XREQ`, `XACK`, and `PAYLOAD` are recovered from a structured `Signal | Direction | Width | Description` table, while uppercase prose context such as `PDF`, `RTL`, `VIP`, `PLL`, `DFT`, `CDC`, `CTS`, `ECO`, and `SoC` stays out of canonical signal inventory
- table-synthesized signal declarations now preserve their originating table ids through `EvidenceIR.table_signal_declaration_provenance` and `InterfaceSignalRecord.supporting_table_ids`, so `SemanticIR` / `IntentIR` can explain which source table supported a canonical signal record
- `specforge learn-priors <intent-ir>...` now builds a local typed `CorpusMemory` prior store under `generated/prior_memory/corpus_memory.json`, harvesting only from validated `IntentIR` artifacts and keeping the learning plane advisory-only
- `specforge corpus-kb [validation-report]... [--kg-fixtures-root <fixture-root>]` now refreshes tracked `R15g` corpus knowledge-base pages under `corpus_kb/` from reviewable evidence such as validation reports, KG fixture outcomes, KG fixture-family summaries, dedicated semantic/truthfulness pattern and typed-prior-memory/table/visual/state-machine/timing/infra/protocol family pages, and review-only prior-candidate projections with family-level gate matrices plus a schema-versioned readiness manifest, using managed blocks so human synthesis is preserved and canonical IR truth is never mutated
- `specforge doctor [--strict]` now inspects Docling ingest readiness, the default Ollama loopback path, and the LM Studio fallback loopback path, reporting the selected Python candidate plus endpoint/model status for both local providers so the local-first pipeline can be preflighted before a long converge run
- Docling runtime discovery is now more robust too: `specforge` first honors `SPECFORGE_DOCLING_PYTHON`, then auto-discovers a repo-local `.venv-docling`, then probes versioned Python candidates such as `python3.11` before falling back to generic `python3` / `python`
- `scripts/bootstrap_docling.sh` now provides the supported repo-local bootstrap path for Docling-backed PDF ingest, with `.venv-docling/` kept local and untracked
- the thing that grows to materialize learning is that typed prior store, not the code and not hidden neural weights: the code defines how priors are harvested/consumed, while `generated/prior_memory/corpus_memory.json` accumulates the learned reusable extraction knowledge over time
- the next cross-document layer after that prior store should be a tracked corpus knowledge base: a persistent compiled synthesis plane beside the KG and `CorpusMemory`, where recurring protocol motifs, extraction failures, contradiction summaries, table/figure families, and infrastructure-semantics notes can accumulate without contaminating per-document canonical IR truth
- `specforge evidence <source-ir>` and `specforge converge <source>` now consult that local prior store by default through `--prior-memory generated/prior_memory/corpus_memory.json`, and the first bounded consumers now use:
  - actor-taxonomy priors to interpret explicit local actor terms already present in section headings and `Source` / `Destination` table columns, including recovering structural `ActorSignalRelation::Drives` edges from width-only section-guided signal tables
  - semantic phrase priors to interpret locally grounded signal-description/prose phrases that normalize to learned semantic-role evidence without weakening the name-noise protections
- `SemanticIR` now has a third bounded prior consumer too: if the current PDF contains local timing text whose phrase shape matches a learned temporal prior and the built-in parser still cannot recover a cycle window on its own, the semantic stage can advisory-recover that cycle window without inventing a timing rule that is not already locally grounded
- `EvidenceIR` now has a fourth bounded prior consumer too: if ingest left a current table as `unknown` but the local header shape matches a learned table-shape prior, evidence extraction can advisory-recover that table kind locally without rewriting `SourceIR` or overriding explicit local table classifications
- GitHub Actions CI is temporarily manual-only through `workflow_dispatch` to conserve account Actions minutes, but the workflow still runs `cargo fmt --all --check`, warning-deny Clippy, warning-deny Rust tests, warning-deny rustdoc, and the mdBook build when manually launched
- the hosted CI path is still driven by `./scripts/run_ci.sh`, so the exact Clippy + warning-deny Rust/rustdoc + docs CI suite remains runnable locally before push even while automatic hosted `push` / `pull_request` triggers are paused
- the current `R15f` slice now learns five safe prior families:
  - actor-taxonomy priors from decisive actor-grounded handshake-role evidence plus conservative self-identifying actor vocabulary (`requester`, `completer`, `manager`, `subordinate`, and similar explicit role terms)
  - semantic-role phrase priors from decisive, non-alias-dependent semantic consensus plus preserved observation text
  - semantic modality-reliability priors from decisive, non-alias-dependent semantic consensus plus the supporting source kinds that carried that consensus
  - temporal-language phrase priors from canonical temporal rules and validated canonical `signal_constraints` / `conditional_rules`
  - table-shape priors from validated `SourceIR` table header signatures chained through validated `IntentIR`
- the latest live four-document AMBA prior-memory run currently harvests `16` actor-taxonomy priors, `5` semantic phrase priors, `4` semantic modality-reliability priors, `266` temporal phrase priors, and `99` table-shape priors from the AXI/APB/AHB/AXI-Stream `IntentIR` artifacts
- the learning plane now also rejects bogus payload/event nouns like `control information` at both harvest and lookup time, so stale non-actor vocabulary cannot keep polluting actor-taxonomy priors once a local extraction bug is fixed
- active-drive actor extraction now also trims relative clauses before choosing the subject actor and rejects descriptive phrases such as `mixture of`, so prose like `An interconnect which connects to components with a mixture of chunking support can drive ARCHUNKEN` keeps `interconnect` as the actor instead of learning or preserving a fake actor
- coordinated active drive/read objects are graph-grounded too: `can drive ARCHUNKEN and RCHUNKV` and `samples ARCHUNKEN and RCHUNKV` now recover the same head actor for both object signals, and the tracked `relative_clause_actor_noise_negative` fixture locks that behavior through `SemanticIR` / `IntentIR` with zero connectivity conflicts
- that change came from the first unseen-protocol stress run too: AXI-Stream now converges in `2` pipeline iterations with full Ollama VLM + NLP Level 3 enabled, validates at `90/100 EXCELLENT`, and now carries complete declared-signal direction and width coverage (`22/22`) after parity-check width recovery; shared infrastructure signals like `ACLK` / `ARESETN` now surface through first-class `infrastructure_signals`, `infrastructure_signal_connectivity`, and a dedicated system-contract note rather than an ordinary missing-producer warning, six same-cycle temporal rules now carry explicit `0`-cycle windows, and the latest interface-grouping truthfulness pass removed the last carried residual decision by filtering width/table metadata out of heuristic grouping and letting explicit interfaces subsume smaller fragments
- the canonical polarity surface is now live across the refreshed AMBA projection too: AXI, APB, AHB, and AXI-Stream each currently report `with_resolved_polarity: 1`; explicit asserted-when-level prose like `CS_N is asserted when LOW`, collective active-level prose like `CS_N and WE_N are active LOW signals`, and safe mixed clause-local prose like `CS_N is active LOW and ENABLE is active HIGH` can now recover non-reset control polarity without guessing from suffixes
- the first prior-consumption slices are now real too:
  - cross-document actor-taxonomy memory can safely recover directions for local actor labels like `Producer` / `Consumer`, and it can now also recover structural KG edges from width-only section-guided headings like `Issuer signals`
  - cross-document semantic phrase memory can safely recover non-hardcoded local role phrases like `XACK can receive the transfer`
  - cross-document temporal phrase memory can safely recover cycle windows for local timing phrases like `PREADY must be asserted one beat later` when the built-in parser cannot
  - cross-document semantic modality-reliability memory can now advisory-adjust local semantic arbitration when the current PDF already contains competing locally grounded role candidates, but it still cannot create a role without those local candidates and it leaves conflict/arbitration state explicit
  - all four stay bounded: prior memory widens local interpretation, but it cannot invent any actor, signal, relation, semantic fact, or timing rule that is not grounded in the current document
- the KG benchmark harness can now also patch staged fixture inputs at `SourceIR` and `EvidenceIR`, which lets tracked fixtures model richer protocol-semantics cases like contested handshake-role evidence without needing an external PDF corpus for every regression
- the KG benchmark harness can now also patch a local fixture-owned `CorpusMemory`, so tracked regressions can prove prior-guided extraction improvements on unseen local phrases without depending on a shared mutable prior file
- `specforge kg-bench` can now also assert canonical semantic candidate and arbitration state directly, so fixtures can lock whether a signal meaning is decisively grounded or still honestly contested instead of inferring that only from side effects like blocked fallbacks or validator findings
- `specforge kg-bench` can now also patch `SourceIR` visual assets and assert persisted validation metric values directly at the evidence, semantic, and intent stages, which lets tracked fixtures lock cross-modality grounding behavior and VLM-note-derived semantics instead of only checking canonical structure or finding ids
- the tracked KG fixture set now also includes a multimodal negative case where table evidence and visual-caption evidence disagree on the same signal role; that fixture locks the honest behavior that visual grounding remains visible while cross-modality resolution stays absent and arbitration remains contested
- the tracked KG fixture set now also includes a stage-patched VLM timing-note gold case where signal meaning is grounded by a timing-diagram note itself, and the evidence-stage validator explicitly proves that the hint came from `vlm_timing_diagram_extraction` rather than caption text
- the tracked KG fixture set now also includes the negative twin of that case: a timing-diagram note can still yield a real timing extraction while carrying zero semantic-role hints when it only describes waveform motion around a handshake-shaped signal name
- the tracked KG fixture set now also tightens the VLM timing spurious-annotation negative case: compact and bracketed sample labels like `D0`, `A1`, `DATA0`, `0xAA`, `D[0]`, and `A[1]` stay out of timing constraints just like `T0`, `Addr 1`, and `Cycle 2`
- the tracked KG fixture set now also includes a waveform-motion VLM timing negative case: motion descriptors and transition spellings like `rising`, `stable`, `falling`, `UNCHANGED`, `RISING_EDGE`, `LOW_TO_HIGH`, `POS_EDGE`, `risingedge`, and `LOW2HIGH` do not become symbolic signal values, while a concrete `HIGH` sample still becomes typed temporal evidence
- the tracked KG fixture set now also includes a motion-only VLM timing annotation negative case: prose annotations like `XREQ rises, remains stable, then falls` do not become timing constraints unless they carry real timing/constraint indicators, while concrete `signals[].values[]` samples still become typed temporal evidence
- the tracked KG fixture set now also includes a VLM state-machine label-noise negative case: clean identifier states and transitions survive, while prose labels like `IDLE state` and `ACCESS phase` do not become canonical FSM names
- the tracked KG fixture set now also includes a VLM state-machine undeclared-transition negative case: identifier-shaped endpoints like `DONE` and `RESET` must be declared as accepted states in the same VLM observation before they can become transition endpoints
- the tracked KG fixture set now also includes a VLM state-machine duplicate-initial gold case: duplicate `IDLE` state labels collapse into one canonical state while preserving a later `is_initial: true` marker
- the tracked KG fixture set now also includes a VLM state-machine multiple-initial negative case: two initial markers keep the extracted graph visible but trigger semantic and intent validation warnings
- the tracked KG fixture set now also includes a VLM state-machine missing-initial negative case: states and transitions remain visible, but zero initial markers trigger the same validation warnings
- the tracked KG fixture set now also includes a visual-source conflict case where a caption and a VLM timing note disagree about the same signal’s role; that fixture locks that visual grounding stays visible while same-asset visual consensus remains honestly absent
- the tracked KG fixture set now also includes active-low VLM timing polarity cases plus non-reset control polarity cases: a reset reported as both `asserted` and `LOW`, or as both `deasserted` and `HIGH`, in a timing diagram becomes typed temporal evidence without creating a false temporal conflict because assertion values stay polarity-relative, mixed clause-local control prose can recover active-low and active-high controls, detached mixed-polarity prose stays unresolved instead of borrowing an implicit subject, and prose/table polarity disagreement stays explicit as a carried conflict
- the tracked KG fixture set now also includes a prior-guided temporal gold/negative pair: the unseen local phrase `PREADY must be asserted one beat later` stays unbounded without prior memory and gains a one-cycle `cycle_window` only when a matching learned temporal prior is staged into the fixture
- the tracked KG fixture set now also includes a prior-guided semantic gold/negative pair: the unseen local phrase `XACK can receive the transfer` stays semantically unresolved without prior memory and gains ready-like semantic recovery only when a matching learned semantic prior is staged into the fixture
- the tracked KG fixture set now also includes a prior-guided actor-taxonomy gold/negative pair: width-only `Issuer signals` / `Acceptor signals` sections stay directionless and graph-empty without prior memory and gain structural actor-signal relations, actor ports, and canonical signal directions only when matching actor-taxonomy priors are staged into the fixture
- the tracked KG fixture set now also includes a prior-guided visual semantic gold/negative pair: the unseen local caption phrase `XACK can sink the transfer` stays semantically unresolved without prior memory and gains ready-like semantic recovery only when a matching visual-caption semantic prior is staged into the fixture
- the tracked KG fixture set now also includes a prior-guided visual-motif gold/negative pair: the same local `XREQ cycle trace` caption stays visually ambiguous without prior memory, but gains a prior-backed classification observation, normative visual role, and VLM/multimodal corroboration target only when a matching visual-motif prior is staged
- the tracked KG fixture set now also includes prior-guided table-shape gold/negative pairs:
  - a locally `unknown` `Name | Direction | Width` table stays inert without prior memory and gains signal-description recovery only when a matching learned table-shape prior is staged into the fixture
  - a locally `unknown` `Parameter | Min | Max | Unit` table stays inert without prior memory and gains timing-parameter recovery only when a matching learned table-shape prior is staged into the fixture
- the tracked KG fixture set now also includes a first AMBA-style gold fixture: a `Source`-column signal-description table plus one guarded constraint must recover driver-side actor ports, semantic request/accept meaning, and a typed handshake-completion temporal rule
- the tracked KG fixture set now also includes an AXI write-response timing gold fixture: a width-only `BVALID` / `BREADY` / `BRESP` channel table plus prose actor relations must recover graph-backed direction, valid/ready meaning, next-cycle response timing, and response stability across the handshake
- the tracked KG fixture set now also includes an AXI write-response ID stability gold fixture: width-only `BID` plus prose actor relations must recover subordinate-owned graph direction and actor-grounded transaction-ID stability across the controlling `BVALID` / `BREADY` handshake
- the tracked KG fixture set now also includes an AXI read-address timing gold fixture: a width-only `ARVALID` / `ARREADY` / `ARADDR` / `ARLEN` channel table plus prose actor relations must recover graph-backed direction, valid/ready meaning, next-cycle read-address ready timing, and read-address stability across the handshake
- the tracked KG fixture set now also includes an AXI read-address ID stability gold fixture: width-only `ARID` plus prose actor relations must recover manager-owned graph direction and actor-grounded transaction-ID stability across the controlling `ARVALID` / `ARREADY` handshake
- the tracked KG fixture set now also includes an AXI write-address ID stability gold fixture: width-only `AWID` plus prose actor relations must recover manager-owned graph direction and actor-grounded transaction-ID stability across the controlling `AWVALID` / `AWREADY` handshake
- the tracked KG fixture set now also includes an AXI write-address control sideband stability gold fixture: width-only `AWPROT`, `AWCACHE`, and `AWLOCK` plus prose actor relations must recover manager-owned graph direction and actor-grounded stability obligations across the controlling `AWVALID` / `AWREADY` handshake
- the tracked KG fixture set now also includes an AXI read-address control sideband stability gold fixture: width-only `ARPROT`, `ARCACHE`, and `ARLOCK` plus prose actor relations must recover manager-owned graph direction and actor-grounded stability obligations across the controlling `ARVALID` / `ARREADY` handshake
- the tracked KG fixture set now also includes an AXI address QoS/region sideband stability gold fixture: width-only `AWQOS`, `AWREGION`, `ARQOS`, and `ARREGION` plus prose actor relations must recover manager-owned graph direction and actor-grounded stability obligations across their matching `AWVALID` / `AWREADY` and `ARVALID` / `ARREADY` handshakes
- the tracked KG fixture set now also includes an AXI address/response USER sideband stability gold fixture: width-only `AWUSER`, `ARUSER`, and `BUSER` plus prose actor relations must recover Manager-owned address USER sidebands and a Subordinate-owned write-response USER sideband across the matching channel handshakes
- the tracked KG fixture set now also includes an AXI read-address sideband stability gold fixture: width-only `ARSIZE` and `ARBURST` sideband fields plus prose actor relations must recover graph-backed direction and actor-grounded stability obligations across the controlling `ARVALID` / `ARREADY` handshake
- the tracked KG fixture set now also includes an AXI read-data timing gold fixture: a width-only `RVALID` / `RREADY` / `RDATA` / `RRESP` channel table plus prose actor relations must recover graph-backed direction, valid/ready meaning, next-cycle read-data valid timing, and read-data stability across the handshake
- the tracked KG fixture set now also includes an AXI read-data ID stability gold fixture: width-only `RID` plus prose actor relations must recover subordinate-owned graph direction and actor-grounded transaction-ID stability across the controlling `RVALID` / `RREADY` handshake
- the tracked KG fixture set now also includes an AXI read-data response stability gold fixture: width-only `RRESP` plus prose actor relations must recover subordinate-owned graph direction and actor-grounded response stability across the controlling `RVALID` / `RREADY` handshake
- the tracked KG fixture set now also includes an AXI read-data last stability gold fixture: width-only `RLAST` plus prose actor relations must recover subordinate-owned graph direction and actor-grounded stability across the controlling `RVALID` / `RREADY` handshake
- the tracked KG fixture set now also includes an AXI data USER sideband stability gold fixture: width-only `WUSER` and `RUSER` plus prose actor relations must recover Manager-owned write-data and Subordinate-owned read-data stability obligations across the matching data-channel handshakes
- the tracked KG fixture set now also includes an AXI write-data timing gold fixture: a width-only `WVALID` / `WREADY` / `WDATA` / `WSTRB` channel table plus prose actor relations must recover graph-backed direction, valid/ready meaning, next-cycle write-data ready timing, and write-data stability across the handshake
- the tracked KG fixture set now also includes an AXI write-data last stability gold fixture: width-only `WLAST` plus prose actor relations must recover manager-owned graph direction and actor-grounded stability across the controlling `WVALID` / `WREADY` handshake
- the tracked KG fixture set now also includes an AXI sideband stability gold fixture: width-only `ARLEN` and `WSTRB` sideband fields plus prose actor relations must recover graph-backed direction and actor-grounded stability obligations across their controlling `ARVALID` / `ARREADY` and `WVALID` / `WREADY` handshakes
- the tracked KG fixture set now also includes an AXI write-address sideband stability gold fixture: width-only `AWLEN`, `AWSIZE`, and `AWBURST` sideband fields plus prose actor relations must recover graph-backed direction and actor-grounded stability obligations across the controlling `AWVALID` / `AWREADY` handshake
- the tracked KG fixture set now also includes an APB write-control stability gold fixture: `PWRITE`, `PWDATA`, and `PSTRB` must remain actor-grounded stable requester outputs while `PSEL` and `PENABLE` are high and `PREADY` is low, without inventing handshake completion during that wait state
- the tracked KG fixture set now also includes an APB address/protection stability gold fixture: `PADDR` and `PPROT` must remain actor-grounded stable Requester outputs while `PSEL` and `PENABLE` are high and `PREADY` is low, again without inventing handshake completion during that wait state
- the tracked KG fixture set now also includes an APB response stability gold fixture: `PRDATA` and `PSLVERR` must remain actor-grounded stable completer outputs while `PSEL`, `PENABLE`, and `PREADY` are high, with `HandshakeComplete(PSEL, PREADY)` preserved for the completed access
- the tracked KG fixture set now also includes an AHB control stability gold fixture: `HADDR`, `HWRITE`, `HSIZE`, `HBURST`, and `HPROT` must remain actor-grounded stable manager outputs while `HREADY` is low and `HSEL` is high, without inventing handshake completion during that wait state
- the tracked KG fixture set now also includes an AHB transfer/lock stability gold fixture: `HTRANS` and `HMASTLOCK` must remain actor-grounded stable Manager outputs while `HREADY` is low and `HSEL` is high, without inventing handshake completion during that wait state
- the tracked KG fixture set now also includes an AHB exclusive/security stability gold fixture: `HEXCL` and `HNONSEC` must remain actor-grounded stable Manager outputs and `HEXOKAY` must remain an actor-grounded stable Subordinate output while `HREADY` is low and `HSEL` is high, without inventing handshake completion during that wait state
- the tracked KG fixture set now also includes an AHB response stability gold fixture: `HRDATA` and `HRESP` must remain actor-grounded stable subordinate outputs while `HREADY` is low and `HSEL` is high, without inventing handshake completion during that wait state
- the tracked KG fixture set now also includes an AHB write-data stability gold fixture: `HWDATA` must remain an actor-grounded stable manager output while `HREADY` is low, `HSEL` is high, and `HWRITE` is high, without inventing handshake completion during that wait state
- the tracked KG fixture set now also includes a bogus-actor-attribution negative fixture: `Clock` / `Reset` infrastructure rows in an AMBA-style `Source` column must not become protocol actors, while the real `Requester` / `Subordinate` rows still recover actor ports and table-grounded semantic roles
- the tracked KG fixture set now also includes a relative-clause actor-noise negative fixture: AXI-style chunking prose must recover `interconnect` as the producer and `Manager` as the consumer for both `ARCHUNKEN` and `RCHUNKV`, while keeping `mixture of` out of the graph and preserving zero connectivity conflicts
- the tracked KG fixture set now also includes a clock/reset protocol-scope negative fixture: protocol-PDF text may define the RTL/VIP-visible `ACLK` / `ARESETN` contract while explicitly leaving physical clock/reset tree construction to the integrating SoC team, without creating concrete topology records or actor ports
- the tracked KG fixture set now also includes a clock/reset generic-advice negative fixture: generic glitch-avoidance, reset-tree, synchronizer, and async-assert/sync-release guidance preserves first-class `ACLK` / `ARESETN` infrastructure signals without creating concrete topology records or ordinary actor ports
- the tracked KG fixture set now also includes a table-misclassification negative fixture: a misclassified `Bits | Name | Description` field table must not synthesize fake top-level signals or semantic roles from field names like `REQ` / `ACK`
- table-based KG relation extraction is now stricter too: `Source` / `Driver` columns yield `Drives`, `Destination` columns yield `Reads`, and direction/infrastructure placeholders like `input`, `Clock`, and `Reset` are filtered instead of becoming fake actors
- prose-based KG relation extraction is stricter too: active-drive subject extraction stays anchored to the current sentence head and strips relative clauses before actor selection, preventing descriptive phrases like `mixture of` from becoming producers while preserving real unresolved producer ambiguity for validation
- table-driven top-level signal synthesis is now stricter too: field-like `Bits | Name | Description` layouts and `... signal fields` captions are filtered before they can generate fake signals, widths, semantic hints, or system-contract facts
- `SemanticIR` and `IntentIR` now preserve the structural KG downstream via `actor_signal_relations`, `actor_ports`, and `signal_connectivity`, while keeping flat `direction_hint` fields only as a compatibility surface
- `kg-bench` can now assert graph-backed direction coverage by signal name directly, so benchmark fixtures no longer need to overload flat `signal_directions_include` when they mean canonical actor-port coverage
- `.fsm` explicit-module/top-composition lowering now has bounded graph/topology-first direction consumers too: matching `IntentIR.actor_ports` can recover child-module port directions when flat module-local hints lag, standalone direct roots can select one target actor from graph-backed output-target ownership while ignoring external readers/drivers that merely share those signals, explicit direct control reads can then recover target-actor input directions for local inventory signals, repeated actor-port direction/width conflicts remain sticky and blocked, standalone sequential DT roots have regression coverage for graph-backed clock/reset system-contract directions, and explicit top links can recover width-only top boundary port directions without inventing `.fsm` port roles
- width-only synthesized signal declarations like `Signal AWVALID is width 1.` now survive into `SemanticIR` / `IntentIR`, so AXI-style `Name | Width | Description` tables can create canonical signal records even when direction comes later from actor-relative graph evidence
- `SemanticIR` and `IntentIR` now also carry `interface_signal_conflicts` so conflicting direction/width evidence for the same signal stays explicit instead of only collapsing the canonical hint to `None`; the `SemanticIR` accumulator keeps that collapse sticky so later duplicate declarations cannot resurrect a poisoned canonical hint
- `SemanticIR` and `IntentIR` now also carry `signal_connectivity_conflicts` so unresolved multi-producer structural ambiguity stays explicit in the canonical layers instead of hiding inside raw connectivity vectors
- `SemanticIR` and `IntentIR` now also carry an initial typed `temporal_rules` surface so clocked behavior is no longer represented only as free-form timing text and prose constraints
- that temporal surface now also recovers bounded `cycle_window` latency from prose/timing text like `within 2 cycles`, `next cycle`, `next tick`, and `next rising edge` instead of leaving every temporal rule unbounded
- when typed temporal rules still survive without actor-relative drive/sample grounding, validation now emits stage-specific `semantic_temporal_actor_grounding_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so actorless rules become explicit review/replay targets instead of quiet canonical drift
- when typed temporal rules still survive without explicit clock/edge grounding, validation now emits stage-specific `semantic_temporal_clock_grounding_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so clockless rules become explicit review/replay targets instead of quiet canonical drift
- when typed temporal rules still survive without explicit `cycle_window` bounds, validation now emits stage-specific `semantic_temporal_cycle_window_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so unbounded rules become explicit review/replay targets instead of passive drift
- ready/valid completion guards like `AWVALID is HIGH and AWREADY is HIGH` now also surface as typed `HandshakeComplete` temporal predicates instead of surviving only as two unrelated scalar guard clauses
- signal-description tables now contribute typed `signal_semantic_hints`, and prose plus alias-grounded prose descriptions can now contribute the same role hints too; `SemanticIR` / `IntentIR` carry those per-signal `semantic_tags`, so handshake completion can be recognized from request/accept meaning even when the signal spellings are not literally `*VALID*` / `*READY*`
- contested semantic arbitration now blocks literal handshake-name fallback, so a signal like `XVALID` no longer auto-becomes valid-like when the preserved evidence still disagrees about its meaning
- that blocked fallback is now surfaced explicitly too: `SemanticIR` emits a residual decision packet and validation reports the affected handshake-shaped signals instead of leaving the withheld promotion implicit
- typed handshake-role recovery is now stricter too: `SemanticIR` only lets observation-backed semantic consensus drive canonical handshake roles, and handshake-shaped signals with fallback-only provisional roles now block raw name fallback instead of quietly regaining handshake semantics through spelling alone
- visual captions and grounded VLM timing-diagram annotations can now contribute the same `signal_semantic_hints` too, with explicit visual-evidence provenance, so meaning-based role inference has an initial multimodal path rather than depending only on tables and prose
- `SemanticIR` and `IntentIR` now also preserve per-signal `semantic_observations` with source kind and provenance, so canonical consumers no longer have to rely on lossy merged `semantic_tags` alone
- `SemanticIR` and `IntentIR` now also preserve explicit per-signal `semantic_candidates`, so competing role hypotheses remain inspectable in the canonical layers instead of being flattened into only a winner-or-none outcome
- `SemanticIR` and `IntentIR` now also preserve explicit per-signal `semantic_arbitration`, so the current lead role, runner-up, evidence margin, and decisive-vs-contested status stay inspectable without forcing an unsafe canonical winner when multiple role candidates remain
- `SemanticIR` and `IntentIR` now also resolve a canonical per-signal semantic role plus modality-aware `semantic_grounding_strength`, so handshake-role consumers can prefer provenance-backed role consensus and validation can distinguish single-source grounding, same-modality repetition, and true cross-modality reinforcement
- `SemanticIR` and `IntentIR` now also carry a canonical `semantic_consensus` summary per resolved signal role, including the supporting source kinds, supporting observation count, and strongest supporting automation confidence, and validation flags any resolved role still lacking that richer consensus profile
- fallback-only resolved semantic roles no longer stay as validator-only drift: `SemanticIR` now emits an explicit residual decision when a role is still resolved without observation-backed consensus, and `IntentIR` adds a matching assumption so provisional role meaning stays inspectable in the canonical layers
- those fallback-only roles now also route into the bounded replay loop: validation emits stage-specific `semantic_role_consensus_surface_rescan_guidance`, and `project-validation` turns it into a local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay plan rather than pretending provisional meaning should self-promote
- semantic-role hints from prose/visual text are now scrubbed of explicit signal identifiers before tag inference, so declarations like `Signal AWVALID is input width 1.` no longer create role consensus from the signal name alone; the surrounding descriptive language now has to carry the meaning
- prose statements and visual captions can now also ground different semantic roles for different signals from the same document region by extracting clause-local per-signal context windows instead of requiring the whole text to resolve to exactly one target
- when a text region mentions both a signal alias and the explicit signal name for the same signal, the explicit signal mention now outranks alias-grounding so the same sentence/caption does not double-count meaning from an alias that was no longer needed
- `SemanticIR` and `IntentIR` now also mark semantic role candidates and consensus summaries as `alias_dependent` when the recovered meaning still depends only on alias-grounded evidence, and validation reports that state explicitly instead of burying it inside source-kind lists
- those alias-dependent consensus states now also route into the bounded replay loop: validation emits stage-specific `semantic_alias_dependent_semantic_consensus_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so alias-grounded meaning must earn stronger non-alias corroboration instead of quietly resting as-is
- prior-guided final consensus now joins that same loop too: when learned modality-reliability priors helped a role converge, validation emits `semantic_prior_guided_semantic_consensus_surface_rescan_guidance`, and `project-validation` treats it as a bounded current-document replay target so stronger local corroboration can be sought instead of treating prior-guided carry-through as fully settled truth
- when canonical signal surfaces still survive without actor-relative graph direction coverage, validation now emits stage-specific `semantic_graph_direction_coverage_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so graph-uncovered signals become explicit review/replay targets instead of passive graph debt
- when carried graph-direction self-conflicts still survive with actor-aware same-actor direction disagreement, validation now emits stage-specific `semantic_graph_direction_conflict_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so graph-direction conflict ids become explicit review/replay targets instead of passive KG disagreement markers
- when protocol signal connectivity still survives with only a consumer side or only a producer side, validation now emits stage-specific `semantic_connectivity_missing_producer_surface_rescan_guidance` / `semantic_connectivity_missing_consumer_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so endpoint-gapped signals become explicit review/replay targets instead of passive structural debt; infrastructure clock/reset sourcing stays on the dedicated system-contract path rather than being forced through protocol replay
- when carried interface-signal conflicts still survive with unresolved direction/width disagreement, validation now emits stage-specific `semantic_interface_signal_conflict_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so interface-conflict ids become explicit review/replay targets instead of passive disagreement markers
- when carried typed temporal conflicts still survive with contradictory value obligations, validation now emits stage-specific `semantic_temporal_conflict_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so temporal-conflict ids become explicit review/replay targets instead of passive contradiction markers
- when carried signal-connectivity conflicts still survive with unresolved producer ambiguity, validation now emits stage-specific `semantic_signal_connectivity_conflict_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so conflict ids become explicit review/replay targets instead of passive ambiguity markers
- when carried signal-polarity conflicts still survive with unresolved active-level disagreement, validation now emits stage-specific `semantic_signal_polarity_conflict_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so polarity-conflict ids become explicit review/replay targets instead of passive polarity disagreement markers
- when `EvidenceIR` itself still carries unresolved active-level disagreement, validation now emits `evidence_signal_polarity_conflict_surface_rescan_guidance`, and `project-validation` turns that into a bounded local `nlp-enrich -> validate` replay lane so `polarity_conflict_*` ids become explicit evidence-local review/replay targets instead of passive upstream disagreement markers
- when `EvidenceIR` itself still carries unresolved semantic-role disagreement, validation now emits `evidence_signal_semantic_conflict_surface_rescan_guidance`, and `project-validation` turns that into a bounded local `nlp-enrich -> validate` replay lane so `semantic_conflict_*` ids become explicit evidence-local review/replay targets instead of passive upstream conflict markers
- when carried signal-semantic conflicts still survive with unresolved role disagreement, validation now emits stage-specific `semantic_signal_semantic_conflict_surface_rescan_guidance`, and `project-validation` turns that into the same local `EvidenceIR -> SemanticIR -> IntentIR? -> validate` NLP replay lane so semantic-conflict ids become explicit review/replay targets instead of passive semantic-role disagreement markers
- validation now also reports when typed `HandshakeComplete` temporal predicates depend on alias-dependent semantic role consensus, so alias-grounded transfer-progress semantics stay visible as weaker grounding instead of looking equivalent to directly grounded handshake meaning
- that weaker grounding now survives in the canonical artifacts too: `SemanticIR` emits an explicit `semantic_alias_dependent_handshake_completion` residual decision, and `IntentIR` adds a matching assumption so alias-grounded handshake progress stays inspectable even before validation runs
- contradictory semantic-role evidence for the same signal now surfaces explicitly as `signal_semantic_conflicts` in `EvidenceIR` and is reported by validation instead of disappearing into an ambiguous dual-tag fallback
- `SemanticIR` and `IntentIR` now also carry those `signal_semantic_conflicts`, so canonical consumers and validation can still see unresolved role disagreement instead of letting it disappear after the evidence stage
- and it now reuses unique KG producers to emit actor-grounded drive predicates for value-timed rules, so temporal semantics can point back to who actually drives the signal
- stable/hold rules now also keep that actor responsibility via actor-grounded stability predicates instead of flattening every producer obligation into a signal-only invariant
- compound `when/if` guards like `HREADY is LOW and HSEL is HIGH` now survive into the typed temporal layer as multiple antecedent predicates instead of being flattened into a single partial condition
- contradictory temporal value obligations under the same grounded context now surface as typed `temporal_conflicts` records instead of staying implicit in the rule set
- the convergent `EvidenceIR` loop now also mines `SignalDescription` tables, explicit asserted-when-level prose such as `asserted when LOW`, unambiguous collective prose such as `CS_N and WE_N are active LOW signals`, and safe clause-local mixed prose such as `CS_N is active LOW and ENABLE is active HIGH` for active-high/active-low facts, merges them conservatively before refining asserted/deasserted constraints, and persists explicit `signal_polarity_conflicts` records when prose/table evidence disagrees so validation can report the conflict instead of hiding it inside a neutral fallback
- `SemanticIR` and `IntentIR` now also carry those `signal_polarity_conflicts`, so contradictory active-level evidence remains visible in the canonical layers instead of disappearing after `EvidenceIR`
- learned negative-knowledge caution now covers polarity-conflict archetypes too, so recurring active-level disagreement shapes can be remembered as advisory review patterns without mutating current-document truth
- a pinned `subs/fsmgen` git submodule now exists as a local `.fsm` reference implementation for upcoming adapter work
- the `SourceIR` schema now reserves:
  - parser-backend identity
  - page-artifact manifests
  - visual-asset manifests
  - placeholder bindings for normalized sources
- PDF normalization is now explicitly treated as structured source capture with page images, figure/table assets, captions, and markdown as a convenience view rather than the sole system of record
- `specforge ingest <pdf>` now performs real Docling-backed structured normalization and materializes promoted markdown, page images, page metadata sidecars, visual assets, metadata JSON, backend raw JSON, and manifest files under `generated/source_ir/<document_key>/normalized`
- repeated PDF ingest for the same document key now atomically replaces that document's `normalized/` bundle instead of layering new files over old ones
- the first real `EvidenceIR` extraction pass now builds section anchors, evidence spans, visual evidence items, figure/caption links, and heuristic extracted statements from ready `SourceIR` artifacts
- the first real `SemanticIR` lifting pass now builds actors, interfaces, backend-neutral system/init records, first-class reset polarity/assertion/release/target semantics, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions from persisted `EvidenceIR` artifacts; boilerplate section filtering and markdown signal-table row parsing ensure the extraction is useful on real chip spec PDFs
- the first real `IntentIR` canonicalization pass now builds intent identity, actor responsibilities, carried interface/control/system/init surface, behaviors, constraints, assumptions, and residual decisions from persisted `SemanticIR` artifacts
- explicit staged IR modules now exist for:
  - `SourceIR`
  - `EvidenceIR`
  - `SemanticIR`
  - `IntentIR`
  - typed adapter lowering
- the first real `.fsm` adapter slices now materialize typed adapter artifacts, emit explicit standalone `?dt:name` text for honest canonical DT cases, emit structured `?fsm:name` text when the canonical state graph is explicit, emit explicit `?top:name` source documents when module/top composition facts are explicit, lower canonical symbol-definition sections, structured reset-role blocks, selector/test-node branches, and compound-update shorthand from the widened semantic model when those canonical shapes map directly into `.fsm`, keep reset polarity honest through the reset signal name because emitted `.fsm` text still carries only `sreset` / `asreset` plus the signal, keep unsupported selector predicates and other unsafe broader-root cases blocked with explicit residual decisions instead of fabricating target syntax, and intentionally keep compatibility-level `?mod:name` / `?module:name` spellings outside the current canonical root-kind model until a real backend-neutral direct-module distinction exists
- the next implementation milestone is semantic-truthfulness hardening: finish the remaining graph-first direction migration, broaden the new meaning-based role inference beyond signal-description tables plus initial prose/alias grounding into richer multimodal grounding, deepen the temporal-rule surface into richer temporal arbitration across modalities and actors, add KG-guided rescans and evidence arbitration, and now broaden the first cross-document learning plane so the extractor can accumulate reusable priors without contaminating per-document canonical truth; adapter expansion remains horizon work until the canonical four-layer pipeline is top-notch
- that truthfulness program now includes a tracked KG benchmark surface under `crates/specforge/test_data/kg_quality/`, with seed gold and negative fixtures for actor ports, name-only semantic noise rejection, multi-producer conflict surfacing, actor-boundary residual quality, contested handshake-name fallback blocking, alias-dependent handshake-completion caveats, both positive and negative direct VLM timing-note semantic grounding, same-asset visual semantic conflict surfacing, collective and mixed clause-local non-reset control polarity recovery plus detached mixed-polarity rejection, AMBA-style `Source`-column and `Destination`-column gold fixtures, representative APB `Requester` / `Completer`, setup/access timing, address-protection-stability, write-control stability, and response-stability gold fixtures, representative AXI width-only plus prose-direction, write-address next-cycle timing, write-response timing, address-response-user-sideband-stability, read-address timing, address-qos-region-sideband-stability, read-address-control-sideband-stability, read-address-sideband-stability, data-user-sideband-stability, read-data timing, read-data-last-stability, write-data timing, write-data-last-stability, sideband-stability, write-address-control-sideband-stability, and write-address-sideband-stability gold fixtures, representative AHB section-heading, wait-state timing, control-stability, transfer-lock-stability, exclusive-security-stability, response-stability, and write-data-stability gold fixtures, a bogus-actor-attribution negative fixture for `Source`-column infrastructure rows, explicit clock/reset topology gold plus generic-advice negative fixtures, a field-table misclassification negative fixture, spurious-timing negative fixtures proving low-value VLM annotation labels and motion-only VLM annotation prose do not become timing constraints, and prior-guided gold/negative pairs for actor-taxonomy direction recovery, bounded temporal recovery, semantic-role recovery, and visual-caption semantic recovery

## Working naming
- repository / project / CLI / crate name: `specforge`
- canonical output: `IntentIR`
- adapter targets:
  - `.fsm`
  - SystemVerilog
  - Verilog
  - VHDL

## Fast ramp-up order
1. `README.md`
2. `SESSION_BOOTSTRAP.md`
3. `INTENTIR_SPEC.md`
4. `ROADMAP.md`
5. `LIVE_ACHIEVEMENT_STATUS.md`
6. `VALIDATION_SNAPSHOT.md`
7. `RUST_CODEBASE_ANALYSIS.md`
8. `USER_GUIDE.md`
9. `DEVELOPMENT_NOTES.md`
10. `CHANGES.md`
11. `MEMORY.md`
12. `COMMIT.md`

## Documentation index
- `README.md`
  - single project entry point and navigation hub
- `docs/book/book.toml`
  - canonical mdBook configuration for user-facing documentation
- `docs/book/src/SUMMARY.md`
  - canonical book table of contents
- `SESSION_BOOTSTRAP.md`
  - exact fresh-session instruction for a new AI or LLM instance
- `INTENTIR_SPEC.md`
  - canonical product and stage specification for `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- `ROADMAP.md`
  - live roadmap for project objectives, sequencing, and remaining work
- `LIVE_ACHIEVEMENT_STATUS.md`
  - authoritative live progress snapshot using the project status vocabulary
- `VALIDATION_SNAPSHOT.md`
  - tracked projection of the latest persisted validation reports into a crash-safe markdown summary
- `docs/FSMGEN_FEEDBACK.md`
  - tracked SPECFORGE feedback for FSMGEN about features and orientation that would make `.fsm` adapter validation stronger
- `RUST_CODEBASE_ANALYSIS.md`
  - live deep-dive analysis of the Rust codebase and its architecture
- `USER_GUIDE.md`
  - end-user oriented guide to how the tool is expected to work
- `DEVELOPMENT_NOTES.md`
  - engineering rationale, design choices, and implementation context
- `CHANGES.md`
  - full detailed summary of the current set of changes
- `MEMORY.md`
  - compact but actionable continuity record for crash/session-loss recovery
- `COMMIT.md`
  - exact commit workflow and commit-time reporting requirements
- `.gitmodules`
- `.github/workflows/ci.yml`
  - git submodule manifest for pinned local reference dependencies

## Project file and directory map
### Current workflow and documentation paths
- `README.md`
- `docs/book/book.toml`
- `docs/book/src/SUMMARY.md`
- `SESSION_BOOTSTRAP.md`
- `INTENTIR_SPEC.md`
- `ROADMAP.md`
- `LIVE_ACHIEVEMENT_STATUS.md`
- `VALIDATION_SNAPSHOT.md`
- `RUST_CODEBASE_ANALYSIS.md`
- `USER_GUIDE.md`
- `DEVELOPMENT_NOTES.md`
- `CHANGES.md`
- `MEMORY.md`
- `docs/FSMGEN_FEEDBACK.md`
- `COMMIT.md`
- `corpus_kb/README.md`
- `corpus_kb/SCHEMA.md`
- `corpus_kb/benchmarks/README.md`
- `corpus_kb/benchmarks/kg-fixtures.md`
- `corpus_kb/failures/README.md`
- `corpus_kb/failures/validation-findings.md`
- `.gitignore`
- `.gitmodules`
- `.github/workflows/ci.yml`
  - GitHub Actions CI workflow for Rust formatting, linting, rustdoc, test validation, and docs validation; temporarily manual-only via `workflow_dispatch` to conserve hosted Actions minutes
- `scripts/run_ci.sh`
  - canonical local/hosted Rust + docs CI runner used both on developer machines and inside GitHub Actions
- `scripts/run_docs_ci.sh`
  - canonical local mdBook build runner used by the main CI script

### Local-only workflow paths
- `git_message_brief.txt`
  - short commit-message file used by the commit workflow
  - must remain untracked
- `questions_keep_untracked.txt`
  - local user backlog/questions scratch file
  - must remain untracked
- `trace.log`
  - optional trace/log output
  - must remain untracked
- `target/`
  - Rust build output
  - must remain untracked

### Reference adapter/tooling paths
- `subs/fsmgen/`
  - pinned local checkout of `fsmgen`, used as read-only contextual reference while building the first `.fsm` adapter
  - currently refreshed to the upstream baseline that includes FSMGEN's own live mdBook at `subs/fsmgen/docs/book/`
  - use its code, tests, strict-mode/support-accounting direction, and book as adapter reference material; do not modify it from this repository
  - if upstream misbehavior is discovered, track it locally as a bug report instead of patching the submodule

### Current Rust implementation paths
- `Cargo.toml`
  - root Rust workspace manifest
- `Cargo.lock`
  - dependency lockfile
- `crates/specforge/Cargo.toml`
  - active CLI crate manifest
- `crates/specforge/src/main.rs`
  - binary entrypoint
- `crates/specforge/src/lib.rs`
  - top-level command dispatch
- `crates/specforge/src/cli.rs`
  - clap-based CLI model
- `crates/specforge/src/error.rs`
  - typed error boundary
- `crates/specforge/src/test_support.rs`
  - shared test synchronization utilities for env-var-mutating CLI tests
- `crates/specforge/src/commands/mod.rs`
  - command module namespace
- `crates/specforge/src/commands/inspect.rs`
  - source/path inspection command
- `crates/specforge/src/commands/doctor.rs`
  - local runtime readiness command for Docling, Ollama, and LM Studio
- `crates/specforge/src/commands/converge.rs`
  - fixed-point whole-pipeline orchestration command
- `crates/specforge/src/commands/ingest.rs`
  - `SourceIR` preview/materialization command
- `crates/specforge/src/commands/evidence.rs`
  - `EvidenceIR` preview/materialization command
- `crates/specforge/src/commands/semantic.rs`
  - `SemanticIR` preview/materialization command
- `crates/specforge/src/commands/intent.rs`
  - `IntentIR` preview/materialization command
- `crates/specforge/src/commands/adapt.rs`
  - `.fsm` adapter preview/materialization command for the current honest DT/FSM/top lowering slices
- `crates/specforge/src/commands/enrich.rs`
  - VLM-backed `SourceIR` visual enrichment command
- `crates/specforge/src/commands/validate.rs`
  - stage-aware validation and report back-annotation command
- `crates/specforge/src/commands/project_validation.rs`
  - validation snapshot projection command for tracked live docs
- `crates/specforge/src/commands/rescan_plan.rs`
  - schema-v2 targeted rescan plan dry-run/execution command
- `crates/specforge/src/commands/kg_bench.rs`
  - tracked KG-quality benchmark command for gold and negative fixture evaluation
- `crates/specforge/src/commands/learn_priors.rs`
  - typed local `CorpusMemory` prior harvesting command
- `crates/specforge/src/commands/corpus_kb.rs`
  - tracked corpus knowledge-base refresh command for `R15g` pages
- `crates/specforge/src/commands/nlp_enrich.rs`
  - LLM-backed NLP Level 3 evidence enrichment command
- `crates/specforge/src/ir/mod.rs`
  - staged IR namespace and stage identifiers
- `crates/specforge/src/ir/source.rs`
  - `SourceIR` types, parser-backend selection, page/visual artifact manifests, and ingest-side residual decisions
- `crates/specforge/src/ir/source/docling_backend.rs`
  - Docling backend orchestration and the embedded Python helper that materializes structured PDF artifacts for `SourceIR`
- `crates/specforge/src/ir/evidence.rs`
  - first real multimodal `EvidenceIR` builder for text spans, captions, figure/table references, visual evidence, and extracted statements
- `crates/specforge/src/ir/semantic.rs`
  - first real `SemanticIR` builder for actors, interfaces, typed signal records, backend-neutral control fragments, explicit module/top composition facts, phases, invariants, contracts, gates, abstractions, decomposition candidates, and residual decisions
- `crates/specforge/src/ir/intent.rs`
  - first real `IntentIR` builder for canonical intent identity, actor responsibilities, carried interface inventory, carried backend-neutral control fragments, carried explicit module/top composition facts, behaviors, constraints, assumptions, and residual decisions
- `crates/specforge/src/ir/adapters.rs`
  - typed adapter artifacts, honest standalone/structured/top-root `.fsm` lowering logic, renderability analysis, and adapter-side residual decisions
- `crates/specforge/src/ir/prior_memory.rs`
  - advisory cross-document `CorpusMemory` prior schema and lookup helpers

### Planned future implementation paths
- `fixtures/`
  - sample specifications, PDFs, markdown conversions, and expected IR snapshots
- `crates/specforge/test_data/kg_quality/`
  - tracked KG-quality fixture set for graph truthfulness, false-positive control, conflict surfacing, and residual-quality checks
- `examples/`
  - example invocations and example stage outputs
- `generated/`
  - local generated IR artifacts; git-ignored by default

## Quick start
```bash
cargo test
./scripts/run_ci.sh
cargo run -p specforge -- --help
cargo run -p specforge -- inspect README.md
cargo run -p specforge -- converge README.md --target fsm
cargo run -p specforge -- ingest README.md --dry-run
cargo run -p specforge -- evidence generated/source_ir/readme/source_ir.json --dry-run
cargo run -p specforge -- semantic generated/evidence_ir/readme/evidence_ir.json --dry-run
cargo run -p specforge -- intent generated/semantic_ir/readme/semantic_ir.json --dry-run
cargo run -p specforge -- adapt generated/intent_ir/readme/intent_ir.json --target fsm --dry-run
cargo run -p specforge -- project-validation generated/intent_ir/readme/intent_ir.json
cargo run -p specforge -- rescan-plan
cargo run -p specforge -- kg-bench
cargo run -p specforge -- clean
cargo run -p specforge -- learn-priors generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/intent_ir.json generated/intent_ir/ihi0024_d_2021_04_amba_apb_protocol_specification/intent_ir.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/intent_ir.json
cargo run -p specforge -- corpus-kb generated/intent_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/validation_report.json generated/intent_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/validation_report.json generated/intent_ir/ihi0033_c_2021_09_amba_5_ahb_protocol_specification/validation_report.json generated/intent_ir/ihi0051_b_2021_04_amba_axi_stream_protocol_specification/validation_report.json
cargo run -p specforge -- corpus-kb --kg-fixtures-root crates/specforge/test_data/kg_quality
```
- `specforge ingest <source> --dry-run` prints computed `SourceIR` JSON without writing artifacts
- `specforge ingest <source>` materializes `generated/source_ir/<document_key>/source_ir.json`
- `specforge evidence <source-ir> --dry-run` prints computed `EvidenceIR` JSON without writing artifacts
- `specforge evidence <source-ir>` materializes `generated/evidence_ir/<document_key>/evidence_ir.json`
- `specforge semantic <evidence-ir> --dry-run` prints computed `SemanticIR` JSON without writing artifacts
- `specforge semantic <evidence-ir>` materializes `generated/semantic_ir/<document_key>/semantic_ir.json`
- `specforge intent <semantic-ir> --dry-run` prints computed `IntentIR` JSON without writing artifacts
- `specforge intent <semantic-ir>` materializes `generated/intent_ir/<document_key>/intent_ir.json`
- `specforge adapt <intent-ir> --target fsm --dry-run` prints computed adapter JSON without writing artifacts
- `specforge adapt <intent-ir> --target fsm` materializes `generated/adapters/fsm/<document_key>/adapter.json` and writes an emitted `.fsm` file when the canonical interface/control/system/init/state surface or explicit module/top composition surface is explicit enough for honest standalone DT, structured FSM, or first-slice `?top:name` lowering
- `specforge converge <source> --target fsm` materializes the loop-backed pipeline entrypoint, defaults to full Ollama VLM + NLP Level 3 enrichment, and stops when `SourceIR`/`EvidenceIR`/`SemanticIR`/`IntentIR`/adapter facts stop changing; add `--rescan-plan generated/validation/rescan_plan.json` to dry-run the targeted validation queue after stability, and add `--execute-rescan-plan` only when you intentionally want whitelisted replay hints executed
- `specforge project-validation <artifact>...` validates the passed artifacts, persists their latest reports, refreshes `VALIDATION_SNAPSHOT.md`, and updates the managed validation projection block in `LIVE_ACHIEVEMENT_STATUS.md`
- `specforge project-validation` also materializes `generated/validation/rescan_plan.json`, a local-only replay-oriented target list with typed inputs and structured command hints; visual-motif corroboration entries include an explicit local VLM `enrich_source_ir` hint before `EvidenceIR` rebuild/validation, and `--rescan-vlm-provider auto-local|ollama|lmstudio|skip` plus optional `--rescan-vlm-model <model>` controls the generated local provider hint without executing rescans automatically
- `specforge rescan-plan [--execute]` consumes that local plan; without `--execute` it only prints pending work, and with `--execute` it dispatches only whitelisted local enrichment/stage rebuild/validate hints from structured args rather than shell text, then records whether validation changed plus the before/after validation delta and conservative arbitration verdict; `--document-key <key>` scopes multi-document queues, and the same engine is available through `converge --rescan-plan <plan>` after the fixed-point loop stabilizes
- `specforge kg-bench` runs the tracked fixture set under `crates/specforge/test_data/kg_quality/` and fails if any gold/negative KG expectation drifts
- `specforge clean [--execute]` reclaims local generated artifacts; by default it dry-runs `generated/source_ir/*/normalized`, `--scope document [--document-key <key>]` removes full per-document generated stage trees, and `--scope all-generated` sweeps the full local `generated/` root when you intentionally want to rebuild everything from scratch
- `specforge learn-priors <intent-ir>...` builds a local `CorpusMemory` JSON file from validated `IntentIR` artifacts, scoped to reusable extraction priors rather than document facts; by default it writes `generated/prior_memory/corpus_memory.json`
- `specforge corpus-kb [validation-report]... [--kg-fixtures-root <fixture-root>]` refreshes tracked `corpus_kb/` synthesis pages from reviewable validation reports, KG fixture outcomes, fixture-family summaries, dedicated family pages for semantic/truthfulness, typed-prior-memory, table, visual, state-machine, timing, infrastructure, and protocol patterns, plus review-only prior-candidate projections with family-level gate matrices and a JSON readiness manifest; it only replaces managed blocks and preserves human-authored notes, so the corpus KB remains guidance and synthesis rather than canonical truth promotion
- PDF execute-mode ingest now prefers `SPECFORGE_DOCLING_PYTHON`, then repo-local `.venv-docling`, then versioned Python probes such as `python3.11`; run `bash scripts/bootstrap_docling.sh` from the repository root when you want the stable repo-local path

## Planned product shape
- stage 0: build `SourceIR` from raw sources, normalized text views, structured page artifacts, and extracted visual assets
- stage 1: build `EvidenceIR` from normalized text, section anchors, evidence spans, figure/caption links, visual evidence, and statement extraction
- stage 2: build `SemanticIR` from actors, interfaces, backend-neutral system/init records, phases, invariants, contracts, gates, assertions, abstractions, and decomposition candidates
- stage 3: build `IntentIR` as the canonical backend-independent intent model, including explicit interface inventory, backend-neutral guarded/action fragments, and backend-neutral system/init records when supported by the evidence
- stage 4: lower `IntentIR` through adapters such as `.fsm`, SystemVerilog, Verilog, and VHDL
- stage 5: validate adapters and back-annotate findings into the IR/documentation surface

## Key operating principles
- `IntentIR` is the canonical endpoint
- `.fsm` is an adapter target, not the core endpoint
- the pipeline is explicit: `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- the staged pipeline can now be driven through a fixed-point entrypoint that re-runs downstream stages until the persisted knowledge snapshot stabilizes
- automation-first, manual-last
- actor-first extraction
- typed IR over string-based generation
- deterministic steps where possible
- structured document parsing first, selective multimodal enrichment second
- LLM assistance where interpretation is required
- markdown is a lossy convenience view for PDFs, not the only normalized representation
- residual decision packets for irreducible ambiguity
- live documentation as critical continuity infrastructure

Read SESSION_BOOTSTRAP.md and start from there.
