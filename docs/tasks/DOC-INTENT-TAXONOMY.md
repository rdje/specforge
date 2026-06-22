# DOC-INTENT-TAXONOMY: chip-spec document intent taxonomy → per-category complete ISF synthesis

## Metadata

- Tree ID: `DOC-INTENT-TAXONOMY`
- Status: `active` (`.0` taxonomy DONE `2026-06-22`; `.1` corpus census DONE `2026-06-22`, read-only; `.2` per-category ISF-completeness gauge DONE `2026-06-22`, read-only; `.3` fast category recognizer COMPLETE `2026-06-22` — `.3a` design / `.3b` implement+validate-reported `d6239217` / `.3c` fixtures+book+KM; `.4a` Gap A — register bit-field lowering: empirical FSMGen-storage verification + verified FSMGen FR DONE `2026-06-22`, docs-only; **`.4a.ii` DONE `2026-06-22` (CODE) — emitted the register bit-field map into the shipped ISF field-structured-storage construct (pin `d327129b7`): 6,570 fields / 2,531 registers / 24 docs now reach `.isf` (was 0), 0 new FSMGen `--strict` diagnostics, 4 wire golds byte-identical**; **`.4d` DONE `2026-06-23` (decision packet, docs-only) — cat-4 CSRs REUSE the existing register/storage abstraction (no new ISF construct, FSMGen titles `(storage …)` the register-map/CSR construct); the cat-4 register gap is EXTRACTION RECALL (RISC-V Debug 179 fields all UNLOCATED, AIA 0 registers captured) → spun out `.4d.i`; instruction/privilege/exception/memory-ordering are honest non-targets**; **`.4c` DONE `2026-06-23` (decision packet, docs-only) — cat-3 topology IS captured (`signal_connectivity` producer→consumer graph + `infrastructure_signals` clock/reset distribution; correcting `.2` "hint-level" to "captured-but-sparse-and-unlowered") but ISF has NO declarative static-topology construct (composition is transaction-level only; the emit is single-initiator-actor) and FSMGen's ATL frontier is behavioral, not a declarative netlist; capture is sparse/noisy → NO FR yet, next = `.4c.i` capture-recall measurement**; **`.4e` DONE `2026-06-23` (triage, docs-only) — the `conditional_rules` ISF-lowering shortfall (`.2` Result 3) is HONEST RESIDUAL, not a gap: 603 rules across 9 docs = 73% prose/undeclared/placeholder + a 27% bucket that is 161/164 bare deontic modals (only 3/603 carry a concrete obligation); no ISF lever, no FR — the only upside is upstream extraction quality. The `.2` measurement phase is now COMPLETE; remaining `.4` work is CODE**; frontier → `.4c.i` cat-3 topology-capture recall (measurement) / `.4d.i` cat-4 RISC-V CSR bit-position recovery (code) / `.4b` Gap B (gated, FSMGen-deferred packet/flit); `.4a.i` superseded by `.4a.ii`)
- Roadmap lane: `R15`/`R16` (north star: COMPLETE IntentIR → FAITHFUL ISF, now made explicit **per document category**)
- Created: `2026-06-22`
- Last updated: `2026-06-22`
- Owner: repo-local workflow
- Owner directive (`2026-06-22`, multi-message): every chip-spec PDF is *about* something — there is a small set of
  **intent categories / purposes** (protocol spec, platform/system-IP, CPU ISA, …). SpecForge must (1) understand which
  category a given PDF is in — *quickly*, (2) extract the necessary information → IntentIR, and (3) **lower EVERYTHING
  (constraints, relations, registers, structures, …) to ISF — ISF is the way to synthesize the PDF's intent**. *In fine,
  all categories must be FULLY handled.* Quality + accuracy + speed; no hurry, SOTA-level. The abstraction "what is this
  PDF about" is a **guiding lens** — capture it in the mdBook and here. Reinforces `[[project_kg_isf_completeness]]`.

## Goal

Define and operationalize the **purpose taxonomy** of chip-spec PDFs, and drive **every** category to a COMPLETE
IntentIR that lowers **completely and elegantly** to ISF:

1. a precise, stable **6-category taxonomy** (below), captured identically in the mdBook and this tree, used as the
   guiding lens for "what is complete intent for *this* document";
2. **fast, deterministic category recognition** so an operator (and the pipeline) can immediately tell which category a
   PDF is in;
3. a **per-category ISF-lowering completeness** program: measure honestly what fraction of each category's intent reaches
   `.isf`, attack the gaps category by category, and — where the current FSMGen ISF lacks the abstraction to capture a
   category naturally/elegantly — **feed the gap back to FSMGen** (it is adding a verification-oriented SV/UVM + VHDL
   lowering path alongside the default synthesizable HDL; both paths will want richer ISF abstractions such as memory
   banks and single/dual-port memory modules — owner-provided roadmap context, `2026-06-22`).

## The taxonomy (the guiding lens — what a chip-spec PDF is *about*)

Each PDF has a single **dominant** purpose. (A document may carry secondary surfaces — e.g. a protocol spec with an
appendix register map — but it classifies on its dominant intent.) The categories, the *intent shape* each carries, the
corpus examples, and the **honest** current ISF-synthesis maturity:

| # | Category (what it is *about*) | The "intent" SpecForge must capture = | Corpus examples | ISF-synthesis maturity (honest, `2026-06-22`) |
|---|---|---|---|---|
| **1** | **Wire-level bus / interconnect protocol** | signals (dir/width), transactions, handshake/temporal rules, actor↔signal relations, polarity | APB, AHB, AXI, AXI-Stream, ACE, CHI, TileLink, Avalon, Wishbone, OCP, CXS/GFB/LTI/DTI/ATP/LPI, OpenCAPI, CCIX, USB, I²C, I²S, CAN, SMBus, SWD | **MATURE** — IntentIR maps 1:1; ISF lowers richly; WIRE-BASED-100 = 1.000 |
| **2** | **Programmable register / memory-mapped IP** | register maps, bit-fields, access/reset, in-memory STRUCTURES (descriptors, queues, page tables, contexts) | RISC-V IOMMU, AMD-IOMMU, Intel VT-d, GIC arch, SMMU/MMU-700, CoreSight TRMs, NVMe, JEDEC eMMC EXT_CSD | **PARTIAL** — registers → ISF storage/reset + field-signals; **frontier = structure / message-field table recall for non-AMBA styles** (the `CORPUS-COVERAGE.2` #21 IOMMU Lever-D gap) |
| **3** | **Platform / system-IP topology & integration** | components, connectivity, clock/reset infrastructure, programming model, integration contract | CoreSight SoC-600, GIC distributor/redistributor, interconnect fabrics | **PARTIAL** — infra signals + actor ports lower; topology stays hint-level |
| **4** | **CPU ISA / privileged architecture** | instructions, CSRs/registers, privilege modes, exceptions, memory-ordering model | RISC-V Debug, RISC-V Advanced Interrupt Architecture, ISA volumes | **THIN** — only partly wire-shaped; least-developed ISF story (likely an ISF feature request / dedicated lowering) |
| **5** | **Physical / electrical / link layer** | signaling levels, encoding, link training, mechanicals | OpenCAPI 25G/32G PHY, USB4 PHY, mechanical specs | **Honest-thin** — not behavioral wire intent → correctly near-empty `.isf` (not a gap) |
| **6** | **Methodology / language / EDA standard / guide** | NOT a chip contract — verification/modeling methodology, HDL/RDL languages, overview guides | UVM, SystemC/TLM, IP-XACT, SystemRDL, OVL, PSS, Liberty, LEF/DEF, overview/user guides | **Non-target** — the honest `guide` class; recognize and never force chip intent |

**Relationship to the existing `document_class`** (`protocol`/`register`/`interface`/`guide`, PDF-VARIANT-DIGESTION.5a):
that surface is a **coarse 4-way structural proxy** inferred from which typed surfaces appeared. The purpose taxonomy is
**richer and semantic**: it folds 1+5 into "protocol", 2+3 into "register"/"interface", has **no home for category 4
(ISA)**, and sends 6 to "guide". This tree builds *on* `document_class` (it is genuine signal), not in place of it.

**ISF-synthesis north star, per category:** "ISF is the way to synthesize the PDF intent" ⇒ for every category, IntentIR
must be COMPLETE and lower FULLY to ISF. The maturity column above is the **honest scorecard** to be made measurable in
`.2`. Categories 5 and 6 are honest non-targets (their thin `.isf` is correct, not a failure); the buildable program is
1 (hold), 2 (structure recall), 3 (topology), 4 (ISA) — plus the FSMGen-ISF abstraction feedback.

## Non-Goals

- Not re-deriving or replacing the structural `document_class` — this tree consumes it as one signal.
- Not building `.fsm`/HDL/SV/UVM/VHDL — FSMGen owns all lowering downstream of `.isf`; this tree only ensures the `.isf`
  SpecForge emits carries each category's intent, and files ISF-abstraction feature requests to FSMGen where needed.
- Not forcing chip intent out of category-6 methodology/guide docs (they classify honestly as non-targets).
- No vendor / chip-name lists anywhere — recognition is structural/behavioral (ADR 0006).

## Acceptance Criteria

- The 6-category taxonomy is captured identically in the mdBook (`document-categories.md`) and this tree.
- A deterministic, fast category recognizer exists and is reported by the CLI (`.3`).
- A per-category ISF-lowering completeness gauge exists, with an honest scorecard (`.2`).
- Each category's buildable gap has an owned leaf; FSMGen ISF-abstraction gaps are filed as verified FRs.
- Every completed leaf is committed through `COMMIT.md`; memory-arch + knowledge-map gates green.

## Task Tree

- ID: `DOC-INTENT-TAXONOMY` · Status: `active` · Goal: 6-category purpose taxonomy → per-category complete ISF synthesis ·
  Children: `.0` (taxonomy + capture, done), `.1` (corpus census), `.2` (per-category ISF-completeness gauge),
  `.3` (fast category recognizer, code), `.4+` (per-category levers + FSMGen feedback).
- ID: `DOC-INTENT-TAXONOMY.0` · Status: `done` (`2026-06-22`, docs-only) · Goal: define the 6-category taxonomy precisely
  and capture it as the guiding lens in BOTH the mdBook and this tree, with the honest per-category ISF-maturity
  scorecard and the owner's FSMGen dual-path + new-abstraction context. **DONE:** taxonomy table above; mdBook page
  `docs/book/src/document-categories.md` (added to `SUMMARY.md` after Architecture Rationale); tree registered in
  `docs/TASK_TREE.md`. No code → all golds + `kg-bench` orthogonal; memory-arch + knowledge-map gates green.
- ID: `DOC-INTENT-TAXONOMY.1` · Status: `done` (`2026-06-22`, read-only measurement) · Goal: **corpus census by
  category** — classify all ingested docs into the 6 categories, measure the distribution, quantify where the
  4-way `document_class` proxy is too coarse. **DONE:** read-only profile of all **78** persisted docs (surface
  counts; no `validate` → zero artifact mutation). **Distribution: 36 wire-protocol / 7 register-IP / 15
  platform-system-IP / 2 CPU-ISA / 4 PHY / 14 methodology-guide.** Three measured structural blind spots: (a)
  cat 2↔3 NOT separable by surface counts (both register/structure-dominant); (b) cat 4 (ISA) has NO distinct
  signature (the 2 ISA docs split across `prose-only` and `reg/struct` buckets — confirms `document_class` has
  no ISA slot); (c) cat 5↔6 indistinguishable (both near-empty). **Deepest finding — the register-heavy-protocol
  trap:** 8 cat-1 protocols (4× CCIX, AXI, CHI, DTI, CHI-C2C) are register/message-dominant → "has registers ⇒
  register-IP" is wrong; the dominant surface is NOT the purpose. Implication: `.3` recognizer needs
  wire-relation shape + front-matter/self-declared type + topology cue beyond counts (ADR-0006, no name lists).
  Report `docs/research/document-intent-category-census.md`; KM `[[document-intent-category-census]]`. No code,
  no canonical mutation → all golds + `kg-bench` orthogonal.
- ID: `DOC-INTENT-TAXONOMY.2` · Status: `done` (`2026-06-22`, read-only measurement) · Goal: **per-category ISF-lowering
  completeness gauge** — for each category, measure honestly what fraction of the document's intent reaches `.isf` (and
  what is honest-absence vs a true gap), producing the prioritized scorecard. **DONE:** per-surface lowering ledger over
  all 78 docs (76 persisted `adapter.json` + 2 read-only `adapt --dry-run`; no `validate`/`adapt` write → zero mutation),
  reproducible via tracked `scripts/measure_isf_completeness.py`. **Two dominant TRUE GAPS measured:** (A) **register
  bit-fields** — 3,449 registers lower 1:1 to opaque width-only `(storage (var (width N)))` but their **12,638 bit-fields
  reach `.isf` ZERO times** across 32 docs (cat 3 largest: 9,308); (B) **message-field structures** — 1,220 fields
  recovered at EvidenceIR across 11 docs but **IntentIR has no carrier** (`has_msgfld_key=false`) → 0 carried / 0 lowered
  (cat-2 NVMe 216/AMD-IOMMU 217 + cat-1 msg-heavy CHI/DTI/CHI-C2C/CCIX). Scorecard MEASURED: cat 1 MATURE, cat 2/3
  PARTIAL, cat 4 THIN, cat 5/6 honest non-targets (5 cat-6 guides over-extract — precision, not completeness). Both gaps
  → the SAME FSMGen ISF-abstraction need (field-structured storage + packet/structure layouts). Report
  `docs/research/document-intent-isf-completeness.md`; KM `[[document-intent-isf-completeness]]`. Objectively measured,
  per-item demonstrated (`[[feedback_scoring_rigor]]`); no fabrication; no code/canonical mutation → golds/`kg-bench`
  orthogonal.
- ID: `DOC-INTENT-TAXONOMY.3` · Status: `done` (`2026-06-22`; `.3a` design + `.3b` implement+validate-reported `d6239217`
  + `.3c` fixtures+book+KM all DONE) · Goal: **fast deterministic category recognizer** so `inspect`/`validate` immediately report a
  PDF's purpose category (the owner's "quickly determine which category"). A richer `document_intent_category` surface
  built on the typed-surface census + structural cues (ADR 0006, no name lists). Acceptance: CLI reports it; fixtures
  lock gold/negative classification; `run_ci.sh` green.
  - `.3a` · Status: `done` (`2026-06-22`, design slice, no code) · **Grounded recognizer design** (below), pinned BEFORE
    coding because `.1` proved counts alone cannot separate cat 2↔3 / recover cat 4 / split cat 5↔6 and must rescue 8
    register-heavy protocols → the recognizer must add cues AND emit honest confidence/residual, never a forced guess.
  - `.3b` · Status: `done` (`2026-06-22`, code slice) · **Implemented** `DocumentIntentCategory` (6 purpose variants +
    `Unresolved`) + `IntentCategoryConfidence` + `DocumentIntentClassification` + the pure
    `classify_document_intent_category(...)` in `crates/specforge/src/ir/completeness.rs` (sibling to `classify_document`,
    same pure-fn + in-file-test pattern), wired into `crates/specforge/src/commands/validate.rs` at the shared census site
    (now bound once and fed to BOTH classifiers): a printed block, an `evidence_document_intent_category` Info finding, and
    `document_intent_category` / `document_intent_category_confidence` metrics. Added front-matter ISA/PHY self-declaration
    helpers (`front_matter_declares_isa` / `front_matter_declares_phy`, generic doc-type vocabulary only — ADR 0006) and a
    shared `front_matter_has_word` helper (the existing `front_matter_doc_type_hint` refactored onto it, behavior identical).
    **Measured refinement of the `.3a` flit clause:** per-document measurement (read off the persisted corpus) PROVED the
    literal `.3a` "flit surface co-present with behavioral/relation shape" cue would MISCLASSIFY register/structure docs as
    wire — NVMe (216 msg + 20 incidental constraints, 0 relations), AMD-IOMMU (98 relations), GIC-600 (101 relations) — so
    the implementation uses the discriminators the data actually supports: (a) flit fields count as a cat-1 cue ONLY when no
    register map is present (CHI/DTI reg=0 vs NVMe/AMD/CCIX reg>0), and (b) a wire-weight vs register/structure-weight
    dominance test that rescues clean wire docs (AXI: wire 401 ≥ struct 229 — register count does NOT veto) while routing
    register-heavy docs to the honest combined category with a residual that names the outweighed wire cue. Only CLEAN wire
    and a self-declared guide are HIGH confidence; everything else is LOW + explicit residual. Build RAM-constrained
    (`CARGO_BUILD_JOBS=2`, RAM monitored). See the Acceptance Checklist below.
  - `.3c` · Status: `done` (`2026-06-22`) · **Fixtures + book + KM.** The per-category gold/negative + honest-residual
    cases are locked by the 13 in-file unit tests added in `.3b` (`crate::ir::completeness`: register-heavy-protocol
    rescue, guide front-matter override, cat 2↔3 / cat 4 / cat 5↔6 residuals, the "only wire+guide are HIGH confidence"
    precision guarantee); `.3c` ADDS an end-to-end integration lock `validate_evidence_ir_reports_document_intent_category`
    in `commands/validate.rs` (the metric + confidence + `evidence_document_intent_category` finding reach the report
    through the real pipeline; the near-empty fall-through → honest `unresolved` residual). **User-facing mdBook chapter:**
    a new "What is the document *about*? — the purpose category" section in `docs/book/src/quality/validation.md` (beside
    `document_class`) + `docs/book/src/document-categories.md` flipped from "target" to "now CLI-reported". **KM fact card**
    `docs/knowledge/document-intent-category-recognizer.md` (map 113→114). **ISA/PHY vocab calibration:** precision-verified
    against real corpus front-matter — ISA vocabulary matches 0 docs (the 2 corpus ISA docs honestly fall through, `.1`
    predicted this), PHY recovers all 4 OpenCAPI PHY docs (added `"physical signaling"` in `.3b`); no further widening
    without risking false positives. `scripts/run_ci.sh` green. See the `.3c` Acceptance Checklist below.

