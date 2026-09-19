---
id: registry-capacity-bounds-are-incoherent
title: A banded registry's three bounds must multiply out, and claims.jsonl was the one place the incoherence bit — repaired by derivation at 21 x 12,288 = 258,048, with eight registries still incoherent without harm
answers:
  - "why did claims.jsonl stop at 94% with 12 of 64 records"
  - "are the doctrine registry capacity bounds coherent"
  - "what is max_records times max_record_bytes supposed to equal"
  - "which banded registries are coherent"
  - "should the claim registry get an archive lifecycle"
  - "why was raising max_bytes alone not the remedy for the claim registry"
  - "how big is a verified claim record"
  - "what per-record ceiling does the claim registry actually need"
  - "which registries are at risk from incoherent capacity bounds"
  - "how do I re-derive the registry capacity census"
  - "what are the claim registry's current max_records max_bytes and max_record_bytes"
  - "how many more claim records can the registry hold"
  - "why is the portable max_bytes cap in check_claim_verification.pl 262144"
  - "what gate reports a registry whose byte bound funds fewer records than it declares"
  - "why did the ceiling_increase_authorities registry start warning about capacity"
date: 2026-09-19
status: current
tags: [live-document-size, doctrine, registry, capacity, claim-verification, adr-0029]
evidence: scripts/measure_registry_capacity_coherence.py; scripts/check_live_document_size.pl (registry_capacity_coherence); docs/research/registry-capacity-coherence-census.md; doctrine/claim_verification/claims.jsonl; docs/tasks/live-document-pressure-headroom/current-and-open-work.md (.36, .36a, .36b, .36c, .36d)
reverify: "python3 scripts/measure_registry_capacity_coherence.py --self-test — expect 9/9, pinning 10 registries scored, 8 incoherent, 2 coherent, claims.jsonl coherent and no longer a binding case, and its declared capacity exactly fundable."
---

A banded JSONL registry declares `max_records`, `max_record_bytes` and `max_bytes`. Those are **not
independent**: admitting `max_records` records of `max_record_bytes` each needs their product, so a
`max_bytes` below it means the declared record capacity **cannot be filled** and the file stops at a
count nothing in the contract names. ADR 0029 states that rule for this repository (*capacity is the
part quantum times the part count*) and `[[FACT-CARD-CAPACITY-HEADROOM]]` applied it; the JSONL
registries never did.

**Measured over all 11 banded registries — 10 declare all three bounds, and 2 are coherent**
(`canonical_catalogs.jsonl`, where `8 × 1024 == 8192`, and `claims.jsonl` since `.36b`).

## The incoherence is class-wide; the harm is not

It is harmless while a registry's real records sit far below its permitted maximum.
`book_quantitative_claims.jsonl` may write 16KB records, writes 370-byte ones, and its byte bound
funds 1,062 records against a declared 896 — the record bound binds first, as intended.

**`claims.jsonl` was the only registry where that protection failed.** It writes the largest records
in the class by a wide margin — mean **5,171 bytes** against the next largest at 1,791 — because a
`verified` claim record carries three legs, a stale gate, a control with a pinned RED region and a
refresh rule, exactly as `CLAIM_VERIFICATION.md` §4 requires. Its byte bound funded **12** records
while its contract declared **64**, and at 94.7% it reached the only bound that was real.

## The repair, and the two things the first sizing got wrong

`.36a` sized the remedy at a per-record ceiling of 8,192 and concluded no raise was needed. `.36b`
re-derived both before applying them, and **neither survived**:

- **The sizing was stale within the same day.** 7,827 bytes was the largest record at `ade27bc2`;
  `95c81cd1` registered a **10,223-byte** one. An 8,192 ceiling would have refused an already-committed
  `verified` record. *A sizing handed to a later leaf is an input to re-derive, not a number to apply.*
- **No coherent triple existed inside the old envelope.** A 10,223-byte record needs
  `max_record_bytes >= 10,240`; the portable cap compiled into `check_claim_verification.pl` held
  `max_bytes` at 131,072; and `13 × 10,240 = 133,120` already exceeds it. The largest coherent
  `max_records` was **12** — the population already held. Zero headroom is a stop with no remedy.

**The cause is one level lower.** All four claim registries compile portable envelopes of roughly
512-1,024 bytes per permitted record (`131_072/128` here, `262_144/512` for published assertions,
`524_288/1_024` for the book census). That ratio fits their real 370-448 byte records; it is five to
ten times under this registry's §4 record shape. The declared bounds inherited an assumption the
standard above them forbids.

**Shipped by `.36b`:** `max_record_bytes` **32,768 → 12,288**, `max_records` **64 → 21**, `max_bytes`
**65,536 → 258,048**, with `21 × 12,288 = 258,048` exactly, and the compiled portable cap moved
131,072 → **262,144**, the value `check_current_claim_census.pl` and `check_published_assertions.pl`
already use. Bytes **94.7% → 24.0%**, records **18.8% → 57.1%**, `funds@real` **12 → 49**,
`coherent` **NO → yes**, nothing at or above a rollover milestone.

## The class statement is a gate, for the biting case only

`registry_capacity_coherence` in `scripts/check_live_document_size.pl` reports, on every commit, any
registry whose byte bound funds fewer records **at the size it really writes** than `max_records`
declares — computed once, over the registries the checker discovers, where `.22b` already computes
the class band. Eight registries stay incoherent without harm and are not failed: a gate that failed
nine on the day it landed is a policy defect, not an author problem. It was observed RED by
revert-and-re-apply on the real registry (`funds 12 … below the 64 its max_records declares`) and
silent after. One live true positive: `ceiling_increase_authorities.jsonl` funds 20 of a declared 32,
visible only while a single-use authority is banked.

## What the repair does not buy

**Nine slots**, one of which `.36b`'s own claim record took, leaving eight.

**It does NOT buy "the envelope is spent" — `.36b` published that and `.36b.1` refused it.** The
per-record ceiling only has to ADMIT the largest record that exists, so trading ceiling headroom for
slots reaches strictly more records than the 21 shipped. Nearly spent, not spent — and those extra
slots cost the headroom the ceiling exists for.

**Do not carry the residual as a number.** It is a function of the largest record, so it shrinks every
time a claim record is written and faster when the record is large: writing `.36b.1`'s own correction
into `claim-registry-capacity-is-coherent` grew that record by 600 bytes and took the residual from 2
slots to 1, inside the slice that was correcting the claim. It is **derived on every run** by
`reachable_records` and gated by three RED cases, with the portable cap read out of
`check_claim_verification.pl`'s own `%hard` block rather than from a description of it. Read it with
`python3 scripts/measure_registry_capacity_coherence.py | tail -5`.

What does hold: the registry has no lifecycle. §4 keeps a `superseded` record in place on purpose —
*"the old record remains historical evidence"* — and `rollover`, `segment` and `archive` appear
**zero** times in `check_claim_verification.pl`, so nothing returns a record slot. Retiring the two
`superseded` records would buy 1,656 bytes, a quarter of one verified record. The trajectory cannot
size the successor either: re-derived per revision, **6 records reached on `2026-08-15`, nothing added
for 35 days, then 7 in a single day**. `.36d` owns the lifecycle and triggers at the 80% record
warning band, **17 of 21** (16 is silent at 76.2%), not on a rate.

Links: [[live-surface-edit-bookkeeping-chain]], [[bounded-decision-frozen-baseline]].
