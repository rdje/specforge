---
id: identifiers-are-opaque-and-one-way-grounded
title: Document identifiers are opaque and semantic promotion is one-way grounded
date: 2026-08-12
status: accepted
scope: genericity, evidence-ir, semantic-ir, intent-ir, validation, isf-adapter
evidence: crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; crates/specforge/src/commands/nlp_enrich.rs; crates/specforge/src/commands/signal_resolve.rs; crates/specforge/src/commands/validate.rs; crates/specforge/src/ir/isf_ir.rs; crates/specforge/src/ir/adapters.rs
reverify: cargo test -p specforge --lib alpha_ && bash scripts/check_chain_currency.sh
answers:
  - "can a signal name imply handshake clock reset polarity or direction"
  - "what does opaque identifier mean in SpecForge"
  - "how are model proposed signal names grounded"
  - "why did ISF renderability fall from 44 to 17"
  - "does renaming a signal change semantic extraction"
  - "what is one-way grounding from EvidenceIR to ISF"
---

# ADR 0037: Document identifiers are opaque and semantic promotion is one-way grounded

## Context

Production treated identifier spelling as evidence. Uppercase and suffix shapes admitted signals; familiar
substrings supplied interface, handshake, clock, reset, polarity, direction, and transaction meaning; later
stages re-inferred roles that earlier stages had not proved. An otherwise structure-preserving alpha-renaming
could therefore change canonical intent and whether an ISF was emitted.

Removing the visible name tests alone exposed a second risk: grammars that had relied on uppercase rejection
could admit arbitrary prose words. Neutrality requires positive structural authority, not a larger allowlist or
denylist.

## Decision

1. Every current-document signal, field, state, actor, and value identifier is opaque. Production may copy it,
   compare it for exact identity, bind it to provenance, or sanitize it for target syntax. Its length, case,
   prefix, suffix, substring, resemblance to a conventional name, vendor familiarity, and protocol familiarity
   carry no semantic authority.
2. Grounding is one-way. EvidenceIR establishes declared identities through typed table structure or bounded
   definitional grammar. Later stages may narrow, carry, contest, or residualize those identities; they may not
   mint an identity or reconstruct a role from spelling.
3. Model proposals must resolve exactly to the current document's declared catalog. A caller-supplied catalog
   may narrow that surface but cannot add an external identity or disable grounding. Case-folded resolution is
   permitted only when it yields exactly one current-document identity; an exact match wins and collisions fail
   closed.
4. Interface, invariant, and transaction admission uses declared/current-document authority plus typed grammar.
   A descriptive behavioral cue can enrich a declared signal; it cannot make an arbitrary token a signal.
5. Clock/reset lowering consumes only a typed upstream system contract. Missing polarity stays `unknown`; a
   conventional suffix does not select it. When the contract is absent or incomplete, diagnostic ISF IR carries
   explicit unresolved placeholders, the adapter is blocked, and no target file is emitted.
6. Validation reports unresolved semantic candidates independently of their spelling. IntentIR does not create
   handshake, request/acknowledge, FIFO, drive, trigger, or transaction behavior from identifier fragments.
7. Alpha-equivariance is the behavioral obligation: renaming declarations and every bound occurrence may
   rename output symbols, but cannot change admission, roles, conflicts, residual disposition, validation
   metrics, or lowering eligibility.

## Consequences

- Exact retained-corpus reconciliation changes 18 EvidenceIR, 73 SemanticIR, 74 IntentIR, and 74 adapter
  artifacts. The large downstream surface is expected because name-derived authority had been copied widely.
- Signal constraints fall from 397 to 344 at EvidenceIR; canonical invariants fall from 31,767 to 28,876; and
  transactions fall from 281 to 239. These losses are preserved source evidence or explicit unresolved gaps,
  not permission for a document-specific exception.
- Signal-neutral conditional rules rise from 2,156 to 2,489: removing a guessed consequent signal stops valid
  system-level conditions from being discarded as constraints on an undeclared identifier.
- Adapter renderability falls from 44/78 to 17/78 because 27 documents lack a complete typed clock/reset
  contract. All 17 emitted ISFs pass the pinned FSMGen strict checker.
- A finite vocabulary census remains supplementary. The signoff architecture still requires the structural
  production/conformance boundary, proof-carrying rules, AST-aware doctrine, and behavioral qualification owned
  by `.6d.ii.e` and `.6d.ii.f`.

## Links

- Genericity invariant: [`0006-no-hardcoded-chip-spec-vocabulary.md`](0006-no-hardcoded-chip-spec-vocabulary.md)
- Generic EvidenceIR: [`0035-protocol-evidence-is-generic-and-document-derived.md`](0035-protocol-evidence-is-generic-and-document-derived.md)
- Identity-independent priors: [`0036-prior-memory-is-identity-independent.md`](0036-prior-memory-is-identity-independent.md)
- Task tree: [`SPEC-TO-INTENT-ALIGNMENT.md`](../tasks/SPEC-TO-INTENT-ALIGNMENT.md)
- Pipeline audit: [`production-genericity-pipeline-audit.md`](../research/production-genericity-pipeline-audit.md)
