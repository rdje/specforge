# Project Data Locality

SpecForge keeps its own data on the repository filesystem. This includes temporary test and runtime
workspaces, generated outputs, Rust build products, Python environments, package caches, model
caches, logs, and disposable fixtures.

The default locations are:

| Kind | Location |
| --- | --- |
| temporary work | `.project-data/tmp/` |
| package/model/application caches | `.cache/` |
| Rust builds | `target/` |
| pipeline and book outputs | `generated/` |
| Docling environment | `.venv-docling/` |
| table-evaluation environment | `.venv-eval/` |

Cargo, the canonical scripts, production temporary-directory creation, Docling/VLM helpers, and the
FSMGen test boundary all derive these paths from the current repository root. You do not need to set
`TMPDIR` before `cargo test` or `bash scripts/run_ci.sh`. An off-root cache symlink or off-volume
project-data root is rejected. Each FSMGen invocation gets a disposable child temp directory, and the
final CI locality recheck rejects leaked `.fsm` or `.log` intermediates.

## After moving the repository

Virtual environments are not portable: their generated launchers contain absolute paths. Rebuild
them from the tracked macOS arm64 locks after a move:

```bash
bash scripts/bootstrap_docling.sh
bash scripts/bootstrap_eval.sh
```

The scripts preserve the previous environment until the new one matches its lock, imports correctly,
and has launchers rooted at the current repository. A failed install restores the previous directory.

Docling models resolve below `.cache/huggingface/hub/`. If they previously lived in a shared home
cache, copy the two required model repositories to the local cache and verify offline use. Leave the
shared cache intact because other projects may own it too.

The only owner-approved boot-volume exceptions are shared `~/.rustup` and `~/.cargo`, plus required
read-only operating-system/toolchain inputs. Rust build outputs remain in the repository `target/`.
The post-compaction hook can also read an optional `../fsmgen` sibling checkout, but only on the same
filesystem; the pinned submodule remains the reproducible authority.

### Known generated-IR move-portability gap

The runtime/cache controls above are enforced, but a post-move audit found a separate persistence gap.
SourceIR stores its canonical input path, and the EvidenceIR, SemanticIR, IntentIR, and adapter builders
canonicalize their input before persisting the upstream-stage pointer. The present generated tree therefore
contains 335 JSON/Markdown artifacts with the deleted pre-SSD repository root; EvidenceIR alone repeats it in
262,592 span/anchor `source_path` values.

This is not current boot-volume I/O—the old repository is absent—but it makes lineage consumers move-fragile.
Some validation and learning paths treat a missing upstream artifact as optional and silently lose cross-stage
checks; recovery paths can fail outright. `ARTIFACT-PATH-PORTABILITY` owns a common repository-relative storage
and safe runtime-resolution contract, backward-compatible legacy loading, verified generated-data migration,
and a fail-closed residue gate. Until that tree closes, do not bulk-rewrite ignored artifacts or treat a passing
temp/cache locality check as proof that persisted IR paths are portable.

## Check the contract

```bash
bash scripts/check_project_data_locality.sh
bash scripts/run_ci.sh
```

The first command checks the path/environment/runtime seams and focused fail-closed cases. The second
runs the complete project gate with those defaults already active.
