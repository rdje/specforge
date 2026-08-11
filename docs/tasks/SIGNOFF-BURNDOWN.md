# SIGNOFF-BURNDOWN: one unsaturated top-line number, over a fixed target set, driven to done

## Metadata

- Tree ID: `SIGNOFF-BURNDOWN`
- Status: `proposed`
- Roadmap lane: cross-cutting — supersedes nothing in the pipeline; changes what the project steers by
- Created: `2026-08-11`
- Owner: repo-local workflow
- Supersedes: `MEASUREMENT-PLANE-CONVERGENCE-RISK` (same concern, wrong root cause — see below)
- Raised by: the director — *"we keep executing task-trees after task-trees with no objective end in sight."*

## The diagnosis, measured

The intuition that the project is stalling is correct, and the cause is **not** a missing gate. The gate
exists, is owner-declared non-negotiable, and has a rigorous no-fake-scoring rule: `WIRE-BASED-100`.

**The gate is saturated.** Re-run `2026-08-11` against the current release binary, every
`-- source-tolerant + filtered (WIRE-BASED-100) --` surface:

| Dataset | Result |
| --- | --- |
| `seed_apb` / `seed_apb_temporal` | 1.000 across constraints, relations, temporal |
| `seed_ahb` / `seed_ahb_temporal` | 1.000 across constraints, relations, temporal |
| `seed_axi` / `seed_axi_temporal` | 1.000 across constraints, relations, temporal |
| `seed_swd_derivation` | 1.000 across frame fields, operations, protocol states, edge timing |
| `seed_i2c_signals` | 1.000 declared signals |
| `seed_swd` | **0.000 signal_constraint** (tp=0 fn=1) — the one known `CSYSPWRUPACK` residual |

**93 gold facts, one of them failing.** A gate reading 100% carries no gradient: it cannot say what to do
next. So work flowed to the only lanes that still produced visible movement — and both are unbounded by
construction.

| Tree | Commits | Last touched |
| --- | ---: | --- |
| `CORPUS-COVERAGE` (breadth — always another PDF) | 103 | **today** |
| `PDF-VARIANT-DIGESTION` (breadth) | 77 | 2026-07-25 |
| `LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION` (governance) | 41 | recent |
| `KG-ISF-COMPLETENESS` (north star) | 37 | today |
| **`WIRE-BASED-100`** (the signoff gate) | **28** | **2026-06-07** |
| `KG-ISF-TRANSACTIONS` (north star) | 22 | 2026-06-17 |

The two trees that define *done* have been dormant for two months. 2,682 commits, 116 closed trees.

## Why the gate saturated: gold is not proportionate to output

The gold was sized to seed the method, never rescaled to the artifact it now judges:

| Target spec | Emitted `.isf` constructs | Gold facts | Verified |
| --- | ---: | ---: | ---: |
| AMBA APB (`ihi0024_e`) | 67 (32 sig · 3 txn · 32 rule) | 22 + a complete 35/35 signal catalog (`.3a`) | **high** |
| AMBA AHB (`ihi0033_c`) | 88 (41 · 2 · 38 · 4 · 3) | 21 | ~24% |
| AMBA AXI (`ihi0022_l`) | 502 (289 · 7 · 121 · 14 · 71) | 13 | **~2.6%** |
| AMBA AXI-Stream (`ihi0051_b`) | 50 (22 · 1 · 26 · 1) | **0** | **0%** |
| ARM ADI/SWD (`ihi0074_a`) | 58 (11 · 0 · 1 · 1 · 45) | 31 | ~53% |

**APB is the only spec whose gold is proportionate to its output, and it is the only spec the project
declared done.** The method works. It was simply never scaled to the other four.

Meanwhile the reported headline — *"44 emitted `.isf`, 44/44 FSMGen `--strict` clean"* — measures syntax, and
the project already proved on its own evidence that strict success is not a fidelity oracle
([[dense-prose-false-signal-loop-reaches-isf]]). Under it, the real distribution is thin: of 78 documents,
44 render, but only **12 carry ≥30 signals**, only **15 carry any transaction at all**, and several emit
mostly storage noise (CoreSight SoC-600: 5 signals, 828 storage records; CCIX: 2 signals, 131 storage).

## Why `MEASUREMENT-PLANE-CONVERGENCE-RISK` is superseded

That tree said the project cannot measure whether a document was understood. True but imprecise: the
instrument exists and is good. The correct statement is **the instrument is pegged**, and a pegged
instrument is indistinguishable from no instrument for the purpose of steering. The fix is not to invent
measurement — it is to rescale the measurement that already exists.

## The plan

**Change what the project steers by, in five moves.** Nothing here touches the IR spine, the stage
boundaries, `.isf` as the product boundary, or the fail-closed grounding rule.

