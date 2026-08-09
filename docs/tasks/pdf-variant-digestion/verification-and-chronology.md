# PDF-VARIANT-DIGESTION — verification and chronology

- Part ID: `verification-and-chronology`
- State: `legacy`

<!-- active-task-source-region:verification:start -->
## Verification log

- `.13b` (`2026-06-14`, ACE measurement — completes the 4-doc leaf): fresh `evidence` rebuild
  of ACE (`ihi0022_h_c…`) completes at 52.3 MB max RSS (no OOM). **201 `signal_presence_records`**
  on tables 0271–0274 / 0276–0279 (0275 refused), 59 conditioned. **+9 gap-fill wires** minted
  with per-item-verified widths (AWBAR 2, AWDOMAIN 2, AWSNOOP 4, CRRESP 5, CDDATA V, 4×BROADCAST*
  1). `validate`: `unexplained_intent_bearing_tables` 35→30, `document_class: protocol` / declared
  `specification`, presence inventory finding fires, manifest fires `signal_presence.matrix_table`.
  **Gauge re-measured live** (qwen2.5:14b-instruct): 78/97 not-entailed (80.4%) / 19 entailed / 0
  abstained — consistent with the standing 76/94. APB_d/ATB/LTI were done+verified pre-crash;
  ACE was gated by `.13b.1` (fixed this session). Measurement-only slice (`generated/` untracked;
  no code change). Commit subject: `PDF-VARIANT-DIGESTION.13b`.
- `.13b.1` (`2026-06-14`, BLOCKER fix): char-correct UTF-8 copy in
  `replace_term_with_placeholder` (`crates/specforge/src/ir/prior_memory.rs`) replacing the
  `bytes[index] as char` per-byte copy that mangled non-ASCII into mojibake and, chained one
  pass per multi-word term in `normalize_prior_phrase`, re-doubled it exponentially (the ACE
  OOM). +4 hermetic regression tests. lib 1583→**1587**; kg-bench **156/156**; full
  `scripts/run_ci.sh` GREEN. **Byte-stability re-proof:** pre-fix vs post-fix fresh
  `evidence --dry-run` over the 15 non-ACE intact bundles = **all BYTE-IDENTICAL** (ACE
  excluded — it OOMs pre-fix), so the fix is a pure no-op on the intact corpus. **ACE
  evidence rebuild now COMPLETES: 21 s / 56 MB max RSS** (was 419 s / 17.2 GB RSS / SIGKILL
  exit 137) — blocker resolved, `.13b` ACE measurement unblocked. KM card
  `prior-phrase-utf8-byte-as-char`. Commit subject: `PDF-VARIANT-DIGESTION.13b.1`.
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

<!-- active-task-source-region:verification:end -->

<!-- active-task-source-region:changelog:start -->
## Changelog

- `2026-06-24`: `.10h` (block-qualified register-mnemonic recovery — the `.10g` residual) DONE,
  measurement-first/CODE/GO. `extract_section_header_registers` (`ir/evidence.rs`) replaces the
  `.10g` drop-all-duplicates gate with a field-set-containment resolver (new pure helper
  `collapse_section_header_register_identity`): a mnemonic reused across ≥2 register-routed
  section-heading containers collapses to ONE record IFF every occurrence's field set is a subset
  of one maximal occurrence (identical cross-references + nested views of one register), keeping
  that fullest occurrence's real layout; disjoint/partial-overlap sets (≥2 genuinely-different
  registers — MEM-AP `CSW` vs JTAG-AP `CSW`) stay an honest residual. Probe-measured the residual
  lives in EXACTLY 2 docs; recovered ARM-Debug 12→15 regs / 57→69 fields (`AUTHSTATUS`/`DEVARCH`/
  `IDR`) + CoreSight 5→6 / 24→29 (`AUTHSTATUS`); full `evidence` register_records ARM-Debug 37→40,
  CoreSight 26→27. `git stash` baseline diff: 8 register/wire/section-header/message golds
  byte-identical, the 2 changed docs ADD records with ZERO removals (all baseline records
  byte-identically preserved). +3 hermetic tests + 1 `#[ignore]` probe; lib 1718→1721; kg-bench
  156/156; full `run_ci.sh` GREEN. ADR-0006 (universal field-set containment, no name list). Book
  `pipeline/evidenceir.md`; KM `section-header-register-identity-collapse`. Commit subject:
  `PDF-VARIANT-DIGESTION.10h`.
- `2026-06-14`: `.13b` (4 AMBA matrix docs) DONE — ACE measurement completed the leaf. ACE
  evidence rebuilt (52.3 MB, no OOM): 201 signal_presence_records on 0271–0274/0276–0279 (0275
  refused, 59 conditioned); +9 gap-fill wires minted with verified widths; unexplained tables
  35→30; gauge re-measured 78/97 not-entailed (qwen2.5:14b-instruct). APB_d/ATB/LTI were done
  pre-crash. Measurement-only (no code; `generated/` untracked). Commit subject:
  `PDF-VARIANT-DIGESTION.13b`.
- `2026-06-14`: `.13b.1` (BLOCKER fix) DONE. `replace_term_with_placeholder`
  (`crates/specforge/src/ir/prior_memory.rs`) now copies one whole UTF-8 char in its `else`
  arm instead of `bytes[index] as char`; the old per-byte copy mangled non-ASCII into
  mojibake and, chained one pass per multi-word term by `normalize_prior_phrase`, re-doubled
  it exponentially — the ACE evidence build hit 17.2 GB RSS / 419 s / SIGKILL. Byte-identical
  on pure-ASCII by construction. +4 hermetic tests; lib 1583→1587; kg-bench 156/156; full
  `run_ci.sh` GREEN. Byte-stability re-proof: 15/15 non-ACE intact bundles BYTE-IDENTICAL
  pre-fix vs post-fix. ACE evidence now completes (21 s / 56 MB) → `.13b` ACE measurement
  unblocked. KM `prior-phrase-utf8-byte-as-char`. Commit subject: `PDF-VARIANT-DIGESTION.13b.1`.
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

<!-- active-task-source-region:changelog:end -->

