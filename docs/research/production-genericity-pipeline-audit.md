# Production-genericity pipeline audit

Status: **discovery and proof-architecture freeze complete; production dependency, proof, rule, structural, and behavioral implementation remains**
Owner: `SPEC-TO-INTENT-ALIGNMENT.6d.ii`
Audit date: 2026-08-12
Discovery revision: `b977a51ff24f966dcf6aca74ccf47d592a4fc452` plus the `.6d.ii.a` replay publication
Qualified identity-remediation revision: `9c38b5699619dfccaa30a99d10c69d1fa5bf58cc`

Accepted signoff architecture: [`ADR 0038`](../decisions/0038-proof-carrying-genericity-kernel.md)

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

## Combined identity-remediation qualification (`.6d.ii.d.iv`)

The qualification boundary is the exact three-commit range after generic EvidenceIR landed:
`89d8dee7..9c38b569` (`.d.i` through `.d.iii`). Git measures 129 changed files, 5,922 insertions,
and 4,357 deletions. The crate-source part is 24 Rust files (+3,522/-3,328), including production edits
and their embedded `cfg(test)` controls; the principal
qualification/data surfaces are 40 conformance-fixture files (+75/-75), 17 corpus-KB files
(+1,000/-347), and ten mdBook files (+145/-77). Those counts describe the complete range rather than
an estimated word-list census.

The resulting current state is exact:

- prior memory is schema 7 with one global identity-independent scope; the validation-gated 13-input
  fixed point is byte-stable at `a416cc8b…6239633`, exposes one contested prior, and removes 37 stale
  prior-created timing constraints while retaining their source statements;
- identifier remediation changes 18 EvidenceIR, 73 SemanticIR, 74 IntentIR, and 74 adapter artifacts;
  evidence constraints move 397→344, invariants 31,767→28,876, transactions 281→239, and
  signal-neutral conditional rules 2,156→2,489. Renderability moves 44→17 because the adapter no
  longer invents clock/reset semantics; all 17 emitted ISFs pass pinned FSMGen strict;
- prompt remediation covers all 13 production constructor families. Corpus-KB routing has ten managed
  Markdown outputs plus schema-2 JSON, and all 156 typed fixtures reproduce. This child intentionally
  changes provider instructions and conformance projections, not persisted canonical IR;
- retained-chain currency is 24/24 for measurable EvidenceIR and 78/78 for each SemanticIR, IntentIR,
  and adapter stage under the current binary.

The focused cross-path qualification is deliberately behavioral, not lexical: the `alpha`, `identity`,
`prompt`, `spelling`, `fixture_name`, empty-declaration, and undeclared-contract filters pass 13, 19,
14, 14, 2, 1, and 1 tests respectively. They cover renamed prompt policy, source classification,
prior-scope quarantine, exact/ambiguous identifier resolution, signal/interface/handshake/clock/reset
non-inference, typed ISF lowering, structural corpus membership, and fail-closed provider proposals.
The complete Rust and repository gates then cover all non-filtered paths and persisted currentness.

This closes the bounded identity/spelling/prompt/corpus remediation parent `.d`; it does **not** claim
whole-core production-genericity signoff. Named production commentary and calibrated evaluation claims,
the proof-carrying core/conformance and registered-grammar architecture, AST/dependency/taint mutations,
and whole-population alpha/identity/paraphrase/held-out behavior remain release blockers in `.e` and `.f`.

## Proof-architecture freeze (`.6d.ii.e.i`)

ADR 0038 makes the signoff direction concrete before production code moves. The target is a compiler-visible
`specforge-core <- specforge-conformance` dependency, opaque document/symbol identities with sealed spelling
capabilities, registered grammars that emit grounded proposals, and one small promotion kernel that admits only
proof-covered canonical claims. Each artifact carries a ruleset-bound proof ledger; each rule declares its premise
kinds, conclusion surface, symbol capability, compatibility, and executable alpha obligation. Proofless historical
artifacts may be inspected, but cannot steer a current canonical build or lower to ISF.

