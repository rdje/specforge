# PDF-VARIANT-DIGESTION: make SpecForge digest as many chip-spec PDF variants as possible

## Metadata

- Tree ID: `PDF-VARIANT-DIGESTION`
- Status: `active` (high-priority program)
- Roadmap lane: `R15`/`R16` (ingestion + extraction breadth)
- Created: `2026-06-07`
- Parent: owner high-priority directive — "make SPECFORGE capable to handle any chip-spec PDF we can throw
  at it, that's the dream, the aim … we should do everything we can to get as close as possible." Defer
  IntentIR→ISF lowering. (`project_pdf_variant_digestion` memory.)

## Goal (the aim — this is the ORIGINAL roadmap goal, not a pivot)

Per the README objective, SpecForge extracts intent from "**protocol, component, system, and
software-interface specifications**" into a backend-independent `IntentIR` — i.e. **any chip-spec PDF** has
always been the goal (forward specification mining). AMBA / `WIRE-BASED-100` was the FIRST *measured* class
(a starting point to prove + per-fact-harden the extraction), never the target. This program pursues that
original goal at full breadth. SpecForge ingests and meaningfully extracts intent from **any chip-spec
PDF** — across every vendor, doc type, and layout. The concrete near-term target is the **82-PDF library** (ARM AMBA/debug/system-IP/ISA/
TRMs, CXL/OpenCAPI/CCIX, USB-IF, RISC-V, Intel/AMD, JEDEC, NXP, NVMe, OpenCores) plus whatever the owner
adds. "Digest" = ingest cleanly (no crash / handle protection) AND produce non-garbage typed extraction
appropriate to the doc (signals/constraints/relations/temporal/FSM/registers/encodings), or an honest
diagnostic when a construct is genuinely out of model.

## Non-goals

- NOT IntentIR→ISF lowering (deferred by the owner; revisit later).
- Do NOT regress the four wire-based specs: APB/AHB/AXI = 100% (constraints/relations/temporal),
  SWD = 100% (frame/operations/FSM). kg-bench stays green.
- Not perfect extraction of every fact in a 1000-page ISA manual — meaningful, measured progress per class.

## Acceptance criteria

- A triage matrix over the corpus: per PDF, ingest status + extraction stats (tables/kinds/signals/
  constraints/relations) — so coverage is measured, not guessed.
- Each addressed variant CLASS: a general feature (ADR-0006-safe, no hardcoded chip names) that unlocks it,
  + a regression PDF copied into `corpus/` + git-tracked, + per-fact measurement where a gold applies.
- No regression on the four wire-based specs; full `scripts/run_ci.sh` green; KM card per durable finding.

## Task tree

- ID: `PDF-VARIANT-DIGESTION` · Status: `active` · Children: `.1` (triage) + per-class feature leaves (`.2`–`.8`) + `.9` (new serial-protocol class)
- ID: `PDF-VARIANT-DIGESTION.9` · Status: `active` · Goal: **digest the new serial-protocol class** — the
  owner downloaded new serial chip-spec PDFs (CAN, SWP, SMBus, I2S) and directed they be copied + git-tracked
  into SpecForge. Import them, then ingest → extract → measure each, extending breadth+precision onto a
  never-seen serial-protocol class (the SWD/ADI `SerialFrameField` + `ProtocolStateRecord` surfaces are the
  relevant machinery). Honest baselines first (read where the fact lives or report a residual; ADR 0006;
  APB/AHB/AXI/SWD stay 100%). Children: `.9.1`, `.9.2`, `.9.3` (CAN frame), `.9.4` (SWP baseline), `.9.5` (SMBus),
  `.9.6` (I2S), `.9.7` (single-word FSM grammar), `.9.8` (SWP prose signals), `.9.9` (constraint-acronym precision).
