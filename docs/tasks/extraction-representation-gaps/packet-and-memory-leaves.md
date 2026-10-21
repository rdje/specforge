# EXTRACTION-REPRESENTATION-GAPS — packet and memory leaves

- Part ID: `packet-and-memory-leaves`
- State: `active`

Open ERG leaves continue here, after `hint-and-notation-leaves` reached its rollover (`2026-10-21`): the reserved space
of captured packet layouts, the corpus memory's refresh, and every later ERG leaf. `.54.a` and `.55.b.iv.a` moved here
whole.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.54.a`
  Status: `open` (`2026-10-21`; CODE; opened by `.54`)
  Goal: **the corpus memory's refresh runs as ADR 0036 declares it.** The store pinned by the behavioral
  qualification (`a416cc8b…`) was learned from 13 IntentIR inputs, 8 of which are now legacy and refused by
  `learn-priors`, so its declared refresh cannot run and it still recalls readings the current reader withdraws (AXI
  `AWTAGOP` ready-like). **Reproduce:** `specforge learn-priors` over the store's `source_artifacts` fails on the
  first legacy input. **Impact:** the store cannot follow any reader change, and a checkout without it reads two
  records differently. Acceptance: re-declare the learning inputs as the IntentIR that validates today (the 31
  proof-carrying chains, or a stated subset), run ADR 0036's convergence loop to a byte-stable store, replay every
  affected chain and the behavioral holdout evidence, re-pin the hash in `behavioral_qualification.json` and the
  holdout evidence with the measured drift, and decide what triggers the next refresh. **Cost, measured
  `2026-10-21`:** `behavioral_qualification.json`'s frozen census pins the store with the replay library and entrypoint
  hashes, which still match today's files, so a refresh re-runs `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f`'s 24-document
  behavioral replay under that contract before the hash moves.

- ID: `EXTRACTION-REPRESENTATION-GAPS.55.b.iv.a`
  Status: `open` (`2026-10-21`; CODE; split from `.55.b.iv`)
  Goal: **a layout's reserved space is typed and cites the document's *Reserved* definition.** USB 3.2's 47 and USB4
  inter-domain's 6 rows that captured layouts mark *Reserved* are padding, not records, so *A reserved bit, byte, word
  or field shall be set to zero by the sender and shall be ignored by the receiver* reaches none of them.
  **Reproduce:** USB 3.2 *Link Control Word Format* `3 | 3:19 | Reserved (R) .`. Acceptance: record each reserved row
  of every message-field family as a typed span of its container with the family's own position reading, cite the
  `keyword_definitions` record where the document defines the keyword (and on the named `bReserved`-style fields),
  keep every field record unchanged, and measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.56`
  Status: `open` (`2026-10-21`; DESIGN; found by `.55.b.iv`)
  Goal: **the ERG task-evidence collection stays under its total-bytes bound.** The contract
  (`doctrine/live_document_size/extraction_representation_gaps_task_evidence.json`, `limits.parts`) bounds every
  part's bytes together at a 786,432-byte health target with an 80% warning; the 17 parts hold about 656 KB
  (83.4%) on `2026-10-21`, 82.9% before this slice, and no leaf owns the warning. The ledger grows by one leaf
  record and one checklist per slice (about 3-5 KiB), so it reaches the 90% band (707,789) in roughly 13 slices.
  **Reproduce:** `perl scripts/check_active_task_evidence.pl --contract
  doctrine/live_document_size/extraction_representation_gaps_task_evidence.json --check` (*bytes_total is at or
  above warning*). Acceptance: decide from what the parts are for — the closed records and checklists of the
  legacy and rolled-over parts are history, so seal them into a dated archive capsule the way the roadmap seals its
  rolled-over direction, or derive the bound from the part quantum (ADR 0029) — leave measured headroom, and keep
  every leaf route and claim region resolving.
