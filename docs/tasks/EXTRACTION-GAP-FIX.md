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
| I2C prose over-capture | declared-signal precision 6/10 = 0.600 (ACK/NACK/DDC/SDR are not bus signals) | `.4a.5` |
| NVMe mnemonic-from-description | field-name recall 0/29 (the mnemonic lives in the DESCRIPTION; `field_name` is the bit-range) | `.4a.3` |
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
- ID: `EXTRACTION-GAP-FIX.1` · Status: `pending` · Goal: **I2C prose acronym/condition precision filter** —
  tighten the prose signal capture (`synthesize_signal_declarations_from_prose`, `.3a`) so it stops emitting
  entities that prose introduces as something OTHER than a bus wire: acknowledge/condition states (ACK/NACK),
  acronyms naming a DIFFERENT bus/interface (DDC), and rate/mode acronyms (SDR). General grammar only — the
  expansion must denote a wire (line/signal/clock/data/pin/bus), not a condition/rate/state — NEVER a denylist
  of those specific tokens. Re-measure on the `.4a.5` I2C gold (precision 0.600 → higher; recall stays 1.000).
  Accept: I2C precision up, recall held, APB/AHB/AXI declared-signal + kg-bench unaffected; hermetic tests.
- ID: `EXTRACTION-GAP-FIX.2` · Status: `pending` · Goal: **NVMe mnemonic-from-description** — when a
  register-field table's name column is itself a bit-range (so `field_name` is e.g. `07:04`) and the
  description leads with a mnemonic token (`MQES: …` / `MQES –  …`), recover the mnemonic as the field
  identity (structural: leading uppercase/identifier token before a `:`/`–`/`-` separator; agnostic). Re-measure
  on the `.4a.3` NVMe gold (field-name recall 0/29 → higher). Accept: NVMe field-name recall up, bit-structure
  recall held, no wire-based regression; hermetic tests.
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

**ACTIVE FRONTIER (`2026-06-08`): `EXTRACTION-GAP-FIX.1`** — the I2C prose acronym/condition precision filter
(the most self-contained, lowest-agnostic-risk, gold-measurable first win; `.4a.5` named it). Then `.2`
(NVMe mnemonic, structural), `.3` (RISC-V register-name, harder), `.4` (RISC-V bit-graphic, hardest/VLM).

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

- (pending — first entry at `.1`.)

## Commit log

- (pending.)

## Changelog

- `2026-06-08`: Created (owner pivot — fix the `PDF-VARIANT-DIGESTION.4`-quantified extraction gaps). `.1`
  (I2C prose precision filter) on the frontier.
