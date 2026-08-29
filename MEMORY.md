# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `CLAIM-VERIFICATION-ADOPTION.10`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`/`.9`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`. The last nine are tracking-only.
- Current state: `LIVE-DOCUMENT-PRESSURE-HEADROOM.2` is closed. `.2a` removed the 160-file task-plane cap
  behind a declared, checker-enforced exemption and deleted the second enforcer (`$MAX_TASKS`); `.2b` retired
  the single-use authority after observing its banked-authority refusal RED on a clean tree; `.2c` sharded
  `docs/TASK_TREE.md` by LIFECYCLE — the landing carries the 25 open trees and routes 150 trees to 3 derived
  parts under `docs/task-catalog/`, taking it 404 -> 295 lines. The bound now measures work in flight, not
  project lifetime, so the directive is delivered rather than relocated. `.2c` also FALSIFIED one of `.6a`'s
  carried values: `current_surfaces` 39, carried on a 29-revision trajectory in which it never moved, became
  40 when the parts surface was registered. A trajectory shows what has not happened, never what cannot — all
  three publishers withdrew it to `--report` in that commit (`.7` eighth instance).
- Next action: `CLAIM-VERIFICATION-ADOPTION.10` — re-adopt the upstream standard, which directive 17 asks be
  checked for updates and which has NOT been re-read since adoption. Read `2026-08-26`
  (`/Volumes/SSD/Documents/github/pgen/docs/CLAIM_VERIFICATION.md`, 245 lines, same volume, read-only): the
  local 205-line `CLAIM_VERIFICATION.md` is a restatement, not a copy, and it has dropped rules that would
  have caught this session's own defect. Missing: §2 "the taxonomy of checks that cannot fail" (a table of
  what each check class still permits, and "a check and the thing it checks must not share a parent");
  Leg 2's "two explanations that predict the same observation are not distinguished by more of that
  observation — you have illustrated, not tested" (exactly the false mechanism `.6a` published and `.6b`
  corrected); and "the cheapest oracle is your own project's history: check whether a case of the same shape
  was already adjudicated" (`.6`'s own commit body held the true mechanism). Carry those into the local
  standard and `TOOLBOX.md`, then re-check whether `.7`'s gate scope should widen at all — it may not need to.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/
  `.9`; `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture producer is still signal-unsafe, so the residue
  recurs. Never run the fixture suite concurrently with the locality gate:
  `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILS if a fixture run
  deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling with no rollover, so do not write a
  research record until `.4` releases that surface. `CHANGES.md` is 68% — about nine records of headroom.
