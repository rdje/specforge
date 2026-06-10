# EXTRACTION-QUALITY-GAUGE: measure the extraction-quality gap — and CHI's is large

## Metadata

- Tree ID: `EXTRACTION-QUALITY-GAUGE`
- Status: `active` (gauge established + CHI measured; the fix leaves are open R-lane work)
- Roadmap lane: `R15e`/`R16` (extraction quality / production-readiness)
- Created: `2026-06-06`
- Parent context: `TABLE-GRITS-CONFORMAL.4` ran the NLI-oracle conformal on CHI and it would not
  calibrate. The diagnosis turned out to be a real extraction-quality gap, not a metric problem.

## The gauge (the reusable capability — "something to catch the size of the gap")

The NLI-oracle pass (`nli-verify` over an EvidenceIR) yields, per document, the **fraction of
extracted constraints the source does NOT entail** — a cheap, automatic **extraction-quality gauge**
(a production-readiness signal). It is a *noisy* oracle (some not-entailed are NLI false-negatives on
complex claims), so treat it as an estimate — but it is **hand-validated** (below) and discriminates
sharply across documents:

| doc | constraints | NLI not-entailed | reading |
|---|---|---|---|
| APB (simple peripheral bus) | 14 | ~29% | tolerable |
| **CHI** (cache-coherency interconnect) | 162 | **~83%** | **far from production** |

## CHI measurement — ~80% of constraints are erroneous (hand-validated)

A random sample of 18 not-entailed CHI constraints was **read against source** (the rigorous check,
not the aggregate). **18/18 were genuine errors** — the NLI signal is largely real here, not noise.
Heuristic categorization of all 162 plus the hand-read taxonomy:

| error type | what it is | examples |
|---|---|---|
| **Spurious subject** (largest) | a non-signal extracted as a signal | `B13` (←"Table B13.25"), `MTE` (a feature), `LICENSEE`/`AMBA` (license/trademark boilerplate), `CMO`/`PCMO` (transaction *types*) |
| **Condition-drop** | real signal+obligation, but the conditional/temporal scope is lost | `TXSACTIVE must be asserted` ← "…*after receiving a snoop* / *until the last flit*" (and **not** recovered in `conditional_rules`: 1 of ~6 survives) |
| **Permission→obligation** | a permissive statement turned into a `must` | `EWA must_be_value` ← "permitted, but **not required**" |
| **Relational mis-extraction** | a relation/equality flattened to a value constraint | `DBID must_be_value` ← "TxnID is set to the same value as DBID" |
| **Duplication** (cross-cutting) | same subject+kind re-emitted from each conditional sentence | **30%** of the 162 are duplicates |

Mechanically-detectable floor (regex): spurious-ref **5%** + condition-drop **15%** = 20%; the
hand-read shows the true rate is ~**80%** (only ~17% pass the NLI, and even those are merely
*plausible*). **SpecForge is far from production-ready on hard, dense, conditional specs.**

## Root cause #1 — entity discrimination is broken (the foundational one)

> "SpecForge needs seriously to learn to discriminate signals vs actors vs everything else. If it
> can't find the objects/entities that interact in a chip-spec, we won't go far." — owner, and the
> data agrees emphatically.

The KG is `(actor —verb→ signal)`; it is **only as good as the typing of its nodes**, and on CHI the
nodes are mis-typed garbage:

- **Actors are garbage.** 73 "actors" extracted include `ADDRESS OF`, `ANY`, `APPLICATION`, `ARE
  MEMORY`, `ATTRIBUTE VALUES`, `BY ANY`, `CACHE STATE` — random noun phrases, not components (the real
  actors are `Requester`, `Completer`, `RN-F`, `Home Node`, …).
- **Signal typing is noisy both directions.** `B13` and `CMO` are wrongly *hinted* as signals; the
  *real* signal `TXSACTIVE` is **absent** from the declared (24) + hinted set entirely.
