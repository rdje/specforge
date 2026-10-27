# RUSTDOC-AND-XTASK: publish the rustdoc API reference, and move the repository's tooling to cargo xtask

## Metadata

- Tree ID: `RUSTDOC-AND-XTASK`
- Status: `active`
- Roadmap lane: repository durability and portability
- Created: `2026-10-23`
- Last updated: `2026-10-27`
- Owner: repo-local workflow

## Goal

SpecForge publishes a rustdoc API reference of its workspace crates beside the mdBook, kept warning-free at
commit cadence, and its repository tooling moves to `cargo xtask` one command at a time: a Rust entry point that
adds no crate to the build, with each port proven to behave exactly as the script it replaces.

## Non-Goals

- Rewriting historical records. Verification logs, commit logs and closed leaves cite the scripts they ran; they
  stay as written, and a ported script stays as a shim that forwards to `cargo xtask`.
- A big-bang port. Each gate moves in its own leaf, with parity evidence, or not at all.
- New crates. `xtask` uses the standard library and `sha2` (since `.13.e`; `serde_json` from `.13.b` to `.17`, when
  `pyjson` replaced it), both locked for the product, so it builds offline and brings nothing new into the build.
- Porting a portable architecture's kit (`.16`): the scripts other repositories copy stay in their portable language.
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
  Goal: the rustdoc API reference, and the incremental move to `cargo xtask` of SpecForge's own tooling
  Children: `RUSTDOC-AND-XTASK.1`-`.19`

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
    compiles the `ir` sources through `#[path]`, so it is the library API. **Corrected by `.7`:** `specforge` has 39
    and `specforge-conformance` 758; each `-p` run also linted the workspace crates the package depends on, and the
    text grep summed them (2,922 = 39 + 758 + 2,125).
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
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: documentation coverage is a ratchet
  Acceptance: record each crate's undocumented public item count and refuse an increase at commit; the counts in
  `.1` are the baseline.
  **Done `2026-10-23`.** **Measured.** rustdoc runs the `missing_docs` lint, and cargo replays a fresh crate's
  rustdoc warnings (`.project-data/tmp/rdx-7/probe`), so the `RUSTDOC` gate's own build can count coverage at no
  extra cost; under `-D warnings` the count is impossible, since `missing_docs` then becomes an error whatever the
  flag order. Per package (`package_id`), the gate's build finds `specforge-core` 2,124, `specforge-conformance`
  758, `specforge` 39, `specforge-production-graph` 33, `xtask` 12. **`.1`'s baseline was wrong** for two crates
  (2,922 and 2,883): a per-package clippy run also lints the workspace crates it depends on (`specforge` = 39 + 758 +
  2,125), and the text grep summed them; corrected in `.1` and the KM card. **Decided.** The check runs `cargo doc
  --message-format=json` under `-W missing_docs`, refuses every other diagnostic (the old `-D warnings` verdict),
  and requires each package's count to equal its ceiling in `doctrine/rustdoc/coverage_ceilings.tsv` (an unlisted
  package has 0): above it the new item must be documented, below it the ceiling comes down in the same commit, so
  slack is never banked. `xtask`'s 12 items are documented here, so it starts at 0. The self-test crate gains one
  undocumented item and must count it once, beside the unresolved link it must report.
  Verification: `cargo test -p xtask --lib` 20/20; `scripts/check_rustdoc.sh` PASS; RED in `.project-data/tmp/rdx-7/`
  Commit: `RUSTDOC-AND-XTASK.7 — documentation coverage is a ratchet in the RUSTDOC gate`

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
  Status: `done` (`2026-10-23`; not ported: retired by `.16`)
  Goal: `cargo xtask check knowledge-map` replaces `knowledge-map/scripts/check_knowledge_map.sh`
  Acceptance: `.6`'s parity evidence, with a new self-test; the script a shim.
  **Retired by `.16`.** The knowledge-map bundle is a portable architecture's kit, copied into any repository as
  Bash 3.2 and POSIX awk (`knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md`), so its check stays shell and
  `cargo xtask doctrines` only runs it.
  Verification: `.16`'s classification
  Commit: `RUSTDOC-AND-XTASK.16 — a portable architecture's kit stays portable (DESIGN)`

- ID: `RUSTDOC-AND-XTASK.10`
  Status: `done` (`2026-10-23`; not ported: retired by `.16`)
  Goal: `cargo xtask check memory-arch` replaces `scripts/check_memory_architecture.sh`
  Acceptance: `.6`'s parity evidence, with a new self-test; the script a shim.
  **Retired by `.16`.** `MEMORY_ARCHITECTURE.md` §9 ships this script as the memory architecture's enforcement for
  any repository, so it stays shell.
  Verification: `.16`'s classification
  Commit: `RUSTDOC-AND-XTASK.16 — a portable architecture's kit stays portable (DESIGN)`

- ID: `RUSTDOC-AND-XTASK.11`
  Status: `done` (`2026-10-23`; not ported: retired by `.16`)
  Goal: `cargo xtask check task-acceptance` replaces `scripts/check_task_acceptance.sh`
  Acceptance: `.6`'s parity evidence, with a new self-test; the script a shim.
  **Retired by `.16`.** `DOCTRINE_ENFORCEMENT.md` §8 lists this script in Group A, the core a project copies
  verbatim, so it stays shell.
  Verification: `.16`'s classification
  Commit: `RUSTDOC-AND-XTASK.16 — a portable architecture's kit stays portable (DESIGN)`

- ID: `RUSTDOC-AND-XTASK.12`
  Status: `done` (`2026-10-23`; CODE; wave 1; checklist below)
  Goal: `cargo xtask check readme-policy` replaces `scripts/check_readme_policy.sh`
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  **Done `2026-10-23`.** `xtask/src/checks/readme_policy.rs` checks the README's two ceilings, the policy's fenced
  adoption note, and the route registry (header, three ceilings, every row, the README's reader links, the seven
  author-overflow routes), printing the script's lines and exit codes; `--self-test` runs its 13 cases on the same
  functions the check uses. The shim keeps the registered path, its own argument refusal and the project-data
  activation, then runs `exec cargo xtask check readme-policy "$@"`. **Parity** with HEAD's script through the
  driver's absolute-path invocation, on 33 inputs: identical stdout, stderr and exit code on 28 (the current tree,
  the self-test, two argument refusals, 24 broken inputs covering its 23 refusal messages). **Two fail-opens, fixed by
  the port:** the script's `while read` loop dropped a last registry row without its newline unchecked, and its
  `IFS=$'\t'` merged runs of tabs, so a row with an empty extra column or a leading or trailing tab read as
  complete; those 4 inputs exit 0 under the script and 1 under the port. The fifth difference: on a missing
  registry the script's self-test leaked awk's error to stderr. **Claim gate:** `claim-verification-contract-published`
  pinned its RED case at lines 113-115 of the script, which now live in Rust, and the gate required a case region
  to lie in the control's producer; a region may now name the file its producer delegates to (`source_region.path`,
  which must be one of the control's inputs), in `scripts/check_claim_verification.pl` (self-test 32 -> 34),
  `scripts/repin_claim_regions.py` (37 -> 38) and `CLAIM_VERIFICATION.md`.
  Verification: `cargo test -p xtask --lib` 15/15; parity and RED in `.project-data/tmp/rdx-12/`
  Commit: `RUSTDOC-AND-XTASK.12 — cargo xtask check readme-policy replaces the README-POLICY gate's script`

- ID: `RUSTDOC-AND-XTASK.13`
  Status: `done` (`2026-10-23`; container; wave 2; `.13.a`-`.13.f` done)
  Goal: the single-file Perl and Python gates move to xtask
  Acceptance: one leaf per doctrine, in order: `SECTION-ANCHORS`, `TASK-NODE-RETENTION`, `CONTRACT-ENCODING`,
  `OWNERSHIP-CITATIONS`, `PUBLISHED-ASSERTIONS`, `RESIDUAL-ACTIONABILITY`; the `DEPENDENCIES` change for locked
  crates with the first that needs one.
  **Opened `2026-10-23` with `.13.a`.** None of the six is a portable kit's script (`.16`'s classification,
  re-checked against the five standards). A Perl or Python gate's shim stays at its path and in its language, since
  callers run it with `perl` or `python3`; it changes to the repository root, where the `cargo xtask` alias
  resolves, after making any path argument absolute, then `exec`s the native check. **xtask is optimized in the dev
  profile** (`[profile.dev.package.xtask]`, `.13.a`): unoptimized, a scan of the tree's 24.7 MB of Markdown took
  0.88 s against Perl's 0.11 s. **`DEPENDENCIES` changed with `.13.b`**, the first port to read JSON: xtask may depend
  only on `serde_json` or `sha2`, both already locked for the product, and never on a product crate (self-test 9 ->
  11). Since `.17` it needs neither: JSON is read and written through `xtask/src/pyjson.rs`, a reproduction of
  Python's own `json` module, so a port's output matches the script's even where Python's wording shows.
  Verification: per child
  Commit: per child

- ID: `RUSTDOC-AND-XTASK.13.a`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask check section-anchors` replaces `scripts/check_section_anchors.pl`
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  **Done `2026-10-23`.** `xtask/src/checks/section_anchors.rs` lists the tracked Markdown, counts every `§` before a
  backticked section that does not follow a backtick, and resolves each qualified reference (a backticked `<path>.md`,
  `§`, a backticked section) by normalized containment in the target's headings, read once per target; `--check`,
  `--report`, `--self-test` and `--root DIR` as before. The Perl shim makes a `--root` absolute, enters the root and
  runs `exec { 'cargo' } qw(cargo xtask check section-anchors)`. **Parity** with HEAD's script through the driver's
  absolute-path invocation, on 18 inputs (the current tree's 42 resolved and 75 bare references, every argument form,
  a relative `--root`, nine fixture repositories with a broken input per refusal and CRLF, NBSP, invalid UTF-8,
  seven-`#`, directory and escaping targets): identical stdout, stderr and exit code on 16. The other two differ only
  in exit code: the script's `die` exited with whatever `errno` held (25, `ENOTTY`, for a usage error), the port exits
  2. **Cost:** 0.11 s for the script; 0.25 s for the shim (cargo's 0.15 s and the optimized check), after a 16 s
  rebuild when xtask changes.
  Verification: `cargo test -p xtask --lib` 27/27; parity and RED in `.project-data/tmp/rdx-13a/`
  Commit: `RUSTDOC-AND-XTASK.13.a — cargo xtask check section-anchors replaces the SECTION-ANCHORS gate's script`

- ID: `RUSTDOC-AND-XTASK.13.b`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask check task-node-retention` replaces `scripts/check_task_node_retention.py`
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  **Done `2026-10-23`.** `xtask/src/checks/task_node_retention.rs` reads `HEAD`'s node ids through `git grep`, the
  working tree's from the three task roots, and the removals registry, reproducing Python where its output shows:
  the multi-line id pattern (an id may run past a newline), text-mode line reading, `repr` of lists and values,
  `str` of the declared total and Python's `==` against it (a string `"1"` is not 1, `1.0` and `true` are), and
  `json.dump(indent=2, sort_keys=True)` with non-ASCII escaped for `--report`; its six self-test cases keep their
  verdicts. The Python shim keeps the `argparse` front end and the module docstring, so help, abbreviations such as
  `--self` and refusals are unchanged, then `exec`s the native check with one mode. **Parity** with HEAD's script
  through the driver's absolute-path invocation, on 27 inputs (the current tree's 1,701 nodes, nine argument forms
  including `-h`, a deletion, two deletions under `--report`, a split, a longer sibling, an id spanning a newline,
  a declared removal, and every registry refusal): identical stdout, stderr and exit code on 26. The other is the
  wording inside an invalid-JSON refusal, serde's `expected value at line 1 column 17` for Python's `Expecting
  value: line 1 column 17 (char 16)`; the refusal, line and column agree. A non-UTF-8 task file or a non-object
  registry line, which crash the script with a traceback, are refusals naming the file or line. **Cost:** 0.14 s
  for the script, 0.16 s for the shim. **`.17` closes the one difference:** read through `pyjson`, all 27 inputs are
  identical (`.project-data/tmp/rdx-13b/new17/`).
  Verification: `cargo test -p xtask --lib` 34/34; parity and RED in `.project-data/tmp/rdx-13b/`
  Commit: `RUSTDOC-AND-XTASK.13.b — cargo xtask check task-node-retention replaces the TASK-NODE-RETENTION gate's
  script`

- ID: `RUSTDOC-AND-XTASK.13.c`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask check contract-encoding` replaces `scripts/check_contract_encoding.py`
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  **Done `2026-10-23`.** `xtask/src/checks/contract_encoding.rs`, on `.17`'s `pyjson`: each contract under `doctrine/`
  is matched against the script's 80 `json.dumps` option sets in its order, the registry read and `--write`
  re-derived, the findings and refusals worded as before, and `--report`'s churn table read from git history (the
  last commit to touch each contract, its changed lines, its changed leaf fields under Python's `==`); its seven
  self-test cases run on a scratch tree under `generated/`. The Python shim keeps the `argparse` front end and passes
  the flags through. **Parity** with HEAD's script through the driver's absolute-path invocation, on 23 inputs (the
  current tree's 32 contracts, nine argument forms including `--write`, `--write --report` and `--wr`, every
  contract and registry refusal): identical stdout, stderr and exit code on all 23, and `--write` writes the
  registry byte for byte as the script does. **Cost:** 0.09 s for the script, 0.22 s for the shim.
  **Found while documenting it:** of the 20 doctrines the driver registers, `docs/DOCTRINE_INSTANCE.md` had no row
  for `CONTRACT-ENCODING`, and the book's `reference/doctrine-enforcement.md` none for five (`CONTRACT-ENCODING`,
  `SECTION-ANCHORS`, `TASK-NODE-RETENTION`, `OWNERSHIP-CITATIONS`, `CONSTRAINT-PART-SPAN`); the rows are added here,
  and `.18` makes the coverage mechanical.
  Verification: `cargo test -p xtask --lib` 43/43; parity and RED in `.project-data/tmp/rdx-13c/`
  Commit: `RUSTDOC-AND-XTASK.13.c — cargo xtask check contract-encoding replaces the CONTRACT-ENCODING gate's
  script`

