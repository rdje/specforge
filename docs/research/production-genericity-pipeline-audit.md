# Production-genericity pipeline audit

Status: **discovery complete; open breach remains in the production surface**
Owner: `SPEC-TO-INTENT-ALIGNMENT.6d.ii.a`
Audit date: 2026-08-12
Audited revision: `b977a51ff24f966dcf6aca74ccf47d592a4fc452` plus the active `.6d.ii.a` replay publication

## Question and non-negotiable boundary

This audit answers: **How does SpecForge extract intent right now, and where can an input identity,
vendor, protocol, signal spelling, document-specific phrase, or reviewed-corpus fact steer the
production result?**

The required boundary is stronger than “the code does not mention the documents in today's corpus.”
The production core must accept any digital-chip specification/PDF and derive its decisions from
document evidence and universal typed semantics. Corpus-specific examples and exact assertions may
exist only in test/conformance code and fixtures. A finite forbidden-vocabulary file cannot prove
that property and is not the planned signoff mechanism.

## What this finding means for project feasibility

The breach does **not** show that a neutral specification-to-intent system is impossible. It shows
that the implementation admitted corpus-specific shortcuts instead of making neutrality a structural
property.

Neutrality is not absence of all prior concepts. An extractor necessarily has a universal digital
intent ontology: signals, fields, registers, states, events, participants, obligations, timing,
provenance, conflict, and uncertainty. The prohibited coupling is different: a production decision
must not depend on which document is being read, a vendor/protocol identity, a symbol's spelling, or
similarity to one tuned corpus phrase.

The feasible architecture therefore has three layers:

1. universal typed digital-intent semantics and document-structure grammars;
2. opaque names, values, relations, and evidence derived from the current input;
3. optional learned/model proposals with explicit provenance, uncertainty, and grounding back to that
   input, never selected by filename/title identity.

No tool can promise perfect automatic recall over every possible PDF: an input can be corrupt,
encrypted, image-only, contradictory, omit necessary context, or state intent ambiguously. The
signoff interpretation of “works on any digital-chip specification” is that the same neutral engine
accepts every input, promotes only justified intent, and emits explicit residuals for undecidable or
unsupported content. It must not silently fabricate an answer or require a new named extraction
branch.

Existing structural fingerprints, typed provenance, declared-symbol grounding, fusion, fidelity
gates, and residual decisions are evidence that this architecture is implementable inside SpecForge.
The correct remediation may temporarily reduce recall: remove specialized shortcuts first, then
recover recall through universal grammars and grounded models. Benchmark retention cannot override
the production boundary.

## Audit denominator and method

The denominator is the compiled production crate, not a selected word list:

- all 71 Rust source files below `crates/specforge/src/`;
- all canonical stages (`SourceIR`, `EvidenceIR`, `SemanticIR`, `IntentIR`, ISF adapter);
- optional VLM, text-LLM, NLI, prior-memory, rescan, validation, evaluation, and corpus-management
  paths that can alter or characterize production output;
- the embedded Python executed by the SourceIR PDF backend;
- the production/test module boundary, including examples that compile against public modules.

Each decision site was classified as one of:

1. **Structural/universal** — typed document structure or closed digital-design semantics;
2. **Input-derived** — an opaque name/value/phrase carried from the current document without a
   baked interpretation;
3. **Model-derived** — a proposal from a configured model, subject to typed grounding/validation;
4. **Learned-prior-derived** — corpus memory selected by a structural, identity-independent key;
5. **Hardcoded domain-specific** — a production decision or schema specialized to an identity,
   family, protocol, signal spelling, response name, or corpus phrase.

Class 5 is a release-blocking violation. Corpus-calibrated global thresholds and prompts that narrow
the task to protocols are also blocking latent bias even when they do not contain an exact document
key. Named examples in production comments do not directly execute, but violate the production
surface boundary and make a future leak easier to normalize; they must move to test/conformance
documentation.

## What the pipeline does now