### Recognizer design (`.3a`, grounded in `completeness.rs` + the `.1`/`.2` measured evidence)

**Inputs (extend the existing `DocumentClassCensus`; ALL already in scope at the evidence-validate census site,
`validate.rs` ~2888):** the current fields PLUS `message_field_records` (`ir.message_field_records.len()` — the cat-1
msg-heavy + cat-2 structure cue, the surface `.2` measured at 1,220) and `signal_presence_records.len()`. (Transactions /
temporal live downstream in IntentIR; v1 deliberately does NOT depend on them — relations + constraints + FSM/frame +
message-fields + front-matter are sufficient and keep the recognizer on the same stage as `document_class`.)

**Output:** `DocumentIntentCategory` (6 variants + `Unresolved`) WITH a `confidence` (high/low) and an explicit
`residual` string when structure+front-matter cannot decide — honest residual over fabrication
(`[[feedback_scoring_rigor]]`, project honest-residual doctrine). Never a forced guess.

**Decision order (first STRONG match wins; emit low-confidence + residual otherwise):**
1. **cat 6 methodology/guide** — if front-matter `declared_type == Guide` → cat 6 **regardless of spurious surface
   counts** (directly encodes the `.2` finding: 5/14 guides over-extract — cortex-a76 sw-opt 537 signals — so structure
   must NOT override a self-declared guide). High confidence.
2. **cat 1 wire-protocol** — substantive WIRE-BEHAVIORAL shape: `actor_signal_relations ≥ Rmin` OR
   `signal_constraints ≥ behavioral-floor` OR FSM/serial-frame present OR a substantial `message_field_records` flit
   surface co-present with behavioral/relation shape. **Register count does NOT veto cat 1** — this is the rescue for
   AXI (rel 348), CHI (msg 106 + behavioral), DTI (msg 159 / sigc 16), CHI-C2C, register-heavy CCIX. High confidence
   when the wire shape is clear.
3. **cat 2 / cat 3 register-or-structure-dominant** — registers/fields/message-fields dominate AND no strong wire
   shape. `.1` proved 2↔3 is **semantic, invisible to counts** → emit `RegisterOrPlatform` with a residual unless a
   reliable topology cue separates them (candidate cue: many register blocks across distinct component actors → cat 3;
   single programming model → cat 2 — to be validated empirically in `.3c`, NOT guessed). Low/medium confidence.
4. **cat 4 CPU-ISA** — `.1` proved NO structural signature → only via front-matter ISA vocabulary
   (instruction-set / privileged-architecture), else falls through to (3)/(6) with a residual. Low confidence.
5. **cat 5 PHY vs cat 6 guide** — both near-empty; structure cannot split (`.1`). Emit `Unresolved(PHY|guide)` with a
   residual unless front-matter PHY/electrical/signaling vocabulary is present. Low confidence.

**Genericity guarantee (ADR 0006):** every cue is structural (typed-surface counts/shapes) or generic doc-type
vocabulary already in `front_matter_doc_type_hint`; **no chip/vendor/protocol-instance name list** anywhere. The
recognizer is a pure function of the census, deterministic, additive (does not replace `document_class`, consumes it as
one input). Fixtures (`.3c`) must lock: the register-heavy-protocol rescue, the guide front-matter override, and the
2↔3 / 4 / 5↔6 honest residuals.
- ID: `DOC-INTENT-TAXONOMY.4` · Status: `active` · Goal: **per-category completeness levers** — drive each buildable
  category's measured ISF-lowering gap (`.2` scorecard) to faithful synthesis, filing an FSMGen ISF-abstraction FR
  wherever the current ISF cannot carry the intent (only after empirically verifying the submodule —
  `[[feedback_verify_fsmgen_before_fr]]`, `docs/FSMGEN_FEEDBACK.md`; never an emitter hack — `[[feedback_isf_no_hacks]]`).
  Children: `.4a` Gap A register bit-field lowering (design + FR, done), `.4a.i` adapter honest-residual (code),
  `.4a.ii` Gap A field-structured-storage emit (code, gated on the FSMGen abstraction), `.4b` Gap B message-field
  carrier+lowering, `.4c` cat-3 topology lowering, `.4d` cat-4 ISA/CSR lowering decision, `.4e` conditional-rule
  lowering triage.
- ID: `DOC-INTENT-TAXONOMY.4a` · Status: `done` (`2026-06-22`, measurement/design, docs-only — no Rust code) · Goal:
  **Gap A — register bit-field ISF lowering**: localize WHERE the `.2`-measured 12,638 fields / 32 docs are lost, and
  decide the doctrine-correct fix (emitter path vs FSMGen FR) from empirical evidence. **DONE:** two read-only probes —
  (1) the code-path map proved the bit-field intent is **fully captured and carried** (`RegisterFieldRecord`
  `source.rs:414` → `IntentIr.register_records` clone `intent.rs:193`) and dropped **only** at the ISF-emit boundary
  (`IsfStorageVar { name, width, reset }` at `isf_ir.rs:852`, rendered opaque `(var NAME (width N) [(reset V)])` at
  `isf_ir.rs:391`), so there is **no SpecForge carry gap**; (2) an **empirical** read of the pinned `subs/fsmgen`
  (`030f8c273`) proved the ISF `(storage …)` grammar declares only opaque width-only scalars — **no named-bit-field /
  packed-record construct** (the shipped `set-field`/`extract` are runtime ops, not a static field-map declaration; the
  feature backlog does not list it). **Decision:** Gap A is a genuine missing ISF abstraction → filed a **verified
  FSMGen FR** (`docs/FSMGEN_FEEDBACK.md`, `## Feature request (2026-06-22) — declarative field-structured storage`),
  not an emitter hack (the three emitter-only alternatives were rejected as fabrication/loss). Gap B shares the same
  missing abstraction and additionally lacks an `Evidence→Intent` carrier (zero `message_field` in `intent.rs`). Report
  `docs/research/register-bit-field-isf-lowering-design.md`; KM `[[register-bit-field-isf-lowering-gap]]`; book
  honest-residual note in `docs/book/src/pipeline/isf-adapter.md`. No code/canonical mutation → golds + `kg-bench`
  orthogonal; memory-arch + knowledge-map gates green. See the `.4a` Acceptance Checklist below.
- ID: `DOC-INTENT-TAXONOMY.4a.i` · Status: `pending` · Goal: emit an explicit adapter honest residual
  `isf_register_fields_not_lowered` (today the field drop at `isf_ir.rs:852` is silent; only the *reset* drop is
  recorded as `isf_storage_reset_not_lowered`) so the largest measurable intent-loss is surfaced in `residual_decisions`.
  CODE — requires the full task-acceptance checklist + `run_ci.sh` + FSMGen `--strict --check` 0-new-diagnostics.
