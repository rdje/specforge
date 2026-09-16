# SpecForge doctrine instance — the live registry and its case studies

> This is one deployment of the portable standard [`DOCTRINE_ENFORCEMENT.md`](../DOCTRINE_ENFORCEMENT.md),
> not a second standard. Section numbering is continuous with it, so a citation like
> `DOCTRINE_ENFORCEMENT.md` §10 still names the material here. The standard states the model; this file
> states what SpecForge actually enforces and what it measured while learning to.

## 3. The three check archetypes — the SpecForge cases

The archetype table and the rule of thumb are in the standard. These are the two measured cases that
taught the repository what a check has to compare.

### A census that reports is not a check

Whichever archetype you pick, the check must **compare**. A script that derives a number, prints it,
and compares nothing permits every drift — and reads as green while doing it.

`PRODUCTION-GRAPH-CENSUS-PIN.0` is the measured instance.
`scripts/check_production_genericity_flow.sh` is gate-tier, runs on every commit, and printed all
eighteen repository-wide census counts:

```text
production-genericity-flow: 141 boundary rows; … 2409 functions; 14988 helper edges; …
```

It compared none of them. The only comparison lived in a `cargo test`, which this repository runs
before a push rather than per commit — so the comparison fired at push cadence while the drift accrued
at commit cadence, and the pins were wrong by **+18 functions, +179 helper edges and +226 decision
sites across 29 commits** with every gate green throughout. The number was in the output the whole
time; nothing read it.

**Two rules follow, and the second is the one people get wrong.**

1. If a check derives a value, give it a declaration to compare against and fail closed naming the
   field and both values. The derivation is already being paid for; only the comparison is new.
2. **Say which of your numbers are boundaries and which are sizes**, because they need different
   declarations. `doctrine/production_genericity/flow_census.json` splits them:
   - *boundary* counts (which types are sources, which roots are rules, which regions are trusted)
     describe the authority structure, do not move on ordinary feature work, and are pinned exactly;
   - *volume* counts (functions, helper edges, decision sites, macros) are sizes that move whenever
     production code is added. **Six consecutive slices moved them**, each having to edit a literal to
     land. An exact pin on those measures the commit rate, not the boundary.

   Volume counts therefore use the `baseline` + `delta` + `owner` + `rationale` shape that
   `doctrine/live_document_size/surfaces.jsonl` already uses for the book's byte total. That does not
   remove the per-commit edit — it converts it from re-pinning a number nothing compares into
   **attributing a census change to the leaf that caused it**, which is what was missing across the 29
   silent commits.

### …including the number a check reports about ITSELF

`PRODUCTION-GRAPH-CENSUS-PIN.2` ran that sweep and found the registry in good shape on the numbers its
checks publish about the **repository** — `278 canonical cards`, `301 facts`, `946 files / 57 surfaces`,
`156/156 KG fixtures` are all compared by a derive-and-diff, a contract, or a gold score. The shape
survives one level up instead, in what each check reports about its **own coverage**. Of the 16 that
print a self-test count, **12 cannot detect a change in it**:

**The test is behavioural, and it is the only reliable one:**

```bash
# delete one self-test case, then:
perl scripts/check_<name>.pl --self-test >/dev/null; echo $?
```

`0` means the check cannot see its own coverage shrink. Do **not** try to answer this by reading the
line the check prints — `PRODUCTION-GRAPH-CENSUS-PIN` tried three times and was wrong three times:

| printed | actually |
| --- | --- |
| `self-test 13/13 passed.` — a hardcoded literal | **guarded**: `$passed != 13` forty lines earlier |
| `self-test $total/$total passed.` — looks tautological | **guarded**: `total=19` is a literal, compared with `[ "$passed" -eq "$total" ]` |
| `self-test $passed/$total` — looks independently derived | **NOT guarded** when `$total++` is in the same case loop |
| `self-test $passed/$passed` | not guarded, and honest about it |

**`$passed/$total` is the trap.** It reads as two independent values and usually is not: with
`$total++` inside the case loop, deleting a case drops both and the exit code stays 0. Measured:
`check_book_quantitative_claims.pl` went `19/19` → `18/18`, exit 0. That form detects a **failing**
case, which is a real and different property.

