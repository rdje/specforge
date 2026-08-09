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
  - "how many sentence-start signal descriptor phrases pollute the retained corpus"
  - "why is bus not a valid parenthetical single-wire head"
  - "what table structure is required before port or pin vocabulary grants signal authority"
  - "does relation-derived direction synthesis independently invent signal names"
  - "does select_initiator_actor choose the first or last equal maximum"
  - "which formal signal declaration predicates does the dense prose authority gate accept"
  - "how does CORPUS-COVERAGE 2 33d ii prevent weak signal names from reentering through relations"
  - "why was CORPUS-COVERAGE 2 33d iii closed without another convergence or adapter filter"
  - "why does USB 3.2 still emit hundreds of low confidence ISF outputs after the four false signals are removed"
  - "what happens when SemanticIR has no authoritative signal declarations"
date: 2026-08-09
tags: [corpus-coverage, dense-prose, signal-inventory, actor-signal-relations, isf, semantic-fidelity, false-positive, table-classification, fixed-point]
evidence: docs/research/dense-prose-signal-authority-measurement.md; generated/source_ir/usb_3_2_revision_1_0_2017_09/source_ir.json; generated/evidence_ir/usb_3_2_revision_1_0_2017_09/evidence_ir.json; generated/semantic_ir/usb_3_2_revision_1_0_2017_09/semantic_ir.json; generated/intent_ir/usb_3_2_revision_1_0_2017_09/intent_ir.json; generated/adapters/isf/usb_3_2_revision_1_0_2017_09/adapter.json; crates/specforge/src/ir/source/docling_backend.rs (classify_table_kind); crates/specforge/src/ir/evidence.rs (collect_known_signal_names, synthesize_signal_declarations_from_prose, actor_signal_relation_surface, synthesize_directions_from_relations); crates/specforge/src/ir/semantic.rs (build_interfaces; retain_authoritative_interface_candidate_signals); crates/specforge/src/ir/isf_ir.rs (select_initiator_actor); docs/tasks/CORPUS-COVERAGE.md (.2.33c/.2.33d)
reverify: "Build USB 3.2 EvidenceIR through adapter with the current release binary. Expect EvidenceIR 8267 statements and 0 actor_signal_relations; IntentIR 18 actors, 0 interfaces, 0 actor_ports/relations, and no AT/USB/ENHANCED/NO port; adapter blocked with 0 signals/rules, no emitted *.isf, and no setportfeature_port_over_current.isf or channel.isf sibling."
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

## Corpus boundary measurement and selected repair (`CORPUS-COVERAGE.2.33d.i`)

The follow-up census establishes that the USB loop is an instance of three corpus-wide authority mismatches,
not four independent name producers. Across 80 retained EvidenceIR artifacts, sentence-start `signal <word>`
has 3,104 formal `is input|output|width` matches and 97 non-declaration matches across 28 documents. Fifteen
weak document/token pairs become relation-active (120 relations), and 11 are promoted to output declarations.
Requiring the emitted formal predicate preserves all formal matches and removes the weak class without a name
list.

The sparse prose fallback carries 17 `Bus (ACRONYM)` width-one declarations across six documents; 14 become
relation-active (135 relations) and 12 gain output declarations. Every one names a bus/protocol rather than a
single wire. Removing only `bus` from the single-wire-head grammar preserves the I2C/I2S/SWD/SWP `line`,
`clock`, `data`, `pin`, and definitional oracles.

The table census finds 32 port/pin-only `signal_description` classifications across 11 documents; 11 tables
across five documents pass the current authority gate and yield 32 raw row candidates. Because the family mixes
real connector pins with false state/status matrices, the repair is structural rather than a blanket drop:
ordinary port/pin vocabulary must be backed by a compact signal/name/symbol/pin identity header or a headerless
connector/pin-diagram shape. Explicit signal inventories and rotated name-column tables remain valid.

Relation-derived direction synthesis is an amplifier, not an independent signal-name source: prose relations
scan only the known-signal set, table relations share the table authority gate, and direction synthesis copies
an existing relation name. Therefore a separate adapter/convergence heuristic is measured unnecessary if the
three catalog fixes and real USB rebuild prove closure. The code slice must lock that with a direct convergence
regression.

Finally, nine of 79 retained IntentIR artifacts have a maximum net-producer tie (51 tied actors). Rust's live
iteration selects the lexicographically last equal maximum, contrary to the nearby first-wins comment. USB's
three `(2,0)` phantoms demonstrate this, but changing tie order would only choose another false actor; the code
slice should preserve behavior, correct the comment, and test the tie explicitly.

