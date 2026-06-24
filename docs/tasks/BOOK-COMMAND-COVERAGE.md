# BOOK-COMMAND-COVERAGE: mdBook command-surface drift reconciliation

## Metadata

- Tree ID: `BOOK-COMMAND-COVERAGE`
- Status: `done` (re-closed `2026-06-24` after `.3`; prior re-close `2026-06-24` after `.2`; first closed `2026-06-14`. `.3` reconciled the Commands-overview "Current surface" LIST drift — it enumerated 21 of the 28 `cli.rs` subcommands; the 7 missing ones already had dedicated sections, only the operational-map list was stale)
- Roadmap lane: `R0` (continuity / doc-alignment doctrine; cross-cuts `R15e`)
- Created: `2026-06-14`
- Last updated: `2026-06-24`
- Owner: repo-local workflow

## Goal

Keep the user-facing mdBook locked to the actual CLI command surface with zero
drift (the book is the owner's only window into what the tool does). Close the
verified gap where canonical `specforge` subcommands exist in the CLI but have no
book coverage, so every command a user can run is documented with purpose,
invocation, behavior, and an honest boundary.

## Non-Goals

- Does not change any runtime behavior or CLI surface (documentation-only).
- Does not reconcile the separate `ROADMAP.md` lane-hierarchy drift (recent
  applied trees `PDF-VARIANT-DIGESTION` / `MEMORY-BOUNDED-INGEST` /
  `FULL-PAGE-INTENT-CAPTURE` not reflected in the R-lane structure; R15e status
  understated) — that is recorded as an Open Question and a candidate sibling
  tree, not this tree's scope.
- Does not re-document commands already covered (`eval-extraction`,
  `audit-extraction` were verified present). *(Revised by `.2`: `nli-verify` was
  judged "covered" by `.1`'s ≥1-mention bar — its substantive treatment is in
  `architecture-rationale.md` — but it lacked a section in the Commands chapter
  itself, so `.2` added one. The richer narrative in `architecture-rationale.md`
  stays; the new section is the Commands-reference entry that points to it.)*

## Acceptance Criteria

- Every canonical `specforge` subcommand in `README.md` lines 38–70 has at least
  one substantive book treatment (a dedicated section or an equal-billing
  documented mention).
- The two verified-missing commands (`entity-type`, `extract-conditions`) are
  documented in the topically-correct chapter, in the book's user-friendly
  why-before-what style, with invocation syntax, behavior, flags, and an honest
  residual/boundary.
- `mdbook build docs/book` succeeds.
- Live docs updated where project state changed; committed via `COMMIT.md`.

## Task Tree

- ID: `BOOK-COMMAND-COVERAGE`
  Status: `done`
  Goal: mdBook command surface is locked to the CLI with zero drift.
  Children: `BOOK-COMMAND-COVERAGE.1`, `BOOK-COMMAND-COVERAGE.2`, `BOOK-COMMAND-COVERAGE.3`

- ID: `BOOK-COMMAND-COVERAGE.3`
  Status: `done` (`2026-06-24`, fresh-session PNT slice)
  Goal: Reconcile the **Commands-overview "Current surface" LIST** drift. The
        operational map at `docs/book/src/commands/overview.md` enumerated only
        **21 of the 28** `cli.rs` subcommands — missing `nli-verify`,
        `entity-type`, `extract-conditions`, `extract-constraints-llm`,
        `eval-extraction`, `audit-extraction`, `grits-consensus` (the
        EXTRACTION-QUALITY-GAUGE / GRITS-cross-tool harness members). Distinct
        from `.1`/`.2` (which guaranteed each command has a dedicated *section* —
        all 7 already had one, in `commands/quality-and-learning.md` /
        `quality/extraction-eval.md`); the *overview list* itself was the stale
        surface, so a user scanning "Current surface:" for the full command set
        saw 75% of it.
  Acceptance: the "Current surface:" list enumerates all 28 commands with flag
        signatures cross-checked against the `cli.rs` clap arg structs (NOT just
        `README.md`); the 7 added entries sit after `signal-resolve` (README
        order); a navigation pointer routes the reader to the detail chapters and
        states the accurate CI-safe-no-op behavior (extraction/judge commands
        default to live `ollama` + no-op on `skip`; `eval-extraction`/
        `audit-extraction` default to `skip`; `grits-consensus` is offline);
        `mdbook build docs/book` succeeds; list bullet count = 28.
  Verification: `mdbook build docs/book` exit 0; awk over the "Current surface"
        block counts 28 `- \`` bullets; each of the 7 confirmed `PRESENT`; flag
        defaults re-derived from `cli.rs` lines 268–365 / 539–554 (caught + fixed
        an over-claimed "all default to no-op" — signal-resolve/nli-verify/
        entity-type/extract-conditions/extract-constraints-llm default `ollama`,
        not `skip`); `check_doctrines.sh` PASS.
  Commit: `BOOK-COMMAND-COVERAGE.3 — complete the Commands overview "Current surface" list (21->28)`

- ID: `BOOK-COMMAND-COVERAGE.2`
  Status: `done` (`2026-06-24`, fresh-session PNT slice)
  Goal: Add a **dedicated Commands-chapter section** for `nli-verify` in
        `docs/book/src/commands/quality-and-learning.md`, beside its sibling
        `extract-constraints-llm`. Rationale: `.1`'s bar was "≥1 substantive
        treatment anywhere" — under which `nli-verify` counted as covered because
        of its rich narrative in `architecture-rationale.md` (the honesty-layer
        section). But it was the **lone quality command without a section in the
        Commands reference itself** (14 of 15 quality commands had one), so a user
        browsing Commands → "Quality, Validation, And Learning" to look up how to
        run it found nothing. `.2` raises the bar to "every command has a
        dedicated Commands-chapter section" and closes that navigability gap.
  Acceptance: `## nli-verify` section present in `commands/quality-and-learning.md`
        with invocation syntax, behavior, flags, the CI-safe/abstain honesty
        boundary, and the persisted-gauge + active-gate (`intent --nli-verify`)
        cross-links; facts cross-checked against `commands/nli_verify.rs` +
        `cli.rs`; `mdbook build docs/book` succeeds; the chapter's quality-command
        section count rises 14 → 15.
  Verification: `mdbook build docs/book` exit 0; `grep '^## ' …quality-and-learning.md`
        shows `## nli-verify` (15 sections total); command facts (flags, persisted
        `extraction_quality_gauge`, skip=no-op, text-model, demoted-contracts)
        cross-checked vs `crates/specforge/src/commands/nli_verify.rs`.
  Commit: `BOOK-COMMAND-COVERAGE.2 — dedicated nli-verify section in the Commands chapter`

- ID: `BOOK-COMMAND-COVERAGE.1`
  Status: `done`
  Goal: Document the two verified-missing EXTRACTION-QUALITY-GAUGE-family
        commands `entity-type` and `extract-conditions` in
        `docs/book/src/commands/quality-and-learning.md`, beside their siblings
        `extract-constraints-llm` / `nli-verify`.
  Acceptance: both commands documented (purpose, invocation, flags, behavior,
        honest boundary) in the chapter's style; `mdbook build docs/book`
        succeeds; book-grep shows ≥1 file covering each.
  Verification: `mdbook build docs/book` succeeds; book grep shows
        `entity-type` and `extract-conditions` each in `commands/quality-and-learning.md`;
        command facts cross-checked against `commands/entity_type.rs`,
        `commands/extract_conditions.rs`, and `cli.rs`.
  Commit: `BOOK-COMMAND-COVERAGE.1 — document entity-type + extract-conditions in the mdBook`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| — | (none) | — | Tree RE-CLOSED `2026-06-24` after `.3` — the Commands-overview "Current surface" list now enumerates all 28 `cli.rs` subcommands (was 21), AND every command has a dedicated Commands-chapter section (`.2`). The two book command surfaces (the operational-map list + the per-command sections) are both locked to the CLI. |

Book method-doc close-rule (`BOOK-METHOD-DOC`): satisfied by construction — this
tree's deliverable *is* user-facing book content (the two command sections in
`commands/quality-and-learning.md`), which is the human-facing how + why for the
capability; the task-tree file remains the machine-tracked authority.

