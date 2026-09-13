# Reading A Normative Obligation

A specification states a requirement in one sentence, and SpecForge has to take five separate
things out of it: which signal it constrains, what kind of obligation it is, what value it binds,
what condition it holds under, and whether it is negated. Each of those is read from some span of
the source text, and when two of them come from different spans the published record is a
composite the document never wrote.

This chapter collects the rules that decide those readings, and the cases that produced each one.
They belong together because they answer one question — *what does this sentence actually say about
this signal?* — and because every one of them was written after a real document was read the wrong
way. The [EvidenceIR](evidenceir.md) chapter covers the stage as a whole.

## A statement that states three obligations yields three records

A sentence is the unit a reader thinks in; a *statement* in SpecForge is often bigger — a paragraph,
or a whole serialized table row. When such a statement carries several requirements, each one is its
own fact:

```text
The Manager can assert the AWVALID signal only when it drives valid address and control
information. When asserted, AWVALID must remain asserted until the rising clock edge after the
Subordinate asserts AWREADY.
```

The first sentence carries no requirement at all. The second carries the write-address handshake
invariant, and SpecForge used to publish nothing from this statement: it stopped at the first clause
carrying a modal and never looked further. Reading every obligation recovers that invariant, and with
it four more handshake rules of the same shape in one specification.

The same change fixes a subtler failure, because the kind used to be read from the WHOLE statement
while the subject and condition came from one clause:

```text
| HSELx a | Subordinate | 1 | Each Subordinate has its own select signal HSELx ... When the
Subordinate is initially selected, it must also monitor the status of HREADY ... When a Subordinate
is selected for a non-IDLE transfer, HSELx must be asserted in the same cycle as the address and
other control signals. |
```

The published record said `HSELx must be asserted` **when the Subordinate is initially selected and
must monitor HREADY** — a kind from the third sentence wearing a condition from the second. Narrowing
the kind to the first clause alone would have been worse: that clause states no kind, so the record
would have been refused entirely and a fact the document plainly states would have been lost. Read
per obligation, the third sentence mints the record with its own condition, and the second mints
nothing, because monitoring a signal is not one of the kinds the vocabulary has.

Two readings make this work on real sentences:

* **A condition may come first.** English fronts a condition as readily as it trails one — *"When
  asserted, AWVALID must remain asserted"*, *"If present, AWSTASHNID ... must be driven LOW"* — and
  the subject of such a clause sits after the comma. Without that, the clause has no subject at all
  and the reader used to widen its search to the whole statement to find one.
* **A clause may not borrow another clause's signals.** *"When ACVALID is asserted, it must remain
  asserted"* names its subject with a pronoun. A search widened to the statement supplies the
  previous sentence's `ACADDR`, `ACPROT` and `ACSNOOP` and publishes a requirement about signals the
  sentence does not mention. Bounded to its own obligation, the clause states no subject this reader
  can resolve, and an explicit residual is the honest answer.

Two clauses restating one requirement stay one record. A clause that states nothing typed still
states nothing typed — it does not borrow a kind from its neighbour.

## An obligation in a table cell belongs to whatever precedes its modal

A signal-description table row declares a signal in its name cell and describes it in its description
cell — and that description may state a requirement:

```text
| RRESP | RRESP_WIDTH | 0b000 (OKAY) | Response for transactions on the read channels.
                                       Must be valid when RVALID is asserted. |
```

That is a real obligation on `RRESP`, and nothing in the sentence says so. The subject lives in the
row's **header**, which is exactly what serializing the row into one `| … | … |` statement throws away.
So SpecForge reads these obligations from the structured table, where the row is still a row.

The rule is one sentence of English grammar: **an obligation binds to the nominal that immediately
precedes its modal.** A description cell is prose *about* a signal, not a sentence *whose subject is*
that signal, so each `must`/`shall` clause in the cell is judged on its own:

