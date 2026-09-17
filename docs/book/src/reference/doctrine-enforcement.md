# Doctrine Enforcement

SpecForge makes some strong promises about itself: *no code change lands without an owning task-tree
leaf first*, *every score is objectively measured*, *extraction stays PDF-agnostic with no hardcoded
chip names*, *the docs never drift from the code*. The natural question a careful reader asks is: **how
do you know those promises are actually kept, and not just written down somewhere and quietly broken?**

This page answers that. The short version: in SpecForge a written rule (a "doctrine") is paired with a
small program that **re-checks the rule from the repository itself** and fails loudly if it is broken —
and that program runs automatically every time someone commits. A rule nobody checks is, in practice, a
suggestion. A rule with a check that blocks the commit is enforced. This is the **fourth portable
architecture** the project carries, alongside the task-trees, the memory architecture, and the
knowledge map. Claim verification is the fifth: it decides what evidence earns a current assertion before a
gate preserves it. The full enforcement standard lives in `DOCTRINE_ENFORCEMENT.md`; the claim standard lives in
`CLAIM_VERIFICATION.md`; this chapter is the friendly tour.

## Why this exists

Two failure modes quietly erode any project's stated discipline:

- **"Trust me" compliance** — a change claims it followed the rule, but nothing proves it.
- **Silent drift** — a rule decays one small exception at a time because nothing re-checks it.

Neither is malicious; both are human. The cure is not to write the rule more emphatically — it is to
make the **compliant path the same path the gate lets through**. So each doctrine gets a check that
re-derives the truth from the files, and the project's git hooks and CI run that check. If the check
passes, the rule held; if it fails, the commit is blocked with a message that says exactly what broke
and how to fix it.

For you as a user, the payoff is concrete: when SpecForge tells you a wire-protocol gold is at `1.000`
or that `kg-bench` is `156/156`, those are not numbers someone typed into a doc — they are oracles the
gate can re-run, so a regression cannot be quietly waved through.

## The core idea, in one line

> **doctrine = a rule + a deterministic check that exits nonzero on any breach.**

Once a rule has such a check, enforcing it is mechanical: one **driver** runs every registered check and
reports a per-doctrine PASS/FAIL; the **git hook** runs the driver locally; **CI** runs the *same*
driver server-side. The prose explains *why* the rule exists; the check decides *whether* it holds.

## The three kinds of checks

Every enforceable doctrine fits one of three shapes, chosen by what makes the proof real:

| Kind | The check… | Why you can trust it | SpecForge examples |
|---|---|---|---|
| **Structural** | re-derives an invariant from the files | it is a fact about the tree — it cannot be faked | the resume pointer (`MEMORY.md`) stays bounded; the derived Knowledge Map is regenerated and in sync |
| **Oracle (re-run)** | re-executes a deterministic tool at fixed inputs and asserts the result | a fabricated claim does not reproduce | `kg-bench` is `156/156`; the WIRE-BASED-100 golds are `1.000`; `cargo fmt`/`clippy`/`test` are clean |
| **Evidence (artifact)** | requires a re-checkable artifact for something that cannot be re-derived | strong when the cited command is re-run | a code change's task leaf carries tool-backed *why+where* + a measured *before→after* |

Structural checks are the strongest because they cannot be gamed; oracle checks are next because a re-run
beats trust; evidence checks are used only where the thing being enforced is a *process* that leaves no
other re-derivable trace.

## What is enforced today

| Doctrine | Kind | Proves |
|---|---|---|
| `MEMORY-ARCH` | structural | the durable 4-layer memory architecture invariants — the standard is present, `MEMORY.md` is a bounded resume pointer, the bootstrap files route to it, the task-tree and decision layers exist |
| `KNOWLEDGE-MAP` | structural | the bounded landing and exact question-shard membership/content regenerate from fact cards without drift |
| `TASK-ACCEPTANCE` | evidence | a Rust code change is owned by a task-tree leaf whose acceptance checklist is ticked **and** backed by real SpecForge tool output |
| `README-POLICY` | structural | the project landing page stays within its locally derived line and byte ceilings, and every reader or author-overflow route closes at a registered, controlled terminal |
| `LIVE-DOC-SIZE` | structural | every tracked Markdown path is classified exactly once, and every declared current-state field satisfies its explicit class, marker, authority, accessor/capture, and executed-verifier contract alongside the lifecycle/pressure/route rules |
| `PROJECT-DATA-LOCALITY` | structural | Cargo, shell, production temporary-workspace, subprocess, and Python runtime seams keep owned data below the current repository and reject stale or escaping roots |
| `PRODUCTION-GENERICITY` | structural | package direction, exact production inventories and rule joins, compiled raw/identity information flow, protected authority, and proof-only persistence all hold without a named-specification exception |
| `CORPUS-FRONTIER` | derive-and-diff | the SourceIR-derived corpus cohort is partitioned exactly into explicit refreshed and remaining sets, retained bundles agree, and the task file states the same counts — source-library relocation cannot impersonate a refresh |
| `CLAIM-VERIFICATION` | structural + oracle | bounded claim records, tracked digest-current artifacts, executed source/known-bad-control commands, complete stale-check coverage, and publication IDs resolve together |
| `PUBLISHED-ASSERTIONS` | deterministic-oracle | every published value in a governed prose region re-derives against its named producer field, because the producer is executed and its report compared — the one thing a digest binding cannot do, since a digest proves a region has not changed and never that its numbers still re-derive. A value may instead be gated by a control with a known-bad case, authored by a decision, or anchored to a revision; there is no outcome for a value carried because it has not moved lately. Two surfaces stating different values for one field fail without running anything, a set is compared as an enumeration rather than a size, and a record whose enumeration could include its own publishing surface must declare that exclusion |
| `RESIDUAL-ACTIONABILITY` | derive-and-diff | the frozen required-residual contract still describes the artifact it claims to describe: its witness pins the tracked reviewed result by digest, its cell decomposition and published ratio re-derive from that result instead of being restated, and every typed residual cause resolves to a real carrier in production source or an honest `null`. It was added after the contract sat red for 43 commits — a later leaf legitimately republished the reviewed result and nothing required the frozen contract to be re-examined, which is precisely the drift an executable contract exists to prevent |
| `PROOF-SEAL-CURRENCY` | oracle | every persisted artifact under `generated/` records a proof seal today's build still accepts — censused across every stage of the proof-carrying stratum, and probed through the product's own loader by running the consuming stage in dry-run form rather than by validating, which would mutate what it reads. Every artifact is probed at `semantic` and `intent`; the two stages whose probes replay extraction are sampled one per distinct seal, and the check says so |
| `PROOF-SEAL-TOTAL` | oracle | the same census with every in-scope artifact probed individually at every non-terminal stage — the only way to see a divergence between documents that share a seal, and CI-tier because a per-document sweep is minutes rather than seconds |
| `CHAIN-CURRENCY` | oracle | every proof-current artifact under `generated/` is exactly what today's binary reproduces from verified upstream authority; a legacy/proofless compatibility refusal is reported as an explicit unmeasurable frontier, while a stale current proof still fails — and retained normalized bundles match their declaration exactly |

