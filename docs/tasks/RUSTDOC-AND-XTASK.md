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
  Children: `RUSTDOC-AND-XTASK.1`-`.7`

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
  Status: `pending`
  Goal: the API reference is published beside the mdBook
  Acceptance: the docs build writes the four crates' rustdoc into `generated/mdbook/specforge/api/`; a book
  reference page links each crate's index; `mdbook build` and the link targets are verified.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.4`
  Status: `pending`
  Goal: `cargo xtask` dispatches to the current tooling
  Acceptance: an `xtask` workspace member (not default, standard library only) and the alias; `cargo xtask
  doctrines`, `ci` and `docs` run `check_doctrines.sh`, `run_ci.sh` and `run_docs_ci.sh` with their arguments and
  exit codes; the production-genericity census reads no `xtask` code (measured, not assumed); the book's
  contributor page documents it.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.5`
  Status: `pending`
  Goal: `cargo xtask docs` builds the mdBook and the API reference natively
  Acceptance: the first native command reproduces `run_docs_ci.sh`'s behaviour and output tree; the script becomes
  a shim.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.6`
  Status: `pending` (DESIGN)
  Goal: the order in which the driver's 23 gates move to xtask
  Acceptance: rank the gates by cost and risk (size, self-tests, callers), open one leaf per gate in that order, and
  state each one's parity evidence.
  Verification: `pending`
  Commit: `pending`

- ID: `RUSTDOC-AND-XTASK.7`
  Status: `pending`
  Goal: documentation coverage is a ratchet
  Acceptance: record each crate's undocumented public item count and refuse an increase at commit; the counts in
  `.1` are the baseline.
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

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `RUSTDOC-AND-XTASK.3` | `pending` | the API reference, now that rustdoc is green |
| 2 | `RUSTDOC-AND-XTASK.4` | `pending` | the dispatcher, with no behaviour change |
| 3 | `RUSTDOC-AND-XTASK.5` | `pending` | the first native command |
| 4 | `RUSTDOC-AND-XTASK.6` | `pending` | the gate port order |
| 5 | `RUSTDOC-AND-XTASK.7` | `pending` | the coverage ratchet |

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

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.1` | `RUSTDOC-AND-XTASK.1 — measure rustdoc and the scripts, decide the API reference and xtask (DESIGN)` | measurement and decisions |
| `.2` | `RUSTDOC-AND-XTASK.2 — rustdoc builds warning-free, and a commit that breaks it is refused` | the link escaped; the `RUSTDOC` gate |

## Changelog

- `2026-10-23`: Created task tree.
