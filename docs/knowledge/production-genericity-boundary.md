---
id: production-genericity-boundary
title: Production genericity is structural and behaviorally invariant
answers:
  - "Can SpecForge theoretically be specification and PDF neutral?"
  - "What does spec-neutral extraction mean?"
  - "Why is a forbidden vocabulary list insufficient?"
  - "Where is the whole extraction-pipeline genericity audit?"
  - "Why was PDF-AGNOSTIC-EXTRACTION closure superseded?"
  - "How does SourceIR classification remain specification neutral?"
  - "What happens when schema-1 SourceIR is loaded?"
  - "Why were legacy SourceIR classifications neutralized?"
  - "Were downstream artifacts reconciled after SourceIR schema 2?"
  - "How does EvidenceIR schema 2 remove protocol-specific extraction authority?"
  - "How are production prompts kept specification neutral?"
  - "How does the corpus knowledge base group fixtures without protocol-family names?"
  - "Can fixture names decide KG capability or prior-candidate routing?"
  - "What exact range qualified the identity and spelling remediation?"
  - "Is SpecForge production-genericity signoff complete after identity remediation?"
  - "Is the generic production core physically separated from conformance code?"
  - "Can specforge-core depend on named fixtures or reviewed evaluation?"
  - "Which crate owns replay, completeness classification, and trajectory snapshots?"
  - "How does SpecForge keep source-owned symbol spelling opaque?"
  - "Can a deserialized proof ledger authorize a canonical claim?"
  - "Which premise kinds can the trusted promotion kernel accept?"
  - "How many production rule families, producer or mutator entrypoints, and canonical seams must migrate?"
  - "Can a temporary conformance artifact use a canonical stage writer?"
  - "How does the proof ledger continue from SourceIR through ISF lowering?"
  - "How is SourceIR proof-carrying?"
  - "Can recomputing SourceIR JSON hashes self-attest an edited claim?"
  - "Why can legacy SourceIR not feed EvidenceIR?"
  - "How are named SourceIR fixtures kept out of canonical production authority?"
  - "How is EvidenceIR proof-carrying?"
  - "Can a model proposal authorize EvidenceIR by itself?"
  - "How are EvidenceIR mutations authorized after extraction?"
  - "Why was EvidenceIR carry-forward removed?"
  - "Can legacy EvidenceIR feed SemanticIR?"
  - "Why can downstream chain currency be unmeasurable rather than stale?"
date: 2026-08-13
status: current
tags: [genericity, extraction, architecture, doctrine]
evidence: docs/research/production-genericity-pipeline-audit.md; docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md; crates/specforge/src/ir/derivation.rs; crates/specforge/src/ir/source.rs; crates/specforge/src/ir/evidence.rs; crates/specforge-core/Cargo.toml; crates/specforge-conformance/Cargo.toml; doctrine/production_genericity/rule_family_inventory.tsv; doctrine/production_genericity/conformance_bypass_inventory.tsv; scripts/check_production_genericity_dependencies.pl; scripts/check_production_genericity_inventory.pl; scripts/check_production_genericity_rules.pl; scripts/check_chain_currency.sh
reverify: cargo test -p specforge-core evidence_proof --offline && cargo test -p specforge-core derivation --offline && perl scripts/check_production_genericity_rules.pl && bash scripts/check_chain_currency.sh --check
---

A neutral SpecForge is feasible when universal digital-intent semantics are separated from opaque,
document-derived symbols and provenance-bearing proposals. It cannot promise perfect recovery from
missing or ambiguous source information; it must residualize that uncertainty. Genericity is proved
by production module/type boundaries, registered grammar access, and renaming/identity/paraphrase
invariance—not by a finite token denylist. The canonical audit and enforcement design are in
`docs/research/production-genericity-pipeline-audit.md`; ADR 0006 owns the invariant.

