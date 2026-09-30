---
id: unread-intent-records-fall-into-six-families
title: The IntentIR assumptions and behaviors no typed record understands fall into six families, and assumptions are lifted by a word
answers:
  - "what are the unread IntentIR assumptions and behaviors"
  - "why are most IntentIR assumptions not understood"
  - "what is scripts/measure_unread_intent_records.py"
  - "which IntentIR assumptions does SpecForge write itself (in this IntentIR pass)"
  - "does any IR field say whether an actor is software"
  - "where are software obligations like the USB4 Connection Manager's procedures carried"
  - "why is a stated default like the DTF value assumed 0 an assumption"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.d"
date: 2026-09-30
status: current
tags: [intent-ir, semantic-ir, census, assumptions, behaviors, software-intent, extraction-representation-gaps, measured]
evidence: scripts/measure_unread_intent_records.py; scripts/measure_category_family_census.py (untyped_not_understood); docs/research/category-family-census.md (What the unread assumptions and behaviors are); docs/tasks/extraction-representation-gaps/table-census-leaves.md (.3.d, .3.d.i-.3.d.iii); crates/specforge/src/ir/intent.rs (the pass assumptions)
reverify: "python3 scripts/measure_unread_intent_records.py   # expect 443 records (166 assumptions, 277 behaviors) on 2026-09-30, and the six families below"
---

An IntentIR `assumptions` or `behaviors` record is free text with provenance. It is **not understood** when no
statement it cites is cited by a typed record (`untyped_not_understood` in the category-family census). On
`2026-09-30` the 31 current chains hold **443**: 166 assumptions, 277 behaviors.

`scripts/measure_unread_intent_records.py` places each in the first family that takes it:

| family | records | what it is | route |
| --- | ---: | --- | --- |
| `pass_assumption` | 17 | IntentIR's own modeling choices, citing no statement (`intent.rs`: *… treated as a backend-neutral channel abstraction in this IntentIR pass.*) | by design |
| `table_row` | 50 | Markdown table rows lifted as prose | the table's family owner (`.3.b`) |
| `apparatus` | 40 | self-reference, figure examples, informative notes | non-contract |
| `software_agent` | 47 | software obligations; 34 in the USB4 connection-manager guide | `.3.d.i` |
| `assumed_by_word` | 111 | rules, defaults and conventions that say *assume* or *treated as* | `.3.d.ii` |
| `permission_or_description` | 178 | permissions, unread obligations, descriptions; 17 mis-classed as timing | `.3.d.iii` |

Two facts a later reader needs. **Assumptions are lifted by a word**: SemanticIR makes an abstraction of a sentence
that says *assume* or *treated as*, so *the DTF value assumed for reporting such faults is 0* — a hardware default —
is filed as an assumption no reader lowers. **No IR field says whether an agent is software**: SemanticIR actors are
role nouns (`agent`, `controller`, `host`, `manager`), so the census's software class rests on a closed list of
agent words the script declares, and it is 6 of 7 precise on its sample (I2C's *output drivers* are hardware).

Links: [[untyped-uncited-tables-fall-into-fifteen-families]], [[captured-region-residual-carrier]].
