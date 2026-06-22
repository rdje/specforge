# Cat-3 (platform / system-IP topology & integration) ISF-lowering decision packet — `DOC-INTENT-TAXONOMY.4c`

- Tree leaf: `DOC-INTENT-TAXONOMY.4c`
- Date: `2026-06-23`
- Type: measurement + decision packet (read-only, docs-only — no Rust code, no canonical-artifact mutation)
- Reinforces: `[[project_kg_isf_completeness]]`, `[[project_doc_intent_taxonomy]]`,
  `[[feedback_verify_fsmgen_before_fr]]`, `[[feedback_isf_no_hacks]]`, `[[feedback_scoring_rigor]]`

## The question

> Does category-3 (platform / system-IP topology & integration) intent — component connectivity, clock/reset
> infrastructure, integration contract — **map onto an existing ISF construct**, or does it need a **new ISF
> abstraction** (a verified FSMGen feature request)?

The `.2` scorecard scored cat-3 as **PARTIAL** ("infrastructure signals + actor ports lower; topology stays at
the hint level"), and the `.0` taxonomy flagged cat-3 topology as "likely needs another FSMGen abstraction."
`.4a.ii` then shipped register **bit-field** lowering — and cat-3 carries the corpus's largest register-field
volume (CoreSight SoC-600 alone ~3,250). This packet measures cat-3's *distinctive* intent (topology, not
registers) and decides the construct question from evidence, before any cat-3 code or FR.

## Method (read-only, reproducible)

Cat-3 is **15 docs** (the `.1` census: CoreSight SoC-600 TRMs, GIC-600/400, CoreSight Base System
Architecture, interconnect fabrics, …). I profiled three representative docs spanning the cat-3 range — a
register/topology-heavy TRM (CoreSight SoC-600, `100806_0701_17…`), a connectivity-heavy IP (GIC-600,
`100336_0106_00…`), and a prose architecture spec (CoreSight Base System Architecture, `den0068…`) — reading
the persisted `intent_ir.json` topology-relevant surfaces (`signal_connectivity`, `infrastructure_signals`,
`actors`, `actor_ports`, `register_records`). I then re-verified what the current FSMGen ISF (`subs/fsmgen`
pin `d327129b7`) declares for cross-component composition / topology / connectivity.

## Measured evidence

### Cat-3 docs DO carry a structured topology surface (correcting "hint-level")

| Doc | `actors` | `actor_ports` | `signal_connectivity` | `infrastructure_signals` | `register_records` |
|---|---|---|---|---|---|
| CoreSight SoC-600 | 60 | 57 | **6** | 0 | 833 |
| GIC-600 | 55 | 147 | **66** | **2** | 33 |
| CoreSight Base System Arch | 5 | 0 | 0 | 0 | 0 (88 prose `interfaces`) |

- `signal_connectivity` is a **typed producer→consumer graph**, e.g. GIC-600
  `{signal_name: "DATA", connectivity_class: "protocol", producer_actor_names: ["MISC ignores"],
  consumer_actor_names: ["cache several", "pmu_int"]}` — i.e. *which component drives which signal to which
  components*. This is genuine captured topology, not merely a prose hint, so the `.2` "hint-level"
  characterization is refined to **captured-but-sparse-and-unlowered**.
- `infrastructure_signals` carries a clock/reset distribution shape: `infrastructure_topology`,
  `distributed_to_actor_ids`, `distribution_status`, `recovered_source_actor_ids`, `kind` — the clock/reset
  distribution tree, structurally.

### …but the topology capture is SPARSE and NOISY

- CoreSight SoC-600 has **60 actors** but only **6** `signal_connectivity` edges (some with empty consumers) —
  far too few to faithfully represent a 60-component subsystem's interconnect.
- Actor names come back `None` in the canonical surface and connectivity names carry extraction noise
  (escaped `pmu\_int`, generic `MISC ignores`, `cache several`).
- So even if ISF could express it, lowering 6 sparse edges would synthesize an unfaithful sliver of the
  topology — a **capture-recall** problem must be quantified before topology is called lowerable.

### FSMGen ISF (pin `d327129b7`) has no declarative static-topology construct

