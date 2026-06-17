---
id: transaction-phase-membership-vlm-vs-channel
title: AXI/SWD per-signal phase membership — the recorded "VLM timing-diagram phase columns" candidate is RESOLVED measurement-first; AXI is recoverable DETERMINISTICALLY from the doc's caption-named `B1.x … channel signals` tables (channel→phase), SWD is honest degenerate absence (2-wire serial), and a VLM-membership lever is NO-GO (redundant + fabrication-prone + RAM-expensive)
answers:
  - "is AXI/SWD per-signal phase membership recoverable from timing diagrams via a VLM"
  - "what did KG-ISF-TRANSACTIONS.2l measure / decide"
  - "is the VLM the right lever for AXI transaction phase membership (no — a deterministic channel-table cue is)"
  - "how is AXI per-signal phase membership recoverable without a VLM"
  - "what are the AXI B1.x channel-signal tables and how do channels map to phases"
  - "which AXI signals belong to which channel (B1.1 write request / B1.2 write data / B1.3 write response / B1.4 read request / B1.5 read data / B1.6 B1.7 snoop)"
  - "why is SWD per-signal phase membership degenerate / empty"
  - "why are SWD recognized phases signal_set empty and transactions ports empty"
  - "what does SWD Figure B4-1 show (single SWDIO wire packet, bit-field time-phases, Host/Target/Host driver)"
  - "did the qwen2.5vl VLM recover a groundable AXI signal-to-phase mapping (no — contradictory, redundant, hallucinated signal semantics)"
  - "how much RAM did the qwen2.5vl:7b VLM use on a timing-diagram crop (13 GB; host hit 87% used, across the 85% kill threshold)"
  - "what is the .2m candidate (deterministic AXI-family channel-membership lever)"
  - "is the VLM-tier transaction frontier exhausted (yes — superseded by the deterministic channel cue / honest absence)"
  - "what is the only VLM-unique signal in AXI timing diagrams (phase ORDER, the .2h residual)"
date: 2026-06-17
tags: [kg-isf-transactions, transactions, phase-membership, vlm, qwen2-5vl, channel, axi, swd, measured, no-go, adr-0006, ram-ceiling, structured-first, north-star]
evidence: docs/tasks/KG-ISF-TRANSACTIONS.md (.2l node + Frontier); generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json (table_signal_declaration_provenance + visual_evidence channel captions); generated/source_ir/.../assets/picture-0011.png / picture-0013.png / picture-0008.png (AXI) + ihi0074_a .../picture-0038.png (SWD Fig B4-1); docs/knowledge/transaction-capture-census.md (.2j NO-GO this completes)
reverify: "All read-only + RAM-SAFE (no VLM needed to re-establish the deterministic finding). AXI (Q1): jq -r '([.visual_evidence[]|select(.asset_kind==\"table_region\")|{(.asset_id):.caption_text}]|add) as $cap | [.table_signal_declaration_provenance[]|select(($cap[.table_id]//\"\")|test(\"(?i)(write|read|snoop) (request|data|response) channel signals\"))|{sig:.signal_name,cap:$cap[.table_id]}]|group_by(.cap)|map({channel:.[0].cap,signals:([.[].sig]|unique|length)})|.[]' generated/evidence_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/evidence_ir.json -> B1.1=26 AW* / B1.2=15 W* / B1.3=14 B* / B1.4=26 AR* / B1.5=18 R* / B1.6=7 AC* / B1.7=5 CR*. SWD (Q2): jq -r '[.transaction_phases[]?|{p:.phase_name,sigs:(.signal_set//[])|length}]' generated/semantic_ir/ihi0074_a_2017_03_09_arm_debug_interface_v6_architecture_specification/semantic_ir.json -> every phase sigs=0; jq '[.transactions[]?|select(.transaction_id|startswith(\"txn_named_\"))|.ports|length]|add' on its intent_ir.json -> 0; SWD declares 13 signals dominated by SWCLK/SWDIO/TCK/TDI/TDO (2-wire serial). VLM (Q3, only if re-running the probe — RAM-heavy, ollama stop first, monitor, kill >=85%): qwen2.5vl:7b on picture-0011/0013 gave contradictory/redundant/hallucinated answers and loaded as 13 GB (host -> 87% used)."
---

