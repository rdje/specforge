# EXTRACTION-REPRESENTATION-GAPS — packet and memory leaves

- Part ID: `packet-and-memory-leaves`
- State: `active`

Open ERG leaves continue here, after `hint-and-notation-leaves` reached its rollover (`2026-10-21`): the reserved space
of captured packet layouts, the corpus memory's refresh, and every later ERG leaf. `.54.a` and `.55.b.iv.a` moved here
whole.

## Post-migration records

- ID: `EXTRACTION-REPRESENTATION-GAPS.54.a`
  Status: `superseded` (`2026-10-21`; CODE; opened by `.54`; measured, then split into `.54.a.i` and `.54.a.ii`)
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
  **Measured `2026-10-21` (no store moved).** `learn-priors` over the 31 current chains' IntentIR accepts all 31 and
  writes a candidate to scratch. Against the pinned store's 13 inputs, the counts are: actor 29 -> 24, semantic 83 ->
  64, modality 4, temporal 443 -> 1,140, table 0 -> 128, visual 1,251 -> 1,430, negative 11 -> 13, profiles 2 -> 3. An
  EvidenceIR dry-run of every chain under the candidate (`evidence --prior-memory`) moves 6 chains beyond the store
  path:
  - AXI loses the `AWTAGOP` hint (`.54`'s target), gains 8 hints and drops 1, nets 3 relations and 4 constraints, and
    changes 591 statements without moving a class.
  - AHB gains 14 relations; TMC gains 2 polarities and 3 declaration provenances; RISC-V debug loses 9 statements.
  - **USB 3.2 loses its four status-word fields and 2 reserved spans, and USB4 inter-domain 2 reserved spans.**

  The regression's cause: the candidate's 128 table-shape priors are new (the pinned store had none), and 123 of them
  rest on one document. USB 3.2's four status tables that the converter mis-typed `signal_description` teach `bit |
  description -> signal_description` (support 1). The prior then re-types the same document's untyped status-word
  tables through `effective_table_kind`, and the bit family's kind gate refuses them. A store learned from today's
  readings entrenches a converter guess the header shape contradicts. ADR 0036's loop would converge on it. So the
  refresh waits on a decision about which kinds a learned shape prior may assign (`.54.a.i`), then runs (`.54.a.ii`).

- ID: `EXTRACTION-REPRESENTATION-GAPS.54.a.i`
  Status: `done` (`2026-10-21`; DESIGN; split from `.54.a`; opened `.54.a.i.a`)
  Goal: **a learned table-shape prior cannot entrench a kind the table's own header contradicts.** 123 of the 128
  candidate table-shape priors rest on one document, and USB 3.2's converter-typed status tables teach `bit |
  description -> signal_description` to the document's untyped bit layouts. **Reproduce:** `learn-priors` over the 31
  chains to scratch, then `evidence --prior-memory <scratch>` on USB 3.2: `message_fields.bit_position_table` produces
  4 -> 0. Acceptance: decide from what a shape prior is for whether it needs cross-document support. Decide whether a
  header shape a reader recognizes (the bit-position vocabulary, the byte and dword layouts) outranks a learned kind,
  and whether the converter's own guess on such a header should be learned at all. Measure the decision on the
  AXI/AHB/TMC/RISC-V gains as well as the USB regressions; open the CODE leaf.
  **Measured `2026-10-21`** (EvidenceIR dry-runs of the 31 chains on scratch stores; nothing pinned moved):
  - **Most of the store is one document.** In the candidate, every one of the 64 semantic phrase priors rests on a
    single document, as do 1,137 of 1,140 temporal, 1,423 of 1,430 visual and 123 of 128 table priors. The pinned store
    is the same shape: all 83 semantic phrase priors, 431 of 443 temporal, 1,195 of 1,251 visual.
  - **A single-document prior replays the document's own past reading back to it.** `AWTAGOP`'s ready-like phrase
    prior was learned under the reader `.50` corrected, and relearning from today's IntentIR re-learns it, because the
    memory itself produced the hint. ADR 0036's loop converges with the stale reading inside the fixed point.
  - **A table-shape prior learns the converter's raw kind** (`harvest_table_shape_priors` reads SourceIR's
    `table_kind`), so USB 3.2's mistyped status tables teach `bit | description -> signal_description`.
  - **Removing all table priors leaves only RISC-V debug moving.** With no table priors, or only the 5 resting on two
    or more documents, only RISC-V debug moves (-9 statements). Every other drift came from single-document table
    priors.
  - **Leave-one-out still lets the bad guess travel.** Dropping each document's own support removes `AWTAGOP` and
    keeps AHB's duplicate suppression. But USB4 still loses 2 reserved spans: USB 3.2's guess travels to it.
  - **Two-document support holds.** Applying only priors with support from at least two documents, the pinned store
    changes one record on the 31 chains (`AWTAGOP`'s hint, gone). A store relearned from all 31 chains is
    byte-identical in EvidenceIR to the pinned store under the same rule.

  **Decided.** (1) A prior acts on a document only when at least two documents support it, in every family, at
  lookup. A single-document prior stays in the store as provenance, but it is that document's own past reading and
  carries no evidence. The rule counts documents and never reads the current one, so it stays identity-independent
  (ADR 0036 decision 1). It is also what makes the convergence loop of decision 4 meaningful: a fixed point can no
  longer hold a reading only the memory sustains. (2) A table whose header a layout reader recognizes (the bit,
  byte, descriptor, width-and-offset or dword layouts) is never re-typed by a learned kind. Under (1) USB's guess is
  inert, but two converters agreeing on a wrong guess would activate it. A new decision record is not written:
  `decision_records` is past its rollover on `bytes_each` and `lines_each`. The rule lives in this record, the book's
  corpus-memory page and a KM card. CODE: `.54.a.i.a`. The refresh (`.54.a.ii`) then changes no EvidenceIR record,
  so its re-qualification becomes a re-pin of an equivalent store.

- ID: `EXTRACTION-REPRESENTATION-GAPS.54.a.i.a`
  Status: `done` (`2026-10-21`; CODE; opened by `.54.a.i`; checklist below)
  Goal: **a prior acts only with two documents' support, and never re-types a layout header.** **Reproduce:** the
  pinned store recalls AXI `AWTAGOP` as ready-like from a single-document phrase prior learned before `.50`.
  Acceptance: every prior-memory lookup in `crates/specforge/src/ir/prior_memory.rs` (each family's
  `*_for`/`*_in_text`/kind lookups) applies only priors whose supporting documents number at least two. A table
  whose header a message-field or register layout reader recognizes takes no learned kind in `effective_table_kind`.
  Then measure every chain at every stage: the expected EvidenceIR effect is exactly AXI's `AWTAGOP` hint. Rebuild the
  affected chains, and keep the book's corpus-memory and handshake pages in lockstep.
  **Done `2026-10-21`.** `CorpusMemory::acting_priors()` keeps, in every family, the priors whose `support_count` (the
  learner's count of distinct supporting documents) reaches `MIN_ACTING_SUPPORT` (2). The evidence and semantic stage
  guidance and the validation reader load the store through it, so every lookup inherits the rule, and the store file
  keeps every prior. `effective_table_kind` gives no learned kind to a header `is_recognized_layout_header`
  recognizes: the bit, three-column bit, dword, register-field, byte-location, descriptor and width-and-offset
  layouts. All 31 chains were rebuilt stage by stage, and only AXI moved. EvidenceIR loses the `AWTAGOP` ready-like
  hint (56 -> 55), and SemanticIR and IntentIR drop that tag from its four interface records. Every `.isf` text is
  byte-identical.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.54.a.i.a` (CODE) — DONE `2026-10-21`

- [x] **REPRODUCE / MEASURE** — `.54.a.i`'s scratch stores: the pinned store, a candidate relearned from the 31 chains,
  and the candidate with no table priors, only two-document table priors, leave-one-out, two-document priors in every
  family, and empty. Each was an EvidenceIR dry-run of the 31 chains, then the rule applied through the real loader.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/prior_memory.rs`: every lookup acted on any prior, and
  nearly every prior rests on one document, so a store replays each document's own past readings. Table-shape priors
  learn the converter's raw kind (`learn_priors.rs` `harvest_table_shape_priors_from_source_ir`). New
  `MIN_ACTING_SUPPORT`, `acting_priors`, `store_with_phrase_support` (test support); `is_recognized_layout_header`,
  `dword_layout_header` (shared with the dword family) in `evidence.rs`; the three load sites. RED, each file restored
  byte-identically, every mutation failing a named test (6): one document acting, a family left unfiltered, and each of the evidence, semantic and validation loads skipping the rule, and a learned kind re-typing a layout.
- [x] **ADDRESSED (verified)** — all 31 chains rebuilt stage by stage: AXI alone moves, as measured beforehand.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `84135ba92`'s. `kg-bench` 158/158. `cargo fmt --check` and `cargo clippy --offline --workspace
  --all-targets -- -D warnings` clean. `cargo test --workspace --lib --exclude specforge-production-graph` 475 / 171 /
  1,908 passed and 1 failed: the semantic fixture's temporal prior rested on one document. Given two, it passes.
  `flow_census.json` rolled (+3 functions, +1 decision site, +14 helper edges); boundary unchanged.
- [x] **GENERICITY (ADR 0006, 0036)** — the rule counts supporting documents and never reads the current one; the
  layout guard reads the table's own header. No document, family or name.
- [x] **LOCKSTEP** — book `quality/corpus-memory.md` and `domain/handshake-semantics.md`; KM cards
  `corpus-memory-schema-7-neutral-scope` and `a-hint-role-counts-only-for-the-signal-that-bears-it` amended; ROADMAP
  R15f. No decision record (`decision_records` past rollover); the rule is recorded at `.54.a.i`.

- ID: `EXTRACTION-REPRESENTATION-GAPS.54.a.ii`
  Status: `open` (`2026-10-21`; CODE; split from `.54.a`; waits for `.54.a.i`)
  Goal: **the corpus memory's refresh runs as ADR 0036 declares it, under `.54.a.i`'s rule.** Acceptance (`.54.a`'s,
  unchanged): relearn from the 31 validated chains to a byte-stable store, replay every affected chain stage by stage,
  re-run the behavioral held-out qualification (`SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii`'s 51 attempts, provider-free)
  against the new store, re-pin `behavioral_qualification.json` and `behavioral_holdout_evidence.json` with the
  measured drift, and state what triggers the next refresh.
  **Ordered after the product leaves (`2026-10-21`).** Under `.54.a.i.a`'s rule the pinned store and one relearned
  from the 31 chains give byte-identical EvidenceIR, so the refresh's effect is the store's provenance, not a reading.
  Its cost is the held-out qualification: the previous run's 35 fresh-pipeline attempts took over two hours and keep
  about 7 GB under `.project-data/tmp/spec-to-intent-f-iii-heldout`. It stays open and runs after `.3.f.iii`.

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
  Status: `done` (`2026-10-21`; CODE; opened by `.55.b.iv.b`; checklist below)
  Goal: **a section headed by a declared field's name describes that field.** CHI C2C describes each message field in
  its own section (*B4.2.1.1 Message type, MsgType* … *B4.2.1.5 RsvdZero*), but its field records carry no
  description, and *RsvdZero*'s states that the field's bits are Reserved, with three obligations. **Reproduce:** CHI
  C2C `RsvdZero` (`message_field_0023`, no description) beside its section's statement. Acceptance: bind a section
  whose heading names exactly one declared field (the name alone or after a comma) to that field's records as its
  description. A field whose own description says its bits are Reserved becomes a reserved span carrying the
  description's obligations, with no keyword citation where the document defines none. Measure every chain.
  **Done `2026-10-21`.** `bind_field_sections` binds after convergence. `section_field_name` reads the heading past
  its outline number (the name alone, or after a comma), and the section's first statement must speak of a field.
  That gate was found by the first dry-run: USB's *8.4.2 Set Link Function*, *8.4.3 U2 Inactivity Timeout* and *9.6.5
  Interface* had bound to same-named fields, but they open on an LMP or a descriptor. Every record of the name gains
  `description_statement_ids`. A field whose description says, in its frame, that it is Reserved
  (`describes_reserved_field`) becomes a named reserved span whose `supporting_statement_ids` cite that statement.
  The obligations stay in the statement: the keyword reader's *and*-split would garble C2C's comma list of agent-modal
  clauses. Dry-run of the 31 chains: 29 identical. CHI C2C: 28 records under 8 names gain descriptions, and its 9
  *RsvdZero* records become reserved spans citing `statement_0470`, with no keyword. USB 3.2: 6 fields gain
  descriptions.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.55.b.iv.b.ii` (CODE) — DONE `2026-10-21`

- [x] **REPRODUCE / MEASURE** — every section anchor of the field-bearing chains whose heading names a declared
  field, with its statements' count and first sentence; CHI C2C's *RsvdZero* statement; the obligation reader's
  output on it.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: no reader joined a field's own section
  to its records, so C2C's fields carried no description and *RsvdZero*'s reserving sentence reached nothing. New
  `section_field_name`, `bind_field_sections`, `describes_reserved_field`,
  `MessageFieldRecord.description_statement_ids`, `ReservedSpanRecord.supporting_statement_ids`, and the description
  path of `reserved_span_surface`. RED, the file restored byte-identically after each (7 mutations, each failing a
  named test): no comma form, an unnumbered heading naming a field, any section describing a field, no description
  frame, a description never reserving, no cited statements, no binding.
- [x] **ADDRESSED (verified)** — dry-run as above; CHI C2C and USB 3.2 rebuilt stage by stage, every later stage
  and `.isf` text byte-identical.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `6b581ea39`'s. `kg-bench` 158/158. `cargo fmt --check` clean; `cargo clippy --offline --workspace
  --all-targets -- -D warnings` clean (after a test's tuple type was named); `cargo test --workspace --lib --exclude
  specforge-production-graph` 474 / 171 / 1,905 passed. `flow_census.json` rolled (+3 functions, +42 decision sites,
  +99 helper edges); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — the document's own outline numbering, its declared field names, the word *field* and
  its *This field … is Reserved* frame; no document, section or field name.
- [x] **LOCKSTEP** — book `pipeline/register-tables.md` (*A field's own section*); KM card
  `a-section-headed-by-a-declared-fields-name-describes-that-field` (new) and
  `a-layouts-reserved-space-is-a-typed-span-citing-the-keyword` amended. No production rule deleted or replaced.

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
  Status: `done` (`2026-10-21`; DOC; opened by `.56`)
  Goal: **every task-evidence collection whose live total warns takes the archive transition.** On `2026-10-21`
  CORPUS-COVERAGE's parts are at 85.9% of their `lines_total` target and no leaf owns the warning. Acceptance: once
  `.56.a` lands, census every contract under `doctrine/live_document_size/*task_evidence*.json`, apply the
  transition to each whose live total warns and that has all-closed parts, with a dated pointer in its tree. A tree
  whose warning has no all-closed part gets an owned leaf in that tree saying so.
  **Done `2026-10-21`.** Census of the eight contracts' live parts totals: only CORPUS-COVERAGE warns (lines 85.9%,
  bytes 70.4%); the others sit at 28.6-60.7% of bytes and 38.6-67.6% of lines. Its three parts that route no leaf
  (`refresh-ledger`, `recent-chronology`, `legacy-chronology`, unchanged since its `2026-08-10` migration) moved byte-
  exact to `docs/archive/tasks/corpus-coverage/`, sealed at `84135ba92`, under a new
  `corpus_task_evidence_archived_parts` surface (exact: 3 files). Its live total is now 245,685 bytes and 2,850 lines
  (69.6% of the lines target), and its contract reports no warning. The corpus root carries a dated pointer. ADR
  0045's rule that no tree is archived for capacity is unchanged; the book and KM card say so. A new decision record
  was not written because `decision_records` is at 59 of 74 files, and the rule lives in `.56`.

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

- **`.3.g.iii`'s measurement and decision** (`2026-10-21`; the leaf's record is in `program-and-census`).
  **Measured `2026-10-21`** (31 chains). 13 tables in three documents title two or more columns by declared or
  collective signal names, and they take five shapes:
  - **A member-named code column per signal.** AXI *A4.7 Physical address space encodings* gives `AxPAS` a distinct
    code for each of its six members. The converter put all seven rows in the header, so no reader sees a data row,
    and the table is a residual.
  - **One signal's bits as columns.** AHB *3-2 Transfer size encoding* splits `HSIZE[2] | HSIZE[1] | HSIZE[0]`, whose
    concatenation is `HSIZE`'s code; no enum comes of it today.
  - **Codes that repeat.** AXI *A4.4 Memory type encoding* gives each memory type an `ARCACHE` and an `AWCACHE` code,
    and both columns repeat codes (`0b1010` is both Write-Through No-Allocate and Write-Allocate on reads). Today only
    `ARCACHE` is minted, a 6-member reading of 12 rows.
  - **Members keyed jointly.** A4.7's `AxPROT[1]` + `AxNSE` (four of the members), APB's untitled `PNSE | PPROT[1] |
    Physical address space`, and AXI *A15.26*'s `SYSCOREQ` + `SYSCOACK`, which key the four coherency-connection
    states (across a continuation table). None is read.
  - **Value-combination and mapping rows.** AXI *A9.1*, *A12.15*, *A13.10*, *A13.11*, *A13.14*: a row states signal
    values and what they mean; all are residuals. Two tables are no encoding: *A6.5* lists signals that must match,
    and an untitled address-alignment bit matrix.

  Separately, 137 tables in 16 documents carry every row as a header row, A4.7 among them.

  **Decided.**
  - A code column that determines its member is that signal's enum: its codes are distinct across the rows that state
    one, and one signal's bit columns concatenate into one code (`.3.g.iii.a`, which reads an encoding table's
    header-only rows as data locally).
  - Jointly keyed members and value-combination rows are one shape: a row binding several signals' values to a member
    or a meaning. Its record and carriage are designed first (`.3.g.iii.b`).
  - A column whose codes repeat maps several members to one code, an alias rather than an enum. How to carry it, and
    whether today's lossy `ARCACHE` enum stands, is `.3.g.iii.c`'s.
  - The signal list and the bit matrix stay declared residuals.
  - The all-header-row tables are a converter defect every reader meets, and `.57` owns them.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.a`
  Status: `done` (`2026-10-21`; CODE; split from `.3.g.iii`; checklist below; opened `.58`)
  Goal: **a code column that determines its member is that signal's enum.** **Reproduce:** AXI *A4.7* (`AxPAS`,
  every row in the header) and AHB *3-2* (`HSIZE[2:0]` as three bit columns) mint no enum. Acceptance: in an encoding
  table, read each signal-titled column whose codes are distinct across the rows that state one as that signal's
  enum, named by the member column. Concatenate one signal's bit columns into its code. Read an encoding table whose
  rows all sit in its header from its second row on. Keep every enum minted today unchanged, and measure every chain
  and the wire golds.
  **Done `2026-10-21`.** `synthesize_signal_column_encodings` runs in the evidence convergence loop after the
  signal-anchor scan. Column titles may be declared signals or collective names (`collective_signal_name_surface`
  over the pass). `(table, enum)` pairs already minted are skipped. `is_one_bit_of_a_split_signal` stops the
  single-column reader from taking one bit of a split signal alone: AHB `HSIZE[2]` had published `BYTE = 0`,
  `HALFWORD = 0`, … and no symbol. Dry-run: 29 identical. AXI gains `AxPAS` (6 members), and AHB's `HSIZE` statements
  carry the concatenated codes. Both rebuilt stage by stage: each gains one symbol and loses one captured-region
  residual. Each `manager.isf` gains the type and the enum, FSMGen strict-clean with 0 diagnostics. **Found:**
  SemanticIR's identifier parser drops an enum member whose name starts with a digit, without a trace. That is 22
  member statements in four documents (AXI's whole `AxSIZE` enum, AHB's `4-word line` and `8-word line`, which also
  sizes `HSIZE` as 2 bits); `.58` owns it.

## Acceptance Checklist (enforced) — `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.a` (CODE) — DONE `2026-10-21`

- [x] **REPRODUCE / MEASURE** — `.3.g.iii`'s census of the 13 multi-signal-column tables; the statements each reader
  emits for A4.7, A4.4 and AHB 3-2; `AxPAS`'s collective record.
- [x] **ROOT CAUSE (WHY + WHERE)** — `crates/specforge/src/ir/evidence.rs`: the encoding readers take one value
  column per table and refuse a multi-key table, never read a header-only table's rows, and read one bit of a split
  signal as a code. New `synthesize_signal_column_encodings`, `is_one_bit_of_a_split_signal`, `encoding_member_name`
  (the existing sanitizer, extracted), and the loop's `signal_titles` and `minted` sets. RED, the file restored
  byte-identically after each (7 mutations, each failing the new test): one bit read alone, header-only rows unread,
  repeating codes minting, any table kind, minted tables re-read, bits least significant first, numbers naming
  members.
- [x] **ADDRESSED (verified)** — dry-run of all 31 chains and the stage-by-stage rebuild of AXI and AHB as above;
  FSMGen `--strict --check --json` clean on both `manager.isf`.
- [x] **NO REGRESSION** — `check_chain_currency.sh` 31 of 31 current. WIRE-BASED-100: all 11 seed reports
  byte-identical to `3ba14a0fc`'s. `kg-bench` 157/158 at first: `enum_code_collision_holdout_gold` pinned `TSIZE`'s
  exclusion on a split-bit table, which the misreading of one bit made collide. Its table now collides on a single
  `TSIZE` column, so the fixture keeps its rule, and kg-bench is 158/158. `cargo fmt --check`; `cargo clippy --offline
  --workspace --all-targets -- -D warnings` clean (after a type alias and `sort_by_key`). `cargo test --workspace
  --lib --exclude specforge-production-graph` 474 / 171 / 1,910 passed, the kg-bench test 1 failed for the same
  fixture, then passed. `flow_census.json` rolled (+3 functions, +30 decision sites, +64 helper edges, +3 semantic
  macros); boundary unchanged.
- [x] **GENERICITY (ADR 0006)** — column titles the document's declarations and collective rules define, the codes'
  own distinctness, and the converter's table kind; no document, table or signal name.
- [x] **LOCKSTEP** — book `pipeline/isf-enums.md` (*A table that codes its member in several signal-titled columns*,
  and the `AxSIZE` statement corrected; *A code that names two things names neither* dated); KM card
  `a-signal-titled-code-column-determining-its-member-is-an-enum` (new) and `enum-member-values-have-no-oracle`
  amended; kg-bench fixture `enum_code_collision_holdout_gold` re-based on a genuine collision. No production rule
  deleted or replaced.

- ID: `EXTRACTION-REPRESENTATION-GAPS.58`
  Status: `open` (`2026-10-21`; CODE; found by `.3.g.iii.a`)
  Goal: **an enum member whose document name starts with a digit reaches its symbol.** SemanticIR's
  `parse_enum_symbol_definition` reads a member name through `parse_identifier`, which refuses a leading digit, and
  drops the statement without a residual. On the 31 chains that is 22 member statements in four documents. AXI 8
  (`AxSIZE` `1_BYTE_PER_TRANSFER` … so no `AxSIZE` enum at all), APB 4, AHB 4 (`HSIZE`'s `4_WORD_LINE`,
  `8_WORD_LINE`, and so `HSIZE` sizes 2 bits, not its declared 3), CHI C2C 6. **Reproduce:**
  `generated/evidence_ir/ihi0022…/evidence_ir.json` *Enum AxSIZE 1_BYTE_PER_TRANSFER = 0.* and no `AxSIZE` in
  SemanticIR. Acceptance: decide a member name that keeps the document's words and is a valid identifier (an
  `.isf` name must lead with a letter) or a typed residual naming the dropped members. Apply it at the one seam, and
  measure every chain, the `.isf` texts and FSMGen strict.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.b`
  Status: `open` (`2026-10-21`; DESIGN; split from `.3.g.iii`)
  Goal: **a row binding several signals' or fields' values to a member or a meaning is typed.** **Reproduce:** APB's
  `PNSE | PPROT[1] | Physical address space`, AXI *A15.26* `State | SYSCOREQ | SYSCOACK`, and AXI *A13.14*'s legal MMU
  combinations are residuals. Acceptance: decide the record (the keys and values a row binds, and the member or
  meaning it names), its carrier to IntentIR, and whether a joint key over states reaches `protocol_states`. Measure
  the 9 tables of this shape on the 31 chains, and the untyped-table census's `signal_value_matrix` (22) and
  `field_value_encoding` (14, keyed by register fields) families it now owns. Then open the CODE leaf.

- ID: `EXTRACTION-REPRESENTATION-GAPS.3.g.iii.c`
  Status: `open` (`2026-10-21`; DESIGN; split from `.3.g.iii`)
  Goal: **a code column that maps several members to one code is carried as what it is.** **Reproduce:** AXI *A4.4*:
  `ARCACHE` `0b1010` is Write-Through No-Allocate and Write-Allocate on reads; today's `ARCACHE` enum keeps 6 of 12
  rows and `AWCACHE` none. Acceptance: decide whether a repeating column is an aliasing encoding (code -> members) or
  stays a residual. Decide whether today's 6-member enum is kept, re-read or withdrawn, and how the parenthesized
  alternates (*0b1110 ( 0b0110 )*) read. Measure AXI and the wire golds.

- ID: `EXTRACTION-REPRESENTATION-GAPS.57`
  Status: `open` (`2026-10-21`; DESIGN; found by `.3.g.iii`)
  Goal: **a table whose every row the converter put in its header has one header row.** 137 tables in 16 documents
  have more than one header row and no body row (the Cortex-A76 guide 37, AXI 26, USB 3.2 23, ADIv6 14), so every
  table reader sees no data. AXI *A4.7* is one. **Reproduce:** `generated/source_ir/ihi0022…/source_ir.json`
  `table_0061` (7 header rows, 0 body rows). Acceptance: census the 137 by shape (front matter, data tables, true
  multi-row headers), decide whether SourceIR normalizes them or each reader does, from what the converter states,
  and measure every chain.