- ID: `RUSTDOC-AND-XTASK.13.d`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask check ownership-citations` replaces `scripts/check_ownership_citations.pl`
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  **Done `2026-10-23`.** `xtask/src/checks/ownership_citations.rs` reads the citation registry (as Perl's `JSON::PP`
  reads it: `pyjson::loads_strict`, the JSON both accept, measured to refuse only Python's non-finite literals and
  lone surrogates), extracts each region's links and backticked ids as the script's two regexes do, resolves each
  current owner's status in either leaf grammar across a partitioned tree's parts, and prints fields as Perl
  stringifies them (booleans `1`/`0`, numbers `%.15g`); its 16 self-test cases keep their verdicts. The Perl shim
  keeps the argument loop, its output layers and the root resolution, so a usage error, a missing root and their
  exit codes (`die` exits with the `errno` those layers leave, 25) are the script's own, then passes `--root`,
  `--registry` and the mode on. **Parity** with HEAD's script through the driver's absolute-path invocation, on 36
  inputs (the current tree's 109 citations in 4 regions, eleven argument forms, 24 fixture roots with a broken
  input per refusal): identical stdout, stderr and exit code on 35. The other names a citation whose `unit` is a JSON
  array: Perl prints the array's address (`ARRAY(0x78592704c8)`, different on every run), the port an empty name.
  **Cost:** 0.09 s for the script, 0.22 s for the shim.
  Verification: `cargo test -p xtask --lib` 50/50; the pyjson corpus 14,964/14,964; parity and RED in
  `.project-data/tmp/rdx-13d/`
  Commit: `RUSTDOC-AND-XTASK.13.d — cargo xtask check ownership-citations replaces the OWNERSHIP-CITATIONS gate's
  script`

- ID: `RUSTDOC-AND-XTASK.13.e`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask check published-assertions` replaces `scripts/check_published_assertions.pl`
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  **Done `2026-10-23`.** `xtask/src/checks/published_assertions.rs` validates the bounded registry, re-executes each
  `derived` producer and membership enumerator and resolves each `dated` revision, checks each exact region
  (`sha2`, already locked and admitted by `DEPENDENCIES`), derives the governed population from the live-document
  surface registry, and scans each claim-tagged paragraph with the script's numeral grammar, its backtracking
  reproduced (`1,9224` publishes `1` and `9224`, `5%a` publishes `5`). Perl's view of values comes from a shared
  `xtask/src/perl.rs`: `%.15g` numbers, booleans as references, truthiness, and `encode_utf8`'s double count of each
  raw non-ASCII byte. Its 25 self-test cases run on the same fixture repository the script built and keep their
  verdicts. The Perl shim keeps the argument loop, its usage line and exit 2, and the root resolution. **Parity**:
  HEAD's script beside the shim on 23 inputs (the current tree's 33 assertions, `--report`, `--produce`, the
  self-test, four argument forms, and fourteen broken inputs from the real registry and two documents, among them
  an unlisted value in `TOOLBOX.md` and a phase switched to `inventory` so `--produce` lists one), identical stdout,
  stderr and exit code on all 23. **Cost:** 0.59 s for the script, 0.68 s for the shim (both re-run every
  producer).
  Verification: `cargo test -p xtask --lib` 61/61; parity and RED in `.project-data/tmp/rdx-13e/`
  Commit: `RUSTDOC-AND-XTASK.13.e — cargo xtask check published-assertions replaces the PUBLISHED-ASSERTIONS gate's
  script`

- ID: `RUSTDOC-AND-XTASK.13.f`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask check residual-actionability` replaces
  `scripts/validate_residual_actionability_contract.py`
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  **Done `2026-10-23`.** `xtask/src/checks/residual_actionability.rs` executes the required-residual rule over the
  contract's 18-case matrix, re-derives the witness's partition from the tracked current result, binds the
  reproduction command to its producer and published authorities, pins the retained chain set by its exact digest,
  and holds every reviewed residual gold to the region law or a declared fact-scoped carrier. Values compare,
  count, hash and print as Python's do (`pyjson`: `==` across `int`, `float` and `bool`, `Counter` arithmetic, `str`
  and `repr` of lists and tuples). Its self-test runs the script's 40 RED mutations of the real contract and
  authorities and its one admissible operation; a unit test also pins the rule's duplicate branch for a residual
  cell, which the contract's own matrix leaves unexercised (Python agrees: `(1, 0)`). The Python shim keeps the
  `argparse` front end. **Parity**: HEAD's script beside the shim on 31 inputs (the current tree, the self-test, four
  argument forms, 25 broken inputs written into the real contract and its three authorities): identical stdout,
  stderr and exit code on all 31, the three `argparse` cases compared under the script's real name, since
  `argparse` wraps its usage line by program-name length. Where the script would stop with a traceback, the port
  reports a refusal; where it iterates a hash-ordered `set`, the port reports in sorted order. **Cost:** 0.05 s for
  the script, 0.12 s for the shim.
  Verification: `cargo test -p xtask --lib` 65/65; parity and RED in `.project-data/tmp/rdx-13f/`
  Commit: `RUSTDOC-AND-XTASK.13.f — cargo xtask check residual-actionability replaces the RESIDUAL-ACTIONABILITY
  gate's script`

- ID: `RUSTDOC-AND-XTASK.14`
  Status: `done` (`2026-10-24`; container; wave 3; `.14.a`-`.14.g` done)
  Goal: the multi-file gates and corpus oracles move to xtask
  Acceptance: one leaf per doctrine, in order: `CORPUS-FRONTIER`, `CONSTRAINT-PART-SPAN`, `PROJECT-DATA-LOCALITY`,
  `CHAIN-CURRENCY`, `PROOF-SEAL-CURRENCY`, `PROOF-SEAL-TOTAL`. **Known consumer (`.13.e`):**
  `scripts/report_self_test_totals.pl` reads the declared self-test totals of `check_chain_currency.sh`,
  `check_proof_seal_currency.sh` and `rebuild_stage_cascade.sh` from their source and fails closed when it cannot,
  so a port that moves a total out of its script updates that producer in the same leaf. **Added by `.14.c`:**
  `.14.g`, the persisted-artifact path tool `PROJECT-DATA-LOCALITY` composes.
  **Opened `2026-10-23` with `.14.a`.** None of the six is a portable kit's script (`.16`). A doctrine whose adapter
  calls a core script ports both into one module; each script stays a shim at its path, since the records cite the
  core and the driver runs the adapter.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.14.a`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask check corpus-frontier` replaces `scripts/check_corpus_frontier.sh` and its core
  `scripts/check_corpus_frontier_census.pl`
  Acceptance: `.6`'s parity evidence, its self-test ported; both scripts shims.
  **Done `2026-10-23`.** `xtask/src/checks/corpus_frontier.rs` derives the cohort from each persisted SourceIR's
  `source` object (sliced from the file's first 8 KiB, the window growing to 4 MiB, by a brace scan that skips
  strings, then read as `JSON::PP` reads it), diffs it against the declared refreshed and remaining sets, the
  retained bundles and the root task's sentences, and prints `--report` as `JSON::PP`'s canonical pretty output
  (indent 3, ` : `, sorted keys). Its 13 self-test cases keep their verdicts. On this repository the adapter runs the
  self-test, then the census; on another root it runs that root's own core, as the script did. The shell shim keeps
  the usage refusal and the project-data activation; the Perl shim keeps the argument loop, the usage line and exit
  2, the `SPECFORGE_CORPUS_FRONTIER_CONTRACT` default and the root resolution, and passes the directory it was run
  from, so a relative `--root` or contract resolves where the script resolved it. **Parity**: HEAD's two scripts
  beside the shims on 49 inputs (the real corpus, `57 cohort = 52 refreshed + 5 remaining`; fifteen argument and
  environment forms; 17 fixture trees, each with one broken input per refusal, through both entry points), identical
  stdout, stderr and exit code on all 49, and on a relative root and refusal run from another directory. **Cost:**
  0.15 s for the script, 0.18 s for the shim; `--report` 0.04 s and 0.15 s.
  Verification: `cargo test -p xtask --lib` 67/67; parity and RED in `.project-data/tmp/rdx-14a/`
  Commit: `RUSTDOC-AND-XTASK.14.a — cargo xtask check corpus-frontier replaces the CORPUS-FRONTIER gate's scripts`

- ID: `RUSTDOC-AND-XTASK.14.b`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask check constraint-part-span` replaces `scripts/check_constraint_part_span.sh` and its census
  `scripts/measure_constraint_part_span.py`
  Acceptance: `.6`'s parity evidence, its self-test ported; both scripts shims.
  **Done `2026-10-23`.** `xtask/src/checks/constraint_part_span.rs` scans `crates/specforge/src/ir/evidence.rs` as the
  script did (its definition regex, the `#[cfg(test)]` item bounds, the span read from each call's argument and the
  `let lowered` binding before it, Python's white space and universal newlines) and holds the topology to its
  declaration: three producers through the typed gateway, one of them over the whole statement, and the gateway the
  only way into the untyped arm. The census mirrors the extractor's grammar (clause split, the six kind arms in
  order, the value binder, negators, reference magnitudes) over the 78 persisted `evidence_ir.json`, read through
  `pyjson`, and prints the report and `--json` as the script did; its 10 mirror cases keep their verdicts. Two unit
  tests add what the script's self-test never covered: the check's refusals on a fixture source (a new producer, a
  second way into the untyped arm, a test helper's call admitted) and the census's strata. The shell shim keeps the
  usage refusal and the project-data activation; the Python shim keeps the `argparse` front end and passes
  `--measure` with the flags set. **Parity**: HEAD's two scripts beside the shims on 68 inputs (the real tree through
  both registered paths in eleven argument forms; 19 fixture roots, each with HEAD's census copied in, in three
  modes: a census corpus with every stratum, unknown prefixes, non-ASCII case mappings, Python-only white space and a
  clause past 200 characters; eight topology moves; two unreadable sources; seven broken artifacts): 48 identical in
  full; 18 identical in stdout and exit code where the script stops with a traceback and the port names the input;
  and two by design: `--help`, whose description no longer says the mirror is written in Python, and a two-member
  set the port prints sorted where Python follows string hashes. **Cost:** the gate 0.18 s for the scripts, 0.19 s
  for the shim; the census 2.37 s and 2.04 s.
  **Found:** the census's KM card `constraint-record-producer-strata` still described four producers and two call
  sites, and its reverify expected `2 callers`, true before `.3k.2a`'s gateway and `EXTRACTION-GAP-FIX.5b`'s keyed-row
  producer; re-derived here from the port's census (364 records: 21 differ by span, 6 in the classifier's reach).
  Verification: `cargo test -p xtask --lib` 72/72; parity and RED in `.project-data/tmp/rdx-14b/`
  Commit: `RUSTDOC-AND-XTASK.14.b — cargo xtask check constraint-part-span replaces the CONSTRAINT-PART-SPAN gate's
  scripts`

- ID: `RUSTDOC-AND-XTASK.14.c`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: `cargo xtask check project-data-locality` replaces `scripts/check_project_data_locality.sh`
  Acceptance: `.6`'s parity evidence; the script a shim; `scripts/project_data_env.sh` stays the shared shell
  library the other scripts source.
  **Done `2026-10-23`.** `xtask/src/checks/project_data_locality.rs` activates the roots through `project_data`,
  requires each seam's contract text in the script's order (`.cargo/config.toml`'s ten forced variables, every
  `scripts/*.sh` entry point's activation, the native commands, both hooks, the product's twelve temporary and
  subprocess seams), checks the two locks and any present Python environment's prefix and launchers, refuses
  `.fsm`/`.log` residue and a user-home path in the post-compact hook, and runs the two tools it composes with the
  activated environment, their output discarded and their errors shown: `check_persisted_artifact_paths.pl`
  (`--self-test`, then `--check`; ported by `.14.g`) and `test_project_data_locality.sh`, which tests the shell
  library and stays shell with it. The shim keeps the argument refusal and the shell activation. `--root DIR`
  checks another tree. A unit test runs the whole gate on a copied tree, one broken input per case.
  **Found: the stale-launcher check never fired.** The script ran `rg` over each environment's `bin/` and
  `pyvenv.cfg` and kept only lines without the root, but every `rg` line begins with its file's absolute path under
  the root, so the filter dropped all of them: a launcher written `#!/Users/someone/old/specforge/.venv-docling/...`
  was matched, then discarded. The port compares each line's own text; on the real environments it finds 69
  matching lines and none stale. The book's claims that the gate detects a stale launcher
  (`reference/troubleshooting.md`, `reference/doctrine-enforcement.md`) were false until now; the latter's "three
  focused shell cases" is five.
  **Parity**: HEAD's script beside the shim on 21 inputs (the real tree and an argument through the registered
  path; 19 fixture roots holding every file the gate reads, HEAD's script inside each: clean, and one break per
  refusal, from a missing `.gitkeep` to a non-repository root): 19 identical, one in stdout and exit code alone (a
  missing hook, where `grep` also printed its own error), and one by design, the stale launcher the script missed.
  **Cost:** about two minutes either way (112 s for the script, 119 s for the shim, one run each), nearly all of it
  `check_persisted_artifact_paths.pl --check` over the persisted corpus.
  Verification: `cargo test -p xtask --lib` 75/75; parity and RED in `.project-data/tmp/rdx-14c/`
  Commit: `RUSTDOC-AND-XTASK.14.c — cargo xtask check project-data-locality replaces the PROJECT-DATA-LOCALITY
  gate's script`