`PUBLISHED-ASSERTIONS` exists because of a defect this manual is itself a past instance of. SpecForge publishes
counts about its own state — census units, adjudicated regions, registered doctrines — and those counts went
stale ten separate times while every gate stayed green, twice inside the very commit that published them. The
reason is structural rather than careless: the controls that watched those sentences were digests, and a digest
proves that a region's bytes have not changed, never that the number inside it still re-derives. Five rounds of
prose correction each went stale in turn, which is what finally settled the design.

So the gate executes the producer. A record names the governed region by path, line range and content hash, the
literal value published there, and a field in a producer's JSON report; the checker runs that producer and
compares. Three escape hatches exist and each must name its own evidence — a `gated` value points at the control
whose failure would follow the value moving, plus an exact known-bad region inside that control; an `authored`
value points at the decision record that fixes it; a `dated` value points at the revision it is anchored to. The
fourth possibility, a value carried because a trajectory shows it has held, is deliberately not expressible: a
trajectory shows what has not happened, never what cannot, and this repository has the commit that proved it.

Two further legs cost almost nothing and catch what the first misses. Two records naming one producer field with
different values fail immediately, with no producer run at all — that is a real recorded case where two surfaces
published different numbers for one quantity and nothing noticed. And a set claim is compared as an
**enumeration**, never as a size, because a membership list can drift while its count holds. A record whose
enumeration could include the very surface that publishes it must declare that exclusion, which is the subtlest
lesson here: a screen that the act of writing the finding turns green is not a check at all.

The grammar that finds the numbers is part of the contract, not an implementation detail. A gate derives its own
population, so a value its scanner cannot see is worse than one it reports unlisted: no record can be asked for
it, and a frozen registry will still call the surface complete. That is not hypothetical — the first version
excluded every hyphen and slash after a numeral, which correctly keeps dates and identifiers such as
`2026-08-28`, `segment-0013` and `SHA-256` out, and also made every count closing a compound adjective
(`27-case`, `56-unit`) or a ratio (`15/15`) invisible. Nineteen published values were affected, one of them a
live self-test count inside a paragraph this gate was written to watch. The rule is now the shape those
identifiers actually take: only `-` followed by a digit is excluded, and a comma joins a numeral only when it
separates exactly three digits, so `1,922` stays one value while `40, noticed` publishes `40`.

Which surfaces are watched is decided the same way — by discovery, with omission treated as an error. The gate
scans every tracked Markdown file for a claim tag and asks the live-document surface registry which surface
owns it; a file whose surface has declared no disposition, or that no surface owns at all, fails the run.
Authors write down what a *surface* is for, never which files belong to it, so a new annotated file joins the
watched set by itself. Exemptions exist and must say why: sealed archive segments and dated engineering
evidence are captures whose currentness is not asserted. And each exemption is proven to be doing work — the
test suite flips an exempt surface to governed and requires the same value to become fatal, because a quiet
green run is otherwise equally consistent with the file never having been found.

The map is now closed. Every published value in a governed region resolves to a record with one of the four
outcomes, so a numeral nobody accounted for fails the build rather than being reported and forgotten. Closing
it took two prose repairs rather than two more records, and both are worth knowing about because the same
shapes recur. A measurement window that names where it started but not where it ended cannot be anchored to
anything — it silently grows — so it now names its closing revision. And a live count whose producer publishes
it only as human-readable output fits no outcome at all: it cannot be compared to a report field and nothing
fails when it moves, so the numeral was removed and the reader runs the command instead. Deleting a value is a
legitimate repair; what the standard forbids is swapping one unwatched number for another.

`CORPUS-FRONTIER` exists because of a defect worth stating plainly. The corpus refresh program tracked its own
remaining work as prose that each slice decremented by one, across twenty-two consecutive refreshes — and a
decrement cannot detect an error at its base, so a document stopped being counted and nobody could see it. The
lesson generalises past this project: re-deriving the rows of a list never validates that the list is complete,
because anything already missing from it is invisible to every check written over it. The doctrine therefore
derives the whole cohort independently and requires every member to occur exactly once in a declared lifecycle
partition. Location is deliberately orthogonal: repairing or relocating a source path does not complete a
current-binary refresh. Retained bundles remain an independent witness and must agree with the refreshed set.

The README guard is unconditional: it evaluates the resulting tree even when a change does not touch
`README.md`. Its data-only route registry distinguishes links readers follow from destinations authors
may use for changing detail, and rejects missing, off-repository, duplicated, or uncontrolled routes.
The project-owned normative contract is `README_POLICY.md`.

Among its focused suites, the live-document gate runs 93 common lifecycle/control cases, 47 neutral derived-state
classification cases, 25 SpecForge authority-adapter cases, and 15 terminal-task source/route/identity/boundary
cases, plus 44 active-task source/topology/route/payload/bound/writer cases, 60 fact-catalog source/plan/route/residue/
bound cases, and the projection-specific Knowledge Map contract and portable-bundle integration suite. Generated
collections must have complete landing membership, exact derive-and-diff content,
bounded repository-local check workspaces, and no stale parts or temporary residue. Exact current fields are
declared rather than guessed: derive-on-read values cannot retain a shadow, verified copies execute their named
authority check, authored intent remains human-owned, and immutable evidence keeps an exact capture boundary.

The active-task contract binds source-locked Git/file identity, exhaustive semantic regions, planned-part
pressure, fixed portable caps, and destination absence. A task-specific part may preserve an exact wide legacy
row only at or below the existing 6,400-byte direct task-evidence cap. Complete inputs bind completion-subject ids as
`legacy` routes and formal container ids absent from those subjects as `structural` routes; both require an exact
full or tree-relative source literal in one primary semantic payload. Its migrated state verifies those routes,
capsule and marker payloads, current root/frontier, index, manifest, per-part/aggregate pressure, and sealed Git
identities. If the bounded root declares an executable owner registry, complete primary routes must equal that
registry exactly, including structural and pending owners that have no completion commit. Its guarded
materializer preflights all inputs, preserves raw legacy bytes beneath
UTF-8 scaffolds, writes the stable root last, validates the final tree, and rolls back only its proven-owned
destinations. The positive continuation fixture also proves that a new active part, eligible frontier,
post-migration route, root, index, manifest, and contract update form one accepted bounded transaction.
The driver invokes the PDF, corpus, and alignment contracts independently. Every non-default contract must
publish the exact repository-relative `--contract … --check` command rendered into its index; a command that
falls back to the PDF authority is rejected.