## Decisions

- `2026-06-14`: Scope was set by **objective verification**, not the surfacing
  agent's aggregate claim. A book grep showed only `entity-type` (0 files) and
  `extract-conditions` (0 files) are genuinely absent; `nli-verify` (4 files),
  `eval-extraction` (1 file), `audit-extraction` (2 files) are already covered.
  The doctrine "re-derive a census with the build's own rules before trusting
  it" applied — the claimed "5 missing" was actually 2.
- `2026-06-14`: Home chapter = `commands/quality-and-learning.md`, immediately
  after `extract-constraints-llm`, because both commands are the
  LLM-judges/Rust-grounds members of the `EXTRACTION-QUALITY-GAUGE` harness
  (`.1` entity typing, `.2` condition capture) and belong next to their family.

## Open Questions

- `ROADMAP.md` lane-hierarchy drift (owner-relevant but separate): the recent
  applied trees are not reflected in the R-lane structure and R15e reads
  "In Progress" though its done-set is large. Candidate sibling leaf
  `BOOK-COMMAND-COVERAGE.2` or a dedicated `ROADMAP-DRIFT-RECONCILE` tree — does
  not block this leaf.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-14` | `BOOK-COMMAND-COVERAGE.1` | `mdbook build docs/book`; book grep (`entity-type`/`extract-conditions` ≥1 file); `check_memory_architecture.sh`; command facts cross-checked vs source | green (build ok; both commands now covered; gate ok; docs-only, lib unchanged 1611) |
| `2026-06-24` | `BOOK-COMMAND-COVERAGE.2` | `mdbook build docs/book` (exit 0); `grep '^## ' commands/quality-and-learning.md` → `## nli-verify` present (14→15 sections); facts cross-checked vs `commands/nli_verify.rs` + `cli.rs`; `check_doctrines.sh` | green (build ok; dedicated section added; doctrines PASS; docs-only, lib unchanged) |
| `2026-06-24` | `BOOK-COMMAND-COVERAGE.3` | `mdbook build docs/book` (exit 0); awk-count of "Current surface" bullets = 28 (was 21); all 7 added commands grep `PRESENT`; flag signatures re-derived from `cli.rs` clap structs (caught + fixed an over-claimed no-op default); `check_doctrines.sh` | green (build ok; list 21→28; doctrines PASS; docs-only, lib unchanged) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `BOOK-COMMAND-COVERAGE.1` | `BOOK-COMMAND-COVERAGE.1 — document entity-type + extract-conditions in the mdBook` | docs-only; closes the verified 2-command book gap |
| `BOOK-COMMAND-COVERAGE.2` | `BOOK-COMMAND-COVERAGE.2 — dedicated nli-verify section in the Commands chapter` | docs-only; the lone quality command lacking a Commands-chapter section now has one |
| `BOOK-COMMAND-COVERAGE.3` | `BOOK-COMMAND-COVERAGE.3 — complete the Commands overview "Current surface" list (21->28)` | docs-only; operational-map list now enumerates all 28 `cli.rs` subcommands |

