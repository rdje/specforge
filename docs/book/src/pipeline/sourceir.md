# SourceIR

`SourceIR` is the document-preservation stage.

Its job is to retain enough faithful structure that later stages do not have to guess unnecessarily.

## What belongs in `SourceIR`

- document identity
- normalization outputs
- page structure
- structured tables
- visual assets
- section layout
- backend metadata

In practical terms, `SourceIR` is where the pipeline decides:

- what the source document is
- how it was normalized
- what tables, figures, and page assets exist
- where those assets came from in the original document

It is the closest thing the system has to a structured "compiled source document".

## Why this stage matters

If `SourceIR` is lossy, the downstream KG cannot recover what was lost reliably.

That is why earlier project investment went heavily into ingest and source preservation:

- table structure
- captions
- sections
- page assets
- figure identity

Those are deterministic, high-leverage wins.

This is why early project effort went so heavily into ingest and preservation rather than immediately chasing higher-level semantics.
If the source layer loses the table grid, the caption link, the section boundary, or the asset identity, later semantic stages can only guess.

## What a good `SourceIR` artifact gives later stages

A strong `SourceIR` should let later stages ask grounded questions like:

- which page did this table come from?
- what was the table header layout?
- which caption belongs to this figure?
- where in the section hierarchy did this statement appear?
- which visual asset was referenced here?

That is a much better foundation than forcing later stages to work from flattened text alone.

The dedicated [Multimodal Evidence And Visual Grounding](multimodal-evidence.md) chapter explains how preserved visual assets flow into visual evidence, VLM observations, and semantic grounding.

Operationally, the source bundle is now managed as a replaceable local cache rather than an append-only dump.
When the same PDF is ingested again, `specforge` stages the new normalization into `normalized.staging/` and swaps it into `normalized/` only after backend success.
That keeps stale page/image leftovers from earlier runs out of the current `SourceIR` evidence trail while preserving the last good bundle if the backend fails mid-rerun.

## What `SourceIR` is not

It is not the place where protocol semantics should be invented.

It should preserve the document faithfully and expose enough structure for later semantic lifting, but not pretend to know the final meaning already.

## Current maturity boundary

`SourceIR` is relatively mature in architecture, but not "done forever".

It is strong enough to be the foundation.
It is not yet assumed universal for every ugly real-world PDF.

Remaining work there is mostly:

- robustness hardening
- benchmarking on messy PDFs
- fallback behavior
- failure detection

not broad new concept invention.

## Typical `SourceIR` failure modes

The main risks at this stage are structural, not semantic.

Examples:

- OCR-heavy or scanned PDFs
- multi-column reading-order drift
- rotated or split tables
- weak caption-to-figure linkage
- unusual appendix layouts
- backend-dependent table-kind misclassification

When those failures happen, the right response is usually:

- preserve the failure honestly
- surface it clearly
- harden normalization or fallback behavior

not to pretend a later semantic stage can recover structure that was never preserved.

## The key rule of this stage

`SourceIR` should maximize faithful preservation and minimize premature interpretation.

That discipline is what allows the later stages to be ambitious without becoming reckless.

## Closed task trees — how each was implemented and verified

### `R6-SOURCE-HARDENING` — close zero-coverage assertion gaps on `SourceIR`

`SourceIR` is the entry point for every SpecForge pipeline; an
untested field on a source record is a regression risk for
document ingestion and adapter targeting. This tree added
regression-only test assertions to every populated-but-untested
field on `SourceIR`-related records. Verified by mutation
testing (cargo-mutants) reducing missed mutants to zero on the
targeted symbols + `scripts/run_ci.sh`. *Authoritative tracking:*
`docs/tasks/R6-SOURCE-HARDENING.md`.

### `REGISTER-MAP-CLASSIFIER-PRECISION` — only real register maps become registers

**What it gives you:** when SpecForge ingests a PDF, a table is
only tagged a *register map* if it actually looks like one — so
your register inventory isn't polluted with tables that merely
*mention* an address.

