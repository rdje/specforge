# SIGNAL-CATALOG-CAPTURE-GAP: protocol specifications that declare no signals at all

## Metadata

- Tree ID: `SIGNAL-CATALOG-CAPTURE-GAP`
- Status: `active`
- Roadmap lane: `R9`/`R15` extraction breadth — signal-declaration capture
- Created: `2026-08-11`
- Last updated: `2026-09-20`
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

Reproducer: see [the classification measurement](../research/empty-signal-catalog-classification.md#reproducer).
The snippet first published here read `metric.get("metric_id")`; `EvidenceIR` validation metrics are
`{name, value}` pairs and carry no `metric_id`, so it reported `class=None` for every document. The breakdown
above is correct and reproduces exactly with the corrected key (`.1`, `2026-08-11`).

## Classification (`SIGNAL-CATALOG-CAPTURE-GAP.1`, `2026-08-11`)

Three presence probes decide each document — an identity-header `signal_description` table, a bare-identifier
section heading, and the formal `Signal <ID> is <predicate>` prose grammar. All are shape predicates over
persisted artifacts and name no document, vendor, or protocol (ADR 0006). Full per-document evidence lives in
[`docs/research/empty-signal-catalog-classification.md`](../research/empty-signal-catalog-classification.md);
the durable conclusion is
[`empty-signal-catalog-is-mostly-honest-absence`](../knowledge/empty-signal-catalog-is-mostly-honest-absence.md).

**32 honest absence · 1 capture miss.**

- **The table gate is not the culprit.** Not one of the 33 has a `signal_description` table with an identity
  header; catalog-bearing documents carry up to 19 each. The 44 `signal_description` classifications present
  across the 33 are register, flit-field, packet-format, and table-of-contents shapes — correctly refused. The
  five `protocol`-classed documents were the sharp end of `.0`; four of them are honest absence.
- **`wbspec_b4_wishbone_b4_specification` is the capture miss.** It declares its interface as a
  **heading-as-declaration convention**: 32 sections whose title *is* the wire name (`CLK_I`, `RST_O`,
  `ACK_I`, `CYC_O`, `DAT_I()`, `STALL_I`, …), each followed by prose giving direction and meaning. 485 of its
  1,622 `SourceIR` content elements name a `*_I`/`*_O` wire. SpecForge reads declarations from tables and
  from a formal prose predicate; this document uses neither.
- **The other four `protocol`-classed documents are honest absence with a nameable reason each**: USB 3.2's
  identifier headings are hub port-feature selectors and its `signal_description` tables are LMP formats and
  the VBUS matrix, already measured false in `dense-prose-false-signal-loop-reaches-isf`; AMBA DTI declares a
  naming *transformation* (`Direction | Suffix` → `*_DTI_DN`) over AXI-Stream signals declared elsewhere;
  Bosch CAN 2.0 specifies frame formats and bit timing, with the wires in the companion physical-layer
  standard; the USB4 connection-manager guide self-reports `document_intent_category = methodology-guide`.
- **The probes are candidate generators, not oracles**, and each hit was read. Every one of the 41
  `signal_description` tables was inspected rather than sampled; relaxing the heading shape to any case adds
  zero wire declarations corpus-wide; and one heuristic in this leaf *was* wrong and was caught (a
  `/Type/Page` byte-regex claimed a 6-page source was 125 pages — `pdfinfo` settled it). The robustness
  section of the measurement records all three, and why a title shape alone cannot become the product rule.

## Acceptance Criteria

1. ✅ (`.1`) Every one of the 33 empty-catalog documents is classified as **honest absence** or **capture
   miss**, with the evidence for the call named per document (the modality the declarations live in, or the
   absence of one).
2. ✅ (`.1`) For each capture miss, the *location* of the declarations is established before any rule is
   written — for the single miss it is a section-heading convention, 32 wire-named titles with direction-bearing
   bodies.
3. The confirmed miss gains a real catalog through a general structural rule, re-measured on the same
   documents that quantified the gap. *(Narrowed by `.1` from "at least the `protocol`-classed misses": four
   of the five `protocol`-classed documents are honest absence, so there is one miss to close, not five.)*
4. No document gains a fabricated signal: precision is measured, not assumed, and a document with no
   declarations still yields none.
5. `kg-bench` 156/156; WIRE-BASED-100 golds hold at 1.000; emitted `.isf` gain no new FSMGen strict diagnostics;
   `scripts/run_ci.sh` green.

## Task Tree

| Leaf | Status | Scope |
| --- | --- | --- |
| `SIGNAL-CATALOG-CAPTURE-GAP.0` | `done` | ownership + the census above; no code |
| `SIGNAL-CATALOG-CAPTURE-GAP.1` | `done` | classify all 33 as honest absence vs capture miss, with per-document evidence |
| `SIGNAL-CATALOG-CAPTURE-GAP.2` | `pending` | design the heading-as-declaration rule against the whole corpus: what shape licenses a title as a declaration, and what it costs on the 45 catalog-bearing documents |
| `SIGNAL-CATALOG-CAPTURE-GAP.3` | `pending` | land the rule; re-measure precision and recall on the same documents that quantified the gap |
| `SIGNAL-CATALOG-CAPTURE-GAP.4` | `superseded` by `EXTRACTION-QUALITY-GAUGE.3k.9` (`2026-09-20`) | stop the normalized-markdown escape from truncating underscore-bearing identifiers — **the same defect, owned twice**; see the note below |
| `SIGNAL-CATALOG-CAPTURE-GAP.5` | `pending` | 14 of 78 `EvidenceIR` artifacts carry no validation report, so they have no `document_class`; decide whether that is a currency gap or a contract gap |
| `SIGNAL-CATALOG-CAPTURE-GAP.6` | `done` | measured: 2 real relationship tables in 1 legacy document, 18 names, **0 in the current stratum** — and the blocker is not the table shape but the `(direction, width)` admission gate, which is `SIGNAL-DECLARATION-ROW-DROP`'s |

## `.6` — a partial catalog, found from the other end (`2026-09-14`)

This tree's census counted documents whose catalog is **empty**. AMBA LPI's is not empty and is still
wrong: it holds **5** signals — `PACCEPT`, `PREQ`, `QACCEPTN`, `QDENY`, `QREQN` — while the document's
own tables name at least eleven.

Found by `EXTRACTION-QUALITY-GAUGE.3k.12`, from a direction that had nothing to do with catalogs: a
logic-level walk stopped at the token `PDENY` in *"a device must set both PACCEPT and PDENY LOW"*,
because a word only continues that walk when it is a **declared** signal. The blocked walk was the
symptom; the missing declaration was the fact.

`Table 3-2 Parity extended P-Channel check signal relationships` is typed `signal_description` and reads

```text
Standard P-Channel signal | Associated check signal
PACTIVE[N-1:0]           | PACTIVECHK[N-1:0]
PSTATE[M-1:0]            | PSTATECHK
PREQ                     | PREQCHK
PACCEPT                  | PACCEPTCHK
PDENY                    | PDENYCHK
```

Ten signals in five rows, and **none of them declares**: the table states no direction and no width, so
every row leaves by `synthesize_signal_declarations`' `NoDirectionAndNoWidth` arm — the loss
`SIGNAL-DECLARATION-ROW-DROP.1` made countable. The Q-Channel has an identical table (`Table 2-2`), and
the five names the catalog does hold come from elsewhere in the document.

**The shape is a general one and that is why it is worth a leaf**: a *relationship* table names a signal
in every cell of both its columns, and the relationship itself — `X is checked by XCHK` — is the fact the
document is stating. A reader that requires a direction or a width cannot see it, and a reader that
admits any two-column table would admit every glossary in the corpus. Size that population before
proposing anything, and check it against `SIGNAL-DECLARATION-ROW-DROP`'s own frontier before opening a
second reader for the same rows.

## `.6` — measured, and the blocker is not where the leaf looked (`2026-09-14`)

The leaf opened on AMBA LPI: a catalog of **5** signals while `Table 3-2 Parity extended P-Channel check
signal relationships` — typed `signal_description` — names **ten in five rows** and declares none. The
proposed subject was a general shape: a two-column *relationship* table that names a signal in every cell
of both columns.

**Measured over every table the declaration reader examines, that shape is almost empty**: 3 tables
corpus-wide, **0 in the current stratum**, and one of the three is a false positive —
CoreSight `table_0047` is `name | description` (`TSCLK | Interface clock`), admitted only because
"Interface clock" is two words beginning with an identifier-shaped token. The real population is **2
tables in 1 document, 18 identifier cells**: LPI's `Table 2-2` (Q-Channel) and `Table 3-2` (P-Channel).
A two-column shape rule is therefore one document's rule with a false positive already attached, which is
this repository's standing finding arriving again.

### Why the rows drop, traced through the real reader

The reader gets further than the leaf assumed, and stops for the reason `SIGNAL-DECLARATION-ROW-DROP` owns:

| step | result on `Standard P-Channel signal \| Associated check signal` |
| --- | --- |
| `is_signal_name_column_header` | matches **both** columns; `name_col` = 0, correctly |
| name token | `PACTIVE`, `PSTATE`, `PREQ`, `PACCEPT`, `PDENY` — all accepted identifiers |
| width column | no `width`/`size`/`bits` header — none |
| `check_signal_col` | **found, at column 1** (`associated check signal` contains `check signal`) |
| `covered_signal_col` | **not found** — the header says `standard p-channel signal`, not `signals covered` |
| direction column | no `direction`/`source`/`destination` header — none |
| outcome | `NoDirectionAndNoWidth`; every row dropped, and counted |

So the identity is read, the table is typed, the name column is right, and the row is discarded because it
offers **no attribute** — not because the reader cannot see the table.

### The real question, and it is not this tree's

**Must a declaration carry an attribute to carry an identity?** The reader requires a direction or a width
before it will admit a signal exists, and a relationship table states neither: it states that `PACTIVE` is
checked by `PACTIVECHK`, which is a fact about two signals that presupposes both. That is
`SIGNAL-DECLARATION-ROW-DROP.1`'s thesis in a new form — the `(direction, width)` arm is the whole of the
measured 18.3% loss — and the decision belongs there, sized against all 482 dropped rows rather than
against LPI's ten.

Recorded here so the next reader of this tree does not re-derive it, and so that a "relationship table"
rule is not written for a population of two tables in one legacy document.

## Current Frontier

`SIGNAL-CATALOG-CAPTURE-GAP.2` — one document is a confirmed capture miss and its declarations are located:
Wishbone's 32 wire-named section headings. The leaf is now a *design* question, not a search: establish what
title shape may license a declaration, and measure what that shape would admit across all 78 documents before
any code is written. The calibration already collected is the starting point — the next-highest compound-heading
count after Wishbone's 32 is 13, and those 13 are software feature selectors, so the naive shape predicate
alone is not safe.

`SIGNAL-CATALOG-CAPTURE-GAP.6` closed `2026-09-14` with no rule: the two-column relationship shape is 2 real
tables in 1 legacy document and 0 in the current stratum, and the rows drop on the `(direction, width)`
admission gate rather than on the table shape. The question it exposes — must a declaration carry an
attribute to carry an identity? — belongs to `SIGNAL-DECLARATION-ROW-DROP`, sized against all 482 dropped
rows.

`SIGNAL-CATALOG-CAPTURE-GAP.4` is **SUPERSEDED** (`2026-09-20`) by `EXTRACTION-QUALITY-GAUGE.3k.9`. Both
leaves are the same defect — the normalized markdown escapes `_` as `\_` and identifier tokenization stops
at the backslash — and both name the same blast radius of **67 of 78 documents**. Owning one defect twice is
how two trees each wait for the other; `.3k.9` is the owner because it has already discharged the parts that
need no rebuild (all 18 contaminated names adjudicated, all *remove*, plus a circularity control observed RED
on the real corpus), and its node carries the population, the fix seam and the magnitude argument in one
place. **It surfaced a third time the same day** inside `EXTRACTION-GAP-FIX.5a`'s refusal evidence —
`AWSNOOP\_WIDTH must be 5` minting a constraint on `AWSNOOP` — which is the same fragment arriving through
the constraint path rather than the catalog path.
**What this tree keeps** is the consequence it measured from its own end: the escape is why several of its 33
zero-catalog documents look emptier than they are. If `.3k.9` ships the unescape, re-run this tree's census
before concluding anything about capture; if `.3k.9` refuses it on the magnitude argument, that refusal is
this tree's answer too and `.4` stays superseded rather than reopening.

The standing honesty guardrail from `EXTRACTION-GAP-FIX` applies verbatim: **you cannot extract what is not
there.** Prefer an honest residual to a guessed catalog.

## Decisions

- `2026-08-11`: own this in its own tree rather than as an open question inside
  `SEMANTIC-EMPTY-CATALOG-FILTER`. That tree closed, and an open question in a closed tree is a question nobody
  is scheduled to answer. The finding is a capture gap, not a grounding defect, and the two have different
  fixes, different oracles, and different blast radii.
- `2026-08-11`: use each document's own persisted `document_class` as the first separator rather than a
  judgement about which names "look like wires". The class is derived from document structure, so it carries no
  vendor or protocol vocabulary (ADR 0006), and it is already an audited surface.

## Open Questions

All three questions opened by `.0` are answered by `.1`. They are kept with their answers because each
changed a leaf.

- ~~Is the truncation a separate defect, or the same one seen from the other side?~~ **Separate, and
  mechanically explained.** `EvidenceIR` statement text is read from the normalized markdown, where Docling
  escapes `_` as `\_`; identifier tokenization stops at the backslash. `SourceIR` content elements carry the
  same sentences unescaped. It is not a rule that cuts at `_`, and it affects 67 of 78 documents including 40
  that already have catalogs. Now owned by `.4` —
  [`evidence-statement-markdown-escape-truncates-identifiers`](../knowledge/evidence-statement-markdown-escape-truncates-identifiers.md).
- ~~Do the 7 `register`-classed documents deserve the same bar? Two are AMBA interface specifications.~~
  **The bar is the same; the evidence decides, and it acquits all seven.** Their `signal_description` tables
  are `Bit Location | Register Description | Attributes`, `Field name | Width (bits) | Value`, and
  `Offset | Description` shapes. The two AMBA ones are no exception.
- ~~Do the 4 unclassified documents need re-validation, or is the class failing on them?~~ **Neither is quite
  right.** They carry *zero validation reports*, so there is no metric surface at all. `document_class` is
  back-annotated by the `validate` command, and 14 of 78 evidence artifacts are in this state — including
  `ihi0022_l_…_amba_axi`, `ihi0024_e_…_amba_5_apb`, and `ihi0033_c_…_amba_5_ahb`. Now owned by `.5`.

New question opened by `.1`, deliberately not answered here:

- AMBA DTI declares a naming *transformation* (`Direction | Suffix` → `*_DTI_DN`) over signals declared in a
  different specification. Honest absence is the right call for the document in isolation, but is a
  cross-document declaration import ever in scope for SpecForge? Local grounding
  (`ROADMAP.md`, canonical document truth) says no by default; nothing here reopens it.

## Blockers

- None. This tree blocks nothing and is blocked by nothing; the demotion packets that expose it are already
  persisted on 67 documents.

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-08-11` | `.0` | census over all 78 persisted `SemanticIR` artifacts for an empty declared-signal set | 33 documents, **all** with zero interface signal records of any confidence — none is merely low-confidence |
| `2026-08-11` | `.0` | crossed those 33 with each document's persisted `document_class` | 17 `guide`, 7 `register`, 5 `protocol`, 4 unclassified — read-only, no artifact mutated |
| `2026-08-11` | `.0` | read the demotion packets on the largest movers | refused subjects mix unmistakable non-signals with real wires cut short (`CLK`/`CYC`/`STB`, `TDATA`/`TKEEP`/`TLAST`, `ACK`/`ERDY`/`NRDY`) |
| `2026-08-11` | `.1` | three presence probes over all 78 persisted chains (identity-header table / bare-identifier heading / formal prose grammar) | **32 honest absence, 1 capture miss.** Probe A = 0 and Probe C = 0 on all 33; Probe B isolates Wishbone at 32 compound wire headings against a next-highest of 13 |
| `2026-08-11` | `.1` | re-read **all 41** `signal_description` tables across the 33 (not the ≤4-per-document sample) | 18 distinct header shapes, every one register / flit-field / packet-format / 8b10b-encoding / status-matrix / table-of-contents. The `.0` count of 44 was a mis-sum and is corrected to 41 |
| `2026-08-11` | `.1` | relaxed the heading probe from uppercase-only to any case across all 33 (blind-spot test) | zero new wire declarations; only section words, glossary entries, CamelCase protocol identifiers, and DTI pseudocode functions. Classification unchanged; the shape predicate is confirmed unsafe as a product rule |
| `2026-08-11` | `.1` | traced the truncated subjects across four surfaces | normalized markdown carries `\_`; `SourceIR` content elements 0/1,622 and table cells 0/739; `EvidenceIR` statements 455/1,501; typed `subject_signal` truncated while its own `source_text` keeps the escaped name |
| `2026-08-11` | `.1` | corpus-wide ingest completeness, `document_profile.page_count` vs `pdfinfo` | 77 of 78 sources available, **none** ingested short. The apparent `jesd235` truncation is a genuine 6-page source PDF; the `/Type/Page` byte-regex that suggested 125 pages is not a reliable count |
| `2026-08-11` | `.1` | validation-metric surface across all 78 evidence artifacts | 64 carry metrics, **14 carry no validation report at all** — the reason four documents have no `document_class` |
| `2026-08-11` | `.1` | corrected the `.0` census reproducer (`metric_id` → `name`) and re-ran it | class breakdown reproduces exactly: 17 `guide` / 7 `register` / 5 `protocol` / 4 unclassified. Read-only throughout; no artifact rebuilt or mutated |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SIGNAL-CATALOG-CAPTURE-GAP.0` | `SIGNAL-CATALOG-CAPTURE-GAP.0 — own the documents that declare no signals at all` | ownership and census only, no code |
| `SIGNAL-CATALOG-CAPTURE-GAP.1` | `SIGNAL-CATALOG-CAPTURE-GAP.1 — 32 honest absences, one capture miss, and the escape that truncates identifiers` | classification only, no code |

## Changelog

- `2026-08-11`: created from a measured finding in `SEMANTIC-EMPTY-CATALOG-FILTER.1`, whose demotion packets
  made the missing catalogs visible per document. Ownership and census only; no code change.
- `2026-08-11` (`.1`): **no `CHANGES.md` entry was written**, deliberately. A ~31-line entry took the ledger to
  1,629 lines = 90.5% of its 1,800-line health target, tripping the mandatory rolling-ledger rollover. Trimming
  the entry to squeak under 1,620 would game a health gate that exists precisely to force the rollover, so the
  entry is deferred into the rollover slice; its content is fully recoverable from this section and from
  [the measurement](../research/empty-signal-catalog-classification.md). The gate was left green, not bypassed.
- `2026-08-11` (`.1`): classified all 33. The prima-facie hypothesis that a `protocol`-classed document with
  zero signals is a probable miss survives for one document of five, so `.2` narrowed from a search across
  several documents to a design question about one convention. Split out `.4` (the markdown-escape truncation,
  which turned out to be a corpus-wide defect independent of this tree's subject) and `.5` (14 evidence
  artifacts with no validation report). Corrected the `.0` reproducer and its table count. No code change.
