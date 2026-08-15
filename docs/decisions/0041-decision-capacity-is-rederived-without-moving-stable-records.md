---
id: decision-capacity-is-rederived-without-moving-stable-records
title: Decision capacity is re-derived without moving stable records
date: 2026-08-15
status: accepted
scope: documentation, decisions, knowledge-map, capacity, continuity, containment
evidence: docs/tasks/DECISION-RECORD-CAPACITY-HEADROOM.md; git decision-record creation history; doctrine/live_document_size/surfaces.jsonl; doctrine/knowledge_map/shard_contract.json; scripts/check_fact_card_catalog.pl
reverify: "find docs/decisions -maxdepth 1 -type f -name '*.md' | wc -l; git log --diff-filter=A --date=short --format='DATE %ad' --name-status -- 'docs/decisions/*.md'; perl scripts/check_fact_card_catalog.pl --check; bash knowledge-map/scripts/check_knowledge_map.sh"
answers:
  - "what is ADR 0041"
  - "why does decision-record capacity become 58"
  - "how is the decision-record file bound derived"
  - "why are existing decision records not moved into partitions"
  - "how do decision slots change Knowledge Map capacity"
  - "what should happen when a decision record approaches its per-file bound"
  - "why is ADR 0038 not split or rewritten"
---

# ADR 0041: Decision capacity is re-derived without moving stable records

## Context

ADR 0029 sized `decision_records.files` from the then-current 30-file population and the measured nine-file
peak active day: `(30 + 9) / 0.90`, rounded to 44. That was a measured buffer, not a permanent architectural
constant. The collection now has 42 files before this decision and therefore 43 including it. The old 44-file
profile is at 97.7% once this decision exists and leaves one slot for all future architecture work.

The original growth history remains the relevant bounded observation. Decision records were created on eleven
active dates; the largest day added nine files, followed by eight and six. Since the earlier profile, four
records arrived on one day and then one per active day. No newer burst exceeds nine, so the peak remains nine.

The collection has two independent pressures:

- count: 43 files against 44 after this decision;
- member shape: accepted ADR 0038 is uniquely largest at 474/512 lines and 31,833/32,768 bytes.

They do not share a remedy. Count capacity can be re-derived. ADR 0038 is accepted historical rationale and is
not appendable; rewriting, splitting, or moving it would destroy a stable path and violate the decision-record
append/supersede doctrine merely to silence a percentage. Future large decisions can keep their actual Context,
Decision, and Consequences bounded and route detailed measurements to their owning task or research evidence.

Readers and writers confirm that no topology migration is warranted. Bootstrap and memory routes address
`docs/decisions/`; the index links every member directly; the Knowledge Map scans `docs/decisions/*.md`; the
fact-card checker derives question capacity from the decision surface; and Markdown links throughout the repo
name stable individual paths. The flat index is only 58 lines before this row and remains far below its own
512-line bound at the selected capacity.

## Decision

### 1. Re-run the existing milestone formula including this decision

Use the same rule as ADR 0029: the current population must be below the 80% warning, and one measured peak active
day must remain below the 90% rollover signal.

The smallest integer satisfying both inequalities for 43 current files and a nine-file peak is 58:

- `43 / 58 = 74.1%`;
- `(43 + 9) / 58 = 89.7%`;
- 57 is insufficient because `52 / 57 = 91.2%`.

This is a re-derivation from changed inputs, not permission to inflate future capacity. When 58 approaches its
warning, the population and creation trajectory must be measured again.

### 2. Move the coupled fact profile in one transaction

ADR 0029 makes fact capacity a structural join. With 336 card slots and 58 decision files, of which the index is
not a fact, `max_facts` becomes `336 + (58 - 1) = 393`. At eight question keys per fact, 393 facts require 3,144
keys; rounded to the Knowledge Map registry's 512-key step, `max_question_keys` becomes 3,584.

The existing 32-shard projection can already hold that key bound and the portable hard caps remain 512 facts /
4,096 keys / 64 shards. No canonical glob, shard count, landing bound, path, or projection algorithm changes.

### 3. Keep the flat stable-path topology

No existing record moves or changes. Adding subdirectories would require rewriting every consumer and durable
link while solving no measured browse or projection problem. Raising directly to the portable maximum would
bank unmeasured authority. Splitting ADR 0038 would mutate accepted history. The minimal coupled profile is the
only selected change.

### 4. Apply and retire exact authority transactionally

`DECISION-RECORD-CAPACITY-HEADROOM.2` must change the decision surface from 44 to 58 files, re-derive both
aggregate bands as `files × per-file`, update the fact/question limits, and carry one exact ceiling-increase
authority. It must prove full-profile feasibility and a fail-closed over-capacity case. If any join fails, all
profile fields roll back together. `.2a` then removes the consumed authority; it cannot remain banked.

## Consequences

- The claim-verification adoption can add its own architecture decision after `.2/.2a` without consuming the last
  unremedied slot.
- Existing decision paths, contents, index semantics, and Knowledge Map inputs remain stable.
- The persistent ADR 0038 warning stays visible and honest. It is not a writable live document, so its remedy is
  future authoring discipline rather than historical mutation.
- The next capacity warning has a known procedure: re-measure population and peak, move every coupled fact-plane
  authority together, and retire the exact increase authority.

## Links

- Task tree: [`DECISION-RECORD-CAPACITY-HEADROOM`](../tasks/DECISION-RECORD-CAPACITY-HEADROOM.md)
- General derivation: [ADR 0029](0029-fact-plane-capacity-is-one-derived-profile-and-aggregates-must-have-a-remedy.md)
- Aggregate reachability: [ADR 0032](0032-no-collection-may-declare-an-aggregate-below-its-own-legal-maximum.md)