- ID: `DOC-INTENT-TAXONOMY.4a.ii` · Status: `done` (`2026-06-22`, CODE) — emitted the IntentIR register field map into
  the shipped ISF field-structured-storage construct; FSMGen pin `d327129b7`. **6,570 register bit-fields now reach
  `.isf` across 2,531 registers in 24 docs** (was 0), 0 new FSMGen `--strict` diagnostics on register docs, 4 wire
  golds byte-identical; see the `.4a.ii` Acceptance Checklist + Verification Log. · Goal: lower the IntentIR register
  field map into the shipped ISF field-structured-storage construct. **This was the highest-leverage buildable lever** (faithfully
  synthesizes the 12,638 register bit-fields / 32 docs that reach `.isf` zero times today) and **supersedes `.4a.i`** —
  with a real lowering target the faithful move is to EMIT the fields, not merely record they were dropped (`.4a.i`
  remains a cheap fallback if `.4a.ii` proves larger than one slice). CODE. **Exact shipped grammar + mapping** (refs on
  `d327129b7`: `subs/fsmgen/docs/ISF_DOWNSTREAM_INTEGRATION_SPEC.md:637`/`:3450`, `docs/book/src/13a-actor-interface.md:468`,
  matrix `13k-…:42`):
  - Emit, behind the existing `(storage (var NAME (width N) [(reset V)]))`, an optional
    `(fields (field FNAME (bits HI LO) [(access …)] [(reset V)] [(enum (M VAL)…)]) …)`. It is **metadata-only /
    schedule-safe** — the scheduled `.fsm` is byte-identical with vs without `(fields …)`, so adding it cannot regress
    the wire golds.
  - Map `RegisterFieldRecord`: `field_name`→`FNAME` (sanitize to a unique HDL identifier), `bits_high`/`bits_low`→
    `(bits HI LO)` (literal inclusive), `access_type`→`(access ro|rw|wo|w1c|w0c|rc|rs|warl|wpri|reserved)` normalized
    (OMIT when it does not map — unsupported tokens fail closed), `reset_value`→`(reset V)` ONLY when the parent reset is
    composed (it matches the parent bit slice by construction; omit when the parent reset is omitted), `enumerated_values`
    →`(enum …)` dropping members that exceed the field width.
  - **Reuse the existing `classify_register_reset` / `register_field_extent` tiling gate (`isf_ir.rs`)** to admit only
    in-width, non-overlapping fields (overlaps / out-of-width fail closed); gaps are allowed.
  - **NO-REGRESSION oracle:** the FSMGen report publishes `inferred_storage[].fields[]: name, msb, lsb, width, access,
    reset, enum` — assert the emitted fields round-trip there; plus `*_passes_fsmgen_strict_validation` 0-new-diagnostics
    on register-bearing docs + the 4 wire golds byte-identical (they carry composable resets but field metadata is
    additive). Full task-acceptance checklist + `run_ci.sh`. Packet/flit layouts (Gap B `.4b`) stay FSMGen-deferred.
- ID: `DOC-INTENT-TAXONOMY.4b` · Status: `pending` · Goal: **Gap B** — add the `Evidence→Intent` `message_field_records`
  carrier (1,220 fields / 11 docs, no IntentIR carrier today), then lower via the same FSMGen structure/packet
  abstraction. Closes the cat-2 structure frontier (the `CORPUS-COVERAGE.2` #21 IOMMU Lever-D) and the cat-1
  message-heavy-protocol gap together. CODE.
- ID: `DOC-INTENT-TAXONOMY.4c` · Status: `done` (`2026-06-23`, measurement + decision packet, read-only, docs-only — no
  Rust code) · Goal: cat-3 topology lowering — promote clock/reset infrastructure + component connectivity from hint-level
  to a synthesizable ISF surface (likely another FSMGen abstraction). **DONE — resolved from measured evidence over 3
  representative cat-3 docs (of 15) + the current FSMGen pin `d327129b7`:** (1) **cat-3's register half already lowers**
  (register maps + bit-fields via `.4a.ii` — CoreSight SoC-600 ~3,250 fields, GIC-600 33 regs — same road as cat-2; not
  the gap); (2) **cat-3 DOES carry a structured topology surface** (`signal_connectivity` producer→consumer graph —
  GIC-600 66 edges / SoC-600 6 — and `infrastructure_signals` with `infrastructure_topology`/`distributed_to_actor_ids`
  clock-reset distribution), refining `.2`'s "hint-level" to **captured-but-sparse-and-unlowered**; (3) **ISF has NO
  declarative static-topology construct** — composition is transaction-level only (`(do child)`, `13f-composition.md`),
  FSMGen's ATL multi-actor frontier is BEHAVIORAL generated-child transaction wiring (not a declarative IP-interconnect
  netlist, verified `14-feature-backlog.md`), and the SpecForge emit is single-initiator-actor
  (`KG-ISF-COMPLETENESS.2a.ii`) so cross-component topology is structurally absent by design; (4) **NO FSMGen FR filed
  yet** — premature because the topology *capture* is sparse/noisy (6 edges across SoC-600's 60 actors; `None`/escaped
  actor names) AND ISF is a per-actor format so static topology may deliberately be the integrator's concern above
  per-module synthesis (resolve WITH FSMGen, not assume — `[[feedback_verify_fsmgen_before_fr]]`). Topology stays an honest
  residual; next lever is a **measurement** (`.4c.i`), not code or a speculative FR. Report
  `docs/research/cat3-topology-isf-lowering-decision.md`; KM `[[cat3-topology-isf-lowering-decision]]`; book
  `document-categories.md` cat-3 maturity refined. No code/canonical mutation → golds + `kg-bench` orthogonal. See the
  `.4c` Acceptance Checklist below.
- ID: `DOC-INTENT-TAXONOMY.4c.i` · Status: `pending` (measurement, read-only) · Goal: **cat-3 topology-capture recall
  measurement** — quantify the density and name-quality of the `signal_connectivity` + `infrastructure_signals` topology
  surfaces across all 15 cat-3 docs (edges-per-actor, fraction with non-`None`/non-escaped endpoints, clock/reset
  distribution coverage). Decides whether the capture is faithful enough to be worth a new ISF construct. Only after this
  does the construct decision become real: a **verified FSMGen FR** for a declarative static-topology construct + a
  multi-actor emit (if the ATL frontier still doesn't subsume it), OR honest-non-target confirmation (topology is the
  integrator's concern above ISF's per-actor scope). Structural density/quality gauge, no name list (ADR 0006).
- ID: `DOC-INTENT-TAXONOMY.4d` · Status: `done` (`2026-06-23`, measurement + decision packet, read-only, docs-only — no
  Rust code) · Goal: cat-4 ISA/CSR lowering decision — does CSR-field / instruction / privilege intent map onto existing
  register/storage abstractions or need a new ISF construct? **DONE — resolved from measured evidence over the corpus's 2
  cat-4 docs + the current FSMGen pin `d327129b7`. Cat-4 intent splits into three:** (1) **CSR / register intent → REUSE
  the existing register/storage abstraction (no new ISF construct, no FSMGen FR)** — CSRs are structurally registers
  (RISC-V Debug captures its 44 Debug-Module CSRs as `register_records`) and FSMGen explicitly titles `(storage (var …
  (fields …)))` the "register map / CSR" construct (`subs/fsmgen/docs/book/src/13a-actor-interface.md:419`/`:468`), the
  same construct `.4a.ii` lowers cat-2/cat-3 fields into; (2) **the cat-4 register gap is EXTRACTION RECALL, not
  abstraction** — RISC-V Debug captures 44 regs / 179 fields with name/access/reset/description but **0/179 are located**
  (no field carries `bits_high`/`bits_low`/`bit_width`; the bit-layout column was unparsed), and RISC-V AIA captures **0
  registers** (its IMSIC/APLIC CSR intent is in prose `conditional_rules`/`behaviors`; 211 empty prose "interfaces"); so
  fields reach `.isf` 0 times not because ISF lacks the construct but because they aren't located → spun out **`.4d.i`**;
  (3) **instruction / privilege-mode / exception / memory-ordering → honest NON-TARGET** (software-visible ISA semantics,
  not synthesizable hardware intent; ISF has no construct + FSMGen lists none → no FR per
  `[[feedback_verify_fsmgen_before_fr]]`/`[[feedback_isf_no_hacks]]`; a conditional-future only if FSMGen's SV/UVM path
  scopes ISA-model verification). Resolves the cat-4 Open Question. Report `docs/research/cat4-isa-csr-lowering-decision.md`;
  KM `[[cat4-isa-csr-lowering-decision]]`; book `document-categories.md` cat-4 maturity refined. No code/canonical mutation
  → golds + `kg-bench` orthogonal. See the `.4d` Acceptance Checklist below.
- ID: `DOC-INTENT-TAXONOMY.4d.i` · Status: `pending` (CODE) · Goal: **cat-4 RISC-V CSR bit-position recovery** — the
  buildable cat-4 lever spun out of `.4d`. Recover the field bit positions for the RISC-V register/CSR layout so RISC-V
  Debug's 179 fields become *located* (parse the bit-layout column/diagram into `bits_high`/`bits_low`), and add a
  RISC-V-shaped register recogniser so RISC-V AIA's IMSIC/APLIC CSR blocks are captured at all. Once a field is located it
  auto-lowers through `.4a.ii` (no emitter change). Must key off structural register-table / bit-layout grammar (a RISC-V
  CSR layout *shape*), never a RISC-V register-name list (ADR 0006). CODE — requires the full task-acceptance checklist +
  `run_ci.sh` + FSMGen `--strict --check` 0-new-diagnostics.
