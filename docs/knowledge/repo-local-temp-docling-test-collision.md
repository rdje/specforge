---
id: repo-local-temp-docling-test-collision
title: Repository-local temp exposes a Docling test root-discovery and mutex-poison cascade
answers:
  - "why do Docling source tests fail when TMPDIR is inside the repository"
  - "why does inspect_docling_runtime PATH probe select RepoLocalVenv in tests"
  - "why do environment-lock tests cascade with PoisonError"
  - "which task owns deterministic Rust tests under repository-local TMPDIR"
date: 2026-08-08
status: current
tags: [testing, docling, tmpdir, locality, mutex]
evidence: docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md; crates/specforge/src/ir/source/docling_backend.rs; crates/specforge/src/test_support.rs
reverify: TMPDIR="$PWD/generated/tmp" cargo test --manifest-path Cargo.toml 'ir::source::' -- --test-threads 16
---

When `TMPDIR` is below the repository, `tempfile::tempdir()` creates the Docling PATH-probe fixture
under a real repository ancestor. `inspect_docling_runtime` walks ancestors, discovers the real
`.venv-docling`, and selects `RepoLocalVenv` before the fixture's `python3.11` PATH candidate. The test
expects `PathProbe` and panics while holding the shared `test_support::env_var_lock`.

Rust's standard mutex then becomes poisoned. Subsequent Docling/source tests call `lock().unwrap()` and
fail with `PoisonError`, so one root assertion appears as a large suite failure. The source-filtered
suite reproduced 27 pass / 11 fail at 16 threads. `.5g.iii` fixed the test seam by injecting whether
repo-local runtime discovery participates in the PATH-ordering probe and made the shared test mutex
recover its inner guard after poison; the same source set is now 38/38 at 16 threads with repository-
local `TMPDIR`, while production runtime discovery remains enabled. Pending leaf
`LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.6a` owns setting same-volume roots in canonical CI and auditing
all other temporary/cache workspaces.
