---
id: proof-carrying-genericity-kernel
title: Production genericity is enforced by opaque capabilities and a proof-carrying promotion kernel
date: 2026-08-12
status: accepted
scope: genericity, architecture, information-flow, proof-ledger, rule-registry, doctrine-enforcement
evidence: doctrine/production_genericity/module_inventory.tsv; doctrine/production_genericity/claim_family_inventory.tsv; doctrine/production_genericity/rule_family_inventory.tsv; doctrine/production_genericity/conformance_bypass_inventory.tsv; docs/research/production-genericity-pipeline-audit.md; crates/specforge-core/Cargo.toml; crates/specforge-conformance/Cargo.toml; scripts/check_production_genericity_dependencies.pl; scripts/check_production_genericity_rules.pl
reverify: perl scripts/check_production_genericity_inventory.pl && perl scripts/check_production_genericity_dependencies.pl && perl scripts/check_production_genericity_rules.pl && perl scripts/check_production_genericity_rules.pl --self-test
answers:
  - "What replaces a forbidden vocabulary list as the proof of production genericity?"
  - "What is SpecForge's trusted promotion kernel?"
  - "How will SpecForge enforce that identifiers are opaque?"
  - "How does a canonical IR claim prove where it came from?"
  - "What is the production genericity module denominator?"
  - "What is the canonical claim-family migration denominator?"
  - "Can old proofless IR artifacts still steer extraction or lower to ISF?"
  - "How are grammar rules admitted into production?"
  - "How does alpha-renaming become a per-rule obligation?"
  - "Why is an AST scan not sufficient by itself?"
---

# ADR 0038: Production genericity is enforced by opaque capabilities and a proof-carrying promotion kernel

## Context

ADR 0006 prohibits specification-, document-, vendor-, protocol-, and identifier-specific production
authority. The whole-pipeline audit found that a finite vocabulary census could locate known leaks but could
not establish the invariant: an unseen alias, path, hash, numeric selector, learned label, prompt bias, or
neutral-looking copied phrase can encode the same dependency.

The remediation through `.6d.ii.d` removes the known identity and spelling decisions. That is necessary but
not a signoff architecture. The current crate still has hundreds of direct string decision sites, public record
constructors, heterogeneous provenance fields, only a partial extractor registry, named production commentary,
and calibrated evaluation logic in the same module graph as canonical producers. A source scan cannot infer
which string operations are benign formatting and which create semantic authority.

The required property is information-flow noninterference: changing only input identity or the spelling of a
bound document symbol cannot change semantic admission, role, conflict, residual, validation, or lowering
eligibility. A source-derived name may flow to output as data; it may not flow to the control decision that says
what the name means.

The architecture combines three established ideas:

1. proof-carrying systems, where a producer supplies evidence that a small consumer-side kernel can check;
2. language-based information-flow control, where capabilities and types constrain which code can observe a
   sensitive value; and
3. metamorphic testing, where transformations with known semantic relations provide an oracle when no single
   expected output is practical.

None is sufficient alone. A proof ledger without opaque inputs can certify the wrong premise; opaque types
without proof terms cannot explain promotion; an AST scan without type boundaries is bypassable; metamorphic
tests sample behavior but do not prevent an untested branch.

## Exact migration denominator

Two machine-readable inventories freeze the pre-implementation denominator:

- `doctrine/production_genericity/module_inventory.tsv` lists all 71 Rust files below
  `crates/specforge/src/` exactly once and assigns each to core, application, conformance, provider, rule,
  lowering, learning, foundation, or test-support disposition plus one primary migration lane.
- `doctrine/production_genericity/claim_family_inventory.tsv` assigns all 168 top-level fields of `SourceIr`,
  `EvidenceIr`, `SemanticIr`, `IntentIr`, and `AdapterArtifact` exactly once across 38 claim families: 5 SourceIR,
  11 EvidenceIR, 11 SemanticIR, 7 IntentIR, and 4 adapter families.

