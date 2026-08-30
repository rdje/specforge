# Claim Verification Standard

This is SpecForge's repository-owned standard for deciding whether a current assertion is earned strongly enough
to publish for someone else to act on. It is the fifth portable architecture, alongside task trees, durable
memory, the Knowledge Map, and doctrine enforcement.

> Re-derive · falsify · make durable. Repeating one kind of check does not create independent evidence.

⛔ **State every rule here in domain-free terms first; an example is an instance, never the statement.** A rule
that can only be applied by matching the shape of its illustration is under-specified for everyone but its author.
This binds the rules written *from* this standard as much as the ones in it: when a task leaf, checklist, or review
note states a rule only through the case that produced it, the next reader applies it only to cases that look like
that case.

The standard is normative for authoring and review. **The bounded registry and gate are active** through
`doctrine/claim_verification/claims.jsonl`, `scripts/check_claim_verification.pl`, and the doctrine driver. The
gate proves record shape, tracked current inputs, rerunnable evidence commands, stale-state failure, and exact
publication-ID resolution; it does not turn a syntactically valid record into semantic truth.

## 1. Scope

A **published claim** is a current-facing assertion about the repository, product, workflow, or measured system
whose truth may change and whose truth could influence a technical or operational decision. Typical examples are:

- current counts, percentages, capacities, coverage, precision, recall, completeness, or defect totals;
- current compatibility, performance, reproducibility, freshness, or pass/fail assertions;
- a carried constant said to describe repository-derived or externally measured state;
- a stochastic comparison, latency, throughput, quality, or resource observation.

The following retain their existing authority and do not need a claim record merely because they contain a
number or exact token:

- an authored policy threshold or architecture choice whose authority is a decision record;
- a schema id, format version, date, digest, path, command literal, or example value used as itself;
- a dated historical observation that is clearly scoped to its original boundary and not reused as current;
- an immutable evidence capture whose currentness is not asserted;
- narrative judgment that makes no falsifiable current-state assertion.

Reusing any historical observation as evidence about the current tree creates a new current claim. A label does
not decide scope: if a reader can reasonably act on the assertion as current truth, it is governed.

## 2. What a check still permits

Before trusting a check, ask the one question that decides whether it is evidence: **what class of defect does this
check still permit?** A check that cannot fail for the defect in front of you is not a weak check; it is not a check.

| the check | catches | still permits |
| --- | --- | --- |
| `sum(parts) == total` | dropped rows, double counts, arithmetic slips | any **redistribution between parts** |
| `count(rows) == expected` | truncation, a missed input | a wrong value in every row |
| a digest over the inputs | stale inputs | every logic defect downstream of them |
| a digest over a governed prose region | that the region's text changed | every number inside it going stale while the text stands |
| tests written from the same document as the implementation | transcription slips | every **misreading** of that document |
| a ticked acceptance box | a forgotten step | the step being done wrong |
| a per-item assertion checked against per-container data | dropped items, wrong values | every item whose own answer differs from its container's — and it reproduces perfectly while getting it wrong |
| per-bucket agreement with an independent instrument | misassignment, redistribution, wrong bucketing | genuinely little |

**The general form: a check and the thing it checks must not share a parent.** When a control and the thing it
controls descend from the same understanding, their agreement carries no information. That is why “I wrote tests and
they pass” is strong evidence about a transcription error and nearly none about a specification error.

The fourth row is this repository's own recurring instance, and it is the reason the general form belongs in a
normative standard rather than in rationale: a digest-bound live document proves that a region has not changed,
never that the numbers inside it still re-derive. Published counts have gone stale under a fully green gate for
exactly that reason; the enumerated instances are held by `CLAIM-VERIFICATION-ADOPTION.7`.

## 3. The three legs

### Leg 1 — re-derive from the source

Name one repository-root-relative command or exact source accessor that reproduces the asserted value from its
canonical authority. Keep the relevant output or a bounded derived artifact. Synchronizing the same literal in
several documents is not re-derivation.

The leg must identify:

- canonical source or input set;
- exact command/accessor and relevant output field;
- deterministic comparison rule, or the stochastic protocol described below;
- evidence boundary or input identity that says which tree/source the result describes.

An external input may use a documented placeholder such as `<external-input>` at the command boundary. Persisted
project paths remain repository-root-relative; a machine-specific absolute path is never claim authority.

**Decide what you are re-deriving before you re-derive it.** A command that reproduces reliably still answers only
the question it was given. When the assertion is about an item and the available data is about that item's
container — a line and its file, a region and its denominator, a member and its group — the container's answer is an
over-approximation, and the check will confirm it faithfully while the item's own answer differs. Match the
granularity of the evidence to the granularity of the claim, and say which level was used.

