---
id: mdbook-doctest-gap
title: The live book builds cleanly but illustrative fences currently fail mdBook doctests
answers:
  - "does mdbook test pass for the SpecForge book"
  - "why does mdbook test interpret ISF and console examples as Rust"
  - "how many SpecForge mdBook doctests currently fail"
  - "is mdbook test part of the canonical SpecForge CI gate"
  - "which task owns mdBook fence classification and doctest enforcement"
date: 2026-08-08
status: current
tags: [mdbook, doctest, documentation, ci, fence-classification]
evidence: docs/tasks/MDBOOK-DOCTEST-HYGIENE.md (.0); scripts/run_ci.sh; scripts/build_docs.sh; docs/book/src
reverify: "mdbook test docs/book"
---

`mdbook build docs/book` and the canonical full CI pass, but the separately invoked `mdbook test docs/book`
currently fails 26 blocks across four chapters: five in the ISF adapter chapter, 18 in the historical R16
temporal-intent chapter, one in the validation chapter, and two in troubleshooting. The failing blocks include
ISF S-expressions, console output, diagrams, and incomplete illustrative Rust APIs/types. They render correctly
as documentation but lack a truthful fence classification, so mdBook sends them to rustdoc as Rust.

This is a latent documentation-test coverage gap, not an `ARTIFACT-PATH-PORTABILITY.2` regression. The canonical
`scripts/run_ci.sh` path invokes the book build, current-truth, membership, and size gates but not mdBook's
doctest command. `MDBOOK-DOCTEST-HYGIENE` owns semantic fence classification first, then adoption of a green
doctest gate. Its repair must not bulk-ignore blocks or delete examples merely because rustdoc rejects them.
