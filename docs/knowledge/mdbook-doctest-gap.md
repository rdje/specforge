---
id: mdbook-doctest-gap
title: The live book classifies illustrative fences explicitly and passes mdBook doctests
answers:
  - "does mdbook test pass for the SpecForge book"
  - "why does mdbook test interpret ISF and console examples as Rust"
  - "how many SpecForge mdBook doctests currently fail"
  - "is mdbook test part of the canonical SpecForge CI gate"
  - "which task owns mdBook fence classification and doctest enforcement"
date: 2026-08-08
status: current
tags: [mdbook, doctest, documentation, ci, fence-classification]
evidence: docs/tasks/MDBOOK-DOCTEST-HYGIENE.md; scripts/run_docs_ci.sh; scripts/run_ci.sh; docs/book/src
reverify: "bash scripts/run_docs_ci.sh"
---

The `.0` baseline showed that `mdbook build docs/book` and canonical full CI passed while the separately invoked
`mdbook test docs/book` failed 26 blocks across four chapters: five in the ISF adapter chapter, 18 in the
historical R16 temporal-intent chapter, one in validation, and two in troubleshooting. The failures were ISF
S-expressions, console output, diagrams/formulas, and incomplete illustrative Rust APIs/types that rendered
correctly but were sent to rustdoc because their fence classification was absent or too strong.

`MDBOOK-DOCTEST-HYGIENE.1` now classifies all 34 openings in those chapters explicitly. Non-Rust material is
`text`; ten deliberately incomplete Rust fragments are `rust,ignore`; three self-contained Rust examples remain
executable `rust`; existing Bash/text blocks remain unchanged. Example bodies are byte-identical, and both
mdBook test and build pass. `MDBOOK-DOCTEST-HYGIENE.2` then placed the native doctest command immediately before
the HTML build in `scripts/run_docs_ci.sh`; the full `scripts/run_ci.sh` gate already invokes that docs entrypoint
exactly once. Fence classification therefore cannot drift while canonical docs CI remains green, and the bounded
task tree is closed.
