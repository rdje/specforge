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
- latest_commit (baseline): **`ed47237f`** `KG-ISF-COMPLETENESS.5.i — gate the enum fallback name + drop emitter orphan (type)`. **68 ahead** of `origin/main` (push at ~200 → HOLD); this `KG-ISF-COMPLETENESS.5.ii measurement` (docs-only) commit → **69 ahead**. Untracked (stays): `.claude/settings.json`, `.cache/` (gitignored symlink).
- **MODE: PNT loop** (this fresh conversation re-ran the full README bootstrap `2026-06-24`: entry docs + `docs/TASK_TREE.md` + KG-ISF-COMPLETENESS tree + the `.5`/`.5.i` decision packet; doctrines GREEN). This session's slices: (A) **`.5.ii` measurement/calibration** [DONE, this commit]; (B) **`.5.ii` code** [next].
- **✅ `KG-ISF-COMPLETENESS.5.ii` MEASUREMENT DONE (`2026-06-24`, docs-only) — the member-quality gate is PER-MEMBER (design corrected, GO).** Read-only census over all 78 IntentIR docs (561 enums / 12 509 members) **overturns the recorded `.5` plan**: a whole-enum drop destroys real codes (AXI-gold `BRESP` fuses 7 prose fragments WITH the 8 genuine codes `OKAY/EXOKAY/SLVERR/DECERR/…`); value-restart is NOT a junk signal (AHB-gold `HPROT` restarts but all 15 members are clean identifiers). Load-bearing signal = per-member NAME shape: a synthesized member whose `_`-token set holds an English **sentence-spine** word (copula/aux/modal `IS`/`ARE`/`BE`/`HAS`/`MUST`; article/demonstrative `THE`/`THIS`; relativizer/subordinator `WHICH`/`WHEN`/`IF`/`BECAUSE`) is prose → drop; collisions EXCLUDED per `.1a` (`A`/`I`/`ITS`/`CAN`/`MAY`/`AM`). **Per-item: precision 1.000 (0/115 clean-anchor), recall 1.000 (269/269 junk-anchor); 30.2 % of members drop.** Honest residuals deferred (glossary `SEE…`, front-matter, section-caption `B2_3_1_…`, `_WIDTH` leaks, restart-of-clean). Report §`.5.ii measurement`; KM `[[generic-enum-conflation]]`.
- next_action (PNT): **`KG-ISF-COMPLETENESS.5.ii` CODE** — land the per-member sentence-spine fragment drop in `synthesize_encoding_declarations_for_enum` (`ir/evidence.rs`): after sanitizing `member_name`, `continue` (skip) if any `_`-token ∈ the spine lexicon (collisions excluded); an emptied enum is not minted (existing no-statements → no-`SymbolDefinition` contract). Define a focused const (e.g. `PROSE_SENTENCE_SPINE_WORDS`), drift-guard it. **Byte-changing on wire golds** (strict improvement) → MUST preserve baseline binary, rebuild gold evidence both ways, prove WIRE-BASED-100 **before==after** via `eval-extraction --provider skip`; `kg-bench` 156/156; FSMGen `--strict` 0; `run_ci.sh` GREEN; corpus census before→after. Then full lockstep + book `pipeline/isf-adapter.md`. Other open leaves deferred (`.1c.ii` upstream-NLP-gated, `.2b` measured-marginal).
- in_flight_uncommitted: **none** once committed (docs-only: research report §`.5.ii measurement`, KM card + `KNOWLEDGE_MAP.md` regen (126 facts), `KG-ISF-COMPLETENESS.md` `.5.ii` node + changelog, CHANGES/DEV_NOTES/LIVE_ACHIEVEMENT/MEMORY). blockers: none. **Repo HANDOFF-READY.**
- **OPERATIONAL REALITY:** the release binary `target/release/specforge` carries the `.5.i` gate (built `2026-06-24 11:50`, post-`.5.i`/pre-`.5.ii`) — it is the BASELINE for the `.5.ii` before/after eval (copy it aside before `cargo build --release`). Persisted `generated/` IntentIR/`.isf` are pre-`.5.i` (durable trace = commits). fsmgen = Perl `subs/fsmgen/bin/fsmgen` (NOT cargo); `--check` needs `--json`. Host RAM headroom OK this session; serialize Docling vs 14B model. Artifact-cleanup standing (≥24h). lib test count **1712** (RUST_CODEBASE_ANALYSIS.md count is AUDIT-DOC-RECONCILE-owned — not refreshed in a docs-only slice).
- **DOCTRINE (non-negotiable):** RUNTIME PDF-AGNOSTIC (ADR-0006), no deny/filter lists, honest RESIDUAL over fabrication; a missing ISF abstraction → verified FSMGen FR, never an emitter hack ([[feedback_isf_no_hacks]] / [[feedback_verify_fsmgen_before_fr]]). `scripts/check_doctrines.sh` green before committing; `scripts/run_ci.sh` green before committing CODE. Push ~every 200 (HOLD).
