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
  Status: `open` (`2026-10-21`; DESIGN; found by `.55.b.iv.a`)
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

- ID: `EXTRACTION-REPRESENTATION-GAPS.56`
  Status: `open` (`2026-10-21`; DESIGN; found by `.55.b.iv`)
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
