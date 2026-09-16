---
id: proof-carrying-stage-ledger
title: Canonical stage artifacts carry one cumulative five-stage proof ledger
answers:
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
  - "How is ISF adapter lowering proof-carrying?"
  - "Does every rendered ISF line have lowering authority?"
  - "Can a blocked adapter be a proof-checked result with no emitted file?"
  - "Can a legacy or proofless adapter emit ISF?"
  - "How is adapter validation backannotation authorized?"
date: 2026-08-15
status: current
tags: [proof, genericity, pipeline, architecture, doctrine]
evidence: docs/research/production-genericity-pipeline-audit.md; docs/research/production-genericity-structural-qualification.md; crates/specforge/src/ir/derivation.rs; crates/specforge/src/ir/source.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; crates/specforge/src/ir/adapters.rs; crates/specforge-core/build.rs; doctrine/production_genericity/rule_family_inventory.tsv; doctrine/production_genericity/conformance_bypass_inventory.tsv; scripts/check_chain_currency.sh; scripts/check_doctrines.sh
reverify: cargo test -p specforge-core derivation --offline && bash scripts/check_chain_currency.sh --check
---

The genericity boundary this substrate serves, and the composed structural doctrine that enforces it,
are [[production-genericity-boundary]].

The `.e.iii` substrate makes opacity and proof admission executable. A source-owned symbol is a private spelling
plus a public scope/ordinal identity; ordinary code cannot display, serialize, string-convert, or spelling-order
it. Sealed capabilities delimit grammar, persistence, presentation, and lowering access. A closed SHA-256 ruleset
and schema-1 claim ledger describe typed capture/model/prior/upstream/axiom premises, allowed spelling-free symbol
uses, and exact conclusion bytes. The kernel is the only proposal-to-proof transition and returns an unforgeable
in-memory verification witness. A deserialized ledger—even at the current schema and ruleset—still requires the
current evidence/conclusion check; old/stale input rebuilds or residualizes, while malformed/future input rejects.
The live inventory is now 78 modules / 41 families / 168 fields. The conformance-owned behavioral harness adds
one downstream module without changing the core dependency direction or the exact field denominator. The SemanticIR migration split the original
carry row after executable proof showed that three surfaces are filtered or visually extended projections.
Existing later-stage producers remain proofless until the
exact `.e.iv` migration, so whole-core signoff remains open.

The first `.e.iv` child froze the exact reviewed migration graph before a stage schema changed. Its current 41 rows
expand to 168 field-root rules and resolve 117 producer/mutator entrypoints, 53 canonical seams, and four
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
24 current / 54 unmeasurable through EvidenceIR, SemanticIR, and IntentIR. The IntentIR and adapter proof
migrations below supersede the old later-stage-local result.

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

Adapter schema 2 completes the five-stage cumulative authority chain. It preserves the entire verified IntentIR
ledger as an exact ordered prefix, then proves all 12 public fields across envelope, lowering, residual, and
validation families. Every populated array record receives a stable claim. Every nonblank rendered ISF line and
every blocking reason also receives an addressable conclusion, while the exact `isf` root binds the complete
typed/rendered model. Thus neither emitted syntax nor honest refusal to emit lives outside the proof graph.

Canonical adapter construction accepts only verified IntentIR and reconstructs identity, paths, typed `IsfIr`,
rendered source, counts, residuals, and renderability. Load, pretty serialization, manifest write, and emitted-file
reconciliation execute that current relation. Validation is a closed mutation limited to `validation_reports`;
test-fixture projection is separately compiled and cannot be enabled through production artifact data. Current
proofless, stale, forged, or unauthorized manifests reject, schema 1 is inspection-only, and future schemas
reject. Exactly 24 reachable adapters migrated with zero pre-existing public-field delta outside
schema/proof/validation. All 24 current manifests are honestly blocked and reconcile to zero emitted files; the
54 historical chains remain upstream-proof-unmeasurable.
