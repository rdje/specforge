---
id: semantic-interface-authority-empty-fallback
title: Declaration-free heuristic interfaces require a signal-led deontic behavior statement
answers:
  - "what happens when SemanticIR has no authoritative signal names"
  - "can low confidence statement tokens create an interface without a formal signal declaration"
  - "why did USB 3.2 produce 918 interfaces and 556 adapter signals"
  - "how many retained documents depended entirely on heuristic SemanticIR interfaces"
  - "what is the corpus impact of the authority empty interface fallback"
  - "does an actor signal relation alone authorize a SemanticIR interface signal"
  - "why is DTI DOWNSTREAM not preserved as a heuristic only wire"
  - "how does retain_authoritative_interface_candidate_signals behave with an empty authority set"
  - "does the authority empty repair preserve formal and system contract interfaces"
  - "how does SemanticIR preserve VALID READY without formal signal declarations"
  - "what grounded heuristic only interface evidence is preserved"
  - "why is the repaired USB 3.2 ISF adapter blocked"
date: 2026-08-09
tags: [semantic-ir, interfaces, signal-authority, fail-closed, corpus-coverage, dense-prose, isf, false-positive]
evidence: docs/research/dense-prose-signal-authority-measurement.md; crates/specforge/src/ir/semantic.rs (build_interfaces; retain_authoritative_interface_candidate_signals; authority_empty_statement_tokens_do_not_become_interfaces); generated/semantic_ir/usb_3_2_revision_1_0_2017_09/semantic_ir.json; generated/intent_ir/usb_3_2_revision_1_0_2017_09/intent_ir.json; generated/adapters/isf/usb_3_2_revision_1_0_2017_09/adapter.json; docs/tasks/CORPUS-COVERAGE.md (.2.33d.iv.b/.iv.c)
reverify: "Build the release binary. Run specforge semantic on the retained USB EvidenceIR, then intent and adapt --target isf. Expect zero interfaces and signal records, zero actor ports/relations, adapter lowering_status blocked with no signals declared in interface, zero ISF signals/rules, and no emitted *.isf sibling. Run the three retain_authoritative_interface_candidate_signals tests, authority_empty_statement_tokens_do_not_become_interfaces, builds_semantic_ir_from_handshake_evidence, and system_contract_signals_become_explicit_interface_records."
---

**Established `2026-08-09` (`CORPUS-COVERAGE.2.33d.iv.b`).** `build_interfaces` first collects typed
signal authority from formal `Signal X is ...` declarations and the document system contract. Statement-level
co-mention groups are a secondary enrichment path. Before this repair,
`retain_authoritative_interface_candidate_signals` treated an empty authority set as an exception and returned
every heuristic candidate unchanged. Ordinary uppercase prose, encodings, and data-table cells could therefore
bootstrap the very interface surface that was supposed to authorize them.

The retained-corpus census found 21 documents whose interface records were entirely low confidence: 5,527
interfaces, 18,397 records, and 4,060 per-document unique names. Their adapters consumed those records as 4,060
signals and 544 rules; 19 were marked renderable. USB 3.2 was the clearest falsification: raw encoding rows
formed 918 interfaces / 2,940 records, deduplicated to 556 one-bit adapter outputs and 170 rules despite zero
actor ports and zero actor/signal relations.

DTI was the only member with any actor graph evidence: one medium-confidence `TBU reads DOWNSTREAM` relation.
Its 2,325 all-low interface records and 294 adapter signals are ordinary token soup, and `DOWNSTREAM` is a prose
direction word rather than a declared wire. A relation can ground actor-relative use of an already authorized
signal, but it does not independently declare the signal or authorize unrelated statement tokens. No
heuristic-only preservation exception was therefore justified.

The fail-closed repair replaces the empty-set allow-all bypass with one narrow positive grammar. Formal
declaration records take their explicit path, and system clock/reset signals enter the typed authority set before
grouping. When neither exists, a multi-signal statement may ground its own group only if it begins with one of
those signal identifiers and immediately makes a deontic signal action: `must`/`shall` plus assertion,
deassertion, or stability. Thus `VALID must remain asserted until READY is observed` remains a canonical
heuristic-only handshake, while encoding rows, ordinary prose, and `TBU reads DOWNSTREAM` fail closed.

Dry-running all 21 affected retained documents still yields zero interfaces and records. APB, AHB, AXI, and SWD
interface surfaces are byte-equivalent to the preserved pre-change binary. Executable tests cover raw-token
rejection, the declaration-free VALID/READY positive, full handshake construction, declared-surface filtering,
and system-contract preservation.