Two checked TSV inventories freeze the migration denominator. `module_inventory.tsv` classifies all 71 compiled
Rust files exactly once, including every mixed, named, calibrated, evaluation, replay, and test-support location.
`claim_family_inventory.tsv` assigns all 168 top-level fields of SourceIR, EvidenceIR, SemanticIR, IntentIR, and
the adapter artifact exactly once across 38 claim families. `check_production_genericity_inventory.pl` derives
both live sets and rejects new, missing, duplicate, malformed, or stale entries; its controlled self-test proves
that an unclassified module and an unclassified artifact field both fail.

This slice changes no production extraction behavior. The architecture and denominator are accepted, not yet
implemented: `.e.ii`–`.e.vii` still own the physical core/conformance split, trusted kernel and capabilities,
rule/claim migration, AST-aware doctrine, adversarial mutations, and exact qualification. `.f` still owns
whole-population behavioral qualification. A vocabulary scan remains supplementary diagnostics only.

## Compiler-visible core/conformance boundary (`.6d.ii.e.ii`)

The first architecture layer is now implemented as three workspace packages:

```text
specforge-core  <-  specforge-conformance
      ^                    ^
      +------ specforge application/CLI
```

`specforge-core` compiles capture, generic provider transport, SourceIR/EvidenceIR/SemanticIR/IntentIR,
validation primitives, and adapter lowering. It does not name or depend on the conformance package.
`specforge-conformance` depends only one way on core and compiles `eval`, completeness/category
characterization, source-to-intent evaluation/replay, trajectory control, and reviewed named snapshot
composition. The application composes both and keeps the former `specforge::eval` and `specforge::ir::*`
public paths as compatibility re-exports. Named fixture identities remain legal in conformance tests; an
expanded production-core diagnostic finds no known named corpus term in the non-test compiled output. That
vocabulary result is review assistance, not the dependency proof.

`scripts/check_production_genericity_dependencies.pl` derives the manifest and module-registry direction and
has five controlled cases: clean graph, direct reverse edge, aliased reverse edge, conformance-to-application
cycle, and oracle-module reinsertion. `cargo tree -p specforge-core --edges normal` independently shows no
internal SpecForge dependency. The exact live inventory grows from the frozen 71-file pre-migration denominator
to 76 classified Rust files while retaining the same 38 claim families and 168 fields. This changes packaging
and visibility, not canonical extraction behavior or persisted schemas.

The boundary removes conformance feedback as a representable core dependency; it does not prove each core claim.
At that checkpoint `.e.iii` still owned the sealed capability/kernel substrate described next; `.e.iv`–`.e.vi`
continue to own rule migration, AST/information-flow enforcement, and adversarial proof/alpha mutations. `.e.vii`
and `.f` own structural and population qualification, so whole-core signoff remains explicitly open.

## Sealed capability and derivation substrate (`.6d.ii.e.iii`)

The trusted substrate now compiles in `specforge-core::ir::derivation`. `OpaqueSymbol` and
`OpaqueDocumentIdentity` retain private spelling but expose only spelling-free scope/ordinal identity and exact
comparison. They implement no `Display`, string conversion, spelling-based ordering, or Serde. Grammar,
persistence, presentation, and lowering access requires a sealed token that application/conformance code cannot
construct. Five external compile-fail tests pin those negative API properties; this is a type boundary, not a
known-word scan.

The schema-1 proof ledger and deterministic SHA-256 ruleset bind each unique stage/surface/record address to its
rule, typed premises, spelling-free symbol-use modes, confidence class, and exact conclusion digest. The promotion
kernel validates current-scope source spans, table cells, and visual regions; independently grounded model
proposals; validated current-document or global-identity-independent priors; registered universal axioms; and
already-verified upstream claims in ledger order. It rejects an unknown or wrong-surface rule, stale/cross-document
premise, unsupported premise kind, excess symbol capability, missing symbol origin, duplicate address, stale
ruleset, and altered conclusion bytes.

