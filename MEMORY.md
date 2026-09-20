# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> A POINTER, not a briefing. It describes *now*: active unit, next action, in-flight work, blockers.
> Doctrine and gates are `COMMIT.md` and `DOCTRINE_ENFORCEMENT.md`, bootstrap order is `AGENTS.md`, hazards
> are the Knowledge Map, and per-tree state is that tree's own `## Current Frontier` — a session reaches all
> of them before this file. Anything else here is in the wrong layer (`MEMORY_ARCHITECTURE.md` §6), and the
> ~50-line cap is a ceiling, not a budget. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: **`LIVE-DOCUMENT-PRESSURE-HEADROOM.37`** SHIPPED `2026-09-20` — the bounded root went
  **229 -> 166 lines (89.5% -> 64.8%)**, bytes 82.4% -> 33.6%, widest line 80.3% -> 22.1%; all three root
  warnings cleared and a closure now costs the root **one owner-registry line**.
- **Attribute growth per section on the BINDING axis before choosing a cut** — the root's writer was
  `## Current Frontier` (+39 of +83 lines) while on BYTES the log windows led (+7,007 of +11,656). Both
  `.34` and `.37` were decided this way; the frontier is now overwritten and the log tables carry no rows.
- **A migrated tree cannot record a closure until it has a writable chronology.** The sealed
  `verification-and-chronology` part is immutable, so post-seal rows piled into the root. New `active` part
  `current-chronology.md` is their home; the mechanics of adding a part are in
  [[bounded-task-root-accretes-in-its-frontier]].
- **`.20`'s published exposure of "seven" was an under-count: it is 14 of 21 verification rows and 14 of 19
  commit rows.** The six missed (`.31` `.32` `.33` `.36b` `.36b.1` `.36c`) were the six most recent. A
  leaf-level check cannot decide this — **test the ROW's content tokens**; that test refused a row a careful
  reading had cleared (the second `.30` row carries `` `pending` ``, absent from its sealed counterpart).
- **`.20` CLOSED `2026-09-20`** — three routing rules in `COMMIT.md`, applied twice; its forward half is
  done and **the backlog is not** (75 of 239 `###` headings still lead with a work-unit id). It answered
  `.34`: `live-docs.md` is the SAME mechanism, so `.34` executes **rule 3 — move the ACCRETING section**.
- **`.34` CLOSED `2026-09-20`** — rule 3's first application to a chapter. The block to move was chosen by
  attributing growth across 40 revisions (**+24,085 of +55,799**), and it was a CHRONOLOGY inside a
  doctrine chapter: `live-docs.md` **121,517 -> 75,698 B (92.7% -> 57.8%)**, `shipped_behavior`'s byte
  warning cleared outright, 183 of 183 pinned regions re-derived at their new locations.
- Next action: **`.34a`** — `evidenceir.md` is now the book maximum at **104,450 B (79.7%)** with its
  *Closed task trees* section at **79.4%** of it; make the same cut before a commit is blocked. Then
  **`.14e`** (part collection 13 of 16 files = **81.25%**).
- Then **`SIGNAL-CATALOG-CAPTURE-GAP.5`** (14 of 78 artifacts have no validation report, so no
  `document_class`), **`KG-ISF-COMPLETENESS.5.v`** (census first), or **`EXTRACTION-QUALITY-GAUGE.3k.9`**.
- **Every governed edit pays a claim-digest bill** — a four-registry fixpoint, and a blind walker
  destroys line-range pins. The whole procedure is in [[bounded-task-root-accretes-in-its-frontier]].
- `cargo test -p specforge --lib <filter>` reports **0 passed, 474 filtered out** for an `ir/` change and
  exits 0. Use `--workspace --lib --exclude specforge-production-graph` (COMMIT.md step 8).
- `LEGACY-SOURCE-RECLASSIFICATION.0`+`.1`: **the bundle, not the label, is the binding constraint.**
- **After every push, watch the hosted run to a verdict** (directive `2026-09-20`); a stalled run is not a pass.
- **A parked leaf keeps its blocker until someone re-derives it.** **Do not restate a handed-down number.**
- **Measure the PASS, not the function** (`.2h.2`); **coverage is a PROVENANCE question** (`.5c`);
  **characterise a difference before counting it** (`.11`); **read the ROOT CAUSE, not the symptom** (`.5`).
- `.36d` owns the claim registry lifecycle at the 80% record band. **Never carry the residual as a number.**
- `BOUNDED-DECISION-PROVIDER`: **provider REJECTED** (ADR 0051); do not buy `TYPESAFE_API_KEY`.
- Corpus 27/27 current. A slow gate is **measured, not attributed**: `probe_exec_assessment_latency.sh`.
- In-flight uncommitted: none; no background job outstanding.
- Blockers: none.