- ID: `RUSTDOC-AND-XTASK.14.d`
  Status: `done` (`2026-10-24`; CODE; checklist below)
  Goal: `cargo xtask check chain-currency` replaces `scripts/check_chain_currency.sh`, and holds the one
  content-identity predicate `scripts/lib/stage_artifact_identity.sh` defined
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim; `report_self_test_totals.pl` still
  reads its declared total; the predicate still defined once for the oracle and the rebuild remedy.
  **Done `2026-10-24`.** `xtask/src/checks/chain_currency.rs` replays every persisted artifact at its persisted
  input with the release build `scripts/lib/corpus_replay_binary.sh` names (asked through that file, so "which
  build answers" stays defined once), extracts the `*_json:` section as the script's `awk` did, and compares content
  identity as `JSON::PP` decoded and canonically re-encoded it: `validation_reports`, `proof_context` and
  `proof_ledger` excluded, numbers compared by Perl's text (`1.0` equals `1`, an integer past twenty characters is a
  string, one past 64 bits a double), sections named `name(n->m)`. It counts unmeasurable, legacy-quarantined,
  orphaned and stale artifacts and checked `.isf` states per stage as the script did, reads each SourceIR's bundle
  line with the script's regex, and holds the retained bundles to their schema-closed declaration; its 22 self-test
  cases keep their verdicts. **The predicate stays single** (`.11`): `scripts/lib/stage_artifact_identity.sh`, which
  `scripts/rebuild_stage_cascade.sh` and the held-out probe source, now calls `--compare-stage-artifact` and
  `--compare-emitted-isf`, passing the caller's directory so a relative path resolves and is named as the caller
  wrote it, and exiting as Perl's `die` did (the failed call's `errno`). `report_self_test_totals.pl` reads the
  declaration from the Rust module (`EXPECTED_SELF_TEST_CASES`, its sixth self-test case). The shim keeps the argument
  loop, `LC_ALL=C`, the knobs and the activation. **Parity**: HEAD's script, library and reporter beside the shims on
  54 inputs (the self-test and three argument forms; the cascade's self-test through the library; the reporter; 23
  crafted pairs and 15 adapter/emission cases through the library; six fixture corpora through the knobs, from a
  current chain to a drifted one, a broken chain and three broken declarations; an absent corpus and a missing
  declaration; and the whole persisted corpus, 78 documents, 31 replayed and current at every stage): 49 identical;
  three in stdout and exit code where Perl died with its own parser or `HASH` message; and two by design, the
  reporter's sixth case and a non-ASCII top-level section name, which Perl printed as a Latin-1 byte and the port as
  UTF-8 (sections are the product's serde field names, so no artifact has one). **Cost** on the persisted corpus:
  933 s for the script, 151 s for the shim; `JSON::PP` decoding the largest artifacts dominated the script.
  Verification: `cargo test -p xtask --lib` 79/79; parity and RED in `.project-data/tmp/rdx-14d/`
  Commit: `RUSTDOC-AND-XTASK.14.d — cargo xtask check chain-currency replaces the CHAIN-CURRENCY gate's script`

- ID: `RUSTDOC-AND-XTASK.14.e`
  Status: `done` (`2026-10-24`; CODE; checklist below)
  Goal: `cargo xtask check proof-seal-currency` replaces `scripts/check_proof_seal_currency.sh`, and holds the one
  seal reader `scripts/lib/proof_seal_scan.sh` defined
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim; `report_self_test_totals.pl` still
  reads its declared total; the seal read still defined once for the gate and the rebuild remedy.
  **Done `2026-10-24`.** `xtask/src/checks/proof_seal_currency.rs` reads each persisted artifact's seal as the
  library did: the necessary-condition prefilter (the `"proof_ledger"` key, a needle carried across 1 MiB reads),
  then the depth-aware scan of the top-level ledger's direct `ruleset_sha256`, string literals consumed whole and
  taken raw, the depth signed as Perl's was, and the batched composition dropping an empty seal as `[ -n "$seal" ]`
  did. It censuses every in-scope artifact at every stage, probes per stage (total at `semantic` and `intent` unless
  `SPECFORGE_PROOF_SEAL_TOTAL_STAGES` says otherwise, one per distinct seal elsewhere, everything under `--total`),
  and classifies each refusal as the script did (the stale-seal diagnostic with its remedy, an absent precondition
  with no verdict, anything else unclassified), the diagnostic flattened and cut to 200 bytes as `tr`, `sed` and
  `cut` left it; its 21 self-test cases keep their verdicts, the end-to-end ones through the registered script with
  recording stubs. **The seal read stays single** (`.11`): `scripts/lib/proof_seal_scan.sh` now calls
  `--proof-ledger-candidates`, `--scan-proof-ledger` and `--sealed-artifacts`, passing the caller's directory; the
  chain stage table and the read-only probe stay defined in that library, which the check reads and calls, and
  which build answers stays `corpus_replay_binary.sh`. `report_self_test_totals.pl` reads the declaration from the
  Rust module. The shim keeps the argument loop, `LC_ALL=C`, the three knobs and the activation. **Parity**: HEAD's
  script, library and reporter beside the shims on 55 inputs (the self-test and three argument forms; the TOTAL
  wrapper's self-test; the cascade's; the reporter; 15 crafted artifacts through `scan_proof_ledger`, from a
  compact ledger to an unterminated literal and a key straddling the first 1 MiB read; both batch readers over all
  of them and over none; the two stratum helpers; nine stub-binary fixture corpora in both tiers, from an accepted
  corpus to a stale seal, an unclassified refusal, an absent input, a terminal stage, a lost seal and a binary that
  is not executable; the stage set emptied and replaced; an absent corpus; and the persisted corpus in both tiers,
  31 in scope and 47 proofless): identical on all 55. **Cost** on the persisted corpus: `--check` 91 s for the
  script and 85 s for the shim, `--total` 141 s and 140 s, the loader probes dominating; the seal read over all 78
  `source_ir.json` is 2.12 s for the Perl library and 0.30 s native.
  Verification: `cargo test -p xtask --lib` 82/82; parity and RED in `.project-data/tmp/rdx-14e/`
  Commit: `RUSTDOC-AND-XTASK.14.e — cargo xtask check proof-seal-currency replaces the PROOF-SEAL-CURRENCY gate's
  script`

- ID: `RUSTDOC-AND-XTASK.14.f`
  Status: `done` (`2026-10-24`; CODE; checklist below)
  Goal: `cargo xtask check proof-seal-total` replaces `scripts/check_proof_seal_total.sh`
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim. **Found by `.14.e`:** the wrapper's
  header says `--self-test` reaches "the same 19 controls"; the owning check declares 21. Correct it with the port.
  **Done `2026-10-24`.** `PROOF-SEAL-TOTAL` has its own registry row only because the driver runs one executable per
  row with no arguments, so its native entry, `run_total` in `xtask/src/checks/proof_seal_currency.rs`, reads the
  first argument as the wrapper did (`--self-test` runs the owning check's self-test; none, `--check` and `--total`
  run the total probe; anything after the first argument is ignored) and calls the owning check directly. The shim
  keeps that dispatch, its usage refusal and the activation; its header no longer carries a control count, which
  is the owning check's declaration (the stale "19" was 21). **Parity**: HEAD's wrapper beside the shim on 9 inputs
  (the self-test, a refusal, every dispatch form including an empty and an ignored extra argument over a stub-binary
  fixture corpus with a divergent document, and the persisted corpus): identical on all 9. **Cost:** 142 s either
  way on the persisted corpus, the probes dominating.
  Verification: `cargo test -p xtask --lib` 83/83; parity and RED in `.project-data/tmp/rdx-14f/`
  Commit: `RUSTDOC-AND-XTASK.14.f — cargo xtask check proof-seal-total replaces the PROOF-SEAL-TOTAL gate's script`

- ID: `RUSTDOC-AND-XTASK.14.g`
  Status: `done` (`2026-10-24`; CODE; opened `2026-10-23` by `.14.c`; checklist below)
  Goal: the persisted-artifact path tool `PROJECT-DATA-LOCALITY` composes,
  `scripts/check_persisted_artifact_paths.pl`, moves to xtask
  Acceptance: `.6`'s parity evidence on all four modes (`--check`, `--report`, `--self-test` with its 18 declared
  assertions, and `--migrate-legacy-root`, dry run and `--execute` on a scratch copy only); the script a shim; the
  native locality gate calls it natively. The book's `reference/project-data-locality.md` says the doctrine runs "12
  scanner self-tests" and "pins eleven producer/consumer files", where the tool declares 18 assertions (12 cases and
  six standalone checks) over 12 contract files: re-derive both and correct the book in the same leaf.
  **Done `2026-10-24`.** `xtask/src/checks/persisted_artifact_paths.rs` reads every JSON file under `generated/` as
  the script did: the twelve producer files and their seams (`persisted_path.rs`'s two callable seams by the script's
  regex), the artifact kinds by path, each line's string members by the script's regex (so a value ends at an escaped
  quote, as Perl's did), each path value decoded with every byte a character, and the external labels each artifact
  kind may carry; past twenty absolute values it reports the count. The migration rewrites a legacy root's path
  prefixes, labels each SourceIR's origin (declared, else repository-owned, else below exactly one authorized external
  root) and each EvidenceIR's after its anchor line, and writes through a sibling temporary. Its 18 self-test
  assertions keep their verdicts. `PROJECT-DATA-LOCALITY` now runs the self-test and the check in-process. The Perl
  shim keeps the argument loop, the output layers (a refusal exits 25, a missing root 2, as before) and the root
  resolution. **Re-derived for the book:** 18 self-test assertions (12 scanner cases, three seam cases, the migration
  rewrite and two origin insertions) and twelve producer files; `reference/project-data-locality.md` said 12 and
  eleven. The corrected line is a new counted candidate, given an `incomplete` region
  (`adjudicate_book_quantity_regions.py --add`; 882 candidate lines, 586 regions). **Parity**: HEAD's script beside
  the shim on 36 inputs (the self-test, the check, the report and the default on the persisted corpus, 5,930 artifacts
  and 3,082,161 path values; six refusals; seven fixture roots for the scan and the contract, from a clean tree to
  Windows and UNC values, an undecodable value, more than twenty absolute values, a symbolic link, broken seams and a
  linked `generated/`, each checked and reported; eleven migrations, dry run and `--execute` on a fresh copy per
  phase, with every refusal): identical stdout, stderr, exit code, and migrated tree on all 36. One behaviour is kept
  for parity: a non-ASCII byte in an offending value prints as Perl printed it, each byte a Latin-1 character.
  **Cost:** the check 94 s for the script, 13 s native; `PROJECT-DATA-LOCALITY` 119 s -> 13 s.
  Verification: `cargo test -p xtask --lib` 86/86; parity and RED in `.project-data/tmp/rdx-14g/`
  Commit: `RUSTDOC-AND-XTASK.14.g — cargo xtask check persisted-artifact-paths replaces the persisted-artifact path
  tool`

- ID: `RUSTDOC-AND-XTASK.15`
  Status: `in_progress` (container; wave 4; sized by `.15.0`)
  Goal: the composite gates move to xtask
  Children: `RUSTDOC-AND-XTASK.15.0`, `.15.a`-`.15.e`, `.15.1`, `.15.f`-`.15.z`
  Acceptance: one leaf per doctrine, in order: `PRODUCTION-GENERICITY`, `LIVE-DOC-SIZE`. Since `.16`,
  `CLAIM-VERIFICATION` and the driver (`check_doctrines.sh`) stay portable kit artifacts and are not ported. Since
  `.15.0`, each composite is a leaf per component script and its orchestrator last.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.0`
  Status: `done` (`2026-10-24`; DESIGN)
  Goal: size wave 4 before porting it
  Acceptance: measure each composite's component tree and its other callers, re-check the kit classification, and
  decide the leaves and their order.
  **Measured `2026-10-24`** (`.project-data/tmp/rdx-15/measure.py`, `measure.txt`): `PRODUCTION-GENERICITY` is a
  188-line shell orchestrator over five components, 3,136 lines: `check_production_genericity_dependencies.pl` 303,
  `_inventory.pl` 292, `_rules.pl` 439, `check_behavioral_genericity_contract.py` 2,086, and
  `check_production_genericity_flow.sh` 16, which only launches the Rust crate `specforge-production-graph` with
  `--check-census`. `LIVE-DOC-SIZE` is a 90-line orchestrator over 20 components, 18,539 lines, from
  `check_task_evidence_contracts.sh` 37 to `check_active_task_evidence.pl` 2,756 and
  `check_rolling_ledger_protocol.pl` 2,579. Two of its components have callers outside it, among them the
  `CLAIM-VERIFICATION` kit: `check_live_document_size.pl` and `check_canonical_collection_catalogs.pl`.
  **Kit classification re-checked:** `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` is an adopted doctrine with a local
  adoption block and a SpecForge implementation, not a kit to copy, so `.16`'s classification of `LIVE-DOC-SIZE` as
  SpecForge's own stands.
  **Decided.**
  - One leaf per component script and the orchestrator last, so the orchestrator moves once and calls every
    component in-process; until then it keeps calling each component's registered path, a shim once ported.
  - `PRODUCTION-GENERICITY` first, being a sixth the size: `.15.a` dependencies, `.15.b` inventory, `.15.c` rules,
    `.15.d` the behavioral contract (its output is a claim's re-derivation:
    `wire-golds-installed-and-evidence-current` expects `7 retained post-boundary and unqualified`), `.15.e` the
    orchestrator, which launches `specforge-production-graph` as `_flow.sh` did, since xtask may not depend on a
    product crate.
  - `LIVE-DOC-SIZE`'s leaves open when `PRODUCTION-GENERICITY` closes, in the same pattern; a component a kit
    calls keeps its path and command line exactly, so the kit is unaffected.
  Verification: `measurement in .project-data/tmp/rdx-15/; doctrine gates at commit`
  Commit: `RUSTDOC-AND-XTASK.15.0 — size wave 4 before porting it (DESIGN)`

- ID: `RUSTDOC-AND-XTASK.15.a`
  Status: `done` (`2026-10-24`; CODE; checklist below)
  Goal: `scripts/check_production_genericity_dependencies.pl` moves to xtask
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  **Done `2026-10-24`.** `xtask/src/checks/production_genericity_dependencies.rs` reads the workspace's members and
  each manifest's `[dependencies]` names with the script's patterns (the table's body begins after the last newline
  in the white space after its header and ends at the next line opening a table; a member is each match of
  `/"([^"]+)"/g`), holds the product's package direction, the graph tool's and xtask's disconnection, xtask's
  already-locked crates, and the core root and IR registry free of oracle modules, and reports in the script's
  order and words; a file it cannot read ends it with the `errno` Perl's `die` left. Its self-test writes the same
  fixture under `generated/` and keeps all 11 verdicts. The Perl shim keeps `Getopt::Long` (so `--root=DIR`,
  `-self-test` and `--self` read as before, and a stray option exits 255) and the root resolution. **Parity**: HEAD's
  script beside the shim on 29 inputs (ten argument forms; 19 fixture roots, one per rule, among them an aliased
  reverse dependency, new crates in xtask, a duplicated member, an oracle module in the registry, a `[dependencies]`
  header at the end of a file and a missing manifest): identical on all 29. Kept for parity: a members list with an
  empty `""` misreads, the regex pairing its second quote with the next string; cargo refuses an empty member, so no
  buildable workspace reaches it. **Cost:** 0.02 s for the script, 0.10 s for the shim.
  Verification: `cargo test -p xtask --lib` 88/88; parity and RED in `.project-data/tmp/rdx-15a/`
  Commit: `RUSTDOC-AND-XTASK.15.a — cargo xtask check production-genericity-dependencies replaces the DEPENDENCIES
  component's script`