These are structural inventories, not vocabulary denylists. A new module or top-level artifact field must be
classified before it can merge. Mixed and conformance rows explicitly own every compiled location that currently
contains named fixtures, reviewed replay, corpus presentation, calibrated classification, or named examples;
`.e.ii` must move those surfaces behind the one-way boundary rather than declaring them generic. The inventory
checker derives the live Rust and artifact-field denominators and rejects missing, duplicate, malformed, or stale
rows. Later `.e` gates compose that exact coverage with the dependency, capability, registry, and promotion checks.

## Decision

### 1. One-way crate dependency

The target graph is:

```text
specforge-core  <-  specforge-conformance
      ^                    ^
      |                    |
      +------ specforge application/CLI
```

`specforge-core` owns generic capture, IR schemas, rule registry, promotion kernel, prior proposals, validation
of proofs, residuals, and ISF lowering. `specforge-conformance` owns named fixtures, reviewed gold, replay,
trajectory, corpus pages, labeled evaluation, calibrated document classification, and named examples. The
application may expose commands from both. Core cannot name or depend on conformance. This must be a Cargo crate
edge, not only a directory convention.

Generic metric math may remain in or move to core only when it has no named dataset, calibrated corpus threshold,
or extraction authority. Mixed `eval`, `completeness`, and `validate` surfaces are split. Calibration can
characterize an output downstream; it cannot promote a canonical fact.

### 2. Opaque identity and symbol capabilities

Input-owned symbols are interned in source/provenance order:

```text
OpaqueSymbol = { document_scope, source_ordinal, private spelling }
```

Semantic code receives the stable scope/ordinal identity and exact-identity operations. It receives no `Deref`,
`AsRef<str>`, public spelling accessor, spelling-based `Ord`, or `Display`. Alpha-renaming changes only the
private spelling while preserving the identity graph and source ordinal.

Only four sealed capabilities may expose spelling:

- `GrammarCapability`: registered grammar reads raw current-document text and returns source-spanned proposals;
- `PersistenceCapability`: transparent serialization/deserialization without semantic branching;
- `PresentationCapability`: diagnostics and UI rendering without returning a decision input;
- `LoweringCapability`: target-safe symbol encoding after semantic roles are already proved.

Capability constructors are private to the trusted kernel. Application, conformance, provider, and ordinary
inference code cannot mint them. Document paths, keys, filenames, titles, and display names use a parallel opaque
identity type and have no grammar or inference capability.

### 3. Raw text is a typed input, not a ubiquitous string

Unparsed document text, table cells, captions, labels, and model output enter core as typed raw evidence. Only a
registered grammar may inspect them for semantics. A grammar returns `GroundedProposal<T>`, never a canonical
record directly. The proposal includes source spans/table cells/visual regions, copied symbol candidates, model
or prior provenance when applicable, and the registered rule id.

Universal literals are explicit axioms in the rule registry. Normative English, binary logic levels, document
layout roles, numeric syntax, and digital-design concepts may be registered when they are truly universal in the
declared domain. A rule cannot acquire legitimacy merely because its literal is absent from a denylist.

### 4. Small trusted promotion kernel

All canonical insertions pass one sealed `PromotionKernel`. The kernel checks a proof term and returns a
`Proved<T>` value. Artifact builders can serialize or lower only proved values; direct record construction is
insufficient authority.

The proof schema is an artifact-level ledger rather than a new provenance field copied inconsistently into every
record:

```text
ProofLedger {
  schema_version,
  ruleset_sha256,
  claims: [ClaimProof]
}

ClaimProof {
  address: { stage, surface, stable_record_key, optional_field_path },
  conclusion_sha256,
  rule_id,
  premises: [PremiseRef],
  symbol_uses: [SymbolUse],
  confidence,
}

PremiseRef =
  SourceSpan | TableCell | VisualRegion | UpstreamClaim |
  GroundedModelProposal | ValidatedPrior | UniversalAxiom
```

The kernel validates, at minimum:

