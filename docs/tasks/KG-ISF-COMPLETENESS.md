# KG-ISF-COMPLETENESS: the KG/IntentIR must be COMPLETE enough to lower faithfully to ISF

## Metadata

- Tree ID: `KG-ISF-COMPLETENESS`
- Status: `active`
- Roadmap lane: `R15`/`R16` (extraction quality / design-intent capture, ISF-fidelity lens)
- Created: `2026-06-16`
- Parent context: owner directive (`2026-06-16`, a multi-message exchange): *"Extracting for extracting
  is not the goal. In fine we need the IntentIR to be complete and contain all the necessary information
  to be lowered to ISF"*, *"the KG shall contain everything (actors, agent, relations, constraints) for
  all of these PDFs."* This REVERSES the prior "defer ISF lowering" steer — ISF-lowering fidelity of the
  KG/IntentIR is now the near-term north star; digestion breadth + `EXTRACTION-QUALITY-GAUGE` /
  `WIRE-BASED-100` are the means to a complete-KG-for-ISF end. Memory: `project_kg_isf_completeness`.

## The point (why this tree exists)

"ISF ready" does NOT mean "the adapter renders without blocking" — **all 36 built docs already render
`.isf` with 0 `blocking_reasons`** (measured `2026-06-16`, `adapt --target isf --dry-run` across the
corpus). The adapter only *renders*; the fidelity of the `.isf` is the fidelity of the **IntentIR**
feeding it. The deliverable is a KG/IntentIR that **precisely captures all real agents and their
intents** (producer/initiator/consumer/receiver/manager/subordinate/interconnect/…), all relations, all
constraints — so the lowering is faithful and complete, not thin.

## The checkable "ISF-complete IntentIR" bar (per doc)

A document's IntentIR is ISF-complete when:
1. **Agents** — every `actor` is a real protocol agent (zero prose-fragment noise), and every agent the
   document actually defines is present.
2. **Relations** — every real agent is connected to the signals it drives/samples/owns (no disconnected
   real agent); every signal has the producer/consumer the document states.
3. **Signals** — every interface signal carries direction + width (+ polarity/role where the document
   grounds it).
4. **Constraints** — every stated obligation is captured with low NLI-not-entailed rate and zero
   fabrication (the `EXTRACTION-QUALITY-GAUGE` surface).
5. **Behaviors/temporal** — every behavior/temporal rule is carried or recorded as an explicit residual
   (no silent drop; `temporal_residuals`).
6. **ISF round-trip** — the adapter renders 0-blockers AND the emitted `.isf` passes FSMGen
   `--strict --check --json` AND every IntentIR surface element appears in the `.isf` or an explicit
   residual.

**Hard gate (non-negotiable):** the wire docs (APB/AHB/AXI/SWD) stay at WIRE-BASED-100 (100% per-fact on
constraints/relations/temporal) through every change. Universal grammar only, no name lists (ADR 0006).
Scope: the deeply-extracted protocol docs first (the 4 wire docs), then the broader protocol corpus;
guide/register-only docs are gauged honestly (they legitimately carry no behavioral agent surface).

## Measured baseline (`2026-06-16`, read-only over wire-doc IntentIR)

The agent surface has TWO coexisting defects (the naive single fix fails — proven):
- **PRECISION** — prose-fragment NON-agents are minted as actors: AXI `for`/`note`/`then_it`/
  `with_write`/`instruction`; APB `ensures`/`exit_from`/`three_levels`/`for`/`state_machine`; SWD
  `class_x`/`number_of`/`system_has`/`details_about`/`read_only_field`/`dbgswenable_flag`/`ir_register`/
  `watcher_circuit`/`then`. These would emit junk agents into the `.isf`. The naive "drop orphans" rule
  FAILS — they carry ports (3–4 each), so they are not orphans.
- **COMPLETENESS** — real agents carry ZERO ports/relations: AXI `producer`/`consumer`/`receiver`/
  `transmitter`/`requester`/`target` (all 0 ports) while `manager`/`subordinate` hold 169/168. The KG
  names these agents but never connects them to the signals they act on — OR they are unconsolidated
  aliases of manager/subordinate. Either way the KG is relation-incomplete for them.

## Task Tree

- ID: `KG-ISF-COMPLETENESS` · Status: `active` · Children: `.0` (this scope/ownership slice), `.1`+ TBD
- ID: `KG-ISF-COMPLETENESS.0` · Status: `done` (`2026-06-16`, docs-only ownership/scoping slice) · Goal:
  own the north star, define the checkable bar, record the measured baseline, reverse the "defer ISF"
  steer in the live docs. No code (doctrine: own before touching). Memory `project_kg_isf_completeness`.
- ID: `KG-ISF-COMPLETENESS.1` · Status: `active` (measurement phase DONE `2026-06-16`; code → `.1a`/`.1b`)
  · Goal: **agent-surface fidelity** — (a) a STRUCTURAL agent-identity gate rejecting prose-fragment
  non-agents; (b) connect/consolidate the disconnected real agents. **Measurement DONE** (read-only over
  the wire-doc IR; report `docs/research/agent-surface-fidelity-measurement.md`, KM card
  `[[agent-surface-defect-taxonomy]]`): the noise splits into THREE structural classes — **A Junk** (the
  captured "subject" is a clause/function-word/descriptor: `For components`, `HPROT bit`, `is recommended`
  with the real subject `Manager` in the that-clause, `section`, `two-cycle response`, `TREADY input`,
  `is permitted`); **B Fragment of a real agent** (`Subordinate extends`→`Subordinate`, `address decoder`/
  `decoder also`→`decoder` — "*An address decoder provides HSELx*" is a genuine AHB fact —
  `Transmitter interface`→`Transmitter`, `Subordinate and decoder`→split); **C Zero-evidence role-term**
  (0 ports AND 0 rels, minted by the SemanticIR Phase-2 role-term scan). **`.0` hypothesis CORRECTED by
  evidence:** the "disconnected real agents (`producer`/`consumer`/`receiver`/`transmitter`) are a
  recoverable relation gap" claim is DISPROVEN for the wire docs — a Class-C term is a drive/read subject
  next to a known signal ≈0 times, so synthesizing relations for them would be FABRICATION. The genuine
  completeness win is Class-B consolidation; Class-C is dropped **per this doc's evidence**.
  **Genericity guardrail proven:** `transmitter` is 0/0 in AXI but `Transmitter` is 22/23 in AXI-Stream —
  same token, opposite status → the drop/keep rule MUST key off "0/0 in *this* doc", never a name list
  (ADR 0006). Frontier → `.1a` then `.1b`.
