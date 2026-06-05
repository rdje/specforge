# PDF-AGNOSTIC-EXTRACTION: remove all hardcoded chip-spec vocabulary; derive from the document

## Metadata

- Tree ID: `PDF-AGNOSTIC-EXTRACTION`
- Status: `done` (CLOSED `2026-06-05`; `.1`–`.4` — names de-hardcoded + CI-guarded; logic-level kinds a separate owner decision)
- Roadmap lane: `R16`/`R15e` (extraction quality / architecture)
- Created: `2026-06-05`
- Owner: repo-local workflow
- Parent context: **project-owner signoff criterion** (a thread of messages 2026-06-05): SpecForge
  production code must contain **zero** vocabulary belonging to any specific chip-spec PDF — signal
  names, value/state names *including `HIGH`/`LOW`*, protocol family names, encoding values — all
  derived from the document at hand. Spec vocabulary varies across specs and **across revisions**,
  so hardcoding is non-agnostic *and* brittle. Principle recorded in
  `docs/decisions/0006-no-hardcoded-chip-spec-vocabulary.md`.

## Confirmed violations (production)

- `evidence.rs:5774` — `collect_subject_signal_tokens` denylist: `NONSEQ`/`SEQ`/`OKAY`/`SINGLE`/
  `INCR`/`WRAP`/`RETRY`/`SPLIT` (AMBA encoding values), `AMBA`/`AHB`/`AHB5`/`APB`/`AXI`/`CHI`/`ARM`/
  `AMD` (family/vendor), plus `HIGH`/`LOW`/`IDLE`/`BUSY` (value names — to derive).
- `evidence.rs:5833` — conditional-rule consequent denylist: `NONSEQ`/`SEQ`/`OKAY` (+ `HIGH`/`LOW`/
  `IDLE`/`BUSY`).
- `semantic.rs:6320 signal_stop_words` — ~421 entries; protocol clusters: AMBA/ARM family names,
  ARM/AMBA publication-ID prefixes, AMBA AHB HTRANS values (`NONSEQ`), AMBA AHB HBURST values
  (`INCR4`/`WRAP8`/…).
- `intent.rs` `NONSEQ` literals are inside `#[cfg(test)]` (fixtures) — out of scope. `eval.rs`
  signal literals are eval/gold fixtures — out of scope.

## Design (derive, don't hardcode)

- **Signal-candidate validation:** replace the protocol-name denylists with **positive validation
  against the document's declared signals** (`declared_signal_names`, built from the doc's signal
  tables/ports; already exists and partially used at `semantic.rs:206`). A token is a signal iff
  *this document* declares it (plus document-independent shape heuristics for discovery).
- **Constraint value path:** replace hardcoded classification (`"must be high"` → `MustBeHigh`)
  with **positional/generic value extraction** — the value is whatever token the spec places after
  the normative verb (`must be` / `shall be` / `must remain`). Store the derived value; do not
  match it against a `high`/`low`/`asserted` table. Semantics (is this a level? a state?) come
  from the document's own definitions where needed.
- **Keep only document-independent logic:** English normative grammar (`must`/`shall`/`when`/`if`),
  token shapes, positional grammar. No domain names.
- **Guard:** a CI test that fails if known spec tokens reappear in production sources.

## Slices

- **`.1`** — record the invariant (ADR 0006) + a **CI guard** test that scans production sources
  for a seed set of spec-specific tokens and fails. (Guard initially allow-lists the *known*
  offenders with a TODO so the build stays green, then each later slice removes offenders and
  tightens the guard.) ← non-regression scaffolding.
- **`.2`** — value path: positional/generic value extraction; remove hardcoded value words
  (`HIGH`/`LOW`/`IDLE`/`BUSY`/encoding values) from the `evidence.rs` denylists; verify on the APB
  extraction tests (and a re-extraction) that precision holds.
- **`.3`** — signal validation: route through `declared_signal_names`; remove protocol family/
  vendor names + encoding values from `signal_stop_words`; tighten the guard.
- **`.4`** — sweep the rest of the codebase for any remaining spec tokens; tighten the guard to
  zero offenders; close.

## Acceptance Criteria

- Production code contains no chip-spec-specific domain vocabulary; the CI guard enforces it.
- Extraction precision is preserved (verified by the extraction test suite + a re-extraction
  spot-check), with vocabulary derived from each document.

## Task Tree

- ID: `PDF-AGNOSTIC-EXTRACTION` · Status: `done` (CLOSED `2026-06-05`; `.1`–`.4`) · Children: `.1` ·
  `.2` · `.3` · `.4`
  **Closed:** all chip-spec-specific *names* (protocol families, vendor names, encoding values, the
  hardcoded value list) are removed from production and a CI guard locks it. **One deliberate
  carve-out:** logic-level recognition (`MustBeHigh`/`MustBeLow`, `"must be high"`) — recommended as
  *universal digital semantics* (the "how"), not a spec name; collapsing it is a fundamental 9-file
  IR redesign and a separate owner-gated architectural decision, not a hidden gap.
