# 0006 — No hardcoded chip-spec vocabulary (PDF-independence is a hard invariant)

- Status: accepted
- Date: 2026-06-05
- Deciders: project owner (signoff criterion), repo-local workflow

## Context

SpecForge ingests intent from **arbitrary** chip-spec PDFs — potentially hundreds, across vendors
and protocol families, and **across revisions of the same spec** (e.g. AMBA renamed
"Master/Slave" → "Manager/Subordinate"; HTRANS/HBURST encodings differ per protocol). Production
code was found to hardcode tokens belonging to specific specs: protocol family names
(`AMBA`/`AHB`/`APB`/`AXI`/`CHI`), AMBA encoding values (`NONSEQ`, `INCR4`, `WRAP8`, `OKAY`), and
even generic value names (`HIGH`/`LOW`) — in `evidence.rs` (two `collect_subject_signal_tokens`
denylists), `semantic.rs` (`signal_stop_words`, ~421 entries), and elsewhere.

Hardcoding spec vocabulary is both **non-agnostic** (a 101st protocol or a custom bus is unknown to
the tool) and **brittle** (it rots when any one spec changes its wording in a later revision).

## Decision

**No SpecForge production code may hardcode domain vocabulary that belongs to, or is derived from,
any specific chip-spec PDF.** This includes, with no exceptions:

- signal / port names (`PSEL`, `HTRANS`, …),
- value / state names — **including `HIGH` and `LOW`** (a spec may use `1`/`0`, `H`/`L`,
  `asserted`, or other conventions),
- protocol family and vendor names (`AMBA`, `AXI`, `ARM`, …),
- protocol-defined encoding values (`NONSEQ`, `INCR4`, `OKAY`, …).

All such vocabulary **must be derived from the document under analysis** — its own signal tables,
port declarations, and stated values — never from a list baked into the code. When the safe choice
and the convenient choice diverge, **prefer the choice that preserves PDF-independence.**

### Boundary (what is NOT domain vocabulary)

The **English normative/structural language the specs are written in** — `must`, `shall`, `when`,
`if`, `until` — is the natural language SpecForge reads, not a name owned by any spec, and may be
used. Document-independent algorithmic logic (token shapes, positional grammar) is likewise fine.
*(If this boundary is later tightened, update this ADR.)*

## Consequences

- The constraint extractor's **value path** must change from hardcoded classification
  (`"must be high"` → `MustBeHigh`) to **positional/generic value extraction** (the value is
  whatever token the spec places after the normative verb), with semantics derived from the
  document, not a `high`/`low` table.
- Signal-candidate validation moves from **denylists of one protocol's words** to **positive
  validation against the document's declared signals** (the `declared_signal_names` registry
  already exists and is partially used).
- A **CI guard** should fail the build if known spec-specific tokens reappear in production sources,
  so the invariant cannot silently regress.
- Tracked by tree `PDF-AGNOSTIC-EXTRACTION`. Test/eval **fixtures** may use real names (they are
  data, not logic) — the invariant is about production code paths.
