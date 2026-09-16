---
id: live-surface-edit-bookkeeping-chain
title: Editing a live surface triggers a fixed chain of derived-state refreshes, and skipping any link fails the doctrine gate
answers:
  - "what else must I update after editing CHANGES.md (the prepend shifts every line-pinned region in doctrine/claim_verification/current_claim_census.jsonl - re-anchor them by CONTENT, never by offset - and the new line 1 needs its own excluded evidence record with scope_reason dated_rolling_ledger_evidence and an evidence_id suffixed with the first 12 hex of its line sha256)"
  - "what must I update after editing an mdBook chapter (re-anchor the line_range_sha256 regions in ALL THREE of book_quantitative_claims.jsonl, published_assertions.jsonl and current_claim_census.jsonl by content; any NEW candidate line needs its own region record plus a bumped expected_candidate_lines; and the book's line/byte totals stale the shipped_behavior aggregate_change authority in doctrine/live_document_size/surfaces.jsonl)"
  - "how do I re-pin claim regions after editing a governed file (python3 scripts/repin_claim_regions.py --check then --apply; it resolves by content across all three registries and REFUSES ambiguity rather than taking the first match, which matters because a blank-line region matches every blank line in the file)"
  - "which files pin line-anchored claim regions that a mid-file insert will shift (book_quantitative_claims.jsonl, published_assertions.jsonl and current_claim_census.jsonl - published_assertions is the one that gets forgotten)"
  - "what breaks when I change doctrine/live_document_size/surfaces.jsonl (three separate surface_registry source pins go stale - in published_assertions.jsonl, book_quantitative_claims.jsonl and current_claim_census.jsonl - plus the durability.artifacts digests in claims.jsonl. Refresh the pins first, then the claim digests, then re-run the gate)"
  - "what must I update after adding a Knowledge Map fact card (regenerate the projection with knowledge-map/scripts/gen_knowledge_map.sh, refresh fact_card_catalog.json planned_outputs from check_fact_card_catalog.pl --print-plan then --write, and bump the fact-card-catalog-count published assertion in published_assertions.jsonl together with its docs/knowledge/INDEX.md line-3 region sha)"
  - "why is a shipped_behavior aggregate_change rationale rejected (it is capped at 512 bytes; the whole surfaces.jsonl record is bounded by max_scalar_bytes)"
  - "why did my rolling-ledger rollover transaction fail with staged output identity drift for manifest.jsonl (a non-ASCII byte in the plan's reason: the manifest writer emits without a UTF-8 layer, so one em dash breaks the staged manifest's identity check and the whole transaction rolls back to exact preflight bytes. A plan reason must be pure ASCII)"
  - "what happens to census evidence records when a rolling ledger rolls over (the records whose regions were sealed into the segment are RETIRED from the census - the bytes live on byte-exact in the segment - and one new record is registered for the new live line 1)"
  - "how do I edit one field of a doctrine JSON contract without reformatting the whole file (derive that FILE's encoder by round-tripping candidates against its exact bytes - there is no repository-wide style: of 27 parseable contracts under doctrine/, 20 reproduce under json.dumps with separators comma-space-colon and a trailing newline, 3 under Perl JSON::PP canonical+pretty, and 4 are hand-authored and reproduce under neither)"
  - "which encoder wrote doctrine/live_document_size/fact_card_catalog.json (python3 json.dumps indent=1 sort_keys=True ensure_ascii=True plus a trailing newline - byte-identical to Perl JSON::PP canonical with indent(1) and space_after but WITHOUT space_before. JSON::PP pretty is wrong for it, because pretty also sets space_before and emits key-space-colon)"
  - "which doctrine JSON contracts must never be re-encoded at all (fsmgen_feedback.json, roadmap_projection.json, spec_to_intent_vertical_eval_schema.json and trajectory_controller_input_schema.json are hand-authored and keep small objects inline on one line, which no encoder emits; re-encoding to change one field expands them 9-27% and destroys that layout)"
  - "is Perl or Python the right language for editing a doctrine JSON contract (neither - language is the wrong axis. The bytes are decided by indent width, key order, ensure_ascii, space_before and the trailing newline, and both languages reproduce the same bytes once those are matched)"
  - "how many lines may MEMORY.md be (50; the active_resume surface caps lines_each and lines_total at 50 and the memory-arch check fails the commit above it, so durable procedure belongs in a fact card or a task tree rather than in the resume pointer)"
date: 2026-09-14
status: current
tags: [doctrine, live-document-size, claim-verification, rolling-ledger, knowledge-map, workflow, bookkeeping]
evidence: "doctrine/live_document_size/fact_card_catalog.json; scripts/check_doctrines.sh; scripts/check_current_claim_census.pl; scripts/check_book_quantitative_claims.pl; scripts/check_fact_card_catalog.pl; scripts/check_rolling_ledger_protocol.pl; scripts/check_live_document_size.sh; doctrine/live_document_size/surfaces.jsonl; doctrine/claim_verification/claims.jsonl; docs/tasks/CHANGES-LEDGER-ROLLOVER.md (.7); docs/tasks/CORPUS-CHAIN-CURRENCY.md (.8)"
reverify: "bash scripts/check_doctrines.sh"
---

