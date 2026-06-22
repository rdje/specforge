# DOC-INTENT-TAXONOMY: chip-spec document intent taxonomy → per-category complete ISF synthesis

## Metadata

- Tree ID: `DOC-INTENT-TAXONOMY`
- Status: `active` (`.0` taxonomy definition + capture DONE `2026-06-22`; `.1` corpus census by category DONE `2026-06-22`, read-only)
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
- ID: `DOC-INTENT-TAXONOMY.2` · Status: `pending` · Goal: **per-category ISF-lowering completeness gauge** — for each
  category, measure honestly what fraction of the document's intent reaches `.isf` (and what is honest-absence vs a true
  gap), producing the prioritized scorecard. Acceptance: gauge + report; objectively measured, per-item demonstrated
  (`[[feedback_scoring_rigor]]`); no fabrication.
- ID: `DOC-INTENT-TAXONOMY.3` · Status: `pending` (gated on `.1`/`.2`) · Goal: **fast deterministic category recognizer**
  so `inspect`/`validate` immediately report a PDF's purpose category (the owner's "quickly determine which category").
  Likely a richer `document_intent_category` surface built on the typed-surface census + structural cues (ADR 0006, no
  name lists). Acceptance: CLI reports it; fixtures lock gold/negative classification; `run_ci.sh` green.
- ID: `DOC-INTENT-TAXONOMY.4+` · Status: `pending` · Goal: **per-category completeness levers** — cat-2 structure /
  message-field table recall (the IOMMU Lever-D), cat-3 topology, cat-4 ISA/CSR lowering, and the **FSMGen-ISF
  abstraction feedback** (memory banks, single/dual-port memory modules, … across FSMGen's synthesizable-HDL and new
  verification-oriented SV/UVM + VHDL paths). Each is its own owned leaf; any FSMGen FR is filed only after empirically
  verifying the current submodule (`[[feedback_verify_fsmgen_before_fr]]`, `docs/FSMGEN_FEEDBACK.md`).

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | `DOC-INTENT-TAXONOMY.1` | `done` (`2026-06-22`) | Census DONE — distribution 36/7/15/2/4/14; denominator established. |
| 1 | `DOC-INTENT-TAXONOMY.2` | `pending` | Per-category ISF-completeness scorecard — turns the maturity column from honest-estimate into objectively-measured (denominator now known per category). |
| 2 | `DOC-INTENT-TAXONOMY.3` | `pending` | Fast category recognizer in the CLI (the owner's "quickly determine which category") — the `.1` census shows it needs wire-relation shape + front-matter/self-declared type + topology cue, not surface counts alone. |

## Decisions

- `2026-06-22`: Adopt the **6-category purpose taxonomy** as the guiding lens (above). It is semantic ("what is the
  document about"), distinct from and built on the structural `document_class`. Categories 5 (PHY/electrical) and 6
  (methodology/guide) are **honest non-targets** for ISF synthesis; the buildable program is categories 1–4.
- `2026-06-22`: ISF is the synthesis target for ALL categories. Where the current FSMGen ISF cannot capture a category
  naturally/elegantly, the gap is fed back to FSMGen as a verified feature request rather than hacked into the emitter
  (`[[feedback_isf_no_hacks]]`). Owner context: FSMGen is adding a verification-oriented SV/UVM + VHDL lowering path
  alongside its default synthesizable HDL, and new ISF abstractions (memory banks, single/dual-port memory modules, …)
  are anticipated for both paths.

## Open Questions

- Exact boundary cues between category 2 (register IP) and 3 (platform/system-IP) when a TRM carries both a register map
  and a topology — resolved empirically in `.1`/`.3` (does not block `.0`).
- Whether category 4 (ISA) lowering needs a new ISF construct or maps onto existing register/storage abstractions —
  resolved in `.2`/`.4` after measurement (does not block `.0`).

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.0` | mdBook build; memory-arch self-check; knowledge-map derive-and-diff; no code → golds/`kg-bench` orthogonal | PASS (committed `8815a8c5`) |
| `2026-06-22` | `DOC-INTENT-TAXONOMY.1` | read-only profile of 78 persisted docs (no `validate` → zero mutation); distribution 36/7/15/2/4/14; memory-arch + knowledge-map gates; no code → golds/`kg-bench` orthogonal | PASS |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `DOC-INTENT-TAXONOMY.0` | `8815a8c5` `DOC-INTENT-TAXONOMY.0 — define the 6-category chip-spec intent taxonomy + capture in mdBook` | docs-only |
| `DOC-INTENT-TAXONOMY.1` | `DOC-INTENT-TAXONOMY.1 — corpus census by category (36/7/15/2/4/14)` | read-only measurement |

## Changelog

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
