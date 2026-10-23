# RUSTDOC-AND-XTASK: publish the rustdoc API reference, and move the repository's tooling to cargo xtask

## Metadata

- Tree ID: `RUSTDOC-AND-XTASK`
- Status: `active`
- Roadmap lane: repository durability and portability
- Created: `2026-10-23`
- Last updated: `2026-10-23`
- Owner: repo-local workflow

## Goal

SpecForge publishes a rustdoc API reference of its workspace crates beside the mdBook, kept warning-free at
commit cadence, and its repository tooling moves to `cargo xtask` one command at a time: a Rust entry point that
needs nothing but the toolchain, with each port proven to behave exactly as the script it replaces.

## Non-Goals

- Rewriting historical records. Verification logs, commit logs and closed leaves cite the scripts they ran; they
  stay as written, and a ported script stays as a shim that forwards to `cargo xtask`.
- A big-bang port. Each gate moves in its own leaf, with parity evidence, or not at all.
- New dependencies. The `xtask` crate uses the standard library only, so it builds offline.
- Requiring documentation on every public item now. Coverage is measured and ratcheted, never switched on blind.

## Acceptance Criteria

- `cargo doc --workspace --no-deps` with `-D warnings` passes, and a commit that breaks it is refused.
- The API reference is built into the book's output and linked from the book.
- `cargo xtask` exists, is not a default member, and dispatches to the current tooling with no behaviour change.
- Every ported command reproduces its script's output and exit codes, self-tests included, before the script
  becomes a shim.
- The book, ROADMAP and `MEMORY.md` state the current tooling.

## Task Tree

- ID: `RUSTDOC-AND-XTASK`
  Status: `active`
  Goal: the rustdoc API reference, and the incremental move to `cargo xtask`
  Children: `RUSTDOC-AND-XTASK.1`-`.15`

- ID: `RUSTDOC-AND-XTASK.1`
  Status: `done` (`2026-10-23`; DESIGN)
  Goal: measure the rustdoc build and the scripts, and decide the API reference and the xtask shape
  Acceptance: the rustdoc build's state and coverage per crate; the scripts by language, size and caller; the
  decisions below.
  **Measured `2026-10-23`** (`.project-data/tmp/rdx-1/`).
  - **Rustdoc.** CI already runs `cargo doc --workspace --no-deps` under `-D warnings` (`scripts/run_ci.sh`), and
    it **fails**: one unresolved intra-doc link, `[CLK_I]` in a doc comment quoting Wishbone's *rising [CLK_I]
    edge* (`crates/specforge/src/ir/evidence.rs`, since `EXTRACTION-REPRESENTATION-GAPS.17`, `2026-09-26`). It is
    the only rustdoc warning in the workspace. The push-time CI is the only place that runs rustdoc, so the break
    sat for four weeks. The build takes 45 s cold and writes 161 MiB to `target/doc` for the four crates.
  - **Coverage.** `cargo clippy --lib -- -W missing_docs` counts undocumented public items: `specforge-core`
    2,119, `specforge` 2,922, `specforge-conformance` 2,883, `specforge-production-graph` 33. `specforge-core`
    compiles the `ir` sources through `#[path]`, so it is the library API.
  - **Scripts.** 123 under `scripts/`: 64 Python (23,831 lines), 32 Perl (26,723), 27 shell (4,694). The doctrine
    driver (`check_doctrines.sh`) calls 23 of them, CI 5, the hooks 1 (the driver), and 53 have no scripted caller
    (maintenance tools run by hand). Docs, doctrine and task records cite them 8,089 times, 2,564 of those
    `run_ci.sh`, mostly in verification logs.
  **Decided.**
  - **The API reference.** The four workspace crates, `--no-deps`, built into `generated/mdbook/specforge/api/`
    beside the book and linked from a book reference page. `specforge_core` is the API a reader wants; the binary and
    tool crates are documented as they are.
  - **Kept green at commit cadence.** A `RUSTDOC` doctrine runs `cargo doc --workspace --no-deps` under
    `-D warnings` in the commit gate, so a broken link is refused when it is written, not at the next push.
  - **Coverage is a ratchet.** No `missing_docs` lint now; a later leaf records the counts and refuses an increase.
  - **The xtask crate.** A workspace member `xtask`, not a default member, standard library only, run through
    `cargo xtask` (an alias in `.cargo/config.toml`). It starts as a dispatcher to the current scripts, with no
    behaviour change, and gains native commands one at a time; its first native command builds the docs.
  - **A port's rules.** A port reproduces the script's stdout, stderr and exit code on the same inputs, ports its
    self-tests, and then the script becomes a shim (`exec cargo xtask …`), so every recorded command keeps working.
    `xtask` is tooling, not product: `.4` measures what the production-genericity census reads and keeps `xtask`
    out of it.
  Verification: `measurement scripts in .project-data/tmp/rdx-1/; doctrine gates at commit`
  Commit: `RUSTDOC-AND-XTASK.1 — measure rustdoc and the scripts, decide the API reference and xtask (DESIGN)`