**Why it mattered.** Running SpecForge across a real
chip-doc corpus showed the table classifier was too eager: any
table with an "address"/"offset" header became a `register_map`.
That swept in *address-assignment tables* (e.g. I2C's reserved
slave-address list), *feature matrices*, *tables of contents*,
and *data-frame layouts* (e.g. eMMC's RPMB packet, whose 512-bit
frame carries `[511:316]` fields that look just like register
bits). The result was dozens of phantom "registers" — and a
completeness check even flagged one for self-contradictory bit
fields, which is how the bug surfaced.

The fix classifies a `register_map` only when the table shows
genuine register *structure* — a real bit range (`7:0`, `[31:16]`,
not a stray `[177]` page reference) or a standalone access token
(`RO`/`RW`/`W1C`/…) — **and** is not a table of contents
(dotted-leader rows) or a data-frame layout (frame column
vocabulary). Verified end-to-end on the corpus: the phantom
registers disappear (eMMC dropped from 18 register tables to 2,
register records 48 → 14) while genuine register/field tables are
kept, with no change to clean specs. *Authoritative tracking:*
`docs/tasks/REGISTER-MAP-CLASSIFIER-PRECISION.md`.

### `REGISTER-CLASSIFIER-ENCODING-FP` — encoding cross-reference tables aren't registers

**What it gives you:** field-*encoding* tables — which map bit
*positions* to per-channel meanings — are now typed as `encoding`,
their true kind, instead of leaking into your register inventory as
phantom registers.

**Why it mattered.** Continuing the corpus run onto the CHI
architecture spec, the same completeness check that caught the
eMMC bug flagged a register with self-contradictory bit fields —
again the symptom of a misclassification. The culprit was CHI's DVM
"field encodings" tables (e.g. *"Table B8.10: Security field
encodings for each DVMType"*): their first column lists bare bit
positions (`2:0`, `3`, `4`, …) and a header reads `X in
REQ.Addr[x]`. That header carries the substring "addr" and the body
carries a `2:0` bit range, so the table tripped the register gate —
yet it is an encoding cross-reference, not a register. The result
was 8 phantom registers named after bit ranges (`2:0`, `13:11`, …),
four of them claiming the same bit.

The fix recognizes an encoding table *before* the register gate, by
two complementary signals: a caption that says "encoding(s)" (author
intent — catches the captioned table) and the `X in …` cross-reference
header idiom (catches caption-less continuation pages). Verified by
re-classifying every table in the corpus (1,989 tables): the two CHI
tables flip `register_map → encoding` (all 8 phantom registers gone),
the 7 genuine register maps and all 210 signal tables are untouched,
and 32 further tables whose captions literally name an encoding move
from `unknown`/`feature_matrix`/`timing_parameter` to the correct
`encoding` — a net precision gain with no regression. *Authoritative
tracking:* `docs/tasks/REGISTER-CLASSIFIER-ENCODING-FP.md`.

### `DOCLING-DEVICE-CPU-DEFAULT` — ingest picks a working compute device

**What it gives you:** PDF ingest works out of the box, including on Apple
Silicon Macs, without per-command environment tweaks.

**Why it mattered.** Docling runs its layout and table models on a compute
device it chooses automatically. On Apple Silicon that auto-choice is the Metal
(MPS) backend — but current PyTorch can't run the float64 math these models need
on MPS, so *every page* of a conversion fails (`Cannot convert a MPS Tensor to
float64 dtype`). The result is a total ingest failure on a perfectly good machine.

The fix makes SpecForge's Docling helper choose the device deliberately instead of
deferring to the broken auto-pick: it uses an explicit `DOCLING_DEVICE` if you set
one, otherwise CUDA when a GPU is present, otherwise CPU — never the MPS auto-path.
GPU machines keep their acceleration; Apple Silicon falls back to correct
(if slower) CPU; and you can still force `DOCLING_DEVICE=mps` on a PyTorch build
that supports it. A defensive fallback keeps ingest running even if a future
Docling version reshapes these options. Verified end-to-end: a fresh ingest with
no environment variables converts cleanly where it previously failed on page one.
*Authoritative tracking:* `docs/tasks/DOCLING-DEVICE-CPU-DEFAULT.md`.
