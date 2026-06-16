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
  - **`.1b.ii`** (`"X interface"→"X"` strip) — DEFERRED: a named-interface block (`CPU interface` in GIC,
    a distinct GICC architectural entity) is NOT the generic noun `CPU`; needs an "only when the leading
    token is already a real connected agent in this doc" sub-gate (actor-set context the relation-subject
    seam lacks).
  - **`.1b.iii`** (coordinated "X and Y" split) — AHB `Subordinate and decoder` 6/4,
    `Exclusive Access Monitor and Subordinate` 4/2; needs the relations duplicated to BOTH agents.
  - **`.1b.iv`** (Class-C zero-evidence drop) — DEFERRED beyond the wire docs: 320 Class-C 0/0 actors
    corpus-wide, but only **21 are PURE-INFERRED** (the unambiguous Phase-2 role-term phantom); **223 are
    PROSE-GROUNDED** + **76 SECTION+INFERRED**, and "PROSE-GROUNDED" is NOT a clean "genuinely-declared
    agent" discriminator. The `.1`/`.1a` measurement validated the drop only on the 4 wire docs; a blanket
    corpus-wide 0/0 drop would delete agents the owner's completeness north star wants kept. So `.1b.iv`
    must first design a defensible provenance re-check (likely: drop only PURE-INFERRED phantoms, or scope
    to the wire docs) — its own measurement-first leaf, NOT forced here.
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
