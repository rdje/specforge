---
id: dormant-serialized-paths-require-portability
title: Dormant serializable path fields require the portability contract before activation
answers:
  - "does a PathBuf need portability handling before it has a producer"
  - "how is FigureRegion raw_image_path serialized"
  - "can FigureRegion raw_image_path store an absolute path"
  - "why did generated artifact scanning miss FigureRegion raw_image_path"
  - "does the persisted path gate cover dormant schemas"
  - "which project rescan string fields are treated as filesystem paths"
  - "how are project rescan working directories kept portable"
date: 2026-08-09
status: current
tags: [artifact-paths, portability, figure-region, rescan-plan, dormant-schema]
evidence: crates/specforge/src/ir/figure_region.rs; crates/specforge/src/commands/project_validation.rs; scripts/check_persisted_artifact_paths.pl; docs/tasks/ARTIFACT-PATH-PORTABILITY.md (.5)
reverify: "cargo test -p specforge ir::figure_region::tests && scripts/check_persisted_artifact_paths.pl --self-test"
---

Persistence ownership begins when a field can serialize, not when its first producer is wired. The independent
portability cold read found that `FigureRegion.raw_image_path` derived Serde directly while no upstream PDF
extractor emitted `FigureRegion` records. Present-artifact scans therefore had no instance capable of exposing
an absolute-path regression.

The optional field now has repository-owned field-level Serde: serialization calls `normalize_for_storage`,
deserialization calls `resolve_reference`, repository-local paths round-trip as current-root runtime paths, and
an unlabeled external absolute path fails closed. The persisted-artifact doctrine pins all three code seams so
removing the contract fails even while the schema remains dormant.

Filesystem identity may also use a non-`PathBuf` schema type. Project-rescan `artifact_path` and replay `path`
strings are produced through `repo_relative_display`; command hints persist `working_directory` as `.`. The
producer oracle pins those expressions, and the artifact scanner treats `working_directory` as path-valued so
a future absolute command root is rejected.
