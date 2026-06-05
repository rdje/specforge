# PDF-AGNOSTIC-EXTRACTION: remove all hardcoded chip-spec vocabulary; derive from the document

## Metadata

- Tree ID: `PDF-AGNOSTIC-EXTRACTION`
- Status: `active` (program; `.1` invariant + guard, then staged remediation)
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

- ID: `PDF-AGNOSTIC-EXTRACTION` · Status: `active` · Children: `.1` · `.2` · `.3` · `.4`
- ID: `PDF-AGNOSTIC-EXTRACTION.1` · Status: `pending` · Goal: ADR + CI guard scaffolding.
- ID: `PDF-AGNOSTIC-EXTRACTION.2` · Status: `pending` · Goal: positional value path; drop value
  words.
- ID: `PDF-AGNOSTIC-EXTRACTION.3` · Status: `pending` · Goal: document-derived signal validation;
  drop family/encoding names.
- ID: `PDF-AGNOSTIC-EXTRACTION.4` · Status: `pending` · Goal: full sweep; guard to zero; close.

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