**A value appearing in *N* documents has *N* chances to be stale.** Prefer one derived source to *N* synchronized
copies: route the reader to the producer's own report rather than restating its output. A constant that is a
function of this repository is **derived or gated, never carried** — and “it has not moved in *N* revisions” is not
a licence to carry it, because a trajectory shows what has not happened, never what cannot.

**Attribute a RED check by revert-and-re-apply, not by reading.** Restore the prior state, confirm the check passes,
re-apply, confirm it fails; that replaces an argument about which change was responsible with a fact, including when
the answer is the auditor's own change. Where the relevant history is tracked, the read-only form is equivalent and
cheaper: re-derive the value from each revision's own producer input (`git show <rev>:<path>`) instead of reasoning
about what a commit must have done.

### Leg 2 — falsify with a dimensionally different oracle

State a competing hypothesis and name evidence capable of separating it from the published interpretation. The
oracle must not merely repeat the primary producer's classifier, arithmetic, or reading of the same prose.

A valid falsification leg identifies:

- the concrete competing hypothesis or defect class;
- the independent observation or controlled mutation that distinguishes it;
- a tracked known-bad case in which the cited control is observed going RED;
- the result that rules out, narrows, or exposes the competitor.

For a registry-backed verified claim, the cited control also binds one named known-bad case to an exact
line-range/SHA-256 region in its tracked producer. The gate executes the whole control and rejects a stale or
misdirected case region; a self-test name without durable mutation evidence is not enough.

Conservation totals cannot falsify bucket misassignment. A source-derived row count cannot falsify a wrong value
in every row. Tests and implementation derived from the same interpretation do not independently validate that
interpretation. These are instances of §2's general form; when no dimensionally different oracle exists, record the
leg as missing.

⛔ **Evidence consistent with both hypotheses illustrates; it does not test.** Two explanations that predict the same
observation are not separated by *more* of that observation. This applies to a claimed **mechanism** exactly as it
applies to a claimed number: an account of *why* a value moved is a published claim, and it is earned only by an
observation the competing account would not have produced.

**A claim about a set carries its enumeration, in both directions.** “Nothing checks X”, “no test covers Y”, “this is
the only caller” are each refuted by a single counterexample, so each is a census rather than an impression and the
enumerating command belongs beside it. The mirror is equally wrong: a search returning *N* hits yields a
**population**, not a count of defects. Classify the population before publishing its size, or a false negative has
merely been traded for a false positive.

**Prefer an oracle you did not build**, and look for one already sitting in the inputs; it is free. ⭐ The cheapest
such oracle is this project's own history: before publishing a finding, check whether a case of the same shape has
already been adjudicated — in the owning task tree, the commit body that introduced it, or a decision record. If it
was ruled the other way, the new finding must name the difference; if no difference can be named, the earlier ruling
wins.

**Derive classifiers, vocabularies, and membership tests from the producer** — the code or corpus that emits the
thing — never from a description of the producer. A candidate list authored from what a surface is believed to
publish will miss what it actually publishes, and the resulting control is green precisely where it is blind.

### Leg 3 — make the result durable and stale-detecting

The producer, oracle/control, manifest, and every artifact dependency are tracked or are explicit read-only
external inputs. A deterministic gate must fail when any tracked dependency moves without the claim being
re-derived or deliberately marked incomplete.

The durability leg identifies:

- tracked producer and control paths;
- complete artifact-input identity, including the producer that wrote the artifact;
- a repository-root-relative stale-state check;
- retained evidence or a deterministic regeneration route;
- the lifecycle owner responsible for refresh, supersession, or explicit incompleteness.

An ignored script, scratch notebook, untracked fixture, or comment saying “re-run this” cannot support a verified
claim. Expensive re-measurement may use a cheap identity gate, but the identity must cover every dependency of the
artifact, not only the headline metric.

⛔ **Replacing a wrong unwatched number with a right unwatched number is not a fix.** The repair that closes a
staleness defect changes what observes the value, not only the value. Once the cost of re-deriving is known, “it was
expensive” is not available as a reason to leave it unwatched — that is what a cheap identity gate is for.

## 4. Publishing contract

Every commit and pull-request description carries exactly one declaration:

```text
Published-claims: none
```

or:

```text
Published-claims: <claim-id>[, <claim-id>...]
```

Use `none` only when the slice introduces or changes no in-scope current assertion. Normative policy and code may
legitimately use `none`; a completion/status update that publishes current counts or pass claims may not.

Each listed claim ID must resolve through `doctrine/claim_verification/claims.jsonl`. The gate validates the
nonempty `git_message_brief.txt` prepared by the commit workflow, or otherwise the current `HEAD` message, and
rejects missing, duplicate, unknown, or superseded declarations. Inline current-facing prose uses the compact tag
`[claim: <claim-id>]` next to the assertion when a reviewer cannot otherwise map it unambiguously.

