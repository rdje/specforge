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

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.1c.i`
- [x] **REPRODUCE / MEASURE** — `.1c` probe (read-only over the 78 persisted IntentIR docs): eMMC `intent_ir.json` carries **153 actors / 349 relations**, of which 29 actor names end in a closed-class preposition/auxiliary (`host has`/`host to`/`host is`/`host with`, `cache in`/`cache is`, `device to`, `CMD to`, `advantage of`, …) — fragments the `.1a`/`.1b.i`/`.1b.iv` gates do not consolidate. Report `docs/research/agent-identity-prose-class-measurement.md`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `consolidate_trailing_fragment` (`crates/specforge/src/ir/evidence.rs`) strips a trailing token only when it is in `NON_ACTOR_LEADING_VERBS` ∪ `NON_ACTOR_TRAILING_DISCOURSE_MARKERS`; trailing prepositions/auxiliaries were deliberately excluded by `.1b.i` (for the conjunction/`.1b.iii` case), so on dense prose `host has`/`host to` survive as separate relation-subject actors at the `normalize_relation_actor_name` seam (leading token is a noun → `.1a` passes; carries a relation → `.1b.iv` can't touch). Confirmed live: `evidence --dry-run` chain on eMMC yields the 11 `host *` variants.
- [x] **ADDRESSED (verified)** — added `NON_ACTOR_TRAILING_FUNCTION_WORDS` to the strip condition. Live new-binary `evidence→semantic→intent` cascade on eMMC: **actors 153 → 138** (−15), relations 349 → 341; the 4 aux/prep `host` variants (`host has`/`host is`/`host to`/`host with`) **merge onto `host`** (host variants 11 → 7 — the remaining 4 are trailing-*verb* `host selects`/`host tries`/etc., honestly outside this closed-class strip → `.1c.ii`); 29 phantom names removed. eMMC `host.isf` still renders (62 signals) and passes FSMGen `--strict --check --json` **success / 0 diagnostics** (emitter-safe).
- [x] **NO REGRESSION** — **WIRE-BASED-100 = 1.000** on fresh-Pattern new-binary evidence (rebuilt all 4 gold docs into a temp evidence-root; `eval-extraction --provider skip --evidence-root <temp>`): constraints APB 6/6 · AHB 6/6 · AXI 3/3; actor-relations APB 5/5 · AHB 6/6 · AXI 6/6 · SWD 1/1; temporal APB 3/3 · AHB 4/4 · AXI 3/3 (SWD lone constraint 0/1 = the documented promotion-only, unchanged). **`kg-bench` 156/156.** **`scripts/run_ci.sh` GREEN** (lib **1704** passed, +2 new tests; clippy/fmt/rustdoc warning-deny + mdBook). Wire golds unaffected — their only trailing strips are the pre-existing `.1b.i` verb/adverb cases; the new aux/prep vocabulary touches 0 wire-gold cases.
- [x] **GENERICITY (ADR 0006)** — universal English grammar (a closed class of prepositions + auxiliaries/modals), a SUBSET of the leading function-word lexicon, NOT a chip/vendor/protocol name list; conjunctions deliberately excluded (still `.1b.iii`). Corpus-wide safety MEASURED: across all 78 persisted IntentIR docs ZERO actors with ≥8 ports are `X <aux/prep>` shaped → the strip never renames a real high-participation agent.
- [x] **LOCKSTEP** — README current-state bullet (`.1c.i`); book `pipeline/evidenceir.md` caveat updated (the strip has LANDED); KM card `agent-trailing-function-word-consolidation`; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md.

- ID: `KG-ISF-COMPLETENESS.1c.ii` · Status: `deferred` (measured `2026-06-23`, read-only; **bounded
  residual — no clean within-document structural gate; the real fix is upstream**) · Goal: precision for the
  bulk dense-prose phantom class — multi-word, single-`REL-INFERRED`, leading-noun relation subjects
  (`basic bus`, `actual sector`, `B write`; eMMC carries 109 multi-word actors). **MEASURED `2026-06-23`**
  (read-only; report `docs/research/agent-identity-prose-class-measurement.md` §8): a within-document
  structural gate is **disproven unsafe**. The most promising candidate — a `.1b.ii`-style **connectivity
  fold** (rewrite a multi-word subject onto its last content token when that token is an independently
  connected single-word agent) — mishandles the real cases on the dense AXI+ACE `ihi0022_h_c`: it correctly
  folds `caching Manager`/`initiating Manager`→`Manager` (descriptive references) but WRONGLY folds
  `Manager component`→`component` (the agent is the modifier `Manager`, not the noun-phrase head), because the
  agent token's POSITION varies (modifier vs. head) so no fixed structural position is safe; a fold-on-first
  rule would instead break `caching Manager`→`caching`. The deeper reason: within one document a real
  descriptive reference (`caching Manager`) and a phantom fragment (`basic bus`) are STRUCTURALLY
  INDISTINGUISHABLE — same single-relation participation, same `SECTION-PHASE`/`PROSE-PARA` provenance markers
  (a relation's statement always sits in some section), same noun-phrase shape; and eMMC carries no
  first-class `ProtocolActorRecord` agent-definition surface (its EvidenceIR actor surface is *only*
  `actor_signal_relations`), so there is no grounding signal to separate them. A name-shape/connectivity
  DROP is forbidden by the genericity guardrail (it loses real agents) and a participation threshold is
  forbidden by the completeness north star (a rare-but-real agent can appear in exactly one relation). **The
  genuine fix is upstream relation-subject extraction precision on descriptive prose** — reading the actual
  grammatical subject of a who-acts-on-what sentence rather than a noun phrase near the signal — i.e. the
  owner-directed in-Rust shallow-parse direction (`[[project_nlp_shallow_parse_direction]]`,
  `docs/tasks/NLP-SHALLOW-PARSE.md`), NOT a downstream actor-surface rule. Until that lands the multi-word
  prose phantoms stay an honest residual; they never reach the emitted `.isf` (the adapter lowers the
  renderable initiator's signals/behaviours, never the raw `actors[]`), so the bounded cost is IntentIR
  `actors[]` precision (bar #1) on dense-prose specs. **`.1c` umbrella outcome:** the clean structural win
  (`.1c.i`) is landed; the remainder is upstream-NLP-gated, recorded as a residual. **Re-open trigger:** a
  later NLP-shallow-parse slice that improves relation-subject extraction on prose, OR a measured
  agent-definition surface that grounds dense-prose actors.
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
- ID: `KG-ISF-COMPLETENESS.1a` · Status: `done` (`2026-06-16`, measurement-first; gate DESIGNED from
  code-seam study + LANDED + WIRE-BASED-100-verified) · Goal: **precision — structural agent-identity
  gate.** **Seam (verified):** both prose
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
  **LANDED `2026-06-16`** — `is_non_actor_phrase_fragment` (evidence.rs) wired into the DRY seam
  `normalize_relation_actor_name` (one edit, all three relation paths), keyed off two universal-grammar
  consts: `NON_ACTOR_LEADING_FUNCTION_WORDS` + `NON_ACTOR_LEADING_VERBS` (ADR-0006-safe — no chip names).
  **Measurement (read-only, all 36 IntentIR docs) drove two corrections:** (a) DROP the colliding
  pronouns `i`/`its` — they wrongly rejected the 16-port `'I'` and the 8-port GIC `'ITS →Distributor…'`
  (the `ITS` agent; case is soft); (b) EXCLUDE all articles/determiners/demonstratives — a determiner can
  precede a REAL agent (`All Managers`), which is a `.1b` strip, not a `.1a` drop. Refined gate then
  rejected ZERO ≥8-port actors across the corpus (23 high-port real-agent proxies preserved) and caught
  all 9 designed targets; the 63 rejects are all non-agents. **Live rebuild (AXI fresh Pattern
  evidence→semantic→intent):** actors 24→21 (`For`/`with write`/`Then it`/`is permitted` dropped), ZERO
  function/verb-led actors leak (confirms no leak via the direct `normalize_table_actor_name` callers),
  real agents preserved at IDENTICAL port counts (`Manager` 169, `Subordinate` 168, `interconnect` 5),
  and the descriptor-noun class (`exclusive`/`instruction`/`monitor`/`Note`/`Shareable`) correctly
  REMAINS (deferred, per design). **WIRE-BASED-100 HELD on fresh-Pattern eval:** constraints APB/AHB/AXI
  6/6 · 6/6 · 4/4 = 1.000; actor-relations APB/AHB/AXI/SWD 6/6 · 6/6 · 6/6 · 1/1 = 1.000; temporal
  AXI/APB/AHB 3/3 · 3/3 · 4/4 = 1.000 (SWD's lone Pattern constraint stays a promotion-only 0/1,
  independent of this actor-name gate). `kg-bench` 156/156; `run_ci.sh` GREEN (fmt + warning-deny Clippy +
  tests 1633 pass/2 ignored, +3 new + rustdoc + mdBook). KM card `[[agent-identity-structural-gate]]`.
  Frontier → `.1b`.
- ID: `KG-ISF-COMPLETENESS.1b` · Status: `active` (umbrella; measurement DONE `2026-06-16` split it into
  focused sub-leaves) · Goal: **completeness — consolidation + zero-evidence honesty.** Original plan:
  (i) Normalize Class-B fragments to the canonical agent token (strip trailing verb/adverb; "X interface"→"X")
  — this transform must run BEFORE the `.1a` reject so `Subordinate extends`→`Subordinate` is kept, not
  dropped (the ordering interaction found in `.1a`'s code-seam study); (ii) split a coordinated "X and Y"
  subject; (iii) drop Class-C zero-evidence actors per-doc (0 ports AND 0 rels), first re-checking
  provenance so a genuinely declared-but-unwired agent is preserved. Explicitly NO relation synthesis for
  Class-C (measured fabrication risk). WIRE-BASED-100 a hard gate.
  **`.1b` MEASUREMENT DONE `2026-06-16`** (read-only over the persisted 36-doc IntentIR corpus; report
  `docs/research/agent-surface-fidelity-measurement.md` §7) — the original (i) splits into TWO distinct
  risk profiles, and (iii) is far broader than the wire-doc scope, so each becomes its own
  measurement-first + WIRE-BASED-100-gated sub-leaf:
  - **`.1b.i`** (trailing verb/adverb strip) — grammatically UNAMBIGUOUS, the clean completeness win.
  - **`.1b.ii`** (`"X interface"→"X"` strip) — **DONE `2026-06-17`** (see the dedicated node below): the
    "only when the leading token is already a real connected agent in this doc" sub-gate was supplied by a
    post-pass over the assembled relation list (`consolidate_interface_actor_relations`, the same
    `actor_signal_relation_surface` slot as `.1b.iii`), where the connected-agent context the pure-string
    relation-subject seam lacks IS available. So `Subordinate interface`/`Transmitter interface` fold onto
    the real agent while the GIC `CPU interface` named block is preserved.
  - **`.1b.iii`** (coordinated "X and Y" split) — AHB `Subordinate and decoder` 6/4,
    `Exclusive Access Monitor and Subordinate` 4/2; needs the relations duplicated to BOTH agents.
  - **`.1b.iv`** (Class-C zero-evidence drop) — **DONE `2026-06-17`** (see the dedicated node below): of the
    320 Class-C 0/0 actors corpus-wide only **21 are PURE-INFERRED** (the unambiguous Phase-2 role-term
    phantom); **223 are PROSE-GROUNDED** + **76 SECTION+INFERRED**, and "PROSE-GROUNDED" is NOT a clean
    "genuinely-declared agent" discriminator. The designed-defensible rule drops ONLY the PURE-INFERRED
    phantoms (responsibilities == exactly the term-scan marker), keeping every grounded 0/0 agent the owner's
    completeness north star wants — 21 dropped / 16 docs / zero connected-or-grounded actors touched.
- ID: `KG-ISF-COMPLETENESS.1b.i` · Status: `done` (`2026-06-16`, measurement-first; gate LANDED +
  WIRE-BASED-100-verified) · Goal: **Class-B trailing-fragment consolidation** — strip a TRAILING universal
  verb (`NON_ACTOR_LEADING_VERBS`) or trailing adverb/discourse-marker (`then`/`also`/`next`/… — NOT
  conjunctions, which are `.1b.iii` coordination) from a relation-subject candidate, keeping the leading
  NOUN, inside `normalize_relation_actor_name` BEFORE the `.1a` reject. So `Subordinate extends`/
  `Subordinate then`→`Subordinate`, `decoder also`→`decoder` — the stranded relations re-attribute onto
  the real agent (dedup merges them) and the fragment actor disappears. **Measured net effect over the
  persisted corpus (ordered consolidate→`.1a`):** ZERO real agents (≥8 ports) vanish; the wire-doc wins
  land (AHB `Subordinate` gains `extends`+`then`'s relations, `decoder` recovers `also`'s); residual junk
  (`does not`→`does`, `It also`→`It`, `is used`→`is`) consolidates to a function-word head the subsequent
  `.1a` reject then removes (net-better); descriptor-noun residue (`chapter`/`section`/`channel(s`) stays
  the deferred descriptor-noun class, no worse than before. WIRE-BASED-100 a hard gate (fresh-Pattern
  temp-evidence-root eval); `cargo build --release` before live measure; `run_ci.sh` before green.
  **LANDED `2026-06-16`** — new `consolidate_trailing_fragment` (`ir/evidence.rs`) + a dedicated
  `NON_ACTOR_TRAILING_DISCOURSE_MARKERS` const (the adverb subset, drift-guarded by a test asserting it ⊆
  `NON_ACTOR_LEADING_FUNCTION_WORDS`), wired into `normalize_relation_actor_name` between the meaningful
  check and the `.1a` reject; returns the input byte-identical when nothing strips (preserves byte-stability
  for fragment-free docs); infra-safe by construction (the full string already passed the infra reject, a
  leading-token prefix adds no substring). **Live AHB rebuild (fresh Pattern evidence→semantic→intent):**
  `Subordinate` 25/23 → **27/26** (absorbed `extends`+`then`), `decoder` 0/0 → **4/2** (recovered from
  `also`); the three fragment actors GONE; actors 25→19. **AXI no-regression:** real agents byte-identical
  (`Manager` 169/168, `Subordinate` 168/166, `interconnect` 5/3); actors stay 21; deferred classes remain.
  **WIRE-BASED-100 HELD 1.000** on fresh-Pattern eval (constraints AXI/APB/AHB 4/4·6/6·6/6; actor-relations
  AXI/APB/AHB/SWD 6/6·6/6·6/6·1/1; temporal AXI/APB/AHB 3/3·3/3·4/4; SWD lone constraint the documented
  promotion-only 0/1, independent). `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1635 pass/2 ignored, +2
  tests). KM card `[[agent-trailing-fragment-consolidation]]`. Frontier → `.1b.ii`/`.1b.iii`/`.1b.iv`.
