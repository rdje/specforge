# Extraction Architecture Contract

This chapter is the durable goal contract for extracting implementation-relevant intent from chip,
protocol, component, system, and software-interface specifications. It defines what the pipeline must
preserve and recover. It does not mirror delivery status; current work belongs in the roadmap and
task trees, while shipped behavior belongs in the topical chapters of this book.

## Evidence is multimodal

A specification distributes meaning across six evidence modalities. A complete extraction design
must account for all six:

1. **Structured tables** — signal inventories, encodings, register maps, timing parameters, and
   feature/capability matrices.
2. **Normative prose** — obligations, conditions, signal-value constraints, timing language, and
   other behavioral requirements.
3. **Timing diagrams** — signal values, edges, cycles, setup/hold annotations, and handshake timing.
4. **State and flow diagrams** — states, transitions, guards, initial/final markers, and topology.
5. **Section structure** — hierarchy and local context that distinguish normative material, signal
   descriptions, appendices, glossary material, and boilerplate.
6. **Document metadata** — source identity, revision, page/asset identity, parser provenance, and
   traceability information.

No one modality is a substitute for the others. Tables are often strongest for inventory and shape;
prose is often strongest for relations and obligations; diagrams may be the most precise timing or
state evidence. The pipeline must preserve their provenance and arbitrate them explicitly.

## Stage obligations

The staged model is normative:

`SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters`

- `SourceIR` is a loss-minimizing capture layer. It preserves normalized source structure, page and
  visual assets, structured tables, content order, metadata, and backend provenance.
- `EvidenceIR` records what the source says and where it says it. It may classify and extract typed
  observations, but it must keep source links and competing evidence visible.
- `SemanticIR` lifts grounded evidence into backend-neutral actors, interfaces, connectivity,
  constraints, state, timing, and other typed meaning. It must not silently re-invent evidence that
  an earlier stage discarded.
- `IntentIR` is the canonical product surface. It packages the strongest justified implementation
  intent without erasing conflicts, assumptions, or residual decisions.
- Adapters lower canonical intent; they do not author missing source semantics.

The individual [pipeline chapters](../pipeline/overview.md) define the current record surfaces and
operational behavior. [Multimodal Evidence And Visual Grounding](../pipeline/multimodal-evidence.md)
defines the current visual path, while the domain chapters define graph and temporal lifting.

## Extraction rules

- Preserve before interpreting. Markdown is useful, but it must not replace meaningful page,
  geometry, table, image, caption, or backend-native evidence.
- Prefer structured extraction over classified strings. Recognizing that a sentence is normative is
  only routing; the goal is a typed constraint with grounded subjects, values, conditions, and
  provenance.
- Keep document truth separate from cross-document experience. Priors can guide local attention or
  arbitration, but cannot become canonical facts without current-document support.
- Keep AI bounded. Models may enrich or propose; grounding, schema checks, arbitration, and
  validation decide what survives.
- Preserve uncertainty. Conflicts and unresolved choices become typed residual decision packets,
  never silent guesses.
- Remain specification-independent. Extraction logic may encode grammar and hardware concepts, but
  must not memorize one protocol's actor, signal, state, or value vocabulary.

## Production genericity and proof status

SpecForge is deliberately specialized for digital-chip design, not for arbitrary document domains. Signals,
registers, fields, actors, states, events, timing, obligations, protocols, ISAs, and implementation relations are
legitimate domain concepts. The neutrality boundary is across document instances: vendor, family, filename,
title, corpus membership, and the spelling of a source-owned identifier are data and provenance, never evidence
of what the document means.

The accepted signoff architecture enforces that distinction structurally:

- a compiler-visible generic core cannot depend on conformance fixtures, named examples, reviewed gold, replay,
  corpus presentation, or calibrated document classifiers;
- document and symbol identities become opaque values. Ordinary semantic code can preserve or exact-compare
  identity but cannot inspect spelling; sealed grammar, persistence, presentation, and lowering capabilities
  expose only the operations their boundary needs;
- a registered grammar may inspect current-document raw evidence and emit a grounded proposal. Models and learned
  priors likewise propose with provenance; none can construct canonical truth directly;
- one small promotion kernel checks typed premises and returns proved claims. An artifact-level proof ledger binds
  every promoted record to the current ruleset, source/table/visual or upstream premises, rule id, symbol uses,
  and conclusion digest; and
- every registered grammar, inference, carry, conflict, residual, and lowering rule declares an alpha-renaming
  obligation. Static dependency/type/AST checks and controlled mutations complement that per-rule contract.

This is not a finite forbidden-word policy. A name list cannot detect a hidden alias or neutral-looking encoded
selector, and a listed term may be legitimate input evidence. The enforceable property is information flow and
checkable derivation: changing only identity or bound-symbol spelling cannot change semantic admission or
lowering eligibility.

Current status: the one-way package boundary and sealed derivation substrate are shipped. `specforge-core`
compiles the generic capture, IR, extraction, semantic, intent, and adapter surfaces without a dependency on
`specforge-conformance`;
`specforge-conformance` depends on core and owns evaluation, completeness characterization, replay, trajectory,
reviewed snapshots, and named fixtures. The `specforge` application composes both and preserves the existing public
module paths. Dependency mutations prove that a reverse edge, application cycle, or oracle-module reinsertion is
rejected. The frozen pre-migration denominator remains 71 modules; the live post-split inventory is 76 modules,
38 claim families, and 168 top-level fields. The added kernel makes the live inventory 77 modules while preserving
the same claim/field denominator.

