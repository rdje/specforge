# Production-genericity pipeline audit

Status: **discovery, proof migration, structural enforcement, adversarial mutation, per-rule alpha, and final structural/migration-delta qualification complete; population behavioral qualification remains**
Owner: `SPEC-TO-INTENT-ALIGNMENT.6d.ii`
Audit date: 2026-08-12
Latest structural update: 2026-08-13 (`.6d.ii.e.vii`)
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

The breach does **not** show that neutral specification-to-intent is impossible; it shows that shortcuts were
admitted instead of making neutrality structural. A domain extractor necessarily knows universal digital intent—
signals, fields, registers, states, events, obligations, timing, provenance, conflict, and uncertainty. It must
not let document/vendor/protocol identity, symbol spelling, or one tuned corpus phrase decide a result.

The feasible architecture has three layers:

1. universal typed digital-intent semantics and document-structure grammars;
2. opaque names, values, relations, and evidence derived from the current input;
3. optional learned/model proposals with explicit provenance, uncertainty, and grounding back to that
   input, never selected by filename/title identity.

No tool can promise perfect recall from corrupt, encrypted, image-only, contradictory, incomplete, or ambiguous
input. “Works on any digital-chip specification” means one neutral engine accepts it, promotes only justified
intent, and residualizes uncertainty without a named branch. Existing typed provenance, grounding, fusion,
fidelity, and residual mechanisms show this is implementable. Removing shortcuts may reduce recall temporarily;
universal grammars and grounded models must recover it without letting benchmarks override the boundary.

## Combined identity-remediation qualification (`.6d.ii.d.iv`)

The exact `89d8dee7..9c38b569` qualification range spans 129 files. It establishes schema-7 global prior
identity, neutral prompts across 13 constructor families, typed structural corpus routing over 156 fixtures,
and exact 18/73/74/74 Evidence/Semantic/Intent/adapter deltas. Removing invented clock/reset authority moves
renderability 44→17; every retained emitted ISF passes pinned FSMGen strict. Focused alpha, identity, prompt,
spelling, fixture-name, declaration, and grounding controls plus full gates qualify the bounded remediation.
The owning task retains exact file/test/metric arithmetic. This closes identity/spelling/prompt/corpus repair,
not structural or population-level genericity signoff.

## Proof-architecture freeze (`.6d.ii.e.i`)

ADR 0038 fixes the signoff architecture: a compiler-visible one-way core/conformance dependency, opaque
document/symbol identities, sealed spelling capabilities, registered grounded proposals, and one promotion
kernel that alone grants canonical authority. Rules declare typed premises, conclusion surface, symbol
capability, compatibility, and alpha obligation; proofless history is inspection-only. Checked module and claim
inventories reject drift across the compiled surface and every top-level artifact field. The ADR and owning task
retain the full frozen denominator and controlled-failure evidence; this audit retains the resulting boundary.

## Compiler-visible core/conformance boundary (`.6d.ii.e.ii`)

The architecture compiles as three workspace packages:

```text
specforge-core  <-  specforge-conformance
      ^                    ^
      +------ specforge application/CLI
```

Core owns generic capture, IR, extraction, validation primitives, and adapter lowering. Conformance depends one
way on core and owns evaluation, completeness/category characterization, replay, trajectory, and reviewed named
snapshots; the application composes both and preserves public compatibility paths. Five dependency mutations
reject direct and aliased reverse edges, an application cycle, and oracle-module reinsertion. This makes
conformance feedback unrepresentable as a core dependency; vocabulary diagnostics remain supplementary only.

## Sealed capability and derivation substrate (`.6d.ii.e.iii`)

`specforge-core::ir::derivation` implements source-order opaque symbol/document atoms with no ordinary spelling,
display, ordering, or Serde API; sealed capabilities grant grammar, persistence, presentation, or lowering access.
The deterministic ruleset and proof ledger bind unique addresses to typed premises, spelling-free symbol uses,
confidence, and exact conclusions. The kernel checks capture/model/prior/axiom/upstream authority and returns the
only non-deserializable witness accepted by canonical seams. Old/stale ledgers rebuild or residualize; malformed
or future ledgers reject; schema-valid bytes never self-authorize. Focused and compile-fail controls pin those
properties. ADR 0038 and the production-genericity Knowledge Map card retain the full type/premise/error contract.

## Exact rule and canonical-seam census (`.6d.ii.e.iv.i`)

The checked graph joins 41 families to 168 field rules, 117 producer/post-build-mutator entrypoints, 53 canonical
seams, and four conformance-only bypass obligations. It includes constructor-independent reclassification,
enrichment, validation, and persistence paths; temporary fixture location grants no authority. SourceIR proves
capture, each downstream stage retains the verified ledger as an exact prefix, and every field plus populated
record receives a claim. Canonical load/serialize/write/build/lower seams re-execute proof; closed mutators extend
it; history is inspection/rebuild/residual input only. The inventories and task evidence own the exact path-level
denominator and controlled checker failures.

