# Doctrine Enforcement Architecture

A portable, **project-agnostic** standard for turning written rules ("doctrines") into
**mechanically enforced** ones — so compliance is *provable and re-checkable*, never a
"trust me" claim. Drop the kit (§8) into any repository and a non-compliant change cannot
land: a local git hook blocks it, and CI makes it un-mergeable.

> **👉 Adopting this in your project? THIS is the only document you need to follow.** Go straight to
> **§8 — The portable replay manifest**: copy the core files (Group A), adapt a handful of knobs
> (Group B), add your harness's bootstrap pointer (Group C), run the 3 setup commands. Sections 1–7
> are the rationale + the check-script contract; §9 is the honest limits; §10 is SpecForge's live
> registry.

> One-line thesis: **a doctrine that is not mechanically checked is not enforced — it is a
> suggestion.** The fix is to pair every doctrine with a deterministic check, run all checks
> from one registry/driver, and gate commits + CI on it.

This file is the **4th portable architecture** SpecForge adopts, alongside the three it already has:

| # | Portable architecture | Owns | Standard |
|---|---|---|---|
| 1 | **Task-trees** | per-unit work memory (goal/frontier/acceptance/verification) | `docs/TASK_TREE.md` |
| 2 | **Memory-architecture** | durable harness-agnostic agent memory (4 layers) | `MEMORY_ARCHITECTURE.md` |
| 3 | **Knowledge-map** | a retrieval layer over fact cards | `knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md` |
| 4 | **Doctrine-enforcement** | turning every rule into a mechanically-gated check | **this file** |

All four are **project- and harness-agnostic**: a project backed by Codex, Claude Code, Gemini, or a
human adopts each by replaying its standard. This one is the sibling of `MEMORY_ARCHITECTURE.md` —
that standard mechanizes the *memory* doctrine; this one generalizes the *same E1→E4
defense-in-depth* to **every** doctrine. The enforcement is **git-level** (hooks + CI), so it fires
identically no matter which harness made the commit.

---

## 0. How to use this file

1. Read it once. Adopt the **check-script contract** (§4) and the **driver+registry** (§5).
2. Copy the agnostic kit (§8): the driver, one example check, the hook, the CI step.
3. For each doctrine you want enforced, write a `check_<doctrine>.sh` and register it.
4. Run the three setup commands (§8). From then on, non-compliance fails fast (hook) and cannot
   merge (CI).

If you remember one rule: **route every doctrine to a check, register it, gate on the driver.**

---

## 1. The problem

Most doctrines live as prose (a README section, a decision record, a code comment). Prose is
**discoverable but not enforceable** — an agent or human can read it and still ignore it, and
nothing catches the violation until much later (or never). The two failure modes:

- **"Trust me" compliance** — a change claims it followed the rule; no artifact proves it.
- **Silent drift** — a rule erodes one exception at a time because nothing re-checks it.

The cure is not more prose. It is to make the **compliant path the gated path**: every doctrine
gets a check that *re-derives the truth from the repository*, and the gates run that check.

---

## 2. The core idea

> **doctrine = a rule + a deterministic check that exits nonzero on any breach.**

Once a doctrine has such a check, enforcement is mechanical:

- one **driver** runs every registered check and reports per-doctrine PASS/FAIL (§5);
- the **git hook** runs the driver (fast local gate, E3);
- **CI** runs the *same* driver (un-bypassable backstop, E4).

The check is the single source of truth for the rule; the prose doc explains *why*, the check
decides *whether*.

---

## 3. The three check archetypes (pick one per doctrine)

Every mechanizable doctrine fits one of three shapes. Pick by what makes the proof real.

| Archetype | The check… | Proof strength | Cost / where to run | SpecForge example |
|---|---|---|---|---|
| **Structural** | re-derives an invariant from the tree (allowlist match, file presence, lockstep/derived-artifact sync) | a fact about the files — cannot be faked | cheap → pre-commit | "`MEMORY.md` ≤ the resume-pointer line cap"; "the derived Knowledge Map is regenerated + staged" |
| **Oracle (re-run)** | re-EXECUTES a deterministic tool at fixed inputs (golden inputs / fixtures) and asserts the result | strongest — a fabricated claim does not reproduce | may be heavy → defer to CI | "`kg-bench` is 156/156"; "WIRE-BASED-100 constraint+temporal F1 = 1.000"; "`cargo test` / clippy / fmt green" |
| **Evidence (artifact)** | requires a re-checkable artifact for an action that cannot be re-derived (e.g. *how* a gap was diagnosed) — pasted tool output in a tracked location, ideally with the cited command re-run | medium → strong (strong when the cited command is re-run) | cheap (presence) / heavy (re-run) | "a Rust code change's task leaf carries a tool-output WHY+WHERE + a measured before→after (`validate`/`adapt`/`kg-bench`)" |