The remedy is a **literal expected count declared beside the suite**, with a `die` on mismatch — not a
contract file, and not `$passed/$total`. After `PRODUCTION-GRAPH-CENSUS-PIN.3` all sixteen checks that
report a self-test count fail closed when a case is removed; before it, twelve did not, and one
(`check_persisted_artifact_paths.pl`) had been publishing 15 for a suite of 18.

Two limits to state whenever this class is reported, because omitting either overclaims:

1. A self-test that **fails** still failed all of these checks — they `die` on a failing case. What was
   unguarded is **coverage**.
2. **A number printed into a void is not evidence.** `check_project_data_locality.sh` runs
   `check_persisted_artifact_paths.pl --self-test >/dev/null`, so its figure never reached a log at
   all. The guard has to be in the exit code, not the message.

`scripts/measure_self_test_coverage_reports.py` emits the report lines and candidate guards as
**evidence and renders no verdict** — a producer that guesses this property publishes the very defect
the doctrine is about. `[[self-test-coverage-guard-is-in-the-exit-path]]` records the method.



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
| `SECTION-ANCHORS` | structural | gate | `scripts/check_section_anchors.pl` | every qualified `` `<path>.md` §`<section>` `` reference resolves to a heading that exists in the file it names. Registered by `LIVE-DOCUMENT-PRESSURE-HEADROOM.4f` after `.4e` partitioned a research record at a section seam, passed all 12 doctrines, and still moved the repository from 20 resolving / 0 unresolved anchors to 13 / 14 — a content-preserving move is not a route-preserving move. Repair is required at the TARGET end (a redirect heading), because seven of those links sat inside sealed `archive_terminal` segments whose source the rollover doctrine forbids editing. Honest limit: bare `§` references carry no path, are resolved from prose context, and are counted but not checked |
| `PROJECT-DATA-LOCALITY` | structural | gate | `scripts/check_project_data_locality.sh` | Cargo, shell, Rust temp/subprocess, Python dependency, and optional runtime-store paths resolve from the current repository and reject off-root or stale-repository ownership |
| `PRODUCTION-GENERICITY` | structural | gate | `scripts/check_production_genericity.sh` | the product package direction and checker disconnection hold; the complete module/claim/rule/bypass inventories resolve; and the compiled production graph rejects raw/identity-driven semantic control, unregistered canonical mutation, protected-authority forgery, and proofless persistence |
| `CORPUS-FRONTIER` | derive-and-diff | gate | `scripts/check_corpus_frontier.sh` | the SourceIR-derived corpus cohort is partitioned exactly by explicit root-neutral refreshed/remaining sets, retained bundles agree with that lifecycle declaration, and the counts agree with the root task file — moving a source library cannot silently change refresh status |
| `CLAIM-VERIFICATION` | structural + oracle | gate | `scripts/check_claim_verification.pl` | the self-bounded claim registry has unique known IDs and honest statuses; every verified claim's tracked producer/input/evidence digests are current; argv-form source and RED-control commands reproduce; stale checks cover every artifact AND are executed at the CI tier, reported as executed / deferred / self-referential so the output never implies a run that did not happen (`LIVE-DOCUMENT-PRESSURE-HEADROOM.18`); and the prepared or committed publication declaration resolves exactly |
| `PUBLISHED-ASSERTIONS` | deterministic-oracle | gate | `scripts/check_published_assertions.pl` | every published value in a governed region **re-derives by executing its named producer and comparing the field** — the leg a digest cannot supply, since a digest proves a region has not changed, never that its numbers still re-derive. A value may instead be `gated`, `authored`, or `dated`, each naming its own evidence; there is no outcome for a value carried on a trajectory. See the paragraph below for the disagreement, enumeration, and self-reference legs |
| `RESIDUAL-ACTIONABILITY` | derive-and-diff | gate | `scripts/validate_residual_actionability_contract.py` | the frozen `.8a` required-residual contract still describes the artifact it claims to describe: its witness pins the tracked current result by digest, its cell decomposition and published ratio **re-derive from that result** rather than being restated, every typed cause resolves to a real production carrier declaration or an honest `null`, and the closed rule cases still produce their frozen required/met counts. Registered by `SPEC-TO-INTENT-ALIGNMENT.9d` after the contract sat red for 43 commits: `SOURCE-IR-REPRODUCIBILITY.2` legitimately republished the current result and no gate required the contract to be re-examined, which is the drift an executable contract exists to prevent |
| `TASK-NODE-RETENTION` | structural | gate | `scripts/check_task_node_retention.py` | a task node may be renamed into descendants or retired through `doctrine/task_nodes/removals.jsonl`, but it may not vanish from every tracked task surface. The task-trees are layer B of `MEMORY_ARCHITECTURE.md` — the only record of why a closed leaf closed the way it did — and every other layer was guarded while their CONTENTS were not: at `ab2c6ee0` one bad splice boundary deleted eight nodes and all fourteen registered doctrines still passed. The rule admits the one legitimate operation, measured rather than assumed: over 200 revisions a node id vanished twice, once as a real split (`.7.2` → `.7.2.0`/`.7.2.1`, correct) and once as that accident. The discriminator was checked against both before it was written and reproduces both verdicts (`TASK-NODE-RETENTION.0`) |
| `OWNERSHIP-CITATIONS` | structural | gate | `scripts/check_ownership_citations.pl` | every work unit cited inside a declared `current_owners` region is CLASSIFIED, and every citation classified as a current owner names a unit whose own `Status:` line is still open. **Why it exists:** eight times a tracked document named a closed tree as the owner of live work, including `LIVE-DOCUMENT-PRESSURE-HEADROOM.7` closing itself while holding three of its own rows. The completeness leg is the point: a NEW citation in a declared region with no record fails closed, which is how the eighth entered `ROADMAP.md` unseen. **Honest limit:** it proves an owner is open, never that it is the right owner, and a `historical` classification is accepted on its stated reason (`.15`) |
| `CONSTRAINT-PART-SPAN` | structural | gate | `scripts/check_constraint_part_span.sh` | the kind classifier's call-site topology still matches the stratification `scripts/measure_constraint_part_span.py` derives its census from, and only the typed gateway reaches its untyped arm. **This row exists because of how it was found:** that script was written to fail closed on exactly that drift, was named in `EXTRACTION-QUALITY-GAUGE.3k`'s verification, and was in no driver. Re-derived per revision with each revision's own scanner it was RED from `.3k.2a` through `.3k.2c` — three leaves that each reported a fully green gate. Writing a fail-closed check is not adopting it (§3). The audit that found it enumerated its siblings, classified: 28 of 86 script-shaped files are unreachable from any driver or doctrine registry, and exactly 2 of those claim a `--check` mode (`EXTRACTION-QUALITY-GAUGE.3k.2j`) |
| `PROOF-SEAL-CURRENCY` | oracle | gate | `scripts/check_proof_seal_currency.sh` | every persisted corpus artifact records a proof seal the CURRENT build's own canonical loader still accepts. The census is total (all five stages, shared predicate `scripts/lib/proof_seal_scan.sh`); the PROBE's scope is per stage since `CORPUS-CHAIN-CURRENCY.6` and active by default since `.9` — `semantic`/`intent` TOTAL, `source-ir`/`evidence` SAMPLED at one per *distinct* seal and saying so, because only those two replay extraction (§10). The probe is the CONSUMING stage in `--dry-run`, never `specforge validate`, which is not idempotent; terminal `adapters/isf` has no consumer and its unprobed status is reported, not counted as a pass. Narrower than `CHAIN-CURRENCY`: a current seal says nothing about content currency (SOURCE-IR-REPRODUCIBILITY.16) |
| `PROOF-SEAL-TOTAL` | oracle | ci | `scripts/check_proof_seal_total.sh` | the same census with EVERY in-scope artifact probed INDIVIDUALLY at every non-terminal stage, which is the only way to see a per-document replay-topology divergence under a shared seal. It exists because the sampled tier once reported green while the canonical loader refused four of twenty-seven documents — every wire-bearing one — for three commits (`SIGNAL-DECLARATION-ROW-DROP.1b`). Controls 17/18 of the owning script's self-test prove the sampled mode MISSES a divergent same-seal document and the total mode names it; control 19 proves a probe whose own input is absent (a held-out normalized bundle) yields NO VERDICT rather than an acceptance. CI tier because a per-document sweep is minutes, not seconds |
| `CHAIN-CURRENCY` | oracle | ci | `scripts/check_chain_currency.sh` | every persisted corpus artifact is exactly what the current binary reproduces from its persisted input — the evidence, semantic, intent, and `.isf`-adapter stages replayed `--dry-run`, plus each emitted `.isf` against the adapter's rendered `source_text` — and the retained normalized bundles that make a document replayable are exactly the set declared in `doctrine/chain_currency/retained_bundles.json` (ADR 0025 decisions 2 and 3) |

