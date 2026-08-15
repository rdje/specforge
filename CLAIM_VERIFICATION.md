# Claim Verification Standard

This is SpecForge's repository-owned standard for deciding whether a current assertion is earned strongly enough
to publish for someone else to act on. It is the fifth portable architecture, alongside task trees, durable
memory, the Knowledge Map, and doctrine enforcement.

> Re-derive · falsify · make durable. Repeating one kind of check does not create independent evidence.

The standard is normative for authoring and review. The bounded claim registry and doctrine check that will make
its provenance shape mechanical are owned by `CLAIM-VERIFICATION-ADOPTION.2`; until that leaf lands, a completed
claim declaration is required but must not be described as mechanically enforced.

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

## 2. The three legs

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

### Leg 2 — falsify with a dimensionally different oracle

State a competing hypothesis and name evidence capable of separating it from the published interpretation. The
oracle must not merely repeat the primary producer's classifier, arithmetic, or reading of the same prose.

A valid falsification leg identifies:

- the concrete competing hypothesis or defect class;
- the independent observation or controlled mutation that distinguishes it;
- a tracked known-bad case in which the cited control is observed going RED;
- the result that rules out, narrows, or exposes the competitor.

Conservation totals cannot falsify bucket misassignment. A source-derived row count cannot falsify a wrong value
in every row. Tests and implementation derived from the same interpretation do not independently validate that
interpretation. When no dimensionally different oracle exists, record the leg as missing.

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

## 3. Publishing contract

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

Each listed claim ID must resolve through the bounded registry introduced by
`CLAIM-VERIFICATION-ADOPTION.2`. Until that registry exists, the owning task leaf must state the claim, its three
legs, and any missing leg explicitly. Once the registry is active, inline current-facing prose uses the compact
tag `[claim: <claim-id>]` next to the assertion when a reviewer cannot otherwise map it unambiguously.

A claim record has one of these honest outcomes:

- **verified** — all three legs are present, current, and internally consistent;
- **incomplete** — one or more named legs are missing, stale, ambiguous, or not independently discriminating;
- **superseded** — another claim ID owns the current assertion and the old record remains historical evidence.

The word “verified” is reserved for the first state. A syntactically complete record does not prove its assertion;
it makes the evidence re-runnable and reviewable.

## 4. Stochastic assertions

A stochastic result publishes an interval or distribution summary, not an unsupported point estimate. Its
re-derivation leg records the seed policy, repetition count, sampling/aggregation method, and relevant environment
identity. Its falsification leg must distinguish the claimed effect from within-arm variance or another named
competitor. If the repetitions cannot support an interval, the evidence gap is explicit.

## 5. Auditor asymmetry

When a new re-derivation disagrees with a published claim, neither value wins automatically. The newer instrument
has usually run fewer times and may itself be wrong. Record the disagreement as “one of these results is wrong,”
exercise the falsification leg, and supersede or repair only after the competing explanations are separated.

## 6. Author workflow

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

## 7. Reviewer workflow

1. Require exactly one `Published-claims:` declaration.
2. Challenge `none` when the diff changes a current count, score, capacity, compatibility statement, or status.
3. For each ID, reproduce leg 1 and verify its exact boundary.
4. Ask what competing hypothesis leg 2 separates; reject a second copy of the primary check.
5. Confirm the control has a tracked RED case rather than an always-green name.
6. Use `git ls-files`/`git check-ignore` to verify producers and dependencies, then perturb identity to confirm the
   stale gate fails.
7. Treat missing legs as explicit incompleteness, not a reason to invent evidence.

## 8. Anti-patterns

- “Checked twice” by the same producer, classifier, source interpretation, or arithmetic identity.
- A control that has never been observed failing on a known-bad input.
- A hash that proves inputs are unchanged but says nothing about downstream logic correctness.
- A current constant copied into prose with only a comment telling future authors to refresh it.
- A producer or oracle that exists only under ignored/scratch/generated state.
- An artifact identity that omits the script or binary that produced the artifact.
- A point estimate for a stochastic quantity without repetition and uncertainty.
- Treating a fresh disagreement as proof that the older value is wrong.
- Marking a claim verified because its registry record parses.

## 9. Repository routes

- Architecture decision: `docs/decisions/0042-actionable-published-claims-require-three-dimensionally-different-legs.md`
- Owning adoption tree: `docs/tasks/CLAIM-VERIFICATION-ADOPTION.md`
- Author/commit workflow: `COMMIT.md`
- Diagnostic and acceptance guidance: `TOOLBOX.md`
- Pull-request review contract: `.github/PULL_REQUEST_TEMPLATE.md`
- Doctrine driver: `scripts/check_doctrines.sh`
- Planned bounded registry/checker: `doctrine/claim_verification/` and
  `scripts/check_claim_verification.pl` (`CLAIM-VERIFICATION-ADOPTION.2`)

All durable paths in claim evidence are relative to the repository root. Generated or external material is an
input only through an explicit tracked authority and lifecycle; it is never silently promoted to canonical
project state.