- ID: `RUSTDOC-AND-XTASK.2`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: rustdoc builds warning-free, and a commit that breaks it is refused
  Acceptance: escape the `[CLK_I]` quotation; register a `RUSTDOC` doctrine (`cargo doc --workspace --no-deps`,
  `-D warnings`) in the commit gate, RED with the link restored; full CI's docs step passes.
  **Done `2026-10-23`.** The quotation is escaped (`\[CLK_I\]`), so rustdoc reads it as text.
  `scripts/check_rustdoc.sh` is registered as the `RUSTDOC` doctrine at `gate` tier, so the pre-commit hook and
  `--fast` run it. It first documents a scratch crate whose doc comment links to nothing and requires that build to
  fail with *unresolved link*, then runs CI's own command, `cargo doc --offline --locked --workspace --no-deps`
  under `-D warnings`. It costs 0.3 s on a tree whose Rust is unchanged and about 75 s when `specforge-core` is
  re-documented, so it stays in the fast set. RED: with the bracket restored the gate fails on `specforge-core`; the
  file restored byte-identically.
  Verification: `bash scripts/check_doctrines.sh --only RUSTDOC` PASS; RED above; oracles in the checklist
  Commit: `RUSTDOC-AND-XTASK.2 — rustdoc builds warning-free, and a commit that breaks it is refused`

- ID: `RUSTDOC-AND-XTASK.3`
  Status: `done` (`2026-10-23`)
  Goal: the API reference is published beside the mdBook
  Acceptance: the docs build writes the four crates' rustdoc into `generated/mdbook/specforge/api/`; a book
  reference page links each crate's index; `mdbook build` and the link targets are verified.
  **Done `2026-10-23`.** `scripts/run_docs_ci.sh` now builds the rustdoc after the book, under the `RUSTDOC`
  doctrine's `-D warnings`, copies `target/doc` into `generated/mdbook/specforge/api/` and fails if any of the four
  crates' `index.html` is missing. The book's new chapter `reference/api-reference.md` (listed in `SUMMARY.md` under
  Reference) says what each crate is, `specforge_core` first, and links each index. Measured: the docs build passes;
  the four links on the built page resolve to real files; the API reference is 196 MiB under the gitignored
  `generated/`.
  Verification: `bash scripts/run_docs_ci.sh` PASS; the built page's four `../api/<crate>/index.html` links resolve
  Commit: `RUSTDOC-AND-XTASK.3 — the API reference is published beside the mdBook`

