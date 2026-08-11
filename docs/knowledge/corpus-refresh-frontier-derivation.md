---
id: corpus-refresh-frontier-derivation
title: The CORPUS-COVERAGE refresh frontier is DERIVED from persisted evidence — cohort = documents whose SourceIR source is not the in-repo `corpus/` tree (57); refreshed = retained normalized bundle OR a `requested_path` off the retired `/Users/…` boot volume (51); remaining = neither (6). Carrying the count instead of deriving it lost `nvme_base_specification_2_0a_2021_07_26` for 22 consecutive refreshes (CORPUS-COVERAGE.4.0, 2026-08-11)
answers:
  - "how do I derive how many corpus refreshes remain (do NOT read a carried number: cohort = count of generated/source_ir/*/source_ir.json whose source.requested_path does NOT begin 'corpus/' = 57; refreshed = those with a retained bundle in doctrine/chain_currency/retained_bundles.json OR a requested_path not beginning '/Users/' = 51; remaining = neither = 6. The identity cohort == refreshed + remaining must hold exactly)"
  - "how many corpus refreshes are done and how many remain (51 of 57 done, six remaining as of 2026-08-11 — NOT the 51 of 56 / five that every surface said before CORPUS-COVERAGE.4.0; the root docs/tasks/CORPUS-COVERAGE.md carries current truth, re-derive rather than trust a number older than the last refresh)"
  - "why was nvme_base_specification_2_0a_2021_07_26 missing from the corpus refresh frontier (it was never excluded by a rule — the remaining count was hand-decremented by one per refresh from .2.29 to .2.51 and never re-derived, so a one-document denominator adjustment made at .2.29 for the project's own `ingest README.md` artifact outlived that artifact and silently shortened the queue. NVMe is the largest of the six at 4,577 elements so it sat at the tail and no candidate re-measurement could surface it)"
  - "which documents remain unrefreshed by CORPUS-COVERAGE.2 (ranked smallest-retained-source: opencapi_25gbps_phy_mechanical_spec_v10 760 elems, opencapi_3_0_transaction_layer_28jan2020 774, opencapi_3_1_transaction_layer_28jan2020 870, lpc_memory_agent_reference_design_guide_17jul2020 891, den0034_a_2013_09_13_debug_and_trace_configuration_and_usage_models 936, nvme_base_specification_2_0a_2021_07_26 4,577)"
  - "why does the refreshed test need BOTH a retention leg and a path leg (the path leg alone accounts for 50 of 51 — 31 cohort members carry a repository-relative .cache/local-references/… path and 19 a same-SSD /Volumes/SSD/… path — but usb_3_2_revision_1_0_2017_09 was refreshed at .2.33 BEFORE the volume migration, so it still records a retired /Users/… path and is recovered only by its retained normalized bundle. Dropping either leg mis-counts)"
  - "why is the in-repo corpus/ tree outside the .2 refresh cohort (its 21 documents are the tracked gold/eval corpus copied into the repository for the reproducible WIRE-BASED-100 path; they were never on the host-local library reached through .cache/local-references/chipdoc, so they have no retired-volume provenance to refresh)"
  - "is re-measuring the candidate list enough to keep the corpus frontier honest (NO — .2.51 correctly re-measured all six of its listed candidates from their own SourceIR profiles and still could not see the omission, because re-deriving the ROWS cannot validate the CARDINALITY. A document already absent from the list is invisible to any check over that list; only the independent identity cohort == refreshed + remaining exposes it)"
  - "what does CORPUS-COVERAGE.4 own (frontier census integrity: .4.0 derived the census, corrected every live surface, and restored the lost document; .4.1 is the pending mechanical gate that makes the identity enforced rather than carried. .3 closed the sibling frontier-vs-lifecycle defect but its currentness check covers retention only, not the remaining-work count)"
  - "does the corpus census correction change which document refresh 52 selects (no — NVMe is the largest of the six at 4,577 elements, so the smallest-retained-source policy still selects opencapi_25gbps_phy_mechanical_spec_v10 at 760 elements. The correction changes the denominator and the length of the tail, not the next pick; NVMe lands last, at .2.57)"
  - "did the corpus census correction move any generated artifact (no — CORPUS-COVERAGE.4.0 is read-only over generated/; check_chain_currency.sh exits 0 at evidence 23/23, semantic 78/78, intent 78/78, isf-adapter 78/78 with retention exactly the 23 declared bundles, before and after)"
date: 2026-08-11
status: current
tags: [corpus-coverage, census, task-tree, continuity, chain-currency]
evidence: docs/tasks/corpus-coverage/frontier-census-integrity.md; docs/tasks/CORPUS-COVERAGE.md; doctrine/chain_currency/retained_bundles.json
reverify: "python3 -c \"import json,os; P=lambda k: json.load(open(f'generated/source_ir/{k}/source_ir.json'))['source']['requested_path']; R=set(json.load(open('doctrine/chain_currency/retained_bundles.json'))['retained']); K=[k for k in sorted(os.listdir('generated/source_ir')) if not P(k).startswith('corpus/')]; rem=[k for k in K if k not in R and P(k).startswith('/Users/')]; print('cohort',len(K),'refreshed',len(K)-len(rem),'remaining',len(rem))\""
---

The `.2` refresh program's progress is a **derived** quantity, not a running total. Every input is a persisted
artifact, so the census reproduces from a clean checkout of `generated/` plus the retention declaration:

| Quantity | Rule | Count (`2026-08-11`) |
| --- | --- | ---: |
| Persisted documents | one `source_ir.json` per key under `generated/source_ir/` | 78 |
| Outside the cohort | `source.requested_path` begins `corpus/` | 21 |
| `.2` cohort | persisted minus in-repo | 57 |
| Refreshed | retained normalized bundle **or** `requested_path` not under `/Users/…` | 51 |
| Remaining | neither witness | 6 |

`57 = 51 + 6` is the invariant. It is the only check that can catch an omission, because it is computed without
reference to any maintained list of remaining documents.

**The failure it catches.** From `.2.29` to `.2.51` the remaining count was decremented by exactly one per
refresh — 27, 26, 25, … 7, 6, 5 — and never re-derived, so an error at the base propagated through twenty-two
consecutive slices untouched. The base error was a denominator adjustment recorded at `.2.29`: the cohort then
included the project's own `ingest README.md` artifact, so "57 documents" was annotated down to "56 real
chip-spec documents". That was correct when written. The README artifact later left `generated/`, making all 57
present cohort members real chip specs, but because the number was only ever decremented the retired adjustment
was never retired with it. The resulting deficit of exactly one dropped `nvme_base_specification_2_0a_2021_07_26`
— the 454-page NVM Express Base Specification 2.0a, source present at 5,154,704 bytes under the authorized
`chipdoc` symlink — out of the queue with no exception recorded anywhere against it.

**Why candidate re-measurement is not a substitute.** `.2.51` deliberately re-measured all six documents it
listed from their own SourceIR `document_profile`s rather than inheriting the previous slice's table. That is
good practice and it verified every row it had — but it re-derived the rows, not the cardinality, and a document
already missing from a list cannot be recovered by checking that list. The lesson generalises past this program:
a self-referential audit validates contents, never completeness. See `[[corpus-refresh-completion-vs-normalized-retention]]`
for the sibling distinction between a completed refresh and a currently retained bundle, and
`[[corpus-task-evidence-containment-design]]` for the containment topology this census is recorded in.

Until `CORPUS-COVERAGE.4.1` lands, the corrected count is still **carried, not enforced** — that is the honest
residual of the correction. Re-derive with the `reverify` command rather than trusting any transcribed number.
