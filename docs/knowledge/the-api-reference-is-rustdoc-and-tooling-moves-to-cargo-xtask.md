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
  - "how many scripts does SpecForge have and who calls them"
  - "how many public items lack documentation (missing_docs)"
  - "is a ported script deleted"
date: 2026-10-23
status: current
tags: [tooling, rustdoc, api-reference, xtask, scripts, ci, measured]
evidence: docs/tasks/RUSTDOC-AND-XTASK.md (.1, .2); scripts/run_ci.sh (cargo doc step); scripts/check_rustdoc.sh; .project-data/tmp/rdx-1/
reverify: "Run RUSTDOCFLAGS='-D warnings' cargo doc --offline --workspace --no-deps; list scripts/*.{py,pl,sh} and grep scripts/check_doctrines.sh for the ones it calls"
---

**Rustdoc (`2026-10-23`).** CI runs `cargo doc --workspace --no-deps` under `-D warnings` (`scripts/run_ci.sh`), and
it failed on one unresolved intra-doc link: `[CLK_I]` in a doc comment quoting Wishbone's *rising [CLK_I] edge*,
written on `2026-09-26`. That was the workspace's only rustdoc warning. Only the push-time CI ran rustdoc, so the
break went unseen for four weeks. Since `RUSTDOC-AND-XTASK.2` the quotation is escaped and the `RUSTDOC` doctrine
(`scripts/check_rustdoc.sh`, gate tier) runs CI's command at every commit, after a self-test that requires a scratch
crate with an unresolved link to fail; 0.3 s unchanged, about 75 s when `specforge-core` is re-documented. The four
crates build in 45 s cold into 161 MiB of `target/doc`. `specforge-core` compiles the `ir` sources through `#[path]`,
so it is the library API. Undocumented public items (`clippy -W missing_docs`): `specforge-core` 2,119, `specforge`
2,922, `specforge-conformance` 2,883, `specforge-production-graph` 33. Coverage is ratcheted (`.7`), not switched on.

**Decided (`.1`).** The rustdoc of the four crates is built into `generated/mdbook/specforge/api/` beside the book and
linked from a reference page: since `.3`, `scripts/run_docs_ci.sh` does both, and the book's
`reference/api-reference.md` links each crate's index (196 MiB under the gitignored `generated/`). `cargo xtask` is a
workspace member `xtask`, not a default member, standard library only, aliased in `.cargo/config.toml`. Since `.4` it
dispatches `doctrines`, `ci` and `docs` to their scripts, byte-identical in output and exit code (`xtask/src/lib.rs`,
`TASKS`), and the production-genericity `DEPENDENCIES` check keeps it dependency-free and disconnected from the
product; its first native command builds the docs (`.5`); the gates move one leaf each, in an order `.6` sets.

**Scripts.** 123 under `scripts/`: 64 Python (23,831 lines), 32 Perl (26,723), 27 shell (4,694). The doctrine driver
calls 23, CI 5, the hooks only the driver; 53 are maintenance tools run by hand. Records cite them 8,089 times, so a
ported script is never deleted: once its xtask command reproduces its output, exit codes and self-tests, it becomes
a shim (`exec cargo xtask …`) and every recorded command keeps working.