- ID: `RUSTDOC-AND-XTASK.4`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask` dispatches to the current tooling
  Acceptance: an `xtask` workspace member (not default, standard library only) and the alias; `cargo xtask
  doctrines`, `ci` and `docs` run `check_doctrines.sh`, `run_ci.sh` and `run_docs_ci.sh` with their arguments and
  exit codes; the production-genericity census reads no `xtask` code (measured, not assumed); the book's
  contributor page documents it.
  **Done `2026-10-23`.** `xtask/` is a workspace member (not a default one) with no dependencies; `.cargo/config.toml`
  aliases `cargo xtask` to `cargo run --quiet --package xtask --`. Its library holds the command table (`TASKS`:
  `doctrines`, `ci`, `docs`), runs a command's script from the repository root by its repository-relative path, with
  the arguments passed through, and returns the script's exit code (128 plus the signal for one a signal ended);
  `help` lists the commands and an unknown one exits 2. **Parity:** `cargo xtask doctrines` and `bash
  scripts/check_doctrines.sh` give byte-identical stdout and stderr and the same exit code for `--list` (0),
  `--bogus` (2), `--only RUSTDOC` (0) and `--only NOPE` (2); the first run differed only in `$0` until the script was
  named by its relative path. `cargo xtask docs` and `run_docs_ci.sh` both pass, with identical step lines and an
  identical API file list. `ci` runs through the same function. **The census:** `module_inventory.tsv` lists only
  `crates/specforge*` paths and `flow_census.json` did not move, so the census reads no `xtask` code; the
  `DEPENDENCIES` component, which refused the unexpected member, now allows `xtask` and requires it to depend on
  nothing and nothing to depend on it (two self-test cases, 7/7 -> 9/9).
  Verification: `cargo test -p xtask --lib` 3/3; parity runs in `.project-data/tmp/rdx-4/`
  Commit: `RUSTDOC-AND-XTASK.4 — cargo xtask dispatches to the current tooling`

- ID: `RUSTDOC-AND-XTASK.5`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask docs` builds the mdBook and the API reference natively
  Acceptance: the first native command reproduces `run_docs_ci.sh`'s behaviour and output tree; the script becomes
  a shim.
  **Done `2026-10-23`.** `xtask/src/project_data.rs` activates the project-data roots as `project_data_env.sh` does
  (the exported variables, the created directories, and the refusals: no SpecForge repository, a path escaping the
  repository, a path on another filesystem, by device id where the shell used `df`). `xtask/src/docs.rs` is the docs
  build: the `mdbook` presence check, `mdbook test` and `mdbook build`, the rustdoc under `-D warnings`, the copy into
  `generated/mdbook/specforge/api/` and the four-crate index check, each step printing the script's line and
  returning the failing step's exit code. A command is now a `Runner`, a script or native code naming the script it
  replaced; `docs` is native, and `scripts/run_docs_ci.sh` is a shim (`exec cargo xtask docs "$@"`), which a test
  requires of every native command's script. The shim still activates the project-data roots first, as
  `PROJECT-DATA-LOCALITY` requires of every `scripts/*.sh`, so cargo's own process resolves them; the doctrine now
  also requires the native seam (`project_data::activate(root)` in `docs.rs`, the filesystem refusal in
  `project_data.rs`), RED with the call removed. **Parity** with HEAD's script: on success both exit 0 with identical
  stdout, stderr (timings masked) and API file list; with `mdbook` off `PATH` both print *mdbook is required but
  not installed* and exit 1, byte-identically.
  Verification: `cargo test -p xtask --lib` 7/7; parity runs in `.project-data/tmp/rdx-5/`
  Commit: `RUSTDOC-AND-XTASK.5 — cargo xtask docs builds the book and the API reference natively`