- the rule exists at the ruleset hash and is allowed to conclude that stage/surface;
- every premise exists, belongs to the current artifact chain, and has an allowed kind;
- source/table/visual references resolve to captured current-document evidence;
- model proposals are independently grounded and priors satisfy their validation/scope contract;
- every input-owned symbol use is copied, exact-compared, or introduced from a cited source location under the
  rule's declared capability; no spelling-derived role is permitted;
- the conclusion digest matches the record and every promoted record has exactly one current proof address;
- carried claims cite an upstream claim and declare a lossless transform; conflicts, merges, and residuals cite
  all contributing premises rather than hiding disagreement.

This kernel checks derivation shape and evidence integrity. It does not pretend to mechanically prove that every
English grammar is linguistically complete. Rule review, alpha obligations, negative controls, and held-out
behavior qualify that layer.

### 5. Versioned rule registry

Every production grammar, inference, merge, conflict detector, carry, residual, and lowering transform has one
`RuleDescriptor`:

```text
RuleDescriptor {
  rule_id,
  version,
  implementation_module,
  premise_kinds,
  conclusion_stage,
  conclusion_surface,
  symbol_capability,
  alpha_obligation,
  compatibility,
}
```

Registry membership is closed and hashed. The existing `Extractor` framework is a useful substrate—25 current
`Extractor` implementations already expose stable names and tiers—but its run manifest is not a proof: it records
counts rather than a derivation for each kept record and covers only part of the pipeline. `.e.iv` extends or
adapts it; it does not create a second competing extractor framework.

Rules may call helpers, but only the registered entrypoint may return a grounded proposal or proved conclusion.
The registry declares whether a rule is symbol-blind, exact-identity-only, symbol-introducing grammar,
lossless-carry, merge/conflict, residual, or target-lowering. Each category has a fixed alpha obligation.

### 6. Alpha-equivariance is attached to each rule

Every rule declares an executable transformation contract. At minimum the harness can rename all symbols bound
in its premises with fresh adversarial spellings and compare normalized conclusions and proof topology. A rule
that introduces symbols must prove their origin spans transform consistently. A carry/lowering rule must preserve
claim topology modulo target-safe renaming. A symbol-blind rule must produce byte-identical non-symbol output.

Static registry completeness and per-rule obligations land in `.e.vi`; whole-document identity perturbation,
paraphrase, held-out families, and complete-population behavior remain `.f`. Passing local alpha obligations is
necessary but not a substitute for that later end-to-end qualification.

### 7. Structural doctrine, with AST analysis as defense in depth

The registered doctrine derives and checks:

- Cargo dependency direction and absence of core-to-conformance imports;
- exact module and claim-family inventory coverage;
- capability visibility and all call sites that expose raw text or symbol spelling;
- rule registry uniqueness, implementation paths, proof conclusion surfaces, and alpha declarations;
- canonical insertion/write/lower sites passing through the kernel;
- raw equality, substring, prefix/suffix, regex, hash, path, and numeric selection in promotion code occurring
  only inside registered rule implementations with allowed capabilities;
- macro definitions/invocations and parse failures as closed-world inputs: unexpanded or unclassified semantic
  construction fails the gate rather than being skipped.

The type and crate boundaries are the primary enforcement. AST/dependency analysis makes bypasses and suspicious
new decision sites reviewable. A generated vocabulary census scans comments, prompts, public identifiers, and
known corpus names only as supplementary diagnostics; it is never the proof authority.

### 8. Compatibility fails closed

Proof-carrying schema versions are explicit. A proofless historical artifact may be inspected or migrated as
captured legacy data, but it cannot feed a canonical downstream build or emit ISF. It must be rebuilt from its
retained upstream source/bundle with current registered rules. Unsupported future proof/rule/schema versions
reject.

Compatibility does not synthesize proofs around old claims. If retained source is insufficient to reproduce a
claim, the claim becomes a source-linked residual. ADR 0025 requires every affected retained chain to be measured,
rebuilt, compared, validated, and made current before the migrating slice commits.

