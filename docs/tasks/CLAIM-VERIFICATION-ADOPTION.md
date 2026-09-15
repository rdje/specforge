# CLAIM-VERIFICATION-ADOPTION: adopt three-leg verification for published claims

## Metadata

- Tree ID: `CLAIM-VERIFICATION-ADOPTION`
- Status: `active`
- Roadmap lane: process / continuity / signoff evidence (cross-cutting)
- Created: `2026-08-15`
- Last updated: `2026-09-16`
- Owner: repo-local workflow
- Owner directive: adopt the upstream claim-verification standard, and re-check it for updates, if SpecForge
  has not already done so.

## Goal

Adopt claim verification as SpecForge's fifth portable architecture. A quantitative or otherwise actionable
published claim must name three dimensionally different legs: a source-derived reproduction, a falsification
oracle capable of distinguishing a competing hypothesis, and a durable tracked producer plus stale-state gate.
Missing legs remain explicit instead of being silently represented as signoff. Non-Goals, acceptance criteria,
and every per-leaf contract live in the task-evidence parts; this root carries the bounded current summary and
the executable owner registry only.

## Current Program State

- The adoption program `.0`-`.5` is complete: the standard, ADR 0042-0044, the bounded registry, the current
  claim census, the mdBook workflow chapter, and the gate-tier doctrine all shipped.
- `.6`-`.6b`, `.10`-`.11a` and `.7`-`.7.3` are complete. Three rounds of re-derivation withdrew published
  figures that did not hold, and the lasting repair is mechanical: `.7.1` executes each producer and compares
  the field, so a stale published count is refused rather than remembered.
- `.8`, `.9`, `.12`-`.15` are complete. `.12` shipped the content-addressed region re-pinner that refuses
  ambiguity; `.13` sized the governed population and refused to widen it; `.14` repaired the one staleness
  marker that carried a per-commit counter; `.15` decided not to execute `stale_check` from the gate.
- Per-leaf goal, acceptance, decision, verification, and measurement detail live in the task-evidence parts.

## Current Frontier

Active adoption frontier: `CLAIM-VERIFICATION-ADOPTION.16`.

`.16` and `.17` are the leaves this tree still records as open. `.16` governs the decision-bearing `N of M`
population only - 285 lines across 43 files, growing about 1.2 per commit - a scope `.13` sized by measuring
the alternative: governing whole surfaces would cost 4,987 records and 5.1 per commit against a sibling
registry whose own capacity bound is 512. `.16` owns its registry and lifecycle rather than extending the
census. `.17` is the first leaf declared after this tree's containment migration, and it asks whether the
census evidence-id convention - the id suffix is its region's content digest prefix, held by 68 of 68
records - should be gated or declared decorative, after one id was invalidated by a content change that the
census accepted in silence.

Two findings this tree raised are owned elsewhere, and the routes are recorded here so no session re-derives
them: executing `durability.stale_check` is `LIVE-DOCUMENT-PRESSURE-HEADROOM.18`, and the byte pressure that
partitioned this tree is `LIVE-DOCUMENT-PRESSURE-HEADROOM.21`.

## Detailed task evidence

[Open the task-evidence index](claim-verification-adoption/INDEX.md) for every detailed task contract,
acceptance checklist, decision, verification entry, commit record, semantic evidence part, and the exact
pre-migration source capsule.

## Executable Owner Registry

These compact declarations preserve stable-path task-owner lookup. The task-evidence index is the primary
detail-routing authority, and the route catalog carries every leaf with its lifecycle and its detail part.