| what precedes the modal | example | disposition |
| --- | --- | --- |
| nothing — the clause opens with the modal | `Must be valid when RVALID is asserted.` | the row's signal is the subject |
| the row's own signal | `PSTRB must not be active during a read transfer.` | the row's signal is the subject |
| a different nominal | `HBURST_WIDTH must be 0 or 3.` | **refused** — this constrains the width parameter |
| a pronoun | `… it must remain constant throughout a burst transfer.` | **refused** — see below |

The third row is the one that matters for precision. `HBURST_WIDTH must be 0 or 3` sits in `HBURST`'s
own description cell, and a reader that merely looks for a declared signal *somewhere* in the row will
attribute the requirement to `HBURST` — publishing "HBURST must be 0", which the document never says and
which drops the alternative `3` on the way.

The statement-level readers, which see the row only as one serialized `| … | … |` line, used to do
exactly that: AHB's artifacts carried `HBURST must_be_value 0` and `HPROT must_be_value 0`, neither of
which the document states. They now apply the same rule — a row keeps its "subject from the other
cells" reading only when the obligation clause names no subject of its own.

The `HPROT` record is worth one more sentence, because the pipeline caught it twice by different
means. AHB also says *"a Manager sets HPROT[0] HIGH, to indicate a data access"*, so the fabricated
`must be 0` was reported as a **temporal conflict**: `HPROT` taking both `HIGH` and `LOW` at the same
`HCLK` rising edge. The document was being made to look inconsistent with itself by a requirement it
never carried. Removing the mis-subjected record removed the contradiction with it.

The pronoun refusal is deliberate and is an honest residual rather than a guess. Both of these are
signal-description rows, both obligations are headed by `it`, and the referents differ:

```text
| HWRITE | … It has the same timing as the address signals, however,
            it must remain constant throughout a burst transfer. |        → it = HWRITE
| HSELx  | … When the Subordinate is initially selected, it must also
            monitor the status of HREADY … |                             → it = the Subordinate
```

Resolving that is anaphora, not a rule, so SpecForge promotes neither. The serialized rows stay in the
artifact — a residual you can read is better than a requirement invented from a coin toss.

One more property follows from working per clause rather than per cell: a cell stating three obligations
yields three records. The serialized statement keeps only the first, which is why a bulleted
`USER`-signal row used to publish a single constraint no matter how much it required.

The name cell is never authority on its own. A row's signal must resolve through the document's own
declared-signal catalog before any obligation is attributed to it, so a table naming something the
document never declares produces nothing. Production code contains no protocol, vendor, or signal-name
list — only the modal, a closed list of helper words, and a closed list of pronouns.

## A negation belongs to the obligation it modifies

A statement often carries several sentences, and only one of them is the obligation a given constraint
records. SpecForge already took the constraint's **subject** and its **condition** from that one
sentence. It took the `negated` flag from the whole statement — so a `must not` anywhere in the text
inverted a requirement minted from somewhere else entirely:

```text
The DV operand must be 1 for IODIR. A command must not be issued while the queue is full.
```

The first sentence is the obligation. The second supplied a `must not`, and the published record said
`IODIR must NOT be 1` — the opposite of what the document states. The flag is now read from the same
bounded clause the subject comes from, so it can only ever describe the obligation it modifies.

The same change completes two obligations the phrase table only knew in the affirmative. A
specification writes the not-asserted requirement in the negative about as often as in the positive,
and `deasserted` is simply its affirmative spelling:

| the document writes | typed as |
| --- | --- |
| `HEXOKAY must not be asserted in the same cycle as HRESP is asserted.` | `must_be_deasserted` |
| `PSTRB must not be active during a read transfer.` | `must_be_deasserted` → `must_be_low` once polarity is known |
| `The size of the transfer, as indicated by HSIZE, must not be changed.` | `must_not_change` |

Without those, each fell to an untyped default and then carried a negation on top of it — a record
asserting that a signal "must not be stable", which is not a thing any of these sentences say.

The `PSTRB` row is worth following, because it is where the pipeline checked itself. APB states that
obligation twice: once in the `PSTRB` signal-description row above, and once in prose —
*"For read transfers, the Requester must drive all bits of PSTRB LOW."* Two extraction paths reach
those two places independently, and they now produce the same typed constraint. Before this change
they disagreed, and the disagreement was the defect rather than a real ambiguity in the specification.