- ISF **composition is transaction-level only**: `(do child)` lets a transaction call another transaction
  *within the same actor* (`13f-composition.md`). It is behavioral sequencing, not component instantiation.
- FSMGen's backlog (`14-feature-backlog.md`) **does** have an active multi-actor frontier — the "ATL"
  generated-child work: *generated ATL tops, actor-to-actor generated-child routing, HDL child wiring,
  parent/child/top artifacts, network topology orchestration*. But this is **behavioral**: it wires children
  *generated from transaction composition* (`spawn`/`do` fan-out, data-movement handoff ports), not a
  **declarative static interconnect netlist** read from a TRM's "component X connects to component Y over bus
  Z" graph. It is not a drop-in home for `signal_connectivity`.
- SpecForge's own ISF emit is a **single-initiator-actor** model (`KG-ISF-COMPLETENESS.2a.ii`:
  `select_initiator_actor` collapses the actor graph to one initiator and names the module after it). So a
  cat-3 doc's 60 captured components do not each become an ISF actor — cross-component topology is structurally
  absent from the emit *by design*, not merely undropped.

## Decision

1. **Cat-3's register intent already lowers** — register maps + bit-fields (`.4a.ii`; SoC-600 ~3,250 fields,
   GIC-600 33 registers), infrastructure signals where present, and actor ports use the same constructs as
   cat-2. **Cat-3's register half is not the gap.**

2. **Cat-3's distinctive intent — component topology / connectivity / clock-reset distribution — is captured
   as a typed surface** (`signal_connectivity`, `infrastructure_signals`), **but ISF has no declarative
   static-topology construct to lower it into**, and FSMGen's nearest frontier (ATL multi-actor composition)
   is **behavioral transaction-orchestration wiring, not a declarative IP-interconnect netlist** (verified
   against the backlog). Lowering topology would additionally require a **multi-actor ISF emit** — a real
   architectural change, not an emitter tweak.

3. **No FSMGen FR is filed yet — and that is the honest call, not deferral for its own sake.** Two measured
   reasons make an FR premature: (a) the topology *capture* is sparse and noisy (6 connectivity edges across
   SoC-600's 60 actors; `None`/escaped actor names), so it is not yet faithful enough to be worth a new ISF
   construct — lowering it now would synthesize an unfaithful sliver; and (b) ISF is, by the project's own
   architecture, a **per-actor** intent format (one `.isf` → one FSMGen module), so static cross-component
   topology may be deliberately the *integrator's* concern **above** per-module synthesis — a scoping question
   to resolve **with** FSMGen, not to assume. Filing a construct FR before either is settled would breach
   `[[feedback_verify_fsmgen_before_fr]]`.

**Net / next lever:** topology stays an **honest residual**; the buildable next step is a **measurement, not
code or an FR** — `.4c.i`: quantify topology-capture recall (`signal_connectivity` + `infrastructure_signals`
density and name quality) across all 15 cat-3 docs. Only once capture is shown faithful does the construct
decision become real: either a **verified FSMGen FR** for a declarative static-topology/connectivity ISF
construct + a multi-actor emit (if the ATL frontier still does not subsume it), **or** confirmation — with
FSMGen — that static topology is deliberately above ISF's per-actor scope, making it an **honest non-target**
(like cat-5 PHY). Measurement decides; nothing is fabricated and no speculative FR is filed.

## Genericity (ADR 0006)

The decision rests on structural evidence (typed topology surfaces; their density; FSMGen's published
composition grammar) — no chip/vendor/protocol-instance name list. The spun-out `.4c.i` recall measurement
must likewise be a structural density/quality gauge, not a per-document name list.

## Gates (this leaf)

- No Rust code, no canonical-artifact mutation → the wire golds / `kg-bench` / emitted `.isf` are
  byte-identical by construction (WIRE-BASED-100 orthogonal).
- `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green;
  knowledge-map derive-and-diff in sync after adding the `[[cat3-topology-isf-lowering-decision]]` fact card.
- Objectively measured, per-item demonstrated (`[[feedback_scoring_rigor]]`): every count above is read
  directly off the persisted corpus and reproducible from `generated/intent_ir/<key>/intent_ir.json`.
