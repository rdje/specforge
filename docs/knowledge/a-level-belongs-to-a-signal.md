---
id: a-level-belongs-to-a-signal
title: A logic level binds to the SIGNAL beside it, read through the document's own catalog — not to the verb that sets it, and not to every signal the statement names
answers:
  - "why did PREQ publish must_be_high when the row sets PREQ LOW"
  - "why was HTRANS must_be_high extracted from a sentence about HSEL"
  - "why did NVM must_be_low come from low level format"
  - "what replaced logic_level_binding_kind_from_text"
  - "how does SpecForge decide which signal a logic level belongs to"
  - "can a signal follow its logic level"
  - "why does the logic level pairing read the declaration catalog instead of the token shape"
  - "what is LOWand and why does the level test read a leading uppercase run"
  - "does a subscript separate a signal from its level"
  - "how does a list of signals share one logic level"
  - "which logic level records did the pairing remove"
date: 2026-09-13
status: current
evidence: crates/specforge/src/ir/evidence.rs (logic_level_bindings, walk_for_level_subjects, token_logic_level, extract_dynamic_signal_constraints, mod extraction_quality_gauge_3k_11, mod wire_based_100_5i); docs/tasks/EXTRACTION-QUALITY-GAUGE.md (.3k.4, .3k.11, .3k.12, .3k.13); docs/book/src/pipeline/obligation-reading.md
reverify: "cargo test -p specforge-core --lib extraction_quality_gauge_3k_11 — eight controls; and `specforge replay-constraints --evidence-root generated/evidence_ir` for the corpus figure"
tags: [evidence-ir, constraint-extraction, genericity, extraction-quality-gauge, claim-verification]
---

The retired `logic_level_binding_kind_from_text` asked one question — *is there a logic level within
six words after a binding verb?* — returned the LAST one, and said nothing about what it belonged to.
The caller then attached that single kind to **every** declared signal the statement named. Two
independent errors in one reader, and one AMBA LPI row shows both:

```text
| P_ACCEPT | … | Device has accepted the request. Controller must set PREQ LOWand PREQCHK HIGH. |
```

Published: `PREQ must_be_high` — the opposite of what the row states.

## Three things about real specification text shaped the fix

| the obvious rule | why the corpus refuses it |
| --- | --- |
| pair with the nearest PRECEDING identifier | *"a controller with an absent or tied LOW QDENY signal"* puts the signal AFTER; the walk goes backward first, forward when backward finds nothing |
| recognise an identifier by its SHAPE (an uppercase run) | `WIRE-BASED-100.5i` feeds this path `signal_alias_000001_ready_000000006d11fd13` and requires identical behaviour. **That control went RED on the first implementation.** Identity is read through the document's own declaration catalog, never through case |
| assume clean word boundaries | the normalizer loses the space in `LOW and` / `HIGH after`, so `token_logic_level` reads a token's leading uppercase RUN as well as the whole token — which is also what stops `PREQCHK HIGH` reaching back past `LOWand` to `PREQ` |

A run must be EXACTLY the level, so a signal whose name merely opens with those letters (`LOWPWR`) is
untouched. A list shares one level (`both QACCEPTn and QDENY LOW`). A SUBSCRIPT is skipped rather than
treated as a boundary — `sets HPROT[0] HIGH` — which the AHB rebuild proved necessary: without it a
correct record is lost alongside the fabricated one.

## What it measured

Corpus replayed is **304 before and 304 after**; the composition is the result.

**11 fabrications removed.** GIC-600 `PMU`/`GIC must_be_high` (the HIGH belongs to the lowercase
tie-off `gicp_allow_ns`) and `MBIST must_be_high` (to the row's own `nmbistreset`); CoreSight
`ATB must_be_low` (to `araddr_m`/`awaddr_m`, while `ATB` comes from `ATB_DATA_WIDTH` a sentence
later); AXI-ACE `WVALID must_be_low` from *"When WVALID is LOW, the write strobes can take any
value"*, a CONDITION; AHB `HTRANS must_be_high` (to `HSEL`); LPI `PREQ`/`PACCEPT must_be_high` (to
`PREQCHK`/`PACCEPTCHK`); NVMe `NVM`/`LBA must_be_low` (*"low level format"*); HBM2 `DM must_be_high`
(*"DM output is not affected by the DBIac function"*).

**15 correct records added**, including ten AMBA LPI P-Channel state-table rows the old reader could
not see, AXI-L `AWSNOOP`/`ARSNOOP must_be_low`, HBM2 `DBI must_be_low` from the `otherwise` branch it
used to swallow, and CoreSight TMC `FULL must_be_high` from *"the FULL output is pulled HIGH"*.

**One correct record is lost and named** (`EXTRACTION-QUALITY-GAUGE.3k.12`): LPI
*"…with the QDENY output absent or tied low"*, where the walk stops at `absent` — a predicate
adjective rather than scaffolding. The same fact stated one figure earlier, with the signal after the
level, still binds. Widening the skip list to fit that one sentence would be fitting the rule to an
instance.

## The modality gate this adjudication surfaced (`.3k.13`, shipped the same day)

The pairing is right and the level is right in AHB's *"It is **recommended** that a Manager sets
HPROT[0] HIGH"* — what was wrong is that a RECOMMENDATION was published as a hard constraint. This
producer types a record from its VALUE BINDER and never from a modal, correctly, so it had **no
modality gate at all**; `EXTRACTION-QUALITY-GAUGE.3k.2a` refuses the same shape in the statement path
but that refusal rides the kind classifier, which this producer never reaches.

`binding_is_non_mandatory` is deliberately narrow, and the reason is the thing to carry forward: **a
flat binding with no modal anywhere is exactly what this reader exists to capture** (*"the FULL output
is pulled HIGH"*, *"AERR, DERR are driven LOW"*). So only an EXPLICIT marker refuses, a mandatory
modal in the same clause outranks a permission in it, and a permission granted in one sentence does
not suppress the requirement stated in the next. Measured population: **2 records** — AHB's
recommendation and one AMBA LPI figure caption (*"shows how a device **can** be interfaced … with an
absent or tied LOW QDENY signal"*), whose real requirement the document states separately in its own
mandatory clauses and which are untouched.

Related: [[the-binding-bearing-clause]], [[one-record-per-obligation-clause]],
[[constraint-record-producer-strata]].
