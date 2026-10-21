# EXTRACTION-REPRESENTATION-GAPS — packet and memory leaves

- Part ID: `packet-and-memory-leaves`
- State: `active`

Open ERG leaves continue here, after `hint-and-notation-leaves` reached its rollover (`2026-10-21`): the reserved space
of captured packet layouts, the corpus memory's refresh, and every later ERG leaf. `.54.a` and `.55.b.iv.a` moved here
whole.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.54.a`
  Status: `open` (`2026-10-21`; CODE; opened by `.54`)
  Goal: **the corpus memory's refresh runs as ADR 0036 declares it.** The store pinned by the behavioral
  qualification (`a416cc8b…`) was learned from 13 IntentIR inputs, 8 of which are now legacy and refused by
  `learn-priors`, so its declared refresh cannot run and it still recalls readings the current reader withdraws (AXI
  `AWTAGOP` ready-like). **Reproduce:** `specforge learn-priors` over the store's `source_artifacts` fails on the
  first legacy input. **Impact:** the store cannot follow any reader change, and a checkout without it reads two
  records differently. Acceptance: re-declare the learning inputs as the IntentIR that validates today (the 31
  proof-carrying chains, or a stated subset), run ADR 0036's convergence loop to a byte-stable store, replay every
  affected chain and the behavioral holdout evidence, re-pin the hash in `behavioral_qualification.json` and the
  holdout evidence with the measured drift, and decide what triggers the next refresh. **Cost, measured
  `2026-10-21`:** `behavioral_qualification.json`'s frozen census pins the store with the replay library and entrypoint
  hashes, which still match today's files, so a refresh re-runs `SPEC-TO-INTENT-ALIGNMENT.6d.ii.f`'s 24-document
  behavioral replay under that contract before the hash moves.

- ID: `EXTRACTION-REPRESENTATION-GAPS.55.b.iv.a`
  Status: `done` (`2026-10-21`; CODE; split from `.55.b.iv`; opened `.55.b.iv.b`)
  Goal: **a layout's reserved space is typed and cites the document's *Reserved* definition.** USB 3.2's 47 and USB4
  inter-domain's 6 rows that captured layouts mark *Reserved* are padding, not records, so *A reserved bit, byte, word
  or field shall be set to zero by the sender and shall be ignored by the receiver* reaches none of them.
  **Reproduce:** USB 3.2 *Link Control Word Format* `3 | 3:19 | Reserved (R) .`. Acceptance: record each reserved row
  of every message-field family as a typed span of its container with the family's own position reading, cite the
  `keyword_definitions` record where the document defines the keyword (and on the named `bReserved`-style fields),
  keep every field record unchanged, and measure every chain.
  **Done `2026-10-21`.** EvidenceIR `reserved_spans` (`ReservedSpanRecord`, family `evidence.message_structure`):
  each message-field family records a padding row through a `reserved` sink where it skips one, at its own position
  reading. `reserved_span_surface` adds every field named as reserved (`bReserved`, linked by `field_id`), then cites
  the document's *Reserved* definition on every span. Dry-run of the 31 chains: 28 identical, every field record
  unchanged. USB 3.2 gains 38 spans (33 padding rows, 5 named fields, 22 containers) and USB4 inter-domain 5, all
  citing `keyword_0005`; the CoreSight TMC gains 1 uncited span (*Reserved, SBZ*). The reserved rows still unread are
  reconciled: USB 3.2's 47 rows in field-bearing tables are 31 spans, 2 named `wReserved`, 8 value sub-rows, 5 bits
  nested inside a field and 1 unit-suffixed offset; USB4's 6 are 3 spans and 3 value sub-rows. Those marks, and CHI
  C2C's abbreviated `RsvdZero`, are `.55.b.iv.b`'s.

- ID: `EXTRACTION-REPRESENTATION-GAPS.55.b.iv.b`
  Status: `done` (`2026-10-21`; DESIGN; found by `.55.b.iv.a`; opened `.55.b.iv.b.i`, `.55.b.iv.b.ii`)
  Goal: **the reserved marks no message-field family reads are typed or declared.** `.55.b.iv.a` types the reserved
  rows a layout places. Four kinds of mark remain. USB 3.2 has 8 value sub-rows (*0000b | Reserved*, *0BH - FFH |
  Reserved*) and USB4 inter-domain 3; each reserves a code value of the field above, which the Reserved definition
  also governs (*Reserved field values shall not be sent*), and no family reads a field's values. USB 3.2 has 5 bits
  nested inside a field's row (*| | 6:2 | Reserved.*), where no family reads a sub-layout, named or reserved. One USB
  3.2 row writes its offset with a unit (*80 | 0:16b | Reserved .*), so no row is read there. CHI C2C names 9 fields
  `RsvdZero`, an abbreviation, not the word, in a document that defines no keyword. **Reproduce:** USB 3.2
  `table_0102` (*| | 0000b | Reserved*) and `table_0103` (*| | 6:2 | Reserved.*). Acceptance: decide for each kind
  whether it becomes a typed record (a reserved value of its field, a reserved span of a sub-layout) or a declared
  residual, from what the document states rather than from a vocabulary list (ADR 0006), and measure every chain.
  **Measured `2026-10-21`.** The value sub-rows and nested bits are one structure. After a field's row, a
  layout restates a sub-header in the description columns: *Value | Type of LMP* or *Bits | Description* (*Bit |
  Encoding* in a descriptor). The rows under it belong to that field. A `Value` sub-table lists its stated code
  values and meanings (*0001b | Set Link Function*); a `Bits` sub-table lays out its own bits (*1 |
  Force_LinkPM_Accept*, *6:2 | Reserved.*). In the field-bearing tables, USB 3.2 restates 5 value and 5 bit
  sub-headers (37 and 17 rows), and USB4 inter-domain 2 and 1 (7 and 5 rows). The families skip them all, named
  values and sub-fields included, so the reserved ones are the visible part of a larger loss. CHI C2C does not use
  `Rsvd` as an abbreviation of a keyword. It defines the field itself in a section headed by its name (*B4.2.1.5
  RsvdZero*: *the use of the bits is Reserved and a Transmitter must set the field value to zero, a Receiver must
  ignore the field and an intermediate agent must forward them unchanged*). It has about ten such sections, one per
  field, and no reader binds them to the field records, which carry no description. The unit-suffixed offset (*80 |
  0:16b*) is ambiguous: USB writes `b` for a binary value, and `16b` is no binary number.
  **Decided.** (1) A sub-header restated after a field row opens that field's sub-table. Its `Value` rows become the
  field's stated values (value literal, meaning); a meaning that opens with *Reserved* marks a reserved value, which
  cites the keyword. Its `Bits` rows become the field's own layout: named sub-fields, and reserved bits as reserved
  spans of the field. This is `.55.b.iv.b.i`. (2) A section whose heading names a declared field describes that field.
  A field whose own description says its bits are Reserved is reserved space, and carries the obligations the
  description states, with no keyword needed. This is `.55.b.iv.b.ii`. (3) `0:16b` stays a declared residual: a bit
  position is never guessed from a cell its own notation makes ambiguous.

- ID: `EXTRACTION-REPRESENTATION-GAPS.55.b.iv.b.i`
  Status: `done` (`2026-10-21`; CODE; opened by `.55.b.iv.b`; checklist below)
  Goal: **a field's restated sub-table types its values and its own bits.** USB 3.2's *Link Management Packet Subtype
  Field* lists LMP subtypes under *Value | Type of LMP*, and *Set Link Function* lays out its bits under *Bits |
  Description*. No family reads either. **Reproduce:** USB 3.2 `table_0102` and `table_0103`. Acceptance: in every
  message-field family whose rows can carry one, read the sub-header restated after a field row and assign the
  following rows to that field until the next field row. `Value` rows become the field's stated values (a Reserved
  meaning marks a reserved value citing the keyword); `Bits` rows become the field's sub-fields and reserved spans,
  at the parent's position reading. Keep every existing field record's identity, and measure every chain.
  **Done `2026-10-21`.** `MessageFieldRecord` gains `values` (`MessageFieldValue`) and `sub_fields`
  (`MessageSubField`, bit ranges relative to the field), and `ReservedSpanRecord` gains `within_field_id`. The
  width-and-offset, descriptor and dword families collect each field row's sub-rows (`sub_row_cells`,
  `raw_column_index`), and `read_field_sub_table` reads them from the restated sub-header (`field_sub_table_header`).
  `cite_reserved_values` cites the keyword on reserved values, and `reserved_span_surface` adds reserved sub-fields
  as spans within their field. A bit range is kept relative to the field, as the document states it: a byte-located
  parent has no bit position to add it to. **Found and fixed:** the dword family nested every empty-dword sub-row as a
  field, so USB4's *Error Response* codes (*1 | Unknown Packet Type*, *3 | Not Supported*, *4 | Not Ready*) were three
  bogus fields. Under a `Value` sub-header they are its values. Dry-run of the 31 chains: 29 identical. USB 3.2 keeps
  its 209 fields; 10 gain 33 values (7 reserved, all cited) and 10 sub-fields (3 named), and the reserved spans go
  38 -> 42 (4 within fields). USB4 inter-domain goes 56 -> 53 fields, and *Error Response* gains 4 values. Declared
  residuals: a fused *Value Meaning* sub-header (USB4 `table_0044`); a bit cell fusing two rows (USB 3.2 `table_0184`,
  *0 1*); the values of a sub-field (*00b Reserved* under *LatencyScale*), which stay in its description; the two-column
  bit family's *| Value* rows.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.55.b.iv.b.i` (CODE) — DONE `2026-10-21`

