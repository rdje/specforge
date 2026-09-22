---
id: constraint-scope-carrier-is-lost
title: A signal constraint read from a list item or table row drops the scope its lead-in or caption states — 186 of 358 records sit under a scope carrier, and in a 40-record sample all 16 restricting scopes were lost
answers:
  - "why is RACK must be LOW published without during reset"
  - "why does AWBURST must be INCR have no condition"
  - "does a constraint keep its list lead-in"
  - "does a table row constraint keep its caption"
  - "what is a scope carrier"
  - "how many constraints lose their governing condition"
  - "can the lead-in be copied into condition_text"
  - "why are nested list items flattened"
  - "what is a row-keyed constraint"
date: 2026-09-23
status: current
tags: [kg-isf-completeness, constraints, conditions, extraction-quality, measurement, fidelity]
evidence: scripts/measure_constraint_scope_carriers.py; docs/research/constraint-scope-carriers.md; docs/tasks/KG-ISF-COMPLETENESS.md (.5.xiii.a.3, .5.xiii.a.3.a)
reverify: "python3 scripts/measure_constraint_scope_carriers.py --sample 40 --seed 13 — expect 358 traced records; replayable bullet 50/46 scoped (32 lead-in, 14 nested), table row 33/33 caption, prose 20; legacy bullet 62/51 (42/9), table row 87/56, prose 106; the seed-13 sample's restricting carriers (during reset, atomic-transaction rules, DVM Complete constraints, When FRESETn is asserted, barrier response constraints, the AXI4-Lite upgrade) all print carries_condition=no."
---

A constraint's `condition_text` comes only from the statement that binds it. A document often states
the scope one level up — a list lead-in (*"During reset, the following interface requirements apply:"*),
a bullet that leads a nested list the text layer flattened (*"- If AWADDR points to the lower half of
the transaction:"*), or a table caption (*"Table A15.23: DVM Complete request constraints"*) — and that
**scope carrier** never reaches the record. `During reset … must drive RACK LOW` is published as *"RACK
must be LOW"*.

Measured `2026-09-23` over all 78 persisted chains: **186 of 358** constraint records sit under a scope
carrier (79 of the 103 the current producer mints). A seeded 40-record sample, read by hand: **16
restricting, 0 kept**; 8 row-keyed (the condition is a sibling key column, like a multi-key encoding
table); 16 neutral (*"The User signals protocol rules are:"*).

**Do not copy the lead-in into `condition_text`.** Forty percent of carriers are neutral, and telling
them apart needs meaning, which ADR 0006 forbids reading with a word list. **Do not residualize every
scoped record either**: the neutral carriers include constraints the WIRE-BASED-100 golds score (AHB
`HAUSER`/`HBUSER`, five items in `seed_ahb.json`; APB's user signals are NOT gold-scored). The structural move is to record the carrier as provenance and measure any
lowering gate against the golds — `.5.xiii.a.3.a`.

Related: [[enum-member-name-column-needs-adjudication-not-a-score]] (the multi-key table is the
row-keyed shape in another surface), [[isf-value-width-operand-contract]].
