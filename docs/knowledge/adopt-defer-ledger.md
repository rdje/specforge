---
id: adopt-defer-ledger
title: Per-author adopt/defer ledger — what SpecForge takes from / leaves out of each grounded author
answers:
  - "what does SpecForge take from a grounded author"
  - "what does SpecForge take from Docling OpenIE LayoutLM Chao Chow LLVM MLIR GoldMine Texada Pnueli"
  - "what research did SpecForge leave out and why"
  - "what parts of the literature are deferred or flagged as future work"
  - "where is the per-author adopt-vs-defer provenance"
  - "what does SpecForge defer from conformal prediction NLI Dempster Snorkel NoRBERT"
date: 2026-06-04
tags: [grounding, provenance, specification-mining, research]
evidence: docs/research/grounding/adopt-defer-ledger.md; docs/tasks/SPEC-MINING-PROVENANCE.md
reverify: ls docs/research/grounding/adopt-defer-ledger.md
---

`docs/research/grounding/adopt-defer-ledger.md` is the per-author provenance ledger: for every
author/work the literature sweep surfaced, it states **Take** (the abstraction SpecForge
adopts), **Leave out (+why)** (what it deliberately does not use, and the reason), and
**Instantiated at** (where the adopted idea lives in the code/IR). Covers the temporal trio
(Pnueli/GoldMine/Texada), Ammons/Daikon, Docling/TableFormer/DocLayNet/PubTables-1M,
OpenIE/ReVerb/Mintz/Zeng/KBP/Hogan, LayoutLM/Donut/DocVQA/Dempster, GCD/Outlines/RAG/SNLI/
Ji-hallucination/SelfCheckGPT/Garcez-Lamb, Yarowsky/Riloff-Jones/NELL/Snorkel/Parisi,
van-Rijsbergen/MUC/Cohen-κ/Chao/Eick/Petersson, Chow/El-Yaniv/Geifman/Vovk/Guo/Scheirer,
RFC2119/NLP4RE/PROMISE/NoRBERT/ACE/Berry-Kamsties/NASA-ARM, LLVM/MLIR/nanopass, and
PSL/SVA/AssertLLM/HLS.

The synthesis: SpecForge consistently TAKES the abstraction + vocabulary + eval methodology +
honesty stance, and consistently LEAVES OUT what needs data/signals it lacks (traces, RTL,
runs, trained models) and the backward direction — applying the borrowed machinery **forward**
(spec → intent). "Already adopted" closed trees: Chao (`RECALL-CHAO-ESTIMATOR`), weak phrases
(`AMBIGUITY-PHRASE-DETECTOR`), LTL rendering (`TEMPORAL-RULE-LTL-RENDER`), temporal eval
(`TEMPORAL-RULE-EVAL`). See `[[spec-mining-framing]]`.
