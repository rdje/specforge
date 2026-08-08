---
id: live-document-containment-and-data-locality
title: Live documents are bounded by lifecycle and project data stays on the repository volume
answers:
  - "how are SpecForge live documents kept bounded"
  - "what owns live-document size limits and transition debt"
  - "where must SpecForge project artifacts caches and temporary workspaces live"
  - "why are rustup and cargo allowed on the boot volume"
date: 2026-08-08
status: current
tags: [documentation, containment, locality, portability, doctrine]
evidence: LIVE_DOCUMENT_SIZE_CONTAINMENT.md; doctrine/live_document_size/surfaces.jsonl; docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md
reverify: bash scripts/check_live_document_size.sh
---

# 0007 — Live-document containment and repository-volume data locality

- Date: 2026-08-08
- Status: accepted
- Deciders: project owner, SpecForge repository workflow

## Context

SpecForge moved with the owner's Git projects from the boot volume to an external SSD. The project
also accumulated multiple mandatory or frequently-read Markdown surfaces measured in hundreds of
kilobytes or megabytes, plus several generated projections and collections without common pressure
controls. A small `MEMORY.md` alone cannot prevent its neighboring status, task, research, or manual
surfaces from becoming the next unbounded bootstrap read.

The owner asked SpecForge to adopt a README stability policy and to review FSMGen's
`LIVE_DOCUMENT_SIZE_CONTAINMENT_ADOPTION_GUIDE.md`. The local audit found the adoption warranted:
the pre-trim README was 602 lines/170,891 bytes; the tracked Markdown set was 12.25 MB; several live
ledgers exceeded 0.5–2.6 MB; and Rust's default temporary-workspace behavior could write project-owned
test/runtime data to boot-volume `/private/tmp` even though the repository now lives elsewhere.

## Decision

Adopt the project-neutral live-document size-containment doctrine as SpecForge's fifth mechanically
enforced doctrine, with these local decisions:

1. Root `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` is the SpecForge-owned authority. The reviewed FSMGen
   copy is precedent, not an upstream; no local threshold, debt, path, or migration result syncs.
2. `doctrine/live_document_size/surfaces.jsonl` is the data-only authority for every parent-tracked
   Markdown surface. Coverage is exact: every `git ls-files '*.md'` path matches one and only one
   surface. The FSMGen gitlink is outside the parent index and remains independently owned.
3. Every surface declares one closed lifecycle: `bounded_snapshot`, `rolling_ledger`,
   `partitioned_canonical`, `generated_projection`, `archive_terminal`, `frozen_legacy`, or
   `maintained_reference`. Lifecycle-specific fields are mandatory, not advisory.
4. Limits are independent across file count, per-file lines/bytes, aggregate lines/bytes, and maximum
   content-line bytes. Healthy limits are derived from reviewed local survivors with explicit
   headroom. Existing breaches keep an immutable measured baseline and bounded transition allowance;
   debt is never relabelled as health.
5. Unique product documentation stays complete. The mdBook is a `maintained_reference`: a bounded,
   direct `SUMMARY.md`, per-part limits, and exact fresh task-owned aggregate-change authority replace
   a decorative fixed aggregate cap.
6. `scripts/check_live_document_size.sh` deterministically checks the resulting tree and composes the
   independent README route guard. It is registered as `LIVE-DOC-SIZE` in
   `scripts/check_doctrines.sh`, so local hooks and CI use the same verdict.
7. Any ceiling increase requires a data-only authority record naming the work unit, owner, old/new
   values, and rationale. Decreases need no exception and become the new ceiling. Adoption baselines
   are initial values, not increases.
8. Project-owned outputs, build products, caches, dependency stores, logs, fixtures, and temporary
   workspaces must resolve from the repository root and remain on its filesystem volume. Shared
   `~/.rustup` and `~/.cargo` are explicit machine-toolchain/dependency exceptions authorized by the
   owner; SpecForge must not treat them as project artifact stores. Leaf `.6` implements and proves
   this locality half, including submodule invocation boundaries and residue checks.

## Consequences

- The live tree becomes a bounded working set without deleting unique product knowledge. Chronology
  moves only through whole-record, identity-checked migrations with direct retrieval and archive
  manifests.
- Existing oversized surfaces are visible debt with named owners. Bounded transition allowance may
  carry only this adoption's continuity updates; it cannot move the baseline or authorize unrelated
  growth.
- New Markdown, changed routes, stale generated projections, off-root paths, broken indexes, modified
  frozen records, or unexplained ceiling increases fail the common doctrine gate.
- The README has a deliberately independent guard because it is the public landing interface; the
  complete live-document checker composes it rather than weakening or replacing it.
- The external-SSD move does not require vendoring the shared Rust toolchain. It does require project
  temp/build/cache defaults and downstream-tool invocations to stop leaking owned data back to the
  boot volume.

## Links

- Doctrine: `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`
- README policy: `README_POLICY.md`
- Registry: `doctrine/live_document_size/surfaces.jsonl`
- Checker: `scripts/check_live_document_size.sh`
- Owning task-tree: `docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md`
