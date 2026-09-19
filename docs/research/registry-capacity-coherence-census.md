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
python3 scripts/measure_registry_capacity_coherence.py --self-test   # 9/9 RED cases after `.36b`
```

---

# `.36b` — what the remedy cost, and the two things this census got wrong

Everything above is `.36a`'s measurement at `ade27bc2` and stands as that dated observation. `.36b`
re-derived it before applying it, as this programme requires, and two of its forward-looking numbers
did not survive.

## The sizing was stale within the same day, and applying it would have been a stop

`.36a` published *"the largest record today is 7,827 bytes, so a per-record ceiling of 8,192 covers
the real shape with headroom."* Per-revision re-derivation, read-only:

```bash
for rev in ade27bc2 95c81cd1 HEAD; do
  git show $rev:doctrine/claim_verification/claims.jsonl \
    | awk '{ if (length($0)+1 > m) m = length($0)+1 } END { print rev, m }' rev=$rev
done
# ade27bc2 7827      <- what .36a measured
# 95c81cd1 10223     <- two commits later, same day
```

`BOUNDED-DECISION-PROVIDER.1a.1` registered a **10,223-byte** record at `95c81cd1`. An 8,192 ceiling
would have refused an already-committed `verified` record: the handed-down derivation would have
landed as a refusal, not a repair. **A sizing passed to a later leaf is an input to re-derive, not a
number to apply** — which is the same discipline this census applied to `.36`'s framing.

That record's size is itself evidence the stop had begun shaping the evidence. Its own `refresh_rule`
says so: *"One record carries both arms deliberately — the claim registry is at its byte ceiling with
no archive path (LIVE-DOCUMENT-PRESSURE-HEADROOM.36), and a second full record would not fit."*

## "A coherence rule, not a raise" was not available: the old envelope admitted no coherent triple

The census concluded that deriving the pair needed no raise. With the real record shape it needs one,
and the arithmetic is decisive. Admitting a 10,223-byte record needs `max_record_bytes >= 10,240`.
`scripts/check_claim_verification.pl` compiled a portable `max_bytes` cap of **131,072**. So

```text
13 x 10,240 = 133,120  >  131,072
```

and the largest coherent `max_records` inside the old envelope was **12** — exactly the population the
file already held. Zero headroom is a stop with no remedy, which `.2c` refuses. Coherence and the
portable cap could not both stand.

## The root cause is one level below this census's finding

The four claim-verification registries compile their portable envelopes independently, and every one
was written at roughly **512-1,024 bytes per permitted record**:

| checker | portable `max_records` | portable `max_bytes` | bytes per record | real mean record |
| --- | ---: | ---: | ---: | ---: |
| `check_claim_verification.pl` | 128 | 131,072 | 1,024 | **5,171** |
| `check_current_claim_census.pl` | 256 | 262,144 | 1,024 | 448 |
| `check_published_assertions.pl` | 512 | 262,144 | 512 | 393 |
| `check_book_quantitative_claims.pl` | 1,024 | 524,288 | 512 | 370 |

That ratio fits three of them. It does not fit the registry whose record shape
`CLAIM_VERIFICATION.md` §4 mandates — three legs, a stale gate, a control with a pinned RED region,
a refresh rule — measuring a 5,171-byte mean and a 10,222-byte maximum. **The declared bounds
inherited an assumption about record size that the standard above them forbids.** The portable triples
carry the same defect the declared ones do: `128 x 32,768` is 4 MiB against a compiled 131,072, and
three of the four multiply out to exactly 4 MiB.

## What `.36b` shipped

| bound | before | after | note |
| --- | ---: | ---: | --- |
| `max_record_bytes` | 32,768 | **12,288** | covers the 10,222-byte largest real record, 2,066 to spare |
| `max_records` | 64 | **21** | `floor(262,144 / 12,288)` |
| `max_bytes` | 65,536 | **258,048** | `21 x 12,288`, exactly |
| compiled portable `max_bytes` | 131,072 | **262,144** | the value two sibling claim registries already compile |

Both record bounds come **down**, as this census asked. Measured after, by the table above:
bytes **94.7% -> 24.0%**, records **18.8% -> 57.1%**, `funds@real` **12 -> 49** against a declared
**64 -> 21**, `coherent` **NO -> yes**, and *at or above a 90% rollover milestone* goes from
`claims.jsonl` to **(none)**. The record bound binds first at the real record size, which is the
protection this census describes.

## The class statement: a gate for the biting case only

`registry_capacity_coherence` in `scripts/check_live_document_size.pl` now reports, on every commit,
any registry whose byte bound funds fewer records at the size it really writes than `max_records`
declares — computed once, over the registries the checker **discovers**, exactly where `.22b` already
computes the class band. Eight registries remain incoherent without harm and are not failed: a gate
that failed nine registries on the day it landed is a policy defect, not an author problem (`.2c`).
The gate applies the exact capacity question — data-record mean, the header's bytes taken off
`max_bytes` — rather than this census's coarser `max_bytes / mean-over-the-whole-file`; on the
population above the two select the same registries, and the difference is visible only where a
registry holds very few records.

It was observed RED by revert-and-re-apply on the real registry, not on a fixture:

```text
live-document-size: warning: registry 'doctrine/claim_verification/claims.jsonl' byte bound funds 12
  records at its real mean of 5149 bytes, below the 64 its max_records declares
```

and silent after re-applying the repaired files byte-identically. It reproduces this census's
12-of-64 from a different language and a different code path.

**One true positive on the day it landed, named rather than repaired.**
`ceiling_increase_authorities.jsonl` funds 20 records of a declared 32 at its real 776-byte record.
That registry is normally empty, so the condition is visible only while a single-use authority is
banked — including `.36b`'s own, which `.36c` retires.

## What the remedy does not buy

Nine slots, of which this leaf's own claim record took one, leaving **eight**.

**`.36b.1` corrected this paragraph.** It first read *"and then the class portable envelope is
spent (258,048 of 262,144)"*, and an audit refused that by arithmetic: the per-record ceiling only has
to **admit** the largest record that exists, so trading ceiling headroom for slots reaches strictly
more records than the 21 shipped. The envelope is *nearly* spent, not spent, and those extra slots
cost the headroom the ceiling exists to provide.

**The exact residual is deliberately not written here, and the reason is itself the finding.** It is a
function of the largest record in the file, so it **shrinks every time a claim record is written, and
faster when the record is large**. Writing `.36b.1`'s own correction into
`claim-registry-capacity-is-coherent` grew that record from 10,944 to 11,544 bytes and took the
residual from **2 slots to 1** — within one slice, by the very edit that was correcting the claim. A
number with that behaviour cannot live in prose. It is **derived on every run** by
`reachable_records`, printed by the census, and gated by three RED cases; the portable cap it uses is
read out of `check_claim_verification.pl`'s own `%hard` block rather than from a description of it.
Read it with:

```bash
python3 scripts/measure_registry_capacity_coherence.py | tail -5
```

The rest stands. The registry has **no lifecycle**: §4 keeps a `superseded` record in place on purpose
(*"the old record remains historical evidence"*), and `rollover`, `segment` and `archive` appear
**zero** times in `check_claim_verification.pl`, so nothing returns a record slot. The trajectory
cannot size the successor either — re-derived per revision, the registry reached 6 records on
`2026-08-15`, added **nothing for 35 days**, then took **7 in a single day** on `2026-09-19` — which
is why `.36d` is triggered by the 80% record warning band at **17 of 21** (16 is silent at 76.2%)
rather than by a rate.
