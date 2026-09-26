---
id: a-condition-never-contains-its-obligation
title: A constraint's `condition_text` is the condition its sentence or its table states, bounded so it never contains the obligation — a fronted condition is read whole to its comma, *For <operation>,*, an infix *if*, *for as long as* and a trailing *for* phrase naming an operation are kept (a counting *for one …* is not), and a two-cell keyed row takes the operation its table caption names; since 2026-09-26 (EXTRACTION-REPRESENTATION-GAPS.10.c/.10.d)
answers:
  - "why does a constraint have no condition although its sentence states one"
  - "where does SpecForge read a constraint's condition (extract_condition_clause, stated_condition_clauses)"
  - "why is a condition like 'X is asserted, Y must remain asserted' wrong"
  - "why were the rows of AXI Table A15.23 DVM Complete request constraints driven unconditionally"
  - "how does a table caption condition its rows"
  - "why was an obligation recorded on the condition's signal (AWIDUNQ instead of RIDUNQ)"
  - "does 'for one response transfer' count as a condition"
  - "why are AXI rule conflicts 16 -> 0"
  - "does a list item take the condition of the sentence introducing its list (yes, since EXTRACTION-REPRESENTATION-GAPS.10.h)"
  - "why is AWLOCK no longer driven in AXI manager.isf (For Atomic transactions, the following restrictions apply:)"
  - "what condition does an obligation inferred with 'which means that' get (its antecedent clause; EXTRACTION-REPRESENTATION-GAPS.31)"
date: 2026-09-26
tags: [evidence, conditions, constraints, extraction-representation-gaps, isf, adr-0006, measured]
evidence: crates/specforge/src/ir/evidence.rs (extract_condition_clause, stated_condition_clauses, without_a_later_obligation, names_an_operation, counts_operations, without_list_markers, condition_keyed_rows_on_their_tables_operation, operation_named_by_caption); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.10.c, .10.d, .10.g); docs/book/src/pipeline/isf-adapter.md
reverify: "cargo test --offline -p specforge-core --lib -- a_constraint_keeps_the_condition_its_sentence_states a_keyed_row_takes_the_operation_its_tables_caption_names a_list_items_markers_do_not_hide_its_fronted_condition. On the live chains: jq '.signal_constraints[] | select(.condition_text != null) | .condition_text' generated/evidence_ir/*/evidence_ir.json | grep -Ec '\\b(must|shall)\\b' -> 10 on 2026-09-26, every one on a legacy chain whose bundle was reclaimed (LTI, NVMe, RISC-V IOMMU); 0 on the 27 live chains."
---

**Measured `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.10.c`, `.10.d`).** The evidence reader used to see only an
infix *when/while/during/unless/provided* and took everything after it. Two failures followed:

- **Lost conditions.** *If present, AWIDUNQ must be asserted for Prefetch transactions*, *For non-stash
  transactions, …* and the ten rows of AXI *Table A15.23: DVM Complete request constraints* carried no condition,
  lowered as unconditional Manager drives, and collided (16 rule conflicts in AXI's adapter).
- **Swallowed obligations.** *If AWAKEUP is asserted in a cycle when ARVALID is asserted …, then AWAKEUP must remain
  asserted …* took the text after the inner *when*, the obligation included.

## The rule

A condition is bounded by the clause grammar, never by the end of the sentence:

- a clause that opens with its condition is read by the fronted reader (`main_clause_of_fronted_conditional`) first;
- an infix condition ends at the comma before a later modal (`without_a_later_obligation`);
- a *for* phrase is a condition only when it names an operation (a closed class of bus-operation nouns) and does not
  count one (*for one response transfer* states how many, not when);
- list markers (*- -If …*) are stripped before the fronted readers, or the subject search takes the condition's
  signal (the `.10.d` `AWIDUNQ`/`RIDUNQ` case);
- a two-cell keyed row (`| ARADDR | Must be zero. |`) takes the operation its table's caption names, for every
  producer that reads the row. The caption is the statement linked to a visual asset that immediately precedes the
  rows; a signal-description table is excluded because its caption names a channel.

## What it changed

EvidenceIR against HEAD's binary: 23 of 27 live chains identical; AXI 70 → 80 constraints, AHB 11 → 10, APB-E and
ADIv6 one or two conditions kept each. AXI's adapter: rule conflicts 16 → 0; `manager.isf` loses 30 conditioned
drives; 20 obligations are listed under a condition that is not `SIGNAL is LEVEL` (3 before). Reading those as guards
is `.10.g`; the tie-offs that now surface unconditionally (`ARLOCK` 0, `ARSNOOP` 0) are `.10.e`.

## List lead-ins (`2026-09-26`, `.10.h`)

A list item takes the condition the sentence introducing its list fronts (`condition_list_items_on_their_lead_in`):
*For Atomic transactions, the following restrictions apply for request attributes:* conditions *- AWLOCK must be
deasserted, not exclusive access.*, whose three unconditional `AWLOCK` 0 drives left AXI's `manager.isf`. Only the
sentence ending in the colon counts, and only a *For* phrase naming an operation or quantifying over *all/each/every*,
or a fronted *If/When* clause: *It has the following rules:* conditions nothing, *For example, …:* is no condition.
AXI's *When using a Valid-Ready transport* and *When transporting tags* lists condition 4 more items; AHB's `HSIZE`
keeps *For all Device memory*.

## Inferred obligations (`2026-09-26`, `.31`)

*The select signal, PSEL , is asserted, which means that PADDR , PWRITE , and PWDATA must be valid* gives each
obligation the clause it is inferred from as its condition (`inference_antecedent`). An antecedent about the subject
itself (*REQ is asserted, which means REQ must be asserted*) adds nothing, because the antecedent producer already
records that fact. The semantic reader closes the appositive (*…, PSEL , is asserted* gives `PSELx = ASSERTED`).
`[[isf-initiator-perspective-direction]]` · `[[a-level-belongs-to-a-signal]]`.
