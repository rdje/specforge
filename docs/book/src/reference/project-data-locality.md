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

After moving projects, revalidate host-library symlinks as well as virtual environments. A relative link under
`.cache/` can still resolve to an old checkout on another volume. Resolve the link, compare its filesystem identity
with the repository, and fail closed before ingest when it contradicts the declared storage layout. Supply the
new same-volume route, or explicitly authorize and hash-verify a bounded read-only input copy into repository-local
project data; do not treat a merely readable stale target as current source authority.

### Persisted-path contract and migrated local state

The runtime/cache controls above are enforced. A post-move audit also found and repaired a separate historical
persistence gap: older SourceIR, EvidenceIR, SemanticIR, IntentIR, adapter, and prior-memory producers had
persisted canonical absolute paths. The measured legacy tree contained 335 JSON/Markdown artifacts and 262,996
values naming the retired repository; EvidenceIR accounted for 262,592 span/anchor values.

This is not current boot-volume I/O—the old repository is absent—but it makes lineage consumers move-fragile.
Some validation and learning paths treat a missing upstream artifact as optional and silently lose cross-stage
checks; recovery paths can fail outright.

The common Rust contract preserves the existing JSON path-string shape while requiring a typed origin at each
call: repository-owned paths encode relative and resolve at the discovered current root; explicit external
inputs may remain absolute but never enter legacy rebasing. An old absolute repository path may rebase only
below a recognized project-data root, to one unambiguous target contained by the current repository. Inputs
that must be opened now require a present leaf. Historical source or provenance references may survive
deliberate cleanup of their materialized leaf, but their nearest existing ancestor must remain contained.
Parent traversal, zero/multiple legacy roots, and symlink escape are errors.

All canonical stages now use that contract. Their Rust values are resolved absolute paths while the process is
running, so normal callers can open them directly. Their JSON and SourceIR sidecar manifests store
repository-owned source/layout/upstream/prior/provenance paths relative to the repository. Source text carries
an origin label; an authorized external Markdown or PDF path remains absolute, while generated assets remain
repository-owned. SemanticIR, IntentIR, and adapter layouts/upstream/emitted-target fields use the same split,
and typed prior memory stores learned source-artifact paths relative. Validation, project-validation, learning,
recovery, KG fixtures, and convergence resolve repository artifacts through the common boundary.

The closure audit also covered path-bearing schemas that have no current artifact. Optional
`FigureRegion.raw_image_path` uses repository-owned field-level serialization even though the upstream figure
extractor is not wired yet. Project-rescan `artifact_path`, replay `path`, and command `working_directory`
strings are likewise pinned to relative forms (`working_directory` is `.`), rather than escaping review because
their Rust type is `String`.

The guarded migration changed exactly 392 of 978 files: 262,996 retired-root values became repository-relative
and 157 missing origin labels were added. The result has the same file set, 721,679,372 logical bytes, and zero
retired-root values; 82 explicitly labeled absolute source-library values remain by design. Source, Evidence,
Semantic, Intent, adapter, learning, and recovery workflows were exercised against the migrated corpus before
the exact same-volume rollback copy was removed.

The locality doctrine now runs 12 scanner self-tests, pins eleven producer/consumer files (including the
dormant FigureRegion field), and scans every JSON artifact under `generated/`. Any absolute path-valued field,
including a command working directory, is rejected unless it is one of the narrow, origin-labeled external
SourceIR/EvidenceIR provenance fields. This covers canonical stages, validation output, rescan plans, and source
sidecars whenever they are present.

## Check the contract

```bash
bash scripts/check_project_data_locality.sh
bash scripts/run_ci.sh
```

The first command checks the path/environment/runtime seams and focused fail-closed cases. The second
runs the complete project gate with those defaults already active.
