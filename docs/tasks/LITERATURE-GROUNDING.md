# LITERATURE-GROUNDING: ground every SpecForge aspect in published research

## Metadata

- Tree ID: `LITERATURE-GROUNDING`
- Status: `done` (CLOSED `2026-06-02` — all 11 aspects grounded with verified citations +
  synthesis map + reach-full-potential backlog + book mirror)
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

- ID: `LITERATURE-GROUNDING.2`
  Status: `done`
  Goal: ground aspect 1 — **document structured extraction**.
  Verification: passed (`2026-06-01`) — `docs/research/grounding/document-extraction.md`
    written with **web-verified** citations only: Docling (arXiv:2408.09869, :2501.17887),
    TableFormer (arXiv:2203.01017), DocLayNet (arXiv:2206.01062), PubTables-1M
    (arXiv:2110.00061); TEDS/GriTS metrics. Alignment (SpecForge's ingest = this SOTA
    stack), adopt (TEDS/GriTS metrics; PubTables-1M functional analysis for table-role;
    canonicalization vs oversegmentation), extend/novelty (protocol-intent lift above the
    structured doc — no doc-AI paper does it), and 3 gaps → candidate future trees. Every
    citation resolvable; none guessed.
  Commit: `see Commit Log`

- ID: `LITERATURE-GROUNDING.3`
  Status: `done`
  Goal: ground aspect 4 — **protocol & temporal semantics**.
  Verification: passed (`2026-06-01`) — `docs/research/grounding/protocol-temporal-semantics.md`
    with web-verified citations: Pnueli LTL (FOCS 1977, 10.1109/SFCS.1977.32), Ammons et al.
    *Mining Specifications* (POPL 2002, 10.1145/503272.503275), Daikon (Ernst et al.,
    ICSE 1999/TSE 2001), Texada *General LTL Specification Mining* (Lemieux et al., ASE
    2015), GoldMine (Vasudevan et al., DATE 2010), W. Li thesis (Berkeley EECS-2014-20),
    *Mining Secure Behavior of HW Designs* (arXiv:2108.09249). Key insight: SpecForge's
    `temporal_rules` ARE the `G(antecedent→consequent)`-with-holes LTL-template shape the
    spec-mining literature formalized; **novelty = mining intent from the spec (forward),
    not from traces/RTL (backward), fail-closed.** Adopt: LTL/MTL templates + MTL for
    cycle_window + temporal-rule eval. Every citation resolvable.
  Commit: `see Commit Log`

