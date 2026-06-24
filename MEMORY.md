# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (memory system), `DOCTRINE_ENFORCEMENT.md` (the 4th
  architecture — doctrines are mechanically gated), and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 200 commits; artifact cleanup ≥ every 24h).
- Before committing run `scripts/check_doctrines.sh` (the registry/driver: memory-arch +
  knowledge-map + task-acceptance); hooks + CI run it too. Retrieval: `KNOWLEDGE_MAP.md`.

## Current state (OVERWRITE this block each update — do not append)
- latest_commit (baseline): **`2de8e0c6`** `KG-ISF-COMPLETENESS.5.ii — measure the enum member-quality gate (PER-MEMBER; design corrected; GO)`. **69 ahead** of `origin/main` (push at ~200 → HOLD); this `KG-ISF-COMPLETENESS.5.ii` CODE commit → **70 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (this fresh conversation re-ran the full README bootstrap `2026-06-24`; doctrines GREEN). This session's slices (both DONE): (A) **`.5.ii` measurement/calibration**; (B) **`.5.ii` code**.
- **✅ `KG-ISF-COMPLETENESS.5.ii` CODE LANDED (`2026-06-24`) — the enum surface has a per-MEMBER sentence-fragment gate that cleans junk-conflated enums WITHOUT losing real codes.** `is_prose_fragment_member_name` + `PROSE_SENTENCE_SPINE_WORDS` (`ir/evidence.rs`) gate the member loop in `synthesize_encoding_declarations_for_enum` (one seam → both call paths): a member whose `_`-token set carries an English sentence-spine word (copula/aux/modal/article/demonstrative/relativizer/subordinator; collisions `a`/`i`/`its`/`can`/`may`/`am` EXCLUDED per `.1a`) is a captured prose sentence → skipped; an emptied enum is not minted (honest residual). **Per-member not per-enum** (whole-enum drop loses AXI `BRESP`'s codes); **not value-restart** (AHB `HPROT` restarts but all members clean). Per-item precision 1.000 (0/115)/recall 1.000 (269/269). AXI `manager.isf` now emits `(BRESP (OKAY 0)(EXOKAY 1)(SLVERR 2)(DECERR 3)(DEFER 4)(TRANSFAULT 5)(RESERVED 6)(UNSUPPORTED 7))`. **WIRE-BASED-100 = 1.000 before==after** (PROVEN, all 10 seeds, scored surface byte-identical); FSMGen `--strict` `success` on AXI+APB; `kg-bench` 156/156; `run_ci.sh` GREEN (lib **1716**, +4). **`.5` enum-surface fidelity now BUILT** (`.5.i` name-gate + `.5.ii` member-gate).
- next_action (PNT): **KG-ISF-COMPLETENESS has no further immediately-buildable leaf** (`.5` built; `.1c.ii` upstream-NLP-gated; `.2b` measured-marginal; `.3` closed). **PNT advances to the next active tree** — scan `docs/TASK_TREE.md` for the next active tree with a buildable frontier (e.g. CORPUS-COVERAGE re-ingest tail is empirically low-value per `[[feedback_not_complete_attack_substantive_gaps]]`; prefer a substantive north-star gap — relation-incompleteness / transaction body-emission / a surfaced extraction-precision lever). Honest residual to consider re-opening: the `.5.ii` deeper member-quality classes (section-caption / `_WIDTH` leak / restart-of-clean sub-enum split) need their OWN measurement before code.
- in_flight_uncommitted: **none** once committed (code: `ir/evidence.rs` member-gate + const + 4 tests; docs: research report §`.5.ii LANDED`, KM card + `KNOWLEDGE_MAP.md` regen (126 facts/928 keys), `KG-ISF-COMPLETENESS.md` `.5.ii` node `done` + acceptance checklist + changelog, README bullet, CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/MEMORY, book `pipeline/isf-adapter.md`). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** the release binary `target/release/specforge` now carries BOTH `.5.i`+`.5.ii` (rebuilt `2026-06-24`). Generated working copy: AXI+APB+CCIX intent/`.isf` were re-emitted post-`.5.ii` during verification; the rest of `generated/` is still pre-`.5.i` (git-ignored; durable trace = commits — a full corpus refresh is available but not required). Baseline binary for `.5.ii` eval was preserved at `scratchpad/specforge.baseline`. fsmgen = Perl `subs/fsmgen/bin/fsmgen` (NOT cargo); `--check` needs `--json`. Host RAM headroom OK. Artifact-cleanup standing (≥24h). lib test count **1716** (RUST_CODEBASE_ANALYSIS.md count is AUDIT-DOC-RECONCILE-owned).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