| Leaf | Status | Scope |
| --- | --- | --- |
| `SIGNOFF-BURNDOWN.0` | `proposed` | fix the target set at five wire specs — APB, AHB, AXI, AXI-Stream, ADI/SWD. The other 73 documents are context and regression ballast, not targets. Guides, PHY notes, and register specs are *correctly* empty (`SIGNAL-CATALOG-CAPTURE-GAP.1`); keeping them in the denominator manufactures permanent failure |
| `SIGNOFF-BURNDOWN.1` | `proposed` | **complete the declared-signal catalog gold for the four unfinished targets**, the way `WIRE-BASED-100.3a` already did for APB (35/35). This is four signal tables, not 500 hand-labeled facts — and the catalog is the one artifact a wire spec states exhaustively, and the one that gates everything downstream since grounding began refusing undeclared subjects |
| `SIGNOFF-BURNDOWN.2` | `proposed` | replace the headline: publish **catalog recall + fabrication count per target**, e.g. "AXI: 289 emitted, 220 in gold, 198 matched → 90% recall, 12 fabricated". A number with headroom, that says what to fix |
| `SIGNOFF-BURNDOWN.3` | `proposed` | drive the burn-down to done, reactivating `WIRE-BASED-100`'s owner-set sequence (AHB → AXI → SWD) with its no-fake-scoring rule intact |
| `SIGNOFF-BURNDOWN.4` | `proposed` | **the tree-opening rule**: a finding lands as a fact card by default. It becomes a *tree* only if it blocks the burn-down. This is the terminating condition the project currently lacks |

**Lane freeze (director's call — it changes direction, so it is not taken here).** `CORPUS-COVERAGE` and
`PDF-VARIANT-DIGESTION` are unbounded; 78 documents is already 15× the target set. The governance plane is
built and works — it caught four real omissions in a single slice on `2026-08-11` — and needs no further
trees. The `changes` ledger rollover is mandatory and must still happen; after it, freeze.

## What this predicts

If the diagnosis is right, `.1` alone will move AXI's verified fraction from ~2.6% to a real number and
immediately expose concrete defects, because the emitted AXI artifact contains 289 signals nobody has
checked. If `.1` lands and the number comes back near-perfect, the diagnosis is wrong and this tree closes.

That is the point: **the plan is falsifiable in one slice.**

## Critical path already known

`SIGNAL-CATALOG-CAPTURE-GAP.4` (the normalized-markdown `\_` escape that truncates identifiers) is **on**
the critical path: AXI carries 359 escaped statements against 980 declared signals, and AHB is affected too,
so the escape directly costs catalog recall on two targets. `SIGNAL-CATALOG-CAPTURE-GAP.5` (14 evidence
artifacts with no validation report) is **not** on it — by `.4` of this tree, it should stay a fact card.

## Non-goals

- Not a rewrite. The typed staged IR spine recovered a four-stage defect mechanism in one sitting on
  `2026-08-11`; it is not the problem.
- Not relaxing grounding, and not relaxing `WIRE-BASED-100`'s no-fake-scoring rule. Gold grows; the bar does not move.
- Not deleting the breadth or governance trees — suspending them, with their state intact.

## Open Questions

- Is five targets right, or should CHI (packet/flit, explicitly *not* wire-based) enter later as a second phase?
- Should the burn-down gate CI, or only report? `SIGNAL-CATALOG-CAPTURE-GAP.1` showed probe hits still need
  human reading; a reporting surface may be the honest first form.

## Blockers

- None technical. This needs one direction decision from the director: accept the target set and the lane
  freeze, or redirect.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-11` | — | ran all 11 eval seed datasets against the current release binary | every filtered WIRE-BASED-100 surface 1.000 except `seed_swd` signal_constraint (the known `CSYSPWRUPACK` residual) — the gate is saturated |
| `2026-08-11` | — | counted gold facts per target against emitted `.isf` construct counts | APB proportionate and done; AXI ~2.6% verified; AXI-Stream 0% (no gold at all) |
| `2026-08-11` | — | commit census by owning tree | `WIRE-BASED-100` 28 commits, last 2026-06-07; `CORPUS-COVERAGE` 103, active today |
| `2026-08-11` | — | emitted-output distribution across all 78 adapters | 44 renderable, 34 blocked; only 12 with ≥30 signals, only 15 with any transaction |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| — | `SIGNOFF-BURNDOWN — the gate is saturated, not missing; rescale it and burn it down` | diagnosis + plan only, no code, no accepted leaf |

## Changelog

- `2026-08-11`: created from a measured diagnosis of why delivery feels endless. Supersedes
  `MEASUREMENT-PLANE-CONVERGENCE-RISK`, which named the right symptom and the wrong cause. `proposed`; enters
  no frontier until the director accepts the target set and the lane freeze.