Compatibility distinguishes envelope shape from authority. Missing, old, or stale ledgers are inspect-only and
must rebuild or residualize; future and malformed ledgers reject. Even a schema-valid, ruleset-matching
deserialized envelope requires full kernel verification. Only the non-deserializable `VerifiedProofLedger` token
permits canonical use, so persisted bytes cannot forge authority around a claim. Sixteen focused controls plus
five compile-fail controls exercise the boundary.

This slice changes no current IR schema, producer, or canonical artifact. It therefore does not claim that the
existing 168 fields are proof-covered: `.e.iv` owns exact migration of all 38 claim families and removal of the
temporary pre-integration dead-code allowance. The live module denominator is 77; rule completeness, AST/taint
enforcement, per-rule alpha execution, and population qualification remain `.e.iv`–`.f` blockers.

## Exact rule and canonical-seam census (`.6d.ii.e.iv.i`)

The migration census now joins all 38 claim families to 168 deterministic field-root rule ids, 113 current
producer or post-build mutator entrypoints, and 39 canonical insertion/write seams. The latter include every
stage writer and `validate`'s separate raw backannotation writer. `rule_family_inventory.tsv` fixes, for every
family, the current entrypoints, allowed typed premises, symbol capability, mandatory alpha obligation,
compatibility, target module, schema owner, canonical seams, and one stage migration leaf. The checker resolves
every declared path/function and rejects missing families, duplicate rules, premise/capability mismatches, stale
seams, and unsafe compatibility.

This inspection found two important classes that a constructor-only migration would miss. First, EvidenceIR can
carry an existing artifact's aliases, statement classes, constraints, and conditional rules, then deduplicate or
reclassify them; provider commands, register recovery, signal resolution, and validation can also mutate an
already-built stage. These paths must verify the incoming cumulative ledger and extend it through a registered
rule. Second, conformance/evaluation code currently patches temporary SourceIR, EvidenceIR, and SemanticIR structs
or rewrites artifact layout before using ordinary persistence/build paths. The four exact entrypoints are tracked
separately in `conformance_bypass_inventory.tsv`; temporary location is not authority.

The accepted integration is one cumulative proof chain. SourceIR proves exact capture before classification;
downstream stages verify and retain the exact ordered upstream ledger as a prefix, then append current-stage
claims. Every top-level field receives a root conclusion even when empty, and populated record collections also
receive stable per-record conclusions. Canonical load, serialization, write, downstream build, and lowering all
verify the current artifact/ledger; public mutation invalidates its digests until a registered mutator extends the
proof chain. Proofless history is inspection/rebuild/residual input only, and future versions fail closed.

Conformance patches will move to noncanonical typed overlays unable to call canonical production sinks; the
evaluation storage rebase will preserve verified proof bytes without authorizing semantic changes. This slice
freezes the reviewed high-level migration graph and cumulative-ledger contract only. It changes no artifact schema,
producer behavior, or canonical output. Stage migrations remain `.e.iv.ii`–`.e.iv.vi`, while `.e.v` must derive
helper-level AST/information-flow closure rather than treating a reviewed entrypoint inventory as sufficient proof.

## Audit denominator and method

### North-star scope: domain-specialized, specification-instance-neutral

SpecForge is not intended to be neutral across arbitrary document domains. It is a supersmart colleague for
digital-chip design: protocols, interfaces, ISAs, registers, state machines, clock/reset and power topology,
timing, ordering, exceptions, coherency, and related implementation intent are legitimate closed domain concepts.
The neutrality obligation is across specification instances, PDF layouts, vendors, protocol families, and opaque
document identifiers. The production core may implement universal digital-design and document-language semantics;
it may not conclude that an arbitrary identifier, title, filename, vendor, family, or familiar example carries one
of those semantics without typed evidence from the current input.

The intended product path is explicit: source documents become successively justified SourceIR, EvidenceIR,
SemanticIR, and IntentIR; supported executable intent lowers to FSMGen ISF; FSMGen owns scheduling and HDL
lowering. A domain-capable model may supply expert proposals, but the verified IR and its provenance—not the
model's familiarity with a named specification—are the authority.

The denominator is the compiled production crate, not a selected word list:

- the frozen 71 Rust source files below `crates/specforge/src/`, plus the five post-split package/transport
  modules in the live 76-file inventory;
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

