---
id: register-field-table-defragmentation
title: A register whose field table a PDF backend split across several tables is de-fragmented into one record (conservative all-field-names-distinct gate)
answers:
  - "why does one register appear as several RegisterRecords / how are split register-field tables merged"
  - "what is consolidate_register_field_fragments / EXTRACTION-GAP-FIX.4c"
  - "how are Docling-fragmented register field tables de-fragmented without fabricating a field set"
  - "why are sbaddress3 / custom0 / a garbled sizelo register NOT merged"
  - "how does de-fragmentation enable the recover-register-bits gate (b)"
date: 2026-06-08
tags: [registers, extraction, evidence, defragmentation, pdf-variant-digestion, extraction-gap-fix, adr-0006]
evidence: crates/specforge/src/ir/evidence.rs (consolidate_register_field_fragments, register_fragments_are_safe_to_merge, merge_register_fragments); docs/tasks/EXTRACTION-GAP-FIX.md (.4c)
reverify: "./target/debug/specforge evidence generated/source_ir/1_0_risc_v_debug_specification/source_ir.json >/dev/null 2>&1 && python3 -c \"import json;r=json.load(open('generated/evidence_ir/1_0_risc_v_debug_specification/evidence_ir.json'))['register_records'];print(len(r),'records');print([len(x['fields']) for x in r if x['register_name']=='dmcontrol'])\"   # → 44 records, dmcontrol [13]"
---

`EXTRACTION-GAP-FIX.4c`: a PDF backend (Docling) often splits ONE register's field-definition table across
several tables, so the evidence builder emitted several `RegisterRecord`s for one register (RISC-V Debug
`dmcontrol` → `regfld_table_0023/0024/0025` = 1+5+7 fields). That inflates the register count AND blocks
`recover-register-bits` gate (b) (`proposal_names_match_fields` needs a register's FULL field set in one record).

`consolidate_register_field_fragments(&mut Vec<RegisterRecord>)` (called in the base `evidence` build after
register synthesis) de-fragments them. It groups records by recovered `register_name` and merges a group ONLY
when `register_fragments_are_safe_to_merge` holds — **all field names across the whole group are distinct
case-insensitively.** That single test is exactly the two safety gates:

- (gate 1) no fragment repeats a field name INTERNALLY → rejects a garbled bit-row read as one field repeated
  (RISC-V `mcontrol` `regfld_table_0075` = `sizelo`×13);
- (gate 2) the fragments are PAIRWISE DISJOINT → rejects distinct registers an upstream heading-association
  collapsed to one name (a `sbaddressN` array all named `sbaddress3`, each carrying the same `address` field;
  `custom0` ×3 all `data`).

So an ambiguous or garbled group is left EXACTLY as it was — **never fabricate a register's field set** (the
owner honesty guardrail; [[register-diagram-bit-recovery-via-tiling]]). A safe merge (`merge_register_fragments`)
takes the first fragment's identity, the UNION of fields in fragment order, the width recomputed from the full
field set, the first resolved `offset_address`, and the de-duplicated UNION of `supporting_statement_ids`; it is
emitted at the FIRST fragment's original position so output order stays deterministic.

**Verified live (RISC-V Debug, NVMe — the in-`corpus/` docs that retain a `normalized/` bundle):** RISC-V Debug
**60 → 44** register records (dmcontrol/dcsr/mcontrol6 merged; custom0/mcontrol/sbaddress3/icount/textra64
honestly left split), NVMe **44 → 42**. The `.4a.2`/`.4a.3` golds are UNCHANGED (their recall is register-scoped
/ pools fragments, so de-fragmentation is pooling-invariant: RISC-V field-name 20/34=0.588, NVMe field-name +
bit-structure 28/29). The `.5b` completeness gauge is cleaner (dmcontrol once, complete). Wire-based specs carry
no register-field tables, so it is a structural no-op for them (and it only touches `register_records` — never
constraints/relations/temporal). Agnostic (ADR 0006 — structural, no chip names).