**Measured + decided `2026-06-17` (`KG-ISF-TRANSACTIONS.2l`, measurement-first, read-only + a bounded VLM probe,
docs-only; owner-chosen fresh session).** This card closes the last `.2j`-recorded transaction candidate — "AXI/SWD
timing-diagram phase columns (VLM-tier)" — the honest way, and records WHY the VLM is the wrong lever.

## The question
`.2i`/`.2j` left AXI and SWD with **empty per-signal phase membership** (the `<qualifier> phase` prose names no
declared wire: AXI 0/4, SWD 0/63). The recorded next lever was reading phase columns off **timing-diagram crops**
with the production VLM (`qwen2.5vl:7b`, `[[project_llm_provider_ollama_qwen]]`). Per the scoring-rigor doctrine
(`[[feedback_scoring_rigor]]`) and the structured-first/best-wins doctrine
(`[[feedback_multi_strategy_best_wins]]`), measure before coding and weigh the cheaper deterministic cue first.

## Q1 (AXI) → GO, but DETERMINISTIC (the VLM lever is superseded)
AXI's transaction phases (address/data/response) ARE its channels, and the document's OWN caption-named tables
declare exactly the signals in each channel. Measured live over the persisted AXI EvidenceIR:
`table_signal_declaration_provenance` (411 entries) maps each signal to the channel table that declared it —
`B1.1 Write request channel signals`→26 `AW*` (write **address** phase), `B1.2 Write data`→15 `W*` (**data**),
`B1.3 Write response`→14 `B*` (**response**), `B1.4 Read request`→26 `AR*`, `B1.5 Read data`→18 `R*`,
`B1.6/B1.7`→snoop. So AXI per-signal phase membership is recoverable **without a VLM**, via the universal caption
cue `<role> channel signals` (role→phase: request→address, data→data, response→response) — the document's own
vocabulary, no name list (ADR 0006). This is the `.2m` candidate.

## Q2 (SWD) → honest DEGENERATE absence (no lever recovers it)
SWD is a 2-wire serial protocol (`SWCLK`+`SWDIO`); its packet phases (request/ACK/data/turnaround) are TIME
segments of bit-fields on the single shared `SWDIO` wire — confirmed visually by **Figure B4-1** (`Start│APnDP│RnW│
A│Parity│Stop│Park │Trn│ ACK │Trn│ WDATA[0:31] │Parity`, "Wire driven by: Host│Target│Host"). The phase names label
bit-fields (`Start`/`ACK`/`WDATA`), not declared signals; SWD's recognised phases all carry `signal_set=[]` and its
named transactions carry `ports=[]`. Per-signal phase membership is structurally degenerate — nothing to recover by
any lever; forcing it would fabricate.

## Q3 (VLM) → evidence-based NO-GO for a VLM-membership lever
Ground truth (read the crops): AXI `timing_diagram`s are generic handshake waveforms (`picture_0008`), credit-timing
examples (`picture_0013` ACLK/ARESETn/CRDT/VALID), or handshake-DEPENDENCY graphs (`picture_0011` `AW*`/`W*`→`B*`) —
none carries a per-declared-signal phase-column cue beyond the deterministic channel mapping. Bounded probe
(`qwen2.5vl:7b`, temp 0): on `picture_0011` the VLM was internally **contradictory** ("does not depict phases" then
listed address/write/response phases), merely re-stated the deterministic channel mapping, and **missed** the
figure's real content (the ordering arrows); on `picture_0013` it correctly said "no phases" but **hallucinated**
signal semantics (called both `CRDT` and `VALID` "write data valid"). **RAM:** the 7B VLM loaded as **13 GB** and
pushed the host to **87% used** — across the 85% autonomous-kill threshold (`[[feedback_ram_ceiling_monitor]]`);
`ollama stop` issued immediately, recovered to 73% free. The only VLM-unique signal in the diagrams is phase ORDER
(`picture_0011` arrows AW/W→B) = the `.2h` residual, already FSMGen-blessed as "don't fabricate order".

## Decision
The "AXI/SWD timing-diagram phase columns (VLM-tier)" candidate is RESOLVED — **replaced by a deterministic
AXI-family channel-membership lever** (`.2m`, RAM-safe, structured-first); SWD per-signal membership + the
VLM-membership lever are **NO-GO honest-absence**. The VLM-tier transaction frontier is exhausted. Gates:
read-only/docs-only — WIRE-BASED-100 untouched, ADR-0006 (the channel-caption cue is the document's own universal
vocabulary). `[[transaction-capture-census]]` · `[[project_kg_isf_transactions]]` · `[[project_kg_isf_completeness]]`.
