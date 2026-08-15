# SPEC-CLARIFICATION-LOOP: autonomous-first, user-assisted specification completion

## Metadata

- Tree ID: `SPEC-CLARIFICATION-LOOP`
- Status: `active`
- Roadmap lane: extraction quality, residual actionability, and PDF-to-ISF completion
- Created: `2026-08-15`
- Last updated: `2026-08-15`
- Owner: repo-local workflow
- Related trees: `SPEC-TO-INTENT-ALIGNMENT`, `AMBIGUITY-PHRASE-DETECTOR`,
  `KG-ISF-COMPLETENESS`, and `KG-ISF-TRANSACTIONS`

## Goal

Make SpecForge fully autonomous wherever source evidence and governed inference suffice, and fully user-assisted
where they do not. When automation cannot safely complete a PDF-to-IntentIR-to-FSMGen-ISF chain, SpecForge must
produce a precise, source-linked, efficiently answerable clarification packet that explains what is missing,
why it matters, what answer forms are acceptable, and exactly what will resume after a validated answer.

The terminal product loop is:

```text
PDF -> autonomous proof-bearing pipeline -> complete ISF
                         |
                         +-> typed clarification packet -> user answer -> validation
                                                               |
                                                               +-> minimal proof-bearing resume -> complete ISF
```

## Product Contract

Each clarification must be a typed artifact, not free-form log prose. At minimum it identifies:

- the source document, current artifact/revision, pipeline stage, exact source spans or visual/table regions,
  and relevant proof or residual identities;
- the unresolved engineering proposition, known alternatives, why autonomous evidence is insufficient, and
  whether the cause is source ambiguity, contradiction, missing source content, extraction limitation, or an
  external design/configuration choice;
- every canonical fact, validation condition, adapter decision, or ISF construct blocked downstream;
- risk, priority, expected information gain, dependencies on other questions, and whether unaffected work may
  continue autonomously;
- accepted answer schema, units/domain/constraints, examples where useful, validation rules, and an explicit
  way to answer unknown, unavailable, not applicable, or defer;
- stable status/currentness, answer provenance, supersession history, and the deterministic resume plan.

An answer is evidence, not automatic truth. It must pass its declared schema, consistency, grounding, authority,
and conflict checks before it may extend canonical proof. Rejected, incomplete, stale, or contradictory answers
remain visible with a specific repair request. A validated answer resumes only the affected dependency closure;
already valid autonomous work is retained, and repeated execution is deterministic and crash-resumable.

## Non-Goals

- Do not fabricate a default merely to avoid asking the user.
- Do not ask the user for information SpecForge can recover autonomously with acceptable cost and confidence.
- Do not turn every residual or warning into a blocking question; deduplicate, group, rank, and distinguish
  blocking from advisory packets.
- Do not let a user answer bypass source/currentness/proof rules or silently rewrite unrelated canonical facts.
- Do not claim this interaction layer alone closes extraction recall, ISF expressiveness, or every remaining
  PDF-to-ISF gap; it makes unresolved boundaries efficient, explicit, and resumable.

## Acceptance Criteria

- One versioned typed clarification schema covers source authority, missing-information taxonomy, downstream
  impact, accepted answers, lifecycle/currentness, and resume instructions without absolute persisted paths.
- Deterministic planning converts eligible residuals, contradictions, completeness findings, and blocked adapter
  decisions into deduplicated, dependency-ordered packets while autonomous work continues where safe.
- Human-readable and machine-readable interfaces explain each question thoroughly and accept validated answers
  without granting unreviewed authority.
- Transactional replay consumes accepted answers through the proof-carrying pipeline, invalidates only affected
  descendants, preserves unrelated current work, and resumes safely after interruption.
- Conformance fixtures prove autonomous completion, assisted completion, unknown/defer, stale answer, conflict,
  tampering, question deduplication, answer supersession, and minimal-resume behavior.
- An end-to-end governed PDF scenario reaches a well-structured FSMGen ISF after a bounded clarification exchange;
  unsupported or still-missing information remains explicit rather than being laundered into success.
- The mdBook documents the complete user workflow with examples, and focused checks plus milestone full CI pass.
- Each completed leaf is committed through `COMMIT.md`.

## Task Tree

- ID: `SPEC-CLARIFICATION-LOOP`
  Status: `active`
  Goal: deliver the autonomous-first, user-assisted completion loop from unresolved PDF evidence to resumed ISF
  Children: `.0`, `.1`, `.2`, `.3`, `.4`, `.5`, `.6`, `.7`

