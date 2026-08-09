---
id: dense-prose-false-signal-loop-reaches-isf
title: Independent false-signal seeds can reinforce through relation-derived directions and reach a syntactically valid but semantically untrustworthy ISF adapter
answers:
  - "why does USB 3.2 emit AT ENHANCED NO and USB as ISF signals"
  - "why does setportfeature port over current become the USB 3.2 adapter actor"
  - "can a dense-prose phantom actor reach emitted ISF"
  - "does FSMGen strict success prove that a SpecForge adapter is semantically faithful"
  - "why is the USB 3.2 adapter syntactically valid but semantically untrustworthy"
  - "what is CORPUS-COVERAGE.2.33d dense-prose adapter trust repair"
date: 2026-08-09
tags: [corpus-coverage, dense-prose, signal-inventory, actor-signal-relations, isf, semantic-fidelity, false-positive, table-classification, fixed-point]
evidence: generated/source_ir/usb_3_2_revision_1_0_2017_09/source_ir.json; generated/evidence_ir/usb_3_2_revision_1_0_2017_09/evidence_ir.json; generated/intent_ir/usb_3_2_revision_1_0_2017_09/intent_ir.json; generated/adapters/isf/usb_3_2_revision_1_0_2017_09/adapter.json; crates/specforge/src/ir/source/docling_backend.rs (classify_table_kind); crates/specforge/src/ir/evidence.rs (collect_known_signal_names, synthesize_signal_declarations_from_prose, actor_signal_relation_surface, synthesize_directions_from_relations); crates/specforge/src/ir/isf_ir.rs (select_initiator_actor); docs/tasks/CORPUS-COVERAGE.md (.2.33c/.2.33d)
reverify: "Build USB 3.2 through adapter with the current release binary. jq extracted_statements 7916/8269/8416-8419 and actor_signal_relations for AT/ENHANCED/NO/USB from generated/evidence_ir/usb_3_2_revision_1_0_2017_09/evidence_ir.json; inspect SourceIR table_0210; group generated/intent_ir/usb_3_2_revision_1_0_2017_09/intent_ir.json actor_ports by actor_name and direction; inspect adapter.json .isf; run subs/fsmgen/bin/fsmgen --strict --check --json generated/adapters/isf/usb_3_2_revision_1_0_2017_09/setportfeature_port_over_current.isf. Expect four false signals, three 2-output/0-input phantom candidates, selected setportfeature_port_over_current, and FSMGen success with zero diagnostics."
---

**Measured `2026-08-09` (`CORPUS-COVERAGE.2.33c`, current release cascade).** USB 3.2 proves a
stronger failure mode than the previously measured dense-prose actor noise in
`[[agent-identity-prose-class-measurement]]`: independent false-signal seeds can reinforce through the
EvidenceIR convergence loop, create connected phantom actors, and reach an emitted `.isf`. The resulting
file passes FSMGen strict syntax with zero diagnostics but is not a faithful hardware model. Repair is owned
by `CORPUS-COVERAGE.2.33d`; the refresh slice records the result as semantically blocked, not successful ISF.

Four different generic seams seed the false inventory:

- `AT`: ordinary source statement 7916 starts `signal at its upstream port ...`.
  `collect_known_signal_names` accepts the token after sentence-initial `signal ` without requiring a
  declaration predicate, uppercasing the preposition into a candidate signal.
- `USB`: the sparse-catalog parenthetical fallback sees `Universal Serial Bus (USB)` in statement 1154.
  Because `bus` is an allowed wire-head noun, it synthesizes `Signal USB is width 1.` even though this use
  names the protocol, not a wire.
- `ENHANCED` and `NO`: SourceIR table 0210 is a VBUS requirements matrix. Its caption contains the ordinary
  word `Port`, so `classify_table_kind` labels it `signal_description`; the first words of row labels
  `Enhanced SuperSpeed` and `No VBUS` then enter the known-signal catalog.

The fixed-point turns these weak candidates into strong-looking declarations. Broad prose relation rules find
45 actor/signal relations involving the four candidate names. `synthesize_directions_from_relations` promotes
the first `Drives` relation for each into `Signal X is output.` (statements 8416–8419), so the next pass treats
them as explicit interface signals. For example, source statement 7132 merely says that
`SetPortFeature(PORT_OVER_CURRENT)` requests shall not be used and may be treated as no-ops; it becomes a
phantom actor that drives both `NO` and `USB`.

IntentIR contains three tied net-producer phantoms at two outputs / zero inputs:
`ClearPortFeature(PORT_CONNECTION`, `ClearPortFeature(PORT_LINK_STATE`, and
`SetPortFeature(PORT_OVER_CURRENT`. `select_initiator_actor` iterates a name-ordered `BTreeMap` and uses
`max_by_key(out, in)`; the live adapter demonstrates that the equal-key winner is the lexicographically last
candidate, yielding `setportfeature_port_over_current`. This also contradicts the nearby comment that a later
equal candidate never displaces an earlier one, but tie order is secondary: every tied candidate is false.

The emitted adapter has four ports (`AT`, `ENHANCED`, `NO`, `USB`), four prose-derived enums, one nonsensical
65-bit storage record, and 29 mostly unconditional rules driving `USB`. FSMGen reports success and zero
diagnostics because those constructs are syntactically legal. Therefore **FSMGen strict is a syntax and
downstream-compatibility gate, not a semantic-fidelity oracle**. Semantic trust must be established before
emission by grounded inventory/classification/relation gates or made an explicit adapter block; strict success
alone must never be reported as faithful output.

The repair must be universal and measurement-first. No USB token, vendor/document key, or expanding word
denylist can be used. It must address the independently unsafe seams, prevent relation-derived declarations
from laundering weak candidates, keep WIRE/FSMGen/KG gold behavior, and add a real USB end-to-end regression.
