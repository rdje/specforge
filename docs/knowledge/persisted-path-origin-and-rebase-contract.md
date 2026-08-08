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
date: 2026-08-08
status: current
tags: [artifact-paths, portability, locality, compatibility, external-inputs]
evidence: crates/specforge/src/persisted_path.rs; docs/tasks/ARTIFACT-PATH-PORTABILITY.md (.1); PROJECT_DATA_LOCALITY.md
reverify: "cargo test -p specforge persisted_path::tests"
---

`persisted_path` preserves the existing JSON path-string shape but requires each typed field seam to declare
`repository_owned` or `external_input`. Repository-owned runtime paths encode relative to the discovered root.
Relative and current-absolute values resolve at that current root; an old absolute value can rebase only from a
recognized project-data root to exactly one existing canonical target below the repository. Missing or multiple
targets, parent traversal, and symlink escape fail closed.

An authorized external input resolves only at its exact absolute path and never enters legacy rebasing. Mixed-
origin SourceIR registration will persist the stable origin label beside its canonical source path in
`ARTIFACT-PATH-PORTABILITY.2`; upstream-stage pointers are intrinsically repository-owned. This avoids wrapping
every existing path in a new schema object while preventing an external path from silently masquerading as
moved project data.