- ID: `SPEC-CLARIFICATION-LOOP.0`
  Status: `done`
  Goal: freeze the product contract, scope, task decomposition, and public direction
  Acceptance: the tree distinguishes this loop from flag-only ambiguity detection; roadmap, mdBook, live status,
  Knowledge Map, and resume pointer describe autonomous-first behavior, typed assistance, validated answers, and
  proof-bearing resume without claiming the runtime already implements it; the required live-status record is
  preserved through an exact whole-record rollover with no edited sealed record or widened limit
  Verification: focused catalogs, Knowledge Map, live-document/roadmap contracts, exact dry-run/applied status
  rollover, mdBook, diff hygiene, and the mandatory doctrine driver pass; this direction-only slice
  intentionally does not run full CI
  Commit: `SPEC-CLARIFICATION-LOOP.0 — own autonomous and assisted completion`

- ID: `SPEC-CLARIFICATION-LOOP.1`
  Status: `done`
  Goal: define the versioned clarification, answer, and lifecycle IR plus its authority boundary
  Acceptance: schemas cover stable identity, source/proof links, gap taxonomy, alternatives, downstream impact,
  priority/information gain, accepted response types, unknown/defer, answer provenance, status/currentness,
  supersession, and resume closure; compatibility fails closed; an ADR freezes what user evidence may authorize
  Verification: 10 focused schema/currentness/compatibility tests; warning-denied workspace Clippy; all five
  production-genericity components at 79 modules / 41 families / 168 fields; warning-denied Rustdoc; mdBook
  test/build; Knowledge Map, fact/decision catalogs, live-size contracts, and mandatory doctrines pass
  Commit: `SPEC-CLARIFICATION-LOOP.1 — define typed clarification and answer authority`

- ID: `SPEC-CLARIFICATION-LOOP.2`
  Status: `pending`
  Goal: plan minimal, high-value clarification packets from governed unresolved state
  Acceptance: one deterministic planner joins eligible residuals, validation/completeness findings,
  contradictions, and blocked lowering decisions; it groups equivalent questions, orders dependencies, computes
  downstream impact, avoids answerable autonomous work, and distinguishes blocking from advisory packets
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-CLARIFICATION-LOOP.3`
  Status: `pending`
  Goal: expose efficient machine-readable and human-readable question workflows
  Acceptance: repository-relative CLI/artifacts list, inspect, export, and answer packets; explanations include
  exact evidence, missing information, alternatives, impact, acceptable answer form, units/constraints, and next
  action; batching and dependency order minimize user round trips; all project data stays repository-local
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-CLARIFICATION-LOOP.4`
  Status: `pending`
  Goal: validate, reconcile, and persist user answers without granting unchecked canonical authority
  Acceptance: typed parsing, grounding/authority/currentness checks, unit/domain validation, cross-answer and
  source contradiction detection, explicit rejection/repair packets, unknown/defer handling, provenance,
  supersession, and tamper-evident persistence all fail closed and are deterministic
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-CLARIFICATION-LOOP.5`
  Status: `pending`
  Goal: resume the minimal affected proof-bearing pipeline transactionally after accepted answers
  Acceptance: dependency closure invalidates and rebuilds only affected descendants; retained autonomous facts
  remain byte/proof current; interrupted runs resume idempotently; answer withdrawal or supersession stales the
  exact dependent claims; adapter/ISF emission remains blocked until every required premise verifies
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-CLARIFICATION-LOOP.6`
  Status: `pending`
  Goal: qualify interaction efficiency, safety, genericity, and honest terminal states
  Acceptance: controlled fixtures cover question floods, duplicates, dependency cycles, stale/tampered/partial/
  conflicting answers, identity and symbol renaming, source movement, unknown/defer, unaffected autonomous
  progress, and bounded round trips; no document/vendor/protocol-specific branch enters production
  Verification: `pending`
  Commit: `pending`

- ID: `SPEC-CLARIFICATION-LOOP.7`
  Status: `pending`
  Goal: publish end-to-end autonomous and assisted PDF-to-FSMGen-ISF qualification and close the tree
  Acceptance: governed examples include a zero-question autonomous path and a bounded-question assisted path;
  accepted answers produce source/answer-linked high-quality IR and strict-valid FSMGen ISF, while unresolved
  cases remain explicit; controller, docs, book, retrieval, cleanup, and selected full CI agree
  Verification: `pending`
  Commit: `pending`

## Acceptance Checklist (enforced) — `SPEC-CLARIFICATION-LOOP.1`

