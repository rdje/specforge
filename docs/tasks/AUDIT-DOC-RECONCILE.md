# AUDIT-DOC-RECONCILE: fix doc drift found by the post-ISF-ONLY audit

## Metadata

- Tree ID: `AUDIT-DOC-RECONCILE`
- Status: `done` (re-closed `2026-06-24` after `.3`; first closed `2026-05-18`)
- Roadmap lane: `R0` (live-doc continuity / accuracy)
- Created: `2026-05-18`
- Last updated: `2026-06-24`
- Owner: repo-local workflow

## Goal

Reconcile the two doc-drift findings from the audit:

1. ROADMAP R15 goal / `remaining:` / one completion criterion still
   describe adapter-side actor-relative direction computation and the
   deleted `.fsm` paths. The only adapter (`.isf`) defaults
   direction/width by design; the actor-relative graph model itself
   remains canonical in `SemanticIR`/`IntentIR`. The text contradicts
   the code (the `.1`/`.7` reconcile added a superseding note but left
   the goal/criterion/remaining wording contradictory).
2. `docs/book/src/commands/overview.md` (and `commands/pipeline.md`)
   document `converge` with only 5 flags; `cli.rs ConvergeArgs` also has
   `--max-iterations`, `--vlm-model`, `--nlp-model`,
   `--nlp-max-sentences`, `--prior-memory`, `--rescan-plan-limit` plus
   unstated defaults.

## Non-Goals

- Do not rewrite ROADMAP R15's historical `done:` bullets (they are a
  true record of completed migration work; only the forward/normative
  goal + criterion + `remaining:` are reconciled).
- Do not change CLI behavior — only document what exists.

## Acceptance Criteria

- R15 `goals:` / completion criteria / `remaining:` are consistent with
  ISF-only reality (graph model canonical in IR; adapter defers
  direction to FSMGen by design).
- The book documents the full `converge` flag surface + defaults
  accurately vs `cli.rs`.
- Docs only; mdBook builds; every leaf committed through `COMMIT.md`.

## Task Tree

- ID: `AUDIT-DOC-RECONCILE`
  Status: `done`
  Goal: `R15 text + book converge flags reconciled to code reality.`
  Children: `.1`, `.2`, `.3`

- ID: `AUDIT-DOC-RECONCILE.3`
  Status: `done` (`2026-06-24`, fresh-session PNT slice)
  Goal: >
    Reconcile `RUST_CODEBASE_ANALYSIS.md` to the current codebase. The
    `2026-06-08` "ramp-up currency correction" stated ≈103,200 LoC / 25
    subcommands / 25 IR modules / 1435 lib tests; all four drifted as the
    EXTRACTION-QUALITY-GAUGE / PDF-VARIANT-DIGESTION / KG-ISF-* /
    MEMORY-BOUNDED-INGEST work landed. Add a new dated currency-correction
    section (the doc's established self-maintenance pattern — older dated
    sections stay as historical record) carrying freshly-verified counts.
  Acceptance: >
    New `## Session update (2026-06-24 …)` section at the top with the four
    counts each backed by a reproducible command; no historical section
    rewritten; no architecture claim reversed; numbers must be the live
    deterministic values, not estimates.
  Verification: `passed` — verified at HEAD: `128,742` LoC / `65` `.rs`
    files (`find … -name '*.rs' -exec cat {} + | wc -l`); `28` subcommands
    (`grep -oE 'Commands::[A-Za-z]+' lib.rs | sort -u | wc -l`); `28`
    `ir/*.rs` modules (`ls ir/*.rs | wc -l`); `cargo test -p specforge
    --lib` = `1709 passed, 0 failed, 4 ignored` (ran `2026-06-24`, 6.86s).
    A potential sub-claim about *which* 3 commands were added was withdrawn
    after finding the `2026-06-08` entry's own enumeration undercounted
    (`recover-register-bits` described in prose but omitted from its list) —
    only the verified live total is stated. Docs-only; `check_doctrines.sh`
    GREEN.
  Commit: `AUDIT-DOC-RECONCILE.3 — refresh RUST_CODEBASE_ANALYSIS.md size/command/module/test counts`

- ID: `AUDIT-DOC-RECONCILE.1`
  Status: `done`
  Goal: >
    Reconcile ROADMAP R15 `goals:` / completion criteria / `remaining:`
    to the ISF-only reality: the actor-relative directed graph is
    canonical in `SemanticIR`/`IntentIR`; the `.isf` adapter
    intentionally defaults unknown direction/width and defers scheduling
    to FSMGen; the deleted `.fsm` adapter-direction paths are removed
    from the forward text (historical `done:` bullets stay as record).
  Acceptance: `R15 forward text non-contradictory vs code; mdBook build green; docs only.`
  Verification: `passed` — R15 `status` reconciled (model deliverable
    Done; adapter-direction forward scope retired with the `.fsm`
    adapter); obsolete 3rd `goal` struck-through with the ISF-only
    rationale; both `.fsm`-consumer `remaining:` bullets replaced with
    "none — retired (not deferred)"; `completion criteria` rewritten to
    the two met, code-true criteria. The 2026-05-18 note and ALL 50+
    historical `done:` bullets left verbatim (Non-Goal honored).
    ROADMAP.md is not in the mdBook `SUMMARY`, so no book impact;
    docs-only.
  Commit: `see Commit Log`