```text
PDF / Markdown
  -> SourceIR: register source, convert PDF with Docling, preserve markdown/tables/pages/visuals
  -> optional VLM: classify/transcribe tables and extract timing/state observations
  -> EvidenceIR: parse blocks, synthesize typed facts, run convergent deterministic extractors,
                 optionally consult prior memory
  -> optional NLP/LLM: propose and ground constraints; optionally replace Pattern constraints
  -> SemanticIR: ground against declared signals, build interfaces/actors/connectivity/system
                 contracts/transactions/temporal rules, fuse modalities, residualize conflicts
  -> IntentIR: carry typed semantics, synthesize executable-intent relations and transactions
  -> adapter/ISF: lower supported typed intent; residualize unsupported or unsafe constructs
  -> validation/evaluation/controller: characterize fidelity, completeness, regressions, and next work
```

`converge` repeats SourceIR enrichment through adapter construction until its knowledge snapshot is
stable. With a live NLP provider, it then promotes grounded LLM constraints, rebuilds downstream
stages once, and measures NLI quality. Provider-free runs use deterministic extractors and any loaded
prior memory.

### Stage-by-stage audit

| Stage | Current extraction decisions | Neutrality verdict |
| --- | --- | --- |
| Source registration/materialization | Source kind, repository-local artifacts, Docling page/table/visual conversion, timing-table structural revalidation | Core lifecycle is structural. Embedded visual/table classifiers are mixed and include corpus-tuned phrases. |
| VLM enrichment | Caption-selected timing/state images; typed JSON prompts; table kind/grid proposals checked against headers | Mostly generic, but prompts narrow the domain to “chip protocol” and caption selection inherits the tuned SourceIR classifier. |
| Evidence assembly | Markdown blocks, spans, section anchors, references, table declarations, typed provenance | Broadly structural/input-derived. |
| Evidence deterministic extraction | Registers, fields, signal inventories, relations, constraints, polarity, semantic hints, timing, frames, states, actors, operations | Mixed. Several useful grammars are universal, but protocol-specific schema/extractors and exact signal/response spellings execute in production. |
| Prior-guided EvidenceIR | Table/visual/actor/semantic/temporal priors | Blocking: priors are selected by a named family inferred from document key/display name. Structural extraction profiles already demonstrate the correct direction. |
| Text LLM/NLP | Per-sentence JSON extraction, declared-signal grounding, entity typing, dedup, optional promotion | Grounding is useful; one production prompt embeds named signal examples and another frames every input as a protocol specification. |
| SemanticIR | Declared-signal gate; interfaces, actors, ports, connectivity, infrastructure, temporal rules/contracts; VLM fusion/fidelity; residuals | Mixed. Core grounding/fusion is generic. Signal-name substring inference and named-family prior routing are blocking. |
| IntentIR | Lossless carry-forward; actor/temporal relations; structural named-transaction recognition; residual decisions | Main synthesis is structural, but it publicly carries protocol-specific types/field names and production comments retain corpus identities. |
| ISF adapter | Typed lowering, syntax validation, conflict rejection, unsupported-record residuals | Lowering logic is mostly neutral. It imports/counts/residualizes protocol-specific schema and contains corpus-specific production commentary. |
| Validation/evaluation | Stage metrics, negative-knowledge lookup, extraction scores, completeness/category classification | Blocking where validation repeats named-family and signal-spelling decisions; evaluation schema exposes named protocol tasks. Category thresholds are corpus-calibrated constants presented as generic. |
| Corpus/trajectory utilities | Learn priors, structural clustering, KG pages, reviewed replay/controller | Structural clustering is neutral. Named family pages/routing and the reviewed-population snapshot are corpus-specific production code. |

## Confirmed production violations

### P0 — document identity selects protocol-family semantics

`ir/prior_memory.rs` defines `ProtocolFamily::{AmbaApb, AmbaAhb, AmbaAxi, AmbaGeneric,
Unknown}`. `ProtocolFamily::infer` searches `document_key` and `display_name` for family fragments.
That value scopes actor, semantic phrase, modality reliability, temporal, table, visual, and negative
knowledge lookups, including a named generic-family fallback.

The decision propagates through:

- `ir/evidence.rs::load_evidence_prior_guidance`;
- `ir/semantic.rs::load_semantic_prior_guidance`;
- `commands/learn_priors.rs` harvest/materialization;
- `commands/validate.rs` negative-knowledge and prior-accounting paths.

