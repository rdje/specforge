---
id: escaped-identifier-fragments-the-catalog
title: A Markdown backslash-escape fragments an identifier, the fragment is DECLARED as a signal, and its own synthesized declaration then defeats every standalone-wins gate
answers:
  - "why does eMMC declare a signal called PARTITION"
  - "where does the backslash underscore in EvidenceIR text come from"
  - "does SourceIR carry the markdown escape"
  - "can the escaped identifier problem be fixed by re-ingesting"
  - "why is standalone wins circular for an escaped identifier fragment"
  - "how many documents declare a fragment of an escaped compound"
  - "how many constraint records have an escaped-fragment subject"
  - "why was EXTRACTION-QUALITY-GAUGE.3k.9 not shipped"
  - "which census measures escaped identifier fragments"
  - "what breaks if the identifier tokenizer unescapes markdown"
date: 2026-09-13
status: current
evidence: scripts/measure_escaped_identifier_fragments.py (--self-test 6/6); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3k.9); crates/specforge/src/ir/evidence.rs (collect_subject_signal_tokens, is_hardware_signal_token)
reverify: "python3 scripts/measure_escaped_identifier_fragments.py — three populations, and `--self-test` for the circularity case"
tags: [evidence-ir, constraint-extraction, source-ir, extraction-quality-gauge, claim-verification]
---

Docling renders an underscore inside an identifier as the Markdown escape `\_`. That is **correct
Markdown**, and it is not SpecForge's doing. The consequences are, and they are not where you would
look for them.

| stage | carries `\_`? | why |
| --- | --- | --- |
| `SourceIr` | **no — zero occurrences** | it stores structured table cells, not rendered Markdown |
| the normalized bundle | yes (`CONTEXTIDR\_EL1`) | it *is* the rendered Markdown |
| `EvidenceIr` | yes (2,908 in eMMC alone) | the evidence stage reads the bundle's text |

Every identifier tokenizer splits on characters that are not alphanumeric-or-underscore, so the
backslash ends the token and `PARTITION\_ACCESS` becomes `PARTITION`.

## The part that is not obvious: the fragment gets DECLARED

eMMC's statement set contains `Signal PARTITION is width 1.` and `Enum PARTITION NOT_DEFINED = 0.`
The catalog holds the fragment, so every subject gate, the polarity pass and the relation reader all
treat it as authority — and a wrong *declaration* is a much deeper failure than a wrong subject.

**And that defeats the repository's standard discriminator.** `EXTRACTION-QUALITY-GAUGE.3g`, `.3h` and
`.3k.11` all use *standalone wins*: a candidate that occurs even once outside the suspect position is
never touched. Here the synthesized declarations **are** those standalone occurrences. **The
contamination manufactures its own evidence of innocence.** A census that does not exclude the
synthesized `Signal …` / `Enum …` forms reports this class as EMPTY — the first cut of the
re-derivation did exactly that, and reported 0 where the answer is 18.

## Three populations, and they are not the same size

`python3 scripts/measure_escaped_identifier_fragments.py`:

* **TEXT — 67 of 78 documents** carry an escaped identifier somewhere. Wide, and mostly provenance: a
  register name in a section title is not intent.
* **CATALOG — 18 declared names across 2 documents** exist ONLY as the head of an escaped compound —
  17 in eMMC (`PARTITION`, `PARTITIONING`, `POWER`, `TAG`, `USER`, …), 1 in GIC-600 (`REQUEST` ←
  `REQUEST_COMPLETE`).
* **RECORDS — 0** published constraint subjects.

## Why it is not fixed yet

"Fix it upstream" is **not available**: re-ingest reproduces the escape by construction, and both
contaminated documents are frozen, so an ingest-side fix reaches neither. The only fix that reaches
them is a change to the shared identifier tokenization — the seam the catalog, every subject reader,
the polarity pass and the relation reader all sit on — which would move the identity layer of the 67
documents that carry the escape in order to correct 18 names in 2, with a published constraint effect
of zero.

Several of the 18 are ordinary English words (`POWER`, `USER`, `CLASS`, `NUMBER`), so withdrawing them
from a catalog can remove records that are correct for unrelated reasons; each needs adjudicating. The
change moves declarations rather than only constraints, so `replay-constraints` alone does not cover
it — it needs a full detached `scripts/check_doctrines.sh --all`.

Related: [[one-record-per-obligation-clause]], [[a-level-belongs-to-a-signal]],
[[persisted-census-measures-published-not-current]].
