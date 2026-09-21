# KG-ISF-COMPLETENESS — program and bar

- Part ID: `program-and-bar`
- State: `legacy`

<!-- kg-isf-completeness-task-source-region:program-and-bar:start -->
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
- Current sequencing update (`2026-08-11`, ADR 0033): the endpoint is unchanged, but ISF is not the present
  blocker. Populate and measure source-grounded PDF content through canonical IntentIR first; take ISF/FSMGen
  work only when that content demonstrates a concrete expressiveness gap. `SPEC-TO-INTENT-ALIGNMENT` owns the
  cross-program measurement and integration work.

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

- ID: `KG-ISF-COMPLETENESS` · Status: `active` · Children: `.0` (scope/ownership), `.1` (agent-surface on the AMBA/structured class, done; `.1c` reopens it for the dense-prose class), `.2` (ISF lowering-fidelity; `.2a.i` width done, `.2a.ii` direction done — initiator-perspective, owner-authorized, `.2a.iii` module-name HDL-sanitization done — owner-chosen, `.2a.iv` enum value-literal emit-gate done — Lever F, HBM2 strict-clean, `.2a.v` unconditional-rule-overlap conflict residual — Lever C, 6 docs FAIL→PASS incl. all 3 wire golds + LPI/LTI/NVMe, `.2a.vi` rule-drive-value validity gate — closes the last AXI+ACE `(port expr)` FAIL → **70/70 renderable strict-clean**), `.3` (relation-completeness — bar #2; DONE `2026-06-24`: 0 stale docs corpus-wide + stage-staleness detector shipped as CORPUS-COVERAGE.1), `.4` (behavior/temporal lowering-completeness — bar #5/#6, broader corpus), `.5` (enum-surface fidelity — the generic-`TABLE` mega-enum conflation; measurement DONE `2026-06-24`, `.5.i` name-gate + emitter orphan-type fix LANDED `2026-06-24` — corpus generic enums 82→8 / total enum records 422→105, WIRE-BASED-100 held 1.000; `.5.ii` sentence-spine member-gate + `.5.iii` `_WIDTH` parameter-leak member-gate LANDED `2026-06-24` — AXI gold `.isf` enum surface now faithful)
- ID: `KG-ISF-COMPLETENESS.1c` · Status: `active` (umbrella; PROBE DONE `2026-06-23`; `.1c.i` LANDED,
  `.1c.ii` deferred-as-bounded-residual — the clean structural win is shipped, the remainder is
  upstream-NLP-gated) · Goal: **agent-identity precision for the DENSE-PROSE doc
  class** (Lever E, spun out of `CORPUS-COVERAGE.2` re-ingest #27 JEDEC eMMC: actors exploded to 153/349 vs
  the structured DRAM #28 HBM2 which CONSOLIDATED 52→38). The `.1a`/`.1b.*` gates are measured clean on the
  AMBA/structured class but were never measured on dense descriptive prose. **PROBE DONE `2026-06-23`**
  (read-only over the 78 persisted IntentIR docs; report `docs/research/agent-identity-prose-class-measurement.md`;
  KM `[[agent-identity-prose-class-measurement]]`):
  - **It is a relation-subject extraction-precision problem, NOT an actors[] prose-mint problem** — of
    eMMC's 153 actors, **148 are CONNECTED** (minted from a single `REL-INFERRED` relation subject at the
    `.1a`/`.1b` seam; the leading token is a noun so `.1a` passes it, and it carries a relation so `.1b.iv`
    can't touch it); only **5 are pure-unconnected**, and those are the grounded-keep generic role terms
    `.1b.iv` correctly preserves (0 droppable — `.1b.iv` already does the right thing here).
  - **A name-SHAPE-only drop is DISPROVEN unsafe** — AMBA's REAL agents (`agent`/`controller`/`decoder`/
    `device`, `address decoder`, `Exclusive Access Monitor`) occupy the SAME shape classes as eMMC's
    phantoms (`adapter`, `basic bus`, `actual sector`), so dropping on shape would destroy real AMBA agents
    and fail WIRE-BASED-100. The fix must be grammatical NORMALIZATION (rewrite, not drop) or
    participation/grounding, never shape alone ([[feedback_avoid_denylists_prefer_structural]]).
  - **The doc class is DENSE-PROSE specs, not "non-AMBA"** — the dense AXI+ACE `ihi0022_h_c` (189 actors)
    and CHI `ihi0050_g` (87) explode too; the terse WIRE-BASED-100 AXI gold `ihi0022_l` (21) is clean.
  - **Scoped sub-leaves:** **`.1c.i`** — extend the proven `.1b.i` trailing-strip to a closed class of
    trailing **prepositions + auxiliaries** (`advantage of`→`advantage`, `host has`→`host`, `cache in`→
    `cache`), the clean landable first gate; **corpus-wide safety already measured CLEAN** — across all 78
    docs ZERO ≥8-port actors are `X <aux/prep>` shaped, so the strip never renames a real high-participation
    agent (the `.1a`/`.1b.i` safety bar) and the 4 wire golds carry no such actor (structurally untouched);
    reach 138 names / ~17 docs. **`.1c.ii`** (deferred-with-trigger) — single-relation noun-phrase phantom
    precision (the bulk ≈120 of eMMC's 153), needs a participation+grounding discriminator and its OWN
    measurement before any gate (a rare-but-real agent can also appear in one relation; completeness north
    star forbids dropping it).
  - **Gates (probe = no code change):** WIRE-BASED-100 + register/wire golds + `kg-bench` orthogonal by
    construction; `scripts/check_doctrines.sh` GREEN; book honesty caveat added to
    `pipeline/evidenceir.md` (the "widen without minting noise" overclaim, corrected for the dense-prose
    class). **Frontier → `.1c.i` DONE `2026-06-23`** (see the node below), then `.1c.ii` measurement.
- ID: `KG-ISF-COMPLETENESS.1c.i` · Status: `done` (`2026-06-23`, measurement-first; gate LANDED +
  WIRE-BASED-100-verified) · Goal: **dense-prose trailing preposition/auxiliary strip** — extend the
  proven `.1b.i` trailing-fragment consolidation (which strips a trailing universal verb/discourse-adverb)
  to a closed class of trailing **prepositions + auxiliaries/modals** (`host has`/`host is`/`host to`/
  `host with` → `host`, `cache in`/`cache is` → `cache`, `device to` → `device`), so the relation
  re-attributes onto the leading agent and its stranded edges merge by dedup instead of surviving as
  separate phantoms. The `.1b` measurement and `.1b.i` deliberately excluded prepositions, but the stated
  reason was the conjunction/coordination case (`.1b.iii`); the `.1c` probe re-measured the
  preposition/auxiliary class on the dense-prose corpus and found it safe. **LANDED** — new const
  `NON_ACTOR_TRAILING_FUNCTION_WORDS` (prepositions + auxiliaries/modals, a deliberate SUBSET of
  `NON_ACTOR_LEADING_FUNCTION_WORDS`, EXCLUDING conjunctions — drift-guarded by
  `trailing_function_words_are_known_leading_non_conjunctions`) added to the `consolidate_trailing_fragment`
  strip condition in `ir/evidence.rs` (same seam/ordering — BEFORE the `.1a` reject; returns byte-identical
  when nothing strips). `before`/`after`/`until` are already discourse markers, not duplicated. Universal
  grammar, no name list (ADR 0006). KM `[[agent-trailing-function-word-consolidation]]`. **Frontier →
  `.1c.ii`** (single-relation noun-phrase phantom precision — the bulk; deferred-with-trigger pending its
  own measurement).

<!-- kg-isf-completeness-task-source-region:program-and-bar:end -->