### 9. Fail-closed mutation set

The doctrine is not accepted until controlled mutations prove it rejects:

- a core dependency on conformance;
- filename/title/document-key selection;
- signal spelling selection through equality, substring, regex, hash, sort order, or numeric tag;
- raw text interpretation outside a registered grammar;
- an unregistered inference rule or direct proofless record insertion;
- a forged or stale premise, ruleset hash, proof address, or conclusion digest;
- laundering an opaque symbol through serialization, formatting, a helper alias, or a macro;
- a protocol-specialized public schema; and
- a registered rule with no alpha obligation.

Legal capture, persistence, presentation, exact identity, target encoding, test fixtures, and conformance examples
must remain admitted through their narrow capability/module boundaries.

## Why this is the signoff direction

The design does not attempt an impossible list of every forbidden future word. It constrains what information a
semantic decision can consume and requires every promoted conclusion to carry a checkable derivation from typed
current-document evidence or an explicit universal axiom. It also makes the remaining uncertainty honest: a
proof can show why a claim was admitted, not that the source contained intent it never expressed.

This is stronger than an AST grep, a denylist, unit tests, prompts that say “be generic,” or provenance vectors
alone. It is also deliberately smaller than formal verification of all natural-language understanding. The
trusted kernel and rule metadata are mechanically checked; the rule semantics are qualified with metamorphic and
held-out behavior.

## Implementation status

`.6d.ii.e.ii` makes the core/conformance direction a Cargo boundary. `.6d.ii.e.iii` installs the accepted trusted
substrate in `crates/specforge/src/ir/derivation.rs`:

- source-order opaque document/symbol atoms expose only spelling-free identity and exact comparison;
- sealed grammar, persistence, presentation, and lowering tokens own all private spelling access;
- a deterministic SHA-256 ruleset covers descriptor version, rule ids, implementation modules, premise kinds,
  conclusion stage/surface, symbol capability, compatibility, alpha obligation, and universal axioms;
- schema-1 claim proofs cover typed premises, symbol-use modes, confidence, unique address, and conclusion digest;
- the kernel validates current capture, grounded model/prior attestations, upstream order, registry authority, and
  exact conclusions before issuing a non-deserializable `VerifiedProofLedger`; and
- persisted compatibility cannot grant authority from shape alone: legacy/stale rebuild or residualize,
  future/malformed reject, and a current envelope still requires full kernel verification.

This status is intentionally bounded. Existing artifact schemas and producer families do not yet carry the
ledger; `.e.iv` owns their exact migration and current-chain reconciliation. The implementation therefore proves
the kernel/capability contract, not whole-pipeline compliance. The live denominator is 77 modules / 38 claim
families / 168 fields, with 16 focused unit controls and five external compile-fail controls.

### Exact rule-migration contract (`.6d.ii.e.iv.i`)

The rule migration now has a checked, stage-owned graph before any artifact schema changes. The 38 claim-family
rows expand deterministically to one root rule for each of the current 168 top-level fields and resolve 113
current producer/mutator entrypoints plus 39 canonical insertion/write seams. A separate inventory owns four
conformance/evaluation entrypoints that temporarily patch SourceIR, EvidenceIR, or SemanticIR values. The checker
joins those declarations to the exact claim-family denominator, resolves every path and function, enforces the
closed premise/capability/alpha matrix, rejects duplicate rule ids, and requires every stage writer plus raw
validation backannotation to be named.

The migration uses one cumulative ledger for the complete pipeline:

1. SourceIR begins from exact normalized capture bytes and document scope. A classification proof cites the
   unclassified captured projection; it cannot cite the record it is classifying.
2. Each downstream stage verifies the complete upstream artifact and ledger, copies those verified claims as an
   exact ordered prefix, and appends only current-stage claims. Every upstream premise resolves to that prefix.
3. Every current top-level field has a root conclusion, including the empty case. Non-empty record collections
   additionally receive stable per-record addresses. The ledger carrier is verifier metadata, not a recursive
   semantic claim.