# The chain a live-surface edit sets off

SpecForge's live documents are covered by derived-state contracts that pin exact line ranges and content
digests. An ordinary slice that edits `CHANGES.md`, an mdBook chapter, `MEMORY.md` or a fact card therefore
also has to refresh everything that pinned what it moved. The gate catches every omission, but only one at a
time, so knowing the whole chain up front turns several failed gate runs into one.

**Order matters, because later links depend on earlier ones.**

1. **Content edits first.** Finish every prose change before touching any contract.
2. **Re-anchor pinned regions by CONTENT.** **Three** files pin `line_range_sha256` regions, not two:
   `current_claim_census.jsonl`, `book_quantitative_claims.jsonl` **and `published_assertions.jsonl`** — the
   last one is easy to miss because it pins *assertions* rather than regions, and an insert into a book chapter
   stales it exactly the same way. A `CHANGES.md` prepend shifts every region by the entry's line count; a book
   edit shifts that file's. Find each recorded digest's new location and rewrite the line numbers — never
   recompute the digest from the old offset, which would silently re-point a claim at different text.
   Measured once (`CORPUS-CHAIN-CURRENCY.8`, a 26-line insert mid-chapter): 4 assertion regions, 2 quantitative
   regions and 2 census evidence regions moved, every one of them by the same +26, with content unchanged.
   **Since `CLAIM-VERIFICATION-ADOPTION.12` (`2026-09-15`) this is a tracked instrument, not hand work:**
   `python3 scripts/repin_claim_regions.py --check` then `--apply`. It resolves by content across all
   three registries and **refuses** when a digest matches more than one location — which is the whole
   point, because a blank-line region matches every blank line in the file (280 of them in
   `live-docs.md`) and a wrong landing is invisible. Do not re-introduce a first-match throwaway.
   The `evidence_id`/`claim_key` suffix is derived from the region's **content** digest, so a pure shift leaves
   every identifier valid and only the line numbers move.
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

## A contract's JSON encoder is a property of the FILE, not of the repository

Several links in that chain end in "rewrite one field of a JSON contract", and that is where a careless
edit does the most damage: re-encoding with the wrong options reformats every line, and the one real
change disappears into hundreds of cosmetic ones. `MEMORY.md` used to generalise this as "use Perl
`JSON::PP` pretty+canonical, not Python". **Measured `2026-09-16` (`LIVE-DOCUMENT-PRESSURE-HEADROOM.23`),
that generalisation does not hold, and the language was never the axis.**

A round-trip census over the **27** parseable JSON contracts under `doctrine/` finds three classes:

- **20 are `json.dumps` output** with `separators=(',', ': ')` and a trailing newline — 15 at `indent=2`,
  insertion-ordered and ASCII-escaped (the default shape), plus four variants that sort keys, drop the
  ASCII escaping, or use `indent=1`.
- **3 were written by Perl `JSON::PP` `canonical`+`pretty`** — the task-evidence contracts. `pretty` sets
  `space_before` as well as `indent_length(3)`, so these alone carry `"key" : value`; that is the only
  difference, and `json.dumps(..., indent=3, sort_keys=True, separators=(',', ' : '))` reproduces them.
- **4 are hand-authored** and reproduce under no encoder at all: `fsmgen_feedback.json`,
  `roadmap_projection.json`, `spec_to_intent_vertical_eval_schema.json` and
  `trajectory_controller_input_schema.json` keep small objects inline on one line. Re-encoding one field
  expands them by **9-27%** and destroys that layout, so edit them as text.

`fact_card_catalog.json` — the one this chain sends you to after adding a card — is in the first class at
`indent=1, sort_keys=True`, so `JSON::PP` `pretty` is exactly wrong for it. **Derive, do not assume:**

```bash
python3 - "$FILE" <<'EOF'
import itertools, json, sys
raw = open(sys.argv[1], 'rb').read(); data = json.loads(raw)
for indent, sort, ascii_, sep in itertools.product(
        (None, 1, 2, 3, 4), (True, False), (True, False), ((',', ': '), (',', ' : '))):
    for nl in (b'\n', b''):
        if raw == json.dumps(data, indent=indent, sort_keys=sort, ensure_ascii=ascii_,
                             separators=sep).encode() + nl:
            print(f'indent={indent} sort_keys={sort} ensure_ascii={ascii_} '
                  f'separators={sep} newline={bool(nl)}'); sys.exit()
print('no encoder reproduces this file — it is hand-authored; edit it as text')
EOF
```

It reproduces **23 of the 27** exactly, including the three `JSON::PP` ones, and prints the hand-authored
message for exactly the four above. That is the proof that language is the wrong axis: the `' : '`
separator alone is what `pretty` adds.

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
