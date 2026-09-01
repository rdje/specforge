# Canonical currency of the persisted corpus — and who owns each shortfall

Owning leaf: `WIRE-BASED-100.9a` (measurement-first, read-only, docs-only — no code change).
Date: `2026-09-01`. Reproducer: `scripts/measure_corpus_canonical_currency.py`.
Sibling to `WIRE-BASED-100.8`, which restored the scoring oracle and then discovered that the wire
golds it exists to protect cannot be scored at all.

## Why this measurement exists

`WIRE-BASED-100.8` closed on `2026-09-01` with an honest statement of what it did not deliver: the
APB, AHB, and AXI golds "remain unmeasurable because their EvidenceIR is legacy", and it routed that
work away with one sentence — *"Re-ingesting the legacy stratum stays with the corpus refresh
frontier."* Its `Non-goal` line repeats the routing: *"re-ingesting the 54 legacy chains (owned by
the corpus refresh frontier)"*.

That routing is wrong, and it is wrong in a way no gate was watching for. This measurement derives
three things `.8` assumed:

1. how much of the persisted corpus the current binary can actually score;
2. which task-tree owns each document it cannot; and
3. what the refresh frontier's published `52 refreshed / 5 remaining` actually means.

Everything below is derived by `scripts/measure_corpus_canonical_currency.py`, which reads persisted
artifact headers, the four schema constants in the Rust sources that define canonical currency, the
frontier's own contract, and the tracked eval datasets. It runs no model, rebuilds no stage, and
writes nothing.

## The definition, taken from the code rather than invented

A document is **measurable** when its persisted EvidenceIR is at the current canonical schema. That is
not a convention chosen for this report — it is the exact predicate `eval-extraction` applies before it
will score anything (`unmeasurable_disposition`,
`crates/specforge/src/commands/eval_extraction.rs:180-191`). Everything else is **legacy**: refused by
design, inspection-only, with a documented re-ingest route the binary itself prints.

The reproducer reads each stage's canonical version out of the constant that defines it rather than
restating it, so a schema bump makes the census disagree loudly instead of agreeing with itself:

| stage | canonical schema | constant |
| --- | ---: | --- |
| SourceIR | 3 | `SOURCE_IR_SCHEMA_VERSION` (`crates/specforge/src/ir/source.rs`) |
| EvidenceIR | 3 | `EVIDENCE_IR_SCHEMA_VERSION` (`crates/specforge/src/ir/evidence.rs`) |
| SemanticIR | 2 | `SEMANTIC_IR_SCHEMA_VERSION` (`crates/specforge/src/ir/semantic.rs`) |
| IntentIR | 2 | `INTENT_IR_SCHEMA_VERSION` (`crates/specforge/src/ir/intent.rs`) |

## 1. Less than a third of the persisted corpus is canonically usable

| population | count |
| --- | ---: |
| persisted documents | 78 |
| **measurable** (EvidenceIR at canonical schema 3) | **24 (30.8%)** |
| legacy (refused, inspection-only) | 54 |

The stratification is total and clean at every stage — there is no partially-migrated document:

| stage | persisted schema versions |
| --- | --- |
| SourceIR | 54× schema 1, 24× schema 3 |
| EvidenceIR | 54× schema 2, 24× schema 3 |
| SemanticIR | 54× schema 1, 24× schema 2 |
| IntentIR | 54× schema 1, 24× schema 2 |

This is the same 54/24 split `WIRE-BASED-100.8b` censused, and it confirms that leaf's correction:
the legacy stratum cannot be named by one schema number, because the legacy version differs per
stage (1/2/1/1 against 3/3/2/2 current). The structural difference is visible in the artifact itself
— a schema-3 SourceIR carries `proof_context` and `proof_ledger` keys that a schema-1 SourceIR does
not have at all. Schema 3 landed in `bb5047c2` (`2026-08-12`).

## 2. Two published coverage numbers, and neither answers the question

Both existing gates are green, and a reader looking for corpus health finds one of them first:

- `scripts/check_chain_currency.sh` → **`24 replayed / 24 current / 0 stale`**. True, and a currency
  statement about the *rebuildable* stratum only. It explicitly declares the other 54 UNMEASURABLE
  and does not count them, so it reads as 100% while describing 31% of the corpus.
- `scripts/check_corpus_frontier.sh` → **`57 cohort = 52 refreshed + 5 remaining`**. True, and a
  completion statement about the *host-library re-ingest program* (`CORPUS-COVERAGE.2`). It reads as
  91% done.

Neither is wrong. Neither says that 54 of 78 persisted documents are refused by the scorer.

