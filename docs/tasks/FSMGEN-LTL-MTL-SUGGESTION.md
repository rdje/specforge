# FSMGEN-LTL-MTL-SUGGESTION: suggest first-class LTL/MTL temporal properties in ISF

## Metadata

- Tree ID: `FSMGEN-LTL-MTL-SUGGESTION`
- Status: `done` (CLOSED `2026-06-04` — suggestion + proposed ISF format filed in the FSMGen
  feedback channel; SpecForge's action is complete, the decision is FSMGen's)
- Roadmap lane: `R6` (FSMGen handoff / temporal semantics)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: **user directive (2026-06-04)** — *"You can also suggest FSMGEN to add full
  support for LTL/MTL in ISF. You know which document to amend for that."* + *"You can also
  suggest the format in ISF of those LTL/MTL."* + *"add a KM card to find this feedback file."*

## Goal

File, through the tracked SpecForge→FSMGen feedback channel (`docs/FSMGEN_FEEDBACK.md`), a
**feature suggestion** that ISF gain a first-class **LTL/MTL temporal-property** construct — the
full `G(antecedent → [X | F[min,max]] consequent)` template — so SpecForge can lower its mined
`temporal_rules` *directly into ISF* (the FSMGen-native alternative to a SpecForge-side SVA
export). Include a **concrete proposed ISF format** (semantic shape; exact syntax FSMGen's), and
add a Knowledge Map card so the feedback channel is findable.

## Why this, and why now

SpecForge's `temporal_rules` are already LTL/MTL by construction (`G(ante→cons)`, `X`, MTL
`F[min,max]`; ADR-0005, `ir/temporal_ltl.rs`). ISF already carries the special case
`(contract <n> (eventually <signal> (within <N>)))` = an MTL bounded-eventually with an empty
antecedent. Generalizing it to the full template would close the spec→checkable-property loop
**inside** the existing `IntentIR → .isf → FSMGEN` handoff — if FSMGen adopts it, the deferred
SpecForge-side SVA export (`TEMPORAL-RULE-SVA-RENDER`) may be unnecessary. This is a
**suggestion, not a bug** (no SpecForge `.isf` is broken; SpecForge does not emit temporal
rules into ISF today), so it uses `FSMGEN_FEEDBACK.md` (the suggestion channel), not the
issue-bundle bug protocol.

## Non-Goals

- NOT emitting temporal rules into ISF now (SpecForge has nothing to change until/unless FSMGen
  adopts the construct — then it is a new owned tree).
- NOT a bug report / issue bundle (this is a feature suggestion).
- NOT deciding SpecForge-SVA-export vs FSMGen-native — that decision stays open with the user
  (`TEMPORAL-RULE-SVA-RENDER` is the logged SpecForge-side alternative).

## Acceptance Criteria

- A dated suggestion section added to `docs/FSMGEN_FEEDBACK.md` with the ask + a concrete
  proposed ISF shape that maps 1:1 onto `TemporalRuleRecord` + the LTL/MTL template; framed as a
  suggestion (not a bug); cross-referencing §4, the existing `eventually` contract, ADR-0005,
  and the deferred `TEMPORAL-RULE-SVA-RENDER`. A KM card (`fsmgen-feedback-channel`) makes the
  channel findable. mdBook + KM gate green. Tree CLOSED (SpecForge's action complete).

## Task Tree

- ID: `FSMGEN-LTL-MTL-SUGGESTION`
  Status: `done`
  Goal: file the suggestion + proposed ISF format in `docs/FSMGEN_FEEDBACK.md`; add the KM card.
  Acceptance: as above.
  Verification: passed (`2026-06-04`) — added "## Suggestion (2026-06-04) — first-class LTL/MTL
    temporal properties in ISF" to `docs/FSMGEN_FEEDBACK.md`: the ask (generalize the existing
    `(eventually s (within N))` to the full `G(antecedent → X|F[min,max] consequent)` template),
    a concrete proposed ISF shape (`(temporal-rule … (clock …(edge …)) (antecedent <pred>…)
    (consequent (window min max) <pred>…))` with `pred = value|stable|handshake`) mapping 1:1
    onto `TemporalRuleRecord`, the rationale (close the loop inside the handoff; ADR-0005:
    mine-don't-check), the explicit exclusion of drive/sample predicates, and the suggestion-not-
    bug framing + the deferred-SVA-export alternative. KM card `fsmgen-feedback-channel` added.
    No code change. Tree CLOSED — the ball is in FSMGen's court (a future response → a follow-up
    tree).

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `FSMGEN-LTL-MTL-SUGGESTION` | `done` | suggestion + proposed ISF format filed; KM card added; **CLOSED** |

**Tree CLOSED `2026-06-04`.** The suggestion is filed in the SpecForge→FSMGen channel; the
decision (adopt the construct? in what syntax?) is FSMGen's. If FSMGen adds it, lowering
SpecForge's `temporal_rules` into ISF becomes a new owned tree (and may retire the deferred
SpecForge-side SVA export).

## Decisions

- `2026-06-04`: use `FSMGEN_FEEDBACK.md` (suggestion channel), not the issue-bundle protocol
  (which is for reproducible bugs). Close on *filing* (SpecForge's part), like
  `FSMGEN-ISSUE-REPORTING`. Keep the SpecForge-SVA-export-vs-FSMGen-native decision open with
  the user.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-04` | `.1` | suggestion + proposed ISF format (1:1 with `TemporalRuleRecord`) filed in `docs/FSMGEN_FEEDBACK.md`; suggestion-not-bug framing; KM card `fsmgen-feedback-channel`; cross-links the deferred `TEMPORAL-RULE-SVA-RENDER`; no code change; CLOSED | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `FSMGEN-LTL-MTL-SUGGESTION` | `FSMGEN-LTL-MTL-SUGGESTION — suggest LTL/MTL-in-ISF + proposed format to FSMGen; log SVA export as deferred; close` | docs-only; suggestion filed |

## Changelog

- `2026-06-04`: Created + CLOSED (user directive) — filed a suggestion (with a concrete proposed
  ISF format) to FSMGen to add first-class LTL/MTL temporal properties to ISF, so SpecForge could
  lower `temporal_rules` directly into ISF; the SpecForge-side SVA export (`TEMPORAL-RULE-SVA-RENDER`)
  is the deferred alternative. KM card `fsmgen-feedback-channel` added.