- [x] **REPRODUCE / MEASURE** — every sub-row of USB 3.2's and USB4's field-bearing tables printed with its table
  header, before any code; the restated sub-header shapes (*Value | …*, *Bits | Description*, *Bit | Encoding*, fused
  *Bits Description*, fused *Value Meaning*); and USB4's three nested fields that were code values.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: the width-and-offset and byte-location
  families dropped any row with no field cells, and `extract_dword_layout_fields` nested every empty-dword row without
  reading its sub-header. New `MessageFieldValue`, `MessageSubField`, `FieldSubTable`, `field_sub_table_header`,
  `sub_row_cells`, `raw_column_index`, `read_field_sub_table`, `cite_reserved_values`, `within_field_id`; the families'
  `sub_rows`. RED, the file restored byte-identically after each mutation, every one failing a named test. 12 of the
  first 13 were killed: the value, bit and fused headers; spanning prose as a value; the continuation; a led row as a
  sub-row; raw equal to effective index; descriptor sub-rows dropped; width-offset sub-rows dropped; a value row
  nesting a field; no within-field span; every value cited. The 13th, an ellipsis guard, was redundant with the
  distinct-cells guard and was removed. An empty value accepted is killed by its own case.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains as above; USB 3.2 and USB4 inter-domain rebuilt
  stage by stage, every later stage and `.isf` text byte-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `531ed21f7`'s. `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace
  --all-targets -- -D warnings` clean (after a test's tuple type was named); `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 171 / 1,904 passed. `flow_census.json` rolled (+5 functions, +23 decision sites,
  +80 helper edges); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — the sub-header words the layout itself restates (*Value*, *Bit(s)*), the table's
  own cells and the document's keyword definition; no document, container or field name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` (*A field's restated sub-table*, and the width-and-offset and
  dword paragraphs corrected); KM card `a-fields-restated-sub-table-types-its-values-and-bits` (new) and
  `a-dword-packet-layout-reaches-message-field-records` amended. No production rule deleted or replaced.