- ID: `RUSTDOC-AND-XTASK.15.b`
  Status: `done` (`2026-10-24`; CODE; checklist below)
  Goal: `scripts/check_production_genericity_inventory.pl` moves to xtask
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  **Done `2026-10-24`.** `xtask/src/checks/production_genericity_inventory.rs` holds each module-inventory row to its
  schema (five non-empty fields, a repository Rust path with no `..` component, a known current and target plane, a
  lane `e.ii`-`e.iv`), each claim-family row likewise (six fields, a `stage.name` family matching its stage, a known
  stage, field names), the inventories against the product's `.rs` files and the five artifact structs' `pub`
  fields read line by line as the script read them, and every module and field classified exactly once; problems
  read in the script's order and words, and an unreadable file ends it with Perl's `errno`. Its 3 self-test cases
  keep their verdicts on the same fixture under `generated/`. The Perl shim keeps `Getopt::Long` and the root
  resolution. **Parity**: HEAD's script beside the shim on 22 inputs (six argument forms; 16 fixture roots holding
  the real inventories and sources with one break each: a wrong header, every row shape, a duplicate, an
  unclassified, stale, hidden or directory-named module, a CRLF inventory, a duplicated family and its fields, an
  unclassified field, a missing struct, a struct line with trailing space, a missing inventory and a missing source):
  identical on all 22. **Cost:** 0.04 s for the script, 0.14 s for the shim.
  Verification: `cargo test -p xtask --lib` 91/91; parity and RED in `.project-data/tmp/rdx-15b/`
  Commit: `RUSTDOC-AND-XTASK.15.b — cargo xtask check production-genericity-inventory replaces the INVENTORY
  component's script`

- ID: `RUSTDOC-AND-XTASK.15.c`
  Status: `done` (`2026-10-24`; CODE; checklist below)
  Goal: `scripts/check_production_genericity_rules.pl` moves to xtask
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  **Done `2026-10-24`.** `xtask/src/checks/production_genericity_rules.rs` holds each rule-migration row to its
  schema (13 fields; its family known to the claim inventory and unique; its stage, stem and migration leaf; its
  premises; its capability and the alpha obligation that capability requires; compatibility; a target module and
  schema owner that exist), resolves every producer, mutator, canonical-seam and bypass entrypoint as the script did
  (`path#symbol`, the function named by the symbol's last non-empty `::` part declared with the script's signature
  pattern, whose optional generics close at the last `>` before the `(`), expands each family's fields into unique
  rule ids, requires the six canonical write seams, and holds each conformance bypass to its stage, leaf and a
  fail-closed disposition. Its 7 self-test cases keep their verdicts on the same fixture under `generated/`. A
  source file is read once rather than once per entrypoint. The Perl shim keeps `Getopt::Long` and the root
  resolution. **Parity**: HEAD's script beside the shim on 34 inputs (five argument forms; 29 fixture roots holding
  the real inventories and sources with one break each, from a wrong header to every rule-row field, malformed,
  duplicate, missing and absent entrypoints, a symbol ending `::`, a generic signature, malformed claim rows, an
  orphan family, every bypass refusal and a missing inventory): 33 identical, and one in stdout and exit code alone,
  a short rule row, where Perl also warned of six uninitialized values. **Cost:** 0.06 s for the script, 0.21 s for
  the shim.
  Verification: `cargo test -p xtask --lib` 94/94; parity and RED in `.project-data/tmp/rdx-15c/`
  Commit: `RUSTDOC-AND-XTASK.15.c — cargo xtask check production-genericity-rules replaces the RULES component's
  script`

- ID: `RUSTDOC-AND-XTASK.15.d`
  Status: `done` (`2026-10-24`; CODE; checklist below)
  Goal: `scripts/check_behavioral_genericity_contract.py` moves to xtask
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim; the claim re-derivation it serves
  still reads its output.
  **Done `2026-10-24`.** `xtask/src/checks/behavioral_genericity_contract.rs` holds the frozen behavioral contract
  to its closed authority set, relations, stage comparison, input planes, failure taxonomy and evidence contract;
  the reviewed recipes and the semantic-negative matrix; each of the 24 population rows to its SourceIR, normalized
  Markdown, live source and five stage artifacts; the population assertions and frozen census; the held-out
  evidence's split, 51 attempts, coverage, 90 strata and Wilson intervals; and the post-boundary declaration
  (ADR 0050), in the script's order and words. It sees values as Python did (equality across `int`, `float` and
  `bool`, `round` to even, `repr`, `OSError` and `UnicodeDecodeError` text, universal newlines, `csv`'s tab dialect,
  `PurePath`), and where Python raised it refuses with the exception's text and exits 1. Its 23 RED cases and the
  mandated grown retained set keep their verdicts. The Python shim keeps `argparse`; `PRODUCTION-GENERICITY` and the
  claims `retained-growth-admitted-as-declared-residual` (its RED region now in the port) and
  `wire-golds-installed-and-evidence-current` run it unchanged. **Found:** the script reported a row's non-integer
  fields in `set` order, which moved with the hash seed (four orders in six seeds; the port keeps declared order);
  its coverage totals rest on CPython 3.12's compensated `sum`, summed per attempt first; and a decoding error in the
  population counts its position from its 8,192-byte read chunk. **Parity**: HEAD's script beside the port on 214
  fixtures, each breaking one of the ten authority files or a SourceIR, 222 runs with 15 self-tests: 174 identical,
  48 where Python raised and the port refused with the same exception text, exit code and stdout, none different.
  **Cost** (quiet machine, before the swap): the script 3.7 s, 6.7 s with `--self-test`; the port 4.3-4.6 s (`.19`
  owns the difference) and 2.0 s, each SourceIR read once as a projection of what it checks; peak memory 2.0 GB
  against 1.5 GB.
  Verification: `cargo test -p xtask --lib` 103/103; parity and RED in `.project-data/tmp/rdx-15d/`
  Commit: `RUSTDOC-AND-XTASK.15.d — cargo xtask check behavioral-genericity-contract replaces the
  BEHAVIORAL-CONTRACT component's script`

