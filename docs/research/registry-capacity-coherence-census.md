# Registry capacity coherence — the class census behind the claim registry's stop

Owning leaf: `LIVE-DOCUMENT-PRESSURE-HEADROOM.36a` (MEASURE the class before proposing anything).
Producer: `python3 scripts/measure_registry_capacity_coherence.py`.

## What `.36` asked, and why the answer is not "more room"

`.36` opened because `doctrine/claim_verification/claims.jsonl` reached **94.7% of its byte ceiling
while using 12 of its 64 records**, with no archive path. Its acceptance forbids proposing a remedy
before measuring the class, on this programme's own relocation rule (`.2a`): `max_records` and
`max_bytes` measure the same resource, so moving one number hands the stop to the other.

The class census says something better than "it needs more room". **The stop is not a fill problem.
It is an incoherence between three numbers that were written as if they were independent.**

## The three bounds are not independent quantities

Every banded registry declares:

| bound | meaning |
| --- | --- |
| `max_records` | how many records it may hold |
| `max_record_bytes` | how large one record may be |
| `max_bytes` | how large the whole file may be |

A registry admitting `max_records` records of `max_record_bytes` each needs their **product**. A
`max_bytes` below that product means the declared record capacity **cannot be filled** — the file
stops first, at a record count nothing in the contract names. ADR 0029 already states this rule for
this repository (*capacity is the part quantum times the part count*), and
`FACT-CARD-CAPACITY-HEADROOM` applied it (`max_cards = cards_per_part × max_parts`). It was never
applied to the JSONL registries.

## The census

```bash
python3 scripts/measure_registry_capacity_coherence.py
```

**11 banded registries, 10 declaring all three bounds, and exactly 1 is coherent.**

| registry | bytes% | recs% | real/rec | permitted/rec | byte bound funds | declared | coherent |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | :---: |
| `claims.jsonl` | **94.7** | 18.8 | **5,171** | 32,768 | **12** | **64** | NO |
| `surfaces.jsonl` | 59.6 | 75.0 | 813 | 4,096 | 120 | 96 | NO |
| `book_quantitative_claims.jsonl` | 51.1 | 60.6 | 370 | 16,384 | 1,062 | 896 | NO |
| `current_claim_census.jsonl` | 47.3 | 61.6 | 448 | 2,048 | 292 | 224 | NO |
| `derived_state_contracts.jsonl` | 42.2 | 58.3 | 494 | 2,048 | 33 | 24 | NO |
| `published_assertions.jsonl` | 24.0 | 31.2 | 393 | 4,096 | 166 | 128 | NO |
| `rolling_ledgers.jsonl` | 21.9 | 50.0 | 1,791 | 6,144 | 18 | 8 | NO |
| `citations.jsonl` | 16.3 | 45.3 | 184 | 2,048 | 178 | 64 | NO |
| `canonical_catalogs.jsonl` | 9.4 | 62.5 | 153 | 1,024 | 53 | 8 | **yes** |
| `ceiling_increase_authorities.jsonl` | 1.2 | 0.0 | — | 4,096 | — | 32 | NO |

`canonical_catalogs.jsonl` is coherent because `8 × 1024 == 8192` exactly — the only registry whose
author did the multiplication.

## The finding: class-wide in the declaration, binding in exactly one place

Incoherence alone is harmless while a registry's **real** records are far below its permitted
maximum. `book_quantitative_claims.jsonl` may write 16KB records and actually writes 370-byte ones,
so its byte bound funds **1,062** records against a declared 896 — the record bound binds first, as
intended, and nobody notices the product was never checked.

**`claims.jsonl` is the only registry where that protection fails.** It writes the largest records in
the class by a wide margin — a mean of **5,171 bytes** against the next largest at 1,791 — because a
`verified` claim record carries three legs, a stale gate, a control with a pinned RED region and a
refresh rule, exactly as `CLAIM_VERIFICATION.md` §4 requires. At that size its byte bound funds
**12** records while its contract declares **64**. It stopped at a number that appears nowhere.

That is why `.36`'s framing needed correcting before a remedy: the registry is not full of waste, and
it does not need an archive to claw back a quarter of one record. **Its capacity was never declared
coherently, and the file simply reached the only bound that was real.**

## What this means for the remedy, which is `.36b`'s

Two things follow, and both narrow the design rather than widen it:

1. **The remedy is a coherence rule, not a raise.** Deriving the pair — pick a realistic
   `max_record_bytes`, then `max_bytes = max_records × max_record_bytes` — makes the declared
   capacity the capacity that exists. `.2a`'s relocation test is satisfied by construction, because
   after the derivation the two bounds stop first at the same place.
2. **Sized, so `.36b` does not have to rediscover it.** The largest record in `claims.jsonl` today is
   **7,827** bytes, so a per-record ceiling of **8,192** covers the real shape with headroom. The
   record count is then a policy choice with an arithmetic price: 16 records costs 131,072 bytes,
   24 costs 196,608. Any raise that does land is subject to `.22`'s single-use ceiling-increase
   authority protocol, and the current `max_record_bytes` of 32,768 — half the whole file — should
   come **down** in the same change, since nothing has ever needed it.

**An archive lifecycle is not the answer here, and the measurement says so.** The two `superseded`
records total 1,656 bytes, a quarter of one verified record. Retiring them buys nothing, and
`CLAIM_VERIFICATION.md` §4 keeps a superseded record in place on purpose — *"the old record remains
historical evidence"*.

## What this census does not establish

- **No bound has moved.** This is the measurement `.36`'s acceptance demanded before a proposal;
  `.36b` owns the change and the authority protocol that governs it.
- **The other nine incoherent registries are not repaired**, and eight of them are not in danger:
  their real records are small enough that the record bound still binds first. They are recorded so
  the next one to grow large records is found by arithmetic rather than by hitting a wall.
- **It is a snapshot of fill.** The self-test pins the *shape* of the finding — 10 scored, 9
  incoherent, 1 coherent, and `claims.jsonl` the only binding case — so a commit that repairs a
  registry or pushes a new one past rollover fails loudly instead of drifting.

## Re-derivation

```bash
python3 scripts/measure_registry_capacity_coherence.py
python3 scripts/measure_registry_capacity_coherence.py --self-test   # 7/7 RED cases
```
