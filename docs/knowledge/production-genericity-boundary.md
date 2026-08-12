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
date: 2026-08-12
status: current
tags: [genericity, extraction, architecture, doctrine]
evidence: docs/research/production-genericity-pipeline-audit.md; docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md; crates/specforge/src/ir/derivation.rs; crates/specforge-core/Cargo.toml; crates/specforge-conformance/Cargo.toml; scripts/check_production_genericity_dependencies.pl; scripts/check_production_genericity_inventory.pl
reverify: cargo test -p specforge-core derivation --offline && cargo test -p specforge-core --doc --offline && perl scripts/check_production_genericity_dependencies.pl && perl scripts/check_production_genericity_inventory.pl
---

A neutral SpecForge is feasible when universal digital-intent semantics are separated from opaque,
document-derived symbols and provenance-bearing proposals. It cannot promise perfect recovery from
missing or ambiguous source information; it must residualize that uncertainty. Genericity is proved
by production module/type boundaries, registered grammar access, and renaming/identity/paraphrase
invariance—not by a finite token denylist. The canonical audit and enforcement design are in
`docs/research/production-genericity-pipeline-audit.md`; ADR 0006 owns the invariant.

SourceIR is the first remediated production boundary. Schema 2 classifies only explicit generic
visual/section forms and typed table roles; operation, participant, filename, protocol, and symbol
spellings cannot grant a kind. Unsupported shapes remain `unknown`. Loading a schema-1 SourceIR
preserves its captured source structure and provenance but neutralizes old diagram/table/section
semantic labels until re-ingest, because those labels were produced by the retired corpus-calibrated
policy. Future schemas fail closed. A read-only 78-document replay measured the intentional re-ingest
delta as 2,293 diagram, 2,123 section, and 3,484 table label changes without mutating an artifact.

ADR 0025 then identified and rebuilt the exact ten affected replayable cascades. Current-binary equality is
restored for 24/24 measurable EvidenceIR and all 78 SemanticIR, IntentIR, and adapter chains; both renderable
ISFs are FSMGen-strict clean. The reconciliation deliberately accepts 194 removed table-derived timing guesses
and other weak label-driven facts. Those are inputs to later generic recovery work, never justification for a
document-specific exception.

EvidenceIR is the second remediated boundary. Schema 2 replaces fixed protocol operations, phase enums, and
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
fail-closed proposal controls pass; current-binary chain currency is 24/24 measurable EvidenceIR and
78/78 at every downstream stage. The expected honesty cost is preserved rather than hidden: constraints,
invariants, transactions, and renderable adapters fall where name-derived authority was removed, while
all 17 remaining ISFs pass FSMGen strict. This closes identity/spelling/prompt/corpus remediation only.
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