- ID: `RUSTDOC-AND-XTASK.6`
  Status: `done` (`2026-10-23`; DESIGN; opened `.8`-`.15`)
  Goal: the order in which the driver's 23 gates move to xtask
  Acceptance: rank the gates by cost and risk (size, self-tests, callers), open one leaf per gate in that order, and
  state each one's parity evidence.
  **Measured `2026-10-23`** (`.project-data/tmp/rdx-6/rank2.py`). The driver registers 20 doctrines (the 23 scripts
  `.1` counted include their helpers). Following each one's direct invocations (comment lines aside), its script tree
  runs from one 103-line shell file (`RUSTDOC`) to 61 files and 33,989 lines (`CLAIM-VERIFICATION`):
  - shell, one or two files: `RUSTDOC` 103 lines, `KNOWLEDGE-MAP` 136, `MEMORY-ARCH` 183, `TASK-ACCEPTANCE` 213,
    `README-POLICY` 277;
  - Perl or Python, one file: `SECTION-ANCHORS` 146, `TASK-NODE-RETENTION` 285, `CONTRACT-ENCODING` 323,
    `OWNERSHIP-CITATIONS` 413, `PUBLISHED-ASSERTIONS` 1,175, `RESIDUAL-ACTIONABILITY` 1,190;
  - several files: `CORPUS-FRONTIER` 631, `CONSTRAINT-PART-SPAN` 652, `CHAIN-CURRENCY` 945, `PROOF-SEAL-CURRENCY`
    1,076, `PROOF-SEAL-TOTAL` 1,111, `PROJECT-DATA-LOCALITY` 1,154;
  - composites: `PRODUCTION-GENERICITY` 7 files / 3,367 lines, `LIVE-DOC-SIZE` 26 / 19,699, `CLAIM-VERIFICATION`
    61 / 33,989.
  `KNOWLEDGE-MAP`, `MEMORY-ARCH` and `TASK-ACCEPTANCE` have no self-test; the others do.
  **Decided.**
  - **The unit.** One doctrine per leaf, its whole direct script tree, as `cargo xtask check <doctrine>` (lower
    case). The driver keeps the doctrine's entry; its script becomes a shim that still activates the project-data
    roots (`PROJECT-DATA-LOCALITY`) and then runs the native check.
  - **Parity evidence.** The old script and the native check on the same inputs: the current tree; every self-test
    case, ported as Rust tests with the same verdicts; and at least one deliberately broken input per refusal, with
    identical stdout, stderr and exit code (paths named as the script names them). A doctrine without a self-test
    gains one in its port, its cases proven against the old script first.
  - **The order.** Wave 1, the small shell gates (`.8`-`.12`); wave 2, the single-file Perl and Python gates (`.13`);
    wave 3, the multi-file gates and corpus oracles (`.14`); wave 4, the composites and then the driver itself (`.15`).
    A wave opens one leaf per doctrine when it starts, in the order above.
  - **Dependencies.** From wave 2, a port may use crates the workspace already locks (`serde_json`, `sha2`), never a
    new crate and never a product crate; the `DEPENDENCIES` component changes with the first port that needs one.
  Verification: `ranking in .project-data/tmp/rdx-6/rank2.txt; doctrine gates at commit`
  Commit: `RUSTDOC-AND-XTASK.6 — the order the doctrine gates move to cargo xtask (DESIGN)`

- ID: `RUSTDOC-AND-XTASK.7`
  Status: `pending`
  Goal: documentation coverage is a ratchet
  Acceptance: record each crate's undocumented public item count and refuse an increase at commit; the counts in
  `.1` are the baseline.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.8`
  Status: `done` (`2026-10-23`; CODE; wave 1; checklist below)
  Goal: `cargo xtask check rustdoc` replaces `scripts/check_rustdoc.sh`
  Acceptance: `.6`'s parity evidence; the script a shim.
  **Done `2026-10-23`.** `cargo xtask check <doctrine>` dispatches the ported checks (`xtask/src/checks/`, a
  `Runner::Checks` command whose table names each check's replaced script); `rustdoc` is the first. It activates the
  project-data roots, writes the scratch crate under `TMPDIR`, requires its `cargo doc` to fail with *unresolved
  link*, then documents the workspace under `-D warnings`, printing the script's lines and exit codes. The shim keeps
  the doctrine's registered path, the project-data activation and its own argument refusal (so the usage line names
  it as the driver invoked it), then runs `exec cargo xtask check rustdoc`. **Parity** with HEAD's script, through
  the driver's absolute-path invocation: the current tree (0), an argument (2) and the `[CLK_I]` link restored (1),
  each with identical stdout, stderr and exit code.
  Verification: `cargo test -p xtask --lib` 10/10; parity in `.project-data/tmp/rdx-8/`
  Commit: `RUSTDOC-AND-XTASK.8 — cargo xtask check rustdoc replaces the RUSTDOC gate's script`