- ID: `EXTRACTION-REPRESENTATION-GAPS.55.b.iv.b.ii`
  Status: `open` (`2026-10-21`; CODE; opened by `.55.b.iv.b`)
  Goal: **a section headed by a declared field's name describes that field.** CHI C2C describes each message field in
  its own section (*B4.2.1.1 Message type, MsgType* … *B4.2.1.5 RsvdZero*), but its field records carry no
  description, and *RsvdZero*'s states that the field's bits are Reserved, with three obligations. **Reproduce:** CHI
  C2C `RsvdZero` (`message_field_0023`, no description) beside its section's statement. Acceptance: bind a section
  whose heading names exactly one declared field (the name alone or after a comma) to that field's records as its
  description. A field whose own description says its bits are Reserved becomes a reserved span carrying the
  description's obligations, with no keyword citation where the document defines none. Measure every chain.

- ID: `EXTRACTION-REPRESENTATION-GAPS.56`
  Status: `done` (`2026-10-21`; DESIGN; found by `.55.b.iv`; opened `.56.a`, `.56.b`)
  Goal: **the ERG task-evidence collection stays under its total-bytes bound.** The contract
  (`doctrine/live_document_size/extraction_representation_gaps_task_evidence.json`, `limits.parts`) bounds every
  part's bytes together at a 786,432-byte health target with an 80% warning; the 17 parts hold about 656 KB
  (83.4%) on `2026-10-21`, 82.9% before this slice, and no leaf owns the warning. The ledger grows by one leaf
  record and one checklist per slice (about 3-5 KiB), so it reaches the 90% band (707,789) in roughly 13 slices.
  **Reproduce:** `perl scripts/check_active_task_evidence.pl --contract
  doctrine/live_document_size/extraction_representation_gaps_task_evidence.json --check` (*bytes_total is at or
  above warning*). Acceptance: decide from what the parts are for — the closed records and checklists of the
  legacy and rolled-over parts are history, so seal them into a dated archive capsule the way the roadmap seals its
  rolled-over direction, or derive the bound from the part quantum (ADR 0029) — leave measured headroom, and keep
  every leaf route and claim region resolving.
  **Measured `2026-10-21`** (after `.55.b.iv.a`): the 17 parts hold 661,007 bytes, 84.1% of the 786,432 target and
  56.0% of the 1,179,648 ceiling, which is the portable cap `check_active_task_evidence.pl` enforces on every tree.
  Acceptance checklists are 261,379 bytes (39.5%), the largest term a CODE slice adds. Eight parts route only closed
  leaves: `table-census-leaves`, `hint-and-notation-leaves`, `unread-intent-leaves`, `default-and-assumption-leaves`,
  `environment-assumption-leaves`, the two `acceptance-checklists` halves and `changelog`. Together they hold
  327,205 bytes, 49.5% of the live total. 37 of the 173 routed leaves are open, and the other nine parts hold them.
  Among the other seven task-evidence trees, CORPUS-COVERAGE's parts warn on `lines_total` (85.9%) and no leaf owns
  it; the rest sit at 28.6-60.7% of bytes.
  **Decided.** The bound is right and stays; the remedy is to shrink the live resource. LDPH `.14e` removed the part
  count and kept `bytes_total` and `lines_total` as the bound on what a tree holds live. ADR 0032 found that adding
  parts never reduces that total, and raising the target past the portable cap is not available. The adoption guide
  (§6) names the remedy for this shape: move sealed history, at whole-record boundaries, to a declared archive
  terminal and leave one checked live pointer. So a part whose every routed leaf is closed may move to the tree's
  archive directory. It moves byte-exact, bound to its sealing commit and blob (the checker's existing `sealed`
  proof), and counts toward the tree's archive surface instead of the live parts total. Its leaves route to the
  archived path, and every reference to it follows. No record is edited, merged or deleted; amending a sealed leaf
  takes a new record in a live part, as the post-migration records already do. Applied today, the eight parts leave
  333,802 live bytes (42.4%), and the transition repeats whenever the total warns again. Splitting checklists from
  records inside a live part is declined: it would cut leaves' records apart, and whole closed parts already carry
  half the total. Implementation: `.56.a`; the corpus tree's adoption: `.56.b`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.56.a`
  Status: `done` (`2026-10-21`; CODE; opened by `.56`; checklist below)
  Goal: **a task-evidence part whose every leaf is closed moves to the archive terminal and out of the live total.**
  Acceptance: `scripts/check_active_task_evidence.pl` gains an archived part. It lives under `archive_directory`, is
  sealed by commit and blob and checked byte-for-byte against that commit, and is excluded from the live `parts`
  totals. Every leaf it routes must be closed, it may not be the current frontier's part, and its source regions
  still resolve once each. The route catalog and index point at the archived path. The tree's archive surface admits
  it under a consumed `ceiling_increase_authorities` record. `--self-test` covers the green case and each refusal
  (an open leaf, the frontier's part, a byte drift, a missing authority). Then move ERG's eight all-closed parts,
  following every reference to their paths (KM cards, the manifest, routes, the contract, `surfaces.jsonl`), copy ->
  verify -> use -> delete. Measure the live total after the move, and keep the book's `reference/live-docs.md` in
  lockstep.
  **Done `2026-10-21`.** The checker accepts `state: archived` with `archived_from`, `sealed_commit` and
  `git_blob`. `validate_archived_part` refuses an archived part whose live copy remains, which routes an open leaf,
  or which is the frontier's part. The seal is read at `archived_from`, and the part leaves the live totals
  (`archived_part_metrics`). The index marks it *— archived*. Self-test 83 -> 91. ERG's eight all-closed parts moved
  under `docs/archive/tasks/extraction-representation-gaps/`, sealed at `4a3d61c5a`: live 17 parts / 661,007 bytes
  -> 9 parts / 337,798 bytes (43.0%); 327,205 bytes archived. The `bytes_total` warning is gone. The parts surface
  drops their targets, and a new `erg_task_evidence_archived_parts` archive-terminal collection holds them
  (exact: 8 files, totals `8 ×` the per-file bounds). The gap-owner check (`trajectory.rs`
  `task_evidence_declares`) also reads the archive directory. 24 KM cards' evidence paths follow. The census drops
  4 evidence records for moved files and gains one for the parts surface's new first member (`census-leaves.md`).
  The root's routing paragraph now sends every new leaf to `packet-and-memory-leaves`.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.56.a` (CODE) — DONE `2026-10-21`

- [x] **REPRODUCE / MEASURE** — the contract's part totals before (661,007 bytes, 84.1% of target); each part's
  routed leaves by lifecycle, checklist bytes and size; the eight all-closed parts (327,205 bytes); every reference
  to their paths (24 KM cards; the contract, manifest, routes, index and registry); every consumer that reads part
  directories (`trajectory.rs` `task_evidence_declares`, no gap owner in the eight parts today).
- [x] **ROOT CAUSE (WHY + WHERE)** — `scripts/check_active_task_evidence.pl` had no part state outside the live
  totals, so closed history counted against the bound forever; `crates/specforge/src/ir/trajectory.rs` read only the
  collection directory. New `validate_archived_part`, the archived arms of the schema, seal, totals and index, and
  `archive_fixture_activity_part`. RED, each file restored byte-identically after each mutation, every one failing
  a named case. In the checker, 8: archived refused as a state, an open leaf allowed, a live copy allowed, the
  frontier's part allowed, archived bytes counted live, the seal read at the archive path, the archive placed in
  the collection directory, `archived_from` allowed on a live part. In Rust, 2: no archive directory read, the
  archive read under the wrong root.
- [x] **ADDRESSED (verified)** — `check_active_task_evidence.pl --contract …extraction_representation_gaps… --check`
  valid with the `bytes_total` warning gone (live 337,798 bytes, 43.0%); the eight archived files byte-identical to
  `4a3d61c5a`'s live copies (checked before each live copy was deleted); `check_live_document_size.pl` 1,280 files
  over 83 surfaces; `check_current_claim_census.pl` and `check_book_quantitative_claims.pl` pass.
- [x] **NO REGRESSION** — `check_active_task_evidence.pl --self-test` 91/91; `check_task_evidence_contracts.sh` green
  for all eight contracts; `check_chain_currency.sh` 31 of 31 current; WIRE-BASED-100 all 11 seed reports
  byte-identical to `a166ef4d8`'s; `kg-bench` 158/158; `cargo fmt --check` clean; `cargo clippy --offline --workspace
  --all-targets -- -D warnings` clean; `cargo test --workspace --lib --exclude specforge-production-graph` 474 / 171 /
  1,900 passed; `check_production_genericity_flow.sh` unchanged.
- [x] **GENERICITY (ADR 0006)** — a part's routed lifecycles, its seal and its directory; no tree, part or leaf
  name in the checker, and the Rust lookup derives the archive directory from the root's own name.
- [x] **LOCKSTEP** — book `reference/live-docs.md` (*A part whose leaves are all closed is archived, not counted*);
  KM card `a-task-evidence-part-whose-leaves-are-all-closed-is-archived` (new); the root routing paragraph and
  `MEMORY.md`. No record edited, merged or deleted.

- ID: `EXTRACTION-REPRESENTATION-GAPS.56.b`
  Status: `open` (`2026-10-21`; DOC; opened by `.56`)
  Goal: **every task-evidence collection whose live total warns takes the archive transition.** On `2026-10-21`
  CORPUS-COVERAGE's parts are at 85.9% of their `lines_total` target and no leaf owns the warning. Acceptance: once
  `.56.a` lands, census every contract under `doctrine/live_document_size/*task_evidence*.json`, apply the
  transition to each whose live total warns and that has all-closed parts, with a dated pointer in its tree. A tree
  whose warning has no all-closed part gets an owned leaf in that tree saying so.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.55.b.iv.a` (CODE) — DONE `2026-10-21`

- [x] **REPRODUCE / MEASURE** — every row of the 31 chains' tables whose cell opens with *Reserved*, by header shape,
  before any code; then each reserved row of a field-bearing table matched against the spans one by one (USB 3.2 47,
  USB4 inter-domain 6), and every field record whose name holds *reserv*/*rsvd*.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: each message-field family dropped a
  *Reserved* row as padding and kept nothing of it, so the keyword's definition reached no bit. New
  `ReservedSpanRecord`, `EvidenceIr.reserved_spans`, `is_reserved_led` (the one test the four name functions now
  share), `is_reserved_field_name`, `record_reserved_span`, `reserved_span_surface`; a `reserved` sink on the five
  table families. RED, the file restored byte-identically after each (12 mutations, each failing a named test): the
  container, explicit-name, head-and-rest, two-column, width-offset, dword and byte-location padding, the named
  fields, the citation, the prefix length, and the merge rule both ways.
- [x] **ADDRESSED (verified)** — evidence dry-run of all 31 chains: 28 identical; USB 3.2 +38 spans, USB4
  inter-domain +5, the CoreSight TMC +1; every message-field record unchanged. All 31 rebuilt stage by stage (the
  ruleset hash moved with the new field): every later stage and `.isf` text byte-identical, and
  `rebuild_stage_cascade.sh --write` re-sealed every chain content-identically.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports byte-identical to `e2bc8aaca`'s (temp paths masked). `kg-bench`
  158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace --all-targets -- -D warnings` clean; `cargo
  test --workspace --lib --exclude specforge-production-graph` 474 / 171 / 1,899 passed and 1 failed, the proof test
  counting 47 EvidenceIR fields; moved to 48, it passes. `flow_census.json` rolled (+4 functions, +20 decision sites, +121 helper edges) and its boundary
  moved for the new producer (rule roots 132, grammar declassifiers 73).
- [x] **GENERICITY (ADR 0006)** — the word *Reserved* a row opens with, the field's own name and the document's own
  keyword definition; no document, container or field name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` (*Reserved space is a typed span*),
  `pipeline/presence-and-assumptions.md`, `pipeline/isf-adapter.md`, `quality/validation.md`,
  `source-to-intent-contract.md`; KM card `a-layouts-reserved-space-is-a-typed-span-citing-the-keyword` (new) and
  `a-keyword-definition-is-typed-with-the-obligations-it-states` amended; ROADMAP states nothing this falsifies. No
  production rule deleted or replaced.