A claim record has one of these honest outcomes:

- **verified** — all three legs are present, current, and internally consistent;
- **incomplete** — one or more named legs are missing, stale, ambiguous, or not independently discriminating;
- **superseded** — another claim ID owns the current assertion and the old record remains historical evidence.

The word “verified” is reserved for the first state. A syntactically complete record does not prove its assertion;
it makes the evidence re-runnable and reviewable.

### Registry execution and identity

Commands are stored as argument arrays with one declared tracked producer and an explicit input list. The checker
never evaluates a shell string. For every `verified` record it executes the re-derivation and falsification
commands, requires the declared exit/output markers, authenticates every producer/input/evidence path through Git,
and compares exact SHA-256 identities. The stale check must cover the complete artifact list. Any dependency move
therefore fails until the owner re-runs the evidence and refreshes or deliberately downgrades the record.

The JSONL control record bounds total records, bytes, record bytes, arrays, and scalars below checker-compiled
portable hard caps. Unknown fields fail closed. The report also inventories the unique governed producers and
rejects ignored or untracked producer-shaped files below `scripts/`, `doctrine/`, `docs/`, or `.github/`. Run the
focused contract with:

```sh
perl scripts/check_claim_verification.pl --self-test
perl scripts/check_claim_verification.pl --check
perl scripts/check_claim_verification.pl --report
```

## 5. Stochastic assertions

A stochastic result publishes an interval or distribution summary, not an unsupported point estimate. Its
re-derivation leg records the seed policy, repetition count, sampling/aggregation method, and relevant environment
identity. Its falsification leg must distinguish the claimed effect from within-arm variance or another named
competitor. If the repetitions cannot support an interval, the evidence gap is explicit.

## 6. Auditor asymmetry

When a new re-derivation disagrees with a published claim, neither value wins automatically. The newer instrument
has usually run fewer times and may itself be wrong. Record the disagreement as “one of these results is wrong,”
exercise the falsification leg, and supersede or repair only after the competing explanations are separated.

## 7. Author workflow

1. Decide whether the slice publishes or changes an in-scope current assertion.
2. If not, use `Published-claims: none` in the commit and review description.
3. If yes, assign a stable lowercase claim ID and state the assertion precisely.
4. Run the re-derivation from its canonical source and retain bounded evidence.
5. Name a competing hypothesis and exercise a dimensionally different control, including a known-bad RED case.
6. Prove every producer/control is tracked and bind every artifact dependency to a stale-state gate.
7. Publish the ID and status; name missing legs instead of implying signoff.

The task-tree acceptance checklist still proves the implementation slice. It does not substitute for this
per-claim evidence. Conversely, claim verification does not replace task ownership, regression testing, or the
doctrine driver.

## 8. Reviewer workflow

1. Require exactly one `Published-claims:` declaration.
2. Challenge `none` when the diff changes a current count, score, capacity, compatibility statement, or status.
3. For each ID, reproduce leg 1 and verify its exact boundary.
4. Ask what competing hypothesis leg 2 separates; reject a second copy of the primary check.
5. Confirm the control has a tracked RED case rather than an always-green name.
6. Confirm the cited case's exact source region and use the report's ignored/untracked census; then perturb
   identity to confirm the stale gate fails.
7. Treat missing legs as explicit incompleteness, not a reason to invent evidence.

## 9. Anti-patterns

- “Checked twice” by the same producer, classifier, source interpretation, or arithmetic identity.
- A control that has never been observed failing on a known-bad input.
- A hash that proves inputs are unchanged but says nothing about downstream logic correctness.
- A current constant copied into prose with only a comment telling future authors to refresh it.
- A producer or oracle that exists only under ignored/scratch/generated state.
- An artifact identity that omits the script or binary that produced the artifact.
- A point estimate for a stochastic quantity without repetition and uncertainty.
- Treating a fresh disagreement as proof that the older value is wrong.
- Marking a claim verified because its registry record parses.
- Evidence equally consistent with the published account and its competitor, cited as falsification.
- A set assertion — “nothing checks X”, “this is the only caller” — published without the command that enumerated it.
- A raw search hit-count published as a defect count without classifying the population.
- An assertion about an item evidenced by data about that item's container.
- A classifier or candidate vocabulary authored from a description of a producer rather than derived from the producer.
- A published cause attributed by reading a diff instead of by revert-and-re-apply or per-revision re-derivation.
- A repository-derived constant carried in prose because a trajectory shows it has not moved.
- A finding published without checking whether the same shape was already adjudicated in this project's history.

## 10. Repository routes