Every `collection` surface must also declare an aggregate at least as large as its own file bound times its
per-file bound, on both the health and ceiling bands (ADR 0032) — otherwise a corpus whose every file is
legal is refused by a total no single file can see. The sole exemption is a declared `aggregate_composition`
whose member counts sum to the file bound, whose products sum exactly to each total, and whose largest member
equals the per-file bound, so a heterogeneous collection proves its legal maximum instead of asserting it. A
role may use one scalar count for equal health/ceiling cardinalities or a closed `{health, ceiling}` count object
when the two bands deliberately admit different file counts; both sums are checked independently.
Every warning and rollover line additionally names the absolute distance to the enforcement ceiling, because
a surface past its health target reports a percentage of a bound it already blew.

Among its focused suites, `LIVE-DOC-SIZE` runs 93 positive and fail-closed lifecycle/control-plane
cases, 47 neutral derived-state classification cases, 25 SpecForge Rust/gitlink authority-adapter cases,
15 neutral terminal-task source/route/identity/boundary cases, and 44 neutral active-task
source/topology/route/payload/bound/writer cases, plus 60 fact-catalog source/plan/route/residue/bound cases. Test
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
identity, bounded active root/index/parts, manifest/frontier integrity, and Git-backed sealing. When a contract
declares an executable owner registry in its bounded root, complete routes must equal that registry exactly;
structural or pending owners cannot disappear merely because they have no completion commit. The live-document
driver invokes the PDF, corpus, and alignment contracts independently. Each contract's published verifier must
resolve back to that exact repository-relative contract path; a non-default index cannot silently check the
default PDF authority.