- ID: `LITERATURE-GROUNDING.4`–`.12`
  Status: `done`
  Goal: per aspect, find + VERIFY authoritative published work; record
    `docs/research/grounding/<aspect>.md`. No unverifiable citations.
  Acceptance: verified per-aspect grounding doc.
  Verification: passed (`2026-06-02`) — all 9 remaining aspects grounded via a 9-agent
    parallel research sweep, every citation web-verified with a resolvable id (none guessed),
    and the highest-fabrication-risk recent works personally spot-verified by the
    orchestrator before commit (AssertLLM arXiv:2402.00386/ASP-DAC'25; Hybrid-NL2SVA
    arXiv:2506.21569/MLCAD'25; QiMeng-CodeV-SVA arXiv:2603.14239; MLLM-VRDU survey
    arXiv:2507.09861 — all confirmed real). One unconfirmed secondary survey (arXiv:2408.01287)
    was DROPPED per the anti-hallucination discipline. Docs written:
      - `.4`  aspect 2  → `staged-ir.md` (LLVM CGO'04, MLIR CGO'21/2002.11054, nanopass ICFP'04/'13)
      - `.5`  aspect 3  → `requirements-extraction.md` (RFC 2119/8174, NLP4RE CSUR'21/2004.01099,
        PROMISE-NFR'07, NoRBERT RE'20, ACE cmp-lg/9603004, Berry-Kamsties, ARM ICSE'97)
      - `.6`  aspect 5  → `knowledge-graph-relation-extraction.md` (OpenIE IJCAI'07, ReVerb
        D11-1142, Mintz P09-1113, PCNN D15-1203, KBP P11-1115, OIE survey C18-1326, Hogan KG CSUR'21)
      - `.7`  aspect 6  → `multimodal-fusion.md` (LayoutLM/v2/v3, Donut 2111.15664, DocVQA
        2007.00398, MLLM-VRDU 2507.09861, Dempster 1967)
      - `.8`  aspect 7  → `neuro-symbolic-bounded-llm.md` (GCD 2305.13971, Outlines 2307.09702,
        RAG 2005.11401, SNLI 1508.05326, Ji hallucination CSUR'23, SelfCheckGPT 2303.08896,
        Garcez&Lamb neurosymbolic)
      - `.9`  aspect 8  → `cross-document-learning.md` (Yarowsky'95, Riloff-Jones'99,
        Mintz P09-1113, NELL AAAI'10, Snorkel 1711.10160, Parisi continual 1802.07569)
      - `.10` aspect 9  → `extraction-evaluation.md` (van Rijsbergen'79, MUC-5'93, Cohen κ'60,
        Krippendorff α, Artstein-Poesio CL'08, Chao'87, Eick ICSE'92, Petersson JSS'04)
      - `.11` aspect 10 → `uncertainty-residual-honesty.md` (Chow'70, El-Yaniv-Wiener JMLR'10,
        Geifman 1705.08500, Vovk conformal'05, Angelopoulos-Bates 2107.07511, Guo 1706.04599,
        Scheirer open-set TPAMI'13)
      - `.12` aspect 11 → `spec-to-hardware.md` (IEEE 1850 PSL, IEEE 1800 SVA, AssertLLM
        2402.00386, Hybrid-NL2SVA 2506.21569, QiMeng-CodeV-SVA 2603.14239, HLS Cong TCAD'11)
    Each doc separates adopt-from-literature vs SpecForge's claimed novelty, per the
    out-of-the-box discipline.

- ID: `LITERATURE-GROUNDING.13`
  Status: `done`
  Goal: synthesize the unifying "SpecForge ↔ literature" map + a prioritized
    reach-full-potential backlog (adopt-now techniques, claimed novelty, research-suggested
    improvements → future owned trees); close.
  Acceptance: synthesis map + backlog; tree CLOSED.
  Verification: passed (`2026-06-02`) — `docs/research/grounding/README.md` written: the
    11-aspect → literature map (anchor / alignment / claimed-novelty per aspect), the
    cross-aspect novelty throughline (forward spec→intent recovery; fail-closed typed
    residuals; capture–recapture recall bounds; closed behavioral protocol ontology;
    advisory-only + negative-knowledge priors), and a 3-tier prioritized reach-full-potential
    backlog (Tier 1 adopt-now: LTL/MTL templates + `.isf`→PSL/SVA export, per-relation +
    temporal-rule eval, conformal calibration; Tier 2 principled replacements: NLI verifier,
    Dempster-rule fusion, κ/α + Chao + partial-match, TEDS/GriTS + functional analysis;
    Tier 3 structural: nanopass differential grammars + stage verifier, NELL/Snorkel prior
    hardening, ambiguity classifier + SHOULD/MAY, cross-sentence RE + canonicalization) —
    each a candidate future owned tree. The earlier `INTENT-COMPLETENESS-RESEARCH` grounding
    is referenced, not repeated. **BOOK-METHOD-DOC close-rule satisfied**: added a
    user-friendly "Why the design is grounded in published research" section to
    `docs/book/src/architecture-rationale.md` (why-before-what; the forward/residual/recall
    throughline; the verify-every-citation rule as a user-trust benefit; pointer to the
    survey). mdBook builds green. Docs-only.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `LITERATURE-GROUNDING.1` | `done` | owned + designed (the program is captured) |
| 2 | `LITERATURE-GROUNDING.2` | `done` | aspect 1 grounded — document extraction (verified citations) |
| 3 | `LITERATURE-GROUNDING.3` | `done` | aspect 4 grounded — protocol & temporal semantics (verified) |
| 4 | `.4`–`.12` | `done` | 9 remaining aspects grounded (verified citations only; 9-agent sweep + spot-verify) |
| 5 | `.13` | `done` | synthesis map + reach-full-potential backlog + book mirror → **tree CLOSED `2026-06-02`** |

**Tree CLOSED `2026-06-02`.** All 11 SpecForge aspects grounded in verified published
research (`.2`–`.12`), the synthesis map + prioritized reach-full-potential backlog landed
(`.13`, `docs/research/grounding/README.md`), and the book mirror added
(`architecture-rationale.md`). The backlog's Tier-1 items are candidate future owned trees.

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
| `2026-06-01` | `.2` | aspect 1 (document extraction) grounded with web-verified citations (Docling 2408.09869/2501.17887, TableFormer 2203.01017, DocLayNet 2206.01062, PubTables-1M 2110.00061; TEDS/GriTS); alignment/adopt/extend/gaps; `docs/research/grounding/document-extraction.md` | `passed` |
| `2026-06-01` | `.3` | aspect 4 (protocol & temporal semantics) grounded with web-verified citations (Pnueli FOCS'77, Ammons POPL'02, Daikon, Texada ASE'15, GoldMine DATE'10, Li EECS-2014-20, arXiv:2108.09249); SpecForge temporal_rules = LTL G(ante→cons) templates; novelty = spec→property (not trace→property); `docs/research/grounding/protocol-temporal-semantics.md` | `passed` |
| `2026-06-02` | `.4`–`.12` | 9 remaining aspects grounded via 9-agent parallel research sweep, verified-citations-only; orchestrator spot-verified the recent high-risk works (AssertLLM 2402.00386, Hybrid-NL2SVA 2506.21569, QiMeng-CodeV-SVA 2603.14239, MLLM-VRDU 2507.09861 — all confirmed) and DROPPED one unverified survey (2408.01287); 9 docs written under `docs/research/grounding/` (staged-ir, requirements-extraction, knowledge-graph-relation-extraction, multimodal-fusion, neuro-symbolic-bounded-llm, cross-document-learning, extraction-evaluation, uncertainty-residual-honesty, spec-to-hardware), each with adopt-vs-novelty separation | `passed` |
| `2026-06-02` | `.13` | synthesis map + reach-full-potential backlog written (`docs/research/grounding/README.md`: 11-aspect→literature table, cross-aspect novelty throughline, 3-tier prioritized backlog → candidate future trees); BOOK-METHOD-DOC close-rule satisfied (user-friendly "Why the design is grounded in published research" section in `architecture-rationale.md`); mdBook builds green; **tree CLOSED** | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `LITERATURE-GROUNDING.1` | `LITERATURE-GROUNDING.1 — own + design the research-grounding program for all SpecForge aspects` | docs-only |
| `LITERATURE-GROUNDING.2` | `LITERATURE-GROUNDING.2 — ground aspect 1 (document structured extraction) with verified citations` | docs-only; verified |
| `LITERATURE-GROUNDING.3` | `LITERATURE-GROUNDING.3 — ground aspect 4 (protocol & temporal semantics) with verified citations` | docs-only; verified |
| `LITERATURE-GROUNDING.4`–`.12` | `LITERATURE-GROUNDING.4-.12 — ground 9 remaining aspects with verified citations (9-agent sweep + spot-verify)` (`2a9eeecc`) | docs-only; verified; 9 grounding docs |
| `LITERATURE-GROUNDING.13` | `LITERATURE-GROUNDING.13 — synthesis map + reach-full-potential backlog + book mirror; close tree` | docs-only; synthesis README + architecture-rationale book section; tree CLOSED |

## Changelog

- `2026-06-01`: Created — own + design the program to ground every SpecForge aspect in
  verified published research (leverage prior art, claim genuine novelty, surface
  improvement opportunities); survey execution deliberately deferred ("later").
- `2026-06-02`: **Tree CLOSED.** Executed the full survey under a "roll until exhaustion"
  PNT directive: `.4`–`.12` grounded the 9 remaining aspects via a 9-agent parallel research
  sweep (verified-citations-only; one unverifiable survey dropped; commit `2a9eeecc`), and
  `.13` synthesized the unifying SpecForge↔literature map + a 3-tier prioritized
  reach-full-potential backlog (`docs/research/grounding/README.md`) and added the
  user-friendly book mirror to `architecture-rationale.md` (BOOK-METHOD-DOC close-rule).
  All 11 aspects done; mdBook green. The backlog's Tier-1 items (LTL/MTL templates +
  `.isf`→PSL/SVA export; per-relation + temporal-rule eval; conformal LLM-tier calibration)
  are the highest-leverage candidate future owned trees.
