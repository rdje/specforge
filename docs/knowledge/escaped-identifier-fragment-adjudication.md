---
id: escaped-identifier-fragment-adjudication
title: All 18 escape-fragmented catalog names are artefacts and adjudicate to removal — the English-word risk is the evidence for it, not against
answers:
  - "are the 18 escape-fragmented declared names real signals or tokenization artefacts (all 18 are artefacts and adjudicate the same way, remove: each has ZERO standalone uppercase occurrences in its document's prose, which is what put it in the population, while the compound it fragments is the genuine identity)"
  - "does removing an English-word catalog name like POWER or USER risk withdrawing a correct record (no, and the evidence runs the other way: eMMC writes the word power 361 times in lowercase prose and the identifier POWER never, so declaring POWER a signal is the defect; the published record population for all 18 is 0, so nothing is withdrawn today)"
  - "which names are escape-fragmented in the persisted corpus and from what (eMMC 17: CLASS from CLASS_6_CTRL, CONTEXT from CONTEXT_CONF, EXCEPTION, LARGE, NATIVE, NUMBER, OPERATION, PARTITION from PARTITION_ACCESS, PARTITIONING, PARTITIONS, PERIODIC, POWER from POWER_CLASS, PRE, PRODUCTION, PROGRAM, TAG from TAG_UNIT_SIZE, USER from USER_WP; GIC-600 1: REQUEST from REQUEST_COMPLETE)"
  - "why does a census of escape-fragmented declarations report zero if you are not careful (the fragmentation mints its own Signal X is width N. and Enum X ... = 0. statements, and the repository's standard standalone-wins discriminator then counts those as standalone occurrences and clears the fragment that created them — the contamination manufactures its own evidence of innocence; measured, disabling the exclusion makes the corpus census report 0 names across 0 documents)"
  - "how is the escaped-identifier circularity controlled (spurious_heads carries the whole classification with an exclude_synthesized switch that exists only for the self-test's RED case; with the exclusion an opaque XQPART fixture classifies as a fragment and without it as empty, and the same removal on the real corpus turns 18 of 2 into 0 of 0)"
  - "how do I re-run the escaped-identifier fragment census (python3 scripts/measure_escaped_identifier_fragments.py, and --self-test for its 8 cases)"
  - "is escape fragmentation the only way an ordinary word becomes a declared signal (no — ACTOR-NOUN-RELATION-DECLARATION owns the same outcome reached by an inferred declaration, so a fix at the tokenization seam addresses only this route)"
date: 2026-09-18
status: current
tags: [extraction-quality-gauge, tokenization, catalog, declarations, census, adjudication, method]
evidence: docs/tasks/extraction-quality-gauge/kind-span-family.md (.3k.9.a); docs/tasks/extraction-quality-gauge/kind-span-successors.md (.3k.9); scripts/measure_escaped_identifier_fragments.py; generated/evidence_ir/jesd84_b50_2013_09_emmc_5_0/evidence_ir.json; generated/evidence_ir/100336_0106_00_2019_02_08_gic_600_technical_reference_manual/evidence_ir.json
reverify: "python3 scripts/measure_escaped_identifier_fragments.py — expect text_population 67 documents, catalog_population 18 names across 2 documents, record_population 0; and --self-test 8/8"
---

Docling renders an underscore inside an identifier as `\_`, which is correct Markdown. The identifier
tokenizers split on any character that is not alphanumeric-or-underscore, so `PARTITION\_ACCESS` is cut at
the backslash and yields `PARTITION` — and the evidence stage then **declares** that fragment, minting
`Signal PARTITION is width 1.` into the document's own statement set. Every subject gate, polarity pass and
relation reader downstream treats it as authority.

## The adjudication: all 18, remove

`EXTRACTION-QUALITY-GAUGE.3k.9` flagged a risk before shipping anything: several of the fragments are
ordinary English words, and removing one from a catalog might withdraw a record that is correct for an
unrelated reason. Read against the documents, that risk is the evidence *for* removal.

Each of the 18 has **zero** standalone uppercase occurrences in prose — that is the criterion that put it
in the population. Counted case-insensitively over the same prose, the lowercase English word is
everywhere: `POWER` 361, `OPERATION` 336, `CLASS` 170, `PARTITION` 161, `NUMBER` 114, `CONTEXT` 122,
`USER` 101, `PARTITIONS` 97, `LARGE` 79, `PRE` 60, `REQUEST` 48 (GIC-600), `TAG` 46, `NATIVE` 34,
`PARTITIONING` 27, `PRODUCTION` 26, `EXCEPTION` 25, `PROGRAM` 11, `PERIODIC` 1.

So eMMC declares `POWER` a signal on the strength of `POWER\_CLASS` and `POWER\_OFF\_LONG`, while writing
the word *power* 361 times and the identifier `POWER` never. The real identity is always the compound —
`PARTITION_ACCESS`, `USER_WP`, `TAG_UNIT_SIZE` are genuine EXT_CSD fields, `REQUEST_COMPLETE` a GIC-600
state. The head is the artefact. No name is retained and the published record population is 0, so nothing
is withdrawn today.

Two source oddities are worth carrying rather than rediscovering: `PRE_SOLDERING_` keeps a trailing
underscore, and `PRODUCTION_STATA_AWARENESS` sits beside `PRODUCTION_STATE_AWARENESS`. Both are the
source's own truncation and typo, which is a reminder that the compound is copied rather than parsed.

## The circularity, and why a census here is not a grep

The fragmentation mints its own declarations, and those synthesized sentences then count as standalone
occurrences of the fragment under the repository's standard *standalone wins* discriminator. **The
contamination manufactures its own evidence of innocence.** A census that does not exclude the synthesized
`Signal …` / `Enum …` forms reports this class as empty, and the leaf's own first re-derivation did
exactly that.

Measured `2026-09-18` by removing the exclusion and re-running the real corpus census:
`catalog_population: 0 declared name(s) across 0 document(s)`. The control is now over the
**classification** rather than over one helper, so a future reader cannot reach emptiness in silence.

## What still has to be true before a fix ships

`.3k.9` remains parked, and the arithmetic is why: the only fix reaching the two contaminated documents is
a change to the shared identifier tokenization, which moves the identity layer of **67** documents to
correct **18 names in 2**, with a published constraint effect of **0**. The remaining preconditions are two
detached runs — `replay-constraints` plus a full `scripts/check_doctrines.sh --all`, and a rebuild-and-diff
of every current-schema document that moves (AXI-L carries `AWSNOOP\_WIDTH` and `WSTRB\_Present`).