The fact-card catalog checker binds its legacy landing to boundary commit/blob/index, raw digest/metrics, all
canonical cards, and an ordered row digest; legacy state also required the future part directory absent. Its
current migrated state enforces a stable direct-ID landing over deterministic 56-card title parts, exact semantic
tuples and resolved
card destinations, complete output hashes/membership, fixed capacity/pressure/aggregate bounds, and residue-free
repository-local writes. It cross-checks the derived 392-card maximum against the 394-file canonical surface,
derives the current 449-fact authority from that maximum plus the `decision_records` file ceiling rather than
pinning a literal, and derives 4,096 question-key slots by multiplying the fact bound by eight and rounding to
the 512-key registry quantum. It requires the title-part generated-projection registry record to be absent in
legacy state and exact in migrated state. The self-test asserts these derivations, the 56-card part quantum times
the seven-part count, and each aggregate band as its file bound times its per-file bound, so no future raise can
move one literal and strand another (ADRs 0029 and 0041). The 394-file surface bound is the profile's single
anchor: every other value derives from it or from `max_parts`, and the fixtures state it once.

`PUBLISHED-ASSERTIONS` is the answer to a defect this repository recorded ten times and could not close by
review. Values published about its own state went stale under a fully green gate, twice inside the very commit
that published them, and five consecutive rounds of prose correction were each invalidated by their own
transaction. The checker never evaluates a shell string: producers, controls, and enumerators are argv arrays,
every region/producer/control path is Git-tracked, and each governed region is pinned by a one-based line range
plus the SHA-256 of those bytes. Its `--self-test` drives the whole fault matrix RED on a disposable
repository-local fixture — drift, a wrong field binding, an unlisted value in a claim-annotated region, a gated
record with no known-bad control region, a self-referential membership without `excludes_self`, two surfaces
disagreeing on one field, an enumeration that drifted while its size held, a stale region digest, an untracked
producer, an outcome outside the four, a duplicate id, a value absent from its own region, a bound breach, a
compound-adjective value, a ratio-form value, and a record whose value absorbed sentence punctuation — before any
PASS on the real tree is trusted. The numeral grammar that derives the population is itself part of the contract:
a value it cannot see is a silent hole rather than an unlisted one, so a numeral closing a compound adjective
(`27-case`) or a ratio (`15/15`) is a published quantity, only `-` followed by a digit stays an identifier or
date fragment, and a comma joins a numeral only when it separates exactly three digits
(`CLAIM-VERIFICATION-ADOPTION.7.1a`). The drift leg was additionally observed RED on **real shipped
prose** by revert-and-re-apply rather than on a fixture alone. The registry is **`frozen`**: its population is
complete, so an unlisted value in a governed region is fatal rather than reported
(`CLAIM-VERIFICATION-ADOPTION.7.2.1`). Its scope is **fail-closed**: every tracked Markdown file
carrying a claim tag is discovered by scanning and resolved to the live-document surface that owns it, and a
file whose surface declares no disposition — or that no surface owns — is an error. Only a surface's
disposition is authored, never its membership, and an exemption must state its reason; a glob list was replaced
because it fails open in the one direction that matters, a file nobody listed being a file nobody checks
(`CLAIM-VERIFICATION-ADOPTION.7.2.0`).

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
the structural alpha obligation of all 170 registered rules. This structural qualification does not claim the
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
fail-closed before any PASS is trusted; the exact case count is published once, at toolbox §7.2a in
`docs/toolbox/gates-build-and-host.md`, where `perl scripts/report_self_test_totals.pl` re-derives it
from the script on every commit. It is stated in one place because it was stated in three and all
three went stale (`CLAIM-VERIFICATION-ADOPTION.7.3`).

