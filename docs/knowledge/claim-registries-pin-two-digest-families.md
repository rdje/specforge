---
id: claim-registries-pin-two-digest-families
title: The claim registries pin TWO digest families — 731 line-range regions and 113 whole-file digests — and the whole-file ones go stale on any `MEMORY.md` edit, converging in exactly one pass
answers:
  - "why did the claim-verification gate fail after I edited MEMORY.md"
  - "what re-pins durability artifacts sha256"
  - "how do I fix a stale durability artifact digest"
  - "what is a record_type source pin"
  - "why is surfaces.jsonl pinned in three registries"
  - "does repin_claim_regions.py cover whole-file digests"
  - "how many claim pins are there"
  - "how many passes does a claim digest refresh need"
  - "do the claim registries depend on each other cyclically"
  - "why is file_digest uncached in repin_claim_regions"
  - "how is a whole-file claim pin detected"
  - "what goes stale when I register a task-evidence part"
date: 2026-09-22
status: current
tags: [claim-verification, doctrine, tooling, live-documents, digests]
evidence: scripts/repin_claim_regions.py (whole_file_pins, classify_whole_file, file_digest, process); doctrine/claim_verification/*.jsonl; docs/toolbox/gates-build-and-host.md §7.7; docs/tasks/CLAIM-VERIFICATION-ADOPTION.md (.12, .21)
reverify: "python3 scripts/repin_claim_regions.py --check on a clean tree — expect `unchanged 844` (731 line-range + 113 whole-file). python3 scripts/repin_claim_regions.py --self-test — expect 27/27. Append one byte to MEMORY.md and re-run --check: expect exactly 3 stale, owned by mdbook-quantitative-census-frozen, current-claim-census-frozen and claim-provenance-gate-active."
---

`repin_claim_regions.py` was built by `CLAIM-VERIFICATION-ADOPTION.12` for `line_range_sha256`
regions. The registries pin **two more classes that carry no line range**, and until `.21` neither
had any instrument:

| family | count | paths | shape |
| --- | ---: | ---: | --- |
| `line_range_sha256` regions | 731 | 63 | three shapes, incl. RED cases inside checker scripts |
| `durability.artifacts[].sha256` (`claims.jsonl`) | 107 | 69 | `{path, role, sha256}` |
| `record_type: "source"` (other three registries) | 6 | 3 | `{path, record_type, schema_version, sha256, source_id}` |

## The whole-file family is the one ordinary work hits

**`MEMORY.md` carries three whole-file pins**, and `COMMIT.md` step 4 asks for it to be overwritten
whenever the resume state changes — so a slice that edits nothing else still leaves three stale.
**`doctrine/live_document_size/surfaces.jsonl` carries four plus all three `source` records**, and
registering a task-evidence part requires editing it: that one edit invalidates 12 pins across all
four registries. `CHANGES.md`, `DEVELOPMENT_NOTES.md` and `TOOLBOX.md` are pinned the same way.

Detection is **structural, never a named shape**: a `path`, a `sha256`, and **no**
`start_line`/`end_line`. The two shapes share no other field, and a named-shape test is exactly what
left `.12`'s first version blind to two of its three line-range shapes. Measured over the live
registries, the structural rule matches 113 pins and nothing else.

## Convergence is ONE pass, and that is structural

`claims.jsonl` pins the three sibling registries; **nothing pins `claims.jsonl`**. The dependency
graph is therefore an acyclic depth-2 DAG. Processing the registries in declaration order refreshes
the `source` records before the artifacts that watch them, so one pass suffices — a verification pass
re-reads every whole-file pin afterwards and prints `CONVERGENCE FAILED` rather than looping, because
a second stale pass would mean the registries had grown a cycle.

`file_digest` is **deliberately uncached**, unlike `read_lines`: `claims.jsonl` pins registries the
same run rewrites, so a cached digest would be the pre-write one and the refresh would converge to
the wrong value while reporting success.

## The hazard is different from the line-range one

A line-range re-pin can land on the wrong line and be invisible, which is why `.12` refuses ambiguity.
A whole-file digest cannot be ambiguous — it either matches or it does not. Its hazard is **silent
blessing**: refreshing one asserts *"I have seen this change and the claim still holds"*. So each
refresh prints its owning claim and whether this working tree edited the path (`[edited here]` /
`[changed elsewhere]` / `[rewritten by this run]` for a registry the run itself moved).

Related: [[canonical-collection-catalogs]], [[doctrine-driver-runs-no-cargo-gate]].
