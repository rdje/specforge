# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-PRESSURE-HEADROOM` — `.14a`/`.4a`/`.4b`/`.4c`/`.4e`/`.4f`/`.4d.i` done; open
  here: `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`. Also open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: `.4d` split on what each dimension MAXIMIZES — not maximum-versus-total: the producer
  computes all three `_each` dimensions as maxima, but `line_bytes_each` ranges over a per-LINE width that
  never accumulates, so it has no growth driver and no meaningful rollover. README had two lines over 96
  bytes in one bullet; reflowed at the narrowest
  line-count-preserving column (88; 87 spills to 6), so 118 lines / 4,637 bytes are UNCHANGED and the widest
  line is 94 (78.3%). `.4d.ii` (validation snapshot) is a Rust producer slice, not a doc edit.
- Next action: PRODUCT lane, on the director's `2026-08-31` steer off the containment plane —
  `KG-ISF-COMPLETENESS` / `KG-ISF-TRANSACTIONS` / `WIRE-BASED-100`. `.4d.ii`/`.15`/`.1` stay owned, not dropped.
- In-flight uncommitted: none after this commit.
- Blockers: none. **A size stop is a symptom — classify the growth driver first**, and before that, whether
  the dimension even HAS one. `.4a` removed a count because the aggregates already bound the resource; `.4c`
  kept the line bound because the population fits it; `.4e` chose partition over rollover because the WRITER
  SET was countable at one leaf. `.4d.i` adds: where a surface bounds both width and size, the width remedy
  SPENDS the other two budgets (+1 line, +2 bytes per break) and README had 2 lines / 3 bytes of headroom, so
  reflow must preserve the line count — the census pins README at L1, L30, L88-L104 by line number + sha256.
  **Losslessness is not route-safety**: `.4e` was byte-exact, passed all 12 doctrines, and still broke 14
  anchors; `.4f` repaired it and registered `SECTION-ANCHORS`, so keep moved headings as redirects. Prepending
  to `CHANGES.md` shifts the line-pinned `current_claim_census.jsonl` regions (re-anchor by CONTENT, never by
  offsets) plus one NEW row for the ledger head; editing `CHANGES.md`/`DEVELOPMENT_NOTES.md`/`MEMORY.md`
  stales `durability.artifacts` digests in three `claims.jsonl` claims and `README.md` a fourth, and a new
  fact card moves `fact_card_catalog.json` `planned_outputs` plus the `fact-card-catalog-count` assertion —
  refresh all of those last. Never infer ownership from a mention: read the owner's own `Status`.
  Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`;
  `CHANGES-LEDGER-ROLLOVER.4` (its `lines_each` crossed 80% here); `SCRATCH-RESIDUE-CONTAINMENT.4` — the
  `generated/` fixture producer is still signal-unsafe; never run the fixture suite concurrently with the
  locality gate. `durability.stale_check` is never executed by any gate (`.18`).
