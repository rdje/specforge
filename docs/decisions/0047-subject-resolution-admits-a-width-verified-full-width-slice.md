---
id: subject-resolution-admits-a-width-verified-full-width-slice
title: Subject resolution admits a width-verified full-width slice, and nothing else
date: 2026-09-18
status: accepted
scope: genericity, evidence-ir, entity-typing, llm-primary, identifier-resolution
evidence: docs/decisions/0037-identifiers-are-opaque-and-one-way-grounded.md; crates/specforge/src/ir/entity_typing.rs (resolve_full_width_slice_alias); crates/specforge/src/ir/evidence.rs (stated_signal_widths); crates/specforge/src/commands/extract_constraints_llm.rs; docs/tasks/extraction-quality-gauge/llm-path-family.md (.3j.2.a, .3j.2.a.i, .3j.2.a.ii)
reverify: "cargo test -p specforge-core --lib alpha_ && cargo test -p specforge --lib alpha_ && cargo test -p specforge-core --lib a_full_width_slice_resolves_and_nothing_else_does && cargo test -p specforge-core --lib the_production_composition_records_the_signal_a_full_width_slice_names && cargo test -p specforge-core --lib stated_signal_widths_reads_the_declared_numeral_and_refuses_a_disagreement"
answers:
  - "what is ADR 0047"
  - "how many ways may a model proposal resolve to a declared signal (three: exact, unique case-fold, and a full-width slice verified against a width the document states)"
  - "may SpecForge resolve ARLEN[7:0] to ARLEN"
  - "why is AWSNOOP[3] not resolved to AWSNOOP"
  - "does ADR 0047 weaken ADR 0037's opaque-identifier rule"
  - "why is a bit slice not a spelling heuristic"
  - "what happens when a signal states no width and its slice is proposed"
  - "why must a slice start at bit 0 as well as span the stated width"
---

# ADR 0047: Subject resolution admits a width-verified full-width slice, and nothing else

## Context

ADR 0037 §3 enumerates how a model proposal may reach a declared identity: an **exact** match, and a
**case-folded** match that yields exactly one current-document identity. `EXTRACTION-QUALITY-GAUGE.3j.2.a.i`
shipped a **third** mode — `X[w-1:0]` resolves to `X` when `w` is the width the document states for `X` —
and §3 reads as an exhaustive permission. A shipped resolution mode whose authority is a task leaf is not
authorized, so `.3j.2.a.ii` opened to either obtain the authority or withdraw the rule.

The evidence is bounded and was gathered before the rule was written. `.3j.2.a` read all 16 persisted
LLM-constraint subjects that carry a declared name and are refused for their spelling. Resolving all of
them would make **4 of 16** records correct — below the 3/7 that `.3j` had already answered NO to — while
**inventing** one subject (`snoop response` carries the declared token `SNOOP`, but its sentence is about
the snoop *response* payload) and **strengthening** one obligation. Restricted to a full-width slice it is
**3 of 3**, and those three are `AWCMO[1:0]` of a stated width 2, `ARLEN[7:0]` of 8, and `ARCACHE[3:0]` of
4.

## Decision

1. **A third resolution mode is authorized, and it is the only addition to ADR 0037 §3.** A proposal
   spelled `X[hi:lo]` resolves to the declared identity `X` when, and only when, `lo == 0`, `hi + 1`
   equals the width **the document itself states** for `X`, and `X` resolves to exactly one identity under
   §3's existing rules. `X[n]` is read as the span `(n, n)` and is subject to the same test.
2. **This does not weaken ADR 0037 §1.** The warrant is not the spelling. `X[w-1:0]` is resolved because
   the document **declares a width** and the span covers it — a typed declaration under §2's
   "bounded definitional grammar", not a resemblance, prefix, suffix or substring inference. No semantic
   property (direction, polarity, handshake, clock, reset, role) is ever derived from the slice.
3. **This does not weaken ADR 0037 §3's actual concern, which is identity minting.** The mode lands on a
   name the document already declares. It cannot create an identity, cannot add an external one, and
   cannot disable grounding. As under §3, an exact match wins and an ambiguous base fails closed.
4. **Both guards are load-bearing and neither may be dropped as redundant.** A slice that reaches the top
   bit but not bit 0 — `X[w-1]` — satisfies `hi + 1 == w` **on its own**, so the width comparison alone
   would admit precisely the one-bit case this decision exists to refuse. `lo == 0` is not implied by it.
5. **A proper sub-slice is never resolved.** `AWSNOOP[3]` against a stated width of 4 is one bit of four.
   Resolving it would put *"AWSNOOP must be LOW"* on the record when the document said *"AWSNOOP[3] must
   be tied LOW"* — a strictly stronger obligation, fabricated silently and trusted by every downstream
   gate. This is the failure mode the decision is bounded to prevent.
6. **A slice whose signal states no width is never resolved.** The test cannot be evaluated, and an
   unanswerable question is not a licence to resolve. This is structural rather than rare: measured
   `2026-09-18`, only **209 of 353** declaration-grammar names across the seven promoted documents state a
   width at all.
7. **A stated width that the document contradicts yields no width.** A name declared with two different
   widths resolves to none, rather than to either of them.
8. **A bare qualifier is not a slice and does not resolve.** `WTAG bits`, `Subordinate LAPM`,
   `snoop response`: measured 1 correct of 6, and refused.
9. **Alpha-equivariance (ADR 0037 §7) is preserved.** Renaming a declaration renames its width statement
   and every bound occurrence, slice spellings included, so admission, roles, conflicts, residual
   disposition and lowering eligibility are unchanged.

## Consequences

- Subject resolution now has exactly three modes. Any fourth requires its own decision record; the
  rejected candidates are recorded above with their measured precision so the next proposal starts from
  evidence rather than from intuition.
- The mode is implemented as `entity_typing::resolve_full_width_slice_alias`, deliberately **beside**
  `resolve_unique_document_identifier` rather than inside it: that function's contract is opaque identity
  and other surfaces depend on it, so slice grammar is asked as a separate question. The width comes from
  `evidence::stated_signal_widths`, which reads the canonical `Signal <name> is width <n>.` grammar under
  exactly the admission rule the declared catalog uses, so the catalog and the widths cannot disagree
  about what a declaration is.
- Measured reach on the persisted corpus: **3 of the 16** carried-name subjects, re-derived through the
  shipped function itself and agreeing with an independent width classifier. No persisted artifact moves,
  because nothing re-grounds a persisted record.
- **ADR 0037's own `reverify` command is corrected here, and the correction is named rather than silent.**
  It ran `cargo test -p specforge --lib alpha_`, which executes **3** alpha-equivariance controls; the
  same filter against `specforge-core` — the crate `crates/specforge/src/ir/**` actually compiles into,
  by `#[path]` — executes **11**. The decision that defines alpha-equivariance was verifying it with a
  command that reached a fifth of its controls. No decision text changed and all 14 pass; only the
  verification command did (`COMMIT-GATE-SINGLE-RUN.5` is the same defect's first instance).

## Links

- Extends [ADR 0037](0037-identifiers-are-opaque-and-one-way-grounded.md) §3.
- Bound by [ADR 0006-genericity](0006-no-hardcoded-chip-spec-vocabulary.md).
- Owning work: `EXTRACTION-QUALITY-GAUGE.3j.2.a`, `.3j.2.a.i`, `.3j.2.a.ii`.