- ID: `KG-ISF-COMPLETENESS.1a` · Status: `pending` (measurement-first; gate DESIGNED from code-seam study
  `2026-06-16`) · Goal: **precision — structural agent-identity gate.** **Seam (verified):** both prose
  paths (`extract_subject_phrase`:3302 / `extract_actor_phrase`:3144) AND the table path funnel through
  the ONE DRY seam `normalize_relation_actor_name`:2082 → `is_meaningful_actor_term`
  (prior_memory.rs:607). Add the structural reject there (one edit, all paths). **Scoped gate (clean,
  bounded, lowest-risk):** reject a candidate whose FIRST content token is a closed-class **function
  word** (for/then/with/next/of/by/from/as/also/is/are/has/… — universal English, NOT a chip-name list,
  ADR-0006-safe; the codebase already uses such lists — SKIP_WORDS/STOP_WORDS/SUBJECT_FOLLOWER_VERBS) OR
  a **leading content verb** (ensures/exit/extends/…). Catches `For`/`Then it`/`with write`/`For
  components`/`is recommended`/`is permitted`/`ensures`/`Exit from`/`next`. **Deliberately verb-LED only,
  NOT contains-a-verb** — "contains a verb" would kill the Class-B fragment `Subordinate extends` (a real
  Subordinate fact `.1b` should consolidate). The descriptor-noun class (`HPROT bit`/`TREADY input`/
  `section`/`number of`/`Note`) is harder (needs a contains-declared-signal sub-gate + furniture-noun
  handling) → a LATER leaf, not forced into `.1a`. **Measure first:** apply the predicate across all 36
  persisted IntentIR docs and confirm it rejects ZERO high-port (≥8) actors (real-agent proxy) anywhere.
  WIRE-BASED-100 (APB/AHB/AXI/SWD per-fact 1.000) a hard gate, verified via the `.3f`/`.3g`
  fresh-Pattern temp-evidence-root eval; `cargo build --release` before live measurement; `run_ci.sh`
  before declaring green.
- ID: `KG-ISF-COMPLETENESS.1b` · Status: `pending` (measurement-first) · Goal: **completeness —
  consolidation + zero-evidence honesty.** (i) Normalize Class-B fragments to the canonical agent token
  (strip trailing verb/adverb; "X interface"→"X") — this transform must run BEFORE the `.1a` reject so
  `Subordinate extends`→`Subordinate` is kept, not dropped (the ordering interaction found in `.1a`'s
  code-seam study); (ii) split a coordinated "X and Y" subject; (iii) drop Class-C zero-evidence actors
  per-doc (0 ports AND 0 rels), first re-checking provenance so a genuinely declared-but-unwired agent is
  preserved. Explicitly NO relation synthesis for Class-C (measured fabrication risk). WIRE-BASED-100 a
  hard gate.
- ID: `KG-ISF-COMPLETENESS.2+` · Status: `pending` · Goal: the remaining bar dimensions per doc
  (relation completeness, signal direction/width coverage, constraint completeness via the gauge,
  behavior/temporal carry, and the ISF round-trip fidelity check), each measurement-first + gold-gated.

## Changelog

- `2026-06-16`: **`.0` DONE** — tree created; owns the owner's `2026-06-16` north star (complete KG/
  IntentIR → faithful ISF; extraction serves ISF-fidelity; reverses "defer ISF"). Defined the checkable
  6-point ISF-complete-IntentIR bar; recorded the measured 2-defect agent baseline (precision noise +
  real-agent relation-incompleteness; the naive orphan filter disproven — junk agents carry ports).
  Frontier → `.1` (agent-surface fidelity, measurement-first). Memory `project_kg_isf_completeness`;
  reverses the `project_pdf_variant_digestion` "defer ISF" note.
- `2026-06-16`: **`.1` MEASUREMENT DONE** (read-only, docs-only — measurement-first before any code).
  Full per-actor port/relation census over the 4 wire docs + AXI-Stream, each noise actor's relations
  resolved to source text. Result: the agent surface has THREE structural defect classes (A Junk /
  B Fragment-of-real-agent / C Zero-evidence role-term). **Corrected the `.0` hypothesis with evidence:**
  the "disconnected real agents are a recoverable relation gap" claim is DISPROVEN — Class-C terms are a
  drive/read subject next to a known signal ≈0 times, so connecting them would be fabrication; the real
  completeness win is Class-B consolidation, and Class-C is dropped per-doc. **Genericity guardrail
  proven** (AXI `transmitter` 0/0 vs AXI-Stream `Transmitter` 22/23 → per-doc evidence-keyed, no name
  list). Spun the code into `.1a` (precision structural gate) + `.1b` (consolidation + zero-evidence
  honesty), each measurement-first + WIRE-BASED-100-gated. Report `docs/research/agent-surface-fidelity-measurement.md`;
  KM card `agent-surface-defect-taxonomy`. No code.
