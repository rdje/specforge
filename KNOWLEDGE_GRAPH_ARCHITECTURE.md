# KNOWLEDGE_GRAPH_ARCHITECTURE
## Origin and purpose
This document crystallizes the architectural vision established in a working session (2026-04-03) through a detailed discussion about how signal direction, actor identity, and behavioral constraints should be extracted from chip protocol specification PDFs.

The core insight: **tables provide signal names; prose provides the semantic relations. A complete chip model requires both, fused into a knowledge graph.**

This document is the long-term reference for the knowledge graph extraction workstream. Any AI session resuming this work should read this document before touching EvidenceIR, SemanticIR, or the signal-resolve command.

---

## The fundamental model: two layers

A chip protocol specification PDF encodes two categories of information:

### Layer 1: Structural (static, spatial — the block diagram)
- Who the actors are (active modules, blocks, IP components)
- What signals connect them
- The direction of each signal relative to each actor
- Together: a typed directed graph = the block diagram

### Layer 2: Behavioral (dynamic, temporal — the waveforms and FSM)
- How signals change over clock cycles
- State machines: states, transitions, guards
- Timing constraints: setup/hold, latency, cycle counts
- The clock is the universal synchronization reference — all digital protocols are synchronous
- Together: the waveforms = the FSM = what generates the RTL

Both layers are distributed throughout the entire PDF. Neither is confined to a single section or a single modality.

---

## Actor identity is behavioral, not lexical

This is the most critical design principle.

"Manager", "master", "initiator", "Requester" are all the same concept: **an entity that initiates transactions**. "Subordinate", "slave", "completer", "target", "Responder" are all the same concept: **an entity that responds to transactions**.

The specific name used in any given spec is irrelevant to the extraction logic. What matters is what the entity DOES:
- If it DRIVES or ASSERTS signals → it is an initiator-role actor
- If it READS or SAMPLES signals → it may be a responder-role actor
- If it ROUTES signals → it is an infrastructure-role actor (decoder, arbiter, interconnect)

The pipeline must never be anchored to vocabulary. It must detect actors by their behavioral role in sentences.

Actors are always, always connected using signals. This is the fundamental invariant of any digital hardware specification. Every actor that appears in a spec will eventually be connected to at least one signal via a verb phrase.

---

## The fundamental unit: the triple

Every sentence in a spec that relates an actor to a signal can be decomposed into a triple:

```
(actor, relation, signal)
```

Real examples from AHB, APB, and AXI:
```
"PREADY is asserted by the slave"                       → (slave, drives, PREADY)
"RDATA is returned by the Completer"                    → (Completer, drives, RDATA)
"The Manager drives HTRANS and HADDR"                   → (Manager, drives, HTRANS), (Manager, drives, HADDR)
"The slave samples HADDR"                               → (slave, reads, HADDR)
"AWADDR carries the write address from the Manager"     → (Manager, drives, AWADDR)
"PREADY is deasserted by the Completer"                 → (Completer, drives, PREADY)
"The Requester must drive PSEL before asserting PENABLE"→ (Requester, drives, PSEL), (Requester, drives, PENABLE)
"HRESP is sampled by the Manager on the rising edge"    → (Manager, reads, HRESP)
```

One sentence may yield multiple triples. The verb and sentence structure encode the relation type.

---

## Direction is computed from the graph, not extracted directly

**Direction is always relative to a specific actor.** "PREADY is input" is meaningless. "PREADY is input_of[Manager]" is meaningful.

The current `direction_hint: Option<InterfaceSignalDirection>` in `InterfaceSignalRecord` is architecturally incomplete — it has no actor reference. It should be replaced or augmented by graph-derived direction.

Direction derivation from the graph:
```
For target actor A and signal S:
  direction = Output  if (A, Drives, S) relation exists with sufficient confidence
  direction = Input   if (A, Reads, S) relation exists, OR
                         (X, Drives, S) where X is a known actor and X ≠ A
```

The port list of actor A is:
- Outputs: all signals where (A, Drives, S) is in the graph
- Inputs: all signals where (A, Reads, S) is in the graph, or where another actor drives them

From this graph, a block diagram can be drawn immediately: actor nodes, signal edges, direction arrows.

---

## The relation vocabulary

Verbs split into two fundamental categories. The pipeline must recognize both active and passive forms.

### Drives (signal flows OUT of actor, signal is OUTPUT of actor)
Active forms: drives, asserts, activates, outputs, returns, generates, provides, sends, sources, initiates, produces, sets, puts, places, issues, presents, supplies
Passive forms: is driven by, is asserted by, is provided by, is returned by, is sourced by, is sent by, is generated by, is issued by, is set by, is produced by