The fact-card contract authenticates its legacy commit/blob/index, raw digest/metrics, 158-card sources and row
union, and former destination absence. Its current migrated state enforces the fixed-size landing router,
deterministic 56-card title parts, semantic tuples and resolved card destinations, exact output
membership/content, fixed capacity and mandatory-rollover bounds, residue cleanup, and repository-local writes.
The landing carries one range row per title part — part number, card count, and inclusive first/last id — and is
rejected if it links a card directly, misorders or drops a row, miscounts a part, or names a boundary id the card
list does not confirm. The 336-card maximum is cross-checked against the 338-file canonical surface, and the
current 393-fact authority is derived from that maximum plus the decision-record file ceiling rather than
pinned as a literal. The 3,584-question-key authority is separately derived as eight keys per fact rounded to
the 512-key registry quantum, so neither fact writer nor the question projection can drift alone. The title-part generated-projection registry record
was forbidden before migration and is exact now; the canonical card surface declares `routed_membership` through
that record, so the generic gate proves every card through exactly one hop.

The project-data gate also runs three focused shell cases. It checks the tracked Cargo environment,
the common shell initializer, Rust production temp/subprocess boundaries, exact Python environment
locks, and any present venv's interpreter/launcher roots. A caller does not need to remember a
`TMPDIR`; the gate rejects missing roots and cache symlinks that escape the repository.

The production-genericity gate is one unconditional composition, not a forbidden-word scan. Its baseline runs
the package dependency/disconnection check, exact module/claim inventory, exact rule/field/producer/seam/bypass
joins, compiled fixed-point information flow, and the frozen behavioral design contract. The graph follows the
complete production Cargo surface and rejects raw or identity-dependent semantic control outside registered
universal grammar/declassification, unregistered canonical mutation, protected capability or proof forgery, and
proofless persistence. The behavioral contract joins all 24 current rows to portable source/normalized-text
identity and every proof-bearing stage, freezes six relations plus the 7-calibration/17-prospective split, and
keeps full-PDF capture separate from the lossy text projection. Missing source/provider and vacuous behavior are
unmeasurable rather than passed. Its checker contains no document/vendor/protocol labels; those remain data.

CI adds `--self-test`: 27 controlled dependency/schema/rule/flow faults, legal display/provenance/test-only uses,
the exact 170-rule structural alpha join, and 17 behavioral-contract/evidence faults covering population or
attempt omission, unsafe authority, hash/tool drift, reviewed-label leakage, split overlap, state laundering,
interval drift, absolute paths, vacuity laundering, and partial relation/stage assertions.
This gates the behavioral oracle's currency. The conformance harness now implements and calibrates full-PDF
unchanged/adversarial identity plus normalized-text alpha, reviewed paraphrase/layout, and semantic-negative pairs across all five
stages. The contract checker also verifies the recipe manifest, source/recipe hashes, review ownership, exact
single-occurrence spans, relation-specific change kinds, safe per-span fields, identifier projection, pinned
conclusions, complete complement, semantic/proof delta declarations, nine-class sensitivity matrix, typed attempt
dispositions, and rich-capture exclusions. A negative passes only when ordinary invariance rejects, its exact
declared delta is present, and the remaining five-stage complement is exact. These focused executions are not the
governed population result. The first held-out execution remains in Git history because it exposed a filename-
derived comparator omission and an overbroad alpha authority. The corrected aggregate closes 34 pass / one fail /
16 unmeasurable / zero invalid: both 17-document PDF relations pass, while the sole eligible six-signal I2C alpha
pair fails below SourceIR and opens `.f.iii.a`. Complete-population replay remains separate signoff work.

It also runs the roadmap current/history contract. The structural check authenticates the exact
1,487-line source capsule, five exhaustive source regions, all 23 workstream ids and owning task
routes, known drift evidence, readers, bounded-current shape, and archive topology. It verifies the
153-line current root plus manifest/index retrieval. Nine focused cases reject record/row loss,
duplication, reordering, chronology leakage, and unsafe paths.

The FSMGen-feedback contract is independently executable. It pins the exact 936-line source capsule,
five exhaustive regions, six closed directed exchanges and their response/resolution evidence, two
stale-current findings, 26 consumers, bounded open-record fields, and archive topology. Ten focused
cases reject source/evidence/heading/path drift and malformed migrated markers/register/manifest state.
The migrated gate authenticates the exact capsule and enforces the 81-line current channel, zero-open
boundary, six-row closed register, and direct archive retrieval.

The validation-snapshot currency check is read-only and review-aware. It authenticates the tracked
last-reviewed snapshot and live-status managed block against four declared report identities and 29
recommendations, pins the exact producer regions that refresh/render/write those surfaces, and verifies
the task evidence that withheld later unreviewed local scores. Ten focused cases reject identity,
report, count, live-block, producer, evidence, path, duplicate, or schema drift without touching
git-ignored artifacts.

The source-PDF registry has a separate read-only membership oracle. The Git index supplies the exact
PDF denominator below `corpus/`; every registry row must map one member to its real `SourceIR`-derived
key and parent directory, and every file must carry the PDF signature. Two pinned key-derivation
functions plus three build seams make code/registry drift fail closed. Thirteen focused cases exercise
identity, membership, duplicate, key, directory, signature, producer, path, and schema failures without
consulting the owner's host-local library or generated artifacts.

The live-document checker is also unconditional. Its JSONL registry is the data-only authority for
the complete tracked Markdown set, while `LIVE_DOCUMENT_SIZE_CONTAINMENT.md` explains the neutral
lifecycle model. It does not confuse a large legacy ceiling with a healthy target: existing debt keeps
an immutable measured baseline and a separately bounded transition allowance. Unique product prose in
the mdBook uses a maintained-reference contract—bounded directly indexed parts plus exact task-owned
aggregate change—instead of a fixed cap that would silently limit legitimate product documentation.
Its current-truth verifier also requires the completed containment/locality status and closing
implementation/verification subsection while rejecting the earlier migration/debt wording.
The adapter also runs 55 repository-volume positive and fail-closed fixtures, including every local
lifecycle, registry schema/size controls, independent pressure axis, and Git-history authority path.

