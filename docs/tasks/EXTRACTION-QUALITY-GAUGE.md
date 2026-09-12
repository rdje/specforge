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

- ID: `EXTRACTION-QUALITY-GAUGE` · Status: `active` · Children: `.0`–`.4` (incl. `.3a`–`.3k`, and
  `.3k.1`–`.3k.4`)
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
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD` · Status: `active` (designed `2026-06-10`, split `.1`–`.4`) ·
  Goal: the SIGNAL-vs-FIELD ontology for packet/flit protocols (CHI/CXL/PCIe-class) — model fields
  (flit/message contents) distinctly from signals (physical wires), per the doc's own
  signals-vs-fields split. The real future target for CHI-like PDFs (owner: "in fine we need to
  handle such cases too").
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.1` · Status: `done` (`2026-06-10`, design + corpus probe,
  docs-only) · Goal: ground the ontology in how the corpus actually declares fields BEFORE coding
  (the `.9.7`/`.9.8` method). **Probed all 49 persisted SourceIRs with structured tables.** Findings:
  - **The document's own table-header vocabulary types its rows** — fields are declared in tables
    whose name column is field-titled (`Field` / `Field name`), signals in `Signal`-titled tables.
    CHI: **36 field tables** (captions "Request channel fields" / "Response packet fields" / "Snoop
    request fields" / "Data packet fields"; shapes `Field|Description`,
    `Field|Affects structure|Description`) yielding **79 distinct field names** — including the
    exact `.gauge`/`.6` mis-typing class (`DBID`, `TxnID`, `ReturnNID`, `Addr`, `Opcode`...); its
    signals live in `Signal|Description` tables captioned "<channel> interface signals"
    (`REQFLITV`, `REQFLITPEND`, `REQLCRDV`...). CHI-C2C: 21–124 field tables, with widths
    (`Field name|Width (bits)|Value`). CXS: "Packet control fields". DTI/USB carry the shape too.
  - **Two populations share the field-titled column** — packet docs (message fields) AND register
    docs (RISC-V Debug 57× `Field|Description|Access|Reset`, Intel VT-d 102×
    `Bits|Access|Default|Field|Description`). **Discriminator (structural, no name lists):
    register-access vocabulary columns (`Access`/`Reset`/`Default`) and/or in-register bit-position
    columns mark a REGISTER-field table** (already owned by the register surface); a field-titled
    table without them declares MESSAGE fields.
  - **Today the field tables are inert**: all 36 CHI field tables are `table_kind: unknown` — fields
    have NO typed home, so field obligations in prose can only become wrong signal constraints or
    be dropped. That is root cause #1's deepest layer.
  - **Defect discovered:** `entity_prompt` (`ir/entity_typing.rs`) literally defines
    `signal = a wire/pin/field carrying a value` — the ontology is conflated at the LLM judgment
    point itself.
  - **Out of first scope (honest residual):** CCIX declares fields as `Bit Location|Field
    Description` (name embedded in prose) and OpenCAPI as `Operand mnemonic|Field width|Description`
    — different strategies later, per multi-strategy/best-wins.
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.2` · Status: `done` (`2026-06-10`) · Goal: **capture** — the
  first-class `message_field_records` EvidenceIR surface. Shipped (`ir/evidence.rs`):
  `MessageFieldRecord` (id, name, container, optional `bit_width`, optional description,
  `supporting_table_ids` provenance) + `message_field_surface` via the `run_surface` framework
  (key = container+name; manifest entry `message_fields`; extractor
  `message_fields.container_field_table`). Gates, all structural: (1) name column is EXACTLY
  `Field`/`Field name` (a merged CCIX `Field Description` column never qualifies); (2) the
  one-place register discriminator — `is_register_field_header` (Access/Reset/Default/Type
  vocabulary) claims the table for the REGISTER surface, never here; (3) the caption must anchor
  "field(s)" to a container noun at distance ≤2 (`channel|packet|message|flit|header|frame|
  request|response` — grammar vocabulary, not a name list), so register/descriptor captions
  ("Address Fields in Remappable Interrupt Request Format", "Mode Register 0") never fire;
  (4) continuation captions ("Table B2.2 Continued from previous page") inherit the container via
  the table-ref token and MERGE provenance into the first record; (5) width read only from a
  SINGLE unqualified width column (`Width (bits)`/`Bits`), plain count or `[hi:lo]` range —
  per-variant `Width (bits) ReqS` stays honest `None`; (6) restriction/status tables
  (`Field name|Restriction`, `Field|Value|Status`) declare nothing. Deterministic (encounter
  order, lookup-only maps). **Measured live (real Rust extractor over all persisted SourceIRs, the
  `#[ignore]`d `message_field_corpus_sweep_local_measurement` harness): fires ONLY on the
  packet-protocol family — CHI 106 fields / 4 containers (Request channel / Response packet /
  Snoop request / Data packet; `TxnID`, `DBID`, `Opcode`... — the exact `.gauge` mis-typing class,
  now typed), CHI-C2C 149/143/189 fields (89/93/164 with width), CCIX 1.x 47/50/51 (~95% with
  width), CXS 1; zero on every register/wire doc.** No-regression PROVEN: git-stash before/after
  rebuild of all 12 intact-bundle docs (incl. RISC-V Debug's 60 register field-tables, AXI's MPAM
  sub-field tables, SMBus) — byte-identical except the additive `message_fields` manifest entry,
  zero message fields each. +6 pure tests (lib 1524) + kg-bench fixtures
  `message_field_table_gold` (fields in `message_field_records` with widths + honest-absence lock,
  NOT in canonical signal inventory) and `message_field_register_table_negative` (register-vocab
  table captioned "message fields" → zero) with new kg-bench assertion keys
  `message_field_count`/`message_fields_include` (incl. `bit_width_absent`)/
  `message_field_names_exclude` — kg-bench 153/153. `run_ci.sh` GREEN. Honest residuals recorded:
  CCIX 2.0 `Bit Location|Field Description`, OpenCAPI `Operand mnemonic`, USB descriptor tables
  (no container bigram) = later strategies; validate/document-class integration deliberately
  deferred. Book: `pipeline/evidenceir.md` subsection. KM [[message-field-records-surface]].
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.3` · Status: `done` (`2026-06-10`) · Goal: **discriminate** —
  the ontology is now in the entity-typing gate. Shipped (`ir/entity_typing.rs`):
  `EntityType::Field` (parse accepts `field`; `as_str` = `field`),
  `EntityEvidence.declared_in_field_table` gathered from the NEW `message_field_records` catalog
  (case-insensitive), grounding rule in `classify_entity` — declared in a field table and in no
  signal table → authoritatively `Field` with NO LLM call; a signal-table declaration outranks
  when a name is in both; `is_valid_signal_subject(Field) == false`, so the `DBID`/`TxnID` class
  can never be a constraint/relation subject. The conflated prompt defect (`.FIELD.1`) is fixed:
  `signal = a physical wire/pin`, `field = a named portion of a packet/flit/message payload (not a
  wire)`, plus a `Declared in a message-field table:` evidence line. All three consumers
  (`extract-constraints-llm`, `entity-type`, `eval-extraction`) inherit through
  `gather_entity_evidence`. **Probed live (qwen2.5:14b-instruct, temp 0) before shipping (the `.8`
  method): controls UNCHANGED — `TXSACTIVE`→signal (the `.1` recovery preserved),
  `LICENSEE`→boilerplate, `CMO`→transaction; undeclared-field boundary characterized honestly —
  "the ReturnNID field …" phrasing → `field`, a bare field-word-free mention → `signal` (same as
  pre-slice; the deterministic catalog, not the model, carries declared fields).** +3 unit tests +
  1 end-to-end test (field table on persisted SourceIR → `message_field_records` → typed `Field` →
  rejected as signal subject; lib 1527); kg-bench 153/153; `run_ci.sh` GREEN. Live CHI constraint
  re-measure stays bound to `.FIELD.4` (CHI normalized bundle cleaned; PDF host-local).
  Book: the `extract-constraints-llm` section now documents the deterministic field rejection.
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.4` · Status: `done` (`2026-06-10`) · Goal: **capture the
  intent** — field-subject obligations become field-scoped constraints instead of dropped, plus
  the CHI-class re-measure on persisted artifacts. **DECISION: a parallel
  `message_field_constraints` EvidenceIR surface, NOT a subject-kind discriminator on
  `SignalConstraintRecord`** — a discriminator would force every downstream signal-constraint
  consumer (eval canonical keys, nli-verify, semantic carry-through, the ISF adapter) to filter
  by kind or silently keep treating fields as wires, which is the exact mis-typing this tree
  exists to kill; a separate surface keeps the canonical signal surface wires-only by
  construction. Shipped: `MessageFieldConstraintRecord` (subject_field + catalog `containers`
  provenance + the SHARED `SignalConstraintKind` vocabulary — what a requirement can SAY is the
  same, what it is ABOUT differs — + same condition/value/statement provenance shape) beside
  `MessageFieldRecord`; serde-additive `EvidenceIr.message_field_constraints` (old artifacts
  load; empty surface serializes to nothing); `ground_constraint_typed` in
  `ir/constraint_extract_llm.rs` dispatches on the `.FIELD.3` entity type AFTER the shared gates
  (`.3a` condition-only, `.3b` permissive-frame, `.8` value recovery, `.2` condition grounding —
  a field obligation passes the SAME discipline, fixture-locked), with `ground_constraint` kept
  as the signal-only view (a field subject still never reaches `signal_constraints`);
  `dedup_merge_by` generalizes the `.4` provenance-merging dedup over both surfaces
  (`dedup_field_constraints` keyed subject+kind+value+negation+condition; containers are catalog
  provenance, not identity); `extract-constraints-llm` routes Field records to the new surface
  (replace semantics, `llm_fieldcon_NNNN` ids, separate report line). +5 pure tests + the
  `.FIELD.3` end-to-end test extended through the REAL entity-typing composition (lib 1532);
  kg-bench 153/153. **Measured live (qwen2.5:14b-instruct temp 0; persisted CHI evidence via the
  `.8` redirected-copy protocol; catalog injected from the REAL `.FIELD.2` extractor over
  persisted SourceIR — 106 fields, reproducing `.FIELD.2` exactly; new `#[ignore]`d
  `message_field_catalog_dump_local_measurement` harness): BASELINE (no catalog) mis-types
  `TagOp must_be_value 0` + `PBHA must_be_value 0` as SIGNAL constraints; WITH CATALOG the
  signal surface is EXACTLY the document's 4 real flit-valid wires
  (`REQFLITV`/`RSPFLITV`/`SNPFLITV`/`DATFLITV` `must_be_high`, conditions preserved) and the 2
  field obligations land in `message_field_constraints` with containers (`TagOp` → Request
  channel/Response packet/Data packet) and merged provenance (the twice-stated TagOp fact = ONE
  record, both statement ids; dedup 3→2). The Pattern artifact's 5 junk `REQ must_be_value`
  records are gone in both runs. Wire controls (extract-constraints-llm on redirected
  APB/AHB/AXI copies + `eval-extraction --provider skip`): volumes EXACTLY the `.3b`/`.4` state
  (APB 20, AHB 12, AXI 54→50 w/ 4 merged), zero field constraints, P=R=F1=1.000 ×3, doc-level
  recall 16/16, conformal empirical_error 0.000.** Honest residuals: (a) field PRESENCE
  requirements ("the MPAM field must be included on the REQ and SNP channels") have no
  constraint-kind slot — probed live, model outputs `[]` (the `.8` root-cause shape; a future
  probed kind-vocabulary slice, NOT guessed); (b) the FULL fresh-yield CHI gauge re-measure
  (the original 162-constraint vintage) still needs re-ingest — PDF host-local, request
  re-provision; (c) validate/document-class integration of the field surfaces stays deliberately
  deferred (same as `.FIELD.2`). Book: `pipeline/evidenceir.md` `.FIELD.4` subsection +
  `commands/quality-and-learning.md` routing rewrite. KM [[message-field-constraints-surface]].
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
- ID: `EXTRACTION-QUALITY-GAUGE.3c` · Status: `done` (`2026-06-14`, CODE + full gold battery;
  unblocked by `PDF-VARIANT-DIGESTION.13c`) · Goal: the **descriptive-narration frame** gate (the
  larger of the two `.3c` classes; relational-value deferred to `.3d`).
  **SHIPPED:** pure `is_descriptive_narration_binding(text)` in `ir/evidence.rs`, called in
  `extract_dynamic_signal_constraints` in the logic-level-binding branch (Pattern path = the root
  mint site, so it cleans BOTH surfaces — the LLM-primary recall universe is the Pattern sentence
  set). Gate fires ONLY when: an ACTION bind verb (`set/sets/setting/drive/drives/driving/driven`,
  never static `tied/held/pulled/forced`) is present, NO mandatory modal (`must`/`shall`/`required
  to` — the bare adjective "required" as in "the required value" is NOT mandatory; this fixed a
  first-cut over-keep caught by the timing-walkthrough test), AND a descriptive marker (`this
  signal` / timing anchor `T<n>` / `figure … shows`). +6 pure tests; lib 1614 → **1620**; full
  `scripts/run_ci.sh` GREEN (incl. a `manual_contains` clippy fix). **Verified (fresh release bin):**
  CHI rebuilt 13 → **5** signal constraints — removing EXACTLY the 8 `*FLITV`/`*LCRDV` description
  cells, keeping the 5 `REQ must_be_value 0/I` field-obligation rows (a DIFFERENT class — the
  `.FIELD.4`/promotion concern, correctly untouched). **Gold-safe (per-item, on gated-Pattern
  rebuilds with backup/restore of the promoted wire artifacts):** APB/AHB/AXI constraints
  P=R=F1=1.000 + WIRE-BASED-100 relations 1.000 + temporal 3/3+4/4+3/3; SWD constraints/relations
  1.000 + SWD-derivation frame/operation/state 1.000; kg-bench 156/156. **HONEST gauge note
  (`feedback_scoring_rigor`):** the CHI NLI not-entailed RATE *rose* 69.2% → 100% (5/5) when the 8
  descriptive constraints dropped — because the NLI judge textually entails "sets HIGH" ⇒ "is HIGH"
  and so had ENTAILED 4 of the 8; but a *valid* strobe (`REQFLITV`) is semantically NOT an always-high
  invariant, so per-item the cleaned surface is strictly MORE correct. The gauge is a textual
  heuristic; the per-item semantic audit is ground truth (same precedent as `.3b`). The 5 remaining
  100%-not-entailed REQ rows are the field-mis-attribution class a converge's now-default promotion
  cleans (the `.4` sweep measured CHI 69.2%→16.7% promoted). Book: `pipeline/evidenceir.md` `.3c`
  subsection (+ FIELD.4 example reconciled). The original `.3c` design follows. **Probe-first (read-only, no 14B, over all
  78 persisted evidence docs, `source_text` of 583 signal_constraints):** the descriptive-narration
  class = **38 records** the deterministic Pattern path mints by reading an *action* of an actor as a
  global invariant — `logic_level_binding_kind_from_text` matches `sets|drives|driven <signal>
  HIGH/LOW` and `extract_dynamic_signal_constraints` mints `MustBeHigh/Low`. Two sub-forms, both
  pure narration: (a) **signal-description cells** — "The transmitter/receiver **sets this signal**
  HIGH to indicate/return …" (CHI's 8 FLITV/LCRDV — the exact records the `.13c` gauge flagged
  not-entailed); (b) **timing-diagram walkthrough** — "At T2, the power controller **sets PREQ
  HIGH**. The interface state is now P\_REQUEST." / "- T3 The Manager **sets FABORT HIGH** to
  request that the WRITE is aborted." / "Figure 3-5 **shows the case where** the controller sets
  PREQ HIGH" (ihi0068 low-power 13, ihi0083 GPIO 13, HBM2 1). **Per-item audit: all 38 are genuine
  errors** — a signal that GOES high as part of its function (cell) or AT a waveform step (timing)
  is not a global `must_be_high` invariant; real timing facts belong in the temporal layer, not a
  flat constraint. **Gold-safety proven by the probe: ZERO APB/AHB/AXI/SWD constraints match either
  sub-form** (gold obligations are phrased "X must be …" / "must drive X LOW" / static "tied HIGH" —
  none is actor-action narration), so the gate cannot regress the wire gold gates. **Placement
  decision: gate the PATTERN path (`extract_dynamic_signal_constraints`), not the LLM-primary path
  where `.3a`/`.3b` live** — because the LLM-primary extractor's recall universe IS the Pattern
  surface's sentences, so refusing to mint at the root cleans BOTH surfaces with one deterministic,
  14B-free-verifiable change (and most of the corpus carries the Pattern surface canonically). Gate
  design (conservative, over-kill-guarded per the `.3a`/`.3b` lesson): drop a logic-level-binding
  constraint ONLY when the bind verb is an ACTION verb (`set/sets/setting/drive/drives/driving/
  driven` — NEVER the static-invariant verbs `tied/held/pulled/forced`), there is NO mandatory modal
  (`must/shall/required`) in the subject clause, AND a descriptive marker is present (the phrase
  `this signal`, a timing anchor `T\d+`, or a figure-narration phrase `figure … shows`). Verify:
  re-run the corpus probe for the exact removed-constraint diff (audit every drop), gold eval ×3 +
  SWD `--provider skip` stay 1.000, kg-bench green, full `run_ci.sh`, CHI gauge re-measured on the
  cleaned surface. Universal grammar only (ADR 0006 — no name lists).
