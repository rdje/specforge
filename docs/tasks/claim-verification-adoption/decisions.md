# CLAIM-VERIFICATION-ADOPTION — decisions

- Part ID: `decisions`
- State: `legacy`

<!-- claim-verification-task-source-region:decisions:start -->
## Decisions

- `2026-08-30` (`.7.2.1a`): **`HEAD` is not an anchor.** A published range ending at `HEAD` names a different
  window every time it is read, so it is the open-ended-window defect regardless of how the sentence is
  phrased. The repair is to drop the count and name the command, not to re-measure it — a re-measured count
  is stale again on the next commit.
- `2026-08-30` (`.7.2.1a`): a count harvested from a repair script's own run counter answers "how many times
  did the loop fire", not "how many rows did this commit change". Publish the per-commit measure with the
  command that re-derives it; the two are not interchangeable even when they are close.
- `2026-08-30` (`.7.2.1a`): a share measured on one ledger against one budget may not be restated across two
  ledgers against a different budget. `changes` is 66.9% of its health target and `live-achievement-status`
  is 59.2%; "two thirds of each" was one number wearing two denominators.
- `2026-08-30` (`.7.2.1a`): when a gate is described as closing a class, **name the population it closes it
  over**. `.7`'s control watches two governed files; the eleventh instance landed on a surface none of the
  three producers reaches.
- `2026-08-30` (`.7.2.1`): a sentence that no closed outcome fits is **repaired, not recorded**. An
  open-ended measurement window is anchored to the revision that closes it, and a live count whose producer
  publishes it only as prose is removed so the reader runs the command. Removing a value is not the
  right-unwatched-number anti-pattern: §3 Leg 3 forbids swapping one unwatched value for another, and nothing
  is stale about a number a surface no longer publishes.
- `2026-08-30` (`.7.2.1`): `dated` is the weakest of the four outcomes — the checker proves only that the
  revision resolves — so a surface holding 26 of them is publishing history, not current state, and the leaf
  says so rather than letting the count of `verified`-looking records imply otherwise.
- `2026-08-30` (`.7.2.0`): the governed scope is **fail-closed**, not listed. Every tracked Markdown file
  carrying a claim tag is discovered, resolved to its live-document surface, and refused if that surface
  declares no disposition or if no surface owns the file. Only a surface's disposition is authored; its
  membership never is. A glob list fails open in the one direction that matters — a file nobody listed is a
  file nobody checks, and nothing says so.
- `2026-08-30` (`.7.2.0`): an exemption must state its reason, and the exemption must be proven load-bearing
  by flipping the surface to governed and observing the same value become fatal. A green run with an exempt
  value unreported is otherwise equally consistent with that file never having been discovered.
- `2026-08-30` (`.7.2.0`): lifecycle cannot decide this scope — `TOOLBOX.md` and the task tree share
  `partitioned_canonical` — and the census's own disposition marks `task_evidence` included, which would put
  85 dated verification-log values in scope. Both are recorded as tried and rejected for a stated reason,
  so the next reader does not reach for them again.
- `2026-08-30` (`.7.1a`): the coverage grammar decides the population, so a value it cannot see is a silent
  hole rather than an unlisted one — no record can be asked for it and `frozen` phase will still call the
  surface complete. A grammar change is therefore a population change, and it must be measured against an
  independent tokenizer and bound to RED cases before the population is closed, never adjusted by reading.
- `2026-08-30` (`.7.1a`): a numeral closing a compound adjective (`27-case`) or a ratio (`15/15`) is a
  published quantity; only `-` followed by a **digit** is an identifier or date fragment. The exclusion is
  narrowed to the form those identifiers actually take instead of excluding the whole punctuation class,
  which is how 19 published values across four claim-annotated files became invisible to the gate.
- `2026-08-30` (`.7.1a`): a comma joins a numeral only when it separates exactly three digits. Absorbing
  sentence punctuation into a value binds the gate to prose rather than to a quantity, and it makes the
  honest record — the one listing the number a reader would act on — report as unlisted.