The chain-currency check answers a question the other gates cannot: *are the artifacts already sitting in
`generated/` still the artifacts this code would produce?* A repair normally rebuilds only the documents
it measured, so every other completed document silently absorbs one code delta at a time. The check
replays each stage `--dry-run` from the **persisted** artifact one stage upstream — evidence, semantic,
intent, the `.isf` adapter, and each emitted `.isf` against the adapter's own rendered text — and fails
when what comes back is not what is stored. It deliberately ignores the `validation_reports` section,
because `specforge validate` writes that *after* the stage has run and the product's own fingerprint
helpers exclude it too. What it cannot measure it says out loud: only the evidence stage needs a
document's normalized markdown bundle, so a document whose bundle was reclaimed is reported as
*unmeasurable* at that one stage rather than quietly counted as fine, while every later stage stays
measurable for the whole corpus. With no `generated/` at all — a fresh clone, a hosted runner — it skips
loudly instead of pretending to have checked something.

Which documents *are* measurable is not left to chance either. Because an ingest keeps its normalized
bundle, the retained set is declared in `doctrine/chain_currency/retained_bundles.json`, and the check
compares that declaration with the bundles actually on disk. A declared bundle that has vanished is an
unauthorized reclamation; a bundle no leaf recorded is a refresh that never declared what it kept. Both
fail closed, so the checkable population can only grow deliberately, one refresh at a time, and a
reclamation must name the leaf that decided it. The declaration is schema-closed — an unknown, mistyped,
unsorted, or self-contradictory field is itself a breach — so it cannot decay into prose.

### When a seal goes stale, and how the corpus gets it back

A stage artifact records the `ruleset_sha256` of the rule set that proved it, and every stage below
records the *cumulative* digest of its upstream's rule set folded with its own. Those digests are built
from the production implementation each rule depends on, so an ordinary edit to a stage's production
module re-seals nothing and invalidates everything below it: the persisted corpus starts failing to load
with `proof ledger ruleset hash is stale`, and one stage further down with `cumulative proof ledger
ruleset hash is stale`.

This is a statement about the *seal*, not about the artifacts. The captured premises are untouched, and
the content is not in question until something measures it. Two remedies exist, and they are not
interchangeable:

- **SourceIR has a proof-only path.** `cargo run --example source_proof_migrate -- --write` re-derives
  each artifact's ledger from its own retained capture. It rewrites the proof and nothing else, so
  public content cannot move.
- **Every stage below it has no such path.** EvidenceIR, SemanticIR, IntentIR, and the adapter cannot
  be re-sealed in place — the only way to give them a current ledger is to re-run the stage, which
  writes real artifact content. That is a heavier act, and `scripts/rebuild_stage_cascade.sh` exists so
  it is a measured one.

The cascade rebuilds `evidence` → `semantic` → `intent` → `isf-adapter` across the proof-carrying
documents, in that order, and proves two things per stage rather than assuming them: that each rebuilt
artifact is content-identical to the one it replaced, and that it validates through the product's own
loader afterward — which is what re-earns the seal and restores the `validation_reports`
back-annotation the rebuild necessarily dropped. Content identity is decided by the *same* predicate the
chain-currency oracle uses; both scripts share one definition
(`scripts/lib/stage_artifact_identity.sh`) so a remedy can never certify itself with a comparison the
gate would not make. A legacy proofless document has no current seal to restore and is reported as an
explicit out-of-scope count, never silently skipped.

A content delta is never absorbed. If a rebuilt artifact differs, the run names the differing top-level
sections, stops at that stage so nothing is rebuilt on top of an unexplained change, and keeps its
pre-write snapshot for attribution. That caution is not theoretical: the ADR 0025 reconciliation found
exactly one real content delta across the whole checkable population. The cascade is a remedy, not an
oracle — after it runs, `scripts/check_chain_currency.sh` is still the only thing that may call the
corpus current.

### Seeing the seal break on the commit that breaks it

Both remedies above answer *how to get the seal back*. The harder question was *when you find out it is
gone*. Chain currency reports a stale seal loudly and refuses to be bypassed, but it is a CI-tier
doctrine, so it first speaks at the push boundary — and a corpus once sat out of seal for weeks and
dozens of commits before anything said so. Nothing was wrong with the artifacts; nothing an ordinary
slice ran could observe the transition.

`scripts/check_proof_seal_currency.sh` closes that specific gap at gate tier, on every commit. It reads
the seal every persisted artifact records, at every stage, for every document in the proof-carrying
stratum — a total census — and then asks today's build whether it still accepts that seal.

For a long time the check asked about **one document per distinct seal**, and argued that this was not a
sample because the census established representativeness. That argument was wrong, and the corpus proved
it. A change to an already-registered EvidenceIR field's *content* leaves the seal exactly where it is —
the seal is a digest over the ruleset — while the loader also verifies a per-document replay topology,
which the content does move. Every one of the 27 artifacts carried the same seal, so one probe ran and
reported green; the loader was in fact refusing four of them, and those four were every wire-bearing
specification in the corpus. The scoring oracle refused every gold for three commits underneath a green
gate. It is the failure `CLAIM_VERIFICATION.md` tabulates as *a per-item assertion checked against
per-container data*: it reproduces perfectly while getting it wrong.

So the check now says which scope it is running, and the scope is decided **per stage** rather than per
tier. Loading a persisted artifact and verifying its proof is cheap; replaying extraction from a
normalized markdown bundle is not, and those are the two kinds of probe the chain contains. Since
`2026-09-14` the gate tier therefore probes **every** document at `semantic` and at `intent`, and keeps
sampling one per distinct seal at `source-ir` and `evidence` — where it still prints that it is blind.
`--total` probes every document at every non-terminal stage and is registered as its own CI-tier
doctrine. On this corpus the fully sampled run is four probes, the gate-tier run is 56, and the total
run is 108. The script's own self-test pins each half with a loader that accepts one document and
refuses its same-seal neighbour: at a sampled stage the check must miss it, at a totally-probed stage it
must name it, and — the case that was missing until the default was turned on — the *shipped default*
must name it with no environment override in play, so quietly emptying the stage set goes red.

Turning that on cost the gate a little over a minute, from 4m13s to 5m30s, and the case for paying it is the
three commits above: a sample of one in twenty-seven had already passed over every wire-bearing
specification in the corpus while the scoring oracle read them.

