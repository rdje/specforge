# EXTRACTION-GAP-FIX: close the extraction gaps PDF-VARIANT-DIGESTION.4 quantified

## Metadata

- Tree ID: `EXTRACTION-GAP-FIX`
- Status: `active`
- Roadmap lane: `R15`/`R16` (extraction quality — the FIX half of the measure→catch→fix loop)
- Created: `2026-06-08`
- Parent: owner decision (`2026-06-08`, the pivot pre-flagged in `PDF-VARIANT-DIGESTION` MEMORY) —
  "pivot to FIXING the now-quantified extraction gaps instead of more measurement". `PDF-VARIANT-DIGESTION.4`
  (per-fact gold `.4a` + VLM audit `.4b`, cross-validated) measured exactly WHERE the broadened table/prose
  extraction is weak; this tree converts that measured weakness into real capability.

## Goal (turn measured weakness into real capability)

`PDF-VARIANT-DIGESTION.4a`/`.4b` left four concrete, source-verified, per-fact-measured extraction gaps. Close
each with a GENERAL, ADR-0006-agnostic fix (structural/grammar only — no chip/vendor/protocol names in the
runtime), re-measure against the SAME in-corpus gold that quantified it (no faking — [[feedback_scoring_rigor]]),
and never regress the four wire-based specs (APB/AHB/AXI/SWD = 100%) or kg-bench.

## Honesty guardrail (owner, `2026-06-08`, non-negotiable): you cannot extract what is not there

**You can only extract a fact that is PRESENT in some readable modality.** A low recall number is NOT a licence
to invent the missing value. So every leaf here must FIRST establish WHERE the fact actually lives, then read
exactly that modality:

- the fact is in a DIFFERENT COLUMN/region the extractor isn't reading (NVMe mnemonic → the description column;
  `.2`) → read that column;
- the fact is in the surrounding TEXT, not the table (RISC-V register name → the preceding heading; `.3`) →
  associate from there, but only when the association is reliable;
- the fact is in a NON-TEXT modality (RISC-V bit positions → the layout GRAPHIC, not the table; `.4`) → read the
  graphic (VLM, where the bits exist), bounded/targeted;
- the entity is NOT a real fact of the wanted kind at all (I2C ACK/NACK/DDC/SDR are conditions/other-bus/rate
  acronyms, not bus wires; `.1`) → REMOVE it (raise precision).

**If a fact is genuinely absent from every readable modality, the honest outcome is a completeness RESIDUAL /
an honest gap, NEVER a fabricated value** (e.g. do not synthesize a register width, a bit range, a field name,
or a register name that the document does not state somewhere). A leaf that cannot find the fact where it
actually lives closes as "verified-absent / honest residual", not as a faked improvement.

## The four quantified gaps (from `PDF-VARIANT-DIGESTION.4`)

| gap | measured | where |
|---|---|---|
| I2C prose over-capture | declared-signal precision 6/10 = 0.600 (ACK/NACK/DDC/SDR are not bus signals) — **CLOSED `.1` → 1.000** | `.4a.5` |
| NVMe mnemonic-from-description | field-name recall 0/29 (the mnemonic lives in the DESCRIPTION; `field_name` is the bit-range) — **CLOSED `.2` → 28/29 = 0.966** | `.4a.3` |
| RISC-V register-name | register-name association 0/60 (synthetic `register_<table_id>` — name is in the preceding heading) — **CLOSED `.3` → 59/60, 0 false names** | `.4a.2` |
| RISC-V bit-layout graphic | bit-extent 0/179 (bit positions live in the layout GRAPHIC, not the field table) | `.4a.2`, `.4b` (VLM flags "lacks bit positions") |

## Non-goals

- NOT IntentIR→ISF lowering (deferred by the owner, as in `PDF-VARIANT-DIGESTION`).
- Do NOT regress APB/AHB/AXI (constraints/relations/temporal) or SWD (frame/operations/FSM) = 100%; kg-bench green.
- No chip/vendor/protocol names in the runtime (ADR 0006); concrete names stay in `test_data/llm_eval/seed_*.json`
  golds + `#[cfg(test)]` fixtures only.

## Acceptance criteria

- Each leaf: a general (agnostic) fix + re-measurement against the owning `.4a` gold showing the targeted metric
  improve, with no wire-based regression; full `scripts/run_ci.sh` green; a KM card / book note per durable change.

## Task tree

- ID: `EXTRACTION-GAP-FIX` · Status: `active` · Children: `.1`–`.4`, `.5`, `.5a`, `.5b`
- ID: `EXTRACTION-GAP-FIX.1` · Status: `done` (`2026-06-08`) · Goal: **I2C prose acronym/condition precision
  filter** — tighten the prose signal capture (`synthesize_signal_declarations_from_prose`, `.3a`) so it stops
  emitting entities prose introduces as something OTHER than a bus wire. **DONE — the agnostic fix is a
  noun-phrase HEAD rule:** the parenthetical form `"<descriptor> (NAME)"` required only that SOME word in a
  4-word window be a descriptor, so a non-wire head still qualified — "An acknowledge clock **pulse** (ACK)"
  and "Not Acknowledge clock **pulse** (NACK)" via `clock`, "Display Data **Channel** (DDC)" and "standard
  data **rate** (SDR)" via `data`. The fix requires the IMMEDIATE head noun (the word right before the
  abbreviation, or the fused prefix in `line(NAME)`) to be the wire noun (same descriptor set:
  line/signal/clock/data/wire/bus/pin) — so the real lines survive (`serial data **line** (SDA)`, `serial
  **clock** (USCL)`, `high-speed **data** (SDAH)`) and the four over-captures drop. **NO denylist of those
  tokens** — pure general grammar (ADR 0006). **Re-measured on fresh re-ingested I2C evidence
  (`DOCLING_DEVICE=cpu`) against the `.4a.5` gold: declared-signal complete-gold precision 0.600 → 1.000
  (fp 4 → 0), recall held 1.000 (all 6 real signals: SDA/SCL/USCL/USDA/SDAH/SCLH).** +1 hermetic test
  reproducing the exact 10 I2C prose contexts (6 kept / 4 dropped); existing parenthetical tests still pass.
  No wire-based regression (the parenthetical path is disabled for ≥8-table-signal specs; SWD uses the
  pin-appositive path); full `run_ci.sh` green (lib 1378 → 1379); kg-bench 151/151. Commit subject:
  `EXTRACTION-GAP-FIX.1`.
