# Arm B — the repaired rule, scored against arm A, and what it does to the pre-registered bar

Owning leaf: `BOUNDED-DECISION-PROVIDER.1a.1` (MEASURE, sub-arm **B1**).
Producer: `python3 scripts/score_bounded_decision_arms.py`.
Scored on the frozen set `docs/research/bounded-decision-adjudication.jsonl`
(sha256 `d3c5898f…`), which `BOUNDED-DECISION-PROVIDER.1` pinned and this leaf does not touch.

## Why this leaf decides the tree

`.1a` exists to stop the evaluation making the only mistake that would guarantee a wrong answer:
measuring a remote provider against known-defective rules, finding it better, and concluding it is
necessary. Almost anything beats a defective rule. The only comparison that can justify a remote
dependency is against **the best thing that can be done locally**.

This record is sub-arm **B1 — the repaired rule**. Sub-arm B2 (the local model tier through
`system-one-adapter-python` against Ollama) is `.1a.2`, and [§7](#7-why-b2-cannot-change-the-answer)
states why its outcome cannot change what follows.

## 1. What B1 proposes

Four rules. Three are grammar and one is a scope correction; none is a vocabulary list standing on
its own. **Nothing here is shipped** — every rule is a candidate scored against the frozen set, so
`.1`'s pinned baseline stays valid and shipping remains a separate decision.

### `caption_admission`

| rule | statement |
| --- | --- |
| **R1** | **A title is not a statement.** A caption-shaped text with no sentence terminator anywhere has no finite main clause; it is a label plus a noun phrase, and a deontic word inside it qualifies a noun. This is what separates `Table A8.2: Opcodes which must be cache line sized` from `Other combinations are not permitted.` |
| **R2** | **A cross-reference reports, it does not oblige.** A sentence that OPENS with a figure/table label followed by a reporting verb describes its referent, so a deontic inside it belongs to the thing referred to. |
| **R3** | **Negated permission is deontic.** Route `r1` carries `must`/`shall`/`required` and misses `(is\|are) not permitted` and `no … (is\|are) allowed`. |

**R2 is anchored to the sentence OPENING, and that is load-bearing rather than stylistic.** The
obvious rule — *refuse when a reporting verb precedes the deontic* — refuses a real prohibition:

```text
The bit combinations that Table 3-7 does not show, are not permitted.
```

`show` precedes `are not permitted` and the sentence is a genuine prohibition, because the reporting
verb sits in a relative clause while the deontic heads the main one. Anchoring to the opening keeps
it. `--self-test` case `r2-is-anchored-to-the-opening` is the control that stops that regressing.

### `declaration_row`

| rule | statement |
| --- | --- |
| **R4** | **A whole-cell direction abbreviation under a `Direction` header is a direction.** Scoped to a row the reader dropped for `no_direction_and_no_width`, so its name was already an identifier. |

R4 is deliberately small. `SIGNAL-DECLARATION-ROW-DROP.2h.0` refused these abbreviations on a census
of the **fallback** population — columns whose header names no direction — where a presence matrix
writing `O` for *Optional* is the hazard. That hazard cannot arise under a header that says
`Direction`. The corpus-wide population is `SIGNAL-DECLARATION-ROW-DROP.2j`'s census, not this
leaf's; here R4 is scored only for what it does to the frozen set.

## 2. The result

```bash
python3 scripts/score_bounded_decision_arms.py
```

| decision | arm | tp | fp | fn | tn | errors | precision | recall | macro-F1 |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `declaration_row` | A | 567 | 0 | 22 | 55 | 22 | `1.0000` | `0.9627` | `0.90715` |
| `declaration_row` | **B1** | 568 | 0 | 21 | 55 | **21** | `1.0000` | `0.9644` | **`0.91077`** |
| `caption_admission` | A | 2 | 5 | 4 | 602 | 9 | `0.2857` | `0.3333` | `0.65014` |
| `caption_admission` | **B1** | 5 | 0 | 1 | 607 | **1** | `1.0000` | `0.8333` | **`0.95413`** |

**`caption_admission`: +0.30399 macro-F1, 9 of 9 arm-A errors corrected, no false positive
introduced.** All five false positives and all four false negatives are gone; the one remaining
error is new, and it is stated below rather than tuned away.

**`declaration_row`: +0.00362, one row corrected.** That is not a disappointment, it is the
measurement `.1` predicted: this decision's errors are not in this decision. Of the 21 that remain,
6 need a different cell (`PROSE-NAME-CELL-DECLARATION`'s question), 8 state no attribute anywhere,
and 7 lost their attribute to text-layer garble before any rule could read it.

### 2.1 The one row B1 loses

`statement_1426` (AXI): *"Table A4.7 shows the physical address spaces with signal encodings and
property that must be True to enable a Manager to use that encoding. **An interface can include
AxPAS or AxPROT/AxNSE signals, not both.**"*

R2 correctly refuses the first sentence — it opens with a label and a reporting verb. The second
sentence is a real prohibition and carries **no deontic B1 recognises**: the obligation is expressed
as `can … , not both`. Arm A admitted this row by accident, on the `must` inside the reported clause
R2 exists to refuse, so B1 trades an accidental true positive for a principled refusal. Extending R3
to `can … not both` was **refused**: one row is not a grammar, which is the same bar `.2b` applied to
the leftward arrow and `.2h.0` applied to the abbreviations.

## 3. The blast radius, adjudicated in full

R3 changes route `r1`, which every statement passes through — not only captions. So what it newly
admits is a corpus question, and a cheap structural rule over-fires until its selection is inspected
([[a-cheap-structural-rule-overfires-until-you-read-its-selection]]).

```bash
python3 scripts/score_bounded_decision_arms.py --blast-radius
```

Over all **15,026** statements in the four documents, counting only statements route `r1` does not
already admit — **every row printed, none sampled**:

| form | newly admits | adjudication |
| --- | ---: | --- |
| `(is\|are\|was\|were) not permitted` | **10** | **10 of 10 are genuine prohibitions** — *"PEs are not permitted to directly access either of these new PAS"*, *"Cache stash transactions are not permitted to cross a cache line boundary"*, *"it is not permitted for the update to propagate to main memory"*, *"It is not permitted for another observer to see some data bytes updated…"* |
| `no … (is\|are) allowed` | **2** | both genuine, and both are the frozen set's own false negatives |

**12 of 12 correct.** Two wider forms were tried and **refused on their own selection**:

| refused form | would have admitted | why it was refused |
| --- | ---: | --- |
| `prohibited` (bare) | 10 | 4 are the licence boilerplate *"TO THE EXTENT NOT PROHIBITED BY LAW…"*, 2 are the No-Allocate allocation **hint**, and 4 are ADIv6 figure titles (*"Figure D2-2 Prohibited duplicate ROM Table reference"*). Zero of the ten is an obligation on the interface. |
| `(is\|are) not (legal\|valid)` | 7 | serialized response-code rows and data-validity statements — *"If RRESP is TRANSFAULT, the read data in that transfer is not valid"*. That is a statement about a transfer's contents, not a requirement. |

The refusals are carried in the producer as `REFUSED_FORMS` and re-derived by `--blast-radius`, so
the refusal is a measurement a reader can reproduce rather than a claim about how hard anyone looked.

## 4. The effort each arm received, stated as `.1a` requires

**Arm A** was scored by `.1` on the product's real decision, not on an intermediate counter — the
distinction that turned six apparent false positives into the true negatives they are.

**Arm B1** got: the full disagreement set from `.1` as its target; four candidate rules; a corpus-wide
adjudication of every statement each admitted form would newly reach; and two wider forms tried and
refused on measurement rather than taste. What it did **not** get: a rule fitted to
`statement_1426`, which was available and was refused for the stated reason; and any tuning against
the 21 remaining `declaration_row` rows, because their evidence is not in the row.

**What could still make B1 stronger, and is not claimed here:** the four documents are one stratum,
and shipping R1–R4 needs the same adjudication across all 78 persisted documents. That is the
shipping leaf's work, not this measurement's.

## 5. What this does to the pre-registered bar

`.1` fixed the bar before any model ran. **C1** requires arm C to beat arm A **and arm B** each by
**≥ 0.05 absolute macro-F1**. Applying it to the numbers above:

| decision | arm B1 | arm C would need | verdict |
| --- | ---: | ---: | --- |
| `caption_admission` | `0.95413` | **≥ `1.00413`** | **arithmetically impossible** |
| `declaration_row` | `0.91077` | ≥ `0.96077` | possible only by correcting **13 of the 21** remaining rows |

**`caption_admission` is closed to a remote provider.** Not because the provider was measured and
lost, but because a local deterministic repair reached a score no arm can beat by the margin the
director's rule requires.

**`declaration_row` is closed for a different and equally hard reason.** C1 needs **13 of the 21**
rows corrected — the producer derives that rather than anyone computing it, after the first draft of
this record published `9`, which is C2's separate error floor and reaches only `0.94567`. C2 wants
errors down to ≤ 12 and ≥ 9 rows; C3 requires **no** new false positive. The 21 remaining rows are 6 whose
name is not in the column the decision is about, 8 that state no attribute at all, and 7 whose
attribute the text layer destroyed. To correct thirteen of them a ranking model would have to admit
identity with no attribute — the exact rule this repository has measured three times and refused,
most recently at **24% precision, 14 real against 45 phantom**
([[a-dropped-declaration-row-is-usually-not-a-signal]]). That buys recall with false positives, which
C3 forbids independently of the score.

**So under the bar as written, arm C cannot clear either decision** — established with zero egress,
no API key, and no model run.

### The honest limits of that conclusion

- It is the bar's arithmetic, not a measurement of Jev. Arm C has not been run and this record makes
  no claim about what it would score.
- It rests on arm B1 being a legitimate arm B — a repair that could actually ship. The blast-radius
  adjudication is what supports that, and it covers four documents, not the corpus.
- If the director judges the `0.05` margin too strict now that a local arm is strong, that is a
  decision to change the bar — which `.1` recorded precisely so it would have to be made openly
  rather than by drift.

## 6. Recommendation

`.1a`'s own terms: *"if B closes the gap, this leaf recommends shipping the local fix and closing the
tree at `.6` with no provider."* It closed the gap on the decision where the gap was.

1. **Ship R1–R3 under `INVARIANT-SHAPE-ADMISSION`**, with the corpus-wide adjudication across all 78
   documents that shipping requires, and R4 under `SIGNAL-DECLARATION-ROW-DROP.2j`, which already
   owns its census.
2. **Close `BOUNDED-DECISION-PROVIDER` at `.6` with no provider**, unless the director elects to run
   arm C anyway or to revisit the margin.
3. **Do not buy the `TYPESAFE_API_KEY` for this evaluation.** It was blocked on procurement; the
   keyless work has now answered the question the key was going to be spent on.

## 7. Why B2 cannot change the answer

Arm B is **the best local alternative**, so it is the maximum over its sub-arms. B2 can only raise
arm B or leave it where B1 put it; it cannot lower the bar arm C has to clear. Since arm C is already
excluded at B1's scores, **B2's outcome cannot change `.6`'s answer.**

B2 therefore stays open in `.1a.2` for completeness and fairness — the tree's rule that arm B must
get comparable effort cuts both ways, and a local model tier is worth knowing about on its own — but
it is recorded as **not decision-relevant**, so the director can decide whether to spend the setup
(`ollama serve`, a repo-volume venv, the MIT adapter) on it.

## 8. Re-derivation

```bash
python3 scripts/score_bounded_decision_arms.py                 # arm A against arm B1
python3 scripts/score_bounded_decision_arms.py --blast-radius  # every newly admitted row, in full
python3 scripts/score_bounded_decision_arms.py --self-test     # 11/11 RED cases
```

The arm scorer imports the frozen-set reader from the baseline producer rather than re-reading it,
so the two cannot drift into two opinions about the population, and its first RED case asserts that
arm A reproduced here equals the matrices `.1` pinned.