SourceIR was the first remediated production boundary. Its schema-2 classifier established that only explicit generic
visual/section forms and typed table roles; operation, participant, filename, protocol, and symbol
spellings cannot grant a kind. Unsupported shapes remain `unknown`. Loading a schema-1 SourceIR
preserves its captured source structure and provenance but neutralizes old diagram/table/section
semantic labels until re-ingest, because those labels were produced by the retired corpus-calibrated
policy. Future schemas fail closed. A read-only 78-document replay measured the intentional re-ingest
delta as 2,293 diagram, 2,123 section, and 3,484 table label changes without mutating an artifact.

ADR 0025 then identified and rebuilt the exact affected replayable cascades. That reconciliation deliberately
accepted removed table-derived timing guesses and other weak label-driven facts. Those are inputs to later generic
recovery work, never justification for a document-specific exception. The later proof migration intentionally
supersedes the older 78-chain currency claim with a stricter proof frontier described below.

EvidenceIR's schema-2 remediation replaced fixed protocol operations, phase enums, and
participant-direction roles with input-derived generic records. Frame, operation, state, and direction
extractors admit only explicit structural grammar and preserve document terms opaquely. Schema-1 loads clear
the old vocabulary-bound protocol surfaces before typed deserialization, preventing a retained artifact from
bypassing the new producer. The carriers remain lossless through SemanticIR, IntentIR, convergence,
validation, evaluation, and adapter residual accounting.

Production model prompts use the same boundary. They describe typed digital-hardware relations, constraints,
contracts, conditions, entity types, diagrams, tables, register fields, audits, and entailment. Relation,
constraint, and contract prompts consume a current-document declaration catalog in declaration/provenance order;
an undeclared contract signal is forced to an explicit residual. Entity typing is stronger: the identifier is
redacted from both model context and helper transport, so only typed evidence and grammar remain. Image prompts
use visible structure while treating labels and values as opaque. No real vendor/protocol signal example or
all-inputs-are-one-protocol framing remains. Alpha controls require the policy to be identical modulo injected
symbols; opaque catalogs are never sorted by spelling because renaming could otherwise perturb prompt order.

Corpus-KB organization is structural too. `kg_bench` derives capability and prior-surface metadata only from
which typed fixture-schema fields are populated; string values, fixture names, source names, vendor names, and
protocol names cannot grant membership. `corpus_kb` selects seven capability pages from those facets. The old
AMBA page and fixture-name substring classifier are gone. Manifest schema 2 reports `prior_present_fixtures` and
`control_fixtures`; those are structural observations, not a claim inferred from “gold” or “negative” spelling.
Exact fixture names remain visible only as conformance provenance.

The combined `.6d.ii.d` qualification compares `89d8dee7..9c38b569`: three commits, 129 changed
files, and 24 compiled-crate Rust source files. Focused alpha, identity, prompt, spelling, fixture-name, and
fail-closed proposal controls pass. The expected honesty cost is preserved rather than hidden: constraints,
invariants, transactions, and renderable adapters fall where name-derived authority was removed. This closes
identity/spelling/prompt/corpus remediation only.
Structural proof enforcement and population-level metamorphic qualification remain release blockers in
`.6d.ii.e` and `.6d.ii.f`; SpecForge is not yet whole-core genericity signoff-ready.

The `.e.ii` package split now makes the first part of that architecture executable. `specforge-core` owns the
generic production compilation unit and has no internal package dependency. `specforge-conformance` points one
way to core and owns `eval`, completeness/category characterization, source-to-intent evaluation/replay,
trajectory control, and the reviewed named snapshot composer. The application depends on both and re-exports the
existing public API paths. A five-control checker rejects direct or aliased reverse dependencies, a conformance-
to-application cycle, and oracle-module reinsertion; Cargo compilation independently verifies Rust visibility.
The live inventory was then 76 modules / 38 claim families / 168 fields. That checkpoint closed dependency
separation only; the following `.e.iii` slice installs the capability/kernel substrate without claiming producer
migration.