### Accepted signoff architecture: proof-carrying extraction, not vocabulary completeness

Neutrality cannot be established by enumerating forbidden fragments: an unseen identifier can encode the same
shortcut, and an innocent listed word can be legitimate input data. The signoff boundary is instead an
information-flow and derivation boundary. Production recognizers and models may propose candidate claims, but a
claim is promotable only when a small kernel can validate a proof term whose premises are typed structural facts or
current-document evidence. Identifier values are opaque atoms: code may preserve them and perform exact identity
comparisons, with unique-only recovery for case-normalized source presentation, but their characters cannot select
a semantic rule. Every registered inference rule must be alpha-equivariant, and generated counter-specs will test
that property under arbitrary renaming, paraphrase, formatting perturbation, and misleading familiar names.

This architecture does not promise to recover intent that the source does not express. It promises that supported
intent is justified without identity shortcuts and that missing, ambiguous, contradictory, external, or unreadable
intent remains explicit. The finite forbidden-vocabulary census remains a useful diagnostic tripwire, never a proof
of neutrality.

The full type, proof, dependency, rule-registry, mutation, and compatibility contract is accepted in
[`ADR 0038`](../decisions/0038-proof-carrying-genericity-kernel.md). Its live migration denominator is reverified
with `perl scripts/check_production_genericity_inventory.pl`.

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
| Source registration/materialization | Source kind, repository-local artifacts, Docling page/table/visual conversion, closed visual-form/section grammar, typed table-role classification, timing-table structural revalidation | Remediated by `.6d.ii.b`: schema-2 classification is identity-independent and fail-closed; schema-1 semantic labels are neutralized on load. |
| VLM enrichment | Caption-selected timing/state images; typed JSON prompts; table kind/grid proposals checked against headers | Prompts now describe generic digital-hardware structure, require visible/current-document grounding, and treat labels as opaque. |
| Evidence assembly | Markdown blocks, spans, section anchors, references, table declarations, typed provenance | Broadly structural/input-derived. |
| Evidence deterministic extraction | Registers, fields, signal inventories, relations, constraints, polarity, semantic hints, timing, frames, states, actors, operations | Protocol structure and identifier-spelling authority are remediated; named compiled documentation is neutralized. Proof-carrying rule admission remains open. |
| Prior-guided EvidenceIR | Table/visual/actor/semantic/temporal priors | Remediated by `.6d.ii.d.i`: schema 7 has one global scope; normalized current-document evidence and structural fingerprints select priors, while identity is provenance only. |
| Text LLM/NLP | Per-sentence JSON extraction, declared-signal grounding, entity typing, dedup, optional promotion | Remediated by `.6d.ii.d.iii`: typed tasks, declaration-ordered catalogs, undeclared-contract residualization, model-proposal-independent constraint typing, and identifier-redacted entity judgment. |
| SemanticIR | Declared-signal gate; interfaces, actors, ports, connectivity, infrastructure, temporal rules/contracts; VLM fusion/fidelity; residuals | Identifier-spelling inference and named compiled documentation are removed; proof-carrying inference admission remains open. |
| IntentIR | Lossless carry-forward; actor/temporal relations; generic transaction synthesis; residual decisions | Name-created behavior is removed and generic carriers are preserved; proof-carrying structural enforcement remains open. |
| ISF adapter | Typed lowering, syntax validation, conflict rejection, unsupported-record residuals | Lowering consumes typed system/actor/temporal contracts and fails closed; proof-covered lowering remains open. |
| Validation/evaluation | Stage metrics, negative-knowledge lookup, extraction scores, completeness/category classification | Prior and signal validation are identity-neutral. Evaluation and calibrated classification now compile downstream in `specforge-conformance`, so they cannot feed a core dependency. |
| Corpus/trajectory utilities | Learn priors, structural clustering, KG pages, reviewed replay/controller | Corpus-KB routing derives capabilities/prior surfaces/control status only from populated fixture-schema fields; names are display/provenance. Replay, trajectory, and reviewed snapshot composition compile downstream of core. |

## Confirmed production violations