- Architecture decision: `docs/decisions/0042-actionable-published-claims-require-three-dimensionally-different-legs.md`
- Owning adoption tree: `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md`
- Author/commit workflow: `COMMIT.md`
- Diagnostic and acceptance guidance: `TOOLBOX.md`
- Pull-request review contract: `.github/PULL_REQUEST_TEMPLATE.md`
- Doctrine driver: `scripts/check_doctrines.sh`
- Bounded registry: `doctrine/claim_verification/claims.jsonl`
- Mechanical checker: `scripts/check_claim_verification.pl`

All durable paths in claim evidence are relative to the repository root. Generated or external material is an
input only through an explicit tracked authority and lifecycle; it is never silently promoted to canonical
project state.

## 11. Upstream provenance and deliberate non-adoption

This standard is a **restatement** of a portable upstream source, not a copy, so a byte diff proves nothing about
currency and the gap has to be read section by section. Directive 17 requires that the source be re-checked for
updates after adoption; this section records the last such reading so the next one starts from a boundary instead
of from scratch.

- Source: `/Volumes/SSD/Documents/github/pgen/docs/CLAIM_VERIFICATION.md` — same volume, read-only, no copy taken.
- Read `2026-08-30`; source mtime `2026-08-26`, SHA-256
  `3ac26c365ed6b0c9c4f714fec8c952379ed02c5ac61bd4edcd5a60553de0f85c`.
- Previous reading: `CLAIM-VERIFICATION-ADOPTION.1` (`2026-08-15`). The source did not change between the two
  readings; the gap closed here was present from the original adoption, not introduced by an upstream revision.

**Adopted at this reading.** Each rule below was absent from every governed claim surface — `CLAIM_VERIFICATION.md`,
`TOOLBOX.md`, `COMMIT.md`, `DOCTRINE_ENFORCEMENT.md`, `AGENTS.md`, the pull-request template, ADRs 0042/0044, and the
mdBook enforcement chapter — before this revision:

| upstream rule | local home |
| --- | --- |
| state every rule domain-free; an example is an instance | preface |
| the taxonomy of what each check class still permits | §2 |
| a check and the thing it checks must not share a parent | §2 |
| match evidence granularity to claim granularity | §3 Leg 1 |
| prefer one derived source to *N* synchronized copies | §3 Leg 1 |
| a repository-derived constant is derived or gated, never carried | §3 Leg 1 |
| attribute a RED check by revert-and-re-apply, not by reading | §3 Leg 1 |
| evidence consistent with both hypotheses illustrates, not tests | §3 Leg 2 |
| a set claim carries its enumeration, in both directions | §3 Leg 2 |
| prefer an oracle you did not build; the cheapest is project history | §3 Leg 2 |
| derive classifiers and membership tests from the producer | §3 Leg 2 |
| a right unwatched number replacing a wrong one is not a fix | §3 Leg 3 |

One refinement is worth recording rather than silently fixing: **the general form was already in this repository,
demoted.** ADR 0042's Context paragraph states that repeated checking fails “when the checks share a parent”, and
the standard then carried only the three instances that form generates. A rule living as rationale in a decision
record while its examples live in the normative text is exactly the defect the preface names — so the promotion
above is the correction, and the ADR keeps the sentence as the reasoning that produced it.

**Deliberately not adopted**, with the reason, so a future reading does not re-open each one:

| upstream material | why it stays upstream |
| --- | --- |
| §1's four-row table and every “measured in the reference deployment” figure | dated measurements of another project; reusing them as current SpecForge evidence would create exactly the claim §1 governs. The same rules are taught here from local recorded instances: `CLAIM-VERIFICATION-ADOPTION` `.6`, `.6a`, `.6b`, `.7`, `.9`. |
| the five-portable-architecture summary table | already carried by `README.md` and `DOCTRINE_ENFORCEMENT.md`; a third copy is the *N*-synchronized-copies defect Leg 1 forbids. |
| §5A's inline provenance-tag syntax (a bracketed re-derive/falsify/durable triple in prose) | superseded by the executable registry plus the compact `[claim: <id>]` annotation. The upstream tag is prose a reader must trust; the local form resolves through a gate that executes the commands. |
| §7 adoption checklist items 1–5 | executed by `CLAIM-VERIFICATION-ADOPTION.0`–`.5`. The standing obligations they created live in §7 and §8; keeping the checklist would read as pending work that is already closed. |

The local §4 registry-execution contract, §7/§8 author and reviewer workflows, and §10 routes have no upstream
counterpart and are retained. Upstream checklist item 6 — re-read the standard substituting this project's own
domain for every example, and rewrite any rule that cannot be restated in local terms — is the procedure that
produced this section, and is the procedure the next reading should repeat.
