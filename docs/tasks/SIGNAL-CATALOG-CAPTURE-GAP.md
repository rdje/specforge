# SIGNAL-CATALOG-CAPTURE-GAP: protocol specifications that declare no signals at all

## Metadata

- Tree ID: `SIGNAL-CATALOG-CAPTURE-GAP`
- Status: `active`
- Roadmap lane: `R9`/`R15` extraction breadth — signal-declaration capture
- Created: `2026-08-11`
- Last updated: `2026-08-11`
- Owner: repo-local workflow
- Parent finding: `SEMANTIC-EMPTY-CATALOG-FILTER.1` (`2026-08-11`), which made the gap visible per document

## Goal

Thirty-three of the 78 corpus documents carry **zero interface signal records of any confidence** — not
low-confidence ones that a threshold excluded, none at all. For most that is the honest answer. For a
measured few it is not: the document is a wire-level protocol specification that plainly names its signals,
and SpecForge captured no catalog for it.

Establish which of the 33 are real capture misses, find where each document's signal declarations actually
live, and close the ones that a general, ADR-0006-agnostic rule can close.

## Non-Goals

- Do not lower the declared-signal confidence bar to manufacture a catalog. A catalog assembled from
  co-mention noise is worse than none — that is the boundary `CORPUS-HARDENING` and the Layer-D authority rule
  already established, and it is not reopened here.
- Do not special-case any document, vendor, or protocol name (ADR 0006). Every fix must be structural grammar.
- Do not re-litigate the grounding filter. `SEMANTIC-EMPTY-CATALOG-FILTER` closed that question: an ungrounded
  record is demoted, not promoted. This tree is about the *missing catalog*, not about what to do without one.
- Do not invent a declaration the document never makes. A guide that describes a subsystem without naming wires
  has an honest empty catalog, and that must stay the answer.

## How the gap became visible (`2026-08-11`)

`SEMANTIC-EMPTY-CATALOG-FILTER.1` made `SemanticIR` refuse canonical authority to any record naming an
undeclared signal, and demote the refusal into a `semantic_ungrounded_records_not_promoted` packet. Reading
those packets is what exposes this: on several documents the refused names are not boilerplate but **real
wires cut short**.

| Document | Refused subjects (sample) | What the document actually spells |
| --- | --- | --- |
| `wbspec_b4_wishbone_b4_specification` | `CLK`, `CYC`, `STB`, `RST`, `STALL` | `CLK_I`, `CYC_O`, `STB_O`, `RST_I`, `STALL_I` |
| `ihi0088_g_2024_06_amba_dti_protocol_specification` | `TDATA`, `TKEEP`, `TLAST`, `TRANS` | AXI-Stream signal names carried by DTI |
| `usb_3_2_revision_1_0_2017_09` | `ACK`, `ERDY`, `NRDY`, `DPH`, `DPP` | link-command and packet-header identifiers |

Alongside those sit unmistakable non-signals — `DATASHEET`, `MUST`, `PCI`, `IMPLEMENTATION`, `PDF`, `AMBA`,
`FPGA` — which is why the refusal itself is correct and this tree is about the *input* to the refusal.

## Census (`2026-08-11`, read-only over all 78 persisted artifacts)

Crossing the 33 empty-catalog documents with each document's own persisted `document_class` separates honest
absence from a probable miss without any name list:

| `document_class` | Count | Reading |
| --- | ---: | --- |
| `guide` | 17 | honest absence — methodology guides, PHY mechanical/signaling notes, test-resource notes |
| `register` | 7 | mixed — register-centric specs where signals are secondary, but two are AMBA interface specs |
| `protocol` | 5 | **prima facie capture misses** — a wire-level protocol specification with zero signals |
| *(none persisted)* | 4 | unclassified; re-validate before judging |

The five `protocol`-classed documents are the sharp end of the tree:
`wbspec_b4_wishbone_b4_specification`, `ihi0088_g_2024_06_amba_dti_protocol_specification`,
`usb_3_2_revision_1_0_2017_09`, `bosch_can_specification_2_0_1991`, and
`usb4_connection_manager_guide_v2_0_2025_11`.

Reproducer:

```bash
python3 - <<'PY'
import json, glob, os
for sp in sorted(glob.glob("generated/semantic_ir/*/semantic_ir.json")):
    key = sp.split(os.sep)[2]
    s = json.load(open(sp))
    recs = [r for i in s.get("interfaces", []) for r in i.get("signal_records", [])]
    if any(r.get("automation_confidence") != "low" for r in recs):
        continue
    cls = None
    ep = f"generated/evidence_ir/{key}/evidence_ir.json"
    if os.path.exists(ep):
        for rep in json.load(open(ep)).get("validation_reports", []):
            for m in rep.get("metrics", []):
                if m.get("metric_id") == "document_class":
                    cls = m.get("value")
    print(f"{key} class={cls} interface_signal_records={len(recs)}")
PY
```

