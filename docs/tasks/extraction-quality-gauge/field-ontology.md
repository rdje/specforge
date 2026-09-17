# EXTRACTION-QUALITY-GAUGE — field ontology

- Part ID: `field-ontology`
- State: `legacy`

<!-- extraction-quality-gauge-task-source-region:field-ontology-leaves:start -->
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD` · Status: `active` (designed `2026-06-10`, split `.1`–`.4`) ·
  Goal: the SIGNAL-vs-FIELD ontology for packet/flit protocols (CHI/CXL/PCIe-class) — model fields
  (flit/message contents) distinctly from signals (physical wires), per the doc's own
  signals-vs-fields split. The real future target for CHI-like PDFs (owner: "in fine we need to
  handle such cases too").
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.1` · Status: `done` (`2026-06-10`, design + corpus probe,
  docs-only) · Goal: ground the ontology in how the corpus actually declares fields BEFORE coding
  (the `.9.7`/`.9.8` method). **Probed all 49 persisted SourceIRs with structured tables.** Findings:
  - **The document's own table-header vocabulary types its rows** — fields are declared in tables
    whose name column is field-titled (`Field` / `Field name`), signals in `Signal`-titled tables.
    CHI: **36 field tables** (captions "Request channel fields" / "Response packet fields" / "Snoop
    request fields" / "Data packet fields"; shapes `Field|Description`,
    `Field|Affects structure|Description`) yielding **79 distinct field names** — including the
    exact `.gauge`/`.6` mis-typing class (`DBID`, `TxnID`, `ReturnNID`, `Addr`, `Opcode`...); its
    signals live in `Signal|Description` tables captioned "<channel> interface signals"
    (`REQFLITV`, `REQFLITPEND`, `REQLCRDV`...). CHI-C2C: 21–124 field tables, with widths
    (`Field name|Width (bits)|Value`). CXS: "Packet control fields". DTI/USB carry the shape too.
  - **Two populations share the field-titled column** — packet docs (message fields) AND register
    docs (RISC-V Debug 57× `Field|Description|Access|Reset`, Intel VT-d 102×
    `Bits|Access|Default|Field|Description`). **Discriminator (structural, no name lists):
    register-access vocabulary columns (`Access`/`Reset`/`Default`) and/or in-register bit-position
    columns mark a REGISTER-field table** (already owned by the register surface); a field-titled
    table without them declares MESSAGE fields.
  - **Today the field tables are inert**: all 36 CHI field tables are `table_kind: unknown` — fields
    have NO typed home, so field obligations in prose can only become wrong signal constraints or
    be dropped. That is root cause #1's deepest layer.
  - **Defect discovered:** `entity_prompt` (`ir/entity_typing.rs`) literally defines
    `signal = a wire/pin/field carrying a value` — the ontology is conflated at the LLM judgment
    point itself.
  - **Out of first scope (honest residual):** CCIX declares fields as `Bit Location|Field
    Description` (name embedded in prose) and OpenCAPI as `Operand mnemonic|Field width|Description`
    — different strategies later, per multi-strategy/best-wins.
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.2` · Status: `done` (`2026-06-10`) · Goal: **capture** — the
  first-class `message_field_records` EvidenceIR surface. Shipped (`ir/evidence.rs`):
  `MessageFieldRecord` (id, name, container, optional `bit_width`, optional description,
  `supporting_table_ids` provenance) + `message_field_surface` via the `run_surface` framework
  (key = container+name; manifest entry `message_fields`; extractor
  `message_fields.container_field_table`). Gates, all structural: (1) name column is EXACTLY
  `Field`/`Field name` (a merged CCIX `Field Description` column never qualifies); (2) the
  one-place register discriminator — `is_register_field_header` (Access/Reset/Default/Type
  vocabulary) claims the table for the REGISTER surface, never here; (3) the caption must anchor
  "field(s)" to a container noun at distance ≤2 (`channel|packet|message|flit|header|frame|
  request|response` — grammar vocabulary, not a name list), so register/descriptor captions
  ("Address Fields in Remappable Interrupt Request Format", "Mode Register 0") never fire;
  (4) continuation captions ("Table B2.2 Continued from previous page") inherit the container via
  the table-ref token and MERGE provenance into the first record; (5) width read only from a
  SINGLE unqualified width column (`Width (bits)`/`Bits`), plain count or `[hi:lo]` range —
  per-variant `Width (bits) ReqS` stays honest `None`; (6) restriction/status tables
  (`Field name|Restriction`, `Field|Value|Status`) declare nothing. Deterministic (encounter
  order, lookup-only maps). **Measured live (real Rust extractor over all persisted SourceIRs, the
  `#[ignore]`d `message_field_corpus_sweep_local_measurement` harness): fires ONLY on the
  packet-protocol family — CHI 106 fields / 4 containers (Request channel / Response packet /
  Snoop request / Data packet; `TxnID`, `DBID`, `Opcode`... — the exact `.gauge` mis-typing class,
  now typed), CHI-C2C 149/143/189 fields (89/93/164 with width), CCIX 1.x 47/50/51 (~95% with
  width), CXS 1; zero on every register/wire doc.** No-regression PROVEN: git-stash before/after
  rebuild of all 12 intact-bundle docs (incl. RISC-V Debug's 60 register field-tables, AXI's MPAM
  sub-field tables, SMBus) — byte-identical except the additive `message_fields` manifest entry,
  zero message fields each. +6 pure tests (lib 1524) + kg-bench fixtures
  `message_field_table_gold` (fields in `message_field_records` with widths + honest-absence lock,
  NOT in canonical signal inventory) and `message_field_register_table_negative` (register-vocab
  table captioned "message fields" → zero) with new kg-bench assertion keys
  `message_field_count`/`message_fields_include` (incl. `bit_width_absent`)/
  `message_field_names_exclude` — kg-bench 153/153. `run_ci.sh` GREEN. Honest residuals recorded:
  CCIX 2.0 `Bit Location|Field Description`, OpenCAPI `Operand mnemonic`, USB descriptor tables
  (no container bigram) = later strategies; validate/document-class integration deliberately
  deferred. Book: `pipeline/evidenceir.md` subsection. KM [[message-field-records-surface]].
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.3` · Status: `done` (`2026-06-10`) · Goal: **discriminate** —
  the ontology is now in the entity-typing gate. Shipped (`ir/entity_typing.rs`):
  `EntityType::Field` (parse accepts `field`; `as_str` = `field`),
  `EntityEvidence.declared_in_field_table` gathered from the NEW `message_field_records` catalog
  (case-insensitive), grounding rule in `classify_entity` — declared in a field table and in no
  signal table → authoritatively `Field` with NO LLM call; a signal-table declaration outranks
  when a name is in both; `is_valid_signal_subject(Field) == false`, so the `DBID`/`TxnID` class
  can never be a constraint/relation subject. The conflated prompt defect (`.FIELD.1`) is fixed:
  `signal = a physical wire/pin`, `field = a named portion of a packet/flit/message payload (not a
  wire)`, plus a `Declared in a message-field table:` evidence line. All three consumers
  (`extract-constraints-llm`, `entity-type`, `eval-extraction`) inherit through
  `gather_entity_evidence`. **Probed live (qwen2.5:14b-instruct, temp 0) before shipping (the `.8`
  method): controls UNCHANGED — `TXSACTIVE`→signal (the `.1` recovery preserved),
  `LICENSEE`→boilerplate, `CMO`→transaction; undeclared-field boundary characterized honestly —
  "the ReturnNID field …" phrasing → `field`, a bare field-word-free mention → `signal` (same as
  pre-slice; the deterministic catalog, not the model, carries declared fields).** +3 unit tests +
  1 end-to-end test (field table on persisted SourceIR → `message_field_records` → typed `Field` →
  rejected as signal subject; lib 1527); kg-bench 153/153; `run_ci.sh` GREEN. Live CHI constraint
  re-measure stays bound to `.FIELD.4` (CHI normalized bundle cleaned; PDF host-local).
  Book: the `extract-constraints-llm` section now documents the deterministic field rejection.
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.4` · Status: `done` (`2026-06-10`) · Goal: **capture the
  intent** — field-subject obligations become field-scoped constraints instead of dropped, plus
  the CHI-class re-measure on persisted artifacts. **DECISION: a parallel
  `message_field_constraints` EvidenceIR surface, NOT a subject-kind discriminator on
  `SignalConstraintRecord`** — a discriminator would force every downstream signal-constraint
  consumer (eval canonical keys, nli-verify, semantic carry-through, the ISF adapter) to filter
  by kind or silently keep treating fields as wires, which is the exact mis-typing this tree
  exists to kill; a separate surface keeps the canonical signal surface wires-only by
  construction. Shipped: `MessageFieldConstraintRecord` (subject_field + catalog `containers`
  provenance + the SHARED `SignalConstraintKind` vocabulary — what a requirement can SAY is the
  same, what it is ABOUT differs — + same condition/value/statement provenance shape) beside
  `MessageFieldRecord`; serde-additive `EvidenceIr.message_field_constraints` (old artifacts
  load; empty surface serializes to nothing); `ground_constraint_typed` in
  `ir/constraint_extract_llm.rs` dispatches on the `.FIELD.3` entity type AFTER the shared gates
  (`.3a` condition-only, `.3b` permissive-frame, `.8` value recovery, `.2` condition grounding —
  a field obligation passes the SAME discipline, fixture-locked), with `ground_constraint` kept
  as the signal-only view (a field subject still never reaches `signal_constraints`);
  `dedup_merge_by` generalizes the `.4` provenance-merging dedup over both surfaces
  (`dedup_field_constraints` keyed subject+kind+value+negation+condition; containers are catalog
  provenance, not identity); `extract-constraints-llm` routes Field records to the new surface
  (replace semantics, `llm_fieldcon_NNNN` ids, separate report line). +5 pure tests + the
  `.FIELD.3` end-to-end test extended through the REAL entity-typing composition (lib 1532);
  kg-bench 153/153. **Measured live (qwen2.5:14b-instruct temp 0; persisted CHI evidence via the
  `.8` redirected-copy protocol; catalog injected from the REAL `.FIELD.2` extractor over
  persisted SourceIR — 106 fields, reproducing `.FIELD.2` exactly; new `#[ignore]`d
  `message_field_catalog_dump_local_measurement` harness): BASELINE (no catalog) mis-types
  `TagOp must_be_value 0` + `PBHA must_be_value 0` as SIGNAL constraints; WITH CATALOG the
  signal surface is EXACTLY the document's 4 real flit-valid wires
  (`REQFLITV`/`RSPFLITV`/`SNPFLITV`/`DATFLITV` `must_be_high`, conditions preserved) and the 2
  field obligations land in `message_field_constraints` with containers (`TagOp` → Request
  channel/Response packet/Data packet) and merged provenance (the twice-stated TagOp fact = ONE
  record, both statement ids; dedup 3→2). The Pattern artifact's 5 junk `REQ must_be_value`
  records are gone in both runs. Wire controls (extract-constraints-llm on redirected
  APB/AHB/AXI copies + `eval-extraction --provider skip`): volumes EXACTLY the `.3b`/`.4` state
  (APB 20, AHB 12, AXI 54→50 w/ 4 merged), zero field constraints, P=R=F1=1.000 ×3, doc-level
  recall 16/16, conformal empirical_error 0.000.** Honest residuals: (a) field PRESENCE
  requirements ("the MPAM field must be included on the REQ and SNP channels") have no
  constraint-kind slot — probed live, model outputs `[]` (the `.8` root-cause shape; a future
  probed kind-vocabulary slice, NOT guessed); (b) the FULL fresh-yield CHI gauge re-measure
  (the original 162-constraint vintage) still needs re-ingest — PDF host-local, request
  re-provision; (c) validate/document-class integration of the field surfaces stays deliberately
  deferred (same as `.FIELD.2`). Book: `pipeline/evidenceir.md` `.FIELD.4` subsection +
  `commands/quality-and-learning.md` routing rewrite. KM [[message-field-constraints-surface]].
<!-- extraction-quality-gauge-task-source-region:field-ontology-leaves:end -->
