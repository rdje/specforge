# PDF-VARIANT-DIGESTION — activity 09 serial protocols

- Part ID: `activity-09-serial`
- State: `legacy`

<!-- active-task-source-region:serial-nodes:start -->
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
<!-- active-task-source-region:serial-nodes:end -->

<!-- active-task-source-region:serial-frontier-history:start -->
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
<!-- active-task-source-region:serial-frontier-history:end -->

