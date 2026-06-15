# Agent-surface fidelity — measured baseline and design (`KG-ISF-COMPLETENESS.1`)

Read-only measurement over the persisted wire-doc IR (`2026-06-16`), executed before any
code per the tree's measurement-first rule. It characterizes the agent-surface defects the
owner's north star targets (the KG/IntentIR must capture all *real* agents and connect them
to their signals), decides the disconnected-agent cause by evidence, and fixes the design for
the `.1a`/`.1b` code slices.

Method: `python3` over `generated/intent_ir/<doc>/intent_ir.json` (actors / `actor_ports` /
`actor_signal_relations`) joined to `generated/evidence_ir/<doc>/evidence_ir.json`
(`extracted_statements`) for the source text behind each relation. No binary, no model, no
mutation — RAM-safe.

## 1. Per-actor port/relation census (wire docs + AXI-Stream guardrail)

Each row is one `actor_name`; `ports` = `actor_ports` count, `rels` = `actor_signal_relations`
count for that actor in that document.

**AXI** (`ihi0022_l`, actors=24, ports=371, rels=348)
- Real, connected: `Manager` 169/168, `Subordinate` 168/166, `interconnect` 5/3.
- Precision noise (carry ports, not agents): `For` 4/2, `with write` 4/2, `exclusive` 3/1,
  `instruction` 3/1, `monitor` 3/1, `Non-volatile Memory` 3/1, `Note` 3/1, `Shareable` 3/1,
  `Then it` 3/1.
- Zero-evidence (0/0): `agent`, `channel`, `consumer`, `controller`, `device`, `producer`,
  `receiver`, `requester`, `server`, `source`, `target`, `transmitter`.

**APB** (`ihi0024_e`, actors=10, ports=83, rels=69)
- Real: `Requester` 32/30, `Completer` 32/30.
- Precision noise: `APB protocol` 4/2, `ensures` 4/2, `Exit from` 4/2, `For` 4/2,
  `three levels` 3/1.
- Zero-evidence: `controller`, `source`, `state machine`.

**AHB** (`ihi0033_c`, actors=25, ports=100, rels=66)
- Real: `Subordinate` 25/23, `Multiplexor` 9/7, `Manager` 8/6, `Exclusive Access Monitor` 4/2.
- Fragment of a real agent (consolidate): `Subordinate and decoder` 6/4 (split),
  `address decoder` 4/2 (→`decoder`), `decoder also` 4/2 (→`decoder`),
  `Exclusive Access Monitor and Subordinate` 4/2 (split), `Subordinate extends` 4/2
  (→`Subordinate`), `Subordinate then` 1/1 (→`Subordinate`).
- Precision noise (junk): `For components` 4/2, `HPROT bit` 4/2, `is recommended` 4/2 (real
  subject `Manager` is inside the that-clause), `next` 4/2, `next access` 4/2, `section` 4/2,
  `two-cycle response` 4/2, `bus` 3/1.
- Zero-evidence: `agent`, `channel`, `controller`, `decoder` (**real but stranded** — see §3),
  `device`, `receiver`, `source`.

**SWD / ADIv6** (`ihi0074_a`, actors=23, ports=23, rels=26)
- Real-ish: `debugger` 3/3, `target` 2/4, `DP` 2/2.
- Noise/fragment: `details about` 2/2, `DP Access` 2/2 (→`DP`?), `IR register` 2/2,
  `number of` 2/2, `Class x` 1/2, `DbgSwEnable flag` 1/1, `field` 1/1, `read-only field` 1/1,
  `system has` 1/1, `then` 1/1, `watcher circuit` 1/1, `serializer` 1/1.
- Zero-evidence: `agent`, `controller`, `device`, `host`, `requester`, `sink`, `source`,
  `state machine`.

**AXI-Stream** (`ihi0051_b`, the genericity guardrail) — actors=9, ports=59, rels=54
- Real, connected: `Transmitter` 22/23, `Receiver` 22/22.
- Noise/fragment: `is permitted` 7/5, `Transmitter interface` 4/2 (→`Transmitter`),
  `TREADY input` 4/2 (junk — a signal, not an agent).
- Zero-evidence: `agent`, `channel`, `controller`, `source`.

## 2. The defect taxonomy (three structural classes)

Resolving each noise actor's relations to source text shows the noise is not one phenomenon:

