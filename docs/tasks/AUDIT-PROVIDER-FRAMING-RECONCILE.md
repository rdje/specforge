# AUDIT-PROVIDER-FRAMING-RECONCILE: reconcile live-doc framing that the LLM/VLM provider "doesn't exist" / R16 CVE crux is "upstream-blocked"

## Metadata

- Tree ID: `AUDIT-PROVIDER-FRAMING-RECONCILE`
- Status: `done`
- Roadmap lane: `R0` (`AUDIT-DOC-RECONCILE` family)
- Created: `2026-05-30`
- Last updated: `2026-05-30`
- Owner: repo-local workflow

## Goal

Reconcile the tracked live docs so they accurately describe what the code
does: the **Ollama + Qwen2.5VL LLM/VLM provider is the production default**
(`converge` defaults both `--vlm-provider` and `--nlp-provider` to `ollama`;
`DEFAULT_LOCAL_MODEL = "qwen2.5vl:7b"`; powers `enrich` VLM diagram
enrichment + `nlp_enrich` NLP Level-3 relation extraction; validated on AMBA
at 90–95/100). Several live docs instead imply the prose LLM/VLM provider
does **not** exist and that the R16 CVE crux is therefore "upstream-blocked."
That framing is inaccurate and is exactly what caused a wrong "the provider
doesn't exist" claim. Fix the framing so the genuinely-remaining R16
extraction work is stated precisely:

- **CVE producer wiring (`parse_constrained_contract`)** — NOT upstream-blocked;
  the prose provider exists. Remaining work = WIRE the existing
  `nlp_enrich`-style provider output into the schema-constrained CVE parser
  (an edit/extension), and add a `signal_relation` extraction type (R14).
- **Typed `FigureRegion` raster/vector decoder** — genuinely the only
  upstream-absent piece (the VLM already reads diagram PNGs to text in
  `VisualAsset.note`, bypassing the typed geometric path).