- `2026-08-30` (`.7.0`): a value published on a governed surface must carry one of exactly four outcomes —
  `derived` (re-executed and compared against a named producer field), `gated` (a named control fails if it
  moves, and that control has a known-bad case), `authored` (a named decision fixes it), or `dated` (anchored to
  a revision). There is deliberately no outcome for a value carried because a trajectory shows it has held.
- `2026-08-30` (`.7.0`): the governed population is derived on every run and may never be stored. Three
  consecutive commits moved their own published population by describing it, so a stored list is guaranteed to
  be wrong at the moment it lands.
- `2026-08-30` (`.7.0`): stop correcting a class that prose cannot close. Five rounds of correction were each
  invalidated by their own transaction. The correcting leaf is retired in favour of the gate; a finding of this
  class is now evidence for `.7`, not a new prose repair.
- `2026-08-30` (`.11a`): an ownership or coverage classifier must be one the finding cannot satisfy by existing.
  A `grep` over `docs/tasks/` for a surface id turns green the moment a leaf names the surface in order to report
  that nothing owns it, and it scores `done` trees as owners. Screens are still useful for *finding* candidates;
  they may not be published as the answer. The publishable form is an explicit reviewed assignment naming an open
  leaf per warned item.
- `2026-08-30` (`.11a`): when a census is published about a doctrine, the denominator is the **doctrine's own
  driver**, not whichever checker was convenient. `check_live_document_size.pl` is a component of `LIVE-DOC-SIZE`;
  quoting its warning set as the gate's is the container-for-item substitution §3 Leg 1 forbids.
- `2026-08-30` (`.11a`): a commit body is a hypothesis, not evidence. §3 Leg 2 makes project history the cheapest
  *oracle* — something to test a finding against — and `.11` quoted `d23e8bae`'s body as a source instead, which
  is how "two publishers" was published as three.
- `2026-08-30` (`.11`): a count published on a governed surface is legitimate only when a **control fails if it
  moves** or an **authored decision fixes it**. "A trajectory shows it has held" is neither, and is retired as a
  reason. Where neither applies, the remedy is withdrawal plus the producer's field name — never substitution of
  today's value, which leaves the next drift exactly as unobservable as the last.
- `2026-08-30` (`.11`): a sweep names its population with a command, not with recall. `.6`, `.6a`, and `.6b` each
  swept surfaces they remembered and each said honestly how far it went; the defect is not the honesty but that no
  step derived the denominator. The enumerating command for this class is a grep for the producer paths over
  tracked non-archive Markdown, and it belongs in the leaf beside the finding.
- `2026-08-15`: claim verification is additive to task trees, memory, retrieval, and doctrine enforcement. It
  defines what earns a published claim; doctrine enforcement makes that definition mechanically difficult to
  bypass.
- `2026-08-15`: the adoption governs current-facing claims someone may act on. Dated historical measurements stay
  immutable unless disproven; their current reuse creates a new claim and must satisfy the three legs.
- `2026-08-15`: enforcement will validate provenance and staleness structure, not pretend a syntactically valid tag
  proves the underlying measurement.
- `2026-08-15`: `.3` will census all 39 nonhistorical governed Markdown surfaces, but will register claims—not
  raw numerals. Version strings, schema ids, dates, example literals, exact hashes, authored priorities, and
  historical captures keep their existing authority class; a current quantitative assertion someone may act on
  is in scope.
- `2026-08-15`: `.2` will use a dedicated self-bounded JSONL registry under `doctrine/claim_verification/` and a
  focused checker registered through the existing doctrine driver. The 14-field derived-state plane remains an
  input/authority source rather than being overloaded with falsification semantics.
- `2026-08-15`: at `.0`, no pull-request or review template existed. `.1` adds the repository-standard GitHub
  pull-request template and aligns `COMMIT.md` / `TOOLBOX.md`; authors state either governed claim tags or that
  the slice publishes no actionable claim.
- `2026-08-15` (ADR 0042): reserve `verified` for re-derivation, dimensionally different falsification with an
  observed RED control, and tracked stale-detecting durability. Govern current actionable assertions rather than
  numeral syntax; current reuse of dated history creates a new claim.
