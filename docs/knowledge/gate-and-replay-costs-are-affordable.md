---
id: gate-and-replay-costs-are-affordable
title: The two runs that parked EXTRACTION-QUALITY-GAUGE.3k.9 as "detached" cost 60 seconds and 19 minutes, both measured inside one working session
answers:
  - "how long does check_doctrines.sh --all take"
  - "how long does replay-constraints over the whole corpus take"
  - "is a full doctrine run too slow to do in a session"
  - "does the full CI-tier doctrine gate pass"
  - "why was EXTRACTION-QUALITY-GAUGE.3k.9 parked"
  - "is the markdown escape defect blocked or refused"
  - "which tree owns the normalized-markdown escape truncation"
date: 2026-09-20
status: current
tags: [extraction-quality-gauge, doctrine-enforcement, measurement, cost, signal-catalog-capture-gap]
evidence: scripts/check_doctrines.sh; crates/specforge/src/commands/replay_constraints.rs; docs/tasks/extraction-quality-gauge/kind-span-family.md (.3k.9); docs/tasks/SIGNAL-CATALOG-CAPTURE-GAP.md (.4)
reverify: "/usr/bin/time -p bash scripts/check_doctrines.sh --all — expect ~1,150 s real and 'ALL 18 executed doctrines PASS'; /usr/bin/time -p specforge replay-constraints --evidence-root generated/evidence_ir — expect ~60 s."
---

`EXTRACTION-QUALITY-GAUGE.3k.9` — the leaf its own tree calls *"where the next real work is"* on the
extraction-recall bottleneck — was parked with the reason *"neither finishes inside a session"*,
naming two runs. Measured `2026-09-20`, both inside one session and alongside other work:

| run | cost |
| --- | ---: |
| `specforge replay-constraints --evidence-root generated/evidence_ir` | **60 s** (`real 60.18`) |
| `bash scripts/check_doctrines.sh --all` | **19 min** (`real 1148.86`), **ALL 18 doctrines PASS** |

A third precondition the same leaf calls detached — *"every current-schema document that moves
rebuilt and diffed"* — was performed the same day by `EXTRACTION-GAP-FIX.5b`: one document through
`evidence → semantic → intent → isf-adapter` with one `validate` per stage takes about a minute,
against a full pre-write snapshot, and the stratum re-verified at 27/27.

**So the parking reason is false and the leaf is a decision, not a scheduling problem.**

## What is NOT refuted, and it is the part that matters

Correcting a timing claim is not an argument for shipping. `.3k.9`'s own magnitude argument is
untouched by any cost measurement: the only fix reaching the two contaminated documents moves the
identity layer of **67** documents to correct **18 names in 2**, with a published constraint effect
of **0**, against a tree rule that *a rule nothing exercises does not ship*. Whoever takes the leaf
decides on that arithmetic; the runs are now merely affordable, not persuasive.

## One defect, formerly owned twice

`SIGNAL-CATALOG-CAPTURE-GAP.4` named the same defect — normalized markdown escapes `_` as `\_` and
identifier tokenization stops at the backslash — with the same 67-of-78 blast radius. Owning one
defect in two trees is how each waits for the other, so `.4` is **superseded** by `.3k.9`, which
already discharged the parts needing no rebuild (all 18 contaminated names adjudicated, all
*remove*, plus a circularity control observed RED). The same fragment surfaces a third time through
the constraint path, in `EXTRACTION-GAP-FIX.5a`'s refusal evidence: `AWSNOOP\_WIDTH must be 5`
minting a constraint on `AWSNOOP`.

Links: [[evidence-statement-markdown-escape-truncates-identifiers]].
