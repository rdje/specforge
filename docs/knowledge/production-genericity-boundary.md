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
  - "How is SemanticIR proof-carrying?"
  - "Does every carried SemanticIR record cite EvidenceIR directly?"
  - "Can recomputing a SemanticIR conclusion hash authorize an edit?"
  - "Can legacy or proofless SemanticIR feed IntentIR?"
  - "How is SemanticIR validation backannotation authorized?"
  - "How is IntentIR proof-carrying?"
  - "How many IntentIR fields and rule families are proved?"
  - "Which IntentIR fields are exact SemanticIR carries?"
  - "Why are actors and actor contracts IntentIR projections rather than lossless carries?"
  - "Can recomputing an IntentIR conclusion hash authorize an edit?"
  - "Can legacy or proofless IntentIR feed an adapter?"
  - "How is IntentIR validation backannotation authorized?"
  - "How is NLI contract demotion authorized without permitting contract invention?"
  - "Does moving the repository invalidate IntentIR proof?"
  - "Why can a test-only Rust edit currently stale production proof?"
  - "Which task owns production-semantic implementation digest scoping?"
date: 2026-08-13
status: current
tags: [genericity, extraction, architecture, doctrine]
evidence: docs/research/production-genericity-pipeline-audit.md; docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md; crates/specforge/src/ir/derivation.rs; crates/specforge/src/ir/source.rs; crates/specforge/src/ir/evidence.rs; crates/specforge-core/Cargo.toml; crates/specforge-conformance/Cargo.toml; doctrine/production_genericity/rule_family_inventory.tsv; doctrine/production_genericity/conformance_bypass_inventory.tsv; scripts/check_production_genericity_dependencies.pl; scripts/check_production_genericity_inventory.pl; scripts/check_production_genericity_rules.pl; scripts/check_chain_currency.sh
reverify: cargo test -p specforge-core intent_proof --offline && cargo test -p specforge-core derivation --offline && perl scripts/check_production_genericity_rules.pl && bash scripts/check_chain_currency.sh --check
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
The live inventory is now 77 modules / 39 families / 168 fields. The SemanticIR migration split the original
carry row after executable proof showed that three surfaces are filtered or visually extended projections.
Existing later-stage producers remain proofless until the
exact `.e.iv` migration, so whole-core signoff remains open.

The first `.e.iv` child froze the exact reviewed migration graph before a stage schema changed. Its current 41 rows
expand to 168 field-root rules and resolve 116 producer/mutator entrypoints, 47 canonical seams, and four
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
EvidenceIR-to-SemanticIR edge is current nor creates a bypass. The later SemanticIR and IntentIR proof migrations
preserve the same 24-current / 54-unmeasurable frontier; historical later-stage bytes do not restore authority.

SemanticIR schema 2 is the third completed stage migration. It retains the verified cumulative EvidenceIR ledger
as an exact prefix and appends root plus per-record claims for 49 fields across 12 semantic families. Every
semantic conclusion depends on an exact registered replay whose stage-level dependency node includes every
upstream claim. Carried collection roots additionally cite the corresponding EvidenceIR root, and every carried
record cites an EvidenceIR conclusion with identical bytes. Timing constraints, signal constraints, and
conditional rules instead use a registered projection family because SemanticIR can filter or visually extend
those collections; they cannot claim lossless-carry authority. This preserves direct lossless-carry evidence while
keeping synthesis/conflict/residual contributor topology linear rather than quadratic.

Canonical verification independently rebuilds SemanticIR from current verified EvidenceIR and the exact captured
identity-independent prior, if one was consulted. Editing a field and recomputing its stored conclusion hash does
not change the registered replay and is rejected. Validation may mutate only `validation_reports` through a
closed typed mutation that replays the predecessor and extends proof. Current proofless or stale artifacts cannot
load, serialize, persist, or feed IntentIR; schemas older than 2 are inspection-only.

Exactly 24 retained chains could migrate because they have current EvidenceIR. Their semantic fields are
unchanged; only schema/proof metadata is added. The other 54 remain explicitly proof-unmeasurable. Currency is
24 current / 54 unmeasurable through EvidenceIR, SemanticIR, and IntentIR. The IntentIR proof migration below
supersedes the old later-stage-local result; adapter proof migration remains open.

IntentIR schema 2 is the fourth completed stage migration. It retains the complete verified cumulative SemanticIR
ledger as an exact ordered prefix, then appends root and per-record claims for all 49 public fields across nine
capability-homogeneous families. Every claim depends on exact registered replay over the complete upstream graph.
Thirty fields are true byte-for-byte SemanticIR carries: eleven actor/interface graph fields and nineteen typed
semantic collections. Their roots and records additionally cite direct SemanticIR claims from the immediately
upstream stage. The filtered `actors` surface has its own projection family, preventing removal of unsupported
phantoms from masquerading as lossless carry.

`actor_contracts` is also a projection family even though ordinary construction begins as an exact clone. The
optional NLI gate may conservatively remove a contract, so a post-gate collection root is no longer byte-identical
to SemanticIR. Its closed mutation is demotion-only: kept contracts preserve order, existing residuals remain an
exact prefix, and each removed contract produces exactly one residual with the typed NLI id. Addition, reorder,
unmatched residuals, or changes to any other field reject. Validation independently owns only
`validation_reports`. Both mutations rebuild their predecessor and extend the cumulative proof before canonical
serialization or writing.

Canonical verification independently rebuilds IntentIR from current verified SemanticIR, normalizes
repository-owned paths before comparison, and executes every registered relation. A recomputed JSON digest cannot
self-authorize an edit. Current proofless, stale, or unauthorized artifacts reject; schemas older than 2 are
inspection-only and cannot feed adapters; future schemas reject. Exactly 24 reachable IntentIRs migrated with
zero pre-existing public-field change. The other 54 remain explicitly proof-unmeasurable behind legacy
SemanticIR, and only the 24 verified product artifacts may build fresh adapters.

Current implementation digests have a safe but over-broad boundary: each migrated stage hashes its whole Rust
module. A change confined to `#[cfg(test)]` code in `intent.rs` therefore staled all 24 current IntentIR proofs;
canonical adapter loading rejected every artifact, proving fail-closure. Exact regeneration from verified
SemanticIR changed zero non-validation IntentIR public fields and zero non-validation adapter fields. Leaf
`.6d.ii.e.iv.vii` owns the stronger target: derive digests from compiled production proof semantics and their
dependency closure, so tests/comments do not create false staleness while a production relation change still
invalidates proof.

AST-aware information-flow and population qualification also remain open work.