That total run took about nineteen minutes until the same day, and it now takes **just under two**. The
difference is not a change to what it proves — it is the cargo profile the check builds. Three commands
replay the persisted corpus against the current build: this seal check, the `CHAIN-CURRENCY` content
oracle, and the rebuild remedy that clears what they report. Each had its own copy of "build the binary
and use `target/debug/specforge`", which is one predicate written three times — the same hazard that
put the seal reader and the content comparison into shared files. They now share
`scripts/lib/corpus_replay_binary.sh`, and it selects the **release** profile.

The reason is worth reading, because the obvious argument for debug is a good one that stopped being
true. The question these checks ask is whether *this commit's* build accepts the artifact, so the build
is part of the check, and a debug build is the cheapest way to obtain one. That holds only while the
build dominates. A build is a fixed cost; a probe is a cost per document, and the probe is a
deserialize-and-verify over artifacts that reach 84 MB — precisely what an unoptimized build is worst
at. One probe over a 39.7 MB artifact costs 63 seconds at debug and 6.5 at release; over a 16 KB one it
costs nothing at either. So the argument was right when a run was four probes and wrong when it became
a hundred and eight, and the crossover is about two large documents.

The verdict does not move, and that was measured rather than assumed: proof verification is digest
comparison and ordered-map lookup, both profiles accept the same 27 of 27 artifacts, and a 43 MB
`intent --dry-run` is byte-identical between them at the same SHA-256. The objections to release were
measured too, and none survived — a cold release build of this workspace is 36 seconds rather than
minutes, the release tree is the *smaller* one, and neither profile is ever built on a clean checkout or
a hosted CI runner at all, because every one of the three commands skips on an absent corpus before it
reaches the build. `CHAIN-CURRENCY` fell from 28 minutes to 12m38s in the same change, so the two
CI-tier corpus doctrines together went from about 47 minutes to under 15.

Four properties of that check are worth stating, because each is a mistake it would have been easy to
make:

- **It asks the product's own loader, and asks it read-only.** `specforge validate` is the obvious way to
  ask, and it is the wrong one: validation back-annotates the artifact, so each call moves its digest,
  and every stage below retains its upstream's ledger as an exact prefix. A gate that validated the
  corpus on every commit would invalidate the chain beneath it on every commit. The read-only way to
  reach the same verified-load path is to run the *consuming* stage with `--dry-run`, which leaves the
  artifact byte-identical.
- **It reports the stage it cannot probe.** The adapter is terminal: it has no consumer, so no read-only
  canonical probe exists for it at all, and chain currency does not close that gap either because its
  content comparison excludes the proof surface by construction. That stage is censused and reported as
  unprobed rather than quietly counted as a pass.
- **A probe whose own input is missing yields no verdict, not a pass.** Three documents' normalized
  bundles are deliberately held out of the corpus, so the probe for them cannot run at all. The total
  mode names each one and counts it separately from the accepted set. Counting an unanswerable question
  as an answer is the same error in a smaller place.
- **It is narrower than chain currency, deliberately.** A current seal says the persisted proof is one
  today's build issues. It says nothing about whether the artifact is still the content today's binary
  would reproduce. That remains chain currency's question, and it remains CI-tier.

A tree with no `generated/` — a fresh clone, or a hosted CI runner — skips loudly and passes, because the
doctrine has nothing to govern there and silence would read as a verdict.

The heavy deterministic oracles — `kg-bench`, the WIRE-BASED-100 golds, the byte-identical
evidence/`.isf` checks, chain currency, the full `cargo` suite, and the mdBook doctest/build pair — are the
strongest leg of all. They are too slow to run on every local commit, so they run in the full CI gate
(`scripts/run_ci.sh`) rather than in the fast pre-commit hook. That is a deliberate split, stated openly
and now visible in the tooling: heavy doctrines stay in the same registry marked *CI-tier*, and the fast
default run prints each one as `DEFER` so a deferred rule can never read as an absent one. The local hook
catches the cheap structural and evidence breaches instantly; the un-fakeable re-run happens in CI.

## The acceptance checklist (for code changes)

SpecForge's most-emphasized rule is *no code change without an owning task-tree leaf first*. That is now
mechanized. When a change touches the Rust extraction/lowering code, the gate requires the owning task
leaf to carry a short checklist, each required box ticked and backed by the cited tool output:

- **ROOT CAUSE (why + where)** — a SpecForge tool located and explained the cause: a `validate` finding
  or metric, an `adapt --target isf` blocking reason, a `kg-bench` fixture diagnostic, a failing
  `cargo test`, a `--dry-run` measurement, a `file:line`.
- **ADDRESSED (verified)** — the change does what it should, measured per item (a before→after count, a
  recovered surface).
- **NO REGRESSION** — backed by a named, re-runnable oracle: `kg-bench 156/156`, WIRE-BASED-100 `1.000`,
  byte-identical golds, `run_ci.sh` green.

A box you tick is a *claim*; the proof is the **oracle re-run** in CI. A self-ticked-but-false
"NO REGRESSION" passes the local presence check and then dies when the oracle is re-run — which is what
makes the box un-self-tickable. The full template and the first-reach tools live in `TOOLBOX.md` at the
repo root; the deeper catalog sections that produce the cited evidence are routed from there into
`docs/toolbox/`.

Changes that do **not** touch Rust code — documentation, scripts, the book itself — are exempt from this
particular gate (they carry their own), so ordinary continuity work is never false-blocked.

## Claim verification comes before enforcement

A green gate can preserve the wrong number perfectly if the number was never independently earned. Every commit
and review description therefore declares either `Published-claims: none` or the stable IDs of current actionable
assertions it publishes or changes. `none` is about claim scope, not file type: a docs-only score change still
needs an ID, while a normative refactor may legitimately publish no empirical claim.

Each governed claim names three different legs: a command/accessor that re-derives it from canonical source; a
falsification oracle that separates a concrete competing hypothesis and has been observed RED on a known-bad
case; and tracked producers plus complete artifact identity and a stale-state gate. A repeated copy of the first
check is not the second leg. Missing evidence remains an explicit `incomplete` status rather than being converted
into “verified.”

### What a check still permits

A green check is evidence only about the defect class it is able to fail on, so the first question a reviewer asks
of any control is what it still permits. A conservation total permits any redistribution between buckets. A row
count permits a wrong value in every row. A digest over a governed prose region permits every number inside that
region going stale while the text stands still — which is exactly how a published count survives a fully green
gate. Tests written from the same document as the implementation permit every misreading of that document. The
general form is that a check and the thing it checks must not share a parent: when a control and the thing it
controls descend from the same understanding, their agreement carries no information.

