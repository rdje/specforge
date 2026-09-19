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
| **additions** | statements route `r1` does not admit that R3 would — a **recall** change, whose risk is admitting a non-obligation. It reaches every statement, not only captions, because route `r1` does | **176** |

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

## 3. What this census establishes, and what it does not

**Establishes:** the repair's corpus-wide selection has been read, not sampled; the precision half
costs no requirement; the recall half is 176 rows whose composition is stated; and three narrowings
were found by reading the selection that four documents could not have surfaced.

**Does not establish** — and shipping needs all of it:

- **No production code has changed.** `is_invariant_like` is untouched. This is the adjudication that
  has to precede the change, not the change.
- **The golds are not re-scored.** `INVARIANT-SHAPE-ADMISSION.1` warns that a caption-route change
  rebuilds the chain for most documents; that cost is unchanged and unpaid.
- **`BOUNDED-DECISION-PROVIDER.1a.1`'s published scores are unaffected**, deliberately. Its producer
  is pinned evidence for a published measurement and is left untouched; the narrowings live here, in
  the shipping census. Both rows its frozen set depends on still admit under the narrowed forms, and
  that is asserted by a RED case rather than assumed.

## 4. Re-derivation

```bash
python3 scripts/measure_caption_admission_repair.py            # the census
python3 scripts/measure_caption_admission_repair.py --rows     # every moved row, in full
python3 scripts/measure_caption_admission_repair.py --self-test # 12/12 RED cases
```

The self-test pins the census's shape — 71 removals (15 title / 56 cross-reference) and 176 additions
(168 / 8, of which 38 are table rows) — so a corpus rebuild that moves it **fails loudly**. That is
the intended behaviour: the adjudication above is of *these* rows, and a moved population has to be
adjudicated again rather than assumed to carry.