- [x] **REPRODUCE / MEASURE** — before this leaf, the IR exposed zero versioned clarification or answer
  envelopes; the only adjacent carrier was `ResidualDecisionPacket`. The leaf adds one schema-1 clarification
  module and exercises it with 10 focused tests.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/source.rs:3613` carried a question, rationale,
  alternatives, and source span, but no stable definition digest, lifecycle/current binding, typed response,
  answer provenance, authority class, or supersession chain. Consequently, the runtime had no fail-closed
  contract through which user-supplied information could later re-enter proof-bearing processing.
- [x] **ADDRESSED (verified)** — `cargo test -p specforge-core clarification --offline` passes 10/10 tests over
  packet validation, typed values, definition digests, current artifact/policy bindings, supersession, and
  fail-closed compatibility; every persisted compatibility result still reports
  `permits_canonical_authority() == false`.
- [x] **NO REGRESSION** — `cargo fmt --all --check`, warning-denied workspace `cargo clippy`, and warning-denied
  Rustdoc pass; the production-genericity gate passes all five components at the unchanged canonical denominator
  of 41 families / 168 fields. This schema-only leaf changes no extraction, semantic, intent, adapter, or
  emitter producer, so KG and WIRE-BASED-100 output surfaces are orthogonal by construction.
- [x] **GENERICITY (ADR 0006)** — the schema and validators use structural evidence, lifecycle, authority, and
  value-kind vocabulary; production code contains no chip, vendor, document, or protocol-name special case.
- [x] **LOCKSTEP** — ADR 0040, the clarification-loop mdBook chapter, architecture rationale, task tree, live
  docs, Knowledge Map fact card, generated catalogs, production inventory, and resume pointer describe the same
  implemented foundation and preserve `.2` as the planner frontier.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `SPEC-CLARIFICATION-LOOP.0` | `done` | product contract and public direction are frozen |
| 2 | `SPEC-CLARIFICATION-LOOP.1` | `done` | schema-1 exchange and fail-closed authority are frozen |
| 3 | `SPEC-CLARIFICATION-LOOP.2` | `pending` | planner consumes the frozen schema and current unresolved surfaces |
| 4 | `SPEC-CLARIFICATION-LOOP.3` | `pending` | UX renders planned packets without becoming semantic authority |
| 5 | `SPEC-CLARIFICATION-LOOP.4` | `pending` | answer ingestion depends on schema and interface contracts |
| 6 | `SPEC-CLARIFICATION-LOOP.5` | `pending` | proof-bearing resume depends on validated answer authority |
| 7 | `SPEC-CLARIFICATION-LOOP.6` | `pending` | qualification attacks the complete implemented loop |
| 8 | `SPEC-CLARIFICATION-LOOP.7` | `pending` | end-to-end closure follows all component evidence |

## Decisions

- `2026-08-15`: autonomy is the default; ask only when governed evidence cannot safely decide or when an external
  design choice is intrinsically outside the PDF.
- `2026-08-15`: a clarification is a typed, versioned, source-linked artifact with downstream impact and a resume
  plan, never an ephemeral chat question or free-form log line.
- `2026-08-15`: a user answer is a new provenance-bearing premise subject to schema, grounding, consistency,
  authority, and currentness checks; it cannot bypass the proof kernel.
- `2026-08-15`: minimize user effort by grouping, deduplicating, dependency-ordering, and ranking questions by
  blocking impact and information gain while continuing unaffected autonomous work.
- `2026-08-15`: `AMBIGUITY-PHRASE-DETECTOR` remains a valid flag-only detector; this tree owns the broader
  question/answer/resume lifecycle and may consume that detector's findings without redefining them.
- `2026-08-15`: `.1` resolves answer authority by keeping every persisted answer envelope untrusted. Source
  locators must resolve to native captured proof, supplements must become governed source, and only an external
  design choice may later use a dedicated narrowly registered proof premise; there is no generic user-answer
  premise.

## Open Questions

- `.2` must calibrate which unresolved findings are worth asking versus leaving as nonblocking residuals; the
  policy must be evidence-driven and generic, not a hardcoded corpus threshold.

## Blockers

- None. `.2` is the next executable leaf.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-15` | `.0` | task and fact catalogs; Knowledge Map; live-document and roadmap contracts; exact
  whole-record status-ledger dry run/apply; mdBook test/build; diff hygiene; mandatory doctrine driver | `passed`;
  segment 0010 is 21 records / 23,508 bytes at `dce3a183…c71e`; live root is warning-safe at 51 records |
| `2026-08-15` | `.1` | 10 clarification tests; warning-denied workspace Clippy/Rustdoc; production
  genericity; mdBook; catalogs/live-size; mandatory doctrines | `passed`; canonical denominator remains 41
  families / 168 fields; graph is 79 files / 2,332 functions / 13,970 edges / 12,196 decisions / 1,457 macros |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.0` | `SPEC-CLARIFICATION-LOOP.0 — own autonomous and assisted completion` | direction frozen; runtime starts in `.1` |
| `.1` | `SPEC-CLARIFICATION-LOOP.1 — define typed clarification and answer authority` | schema 1 + ADR 0040; planner is next |

## Changelog

- `2026-08-15`: Created from the product direction that PDF-to-FSMGen-ISF should be autonomous wherever possible
  and should otherwise explain missing information thoroughly, accept validated feedback, and resume efficiently.
- `2026-08-15`: The one required live-status record reached the 72-record fail-closed threshold, so `.0` also
  owns the exact segment-0010 rollover necessary to preserve that public alignment without changing a limit.
- `2026-08-15`: `.1` implements the versioned exchange/currentness foundation and closes the answer-authority
  decision without claiming planner, validator, or replay behavior.