**The frontier's `refreshed` does not mean current.** Of its 52 declared-refreshed documents, **31 are
still legacy** and only 21 are canonically current. The census contract is honest about why — it
defines `refreshed` as "DECLARED completed document keys" for the host-library sweep. Read exactly, its
membership check constrains that declaration in one direction only: a *retained* normalization bundle
FORCES a key into `refreshed` (`retained cohort document '$key' is not declared refreshed`), and a
declared-*remaining* key must have no retained bundle. Nothing anywhere ties `refreshed` to a schema.
But the sentence a human or agent reads on the terminal is `52 refreshed + 5 remaining`, and that
sentence invites exactly the inference `WIRE-BASED-100.8` made. The sweep completed under a SourceIR
schema that no longer carries canonical authority.

**The frontier also excludes the wire golds by contract.** Its cohort rule is
`excluded_source_prefixes: ["corpus/"]`, documented in
`scripts/check_corpus_frontier_census.pl` as *"`corpus/` is the tracked in-repo gold/eval corpus —
copied into the repository, never part of the host-library refresh program."* Of the 78 persisted
documents, **21 are `corpus/`-sourced** and therefore outside the cohort entirely; **18 of those 21 are
legacy**.

## 3. Who owns each of the 54 legacy documents

| owner today | legacy documents |
| --- | ---: |
| refresh frontier: declared refreshed (but still legacy) | 31 |
| refresh frontier: declared remaining | 5 |
| **outside the refresh frontier (excluded `corpus/` prefix) — no OPEN owner** | **18** |

The 18 are precisely the in-repo gold/eval corpus. `WIRE-BASED-100.8` handed its remainder to an owner
whose own contract excludes it: the frontier could not have picked this work up, would never report it
as outstanding, and its `5 remaining` would still read `5` after every wire gold had rotted.

**A closed leaf did once own exactly this, and that is the actual cause.** `CORPUS-PATTERN-REUSE.3c`
(`done`, `2026-06-09`) re-ingested APB `ihi0024_e`, AHB `ihi0033_c`, AXI `ihi0022_l` and AXI-Stream
`ihi0051_b` with `DOCLING_DEVICE=cpu`, rebuilt their evidence, and re-verified the wire scores at
`1.000` on that fresh evidence. So these documents were not neglected — they were refreshed, and the
refresh was later invalidated from underneath. The persisted wire SourceIRs were last written
`2026-08-09`; canonical schema 3 landed `2026-08-12` (`bb5047c2`), three days later. The three
`corpus/`-sourced documents that are measurable today were written `2026-08-28`, after it.

That reframes the defect. It is not "somebody forgot to re-ingest": a completed refresh went legacy
because a schema bump moved the canonical bar, **and no gate reported it** — the frontier excludes
these documents by cohort rule, and chain-currency counts only the stratum that is already current. The
lesson for a future "nobody owns X" claim is that the absence of an *open* owner is not the absence of
an owner: the closed leaf is where the causal story lives, and it must be searched before the claim is
published.

## 4. The consequence: five of seven gold documents cannot be scored

The reproducer derives the scored set from the tracked eval datasets' own `doc_key` fields
(`crates/specforge/test_data/llm_eval/*.json`) rather than any name list.

| document | datasets | status | open owner |
| --- | --- | --- | --- |
| `ihi0074_a…arm_debug_interface_v6` | `seed_swd`, `seed_swd_derivation` | measurable | — |
| `um10204…i2c_bus_specification` | `seed_i2c_signals` | measurable | — |
| `ihi0024_e…amba_5_apb` | `seed_apb`, `seed_apb_temporal` | **UNMEASURABLE** | none |
| `ihi0033_c…amba_5_ahb` | `seed_ahb`, `seed_ahb_temporal` | **UNMEASURABLE** | none |
| `ihi0022_l…amba_axi` | `seed_axi`, `seed_axi_temporal` | **UNMEASURABLE** | none |
| `1_0_risc_v_debug_specification` | `seed_riscv_debug_registers` | **UNMEASURABLE** | none |
| `nvme_base_specification_2_0a…` | `seed_nvme_registers` | **UNMEASURABLE** | frontier: remaining |

**Two of seven.** SpecForge's entire re-derivable evaluation surface is the SWD/ADI and I2C documents.
Four of the five unmeasurable gold documents have no OPEN owning leaf (the APB/AHB/AXI three were last
owned by the closed `CORPUS-PATTERN-REUSE.3c`, §3), and the fifth is owned only incidentally, by a
program that is not tracking schema currency.