- ID: `AUDIT-DOC-RECONCILE.2`
  Status: `done`
  Goal: >
    Document the full `converge` flag surface + defaults in
    `docs/book/src/commands/overview.md` (and `commands/pipeline.md`
    where relevant) accurately vs `crates/specforge/src/cli.rs`
    `ConvergeArgs`.
  Acceptance: `Book converge flags/defaults match cli.rs; mdBook build green; docs only.`
  Verification: `passed` — `pipeline.md#converge` gained a "Flags"
    table with all 12 `ConvergeArgs` entries + exact defaults
    transcribed from `cli.rs` (incl. the previously-undocumented
    `--max-iterations 8`, `--vlm-model`, `--nlp-model`,
    `--nlp-max-sentences 0`, `--prior-memory <default path>`,
    `--rescan-plan-limit 0`); `overview.md` one-liner completed + linked
    to the reference. Full `scripts/run_ci.sh` green (1054 passed;
    mdBook builds). Docs only.
  Commit: `see Commit Log`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `AUDIT-DOC-RECONCILE.1` | `done` | R15 forward text reconciled to ISF-only reality |
| 2 | `AUDIT-DOC-RECONCILE.2` | `done` | Book converge flags/defaults now match `cli.rs` |
| 3 | `AUDIT-DOC-RECONCILE.3` | `done` | `RUST_CODEBASE_ANALYSIS.md` size/command/module/test counts refreshed to live values |

Tree RE-CLOSED `2026-06-24`. The post-ISF-ONLY audit program closed at `.2`;
`.3` re-opened the tree for a single live-doc currency refresh
(`RUST_CODEBASE_ANALYSIS.md` had drifted 4 counts) and re-closed it. The tree
stays the standing home for one-off live-doc↔code currency reconciliations.

## Decisions

- `2026-05-18`: Only R15's forward/normative text is reconciled;
  historical `done:` bullets are preserved as record.

## Open Questions

- None.

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-18` | `AUDIT-DOC-RECONCILE.1` | R15 forward-text reconcile (status/goals/remaining/criteria) vs ISF-only code reality; historical bullets preserved | `passed` (docs-only; ROADMAP not in mdBook) |
| `2026-05-18` | `AUDIT-DOC-RECONCILE.2` | `converge` flag/default table vs `cli.rs ConvergeArgs`; full `scripts/run_ci.sh` incl. mdBook | `passed` (12/12 flags+defaults documented; 1054 passed; mdBook green) |
| `2026-06-24` | `AUDIT-DOC-RECONCILE.3` | `RUST_CODEBASE_ANALYSIS.md` 4 counts vs live tree: LoC (`find … wc -l`=128,742/65 files), subcommands (`grep Commands:: lib.rs`=28), IR modules (`ls ir/*.rs`=28), lib tests (`cargo test --lib`=1709/0/4); `check_doctrines.sh` | `passed` (new dated section added; historical sections preserved; one over-specific sub-claim withdrawn after self-check; docs-only, lib unchanged 1709) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `AUDIT-DOC-RECONCILE.1` | `AUDIT-DOC-RECONCILE.1 — reconcile ROADMAP R15 forward text to ISF-only reality` | docs-only; historical `done:` bullets preserved |
| `AUDIT-DOC-RECONCILE.2` | `AUDIT-DOC-RECONCILE.2 — document full converge flag surface + defaults` | book `pipeline.md`/`overview.md` vs `cli.rs`; tree CLOSED |
| `AUDIT-DOC-RECONCILE.3` | `AUDIT-DOC-RECONCILE.3 — refresh RUST_CODEBASE_ANALYSIS.md size/command/module/test counts` | docs-only; 4 drifted counts refreshed to live deterministic values; tree re-closed |

## Changelog

- `2026-05-18`: Created from the post-ISF-ONLY audit (R15 text drift +
  book converge-flag completeness).
- `2026-05-18`: `.1` done — ROADMAP R15 `status`/`goals`/`remaining`/
  `completion criteria` reconciled to ISF-only reality (canonical
  actor-relative graph in IR + graph-first validation = met; deleted
  `.fsm` adapter-direction forward work retired, not deferred). The
  2026-05-18 note and all historical `done:` bullets preserved verbatim
  per the Non-Goal. Frontier → `.2` (book converge flags).
- `2026-05-18`: `.2` done; tree CLOSED. `pipeline.md#converge` now has a
  12-row Flags table with exact `cli.rs ConvergeArgs` defaults (closing
  the audit's "documents only 5 flags + unstated defaults" finding);
  `overview.md` one-liner completed + linked. Full CI green incl.
  mdBook. **This was the last leaf of the last audit-driven tree — the
  post-ISF-ONLY audit remediation program is complete.**
- `2026-06-24`: Tree RE-OPENED + `.3` done (fresh-session PNT slice,
  surfaced by the session-start codebase survey). `RUST_CODEBASE_ANALYSIS.md`'s
  `2026-06-08` currency section had drifted four counts (≈103.2K→**128,742**
  LoC / 65 files; 25→**28** subcommands; 25→**28** IR modules;
  1435→**1709** lib tests). Added a new `## Session update (2026-06-24 …)`
  currency-correction section with each count backed by a reproducible
  command; historical sections left verbatim; no architecture claim
  reversed. Self-check caught + withdrew one over-specific sub-claim
  (which 3 commands were added — the old entry's enumeration undercounted),
  keeping only the verified live total. Docs-only; `mdbook`/`run_ci`
  orthogonal (no Rust change); `check_doctrines.sh` GREEN. Tree RE-CLOSED.