Renaming an identical PDF can therefore change the prior search scope. This is direct
identity-dependent extraction behavior. Replace it with document-derived structural fingerprints or
an opaque caller-supplied profile whose provenance is explicit; the core must never infer semantics
from a filename/title.

### P0 — protocol-specific EvidenceIR schema and extractors

`ir/evidence.rs` exposes `SwdOperation`, `SwdioDirection`, and `swd_operations` in the production
schema. The associated production code:

- gates serial-frame extraction on phrases and spellings including `serial wire`, `packet request`,
  `swdio`, `swclk`, and `shift-dr`;
- assigns phases through named fields and exact tokens, including `ACK`, request/data spellings, and
  named control bits;
- maps exact actor words to host/target drive direction;
- registers explicitly named state extractors and emits a named line-state-machine identity;
- recognizes exact response branches and fixed meanings for successful/OK, WAIT, and FAULT;
- manufactures protocol-prefixed record identifiers.

These records are then carried by `SemanticIR`, `IntentIR`, `converge`, validation, `eval.rs`,
`eval_extraction`, completeness, corpus clustering, and the adapter. The downstream copies are not
independent extraction errors, but they make the specialization part of the canonical public model.

The replacement must be generic `FrameField`, `OperationBranch`, `ParticipantDrive`, and
`ProtocolState` semantics whose names, response values, phase counts, and participant roles are
document-derived. A protocol-specific conformance fixture may assert that those generic records
represent a particular protocol.

### P0 — signal spelling changes semantic authority

`ir/semantic.rs::classify_handshake_signal_from_name` assigns handshake roles when a signal name
contains `valid` or `ready`; `commands/validate.rs::handshake_name_heuristic_role` repeats the rule.
`should_emit_interface_candidate` grants single-token interface authority for suffixes/substrings
such as active-low, reset, and clock naming conventions. `signal_stop_words` contains standards,
technology, interface, and architecture names and thereby changes which uppercase input tokens can
be signals. `ir/evidence.rs::is_abstract_transport_signal_token` likewise matches a fixed set of
signal spellings.

These are not closed digital semantics: they infer meaning from arbitrary identifiers. Roles must
come from table columns, prose, diagrams, explicit declarations, or a model proposal grounded back
to that evidence. Alpha-renaming must preserve the same graph and obligations modulo renamed
symbols.

### P0 — PDF classification contains corpus-calibrated phrases

The embedded Python in `ir/source/docling_backend.rs` classifies visuals and tables before
EvidenceIR. Its executable lists include operation/burst phrases and interface-role phrases derived
from particular protocol families. Its register-table exclusion contains a packet-layout vocabulary
derived from one storage protocol. Those branches decide which images reach the VLM and whether a
table becomes a register map, so they materially affect extraction.

Replace phrase collections with structural features (geometry, repeated clock lanes, transition
edges, header/value shape, range/access topology) plus an honest unknown result. Generic English
document-type grammar may be centralized and typed, but corpus idioms cannot be embedded in this
backend.

### P0 — named corpus code is compiled as product code

`commands/corpus_kb.rs` has a named protocol-family label, output page, prompts, and fixture-name
classifier. At the audited revision, the reviewed trajectory composer was also exported from
`ir/mod.rs` and exercised through a production example even though it authenticates exact reviewed
documents and results. This slice removes that second breach: the composer now lives below the
`#[cfg(test)]` `test_support` module, the public IR registration and production example are gone, and
its named assertions remain test-only. `commands/corpus_kb.rs` remains open remediation work.

Corpus organization, review gold, and exact trajectory snapshots are conformance/evaluation assets,
not product extraction. They must compile only under test/conformance targets and cannot be imported
by the production core.

### P1 — production model prompts carry unnecessary domain priors

`commands/nlp_enrich.rs::build_nlp_prompt` describes the agent as a protocol analyzer and gives
named real signal examples. `commands/enrich.rs` similarly frames diagrams/tables as protocol PDFs.
`ir/constraint_extract_llm.rs::extraction_prompt` is otherwise schema-oriented and uses neutral
placeholders, but its “wire/pin/field” definition currently conflates fields with signals and should
remain aligned with entity typing.

