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

### Persisted-path contract and present-data migration gap

The runtime/cache controls above are enforced, but a post-move audit found a separate historical persistence
gap. Older SourceIR, EvidenceIR, SemanticIR, IntentIR, adapter, and prior-memory producers persisted canonical
absolute paths. The present generated tree therefore contains 335 JSON/Markdown artifacts with the deleted
pre-SSD repository root; EvidenceIR alone repeats it in 262,592 span/anchor `source_path` values.

This is not current boot-volume I/O—the old repository is absent—but it makes lineage consumers move-fragile.
Some validation and learning paths treat a missing upstream artifact as optional and silently lose cross-stage
checks; recovery paths can fail outright.

The common Rust contract preserves the existing JSON path-string shape while requiring a typed origin at each
call: repository-owned paths encode relative and resolve at the discovered current root; explicit external
inputs may remain absolute but never enter legacy rebasing. An old absolute repository path may rebase only
below a recognized project-data root, to exactly one existing canonical target contained by the current
repository. Parent traversal, zero/multiple targets, and symlink escape are errors.

All canonical stages now use that contract. Their Rust values are resolved absolute paths while the process is
running, so normal callers can open them directly. Their JSON and SourceIR sidecar manifests store
repository-owned source/layout/upstream/prior/provenance paths relative to the repository. Source text carries
an origin label; an authorized external Markdown or PDF path remains absolute, while generated assets remain
repository-owned. SemanticIR, IntentIR, and adapter layouts/upstream/emitted-target fields use the same split,
and typed prior memory stores learned source-artifact paths relative. Validation, project-validation, learning,
recovery, KG fixtures, and convergence resolve repository artifacts through the common boundary.

Loading an old unlabeled artifact infers present external identity exactly or uses the bounded legacy rebase for
repository data. Code activation is complete; only the current ignored corpus is still old. Its next leaf adds a
fail-closed present-artifact gate, migrates it with file/byte/hash and real-workflow verification, and proves no
retired-root residue remains. Until that verified migration lands, do not bulk-rewrite the ignored artifacts or
mistake producer readiness for completed present-data migration.

## Check the contract

```bash
bash scripts/check_project_data_locality.sh
bash scripts/run_ci.sh
```

The first command checks the path/environment/runtime seams and focused fail-closed cases. The second
runs the complete project gate with those defaults already active.
