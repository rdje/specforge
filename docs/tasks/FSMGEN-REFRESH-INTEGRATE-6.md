# FSMGEN-REFRESH-INTEGRATE-6: refresh the FSMGen pin and integrate the current upstream contract

## Metadata

- Tree ID: `FSMGEN-REFRESH-INTEGRATE-6`
- Status: `done`
- Roadmap lane: `R6` (× `R15`/`R16` — the real FSMGen strict canary is the executable-intent boundary)
- Created: `2026-08-12`
- Last updated: `2026-08-12`
- Owner: repo-local workflow
- Owner directive (`2026-08-12`): "Whenever you have time, please update FSMGEN submosule as he recently pushed."

## Goal

Advance the read-only `subs/fsmgen` gitlink from the currently pinned `d327129b7` to the fetched
`origin/main` tip, audit every intervening upstream commit and current downstream-integration/book contract,
empirically verify SpecForge's emitted `.isf` surface on the new binary, and integrate only contract or product
truth that the upstream delta actually establishes.

## Non-Goals

- Do not modify or commit inside `subs/fsmgen`; it is an upstream read-only authority.
- Do not infer adoptable behavior from commit subjects alone; inspect the changed contract, handoff, book, and code.
- Do not hide an incompatible contract change by weakening SpecForge's strict canaries or generated expectations.
- Do not mix unrelated SpecForge extraction or trajectory work into this refresh.

## Acceptance Criteria

- The fetched `origin/main` identity, ancestry, exact commit range, and changed-file inventory are recorded.
- `subs/fsmgen` is detached at that exact upstream tip, its working tree is clean, and only the parent gitlink changes.
- Relevant FSMGen handoff, downstream-integration contract, feature matrix, book, and release/change records are read;
  every adoptable, incompatible, or no-action conclusion has exact upstream evidence.
- Focused real-binary strict canaries pass on the new pin; broader SpecForge gates run in proportion to the delta.
- SpecForge does not fabricate rule-over-transaction precedence absent from source authority. The compatibility
  repair must be generic, must preserve source-grounded behavior, and must leave every current emitted artifact
  accepted by the new strict binary without diagnostics.
- FSMGen pin copies, Knowledge Map, mdBook, task/live state, and derived-state checks agree with the index gitlink.
- The slice is committed through `COMMIT.md` with no submodule-local change or project-data residue.

## Task Tree

- ID: `FSMGEN-REFRESH-INTEGRATE-6` · Status: `done` · Goal: refresh cycle 6 — pin, audit, verify,
  and integrate the current upstream contract. Children: `.1`.
- ID: `FSMGEN-REFRESH-INTEGRATE-6.1` · Status: `done` · Goal: fetch and inspect upstream,
  bump the gitlink, retire ungrounded Cartesian rule priorities exposed by the new strict ambiguity gate,
  verify the real binary, reconcile durable/current pin truth, and close the refresh.

## Acceptance Checklist (enforced) — `FSMGEN-REFRESH-INTEGRATE-6.1`

- [x] **REPRODUCE / MEASURE** — fetch `origin/main`; record old/new full revisions, ancestry, commit count,
  subjects, and changed paths before drawing a contract conclusion.
- [x] **ROOT CAUSE (WHY + WHERE)** — classify the upstream delta from its authoritative contract/book/code
  sources and identify exactly which SpecForge integration seam, if any, is affected. For this refresh, prove
  whether the new `isf_ambiguous_rule_transaction_drive_priority` failure is an upstream regression or an
  unsupported precedence assertion emitted by SpecForge.
- [x] **ADDRESSED (verified)** — pin the parent gitlink to the fetched tip and implement only required
  compatibility/current-truth integration established by the audit. Remove or residualize ungrounded
  rule-over-transaction precedence generically; do not special-case the failing document, signal, or protocol.
- [x] **NO REGRESSION** — keep the submodule worktree clean; run focused FSMGen strict canaries, derived pin
  currentness, doctrines, mdBook, and the broader CI gate warranted by the executable dependency update.
- [x] **GENERICITY (ADR 0006)** — do not add document/vendor/protocol exceptions or pin-specific product logic.
- [x] **LOCKSTEP** — task tree, index, Knowledge Map, mdBook, live docs, resume pointer, and every declared
  FSMGen pin copy agree with the stage-zero gitlink.

## Current Frontier

Closed. Refresh cycle 6 has no open leaf; resume the roadmap at `SPEC-TO-INTENT-ALIGNMENT.6b`, subject to
the normal roadmap ranking against the pending `DOC-INTENT-TAXONOMY.4c.ii` reassessment.

## Decisions

- Current gitlink target (abbreviated): `a51dcdad0`.
- `2026-08-12`: use a new refresh tree rather than reopening a closed historical cycle. The current pin is
  `d327129b7`; the new tip and its product meaning remain deliberately unasserted until fetched and inspected.