That general form was already in this repository, as reasoning inside ADR 0042, while the standard carried only
the instances it generates. `CLAIM-VERIFICATION-ADOPTION.10` promoted it to normative text alongside the rest of
the upstream material a section-by-section re-reading found to have no local home.

### Illustration is not falsification

A falsification leg fails silently when its evidence is equally consistent with the account being published and
the account it was supposed to rule out. Predicting the observation is not the test; producing an observation the
competitor would not have produced is. This binds a claimed **mechanism** — an account of why a value moved — every
bit as much as it binds the value, which is the correction `CLAIM-VERIFICATION-ADOPTION.6b` had to make to `.6a`
after an unmeasured causal story was published in place of a measured one.

Three cheap disciplines follow, and each closes a defect this repository has actually made. Consult the project's
own adjudicated history before publishing a finding: the owning task tree and the commit body that introduced a
value are the cheapest oracle available, and neither costs a measurement. Derive a classifier or candidate
vocabulary from the producer that emits the thing, never from a description of that producer, or the control ends
up green precisely where it is blind. Attribute a red check by revert-and-re-apply, or by re-deriving the value
from each revision's own producer input, rather than by reading a diff and inferring a cause.

A claim about a set also carries its enumeration, in both directions. “Nothing checks this” is refuted by a single
counterexample, so the command that enumerated the checkers belongs beside the assertion; and a search hit count is
a population rather than a count of defects until that population has been classified.

ADR 0042 and `CLAIM_VERIFICATION.md` freeze that author/reviewer contract. The active self-bounded JSONL registry
stores unique statuses, argv-form commands, complete tracked artifact membership, exact SHA-256 identities,
refresh ownership, and retained evidence. The gate executes every verified source/control command, rejects digest
drift or incomplete stale-check coverage, and resolves the prepared commit message—or otherwise `HEAD`—against
known non-superseded IDs. Its twenty-seven-case fixture matrix observes valid honest statuses plus missing,
unknown, duplicate, stale, untracked, unsafe, false-RED, exact-RED-evidence, supersession, and bound controls go
RED. This proves provenance is current and rerunnable; it does not make a wrong assertion true merely because
its record parses.

The producer/RED closure makes the falsification evidence concrete. Each of the seven cited controls binds a
named perturbation and expected diagnostic to an exact region in its tracked producer; the gate executes the
control and makes region drift RED. The independent catalog-feasibility command now proves its own controlled
sub-ceiling failure before publishing the live result. Its report derives seven controls, seven exact RED regions,
and six unique governed producers, with zero ignored or untracked producer-shaped candidates under the governed
source roots. The expanded twenty-seven-case matrix makes missing, stale, or misdirected RED evidence and both
scratch-producer classes fail. `[claim: claim-provenance-gate-active]`

### A concrete author and reviewer walk-through

Start with classification, not JSON. “The workflow-standard collection has its authored title” is a policy or
identity statement; “a dated run observed a particular result” remains historical when it keeps that boundary;
and a schema version is an identity token. None becomes a current measured claim merely because it contains a
number. By contrast, “the current workflow-standard collection fits the selected capacity profile” can guide a
repository decision and can drift, so it resolves to `workflow-standard-capacity-profile`.

An author can reproduce that real SpecForge example without copying its result from this page:

```sh
perl scripts/measure_workflow_standard_capacity.pl --check
perl scripts/check_claim_verification.pl --probe workflow-standard-capacity-profile
perl scripts/check_claim_verification.pl --check
```

The first command derives the current profile from tracked membership and history. The probe computes the
catalog independently and proves a controlled sub-ceiling case goes RED. The final command authenticates the
tracked inputs, exact known-bad region, stale-state membership, and publication IDs. A commit that changes this
current assertion then declares:

```text
Published-claims: workflow-standard-capacity-profile
```

If an author has only the new measurement, the honest result is not a half-filled verified record. It is an
incomplete claim whose absent legs are machine-readable. This illustrative row is valid shape, but it is not a
claim about the live repository and should not be added to the registry without an owning task:

```json
{"record_type":"claim","schema_version":1,"claim_id":"example-throughput-observation","status":"incomplete","assertion":"A bounded trial observed a throughput change.","owner":"example-owner","missing_legs":["falsification","durability"]}
```

Stochastic observations need more than a point estimate. Retain the seed policy, repetition count, aggregation
method, environment and input identity, and an interval or distribution summary. The competing control must
separate the claimed effect from within-arm variance or another named explanation. If the available repetitions
cannot support that interval, keep the claim incomplete; a precise-looking point value does not repair the gap.

Review is deliberately asymmetric. The author must make every leg succeed, while an auditor only needs one
credible break. If a new capacity script disagrees with the registered result, the reviewer does not promote the
newer value automatically. They rerun the source derivation, exercise the strict-boundary and catalog controls,
check the exact RED region and producer census, and perturb one watched identity. The disagreement remains “one
instrument or interpretation is wrong” until those competing explanations are separated.

A closing review can replay the complete current boundaries directly:

```sh
perl scripts/check_claim_verification.pl --self-test
perl scripts/check_claim_verification.pl --report
perl scripts/check_current_claim_census.pl --report
perl scripts/check_book_quantitative_claims.pl --report
bash scripts/check_doctrines.sh
```

The reports answer different questions: provenance shape and current execution, silent current-claim closure,
manual-wide quantitative adjudication, and repository doctrine composition. A green provenance report never
promotes the book's explicitly incomplete assertion regions or turns a malformed interpretation into truth.

### The current-claim census freezes evidence before repair

The migration sweep is deliberately staged. Its bounded contract derives eligible surfaces from the live-
document registry, assigns every current surface either to one or more of five semantic views or to an explicit
standard-scope exclusion, and binds eventual findings to tracked paths plus exact line-range hashes. Derived and
identity-gated outcomes require a directly executed argv-form verifier; registered outcomes join the current
claim registry; incomplete and excluded outcomes must name what is missing or why the assertion is out of scope.

The initial implementation boundary was `inventory`, where evidence records were forbidden rather than silently
treated as findings. Before the result could freeze, a controlled first pass caught that multi-view surfaces
emitted only their first-view anchor. Candidate identity now includes the semantic view, and frozen validation
requires evidence for every included surface and every required view.