Adapter schema 2 preserves the verified IntentIR ledger as an exact ordered prefix and proves all 12 local fields
across envelope, lowering, residual, and validation families. Every populated array record, nonblank rendered ISF
line, and blocking reason has a claim. Canonical build/load/serialize/write/emitted-file reconciliation and closed
validation mutation re-execute current lowering, so proofless, stale, forged, unauthorized, legacy, or future
artifacts cannot emit or preserve authoritative ISF. Exactly 24 reachable adapters migrated with zero public
delta outside schema/proof/validation; all 24 are currently blocked and reconcile to zero emitted files. Fifty-four
historical chains remain upstream-proof-unmeasurable rather than receiving synthetic authority.

Whole-module implementation digests safely failed closed but over-invalidated: one test-only edit staled all 24
IntentIR proofs without changing product fields. `.e.iv.vii` replaced them with compiler-derived production
registry/verifier/item/import/kernel closures, preserving currency for inert edits while invalidating real
relation changes.

## Compiled production syntax graph (`.6d.ii.e.v.ii`)

A fourth workspace package, `specforge-production-graph`, derives the structural denominator without depending
on any product package and without allowing any product package to depend on it. It reads Cargo metadata under
the default production feature closure, evaluates compiler and Cargo configuration, starts from all four live
library/binary targets, and parses the complete 78-file inventory. The current result is 77 reachable production
files plus one explicit test-support-only file, 78 target-module identities, 3,260 items, 1,212 imports,
26,072 call sites, five local macro definitions, and 9,674 macro invocations. The additional reachable file is
the conformance-owned behavioral harness; it remains downstream of core.

The analyzer resolves ordinary and `#[path]` modules, inline modules, imports, re-exports, globs, and aliases;
rejects absent, duplicate, ambiguous, unparsed, disabled-but-misclassified, or unsupported syntax; and emits
deterministic repository-relative JSON. It links exact local calls and macro definitions where syntax proves the
binding, while classifying method calls, associated calls, external/prelude bindings, callable expressions, and
other type-dependent dispatch honestly for the compiler oracle. Unit controls run the live graph twice for
byte identity and seed eight independent fail-closed faults. This is the exact substrate for `.e.v.iii`; it does
not yet prove raw/identity noninterference or proof-only promotion, and it is not yet the registered doctrine.

## Whole-production information-flow result (`.6d.ii.e.v.iii`)

The structural tool now joins the compiled graph to `information_flow_boundary.tsv`, a closed registry of typed
source, grammar, declassifier, proof, authority, and sink roles. Its 140 rows resolve to 22 source aggregate
types, 13 source fields, three exact legacy-string parameter classes, two provider returns, 119 rule roots, 64
grammar/declassifier roots, 12 canonical seams, 25 proof gates, 11 trusted regions, five non-authoritative
regions, and 15 protected types. Rule roots, canonical
seams, canonical fields, and conformance bypasses are derived directly from their existing inventories rather
than being copied into a second reviewed list. A malformed, duplicate, or unresolved registry row fails closed.

Symbolic dependencies propagate to a deterministic fixed point across 2,268 production functions and 11,811
helper edges. Parameters, typed fields, provider returns, local and conservatively resolved method calls,
closures, expression results, and macro tokens retain raw/identity dependence. The check rejects that dependence
at 11,339 branch/selection/decision sites unless the call topology proves membership in a registered universal
grammar, narrow exact-identity declassifier, trusted proof boundary, or non-authoritative inspection surface.
Sensitive input entering an unclassified semantic macro fails instead of being ignored.

Canonical authority is checked separately: canonical field assignment/mutation, 19 protected constructions, and
28 protected calls are confined to their registered rule/kernel regions; all 12 seams must reach stage-matched
proof gates; non-authoritative and identity-declassifier closure must remain disjoint from seams; and persistence
sinks accept only values produced by registered verified serializers. Thirteen controlled flow mutations reject
an identity selector; distinct raw literal, substring, and regex decisions; cross-class declassification; direct
or aliased unregistered canonical mutation; capability forgery; helper or macro laundering; a proofless or
wrong-stage seam; and duplicate registry identity. The clean fixture simultaneously admits raw spelling in
display, provenance capture, and excluded test support.

