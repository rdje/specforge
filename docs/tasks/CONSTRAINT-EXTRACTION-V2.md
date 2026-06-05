# CONSTRAINT-EXTRACTION-V2: fix the 3 constraint bug classes REEXTRACTION-REMEASURE found

## Metadata

- Tree ID: `CONSTRAINT-EXTRACTION-V2`
- Status: `done` (CLOSED `2026-06-05`; `.1`–`.3`)
- Roadmap lane: `R16`/`R15e` (extraction quality)
- Created: `2026-06-05`
- Parent context: `REEXTRACTION-REMEASURE` re-extracted the real APB spec (constraints 42→16) and the
  11 still-NLI-flagged claims revealed 3 concrete, current bug classes. This tree fixes them, in the
  deterministic pattern extractor. All fixes are agnostic (positional / centralized-vocabulary), no
  chip-spec names (ADR 0006).

## The 3 bug classes

1. **Value-as-subject in non-`"must be"` constructions.** `"PSLVERR is driven LOW when …"` and
   `"… is tied LOW"` yield a bogus `"LOW must be stable"` — the logic-level value `LOW` is collected
   as the subject. (Positional exclusion only fires after `"must be"`.) **Fix:** a logic-level word
   (the centralized, owner-confirmed `normative_vocab::LOGIC_*_VALUES` — universal "how") is never a
   constraint subject.
2. **Table-cell claim text not split per obligation.** A multi-bullet cell
   (`PAUSER must be valid when … • … same value … • …`) becomes one malformed claim carrying the
   whole cell. **Fix (planned):** split table-cell text on bullet/sentence boundaries before
   extraction so each obligation is its own claim.
3. **`"which means"` condition mis-attribution.** `"PSEL is asserted, which means PADDR/PWRITE/PWDATA
   must be valid"` yields `"PSEL must be VALID"` (PSEL is the trigger). **Fix (planned):** treat
   `"which means"` / `", which …"` like the existing `when`-clause antecedent handling.

## Task Tree

- ID: `CONSTRAINT-EXTRACTION-V2` · Status: `done` (CLOSED `2026-06-05`) · Children: `.1` `.2` `.3`
- ID: `CONSTRAINT-EXTRACTION-V2.1` · Status: `done` · Goal: logic-level value never a subject.
  Done (`2026-06-05`): `is_logic_level_token` (references the centralized, owner-confirmed
  `normative_vocab::LOGIC_*_VALUES`) added to `collect_subject_signal_tokens`' filter — a logic-level
  word is never collected as a subject, closing the `"tied/driven LOW"` → `"LOW must be stable"`
  leak. +1 test (real APB construction). CI green 1266. Agnostic (centralized universal "how", no
  chip-spec names).
- ID: `CONSTRAINT-EXTRACTION-V2.2` · Status: `done` · Goal: split multi-obligation table cells.
  Done (`2026-06-05`): `constraint_bearing_sentence` now also splits on `•`/newlines (so a
  multi-bullet cell yields one obligation per clause, not one constraint carrying the whole cell),
  and `condition_text` is extracted from that same bounded obligation — so a later bullet no longer
  bleeds into a constraint's condition (the malformed `"PAUSER must be VALID … • PAUSER must have
  …"` claims). +1 test (real APB cell → condition has no bullet/bleed). CI green 1268.
- ID: `CONSTRAINT-EXTRACTION-V2.3` · Status: `done` · Goal: `"which means"` condition handling.
  Done (`2026-06-05`): `consequent_after_inference_marker` — for `"<antecedent>, which means
  <obligation>"` (and `which implies`/`meaning that`) the subject is extracted from the **consequent**,
  so the antecedent's trigger signal is not mis-attributed the obligation. Wired into
  `extract_signal_constraints`' `subject_part`. +1 test (real APB `"PSEL is asserted, which means
  PADDR/PWRITE/PWDATA must be valid"` → subject is the consequent signals, not PSEL). CI green 1267.

## Changelog

- `2026-06-05`: Created — fixes the 3 bug classes `REEXTRACTION-REMEASURE` surfaced on real APB data.
- `2026-06-05`: **CLOSED** — all 3 fixed, agnostic (centralized vocab / positional), +3 tests, CI
  green 1268. `.1` logic-level word never a subject; `.2` multi-bullet cells bounded per obligation;
  `.3` `"which means"` antecedent excluded from the subject.
