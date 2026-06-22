# Document-intent category census (DOC-INTENT-TAXONOMY.1)

- Tree leaf: `DOC-INTENT-TAXONOMY.1`
- Date: `2026-06-22`
- Type: measurement, **read-only** (profiled the persisted `generated/evidence_ir/*` + `generated/intent_ir/*`
  JSONs; no `validate` run, so no artifact back-annotation; zero canonical mutation)
- Question: *what is the distribution of the 78 ingested docs across the 6-category purpose taxonomy
  (`DOC-INTENT-TAXONOMY.0`), and where is a typed-surface-count classifier (the basis of the existing 4-way
  `document_class`) too coarse to recover the purpose category?*

## Method

For each of the **78** persisted docs, read the typed-surface counts that discriminate purpose:
`actor_signal_relations` (rel), `signal_constraints` (sigc), `register_records` (reg), `message_field_records`
(msg), `signal_presence_records` (pres), `conditional_rules` (cond), and IntentIR `transactions` (txn). Two
labelings were computed and compared:

1. **Ground-truth purpose category** (1–6) — assigned per document from its known purpose (a measurement
   label, not shipped code; the runtime recognizer `.3` must be structural per ADR 0006).
2. **Structural-only guess** — a deterministic rule over the surface counts alone, *intentionally* unable to
   separate cat 2↔3, 4↔6, or 5↔6, so the collapse exposes exactly what surface counts cannot decide:
   `wire = rel+sigc+pres+txn`, `regstruct = reg+msg`; `regstruct≥40 → reg/struct(2|3)`; `wire≥20 ∧
   regstruct<20 → wire(1)`; `regstruct≥5 → reg-ish(2|3)`; `wire≥5 → wire(1)`; `cond≥10 → prose-only(4|6)`;
   else `near-empty(5|6)`.

Reproduce: the script lives in the `DOC-INTENT-TAXONOMY.1` commit message / can be re-run against the
persisted corpus; results below are verbatim.

## Result 1 — purpose-category distribution (78 docs)

| # | Category | Docs | Share |
|---|---|---:|---:|
| 1 | Wire-level bus / interconnect protocol | 36 | 46% |
| 2 | Programmable register / memory-mapped IP | 7 | 9% |
| 3 | Platform / system-IP topology & integration | 15 | 19% |
| 4 | CPU ISA / privileged architecture | 2 | 3% |
| 5 | Physical / electrical / link layer | 4 | 5% |
| 6 | Methodology / language / EDA standard / guide | 14 | 18% |

**Reading:** the ingested corpus is wire-protocol-dominant (46%) — which is why category 1 is the mature one.
Categories 2+3 (register/platform) together are **28%** and are the largest *buildable* expansion. Category 4
(ISA) is barely present (2 docs) — the corpus has no full ISA volumes yet. Categories 5+6 (**23%**) are the
honest non-targets.

## Result 2 — where surface counts collapse (why `document_class` is coarse)

The structural-only guess vs ground-truth (each cell = how many docs of that ground-truth category fell into
the structural bucket):

| Structural bucket | Ground-truth categories that land here |
|---|---|
| `wire(1)` | **cat1×19** · cat2×1 · cat3×1 · cat6×3 |
| `reg/struct(2\|3)` | cat1×8 · cat2×3 · **cat3×7** · cat4×1 |
| `reg-ish(2\|3)` | cat1×1 · cat2×1 · **cat3×5** |
| `prose-only(4\|6)` | cat1×7 · cat3×1 · cat4×1 · **cat6×3** |
| `near-empty(5\|6)` | cat1×1 · cat2×2 · cat3×1 · **cat5×4 · cat6×8** |

Three structural blind spots fall straight out:

