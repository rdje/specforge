# Chip-Spec Document Categories

Before SpecForge can extract a document's intent, it helps to step back and ask a simple question:
**what is this PDF actually *about*?**

Every chip-specification PDF has a *purpose*. One document defines how two blocks talk to each other on a
bus; another defines the registers software pokes to configure an IP; another defines a CPU's instruction
set. Knowing the **category** up front is not bookkeeping — it is the lens that tells SpecForge what
"complete intent" even *means* for that document, and therefore how to lower it faithfully to `.isf`.

This page describes the six categories SpecForge recognizes, what each one is *about*, and — honestly —
how completely SpecForge can synthesize each one to ISF today. It is meant to be read as a **guide**: when
you hand SpecForge a new PDF, this is the mental model for what to expect.

## Why categorize at all?

A wire-level bus protocol and a register programming manual are both "chip specs", but their intent lives
in completely different places. Holding both to the *same* completeness bar would be wrong twice over:

- it would penalize the register manual for having no handshake timing (it was never supposed to), and
- it would let the bus protocol off the hook for missing a transaction (which it absolutely should have).

So SpecForge treats the category as a **per-document yardstick**. A register IP is "complete" when its
registers, fields, and structures are captured; a bus protocol is "complete" when its signals,
transactions, relations, and temporal rules are captured. Same tool, different bar — chosen by category.

> **ISF is the synthesis target for *every* category.** SpecForge's north star is that each document's
> intent, whatever its category, is recovered into a complete `IntentIR` and lowered fully to `.isf`.
> `.isf` is how SpecForge *synthesizes* the PDF's intent for the downstream FSMGen toolchain.

## The six categories

A document classifies on its **dominant** purpose. (A protocol spec may carry a small register appendix,
or a TRM may sketch a topology — those are secondary surfaces; the category is the document's center of
gravity.) The honest "ISF maturity" column reflects where SpecForge stands as of 2026-06-22.

| # | Category — what it is *about* | The intent SpecForge must capture | Examples | ISF-synthesis maturity (honest) |
|---|---|---|---|---|
| **1** | **Wire-level bus / interconnect protocol** | signals (direction/width), transactions, handshake & temporal rules, actor↔signal relations, polarity | APB, AHB, AXI, AXI-Stream, ACE, CHI, TileLink, Avalon, Wishbone, OCP, CXS/GFB/LTI/DTI/ATP/LPI, OpenCAPI, CCIX, USB, I²C, I²S, CAN, SMBus, SWD | **Mature** — IntentIR maps directly; ISF lowers richly (the wire-protocol gold suite scores 1.000) |
| **2** | **Programmable register / memory-mapped IP** | register maps, bit-fields, access/reset semantics, in-memory **structures** (descriptors, queues, page tables, contexts) | RISC-V IOMMU, AMD-IOMMU, Intel VT-d, GIC architecture, SMMU/MMU-700, CoreSight TRMs, NVMe, JEDEC eMMC EXT_CSD | **Mostly there** — register maps + reset lower, and their **named bit-fields now lower** to field-structured storage; the remaining frontier is **in-memory structure recall** (descriptors/queues) for non-AMBA layouts |
| **3** | **Platform / system-IP topology & integration** | components, connectivity, clock/reset infrastructure, programming model, integration contract | CoreSight SoC-600, GIC distributor/redistributor, interconnect fabrics | **Partial** — its register maps and bit-fields lower in volume (CoreSight SoC-600 ~3,250 fields), the same road as cat-2; its *topology* (component connectivity, clock/reset trees) is **captured but too sparse to lower** (measured 0.355 connectivity edges/component, 24% fully-connected, no resolved clock/reset source) — the gap is extraction recall, not a missing ISF construct |
| **4** | **CPU ISA / privileged architecture** | instructions, CSRs/registers, privilege modes, exceptions, memory-ordering model | RISC-V Debug, RISC-V Advanced Interrupt Architecture, ISA volumes | **Thin, now diagnosed** — its CSRs *are* registers and reuse the register/storage abstraction (their bit-fields lower once *located*); the remaining gap is **extraction recall** — the bit positions live in the register's *layout graphic* (a vision read), not in any parseable text table — **not** a missing ISF construct. Instructions, privilege modes, and exceptions are honest non-targets |
| **5** | **Physical / electrical / link layer** | signaling levels, encoding, link training, mechanicals | OpenCAPI 25G/32G PHY, USB4 PHY, mechanical specs | **Honest-thin** — this is not behavioral wire intent, so a near-empty `.isf` is *correct*, not a miss |
| **6** | **Methodology / language / EDA standard / guide** | *(not a chip contract)* — verification/modeling methodology, HDL/RDL languages, overview guides | UVM, SystemC/TLM, IP-XACT, SystemRDL, OVL, PSS, Liberty, LEF/DEF, overview/user guides | **Non-target** — recognized as such; SpecForge never forces chip intent out of these |

