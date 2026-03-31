# MEMORY
## Purpose
- maintain a compact but actionable continuity record for interrupted sessions
- preserve enough context to resume work quickly after session loss, tool restart, machine crash, or model handoff

## Current project identity
- repository name: `specforge`
- working CLI/binary name: `spec2fsm`
- implementation language: Rust
- product shape: staged protocol/module-spec extraction tool, not one-shot direct conversion

## What the user has explicitly required
- live documents are critical infrastructure, not optional afterthoughts
- all live documents together must preserve full operational continuity after session loss, crashes, or tool restarts
- `RUST_CODEBASE_ANALYSIS.md` must remain a live deep-dive analysis of the Rust codebase
- `MEMORY.md` must be updated after completed tasks and also at meaningful checkpoints while long tasks are in progress
- `MEMORY.md` must contain the latest committed Git hash and the corresponding brief commit message, or explicitly state that no commit exists yet
- `MEMORY.md` must stay compact, precise, and operational rather than becoming a verbatim transcript
- `CHANGES.md` must contain the full detailed set of changes about to be committed
- `DEVELOPMENT_NOTES.md` must capture engineering decisions, rationale, and context
- `USER_GUIDE.md` must explain the tool from the user perspective
- `ROADMAP.md` must track the goals, remaining work, and advancement
- `README.md` must be the single entry point and its last line must be:
  - `Read SESSION_BOOTSTRAP.md and start from there.`
- `SESSION_BOOTSTRAP.md` content is user-specified and must stay aligned with that instruction

## Repository state at this checkpoint
- Git repository exists
- the repo originally contained only `COMMIT.md` and `.git/`
- the initial documentation and continuity surface has been established
- the first Rust workspace now exists:
  - `Cargo.toml`
  - `Cargo.lock`
  - `crates/spec2fsm/`
- the first runnable CLI command surface now exists:
  - `inspect <path>`
  - `ingest <source> --dry-run`

## Latest committed baseline
- latest_commit_hash: none yet
- latest_commit_brief_message: none yet
- continuity_rule:
  - refresh this section whenever a new latest committed baseline exists at the time `MEMORY.md` is updated

## Important session history
- user approved the idea of a staged Rust tool
- the repository `specforge/` was created and Git-initialized
- `COMMIT.md` was reviewed and its workflow constraints were loaded
- the user clarified that live documents must support seamless crash recovery and mid-task resumption
- the user provided exact required content for `SESSION_BOOTSTRAP.md`
- the initial documentation/bootstrap surface was created before any Rust code
- Rust toolchain availability was verified:
  - `cargo 1.89.0`
  - `rustc 1.89.0`
- the user then asked to proceed with the next task, which is the first Rust workspace and CLI scaffold
- the first Rust workspace and CLI scaffold were then implemented and validated successfully
- the user then asked to reload the prior AXI extraction workspace and use its method as the starting precedent for the current `specforge` work
- the AXI method, prompt, worksheet, dossier, actor catalog, and decomposition artifacts were re-read and confirmed as the current methodological baseline
- the main refinement decision is that `specforge` should preserve that staged method but keep typed Rust data as the system of record, with markdown worksheets/catalogs/decomposition artifacts generated from typed data later
- the user then clarified that the real project objective is specification intent capture and that automation should be pushed toward 100% wherever safely possible
- any work that cannot yet be automated must be reduced to structured minimal decision points rather than left as ad hoc manual interpretation

## In-flight work in this session
- establish the initial live-document set
- make `README.md` the single project entry point
- record the initial roadmap, live status, and Rust architecture baseline
- persist the user's continuity requirements into repo documentation
- reconcile wording and status across the live docs
- scaffold the first Rust workspace and `spec2fsm` CLI
- refresh the live docs to reflect the new codebase state
- recover the AXI extraction precedent and map it onto the next ingest/IR implementation slice
- run the first repository commit workflow for the initial bootstrap and documentation/code baseline

## Current execution checkpoint
- live-document bootstrap is complete and the first Rust workspace now exists
- validation already completed for the initial workspace and CLI surface
- baseline validation was re-run successfully during the first commit workflow
- the next immediate action is to implement the first real ingest manifest and normalized source model
- that next slice should now explicitly carry forward:
  - dossier/identity/conversion provenance
  - normalized markdown promotion paths
  - section-map and evidence hooks
  - explicit distinction between source facts, derived rules, local design decisions, and abstractions
  - residual-decision scaffolding for incomplete automation
  - intent capture as the organizing goal of the typed model

## If resuming from an interruption
1. read `README.md`
2. read `SESSION_BOOTSTRAP.md`
3. inspect `LIVE_ACHIEVEMENT_STATUS.md`
4. inspect `ROADMAP.md`
5. inspect `RUST_CODEBASE_ANALYSIS.md`
6. continue with the next implementation slice unless the user redirects

## Recommended next implementation slice
- implement a real ingest manifest
- add normalized source-model data structures
- add a residual-decision type for unresolved but structured user choices
- keep `inspect` as the lightweight deterministic inspection command
- evolve `ingest --dry-run` into the first real staged ingest entrypoint

## Commit status
- no commit has been made yet for this repository in this session
- before any commit, follow `COMMIT.md`
