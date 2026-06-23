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

- ID: `KG-ISF-COMPLETENESS` · Status: `active` · Children: `.0` (scope/ownership), `.1` (agent-surface on the AMBA/structured class, done; `.1c` reopens it for the dense-prose class), `.2` (ISF lowering-fidelity; `.2a.i` width done, `.2a.ii` direction done — initiator-perspective, owner-authorized, `.2a.iii` module-name HDL-sanitization done — owner-chosen), `.3` (relation-completeness — bar #2), `.4` (behavior/temporal lowering-completeness — bar #5/#6, broader corpus)
- ID: `KG-ISF-COMPLETENESS.1c` · Status: `active` (umbrella; measurement-first PROBE DONE `2026-06-23`,
  read-only, docs-only — splits into sub-leaves) · Goal: **agent-identity precision for the DENSE-PROSE doc
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
    class). **Frontier → `.1c.i`** (trailing aux/prep strip — measurement-clean, landable; best on a fresh
    session for full sharpness per the high-stakes gate-code rule), then `.1c.ii` measurement.
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
- ID: `KG-ISF-COMPLETENESS.3` · Status: `active` (measurement DONE `2026-06-17`, read-only; owner-directed
  substantive north-star push after the owner pushed back on "buildable frontier exhausted") · Goal:
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

## Changelog

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