- **Class A — Junk** (no agent: the captured "subject" is a clause / function-word /
  descriptor). `For components` ← "*For components that support…*"; `HPROT bit` ← "*each HPROT
  bit*"; `is recommended` ← "*It is recommended that a Manager sets HPROT[0] HIGH*" (the real
  subject `Manager` is in the that-clause); `section`, `next`, `next access`, `bus`,
  `two-cycle response`, `TREADY input`, `is permitted`, `Note`, `Then it`. → **reject**
  (precision).
- **Class B — Fragment of a real agent** (subject + trailing verb/adverb, or a coordinated
  "X and Y"). `Subordinate extends`/`Subordinate then` → `Subordinate`; `address decoder`/
  `decoder also` → `decoder` ("*An address decoder provides HSELx*" is a **genuine** AHB
  fact); `Transmitter interface` → `Transmitter`; `Subordinate and decoder` → split. →
  **consolidate** (completeness).
- **Class C — Zero-evidence role-term** (0 ports AND 0 rels; minted by the SemanticIR Phase-2
  role-term scan, `build_actors`, semantic.rs:2727). Generic vocabulary the document mentions
  but never connects to a wire. → see §3.

## 3. Decisive measurement: the disconnected agents are NOT recoverable (no source evidence)

The tree's `.0` hypothesis was *"real agents `producer`/`consumer`/`receiver`/`transmitter`
are named but never connected — a relation gap to fill."* Measured and **corrected**: across
all four wire docs, a Class-C actor appears as the subject of a drive/read verb next to a
**known signal** essentially **zero** times:

- The only AXI "hit" is `source` in "*Even if a **Subordinate** has only one source of read
  data, it must assert the RVALID signal*" — `source` is the noun "source of read data"; the
  verb's real subject is `Subordinate`. False match.
- The only SWD "hit" is `state machine` in "*this state machine is in either the
  Test-Logic-Reset state…*" — it is *in a state*, not driving a wire. False match.
- `transmitter` (AXI) has 8 subject-of-verb mentions but **0** beside any AXI signal — the AXI
  doc discusses it generically (coherency/DVM prose), never as a port owner.

So for the wire docs, Class-C terms are **generic-vocabulary false positives**, not
disconnected real agents. "Connecting" them to signals would be **fabrication** (no grounding;
violates ADR-0006 honesty). The genuine completeness win is **consolidation** (Class B): AHB
`decoder` reads 0/0 only because its real relations (`drives HSELx`) are stranded under the
fragment forms `address decoder` and `decoder also`.

## 4. Genericity guardrail (proven): the rule must be per-doc evidence-keyed, never a name list

The same token flips status by document: `transmitter`/`receiver` are zero-evidence phantoms
in **AXI** (0/0) but fully-connected real agents in **AXI-Stream** (`Transmitter` 22/23,
`Receiver` 22/22). Therefore a Class-C drop rule MUST be **"0 ports AND 0 relations in *this*
document"** — purely structural/evidential, with **no global term list** (ADR 0006). That rule
drops AXI's phantom `transmitter` while keeping AXI-Stream's real `Transmitter`.

## 5. Design for the code slices (each measurement-first, WIRE-BASED-100 a hard gate)

- **`.1a` — precision structural agent-identity gate.** At the relation-subject extraction
  point (evidence.rs `extract_subject_phrase` / `extract_actor_phrase` /
  `normalize_relation_actor_name`), reject Class-A subjects by *structure*: function-word /
  verb / adverb-led, or a clause fragment, or a descriptor that is not an agent-shaped noun
  (`For`, `Then it`, `is recommended`, `next`, `section`, `two-cycle response`, `HPROT bit`,
  `TREADY input`, `is permitted`). Universal grammar only — characterize the shared structure
  (leading non-agent token classes) and gate on it, not on names.
- **`.1b` — consolidation + zero-evidence honesty.** (i) Normalize Class-B fragments to the
  canonical agent token (strip a trailing verb/adverb; map "X interface"→"X"); (ii) split a
  coordinated "X and Y" subject; (iii) drop Class-C zero-evidence actors **per this doc's
  evidence** (0 ports AND 0 rels) — first re-checking their provenance (`responsibilities`,
  any non-drive/read "defined-as-agent" grounding) so a genuinely-declared-but-unwired agent
  is preserved. Explicitly **do not** synthesize relations for Class-C terms (measured
  fabrication risk).

## 6. What this does NOT claim

This measured only the four wire docs + AXI-Stream (the deeply-extracted protocol set the bar
gates on). The broader protocol corpus is scoped for `.2+`. The taxonomy and the per-doc
evidence-keyed rule are expected to generalize, but each code slice re-measures on its own
gold before landing.
