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
| RISC-V register-name | register-name association 0/60 (synthetic `register_<table_id>` — name is in the preceding heading) | `.4a.2` |
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

- ID: `EXTRACTION-GAP-FIX` · Status: `active` · Children: `.1`–`.4`
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
- ID: `EXTRACTION-GAP-FIX.3` · Status: `pending` · Goal: **RISC-V register-name-from-heading** — associate a
  register-field table's register name with the nearest preceding section heading / register-name caption
  instead of the synthetic `register_<table_id>` (the `.2c`-deferred association; needs a reliable
  reading-order or page/section anchor, structural + agnostic — do NOT fake). Re-measure on the `.4a.2` RISC-V
  gold (register-name association 0/60 → higher). Accept: association up with no false names; no wire-based
  regression; hermetic tests. (Harder — `.2c` deferred it for a real reading-order reason; may split.)
- ID: `EXTRACTION-GAP-FIX.4` · Status: `pending` · Goal: **RISC-V bit-layout-graphic parse** — recover bit
  positions for register fields whose bits live in the layout GRAPHIC, not the field table (the `.4b` audit's
  dominant flag). Likely a VLM/structural read of the register-diagram image, bounded/targeted (the `.2b`
  scaling finding), gated + agnostic. Re-measure on the `.4a.2` RISC-V gold (bit-extent 0/179 → higher).
  Accept: bit-extent up with verified-correct positions (no fabrication); no wire-based regression. **If the
  graphic read cannot recover a field's bits, that field's bit-extent stays an honest completeness residual —
  bits are NEVER synthesized from a table that does not contain them** (the honesty guardrail above). (Hardest —
  may split; the VLM is a targeted/sampled tool.)

## Current frontier

**ACTIVE FRONTIER (`2026-06-08`): `EXTRACTION-GAP-FIX.3`** — RISC-V register-name-from-heading (associate a
register-field table's register name with the nearest preceding section heading / register-name caption instead
of the synthetic `register_<table_id>`; the `.2c`-deferred association — needs a reliable reading-order or
page/section anchor, structural + agnostic, do NOT fake). **`.1` DONE** — I2C prose precision 0.600 → 1.000 via
the noun-phrase HEAD rule. **`.2` DONE** — NVMe mnemonic-from-description (parenthetical defined-term `(MQES):`
recovery, the real structure, not the hypothesized leading-bare form), field-name recall 0/29 → 28/29 = 0.966.
Then `.4` (RISC-V bit-graphic, hardest/VLM-or-residual). Remember the honesty guardrail: read where the fact
lives, else honest residual. (`.3` is harder — `.2c` deferred it for a real reading-order reason; may split.)

## Decisions

- Order by tractability + agnostic-safety + measurability: I2C filter → NVMe mnemonic → RISC-V name → RISC-V
  bit-graphic. The first two are clean structural/grammar fixes against existing golds; the last two are the
  genuinely-hard reading-order / image-parse problems (`.2c` already deferred `.3`'s association for a real
  reason), so they may split and are sequenced last.
- Every fix re-measures on the SAME `.4a` gold that quantified the gap (closed-loop, no faking).

## Open questions

- `.3` register-name: is a reliable structural heading→table association available (page anchor / caption /
  reading-order), or does it need the VLM? (resolve when picked.)
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

## Commit log

- `.1` → `EXTRACTION-GAP-FIX.1 — I2C prose precision: noun-phrase head must be a wire noun (0.600 → 1.000)`
- `.2` → `EXTRACTION-GAP-FIX.2 — NVMe mnemonic from description defined-term (field-name recall 0/29 → 28/29)`

## Changelog

- `2026-06-08`: Created (owner pivot — fix the `PDF-VARIANT-DIGESTION.4`-quantified extraction gaps) + owner
  honesty guardrail baked in (cannot extract what is not there → read the right modality, else residual).
  `.1` (I2C prose precision filter) DONE → precision 0.600 → 1.000.
- `2026-06-08`: `.2` (NVMe mnemonic-from-description) DONE → field-name recall 0/29 → 28/29 = 0.966. The real
  structure was the parenthetical defined-term `(MQES):`, not the hypothesized leading-bare form; recovered it
  in both register paths, honest residual for the one term-less field. Frontier moves to `.3` (RISC-V
  register-name-from-heading).