Prompts are executable extraction policy. They must describe the neutral typed output and use
synthetic placeholders generated per request, while injecting only document-derived grounding.

### P1 — corpus-calibrated thresholds are presented as universal classification

`ir/completeness.rs` uses fixed behavioral/connectivity/state/message-field thresholds selected from
the retained corpus. The predicates do not branch on a name, but the constants can encode sampling
bias and are used to classify arbitrary documents. They are acceptable only as an explicitly
versioned, uncertainty-bearing calibrated model with held-out evidence; they cannot be claimed as a
universal proof of document purpose.

The classification result must retain the measured domain/version, confidence, and unresolved path,
or derive its thresholds from a validated generic model. Extraction itself must not be suppressed by
the category decision.

### P1 — production comments and public names normalize specialization

Production comments and doc comments across `evidence.rs`, `intent.rs`, `semantic.rs`, `isf_ir.rs`,
`adapters.rs`, `completeness.rs`, `entity_typing.rs`, `protocol_graph.rs`, `source.rs`, and the embedded
backend describe fixes through named corpus examples. They do not all affect execution, but the owner
boundary permits those examples only in tests. Move the evidence to conformance tests/research notes
and describe production behavior in structural terms.

## Production-source disposition

This table accounts for the complete source tree while keeping the detailed findings above at the
decision sites.

| Source group | Files | Verdict |
| --- | --- | --- |
| Entrypoint/platform | `lib.rs`, `main.rs`, `cli.rs`, `error.rs`, `persisted_path.rs`, `project_data.rs` | Neutral infrastructure. |
| Test support | `test_support.rs`; test-only functions in `ir/mod.rs` | Test-only by design; keep outside production dependencies. |
| Thin stage/utility commands | `adapt`, `clean`, `doctor`, `evidence`, `ingest`, `inspect`, `intent`, `semantic`, `extract_conditions`, `extract_contracts`, `recover_register_bits`, `rescan_plan`, `signal_resolve` | No identity-specific extraction branch found; they inherit the IR behavior they invoke. |
| Model/quality commands | `audit_extraction`, `enrich`, `extract_constraints_llm`, `grits_consensus`, `nli_verify`, `nlp_enrich`, `project_validation` | Mixed: core transport/scoring is neutral; prompt framing/examples and inherited named schemas need correction. Corpus literals observed in these files are otherwise inside test modules. |
| Orchestration/evaluation commands | `converge`, `eval_extraction`, `kg_bench`, `validate` | Mixed/blocking through named schema, family inference, signal-name inference, or corpus test framework exposed as production. |
| Corpus commands | `corpus_cluster`, `learn_priors`, `corpus_kb` | Structural clustering is the neutral model; named family prior/KG routing is blocking. |
| SourceIR | `ir/source.rs`, `ir/source/docling_backend.rs` | Source/path lifecycle and structural timing authority are neutral; embedded classifiers are mixed/blocking. |
| EvidenceIR/extraction | `ir/evidence.rs`, `extractor.rs`, `extraction_filters.rs`, `entity_typing.rs`, `condition_extract.rs`, `constraint_extract_llm.rs`, `nlp_relation_extract.rs`, `normative_vocab.rs`, `register_bits.rs` | Mixed/blocking in EvidenceIR schema/extractors and name lists; framework, grounding primitives, and most parsers are reusable. Named production commentary must move. |
| Prior/reuse | `ir/prior_memory.rs`, `ir/corpus_cluster.rs` | Named family prior routing is blocking; structural fingerprint clustering is neutral. |
| SemanticIR | `ir/semantic.rs`, `ambiguity.rs`, `contract.rs`, `cve.rs`, `fidelity.rs`, `figure_region.rs`, `fusion.rs`, `nli_verify.rs`, `protocol_graph.rs`, `temporal_ltl.rs`, `waveform.rs` | Core typed fusion/verification is neutral; semantic name inference and named public commentary are blocking. Figure-region corpus paths are test-only. |
| IntentIR | `ir/intent.rs` | Structural synthesis is broadly neutral; protocol-specific carried schema/public names and named production commentary are blocking. |
| Adapter | `ir/adapters.rs`, `ir/isf_ir.rs` | Typed lowering is broadly neutral; carried named schema and named production commentary remain. |
| Completeness/evaluation | `ir/completeness.rs`, `eval.rs` | Corpus-calibrated model and named evaluation schema require separation/versioning; corpus examples after test boundaries are allowed. |
| Replay/trajectory | `ir/source_to_intent_eval.rs`, `source_to_intent_replay.rs`, `trajectory.rs`; `test_support/trajectory_snapshot.rs` | Engines are generic; reviewed snapshot composition is now test/conformance-only. |
| Module registry | `ir/mod.rs`, `commands/mod.rs` | Currently compiles corpus/evaluation specialization into production; must enforce the new dependency boundary. |

