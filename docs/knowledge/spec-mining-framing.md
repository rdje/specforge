---
id: spec-mining-framing
title: SpecForge is forward specification mining (spec -> intent, not implementation -> spec)
answers:
  - "what is SpecForge doing in academic or research terms"
  - "is SpecForge specification mining"
  - "how does SpecForge relate to GoldMine Texada Pnueli Ammons"
  - "what does SpecForge take from the spec-mining literature and what does it leave out"
  - "why is SpecForge called forward specification mining"
date: 2026-06-04
tags: [framing, specification-mining, grounding, temporal]
evidence: docs/research/grounding/adopt-defer-ledger.md; docs/book/src/architecture-rationale.md; README.md
reverify: grep -rn "forward specification mining" README.md docs/book/src/architecture-rationale.md
---

SpecForge does **specification mining** (Ammons, Bodík & Larus, POPL 2002, DOI
10.1145/503272.503275 — automatically discovering the formal spec a system obeys) but run
**forward**: the literature recovers a spec *from an implementation* (traces / RTL / code —
backward); SpecForge mines typed design intent *from the human-authored specification document
itself* (prose + tables + figures), before any implementation exists. The machinery is
borrowed from the (backward) literature: the temporal rules are the `G(antecedent →
consequent)` LTL template (Pnueli, FOCS 1977) that GoldMine (DATE 2010) and Texada (ASE 2015)
mine from traces/RTL — SpecForge instantiates it from spec prose instead.

The per-author **Take / Leave-out+why / Instantiated-at** provenance ledger lives at
`docs/research/grounding/adopt-defer-ledger.md`; the book framing is in
`architecture-rationale.md` ("forward specification mining"). See task-tree
`SPEC-MINING-PROVENANCE`. Related: `temporal-rule-ltl-rendering`.