### Resolved in `.6d.ii.d.i` — document identity selected protocol-family semantics

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

`.6d.ii.d.i` removes that path. CorpusMemory schema 7 has only `PriorScope::Global`; EvidenceIR and
SemanticIR loaders no longer receive a document key or display name, and learning groups priors by normalized
current-document evidence and typed/structural keys. A current scope other than `global` rejects. Schemas 1–6
quarantine their seven identity-scoped semantic prior families before typed load rather than relabelling them as
global truth; identity-independent structural profiles may survive.

The migration also exposed stale feedback authority. The old schema-6 store declared 14 accepted inputs,
including one deleted artifact, and could not be reproduced from the 13 current inputs because they lacked the
persisted validation required by policy. After validating those inputs, schema-7 learning converged in two
iterations; the third learn was byte-identical. The final store contains 29 actor, 83 semantic, four modality,
443 temporal, zero table, 1,251 visual, 11 negative, and two structural-profile priors. One global actor-term
contradiction is surfaced as contested. Whole-chain replay is exact at 24/24 measurable EvidenceIR and 78/78 at
every downstream stage. ADR 0036 records the fail-closed compatibility and fixed-point rule.

### Resolved in `.6d.ii.c` — protocol-specific EvidenceIR schema and extractors

At the audited revision, `ir/evidence.rs` exposed `SwdOperation`, `SwdioDirection`, and
`swd_operations` in the production schema. The associated production code:

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

`.6d.ii.c` installs the replacement. EvidenceIR schema 2 now carries generic frame fields, operation
records, participant-drive relations, and protocol states. Phase, branch, operation, field, response,
participant, and state terms are opaque current-document strings. Frame extraction requires explicit
frame/packet or phase binding; direction requires an explicit source-to-destination clause; operation
records require an explicit phase cardinality; state grammars require repeated inventory or explicit
machine/transition structure. Fixed response branches, phase enums, actor-role maps, named extractor
identities, and protocol-prefixed ids are gone.

Schema-1 protocol surfaces are cleared in untyped JSON before deserialization and the in-memory artifact
is upgraded to schema 2. This is deliberately fail-closed: unrelated evidence survives, but retained
vocabulary-bound protocol records cannot bypass the new producer. SemanticIR, IntentIR, convergence,
validation, evaluation, and the adapter carry the new records losslessly. Protocol-specific conformance
fixtures remain legal test/input data; they no longer define production policy.

The complete ADR 0025 reconciliation makes the cost explicit. All 78 persisted EvidenceIRs are schema 2:
24 were re-extracted from retained normalized bundles and 54 were migrated by preserving unrelated evidence
while clearing obsolete protocol authority. Compared with the exact pre-change backup, all 78 EvidenceIR files
change, while 24 SemanticIR/IntentIR/adapter chains have content deltas. The old population held 22 fixed-phase
frame records in four documents, four named operations in one, and 76 state records in 24. The neutral producer
emits five generic operations in two documents and 40 structurally admitted states in three; no retained source
satisfies the stricter explicit phase-binding grammar for a frame record. Deterministic currency is 24/24 for
measurable EvidenceIR and 78/78 for every downstream stage. This is an honest recall frontier, not permission to
restore a specification name.

This closes only the protocol-structure carrier/extractor breach. The repository is not yet genericity signoff-
ready: `.6d.ii.d` owns remaining identity/spelling authority, `.e` owns structural production-boundary
enforcement and named production commentary, and `.f` owns alpha-renaming/identity/paraphrase/held-out behavior.

### Resolved in `.6d.ii.d.ii` — signal spelling changed semantic authority

