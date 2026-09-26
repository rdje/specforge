---
id: a-hold-keeps-its-release
title: A *must remain LEVEL until RELEASE* obligation is a hold (`SignalConstraintKind::MustRemainUntil { value, release }`), never a constant level; its temporal rule takes the release as an exception (guard `(! release)`), is listed when the release states no value, and no stage lowers it as a constant drive; since 2026-09-26 (EXTRACTION-REPRESENTATION-GAPS.25.c)
answers:
  - "how is 'must remain asserted until X' represented"
  - "what is SignalConstraintKind::MustRemainUntil / must_remain_until"
  - "where does the until release go (the temporal rule's exceptions, not the condition)"
  - "why is a hold listed with 'the hold's release … was not read'"
  - "why is AXI AWAKEUP's hold guard unchanged (the condition already excludes the release)"
  - "why did the APB WIRE gold for PWAKEUP change to must_remain_until"
  - "why does a release need 'is'/'are' to be read"
date: 2026-09-26
tags: [evidence, semantic, temporal, hold, isf, extraction-representation-gaps, adr-0006, measured]
evidence: crates/specforge/src/ir/evidence.rs (type_holds, hold_release); crates/specforge/src/ir/semantic.rs (build_temporal_rules, the release block); crates/specforge/src/ir/intent.rs (ConstraintLevel::Hold); crates/specforge/src/ir/isf_ir.rs (LevelRefusal::Hold, unread_hold_release, guard_from_candidates); crates/specforge/test_data/llm_eval/seed_apb.json (statement_0322); docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md (.25.c, .26.b)
reverify: "cargo test --offline -p specforge-core --lib -- a_remain_until_obligation_is_a_hold_with_its_release a_hold_is_lowered_with_its_release_or_listed. jq -r '.signal_constraints[] | select(.constraint_kind.kind == \"must_remain_until\") | .subject_signal' generated/evidence_ir/*/evidence_ir.json -> 9 on 2026-09-26 (AXI AWAKEUP x4, ACWAKEUP, ASKSTOP, ACVALID, CRVALID; APB PWAKEUP)."
---

**Measured `2026-09-26` (`EXTRACTION-REPRESENTATION-GAPS.25.c`).** Nine obligations on the live chains state *must
remain LEVEL until RELEASE*, and all nine were recorded as a constant level (`must_be_high`, `must_be_asserted`). In
five of them the release appeared nowhere in the IR: lowered, the constraint path would drive the level always, and the
temporal path would hold it forever. None was emitted, because each was a Manager input or had an unread condition.

Now:

- **Evidence** retypes the obligation as `MustRemainUntil { value, release }` (`type_holds`). The release is the text
  after *until*, cut only at a marker that opens a separate condition (*… until PREADY is asserted if …*); *after* and
  *before* stay in it (*until the rising clock edge after the Manager asserts the ACREADY signal*). The value is refined
  through a stated polarity, like the level kinds.
- **Semantic** gives the rule the level as its consequent, and the release as an exception, but only when the release
  states a value (*ACTIVATEACK is LOW*). The clause reader also takes a bare capitalized word as a value, which would
  read *… handshake of the DVM transaction* as `ACVALID = DVM`. That latent leniency is `.26.b`.
- **IntentIR** makes no invariant of a hold.
- **The adapter** refuses a hold as a constant level (*is a hold*), and lists a hold with no exception (*the hold's
  release … was not read*). It drops an exception its guard already excludes, so AXI's `AWAKEUP` holds keep
  `(& (== AWAKEUP 1) (== SYSCOREQ 1) (== SYSCOACK 0))`, and `manager.isf` is byte-identical.

The APB WIRE-BASED-100 gold for *PWAKEUP must remain asserted until PREADY is asserted …* moved from
`must_be_asserted` to `must_remain_until`, as the sentence states. HEAD's binary scores 6/6 on the old gold and 5/6 on
the new one; the new binary scores 5/6 and 6/6. `[[a-condition-never-contains-its-obligation]]` ·
`[[a-tie-off-is-not-a-drive]]`.