## A specification writes "the same value" when it means "does not change"

The kind table reads `must be stable`, `must remain stable`, `must hold`, `must not change`. APB
states the same obligation in a form none of those match:

```text
| PAUSER | ... | • PAUSER must be valid when PSELx is asserted.
                 • PAUSER must have the same value in the Setup and Access phase of a transfer. |
```

One value across two phases, or across every cycle of one, is a signal that does not change — so the
phrase now types as `must_not_change` instead of falling through to the fallback.

Where the phrase sits in the table is not cosmetic. A signal-description cell often states several
obligations, and the first phrase that matches types the whole record. Placed ahead of the validity
rule, `must have the same value` took over cells like the one above and `PAUSER must be valid` stopped
being recorded at all. It sits behind that rule instead, so it fires only where nothing else matched —
which is exactly the case it was added for.

The phrases carry their modal, so the many descriptive uses in the corpus — *"implementations that
have the same value"*, *"It does not have the same value"* — state nothing normative and are read as
nothing. And *"the same value **in** the Setup and Access phase"* stays distinct from *"the same value
**as** PWUSER"*: the second is a relation between two operands, which SpecForge refuses rather than
flattens.

## A passive obligation puts its verb where the value goes

When a sentence binds a value — *"must be"*, *"shall be"*, *"must remain"* — SpecForge reads the word
that follows it. In a passive obligation that word is not a value at all:

| the document writes | what the slot holds |
| --- | --- |
| `the entry must be invalidated` | `invalidated` — what happens TO the entry |
| `the tags in memory must be updated` | `updated` — an action |
| `AWTAGOP must be Invalid` | `Invalid` — a state the signal is in |

The first two published typed constraints saying a signal equals `INVALIDATED` and `UPDATED`. The
third is a real value.

The rule is written to be exactly as wide as that difference. A word is accepted as a value when the
**document itself declares it** as one, when it is a logic level, or when it is a numeric literal.
Only the remaining case — a bare past participle nothing in the specification calls a value — is
refused. A protocol whose cache states include `Shared` keeps it, because that protocol declares it.

The validity convention is deliberately outside this test. *"PNSE must be valid when PSEL is
asserted"* types as `must_be_value VALID` by an older and separate rule, where `VALID` comes from the
phrase rather than from a value vocabulary; subjecting it to the same check was measured and reverted
after it retyped seventeen correct records across two protocol specifications.

## A predicate that names a scope or another operand is not a value

The same slot fills with one more thing that is not a value. NVMe writes *"The ANA Group Identifier
(ANAGRPID) for each ANA Group shall be unique within the NVM subsystem"*, which puts `unique` where a
state belongs, and SpecForge published `must_be_value UNIQUE` — a state no signal can equal.

The difference is in the grammar, not in the word. A state is complete at the predicate; a relation
has to say what it holds *against*:

| the document writes | what the slot holds |
| --- | --- |
| `AWTAGOP must be Invalid` | a state — complete at the word |
| `... shall be unique within the NVM subsystem` | a relation — and the scope it holds over |
| `... shall be compatible with all supported lane widths` | a relation — and its other operand |
| `... must be greater than 0` | a relation — and the bound it is measured against |

So the test is positional rather than a list of adjectives: a predicate immediately followed by a
preposition that introduces an operand or a scope is refused, and the sentence stays counted as an
uncaptured normative statement instead of becoming a fabricated value.

*"by"* is deliberately not one of those prepositions. It marks who performs an action rather than
what a relation holds against, and the passive obligations it follows are already refused by the rule
above.

The three routes that admit a value run first and are untouched: a word the document itself declares
as a value, a logic level, and a numeric literal all bind however the sentence continues.

## An obligation that names no kind states no constraint

The constraint vocabulary has a fixed set of kinds — stable, high, low, asserted, deasserted, must
not change, a named value. When a sentence matches none of them, SpecForge used to publish the first
one anyway: `must_be_stable`, the terminal fallback.