The implementation contains no named-spec vocabulary rule. It is deliberately stronger than a vocabulary
census and deliberately not presented as AST-alone proof: Cargo/Rust privacy owns types and sealed capabilities,
the AST owns closed flow/topology coverage, and executable current-binary replay owns semantic authority. The
registered `PRODUCTION-GENERICITY` doctrine now composes the dependency, exact inventory, rule-join, and graph/
flow checks unconditionally over the current production surface. Its CI qualification mode adds 27 controlled
mutations across dependency, inventory/schema, rule/alpha/bypass, and flow/authority boundaries. A test-only
oracle executes the structural premise/compatibility condition for each selected alpha obligation, and an exact
independent join proves all 168 runtime descriptors agree with the inventory without changing proof identity.
`.f.ii` now supplies the conformance-owned metamorphic harness, reviewed invariant calibrations, and complete
semantic-negative sensitivity. `.f` retains held-out and whole-population behavior qualification.

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
| Evidence deterministic extraction | Registers, fields, signal inventories, relations, constraints, polarity, semantic hints, timing, frames, states, actors, operations | Protocol structure and identifier-spelling authority are remediated; schema-3 EvidenceIR proves all 39 fields across 11 registered families and closes productive mutations. |
| Prior-guided EvidenceIR | Table/visual/actor/semantic/temporal priors | Remediated by `.6d.ii.d.i`: schema 7 has one global scope; normalized current-document evidence and structural fingerprints select priors, while identity is provenance only. |
| Text LLM/NLP | Per-sentence JSON extraction, declared-signal grounding, entity typing, dedup, optional promotion | Remediated by `.6d.ii.d.iii`: typed tasks, declaration-ordered catalogs, undeclared-contract residualization, model-proposal-independent constraint typing, and identifier-redacted entity judgment. |
| SemanticIR | Declared-signal gate; interfaces, actors, ports, connectivity, infrastructure, temporal rules/contracts; VLM fusion/fidelity; residuals | Schema-2 authority proves all 49 fields across 12 families, directly binds true EvidenceIR carries, and registers filtered/extended projections, synthesis, residual, and validation replay. |
| IntentIR | Exact carry-forward; filtered actor/contract projection; actor/temporal relations; generic transaction synthesis; residual decisions | Schema-2 authority proves all 49 fields across nine homogeneous families, directly binds 30 true SemanticIR carries, and closes validation plus demotion-only NLI mutation. |
| ISF adapter | Typed lowering, syntax validation, conflict rejection, unsupported-record residuals | Schema-2 authority proves all 12 fields, rendered source lines, blocking reasons, residuals, and closed validation from current verified IntentIR; proofless/stale artifacts cannot emit. |
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
| SourceIR | `ir/source.rs`, `ir/source/docling_backend.rs` | Neutral at schema 3: lifecycle is structural, classifiers use generic form/role grammar with honest unknown, exact capture/classification relations are executable, and legacy semantic labels fail closed. |
| EvidenceIR/extraction | `ir/evidence.rs`, `extractor.rs`, `extraction_filters.rs`, `entity_typing.rs`, `condition_extract.rs`, `constraint_extract_llm.rs`, `nlp_relation_extract.rs`, `normative_vocab.rs`, `register_bits.rs` | Signal identities and prompt symbols are opaque and one-way grounded; schema-3 EvidenceIR proves deterministic extraction plus every closed enrichment/backannotation mutation. |
| Prior/reuse | `ir/prior_memory.rs`, `ir/corpus_cluster.rs` | Neutral at schema 7: one global scope, fail-closed legacy quarantine, normalized evidence keys, and structural fingerprints; no document identity selects a lookup. |
| SemanticIR | `ir/semantic.rs`, `ambiguity.rs`, `contract.rs`, `cve.rs`, `fidelity.rs`, `figure_region.rs`, `fusion.rs`, `nli_verify.rs`, `protocol_graph.rs`, `temporal_ltl.rs`, `waveform.rs` | Typed fusion/verification no longer assigns meaning from identifier spelling; schema-2 proof executes carry/projection/synthesis/residual relations and closed validation. Figure-region corpus paths are test-only. |
| IntentIR | `ir/intent.rs` | Structural synthesis no longer creates behavior from name fragments; schema-2 proof executes exact carry, projection, product/relation synthesis, residual, validation, and conservative NLI demotion. |
| Adapter | `ir/adapters.rs`, `ir/isf_ir.rs` | Clock/reset lowering consumes typed system authority and fails closed when absent/incomplete; schema-2 proof covers every field, rendered line, blocking reason, residual, and canonical output seam. |
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

## Production-semantic proof identity result

Leaf `.6d.ii.e.iv.vii` replaces whole-module proof implementation hashes with compiler-visible,
schema-versioned stage closures. Each closure starts at the canonical production registry, follows the selected
verifier plus referenced stage-local production items and exact import bindings, and includes the complete
trusted derivation kernel. It removes comments, docs, formatting, and configured test/conformance branches;
unknown platform configuration is retained conservatively. Root absence or ambiguity fails the build.

