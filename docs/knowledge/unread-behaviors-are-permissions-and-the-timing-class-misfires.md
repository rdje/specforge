---
id: unread-behaviors-are-permissions-and-the-timing-class-misfires
title: The unread IntentIR behaviors are mostly permissions, and 645 of 813 timing_constraint statements state no time because the timing arm matches substrings
answers:
  - "what are the unread permission_or_description behaviors (154 of 176 are permissions)"
  - "what modalities does measure_unread_intent_records.py give (obligation, recommendation, permission, capability, description)"
  - "why are Wishbone's MAY sentences classed timing_constraint"
  - "why do 645 of 813 timing_constraint statements carry no time expression"
  - "which words does the timing arm of classify_statement match (cycle, cycles, at least; cycle, ns, ps, time, edge as substrings)"
  - "why is 'A device shall not assume that transactions occur at the same location' filed apparatus"
  - "what are EXTRACTION-REPRESENTATION-GAPS.3.d.iii.a, .b and .c"
date: 2026-10-19
status: current
tags: [intent-ir, behaviors, permissions, timing, classifier, census, measured, extraction-representation-gaps]
evidence: scripts/measure_unread_intent_records.py (MODALITIES, modality); crates/specforge/src/ir/evidence.rs (classify_statement, the TimingConstraint arm; is_non_contract_assumption's premise phrases); docs/archive/tasks/extraction-representation-gaps/unread-intent-leaves.md (.3.d.iii); docs/archive/tasks/extraction-representation-gaps/default-and-assumption-leaves.md (.3.d.iii.a-.3.d.iii.c)
reverify: "python3 scripts/measure_unread_intent_records.py | rg 'permission_or_description|modalities'   # 2026-10-19: 176; permission 154, obligation 8, description 7, recommendation 4, capability 3"
---

After `.3.d.ii.b.ii` the census's `permission_or_description` family holds **176** IntentIR behaviors that no typed
record cites, and `EXTRACTION-REPRESENTATION-GAPS.3.d.iii` (`2026-10-19`) gave each a modality (first match wins):
permission 154 (*may*, *permitted*, *allowed*, *optional*), obligation 8 (*must*, *shall*, *is required to*),
description 7, recommendation 4 (*should*, *recommended*), capability 3 (*can*). USB 3.2 holds 51 of the permissions,
IOMMU 35, Wishbone 20 and the USB4 connection-manager guide 18. So the family is a permission family, and a permission
has no carrier (`.3.d.iii.a`).

**The timing class misfires corpus-wide.** `classify_statement`'s timing arm requires a word from one list and a word
from another, both matched as substrings: ` cycle`, `cycles`, `at least`, setup/hold-time words and timing
abbreviations in the first, `shall`, `must`, `cycle`, `ns`, `ps`, `time`, `edge` in the second. `cycles` satisfies
both alone, `ns` matches *transactions*, and `at least` matches a size. Over the 31 chains **645 of 813**
`timing_constraint` statements (79%) carry no time expression — Wishbone 268, USB 3.2 72, ADIv6 62, AXI 60 — among
them the 17 unread Wishbone, USB 3.2, I2C and OpenCAPI behaviors (*MASTER and SLAVE interfaces MAY be designed so that
they do not support the BLOCK cycles*). **Fixed `2026-10-20`** by `.3.d.iii.b`: the timing words now admit a
sentence only with a stated time, and 534 of the 813 left the class ([[a-timing-constraint-states-a-time]]).

**Seven obligations are filed apparatus.** `.3.d.ii.b.i`'s premise phrase *assume that* files a sentence such as *A
device shall not assume that transactions occur at the same location within each service interval* as apparatus,
though it states the device's obligation; `.3.d.iii.c` owns them with the 14-record remainder.

Links: [[assume-sentences-fall-into-six-shapes]], [[an-assume-sentence-about-the-document-is-apparatus]],
[[a-timing-constraint-states-a-time]].
