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

## Path identity and repository moves

`SourceIR` distinguishes the source's ownership from its runtime location. Repository-owned source paths,
artifact layouts, normalized Markdown, page metadata/images, visual assets, and caption sources are written as
repository-relative JSON strings. An explicitly authorized PDF or Markdown input outside the repository keeps
its exact absolute path and carries `path_origin: "external_input"`; a source inside the repository carries
`"repository_owned"`.

When `SourceIR` is built or loaded, those stored values are resolved to absolute paths under the current
repository for normal I/O. That means moving the repository does not require every downstream caller to learn a
new path API, and new artifacts do not remember the old workstation or mount point. Older unlabeled absolute
repository paths can load only when their recognized project-data suffix identifies exactly one existing target
under the current root. An unrelated external absolute path is never silently rebased.

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

## Bounded-memory ingestion of very large PDFs

Chip-spec PDFs get big — hundreds to thousands of pages. SpecForge ingests them with **bounded
memory** so a large document can never exhaust the host's RAM and crash it. The principle is
simple and strict: **a bigger file may take longer, but it never lowers quality and never risks
the host.**

How it works: for documents above a page threshold, the Docling backend converts the PDF in
**bounded page ranges** rather than all at once. Each range is rendered, its page images and
structured records are written to disk, and that range's heavy data is freed before the next —
so peak memory stays proportional to the *batch size*, not the *page count*. Documents at or
below the threshold use the original single-pass conversion unchanged.

Two environment variables tune this (defaults are chosen so every normal document keeps the
single-pass path):

- `SPECFORGE_INGEST_BATCH_THRESHOLD` — page count above which batching activates (default `512`).
- `SPECFORGE_INGEST_BATCH_PAGES` — the **ceiling** on pages per batch when batching (default `64`).
  Smaller batches use less peak memory and run slower; adaptive sizing (see *Sizing the batch to the
  host* below) may lower the actual batch beneath this on a small machine, and you can lower the
  ceiling yourself on a constrained host.

What does *not* change with batching: the document profile, structured tables, content elements,
sections, and figure/table images are the same complete, full-resolution capture you would get
from a single pass. The only observable difference is that a running page-header that Docling
happens to merge across a page boundary in single-pass mode may appear as two separate boilerplate
elements — never a loss of signals, tables, or intent.

## Sizing the batch to the host

The batch size above has a sensible default (64 pages) tuned for a typical workstation — but the
*right* batch for a 4 GB container is not the right batch for a 32 GB server. A batch that is too
large for a small machine would push it past the memory safeguard's ceiling and the ingest would be
aborted every time, no matter how patient you are. So SpecForge **sizes each batch to the machine it
is running on**, automatically.

The signal it uses is the host's **total physical RAM** — deliberately *not* the momentary free
memory. Total RAM is a fixed property of the machine, so the chosen batch size is the same on every
run: the *same machine always ingests the same way*, which keeps results reproducible. (Reacting to
moment-to-moment free memory would make the batching — and so the exact output at batch boundaries —
wobble from run to run; transient pressure from other programs is instead handled by the memory
safeguard above.)

The sizing is a simple, conservative ladder, chosen to keep a batch's peak memory near a safe
fraction of RAM:

- **≥ 16 GB** — the full ceiling (64 by default). Every normal workstation and server lands here, so
  its ingest is **byte-for-byte identical** to before.
- **8–16 GB** — at most 32 pages per batch.
- **4–8 GB** — at most 16 pages per batch.
- **< 4 GB** — a floor of 8 pages per batch.

On a small machine the ingest therefore *completes* — slower, in more and smaller batches — where a
fixed large batch would have been aborted. This is the project's standing rule made concrete:
**speed flexes, quality does not.** Every page is still converted at full fidelity; only the working
batch shrinks. (As with single-pass vs. batched, a smaller batch can place a benign boilerplate
boundary slightly differently — never a loss of signals, tables, or intent. A machine large enough
for the full batch has no such difference at all.)

One environment variable controls it:

- `SPECFORGE_INGEST_ADAPTIVE_BATCH` — on by default; set it to `off` (or `none`/`disabled`) to force
  the fixed `SPECFORGE_INGEST_BATCH_PAGES` ceiling regardless of host RAM (the original behavior).

If SpecForge cannot read the host's total RAM for any reason, it keeps the ceiling — never making a
run *worse* on missing information.

## Bounded disk footprint of very large PDFs

Memory is not the only resource a giant PDF can exhaust — disk is the other. The single largest
thing the normalized bundle writes is a **full-resolution image of every page**: one PNG per page.
On a thousand-page manual that is tens of gigabytes of page rasters alone — even though nothing
downstream ever reads them. SpecForge's vision steps (`enrich`, `audit-extraction`,
`recover-register-bits`) only ever read the *figure and table region crops*, never the full-page
rasters.

So for large documents SpecForge keeps each page image only long enough to crop those region
images out of it, then **does not persist the page raster to disk**. The effect:

- ingest disk grows with the number of *figures and tables* (`O(assets)`), not the number of
  *pages* (`O(pages)`) — a thousand-page PDF no longer writes a thousand page PNGs;
- every figure/table region image is still captured at **full resolution** — nothing the pipeline
  reads is lost;