4. Canonical load, serialization, write, downstream build, and ISF lowering verify current schema, ruleset,
   artifact digests, complete field coverage, and ledger topology. A post-build mutation makes that verification
   stale until a registered rule extends the ledger.
5. Proofless history remains inspectable and may be rebuilt or residualized from retained evidence; it cannot be
   wrapped in synthetic proofs. Unsupported future proof, ruleset, or artifact schemas reject.

A repository-local temporary file is not a trust boundary. Conformance fixtures must become typed noncanonical
overlays that cannot call canonical writers, downstream production builders, or lowering. Evaluation-only storage
rebasing must preserve verified proof bytes and grant no semantic mutation. This prevents named test data from
acquiring production authority merely because it was written through an ordinary stage type.

This child freezes the reviewed high-level producer/mutator graph and compatibility design; it does not yet make
the 168 fields proof-carrying. Stage children `.e.iv.ii` through `.e.iv.vi` own that implementation. The later
AST/information-flow doctrine still derives raw decision-site and helper-call closure rather than trusting this
reviewed entrypoint list as the final static proof.

## Consequences

- Schema and artifact churn will be large. Every changed retained chain is governed by ADR 0025; recall loss is
  accepted when the old claim cannot earn a current proof.
- Public `String` identifiers and direct record construction must contract behind typed APIs. This is an
  intentional compatibility break inside the core; transparent persisted spelling remains available through
  versioned schema migration and presentation capabilities.
- Core/conformance separation may retain one user-facing binary, but it introduces real workspace crates so the
  forbidden dependency is compiler-visible.
- Provider output and learned priors remain useful as proposals, never self-authenticating truth.
- `.e.ii` and `.e.iii` implement the package and kernel foundations; `.e.iv`–`.e.vii` migrate, enforce, and
  structurally qualify them. `.f` still owns whole-population metamorphic and held-out signoff. The project must
  not claim production genericity before both parents close.

## Research basis

- George C. Necula, *Proof-Carrying Code*, POPL 1997: <https://doi.org/10.1145/263699.263712>
- Andrei Sabelfeld and Andrew C. Myers, *Language-Based Information-Flow Security*, IEEE JSAC 2003:
  <https://doi.org/10.1109/JSAC.2002.806121>
- Tsong Y. Chen, Shing C. Cheung, and Shiu Ming Yiu, *Metamorphic Testing: A New Approach for Generating Next
  Test Cases*, HKUST-CS98-01: <https://www.cse.ust.hk/~scc/publ/CS98-01-metamorphictesting.pdf>
- Rust Reference, *Visibility and privacy*: <https://doc.rust-lang.org/reference/visibility-and-privacy.html>

These sources motivate the composition; the concrete symbol interning, proof ledger, rule metadata, and
core/conformance migration are SpecForge's architecture for this problem.

## Links

- Genericity invariant: [`0006-no-hardcoded-chip-spec-vocabulary.md`](0006-no-hardcoded-chip-spec-vocabulary.md)
- Persisted-chain currency: [`0025-persisted-chain-currency-is-measured-not-assumed.md`](0025-persisted-chain-currency-is-measured-not-assumed.md)
- Generic EvidenceIR: [`0035-protocol-evidence-is-generic-and-document-derived.md`](0035-protocol-evidence-is-generic-and-document-derived.md)
- Identity-independent priors: [`0036-prior-memory-is-identity-independent.md`](0036-prior-memory-is-identity-independent.md)
- Opaque identifiers: [`0037-identifiers-are-opaque-and-one-way-grounded.md`](0037-identifiers-are-opaque-and-one-way-grounded.md)
- Task tree: [`SPEC-TO-INTENT-ALIGNMENT.md`](../tasks/SPEC-TO-INTENT-ALIGNMENT.md)
- Pipeline audit: [`production-genericity-pipeline-audit.md`](../research/production-genericity-pipeline-audit.md)