The most important honesty in that table is the last two rows: categories **5** and **6** are *supposed*
to produce little or no `.isf`. SpecForge reporting "nothing to synthesize here" for a PHY spec or a UVM
user guide is the **right** answer — not a failure to be papered over. The buildable program is
categories **1–4**.

## How completely does each category reach `.isf` today? (measured)

The "maturity" column above is not a vibe — it is **measured**. SpecForge profiled all 78 ingested documents
and asked, surface by surface, *how much of each document's captured intent actually appears in the emitted
`.isf`*, and — for whatever does not — whether that absence is **honest** (the document genuinely carries
nothing there) or a **real gap** (the intent was captured but no `.isf` construct yet expresses it). Measuring
per surface, rather than as one blended percentage, keeps the score honest: a single number is easy to game,
eight separate ones are not.

One finding used to dominate — register **bit-fields** were captured but not synthesized. That gap is now
**closed**: when this was first measured, **12,638 individual bit-fields** reached the `.isf` zero times even
though the register *maps* did lower. FSMGen then shipped a declarative field-structured-storage construct, and
SpecForge now lowers into it (`DOC-INTENT-TAXONOMY.4a.ii`):

- **Register maps *and* their bit-fields lower.** Each register lowers as a storage variable, and its named
  bit-fields are now emitted as a nested `(fields (field NAME (bits HI LO) [(access …)] [(reset V)] [(enum …)])
  …)` block — the part that says *which* bits mean *what*, with access and reset behavior. Across the corpus
  this is **6,570 bit-fields across 2,531 registers in 24 documents** (CoreSight SoC-600 alone contributes
  ~3,250). The fields that do *not* lower are honest residuals — bits with no documented range, ambiguous
  (repeated) field names, and overlapping or duplicate-named registers — and they stay in the IntentIR map and
  are summarized in the adapter's `residual_decisions`; nothing is fabricated.
- **In-memory structures are captured but not carried forward (still open).** Packet, flit, descriptor, queue,
  and page-table layouts — **1,220 fields** across 11 documents (NVMe's command structures, AMD-IOMMU's tables,
  CHI/DTI/CHI-C2C/CCIX message fields) — are recovered during extraction, but they currently stop one stage
  short of the canonical `IntentIR`, so they are not yet lowered. For the message-based coherent protocols,
  those flit fields are the real intent of the document.

The remaining structure gap needs both an `Evidence→IntentIR` carrier *and* a packet/structure-layout ISF
construct — the same family of abstractions (memory banks, single- and dual-port memories, structured records)
that FSMGen is growing. SpecForge's plan is to lower it the same way the bit-field gap was closed: file a
verified FSMGen feature request and lower into the shipped construct, rather than hack the emitter (see the
FSMGen feedback loop below).

The rest of the scorecard, in plain terms:

| Category | How complete to `.isf`, measured | The honest residual |
|---|---|---|
| **1 — wire protocol** | **Mature.** Signals, actor relations, constraints, timing rules, and enums all lower; the wire-protocol gold suite holds at a perfect 1.000. | Message/flit fields for the handful of register-heavy protocols (above); transaction *bodies* lower only where the document spells out the steps. |
| **2 — register IP** | **Mostly there.** Register maps, their **bit-fields** (bits / access / reset / enum), and enums lower. | In-memory structures (descriptors/queues) — still captured one stage short of `IntentIR` — plus the honest field residuals (unlocated, ambiguous, or overlapping bits). |
| **3 — platform / system-IP** | **Partial, now diagnosed.** Registers and their bit-fields lower in volume (thousands; CoreSight SoC-600 alone ~3,250 fields) — the same road as cat-2. | The *topology* (what connects to what, clock/reset trees) **is captured** as a typed producer→consumer connectivity + clock/reset distribution surface, but it is **too sparse to lower faithfully** — measured: only **0.355** connectivity edges per component (vs **4.108** on wire protocols, where the protocol signal graph *is* the topology), only **24%** of those edges name both a driver and a sink (vs 85% on wire), and **none** of the captured clock/reset signals resolve their source. So the gap is **extraction recall**, not a missing ISF construct — the same kind of gap as cat-4. Even with faithful capture, lowering would need a multi-actor `.isf` emit (today single-actor) and a static-topology construct ISF does not have — a question to resolve *with* FSMGen, or to treat topology as the *integrator's* concern above per-module synthesis. |
| **4 — CPU ISA** | **Thin, now diagnosed.** A CSR *is* a register: it reuses the register/storage abstraction, and its bit-fields lower the moment they are *located* — the same road as categories 2 and 3, no new construct needed. | The cat-4 register gap is **extraction recall**, not a missing abstraction: RISC-V CSR bit positions live in the register's bit-layout **graphic**, not in any text table (measured: **53 of 56** RISC-V Debug register diagrams are images; the handful Docling flattened to a table are garbled or carry symbolic `XLEN`-relative positions, so a *deterministic text parse would fabricate* — it was measured and rejected). So the recall lever is a **sharper vision read** of that graphic — the existing `recover-register-bits` command — bounded today by the local vision model's accuracy, not a parser. RISC-V Debug captures 44 registers but **0 located fields**; RISC-V AIA captures **0 registers** (its register intent sits in prose, and its document is not yet re-ingested). Instructions, privilege modes, exceptions, and memory-ordering are **honest non-targets** — software-visible ISA semantics, not synthesizable hardware intent. |
| **5 — PHY** | **Correctly near-empty.** A thin `.isf` is the right answer here. | — (not behavioral wire intent). |
| **6 — guide** | **Correctly near-empty for most.** | A few guides currently *over*-produce `.isf` content they shouldn't — a precision matter for the category recognizer, not a synthesis gap. |