## Why a forbidden-vocabulary TSV is not signoff proof

A denylist can catch only spellings someone anticipated. It cannot detect:

- a new vendor, protocol, document, or signal name;
- fragments, aliases, abbreviations, translations, or obfuscations;
- a neutral-looking phrase copied from one corpus and used as an exact behavior gate;
- identity coupling through hashes, IDs, paths, numeric thresholds, or learned labels;
- model prompts and schema types that bias behavior without containing a banned token.

A generated leak census remains useful as a diagnostic and mutation fixture, but never as the
genericity authority.

## Required signoff architecture

The release proof must combine structural prevention with behavioral invariance:

1. **One-way module boundary.** Production core owns generic IR and registered universal grammars.
   Conformance/evaluation owns named fixtures, reviewed gold, exact snapshots, corpus reports, and
   compatibility migrations. Core cannot depend on conformance.
2. **Opaque input symbols and identity.** Document names, paths, titles, signal/field/state names, and
   values are carried as input-derived symbols. Extraction decision APIs cannot inspect document
   identity. Semantic roles require cited evidence, not symbol spelling.
3. **Registered grammar boundary.** Raw document text may be interpreted only through audited grammar
   modules that return typed, source-spanned proposals. An AST-aware doctrine rejects ad-hoc raw
   string equality/substring/regex decisions in production extractors and rejects forbidden module
   dependencies. Universal grammar evolution is reviewed as semantics, not as a vocabulary exception.
4. **Grounding and residuals.** Every promoted record cites source spans/tables/visuals or an explicitly
   provenance-bearing model/prior proposal; unresolved or contradictory evidence remains residual.
5. **Metamorphic invariance.** Alpha-renaming every input-defined symbol and adversarially changing
   filename/title/vendor-like tokens must leave the typed graph invariant modulo the renaming. A
   structure-preserving paraphrase suite checks that behavior is not pinned to one corpus sentence.
6. **Negative controls and held-out documents.** Mutation fixtures inject a raw literal decision,
   identity dependency, forbidden core-to-conformance import, schema specialization, and name-based
   semantic inference; every mutation must fail. Held-out digital-spec families measure generalization
   without tuning on the qualification set.
7. **Supplementary census.** A derived named-token/literal report covers comments, prompts, public
   identifiers, and suspicious constants. It assists review and enforces the owner's zero-named-example
   production-surface rule, but is explicitly not the semantic proof.

## Historical correction

`PDF-AGNOSTIC-EXTRACTION.4` previously closed genericity using a small seeded check focused on
`signal_stop_words`, excluding comments/tests/fixtures. This audit disproves that repository-wide
claim: executable identity, protocol, signal, and corpus phrase decisions remained elsewhere. The old
evidence is still useful for the specific list it checked, but it cannot support whole-production
neutrality and must be annotated as superseded by this audit and the `.6d.ii` remediation tree.

## Exit criteria

This audit closes only the discovery slice. The production-genericity parent remains open until:

- every P0/P1 item above is removed or relocated to conformance;
- the core compiles without corpus-specific public types/modules;
- the AST/dependency mutations fail closed;
- alpha-renaming, identity perturbation, paraphrase, negative-control, held-out, full replay, and full
  repository gates pass;
- code, roadmap, task tree, live docs, Knowledge Map, and mdBook all state the same verified boundary.
