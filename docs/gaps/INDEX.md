# Missing pieces from PDF to `.isf`: the index

Owner: `EXTRACTION-REPRESENTATION-GAPS.36`. This directory is the one place that lists everything SpecForge cannot yet
carry from a specification to strict-valid `.isf`, whichever side owns it, and who holds each item. It indexes; it
does not restate. Each item's detail lives in its owning leaf or request packet.

- [SpecForge's items](specforge.md): what SpecForge must still read, type, carry or lower, by document category
  and pipeline stage (S1–S25).
- [FSMGen's ladder](fsmgen-ladder.md): every `.isf` capability SpecForge needs, in the order a specification needs
  them, each marked supported, fixed upstream, requested, anticipated or a question (F1–F29). The table below
  keeps only the rungs FSMGen does not yet provide.
- Neighbours, not copies: the book's [gap ledger](../book/src/reference/representation-gaps.md) explains the gaps
  to a reader; [`docs/FSMGEN_FEEDBACK.md`](../FSMGEN_FEEDBACK.md) and
  [`docs/upstream/fsmgen/`](../upstream/fsmgen/PROCESS.md) hold the exchanges with FSMGen.

## Rules

1. Every item has an owner whose own `Status:` line is open. The table below is an ownership region: the
   `OWNERSHIP-CITATIONS` doctrine refuses an owner that is closed or missing, and a cited unit with no
   classification record.
2. A new missing piece gets a leaf in the tree that owns its category first, then a row here.
3. An FSMGen rung moves to *requested* only through [`PROCESS.md`](../upstream/fsmgen/PROCESS.md): a reproduced gap,
   a signed-off packet and an open feedback record. *Anticipated* rungs are planning, never requests (ADR 0033).
4. Re-check the ladder on every FSMGen pin refresh; move a closed item's row out when its leaf closes.

## The index

Status on `2026-09-26`. Side: **S** SpecForge, **F** FSMGen (the owner is the SpecForge leaf that tracks it).

<!-- gap_catalog_owners:start -->

| Item | Side | Road | Stage | Status | Owner |
| --- | --- | --- | --- | --- | --- |
| S1 | S | register, platform, CPU ISA | source | open | `EXTRACTION-REPRESENTATION-GAPS.3` |
| S2 | S | all | all | in progress | `EXTRACTION-REPRESENTATION-GAPS.3` |
| S3 | S | all | evidence | open | `KG-ISF-COMPLETENESS.1` |
| S4 | S | all | semantic | open | `EXTRACTION-REPRESENTATION-GAPS.18` |
| S5 | S | wire protocol | evidence, adapter | open | `EXTRACTION-REPRESENTATION-GAPS.7`, `EXTRACTION-REPRESENTATION-GAPS.7.e.i` |
| S6 | S | wire protocol | adapter | open | `EXTRACTION-REPRESENTATION-GAPS.9` |
| S7 | S | wire protocol | evidence, adapter | open | `EXTRACTION-REPRESENTATION-GAPS.10` |
| S8 | S | wire protocol | evidence | blocked on F7 | `EXTRACTION-REPRESENTATION-GAPS.10.g`, `EXTRACTION-REPRESENTATION-GAPS.10.g.iii` |
| S9 | S | wire protocol | adapter | open; needs F12 | `EXTRACTION-REPRESENTATION-GAPS.11` |
| S10 | S | wire protocol | semantic | open | `EXTRACTION-REPRESENTATION-GAPS.12` |
| S11 | S | wire protocol | evidence | open | `EXTRACTION-REPRESENTATION-GAPS.17.a` |
| S12 | S | wire protocol | evidence, semantic | open; needs F15 | `EXTRACTION-REPRESENTATION-GAPS.37` |
| S13 | S | wire protocol | evidence | open | `WIRE-BASED-100.10d` |
| S14 | S | wire protocol | evidence | open | `WIRE-BASED-100.10f` |
| S15 | S | wire protocol | semantic, intent | open | `KG-ISF-TRANSACTIONS` |
| S16 | S | wire protocol (serial) | evidence | in progress | `PDF-VARIANT-DIGESTION.9.3b`, `PDF-VARIANT-DIGESTION.9.10`, `PDF-VARIANT-DIGESTION.9.7` |
| S17 | S | register IP | adapter | open | `EXTRACTION-REPRESENTATION-GAPS.13`, `DOC-INTENT-TAXONOMY.4a.i` |
| S18 | S | register IP, wire protocol | intent | open; then F23 | `DOC-INTENT-TAXONOMY.4b` |
| S19 | S | register IP | evidence, adapter | open | `KG-ISF-COMPLETENESS.2a.vii`, `KG-ISF-COMPLETENESS.2a.vii.c.iv`, `KG-ISF-COMPLETENESS.2a.vii.c.v` |
| S20 | S | register IP | evidence | open | `SIGNAL-DECLARATION-ROW-DROP.2f` |
| S21 | S | register IP | adapter | open | `KG-ISF-COMPLETENESS.5` |
| S22 | S | platform | evidence, adapter | open; needs F25 | `EXTRACTION-REPRESENTATION-GAPS.14`, `DOC-INTENT-TAXONOMY.4c.ii` |
| S23 | S | CPU ISA | source, evidence | decided residual | `DOC-INTENT-TAXONOMY.4` |
| S24 | S | all | source | open | `TEXT-LAYER-IDENTIFIER-SPLIT.1` |
| S25 | S | all | source | not started | `FULL-PAGE-INTENT-CAPTURE.2` |
| F2 | F | all | adapter | fixed upstream | `EXTRACTION-REPRESENTATION-GAPS.21` |
| F4 | F | wire protocol, register IP | adapter | requested (acknowledged) | `FSMGEN-REQUESTS.1` |
| F5 | F | all | adapter | requested (forwarded) | `EXTRACTION-REPRESENTATION-GAPS.6` |
| F6 | F | all | adapter | requested (forwarded) | `EXTRACTION-REPRESENTATION-GAPS.5` |
| F7 | F | wire protocol | adapter | requested (forwarded) | `EXTRACTION-REPRESENTATION-GAPS.4` |
| F8 | F | all | adapter | fixed upstream | `FSMGEN-REFRESH-INTEGRATE-10.1` |
| F11 | F | all | adapter | requested (acknowledged) | `FSMGEN-REQUESTS.2` |
| F12 | F | wire protocol | adapter | requested (forwarded) | `EXTRACTION-REPRESENTATION-GAPS.15` |
| F13 | F | wire protocol | adapter | requested (forwarded) | `EXTRACTION-REPRESENTATION-GAPS.16` |
| F15 | F | wire protocol | adapter | anticipated | `EXTRACTION-REPRESENTATION-GAPS.37` |
| F18 | F | wire protocol | adapter | requested (forwarded) | `EXTRACTION-REPRESENTATION-GAPS.20` |
| F23 | F | register IP, wire protocol | adapter | anticipated | `DOC-INTENT-TAXONOMY.4b` |
| F26 | F | platform | adapter | anticipated (multi-bit crossings) | `EXTRACTION-REPRESENTATION-GAPS.3` |
| F28 | F | tooling | — | requested (acknowledged) | `FSMGEN-REQUESTS.3` |
| F29 | F | wire protocol | adapter | anticipated | `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii` |

<!-- gap_catalog_owners:end -->

## Counts

25 SpecForge items and 14 FSMGen rungs not yet provided, of FSMGen's 28. The largest open body of work is on the
SpecForge side and upstream: most register and platform documents, and every CPU-ISA one, cannot yet be replayed on
current code (S1), so their items (S17–S23) are measured mostly on legacy artifacts.
