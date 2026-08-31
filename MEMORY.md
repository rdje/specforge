# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `LIVE-DOCUMENT-PRESSURE-HEADROOM` — `.14a`, `.4a` and `.4b` are done. Open in this tree:
  `.1`/`.3`/`.4c`/`.4d`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`. Also open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`;
  `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`.
- Current state: `.14a` sharded the alignment task-evidence index by lifecycle (ADR 0046); `.4a` removed the
  research-plane file count behind ADR 0045's gated exemption; `.4b` retired the consumed authority
  after observing its banked-refusal RED.
- Next action: `LIVE-DOCUMENT-PRESSURE-HEADROOM.4c` — one research record is **639 of a 640-line** per-file
  ceiling with health == ceiling, so a one-line correction to it is still refused; decide between a lossless
  split at a section boundary (now affordable, the file count is unbounded) and a re-derived per-file profile.
  Then `.4d` (validation snapshot 544/640, README 108/120) and `.15` (the closed-owner gate).
- In-flight uncommitted: none after this commit.
- Blockers: none. Never infer ownership from a mention: read the owner's own `Status`, and check
  that a leaf you close holds no assignment rows. Prepending to `CHANGES.md` or editing a book chapter moves
  pinned census regions: read the file BEFORE opening it for write, prove the prepend is pure, shift, then
  re-verify each `sha256(line bytes + "\n")` by CONTENT anchor, never by offset arithmetic. Owned, not fixed:
  `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.8`/`.9`/`.12`/`.13`; `CHANGES-LEDGER-ROLLOVER.4`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture producer is still signal-unsafe. Never run the
  fixture suite concurrently with the locality gate. `durability.stale_check` is never executed by any gate
  (`.18`), so no claim's staleness marker is load-bearing today.
