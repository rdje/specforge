---
id: the-api-reference-is-rustdoc-and-tooling-moves-to-cargo-xtask
title: The API reference is the workspace's rustdoc beside the mdBook, and repository tooling moves to cargo xtask one command at a time
answers:
  - "how is the SpecForge API reference built (cargo doc, rustdoc)"
  - "where is the API reference (generated/mdbook/specforge/api, reference/api-reference.md)"
  - "does cargo doc pass with -D warnings (CLK_I unresolved link)"
  - "what is the RUSTDOC doctrine / check_rustdoc.sh"
  - "what is cargo xtask in SpecForge / RUSTDOC-AND-XTASK"
  - "which cargo xtask commands exist (doctrines, ci, docs)"
  - "in what order do the doctrine gates move to cargo xtask"
  - "what is cargo xtask check (rustdoc, readme-policy, section-anchors, task-node-retention, contract-encoding, ownership-citations, published-assertions, residual-actionability, corpus-frontier, constraint-part-span, project-data-locality, chain-currency, proof-seal-currency, proof-seal-total, production-genericity)"
  - "which crates may xtask depend on (serde_json, sha2)"
  - "how do the xtask ports read JSON like Python (pyjson)"
  - "what checks that the doctrine tables match the driver registry (DOCTRINE-TABLES)"
  - "can a claim's RED region live outside its producer (source_region.path)"
  - "which scripts are not ported to xtask (portable architecture kits)"
  - "how many scripts does SpecForge have and who calls them"
  - "how many public items lack documentation (missing_docs)"
  - "what is the documentation coverage ratchet (coverage_ceilings.tsv)"
  - "is a ported script deleted"
date: 2026-10-27
status: current
tags: [tooling, rustdoc, api-reference, xtask, scripts, ci, measured]
evidence: docs/tasks/RUSTDOC-AND-XTASK.md (.1, .2, .7, .12, .13.a-.13.f, .14.a-.14.g, .15.0-.15.1, .17, .18, .19); scripts/run_ci.sh (cargo doc step); scripts/check_rustdoc.sh; .project-data/tmp/rdx-1/
reverify: "Run RUSTDOCFLAGS='-D warnings' cargo doc --offline --workspace --no-deps; list scripts/*.{py,pl,sh} and grep scripts/check_doctrines.sh for the ones it calls"
---

**Rustdoc (`2026-10-23`).** CI runs `cargo doc --workspace --no-deps` under `-D warnings` (`scripts/run_ci.sh`), and
it failed on one unresolved intra-doc link: `[CLK_I]` in a doc comment quoting Wishbone's *rising [CLK_I] edge*,
written on `2026-09-26`. That was the workspace's only rustdoc warning. Only the push-time CI ran rustdoc, so the
break went unseen for four weeks. Since `RUSTDOC-AND-XTASK.2` the quotation is escaped and the `RUSTDOC` doctrine
(`scripts/check_rustdoc.sh`, gate tier) runs CI's command at every commit, after a self-test that requires a scratch
crate with an unresolved link to fail; 0.3 s unchanged, about 75 s when `specforge-core` is re-documented. The four
crates build in 45 s cold into 161 MiB of `target/doc`. `specforge-core` compiles the `ir` sources through `#[path]`,
so it is the library API. **Coverage (`.7`, corrected).** Undocumented public items, counted per package from the
rustdoc gate's own build: `specforge-core` 2,124, `specforge-conformance` 758, `specforge` 39,
`specforge-production-graph` 33, `xtask` 0. `.1` had published 2,922 for `specforge` and 2,883 for
`specforge-conformance`: `cargo clippy -p <crate> -- -W missing_docs` also lints the workspace crates a package
depends on, and a text grep summed them (2,922 = 39 + 758 + 2,125). The counts are a ratchet, not a lint switched on:
the `RUSTDOC` gate refuses any package whose count differs from its ceiling in
`doctrine/rustdoc/coverage_ceilings.tsv`, so an undocumented item cannot be added and a documented one lowers the
ceiling.