### Reads (signal flows INTO actor, signal is INPUT to actor)
Active forms: reads, samples, monitors, accepts, receives, checks, observes, detects, captures, uses, latches
Passive forms: is read by, is sampled by, is monitored by, is accepted by, is received by, is captured by

### Transfer verbs (encode both sides simultaneously)
"A transfers X to B"              → A drives X, B reads X
"X carries data from A to B"      → A drives X, B reads X
"A sends X to B"                  → A drives X, B reads X
"B receives X from A"             → A drives X, B reads X

### The "is" + role pattern
"PREADY is an output of the slave"  → (slave, drives, PREADY)
"HTRANS is an input to the slave"   → (slave, reads, HTRANS)
"HADDR is the address from Manager" → (Manager, drives, HADDR)

---

## Tables vs prose: complementary, not redundant

### What tables provide
- Signal NAMES (reliably, always)
- Signal WIDTH (sometimes — numeric values like "1", "32"; parametric like "ADDR_WIDTH" are opaque)
- Source/direction sometimes in a "Source" or "Direction" column — but only when the column values are "output"/"input", which is NOT the case in AMBA 5 specs using Requester/Completer terminology

### What prose provides
- Actor-signal RELATIONS (verbs connect actors to signals)
- Direction resolution for signals whose table column uses non-standard values
- Width in prose form: "AWADDR is ADDR_WIDTH bits wide", "AWSIZE is 3 bits"
- Behavioral constraints: when signals change, what values they take

### Real examples of the gap (validated 2026-04-03)

APB Table 2-1: `PADDR | Requester | ADDR_WIDTH | Address`
- "Requester" is NOT "output" or "input" → the current direction parser returns None → signal dropped
- Fix: prose says "PADDR is driven by the Requester" → (Requester, drives, PADDR) → PADDR is output_of[Requester]

AXI tables: `AWADDR | 32 | - | Write address`
- No direction column at all
- Fix: prose says "The Manager drives AWADDR to specify the write address" → (Manager, drives, AWADDR)

AHB works by accident: its section headings say "Manager signals" and "Subordinate signals" which match the existing section-title heuristic. This is fragile and spec-specific.

**The correct solution is general: always extract actor-signal relations from prose, regardless of table structure.**

---

## The knowledge graph in the IR

### New record type in EvidenceIR

```rust
pub struct ActorSignalRelation {
    /// Normalized actor name as it appears in the spec (e.g. "slave", "Manager", "Requester")
    pub actor_name: String,
    /// Uppercase hardware signal name (e.g. "PREADY", "HTRANS", "AWADDR")
    pub signal_name: String,
    /// Whether the actor drives or reads the signal
    pub relation: RelationKind,
    pub automation_confidence: AutomationConfidence,
    pub source_statement_ids: Vec<String>,
}

pub enum RelationKind {
    /// Actor drives/asserts/outputs the signal — signal is OUTPUT of this actor
    Drives,
    /// Actor reads/samples/monitors the signal — signal is INPUT to this actor
    Reads,
}
```

### New EvidenceIR field

```rust
/// Actor-signal relation graph extracted from prose verb phrases.
/// Each record is a (actor, drives|reads, signal) triple.
/// Together these records form the structural knowledge graph of the spec.
#[serde(default)]
pub actor_signal_relations: Vec<ActorSignalRelation>,
```

### Changed SemanticIR direction derivation

Instead of the current approach (table-only, with section-title fallback):
1. First, check `actor_signal_relations` graph for the target actor and signal
2. If (actor, Drives, signal) found → direction = Output
3. If (actor, Reads, signal) found, or (other_actor, Drives, signal) found → direction = Input
4. Fall back to existing table-based synthesis only if graph has no entry

---

## New command: specforge signal-resolve

```
specforge signal-resolve <evidence-ir> [--vlm-provider ollama] [--vlm-model qwen2.5vl:7b]
```

Takes an EvidenceIR. For every signal in the spec without a resolved direction, attempts resolution through three tiers:

**Tier 2 — Prose pattern matching (deterministic, no LLM)**
Runs verb-pattern rules against all `extracted_statements` containing known signal names.
Produces `ActorSignalRelation` records with `AutomationConfidence::Medium`.

Verb patterns implemented:
- Passive drives: `"(signal) is (driven|asserted|provided|returned|sent) by (actor)"`
- Active drives: `"(actor) (drives|asserts|activates|outputs|generates|provides) (signal)"`
- Passive reads: `"(signal) is (read|sampled|monitored|accepted|received) by (actor)"`
- Active reads: `"(actor) (reads|samples|monitors|accepts|receives|checks) (signal)"`
- Transfer: `"(actor) (transfers|carries|sends) (signal) to (actor2)"` → actor drives, actor2 reads

