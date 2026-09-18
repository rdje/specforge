---
id: llm-primary-recall-ceiling
title: The LLM-primary constraint path is shown 18.4% of the obligations that exist — its precision work operates on a fifth of the population
answers:
  - "what fraction of a document's obligations does the LLM-primary constraint promotion ever see (18.4% on the measured stratum — 326 statements state an obligation about a signal the document declares and the promotion is shown 60 of them; 10.6% on the historical stratum, 1113 to 118)"
  - "why can the LLM-primary path never find a constraint the deterministic extractors missed (promote_constraints builds its universe from the distinct source_text of the constraints ALREADY persisted, one provider call each, so a statement that produced no Pattern constraint is never put in a prompt; LLM-PRIMARY-PROMOTION.1 made it a refinement pass by design)"
  - "what is the LLM recall ceiling per document (AXI ihi0022_l_2025_08 209 to 37 = 17.7%, AHB ihi0033_c 57 to 10 = 17.5%, I2C um10204 27 to 3 = 11.1%, APB ihi0024_e 21 to 9 = 42.9%, ADIv6 ihi0074_a 12 to 1 = 8.3%)"
  - "how should a precision figure from the LLM-primary path be read (as a statement about 18% of the population — every .3j result, the 3/7 gate precision, the 36 ungrounded subjects, the 16 carried names, the 7 row-keyed records, measures model behaviour on what the deterministic extractors already found)"
  - "is the extraction-quality bottleneck the model or the deterministic extractors (the deterministic extractors — perfect model precision cannot move the 82% of obligations that never reach a prompt, because recall upstream decides what is proposed)"
  - "how is the obligation denominator built so the recall gap is not inflated (two steps: this repository's RFC-2119 vocabulary — must/shall whole-word plus the modal phrase required to — then keep only statements that also name a catalog-declared signal, decided by declared_signal_catalog and token_occurrences; 1309 drops to 326 at the second step on the measured stratum)"
  - "how do I re-run the LLM recall ceiling measurement (cargo test -p specforge-core --lib llm_recall_ceiling -- --ignored --nocapture)"
date: 2026-09-18
status: current
tags: [extraction-quality-gauge, llm-primary, recall, census, measurement, method]
evidence: docs/tasks/extraction-quality-gauge/llm-path-sealed.md (.3j.3); crates/specforge/src/commands/extract_constraints_llm.rs (promote_constraints); crates/specforge/src/ir/constraint_extract_llm.rs (llm_recall_ceiling_local_measurement, token_occurrences); crates/specforge/src/ir/entity_typing.rs (declared_signal_catalog); crates/specforge/src/ir/evidence.rs (is_descriptive_narration_binding, the must/shall vocabulary); docs/book/src/commands/pipeline.md
reverify: "cargo test -p specforge-core --lib llm_recall_ceiling -- --ignored --nocapture — expect MEASURED 5 documents / 1309 obligation statements / 326 about a declared signal / 60 shown / ceiling 18.4% / 266 unreachable, and HISTORICAL 33 / 8763 / 1113 / 118 / 10.6% / 995"
---

`promote_constraints` opens by collecting the distinct `source_text` of the constraints **already
persisted**, and issues one provider call per distinct sentence. That is deliberate — it is a refinement
of what the deterministic Pattern extractors found, never a discovery pass. The consequence is absolute: a
statement those extractors produced no constraint from is a statement the model is never shown. No prompt,
no proposal, no chance.

What was missing is how much that gives up.

## Measured 2026-09-18

| stratum | documents | obligation statements | about a declared signal | shown to the model | ceiling |
| --- | ---: | ---: | ---: | ---: | ---: |
| measured | 5 | 1,309 | 326 | 60 | **18.4%** |
| historical | 33 | 8,763 | 1,113 | 118 | 10.6% |

Per document on the measured stratum: AXI `ihi0022_l_2025_08` 209 → 37 (17.7%), AHB `ihi0033_c` 57 → 10
(17.5%), I2C `um10204` 27 → 3 (11.1%), APB `ihi0024_e` 21 → 9 (42.9%), ADIv6 `ihi0074_a` 12 → 1 (8.3%).

The historical row is dated evidence under `ADR 0048` §3 and is not publishable as current.

## The second step of the denominator is what makes the number defensible

Counting every `must`/`shall` statement would have given a ceiling of 60/1,309 — about 4.6% — and it would
have been wrong. Most obligations in a chip specification are about protocol behaviour, not about a named
wire, and could never have produced a signal constraint. Keeping only statements that also **name a
catalog-declared signal**, using `declared_signal_catalog` and `token_occurrences`, drops 1,309 to 326.
The remaining population is the one the extractor is actually accountable for.

## What it means for every precision figure on this path

`EXTRACTION-QUALITY-GAUGE.3j` and its family measured the model's behaviour carefully: 3 of 7 positional
gate refusals correct, 36 ungrounded subjects of 149 records, 16 carried names, 7 row-keyed records. Every
one of those is a statement about **18% of the obligations that exist**. Improving what the model does
with what it sees cannot move the other four-fifths; that is the deterministic extractors' recall, and it
is upstream of anything the promotion can do. `[[measured-stratum-promotion-population]]` sizes the other
half of the same picture: 62 provider calls over 5 documents.
