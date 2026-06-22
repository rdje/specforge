# DOC-INTENT-TAXONOMY: chip-spec document intent taxonomy → per-category complete ISF synthesis

## Metadata

- Tree ID: `DOC-INTENT-TAXONOMY`
- Status: `active` (`.0` taxonomy definition + capture DONE `2026-06-22`; `.1` corpus census by category DONE `2026-06-22`, read-only; `.2` per-category ISF-completeness gauge DONE `2026-06-22`, read-only; `.3a` recognizer design DONE `2026-06-22`; `.3b` recognizer IMPLEMENTED + validate-reported DONE `2026-06-22`, code; frontier → `.3c` fixtures + book + KM)
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
- ID: `DOC-INTENT-TAXONOMY.3` · Status: `in-progress` (`.3a` design DONE, `.3b` implement DONE; `.3c` fixtures+CI+book
  remaining) · Goal: **fast deterministic category recognizer** so `inspect`/`validate` immediately report a
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
  - `.3c` · Status: `pending` · **Fixtures + CI** — gold/negative fixtures locking each category and the honest-residual
    cases (the register-heavy-protocol rescue, the cat-6-over-extraction front-matter override, the cat 2↔3 / cat 4 /
    cat 5↔6 honest-low-confidence residuals); `scripts/run_ci.sh` green; book + live-doc sync.

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
- ID: `DOC-INTENT-TAXONOMY.4+` · Status: `pending` · Goal: **per-category completeness levers** — cat-2 structure /
  message-field table recall (the IOMMU Lever-D), cat-3 topology, cat-4 ISA/CSR lowering, and the **FSMGen-ISF
  abstraction feedback** (memory banks, single/dual-port memory modules, … across FSMGen's synthesizable-HDL and new
  verification-oriented SV/UVM + VHDL paths). Each is its own owned leaf; any FSMGen FR is filed only after empirically
  verifying the current submodule (`[[feedback_verify_fsmgen_before_fr]]`, `docs/FSMGEN_FEEDBACK.md`).

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

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `DOC-INTENT-TAXONOMY.1` | `done` (`2026-06-22`) | Census DONE — distribution 36/7/15/2/4/14; denominator established. |
| — | `DOC-INTENT-TAXONOMY.2` | `done` (`2026-06-22`) | ISF-completeness scorecard MEASURED — maturity column now objective; two dominant true gaps (register bit-fields 12,638→0; message-field structures 1,220→0, no Intent carrier). |
| — | `DOC-INTENT-TAXONOMY.3a` | `done` (`2026-06-22`) | Recognizer DESIGN pinned (grounded in `completeness.rs` + measured `.1`/`.2` evidence): cues, decision order, honest-residual policy, ADR-0006 genericity — de-risks the hard name-list-free classifier before coding. |
| — | `DOC-INTENT-TAXONOMY.3b` | `done` (`2026-06-22`) | Recognizer IMPLEMENTED + reported by `validate` (`classify_document_intent_category` in `completeness.rs`, wired into `validate.rs`); corpus: 21 wire / 8 guide (both high) / 28 register-or-platform / 16 unresolved / 5 PHY (low) with 0 high-confidence false positives. Refined the `.3a` flit clause its own data falsified. kg-bench 156/156, cargo test 1695/0, clippy/fmt clean. |
| 1 | `DOC-INTENT-TAXONOMY.3c` | `pending` | Fixtures lock gold/negative + honest-residual cases (register-heavy-protocol rescue, guide front-matter override, 2↔3 / 4 / 5↔6 residuals); user-facing mdBook chapter (`quality/validation.md` + `document-categories.md` "now live"); KM card; calibrate ISA/PHY front-matter vocab against real strings; `run_ci.sh` green. |
| 2 | `DOC-INTENT-TAXONOMY.4+` | `pending` | Per-category levers; `.2` makes **register bit-field lowering (Gap A)** the highest-leverage first lever (32 docs, 12,638 fields), then **message-field structure carry + lowering (Gap B)** — both pending the same FSMGen ISF-abstraction (field-structured storage / packet layouts), filed as verified FRs after empirical submodule check. |

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

## Open Questions

- Exact boundary cues between category 2 (register IP) and 3 (platform/system-IP) when a TRM carries both a register map
  and a topology — resolved empirically in `.1`/`.3` (does not block `.0`).
- Whether category 4 (ISA) lowering needs a new ISF construct or maps onto existing register/storage abstractions —
  `.2` measured cat 4 as THIN (only the register-shaped surface lowers; bit-fields 0, no instruction/CSR/privilege/
  exception construct); the construct-vs-reuse decision is now an owned `.4+` decision packet, informed by Gap A
  (register bit-field lowering) which cat 4 shares.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.0` | mdBook build; memory-arch self-check; knowledge-map derive-and-diff; no code → golds/`kg-bench` orthogonal | PASS (committed `8815a8c5`) |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.1` | read-only profile of 78 persisted docs (no `validate` → zero mutation); distribution 36/7/15/2/4/14; memory-arch + knowledge-map gates; no code → golds/`kg-bench` orthogonal | PASS |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.2` | read-only per-surface lowering gauge over 78 docs (76 `adapter.json` + 2 `adapt --dry-run`, verified no write); measured 12,638 register-fields→0 + 1,220 msg-fields→0 (no Intent carrier); reproducer `scripts/measure_isf_completeness.py`; mdBook builds; memory-arch + knowledge-map (112 facts) gates green; no code/canonical mutation → golds/`kg-bench` orthogonal | PASS |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.3a` | design slice (no code): recognizer algorithm grounded in `completeness.rs` (`classify_document`) + measured `.1`/`.2` evidence; verified the cues are in scope at `validate.rs` ~2888; memory-arch + knowledge-map gates green; no code → golds/`kg-bench` orthogonal | PASS |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.3b` | `cargo fmt --all --check` clean; `cargo clippy --all-targets -- -D warnings` clean; completeness lib `66/66` (13 new recognizer tests); full `cargo test` `1695 passed; 0 failed` (warning-deny); `kg-bench 156/156`; live `validate` over all 78 docs → 21 wire / 8 guide (high) / 28 register-or-platform / 16 unresolved / 5 PHY (low), 0 high-confidence false positives; WIRE-BASED-100 orthogonal (pure new fn + additive reporting, no extraction/emitter touch) | PASS |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `DOC-INTENT-TAXONOMY.0` | `8815a8c5` `DOC-INTENT-TAXONOMY.0 — define the 6-category chip-spec intent taxonomy + capture in mdBook` | docs-only |
| `DOC-INTENT-TAXONOMY.1` | `DOC-INTENT-TAXONOMY.1 — corpus census by category (36/7/15/2/4/14)` | read-only measurement |
| `DOC-INTENT-TAXONOMY.2` | `DOC-INTENT-TAXONOMY.2 — per-category ISF-completeness gauge (register fields 12,638→0; structures 1,220→0)` | read-only measurement |
| `DOC-INTENT-TAXONOMY.3a` | `DOC-INTENT-TAXONOMY.3a — recognizer design (grounded; honest-residual, ADR-0006)` | design slice, no code |
| `DOC-INTENT-TAXONOMY.3b` | `DOC-INTENT-TAXONOMY.3b — implement the 6-category purpose recognizer (validate-reported; measured refinement of .3a)` | code slice |

## Changelog

- `2026-06-22`: `.3b` recognizer IMPLEMENTED + reported by `validate` (code slice). Added `DocumentIntentCategory`
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
