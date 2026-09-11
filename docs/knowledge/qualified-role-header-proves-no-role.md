---
id: qualified-role-header-proves-no-role
title: A column header that qualifies a closed role proved no role, which is why one classifier change silently emptied 115 typed AXI declarations
answers:
  - "why did a Name | Signals covered | Width | Check enable table classify as unknown"
  - "which revision narrowed SourceIR table classification (dee0740f, 2026-08-12, SPEC-TO-INTENT-ALIGNMENT.6d.ii.b — whole-label role equality replaced substring role matching; bb5047c2 re-encoded it in the Rust authority the same day)"
  - "where is the gate that stops an unknown-kind table from producing signal declarations (should_treat_table_as_top_level_signal_description, crates/specforge/src/ir/evidence.rs — the _ => continue width/direction arm is never reached for such a table)"
  - "may a closed classifier role carry a qualifier (yes — e125aac7 admitted one balanced parenthesized qualifier for register carriers; WIRE-BASED-100.10 admits a generic interface noun as a whole word for the signal role)"
  - "how do I tell whether a persisted SourceIR was produced before or after the classification narrowing"
  - "do equal SourceIR counts prove an identical artifact (no — 320/333/286/3652/527 all held while table_kind moved on 67 of 286 tables, section_kind on 56 of 527 and diagram_kind on 20 of 333)"
  - "what happens to every persisted SourceIR proof when I change SourceIR production semantics"
  - "how do I refresh a stale SourceIR proof without a Docling re-ingest (source_proof_migrate --retained-manifest doctrine/chain_currency/retained_bundles.json --write, then re-run evidence/semantic/intent/adapt)"
  - "why can the APB, AHB and AXI SourceIR proofs not be migrated from a retained bundle"
  - "how do I measure a classifier change's blast radius before shipping it"
  - "why is a green wire-protocol score not evidence that extraction is intact"
  - "why did a signal table declare English words like Secure, Stream, Asserted or The (its name column scored zero because the column scorer did not strip a cell's punctuation the way the row loop does, so a paired name cell like AWMMUSECSID, ARMMUSECSID lost, and the Description column won the override)"
  - "which column does synthesize_signal_declarations read names from, and when does content overrule the header (a distinct-hardware-token score per column, overruling the header only on a lead of at least two tokens)"
  - "why did AXI have 134 actors when APB has 8 and AHB has 25"
date: 2026-09-11
status: current
tags: [sourceir, classification, adr-0006, proof-kernel, chain-currency, wire-based-100, regression]
evidence: "crates/specforge/src/ir/source.rs (classified_table_kind, header_names_signals, header_has_role); crates/specforge/src/ir/source/docling_backend.rs (classify_table_kind, classifier_header_names_signals); crates/specforge/src/ir/evidence.rs:4041; crates/specforge/examples/source_proof_migrate.rs; docs/tasks/WIRE-BASED-100.md (.9d, .10, .10a, .10b)"
reverify: "cargo test -p specforge-core --lib qualified_signal_header_still_names_signals signal_noun_alone_does_not_fabricate_a_signal_table embedded_source_classifiers_are_structural_and_identity_invariant"
---

# A role a header qualifies is still that role

SpecForge classifies a captured table by **closed structural roles** read from its column headers. Since
`dee0740f` (`2026-08-12`, "make SourceIR classification neutral") a header proves a role only when its whole
normalized label *is* that role — deliberately, so that a document symbol appearing inside a label can never
invent authority. The rule it enforces for signal tables is: a name role **plus** an explicit
signal/port/pin column **plus** a width (or a direction column).

The narrowing overshot on one shape. `Name | Signals covered | Width | Check enable` satisfies that rule in
substance — `Signals covered` names signals as plainly as `Signal` does — but its label is not *equal* to any
role, so the table fell to `unknown`. **An `unknown` table never reaches declaration synthesis at all**:
`should_treat_table_as_top_level_signal_description` (`crates/specforge/src/ir/evidence.rs:4041`) refuses any
kind but `SignalDescription`, so the width/direction `_ => continue` arm downstream is not even consulted.
On AMBA AXI that cost **115 typed `*CHK` declarations** (`table_signal_declaration_provenance` 411 → 265
records, 304 → 170 distinct) and **130 declared signals** — and **not one of the six WIRE-BASED-100 scores
moved**, because a gold only tests the facts it names.

The repair had already been made once for a different role: `e125aac7` (`2026-08-15`, "restore structural
register carriers") admitted **one balanced parenthesized qualifier** so `Address (A[3:2], BANK)` could prove
the address role. `WIRE-BASED-100.10` is the same repair for the signal role, in the un-parenthesized form:
a header proves the explicit-signal role when a **generic interface noun** (`signal`/`signals`/`port`/
`ports`/`pin`/`pins`) appears as a **whole word** in its label — the same authority the caption rule already
used for `Table … signals`, and identity-independent because the noun set is closed and universal.

## Three habits this cost is worth buying

**Equal counts are not an identical artifact.** `WIRE-BASED-100.9d` compared page/visual/table/content/
section counts across a re-ingest, found all five identical, and concluded the ingest was exonerated and the
change lay in the EvidenceIR producer. Every count held and every table cell was byte-identical — while
`table_kind` differed on 67 of 286 tables. Diff the *classification* fields, not the cardinalities.

**Measure a classifier change over the whole persisted corpus before shipping it.** Running each candidate
over all 77 persisted SourceIRs (11,033 tables) separated a surgical fix — **7 tables, all
`unknown → signal_description`, every one verified by hand** — from two plausible-looking variants that
minted **+425** register maps and **+1,277** encodings. The same sweep attributes a persisted artifact to its
producer generation: classify with each revision's own extracted classifier and compare to the stored
`table_kind`; 47 of 77 documents match the pre-`dee0740f` classifier and 30 match today's.

**A SourceIR production change stales every persisted SourceIR proof.** `SOURCE_PRODUCTION_SEMANTIC_SHA256`
is generated at build time from the producer graph reachable from `source_rule_registry`, so touching
`classified_table_kind` moves it and every persisted chain fails `proof ledger ruleset hash is stale` — which
empties the measurable census and, with it, the ability to score anything. The cheap remedy is
`source_proof_migrate --retained-manifest doctrine/chain_currency/retained_bundles.json --write` (rebuilds
SourceIR from the retained normalized bundle in seconds, refreshing the proof without touching content),
followed by re-running `evidence → semantic → intent → adapt`. It reaches only documents that still hold a
bundle: APB, AHB and AXI are held out under `RETAINED-BUNDLE-POPULATION-FROZEN`, so they need a real
re-ingest. Budget that refresh into the same slice as the producer change.

## The same shape, one stage later

`WIRE-BASED-100.10a` found the identical failure mode inside `synthesize_signal_declarations`: its
column scorer and its row loop disagreed about how to read a cell, so a name column full of signal *pairs*
scored zero and the override handed the table to its Description column. Prose became declarations, then
actors, then `.isf` ports. The repair is the same shape as the one above — make the two readers agree — plus
a decisive-margin rule, because a one-token lead is noise a prose column reaches by accident. Corpus-wide it
moved 18 tables and every one moved back to the column its header always named; AXI's actor count fell
`134 → 25`. **Both defects were invisible to all six wire-protocol scores, in both directions:** the scores
did not move when the facts were lost and did not move when they came back.

Links: [[live-surface-edit-bookkeeping-chain]], [[chain-currency-doctrine]],
[[retained-bundle-population-is-frozen]], [[corpus-canonical-currency-and-ownership]].
