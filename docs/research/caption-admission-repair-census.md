# Caption-admission repair — the whole-corpus census, adjudicated row by row

Owning leaf: `INVARIANT-SHAPE-ADMISSION.6` (MEASURE + ADJUDICATE, before any production change).
Producer: `python3 scripts/measure_caption_admission_repair.py`.

## Why a second census

`BOUNDED-DECISION-PROVIDER.1a.1` built a deterministic repair of `is_invariant_like`'s caption
handling and measured it on a frozen four-document set, where it corrected 9 of 9 errors and
introduced none. That is a score, not a licence to ship. This repository's standing rule says why: **a
cheap structural rule over-fires until someone reads what it selects**
([[a-cheap-structural-rule-overfires-until-you-read-its-selection]]), and four documents are not the
corpus.

So the same rules were put to **all 78 persisted documents — 261,508 statements, 13,136 of them
caption-shaped** — and every row they move is printed and adjudicated here.

## The population

| half | what it is | rows |
| --- | --- | ---: |
| **removals** | caption statements route `r1` admits today that R1 or R2 would refuse — a **precision** change, whose risk is deleting a real requirement | **71** |
| **additions** | non-caption statements route `r1` does not admit that R3 would — a **recall** change, whose risk is admitting a non-obligation | **176** |
| **caption additions** | caption statements route `r1` does not admit that R3 would, added by `.6a.1` when the production rule turned out to have a third population this census had skipped past (§2.5) | **6** |

## 1. Removals — 71, and no requirement is lost

| rule | rows | what they are |
| --- | ---: | --- |
| **R1** a title has no finite main clause | 15 | *"Table D5-3 Required snoop transaction behavior"*, *"Table 4-1 Registers that must be initialized"*, *"Table 6-3: DisplayPort Required Bandwidth (Gbps)"* |
| **R2** a cross-reference reports rather than obliges | 56 | *"Table B2-5 shows the required behavior of a CoreSight component…"*, *"Figure 3-15 shows the integration required between…"*, *"Table B15.1 shows the interface states and the rules that the Requester must follow…"* |

**Sixty-nine carry no self-contained requirement.** They are labels, or sentences whose main verb is
`shows`/`lists`/`summarizes` and whose deontic word describes the referent.

**Two do state a requirement, and they are the interesting ones.** TileLink `1.7.1` and `1.8.0` both
carry the figure caption:

```text
Figure 3.1: Valid must be driven LOW for at least 100 cycles during reset
```

That is a finite clause — subject, modal, predicate — packaged as a caption, and R1's structural
proxy (*no sentence terminator, therefore a title*) refuses it. **The proxy is wrong for this
shape**, and the honest disposition is recorded rather than the count adjusted:

> Each document states the same rule in prose, independently of the figure —
> *"Before deasserting reset, a valid, c valid, and e valid must be driven LOW by the master, while b
> valid and d valid must be driven LOW by the slave…"* — and route `r1` admits that sentence on
> `must`. **The requirement survives the removal in both documents.**

So the precision change costs no requirement corpus-wide. The proxy's blind spot is real and is
recorded as a known limit of R1 rather than repaired: one distinct sentence, in two editions of one
specification, is not a grammar — the same bar `.2b` applied to the leftward arrow and `.2h.0` to the
direction abbreviations.

## 2. Additions — 176, and the corpus supplied three refinements

The four-document census adjudicated 12 additions and found 12 of 12 correct. At corpus scale the
same forms select **176**, and reading them produced three narrowings, each costing nothing else.

### 2.1 `was`/`were` is dropped

The form `BOUNDED-DECISION-PROVIDER.1a.1` scored was `(is|are|was|were) not permitted`. Corpus-wide
the past-tense arm admits **exactly two** rows, both from the same document:

```text
Prior to Issue G, the UC, UD, and UDP initial cache states at the sending of a
ReadPreferUnique were not permitted.
```

A prohibition scoped to a superseded issue is **document history**, not this document's requirement.
Dropping `was|were` removes both and nothing else.

### 2.2 The negated-existential window is tightened

`no … (is|are) allowed` was scored with a window of `[^.]{0,80}`. Corpus-wide that matched across a
clause boundary, once:

```text
When a transaction includes a No\_snoop == 1 flag, it indicates that the transaction is
allowed to 'opt-out' of hardware cache coherency
```

The `no` belongs to a **signal name**, and the permission is *granted*, not withheld — the exact
inversion of what the rule is for. A negated subject and its verb share one clause, so the window
becomes `[^.,;:]{0,60}`: a comma between them means they are not subject and predicate of the same
clause. Measured, that drops the one false row and keeps all eight real ones, including both rows the
frozen set depends on.

### 2.3 Serialized table rows are counted as their own stratum, not refused

**38 of the 176** begin with `|`. They are *not* refused here, and that is deliberate: route `r1`
already admits a table row carrying `must`, and `INVARIANT-SHAPE-ADMISSION.0` measured **769** such
rows and refused to delete them because 699 carry content found nowhere else. These enter on exactly
the same footing. They are reported separately so a reader does not take the recall change for 176
sentences of prose when **138** of it is.

### 2.4 What the 176 are

Overwhelmingly genuine prohibitions, across 27 documents and 129 distinct texts (the corpus holds
several editions of some specifications, so rows repeat):

```text
A cache state change from UC to UCE is not permitted.
Cache stash transactions are not permitted to cross a cache line boundary.
The merging of transfers with different TID or TDEST values is not permitted.
No device is allowed to acknowledge at the reception of the START byte.
During INTERMISSION no station is allowed to start transmission of a DATA FRAME or REMOTE FRAME.
```

