# MEMORY-BOUNDED-INGEST: bounded-memory ingestion of very large PDFs

## Metadata

- Tree ID: `MEMORY-BOUNDED-INGEST`
- Status: `active`
- Roadmap lane: `R2`/`R8` (SourceIR ingestion / Tier-1 capture robustness)
- Created: `2026-06-14`
- Last updated: `2026-06-14`
- Owner: repo-local workflow

## Goal

Make `specforge ingest` effectively **immune to chip-spec PDF file size** — 10 MB, 100 MB,
300 MB, 1 GB, 3 GB and beyond — bounded in **both RAM and DISK**, so no document can exhaust a
host's resources and crash/reboot it, and SPECFORGE runs in the **most restricted RAM/disk
environments**. **Quality is invariant — only SPEED flexes.** Under tight resources SPECFORGE runs
*slower* (smaller batches, incremental disk staging, aggressive freeing) but its SOTA-quality
outcome stays **fully intact**: never lower fidelity, never drop or coarsen captured intent. Owner
directive (`2026-06-14`, reinforced repeatedly, escalated to full size-immunity, then clarified:
"speed might be impacted but the level of top, SOTA quality shall remain untouched"). Hosts have
limited resources no matter how large they look; the 24 GB dev host crashes/reboots in the 90→93%
used-RAM danger zone (`[[feedback_ram_ceiling_monitor]]`, `[[project_big_pdf_memory_bounded_ingest]]`).

The immediate corpus need is docs up to 930 pages (GIC-600 930, CoreSight SoC-600 842, SMMU-700
717, Cortex-A76 620, CHI 585, USB-3.2 548); the standing goal is bounded resources at **any**
size. The two dominant unbounded costs are (a) **RAM** — holding a full-res image for every page
during conversion (peak ∝ page count) — addressed by `.1`; and (b) **DISK** — the `normalized/`
bundle stores a full-res PNG per page, so a multi-thousand-page PDF can write tens of GB —
addressed by the disk-bounding leaves below.

## Non-Goals

- Not changing the SourceIR schema or the downstream EvidenceIR→…→adapter pipeline.
- **NOT trading quality for resources.** Fidelity and captured-intent completeness are invariant;
  the ONLY thing that may flex under tight resources is SPEED (smaller batches, more disk staging).
  Lossy levers — lower image resolution, compression that drops detail, sampling/skipping pages or
  tables — are OUT. Disk-bounding (`.3`) bounds the *working set* (incremental + free), not the
  captured fidelity; full-res figure/table region images and all typed surfaces stay intact.
- Not re-ingesting or perturbing existing gold/intact docs — the batched path activates only above
  a threshold set above every doc we already ingest single-pass (max 500p), so they stay
  byte-identical.
- Not optimizing for raw speed — the aim is bounded RAM/disk at any file size with quality intact;
  slower-under-restriction is acceptable, lower-quality is not.

## Acceptance Criteria

- A big PDF (CHI, 585p) ingests to a complete `SourceIR` + `normalized/` bundle without the
  Docling backend being memory-killed, with peak system RAM held safely below the danger zone.
- Small/medium docs (≤ threshold) keep the **exact current single-pass path** and re-ingest
  byte-identical (proven on a representative gold doc).
- Stays PDF-agnostic (ADR 0006) and preserves the staged-swap (a failed run never destroys the
  last good `normalized/`; `source_ir.json` survives).
- Focused checks + full `scripts/run_ci.sh` green; CHI verification run under the autonomous
  RAM guard; live docs + book updated; committed per `COMMIT.md`.

## Task Tree

- ID: `MEMORY-BOUNDED-INGEST`
  Status: `active`
  Goal: resource-bounded ingestion immune to PDF file size (RAM + DISK)
  Children: `.1`, `.2`, `.3`, `.4`, `.5`