The takeaway: **the wire-protocol road is built; the register, platform, and ISA roads are paved partway** —
but the next stretch differs by category. For register and platform IP it is *synthesizing* captured
structure (fields and layouts) once ISF can express it. For CPU ISA the lowering road already exists — a CSR
is a register — so the next stretch is *recall*, and it is a **vision** problem, not a parsing one: RISC-V
draws each register's bit positions in a layout *graphic* (measured: 53 of 56 Debug registers are images, and
the few that a backend flattened to text are garbled or `XLEN`-symbolic, so parsing them would fabricate). So
reading those positions is the job of the vision-backed `recover-register-bits` command, bounded today by how
sharply the local model reads dense diagrams — while the non-register ISA semantics (instructions, privilege,
exceptions) stay an honest non-target.

## How SpecForge determines a document's category

SpecForge infers a coarse structural **document class** (`protocol` / `register` / `interface` /
`guide`) purely from *which typed surfaces the extraction produced* — no vendor or chip names are ever
used (see [the genericity guardrail](architecture-rationale.md)). That class is a useful proxy, but it is
deliberately coarse: it folds the six purpose categories above into four buckets and has no distinct slot
for a CPU ISA.

The richer six-category taxonomy on this page is **now reported directly by the CLI**: `validate` emits a
`document_intent_category` — `wire-protocol`, `register-or-platform`, `cpu-isa`, `physical-link`,
`methodology-guide`, or an honest `unresolved` — beside the structural `document_class`, so the moment you
point it at a PDF you (and the pipeline) know which yardstick applies. It is deterministic and built on the
same structural, name-list-free signals (plus the document's own front-matter doc-type words), never on a
list of known specifications, and it is honest about uncertainty: only a clean wire-behavioural shape and a
self-declared guide are reported at **high** confidence, while the categories the structure genuinely cannot
separate — register-IP versus platform-IP (folded into `register-or-platform`), CPU-ISA, and
physical-versus-guide — are reported at **low** confidence with an explicit residual rather than a forced
guess. A register map never vetoes a real wire protocol, and message/packet fields count as wire intent
only when no register map is present, so a register-heavy bus protocol is not misfiled as a register IP. See
the [Validation](quality/validation.md) chapter for how to read the metric, the confidence, and the
residual. (The honest distribution over today's corpus: 21 `wire-protocol` and 8 `methodology-guide` at high
confidence with zero high-confidence mislabels, the rest reported at low confidence with their residuals.)

## What "fully handled" requires — and the FSMGen feedback loop

The goal is that **all six categories are fully handled**: each document's intent recovered completely and
synthesized to `.isf`. That is a deliberate, quality-first program, not a quick sweep — categories 2, 3,
and 4 each need real extraction and lowering work.

It also reaches *downstream*. ISF is consumed by FSMGen, which lowers it to hardware — and FSMGen is
growing **two** lowering paths: its default **synthesizable HDL** path, and a new **verification-oriented**
path (SystemVerilog/UVM + VHDL). Capturing every category naturally and elegantly will, at some point,
require ISF abstractions the current grammar does not yet have — for example **memory banks** and
**single- and dual-port memory modules** for the register/structure and platform categories. Where SpecForge
hits such a wall, the right move is **not** to hack the emitter, but to **feed the gap back to FSMGen** as a
considered feature request so ISF gains the abstraction cleanly (see the
[ISF Adapter](pipeline/isf-adapter.md) chapter and the project's FSMGen feedback notes). Both of FSMGen's
lowering paths benefit from a richer, more expressive ISF.

This taxonomy is the map for that journey: it names every destination, marks how far each road is built,
and makes the honest gaps visible so they can be closed one category at a time.