At its `.3b.4` boundary the repaired result contained 56 exact evidence units: 11 derived, seven
identity-gated, six registered, zero incomplete, and 32 excluded. That is a dated boundary, not a standing
partition. `CLAIM-VERIFICATION-ADOPTION.6a` measured the totals across 28 consecutive revisions and found that
ordinary, unrelated work moves them: a rolling-ledger rollover carries claim-annotated regions out of the live
window, and any commit that adds or drops a `[claim: <id>]` annotation moves the registered and closure counts.
So the unit total, the excluded and registered counts, and candidate closure are read from `--report` rather
than carried here. `CLAIM-VERIFICATION-ADOPTION.11` then withdrew the rest of the vector for a stronger reason
than movement: a value that has merely held is not a constant, only an unmoved one, so a count is published here
only when a control fails if it moves or an authored decision fixes it. Two qualify. Zero unresolved candidates
is gated — the checker raises an error for any produced candidate lacking exact evidence or a current registered
annotation — and the five semantic views are authored by the census design. `authority_outcomes.derived` and
`authority_outcomes.identity_gated` were carried here until one commit registered a single new surface and moved
both, and the absence of an `incomplete` outcome is the same class; all three are read from `--report`. The five
old broad anchors for README maintained references, mdBook quantitative claims, workflow/doctrine baselines,
knowledge-card references, and FSMGen issue-packet references have narrow identity or registered authority; the
other units retain their original outcomes. `[claim: current-claim-census-frozen]` Reproduce the result with:

```sh
perl scripts/check_current_claim_census.pl --self-test
perl scripts/check_current_claim_census.pl --check
perl scripts/check_current_claim_census.pl --report
perl scripts/check_current_claim_census.pl --produce
```

The closing audit also reverses the join: every produced candidate must have exact frozen evidence or carry a
current registered claim annotation. Its dated boundaries were 79 = 51 + 28 at `.3c`, 84 = 51 + 33 at `.4`, and
86 = 51 + 35 + 0 unresolved at `.5`; the current triple is whatever `--report` prints, because each of those
numbers moved without the census producer changing. What does not move is the closure property itself:
unresolved candidates must be zero.
The twenty-eight-case suite instantiates every outcome family and drives family-specific plus
surface/view/path/region/source/identity coverage faults RED.

The census also checks that it can still **mirror** the plane it censuses. It derives its denominator from
`doctrine/live_document_size/surfaces.jsonl`, refuses any disagreement, and requires one disposition per
current surface plus at least one frozen evidence unit per included one — so the two registries are joined by
an identity while their capacities were sized independently, and nothing compared them. Two statements, two
outcomes: today's population plus one measured partition event must fit, or the run fails, because a census
that cannot hold the next ordinary commit is a refusal nobody saw coming; and the run warns, printing its own
arithmetic, when the plane's *reachable* ceiling would put the census in its warning band. Reachable is the
word that matters. The surface registry's record bound counts archive and frozen surfaces too, and those only
accumulate — their files are immutable and a tracked Markdown no surface classifies is refused, so a record
cannot be reclaimed. Comparing the two bounds directly warns about a fully-current plane the registry cannot
hold; subtracting the archive floor first is what makes the line true. The per-surface cost is measured from
the census on every run, and the rolling-ledger head rows are excluded because a rollover reclaims them.

The repair changed only the five frozen frontier keys and added one exact authority unit for each. Any unrelated
surface or changed exact region still makes the frozen census fail closed.

Repair was split by authority rather than by filename count. README, knowledge-card, and FSMGen issue-packet
anchors first receive exact route/catalog membership evidence, which proves navigation identity but deliberately
does not certify member prose. The workflow anchor separates authored policy from the already registered
capacity assertion. Quantitative mdBook prose receives a bounded exact-region contract; the selective book-
currentness check is not promoted into blanket evidence for the whole manual. The closing leaf binds those
authorities atomically, and the assertion-level book regions that remain explicitly unverified are counted by
`check_book_quantitative_claims.pl --report` under `authority_outcomes.incomplete` — a total the manual moves
whenever it changes, including the commit you are reading.

The maintained-reference audit found no missing route control. README policy, the routed fact-card catalog, and
the canonical collection catalog each derive membership from tracked authority and each observes malformed or
drifted fixtures go RED. The repair therefore reuses those producers at their exact navigation boundary; it does
not add a parallel manifest and does not interpret a green route as semantic certification of linked prose.

Workflow doctrine uses a different split. The collection title is authored normative identity, while the
actionable capacity paragraph resolves to the existing `workflow-standard-capacity-profile` claim and its
derivation, strict-boundary controls, and independent catalog-feasibility probe. Reusing that one authority
avoids turning the pull-request template into numeric evidence or maintaining another copy of the result.
`[claim: workflow-standard-capacity-profile]`

The manual-wide quantitative repair starts from prose, not every digit. After fenced examples are excluded, a
bounded lexical grammar emits percentages, fractions, and numeric dimensional units as review candidates. Every
candidate must be covered exactly once by a tracked exact region, then semantic authority—not syntax—classifies
it as current derived/identity-gated/registered/incomplete evidence or a named authored, example/identity, or
dated-evidence exclusion. Inventory, adjudication, and result freeze are separate transactions so the grammar
cannot be weakened in response to its own findings.

The inventory is executable and self-bounded:

```bash
perl scripts/check_book_quantitative_claims.pl --self-test
perl scripts/check_book_quantitative_claims.pl --check
perl scripts/check_book_quantitative_claims.pl --report
perl scripts/check_book_quantitative_claims.pl --produce
```

The live contract reports `book_files=39`, `candidate_lines=307`, and `candidate_files=21`. Inventory established
that review denominator without semantic regions; frozen phase now binds every emitted path/line/SHA candidate
exactly once to a derived or identity verifier, a current claim, honest missing legs, or one closed exclusion
reason. The self-test exercises nineteen positive and controlled-
negative cases, and all fixtures are created and removed on the repository volume.

## How the gates are layered

Defense in depth, the same four-layer model the memory architecture uses. Each layer catches what the
last misses:

1. **Discovery** — the doctrine is named in the entrypoint docs (`README.md`, `TOOLBOX.md`,
   `docs/decisions/`, this book) and every harness bootstrap file, so it is unmissable.
2. **Self-check** — each `check_*.sh` is the single source of truth for one rule.
3. **Git hook** — `.githooks/pre-commit` runs the driver; a non-compliant tree cannot commit locally.
4. **CI** — the same driver runs server-side, where it cannot be bypassed from a clone.

Honest limits, stated rather than hidden: a local hook can be skipped (`--no-verify`), so CI is the real
backstop — and SpecForge's hosted CI is currently manual-only to conserve build minutes, which means the
un-fakeable oracle re-run happens at the next CI/`run_ci.sh` run, not the instant you commit. The
strongest guarantee is restored by re-enabling an automatic CI gate.