- **Cat 2 ↔ cat 3 are NOT separable by surface counts.** Register/structure-dominant docs span register-IP
  (cat 2), platform/system-IP (cat 3), *and* register-heavy protocols (cat 1) and even ISA (cat 4). The
  `reg/struct` + `reg-ish` buckets hold cat3×12, cat2×4, cat1×9, cat4×1. The IP-vs-platform distinction is
  **semantic** (one IP's programming model vs a multi-component subsystem with topology), invisible to counts.
- **Cat 4 (ISA) has no distinct structural signature.** The two ISA docs split across two *different*
  buckets — RISC-V Advanced Interrupt Architecture reads as `prose-only(4|6)` (reg 0, cond 39), RISC-V Debug
  reads as `reg/struct(2|3)` (reg 44, rel 20, cond 78). This is the empirical confirmation of the taxonomy's
  claim that `document_class` has *no ISA slot* and that structure alone cannot mint one.
- **Cat 5 ↔ cat 6 are indistinguishable** (both `near-empty`): cat5×4 + cat6×8 share the low-structure bucket.
  Separating "honest non-target PHY" from "honest non-target guide" needs front-matter / title cues, not counts.

## Result 3 — the register-heavy-protocol trap (the deepest finding)

**8 cat-1 protocols are register/structure-dominant** — a naive "registers ⇒ register-IP" rule would
misclassify them as cat 2/3:

| Doc | reg | msg | rel | why it's still a protocol |
|---|---:|---:|---:|---|
| CCIX r1.0 / r1.0a / rev1.1 / rev2.0 | 131–143 | 45–92 | 0–2 | coherent-interconnect protocol whose spec is register/message-table-heavy |
| AXI (`ihi0022_l`) | 71 | 0 | 348 | the 71 "registers" are positionless encoding pseudo-tables; the 348 relations are the real intent |
| CHI arch (`ihi0050_g`) | 0 | 106 | 79 | coherent protocol; message fields are flit fields, not MMIO registers |
| DTI (`ihi0088`) | 0 | 159 | 1 | message-based protocol; the 159 fields are message fields (`.10f`) |
| CHI-C2C (`ihi0098_b`) | 81 | 210 | 0 | chip-to-chip coherent protocol; register+message-table-heavy |

The lesson for the `.3` recognizer: **the dominant typed surface is not the purpose.** A register/message
surface is necessary-but-not-sufficient evidence; the recognizer must also weigh the *presence and shape* of
the wire-relation/transaction surface and document-level cues (self-declared type, section vocabulary), never
collapse "has registers" to "is a register IP."

## Implications

- **For `.3` (fast category recognizer):** surface counts give a strong cat-1 signal (19/36 land cleanly in
  `wire(1)`) and a clean cat-5/6 *non-target* signal (low-structure), but they CANNOT separate 2/3, recover 4,
  or split 5/6. The recognizer needs, in addition to counts: (a) the wire-relation/transaction *shape* to
  rescue register-heavy protocols, (b) document front-matter / self-declared type (already partially read by
  `document_type_declared`, PDF-VARIANT-DIGESTION.5c) to separate guide/PHY/ISA, and (c) a structural
  topology/component cue to separate platform (cat 3) from single-IP register programming model (cat 2). All
  ADR-0006 structural — no vendor/name lists.
- **For `.2` (per-category ISF-completeness gauge):** the denominator is now known (36/7/15/2/4/14). The gauge
  should report completeness *within* each category against that category's own bar, and treat cat 5/6 as
  non-targets (a thin `.isf` there is correct, not a gap), matching the `DOC-INTENT-TAXONOMY.0` scorecard.
- **Genericity:** the labels here are a one-off measurement; the recognizer stays structural. The census is
  reproducible against the persisted corpus and carries no runtime name list.

## Honest caveats

- A handful of docs are genuinely borderline (e.g. ARM Debug Interface v6 as cat-3 system-IP vs cat-1 debug
  protocol; eMMC as cat-1 device protocol vs cat-2 register; OpenCAPI Data-Link-Layer as cat-1 vs cat-5).
  These were assigned by dominant purpose and are flagged here, not hidden — they are exactly the cases the
  `.3` recognizer must handle with a confidence/residual, never a forced guess.
- Two docs are under-extracted thin (`jesd235` HBM, with only 2 actors) — their category is assigned by title;
  their thin surface is a separate extraction-recall matter, not a category signal.