- the page artifact still records the page's full-resolution dimensions, so the raster stays
  well-defined and can be re-rendered on demand if a future consumer ever needs it; the
  `page_image_path` is simply reported as absent rather than pointing at a file.

As with bounded memory, this is **quality-invariant** — only the on-disk working set shrinks. By
default page rasters are persisted for normal documents (so small documents stay byte-for-byte
unchanged) and skipped for large ones, using the same threshold that triggers batching. One
environment variable overrides the decision explicitly:

- `SPECFORGE_INGEST_SAVE_PAGE_IMAGES` — `1` to always persist per-page rasters (even for large
  documents), `0` to never persist them (even for small ones). Unset, the default is "persist at or
  below `SPECFORGE_INGEST_BATCH_THRESHOLD`, skip above it."

## How large is the `source_ir.json` itself?

The page rasters were the *heavy* artifact; the `source_ir.json` that describes the document — its
typed text elements, tables, sections, and page records — is comparatively tiny. It does grow with
the document, though, so it is worth being honest about how much. Measured across the whole reference
library, the file scales **linearly at roughly 9 KB per page** (mostly the per-page list of typed
content elements). In practice that means:

- the largest real chip spec in the library — an 842-page technical reference manual — produces a
  `source_ir.json` of about **10 MB**; a 930-page architecture spec, about 7 MB;
- a hypothetical 2,000-page manual would be on the order of ~18 MB — still trivial to hold and to
  re-read.

So for **every realistic specification** the typed artifact is comfortably bounded, and there is no
need to stream or chunk its assembly. The honest caveat is at the far extreme: the downstream stages
(`evidence`, `semantic`, `intent`) read the whole `source_ir.json` into memory at once, so a document
of *tens of thousands* of pages would eventually make that load itself large. No chip-spec PDF comes
close to that, so SpecForge tracks it as a known, deferred boundary rather than pre-engineering for a
size that does not occur — the same "build it when a real document needs it" discipline used for the
on-demand page-image path above.

## Autonomous host-memory safeguard

Bounded batching and bounded disk make ingestion *predictable* — but a host can still be under
memory pressure for reasons that have nothing to do with SpecForge (other applications, a model
loaded in the background). To guarantee that ingesting a document **never crashes the machine**,
SpecForge watches the host's own memory while the Docling step runs and stops *itself* before the
system reaches a danger level — no external babysitting required.

How it works, in plain terms:

- before launching the heavy conversion, and then at a steady interval while it runs, SpecForge
  reads the system's used-memory percentage using the platform's own tool (`memory_pressure` on
  macOS, `/proc/meminfo` on Linux) — the same number you would watch in a system monitor;
- if used memory reaches the safety ceiling, SpecForge **cleanly aborts the ingest**: it stops the
  Docling subprocess and reports a clear, typed error explaining what happened and how to proceed;
- the host is preserved, and so is your data — because the new bundle is built in a staging area and
  only swapped in on success, an abort leaves the **previous good `normalized/` bundle and
  `source_ir.json` completely intact**. You lose only the unfinished run, never prior work.

The default ceiling is **85% used** — deliberately below the point where a desktop host starts to
thrash and risks a reboot. Two environment variables tune the safeguard:

- `SPECFORGE_INGEST_RAM_ABORT_PERCENT` — the used-memory percentage at which to abort (default
  `85`). Set it to `off` (or `none`/`disabled`) to turn the guard off entirely; values outside the
  `0–100` range also disable it.
- `SPECFORGE_INGEST_RAM_SAMPLE_SECS` — how often to re-check memory while ingesting (default `2`
  seconds, minimum `1`).

This is a *safety* feature, not a quality trade-off: a healthy ingest on a host with headroom runs
exactly as before. The guard only ever acts when continuing would put the whole machine at risk —
and when it does, it fails honestly and reversibly rather than taking the host down with it.

## Pre-flight disk check

The memory guard above protects the host *while* ingesting. SpecForge also looks ahead *before*
ingesting: a quick disk pre-flight that refuses to start a run the filesystem cannot finish, instead
of filling the disk partway through. It runs at the very start of materialization — **before any
staging directory is created** — so a refusal touches nothing on disk and any previous normalized
bundle is left completely intact.

How much disk does an ingest need? Precisely, that depends on how many figures and tables the
document has, which isn't known until after it has been read. Rather than guess at a number it
can't compute, SpecForge scales the requirement off the one cheap signal it *does* have up front —
the **source PDF's file size** — with generous headroom: roughly `128 MB + (4 × the source size)`.
It then reads the free space on the target filesystem (using the system's own `df`, no extra
dependency) and, if there isn't enough, stops with a clear typed error naming the free space, the
estimated need, and how to proceed. If it can't read the free space for any reason, it stays out of
the way and lets the ingest proceed — it never refuses a run on missing information.

One environment variable tunes it:

- `SPECFORGE_INGEST_MIN_FREE_DISK_MB` — a number sets an explicit fixed free-MB floor (overriding
  the size-based estimate); `off` (or `none`/`disabled`/`0`) turns the pre-flight off entirely.
  Unset, it uses the size-based estimate.

(The RAM half of "check resources before launching" is already covered by the memory guard's
pre-spawn sample described above — it won't even start a heavy ingest on a host that is already
over the memory ceiling.)

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