- ID: `RUSTDOC-AND-XTASK.9`
  Status: `pending` (CODE; wave 1)
  Goal: `cargo xtask check knowledge-map` replaces `knowledge-map/scripts/check_knowledge_map.sh`
  Acceptance: `.6`'s parity evidence, with a new self-test; the script a shim.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.10`
  Status: `pending` (CODE; wave 1)
  Goal: `cargo xtask check memory-arch` replaces `scripts/check_memory_architecture.sh`
  Acceptance: `.6`'s parity evidence, with a new self-test; the script a shim.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.11`
  Status: `pending` (CODE; wave 1)
  Goal: `cargo xtask check task-acceptance` replaces `scripts/check_task_acceptance.sh`
  Acceptance: `.6`'s parity evidence, with a new self-test; the script a shim.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.12`
  Status: `pending` (CODE; wave 1)
  Goal: `cargo xtask check readme-policy` replaces `scripts/check_readme_policy.sh`
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.13`
  Status: `pending` (container; wave 2)
  Goal: the single-file Perl and Python gates move to xtask
  Acceptance: one leaf per doctrine, in order: `SECTION-ANCHORS`, `TASK-NODE-RETENTION`, `CONTRACT-ENCODING`,
  `OWNERSHIP-CITATIONS`, `PUBLISHED-ASSERTIONS`, `RESIDUAL-ACTIONABILITY`; the `DEPENDENCIES` change for locked
  crates with the first that needs one.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.14`
  Status: `pending` (container; wave 3)
  Goal: the multi-file gates and corpus oracles move to xtask
  Acceptance: one leaf per doctrine, in order: `CORPUS-FRONTIER`, `CONSTRAINT-PART-SPAN`, `PROJECT-DATA-LOCALITY`,
  `CHAIN-CURRENCY`, `PROOF-SEAL-CURRENCY`, `PROOF-SEAL-TOTAL`.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15`
  Status: `pending` (container; wave 4)
  Goal: the composite gates and the driver move to xtask
  Acceptance: one leaf per doctrine, in order: `PRODUCTION-GENERICITY`, `LIVE-DOC-SIZE`, `CLAIM-VERIFICATION`; then
  `check_doctrines.sh` itself.
  Verification: `pending`
  Commit: `pending`

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.2` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — `RUSTDOCFLAGS="-D warnings" cargo doc --offline --workspace --no-deps` failed:
  `unresolved link to CLK_I` in `crates/specforge/src/ir/evidence.rs`, the workspace's only rustdoc warning (`.1`).
- [x] **ROOT CAUSE (WHY + WHERE)** — a doc comment quoting Wishbone's *rising [CLK_I] edge*
  (`crates/specforge/src/ir/evidence.rs:206`, written by `EXTRACTION-REPRESENTATION-GAPS.17`) reads as an intra-doc
  link, and only push-time CI ran rustdoc. RED: the
  bracket restored makes `scripts/check_rustdoc.sh` fail on `specforge-core`; the file restored byte-identically.