- ID: `EXTRACTION-QUALITY-GAUGE.3d` · Status: `done` (`2026-06-14`, CODE + gold battery; spun from
  the `.3c` probe) · Goal: the **relational-value frame** — the second `.3c` class the corpus probe
  surfaced. An *inter-signal/field equality* ("ALLOW_UW **must be equal to the value of** ALLOW_PW",
  or the bounded "a value **less than or equal to the value of** the NVM Set Identifier Maximum
  field") has no slot in the constraint-kind vocabulary, so BOTH value-binding paths mis-mint a
  garbage `must_be_value` (a truncated value lifted off a condition token — DTI's `<token>
  must_be_value "E"` with wrong subjects `BYPASS`/`STRW`/`EL3`). **SHIPPED:** pure
  `is_relational_equality_constraint(text)` (keys on "… the value of …" / "the same value as …" —
  NOT bare "equal to", so a literal "equal to 0" is untouched), wired as an early refuse in BOTH
  `extract_dynamic_signal_constraints` (`dyn_sigcon_*`, the DTI 15) AND `extract_signal_constraints`
  (`sigcon_*`, the NVMe 5 — a SECOND extractor the per-item audit caught: the NVMe relational record
  was minted there, not by the dynamic path). Decision: refuse → honest residual (a typed
  `MustEqual{other}` kind was the alternative, deferred — no consumer needs inter-signal equality
  yet). +2 pure tests; lib 1620 → **1622**; full `run_ci.sh` GREEN; kg-bench 156/156. **Verified:**
  NVMe rebuilt 26 → **21** (all 5 relational records gone, 0 remaining) — DTI's 15 `dyn_sigcon_*`
  would clear identically but its `normalized/` bundle was artifact-reclaimed (canonical cleanup
  rides the next DTI re-ingest; gate proven by the NVMe rebuild + unit tests). **Gold-safe** per-item
  on gated-Pattern rebuilds of ALL four wire docs (backup/restore, non-destructive): APB/AHB/AXI
  constraints 1.000 + WIRE-BASED-100 relations 1.000 + temporal 3/3+4/4+3/3; SWD constraints/
  relations 1.000 + SWD-derivation operation/state/frame 1.000 — probe-confirmed gold-safe (no
  wire-doc obligation is a relational equality). NVMe `must_be_stable` field-description cells
  (`This field indicates …`) are a SEPARATE descriptive-field-cell shape (a future gate), not
  relational. Book: `pipeline/evidenceir.md` `.3d` paragraph. **The `.3` constraint-precision program
  (`.3a` condition-subject / `.3b` permissive-frame / `.3c` descriptive-narration / `.3d`
  relational-value) is now complete.**
- ID: `EXTRACTION-QUALITY-GAUGE.3e` · Status: `done` (`2026-06-15`, CODE + full gold battery; PNT pick
  — the candidate future leaf `.3d` recorded: the NVMe `must_be_stable` "This field indicates …"
  descriptive-field-cell class, a decidable, probe-first, no-14B sibling of `.3c`/`.3d`) · Goal: the
  **descriptive-field-cell spurious-subject** gate.
  **SHIPPED:** pure `is_descriptive_field_cell_spurious_subject(text, subject)` in `ir/evidence.rs`,
  wired as a `subject_signals.retain(…)` in BOTH value-binding extractors (`extract_signal_constraints`
  `sigcon_*` — where all 9 errors mint — AND `extract_dynamic_signal_constraints` `dyn_sigcon_*`, so a
  future doc on the dynamic path is covered; the dynamic NVMe keeps stay kept). The gate fires ONLY
  when: the subject is a plain identifier; the source carries a `"this field <descriptive-verb>"`
  marker (`indicates`/`specifies`/`describes`/`contains`/`defines`/`represents`/`reports`/`identifies`/
  `provides` — never the obligation lead "this field shall/must/should"); AND the subject does NOT
  occur (identifier-boundary, case-insensitive) BEFORE the marker (= it is not the field's own name).
  +3 pure tests (drop / keep-own-mnemonic / never-touch-non-field); lib **1622 → 1625**; `cargo fmt`
  clean; warning-deny clippy clean. **Verified (fresh release bin, canonical rebuilds):** NVMe 21 → 20
  (drops exactly `sigcon_0004 FFFF must_be_stable`; the 4 dynamic-path keeps `SANICAP`/`HMDLLA`/
  `HMDLAL`/`ELEN` + `CBA`×2 + the real `ANA…` `must_not_change` sentence all kept; NVMe semantic+intent
  rebuilt to stay coherent), CCIX r1.0a 23 → 15 (drops exactly the 8 descriptive-cell subjects
  `CCIX`/`PCI`/`SRAM`/`DDR`/`NVDIMM`/`HBM`/`SAMA`/`CCIX`; the non-"This field" `HAC`/`HAM`/`DDR`/
  `NVDIMM`/… table rows correctly UNTOUCHED — scope discipline). **Corpus residual = 0** (no
  descriptive-field-cell spurious-subject record remains in the 78-doc corpus). **Gold-safe (live
  battery, gated-Pattern rebuilds of all four wire docs with backup/restore — non-destructive):**
  APB/AHB/AXI constraints **1.000** + relations **1.000** + temporal **3/3+4/4+3/3**; SWD constraints/
  relations **1.000** + SWD-derivation frame **11/11**/operation **4/4**/state **13/13**; kg-bench
  **156/156**. Refuse → honest residual (the field's real obligation, if any, rides its own mnemonic;
  recovering the correct subject of a `"the field must indicate 0h"`-style obligation is a separate
  recall concern, not this precision gate). The out-of-scope `DDR`/`NVDIMM` `must_be_stable` table rows
  that lack a "This field" marker are an honest residual (a different structural shape; no denylist —
  `feedback_avoid_denylists_prefer_structural`). Book: `pipeline/evidenceir.md` `.3e` paragraph. A register/structure field-definition cell narrates what the field IS — the
  field's own name PRECEDES a `"This field <descriptive-verb>"` marker, while value-meaning tokens
  appear only AFTER it (memory-type names `SRAM`/`DDR`/`NVDIMM`/`HBM`, protocol acronyms `CCIX`/`PCI`,
  a hex literal `FFFF`, a truncated `SAMA`). The deterministic constraint path lifts one of those body
  tokens as the subject and mints a garbage `must_be_*` whose subject is not a wire/field at all
  (`.gauge` root cause #1, the "Spurious subject" class — largest). **Probe-first (read-only, no 14B,
  over all 78 persisted evidence docs, `source_text` of every signal_constraint):** the class =
  **15 records / 2 register-structure docs** whose `source_text` carries a `"This field <verb>"`
  descriptive marker — CCIX r1.0a ×8 (`MultiPortDevCap`/`MemPoolSpcificMemTypeCap`/`MemPoolAddrCap`
  cells → subjects `CCIX`/`PCI`/`SRAM`/`DDR`/`NVDIMM`/`HBM`/`SAMA`) + NVMe ×7. **Per-item audit:**
  the **9** whose subject appears ONLY AFTER the marker are genuine spurious-subject errors (8 CCIX +
  NVMe `sigcon_0004` `FFFF`); the **6** whose subject is the cell's own leading mnemonic
  (`(CBA):`/`(SANICAP):`/`(HMDLLA):`/`(HMDLAL):`/`(ELEN):` — appears BEFORE the marker, with a real
  `shall` obligation) are legitimate and MUST be kept. **Discriminator (structural, ADR 0006 — no name
  lists):** in a `"This field <descriptive-verb>"` cell, the subject must appear (whole word,
  case-insensitive) BEFORE the marker (= it is the field's name) → keep; a subject appearing only
  after it was lifted from the descriptive body → spurious → drop. All 9 errors are minted in
  `extract_signal_constraints` (`sigcon_*`); gate wired into BOTH value-binding extractors (mirrors
  `.3d`) so a future doc on the dynamic path is covered too — the 4 dynamic-path NVMe keeps stay kept
  by the before-marker rule. **Gold-safe by the probe: ZERO APB/AHB/AXI/SWD constraints match** (wire
  docs declare no `"This field"` cells; obligations are phrased "X must be …"). Verify: NVMe rebuild
  21→20 (FFFF gone), CCIX r1.0a rebuild 23→15 (8 gone), wire gold ×4 stay 1.000 on gated-Pattern
  rebuilds, kg-bench green, full `run_ci.sh`. Refuse → honest residual (the field's real obligation,
  if any, rides its own mnemonic — recall of the correct subject is a separate concern).
- ID: `EXTRACTION-QUALITY-GAUGE.3f` · Status: `done` (`2026-06-15`, CODE + full gold battery; PNT pick —
  owner-chosen "EQG constraint precision" direction; the per-item NVMe audit under `.3e` left this
  residual) · Goal: the **alphabetic-value word-boundary** gate on the deterministic value binder.
  **SHIPPED + VERIFIED:** pure `lead_binds_value(text, lead, value_lower)` in `ir/evidence.rs` replaces
  the substring `contains_any` inside `extract_discovered_state_value_from_text` (the ONE shared value
  matcher used by `extract_dynamic_signal_constraints` `dyn_sigcon_*`) — it requires a trailing
  identifier boundary after the matched value ONLY when the value ends in a letter; numeric values stay
  lenient (preserve "0h"). +3 tests, lib 1625 → **1628**, `cargo fmt` clean, warning-deny clippy clean
  (`let`-chain collapse), full `run_ci.sh` GREEN, kg-bench **156/156**. **Verified (fresh release bin):**
  NVMe `evidence --dry-run` **20 → 19** — removes EXACTLY `dyn_sigcon_0013 SANICAP must_be_value NO`
  (the "shall be `no`n-zero" fragment), nothing added, `message_field_records`/`timing_constraints`
  byte-identical. **Wire-build invariance proven:** a fresh-Pattern rebuild (new bin) of all four wire
  docs has **0** alphabetic `must_be_value` records that `.3f` would alter → OLD/NEW bins produce
  identical wire Pattern builds. **Gold-safe (non-destructive temp-evidence-root eval, new bin):**
  APB/AHB/AXI/SWD constraints **1.000** + relations **1.000** (content-anchored) + APB/AHB/AXI temporal
  **3/3+4/4+3/3**; SWD-derivation frame **11/11** / operation **4/4** / state **13/13**;
  `seed_nvme_registers` field-name recall **28/29** unchanged. Refuse → honest residual.
  **Probe-first (read-only, no 14B, over all 78 persisted evidence docs):** the dynamic value binder
  `extract_discovered_state_value_from_text` (`ir/evidence.rs`) matches a discovered enum value behind a
  normative lead phrase (`must be`/`shall be`/`must remain`/`shall remain` `<value>`) with a PLAIN
  substring `contains_any`, so an ALPHABETIC value is lifted out of a longer word: NVMe `SANICAP` mints
  `dyn_sigcon_0013 must_be_value NO` off "this field **shall be `no`n-zero**" — the true obligations are
  "shall be non-zero" / "shall be cleared to 0h", so `NO` is a fabricated fragment
  (`feedback_scoring_rigor` — honest residual over a fabricated fact). **Measured fix:** require a
  trailing identifier boundary after the matched value ONLY when the value ends in a LETTER (an
  alphabetic enum value `NO`/`YES`/`VALID` must match a WHOLE word); a numeric value keeps lenient
  matching so a radixed literal ("shall be `0`h" → ELEN/RECFMT, genuine) still binds. **Corpus impact
  (measured pre-build over the 78-doc persisted surface):** removes EXACTLY 1 record (NVMe `SANICAP NO`)
  and flips ZERO wire-doc (APB/AHB/AXI/SWD) constraints; the genuine numeric `must_be_value 0` /
  "shall be 0h" family is untouched — the blanket after-boundary rule was REJECTED by measurement
  (it would wrongly drop ELEN/RECFMT "shall be 0h", where `0` legitimately prefixes `0h`). Universal
  grammar, no name lists (ADR 0006); root-cause fix in the ONE shared matcher (DRY); the `is <value>
  when` form is inherently whole-word-bounded and unchanged. Acceptance: +unit tests (the non-zero bug
  → None, whole-word `NO` → bound, `0h` numeric preserved, `valid` preserved); NVMe `evidence
  --dry-run` 20→19 (SANICAP NO gone, other 19 identical); gated-Pattern rebuild + `eval-extraction
  --provider skip` of all four wire docs (constraints+relations+temporal) stays 1.000 with
  `seed_nvme_registers` unaffected; kg-bench 156/156; full `run_ci.sh` GREEN; book
  `pipeline/evidenceir.md` `.3f` note + README bullet + KM card.
- ID: `EXTRACTION-QUALITY-GAUGE.3g` · Status: `done` (`2026-06-15`, CODE + full gold battery; PNT pick —
  owner-chosen "EQG constraint precision" direction; the per-item NVMe audit residual orthogonal to
  `.3e`) · Goal: the **dotted-cross-reference spurious-subject** gate.
  **SHIPPED + VERIFIED:** pure `is_dotted_cross_reference_subject(text, subject)` in `ir/evidence.rs`
  (drop iff the subject is a plain identifier AND every whole-word occurrence is immediately preceded by
  `<ident>.`), wired as a `subject_signals.retain(…)` in BOTH value-binding extractors right after the
  `.3e` retain. +2 test fns (dotted-ref dropped; standalone/own-mnemonic/mixed/absent kept), lib 1628 →
  **1630**, `cargo fmt` clean, warning-deny clippy clean, full `run_ci.sh` GREEN, kg-bench **156/156**.
  **Verified (fresh release bin, `.3f`+`.3g`):** NVMe `evidence --dry-run` **20 → 18** — removes exactly
  `SANICAP NO` (`.3f`) + `MPS must_be_value 0` (`.3g`, from `CC.MPS`); `BADD`'s genuine alignment
  obligation is KEPT; nothing added. Wire Pattern builds byte-identical (the gate alters 0 wire records);
  gold-safe on the non-destructive temp-evidence-root eval: APB/AHB/AXI/SWD constraints + relations +
  temporal + SWD-derivation all **1.000**; `seed_nvme_registers` 28/29 unchanged. Refuse → honest
  residual. **Probe-first (read-only, all 78 persisted evidence
  docs):** a constraint subject lifted from a `Reg.Field` dotted cross-reference inside the cell body —
  the NVMe `BADD` cell "Buffer Address (BADD): Indicates the host memory address … aligned to the memory
  page size (**CC.MPS**). The least significant bits … shall be 0" yields the genuine `BADD must_be_value
  0` (alignment) AND a spurious co-subject `MPS` (value 0) lifted from `CC.MPS` — a cross-reference to the
  CC register's MPS field, NOT this cell's subject. `.3e` does not reach it (the cell has no "this field
  <verb>" marker — it reads "(BADD): Indicates …"). **Measured impact:** the class = subjects whose EVERY
  whole-word occurrence in the source is immediately preceded by `<ident>.` — **1 record corpus-wide
  (NVMe `MPS`), 0 wire-doc**; the pure-hex-literal alternative was REJECTED by measurement (it would
  wrongly flag the real fields `CBA`/`BADD`, all-hex-letter names). **Fix:** pure
  `is_dotted_cross_reference_subject(text, subject)` (drop iff every whole-word occurrence is dotted-ref-
  prefixed) wired as a `subject_signals.retain(…)` in BOTH value-binding extractors, mirroring `.3e`;
  universal grammar (`Reg.Field` cross-reference is a cross-vendor register-spec idiom — structural, no
  name lists, ADR 0006). The cell's genuine subject (`BADD`, written "(BADD):") is kept. Acceptance:
  +unit tests (dotted-ref subject dropped; standalone/own-mnemonic subject kept; mixed standalone+dotted
  kept); NVMe `evidence --dry-run` drops `MPS` (18 with `.3f`+`.3g`); wire Pattern builds byte-identical
  (0 records altered, proven by fresh-bin scan) → wire gold ×4 1.000; kg-bench 156/156; full `run_ci.sh`
  GREEN; book `pipeline/evidenceir.md` `.3g` note + KM card.
- ID: `EXTRACTION-QUALITY-GAUGE.3h` · Status: `done` (`2026-08-10`, CODE + full gold battery) · Goal: the
  **value-position spurious-subject** gate — the last member of the `.3d`–`.3g` family, carried over as a named
  residual from `CORPUS-COVERAGE.2.50a` rather than left as a note.
  **Probe-first (read-only, all 80 persisted evidence docs):** a value-binding preposition phrase puts the bound
  VALUE after it, never the constrained thing, so a candidate reachable only there is a literal wearing a
  subject's clothes. NVMe: "… all entries … **shall have the Controller ID field set to FFFFh**" mints
  `FFFF must_be_value NO` — the obligation is on the Controller ID field, `FFFF` is the hex literal it is set to,
  and the record's own `target_value` (`NO`, from a later "shall be no more than one") comes from a *different*
  phrase, so the pattern path's positional value exclusion never reaches it. **Measured impact:** the class =
  subjects whose EVERY uppercase-run occurrence is immediately preceded by a value binder — **1 record
  corpus-wide (NVMe `FFFF`), 0 wire-doc, 0 in any rebuildable document.**
  **SHIPPED + VERIFIED:** pure `is_value_position_subject(text, subject)` in `ir/evidence.rs` over
  `uppercase_run_tokens` (the same tokenization `collect_subject_signal_tokens` uses, so `FFFFh` is seen as the
  candidate `FFFF` the extractor actually lifted), wired as a `subject_signals.retain(…)` in BOTH deterministic
  extractors right after the `.2.50a` retain. Standalone wins, exactly as in `.3g`: a candidate that occurs even
  once outside a value position is never touched. +3 test fns (the exact NVMe sentence yields no `FFFF`;
  standalone-beats-value-position; `PSEL must be set to HIGH` still yields `PSEL`), lib 1,804 → **1,807**.
  The all-hex-literal shortcut stays rejected by the `.3g` measurement (it would flag real `CBA`/`BADD`).
  Universal grammar, no name/radix/literal list (ADR 0006).
  **Honest limit:** NVMe has no normalized bundle, so its persisted artifact keeps the `FFFF` record until that
  document's own refresh re-ingests it; the replay proves the code is correct and ADR 0025's currency check
  reports the unmeasurable population rather than implying coverage.

- ID: `EXTRACTION-QUALITY-GAUGE.3i` · Status: `done` (`2026-09-12`) · Goal: **a flag that modifies an
  obligation must be read from that obligation.** Opened by `INVARIANT-SHAPE-ADMISSION.3` over one
  imprecise record and closed over a larger and sharper defect: `negated` was computed across the WHOLE
  statement while the subject and the condition already came from `constraint_bearing_sentence`, so a
  `must not` in one sentence flipped a constraint minted from another. RISC-V IOMMU published
  `The DV operand must be 1 for IODIR` as a **negated** constraint for exactly that reason.
  **Measured, read-only over the persisted corpus:** 20 negated records; **4** carry a negation that is
  not in their own obligation clause; **all 20** sit on a kind the classifier never matched, because
  every phrase in the table is affirmative (`must be X`) and a negated obligation never contains one.
  Shipped: the narrowing in BOTH deterministic paths (the dynamic path also stopped carrying a second
  copy of the negator list), plus the two spellings whose absence caused the defaulting —
  `must not be changed` → `MustNotChange`, and `must not be asserted` / `must not be active` →
  `MustBeDeasserted`, reached only after the affirmative `must be asserted` arm, which those strings do
  not contain.
  Rebuilt APB + AHB: **3 records retyped, 0 added, 0 removed** —
  `PSTRB` `must_be_stable`+negated → **`must_be_low`**, `HSIZE` → `must_not_change`,
  `HEXOKAY` → `must_be_deasserted`. `PSTRB` is the corroboration: it is refined to `LOW` by the polarity
  layer and now **agrees with `dyn_sigcon_0015`**, the same document's prose
  *"For read transfers, the Requester must drive all bits of PSTRB LOW"*. Two independent extraction
  paths, two different places in the document, one typed obligation — where before `3i` they
  contradicted each other, which is why APB's `actor_contracts` fall 15 → 14.
  Producer: the census in this leaf's Verification Log; controls in `mod extraction_quality_gauge_3i`.
  Prerequisite: none. Verification: all 20 adjudicated; observed RED; APB + AHB rebuilt.
  Commit: `EXTRACTION-QUALITY-GAUGE.3i`

- ID: `EXTRACTION-QUALITY-GAUGE.3k` · Status: `active` (opened `2026-09-12` by `.3i`; **scoped +
  split** `2026-09-12`) · Children: `.3k.1`, `.3k.2`, `.3k.3`, `.3k.4` · Goal: **every part of a
  published constraint must be read from the span that produced the record.** `.3i` established that
  for the negation; this container owns the rest. Its first result is that **both numbers `.3i`
  handed it were measured over the wrong population**, so the leaf is split around the populations
  that actually exist rather than around the two it inherited.
  **The scoping `.3k` demanded, done** — `scripts/measure_constraint_part_span.py`, read-only over
  all 78 persisted artifacts / 349 signal-constraint records, stratified by PRODUCER:
  `classify_signal_constraint_kind` has exactly **two** callers.
  `extract_signal_description_row_constraints` (`row_sigcon_*`, 12 records) already hands it ONE
  clause and is the reference implementation for this whole container — its census rows are 0/0/0.
  `extract_signal_constraints` (`sigcon_*`, 111 records) is the only caller that reads the whole
  statement. The other two producers of the same record type **never reach the classifier**: the
  dynamic path types a record from its VALUE BINDER
  (`extract_discovered_state_value_from_text` / `logic_level_binding_kind_from_text`) and the LLM
  path parses the kind the model NAMED (`parse_kind`).
  **Correction 1 — the kind-span population is 4, not 18.** Running the clause-vs-whole comparison
  over all 349 records gives **19** (not 18), of which **12 are `llm_sigcon_*` and 3 are
  `dyn_sigcon_*` — strata whose kind never passes through the function the change would edit**. The
  classifier's own population is **4 `sigcon_*` records**: AHB `sigcon_0002` and NVMe
  `sigcon_0005`/`0006`/`0007`.
  **Correction 2 — the "7 negated records on an untyped default" do not exist.** All 7 are
  `dyn_sigcon_*`, and that path **never publishes the untyped default**: every one of its 77 records
  carries a kind its own binder typed (`must_be_high` 15 / `must_be_low` 23 / `must_be_value` 39).
  The 7 was read off a classifier that does not run on them. The real population of "a negation
  stacked on a kind the document never typed" is **4 records — DTI `sigcon_0002`–`0005` — and they
  are the SAME 4 records as the relational magnitudes**, not a separate 7 plus 4. DTI publishes
  `OAS must_be_stable, negated` and `DTI must_be_stable, negated` — *"OAS must not be stable"* — from
  *"The range given by this field must not be greater than the size indicated by the OAS field …"*,
  a sentence that names no stability, whose subject is *"this field"*, and in which `OAS` is the
  right operand and `DTI` a message-name prefix. One refusal removes all three defects at once.
  **The decision the leaf asked for — what a clause is, per producer.** A record's parts must come
  from the span that PRODUCED the record, and that span is not the same construct for every producer:
  the pattern path is minted by a MODAL OBLIGATION, so its clause is `constraint_bearing_sentence`
  (already its subject's, condition's and negation's span — the kind is the only part still outside);
  the row path is minted by ONE clause of a description cell and already uses it throughout; the
  dynamic path is minted by a VALUE BINDING that **need not be modal at all** (*"X is tied HIGH"*),
  so `constraint_bearing_sentence` is the WRONG narrowing for it — it locates a modal the record may
  not have, and would silently move the record's span to an unrelated sentence. The dynamic path
  needs a BINDING-bearing clause, which no helper computes today; that is why `.3k.4` is separate
  rather than "call the same helper in both paths". The LLM path is out of scope here: a model names
  a subject and a kind deliberately rather than scanning a span, so the span question is a different
  question (`.3j`).
  **Ordering rationale.** `.3k.1` is strictly subtractive and lands first. `.3k.2` must land before
  `.3k.3`, because narrowing the kind's span moves 3 of its 4 records onto the ungated `generic_value`
  arm (NVMe would publish `ANAGRPID must_be_value UNIQUE`, a value lifted off the adjective following
  `shall be`); fixing the span before the arm would trade one fabricated fact for another.
  Verification: `python3 scripts/measure_constraint_part_span.py --self-test` (9/9) and `--check`
  (`kind-classifier call sites unchanged (2 callers, 1 reading the whole statement)`) both green; the
  census above re-derived from the persisted corpus, every listed record adjudicated against its own
  source text in this node. **Two observed-RED controls, because this census exists to stop a number
  being published from something nothing exercised:** (a) a third caller injected into `evidence.rs`
  (`zeta_probe_caller`) makes `--check` exit 1 naming the found set against the expected one, and the
  file was restored byte-identical (`git status --short crates/` clean); (b) deleting one self-test
  case makes `--self-test` exit 1 with `ran 8 cases, expected 9` — the total is a literal declared
  independently of the case list, the `PRODUCTION-GRAPH-CENSUS-PIN.3` property.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k`

- ID: `EXTRACTION-QUALITY-GAUGE.3k.1` · Status: `pending` · Goal: **refuse a comparative MAGNITUDE
  whose right operand is a REFERENCE.** `.3d` already refuses an inter-operand EQUALITY (*"X must be
  equal to the value of Y"*) because the constraint vocabulary has no slot for it; *"must not be
  greater than the size indicated by the OAS field"* is the same shape one relation along, and the
  vocabulary has no slot for it either. A magnitude against a LITERAL (*"must be greater than 0"*)
  must stay untouched — it is a value binding, and `.3d`'s own line between "the value of <other>"
  and a literal is the line to reuse.
  **Population: 4 records, all `sigcon_*`, all DTI** (`sigcon_0002`–`0005`), and it is simultaneously
  the entire negation-on-untyped-default population and 4 spurious subjects (`OAS` is the right
  operand; `DTI` is a message-name prefix). Strictly subtractive: nothing is retyped, 4 fabricated
  records are removed. Zero records in any other document, zero in the dynamic/row/LLM strata.
  Prerequisite: none. Verification: the census re-run to 0; a control pair (reference operand refused,
  literal operand kept); observed RED; DTI rebuilt and diffed.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2` · Status: `pending` · Goal: **what a clause that types nothing
  may publish.** Two arms of `classify_signal_constraint_kind` emit a fact the document did not
  state: the terminal `untyped_default` publishes `MustBeStable` for any obligation no phrase matched
  (**26 `sigcon_*` records** when classified over their own clause, 25 over the whole statement), and
  the `generic_value` arm lifts whatever word follows `must be `/`shall be ` as a typed value with no
  gate at all (**11 over the clause, 4 over the whole statement**). This leaf decides what each may
  publish — refuse, or admit with an explicit untyped marker — and is the precision leaf of the
  family. Prerequisite: none (but `.3k.3` depends on it). Verification: all 37 adjudicated
  individually; observed RED; the chain rebuilt for every document whose artifacts move.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.3` · Status: `pending` · Goal: **the kind reads its own obligation
  clause** — the original `.3k` goal, at its true size.
  `classify_signal_constraint_kind(&text.to_ascii_lowercase())` becomes
  `classify_signal_constraint_kind(&constraint_bearing_sentence(text).to_ascii_lowercase())` in
  `extract_signal_constraints`, joining the subject, the condition and (since `.3i`) the negation,
  which all already come from that span. **Population: 4 `sigcon_*` records.** AHB `sigcon_0002` is
  the mechanism `INVARIANT-SHAPE-ADMISSION.3` reported without explaining: its clause is *"When the
  Subordinate is initially selected, it must also monitor the status of HREADY…"*, which contains no
  kind phrase at all, and the published `must_be_asserted` comes from the NEXT sentence, *"HSELx must
  be asserted in the same cycle…"* — kind from one clause, condition from another. NVMe
  `sigcon_0005`/`0006`/`0007` take `must_not_change` from a sentence two clauses later whose own
  obligation is conditional on a capability bit. Prerequisite: `.3k.2` (see the container's ordering
  rationale). Verification: all 4 adjudicated; observed RED; AHB + NVMe rebuilt and diffed.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.4` · Status: `pending` · Goal: **the dynamic path's span
  discipline.** After `.3i` it reads its negation from `constraint_bearing_sentence` while its
  subject (`text_before_condition_marker(&statement.text)`), its value binder
  (`extract_discovered_state_value_from_text` over the whole lowered statement) and its condition
  (`extract_condition_clause(&statement.text)`) all read the WHOLE statement — so it is now the one
  producer whose parts are provably drawn from two different spans. The fix is not to call
  `constraint_bearing_sentence`: this path's records are minted by a value binding that need not be
  modal, so the clause it needs is the BINDING-bearing clause. Define it, then apply it to all four
  parts at once. Prerequisite: none. Verification: the population measured in-leaf against the real
  binder (it cannot be mirrored — `discovered_values` is derived per document); observed RED; the
  chain rebuilt for every document whose artifacts move.

- ID: `EXTRACTION-QUALITY-GAUGE.3j` · Status: `pending` (opened `2026-09-12` by
  `INVARIANT-SHAPE-ADMISSION.5`) · Goal: **the LLM-primary constraint path applies none of the
  positional spurious-subject gates this family built.** `.2.50a`, `.3e`, `.3g`, `.3h` and now
  `INVARIANT-SHAPE-ADMISSION.5` are all wired as `subject_signals.retain(…)` in the two DETERMINISTIC
  extractors only. `crates/specforge/src/ir/constraint_extract_llm.rs` has its own gates —
  `ground_constraint` / `ground_constraint_typed`, which check catalog membership, drop invented
  subjects and drop condition-only subjects — but nothing that reads WHERE in the sentence the subject
  sits. So a model proposal is grounded on "is this a declared signal?" and never on "is this the
  thing the sentence constrains?".
  **Measured instance:** AXI-H `llm_sigcon_0025`/`0027` attribute `WTAGUPDATE must be deasserted` to
  `WTAG` — the scan lifted a shorter declared name out of a longer identifier — and
  `INVARIANT-SHAPE-ADMISSION.5`'s narrowing cannot reach them because they are not produced by the path
  it gates. The census (`python3 scripts/measure_table_row_foreign_subject.py`) names them.
  **Scope this before implementing.** The question is not "call the five retains from the LLM path
  too": it is whether a grounded model proposal SHOULD be judged by a gate designed for a pattern
  path's full-text scan, since the model is not scanning — it names a subject deliberately, and a gate
  built to catch scanning artefacts may refuse a correct proposal the deterministic path could never
  have made. Measure how many current `llm_sigcon_*` records each of the five would refuse before
  wiring any of them.
  Prerequisite: none. Verification: the per-gate refusal count over the persisted `llm_sigcon_*`
  population, adjudicated individually; observed RED for whichever gates are wired; the chain rebuilt
  for every document whose artifacts move.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3i`

- [x] **REPRODUCE / MEASURE** — two read-only censuses over the 78-document persisted corpus. 20 records
  carry `negated`; **4** have no negator in their own obligation clause (RISC-V IOMMU
  `dyn_sigcon_0008`/`0009` from `The DV operand must be 1 for IODIR`, NVMe `dyn_sigcon_0018`, I2C
  `dyn_sigcon_0008`); **20 of 20** sit on a kind no phrase matched, because every phrase in the table is
  affirmative and a negated obligation never contains one. The second census — phrase match over the
  clause vs over the whole text — differs on **18 of 349**.
  **CORRECTED by `.3k`'s scoping (`2026-09-12`): both of this leaf's censuses were run over the whole
  349-record table, and `classify_signal_constraint_kind` has only 111 of them in its reach.** The
  second census re-derives to **19** over 349 — of which 12 are `llm_sigcon_*` and 3 `dyn_sigcon_*`,
  strata whose kind never passes through that function — and to **4** over the classifier's own
  `sigcon_*` population, which is what `.3k.3` inherits. "20 of 20 sit on a kind no phrase matched" is
  true of the `sigcon_*` stratum and vacuous for the `dyn_sigcon_*` one, whose kind comes from its
  value binder. The fix this leaf shipped is unaffected — a negation read from another sentence is a
  defect at any population size — but the numbers it published are not the ones it measured.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`. `extract_signal_constraints`
  computed `let lowered = text.to_ascii_lowercase()` over the WHOLE statement and used it for both kind
  and negation, while the subject used `constraint_bearing_sentence(text)` and the condition said so in
  its own comment (*"from the SAME bounded obligation the subject came from"*,
  `CONSTRAINT-EXTRACTION-V2.2`). `extract_dynamic_signal_constraints` did the same with its own inline
  copy of the negator list. The record's parts were drawn from different spans of one statement.
- [x] **ADDRESSED (verified)** — `obligation_is_negated(&constraint_bearing_sentence(…).to_ascii_lowercase())`
  in both paths, the dynamic path's duplicated list replaced by the shared predicate `.3` extracted, and
  the two missing spellings added. APB + AHB rebuilt `evidence → validate → semantic → validate → intent
  → validate → adapt`, zero failures: **3 retyped, 0 added, 0 removed**. `PSTRB`
  `must_be_stable`+negated → `must_be_low`, `HSIZE` → `must_not_change`, `HEXOKAY` →
  `must_be_deasserted`. **`PSTRB` is the corroboration**: polarity-refined to `LOW`, it now agrees with
  `dyn_sigcon_0015`, the same document's prose *"For read transfers, the Requester must drive all bits
  of PSTRB LOW"* — two independent extraction paths converging where they previously contradicted, which
  is why APB's `actor_contracts` fall 15 → 14.
  **Observed RED**: with the narrowing reverted,
  `a_negation_in_another_sentence_does_not_negate_this_obligation` fails and emits
  `MustBeValue { value: "1" }, negated: true` from `The ZETADV operand must be 1 for ZETADIR` — the
  fixture reproduces the live `dyn_sigcon_0008` exactly.
- [x] **NO REGRESSION** — `cargo test` 472 / 168 / **1435** / 4 green. One existing control changed
  expectation, and it is **the fix landing rather than a regression**:
  `invariant_shape_admission_3::a_row_states_a_constraint_only_when_its_clause_binds_to_that_row_signal`
  asserted `PSTRB` was `negated`; `INVARIANT-SHAPE-ADMISSION.3`'s own result recorded that record as
  imprecise and named this leaf as its owner. It now asserts `must_be_deasserted` with `negated: false`
  by the `WIRE-BASED-100.5b` guard. `cargo fmt --check` and `cargo clippy --all-targets -D warnings`
  green; `scripts/check_doctrines.sh` green. Retention at the declared 24: both held-out bundles
  restored from `generated/preserved/WIRE-BASED-100.10/`, `diff -r`-verified unchanged by the rebuild,
  and removed. Pre-rebuild snapshot at `generated/preserved/EXTRACTION-QUALITY-GAUGE.3i/pre-rebuild/`.
- [x] **GENERICITY (ADR 0006)** — a span narrowing plus four phrase strings that are the negative
  spellings of forms already in the table. No document, protocol, vendor, or signal name appears in the
  rule; the controls use invented names (`ZETADV`, `ZETASIZE`, `ZETAOKAY`, `ZETASTRB`).
- [x] **LOCKSTEP** — `docs/book/src/pipeline/evidenceir.md` gains "A negation belongs to the obligation
  it modifies". No production rule was deleted; two phrase forms were added and one span narrowed, and
  the book says which.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3h`

- [x] **REPRODUCE / MEASURE** — read-only census over all 248 deterministic constraint records: exactly one
  value-position-only subject (NVMe `dyn_sigcon_0011` `FFFF`), across one document, zero wire-protocol records.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: the pattern path excludes only the
  value it bound itself, and the dynamic path's subject scan admits any uppercase run, so a literal bound by a
  *different* phrase (`set to FFFFh`) reaches the subject slot; `.3d`–`.3g` express no value-position rule.
- [x] **ADDRESSED (verified)** — `is_value_position_subject` guards both paths; the exact NVMe sentence now yields
  no `FFFF` record, and standalone/ordinary value-binding subjects are provably kept.
- [x] **NO REGRESSION** — `kg-bench` 156/156; full `run_ci.sh` GREEN with 1,807 tests / five ignored; an isolated
  replay over all 22 rebuildable documents is unchanged 22/22, so the gate alters zero live records outside NVMe;
  57/57 emitted ISFs stay FSMGen-strict clean.
- [x] **GENERICITY (ADR 0006)** — value-binder preposition grammar plus identifier-boundary occurrence; no name,
  radix, literal-shape, vendor, or document list.
- [x] **LOCKSTEP** — code, this leaf, the book gate narrative, the Knowledge Map card, and the resume pointer
  agree before commit.
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
- ID: `EXTRACTION-QUALITY-GAUGE.0` · Status: `done` (`2026-06-10`; PNT pick — `.3c`'s
  relational-vs-value class has no measurable target on persisted artifacts: it was observed on CHI,
  whose fresh re-measure needs the host-local PDF re-provided, and the wire docs' FP ledger is
  currently clean at P=1.000 ×3) · Goal: wire the gauge into converge/CI as the STANDING per-doc
  quality report. **Shipped exactly per design (below) + verified live.** Verification
  (`2026-06-10`): +5 pure tests (2 `ir/nli_verify` builder/staleness + 3 `validate` reporting; lib
  1532→1537) + kg fixture `extraction_quality_gauge_persisted_gold` (kg-bench 153→**154/154**;
  locks metrics, Info finding, exactly-half-is-NOT-majority boundary, warnings excluded).
  **Live wire docs (qwen2.5:14b-instruct):** APB 4/14 not-entailed (28.6% — reproduces the
  tree's recorded ~29%; Info only; persisted ids `sigcon_0007/0008/0010`+`dyn_sigcon_0015` all
  verified present in the artifact, metrics back-annotated `28.6`), AHB 9/15 (60.0% — Warning
  fires), degraded persisted CHI 9/13 (69.2% — Warning), AXI 91/100 labeled + 2 abstained
  (91.0% — Warning). KEY HONEST FINDING the standing gauge makes visible: the CANONICAL
  artifacts still carry the PATTERN surface (the `.3a`/`.3b`/`.4` cleaned LLM-primary surfaces
  live only on /tmp redirected measurement copies — promoting them is the natural follow-up
  lever). **Live converge end-to-end (I2S, vlm skip + nlp ollama, DOCLING_DEVICE=cpu):** 2
  passes stable → gauge measured post-stability, persisted (`nlp3_sigcon_0001`), printed at the
  stable branch AND in the convergence summary; per-item read: `SCK must_be_asserted` extracted
  from an edge-synchronization *permission* sentence = genuine mis-extraction, correctly
  flagged. `run_ci.sh` GREEN. Book: `quality/validation.md` standing-gauge section,
  `pipeline/evidenceir.md` `.0` subsection, `commands/pipeline.md` converge subsection,
  `commands/quality-and-learning.md` validate list, `architecture-rationale.md` NLI section
  extension. KM [[extraction-quality-gauge-standing]]. Design:
  - **Persist the measurement** — today `nli-verify` is print-only, so the gauge dies with the
    terminal. New additive `EvidenceIr.extraction_quality_gauge: Option<ExtractionQualityGaugeRecord>`
    (`#[serde(default, skip_serializing_if = Option::is_none)]` — old artifacts load; absent
    serializes to nothing): `model`, `constraints_total` (signal-constraint surface size at
    measurement), `entailed`, `not_entailed`, `abstained` (Unknown verdicts — honest no-label),
    `not_entailed_constraint_ids` (review routing). Derived fractions are computed, never stored.
    Pure builder `gauge_from_conformal_pass` in `ir/nli_verify.rs` reuses the ONE existing NLI pass
    (`nli_conformal_pass`) — no second sweep of LLM calls. Encounter-order ids only
    (`EVIDENCE-DETERMINISM`: no hash-iteration order reaches output).
  - **`nli-verify` persists it** (same back-annotation semantics as `validate`:
    `ir.write_to_disk()` honors the recorded `artifact_layout`, so the redirected-copy measurement
    protocol keeps working). `--vlm-provider skip` stays a strict no-op.
  - **`converge` measures it after stability** — at the stable branch (after the rescan-plan step,
    so the gauge describes the FINAL artifact), when `--nlp-provider` is not `skip`, via a shared
    `measure_and_persist_gauge` helper (one implementation for both commands); summary prints the
    per-doc gauge. Evidence rebuilds drop the field to `None` by construction
    (`carry_forward_existing_knowledge` never carries it) — a rebuilt surface honestly requires a
    fresh measurement, and converge provides exactly that.
  - **`validate` reports it (the CI-safe surface — no provider needed, reads the persisted record):**
    metrics `extraction_quality_labeled` / `extraction_quality_not_entailed` /
    `extraction_quality_abstained` / `extraction_quality_not_entailed_pct` (`n/a` when never
    measured — the `recall_estimate_pct` precedent); Info finding `evidence_extraction_quality_gauge`
    (related_ids = the not-entailed constraint ids); Warning
    `evidence_extraction_quality_majority_not_entailed` when not_entailed > labeled/2 (scale-free
    "more wrong than right" line — the CHI-class shape, no magic corpus-tuned threshold); Warning
    `evidence_extraction_quality_gauge_stale` when the constraint surface changed since measurement
    (count mismatch OR a recorded not-entailed id no longer present — catches the
    `extract-constraints-llm` replace case whose ids are re-keyed).
  - **Tracked lock**: kg-bench `EvidenceIrPatch` gains an optional gauge patch + a fixture locking
    the validate metrics/finding; pure unit tests cover builder counts, staleness, majority warning,
    and honest absence.
  Acceptance: live gauge persisted + validated on the persisted wire docs (APB/AHB/AXI) and on a
  CHI-class doc (the Warning shape); full `run_ci.sh` GREEN; book (nli-verify, validate, converge,
  EvidenceIR pages) + README + KM card.

## Changelog

- `2026-09-12` — **`.3k` SCOPED + SPLIT** into `.3k.1`–`.3k.4`, and the scoping falsified both numbers
  it inherited. `scripts/measure_constraint_part_span.py` (new, read-only, 78 artifacts / 349 records)
  stratifies the constraint table by PRODUCER, because `classify_signal_constraint_kind` has exactly
  two callers and only one of them reads the whole statement. The kind-span population is **4
  `sigcon_*` records**, not 18 — 15 of the 19 the whole-table comparison finds belong to the dynamic
  and LLM paths, whose kind never passes through that function. The "7 negated records on an untyped
  default" **do not exist**: all 7 are `dyn_sigcon_*`, and that path types every one of its 77 records
  from its own value binder. The real population is the **same 4 DTI records** as the relational
  magnitudes, which publish *"OAS must not be stable"* from a sentence about a range comparison. The
  decision the leaf asked for is recorded in its node: a record's parts come from the span that
  produced the record, and that span is a modal clause for the pattern path, one description-cell
  clause for the row path, and a BINDING-bearing clause — which no helper computes yet — for the
  dynamic path, whose records need not contain a modal at all.

- `2026-09-12` — `.3i` closed, and it closed over a larger defect than it opened on. `negated` was read
  from the whole statement while the subject and condition came from the obligation clause, so a
  `must not` in one sentence flipped a constraint minted from another — 4 records corpus-wide, including
  `The DV operand must be 1 for IODIR` published as negated. Narrowed in both deterministic paths, and
  the two missing negative spellings added so the obligations stop defaulting. 3 records retyped across
  APB and AHB with nothing added or removed. `.3k` opened for the other half: the KIND's span (18 of 349
  records match a phrase outside their own clause — the mechanism behind `HSELx`'s mis-conditioning) and
  the refusal of a negation stacked on an untyped default (7 records) plus 4 relational magnitudes.
  **Both counts corrected by `.3k` (`2026-09-12`) — see that node.** They were measured over all four
  producers of a `SignalConstraintRecord`; only two reach the kind classifier. The kind-span population
  is **4**, and the "7 negated records on an untyped default" do not exist: all 7 are `dyn_sigcon_*`,
  and that path never publishes the untyped default. The real population is the SAME 4 DTI records as
  the relational magnitudes.

- `2026-08-10`: **`.3h` DONE** — the value-position spurious-subject gate ships, closing the last
  named residual of the `.3d`–`.3g` family (NVMe `FFFF` from `set to FFFFh`). Probe: 1 record corpus-wide, 0
  wire-doc. Both deterministic paths guarded; standalone occurrence always wins. kg-bench 156/156, full CI green
  at 1,807 tests, 22/22 rebuildable documents byte-unchanged.

- `2026-06-15`: **`.3g` DONE** — the dotted-cross-reference spurious-subject gate ships (PNT pick;
  owner-chosen "EQG constraint precision" direction; orthogonal sibling of `.3e`). A register/structure
  cell that cross-references ANOTHER register's field by dotted notation ("… aligned to the memory page
  size (`CC.MPS`)") had its trailing component (`MPS`) lifted as a spurious co-subject of the cell's own
  obligation → fabricated `MPS must_be_value 0`. `.3e` doesn't reach it (the cell has no "this field
  <verb>" marker). Fix: pure `is_dotted_cross_reference_subject` (drop iff every whole-word occurrence is
  preceded by `<ident>.`; a standalone occurrence is always kept) wired into BOTH value-binding
  extractors; universal `Reg.Field` cross-reference grammar, no name lists (ADR 0006). The pure-hex-
  literal alternative was REJECTED by measurement (it would wrongly flag the real fields `CBA`/`BADD`).
  +2 test fns, lib 1628→**1630**, full `run_ci.sh` GREEN, kg-bench 156/156. Live: NVMe 20→18 (`SANICAP
  NO` from `.3f` + `MPS 0` from `.3g`; `BADD` alignment kept), 0 wire build change, wire gold ×4 1.000,
  `seed_nvme_registers` 28/29 unchanged. Refuse → honest residual. Book `pipeline/evidenceir.md` `.3g`
  paragraph. KM [[dotted-cross-reference-subject-gate]]. The `.3` constraint-precision program now spans
  `.3a`–`.3g`; the remaining NVMe residuals (prose-subject `NVM`/`LBA`/`FFFF` catalog-quality class +
  the tangled spurious-digit-value class) need deeper non-narrow work, not another micro-gate.
- `2026-06-15`: **`.3f` DONE** — the alphabetic-value word-boundary gate ships (PNT pick; owner-chosen
  "EQG constraint precision" direction). The dynamic value binder
  `extract_discovered_state_value_from_text` matched a discovered enum value behind a normative lead
  (`must be`/`shall be`/`must remain`/`shall remain` `<value>`) with a PLAIN substring `contains_any`,
  so an alphabetic value was lifted out of a longer word: NVMe `SANICAP` minted `dyn_sigcon_0013
  must_be_value NO` off "this field **shall be `no`n-zero**" (the real obligations are "shall be
  non-zero" / "shall be cleared to 0h" — `NO` is a fabricated fragment, `feedback_scoring_rigor`). Fix:
  new pure `lead_binds_value` requires a trailing identifier boundary after the value ONLY when it ends
  in a LETTER (an alphabetic enum value matches a WHOLE word); a numeric value keeps lenient matching so
  a radixed literal ("shall be `0`h" → ELEN/RECFMT, genuine) still binds — the blanket after-boundary
  rule was REJECTED by measurement (it would wrongly drop "shall be 0h"). Universal grammar, no name
  lists (ADR 0006); root-cause fix in the ONE shared matcher (DRY). +3 tests, lib 1625→**1628**, full
  `run_ci.sh` GREEN, kg-bench 156/156. Live: NVMe 20→19 (only `SANICAP NO`), 0 wire-doc build change
  (proven), wire gold ×4 1.000 (constraints/relations/temporal/SWD-derivation) on a non-destructive
  fresh-Pattern temp-root eval, `seed_nvme_registers` 28/29 unchanged. Refuse → honest residual. Book
  `pipeline/evidenceir.md` `.3f` paragraph. KM [[value-binder-alphabetic-whole-word]]. The `.3`
  constraint-precision program now spans `.3a`–`.3f`.
- `2026-06-15`: **`.3e` DONE** — the descriptive-field-cell spurious-subject gate ships (PNT pick; the
  `.3d`-recorded candidate future leaf). Pure `is_descriptive_field_cell_spurious_subject(text, subject)`
  in `ir/evidence.rs`, wired into BOTH value-binding extractors as a `subject_signals.retain(…)`: inside
  a `"this field <descriptive-verb>"` field-definition cell, a subject that does NOT appear before the
  marker (i.e. is not the field's own name) was lifted from the descriptive body — a spurious subject
  (`SRAM`/`DDR`/`CCIX`/`FFFF`) — and is dropped; the field's own mnemonic (`CBA`/`SANICAP`/`ELEN`,
  appearing before) is kept. Universal grammar, no name lists (ADR 0006). +3 tests, lib 1622→**1625**,
  `run_ci.sh` GREEN, kg-bench 156/156. Live: NVMe 21→20 (lone `FFFF`), CCIX r1.0a 23→15 (8 enum/protocol
  subjects); corpus residual 0; the non-"This field" `DDR`/`NVDIMM` table rows correctly untouched
  (scope discipline). Gold-safe per-item on gated-Pattern rebuilds of all four wire docs (backup/restore):
  APB/AHB/AXI constraints+relations+temporal 1.000, SWD constraints/relations + derivation frame/op/state
  1.000. Refuse → honest residual. Book `pipeline/evidenceir.md` `.3e` paragraph. See
  [[extraction-quality-gauge-standing]].
- `2026-06-14`: **`.3d` DONE** — the relational-value frame gate ships, completing the `.3`
  constraint-precision program. Pure `is_relational_equality_constraint` (keys on "… the value of …"
  / "the same value as …", never bare "equal to") refuses an inter-signal/field equality in BOTH
  value-binding extractors (`extract_dynamic_signal_constraints` `dyn_sigcon_*` = DTI's 15, AND
  `extract_signal_constraints` `sigcon_*` = NVMe's 5, a second extractor the per-item audit caught).
  +2 tests, lib 1620→1622, full `run_ci.sh` GREEN, kg-bench 156/156. Live: NVMe 26→21 (5 relational
  gone); DTI 15 `dyn_sigcon_*` clear identically but its bundle was artifact-reclaimed (rides next
  re-ingest). All wire gold gates 1.000 on gated-Pattern rebuilds (backup/restore). Honest residual
  over a fabricated `must_be_value` (a `MustEqual{other}` typed kind deferred — no consumer needs
  it). Book `.3d` paragraph. See [[extraction-quality-gauge-standing]].
- `2026-06-14`: **`.3c` DONE** — the descriptive-narration frame gate ships. Pure
  `is_descriptive_narration_binding` in the Pattern path (`extract_dynamic_signal_constraints`)
  drops a logic-level binding that is actor-action narration, not an invariant: an action verb
  (`sets/drives`, never static `tied/held/pulled/forced`) + no mandatory modal (`must/shall/
  required to`) + a descriptive marker (`this signal` / timing anchor `T<n>` / `figure … shows`).
  Root-mint placement cleans both surfaces (the LLM-primary recall universe is the Pattern
  sentences). +6 tests, lib 1614→1620, full `run_ci.sh` GREEN, kg-bench 156/156. Live: CHI 13→5
  (the 8 `*FLITV`/`*LCRDV` description cells dropped; the 5 `REQ` field-obligation rows kept = a
  separate class); ALL wire gold gates 1.000 (APB/AHB/AXI constraints+temporal, SWD frame/op/state)
  on gated-Pattern rebuilds (promoted wire artifacts backed up + restored — non-destructive).
  Honest gauge note: CHI not-entailed RATE rose 69.2%→100% because the NLI judge textually entails
  "sets HIGH"⇒"is HIGH" and had entailed 4 of the 8 dropped — but a *valid* strobe is semantically
  not always-high, so per-item the cleaned surface is strictly more correct (`feedback_scoring_rigor`;
  same precedent as `.3b`). Relational-value ("equal to the value of", ~15 DTI records) deferred to
  `.3d`. Book `.3c` subsection added (FIELD.4 example reconciled). See [[extraction-quality-gauge-standing]].
- `2026-06-14`: **`.3c` UNBLOCKED** by `PDF-VARIANT-DIGESTION.13c` — the "needs a measurable
  target (CHI re-ingest)" gap is closed. CHI evidence rebuilt on canonical from its intact `.2`
  source bundle (no re-ingest) and the gauge re-measured with `qwen2.5:14b-instruct`: **9/13
  not-entailed (69.2%)**, persisted + `validate`-reported. The not-entailed set is the concrete
  `.3c` corpus material — two crisp error classes on the Pattern surface: (1) **field obligations
  mis-attributed to channels** (`TagOp`/`PBHA` "must be 0" → "REQ must be 0"; MPAM "must be
  included" → "REQ must be I"), which the `.FIELD.4` field-routing + the now-default
  `LLM-PRIMARY-PROMOTION.5` promotion clean (the `.4` sweep measured CHI 69.2%→16.7% promoted);
  and (2) **descriptive narration read as an invariant** ("the receiver sets REQLCRDV HIGH" →
  "REQLCRDV must be HIGH"). CHI's typed flit-field inventory also landed on canonical via the
  rebuild (`message_field_records` 0→106). `.3c` (relational-vs-value + descriptive frames) can now
  proceed against this measured target.
- `2026-06-10`: `.0` DONE — the gauge is now a STANDING persisted measurement: `nli-verify`
  back-annotates `extraction_quality_gauge` onto the EvidenceIR (never persisting a vacuous
  all-abstained pass), `converge` re-measures it after stabilization via the shared
  `measure_and_persist_gauge` and prints it in the convergence summary, and `validate` reports
  it provider-free (`extraction_quality_*` metrics with honest `n/a`, Info finding carrying the
  not-entailed ids, scale-free majority-erroneous Warning, staleness Warning on surface change).
  Live: APB 28.6% Info-only vs AHB 60% / CHI 69% / AXI 91% majority-flagged — the canonical
  Pattern surfaces' quality is now visible (the cleaned LLM-primary surfaces were never promoted
  off the /tmp measurement copies); I2S converge demo took the measurement end-to-end.
  kg-bench 154/154; lib 1537. See [[extraction-quality-gauge-standing]]. The original `.0`–`.4`
  backlog is now fully closed; remaining open leaf = `.3c` (relational-vs-value + descriptive
  frames; needs a measurable target — CHI re-ingest or a probed corpus example).
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
- `2026-06-10`: `.FIELD.4` DONE — field obligations are CAPTURED, not dropped: the parallel
  `message_field_constraints` surface (decision: separate surface over a subject-kind
  discriminator — downstream `signal_constraints` consumers keep seeing wires only, by
  construction), `ground_constraint_typed` routing field-typed subjects through the SAME
  grounding gates, provenance-merging dedup generalized over both surfaces. Measured live on
  persisted CHI (catalog injected from the real `.FIELD.2` extractor): signal surface = exactly
  the 4 flit-valid wires; `TagOp`/`PBHA` content rules = 2 field records with channel containers
  and merged provenance; wire controls APB/AHB/AXI at exact prior volumes, zero field
  constraints, P=R=F1=1.000 ×3. Residuals: field-presence kind (MPAM "must be included" —
  probed `[]`), full fresh-yield CHI re-measure (needs re-ingest), validate integration
  deferred. See [[message-field-constraints-surface]].
- `2026-06-10`: `.FIELD.3` DONE — `EntityType::Field` grounded on the `message_field_records`
  catalog: a declared field types as `Field` deterministically (no LLM call) and is rejected as a
  signal-constraint subject; signal-table declarations outrank; the conflated "wire/pin/field"
  prompt is split. Probed live pre-ship: entity-judgment controls unchanged
  (TXSACTIVE/LICENSEE/CMO), undeclared-field boundary honestly characterized (phrasing-dependent —
  why the catalog, not the model, carries the ontology). lib 1527; kg-bench 153/153. Remaining:
  `.FIELD.4` field-scoped constraints + the CHI-class gauge re-measure.
- `2026-06-10`: `.FIELD.2` DONE — `message_field_records` is live: packet/flit protocols' declared
  message fields now have a first-class typed home (CHI 106 / C2C up to 189 with real widths /
  CCIX ~50 — measured with the real extractor over the persisted corpus), the register surface
  keeps priority over shared `Field` columns (12-doc stash-diff byte-identical except the additive
  manifest entry), and the gold/negative fixture pair locks both directions (kg-bench 153/153,
  lib 1524). Next: `.FIELD.3` grounds `EntityType::Field` on this catalog.
- `2026-06-10`: `.FIELD.1` DONE (design + corpus probe, docs-only) — the signal-vs-field ontology is
  grounded in the documents' own table-header vocabulary: field-titled name columns declare fields
  (CHI: 36 tables / 79 names incl. the `DBID`/`TxnID` mis-typing class; CHI-C2C carries widths),
  `Signal`-titled tables declare signals; register-field tables are discriminated structurally by
  register-access columns (`Access`/`Reset`/`Default`), no name lists (ADR 0006). CHI's field
  tables are currently `table_kind: unknown` → inert, so fields have no typed home — and
  `entity_prompt` itself conflates field into signal ("a wire/pin/field"). Split: `.FIELD.2`
  capture surface → `.FIELD.3` entity-type grounding → `.FIELD.4` field-scoped constraints +
  CHI-class re-measure. CCIX/OpenCAPI field shapes are honest later strategies. See
  [[packet-field-table-declaration]].