- `2026-08-15`: the exact author/reviewer declaration is `Published-claims: none` or comma-separated stable claim
  IDs. `none` is scope-based, not a docs/code exemption; syntactic mechanical enforcement begins only in `.2`.
- `2026-08-15`: the required root standard plus PR template raise canonical workflow standards from 12/16 to
  14/16 files (87.5%). `.1a` owns a measured repeatable remedy; `.1` does not hide the warning or widen a bound.
- `2026-08-15` (ADR 0043): retain explicit stable membership and minimally re-derive 21 slots from 14 current
  members plus the measured four-member peak. Per-file limits stay fixed; aggregate reachability moves with count.
- `2026-08-15` (`.2` design): use strict self-bounded JSONL claim records, argv-form commands, exact tracked
  artifact digests, executed source/control probes, and exact message ID resolution. Do not execute a shell string
  or duplicate hook/CI wiring; the existing doctrine driver is the sole composition seam.
- `2026-08-15` (ADR 0044): verified records join executed re-derivation/RED-control commands to the complete
  tracked SHA-256 artifact set; incomplete/superseded records preserve uncertainty, and registry validity never
  substitutes for semantic truth.
- `2026-08-15`: decompose `.3` into denominator/gap freeze (`.3a`), measured repair (`.3b`), and independent
  clean-boundary closure (`.3c`). A census design and its repairs must not share one unreviewable transaction.
- `2026-08-15` (`.3a.0`): derive the denominator from the surface registry: 55 total minus 15 archive terminals
  minus one frozen legacy surface = 39 current surfaces. Rolling-ledger live windows remain included; archive
  members and the frozen dated capture do not become current merely because they contain constants.
- `2026-08-15` (`.3a.0`): the census has five required category views—current status, roadmap/controller
  projections, maintained references, doctrine baselines, and mdBook quantitative claims. Views may refer to one
  governed surface for different semantic questions, but each evidence unit/claim key is unique and each of the
  39 surfaces has exactly one inclusion/exclusion disposition.
- `2026-08-15` (`.3a.0`): every evidence unit binds a tracked repository-relative path plus an exact region/
  assertion identity and one closed outcome: `derived`, `identity_gated`, `registered`, `incomplete`, or
  `excluded`. The first two require an executable verifier, `registered` requires a known current claim id,
  `incomplete` names missing legs, and `excluded` names a standard scope reason. Unknown fields/outcomes fail.
- `2026-08-15` (`.3a.0`): the JSONL control bounds records/bytes/record bytes/arrays/scalars below checker hard
  caps. `.3a.1` implements coverage and fault controls; `.3a.2` freezes results; neither may rewrite a source
  assertion. Repairs begin only in `.3b` from the frozen gap identities.
- `2026-08-15` (`.3a.1`): keep the real census contract in `inventory` phase with zero adjudicated evidence
  records. It derives rather than repeats lifecycle eligibility, gives all 39 current surfaces exactly one
  disposition, and emits deterministic review anchors plus known derived-state/registered-claim candidates.
- `2026-08-15` (`.3a.1`): exact evidence identity is a one-based line range plus SHA-256 of those source bytes.
  A frozen `derived`/`identity_gated` unit must execute a tracked argv-form producer/input join; `registered`
  resolves a non-superseded claim ID; `incomplete` and `excluded` carry their required explicit reason fields.
- `2026-08-15` (`.3a.2`): the first result-production pass exposed a structural gap: multi-view surfaces emitted
  only their first-view review anchor. The freeze therefore requires at least one evidence unit for every
  included surface **and** every required semantic view; the producer keys review anchors by surface + view +
  path + line so overlapping views remain independently adjudicable.
- `2026-08-15` (`.3a.2`): the final 51-unit authority census closes as 11 `derived`, four `identity_gated`, four
  `registered`, five `incomplete`, and 27 `excluded`. The incomplete units are the README maintained-reference
  surface, mdBook quantitative surface, workflow-doctrine baseline surface, knowledge-card reference surface,
  and FSMGen issue-packet reference surface; `.3b` may repair only those five exact keys.
