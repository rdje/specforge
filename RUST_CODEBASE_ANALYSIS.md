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
  - `learn-priors`
  - `nlp-enrich`
- the canonical product boundary remains `IntentIR`, not `.fsm`
- the staged pipeline is operational through `SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`
- a whole-pipeline fixed-point entrypoint now exists via `specforge converge`, which reuses persisted artifacts, defaults to Ollama VLM + NLP Level 3, and stops when the cross-stage knowledge snapshot is stable
- `SourceIR` now captures structured Docling output, typed content elements, structured tables, visual assets, and document-profile metadata
- `EvidenceIR` now synthesizes typed declarations and records from tables, preserves typed NLP outputs, persists alias-learning state, extracts actor-signal relation triples from prose and signal-description tables, runs a monotone convergence loop so discovered enum facts and prose polarity can unlock additional signal constraints without hardcoded protocol-specific value lists, and now keeps polarity disagreement explicit through typed conflict records instead of only via a neutralized fallback
- `SemanticIR` now lifts that evidence into interfaces, explicit interface-signal conflict records for conflicting direction/width evidence, actor-relative port/connectivity records, explicit signal-connectivity conflict records for unresolved multi-producer ambiguity, system/reset/init records, control/state records, timing/register records, and filtered NLP constraints, with VLM observations merged into the semantic surface
- `IntentIR` now carries forward the canonical signal/control/system/state/register/timing surface plus the actor-relative KG needed for honest downstream lowering
- the current `.fsm` adapter slice is real and intentionally narrow: it can emit honest `?dt:name`, `?fsm:name`, and `?top:name` outputs when the canonical facts are explicit enough
- the enrichment, convergence, validation, benchmark, and first prior-learning toolchain is also real: `specforge enrich`, `specforge nlp-enrich`, `specforge converge`, `specforge validate`, `specforge kg-bench`, and `specforge learn-priors` are wired into the CLI and exercised by the workspace tests
- the local runtime boundary is now operationally stronger too: `specforge doctor` reports Docling readiness, the default Ollama loopback readiness, and LM Studio fallback readiness directly, repo-local `.venv-docling` auto-discovery is supported, and the backend now probes versioned Python candidates like `python3.11` before giving up on fresh ingest
- GitHub Actions CI is now part of the repo baseline and runs `cargo fmt --all --check` plus `cargo test --manifest-path Cargo.toml` on every `push` and `pull_request`, which keeps the hosted validation path aligned with the local Rust quality gate
- that CI path now has a single checked-in entrypoint at `scripts/run_ci.sh`, and the GitHub workflow calls that script directly so local and hosted Rust validation do not drift apart
- the remaining dominant gaps are semantic-truthfulness gaps: finishing the remaining graph-first consumers, deepening the temporal-rule layer into richer actor-relative and contradiction-aware clocked semantics, KG-guided rescans, evidence arbitration, benchmark-quality evaluation, and eventually adding a separate cross-document prior-learning plane that can improve extraction without leaking facts across documents; adapter expansion is now horizon work
- the workspace currently validates with `cargo test --manifest-path Cargo.toml`, with 199 passing tests


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
- resolved signal polarity now also survives directly on canonical `InterfaceSignalRecord`s and is visible in validator metrics as `with_resolved_polarity`; after the latest AHB infrastructure-interface fix, AXI/APB/AHB/AXI-Stream now all report `1`, so the remaining polarity work is broader non-reset control coverage rather than carry-through plumbing
- `specforge converge` now excludes downstream adapter residual work from `knowledge_fact_count`, so fewer adapter residual decisions do not falsely trip the monotone-knowledge guard
- `generated/` is now intentionally git-ignored and untracked, so local validation snapshots must be recorded in the live docs instead of relying on versioned artifacts
- latest local validation snapshot is now AXI `85/100 GOOD`, APB `90/100 EXCELLENT`, AHB `94/100 EXCELLENT`, and AXI-Stream `90/100 EXCELLENT`; this refresh replaced a stale optimistic snapshot, the current tracked four-artifact projection now uses APB `IHI0024_E`, AHB remains in the excellent lane, and the latest AXI truthfulness fixes removed the last blocked-handshake residual, the false `AWAKEUP` / `CRVALID` semantic-role conflicts, the false `ACLK` / `ARESETN` interface-direction conflicts, and the bogus `Tie-off`-driven `BROADCAST*` missing-consumer warning while leaving AXI as the main remaining live quality outlier
- APB `IHI0024_D` was re-run from the original PDF through full `specforge converge` with Ollama VLM + NLP Level 3 and converged in 2 passes
- AXI `IHI0022_L` was re-run from the original PDF through full `specforge converge` with Ollama VLM + NLP Level 3, converged in 2 passes, and recovered timing to reach 94/100 EXCELLENT
- `extract_alias_phrase()` now rejects markdown/table marker prefixes `-`, `|`, and `#`, closing the last small R12 cleanup in the NLP alias-learning loop
- the documented README staged flow was re-executed on `README.md` through `inspect -> ingest -> evidence -> semantic -> intent -> adapt --dry-run`, confirming the current entry path still runs end-to-end
- the roadmap now explicitly treats adapter expansion as horizon work; the next structural gaps are making the actor-relative graph primary, broadening the new table/prose/alias-grounded role inference into richer multimodal grounding, adding deeper explicit temporal semantics, and hardening KG quality/evaluation
- the current architecture is still intentionally document-local, which is correct for truthfulness, but the next strategic expansion after the current semantic-truthfulness work should be a separate cross-document learning plane that stores reusable extraction priors rather than cross-document facts
- the local test suite is now at `202/202` passing after the first prior-consumption slice landed
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
  - a representative AXI-style gold fixture that proves width-only channel tables plus prose drive/sample relations recover actor-relative direction, signal inventory, request/accept semantics, and typed handshake completion without any table direction column
  - a representative AXI-style timing gold fixture that proves width-only channel tables plus prose actor relations can also recover next-cycle timing, actor-grounded temporal predicates, and handshake completion together
  - a representative AHB-style gold fixture that proves `Manager signals` / `Subordinate signals` section context recovers per-signal direction and width correctly from staged `SourceIR`
  - a representative AHB-style timing gold fixture that proves the same section-heading and destination-column context can also recover wait-state timing, actor-grounded temporal predicates, bounded next-cycle latency, and multi-predicate guards together
  - the canonical parser now also accepts width-only synthesized declarations like `Signal AWVALID is width 1.`, which closes the specific AXI gap where table-grounded widths previously stopped at actor relations and never became real interface signal records
  - a bogus-actor-attribution negative fixture that proves `Clock` / `Reset` infrastructure rows in AMBA-style `Source` columns do not become protocol actors while the true requester/subordinate rows still survive canonically
  - a field-table misclassification negative fixture that proves a misclassified `Bits | Name | Description` table does not synthesize fake top-level signals or semantic roles from field names that merely look signal-like
  - a spurious-timing negative fixture that proves low-value VLM timing-diagram labels like `T0`, `Addr 1`, and `Cycle 2` still count as timing-diagram extraction at the evidence stage but do not survive into `TimingConstraintRecord` or `TemporalRuleRecord`
