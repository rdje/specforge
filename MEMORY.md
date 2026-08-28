# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `SOURCE-IR-REPRODUCIBILITY.16`, then `.7`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`/`.14`/`.16`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CLAIM-VERIFICATION-ADOPTION.1a`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`. The last six are tracking-only.
- Current state: `.15` rebuilt the whole downstream chain that `.14`'s SourceIR re-seal stranded —
  **24 rebuilt / 24 content-identical / 0 content-changed / 24 validated / 0 failed at every one of
  evidence, semantic, intent, isf-adapter**, verified against an 842 MB snapshot taken before any write.
  Remedy and oracle now share ONE predicate (`scripts/lib/stage_artifact_identity.sh`, extracted
  byte-for-byte; sabotaging it drives the oracle's own self-test 22/22 -> 20/22). The slice's real
  lesson: **`specforge validate` is not idempotent** — each call appends a `validation_backannotation`
  mutation and moves the digest, and every downstream stage retains its upstream ledger as an exact
  prefix, so a post-run validate census broke the chain it was measuring and forced a second rebuild.
  That is designed behaviour (ADR 0038), not a defect; `proof_ledger.claims` is 480 at one mutation and
  480 at four. The read-only way to ask the same loader is the consuming stage in `--dry-run`.
  `rebuild_stage_cascade.sh --check` had the mutating defect and is fixed, with recording-stub controls
  13/14 observed RED at 12/14 on the shipped known-bad code.
  `[claim: claim-provenance-gate-active]`
  `[claim: mdbook-quantitative-census-frozen]`
  `[claim: current-claim-census-frozen]`
- Next action: run `SOURCE-IR-REPRODUCIBILITY.16` — the gate-tier seal check. It must answer "carries no
  ledger" without reading the whole file (54 proofless source_ir cost 14.3 s of a 21.0 s exact scan; grep
  is 0.295 s) and must NOT probe with `specforge validate`.
- In-flight uncommitted: none after this commit; `check_chain_currency.sh` was re-run read-only to certify
  the corpus. This pointer's fixed prose is capped at a derived 12 lines (`MEMORY_ARCHITECTURE.md` §6).
- Blockers: none. Owned, not fixed: `.13` (frozen reviewed fixture not re-derivable),
  `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/`.9`, and `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`. Unowned: `generated/`
  holds 138 `live-document-size-tests.*` dirs (15 MB, 5 clusters). NOT a harness defect — a clean
  `perl scripts/test_live_document_size.pl` run leaks **0** (measured twice, exit 0) and holds a peak of
  **84** live fixtures mid-run, cleaning all of them at exit. Every residue cluster (1/1/53/30/53) is
  below 84, so they are runs that never reached normal exit — a kill bypasses File::Temp's END cleanup.
  WHY those runs died is unknown; do not record a trigger without measuring one. Real gap:
  nothing reclaims them (`specforge clean` reaches them only via `--scope all-generated`, which discards
  the whole corpus) and no gate sees them (`check_live_document_size.pl` reads `git ls-files --cached`
  and skips `generated/`; the locality gate scans only `.project-data/tmp` at depth 1, files only).
  `SCRATCH-RESIDUE-CONTAINMENT` excludes `generated/` by its Non-Goals. Needs its own tree.
