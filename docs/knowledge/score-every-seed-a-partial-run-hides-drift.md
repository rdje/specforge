---
id: score-every-seed-a-partial-run-hides-drift
title: A slice must score all eleven WIRE-BASED-100 seeds; .17 scored six and seed_swd fell from 1.000 to 0 unseen for three days
answers:
  - "which WIRE-BASED-100 seeds does a slice have to score"
  - "why did seed_swd's constraint score 0"
  - "why is seed_swd's CSYSPWRUPACK gold must_be_high and not must_be_asserted"
  - "when is a WIRE-BASED-100 gold refined instead of the extractor changed"
  - "can an old specforge binary run eval-extraction on today's artifacts"
date: 2026-09-29
status: current
tags: [wire-based-100, eval, gold, polarity, regression, measured]
evidence: crates/specforge/test_data/llm_eval/seed_swd.json (statement_1247 label_note); docs/tasks/extraction-representation-gaps/recent-leaves.md (.40); docs/book/src/quality/extraction-eval.md (Honesty notes)
reverify: "for s in crates/specforge/test_data/llm_eval/seed_*.json; do target/release/specforge eval-extraction $s --provider skip; done — 11 datasets; seed_swd's source-tolerant signal_constraint line reads tp=1 fp=0 fn=0"
---

There are eleven seeds in `crates/specforge/test_data/llm_eval/`: six AMBA (`seed_apb`, `seed_ahb`, `seed_axi` and
their `_temporal` twins), `seed_swd`, `seed_swd_derivation`, `seed_i2c_signals`, `seed_nvme_registers` and
`seed_riscv_debug_registers`. Run each with `eval-extraction … --provider skip`, compare with the HEAD binary's report
after masking the temp paths, and read every line that moves. Many moves are only the report's stored-relation count
(`existing_actor_signal_relations`).

`EXTRACTION-REPRESENTATION-GAPS.17` (`2026-09-26`, `fc04dd39`) scored the six AMBA seeds. Its section-polarity rule
made ADIv6's `CSYSPWRUPACK` active-high and refined *must be asserted* to `must_be_high`. `seed_swd`'s gold said
`must_be_asserted`, so its one constraint went from tp=1 to fp=1 fn=1, and it stayed unseen until `2026-09-29`.
Building `fc04dd39^` and `fc04dd39` in a scratch worktree showed the move. Old binaries cannot `eval-extraction` today's
persisted EvidenceIR (*proof topology is stale*), so compare their `evidence --dry-run` on the current SourceIR instead.

A gold is refined, not the extractor, when the IR states what the document states and the gold states less. Here the
section says *assert denotes taking the signal HIGH*; `.40` moved the gold to `must_be_high`, as `.25.c` moved APB's
hold to `must_remain_until`. Record the binary's score on both golds.
