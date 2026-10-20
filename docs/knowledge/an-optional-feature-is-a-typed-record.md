---
id: an-optional-feature-is-a-typed-record
title: A sentence calling a feature optional is an optional_features record — feature, implementer and condition as written — carried to IntentIR, with one adapter residual per document
answers:
  - "what is optional_features / OptionalFeatureRecord"
  - "why is optionality not a permission_rules modality"
  - "how is 'It is optional for a Subordinate component to support Atomic transactions' read"
  - "what is stated_options / optional_feature_surface"
  - "why is 'Dotted transistors are optional open-drain outputs' not an option (an adjective)"
  - "why is 'optional-normative' refused (a joined word)"
  - "why does 'This register is optional' keep the demonstrative (no referent resolution)"
  - "what is isf_protocol_optional_features"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.d.iii.d and .51"
date: 2026-10-20
status: current
tags: [evidence-ir, optional, carried-evidence, isf-adapter, census, adr-0006, adr-0016, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (OptionalFeatureRecord; optional_feature_surface; stated_options; option_clause; OPTION_FOLLOWERS; optional_feature_tests); crates/specforge/src/ir/semantic.rs and intent.rs (optional_features); crates/specforge/src/ir/adapters.rs (optional_feature_residual_packet); scripts/measure_category_family_census.py (TYPED_SURFACES); docs/tasks/extraction-representation-gaps/default-and-assumption-leaves.md (.3.d.iii.d)
reverify: "cargo test --offline -p specforge-core --lib optional_feature_tests && python3 -c \"import json,glob; es=[json.load(open(p)) for p in glob.glob('generated/intent_ir/*/intent_ir.json')]; print(sum(len(e.get('optional_features',[])) for e in es if e.get('proof_context')))\"   # 2026-10-20: 52 on the 31 proof-carrying chains"
---

`.3.d.iii.a` reads what a subject *may* do; a sentence that calls a thing optional states that the thing may be
absent, and its subject is the feature, not an actor. Since `EXTRACTION-REPRESENTATION-GAPS.3.d.iii.d`
(`2026-10-20`) EvidenceIR carries each one as an `OptionalFeatureRecord { option_id, feature, implementer,
condition, supporting_statement_ids }` in the `evidence.normative` family beside `permission_rules`; SemanticIR and
IntentIR carry the list unchanged. Example: `{"feature": "support Atomic transactions", "implementer": "a
Subordinate component"}`.

**Grammar (`stated_options`).** The marker is *is optional* or *are optional* (*also* allowed between), followed by
the text's end, a space, `,`, `.`, `;`, `:` or `)` (so *optional-normative* is refused). A following word must be one
of `OPTION_FOLLOWERS` (a condition opener, a conjunction, *to*, *whether*, *normative*); any other word makes
*optional* an adjective (*are optional open-drain outputs*) or a contrast (*is optional otherwise*). The feature is
the clause before the marker from the sentence's last clause break, read past bullets and connectives (*while*,
*whereas*); inside an unclosed parenthesis, its own clause; after *because*/*since*, the reason's. A condition
follows *optional* or opens the sentence before the break. *It is optional for S to V* gives feature V and
implementer S; *it is optional whether X* gives *whether X*; any other *It is optional* is refused. **Refused** (15
of 76 uses on the 31 chains): 6 adjectives, 5 pronouns (*All are optional*), a hypothetical, a relative clause
(*most of which*), a contrast (*otherwise* or *else* anywhere before the marker), and one with no subject. Table rows
and apparatus are not read; the same option stated twice is one record.

**Not decided by grammar.** A demonstrative feature (*This register*, *This field*, *Both these features*) keeps its
words; what it refers to is in the surrounding section, not the sentence. A lead-in that is not a clause break stays
in the feature (*In WISHBONE the variable clock generator capability*).

**Measured `2026-10-20`.** 52 records in 13 chains from 61 statements (RISC-V debug 9, USB 3.2 9, ADIv6 8, Wishbone
8, AXI 6); 10 with a condition, 2 with an implementer. A Python port of the grammar produced the same 52. Each changed
chain moves only `optional_features` at every stage; each adapter gains one `isf_protocol_optional_features`
residual naming its records by id range; `.isf` texts are byte-identical; the new rule field re-sealed every chain.
Unread IntentIR records 140 -> 139: 37 of the 61 statements are source facts IntentIR lifts as no record and 23 were
read already; AXI's *Domain signaling is optional, if an interface does not have Domain signaling …* was the one.
ADIv6's `option_0006` reads its pseudocode appendix (*The block of lines consisting of elsif …*): `.51`.

Links: [[a-stated-permission-is-a-typed-record]], [[an-equivalence-rule-is-a-typed-record]],
[[stage-seal-closure-membership]].