Rule of thumb: prefer **structural** (cannot be faked) → then **oracle** (re-run beats trust) →
use **evidence** only where the thing being enforced is an *action/process* that leaves no other
re-derivable trace. For evidence checks, make them as oracle-like as possible (re-run the cited
command) so they are not bypassable by pasting fake output.

---

## 4. The check-script contract (precise — this is what makes it portable)

A doctrine check is **any executable** that obeys this contract. Get this right and any project,
any language, can add doctrines that "just work" with the driver.

1. **Exit code is the verdict.** `exit 0` ⟺ the doctrine holds; **any nonzero** ⟺ a breach.
2. **Explain on breach.** On nonzero, print a human-actionable message to **stderr** (what broke,
   where, how to fix). On pass, stay quiet or print one OK line.
3. **Deterministic.** Same repository state → same verdict. No clocks, no network, no randomness
   (or pin the seed). This is what lets the gate be trusted and CI re-run it.
4. **Reads the repository (+ `git`), mutates nothing** (a *derive-and-stage* step — like
   regenerating a derived artifact — is allowed but must be idempotent and explicit).
5. **Scope-aware where relevant.** A check about a *change* should look at the staged set
   (`git diff --cached --name-only`) or an explicit range, and **exempt** changes it does not
   govern (e.g. a code-only doctrine exempts pure-docs commits) — so it never blocks unrelated work.
6. **Self-contained + path-agnostic.** Resolve the repo root from the script's own location
   (`ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"`); reference repo-relative paths only.
7. **Fast, or deferred.** If a check is too slow for pre-commit, keep it in the registry but mark
   it CI-only (run the cheap structural proxy locally, the full oracle in CI).

A check that obeys (1)–(7) is portable: the driver does not care what it checks or how.

---

## 5. The registry + driver (the general enforcer)

One driver owns the list of doctrines and runs them all. The **registry is the source of truth**
for "which doctrines are enforced by what"; a human-readable manifest mirrors it.

- **Registry**: a list of `id | tier | what-it-proves | path/to/check.sh`.
- **Tier**: `gate` (cheap enough for the pre-commit hook, and re-run in CI) or `ci` (§4.7 — too heavy
  for a hook, so it is deferred but stays registered). A default run executes the `gate` tier and
  prints every deferred doctrine as **DEFER**, so a CI-tier doctrine is enforced-but-deferred, never
  silently absent from the report. `--all` runs every tier and is what the CI entrypoint invokes.
- **Driver**: runs every in-tier check (collecting *all* results, not stopping at the first failure),
  prints a per-doctrine report, and exits nonzero iff any failed. It also **meta-checks** that
  every registered check — deferred ones included — exists and is executable, so a registry entry can
  never be a dangling promise.
- **Adding a doctrine** = write a `check_*.sh` obeying §4 + add one registry line. Nothing else.

SpecForge ships the reference driver at `scripts/check_doctrines.sh` and the evidence-archetype
check at `scripts/check_task_acceptance.sh`.

---

## 6. The "reasoned-from-evidence" pattern (process made checkable)

