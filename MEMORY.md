# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.9c` (next). Open in `.9`: `.9c` (prose leg) and the population replay.
  Also open: `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`;
  `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`;
  `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`;
  `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`/`.7`. The last ten are
  tracking-only.
- Current state: product lane. `.9b` shipped the table-region carrier (coverage asked per kind — a visual region
  by `evidence_id`/`figure:<asset_id>`, a table region by its table id in `SemanticIr::cited_table_ids`), and
  `.9d` repaired and gated the frozen residual contract it found red. `RESIDUAL-ACTIONABILITY` is now the 13th
  registered doctrine, so a republished current result can no longer leave the frozen contract stale unobserved.
- Next action: `SPEC-TO-INTENT-ALIGNMENT.9c` — the prose leg. It is blocked upstream and the block is the work:
  `EvidenceIR` carries no `SourceIR` content-element identity (`elem_\d+` occurs zero times in a complete
  `evidence_ir.json`; `EvidenceSpan` has only `span_id`/page/line and `ExtractedStatement` only `statement_id`),
  so a prose residual cannot name the `elem_00219`/`elem_00017` region the reviewed cells anchor on. `.9c` must
  carry that identity into `EvidenceIR` first, then emit one residual per captured intent-bearing prose statement
  no canonical record cites, excluding the `source_fact` fallback class on the same "capture never established
  the region as intent-bearing" rule that already excludes `VisualAssetKind::Unknown` — including it would emit
  one residual per uncited paragraph (Arm Debug: 6,555 of 6,658, against 440 of 543 with the fallback excluded).
  An `EvidenceIR` schema move must be reconciled across all 24 proof-carrying chains.
- In-flight uncommitted: none after this commit.
- Blockers: none. Known and owned, not fixed: `informational_disclaimer` is unreachable by ANY structural rule
  (its statement carries the `source_fact` fallback class), and two of the four required-and-absent cells expect
  a residual label no non-circular projection can produce (`table_0004|toc_non_contract`,
  `elem_00017|informational_non_contract`, against the region law `<region_id>|<family>`); both need an owner
  decision, not an implementation, and are recorded in `docs/tasks/spec-to-intent-alignment/region-kind-generalisation.md`.
  Also owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12` (per-slice
  region re-pin is hand work with a silent-wrong-line hazard — do it by CONTENT digest, never by offset
  arithmetic) / `.13`; `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`/`.7`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture producer is still signal-unsafe. Never run the
  fixture suite concurrently with the locality gate: `check_persisted_artifact_paths.pl` walks every `*.json`
  under `generated/` and FAILS if a fixture run deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file
  ceiling with no rollover. CONTAINMENT, BLOCKING: the active task index is at 89.1% of its line target — one
  further leaf route crosses mandatory rollover, so `.9c` must roll the index BEFORE declaring a leaf; the
  bounded root is at 86.3% and a root Verification Log row must stay under about 437 bytes or it trips
  `line_bytes` alone. `DOCTRINE_ENFORCEMENT.md` §10 is now in lockstep with the driver at 13 entries, but
  nothing checks that it stays so. Read every live-document percentage from
  `perl scripts/check_live_document_size.pl` rather than carrying it here.