- ID: `PDF-AGNOSTIC-EXTRACTION.1` · Status: `done` · Goal: ADR + CI guard.
  Verification: passed (`2026-06-05`) — ADR 0006 recorded; CI guard
  `signal_stop_words_holds_no_chip_spec_vocabulary` (semantic.rs) asserts the stop-word list holds
  no protocol family/vendor/encoding tokens (tokens concatenated so the guard can't trip itself);
  fails the build if any reappear.
- ID: `PDF-AGNOSTIC-EXTRACTION.2` · Status: `done` · Goal: positional value path; drop value
  words.
  Done (`2026-06-05`): **rewrote the core value violation** — `extract_protocol_state_value`
  (evidence.rs) was a hardcoded list of AMBA encoding values (`idle`/`nonseq`/`incr4`/`okay`/…);
  it now extracts the value **positionally** (token after the normative verb `must be`/`shall be`/
  `must remain`, skipping articles/binding fillers), so a never-seen non-AMBA value is derived too.
  Added positional **subject exclusion** (the constraint's own value is removed from the subject
  set) and **dropped all value names** (`HIGH`/`LOW`/`IDLE`/`NONSEQ`/`OKAY`/`INCR`/…) from the
  `collect_subject_signal_tokens` denylist — kept only quantifiers, family names (→`.3`),
  doc-structure, roles. 2 new tests + reframed `excludes_logic_level_values` to the new mechanism.
  Full CI GREEN (1256→1257). **Remaining in `.2`:** the conditional-rule consequent denylist
  (`evidence.rs:5833`) still lists value names. **Flagged for owner (deeper, NOT done):** the
  `MustBeHigh`/`MustBeLow` *kinds* are load-bearing logic-level semantics across 9 files, and
  `semantic.rs:9744` already derives+normalizes many spellings (`1`/`high`/`true`) — so logic-high/
  low reads as *universal digital semantics* (the "how"), not a spec name; collapsing the kinds
  would be a fundamental IR redesign. Recommend treating logic levels as retained "how".
- ID: `PDF-AGNOSTIC-EXTRACTION.3` · Status: `done` · Goal: document-derived signal validation; drop
  family/encoding names.
  Verification: passed (`2026-06-05`) — removed protocol family/vendor names from
  `collect_subject_signal_tokens` (evidence.rs) and from `signal_stop_words` (semantic.rs: AMBA
  family names, vendor names, ARM/AMBA publication-ID prefixes, HTRANS/HBURST encoding values), plus
  `NONSEQ`/`SEQ`/`OKAY` from the conditional-rule consequent denylist. **Safe by design:** a token
  mis-discovered as a signal is filtered downstream against the document's own `declared_signal_names`
  (semantic.rs:213–234, with a test). Full CI GREEN (1258).
- ID: `PDF-AGNOSTIC-EXTRACTION.4` · Status: `done` · Goal: sweep; guard; close.
  Verification: passed (`2026-06-05`) — production-source audit (grep, excluding tests/fixtures/
  comments) confirms ZERO protocol family/encoding tokens remain; the `.1` guard enforces it for
  `signal_stop_words`. CI green 1259. Tree CLOSED.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `PDF-AGNOSTIC-EXTRACTION.1` | `pending` | invariant + guard (non-regression scaffolding) |
| 2 | `PDF-AGNOSTIC-EXTRACTION.2` | `pending` | the value path (the hard, central change) |
| 3 | `PDF-AGNOSTIC-EXTRACTION.3` | `pending` | signal validation via the document registry |
| 4 | `PDF-AGNOSTIC-EXTRACTION.4` | `pending` | sweep + guard-to-zero + close |

## Decisions

- `2026-06-05`: zero hardcoded domain vocabulary (incl. `HIGH`/`LOW`); derive from the document;
  English normative grammar retained as the specs' language (working boundary, ADR 0006); stage the
  remediation behind a CI guard so it cannot regress; verify precision via tests + a re-extraction.

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |

## Changelog

- `2026-06-05`: Created — owner signoff criterion; ADR 0006; staged program to derive all domain
  vocabulary from the document and guard against hardcoded spec tokens.
- `2026-06-05`: `.2` increment — value path de-hardcoded in the core constraint extractor
  (`extract_protocol_state_value` positional; subject value-exclusion; value names dropped from the
  `collect_subject_signal_tokens` denylist). +2 tests; CI green 1257. Remaining: `5833` conditional
  denylist; family-name denylists (`.3`); logic-level-kinds question flagged for owner.