- ID: `MEMORY-BOUNDED-INGEST.1`
  Status: `done`
  Goal: implement page-range batched conversion in the Docling backend helper (+ Rust config),
  gated above a page threshold; extract the per-doc element processing into one shared routine
  so single-pass and batched modes produce the same record shapes.
  Acceptance: code lands behind a page-count gate; small docs unchanged; `run_ci.sh` green; the
  helper frees each batch (`del`+`gc.collect()`) so peak memory is O(batch), not O(page count).
  Verification: `done (2026-06-14)` — extracted `_IngestAccumulator` + `process_converted_document`
  (the two per-doc loops moved verbatim, accumulators externalized); `detect_pdf_page_count`
  (pypdfium2, cheap) + `_env_int`; `main` dispatches single-pass (`convert(path)`, unchanged) when
  `page_count <= SPECFORGE_INGEST_BATCH_THRESHOLD` (default 512) else page-range batches of
  `SPECFORGE_INGEST_BATCH_PAGES` (default 64), freeing each batch. Python syntax py_compiled; Rust
  builds; full `run_ci.sh` GREEN (1587 tests). On a temp 14-page doc (no gold touched): **(A)
  single-pass before-vs-after-refactor `source_ir.json` BYTE-IDENTICAL**; **(B)** forced batched
  (threshold=4, batch=4 → 4 batches) vs single-pass: `page_no` ABSOLUTE across batches (1–14, zero
  duplicate page/table/element ids), tables/visual/sections counts equal, table (id,caption) sets
  equal — the only delta is one boilerplate running-header (`"I 2 S bus specification"`) that
  single-pass merges across one page boundary and batched keeps as two (benign boundary artifact,
  no content/signal/table loss). Commit: `MEMORY-BOUNDED-INGEST.1`.
  Commit: `MEMORY-BOUNDED-INGEST.1`

- ID: `MEMORY-BOUNDED-INGEST.2`
  Status: `done`
  Goal: verify on CHI (585p) end-to-end under the autonomous RAM guard — ingest completes with
  bounded peak RAM, `normalized/` + `source_ir.json` materialize; re-prove small-doc
  byte-identity on a gold doc; then unblock `PDF-VARIANT-DIGESTION.13c`.
  Acceptance: CHI ingest succeeds under the ≥85%-used kill ceiling; a gold doc re-ingests
  byte-identical; measurements recorded.
  Verification: `done (2026-06-14)` — CHI (`ihi0050_g`, 585p) ingested via the batched path
  (10 batches × 64p, `DOCLING_DEVICE=cpu`) under an autonomous RAM guard (2s sample, kill at
  ≥84% used): **completed rc=0, NOT killed, PEAK memory_pressure used = 20%** (vs the
  single-pass OOM at 17.2 GB RSS / SIGKILL); swap unchanged; RAM 83% free after. source_ir
  complete + faithful — profile 585p / 368 tables / 122 figures / 7799 elements / 1202 sections
  (matches the prior single-pass profile), 585 page artifacts with ABSOLUTE unique page numbers
  1–585 (no cross-batch collisions), backend raw = batched envelope, 585 page PNGs on disk,
  normalized 528 MB. ~19 min wall (multi-pass, slower — quality intact). Small-doc byte-identity
  was already proven in `.1` (temp-14p before/after). `PDF-VARIANT-DIGESTION.13c` UNBLOCKED.
  Commit: `MEMORY-BOUNDED-INGEST.2`