Controlled mutations prove comments, docs, formatting, top-level/nested test code, unrelated production items,
and unrelated imports preserve identity, while a referenced helper or import binding changes it. The final
binding-aware migration changes only proof/validation metadata across 24 artifacts at each of five stages;
all 120 remaining public comparisons are exact, chain currency is 24 current / zero stale / 54 unmeasurable,
and the 120-file same-volume rollback snapshot is removed. This closes registered proof-relation identity. It
did not close the audit alone; the later `.e.v` graph/flow/doctrine work and `.e.vi` adversarial/alpha work now
close structural qualification. `.e.vii` owns final structural/migration-delta publication, and `.f` still owns
population-level behavioral invariance.

## Final structural qualification

Leaf `.6d.ii.e.vii` reconciles the whole structural program rather than adding another enforcement mechanism.
The exact clean range `f7da4ab8..07b1f874` contains 15 committed `.e` slices and changes 146 tracked files with
26,093 insertions and 3,756 deletions. The final 24-chain adapter population contains one ruleset hash, all 168
registered rule ids, and 148,708 cumulative claims: 31,382 SourceIR, 89,758 EvidenceIR, 12,399 SemanticIR,
14,288 IntentIR, and 881 adapter claims.

Schema and currentness agree at every stage: SourceIR is 24 schema-3 current / 54 schema-1 legacy; EvidenceIR is
24 schema-3 current / 54 schema-2 legacy; SemanticIR, IntentIR, and adapters are each 24 schema-2 current / 54
schema-1 legacy. The 24 document-key sets are identical. Exact proof-only migration compared 120 artifacts after
excluding only proof context, proof ledger, and validation reports; every remaining value, including all
residual decisions, was unchanged. Current residual surfaces contain zero SourceIR, three SemanticIR, eight
IntentIR, and 62 adapter objects, so the proof-migration residual delta is zero. All 24 current adapters are
blocked/no-file, and 54 chains remain explicitly proof-unmeasurable rather than receiving synthetic proof.

The detailed per-chain ledger/residual attribution and reproducible commands live in
[`production-genericity-structural-qualification.md`](production-genericity-structural-qualification.md).
Structural qualification is therefore complete. Population alpha-renaming, identity perturbation, paraphrase,
negative controls, held-out families, and full behavioral replay remain exclusively `.f`.

## Initial held-out behavioral diagnostic

Leaf `.6d.ii.f.iii` has executed all three eligible recipe-free relations across the frozen 17-document
prospective partition: 51 attempts and 90 relation/stratum rows. Unchanged PDF replay passes 17/17. Adversarial
identity is 0/17 at exactly the filename-derived `stable_artifact_stem` and its proof premise, exposing an
omitted comparator normalization inside the already-declared document-identity allowance. Symbol alpha is
0 pass / 15 fail / one unmeasurable / one invalid because the conformance transform admitted schema actor,
state, and similar meaning-bearing prose values as if they were opaque symbols. The I2C row alone has an
independently declared six-signal catalog; the other prospective rows cannot honestly supply this relation.

The initial artifact remains failed evidence in Git history. Its oracle defects were repaired narrowly. The
corrected result is 34 pass / one fail / 16 unmeasurable / zero invalid: unchanged and identity pass 17/17;
15 rows lack typed opaque alpha declarations; one is vacuous; and the one measurable I2C alpha pair fails after
a valid six-signal, 352-occurrence bijection. SourceIR remains invariant, but EvidenceIR loses two proof claims
and downstream SemanticIR, IntentIR, and ISF expose 12,441 undeclared paths.

The corrected checker reconstructs split, attempts, execution modes, retained-evidence provenance, aggregates,
dispositions, denominators, strata, and descriptive Wilson intervals; 17/17 controlled mutations reject. No
held-out label, outcome, interval, alias, or threshold entered production. The real coupling is now owned by
`.6d.ii.f.iii.a` before whole-population replay or final genericity decision.

## Historical correction

`PDF-AGNOSTIC-EXTRACTION.4` previously closed genericity using a small seeded check focused on
`signal_stop_words`, excluding comments/tests/fixtures. This audit disproves that repository-wide
claim: executable identity, protocol, signal, and corpus phrase decisions remained elsewhere. The old
evidence is still useful for the specific list it checked, but it cannot support whole-production
neutrality and must be annotated as superseded by this audit and the `.6d.ii` remediation tree.

## Exit criteria

Discovery and structural qualification are complete. The production-genericity parent remains open until:

- every P0/P1 item above is removed or relocated to conformance;
- the core compiles without corpus-specific public types/modules;
- the already-green AST/dependency mutation and per-rule structural-alpha gates remain green;
- alpha-renaming, identity perturbation, paraphrase, negative-control, held-out, full replay, and full
  repository gates pass;
- code, roadmap, task tree, live docs, Knowledge Map, and mdBook all state the same verified boundary.