`ir/semantic.rs::classify_handshake_signal_from_name` assigns handshake roles when a signal name
contains `valid` or `ready`; `commands/validate.rs::handshake_name_heuristic_role` repeats the rule.
`should_emit_interface_candidate` grants single-token interface authority for suffixes/substrings
such as active-low, reset, and clock naming conventions. `signal_stop_words` contains standards,
technology, interface, and architecture names and thereby changes which uppercase input tokens can
be signals. `ir/evidence.rs::is_abstract_transport_signal_token` likewise matches a fixed set of
signal spellings. The downstream adapter repeats the prohibited inference:
`ir/isf_ir.rs::IsfIr::from_intent_ir` selects clock/reset infrastructure from `clk`/`clock` and
`rst`/`reset` substrings, overrides an explicit reset kind when the name ends in `_n`/`_b`, and
otherwise manufactures conventional clock/reset identifiers and semantics. This adapter finding was
not explicit in the initial discovery text; the active `.6d.ii.d.ii` census corrected its scope
before product-code remediation.

These are not closed digital semantics: they infer meaning from arbitrary identifiers. Roles must
come from table columns, prose, diagrams, explicit declarations, typed upstream records, or a model
proposal grounded back to that evidence. Alpha-renaming must preserve the same graph, obligations,
validation result, and lowering decision modulo renamed symbols; downstream stages may not
reconstruct a role that the upstream evidence left unresolved.

`.6d.ii.d.ii` replaces that entire path with opaque identity and one-way grounding. EvidenceIR signal
admission uses typed table structure or bounded definition grammar; identifier syntax is case- and
length-independent. Model/NLP proposals must match the current document's declaration catalog exactly, with
exact-first unique-only resolution and no empty-catalog escape. SemanticIR interfaces and descriptive
invariants require declared authority, semantic-role disagreement remains unresolved, and IntentIR no longer
mints handshake, request/acknowledge, FIFO, drive, trigger, or transaction behavior from name fragments.
Validation measures unresolved role evidence without testing the identifier. The ISF adapter accepts clock,
reset timing, and reset polarity only from the typed system contract; missing or unknown semantics produce an
explicit blocked diagnostic form rather than conventional defaults.

Removing case-based rejection exposed two under-specified prose grammars during the required retained replay.
The first word of a multi-word parenthetical aside and the word after `pin,` had been accepted only because
ordinary prose was usually lowercase. The repaired grammar requires a complete one-token parenthetical, a
bounded pin appositive, and rejects a parenthetical modifier when the wire noun is already modified by a marked
identifier. This is positive structural authority, not a replacement word list. A second replay exposed an
invariant grammar that had used “contains an uppercase token” as its grounding predicate; it now requires an
exact declared signal for descriptive behavior, while deontic statements remain identifier-neutral.

Exact rollback comparison accounts for the result: 18 EvidenceIR, 73 SemanticIR, 74 IntentIR, and 74 adapters
change. Evidence signal constraints move 397→344; SemanticIR invariants 31,767→28,876; transactions 281→239;
signal-neutral conditional rules 2,156→2,489. ISF renderability moves 44→17 because 27 documents lack a complete
typed clock/reset contract. All 17 emitted targets pass pinned FSMGen strict, and chain currency is exact at
24/24 measurable EvidenceIR and 78/78 for every downstream stage. ADR 0037 records the durable boundary.

### Resolved in `.6d.ii.b` — SourceIR PDF classification contained corpus-calibrated phrases

At the audited revision, embedded Python in `ir/source/docling_backend.rs` classified visuals and
tables before EvidenceIR using operation/burst phrases, interface-role phrases, field-reference
spellings, and packet-layout exclusions derived from particular protocol families. Those branches
decided which images reached the VLM and whether a table became a register map.

`.6d.ii.b` removes that authority. Diagram classification now accepts only an explicitly named
generic visual form; operation, participant, and symbol text remains `unknown`. Table
classification uses normalized whole header roles and required structural conjunctions: signal
identity plus direction (or explicit signal identity plus width), value plus meaning, register name
plus access plus address/range, scalar timing roles plus context, or feature plus support. Ambiguous
`Name | Width | Description`, bare hex/address layouts, substring collisions, and unsupported
forms remain `unknown`. Section classification uses whole generic heading phrases, so a fragment
such as `port` cannot acquire authority merely by occurring inside another word.

The carrier is versioned as SourceIR schema 2. Loading schema 1 preserves source text, grids,
geometry, provenance, and assets but neutralizes diagram/table/section semantic labels to
`unknown`/`normative`; re-ingest is required to reconstruct schema-2 labels. Future schemas are
rejected. This prevents retained corpus-tuned labels from bypassing the repaired ingest boundary.

