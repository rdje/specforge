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
| `CHAIN-CURRENCY` | oracle | every proof-current artifact under `generated/` is exactly what today's binary reproduces from verified upstream authority; a legacy/proofless compatibility refusal is reported as an explicit unmeasurable frontier, while a stale current proof still fails — and retained normalized bundles match their declaration exactly |

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

Among its focused suites, the live-document gate runs 81 common lifecycle/control cases, 47 neutral derived-state
classification cases, 25 SpecForge authority-adapter cases, and 15 terminal-task source/route/identity/boundary
cases, plus 44 active-task source/topology/route/payload/bound/writer cases, 58 fact-catalog source/plan/route/residue/
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
the exact 168-rule structural alpha join, and 17 behavioral-contract/evidence faults covering population or
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
makes the box un-self-tickable. The full template, and the catalog of SpecForge's diagnostic tools that
produce the cited evidence, live in `TOOLBOX.md` at the repo root.

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

ADR 0042 and `CLAIM_VERIFICATION.md` freeze that author/reviewer contract. The active self-bounded JSONL registry
stores unique statuses, argv-form commands, complete tracked artifact membership, exact SHA-256 identities,
refresh ownership, and retained evidence. The gate executes every verified source/control command, rejects digest
drift or incomplete stale-check coverage, and resolves the prepared commit message—or otherwise `HEAD`—against
known non-superseded IDs. Its 22-case fixture matrix observes valid honest statuses plus missing, unknown, duplicate, stale, untracked,
unsafe, false-RED, supersession, and bound controls go RED. This proves provenance is current and rerunnable; it
does not make a wrong assertion true merely because its record parses. Current-constant migration, the broader
tracked-producer/RED-control audit, and final worked examples remain in the next adoption leaves.

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

The frozen result contains 51 exact evidence units: 11 derived, four identity-gated, four registered, five
incomplete, and 27 excluded. The incomplete frontier is deliberately narrow: README maintained references,
mdBook quantitative claims, workflow/doctrine baselines, knowledge-card references, and FSMGen issue-packet
references. This is a classification result, not a repair; those five source surfaces remain unchanged until the
separately owned repair slice. `[claim: current-claim-census-frozen]` Reproduce the result with:

```sh
perl scripts/check_current_claim_census.pl --self-test
perl scripts/check_current_claim_census.pl --check
perl scripts/check_current_claim_census.pl --report
perl scripts/check_current_claim_census.pl --produce
```

Only the separately owned repair slice may change one of the five incomplete source assertions. Any other
surface or any changed exact region makes the frozen census fail closed first.

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

# ask the chain-currency oracle directly, or prove it is fail-closed first:
bash scripts/check_chain_currency.sh
bash scripts/check_chain_currency.sh --self-test

# run structural genericity plus the frozen behavioral design contract directly:
bash scripts/check_production_genericity.sh

# run the book's Rust examples and then build the complete HTML book:
bash scripts/run_docs_ci.sh

# the full gate, including the heavy oracles (run before committing Rust code):
bash scripts/run_ci.sh
```

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
