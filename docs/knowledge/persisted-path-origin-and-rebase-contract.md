---
id: persisted-path-origin-and-rebase-contract
title: Persisted paths separate repository ownership from authorized external inputs
answers:
  - "how does SpecForge serialize repository owned paths"
  - "how does SpecForge resolve a persisted path after the repository moves"
  - "can an external absolute input be rebased into the repository"
  - "what makes legacy absolute path rebasing safe and unambiguous"
  - "does the move portability repair change PathBuf JSON fields"
  - "how are repository owned and external input path origins labeled"
  - "do SourceIR and EvidenceIR keep absolute paths in memory"
  - "which SourceIR and EvidenceIR paths serialize repository relative"
  - "which SemanticIR IntentIR adapter and prior memory paths serialize repository relative"
  - "can provenance load after source normalized cleanup removes its leaf"
  - "which persisted paths must exist and which may be historical references"
  - "does the path portability contract cover FigureRegion raw images"
date: 2026-08-09
status: current
tags: [artifact-paths, portability, locality, compatibility, external-inputs]
evidence: crates/specforge/src/persisted_path.rs; crates/specforge/src/ir/source.rs; crates/specforge/src/ir/evidence.rs; crates/specforge/src/ir/figure_region.rs; crates/specforge/src/ir/semantic.rs; crates/specforge/src/ir/intent.rs; crates/specforge/src/ir/adapters.rs; crates/specforge/src/ir/prior_memory.rs; docs/tasks/ARTIFACT-PATH-PORTABILITY.md (.1-.5); PROJECT_DATA_LOCALITY.md
reverify: "cargo test -p specforge persisted_path::tests"
---

`persisted_path` preserves the existing JSON path-string shape but requires each typed field seam to declare
`repository_owned` or `external_input`. Repository-owned runtime paths encode relative to the discovered root.
Relative and current-absolute values resolve at that current root; an old absolute value can rebase only from a
recognized project-data root to exactly one existing canonical target below the repository. Missing or multiple
targets, parent traversal, and symlink escape fail closed.

Existence is role-specific. `resolve_existing` remains mandatory for stage inputs and any path that a workflow
must open now. `resolve_reference` is limited to SourceIR/EvidenceIR source and provenance fields whose leaf may
have been intentionally reclaimed after downstream capture. Repository references still require a contained
existing ancestor and unique legacy suffix; an explicitly labeled missing external reference preserves its exact
identity. Storage normalization keeps a missing in-repository reference relative, even if its caller-facing
origin label was external.

An authorized external input resolves only at its exact absolute path and never enters legacy rebasing. SourceIR
registration now persists the stable origin label beside its canonical source path; upstream-stage pointers are
intrinsically repository-owned. SourceIR and EvidenceIR builders/loaders expose resolved absolute paths in memory
for existing consumers, while `to_pretty_json` and `write_to_disk` serialize normalized clones. Their source and
artifact layouts, normalized page/visual assets, upstream/prior pointers, and section/span/visual provenance are
therefore move-safe without wrapping every existing path in a new schema object.

SemanticIR, IntentIR, and adapter loaders/builders apply the same two-form model to their upstream pointers and
artifact layouts; the adapter also normalizes its optional emitted `.isf` target. `CorpusMemory::to_pretty_json`
normalizes learned `source_artifacts[].artifact_path`, and the learning writer resolves its output only at the
I/O boundary. Validation, project-validation, recovery, KG fixtures, and convergence use the same resolver for
repository artifacts and outputs. These downstream fields are intrinsically repository-owned, so they need no
additional mixed-origin schema label.

Evidence text provenance inherits the promoted Markdown origin because authorized external Markdown remains a
real external input. Evidence visual provenance is repository-owned because those images and caption sources
come from SourceIR's normalized artifact bundle. `ARTIFACT-PATH-PORTABILITY.4` migrated the current ignored
corpus and added a doctrine oracle over all present generated JSON. Current artifacts contain zero
repository-owned absolute values; labeled external source provenance is the only absolute allowance.

The closure cold read extended the contract to serializable surfaces that have no present artifact. Optional
`FigureRegion.raw_image_path` uses field-level repository-owned normalization/reference resolution before its
upstream producer exists. Project-rescan artifact/replay strings are pinned to repository-relative display and
its `working_directory` is an explicitly scanned path-valued field. The permanent oracle now pins eleven
producer/consumer files and its 12 self-tests cover this dormant/non-suffix boundary.
