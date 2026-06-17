# Relation-completeness measurement (`KG-ISF-COMPLETENESS.3`, `2026-06-17`)

**Question (owner-directed, north-star bar #2):** several documents carry actors and
constraints but **zero** `actor_signal_relations`. Is that a recoverable relation-extraction
gap, or does the document genuinely not state actor↔signal relations? Read-only census over
all 78 persisted `evidence_ir.json` + 36 `intent_ir.json` artifacts, plus content sampling.

## Method

1. Intent-level census: count `actor_signal_relations` per `intent_ir.json` → flagged the
   docs at 0.
2. Evidence-level census: count `actor_signal_relations` per `evidence_ir.json` for the same
   docs, plus `normalized/` bundle presence and artifact mtimes.
3. Compared evidence-vs-intent relation counts + mtimes to separate **staleness** from
   **genuine absence**.
4. Content sampling: for the genuine-0 docs, grepped the extracted statements for drive/read
   cues near signal-shaped tokens, and counted declared wire signals.
5. Empirical proof: rebuilt one stale doc (`semantic`→`intent`, deterministic) and re-counted.

## Finding — two distinct causes, the 0-relation docs are NOT an extraction gap

### (A) STALE IntentIR — recoverable, fabrication-free

The canonical `intent_ir.json` is **stale relative to its `evidence_ir.json`** for the
wire/bus docs: the relations were extracted into EvidenceIR but the downstream IntentIR was
not rebuilt afterward, so it still shows the older 0.

| doc | evidence rels | intent rels | intent mtime | evidence mtime |
|---|---|---|---|---|
| `tilelink_1_7_1` | 39 | **0** | 05-16 | 06-08 |
| `tilelink_1_8_0` | 40 | **0** | 05-16 | 06-08 |
| `um10204` (I2C) | 17 | **0** | 06-15 | 06-15 |
| `wbspec` (Wishbone) | 1 | **0** | 05-16 | 06-08 |

Plus a broader "intent older than evidence" set whose counts happen to match but whose intent
predates the evidence: `gic_600`, `mmu_700`, `ihi0082` (ATS), `dti`, `opencapi`×3, `usb4`.

**Empirical proof (live, deterministic — no LLM, no Docling, RAM steady 77%):** rebuilding
`tilelink_1_7_1` `semantic`→`intent` recovered:

```
BEFORE  int actor_signal_relations: 0
AFTER   int actor_signal_relations: 39  | actor_ports: 69 | actors: 40 | connected: 40
```

Cause is **operational, not a code bug**: EvidenceIR was rebuilt under a sweep
(`.12a`/`.12b`/CORPUS-PATTERN-REUSE) without cascading the downstream stages. `converge`
rebuilds the whole chain; the individual `evidence` command does not auto-cascade.

### (B) HONEST ABSENCE — the register/command/coherency docs declare ~0 wire signals

These docs have 0 relations **even at EvidenceIR**, and content sampling shows why: they have
essentially no wire signals to relate.

| doc | declared wire signals | what the "drive/read cues" actually are |
|---|---|---|
| `nvme` | 0 | ToC entries (`Figure 122: SGL Read Example`), an address (`153 rd Drive`) |
| `risc_v_iommu` | 0 | register-access legends (`RO Read-only`, `RW Read-Write`, `RW1C`) |
| `ccix` | 0 | agent-MESSAGE prose (`Request Agent performs read and write transactions`) |
| `intel_virtualization` (VT-d) | 0 | register/structure descriptions |
| `i2s` | 3 | electrical-drive prose only (`driven by a slow clock`, `drive one TTL input`) |

These are register / command / coherency protocols. Their design intent is captured by
SpecForge in **other** surfaces — `register_records` (e.g. NVMe's 143 register tables),
`message_field_records` (CCIX's 161 byte-offset fields), and `transactions` — **not** by
actor↔signal drive/read relations. **0 actor-signal relations is correct**; minting relations
there would fabricate (the exact north-star caution `[[agent-surface-defect-taxonomy]]`).

## Genericity insight

The `actor_signal_relations` surface is **intrinsically wire-protocol-shaped** ("agent drives
SIGNAL"). For register/message/coherency protocols, *relation*-completeness is the wrong bar
dimension; *register*- and *message-field*-completeness is the right one. A doc legitimately
scores 0 on the relation bar while being complete on its own dominant surface — and that is
honest, not a miss.

## Follow-ups (frontier)

- **(i) Corpus refresh (owner gap #2):** rebuild the stale downstream stages so the recovered
  relations land in the canonical local corpus; surface which docs need re-ingest (those
  lacking a `normalized/` bundle — ~24 of the 30 0-relation evidence docs — cannot be
  evidence-rebuilt without the source PDF + Docling).
- **(ii) Stage-staleness detector (candidate code slice):** `validate` should warn when a
  downstream artifact is stale relative to its upstream (an intent silently carrying fewer
  relations than its evidence is exactly the kind of incompleteness the north star says must be
  surfaced, not hidden). Generic, ADR-0006, no name list.

## Reverify

```
# Staleness (evidence has relations, intent shows 0):
python3 -c "import json; e=len(json.load(open('generated/evidence_ir/tilelink_1_7_1_specification/evidence_ir.json'))['actor_signal_relations']); i=len(json.load(open('generated/intent_ir/tilelink_1_7_1_specification/intent_ir.json'))['actor_signal_relations']); print('evidence',e,'intent',i)"
# Recovery (deterministic rebuild):
./target/release/specforge semantic generated/evidence_ir/tilelink_1_7_1_specification/evidence_ir.json
./target/release/specforge intent generated/semantic_ir/tilelink_1_7_1_specification/semantic_ir.json
# Honest absence (0 declared wire signals on the register/command docs):
python3 -c "import json,re; d=json.load(open('generated/evidence_ir/nvme_base_specification_2_0a_2021_07_26/evidence_ir.json')); print('nvme declared signals', len({re.match(r'Signal (\w+) ',s['text']).group(1) for s in d['extracted_statements'] if s['text'].startswith('Signal ')}))"
```