- `2026-08-15` (`.3b.0`): a broad title anchor cannot earn authority for the prose below it. Route membership is
  repaired through the existing README/catalog producers, authored workflow identity remains out of claim scope
  while its actionable capacity paragraph joins the existing verified claim, and mdBook quantitative assertions
  receive their own exact-region contract instead of borrowing the selective book-currentness checker.
- `2026-08-15` (`.3b.1`): the three maintained-reference gaps need no new checker. README policy, routed fact-
  card membership, and canonical collection membership already derive their exact routes and each has a
  controlled missing/drift/bound class observed RED. `.3b.4` will bind those authorities without claiming that
  navigation identity verifies member prose.
- `2026-08-15` (`.3b.2`): workflow doctrine prose has two authority classes. The collection/title is authored
  normative identity; the actionable capacity paragraph already resolves to `workflow-standard-capacity-profile`
  with its tracked derivation, strict-boundary RED controls, and independent catalog-feasibility probe. No copied
  capacity value or second claim record is introduced.
- `2026-08-15` (`.3b.3.0`): the book quantitative repair uses a prose-only lexical candidate grammar as a
  completeness alarm, not a semantic classifier. The provisional shell census recorded 301 candidate lines
  across 21 files; exact region adjudication must cover each once, while code fences remain outside prose scope.
  The grammar and outcome schema freeze before `.3b.3.1` implementation.
- `2026-08-15` (`.3b.3.1`): the executable parser replayed the frozen grammar against both the `.3b.3.0` parent
  and current book and found 304 candidate lines across the same 21 files. The three-line delta is not book
  drift: the untracked provisional scan undercounted a stable input and left no reproducible artifact. The
  bounded contract now records the executable denominator, and future disagreement fails instead of silently
  changing scope.
- `2026-08-15` (`.3b.3.2`): exact single-line regions classify all 304 candidates: 75 current actionable lines
  remain honestly incomplete on all three claim legs, eight workflow-capacity lines join the verified current
  claim, and 221 lines are exact scope exclusions (26 authored thresholds/choices, eight examples, one schema/
  identity literal, and 186 dated boundary observations). A current-language competing scan reopened every
  excluded `current`/`live`/`today`/`now` hit and confirmed its dated section or named authored/example scope.
- `2026-08-15` (`.3b.3.2`): the first frozen replay found that the checker applied `max_array_items` to the JSONL
  record list rather than only arrays inside records. Inventory's four source records hid the dimensional error;
  the exact region set exposed it. Validation now leaves record capacity to `max_records`, applies array bounds
  per record, and a nineteenth controlled case prevents recurrence.
- `2026-08-15` (`.3b.3.3`): `mdbook-quantitative-census-frozen` verifies the census mapping, not the truth of its
  75 incomplete assertions. Re-derivation executes the current exact-coverage report; controlled mutations
  challenge coverage, regions, sources, outcomes, joins, and bounds; digest-complete durability watches the
  producer, frozen contract, source identities, lockstep publications, fact card, and retained task evidence.
  The Knowledge Map card makes that authority/uncertainty boundary retrievable before `.3b.4` consumes it.
- `2026-08-15` (`.3b.4`): close a broad incomplete title by narrowing its question, not by declaring the whole
  document verified. The five old anchors become explicit identity exclusions and gain five separate exact
  authorities: three executable routes plus the registered workflow-capacity and mdBook-mapping claims. The
  resulting 56-unit vector has zero outer incompletes while the 75 inner mdBook incompletes remain unchanged.
- `2026-08-15` (`.3c`): frozen evidence coverage and producer-candidate closure are independent invariants. Every
  produced anchor must have an exact evidence key or a current non-superseded registered annotation. The clean
  final lockstep boundary closes 79 candidates as 51 exact keys + 28 registered annotations + zero unresolved.
  The three-candidate increase from the pre-publication replay comes from the closure's own synchronized change,
  book, and task annotations; the
  27-case suite independently drives all five outcome families and every surface/view/path/identity join RED.

<!-- claim-verification-task-source-region:decisions:end -->