- **64% of the 162 constraint subjects** are in **no** declared/hinted signal set — the extractor is
  largely guessing, and the catalogs that should ground it are themselves unreliable.

So the spurious-subject errors are a symptom of a foundational gap: SpecForge cannot reliably
**classify a token's entity type** (signal | actor | transaction | feature | state | structural-ref |
boilerplate). The ingredients exist (`table_signal_declaration_provenance`, `signal_semantic_hints`,
`actor_signal_relations`) but are noisy and **not enforced** at extraction. **Fix this first — every
downstream relation, constraint, and intent inherits the node typing.**

## Root cause #2 — the constraint unit doesn't match the unit of meaning

CHI is a deeply **conditional / temporal / stateful** protocol ("when X in state S, Y must Z until
W"). SpecForge's unit of extraction — the flat, mostly-**unconditional** `SignalConstraint` —
structurally mismatch that. Adequate on APB (simple, mostly unconditional); it breaks on CHI. Prior
"intent-capture" validation leaned on simpler specs; CHI exposes the real limit.

## Fix backlog (open R-lane work)

- `.1` **Entity discrimination** (THE foundational fix): a typed classifier for every candidate token
  — `signal | actor | transaction | feature | state | structural-ref | boilerplate` — grounded in
  **derived** signals (ADR 0006), not hardcoded names:
  - **declaration site** — signals are declared in pin/port/signal tables (`table_signal_declaration_provenance`);
    actors in a components/glossary section; the *region* a token is defined in types it;
  - **linguistic role** — actors are *subjects* of normative verbs ("the Completer must drive…");
    signals are *objects* ("…drive PREADY") — reuse the actor-signal-relation grammar;
  - **structural cues** — "token after *Table*/*Figure*" → ref; front-matter/legal region → boilerplate;
  - **cross-reference** — a real signal recurs in the signal table *and* prose; a ref appears only in "see …".
  Then **enforce** it: a constraint/relation subject must be a token typed `signal` (and clean the
  noisy catalogs — `TXSACTIVE` must be *in*, `B13`/`CMO` *out*; the garbage actors must be rejected).
- `.2` **Conditional/temporal constraints as first-class**: stop flattening — carry each constraint's
  condition/temporal scope (the extractor discards it *before* the temporal layer can capture it).
  This is the big one.
- `.3` **Permission-vs-obligation gate** + **relational-vs-value disambiguation** (reuse the NLI
  condition-vs-obligation machinery).
- `.4` **Dedup** constraints by (subject, kind, condition).
- `.0` **Wire the gauge** into `converge`/CI as a per-doc quality report (the standing measurement).

## `.1` prototype — built + measured (the architecture is validated)

`ir/entity_typing.rs` + `entity-type` command embody the "human-SpecForge in Rust" structure: Rust
**gathers** each token's grounding evidence (`gather_entity_evidence`), the **LLM judges**
(`propose_entity_type_llm`), Rust **grounds** the judgment (`classify_entity` — the document overrides
the model where authoritative, defers where silent), and the **enforcement gate**
(`is_valid_signal_subject`) keeps only `Signal`-typed subjects. Document-grounded → works on any spec.
Unit-tested (injected `propose`, no provider).

**Measured on real CHI (66 distinct subjects):** typed **27/66 as non-signal** and filtered them —
correctly: `B13/B14/B16`→structural_ref, `AMBA/LICENSEE`→boilerplate, `CMO/PCMO/DVMO`→transaction,
`MTE/MEC/DVM`→feature, and crucially `TXSACTIVE` (the real signal the *catalogs missed*) **recovered**
as signal by the LLM. So **we *can* harness the LLM for discrimination — the architecture holds.**

**But discrimination alone is not sufficient.** Removing the 27 → drops **49/162 (30%)** constraints,
**84% of which were NLI-not-entailed** (genuine garbage). Yet the gauge moves only **17% → 18%**,
because the *kept* signal constraints are still ~82% not-entailed — they are real signals whose
**conditions were dropped** (root cause #2). The path to a working SpecForge is to apply the SAME
harness pattern to *each* error mode (`.2`–`.4`), not entity typing alone. One front proven; several
remain.

## `.2` result + the strategic pivot (`2026-06-06`)

`ir/condition_extract.rs` + `extract-conditions` (Rust gathers → LLM judges the condition clause →
Rust grounds it: a condition is kept only if ≥60% of its content words appear in the source — no
hallucinated conditions; +4 tests). **Measured on CHI:** 25 candidates, **22 conditions captured +
grounded (88%)**, and they flip correctly — `TXSACTIVE` constraints became NLI-entailed once their
condition (`"before or in the same cycle in which the first flit…"`) was restored.

**Trajectory (CHI NLI-entailed):** `17%` (neither) → `22%` (`.2`) → `.1`-only `18%` → **`.1`+`.2` `22%`**.
Each front works; each moves the gauge a few points; they're complementary (`.2` fixes condition-drop,
`.1` removes garbage nodes — `B13` got a condition but stays not-entailed because it's a spurious
*subject*).

**Strategic read:** patching the Pattern extractor error-mode-by-error-mode converges slowly — the
base is ~80% wrong. The proven components (`.1` typed-grounded subjects, `.2` grounded conditions) are
the pieces of a better answer: **compose them into an LLM-PRIMARY, Rust-grounded constraint
*extractor*** that emits `(typed-signal subject, obligation, grounded condition)` in one pass, rather
than bolting fixes onto a flat pattern extractor. That is the real test of the harness thesis and the
recommended next tree (`EXTRACTION-QUALITY-GAUGE.5`, or its own tree) — replace, don't patch. `.3`/`.4`
(permission/relational, dedup) remain useful but secondary to the replacement.

## `.5` result — the thesis is VALIDATED (`2026-06-06`)

`ir/constraint_extract_llm.rs` + `extract-constraints-llm`: per sentence the LLM proposes structured
`(subject, kind, condition)` constraints; Rust grounds each (subject types as `Signal`; condition must
appear in source). Composes `.1`+`.2` into a real extractor that REPLACES the Pattern set. +4 tests.

**Measured on CHI (134 sentences, ~2 min):**

| extractor | constraints | NLI-entailed |
|---|---|---|
| Pattern (baseline) | 162 | 28 = **17%** |
| `.1`+`.2` patched | 113 | ~22% |
| **LLM-primary (`.5`)** | **44** | 28 = **64%** |

**PRECISION jumps ~4× (17%→64%)** — that part is robust. **But recall is traded, not free** (recall
proxy, by subject+kind): the LLM-primary's 28 entailed are a *different* set from the Pattern's — it
covers only **9/22 (41%)** of the Pattern's known-good constraints while finding **18 net-new**. The
divergence is concentrated in `must_be_value` (the error-prone relational class — `DBID`, `TXNID`,
`RETURNNID`… — the LLM mostly dropped; *some correctly* as relational mis-extractions the NLI happened
to pass, *some* maybe genuine misses). So **replace buys large precision at some recall cost** — it
does *not* strictly dominate the Pattern set.

**Honest verdict:** the thesis "*replace can beat patch on quality*" is **supported on precision**
(4×, robust across the same oracle), but "*replace IS net-better*" is **not settled** — that needs a
small CHI **gold** to measure recall/F1. The earlier "kept the same 28" framing was wrong (same
*count*, different *set*); corrected here. The remaining 16 not-entailed + the recall gap are the next
work.

**Strategic conclusion → the path forward:** the harness pattern works and replace shows a large
precision gain, so the program is to *replace* each extraction stage (relations, temporal rules,
registers) with an LLM-primary, Rust-grounded extractor measured by the gauge — **but gate it on `.6`
first** (a CHI gold to confirm recall isn't being sacrificed). Precision-without-recall would be a trap
the gauge alone can't see. Settle that, then scale the replace program. That is the concrete (and
honestly-qualified) path to "human-SpecForge in Rust."

## Task Tree

- ID: `EXTRACTION-QUALITY-GAUGE` · Status: `active` · Children: `.0`–`.4`
- ID: `EXTRACTION-QUALITY-GAUGE.gauge` · Status: `done` · Goal: establish the NLI-oracle not-entailed
  rate as a per-doc extraction-quality gauge; measure CHI (~83%) + APB (~29%), hand-validate (18/18).
- ID: `EXTRACTION-QUALITY-GAUGE.1` · Status: `done` (prototype) · Goal: **entity discrimination** —
  derived typed classifier + LLM judgment + Rust grounding + enforcement. Built (`ir/entity_typing.rs`
  + `entity-type` cmd, tested); measured on CHI: 27/66 subjects filtered correctly (TXSACTIVE
  recovered), drops 30% of constraints (84% were wrong), gauge 17%→18%. Architecture validated;
  follow-up = wire the gate into the real extractor path + improve fine sub-typing.
- ID: `EXTRACTION-QUALITY-GAUGE.2` · Status: `done` (prototype) · Goal: conditional/temporal
  constraints first-class. `ir/condition_extract.rs` + `extract-conditions` cmd (LLM-judged,
  source-grounded), tested; CHI 22/25 captured, gauge 17%→22%. Complementary to `.1`.
- ID: `EXTRACTION-QUALITY-GAUGE.5` · Status: `done` · Goal: **LLM-primary grounded constraint
  EXTRACTOR** composing `.1`+`.2`. Built (`ir/constraint_extract_llm.rs` + `extract-constraints-llm`,
  tested) + measured: CHI 162→44 constraints, **precision 17%→64% (~4×)** but recall TRADED (covers
  41% of Pattern-good + 18 net-new). Thesis supported on precision; net-better needs a CHI gold.
- ID: `EXTRACTION-QUALITY-GAUGE.6` · Status: `done` (CORRECTED) · Goal: settle replace-vs-patch
  recall. **Domain correction (owner + CHI B2.4):** CHI is a packet/flit protocol, NOT a wire bus —
  `DBID`/`TxnID`/`ReturnNID` are *transaction-identifier FIELDS* (contents of flits), **not signals**.
  So the LLM-primary extractor was **correct** to exclude them from *signal* constraints; my earlier
  "genuine miss" verdict was wrong (the Pattern extractor had mis-typed fields *as* signals). The CHI
  "recall loss" was largely **correct field-exclusion**, plus a smaller real gap.
- ID: `EXTRACTION-QUALITY-GAUGE.7` · Status: `done` (clean test) · Goal: settle recall on a clean
  WIRE-BASED spec (no signal/field confound). Ran `.5` on **APB** vs its gold: **recall 4/6 = 67%** —
  the 2 misses (`PBUSER`/`PNSE` `must_be_value VALID`) **are** signals. So there IS a real, **localized
  `must_be_value` recall gap** even on clean signals — separable from CHI's field issue. Two distinct
  problems, both characterized. Next = `.8`.
- ID: `EXTRACTION-QUALITY-GAUGE.8` · Status: `done` (`2026-06-10`) · Goal: close the `must_be_value`
  recall gap in the LLM-primary extractor; re-measure on APB + AXI/AHB. **Root cause (probed live
  BEFORE coding, qwen2.5:14b-instruct temp 0 on the exact persisted sentences):** (1) the extraction
  prompt's kind vocabulary cannot express a *validity* requirement — for both gold sentences
  ("PNSE/PBUSER must be valid when …") the model outputs `[]`, so the whole requirement vanishes
  before grounding ever runs; (2) two silent-drop paths compound it: `parse_kind` rejects the model's
  natural `must_be_valid` spelling, and rejects `must_be_value` with no echoed value (`value?`). The
  gold convention (= the Pattern extractor's own output) is `must_be_value` + `VALID`. **Fix
  (generic, no chip names, no lists):** (a) the prompt states the typed convention ("a validity
  requirement — <signal> must be valid — is kind must_be_value with value VALID"; probed pre-code:
  both misses recover, asserted-control + negative-control unchanged); (b) Rust backstop —
  `parse_kind` accepts the `must_be_valid`/`valid` spellings, and `ground_constraint` recovers a
  value the model named but did not echo from the SOURCE sentence by reusing
  `extract_protocol_state_value` (the Pattern extractor's own binder grammar, now `pub(crate)` —
  grounded, never fabricated; unrecoverable → honest drop); (c) +4 pure tests (injected, no
  provider; lib 1503). **Measured (pre-fix vs post-fix, same redirected-copy protocol, eval
  canonical keys, document-level gold-fact recall):** APB **4/6 → 6/6** (pre-fix independently
  reproduced the `.7` number; both misses = `PNSE`/`PBUSER` `must_be_value VALID`), AHB **2/6 →
  6/6** (all four pre-fix misses were `HAUSER`/`HWUSER`/`HRUSER`/`HBUSER` `must_be_value VALID`;
  labeled-statement FPs *dropped* 2→1 — the pre-fix condition-junk `HREADY must_be_high` +
  `HRESP must_not_change` read out of the HRUSER sentence is gone), AXI (zero `must_be_value` gold —
  the no-regression control) **4/4 → 4/4** doc-level and **3/4 → 4/4** strict (the prompt change
  un-suppressed the conditional `ASKSTOP must be LOW when ACTIVATEACK is LOW` statement). **Total:
  10/16 → 16/16 gold constraint facts; every one of the six pre-fix misses was a `must_be_value
  VALID` fact and every one is recovered.** Honest FP ledger (all in the open `.3`
  condition/permission class, net 3→3): APB `PSELx must_be_high` unchanged pre/post; AHB 2→1; AXI
  0→1 (`ACTIVATEACK must_be_value LOW` — the model reads the *when*-clause subject as a second
  obligation on the newly-extracted statement). LLM-primary volumes: APB 12→21, AHB 11→16, AXI
  54→59 (vs Pattern 14/15/102).
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD` · Status: `pending` · Goal: the SIGNAL-vs-FIELD ontology for
  packet/flit protocols (CHI/CXL/PCIe-class) — model fields (flit contents) distinctly from signals
  (physical wires), per the doc's own B16-Signals vs B2.x-Fields split. The real future target for
  CHI-like PDFs (owner: "in fine we need to handle such cases too").
- ID: `EXTRACTION-QUALITY-GAUGE.3` · Status: `active` (split `2026-06-10`) · Goal: permission/relational
  disambiguation. Split after `.8` measured the residual FP ledger: ALL three labeled-statement FPs are
  the **condition-subject-read-as-obligation** class — APB `PSELx must_be_high` (if-clause), AHB
  `HRESP must_be_value ERROR` (unless-clause), AXI `ACTIVATEACK must_be_value LOW` (when-clause).
- ID: `EXTRACTION-QUALITY-GAUGE.3a` · Status: `done` (`2026-06-10`) · Goal: a deterministic
  **condition-only-subject gate** in the LLM-primary grounding path: a proposed constraint whose
  subject appears ONLY inside subordinate conditional clauses of the source sentence
  (when/if/unless/while/until/after/before/whenever/provided-that/as-long-as — universal grammar, no
  name lists) is the condition's subject, not an obligation's, and is dropped
  (`is_condition_only_subject` + `conditional_clause_spans` + `token_occurrences` in
  `ir/constraint_extract_llm.rs`, called from `ground_constraint` after signal typing). **The
  per-item audit caught two real defects in the first cut before they could ship** — (1) clause
  spans ran past sentence-final punctuation and swallowed the NEXT sentence's main clause; (2)
  "while **driving** HREADYOUT LOW" is action *coordination* (the obligation IS on HREADYOUT), not a
  condition — fixed by terminating spans at `.!?` too and by skipping marker+gerund clauses; the
  defective first cut wrongly dropped AHB's two `HREADYOUT` records, the corrected gate restores
  them while still killing all three target FPs (both behaviors fixture-locked in tests). +8 pure
  tests total (lib 1511). **Measured (the `.8` redirected-copy protocol; post-`.8` numbers = the
  baseline): APB P 0.857→1.000, AHB P 0.857→1.000, AXI P 0.800→1.000, recall HELD at 16/16 —
  every doc now scores P=R=F1=1.000 on the labeled constraint task, and the split-conformal
  tier-agreement threshold now calibrates on all three (empirical_error 0.000).** The three killed
  FPs, per item: APB `PSELx must_be_high` (if-clause), AHB `HRESP must_be_value ERROR`
  (unless-clause), AXI `ACTIVATEACK must_be_value LOW` (when-clause).
- ID: `EXTRACTION-QUALITY-GAUGE.3b` · Status: `done` (`2026-06-10`) · Goal: permission-vs-obligation
  gate. **Probed (the AHB sentences read against source):** `HPROT[0] must_be_high` ← "It is
  **recommended** that a Manager sets HPROT[0] HIGH…" (no must/shall) and `HSEL must_be_high` +
  `HTRANS must_be_value IDLE` ← "An alternative implementation **would be** for HSEL to be tied
  HIGH…" (hypothetical) are frame errors; but `HEXOKAY must_be_deasserted` ← "It is **permitted**
  for a Manager … the Exclusive Write transfer **must** fail and HEXOKAY **must** be deasserted"
  is a REAL conditional obligation behind a permissive lead-in. **Shipped gate:
  `is_permissive_only_subject_frame` — scoped to the SENTENCES CONTAINING THE SUBJECT** (same
  split as `is_normative_for_subject`): a permissive frame word
  (recommended/permitted/permissible/optional/may/can/could/would — word-boundary, universal
  normative vocabulary) in a subject-sentence with NO mandatory frame (must/shall) in any
  subject-sentence → drop; mandatory present → keep outright. **The per-item audit killed the
  first cut again**: a BLOCK-scoped check over-killed AHB 15→8 — the incidental "Although an OKAY
  response **can** be given in a single cycle" softened the ERROR-procedure sentences and wrongly
  dropped `HRESP`×2 + `HREADYOUT`×2; sentence-scoping restores them (15→12 = exactly the three
  frame errors; over-kill shape test-locked). **Measured (refined gate, the `.8`/`.3a` protocol):
  AHB 15→12 with precisely `HPROT[0]`/`HSEL`/`HTRANS-IDLE` removed and `HRESP`/`HREADYOUT`/
  `HEXOKAY`/`H*USER` all kept; APB 20 and AXI 54 unchanged (controls); all three docs stay
  P=R=F1=1.000 against the EXTENDED gold.** Honest scoring note: the eval-time WIRE-BASED-100
  filter (`is_normative_for_subject`) already masked this FP class from the labeled gauge — the
  `.3b` win is at the CANONICAL ARTIFACT level (the EvidenceIR downstream consumers read is now
  clean of frame errors), and the two new AHB gold NEGATIVE items (statements `0561`/`0678`,
  `agent_drafted`) are regression armor if the eval-time filter ever weakens. +6 pure tests (lib
  1517). Relational-vs-value ("set to the same value as") and descriptive-narration frames stay a
  later sub-slice (`.3c`, pending).
- ID: `EXTRACTION-QUALITY-GAUGE.4` · Status: `done` (`2026-06-10`) · Goal: constraint dedup by
  (subject, kind, condition) in the LLM-primary extractor. Shipped: pure `dedup_constraints` —
  canonical key = the eval's `signal_constraint_record_key` (subject + kind incl. value + negation)
  plus the normalized condition; first record kept (stable ids/order), duplicates'
  `supporting_statement_ids` merged in so provenance is preserved, never lost; lookup-only map (no
  hash-iteration order reaches output — `EVIDENCE-DETERMINISM`); called once in
  `extract-constraints-llm` after grounding, which now prints `grounded → deduped (N merged)`.
  +2 pure tests (lib 1519). **Measured live (3 docs): AXI 54→50 — `AWIDUNQ must_be_asserted (if
  present)` re-extracted from 3 statements collapsed to ONE record carrying all 3 statement ids,
  same for `WTAGUPDATE must_be_deasserted`; APB 20→20 and AHB 12→12 this run (this run's two AHB
  `HRESP` records carry DIFFERENT conditions — "To start the ERROR response" vs "In the next
  cycle…" — genuinely different facts, correctly NOT merged; the no-condition ×2 shape from the
  earlier run is locked by unit test instead). Eval stays P=R=F1=1.000 on all three (scoring uses
  key sets — the win is canonical-artifact cleanliness + merged provenance).**
- ID: `EXTRACTION-QUALITY-GAUGE.0` · Status: `pending` · Goal: wire the gauge into converge/CI.

## Changelog

- `2026-06-06`: Created. Gauge established (NLI-oracle not-entailed rate); CHI measured at ~80%
  erroneous (hand-validated 18/18), taxonomy recorded; fix backlog opened. See
  [[conformal-tier-agreement-degenerate]] and `docs/tasks/TABLE-GRITS-CONFORMAL.md`.
- `2026-06-10`: `.8` DONE — the `must_be_value` recall gap is closed. Root cause: the prompt could
  not express a validity requirement (model emits `[]`), compounded by two silent `parse_kind`
  drops. Fix: prompt states the `must_be_value`+`VALID` convention; Rust recovers an un-echoed
  value from the source sentence via the Pattern extractor's own binder grammar. Measured
  pre→post: APB 4/6→6/6, AHB 2/6→6/6, AXI 4/4→4/4 (control) — 10/16→16/16 gold facts, FP ledger
  net unchanged (the residual FP class is `.3` condition/permission work). See
  [[llm-primary-must-be-value-recall]].
- `2026-06-10`: `.3a` DONE — the condition-only-subject gate kills the entire measured
  condition-read-as-obligation FP class: **APB/AHB/AXI all P=R=F1=1.000 on the labeled constraint
  task** (FPs 3→0, recall held 16/16, conformal calibrates on all three). The per-item audit caught
  two first-cut defects (span crossing sentence ends; gerund action-coordination misread as a
  condition) before commit. See [[llm-primary-condition-subject-gate]]. Open: `.3b`
  permission-vs-obligation + relational-vs-value.
- `2026-06-10`: `.3b` DONE — the permissive-only frame gate (subject-sentence-scoped) removes the
  probed recommendation/hypothetical frame errors from the CANONICAL artifact (AHB 15→12, exactly
  `HPROT[0]`/`HSEL`/`HTRANS-IDLE`; `HEXOKAY`'s mandatory clause and the ERROR-procedure
  `HRESP`/`HREADYOUT` records survive); APB/AXI controls unchanged; eval stays perfect on the
  extended gold (+2 AHB negatives = regression armor). The per-item audit again killed the first
  cut (block-scoped modal check over-killed 15→8 via an incidental "can"). See
  [[llm-primary-permissive-frame-gate]]. Remaining: `.3c` relational-vs-value +
  descriptive-narration frames.
- `2026-06-10`: `.4` DONE — `dedup_constraints` (subject+kind+value+negation+condition key,
  provenance-merging, determinism-safe) in `extract-constraints-llm`. Live: AXI 54→50 (two
  triple-stated facts each collapsed to one record carrying all three statement ids); different
  conditions stay distinct facts; eval unchanged at P=R=F1=1.000 ×3. The original `.gauge`
  duplication taxonomy (~30% on CHI) now has its mechanism in place for the CHI-class re-measure
  once `.FIELD` lands.
