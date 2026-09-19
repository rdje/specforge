---
id: registry-capacity-bounds-are-incoherent
title: Nine of ten banded registries declare a record capacity their byte bound cannot fund, and claims.jsonl is the only one where that bites — it stops at 12 records while declaring 64
answers:
  - "why did claims.jsonl stop at 94% with 12 of 64 records"
  - "are the doctrine registry capacity bounds coherent"
  - "what is max_records times max_record_bytes supposed to equal"
  - "which banded registry is coherent"
  - "should the claim registry get an archive lifecycle"
  - "why is raising max_bytes not the remedy for the claim registry"
  - "how big is a verified claim record"
  - "what per-record ceiling does the claim registry actually need"
  - "which registries are at risk from incoherent capacity bounds"
  - "how do I re-derive the registry capacity census"
date: 2026-09-19
status: current
tags: [live-document-size, doctrine, registry, capacity, claim-verification, adr-0029]
evidence: scripts/measure_registry_capacity_coherence.py; docs/research/registry-capacity-coherence-census.md; doctrine/claim_verification/claims.jsonl; doctrine/live_document_size/surfaces.jsonl; docs/tasks/LIVE-DOCUMENT-PRESSURE-HEADROOM.md (.36, .36a, .36b)
reverify: "python3 scripts/measure_registry_capacity_coherence.py --self-test — expect 7/7, pinning 10 registries scored, 9 incoherent, 1 coherent, and claims.jsonl as the only registry whose byte bound funds fewer records than max_records declares."
---

A banded JSONL registry declares `max_records`, `max_record_bytes` and `max_bytes`. Those are **not
independent**: admitting `max_records` records of `max_record_bytes` each needs their product, so a
`max_bytes` below it means the declared record capacity **cannot be filled** and the file stops at a
count nothing in the contract names. ADR 0029 states that rule for this repository (*capacity is the
part quantum times the part count*) and `[[FACT-CARD-CAPACITY-HEADROOM]]` applied it; the JSONL
registries never did.

**Measured over all 11 banded registries — 10 declare all three bounds, and exactly 1 is coherent**
(`canonical_catalogs.jsonl`, where `8 × 1024 == 8192`).

## The incoherence is class-wide; the harm is not

It is harmless while a registry's real records sit far below its permitted maximum.
`book_quantitative_claims.jsonl` may write 16KB records, writes 370-byte ones, and its byte bound
funds 1,062 records against a declared 896 — the record bound binds first, as intended.

**`claims.jsonl` is the only registry where that protection fails.** It writes the largest records in
the class by a wide margin — mean **5,171 bytes** against the next largest at 1,791 — because a
`verified` claim record carries three legs, a stale gate, a control with a pinned RED region and a
refresh rule, exactly as `CLAIM_VERIFICATION.md` §4 requires. Its byte bound therefore funds **12**
records while its contract declares **64**, and at 94.7% it reached the only bound that was real.

## What follows for the remedy

- **A coherence rule, not a raise.** Derive the pair — choose a realistic `max_record_bytes`, then
  `max_bytes = max_records × max_record_bytes` — and `[[LIVE-DOCUMENT-PRESSURE-HEADROOM]].2a`'s
  relocation test is satisfied by construction, because both bounds then stop at the same place.
- **Sized:** the largest record today is **7,827** bytes, so a per-record ceiling of **8,192** covers
  the real shape. The record count is then a policy choice with an arithmetic price — 16 records cost
  131,072 bytes, 24 cost 196,608 — and the current `max_record_bytes` of 32,768, half the whole file,
  should come down in the same change. Any raise is subject to `.22`'s single-use ceiling-increase
  authority.
- **An archive lifecycle is refused by measurement.** The two `superseded` records total 1,656 bytes,
  a quarter of one verified record, and §4 keeps a superseded record in place on purpose —
  *"the old record remains historical evidence"*.

No bound has moved; this is the measurement that had to precede the proposal.
