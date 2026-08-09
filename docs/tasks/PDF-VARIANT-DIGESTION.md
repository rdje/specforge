# PDF-VARIANT-DIGESTION: make SpecForge digest as many chip-spec PDF variants as possible

## Metadata

- Tree ID: `PDF-VARIANT-DIGESTION`
- Status: `active`
- Roadmap lane: `R15`/`R16` extraction breadth
- Created: `2026-06-07`
- Current-state normalization: `2026-08-09` under ADR 0019

## Goal

Make SpecForge ingest and meaningfully extract implementation-relevant intent from the broad variety of chip
specifications it may encounter: protocols, components, systems, software-visible interfaces, registers, tables,
figures, prose, and mixed layouts. A supported document either yields grounded typed intent appropriate to its
content or an honest diagnostic for constructs that remain outside the model.

## Non-Goals

- Do not fabricate semantic certainty or add chip/vendor-specific extraction lists.
- Do not weaken the APB/AHB/AXI/SWD regression baseline.
- Do not treat perfect fact extraction from every page as the prerequisite for measured class-level progress.
- Do not lower IntentIR directly to `.fsm` or HDL; `.isf` remains the SpecForge adapter boundary.

## Acceptance Criteria

- Corpus breadth is measured per document and per structural class rather than inferred from successful parsing.
- Each addressed variant class lands a general, source-grounded extraction or validation improvement.
- Reproducible inputs and regression examples reside in the repository when licensing permits.
- Typed output, residuals, and quality measurements remain provenance-carrying and independently verifiable.
- The wire-protocol golds, focused fixtures, full Rust gates, and mdBook remain synchronized.

## Activity State

The status below applies ADR 0019's precedence rules. Literal historical wording remains unchanged in the exact
source capsule and marked semantic payloads.

| Activity | Current state | Current meaning |
| --- | --- | --- |
| `.1` | done | corpus triage and baseline accounting |
| `.2` | done | deterministic/VLM table-shape and register recovery |
| `.3` | done | prose signal and actor extraction |
| `.4` | done | extraction precision and audit surfaces |
| `.5` | done | document classification and completeness gauge |
| `.6` | blocked_revalidation | legacy VLM candidate requires current evidence and a newly scoped continuation |
| `.7` | blocked_revalidation | legacy USB candidate requires current evidence and a newly scoped continuation |
| `.8` | done | generalized prose-actor capture |
| `.9` | open_without_eligible_child | serial-protocol work landed; one legacy candidate needs revalidation |
| `.10` | done | table/layout/section-heading register and message-field recovery |
| `.11` | done | document-class revisit completed with the re-ingest sweep |
| `.12` | done | trapped signal rows and presence-condition matrices |
| `.13` | done | bounded corpus import and canonical re-ingest sweep |

## Current Frontier

No eligible frontier.

The broad program remains active, but a future continuation must be scoped from current corpus evidence as a new
top-level activity. It must not silently reactivate a stale historical status token.

## Revalidation Conditions

| Legacy candidate | Condition before a continuation may be created |
| --- | --- |
| `.6` | remeasure current zero-yield/VLM opportunities against present artifacts and providers |
| `.7` | establish a current, reproducible USB evidence boundary and a bounded extraction hypothesis |
| `.9.10` | remeasure participation-grounded prose-signal identity after the shallow-parser work |

## Current Leaf Contract

None. A new product leaf requires a newly scoped top-level activity, an owning semantic part, current evidence,
and the ordinary task-tree acceptance checklist before any product or documentation change.

## Detailed task evidence

[Complete semantic history, primary leaf routes, and exact provenance](pdf-variant-digestion/INDEX.md)

## Writer Contract

- Update this root and exactly one owning active semantic part in the same commit.
- Update the index and manifest when route membership, part state, or measured identity changes.
- Split an active part at an existing child/container boundary before mandatory rollover.
- Seal a completed part through the following Git-backed state transaction; continue through a new part.
- Run `perl scripts/check_active_task_evidence.pl --check`, all doctrines, and `COMMIT.md` before proceeding.

## Decisions

- Completion commits plus inline verification outrank stale open headers.
- Explicit unmet dependencies block; ambiguous candidates do not enter the frontier.
- Historical text is immutable inside source-region markers and the exact capsule.
- Current-state truth lives here; detailed task authority lives in the bounded semantic collection.

## Verification Log

| Date | Scope | Result |
| --- | --- | --- |
| `2026-08-09` | committed legacy boundary | 2,393 lines / 222,616 bytes / SHA-256 `9284dce4…a19d4` |
| `2026-08-09` | current-state reconciliation | active program; no inferred legacy continuation |
| `2026-08-09` | bounded topology | 15 exact regions / seven semantic parts / 52 primary routes |

## Commit Log

| Boundary | Meaning |
| --- | --- |
| `f04db37a` | final complete unpartitioned source and exact migration inputs |
| `1abfb49c` | last product-era change to the legacy task source |
