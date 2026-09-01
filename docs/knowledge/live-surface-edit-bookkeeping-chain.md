---
id: live-surface-edit-bookkeeping-chain
title: Editing a live surface triggers a fixed chain of derived-state refreshes, and skipping any link fails the doctrine gate
answers:
  - "what else must I update after editing CHANGES.md (the prepend shifts every line-pinned region in doctrine/claim_verification/current_claim_census.jsonl - re-anchor them by CONTENT, never by offset - and the new line 1 needs its own excluded evidence record with scope_reason dated_rolling_ledger_evidence and an evidence_id suffixed with the first 12 hex of its line sha256)"
  - "what must I update after editing an mdBook chapter (re-anchor doctrine/claim_verification/book_quantitative_claims.jsonl regions by content; any NEW candidate line needs its own region record plus a bumped expected_candidate_lines; and the book's line/byte totals stale the shipped_behavior aggregate_change authority in doctrine/live_document_size/surfaces.jsonl)"
  - "what breaks when I change doctrine/live_document_size/surfaces.jsonl (three separate surface_registry source pins go stale - in published_assertions.jsonl, book_quantitative_claims.jsonl and current_claim_census.jsonl - plus the durability.artifacts digests in claims.jsonl. Refresh the pins first, then the claim digests, then re-run the gate)"
  - "what must I update after adding a Knowledge Map fact card (regenerate the projection with knowledge-map/scripts/gen_knowledge_map.sh, refresh fact_card_catalog.json planned_outputs from check_fact_card_catalog.pl --print-plan then --write, and bump the fact-card-catalog-count published assertion in published_assertions.jsonl together with its docs/knowledge/INDEX.md line-3 region sha)"
  - "why is a shipped_behavior aggregate_change rationale rejected (it is capped at 512 bytes; the whole surfaces.jsonl record is bounded by max_scalar_bytes)"
  - "why did my rolling-ledger rollover transaction fail with staged output identity drift for manifest.jsonl (a non-ASCII byte in the plan's reason: the manifest writer emits without a UTF-8 layer, so one em dash breaks the staged manifest's identity check and the whole transaction rolls back to exact preflight bytes. A plan reason must be pure ASCII)"
  - "what happens to census evidence records when a rolling ledger rolls over (the records whose regions were sealed into the segment are RETIRED from the census - the bytes live on byte-exact in the segment - and one new record is registered for the new live line 1)"
  - "how many lines may MEMORY.md be (50; the active_resume surface caps lines_each and lines_total at 50 and the memory-arch check fails the commit above it, so durable procedure belongs in a fact card or a task tree rather than in the resume pointer)"
date: 2026-09-01
status: current
tags: [doctrine, live-document-size, claim-verification, rolling-ledger, knowledge-map, workflow, bookkeeping]
evidence: "scripts/check_doctrines.sh; scripts/check_current_claim_census.pl; scripts/check_book_quantitative_claims.pl; scripts/check_fact_card_catalog.pl; scripts/check_rolling_ledger_protocol.pl; scripts/check_live_document_size.sh; doctrine/live_document_size/surfaces.jsonl; doctrine/claim_verification/claims.jsonl; docs/tasks/CHANGES-LEDGER-ROLLOVER.md (.7)"
reverify: "bash scripts/check_doctrines.sh"
---

# The chain a live-surface edit sets off

SpecForge's live documents are covered by derived-state contracts that pin exact line ranges and content
digests. An ordinary slice that edits `CHANGES.md`, an mdBook chapter, `MEMORY.md` or a fact card therefore
also has to refresh everything that pinned what it moved. The gate catches every omission, but only one at a
time, so knowing the whole chain up front turns several failed gate runs into one.

**Order matters, because later links depend on earlier ones.**

1. **Content edits first.** Finish every prose change before touching any contract.
2. **Re-anchor pinned regions by CONTENT.** `current_claim_census.jsonl` and `book_quantitative_claims.jsonl`
   pin `line_range_sha256` regions. A `CHANGES.md` prepend shifts every one of them by the entry's line count;
   a book edit shifts that file's. Find each recorded digest's new location and rewrite the line numbers —
   never recompute the digest from the old offset, which would silently re-point a claim at different text.
3. **Register what is genuinely new.** A new `CHANGES.md` opening record needs an `excluded` evidence record
   (`scope_reason: dated_rolling_ledger_evidence`). A new book candidate line needs its own region record and
   a bumped `expected_candidate_lines`.
4. **Refresh aggregate authorities.** Book line/byte totals stale `shipped_behavior`'s `aggregate_change` in
   `surfaces.jsonl`: set `baseline` to the previous baseline plus delta, compute the new delta, and give it
   this slice's `authority_id`, `owner` and a `rationale` **under 512 bytes**.
5. **Refresh the pins on `surfaces.jsonl` itself.** Changing it stales a `surface_registry` source digest in
   **three** files — `published_assertions.jsonl`, `book_quantitative_claims.jsonl`,
   `current_claim_census.jsonl`.
6. **Fact-card side effects.** A new card changes the catalog: regenerate the projection, refresh
   `fact_card_catalog.json` `planned_outputs`, and bump both the `fact-card-catalog-count` published assertion
   and its `docs/knowledge/INDEX.md` line-3 region digest.
7. **Claim digests last.** `claims.jsonl` `durability.artifacts` covers `CHANGES.md`, `MEMORY.md`,
   `surfaces.jsonl`, `fact_card_catalog.json` and both census files. Refresh after everything else, then read
   off which claim IDs changed — that set *is* the commit's `Published-claims:` declaration.

## Two traps that cost a whole transaction

**A rollover plan's `reason` must be pure ASCII.** The manifest writer emits without a UTF-8 layer, so a single
em dash makes the staged manifest fail its own identity check and the entire rollover rolls back — correctly,
to exact preflight bytes, but the diagnostic (`staged output identity drift`) names the file rather than the
cause. A rollover also **retires** the census evidence records whose regions it sealed, since those bytes now
live in the segment, and needs one new record for the new live line 1.

**`MEMORY.md` is capped at 50 lines.** The `active_resume` surface enforces it and `memory-arch` fails the
commit above it. Durable procedure — this card, for instance — belongs in the Knowledge Map or a task tree, not
in the bounded resume pointer.

Links: [[chain-currency-doctrine]], [[current-claim-census-freeze]], [[mdbook-quantitative-census-freeze]].
