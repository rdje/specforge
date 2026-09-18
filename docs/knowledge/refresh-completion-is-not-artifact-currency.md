---
id: refresh-completion-is-not-artifact-currency
title: A completed corpus refresh is a provenance fact, not a currency one — 31 of the 52 refreshed documents no longer load, and 6 loadable ones were never in the cohort
answers:
  - "why do CORPUS-COVERAGE's 52 of 57 refreshes and ADR 0048's 27 of 78 loadable documents disagree (they count different things: a refresh makes a document's source repository-local and rebuilds it with the binary of the day, which is provenance; loadability is whether the persisted artifact is at the current schema with a verified proof ledger)"
  - "how many of the completed corpus refreshes are still canonically loadable (21 of 52 — the other 31 were made inspection-only when EVIDENCE_IR_SCHEMA_VERSION bumped to 3 on 2026-08-13, after their refreshes landed)"
  - "which loadable documents are outside the CORPUS-FRONTIER refresh cohort (6 — ihi0022_l_2025_08, ihi0024_e, ihi0033_c, ihi0074_a, um10204, um11732; the cohort rule selects on source.requested_path NOT beginning with corpus/, and these were ingested from corpus/ already, so they never needed a refresh)"
  - "is the corpus stratum boundary at SourceIR or at EvidenceIR (both, and they never disagree — across all 78 persisted documents SourceIR schema 3 with a proof ledger and EvidenceIR schema 3 coincide exactly, with no document in a mixed state, so a currency probe may use either stage)"
  - "what is the exact partition of the 78 persisted documents by schema and refresh cohort (78 = 21 refreshed-and-loadable + 6 outside-cohort-and-loadable + 31 refreshed-but-legacy + 15 outside-cohort-and-legacy + 5 remaining-and-legacy)"
  - "does CHAIN-CURRENCY reporting 78 chains current contradict the canonical loader refusing 51 (no — CHAIN-CURRENCY measures stage-local reproducibility, that each stage still replays from its persisted upstream; canonical loadability is a different property and ADR 0048 governs it)"
  - "how do I re-derive the refresh-versus-currency partition (read doctrine/corpus_frontier/census.json for the refreshed and remaining sets, then join it against each generated/source_ir/<doc>/source_ir.json proof ledger and each generated/evidence_ir/<doc>/evidence_ir.json schema_version)"
date: 2026-09-18
status: current
tags: [corpus, chain-currency, corpus-frontier, adr-0048, measurement, claim-hygiene, method]
evidence: docs/tasks/corpus-coverage/frontier-census-integrity.md (CORPUS-COVERAGE.5); doctrine/corpus_frontier/census.json; docs/decisions/0048-the-persisted-corpus-has-a-measured-and-a-historical-stratum.md; crates/specforge/src/ir/evidence.rs (EVIDENCE_IR_SCHEMA_VERSION, load_from_path); docs/tasks/CORPUS-COVERAGE.md; ROADMAP.md
reverify: "python3 -c \"import json,glob,os;c=json.load(open('doctrine/corpus_frontier/census.json'));r=set(c['refreshed']);m=set(c['remaining']);rows=[(k.split('/')[2], any('proof' in x.lower() or 'ledger' in x.lower() for x in json.load(open(k)))) for k in sorted(glob.glob('generated/source_ir/*/source_ir.json'))];import collections;print(collections.Counter(('loadable' if p else 'legacy', 'refreshed' if d in r else ('remaining' if d in m else 'outside')) for d,p in rows))\" — expect loadable/refreshed 21, loadable/outside 6, legacy/refreshed 31, legacy/outside 15, legacy/remaining 5"
---

Two figures in this repository are both correct and were one inference apart from being read as the same
thing. `CORPUS-COVERAGE` publishes *"52 of 57 real chip-spec refreshes are complete"*. `ROADMAP.md`
publishes *"27 chains are re-derivable and 51 stay legacy"*.

## They count different properties

A **refresh** re-ingests a document from a repository-local source with the binary of the day. It fixes
*provenance*: where the source lives and which producer built the chain. The `CORPUS-FRONTIER` cohort is
derived from `source.requested_path` **not** beginning with `corpus/` — a document already ingested from
`corpus/` is outside the cohort because it never needed one.

**Loadability** is whether the persisted artifact is at the current schema with a verified proof ledger, so
`EvidenceIr::load_from_path` accepts it. `ADR 0048` names the two resulting strata.

Nothing keeps the first property fresh. A refresh completed in July was current the day it ran and became
inspection-only when `EVIDENCE_IR_SCHEMA_VERSION` went to 3 on `2026-08-13`.

## The exact partition, measured 2026-09-18

| SourceIR | proof ledger | EvidenceIR | cohort | documents |
| --- | --- | --- | --- | ---: |
| schema 3 | yes | schema 3 | refreshed | 21 |
| schema 3 | yes | schema 3 | outside cohort | 6 |
| schema 1 | no | schema 2 | refreshed | 31 |
| schema 1 | no | schema 2 | outside cohort | 15 |
| schema 1 | no | schema 2 | remaining | 5 |

- **31 of 52 completed refreshes no longer load.** *52 complete* does not mean *52 usable*, and under
  `ADR 0048` §5 that is not a defect to repair.
- **6 loadable documents were never in the cohort**, and they are the ones current work needs:
  `ihi0022_l_2025_08`, `ihi0024_e`, `ihi0033_c`, `ihi0074_a`, `um10204`, `um11732`. Five of them are the
  entire promotable population in `[[measured-stratum-promotion-population]]`. A reader who takes 57 as
  "the corpus" misses all six.
- **The strata are one boundary.** SourceIR and EvidenceIR currency coincide exactly across all 78; no
  document is in a mixed state.

## A third sense of "current" sits beside these two

`CORPUS-COVERAGE` also reports *78 downstream chains, all measured current by the `CHAIN-CURRENCY` gate*.
That gate measures **stage-local reproducibility** — each stage still replays from its persisted upstream —
which is true of legacy artifacts too. It is not canonical loadability and does not contradict it.

This is the third time this tree has had to separate a fact about *where a source lives or when it was
built* from a fact about *what the artifact can be used for*: `SPEC-TO-INTENT-ALIGNMENT.4b` decoupled path
locality from refresh completion, `CORPUS-COVERAGE.4.1` made cohort membership derived rather than
hand-listed, and `CORPUS-COVERAGE.5` separates refresh completion from artifact currency.