That is not a default. It is an assertion about a signal, and the sentences it fired on say nothing
of the sort:

```text
| Manager: False | ARCHUNKEN is not present. RCHUNKV is not present. |
It is recommended, but not required, that PSLVERR is driven LOW when PSEL, PENABLE, or PREADY are LOW.
- T1 FREADY signal remains HIGH.
```

A presence table cell, a recommendation the document explicitly marks as *not required*, and a
waveform step. Every one of them published a signal that "must be stable". Read against source, every
live record on that fallback was wrong.

The statement path now refuses instead. The sentence stays counted as a normative statement no typed
record cites — an honest residual, which is what keeps the gap visible rather than filling it with a
fact the document does not contain.

**The table-row reader kept the fallback for a while, and losing it is the more interesting half.**
That reader proves something the statement path cannot: before it classifies anything it has checked
that the clause binds to its own row's signal — English binds an obligation to the nominal immediately
before its modal, and a clause that fails that test is dropped. So when its classifier found no
phrase, the argument went, what it was holding must be a real obligation about a known signal written
in a spelling the table lacks:

```text
| PAUSER | ... | PAUSER must have the same value in the Setup and Access phase of a transfer. |
```

That IS a stability requirement, and it was the whole population the argument was made about. Once the
table learned that spelling, those clauses stopped reaching the fallback — and what was left reaching
it was a different kind of sentence entirely:

```text
LRADDR bits within the range indicated by LRSIZE must match LAADDR.
Must be aligned to a burst size.
Must not be present.
```

A match against another operand, an alignment, a presence claim. Each one binds to its row's signal
perfectly well; none of them states anything the constraint vocabulary can hold. **Proving the subject
does not make the kind readable**, and the fallback answered `must_be_stable` for all three — for the
last one, negated, so a cell saying a signal must not be *present* published that it must not be
*stable*.

Both readers refuse now, and the terminal arm is unreachable as a published kind. The sentence stays
counted as a normative statement no typed record cites, which is the accounting that keeps the gap
visible rather than filling it.

## One clause, one modal vocabulary

Three different parts of a constraint record each ask the same question of the same clause — *does
this state an obligation, and is it negated?* — and for a while each of them answered with its own
list of modal words:

| what it decides | the modals it read |
| --- | --- |
| is the obligation negated | `must not`, `shall not`, `must never`, `shall never`, `cannot`, `will not` |
| which sentence IS the obligation | `must`, `shall` |
| which kind the obligation states | every phrase in the table is spelled `must` or `shall` |

So a requirement a specification writes with `cannot` was recognised as negated by the first reader,
given no sentence of its own by the second, and typed as nothing by the third:

```text
The PATH_CS_0.PM Packet Support bit cannot be changed after the path valid bit is set to 1b.
```

`must not be changed` types as `must_not_change`. The sentence above is the same obligation, and it
matched nothing. The fix is not another phrase — it is to stop the table needing one phrase per
modal. The equivalent negative modals are reduced to the one form the table is written in before any
phrase is tried, and the sentence scan reads the same set, so all three readers now agree about what
counts as an obligation.

Both halves are necessary, and the reason is worth stating because it is easy to ship only the first.
Teaching the classifier a modal the *sentence* scan still cannot find leaves the record's span falling
back to the whole statement, which for a serialized table row means the row's other cells. Measured
over the corpus, that alone published two constraints whose subject was a row's `NOTE` marker, lifted
from a legacy-considerations note. With both readers taught, the obligation has a sentence of its own,
the marker is not in it, and nothing is published.

The measured effect on the corpus is **no change at all**: re-running the real producer over every
persisted document's own statements yields exactly the records it yielded before — none added, none
removed, none retyped. That is the honest result. The corpus states these obligations about register
fields and packet bits rather than about declared wires, so they are refused a step later for a
different and correct reason; the classifier is now right about them regardless, and the next document
that states one about a wire will be read rather than dropped.

## A value binding does not have to be an obligation

Most of this chapter is about sentences with a modal. A specification also states a binding flatly:

```text
QDENY is tied LOW when denial is not implemented.
AERR, DERR are driven LOW when parity check is suspended during power-down.
```

There is no *must* in either, and both state something a downstream consumer needs. SpecForge reads
them with a separate pass that looks for a bound **value** rather than for an obligation, and the
distinction matters for the same reason every other rule in this chapter does: a record's parts have
to come from the span that produced it, and for these sentences that span is the clause that carries
the **binding**, not the clause that carries a modal.

Looking for a modal in a sentence that has none finds whatever modal the rest of the paragraph
happens to contain. That produced two shapes of wrong record:

| the document writes | the published record | what went wrong |
| --- | --- | --- |
| `... sets PACCEPT HIGH. Once the controller samples PACCEPT HIGH, the device cannot assume ...` | `PACCEPT must be HIGH`, **negated** | the `cannot` is two sentences away and is about something else |
| `If LATRANS is SPEC, LRPROT must be 0. When LRRESP is FaultAbort, this signal is not valid.` | `LRPROT must be 0` **when LRRESP is FaultAbort** | the condition contradicts the record it was attached to |

The binding clause is found by asking the binder itself where it bound, rather than by a second rule
that could disagree with it, and the condition and the negation are then read there. The bound value
is deliberately left exactly as it was: searching for bindings clause by clause finds more of them,
and some of those are wrong for an unrelated reason — a level is currently paired with the verb that
sets it rather than with the signal it belongs to, so `Controller must set PREQ LOW and PREQCHK HIGH`
can attach `HIGH` to `PREQ`. Recall waits for that to be fixed.

## A bound stated against another operand is not a value

The constraint vocabulary can say *"this signal must be `HIGH`"*, *"must be stable"*, *"must not
change"*. It has no way to say *"this field is bounded by that field"*, and a specification says that
constantly:

```text
The range given by this field must not be greater than the size indicated by the OAS field
of the DTI_TBU_CONDIS_ACK message.
```

Nothing in that sentence is a value, and nothing in it is stable. Read as a value constraint it
produced two records asserting that `OAS` and `DTI` **must not be stable** — a fact the document does
not state, attached to a signal it does not constrain. `OAS` is the right-hand operand of the
comparison; `DTI` is the leading fragment of a message name.

SpecForge refuses the sentence instead. That is the same answer it already gives to an inter-operand
*equality* (*"X must be equal to the value of Y"*): an explicit residual is honest where a fabricated
constraint is not, and the relation stays visible in the evidence for a later stage that can hold it.

The line is drawn at what the comparison is made against, not at the comparison itself. This rule
asks only whether the bound is another operand:

| the document writes | SpecForge |
| --- | --- |
| `... must not be greater than the size indicated by the OAS field` | refused here — the bound is another operand |
| `... must not be wider than the value of the ALLOW_PW field` | refused here — same shape |
| `The value of PRANGE must be greater than 0` | not refused here — the bound is a literal |
| `... sets DBI HIGH when the number of transitioning data bits within a byte is greater than 4` | kept — a count against a literal, and the binding is the level |

A magnitude against a literal is not a *reference* magnitude, so this rule leaves it alone. It does
not follow that such a sentence yields a constraint: its value slot holds `greater`, which the
scope-or-operand rule above refuses for its own reason. The vocabulary has no *at least* kind, so both
shapes end as explicit residuals — they simply reach that answer through different rules.

The reference phrase must follow the comparative immediately. A sentence that merely mentions "the
value of X" somewhere and separately compares against a number is not this shape, and is untouched.

**The refusal is a property of the rule, so every reader applies it.** For a while it was not: both
refusals were wired into the prose reader only, and the table-row reader — which reaches obligations
the prose reader cannot see at all — asked the same sentence the same question and got a fabricated
answer. A description cell stating the comparison above published *"must not be stable"*, and one
stating the equality published a value binding whose value was the word `value`, lifted straight out of
the phrase *the value of*. Both readers now refuse, for the same reason, on the clause each is holding.
No corpus document currently states either relation in a signal-description cell; the rule is there
because the next one will.