- `2026-08-12`: fetched tip `a51dcdad0a7e752e638abfe3ab414f7f3911889d` is a fast-forward descendant of
  `d327129b718ab29fc889db026c19257b0f7fcc49`: 1,139 commits dated 2026-06-22 through 2026-08-11,
  touching 2,630 files (`+453851/-143729`). The range materially advances IAL2, IAL1/ISF, HIAL/IASIM,
  VIAL, containment, and governance surfaces; current ISF remains public spec v0.6 and schedule-report v1.
- `2026-08-12`: the new strict binary rejects one of 44 current artifacts with
  `isf_ambiguous_rule_transaction_drive_priority`. Baseline accepts the same artifact. Upstream commit
  `1dbff8fc6` intentionally fails closed when a named drive has multiple local transaction callers because
  no unique transaction owner can be proven. SpecForge's `isf_ir.rs` fabricated every rule over every
  transaction as a Cartesian priority product despite IntentIR/SemanticIR carrying no precedence authority.
  The compatible correction is therefore to retire or residualize that unsupported assertion, not weaken
  FSMGen or add a pin/document/signal exception.
- `2026-08-12`: corpus experiment removed all 3,816 generated priority lines across the 12 affected current
  artifacts. Forty-three of 44 then passed with zero diagnostics. The one remaining AHB artifact exposed the
  second honest relation: `idle_transfer` is the unique local caller of the `HTRANS` named drive while five
  rules can write the same target, so the source must state a winner and does not. The generic emitter repair
  now residualizes rules only for a named-drive target with exactly one distinct local transaction caller;
  it emits no adapter-invented priority. Multi-caller targets remain priority-free because FSMGen cannot assign
  them one transaction owner and accepts that shape without diagnostics.
- `2026-08-12`: current FSMGen also adds bounded static actor-instance/group metadata and transaction-scoped
  actor/pin handoffs. This does not by itself prove that SpecForge's sparse category-3 topology can lower to
  ISF; record a separate roadmap-aligned reassessment rather than silently preserving the older capability premise.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-12` | `.1` baseline | parent index gitlink; submodule status/remote | `d327129b718ab29fc889db026c19257b0f7fcc49`; clean detached worktree; `origin` is `git@github.com:rdje/fsmgen.git` |
| `2026-08-12` | `.1` upstream audit | ancestry; exact range/diff; current contract/book/handoff; manifest | new tip `a51dcdad0a7e752e638abfe3ab414f7f3911889d`; fast-forward; 1,139 commits; 2,630 files; manifest succeeds |
| `2026-08-12` | `.1` focused canaries | `cargo test -p specforge --lib fsmgen_strict -- --nocapture` | 7 passed; 0 failed; 1,855 filtered |
| `2026-08-12` | `.1` corpus compatibility | new strict binary over 44 current `.isf`; exact failing file replayed at old pin | 43 accepted; one `AWSNOOP` multi-caller named-drive ambiguity at new pin; same file accepted at old pin; upstream gate is intentional |
| `2026-08-12` | `.1` no-priority experiment | mechanically remove 3,816 priorities from 12 affected current emits; run all 44 through new strict binary | 43 success / zero diagnostics; one AHB unique-caller `HTRANS` conflict remains, proving deletion alone is insufficient |
| `2026-08-12` | `.1` focused repair | helper/render unit tests; dry-run AXI+ACE multi-caller and AHB unique-caller artifacts | 3/3 tests pass; both real artifacts success / zero diagnostics; AHB gains five `isf_rule_transaction_conflict_*` residuals |
| `2026-08-12` | `.1` rebuilt corpus | rebuild 78 adapter manifests from persisted IntentIR; run all 44 emitted ISFs on new tip; inspect priorities/residuals | 44/44 success / zero diagnostics; zero priority lines; five new residuals in one adapter |
| `2026-08-12` | `.1` chain currency | `scripts/check_chain_currency.sh --check` | evidence 24 current / 54 explicitly unmeasurable reclaimed bundles; semantic 78/78, intent 78/78, adapters 78/78 current; retention exact |
| `2026-08-12` | `.1` signoff tail | format; warnings-as-errors Clippy; full Rust tests; warnings-as-errors rustdoc; docs CI; project-data locality | all pass on the staged product and documentation delta |
| `2026-08-12` | `.1` aggregate CI | `scripts/run_ci.sh` | all 8 doctrines including chain currency; format; warning-deny Clippy; 1,859 passed / 5 ignored / 0 failed; warning-deny rustdoc; mdBook test/build; final locality all pass |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-REFRESH-INTEGRATE-6.1` | `FSMGEN-REFRESH-INTEGRATE-6.1 — pin a51dcdad0 and retire fabricated rule priorities` | refresh, audit, verify, integrate, and close |

## Changelog

- `2026-08-12`: Created before fetching or changing the gitlink in response to the owner's explicit refresh request.
- `2026-08-12`: Recorded the exact upstream inventory and a root-caused compatibility failure before changing
  SpecForge code; expanded `.1` ownership to the generic precedence repair and zero-diagnostic corpus gate.
- `2026-08-12`: Closed `.1` and the refresh tree after all focused checks and the aggregate repository CI gate passed.