- ID: `EXTRACTION-GAP-FIX.2` · Status: `done` (`2026-06-08`) · Goal: **NVMe mnemonic-from-description** — when
  a register-field table's name column is itself a bit-range (so `field_name` is e.g. `07:04`), recover the
  field mnemonic from the description. **DONE — but the real structure differed from the hypothesis (honesty
  guardrail: read where the fact actually lives).** Inspecting the persisted NVMe SourceIR proved the mnemonic
  is NOT a leading `MQES:` token; it is the parenthesized abbreviation in the universal defined-term prefix
  `Full Field Name (MNEMONIC): …` (e.g. *"Maximum Queue Entries Supported (MQES): …"*). So the fix recovers the
  first `(<MNEMONIC>):` group (uppercase-alphanumeric token, the `:` required so a passing reference like
  `(CC.MPS)` is not picked) — `field_mnemonic_from_description` + `is_bit_range_token` + `is_field_mnemonic_token`
  in `ir/evidence.rs`. Applied in BOTH register paths (`synthesize_register_field_tables` AND
  `synthesize_register_records`) so it fires whichever path a doc's classification routes the table through.
  Gate: only when there is no dedicated name/field column AND the captured name is a bit-range token. **NO
  leading-bare-token path** — that form appears in no in-corpus gold and risks false positives (`RO:`, enum
  `00:`), so the grounded parenthetical form is the only one implemented (scoring rigor). **Honesty guardrail:
  no defined term → the bit-range stays as an honest residual, never fabricated.** **Re-measured on the
  `.4a.3` NVMe gold (same persisted source, stash-isolated baseline): field-name recall 0/29 → 28/29 = 0.966
  (the lone miss `CAP.CRMS` has no clean `(CRMS):` term in the source → verified-absent residual), bit-structure
  recall held 27/29 → 28/29.** +2 hermetic tests (NVMe-style parenthetical recovery + grammar-only edge cases);
  existing register-field tests unchanged. No wire-based regression (the path only fires on bits-only
  register-field tables; APB/AHB/AXI/SWD have none); full `run_ci.sh` green (lib 1379 → 1381); kg-bench 151/151.
  Commit subject: `EXTRACTION-GAP-FIX.2`.
- ID: `EXTRACTION-GAP-FIX.3` · Status: `done` (`2026-06-08`) · Goal: **RISC-V register-name-from-heading** —
  associate a register-field table's register name with the preceding section heading instead of the synthetic
  `register_<table_id>`. **DONE — the open question is resolved: a reliable PAGE-BASED structural anchor exists
  (the `.2c` deferral was about `content_elements` reading-order, but page-number proximity is enough — each
  register subsection's heading and its field table sit on the same/adjacent page).** Inspecting the persisted
  RISC-V Debug SourceIR proved the name lives in a heading like *"3.14.1. Debug Module Status (dmstatus, at
  0x11)"* — a parenthetical whose leading identifier is the register name and which also carries the register's
  hex ADDRESS. `register_name_from_heading` recovers it; `contains_hex_address` (`0x<hex>`) is the gate that
  separates a register-definition heading from an arbitrary prose parenthetical (so a name is NEVER fabricated);
  `nearest_section_title_original` does the page-based association; wired as a fallback after the caption path in
  `synthesize_register_field_tables`. The address co-occurrence is a UNIVERSAL register-map fact (name +
  address), not a chip name — ADR 0006 preserved (no RISC-V phrasing hardcoded; works for any spec that names +
  addresses registers in headings). **Honesty guardrail: no defining heading → synthetic name kept (residual),
  never invented.** **Re-measured on the `.4a.2` RISC-V gold (same persisted source, stash-isolated): register-
  name association 0/60 → 59/60** (`dmstatus`/`dmcontrol` correct; the 1 residual heading has no address
  parenthetical), **0 fabricated names** (every recovered name traces to a defining heading — verified), field-
  name recall unchanged 20/34. Strict per-fact recall stays 0.000 (bits 0/179 live in the layout graphic → `.4`,
  not the name). +2 hermetic tests; no wire-based regression; full `run_ci.sh` green (lib 1381 → 1383); kg-bench
  151/151. Commit subject: `EXTRACTION-GAP-FIX.3`.
- ID: `EXTRACTION-GAP-FIX.4` · Status: `done` at an HONEST BOUNDARY (`2026-06-08`; `.4a` built + `.4b`
  measured) · Goal: **RISC-V bit-layout-graphic parse** — recover bit positions for register fields whose bits
  live in the layout GRAPHIC, not the field table (bit-extent 0/179). **Outcome: the recovery MACHINERY is built
  (`.4a`, hermetic 14/14 on clean widths) and LIVE-MEASURED (`.4b`), the honesty guardrail is fully validated on
  real data (zero fabrication), but the bit-extent metric is UNCHANGED at 0/179** because the local `qwen2.5vl:7b`
  read on these dense diagrams is not clean enough to pass the standard-width gate (spurious reserved cells /
  off-by-one → sum ≠ 32 → honest residual) AND two upstream plumbing gaps block auto-resolution (diagrams
  classified `unknown` not `RegisterBitfield`; field tables fragmented). Closing the metric is the proposed
  follow-up `.4c` (plumbing + VLM-read robustness / stronger VLM). This is the owner's honesty guardrail in
  action: you do not fabricate a bit you cannot read cleanly. **Investigation `2026-06-08` resolved the open
  question and produced a validated design (the hard intellectual work of the leaf):**
  - **The deterministic table path FABRICATES — rejected.** Docling captures the bit diagram as a table
    (`table_0020` for dmstatus) but garbles it: the low half matches the gold exactly, the high half is wrong
    (`ndmresetpending` captured bit 16, gold 24) and reserved-gap bits are dropped. Reconstructing from it would
    synthesize wrong bits → violates the honesty guardrail. NOT used.
  - **The bits live in the rendered diagram IMAGE** (`normalized/assets/table-0020.png`, `picture-0020.png`),
    where the bit-position labels are correct and match the gold.
  - **The local VLM (`qwen2.5vl:7b`) reads field NAMES + ORDER + per-field WIDTHS reliably, but misreads
    absolute MSB positions of WIDE fields** (it grabs the wrong one of the two edge-numbers a wide cell prints,
    e.g. `hartsello` → 16 instead of 25, cascading an off-by-N). Confirmed on BOTH dmstatus and dmcontrol.
  - **DESIGN (validated): discard the VLM's absolute positions; reconstruct them from order + widths by
    cumulative LSB tiling** (a register's fields tile it with no gaps — a structural law the VLM can't violate).
    Verified in python against the gold: **dmcontrol → 14/14 exact** (`hartsello [25:16]`, `hartselhi [15:6]`,
    …). A **tiling gate** keeps it honest: dmstatus's wide reserved field (width 7) is VLM-misread as width 1 →
    widths sum to 26 ≠ 32 → **gate rejects → honest residual** (no fabrication), exactly the guardrail.
  - Splits into `.4a` (build the tiling-gated VLM recovery) + `.4b` (live measurement on the `.4a.2` gold).