**The seal probe's scope is a per-stage decision, and it is ON at `semantic` and `intent`.** A probe
that loads a persisted artifact and verifies its proof is cheap; one that replays extraction from a
normalized markdown bundle is not, and the chain contains both. So `PROOF-SEAL-CURRENCY` probes every
in-scope artifact at `semantic` and `intent` and samples one per distinct seal at `source-ir` and
`evidence`, where it prints that it is blind. The measured reason a per-seal probe is not enough: the
corpus carries **1 distinct seal over 27 artifacts**, so that sample is 1 in 27, and at `48def695` it
reported green while the canonical loader refused **4 of the 27 — every wire-bearing specification —
for three commits** (`SIGNAL-DECLARATION-ROW-DROP.1b`). Activating the two total stages cost the gate
**4m13.0s → 5m30.1s and 5m24.5s over two post-change samples, so +72-77 s, about +29%**, measured
before and after on one tree (`CORPUS-CHAIN-CURRENCY.9`).
Self-test **21** pins the shipped default itself rather than the mechanism — 17b passes the stage set
in explicitly, so while the default was empty nothing tested it — and emptying it now goes RED at
20/21. `PROOF-SEAL-TOTAL` (CI tier, 1m59.2s) covers the two sampled stages.

**Which build answers for the corpus is one predicate, not three.** `CHAIN-CURRENCY`,
`PROOF-SEAL-CURRENCY`/`PROOF-SEAL-TOTAL` and the `rebuild_stage_cascade.sh` remedy all replay the
persisted corpus against the current build, and a gate and a remedy that disagreed about which loader
answers for an artifact would leave a debt no compliant work could clear. The seal read and the chain
table already live in `scripts/lib/proof_seal_scan.sh`, and content identity in
`scripts/lib/stage_artifact_identity.sh`, for exactly that reason; the binary was the third such
predicate and was copied out three times. `scripts/lib/corpus_replay_binary.sh` is its one home, and it
owns the profile — **release** since `CORPUS-CHAIN-CURRENCY.8`, measured rather than assumed. The
verdict is profile-independent (proof verification is digest comparison and ordered-map lookup; the
workspace has no `cfg(debug_assertions)`, and a 43 MB `intent --dry-run` is byte-identical between the
two profiles), while the cost is not: one probe over a 39.7 MB artifact is 6.5 s against 63.1 s. A cold
release build of this workspace is 36 s, and neither profile is ever built on a corpus-less machine,
because every one of the three entrypoints skips before it reaches the build.

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

