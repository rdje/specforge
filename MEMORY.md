# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SPEC-TO-INTENT-ALIGNMENT.9` (`.9a`/`.9b`/`.9d`/`.9e` complete). Open in `.9`: `.9c` (prose leg)
  and the population replay. Also open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`/`.7`. The last ten are tracking-only.
- Current state: product lane. `.9e` closed both halves: it withdrew two carried-forward findings, then repaired
  the two reviewed golds no carrier could emit (`table_0004|toc_non_contract`, `elem_00017|informational_non_contract`
  → `<region_id>|<family>`) and put the key law under the `RESIDUAL-ACTIONABILITY` gate as `residual_gold_law`.
  Metric-neutral by execution, not assertion: the evaluator reproduces `result_snapshot.json` byte-exactly and
  only four echoed gold strings move. The published current result was re-summarized in place under `.8b`'s
  mechanism (`replay_sha256` + `resummarized_by`). `.9e` also found and repaired a real breach: `.9b`'s
  published "clean fmt" claim was false — `cargo fmt --all --check` failed from `0a703cc0` — so full CI had been
  red for three commits.
- Next action: `SPEC-TO-INTENT-ALIGNMENT.9c` — carry the `SourceIR` content-element identity into `EvidenceIR`
  so a prose residual can name the region a reviewed prose cell anchors on (`elem_\d+` occurs zero times in a
  complete `evidence_ir.json`), then generalise the carrier to prose and reconcile the schema move across all 24
  proof-carrying chains. `.9c` now has a satisfiable gold (`elem_00017|informational_disclaimer`); before `.9e`
  it did not. The population replay follows the carrier work and is what republishes
  `current_result_snapshot.json`, which still predates `.9b`'s table carrier.
- In-flight uncommitted: none after this commit.
- Blockers: none. CONTAINMENT, re-derived: the bounded root was AT mandatory rollover (90.3%) and `.9e` rolled it
  to **79.8%** by removing eleven closed-lane `.6d.ii.f` verification rows, after checking every distinctive
  figure they carried already appears in the behavioral-qualification part. The **active task index is still
  115 of 128 lines (89.8%)** and rollover begins at 115.2, so the **next new leaf** must roll the index first —
  this is unchanged and still binding. Read every figure from
  `perl scripts/check_active_task_evidence.pl --contract doctrine/live_document_size/spec_to_intent_task_evidence.json --report`
  and `perl scripts/check_live_document_size.pl` rather than carrying it here. Editing a book chapter re-pins
  frozen census regions: do it by CONTENT anchor and recompute `sha256(line bytes + "\n")`, never by offset
  arithmetic, and keep a pinned quantity on its own pinned line. Owned, not fixed:
  `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`/`.7`; `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture
  producer is still signal-unsafe. Never run the fixture suite concurrently with the locality gate:
  `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILS if a fixture run deletes
  one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling with no rollover. `DOCTRINE_ENFORCEMENT.md` §10
  is in lockstep with the driver at 13 entries, but nothing checks that it stays so.
