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
  - "Is the generic production core physically separated from conformance code?"
  - "Which crate owns replay, completeness classification, and trajectory snapshots?"
  - "Can a test-only Rust edit stale production proof?"
  - "How are production implementation digests derived?"
  - "What changes a production implementation digest?"
  - "How is the compiled production Rust graph derived?"
  - "How does the genericity graph distinguish exact calls from compiler-resolved dispatch?"
  - "What syntax uncertainty makes the production genericity graph fail closed?"
  - "How is raw and identity information flow enforced across production helpers?"
  - "What closed registry defines the production information-flow boundary?"
  - "How does the structural analyzer enforce proof-only canonical promotion?"
  - "Which registered doctrine enforces the complete clean production-genericity boundary?"
  - "How are runtime alpha obligations, adversarial mutations, and legal raw-spelling uses structurally qualified?"
  - "What exact proof population and migration delta closed structural genericity qualification?"
  - "What evidence closes current production-genericity signoff?"
date: 2026-08-15
status: current
tags: [genericity, extraction, architecture, doctrine]
evidence: docs/research/production-genericity-pipeline-audit.md; docs/research/production-genericity-structural-qualification.md; docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md; crates/specforge-core/build.rs; crates/specforge-core/Cargo.toml; crates/specforge-conformance/Cargo.toml; doctrine/production_genericity/rule_family_inventory.tsv; doctrine/production_genericity/conformance_bypass_inventory.tsv; doctrine/production_genericity/information_flow_boundary.tsv; tools/production-genericity-graph/src/analyzer.rs; tools/production-genericity-graph/src/flow.rs; scripts/check_production_genericity.sh; scripts/check_production_genericity_dependencies.pl; scripts/check_production_genericity_graph.sh; scripts/check_production_genericity_flow.sh; scripts/check_production_genericity_inventory.pl; scripts/check_production_genericity_rules.pl; scripts/check_doctrines.sh
reverify: scripts/check_production_genericity.sh --self-test && cargo test -p specforge-production-graph --lib --offline && cargo test -p specforge-core production_semantic_digests_are_compiler_derived_stage_closures --offline
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
Structural proof enforcement and population-level metamorphic qualification were the remaining release blockers
at that checkpoint. Later `.6d.ii.e` and `.6d.ii.f` close both; the final bounded signoff is recorded below.

The `.e.ii` package split now makes the first part of that architecture executable. `specforge-core` owns the
generic production compilation unit and has no internal package dependency. `specforge-conformance` points one
way to core and owns `eval`, completeness/category characterization, source-to-intent evaluation/replay,
trajectory control, and the reviewed named snapshot composer. The application depends on both and re-exports the
existing public API paths. A five-control checker rejects direct or aliased reverse dependencies, a conformance-
to-application cycle, and oracle-module reinsertion; Cargo compilation independently verifies Rust visibility.
The live inventory was then 76 modules / 38 claim families / 168 fields. That checkpoint closed dependency
separation only; the following `.e.iii` slice installs the capability/kernel substrate without claiming producer
migration.

The opacity kernel, the cumulative ledger those layers presuppose, and the five completed stage
migrations are [[proof-carrying-stage-ledger]].

Implementation digests now have a production-semantic boundary. The compiler-visible core build roots each stage
at its canonical production registry, follows the selected verifier plus referenced stage-local production items
and exact import bindings, and includes the complete trusted derivation kernel. It removes comments, docs,
formatting, and test/conformance branches, including nested configured code. Missing or ambiguous roots fail the
build. Controlled mutants prove those inert edits preserve identity while a referenced helper-body or import
binding change changes the digest; runtime tests prove the generated closure is the one consumed by all five
registries. Exact proof-only migration changes no non-proof/non-validation bytes across 120 stage artifacts, and
chain replay remains 24 current / zero stale / 54 proof-unmeasurable.

This closes registered proof-relation identity, not whole-core genericity by itself. Current-binary
reconstruction still observes builder/lowering dependencies outside the registry-rooted digest. The following
AST information-flow and adversarial structural layers close that structural gap; population behavior remains
open work.

The production syntax substrate derives the complete current graph. Four Cargo targets reach 78 of 79 inventoried
files; the remaining application test-support file is explicit. The modules contain deterministic item, import/
alias, call, local/external/builtin/attribute-macro nodes and distinguish exact calls from conservative compiler-
resolved dispatch. Missing inventory, unknown configuration, absent/ambiguous modules, duplicate items/aliases,
parse failure, opaque syntax, and non-relative output fail closed. The tool is disconnected from product crates.

The flow layer joins a closed 141-row typed registry to that graph. It resolves raw/identity sources, grammar and
exact-identity declassifiers, proof gates/values, trusted/non-authoritative regions, and protected types. Its fixed
point covers 2,359 functions, 14,629 helper edges, 12,618 decision sites, 1,462 semantic macros, and canonical-
seam topology; sensitive unresolved macros reject. Thirteen mutations challenge identity/literal/substring/regex
decisions, cross-class inference, authority forgery, laundering, proofless persistence, and registry duplication.
The clean fixture admits display, provenance capture, and test-only uses only when they grant no authority.

The registry names Rust structure, never a document/vendor/protocol vocabulary; Cargo/privacy, executable replay,
and the AST remain distinct type, semantic, and topology authorities. Closing full CI on 2026-08-16 caught the
Rust test's stale 78-module/140-row snapshot; seven corrected expectations and this card's replay now check the
current derived graph and independent crate oracle together.

`scripts/check_production_genericity.sh` composes dependency, inventory, rule, graph, and flow checks under the
unconditional doctrine. Its 27 controlled mutations cover those layers; every test-only `RuleDescriptor` runs
its alpha obligation, and the independent oracle joins all 168 runtime rules to their structural contract. The
conformance harness closes identity, alpha, paraphrase/layout, and negative sensitivity.

Final qualification covers `f7da4ab8..07b1f874`: 24 keys are current and 54 unmeasurable; 168 rules cover
148,708 claims, 120 exact comparisons, and exact 0/3/8/62 residual deltas. All 24 adapters are blocked/zero-file.

Behavioral `.f` closes at 35 pass / 0 fail / 16 unmeasurable / 0 invalid and 39/0/1 IntentIR TP/FP/FN with
42/42 provenance, 117/118 conservation, zero fabrication, and one APB loss. Production is neutral only within
this governed boundary; unmeasurable alpha strata, incomplete coverage/accounting, blocked adapters, history,
and `.7` remain explicit. See `docs/research/production-genericity-structural-qualification.md`.
