---
id: evidence-signal-declaration-utf8-boundary-panic
title: Evidence signal-declaration catalogs panic when a match follows a multi-byte bullet
answers:
  - "why does USB 3.2 EvidenceIR panic on start byte index is not a char boundary"
  - "where is the U+F0B7 signal integrity panic in evidence extraction"
  - "is collect_known_signal_names UTF-8 safe"
  - "does the explicit-direction signal catalog repeat the UTF-8 boundary bug"
  - "why does slicing idx minus 2 before signal panic"
date: 2026-08-09
status: current
tags: [evidence-ir, utf-8, signal-declaration, panic, usb, corpus-coverage]
evidence: crates/specforge/src/ir/evidence.rs (collect_known_signal_names and collect_signals_with_explicit_direction_declarations); docs/tasks/CORPUS-COVERAGE.md (.2.33a-.2.33b)
reverify: "target/release/specforge evidence generated/source_ir/usb_3_2_revision_1_0_2017_09/source_ir.json; cargo test -p specforge signal_declaration_catalog_utf8_boundary"
---

**Open at `.2.33a`; repair owned by `.2.33b`.** USB 3.2's normalized Markdown contains the list item
`- U+F0B7 Signal integrity ...`. Evidence statement normalization lowercases the ASCII word but preserves the
three-byte private-use bullet. `str::match_indices("signal ")` returns a valid **byte** offset at the match, then
both declaration catalogs test for a preceding `. ` sentence boundary with `lowered[idx - 2..idx]`. Subtracting
two bytes lands inside the bullet, so Rust panics before EvidenceIR is written.

The defect is duplicated in `collect_known_signal_names` and
`collect_signals_with_explicit_direction_declarations`; fixing only the first call site would leave the same panic
latent in the direction-aware path. The correct universal boundary is to take the already-valid prefix ending at
`idx` and ask whether that prefix ends with `. `—never subtract a character count from a byte offset. Production
logic must not name USB, the bullet codepoint, or the surrounding prose. A regression must cover both the private-
use bullet non-declaration and a legitimate `Signal X ...` declaration after a sentence containing non-ASCII text.