- ID: `KG-ISF-COMPLETENESS.1b.ii` · Status: `done` (`2026-06-17`, measurement-first; LANDED +
  WIRE-BASED-100-verified) · Goal: **named-interface consolidation** — fold an `"X interface"` relation
  subject onto the bare agent `"X"` (`Subordinate interface`→`Subordinate`, `Transmitter interface`→
  `Transmitter`) so the relations stranded under the wordy interface form re-attribute onto the genuine
  agent, WITHOUT conflating a distinct named architectural block (GIC `CPU interface` = the GICC, NOT a
  generic `CPU`). The `.1b` measurement (§7.1) deferred this until the safe gate had the actor-set context
  the pure-string seam `normalize_relation_actor_name` lacks. **Measurement DONE `2026-06-17`** (report §9;
  read-only census over the persisted corpus + the fresh wire IR): only 6 docs carry an `"* interface"`
  actor — 3 SAFE-MERGE (`X` is an independent connected agent in this doc: AXI+ACE `Subordinate interface`,
  AXI-Stream `Transmitter interface`, CoreSight `AXI interface`) and 5 CONFLATION-RISK (`X` not connected:
  GIC `CPU interface`/`Q-Channel interface`/`AXI4-Stream interface`, CoreSight `AXI interface` ×2). The
  same token `"AXI interface"` is SAFE in one CoreSight doc and a RISK in two others → the gate MUST key off
  "X is a connected agent in *this* doc", never a name list (ADR 0006), exactly the genericity guardrail
  proven for Class-C. **Crucially none of the 4 WIRE-BASED-100 gold docs (APB/AHB/AXI/SWD) carry an
  `"* interface"` actor → the gold relation surface is structurally untouched.** **Seam:** a new post-pass
  `consolidate_interface_actor_relations` in `actor_signal_relation_surface` (`ir/evidence.rs`), placed
  AFTER `split_coordinated_actor_relations` (so a split-produced `"X interface"` conjunct is caught) and
  BEFORE `dedup_actor_signal_relations` (so the rewritten relation merges with X's existing ones) — the same
  post-pass slot the `.1b.iii` split uses, where the full relation list (hence the connected-agent context)
  is available, which the relation-subject string seam is not. **LANDED `2026-06-17`** —
  `strip_interface_suffix` + `consolidate_interface_actor_relations` (the connected set = relation subjects
  that are NOT themselves `"* interface"` forms; a subject is rewritten only when its stripped lead is in
  that set); +3 tests (suffix-strip recognition, safe-merge fires, conflation guard keeps `CPU interface`).
  **Live (fresh post-`.1a`/`.1b` rebuild):** AXI-Stream `Transmitter interface` GONE → `Transmitter` 23 rels;
  AXI+ACE `Subordinate interface` GONE → `Subordinate` 49 rels; the 4 wire docs carry no `"* interface"`
  actor (byte-identical relation surface). **WIRE-BASED-100 HELD 1.000** (constraints APB/AHB/AXI 6/6·6/6·4/4,
  relations 5/5·6/6·6/6, temporal 3/3·4/4·3/3 — source-tolerant filtered); `kg-bench` 156/156; `run_ci.sh`
  GREEN (lib **1657**, +3). The reclaimed CoreSight + missing-source GIC RISK docs are not live-rebuildable,
  so the conflation guard is locked by the `CPU interface` unit test rather than a live rebuild. KM card
  `[[agent-interface-block-consolidation]]`. **`.1b` umbrella COMPLETE** (`.1b.i`/`.1b.ii`/`.1b.iii`/`.1b.iv`
  all done); with `.1a`, `KG-ISF-COMPLETENESS.1` agent-surface fidelity is fully built. Frontier →
  `.2a` direction (FSMGen-neutral, deferred) / `KG-ISF-TRANSACTIONS` body-emission (FSMGen-parked).
- ID: `KG-ISF-COMPLETENESS.1b.iii` · Status: `done` (`2026-06-16`, measurement-first; LANDED +
  WIRE-BASED-100-verified) · Goal: **coordinated-subject split** — a relation whose subject is a coordinated "X and Y"
  ("*the Subordinate and decoder read HADDR*") is split into one relation per conjunct, so BOTH genuine
  agents are connected to the signal the document says they both act on. Universal grammar (the coordinating
  conjunction `" and "`), no name list (ADR 0006). **Restricted to `" and "` (conjunction = both);
  deliberately NOT `" or "`** — a disjunction is ambiguous (only one acts), so splitting it would fabricate.
  Each conjunct is re-validated through the full agent gate (`normalize_relation_actor_name`, i.e. the
  `.1a`+`.1b.i` chain); the split fires only when ≥2 conjuncts survive, else the subject is left exactly
  as-is. Placed as a post-pass in `actor_signal_relation_surface` BETWEEN
  `augment_check_signal_relations_from_tables` and `dedup_actor_signal_relations`, so the split relations
  dedup (first-wins by actor/signal/is_drives) against existing ones and the coordinated-fragment actor
  vanishes. **Measured (read-only over the corpus):** exactly 2 coordinated-subject actors, both AHB —
  `Subordinate and decoder` 6/4 (both conjuncts real) and `Exclusive Access Monitor and Subordinate` 4/2;
  net-new completeness after dedup: `decoder` +4 reads (HADDR/HADDRCHK/HCTRLCHK1/HNONSEC), `Subordinate`
  +3, `Exclusive Access Monitor` +1. **WIRE-BASED-100 safe:** the AHB relation gold is 6 `drives` facts on
  unrelated statements (HRESP/HREADYOUT/H*USER), none coordinated, and the scorer does not penalize
  off-gold relations (AXI 343 rels / fp=0 in the `.1b.i` eval). Hard gate via fresh-Pattern eval;
  `run_ci.sh` before green.
  **LANDED `2026-06-16`** — `split_coordinated_actor_subject` + `split_coordinated_actor_relations`
  (+ `relation_actor_id_slug`) in `ir/evidence.rs`, wired as a post-pass in `actor_signal_relation_surface`
  between `augment_check_signal_relations_from_tables` and `dedup_actor_signal_relations`. Splits on
  word-bounded `and` only (re-validates each conjunct through `normalize_relation_actor_name`; fires only
  when ≥2 survive, else leaves the subject as-is); the coordinated relation is REPLACED by one per conjunct
  with a deterministic per-conjunct id (`<id>__<slug>`). **Live AHB rebuild:** the 2 coordinated-fragment
  actors GONE, `decoder` 4/2 → **8/6** (gained the 4 coordinated reads), `Subordinate` 27/26 → **32/31**,
  `Exclusive Access Monitor` 4/2 → **5/3**; actors 19→17. **WIRE-BASED-100 HELD 1.000** (constraints
  AXI/APB/AHB 4/4·6/6·6/6; actor-relations AXI/APB/AHB/SWD 6/6·6/6·6/6·1/1; temporal 3/3·3/3·4/4).
  `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1637 pass/2 ignored, +2 tests). KM card
  `[[agent-coordinated-subject-split]]`. Frontier → `.1b.ii` / `.1b.iv` (both deferred-with-trigger) / `.2+`.
- ID: `KG-ISF-COMPLETENESS.1b.iv` · Status: `done` (`2026-06-17`, measurement-first; LANDED +
  WIRE-BASED-100-verified) · Goal: **Class-C PURE-INFERRED phantom drop** — remove the unambiguous
  zero-evidence role-term phantoms the SemanticIR Phase-2 scan (`build_actors`, semantic.rs:3177) mints
  whenever a statement merely MENTIONS a generic role word, WITHOUT deleting an agent the document
  genuinely discusses (the owner's completeness north star). **Measurement DONE `2026-06-17`** (report
  `docs/research/agent-surface-fidelity-measurement.md` §8; KM `[[agent-pure-inferred-phantom-drop]]`):
  rebuilt the 4 wire docs evidence→semantic→intent into a TEMP evidence-root with the CURRENT
  post-`.1a`/`.1b` binary (WRITE-PATH GOTCHA — the canonical corpus is still the pre-`.1a` baseline) and
  categorised every 0/0 actor by IntentIR responsibility provenance. **The defensible discriminator**
  (research-recommended "drop only the PURE-INFERRED phantoms"): an actor is a phantom iff its
  `responsibilities` is EXACTLY the single term-scan marker ``"semantic role inferred around `X` evidence"``
  — distinct from the relation-evidence summary a CONNECTED actor carries
  (``"semantic role inferred from actor-signal relation evidence around `X`"``, which fails the marker) and
  from a grounded 0/0 actor that also carries a phase (`participate in …`) or contract responsibility (set
  length > 1). So a SECTION+INFERRED / PROSE-GROUNDED actor (AXI `transmitter`, SWD `host`, GIC `arbiter`)
  is KEPT. **Seam (DRY, single place):** `build_intent_actors` (`ir/intent.rs`) — where responsibilities are
  assembled — `continue`-skips a pure-inferred phantom; everything downstream (`actor_ids` → behaviors,
  assumptions) derives from the returned `actors`, so the drop propagates with no dangling reference (a
  phantom is 0/0 by construction so nothing else references it). **LANDED `2026-06-17`** —
  `is_pure_inferred_phantom_role` (marker-SHAPE match, ADR-0006, no name list) + the guard in
  `build_intent_actors`; +2 tests (a producer-template drift guard + a drop-phantom-keep-grounded behavior
  test). **Measured:** drops exactly the **21 corpus-wide phantoms / 16 docs** (wire docs: APB `controller`,
  AHB `agent`; AXI/SWD 0), **ZERO connected or grounded actors touched** (invariant proven over the fresh
  wire IR AND the persisted 36-doc corpus). **Stage-diff proof (old vs new binary, fresh temp roots):**
  evidence + semantic byte-identical (modulo the embedded input-path field), intent differs ONLY by the
  removed phantom actors + the phantom id leaving the global behaviors' `actor_ids` list; the gold-bearing
  `actor_signal_relations` / `signal_constraints` / `temporal_rules` are BYTE-IDENTICAL on all 4 wire docs.
  **`.isf` byte-identical** (the emitter lowers signals/behaviors, never the raw `actors[]`). **WIRE-BASED-100
  HELD 1.000** on the fresh post-`.1b.iv` evidence (eval-extraction `--provider skip`: constraints
  APB/AHB/AXI 6/6·6/6·4/4, relations APB/AHB/AXI 6/6·6/6·6/6, temporal APB/AHB/AXI 3/3·4/4·3/3 — all
  source-tolerant filtered F1 = 1.000); `kg-bench` 156/156 (no fixture asserts a phantom actor);
  `run_ci.sh` GREEN (lib **1654**, +2). Book `pipeline/intentir.md` "How the actor surface stays faithful"
  (consolidates the `.1a`/`.1b.i`/`.1b.iii`/`.1b.iv` agent-surface story, previously book-undocumented).
  Class-C taxonomy now CLOSED for the unambiguous phantom subset; the broader PROSE-GROUNDED 0/0 set stays
  honestly KEPT (no clean genuinely-declared discriminator — research §7.3). Frontier → `.1b.ii` (deferred,
  named-block conflation) / `.2a` direction (deferred, FSMGen-neutral) / `KG-ISF-TRANSACTIONS` body-emission
  (FSMGen-parked).
- ID: `KG-ISF-COMPLETENESS.2` · Status: `active` (measurement DONE `2026-06-17`, read-only, docs-only;
  code → `.2a`/`.2b`) · Goal: **gauge which remaining bar dimension carries the largest faithful-lowering
  gap** (relation completeness / signal direction-width / constraint gauge / behavior-temporal carry / ISF
  round-trip), measurement-first before any code. **Measurement DONE** (report
  `docs/research/isf-lowering-fidelity-measurement.md`; KM card `[[isf-lowering-fidelity-gauge]]`): read the
  full ISF lowering (`from_intent_ir`/`render`) + replicated its per-element filters over all 36 IntentIR
  docs. **Findings:** (1) the 22k `behaviors` + 22k `constraints` are FREE-TEXT legacy surfaces whose
  content is already lowered via the typed twins (`signal_constraints`/`temporal_rules`/`conditional_rules`)
  — re-lowering them is the redundant rendering `KG-ISF-TRANSACTIONS.2b` removed, NOT a gap; (2) temporal
  rules + register resets already carry honest residuals; (3) the typed-rule lowering `continue`-skips
  17 425 elements with no residual, BUT they are NOT lost grounded intent — 16 179 are un-grounded
  free-text temporal_invariants (a ToC heading classified as an invariant), 1 095 are no-consequent prose
  (legal boilerplate, vague "must"/"shall"), and the 151 "undeclared-signal" cases (0 on all 4 wire docs)
  are dominated by register/struct-field paths (`process_id[19:17]`/`DC.tc.SXL`/`DID`/`Reserved`) that
  belong to the register/message-field surface, not wire rules → mass-residualizing would be dishonest
  noise; **bar #6 is already honest for grounded wire intent**; (4) **the largest TRUE infidelity is signal
  direction/width**: the emitter reads only the legacy flat `direction_hint`/`width_hint` (None for
  85–98%) and defaults to `output`/`width-1` (AXI `.isf` = 283 `(output)` vs 4 `(input)`, all width 1),
  ignoring the canonical actor-relative graph (`actor_ports`) — but direction is relationship-relative, the
  `.isf` is a single flat module, and the default is the DELIBERATE, documented `R6-ISF-ADAPTER.4` policy
  (FSMGen schedules) in tension with the north star (owner decision). Spun → `.2a` (direction/width
  fidelity, deferred-with-trigger + owner steer) + `.2b` (lowering-coverage visibility gauge, buildable).
  WIRE-BASED-100 a hard gate on anything built. No code.
- ID: `KG-ISF-COMPLETENESS.2a` · Status: `deferred` (with-trigger; needs an FSMGen-contract check +
  reference-boundary design + an OWNER decision) · Goal: **signal direction/width fidelity in the emitted
  `.isf`** — carry the document-grounded direction/width instead of defaulting to `output`/`width-1`. The
  largest measured true infidelity (`.2`), but design-gated: (i) direction is relationship-relative (every
  signal is both an input and output across actors) and the `.isf` is a SINGLE flat module
  (`derive_isf_actor_name`) → needs a chosen reference boundary; (ii) `R6-ISF-ADAPTER.4` deliberately
  defaults direction/width because FSMGen owns scheduling — overriding it is an OWNER decision in tension
  with the `2026-06-16` north star; (iii) width is mostly symbolic (`*_WIDTH`) → needs an FSMGen-contract
  check on `(width PARAM)`; (iv) actor-port direction must be recovered on FRESH post-`.1a` canonical
  artifacts (the persisted corpus still carries phantom actors like `For`). **Re-open trigger:** an FSMGen-
  contract answer (does `--strict --check` use signal direction? does it accept symbolic widths?) + owner
  steer on the north-star-vs-policy tension. Measurement-first; WIRE-BASED-100 + FSMGen `--strict` hard
  gates (large wire-doc `.isf` blast radius).
  **FSMGen-contract TRIGGER DONE `2026-06-17`** (empirical binary probe + book contract; KM card
  `[[fsmgen-ignores-signal-direction]]`): **direction is FSMGen-NEUTRAL** — flipping a driven
  `(output SWDIO)`→`(input SWDIO)` still passes `--strict` (`(set port expr)` has no direction constraint),
  so the `output` default is NOT a faithful-lowering gap, only a human-readability/owner-philosophy choice
  (relationship-relative) → **direction stays deferred, low-value** (no technical motivation; revisit only
  on explicit owner steer). **Width** must resolve to a positive integer (concrete passes; undefined
  symbolic fails closed) → the clean win is the concrete grounded width, which landed as `.2a.i` below.
  **DIRECTION RE-OPENED + RESOLVED `2026-06-18` by explicit owner steer (the recorded re-open trigger):** the owner
  chose initiator-perspective direction emission (AskUserQuestion `2026-06-17`), reframing the `.2a` "FSMGen-neutral
  → low-value" finding as a north-star faithfulness gap → built as `.2a.ii` below (the reference-boundary = the
  structurally-identified initiator). `.2a` is now fully resolved (width `.2a.i` + direction `.2a.ii`).
- ID: `KG-ISF-COMPLETENESS.2a.i` · Status: `done` (`2026-06-17`, measurement-first; LANDED +
  verified) · Goal: **emit the grounded concrete signal width from the actor-port graph** where the
  emitter currently defaults to width 1. Measured: of 3 308 width-1 signals, **69 (2.1%) carry a single
  unambiguous concrete width > 1** in `actor_ports[].width_hint`, **0 conflicts** (wire docs: AXI 32 —
  `ARSIZE→3`/`ARBURST→2`/`ARCACHE→4`/…; AHB 2 — `HSIZE→3`/`HTRANS→2`; APB/SWD 0). **LANDED:**
  `actor_port_concrete_widths` (`ir/isf_ir.rs`) maps each signal to its single unambiguous concrete graph
  width; the signal lowering prefers it over the width-1 default (never overrides a real > 1 hint, never
  guesses on conflict → honest width-1 stays). ADR-0006 (universal, no name list); +1 unit test. **Live
  (release):** AXI `.isf` 32 signals gain real widths / AHB 2, **non-signal lines 0**, APB/SWD
  byte-identical; **0 NEW FSMGen `--strict` diagnostics** (AXI/AHB keep their byte-identical PRE-EXISTING
  rule-lowering diagnostics `constraint_33 (port expr)` / `HAUSER` conflict, unrelated to widths).
  WIRE-BASED-100 structurally unaffected (emitter-only change, downstream of extraction); `kg-bench` not
  affected; `run_ci.sh` GREEN (lib 1652, +1). KM cards `[[fsmgen-ignores-signal-direction]]` +
  `[[isf-lowering-fidelity-gauge]]`. Book `pipeline/isf-adapter.md` signal-width note.
- ID: `KG-ISF-COMPLETENESS.2a.ii` · Status: `done` (`2026-06-18`, measurement-first; **OWNER-AUTHORIZED
  `2026-06-17`** — the explicit owner steer the `.2a` deferral required: "Build it, initiator perspective"; LANDED
  + verified) · Goal:
  **emit the grounded actor-relative signal DIRECTION in the `.isf` interface, from the protocol's
  primary/initiator actor's perspective**, instead of defaulting non-`Input` signals to `(output)`
  (`isf_ir.rs:693-694`). The owner reframed the `.2a` "FSMGen-neutral → low-value" finding: it is a north-star
  FAITHFULNESS gap (a signal the perspective actor READS must be `(input)`, not a defaulted `(output)`), even
  though FSMGen ignores direction for strict validity. **Owner choice (AskUserQuestion `2026-06-17`):** initiator
  perspective (Manager/Requester/Host) — a signal the initiator Drives → `(output)`, Reads → `(input)`,
  genuinely-ungrounded → keep `(output)` as an honest residual. **Design constraints:** (1) identify the initiator
  STRUCTURALLY (ADR-0006, no `Manager`/`Requester` name list — e.g. the actor with the dominant Drives footprint
  / driving the request/address signals, to be chosen measurement-first); (2) the perspective must be CONSISTENT
  with whatever the module represents (today `derive_isf_actor_name` = `actors.first()`); (3) **strict-safety blast
  radius** — the emitter emits top-level `(drive (X val))` blocks + transaction drives for outputs, so a signal
  flipped to `(input)` must NOT also be driven (FSMGen rejects driving an input / a `drive` of an undefined
  output), MEASURE the interaction and emit drives only for outputs. **Gates (hard):** WIRE-BASED-100 (orthogonal
  — emitter-only, downstream of extraction), FSMGen `--strict --check` 0 NEW diagnostics on all 4 wire docs,
  `kg-bench` 156/156, `run_ci.sh` GREEN; ADR-0006; honest residual over fabrication.
  **DONE — what landed:** (1) `select_initiator_actor(&actor_ports)` (`ir/isf_ir.rs`) — the initiator is the
  **net-producer** actor (output ports strictly exceed input ports) maximizing `(out, in)` lexicographically;
  structural, no name list (ADR 0006). `out > in` excludes balanced prose-fragment actors (AHB `address decoder`)
  and input-dominant completers (`Subordinate`/`Completer`); the `(out, in)` tiebreak prefers a real initiator
  (reads responses) over an output-only register fragment. No net producer → `None` → prior default-`output`
  behavior (honest residual, byte-identical). (2) `initiator_perspective_directions` — the initiator's per-signal
  direction map (`Drives`→`Output`, `Reads`→`Input`; a both-driven-and-read / `InOut` / `Unknown` signal OMITTED →
  residual). (3) the signal loop prefers that map, else the flat hint, else `(output)`. (4) `derive_isf_actor_name`
  (`ir/adapters.rs`) names the module after the SAME initiator so the label and its `(input)`/`(output)` interface
  are coherent. **Strict-safety:** the per-output named-drive block is already `IsfDirection::Output`-filtered, so a
  signal flipped to `(input)` is auto-suppressed from drives (FSMGen rejects driving an input) — no fabrication.
  **Measured (per-item, `2026-06-18`):** initiators AHB `Manager` (6/2), APB `Requester` (20/12), AXI `Manager`
  (116/52), SWD/debug `debugger` (2/1) — all correct. **Direction flip (baseline-vs-after via `git stash`, emitted
  `.isf`):** APB input **2→12** / output 30→20 (faithful — drives PADDR/PWDATA/PWRITE/PSEL/PENABLE…, reads
  PRDATA/PREADY/PSLVERR/PBUSER…), AXI input **4→52** / output 283→235, SWD input **0→1** / output 13→12, **AHB input
  0→0** (HONEST RESIDUAL — the stale persisted intent grounds Manager only to 6 sideband outputs + HCLK/HRESETN
  inputs, excluded as clock/reset; the rich HADDR/HREADY/HRDATA signals carry no Manager relation in that artifact →
  default `(output)`; a fresh post-`.1a` rebuild flips more). Module names: AHB `address_decoder`→`manager`, APB
  `apb_protocol`→`requester`, AXI `agent`→`manager`, SWD `agent`→`debugger`. **Gates ALL GREEN:** **FSMGen
  `--strict --check` 0 NEW diagnostics on all 4 wire docs** (baseline byte-identical: AHB `HAUSER` + AXI `ASKSTOP`
  pre-existing rule-write conflicts unchanged; APB/SWD PASS); `run_ci.sh` GREEN (lib **1677→1679**, +2 tests:
  `select_initiator_actor_picks_the_net_producer`, `initiator_perspective_directions_are_grounded_and_residual_safe`);
  `kg-bench` 156/156; WIRE-BASED-100 orthogonal (emitter-only, downstream of all extraction); ADR-0006; honest
  residual. KM card `isf-initiator-perspective-direction`. Book `pipeline/isf-adapter.md` "Which way does each signal
  point?". `[[project_kg_isf_completeness]]` /
  `[[feedback_no_hardcoded_chip_spec_names]]` / `[[feedback_isf_no_hacks]]`.
- ID: `KG-ISF-COMPLETENESS.2a.iii` · Status: `done` (`2026-06-21`, owner-chosen via AskUserQuestion — "Fix
  ISF emitter bug B"; LANDED + verified) · Goal: **HDL-sanitize the emitted `.isf` MODULE NAME so a prose-fragment initiator
  actor name no longer breaks the WHOLE `.isf`.** Surfaced by `CORPUS-COVERAGE.2` (re-ingest #14, GIC-600):
  FSMGen `--strict --check` rejects the whole file with `Malformed top-level FSM source
  '?fsm:redistributor→_distributor…'. expects '?fsm:name' with an HDL-identifier-compatible module name
  ([A-Za-z_]\w*)`. Root cause: `derive_isf_actor_name` (`ir/adapters.rs`, the `.2a.ii` initiator-named module)
  does its OWN minimal `.replace([' ', '-', '.'], "_")`, which leaves the arrow `→` (and any other
  non-`[A-Za-z0-9_]` char) in the name, so the `(actor <name>)` header — and FSMGen's derived `?fsm:<name>` —
  is a malformed identifier. **Design (correctness-checked before coding):** (1) `from_intent_ir` re-derives the
  initiator INTERNALLY and raw for direction matching (`isf_ir.rs:684`), so the passed `actor_name` is used ONLY
  for the module LABEL — sanitizing it CANNOT break initiator port-matching (verified). (2) flip the existing
  `sanitize_isf_name` (`ir/isf_ir.rs`) from a char DENYLIST (which enumerates ASCII punctuation but misses `→`
  / unicode — and is itself an anti-pattern per `[[feedback_avoid_denylists_prefer_structural]]`) to a char
  ALLOWLIST: keep `[A-Za-z0-9_]`, map every other char to `_` — exactly FSMGen's `[A-Za-z_]\w*` contract,
  universal, no name list (ADR 0006). The allowlist is byte-identical to the denylist on every ASCII-punctuation
  input already covered (the existing unit cases + the wire-gold signal names are pure alphanumeric), so it ONLY
  changes previously-broken names (`→`/unicode) → a fix, never a regression. (3) route `derive_isf_actor_name`'s
  module-name through the shared `sanitize_isf_name` (make it `pub(crate)`) so the module label and the internal
  signal/rule/register identifiers obey ONE rule (no drift), and clean names (`Manager`→`manager`,
  `Requester`→`requester`, `debugger`) stay byte-identical. **Acceptance / gates (hard):** GIC-600's `.isf` module
  header becomes a valid HDL identifier and its FSMGen module-name error clears; the 4 wire-gold `.isf` +
  register golds are BYTE-IDENTICAL (clean names → no-op); WIRE-BASED-100 orthogonal (emitter-only); `kg-bench`
  156/156; `run_ci.sh` GREEN; ADR-0006; honest residual. Discovery cross-ref: `CORPUS-COVERAGE.2` Lever B.
  **DONE — what landed:** (1) `sanitize_isf_name` (`ir/isf_ir.rs`) flipped from a char DENYLIST to a char
  ALLOWLIST — lowercase, keep `[A-Za-z0-9_]`, map every other char (incl. `→`/unicode the denylist missed) to
  `_`, then the existing collapse-`__`/trim/empty→`unnamed`/leading-digit→`reg_` guards; made `pub(crate)`.
  (2) `derive_isf_actor_name` (`ir/adapters.rs`) routes all three module-name candidates (initiator /
  `actors.first()` / `document_key`) through `sanitize_isf_name` instead of its own minimal replace. (3) the
  existing `sanitize_isf_name` unit test gains the arrow/unicode cases + a validity loop (every output is a valid
  HDL identifier). **Verified (release binary, `2026-06-21`):** GIC-600's re-ingested `.isf` header →
  `(actor redistributor_distributor_distributor_redistributor` (valid id) and FSMGen `--strict --check`
  `success=true`, **0 diagnostics** — the module-name error cleared, the whole `.isf` now lowers strict-valid.
  **Wire golds BYTE-IDENTICAL:** APB `(actor requester` (0 diag), SWD `(actor debugger` (0 diag), AHB/AXI
  `(actor manager` showing ONLY the pre-existing `HAUSER`/`ASKSTOP` rule-write conflicts → **0 NEW diagnostics**
  (allowlist is a no-op on their pure-alphanumeric names). **Gates ALL GREEN:** `run_ci.sh` GREEN (lib **1679**,
  the existing sanitize test extended — no count change); `kg-bench` 156/156; WIRE-BASED-100 orthogonal
  (emitter-only); ADR-0006; allowlist-not-denylist (`[[feedback_avoid_denylists_prefer_structural]]`). KM card
  `isf-module-name-hdl-sanitization`; book `pipeline/isf-adapter.md`. `[[project_kg_isf_completeness]]`.
- ID: `KG-ISF-COMPLETENESS.2a.iv` · Status: `done` (`2026-06-23`, measurement-first; LANDED + verified) ·
  Goal: **gate ISF enum emission on member-value FSMGen-emittability so a mega-conflated binary-as-decimal
  enum no longer breaks the WHOLE `.isf`.** Surfaced by `CORPUS-COVERAGE.2` (re-ingest #28, JEDEC HBM2 DRAM —
  "Lever F"): FSMGen `--strict --check` rejects the whole file with `Package … contains '+enums' entry for
  enum member 'TABLE.REPAIR_LANE_8' with value token '1000', but package symbol values currently must resolve
  to literal scalar values such as '0', '8'3', '8'hA5'`. **MEASUREMENT-FIRST CORRECTION (the discipline
  catching a wrong hypothesis):** the initial root-cause hypothesis was "count-derived enum width overflow"
  (`IsfIr::from_intent_ir` `ir/isf_ir.rs:878` sets the backing `(type … (bits B))` width from member COUNT,
  `ceil(log2(count))`, so a value bigger than `2^B` overflows). A width-fits gate was coded — and a
  **before/after `.isf` diff DISPROVED it**: it wrongly dropped legitimate AXI `AWATOP`(49)/`AWSNOOP`/`ARSNOOP`
  and AHB `TABLE`(64) enums, and a **value sweep against the real FSMGen** showed FSMGen ACCEPTS bare decimals
  of ANY magnitude (`999`/`1020`/`69152` pass — GIC-600's `TABLE` has `69152` and is strict-clean) — so width
  is NOT the rule. The TRUE rule: FSMGen's package-symbol parser rejects a **BARE token of only binary digits
  (`0`/`1`) with length >= 4** (it treats it as an un-qualified binary literal: `1000`/`1010`/`1111`/`10000`
  fail; `0`/`1`/`10`/`111` (<=3 binary digits) and any value with a 2-9 digit pass; `4'b1000`/`16'd1000`
  qualified pass at any magnitude). HBM2's `TABLE.REPAIR_LANE` values (`1000,1001,1110,1111`) are exactly that
  — BINARY codes the extractor mis-read as bare decimals. **Root cause (WHY+WHERE):** `emitted_enums()`
  (`isf_ir.rs:352`) gated ONLY on `is_safe_isf_scalar_value` (non-empty + no whitespace), so a bare `1000`
  reached FSMGen. The `TABLE` enum is itself a mis-extraction — a generic `TABLE` mega-enum conflating ~10
  distinct doc tables (REPAIR_LANE codes + microbump pitches + test-op lists + IDD currents) with
  restarting/duplicate values and sentence-fragment member names — so emitting it is fabrication. **DONE —
  what landed:** (1) new `isf_enum_value_is_emittable_literal(value)` helper (`ir/isf_ir.rs`, beside
  `is_safe_isf_scalar_value`) — FALSE only for a bare `[01]`-only token of length >= 4 (the FSMGen-rejected
  binary-literal shape), TRUE for every legit decimal / qualified literal; verified against the real FSMGen by
  a value sweep, no name list (ADR 0006). (2) new free fn `isf_enum_is_emittable(e)` — non-empty + every
  member a safe scalar AND emittable-literal; `emitted_enums()` filters on it. (3) new `enum_residuals(&self)`
  accessor records an `isf_enum_value_literal_<name>` packet for each enum dropped *specifically* by this gate
  (all members safe-scalar but ≥1 binary-token) — pre-existing operator-expression drops keep their prior
  silent exclusion (no new residual → those docs' adapter surface byte-identical). (4) `adapters.rs` extends
  `residual_decisions` with `isf_model.enum_residuals()`. (5) two unit tests (binary-looking value excluded +
  residual; 999-boundary + `16'd1000` radix token still emit). **Verified (release binary, `2026-06-23`):**
  HBM2's re-emitted `hbm.isf` drops ONLY the malformed `TABLE` (residual `isf_enum_value_literal_table`) and
  FSMGen `--strict --check --json` is now **success / 0 diagnostics**; the doc's other enums incl. `EXTEST_RX`
  (212) and `DWORD_MISR` (19) are correctly KEPT (their values aren't binary-token shaped); HBM2 carries **0**
  `TABLE.<member>` references so dropping it strands nothing. **Corpus-wide byte-identical EXCEPT HBM2:** a
  fresh re-emit + `diff` of all `.isf` (wire golds APB/AHB/AXI/SWD + Avalon + CoreSight SoC-600 `69152`-class +
  GIC-600 `69152` + ARM-Debug `3360`) shows **the only changed file is `jesd235a_2015_11_hbm2_dram/hbm.isf`** —
  the binary-token criterion never flags a legit decimal, so every currently-clean enum is untouched. **Gates
  ALL GREEN:** `run_ci.sh` GREEN (lib **1706**, +2); `kg-bench` 156/156; WIRE-BASED-100 orthogonal (emitter
  only — wire-gold `.isf` byte-identical). Discovery cross-ref: `CORPUS-COVERAGE.2` Lever F. KM card
  `[[isf-enum-value-literal-emit-gate]]`; book `pipeline/isf-adapter.md`. `[[project_kg_isf_completeness]]` /
  `[[feedback_isf_no_hacks]]` / `[[feedback_verify_fsmgen_before_fr]]`. **The upstream mega-enum conflation +
  binary-as-decimal mis-read (generic `TABLE` sweeping many tables) stays an honest residual → a future
  extraction-precision lever.**

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.iv`
- [x] **REPRODUCE / MEASURE** — `adapt --target isf` on the persisted HBM2 `intent_ir.json` emits `hbm.isf`; the real `subs/fsmgen/bin/fsmgen --strict --check --json hbm.isf` returns `success:false` with `enum member 'TABLE.REPAIR_LANE_8' value token '1000'` rejected. A value sweep against the same FSMGen pins the rule: `1000`/`1010`/`1111`/`10000` fail, `999`/`1020`/`69152` and short `0/1/111` and qualified `4'b1000`/`16'd1000` pass.
- [x] **ROOT CAUSE (WHY + WHERE)** — `emitted_enums()` (`crates/specforge/src/ir/isf_ir.rs:352`) gated only on `is_safe_isf_scalar_value` (whitespace-free), so a bare binary-looking token (`1000`) reached FSMGen, which rejects an un-qualified `[01]`-only token of length >= 4. The HBM2 `TABLE` is a mis-extracted mega-enum (REPAIR_LANE binary codes mis-read as decimals + ~10 conflated tables). Initial "count-derived width overflow" hypothesis (`isf_ir.rs:878`) was DISPROVEN by a before/after `.isf` diff (it wrongly dropped AXI `AWATOP`/AHB `TABLE`) and the FSMGen value sweep (`69152` accepted).
- [x] **ADDRESSED (verified)** — new `isf_enum_value_is_emittable_literal` + `isf_enum_is_emittable` + `enum_residuals` (`isf_ir.rs`) + `adapters.rs` residual wiring. HBM2 `hbm.isf` now FSMGen `--strict --check --json` **success / 0 diagnostics**; only `TABLE` dropped → residual `isf_enum_value_literal_table`; `EXTEST_RX`(212)/`DWORD_MISR`(19) KEPT. 2 new unit tests pass.
- [x] **NO REGRESSION** — fresh re-emit + `diff` of all emitted `.isf`: the ONLY changed file is HBM2 `hbm.isf` — wire golds (APB/AHB/AXI/SWD) + Avalon + CoreSight SoC-600 + GIC-600 + ARM-Debug `.isf` BYTE-IDENTICAL. `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1706 passed, +2; clippy/fmt/rustdoc warning-deny + mdBook); WIRE-BASED-100 orthogonal (emitter-only, wire `.isf` byte-identical).
- [x] **GENERICITY (ADR 0006)** — universal token grammar (a bare `[01]`-only token of length >= 4 = an un-qualified binary literal FSMGen rejects), NOT a chip/vendor/protocol name list; verified against the real FSMGen, not guessed (`[[feedback_verify_fsmgen_before_fr]]`); every legitimate decimal / qualified literal passes, so currently-clean enums are byte-identical.
- [x] **LOCKSTEP** — README current-state bullet; book `pipeline/isf-adapter.md`; KM card `isf-enum-value-literal-emit-gate`; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md; `CORPUS-COVERAGE.2` Lever-F + strict-tally corrected (DTI Lever A already resolved; HBM2 Lever F now resolved).

- ID: `KG-ISF-COMPLETENESS.2a.v` · Status: `done` (`2026-06-23`, measurement-first; LANDED + verified
  against the real FSMGen + full corpus sweep) · Goal: **close the open ISF strict-FAIL class — the
  `isf_conflicting_rule_writes` cross-guard conflict ("Lever C"), surfaced on the AMBA Low Power Interface
  (`ihi0068_d`) but MEASURED to also hit the AXI/AHB/AXI-Stream wire golds, LTI, and NVMe** (north-star
  bar #6: every renderable `.isf` is strict-valid; a dropped obligation becomes an honest residual, never a
  silent loss or a fabricated resolution). **MEASUREMENT-FIRST CORRECTION:** the resume-pointer tally
  "27/28 renderable clean, 1 FAIL (LPI)" was STALE — a full fresh current-binary re-emit + FSMGen sweep
  showed the conflict was NOT LPI-only (the cached wire-gold `.isf` were byte-identical to fresh AND already
  FSMGen-FAIL on this conflict). After the fix: **6 docs FAIL→PASS** (AXI `ihi0022_l`, AHB `ihi0033_c`,
  AXI-Stream `ihi0051_b`, LPI `ihi0068_d`, LTI `ihi0089_d`, NVMe), **0 PASS→FAIL regressions**. **TALLY
  CORRECTION (`2026-06-23`, same slice):** the per-`.isf` tally was first reported over a stale-inclusive
  107-file cache; `adapt` emits only ONE primary-actor `.isf` per doc, so 37 of those were STALE cruft from
  older binary versions (cleaned). Over the **70 current-emit `.isf` (one per doc)** the honest post-fix tally
  is **69/70 PASS** — the LONE remaining FAIL is `ihi0022_h_c` (combined AXI+ACE) on the ORTHOGONAL `(port
  expr)` grammar (a spun-out `ISF-VALUE-WIDTH-EMIT` Non-Goal), NOT this conflict class. So after Lever C the
  ISF-emit strict-FAIL frontier is effectively CLOSED (Levers A/B/C/F resolved).
  **REPRODUCE (read-only, real FSMGen):** `subs/fsmgen/bin/fsmgen --strict --check --json` on the persisted
  LPI `controller.isf` returns `success:false` / 1 diagnostic: `ISF conflict 'isf_conflicting_rule_writes' on
  target 'PREQ': … rule 'rule_5' (rule_action, <- 1) conflicts with rule
  'temporal_temporal_signal_constraint_dyn_sigcon_0012' (rule_action, <- 0)`. A second, identical-shape
  conflict on `PACCEPT` (`constraint_3` ←1 vs `temporal_…_dyn_sigcon_0011` ←0) is masked behind it (FSMGen
  confesses one conflict at a time). **ROOT CAUSE (WHY + WHERE):** `rule_5`/`constraint_3` are **unconditional**
  (`IsfRule.condition == ""` → rendered with no guard, always active) and drive `PREQ`/`PACCEPT` ←1;
  `..._dyn_sigcon_0012`/`..._0011` are **guarded** (`(== PACCEPT 0)`) and drive ←0. SpecForge's existing
  conflict dedup `dedup_conflicting_rules` (`crates/specforge/src/ir/isf_ir.rs:2512`) keys on
  `(signal, condition)` — the **same-guard** overlap model — so an unconditional rule (guard `""`) and a guarded
  rule (guard `(== PACCEPT 0)`) hash to different keys and the conflict is never detected, yet FSMGen flags it:
  its `_condition_terms_prove_disjoint` (`subs/fsmgen/perl/FSM/Scheduler/ISF/LoweringIR.pm:10458`) can NEVER
  prove an absent/empty condition disjoint, so an unconditional rule's firing set ⊇ every guard → it overlaps
  any different-value rule on the same target. **DESIGN (validated empirically, GO):** after the same-guard
  dedup (which leaves ≤1 unconditional value per signal — two unconditional rules on one signal share key
  `(S,"")`), add a second pass `drop_unconditional_overlap_conflicts`: for each signal `S` with a kept
  unconditional driver value `V`, DROP (+ honest `ResidualDecisionPacket`) every other rule driving `S` to a
  value `≠ V`. This is **precise** — it drops exactly FSMGen's flagged case (unconditional-overlap) and nothing
  else, so a currently-strict-clean doc (which by construction CANNOT contain such a config, or FSMGen would
  already reject it) re-emits byte-identical. The **`(priority …)` escape-hatch alternative was tested and
  REJECTED**: it cleared one pair but `..._0012` then conflicted with the next unconditional `PREQ←1` rule
  (`rule_6`), so keeping the guarded minority rule would require asserting an ungrounded precedence over EVERY
  same-value unconditional rule — fabrication, against `[[feedback_isf_no_hacks]]`. The general
  different-guard-overlap case (two non-empty guards that overlap) is NOT in LPI (measured) and stays an honest
  out-of-scope residual. Empirical validation on the real FSMGen: dropping `..._0011` + `..._0012` →
  `success:true` / 0 diagnostics. ADR-0006: structural (empty-guard ⇒ overlaps-all), no chip/vendor/protocol
  name list. KM card `[[isf-unconditional-rule-overlap-conflict]]` (to write). `[[project_kg_isf_completeness]]`
  / `[[feedback_verify_fsmgen_before_fr]]`. **Frontier → after this lands, the renderable strict tally is
  28/28; remaining `.2`/`.3` north-star work is corpus-refresh + relation-completeness (Docling/RAM-gated).**

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.v`
- [x] **REPRODUCE / MEASURE** — `subs/fsmgen/bin/fsmgen --strict --check --json generated/adapters/isf/ihi0068_d_2021_10_amba_low_power_interface_specification/controller.isf` → `success:false`, 1 diagnostic `isf_conflicting_rule_writes` on `PREQ` (`rule_5` ←1 vs `temporal_temporal_signal_constraint_dyn_sigcon_0012` ←0); a second identical-shape `PACCEPT` conflict (`constraint_3` ←1 vs `..._0011` ←0) is masked behind it. A full fresh re-emit + FSMGen sweep further showed the AXI `ihi0022_l` / AHB `ihi0033_c` / AXI-Stream `ihi0051_b` wire golds + LTI `ihi0089_d` + NVMe were ALL FSMGen-FAIL on this same conflict class (the stale "27/28 clean" tally was wrong; cached wire-gold `.isf` byte-identical to fresh AND FAIL).
- [x] **ROOT CAUSE (WHY + WHERE)** — `dedup_conflicting_rules` (`crates/specforge/src/ir/isf_ir.rs`) keys conflict detection on `(signal, condition)` (same-guard only); an unconditional rule (`IsfRule.condition == ""`) and a guarded rule (`(== PACCEPT 0)`) hash to different keys so the overlap is missed, but FSMGen's `_condition_terms_prove_disjoint` (`subs/fsmgen/perl/FSM/Scheduler/ISF/LoweringIR.pm:10458`) never proves an absent condition disjoint, so the unconditional rule overlaps every guard on its target → `isf_conflicting_rule_writes` (`_build_conflict_issues:10888`). Confirmed by the FSMGen `--strict --check` diagnostic naming `rule_5` (guard-less) vs the guarded `..._0012`.
- [x] **ADDRESSED (verified)** — new pass `drop_unconditional_overlap_conflicts` (`isf_ir.rs`, after the same-guard dedup, before priority emit): per signal with an unconditional driver value `V`, drop every other rule driving it to `≠ V` + record `isf_unconditional_overlap_<name>` residual. LPI `controller.isf` (and `channel.isf`) now FSMGen `--strict --check --json` **success / 0 diagnostics**; exactly `..._0011` (`PACCEPT←0`) and `..._0012` (`PREQ←0`) dropped, both surfaced as `isf_unconditional_overlap_*` residuals (adapter `residual_decision_count` 4→6). Corpus: **6 docs FAIL→PASS** (AXI `ihi0022_l` manager, AHB `ihi0033_c` manager, AXI-Stream `ihi0051_b` transmitter, LPI controller, LTI `ihi0089_d` coherent_host, NVMe channel). +2 unit tests (`drop_unconditional_overlap_conflicts_drops_guarded_minority_keeps_unconditional`, `..._noop_without_unconditional_driver`).
- [x] **NO REGRESSION** — full re-emit diff (current binary WITH vs WITHOUT the change, via `git stash` of `isf_ir.rs`): exactly **7 `.isf` differ** (all 7 are current primary emits), every other emit byte-identical; FSMGen on the 7 → **6 FAIL→PASS, 0 PASS→FAIL**, the 1 still-FAIL (AXI+ACE `ihi0022_h_c` manager) fails on the **orthogonal** pre-existing `(port expr)` grammar (a spun-out `ISF-VALUE-WIDTH-EMIT` Non-Goal), unchanged. **Honest current-emit tally** (after cleaning 37 stale cache-cruft `.isf` from older binaries — `adapt` emits one primary actor per doc): **69 of 70 PASS**, the lone FAIL = `ihi0022_h_c` `(port expr)`. `kg-bench` **156/156**; `run_ci.sh` **GREEN** (lib **1708**, +2; clippy/fmt/rustdoc warning-deny + mdBook). **WIRE-BASED-100 orthogonal by construction** — the only Rust file changed is the `.isf` emitter `isf_ir.rs`; `eval-extraction` reads the IR, never the `.isf`.
- [x] **GENERICITY (ADR 0006)** — universal ISF-semantics rule (an empty-guard rule is unconditional ⇒ overlaps every guard on its target, mirroring FSMGen's own disjointness model), NOT a chip/vendor/protocol-name list; the `(priority …)` escape-hatch was empirically TESTED and rejected as ungrounded precedence (it cleared one pair then `rule_6` conflicted next — keeping the minority would require asserting a winner over every unconditional rule = fabrication, `[[feedback_isf_no_hacks]]`).
- [x] **LOCKSTEP** — README current-state bullet; book `pipeline/isf-adapter.md`; KM card `isf-unconditional-rule-overlap-conflict`; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md; `CORPUS-COVERAGE.2` strict tally corrected (Lever C).

- ID: `KG-ISF-COMPLETENESS.2a.vi` · Status: `done` (`2026-06-23`, measurement-first; LANDED + verified) ·
  Goal: **close the LAST ISF-emit strict-FAIL — the AMBA AXI+ACE (`ihi0022_h_c`) `(port expr)` grammar
  failure** so the renderable corpus reaches **70/70 FSMGen-`--strict`-clean** (the spun-out
  `ISF-VALUE-WIDTH-EMIT` Non-Goal, now the frontier after Lever C). **REPRODUCE (real FSMGen):** `fsmgen
  --strict --check --json` on the AXI+ACE `manager.isf` returns `success:false` / `Error: rule
  'constraint_48' assignment actions require '(port expr)'`. **ROOT CAUSE (WHY + WHERE):** the rule's drive
  VALUE is free PROSE — `(RLOOP the value that was presented on the ARLOOP signal)` / `(BLOOP … AWLOOP …)`
  (a loopback-tag obligation the extractor captured as a sentence, not a literal). FSMGen requires a rule
  assignment action's RHS to be a renderable value expression `(port expr)`, so a multi-word prose value
  breaks the file. The emitter renders rule drive values VERBATIM (`render_isf_control_expression`,
  `ir/isf_ir.rs`) with NO validity gate on the value (the existing gates cover enum members `.2a.iv` and
  value width `ISF-VALUE-WIDTH-EMIT`, not the rule's own scalar). **MEASURED scope (read-only over all 70
  current-emit `.isf`):** EXACTLY 4 prose-valued drive lines, ALL in `ihi0022_h_c` (`constraint_48`/`_49` +
  the two `tinv_sc_llm_sigcon_0061`/`_0062` duplicates); ZERO other docs carry a whitespace-bearing rule
  drive value, so a value-validity gate is corpus-safe by construction. **FIX (drop + honest residual):**
  new pass `drop_unrenderable_rule_values` (`isf_ir.rs`, before the width/dedup passes) drops a rule whose
  ANY drive value is not `is_safe_isf_scalar_value` (the existing non-empty/whitespace-free scalar test) and
  records an `isf_rule_value_<name>` residual. The prose value is unrecoverable — "the value PRESENTED on
  ARLOOP" is a temporal loopback, not the current `(port ARLOOP)`, so recovering a `(port expr)` would
  fabricate the timing — honest residual over fabrication (`[[feedback_isf_no_hacks]]`). ADR-0006: a
  structural value-shape test, no chip/vendor/protocol name list. Empirically validated: dropping the 4 →
  AXI+ACE FSMGen `success:true` / 0 diagnostics → **70/70**. KM `[[isf-unrenderable-rule-value-residual]]`.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.2a.vi`
- [x] **REPRODUCE / MEASURE** — `subs/fsmgen/bin/fsmgen --strict --check --json generated/adapters/isf/ihi0022_h_c_2021_01_amba_axi_and_ace_protocol_specification/manager.isf` → `success:false`, `Error: rule 'constraint_48' assignment actions require '(port expr)'`. Read-only scan of all 70 current-emit `.isf`: exactly 4 whitespace-bearing rule drive values, all in `ihi0022_h_c` (`constraint_48`/`_49`/`tinv_sc_llm_sigcon_0061`/`_0062` = `RLOOP`/`BLOOP` ← "the value that was presented on the ARLOOP/AWLOOP signal"); 0 elsewhere.
- [x] **ROOT CAUSE (WHY + WHERE)** — the `.isf` emitter renders a rule's drive value verbatim (`ir/isf_ir.rs`) with no value-validity gate; a constraint whose extracted value is PROSE (`the value that was presented on the ARLOOP signal`) emits `(RLOOP <prose>)`, which FSMGen rejects because a rule assignment action RHS must be a `(port expr)` (renderable value expression), not free text. Confirmed by the FSMGen `--strict --check` diagnostic naming `constraint_48`.
- [x] **ADDRESSED (verified)** — new pass `drop_unrenderable_rule_values` (`isf_ir.rs`, before the width/dedup passes) drops a rule whose any drive value fails `is_safe_isf_scalar_value` and records an `isf_rule_value_<name>` residual. AXI+ACE `manager.isf` now FSMGen `--strict --check` **success / 0 diagnostics**; exactly the 4 prose-valued rules (`constraint_48`/`_49`/`tinv_sc_llm_sigcon_0061`/`_0062`) dropped + residualized (`isf_rule_value_*`). +1 unit test (`drop_unrenderable_rule_values_drops_prose_keeps_scalars`).
- [x] **NO REGRESSION** — only `ihi0022_h_c` (the only doc with a prose-valued rule, measured) changes; every other current emit byte-identical. **Full FSMGen sweep over all 70 current-emit `.isf` → 70/70 strict-clean** (was 69/70; 0 PASS→FAIL). `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1709**, +1; clippy/fmt/rustdoc warning-deny + mdBook); WIRE-BASED-100 orthogonal by construction (emitter-only — `eval-extraction` reads the IR, never the `.isf`).
- [x] **GENERICITY (ADR 0006)** — a structural value-shape test (a rule drive value must be a non-empty whitespace-free scalar token to be renderable), NOT a chip/vendor/protocol-name list; the prose loopback value is unrecoverable as an exact `(port expr)` (the temporal "was presented" semantics), so honest residual over a fabricated `(port ARLOOP)` approximation.
- [x] **LOCKSTEP** — README current-state bullet; book `pipeline/isf-adapter.md`; KM card `isf-unrenderable-rule-value-residual`; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md; `CORPUS-COVERAGE.2` strict tally → 70/70.

- ID: `KG-ISF-COMPLETENESS.2b` · Status: `deferred` (measured-MARGINAL `2026-06-17`, read-only) · Goal:
  **ISF lowering-coverage visibility gauge** — make the lowering's per-surface coverage visible as adapter
  metadata + a `validate <intent-ir>` surface. **Measured marginal → not building now:** `.2` established
  the lowering is already faithful for grounded intent, so a coverage gauge is **noise-dominated** — a
  headline "257 / 16 463 temporal_invariants lowered" is dominated by the 16 179 ungroundable empty-subject
  invariants (ToC headings) `.2` showed are honest absences, so the number misleads rather than informs;
  the actionable signal (a grounded typed rule on a DECLARED signal that did not lower) is ≈0 on the wire
  docs and ~151 corpus-wide register-field paths that correctly belong to the register surface. An honest
  gauge would need full per-category breakdown for little operator value (the project already has rich
  `validate` inventories). Re-open only if an owner wants the categorized ISF-lowering residual surfaced.
  ADR-0006 (universal counts, no name list).
- ID: `KG-ISF-COMPLETENESS.3` · Status: `done` (`2026-06-24` — measurement DONE `2026-06-17`, CLOSED
  `2026-06-24`; read-only; owner-directed substantive north-star push after the owner pushed back on
  "buildable frontier exhausted") · Goal:
  **bar #2 relation-completeness** — investigate why whole docs carry actors+constraints but ZERO
  `actor_signal_relations` (`nvme`/`tilelink`/`wbspec`/`i2c`/`ccix`/VT-d/IOMMU), the exact "real-agent
  relation-incompleteness" the north star names. **Measured (read-only census over 78 evidence + 36 intent
  artifacts + content sampling): the 0-relation docs are NOT a relation-extraction gap.** Two distinct
  causes, cleanly separated:
  **(A) STALE IntentIR (recoverable).** The canonical `intent_ir.json` is stale relative to its
  `evidence_ir.json` corpus-wide: `tilelink_1_7_1` carries **39** relations in evidence but **0** in its
  (05-16-dated) intent; `tilelink_1_8_0` 40→0, `um10204` i2c 17→0, `wbspec` 1→0; plus a broader "intent
  older than evidence" set (gic_600 / mmu_700 / ihi0082 ATS / dti / opencapi×3 / usb4). A **deterministic
  `semantic`→`intent` rebuild** (no LLM, no Docling) recovers them — **PROVEN live on `tilelink_1_7_1`:
  relations 0→39, actor_ports 0→69, all 40 actors connected** (RAM steady 77%). Operational cause (evidence
  rebuilt under a sweep without cascading downstream; `converge` rebuilds the whole chain, stage commands do
  not auto-cascade), not a code bug.
  **(B) HONEST ABSENCE (not a gap).** `nvme`/`risc_v_iommu`/VT-d/`ccix` declare **~0 wire signals**
  (content-sampled: their drive/read "cues" are ToC entries, register-access descriptions `RO`/`RW`, and
  agent-MESSAGE/transaction prose — never agent-SIGNAL relations). These are register/command/coherency
  protocols whose intent lives in `register_records`/`message_field_records`/`transactions`, NOT in
  actor-signal relations. 0 relations is CORRECT; forcing them would FABRICATE (the north-star caution).
  **Genericity insight:** the actor-signal-relation surface is intrinsically wire-protocol-shaped;
  relation-completeness is the wrong bar dimension for register/message protocols. **Frontier → (i)** corpus
  refresh so the recovered relations land canonically (owner gap #2); **(ii)** a generic STAGE-STALENESS
  detector in `validate` so a stale downstream artifact silently dropping relations is surfaced, not hidden
  (candidate code slice — serves "the KG must be COMPLETE"). ADR-0006. Report
  `docs/research/relation-completeness-measurement.md`; KM `[[relation-completeness-staleness-vs-absence]]`.
  **CLOSED `2026-06-24` (verification-only, no code change).** Both frontier sub-steps satisfied:
  **(i) corpus refresh** — a full re-census over all 78 persisted `intent_ir.json` vs their
  `evidence_ir.json` finds **0 stale docs** (zero with `evidence>0 & intent==0`); the
  `CORPUS-COVERAGE.2` re-ingest sweep rebuilt the affected docs via `converge` (whole-chain cascade),
  so the recovered relations landed canonically — `tilelink_1_7_1` 33/33, `tilelink_1_8_0` 34/34,
  `um10204` I2C **17/17**, `gic_600` 101/101, `mmu_700` 25/25, ATS `ihi0082` 9/9, DTI 1/1, opencapi
  transaction-layer 15/15, USB4 13/13 (each `intent`==`evidence`); `wbspec` 0/0 (reclassified to
  honest-absence); 33 register/PHY/command docs at 0/0 = the correct (B) honest-absence class.
  **(ii) stage-staleness detector** — shipped + unit-tested as `CORPUS-COVERAGE.1`
  (`stage_staleness_relation_finding` → `semantic_stale_relations_dropped` /
  `intent_stale_relations_dropped`, category `stage_staleness`; KM `[[stage-staleness-validate-detector]]`).
  Bar #2 relation-completeness is therefore resolved for the recoverable class, correctly N/A for
  register/message protocols, and held at WIRE-BASED-100 = 1.000 on the wire class — no fabrication.
  Closure appended to the report; gates orthogonal (no code change); `check_doctrines.sh` GREEN.
- ID: `KG-ISF-COMPLETENESS.4` · Status: `done` (`2026-06-17`, read-only measurement, docs-only) · Goal:
  **bar #5/#6 re-assessment on the broader 78-doc corpus** — `.2` measured ISF-lowering fidelity over 36
  docs; `CORPUS-COVERAGE.0` then doubled the corpus to 78 (register/coherency/profile-heavy), so the
  resume-pointer standing candidate (b) was to re-check whether behavior/temporal rules are still
  carried-or-residual (no silent drop) at the new scale. **Measured (faithful Python replication of the
  three `IsfIr::from_intent_ir` `(rule)` filters over all 78 `intent_ir.json`):** `conditional_rules` 2237
  total / 516 lower / 1670 no-consequent / **51 undeclared-named**; `signal_constraints` 369 / 287 / 0 /
  **82**; `temporal_invariants` 29343 / 286 / 29030 empty-subject / **27**; the residualizing path
  (`temporal_rules` 320 + `actor_contracts` 211) is bar-#5-safe by construction
  (`[[isf-temporal-lowering-no-silent-drop]]`). The 30 700 empty-subject/no-consequent drops are correctly
  silent ("absence is not an event"). The **160 undeclared-NAMED-subject drops** (the only genuine bar-#5
  silent-loss candidate) were characterised item-by-item: 5 already on the register surface; the other 155
  are **field content + noise, NOT wire intent** — register/message FIELD mnemonics (NVMe `MTFA`/`HMDLAL`,
  CCIX `SAMH`/`ESMD`, RISC-V `DC.tc.SXL`; homed on the field surfaces by design — the `.isf` does not lower
  fields), DTI message-field obligations leaked into `signal_constraints` (DTI carries **no
  `message_field_records`** — a field-recognition gap, spun out), prose/hex noise (`DMA`/`TLB`/`PCI`/`IBM`/
  `FFFF`/`FFFFFFFF_FFFFFFFF`/`Reserved`/`this bit`), and the ATP `ihi0082` cluster of real AXI names from
  **garbled** VLM fragments (`"RREADY is RBR"`). **0 undeclared/silent drops on all four wire docs**
  (re-confirmed at 2× scale). **Conclusion: bar #5/#6 HOLDS on the broader corpus — no buildable
  lowering-residual lever; the adapter's silent skip of an undeclared-subject rule is CORRECT** (the genuine
  gaps are upstream: route field obligations to the field surface / filter prose noise — never residualize
  at the adapter, which would reintroduce exactly the noise `.2`/`.2b` rejected). CONFIRMS + extends `.2`
  to the doubled corpus. ADR-0006 (universal counts, structural classification, no name list).
  **Spun-out grounded observations** (each needs own measurement-first ownership): **(1)** DTI `ihi0088`
  message-field recognition gap → `EXTRACTION-QUALITY-GAUGE.FIELD`/`PDF-VARIANT-DIGESTION` (most
  actionable); **(2)** signal-inventory prose noise (`AMBA`/`ARM`/`APCI` minted as DTI signals) →
  signal-precision; **(3)** ATP VLM-fragment quality. Report
  `docs/research/behavior-temporal-lowering-completeness-78doc.md`; KM
  `[[behavior-temporal-lowering-broader-corpus]]`. `[[project_kg_isf_completeness]]` /
  `[[feedback_not_complete_attack_substantive_gaps]]`.

- ID: `KG-ISF-COMPLETENESS.5` · Status: `active` (umbrella; **measurement DONE `2026-06-24`**, read-only,
  docs-only; code → `.5.i`/`.5.ii`/`.5.iii` all LANDED `2026-06-24`) · Goal: **enum-surface fidelity (bar #6) — the generic-`TABLE`
  mega-enum conflation.** Surfaced by the `CORPUS-COVERAGE.2` re-ingests of JEDEC HBM2 (#28) and AMBA CHI C2C
  (#29), explicitly deferred by `.2a.iv` (Lever F) as "a future extraction-precision lever". **Measured
  (read-only, current binary + 78-doc persisted corpus; reproducer in the report):** the `.isf` emits a
  generic junk-named enum (HBM2 `(type TABLE (bits 6))`) fusing ~7 unrelated value-tables (29 dup values, 30
  sentence-fragment member names). **Origin = EXTRACTION, not the emitter:** `derive_encoding_enum_name`'s
  fallback (`evidence.rs:4457-4461`) names an unmatched encoding table after the first caption token passing
  `is_hardware_signal_token` (`evidence.rs:7106`, which accepts `Table`→`TABLE`), and `build_symbol_definitions`
  (`semantic.rs:2782-2789`) merges every same-named table into ONE enum by name (copied verbatim to IntentIR
  `intent.rs:189`; lowered faithfully `isf_ir.rs:889-912`). **Corpus:** 56/78 docs carry a generic-named enum
  (96 generic `TABLE`/`FIGURE`/`DATA`/… vs 493 real); ~95 reach `.isf`. **Key insight:** a name-only gate
  catches 96 generic but MISSES **271 real-named-but-junk** enums (`COMMAND`/`DWORD_MISR`/`AMBA` — fragment
  names + restarting values; only 222 of 493 real enums are clean) → the load-bearing signal is **member
  quality**, the name is the symptom. **Decision: GO, extraction-side, decomposed** (`.5.i` safe name-gate +
  emitter orphan-type fix; `.5.ii` member-quality gate). Universal structural rule, ADR-0006 (no chip-name
  list). **WIRE-BASED-100:** scores ORTHOGONAL (enums unscored — `eval-extraction` golds carry no enum), but
  the `.isf` BYTES change on all 4 wire golds (each emits a junk `TABLE`; AHB's fuses HTRANS+HSIZE which
  already have correct enums) — a strict IMPROVEMENT, not byte-identical, needing a deliberate snapshot
  refresh. No code → all oracles orthogonal by construction; `check_doctrines.sh` GREEN; KM 125→126. Report
  `docs/research/generic-enum-conflation-measurement.md`; KM `[[generic-enum-conflation]]`.
  `[[feedback_scoring_rigor]]` / `[[feedback_avoid_denylists_prefer_structural]]`.

- ID: `KG-ISF-COMPLETENESS.5.i` · Status: `done` (`2026-06-24`, fresh focused session per the high-stakes
  gate-code rule; LANDED + WIRE-BASED-100-verified + corpus-censused) · Goal: **the safe extraction-side
  fallback name-gate + the emitter orphan-`(type)` fix.** (1) `derive_encoding_enum_name` (`evidence.rs:4457-4461`)
  must NOT return a name that is merely a document-structure token — gate it positively (the candidate token
  must independently be a declared signal / column header) and return `None` otherwise (a `None` fallback is
  the EXISTING contract: both call sites `continue` → no enum minted → honest residual). Kills all 96 generic
  enums AND the conflation (the merge is keyed on the shared name). (2) `isf_ir.rs:403-409` emits ALL
  `self.types` unconditionally → a Lever-F-residualized enum still leaves an orphan `(type TABLE (bits 6))`
  line; gate the types block by `emitted_enums()`. **Design (locked by read-only probe `2026-06-24`):** the
  defect lives ONLY in the fallback; the genuinely-named enums (HBM2 `COMMAND`/`EXTEST_RX`/`DWORD_MISR`,
  wire-gold `PPROT`/`HTRANS`/`HSIZE`/`HPROT`/`HRESP`/`WLAST`/`RLAST`/`BRESP`/`RRESP`/`TKEEP`) come from the
  **signal-match path** (the candidate IS a declared signal matched in caption/section/header, returned BEFORE
  the fallback ever runs), so the fallback gate cannot touch them — they stay byte-identical. The fallback
  produces only document-structure/caption-word names (`TABLE`/`FIGURE`/`COLUMN`/`DATA`/`ANNEX`, and the AMBA
  caption words `AMBA`/`BYTE`/`READ`/`CACHE`/`RELEASE`), none of which is a declared signal or a column header
  → all dropped. The structural gate is therefore strictly BETTER than a name-only gate (it also catches the
  fallback-origin real-named-but-junk like `AMBA`); the SIGNAL-MATCH-origin real-named-but-junk
  (`COMMAND`/`DWORD_MISR` — declared signals with fragment members) is untouched and stays `.5.ii`'s job.
  Acceptance: see the dedicated checklist below.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.5.i` — DONE `2026-06-24`
- [x] **REPRODUCE / MEASURE** — current `.isf` emits a generic junk enum: HBM2 `hbm.isf` `(type TABLE (bits 6))` fusing ~7 value-tables (57 members, 29 dup values, 30 fragment names); corpus-wide 96 generic-named enums across 56/78 docs reach `.isf`. Wire golds (BEFORE, `grep '(type'`): APB `(type TABLE (bits 3))` beside real `PPROT`; AHB `(type TABLE (bits 5))`+`(type AMBA (bits 4))` beside `HTRANS`/`HSIZE`/`HPROT`/`HRESP`; AXI `(type TABLE (bits 8))`+`BYTE`/`READ`/`CACHE`/`RELEASE`/`AMBA` beside `WLAST`/`RLAST`/`BRESP`/`RRESP`; SWP `(type TABLE (bits 5))`+`(type ANNEX (bits 2))` beside `CLT`. Report `docs/research/generic-enum-conflation-measurement.md`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `derive_encoding_enum_name` fallback (`crates/specforge/src/ir/evidence.rs:4457-4461`) names an unmatched encoding table after the FIRST caption whitespace token passing `is_hardware_signal_token` (`evidence.rs:7106`, which accepts `Table`→`TABLE`); `build_symbol_definitions` (`semantic.rs:2782-2789`) then MERGES every same-named table into one enum by name (→ the conflation), copied to IntentIR (`intent.rs:189`), lowered by `isf_ir.rs:889-912`. The emitter ALSO emits all `self.types` unconditionally (`isf_ir.rs:403-409`), orphaning a `(type)` whose enum is dropped by `isf_enum_is_emittable`. Both call sites (`scan_encoding_tables_by_signal_anchor`:4668 `Some(known_signals)`; `synthesize_encoding_declarations`:11907 `None`) funnel through the one fallback. Probe `2026-06-24`: HBM2's real enums are signal-match-origin; only `TABLE`/`COLUMN`/`DATA` are fallback-origin.
- [x] **ADDRESSED (verified, measured per-item)** — two edits: (1) `derive_encoding_enum_name` fallback keeps the candidate ONLY when independently evidenced (declared signal OR column-header reference token), else `None`; (2) emitter gates the `(types)` block by `emitted_enums()`. Per-doc before→after (baseline vs gated binary; full `evidence→semantic→intent→adapt` rebuild from persisted source_ir): **HBM2** types 15→6 (drop `TABLE`/`COLUMN`/`COMMAND` + 6 orphan types; keep `UPDATEWR`/`EXTEST_RX`/`DATA`/`HBM_RESET`/`DWORD_MISR`/`CHANNEL_ID` byte-identical); **APB** `PPROT`+`TABLE`→`PPROT`; **AHB** drop `TABLE`/`AMBA`, keep `HTRANS`/`HSIZE`/`HPROT`/`HRESP`; **AXI** drop `TABLE`/`BYTE`/`READ`/`CACHE`/`RELEASE`/`AMBA`, keep 22 real signal enums; **SWD** drop 9 (`TABLE`/`REGISTER`/`ACCESSING`/`OF`/`USAGE`/`DRIVES`/`ARM`/`ATTRIBUTES`/`OK`), keep `TDI`; **SWP** `TABLE`/`CLT`/`ANNEX`→none; worst-offender type-counts amd 34→5, gic_600 23→6, coresight 19→5, cortex_a76 16→7, CHI 14→4, CHI-C2C 7→1. **Corpus census (33 rebuildable docs):** generic-named enums **82→8**, total enum records **422→105**; the 8 survivors are document-evidenced column-header/declared-signal tokens (CCIX "Table of Contents" headers etc.) → honest `.5.ii` residuals. Report §`.5.i LANDED`; +4 unit tests (3 fallback-gate + strengthened orphan-type test) green.
- [x] **NO REGRESSION** — **WIRE-BASED-100 = 1.000, before == after (PROVEN)** by rebuilding each gold doc's evidence with BOTH binaries and running `eval-extraction --provider skip`: APB signal_constraint 6/6 · relation 5/5; AHB 6/6·6/6; AXI 3/3·6/6; APB/AHB/AXI temporal 3/3·4/4·3/3; SWD relation 1/1 + documented promotion-only 0/1 — all identical old-vs-new. nvme-registers + i2c-signals golds identical old-vs-new. **`kg-bench` 156/156.** **`run_ci.sh` GREEN** (lib **1712** passed; warning-deny clippy/fmt/rustdoc + mdBook). Every affected `.isf` stays **FSMGen `--strict` 0 diagnostics** (renderable corpus strict-clean). The real-named signal-match enums are **byte-identical**; the only `.isf` body change beyond the enum block is on AXI/nvme, where off-gold junk constraints derived from dropped-enum `discovered_values` disappear (distinct constraint facts identical, score-orthogonal) — corrects the `.5` "orthogonal because unscored" reasoning to "orthogonal because off-gold + filtered, proven by the eval".
- [x] **GENERICITY (ADR 0006)** — universal structural rule: a fallback enum name must be independently evidenced (a declared signal OR a column-header reference token of the table), NOT a chip/vendor/structure-word name list. Proven structural, not a denylist: `DATA` is KEPT where it is a genuine gic_600 signal but DROPPED where it is a bare HBM2 caption word; the generic-name SET in the report is a measurement aid, never the gate.
- [x] **LOCKSTEP** — README current-state bullet; book `pipeline/isf-adapter.md` enum-fidelity note; KM card `[[generic-enum-conflation]]` updated to LANDED; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md.

- ID: `KG-ISF-COMPLETENESS.5.ii` · Status: `done` (**measurement DONE + gate LANDED `2026-06-24`**,
  before/after-WIRE-BASED-100-verified) · Goal: **the member-quality gate** for the
  residual junk enums a NAME gate cannot reach. After `.5.i` the residual is (a) the **8 document-evidenced
  generic-named survivors** (`DATA`/`TABLE`/`CACHE`/`ENCODING`/`ATTRIBUTES` the `.5.i` gate KEEPS because the
  token IS a declared signal or column header — e.g. CCIX "Table of Contents" cells, still cross-table-
  conflated) + (b) the **signal-match-origin junk-member enums** (NAME is a real signal, MEMBERS are
  sentence fragments — HBM2 `DWORD_MISR`, AXI `BRESP`/`RRESP`/`ARCACHE`).
  **MEASUREMENT DONE `2026-06-24`** (read-only over all 78 persisted IntentIR docs / 561 enums / 12 509
  members; report `docs/research/generic-enum-conflation-measurement.md` §`.5.ii measurement`; KM
  `[[generic-enum-conflation]]`) — it **OVERTURNS the recorded `.5` plan**:
  - **The gate is PER-MEMBER, not per-enum.** A whole-enum drop destroys real codes — the surviving junk
    enums are CONFLATIONS of a junk table and a clean table: AXI-gold `BRESP` fuses 7 prose fragments +
    `BRESP_WIDTH` WITH the 8 genuine codes `OKAY/EXOKAY/SLVERR/DECERR/DEFER/TRANSFAULT/RESERVED/UNSUPPORTED`.
    The fix drops the prose MEMBERS and keeps the codes (BRESP 16→9).
  - **Value-restart is NOT a junk signal.** AHB-gold `HPROT` restarts (3 fused sub-encodings) yet every one
    of its 15 members is a clean identifier — a restart-gate would be a false positive losing real intent.
    Restart is dropped from the rule; conflation-of-clean-tables stays kept (honest residual: sub-enum
    splitting deferred).
  - **The load-bearing signal is per-member NAME shape — an English sentence-SPINE token.** The synthesis
    sanitizes a name-cell to `[A-Z0-9_]`, so a whole prose sentence becomes one `_`-joined member name; a
    real hardware symbol never contains a copula/auxiliary/modal (`IS`/`ARE`/`BE`/`HAS`/`MUST`/…),
    article/demonstrative (`THE`/`THIS`/…), or relativizer/subordinator (`WHICH`/`WHEN`/`IF`/`BECAUSE`/…).
    A member with a spine token is a prose fragment → drop. **Collisions EXCLUDED** (the `.1a` discipline):
    `A`/`I`/`ITS`/`CAN`/`MAY`/`AM` (article-vs-suffix, GIC `ITS`, CAN-bus, month). Universal grammar, ADR
    0006 — NOT a chip/structure-word name list.
  - **Precision/recall (per-item, `[[feedback_scoring_rigor]]`):** clean anchor **0/115 flagged → precision
    1.000**; junk anchor **269/269 caught → recall 1.000**; corpus-wide 3 781/12 509 (30.2 %) members drop.
    Wire-gold blast radius: `BRESP` 16→9 (recovers codes), `PPROT`/`ARCACHE`/`AWCACHE`/`RLAST`/`WLAST`→0
    (pure prose dropped), `HSIZE`/`HTRANS`/`HRESP` untouched. **Honest residuals deferred:** glossary
    `SEE…` refs, front-matter/ToC members, section-caption (`B2_3_1_…`) members, `_WIDTH` leaks,
    value-restart-with-clean-members.
  - **GO** — land a per-member sentence-spine fragment drop in `synthesize_encoding_declarations_for_enum`
    (`evidence.rs`): skip a member whose sanitized name carries a spine token; an emptied enum is not minted
    (existing no-statements → no-`SymbolDefinition` contract → honest residual). Byte-changing on wire golds
    (strict improvement) → the code slice needs a before/after WIRE-BASED-100 eval, NOT a byte-identical
    argument; `kg-bench` + FSMGen `--strict` + `run_ci.sh` hard gates.
  - **LANDED `2026-06-24`** — `is_prose_fragment_member_name` + the `PROSE_SENTENCE_SPINE_WORDS` const
    (`ir/evidence.rs`) gate the member loop in `synthesize_encoding_declarations_for_enum` (the single
    member-synthesis seam, so both `None`/`Some(known_signals)` paths are covered): a member whose
    `_`-token set carries a spine word is skipped, so a conflated enum keeps its codes and a pure-prose
    table emits no statements → no enum. +4 tests (collision-exclusion drift guard, predicate unit, two
    integration tests on the synthesis seam). **Verified:** WIRE-BASED-100 = **1.000 before==after** (all 10
    seeds — before/after eval on gold evidence rebuilt with the preserved baseline vs gated binary; scored
    surface byte-identical); AXI `manager.isf` now emits `(BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)(DECERR 3)
    (DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED 7))` (codes recovered) + APB pass FSMGen `--strict`
    `success`; `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1716**, +4). Acceptance checklist below.

- ID: `KG-ISF-COMPLETENESS.5.iii` · Status: `done` (**measurement DONE + gate LANDED `2026-06-24`**,
  before/after-WIRE-BASED-100-verified) · Goal: **the deeper enum member-quality residual classes** `.5.ii` left
  open (glossary `SEE…`, front-matter/ToC, section-caption `B2_3_1_…`, `_WIDTH` parameter leaks,
  value-restart-of-all-clean) — measure them per-item, build only the one that is both material and cleanly
  gateable. **MEASUREMENT DONE `2026-06-24`** (read-only over the 78 persisted IntentIR docs; reproducer
  `scripts/measure_enum_width_leak.py`; report `docs/research/generic-enum-conflation-measurement.md`
  §`.5.iii measurement`; KM `[[generic-enum-conflation]]`):
  - **Most deeper-residual members are SUBSUMED by `.5.i`.** Of 54 `_WIDTH` members, 47 sit in
    generic-named enums (`TABLE`/`TRANSLATION`/`BIT`) that `.5.i` already drops whole → no `.isf` reach;
    likewise the 319 section-caption survivors are dominated by `.5.i`-dropped `DEBUG`-class enums.
  - **The `_WIDTH` leak DOES reach a wire-gold `.isf` and is materially damaging.** The 7 remaining
    `_WIDTH` members are in real-signal-named enums in the AXI **gold** `ihi0022_l`: `(BRESP (BRESP_WIDTH 0)
    (OKAY 0)…)` duplicates value `0`, `(RRESP (RRESP_WIDTH 0))` REPLACES the real RRESP codes,
    `(AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1))` is pure junk, `(AWCMO (AWCMO_WIDTH 0)…)` duplicates `0`.
    A width PARAMETER (`Enum BRESP BRESP_WIDTH = 0.`) leaked into the value enum — a false bar-#6 fact
    (unscored by WIRE-BASED-100, hence held at 1.000 while the `.isf` carried junk).
  - **The discriminator is FP-free and document-grounded (ADR 0006):** drop a member named `<X>_WIDTH`
    iff `X` is a declared signal OR the enum's own name. All 6 caught prefixes are TRUE declared signals;
    the corpus FP set is EMPTY (no legit `FULL_WIDTH`/`HALF_WIDTH` value exists, and the declared-signal
    rule would never catch one — `FULL`/`HALF` are not signals). Per-member, not per-enum (keeps BRESP's
    codes; empties RRESP/AXSNOOP → honest residual).
  - **The other classes are NO-GO** — section-caption/table-ref has no FP-free gate (leading `[A-Z]?digit`
    collides with real codes `D1`/`L2`); restart-of-clean has no fidelity defect (all members real, mostly
    `.5.i`-dropped); glossary/front-matter are tiny + name-ish. Honest residuals.
  - **GO** — land a per-member `_WIDTH` parameter-leak drop in `synthesize_encoding_declarations_for_enum`
    (`evidence.rs`), parallel to the `.5.ii` spine gate; thread the existing `known_signals` set (in scope
    at the signal-match caller `evidence.rs:4700`) for the declared-signal arm, enum-self-name needs no
    plumbing. Byte-changing on the AXI wire gold → before/after WIRE-BASED-100 eval REQUIRED (a strict
    improvement: scored surface byte-identical, `manager.isf` recovers BRESP/AWCMO + drops RRESP/AXSNOOP).
    `kg-bench` + FSMGen `--strict` + `run_ci.sh` hard gates. Acceptance checklist below.
  - **CODE LANDED `2026-06-24`** — `is_width_parameter_leak_member(member, enum, known_signals)` +
    a `continue`-skip in `synthesize_encoding_declarations_for_enum`'s member loop right after the
    `.5.ii` spine gate (`ir/evidence.rs`): a member named `<X>_WIDTH` where `X` is the enum's own name
    OR a declared signal (the `known_signals` set, now threaded from the signal-match caller; the `None`
    caller covers the enum-self case) is a width PARAMETER → skipped; a pure-parameter enum empties (no
    statements → no `SymbolDefinition` → honest residual). +2 tests (predicate FP-guard incl. `FULL_WIDTH`
    kept; synthesis seam keeps BRESP codes / empties AXSNOOP). **Verified:** AXI evidence rebuild drops
    EXACTLY the 7 leaks (`BRESP/RRESP/RCHUNKNUM/RCHUNKSTRB/AWSNOOP/ARSNOOP/AWCMO _WIDTH`; statements
    6414→6407; non-Enum statement set byte-identical); AXI `manager.isf` now emits `(BRESP (OKAY 0)…)`
    (8 clean codes, no dup) + `(AWCMO (CLEAN_AND_INVALIDATE 0)(CLEAN_ONLY 1))` and the false
    `(RRESP (RRESP_WIDTH 0))`/`AXSNOOP`/`RCHUNK*` enums are gone; FSMGen `--strict --check` **success / 0
    diagnostics**. **WIRE-BASED-100 = 1.000 before==after** (AXI before/after eval identical; APB/AHB/SWD/i2c
    evidence byte-identical old-vs-new → gate inert). `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1718**,
    +2). Acceptance checklist below. The `SECSID_WIDTH`/`SID_WIDTH`/`SSID_WIDTH` self-named pseudo-enums
    (enum NAME ends `_WIDTH`, prefix not a declared signal) stay an honest residual (out of scope — a
    name-level case overlapping `.5.i`).

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.5.ii` — DONE `2026-06-24`
- [x] **REPRODUCE / MEASURE** — read-only census over all 78 persisted IntentIR docs (561 enums / 12 509 members). The `.5.i` name-gate cannot reach enums whose NAME is a real signal but whose MEMBERS are junk: AXI-gold `BRESP` is 16 members = 7 prose fragments + `BRESP_WIDTH` FUSED with the 8 genuine codes; AHB-gold `HPROT` has value-restart but 15 clean members. Report `docs/research/generic-enum-conflation-measurement.md` §`.5.ii measurement`.
- [x] **ROOT CAUSE (WHY + WHERE)** — `synthesize_encoding_declarations_for_enum` (`crates/specforge/src/ir/evidence.rs`) sanitizes a name-cell to `[A-Z0-9_]`, so a prose sentence in a name-cell becomes one `_`-joined member name; `build_symbol_definitions` (`semantic.rs`) then accumulates those members by enum name. The per-enum drop the `.5` packet proposed is wrong (loses BRESP's codes; false-positives HPROT) — the gate must be per-MEMBER on NAME shape.
- [x] **ADDRESSED (verified, measured per-item)** — added `PROSE_SENTENCE_SPINE_WORDS` (38 copula/aux/modal/article/demonstrative/relativizer/subordinator words; collisions `a`/`i`/`its`/`can`/`may`/`am` EXCLUDED) + `is_prose_fragment_member_name`; the member loop `continue`-skips a fragment member. Per-item: clean anchor 0/115 flagged (precision 1.000), junk anchor 269/269 caught (recall 1.000). Landed wire-gold census (baseline vs gated evidence): AXI enum-member statements 148→91, `BRESP` recovers `OKAY/EXOKAY/SLVERR/DECERR/DEFER/TRANSFAULT/RESERVED/UNSUPPORTED` (`.isf` emits them), `ARTAGOP`/`AWTAGOP`/`RLAST`/`WLAST`→0 (pure prose), APB 7→4. CCIX `TABLE` stays 27 clean-identifier members (the cross-table-merge-of-clean residual — correctly untouched).
- [x] **NO REGRESSION** — **WIRE-BASED-100 = 1.000, before == after (PROVEN)** by rebuilding each gold doc's evidence with BOTH binaries (preserved baseline = post-`.5.i`; gated = post-`.5.ii`) and running `eval-extraction --provider skip`: APB constraint 6/6·relation 5/5; AHB 6/6·6/6; AXI 3/3·6/6; temporal APB/AHB/AXI 3/3·4/4·3/3; SWD relation 1/1 + documented promotion-only 0/1; SWD-derivation 11/11·4/4·13/13; i2c declared_signal 6/6 — all identical old-vs-new (only the `evidence_root`/temp-path lines differ). `kg-bench` 156/156. `run_ci.sh` GREEN (lib **1716**, +4 tests; warning-deny clippy/fmt/rustdoc + mdBook). Rebuilt AXI + APB `.isf` pass FSMGen `--strict --check --json` (`success: true`).
- [x] **GENERICITY (ADR 0006)** — universal English grammar (a closed-class sentence-spine lexicon), NOT a chip/structure-word name list; the collision exclusions are the SAME structural discipline `.1a` applies. Proven structural: a member like `WRITE_BACK__SHAREABLE` or `REPAIR_LANE_0` is kept anywhere; only sentence-shaped members drop. Drift-guard test pins the collision exclusions.
- [x] **LOCKSTEP** — README current-state bullet (`.5.ii`); book `pipeline/isf-adapter.md` enum-fidelity note extended; KM card `[[generic-enum-conflation]]` → `.5.ii` LANDED + `KNOWLEDGE_MAP.md` regen; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md. **`.5` enum-surface fidelity is now built** (`.5.i` name-gate + `.5.ii` member-gate); residual deeper member-quality classes are honest residuals.

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.5.iii` (MEASUREMENT) — DONE `2026-06-24`
- [x] **REPRODUCE / MEASURE** — read-only census over all 78 persisted IntentIR docs of the five deeper member-quality classes `.5.ii` deferred. Tracked deterministic reproducer `scripts/measure_enum_width_leak.py` (RAM-safe; no VLM/Docling/rebuild). Report `docs/research/generic-enum-conflation-measurement.md` §`.5.iii measurement`. Numbers: 54 `_WIDTH` members (7 caught in real-named enums / 47 in `.5.i`-dropped generics / **0** legit-width-value false positives); 319 section-caption survivors (no FP-free gate); 101 restart-of-clean enums (no defect).
- [x] **ROOT CAUSE (WHY + WHERE)** — `synthesize_encoding_declarations_for_enum` (`ir/evidence.rs`) admits a config/parameter row (`Enum BRESP BRESP_WIDTH = 0.`, confirmed in `generated/evidence_ir/ihi0022_l*`) as an encoding member; the width PARAMETER of signal X is not an encoding VALUE. It reaches the AXI gold `manager.isf`: `(BRESP (BRESP_WIDTH 0)(OKAY 0)…)` dup-value, `(RRESP (RRESP_WIDTH 0))` replaces real codes, `(AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1))` pure junk.
- [x] **DECISION (per-item, `[[feedback_scoring_rigor]]`)** — **GO** on a per-member `_WIDTH` parameter-leak gate (drop `<X>_WIDTH` iff `X` is a declared signal OR the enum name); precision 1.000 / FP set empty corpus-wide (no `FULL_WIDTH`-style value exists, and the declared-signal arm would never catch one). **NO-GO** on section-caption (leading `[A-Z]?digit` collides with real codes `D1`/`L2`), restart-of-clean (no fidelity defect — `.5.ii` proved restart is not junk), glossary/front-matter (tiny + name-ish) → honest residuals.
- [x] **GENERICITY (ADR 0006)** — the gate is document-grounded (the doc's own declared-signal set / the enum's own name), NOT a structure-word name list; a legit link-width enum (`FULL_WIDTH`/`HALF_WIDTH`) is preserved because its prefix is not a declared signal. Same "independently evidenced" discipline as `.5.i`.
- [x] **NO CODE → ORACLES ORTHOGONAL** — measurement/docs-only slice; no Rust touched → WIRE-BASED-100 / register+wire golds / `kg-bench` orthogonal by construction; `scripts/check_doctrines.sh` GREEN (memory-arch + knowledge-map + task-acceptance).
- [x] **LOCKSTEP** — report §`.5.iii measurement` + tracked reproducer; `.5.iii` node added; KM card `[[generic-enum-conflation]]` extended (`.5.iii`) + `KNOWLEDGE_MAP.md` regen; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md / `docs/TASK_TREE.md` index frontier. **Frontier → the `.5.iii` CODE slice** (the `_WIDTH` gate; before/after WIRE-BASED-100 eval required).

## Acceptance Checklist (enforced) — `KG-ISF-COMPLETENESS.5.iii` (CODE) — DONE `2026-06-24`
- [x] **ROOT CAUSE (WHY + WHERE)** — `synthesize_encoding_declarations_for_enum` (`crates/specforge/src/ir/evidence.rs`) admitted a width-PARAMETER row as an encoding member: the AXI gold `evidence_ir.json` carries `Enum BRESP BRESP_WIDTH = 0.` (+ 6 more), and the `.isf` emitted `(BRESP (BRESP_WIDTH 0)(OKAY 0)…)` (dup value `0`) / `(RRESP (RRESP_WIDTH 0))` (real codes replaced) / `(AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1))` — a false bar-#6 `.isf` fact. Localized by the `.5.iii` measurement (reproducer `scripts/measure_enum_width_leak.py`): 7 leaks in real-named enums, all in `ihi0022_l`, all prefixes declared signals.
- [x] **ADDRESSED (verified, measured per-item)** — added `is_width_parameter_leak_member(member, enum, known_signals)` + a `continue`-skip in the member loop after the `.5.ii` spine gate; threaded `known_signals` (in scope at the signal-match caller, `evidence.rs:4708 Some(known_signals)`) for the declared-signal arm, enum-self-name for the `None` caller. Measured per-item: AXI evidence rebuild drops EXACTLY the 7 `_WIDTH` leaks (`BRESP/RRESP/RCHUNKNUM/RCHUNKSTRB/AWSNOOP/ARSNOOP/AWCMO`; statements 6414→6407; the non-Enum statement TEXT set byte-identical old-vs-new); AXI `manager.isf` now emits `(BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)(DECERR 3)(DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED 7))` (8 clean codes) + `(AWCMO (CLEAN_AND_INVALIDATE 0)(CLEAN_ONLY 1))`; the false `RRESP`/`RCHUNKNUM`/`RCHUNKSTRB`/`AXSNOOP` `_WIDTH`-only enums are gone (emptied → honest residual); FSMGen `--strict --check --json` **success / 0 diagnostics**.
- [x] **NO REGRESSION** — **WIRE-BASED-100 = 1.000 before==after (PROVEN)**: AXI before/after `eval-extraction --provider skip` on baseline-vs-gated evidence is byte-identical (seed_axi constraint 4/4 + relation 6/6, seed_axi_temporal 3/3); APB/AHB/SWD/i2c evidence rebuilt with both binaries is **byte-identical** (the gate is inert — the 7 leaks are AXI-only). Full wire eval on canonical (new) evidence: APB 6/6·6/6·3/3, AHB 6/6·6/6·4/4, AXI 4/4·6/6·3/3, SWD relation 1/1 (+ documented promotion-only constraint 0/1), SWD-derivation 11/11·4/4·13/13, i2c 6/6. `kg-bench` **156/156**. `run_ci.sh` GREEN (lib **1718**, +2 tests; warning-deny clippy/fmt/rustdoc + mdBook). FSMGen `--strict` success on the rebuilt AXI `.isf`.
- [x] **GENERICITY (ADR 0006)** — document-grounded gate (the enum's own name + the document's declared-signal set), NOT a structure-word name list; corpus false-positive set EMPTY — no legit `FULL_WIDTH`/`HALF_WIDTH` value exists, and the declared-signal arm never catches one because `FULL`/`HALF` are not declared signals (the predicate unit test pins `FULL_WIDTH` KEPT). Same "independently evidenced" discipline as `.5.i`.
- [x] **LOCKSTEP** — report §`.5.iii LANDED`; KM card `[[generic-enum-conflation]]` → `.5.iii` LANDED + `KNOWLEDGE_MAP.md` regen; README current-state bullet (`.5.iii`); book `pipeline/isf-adapter.md` enum-fidelity note extended; CHANGES.md / DEVELOPMENT_NOTES.md / LIVE_ACHIEVEMENT_STATUS.md / MEMORY.md / `docs/TASK_TREE.md` index. **`.5` enum-surface fidelity: `.5.i` name-gate + `.5.ii` spine member-gate + `.5.iii` `_WIDTH` member-gate** — the AXI gold `.isf` enum surface is now faithful; the `*_WIDTH`-named self-pseudo-enums are an honest residual.

## Changelog

- `2026-06-24`: **`.5.iii` DONE — CODE: per-member `_WIDTH` parameter-leak enum gate.** Same fresh-session
  PNT continuation (the two slices of `.5.iii`: measurement, then code). Added
  `is_width_parameter_leak_member` (`ir/evidence.rs`) + a `continue`-skip in
  `synthesize_encoding_declarations_for_enum`'s member loop after the `.5.ii` spine gate: a member named
  `<X>_WIDTH` where `X` is the enum's own name OR a declared signal (the `known_signals` set, now threaded
  from the signal-match caller) is a width PARAMETER → dropped, so a polluted enum keeps its codes (AXI
  `BRESP` keeps OKAY/EXOKAY/SLVERR/DECERR/…) and a pure-parameter enum empties (no statements → no
  `SymbolDefinition` → honest residual). +2 tests. **Verified per-item:** AXI evidence rebuild drops
  EXACTLY the 7 leaks (statements 6414→6407; non-Enum statement set byte-identical); the AXI `manager.isf`
  now emits `(BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)(DECERR 3)(DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED
  7))` (8 clean codes, no dup) + `(AWCMO (CLEAN_AND_INVALIDATE 0)(CLEAN_ONLY 1))`, and the false
  `(RRESP (RRESP_WIDTH 0))`/`AXSNOOP`/`RCHUNK*` enums are gone; FSMGen `--strict --check` **success / 0
  diagnostics**. **NO REGRESSION:** WIRE-BASED-100 = **1.000 before==after** (AXI before/after eval
  identical; APB/AHB/SWD/i2c evidence byte-identical old-vs-new → gate inert); `kg-bench` 156/156;
  `run_ci.sh` GREEN (lib **1718**, +2). ADR-0006 (document-grounded, FP set EMPTY — `FULL_WIDTH` kept).
  **`.5` enum-surface fidelity now has all three member gates** (`.5.i` name + `.5.ii` spine + `.5.iii`
  `_WIDTH`); the `*_WIDTH`-named self-pseudo-enums (SECSID_WIDTH/SID_WIDTH/SSID_WIDTH) stay an honest
  residual. Report §`.5.iii LANDED`; KM `[[generic-enum-conflation]]` → `.5.iii` LANDED. Frontier →
  KG-ISF-COMPLETENESS has no further immediately-buildable leaf (`.1c.ii` upstream-NLP-gated, `.2b`
  measured-marginal); PNT advances to the next active tree.

- `2026-06-24`: **`.5.iii` MEASUREMENT DONE → GO on the `_WIDTH` parameter-leak gate (read-only, docs-only).**
  Fresh-session PNT continuation off `.5.ii` (the first active tree, north star; the resume pointer flagged
  the `.5.ii` deeper member-quality residuals as the re-open candidate "needing their own measurement"). A
  read-only census over all 78 persisted IntentIR docs (tracked reproducer `scripts/measure_enum_width_leak.py`)
  measured the five deferred classes. **The deeper classes are largely SUBSUMED by `.5.i`** (47/54 `_WIDTH`
  members and the bulk of the 319 section-caption survivors sit in generic-named enums `.5.i` already drops
  whole). **But the `_WIDTH` leak reaches the AXI WIRE-GOLD `.isf` and is materially damaging:** 7 members in
  real-signal-named enums (`BRESP`/`RRESP`/`RCHUNKNUM`/`RCHUNKSTRB`/`AXSNOOP`/`AWCMO`, all in `ihi0022_l`)
  emit `(BRESP (BRESP_WIDTH 0)(OKAY 0)…)` (dup value), `(RRESP (RRESP_WIDTH 0))` (real codes replaced),
  `(AXSNOOP (AWSNOOP_WIDTH 0)(ARSNOOP_WIDTH 1))` (pure junk) — a width PARAMETER mis-read as an encoding
  VALUE (`Enum BRESP BRESP_WIDTH = 0.`). Unscored by WIRE-BASED-100 (enum surface orthogonal — why it held
  1.000 while the `.isf` carried junk). **Clean, FP-free, ADR-0006 discriminator:** drop `<X>_WIDTH` iff `X`
  is a declared signal OR the enum's own name (all 6 caught prefixes are declared signals; corpus FP set
  EMPTY — no `FULL_WIDTH`-style value exists, and `FULL`/`HALF` are never declared signals). Per-member, not
  per-enum (keeps BRESP's codes; empties RRESP/AXSNOOP → honest residual). **NO-GO** on section-caption (FP
  risk: `D1`/`L2` real codes), restart-of-clean (no defect), glossary/front-matter (tiny). No code → all
  oracles orthogonal; `check_doctrines.sh` GREEN. Report §`.5.iii measurement`; KM `[[generic-enum-conflation]]`.
  `[[feedback_scoring_rigor]]` / `[[feedback_avoid_denylists_prefer_structural]]`. Frontier → the `.5.iii`
  CODE slice (the `_WIDTH` gate; before/after WIRE-BASED-100 eval required, byte-changing on the AXI gold).

- `2026-06-24`: **`.5.ii` DONE — CODE: per-member sentence-spine enum-fragment gate.** Same fresh focused
  session as the `.5.ii` measurement (the two slices of `.5.ii`: calibration, then code). Added
  `PROSE_SENTENCE_SPINE_WORDS` + `is_prose_fragment_member_name` (`ir/evidence.rs`) and a `continue`-skip in
  `synthesize_encoding_declarations_for_enum`'s member loop: a synthesized member whose `_`-token set holds
  an English sentence-spine word (copula/aux/modal/article/demonstrative/relativizer/subordinator;
  collisions `a`/`i`/`its`/`can`/`may`/`am` EXCLUDED per `.1a`) is a captured prose sentence → dropped, so a
  conflated enum keeps its genuine codes (AXI `BRESP` recovers `OKAY/EXOKAY/SLVERR/DECERR/…`) and a
  pure-prose table empties (no statements → no `SymbolDefinition` → honest residual). +4 tests. **NO
  REGRESSION:** WIRE-BASED-100 = **1.000 before==after** (PROVEN — before/after `eval-extraction` over all
  10 seeds on gold evidence rebuilt with the preserved baseline vs gated binary, scored surface
  byte-identical); rebuilt AXI + APB `.isf` pass FSMGen `--strict` `success`; `kg-bench` 156/156; `run_ci.sh`
  GREEN (lib **1716**, +4). ADR-0006 (universal grammar, no name list). **`.5` enum-surface fidelity built**
  (`.5.i` name-gate + `.5.ii` member-gate); deeper member-quality classes (glossary/front-matter/
  section-caption/`_WIDTH`/restart-of-clean) are honest residuals. Report §`.5.ii LANDED`; KM
  `[[generic-enum-conflation]]` → LANDED. Frontier → KG-ISF-COMPLETENESS has no further immediately-buildable
  leaf (`.1c.ii` upstream-NLP-gated, `.2b` measured-marginal); PNT advances.

- `2026-06-24`: **`.5.ii` MEASUREMENT/CALIBRATION DONE → design CORRECTED + GO (read-only, docs-only).**
  Fresh-session PNT continuation. Read-only member-quality census over all 78 persisted IntentIR docs (561
  enums / 12 509 members). **Overturns the recorded `.5` plan:** the gate is **per-member**, not per-enum (a
  whole-enum drop destroys AXI `BRESP`'s real codes `OKAY/EXOKAY/SLVERR/DECERR/…` fused with prose), and
  **value-restart is NOT a junk signal** (AHB `HPROT` restarts but every member is a clean identifier). The
  load-bearing signal is per-member NAME shape: an English **sentence-spine** token (copula/aux/modal/
  article/demonstrative/relativizer/subordinator) marks a prose-fragment member. Collisions EXCLUDED per the
  `.1a` discipline (`A`/`I`/`ITS`/`CAN`/`MAY`/`AM`). **Precision/recall:** clean anchor 0/115 flagged
  (precision 1.000), junk anchor 269/269 caught (recall 1.000); 30.2 % of members drop. Honest residuals
  deferred (glossary `SEE…`, front-matter, section-caption `B2_3_1_…`, `_WIDTH` leaks, restart-of-clean).
  **GO** — land a per-member sentence-spine fragment drop at `synthesize_encoding_declarations_for_enum`
  (`evidence.rs`); byte-changing on wire golds (strict improvement) → before/after WIRE-BASED-100 eval
  required on the code slice. No code → all oracles orthogonal; `check_doctrines.sh` GREEN. Report §`.5.ii
  measurement`; KM `[[generic-enum-conflation]]`. `[[feedback_scoring_rigor]]` /
  `[[feedback_avoid_denylists_prefer_structural]]`. Frontier → the `.5.ii` code slice.

- `2026-06-24`: **`.5.i` DONE — CODE: extraction-side fallback name-gate + emitter orphan-`(type)` fix.**
  Fresh focused session per the high-stakes gate-code rule (it changes wire-gold `.isf` bytes). Two edits:
  (1) `derive_encoding_enum_name`'s caption-token fallback (`ir/evidence.rs`) keeps the candidate ONLY when
  it is independently evidenced — a declared signal OR a column-header reference token of the table — else
  returns `None` (existing `continue` contract → honest residual), killing the generic enums AND the
  merge-by-name conflation; (2) the emitter (`ir/isf_ir.rs`) gates the `(types)` block by `emitted_enums()`
  so a member-dropped enum leaves no orphan `(type …)`. **Two corrections to the `.5` measurement, both
  making the result cleaner:** the genuinely-named enums come from the signal-match loop ABOVE the fallback
  (byte-identical), so the structural gate is *strictly better* than a name-only gate (it also drops
  fallback-origin "real-named-but-junk" like `COMMAND`/`AMBA`/`READ`/`CACHE`/`RELEASE`); and the gate also
  cleans off-gold junk constraints whose value was a dropped-enum member (AXI `ACTIVATEACK A` → grounded
  `ACTIVATEACK 1`), score-orthogonal (distinct constraint facts identical). **Measured:** corpus census (33
  rebuildable docs) generic-named enums **82→8** / total enum records **422→105**; per-doc before→after on
  HBM2 + 4 wire golds + AXI-Stream + SWP + 6 worst offenders (amd 34→5, gic_600 23→6, coresight 19→5,
  cortex_a76 16→7, CHI 14→4, CHI-C2C 7→1) — real signal enums byte-identical, all FSMGen `--strict` 0
  diagnostics. The 8 survivors are document-evidenced (column-header/declared-signal) honest `.5.ii`
  residuals. **NO REGRESSION:** WIRE-BASED-100 = 1.000 before==after (PROVEN by before/after eval on
  rebuilt gold evidence); nvme-registers + i2c golds identical; `kg-bench` 156/156; `run_ci.sh` GREEN (lib
  1712, +4 tests). ADR-0006 (structural, no name list — `DATA` kept where a real signal, dropped where a
  caption word). Report §`.5.i LANDED`; KM `[[generic-enum-conflation]]` → LANDED. Frontier → `.5.ii`.

- `2026-06-24`: **`.5` OPENED — generic-`TABLE` mega-enum conflation MEASURED (read-only, docs-only).**
  Fresh-session PNT pivot off `CORPUS-COVERAGE.2` (the re-ingest tail is now empirically low-value — #32 was a
  legal-exhibit excerpt — so per `[[feedback_not_complete_attack_substantive_gaps]]` the high-value move is
  acting on a surfaced upstream extraction-precision lever). A delegated read-only agent + my own verification
  localized the defect to EXTRACTION (`evidence.rs` enum-name fallback + `semantic.rs` merge-by-name, NOT the
  emitter), measured it corpus-wide (56/78 docs / 96 generic + 271 real-named-but-junk enums; HBM2 `TABLE` = 7
  fused tables), and established the decisive insight that a name-only gate is insufficient (the load-bearing
  signal is member quality). Decision: GO, extraction-side, decomposed into `.5.i` (safe name-gate + emitter
  orphan-`(type)` fix — the next code slice; deferred to a focused/fresh session because it changes wire-gold
  `.isf` bytes) and `.5.ii` (member-quality gate, calibration-gated). WIRE-BASED-100 scores orthogonal; `.isf`
  bytes change (strict improvement). No code → all oracles orthogonal; `check_doctrines.sh` GREEN; knowledge
  map 125→126. Report `docs/research/generic-enum-conflation-measurement.md`; KM `[[generic-enum-conflation]]`.

- `2026-06-24`: **`.3` CLOSED — bar #2 relation-completeness resolved (verification-only, no code change).**
  Fresh-session PNT pick (first eligible leaf of the first active tree). Re-census over all 78 persisted
  `intent_ir.json` vs `evidence_ir.json` → **0 stale docs** corpus-wide (the `CORPUS-COVERAGE.2` re-ingest
  sweep rebuilt the once-stale docs through `converge`, cascading the whole chain; recovered relations now
  land canonically: tilelink 33/34, I2C 17, gic_600 101, mmu_700 25, ATS 9, DTI 1, opencapi-TL 15, USB4 13 —
  each `intent`==`evidence`). Sub-step (ii), the generic stage-staleness `validate` detector, was already
  shipped + unit-tested as `CORPUS-COVERAGE.1` (`stage_staleness_relation_finding`). The 33 register/PHY/
  command docs at 0/0 are the correct (B) honest-absence class — relation-completeness is N/A there (their
  intent lives on register/message-field/transaction surfaces); wire protocols held at WIRE-BASED-100 = 1.000.
  No fabrication. Report closure appended (`docs/research/relation-completeness-measurement.md`); KM cards
  `[[relation-completeness-staleness-vs-absence]]` (closure addendum) + `[[stage-staleness-validate-detector]]`.
  All oracles orthogonal (no code change); `check_doctrines.sh` GREEN. Frontier → the tree's remaining open
  leaves are both deferred (`.1c.ii` upstream-NLP-gated, `.2b` measured-marginal), so KG-ISF-COMPLETENESS has
  no further immediately-buildable leaf; PNT advances to the next active tree.
- `2026-06-23`: **`.1c.ii` MEASURED → deferred as a bounded residual (read-only, docs-only).** Measured the
  bulk dense-prose phantom class (multi-word, single-`REL-INFERRED`, leading-noun subjects). A within-document
  structural gate is **disproven unsafe**: the `.1b.ii`-style connectivity fold mishandles the real cases on
  AXI+ACE `ihi0022_h_c` (`caching Manager`→`Manager` correct, but `Manager component`→`component` WRONG — the
  agent is the modifier, not the head; the agent token's position varies). Within one doc a real descriptive
  reference (`caching Manager`) and a phantom (`basic bus`) are structurally indistinguishable (same single
  relation, same provenance markers, same shape), and eMMC carries no `ProtocolActorRecord` grounding surface.
  A drop is forbidden by the genericity guardrail; a participation threshold by completeness. **The genuine
  fix is upstream relation-subject extraction precision on descriptive prose** (`[[project_nlp_shallow_parse_direction]]`),
  not a downstream rule → recorded as an honest residual (the phantoms never reach `.isf`). `.1c` umbrella
  outcome: `.1c.i` shipped, `.1c.ii` upstream-NLP-gated. Report §8; no code change → all oracles orthogonal;
  `check_doctrines.sh` GREEN.
- `2026-06-23`: **`.1c.i` DONE — CODE: dense-prose trailing preposition/auxiliary strip.** Same-session
  continuation of the `.1c` probe (fresh session, measurement in context). Extended
  `consolidate_trailing_fragment` (`ir/evidence.rs`) with a new `NON_ACTOR_TRAILING_FUNCTION_WORDS` const
  (prepositions + auxiliaries/modals, subset of the leading function-word lexicon, conjunctions excluded —
  drift-guarded), so a relation subject like `host has`/`host to`/`cache in` consolidates onto its leading
  agent (`host`/`cache`) the same way `.1b.i` folds `Subordinate extends`→`Subordinate`. **Verified:** eMMC
  live cascade actors 153→138 (the 4 aux/prep `host` variants merge onto `host`; 29 phantom names removed),
  `host.isf` strict-clean; **WIRE-BASED-100 = 1.000** (fresh-Pattern new-binary eval — constraints
  APB/AHB/AXI 6/6·6/6·3/3, relations APB/AHB/AXI/SWD 5/5·6/6·6/6·1/1, temporal 3/3·4/4·3/3); `kg-bench`
  156/156; `run_ci.sh` GREEN (lib 1704, +2 tests). ADR-0006 (closed-class grammar, corpus-safe: 0 ≥8-port
  actors are `X<aux/prep>` across all 78 docs). KM `[[agent-trailing-function-word-consolidation]]`;
  README/book/live-docs synced. Frontier → `.1c.ii` (single-relation noun-phrase precision, deferred).
- `2026-06-23`: **`.1c` OWNED + PROBE DONE — agent-identity precision for the DENSE-PROSE doc class (Lever E).**
  Fresh-session PNT pivot off `CORPUS-COVERAGE.2` (owner "attack substantive gaps, not easy incremental"): two
  consecutive re-ingests (#27 eMMC, #28 HBM2) each surfaced a NEW substantive lever, so the high-value move is
  ACTING on a surfaced lever, not grinding more re-ingests. Read-only measurement over the 78 persisted IntentIR
  docs (no code change). **Findings:** (1) the eMMC explosion (153 actors/349 rel vs stale 20/23) is a
  **relation-subject extraction-precision** problem — 148/153 actors are CONNECTED phantoms minted from a single
  `REL-INFERRED` relation whose leading token is a noun (so `.1a` passes, `.1b.iv` can't touch); only 5 are
  pure-unconnected (the grounded-keep role terms `.1b.iv` correctly preserves; 0 droppable). (2) A name-SHAPE-only
  drop is DISPROVEN unsafe — AMBA's real agents share the same shape classes as eMMC's phantoms → a shape drop
  would fail WIRE-BASED-100; the fix must be grammatical normalization or participation/grounding, never shape.
  (3) The doc class is DENSE-PROSE (AXI+ACE `ihi0022_h_c` 189 + CHI `ihi0050_g` 87 explode too; terse AXI gold
  `ihi0022_l` 21 clean). **Scoped:** `.1c.i` (extend `.1b.i` trailing-strip to prepositions+auxiliaries —
  corpus-wide safety MEASURED CLEAN, 0 ≥8-port actors are `X<aux/prep>` shaped across all 78 docs, wire golds
  untouched, reach 138 names/~17 docs) = the landable first gate; `.1c.ii` (single-relation noun-phrase phantom
  precision, the bulk ≈120) deferred-with-trigger pending its own measurement. Report
  `docs/research/agent-identity-prose-class-measurement.md`; KM `[[agent-identity-prose-class-measurement]]`; book
  honesty caveat added to `pipeline/evidenceir.md`. `check_doctrines.sh` GREEN; `kg-bench`/WIRE-BASED-100
  orthogonal (no extraction change). Frontier → `.1c.i`.
- `2026-06-18`: **`.2a.ii` DONE — CODE (OWNER-AUTHORIZED): initiator-perspective signal DIRECTION emission.** The
  owner's `2026-06-17` steer (AskUserQuestion: "Build it, initiator perspective") satisfied the `.2a` re-open
  trigger. The emitted `.isf` interface now lowers grounded actor-relative direction from the protocol's INITIATOR
  actor's perspective instead of defaulting non-`Input` signals to `(output)`. `select_initiator_actor` (`ir/isf_ir.rs`)
  picks the net-producer actor (out > in) maximizing `(out, in)` — structural, no name list (ADR 0006); validated
  per-item: AHB `Manager` (6/2), APB `Requester` (20/12), AXI `Manager` (116/52), SWD `debugger` (2/1).
  `initiator_perspective_directions` maps the initiator's `Drives`→`(output)`/`Reads`→`(input)` (ambiguous/InOut/
  Unknown omitted → residual); the signal loop prefers it, else flat hint, else `(output)`; `derive_isf_actor_name`
  (`ir/adapters.rs`) names the module after the same initiator (coherent). Strict-safe because the per-output
  named-drive block is already Output-filtered → inputs auto-suppressed from drives. **Measured flip (baseline-vs-after
  via `git stash`):** APB input 2→12, AXI 4→52, SWD 0→1 (faithful protocol direction); AHB 0→0 (honest residual —
  stale Manager grounding limited to sidebands + clock/reset). **Gates:** FSMGen `--strict` 0 NEW diagnostics on all
  4 wire docs (AHB `HAUSER` + AXI `ASKSTOP` pre-existing rule conflicts unchanged; APB/SWD PASS); `run_ci.sh` GREEN
  (lib 1677→1679, +2 tests); `kg-bench` 156/156; WIRE-BASED-100 orthogonal (emitter-only); ADR-0006. KM card
  `isf-initiator-perspective-direction`; book `pipeline/isf-adapter.md`. `.2a` fully resolved (width `.2a.i` +
  direction `.2a.ii`). `[[project_kg_isf_completeness]]`.
- `2026-06-17`: **`.4` measurement DONE** (read-only, docs-only) — bar #5/#6 re-assessment on the broader
  78-doc corpus (after `CORPUS-COVERAGE.0` doubled it 36→78). Faithfully replicated the three
  `from_intent_ir` `(rule)` filters: 160 undeclared-named-subject drops are field content + prose/VLM noise,
  not wire intent (0 on all 4 wire docs); the adapter's silent skip is correct; bar #5/#6 HOLDS (confirms +
  extends `.2`/`.2b`). Spun out the DTI message-field-recognition gap (most actionable next), signal-
  inventory prose noise, and ATP VLM-fragment quality. Report
  `docs/research/behavior-temporal-lowering-completeness-78doc.md`; KM
  `behavior-temporal-lowering-broader-corpus`.

- `2026-06-17`: **`.3` measurement DONE** (read-only; owner-directed substantive push after the owner
  pushed back on the "buildable frontier exhausted" framing). Bar #2 relation-completeness: the 0-relation
  docs are NOT a relation-extraction gap — (A) STALE intent_ir recoverable by a deterministic
  `semantic`→`intent` rebuild (PROVEN: `tilelink_1_7_1` relations 0→39, 40/40 actors connected; also
  tilelink_1_8_0/i2c/wbspec + a broader stale set), and (B) HONEST ABSENCE on register/command/coherency
  docs (nvme/iommu/vt-d/ccix declare ~0 wire signals; intent lives in register/message-field/transaction
  surfaces; forcing relations = fabrication). Frontier → corpus refresh (#2) + a generic stage-staleness
  `validate` detector. Report `docs/research/relation-completeness-measurement.md`; KM
  `relation-completeness-staleness-vs-absence`.
- `2026-06-17`: **`.1b.ii` DONE** — named-interface consolidation LANDED, completing the `.1b` umbrella
  (`.1b.i`/`.1b.ii`/`.1b.iii`/`.1b.iv` all done → with `.1a`, agent-surface fidelity `.1` is fully built).
  New `strip_interface_suffix` + `consolidate_interface_actor_relations` post-pass in
  `actor_signal_relation_surface` (`ir/evidence.rs`), after the `.1b.iii` split and before dedup, rewrites an
  `"X interface"` relation subject to the bare `"X"` ONLY when `X` is independently a connected relation
  subject in this doc — so `Subordinate interface`→`Subordinate` / `Transmitter interface`→`Transmitter`
  fold onto the real agent while the GIC `CPU interface` named block (whose `CPU` is never an agent on its
  own) is preserved. Per-doc connected-agent gate, no name list (ADR 0006) — the same token can be SAFE in
  one doc and a RISK in another (CoreSight `AXI interface`). Census: 6 docs carry an `"* interface"` actor
  (3 safe / 5 risk); none of the 4 WIRE-BASED-100 gold docs do, so the gold relation surface is structurally
  untouched. Live (fresh post-`.1a`/`.1b` rebuild): AXI-Stream `Transmitter interface`→`Transmitter` (23
  rels), AXI+ACE `Subordinate interface`→`Subordinate` (49 rels), 4 wire docs unchanged. WIRE-BASED-100 HELD
  1.000 (constraints + relations + temporal, APB/AHB/AXI); `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1657,
  +3 tests; conflation guard locked by the `CPU interface` unit test — the reclaimed CoreSight / missing GIC
  RISK docs are not live-rebuildable). Book `pipeline/intentir.md` "How the actor surface stays faithful";
  KM card `[[agent-interface-block-consolidation]]`; report §9.
- `2026-06-17`: **`.1b.iv` DONE** — Class-C PURE-INFERRED phantom drop LANDED (measurement-first; the
  substantive remaining buildable agent-surface lever after `.2b` measured-marginal). New
  `is_pure_inferred_phantom_role` + a guard in `build_intent_actors` (`ir/intent.rs`) skips an actor whose
  `responsibilities` is exactly the single SemanticIR Phase-2 term-scan marker
  ``"semantic role inferred around `X` evidence"`` — pure generic-vocabulary noise (0 ports, 0 relations,
  no phase/contract grounding), keyed on the marker SHAPE not a name list (ADR 0006). Measured on a fresh
  post-`.1a`/`.1b` rebuild of the 4 wire docs into a temp evidence-root (WRITE-PATH GOTCHA): drops exactly
  the 21 corpus-wide phantoms / 16 docs (wire: APB `controller`, AHB `agent`) with ZERO connected or
  grounded actors touched; genuinely-discussed-but-unwired agents (AXI `transmitter`, SWD `host`, GIC
  `arbiter`) deliberately KEPT (completeness over pruning). Stage-diff: evidence + semantic byte-identical,
  intent differs only by the dropped phantoms (+ the phantom id leaving global behaviors' `actor_ids`);
  `actor_signal_relations`/`signal_constraints`/`temporal_rules` byte-identical; `.isf` byte-identical
  (emitter never reads `actors[]`). WIRE-BASED-100 HELD 1.000 (constraints + relations + temporal,
  APB/AHB/AXI, fresh eval); `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1654, +2 tests). Book
  `pipeline/intentir.md` "How the actor surface stays faithful". KM card
  `[[agent-pure-inferred-phantom-drop]]`; report §8. The Class-C taxonomy is now closed for the
  unambiguous phantom subset; the PROSE-GROUNDED 0/0 set stays honestly kept. Frontier → `.1b.ii` /
  `.2a` direction (both deferred-with-trigger) / `KG-ISF-TRANSACTIONS` body-emission (FSMGen-parked).
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
- `2026-06-16`: **`.1a` DONE** — precision structural agent-identity gate LANDED (first code in this
  tree). New `is_non_actor_phrase_fragment` in `ir/evidence.rs` rejects a relation-subject candidate
  whose FIRST content token is a universal function word (`NON_ACTOR_LEADING_FUNCTION_WORDS`) or a
  leading verb (`NON_ACTOR_LEADING_VERBS`) — ADR-0006-safe grammar, NOT a chip-name list — wired into
  the DRY seam `normalize_relation_actor_name` so all three relation paths gate at once. Measurement-first
  over all 36 IntentIR docs corrected the candidate lists twice (drop `i`/`its` for the `'I'`/GIC `ITS`
  collisions; exclude articles/determiners — `All Managers` is a `.1b` strip), landing on ZERO ≥8-port
  rejects / 9 targets caught / 63 non-agent rejects. Live AXI rebuild: actors 24→21, junk dropped, real
  agents byte-for-byte preserved (`Manager` 169 ports, `Subordinate` 168, `interconnect` 5),
  descriptor-noun class deferred per design. WIRE-BASED-100 held at 1.000 (constraints APB/AHB/AXI,
  relations ×4 docs, temporal ×3) on fresh-Pattern eval; `kg-bench` 156/156; `run_ci.sh` green (+3 tests,
  lib 1633 pass/2 ignored). KM card `agent-identity-structural-gate`. Frontier → `.1b`.
- `2026-06-16`: **`.1b` MEASUREMENT DONE + split into sub-leaves** (read-only over the persisted 36-doc
  IntentIR corpus, before any code). Confirmed the persisted corpus is the PRE-`.1a` baseline (the `.1a`
  rebuild went to a temp evidence-root, not canonical — the WRITE-PATH GOTCHA). Modelled each `.1b`
  transform corpus-wide: (i) the trailing verb/adverb strip is grammatically unambiguous and the clean
  win; the `"X interface"→"X"` half carries a named-block conflation risk (GIC `CPU interface` ≠ `CPU`)
  → `.1b.ii`; (iii) coordinated "X and Y" → `.1b.iii`; the Class-C 0/0 drop is 320 actors corpus-wide
  (only 21 PURE-INFERRED vs 223 PROSE-GROUNDED + 76 SECTION+INFERRED) and validated only on wire docs
  → `.1b.iv`, deferred until a defensible provenance re-check is designed. Report
  `docs/research/agent-surface-fidelity-measurement.md` §7. Frontier → `.1b.i`. No code.
- `2026-06-16`: **`.1b.i` DONE** — Class-B trailing-fragment consolidation LANDED. New
  `consolidate_trailing_fragment` in `ir/evidence.rs` strips a trailing `NON_ACTOR_LEADING_VERBS` token or
  a `NON_ACTOR_TRAILING_DISCOURSE_MARKERS` adverb (the latter a new const, drift-guarded ⊆ the leading
  function-word lexicon), keeping the leading agent noun, wired into the DRY seam
  `normalize_relation_actor_name` BEFORE the `.1a` reject — so `Subordinate extends`/`Subordinate then`→
  `Subordinate` and `decoder also`→`decoder` re-attribute their stranded relations onto the real agent
  (dedup merge) instead of surviving as phantom fragment actors. ADR-0006-safe (parts of speech, no name
  list); conjunctions (coordination, `.1b.iii`) and `"X interface"` (`.1b.ii`) deliberately excluded;
  byte-identical when nothing strips. Live AHB: `Subordinate` 25/23→27/26, `decoder` 0/0→4/2, actors
  25→19; AXI real agents byte-identical, actors 21. WIRE-BASED-100 HELD 1.000 (constraints ×3 / relations
  ×4 / temporal ×3) on fresh-Pattern eval; `kg-bench` 156/156; `run_ci.sh` GREEN (+2 tests, lib 1635
  pass/2 ignored). KM card `agent-trailing-fragment-consolidation`. Frontier → `.1b.ii`/`.1b.iii`/`.1b.iv`.
- `2026-06-17`: **`.2b` MEASURED-MARGINAL → deferred** (read-only, docs-only). Assessed the lowering-
  coverage gauge's value before building: `.2` showed the lowering is already faithful for grounded intent,
  so a coverage gauge is noise-dominated (a "257/16 463 temporal_invariants lowered" headline is dominated
  by 16 179 ungroundable ToC-heading invariants — honest absences, not a lowering gap; the actionable
  undeclared-but-grounded signal is ≈0 on wire docs / ~151 corpus register-field paths). Not worth a code
  slice. **Frontier → `.1b.iv`** (Class-C zero-evidence actor drop), now the substantive remaining
  buildable lever: a read-only count put zero-evidence (0 ports AND 0 relations) actors at **34% of all
  actors corpus-wide** (331/969; AXI 12/24, AHB 7/25, SWD 8/23, APB 3/10 on the pre-`.1a` persisted corpus
  — `agent`/`channel`/`consumer`/`producer`/`controller`/`device`/…). `.1b.iv` stays measurement-first +
  gate-risky (needs the PURE-INFERRED-vs-PROSE-GROUNDED provenance discriminator designed on freshly-rebuilt
  post-`.1a`/`.1b` evidence; WIRE-BASED-100 + the owner's completeness intent are hard gates) — a careful
  slice best done with fresh-session sharpness.
- `2026-06-17`: **`.2a.i` DONE** — signal width fidelity LANDED. The `.2a` FSMGen-contract trigger ran
  first (empirical binary probe + book contract, KM `fsmgen-ignores-signal-direction`): **direction is
  FSMGen-neutral** (flipping a driven output→input still passes `--strict`) → direction stays deferred
  (owner-philosophy only); **width** must resolve to a positive integer → emit the concrete grounded width.
  `actor_port_concrete_widths` (`ir/isf_ir.rs`) recovers each signal's single unambiguous concrete graph
  width (`actor_ports[].width_hint`) and the signal lowering prefers it over the width-1 default; never
  overrides a real >1 hint, never guesses on conflict (0 conflicts measured); ADR-0006, no name list; +1
  test. Live: AXI 32 + AHB 2 signals gain real widths (`ARSIZE→3`/`HSIZE→3`/…), non-signal lines 0,
  APB/SWD byte-identical, **0 new FSMGen `--strict` diagnostics** (AXI/AHB keep byte-identical pre-existing
  rule diagnostics), `run_ci.sh` green (lib 1652). WIRE-BASED-100 structurally unaffected (emitter-only).
  Report §6; KM cards `fsmgen-ignores-signal-direction` + `isf-lowering-fidelity-gauge`; book
  `pipeline/isf-adapter.md`.
- `2026-06-17`: **`.2` MEASUREMENT DONE** (read-only, docs-only — the `.2+` umbrella resolved into a
  concrete measurement leaf + two spun sub-leaves). Gauged which ISF-fidelity bar dimension has the largest
  faithful-lowering gap by reading the full ISF lowering and replicating its per-element filters over all 36
  IntentIR docs. **Verdict:** bar #6 (round-trip) is already honest for grounded wire intent — the 22k
  `behaviors`/`constraints` are free-text twins already lowered typed; the 17 425 typed-rule "drops" are
  un-groundable free-text (16 179 empty-subject temporal_invariants = ToC headings) / no-consequent prose /
  non-wire register-field paths (0 undeclared drops on all 4 wire docs), so mass-residualizing would be
  dishonest noise. The largest TRUE infidelity is **signal direction/width** (~98% defaulted output/width-1,
  ignoring the actor-relative graph) — but it is design-gated (relationship-relative + single flat module +
  the deliberate `R6-ISF-ADAPTER.4` default + an owner decision). Spun `.2a` (direction/width fidelity,
  deferred-with-trigger + owner steer) + `.2b` (lowering-coverage visibility gauge, buildable, additive).
  Report `docs/research/isf-lowering-fidelity-measurement.md`; KM card `isf-lowering-fidelity-gauge`. No
  code; WIRE-BASED-100 untouched.
- `2026-06-16`: **`.1b.iii` DONE** — coordinated-subject split LANDED. `split_coordinated_actor_subject` +
  `split_coordinated_actor_relations` (+ `relation_actor_id_slug`) in `ir/evidence.rs`, a post-pass in
  `actor_signal_relation_surface` before dedup: a relation whose subject is a coordinated "X and Y"
  ("*the Subordinate and decoder read HADDR*") is replaced by one relation per conjunct, so BOTH genuine
  agents connect to the signal they both act on. Splits on `and` ONLY (conjunction = both); `or` excluded
  (disjunction = ambiguous → fabrication); each conjunct re-validated through the full agent gate; fires
  only when ≥2 survive. Measured: 2 coordinated subjects corpus-wide (both AHB), both conjuncts real. Live
  AHB: `decoder` 4/2→8/6, `Subordinate` 27/26→32/31, `Exclusive Access Monitor` 4/2→5/3, fragments gone,
  actors 19→17. WIRE-BASED-100 safe — the AHB relation gold is 6 `drives` facts on unrelated statements,
  and the scorer doesn't penalize off-gold relations (AXI 343 rels / fp=0). WIRE-BASED-100 HELD 1.000;
  `kg-bench` 156/156; `run_ci.sh` GREEN (+2 tests, lib 1637 pass/2 ignored). KM card
  `agent-coordinated-subject-split`. Frontier → `.1b.ii`/`.1b.iv` (deferred-with-trigger) / `.2+`.
