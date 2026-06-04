# SPEC-MINING-PROVENANCE: name the discipline + a per-author adopt/defer ledger

## Metadata

- Tree ID: `SPEC-MINING-PROVENANCE`
- Status: `active` (`.1` design done; `.2` = temporal trio + framing; `.3`+ = the rest)
- Roadmap lane: `R0`/`R15e` (research grounding / live-doc continuity)
- Created: `2026-06-04`
- Owner: repo-local workflow
- Parent context: **user directive (2026-06-04, two messages).** (1) *"They use the expression
  'specification mining', which describes what SpecForge does when extracting intent from
  chip-spec PDFs."* (2) *"SpecForge can benefit a lot from Pnueli/GoldMine/Texada — their
  works are exactly the abstractions SpecForge needs. Create KM cards, thoroughly document in
  the book / live docs / task-trees what SpecForge TAKES from those guys, what it LEAVES OUT
  for now and WHY. Do the same for the other authors found during the literature sweep."*

## Goal

Two complementary outputs, both grounded and retrievable:

1. **Name the discipline.** Adopt **"specification mining"** as SpecForge's self-description
   across README / book / live-docs — with the forward-direction novelty explicit: classic
   spec mining recovers a spec *from an implementation* (RTL, traces, code); SpecForge mines
   intent *from the human-authored specification itself*, before any implementation exists
   (**forward specification mining**). Coined by Ammons, Bodík & Larus, *Mining Specifications*,
   POPL 2002 (DOI 10.1145/503272.503275).
2. **A per-author adopt/defer provenance ledger.** For each leveraged author/work from the
   literature sweep, a crisp record with three fields:
   - **Take** — the abstraction/technique SpecForge adopts.
   - **Leave out (for now) + why** — what of their work SpecForge does *not* use, and the
     reason (so it is a recorded decision, not an oversight a future agent re-litigates).
   - **Instantiated at** — where in SpecForge's code/IR/docs the adopted idea lives.
   Recorded in a ledger doc (`docs/research/grounding/adopt-defer-ledger.md`), surfaced as
   Knowledge Map cards for the high-leverage authors, and grounded in the book.

This sharpens the existing per-*aspect* grounding (`docs/research/grounding/*.md`) into
per-*author* provenance, and makes "what do we take from GoldMine?" a one-lookup answer.

## Discipline (carried over from LITERATURE-GROUNDING)

- **Every citation real + verifiable** (resolvable id); reuse the already-verified citations
  from `docs/research/grounding/*.md`, never invent.
- **Honest about what we leave out.** The "leave out + why" field is mandatory — the value is
  in the deliberate boundary, not just the borrowing.
- Advisory documentation, not code (no extraction change); any technique we later decide to
  adopt is a separate code-owning tree.

## Prioritization

`.2` does the **temporal trio** first (the user's flagged priority + SpecForge's most direct
leverage — the `G(antecedent → consequent)` LTL template our `temporal_rules` and
`ir/temporal_ltl.rs` instantiate) **and** the "specification mining" framing. `.3`+ extend the
ledger to the remaining swept authors (Ammons/Daikon; Docling/TableFormer/DocLayNet/PubTables-1M;
OpenIE/ReVerb/Mintz/Hogan; LayoutLM/Donut/Dempster; GCD/Outlines/RAG/SNLI/SelfCheckGPT/Garcez-Lamb;
Yarowsky/Riloff-Jones/NELL/Snorkel; van Rijsbergen/MUC/Cohen/Chao; Chow/El-Yaniv/Vovk/Guo/Scheirer;
IEEE PSL/SVA/AssertLLM/HLS), then a synthesis + close.

## Acceptance Criteria

- `.1` design owned (this file), registered in `docs/TASK_TREE.md`.
- `.2`: `adopt-defer-ledger.md` created with the temporal-trio entries (Pnueli / GoldMine /
  Texada — Take / Leave-out+why / Instantiated-at, verified citations); the "specification
  mining" framing adopted in `README.md` + `architecture-rationale.md` (+ live-docs as
  apt); KM card(s) for the framing + temporal-trio provenance; mdBook + KM gate green; CI green.
- `.3`+: the remaining authors folded into the ledger (batched), each with the three fields +
  verified citation; a final synthesis; tree CLOSED.

## Task Tree

- ID: `SPEC-MINING-PROVENANCE`
  Status: `active`
  Children: `.1` (design) · `.2` (temporal trio + framing) · `.3`+ (rest of the authors) ·
    `.N` (synthesis + close)

- ID: `SPEC-MINING-PROVENANCE.1`
  Status: `done`
  Goal: own + design (this file) — the two outputs (framing + per-author ledger), the
    three-field ledger format, the verify-every-citation discipline, the prioritization
    (temporal trio + framing first). Docs-only.
  Acceptance: design recorded; registered.
  Verification: passed (`2026-06-04`) — scoped from the user's two messages; ledger format
    fixed (Take / Leave-out+why / Instantiated-at); "specification mining" framing + the
    forward-vs-backward novelty articulated with the verified Ammons POPL'02 citation; reuses
    the verified grounding citations; prioritization set (temporal trio + framing in `.2`).
  Commit: `see Commit Log`

- ID: `SPEC-MINING-PROVENANCE.2`
  Status: `pending`
  Goal: temporal-trio ledger entries (Pnueli / GoldMine / Texada) + adopt the "specification
    mining" framing (README + architecture-rationale) + KM card(s); verify (mdBook + KM gate +
    CI green).
  Acceptance: ledger temporal-trio entries; framing adopted; KM card(s); CI green.

- ID: `SPEC-MINING-PROVENANCE.3`
  Status: `pending`
  Goal: extend the ledger to the remaining swept authors (batched), verified citations; then
    synthesize + CLOSE.
  Acceptance: full ledger; synthesis; tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `SPEC-MINING-PROVENANCE.1` | `done` | owned + designed (format + framing + priorities) |
| 2 | `SPEC-MINING-PROVENANCE.2` | `pending` | temporal trio + "specification mining" framing + KM card(s) |
| 3 | `SPEC-MINING-PROVENANCE.3` | `pending` | remaining authors → ledger; synthesis; close |

## Decisions

- `2026-06-04`: per-*author* provenance (Take / Leave-out+why / Instantiated-at) on top of the
  per-*aspect* grounding — the "leave out + why" boundary is the load-bearing part. Adopt
  "specification mining" as the discipline name with the forward-vs-backward distinction
  explicit. Reuse verified citations; no new code (advisory).

## Blockers

- None. Docs + framing only.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-04` | `.1` | program scoped from the user's directive; 3-field ledger format + spec-mining framing (forward vs backward, Ammons POPL'02 verified) fixed; prioritization set; docs-only | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SPEC-MINING-PROVENANCE.1` | `SPEC-MINING-PROVENANCE.1 — own + design the spec-mining framing + per-author adopt/defer ledger` | docs-only |

## Changelog

- `2026-06-04`: Created (user directive) — name SpecForge's discipline **"specification
  mining"** (forward: spec→intent, vs the literature's backward implementation→spec) and build
  a per-author **adopt/defer provenance ledger** (Take / Leave-out+why / Instantiated-at) in
  KM + book + a ledger doc, starting with the temporal trio (Pnueli/GoldMine/Texada).