- ID: `PDF-VARIANT-DIGESTION.9.1` · Status: `done` · Goal: import the selected serial specs
  (CAN/SWP/SMBus/I2S) into `corpus/<vendor>/.../current/` + `SOURCE_PDF_REGISTRY.md`, git-tracked, so a
  re-ingest is always reproducible (owner directive `2026-06-08`: "selected ones shall be copied and git
  tracked in SPECFORGE repo"; [[feedback_source_pdfs_in_repo]] — do NOT record the owner's library path).
  Acceptance: the PDFs copied under canonical vendor paths, registered, `git add`-tracked, each verified to
  begin with a real PDF header. Verification: 4 PDFs copied + registered + `%PDF-`-verified; memory-arch + KM
  hooks green. Commit: `c265062b` (`PDF-VARIANT-DIGESTION.9.1`).
- ID: `PDF-VARIANT-DIGESTION.9.2` · Status: `done` · Goal: **CAN 2.0 honest baseline** — ingest
  (`DOCLING_DEVICE=cpu`) → evidence → validate on a never-seen serial protocol; record the document_class, the
  class-aware completeness gauge, and the serial-frame / protocol-FSM / signal-inventory yield; state the
  honest extraction gap (or honest "already digests well") and spin the next leaf from it. No fabrication (ADR
  0006). Acceptance: baseline metrics recorded in this tree (+ KM card if a durable fact emerges); APB/AHB/AXI/SWD
  unaffected. Verification: clean ingest (72 pages / 98 visual assets / `ready` / 0 residuals); evidence 269
  anchors / 751 statements / 98 visual / 0 KG relations. `validate` baseline = **`document_class: guide` but
  `document_type_declared: specification` → ⚠ UNDER-EXTRACTED spec** (the `.5a`/`.5c` machinery fires correctly):
  deterministic yield **0 signals / 0 registers / 0 relations / 0 constraints**, 12 narrative conditional rules,
  98 un-enriched figures, 19 prose residuals. **Honesty-guardrail confirmed the facts ARE present in prose** —
  frame fields (START OF FRAME 15× / Arbitration 7× / Control 6× / Data 14× / CRC 49× / ACK 48× / EOF 12× / DLC
  15× / IDENTIFIER 32× / RTR 14×) and the error-state FSM (error-active 8× / error-passive 19× / bus-off 8× /
  fault-confinement 22× / TEC 110× / REC 144×). Root: `extract_serial_frame_fields`/`extract_protocol_states`
  (evidence.rs) were tuned to SWD prose and don't generalize to CAN's frame-field-heading + FSM prose → lever
  `.9.3`. Commit: pending (this slice).
- ID: `PDF-VARIANT-DIGESTION.9.3` · Status: `active` · Goal: **CAN prose serial-frame + error-state-FSM
  extraction lever** — generalize the SWD-tuned prose extractors (AGNOSTIC, ADR 0006 — no `CAN`/`SOF`/`bus-off`
  literals; derive from the document's own grammar) so CAN's serial frame + error-state FSM surface as typed
  `SerialFrameField` / `ProtocolStateRecord`. Split into `.9.3a` (FSM, cleaner/more universal — do first) +
  `.9.3b` (frame fields). Children: `.9.3a`, `.9.3b`.
- ID: `PDF-VARIANT-DIGESTION.9.3a` · Status: `done` · Goal: **CAN error-state FSM via a new AGNOSTIC
  quoted-mode-state extractor.** The existing `extract_protocol_states`/`find_states_with_actions` require the
  SWD/JTAG shape (`Capitalized-Hyphen state`) and a TAP/scan-chain doc-gate → 0 on CAN. Add a new additive path
  `extract_quoted_mode_states` keyed on a universal FSM grammar: a single-quoted name (1–3 words) bound to a
  GENERIC actor-noun subject (node/unit/station/device) via adjective form (`'<name>' <actor>`) or predicate
  form (`<actor> is|are|be|becomes|become '<name>'`), with recurrence ≥2 and ≥2 distinct states (an FSM has
  multiple states). This naturally excludes quoted bit-values (`'dominant'`/`'recessive'`, subject = *bit*) and
  bus conditions (`'bus idle'`, subject = *bus*). Capture the `when <condition>` transition clause as the
  state's `action` when present; normalize internal whitespace; accept ASCII + typographic quotes. Acceptance:
  CAN recovers `error active` / `error passive` / `bus off` as `ProtocolStateRecord`s (fresh evidence re-measure),
  ZERO fabrication (only quoted node-modes the prose states), **SWD/ADI FSM extraction unchanged** (additive +
  deduped), an in-corpus non-FSM doc (I2C) stays 0 protocol states; hermetic positive + negative tests; full
  `run_ci.sh` green + kg-bench 151/151; book (`pipeline/evidenceir.md`) + KM refreshed. Verification: new
  `extract_quoted_mode_states`/`quoted_mode_states_in`/`is_quoted_mode_state_name` (additive, deduped by name
  after the SWD paths). **Live re-measure: CAN 0 → 3 states** (`error active` / `error passive` / `bus off`,
  supports 8/18/4, honest `when …` actions), ZERO spurious. **No regression:** NVMe / I2C / RISC-V Debug stay 0
  protocol_states; **re-ingested ADI/SWD gains 0 `mode_state_*`** and keeps its 13 SWD/JTAG states intact. 5 new
  hermetic tests (positive + 4 negative) + 35 SWD tests pass; full `run_ci.sh` GREEN (fmt + clippy `-D warnings`
  + lib 1440 → 1445 + rustdoc + docs); kg-bench 151/151; KM card `agnostic-quoted-mode-fsm` + book subsection.
  Commit: `3d85543c`.
- ID: `PDF-VARIANT-DIGESTION.9.3b` · Status: `in_progress` · Goal: **CAN serial frame fields.** Capture CAN's
  named frame-field sequence (SOF → Arbitration → Control → Data → CRC → ACK → EOF) agnostically.
  **Design (probe-locked `2026-06-09` over all 76 persisted evidence docs BEFORE coding):** CAN names its frame
  in a **composition list** — "A DATA FRAME is composed of seven different bit fields: START OF FRAME,
  ARBITRATION FIELD, CONTROL FIELD, DATA FIELD, CRC FIELD, ACK FIELD, END OF FRAME" — with per-field widths in
  scattered prose ("CONTROL FIELD consists of six bits", "ACK FIELD is two bits long"). The `SerialFrameField`
  surface FITS without change: `phase` is optional (CAN's 7-field frame needn't use SWD's request/ack/data
  phases → `None`), `order` holds the composition sequence, `bit_width` is optional. New additive
  `extract_composition_frame_fields`: (1) the composition list SCOPES which fields are captured (so scattered
  "N bits" mentions of non-frame items — ERROR FLAG / OVERLOAD DELIMITER / INTERMISSION — are excluded); (2) a
  width is recorded ONLY when the field name is the direct subject of a **plural** "<num> bits" count, NEVER
  when "<num> bit" modifies a sub-field ("the 11 bit IDENTIFIER" → ARBITRATION FIELD stays `None`, not a wrong
  12-vs-11) — the HONESTY GUARDRAIL: residual over fabrication. Multi-word ALL-CAPS field names; number-word +
  digit widths; ADR 0006 (grammar, no names). **Probe result: only CAN fires (0 corpus false positives); 7
  ordered fields; widths CONTROL=6, ACK=2, the rest honest `None`** (SOF anaphoric, DATA variable, CRC/EOF
  stated only for sub-parts). Additive (separate fn, disjoint from the SWD `is_serial_doc` path: CAN lacks the
  SWD markers, SWD lacks the composition shape) → APB/AHB/AXI/SWD untouched. **Status: `done` (`2026-06-09`).**
  Implemented as `extract_composition_frame_fields` + `is_frame_field_name` + `stated_frame_field_bit_width` +
  `parse_count_word` (`ir/evidence.rs`), merged after the SWD path deduped by name. **Verification (fresh CAN
  evidence rebuild off persisted source_ir, no Docling re-ingest): CAN `serial_frame_fields` 0 → 7 ordered**
  (`START OF FRAME`/`ARBITRATION FIELD`/`CONTROL FIELD`=6/`DATA FIELD`/`CRC FIELD`/`ACK FIELD`=2/`END OF FRAME`,
  order 0–6; widths only the two directly-stated, the rest honest `None`). **No regression — SWD/ADI keeps its
  11 SWD-path frame fields, AXI 0** (corpus probe: only CAN fires). 4 new hermetic tests; `scripts/run_ci.sh`
  GREEN (fmt + clippy `-D warnings` + lib **1472** + rustdoc + mdBook) + kg-bench 151/151. KM
  `can-composition-frame-fields`; book subsection in `pipeline/evidenceir.md`. Commit: pending (this slice).
- ID: `PDF-VARIANT-DIGESTION.9.4` · Status: `done` · Goal: **SWP (Single Wire Protocol) honest
  baseline** — ingest (`DOCLING_DEVICE=cpu`) → evidence → validate; record document_class, completeness gauge,
  and the signal / FSM / frame yield (including whether the new `.9.3a` quoted-mode FSM lever already fires on
  SWP); state the honest gap and spin the next lever. No fabrication (ADR 0006). Acceptance: baseline metrics
  recorded in this tree; APB/AHB/AXI/SWD unaffected. Verification: ingested (147 anchors / 1303 statements / 82
  visual); `document_class: protocol` (11 signal_constraints), but **0 signals / 0 FSM / 0 frame / 0 actors /
  0 relations** and the `.9.3a` quoted-mode lever does NOT fire (SWP doesn't quote its states). **Honest finding
  — SWP under-extracts DIFFERENTLY than CAN, and the facts ARE present:** (1) SWP HAS an FSM (`ACTIVATED state`
  ×8, `DEACTIVATED state` ×10, `Reset State` ×2, `S1 state`) but it's a **single capitalized word + "state"**
  grammar that neither the SWD path (`looks_like_state_name` requires hyphen/slash) nor `.9.3a` (requires quotes)
  captures → a THIRD FSM grammar lever; (2) SWP HAS single-wire signals (`S1` ×56, `S2` ×41, `SWIO` ×38) in
  prose, not a table → 0 captured → a prose-signal lever; (3) the 11 "signal_constraints" are mostly NOISE —
  layer/protocol acronyms (`UICC`/`SWP`/`CLF`/`SHDLC`/`RSET`/`CLT`) mis-captured as signal subjects → a
  constraint-subject precision lever. Spun future leaves `.9.7`–`.9.9` (below). Commit: pending (this slice).
- ID: `PDF-VARIANT-DIGESTION.9.5` · Status: `done` (`2026-06-09`) · Goal: **SMBus 3.3.1 honest baseline** —
  ingest (`DOCLING_DEVICE=cpu`) → evidence → validate the never-before-ingested SMBus spec; record the
  document_class, the class-aware completeness gauge, and the signal / FSM / frame / actor / relation yield
  (including whether the `.9.3a` quoted-mode, `.9.7` transition-bound, and `.9.8` definitional-signal levers
  already fire on SMBus); state the honest gap and spin the next lever. No fabrication (ADR 0006). Acceptance:
  baseline metrics recorded in this tree; APB/AHB/AXI/SWD unaffected (docs-only slice — no code change).
  **Verification:** clean ingest (83 pages / 100 visual assets / `ready` / 0 residuals / `document_key:
  smbus_3_3_1_2024_10_20_system_management_bus_specification`); evidence = 139 section anchors / 1076 spans / 100
  visual / 163 links / 1131 statements. `validate` → **`document_class: guide`, `document_type_declared:
  specification` → ⚠ UNDER-EXTRACTED spec** (the `.5a`/`.5c` machinery fires correctly, exactly as on CAN):
  deterministic yield **1 real signal (`SMBCLK`) + 1 NOISE (`WIRE`)** [validate counts "2 signals"], **0 protocol_states /
  0 serial_frame_fields / 0 protocol_actors / 0 actor_signal_relations / 0 signal_constraints**, 26 narrative
  conditional rules, **84 timing_constraints (from tables — STRONG)**, 1 register_record (21 fields, unnamed),
  1 signal_polarity (`SMBCLK` active_low), 1 signal_semantic_hint (`SMBCLK` ready-like, dubious — derived from a
  STOP-condition definition); completeness gauge `not applicable` (guide); 3 unexplained intent-bearing tables
  (`table_0012` timing_parameter, `table_0022` **signal_description** → produced no record, `table_0042`
  register_map); 100 un-enriched figures; convergence converged in 2 passes (+52 new facts). **Honest finding —
  SMBus under-extracts DIFFERENTLY AGAIN, and the facts ARE present:** (1) SMBus has **no signal table** (the
  `| Signal |`/`| Pin |` header grep is empty) — its bus signals live ONLY in PROSE: `SMBCLK` (40×) / `SMBDAT`
  (32×) as definite-article/collective "the SMBCLK line", "Both SMBCLK and SMBDAT lines are bi-directional",
  plus the optional `SMBSUS#` (12×) / `SMBALERT#` (16×) as "`SMBSUS#` is an optional signal" / "`SMBALERT#` is a
  wired-AND signal". Only `SMBCLK` was captured (1/≥4 recall); `SMBDAT`/`SMBSUS#`/`SMBALERT#` missed. (2) The
  `.9.8` definitional copula (`<NAME> is a/an signal`) does NOT fire here because the `#` active-low suffix +
  the descriptor word between "a/an" and "signal" ("optional", "wired-AND") both fall outside its grammar — so a
  NEW prose-signal lever is genuinely needed (probe-locked, not a re-tune). (3) `WIRE` is an honest false
  positive from "two-**wire** bus" / "**wired**-AND" — a precision blemish, not a fabrication. (4) actors
  "controller" (296×) / "target" (280×) are heavily present but 0 protocol_actors → a prose-actor gap on the
  "bus controller"/"bus target" idiom. Spun future lever `.9.10` (below). Commit: pending (this slice).
- ID: `PDF-VARIANT-DIGESTION.9.6` · Status: `done` (`2026-06-09`) · Goal: **I2S (NXP UM11732) honest baseline** —
  ingest (`DOCLING_DEVICE=cpu`) → evidence → validate the never-before-ingested NXP I2S bus spec; record the
  document_class, completeness gauge, and the signal / FSM / actor / relation / timing yield; state the honest
  gap and spin the next lever. No fabrication (ADR 0006). Acceptance: baseline metrics recorded in this tree;
  APB/AHB/AXI/SWD unaffected (docs-only slice — no code change). **Verification:** clean ingest (14 pages / 27
  visual assets / `ready` / 0 residuals / `document_key: um11732_v3_2022_02_17_i2s_bus_specification`); evidence =
  24 section anchors / 152 spans / 27 visual / 14 links / 154 statements. `validate` → **`document_class: guide`,
  `document_type_declared: specification` → ⚠ UNDER-EXTRACTED spec** (the `.5a`/`.5c` machinery fires correctly,
  as on CAN/SMBus). Deterministic yield: **2 real signals (`SCK` + `SD`)** captured from prose — but `WS` (word
  select, the most-referenced signal at 11×) MISSED; **1 protocol_actor** (the `.8` prose-actor grammar fires →
  "controller", from "the device generating SCK and WS is the controller" — better than SMBus's 0); 0
  protocol_states / 0 serial_frame_fields / 0 actor_signal_relations / 0 signal_constraints / 0 register_records;
  **1 conditional_rule which is NXP LEGAL-BOILERPLATE NOISE** ("otherwise agreed in a valid written individual
  agreement …", correctly excluded from the doc-class decision); **0 timing_constraints despite 2 intent-bearing
  `timing_parameter` tables** (`table_0004` / `table_0005`) → produced no record; 0 signal_semantic_hints / 0
  signal_polarities; completeness gauge `not applicable` (guide). **Honest finding — I2S under-extracts
  DIFFERENTLY AGAIN, and the facts ARE present:** (1) I2S has **no signal table** — its 3 lines (`SCK` serial
  clock 5×, `WS` word-select 11×, `SD` serial data 2×) are declared in prose; `SCK`/`SD` were captured but `WS`
  was MISSED (a prose-bus-line recall quirk — likely the 2-letter all-caps token vs the parenthetical/defining
  construction — to be probed under `.9.10`'s family). (2) The 2 `timing_parameter` tables yield 0 records
  because their **leading parameter/symbol column header is BLANK** (`|  | MIN | TYP | MAX | CONDITION |`), so no
  symbol anchors the row (contrast: SMBus's timing tables yielded 84) → a blank-leading-column timing-table
  recovery lever `.9.11`. (3) the lone conditional rule is legal boilerplate, not protocol semantics. Spun
  future lever `.9.11` (below); `WS` recall folded into `.9.10`'s probe scope. Commit: pending (this slice).
- ID: `PDF-VARIANT-DIGESTION.9.11` · Status: `done` (`2026-06-09`) · Goal: **recover
  timing-parameter tables whose data rows are trapped in `header_rows`** so they yield typed
  `timing_constraints` instead of 0 records — AGNOSTICALLY (ADR 0006; structure only, no name list, no case
  dependence) and without regressing the tables that already work. **PROBE (`2026-06-09`, root cause LOCKED — it
  is NOT a blank-header problem):** `synthesize_timing_constraints` already defaults `name_col` to 0, so a blank
  leading header is fine. The real blocker is upstream: in I2S `table_0004` (and SMBus `table_0012`) the data
  rows are misclassified into `header_rows` because Docling marks the row-LABEL cell `is_header=true` (e.g.
  `["clock period T"(hdr), "360", "400", "440", "T tr = 360"]`), leaving `body_rows` EMPTY → the
  `body_rows.is_empty()` guard skips the whole table. **Structural discriminator (list-free, case-free):** a
  `header_rows` entry PAST the first column-header row whose VALUE cells are `is_header=false` is a trapped DATA
  row; a genuine multi-row column header (SMBus `table_0011`/`0013` 2nd header row; I2S `table_0005`'s nested
  TRANSMITTER/RECEIVER × LOWER/UPPER cross-tab with `is_header=true` value cells) is NOT → stays an honest
  residual (no fabrication). **Fix:** in `synthesize_timing_constraints`, build the effective data-row set =
  `body_rows` + recovered trapped header-rows (header_rows after [0] whose non-label cells are `is_header=false`),
  then process as today (name_col=0). Recovers I2S `table_0004` AND SMBus `table_0012` (bonus); SMBus's working
  tables keep their `body_rows` untouched; the nested I2S `table_0005` stays a residual. Acceptance: I2S 0 → N
  timing_constraints; SMBus ≥ 84 (no regression, only genuine gains); wire-based unaffected; hermetic tests
  (trapped-row recovery + nested-header residual + normal-body unchanged); full `run_ci.sh` + kg-bench; book + KM.
  Spun from the `.9.6` I2S baseline.
  **Verification (`2026-06-09`):** implemented as an additive structural recovery in `synthesize_timing_constraints`
  (`ir/evidence.rs`) — `effective_rows = body_rows + header_rows[1..] filtered to data-shaped rows`
  (`len ≥ 2 && first cell non-empty && all value cells is_header=false`), then the existing name/min/typ/max
  extraction (name_col already defaults to 0). **Live re-measure (fresh evidence rebuild off persisted
  source_ir): I2S `timing_constraints` 0 → 5** (`clock period T` 360/400/440, `clock HIGH t HC` min 110,
  `clock LOW t LC` min 110, `set-up time t sr` min 60, `hold time t htr` min 0 — empty value cells stay `None`,
  not fabricated). **No regression: SMBus held at 84** (its `table_0012` shape doesn't match the data-row test →
  stays an honest residual, never fabricated; nested I2S `table_0005` likewise residual). 3 new hermetic tests
  (trapped-row recovery / nested-header residual / normal-body unchanged). Full `scripts/run_ci.sh` GREEN (fmt +
  clippy `-D warnings` + lib **1472 → 1475** + rustdoc + mdBook) + kg-bench 151/151 (no eval/fixture regressed →
  wire-based + SWD unaffected). KM `timing-table-trapped-row-recovery`; book subsection in
  `pipeline/evidenceir.md`. Commit: pending (this slice).
- ID: `PDF-VARIANT-DIGESTION.9.12` · Status: `done` (CLOSED, investigate-only no-build) (`2026-06-09`) · Goal:
  **triage the remaining serial-class `unexplained_intent_bearing_tables` and decide build-vs-residual honestly.**
  The SMBus + I2S baselines flagged tables that "produced no record"; the conservative region-accounting can't
  tell a real miss from a table with nothing typed to extract. Probe-checked each (read the persisted
  `source_ir` structure, no code): **(1) SMBus `table_0022` (signal_description)** is a degenerate 2-cell
  bitfield fragment (`MSB`/`LSB` under `Supported Protocols` / `SMBus Version Bits [3:0]`) — nothing real to
  extract; **(2) SMBus `table_0042` (register_map)** is an ADDRESS-ASSIGNMENT table (`Target Address [7:1] |
  R/W# | Description | Specification`), not a register-field layout — forcing it into register records would
  fabricate; **(3) I2S `table_0005`** is a 3-level nested cross-tab (`TRANSMITTER`/`RECEIVER` × `LOWER`/`UPPER
  LIMIT` × `MIN`/`MAX`, parameter in col 1, wrapping prose in col 0, mostly-empty value cells) whose core
  parameters are ALREADY captured from the simpler `table_0004` by `.9.11`. **Decision: do NOT build — all three
  are HONEST RESIDUALS** (residual over fabrication; the region-accounting flag is conservative by design). The
  serial-class STRUCTURAL table levers are now exhausted (the buildable one, `.9.11` timing, is done); the only
  remaining serial gaps are PROSE-signal capture (`.9.10`/`.9.8b`), which are parked behind participation-based
  signal identity (`NLP-SHALLOW-PARSE`), not a list/case crutch. Docs-only (no code). Commit: pending (this slice).
- ID: `PDF-VARIANT-DIGESTION.9.10` · Status: `in_progress` (probe DONE `2026-06-09`; implementation gated on one
  owner decision — see below) · Goal: **SMBus-class prose bus-line signal grammar** —
  recover I2C/SMBus-derived 2-wire bus signals declared ONLY in prose (no signal table) — `SMBCLK`/`SMBDAT` as
  definite-article "the `<NAME>` line" / collective "`<NAME>` and `<NAME>` lines are <property>", and the
  active-low optional signals `SMBSUS#`/`SMBALERT#` as "`<NAME>#` is a/an <descriptor> signal" — AGNOSTICALLY
  (ADR 0006: derive the names, never list them) and without SWD/CAN/SWP/wire-based regression or corpus false
  positives, dropping the `WIRE` two-wire/wired-AND false positive. **Must be probe-locked over all persisted
  evidence docs BEFORE coding** (the `.9.7`/`.9.8` methodology) — `.9.8` already PROVED descriptor-apposition is
  corpus-toxic, so the descriptor-tolerant "is a/an X signal" form needs careful gating (the `#` suffix + a
  bounded descriptor allowlist or all-caps-identifier subject test). Honesty guardrail: residual over
  fabrication. Also in scope (from the `.9.6` I2S baseline): I2S's `WS` (word select, 11×) is a prose bus-line
  signal that was MISSED while `SCK`/`SD` from the same doc were captured — the probe must explain that asymmetry
  (likely the 2-letter all-caps token or the specific defining construction) and recover `WS` without
  fabrication. Spun from the `.9.5` SMBus baseline (extended by `.9.6`).
  **PROBE (`2026-06-09`, faithful — ran the candidate grammars over ALL 78 persisted `evidence_ir`
  statement-text corpora, the `.9.7`/`.9.8` method; NOT the 8 surviving normalized markdowns):**
  - **FORM A — "the/The `<NAME>` line" (case-insensitive article, all-caps identifier, optional trailing `#`):
    the WINNER.** Fires on only **5/78 docs**, all 2-wire/serial buses, and recovers exactly the missed signals:
    SMBus → `SMBCLK`/`SMBDAT`/`SMBSUS#`; I2S → `WS` (the article was capitalized — "The WS line" — which a
    case-sensitive probe would have missed); plus bonus real signals on I2C (`SCL`/`SCLH`/`SDA`/`USCL`/`USDA`)
    and eMMC (`CMD`/`DAT`/`DAT0`). **Crucially it fires on ZERO wire-based docs (APB/AHB/AXI/SWD → NONE), so the
    wire-based byte-identical invariant is structurally preserved.**
  - **FORM B — "`<NAME>` and `<NAME>` lines" — a redundant subset of A** (SMBus → `SMBCLK`/`SMBDAT`); optional.
  - **FORM C — "`<NAME>` is a/an [descriptor] signal" — REJECTED.** It fires on the WIRE-BASED docs (AXI →
    `BRESP`/`RRESP`/`AWAKEUP`; APB → `PCLK`/`PPROT`/`PSLVERR`/`PSTRB`; AHB → `HWSTRB`), so it would change their
    output and break their 100% byte-identical guarantee. (This is why SMBus's `SMBSUS#`/`SMBALERT#` "is a/an …
    signal" definitions must NOT be harvested by a general copula — only the `#`-suffixed, `line`-anchored Form A.)
  - **OPEN DESIGN FORK (gates the build):** Form A also captures **power-supply rails `VDD`/`VSS`** ("the VDD
    line") on I2C/eMMC, and a stray `DLEN` on I2C. I2C is a **measured** doc (declared-signal gold precision
    0.600), so admitting `VDD`/`VSS`/`DLEN` as signals would REGRESS a tracked score (scoring rigor — owner
    non-negotiable). The agnostic fix is to extend the EXISTING universal-term denylist `is_signal_synthesis_non_signal`
    (which already holds universal hardware vocabulary `CLOCK`/`RESET`/`PORT`/`PIN` — not chip names) with the
    universal supply-rail set (`VDD`/`VSS`/`VCC`/`GND`/`VBAT`/…). That is consistent with the `CLOCK`/`RESET`
    precedent and ADR 0006's "universal how, not a name" boundary — but because it touches a non-negotiable
    (ADR 0006 + a measured score), the owner's explicit steer is requested before coding.
  **Plan once decided:** add Form A as a 4th additive form in `synthesize_signal_declarations_from_prose` (under
  the same `enable_parenthetical` sparse-catalog gate), via `is_hardware_signal_token` (strip a trailing `#`
  for the check, keep it in the emitted name) + `is_signal_synthesis_non_signal` (extended); hermetic tests
  (SMBus `SMBCLK`/`SMBDAT`/`SMBSUS#`, I2S `WS`, wire-based stays 0, power-rail excluded); full `run_ci.sh` +
  kg-bench; re-measure I2C precision to PROVE no regression; book + KM. Honesty guardrail: residual over
  fabrication.
- ID: `PDF-VARIANT-DIGESTION.9.7` · Status: `in_progress` · Goal: **single-word `<NAME> state` FSM grammar** —
  generalize the SWD `<Name> state` path (`looks_like_state_name` + the TAP/scan-chain doc-gate) to also accept a
  single capitalized state word (`ACTIVATED state`, `DEACTIVATED state`) behind a safe generic FSM doc-gate, so
  SWP-class state machines surface, AGNOSTICALLY (ADR 0006) and without SWD/CAN regression or corpus false
  positives.
  **Design (empirically locked `2026-06-09` by probing the proposed grammar over ALL 80 persisted evidence
  docs BEFORE coding — no guessing):** the discriminator that makes single-word matching safe is a
  **transition/locative binding** (the analogue of `.9.3a`'s actor-noun binding): a state is an **all-caps**
  token (≥2 chars, ≥1 letter, hyphens allowed) sitting in `<TRIGGER> [the|a|an] <NAME> state`, where TRIGGER ∈
  {enter(s)/into/leave(s)/exit(s)/to/in/from/reach(es)/remain(s)/stay(s)/move(s)/transition(s)/return(s)/
  put(s)/place(s)} — i.e. a state one ENTERS / EXITS / is IN. Two self-gates (same as `.9.3a`, NOT a keyword
  gate): each name must **recur in ≥2 statements** and a doc must yield **≥2 distinct** such states (an FSM has
  several states). A **keyword doc-gate ("state machine"/"FSM") was REJECTED** because SWP never uses those
  phrases (0 hits) — the structural self-gate IS the "safe generic FSM gate". Defensive guards: an
  after-token guard drops `<X> state machine|diagram` (X names the machine, not a state), a logic-level/booleans
  denylist (HIGH/LOW/ON/OFF/SET/CLEAR/TRUE/FALSE/…), and ARM's architectural pseudo-values UNKNOWN/UNPREDICTABLE
  (universal spec vocabulary, never states; ADR 0006-safe). Realized as an **additive sibling**
  `extract_transition_bound_states` (deduped by name after the existing three paths) so the SWD/CAN extractors
  are byte-for-byte untouched → zero regression by construction (the `.9.3a` engineering choice). **Probe
  result:** SWP → `DEACTIVATED/ACTIVATED/SUSPENDED/HALT`; SWD/ADI → 0 (only `UNKNOWN` reaches ≥2 supports, 1
  distinct < 2 — and now denylisted anyway); CAN → 0 from this path (keeps its 3 quoted-mode states);
  APB → `SETUP/ACCESS` and AXI → `STOP/ACTIVATE/DEACTIVATE` (their REAL operating / low-power Q-Channel
  states — honest improvement, no scored-metric change, no kg-bench `protocol_states` fixture exists); 18 docs
  total gain a genuine FSM surface (CHI/CXS/DTI/CCIX/CoreSight/eMMC/USB4).
  **Status: `done` (`2026-06-09`).** Implemented as the additive sibling `extract_transition_bound_states` +
  `transition_bound_state_names_in` + `is_bare_state_name` (evidence.rs), wired + deduped after the three
  existing FSM paths. **Live re-measure on fresh-rebuilt evidence: SWP `protocol_states` 0 → 4 `named_state_*`
  (`DEACTIVATED`/`ACTIVATED`/`SUSPENDED`/`HALT`).** No regression: **SWD/ADI** keeps its 13 states (8
  `protocol_state_*` + 5 `swd_line_state_*`), 0 `named_state_*` → SWD derivation eval `serial_frame_field` /
  `swd_operation` / `protocol_state` all `P=R=F1=1.000`; **CAN** keeps its 3 `mode_state_*`, 0 `named_state_*`.
  8 new hermetic tests (positive SWP, after-guard machine/diagram, requires-binding, single-state,
  non-recurring, lowercase/logic-level/pseudo-value rejection, parallel-bus real-state capture,
  `is_bare_state_name` unit). Full `scripts/run_ci.sh` GREEN (fmt + clippy `-D warnings` + lib 1440 → 1448 +
  rustdoc + mdBook) + kg-bench 151/151. Book subsection in `pipeline/evidenceir.md`; KM card
  `transition-bound-state-fsm` (KM 45 → 46 facts). Commit: pending (this slice).
- ID: `PDF-VARIANT-DIGESTION.9.8` · Status: `done` (`2026-06-09`) · Goal: **SWP prose single-wire signal
  capture** — recover `S1`/`S2`/`SWIO`-class single-wire signals defined in prose (not in a signal table),
  agnostically.
  **Design (empirically LOCKED `2026-06-09` by probing candidate grammars over ALL 80 persisted evidence docs
  BEFORE coding — the `.9.7` methodology):** SWP names its signals only in PROSE, in four observed forms —
  (1) copular DEFINITION "S1 is a signal in the voltage domain …", "S2 is a signal in the current domain …";
  (2) glossary colon "S1: signal from the master to a slave", "S2: signal from the slave to the master";
  (3) descriptor apposition "the signal S1" / "Signal S2" / "the SWIO signal"; (4) abbreviation-table
  expansion "SWIO | Single Wire protocol Input/Output". The probe DECIDED among them:
  - **descriptor apposition (`signal <NAME>` / `<NAME> signal`)** — REJECTED, corpus-toxic: bare `signal X`
    yields IS/TO/NAMES/FROM/CONTROL/DATA across 30-47 docs; even a recurrence-self-gated variant floods
    SWD/ADI (sparse-catalog, runs prose capture) with `AP/APB/ARM/DATA/OF/JTAG/HPROT/…`, which would regress
    SWD's WIRE-BASED-100 100% (declared signals gate constraint/relation subjects). No.
  - **abbreviation-expansion (I/O marker)** — REJECTED, not corpus-clean: gets SWIO + eMMC's real `DAT1-7`/
    `CMD`, but also `MMIO`(7 docs)/`MEM`/`DMA`/`IOVA`/`IOTLB` (memory/addressing concepts whose expansion
    merely contains "input/output"). No (a separate `.9.8b`/`.9.9`-adjacent surface if ever wanted).
  - **definitional (copula + glossary-colon)** — ACCEPTED: with the candidate required to be an all-uppercase
    identifier token (`is_hardware_signal_token` on the ORIGINAL token, so lowercase English subjects like
    "an interrupt is a signal" / "it is a signal" can never qualify) the probe yields, corpus-wide over all
    persisted evidence docs, EXACTLY `S1` + `S2` and ZERO garbage. This IS the SWP signal capture: SWP has
    exactly two signals — S1 (voltage, master→slave) and S2 (current, slave→master) — both carried on the
    shared **SWIO contact (C6)**. **SWIO is the physical contact, not a third logical signal** (lines 847/879:
    "S1 shares the same electrical contact as S2 … (contact C6)"), and has no corpus-clean definitional form →
    honest residual, never fabricated (HONESTY GUARDRAIL). Realized as a third additive form inside
    `synthesize_signal_declarations_from_prose`, under the existing sparse-catalog fallback gate, so APB/AHB/
    AXI (table-rich, never run prose) and SWD (sparse, but probe adds 0) are untouched. ADR 0006 (grammar, no
    names). **Verification (`2026-06-09`): SWP declared signals 0 → `S1`/`S2`** (fresh evidence rebuild off
    the intact persisted SWP `source_ir`, no Docling re-ingest). **No regression — proven by a before/after
    `git stash` diff: SWD/ADI declared set BYTE-IDENTICAL** with vs without `.9.8` (`NSRST`/`SWCLK`/`SWDIO`/
    `TCK`/`TDI`/`TDO`/`DBGTDI`/`DBGTDO`/`DBGTMS`/`NSRSTOUT`/`PORTCONNECTED`/`CSYSPWRUPACK` unchanged — the
    pre-existing `LEVEL`/`level` noise is NOT from `.9.8` and is out of scope). Corpus-wide the new grammar
    adds ONLY `S1`/`S2` (probe over all persisted evidence docs). 7 new hermetic tests (copula+colon capture,
    lowercase-subject rejection, non-identifier colon-head rejection, non-signal-predicate ignore, denylist,
    sparse-catalog gating, `definitional_signal_names` unit). `scripts/run_ci.sh` GREEN (fmt + clippy
    `-D warnings` + lib **1468** + rustdoc + mdBook) + kg-bench 151/151. KM card `definitional-signal-capture`;
    book subsection in `pipeline/evidenceir.md`. Commit: pending (this slice).
  **Honest residual / follow-up:** SWIO (the shared C6 contact) — capturable only from the noisy
  abbreviation-table-I/O or `<NAME> signal` apposition; a dedicated `.9.8b` (abbreviation-expansion signal
  miner with a structural I/O-vs-concept discriminator to separate SWIO/`DAT*`/`CMD` from MMIO/DMA/IOVA) can
  revisit it if the owner wants the contact surfaced as a signal.
- ID: `PDF-VARIANT-DIGESTION.9.9` · Status: `done` (`2026-06-09`, CLOSED — already-enforced, no build) · Goal:
  **constraint-subject precision on protocol/layer acronyms** — stop `UICC`/`SWP`/`CLF`/`SHDLC`-style
  document/layer acronyms from becoming `signal_constraint` subjects (a precision gap that inflates
  `document_class`), agnostically, without regressing the wire-based 100%.
  **Investigation (`2026-06-09`, probe-first): the violation does NOT manifest in current canonical IR.** A
  corpus-wide scan of all **76** persisted evidence docs found **0** `signal_constraints` whose `signal_name`
  is not a declared signal of that document — i.e. the precision invariant the `.9.9` node targets is already
  STRUCTURALLY enforced by the existing constraint-subject-must-be-a-declared-signal gate
  (`extract_signal_constraints` → `collect_known_signal_names`; WIRE-BASED-100.5i). On SWP specifically:
  `signal_constraints` is now **0** (the acronyms `UICC`/`SWP`/`CLF`/`SHDLC` aren't declared signals, and
  `.9.8` declared only the real `S1`/`S2`), so document_class is NOT acronym-inflated (SWP classes `protocol`
  off 4 FSM states + 5 relations, not fake constraints). The `.9.4` baseline's "11 noisy signal_constraints"
  was a pre-`.9.8` / different-metric observation that no longer reproduces on a fresh current-code rebuild.
  **Decision: do NOT build** — writing an acronym filter would be dead code guarding an invariant the declared-
  signal gate already guarantees corpus-wide. Honest no-build close (precedent: `SYMBOL-CLOSURE-CORPUS-VALIDATION`,
  `CONSTRAINT-CONDITION-SUBJECT` stale-artifact finding). Verification: corpus scan 76 docs / 0 violations.
  Commit: pending (docs-only, this slice).
- ID: `PDF-VARIANT-DIGESTION.1` · Status: `in_progress` · Goal: **triage sweep** — ingest a diverse sample
  (one per family: ARM-TRM GIC-400, Wishbone, NXP I2C, RISC-V Debug, CCIX, Avalon, USB4, OpenCAPI) through
  `ingest`→`evidence`, record ingest status + table-kind census + extraction stats, and identify the
  per-variant FAILURE MODES (ingest crash / password protection / empty or garbage extraction / non-table
  structures / missing catalogs). Output `/tmp/digest_triage.txt` → distilled into this tree + KM. Then
  decompose into per-class feature leaves prioritized by impact (how many of the 82 each unlocks).

## Triage matrix (`.1`, `2026-06-07`, 8 diverse families — all INGEST cleanly; docling robust)

| spec | tables unknown/total | signals | relations | constraints |
|---|---|---|---|---|
| Avalon (Intel interface) | 8/34 | 34 | 179 | 0 |
| GIC-400 (ARM TRM) | 14/25 | 19 | 11 | 0 |
| NXP I2C | 14/23 | 0 | 0 | 19 |
| RISC-V Debug | 116/134 | 10 | 20 | 0 |
| CCIX | 220/261 | 0 | 0 | 35 |
| USB4 | 23/23 | 0 | 0 | 6 |
| Wishbone / OpenCAPI | (ingest ok; stats pending) | | | |

**Two dominant, genericity-preserving levers (structure/grammar, not names):**
- **Lever A — table-kind classification.** "unknown" dominates (CCIX 220/261, RISC-V 116/134, USB4 23/23);
  Avalon, which classifies well, yields 34 signals + 179 relations. Generalizing the table classifier lifts
  every doc at once. #1 impact.
- **Lever B — prose ENTITY capture: signals AND actors/agents** (owner `2026-06-07`). I2C/CCIX/USB4 show 0
  signals AND 0 relations because their entities live in prose/figures, not classified tables. Extend the
  prose-pin-appositive (`SWD-SERIAL-EXTRACTION.2`) to capture signals from more prose forms AND to ground
  the ACTOR/AGENT model from prose ("the host debugger issues requests", "the Manager initiates", "the SMMU
  translates", "the requesting agent") — not only as the inferred subject of a relation.

## `.1` refinement (why tables are "unknown") — investigated `2026-06-07`

The "unknown" tables are a MIX, so the raw count overstates the gap:
- **Non-data noise** — TOC entries ("Preface … 1", "1.1. SCOPE 8"), chapter/section indexes, revision
  history ("Version | Comments | Issue Date"), dotted page-leaders. These SHOULD stay unextracted (CCIX's
  220 is largely its huge TOC). A general structural filter (page-number column, dotted leaders, "Table of
  Contents"/"Chapter"/"Version…Issue Date" headers) cleans the signal + avoids noise extraction.
- **Real data tables, unclassified** — e.g. RISC-V `Field | Description | Access | Reset` (register-field
  tables), access legends, argument tables. These ARE intent and are dropped today. OpenCAPI shows the
  upside (it classified 16 register_map → register intent captured).

OpenCAPI: 49 tables (27 unknown, 16 register_map, …), 0 signals/relations/constraints. (Wishbone stats hit
a stats-script regex bug — ingest OK; re-measure with the hardened helper.)

## Whole-corpus re-triage (`.1` re-run, `2026-06-08`) — Lever A+B uplift measured

Ran the deterministic pipeline (ingest→evidence, no VLM) over all 82: **75 OK** (+Wishbone), **5 ingest
timeouts** (giant ISA manuals — ARM A32/T32, A64, registers; Intel SDM; USB4 v2.0 — need a longer ingest
budget), **1 evidence-fail** (USB 3.2 — bug). **Aggregate (vs the original triage where most non-AMBA docs
were 0): 62 docs yield signals, 34 registers, 49 relations; totals 1,908 signals / 2,953 registers / 10,632
fields / 3,077 relations.** #3 confirmed: CCIX (was 0 → sig/reg/fld/con/rel), I2C (sig 10), OpenCAPI, USB4
CM/inter-domain all extract. ~9 OK docs still yield 0 (guides / ISA / image-table-heavy) = the VLM frontier
(`.2b`/`.2b'`). Matrix `docs/corpus_coverage_2026-06-08.md`; KM `corpus-coverage-sweep`. NEXT: `#2` VLM
grid-repair demo on a bits-bearing doc; giants' ingest budget; USB 3.2 evid-fail.

## Current frontier

**OWNER-DIRECTED PIVOT (`2026-06-08`): ACTIVE LEAF `PDF-VARIANT-DIGESTION.9.3`.** The owner unblocked the program
by downloading a new **serial chip-spec class** (CAN / SWP / SMBus / I2S) and directed they be copied + git-tracked
into SpecForge. This takes frontier priority over `.6`/`.7` (still blocked on host-local zero-yield / USB-3.2 PDFs).
**`.9.1` (import + register, commit `c265062b`) DONE. `.9.2` (CAN 2.0 honest baseline) DONE** — CAN is an honest
**UNDER-EXTRACTED spec**: deterministic yield 0 signals/registers/relations/constraints, the `.5a`/`.5c` machinery
correctly flags `document_class: guide` + `document_type_declared: specification` → ⚠ under-extracted (route to
VLM/prose frontier), and the honesty guardrail confirmed CAN's frame fields + error-state FSM ARE richly present in
the prose (frame: SOF/Arbitration/Control/Data/CRC/ACK/EOF; FSM: error-active/passive/bus-off + TEC/REC). Root: the
SWD-tuned prose extractors (`extract_serial_frame_fields`/`extract_protocol_states`) don't generalize to CAN's
prose. `.9.3` split into `.9.3a` (FSM) + `.9.3b` (frame). **`.9.3a` DONE** — new agnostic `extract_quoted_mode_states`
recovers CAN's error-state FSM (`error active`/`error passive`/`bus off`) from quoted node-modes; CAN 0 → 3,
zero spurious, ADI/SWD gains 0 `mode_state_*` (13 SWD states intact), NVMe/I2C/RISC-V 0; full `run_ci.sh` green +
kg-bench 151/151. **`.9.4` (SWP baseline) DONE** — SWP is `document_class: protocol` but 0 signals/FSM/frame; the
breadth survey shows each serial spec under-extracts DIFFERENTLY and surfaced a concrete lever backlog:
**`.9.7`** (single-word `<NAME> state` FSM grammar — captures SWP's `ACTIVATED`/`DEACTIVATED`, a 3rd grammar neither
the SWD hyphen-path nor `.9.3a`'s quote-path catches), **`.9.8`** (SWP prose single-wire signals `S1`/`S2`/`SWIO`),
**`.9.9`** (constraint-subject precision — drop layer/protocol acronyms `UICC`/`SWP`/`CLF`/`SHDLC` mis-read as signal
subjects). **`.9.7` (single-word `<NAME> state` FSM grammar) DONE `2026-06-09`** — the predicted highest-leverage
lever delivered: additive `extract_transition_bound_states` recovers SWP's interface FSM (`ACTIVATED`/`DEACTIVATED`/
`SUSPENDED`/`HALT`, 0 → 4) via a transition/locative binding (a state is one you enter/leave/are-in), and the
single grammar lifted **18 corpus docs** to a real FSM surface (CHI/CXS/DTI/CCIX/CoreSight/eMMC/USB4 + APB
`SETUP`/`ACCESS` + AXI low-power `RUN`/`STOP`/`ACTIVATE`/`DEACTIVATE`) with SWD (13 states, eval 100%) and CAN
(3 quoted states) byte-for-byte unchanged; `run_ci.sh` green (lib 1448) + kg-bench 151/151; KM
`transition-bound-state-fsm`. **`.9.8` (SWP prose single-wire signals) DONE `2026-06-09`** — a third additive
prose form (the definitional copula `<NAME> is a/an signal` + glossary colon `<NAME>: signal …`, candidate
required to be an all-uppercase identifier token) recovers SWP's two signals **`S1`/`S2` (0 → 2)** with ZERO
corpus garbage (descriptor-apposition and abbreviation-expansion alternatives both PROBE-REJECTED as toxic);
SWD/ADI declared set byte-identical (no regression, proven by stash diff); SWIO is the shared C6 contact, an
honest residual (`.9.8b`). `run_ci.sh` green (lib 1468) + kg-bench 151/151; KM `definitional-signal-capture`.
**`.9.9` (constraint-acronym precision) CLOSED `2026-06-09`** — investigate-only: the declared-signal gate
already enforces the invariant corpus-wide (76 docs / 0 non-declared constraint subjects), no build needed.
**`.9.3b` (CAN frame fields) DONE `2026-06-09`** — `extract_composition_frame_fields` recovers CAN's frame
STRUCTURE (0 → 7 ordered fields) from the prose composition list + honest directly-stated widths (CONTROL=6,
ACK=2, rest residual `None`, no fabrication); only CAN fires (probe), SWD/AXI untouched; lib 1472, kg-bench
151/151. **`.9.5` (SMBus 3.3.1 honest baseline) DONE `2026-06-09`** — first-ever ingest (83 pp / 100 visual /
`ready`); `document_class: guide` + `document_type_declared: specification` → ⚠ UNDER-EXTRACTED spec (machinery
correct, like CAN). Deterministic yield **1 real signal (`SMBCLK`) + 1 noise (`WIRE`); 0 FSM / 0 frame /
0 actors / 0 relations / 0 constraints; 26 conditional rules; 84 timing_constraints (strong); 1 register table
(21 fields)**. Honest finding: SMBus declares its bus signals (`SMBCLK`/`SMBDAT`/`SMBSUS#`/`SMBALERT#`) ONLY in
prose (NO signal table) in idioms none of the existing grammars catch — the `.9.8` definitional copula misses
the `#` active-low suffix + the descriptor word ("optional"/"wired-AND") — so 3/≥4 signals missed → a NEW
prose-signal lever `.9.10` (probe-locked, spun). **`.9.6` (I2S NXP UM11732 honest baseline) DONE `2026-06-09`** —
first-ever ingest (14 pp / 27 visual / `ready`); `document_class: guide` + `document_type_declared: specification`
→ ⚠ UNDER-EXTRACTED spec. Yield **2 real signals (`SCK`+`SD`) captured from prose but `WS` (word select, 11×)
MISSED; 1 protocol_actor ("controller" — the `.8` grammar fires, better than SMBus); 0 FSM / 0 frame / 0
relations / 0 constraints; 1 conditional rule = NXP legal boilerplate noise; 0 timing_constraints despite 2
`timing_parameter` tables (blank leading symbol-column shape)**. Honest finding: I2S under-extracts differently
again → spun `.9.11` (blank-leading-column timing-table recovery) + folded `WS` into `.9.10`'s probe scope.
**`.9.10` (prose bus-line signal grammar) PROBE DONE `2026-06-09`** — over all 78 persisted `evidence_ir` docs:
Form A ("the/The `<NAME>` line") is the corpus-safe winner (5/78 docs, 0 wire-based, recovers SMBus
`SMBCLK`/`SMBDAT`/`SMBSUS#` + I2S `WS` + bonus I2C/eMMC); Form C ("is a/an … signal") REJECTED (fires on
APB/AHB/AXI → breaks byte-identical). **Implementation gated on ONE owner decision: how to treat power-supply
rails `VDD`/`VSS` (Form A captures them on the MEASURED I2C doc → would regress its gold precision 0.600) —
recommended: extend the existing universal-term denylist `is_signal_synthesis_non_signal` (CLOCK/RESET
precedent; ADR-0006-safe universal supply vocabulary).** **OWNER STEER (`2026-06-09`): NO denylist — "using a
list to handle 100s of PDFs is a sign of weakness"; the clean fix needs participation-based signal identity
(driven/asserted/sampled binding), which needs the deterministic shallow-parser → `.9.10` PARKED behind
`NLP-SHALLOW-PARSE`, not shipped with a list/case crutch.** **`.9.11` (header-trapped timing-table recovery)
DONE `2026-06-09`** — structural recovery of timing data rows Docling trapped in `header_rows`; **I2S 0 → 5
timing_constraints, SMBus held at 84**, nested/complex tables stay honest residuals; lib 1475, kg-bench 151/151.
**`.9.12` (CLOSED, investigate-only) DONE `2026-06-09`** — triaged the remaining serial `unexplained_intent_bearing_tables`:
SMBus `table_0022` (degenerate bitfield), SMBus `table_0042` (address-assignment, not register fields), and I2S
`table_0005` (3-level nested cross-tab, core params already from `table_0004`) are ALL honest residuals (do not
build; residual over fabrication). **The serial-class STRUCTURAL table levers are now exhausted** (the buildable
one, `.9.11`, is done); the remaining serial gaps are PROSE-signal (parked). **Frontier (any of, owner may
steer):** `.10` (corpus register-table-shape recovery — probe DONE `2026-06-10`, build sub-leaves below) ·
`.9.10`/`.9.8b` (prose-signal — PARKED behind participation-based identity / `NLP-SHALLOW-PARSE`, no
list/case) · `CORPUS-PATTERN-REUSE.3` (corpus-cluster
CLI, on-#1). `.6`/`.7` (the older VLM-frontier / USB-3.2 leaves) stay blocked on host-local PDFs.

**`PDF-VARIANT-DIGESTION.10` — corpus register-table-shape recovery (the broader-corpus
structural probe, run `2026-06-10`).** · Status: `in_progress` (`.10p` probe DONE; build
sub-leaves pending, probe-first per family). The corpus-wide census over all 77 persisted
SourceIRs: **7,012 `unknown`-kind structured tables**, and the top normalized header
signatures are overwhelmingly REGISTER-FIELD variants (~2,000 tables) — all structural
header vocabulary, zero chip names (ADR 0006). Crossing each doc's register-shaped
unknown-table count against its extracted register yield separates healthy docs (extraction
works without the kind label: CoreSight-0701 598→833 regs/2,978 fields; NVMe 199→42/201;
VT-d 152→103/318; RISC-V Debug 59→44/179) from the REAL gaps:
  | doc | reg-shaped unknown tables | extracted regs/fields | dominant unexplained signature |
  | --- | --- | --- | --- |
  | AMD IOMMU 48882 | 167 | 11/**0** | `bits \| description` (+287-count family ×3 docs) |
  | CCIX ×4 versions | ~151 each | 8/11 each | `bit location \| register description \| attributes` (481 corpus-wide) + `byte location \| size \| register description \| attributes \| m/o` |
  | CoreSight TMC ddi0461 | 56 | 2/50 | `bits \| name \| description` family |
  | GIC-600 TRM | 72 | 15/214 | `bits \| name \| function` (129 corpus-wide, 5 docs) |
  | MMU-700 TRM | 68 | 13/91 | `bits \| name \| description` (68) |
  | CHI C2C (ihi0098) | 84 | 76/315 | `bits \| field \| description \| access type \| reset` (65) — partial |
  | CHI G (ihi0050) | 28 | **0/0** | degraded ingest (host-local re-provide blocks full fix) |
  Build sub-leaves (each probe-first on ITS family, structural only, no lists/case,
  wire-docs + register golds stay green): `.10a` the `bit location`-keyed register/field
  vocabulary (CCIX family, ~600 tables across 4 docs — biggest single unlock); `.10b` the
  two-column `bits \| description` shape (AMD IOMMU + relatives, ~290); `.10c` the
  `name \| function` column synonym (GIC-600 +4 docs, ~129). Honest boundary: a shape is
  recovered only when its rows actually parse as bit-range + field semantics — otherwise it
  stays an explicit residual (no fabrication).

**`PDF-VARIANT-DIGESTION.10a` — `bit location` register-field vocabulary (CCIX family).**
· Status: **DONE `2026-06-10`** (probe → measured design → build → live per-item verification).
**Verification log:** lib tests 1546→1553 (7 new hermetic: gate vocabulary, identifier-shape,
paren+frame, all four leading-identifier bleed gates, caption-offset grammar, full
synthesize integration incl. the byte-location negative twin); kg-bench **154/154**; full
`scripts/run_ci.sh` GREEN. **Old-vs-new dry-run parity sweep over ALL 12 intact-bundle
docs (NVMe, RISC-V Debug, APB/AHB/AXI/AXI-Stream, SWD/ADI, I2C, SMBus, I2S, CAN, SWP):
byte-IDENTICAL** — zero drift on every gold-measured doc, promoted canonical surfaces
untouched. **Live run on the real persisted CCIX/CoreSight tables** (stub-markdown /tmp
copies; canonical untouched — their normalized bundles are host-local-blocked like
`.6`/`.7`): CCIX rev2.0 **8 regs/11 fields → 143 regs/389 fields** (259 named + 130 honest
bit-range residuals, 40 registers with recovered byte offsets; e.g. `CCIX PL DVSEC Header
at Byte Offset 04h` = CCID 15:0 RO / DVSECRevID 19:16 RO / DVSECLength 31:20 RO);
CoreSight 0100/0200 upgrade EXACTLY the 13 predicted fields each (`ATDATA127`, `AFVALID`,
`ID0_20_2F`…, bit positions preserved) and the predicted single residual mis-name exists
exactly once corpus-wide (`register_table_0149`, page-wrap bleed). Canonical CCIX/CoreSight
EvidenceIRs refresh whenever those PDFs are re-provided and re-ingested. Book:
`pipeline/evidenceir.md` subsection; KM card `bit-location-register-field-vocabulary`.
**Family probe over the persisted corpus:** 542 `bit location`-headed unknown tables exist
ONLY in the 4 CCIX SourceIRs (0 outside — gate extension corpus-safe); 1,432 data rows;
98.3% of bit cells parse as bit ranges. The register identity lives in the caption
(`<Name> Register fields at Byte Offset 04h` / `<Name> Register at Byte Offset-0Ch`); the
FIELD NAME is fused into the description cell as its leading identifier (`CCID This field
indicates …`), beside `Reserved …` rows (395) and honest residual rows (`See Table 7-1 …`,
wrapped continuation bleed). Why today yields 8 regs/11 fields per doc: the
`is_register_field_header` gate misses (`bit location` ≠ `bit`/`bits`, no `field`/`name`
cell), and in the `field description` variants the description column itself is taken as
the name column, so sentence-length "names" die at the >4-words gate.
**Measured design (every gate demonstrated per-item on the corpus):**
  1. gate + bits-column vocabulary unified: a bit-POSITION header (`bits`/`bit`/`bit
     range`/`position`/`bit location`) counts as the field/bits column evidence
     (probe P2: the full unified vocabulary newly admits ONLY the 542 CCIX tables);
  2. a name-ish header that also contains `description` is a DESCRIPTION column, never the
     field-name column (probe P1: only the 52 CCIX `field description` tables change);
  3. mnemonic recovery on the existing bit-range-name path gains two grammar forms after
     the untouched `(MNEMONIC):` form: **paren+frame** `Full Name (Ident) This
     field/bit/value/register …` (9/doc, recovers mixed-case `SevNocomm`/`LogLen` class) and
     **leading-identifier** `<Ident> This field …` gated by: identifier-shaped token (not a
     plain Titlecase/lowercase English word — `See`/`Indicates`/`Error` rejected), ≠ the
     row's own access cell (`RO Reserved bit …` bleed), remainder not `Reserved…`-led,
     unique among the table's leading tokens (kills ADI `ASCII Identity code` ×3), and no
     LATER mid-cell defined-term `<Ident> This field` (kills `… (except FLR).
     LinkCreditSendEnable This field …` continuation bleed);
  4. caption locator `… at Byte Offset <tok>` → `offset_address` (verbatim token; `from …
     through …` ranges stay None — never collapsed to a guessed point).
**Measured outcome:** CCIX ×4: 9 form1 + 208–249 form2 mnemonics/doc (vs 11 fields TOTAL
today); CoreSight 0100/0200 TRMs: +13 genuine each (`ATDATA127`, `ID0_20_2F`, `ATREADYS` —
all eyeballed); NVMe (63 residual rows), RISC-V, ADI, SWP, I2C: UNCHANGED by construction
(probe-proven 0 new accepts). Known residuals (honest, quantified): ONE wrong-name row
corpus-wide (rev2.0 `table_0149` — `CCIX` from HAQREQ continuation bleed with no
defined-term marker; locally indistinguishable from a genuine row, documented not gamed);
the `attibutes` typo family (4 tables) stays residual (typo vocabulary is a list — not
added); `byte location \| size \| register description` tables (~60) are register-AT-OFFSET
structures, NOT bit fields — treating byte offsets as bit ranges would fabricate, so they
stay residual (future lever: map them to register-map records). Side effect noted:
`audit-extraction` uses the same gate for its sampling labels — classification labels
shift on CCIX-class tables only.

**(SUPERSEDED active note) `PDF-VARIANT-DIGESTION.6`/`.7`** — item ② (`.5`) COMPLETE and `.8` (broaden
prose-actor capture) DONE. **`.5a` + `.5b` + `.5c` are DONE** — structural doc-class routing
(protocol/register/interface/guide), the front-matter doc-type signal (true guide vs under-extracted spec), and
the class-aware per-doc completeness gauge (`.5b`) are all live in `validate` with honest guide reporting. **`.8`
DONE** — robust structural agent-definition grammar broadens prose-actor capture (projected 14 → 20 docs,
garbage-free, no denylist). Remaining: `.6` (VLM levers on zero-yield) / `.7` (USB 3.2 evidence-fail). Live distribution over the 74 persisted docs: protocol 25 /
register 11 / interface 22 / guide 16; `.5c` further split the 16 guides into 11 TRUE guides + 5 UNDER-EXTRACTED
specs (front-matter self-declares a spec → flagged for the VLM frontier `.6`, not silently dismissed). **`.4` (correctness/precision verification, item ①) is COMPLETE** — both
`.4a` (per-fact gold on register fields + prose signals) and `.4b` (VLM proposer/verifier audit) done.
**`.4b` DONE** (`.4b.1` harness + `.4b.2` live measurement): the `audit-extraction` VLM audit gives a
table-kind precision ESTIMATE that **discriminates extraction quality and independently corroborates `.4a`** —
RISC-V Debug **0.250/0.375** (register tables flagged for lacking in-table bit positions, matching `.4a.2`'s
bit-extent 0/179) vs NVMe **0.750** (register tables confirmed, matching `.4a.3`'s 0.931 bit-structure recall;
caught a real feature-matrix→timing misclassification). **`.4a` is DONE** (`.4a.1`–`.4a.5`): the
register-field surface is
measured on two opposite-shaped docs (RISC-V field-name recall 0.588; NVMe bit-structure recall 0.931) and the
declared-signal surface on I2C (recall 1.000 / precision 0.600). Five extraction-fix targets are now quantified
(RISC-V register-name + bit-graphic; NVMe mnemonic; I2C acronym/condition filter; …). The `.2`–`.3b` leaves
below are DONE (Lever A + B); `.4`–`.8` are the "make the breadth trustworthy" program.

- `PDF-VARIANT-DIGESTION.2` (Lever A, deterministic strategy) — **DONE**: `synthesize_register_field_tables`
  recovers register-FIELD tables the classifier left `unknown` (header-in-body `Field|Description|Access|
  Reset`, `Bits|Type|Reset|Description`, …) → `RegisterRecord`s. Designed from a corpus survey of real
  register-table shapes/access-notations/bit-formats ([[project_flexible_register_model]]); access/reset are
  free strings, bit ranges parse zero-padded, header-echo legend rows dropped. **RISC-V Debug: 60 regs / 179
  fields** from previously-`unknown` tables; APB/AHB/AXI/SWD source-tolerant stay 1.000 (additive); +4
  hermetic tests; full `run_ci.sh` green. RISC-V Debug PDF copied into `corpus/`. KM
  `register-field-table-extraction`.
- `PDF-VARIANT-DIGESTION.2b` (Lever A, VLM strategy) — **DONE (classification)**: Qwen2.5VL reads the
  `table_region` images and reclassifies `unknown` tables (validated live — it read the RISC-V `dmcontrol`
  table's kind + all 5 field names from the image). `enrich --vlm-provider ollama` now runs
  `classify_unknown_tables_via_vlm` (shared `vlm_image_query`; `parse_vlm_table_kind`), best-wins (only
  `unknown` tables touched; `register_field`/TOC/other not reapplied — grammar path / noise), writes
  `table_kind` back so a re-run of `evidence` fires the deterministic extractor. Gated (`skip` = no-op);
  encryption irrelevant (docling renders the images). **VERIFICATION GATE** (`vlm_kind_structurally_consistent`):
  the VLM proposes, structure disposes — a kind is applied only when the table header matches it, so the VLM's
  over-classifications (register-field / operation tables → `signal_description`) are rejected. Measured on
  RISC-V: without gate 22 reclassified (+9 real DMI signals but +5 garbage); WITH gate **1 reclassified → 9
  genuine DMI signals (`REQ_*/RSP_*`), 0 garbage**. +2 hermetic tests; full CI green. KM `vlm-table-strategy`.
- `PDF-VARIANT-DIGESTION.2b'` (Lever A, VLM extraction) — **DONE**: VLM GRID REPAIR. ~10% of corpus tables
  (262) are degenerate (Docling failed to structure them: ≤1 column). `enrich --vlm-provider` now runs
  `repair_degenerate_tables_via_vlm` — the VLM transcribes the table image to a JSON grid
  (`build_table_extract_prompt`/`parse_vlm_grid`, serde_json, ≥2 columns), REPLACING the degenerate
  header/body so the deterministic extractors run (best-wins at the STRUCTURE level: Docling grid vs VLM
  grid). Kind set only when the repaired header is structurally consistent (`.2b` gate). +1 hermetic test;
  full CI green; gated (skip = no-op).
- `PDF-VARIANT-DIGESTION.2c` (model flexibility) — **DONE (core)**: the register model now carries, all
  backward-compatible: `RegisterRecord.size_bits`; `RegisterFieldRecord.bit_width` (a field is
  `name + offset(=bits_low, LSb) + width`; range/single-bit/offset+width all map) + `enumerated_values`
  (`RegisterFieldEnumRecord { value, meaning }`); access/reset stay FREE strings. Populated deterministically
  where data exists: width from `[high:low]`, register size from max field MSb, inline enums from
  binary/hex/Verilog literals in descriptions (conservative — no bare-int false positives). A no-garbage
  filter drops bit-LAYOUT grids (register-diagram tables whose "fields" are bare bit numbers). +3 hermetic
  tests. **Real-data demo: NVMe → 44 registers / 199 fields, all with `bit_width`** (3 bit-layout grids
  filtered); RISC-V unaffected (60/179); APB/AHB/AXI/SWD stay 100%. NVMe added to `corpus/`. Owner-confirmed
  model
  ([[project_flexible_register_model]]). **Width is MANDATORY (physical):** a register is bit-storage so a
  width always exists; an unresolved `size_bits` is a COMPLETENESS GAP (parametric XLEN / cross-document),
  not optional — `validate` reports `registers_unresolved_width` (RISC-V 60/60 vs NVMe 0/44). REMAINING:
  register NAME from preceding heading (synthetic today — tables aren't in Docling's content_elements
  reading-order, so reliable association is deferred, not faked); block/base grouping; array/instance;
  parametric `size_expr` + cross-document width resolution.
- `PDF-VARIANT-DIGESTION.2d` (deterministic) — **DONE**: `table_is_noise` flags non-data NOISE tables (table
  of contents, list of tables/figures, revision history, section index) by general structure — dotted
  page-leaders, contents/revision caption-or-header, or rows mostly prefixed by a section number (no chip
  names). The VLM passes skip them (no wasted calls; CCIX has ~220 TOC tables) and `enrich` reports
  `tables_skipped_as_noise`. +1 hermetic test; full CI green.
- `PDF-VARIANT-DIGESTION.3a` (Lever B, prose SIGNALS) — **DONE**: `synthesize_signal_declarations_from_prose`
  gained the parenthetical-abbreviation form ("a serial data line (SDA)") on top of the pin appositive (`.2`).
  Guards (from live I2C runs): sparse-catalog FALLBACK gate (parenthetical runs only when <8 table signals —
  AXI etc. untouched, fixed a 1.000→0.857 AXI regression), uppercase-acronym gate (rejects "(resulting…)"),
  universal denylist (READ/WRITE/MODE). **I2C: 0 → 10 declared signals** (SDA/SCL + Hs SCLH/SDAH + USCL/USDA
  + ACK/NACK/DDC/SDR). Wire-based specs stay 1.000; +3 hermetic tests; full CI green. I2C PDF added to
  `corpus/`. KM `prose-signal-capture`.
- `PDF-VARIANT-DIGESTION.3b` (Lever B, prose ACTORS/AGENTS) — **DONE**: new `ProtocolActorRecord` surface +
  `extract_protocol_actors` capture agents a spec DEFINES in prose — "A <name> is the device which/that
  <capability>" and "considered a/the <name>" — grounding the agent model from prose, not only as a relation
  subject. `is_agent_noun` rejects function/structural words (general; admits vendor agents like SMMU). **I2C:
  2 actors — controller (def: "the device that initiates a data transfer … and generates the clock") +
  target.** Additive new surface (no eval impact; wire-based unaffected); +2 hermetic tests; full CI green.
  KM `prose-signal-capture`.

## Planned next — from the corpus-sweep learnings (`2026-06-08`)

The sweep proved BREADTH (66/74 in-scope docs yield extraction; 1,908 signals / 2,953 registers / 10,632
fields / 3,077 relations) but only **4/74 (5%) have verified precision** (APB/AHB/AXI/SWD golds). The
through-line for these leaves: make the breadth TRUSTWORTHY (objectively measured, no faking) before widening
it further. Priority order ① → ⑤.

- ID: `PDF-VARIANT-DIGESTION.4` · Status: `done` (`2026-06-08`; `.4a` + `.4b` done) · **① Correctness/precision
  verification of the broadened extraction** (the 95% that is coverage-only). Two complementary methods landed:
  per-fact GOLD on in-corpus docs (`.4a`) + a VLM proposer/verifier AUDIT that needs no gold (`.4b`); the two
  agree on which docs have strong vs weak extraction (cross-validated). Children:
  - ID: `PDF-VARIANT-DIGESTION.4a` · Status: `done` (`2026-06-08`; all children `.4a.1`–`.4a.5` done) · Goal: sample-gold the new surfaces (register fields,
    prose signals) on diverse in-scope docs whose PDFs are git-tracked in `corpus/` (so the gold is
    reproducible + independently verifiable); score per-fact with WIRE-BASED-100 rigor. **SPLIT `2026-06-08`:**
    the eval scorer (`eval.rs` / `commands/eval_extraction.rs`) has NO register-field or declared-signal task
    yet — a real lower-level dependency, so the eval SURFACE is built first (additive, hermetic), THEN per-doc
    gold on FRESH re-ingested evidence (the eval scores persisted evidence — [[eval-scores-persisted-evidence]]).
    In-corpus docs that exercise the new surfaces: RISC-V Debug + NVMe (register fields, `.2`/`.2c`), I2C
    (prose signals, `.3a`). Children:
    - ID: `PDF-VARIANT-DIGESTION.4a.1` · Status: `done` (`2026-06-08`) · Goal: register-field per-fact EVAL SURFACE —
      `EvalTask::RegisterField` + `GoldFact::RegisterField {register, field, bits_high?, bits_low?, bit_width?}`
      + a canonical key (normalize to `register|field|offset|width`) + `index_register_field_predictions` + an
      `extract_on_copy` branch reading EvidenceIR `register_records` (deterministic, like the SWD surfaces).
      Acceptance: additive (no extraction-behavior change); gold↔record key-match + closed-world scoring
      hermetic tests; full `run_ci.sh` green; APB/AHB/AXI/SWD eval unaffected.
    - ID: `PDF-VARIANT-DIGESTION.4a.2` · Status: `done` (`2026-06-08`) · Goal: RISC-V Debug register-field gold — author an
      independently source-verified gold for a bounded set of RISC-V Debug registers (e.g. `dmcontrol`,
      `dmstatus`, `abstractcs`) from the `corpus/` PDF, FRESH re-ingest (`DOCLING_DEVICE=cpu`), measure per-fact
      P/R/F1. Acceptance: each gold field checked against the source table (no faking, [[feedback_scoring_rigor]]).
      **DONE (measure & surface, owner directive):** authored `seed_riscv_debug_registers.json` — `dmstatus`
      (20 fields) + `dmcontrol` (14 fields) = 34, bit positions transcribed from the spec's bit-layout
      graphics (§3.14.1/3.14.2, RISC-V Debug 1.0). FRESH re-ingest + evidence rebuild. Added a register-agnostic
      `register_field_name_recall` + `register_field_completeness` (additive measurement, no extraction change).
      **Measured: field-name recall 20/34 = 0.588** (`dmcontrol` 14/14, `dmstatus` 6/20 — docling dropped the
      middle page-fragment of the page-split `dmstatus` table); register-name association 0/60 (synthetic names);
      bit-extent completeness 0/179 (bits live in the graphic, not the field table) → strict per-fact 0.000,
      honestly surfaced not hidden. 14 misses independently verified as REAL (not gold-spelling drift). +3
      hermetic tests; book section in `quality/extraction-eval.md`. Gaps → candidate fix leaves (register-name
      heading association; bit-layout-graphic parsing).
    - ID: `PDF-VARIANT-DIGESTION.4a.3` · Status: `done` (`2026-06-08`) · Goal: NVMe register-field gold — same rigor on a
      diverse vendor/layout (NVMe controller registers) from the `corpus/` PDF; measure per-fact P/R/F1.
      **DONE:** authored `seed_nvme_registers.json` — CAP (15) + CC (8) + CSTS (6) = 29 fields, bit ranges +
      mnemonics transcribed from the spec's `Bits|Type|Reset|Description` tables (§3.1.3.1/3.1.3.5/3.1.3.6).
      Discovered NVMe is the **INVERSE failure** of RISC-V Debug → added `register_bit_structure_recall`
      (register-scoped via `register_name_has_token`, mnemonic-agnostic, pools page-split fragments;
      additive measurement, no extraction change). **Measured: bit-structure recall 27/29 = 0.931**
      (2 misses `CAP.CRMS`/`CC.EN` at page-fragment boundaries, both REAL); field-name recall 0/29
      (mnemonics live in the DESCRIPTION, `field_name` is the bit-range); register-name 44/44; bit-extent
      199/199 → strict per-fact 0.000 on BOTH docs for OPPOSITE reasons (why two recall views are essential).
      Measured against verified-current persisted evidence (postdates last extraction commit; PDF git-tracked).
      +3 hermetic tests; book section + KM updated. Third fix leaf identified: extract mnemonic from description.
    - ID: `PDF-VARIANT-DIGESTION.4a.4` · Status: `done` (`2026-06-08`) · Goal: declared-signal (prose-capture) per-fact EVAL
      SURFACE — `EvalTask::DeclaredSignal` + `GoldFact::DeclaredSignal {signal, direction?}` + key + indexer +
      an `extract_on_copy` branch reading the EvidenceIR signal inventory. Acceptance: additive; hermetic
      tests; CI green. **DONE:** added `EvalTask::DeclaredSignal` + `GoldFact::DeclaredSignal {signal,
      direction?}` + `declared_signal_key` (name + optional direction; name-only gold matches a no-direction
      record) + `declared_signal_record_key` + `index_declared_signal_predictions`. The canonical inventory
      lives on SemanticIR (`interfaces[].signal_records`, `InterfaceSignalRecord` = name + `direction_hint`
      Input/Output/Internal), so the runner branch builds SemanticIR on a temp copy (like the TemporalRule
      task) and pools all interfaces' signal_records. Additive — no extraction change. +2 hermetic tests
      (gold↔record key match incl. name-only/direction discrimination; closed-world scoring). full
      `run_ci.sh` green (1371); kg-bench 151/151. Book note + the live I2C measurement land with `.4a.5`
      (the surface is latent until a gold ships, mirroring `.4a.1`).
    - ID: `PDF-VARIANT-DIGESTION.4a.5` · Status: `done` (`2026-06-08`) · Goal: I2C prose-signal gold — independently
      source-verified gold for the I2C prose signals (SDA/SCL/…) from the `corpus/` PDF; measure per-fact P/R/F1.
      **DONE (closes `.4a`):** authored `seed_i2c_signals.json` — the COMPLETE set of I2C-bus signals
      (SDA/SCL §3.1.1, Hs SDAH/SCLH §3.6, UFm USDA/USCL §3.2.1), each verified against UM10204's "signals"
      sections. Added `declared_signal_complete_gold_precision` (document-level precision over the produced
      signal set + named false positives; valid only for a complete-enumeration gold — additive, no extraction
      change). **Measured (`--provider skip`): recall 1.000** (all 6 genuine signals, source-tolerant) but
      **precision 6/10 = 0.600** — 4 over-captures named: ACK/NACK (conditions on SDA, §3.1.6), DDC (different
      bus, §4.6), SDR (I3C rate acronym). +2 hermetic tests; book section (declared signals) + KM card. Fix
      leaf: tighten the prose acronym/condition filter (now quantified). Full `run_ci.sh` green; kg-bench green.
  - ID: `PDF-VARIANT-DIGESTION.4b` · Status: `done` (`2026-06-08`; `.4b.1` + `.4b.2` done) · Goal: automated proposer/verifier AUDIT — re-read a
    random sample of extracted registers/signals against their table IMAGE with the VLM (the `.2b`
    consistency gate run as an audit) → a corpus-scale precision ESTIMATE + a flagged-mismatch list. Accept:
    a measured precision estimate over a stated sample size; garbage surfaced, not hidden. **SPLIT
    `2026-06-08`** into the audit HARNESS (`.4b.1`) and the live measurement (`.4b.2`) — the harness is an
    independently reviewable, hermetic, additive capability; the live estimate depends on it + a running VLM
    (mirrors `.4a`'s eval-surface → per-doc-gold split). Children:
    - ID: `PDF-VARIANT-DIGESTION.4b.1` · Status: `done` (`2026-06-08`) · Goal: the audit HARNESS — a new
      `audit-extraction <source-ir>` command that selects intent-bearing tables by STRUCTURE (the same
      predicates the extractors use: `RegisterMap`/`SignalDescription`/`Encoding`/`TimingParameter` kinds +
      `Unknown` tables matching `is_register_field_header`), takes a bounded reproducible sample
      (`--sample`/`--seed`; FNV-1a order, no RNG dep), and (live) asks the VLM per table "an extractor read
      this as a <kind> table — correct?" → a `table_kind_precision_estimate` (consistent/judged, errors
      excluded, `None` when nothing judged) + a named flagged-mismatch list. Default `--provider skip` =
      plan-only (lists the sample, no VLM calls; CI-safe). Agnostic by construction (ADR 0006): structural
      selection/judgment, no chip names in the prompt (hermetic test asserts none leak), structural sampling.
      **DONE:** `commands/audit_extraction.rs` (+ `vlm_image_query`/`is_register_field_header` made
      `pub(crate)`); 5 hermetic tests (classification, deterministic+seed-sensitive sampling, tolerant verdict
      parse, agnostic+STRICT-JSON prompt, precision/flag math). Additive — no extraction-behavior change;
      APB/AHB/AXI/SWD eval + kg-bench 151/151 unaffected; full `run_ci.sh` green (lib 1373 → 1378). Plan-only
      verified on RISC-V Debug (78 intent-bearing tables) + I2C (7 timing tables); seed reshuffle confirmed on
      real data; ONE live VLM call proved the execute path end-to-end (`table_0080` → consistent, estimate
      1.000, 0 flagged). Book section in `quality/extraction-eval.md`; KM card `extraction-audit-vlm`.
    - ID: `PDF-VARIANT-DIGESTION.4b.2` · Status: `done` (`2026-06-08`) · Goal: the live measurement — run
      `audit-extraction --provider ollama` over a bounded sample on the in-corpus docs that exercise the
      broadened extraction (RISC-V Debug / NVMe register fields; a register/signal doc for breadth), record
      the table-kind precision ESTIMATE per doc + the flagged-mismatch list into this tree + the book + KM.
      Accept: a measured estimate over a stated sample size per doc; every disagreement surfaced by name, not
      hidden; the VLM kept to a bounded sample (the `.2b` scaling finding). **DONE — live qwen2.5vl:7b audit,
      bounded sample 8/doc:**
      - **RISC-V Debug** (78 intent-bearing tables): seed 0 → **estimate 0.250** (2/8 consistent, 6 flagged,
        0 VLM errors, ~71 s); seed 1 → **0.375** (3/8, 5 flagged, ~46 s). Consistently LOW; the VLM flags the
        register tables for **lacking in-table bit positions** ("not a detailed bit-field definition table",
        "the table lacks bit positions") — INDEPENDENTLY corroborating `.4a.2`'s gold finding (bit-extent
        0/179; RISC-V's bits live in the layout graphic, not the field table).
      - **NVMe 2.0a** (118 intent-bearing tables): seed 0 → **estimate 0.750** (6/8 consistent, 2 flagged,
        ~51 s). HIGH; the VLM confirms the register tables (corroborating `.4a.3`'s 0.931 bit-structure
        recall) AND caught a REAL false positive — `table_0035` classified `timing` is actually a feature
        matrix ("lists controller features and supported modes with 0/1 instead of min/typ/max units").
      - **Cross-validation:** the audit's precision ESTIMATE tracks the per-fact gold quality (RISC-V weak ↔
        low estimate; NVMe strong ↔ high estimate) — two independent methods agreeing is what makes the audit
        a trustworthy instrument for the ungolded breadth.
      - **Robustness:** Avalon returned 8 VLM errors → `n/a (no table judged)` and **fabricated no number**
        (its `normalized/` images were reclaimed by an earlier `clean`; the audit needs on-disk table images,
        which only the git-tracked re-ingested RISC-V/NVMe retain — a signal-bearing breadth audit on a fresh
        re-ingest is a noted follow-up). Book section refreshed with the real numbers; KM `extraction-audit-vlm`
        updated. Commit subject: `PDF-VARIANT-DIGESTION.4b.2`.
- ID: `PDF-VARIANT-DIGESTION.5` · Status: `done` (`2026-06-08`; all children `.5a`/`.5b`/`.5c` done) ·
  **② Doc-class routing + per-doc completeness gauge.** Children: `.5a` (done) · `.5c` (done) · `.5b` (done).
  Item ② COMPLETE — `validate <evidence>` now reports the document class, the front-matter self-declared type
  (true guide vs under-extracted spec), AND a class-aware per-doc completeness gauge. Frontier moves to `.6`
  (VLM levers on the addressable zero-yield) / `.7` (USB 3.2 evidence-fail) / `.8` (broaden prose-actor capture).
  - ID: `PDF-VARIANT-DIGESTION.5a` · Status: `done` (`2026-06-08`) · Goal: detect doc class (protocol / register /
    interface / guide) from structure; apply class-appropriate surfaces; report GUIDES as "low structured
    design-intent" honestly (not a 0 failure). Accept: each doc tagged with a class; the 8 zero-yield docs
    correctly identified as guides / image-heavy, not silent misses. **DONE — pure
    `crate::ir::completeness::classify_document(DocumentClassCensus) -> DocumentClassification`
    (`{Protocol,Register,Interface,Guide}` + rationale), surfaced in `validate` as a `document_class` metric +
    an `evidence_document_class` Info finding + a console "Document Class" block. Decision (low-noise surfaces
    only): Register (registers dominate connectivity AND behavioral) → Protocol (`signal_constraints ≥ 3` or
    FSM/serial-frame) → Interface (signal inventory + connectivity, no behavior) → Guide (honest floor).
    REAL-DATA correction baked in: `conditional_rules` is OVER-PRODUCED (12× on the GIC overview guide, 78× on
    RISC-V Debug, 250× on NVMe) → EXCLUDED from the decision (kept in census, flagged "not class-determining").
    Agnostic (ADR 0006 — generic floors 2/3/3, no chip names). +11 hermetic tests (real APB/AHB/AXI/SWD/I2C/
    NVMe/RISC-V/Avalon/guide shapes); fmt + clippy `-D warnings` clean; lib 1400 → 1410; kg-bench 151/151.
    Live-verified over all 74 persisted evidence docs: protocol 25 / register 11 / interface 22 / guide 16 (the
    software/overview/optimization guides correctly → guide). No wire-based regression. KM card
    `document-class-from-structure`.**
  - ID: `PDF-VARIANT-DIGESTION.5c` · Status: `done` (`2026-06-08`) · Goal (owner-suggested `2026-06-08`): read the document's
    OWN front-matter — title, table of contents, preface/about/scope of the first chapter ("usually clearly
    stated in the early pages of the first chapter") — for a self-declared doc-TYPE signal (generic grammar:
    guide/overview/manual/specification/architecture/protocol/datasheet words, NO chip/vendor names, ADR 0006).
    Use it to (a) corroborate the `.5a` structural class, and (b) crucially separate a TRUE guide from a
    SPECIFICATION we under-extracted (a doc whose structure is empty but whose title says "specification /
    architecture / protocol / manual" is an image/table-heavy extraction GAP → the VLM frontier `.6`, NOT a
    real guide). Accept: a title/front-matter doc-type hint surfaced in `validate`; structurally-empty docs whose
    front-matter self-declares a spec are flagged as under-extracted (not silently called "guide"). **DONE —
    pure `front_matter_doc_type_hint(&str) -> DeclaredDocType {Guide,Specification,Unknown}` over generic
    doc-type vocabulary (guide/tutorial/"learn the architecture"/application-note vs specification/architecture/
    protocol/standard/datasheet/reference-manual; guide phrasings win over spec words so Arm's "Learn the
    architecture …" series reads as a guide; "overview"/"introduction" EXCLUDED because every spec has those
    chapters; whole-word match so "guidelines" ≠ "guide"). `classify_document` now takes the front-matter signal
    and sets `under_extracted_spec = (class==Guide && declared==Specification)`. `validate` reads the title +
    first 12 section headings off the sibling SourceIR (the `document_profile.title` is empty in practice → the
    early headings carry the signal), prints `document_type_declared` + an under-extracted line, adds a
    `document_type_declared` metric + a WARNING `evidence_document_underextracted_spec` finding. REALITY: the
    document title is empty; the early first-chapter headings hold the type (owner was right). +6 hermetic tests
    (real corpus framings). Agnostic (ADR 0006). fmt + clippy `-D warnings` clean; lib 1410 → 1416; kg-bench
    151/151. **Live: of the 16 `guide`-classed docs, 11 are TRUE guides (declared guide/unknown) and 5 are
    UNDER-EXTRACTED specs flagged for the VLM frontier** — RISC-V Advanced Interrupt *Architecture*, JESD235
    *JEDEC STANDARD* HBM, CoreSight Base System *Architecture* (+2). No wire-based regression. KM card
    `document-class-from-structure` updated.**
  - ID: `PDF-VARIANT-DIGESTION.5b` · Status: `done` (`2026-06-08`) · Goal: per-doc COMPLETENESS
    gauge (every register has fields? every signal a direction? unaccounted intent-bearing tables?) — extend the
    mandatory-width flag into a coverage/quality report surfaced by `validate`. Accept: honest per-doc gap
    counts; no fabrication; class-aware. **DONE** — pure `document_completeness_gauge` + `DocumentCompletenessGauge`
    / `CompletenessGap` in `ir/completeness.rs`; `validate <evidence>` prints a Document Completeness Gauge block +
    `document_completeness_gaps` / `document_completeness_applicable` metrics + an Info `evidence_document_completeness`
    finding. +5 hermetic tests (guide→not-applicable; register-doc held to fields+width; protocol-with-no-registers
    shows no register gap; fully-attributed doc is complete; sample bounded). fmt + clippy `-D warnings` clean; lib
    1416 → 1421; kg-bench 151/151; APB/AHB/AXI/SWD eval unaffected (additive). Live over persisted evidence: APB
    (protocol) 1 gap (signals_without_direction 1/32, tables 0/8); RISC-V Debug (register) registers_unresolved_width
    60/60 (XLEN-parametric, corroborates `.4a.2`) + all 60 registers have fields; Avalon (interface) signals 11/34 +
    1/13 tables; GIC overview guide → not applicable (never penalized). Book `quality/validation.md` subsection; KM
    `document-class-from-structure` updated.
    **Design (`2026-06-08`):** a pure `crate::ir::completeness::document_completeness_gauge(class, registers,
    declared_signal_count, signals_missing_direction, intent_bearing_table_count, unexplained_tables) ->
    DocumentCompletenessGauge { class, applicable, gaps: Vec<CompletenessGap{kind, missing, total, sample}> }`.
    CLASS-AWARE keyed off the `.5a` `DocumentClass`: a `Guide` is `applicable=false` (low structured
    design-intent — nothing to gauge; the `.5c` under-extracted flag carries the "actually a spec" case), so a
    guide is NEVER penalized for 0 registers/signals. For Register/Protocol/Interface, each dimension is gauged
    only when its denominator (`total`) > 0 — so a protocol with no registers shows no register gap and a
    register doc IS held to "every register has fields / a resolved width". Dimensions (all read off
    already-built IR — pure observation, no fabrication): `registers_without_fields`,
    `registers_unresolved_width` (reuses `registers_with_unresolved_width`), `signals_without_direction`
    (declared inventory − `collect_signals_with_explicit_direction_declarations`, which already folds in
    relation-derived directions), `unexplained_intent_bearing_tables` (reuses `unexplained_intent_bearing_tables`).
    Each gap carries a bounded `sample` (≤8) of affected names/ids for review. Surfaced in `validate <evidence>`
    as a console block + a `document_completeness_gaps` metric + an Info `evidence_document_completeness`
    finding (Info, not Warning — an honest known-incomplete report, not a correctness error; matches the
    severity-gating doctrine). Agnostic (ADR 0006 — no chip names; structural only). Keep APB/AHB/AXI/SWD at
    100%; additive (no extraction-behavior change).
- ID: `PDF-VARIANT-DIGESTION.6` · Status: `pending` · **③ VLM levers on the addressable zero-yield** — run
  `.2b`/`.2b'` on the image-table-heavy zero docs (OpenCAPI PHY-mech / AFU). Accept: measured uplift (tables
  reclassified/repaired → records) with 0 garbage (verification gate); honest report where the VLM also can't.
- ID: `PDF-VARIANT-DIGESTION.7` · Status: `pending` · **④ Concrete defects from the sweep** — investigate +
  fix the USB 3.2 evidence-build FAIL; adopt "measure from typed `evidence_ir/` artifacts" as the convention
  (the sweep's `rel` variable-collision bug). (Giant-ingest chunking deferred — those were ISA, now out of
  scope.)
- ID: `PDF-VARIANT-DIGESTION.8` · Status: `done` (`2026-06-08`) · **⑤ Broaden prose-actor capture**
  (14/74 on the current persisted set) — add agent-definition forms. Accept: more docs with actors,
  garbage-free, no regression on the wire-based specs.
  **DONE — robust STRUCTURAL grammar (no fragile denylist; reviewer-flagged + redesigned).** Generalized the
  `.3b` Form 1 (literal `"<NAME> is the device which/that <capability>"`) into `agent_definitions(text)`:
  `"<NAME> is a/an/the <agent-class> {that|which} <capability>"` over a CONSERVATIVE generic agent-class
  ALLOWLIST `AGENT_CLASS_NOUNS` (device/component/agent/module/entity/controller/manager/master/initiator/
  peripheral/bridge/engine/processor/host/node/subsystem — the ambiguous data/structural words "unit"/"block"
  deliberately EXCLUDED; an allowlist fails safe toward fewer captures). **First pass over-captured 4 garbage
  actors on real data; an expert reviewer flagged a growing structural-noun DENYLIST as fragile → redesigned to
  pure STRUCTURAL signals (no enumeration):** (1) `strip_trailing_parenthetical` drops a `"(refer to section
  3.1.2.1)"` cross-ref before NAME extraction (recovers the real "controller", kills "section"); (2) the NAME
  search is confined to the CURRENT SENTENCE so an anaphor `"… host system. It is the entity that …"` can't reach
  back across a period (kills "system"); (3) a NO-PREPOSITION-in-subject guard (prepositions are a CLOSED
  grammatical class, not an open noun list) rejects a prepositional-phrase object `"a use case FOR multiple HSELx
  signals is a peripheral that …"` (kills "signals"). NAME still gated by the pre-existing `is_agent_noun`
  function-word denylist (UNCHANGED — no growth) + deduped; Form 2 ("considered a/the/an <NAME>") unchanged.
  Generic grammar, no chip/vendor names (ADR 0006). +6 hermetic tests (generalized class nouns; reject
  non-agent-class; reject part-of; parenthetical cross-ref; prepositional-object; cross-sentence anaphora). fmt +
  clippy `-D warnings` clean; lib 1421 → 1427; kg-bench 151/151. **VERIFIED on real data:** canonical (real Rust
  extractor, the only 3 docs with un-reclaimed `normalized/`): NVMe 0→1 clean "controller" (section gone via the
  parenthetical strip), I2C controller/target + RISC-V "trap" unchanged. Breadth PROJECTION (faithful Python
  mirror of the grammar over persisted statements; canonical requires re-ingest of the 72 reclaimed-`normalized`
  docs): **14 → 20 docs (+6)**, every newly-gained actor a genuine agent (A76 core / ETM trace unit, CoreSight
  splitter, CCIX Transport port, AXI manager, CoreSight component) — verified per-item. APB/AHB/AXI/AXI-Stream/SWD
  project 0 actors (clean; eval unaffected — actors are an additive surface, not scored).

`.2b'` grid-repair is proven end-to-end on RISC-V (2 tables) + the `parse_vlm_grid` test. SCALING FINDING
(`2026-06-08`): `enrich --vlm` calls the VLM per unknown table → impractically slow on table-heavy docs
(NVMe 100s) → the VLM is a TARGETED/SAMPLED tool, not a full-doc pass; `.4b`/`.6` must operate on a bounded
set, not the whole doc/corpus.

## Decisions

- Triage-first: measure what breaks across variants before building, so features target real gaps (the
  owner's measure/think-before-coding ethos).
- Operate ingestion from the owner library for the survey; copy a PDF into `corpus/` + git-track it only
  when SpecForge gains a feature for it (do NOT log the library path — `feedback_source_pdfs_in_repo`).

## Open questions

- Which variant classes are highest-impact (unlock the most of the 82)? — answered by `.1`.
- How to score "digestion" for non-signal-table docs (TRMs/ISA manuals) — a coverage/quality gauge vs a
  per-fact gold?

## Blockers

- None. (Some PDFs may be password-protected — e.g. the ADI one — or very large; expect and handle.)

## Verification log

- `.9.12` (`2026-06-09`, investigate-only no-build): triaged the remaining serial `unexplained_intent_bearing_tables`.
  SMBus `table_0022` = degenerate 2-cell bitfield fragment (MSB/LSB); SMBus `table_0042` = address-assignment
  table (`Target Address [7:1] | R/W# | Description | Specification`), not register fields; I2S `table_0005` =
  3-level nested cross-tab whose core parameters are already captured by `.9.11`. All HONEST RESIDUALS (do not
  build — forcing them fabricates; the region-accounting flag is conservative). Serial-class structural table
  levers exhausted; remaining serial gaps are prose-signal (parked behind participation identity). Docs-only.
  Commit subject: `PDF-VARIANT-DIGESTION.9.12`.
- `.9.11` (`2026-06-09`): header-trapped timing-table recovery. Root cause (probe-locked): I2S `table_0004` /
  SMBus `table_0012` leave `body_rows` EMPTY because Docling marks each row-LABEL cell `is_header=true`, trapping
  the data rows in `header_rows` → the `body_rows.is_empty()` guard skipped the table → 0 records. Fix
  (`synthesize_timing_constraints`, `ir/evidence.rs`, additive + structural, no list / no case): `effective_rows
  = body_rows + header_rows[1..]` filtered to data-shaped rows (`len ≥ 2 && first cell non-empty && all value
  cells is_header=false`); a genuine multi-row column header keeps `is_header=true` value cells → excluded →
  honest residual. Live re-measure: **I2S `timing_constraints` 0 → 5** (clock period/HIGH/LOW, set-up, hold —
  empty cells stay `None`); **SMBus held at 84** (its `table_0012` shape doesn't match → residual, no
  fabrication). 3 hermetic tests (trapped-row / nested-header residual / normal-body unchanged). Full
  `scripts/run_ci.sh` GREEN (fmt + clippy `-D warnings` + lib 1472 → 1475 + rustdoc + mdBook); kg-bench 151/151
  (no eval/fixture regressed → wire-based + SWD unaffected). KM `timing-table-trapped-row-recovery`; book
  subsection in `pipeline/evidenceir.md`. Commit subject: `PDF-VARIANT-DIGESTION.9.11`.
- `.9.10` PROBE (`2026-06-09`): faithful corpus-wide probe of the prose bus-line signal grammars over ALL 78
  persisted `evidence_ir` statement-text corpora (the `.9.7`/`.9.8` method — NOT the 8 surviving normalized
  markdowns; an early markdown-only, case-sensitive probe under-counted and missed I2S "The WS line"). Result:
  **Form A "the/The `<NAME>` line" = corpus-safe winner** (5/78 docs, all 2-wire buses; SMBus `SMBCLK`/`SMBDAT`/
  `SMBSUS#`, I2S `WS`, bonus I2C `SCL`/`SCLH`/`SDA`/`USCL`/`USDA` + eMMC `CMD`/`DAT`/`DAT0`; **0 wire-based hits
  → byte-identical preserved**); Form B (collective lines) = redundant subset; **Form C "is a/an … signal" =
  REJECTED** (fires on AXI `BRESP`/`RRESP`, APB `PCLK`/`PPROT`/`PSLVERR`/`PSTRB`, AHB `HWSTRB` → breaks
  wire-based byte-identical). OPEN FORK gating the build: Form A captures power rails `VDD`/`VSS` on the MEASURED
  I2C doc (gold precision 0.600) → admitting them regresses a tracked score; recommended agnostic fix = extend
  the existing universal-term denylist `is_signal_synthesis_non_signal` (CLOCK/RESET precedent; universal supply
  vocabulary, ADR-0006-safe) — surfaced to owner before coding. Docs-only checkpoint (probe finding recorded;
  no code). Commit subject: `PDF-VARIANT-DIGESTION.9.10` (probe).
- `.9.6` (`2026-06-09`): I2S (NXP UM11732) honest baseline (first-ever ingest of this spec). `DOCLING_DEVICE=cpu
  ingest` → 14 pages / 27 visual assets / `ready` / 0 residuals / `document_key:
  um11732_v3_2022_02_17_i2s_bus_specification`. `evidence` → 24 anchors / 152 spans / 27 visual / 14 links / 154
  statements. `validate` → `document_class: guide`, `document_type_declared: specification`, ⚠ `under-extracted
  spec`. Deterministic yield: 2 real signals (`SCK` + `SD`, captured from prose) but `WS` (word select, 11×)
  MISSED; 1 protocol_actor ("controller", prose — the `.8` grammar fires); 0 protocol_states / 0
  serial_frame_fields / 0 actor_signal_relations / 0 signal_constraints / 0 register_records; 1 conditional rule
  = NXP legal boilerplate noise ("otherwise agreed in a valid written individual agreement …"); 0
  timing_constraints despite 2 `timing_parameter` tables (`table_0004` / `table_0005`) flagged unexplained; 0
  signal_semantic_hints / 0 signal_polarities; completeness gauge `not applicable` (guide). Honesty-guardrail
  recon: I2S has no signal table; `SCK` (5×) / `WS` (11×) / `SD` (2×) declared in prose ("the device generating
  SCK and WS is the controller", "the WS line", "the WS signal"); the 2 timing tables have a BLANK leading
  parameter/symbol column header (`|  | MIN | TYP | MAX | CONDITION |`) so no symbol anchors the row. Conclusion:
  under-extracted differently than CAN/SWP/SMBus → spun `.9.11` (blank-leading-column timing-parameter table
  recovery) + folded `WS` recall into `.9.10`'s probe scope. Docs-only slice (generated IR git-ignored; no code
  change → APB/AHB/AXI/SWD trivially unaffected). Commit subject: `PDF-VARIANT-DIGESTION.9.6`.
- `.9.5` (`2026-06-09`): SMBus 3.3.1 honest baseline (first-ever ingest of this spec). `DOCLING_DEVICE=cpu
  ingest` → 83 pages / 100 visual assets / `ready` / 0 residuals / `document_key:
  smbus_3_3_1_2024_10_20_system_management_bus_specification`. `evidence` → 139 anchors / 1076 spans / 100 visual
  / 163 links / 1131 statements. `validate` → `document_class: guide`, `document_type_declared: specification`,
  ⚠ `under-extracted spec` warning (the `.5a`/`.5c` routing fires correctly). Deterministic yield: 1 real signal
  (`SMBCLK`) + 1 NOISE (`WIRE`, from "two-wire"/"wired-AND") [validate counts "2 signals"]; 0 protocol_states /
  0 serial_frame_fields / 0 protocol_actors / 0 actor_signal_relations / 0 signal_constraints; 26 conditional
  rules; 84 timing_constraints (from tables — strong); 1 register_record (21 fields, unnamed); 1 signal_polarity
  (`SMBCLK` active_low, medium); 1 signal_semantic_hint (`SMBCLK` ready-like, dubious — from a STOP-condition
  definition); 3 unexplained intent-bearing tables (`table_0012` timing_parameter / `table_0022`
  signal_description / `table_0042` register_map); completeness gauge `not applicable` (guide); converged in 2
  passes (+52 facts). Honesty-guardrail recon of the normalized md confirmed present-but-unextracted facts:
  `SMBCLK` (40×) / `SMBDAT` (32×) are bi-directional bus lines declared only in prose ("the SMBCLK line", "Both
  SMBCLK and SMBDAT lines are bi-directional") with NO signal table; `SMBSUS#` (12×) / `SMBALERT#` (16×) are
  optional signals declared as "SMBSUS# is an optional signal" / "SMBALERT# is a wired-AND signal" — the `.9.8`
  definitional copula misses both the `#` active-low suffix and the descriptor word between "a/an" and "signal";
  actors "controller" (296×) / "target" (280×) heavily present but 0 protocol_actors. Conclusion: under-extracted
  differently than CAN/SWP → new prose bus-line signal lever `.9.10` (probe-locked). Docs-only slice (generated
  IR git-ignored; no code change → APB/AHB/AXI/SWD trivially unaffected). Commit subject:
  `PDF-VARIANT-DIGESTION.9.5`.
- `.9.4` (`2026-06-08`): SWP honest baseline. `DOCLING_DEVICE=cpu ingest` → 147 anchors / 1303 statements / 82
  visual. `evidence` + `validate` → `document_class: protocol`; 0 signal_declarations / 0 protocol_states / 0
  serial_frame_fields / 0 protocol_actors / 0 actor_signal_relations / 1 register_record; 11 signal_constraints.
  Recon of the normalized md confirmed present-but-unextracted facts: FSM states `ACTIVATED state` (8×),
  `DEACTIVATED state` (10×), `Reset State` (2×), `S1 state` (single-word `<NAME> state` grammar); signals `S1`
  (56×), `S2` (41×), `SWIO` (38×) in prose; and the 11 constraints subject on acronyms `UICC`/`SWP`/`CLF`/`SHDLC`/
  `RSET`/`CLT` (noise). Conclusion: under-extracted differently than CAN → levers `.9.7`/`.9.8`/`.9.9`. Docs-only
  (generated IR git-ignored). Commit subject: `PDF-VARIANT-DIGESTION.9.4`.
- `.9.3a` (`2026-06-08`): CAN error-state FSM via the new agnostic `extract_quoted_mode_states` (+
  `quoted_mode_states_in` + `is_quoted_mode_state_name`), wired additively after the SWD FSM paths and deduped by
  state name. Grammar (ADR 0006): single-quoted 1–3-word name bound to a generic actor-noun
  (node/unit/station/device) via adjective (`'<name>' <actor>`) or predicate (`<actor> <link-verb> '<name>'`)
  form, recurrence ≥2, ≥2 distinct states. Live re-build: **CAN `protocol_states` 0 → 3** (`error active` s8 /
  `error passive` s18 / `bus off` s4; honest `when …` actions), zero spurious. **No regression:** rebuilt NVMe /
  I2C / RISC-V Debug = 0 protocol_states; **re-ingested ADI/SWD = 0 `mode_state_*` + its 13 SWD/JTAG states
  intact** (Capture/Shift/Update-IR/DR, Run-Test/Idle, Test-Logic-Reset, Reset, Operating, Protocol error,
  Lockout, Dormant). Tests: 5 new hermetic (1 positive + 4 negative: bit-value/bus-condition reject, single-mode,
  non-recurrence, parallel-bus) + 35 SWD serial pass. Full `scripts/run_ci.sh` GREEN (fmt + clippy `-D warnings`
  + lib 1440 → 1445 + rustdoc + docs); kg-bench 151/151. KM card `agnostic-quoted-mode-fsm`; book subsection in
  `pipeline/evidenceir.md`. Commit subject: `PDF-VARIANT-DIGESTION.9.3a`.
- `.9.2` (`2026-06-08`): CAN 2.0 honest baseline. `DOCLING_DEVICE=cpu ingest` → 72 pages / 98 visual assets /
  `normalization_status: ready` / 0 residuals / `document_key: bosch_can_specification_2_0_1991`. `evidence` →
  269 section anchors / 751 statements / 98 visual / 6 links / 0 actor-signal relations. `validate` →
  `document_class: guide`, `document_type_declared: specification`, ⚠ `under-extracted spec` warning (the
  `.5a`/`.5c` routing fires correctly); 0 signals / 0 registers / 0 relations / 0 signal_constraints; 12
  narrative conditional rules; completeness gauge `not applicable` (guide); 19 nlp prose residuals; 98 figures
  with no VLM observations. Honesty-guardrail grep of the normalized md confirmed the facts ARE present in prose
  (frame fields + error-state FSM counts above). Conclusion: genuine under-extraction, lever `.9.3`. Docs-only
  slice (generated IR is git-ignored). Commit subject: `PDF-VARIANT-DIGESTION.9.2`.
- `.9.1` (`2026-06-08`): imported CAN/SWP/SMBus/I2S under canonical `corpus/` vendor paths, registered in
  `SOURCE_PDF_REGISTRY.md`, each verified to begin with `%PDF-`; memory-arch + knowledge-map pre-commit hooks
  green. Commit `c265062b`.
- `.1`: triage sweep launched `2026-06-07` over 8 diverse families.
- `.4a.1` (`2026-06-08`): register-field per-fact eval surface added to `eval.rs` + `commands/eval_extraction.rs`
  (`EvalTask::RegisterField`, `GoldFact::RegisterField`, normalized `register|field|offset|width` key,
  `index_register_field_predictions`, deterministic `extract_on_copy` branch over EvidenceIR `register_records`).
  Additive — no extraction-behavior change. +3 hermetic tests (bit-extent normalizer; gold↔record key match incl.
  range≡offset+width, wrong-extent divergence, register-as-identity; closed-world scoring). `cargo fmt --check`
  clean, `cargo clippy -D warnings` clean, `kg-bench` 151/151, full lib suite 1360 → 1363. Book note deferred to
  `.4a.2` (the surface is latent until a gold ships). Commit subject: `PDF-VARIANT-DIGESTION.4a.1`.

- `.4b.1` (`2026-06-08`): audit harness landed — `commands/audit_extraction.rs` (new `audit-extraction`
  command) + `vlm_image_query`/`is_register_field_header` made `pub(crate)`. Structural sampler
  (`audited_kind` mirrors the extractor predicates; `select_sample` FNV-1a deterministic, seed-sensitive),
  kind-aware agnostic STRICT-JSON audit prompt, tolerant `{consistent,reason}` verdict parser,
  `table_kind_precision_estimate` (consistent/judged, errors out of the denominator, `None` when nothing
  judged) + named flagged-mismatch list. `--provider skip` plan-only default (CI-safe). +5 hermetic tests.
  `cargo fmt --check` clean, `cargo clippy -D warnings` clean (dropped the `enum_variant_names` postfix),
  full lib suite 1373 → 1378, kg-bench 151/151, APB/AHB/AXI/SWD eval unaffected (additive). Plan-only run on
  RISC-V Debug (78 intent-bearing tables sampled 8) + I2C (7 timing tables); seed-1 reshuffle confirmed on
  real data; ONE live `--provider ollama` call proved the execute path (`table_0080` → consistent, estimate
  1.000, 0 flagged). Book `quality/extraction-eval.md` + KM `extraction-audit-vlm`. Commit subject:
  `PDF-VARIANT-DIGESTION.4b.1`.

- `.5a` (`2026-06-08`): structural document-class classifier landed — pure
  `crate::ir::completeness::classify_document` over a `DocumentClassCensus`, surfaced in `validate`
  (`document_class` metric + `evidence_document_class` Info finding + console block). Real-data correction:
  `conditional_rules` is over-produced (12× on GIC overview guide, 78× RISC-V, 250× NVMe) → excluded from the
  decision; routes only on low-noise surfaces (registers / signal constraints / relations / declared signals /
  FSM / frame). +11 hermetic tests. fmt + clippy `-D warnings` clean; full lib suite 1400 → 1410; kg-bench
  151/151; APB/AHB/AXI/SWD eval unaffected (additive). Live-verified over all 74 persisted evidence docs:
  protocol 25 / register 11 / interface 22 / guide 16. Book section `quality/validation.md`; KM card
  `document-class-from-structure`. Commit subject: `PDF-VARIANT-DIGESTION.5a`.

- `.5c` (`2026-06-08`): front-matter doc-type signal landed — pure `front_matter_doc_type_hint` over generic
  doc-type vocabulary (guide phrasings rank above spec words; "overview"/"introduction" excluded — every spec
  has those chapters; whole-word match so "guidelines" ≠ "guide"); `classify_document` consumes it and sets
  `under_extracted_spec = (class==Guide && declared==Specification)`. `validate` reads the title + first 12
  section headings off the sibling SourceIR (the `document_profile.title` is empty in practice — the early
  headings carry the signal, confirming the owner's point that the type is stated in the early pages), adds a
  `document_type_declared` metric + a WARNING `evidence_document_underextracted_spec` finding + console lines.
  +6 hermetic tests on real corpus framings. fmt + clippy `-D warnings` clean; full lib suite 1410 → 1416;
  kg-bench 151/151; APB/AHB/AXI/SWD eval unaffected. Live: of the 16 `guide`-classed docs, 11 are TRUE guides +
  5 are UNDER-EXTRACTED specs flagged for the VLM frontier (RISC-V Advanced Interrupt Architecture, JESD235
  JEDEC STANDARD HBM, CoreSight Base System Architecture, +2). KM card `document-class-from-structure` updated.
  Commit subject: `PDF-VARIANT-DIGESTION.5c`.

- `.5b` (`2026-06-08`): class-aware per-doc completeness gauge landed — pure
  `crate::ir::completeness::document_completeness_gauge` + `DocumentCompletenessGauge`/`CompletenessGap`.
  Keyed off the `.5a` `DocumentClass`: `Guide` → `applicable=false` (never penalized for a missing surface);
  Register/Protocol/Interface → each dimension gauged only when its denominator > 0. Dimensions read off
  already-built IR (no fabrication): `registers_without_fields`, `registers_unresolved_width` (reuses the
  mandatory-width flag), `signals_without_direction` (declared inventory minus
  `collect_signals_with_explicit_direction_declarations`, now `pub(crate)`),
  `unexplained_intent_bearing_tables`. `validate <evidence>` prints a Document Completeness Gauge block +
  `document_completeness_gaps` / `document_completeness_applicable` metrics + an Info
  `evidence_document_completeness` finding. +5 hermetic tests; `cargo fmt --check` + `cargo clippy -D warnings`
  clean; full lib 1416 → 1421; kg-bench 151/151; APB/AHB/AXI/SWD eval unaffected (additive). Live over persisted
  evidence: APB (protocol) 1 gap (1/32 signals_without_direction); RISC-V Debug (register) registers_unresolved_width
  60/60 (XLEN-parametric — corroborates `.4a.2`) with all 60 registers carrying fields; Avalon (interface) 11/34
  signals + 1/13 tables; GIC overview guide → not applicable. Book `quality/validation.md` subsection; KM
  `document-class-from-structure` updated. Commit subject: `PDF-VARIANT-DIGESTION.5b`.

- `.8` (`2026-06-08`): broadened prose-actor capture via a ROBUST structural grammar. Generalized the `.3b`
  literal "is the device that/which" anchor to `agent_definitions(text)` over a conservative agent-class
  ALLOWLIST (no "unit"/"block"). An expert reviewer flagged an interim structural-noun DENYLIST as fragile →
  redesigned to pure structural signals: `strip_trailing_parenthetical` (kills "section"), current-sentence-only
  NAME search (kills cross-sentence anaphora "system"), and a no-preposition-in-subject guard (closed
  grammatical class — kills the prepositional-phrase object "signals"). The pre-existing `is_agent_noun`
  function-word denylist is UNCHANGED (no growth). +6 hermetic tests; fmt + clippy `-D warnings` clean; lib
  1421 → 1427; kg-bench 151/151. Canonical real-extractor verify (3 docs with un-reclaimed normalized): NVMe
  0→1 clean "controller"; I2C/RISC-V unchanged. Projection (Python mirror over persisted statements; canonical
  needs re-ingest): 14 → 20 docs (+6), every gained actor genuine (A76 core / ETM unit, CoreSight splitter, CCIX
  port, AXI manager, CoreSight component). APB/AHB/AXI/AXI-Stream/SWD project 0 actors (clean; additive — eval
  unaffected). Commit subject: `PDF-VARIANT-DIGESTION.8`.

## Changelog

- `2026-06-09`: `.9.11` (header-trapped timing-table recovery) DONE. Structural fix in
  `synthesize_timing_constraints`: recover timing data rows Docling trapped in `header_rows` (row-label cell
  `is_header=true` → empty `body_rows`). Additive, no list, no case. I2S 0 → 5 timing_constraints; SMBus held at
  84 (complex tables stay honest residuals). 3 hermetic tests; `run_ci.sh` GREEN (lib 1475) + kg-bench 151/151.
  Owner steer: `.9.10` parked behind the shallow-parser (no denylist — "a list to handle 100s of PDFs is a sign
  of weakness"). Commit subject: `PDF-VARIANT-DIGESTION.9.11`.
- `2026-06-09`: `.9.6` (I2S NXP UM11732 honest baseline) DONE. First-ever ingest (14 pp / 27 visual / `ready` /
  0 residuals) → evidence (24 anchors / 152 spans / 154 statements) → `validate`: `document_class: guide`,
  `document_type_declared: specification` → ⚠ under-extracted spec. Yield 2 real signals (`SCK`+`SD`) captured
  from prose, `WS` (11×) MISSED; 1 protocol_actor ("controller", `.8` grammar fires); 0 FSM/frame/relations/
  constraints; 1 conditional rule = NXP legal boilerplate noise; 0 timing_constraints despite 2
  `timing_parameter` tables (blank leading symbol column). Spun `.9.11` (blank-column timing-table recovery) +
  folded `WS` into `.9.10`. Docs-only (no code change). Commit subject: `PDF-VARIANT-DIGESTION.9.6`.
- `2026-06-09`: `.9.5` (SMBus 3.3.1 honest baseline) DONE. First-ever ingest (83 pp / 100 visual / `ready` / 0
  residuals) → evidence (139 anchors / 1076 spans / 1131 statements) → `validate`: `document_class: guide`,
  `document_type_declared: specification` → ⚠ under-extracted spec. Deterministic yield 1 real signal (`SMBCLK`)
  + 1 noise (`WIRE`); 0 FSM / 0 frame / 0 actors / 0 relations / 0 constraints; 26 conditional rules; 84
  timing_constraints; 1 register table (21 fields). Honest finding: SMBus declares `SMBCLK`/`SMBDAT`/`SMBSUS#`/
  `SMBALERT#` ONLY in prose (no signal table); the `.9.8` definitional copula misses the `#` active-low suffix +
  the descriptor word, so 3/≥4 signals missed → new prose bus-line signal lever `.9.10` (probe-locked). Docs-only
  (no code change). Commit subject: `PDF-VARIANT-DIGESTION.9.5`.
- `2026-06-08`: `.9.4` (SWP honest baseline) DONE. Ingested SWP (147 anchors / 1303 statements / 82 visual);
  `document_class: protocol` (11 signal_constraints) but 0 signals/FSM/frame/actors; the `.9.3a` quoted-mode lever
  does not fire. Honest breadth finding: SWP's facts ARE present but under-extracted differently — an FSM in a
  single-word `<NAME> state` grammar (`ACTIVATED`/`DEACTIVATED`/`Reset`), single-wire signals in prose
  (`S1`/`S2`/`SWIO`), and 11 noisy constraints from layer/protocol acronyms. Spun lever backlog `.9.7` (single-word
  FSM grammar) / `.9.8` (SWP prose signals) / `.9.9` (constraint-acronym precision). Docs-only.
- `2026-06-08`: `.9.3a` (CAN error-state FSM via agnostic quoted-mode extractor) DONE. Split `.9.3` →
  `.9.3a` (FSM, done) + `.9.3b` (frame fields, pending). New additive `extract_quoted_mode_states` (grammar:
  single-quoted name + generic actor-noun node/unit/station/device + recurrence ≥2 + ≥2 distinct states; ADR
  0006). **CAN 0 → 3 states** (error active/passive/bus off), zero fabrication/spurious; NVMe/I2C/RISC-V Debug
  stay 0; re-ingested ADI/SWD gains 0 `mode_state_*` (13 SWD states intact). 5 new tests; full `run_ci.sh` green
  (lib 1440 → 1445); kg-bench 151/151; KM `agnostic-quoted-mode-fsm` + book subsection (`pipeline/evidenceir.md`).
- `2026-06-08`: `.9.2` (CAN 2.0 honest baseline) DONE. Ingested CAN (72 pp / 98 visual / `ready` / 0 residuals),
  built evidence (269 anchors / 751 statements), validated. **Baseline: CAN is an honest UNDER-EXTRACTED spec** —
  deterministic yield 0 signals/registers/relations/constraints; the `.5a`/`.5c` machinery correctly flags it
  (`document_class: guide` + front-matter `specification` → ⚠ under-extracted, route to VLM/prose frontier). The
  honesty guardrail confirmed the frame fields + error-state FSM ARE present in CAN's prose (SOF/Arbitration/
  Control/Data/CRC/ACK/EOF; error-active/passive/bus-off + TEC/REC). Root: SWD-tuned prose extractors don't
  generalize. Spun lever `.9.3` (generalize `extract_serial_frame_fields`/`extract_protocol_states`, agnostic).
- `2026-06-08`: `.9.1` (import + register the 4 serial PDFs, git-tracked) DONE — commit `c265062b`.
- `2026-06-08`: Added `.9` (new serial-protocol class) — owner unblocked the program by downloading new serial
  chip-spec PDFs (CAN/SWP/SMBus/I2S) and directed they be copied + git-tracked into SpecForge. `.9.1` (import +
  register) → `.9.2` (CAN 2.0 honest baseline) → `.9.3` (prose serial-frame+FSM lever), ahead of the
  host-local-blocked `.6`/`.7`. Owner directive: "selected ones shall be copied and git tracked in SPECFORGE repo."
- `2026-06-08`: `.8` (broaden prose-actor capture) DONE — robust structural agent-definition grammar (general
  agent-class allowlist; parenthetical-strip + current-sentence + no-preposition guards instead of a fragile
  structural-noun denylist, after expert review). Projected 14 → 20 docs with actors (+6), all genuine; NVMe
  canonically 0→1 ("controller"); wire-based 0/clean. lib 1427; kg-bench 151/151. Frontier `.6`/`.7`.
- `2026-06-08`: `.5b` (class-aware per-doc completeness gauge) DONE → `.5` (item ②) CLOSED. `validate <evidence>`
  now reports how COMPLETE the typed intent it produced is, judged per the `.5a` class: a guide is "not
  applicable" (never penalized for 0 registers); protocol/register/interface gauge each dimension only when its
  denominator > 0. Dimensions: registers_without_fields, registers_unresolved_width, signals_without_direction,
  unexplained_intent_bearing_tables. +5 hermetic tests; lib 1421; kg-bench 151/151. Frontier moves to `.6`/`.7`/`.8`.
- `2026-06-08`: `.5b` (class-aware per-doc completeness gauge) taken `in_progress`. PNT from the `.5c` close into
  the `.5` frontier leaf. Design recorded on the `.5b` node: a pure `document_completeness_gauge` keyed off the
  `.5a` `DocumentClass` (Guide → not applicable, so a guide is never penalized; Register/Protocol/Interface →
  gauge each dimension only when its denominator > 0) over `registers_without_fields`,
  `registers_unresolved_width`, `signals_without_direction`, `unexplained_intent_bearing_tables`, surfaced in
  `validate <evidence>`. Honest gap counts, no fabrication (ADR 0006). Frontier stays `.5b` until landed.
- `2026-06-08`: `.5c` (owner-suggested front-matter/ToC doc-type signal) DONE. `validate` now reads the
  document's early headings for a self-declared type and flags a structurally-empty doc that self-declares a
  spec as UNDER-EXTRACTED (not a true guide) → VLM frontier. Live: 16 guides split into 11 true + 5
  under-extracted. Frontier moves to `.5b` (per-doc completeness gauge).
- `2026-06-08`: `.5a` (structural doc-class routing + honest guide reporting) DONE. `validate` now reports a
  `protocol`/`register`/`interface`/`guide` class from the low-noise intent surfaces (conditional-rules
  excluded as over-produced narrative noise — the real-data correction). Live distribution over 74 docs:
  protocol 25 / register 11 / interface 22 / guide 16. Frontier moves to `.5c` (owner-suggested front-matter/ToC
  doc-type signal — corroborate the class + separate a true guide from an under-extracted spec), then `.5b`.
- `2026-06-07`: Created (owner high-priority directive — digest any chip-spec PDF). `.1` triage sweep in flight.
- `2026-06-08`: Split `.4b` (proposer/verifier VLM audit) into `.4b.1` (audit harness — DONE) + `.4b.2` (live
  measurement — pending); `.4b` → `active`, frontier moves to `.4b.2`. Mirrors `.4a`'s eval-surface → per-doc
  split (the harness is the lower-level dependency of the measurement).
- `2026-06-08`: `.4b.2` live measurement DONE → `.4b` + `.4` (precision-verification item ①) CLOSED. Live
  qwen2.5vl:7b audit: RISC-V Debug 0.250/0.375 (bit-position gap, corroborates `.4a.2`), NVMe 0.750 (confirmed +
  caught a feature→timing misclassification, corroborates `.4a.3`). The audit estimate tracks gold quality
  (cross-validated). Frontier moves to `.5a` (doc-class routing + per-doc completeness gauge — item ②).
- `2026-06-08`: `.5a` (doc-class routing + honest guide reporting) taken `in_progress`; `.5` → `active`. PNT
  from the sibling `EXTRACTION-GAP-FIX` honest boundary into this HIGH-PRIORITY program's item ②. Design: a
  pure `classify_document` over a structural census (registers / behavioral obligations / signal-inventory +
  connectivity / FSM / frame) → `Protocol`/`Register`/`Interface`/`Guide`, with `Guide` as an HONEST floor for
  low-structured-design-intent docs (not a silent 0-yield miss); surfaced in `validate` as a `document_class`
  metric + an Info `evidence_document_class` finding. Agnostic (ADR 0006 — small generic structural floors, no
  chip/vendor names).
- `2026-06-08`: Split `.4a` (precision verification) into `.4a.1`–`.4a.5` — the eval scorer has no
  register-field/declared-signal task yet (a real lower-level dependency, PNT split rule). `.4` + `.4a` →
  `active`; `.4a.1` (register-field eval surface) → `in_progress` and onto the frontier. Scope kept to
  in-corpus reproducible docs (RISC-V Debug, NVMe, I2C) so gold is independently verifiable, not the
  owner-library-only CCIX/OpenCAPI/GIC.

## Tooling + multi-strategy (`2026-06-07`)

PDF reading is multi-strategy, best-wins-per-PDF ([[?]]): **docling** (structured `content_elements`, the
pipeline path) + **`scripts/pdf_text.py`** (raw text via pypdf) both read all 82 (incl. the 12
permission-encrypted ones, which open with an empty password). The Claude Read tool is unreliable here and
NOT fixable by a plugin (GitHub #38530) — use the two paths above. `scripts/decrypt_pdf.py` strips
encryption for tools that need it. pypdf+cryptography installed in `.venv-docling`. KM
`pdf-encryption-and-read-access`. **Lever A** is likewise multi-strategy: deterministic header-grammar
classifier + the VLM (Qwen2.5VL) on unknown tables, best-wins-per-PDF.

