# SIGNOFF-BURNDOWN: WITHDRAWN — this tree was archaeology

## Metadata

- Tree ID: `SIGNOFF-BURNDOWN`
- Status: `superseded` — routed to [`KG-ISF-COMPLETENESS`](KG-ISF-COMPLETENESS.md) `.2a`/`.4`, which
  already own everything this tree proposed
- Created / withdrawn: `2026-08-11` (same day)
- Also supersedes: `MEASUREMENT-PLANE-CONVERGENCE-RISK` (withdrawn on the same grounds)

## What happened

Asked whether SpecForge was converging, I built a diagnosis from direct measurement of the artifacts and
proposed a recovery plan. The measurements were real. **The conclusions were re-derivations of facts the
project had already established, recorded, and decided on** — which is precisely the *archaeology* the
bootstrap (`AGENTS.md`, `CLAUDE.md`, `knowledge-map/KNOWLEDGE_MAP_ARCHITECTURE.md`) instructs an agent to
avoid by grepping `KNOWLEDGE_MAP.md` **before** re-deriving anything from code or runtime.

I grepped the Knowledge Map once, for signal-declaration facts, at the start of an unrelated leaf. I never
grepped it again as I moved into architecture and strategy. Every headline below was already there.

| What I presented as a finding | Where it already lived |
| --- | --- |
| "`.isf` emits `(width 1)` everywhere; the adapter defaults" | `KG-ISF-COMPLETENESS.2a`: *"the emitter reads only the legacy flat `direction_hint`/`width_hint` (None for 85–98%) and defaults to `output`/`width-1` (AXI `.isf` = 283 `(output)` vs 4 `(input)`, all width 1)"* — same defect, same AXI numbers, measured first |
| "parameterised widths are 100% lost; one bounded feature" | `KG-ISF-COMPLETENESS.2a`: *"width is mostly symbolic (`*_WIDTH`) → needs an FSMGen-contract check on `(width PARAM)`"* — already identified, **plus** the blocking question I never asked: does `--strict --check` even accept a symbolic width? |
| "behaviour never lowers; `consequent_action` is a placeholder — the real frontier" | Knowledge Map: *"the only improvement path for conditional rules — upstream EXTRACTION recovering the concrete obligation from the conditional's `source_text` … lower-leverage than register/structure/topology"* — analysed **and prioritised**, with a verdict opposite to mine |
| "the project has no instrument and no gradient" | `.2a` is `deferred` **with-trigger**, blocked on an FSMGen-contract check, a reference-boundary design, and an **OWNER decision**, because carrying direction/width is in tension with the `2026-06-16` north star (FSMGen owns scheduling) |

That last row is the one that matters. I reported a project drifting without direction. What the record
actually shows is a deliberate, measured, owner-gated **deferral** awaiting a decision from the director.
Those are opposite diagnoses, and I asserted the wrong one with confidence.

## Also wrong, in the same session

- Claimed a corpus document was a truncated ingest on a `/Type/Page` byte-regex (125 pages) — `pdfinfo` says
  the source PDF genuinely is 6 pages.
- Published "APB inventory recall 81.1%" — the real figure is **32/32, zero fabricated**. The error counted
  parameter rows as missing signals and missed `PCLK`/`PRESETN` because they emit as `(clock …)`/`(reset …)`.
- Proposed scaling catalog gold as the fix, then invalidated it hours later; APB already has every name.

Three measurement errors and one inverted diagnosis, all found by pushing one level further. Recorded here
because the pattern — assert, get challenged, measure, discover the assertion was wrong — is the thing to
distrust, not any single number.

## What survives, and where it goes

One artifact may still be worth something to whoever picks up `.2a`, offered **without any claim of
novelty** (`declared_signal_complete_gold_precision` and the I²C complete-gold surface already exist, and
this may duplicate them — verify before use):

`.project-data/tmp/signal-catalog-capture-gap-1/pdf_to_isf_fidelity.py` scores an emitted `.isf` against the
document's own inventory table — identity column plus Width column, rows with width `-` excluded as
parameter declarations, `(clock …)`/`(reset …)` counted as emitted. No gold, no labelling. Corrected run,
`2026-08-11`:

| Spec | stated | emitted | recall | fabricated | literal widths | parameterised widths lost |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| AMBA APB | 32 | 32 | 100.0% | 0 | 17/17 | 15/15 |
| AMBA AXI | 324 | 289 | 88.9% | 1 | 210/210 | 78/78 |
| AMBA AHB | 24 | 41 | 87.5% | **20** | 10/10 | 11/11 |
| AMBA AXI-Stream | 25 | 22 | 84.0% | 1 | 9/10 | 11/11 |
| AMBA LTI | 93 | 69 | 65.6% | 8 | 26/31 | 30/30 |

The parameterised-width column restates `.2a`'s known defect. The **fabrication** column (AHB 20, LTI 8) is
the only cell I did not find already recorded, and given this session's record that should be treated as
"unverified, probably known" rather than as a finding. It belongs to `KG-ISF-COMPLETENESS`, not here.

## Blockers

- None. This tree is withdrawn and enters no frontier. It is kept rather than deleted so the failure mode is
  legible to the next session: **grep the Knowledge Map before forming a diagnosis, not just before touching
  code.**

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| — | `SIGNOFF-BURNDOWN — the gate is saturated, not missing; rescale it and burn it down` | the withdrawn original |
| — | `SIGNOFF-BURNDOWN — withdraw; the diagnosis was archaeology over KG-ISF-COMPLETENESS.2a` | this withdrawal |

## Changelog

- `2026-08-11`: created, corrected once, then withdrawn the same day. Its content duplicated
  `KG-ISF-COMPLETENESS.2a` (deferred, owner-gated) and a Knowledge Map verdict on conditional-rule
  leverage. No code was written and no roadmap surface was edited on its premises.