**Tier 3 — LLM extraction (for complex sentences)**
New extraction type `signal_relation` added to `nlp-enrich` prompt.
For sentences where Tier 2 patterns don't match but actors and signals are clearly present.
LLM returns: `{"type":"signal_relation","actor":"Manager","signal":"HTRANS","relation":"drives"}`
Produces records with `AutomationConfidence::Low`.

**Convergence with Form 2**
The alias feedback loop: once an actor-signal triple is extracted in pass N, the actor name and signal name are added to the grounding context for pass N+1, improving LLM resolution of subsequent sentences.

---

## The behavioral layer: the synchronous clock-tick transfer model

Every behavioral constraint in a chip spec is a statement about signal values at specific instants relative to clock edges. This is the foundational physical model of synchronous RTL design.

### Clock ticks and the T-/T+ notation

A free-running clock has an infinite sequence of ticks at times `T0, T1 = T0+P, T2 = T0+2P, ...` where P is the clock period (constant for a non-drifting clock). Each tick is a zero-width instant corresponding to the posedge (rising edge) of the clock signal.

For any tick at time Tn:
- `Tn−` = the instant immediately BEFORE the tick = the stable value that registers see and capture
- `Tn+` = the instant immediately AFTER the tick = the new value that registers emit

### The two fundamental synchronous actions

**Drive/Assert at Tn**: the actor makes the signal take its new value starting at `Tn+`. The signal holds that value until at least `T(n+1)−`.

**Sample/Read at Tn**: the actor captures the value that the signal had at `Tn−` (the stable value just before the tick).

### The minimum synchronous transfer: 1 clock cycle

```
T0:  Actor A drives signal S    →  S becomes valid at T0+
T1:  Actor B samples signal S   →  B reads S value at T1−  (= what A put there at T0+)
```

B CANNOT sample at T0 — the new value was not present at T0−. T1 is the earliest possible read. This is why 1 clock cycle (2 consecutive ticks) is the minimum transfer latency in any synchronous system. It is an architectural invariant, not a performance number.

Multi-cycle transfers: B may sample at `T2−`, `T3−`, ..., `TN−` based on the protocol handshake. The protocol defines WHEN B is allowed to sample. AHB HREADY, AXI READY/VALID, APB PREADY — all implement this latency contract.

### Mapping spec sentences to the clock-tick model

Every behavioral constraint in a chip spec is an implicit statement about signal values at `Tn−` or `Tn+`. The spec uses natural language; the clock-tick model is the physical interpretation:

| Spec sentence | Physical meaning |
|---|---|
| "HADDR must be stable when HREADY is LOW" | ∀ Tn : HREADY(Tn−) = LOW → HADDR(Tn+) = HADDR(Tn−) |
| "HTRANS must be NONSEQ during the address phase" | ∀ Tn in address phase : HTRANS(Tn−) = NONSEQ |
| "PREADY is sampled on the rising edge of PCLK" | B reads PREADY(Tn−) at each tick Tn |
| "The transfer completes within 2 clock cycles" | latency(A drives S → B samples S) ≤ 2P |
| "HWRITE is tied HIGH for the entire burst" | ∀ Tn in burst : HWRITE(Tn−) = HIGH → HWRITE(Tn+) = HIGH |

### VALID/READY handshake is the clock-tick model in action

The VALID/READY (or HREADY/HTRANS) handshake used in AHB, AXI, and APB is an implementation of the clock-tick transfer model with variable latency:
- Sender asserts VALID at Tn (S valid at Tn+)
- Receiver asserts READY at Tm (Tm ≥ Tn)
- Transfer captured on the FIRST tick where VALID(Tn−) = HIGH AND READY(Tn−) = HIGH
- The latency is (m − n) clock cycles, bounded by protocol constraints

### What the existing IR records represent

The current IR records are correct but implicit about the clock-tick model:

`SignalConstraintRecord { subject: HADDR, kind: MustBeStable, condition: "HREADY is LOW" }`
⇒ Physical meaning: ∀ Tn : HREADY(Tn−) = LOW → HADDR(Tn+) = HADDR(Tn−)

`ConditionalRuleRecord { antecedent: "HREADY is asserted", consequent_signal: HTRANS, action: "must be NONSEQ" }`
⇒ Physical meaning: ∀ Tn : HREADY(Tn−) = HIGH → HTRANS(Tn−) = NONSEQ

When the RTL code generator or assertion generator processes these records, it applies the clock-tick model to produce:
```systemverilog
assert property (@(posedge HCLK) !HREADY |-> $stable(HADDR));
assert property (@(posedge HCLK) HREADY |-> HTRANS == NONSEQ);
```

The knowledge graph (Layer 1: structural) and the behavioral records (Layer 2: temporal / clock-tick) together constitute a complete model of the protocol — sufficient to generate synthesizable RTL and verification assertions.