The `.e.iii` substrate makes opacity and proof admission executable. A source-owned symbol is a private spelling
plus a public scope/ordinal identity; ordinary code cannot display, serialize, string-convert, or spelling-order
it. Sealed capabilities delimit grammar, persistence, presentation, and lowering access. A closed SHA-256 ruleset
and schema-1 claim ledger describe typed capture/model/prior/upstream/axiom premises, allowed spelling-free symbol
uses, and exact conclusion bytes. The kernel is the only proposal-to-proof transition and returns an unforgeable
in-memory verification witness. A deserialized ledger—even at the current schema and ruleset—still requires the
current evidence/conclusion check; old/stale input rebuilds or residualizes, while malformed/future input rejects.
The live inventory is now 77 modules / 38 families / 168 fields. Existing producers remain proofless until the
exact `.e.iv` migration, so whole-core signoff remains open.

The first `.e.iv` child freezes the exact reviewed migration graph before a stage schema changes. Its 38 rows
expand to 168 field-root rules and resolve 111 current producer/mutator entrypoints, 39 canonical seams, and four
conformance-only mutation bypasses. The target is one cumulative ledger: each stage verifies and preserves the
exact upstream proof prefix, then appends its own field-root and per-record claims. Canonical load, serialization,
write, downstream build, and ISF lowering all require a current complete ledger; an unregistered mutation makes
the artifact stale. Proofless history can be inspected, rebuilt, or residualized but cannot gain synthetic proof.
A temporary conformance path grants no authority: fixture edits must use noncanonical overlays that cannot call a
canonical writer/build/lowering seam. This census is the high-level migration contract, not the final AST proof;
the later structural doctrine must still derive helper-level decision closure mechanically.

SourceIR schema 3 is the first completed stage migration. Its 19 fields are covered across envelope/integrity,
capture, classification, residual, and validation/evaluation families. The private proof context contains exact
neutral capture bytes and exact grounded proposal payloads; each registered descriptor is paired with executable
Rust verification and an implementation digest. Canonical load and every write/downstream seam re-execute the
relation against exact premise and conclusion bytes, so recomputing a JSON conclusion hash cannot self-attest an
edit. Table and visual records carry real typed `TableCell`/`VisualRegion` premises. Classification must replay
from an `unknown` capture, and a grounded model refinement must cite its exact response plus direct typed target.

Compatibility remains fail-closed. Current proofless, stale, malformed, and future SourceIR cannot feed
EvidenceIR. Legacy SourceIR can only load through the inspection API, which neutralizes its old semantic labels.
An audited maintenance feature migrated the exact 24 documents whose source and normalized capture bundles were
still retained; the other 54 remain inspection-only. Named conformance fixtures now use typed noncanonical
overlays with no canonical writer, while each production writer independently reloads its upstream canonical
artifact.

EvidenceIR schema 3 is the second completed stage migration. It verifies and retains the complete SourceIR
ledger as an exact ordered prefix, then proves all 39 public fields across 11 families with field-root and
per-record or per-alias claims. Its private replay context owns the exact normalized Markdown, the exact validated
prior when consulted, and an ordered list of closed typed post-build proposals. A proposal records what a bounded
extractor or model-assisted command changed and the existing current-document support it cited; it does not
self-attest semantic truth. The current registered stage derivation must reconstruct every conclusion exactly.

Post-build mutation is closed by kind and allowed field set for NLP enrichment, condition and constraint
extraction, contract extraction, signal resolution, register-bit recovery, NLI measurement, and validation
backannotation. Each mutation rebuilds the verified predecessor, applies the exact patch, and extends proof before
writing. The old path that copied selected state from an older EvidenceIR without recording a derivation is gone.
Schemas 1 and 2 are inspection-only and cannot feed SemanticIR.

Exactly 24 retained documents have both verifiable SourceIR capture and current schema-3 EvidenceIR; their
extracted fields and SemanticIR replay are unchanged by this migration. The other 54 remain behind a deliberate
legacy/proofless frontier until owned source recapture is available. `CHAIN-CURRENCY` reports only that closed
condition as unmeasurable. It still fails any stale current proof, so the distinction neither claims the historical
EvidenceIR-to-SemanticIR edge is current nor creates a bypass. IntentIR and adapters remain 78/78 stage-locally
reproducible from persisted SemanticIR, but that is not end-to-end proof authority. SemanticIR, IntentIR, adapter,
AST-aware information-flow, and population qualification remain open work.