- ID: `RUSTDOC-AND-XTASK.15.d.i`
  Status: `done` (`2026-10-24`; CODE; found by `.15.d`'s RED; checklist below)
  Goal: each subset-floor case in the behavioral self-test is caught by its own binding again
  Acceptance: deleting the subset-floor append, or the undeclared-excess append, fails the self-test and
  `cargo test -p xtask --lib`; each of the six boundary cases fails only with the problem written for it; claim
  `retained-growth-admitted-as-declared-residual` re-observed.
  **Found `2026-10-24`.** RED deleted each append in the port and both survived; so do they in HEAD's script. The
  cases `vanished frozen key` and `unreported post-boundary key` reuse the live post-boundary declaration, which has
  held seven keys since `EXTRACTION-REPRESENTATION-GAPS.3.a` (`2026-09-26`), and none of the seven is in those cases'
  retained set, so each case fails on seven `is not a retained bundle` problems whether or not its own binding
  fires. The claim's RED evidence, that the deletion misses its case, has been false since then. **Done.** The two
  cases now carry an empty declaration, consistent with their retained sets, and every boundary case names the
  problem its own binding writes: the self-test counts it caught only when that problem is present and no other
  is, and reports `missed` or `caught … only beside another problem` otherwise. Still 23/23 with 1/1 admitted.
  Deleting the subset-floor append now prints `self-test missed vanished frozen key` and exits 1; the claim's
  result records the 2026-09-26 to 2026-10-24 gap and the re-observation.
  Verification: `cargo test -p xtask --lib` 103/103; RED in `.project-data/tmp/rdx-15d/red/`
  Commit: `RUSTDOC-AND-XTASK.15.d.i — each subset-floor self-test case is caught by its own binding again`

- ID: `RUSTDOC-AND-XTASK.15.e`
  Status: `done` (`2026-10-24`; CODE; checklist below)
  Goal: `cargo xtask check production-genericity` replaces `scripts/check_production_genericity.sh`
  Acceptance: `.6`'s parity evidence; the script a shim; the components called in-process and the graph crate
  launched as `_flow.sh` launched it.
  **Done `2026-10-24`.** `xtask/src/checks/production_genericity.rs` composes the gate as the script did: every
  component runs and reports, and no failure stops the rest. The four xtask components run in-process through a
  new sink, `checks::Output` (the terminal, or one buffer holding both streams in the order they were written, as
  `$(… 2>&1)` captured them); the graph crate (`--check-census`) and the two qualification tests run through
  `cargo` with the project-data environment, their two streams one pipe. A passing component's output goes to
  standard output and a failing one's to standard error, trailing newlines trimmed as command substitution trimmed
  them; then the corpus-absent refusal and the `SPECFORGE_CORPUS_ABSENT` skips, the dangling-id meta-check, the
  report, and the summary that never counts a skip. The shim keeps the activation and the argument check and passes
  the name it was invoked by (`--program`), which the refusals quote; `scripts/check_production_genericity_flow.sh`
  stays the graph's standalone entry. **Parity**: HEAD's script beside the shim, each run by its absolute path as
  the driver runs it, on 13 cases (the default; three refused argument lists; a failing behavioral contract,
  information flow and inventory; the corpus hidden and undeclared, declared, and declared empty; declared with the
  corpus present; `--self-test`, alone and with the corpus hidden and declared): 11 identical, and both `--self-test`
  runs identical apart from `cargo test`'s elapsed-time lines. **Cost:** the graph analysis dominates: by default
  75-97 s for the script and 73-88 s for the shim, on a machine another repository's jobs held at a load average of
  11-17; no difference shows through that noise.
  Verification: `cargo test -p xtask --lib` 109/109; parity and RED in `.project-data/tmp/rdx-15e/`
  Commit: `RUSTDOC-AND-XTASK.15.e — cargo xtask check production-genericity replaces the PRODUCTION-GENERICITY
  gate's script`

- ID: `RUSTDOC-AND-XTASK.15.1`
  Status: `done` (`2026-10-27`; DESIGN; opened `.15.f`-`.15.z`)
  Goal: `LIVE-DOC-SIZE`'s component leaves
  Acceptance: re-measure its 20 component scripts and their callers outside the orchestrator; one leaf per
  component, the orchestrator last, in an order that keeps every kit caller's path and command line unchanged.
  **Measured `2026-10-27`** at `f1fa33d4c` (`.project-data/tmp/rdx-15-1/measure.py`, `measure.txt`).
  - The direct set is still **20 components / 18,539 lines**, under the 90-line orchestrator. As `.15.0`
    accounted it, the already-native README guard and shared `project_data_env.sh` are not port leaves.
  - The caller census covers tracked executable sources and command declarations, not only shell-shaped calls.
    `check_active_task_evidence.pl` is called by `check_task_evidence_contracts.sh` and eight contract
    declarations; `check_derived_state_contracts.pl` by `check_book_current_truth.sh` and its test;
    `check_live_document_size.pl` by its test and the claim/census controls; and
    `check_canonical_collection_catalogs.pl` by the claim kit and five census declarations. Other components
    have registry command declarations or no outside caller. Comments that merely name a producer are not calls.
  - **One helper was invisible to the orchestrator-only walk:** `test_derived_state_authorities.pl` constructs
    the path of `check_derived_state_authorities.pl` (268 lines) through `File::Spec`. That adapter also serves
    `check_book_current_truth.sh`, two derived-state contracts and two census verifiers. It belongs with its
    test in `.15.j`, bringing that leaf to 679 lines and the measured port boundary to 18,807 lines. The only
    other outside-set script names are the portable Knowledge Map generator/checker, which stay external kit
    commands, and a comment naming `measure_registry_capacity_coherence.py`, which is not a call.
  **Decided.**
  - Every leaf keeps `.6`'s contract: parity on the current tree, every existing self-test and one broken input
    per refusal, identical stdout/stderr/exit status, and a shim at every old path. A test-only component ports
    its whole matrix. Registry, census and kit invocations keep their paths and argv; RED regions may delegate
    to the Rust producer under `.12`'s existing rule. No product-crate dependency or new locked crate.
  - The order is ascending measured size, with a producer before its caller: `.15.k` before `.15.l`, `.15.t`
    before `.15.u`, and `.15.x` before `.15.y`. `.15.j` ports its helper and test together. `.15.z` is the
    orchestrator last, calling every component and the already-native README guard in-process.
  - `.15.i` also moves the claim kit's source-literal read of `MAX_ROW_BYTES` to the native producer, because
    making the Perl file a shim must not strand that consumer or leave a second authority in the shim.
  Verification: measurement above; `check_doctrines.sh --fast` PASS; `check_doctrines.sh --only LIVE-DOC-SIZE` PASS
  Commit: `RUSTDOC-AND-XTASK.15.1 — LIVE-DOC-SIZE's component leaves, measured and ordered (DESIGN)`

- ID: `RUSTDOC-AND-XTASK.15.f`
  Status: `done` (`2026-10-27`; CODE; checklist below)
  Goal: `cargo xtask check knowledge-map-projection-tests` replaces the 94-line
    `scripts/test_knowledge_map_projection.sh`
  Acceptance: `.15.1`'s common port contract; all eight generation, drift, collision, cleanup, staging, bound,
    name and residue checks reproduce. The portable generator and checker stay external commands at their paths.
  **Done `2026-10-27`.** `xtask/src/checks/knowledge_map_projection.rs` builds the fixture, invokes the portable
    generator and checker at their unchanged paths, and reproduces all eight checks, their fail-closed assertions,
    both output streams, exit statuses and cleanup. The shell script is a shim that keeps project-data activation;
    arguments remain ignored, as the script ignored them. `run_to` also writes through `checks::Output`, ready for
    `.15.z`. **Parity:** HEAD's script beside the port on the current tree, with an ignored argument, and with a
    controlled `cksum` mutation that changes the projection during the collision check: identical stdout, stderr
    and exit code on all three (0, 0 and 1 respectively); the RED path names `collision failure replaced a
    committed output`. The shell shim and native entry are identical too. Unit tests cover every assertion RED;
    a whole-`generated/` cleanup census proved racy under parallel tests and was narrowed to the test's own
    fixture, then passed repeatedly.
  Verification: `cargo test -p xtask --lib` 111/111; success/argument/RED parity in `.project-data/tmp/rdx-15f/`;
    `mdbook test` and `mdbook build` PASS with the existing `<operation>` warning unchanged
  Commit: `RUSTDOC-AND-XTASK.15.f — cargo xtask check knowledge-map-projection-tests replaces the projection suite`

- ID: `RUSTDOC-AND-XTASK.15.g`
  Status: `done` (`2026-10-27`; CODE; checklist below)
  Goal: `cargo xtask check fsmgen-request-bundles` replaces the 355-line
    `scripts/check_fsmgen_request_bundles.pl`
  Acceptance: `.15.1`'s common port contract, including `--check`, `--self-test` and `--root`; every request
    packet, ledger join, legacy exemption and portability refusal agrees.
  **Done `2026-10-27`.** `xtask/src/checks/fsmgen_request_bundles.rs` holds every packet to the request schema,
    lifecycle, layout, summary, signoff, verification, ownership, ledger and portability rules, reading JSON and
    scalar values through `pyjson` and `perl`. Its 17 self-test cases copy the real packets and task trees under
    `generated/` and keep their verdicts: one accepted baseline and 16 refusals, including both ledger directions,
    the current-state-dependent status mutation and every portability refusal. The Perl shim keeps the original
    argument loop and usage text, resolves the requested root for the check and uses the repository only as Cargo's
    fallback working directory, so a nonexistent root keeps its original diagnostic and exit code. **Parity:** the
    original beside native and shim on `--check`, `--self-test`, a relative root, a nonexistent root, an unknown
    option, a missing option value and an actual unknown-owner packet: identical stdout, stderr and exit code on
    all of them; self-test is 17/17.
  Verification: `cargo test -p xtask --lib` 113/113; parity and RED in `.project-data/tmp/rdx-15g/`
  Commit: `RUSTDOC-AND-XTASK.15.g — cargo xtask check fsmgen-request-bundles replaces the request-bundle checker`

- ID: `RUSTDOC-AND-XTASK.15.h`
  Status: `done` (`2026-10-27`; CODE; checklist below)
  Goal: `cargo xtask check task-tree-catalog` replaces the 374-line `scripts/check_task_tree_catalog.pl`
  Acceptance: `.15.1`'s common port contract, including `--check`, `--write`, `--self-test` and `--root`; the
    landing and every catalog part regenerate byte-identically, with its 13 parser/partition/bound cases.
  **Done `2026-10-27`.** `xtask/src/checks/task_tree_catalog.rs` parses every real tree, holds titles, statuses,
    row/part/section bounds and the 56-member quantum, and renders the bounded landing and every derived part
    byte for byte. Marker replacement and legacy-section migration, stale-part detection/removal and same-directory
    atomic writes are included; the Perl shim keeps modes, root diagnostics and UTF-8 output. All 13 parser,
    partition and bound cases keep their verdicts. **Parity:** original and port on `--check`, `--self-test`, a full
    copied-tree `--write` with an open-status change and an unplanned part, malformed markers, that unplanned part,
    and a nonexistent root: identical stdout, stderr and exit code, with the complete generated trees identical.
  Verification: `cargo test -p xtask --lib` 115/115; check/write/refusal parity in `.project-data/tmp/rdx-15h/`
  Commit: `RUSTDOC-AND-XTASK.15.h — cargo xtask check task-tree-catalog replaces the catalog generator`

- ID: `RUSTDOC-AND-XTASK.15.i`
  Status: `pending`
  Goal: `cargo xtask check canonical-collection-catalogs` replaces the 393-line
    `scripts/check_canonical_collection_catalogs.pl`
  Acceptance: `.15.1`'s common port contract, including `--check`, `--write`, `--self-test`, `--root`, `--config`
    and `--surfaces`; every index regenerates byte-identically. The claim kit reads `MAX_ROW_BYTES` from the
    native producer, and all five census commands and both claim controls keep their interface.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.j`
  Status: `pending`
  Goal: `cargo xtask check derived-state-authority-tests` replaces the 411-line
    `scripts/test_derived_state_authorities.pl` and ports its 268-line helper
    `scripts/check_derived_state_authorities.pl` as `derived-state-authorities`
  Acceptance: `.15.1`'s common port contract for both scripts, including `--quiet`, `--root`, `--registry` and
    `--contract`; both project adapters and all positive/fail-closed fixtures agree. The book-truth caller,
    two derived-state contracts and two census verifiers keep their paths and argv.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.k`
  Status: `pending`
  Goal: `cargo xtask check derived-state-contracts` replaces the 574-line
    `scripts/check_derived_state_contracts.pl`
  Acceptance: `.15.1`'s common port contract, including `--root`, `--registry`, `--surfaces` and `--report`;
    every field class, marker, authority, secondary-copy and executed-verifier contract agrees, and the
    book-truth caller is unchanged.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.l`
  Status: `pending`
  Goal: `cargo xtask check derived-state-contract-tests` replaces the 530-line
    `scripts/test_derived_state_contracts.pl`
  Acceptance: `.15.1`'s common port contract, including `--quiet`; the complete positive and fail-closed matrix
    runs against `.15.k`'s native checker through the preserved script interface, with no fixture residue.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.m`
  Status: `pending`
  Goal: `cargo xtask check knowledge-map-shard-contract` replaces the 675-line
    `scripts/check_knowledge_map_shard_contract.pl`
  Acceptance: `.15.1`'s common port contract for every existing mode; shard topology, capacity and exact
    projection contracts agree. The portable generator's `--print-contract` query remains an external kit call.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.n`
  Status: `pending`
  Goal: `cargo xtask check validation-snapshot-currentness` replaces the 706-line
    `scripts/check_validation_snapshot_currentness.pl`
  Acceptance: `.15.1`'s common port contract for every existing mode; the reviewed boundary, source identity,
    partition coverage and all current/census verifier outputs agree.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.o`
  Status: `pending`
  Goal: `cargo xtask check task-tree-archive` replaces the 746-line `scripts/check_task_tree_archive.pl`
  Acceptance: `.15.1`'s common port contract for every existing mode; exact source/terminal identity, manifest,
    retrieval, continuation and residue checks agree.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.p`
  Status: `pending`
  Goal: `cargo xtask check source-pdf-registry-currentness` replaces the 751-line
    `scripts/check_source_pdf_registry_currentness.pl`
  Acceptance: `.15.1`'s common port contract for every existing mode; Git-indexed registry authority, source
    identity, external-origin labels and all current/census verifier outputs agree.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.q`
  Status: `pending`
  Goal: `cargo xtask check corpus-kb-currentness` replaces the 757-line
    `scripts/check_corpus_kb_currentness.pl`
  Acceptance: `.15.1`'s common port contract for every existing mode; managed-page membership, source identity,
    fixture staging and all current/census verifier outputs agree.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.r`
  Status: `pending`
  Goal: `cargo xtask check fsmgen-feedback-protocol` replaces the 790-line
    `scripts/check_fsmgen_feedback_protocol.pl`
  Acceptance: `.15.1`'s common port contract for every existing mode; current-boundary, archive, request-link,
    required-literal and gitlink contracts agree, with all current/census verifier interfaces unchanged.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.s`
  Status: `pending`
  Goal: `cargo xtask check roadmap-projection-contract` replaces the 1,169-line
    `scripts/check_roadmap_projection_contract.pl`
  Acceptance: `.15.1`'s common port contract for every existing mode; bounded-root, archive, owner-projection,
    manifest and exact reconstruction checks agree, with all current/census verifier interfaces unchanged.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.t`
  Status: `pending`
  Goal: `cargo xtask check live-document-size-core` replaces the 1,554-line
    `scripts/check_live_document_size.pl`
  Acceptance: `.15.1`'s common port contract, including `--root`, `--registry`, `--routes`, `--authorities`,
    `--report` and `--no-history`; surface coverage, lifecycle, pressure, registry and warning semantics agree,
    and every claim/census command keeps its interface.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.u`
  Status: `pending`
  Goal: `cargo xtask check live-document-size-tests` replaces the 1,559-line
    `scripts/test_live_document_size.pl`
  Acceptance: `.15.1`'s common port contract, including `--quiet`; the complete positive and fail-closed matrix
    runs against `.15.t`'s native checker through the preserved script interface, with no fixture residue.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.v`
  Status: `pending`
  Goal: `cargo xtask check fact-card-catalog` replaces the 1,729-line `scripts/check_fact_card_catalog.pl`
  Acceptance: `.15.1`'s common port contract for every existing mode; card membership, capacity, titles,
    question keys and every generated index/part regenerate byte-identically, with census/producer commands
    unchanged.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.w`
  Status: `pending`
  Goal: `cargo xtask check rolling-ledger-protocol` replaces the 2,579-line
    `scripts/check_rolling_ledger_protocol.pl`
  Acceptance: `.15.1`'s common port contract for every existing mode; self-tests, exact plan identity, dry-run,
    root-last apply, rollback, chronology and warning-safe capacity behavior agree, including byte restoration.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.x`
  Status: `pending`
  Goal: `cargo xtask check active-task-evidence` replaces the 2,756-line
    `scripts/check_active_task_evidence.pl`
  Acceptance: `.15.1`'s common port contract for every existing mode and all eight declared contracts; live
    graph, parts, routes, sealed/archive identity and generated projections agree byte-for-byte, with every
    contract verifier interface unchanged.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.y`
  Status: `pending`
  Goal: `cargo xtask check task-evidence-contracts` replaces the 37-line
    `scripts/check_task_evidence_contracts.sh`
  Acceptance: `.15.1`'s common port contract, including `--root`; the single eight-contract list runs every
    contract through `.15.x` in-process, preserves fail-accumulation and adds a focused self-test.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15.z`
  Status: `pending`
  Goal: `cargo xtask check live-document-size` replaces the 90-line `scripts/check_live_document_size.sh`
  Acceptance: `.6`'s parity evidence and the script a shim. Every ported component and the already-native README
    guard run in-process; self-test/check phases, fail-accumulation, `--root` and project-data activation agree.
    Portable Knowledge Map commands stay external, and no kit caller's path or argv changes.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.16`
  Status: `done` (`2026-10-23`; DESIGN; corrects `.6`; retired `.9`-`.11`)
  Goal: a portable architecture's kit stays portable
  Acceptance: classify every doctrine's script against the five portable architectures' kit manifests, and keep the
  kit artifacts out of the port.
  **Found.** Reading `knowledge-map/scripts/check_knowledge_map.sh` for `.9` showed its header, *Portable runtime:
  Bash 3.2+*, and that it runs the generator by `$SCRIPT_DIR`, which `.6`'s ranking did not follow. SpecForge adopts
  five portable architectures (`DOCTRINE_ENFORCEMENT.md`): task trees, memory architecture, knowledge map, doctrine
  enforcement and claim verification, each a kit other repositories copy. A kit script ported to SpecForge's Rust
  `xtask` would no longer be the kit, so `.6` ordered the wrong set.
  **Measured `2026-10-23`** (`.project-data/tmp/rdx-16/kit.py`): naming each doctrine's script in the five standards,
  four are kit artifacts: `MEMORY-ARCH` (`MEMORY_ARCHITECTURE.md` §9), `KNOWLEDGE-MAP` (the `knowledge-map/` bundle),
  `TASK-ACCEPTANCE` (`DOCTRINE_ENFORCEMENT.md` §8 Group A) and `CLAIM-VERIFICATION` (`CLAIM_VERIFICATION.md`), and
  the driver `check_doctrines.sh` and the git hooks are Group A too. The other 16 doctrines are SpecForge's own.
  **Decided.**
  - **A kit artifact is not ported.** It stays in its portable language, and `cargo xtask doctrines` runs it through
    the driver; a fix to it is a fix to the kit, made in place.
  - **The port set** is the 16 SpecForge doctrines: `RUSTDOC` (done, `.8`), `README-POLICY` (done, `.12`), wave 2's six
    single-file gates (`.13`), wave 3's six multi-file gates and oracles (`.14`), and wave 4's two composites (`.15`).
  - `.9`-`.11` are retired with this reason, their nodes kept as history.
  Verification: `classification in .project-data/tmp/rdx-16/kit.txt; doctrine gates at commit`
  Commit: `RUSTDOC-AND-XTASK.16 — a portable architecture's kit stays portable (DESIGN)`

- ID: `RUSTDOC-AND-XTASK.17`
  Status: `done` (`2026-10-23`; CODE; checklist below)
  Goal: the ports read and write JSON exactly as Python's `json` module does
  Acceptance: one reader and writer that agree with CPython on its error messages, key order and encoder options,
  proven against Python on a corpus; the ports that read JSON use it.
  **Found by `.13.c`.** `CONTRACT-ENCODING` asks, of each contract under `doctrine/`, which of 80 `json.dumps`
  option sets reproduces its bytes, so its port must write exactly what Python writes, in key order, with Python's
  float `repr`; `serde_json` sorts keys and formats floats otherwise, and its error wording was already `.13.b`'s
  one divergence. **Done `2026-10-23`.** `xtask/src/pyjson.rs` reproduces CPython 3.14's C scanner (its messages
  and code-point positions, the trailing-comma refusals, surrogate pairs and the lone surrogates it keeps, the
  4,300-digit `int` limit, a repeated key keeping its first position) and its encoder (`indent`, `sort_keys`,
  `ensure_ascii`, `separators`, Python's float `repr`), with Python's `==`, truthiness, `repr` and `str` of the
  values. **Proven** by a table of CPython's answers in the unit tests and an ignored differential test that reads a
  corpus Python wrote (`.project-data/tmp/rdx-17/corpus.py`): 14,964 cases, 12,755 of them malformed inputs mutated
  from the repository's JSON with Python's exact error, and every contract under all 80 encoder option sets, all
  agree. `task-node-retention` now reads its registry through it (27 of 27 parity inputs identical), and xtask
  depends on nothing again.
  Verification: `cargo test -p xtask --lib` 38/38; the corpus test (`--ignored`) 14,964/14,964
  Commit: `RUSTDOC-AND-XTASK.17 — pyjson: the ports read and write JSON exactly as Python does`

- ID: `RUSTDOC-AND-XTASK.18`
  Status: `done` (`2026-10-23`; CODE; found by `.13.c`; checklist below)
  Goal: every doctrine the driver registers is documented in both doctrine tables, mechanically
  Acceptance: a check refuses a registered doctrine with no row in `docs/DOCTRINE_INSTANCE.md` or in the book's
  `reference/doctrine-enforcement.md` table, and a row naming no registered doctrine; RED on each.
  **Measured `2026-10-23`.** After `.13.c`'s added rows, the driver's 20 entries, §10's 20 rows and the book's 20
  agree on every id, and §10 agrees with the driver on every tier and check and with the book on every kind, so all
  three can be enforced. **Done.** A new gate-tier doctrine, `DOCTRINE-TABLES`, native from its first commit:
  `xtask/src/checks/doctrine_tables.rs` reads the driver's `DOCTRINES` entries, §10's table and the book's, and
  refuses a registered doctrine missing from either table, a row for an unregistered one, a repeated row, and a tier,
  check or kind that differs; `scripts/check_doctrine_tables.sh` is its registered path (a shim that refuses
  arguments and activates the project-data roots). It documents itself in both tables, so the registry is 21
  doctrines. Not a change to the driver, a portable kit artifact (`.16`): it only gains the entry. The port set stays
  `.16`'s 16 SpecForge doctrines that had scripts; this 17th was never a script.
  Verification: `cargo test -p xtask --lib` 53/53; self-test 8/8; RED in `.project-data/tmp/rdx-18/`
  Commit: `RUSTDOC-AND-XTASK.18 — DOCTRINE-TABLES: both doctrine tables match the driver's registry`

- ID: `RUSTDOC-AND-XTASK.19`
  Status: `pending` (found by `.15.d`)
  Goal: a native check that parses as much JSON as its script did is no slower than the script
  Acceptance: the behavioral contract's default mode at or under the Python script's time on the same tree,
  measured; `pyjson`'s corpus proof (`.17`) still 14,964 of 14,964; no product crate's build changed.
  **Measured `2026-10-24` (`.15.d`).** The behavioral contract parses 950 MB of stage artifacts and hashes 33 MB of
  sources: 4.3-4.5 s natively against the script's 3.7 s. In a 3 s sample of the native run, 432 of 2,151 samples
  are `pyjson::parse` collecting the whole input into a `Vec<char>` before it scans, and about 700 are `sha2`'s
  software compression, which the dev profile leaves unoptimized because `[profile.dev.package.xtask]` optimizes
  the tooling crate alone (`.13.a`). Peak memory is 2.0 GB against the script's 1.5 GB.
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

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.12` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `README-POLICY` doctrine ran only as shell; its script exits 0 on 4 malformed
  registries (`.project-data/tmp/rdx-12/old/`: `unterminated-last-row`, `row-empty-column`, `row-leading-tab`,
  `row-trailing-tab`).
- [x] **ROOT CAUSE (WHY + WHERE)** — HEAD's `scripts/check_readme_policy.sh:158`: `while IFS=$'\t' read -r` skips an
  unterminated last line and treats a tab as whitespace. RED 4/4, every file restored byte-identically with a fresh
  mtime: the line ceiling made inclusive, tabs merged again, the unterminated row dropped, the shim running another
  check; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 1 -> 2; identical output on 28 of 33 inputs, the 5 differences the
  two fixed fail-opens and the leaked awk error.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 15
  passed; claim-gate self-test 34/34 and repin self-test 38/38; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `CLAIM_VERIFICATION.md`'s RED-region rule; `README_POLICY.md`'s adoption note;
  `docs/DOCTRINE_INSTANCE.md`'s `README-POLICY` row; `docs/toolbox/gates-build-and-host.md`; book
  `getting-started.md`; KM card `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.15.h` (CODE) — DONE `2026-10-27`

- [x] **REPRODUCE / MEASURE** — the task-tree catalog ran only as its 374-line Perl generator: 172 real trees
  across four parts with 39 open routes, and 13/13 parser, partition and bound cases. HEAD's script and baselines
  are in `.project-data/tmp/rdx-15h/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native component: `xtask/src/checks/mod.rs` listed 23 checks. RED: all 13
  cases, plus malformed markers, an unplanned part and a nonexistent root, produce the same refusal. The first
  marker check failed because the extractor had omitted the required LF after `END`; including it made the real
  landing and write path agree byte for byte.
- [x] **ADDRESSED (verified)** — native checks 23 -> 24; check, self-test, full copied-tree write and every
  representative refusal are identical in stdout, stderr and exit status; the generated trees are byte-identical.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 115
  passed; `check_doctrines.sh --fast`, `--only LIVE-DOC-SIZE,PROJECT-DATA-LOCALITY` PASS; mdBook and full gate.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code, extraction behavior or dependency graph changed.
- [x] **LOCKSTEP** — book `getting-started.md` lists the new native check; KM card and `MEMORY.md` updated; no
  production rule was deleted or replaced.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.15.g` (CODE) — DONE `2026-10-27`

- [x] **REPRODUCE / MEASURE** — the FSMGen request-bundle gate ran only as its 355-line Perl script: 16 current
  packets plus three legacy exemptions, and 17 self-test cases (one accepted, 16 refused). HEAD's script and
  baselines are in `.project-data/tmp/rdx-15g/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native component: `xtask/src/checks/mod.rs` listed 22 checks. RED: the
  real tree plus all 17 mutations runs in `the_real_tree_and_all_self_test_cases_hold`; an actual unknown-owner
  packet is refused identically. A UTF-8 delimiter-length bug was located by the multi-field ledger case, and a
  nonexistent-root probe caught the shim changing the original diagnostic and exit 1 to 2; both were fixed and
  re-proven before the full suite.
- [x] **ADDRESSED (verified)** — native checks 22 -> 23; check, self-test, relative/nonexistent roots, unknown
  options, missing values and the full unknown-owner refusal are byte-identical in stdout, stderr and exit code.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 113
  passed; `check_doctrines.sh --fast`, `--only LIVE-DOC-SIZE,PROJECT-DATA-LOCALITY` PASS; mdBook and full gate.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code, extraction behavior or dependency graph changed.
- [x] **LOCKSTEP** — book `getting-started.md` lists the new native check; KM card and `MEMORY.md` updated; no
  production rule was deleted or replaced.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.15.f` (CODE) — DONE `2026-10-27`

- [x] **REPRODUCE / MEASURE** — the Knowledge Map projection integration suite ran only as its 94-line shell
  script: `8/8` on stdout, five kit lines on stderr, exit 0, and an arbitrary argument ignored. HEAD's script and
  those baselines are in `.project-data/tmp/rdx-15f/old/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native component: `xtask/src/checks/mod.rs` listed 21 checks. RED:
  `every_integration_assertion_fails_closed` exercises each new assertion with a broken fixture, and a controlled
  `cksum` mutation makes both implementations report `collision failure replaced a committed output`, exit 1.
  The first whole-`generated/` cleanup assertion also failed under parallel tests on another test's live fixture;
  its mechanism was the shared census, narrowed to the test's own path before the rerun passed.
- [x] **ADDRESSED (verified)** — native checks 21 -> 22; success, ignored-argument and collision-RED runs are all
  byte-identical in stdout, stderr and exit status, and the compatibility shim agrees with the native entry.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 111
  passed; `check_doctrines.sh --fast`, `--only LIVE-DOC-SIZE,PROJECT-DATA-LOCALITY` PASS; `mdbook test` and
  `mdbook build` PASS with the existing `<operation>` warning unchanged; full doctrine gate at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code, extraction behavior or dependency graph changed.
- [x] **LOCKSTEP** — book `getting-started.md` lists the new native check; KM card and `MEMORY.md` updated; no
  production rule was deleted or replaced.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.15.e` (CODE) — DONE `2026-10-24`

- [x] **REPRODUCE / MEASURE** — `PRODUCTION-GENERICITY` ran as a 188-line shell orchestrator over its components'
  shims: 5 of 5 baseline components PASS in 77 s. HEAD's copy is the parity baseline in `.project-data/tmp/rdx-15e/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native orchestrator: `xtask/src/checks/mod.rs` listed 20 checks, and its
  components printed straight to the terminal, so none could be captured in-process. RED 6/6, restored with a fresh
  mtime: trailing newlines kept, a cargo component's standard error left out of its pipe, a missing `source_ir`
  read as a corpus, a skipped component counted as executed, a corpus-dependent id naming no component, the capture
  dropping standard output; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — 13 parity cases: 11 identical, the two `--self-test` runs identical apart from
  `cargo test`'s elapsed-time lines.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 109
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — book `getting-started.md` (the native doctrine checks); `docs/DOCTRINE_INSTANCE.md`'s
  `PRODUCTION-GENERICITY` row; `docs/toolbox/gates-build-and-host.md` §7.2c; KM card
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.15.d.i` (CODE) — DONE `2026-10-24`

- [x] **REPRODUCE / MEASURE** — `.15.d`'s RED: deleting the subset-floor append, or the undeclared-excess append,
  left the self-test at 23/23 and `cargo test -p xtask --lib` green, in the port and in HEAD's script alike.
- [x] **ROOT CAUSE (WHY + WHERE)** — `run_self_test`'s cases `vanished frozen key` and `unreported post-boundary
  key` reused the live post-boundary declaration; its seven keys lie outside their retained sets, so each case
  failed on seven `is not a retained bundle` problems whatever its own binding did. RED 4/4, restored with a fresh
  mtime: either append deleted, or either case given the live declaration back; each fails `cargo test -p xtask
  --lib`.
- [x] **ADDRESSED (verified)** — each boundary case fails with its own problem alone; deleting the subset-floor
  append prints `self-test missed vanished frozen key` and exits 1; self-test 23/23 with 1/1 admitted.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `xtask` 103 passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — claim `retained-growth-admitted-as-declared-residual` re-observed; the book's six subset-floor
  faults are unchanged.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.15.d` (CODE) — DONE `2026-10-24`

- [x] **REPRODUCE / MEASURE** — `PRODUCTION-GENERICITY`'s behavioral contract ran only as Python
  (`scripts/check_behavioral_genericity_contract.py`, 2,086 lines): `24 current documents; … 7 retained
  post-boundary and unqualified` in 3.7 s, its self-test 23/23 with 1/1 admitted in 6.7 s. HEAD's copy is the
  parity baseline in `.project-data/tmp/rdx-15d/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native component: `xtask/src/checks/mod.rs` listed 19 checks. RED, each
  restored byte-identically with a fresh mtime: 13 of 15 fail `cargo test -p xtask --lib` (a 4,096-byte decoding
  chunk, `round` half away from zero, an uncompensated `sum`, a collapsed two-slash root, `int()` admitting a
  leading underscore, a header skipping blank records, repeated prospective keys counted twice, a relation hashed
  before its key is known, malformed attempts skipped by the counts, a plane id hashed before its relations, the
  SourceIR projection dropping the claims, `0xff` read as a lead byte, the nested coverage sum flattened). The other
  two, deleting either subset-floor append, survive in HEAD's script as well: a found defect, `.15.d.i`.
- [x] **ADDRESSED (verified)** — 222 parity runs over 214 fixtures: 174 identical, 48 refused where Python raised,
  none different; self-test 23/23 with 1/1 admitted; the claim re-derivations read the same lines.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 103
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes. The port keeps the script's data-only
  labels (the self-test's three gold keys, the six category names).
- [x] **LOCKSTEP** — KM card `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`; claims
  `retained-growth-admitted-as-declared-residual` and `wire-golds-installed-and-evidence-current` name the port as a
  control; the book states the check's behaviour, not its language, and is unchanged.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.15.c` (CODE) — DONE `2026-10-24`

- [x] **REPRODUCE / MEASURE** — `PRODUCTION-GENERICITY`'s `RULES` component ran only as Perl
  (`scripts/check_production_genericity_rules.pl`); HEAD's copy is the parity baseline in
  `.project-data/tmp/rdx-15c/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native component: `xtask/src/checks/mod.rs` listed 18 checks. RED 7/7, the
  file restored byte-identically with a fresh mtime: a duplicate entry admitted, trailing empty `::` parts kept,
  generics closed at the first `>`, the word boundary before `fn` ignored, an alpha mismatch admitted, the canonical
  seams unchecked, an unsafe bypass disposition admitted; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — 33 of 34 inputs identical, one in stdout and exit code; self-test 7/7.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 94
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — KM card `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.15.b` (CODE) — DONE `2026-10-24`

- [x] **REPRODUCE / MEASURE** — `PRODUCTION-GENERICITY`'s `INVENTORY` component ran only as Perl
  (`scripts/check_production_genericity_inventory.pl`); HEAD's copy is the parity baseline in
  `.project-data/tmp/rdx-15b/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native component: `xtask/src/checks/mod.rs` listed 17 checks. RED 7/7, the
  file restored byte-identically with a fresh mtime: a duplicate module admitted, stale paths unreported, an unknown
  target plane admitted, a doubly classified field admitted, a field read with a space before its colon, `..`
  components admitted, unclassified fields unreported; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — 22 of 22 inputs identical; self-test 3/3.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 91
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — KM card `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.15.a` (CODE) — DONE `2026-10-24`

- [x] **REPRODUCE / MEASURE** — `PRODUCTION-GENERICITY`'s `DEPENDENCIES` component ran only as Perl
  (`scripts/check_production_genericity_dependencies.pl`); HEAD's copy is the parity baseline in
  `.project-data/tmp/rdx-15a/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native component: `xtask/src/checks/mod.rs` listed 16 checks. RED 7/7, the
  file restored byte-identically with a fresh mtime: the reverse dependency admitted, the alias admitted, the tool's
  dependency on the product admitted, the `trajectory` module renamed out of the oracle set, a table's body run past
  the next table, an empty quoted member skipped whole, any crate admitted to xtask; each fails
  `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — 29 of 29 inputs identical; self-test 11/11.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 88
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — KM card `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.14.g` (CODE) — DONE `2026-10-24`

- [x] **REPRODUCE / MEASURE** — the persisted-artifact path contract ran only as Perl
  (`scripts/check_persisted_artifact_paths.pl`, 94 s over the persisted corpus); HEAD's copy is the parity baseline in
  `.project-data/tmp/rdx-14g/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native tool: `xtask/src/checks/mod.rs` listed 15 checks, and
  `PROJECT-DATA-LOCALITY` spawned the Perl tool twice. RED 7/7, the file restored byte-identically with a fresh
  mtime: `_root` keys ignored, UNC paths not absolute, the EvidenceIR's external label unauthorized, the message cap
  off by one, escapes taken raw, the legacy needle without its slash, label lines skipped; each fails
  `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — 36 of 36 inputs identical, migrated trees included; self-test 18/18; the book's
  counts corrected.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 86
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `.14` closed; `docs/DOCTRINE_INSTANCE.md`'s `PROJECT-DATA-LOCALITY` row; book
  `reference/project-data-locality.md`; KM cards `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask` and
  `cross-stage-artifact-paths-are-absolute`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.14.f` (CODE) — DONE `2026-10-24`

- [x] **REPRODUCE / MEASURE** — the `PROOF-SEAL-TOTAL` entry ran only as shell (`scripts/check_proof_seal_total.sh`,
  over the owning script); HEAD's copy is the parity baseline in `.project-data/tmp/rdx-14f/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native entry: `xtask/src/checks/mod.rs` listed 14 checks; and HEAD's
  `scripts/check_proof_seal_total.sh:19` carried "the same 19 controls" by hand. RED 4/4, every file restored
  byte-identically with a fresh mtime: `--check` no longer total, the last argument read, the self-test running the
  probe, the shim running the sampled gate; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 13 -> 14; identical output on 9 of 9 inputs, the persisted corpus
  among them.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 83
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md`'s row; book `getting-started.md`; KM card
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.14.e` (CODE) — DONE `2026-10-24`

- [x] **REPRODUCE / MEASURE** — the `PROOF-SEAL-CURRENCY` doctrine and its seal reader ran only as shell and Perl
  (`scripts/check_proof_seal_currency.sh`, `scripts/lib/proof_seal_scan.sh`); HEAD's copies, with the totals
  reporter's, are the parity baseline in `.project-data/tmp/rdx-14e/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 13 checks. RED 7/7, every
  file restored byte-identically with a fresh mtime: the prefilter's carry dropped, the ledger's depth relaxed, the
  seal read at any depth, an empty seal kept, the diagnostic unsqueezed, the default total stages emptied, the
  library calling the wrong reader; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 12 -> 13; identical output on 55 of 55 inputs, both tiers of the
  persisted corpus among them; self-test 21/21; the cascade's 14/14 through the library.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 82
  passed; `report_self_test_totals.pl --check` unchanged (22 / 21 / 14); `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `.14.f`; `docs/DOCTRINE_INSTANCE.md`'s row; book `getting-started.md`; KM cards
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask` and `proof-seal-currency-gate`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.14.d` (CODE) — DONE `2026-10-24`

- [x] **REPRODUCE / MEASURE** — the `CHAIN-CURRENCY` doctrine and its content-identity predicate ran only as shell
  and Perl (`scripts/check_chain_currency.sh`, `scripts/lib/stage_artifact_identity.sh`); HEAD's copies, with the
  totals reporter's, are the parity baseline in `.project-data/tmp/rdx-14d/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 12 checks. RED 7/7, every
  file restored byte-identically with a fresh mtime: `proof_ledger` compared, a long integer kept numeric, a stale
  `.isf` sibling admitted, a duplicate retained key admitted, a negative current count allowed, the `*_json:` marker
  line kept, the library calling the other predicate; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 11 -> 12; 49 of 54 inputs identical, three in stdout and exit code,
  two different by design; the persisted corpus identical in both phases; self-test 22/22; the cascade's 14/14
  through the library.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 79
  passed; `report_self_test_totals.pl --check` unchanged (22 / 21 / 14); `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md`'s row; book `getting-started.md` and
  `reference/doctrine-enforcement.md`; KM cards `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask` and
  `chain-currency-doctrine`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.14.c` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `PROJECT-DATA-LOCALITY` doctrine ran only as shell
  (`scripts/check_project_data_locality.sh`); HEAD's copy is the parity baseline in `.project-data/tmp/rdx-14c/`. A
  launcher into another repository copy, written into a probe environment, is matched by the script's `rg` and then
  dropped by its own filter.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 11 checks; and HEAD's
  `scripts/check_project_data_locality.sh:94-95` filtered `rg`'s path-prefixed lines with `grep -Fv "$ROOT"`. RED
  5/5, the file restored byte-identically with a fresh mtime: the required-text test inverted, the stale filter made
  vacuous again, `.log` residue ignored, the composed tools' verdicts ignored, hidden entry points skipped; each
  fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 10 -> 11; 19 of 21 inputs identical, one in stdout and exit code, one
  different by design; a stale launcher now fails the gate.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 75
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `.14` and `.14.g`; `docs/DOCTRINE_INSTANCE.md`'s row; book `getting-started.md` and
  `reference/doctrine-enforcement.md`; KM cards `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask` and
  `project-data-locality-enforcement`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.14.b` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `CONSTRAINT-PART-SPAN` doctrine ran only as shell and Python
  (`scripts/check_constraint_part_span.sh` over `scripts/measure_constraint_part_span.py`); HEAD's copies are the
  parity baseline in `.project-data/tmp/rdx-14b/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 10 checks. RED 6/6, the file
  restored byte-identically with a fresh mtime: a test module's caller counted, the `clause` span unread, a negated
  spelling dropped from the arms, the declared total 10 -> 9, the untyped-arm guard disabled, `#[cfg(test)]` bounding
  only modules; each fails `cargo test -p xtask --lib`, and all but the disabled guard, which the clean tree cannot
  show, also turn `scripts/check_constraint_part_span.sh` red.
- [x] **ADDRESSED (verified)** — ported gates 9 -> 10; 48 of 68 inputs identical in full, 18 in stdout and exit
  code, two different by design; self-test 10/10.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 72
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md`'s row; book `getting-started.md`; KM cards
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask` and `constraint-record-producer-strata`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.14.a` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `CORPUS-FRONTIER` doctrine ran only as shell and Perl
  (`scripts/check_corpus_frontier.sh` over `scripts/check_corpus_frontier_census.pl`); HEAD's copies are the parity
  baseline in `.project-data/tmp/rdx-14a/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 9 checks. RED 6/6, the file
  restored byte-identically with a fresh mtime: a remaining key's retained bundle accepted, a cohort member missing
  from the partition accepted, the cohort count never compared, a duplicate remaining key accepted, the declared
  total 13 -> 12, the root task's sentences never read; each turns `scripts/check_corpus_frontier.sh` red (exit 1,
  naming the case).
