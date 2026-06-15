---
id: agent-surface-defect-taxonomy
title: The KG agent surface has 3 structural defect classes (junk / fragment / zero-evidence); the "disconnected real agents" are measured fabrication-risk, not a recoverable gap; the fix must be per-doc evidence-keyed (ADR 0006)
answers:
  - "why do producer/consumer/receiver/transmitter actors carry 0 ports and 0 relations"
  - "are the disconnected/unconnected KG agents a recoverable relation gap or false positives"
  - "should specforge connect producer/consumer/etc. to their signals"
  - "what are the agent-surface precision and completeness defects (KG-ISF-COMPLETENESS.1)"
  - "why is For / Then it / is recommended / next / HPROT bit / TREADY input minted as an actor"
  - "how should the agent-identity / actor noise gate stay agnostic (no name list, ADR 0006)"
  - "where do zero-port actors come from in the IntentIR actor surface"
  - "is dropping a zero-evidence actor safe across AXI vs AXI-Stream"
date: 2026-06-16
tags: [kg-isf-completeness, actors, agent-surface, actor-signal-relations, precision, completeness, consolidation, adr-0006, measured, wire-based-100]
evidence: docs/research/agent-surface-fidelity-measurement.md (full per-doc census + source-text resolution); crates/specforge/src/ir/evidence.rs (extract_subject_phrase / extract_actor_phrase / normalize_relation_actor_name — relation-subject capture); crates/specforge/src/ir/semantic.rs (build_actors Phase-2 role-term scan ~:2727, build_actor_ports ~:2885); docs/tasks/KG-ISF-COMPLETENESS.md (.1)
reverify: "python3 over generated/intent_ir/{ihi0022_l_*AXI, ihi0051_b_*AXI-Stream}/intent_ir.json — count actor_ports + actor_signal_relations per actor_name: AXI 'transmitter' = 0/0 (phantom) while AXI-Stream 'Transmitter' = 22/23 (real) → same term, opposite status → the drop/keep rule MUST be 'in this doc' evidence, never a global name list. Then grep the zero-evidence terms as drive/read subjects near a known signal across the wire docs → ~0 real hits (fabrication risk)."
---

**Measured `2026-06-16` (read-only over the wire-doc IntentIR; `docs/research/agent-surface-fidelity-measurement.md`).**
The KG/IntentIR actor surface carries two coexisting defects with a SHARED root in relation-subject
capture, splitting into three structural classes:

- **Class A — Junk** (precision): the captured "subject" is a clause / function-word / descriptor, not
  an agent — `For components`, `HPROT bit`, `is recommended` (the real subject `Manager` is in the
  that-clause), `section`, `next`, `two-cycle response`, `TREADY input`, `is permitted`, `Note`,
  `Then it`. These carry ports (3–4 each), so the naive "drop orphan actors" filter FAILS. → reject by a
  STRUCTURAL agent-identity gate (leading non-agent token class; no name list, ADR 0006).
- **Class B — Fragment of a real agent** (completeness): subject + trailing verb/adverb, or a coordinated
  "X and Y" — `Subordinate extends`/`Subordinate then`→`Subordinate`; `address decoder`/`decoder also`→
  `decoder` ("*An address decoder provides HSELx*" is a genuine AHB fact); `Transmitter interface`→
  `Transmitter`; `Subordinate and decoder`→split. The real agent's relations are stranded under the
  fragment, leaving the clean role-term at 0/0 (AHB `decoder` reads 0/0 only for this reason). →
  consolidate fragment→canonical.
- **Class C — Zero-evidence role-term** (0 ports AND 0 rels): minted by the SemanticIR Phase-2 role-term
  scan (`build_actors`) for generic vocabulary the document mentions but never wires (AXI
  `producer`/`consumer`/`receiver`/`transmitter`/`requester`/`target`/`agent`/…).

**The decisive correction:** the `.0` hypothesis "real agents named-but-never-connected = a recoverable
relation gap" is DISPROVEN for the wire docs — a Class-C term is a drive/read subject next to a known
signal ≈0 times (the apparent AXI `source` hit is the noun "source of read data" whose verb-subject is
`Subordinate`; the SWD `state machine` hit is "in a state"). **Synthesizing relations for them would be
fabrication.** The genuine completeness win is Class-B consolidation; Class-C is dropped per-doc.

**Genericity guardrail (proven):** `transmitter`/`receiver` are 0/0 phantoms in AXI but 22/23 real agents
in AXI-Stream — same token, opposite status. Any drop/keep rule MUST key off "0 ports AND 0 relations in
*this* document", never a global term list (ADR 0006).

See [[prose-signal-capture]] (the existing agent-definition grammar this gate complements),
[[actor-signal-direction-passive-active-handled]], [[nlp-coordination-already-handled]] (object
coordination already handled; subject coordination is the Class-B "X and Y" split), and
[[extractor-path-architecture]].