---

## Pipeline validation results (2026-04-03)

### AHB (IHI0033_C, 2021) — 86/100 GOOD

Works well because:
- Signal tables have "Manager signals" / "Subordinate signals" section headings
- These headings match the existing section-title heuristic → direction inferred
- 17 declared signals (100% direction, 58% width)
- 71 NLP constraints (signal + conditional)
- 46 residual NormativeStatements (structural/architectural, no named signal)

AHB works **by coincidence** of vocabulary, not by design. The pipeline would fail on any spec that uses different headings.

### APB (IHI0024_E, 2023) — 35/100 NEEDS IMPROVEMENT

Root cause: APB Table 2-1 "Source" column contains "Requester" and "Completer" — not "output" and "input". The current direction cell parser returns None → all signals dropped → 0 declared signals → 0% direction coverage.

Quick fix available (5 lines): extend the direction cell parser to recognize Requester→output, Completer→input, initiator→output, responder→input, clock→input, reset→input.

Real fix (this workstream): extract actor-signal relations from APB prose directly, independent of table column values.

### AXI (IHI0022_L, 2025) — 85/100 (misleading)

Score is inflated: only 1 signal declared (out of ~100+), but 1/1 = 100% direction coverage, making the score look good.

Root cause: AXI signal tables have no "Source" or "Direction" column at all. The format is `Name | Width | Default | Description`. Direction must come entirely from prose.

Prose examples that the knowledge graph approach would handle:
- "The Manager drives AWADDR, AWSIZE, and AWBURST to define the transfer characteristics"
- "RDATA is returned from the Subordinate to the Manager on the read data channel"
- "BVALID is driven by the Subordinate to indicate a valid write response"

With actor-signal relation extraction, all ~80+ AXI signals would get direction from prose.

---

## Known bugs to fix alongside the knowledge graph work

### Bug 1 — Alias extraction garbage filter (low priority, 2-line fix)
`extract_alias_phrase()` in `nlp_enrich.rs` produces "- the address" from a sentence that starts with a markdown list marker "- ". The phrase starts with "-" which is not a valid noun phrase.
Fix: reject phrases where the first word starts with "-", "|", "#", or other markdown tokens.

### Bug 2 — APB direction from source column (medium priority, 5-line fix)
In `synthesize_signal_declarations()` in `evidence.rs`, the direction cell value parser only recognizes "output" and "input" literally.
Fix: extend to also recognize:
- "requester", "initiator", "manager", "master" → "output"
- "completer", "responder", "subordinate", "slave", "target" → "input"
- "clock", "reset", "system" → "input" (global/system signals)

Bug 2 is a quick workaround that fixes APB immediately. The knowledge graph approach (signal-resolve command) is the permanent general solution.

---

## Implementation sequence

### Phase 1: Quick fixes (1-2 hours)
- Fix Bug 1 (alias garbage filter)
- Fix Bug 2 (APB direction from source column)
- Re-run APB pipeline → expected score ~75/100

### Phase 2: Tier 2 prose extraction (1-2 days)
- Add `ActorSignalRelation` record type to `source.rs`
- Add `actor_signal_relations: Vec<ActorSignalRelation>` to `EvidenceIr`
- Implement `extract_actor_signal_relations()` in `evidence.rs` with verb-pattern rules
- Use in `SemanticIr::build_interfaces()` to compute direction from graph
- Re-run APB and AXI → expected: ~80+/100 for both

### Phase 3: Tier 3 LLM extraction (1-2 days)
- Add `signal_relation` extraction type to `nlp-enrich` prompt
- Implement `specforge signal-resolve` command (or integrate into `nlp-enrich`)
- Re-run all three specs → expected: 85-90+/100 across the board

### Phase 4: Direction model in SemanticIR (2-3 days)
- Replace `direction_hint: Option<InterfaceSignalDirection>` with actor-relative model
- `InterfaceSignalRecord` carries `drives_actors: Vec<String>` and `read_by_actors: Vec<String>`
- Direction for adapter lowering computed at adapter time, relative to the target actor
- This makes the SystemVerilog/Verilog adapter possible for multi-actor specs

---

## Future: Level 4 NER/RE model

The current Tiers 2+3 build a training corpus. Every `ActorSignalRelation` record with human-reviewed accuracy is a labeled training example. Once 5K+ reviewed examples from 10+ specs exist, a fine-tuned NER+RE model (BERT/DeBERTa-class) could replace Tiers 1+2+3 entirely:
- No keyword lists needed
- No LLM calls needed
- Generalizes across all AMBA specs, all vendors
- Handles passive voice, negation, ellipsis, co-reference natively

This is the Level 4 SOTA target described in `EXTRACTION_ARCHITECTURE.md`.