- [x] **ADDRESSED (verified)** — ported gates 8 -> 9; identical output on 49 of 49 inputs; self-test 13/13.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 67
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `.14` and its six leaves; `docs/DOCTRINE_INSTANCE.md`'s row; the falsification control
  `corpus-frontier-retention-red-matrix`'s region; book `getting-started.md`; KM cards
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask` and `corpus-refresh-frontier-derivation`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.13.f` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `RESIDUAL-ACTIONABILITY` doctrine ran only as Python
  (`scripts/validate_residual_actionability_contract.py`); HEAD's copy is the parity baseline in
  `.project-data/tmp/rdx-13f/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 8 checks. RED 6/6, every file
  restored byte-identically with a fresh mtime: a duplicate residual credited, the family state reading the other
  bucket, the retained-set digest never compared, a boolean hashed apart from its integer, strings sorted in
  reverse, the shim running another check; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 7 -> 8, which completes wave 2; identical output on 31 of 31 inputs;
  self-test 40/40 RED and 1/1 admitted.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 65
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md`'s row; book `getting-started.md`; KM card
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.13.e` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `PUBLISHED-ASSERTIONS` doctrine ran only as Perl
  (`scripts/check_published_assertions.pl`); HEAD's copy is the parity baseline in `.project-data/tmp/rdx-13e/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 7 checks, and xtask had no
  SHA-256. RED 6/6, every file restored byte-identically with a fresh mtime: a hyphenated identifier taken as a
  value, commas kept when values are compared, a boolean leaf taken for a field, inline code scanned for values, a
  cross-surface disagreement accepted, the shim running another check; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 6 -> 7; identical output on 23 of 23 inputs; self-test 25/25.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 61
  passed; `Cargo.lock` changes only by xtask's `sha2` edge; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — the tree's Non-Goals and `.14`; `docs/DOCTRINE_INSTANCE.md`'s row; the profile comment in
  `Cargo.toml`; book `getting-started.md`; KM card `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.18` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — before `.13.c`, `docs/DOCTRINE_INSTANCE.md` §10 lacked 1 of the driver's 20
  doctrines and the book's table 5, under a fully green gate; after it, all three agree on ids, tiers, checks and
  kinds.