The real USB rebuild now carries zero interfaces, zero interface records, zero actor ports, and zero actor
relations. The adapter is honestly blocked on `no signals declared in interface`, emits zero signals/rules, and
the successful blocked write removes the obsolete `channel.isf`, leaving only `adapter.json`. Typed behaviors,
constraints, transactions, storage, and exact residual evidence remain in canonical IR; no hardware surface is
fabricated merely to make the adapter renderable.

Final `.iv.c` signoff reproduced all four downstream hashes both before and after documented SourceIR validation
backannotation and the same typed result, passed focused/WIRE/KG/full-CI/book/doctrine/path/locality gates, then
deleted the authenticated nine-file rollback with zero task-id residue. The USB parent tree is closed; the current
70 emitted adapters remained under the established strict-clean contract at that boundary.

**Independent transfer proof (`CORPUS-COVERAGE.2.34b.ii.b`).** USB4 Inter-Domain Service previously carried one
`USB4` interface signal, two rules, and eight enums. A fresh current-binary cascade retains two `USB4` conditional
consequents as source evidence but produces zero relations, interfaces, ports, connectivity, adapter signals, or
rules. Lowering blocks on `no signals declared in interface`, keeps six storage records, removes the obsolete
`channel.isf`, and leaves exactly `adapter.json`. The live emitted set is now 69/69 FSMGen-strict clean. This
second document confirms the authority-empty repair is structural rather than USB 3.2-specific.

**Third sibling transfer (`CORPUS-COVERAGE.2.35`).** USB4 Connection Manager previously promoted 13 prose
relations into four interfaces, 11 ports, three signals (`SB`/`USB`/`USB4`), three rules, and one generic enum.
The current cascade retains four exact `USB4` structured references as source evidence but produces zero
relations, interfaces, ports, or connectivity. The self-declared methodology guide blocks on no declared
interface signals, removes `device_also.isf`, and leaves exactly `adapter.json`; the live set is 68/68 strict-clean.

**Fourth guide transfer (`CORPUS-COVERAGE.2.37`).** AArch64 External Debug previously carried 64 heuristic
interfaces and a 73-signal `agent.isf` despite zero evidence relations or declarations. The current cascade
retains 243 statements, five conditionals, and 78 behaviors, but removes the unsupported interfaces and the
diagram-label-derived `host` actor. The high-confidence methodology guide blocks on no declared interface
signals, removes `agent.isf`, and leaves exactly `adapter.json`; the live emitted set is 66/66 strict-clean.
Unlike the intervening CoreSight Base System architecture transfer, no under-extracted-spec warning applies.

**Fifth guide transfer (`CORPUS-COVERAGE.2.38`).** Introducing CoreSight appeared stronger because retained
EvidenceIR carried nine relations, but those records are sentence fragments such as `RAM is reads APB`,
`means drives ATB`, and `debugger does reads DRW`, not declarations or stable endpoints. Fresh extraction removes
all nine relations plus six interfaces, eight ports, four connectivity edges, and the four-signal/two-enum
surface. The guide retains 228 statements and one signal constraint, blocks with one honest unsupported-temporal
residual, and leaves exactly `adapter.json`; the live emitted set remains 66/66 strict-clean.

**Sixth guide transfer (`CORPUS-COVERAGE.2.39`).** The OpenCAPI Ready engineering note's retained chain looked
smaller but more explicit: synthetic `statement_0114` said `Signal DL is width 1.`, producing one high-confidence
interface and a one-output blocked adapter. The cited source is actually a headerless `Terms` table whose `DL`
row defines data link layer beside other abbreviations. Fresh extraction preserves all 105 source elements but
drops only that synthetic statement (113→112), then yields zero interfaces/signals and blocks on both no declared
signals and no behavior. Glossary syntax does not become declaration authority; the live emitted set remains
66/66 strict-clean.

**Seventh guide transfer (`CORPUS-COVERAGE.2.40`).** The OpenCAPI Certified sibling independently repeats the
same terms-table trap: synthetic `statement_0174` turns the `DL` definition for data link layer into one
high-confidence interface and one adapter output. Fresh extraction preserves all 173 source elements but drops
only that statement (173→172), leaving zero interfaces/signals and blocking on both no declared signals and no
behavior. The live emitted set remains 66/66 strict-clean; sibling agreement confirms the authority rule is
structural rather than a Ready-note exception.
