---
id: transaction-channel-membership
title: KG-ISF-TRANSACTIONS.2m — a transaction's signal-set membership is grouped by the document-declared CHANNEL, recovered DETERMINISTICALLY from the universal `<role> channel signals` table-caption cue (EvidenceIR `signal_channel_memberships` → SemanticIR → IntentIR `TransactionIntent.channel_membership`); ambiguity-gated for boundary precision, metadata-only (never lowered to `.isf`)
answers:
  - "how does SpecForge group a transaction's signals by channel"
  - "what is KG-ISF-TRANSACTIONS.2m / the channel-membership lever"
  - "how is AXI per-signal channel membership recovered without a VLM"
  - "what is the `<role> channel signals` caption cue and how is it parsed"
  - "where does signal_channel_memberships live (EvidenceIR) and how is it carried (SemanticIR -> IntentIR)"
  - "what is TransactionIntent.channel_membership and where is it built (mint_named_transaction)"
  - "why is the channel role kept verbatim instead of mapped to address/data/response phases"
  - "how does the ambiguity gate keep channel membership boundary-precise (bar #3)"
  - "how are continuation table fragments (B1.1 Continued from previous page) chained to a channel role"
  - "how does the table-number grammar handle both B1.1 colon and A2-2 dash forms"
  - "why is channel membership metadata-only and not lowered to .isf"
  - "does .2m change the emitted .isf or the WIRE-BASED-100 surfaces (no — provably orthogonal)"
  - "which validate metrics/finding surface channel membership"
  - "which AXI transactions gain channel grouping (atomic/prefetch/writezero/writedeferrable/narrow_transfer)"
date: 2026-06-17
tags: [kg-isf-transactions, transactions, channel-membership, axi, ace, chi, adr-0006, structured-first, deterministic, metadata, wire-based-100, evidence-ir, intent-ir, north-star]
evidence: "crates/specforge/src/ir/evidence.rs (SignalChannelMembershipRecord, build_signal_channel_memberships / derive_channel_role / derive_continuation_table_number / split_leading_table_number, EvidenceIr.signal_channel_memberships); crates/specforge/src/ir/semantic.rs (SemanticIr.signal_channel_memberships carry); crates/specforge/src/ir/intent.rs (TransactionChannelMembership, TransactionIntent.channel_membership, mint_named_transaction grouping); crates/specforge/src/commands/validate.rs (transactions_with_channel_membership / transaction_channel_groups + intent_transaction_channel_membership); docs/tasks/KG-ISF-TRANSACTIONS.md (.2m node); docs/book/src/pipeline/intentir.md (Grouping a transaction's signals by channel)"
reverify: "Deterministic + RAM-safe (no VLM). Build the binary (cargo build --release -p specforge) then rebuild AXI through the stages into temp JSON: specforge evidence generated/source_ir/ihi0022_l_2025_08_amba_axi_protocol_specification/source_ir.json --dry-run | sed -n '/^{/,$p' > /tmp/ev.json (assert jq '.signal_channel_memberships|length' == 154); specforge semantic /tmp/ev.json --dry-run | sed -n '/^{/,$p' > /tmp/sem.json; specforge intent /tmp/sem.json --dry-run | sed -n '/^{/,$p' > /tmp/int.json; specforge validate /tmp/int.json | grep with_channel_membership -> 'with_channel_membership: 5 (10 channel group(s))'. WIRE-BASED-100 orthogonality: git stash the .2m diff, rebuild AXI evidence, diff vs the .2m evidence -> only signal_channel_memberships differs; adapt /tmp/int.json --target isf --dry-run with vs without channel_membership stripped -> identical .isf source (emitter isf_ir.rs lowers tx.steps only). kg-bench 156/156; run_ci.sh GREEN (lib 1664)."
---

**Established `2026-06-17` (`KG-ISF-TRANSACTIONS.2m`, measurement-first, CODE — GO).** This card records HOW
SpecForge groups a recognized transaction's signal-set membership (`.2c`/`.2k`) by the **channel** the document
declares each signal belongs to, deterministically, with no VLM and no chip-name list. It is the build-out of the
`.2l` Q1 finding (`[[transaction-phase-membership-vlm-vs-channel]]`) that AXI's transaction phases ARE its
channels and the channels are declared in the document's own captions.

## The cue (universal, no name list — ADR 0006)
A signal earns a channel from the table that DECLARED it, when that table's caption is of the universal form
**`<role> channel signals`** — `Table B1.1: Write request channel signals` → channel role `write request`,
`B1.2: Write data channel signals` → `write data`, `B1.3: Write response`, `B1.4: Read request`, `B1.5: Read
data`, `B1.6/B1.7` snoop. The role is the words the document places before "channel signals", kept **verbatim**
(lowercased). It is deliberately NOT re-interpreted into the abstract `address`/`data`/`response` phase names —
that mapping is AXI-family semantic knowledge, is not universal (a platform interconnect's channels need not be
addr/data/resp), would duplicate/conflict with the `.2g` prose-derived `transaction_phases`, and risks
fabricating a phase the document never named for a signal. So channel membership is a **distinct typed dimension**,
complementary to phase membership.