The hardest doctrine to enforce is a *process* ("you followed a root-cause procedure and reasoned
from the evidence"). You cannot read an author's mind — so reframe it into something mechanical:

> **A correct diagnosis is one whose documented cause→fix→effect chain REPRODUCES under
> independent re-execution.**

Mechanize it as a **two-signal evidence check** (the procedure made checkable):

1. **DIAGNOSIS signal (WHY+WHERE)** — the leaf pastes output from the tool that *located and
   explained* the cause (a `validate` finding/metric, an `adapt --target isf` blocking reason, a
   `kg-bench` fixture diagnostic, a failing `cargo test <name>` with a `file:line` locus).
2. **VERIFICATION signal (effect)** — the leaf pastes the *measured before→after* of the change
   (a metric delta, byte-identical golds, `kg-bench` 156/156, WIRE-BASED-100 = 1.000, `run_ci.sh`
   green).

The gate requires **both** (you must have located the cause *and* measured the effect). The
**oracle leg** then re-runs the cited deterministic commands in CI: a fabricated cause→fix→effect
chain will not reproduce, so it fails. At that point the distinction between "reasoned" and
"fabricated" collapses — *a reproducible chain is, operationally, a correct change.* That is the
scientific-method standard, and it is the strongest enforceable proxy for "reasoned from evidence."

### 6.1 A box is EARNED, not ticked (self-ticking is not proof)

A checklist `[x]` an author writes is a **claim**, not proof — a task could tick "NO REGRESSION" and
move on without earning it. So **ticking must never be the proof; the oracle re-run is.** Three legs,
in increasing strength:

1. **Presence (cheap, local hook):** the box exists and is ticked, with a tool-output *signature*
   next to it. This catches "forgot to do the step." It is, by itself, *self-tickable* — be honest
   about that; it is necessary, not sufficient.
2. **Evidence-shape:** the box co-occurs with a string only the real tools emit (a `kg-bench`
   `156/156`, a `WIRE-BASED-100` `1.000`, a `cargo` gate name). Raises the cost of faking, does not
   eliminate it.
3. **Oracle re-run (un-fakeable, CI / `run_ci.sh`):** the gate **re-executes the deterministic
   oracle the box claims** — e.g. a "NO REGRESSION" box is *earned* only when re-running `kg-bench`
   (156/156), the WIRE-BASED-100 constraint+temporal/relation golds, the byte-identical
   evidence/`.isf` checks, and `cargo fmt/clippy/test` all reproduce green. A self-ticked-but-false
   box passes leg 1 and dies at leg 3. **This is the leg that makes the box un-self-tickable.**

Therefore: every gated box **must cite a NAMED, re-runnable oracle** (a gate/command + its
deterministic result), so CI can re-run exactly that and *earn* the box independently of the tick. A
box with no re-runnable oracle stays advisory, never hard-gated on the tick alone. **Honest limit:**
leg 3 lives at CI (E4); SpecForge's hosted CI is currently manual-only (`workflow_dispatch`) to
conserve Actions minutes, so the un-fakeable re-run only happens when someone runs `scripts/run_ci.sh`
or dispatches CI — self-ticking is caught at the next gate run, not instantly. Re-enabling an auto CI
oracle job is what makes "earned, not ticked" hold *no matter what*.

---

## 7. Enforcement layering (E1→E4 — defense in depth)

Same model as `MEMORY_ARCHITECTURE.md` §9. Each layer catches what the last misses.

- **E1 — Discovery.** The doctrine is unmissable: named in the entrypoint docs (`README.md`,
  `TOOLBOX.md`, `docs/decisions/`, the mdBook), and routed to from every harness bootstrap file
  (`AGENTS.md`, `CLAUDE.md`, …). Discovery alone is *not* enforcement.
- **E2 — Self-check.** Each `check_*.sh` (the single source of truth for one doctrine) + the driver.
- **E3 — Git hook.** `.githooks/pre-commit` runs the driver; a non-compliant tree cannot commit
  locally (activate once with `git config core.hooksPath .githooks`). *Honest limit:* a local hook
  can be `--no-verify`'d or skipped if `core.hooksPath` is not set — it catches the common case
  cheaply; it is **not** the backstop.
- **E4 — CI.** The **same** driver runs server-side (`scripts/run_ci.sh`); `--no-verify` cannot
  reach it, so a non-compliant branch **cannot merge**. This is the un-bypassable layer — *only as
  strong as CI actually running.* SpecForge's hosted CI is currently manual-only to conserve Actions
  minutes; that is a real gap, stated not hidden — re-enabling an auto doctrine-gate job restores the
  "no matter what" guarantee.

To land non-compliant work, an author would have to defeat all four — and E4 cannot be defeated
from a clone.

---

## 8. The portable replay manifest (any project, any harness — "it just works")

Reproducible by replay: the **exact list of artifacts** a project copies/writes and the **three
commands** it runs. Path-agnostic and copy-pasteable, exactly like `MEMORY_ARCHITECTURE.md` §9.1.
Group A is verbatim; Group B is one tiny adapt; Group C is per-harness discovery; Group D is your
own doctrines.

### A — CORE, copy VERBATIM (project- and harness-neutral)
| Artifact | Role |
|---|---|
| `scripts/check_doctrines.sh` | the registry+driver — runs every in-tier check, reports (PASS/FAIL/DEFER), exits nonzero on any breach |
| `scripts/check_task_acceptance.sh` | reference EVIDENCE check (the task-acceptance checklist gate) |
| `.githooks/pre-commit` | E3 local gate: regenerate derived artifacts, then run the driver |
| `.githooks/commit-msg` | E3: require an identifier-shaped work-unit id in the subject |
| `DOCTRINE_ENFORCEMENT.md` | this standard |
| `TOOLBOX.md` | the diagnostic-toolbox catalog + the **acceptance-checklist template** a code change must satisfy |

### B — ADAPT (the only project-specific knobs)
- `scripts/check_doctrines.sh`: edit the `DOCTRINES=(…)` array (your doctrine ids → tier → check scripts).
- `scripts/check_task_acceptance.sh`: the "what counts as a code change" path globs + the evidence/checklist signature regexes (your tools' output strings).
- `TOOLBOX.md`: your project's tools + the required checklist boxes.
- which heavy checks are CI-only vs pre-commit.

### C — DISCOVERY, one bootstrap pointer per harness (all IDENTICAL content; each points at `README.md` + `MEMORY_ARCHITECTURE.md` + `TOOLBOX.md` + this file)
`AGENTS.md` (Codex / Amp / common), `CLAUDE.md` (Claude Code), `GEMINI.md` (Gemini CLI),
`.cursorrules` (Cursor), `.windsurfrules` (Windsurf), `.github/copilot-instructions.md` (Copilot).
Ship whichever harnesses your team uses; keep them byte-identical.

### D — OPTIONAL harness hooks (a bonus where supported — NOT required for enforcement)
`.claude/settings.json` (Claude Code `SessionStart`/`PreToolUse` reminders). **Codex and other
harnesses without a hook system rely on Group C discovery + the git-level enforcement (A), which is
harness-neutral.** The reminders only *nudge*; the gate is what *enforces*.

### E — PER-PROJECT, write your own
- `scripts/check_<doctrine>.sh` per doctrine (the §4 contract) + one registry line in the driver.
- `docs/decisions/<directive>.md` for the human "why".

### The three commands (once)
```bash
chmod +x scripts/check_*.sh
git config core.hooksPath .githooks          # activate the local gate (E3)
# add ONE line to your CI pipeline (E4):  bash scripts/check_doctrines.sh --all
```

**Harness-agnostic guarantee.** The ENFORCEMENT (A) is git-level: `.githooks/pre-commit` + CI run
`check_doctrines.sh` regardless of whether the commit came from Codex, Claude Code, Gemini, or a
human. DISCOVERY (C) is per-harness via the bootstrap pointer files. Optional hooks (D) add in-context
reminders where the harness supports them. So a project backed by **Codex or Claude Code (or both)**
gets the **same** four-layer gate — non-compliant work lands only by defeating all four, and E4
cannot be defeated from a clone.

---

## 9. Honest limits (state them; do not over-claim)

- **Local hooks are bypassable** (`--no-verify`, unset `hooksPath`). CI is the real backstop; if CI
  is paused/manual, enforcement is only as strong as the next CI/manual run. *Re-enabling auto CI is
  the true "no matter what."* SpecForge's hosted CI is presently manual-only.
- **Evidence-presence can be gamed** by pasting fake tool output — *unless* the check re-runs the
  cited command (the oracle leg). Prefer structural and oracle checks; make evidence checks
  re-execute where possible.
- **A check cannot prove intent / understanding** — only that the *artifacts and oracles reproduce*.
  That reproducibility is the point: a reproducible cause→fix→effect chain is the operational
  definition of a correct change, regardless of how it was produced.
- **Goal is expensive-and-visible non-compliance, not literal impossibility** — defense in depth,
  not a single unbreakable wall.

---

## 10. The live SpecForge instance (this repo's registry)

The reference deployment. Enforced by `scripts/check_doctrines.sh` via `.githooks/pre-commit` (E3)
+ `scripts/run_ci.sh` / CI (E4).

| Doctrine | Archetype | Tier | Check | Proves |
|---|---|---|---|---|
| `MEMORY-ARCH` | structural | gate | `scripts/check_memory_architecture.sh` | the durable 4-layer memory-architecture invariants (`MEMORY_ARCHITECTURE.md` §9): the standard present, `MEMORY.md` bounded, bootstrap pointers route to it, layers B/C present |
| `KNOWLEDGE-MAP` | structural | gate | `knowledge-map/scripts/check_knowledge_map.sh` | the bounded landing and exact generated question-shard membership/content are regenerated + in sync with fact sources |
| `TASK-ACCEPTANCE` | evidence | gate | `scripts/check_task_acceptance.sh` | a staged Rust code change is owned by a staged `docs/tasks/*.md` leaf whose acceptance checklist carries ROOT CAUSE + ADDRESSED + NO REGRESSION, ticked and backed by SpecForge tool signatures (see `TOOLBOX.md`) |
| `README-POLICY` | structural | gate | `scripts/check_readme_policy.sh` | root `README.md` stays within its independently derived line/byte ceilings; every reader link and author-overflow destination is a repository-owned, controlled terminal in the route registry |
| `LIVE-DOC-SIZE` | structural | gate | `scripts/check_live_document_size.sh` | every parent-tracked Markdown path is classified exactly once; each declared current-state field has an explicit derive-on-read, verified-copy, authored-intent, or immutable-evidence contract; and all lifecycle-specific locality, pressure, route, currency, authority, capture, and history rules pass |
| `PROJECT-DATA-LOCALITY` | structural | gate | `scripts/check_project_data_locality.sh` | Cargo, shell, Rust temp/subprocess, Python dependency, and optional runtime-store paths resolve from the current repository and reject off-root or stale-repository ownership |
| `PRODUCTION-GENERICITY` | structural | gate | `scripts/check_production_genericity.sh` | the product package direction and checker disconnection hold; the complete module/claim/rule/bypass inventories resolve; and the compiled production graph rejects raw/identity-driven semantic control, unregistered canonical mutation, protected-authority forgery, and proofless persistence |
| `CORPUS-FRONTIER` | derive-and-diff | gate | `scripts/check_corpus_frontier.sh` | the SourceIR-derived corpus cohort is partitioned exactly by explicit root-neutral refreshed/remaining sets, retained bundles agree with that lifecycle declaration, and the counts agree with the root task file — moving a source library cannot silently change refresh status |
| `CHAIN-CURRENCY` | oracle | ci | `scripts/check_chain_currency.sh` | every persisted corpus artifact is exactly what the current binary reproduces from its persisted input — the evidence, semantic, intent, and `.isf`-adapter stages replayed `--dry-run`, plus each emitted `.isf` against the adapter's rendered `source_text` — and the retained normalized bundles that make a document replayable are exactly the set declared in `doctrine/chain_currency/retained_bundles.json` (ADR 0025 decisions 2 and 3) |

Every `collection` surface must also declare an aggregate at least as large as its own file bound times its
per-file bound, on both the health and ceiling bands (ADR 0032) — otherwise a corpus whose every file is
legal is refused by a total no single file can see. The sole exemption is a declared `aggregate_composition`
whose member counts sum to the file bound, whose products sum exactly to each total, and whose largest member
equals the per-file bound, so a heterogeneous collection proves its legal maximum instead of asserting it.
Every warning and rollover line additionally names the absolute distance to the enforcement ceiling, because
a surface past its health target reports a percentage of a bound it already blew.

Among its focused suites, `LIVE-DOC-SIZE` runs 81 positive and fail-closed lifecycle/control-plane
cases, 47 neutral derived-state classification cases, 25 SpecForge Rust/gitlink authority-adapter cases,
15 neutral terminal-task source/route/identity/boundary cases, and 29 neutral active-task
source/topology/route/payload/bound cases, plus 41 fact-catalog source/plan/route/residue/bound cases. Test
workspaces are disposable and always created below
repository-local `generated/`.
Schema fields, arrays, scalars, routes, independent size axes, baselines, exact field markers, stored
derive-on-read shadows, verifier execution, capture boundaries, Cargo semver normalization, Git-index
mode/object authority, declared secondary roles/paths/ownership, no-fallback alternate paths, and ceiling history
are therefore re-proved on every gate. Undeclared
date/number/hash lookalikes explicitly remain ordinary prose; the checker never guesses fields.
The mdBook maintained-reference record additionally executes `scripts/check_book_current_truth.sh`,
which binds load-bearing product claims to code, requires the completed containment/locality status
and closing method-doc subsection, and rejects superseded product or migration wording.
The same adapter executes `scripts/check_rolling_ledger_protocol.pl`: ten parser/control self-tests
plus the real four-ledger plan prove whole-record boundaries, exact source identity,
bounded warning-safe survivors, repository-relative archive routes, and required reader/writer seams.
After a ledger migrates, that verifier switches identity authority to its immutable source capsule and
checks the manifest, bounded index, live limits, and exact retained-record suffix.
The adapter also executes `scripts/check_roadmap_projection_contract.pl`: its focused cases and real
contract pin the exact pre-migration roadmap identity, five exhaustive semantic regions, ordered
23-workstream/task ownership, drift evidence, consumers, bounded-current structure, and archive
topology. The migrated contract authenticates the immutable capsule at the pinned identity and
verifies the bounded current root plus manifest/index retrieval on every run. Its sealed-rollover
series (ADR 0030) is proved on the same run: every declared capsule must exist byte-for-byte at its
declared metrics and digest, be reachable from the bounded archive index, carry a chronological seal
date and unique id, stay distinct from the pre-migration capsule, and sit within the *current root's*
enforcement ceilings — so a capsule above them fails closed rather than legitimizing an overflow. The
declared `rollover_policy` bounds the series and prints its count and named remedy at the warning,
because `archive_terminal` surfaces are exempt from the generic milestone report. The same run also
enforces per-section bounds on the current root (ADR 0031): the declared sections must equal the
required H2 order exactly, each carries its own line bound and remedy, and their legal sum plus
scaffold must fit the file's health target — so an accreting section fails closed and names its own
remedy while the file is still under half its ceiling.
The adapter also executes `scripts/check_fsmgen_feedback_protocol.pl`. Its focused cases and real
contract pin the exact feedback source, five exhaustive regions, six closed exchanges with explicit
direction/status/evidence, two stale-current findings, 26 consumers, bounded open-record schema, and
archive topology. Planned state rejects source drift; migrated state will switch identity to the exact
capsule and verify the bounded current channel, closed register, open records, manifest, and index.
The active-task evidence checker separately binds a source-locked task to its boundary commit, Git object/index,
file digest/metrics, exhaustive semantic regions, planned-part pressure, fixed portable caps, and complete
destination absence. Its complete/migrated states add exact legacy route membership, capsule and marker payload
identity, bounded active root/index/parts, manifest/frontier integrity, and Git-backed sealing.

The fact-card catalog checker binds its legacy landing to boundary commit/blob/index, raw digest/metrics, all
canonical cards, and an ordered row digest; legacy state also required the future part directory absent. Its
current migrated state enforces a stable direct-ID landing over deterministic 56-card title parts, exact semantic
tuples and resolved
card destinations, complete output hashes/membership, fixed capacity/pressure/aggregate bounds, and residue-free
repository-local writes. It cross-checks the derived 336-card maximum against the 338-file canonical surface,
derives the 379-fact question authority from that maximum plus the `decision_records` file ceiling rather than
pinning a literal, and requires the title-part generated-projection registry record to be absent in legacy
state and exact in migrated state. The self-test asserts the derivation itself — capacity is the 56-card part
quantum times the six-part count, and each aggregate band is its file bound times its per-file bound — so no
future raise can move one literal and strand another (ADR 0029).

`PROJECT-DATA-LOCALITY` composes three focused shell cases with resulting-tree checks. It proves the
tracked temp root exists before Cargo starts, all canonical scripts establish the common environment,
production temp/subprocess seams use the Rust locality helper, Python environments have tracked lock
authority, and any present venv resolves and launches through this repository. Missing roots and
off-root cache symlinks fail; `~/.rustup` and `~/.cargo` remain explicit shared inputs.

`PRODUCTION-GENERICITY` is the single unconditional composition point for ADR 0006 and ADR 0038. Its
wrapper runs four independent checks without stopping at the first failure: compiler-visible package
direction and checker disconnection; exact production-module and claim-family inventory; exact rule,
field, producer/mutator, seam, and conformance-bypass joins; and the compiled graph's fixed-point raw/
identity information-flow plus proof-only canonical-authority analysis. The registry and analysis use
typed Rust paths and structural classes, never a finite document/vendor/protocol/signal vocabulary.
The proof is deliberately compositional: Cargo compilation and Rust privacy remain the type/capability
oracle, executable current-binary proof replay remains semantic authority, and the AST layer supplies
closed production flow/topology coverage. The default invocation remains the fast clean-tree gate.
`scripts/check_production_genericity.sh --self-test`, called explicitly by `scripts/run_ci.sh`, adds six
forbidden dependency mutations, two inventory/schema mutations, six rule/alpha/bypass mutations, thirteen
flow/authority mutations, legal display/provenance/test-only controls, and an exact runtime join that executes
the structural alpha obligation of all 168 registered rules. This structural qualification does not claim the
population renaming/paraphrase/held-out behavior owned by `.f`.

`CHAIN-CURRENCY` is the registry's first `ci`-tier doctrine. It re-executes the pipeline rather than
reading a claim: each stage is replayed `--dry-run` from the **persisted** upstream artifact, and the
result must equal the persisted downstream artifact. Content identity excludes `validation_reports`,
because `specforge validate` back-annotates that section after the stage runs and the product's own
`*_ir_fingerprint` helpers clear the same field before hashing — the check adopts the code's identity
rule instead of inventing one. Measurability is stated, never implied: only the evidence stage needs a
document's normalized markdown bundle, so a reclaimed bundle makes that one stage **unmeasurable** and
the check always prints that count; every later stage reads a persisted artifact and stays measurable
corpus-wide. An absent `generated/` skips loudly and passes, because a fresh clone and hosted CI have
no corpus for the doctrine to govern.

Its second leg makes measurability itself accountable (ADR 0025 decision 3): a bundle is retained
because a refresh chose to keep it, so `doctrine/chain_currency/retained_bundles.json` declares the
exact retained document-key set, and the check compares it with what is on disk. A declared bundle that
is gone is an unauthorised reclamation; a bundle no leaf declared is a refresh that failed to record
what it kept. Both fail closed, and a deliberate reclamation is a `reclamations` record naming its
owning leaf and reason — which is what "task-owned" means mechanically. The declaration is
schema-closed (unknown, missing, mistyped, unsorted, duplicated, or self-contradictory fields are
breaches), so it cannot decay into free-form prose. `--self-test` proves both comparison cores
fail-closed in sixteen cases before any PASS is trusted.

Deterministic-oracle doctrines that run via `scripts/run_ci.sh` / CI (`kg-bench` 156/156,
WIRE-BASED-100 constraint+temporal/relation golds = 1.000, the byte-identical evidence/`.isf` checks,
`cargo fmt`/`clippy`/`test`/`doc`) are the strongest leg — they re-execute the real tools, so cited
numbers are independently re-verified. They are heavy, so they stay on the CI path, not pre-commit
(§4.7). The registered structural genericity doctrine is the fast tree-wide boundary; the later alpha,
mutation, held-out, and population oracles complement it rather than reducing the invariant to a
literal-name scanner.

To add a doctrine here: write `scripts/check_<id>.sh` (§4 contract), add one line to the driver's
`DOCTRINES` array, and add a row above. The driver's meta-check fails if the script is missing.

---

## 11. Anti-patterns

- ❌ A doctrine that lives only as prose, with no check.
- ❌ "Trust me, I followed the procedure" with no re-checkable artifact.
- ❌ An evidence check that greps for a signature but never re-runs the oracle (fakeable).
- ❌ A registry entry pointing at a check that does not exist (a dangling promise — the meta-check catches this).
- ❌ A check with side effects / nondeterminism (then the gate cannot be trusted).
- ❌ Relying on the local hook as the backstop (it is bypassable — CI is the backstop).
- ❌ Over-claiming "impossible to violate" — the honest claim is "expensive, visible, and blocked at every active gate."

---

*This document is itself an instance of the architecture it describes: a portable, in-repo,
git-tracked standard backed by a runnable driver and mechanical gates — adoptable by any project
by following §8.*
