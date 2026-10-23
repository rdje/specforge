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
  Children: `RUSTDOC-AND-XTASK.1`-`.16`

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
  Status: `in_progress` (container; wave 3; `.14.a` done)
  Goal: the multi-file gates and corpus oracles move to xtask
  Acceptance: one leaf per doctrine, in order: `CORPUS-FRONTIER`, `CONSTRAINT-PART-SPAN`, `PROJECT-DATA-LOCALITY`,
  `CHAIN-CURRENCY`, `PROOF-SEAL-CURRENCY`, `PROOF-SEAL-TOTAL`. **Known consumer (`.13.e`):**
  `scripts/report_self_test_totals.pl` reads the declared self-test totals of `check_chain_currency.sh`,
  `check_proof_seal_currency.sh` and `rebuild_stage_cascade.sh` from their source and fails closed when it cannot,
  so a port that moves a total out of its script updates that producer in the same leaf.
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
  Status: `pending`
  Goal: `cargo xtask check constraint-part-span` replaces the `CONSTRAINT-PART-SPAN` gate's script
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.14.c`
  Status: `pending`
  Goal: `cargo xtask check project-data-locality` replaces the `PROJECT-DATA-LOCALITY` gate's script
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim; `scripts/project_data_env.sh` stays
  the shared shell library the other scripts source.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.14.d`
  Status: `pending`
  Goal: `cargo xtask check chain-currency` replaces the `CHAIN-CURRENCY` gate's script
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim; `report_self_test_totals.pl` still
  reads its declared total.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.14.e`
  Status: `pending`
  Goal: `cargo xtask check proof-seal-currency` replaces the `PROOF-SEAL-CURRENCY` gate's script
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim; `report_self_test_totals.pl` still
  reads its declared total.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.14.f`
  Status: `pending`
  Goal: `cargo xtask check proof-seal-total` replaces the `PROOF-SEAL-TOTAL` gate's script
  Acceptance: `.6`'s parity evidence, its self-test ported; the script a shim.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.15`
  Status: `pending` (container; wave 4)
  Goal: the composite gates move to xtask
  Acceptance: one leaf per doctrine, in order: `PRODUCTION-GENERICITY`, `LIVE-DOC-SIZE`. Since `.16`,
  `CLAIM-VERIFICATION` and the driver (`check_doctrines.sh`) stay portable kit artifacts and are not ported.
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
| 1 | `RUSTDOC-AND-XTASK.14.b` | `pending` | wave 3's second gate, `CONSTRAINT-PART-SPAN` |
| 2 | `RUSTDOC-AND-XTASK.14.c`-`.14.f` | `pending` | the rest of wave 3, in `.14`'s order |
| 3 | `RUSTDOC-AND-XTASK.15` | `pending` | wave 4: the composites, opening one leaf per doctrine as each starts |

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
| `2026-10-23` | `.16` | each doctrine's script against the five kit manifests | 4 kit artifacts plus the driver and hooks; 16 to port |

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
| `.16` | `RUSTDOC-AND-XTASK.16 — a portable architecture's kit stays portable (DESIGN)` | `.9`-`.11` retired; `.15` rescoped |

## Changelog

- `2026-10-23`: Created task tree.
