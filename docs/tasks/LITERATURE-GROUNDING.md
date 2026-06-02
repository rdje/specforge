# LITERATURE-GROUNDING: ground every SpecForge aspect in published research

## Metadata

- Tree ID: `LITERATURE-GROUNDING`
- Status: `active` (`.1` design done; `.2`+ = the survey, a deliberate "later" program)
- Roadmap lane: `R0`/`R15e` (research / foundations)
- Created: `2026-06-01`
- Owner: repo-local workflow
- Parent context: user directive — SpecForge does many things, and each part/aspect may
  already be **extensively studied** in papers, articles, books, and academia. We must not
  reinvent the wheel where proven work exists, while still thinking out-of-the-box. The
  goal is to help SpecForge reach its full potential by standing on existing research.

## Goal

For each distinct SpecForge aspect, map "what SpecForge does" → "what the literature
already establishes", producing, per aspect:
1. **Prior art** — the authoritative published work (papers / articles / books / standards)
   that studies this problem, with the standard terminology and proven techniques.
2. **Alignment** — where SpecForge already matches or instantiates that work (validate the
   design; adopt the vocabulary).
3. **Adopt** — proven techniques SpecForge should borrow rather than re-derive.
4. **Extend / novelty** — where SpecForge genuinely goes beyond or recombines the
   literature (the out-of-the-box parts worth claiming).
5. **Gaps / opportunities** — where research suggests concrete improvements → each becomes
   a candidate future owned tree.

The deliverable is a `docs/research/grounding/` set (one doc per aspect) + a unifying map.

## Methodology (and the non-negotiable discipline)

- Mirror the proven approach of `INTENT-COMPLETENESS-RESEARCH` (R15e), which grounded the
  *completeness* aspect via a multi-agent research workflow that **verified the reframe and
  forced 5 corrections**. Use research agents (web search/fetch) per aspect to find and
  **verify** authoritative sources.
- **Anti-hallucination is mandatory.** Every cited source MUST be real and verifiable
  (venue, authors, year, and a resolvable identifier — DOI / arXiv id / ISBN / URL). A
  citation that cannot be verified is dropped, not guessed. (This is the single biggest
  risk of a literature-grounding effort and the reason the completeness program's
  9-agent workflow explicitly cross-checked references.)
- **Think-out-of-the-box is explicit, not implicit.** Each aspect doc must separate
  "settled in the literature → adopt" from "SpecForge's genuine novelty → keep/claim" so
  grounding does not flatten the project into only what already exists.
- Findings are advisory research, not code. Any technique SpecForge decides to adopt is a
  **separate, code-owning task-tree** downstream (per the doctrine).

## SpecForge aspects to ground (the survey backlog)

(Each is a `.2`+ leaf. The completeness aspect is ALREADY done — see Non-Goals.)

1. **Document structured extraction** — PDF layout analysis, table structure recognition,
   reading-order, figure/caption linkage (Docling-class document AI).
2. **Staged IR / progressive lowering** — multi-stage typed intermediate representations,
   compiler IR design, semantics-preserving lowering, provenance through stages.
3. **Requirements / specification intent extraction** — requirements engineering, NLP for
   requirements, specification mining, normative-language extraction.
4. **Protocol & temporal semantics** — handshake/protocol modeling, temporal logic
   (LTL/MTL), assertion/specification mining, FSM inference from descriptions.
5. **Knowledge-graph & relation extraction** — actor/signal/relation IE, ontology design,
   KG construction from technical text.
6. **Multimodal evidence fusion** — combining text + tables + figures + VLM observations;
   cross-modal agreement/conflict; evidence aggregation.
7. **Neuro-symbolic / bounded-LLM extraction** — LLM as a constrained hypothesis generator,
   constrained decoding, entailment/verification gates, hallucination mitigation,
   fail-closed design.
8. **Cross-document learning plane** — corpus-level prior accumulation, transfer/lifelong
   learning, weak supervision, knowledge-base bootstrapping.
9. **Extraction evaluation methodology** — precision/recall/F1 for IE, gold-set design,
   capture–recapture recall, inter-annotator/label quality. (Partly touched by the
   completeness recall-estimation work.)
