# SYMBOL-CLOSURE-CORPUS-VALIDATION: corpus-validate (and settle) the descoped symbol-closure detector

## Metadata

- Tree ID: `SYMBOL-CLOSURE-CORPUS-VALIDATION`
- Status: `done` (CLOSED — negative result: detector confirmed low-value; not built)
- Roadmap lane: `R15e` (completeness / miss detectors)
- Created: `2026-06-01`
- Owner: repo-local workflow
- Parent context: `COMPLETENESS-CLOSURE-INVARIANTS.3` descoped symbol closure as
  "genuinely hard … missed-decl vs external indistinguishable **without careful
  corpus validation**". This tree IS that careful corpus validation.

## Goal

Decide, with corpus evidence (not assertion), whether a symbol-closure /
interface-inventory-coverage miss detector — "a signal referenced by the typed
facts but absent from the declared interface inventory is a candidate miss" — is
worth building, or whether the earlier descoping holds.

## What was done (read-only recon over stored SemanticIR artifacts)

No re-ingest needed: ran the analysis directly on the ~10 stored
`generated/semantic_ir/*/semantic_ir.json` (the canonical inventory lives at the
semantic stage). Reference vs declaration sets compared on the **typed** fields:
`signal_constraints.subject_signal`, `actor_signal_relations.signal_name`,
`signal_polarities.signal_name` vs `interfaces[].signals`.

## Findings (i2c `um10204`, the sharpest case)

1. **Naive typed closure is trivially satisfied (empty).** Every typed-referenced
   signal is already in the inventory — because the inventory is *built from* those
   same typed facts. So "referenced-but-undeclared" at the typed level finds
   nothing. Confirmed empty on i2c.
2. **The inventory carries noise.** Declared "signals" include `IOL` (a DC current
   parameter), `LED` / `PULL_UP_DEVICE` (example devices), `START` / `STOP` (bus
   *phases*), `VSS` (a power rail), `W` (a stray token). A closure invariant does
   not catch these (they are declared, just not real) — and "is this a real signal"
   has no exact, low-FP test.
3. **The real miss is invisible to closure.** `SDA` — the I2C *data* line, one of
   the protocol's two wires — is referenced **90×** in the SemanticIR (constraints,
   timing, rules) yet is **absent from the inventory**. But it is never a *typed*
   reference (it lives only in prose: "The data on the **SDA** line must be
   stable…"); its constraints did not parse `SDA` as the subject. So even a richer
   typed-closure detector misses it. Catching `SDA` would require **prose-scanning**
   for signal-like tokens — exactly the high-false-positive approach (cf. the i2c
   noise above) the descoping warned against.

## Decision

**Do NOT build the symbol-closure / inventory-coverage detector.** The descoping is
**corpus-validated**: at the typed level it is empty; at the prose level it is
high-FP and indistinguishable from extraction noise without a ground-truth oracle.
Building it would add machinery that produces noise, not signal — against the
"residual-honesty / precision-first" doctrine.

The genuinely valuable findings it surfaced are *capture* problems, not *closure*
problems, and are recorded as `CORPUS-HARDENING` follow-up candidates:
- **i2c `SDA` missing from the inventory** (high value: a flagship 2-wire bus
  loses one wire). Root cause: `SDA` is described only in prose and its constraints
  fail to promote `SDA` as the subject, so it never reaches the interface inventory,
  while `SCL` does (the SCL/SDA asymmetry). Fix is a constraint-subject /
  inventory-construction change — broad regression risk + needs re-ingest to
  validate → deferred to a future owned tree when Docling is available.
- **i2c inventory noise** (`IOL`/`LED`/`START`/`STOP`/`VSS`/`W`/`PULL_UP_DEVICE`):
  non-signal tokens promoted to the inventory — a precision issue with several
  distinct sources (DC parameter table, figures, phase words, power rails); no
  clean single filter; future precision tree.

## Task Tree

- ID: `SYMBOL-CLOSURE-CORPUS-VALIDATION`
  Status: `done`
  Goal: corpus-validate the descoped symbol-closure detector; settle build/no-build.
  Children: `.1`

- ID: `SYMBOL-CLOSURE-CORPUS-VALIDATION.1`
  Status: `done`
  Goal: recon stored SemanticIR; compute typed closure; assess precision; decide.
  Verification: >
    passed (`2026-06-01`) — recon over stored SemanticIR (no re-ingest). Typed
    closure empty on i2c (inventory is reference-derived); inventory noise + the
    prose-only `SDA` miss show the detector is low-value/high-FP. Decision: do not
    build; descoping corpus-validated. Two capture-problem findings routed to
    `CORPUS-HARDENING`. Docs-only.
  Commit: `see Commit Log`

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `SYMBOL-CLOSURE-CORPUS-VALIDATION.1` | `done` | recon done; no-build decision recorded; tree CLOSED |

Tree **CLOSED** (`2026-06-01`): negative result — the detector is not worth
building; the descoping is now corpus-validated rather than asserted.

## Decisions

- `2026-06-01`: NO-BUILD. Typed symbol closure is trivially empty; prose-level is
  high-FP. The real value is in fixing capture (i2c `SDA`), tracked under
  `CORPUS-HARDENING`, not in a closure detector.

## Blockers

- None for this tree (decision made). The follow-on capture fixes are re-ingest-
  gated (`.venv-docling` absent; corpus tree moved).

## Verification Log

| Date | Leaf | Checks | Result |
| --- | --- | --- | --- |
| `2026-06-01` | `.1` | recon over stored SemanticIR; typed closure empty (i2c); inventory noise + prose-only SDA miss documented; no-build decision; findings routed | `passed` |

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `SYMBOL-CLOSURE-CORPUS-VALIDATION.1` | `SYMBOL-CLOSURE-CORPUS-VALIDATION.1 — corpus-validate descoped symbol closure (no-build); route i2c capture findings` | docs-only |

## Changelog

- `2026-06-01`: Created + CLOSED — corpus-validated the descoped symbol-closure
  detector as low-value (negative result); routed the i2c `SDA`-missing +
  inventory-noise capture findings to `CORPUS-HARDENING`.