**Two residual classes are named rather than fixed**, because neither has a shape rule that does not
also refuse real requirements:

- **Revision-history entries** — *"| Correction: Use of SnpDVMOp is not permitted | B7.1 Introduction |"*,
  *"Appendix A.2: Added clarification that sending the Alert Response Address … is not permitted"*.
  About **4 rows**. They are prohibitions in content and change-log entries in function, and telling
  them apart needs a section-kind signal the admission layer does not have.
- **Subjectless bullet continuations** — *"- Is not permitted to send DVMReq or DVMSync requests to a
  host."* The prohibition is real; its subject is in the preceding bullet. That is
  `INVARIANT-SHAPE-ADMISSION`'s separate subject question, not an admission defect.

## 2.5 The third stratum — `.6a.1`, found by writing the production rule against this census

This census measured two populations. **The rule it is a census of has three**, and the gap was found
by writing the Rust and discovering the code had a cell the table did not:

| | route `r1` admits today | route `r1` does not |
| --- | --- | --- |
| **caption-shaped** | §1 removals — 71 | **never counted** |
| **not caption-shaped** | unchanged | §2 additions — 176 |

Production evaluates a caption against R1 + R2 + **R3**, exactly as `repaired_caption_admits` does, so
a caption route `r1` does not admit can still be admitted by R3's second sentence. The first pass
`continue`d past that cell. Worse, RED 3's `r2-is-anchored-to-the-opening` case is *precisely* a
member of it — the rule was unit-tested on the shape while no corpus row of the shape was enumerated,
which is the difference between a case and a census.

**Measured: 6 rows, across 5 documents, every one printed and adjudicated.**

**Four are requirements the pipeline does not publish today, and gains:**

```text
Table A13.14 shows the legal combinations of AxMMU signals and PAS. Other combinations are not permitted.
Table 3-7 shows the mapping between HPROT[6:2] signaling and the memory type. The bit combinations
  that Table 3-7 does not show, are not permitted.
Figure B5-3 shows that the SWD-to-JTAG sequence begins with two SWDIOTMS LOW cycles after the line
  reset. No additional SWDIOTMS LOW cycles are allowed.            (and the same for Figure B5-8)
```

Each is a caption whose first sentence R2 refuses as a cross-reference and whose **second** sentence
is a self-contained prohibition. The AHB row is the sentence R2's opening-anchor exists for.

**Two are table-reading descriptions, and they are named rather than refused:**

```text
Combinations of initial state and end state that are not permitted as a result of a snoop transaction
  are excluded from Table D5-4.                                    (AXI-H)
Empty table cells indicate a combination that is not permitted or not applicable.                (CHI)
```

In both the main verb is `excluded`/`indicate` and the deontic sits in a subordinate clause: they say
something true about how to read a table, not what an implementation must do. **They are admitted.**
Refusing them needs a main-clause-binding rule written for two rows out of 261,508, and two rows are
not a grammar — the bar `.2j` applied to the direction abbreviations and `.2b` to the leftward arrow.
They join the revision-history entries and subjectless bullets as a named residual class.

**Net for the third stratum: +6 published constraints, 4 of them real requirements, 0 lost.** The
self-test pins the population at 6 and adds two checks a unit test cannot give — that the stratum is
enumerated at all, and that it is counted separately rather than folded into the 176, where six rows
would have been invisible.

## 3. What this census establishes, and what it does not

**Establishes:** the repair's corpus-wide selection has been read, not sampled; the precision half
costs no requirement; the recall half is 176 rows whose composition is stated; and three narrowings
were found by reading the selection that four documents could not have surfaced.

**Did not establish, at the time it was written** — and shipping needed all of it. `.6b` shipped on
`2026-09-19` and discharged both; the record is kept in the past tense rather than deleted, because
the adjudication above is what earned the change:

- **No production code had changed.** `is_invariant_like` was untouched; this was the adjudication
  that had to precede the change. `.6b` then moved the caption test **before** the modal route and
  added R3 to the non-caption path.
- **The chain was not rebuilt.** `.6b` rebuilt the 27 proof-carrying documents from the semantic
  stage down; 7 moved, `check_chain_currency.sh` reports 27/27 current, and reverting the rule
  reproduces the pre-change artifacts byte-for-byte.
- **This census still measures the RULE over the evidence corpus, not the shipped delta.** It reads
  `generated/evidence_ir/`, which the change does not touch, so its three counts are unchanged by
  shipping and continue to serve as the "the corpus has not moved" pin. The **published** delta is a
  different and smaller number — `-8 / +15` over the proof-carrying stratum — because this census
  compares against route `r1` alone while production has three routes.
- **`BOUNDED-DECISION-PROVIDER.1a.1`'s published scores are unaffected**, deliberately. Its producer
  is pinned evidence for a published measurement and is left untouched; the narrowings live here, in
  the shipping census. Both rows its frozen set depends on still admit under the narrowed forms, and
  that is asserted by a RED case rather than assumed.

## 4. Re-derivation

```bash
python3 scripts/measure_caption_admission_repair.py            # the census
python3 scripts/measure_caption_admission_repair.py --rows     # every moved row, in full
python3 scripts/measure_caption_admission_repair.py --self-test # 15/15 RED cases
```

The self-test pins the census's shape — 71 removals (15 title / 56 cross-reference), 176 additions
(168 / 8, of which 38 are table rows) and 6 caption additions — so a corpus rebuild that moves it
**fails loudly**. That is
the intended behaviour: the adjudication above is of *these* rows, and a moved population has to be
adjudicated again rather than assumed to carry.