- ID: `CLAIM-VERIFICATION-ADOPTION`
- ID: `CLAIM-VERIFICATION-ADOPTION.0`
- ID: `CLAIM-VERIFICATION-ADOPTION.1`
- ID: `CLAIM-VERIFICATION-ADOPTION.1a`
- ID: `CLAIM-VERIFICATION-ADOPTION.1b`
- ID: `CLAIM-VERIFICATION-ADOPTION.2`
- ID: `CLAIM-VERIFICATION-ADOPTION.3`
- ID: `CLAIM-VERIFICATION-ADOPTION.3a`
- ID: `CLAIM-VERIFICATION-ADOPTION.3a.0`
- ID: `CLAIM-VERIFICATION-ADOPTION.3a.1`
- ID: `CLAIM-VERIFICATION-ADOPTION.3a.2`
- ID: `CLAIM-VERIFICATION-ADOPTION.3b`
- ID: `CLAIM-VERIFICATION-ADOPTION.3b.0`
- ID: `CLAIM-VERIFICATION-ADOPTION.3b.1`
- ID: `CLAIM-VERIFICATION-ADOPTION.3b.2`
- ID: `CLAIM-VERIFICATION-ADOPTION.3b.3`
- ID: `CLAIM-VERIFICATION-ADOPTION.3b.3.0`
- ID: `CLAIM-VERIFICATION-ADOPTION.3b.3.1`
- ID: `CLAIM-VERIFICATION-ADOPTION.3b.3.2`
- ID: `CLAIM-VERIFICATION-ADOPTION.3b.3.3`
- ID: `CLAIM-VERIFICATION-ADOPTION.3b.4`
- ID: `CLAIM-VERIFICATION-ADOPTION.3c`
- ID: `CLAIM-VERIFICATION-ADOPTION.4`
- ID: `CLAIM-VERIFICATION-ADOPTION.5`
- ID: `CLAIM-VERIFICATION-ADOPTION.6`
- ID: `CLAIM-VERIFICATION-ADOPTION.6a`
- ID: `CLAIM-VERIFICATION-ADOPTION.6b`
- ID: `CLAIM-VERIFICATION-ADOPTION.7`
- ID: `CLAIM-VERIFICATION-ADOPTION.7.0`
- ID: `CLAIM-VERIFICATION-ADOPTION.7.1`
- ID: `CLAIM-VERIFICATION-ADOPTION.7.1a`
- ID: `CLAIM-VERIFICATION-ADOPTION.7.2.0`
- ID: `CLAIM-VERIFICATION-ADOPTION.7.2.1`
- ID: `CLAIM-VERIFICATION-ADOPTION.7.2.1a`
- ID: `CLAIM-VERIFICATION-ADOPTION.13`
- ID: `CLAIM-VERIFICATION-ADOPTION.16`
- ID: `CLAIM-VERIFICATION-ADOPTION.12`
- ID: `CLAIM-VERIFICATION-ADOPTION.14`
- ID: `CLAIM-VERIFICATION-ADOPTION.15`
- ID: `CLAIM-VERIFICATION-ADOPTION.8`
- ID: `CLAIM-VERIFICATION-ADOPTION.10`
- ID: `CLAIM-VERIFICATION-ADOPTION.10a`
- ID: `CLAIM-VERIFICATION-ADOPTION.11`
- ID: `CLAIM-VERIFICATION-ADOPTION.11a`
- ID: `CLAIM-VERIFICATION-ADOPTION.7.3`
- ID: `CLAIM-VERIFICATION-ADOPTION.9`
- ID: `CLAIM-VERIFICATION-ADOPTION.17`

## Verification Log

The complete dated log is in the verification-and-chronology part. These are the most recent entries.

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-09-15` | `.15` | all five `stale_check` commands read and classified; declared-binary audit | three of five would invoke the checker that runs them; not executed |
| `2026-09-15` | `.14` | all five `stale_check` commands executed against real stdout | 1 of 5 stale; marker replaced by a phase assertion carrying no count |
| `2026-09-15` | `.13` | sibling-registry capacity measured; per-commit growth derived over the tracked window | widening refused on cost; the narrow successor `.16` is sized |
| `2026-09-15` | `.12` | `repin_claim_regions.py --check` 562 unchanged; `--self-test` 13/13 | content-addressed re-pin refuses ambiguity; registry byte-unchanged |
| `2026-09-15` | `.8` | census rollover lifecycle measured over 120 revisions | 127 to 115 records net; the lifecycle runs 2.6x ahead of the bound |
| `2026-09-14` | `.9` | 458/458 adjudicated; grammar control reverted and observed RED | demonstrated-noun gap closed with a trailing-boundary precision fix |
| `2026-09-14` | `.7.3` | three published self-test case counts bound to their declaring script | all three had gone stale at once; each now re-derives |

## Commit Log

The complete log is in the verification-and-chronology part. These are the most recent entries.

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `.15` | `CLAIM-VERIFICATION-ADOPTION.15 — do NOT execute stale_check` | three of five would be the checker invoking itself |
| `.13` | `CLAIM-VERIFICATION-ADOPTION.13 — decided by cost` | population not widened; the narrow successor is handed a finished sizing |
| `.8` | `CLAIM-VERIFICATION-ADOPTION.8 — closed by measurement` | the census lifecycle already exists and runs ahead of the bound |
| `.14` | `CLAIM-VERIFICATION-ADOPTION.14 — a stale_check with a per-commit counter is stale by construction` | one marker repaired; nothing executes them |
| `.12` | `CLAIM-VERIFICATION-ADOPTION.12 — re-pin claim regions by content` | 562 regions across 63 files; ambiguity refused rather than guessed |
| `.9` | `CLAIM-VERIFICATION-ADOPTION.9 — close the candidate vocabulary's blind spot` | four demonstrated nouns admitted; grammar control goes RED on revert |
| `.7.3` | `CLAIM-VERIFICATION-ADOPTION.7.3 — bind published self-test counts to their script` | three stale counts found at once and made derivable |

## Current Update Protocol

Future work on this tree updates this bounded root and exactly one owning active semantic part in the same
commit. Legacy marked payloads and the exact source capsule are immutable. Route, membership, state, or
measured-metric changes also regenerate the index, route catalog, and manifest through the contract writer.
At warning pressure, split or rotate the active part at a declared task boundary before rollover, then run the
target contract, the doctrine driver, and the commit workflow.
