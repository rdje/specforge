# REEXTRACTION-REMEASURE: re-ingest + re-extract the APB spec to measure current quality

## Metadata

- Tree ID: `REEXTRACTION-REMEASURE`
- Status: `done` (CLOSED `2026-06-05`; `.1`)
- Roadmap lane: `R16`/`R15e` (extraction quality, real-data validation)
- Created: `2026-06-05`
- Parent context: owner directive ("do all 5 bullets") — **item 3 of 5**.

## What was done

Re-ingested the real AMBA APB spec (`IHI0024_E_2023-02`) through docling (`DOCLING_DEVICE=cpu`,
regenerating the cleaned-up normalized markdown), rebuilt the EvidenceIR with the **current**
extraction code, and re-ran `nli-verify` (qwen2.5:14b) — to measure whether this session's
extraction-quality work actually landed on real data (the prior measurement was on a stale artifact).

## Result — the session's fixes are validated

| metric | stale artifact | fresh (current code) |
| --- | --- | --- |
| `signal_constraints` | **42** | **16** |
| NLI not-entailed | 36 → 33 | 11 |
| `actor_signal_relations` | (n/a) | **69** (reads 39 / drives 30) |

The **gross constraint over-generation is resolved** (42 → 16): the de-hardcoded positional value
path (PDF-AGNOSTIC-EXTRACTION.2), the condition-clause subject fix (CONSTRAINT-CONDITION-SUBJECT),
and the existing width-param/cross-sentence precision filters removed the spurious constraints the
NLI run had flagged. The corpus-mined relation verbs (VERB-COVERAGE-CORPUS) now recover **69**
actor→signal KG edges.

## Remaining-work map (the 11 still NLI-flagged → next extraction-quality frontier)

Reading the fresh flags gives **three concrete, real bug classes** (NOT stale):

1. **Value-as-subject in non-`"must be"` constructions** — `"PSLVERR is driven LOW when …"` and
   `"tied LOW"` yield `"LOW must be stable"` (the value `LOW` extracted as the subject). The
   positional value exclusion handles `"must be LOW"` but not `tied`/`driven <value>`.
2. **Table-cell claim text not split per obligation** — multi-bullet cells
   (`PAUSER must be valid when …  • … same value …  • …`) become one malformed claim carrying the
   whole cell; the subject is right but the claim text needs per-bullet/sentence splitting.
3. **`"which means"` condition mis-attribution** — `"PSEL is asserted, which means PADDR/PWRITE/
   PWDATA must be valid"` yields `"PSEL must be VALID"` (PSEL is the trigger, not the subject) — a
   construction the `when`-clause fix does not yet cover.

→ Recommend a follow-up tree `CONSTRAINT-EXTRACTION-V2` for (1)–(3). Not fixed here (this leaf is the
*measurement*, per the bullet; fixes are new owning leaves).

## Verification

Done (`2026-06-05`) — ingest exit 0 (markdown regenerated), evidence rebuild exit 0, `nli-verify`
exit 0; counts above captured from the fresh artifacts. (Generated artifacts are gitignored.)

## Task Tree

- ID: `REEXTRACTION-REMEASURE` · Status: `done` · Children: `.1`
- ID: `REEXTRACTION-REMEASURE.1` · Status: `done` · Goal: re-ingest + re-extract + re-measure APB;
  report deltas + remaining issues. Verification above.

## Changelog

- `2026-06-05`: Created + CLOSED — re-measured APB on current code (constraints 42→16, 69 relations);
  identified 3 remaining bug classes for a `CONSTRAINT-EXTRACTION-V2` follow-up. (Owner "do all 5" —
  item 3/5.)