## Changelog

- `2026-06-14`: Created task tree; surfaced during the session-start
  README/roadmap/codebase/mdBook read as a verified book↔code command-surface
  drift; `.1` set in_progress.
- `2026-06-14`: `.1` done — documented `entity-type` + `extract-conditions` in
  `commands/quality-and-learning.md`; book builds; gap closed. Tree CLOSED
  (acceptance criteria met). ROADMAP lane-hierarchy drift recorded as a
  spun-out follow-up (separate tree), explicitly out of scope here.
- `2026-06-24`: Tree RE-OPENED + `.2` done (fresh-session PNT slice, surfaced by
  the session-start mdBook survey). A book grep confirmed `nli-verify` — though
  substantively covered in `architecture-rationale.md` (so `.1` correctly counted
  it as covered under the ≥1-mention bar) — was the **only** quality command
  without a dedicated section in the Commands chapter (14 of 15 had one). `.2`
  raised the bar to "every command has a dedicated Commands-chapter section" and
  added `## nli-verify` to `commands/quality-and-learning.md` (purpose, invocation,
  flags, the CI-safe/abstain + persisted-gauge + active-gate honesty boundary),
  facts cross-checked against `commands/nli_verify.rs` + `cli.rs`; `mdbook build`
  green; section count 14 → 15. Tree RE-CLOSED. The richer narrative in
  `architecture-rationale.md` is unchanged — the new entry is the Commands-reference
  pointer to it.
- `2026-06-24`: Tree RE-OPENED + `.3` done (fresh-session PNT slice, surfaced by a
  session-start mdBook drift survey and then RE-DERIVED by hand per the
  "re-derive a census before trusting it" doctrine — `cli.rs` has 28 subcommands,
  `commands/overview.md`'s "Current surface:" list enumerated 21). The 7 missing
  (`nli-verify`, `entity-type`, `extract-conditions`, `extract-constraints-llm`,
  `eval-extraction`, `audit-extraction`, `grits-consensus`) each already had a
  dedicated section (so `.2`'s "every command has a section" bar still held) — the
  *overview list* was the stale surface. `.3` added the 7 (flag signatures
  cross-checked against the `cli.rs` clap arg structs, not just `README.md`) after
  `signal-resolve` in README order, plus a navigation pointer whose CI-safe-no-op
  wording was corrected after re-deriving the actual provider defaults (the
  extraction/judge commands default to live `ollama`, not `skip`). `mdbook build`
  green; list 21 → 28. Tree RE-CLOSED.
