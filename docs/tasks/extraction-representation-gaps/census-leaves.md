# EXTRACTION-REPRESENTATION-GAPS — PDF-to-IR census leaves

- Part ID: `census-leaves`
- State: `active`

New records of the `.3` family (the PDF-to-IR census and the leaves it found) continue here, after
`program-and-census` reached its size warning (`.3.f`, `2026-09-28`).

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.i`
  Status: `done` (`2026-09-29`; CODE; found by `.3.f`; checklist below)
  Goal: **a timing limit keeps the row condition it is stated under.** Timing tables head a `Conditions` column (16
  tables in 4 current chains: I2C 6, both OpenCAPI PHY signalling specs 4 each, I2S 2) and the reader ignores it, so
  one parameter gives several records with different limits and nothing saying which applies when: in I2C 8
  (table, parameter, condition) keys hold 16 records, e.g. `I OL` at `V OL = 0.4 V` (min 3) and `V OL = 0.6 V` (min
  6), `t HD;DAT` for *CBUS compatible controllers* (5.0) and *I2C-bus devices* (0), `t rCL` by load. Some rows are
  condition headings with no value (`V OL`: *(open-drain) at 3 mA sink current*, refined by the next rows). Acceptance:
  read a column the table heads `Condition(s)` into the record, carried; census every chain; the same temporal-rule
  exclusion; I2C's 8 keys resolved.
  **Done.** `TimingConstraintRecord.row_conditions` (a list, omitted when empty) carries the row's cell verbatim from
  a column a header names `Condition`, `Conditions`, `Test condition` or `Test conditions` and nothing else
  (`timing_condition_column`, shared by every layout). A heading row (a condition, no value in any layout, the same
  parameter name as the next row) is inherited, first, by the following rows of that name (`timing_row_conditions`):
  I2C *Table 12*'s `V OL`, `t rCL`, `t fCL`. `is_temporal_rule_input` now also requires no row condition.
  **Measured:** exactly the four chains with such a column move, only in `timing_constraints` and with no record added
  or removed: 93 records gain a condition (I2C 49, I2S 11, OpenCAPI 25G signalling 17, OpenCAPI 4.0 32G PHY 16), every
  one the verbatim cell, read against its row. I2C's ambiguous (table, parameter, mode) keys go 8 -> 0. The column holds
  what the document put there: conditions, note references (OpenCAPI `Note 1`), a remark (I2C `t HD;STA`, *After this
  period, the first clock pulse is generated.*) and I2S derivations (`min > 0.35T = 140 (at typical data rate)`), so
  the carrier promises the cell, not a parsed condition. Consequence: I2S's info finding
  `semantic_temporal_rule_surface_missing` cites 1 input instead of 8, its 11 limits being conditioned.
  Commit: `EXTRACTION-REPRESENTATION-GAPS.3.f.i — a timing limit keeps the row condition it is stated under`

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.ii`
  Status: `open` (`2026-09-28`; DESIGN + CODE; found by `.3.f`)
  Goal: **a single-condition timing table keeps its condition.** I2C's Hs-mode *Table 12* and Ultra Fast-mode *Tables
  14, 15* head their one `Min | Max` pair with the mode, exactly as USB 3.2 heads a pair with `Limits`, so `.3.f` reads
  all four as scalar: 37 I2C records carry no condition and read as unconditional beside Table 11's per-mode ones.
  Decide from structure whether one spanning label is a condition (a candidate: the caption restates it, *… for
  Hs-mode I2C-bus devices*), and measure.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.f.iii`
  Status: `open` (`2026-09-28`; DESIGN + CODE; found by `.3.f`)
  Goal: **a grouped timing table that names no parameter column.** USB 3.2 gives per-condition limits with rows as
  parameters in *Table 6-20* (`5GT/s | 10GT/s`), *6-31* (`U1 Exit | U2/Loopback Exit | U3 Wakeup`) and *6-33*
  (`Transmit | Receive`), but its *Table 6-30* fragments put the parameters in the groups (`tBurst | tRepeat`) and
  the conditions in the rows (`Polling.LFPS`, `Ping.LFPS`). None heads its row-label column `Symbol`, `Parameter` or
  `Name`, so `.3.f` reads none (a residual each). Decide from the table's structure which dimension is the parameter,
  or state why it cannot be, and measure.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.f.i` (CODE) — DONE `2026-09-29`

- [x] **REPRODUCE / MEASURE** — I2C after `.3.f`: 8 (table, parameter, mode) keys hold two records each with different
  limits (`I OL` Fast-mode 3 and 6 mA; `t HD;DAT` Standard-mode 5.0 and 0 µs; Table 12's `V OL`, `t rCL`, `t fCL`,
  `t fDA`; Table 13's `C b`), 16 timing tables in 4 current chains head a `Condition(s)` column.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/source.rs` `TimingTableColumns` had no condition column
  and `TimingConstraintRecord` no carrier, so `synthesize_timing_constraints` (`ir/evidence.rs`) dropped the cell. RED,
  each alone with the file restored byte-identically: without the heading inheritance
  `a_timing_limit_keeps_the_row_condition_it_is_stated_under` fails; without the predicate's row clause
  `a_per_condition_limit_is_not_a_temporal_rule_input`, that test and
  `a_per_condition_timing_limit_is_carried_but_derives_no_temporal_rule` fail; accepting any header word starting
  `condition` fails `a_timing_table_names_its_condition_column_by_its_header_alone`.
- [x] **ADDRESSED (verified)** — I2C, I2S and both OpenCAPI PHY signalling chains rebuilt stage by stage, each stage
  validated, the adapter last: 93 records gain their condition, I2C's ambiguous keys 8 -> 0.
- [x] **NO REGRESSION** — EvidenceIR dry-run of all 31 current chains against their persisted artifacts (current at
  HEAD): 27 identical, the 4 differ only in `timing_constraints`, no record added, removed or otherwise changed.
  Rebuilt, their SemanticIR and IntentIR differ only there and their adapters only in proof surfaces;
  `check_chain_currency.sh` 31 of 31 current at every stage after the rebuilds. WIRE-BASED-100: all 11 seeds identical to HEAD's (temporary paths masked).
  `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings`
  clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 170 / 1,773 passed, 0 failed.
  Every stage's production digest unchanged. `flow_census.json` rolled forward under this leaf (+2 functions, +10
  decision sites, +36 helper edges, +1 macro; boundary unchanged).
- [x] **GENERICITY (ADR 0006)** — a closed header vocabulary and row structure (same name, no value); no document,
  parameter or condition text.
- [x] **LOCKSTEP** — book `pipeline/evidence-rule-chronology.md` (the timing-table section gains the row condition,
  with I2C's Table 12 heading rows as the example), `pipeline/semanticir.md` and `pipeline/intentir.md`; KM
  `timing-table-structural-authority` amended. Nothing described the dropped column.
