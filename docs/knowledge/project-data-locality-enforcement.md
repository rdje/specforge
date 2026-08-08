---
id: project-data-locality-enforcement
title: Project-owned temp, cache, dependency, and subprocess data resolves from the current repository
answers:
  - "where does SpecForge store temporary files and caches"
  - "does cargo test need TMPDIR set manually"
  - "what must be rebuilt after moving the SpecForge repository"
  - "why did the moved Python virtual environments still access the old repository"
  - "where are Docling models stored for SpecForge"
  - "which boot-volume Rust directories are allowed"
date: 2026-08-08
status: current
tags: [locality, portability, cache, tempfile, docling, python, cargo]
evidence: PROJECT_DATA_LOCALITY.md; .cargo/config.toml; scripts/project_data_env.sh; scripts/check_project_data_locality.sh; crates/specforge/src/project_data.rs; docs/tasks/LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.md
reverify: "bash scripts/check_project_data_locality.sh && test ! -e \"$HOME/Documents/github/specforge\""
---

# Project data follows the current repository

Cargo and canonical shell entrypoints force temporary and cache paths below the current repository.
Production Rust tempdirs use `.project-data/tmp/`; child Docling, curl/VLM helper, and pinned FSMGen
commands receive the same repository-derived environment. `target/`, `generated/`, `.cache/`, and both
Python environments remain local rebuildable stores. Shared `~/.cargo` and `~/.rustup` are the only
owner-approved boot-volume Rust exceptions.

The SSD move exposed a non-obvious Python boundary: copying `.venv-docling` and `.venv-eval` preserved
53 launcher/activation references to `$HOME/Documents/github/specforge`. Because that old path was a
real 2.9 GiB repository rather than a symlink, invoking an SSD console script started the boot-volume
interpreter even though invoking SSD `bin/python` directly used SSD packages. Exact locks and
rollback-safe bootstrap scripts now rebuild both venvs at their final current path; the stale-reference
count is zero.

The two Docling model repositories (342 MiB and 164 MiB) were copied byte-for-byte from the ambiguous
shared Hugging Face cache into `.cache/huggingface/hub`. Counts, allocated bytes, rsync checksums,
revision-aware offline resolution, and a real network-disabled one-page Docling ingest all passed.
The shared cache was deliberately preserved. After tracked-history ancestry, identical local-file,
submodule, dependency-inventory, and generated-residue checks, the exact old boot-volume SpecForge
tree was deleted and an absence census plus SSD-only runtime probes passed.

FSMGen's `File::Temp::tempfile` lowering seam leaves generated `.fsm` files behind when given a
long-lived shared `TMPDIR`. SpecForge therefore gives each pinned FSMGen test invocation its own
repository-local temporary directory and drops that directory only after the child exits. The final
CI locality recheck rejects any direct `.fsm` or `.log` residue left by the complete producer run.