- [x] **ROOT CAUSE (WHY + WHERE)** — nothing compared the tables with the registry: HEAD's
  `scripts/check_doctrines.sh` `DOCTRINES` array had no reader but the driver. RED 5/5, every file restored
  byte-identically: a tier never compared, a kind never compared, an unregistered row accepted (each fails `cargo
  test -p xtask --lib`), the new doctrine's book row renamed (refused end to end by
  `scripts/check_doctrine_tables.sh`, exit 1, from both sides), the shim running another check.
- [x] **ADDRESSED (verified)** — registered doctrines 20 -> 21, each held to both tables; self-test 8 cases.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 53
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — both doctrine tables' new row; book `getting-started.md`; KM cards
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask` and `gate-and-replay-costs-are-affordable` (its
  `reverify` expected a fixed count of executed doctrines, already stale at 20).

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.13.d` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `OWNERSHIP-CITATIONS` doctrine ran only as Perl
  (`scripts/check_ownership_citations.pl`); baseline on 36 inputs in `.project-data/tmp/rdx-13d/old/`; `JSON::PP`'s
  acceptance measured on 21 edge inputs.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 5 ported gates, and
  `pyjson` had no reader for the JSON `JSON::PP` accepts. RED 6/6, every file restored byte-identically with a
  fresh mtime: a lone surrogate accepted by the strict reader, a historical citation checked as an owner, a link
  required to start its parenthesis, an uppercase leaf segment accepted, a same-line `Status` taken for the
  two-line grammar, the shim running another check; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 5 -> 6; identical output on 35 of 36 inputs, the other a Perl memory
  address; the pyjson corpus still 14,964/14,964.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 50
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md`'s `OWNERSHIP-CITATIONS` row; book `getting-started.md`; KM card
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.13.c` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `CONTRACT-ENCODING` doctrine ran only as Python
  (`scripts/check_contract_encoding.py`); baseline on 23 inputs in `.project-data/tmp/rdx-13c/old/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 4 ported gates. RED 6/6,
  every file restored byte-identically with a fresh mtime: the option order changed, separators labelled without a
  space, a re-encoded hand-authored contract accepted, a vanished declaration ignored, a repeated flattened path
  counted twice, the shim running another check; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 4 -> 5; identical output on 23 of 23 inputs; `--write` byte-identical;
  doctrine-table rows missing 1 + 5 -> 0.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 43
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md`'s new `CONTRACT-ENCODING` row; the book's
  `reference/doctrine-enforcement.md` (five rows) and `getting-started.md`; KM card
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.17` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — `serde_json` reworded Python's JSON errors (`.13.b`'s one divergence, 26 of 27
  inputs identical) and cannot reproduce `json.dumps`: it sorts keys and writes `1e16` as `10000000000000000`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no Python-faithful JSON: HEAD's `xtask/src/checks/task_node_retention.rs:95`
  parsed with `serde_json::from_str`. RED 6/6, every file restored byte-identically with a fresh mtime: the `\\u`
  bound made inclusive, a repeated key kept twice, a trailing comma in an object accepted, DEL left unescaped under
  `ensure_ascii`, the scientific threshold at 16, zero compared by its sign; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — corpus agreement 14,964/14,964; `task-node-retention` parity 26 -> 27 of 27;
  xtask dependencies 1 -> 0.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 38
  passed; `Cargo.lock` as before `.13.b`; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — the tree's Non-Goals, `.13` and `.13.b`; `xtask/Cargo.toml` and the profile comment in
  `Cargo.toml`; KM card `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.13.b` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `TASK-NODE-RETENTION` doctrine ran only as Python
  (`scripts/check_task_node_retention.py`); baseline on 27 inputs in `.project-data/tmp/rdx-13b/old/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 3 ported gates, and HEAD's
  `scripts/check_production_genericity_dependencies.pl:115` refused any xtask dependency, so no port could read
  JSON. RED 5/5, every file restored byte-identically with a fresh mtime: a descendant matched without its dot
  boundary, a declared removal ignored, an id stopped at a newline, missing fields left unsorted, the shim running
  another check; each fails `cargo test -p xtask --lib`.