- ID: `MEMORY-BOUNDED-INGEST.3`
  Status: `done`
  Goal: bound the **DISK** footprint of the `normalized/` bundle for very large PDFs **without any
  fidelity loss** (the dominant cost is the full-res PNG per page — a multi-thousand-page doc can
  write tens of GB). **Codebase investigation (read-only, recorded in Decisions) settled the
  mechanism precisely:** the per-page full-res PNG (`normalized/pages/page-NNNN.png`,
  `PageArtifact.page_image_path`) is read by **NO** downstream consumer — `enrich`,
  `audit-extraction`, and `recover-register-bits` all read only figure/table REGION images
  (`VisualAsset.image_path`); the IR stages and `validate` only COUNT page artifacts. BUT the
  Docling helper crops those region images from the in-memory page image (`element.get_image(doc)`),
  so the page image must still be **generated**. Quality-preserving lever: keep
  `generate_page_images=True` (region cropping unaffected, region images stay full-res on disk) but
  **skip PERSISTING the per-page PNG to disk for large docs** — so ingest disk is O(assets), not
  O(pages), with zero fidelity loss; `page_image_path` becomes an honest `None` while the page's
  full-res dimensions stay recorded. Gated by the same large-doc condition as batching
  (default: skip when `pages > SPECFORGE_INGEST_BATCH_THRESHOLD`), explicitly overridable via
  `SPECFORGE_INGEST_SAVE_PAGE_IMAGES=1/0`. Small docs keep the EXACT historical bundle
  (byte-identical). NO lossy levers (no lower resolution / detail-dropping compression / page or
  table skipping). PDF-agnostic.
  Acceptance: a large-page-count doc ingests with no per-page PNGs on disk (disk O(assets)) while
  every figure/table region image stays full-res and byte-identical to the saved-mode run; small
  docs (≤ threshold) re-ingest byte-identical; explicit override works both ways; `run_ci.sh` green.
  Verification: `done (2026-06-14)` — implemented in the Docling helper as a generate-in-memory /
  skip-persist gate (`_env_flag` + `save_page_images` passed into `process_converted_document`;
  `page.image.pil_image.save()` and the recorded `page_image_path`/`rendered_image.path` are skipped
  when `save_page_images` is false; `width_px`/`height_px`/`dpi` still recorded). py_compile OK; full
  `scripts/run_ci.sh` GREEN (lib 1587, clippy/rustdoc warning-denied, mdBook, memory-arch, KM sync).
  Live-verified on a throwaway CAN copy (72p, key `diskbound_probe_can`, never touched gold), CPU
  device, RAM ≤15% used: **(A)** default → 72 page PNGs, all `page_image_path` set, AND **byte-identical
  to the same ingest with the change git-stashed (old code)** — small-doc path unchanged; **(B)**
  `SPECFORGE_INGEST_SAVE_PAGE_IMAGES=0` → 0 page PNGs, 98 region crops byte-identical to (A),
  `source_ir.json` identical to (A) once `page_image_path` is neutralized (only field that changes);
  **(C)** `SPECFORGE_INGEST_BATCH_THRESHOLD=1` (force large) default → AUTO-skip (0 PNGs) + batched,
  region crops intact; **(D)** large + `SAVE=1` → override KEEP (72 PNGs). Disk: per-page PNGs were
  21 MB vs 1.8 MB region crops on CAN — skip drops the O(pages) cost, scaling on big docs. Throwaway
  bundle removed after measurement.
  Commit: `MEMORY-BOUNDED-INGEST.3`

- ID: `MEMORY-BOUNDED-INGEST.3b`
  Status: `proposed`
  Goal: (follow-up, build only if a real consumer needs it) provide a targeted **on-demand
  single-page full-res render** path so a specific page image can be reproduced without a full
  re-ingest, satisfying "any page image later produced is byte-for-byte full-res" with surgical
  cost. Tracked honestly because `.3` stops persisting page images; today NO consumer reads page
  images (only region images), so this is deferred (YAGNI) rather than built speculatively. Until
  then, a page image can still be reproduced full-res by re-ingesting with
  `SPECFORGE_INGEST_SAVE_PAGE_IMAGES=1`.
  Acceptance: a `render-page`-style path emits the full-res PNG for one requested page, byte-for-byte
  identical to the eager-ingest image; bounded RAM/disk.
  Verification: `pending`
  Commit: `pending`