- the harness can now also assert canonical actor-signal relations directly, so tracked gold fixtures can lock `Drives` versus `Reads` truth instead of checking only actor-port projections or relation counts
- the harness can now also patch `SourceIR.document_sections` and assert per-signal canonical direction directly, which is important for protocol families like AHB where section-heading context still carries real directionality
- the harness now also asserts canonical semantic candidates and decisive-vs-contested semantic arbitration directly, which is a better `R15e` truthfulness check than inferring arbitration quality only from blocked fallback or validation side effects
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
- `crates/specforge/src/commands/nlp_enrich.rs`
- `crates/specforge/src/test_support.rs`
- `crates/specforge/src/ir/mod.rs`
- `crates/specforge/src/ir/source.rs`
- `crates/specforge/src/ir/source/docling_backend.rs`
- `crates/specforge/src/ir/evidence.rs`
- `crates/specforge/src/ir/semantic.rs`
- `crates/specforge/src/ir/intent.rs`
- `crates/specforge/src/ir/adapters.rs`
- `subs/fsmgen/`

### Rust-specific contents still absent
- no dedicated `specforge-source` crate
- no dedicated `specforge-evidence` crate
- no dedicated `specforge-semantic` crate
- no dedicated `specforge-intent` crate
- no dedicated `specforge-adapters` crate
- no dedicated validation crate
- no integration-test harness beyond crate-local unit tests
- no additional real builders beyond the current `SourceIR`/`EvidenceIR`/`SemanticIR`/`IntentIR` slices and the first `.fsm` adapter slice

### Immediate implication
- the codebase is no longer mostly scaffolding; the main open problem is semantic truthfulness across the four IR layers, especially graph primacy, temporal semantics, multimodal rescans, and evidence arbitration
- the practical risk is now split across two partial migrations:
  - `SemanticIR` and `IntentIR` still expose both graph-native actor-relative records and legacy flat `direction_hint` fields; validation/scoring is now graph-first, but some downstream compatibility and consumer paths still consult the flat hints directly
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
- `src/commands/kg_bench.rs`
  - tracked KG-quality benchmark harness over staged IR artifacts and persisted validation findings
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
- the actor-signal relation graph now survives into `SemanticIR` / `IntentIR`, but legacy interface records still flatten some downstream consumers onto actor-agnostic `direction_hint` values
- the workspace still emits five compiler warnings in normal `cargo test` / `cargo run` flows: five dead-code helpers across `ir/adapters.rs` and `ir/semantic.rs`
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
- keep the latest local APB/AHB/AXI 95/95/94 snapshot visible as follow-on work lands, and close the remaining AXI width/connectivity gaps from that improved baseline

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
- current test count: 99 (all passing)
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
  - polarity refinement from active-low / active-high prose
  - caption and figure-reference grounding into visual evidence
  - VLM observation injection (TimingDiagramExtraction, StateMachineExtraction from VisualAsset.note)
  - handshake-driven `SemanticIR` actor/interface/invariant extraction
  - structured-table signal direction+width extraction through EvidenceIR → SemanticIR
  - parametric-width handling through the IR pipeline
  - VLM timing diagram annotation → TimingConstraintRecord in SemanticIR
  - VLM state machine extraction → RegularStateRecord + StateTransitionRecord in SemanticIR
  - ambiguous visual-grounding residual decisions in `SemanticIR`
  - handshake-driven `IntentIR` identity/behavior/constraint/assumption construction
  - residual-decision preservation from `SemanticIR` into `IntentIR`
  - NLP Level 3 extraction, backannotation, and alias learning
  - markdown-marker alias rejection for Form 2 alias learning
  - actor-signal relation extraction from prose and table roles
  - AMBA `Source` / `Driver` / `Destination` signal-direction handling
  - `.fsm` adapter renderability (12 adapter cases)
  - `specforge validate` for all four IR stages
- next tests should cover:
  - richer APB and AXI end-to-end fixtures for relation-driven direction coverage
  - actor-relative direction modeling once it lands in `SemanticIR` / `IntentIR`
  - validation back-annotation persistence
  - wider `.fsm` renderability coverage and snapshot stability on protocol-heavy fixtures
  - future adapter targets beyond the current `.fsm` slice

## Validation completed in this session
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