- ID: `DOC-INTENT-TAXONOMY.4e` · Status: `done` (`2026-06-23`, measurement triage, read-only, docs-only — no Rust code) ·
  Goal: conditional-rule lowering triage (`.2` Result 3) — per-item, separate honest residual from a real lever before any
  fraction is called a gap. **DONE — measured 603 `conditional_rules` across 9 representative docs (all 4 buildable
  categories), classified against each doc's declared-signal inventory + consequent quality: A no-consequent prose 392
  (65%) / B undeclared signal 14 (2%) / C declared+placeholder 33 (6%) / D declared+non-placeholder action 164 (27%). The
  D "candidate-lever" bucket is NOT a clean lever — 161/164 are bare deontic modals (`shall`/`must`/`shall not`) with no
  concrete obligation, only 3/603 carry a concrete value/level cue. Lowering a bare modal would fabricate the obligation
  (forbidden — `[[feedback_isf_no_hacks]]`). VERDICT: the conditional-rule shortfall is HONEST RESIDUAL, NOT an
  ISF-completeness gap — no buildable ISF lever, no FSMGen FR; the adapter already lowers the 516 cleanly-grounded
  conditional obligations corpus-wide. The only upside is upstream EXTRACTION quality (the grounded constraint extractor
  recovering the concrete obligation from the modal conditionals' `source_text`), owned by the extraction-quality program,
  distinct from `.4` and lower-leverage than register/structure/topology — recorded as a cross-reference, NOT minted as a
  `.4` gap (`[[feedback_scoring_rigor]]`: a 0.5%-concrete residual is not a gap).** Closes `.2` Result 3. Report
  `docs/research/conditional-rule-lowering-triage.md`; KM `[[conditional-rule-lowering-triage]]`. No code/canonical
  mutation → golds + `kg-bench` orthogonal. See the `.4e` Acceptance Checklist below.

## Acceptance Checklist (enforced) — `DOC-INTENT-TAXONOMY.3b`

- [x] **REPRODUCE / MEASURE** — baseline: `validate <evidence-ir>` had NO purpose-category surface (only the 4-way
  `document_class`). Measured the per-document census off the persisted corpus (read-only): NVMe `register_records=42`/
  `signal_constraints=20`/`actor_signal_relations=0`/`message_field_records=216`; AMD-IOMMU `rel=98`/`msg=217`/`reg=8`;
  AXI `rel=348`/`reg=71`; CHI `msg=106`/`reg=0`/`fsm=7`; GIC-600 `reg=33`/`rfld=293`/`rel=101`. Reproducer: per-doc count
  over `generated/evidence_ir/*/evidence_ir.json` (the `.1`/`.2` census denominator, 78 docs).
- [x] **ROOT CAUSE (WHY + WHERE)** — the literal `.3a` "flit surface co-present with behavioral/relation shape" cat-1 cue
  is falsified by its own corpus data: it would classify register/structure docs as wire, because their incidental
  `signal_constraints` (NVMe 20) and relation surface (AMD-IOMMU 98, GIC-600 101) satisfy the co-presence gate. WHERE:
  the recognizer lives in `crates/specforge/src/ir/completeness.rs` (`classify_document_intent_category`), fed from the
  `document_class` census site `crates/specforge/src/commands/validate.rs:~2888`. Evidence: live `validate` on those docs
  would print `document_intent_category: wire-protocol` under the naive cue, contradicting the `.1` census (NVMe/AMD are
  register/structure, not wire).
- [x] **ADDRESSED (verified)** — implemented the recognizer with the discriminators the measurement supports: flit fields
  are a cat-1 cue ONLY when `registers == 0`, plus a wire-weight vs register/structure-weight dominance test. Live
  `validate` over all 78 persisted docs (before → after): NVMe `(none)` → `register-or-platform (low)` (structure 459 vs
  wire 20); AMD-IOMMU → `register-or-platform (low)` with the "register-heavy WIRE protocol" residual (struct 225 vs wire
  98); GIC-600 → `register-or-platform (low)` (struct 326 vs wire 110); AXI → `wire-protocol (high)` (wire 401 ≥ struct
  229 — register count does not veto); CHI → `wire-protocol (high)` (flit, no register map); the 2 OpenCAPI PHY signaling
  specs → `physical-link (low)`; the GIC overview guide → `methodology-guide (high)`. Corpus distribution: 21 wire-protocol
  (high) / 8 methodology-guide (high) / 28 register-or-platform (low) / 16 unresolved (low) / 5 physical-link (low) — and
  **0 high-confidence false positives** (every wire/guide high-confidence call verified genuinely correct).
- [x] **NO REGRESSION** — `kg-bench 156/156`; completeness lib tests `66/66` (incl. 13 new recognizer tests); full
  `cargo test` `1695 passed; 0 failed` (warning-deny); `cargo fmt --all --check` clean; `cargo clippy --all-targets -D
  warnings` clean. WIRE-BASED-100 is provably **orthogonal**: this slice adds a pure new function + additive `validate`
  reporting only — it touches NO extraction/semantic/intent/emitter path, so `signal_constraints`/`actor_signal_relations`/
  `temporal_rules` and the emitted `.isf` are byte-identical by construction (the wire golds are unaffected). `run_ci.sh`
  green (see Verification Log).
- [x] **GENERICITY (ADR 0006)** — every cue is a structural typed-surface count/shape or generic front-matter doc-type
  vocabulary (guide / specification / instruction-set / privileged-architecture / physical-layer); NO chip/vendor/
  protocol-instance name list. Precision-verified on the corpus: the ISA vocabulary matched 0 docs (the corpus ISA docs
  honestly fall through to a residual), and the PHY phrase `"physical signaling"` matched ONLY the 2 PHY signaling specs.
- [x] **LOCKSTEP** — `.3b` updates the tracked continuity docs (README validate-surface bullet, `CHANGES.md`,
  `DEVELOPMENT_NOTES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `RUST_CODEBASE_ANALYSIS.md`, `MEMORY.md`). The user-facing mdBook
  chapter (`quality/validation.md` beside `document_class`; `document-categories.md` "now CLI-reported"), the KM fact card,
  and the gold/negative + honest-residual fixtures are the explicit `.3c` deliverable (pinned decomposition), landing in
  the immediately-following slice.

## Acceptance Checklist (enforced) — `DOC-INTENT-TAXONOMY.3c`

- [x] **REPRODUCE / MEASURE** — baseline: after `.3b` the recognizer logic was locked by 13 in-file unit tests, but the
  validate-surface WIRING (census → `classify_document_intent_category` → metric + finding) had NO end-to-end regression
  lock, and the user-facing mdBook + KM card were the pinned `.3b`→`.3c` lockstep gap. Measured live: `validate` emits
  `document_intent_category` / `document_intent_category_confidence` + `evidence_document_intent_category` across all 78
  docs.
- [x] **ROOT CAUSE (WHY + WHERE)** — the surface emission lived only in `crates/specforge/src/commands/validate.rs`
  (`document_intent_category` metric + `evidence_document_intent_category` finding) with no integration test asserting it
  reaches the persisted report; a future refactor of the census site could silently drop it. WHERE: the new test
  `validate_evidence_ir_reports_document_intent_category` in `commands/validate.rs` builds an EvidenceIR through the real
  pipeline and asserts the metric/confidence/finding (the near-empty doc → honest `unresolved` residual path).
- [x] **ADDRESSED (verified)** — added the end-to-end integration test (passes: metric `unresolved`, confidence `low`,
  finding present with its residual); the per-category gold/negative cases remain locked by the 13 `.3b` unit tests.
  Documented the surface for users: a new purpose-category section in `docs/book/src/quality/validation.md` beside
  `document_class`, and `docs/book/src/document-categories.md` flipped from "target" to "now CLI-reported"; wrote the KM
  fact card `docs/knowledge/document-intent-category-recognizer.md` (map regenerated 113 → 114 facts, in sync).
- [x] **NO REGRESSION** — `kg-bench 156/156`; full `cargo test` green (warning-deny, now incl. the new integration test);
  `cargo fmt`/`clippy -D warnings` clean; `mdbook build` green; knowledge-map derive-and-diff in sync; `run_ci.sh` green.
  WIRE-BASED-100 **orthogonal** (a test + docs only — no extraction/semantic/intent/emitter path touched).
- [x] **GENERICITY (ADR 0006)** — no production-logic change; the documented recognizer is name-list-free (structural
  census + generic front-matter doc-type vocabulary). ISA/PHY vocab precision-verified on the corpus (ISA matches 0 docs;
  `"physical signaling"` matches only the 2 PHY signaling specs).
- [x] **LOCKSTEP** — mdBook (`quality/validation.md` + `document-categories.md`), KM card + regenerated `KNOWLEDGE_MAP.md`,
  and the task tree / `TASK_TREE.md` / `CHANGES.md` / `DEVELOPMENT_NOTES.md` / `LIVE_ACHIEVEMENT_STATUS.md` / `MEMORY.md`
  all updated in this slice.

## Acceptance Checklist (enforced) — `DOC-INTENT-TAXONOMY.4a`

- [x] **REPRODUCE / MEASURE** — baseline from `.2` (`docs/research/document-intent-isf-completeness.md`,
  `scripts/measure_isf_completeness.py`): register **bit-fields** reach `.isf` **0 times** — `12,638` fields across
  `32` docs — while registers lower 1:1 to opaque `(storage (var … (width N)))`. Live datum: RISC-V IOMMU `.isf`
  storage is `(var register_table_0033 (width 26))` with no field substructure though IntentIR carried 147 fields / 33
  registers.
- [x] **ROOT CAUSE (WHY + WHERE)** — two read-only probes. (1) Code-path map: the bit-field metadata is FULL at
  extraction (`RegisterFieldRecord` — name/bits_high/bits_low/width/access/reset/description/enums,
  `crates/specforge/src/ir/source.rs:414`) and carried UNCHANGED into IntentIR
  (`IntentIr.register_records = semantic_ir.register_records.clone()`, `crates/specforge/src/ir/intent.rs:193`;
  field `:75`); it is discarded ONLY at the ISF-emit boundary — `IsfStorageVar { name, width, reset }` built at
  `crates/specforge/src/ir/isf_ir.rs:852` (reading `r.fields` only to compose a register-wide reset), rendered opaque at
  `:391`. So there is NO SpecForge carry gap. (2) Empirical FSMGen probe on the pinned `subs/fsmgen` (`030f8c273`): the
  ISF `(storage …)` grammar declares opaque width-only scalars only (`ISF_DOWNSTREAM_INTEGRATION_SPEC.md` §8) — NO
  named-bit-field / packed-record declaration; the shipped `set-field`/`extract` are runtime ops (`13k` matrix), and the
  feature backlog (`14-…`) does not list field-structured storage. Gap A is a missing ISF abstraction, not an emitter
  bug.
- [x] **ADDRESSED (verified)** — filed a **verified FSMGen feature request** for declarative field-structured storage
  (`docs/FSMGEN_FEEDBACK.md`, `## Feature request (2026-06-22) — declarative field-structured storage`), grounded in the
  empirical submodule verification (`[[feedback_verify_fsmgen_before_fr]]`) and NOT an emitter hack
  (`[[feedback_isf_no_hacks]]`): the three emitter-only alternatives (per-field vars / runtime `extract` / comments) are
  documented and rejected as fabrication or intent-loss. Design report `docs/research/register-bit-field-isf-lowering-design.md`;
  follow-on code leaves `.4a.i` (adapter honest residual) and `.4a.ii` (field-structured emit, gated on FSMGen) recorded.
- [x] **NO REGRESSION** — measurement/design leaf, **no Rust code**, no canonical-artifact mutation → the wire golds /
  `kg-bench` / emitted `.isf` are byte-identical by construction (WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh`
  green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync
  (114 → 115 facts / 824 keys).
- [x] **GENERICITY (ADR 0006)** — docs/FR only; the FR's proposed construct is structural (bit ranges + access + reset +
  enum), with no chip/vendor/protocol-instance name list. N/A for runtime code (none).
- [x] **LOCKSTEP** — `docs/FSMGEN_FEEDBACK.md` (the FR), `docs/research/register-bit-field-isf-lowering-design.md`, KM
  card `docs/knowledge/register-bit-field-isf-lowering-gap.md` + regenerated `KNOWLEDGE_MAP.md`, book honest-residual
  note in `docs/book/src/pipeline/isf-adapter.md`, and the task tree / `TASK_TREE.md` / `CHANGES.md` /
  `DEVELOPMENT_NOTES.md` / `LIVE_ACHIEVEMENT_STATUS.md` / `MEMORY.md` all updated in this slice.

## Acceptance Checklist (enforced) — `DOC-INTENT-TAXONOMY.4a.ii`

- [x] **REPRODUCE / MEASURE** — baseline from `.2`/`.4a`: register **bit-fields** reach `.isf` **0 times**
  (`12,638` fields captured in `IntentIr.register_records` across `35` register-bearing docs) — the emitter
  built `IsfStorageVar { name, width, reset }` at `isf_ir.rs:852` and rendered opaque `(var NAME (width N)
  [(reset V)])` at `:391`, discarding every field. Grounding measurement over the persisted `generated/intent_ir/*`
  corpus (read-only, reproducible): `8,708 / 12,638` fields are **located** (carry `bits_high`/`bits_low` or
  `bits_low`+`bit_width`); `130` registers carry a sanitized-name collision (dominated by reserved gaps `res0`×115 /
  `reserved`×53, plus mis-extraction dups e.g. `size`×9); `36` carry a located-field bit overlap; access is
  dominated by mappable tokens (`RO` 4376 / `RW` 2189 / `WO` 236 / `WARL` 139 / `R` 110 / `WPRI` 35 / `RW1C` 18 ≈
  88 % of the 8,177 access cells); enums are rare (6 members / 4 fields).
- [x] **ROOT CAUSE (WHY + WHERE)** — the loss is a single emit-time discard, NOT a carry gap: `RegisterFieldRecord`
  (`source.rs:414`: name/bits_high/bits_low/bit_width/access_type/reset_value/enumerated_values) is carried UNCHANGED
  into `IntentIr.register_records` (`intent.rs:193`) and then dropped at `isf_ir.rs:852` (the `IsfStorageVar` build
  read `r.fields` only to compose a register-wide `(reset V)` via `classify_register_reset`). FSMGen had no field
  construct on the old pin (`.4a` verified `030f8c273`); pin `d327129b7` (`FSMGEN-REFRESH-INTEGRATE-5`) SHIPPED
  declarative `(fields (field NAME (bits HI LO) [(access …)] [(reset V)] [(enum …)]))` — empirically re-verified on the
  live binary: a hand-authored fields block returns `fsmgen --strict --check --json` `diagnostic_summary.success=true`,
  round-trips in `fsmgen --emit-schedule-json` `inferred_storage[].fields[]` (`name/msb/lsb/width/access/reset/enum`),
  and an out-of-width field fails closed (`field 'big' bits [9:0] exceed parent width 8`).
- [x] **ADDRESSED (verified)** — added `register_storage_fields` (`isf_ir.rs`) + `normalize_field_access` and rendered
  the `(fields …)` block behind each `(var …)`. Admission gate (ADR-0006 structural, no name list): located fields
  only (unlocated → honest gap); DROP every field whose sanitized name collides (uniformly handles reserved gaps +
  mis-extraction dups → FSMGen's duplicate-name fail-closed); the survivors must be non-overlapping (any residual
  overlap fails closed the whole register); `(access)` normalized to the 10-token FSMGen set, omitted when unmapped;
  field `(reset)` emitted only when the parent reset is composed (`Emit`), as the parent-V bit slice (matches by
  construction — FSMGen's "field reset must match parent slice"); `(enum)` members keep numeric values fitting the
  field width, `meaning`→member (deduped). Measured live with the REAL emitter across the corpus (`adapt --target
  isf` over `generated/intent_ir/*`): **6,570 register bit-fields now reach `.isf` across 2,531 registers in 24 docs**
  (top emitters CoreSight SoC-600 1,166 / 1,118 / 974; GIC arch `ihi0069` 424; CCIX 380; SMMU `ihi0070` 332; CHI-C2C
  276; the remaining captured fields are honest residuals — unlocated bits / ambiguous (shared) names /
  overlap-failclosed registers / duplicate-named registers). Per-document strict proof on real data: RISC-V IOMMU
  (122 fields), GIC arch (424), CoreSight SoC-600 (974) each go from persisted `fsmgen --strict` `success / 0 diags`
  → new `success / 0 diags` (0 NEW diagnostics). New unit tests on the pure derivation (`register_storage_fields`) +
  the access normalizer + render; a new `register_fields_pass_fsmgen_strict_and_round_trip` end-to-end test renders a
  fields-bearing `IsfIr`, asserts `fsmgen --strict` success AND the `inferred_storage[].fields[]` round-trip.
- [x] **NO REGRESSION** — the 4 WIRE-BASED-100 golds emit **0** fields (AXI `ihi0022_l` / APB `ihi0024_e` / AHB
  `ihi0033_c` / AXI-Stream `ihi0051_b` carry no located composable register fields) → their emitted `.isf` is
  **byte-identical** (verified by the corpus emit scan + `adapt --dry-run`); the field block is metadata-only /
  schedule-safe (FSMGen `.fsm` byte-identical with vs without `(fields …)`), so WIRE-BASED-100 is orthogonal by
  construction. `kg-bench 156/156`; `*_passes_fsmgen_strict_validation` ×7 PASS (the new round-trip test + the 6
  existing canaries, 0 new diagnostics); full `cargo test` green (warning-deny); `cargo fmt`/`clippy -D warnings`
  clean; `run_ci.sh` green.
- [x] **GENERICITY (ADR 0006)** — every gate is structural/universal register grammar: bit-extent arithmetic, a
  collision-keyed (count≥2) ambiguity drop, an overlap mask, and a universal RTL access-token normalizer
  (`ro|rw|wo|w1c|w0c|rc|rs|warl|wpri|reserved` + unambiguous synonyms `r→ro`/`w→wo`/`r/w→rw`/`rw1c→w1c`); NO
  chip/vendor/protocol-name list; unmapped access / unlocated / ambiguous / overlapping fields are honest residuals,
  never fabricated.
- [x] **LOCKSTEP** — mdBook (`pipeline/isf-adapter.md` "Register bit-fields" section flipped from residual to lowered +
  closed-task subsection; `document-categories.md` cat-2 maturity updated), KM card
  `register-bit-field-isf-lowering-gap.md` + regenerated `KNOWLEDGE_MAP.md`, and the task tree / `TASK_TREE.md` /
  `README.md` / `CHANGES.md` / `DEVELOPMENT_NOTES.md` / `RUST_CODEBASE_ANALYSIS.md` / `LIVE_ACHIEVEMENT_STATUS.md` /
  `MEMORY.md` all updated in this slice.

## Acceptance Checklist (enforced) — `DOC-INTENT-TAXONOMY.4d`

- [x] **REPRODUCE / MEASURE** — baseline from `.2`: cat-4 scored **THIN** (only the register-shaped surface lowers).
  Profiled the corpus's 2 cat-4 docs read-only off the persisted IR (reproducible from
  `generated/intent_ir/<key>/intent_ir.json`): RISC-V Debug = **44 `register_records` / 179 fields / 0 located** (every
  field carries `field_name`+`access_type`+`reset_value`+`description` but none carries `bits_high`/`bits_low`/
  `bit_width`); RISC-V AIA = **0 `register_records`** (39 prose `conditional_rules` naming IMSIC/APLIC, 493 behaviors, 211
  `interfaces` whose signal records are all `None` with 0 constraints / 0 relations). Re-verified FSMGen pin `d327129b7`:
  `(storage (var … (fields …)))` is titled the register-map/CSR construct (`13a-actor-interface.md:419`/`:468`); no
  instruction/privilege/exception construct exists (`13a`–`13i`, matrix, integration spec).
- [x] **ROOT CAUSE (WHY + WHERE)** — cat-4's THIN score is two distinct causes, neither an ISF-abstraction gap for CSRs.
  (a) RISC-V Debug fields are *unlocated*: the `field_table` register strategy (`crates/specforge/src/ir/evidence.rs`)
  captured field identity but not the bit-layout column, so the `.4a.ii` located-fields gate (`isf_ir.rs
  register_storage_fields`) admits 0 of them. (b) RISC-V AIA registers are *uncaptured*: no current register strategy
  matches RISC-V's CSR layout — the `.10g` `<NAME>, bits [hi:lo]` section-heading family fires only on ARM `ihiXXXX`
  architecture specs. Both are extraction-recall causes; the ISF register/storage construct itself is sufficient.
- [x] **ADDRESSED (verified)** — produced the decision packet (`docs/research/cat4-isa-csr-lowering-decision.md`): CSR
  intent **reuses the existing register/storage abstraction** (no new ISF construct, no FSMGen FR); the register gap is an
  **extraction-recall leaf `.4d.i`** (locate RISC-V CSR fields + capture AIA CSR blocks → auto-lower via `.4a.ii`);
  instruction/privilege/exception/memory-ordering are an **honest non-target** (conditional-future FR only if FSMGen's
  SV/UVM path scopes ISA-model verification). The construct-vs-reuse Open Question is resolved with measured evidence,
  not a guess.
- [x] **NO REGRESSION** — measurement/decision leaf, **no Rust code**, no canonical-artifact mutation → the wire golds /
  `kg-bench` / emitted `.isf` are byte-identical by construction (WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh`
  green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync
  (115 → 116 facts).
- [x] **GENERICITY (ADR 0006)** — the decision rests on structural evidence (register vs non-register intent shape;
  located vs unlocated fields) and FSMGen's published grammar — no chip/vendor/protocol-instance name list. The spun-out
  `.4d.i` lever is mandated to key off structural RISC-V CSR-layout *shape*, never a RISC-V register-name list. N/A for
  runtime code (none in this leaf).
- [x] **LOCKSTEP** — `docs/research/cat4-isa-csr-lowering-decision.md` (the packet), KM card
  `docs/knowledge/cat4-isa-csr-lowering-decision.md` + regenerated `KNOWLEDGE_MAP.md`, book `docs/book/src/document-categories.md`
  (cat-4 maturity refined), and the task tree / `TASK_TREE.md` / `CHANGES.md` / `DEVELOPMENT_NOTES.md` /
  `LIVE_ACHIEVEMENT_STATUS.md` / `MEMORY.md` all updated in this slice.

## Acceptance Checklist (enforced) — `DOC-INTENT-TAXONOMY.4c`

- [x] **REPRODUCE / MEASURE** — baseline from `.2`: cat-3 scored **PARTIAL** ("infrastructure signals + actor ports lower;
  topology stays at the hint level"). Profiled 3 representative cat-3 docs (of 15) read-only off the persisted IR
  (reproducible from `generated/intent_ir/<key>/intent_ir.json`): CoreSight SoC-600 = 60 actors / 57 actor_ports / **6**
  `signal_connectivity` / 0 `infrastructure_signals` / 833 registers; GIC-600 = 55 / 147 / **66** / **2** / 33; CoreSight
  Base System Arch = 5 / 0 / 0 / 0 / 0 (88 prose `interfaces`). `signal_connectivity` is a typed producer→consumer graph;
  `infrastructure_signals` carries `infrastructure_topology` / `distributed_to_actor_ids` (clock/reset distribution).
- [x] **ROOT CAUSE (WHY + WHERE)** — topology does not reach `.isf` for two structural reasons, neither an extraction-only
  bug. (a) ISF has **no declarative static-topology construct**: composition is transaction-level only
  (`subs/fsmgen/docs/book/src/13f-composition.md`, `(do child)`), and FSMGen's multi-actor "ATL" backlog
  (`14-feature-backlog.md`) wires children *generated from transaction composition* — behavioral orchestration, not a
  declarative IP-interconnect netlist. (b) The SpecForge emit is **single-initiator-actor**
  (`crates/specforge/src/ir/isf_ir.rs` `select_initiator_actor`, `KG-ISF-COMPLETENESS.2a.ii`), so a 60-component doc emits
  one actor — cross-component topology is structurally absent by design. The capture itself is also **sparse/noisy** (6
  edges across SoC-600's 60 actors; `None`/escaped actor names).
- [x] **ADDRESSED (verified)** — produced the decision packet
  (`docs/research/cat3-topology-isf-lowering-decision.md`): cat-3's register half already lowers (`.4a.ii`); topology is
  **captured-but-sparse-and-unlowered** (refining `.2`'s "hint-level"); ISF has no static-topology construct and FSMGen's
  ATL frontier is not a home; **no FR is filed yet** (premature — capture too sparse/noisy + ISF is a per-actor format
  where static topology may be the integrator's concern above per-module synthesis); the buildable next step is the
  measurement `.4c.i` (topology-capture recall across all 15 cat-3 docs), after which the construct decision (verified FR
  vs honest-non-target) becomes real. No fabrication, no speculative FR.
- [x] **NO REGRESSION** — measurement/decision leaf, **no Rust code**, no canonical-artifact mutation → the wire golds /
  `kg-bench` / emitted `.isf` are byte-identical by construction (WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh`
  green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync
  (116 → 117 facts).
- [x] **GENERICITY (ADR 0006)** — the decision rests on structural evidence (typed topology surfaces; their density;
  FSMGen's published composition grammar) — no chip/vendor/protocol-instance name list. The spun-out `.4c.i` recall
  measurement is mandated to be a structural density/quality gauge, not a per-document name list. N/A for runtime code
  (none in this leaf).
- [x] **LOCKSTEP** — `docs/research/cat3-topology-isf-lowering-decision.md` (the packet), KM card
  `docs/knowledge/cat3-topology-isf-lowering-decision.md` + regenerated `KNOWLEDGE_MAP.md`, book
  `docs/book/src/document-categories.md` (cat-3 maturity refined), and the task tree / `TASK_TREE.md` / `CHANGES.md` /
  `DEVELOPMENT_NOTES.md` / `LIVE_ACHIEVEMENT_STATUS.md` / `MEMORY.md` all updated in this slice.

## Acceptance Checklist (enforced) — `DOC-INTENT-TAXONOMY.4e`

- [x] **REPRODUCE / MEASURE** — baseline from `.2` Result 3: the `constraints + temporal + conditional → (rule)` lowering
  ratio is low (cat 1 ~41% / cat 2 ~45% / cat 3 ~11% / cat 4 ~22%), dominated by `conditional_rules`, with the
  honest-vs-lever verdict explicitly deferred to a `.4+` triage. Measured per item over 9 representative docs (all 4
  buildable categories, reproducible from `generated/intent_ir/<key>/intent_ir.json`): **603 `conditional_rules`** → A
  no-consequent prose 392 / B undeclared signal 14 / C declared+placeholder 33 / D declared+non-placeholder action 164;
  the D action distribution is `shall` 45 / `must` 35 / `shall not` 14 / `shall be cleared` 10 / `must not` 7 / … =
  **161/164 bare deontic modals, only 3/603 carry a concrete value/level cue.**
- [x] **ROOT CAUSE (WHY + WHERE)** — a conditional lowers to an ISF `(rule)` only when it names a declared signal AND a
  concrete obligation. A/B/C (439, 73%) fail that by construction (prose condition / undeclared signal / no obligation);
  the D bucket fails because the captured `consequent_action` is a deontic modal fragment, not the concrete value the
  document states only in prose — rendering it would fabricate the obligation. WHERE: the gap is upstream (the constraint
  extractor capturing `consequent_action` as a modal, `extract-constraints-llm` / `EXTRACTION-QUALITY-GAUGE`), NOT the ISF
  adapter (which already lowers the 516 cleanly-grounded conditional obligations corpus-wide).
- [x] **ADDRESSED (verified)** — produced the triage (`docs/research/conditional-rule-lowering-triage.md`): the
  conditional-rule shortfall is **honest residual, not an ISF-completeness gap** — no buildable ISF lever, no FSMGen FR.
  The only improvement path is upstream extraction quality (recover the concrete obligation from the modal conditionals'
  `source_text`), owned by the extraction-quality program and lower-leverage than register/structure/topology — recorded
  as a cross-reference, deliberately NOT minted as a `.4` gap (a 0.5%-concrete residual is not a gap). Closes `.2` Result
  3 and completes the `.2` measurement phase.
- [x] **NO REGRESSION** — measurement/triage leaf, **no Rust code**, no canonical-artifact mutation → the wire golds /
  `kg-bench` / emitted `.isf` are byte-identical by construction (WIRE-BASED-100 orthogonal). `scripts/check_doctrines.sh`
  green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync
  (117 → 118 facts).
- [x] **GENERICITY (ADR 0006)** — the triage keys off structural signal-declaration membership and consequent-action
  shape (concrete value/level/edge cue vs bare modal) — no chip/vendor/protocol-instance name list. N/A for runtime code
  (none in this leaf).
- [x] **LOCKSTEP** — `docs/research/conditional-rule-lowering-triage.md` (the triage), KM card
  `docs/knowledge/conditional-rule-lowering-triage.md` + regenerated `KNOWLEDGE_MAP.md`, and the task tree /
  `TASK_TREE.md` / `CHANGES.md` / `DEVELOPMENT_NOTES.md` / `LIVE_ACHIEVEMENT_STATUS.md` / `MEMORY.md` all updated in this
  slice. (No mdBook change: this leaf confirms existing honest-residual framing — no user-facing capability drift; the
  book's category scorecard does not claim conditional rules lower.)

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `DOC-INTENT-TAXONOMY.1` | `done` (`2026-06-22`) | Census DONE — distribution 36/7/15/2/4/14; denominator established. |
| — | `DOC-INTENT-TAXONOMY.2` | `done` (`2026-06-22`) | ISF-completeness scorecard MEASURED — maturity column now objective; two dominant true gaps (register bit-fields 12,638→0; message-field structures 1,220→0, no Intent carrier). |
| — | `DOC-INTENT-TAXONOMY.3a` | `done` (`2026-06-22`) | Recognizer DESIGN pinned (grounded in `completeness.rs` + measured `.1`/`.2` evidence): cues, decision order, honest-residual policy, ADR-0006 genericity — de-risks the hard name-list-free classifier before coding. |
| — | `DOC-INTENT-TAXONOMY.3b` | `done` (`2026-06-22`) | Recognizer IMPLEMENTED + reported by `validate` (`classify_document_intent_category` in `completeness.rs`, wired into `validate.rs`); corpus: 21 wire / 8 guide (both high) / 28 register-or-platform / 16 unresolved / 5 PHY (low) with 0 high-confidence false positives. Refined the `.3a` flit clause its own data falsified. kg-bench 156/156, cargo test 1695/0, clippy/fmt clean. |
| — | `DOC-INTENT-TAXONOMY.3c` | `done` (`2026-06-22`) | Recognizer FIXTURES + book + KM landed: 13 in-file unit golds (`.3b`) + an end-to-end `validate` integration test; user-facing mdBook chapter (`quality/validation.md` + `document-categories.md` "now CLI-reported"); KM card (map 113→114); ISA/PHY vocab precision-verified. `run_ci.sh` green. **`.3` recognizer COMPLETE.** |
| — | `DOC-INTENT-TAXONOMY.4a` | `done` (`2026-06-22`) | Gap A localized + decided: bit-field intent reaches IntentIR fully, dropped only at `isf_ir.rs:852`; the current ISF `(storage …)` has no field-structured construct (verified pin `030f8c273`) → a **verified FSMGen FR** (not an emitter hack). Design report + KM card + book note. Docs-only → golds/`kg-bench` orthogonal. |
| — | `DOC-INTENT-TAXONOMY.4a.ii` | `done` (`2026-06-22`) | **Register bit-field emit DONE.** Emitted `(var … (fields (field …)))` from the IntentIR register field map — **6,570 fields / 2,531 registers / 24 docs now reach `.isf`** (was 0). Metadata-only/schedule-safe; reused the `register_field_extent` tiling gate; verified via `inferred_storage[].fields[]` round-trip + `*_passes_fsmgen_strict_validation` ×7 + register docs 0-new-diagnostics + 4 wire golds byte-identical; `kg-bench 156/156`; `run_ci.sh` green. The `.4a.i` honest residual is folded in (`isf_register_fields_not_lowered` for the unlowered remainder). |
| — | `DOC-INTENT-TAXONOMY.4d` | `done` (`2026-06-23`) | **Cat-4 ISA/CSR decision packet DONE.** Resolved the Open Question from measured evidence: cat-4 CSRs **reuse the existing register/storage abstraction** (no new ISF construct — FSMGen titles `(storage …)` the register-map/CSR construct); the cat-4 register gap is **extraction recall** (RISC-V Debug 179 fields all unlocated, AIA 0 registers captured) → spun out `.4d.i`; instruction/privilege/exception/memory-ordering are **honest non-targets**. Docs-only → golds/`kg-bench` orthogonal. |
| — | `DOC-INTENT-TAXONOMY.4c` | `done` (`2026-06-23`) | **Cat-3 topology decision packet DONE.** Cat-3's register half already lowers (`.4a.ii`); cat-3 **does** carry a typed topology surface (`signal_connectivity` + `infrastructure_signals`) — but it is **sparse/noisy** and ISF has **no declarative static-topology construct** (composition is transaction-level only; emit is single-initiator-actor; FSMGen's ATL frontier is behavioral, not a netlist). **No FR yet** (premature) → spun out `.4c.i` capture-recall measurement. Docs-only → golds/`kg-bench` orthogonal. |
| — | `DOC-INTENT-TAXONOMY.4e` | `done` (`2026-06-23`) | **Conditional-rule lowering triage DONE (`.2` Result 3 closed).** 603 `conditional_rules` / 9 docs → 73% honest residual (prose/undeclared/placeholder) + a 27% bucket that is 161/164 bare deontic modals (only 3/603 concrete). Verdict: **honest residual, NOT an ISF gap** — no lever, no FR; the adapter already lowers the 516 clean conditional obligations; the only upside is upstream extraction quality. `.2` measurement phase complete. Docs-only → golds/`kg-bench` orthogonal. |
| 1 | `DOC-INTENT-TAXONOMY.4c.i` | `pending` (measurement) | Cat-3 topology-capture recall measurement across all 15 cat-3 docs (edges-per-actor, endpoint name quality, clock/reset coverage) → decides FSMGen-FR vs honest-non-target. Read-only, RAM-light. |
| 2 | `DOC-INTENT-TAXONOMY.4d.i` | `pending` (code) | Cat-4 RISC-V CSR bit-position recovery — locate RISC-V Debug's fields + capture AIA's IMSIC/APLIC CSR blocks so they auto-lower via `.4a.ii`. Structural RISC-V CSR-layout grammar, no name list. The genuine buildable cat-4 lever. |
| 3 | `DOC-INTENT-TAXONOMY.4b` | `pending` (gated) | Gap B — `Evidence→Intent` `message_field_records` carrier (1,220 fields / 11 docs), then lower; **stays gated** — FSMGen explicitly deferred packet/flit layouts. CODE. |
| 4 | `DOC-INTENT-TAXONOMY.4a.i` | `superseded` | Adapter honest residual `isf_register_fields_not_lowered` — **superseded by `.4a.ii`**, which both EMITS the fields AND records the unlowered remainder as that very residual. No separate slice needed. |

## Decisions

- `2026-06-22`: Adopt the **6-category purpose taxonomy** as the guiding lens (above). It is semantic ("what is the
  document about"), distinct from and built on the structural `document_class`. Categories 5 (PHY/electrical) and 6
  (methodology/guide) are **honest non-targets** for ISF synthesis; the buildable program is categories 1–4.
- `2026-06-22`: ISF is the synthesis target for ALL categories. Where the current FSMGen ISF cannot capture a category
  naturally/elegantly, the gap is fed back to FSMGen as a verified feature request rather than hacked into the emitter
  (`[[feedback_isf_no_hacks]]`). Owner context: FSMGen is adding a verification-oriented SV/UVM + VHDL lowering path
  alongside its default synthesizable HDL, and new ISF abstractions (memory banks, single/dual-port memory modules, …)
  are anticipated for both paths.
- `2026-06-22` (`.2`): the per-category maturity column is now **objectively measured** (per-surface lowering ledger,
  read-only). The two dominant true gaps — register **bit-fields** (12,638 captured, 0 lowered; registers emit opaque
  width-only storage vars) and message-field **structures** (1,220 captured at EvidenceIR, 0 carried to IntentIR) —
  converge on the SAME missing FSMGen ISF abstraction (field-structured storage + packet/structure layouts), confirming
  the `.0` "new ISF abstractions anticipated" decision with hard numbers. Signals are deliberately NOT scored as a
  present/lowered ratio (the `.isf` signal set has a different basis than IntentIR `interfaces`). Cat 5/6 confirmed honest
  non-targets; the cat-6 over-extraction (5/14 guides emit spurious `.isf`) is a `.3`-recognizer precision motive, not an
  ISF-completeness gap.
- `2026-06-23` (`.4d`): category-4 (CPU ISA) intent **splits into three**, and only one is a buildable SpecForge lever.
  (1) **CSR / register intent reuses the existing ISF register/storage abstraction** — there is **no new ISF construct**
  for CSRs and **no FSMGen FR**: CSRs are structurally registers (RISC-V Debug captures its 44 CSRs as `register_records`)
  and FSMGen explicitly names `(storage (var … (fields …)))` the "register map / CSR" construct. (2) **The cat-4 register
  gap is EXTRACTION RECALL, not a missing abstraction** — fields don't lower because they are *unlocated* (RISC-V Debug
  0/179 fields carry bit positions) or the registers are *uncaptured* (RISC-V AIA 0 registers; CSR intent in prose), not
  because ISF can't express them; the fix is an **extraction leaf** (`.4d.i`), not an FR. (3) **Instruction / privilege /
  exception / memory-ordering intent is an honest NON-TARGET** for ISF synthesis (software-visible ISA semantics, not
  synthesizable hardware intent; ISF has no construct + FSMGen lists none — a verified FR is a conditional-future only if
  FSMGen's SV/UVM verification path explicitly scopes ISA-model verification). No fabrication, no speculative FR, no
  emitter hack (`[[feedback_isf_no_hacks]]` / `[[feedback_verify_fsmgen_before_fr]]`).
- `2026-06-23` (`.4c`): category-3 (platform / system-IP topology) has **two halves**. (1) **The register half already
  lowers** (register maps + bit-fields via `.4a.ii`; infrastructure signals; actor ports — same constructs as cat-2). (2)
  **The distinctive topology half is captured but unlowerable today.** Cat-3 docs carry a typed topology surface
  (`signal_connectivity` producer→consumer graph; `infrastructure_signals` clock/reset distribution) — so `.2`'s
  "hint-level" is refined to **captured-but-sparse-and-unlowered** — but ISF has **no declarative static-topology
  construct** (composition is transaction-level only; FSMGen's ATL multi-actor frontier is behavioral generated-child
  wiring, not a declarative netlist; the SpecForge emit is single-initiator-actor so topology is structurally absent by
  design). **No FSMGen FR is filed yet** — premature because the topology capture is sparse/noisy AND ISF is a per-actor
  format where static topology may deliberately be the integrator's concern above per-module synthesis (a scoping question
  for FSMGen). The buildable next step is a **measurement** (`.4c.i` topology-capture recall), after which the construct
  decision (FR vs honest-non-target) becomes real. No fabrication, no speculative FR (`[[feedback_verify_fsmgen_before_fr]]`
  / `[[feedback_isf_no_hacks]]`).
- `2026-06-23` (`.4e`): the `conditional_rules` ISF-lowering shortfall (`.2` Result 3) is **honest residual, not a gap**.
  Per-item triage of 603 rules / 9 docs: 73% are prose conditions / undeclared-signal / placeholder consequents (cannot
  become an ISF `(rule)` without fabrication), and the 27% declared-consequent bucket is 161/164 **bare deontic modals**
  (`shall`/`must`) with no concrete obligation — only 3/603 carry a concrete value/level cue. There is **no buildable ISF
  lever and no FSMGen FR**: the adapter already lowers the cleanly-grounded conditional obligations (516 corpus-wide), and
  rendering a bare modal would fabricate the obligation (`[[feedback_isf_no_hacks]]`). The only improvement path is
  **upstream extraction quality** (recover the concrete obligation from the modal conditionals' `source_text`), owned by
  the extraction-quality program (`extract-constraints-llm` / `EXTRACTION-QUALITY-GAUGE`), not the `.4` ISF-lowering
  program — recorded as a cross-reference, NOT minted as a `.4` gap (`[[feedback_scoring_rigor]]`). With `.4e` closed, the
  `.2` per-category scorecard's **measurement phase is complete**; the remaining `.4` work is CODE (`.4d.i`, `.4c.i`,
  `.4b`).

## Open Questions

- Exact boundary cues between category 2 (register IP) and 3 (platform/system-IP) when a TRM carries both a register map
  and a topology — resolved empirically in `.1`/`.3` (does not block `.0`).
- ~~Whether category 4 (ISA) lowering needs a new ISF construct or maps onto existing register/storage abstractions~~ —
  **RESOLVED `2026-06-23` by `.4d`**: CSR / register intent **maps onto the existing register/storage abstraction** (no
  new ISF construct — FSMGen titles `(storage (var … (fields …)))` the register-map/CSR construct; CSRs are structurally
  registers); the cat-4 register *gap* is **extraction recall** (unlocated fields / uncaptured AIA registers), spun out as
  `.4d.i`; instruction/privilege/exception/memory-ordering are **honest non-targets** (no ISF construct, no FR). See
  `docs/research/cat4-isa-csr-lowering-decision.md` / `[[cat4-isa-csr-lowering-decision]]`.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.0` | mdBook build; memory-arch self-check; knowledge-map derive-and-diff; no code → golds/`kg-bench` orthogonal | PASS (committed `8815a8c5`) |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.1` | read-only profile of 78 persisted docs (no `validate` → zero mutation); distribution 36/7/15/2/4/14; memory-arch + knowledge-map gates; no code → golds/`kg-bench` orthogonal | PASS |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.2` | read-only per-surface lowering gauge over 78 docs (76 `adapter.json` + 2 `adapt --dry-run`, verified no write); measured 12,638 register-fields→0 + 1,220 msg-fields→0 (no Intent carrier); reproducer `scripts/measure_isf_completeness.py`; mdBook builds; memory-arch + knowledge-map (112 facts) gates green; no code/canonical mutation → golds/`kg-bench` orthogonal | PASS |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.3a` | design slice (no code): recognizer algorithm grounded in `completeness.rs` (`classify_document`) + measured `.1`/`.2` evidence; verified the cues are in scope at `validate.rs` ~2888; memory-arch + knowledge-map gates green; no code → golds/`kg-bench` orthogonal | PASS |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.3b` | `cargo fmt --all --check` clean; `cargo clippy --all-targets -- -D warnings` clean; completeness lib `66/66` (13 new recognizer tests); full `cargo test` `1695 passed; 0 failed` (warning-deny); `kg-bench 156/156`; live `validate` over all 78 docs → 21 wire / 8 guide (high) / 28 register-or-platform / 16 unresolved / 5 PHY (low), 0 high-confidence false positives; WIRE-BASED-100 orthogonal (pure new fn + additive reporting, no extraction/emitter touch) | PASS (committed `d6239217`) |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.3c` | end-to-end integration test `validate_evidence_ir_reports_document_intent_category` PASS; `kg-bench 156/156`; full `cargo test` green (warning-deny, +1 test); `cargo fmt`/`clippy -D warnings` clean; `mdbook build` green; knowledge-map derive-and-diff in sync (113→114 facts); `run_ci.sh` green; WIRE-BASED-100 orthogonal (test + docs only) | PASS |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.4a` | measurement/design + verified FSMGen FR (docs-only, no Rust code); code-path map (`source.rs:414`→`intent.rs:193`→`isf_ir.rs:852`) + empirical FSMGen-storage probe on pin `030f8c273` (opaque `(var)` only, no field structure); `scripts/check_doctrines.sh` green; `mdbook build` green; knowledge-map derive-and-diff in sync (114→115 facts / 824 keys); no code/canonical mutation → golds + `kg-bench` orthogonal by construction | PASS |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.4a.ii` | CODE — emit register bit-fields to ISF `(fields …)`. `cargo fmt --all --check` clean; `cargo clippy --all-targets -D warnings` clean; full `cargo test` `1702 passed / 0 failed` (warning-deny, +6 new tests incl. `register_fields_pass_fsmgen_strict_and_round_trip`); `kg-bench 156/156`; real-emitter corpus scan **6,570 fields / 2,531 registers / 24 docs** (was 0); RISC-V IOMMU (122) / GIC `ihi0069` (424) / CoreSight SoC-600 (974) `fsmgen --strict` `success / 0 diags` before AND after (0 new); 4 wire golds (AXI/APB/AHB/AXI-Stream) emitted `.isf` **byte-identical** (`adapt` old-vs-new diff empty); `inferred_storage[].fields[]` round-trip asserted; `run_ci.sh` green | PASS |
| `2026-06-23` | `DOC-INTENT-TAXONOMY.4d` | measurement + decision packet (read-only, docs-only — no Rust code); profiled the 2 cat-4 docs off persisted IR (RISC-V Debug 44 regs / 179 fields / **0 located**; RISC-V AIA **0 registers**, CSR intent in 39 prose `conditional_rules` + 211 empty `interfaces`); re-verified FSMGen pin `d327129b7` ISF (storage = register-map/CSR construct; no instruction/privilege/exception construct); decision = reuse register/storage for CSRs + spin out `.4d.i` extraction-recall lever + honest non-target for non-register ISA semantics; `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync (115→116 facts); no code/canonical mutation → golds + `kg-bench` orthogonal by construction | PASS |
| `2026-06-23` | `DOC-INTENT-TAXONOMY.4c` | measurement + decision packet (read-only, docs-only — no Rust code); profiled 3 representative cat-3 docs off persisted IR (CoreSight SoC-600 60 actors / **6** `signal_connectivity` / 833 regs; GIC-600 55 actors / **66** `signal_connectivity` / **2** `infrastructure_signals`; CoreSight BSA 5 actors / 0 connectivity / 88 prose interfaces); re-verified FSMGen pin `d327129b7` (composition transaction-level only; ATL frontier = behavioral generated-child wiring, not a declarative netlist); decision = register half already lowers + topology captured-but-sparse-and-unlowered + no static-topology ISF construct + NO FR yet → spun out `.4c.i` capture-recall measurement; `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync (116→117 facts); no code/canonical mutation → golds + `kg-bench` orthogonal by construction | PASS |
| `2026-06-23` | `DOC-INTENT-TAXONOMY.4e` | measurement triage (read-only, docs-only — no Rust code); classified 603 `conditional_rules` across 9 representative docs (all 4 buildable categories) vs each doc's declared-signal inventory (`interfaces[].signals` + `signal_records[].signal_name` + `actor_ports[].signal_name`) + consequent quality → A no-consequent 392 / B undeclared 14 / C placeholder 33 / D declared+action 164 (of which 161 bare modal `shall`/`must`, only 3/603 concrete); verdict = honest residual, no ISF lever, no FR (only upstream extraction-quality upside); `.2` Result 3 closed; `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green; knowledge-map derive-and-diff in sync (117→118 facts); no code/canonical mutation → golds + `kg-bench` orthogonal by construction | PASS |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `DOC-INTENT-TAXONOMY.0` | `8815a8c5` `DOC-INTENT-TAXONOMY.0 — define the 6-category chip-spec intent taxonomy + capture in mdBook` | docs-only |
| `DOC-INTENT-TAXONOMY.1` | `DOC-INTENT-TAXONOMY.1 — corpus census by category (36/7/15/2/4/14)` | read-only measurement |
| `DOC-INTENT-TAXONOMY.2` | `DOC-INTENT-TAXONOMY.2 — per-category ISF-completeness gauge (register fields 12,638→0; structures 1,220→0)` | read-only measurement |
| `DOC-INTENT-TAXONOMY.3a` | `DOC-INTENT-TAXONOMY.3a — recognizer design (grounded; honest-residual, ADR-0006)` | design slice, no code |
| `DOC-INTENT-TAXONOMY.3b` | `d6239217` `DOC-INTENT-TAXONOMY.3b — implement the 6-category purpose recognizer (validate-reported; measured refinement of .3a)` | code slice |
| `DOC-INTENT-TAXONOMY.3c` | `DOC-INTENT-TAXONOMY.3c — recognizer fixtures + user-facing mdBook chapter + KM card (.3 recognizer complete)` | test + docs slice |
| `DOC-INTENT-TAXONOMY.4a` | `DOC-INTENT-TAXONOMY.4a — Gap A register bit-field ISF lowering: verified FSMGen field-structured-storage FR (measurement/design)` | measurement/design + FR, docs-only |
| `DOC-INTENT-TAXONOMY.4a.ii` | `DOC-INTENT-TAXONOMY.4a.ii — emit register bit-field map into ISF field-structured storage (6,570 fields / 24 docs)` | code slice (isf_ir.rs emitter) |
| `DOC-INTENT-TAXONOMY.4d` | `DOC-INTENT-TAXONOMY.4d — cat-4 ISA/CSR lowering decision: CSRs reuse register/storage; gap is extraction recall (spun out .4d.i)` | measurement + decision packet, docs-only |
| `DOC-INTENT-TAXONOMY.4c` | `DOC-INTENT-TAXONOMY.4c — cat-3 topology lowering decision: topology captured-but-sparse, no static-topology ISF construct, no FR yet (spun out .4c.i)` | measurement + decision packet, docs-only |
| `DOC-INTENT-TAXONOMY.4e` | `DOC-INTENT-TAXONOMY.4e — conditional-rule lowering triage (.2 Result 3): honest residual, no ISF lever, no FR (.2 measurement phase complete)` | measurement triage, docs-only |

## Changelog

- `2026-06-23`: **`.4e` DONE (measurement triage, read-only, docs-only — no Rust code)** — the `conditional_rules`
  ISF-lowering shortfall (`.2` Result 3) is **honest residual, not a gap**. Per-item triage of 603 rules across 9
  representative docs (all 4 buildable categories) against each doc's declared-signal inventory + consequent quality: A
  no-consequent prose 392 (65%) / B undeclared signal 14 (2%) / C declared+placeholder 33 (6%) / D declared+non-placeholder
  action 164 (27%). The D "candidate-lever" bucket is NOT a clean lever — **161/164 are bare deontic modals**
  (`shall`/`must`/`shall not`) with no concrete obligation, only **3/603** carry a concrete value/level cue. Lowering a
  bare modal would fabricate the obligation (`[[feedback_isf_no_hacks]]`). **VERDICT: no buildable ISF lever, no FSMGen
  FR** — the adapter already lowers the 516 cleanly-grounded conditional obligations corpus-wide; the only upside is
  upstream EXTRACTION quality (recover the concrete obligation from the modal conditionals' `source_text`), owned by the
  extraction-quality program, distinct from `.4` and lower-leverage than register/structure/topology — recorded as a
  cross-reference, NOT minted as a `.4` gap (`[[feedback_scoring_rigor]]`). Closes `.2` Result 3 → the `.2` per-category
  scorecard's **measurement phase is COMPLETE**; remaining `.4` work is CODE (`.4d.i`, `.4c.i`, `.4b`). Report
  `docs/research/conditional-rule-lowering-triage.md`; KM `[[conditional-rule-lowering-triage]]` (map 117→118).
  `check_doctrines.sh` + `mdbook build` green; no code/canonical mutation → golds/`kg-bench` orthogonal.
- `2026-06-23`: **`.4c` DONE (measurement + decision packet, read-only, docs-only — no Rust code)** — resolved the cat-3
  (platform / system-IP topology) ISF-lowering question from measured evidence over 3 representative cat-3 docs (of 15) +
  the current FSMGen pin `d327129b7`. **Two halves:** (1) **the register half already lowers** (register maps + bit-fields
  via `.4a.ii`; CoreSight SoC-600 ~3,250 fields, GIC-600 33 regs; infrastructure signals; actor ports — same road as
  cat-2, not the gap); (2) **the distinctive topology half is captured but unlowerable** — cat-3 docs carry a typed
  topology surface (`signal_connectivity` producer→consumer graph — GIC-600 66 edges / SoC-600 6 — and
  `infrastructure_signals` with `infrastructure_topology`/`distributed_to_actor_ids` clock-reset distribution), refining
  `.2`'s "hint-level" to **captured-but-sparse-and-unlowered**, but ISF has **no declarative static-topology construct**
  (composition is transaction-level only, `13f-composition.md`; FSMGen's ATL multi-actor frontier is BEHAVIORAL
  generated-child transaction wiring, not a declarative IP-interconnect netlist, `14-feature-backlog.md`; the SpecForge
  emit is single-initiator-actor — `KG-ISF-COMPLETENESS.2a.ii` — so cross-component topology is structurally absent by
  design). **NO FSMGen FR filed yet** — premature because the topology capture is sparse/noisy (6 edges across SoC-600's
  60 actors; `None`/escaped actor names) AND ISF is a per-actor format where static topology may deliberately be the
  integrator's concern above per-module synthesis (resolve WITH FSMGen — `[[feedback_verify_fsmgen_before_fr]]`). Topology
  stays an honest residual; next lever is a **measurement** (`.4c.i` topology-capture recall across all 15 cat-3 docs),
  after which the construct decision (FR vs honest-non-target) becomes real. Report
  `docs/research/cat3-topology-isf-lowering-decision.md`; KM `[[cat3-topology-isf-lowering-decision]]` (map 116→117); book
  `document-categories.md` cat-3 maturity refined. `check_doctrines.sh` + `mdbook build` green; no code/canonical mutation
  → golds/`kg-bench` orthogonal. Frontier → `.4c.i` (cat-3 measurement) / `.4d.i` (cat-4 code) / `.4e` (conditional
  triage) / `.4b` Gap B (gated).
- `2026-06-23`: **`.4d` DONE (measurement + decision packet, read-only, docs-only — no Rust code)** — resolved the cat-4
  (CPU ISA / privileged architecture) ISF-lowering Open Question from measured evidence over the corpus's 2 cat-4 docs
  (RISC-V Debug, RISC-V AIA) and the current FSMGen pin `d327129b7`. **Cat-4 intent splits into three:** (1) **CSR /
  register intent REUSES the existing register/storage abstraction** — no new ISF construct, no FSMGen FR (CSRs are
  structurally registers — RISC-V Debug captures its 44 Debug-Module CSRs as `register_records`; FSMGen titles
  `(storage (var … (fields …)))` the "register map / CSR" construct, `13a-actor-interface.md:419`/`:468` — the same
  construct `.4a.ii` lowers cat-2/cat-3 fields into); (2) **the cat-4 register gap is EXTRACTION RECALL, not abstraction**
  — RISC-V Debug captures 44 regs / 179 fields with name/access/reset/description but **0/179 located** (no bit positions;
  bit-layout column unparsed by the `field_table` strategy) and RISC-V AIA captures **0 registers** (its IMSIC/APLIC CSR
  intent is in prose `conditional_rules`/`behaviors`; 211 empty prose "interfaces"), so fields reach `.isf` 0 times
  because they aren't *located*, not because ISF can't express them → spun out **`.4d.i`** (recover RISC-V CSR field bit
  positions + a RISC-V-shaped register recogniser; once located, fields auto-lower via `.4a.ii`); (3) **instruction /
  privilege-mode / exception / memory-ordering → honest NON-TARGET** (software-visible ISA semantics, not synthesizable
  hardware intent; ISF has no construct + FSMGen lists none → no FR per `[[feedback_verify_fsmgen_before_fr]]` /
  `[[feedback_isf_no_hacks]]`; conditional-future only if FSMGen's SV/UVM path scopes ISA-model verification). Report
  `docs/research/cat4-isa-csr-lowering-decision.md`; KM `[[cat4-isa-csr-lowering-decision]]` (map 115→116); book
  `document-categories.md` cat-4 maturity refined. `check_doctrines.sh` + `mdbook build` green; no code/canonical mutation
  → golds/`kg-bench` orthogonal. Frontier → `.4d.i` (cat-4 build lever) / `.4b` Gap B (gated) / `.4c` cat-3 topology /
  `.4e` conditional triage.
- `2026-06-22`: **`.4a.ii` DONE (CODE)** — emitted the IntentIR register bit-field map into FSMGen's shipped declarative
  field-structured-storage construct (pin `d327129b7`). New `IsfStorageField` + `IsfStorageVar.fields` + the render of
  the nested `(fields (field NAME (bits HI LO) [(access …)] [(reset V)] [(enum (M V)…)]) …)` block, fed by the pure
  `register_storage_fields` (`isf_ir.rs`) and `normalize_field_access`. Admission is structural/fail-closed (ADR-0006):
  located fields only (unlocated → honest gap); a sanitized-name collision (count ≥ 2) drops the whole colliding group
  (uniformly handles `res0`-style reserved gaps + flattened mis-extraction dups → FSMGen's duplicate-name fail-closed);
  any residual overlap fails the register's field block closed; `(access)` normalized to FSMGen's 10-token set (omitted
  when unmapped); a field `(reset)` only when the parent reset is composed, as that value's own bit slice (matches the
  parent slice by construction); `(enum)` keeps width-fitting numeric members. The unlowered remainder is recorded as
  the `isf_register_fields_not_lowered` adapter residual (folds in `.4a.i`). **Measured live with the real emitter:
  6,570 register bit-fields now reach `.isf` across 2,531 registers in 24 docs (was 0)** — CoreSight SoC-600
  1,166/1,118/974, GIC arch `ihi0069` 424, CCIX 380, SMMU `ihi0070` 332, CHI-C2C 276, … Metadata-only / schedule-safe:
  the 4 WIRE-BASED-100 golds emit 0 fields → emitted `.isf` byte-identical (WIRE-BASED-100 orthogonal); RISC-V IOMMU /
  GIC / CoreSight SoC-600 keep `fsmgen --strict` `success / 0 diagnostics` (0 NEW); `inferred_storage[].fields[]`
  round-trip asserted on the live binary; `kg-bench 156/156`; `cargo test 1702/0` (warning-deny, +6 tests);
  `run_ci.sh` green. `.4a.i` is superseded (it both emits the fields and records the unlowered residual). Frontier →
  `.4b` Gap B (FSMGen-deferred packet/flit) / `.4c` cat-3 topology / `.4d` cat-4 ISA / `.4e` conditional triage.
- `2026-06-22`: **`.4a.ii` UN-GATED** — FSMGen SHIPPED the declarative field-structured-storage construct
  (`ISF-FIELD-STRUCTURED-STORAGE-FRONTIER.1`/`.2`, pin `d327129b7`, ingested via `FSMGEN-REFRESH-INTEGRATE-5`). The Gap-A
  field-structured emit is now buildable and is the highest-leverage lever; it supersedes `.4a.i` (the honest residual,
  now a fallback). Captured the exact shipped grammar `(var … (fields (field NAME (bits HI LO) [(access …)] [(reset V)]
  [(enum …)])))`, the `RegisterFieldRecord`→ISF mapping, the fail-closed rules (overlap/out-of-width/unsupported-access/
  field-reset-must-match-parent-slice/enum-fits), and the `inferred_storage[].fields[]` verification surface in the
  `.4a.ii` node. Gap B (`.4b`, packet/flit structures) stays FSMGen-deferred. No SpecForge code yet (refresh + un-gate
  only) → golds/`kg-bench` orthogonal.
- `2026-06-22`: `.4a` Gap A — register bit-field ISF lowering DONE (measurement/design + verified FSMGen FR, docs-only,
  no Rust code). Two read-only probes localized the loss and decided the fix: (1) the bit-field metadata is fully
  captured (`RegisterFieldRecord`, `source.rs:414`) and carried UNCHANGED into IntentIR
  (`IntentIr.register_records` clone, `intent.rs:193`), dropped ONLY at the ISF-emit boundary
  (`IsfStorageVar { name, width, reset }`, `isf_ir.rs:852`; rendered opaque `(var NAME (width N) [(reset V)])` at
  `:391`) → no SpecForge carry gap; (2) an empirical read of the pinned `subs/fsmgen` (`030f8c273`) proved the ISF
  `(storage …)` grammar declares opaque width-only scalars only — no named-bit-field / packed-record construct (the
  shipped `set-field`/`extract` are runtime ops, not a static field-map declaration; not on the FSMGen backlog). Gap A
  is a missing ISF abstraction → filed a **verified FSMGen FR** (`docs/FSMGEN_FEEDBACK.md`,
  `## Feature request (2026-06-22) — declarative field-structured storage`), NOT an emitter hack (the three
  emitter-only alternatives rejected as fabrication/loss). Gap B (1,220 message-fields) shares the same missing
  abstraction and additionally lacks an `Evidence→Intent` carrier (zero `message_field` in `intent.rs`). Decomposed
  `.4` → `.4a` (done) / `.4a.i` adapter honest-residual (code) / `.4a.ii` field-structured emit (gated on FSMGen) /
  `.4b` Gap B carrier / `.4c` cat-3 topology / `.4d` cat-4 ISA / `.4e` conditional-rule triage. Report
  `docs/research/register-bit-field-isf-lowering-design.md`; KM `[[register-bit-field-isf-lowering-gap]]` (map
  114→115); book honest-residual note in `docs/book/src/pipeline/isf-adapter.md`. `check_doctrines.sh` + `mdbook build`
  green; no code/canonical mutation → golds/`kg-bench` orthogonal. Frontier → `.4a.i` (RAM-light, no FSMGen dependency)
  then `.4b`.
- `2026-06-22`: `.3c` recognizer FIXTURES + book + KM landed (test + docs slice) → **`.3` recognizer COMPLETE**. Added the
  end-to-end integration test `validate_evidence_ir_reports_document_intent_category` (the metric + confidence + finding
  reach the persisted report through the real pipeline; near-empty → honest `unresolved` residual); the per-category
  gold/negative cases stay locked by the 13 `.3b` in-file unit tests. User-facing mdBook: new purpose-category section in
  `docs/book/src/quality/validation.md` beside `document_class` + `document-categories.md` flipped from "target" to "now
  CLI-reported". KM fact card `document-intent-category-recognizer` (map 113→114 facts / 815 keys). ISA/PHY vocab
  precision-verified (ISA 0 corpus docs — honest fall-through; PHY recovers all 4 OpenCAPI PHY docs). kg-bench 156/156;
  cargo test green; run_ci.sh green; WIRE-BASED-100 orthogonal. Frontier → `.4+` per-category levers (Gap-A register
  bit-field lowering) + FSMGen ISF-abstraction FRs.
- `2026-06-22`: `.3b` recognizer IMPLEMENTED + reported by `validate` (code slice), committed `d6239217`. Added `DocumentIntentCategory`
  (6 purpose variants + `Unresolved`), `IntentCategoryConfidence`, `DocumentIntentClassification`, and the pure
  `classify_document_intent_category` to `completeness.rs` (sibling to `classify_document`), plus the front-matter
  ISA/PHY self-declaration helpers; wired into `validate.rs` at the shared census site (bound once, fed to both
  classifiers) as a printed block, an `evidence_document_intent_category` Info finding, and `document_intent_category` /
  `document_intent_category_confidence` metrics. **Refined the `.3a` flit clause** after per-document measurement proved
  it would misclassify register/structure docs (NVMe/AMD-IOMMU/GIC-600) as wire: flit fields count as cat-1 only when no
  register map is present, plus a wire-vs-structure weight dominance test (register count never vetoes a clean wire shape:
  AXI wire 401 ≥ struct 229). Only clean wire + self-declared guide are HIGH confidence; everything else is LOW + explicit
  residual. Live over 78 docs: 21 wire / 8 guide (high) / 28 register-or-platform / 16 unresolved / 5 PHY (low), 0
  high-confidence false positives. kg-bench 156/156; cargo test 1695/0; fmt/clippy clean; WIRE-BASED-100 orthogonal.
  Frontier → `.3c` (fixtures + mdBook chapter + KM card + ISA/PHY vocab calibration).
- `2026-06-22`: `.3a` recognizer DESIGN pinned (design slice, no code). Grounded the fast category recognizer in
  the real `completeness.rs` classifier (`classify_document`) + the measured `.1`/`.2` evidence: extend the census
  with `message_field_records` (the 1,220-field cat-1-msg/cat-2-structure cue), a 6-category + `Unresolved` output
  carrying confidence + an explicit residual, and a decision order that (1) honors a self-declared guide over
  spurious surface counts (the `.2` over-extraction finding), (2) rescues register-heavy protocols via wire shape
  (register count never vetoes cat 1), and (3) emits honest low-confidence residuals where `.1` proved structure
  cannot decide (cat 2↔3, cat 4 ISA, cat 5↔6) — never a forced guess. Verified the cues are in scope at the
  evidence-validate census site (`validate.rs` ~2888). Decomposed `.3` → `.3a` (design, done) / `.3b` (implement) /
  `.3c` (fixtures+CI). Frontier → `.3b` (build-heavy → fresh session). No code → golds/`kg-bench` orthogonal.
- `2026-06-22`: `.2` per-category ISF-lowering completeness gauge DONE (read-only). Per-surface lowering ledger
  over all 78 docs (76 persisted `adapter.json` + 2 read-only `adapt --dry-run`; no `validate`/`adapt` write →
  zero mutation), reproducible via tracked `scripts/measure_isf_completeness.py`. Measured the `.0` maturity
  column: cat 1 MATURE / cat 2 + 3 PARTIAL / cat 4 THIN / cat 5 + 6 honest non-targets. Two dominant TRUE GAPS:
  (A) register **bit-fields** 12,638 captured → 0 lowered across 32 docs (registers emit opaque width-only
  storage; cat 3 largest at 9,308); (B) message-field **structures** 1,220 captured at EvidenceIR → 0 carried to
  IntentIR (`has_msgfld_key=false`) across 11 docs. Both converge on the same FSMGen ISF-abstraction need
  (field-structured storage + packet/structure layouts). 5/14 cat-6 guides over-extract (precision motive for
  `.3`, not a completeness gap). Report `docs/research/document-intent-isf-completeness.md`; KM
  `[[document-intent-isf-completeness]]` (map now 112 facts). Frontier → `.3` fast category recognizer; `.4+`
  Gap-A register-field lowering as the highest-leverage first lever.
- `2026-06-22`: `.1` corpus census by category DONE (read-only). Profiled all 78 persisted docs by typed surface
  (no `validate` → zero mutation). Distribution: 36 wire-protocol / 7 register-IP / 15 platform-system-IP / 2
  CPU-ISA / 4 PHY / 14 methodology-guide. Measured 3 structural blind spots (cat 2↔3 inseparable; ISA has no
  signature; cat 5↔6 indistinguishable) + the register-heavy-protocol trap (8 cat-1 protocols are
  register/message-dominant). Report `docs/research/document-intent-category-census.md`; KM
  `[[document-intent-category-census]]`. Frontier → `.2` per-category ISF-completeness gauge.
- `2026-06-22`: Created on the owner's multi-message directive (intent-category taxonomy + ISF-as-synthesis-for-all-categories).
  `.0` DONE (docs-only): defined the precise 6-category purpose taxonomy as the guiding lens, captured it identically in
  the mdBook (`document-categories.md`) and this tree, recorded the honest per-category ISF-synthesis maturity scorecard,
  and registered the owner's FSMGen dual-path (synthesizable HDL + verification SV/UVM + VHDL) + new-ISF-abstraction
  (memory banks, single/dual-port memory) context. Frontier → `.1` corpus census by category.