## Implemented authority boundary (`CORPUS-COVERAGE.2.33d.ii`)

The shared declaration parser now requires the canonical `Signal <identifier> is <predicate>` grammar. Accepted
predicates are `input`, `output`, `inout`, `internal`, `local`, and `width`; the valid `inout` arm is a product
contract even though the retained measurement contains no occurrence. The full library gate caught and retained
that zero-corpus case through the SWD interface-edge tests. Ordinary phrases such as `signal at its upstream
port` and `Signal level ...` no longer enter either declaration catalog.

The parenthetical fallback no longer treats `bus` as a single-wire head. The remaining `line`, `signal`,
`clock`, `data`, `wire`, and `pin` heads plus the independent pin-appositive and definitional grammars preserve
the I2C/I2S/SWD/SWP contracts. The shared table authority gate now requires an explicit signal caption, a
compact signal/name/symbol/pin identity header, or a headerless connector-pin diagram. The real USB state legend,
VBUS requirements matrix, and port-status table shapes fail that gate; rotated `Name` inventories, SWJ routing,
and MIPI connector diagrams pass.

A direct build regression combines the exact weak declaration, `Universal Serial Bus (USB)`, VBUS matrix, and
actor-relation shapes. `AT`, `USB`, `ENHANCED`, and `NO` are absent from the resulting known-signal set, relation
surface, table provenance, and direction declarations. This proves the fixed point is closed at its three name
authorities; no adapter deny-filter is added. Initiator selection remains behavior-identical and now explicitly
tests the lexicographically last equal maximum.

## Conditional convergence backstop closed (`CORPUS-COVERAGE.2.33d.iii`)

No separate downstream filter is warranted. The affected Rust paths remain byte-identical to implementation
commit `5c95a041`; the combined regression proves all four weak names absent across the complete EvidenceIR fixed
point, and four supporting authority/tie tests also pass. Relations consume only names admitted by the formal or
table catalogs, while direction synthesis copies an existing relation name. A rejected name therefore has no
remaining re-entry seam. Filtering again in convergence or the adapter would duplicate policy and could delete a
legitimate relation-grounded direction. The real USB cascade in `.d.iv` remains the independent falsification
gate for this conclusion.

## Real-cascade falsification: a second authority-empty interface loop (`.2.33d.iv`)

The repaired current-binary rebuild validates the original fixed-point repair: EvidenceIR falls from 45 actor
relations to zero; IntentIR falls from 51 to 18 actors and from 42 actor ports / 45 relations to zero; the four
weak names and the phantom `setportfeature_port_over_current` actor do not reach the current adapter. Writer
reconciliation also removes the obsolete false-actor file, so it cannot masquerade as current output.

The same run falsifies the broader signoff. With no formal signal declarations,
`retain_authoritative_interface_candidate_signals` returns every heuristic candidate unchanged. `build_interfaces`
therefore turns uppercase tokens co-mentioned in raw source facts into 918 low-confidence interfaces. Examples
are hexadecimal/encoding-table rows: statement 7800 (`F0 84 | 01 | A0 | ...`) creates interface signals
`A0/A4/AF/D5/EE/F0`; statement 7644 (`D0.5 | A0 | ...`) creates `A0/D0`. IntentIR carries 2,940 signal records,
and the adapter deduplicates them into 556 one-bit outputs even though the actor graph is empty. The result is a
different fabricated hardware surface, so `.d.iv.b` owns a measurement-first authority repair and `.d.iv.c`
retains final USB signoff. The upstream fixed-point conclusion remains valid; it was not sufficient by itself.

## Authority-empty loop closed (`.2.33d.iv.b`)

The corpus census found 21 all-low-confidence documents carrying 5,527 interfaces / 18,397 signal records; 19
adapters were marked renderable and consumed 4,060 per-document unique names. The empty-authority bypass is now
replaced by a fail-closed positive grammar. Formal and clock/reset interfaces remain on their typed paths; a
declaration-free group survives only when its statement begins with one candidate and makes a deontic signal
action. This preserves the canonical `VALID must remain asserted until READY is observed` handshake without
admitting raw rows, ordinary prose, or relation-only `DOWNSTREAM`.

All 21 affected documents dry-run to zero interfaces/records. The real USB rebuild now blocks on `no signals
declared in interface`, carries zero adapter signals/rules, emits no `.isf`, and removes the obsolete
`channel.isf`. See `[[semantic-interface-authority-empty-fallback]]` for the complete causal and corpus record.