- ID: `EXTRACTION-GAP-FIX.4a` · Status: `done` (`2026-06-08`) · Goal: **build the tiling-gated register-diagram
  bit recovery.** **DONE exactly to the validated `.4` design.** New **pure core** `ir/register_bits.rs`:
  `RegisterDiagramFieldProposal {field_name, width}`, `reconstruct_bits_by_tiling` (cumulative LSB tiling that
  returns `None` unless the widths sum to a standard register width — gate (a)), `proposal_names_match_fields`
  (exact MULTISET match against the register's own field table — gate (b); exact-not-subset so a hallucinated,
  dropped, or renamed field is rejected), and `recover_bits_for_register` (attaches bits only when BOTH gates
  pass and EVERY field is currently bits-missing — never overrides deterministic table extraction — else a typed
  `RegisterBitRecoveryOutcome` residual). PURE: no I/O, no chip names (ADR 0006 — only the universal register
  widths 8/16/32/64/128 + the "fields tile a register" structural law, in the same spirit as the logic-level
  "how"). New **command** `recover-register-bits <evidence-ir> [--vlm-provider …] [--vlm-model …] [--dry-run]`
  (`commands/recover_register_bits.rs`, EvidenceIR-in/out, mirrors `nlp-enrich`): loads the sibling SourceIR via
  `EvidenceIr.source_ir_path`, maps each bits-missing register (`regfld_<table_id>`) → source table → page → a
  `RegisterBitfield` visual asset on that page (caption-name preferred, refuses to guess among several), reads
  `(name,width)` MSB→LSB via the shared `enrich::vlm_image_query` transport + the `SPECFORGE_VLM_HELPER` hermetic
  hook, runs the gated core, writes back. Default `--vlm-provider skip` = CI-safe no-op; **the live VLM is NOT a
  CI dependency**. Wiring the command makes the pure core production-reachable, so the helpers do not trip
  `dead_code` under `-D warnings` (the noted hazard). **Hermetic tests use the REAL
  `seed_riscv_debug_registers.json` data:** `dmcontrol` (14 named fields tile all 32 bits) → recovered exactly
  (incl. `hartsello [25:16]`, `hartselhi [15:6]`); `dmstatus` (reserved gaps + the width-7 reserved field
  misread as 1) → honest residual, zero bits; name-mismatch even when the widths tile 32 → residual; + parser,
  helper-hook, skip-noop, and end-to-end command tests. fmt + clippy `-D warnings` clean; full lib suite 1383 →
  **1400**; kg-bench 151/151; no wire-based regression (the path only fires on bits-missing register-field
  tables; APB/AHB/AXI/SWD have none). Commit subject: `EXTRACTION-GAP-FIX.4a`. KM card
  `register-diagram-bit-recovery-via-tiling`. Live measurement of the bit-extent metric is `.4b`.
- ID: `EXTRACTION-GAP-FIX.4b` · Status: `done` (`2026-06-08`) · Goal: **live measurement** — run `.4a` with the
  live `qwen2.5vl:7b` on the RISC-V Debug register diagrams and re-measure bit-extent. **DONE — honest result:
  the metric did NOT improve (bit-extent stays 0/179), and that is the CORRECT, non-fabricating outcome on this
  data.** Three findings, all evidence-backed (no code change — ran the existing command + direct VLM probes):
  - **End-to-end live run** (`recover-register-bits … --vlm-provider ollama` on the re-ingested RISC-V evidence,
    `ollama` up, `qwen2.5vl:7b` present): **all 60 register records → `residual_no_diagram`, 0 fields written,
    evidence untouched** — zero fabrication. The command's resolution does not reach the VLM because **(1)** the
    bit-layout diagrams are ingest-classified `diagram_kind=unknown`, not `RegisterBitfield` (so the resolver's
    diagram filter misses them), and **(2)** the field-definition tables are **FRAGMENTED** across multiple
    Docling tables (dmcontrol split across `regfld_table_0023/0024/0025` = 1+5+7 fields; dmstatus only 5 of 20),
    so no single `RegisterRecord` holds a register's full field set — gate (b) could not match even if a diagram
    resolved.
  - **Direct live VLM probes** (bypassing resolution — fed `picture-0020.png` (dmcontrol) and `table-0020.png`
    (dmstatus) to `qwen2.5vl:7b` with the command's prompt, temp 0): the model reads the field **names + order
    correctly** but makes **width/reserved errors** — dmcontrol: the 14 named widths are right but it inserts a
    spurious width-5 `reserved` cell where there is no gap → widths sum **37 ≠ 32**; dmstatus: the reserved
    cells appear but an off-by-one → sum **33 ≠ 32**. **Every erroneous read fails the standard-width tiling gate
    (a) → honest residual.** So the local VLM's read on these dense diagrams is not clean enough to pass the
    gate.
  - **The guardrail is fully validated on real data:** every real VLM error (spurious reserved, off-by-one) was
    caught by gate (a); nothing was fabricated; bit-extent held at 0/179. The tiling MATH is proven correct by
    the `.4a` hermetic tests (14/14 on clean widths); the live LOCAL model simply does not produce clean enough
    widths here. As `.4` anticipated, **a stronger VLM is the complementary lever** (it would rescue these
    reads), alongside two upstream plumbing fixes: classify register bit-layout diagrams as `RegisterBitfield`,
    and de-fragment/merge a register's field tables. Those are the honest follow-up (proposed `.4c` plumbing +
    a VLM-read-robustness leaf — voting/upscaling/sharper prompt). Docs-only. Commit subject:
    `EXTRACTION-GAP-FIX.4b`.

