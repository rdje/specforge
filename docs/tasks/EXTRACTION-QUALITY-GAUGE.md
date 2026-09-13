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
  `.3k.1`–`.3k.6` with `.3k.2a`–`.3k.2j`)
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
  split** `2026-09-12`) · Children: `.3k.1`–`.3k.6` (`.3k.2` carries `.3k.2a`–`.3k.2j`) · Goal: **every part of a
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
  **Ordering rationale.** `.3k.1` is strictly subtractive and landed first. `.3k.2` must land before
  `.3k.3`, because narrowing the kind's span moves 3 of its 4 records onto the ungated `generic_value`
  arm (NVMe would publish `ANAGRPID must_be_value UNIQUE`, a value lifted off the adjective following
  `shall be`); fixing the span before the arm would trade one fabricated fact for another. `.3k.6`
  (opened by `.3k.1`) should land before `.3k.2` is sized, for the reason `.3k.1` discovered.
  **Amendment (`2026-09-12`, from `.3k.1`) — every population in this node is a PUBLISHED population,
  not an actionable one.** Only 24 of the 78 documents keep a normalized bundle, so the other 54
  evidence artifacts are frozen at whatever generation wrote them and can carry records the current
  extractor would not mint. `.3k.1`'s four DTI records turned out to be exactly that: published, and
  reproducible by nothing. Every remaining child must re-derive its population by running the real
  producer on each record's own `source_text` before sizing its change
  (`[[persisted-census-measures-published-not-current]]`); `.3k.6` shipped the instrument that makes
  that mechanical, and its corpus answer is **144 of 179 reproduce, 35 do not** — so roughly one
  published deterministic constraint in five is not what this code would produce today.
  **Amendment (`2026-09-13`, from `.3k.2e`-`.3k.2j`) — the ROW path's populations are bounded by what its
  producer can SEE, and that is 27 of 78 documents.** `extract_signal_description_row_constraints` selects
  tables by `TableKind::SignalDescription`, and a legacy `SourceIr` is loaded with every classification
  neutralized to `Unknown`, so 51 documents are invisible to it
  (`[[legacy-source-classifications-are-neutralized-on-load]]`). Measured over the 27 it can see and the
  102 tables that pass its own gate: it mints **12 records, all already published**, and `.3k.2e`,
  `.3k.2f`, `.3k.2h` and `.3k.2i` each measure an actionable population of **zero**. Two of them shipped
  anyway, on `.3k.1`'s footing, because their class is demonstrable through the real reader; two did not,
  because theirs is not. **The remaining row-path work is therefore blocked on re-ingest rather than on
  analysis**, and the measurable frontier moves back to the statement and dynamic paths — `.3k.3`,
  `.3k.4`, `.3k.5` — which see all 78.
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