10. **Uncertainty / residual-honesty** — open-world assumption, uncertainty
    quantification, abstaining/selective prediction, "known-unknown" representation.
11. **Spec → hardware lowering** — specification-to-RTL / property generation, the `.isf`
    adapter's place in spec-to-implementation toolchains.

## Non-Goals

- NOT re-doing the **completeness** aspect — `INTENT-COMPLETENESS-RESEARCH` (CLOSED) already
  grounded it (framework + closed-ontology coverage + capture–recapture [Chao Mh] +
  competency questions + KG completeness/LCWA), with verified references in
  `docs/research/` (incl. `literature-grounding.md`). This tree REFERENCES that and covers
  the remaining aspects + the unifying map.
- NOT adopting any technique here (that's downstream code-owning trees).
- NOT manufacturing citations — see the anti-hallucination discipline.

## Acceptance Criteria

- `.1` design owned (this file). Each aspect leaf: a `docs/research/grounding/<aspect>.md`
  with verified prior art + alignment + adopt + extend/novelty + gaps, every citation
  resolvable. Final synthesis leaf: a unifying map + a prioritized "reach-full-potential"
  backlog feeding future owned trees. Docs-only; mdBook/links consistent; tree CLOSED when
  the survey + synthesis are complete.

## Task Tree

- ID: `LITERATURE-GROUNDING`
  Status: `active`
  Children: `.1` (design) · `.2`–`.12` (one per aspect above) · `.13` (synthesis + backlog)

- ID: `LITERATURE-GROUNDING.1`
  Status: `done`
  Goal: own + design (this file) — scope, aspect enumeration, methodology + anti-
    hallucination discipline + out-of-the-box framing, output layout, reference the
    completeness grounding. Docs-only.
  Verification: passed (`2026-06-01`) — program scoped: 11 aspects enumerated from the real
    codebase, methodology fixed (verified-citations-only research workflow per the proven
    `INTENT-COMPLETENESS-RESEARCH` approach), out-of-the-box framing made explicit, the
    already-grounded completeness aspect referenced as the template + excluded. Registered.
  Commit: `see Commit Log`

- ID: `LITERATURE-GROUNDING.2`–`.12`
  Status: `pending` (the "later" survey — one aspect each, per the list above)
  Goal: per aspect, find + VERIFY authoritative published work; record
    `docs/research/grounding/<aspect>.md` (prior art / alignment / adopt / extend-novelty /
    gaps). No unverifiable citations.
  Acceptance: verified per-aspect grounding doc.

- ID: `LITERATURE-GROUNDING.13`
  Status: `pending`
  Goal: synthesize the unifying "SpecForge ↔ literature" map + a prioritized
    reach-full-potential backlog (adopt-now techniques, claimed novelty, research-suggested
    improvements → future owned trees); close.
  Acceptance: synthesis map + backlog; tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `LITERATURE-GROUNDING.1` | `done` | owned + designed (the program is captured) |
| 2 | `.2`–`.12` | `pending` | the deliberate "later" per-aspect survey (verified citations only) |
| 3 | `.13` | `pending` | synthesis + reach-full-potential backlog |

## Decisions

- `2026-06-01`: capture/own now (per doctrine), execute the survey "later" (user framed it
  as a future program) — this leaf fixes the scope + the verify-every-citation discipline
  so the later execution can't drift into fabricated references.
- `2026-06-01`: reuse the multi-agent research-workflow pattern from
  `INTENT-COMPLETENESS-RESEARCH`; keep adoption decisions in downstream code-owning trees.

## Open Questions

- Granularity: some aspects (e.g. temporal semantics, protocol modeling) may split into
  finer leaves once their literature breadth is scoped in `.2`+.

## Blockers

- None (research/design). The per-aspect survey leaves want web research access when run.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | program owned + scoped (11 aspects, methodology + anti-hallucination + out-of-the-box framing); completeness grounding referenced + excluded; docs-only | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `LITERATURE-GROUNDING.1` | `LITERATURE-GROUNDING.1 — own + design the research-grounding program for all SpecForge aspects` | docs-only |

## Changelog

- `2026-06-01`: Created — own + design the program to ground every SpecForge aspect in
  verified published research (leverage prior art, claim genuine novelty, surface
  improvement opportunities); survey execution deliberately deferred ("later").