- ID: `MEMORY-BOUNDED-INGEST.4`
  Status: `proposed`
  Goal: complete at **full quality even in the most restricted** RAM/disk environments by trading
  SPEED, not quality — a pre-flight resource check, adaptive batch size (shrink the batch / spill
  more to disk under pressure → slower but identical output), and the autonomous RAM guard as a
  first-class ingestion safeguard. A clear typed error is the ABSOLUTE last resort only when the
  task is genuinely impossible (e.g., disk physically full) — never a quality compromise, never a
  silent partial result.
  Acceptance: ingestion never OOM-crashes the host; under restriction it completes with the SAME
  output (just slower / smaller batches), or — only if truly impossible — fails with a typed
  diagnostic and an intact prior `normalized/` (staged-swap).
  Children: `.4a`, `.4b`, `.4c`
  Verification: `pending`
  Commit: `pending`

- ID: `MEMORY-BOUNDED-INGEST.4a`
  Status: `proposed` (NEXT — designed `2026-06-14`, ready to implement in a fresh session)
  Goal: make the autonomous RAM guard a **first-class, built-in** ingest safeguard (today it is an
  external shell wrapper I have to remember to apply). While the Docling subprocess runs, `specforge`
  itself samples system memory and aborts the ingest CLEANLY before the host crosses a configurable
  danger ceiling — directly encoding the owner's non-negotiable "kill at ≥85% used; never crash the
  host" rule into the tool.
  Design (decided; no code written yet — working tree is clean):
  - **No new dependency** (crate stays at 4 deps): read memory via the platform's OWN tool, matching
    the exact metric the owner monitors. macOS → run `memory_pressure`, parse "free percentage", used
    = 100 − free. Linux → read `/proc/meminfo`, used% = (1 − MemAvailable/MemTotal)×100. Other OS →
    unreadable → guard inert (warn once). All parsers are PURE fns (unit-tested on sample text).
  - **Mechanism** in `docling_backend::materialize_pdf`: replace the blocking `command.output()` with
    spawn + a poll loop — `child.try_wait()` + sample memory every `SPECFORGE_INGEST_RAM_SAMPLE_SECS`
    (default 2) — redirecting child stdout/stderr to temp FILES (avoids pipe-fill deadlock while
    polling). Sample ONCE before spawn too (don't even launch a heavy ingest if already in danger).
    On breach: `child.kill()`, then `cleanup_path_if_exists(staged_normalized_root)` (the staged-swap
    means the last-good `normalized/` + `source_ir.json` are untouched) and return a typed error.
  - **Config**: `SPECFORGE_INGEST_RAM_ABORT_PERCENT` (default **85** = owner policy; `off`/`none`/
    `>=100`/`<=0` → disabled; garbage → default), `SPECFORGE_INGEST_RAM_SAMPLE_SECS` (default 2, min 1).
  - **New** `AppError::IngestAbortedForMemory { program, used_percent, ceiling_percent }` with an
    actionable Display (host preserved; free memory / raise the ceiling / set it off). Add the Display
    arm (only `impl Display` matches AppError exhaustively; `main.rs` just prints it).
  - **Testability (host-safe)**: factor `run_backend_with_ram_guard(command, &guard, stdout_path,
    stderr_path, used_percent_fn)` with the reader as a generic `Fn() -> Option<f64>` (DI). Tests:
    pure parsers + `should_abort_for_memory(used, ceiling)=used>=ceiling` + `RamGuardConfig::from_env`;
    pre-spawn abort (reader=99 + `sleep 30` child → returns fast, never spawns); mid-run kill (reader
    10-then-99 via a `Cell` + `sleep 30` child → killed quickly); completes (reader=10 + `true`). Tests
    build `RamGuardConfig` directly with a millisecond sample so they run fast. NO real memory pressure.
  - **CRITICAL determinism fix**: the existing source.rs stub-helper ingest tests
    (`pdf_source_ir_materialization_uses_backend_helper_and_writes_manifests`,
    `..._replaces_stale_normalized_artifacts`, `..._failed_materialization_keeps_existing...`) now run
    through the guard with the REAL reader at default 85% — a hot CI machine (>85% used) could
    false-abort them. They MUST set `SPECFORGE_INGEST_RAM_ABORT_PERCENT=off` via the existing
    `EnvVarGuard` pattern (under `env_var_lock`) so behavior is identical to before (assertions unchanged).
  Acceptance: ingest aborts cleanly with the typed error + an intact prior `normalized/` when memory
  would cross the ceiling (verified via DI, no real pressure); a normal small-doc ingest stays
  byte-identical (`source_ir.json` unaffected — only stdout/stderr move to files; summary unchanged);
  stub-helper tests deterministic; full `run_ci.sh` green; book + KM updated.
  Verification: `pending`
  Commit: `pending`

- ID: `MEMORY-BOUNDED-INGEST.4b`
  Status: `proposed`
  Goal: richer PRE-FLIGHT resource check — estimate the ingest's RAM/disk need from the cheap page
  count and check available disk + RAM before launching; fail fast with an actionable typed
  diagnostic (and an intact prior bundle) rather than starting work that cannot finish.
  Verification: `pending`
  Commit: `pending`

- ID: `MEMORY-BOUNDED-INGEST.4c`
  Status: `proposed`
  Goal: adaptive batch sizing under SUSTAINED memory pressure — shrink the batch / spill more to disk
  so a constrained host still completes with byte-identical output, just slower (speed flexes, quality
  invariant).
  Verification: `pending`
  Commit: `pending`

- ID: `MEMORY-BOUNDED-INGEST.5`
  Status: `proposed`
  Goal: bound the in-RAM + on-disk size of the accumulated summary / `source_ir.json` itself for
  extreme page counts (the lightweight records still grow with page count). Evaluate streaming /
  chunked summary assembly so even a 10000-page doc holds bounded summary state.
  Acceptance: summary assembly memory + `source_ir.json` size stay bounded at extreme page counts.
  Verification: `pending`
  Commit: `pending`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `MEMORY-BOUNDED-INGEST.4a` | `proposed` | **NEXT — fully designed, ready to implement.** Built-in autonomous RAM guard (owner's non-negotiable "never crash the host"); design captured under the `.4a` node + Decisions |
| 2 | `MEMORY-BOUNDED-INGEST.4b` | `proposed` | richer pre-flight RAM/disk check (fail fast before launching) |
| 3 | `MEMORY-BOUNDED-INGEST.4c` | `proposed` | adaptive batch sizing under sustained pressure (slower, identical output) |
| 4 | `MEMORY-BOUNDED-INGEST.5` | `proposed` | bound summary / `source_ir.json` + O(pages) per-page JSONs at extreme page counts |
| — | `MEMORY-BOUNDED-INGEST.3b` | `proposed` | targeted on-demand single-page render (deferred/YAGNI — no consumer reads page images today) |

`.3` (DISK-footprint bounding — skip persisting per-page PNGs for large docs) `done` 2026-06-14;
RAM dimension (`.1`/`.2`) + DISK dimension (`.3`) now both delivered.

`.1` (RAM page-range batching) `done` 2026-06-14; `.2` (CHI 585p proof, peak 20% used) `done`
2026-06-14 — `PDF-VARIANT-DIGESTION.13c` is now unblocked.

## Decisions

- `2026-06-14`: **Root cause** — the embedded Docling helper
  (`crates/specforge/src/ir/source/docling_backend.rs`) calls
  `converter.convert(str(input_path))` on the whole PDF with `images_scale = 2.0` +
  `generate_page_images = True`, so a full-res image for every page is held in `doc.pages`
  simultaneously. Peak memory ∝ page count → unbounded; CHI (585p) is killed at conversion.
- `2026-06-14`: **Mechanism** — Docling 2.84's `DocumentConverter.convert(source, …,
  page_range=(lo, hi))` (1-based inclusive; default `(1, MAXINT)`) lets us convert bounded page
  ranges. Internal `settings.perf.page_batch_size = 4` is model-inference batching only and does
  not bound the held document.
- `2026-06-14`: **Threshold design** — batch only when `page_count > SPECFORGE_INGEST_BATCH_THRESHOLD`
  (default **512**, above the 500p max of every currently-ingested doc), so all existing gold /
  intact docs keep the EXACT single-pass call and stay byte-identical. Batch size
  `SPECFORGE_INGEST_BATCH_PAGES` (default **64**, a multiple of Docling's internal 4). Page count
  is detected cheaply before conversion (pypdfium2 / Docling backend page count — verify).
- `2026-06-14`: **Merge** — the `DoclingBackendSummary` manifest (page_artifacts /
  structured_tables / content_elements / document_sections / profile) is the SourceIR-bearing
  product and holds only lightweight records, so it accumulates across batches with continued
  counters (picture/table/reading-order/section). The raw `export_to_dict()` is only a
  provenance *path* (`caption_source_path`), and markdown is a lossy convenience view — both can
  be merged/concatenated per batch without fidelity risk. Page images are saved to disk per batch
  then freed.
- `2026-06-14`: **Safety** — every verification re-ingest runs under the autonomous RAM guard
  (kill at ≥85% used; below the 90% danger floor), one heavy job at a time, model unloaded during
  ingests (`[[feedback_ram_ceiling_monitor]]`).
- `2026-06-14` (`.3`): **Page-image consumer investigation (read-only)** — exhaustive search of
  `crates/specforge/src` found `PageArtifact.page_image_path` is read by **no** downstream consumer.
  Every VLM-backed command reads only figure/table REGION images via `VisualAsset.image_path`:
  `enrich` (`commands/enrich.rs` — region crops only), `audit-extraction`
  (`commands/audit_extraction.rs` — table-region lookup), `recover-register-bits`
  (`commands/recover_register_bits.rs` — `DiagramKind::RegisterBitfield` region assets). The IR
  stages (`evidence`/`semantic`/`intent`) and `validate`/`converge` only COUNT `page_artifacts`,
  never open the images. So skipping page-image PERSISTENCE is a pure, zero-fidelity-loss disk win.
- `2026-06-14` (`.3`): **Mechanism — generate-in-memory, skip-persist (NOT disable generation).**
  The Docling helper crops region images from the in-memory page image
  (`PictureItem/TableItem.get_image(doc)` in `docling_backend.rs`), so `generate_page_images=True`
  must stay (disabling it would break region cropping and LOSE load-bearing region images — a
  quality regression). The disk cost is the explicit `page.image.pil_image.save(...)` write only.
  `.3` therefore keeps generation and skips the `.save()` for large docs, leaving region images
  full-res and intact while dropping the O(pages) PNG disk cost. `page_image_path`/
  `rendered_image.path` become honest `None`; the page's full-res `width_px`/`height_px`/`dpi` stay
  recorded (so on-demand regeneration in `.3b` is well-defined). No Rust struct change: both path
  fields are already `Option<PathBuf>`.

## Open Questions

- Does Docling report absolute page numbers (`page.page_no`) within a `page_range` batch, or
  range-relative? (Must be absolute, else page IDs collide across batches — verify in `.1`.)
- Is `convert(path)` byte-identical to `convert(path, page_range=(1, page_count))`? (Not relied
  upon — small docs keep the no-page_range call — but informs whether the gate could be removed
  later.)

## Blockers

- None. (`PDF-VARIANT-DIGESTION.13c` is blocked ON this tree; `.13d` likewise for its biggest docs.)

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-14` | `MEMORY-BOUNDED-INGEST.1` | py_compile + `cargo build` + full `run_ci.sh` (1587) + temp-14p before/after | GREEN; (A) single-pass BYTE-IDENTICAL; (B) batched complete+correct (`page_no` absolute, surfaces match, one benign boilerplate-header boundary split) |
| `2026-06-14` | `MEMORY-BOUNDED-INGEST.2` | CHI (585p) ingest under the autonomous RAM guard | GREEN — completed rc=0, NOT killed, PEAK used 20%; source_ir complete (585p/368 tables, abs unique page nums), 528 MB normalized, ~19 min |
| `2026-06-14` | `MEMORY-BOUNDED-INGEST.3` | py_compile + full `run_ci.sh` (1587) + 4-mode live CAN ingest (throwaway key) + stash-rebuild byte-identity | GREEN; (A) default 72 PNGs == old-code byte-identical; (B) SAVE=0 → 0 PNGs, region crops byte-identical, source_ir identical bar `page_image_path`; (C) force-large default → auto-skip+batched; (D) large+SAVE=1 → keep; per-page PNGs 21 MB vs 1.8 MB crops |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `MEMORY-BOUNDED-INGEST.1` | `MEMORY-BOUNDED-INGEST.1` | page-range batched ingestion behind a 512p gate (`9cb16985`) |
| `MEMORY-BOUNDED-INGEST.2` | `MEMORY-BOUNDED-INGEST.2` | CHI 585p ingested under the RAM guard, peak 20% used (`1ef6a1b4`) |
| `MEMORY-BOUNDED-INGEST.3` | `MEMORY-BOUNDED-INGEST.3` | disk-footprint bounding — skip persisting per-page PNGs for large docs (generate-in-memory for region crops, skip the disk write); `SPECFORGE_INGEST_SAVE_PAGE_IMAGES` override |

## Changelog

- `2026-06-14`: Created task tree (owner-directed, surfaced by the CHI `.13c` OOM). Design
  recorded in Decisions.
- `2026-06-14`: `.1` DONE — implemented page-range batched ingestion in the Docling helper
  (`_IngestAccumulator` + `process_converted_document` + `detect_pdf_page_count` + `_env_int`;
  single-pass unchanged ≤512p, batched >512p freeing each batch). py_compile + full CI green;
  temp-14p proof: single-pass byte-identical, batched complete/correct (one benign boilerplate
  boundary split). Commit `9cb16985`.
- `2026-06-14`: `.2` DONE — CHI (585p) ingested via the batched path under the autonomous RAM
  guard: completed rc=0, NOT killed, PEAK memory_pressure used = 20% (vs the single-pass OOM at
  17.2 GB / SIGKILL); source_ir complete + faithful (585p / 368 tables / abs unique page numbers),
  528 MB normalized, ~19 min, quality intact. `PDF-VARIANT-DIGESTION.13c` unblocked. Tree stays
  active for the size-immunity program: `.3` DISK (on-demand full-res page images) / `.4`
  restricted-env / `.5` summary streaming.
- `2026-06-14`: `.3` DONE — DISK-footprint bounding. Read-only investigation found NO downstream
  consumer reads the per-page full-res PNG (only figure/table region crops are read), but region
  crops are cropped from the in-memory page image, so generation must stay. Implemented
  generate-in-memory / skip-persist in the Docling helper (`_env_flag` + `save_page_images` gate):
  large docs no longer write `page-NNNN.png` (disk O(assets) not O(pages)); region crops stay
  full-res; `page_image_path` becomes honest `None` with dimensions retained. Default skips above
  `SPECFORGE_INGEST_BATCH_THRESHOLD`, persists at/below (small docs byte-identical); override via
  `SPECFORGE_INGEST_SAVE_PAGE_IMAGES=1/0`. Verified A/B/C/D on a throwaway CAN copy + stash-rebuild
  byte-identity; full CI green (1587). Book + KM card `page-image-disk-bounding` added. Both the RAM
  (`.1`/`.2`) and DISK (`.3`) dimensions of size-immunity are now delivered; `.4`/`.5` remain.
