# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)
- Active unit: `CLAIM-VERIFICATION-ADOPTION.7`. Open: `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.7`/`.8`/`.9`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `SPEC-TO-INTENT-ALIGNMENT.9`;
  `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM.1`/`.3`/`.4`/`.6`. The last nine are tracking-only.
- Current state: `CLAIM-VERIFICATION-ADOPTION.10` is closed. The upstream standard was re-read section by section,
  which directive 17 asks for and which nothing had done since `.1`. The source has NOT moved (mtime `2026-08-26`),
  and the local standard was missing a set of its normative rules anyway — so the gap dated from the original
  adoption, and a currency check built on "has upstream changed?" would have returned green forever. `CLAIM_VERIFICATION.md`
  is a RESTATEMENT, not a copy: neither a byte diff nor a digest comparison answers the currency question. §11 now
  records the reading boundary, the single enumeration of adopted rules with each one's local home, and four
  deliberate refusals with reasons. The adoption caught itself: the first absence probe covered one file and would
  have published a claim about the repository on evidence about that file — Leg 1's granularity rule, adopted in
  the same commit. Widening it found ADR 0042 already carrying the general form as rationale while the standard
  carried only its instances, so that rule was DEMOTED here rather than absent. `.10a` then corrected `.10`
  itself: asked a second time whether the findings held, the narrow probe re-derived to SEVENTEEN terms, not the
  fifteen published on four surfaces (fifteen is the discriminating subset, and the widened probe's count) — a
  real quantity substituted for the one the sentence names, `.7`'s ninth instance one commit later. A second
  defect in the same paragraph, "visible to no gate this repository has", was a set claim without its
  enumeration. Both corrected. The self-catch rate for this adoption is THREE, two of them set claims, and the
  third was found only because the director asked again.
- Next action: `CLAIM-VERIFICATION-ADOPTION.7` — its scope question is now answered and the leaf NARROWS. A claimed
  mechanism is a review obligation under the now-normative illustration rule, because no checker can decide whether
  two accounts predict the same observation; the gate stays on counts. Build the bounded declared map binding each
  published count in a claim-annotated prose region to its producer command and the exact field of that producer's
  report, re-deriving and comparing rather than pattern-matching numbers out of prose, with an unlisted count in a
  governed region reported rather than ignored. Keep the two residues `.10` identified: the cross-surface
  disagreement signal (two governed regions stating different values for one quantity, needing no producer), and an
  optional registry field naming which prior adjudication a mechanism claim was checked against. RED controls must
  prove a drifted count, a count bound to the wrong field, and an unmapped count in a governed region.
- In-flight uncommitted: none after this commit.
- Blockers: none. Owned, not fixed: `SOURCE-IR-REPRODUCIBILITY.13`; `CLAIM-VERIFICATION-ADOPTION.7`/`.8`/
  `.9`; `CHANGES-LEDGER-ROLLOVER.4`; `LIVE-DOCUMENT-PRESSURE-HEADROOM.4`/`.6`;
  `SCRATCH-RESIDUE-CONTAINMENT.4` — the `generated/` fixture producer is still signal-unsafe, so the residue
  recurs. Never run the fixture suite concurrently with the locality gate:
  `check_persisted_artifact_paths.pl` walks every `*.json` under `generated/` and FAILS if a fixture run
  deletes one mid-walk. `docs/research/*.md` is 63 of a 64-file ceiling with no rollover, so do not write a
  research record until `.4` releases that surface. Read every live-document percentage from
  `perl scripts/check_live_document_size.pl` rather than carrying it here; its warnings currently name
  `research_records`, `shipped_behavior` (`docs/book/src/pipeline/evidenceir.md`), `rust_analysis`,
  `validation_snapshot`, and `workflow_standards` — all five are already on
  `LIVE-DOCUMENT-PRESSURE-HEADROOM`'s opening pressure boundary table, so none is an unowned warning.
