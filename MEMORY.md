# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.9e` (the reviewed-label repair is its open half). Open in `.9`: `.9c`
  (prose leg) and the population replay. Also open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`/`.7`. The last ten are tracking-only.
- Current state: product lane. `.9b` shipped the table-region carrier, `.9d` repaired and gated the frozen
  residual contract (`RESIDUAL-ACTIONABILITY` is the 13th doctrine), and `.9e` re-derived the three statements
  those leaves handed forward for owner judgement and **withdrew two of them**: "unreachable by any structural
  rule" (a positional predicate reaches `elem_00017`; only the carrier's intent-bearing test cannot) and the
  index containment arithmetic (off by one, and `.9c` was never blocked). Do not re-inherit either.
- Next action: `SPEC-TO-INTENT-ALIGNMENT.9e`'s open half — repair the two reviewed residual labels. The four
  required-and-absent cells' only carrier is region-scoped and its key law is `<region_id>|<family>`;
  `table_0067|packed_page_table_entry` and `elem_00219|software_guidance` follow it, while
  `table_0004|toc_non_contract` and `elem_00017|informational_non_contract` are hand-written cause labels no
  non-circular projection can produce. Before normalising them, show the reviewed key is redundant with the
  cell's own region-and-family predicate (it is: `residual.semantic_ir.predicates` already pins both), and prove
  metric-neutrality at the pre-change baseline — both cells are unmet before and after, so no published ratio
  may move. `.9c` (prose leg) is blocked upstream: `EvidenceIR` carries no `SourceIR` content-element identity
  (`elem_\d+` occurs zero times in a complete `evidence_ir.json`), so it must carry that identity into
  `EvidenceIR` first and reconcile the schema move across all 24 proof-carrying chains.
- In-flight uncommitted: none after this commit.
- Blockers: none. CONTAINMENT, re-derived and now exact: the active task index is at **115 of 128 lines
  (89.8%)** after `.9e`'s row, and rollover begins at 115.2 — so the **next new leaf** is the one that must roll
  the index first, not the current set. The bounded root is at 88.3% of bytes / 85.2% of lines after a trim, and
  a root Verification Log row must stay under about 437 bytes. Read every figure from
  `perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/spec_to_intent_task_evidence.json --report`
  and `perl scripts/check_live_document_size.pl` rather than carrying it here — this session recorded a wrong
  containment number twice by restating instead of re-deriving. Owned, not fixed:
  `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12` (per-slice region re-pin is hand
  work with a silent-wrong-line hazard — do it by CONTENT digest, never by offset arithmetic) / `.13`;
  `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`/`.7`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture producer is still signal-unsafe. Never run the
  fixture suite concurrently with the locality gate: `check_persisted_artifact_paths.pl` walks every `*.json`
  under `generated/` and FAILS if a fixture run deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file
  ceiling with no rollover. `DOCTRINE_ENFORCEMENT.md` §10 is in lockstep with the driver at 13 entries, but
  nothing checks that it stays so.
