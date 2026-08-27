---
id: source-ir-ingest-not-reproducible
title: SourceIR ingest is not reproducible across time, and no gate can see it
answers:
  - "why did the Cortex-A76 reviewed source region disappear"
  - "why is exact source region capture 13 of 14"
  - "why does the reviewed prose sit at elem_00230 instead of elem_00219"
  - "is SourceIR covered by the chain currency doctrine"
  - "does check_chain_currency re-run ingest"
  - "why do replayed SourceIR digests always differ between replay roots"
  - "is SpecForge PDF ingest deterministic"
  - "what changed between the persisted source_ir and a fresh ingest"
  - "which task owns the SourceIR reproducibility gap"
  - "why are reviewed fixture anchors fragile"
date: 2026-08-27
status: current
tags: [source-ir, ingest, reproducibility, chain-currency, measurement-integrity, docling]
evidence: docs/tasks/SOURCE-IR-REPRODUCIBILITY.md; docs/tasks/spec-to-intent-alignment/residual-carrier.md; scripts/check_chain_currency.sh; crates/specforge/test_data/source_to_intent_vertical/build_fixture.py
reverify: "python3 -c \"import json; d=json.load(open('generated/source_ir/pjdoc_466751330_7215_10_0_cortex_a76_software_optimization_guide/source_ir.json')); n='it is recommended that GPR registers be filled/spilled to the VPR'; print(len(d['content_elements']), [i['element_id'] for i in d['content_elements'] if n in i['text']])\""
---

`SPEC-TO-INTENT-ALIGNMENT.8d` replayed the reviewed population from clean production and the Cortex-A76 guide's
reviewed prose cell failed its source-region query for the first time. The cell is anchored on the **ordinal**
element id `elem_00219` plus an asserted excerpt; the current toolchain places that prose at `elem_00230`.
Ingest now emits 260 content elements where the persisted chain holds 249 — an 11-element shift, with table
(68), visual-asset (71), and page (46) counts identical. Exact source-region capture therefore reads 13/14.

**The cause is not this repository's code**, and each usual suspect is excluded by direct measurement rather
than by argument:

- **Input** — the PDF is byte-identical throughout, SHA-256 `8358c5ae…3a22`, unchanged on disk since
  `2026-04-02`; the replay orchestrator asserts source digest equality before ingesting.
- **Revision** — `483e525d` and `3d04bde0` (the commit *before* the only revision in range that touched
  `crates/specforge/src/ir/source/docling_backend.rs`) both produce 260. Between the published `.7c.ii`
  revision and `483e525d`, the only change under `crates/specforge/src/ir/source*` is additive.
- **Toolchain** — Docling `2.84.0` and its sibling packages are unchanged on disk since `2026-08-08`, and no
  model blob under `.cache/huggingface` is newer than `2026-08-09`.
- **Batching** — batched (`SPECFORGE_INGEST_BATCH_PAGES=16`) and unbatched ingest both produce 260.
- **Run-to-run noise** — two back-to-back runs are byte-identical after normalizing the replay root, so ingest
  is stable *within* a session.

Two consequences follow, and the second is the structural one.

**Stage digests always differ across replay roots, and that is not drift.** `SourceIR` embeds its own output
paths — `artifact_layout`, `normalization_plan`, every `page_artifacts` and `visual_assets` entry, and the
`proof_context` premises that quote them — so two replays into differently named roots produce different
digests for identical content. Compare replayed artifacts only after normalizing the root; the `.8d` control
did exactly that and found `content_elements`, `structured_tables`, `page_artifacts`, and `visual_assets`
identical.

**`SourceIR` is outside the chain-currency oracle.** `scripts/check_chain_currency.sh` replays each stage at
its **fixed persisted input**: evidence from the persisted `source_ir.json`, semantic from the persisted
`evidence_ir.json`, and so on. It never re-runs ingest, so a 24/24 current chain is a true statement about
EvidenceIR through the ISF adapter and says nothing about the artifact all of them descend from. A change in
ingest can therefore invalidate every persisted `generated/source_ir/*` while every doctrine stays green.

The reviewed fixture behaved correctly throughout: `source_record` in
`crates/specforge/test_data/source_to_intent_vertical/build_fixture.py` refuses an element whose text does not
contain the reviewed excerpt, so it failed closed rather than scoring the wrong element. Re-anchoring the cell
to `elem_00230` would make the fixture agree with whatever ingest currently emits, which is the drift the
anchor exists to detect — so `.8d` published 13/14 instead.

[[source-to-intent-vertical-evaluator]] owns the oracle and
[[required-residual-actionability-denominator]] owns the residual rule this replay was measuring.
`SOURCE-IR-REPRODUCIBILITY` owns the census of standing drift, content-addressed reviewed anchors, and the
chain-currency blind spot at the ingest boundary.
