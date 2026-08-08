# Project-Data Locality Standard

SpecForge-owned data must stay with the repository. Moving the repository to another directory,
volume, or computer must not silently send its temporary work, caches, dependency stores, logs, or
generated outputs back to a previous path or to an operating-system temporary directory.

## Required roots

Every persisted project path is repository-relative. At runtime, tools discover the current
repository root and derive absolute process paths only for the duration of that process.

The Rust `persisted_path` boundary makes that distinction explicit without changing existing JSON path
strings. A repository-owned path encodes relative and resolves below the discovered current root. An
explicitly authorized external input can remain absolute and is never eligible for old-root rebasing. Legacy
repository paths may rebase only from a recognized project-data root to exactly one existing target below the
current root; missing, ambiguous, traversing, or symlink-escaping values fail closed. SourceIR, EvidenceIR,
SemanticIR, IntentIR, adapter artifacts, and typed prior memory now serialize their repository-owned
registration, layout, lineage, provenance, emitted-target, and learned-source paths through that boundary while
restoring absolute current-root values in memory. Validation, project-validation, learning, recovery, KG, and
convergence consumers resolve repository paths at their I/O boundaries. Only the present generated corpus
remains owned by the open `ARTIFACT-PATH-PORTABILITY.4` migration and residue-enforcement leaf.

| Data | Repository-relative root | Authority |
| --- | --- | --- |
| temporary workspaces | `.project-data/tmp/` | `.cargo/config.toml`, `scripts/project_data_env.sh`, Rust `project_data` module |
| application and dependency caches | `.cache/` | the same shell/Rust environment contract |
| Rust build products | `target/` | Cargo workspace default |
| generated pipeline and documentation output | `generated/` | command-specific artifact layouts |
| Docling runtime | `.venv-docling/` | `requirements/docling-macos-arm64.lock.txt` |
| evaluation runtime | `.venv-eval/` | `requirements/eval-macos-arm64.lock.txt` |

`.project-data/tmp/.gitkeep` makes the temporary root exist in a fresh checkout before Cargo starts a
compiler, build script, or test. Its contents, `.cache/`, both virtual environments, `target/`, and
`generated/` are rebuildable and ignored by Git.

## Process environment

Canonical shell entrypoints source `scripts/project_data_env.sh`. Cargo applies the same relative,
forced defaults from `.cargo/config.toml`. Production Rust code does not trust ambient `TMPDIR`:
`project_data::tempdir()` creates temporary directories below `.project-data/tmp`, and
`project_data::configure_command()` passes the local contract to Docling, curl/VLM helpers, and the
pinned FSMGen subprocess boundary.

The controlled variables are:

- `SPECFORGE_REPO_ROOT`
- `TMPDIR`, `TMP`, and `TEMP`
- `XDG_CACHE_HOME`
- `HF_HOME` and `HUGGINGFACE_HUB_CACHE`
- `PIP_CACHE_DIR`
- `TORCH_HOME`
- `MPLCONFIGDIR`

The shell and Rust initializers create these roots, reject a symlink that escapes the repository, and
on Unix verify that every root has the repository's filesystem device. Callers do not need to export
`TMPDIR` manually.

FSMGen's pinned lowerer creates `File::Temp` `.fsm` intermediates but does not unlink them after a
normal invocation. Each SpecForge test invocation therefore receives a disposable child directory as
its `TMPDIR`/`TMP`/`TEMP`; the parent removes that directory after the child exits. The full CI flow
rechecks locality after all producers and rejects direct `.fsm` or `.log` residue in the shared temp
root.

## Explicit shared exceptions

The owner authorizes `~/.rustup` and `~/.cargo` as shared machine toolchain/dependency inputs. They may
remain on the boot volume and must not be copied or deleted as though they belonged only to
SpecForge. The repository still keeps Cargo build products in `target/`; it does not redirect them to
the shared Cargo home.

Operating-system executables and Homebrew interpreters are external tool dependencies. They are
read-only inputs where possible, not SpecForge data stores. No other user-home or OS-temporary cache
is an implicit exception.

The tracked post-compaction hook may optionally read an ahead-of-submodule FSMGen checkout. It derives
that checkout as the repository's `../fsmgen` sibling (or a caller-authorized override), reads it only
when it is on the repository filesystem, and otherwise skips it. The pinned `subs/fsmgen` gitlink
remains the reproducible downstream authority.

## Python environments and model caches

Python virtual environments embed absolute launcher and activation paths. They are therefore
rebuildable dependency stores, not portable artifacts. After any repository move, rebuild them from
the tracked locks:

```bash
bash scripts/bootstrap_docling.sh
bash scripts/bootstrap_eval.sh
```

Both scripts accept only repository-relative environment/lock overrides, build at the final local
path, compare `pip freeze --all` with the lock, smoke-test imports and launchers, and restore the
previous environment if a rebuild fails. SpecForge invokes the venv interpreter directly; it never
persists the absolute path in tracked configuration.

Docling models belong below `.cache/huggingface/hub/`. A shared pre-existing Hugging Face cache is
ambiguous global data: copy the exact required model repositories into the project cache, verify
counts/bytes/checksums and an offline resolution or ingest, then stop project access to the shared
copy. Do not delete the shared cache.

## Repository-move protocol

For exact project-owned data left on another volume:

1. identify the exact old root and prove whether tracked history, untracked files, ignored stores, or
   submodules contain anything not present in the new root;
2. copy data that must survive into its repository-derived destination;
3. verify file counts, byte sizes, checksums when material, and a real workflow from the new root;
4. delete only the exact old project-owned residue; and
5. run a residue census proving the old path no longer exists.

Never delete an ambiguous shared cache. If ownership cannot be proven, populate the local cache and
cease access to the shared copy instead.

## Enforcement and verification

`PROJECT-DATA-LOCALITY` is registered in `scripts/check_doctrines.sh`. Its checker validates Cargo and
shell defaults, required production temp/subprocess seams, Python lock authority, and any present
venv's direct prefix and launcher paths. Its focused shell cases reject missing roots and off-root
cache symlinks and prove all configured roots use the repository filesystem.

Run:

```bash
bash scripts/check_project_data_locality.sh
bash scripts/run_ci.sh
```

The full CI command needs no caller-supplied temp or cache environment.