A read-only replay of the new classifier across all 78 retained SourceIR documents measured the
intentional impact before legacy fail-closure: 2,293 diagram labels, 2,123 section labels, and 3,484
table labels would change on re-ingest. The dominant removals were 1,918 old register-bitfield and
317 old timing-diagram guesses, 1,954 old encoding guesses, 218 feature guesses, and 136 timing-table
guesses becoming unknown; 754 previously unknown tables gain structurally complete register-map
authority. That diagnostic did not mutate an artifact.

The required ADR 0025 currency run subsequently found exactly ten affected replayable EvidenceIR
cascades. All ten were backed up on the repository volume, rebuilt through the ISF adapter, compared,
and validated. Exact currency is now 24/24 measurable EvidenceIR and 78/78 SemanticIR, IntentIR, and
adapter chains; the two renderable outputs pass pinned FSMGen strict with zero diagnostics. The delta
removes 194 weak table-derived timing records across four documents, weak actor/register authority in
one debug-document chain, and other isolated label-driven facts; a structurally complete field-layout
chain gains registers under the positive conjunction. The large recall reduction is accepted evidence
that the old labels encoded weak corpus shortcuts, not a reason to keep them. Generic recovery belongs
to later EvidenceIR grammar/evidence leaves and cannot restore a result through document identity.

### Resolved in `.6d.ii.d.iii` — named corpus code was compiled as product code

`commands/corpus_kb.rs` has a named protocol-family label, output page, prompts, and fixture-name
classifier. At the audited revision, the reviewed trajectory composer was also exported from
`ir/mod.rs` and exercised through a production example even though it authenticates exact reviewed
documents and results. This slice removes that second breach: the composer now lives below the
`#[cfg(test)]` `test_support` module, the public IR registration and production example are gone, and
its named assertions remain test-only. `.d.iii` removes the remaining named routing: `corpus_kb` no longer
classifies fixture names, emits an AMBA page, or infers positive/negative status from filename fragments.
`kg_bench` computes a typed structure profile from populated schema fields; corpus pages select only those
capabilities. The prior-candidate manifest schema 2 distinguishes prior-present and control surfaces without
claiming that a named “gold” fixture is positive. Exact fixture identities remain legal conformance provenance.

Corpus organization, review gold, and exact trajectory snapshots are conformance/evaluation assets,
not product extraction. The extraction core cannot import them. The remaining `.e` structural boundary must
also place named conformance/oracle implementation behind an explicit conformance target; `.d.iii` closes the
semantic routing breach without claiming that later physical module separation is already complete.

### Resolved in `.6d.ii.d.iii` — production model prompts carried unnecessary domain priors

`commands/nlp_enrich.rs::build_nlp_prompt` described the agent as a protocol analyzer and gave
named real signal examples. `commands/enrich.rs` similarly framed diagrams/tables as protocol PDFs,
and `ir/constraint_extract_llm.rs::extraction_prompt` conflated fields with signals. The complete
constructor census then found a subtler breach: `entity_prompt` sent the raw identifier to the model and
explicitly relied on familiar-name world knowledge; contract and LLM-primary constraint prompts also lacked a
closed current-document declaration catalog.

