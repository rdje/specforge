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

**Guiding principle (owner's words):** *"SpecForge should be smart about how to extract things from
a chip-spec PDF — it should remember how to do things right, but it shouldn't remember particular
names."* The **method** (how to read tables, normative grammar, structure) is the intelligence to
keep; the **names** are read from the document, never stored.

**No SpecForge production code may hardcode domain vocabulary that belongs to, or is derived from,
any specific chip-spec PDF.** This includes, with no exceptions:

- signal / port names (`PSEL`, `HTRANS`, …),
- protocol-defined **value / state / encoding names** a spec assigns (`NONSEQ`, `INCR4`, `OKAY`,
  `IDLE`, and any custom enum states) — derived positionally from the document,
- protocol family and vendor names (`AMBA`, `AXI`, `ARM`, …).

All such vocabulary **must be derived from the document under analysis** — its own signal tables,
port declarations, and stated values — never from a list baked into the code. When the safe choice
and the convenient choice diverge, **prefer the choice that preserves PDF-independence.**

### Boundary (what is NOT domain vocabulary)

The **English normative/structural language the specs are written in** — `must`, `shall`, `when`,
`if`, `until` — is the natural language SpecForge reads, not a name owned by any spec, and may be
used. Document-independent algorithmic logic (token shapes, positional grammar) is likewise fine.

**Logic levels are *how*, not a name.** The universal binary-logic concept — `high` = logic 1,
`low` = logic 0 — is assumed by *every* digital specification and *defined* by none, and SpecForge
cannot lower `"X must be high"` to the correct assertion without knowing high = 1. It is therefore
the irreducible **"how."** Crucially, SpecForge does **not** assume one spelling: a logic level is
recognized through a **convention-agnostic** set — `1`/`0`, `high`/`low`, `true`/`false`,
`1'b1`/`1'b0` — so a spec writing `1`, `H`, or `true` is handled equally. This set is centralized in
`crates/specforge/src/ir/normative_vocab.rs` as the single authority. A spec's **own value/state
names** (above — `NONSEQ`, `IDLE`, custom enum states) remain *derived from the document*; only the
two universal levels are retained. *(This refines the earlier draft that listed `HIGH`/`LOW` as
forbidden: the forbidden thing is presuming a spec's value vocabulary or one spelling convention —
not the universal binary-logic concept itself.)*

## Consequences

- The constraint extractor's **value path** extracts the value **positionally** (whatever token the
  spec places after the normative verb), with a spec's own value names derived from the document.
  Logic-level spellings among those tokens map to `MustBeHigh`/`MustBeLow` through the centralized
  convention-agnostic vocabulary (the retained universal "how"), not an ad-hoc `high`/`low` table.
- Signal-candidate validation moves from **denylists of one protocol's words** to **positive
  validation against the document's declared signals** (the `declared_signal_names` registry
  already exists and is partially used).
- A **CI guard** should fail the build if known spec-specific tokens reappear in production sources,
  so the invariant cannot silently regress.
- Tracked by tree `PDF-AGNOSTIC-EXTRACTION`. Test/eval **fixtures** may use real names (they are
  data, not logic) — the invariant is about production code paths.