## Where it is computed and carried
EvidenceIR build is the only stage with BOTH the provenance (signal → table_id, in
`table_signal_declaration_provenance`) and the SourceIR table captions, so the join lives there:
`build_signal_channel_memberships(structured_tables, provenance)` (`ir/evidence.rs`) →
`EvidenceIr.signal_channel_memberships: Vec<SignalChannelMembershipRecord {signal_name, channel_role, table_ids}>`
(serde-skip-if-empty). SemanticIR carries it forward verbatim (clone from `evidence_ir`), mirroring the
`transaction_anchors` data flow. IntentIR's `mint_named_transaction` (`ir/intent.rs`) groups each named
transaction's membership ports by channel role → `TransactionIntent.channel_membership:
Vec<TransactionChannelMembership {channel_role, ports}>`.

## Three precision mechanisms (signoff-quality, measured)
1. **Table-number grammar** — `derive_channel_role` strips an optional `Table ` + a leading table-number token
   (`[A-Za-z]*\d+(?:[.\-]\d+)*` — handles both the 2025 `B1.1` colon form and the 2021 `A2-2` dash form) BEFORE
   the role, so a dash digit never leaks into the role; a role that still holds a digit or any non-letter is
   rejected. `channel signal[s]` is the head terminator (bare `... channel` without `signals` is NOT a cue).
2. **Continuation chaining** — a role-stripped fragment caption (`B1.1 Continued from previous page`) inherits the
   head table's role by the caption's own table NUMBER (`derive_continuation_table_number`), so a channel table
   split across pages does not strand its tail (recovers AXI write-request 26→50).
3. **Ambiguity gate (boundary precision, bar #3)** — a signal earns a channel ONLY when every channel-captioned
   table that declares it agrees on exactly ONE role; a signal whose channel captions disagree is dropped (honest
   `unmapped` residual, never a guessed channel). This is what keeps the noisier 2021 AXI+ACE doc honest: it
   lists the same write-address signals under both `Write address channel signals` and per-interface `Manager /
   Memory Subordinate interface write channel signals` tables → 31 of 105 signals ambiguous → dropped.

## Measured (read-only + live rebuild; `generated/` gitignored)
- **2025 AXI (`ihi0022_l`)** — clean: 154 signals → 8 roles, 0 ambiguous. Live `validate`:
  `with_channel_membership: 5 (10 channel groups)`; `atomic_transaction` → {write request×12, write data×3, read
  data×4, write response×3} (multi-channel — a read-modify-write picture), `prefetch_transaction` → {write
  request×7, read data×1}, `writedeferrable` → {write request×9, write response×1}, `writezero` → {write
  request×6}, `narrow_transfer` → {read data×3}. AXI's `phase_membership` is 0 (its prose names no phase signal),
  so the channel grouping FILLS the `.2i` AXI-empty grouping exactly as intended.
- **2021 AXI+ACE (`ihi0022_h_c`)** — 74 clean, 31 ambiguous dropped (honest `unmapped`).
- **AXI-Stream / APB / AHB / SWD** — no `<role> channel signals` captions → empty surface (no fabrication).

## Why it cannot touch WIRE-BASED-100 or the `.isf` (proven, not assumed)
`channel_membership` is additive metadata; the ISF emitter (`isf_ir.rs`) lowers `tx.steps` ONLY (never `ports` /
`phase_membership` / `channel_membership`). A `git stash` HEAD-before-`.2m` vs HEAD-with-`.2m` AXI EvidenceIR
rebuild diff shows the ONLY changed field is `signal_channel_memberships` (154); all wire-gold surfaces
(`actor_signal_relations`/`signal_constraints`/`conditional_rules`/`signal_polarities`/`extracted_statements`)
byte-identical. Adapting the AXI intent with vs without `channel_membership` yields byte-identical `.isf` source.
So WIRE-BASED-100 is provably orthogonal and the emitted `.isf` is byte-identical. `kg-bench` 156/156 (no fixture
has channel captions); `run_ci.sh` GREEN (lib 1664).

Related: `[[transaction-phase-membership-vlm-vs-channel]]` (the `.2l` measurement that chose this lever),
`[[transaction-membership-subsection-scope]]` (`.2k`, the membership this groups), `[[transaction-capture-census]]`,
`[[project_kg_isf_transactions]]`, `[[feedback_scoring_rigor]]`, `[[feedback_no_hardcoded_chip_spec_names]]`,
`[[feedback_multi_strategy_best_wins]]`.