- ID: `EXTRACTION-QUALITY-GAUGE.3k.1` · Status: `done` (`2026-09-12`, CODE) · Goal: **refuse a
  comparative MAGNITUDE whose right operand is a REFERENCE.** `.3d` already refuses an inter-operand
  EQUALITY (*"X must be equal to the value of Y"*) because the constraint vocabulary has no slot for
  it; *"must not be greater than the size indicated by the OAS field"* is the same shape one relation
  along, and the vocabulary has no slot for it either. A magnitude against a LITERAL (*"must be
  greater than 0"*) must stay untouched — it is a value binding, and `.3d`'s own line between "the
  value of <other>" and a literal is the line this reuses.
  **The population statement this leaf opened with was wrong, and finding out why is its main
  result.** It said *"4 records, all `sigcon_*`, all DTI"*. Those four records are published, but
  **today's extractor reproduces none of them**: every candidate subject in DTI's sentence is named
  only AFTER the obligation's lead, so `CORPUS-COVERAGE.2.50a`'s pre-lead subject authority
  (`is_post_passive_binding_only_subject`) reaches it first and `extract_signal_constraints` returns
  an empty vector. Verified by running the real producer on the live sentence with `OAS`/`DTI`
  declared — `records=[]`, `post_passive OAS=true DTI=true`. The four records predate that gate and
  the document has no retained normalized bundle, so the artifact is frozen where it is.
  **The CLASS is nevertheless live, and that is why this shipped rather than closing as covered.**
  The same grammar with the constrained signal named BEFORE the lead still mints the fabricated pair:
  *"ZETARANGE must not be greater than the size indicated by the ZETAOAS field"* → `MustBeStable` +
  `negated: true`, i.e. **"ZETARANGE must not be stable"** — exactly what DTI published. The dynamic
  path is reachable too: *"The controller drives ZETARANGE LOW whenever the requested span is larger
  than the number of entries the ZETAOAS field reports"* → `ZETARANGE must_be_low` AND
  `ZETAOAS must_be_low`, the right operand minted as a second subject.
  Shipped: pure `is_reference_magnitude_constraint` — a comparative marker IMMEDIATELY followed by a
  phrase naming another operand's attribute — wired beside `.3d`'s refusal in BOTH deterministic
  paths. +6 tests, `specforge-core` lib 1,435 → 1,440.
  **Amended `2026-09-13` by `.3k.2k`: one of this leaf's controls pinned a fabrication.**
  `a_magnitude_against_a_literal_still_yields_its_constraint` asserted that *"The value of ZETARANGE
  must be greater than 0"* still yields a record and called that record a value binding. It is
  `ZETARANGE must_be_value GREATER` — the comparative in the value slot, with the literal `0` the
  sentence names nowhere in it. The line this leaf drew is real and stands: a LITERAL right operand is
  not a REFERENCE magnitude and `is_reference_magnitude_constraint` correctly returns false for it. But
  that is a difference between two REFUSALS, not between a refusal and a capture — the vocabulary has
  no `at least` kind, so the literal shape ends as a residual too, reached by `.3k.2k`'s value-slot
  rule instead. The control now pins this leaf's own gate verdict, which is what it was reaching for.
  **Honest limits, both named rather than absorbed:** (a) the four DTI records stay in the persisted
  artifact until that document is re-ingested, exactly as `.3h`'s NVMe `FFFF` record does; (b) the
  lead list is the measured one — extending it with `that supported by` / `the maximum` / `the
  minimum` would refuse RISC-V IOMMU `dyn_sigcon_0008`/`0009`, whose own obligation clause states no
  relation at all, which would be right by accident and is owned by `.3k.5`.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.1`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.5` · Status: `done` (`2026-09-13`, CODE; opened `2026-09-12` by
  `.3k.1`) · Goal:
  **the refusal gates are statement-scoped while the records they suppress are clause-scoped.**
  `is_relational_equality_constraint` (`.3d`) and `is_reference_magnitude_constraint` (`.3k.1`) are
  both evaluated over the WHOLE statement, so a relation stated in one sentence refuses an obligation
  minted from another — the same span defect this container is about, one level up, on the refusal
  side. **Measured, read-only over 261,508 persisted statements:** the equality phrase appears in 181
  statements and lies OUTSIDE the obligation clause in **4** of them; clause-scoping would admit
  those 4, which recovers one real constraint (NVMe `statement_4474`, *"all bytes of this field shall
  be cleared to 0h"*, currently refused because the cell's descriptive body says *"contains the same
  value as reported in …"*) and exposes one fabricated one (NVMe `statement_5826`, *"The Port
  Identifier … shall be unique"* → `must_be_value UNIQUE`, which is `.3k.2`'s class). Two more are
  unadjudicated. The magnitude leads this leaf owns (`that supported by`, `the maximum`, `the
  minimum`) can only be added once the span is decided: statement-scoped they refuse RISC-V IOMMU
  `dyn_sigcon_0008`/`0009` for a relation in a later sentence. **SHIPPED `2026-09-13` — the SPAN. The LEADS are deliberately NOT shipped, and
  finding out why is this leaf's second result.** Both refusals now read the OBLIGATION in the
  statement path and the BINDING in the dynamic path; the row reader has read them that way since
  `.3k.2f`, so all three producers finally agree. **Corpus: replayed 302 → 302 and reproduced 127 →
  127 across all 77 loadable documents — nothing moves, nothing is rebuilt.**
  **The four statements this leaf was sized from no longer describe the code.** They were counted with
  a Python mirror in `.3k.1`'s day; since then `.3k.3` made the statement path read per obligation and
  `.3k.2k` refused a relational predicate in the value slot. The one record the node expected to
  RECOVER (NVMe `statement_4474`) is not recovered, and the reason is worth keeping: it is refused by
  `EXTRACTION-QUALITY-GAUGE.3e`'s descriptive-field-cell gate — a THIRD statement-scoped gate this
  leaf does not own. The one it expected to EXPOSE (NVMe `statement_5826` → `must_be_value UNIQUE`)
  cannot appear, because `.3k.2k` refuses it. **The prerequisite did its job in a way the node could
  not have predicted: it emptied the admitted set instead of cleaning it.**
  **The leads (`that supported by`, `the maximum`, `the minimum`) are unblocked and still unshipped.**
  `.3k.1` could not add them because statement-scoped they refused RISC-V IOMMU `dyn_sigcon_0008`/
  `0009` for a relation in a later sentence; clause-scoped that objection is gone, and the shapes are
  live — `the maximum` follows a comparative **28** times in the corpus, `the minimum` **17**, `that
  supported by` **6**, in sentences like *"a TID value that is greater than the maximum supported
  TID"*. They are not added because **nothing reaches them**: in the statement path `.3k.2a` refuses a
  clause that types no kind and `.3k.2k` refuses a value slot holding the comparative itself, so every
  candidate is gone before this gate is asked, and I could not construct a reachable case. Adding
  vocabulary no case exercises is what this family refuses to do. The reason is written into the
  gate's own doc comment so the next reader inherits the measurement rather than the plan.
  Prerequisite: `.3k.2` (so the admitted-set is not a fabrication set) — **satisfied**. Verification:
  see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.5`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.6` · Status: `done` (`2026-09-12`, CODE) · Goal: **an instrument
  that answers "does today's extractor still produce this persisted record".** `.3k` sized its
  children from the persisted corpus and `.3k.1` then discovered the corpus is not one code
  generation: only 24 of 78 documents keep a normalized bundle (plus APB/AHB/AXI held out under
  `generated/preserved/WIRE-BASED-100.10/`), so the other 54 artifacts are frozen at whatever
  generation wrote them. Hand-writing a unit test per record does not scale to `.3k.2`'s population.
  **Shipped: `specforge replay-constraints <evidence-ir>` / `--evidence-root <root>`.** It re-runs the
  REAL producer (`extract_normative_signal_constraints`) over an artifact's own `extracted_statements`
  and compares by the producer's own merge identity — subject, kind, value, condition, negation and
  source text, never the ids. It works for the frozen 54 because the deterministic constraint surface
  is a function of the STATEMENTS, not of the PDF, and it reads the legacy/proofless stratum through
  `load_for_inspection` rather than the canonical loader that refuses it.
  **The corpus answer: 144 of 179 published deterministic records still reproduce; 35 do not.** By
  cause: 17 have no positional gate against them (their kind, condition or negation moved), 12 are
  refused by `CORPUS-COVERAGE.2.50a`, 4 by `.3k.1`+`.2.50a` together (the DTI class), 1 by `.3h`, 1 by
  `.3g`. Ten documents are partial and one — AMBA CXS, 0/2 — is fully frozen. **One artifact is a
  named skip**, not a silent omission: I2C is current-schema with a stale proof, which
  `load_for_inspection` still verifies.
  **Two properties make the verdict usable.** (a) A published subject the artifact's statements no
  longer declare is GRANTED a synthetic `Signal <name> is …` declaration, so "not reproduced" can
  never quietly mean "the catalog shrank" — 67 subjects corpus-wide needed one, which is itself a
  finding. (b) The verdict is ASYMMETRIC and the command says so: "not reproduced" is sound because a
  widened catalog can only admit more subjects, while `unpersisted_replay_records` is not a drift
  measure, because the build applies convergence stages the replay does not.
  **The first version of the catalog widening was wrong and a control caught it**: it inserted names
  into the `HashSet` passed to `extract_normative_signal_constraints`, which only the
  inference-antecedent sibling reads — both deterministic paths derive their own catalog from the
  statements. The corpus figure moved 120/179 → 144/179 once the widening was done with declaration
  STATEMENTS instead. That is the same failure shape as the rest of this family, caught this time by a
  control written before the number was published.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.6`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2` · Status: `active` (split `2026-09-12` after `.3k.6` re-sized it;
  re-split `2026-09-12` by `.3k.2d`'s census) · Children: `.3k.2a`-`.3k.2j` · Goal: **what a clause that types nothing may
  publish.**
  Two arms of `classify_signal_constraint_kind` emit a fact the document did not state: the terminal
  `untyped_default` publishes `MustBeStable` for any obligation no phrase matched, and the
  `generic_value` arm lifts whatever word follows `must be `/`shall be ` as a typed value with no gate
  at all. Re-sized with `replay-constraints` before splitting, exactly as the container's amendment
  requires: the published counts were 26 and 11; the REPRODUCED counts — the population a change can
  move — are **17 and 4**. Reading all 21 separates them cleanly, and the split follows the reading
  rather than the arm: **17 of 17 untyped-default records are wrong** and the arm must be refused,
  while **2 of 4 `generic_value` records are right** (`AWTAGOP must be Invalid`, `CKE must be held
  LOW`) so that arm needs a GATE, not a refusal. A third finding came out of the same reading: the
  ROW path's 4 untyped-default records are *correct* and merely under-typed, which is why the refusal
  is asymmetric.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2a` · Status: `done` (`2026-09-12`, CODE) · Goal: **the terminal
  `MustBeStable` is not a default, it is a fabrication — refuse it in the statement path.**
  **All 17 reproduced records adjudicated, 17 of 17 wrong:** two CoreSight `APB must_be_stable` from a
  barrier-transaction description; four AXI records from `| Manager: False | ARCHUNKEN is not present.
  …` (a presence cell); one from `Manager RCHUNKV input is tied low` (a level, not stability); two APB
  `PSLVERR` from *"It is recommended, but not required…"* and *"Completers are not required to
  support PSLVERR"*; one AHB `HRESP` from a two-cycle response description; six waveform narrations
  (`- T1 FREADY signal remains HIGH`, `At T3 … QDENY remains LOW`, `AERR is driven HIGH for 1 tCK`).
  Not one of them says anything about stability.
  **The asymmetry is the design.** The refusal is in the STATEMENT path only. The row path
  (`extract_signal_description_row_constraints`) keeps the fallback because it has already proved,
  via `obligation_subject`, that its clause binds to its row's own signal — so an untyped obligation
  there is a real obligation with a spelling the table lacks. Its 4 records are
  `PAUSER`/`PWUSER must have the same value in the Setup and Access phase`, which IS a stability
  obligation; typing it properly is `.3k.2c`.
  Shipped: `classify_signal_constraint_kind_typed`, which returns `None` exactly when the terminal arm
  is reached with no stability/validity phrase anywhere in the clause; `extract_signal_constraints`
  refuses on `None`. `classify_signal_constraint_kind` is unchanged, so the row path is untouched.
  **Superseded in part (`2026-09-13`, by `.3k.2e`): the asymmetry is gone.** It rested on the four APB
  `must have the same value` clauses, which `.3k.2c` then typed, so nothing correct reaches the row
  path's fallback any more and every producer goes through the typed gateway. The reasoning stands for
  its own population; the population is what changed.
  **Chain rebuilt for all three documents whose artifacts move — APB, AHB and AXI-L — because the
  change stales their proofs and all three have a held-out bundle.** `evidence → validate → semantic →
  validate → intent → validate → adapt` each: **8 records removed, 0 added, 0 retyped** (AXI 45→40,
  APB 25→23, AHB 14→13); every removal is one of the 17. The other 9 stay in documents with no bundle
  to rebuild from, and `replay-constraints` now reports them as not-reproduced rather than hiding them.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2a`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2b` · Status: `done` (`2026-09-12`, CODE) · Goal: **a passive
  obligation puts its VERB in the value slot.** `extract_protocol_state_value` lifts the first
  non-filler word after `must be `/`shall be `/`must remain `/`shall remain ` and says nothing about
  what it is. **Population: 4 reproduced `sigcon_*`, and it is 2-2** — right: AXI
  `AWTAGOP must_be_value INVALID` from *"AWTAGOP must be Invalid"* (an adjective) and HBM2
  `CKE must_be_value LOW` from *"CKE must be held LOW"* (a logic level); wrong: RISC-V IOMMU
  `GSCID` and DTI `DO_NOT_CACHE`, both `INVALIDATED` lifted out of *"must be invalidated"* — a past
  participle, which is what happens TO the thing, not what it equals.
  **The first design was too wide, was measured, and was reverted before it shipped.** Gating every
  value arm against the document's discovered enum values retyped **13 correct APB and 4 correct AHB
  records** from `must_be_value VALID` to `must_be_stable`: the `must be valid` arm's value is the
  validity CONVENTION `.8` established, not a word admitted on position, and `VALID` is in no
  document's enum set. The validity arm is therefore explicitly not gated, and the code says why.
  Gating only the generic arm against discovered values then still cost the one correct record whose
  enum table the discovery pass does not read (`AWTAGOP … Invalid`).
  **What shipped is exactly as wide as the evidence**: `is_admissible_state_value` admits a value the
  document declares, a logic level, or a numeric literal, and otherwise refuses only the PAST
  PARTICIPLE shape. So `Invalid` binds, `INVALIDATED` and `UPDATED` do not, and a participle-shaped
  enum member the specification does declare (`Shared`) is admitted through the document route — the
  pair that makes the override meaningful rather than decorative.
  **Measured effect: 2 fabricated records refused, 0 correct records lost, 0 artifacts changed.**
  Both instances live in documents with no normalized bundle, so `replay-constraints` reports them as
  not-reproduced (125 of 171, up from 127 of 171 by exactly these two) and nothing in `generated/`
  moves — the leaf is its own illustration of why the published and actionable populations differ.
  **Residual named, not absorbed:** AXI `WTAGUPDATE must_be_value UPDATED` survives, because its cell
  reaches the UNGATED validity arm on a `must be valid` later in the same cell while
  `extract_protocol_state_value` binds from the first `must be ` in the text. That is a span defect,
  and it belongs to `.3k.3`.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2b`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2d` · Status: `done` (`2026-09-12`, CODE) · Goal: **one modal
  vocabulary for the whole record.** Three functions read an obligation clause for its modal and each
  carried its own vocabulary: `obligation_is_negated` accepts `must not`/`shall not`/`must never`/
  `shall never`/`cannot`/`will not`; `constraint_bearing_sentence` looked only for `must`/`shall`; and
  every phrase in `classify_signal_constraint_kind` is spelled `must`/`shall`. So an obligation a
  document states with `cannot` was flagged NEGATED by the first, given no sentence of its own by the
  second, and typed as nothing by the third. This is the remaining half of `.3i`'s second finding
  (*"every phrase in the table is affirmative"*) one level up: every phrase in the table is also
  MODAL-specific, and `.3i` answered that with three more literals rather than with the rule.
  **The leaf opened on a different mechanism and its own census refuted it.** It said
  `extract_protocol_state_value` has no negated binder, citing *"The DV operand must not be 1 for
  IODIR"*. That sentence is a TEST STRING, not a corpus record: the document's own statement is
  RISC-V IOMMU `statement_1033`, class `conditional_rule`, and its records are `dyn_sigcon_0008`/
  `0009` — the DYNAMIC path, which never reaches this classifier at all. The rule was written from a
  description of the producer instead of from the producer (`CLAIM_VERIFICATION.md` §3 Leg 2).
  **The census, over all 78 persisted artifacts, enumerating the full cross product of the six modals
  `obligation_is_negated` accepts with the binder verbs `extract_protocol_state_value` reads.**
  Statement path: **31** `signal_value_constraint` statements carry a negated binder — 13 reach
  `must_not_change`, 6 reach `must_be_deasserted`, **12 reach the generic/untyped arm**. All 12 read
  against source: **9** are *"cannot be changed"* / *"will not be changed"* (eMMC ×2, USB4 ×5, SMBus,
  Wishbone) — a no-change obligation whose modal the table lacks; **2** are DTI reference magnitudes,
  correctly refused by `.3k.1`; **1** is AMBA LPI *"QREQn cannot be driven HIGH until the handshake is
  completed"* — a level obligation the table ALSO already owns (`must be driven high`), blocked by the
  same modal. Row path: 50 admitted obligation clauses, **0** carrying one of these modals.
  **So the value binder's own population is zero.** Not one of the 12 needs a negated binder; ten of
  them need the modal, and the vocabulary slot `.3k.2b` named (`MustBeValue` + `negated`) is reached
  by no corpus clause in either caller. The gap is real as a capability and empty as a population.
  Shipped: `normalize_obligation_modal` reduces the equivalent negative modals to the `must not` form
  the table is written in, and `sentence_states_an_obligation` gives the obligation-sentence scan that
  same vocabulary. **Measured over all 78 artifacts, record by record: 0 added, 0 removed, 0 retyped**
  — `replay-constraints` is identical before and after (171 persisted / 125 reproduced / 46 not / 126
  unpersisted), and APB rebuilt to the same 23 records with the same ids.
  **The zero is the result, not the absence of one, and the intermediate measurement proves the two
  halves cannot ship apart.** With only the classifier half, the replay gains exactly 2 records —
  eMMC `NOTE must_not_change` twice, from `| NOTE 1 | … A Device … will not change its state to the
  rcv state. … |`, whose subject is the serialized row's own NOTE marker. Teaching the classifier a
  modal the sentence scan cannot find moves the record's span to the whole row; teaching both leaves
  the obligation with a sentence of its own, and the marker is not in it.
  **Four findings routed rather than absorbed:** `.3k.2e` (the row path's untyped fallback), `.3k.2f`
  (the row path applies neither vocabulary-slot refusal), `.3k.2g` (`replay-constraints` does not judge
  the `row_sigcon_*` stratum at all), `.3k.2h` (`obligation_subject` is the next function still reading
  `must`/`shall` only, and a continuation row's empty name cell drops every obligation in it), `.3k.2i`
  (the row path's generic-value arm).
  **Correction (`2026-09-13`, by `.3k.2g`) — the row-path populations this leaf routed were measured
  with a MIRROR and three of them are withdrawn.** The census selected tables on the persisted
  `table_kind`; the producer selects on the loaded one, and a legacy artifact has every classification
  neutralized to `Unknown`. Re-derived with the real producer, the row path mints 12 records over the
  whole corpus, all already published: `.3k.2e`'s "17 of 17 wrong" is **0 actionable**, and `.3k.2i`'s
  "4 right / 5 wrong" is likewise unmeasurable rather than current. Each node carries its own
  re-derivation; the readings stand as estimates for the 51 legacy documents, never as counts.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2d`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2e` · Status: `done` (`2026-09-13`, CODE) · Goal: **proving the
  SUBJECT does not make the KIND readable — the row path's untyped fallback is refused too.**
  `.3k.2a` refused the terminal `MustBeStable` in the statement path and kept it in the table-row
  reader, on the reasoning that this reader has already proved via `obligation_subject` that its clause
  binds to its row's own signal, so an untyped obligation there must be a real obligation with a
  spelling the table lacks. **That reasoning was sound for the population it was made about and is no
  longer about any population.** It rested on four APB `must have the same value` clauses; `.3k.2c`
  gave the table that spelling and they stopped reaching the fallback at all.
  What reaches it now is a different thing: a clause whose obligation the constraint vocabulary cannot
  express. Both callers of the kind classifier now go through
  `classify_signal_constraint_kind_typed`, so **the terminal arm is unreachable as a published kind by
  any producer**, and `scripts/measure_constraint_part_span.py --check` pins that as its own invariant
  alongside the two producers' spans.
  **Measured effect: zero, and it is stated as a class rather than a count.** `replay-constraints` with
  the row stratum judged is unchanged at 183 persisted / 137 reproduced / 46 not-reproduced; all 12
  published `row_sigcon_*` records are typed by an arm the document wrote, so the refusal costs nothing
  measurable. The class is demonstrated through the REAL row reader: a MATCH (*"ZETAREADY must match
  OMEGABURST"*), an ALIGNMENT (*"Must be aligned to a burst size"*) and a PRESENCE claim (*"Must not be
  present"*) each publish `must_be_stable` without the refusal — the last one negated, i.e. *"SIGMASTRB
  must not be stable"* — and all three vanish with it. The over-kill guard is the half that decides
  whether the refusal is safe, and it is asserted as the four shapes the live records are made of:
  header-supplied validity, `.3k.2c`'s no-change spelling, a negative polarity form, and a plain value
  binding all still publish.
  **`.3k.2d`'s "17 of 17 wrong" stays withdrawn** (`.3k.2g`): that census selected tables on the
  persisted `table_kind` and the producer selects on the loaded one. The reading stands as an estimate
  for the 51 documents whose classifications are neutralized on load
  (`[[legacy-source-classifications-are-neutralized-on-load]]`), never as a current count.
  **Found in passing, and owned rather than noted:** `scripts/measure_constraint_part_span.py --check`
  — `.3k`'s own declared verification — has been RED since `.3k.2a`, and `.3k.2a`/`.3k.2b`/`.3k.2c`
  each shipped over it because nothing runs it. Re-derived per revision with each revision's own
  scanner, not by reading. `.3k.2j` owns wiring it into a driver.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2e`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2j` · Status: `done` (`2026-09-13`, DOCTRINE) · Goal: **a fail-closed
  check that no driver runs is not a check — writing one is not adopting it.**
  `scripts/measure_constraint_part_span.py` was built by `.3k` precisely so that a change to the kind
  classifier's call-site topology would fail closed rather than silently restratify the census that
  sized this whole container, and `.3k` names `--check` and `--self-test` in its verification. It was in
  no driver: not `scripts/check_doctrines.sh`, not `scripts/run_ci.sh`, not `.githooks/`.
  **Attributed by re-derivation, not by reading.** Running each revision's own scanner over that
  revision's own `evidence.rs` (`git show <rev>:<path>`): green at `.3k.1`; RED from `.3k.2a`, which
  introduced `classify_signal_constraint_kind_typed` so the bare classifier's only production caller
  became the wrapper; still RED at `.3k.2c`. Three leaves shipped over it, each reporting a fully green
  doctrine gate — because the gate never included it.
  Shipped: `scripts/check_constraint_part_span.sh` (self-test then check, the order `CORPUS-FRONTIER`
  states — a gate is not trusted on a day its own negatives have not been re-proven), registered
  `CONSTRAINT-PART-SPAN|gate` with its `DOCTRINE_ENFORCEMENT.md` §10 row. Gate tier is earned rather
  than assumed: `--check` is read-only, offline and sub-second, and touches the persisted corpus only
  on the census path, never on `--check`.
  **The set claim, with its enumeration, because "nothing else is unrun" is refuted by one
  counterexample.** Of **86** script-shaped files under `scripts/`, `knowledge-map/scripts/` and
  `tools/`, **28** are unreachable from any driver, git hook, CI workflow, or doctrine registry — and a
  raw reachability count is a population, not a defect count, so it is classified before it is
  published. Exactly **2** of the 28 offer a `--check` mode, i.e. claim gate semantics while nothing
  executes them: this leaf's subject, and `scripts/validate_canonical_recovery_contract.py`. The second
  is **already adjudicated here** — `RETAINED-BUNDLE-POPULATION-FROZEN` carries *"Does
  `scripts/validate_canonical_recovery_contract.py` still have an owner?"* as an open question — so it
  is named and left to its owner rather than re-opened. The remaining 26 are one-shot measurement probes
  (`measure_*.py`, each of which produced a number for one leaf) and PDF utilities, none of which claims
  a gate mode. The first pass of this census said 33 and was wrong: it missed that a doctrine registry
  invokes a verifier by `argv` rather than by a shell reference, which is the same
  derive-from-the-producer failure this container keeps finding.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2j`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2f` · Status: `done` (`2026-09-13`, CODE) · Goal: **a refusal that
  says the constraint VOCABULARY has no slot for what a clause states belongs to the rule, not to one
  caller.** `.3d`'s `is_relational_equality_constraint` and `.3k.1`'s `is_reference_magnitude_constraint`
  make the same argument — *the vocabulary can say "this signal must be `HIGH`" and cannot say "this
  operand is bounded by that one", so refuse rather than fabricate* — and both were evaluated only
  inside `extract_signal_constraints`. The table-row reader called neither, so one sentence was refused
  as a statement and published as a row.
  Shipped: both predicates evaluated over the CLAUSE the row reader is already holding, which is this
  producer's own unit and is also the scope `.3k.5` is moving the statement path's copies toward.
  **Measured corpus effect: zero. That is the honest result and it is stated as a class, not as a
  count.** With the row stratum judged (`.3k.2g`), `replay-constraints` is unchanged at 183 persisted /
  137 reproduced / 46 not-reproduced, and no admitted row clause in the 26 judgeable documents matches
  either predicate. The leaf ships on `.3k.1`'s footing — a live, demonstrable class with an empty
  published population — and the demonstration is the observed-RED control, run through the REAL row
  reader rather than a mirror:
  *"ZETARANGE must not be greater than the size indicated by the ZETAOAS field"* → `ZETARANGE
  must_be_stable, negated: true`, i.e. **"ZETARANGE must not be stable"** — the exact AMBA DTI
  fabrication `.3k.1` closed in the statement path — and *"OMEGABURST must be equal to the value of
  ZETAREADY"* → `OMEGABURST must_be_value VALUE`, the value lifted out of the phrase *the value of*.
  Both vanish with the refusal wired and both return when it is removed.
  **Honest limit, inherited not absorbed:** 51 of 78 documents have their table classifications
  neutralized on load (`[[legacy-source-classifications-are-neutralized-on-load]]`), so "no admitted row
  clause matches" is a statement about the 26 the producer can see. It is not a clean bill for the rest.
  Prerequisite: `.3k.2e` — **waived**, deliberately. `.3k.2e` refuses a clause whose kind is UNTYPED;
  this one refuses a clause whose kind types perfectly well and whose MEANING has no slot, which is why
  the `must_be_value VALUE` shape above is invisible to `.3k.2e` and survives it. The two are
  independent, and ordering them was an assumption `.3k.2g`'s re-sizing removed.
  Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2f`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2g` · Status: `done` (`2026-09-13`, CODE) · Goal: **the replay
  instrument judged two of the three deterministic producers and said nothing about the third.**
  `.3k.6` shipped `replay-constraints` so a change could be sized against what today's extractor
  actually mints, and `.3k.2a`–`.3k.2d` each used it — while it filtered the judged set to `sigcon_*`
  and `dyn_sigcon_*`. So `.3k.2c` had to state in prose that its retype *"is inside the row stratum,
  which the replay does not judge"*, and `.3k.2d` had to restore a held-out bundle and rebuild a whole
  document chain to prove that same stratum unmoved. An instrument a family sizes itself with cannot
  have a producer-shaped hole in it.
  Shipped: `replay_persisted_signal_constraints` takes the document's own `SourceIr` — which the
  artifact already names, repository-root-relative — and composes the table-row pass exactly as the
  build does (append after the statement paths, refine polarity BEFORE the dedup, then dedup against
  the established count), with the build's own catalog (statement declarations ∪ the
  signal-description tables' names). `row_sigcon_*` joins the judged set when and only when that pass
  actually ran.
  **The second half had to be measured rather than assumed, and it is the leaf's main result.** A
  LEGACY `SourceIr` loads, and `neutralize_legacy_source_classifications` sets every `table_kind` to
  `Unknown` — correctly, because only the current schema plus a verified proof ledger carries
  classification authority. The row producer selects tables by `TableKind::SignalDescription`, so over
  such an artifact it selects NONE and returns an empty result **indistinguishable from "this document
  states no row obligation"**. Measured on AMBA LTI: its persisted SourceIR marks 25 tables
  `signal_description`, and after a legacy load **0 of its 88 tables pass the producer's own gate**.
  Reporting that as a judged stratum would publish a silent zero — the exact failure `.3k.6` exists to
  retire. `SourceIr::carries_canonical_source_classifications` gates it, and the report publishes both
  halves: **26 documents judged, 51 not** (51 legacy SourceIRs; one document is the named EvidenceIR
  skip).
  **Corpus result: 171 → 183 persisted deterministic records, 125 → 137 reproduced, `not_reproduced`
  unchanged at 46.** All 12 published `row_sigcon_*` records reproduce — which re-derives, without a
  rebuild, exactly what `.3k.2d` had to rebuild APB to establish, and extends it to AHB and AXI-L.
  **This leaf's first act was to falsify a population `.3k.2d` published one commit earlier.**
  `.3k.2e` was opened saying the row path's untyped fallback is *"17 of 17 wrong"*, from a Python
  census that selected tables on the persisted `table_kind` field. The real producer selects on the
  LOADED one, and over the 26 judgeable documents it mints **12 row records and not one more** — zero
  on the untyped fallback, zero unpersisted. `.3k.2e` is re-sized in place; see its node.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2g`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2h` · Status: `pending` (opened `2026-09-12` by `.3k.2d`) · Goal:
  **the row path's two remaining readers of a narrower vocabulary.** (a) `obligation_subject` decides
  whether a description clause states an obligation by looking for `must`/`shall` only, so the modal
  gap `.3k.2d` closed for the statement path is still open one function along: a row cell stating
  *"X cannot be asserted while Y is high"* is `NotAnObligation` and the clause is dropped. (b) A
  signal-description table split across pages emits continuation rows whose NAME cell is empty, and
  `resolve_declared_signal_identifier("")` fails, so every obligation in the continuation is dropped —
  AMBA LTI `table_0014` (*"Table B4.1 Continued from previous page"*) loses two, including
  *"When LAMMUV is 1 and LAPM is 1, LAFLOW must not be Stall"*, the corpus's only genuine NEGATED VALUE
  binding and therefore the only clause that would have given `.3k.2d`'s original mechanism a
  population. Both are recall, both are in the row reader, and (b) decides whether the vocabulary slot
  `MustBeValue` + `negated` is ever reached at all.
  **SIZED `2026-09-13` against the real gate, and BOTH halves measure zero — so this leaf is blocked on
  re-ingest, not on a decision.** Over all **27** current-schema documents and the **102** tables that
  pass `should_treat_table_as_top_level_signal_description`, the probe finds **0** rows with an empty
  name cell and **0** description clauses stating an obligation with `cannot`/`will not`/`never` and no
  `must`/`shall`. The LTI `table_0014` instance that motivated (b) is real and is in a document whose
  `SourceIr` is schema 1, so the row producer cannot see it at all
  (`[[legacy-source-classifications-are-neutralized-on-load]]`). Neither half is refuted; both are
  **unmeasurable** until re-ingest reaches those documents, and shipping either now would be a rule with
  no reachable instance and no demonstrated class — weaker footing than `.3k.1`'s, which had one.
  Prerequisite: the owning document's re-ingest (`CORPUS-CHAIN-CURRENCY`). Verification: re-size both
  halves with the same probe once the judged-document count moves; each half adjudicated; observed RED
  on a demonstrated instance, not an invented one.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2i` · Status: `pending` (opened `2026-09-12` by `.3k.2d`) · Goal:
  **the row path's generic-value arm, read.** The same census that sized `.3k.2e` found 9 admitted row
  clauses reaching the `generic_value` arm, read as 4 right / 5 wrong. **Both numbers are withdrawn as
  CURRENT by `.3k.2g`** for the same reason `.3k.2e`'s are: the census selected tables on the persisted
  `table_kind` and the producer selects on the loaded one, so none of the 9 is reachable today. They are
  the reading for 51 legacy documents whose classifications this build has not re-derived, and the leaf
  must re-derive its own population with `replay-constraints` (row stratum judged) before it ships. Right: `LAPM must be 0`,
  `LAPRIV must be 0`, `LRHWATTR must be 0`, `LRMECID must be 0`. Wrong, and each for its own reason:
  *"LAPAS must be Non-secure or Secure"* publishes `NON` — the value binder splits at the hyphen, and
  the clause is a DISJUNCTION the slot cannot hold either (×2); *"LRATTR must be Snoopable Write-Back"*
  publishes `SNOOPABLE`, the first word of a two-word value; *"One write response must be sent for each
  write command"* publishes `SENT`, a passive verb `.3k.2b`'s participle rule misses because `sent` does
  not end in `ed`; *"… LRATTR must match LAATTR, with the exception of the allocation hint which must be
  Allocate …"* publishes `ALLOCATE`, a value lifted from a different clause of the same sentence
  (`.3k.3`'s span defect, in the row path).
  **Re-derived `2026-09-13`: the row reader mints 12 records over the whole judgeable corpus and every
  one is already published, so this arm's actionable population is 0 as well.** The nine readings stand
  as the estimate for the 51 legacy documents, which is where all nine live; the leaf is blocked on the
  same re-ingest `.3k.2h` names, not on a decision.
  Prerequisite: the owning documents' re-ingest (`CORPUS-CHAIN-CURRENCY`). Verification: each admitted
  clause adjudicated; observed RED per rule; the corpus delta measured with the row stratum judged
  (`.3k.2g`).
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2c` · Status: `done` (`2026-09-12`, CODE) · Goal: **the spelling
  this corpus uses for a no-change obligation.** APB writes it as *"PAUSER must have the same value in
  the Setup and Access phase of a transfer"* and *"… in every cycle during the Access phase"*; the
  phrase table knew `must be stable`, `must remain stable`, `must hold` and nothing of this form, so
  4 `row_sigcon_*` records reached the untyped fallback and were published as `must_be_stable` by
  accident. They are the only reason `.3k.2a` had to leave the row path's fallback in place.
  **Typed as `MustNotChange`**, because that is what the sentence says: the value is the SAME across
  two phases, or across every cycle of one — it does not change. The table already reads *"must
  remain stable"* (the same obligation over time) that way.
  **The placement is the load-bearing decision, and it was measured.** A serialized signal-description
  cell routinely carries BOTH obligations — *"• PAUSER must be valid when PSELx is asserted. • PAUSER
  must have the same value …"* — and the first arm to match types the whole record. Ahead of the
  validity arm, this phrase retyped APB `sigcon_0009`/`0010` from `must_be_value VALID` to
  `must_not_change`, losing a fact the document states. Behind it, it fires exactly where nothing else
  matched. Both the phrase and its position carry their own RED control.
  **Measured: 4 records retyped, 0 added, 0 removed, one document** — APB `row_sigcon_0018`/`0019`/
  `0021`/`0022`, `must_be_stable` → `must_not_change`; `sigcon_0009`/`0010` keep `must_be_value VALID`.
  The 56 corpus statements containing *"have the same value"* are almost all descriptive
  (*"implementations that have the same value"*, *"It does not have the same value"*) and are untouched
  because the phrases carry their modal.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2c`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2k` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.3k.3`'s addition measurement) · Goal: **a predicate that states a RELATION is not a value.**
  `.3k.2b` asked the neighbouring question about the same slot — value or the obligation's VERB — and
  refused the passive participle. This is the other way the slot fills with something that is not a
  value: a predicate ADJECTIVE whose truth is not about the subject alone. NVMe states *"The ANA Group
  Identifier (ANAGRPID) for each ANA Group shall be unique within the NVM subsystem"*; the generic arm
  lifts `unique` and publishes `must_be_value UNIQUE`, and there is no state `UNIQUE` a signal equals.
  It is `.3d`'s and `.3k.1`'s class one relation along — an inter-operand EQUALITY and a comparative
  MAGNITUDE against a reference operand — and the constraint vocabulary has no slot for any of the
  three. An honest residual keeps the statement counted as an uncaptured normative statement; a
  fabricated value does not.
  **Why it is opened here rather than inside `.3k.3`, and why now.** The container's ordering rationale
  says `.3k.2` lands before `.3k.3` *"because narrowing the kind's span moves 3 of its 4 records onto
  the ungated `generic_value` arm (NVMe would publish `ANAGRPID must_be_value UNIQUE`)"*. `.3k.3`'s
  addition measurement ran the real producer under the narrowed span and that prediction came true
  exactly: NVMe gains `ANA must_be_value` and `ANAGRPID must_be_value`. `.3k.2b` shipped the gate that
  was supposed to stop this and its rule — refuse a PAST PARTICIPLE — does not reach an adjective. So
  the arm is not finished, and `.3k.3` cannot land on top of it.
  **Actionable population today: ZERO, and the leaf says so rather than implying coverage.** Over all
  78 persisted artifacts, no judged `must_be_value` record carries a relational value: the census of
  every published deterministic `must_be_value` value is `0`/`1`/`5`/`12`/`0B01`/`0B11`/`VALID`/`LOW`/
  `NO`/`SET`/`PACKED`/`INVALID`/`INVALIDATED`/`UPDATED`. The class is nevertheless live and
  demonstrable through the REAL producer on a REAL corpus sentence, which is the `.3k.1` footing
  `.3k.2e`/`.3k.2f` also shipped on: hand `extract_normative_signal_constraints` NVMe
  `statement_7397`'s own first obligation clause and today's code returns `must_be_value UNIQUE`.
  **The discriminator is positional, not lexical (ADR 0006).** English marks the difference in the
  grammar rather than in the word: a state is complete at the predicate (`Invalid`, `LOW`, `0b01`),
  while a relation must name its second operand or its scope, and it does so with a preposition
  IMMEDIATELY after the predicate — `unique within <scope>`, `compatible with <other>`, `less than
  <other>`. No adjective list and no document vocabulary. `by` is deliberately NOT in the set: it
  marks an AGENT, not an operand, and the participles it follows are already `.3k.2b`'s.
  **Ordered after the three admissibility routes that `.3k.2b` measured**, so a document-declared
  value, a logic level and a numeric literal are untouched: the test can only ever fire in the final
  `everything else is admissible` branch `.3k.2b` left open.
  **It caught a fabrication one leaf back.** `.3k.1`'s control
  `a_magnitude_against_a_literal_still_yields_its_constraint` went RED, and it was right to: the record
  it pinned is `ZETARANGE must_be_value GREATER`, the comparative lifted into the value slot, with the
  literal `0` the sentence names nowhere in it. `.3k.1` described it as *"a value binding"*; it is not
  one, and the vocabulary has no `at least` kind either, so a magnitude against a literal has no more
  of a slot than a magnitude against a reference. The control now pins the property `.3k.1` actually
  owns — its own gate's verdict on a literal operand — and the amendment is written into `.3k.1`.
  **A control that asserts a record EXISTS pins whatever that record says**, fabrication included,
  which is how this one survived a leaf written to remove fabrications.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.2k`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.3` · Status: `done` (`2026-09-13`, CODE) · Goal: **the kind reads its own obligation
  clause** — the original `.3k` goal, at its true size, which turned out to be **one record per OBLIGATION**.
  `classify_signal_constraint_kind(&text.to_ascii_lowercase())` becomes
  `classify_signal_constraint_kind(&constraint_bearing_sentence(text).to_ascii_lowercase())` in
  `extract_signal_constraints`, joining the subject, the condition and (since `.3i`) the negation,
  which all already come from that span. **Population: 4 `sigcon_*` records.** AHB `sigcon_0002` is
  the mechanism `INVARIANT-SHAPE-ADMISSION.3` reported without explaining: its clause is *"When the
  Subordinate is initially selected, it must also monitor the status of HREADY…"*, which contains no
  kind phrase at all, and the published `must_be_asserted` comes from the NEXT sentence, *"HSELx must
  be asserted in the same cycle…"* — kind from one clause, condition from another. NVMe
  `sigcon_0005`/`0006`/`0007` take `must_not_change` from a sentence two clauses later whose own
  obligation is conditional on a capability bit.
  **Inherited from `.3k.2b` (`2026-09-12`) — the same defect in the VALUE slot.** AXI
  `WTAGUPDATE must_be_value UPDATED` survives every gate this family has built because its table cell
  matches the `must be valid` arm on a phrase LATER in the cell while `extract_protocol_state_value`
  binds from the FIRST `must be ` in the text, which is *"the tags in memory must be updated"*. The
  arm matched on one span and its value came from another, so narrowing the classifier's span fixes
  the value binder at the same time — re-measure the value slot here, not only the kind.
  **RE-DERIVED `2026-09-13` with `replay-constraints` (row stratum judged): the population is 3, not 4.**
  AHB `sigcon_0002` and NVMe `sigcon_0005`/`0006` still reproduce; NVMe `sigcon_0007` (`NVM
  must_not_change`) does not — `CORPUS-COVERAGE.2.50a` refuses its subject, which is a message-name
  fragment rather than a signal. Read against source, the other three are exactly this leaf's defect:
  NVMe's statement's first modal sentence is *"The ANA Group Identifier (ANAGRPID) … shall be unique
  within the NVM subsystem"* — uniqueness, not no-change — while the published `must_not_change` comes
  from the NEXT sentence, whose own obligation is conditional on a capability bit that the record drops;
  and two of its three subjects (`ANA`, `NVM`) are fragments.
  **A trap this leaf must not walk into, found while re-deriving.** AHB `sigcon_0002` is CORRECT today,
  by accident: its clause is the first modal sentence (*"When the Subordinate is initially selected, it
  must also monitor the status of HREADY…"*), which states no kind, and the published
  `HSELx must_be_asserted` is lifted from the THIRD sentence of the same serialized row, where the
  document does say it. Narrowing the classifier's span as this node describes would type that clause as
  nothing and `.3k.2a` would then refuse it — **losing a record the document supports**. So the span
  narrowing alone is not the fix. The real defect underneath is that `extract_signal_constraints` takes
  only the FIRST modal sentence of a statement and drops obligations 2..n, which is the collapse
  `INVARIANT-SHAPE-ADMISSION.3` named for the row path (*"a cell stating three obligations yields
  three — the serialized statement collapses them into one and keeps only the first"*). One record per
  clause, the way the row reader already works, is the shape to size — and that is recall, so it must be
  measured as an ADDITION before it is shipped as a narrowing.
  **ADDITION MEASURED `2026-09-13`, with the real producer, before any of it shipped** — the node asked
  for exactly this and it changed the design twice. A prototype was built (one record per obligation
  clause, every part read from that clause), `replay-constraints --json` was captured per document
  before and after, and the two runs were diffed over the **74 documents comparable in both** (AHB,
  AXI-L and APB-E drop out: the prototype moves their content, which stales their proofs and they then
  refuse to LOAD — the standing hazard, and its first useful use as a signal).
  **First result: +19 records, and −2, and the −2 are this leaf's own population.** NVMe `sigcon_0005`
  and `0006` stop reproducing, which is the fabrication this leaf exists to remove; **not one other
  persisted record is lost corpus-wide**. AHB `sigcon_0002` is not lost either — its third clause mints
  the same subject and kind with the CONDITION its own clause states, so the trap recorded above is
  avoided by reading every clause rather than by narrowing to the first.
  **Second result, and the reason the first design was wrong: the whole-statement subject FALLBACK is
  a cross-clause leak, and per-clause reading multiplies it.** When a clause's subject part yields no
  signal the reader scans the entire statement, so every clause's kind is handed the same
  statement-wide subject set. AXI `When the ACVALID signal is asserted the snoop address and control
  signals on ACADDR, ACPROT, and ACSNOOP must not change… When ACVALID is asserted, it must remain
  asserted until ACREADY is asserted` minted `ACADDR/ACPROT/ACSNOOP must_be_asserted`: the second
  clause's obligation is about ACVALID, its own subject is the pronoun `it`, and the statement-wide
  scan supplied the first clause's three signals. LTI minted `signal must_be_low` and
  `LAFLOW must_be_low` the same way.
  **Third result, and the missing piece: a FRONTED condition leaves the subject part empty.**
  `text_before_condition_marker` cuts at the EARLIEST marker, so `When <cond>, <subject> must <kind>`
  cuts at offset 0 — which is WHY the fallback fires so often, and why AHB `sigcon_0002` took its
  subject from a different sentence than its condition in the first place. The main clause of a
  fronted conditional begins after its comma. With that read and the fallback bounded by the
  obligation, the fabrications above disappear, `LAOGV must_be_low` replaces the fragment
  `LAOG must_be_low`, and the additions become **+22 on the same 74 documents**: AXI's five
  `*VALID must remain asserted` handshake invariants, AXI's four `AWSTASH* must be driven LOW`,
  LTI `LMOPENREQ must be asserted`, HBM2 `CKE must be driven LOW`, and the rest.
  **Fourth result — the container's ordering rationale is confirmed, not assumed, and it blocks.**
  NVMe's clause 1 (*"… shall be unique within the NVM subsystem"*) lands on the ungated `generic_value`
  arm exactly as `.3k` predicted, so the prototype trades two fabrications for two others.
  `EXTRACTION-QUALITY-GAUGE.3k.2k` was opened and landed for that, and this leaf resumes on top of it.
  **Two open questions for the implementation, both raised by the measurement and neither yet decided:**
  (a) APB's four `PAUSER`/`PWUSER must have the same value` clauses are then minted by BOTH the
  statement path and the row path, with the same subject/kind/condition and different `source_text`, so
  the merge key does not collapse them — decide whether that is a duplication to fix here or a
  cross-producer question of its own; (b) eMMC mints `PARTITION must_not_change` from
  `PARTITION\_ACCESS`, a fragment produced by the normalizer's escaped underscore, which is a
  tokenization defect rather than a span defect.
  **SHIPPED `2026-09-13`.** `constraint_bearing_sentences` yields every obligation clause in document
  order (the singular helper is now its first element, so the two cannot drift); the per-statement body
  became a per-obligation loop reading kind, value, negation, subject and condition from the ONE clause
  that mints the record; `obligation_subject_part` reads the main clause of a fronted conditional;
  the subject fallback is bounded by the obligation; and `is_post_passive_binding_only_subject_in`
  judges the obligation the record came from rather than the statement's FIRST one. Records restating
  one obligation dedupe on the producer's own merge identity. `source_text` deliberately stays the
  STATEMENT: it is what `supporting_statement_ids` cites and what the replay's merge identity keys on.
  **Measured on the three rebuilt documents, and the container's two inherited residuals both closed.**
  AHB 13 → 13 with `HSELx must_be_asserted`'s condition CORRECTED from the previous clause's to its
  own (*"a Subordinate is selected for a non-IDLE transfer"*) — the trap avoided rather than walked
  into. AXI-L 40 → 53: five `*VALID must remain asserted` handshake invariants, four
  `AWSTASH* must be driven LOW`, `WTAGUPDATE must_be_deasserted`, four `WTAG` value records — and
  `WTAGUPDATE must_be_value UPDATED` REMOVED, which is `.3k.2b`'s named residual closed exactly as it
  predicted ("narrowing the classifier's span fixes the value binder at the same time"). APB-E 23 → 27.
  Corpus-wide over the 74 replayable documents: **187 → 211 replayed, and the only persisted records
  that stop reproducing are NVMe `sigcon_0005`/`0006`, this leaf's own population.**
  **A new temporal conflict is a RESULT, not a regression.** AXI-L gains `WTAG post_tick VALID vs
  ZERO`: the document states both, conditional on `WTAGOP`'s enum row, and the condition lives in the
  row's value cell rather than in a `when` clause. The pipeline surfaces the ambiguity instead of
  silently keeping one — which is the roadmap's own contract for undecided evidence.
  **Four residuals found by the measurement, each given its own leaf rather than absorbed:** `.3k.7`
  (AXI `WSTRB must_be_value VALID`, a subject inside `enabled by WSTRB` that the table-row exemption
  admits), `.3k.8` (the statement path and the row path now publish APB's six `PAUSER`/`PWUSER`
  obligations twice, differing only in `source_text`), `.3k.9` (eMMC `PARTITION` from
  `PARTITION\_ACCESS`, an escaped-underscore tokenization fragment), `.3k.10` (a fronted condition
  that opens the STATEMENT carries no leading space, so its marker is never seen).
  Prerequisite: `.3k.2` (see the container's ordering rationale) — **satisfied `2026-09-13` by
  `.3k.2k`**. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.3`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.7` · Status: `pending` (opened `2026-09-13` by `.3k.3`) · Goal:
  **a table row's subject exemption survives a clause that has its own subject.**
  `is_post_passive_binding_only_subject` exempts a serialized table row from its pre-lead subject
  authority, because a row's other cells legitimately name the subject an obligation cell constrains.
  `INVARIANT-SHAPE-ADMISSION.5` withdraws that exemption when the clause HEADS with a different
  identifier. It does not withdraw it when the identifier sits one descriptor back, and AXI
  `| Match | 0b11 | … WTAG bits must be valid for byte lanes that are enabled by WSTRB. |` is exactly
  that: head `bits`, a common noun, so the exemption stands and `WSTRB` — reachable only inside the
  trailing `enabled by` phrase — is published as a co-subject of an obligation about `WTAG`.
  **The obvious rule was tried in `.3k.3` and MEASURED, and it is too wide:** withdrawing the
  exemption whenever the clause names any identifier before its lead costs three reproduced persisted
  records (MMU-700 `dyn_sigcon_0008`, RISC-V IOMMU `dyn_sigcon_0007`, NVMe `dyn_sigcon_0015`) and
  removes AXI `AWSIZE`/`AWLEN`/`AWCMO` and LTI `LASSID`/`LRMECID` value records — and it reaches the
  DYNAMIC path, which `.3k.4` owns, through the two-argument wrapper. Each of those eight must be
  adjudicated individually before any version of this ships. Prerequisite: none. Verification: all
  eight adjudicated; observed RED; the chain rebuilt for every document whose artifacts move.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.8` · Status: `pending` (opened `2026-09-13` by `.3k.3`) · Goal:
  **one obligation, two producers, two records.** APB-E publishes `PAUSER must_be_value VALID`,
  `PAUSER must_not_change` ×2 and the three `PWUSER` equivalents **twice** — once as `sigcon_*` from
  the statement path reading the serialized row, once as `row_sigcon_*` from the table reader reading
  the same cell. They differ only in `source_text`: the statement path cites the whole row, the row
  path cites the clause. `dedup_appended_signal_constraints` keys on
  `signal_constraint_merge_key`, which INCLUDES `source_text`, so it cannot see them as the same fact.
  **Pre-existing but amplified: it was 2 records before `.3k.3` and is 6 after**, because the
  statement path now reads every clause of the row the row reader already reads.
  **The decision is which provenance survives, and it is not obvious**: the row path's `source_text`
  is strictly better (the obligation's own words), but the dedup is deliberately one-directional so
  the established pattern/dynamic surface stays byte-for-byte. Changing the merge key also moves
  `replay-constraints`' reproduction identity for the whole corpus, so the population must be measured
  before and after with that in mind. Prerequisite: none. Verification: the corpus-wide duplicate pair
  count re-derived with the producer; observed RED; the chain rebuilt for every document whose
  artifacts move.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.9` · Status: `pending` — **re-derived and re-owned `2026-09-13`;
  the premise it was opened on is wrong and the disposition is DO NOT SHIP YET, on evidence** (opened
  `2026-09-13` by `.3k.3`) · Goal: **a Markdown escape fragments an identifier, and the fragment is
  then DECLARED as a signal.**
  `.3k.3` opened this as a tokenization nuisance: the normalizer emits `PARTITION\_ACCESS`,
  `collect_subject_signal_tokens` splits on any character that is not alphanumeric-or-underscore, the
  backslash ends the token, and eMMC mints `PARTITION must_not_change` about a signal the
  specification does not have. `EXTRACTION-QUALITY-GAUGE.3k.1`'s own test comment records the same
  mechanism producing `ZETADTI` out of `ZETADTI\_TBU\_CONDIS\_ACK`.
  **Where the escape comes from, measured rather than assumed.** It is not SpecForge's: it is
  Docling's, and it is CORRECT Markdown — an underscore inside an identifier must be escaped. The
  persisted `SourceIr` carries **zero** occurrences (it stores structured table cells); the normalized
  bundle carries them (`CONTEXTIDR\_EL1`); and EvidenceIR carries 2,908 in eMMC alone, because the
  evidence stage reads the bundle's text. **So "fix it upstream" is not available**: re-ingest
  reproduces it by construction, and both contaminated documents are frozen anyway, so an ingest-side
  fix would reach neither of them. The fix has to be at the reader, which is also the only place that
  reaches the frozen stratum — the same property that makes `replay-constraints` work.
  **THE FINDING THAT MATTERS, and it is why this leaf must not be shipped from its opening premise.**
  The fragmentation does not merely produce a bad subject: it produces a bad **DECLARATION**. eMMC's
  statement set contains `Signal PARTITION is width 1.` and `Enum PARTITION NOT_DEFINED = 0.` — the
  catalog holds the fragment, so every subject gate, polarity pass and relation reader downstream
  treats it as authority. **And that defeats the repository's standard discriminator.** `.3g`/`.3h`/
  `.3k.11` all use *standalone wins* — a candidate that occurs even once outside the suspect position
  is never touched — and here the synthesized declarations ARE those standalone occurrences. **The
  contamination manufactures its own evidence of innocence.** A census that does not exclude the
  synthesized `Signal …`/`Enum …` forms reports this class as empty; the first cut of this
  re-derivation did exactly that.
  **The measured populations, three of them, and they are not the same size**
  (`python3 scripts/measure_escaped_identifier_fragments.py`, shipped with this re-derivation,
  `--self-test` 6/6):
  * TEXT — **67 of 78 documents** carry an escaped identifier. Wide, and mostly provenance: a register
    name in a section title is not intent.
  * CATALOG — **18 declared names across 2 documents** exist ONLY as the head of an escaped compound:
    17 in eMMC (`PARTITION` ← `PARTITION_ACCESS`, `PARTITIONING` ← `PARTITIONING_EN`, `POWER` ←
    `POWER_CLASS`, `TAG` ← `TAG_UNIT_SIZE`, …) and 1 in GIC-600 (`REQUEST` ← `REQUEST_COMPLETE`).
  * RECORDS — **0** published constraint subjects. Not one. The eMMC record `.3k.3` mints is not
    persisted, so this becomes 1 only when that document's artifact is next refreshed.
  **THE CALL: do not ship it now, and the reason is the measurement rather than the calendar.** The
  only fix that reaches the two contaminated documents is a change to the shared identifier
  tokenization — the seam the catalog, every subject reader, the polarity pass and the relation reader
  all sit on — and it would move the identity layer of the **67** documents that carry the escape in
  order to correct **18 names in 2** of them, with a published constraint effect of **zero**. This
  family's own rule, applied twice already today, is that a rule nothing exercises does not ship
  (`.3k.5`'s magnitude leads, and `.3k.2k`'s zero population shipped only because its class was
  demonstrable through the real reader on a real sentence — this one's is not, in the records).
  Shipping it would also be unmeasurable at handoff: `CHAIN-CURRENCY` re-executes the whole pipeline
  and does not finish inside a session.
  **What has to be true before it ships**, in order: (a) the corpus effect of unescaping at the
  tokenization seam measured with `replay-constraints` AND with a full `scripts/check_doctrines.sh
  --all`, run detached, because this moves declarations and not only constraints; (b) every current-schema
  document that moves rebuilt and diffed (AXI-L carries `AWSNOOP\_WIDTH` and `WSTRB\_Present`, so it
  will move); (c) the 18 names adjudicated individually — several are real English words (`POWER`,
  `USER`, `CLASS`, `NUMBER`) whose removal from a catalog may withdraw records that are correct for
  unrelated reasons; (d) a control for the CIRCULARITY above, so the next reader cannot re-derive this
  as empty.
  Prerequisite: none. Verification: the three populations re-derived with the shipped census; all 18
  names adjudicated individually; observed RED; the chain rebuilt for every document whose artifacts
  move; `--all` doctrines green.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.10` · Status: `pending` (opened `2026-09-13` by `.3k.3`) · Goal:
  **a fronted condition that opens the STATEMENT carries no leading space.**
  `text_before_condition_marker` matches `" when "`, `" if "`, … with a leading space, so a condition
  fronting the first clause of a statement is invisible to it and the condition's own signals stay in
  the subject part. AXI `When the ACVALID signal is asserted the snoop address and control signals on
  ACADDR, ACPROT, and ACSNOOP must not change, …` therefore publishes `ACVALID must_not_change`
  alongside the three real subjects. `.3k.3` pinned the shape in
  `a_pronoun_subject_does_not_borrow_a_sibling_clauses_signals` rather than fixing it, because the
  cheap repair is wrong here: this sentence's first comma is a LIST separator, not the condition's
  boundary, so taking the text after it would also drop `ACADDR`. The clause boundary has to be found,
  not guessed. `split_conditional_sentence` already carries a leading-marker list for the same
  question and is the place to start. Prerequisite: none. Verification: the corpus population of
  statement-initial fronted conditions measured with the real reader and adjudicated; observed RED;
  the chain rebuilt for every document whose artifacts move.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.4` · Status: `done` (`2026-09-13`, CODE) · Goal: **the dynamic path's span
  discipline.** After `.3i` it reads its negation from `constraint_bearing_sentence` while its
  subject (`text_before_condition_marker(&statement.text)`), its value binder
  (`extract_discovered_state_value_from_text` over the whole lowered statement) and its condition
  (`extract_condition_clause(&statement.text)`) all read the WHOLE statement — so it is now the one
  producer whose parts are provably drawn from two different spans. The fix is not to call
  `constraint_bearing_sentence`: this path's records are minted by a value binding that need not be
  modal, so the clause it needs is the BINDING-bearing clause. Define it, then apply it to all four
  parts at once. **SHIPPED `2026-09-13`, and the shape that shipped is narrower than the node
  described, because the wider one was built and measured first.** `binding_bearing_clause` FINDS the
  binding exactly as the statement-wide reader found it — same value, same kind, bit for bit — and
  then LOCATES the first clause that reproduces it; the condition and the negation are read there.
  **Running the binders per clause instead was measured and rejected twice.** Narrowing the SUBJECT
  to that clause costs ten NVMe records whose subject is the cell's leading MNEMONIC while the binding
  is in the descriptive body (`| 17:16 | Record Format (RECFMT): … shall be 0h. |`) — the same
  row asymmetry `is_post_passive_binding_only_subject` gate (2) already encodes, and 30 reproduced
  records corpus-wide. Searching per clause then ADDS six records in AMBA LPI alone, and three of the
  six are `PREQ`/`PACCEPT must_be_high` off state-table rows that set those signals LOW, because
  `logic_level_binding_kind_from_text` pairs a level with the BIND VERB rather than with a signal
  (`.3k.11`). A span leaf must not ship recall through a pairing that is still wrong — the ordering
  `.3k` imposed on `.3k.2` before `.3k.3`, applied again.
  **Measured, corpus-wide over all 77 loadable documents: `replayed_total` is UNCHANGED in every
  single document (304 → 304). Zero records added, zero removed, 19 corrected**, and all three
  current-schema documents still load, so nothing is rebuilt. All 19 adjudicated individually with
  the instrument this leaf extended: **8 had a condition taken from a clause the record does not come
  from** (MMU-700 ×6, where `LRPROT`/`LAPROT must be 0` carried *"When LRRESP is FaultAbort … this
  signal is not valid"* — a clause that CONTRADICTS the record; CoreSight ×1; GIC-600 ×1, a trailing
  cell delimiter), **5 had a condition that ran past its clause into the next sentence** (HBM2, where
  `AERR/DERR must_be_low when parity check is suspended during power-down` swallowed *"Signals are
  shown with tPARAC=0 …"*, and `DBI must_be_high` swallowed its own `otherwise` branch), **5 carried a
  negation from a clause two sentences away** (GIC-600 ×2, AMBA LPI ×3 — `PREQ`/`PACCEPT
  must_be_high` **negated** by a `cannot` about assuming properties of a previous power state), and
  **1 had a run-on condition spanning a duplicated cell** (NVMe `ELEN`).
  **The instrument was extended because the verdict could not be adjudicated.**
  `replay-constraints` reported *"the kind, condition or negation moved"* and printed none of them;
  `ConstraintReplayVerdict` now carries `condition_text`, `negated` and a bounded `source_text`
  excerpt, which is what made all 19 readable from the report rather than from a re-derivation. That
  is an amendment to `.3k.6`'s surface, owned here because this leaf is what needed it.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.4`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.11` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.3k.4`) · Goal: **the logic-level binder pairs a level with a VERB, not with a SIGNAL.**
  `logic_level_binding_kind_from_text` finds the first bind verb (`drive`/`set`/`tied`/…) and then
  takes the LAST logic value within six words of it, and the caller pairs that kind with every
  declared signal the statement names. So `| P_ACCEPT | … | Controller must set PREQ LOWand PREQCHK
  HIGH. |` publishes **`PREQ must_be_high`** — the level belongs to `PREQCHK`, one token later, and
  the record says the opposite of what the row states. Two such records are already published in AMBA
  LPI, and `.3k.4` measured that searching the binder per clause would add three more.
  **Size it against the real binder before changing it**, and note the normalizer artifact in the
  same population: the same rows read `LOWand`/`HIGHafter` with the space lost, so a fix that assumes
  clean word boundaries will behave differently on the corpus than on a test string
  (`[[one-modal-vocabulary-per-constraint-record]]`'s lesson, one layer down). The obvious rule — pair
  the level with the nearest preceding identifier — must be measured against every `must_be_high`/
  `must_be_low` record the dynamic path currently publishes, because that path is 77 of the corpus's
  deterministic records.
  **SHIPPED `2026-09-13`, and the obvious rule was wrong in three separate ways the corpus showed.**
  (a) *Nearest PRECEDING* is wrong: AMBA LPI writes *"a controller with an absent or tied LOW QDENY
  signal"*, so the walk goes backward first and forward only when backward finds nothing. (b) *Shape*
  is wrong: an identifier cannot be recognised by its CASE, because
  `WIRE-BASED-100.5i`'s alpha-invariance control feeds this path
  `signal_alias_000001_ready_000000006d11fd13` and requires identical behaviour — so the DOCUMENT'S OWN
  CATALOG decides what an identifier is, which is the repository's idiom everywhere else and the only
  ADR-0006-safe answer. That control went RED on the first implementation and is the reason this leaf
  has a catalog parameter at all. (c) *Clean word boundaries* are wrong: the normalizer loses the space
  in `LOW and` / `HIGH after`, so `token_logic_level` reads a token's leading uppercase RUN as well as
  the whole token — and that run is also what stops `PREQCHK HIGH` reaching back past `LOWand` to
  `PREQ`. A SUBSCRIPT is skipped rather than treated as a boundary (`sets HPROT[0] HIGH`), which the
  AHB rebuild proved necessary: without it a correct record is lost alongside the fabricated one.
  `logic_level_binding_kind_from_text` is RETIRED — it answered "is there a level after a binding
  verb" and nothing about what the level belonged to.
  **Measured, all 22 reproduced logic-level records adjudicated individually, plus every record the
  change adds or removes. Corpus replayed is 304 before and 304 after**, and the composition is the
  result: **11 fabrications removed** — GIC-600 `PMU`/`GIC must_be_high` (the HIGH belongs to the
  lowercase tie-off `gicp_allow_ns`) and `MBIST must_be_high` (it belongs to the row's own
  `nmbistreset`); CoreSight `ATB must_be_low` (it belongs to `araddr_m`/`awaddr_m`, while `ATB` comes
  from `ATB_DATA_WIDTH` a sentence later); AXI-ACE `WVALID must_be_low` from *"When WVALID is LOW, the
  write strobes can take any value"*, a CONDITION; AHB `HTRANS must_be_high` (it belongs to `HSEL`);
  LPI `PREQ`/`PACCEPT must_be_high` (they belong to `PREQCHK`/`PACCEPTCHK`); NVMe `NVM`/`LBA
  must_be_low` from *"used to low level format the NVM media"*; HBM2 `DM must_be_high` (*"DM output is
  not affected by the DBIac function"*). **15 correct records added** — ten AMBA LPI P-Channel
  state-table rows (`Controller has set PREQ LOW…`, `Device must set PACCEPT LOW`), AXI-L
  `AWSNOOP`/`ARSNOOP must_be_low` from *"An attached Subordinate must have its AWSNOOP input tied
  LOW"*, HBM2 `DBI must_be_low` from the `otherwise` branch it used to swallow and `CKE must_be_low`,
  and CoreSight TMC `FULL must_be_high` from *"the FULL output is pulled HIGH"*.
  **One correct record is LOST and is named rather than absorbed:** LPI `QDENY must_be_low` from
  *"…with the QDENY output absent or tied low"*. The walk stops at `absent`, which is a predicate
  adjective rather than scaffolding, and the sibling sentence one figure earlier (*"an absent or tied
  LOW QDENY signal"*, signal AFTER the level) still binds. Widening the skip list to fit this one
  sentence would be fitting the rule to an instance; the residual is `.3k.12`.
  **`.3k.4`'s control is superseded in part** and says so in place: it compared the located clause's
  kind against `logic_level_binding_kind_from_text`, and that function no longer exists. Its reasoning
  stands for the DISCOVERED-VALUE binder, which is what it now asserts.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.11`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.12` · Status: `pending` (opened `2026-09-13` by `.3k.11`) · Goal:
  **a predicate between a signal and its level stops the level finding it.** AMBA LPI states the same
  fact two ways one figure apart: *"a controller with an absent or tied LOW QDENY signal"* binds,
  because the signal FOLLOWS the level; *"with the QDENY output absent or tied low"* does not, because
  walking back from the level reaches `absent` — an adjective predicated of the signal, not the
  scaffolding (`the`, `its`, `input`, `signal`) the walk skips. One measured record, and `.3k.11`
  deliberately did not widen its skip list to fit it: a list tuned to one sentence is a mirror of that
  sentence. The question to answer first is whether PREDICATE ADJECTIVES are a class the walk should
  cross at all — *"absent"*, *"present"*, *"unused"*, *"reserved"* all sit in that position in this
  corpus — and what crossing them costs elsewhere. **Size it against every logic-level record before
  changing the walk**, the way `.3k.11` was sized. Prerequisite: none. Verification: the corpus
  population of a predicate between a signal and its level, adjudicated individually; observed RED;
  the chain rebuilt for every document whose artifacts move.
- ID: `EXTRACTION-QUALITY-GAUGE.3k.13` · Status: `done` (`2026-09-13`, CODE; opened the same day by
  `.3k.11`) · Goal: **the dynamic path has no MODALITY gate.** AHB `dyn_sigcon_0012` publishes `HPROT must_be_high` from
  *"It is **recommended** that a Manager sets HPROT[0] HIGH"*. The pairing is right and the level is
  right; what is wrong is that a RECOMMENDATION is published as a hard constraint.
  `EXTRACTION-QUALITY-GAUGE.3k.2a` refused exactly this shape in the statement path — *"It is
  recommended, but not required, that PSLVERR is driven LOW"* was one of its seventeen — but that
  refusal rides the kind classifier, which this path never reaches: it types a record from its VALUE
  BINDER. So the class is live here and nowhere gated.
  **SHIPPED `2026-09-13`. Actionable population: 2, both adjudicated, and the leaf is as small as its
  population.** AHB `dyn_sigcon_0012` (*"It is **recommended** that a Manager sets HPROT[0] HIGH"*) and
  AMBA LPI `dyn_sigcon_0009` (*"Figure 2-16 shows how a device **can** be interfaced directly to a
  controller with an absent or tied LOW QDENY signal"* — a permitted configuration, in a figure
  caption). Corpus replayed **292 → 291 over the 76 comparable documents**, and the AHB rebuild takes
  it 12 → 11; nothing else in the corpus moves. LPI's real requirement survives untouched, because the
  document states it in its own mandatory clauses (`dyn_sigcon_0007`/`0008`, *"QDENY must be tied
  LOW"*) — which is the shape of evidence that makes the refusal safe rather than merely defensible.
  **The control that defines the gate's limit is the one that keeps this producer alive.** A
  specification binds a signal FLATLY all the time — *"the FULL output is pulled HIGH"*, *"AERR, DERR
  are driven LOW"* — and those are real invariants with no modal anywhere. So the refusal needs an
  EXPLICIT non-mandatory marker, and a mandatory modal in the same clause outranks it; a permission
  granted in one sentence does not suppress the requirement stated in the next
  (*"A Manager … can set the width to 0. An attached Subordinate must have its AWSNOOP input tied
  LOW."*). A modality gate written any wider on a path that reads BINDINGS rather than OBLIGATIONS
  would refuse almost everything it exists to capture.
  **Two `.3k.11` controls were retargeted, not weakened.** Both were written from corpus sentences
  that this leaf now refuses, so each now asserts its property on `logic_level_bindings` directly —
  one rule, one control. Their reasoning is unchanged and is recorded in place.
  Prerequisite: none. Verification: see the acceptance checklist below.
  Commit: `EXTRACTION-QUALITY-GAUGE.3k.13`

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

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.5`

- [x] **REPRODUCE / MEASURE** — `replay-constraints` over all 77 loadable documents: **302 replayed
  and 127 reproduced, before and after — nothing moves.** The four statements the node was sized from
  were counted with a Python mirror before `.3k.3` and `.3k.2k` existed, and neither survives as this
  leaf's population: the record it expected to recover is refused by `.3e`'s descriptive-field-cell
  gate, and the record it expected to expose is refused by `.3k.2k`. The class is nevertheless
  demonstrable through the real reader, which is the `.3k.1` footing `.3k.2e`/`.3k.2f`/`.3k.2k`
  shipped on.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`. Both
  `is_relational_equality_constraint` (`.3d`) and `is_reference_magnitude_constraint` (`.3k.1`) were
  evaluated over `&statement.text` in both deterministic statement-level producers, while the records
  they suppress became clause-scoped in `.3k.3` and `.3k.4`. So a relation mentioned anywhere in a
  statement refused every obligation that statement states — the container's own defect, on the
  refusal side.
- [x] **ADDRESSED (verified)** — the two guard clauses moved inside the loops that already hold the
  clause: the obligation in `extract_signal_constraints`, the binding in
  `extract_dynamic_signal_constraints` (both binders). **Three controls in
  `mod extraction_quality_gauge_3k_5`, two observed RED with the refusals put back on the statement**
  and the file restored byte-identically: a relation in another clause no longer refuses this
  obligation, the clause that STATES the relation is still refused, and a statement stating both keeps
  only what the vocabulary can hold.
- [x] **NO REGRESSION** — corpus replay identical (302/127), so no persisted artifact moves and no
  chain rebuild is owed; all 77 documents still load. Wire golds hold: `signal_constraint
  P=R=F1=1.000` with **fp=0** on APB, AHB and AXI. `kg-bench` **156/156**; `cargo fmt --all --check`,
  `cargo clippy --offline --all-targets -D warnings` and the whole workspace suite green
  (`specforge-core` lib 1,500 → **1,503**). `flow_census.json` re-derived and attributed: one decision
  site, no new function.
- [x] **GENERICITY (ADR 0006)** — a scope change to two existing rules; no new vocabulary. The three
  leads that WOULD have been vocabulary are explicitly not added, with the measurement recorded.
- [x] **LOCKSTEP** — the book's *"A bound stated against another operand is not a value"* section says
  *"Both readers now refuse, for the same reason, on the clause each is holding"* — which this leaf is
  what finally makes true of all three producers, so the sentence stands and is now accurate rather
  than aspirational. `is_reference_magnitude_constraint`'s doc comment asserted the gate *"is evaluated
  over the whole statement"*; that is now false and is rewritten in the same edit, together with the
  measurement that keeps the leads out. No KM card: the durable fact is
  `[[one-record-per-obligation-clause]]`'s, and this leaf is its refusal-side completion rather than a
  new mechanism.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.13`

- [x] **REPRODUCE / MEASURE** — the population is **2 records**, surfaced by `.3k.11`'s own
  adjudication of all 22 reproduced logic-level records and confirmed with `replay-constraints`:
  corpus replayed **292 → 291** over the 76 comparable documents, and AHB 12 → 11 on rebuild. AHB
  `dyn_sigcon_0012` is a RECOMMENDATION; AMBA LPI `dyn_sigcon_0009` is a PERMISSION in a figure
  caption. Nothing else in the corpus moves.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `extract_dynamic_signal_constraints`. This producer types a record from its VALUE BINDER and never
  from a modal — correctly, because a flat binding is a real invariant — so it had no modality gate at
  all. `EXTRACTION-QUALITY-GAUGE.3k.2a` refuses the same shape in the statement path, but that refusal
  rides the kind classifier, which this producer never reaches.
- [x] **ADDRESSED (verified)** — `binding_is_non_mandatory`, applied to the clause each binder bound
  in, for both binders. **Five controls in `mod extraction_quality_gauge_3k_13`, two observed RED with
  the predicate stubbed to `false`** and the file restored byte-identically. The other three are the
  gate's limit: a flat binding with no modal at all still mints, a mandatory modal outranks a
  permission in its own clause, and a permission in another clause does not suppress the requirement.
  AHB rebuilt (`evidence → validate → semantic → validate → intent → validate → adapt`, each validated
  exactly once, upstream-first; bundle restored, `diff -rq` clean, removed, retention back to **24**).
- [x] **NO REGRESSION** — the wire golds hold: `signal_constraint P=R=F1=1.000` with **fp=0** on APB,
  AHB and AXI, document-level fact recall **1.000**. `kg-bench` **156/156**; `cargo fmt --all --check`,
  `cargo clippy --offline --all-targets -D warnings` and the whole workspace suite green
  (`specforge-core` lib 1,495 → **1,500**). `flow_census.json` re-derived and attributed. Only AHB's
  artifacts move, and only by the one record.
- [x] **GENERICITY (ADR 0006)** — universal English deontic modality, the RFC-2119 distinction every
  specification in this corpus is written against. No document, protocol or vendor vocabulary; every
  test identifier alpha-renamed.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md` gains the distinction beside the dynamic
  path's own section, since that section is what explains why this producer reads bindings rather than
  obligations. KM card: `[[a-level-belongs-to-a-signal]]` is amended rather than duplicated — it
  already carries this residual as the one the adjudication surfaced, and it is the same reader.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.11`

- [x] **REPRODUCE / MEASURE** — the actionable population derived with `replay-constraints`: of the
  dynamic path's judged records, **22 reproduce with a logic-level kind**, and all 22 were adjudicated
  individually against their own source before anything was written. Six were wrong — AHB `HTRANS`
  (the level belongs to `HSEL`), LPI `PREQ must_be_high` ×2 (it belongs to `PREQCHK`), NVMe
  `NVM`/`LBA must_be_low` (*"low level format"*), and AHB `HPROT` (right pairing, wrong MODALITY —
  routed to `.3k.13`).
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `logic_level_binding_kind_from_text` (now retired). It returned the LAST logic level within six
  words of a binding verb and said nothing about what that level belonged to; the caller then attached
  that one kind to EVERY declared signal the statement named. Two independent errors in one reader, and
  both are visible in one row: `| P_ACCEPT | … | Controller must set PREQ LOWand PREQCHK HIGH. |`
  publishes `PREQ must_be_high`.
- [x] **ADDRESSED (verified)** — `logic_level_bindings` pairs each level with the signals ADJACENT to
  it, walking backward first and forward when backward finds nothing, stopping at another level, and
  reading identity through the DOCUMENT'S OWN CATALOG. **Eight controls in
  `mod extraction_quality_gauge_3k_11`, three observed RED by removing exactly one rule each** — the
  level delimiter (`ZETAREQ must_be_high` reappears), the per-level subjects (`ZETATRANS must_be_high`
  reappears), and the forward walk — with the file restored byte-identically each time.
  **Corpus replayed is 304 before and 304 after, and the composition is the result: 11 fabrications
  removed, 15 correct records added, 1 correct record lost and named** (`.3k.12`). Every one of the 27
  is listed with its sentence in the node. **Two documents rebuilt** (AHB 13 → 12, AXI-L 53 → 55), the
  bundles restored from `generated/preserved/WIRE-BASED-100.10/`, `diff -rq` clean, removed again,
  retention back to **24**.
- [x] **NO REGRESSION** — the wire golds hold: `signal_constraint P=R=F1=1.000` with **fp=0** on APB,
  AHB and AXI, `temporal_rule 1.000`, document-level fact recall **1.000** for constraints and
  relations. `WIRE-BASED-100.5i`'s alpha-invariance control passes and is the control that shaped the
  design. `kg-bench` **156/156**; `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D
  warnings` and the whole workspace suite green (`specforge-core` lib 1,487 → **1,495**).
  `flow_census.json` re-derived and attributed.
- [x] **GENERICITY (ADR 0006)** — universal English adjacency plus the logic-level vocabulary the
  repository already carries, with identity read only through the document's own declaration catalog.
  The alpha-invariance control is the proof rather than the claim: an opaque signal alias binds exactly
  as a conventional name does, and the same sentence with an undeclared name binds nothing.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md`: `.3k.4`'s section said a level *"is
  currently paired with the verb that sets it"* and that *"recall waits for that to be fixed"* — that
  is now false and is rewritten in the same edit, which is the `BOOK-BEHAVIOUR-CURRENCY` case of a
  changed rule leaving standing book text. KM card `[[a-level-belongs-to-a-signal]]`. Two residuals
  routed to `.3k.12` and `.3k.13` rather than left in prose.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.4`

- [x] **REPRODUCE / MEASURE** — built as a prototype and measured with `replay-constraints` before the
  design was settled, which rejected TWO wider shapes. (a) Narrowing the SUBJECT to the binding clause:
  **99 → 69 reproduced**, ten NVMe records lost because a register row names its subject in the cell
  mnemonic and binds in the body. (b) Searching the binders per clause: **+6 records in AMBA LPI**,
  three of them `must_be_high` off rows that set the signal LOW. The shipped shape measures
  **304 → 304 replayed in every one of the 77 loadable documents — zero added, zero removed — and 19
  corrected**, each adjudicated individually below.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `extract_dynamic_signal_constraints`. `.3i` gave this path `constraint_bearing_sentence` for its
  negation, which locates an obligation MODAL — and this producer's record is minted by a VALUE
  BINDING that need not be modal at all. So the negation came from whichever clause happened to carry
  a `must`/`cannot` while the condition came from the whole statement, and neither had to be the
  clause that bound the value. Evidence: AMBA LPI `dyn_sigcon_0014`/`0015` published `must_be_high`
  **negated** from a `cannot` two sentences away; MMU-700 `dyn_sigcon_0007`/`0008` carried a condition
  that says the signal is not valid.
- [x] **ADDRESSED (verified)** — `binding_bearing_clause` finds the binding statement-wide exactly as
  before and then locates the first clause that reproduces it, failing OPEN to the whole statement
  when none does; `condition_text` and `negated` read that clause. The kind and value are unchanged by
  construction and a control asserts it. **Five controls in `mod extraction_quality_gauge_3k_4`, two
  observed RED with the two reads reverted** — the condition control reporting the exact corpus
  string `"ZETARESP is FaultAbort, this signal is not valid. Width is 3-bit. |"` — and the file
  restored byte-identically. All 19 corrections adjudicated: 8 conditions from a foreign clause, 5
  conditions running past their clause, 5 negations from a clause two sentences away, 1 run-on
  condition across a duplicated cell.
- [x] **NO REGRESSION** — `replay-constraints --evidence-root generated/evidence_ir`:
  **304 replayed before and after, in every document**, so no persisted artifact moves, all three
  current-schema documents still load, and no chain rebuild is owed. `kg-bench` **156/156**;
  `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings` and the whole
  workspace suite green (`specforge-core` lib 1,482 → **1,487**). `flow_census.json` re-derived and
  attributed: `analyzed_functions` +2, `decision_sites` +5, `helper_edges` +3, `semantic_macros` +1.
- [x] **GENERICITY (ADR 0006)** — the clause split is the repository's existing punctuation split and
  the binding is located by asking the PRODUCER's own binder, so no second rule, no vocabulary, no
  document, protocol or vendor name. Every test identifier is alpha-renamed.
- [x] **LOCKSTEP** — book `commands/quality-and-learning.md` documents the three fields
  `replay-constraints` verdicts gained, because a reader is told a record moved and must be able to
  see what moved; `pipeline/obligation-reading.md` gains the dynamic path's own span rule beside the
  statement path's. KM card `[[the-binding-bearing-clause]]`. The LPI pairing defect is routed to
  `.3k.11` rather than left in prose.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.3`

- [x] **REPRODUCE / MEASURE** — the change was built as a PROTOTYPE and measured with the real
  producer before any of it shipped, which the node required and which changed the design twice.
  `replay-constraints --json` captured per document before and after, diffed over the 74 documents
  comparable in both (AHB/AXI-L/APB-E drop out — the change moves their content, which stales their
  proofs so they refuse to LOAD, and that is itself the signal that they need rebuilding). Final:
  **187 → 211 replayed; 61 → 59 reproduced, and the only two lost are NVMe `sigcon_0005`/`0006`, this
  leaf's own population**. The first prototype measured **+19 with three fabrications**
  (`ACADDR/ACPROT/ACSNOOP must_be_asserted` from a clause whose subject is the pronoun `it`;
  LTI `signal`/`LAFLOW must_be_low`), and the second confirmed the container's ordering rationale by
  publishing NVMe `must_be_value UNIQUE` — which is why `.3k.2k` was opened and landed first.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `extract_signal_constraints`. The loop ran ONCE per statement over
  `constraint_bearing_sentence(text)`, the FIRST clause carrying a modal, and classified the kind over
  the WHOLE statement. So obligations 2..n were dropped, and the one record that survived could take
  its kind from one clause and its condition from another — AHB `| HSELx a | … |` did exactly that.
  Three readings underneath it were wrong for the same reason: a FRONTED condition cuts
  `text_before_condition_marker` at offset 0 and leaves no subject part; the subject fallback then
  scanned the whole STATEMENT; and `is_post_passive_binding_only_subject` re-derived the obligation as
  the statement's FIRST clause, so it judged the Nth record against the 1st record's clause.
- [x] **ADDRESSED (verified)** — `constraint_bearing_sentences` (all obligation clauses in document
  order, with the singular helper as its first element so the two cannot drift), a per-obligation
  loop, `obligation_subject_part` (the main clause of a fronted conditional), an obligation-bounded
  subject fallback, `is_post_passive_binding_only_subject_in` (told which obligation it judges), and
  an in-statement dedup on the producer's own merge identity. **Eight controls in
  `mod extraction_quality_gauge_3k_3`, each part observed RED by disabling exactly that part and the
  file restored byte-identically each time:** all-clauses (2 RED), the fronted-condition subject
  (2 RED), the obligation-bounded fallback (1 RED — `ZETASEL must_be_stable`, a second contradictory
  kind for the signal the first clause constrains), the dedup (1 RED). **Three documents rebuilt**
  (`evidence → validate → semantic → validate → intent → validate → adapt`, each validated exactly
  once, upstream-first, bundles restored from `generated/preserved/WIRE-BASED-100.10/` and removed
  again with `diff -rq` clean, retention back to **24**): AHB 13 → 13 with its condition corrected,
  APB-E 23 → 27, AXI-L 40 → 53 — and `WTAGUPDATE must_be_value UPDATED` removed, closing `.3k.2b`'s
  named residual. `.3i`'s inherited claim is closed too: every part of a `sigcon_*` record now comes
  from one span.
- [x] **NO REGRESSION** — the wire golds hold at the bar: `signal_constraint P=R=F1=1.000` with
  **fp=0** on APB, AHB and AXI, `temporal_rule 1.000` on all three, and document-level fact recall
  **1.000** for constraints and relations. Section-by-section the rebuilt EvidenceIRs move ONLY
  `signal_constraints`, their derived `fact_provenance`, and `conditional_rules` **by id alone**
  (content byte-identical, ids shifted by the shared counter) — `actor_signal_relations` is untouched,
  which is why the golds' `drives` attribution numbers are unchanged and pre-existing. AHB's IntentIR
  and `.isf` are unchanged entirely. `kg-bench` **156/156**; `cargo fmt --all --check`, `cargo clippy
  --offline --all-targets -D warnings` and the whole workspace suite green (`specforge-core` lib
  1,474 → **1,482**). `flow_census.json` re-derived and attributed: `analyzed_functions` +3,
  `decision_sites` +6, `helper_edges` +37, `semantic_macros` +1. AXI-L gains one temporal conflict
  (`WTAG` VALID vs ZERO) — a correct surfacing of an ambiguity the document states across two enum
  rows, not a regression.
- [x] **GENERICITY (ADR 0006)** — universal English clause structure only: a statement decomposes into
  clauses at the punctuation the row reader already uses, an obligation is a clause carrying one of
  the modals `.3k.2d` fixed, and a fronted conditional's main clause follows its comma. No document,
  protocol, vendor or token list; every test identifier is alpha-renamed.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md` gains *"A statement that states three
  obligations yields three records"* as the chapter's first rule, since every other rule in it now
  operates per obligation. No book text described the old one-record-per-statement behaviour, so
  nothing is deleted. KM card `[[one-record-per-obligation-clause]]`. Four residuals routed to
  `.3k.7`–`.3k.10` rather than left in prose.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2k`

- [x] **REPRODUCE / MEASURE** — the class demonstrated with the REAL producer on real corpus grammar:
  `extract_signal_constraints` over NVMe `statement_7397`'s own obligation clause returns
  `Some("must_be_value:UNIQUE")`, and over OpenCAPI `statement_0613`'s
  `Some("must_be_value:COMPATIBLE")` — both captured as the observed-RED assertion output, not
  predicted. Actionable population over the persisted corpus: **0**. The published deterministic
  `must_be_value` census is `0`/`1`/`5`/`12`/`0B01`/`0B11`/`VALID`/`LOW`/`NO`/`SET`/`PACKED`/`INVALID`/
  `INVALIDATED`/`UPDATED` and carries no relational value, so this leaf ships on the `.3k.1` footing
  `.3k.2e`/`.3k.2f` also used: the class is live and demonstrable through the reader even where the
  corpus has not yet published it. What makes it urgent rather than speculative is `.3k.3`'s addition
  measurement, which ran the narrowed producer over all 78 artifacts and watched NVMe gain exactly the
  two `must_be_value` records the container's ordering rationale predicted.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `classify_signal_constraint_kind`'s terminal `generic_value` arm via `is_admissible_state_value`.
  `.3k.2b` gated that slot on ONE shape — the passive participle — and an adjective wears no participle
  ending, so `unique`, `compatible` and `greater` walk straight through a gate built to stop
  `invalidated`. The slot asks "is this word a value?" and had no way to notice that the predicate is
  not satisfied by the subject alone.
- [x] **ADDRESSED (verified)** — `value_slot_states_a_relation` refuses a predicate whose next word
  introduces a second operand or a scope, reached only from `is_admissible_state_value`'s final branch
  so the declared/level/numeric routes still win first; `protocol_state_value_and_complement` returns
  the value and that next word from ONE scan, with `extract_protocol_state_value` delegating to it.
  **Six controls in `mod extraction_quality_gauge_3k_2k`, two of them observed RED with the rule
  removed** — `must_be_value:UNIQUE` and `must_be_value:COMPATIBLE` — and the file restored
  byte-identically afterwards. The other four are the line the rule must not cross: a state complete at
  the word still binds, `by` is not a relation marker, all three admissibility routes win first, and
  the complement comes from the binder the value came from.
- [x] **NO REGRESSION** — `replay-constraints --evidence-root generated/evidence_ir` is **identical
  across the change: 183 persisted / 263 replayed / 137 reproduced / 126 unpersisted, over all 77
  loadable documents**, so no persisted artifact moves and no chain rebuild is owed. `kg-bench`
  **156/156**; `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings` and the
  whole workspace suite green (`specforge-core` lib 1,468 → **1,474**). One existing control failed and
  was CORRECTED rather than accommodated — see `.3k.1`'s amendment. `doctrine/production_genericity/
  flow_census.json` re-derived and attributed: `analyzed_functions` +2, `helper_edges` +34.
- [x] **GENERICITY (ADR 0006)** — universal English clause grammar: a predicate's complement marker.
  No adjective list, no value vocabulary, no document, protocol or vendor name. The one lexical set is
  a closed list of English prepositions, the same footing as `.3d`'s comparative markers, and `by` is
  excluded by its grammatical ROLE rather than by taste.
- [x] **LOCKSTEP** — book `pipeline/obligation-reading.md` gains *"A predicate that names a scope or
  another operand is not a value"* beside `.3k.2b`'s section, since both are about the same slot. The
  chapter's existing *"A bound stated against another operand is not a value"* table asserted
  `The value of PRANGE must be greater than 0` was **kept**; that is the sentence whose record this
  leaf refuses, so the row and its paragraph are corrected in the same edit — the
  `BOOK-METHOD-DOC`/`BOOK-BEHAVIOUR-CURRENCY` case of a changed rule leaving standing book text. KM card
  `[[a-relational-predicate-is-not-a-value]]`.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2j`

- [x] **REPRODUCE / MEASURE** — the check's own history re-derived per revision with that revision's own
  scanner over that revision's own `evidence.rs`: green at `1ada364a` (`.3k.1`), RED at `24a605e8`
  (`.3k.2a`) and still RED at `a81d70ab` (`.3k.2c`). And the sibling census, classified rather than
  counted: 86 script-shaped files, 28 unreachable from any driver/hook/workflow/doctrine registry, **2**
  of those claiming `--check` semantics.
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_doctrines.sh` is the registry and the script was
  never added to it; `scripts/run_ci.sh` and `.githooks/pre-commit` both delegate to that registry, so
  one omission removed all three enforcement legs at once. The script itself was correct and simply
  never executed.
- [x] **ADDRESSED (verified)** — `scripts/check_constraint_part_span.sh` registered as
  `CONSTRAINT-PART-SPAN|gate`; the driver reports **PASS** and the count moves 13 → 14 executed
  doctrines. **Two observed-RED controls, both run through the DRIVER rather than the script, because
  what failed here was the wiring and not the check:** (a) making the row reader call the untyped
  classifier directly fails `CONSTRAINT-PART-SPAN` in the doctrine report; (b) adding a third producer
  (`zeta_probe_third_producer`) fails it naming the found set against the expected one — the exact drift
  `.3k` built the script for. `crates/` restored byte-identically after both (`git diff --stat` empty).
- [x] **NO REGRESSION** — no production code changes in this slice; `crates/` is byte-identical to the
  previous commit. `scripts/check_doctrines.sh` reports **all 14 gate-tier doctrines PASS**, the meta-check
  that every registered enforcer exists and is executable included. `cargo fmt --all --check`, `cargo
  clippy --offline --all-targets -D warnings` and the whole workspace suite stay green; `kg-bench`
  **156/156**.
- [x] **GENERICITY (ADR 0006)** — enforcement wiring only; no extraction rule, no vocabulary, no
  document, protocol or vendor name.
- [x] **LOCKSTEP** — the adapter, the registry line, and the `DOCTRINE_ENFORCEMENT.md` §10 row are the
  three surfaces that must agree and they are written together; the §10 row records how the gap was
  found and the sibling enumeration, so the next reader inherits the census rather than the conclusion.
  `TOOLBOX.md` needs no change: it catalogs diagnostic tools, and this script's entry is its doctrine
  row. `RETAINED-BUNDLE-POPULATION-FROZEN` keeps the one sibling finding; it is named here, not moved.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2e`

- [x] **REPRODUCE / MEASURE** — derived with the producer, after `.3k.2g` made that possible. Over the
  26 documents whose table classifications the current loader accepts, the row reader mints **12
  records and not one more**, and every one is typed by an arm the document wrote — so the untyped
  fallback's actionable population is **0**, not the 17 `.3k.2d` published from a mirror over the
  persisted `table_kind`. Baseline `replay-constraints --evidence-root generated/evidence_ir`: 183
  persisted / 137 reproduced / 46 not-reproduced.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `extract_signal_description_row_constraints`: it called `classify_signal_constraint_kind`, whose
  terminal arm returns `MustBeStable` for a clause no phrase matched. `.3k.2a` left that deliberately,
  and its justification is what expired: the four APB clauses it reasoned from were typed by `.3k.2c`.
  Demonstrated on the real reader — a MATCH, an ALIGNMENT and a PRESENCE claim each publish
  `must_be_stable`, the last negated.
- [x] **ADDRESSED (verified)** — the row reader goes through `classify_signal_constraint_kind_typed`
  and refuses on `None`, so no producer can publish the terminal arm. **Observed RED:** restoring the
  untyped call republishes exactly those three records (`ZETAREADY`/`OMEGABURST` `must_be_stable`,
  `SIGMASTRB must_be_stable, negated: true`) while the over-kill guard stays green — the pair is the
  property, because a refusal that also dropped a stated obligation would be a worse trade than the
  fabrication. Corpus re-measured: **183 / 137 / 46, unchanged** — 0 added, 0 removed, 0 retyped, no
  persisted artifact moves.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings`
  and the whole workspace suite green; `specforge-core` lib **1,466 → 1,468**. `kg-bench` **156/156**.
  WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000` on APB/AHB/AXI/SWD. All 13 gate-tier doctrines
  PASS; `flow_census.json` unmoved by this slice (the refusal replaces an assignment rather than adding
  a branch the census counts). The over-kill guard asserts all four arms the live records use still
  publish from a row: header-supplied validity, `.3k.2c`'s no-change spelling, a negative polarity
  form, and a plain value binding.
- [x] **GENERICITY (ADR 0006)** — no new rule and no vocabulary: one caller now uses the typed
  gateway the other already used. Controls use invented names (`ZETAREADY`, `OMEGABURST`,
  `ALPHACHUNK`, `SIGMASTRB`).
- [x] **LOCKSTEP** — code, this leaf, `.3k.2a` (whose asymmetry is superseded in place, with the
  reasoning kept and its population named as what changed), the book's EvidenceIR chapter, and the
  resume pointer agree before commit. **A production rule IS replaced here** — the row path's fallback
  — and the book text that described it is repaired rather than left standing: the "An obligation that
  names no kind states no constraint" section ended with a subsection asserting the table-row reader
  keeps the fallback and why, which is no longer true. That section now lives in
  `docs/book/src/pipeline/obligation-reading.md`: this leaf's own book edit pushed the EvidenceIR
  chapter 299 bytes past its ceiling and `shipped_behavior` blocked the commit, which
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.19` resolved by splitting the chapter rather than by trimming the
  paragraph — the remedy `.3` had already adjudicated for this exact surface. `scripts/measure_constraint_part_span.py` is
  re-derived, not re-pinned: its topology now tracks the typed gateway and adds the untyped-caller
  invariant, and its self-test gains a case for the `#[cfg(test)]` scope bug the re-derivation exposed
  (9 → 10 declared cases).

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2f`

- [x] **REPRODUCE / MEASURE** — a set claim, carried with its enumeration in both directions.
  `is_relational_equality_constraint` and `is_reference_magnitude_constraint` have exactly one caller
  each, `extract_signal_constraints`; the row reader calls neither. Population over the corpus, derived
  with the real producer now that the row stratum is judged (`.3k.2g`): of the row clauses the 26
  judgeable documents admit, **0** match either predicate. Baseline `replay-constraints
  --evidence-root generated/evidence_ir`: 183 persisted / 137 reproduced / 46 not-reproduced.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `extract_signal_description_row_constraints`: the clause goes from `obligation_subject` straight to
  `classify_signal_constraint_kind`, with no vocabulary-slot gate between them. Demonstrated through
  the real reader, not read off a diff: a signal-description table whose cell states either relation
  publishes `ZETARANGE must_be_stable, negated: true` and `OMEGABURST must_be_value VALUE`.
- [x] **ADDRESSED (verified)** — both predicates evaluated over the clause, before classification.
  `a_clause_with_no_vocabulary_slot_is_refused_in_the_row_path_too` goes from two fabricated records to
  none. **Observed RED:** removing the guard restores exactly those two records, with the kinds quoted
  above; restored green. Corpus re-measured after the change: **183 / 137 / 46, unchanged** — 0 added,
  0 removed, 0 retyped, and no persisted artifact moves.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings`
  and the whole workspace suite green; `specforge-core` lib **1,464 → 1,466**. `kg-bench` **156/156**.
  WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000` on APB/AHB/AXI/SWD and `temporal_rule 1.000` on
  APB/AHB/AXI. All 13 gate-tier doctrines PASS. `flow_census.json` re-derived and attributed to this
  leaf (+1 decision site, +2 helper edges). The over-kill guard is its own control: an ordinary row
  obligation still extracts, and both predicates are asserted to keep NOT firing on a magnitude against
  a literal (*"must be greater than 0"*) or on `.3k.2c`'s *"the same value IN …"*.
- [x] **GENERICITY (ADR 0006)** — no new rule; two existing predicates reach a second caller. Both are
  keyed on ordinary English comparatives and reference leads, and the controls use invented names
  (`ZETARANGE`, `ZETAOAS`, `OMEGABURST`, `ZETAREADY`).
- [x] **LOCKSTEP** — code, this leaf, and the book's EvidenceIR chapter agree before commit: the "A
  bound stated against another operand is not a value" section said the deterministic paths refuse
  these sentences, which was true of one path and is now true of both, so the chapter's claim is
  repaired rather than extended. No production rule is deleted or replaced. The resume pointer's next
  action moves on.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2g`

- [x] **REPRODUCE / MEASURE** — the instrument's own blind spot, stated as a census rather than an
  impression: `replay_persisted_signal_constraints` filtered the judged set to `sigcon_*` and
  `dyn_sigcon_*`, so the 12 published `row_sigcon_*` records (APB 10, AXI-L 1, AHB 1) were invisible to
  every corpus figure `.3k.6` and `.3k.2a`–`.3k.2d` quoted. Baseline
  `replay-constraints --evidence-root generated/evidence_ir`: 171 persisted / 125 reproduced / 46 not.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `replay_persisted_signal_constraints`: the `deterministic` filter names two id prefixes, and the
  replay never calls `extract_signal_description_row_constraints` because that producer needs a
  `SourceIr` the function was not given. The second cause was found by probing the real producer, not
  by reading: with LTI's persisted `SourceIr` loaded through `load_for_inspection`,
  `should_treat_table_as_top_level_signal_description` passes **0 of 88** tables — because
  `neutralize_legacy_source_classifications` (`crates/specforge/src/ir/source.rs`) sets every
  `table_kind` to `Unknown` for a legacy artifact, while that document's persisted JSON marks 25 tables
  `signal_description`.
- [x] **ADDRESSED (verified)** — the row pass is composed exactly as the build composes it and the
  stratum is gated on `SourceIr::carries_canonical_source_classifications`. Corpus: **183 persisted /
  137 reproduced / 46 not-reproduced**, i.e. +12 persisted and +12 reproduced — **all 12 row records
  reproduce**, which re-derives without a rebuild what `.3k.2d` had to rebuild APB to establish, and
  extends it to AHB and AXI-L. The statement and dynamic strata are byte-identical to the baseline,
  compared record by record. **Three observed-RED controls, one per decision:** (a) suppressing the row
  pass while still judging the stratum makes
  `the_row_stratum_is_judged_only_when_its_producer_can_run` fail, reporting a live record as lost;
  (b) removing the schema gate makes the corpus report claim **77 documents judged instead of 26**,
  including LTI, whose producer sees nothing — the silent zero itself; (c) neutralizing `table_kind` on
  an otherwise identical `SourceIr` empties the row producer's output in
  `a_legacy_source_ir_is_not_a_classification_authority`. All restored green.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings`
  and the whole workspace suite green; `specforge-core` lib **1,461 → 1,464**. `kg-bench` **156/156**.
  WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000` on APB/AHB/AXI/SWD, `temporal_rule 1.000` on
  APB/AHB/AXI. `scripts/check_doctrines.sh` all 13 gate-tier PASS. `flow_census.json` re-derived and
  attributed to this leaf (+2 analyzed functions, +9 decision sites, +6 helper edges, +1 semantic
  macro); the workspace's own `current_repository_flow_is_complete_local_and_deterministic` caught the
  drift before the driver did, which is the second gate working.
- [x] **GENERICITY (ADR 0006)** — no document, protocol, vendor or signal name enters the production
  path; the gate is a schema comparison and the composition mirrors the build's. The controls build
  their own `SourceIr` with invented names (`ZETAREADY`, `ZETASELX`). AMBA LTI is named only in a
  comment and in this leaf, as the measured instance.
- [x] **LOCKSTEP** — code, this leaf, `.3k.2e`/`.3k.2i` (whose populations this leaf withdraws), the
  book's EvidenceIR chapter, `TOOLBOX.md` §5.5, and the resume pointer agree before commit. No
  production rule is deleted or replaced. Knowledge Map card
  `[[legacy-source-classifications-are-neutralized-on-load]]` records the mechanism, because a pass
  keyed on any typed source classification hits it and the empty result looks like an answer.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2d`

- [x] **REPRODUCE / MEASURE** — the population enumerated in both directions over all 78 persisted
  artifacts, as the full cross product of the six modals `obligation_is_negated` accepts with the
  binder verbs `extract_protocol_state_value` reads, not the four spellings the leaf opened with
  (which gave 2 and was wrong). Statement path: **31** `signal_value_constraint` statements carry a
  negated binder — 13 `must_not_change`, 6 `must_be_deasserted`, **12 generic/untyped**. All 12 read
  against source: 9 `cannot/will not be changed`, 2 DTI reference magnitudes (`.3k.1` refuses them),
  1 AMBA LPI *"QREQn cannot be driven HIGH until the handshake is completed"*. Row path: 50 admitted
  obligation clauses, 0 carrying one of these modals. Baseline `replay-constraints --evidence-root
  generated/evidence_ir`: 171 persisted / 125 reproduced / 46 not / 126 unpersisted, captured per
  record before the change.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`. Three functions read one
  clause for its modal with three vocabularies: `obligation_is_negated` (six modals),
  `constraint_bearing_sentence` (`must`/`shall` only, so an obligation stated with `cannot` gets no
  sentence and the span silently falls back to the whole statement), and
  `classify_signal_constraint_kind` (every phrase spelled `must`/`shall`, so the same clause types as
  nothing and — since `.3k.2a` — publishes nothing). Not the mechanism the leaf named: the record it
  cited, RISC-V IOMMU `dyn_sigcon_0008`/`0009`, comes from `statement_1033`, class `conditional_rule`,
  through the dynamic path, which does not call this classifier. Verified by locating the statement
  and its records in the artifact, not by reading the leaf.
- [x] **ADDRESSED (verified)** — `normalize_obligation_modal` reduces `cannot`/`can not`/`will not`/
  `must never`/`shall never` to `must not`; `sentence_states_an_obligation` gives the sentence scan the
  same set. **Corpus effect measured record by record over all 78 artifacts: 0 added, 0 removed, 0
  retyped** — the per-record `replay-constraints` dump is byte-identical to the baseline. APB rebuilt
  (`evidence → validate → semantic → validate → intent → validate → adapt`): 23 constraint records,
  identical identities AND identical ids, all 10 `row_sigcon_*` included; only `proof_context`,
  `proof_ledger` and `validation_reports` differ, and the restored bundle was `diff -r` byte-identical
  before removal (retention back at 24). **Two observed-RED controls, one per decision, each isolating
  the other:** disabling `normalize_obligation_modal` fails
  `the_equivalent_negative_modals_reach_the_same_kinds` with `MustBeStable` where `MustNotChange` is
  correct (and `a_self_negating_kind_still_refuses_the_flag_under_any_modal` with it); narrowing
  `sentence_states_an_obligation` back to `must`/`shall` fails
  `the_obligation_sentence_is_found_by_the_same_modal_the_kind_is`, returning the whole serialized row
  where the obligation's own sentence is correct. Both restored green.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, `cargo clippy --offline --all-targets -D warnings`
  and the full suite green; `specforge-core` lib **1,456 → 1,461**. `kg-bench` **156/156**.
  WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000` on APB/AHB/AXI/SWD and `temporal_rule
  P=R=F1=1.000` on APB/AHB/AXI. `scripts/check_doctrines.sh` all 13 gate-tier PASS after
  `doctrine/production_genericity/flow_census.json` was re-derived and attributed to this leaf
  (`analyzed_functions` +2, `helper_edges` +3, `decision_sites` −1 — the inline disjunction
  `constraint_bearing_sentence` spelled becomes one `contains_any` call). Corpus replay unchanged at
  171 / 125 / 46 / 126.
- [x] **GENERICITY (ADR 0006)** — universal English modal equivalence. Five modal strings and one
  canonical form, no document, protocol, vendor or signal name; the set is exactly
  `obligation_is_negated`'s, so the two halves of a record cannot again recognise different modals.
  Controls use invented names (`ZETALEN`, `ZETAOKAY`, `ZETARESP`, `ZETASEL`, `ZETADATA`, `ZETASTRB`,
  `ZETANOTE`).
- [x] **LOCKSTEP** — code, this leaf, the book's EvidenceIR chapter and the resume pointer agree before
  commit. No production rule is deleted or replaced, so no book text describes behaviour that is now
  gone; the chapter gains the modal rule beside `.3i`'s negation rule and `.3k.2c`'s phrase rule.
  Knowledge Map card `[[one-modal-vocabulary-per-constraint-record]]` records the measured zero so the
  next session does not re-derive it.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2c`

- [x] **REPRODUCE / MEASURE** — 56 persisted statements contain *"have/has the same value"*; only 6
  deterministic records come from one: APB `sigcon_0009`/`0010` (typed by the validity arm, correct)
  and `row_sigcon_0018`/`0019`/`0021`/`0022` (the untyped fallback, published as `must_be_stable`).
  The 4 row records were read against source: each states that a signal holds one value across two
  phases or across every cycle of one, which is a no-change obligation.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `classify_signal_constraint_kind`: the phrase table carries `must not change`, `must remain stable`,
  `must be stable` and `must hold`, and no form of *"must have the same value"*. The obligation is
  real and the reader had no spelling for it, so it fell through to the terminal arm.
- [x] **ADDRESSED (verified)** — a new arm for `must have the same value` / `shall have the same
  value` → `MustNotChange`, placed AFTER the validity arm. APB rebuilt
  (`evidence → validate → semantic → validate → intent → validate → adapt`): **4 records retyped, 0
  added, 0 removed**, and `sigcon_0009`/`0010` keep `must_be_value VALID`. **Two observed-RED
  controls, one per decision:** removing the phrase makes
  `the_same_value_spelling_types_as_no_change` fail with `MustBeStable`, and moving the arm ahead of
  the validity arm makes `a_cell_stating_both_obligations_keeps_its_validity_kind` fail with
  `MustNotChange` where `MustBeValue { value: "VALID" }` is correct. Both pass restored.
- [x] **NO REGRESSION** — `kg-bench` **156/156**; WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000`
  on APB, AHB, AXI and SWD. `cargo fmt --all --check`, `cargo clippy --all-targets -D warnings`,
  `cargo test` green; `specforge-core` lib 1,453 → 1,456. `replay-constraints` corpus unchanged at
  171 persisted / **125** reproduced / 46 not-reproduced / 1 named skip — the retype is inside the
  row stratum, which the replay does not judge, and it moved nothing in the two it does. Retention
  back at **24**, APB's bundle `diff -r`-verified byte-identical before removal.
- [x] **GENERICITY (ADR 0006)** — two phrase strings of ordinary English, each carrying its modal so a
  descriptive *"implementations that have the same value"* is untouched. No document, protocol, vendor
  or signal name; the controls use invented names (`ZETAUSER`, `ZETASELX`, `ZETAWUSER`, `ZETAPMCR`).
- [x] **LOCKSTEP** — code, this leaf, the book's EvidenceIR chapter, and the resume pointer agree
  before commit. `.3d`'s equality refusal is asserted to keep NOT firing on *"the same value IN …"*
  while still firing on *"the same value AS …"*, so the two rules stay distinguishable.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2b`

- [x] **REPRODUCE / MEASURE** — the reproduced `generic_value` population is 4 `sigcon_*` records, all
  read against source: AXI `AWTAGOP must_be_value INVALID` (*"- AWTAGOP must be Invalid."*, correct),
  HBM2 `CKE must_be_value LOW` (*"CKE must be held LOW"*, correct), RISC-V IOMMU `GSCID` and DTI
  `DO_NOT_CACHE` (both `INVALIDATED` from *"must be invalidated"*, fabricated). A fifth instance,
  AXI `WTAGUPDATE must_be_value UPDATED` from *"the tags in memory must be updated"*, surfaced during
  the rebuild and is recorded as this leaf's residual rather than absorbed.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `classify_signal_constraint_kind`'s generic arm: `extract_protocol_state_value` returns the first
  non-filler word after a binding lead and the arm publishes it as a typed value with no test of what
  the word is. In a passive obligation that word is the obligation's verb.
- [x] **ADDRESSED (verified)** — `is_admissible_state_value(value, discovered_values)`: the document's
  own declared enum members first, then logic levels, then numeric literals, and otherwise a refusal
  of the past-participle shape only. Both deterministic call sites now derive the document's value
  vocabulary with `collect_discovered_enum_values`, the same set the dynamic path already binds
  against. **Two measurements steered the design and both are recorded above**: gating every value arm
  retyped 13 APB + 4 AHB correct records (reverted), and gating the generic arm against discovered
  values alone lost `AWTAGOP … Invalid` (narrowed). **Observed RED** with the admissibility test
  removed: `a_passive_participle_in_the_value_slot_is_not_a_value` emits
  `Some("must_be_value:INVALIDATED")` — the live `GSCID` defect on invented names — and
  `a_participle_the_document_declares_as_a_value_is_admitted` fails on its negative half; both pass
  restored.
- [x] **NO REGRESSION** — the chain was rebuilt for all three documents with a held-out bundle (APB,
  AHB, AXI-L): **0 records removed, 0 added, 0 retyped** in the final shape, and each bundle
  `diff -r`-verified byte-identical before removal, retention back at **24**. `kg-bench` **156/156**.
  WIRE-BASED-100 golds `signal_constraint P=R=F1=1.000` on APB, AHB, AXI and SWD.
  `cargo fmt --all --check`, `cargo clippy --all-targets -D warnings`, `cargo test` green;
  `specforge-core` lib 1,449 → 1,454. `replay-constraints` corpus: 171 persisted, **125** reproduced
  (was 127 — exactly the two fabrications), 46 not-reproduced, 1 named skip.
- [x] **GENERICITY (ADR 0006)** — universal English participle grammar plus the document's own
  declared vocabulary. No value list, no protocol, vendor, document or signal name; the controls use
  invented names (`ZETAGSCID`, `ZETATAGOP`, `ZETASTATE`, `ZETACKE`, `ZETAUSER`, `ZETASEL`) and an
  invented enum (`ZETASTATES Shared = 0`).
- [x] **LOCKSTEP** — code, this leaf, the book's EvidenceIR chapter, and the resume pointer agree
  before commit; the `WTAGUPDATE` residual is written into `.3k.3` rather than left in a comment.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.2a`

- [x] **REPRODUCE / MEASURE** — sized with `replay-constraints` FIRST, per the container's amendment.
  The published untyped-default population is 26 `sigcon_*`; the REPRODUCED population is **17**, and
  all 17 were read against their own source text. 17 of 17 are wrong: a barrier-transaction
  description (CoreSight ×2), a presence table cell (AXI ×4), a tied-low level statement (AXI ×1),
  two explicitly non-required recommendations (APB ×2), a response-duration description (AHB ×1), and
  six waveform narrations (LPI ×2, GFB ×3, HBM2 ×2 — `- T1 FREADY signal remains HIGH`,
  `At T3 … QDENY remains LOW`, `AERR is driven HIGH for 1 tCK`). The same census over the ROW stratum
  is the control that shaped the fix: its 4 untyped-default records are `PAUSER`/`PWUSER must have the
  same value in the Setup and Access phase`, which are CORRECT and merely under-typed.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`,
  `classify_signal_constraint_kind`'s terminal `else` arm: when no phrase matches and no value binds,
  it returned `SignalConstraintKind::MustBeStable`. In the statement path that arm fires on any
  sentence the classifier happened to class `SignalValueConstraint`, so a presence cell or a waveform
  step became an assertion that a signal must be stable. The row path reaches the same arm only after
  `obligation_subject` has proved the clause binds to its row's signal, which is why the same default
  is honest there and a fabrication here.
- [x] **ADDRESSED (verified)** — `classify_signal_constraint_kind_typed` returns `None` exactly for
  that arm (no stability or validity phrase anywhere in the clause) and `extract_signal_constraints`
  refuses on `None`; `classify_signal_constraint_kind` is byte-unchanged, so the row path is untouched.
  **Observed RED** with the call reverted: `a_statement_that_names_no_kind_yields_no_constraint` emits
  two `MustBeStable` records from `| Manager: False | ZETACHUNKEN is not present. …`, and
  `a_recommendation_that_names_no_kind_yields_no_constraint` emits one from
  `It is recommended, but not required, that ZETASLVERR is driven LOW …`; both pass restored.
  Controls hold: `must be stable`, `must be asserted` and `must not change` still type, and
  `classify_signal_constraint_kind` still answers `MustBeStable` for the row path's `must have the
  same value …` while the typed variant answers `None`.
- [x] **NO REGRESSION** — **chain rebuilt for all three documents whose proofs the change stales**
  (APB, AHB, AXI-L; each has a held-out bundle):
  `evidence → validate → semantic → validate → intent → validate → adapt --target isf`, zero failures,
  **8 records removed, 0 added, 0 retyped** — AXI 45→40, APB 25→23, AHB 14→13 — and every removal is
  one of the 17 adjudicated records. `kg-bench` **156/156**. WIRE-BASED-100 golds unchanged at
  `signal_constraint P=R=F1=1.000` for APB, AHB, AXI and SWD, and `actor_signal_relation` 1.000 on all
  four filtered. `cargo fmt --all --check`, `cargo clippy --all-targets -D warnings`, `cargo test`
  green; `specforge-core` lib 1,445 → 1,449. Retention back at the declared **24** bundles, each
  `diff -r`-verified byte-identical against its held-out copy before removal; corpus frontier
  `57 = 52 + 5` unchanged. Pre-rebuild snapshot at
  `generated/preserved/EXTRACTION-QUALITY-GAUGE.3k.2a/pre-rebuild/`.
- [x] **GENERICITY (ADR 0006)** — one negative predicate over the existing phrase table. No document,
  protocol, vendor or signal name; controls use invented names (`ZETACHUNKEN`, `ZETACHUNKV`,
  `ZETASLVERR`, `ZETASEL`, `ZETAADDR`, `ZETASTRB`, `ZETAUSER`).
- [x] **LOCKSTEP** — code, this leaf, the book's EvidenceIR chapter, and the resume pointer agree
  before commit. The two findings the reading produced are not absorbed: the `generic_value` arm is
  2-2 and owned by `.3k.2b`, the row path's missing spelling by `.3k.2c`.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.6`

- [x] **REPRODUCE / MEASURE** — `cargo run -- replay-constraints --evidence-root generated/evidence_ir`:
  `documents_scanned: 78`, `documents_skipped: 1`, `persisted_deterministic_records: 179`,
  `reproduced: 144`, `not_reproduced: 35`, `granted_declarations: 67`. Ten partial documents and one
  fully frozen (`ihi0079_b … amba_cxs 0/2`). By refusing gate: 17 none, 12 `CORPUS-COVERAGE.2.50a`,
  4 `.3k.1`+`.2.50a`, 1 `.3h` value-position, 1 `.3g` dotted-cross-reference.
- [x] **ROOT CAUSE (WHY + WHERE)** — the question had no instrument. `crates/specforge/src/ir/evidence.rs`
  exposed no way to ask a persisted record whether it still comes out, and `generated/` cannot answer
  it: 54 of 78 documents have no normalized bundle, so their artifacts cannot be rebuilt and freeze at
  the generation that wrote them. `.3k.1` had to establish that by hand, one probe per record.
- [x] **ADDRESSED (verified)** — `replay_persisted_signal_constraints` in `ir/evidence.rs` plus the
  `replay-constraints` command. **Calibrated in both directions against artifacts whose generation is
  known:** APB 15/15 and AHB 13/13 — both rebuilt by `.3i`, so a current artifact reproduces
  completely — and AMBA CXS 0/2, whose two records are exactly the `must not be asserted` shape `.3i`
  retyped, so a known-stale artifact reproduces nothing. Two independent confirmations that the
  instrument measures generation drift rather than noise. Fidelity control run separately: every
  persisted record's `source_text` and `supporting_statement_ids` resolve inside its own artifact
  (0 orphans corpus-wide), so a NOT-REPRODUCED verdict is never a missing-statement artifact.
  **Observed RED, and it changed the published number:** the first widening seeded the catalog
  `HashSet` the producer takes, which only the inference-antecedent sibling reads;
  `a_subject_no_statement_declares_is_still_granted_its_trial` failed, and fixing it to insert
  declaration STATEMENTS moved the corpus figure 120/179 → 144/179.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, `cargo clippy --all-targets -D warnings`, and
  `cargo test` green; `specforge-core` lib 1,441 → 1,445, no existing expectation changed.
  `scripts/check_doctrines.sh` green. Read-only by construction: the command never writes, and it
  loads through `load_for_inspection`, so no artifact's proof context moves.
- [x] **GENERICITY (ADR 0006)** — no document, protocol, vendor or signal name in the producer or the
  command; the strata are selected by `constraint_id` prefix and the controls use invented names
  (`ZETASTRB`, `ZETAOAS`, `ZETADTI`, `ZETAKEEP`).
- [x] **LOCKSTEP** — code, this leaf, `[[persisted-census-measures-published-not-current]]` (whose
  `reverify` is now this command), the book's command pages, `TOOLBOX.md` §5.5 and the chooser row,
  and the resume pointer agree before commit.

### Acceptance Checklist (enforced) — `EXTRACTION-QUALITY-GAUGE.3k.1`

- [x] **REPRODUCE / MEASURE** — two censuses, and the second one overturned the first.
  (1) `python3 scripts/measure_constraint_part_span.py` over the 78 persisted artifacts: the
  `reference-operand magnitude` row is **4** in the `sigcon_*` stratum and **0** in `dyn_sigcon_*`,
  `row_sigcon_*` and `llm_sigcon_*` — AMBA DTI `sigcon_0002`–`0005`, publishing
  `OAS must_be_stable, negated` and `DTI must_be_stable, negated`, i.e. *"OAS must not be stable"*.
  The same four are the ENTIRE `negation on untyped default` population, so the two halves `.3k`
  inherited as "7 plus 4" are one set of 4.
  (2) **Running the real producer on that exact sentence yields `records=[]`.** A probe calling
  `extract_signal_constraints` with `OAS`/`DTI` declared reported
  `PROBE records=[] … post_passive OAS=true DTI=true`: `CORPUS-COVERAGE.2.50a` already refuses both
  subjects because the sentence names them only after `must not be`. The four records predate that
  gate, and DTI has no retained normalized bundle, so nothing has rewritten them
  (`[[persisted-census-measures-published-not-current]]`).
  (3) The class is reachable anyway — a third probe on the same grammar with the subject named BEFORE
  the lead returned `ZETARANGE MustBeStable negated: true`, and on the dynamic path
  `ZETARANGE MustBeLow` plus `ZETAOAS MustBeLow`. That is what this leaf gates.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`.
  `is_relational_equality_constraint` (`.3d`) expresses the refusal for an inter-operand EQUALITY and
  nothing for a comparative MAGNITUDE, so a bound stated against another operand reaches the kind
  classifier, matches no phrase in its table, falls to the terminal `MustBeStable` default, and then
  carries the sentence's `must not` on top — a record asserting that a signal must not be stable,
  which no sentence of this shape says. The right operand is additionally minted as a second subject
  (`ZETAOAS must_be_low` above; `OAS` in the live artifact).
- [x] **ADDRESSED (verified)** — pure `is_reference_magnitude_constraint(text)`: a comparative marker
  (`greater than `, `less than `, `larger than `, … each carrying its trailing space) IMMEDIATELY
  followed by a phrase naming another operand's attribute (`the value of`, `the size indicated by`,
  `the number of`, `that indicated by`, …). Wired beside the `.3d` refusal in BOTH deterministic
  extractors. **Observed RED with both call sites reverted:**
  `a_magnitude_against_a_referenced_operand_yields_no_constraint` fails emitting
  `MustBeStable, negated: true` from *"ZETARANGE must not be greater than the size indicated by the
  ZETAOAS field"*, and `…_yields_no_dynamic_constraint` fails emitting two `MustBeLow` records; both
  pass restored. The literal controls hold in both directions: *"The value of ZETARANGE must be
  greater than 0"* still extracts, and the HBM2 shape *"sets ZETADBI HIGH when the number of
  transitioning data bits within a byte is greater than 4"* still yields exactly `must_be_high`.
- [x] **NO REGRESSION** — `cargo fmt --all --check` green; `cargo clippy --all-targets -D warnings`
  green; `cargo test` green with `specforge-core` lib **1,435 → 1,440** and no existing expectation
  changed. `scripts/check_doctrines.sh` green (13/13 executed, 2 CI-tier deferred). No persisted
  artifact moves: the only corpus instance is in a document with no normalized bundle, so there is
  nothing to rebuild and nothing to diff — stated as a limit, not as coverage, exactly as `.3h` did
  for NVMe `FFFF`.
- [x] **GENERICITY (ADR 0006)** — two phrase lists of ordinary English comparatives and reference
  leads. No document, protocol, vendor, register or signal name appears in the rule; every control
  uses invented names (`ZETARANGE`, `ZETAOAS`, `ZETADTI`, `ZETADBI`).
- [x] **LOCKSTEP** — code, this leaf, the two Knowledge Map cards, the book's EvidenceIR gate
  narrative, and the resume pointer agree before commit. The refusal's own span defect is not
  silently inherited: it is written down and owned by `.3k.5`.

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

- `2026-09-13` — **`.3k.2j` CLOSED (DOCTRINE) — the check `.3k` wrote is now a check anything runs.**
  `scripts/measure_constraint_part_span.py --check` exists to fail closed when the kind classifier's
  call-site topology moves, is named in `.3k`'s verification, and was in no driver. Re-derived per
  revision with each revision's own scanner it was green at `.3k.1`, RED from `.3k.2a` — the typed
  gateway — and still RED at `.3k.2c`: three leaves reported a green gate over a failing check.
  Registered as `CONSTRAINT-PART-SPAN|gate` with its §10 row; the driver now executes 14 doctrines and
  was observed RED on both drifts the script guards. The sibling enumeration is classified rather than
  counted: 86 script-shaped files, 28 unreachable from any driver or doctrine registry, and exactly 2 of
  those claim `--check` semantics — this one and `validate_canonical_recovery_contract.py`, whose
  ownership `RETAINED-BUNDLE-POPULATION-FROZEN` already has open. The first pass said 33 and was wrong,
  by missing that a doctrine registry invokes a verifier by `argv` rather than by a shell reference.
- `2026-09-13` — **`.3k.2e` CLOSED (CODE) — proving the subject does not make the kind readable.** The
  table-row reader now goes through `classify_signal_constraint_kind_typed` too, so the terminal
  `MustBeStable` is unreachable as a published kind by any producer. `.3k.2a`'s asymmetry is superseded
  by its own successor: it rested on four APB `must have the same value` clauses, and `.3k.2c` typed
  them. Corpus effect **zero** (183/137/46 unchanged; all 12 live row records are typed by an arm the
  document wrote), so the class is demonstrated through the real reader rather than counted — a MATCH,
  an ALIGNMENT and a PRESENCE claim each publish `must_be_stable` without the refusal, the last one
  negated. **Found in passing and owned as `.3k.2j`:** `scripts/measure_constraint_part_span.py --check`
  is `.3k`'s own declared verification and is in NO driver — re-derived per revision with each
  revision's own scanner, it has been RED since `.3k.2a`, and three leaves shipped over it. Repaired
  here (it now tracks the typed gateway and pins that the untyped arm has exactly one caller) plus a
  self-test case for the `#[cfg(test)]` scope bug that re-derivation exposed; wiring it into a driver is
  `.3k.2j`.
- `2026-09-13` — **`.3k.2f` CLOSED (CODE) — the two vocabulary-slot refusals reach the row path.**
  `.3d`'s inter-operand equality and `.3k.1`'s comparative magnitude both argue that the vocabulary has
  no slot for the sentence, and both were wired into `extract_signal_constraints` alone — so one
  sentence was refused as a statement and published as a table row. Corpus effect is **zero** (183 /
  137 / 46 unchanged, no admitted row clause matches either predicate in the 26 judgeable documents),
  so the leaf ships on `.3k.1`'s footing and the class is demonstrated instead of counted: through the
  REAL row reader, the guard's removal republishes `ZETARANGE must_be_stable, negated` — *"must not be
  stable"*, the exact AMBA DTI fabrication `.3k.1` closed — and `OMEGABURST must_be_value VALUE`, the
  value lifted out of the phrase *the value of*. `.3k.2e` was listed as its prerequisite and the
  requirement is waived with a reason: that leaf refuses an UNTYPED kind, this one refuses a clause
  whose kind types perfectly well, so neither reaches the other's population.
- `2026-09-13` — **`.3k.2g` CLOSED (CODE) — the replay judges the third producer, and its first act was to
  withdraw a population `.3k.2d` published one commit earlier.** `replay-constraints` filtered the
  judged set to `sigcon_*` and `dyn_sigcon_*`, so every corpus figure this family quoted was blind to
  the 12 `row_sigcon_*` records; it now takes the document's own `SourceIr` and composes the row pass
  the way the build does. **171 → 183 persisted, 125 → 137 reproduced, `not_reproduced` unchanged at
  46: all 12 row records reproduce**, re-deriving without a rebuild what `.3k.2d` had to rebuild APB to
  establish. The half that had to be measured: a LEGACY `SourceIr` loads with every `table_kind`
  neutralized to `Unknown`, so the row producer selects no table and returns an empty result
  indistinguishable from *"this document states no row obligation"* — AMBA LTI marks 25 tables
  `signal_description` in its persisted JSON and passes **0 of 88** after a legacy load. The stratum is
  therefore gated on the schema and the report publishes **26 judged / 51 not**. That same fact
  falsifies `.3k.2e`'s *"17 of 17 wrong"* and `.3k.2i`'s *"4 right / 5 wrong"*: both were mirrored off
  the persisted `table_kind`, and the real producer mints 12 row records corpus-wide and not one more.
  Both nodes are re-sized in place; the readings stand as estimates for the 51 legacy documents.
- `2026-09-12` — **`.3k.2d` CLOSED (CODE) — one modal vocabulary for the whole record, and the
  mechanism the leaf opened with was refuted by its own census.** The leaf said
  `extract_protocol_state_value` has no negated form, citing *"The DV operand must not be 1 for
  IODIR"*; that is a TEST STRING, and the document's own statement is RISC-V IOMMU `statement_1033`,
  class `conditional_rule`, whose records come from the DYNAMIC path and never reach this classifier.
  Enumerating the real population — the full cross product of the six modals `obligation_is_negated`
  accepts with the binder verbs — gives **31** statements, of which **12** reach the untyped arm; all
  12 read against source, and **the negated value binder is needed by none of them**. Ten need the
  MODAL: every phrase in the kind table is spelled `must`/`shall` while the negation detector accepts
  `cannot`/`will not`/`must never`/`shall never`, so one clause was read with two vocabularies.
  `constraint_bearing_sentence` was the third reader and the narrowest, which is why the two halves
  ship together: with only the classifier taught, the corpus gains 2 records whose subject is a
  serialized row's `NOTE` marker; with both, it gains none. **Measured over all 78 artifacts, record
  by record: 0 added, 0 removed, 0 retyped**, and APB rebuilt to the same 23 records with the same
  ids. Five findings routed to `.3k.2e`–`.3k.2i` rather than absorbed, the largest being that the row
  path's untyped fallback — kept deliberately by `.3k.2a` — is **17 of 17 wrong** once `.3k.2c`
  removed the four records `.3k.2a` reasoned from.
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