**What "24 measurable" does and does not assert.** It is an ADMISSION count: 24 documents pass the
schema gate `eval-extraction` applies before it will score. It is not a demonstration that 24 documents
produce a score. Only **2 of the 24 carry an eval gold at all**, so only those two are scoreable today;
the remaining 22 would be admitted if a gold existed for them. Two limits are recorded rather than
glossed: a schema-3 document that fails canonical verification for any *other* reason is not reported
UNMEASURABLE but propagates as an error and **aborts the run** (`build_predictions`,
`commands/eval_extraction.rs` — `unmeasurable_disposition` returns `None` for any non-schema refusal,
which is deliberate, so a real defect cannot hide inside a disposition); and this census verified
end-to-end scoring only for the two gold-carrying documents, which is all the datasets permit.

Reproduced live at the head of this measurement:

```text
$ ./target/release/specforge eval-extraction \
      crates/specforge/test_data/llm_eval/seed_apb.json --provider skip
  -- UNMEASURABLE documents (no score is reported for these) --
    ihi0024_e_2023_02_amba_5_apb_protocol_specification: persisted EvidenceIR is schema 2, below the
    current canonical schema 3; it is legacy/proofless and inspection-only. Re-ingest the document
    (`specforge ingest <pdf>` then `evidence`/`semantic`) to make it measurable.
      16 gold item(s) withheld from scoring
=== Extraction eval: no measurable document in this dataset ===
```

The binary states the remedy in its own refusal. What is missing is an OPEN owner for it.

## 5. The route is available, and it is in the repository

`WIRE-BASED-100.5d` git-tracked the wire source PDFs precisely so a re-ingest could never be blocked
again by a reclaimed normalized source. All three are present and registered in
`corpus/SOURCE_PDF_REGISTRY.md`: APB `IHI0024_E` (516 KB), AHB `IHI0033_C` (957 KB), AXI `IHI0022_L`
(2.0 MB). `specforge doctor` reports the repo-local Docling runtime ready (Python 3.11.15 /
Docling 2.84.0), and the three currently-measurable `corpus/`-sourced documents — SWD/ADI, I2C, and
I2S — were produced through exactly this route after the `2026-08-12` schema bump, so the route is
demonstrated rather than assumed. It is demonstrated on **these very documents** too:
`CORPUS-PATTERN-REUSE.3c` ran `DOCLING_DEVICE=cpu` re-ingests of APB, AHB, AXI and AXI-Stream on
`2026-06-09`, rebuilt their evidence, and re-scored the wire golds at `1.000` — the same sequence
`.9b`–`.9d` will run, differing only in that the current binary now emits a proof-carrying schema-3
chain. The eval golds are content-anchored (`WIRE-BASED-100.1`), so they survive the re-segmentation a
re-ingest causes; that property is what `.1` was built for.

The scoring path itself does not need a model server: the wire scores are taken with
`--provider skip`, the deterministic pattern baseline.

## 6. What this measurement decides

**GO** on owning the wire re-ingest under `WIRE-BASED-100`, one document per leaf, smallest first, so
each carries its own before/after evidence: `.9b` APB, `.9c` AHB, `.9d` AXI.

**NO-GO** on this leaf absorbing the other two shortfalls. They are real and now recorded, but they
belong elsewhere and must not be smuggled into a wire-protocol tree:

- `1_0_risc_v_debug_specification` is a `PDF-VARIANT-DIGESTION` register-class gold, not a wire spec.
- The 31 refreshed-but-legacy cohort documents are a corpus-program question about what `refreshed`
  should mean, not a wire-protocol question.

Both are surfaced in `WIRE-BASED-100.9`'s node as findings routed out, so the next session finds them
without this tree claiming them.

**A third shortfall this measurement uncovers and also does not own: nothing gates canonical currency.**
The wire chains went legacy because a completed refresh (`CORPUS-PATTERN-REUSE.3c`, `2026-06-09`) was
invalidated by a schema bump three days after their last rebuild, and no check reported it. A gate that
failed when a persisted chain fell below the canonical schema would have caught this on `2026-08-12`
instead of leaving it to be found by a leaf that happened to need the scorer. This census is the
derivation such a gate would use; turning it into an enforced doctrine is a `DOCTRINE-ENFORCEMENT`-class
decision, not a wire-protocol leaf, and it is deliberately left unowned here rather than half-adopted.

**A caution this leaf records rather than resolves.** A re-ingest replaces a persisted chain, and the
legacy artifacts cannot be regenerated by the current binary — the old binary that produced them is
gone. Every wire number published before `2026-08-12` was measured on evidence a re-ingest destroys.
The correct posture is the one `.8c` already established for the retired SWD score: a re-derived
number replaces the old one, and a number that cannot be re-derived is withdrawn rather than carried.
`.9b` preserves the outgoing chain on the repository volume before rebuilding it, so the before/after
comparison is exact rather than remembered.

## Reproduce

```bash
python3 scripts/measure_corpus_canonical_currency.py          # human-readable
python3 scripts/measure_corpus_canonical_currency.py --json   # raw census
```

Read-only; no model, no rebuild, no write. Safe at any time.
