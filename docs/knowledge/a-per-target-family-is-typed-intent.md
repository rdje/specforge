---
id: a-per-target-family-is-typed-intent
title: A declared signal the document gives one instance per actor of a declared role (APB's PSELx for each Completer, AHB's HSELx for each Subordinate) is a SemanticIR signal_families record carried into IntentIR, the document's own name for a declared signal is a persisted signal_coreferences record every reader resolves through, and an actor that drives a family declares it once and lists it; since 2026-09-26 (EXTRACTION-REPRESENTATION-GAPS.24.b)
answers:
  - "what is signal_families / SignalFamilyRecord"
  - "what is signal_coreferences / SignalCoreferenceRecord"
  - "why does APB-E have an invariant for PNSE when PSEL is asserted"
  - "why does requester.isf list isf_family_driven_once for PSELx"
  - "how is PSEL tied to PSELx without reading the trailing x"
  - "why does AHB's HSELx family produce no adapter residual"
date: 2026-09-26
tags: [semantic, intent, isf, families, coreference, adr-0037, extraction-representation-gaps, measured]
evidence: crates/specforge/src/ir/semantic.rs (SignalCoreferenceRecord, SignalFamilyRecord, document_signal_coreference_records, document_signal_family_records, signal_coreference_map); crates/specforge/src/ir/intent.rs (extract_condition_from_text, signal_families); crates/specforge/src/ir/isf_ir.rs (family_driven_once_residual_packet); doctrine/production_genericity/claim_family_inventory.tsv; docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.24, .24.a, .24.b, .14)
reverify: "cargo test --offline -p specforge-core --lib -- a_family_is_one_declared_signal_per_declared_actor a_co_reference_record_keeps_its_role_phrase_and_statements a_constraint_condition_is_read_whole_or_the_invariant_is_not_made a_family_the_actor_drives_is_declared_once_and_listed. jq -c '.signal_families' generated/semantic_ir/ihi0024_e_2023_02_amba_5_apb_protocol_specification/semantic_ir.json -> [{\"signal_name\":\"PSELx\",\"per_actor\":\"Completer\",\"supporting_statement_ids\":[\"statement_0168\"]}] on 2026-09-26."
---

**Landed `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.24.b`, after `.24.a`'s decision).** Two facts are now typed
records in SemanticIR, read once and skipped when empty, so only APB-E and AHB carry them:

- `signal_coreferences`: the document's own name for a declared signal, tied by a repeated appositive role phrase.
  APB-E writes *Select signal, PSEL* and *Select signal, PSELx*, giving `PSEL` → `PSELx` (statements 0203, 0211,
  0243). The temporal-rule builder takes this map instead of recomputing it, and IntentIR's invariant reader now
  resolves through it. APB-E's *PNSE must be valid when PSEL is asserted* (`sigcon_0007`) therefore gains its
  invariant `tinv_sc_sigcon_0007`, which the contract path already had. AHB never ties `HSEL` to `HSELx`, so it has
  none.
- `signal_families`: a sentence that names exactly one declared signal and gives it one instance per actor of a
  declared role, via *for each ROLE* or *Each ROLE has its own …*. APB-E's `PSELx` per `Completer` comes from 0168;
  AHB's `HSELx` per `Subordinate` comes from 0277 and 0658. *For each 8 bits of HWDATA* and *for each channel* name
  no actor. Nothing is read from a trailing `x` (ADR 0037).

IntentIR carries `signal_families` as a lossless carry (`intent.carried_semantics`). The adapter declares a family
once. When the lowered actor *drives* it, as APB-E's Requester drives `PSELx`, the adapter lists
`isf_family_driven_once`: every instance follows the signal's contract, and the document does not state how many
there are. A reader of the family, such as AHB's Manager (the decoder drives `HSELx`), gets no residual. No `.isf`
changed; fanning out one instance per target is composition (`.14`). Registering the fields moved the rule
inventory from 170 to 173 rules. Every live chain's proof digest moved, and all 27 were regenerated.
`[[an-invariant-holds-at-every-cycle]]` · `[[a-polarity-rule-reaches-its-stated-scope]]`.
