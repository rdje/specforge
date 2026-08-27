# PROVIDER-MODEL-STORE-LOCALITY: decide and gate where the VLM/NLP model store lives

## Metadata

- Tree ID: `PROVIDER-MODEL-STORE-LOCALITY`
- Status: `active` (`.0` measured; `.1`–`.2` pending)
- Roadmap lane: repository durability and portability (sibling of `LIVE-DOC-STOP-RISK`)
- Created: `2026-08-27`
- Last updated: `2026-08-27`
- Owner: repo-local workflow

## Goal

Bring the Ollama model store under the project-data locality standard — either as an explicitly authorized
external toolchain dependency or as a repository-derived store — and make whichever answer we choose a
**checked** fact rather than an unstated one.

## Non-Goals

- Do not delete or relocate an ambiguous shared cache. `PROJECT_DATA_LOCALITY.md` forbids it, and the same
  store may serve other projects on this machine.
- Do not make Ollama a required dependency of deterministic commands; the source-to-IntentIR replay path is
  provider-free and must stay that way.

## Reproduction and measurement (`.0`, `2026-08-27`, complete)

`PROJECT_DATA_LOCALITY.md` pins every model and cache root SpecForge's own child processes populate, and
`scripts/project_data_env.sh` plus `.cargo/config.toml` force each one to a repository-relative path:
`XDG_CACHE_HOME`, `HF_HOME`, `HUGGINGFACE_HUB_CACHE`, `PIP_CACHE_DIR`, `TORCH_HOME`, `MPLCONFIGDIR`, and the
temp roots. The standard then states the boundary explicitly: `~/.rustup` and `~/.cargo` are the authorized
shared toolchain inputs, Docling models "belong below `.cache/huggingface/hub/`", and **"No other user-home or
OS-temporary cache is an implicit exception."**

The Ollama model store is not among them, and it is the store the *default* VLM/NLP path uses:

| Fact | Measurement |
| --- | --- |
| Store path | `~/.ollama/models` |
| Size / files | 15 GB, 21 files |
| Device | `10000100000001a` (boot volume) |
| Repository device | `100001c0000001a` (`/Volumes/SSD`) |
| Default model | `qwen2.5vl:7b`, reported by `specforge doctor` |
| Locality checker seam | none — `scripts/check_project_data_locality.sh` never mentions Ollama |
| Env pin | none — `OLLAMA_MODELS` appears in no tracked configuration |

So the doctrine is *silent* here rather than satisfied: a 15 GB off-volume store feeds a production path, and
the gate that exists precisely to catch that cannot see it. The gap is a declaration gap, not evidence of data
loss — nothing is broken today, and the repository still runs its deterministic path with no provider at all.

### Why this is not simply "pin another env var"

The five pinned caches are populated by processes SpecForge launches, so forcing an environment variable
settles them. `OLLAMA_MODELS` is read by the **`ollama serve` daemon**, whose lifecycle SpecForge does not own:
the client seam is an HTTP endpoint (`http://localhost:11434/v1/chat/completions`), not a filesystem path.
Exporting `OLLAMA_MODELS` from `scripts/project_data_env.sh` would therefore change nothing unless the daemon
were also started from an activated environment, and would create a second 15 GB copy for anyone who did.

That asymmetry is why `.1` measures and decides before `.2` implements, and why the authorized-dependency
option is currently the more defensible one.

## Planned children

- ID: `PROVIDER-MODEL-STORE-LOCALITY.1`
  State: `pending`
  Goal: decide the disposition on evidence
  Acceptance: a decision record states whether the Ollama store is an authorized external toolchain dependency
  or a repository-derived store, names the daemon-lifecycle constraint above as the deciding evidence, and
  records the rejected option with its cost; the reviewed alternatives include a project-local store populated
  per the copy/verify/use/delete protocol

- ID: `PROVIDER-MODEL-STORE-LOCALITY.2`
  State: `pending`
  Goal: make the decision mechanically enforced
  Acceptance: `PROJECT_DATA_LOCALITY.md` names the provider model store explicitly in its shared-exception or
  project-store section, `scripts/check_project_data_locality.sh` gains a seam that fails when that declaration
  is absent or contradicted, and a RED control proves the seam fires; the mdBook project-data-locality chapter
  states the same rule
  Prerequisite: `PROVIDER-MODEL-STORE-LOCALITY.1`

## Open Questions

- Does any other locally-served provider (LM Studio, whose fallback endpoint SpecForge also probes) carry the
  same unstated store? `.1` should answer for both rather than only the default.

## Blockers

- None. `.1` is a decision slice needing no new measurement beyond `.0`.

## Verification Log

| Date | Unit | Result |
| --- | --- | --- |
| `2026-08-27` | `.0` reproduction | the Ollama store is 15 GB / 21 files at `~/.ollama/models` on the boot volume, a different device from the repository; the locality doctrine authorizes only `~/.rustup` and `~/.cargo`, routes Docling models to `.cache/huggingface/hub/`, and declares no other user-home cache an implicit exception; `scripts/check_project_data_locality.sh` contains no Ollama seam and no tracked configuration sets `OLLAMA_MODELS`, while `.cargo/config.toml` and `scripts/project_data_env.sh` force all seven other cache and temp roots repository-relative |

## Commit Log

| Unit | Commit | Outcome |
| --- | --- | --- |
| `.0` | `PROVIDER-MODEL-STORE-LOCALITY.0 — record the unauthorized provider model store` | route the off-volume Ollama model store and its missing locality declaration into an owning tree |

## Update protocol

Each child updates this file's node state, verification log, and commit log in its own commit. `.1`'s decision
belongs in `docs/decisions/` and is referenced here rather than restated.
