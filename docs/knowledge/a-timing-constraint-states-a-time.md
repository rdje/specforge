---
id: a-timing-constraint-states-a-time
title: A statement is a timing_constraint only when the timing words come with a stated time, read as whole words — 534 of 813 left the class on 2026-10-20 and none joined it
answers:
  - "what counts as a stated time (states_a_time)"
  - "why is 'MASTER and SLAVE interfaces MAY be designed so that they do not support the BLOCK cycles' no longer timing_constraint"
  - "does 'the current cycle' or 'at least 4 bytes' state a time (no)"
  - "is tPortConfiguration or tSU a stated time (yes: a lowercase t before a capital names a timing parameter)"
  - "why is 'A 2 ms timer shall be started' not timing_constraint (none of the admitting words)"
  - "how many timing_constraint statements are there on the 31 chains (279 since 2026-10-20)"
  - "what did the timing fix add downstream (47 conditional rules, 9 signal hints, one Wishbone residual)"
  - "what is EXTRACTION-REPRESENTATION-GAPS.3.d.iii.b"
date: 2026-10-20
status: current
tags: [evidence-ir, classifier, timing, statement-class, measured, extraction-representation-gaps]
evidence: crates/specforge/src/ir/evidence.rs (classify_statement's timing arm; states_a_time; TIME_UNITS, TIME_COUNT_WORDS, RELATIVE_CYCLE_WORDS, TIMING_PARAMETERS, TIME_PHRASES, EDGE_WORDS; stated_time_tests); docs/archive/tasks/extraction-representation-gaps/default-and-assumption-leaves.md (.3.d.iii.b)
reverify: "cargo test --offline -p specforge-core --lib stated_time_tests && python3 -c \"import json,glob; es=[json.load(open(p)) for p in glob.glob('generated/evidence_ir/*/evidence_ir.json')]; print(sum(1 for e in es if e.get('proof_context') for s in e['extracted_statements'] if s.get('class')=='timing_constraint'))\"   # 2026-10-20: 279 on the 31 proof-carrying chains"
---

`classify_statement`'s timing arm keeps its two word lists (` cycle`, `cycles`, `at least`, the setup/hold/edge words
and abbreviations; then `shall`, `must`, `cycle`, `ns`, `ps`, `time`, `edge`), but since
`EXTRACTION-REPRESENTATION-GAPS.3.d.iii.b` (`2026-10-20`) they admit a sentence only when `states_a_time` also holds.
That test reads whole words (an underscore stays inside a token, a possessive *'s* is dropped) and accepts:

- a number and a unit, apart or fused (*12 ms*, *2-cycle*, *0.3 UI*, *10ms*, *1.5us*); *clock* counts only before
  *cycle(s)*/*period(s)* or at the end, *bit* only as *bit time(s)*, and a fused *s* never (*write 1s*);
- a count of cycles, clocks or seconds (*one clock cycle*, *a single cycle*, *within N cycles*, *three seconds*) and of
  a named clock's cycles (*50 SWCLKTCK cycles*: the middle word holds *clk* or *tck*);
- an event placed at a cycle with *next*, *previous*, *same*, *following*, *subsequent*, *first* or *last* (*the
  current cycle* places nothing: in Wishbone it is the bus cycle under way), or *cycle N*;
- a timing parameter: the abbreviations `tsu`, `thd`, `tckh`, `tckl`, `tco`, `tpd`, `toh`, `tih`, or any token written
  as a lowercase *t* before a capital (all 98 such tokens on the 31 chains are USB 3.2, USB4, I2C and I2S timing
  parameters);
- *setup time*, *hold time*, *clock period*, *propagation delay*, *clock edge*, or *rising/falling/positive/negative*
  with *edge* at most one word later. A latency with no value (*the maximum latency … to be calculated*) is none.

**Measured over the 31 chains** (evidence dry-run against `2d61dc479`'s): 534 of 813 statements leave the class (374 ->
`source_fact`, 109 -> `normative_statement`, 51 -> `conditional_rule`; Wishbone 249, USB 3.2 52, ADIv6 41) and 279
keep it; the gate only narrows, so no sentence joins. The readers of the other classes then add 47 conditional rules in
11 chains (Wishbone 18, USB 3.2 9) and 9 prose signal hints in 3; nothing is removed. Rebuilt stage by stage, SemanticIR
and IntentIR move only `conditional_rules` and, in TMC, AHB and Wishbone, the hinted signals' semantic candidates;
Wishbone's adapter gains one `isf_obligations_not_lowered` residual (five obligations it cannot lower), AXI's and
ADIv6's residual texts renumber rule ids, and every `.isf` text is byte-identical. Six of the 9 new hints give a role
the sentence does not (`.50`).

**Deliberately not done.** A sentence that states a time but holds none of the admitting words keeps its class (*A 2
ms timer (tLoopbackExitTimeout) shall be started upon entry to the substate* stays `normative_statement`; 728 such
statements, 273 of them table rows). Widening the gate would move them for a label
only: the class's one dedicated reader, `extract_interface_edge_timings`, needs a sampling or driving clause on an
explicit edge, which the admitting words already cover.

Links: [[unread-behaviors-are-permissions-and-the-timing-class-misfires]].
