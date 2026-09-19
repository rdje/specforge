---
id: wire-golds-held-out-not-lost
title: The three wire-based gold bundles were never lost — they are held out under generated/preserved/, and all three replay CONTENT SAME
answers:
  - "why does specforge evidence fail with path does not exist normalized md for APB AHB or AXI"
  - "do AXI APB and AHB need a re-ingest from PDF"
  - "can the three wire-based golds be rebuilt through the evidence stage"
  - "where are the AHB and AXI normalized bundles"
  - "where is the AXI normalized bundle"
  - "where is the AHB normalized bundle"
  - "is a missing normalized bundle the same as a lost normalized bundle"
  - "how long does it take to replay APB AHB and AXI evidence"
  - "is it safe to install a held-out normalized bundle"
  - "why was the CORPUS-CHAIN-CURRENCY.10 re-ingest refused"
  - "which documents are undeclared rather than unrebuildable"
date: 2026-09-19
status: current
tags: [corpus, currency, retention, wire-based-100, gold, adr-0025, measurement-integrity]
evidence: docs/tasks/CORPUS-CHAIN-CURRENCY.md; docs/tasks/RETAINED-BUNDLE-POPULATION-FROZEN.md; generated/preserved/WIRE-BASED-100.10/; scripts/lib/stage_artifact_identity.sh; scripts/check_chain_currency.sh
reverify: "ls -d generated/preserved/WIRE-BASED-100.10/*-normalized-bundle-held-out"
---

`specforge evidence generated/source_ir/ihi0024_e_…/source_ir.json --dry-run` fails with *"path does not
exist: …/normalized/ihi0024_e_….md"*, and the same is true for `ihi0033_c` (AHB) and `ihi0022_l` (AXI).
That observation is correct and the obvious inference from it is **wrong**. The bundle is not gone. It is
**held out**, deliberately, by a named leaf.

## Where they actually are

| document | held-out bundle |
| --- | --- |
| `ihi0024_e` (APB) | `generated/preserved/WIRE-BASED-100.10/apb-normalized-bundle-held-out/`, and `…/WIRE-BASED-100.9b/…` |
| `ihi0033_c` (AHB) | `generated/preserved/WIRE-BASED-100.10/ahb-normalized-bundle-held-out/`, and `…/WIRE-BASED-100.9c/…` |
| `ihi0022_l` (AXI) | `generated/preserved/WIRE-BASED-100.10/axi-normalized-bundle-held-out/`, and `…/WIRE-BASED-100.9d/…` |

`WIRE-BASED-100.9b`/`.9c`/`.9d` re-ingested the three golds on `2026-09-10`; `WIRE-BASED-100.10` re-ingested
them again on `2026-09-11`, which is the run that wrote the SourceIR they carry today. Every run parked the
bundle outside the normalized root instead of declaring it, because declaring it reddens two gate-tier
doctrines — that freeze is [[retained-bundle-population-is-frozen]], and it is the cause of this whole
situation.

## They replay, and they replay to the same content

Measured `2026-09-19` by `CORPUS-CHAIN-CURRENCY.10a`. Per document: copy the held-out bundle to the
normalized root the document's own SourceIR declares, run `evidence --dry-run` on `target/release/specforge`,
strip the `*_json:` preamble the way `check_chain_currency.sh` does, and compare against the persisted
EvidenceIR with `compare_stage_artifact` — the gate's own comparator from
`scripts/lib/stage_artifact_identity.sh`, not a second one.

| document | replay | elapsed | vs persisted EvidenceIR |
| --- | --- | ---: | --- |
| APB | exit 0 | 0.30 s | **CONTENT SAME** |
| AHB | exit 0 | 0.67 s | **CONTENT SAME** |
| AXI | exit 0 | 3.28 s | **CONTENT SAME** |

**4.25 s for all three.** Copy, never move, so the held-out originals are never exposed; remove the copy
afterwards, or `check_chain_currency.sh` fails closed on a bundle that no leaf declared.

## Why the distinction is worth a card

`CORPUS-CHAIN-CURRENCY.10` read "no bundle at the normalized root" as "cannot be re-derived at all", and
decided on that premise to **re-ingest all three golds from PDF** — accepting a regression risk on the
`WIRE-BASED-100` scores these documents hold at `1.000`, with no version-control rollback because
`generated/` is git-ignored. `.10a` refused the execution. The artifact was already on the volume, and
obtaining it again would have paid a real risk for nothing.

**CONTENT SAME is also the safety argument for the correct remedy.** Every downstream stage reads the
persisted EvidenceIR; an identical replay moves no downstream input; therefore installing these bundles
cannot move a downstream score. A re-ingest offers no such closure — it rewrites the SourceIR.

So the accurate statement about these three documents is that they are **undeclared, not unrebuildable**.
What is missing is permission to install the artifact where the producer reads it, not the artifact.
Installing and declaring all three is `RETAINED-BUNDLE-POPULATION-FROZEN.3`, blocked on that tree's `.1` and
`.2`, never on Docling.

Links: [[retained-bundle-population-is-frozen]], [[chain-currency-doctrine]],
[[corpus-canonical-currency-and-ownership]], [[source-ir-reingest-trades-captions-for-figure-text]].