Also satisfies `SESSION_BOOTSTRAP.md` step 3 ("update
`RUST_CODEBASE_ANALYSIS.md` if necessary") — it is necessary.

## Non-Goals

- No code change. This is a doc-reconcile tree only (`AUDIT-DOC-RECONCILE`
  doctrine: text describes what the code does; never the inverse).
- Does NOT wire the CVE producer or add `signal_relation` — those are real
  code trees to be proposed/authorized separately.
- Does NOT re-open any closed R16 leaf; the remaining extraction items stay
  honestly-deferred FUTURE trees (their *framing* is corrected, not status).

## Acceptance Criteria

- `ROADMAP.md` "Immediate next milestone" + `recommended order` no longer
  imply the prose LLM/VLM provider is absent; they state the provider is
  production-default and scope the remaining R16 extraction work correctly
  (CVE wiring unblocked; only raster/vector `FigureRegion` upstream-absent).
- `docs/TASK_TREE.md` line-40 R16 row reworded the same way.
- `INTENTIR_SPEC.md` stale `.fsm` example blocks (`adapter_targets`,
  `"target": "fsm"`) reconciled to `.isf`.
- `RUST_CODEBASE_ANALYSIS.md` updated: adds the Ollama/Qwen provider as
  production-default (not deferred), lists all 17 commands (adds
  `corpus_kb`, `clean`), and marks/removes the stale `.fsm`-adapter section.
- `MEMORY.md`, `CHANGES.md`, `LIVE_ACHIEVEMENT_STATUS.md`, `DEVELOPMENT_NOTES.md`
  updated per Completion Rules; full `scripts/run_ci.sh` green (incl. mdBook).
- Book method-doc close-rule satisfied (close leaf refreshes the topically
  correct mdBook subsection — `reference/documentation-scope.md`,
  the `AUDIT-DOC-RECONCILE` doctrine home).

## Task Tree

- ID: `AUDIT-PROVIDER-FRAMING-RECONCILE`
  Status: `done`
  Goal: reconcile provider-framing drift across live docs
  Children: `.1`

- ID: `AUDIT-PROVIDER-FRAMING-RECONCILE.1`
  Status: `done`
  Goal: >
    Reconcile the provider-framing drift across the live docs in one
    slice (same ownership area, one signoff unit): (a) `ROADMAP.md`
    (Immediate next milestone + recommended order + R16 section) and
    `docs/TASK_TREE.md` R16 row — provider is production-default, CVE
    wiring unblocked, raster/vector `FigureRegion` the only upstream-absent
    piece; (b) `INTENTIR_SPEC.md` stale `.fsm` example blocks → `.isf`;
    (c) refresh `RUST_CODEBASE_ANALYSIS.md` per SESSION_BOOTSTRAP step 3
    (Ollama/Qwen production-default, all 17 commands incl. `corpus_kb` +
    `clean`, stale `.fsm`-adapter section marked historical). Close the
    tree with the book method-doc subsection + live-doc sync.
  Acceptance: >
    live docs match code reality (no "prose LLM/VLM provider doesn't
    exist" implication anywhere); INTENTIR_SPEC has no `.fsm` adapter-target
    examples; RUST_CODEBASE_ANALYSIS current on provider + 17-command
    surface; book close-rule subsection added to
    `reference/documentation-scope.md`; full `scripts/run_ci.sh` green.
  Verification: >
    passed (`2026-05-30`) — reconciled `ROADMAP.md` (Immediate next
    milestone + recommended-order item 1 + R16 "remaining open scope" +
    the `constrained:` dormancy note), `docs/TASK_TREE.md` R16 row,
    `INTENTIR_SPEC.md` (both `.fsm` example blocks → `.isf`, the adapter
    example now matches the real `IsfAdapterArtifact` serde shape verified
    in `adapters.rs`), and added a dated currency banner to
    `RUST_CODEBASE_ANALYSIS.md` (provider production-default; 17-command
    surface; `.fsm` sections marked historical). Book close-rule subsection
    added to `reference/documentation-scope.md`. Full `scripts/run_ci.sh`
    green (incl. mdBook).
  Commit: `see Commit Log`

## Current Frontier

**Tree closed `2026-05-30`** — single reconcile leaf is `done`.

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `AUDIT-PROVIDER-FRAMING-RECONCILE.1` | `done` | provider-framing reconcile complete; tree CLOSED |

## Decisions

- `2026-05-30`: created after a thorough SESSION_BOOTSTRAP execution found
  the live docs propagate a "prose LLM/VLM provider doesn't exist / R16 crux
  upstream-blocked" framing that contradicts the production-default
  Ollama+Qwen2.5VL provider (`commands/enrich.rs`, `commands/nlp_enrich.rs`,
  `commands/doctor.rs`). Reconcile per `AUDIT-DOC-RECONCILE` doctrine; this
  also discharges `SESSION_BOOTSTRAP` step 3.

## Open Questions

- None. (Whether to WIRE the CVE producer / add `signal_relation` is a
  separate code tree, out of scope here.)

## Blockers

- None.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-05-30` | `AUDIT-PROVIDER-FRAMING-RECONCILE.1` | ROADMAP + TASK_TREE + INTENTIR_SPEC + RUST_CODEBASE_ANALYSIS reconciled; INTENTIR_SPEC adapter example checked against real `IsfAdapterArtifact` serde shape; book subsection added; full `scripts/run_ci.sh` (incl. mdBook) | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `AUDIT-PROVIDER-FRAMING-RECONCILE.1` | `AUDIT-PROVIDER-FRAMING-RECONCILE.1 — reconcile "LLM/VLM provider doesn't exist" framing across live docs; close tree` | docs-only; provider is production-default; CVE wiring NOT upstream-blocked |

## Changelog

- `2026-05-30`: `.1` — reconciled the "prose LLM/VLM provider doesn't exist /
  R16 CVE crux upstream-blocked" framing across `ROADMAP.md`,
  `docs/TASK_TREE.md`, `INTENTIR_SPEC.md` (stale `.fsm` examples → `.isf`),
  and `RUST_CODEBASE_ANALYSIS.md` (dated currency banner); book subsection
  added; **tree CLOSED**.
- `2026-05-30`: Created — reconcile live-doc provider framing + refresh
  RUST_CODEBASE_ANALYSIS per SESSION_BOOTSTRAP step 3.