- [x] **ADDRESSED (verified)** — workspace rustdoc warnings 1 -> 0; the `RUSTDOC` gate passes.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current (a doc comment moves no ruleset hash);
  `check_production_genericity_flow.sh` unchanged; `cargo fmt --check` and `cargo clippy --offline --workspace
  --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 /
  1,943 passed.
- [x] **GENERICITY (ADR 0006)** — a doc-comment escape and a build gate; no extraction behaviour changes.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md` and the book's `reference/doctrine-enforcement.md` list `RUSTDOC`;
  KM card `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.4` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — `cargo xtask` did not exist; the tooling ran only as `bash scripts/...` (`.1`'s
  inventory: 123 scripts, 23 of them the doctrine driver's).
- [x] **ROOT CAUSE (WHY + WHERE)** — no Rust entry point: `Cargo.toml` had no `xtask` member and
  `.cargo/config.toml` no alias. RED 3/3 on `xtask/src/lib.rs`, restored byte-identically: a signal reported as its
  raw code, a command naming a missing script, an existing command not found; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — commands 0 -> 3, each byte-identical to its script on the runs above.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean (with `xtask` in the workspace); `cargo test --workspace --lib --exclude specforge-production-graph` 475 /
  171 / 1,943 and `xtask` 3 passed; `check_doctrines.sh --only PRODUCTION-GENERICITY` PASS, its flow census
  unchanged; no product code changed, so no chain moved.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — book `getting-started.md` (*Local verification*); KM card
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.5` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — `cargo xtask docs` ran `scripts/run_docs_ci.sh` (`.4`); no command was native.
- [x] **ROOT CAUSE (WHY + WHERE)** — the docs build lived only in shell (`scripts/run_docs_ci.sh`, sourcing
  `scripts/project_data_env.sh`). RED 4/4, every file restored byte-identically: a non-executable file found on
  `PATH`, a nested directory not copied, a non-repository accepted, the shim no longer running the native command;
  each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — native commands 0 -> 1, byte-identical to the script on success and on the
  missing-`mdbook` path.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 7 passed;
  `check_doctrines.sh --only PRODUCTION-GENERICITY,RUSTDOC,PROJECT-DATA-LOCALITY` PASS; the shim `bash
  scripts/run_docs_ci.sh` passes.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — book `getting-started.md` and `reference/api-reference.md`; KM card
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.8` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `RUSTDOC` doctrine ran only as shell (`scripts/check_rustdoc.sh`, `.2`).
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask` had no `check` command. RED 4/4, every file restored
  byte-identically: the self-test crate linking to a real item, the doctrine's `-D warnings` dropped, an unknown
  check accepted, the shim no longer running the native check; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 0 -> 1, byte-identical to the script on three inputs.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 10
  passed; `check_doctrines.sh --gate` PASS at commit (the `RUSTDOC` entry now runs through its shim).
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md`'s `RUSTDOC` row; KM card
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `RUSTDOC-AND-XTASK.9`-`.12` | `pending` | wave 1, in order |
| 2 | `RUSTDOC-AND-XTASK.7` | `pending` | the coverage ratchet |
| 3 | `RUSTDOC-AND-XTASK.13`-`.15` | `pending` | waves 2-4 |

## Decisions

- `2026-10-23`: the director asked for the rustdoc API reference and an incremental move to `cargo xtask`; `.1`
  records the measured shape and the rules each later leaf follows.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-10-23` | `.1` | rustdoc build under `-D warnings`; `missing_docs` per crate; scripts inventory | fails on one link; counts and inventory recorded |
| `2026-10-23` | `.2` | `RUSTDOC` doctrine; RED with the link restored; oracles | PASS; RED fails as it must |
| `2026-10-23` | `.3` | `run_docs_ci.sh`; the built page's API links | PASS; four links resolve |
| `2026-10-23` | `.4` | xtask tests; dispatcher parity with the scripts | 3/3; identical output and exit codes |
| `2026-10-23` | `.5` | xtask tests; native docs parity with HEAD's script | 7/7; identical on success and with mdbook absent |
| `2026-10-23` | `.6` | the 20 doctrines' direct script trees | ranked; waves set |
| `2026-10-23` | `.8` | xtask tests; RUSTDOC parity through the driver's invocation | 10/10; identical on three inputs |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.1` | `RUSTDOC-AND-XTASK.1 — measure rustdoc and the scripts, decide the API reference and xtask (DESIGN)` | measurement and decisions |
| `.2` | `RUSTDOC-AND-XTASK.2 — rustdoc builds warning-free, and a commit that breaks it is refused` | the link escaped; the `RUSTDOC` gate |
| `.3` | `RUSTDOC-AND-XTASK.3 — the API reference is published beside the mdBook` | the docs build and the book chapter |
| `.4` | `RUSTDOC-AND-XTASK.4 — cargo xtask dispatches to the current tooling` | the `xtask` crate and alias |
| `.5` | `RUSTDOC-AND-XTASK.5 — cargo xtask docs builds the book and the API reference natively` | the first native command; the docs script a shim |
| `.6` | `RUSTDOC-AND-XTASK.6 — the order the doctrine gates move to cargo xtask (DESIGN)` | four waves; parity rules |
| `.8` | `RUSTDOC-AND-XTASK.8 — cargo xtask check rustdoc replaces the RUSTDOC gate's script` | the first ported gate |

## Changelog

- `2026-10-23`: Created task tree.
