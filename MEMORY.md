# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-PRESSURE-HEADROOM` — `.14a`, `.4a`, `.4b` and `.4c` are done. Open in this tree:
  `.1`/`.3`/`.4d`/`.4e`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`. Also open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: `.4c` partitioned the 639/640-line genericity audit at its chronology seam into
  `docs/research/production-genericity-qualification-results.md` (audit now 467/640); losslessness was proved
  against HEAD, not by reading the diff. Its own ledger record then crossed `CHANGES.md`'s 90% signal, so
  `CHANGES-LEDGER-ROLLOVER.6` rolled the ledger in the same commit (root now 69.2% / 74.0%). Fact card
  `research-record-size-profile` records the population profile so it is not re-derived.
- Next action: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4e` — the split RELOCATED the research maximum onto
  `generic-enum-conflation-measurement.md` at **559/640 (87.3%)**, and unlike the audit that record has a live
  writer (active `KG-ISF-COMPLETENESS.5`, last append `5c2fe11f`). Decide its lifecycle remedy before that lane's
  next measurement lands. Then `.4d` (validation snapshot 544/640, README 108/120) and `.15` (closed-owner gate).
- In-flight uncommitted: none after this commit.
- Blockers: none. **A size stop is a symptom — classify the growth driver before choosing a remedy.** Partition,
  rollover and re-profile are not interchangeable: `.4a` removed a count because the aggregates already bound the
  resource, `.4c` kept a line bound because the population fits it (mean 172, p95 372, only 2 of 63 above 80%) and
  split the record because its writer set was unbounded. Prove any partition mechanically — moved blocks
  byte-identical and in order, retained prefix/suffix byte-identical, multiset difference empty — then inspect
  routes separately, since a content-preserving move still breaks anchor deep-links and "the findings above"
  back-references. Prepending to `CHANGES.md` shifts 22 line-pinned census regions in
  `doctrine/claim_verification/current_claim_census.jsonl`: re-anchor each by CONTENT and re-verify its
  `sha256(line bytes + "\n")`, never by offset arithmetic. Editing `CHANGES.md`/`DEVELOPMENT_NOTES.md`/`MEMORY.md`
  also stales whole-file `durability.artifacts` digests in three `claims.jsonl` claims; refresh them last, after
  every other edit. Ownership citations reach past the live-size gate: never infer ownership from a mention, read
  the owner's own `Status`. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`;
  `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`; `CHANGES-LEDGER-ROLLOVER.4`; `SCRATCH-RESIDUE-CONTAINMENT.4` —
  the `generated/` fixture producer is still signal-unsafe. Never run the fixture suite concurrently with the
  locality gate. `durability.stale_check` is never executed by any gate (`.18`), so no claim's staleness marker is
  load-bearing today.