**Decided (`.1`).** The rustdoc of the four crates is built into `generated/mdbook/specforge/api/` beside the book and
linked from a reference page: since `.3`, `scripts/run_docs_ci.sh` does both, and the book's
`reference/api-reference.md` links each crate's index (196 MiB under the gitignored `generated/`). `cargo xtask` is a
workspace member `xtask`, not a default member, aliased in `.cargo/config.toml`. Since `.4` it dispatches `doctrines`,
`ci` and `docs` to their scripts, byte-identical in output and exit code (`xtask/src/lib.rs`, `TASKS`), and the
production-genericity `DEPENDENCIES` check keeps it disconnected from the product and limited to crates the workspace
already locks (since `.13.b`): only `sha2`, for the claim registries' exact digests (`.13.e`), since the ports read
and write JSON through `xtask/src/pyjson.rs`, a reproduction of Python's `json` module proven against Python on 14,964
cases (`.17`), and see Perl's view of values through `xtask/src/perl.rs`; since `.5` `docs` is native
(`xtask/src/docs.rs`, with `project_data.rs` for the project-data roots) and `scripts/run_docs_ci.sh` is its shim; the
gates move one leaf each as `cargo xtask check <doctrine>` (`xtask/src/checks/`; `rustdoc` since `.8`, `readme-policy`
since `.12`, `section-anchors` since `.13.a`, `task-node-retention` since `.13.b`, `contract-encoding` since `.13.c`,
`ownership-citations` since `.13.d`, `published-assertions` since `.13.e`, `residual-actionability` since `.13.f`,
which completes wave 2, `corpus-frontier` since `.14.a`, its adapter and Perl core one module, and
`constraint-part-span` since `.14.b`, its adapter and Python census likewise, and `project-data-locality` since
`.14.c`, which runs the persisted-artifact path tool in-process since `.14.g` (the tool itself
`cargo xtask check persisted-artifact-paths`) and still runs the shell activation's own cases, and
`chain-currency` since `.14.d`, which also holds the one content-identity predicate the rebuild remedy reaches
through `scripts/lib/stage_artifact_identity.sh`, and
`proof-seal-currency` since `.14.e`, which holds the seal reader `scripts/lib/proof_seal_scan.sh` now calls, with
its CI-tier entry `proof-seal-total` since `.14.f`; `doctrine-tables`,
the `DOCTRINE-TABLES` doctrine `.18` added after both doctrine tables were found to have drifted from the driver's
registry, was written natively), in `.6`'s four waves: the small shell gates (`.8`-`.12`), the single-file Perl and
Python gates, the multi-file gates and corpus oracles, then the composites and the driver (wave 4 sized by
`.15.0`: each composite a leaf per component script, its orchestrator last, `PRODUCTION-GENERICITY`'s 3,136 lines
before `LIVE-DOC-SIZE`'s 18,539; `production-genericity-dependencies` since `.15.a`,
`production-genericity-inventory` since `.15.b`, `production-genericity-rules` since `.15.c`,
`behavioral-genericity-contract` since `.15.d`, whose script's problem order varied with Python's hash seed and
whose totals used CPython 3.12's compensated `sum`; `.19` owns its 0.6 s over the script; and
`production-genericity` since `.15.e`, its four xtask components in-process through `checks::Output`, which
captures both streams as `2>&1` did. `LIVE-DOC-SIZE` is next: `.15.1` re-measured its 20 direct components as
18,539 lines and found the 268-line derived-state authority helper behind a `File::Spec` path, then opened
`.15.f`-`.15.z`, producers before callers and the orchestrator last, with every kit path and argv preserved); a port proves
parity on the current tree, every self-test case and a broken input per refusal, and from wave 2 may use crates the
workspace already locks. Since `.16` the port covers SpecForge's own 16 doctrines only: the
kit artifacts of the five portable architectures (`check_memory_architecture.sh`, the `knowledge-map/` bundle,
`check_task_acceptance.sh`, `check_claim_verification.pl`, the driver and the hooks) stay in their portable language,
because other repositories copy them. The `readme-policy` port fixed two fail-opens in the script it replaced: its
`while read` loop skipped a last registry row without its newline, and `IFS=$'\t'` merged runs of tabs, so a row with
an empty extra column read as complete. A claim's RED region may now name the file its producer delegates to
(`source_region.path`, one of the control's inputs), because a shim's RED case lives in the Rust it runs.

**Scripts.** 123 under `scripts/`: 64 Python (23,831 lines), 32 Perl (26,723), 27 shell (4,694). The doctrine driver
calls 23, CI 5, the hooks only the driver; 53 are maintenance tools run by hand. Records cite them 8,089 times, so a
ported script is never deleted: once its xtask command reproduces its output, exit codes and self-tests, it becomes
a shim (`exec cargo xtask …`) and every recorded command keeps working.