The kernel's compiler-visible contract is now concrete. Source-order opaque symbol and document atoms expose only
scope/ordinal identity and exact comparison; they have no display, string-conversion, spelling-order, or Serde
surface. Sealed grammar, persistence, presentation, and lowering tokens control private spelling access. A closed,
SHA-256-hashed rule registry declares allowed premise kinds, conclusion stage/surface, symbol capability, and a
mandatory alpha-obligation category. Promotion checks exact captured source spans, table cells, visual regions,
grounded model attestations, validated scoped priors, registered universal axioms, ordered upstream claims,
spelling-free symbol uses, unique claim addresses, and conclusion digests. It returns a non-deserializable
verification witness; a merely schema-valid persisted ledger still has no canonical authority.

This is still not whole-core genericity signoff, but all five stage migrations are now concrete.
SourceIR schema 3 executes all five source-family derivations. EvidenceIR schema 3 verifies that exact source
ledger as its ordered prefix, then executes all 11 EvidenceIR families from exact current-document capture,
optional validated prior, and ordered typed enrichment proposals. Historical claims are never wrapped in a
synthetic proof. SemanticIR schema 2 verifies that exact cumulative EvidenceIR prefix and executes all 12
semantic carry, merge, conflict, synthesis, residual, and validation families. IntentIR schema 2 verifies the
cumulative SemanticIR prefix and executes all nine product families: 30 exact carried fields receive direct
SemanticIR antecedents, while filtered actors, conservatively NLI-filterable contracts, synthesized products,
relations, identities, residuals, and validation use their registered projection or mutation authority. Adapter
schema 2 preserves that complete IntentIR ledger and proves four local families covering all 12 fields, every
populated array record, every nonblank rendered ISF line, and every blocking reason. Canonical build, load,
serialization, write, emitted-file reconciliation, and closed validation mutation execute current replay.
AST/information-flow gates, per-rule
alpha execution, structural qualification, and whole-population behavioral qualification, remain mandatory.
Until those close, SpecForge does not claim production-genericity signoff.

Implementation digests now follow the registered production relation rather than whole Rust modules. The core
build roots each stage at its canonical production registry and hashes the selected verifier, recursively
referenced stage-local production items, exact import bindings, and the complete trusted derivation kernel after
removing comments, docs, formatting, and test/conformance branches. Controlled mutants prove inert edits preserve
identity and referenced production edits change it; missing or ambiguous roots fail compilation. The exact
proof-only migration changes no non-proof/non-validation content across 120 artifacts and leaves the measurable
chain current. Current-binary replay remains the behavioral guard outside that registry-rooted relation, while
the pending AST doctrine owns the broader helper/raw-text/identity-flow boundary.

That migration now has an exact checked contract. The 41 claim families expand to 168 current top-level-field
rules and cover 116 reviewed producer/mutator entrypoints plus 53 canonical seams. One cumulative ledger starts
from exact SourceIR capture, is verified and retained as an ordered prefix at every downstream stage, and gains
field-root proofs (including empty fields) plus stable per-record proofs. Canonical load, serialization, write,
downstream build, and ISF lowering require a complete current ledger; post-build mutation invalidates authority
until a registered rule proves the change. Historical proofless artifacts remain inspectable but must rebuild or
residualize rather than receive synthetic proofs.

Conformance fixtures do not receive an exception. SourceIR, EvidenceIR, and SemanticIR fixture patches use
noncanonical typed overlays or a compiled test-only closed mutation that cannot create production authority; the
same closure remains required for later stages. A temporary filename or repository-local location is storage,
never evidence of trust. The checked census is the integration denominator, while the stage-specific ledgers are
the executable authority.

## Target quality

A high-quality result should expose, when the source supports them:

- complete signal and field inventories with direction, width, role, and source support;
- actor-relative producer/consumer connectivity rather than perspective-free direction guesses;
- symbol/encoding, register, message-field, feature, and capability structure;
- explicit clock/reset and infrastructure semantics;
- typed state, transition, temporal, handshake, and other behavioral constraints;
- source-backed visual and cross-modality evidence;
- conflicts, coverage gaps, assumptions, and residual decisions that remain unresolved; and
- enough backend-independent structure for an adapter to lower without inventing semantics.

This target is deliberately stronger than a score or one protocol fixture. Validation and corpus
benchmarks measure progress; they do not redefine the architecture.

## Completion order

The specification-to-executable-intent endpoint does not make adapter syntax the first problem. SpecForge
first accounts for the PDF's relevant modalities, carries source-grounded semantics through canonical
`IntentIR`, and makes every loss or unresolved interpretation explicit. Only then can an adapter result prove
that ISF lacks a required abstraction. ISF/FSMGen is actively evolvable, but language work is triggered by a
real, reviewed IntentIR value—not by a predicted future need.

Program convergence is likewise not inferred from a stable pipeline or a valid output file. The
[trajectory contract](../quality/trajectory.md) measures source capture, semantic correctness/completeness,
stage conservation, honesty, production-path participation, robustness, and eventual executable behavior as
separate dimensions.

## Authority routes

- [Architecture Rationale](../architecture-rationale.md) explains why the design takes this shape.
- [IntentIR Product Contract](intentir-contract.md) defines the canonical output boundary.
- [Validation And Learning](../quality/validation.md) explains current quality signals.
- [Current roadmap](../../../../ROADMAP.md) and the [task-tree catalog](../../../TASK_TREE.md) own
  implementation direction and delivery state.