Prompts are executable extraction policy. They must describe the neutral typed output and use
synthetic placeholders generated per request, while injecting only document-derived grounding. `.d.iii` applies
that rule to relation, NLP constraint, contract, condition, entity, VLM diagram/table/audit, register-diagram,
constraint-extraction, and NLI prompts. Entity typing substitutes `<OPAQUE_IDENTIFIER>` in both the prompt and
helper transport. Signal/value-carrier catalogs preserve declaration/provenance order—sorting by spelling was
rejected because alpha-renaming could reorder a transformer input. Contract proposals that reference an
undeclared signal become explicit residuals, and LLM-primary constraints type only against independently built
signal/field catalogs. Alpha controls pin policy equality modulo injected symbols; named real examples are absent.

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
| Entrypoint/platform | `specforge-core`, `specforge-conformance`, and application package roots; `main.rs`, `cli.rs`, `error.rs`, `persisted_path.rs`, `project_data.rs`, `provider.rs` | Compiler-visible one-way package graph plus neutral infrastructure. |
| Test support | `test_support.rs`; test-only functions in `ir/mod.rs` | Test-only by design; keep outside production dependencies. |
| Thin stage/utility commands | `adapt`, `clean`, `doctor`, `evidence`, `ingest`, `inspect`, `intent`, `semantic`, `extract_conditions`, `extract_contracts`, `recover_register_bits`, `rescan_plan`, `signal_resolve` | No identity-specific extraction branch found; they inherit the IR behavior they invoke. |
| Model/quality commands | `audit_extraction`, `enrich`, `extract_constraints_llm`, `grits_consensus`, `nli_verify`, `nlp_enrich`, `project_validation` | Prompt policy is typed, current-document-grounded, opaque-symbol, and alpha-controlled. Corpus literals observed in these files are test-only or historical commentary owned by `.e`. |
| Orchestration/evaluation commands | `converge`, `eval_extraction`, `kg_bench`, `validate` | Validation/orchestration do not infer semantics from identifier spelling; KG fixture capability metadata is structural. The application may compose core and conformance, while neither package can depend on it. |
| Corpus commands | `corpus_cluster`, `learn_priors`, `corpus_kb` | Structural clustering, schema-7 learning, and schema-field-derived corpus-KB organization are neutral; fixture identity is display/provenance only. |
| SourceIR | `ir/source.rs`, `ir/source/docling_backend.rs` | Neutral at schema 2: lifecycle is structural, classifiers use generic form/role grammar with honest unknown, and legacy semantic labels fail closed. |
| EvidenceIR/extraction | `ir/evidence.rs`, `extractor.rs`, `extraction_filters.rs`, `entity_typing.rs`, `condition_extract.rs`, `constraint_extract_llm.rs`, `nlp_relation_extract.rs`, `normative_vocab.rs`, `register_bits.rs` | Signal identities and prompt symbols are opaque and one-way grounded; compiled documentation uses only structural examples. Framework, grounding primitives, and parsers are reusable; proof-rule migration remains open. |
| Prior/reuse | `ir/prior_memory.rs`, `ir/corpus_cluster.rs` | Neutral at schema 7: one global scope, fail-closed legacy quarantine, normalized evidence keys, and structural fingerprints; no document identity selects a lookup. |
| SemanticIR | `ir/semantic.rs`, `ambiguity.rs`, `contract.rs`, `cve.rs`, `fidelity.rs`, `figure_region.rs`, `fusion.rs`, `nli_verify.rs`, `protocol_graph.rs`, `temporal_ltl.rs`, `waveform.rs` | Typed fusion/verification no longer assigns interface, invariant, handshake, infrastructure, or polarity meaning from identifier spelling; compiled documentation is neutral. Proof-rule enforcement remains open. Figure-region corpus paths are test-only. |
| IntentIR | `ir/intent.rs` | Structural synthesis no longer creates transactions or actor/temporal behavior from name fragments; compiled documentation is neutral and the proof-carrying boundary remains open. |
| Adapter | `ir/adapters.rs`, `ir/isf_ir.rs` | Clock/reset lowering consumes the typed system contract and fails closed when it is absent/incomplete; compiled documentation is neutral and proof-covered lowering remains open. |
| Completeness/evaluation | `ir/completeness.rs`, `eval.rs` | Public reusable APIs compile only in `specforge-conformance`; corpus calibration and named tests cannot become core dependencies. |
| Replay/trajectory | `ir/source_to_intent_eval.rs`, `source_to_intent_replay.rs`, `trajectory.rs`; `test_support/trajectory_snapshot.rs` | Public engines and named reviewed snapshot composition compile only downstream in `specforge-conformance`. |
| Module registry | core `ir/mod.rs`; conformance `ir.rs`; application facade | Core excludes every oracle module; conformance imports core one way; the application preserves compatibility re-exports. Dependency mutations fail closed. |

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