- [x] **ADDRESSED (verified)** — ported gates 3 -> 4; identical output on 26 of 27 inputs, the other only serde's
  wording of a JSON error; `DEPENDENCIES` self-test 9 -> 11 (a new crate refused, `serde_json` admitted).
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 34
  passed; `Cargo.lock` changes only by xtask's `serde_json` edge; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes, and the product's dependency graph is
  unchanged.
- [x] **LOCKSTEP** — the tree's Goal and Non-Goals; `docs/DOCTRINE_INSTANCE.md`'s `TASK-NODE-RETENTION` row; book
  `getting-started.md`; KM card `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`; the profile
  comment in `Cargo.toml`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.13.a` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — the `SECTION-ANCHORS` doctrine ran only as Perl (`scripts/check_section_anchors.pl`);
  baseline on 18 inputs in `.project-data/tmp/rdx-13a/old/`.
- [x] **ROOT CAUSE (WHY + WHERE)** — no native check: `xtask/src/checks/mod.rs` listed 2 ported gates. RED 5/5, every
  file restored byte-identically with a fresh mtime: backticks kept by normalization, any target accepted, a quoted
  `§` counted, a heading required to equal the section, the shim running another check; each fails `cargo test -p
  xtask --lib`. Unoptimized, the port took 0.88 s -> 0.11 s optimized (`Cargo.toml` `[profile.dev.package.xtask]`).
- [x] **ADDRESSED (verified)** — ported gates 2 -> 3; identical output on 16 of 18 inputs, the two others only the
  script's `errno`-valued exit code (25 -> 2).
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 27
  passed; the production-genericity `DEPENDENCIES` check and its self-test 9/9 pass with the profile table;
  `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `docs/DOCTRINE_INSTANCE.md`'s `SECTION-ANCHORS` row; book `getting-started.md`; KM card
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Acceptance Checklist (enforced) — `RUSTDOC-AND-XTASK.7` (CODE) — DONE `2026-10-23`

- [x] **REPRODUCE / MEASURE** — no coverage gate: a new undocumented public item passed every check, and `.1`'s
  published baseline summed workspace dependencies (`specforge` 2,922, really 39; `specforge-conformance` 2,883,
  really 758).
- [x] **ROOT CAUSE (WHY + WHERE)** — HEAD's `xtask/src/checks/rustdoc.rs:12` documented under `-D warnings` with no
  `missing_docs`, and `-D warnings` turns that lint into an error whatever the order, so the gate's build could not
  count; `.1` counted with `clippy -p` and a text grep. RED 6/6, every file restored byte-identically with a fresh
  mtime: `missing_docs` unrecognised, an increase allowed, a decrease banked, the lint switched off (each fails
  `cargo test -p xtask --lib`); a ceiling raised by one and an undocumented `xtask` item (each refused by
  `scripts/check_rustdoc.sh`, exit 1).
- [x] **ADDRESSED (verified)** — ratcheted packages 0 -> 5; undocumented `xtask` items 12 -> 0; the gate's verdict
  on the current tree PASS, 72 s once after the flag change and 1 s warm.
- [x] **NO REGRESSION** — `cargo fmt --check` and `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 / 1,943 and `xtask` 20
  passed; `check_doctrines.sh --gate` PASS at commit.
- [x] **GENERICITY (ADR 0006)** — tooling only; no product code changes.
- [x] **LOCKSTEP** — `.1`'s coverage bullet corrected; the driver's `RUSTDOC` entry; `docs/DOCTRINE_INSTANCE.md` and
  the book's `reference/doctrine-enforcement.md` rows; `scripts/check_rustdoc.sh`; KM card
  `the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask`.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `RUSTDOC-AND-XTASK.15.i` | `pending` | the 393-line canonical collection catalogs and their claim-kit consumer |
| 2 | `RUSTDOC-AND-XTASK.15.j` | `pending` | the derived-state authority adapter and its test, 679 lines together |
| 3 | `RUSTDOC-AND-XTASK.15.k` | `pending` | the 574-line derived-state contract checker, before its test |
| 4 | `RUSTDOC-AND-XTASK.15.l` | `pending` | the 530-line derived-state contract test matrix |
| 5 | `RUSTDOC-AND-XTASK.15.m` | `pending` | the 675-line Knowledge Map shard contract |
| 6 | `RUSTDOC-AND-XTASK.15.n` | `pending` | the 706-line validation-snapshot currentness check |
| 7 | `RUSTDOC-AND-XTASK.15.o` | `pending` | the 746-line task-tree archive check |
| 8 | `RUSTDOC-AND-XTASK.15.p` | `pending` | the 751-line Source PDF registry currentness check |
| 9 | `RUSTDOC-AND-XTASK.15.q` | `pending` | the 757-line corpus KB currentness check |
| 10 | `RUSTDOC-AND-XTASK.15.r` | `pending` | the 790-line FSMGen feedback protocol |
| 11 | `RUSTDOC-AND-XTASK.15.s` | `pending` | the 1,169-line roadmap projection contract |
| 12 | `RUSTDOC-AND-XTASK.15.t` | `pending` | the 1,554-line core surface checker, before its test |
| 13 | `RUSTDOC-AND-XTASK.15.u` | `pending` | the 1,559-line core surface test matrix |
| 14 | `RUSTDOC-AND-XTASK.15.v` | `pending` | the 1,729-line fact-card catalog |
| 15 | `RUSTDOC-AND-XTASK.15.w` | `pending` | the 2,579-line rolling-ledger protocol |
| 16 | `RUSTDOC-AND-XTASK.15.x` | `pending` | the 2,756-line active-task-evidence checker, before its wrapper |
| 17 | `RUSTDOC-AND-XTASK.15.y` | `pending` | the 37-line eight-contract wrapper, after `.15.x` |
| 18 | `RUSTDOC-AND-XTASK.15.z` | `pending` | the 90-line orchestrator, last so every component runs in-process |
| 19 | `RUSTDOC-AND-XTASK.19` | `pending` | the behavioral contract's 0.6 s over its script, measured in `.15.d` |

## Decisions

- `2026-10-23`: the director asked for the rustdoc API reference and an incremental move to `cargo xtask`; `.1`
  records the measured shape and the rules each later leaf follows.
- `2026-10-27`: `.15.1` opens `.15.f`-`.15.z` after measuring all 20 direct components, their external callers
  and the derived-state authority helper hidden behind a `File::Spec` path; producer-before-caller ordering and
  stable kit interfaces are mandatory.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-10-27` | `.15.h` | `cargo test -p xtask --lib`; check/self-test/full-write/refusal parity; Rust oracles and focused doctrine gates | 115/115 and self-test 13/13; every comparison and the complete generated trees byte-identical; malformed-marker, stale-part and nonexistent-root refusals agree |
| `2026-10-27` | `.15.g` | `cargo test -p xtask --lib`; check/self-test/argument/root/unknown-owner parity; Rust oracles and focused doctrine gates | 113/113 and self-test 17/17; every comparison byte-identical in stdout, stderr and exit status; all 16 refusal classes pass |
| `2026-10-27` | `.15.f` | `cargo test -p xtask --lib`; success/argument/collision-RED parity; Rust oracles and focused doctrine gates | 111/111; all three old/new runs byte-identical in stdout, stderr and exit status; all 8 integration checks and every assertion RED control pass |
| `2026-10-27` | `.15.1` | measurement over all 20 direct components, tracked callers and script-name closure; `check_doctrines.sh --fast`; `check_doctrines.sh --only LIVE-DOC-SIZE` | 20 / 18,539 lines plus one 268-line helper; `.15.f`-`.15.z` opened with producers before callers; both focused gates PASS |
| `2026-10-23` | `.1` | rustdoc build under `-D warnings`; `missing_docs` per crate; scripts inventory | fails on one link; counts and inventory recorded |
| `2026-10-23` | `.2` | `RUSTDOC` doctrine; RED with the link restored; oracles | PASS; RED fails as it must |
| `2026-10-23` | `.3` | `run_docs_ci.sh`; the built page's API links | PASS; four links resolve |
| `2026-10-23` | `.4` | xtask tests; dispatcher parity with the scripts | 3/3; identical output and exit codes |
| `2026-10-23` | `.5` | xtask tests; native docs parity with HEAD's script | 7/7; identical on success and with mdbook absent |
| `2026-10-23` | `.6` | the 20 doctrines' direct script trees | ranked; waves set |
| `2026-10-23` | `.8` | xtask tests; RUSTDOC parity through the driver's invocation | 10/10; identical on three inputs |
| `2026-10-23` | `.16` | each doctrine's script against the five kit manifests | 4 kit artifacts plus the driver and hooks; 16 to port |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.15.h` | `RUSTDOC-AND-XTASK.15.h — cargo xtask check task-tree-catalog replaces the catalog generator` | the 374-line generator native; full write and generated trees byte-identical |
| `.15.g` | `RUSTDOC-AND-XTASK.15.g — cargo xtask check fsmgen-request-bundles replaces the request-bundle checker` | the 355-line checker and all 17 cases native; exact argument and refusal parity |
| `.15.f` | `RUSTDOC-AND-XTASK.15.f — cargo xtask check knowledge-map-projection-tests replaces the projection suite` | the 94-line integration suite native; success, argument and collision-RED parity |
| `.15.1` | `RUSTDOC-AND-XTASK.15.1 — LIVE-DOC-SIZE's component leaves, measured and ordered (DESIGN)` | 20 direct components plus one helper; `.15.f`-`.15.z` opened |
| `.1` | `RUSTDOC-AND-XTASK.1 — measure rustdoc and the scripts, decide the API reference and xtask (DESIGN)` | measurement and decisions |
| `.2` | `RUSTDOC-AND-XTASK.2 — rustdoc builds warning-free, and a commit that breaks it is refused` | the link escaped; the `RUSTDOC` gate |
| `.3` | `RUSTDOC-AND-XTASK.3 — the API reference is published beside the mdBook` | the docs build and the book chapter |
| `.4` | `RUSTDOC-AND-XTASK.4 — cargo xtask dispatches to the current tooling` | the `xtask` crate and alias |
| `.5` | `RUSTDOC-AND-XTASK.5 — cargo xtask docs builds the book and the API reference natively` | the first native command; the docs script a shim |
| `.6` | `RUSTDOC-AND-XTASK.6 — the order the doctrine gates move to cargo xtask (DESIGN)` | four waves; parity rules |
| `.8` | `RUSTDOC-AND-XTASK.8 — cargo xtask check rustdoc replaces the RUSTDOC gate's script` | the first ported gate |
| `.16` | `RUSTDOC-AND-XTASK.16 — a portable architecture's kit stays portable (DESIGN)` | `.9`-`.11` retired; `.15` rescoped |

## Changelog

- `2026-10-27`: `.15.h` moves the task-tree catalog generator to `cargo xtask check` with exact write,
  marker, stale-part and root parity.
- `2026-10-27`: `.15.g` moves the FSMGen request-bundle checker and its 17 cases to `cargo xtask check` with
  exact argument, root and refusal parity.
- `2026-10-27`: `.15.f` moves the Knowledge Map projection integration suite to `cargo xtask check` with exact
  success, ignored-argument and collision-RED parity.
- `2026-10-27`: `.15.1` measures and orders the `LIVE-DOC-SIZE` component ports as `.15.f`-`.15.z`.
- `2026-10-23`: Created task tree.
