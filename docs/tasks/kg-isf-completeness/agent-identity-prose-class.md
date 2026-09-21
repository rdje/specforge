# KG-ISF-COMPLETENESS — agent identity prose class

- Part ID: `agent-identity-prose-class`
- State: `legacy`

<!-- kg-isf-completeness-task-source-region:agent-identity-prose-class:start -->
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

<!-- kg-isf-completeness-task-source-region:agent-identity-prose-class:end -->