## Acceptance Criteria

1. Every one of the 33 empty-catalog documents is classified as **honest absence** or **capture miss**, with the
   evidence for the call named per document (the modality the declarations live in, or the absence of one).
2. For each capture miss, the *location* of the declarations is established before any rule is written — a
   signal-description table, a section-heading convention, a prose declaration form, or a figure.
3. At least the `protocol`-classed misses gain a real catalog through a general structural rule, re-measured on
   the same documents that quantified the gap.
4. No document gains a fabricated signal: precision is measured, not assumed, and a document with no
   declarations still yields none.
5. `kg-bench` 156/156; WIRE-BASED-100 golds hold at 1.000; emitted `.isf` gain no new FSMGen strict diagnostics;
   `scripts/run_ci.sh` green.

## Task Tree

| Leaf | Status | Scope |
| --- | --- | --- |
| `SIGNAL-CATALOG-CAPTURE-GAP.0` | `done` | ownership + the census above; no code |
| `SIGNAL-CATALOG-CAPTURE-GAP.1` | `pending` | classify all 33 as honest absence vs capture miss, with per-document evidence |
| `SIGNAL-CATALOG-CAPTURE-GAP.2` | `pending` | for the confirmed misses, locate where the declarations live (per the honesty guardrail) |
| `SIGNAL-CATALOG-CAPTURE-GAP.3` | `pending` | close what a general structural rule can close; re-measure precision and recall |

## Current Frontier

`SIGNAL-CATALOG-CAPTURE-GAP.1` — classify the 33. Start with the five `protocol`-classed documents, because a
wire-level protocol specification with zero signals is the least ambiguous call available, and Wishbone is the
smallest and most explicit of them (`CLK_I`/`STB_O`/`ACK_I` are spelled in a signal-description table).

The standing honesty guardrail from `EXTRACTION-GAP-FIX` applies verbatim: **you cannot extract what is not
there.** Establish where a declaration actually lives before reading it, and prefer an honest residual to a
guessed catalog.

## Decisions

- `2026-08-11`: own this in its own tree rather than as an open question inside
  `SEMANTIC-EMPTY-CATALOG-FILTER`. That tree closed, and an open question in a closed tree is a question nobody
  is scheduled to answer. The finding is a capture gap, not a grounding defect, and the two have different
  fixes, different oracles, and different blast radii.
- `2026-08-11`: use each document's own persisted `document_class` as the first separator rather than a
  judgement about which names "look like wires". The class is derived from document structure, so it carries no
  vendor or protocol vocabulary (ADR 0006), and it is already an audited surface.

## Open Questions

- Is the truncation itself (`CLK` from `CLK_I`, `PWR` from `PWR_GOOD`) a *separate* extractor defect, or the
  same one seen from the other side? `SEMANTIC-EMPTY-CATALOG-FILTER` explicitly did not claim it. If the
  extractor is cutting identifiers at `_`, that may be the whole gap for suffix-convention documents like
  Wishbone.
- Do the 7 `register`-classed documents deserve the same bar? Two are AMBA interface specifications, where a
  missing signal catalog is as surprising as in a `protocol`-classed document.
- Do the 4 documents with no persisted `document_class` simply need re-validation, or is the class itself
  failing on them?

## Blockers

- None. This tree blocks nothing and is blocked by nothing; the demotion packets that expose it are already
  persisted on 67 documents.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-11` | `.0` | census over all 78 persisted `SemanticIR` artifacts for an empty declared-signal set | 33 documents, **all** with zero interface signal records of any confidence — none is merely low-confidence |
| `2026-08-11` | `.0` | crossed those 33 with each document's persisted `document_class` | 17 `guide`, 7 `register`, 5 `protocol`, 4 unclassified — read-only, no artifact mutated |
| `2026-08-11` | `.0` | read the demotion packets on the largest movers | refused subjects mix unmistakable non-signals with real wires cut short (`CLK`/`CYC`/`STB`, `TDATA`/`TKEEP`/`TLAST`, `ACK`/`ERDY`/`NRDY`) |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SIGNAL-CATALOG-CAPTURE-GAP.0` | `SIGNAL-CATALOG-CAPTURE-GAP.0 — own the documents that declare no signals at all` | ownership and census only, no code |

## Changelog

- `2026-08-11`: created from a measured finding in `SEMANTIC-EMPTY-CATALOG-FILTER.1`, whose demotion packets
  made the missing catalogs visible per document. Ownership and census only; no code change.