## Running and extending it yourself

```bash
# run the fast gate the pre-commit hook uses (CI-tier doctrines are listed as DEFER):
bash scripts/check_doctrines.sh

# run every registered doctrine, CI-tier ones included:
bash scripts/check_doctrines.sh --all

# list the registered doctrine ids, or run only the ones you name:
bash scripts/check_doctrines.sh --list
bash scripts/check_doctrines.sh --only LIVE-DOC-SIZE,README-POLICY

# the early signal before ANY commit: the gate tier minus the measured costliest four:
bash scripts/check_doctrines.sh --fast

# ask the chain-currency oracle directly, or prove it is fail-closed first:
bash scripts/check_chain_currency.sh
bash scripts/check_chain_currency.sh --self-test

# report the persisted corpus's proof-seal state, then restore a stale one by rebuilding the chain:
bash scripts/rebuild_stage_cascade.sh --check
bash scripts/rebuild_stage_cascade.sh --write
bash scripts/rebuild_stage_cascade.sh --self-test

# run structural genericity plus the frozen behavioral design contract directly:
bash scripts/check_production_genericity.sh

# run the book's Rust examples and then build the complete HTML book:
bash scripts/run_docs_ci.sh

# the full gate, including the heavy oracles (run before committing Rust code):
bash scripts/run_ci.sh
```

### Selecting less than the whole gate, without pretending otherwise

The pre-commit hook runs the complete driver, so a full manual run before committing means every slice
pays the gate twice. Two selectors exist so the manual pass can be cheaper without becoming dishonest.

The rule that falls out is stronger than "prefer the subset": **never run the full driver by hand at
all.** Write `G` for the cost of one gate. A manual full run costs `G + G` when it passes, against `G`
for letting the hook be the only full run; when it fails, both cost `G + fix + G`. The manual run is
therefore never cheaper and is usually twice the price, and that holds whatever the slice touched — a
Rust change no more than a documentation one. What a Rust change needs instead is the signal the
doctrine gate never provides, because the gate neither compiles nor tests: `cargo fmt`, `cargo clippy`
and the library test suite, plus `--only PRODUCTION-GENERICITY` when the producer graph could move.

`--only ID[,ID...]` runs the doctrines you name, and `--list` prints the ids. Use it instead of invoking
a `check_*.sh` by hand: the driver runs each enforcer exactly as the hook does, with no arguments, and
asserts the exit status itself, so no caller has to know a per-script flag. An id that names no
registered doctrine is refused rather than quietly selecting nothing.

`--fast` runs the gate tier minus the four doctrines measured costliest — `LIVE-DOC-SIZE`,
`PROJECT-DATA-LOCALITY`, `PROOF-SEAL-CURRENCY`, `PRODUCTION-GENERICITY`. The subset is declared in the
driver as an *exclusion*, so a newly registered doctrine belongs to the fast set automatically and
dropping one costs a deliberate edit; the list is meta-checked against the registry on every run, so
renaming a doctrine cannot leave a dangling exclusion behind.

Three properties keep a subset from reading as a gate. Every unselected doctrine is reported `SKIP`
rather than omitted. A subset run prints a banner a full run never prints, and `--fast` also prints the
doctrines it did not run, so the caveat travels with any line copied into a task record. And `--fast`
refuses outright when the pre-commit hook is not active, because a subset is only safe while something
else pays for the rest — in an unhooked clone it would be the only enforcement that ever ran, and it
would report success.

A green `--fast` is an early signal, not a verdict. Of the five doctrines that blocked a commit during
the session this was measured in, `--fast` runs three and omits two — and the omission is not theoretical:
on the very slice that added `--fast`, `--fast` went green and the hook's full run then failed
`LIVE-DOC-SIZE`, one of the four it skips. Name the doctrine your slice can actually break and add it
with `--only`; that is cheaper than the whole gate and sharper than hoping the subset covers it.

The driver prints a per-doctrine report and exits nonzero if any check fails. Adding a new enforced
doctrine is intentionally a two-step move: write a `scripts/check_<id>.sh` that obeys the check-script
contract (exit nonzero on breach, deterministic, reads the repo and mutates nothing, scope-aware), then
add one line to the driver's registry naming its tier. The driver meta-checks that every registered
check — deferred ones included — actually exists and is executable, so a registry entry can never become
a dangling promise.

## How this was verified

The adoption was landed as the task-tree `DOCTRINE-ENFORCEMENT-ADOPT` (`.0` framework, `.1` the native
task-acceptance check + this toolbox, `.2` this chapter). The driver was run standalone before it was
wired into the hook, so the live commit gate was never pointed at an unverified driver; the
task-acceptance check was exercised across all five of its paths (exempt, block-when-no-leaf,
pass-with-evidence, block-when-unticked, block-when-unbacked) with throwaway staged files that were fully
reverted, before it was allowed to gate a real commit. No extraction or emitter Rust was touched, so the
WIRE-BASED-100 golds and `kg-bench` (156/156) are orthogonal by construction.

## Quantitative adjudication result

The authority freezes an exact region set whose totals move whenever the manual itself changes — including
the commit you are reading, which edited this chapter — so they are derived on read rather than printed here.
`perl scripts/check_book_quantitative_claims.pl --report` gives the current `regions` and the
`registered` / `incomplete` / `excluded` split, and the frozen contract's declared expectations live in
`doctrine/claim_verification/book_quantitative_claims.jsonl` (`expected_book_files`,
`expected_candidate_files`, `expected_candidate_lines`). Exclusions stay separated by scope reason — authored
threshold or choice, example or command literal, schema/date/path/digest identity, and dated boundary evidence
— and those labels are exact-region scope, not a claim that the rest of a section shares the same meaning.
Registered regions are limited to the verified workflow-capacity profile. Every other current actionable assertion keeps re-derivation, independent
falsification, and durability visibly missing until a later repair earns those legs.
`[claim: mdbook-quantitative-census-frozen]`

The first full replay also found and fixed a bound-dimension defect: record capacity belongs to `max_records`,
while `max_array_items` applies only to arrays inside one record. The controlled suite now includes a contract
whose record count exceeds its nested-array limit but remains inside the independent record limit.

This claim verifies the mapping and its honest uncertainty, not the truth of an incomplete line. Reproduce the
boundary with the checker commands above; `--self-test` challenges the failure classes before `--check` executes
the tracked exact-region result.