- ID: `EXTRACTION-GAP-FIX.4c` · Status: `done` (`2026-06-08`; de-fragmentation half; diagram-classification +
  VLM-robustness stay future) · Goal: **de-fragment a register's field tables** — the first of the two `.4b` plumbing gaps (the other, classify register bit-layout diagrams as
  `RegisterBitfield`, + VLM-read robustness stay future). On RISC-V Debug a register's fields are SPLIT across
  multiple Docling tables → multiple `RegisterRecord`s for one register (dmcontrol `regfld_table_0023/0024/0025` =
  1+5+7 fields; dcsr 4 records; mcontrol6 6) so no single record holds the full field set (blocks bit-recovery
  gate (b) AND inflates the record count). **Design (`2026-06-08`):** a pure `consolidate_register_field_fragments`
  that groups records by recovered `register_name` and merges a group ONLY through SAFETY GATES that leave the
  real hazards un-merged (honesty guardrail — never fabricate a register's field set): (gate 1) no fragment has
  an internal DUPLICATE field name (rejects the garbled mcontrol `regfld_table_0075` = `sizelo`×13), (gate 2) the
  field-name sets are PAIRWISE DISJOINT across the group (rejects array-collapsed `sbaddress3`×4 all `address`,
  `custom0`×3 all `data` — distinct registers `.3` mapped to one heading), and only real recovered names (not the
  synthetic `register_<id>`, which is unique per record anyway). A safe merge unions the fields in fragment order
  + unions `supporting_statement_ids`, emitted deterministically at the first fragment's position. Wired into the
  base `evidence` build so a standalone `evidence` is de-fragmented (matches what the merged record needs).
  VERIFY on RISC-V Debug (on-disk `normalized/`, in `corpus/`): dmcontrol/dcsr/mcontrol6 merge to one complete
  record; sbaddress3/custom0/mcontrol stay split (honest); `.4a.2`/`.4a.3` golds hold (register-scoped recall
  pools fragments → unchanged); NVMe distinct-name registers unaffected; APB/AHB/AXI/SWD have no register-field
  tables → unaffected; kg-bench 151/151. ADR 0006 (structural, no chip names). Does NOT touch the wire-based
  constraint surface.
  **DONE — pure `consolidate_register_field_fragments` + `register_fragments_are_safe_to_merge` (the two safety
  gates reduce to ONE test: all field names across the same-name group are distinct case-insensitively) +
  `merge_register_fragments` (union fields in fragment order, union supporting ids, recompute width from the full
  field set, emitted at the first fragment's position → deterministic order), wired into the base `evidence` build
  after register synthesis. +6 hermetic tests on real shapes (dmcontrol 3→1/13 fields; garbled `sizelo`×13 blocks;
  array `sbaddress3`×address blocks; distinct-name order preserved; singleton untouched; case-only repeat blocks).
  fmt + clippy `-D warnings` clean; full lib 1427 → 1433; kg-bench 151/151. VERIFIED live on the on-disk/in-corpus
  docs: **RISC-V Debug 60 → 44 register records** (16 fragments merged — dmcontrol now ONE record with all 13
  fields; the hazards custom0/mcontrol/sbaddress3/icount/textra64 honestly stay split), **NVMe 44 → 42** (2 safe
  merges; CAP/CC/PMRCAP conservatively left). Golds HOLD (`.4a.2` RISC-V field-name recall 20/34=0.588 unchanged,
  register-name association 59/60→43/44 same ~0.98; `.4a.3` NVMe field-name 28/29, bit-structure 28/29 unchanged).
  The `.5b` completeness gauge is visibly cleaner (dmcontrol once, complete). Also ENABLES `recover-register-bits`
  gate (b) for the merged registers. Commit subject: `EXTRACTION-GAP-FIX.4c`. KM card
  `register-field-table-defragmentation`.

- ID: `EXTRACTION-GAP-FIX.4d` · Status: `done` (`2026-06-08`) · Goal: **close `.4b` plumbing gap #1
  — the `recover-register-bits` resolver skips register bit-layout diagrams the ingest classifier left
  `diagram_kind=unknown`** (`resolve_diagram_image_for_register`'s fallback filters `RegisterBitfield` only). With
  gap #2 (de-fragmentation, `.4c`) already fixed, this is the last plumbing piece. **Design:** after the existing
  caption-name path and the unique-`RegisterBitfield` fallback, add a final safe path — when the register's page
  carries EXACTLY ONE diagram image, use it regardless of `diagram_kind` (a single image on a register-definition
  page is unambiguously that register's layout; >1 image stays a residual — never guess). Safe because the
  downstream tiling-width gate (a) + name-match gate (b) still reject any wrong read → never a fabrication
  (honesty guardrail). Hermetic test (an `unknown` single-page diagram resolves; two images → residual). Then RUN
  LIVE (`--vlm-provider ollama`, `qwen2.5vl:7b` up) on the re-ingested/de-fragmented RISC-V Debug evidence and
  re-measure bit-extent — EXPECTED honest outcome (per `.4b`): more registers now resolve to a diagram, the local
  VLM still mis-sizes most dense reads → gate rejects → residual, so the metric stays ~0 with BOTH plumbing gaps
  now closed and the ONLY remaining lever a stronger VLM (full honest program closure). ADR 0006; does not touch
  the wire-based surface.
  **DONE — exactly the expected honest outcome.** `resolve_diagram_image_for_register` gained a final safe path:
  when a register's page carries EXACTLY ONE diagram image, use it regardless of `diagram_kind` (>1 → residual,
  never guess). +2 hermetic tests (single `unknown` image resolves; two images → residual). fmt + clippy
  `-D warnings` clean; full lib 1433 → 1435; kg-bench 151/151. **LIVE on the de-fragmented RISC-V Debug evidence
  (`--vlm-provider ollama`, `qwen2.5vl:7b`, all 44 registers): registers_recovered 0, residuals 44, ZERO
  fabrication** (evidence untouched). The KEY change vs `.4b`: several registers now RESOLVE a diagram and REACH
  the VLM (`hartinfo`/`sbcs`/`dpc`/`textra64` → `residual_non_standard_width`; `mcontrol` → `residual_name_mismatch`
  / `no_proposals`), instead of `.4b`'s uniform `residual_no_diagram` — so the guardrail is now validated
  END-TO-END through the resolver (the local VLM reaches the diagram, mis-sizes it → gate (a) rejects → honest
  residual), not only via direct probes. The registers still `residual_no_diagram` have 0 or >1 images on their
  page (resolver correctly refuses to guess). **Both `.4b` plumbing gaps are now closed (`.4c` de-fragmentation +
  `.4d` resolver); the EGF `.4` bit-recovery machinery is COMPLETE and the metric boundary is now PURELY VLM
  accuracy** — the local `qwen2.5vl:7b` is not accurate enough on these dense diagrams; a stronger VLM is the
  sole remaining lever (honest program closure). Commit subject: `EXTRACTION-GAP-FIX.4d`.

- ID: `EXTRACTION-GAP-FIX.5` · Status: `done` (`2026-09-18`, MEASUREMENT + ROOT CAUSE) · Goal:
  **`EXTRACTION-QUALITY-GAUGE.3j.3` made deterministic recall the binding constraint on extraction
  quality; find out what actually bounds it.** `.3j.3` measured that the LLM-primary path is shown 60 of
  326 obligations about declared signals — an 18.4% ceiling — because its universe is whatever the
  deterministic producer already emitted. That makes "why does the deterministic producer emit so little"
  the question with the most leverage in the programme, and nothing had asked it.
  Instrument: `constraint_recall_gap_local_measurement` (`ir/evidence.rs`, `--ignored`, read-only), which
  runs the **production** producer — `extract_normative_signal_constraints` over the statement path and
  `extract_signal_description_row_constraints` over the row path, composed exactly as
  `replay_persisted_signal_constraints` composes them — so "not emitted" means *today's code mints
  nothing*, not that an older build happened not to persist it.
  **Measured over the measured stratum (`ADR 0048`, so the numbers are publishable as current):
  379 statements state an obligation about a declared signal and the producer emits a record from 60 —
  deterministic recall 15.8%.** The 60 is the same 60 `.3j.3` found as the model's visible universe, which
  cross-checks the two instruments against each other from opposite directions.
  **The gap is not 319 losses, and reporting it as one would overstate the loss.** A statement the
  constraint surface drops may still be represented: **63** are held by the conditional-rule surface and
  **9** by actor-signal relations, leaving **247 held by no persisted surface at all**. First-match
  partition of those 247: **51 table row / 41 ordering / 47 actor-subject / 108 other**.
  **ROOT CAUSE, and it is not the grammar.** `extract_signal_constraints` opens with
  `if !matches!(statement.class, StatementClass::SignalValueConstraint) { continue; }`. The constraint
  path never sees a statement the upstream classifier labelled anything else. Of the 379 obligations,
  **86** carry that class and **195** carry `NormativeStatement`; the rest are `ConditionalRule` 65
  (which is where the 63 above are held), `TimingConstraint` 24, `DerivedRule` 5, `ExplicitAbstraction` 3,
  `SourceFact` 1. **Within what it is allowed to read the grammar converts 60 of 86 — 69.8%. Across all
  obligations it converts 60 of 379 — 15.8%. The 4.4x difference is classification, not parsing.**
  **That was established by control, after the first probe was wrong.** A fixture asserting the grammar
  could not read `XQBURST must be LOW.` was **invalid**: two sentence shapes the real corpus demonstrably
  *does* extract from came out empty in it too, which is a fixture defect and not a producer defect. With
  the class corrected the grammar reads every shape tried — level, bare enum, numeric, stability, and
  remain-asserted-until. Widening the grammar would therefore have been the wrong fix, and the control
  `the_statement_class_and_not_the_grammar_decides_what_the_constraint_path_reads` now pins both
  directions so the root cause cannot be re-derived as a grammar problem.
  **No producer change ships here, deliberately.** 41 of the unrepresented 247 are ordering obligations
  (*"must wait for AWVALID … before asserting BVALID"*) and the constraint vocabulary
  `(subject, kind, value, condition)` has no slot for them; feeding `NormativeStatement` into the
  constraint path wholesale would mint exactly the fabrications `.3a`/`.3b`/`.3d`/`.3k.1` exist to refuse,
  and this tree's honesty guardrail forbids it. Which shapes have a typed slot is `.5a`'s adjudication.
  Prerequisite: none. Blocks: `.5a`.
  Verification: the production-producer measurement, the cross-check against `.3j.3`'s 60, the
  classification control with both directions, and the workspace oracle
  Commit: `EXTRACTION-GAP-FIX.5 — deterministic recall is bounded by classification, not by grammar`

- ID: `EXTRACTION-GAP-FIX.5a` · Status: `done` (`2026-09-18`, ADJUDICATION + DESIGN) · Goal:
  **adjudicate which of the 195 `NormativeStatement` signal obligations have a typed slot.**
  **The general widening is REFUSED, on the tree's own standard.** Simulated by relabelling every
  `NormativeStatement` to the class the constraint path reads — exactly what widening the filter would do
  — and reading **every** record it would mint: **43 records from 37 statements, of which roughly 15 are
  correct**. The failures are not marginal. *"RCHUNKV must be the same for every response transfer"* mints
  `MustBeValue SAME`; *"AWSNOOP must be set to all zeros"* mints `ALL`; *"ACTIVATEREQ must be glitch free"*
  mints `GLITCH`; *"AWSNOOP\_WIDTH must be 5"* mints the subject **AWSNOOP** — the escape-fragment defect
  `.3k.9` owns, arriving through a second door. At ~35% precision this sits inside the band `.3j` refused
  at 3/7 and `.3j.2.a` refused at 4/16.
  **One shape inside it is exact, and it is specified with the guards that make it so.** A **two-cell table
  row whose first cell IS a declared signal** states that signal's obligation, and the subject is the row's
  **key** rather than a token lifted out of prose — which is precisely what the prose failures get wrong.
  Measured over the corpus: **12 such rows, of which the guards admit 10 and refuse 2, and the split is
  perfect** — all 10 admitted are correct (`| ARSNOOP | Must be 0b1110 . |`, `| ARADDR | Must be zero. |`,
  `| ARBURST | Must be INCR ( 0b01 ). |`, `| ARDOMAIN | Must be Shareable ( 0b01 or 0b10 ). |`, …) and both
  refused are the two that would have been wrong: `| ARSIZE | Must be equal to the data channel width or
  Max_Transaction_Bytes |` is an inter-signal equality with no typed slot (`.3d`'s refusal class arriving
  through a table) and `| ARCACHE | Must be Modifiable, Non-cacheable ( 0b0010 ) |` is a compound value
  whose first fragment alone would be published.
  Three guards, none subsuming another: **exactly two cells** refuses the four-cell signal-DESCRIPTION row
  (AMBA APB's `PSTRB` row, which otherwise mints `MustBeDeasserted` from a description); a **comma**
  refuses the compound; a **disjunction or leading `equal to`** refuses the equality. A trailing `( … )` is
  stripped before those tests because it ENCODES the value rather than extending it — which is why
  `Must be Shareable ( 0b01 or 0b10 )` is correctly admitted as the single value *Shareable*. The rule adds
  **no second value grammar**: it rewrites the row into the canonical sentence its cells already state and
  hands it to the same path prose uses, so the value vocabulary keeps one owner.
  **WHY IT IS NOT WIRED HERE, and this is the finding with the widest reach.** The reader was implemented
  and composed into `extract_normative_signal_constraints`, and AXI's proof-carrying artifact immediately
  stopped loading: *"registered derivation `evidence.claim.schema_version.root` output or input topology is
  stale"*. **A/B confirmed the cause** — with the composition removed the same artifact loads 6,451
  statements; with it restored the document leaves the measured stratum and the corpus census falls from 5
  documents to 4 and from 379 obligations to 144. A change to a **registered evidence derivation**
  invalidates every proof-carrying artifact until it is rebuilt, so **a producer change and a corpus
  rebuild are one transaction** — not two slices. That is the same detached-run window
  `EXTRACTION-QUALITY-GAUGE.3k.9` is parked on, and two leaves now need it.
  The rule therefore ships as a **specified, sized and verified design** rather than as dead production
  code: its predicate lives in the measurement module, where the corpus can exercise it without touching a
  derivation, and `.5b` wires it in the same transaction as the rebuild.
  Prerequisite: `.5`. Blocks: `.5b`.
  Verification: the simulated widening read record by record, the guard split measured at 10 admit / 2
  refuse, the A/B on the proof topology, and the workspace oracle
  Commit: `EXTRACTION-GAP-FIX.5a — refuse the widening, specify the one exact row rule, and find what wiring it costs`

- ID: `EXTRACTION-GAP-FIX.5b` · Status: `pending` (opened `2026-09-18` by `.5a`) · Goal:
  **wire the signal-keyed obligation row reader, in one transaction with the rebuild it forces.** The rule,
  its three guards and its expected corpus effect are frozen by `.5a`: **+10 records, 0 fabrications, 2
  refusals that are both correct**. What this leaf owns is the part `.5a` proved cannot be separated —
  composing it into `extract_normative_signal_constraints` invalidates the proof of every artifact whose
  recorded derivation topology it moves, so the slice must rebuild those artifacts and re-verify the
  stratum in the same commit, or the measured stratum silently shrinks.
  Order, and it is not negotiable: measure the stratum **before**; wire the reader with its controls and an
  observed RED; rebuild every affected chain; re-measure the stratum and prove it is **27 again, not 26**;
  then re-derive `.5`'s recall figure, which should move 60 → 70 of 379.
  **The blocker was named twice and is now GONE, which is why this paragraph keeps both readings.**
  The leaf first said it needed *a detached window*. Measured `2026-09-19`, the rebuild was not the
  problem: `specforge evidence` costs **0.5 s** and `semantic` **0.8 s** on a bundle-retaining document,
  so 24 of the 27 rebuild through both stages in about a minute, in-session. The leaf then named the real
  obstacle — AXI, APB and AHB retained **no normalized bundle** and so could not be rebuilt at all, and
  **all ten of this rule's records are in AXI** — and waited on `CORPUS-CHAIN-CURRENCY.10` for a re-ingest
  decision.
  **Re-derived `2026-09-20`, and it no longer holds.** `RETAINED-BUNDLE-POPULATION-FROZEN.3` installed and
  declared all three golds on `2026-09-19`, the day AFTER this note was written, so each of them now has
  its `normalization_plan.promoted_markdown_path` present on disk
  (`generated/source_ir/<document>/normalized/<document>.md`, verified for `ihi0022_l`, `ihi0024_e` and
  `ihi0033_c`), and `check_chain_currency.sh` reports **evidence: 27 replayed, 27 current, 0 stale** with
  retention exactly the declared set. **No re-ingest is needed and no SourceIR is rewritten**, so the
  `WIRE-BASED-100` concern this leaf recorded does not arise either. The leaf is ELIGIBLE.
  *A parked leaf keeps its blocker until someone re-derives it; this one was stale by one day.*
  Prerequisite: `.5a` (met) **and** `CORPUS-CHAIN-CURRENCY.10` (decided; its retention remedy is what
  discharged the blocker). Blocks: nothing.
  Verification: pending
  Commit: pending

### Acceptance Checklist (enforced) — `EXTRACTION-GAP-FIX.5`

- [x] **REPRODUCE / MEASURE** —
  `cargo test -p specforge-core --lib constraint_recall_gap -- --ignored --nocapture` reports
  **5 documents / 379 obligation statements / 60 produced / 319 gap / 15.8% recall**, the surface split
  **63 conditional / 9 relation / 247 unrepresented**, the partition **51 / 41 / 47 / 108**, and the
  classification split **86 SignalValueConstraint / 195 NormativeStatement / 65 ConditionalRule /
  24 TimingConstraint / 5 DerivedRule / 3 ExplicitAbstraction / 1 SourceFact**.
- [x] **ROOT CAUSE (WHY + WHERE)** — `ir/evidence.rs` `extract_signal_constraints`, first statement of
  its loop: `if !matches!(statement.class, StatementClass::SignalValueConstraint) { continue; }`. The
  bound is upstream classification, and the measurement quantifies it at 4.4x.
- [x] **ADDRESSED (verified)** — the root cause is established by a control that fails in both
  directions rather than by reading the code: five obligation shapes are read when the class admits them,
  and the identical sentence mints nothing under `NormativeStatement`, `SourceFact` or `DerivedRule`. The
  first fixture that suggested a grammar defect was **invalidated by its own controls** — two corpus
  shapes known to extract came out empty in it — and was corrected before any conclusion was recorded.
- [x] **NO REGRESSION** — `cargo test --workspace --lib --exclude specforge-production-graph`
  **2,199 passed / 14 ignored / 0 failed** (specforge-core 1,558: +2 controls, +1 ignored measurement);
  `cargo fmt --all -- --check` clean; `cargo clippy --workspace --all-targets` reports only the
  pre-existing `too_many_arguments`. No production function added or changed — the slice is one
  `--ignored` measurement and one control module — so every artifact in `generated/` is byte-identical
  and no wire-based spec can move.
- [x] **GENERICITY (ADR 0006)** — the measurement reads the RFC-2119 modal vocabulary this repository
  already uses in three production sites, the document's own catalog, and statement classes; the controls
  use opaque `XQ*` tokens. No chip, vendor or protocol name enters the runtime, and the document names in
  this record are the result of the partition rather than an input to it.
- [x] **LOCKSTEP** — no production rule changed and none was deleted, so no book text describes behaviour
  that has gone; the book gains the routing rule at `.5a` if one ships. Fact card:
  `[[deterministic-constraint-recall-is-bounded-by-classification]]`.

## Current frontier

**ALL 4 quantified gaps now WORKED (`2026-06-08`); the tree is at an honest boundary.** `.1` (I2C prose
precision 0.600→1.000), `.2` (NVMe mnemonic 0/29→28/29), `.3` (RISC-V register-name 0/60→59/60) all produced
measured metric gains. `.4` (RISC-V bit-layout-graphic) is **machinery-built + live-measured + guardrail-
validated, but its metric is honestly UNCHANGED** (bit-extent 0/179): `.4a` built the tiling-gated recovery
(hermetic 14/14 on clean widths) and `.4b` measured it live — the local `qwen2.5vl:7b` reads field names+order
correctly but makes width/reserved errors on these dense diagrams (dmcontrol sum 37, dmstatus sum 33), so the
standard-width gate rejects → honest residual, **zero fabrication**; and two upstream plumbing gaps (diagrams
classified `unknown` not `RegisterBitfield`; field tables fragmented across Docling tables) block auto-
resolution. **BOTH `.4b` plumbing gaps NOW CLOSED (`2026-06-08`):** `.4c` de-fragmentation
(`consolidate_register_field_fragments` merges a register's Docling-split field tables behind a conservative
all-field-names-distinct gate — RISC-V Debug 60→44 records, dmcontrol complete/13 fields, enables gate (b)) +
`.4d` resolver (`resolve_diagram_image_for_register` now uses the unique image on a register's page regardless of
`diagram_kind`, so register diagrams left `unknown` by the ingest classifier are found). **The EGF `.4`
bit-recovery machinery is now COMPLETE and the metric boundary is PURELY VLM accuracy.** A live run on the
de-fragmented RISC-V Debug evidence (`qwen2.5vl:7b`, all 44 registers) recovered 0 / residualed 44 with ZERO
fabrication — but several registers now REACH the VLM and get gated `residual_non_standard_width` /
`residual_name_mismatch` (vs `.4b`'s uniform `residual_no_diagram`), so the guardrail is validated end-to-end; the
local VLM simply isn't accurate enough on these dense diagrams. **The SOLE remaining lever is a stronger VLM**
(VLM-read robustness: a larger/cloud VLM, voting, image upscaling, sharper prompt) — the honest boundary holds
(recovery fabricates nothing). **REOPENED `2026-09-18` by a NEW quantified gap, which is what this tree is for.** `.5` measured
deterministic constraint recall at **15.8%** (60 of 379 obligations about declared signals) and found the
bound is **upstream classification, not the grammar**: the constraint path reads only
`StatementClass::SignalValueConstraint`, 195 of the 379 carry `NormativeStatement`, and within what it is
allowed to read the grammar converts 69.8%. `.5a` adjudicated: the general widening is REFUSED at ~35% precision, one exact shape — a two-cell table
row keyed on a declared signal — is specified with guards that split its 12 instances 10 admit / 2 refuse
perfectly, and wiring it is blocked on a rebuild because composing a reader into a registered evidence
derivation invalidates proof-carrying artifacts. `.5b` owns that transaction and is **ELIGIBLE since
`2026-09-20`**: its stated blocker — the three golds retaining no normalized bundle — was discharged by
`RETAINED-BUNDLE-POPULATION-FROZEN.3` the day after the note was written, and re-deriving it is what found
that out. The
original four gaps remain as below. **Other eligible work is in a sibling active tree** (`PDF-VARIANT-DIGESTION` frontier
`.6`/`.7` — currently blocked on host-local PDFs; or `EXTRACTION-QUALITY-GAUGE` — its `.4` constraint-dedup proven
NOT a clean win: AXI's same-`(subject,kind,value)` constraints mix conditional vs unconditional obligations whose
condition lives only in `source_text`, so content-consolidation is unsafe and the byte-identical-safe dedup is
negligible ~1-2/doc). Honesty guardrail throughout: read where the fact lives / trust only a clean read, else
honest residual.

## Decisions

- Order by tractability + agnostic-safety + measurability: I2C filter → NVMe mnemonic → RISC-V name → RISC-V
  bit-graphic. The first two are clean structural/grammar fixes against existing golds; the last two are the
  genuinely-hard reading-order / image-parse problems (`.2c` already deferred `.3`'s association for a real
  reason), so they may split and are sequenced last.
- Every fix re-measures on the SAME `.4a` gold that quantified the gap (closed-loop, no faking).

## Open questions

- `.3` register-name: **RESOLVED (`2026-06-08`)** — a reliable PAGE-BASED structural anchor exists. The `.2c`
  deferral worried about `content_elements` reading-order, but page-number proximity (`nearest_section_title_*`)
  is sufficient: a register's defining heading and its field table sit on the same/adjacent page. The register
  name is the leading identifier of a heading parenthetical that ALSO carries a hex address (`(dmstatus, at
  0x11)`), the address being the universal "this defines a register" gate. No VLM needed. Verified on pages
  32–38 of RISC-V Debug (only register-defining headings there — no intervening noise) and 0 fabricated names.
- `.4` bit-graphic: VLM transcription of the register-diagram image vs a structural bit-row parse? (resolve when picked.)

## Blockers

- None. (`.3`/`.4` are hard, not blocked — split until a real blocker is visible.)

## Verification log

- `.1` (`2026-06-08`): noun-phrase HEAD rule in `synthesize_signal_declarations_from_prose` (immediate head
  noun must be a wire noun, replacing the 4-word descriptor window). Grounded in the real UM10204 prose
  (traced all 10 parenthetical acronyms from the persisted I2C SourceIR/EvidenceIR). +1 hermetic test
  (`parenthetical_head_must_be_a_wire_noun_i2c_precision`); 2 existing parenthetical tests still pass.
  Re-ingested I2C (`DOCLING_DEVICE=cpu`, 38 s) → rebuilt evidence → `eval-extraction seed_i2c_signals.json`:
  declared-signal complete-gold precision **0.600 → 1.000** (fp 4 → 0), recall **1.000** held. fmt +
  clippy `-D warnings` clean; full lib suite 1378 → 1379; kg-bench 151/151. No wire-based regression
  (parenthetical path disabled for ≥8-table-signal specs).
- `.2` (`2026-06-08`): **First established WHERE the mnemonic lives** (honesty guardrail) — inspected the
  persisted NVMe SourceIR (`structured_tables` 0035–0038, the CAP/CC/CSTS register tables) and proved the
  mnemonic is the parenthesized abbreviation in the defined-term prefix `Full Field Name (MNEMONIC): …`, NOT
  the leading-bare `MQES:` token the `.2` goal hypothesized. Implemented `field_mnemonic_from_description`
  (first `(<uppercase-alnum>):` group; the colon required to reject passing refs like `(CC.MPS)`),
  `is_bit_range_token`, `is_field_mnemonic_token` in `ir/evidence.rs`; wired into BOTH
  `synthesize_register_field_tables` and `synthesize_register_records` behind a gate (no explicit name/field
  column AND the captured name is a bit-range). Pure grammar (ADR 0006); no chip names; no denylist; no
  leading-bare path (no in-corpus gold needs it, FP-risk). +2 hermetic tests
  (`register_field_bits_only_recovers_mnemonic_from_description` reproducing the real CAP rows;
  `field_mnemonic_recovery_is_grammar_only` for the edge cases incl. non-fabrication). **Re-measured on the
  `.4a.3` NVMe gold from the SAME persisted SourceIR, stash-isolated baseline vs fixed:** field-name recall
  **0/29 → 28/29 = 0.966** (lone miss `CAP.CRMS` — no clean `(CRMS):` term in the source → verified-absent
  residual, not fabricated), bit-structure recall held **27/29 → 28/29**. The persisted SourceIR reproduced
  the EXACT gold baseline (0/29, 27/29 bits), so it is a faithful basis and re-ingest is unnecessary for this
  description-parse metric (the prose is deterministic on the same PDF). fmt + clippy `-D warnings` clean; full
  lib suite 1379 → 1381; kg-bench 151/151. No wire-based regression (the path only fires on bits-only
  register-field tables; APB/AHB/AXI/SWD have none).
- `.3` (`2026-06-08`): **Resolved the open question by INVESTIGATION first** — probed the persisted RISC-V Debug
  SourceIR: register-field tables have empty captions; the register name lives in the section heading
  *"3.14.1. Debug Module Status (dmstatus, at 0x11)"*; pages 32–38 carry only register-defining headings (no
  intervening noise) so page-based nearest-preceding-section association is reliable. Implemented
  `register_name_from_heading` (leading identifier of a parenthetical that also carries a `0x` address via
  `contains_hex_address`; `is_register_name_token` shape gate), `nearest_section_title_original` (page-based,
  original case), wired as a fallback after the caption path in `synthesize_register_field_tables`. Address-gated
  so a prose parenthetical can't mint a name; ADR 0006 (no RISC-V phrasing). +2 hermetic tests
  (`register_name_from_heading_is_grammar_only` incl. no-address/prose negatives;
  `register_name_recovered_from_defining_section_heading` incl. a synthetic-name-kept negative). **Re-measured on
  the `.4a.2` RISC-V gold from the SAME persisted source (stash-isolated): register-name association 0/60 →
  59/60**, `dmstatus`/`dmcontrol` correct, **0 fabricated names** (python cross-check: every recovered name
  traces to a defining heading with a `0x` address), field-name recall unchanged 20/34. Strict per-fact recall
  stays 0.000 — bits are 0/179, in the layout graphic (`.4`). fmt + clippy `-D warnings` clean; full lib suite
  1381 → 1383; kg-bench 151/151. No wire-based regression.
- `.4a` (`2026-06-08`): Built the validated `.4` design as TWO pieces — a pure gated core and its production
  command. Pure core `ir/register_bits.rs`: `reconstruct_bits_by_tiling` lays MSB→LSB `(name,width)` proposals
  down from an exclusive top counter so each field claims `[top-width, top-1]` (no underflow; the standard-width
  gate guarantees the last lands on bit 0) and returns `None` unless the widths sum to 8/16/32/64/128 — gate
  (a); `proposal_names_match_fields` is an exact multiset comparison against the register's own field table —
  gate (b); `recover_bits_for_register` attaches bits only when both gates pass and every field is bits-missing,
  else a typed residual. Command `commands/recover_register_bits.rs` resolves the register's `RegisterBitfield`
  diagram on its page in the sibling SourceIR, reads `(name,width)` MSB→LSB via the shared image transport + the
  `SPECFORGE_VLM_HELPER` hook, runs the core, writes back; default `--vlm-provider skip` no-op. Wired in
  `cli.rs`/`lib.rs`/`commands/mod.rs`/`ir/mod.rs`. **Hermetic tests use the real `seed_riscv_debug_registers.json`
  data:** dmcontrol 14 fields → 14/14 exact bits (`hartsello [25:16]`, `hartselhi [15:6]`); dmstatus
  reserved-gap + width-7-misread → honest residual (0 bits); name-mismatch with tiling-valid widths → residual;
  + parser/string-width/garbage, helper-hook proposer, end-to-end command, and skip-noop tests (17 new). ADR
  0006 clean (only universal widths + the tiling law in the runtime; chip names only in test data). fmt + clippy
  `-D warnings` clean (pure core production-reachable → no dead-code); full lib suite 1383 → **1400**; kg-bench
  151/151; no wire-based regression. Live bit-extent measurement deferred to `.4b`. KM card
  `register-diagram-bit-recovery-via-tiling`.
- `.4b` (`2026-06-08`): **Live measurement, honest result — metric UNCHANGED (0/179), zero fabrication.** End-to-end
  live run (`recover-register-bits … --vlm-provider ollama`, `qwen2.5vl:7b`) → 60/60 register records
  `residual_no_diagram`, evidence untouched (no diagram resolves: diagrams classified `unknown` not
  `RegisterBitfield`; field tables fragmented across Docling tables so no record holds a register's full field
  set). Direct VLM probes of `picture-0020.png`/`table-0020.png` (temp 0) → the model reads names+order
  correctly but adds a spurious width-5 `reserved` (dmcontrol sum 37) / off-by-one (dmstatus sum 33) → gate (a)
  rejects every erroneous read → honest residual. The guardrail is fully validated on real data (every real VLM
  error caught, nothing fabricated); the tiling math is proven by `.4a`'s hermetic 14/14. Closing the metric is
  the proposed follow-up `.4c` (classify register bitfield diagrams + de-fragment field tables + VLM-read
  robustness / stronger VLM). Docs-only (no code change).
- `.4c` (`2026-06-08`): **de-fragmentation half of the `.4b` plumbing built + verified.** Pure
  `consolidate_register_field_fragments` (groups register records by recovered name; merges a group only when
  `register_fragments_are_safe_to_merge` = all field names across the group are distinct case-insensitively — the
  two safety gates collapse to that one test; `merge_register_fragments` unions fields in fragment order + unions
  supporting ids + recomputes width from the full set, emitted at the first fragment's position for deterministic
  order), wired into the base `evidence` build after register synthesis. +6 hermetic tests on real RISC-V shapes.
  fmt + clippy `-D warnings` clean; full lib 1427 → 1433; kg-bench 151/151. **Live (rebuilt on-disk evidence):
  RISC-V Debug 60 → 44 records** (dmcontrol 3→1 with all 13 fields; dcsr/mcontrol6 merged; the hazards
  custom0/mcontrol/sbaddress3/icount/textra64 honestly NOT merged), **NVMe 44 → 42**. **Golds held:** RISC-V
  field-name recall 20/34=0.588 (register-agnostic → pooling-invariant), register-name 43/44 (~0.98, was 59/60);
  NVMe field-name 28/29, bit-structure 28/29; both unchanged. `.5b` gauge cleaner (dmcontrol once, complete). Also
  enables `recover-register-bits` gate (b) for merged registers. Wire-based unaffected (no register-field tables;
  consolidation can't touch constraints/relations/temporal). Commit subject: `EXTRACTION-GAP-FIX.4c`.
- `.4d` (`2026-06-08`): **plumbing gap #1 closed — resolver finds register diagrams classified `unknown`.**
  `resolve_diagram_image_for_register` gained a final safe path: when the register's page carries exactly ONE
  diagram image, use it regardless of `diagram_kind` (>1 → residual, never guess); the downstream tiling +
  name-match gates still reject any wrong read, so it never fabricates. +2 hermetic tests (single `unknown` image
  resolves; two images → residual). fmt + clippy `-D warnings` clean; full lib 1433 → 1435; kg-bench 151/151.
  **Live (RISC-V Debug, `qwen2.5vl:7b`, all 44 registers): recovered 0 / residuals 44, ZERO fabrication.** vs
  `.4b`, several registers now reach the VLM and are gated `residual_non_standard_width`
  (`hartinfo`/`sbcs`/`dpc`/`textra64`) or `residual_name_mismatch`/`no_proposals` (`mcontrol`) — guardrail
  validated END-TO-END. Both plumbing gaps now closed (`.4c`+`.4d`); the EGF `.4` machinery is COMPLETE and the
  metric boundary is purely VLM accuracy. Commit subject: `EXTRACTION-GAP-FIX.4d`.

## Commit log

- `.1` → `EXTRACTION-GAP-FIX.1 — I2C prose precision: noun-phrase head must be a wire noun (0.600 → 1.000)`
- `.2` → `EXTRACTION-GAP-FIX.2 — NVMe mnemonic from description defined-term (field-name recall 0/29 → 28/29)`
- `.3` → `EXTRACTION-GAP-FIX.3 — RISC-V register name from defining heading (register-name 0/60 → 59/60)`
- `.4a` → `EXTRACTION-GAP-FIX.4a — tiling-gated register-diagram bit recovery (pure core + recover-register-bits)`
- `.4b` → `EXTRACTION-GAP-FIX.4b — live measurement: guardrail validated, metric unchanged 0/179, zero fabrication`
- `.4c` → `EXTRACTION-GAP-FIX.4c — de-fragment split register-field tables (RISC-V 60→44 records; golds hold)`
- `.4d` → `EXTRACTION-GAP-FIX.4d — resolver finds register diagrams classified unknown; machinery complete (metric VLM-gated)`

## Changelog

- `2026-06-08`: Created (owner pivot — fix the `PDF-VARIANT-DIGESTION.4`-quantified extraction gaps) + owner
  honesty guardrail baked in (cannot extract what is not there → read the right modality, else residual).
  `.1` (I2C prose precision filter) DONE → precision 0.600 → 1.000.
- `2026-06-08`: `.2` (NVMe mnemonic-from-description) DONE → field-name recall 0/29 → 28/29 = 0.966. The real
  structure was the parenthetical defined-term `(MQES):`, not the hypothesized leading-bare form; recovered it
  in both register paths, honest residual for the one term-less field.
- `2026-06-08`: `.3` (RISC-V register-name-from-heading) DONE → register-name association 0/60 → 59/60, 0 false
  names. Open question resolved by investigation: page-based association to the defining heading's `(name, at
  0x..)` parenthetical, gated by the hex address (no fabrication). Frontier moves to `.4` (RISC-V bit-graphic,
  the last + hardest gap).
- `2026-06-08`: `.4a` (tiling-gated register-diagram bit recovery) DONE → built the validated `.4` design. Pure
  core `ir/register_bits.rs` (cumulative LSB tiling + two anti-fabrication gates: standard-width tiling +
  exact field-name multiset match) and command `recover-register-bits` (EvidenceIR-in/out, VLM reads
  `(name,width)` off the `RegisterBitfield` diagram via the shared transport + `SPECFORGE_VLM_HELPER` hook,
  skip-default no-op). Hermetic tests on real dmcontrol (14/14 exact) + dmstatus (honest residual) +
  name-mismatch (residual). lib 1383 → 1400; kg-bench 151/151; no wire-based regression; ADR 0006 clean.
  Frontier moves to `.4b` (live measurement of bit-extent on the `.4a.2` gold; needs `ollama serve`).
- `2026-06-08`: `.4b` (live measurement) DONE → **honest result: metric UNCHANGED (bit-extent 0/179), zero
  fabrication.** The end-to-end live run residualed all 60 registers (diagrams classified `unknown`; field
  tables fragmented), and direct VLM probes showed the local `qwen2.5vl:7b` reads names+order correctly but adds
  spurious reserved cells / off-by-one widths (dmcontrol sum 37, dmstatus sum 33) → the standard-width gate
  rejects → honest residual. The guardrail is fully validated on real data; closing the metric is the proposed
  follow-up `.4c` (diagram classification + field-table de-fragmentation + VLM-read robustness / stronger VLM).
  All 4 originally-quantified gaps are now worked (`.1`/`.2`/`.3` with measured gains; `.4` machinery + measure
  + honest boundary). NEXT eligible PNT work is in a sibling active tree (`PDF-VARIANT-DIGESTION.5a` or
  `EXTRACTION-QUALITY-GAUGE`).
- `2026-06-08`: `.4c` (de-fragment split register-field tables — the first `.4b` plumbing gap) DONE.
  `consolidate_register_field_fragments` merges a register's Docling-split field tables into one record behind a
  conservative all-field-names-distinct safety gate (garbled/array-collapsed groups stay split — honesty
  guardrail). Live: RISC-V Debug 60 → 44 records (dmcontrol complete, 13 fields), NVMe 44 → 42; `.4a.2`/`.4a.3`
  golds unchanged; `.5b` gauge cleaner; lib 1433; kg-bench 151/151.
- `2026-06-08`: `.4d` (resolver finds register diagrams classified `unknown` — the SECOND `.4b` plumbing gap)
  DONE. `resolve_diagram_image_for_register` now uses the unique image on a register's page regardless of
  `diagram_kind` (>1 → residual). Live RISC-V Debug run (`qwen2.5vl:7b`, 44 registers): recovered 0 / residuals
  44, zero fabrication, but several registers now reach the VLM and gate honestly
  (`residual_non_standard_width`/`residual_name_mismatch`) vs `.4b`'s uniform `residual_no_diagram` — guardrail
  validated end-to-end. **BOTH plumbing gaps closed; the EGF `.4` bit-recovery machinery is COMPLETE and the
  metric boundary is purely VLM accuracy — a stronger VLM is the sole remaining lever.** lib 1435; kg-bench
  151/151.
