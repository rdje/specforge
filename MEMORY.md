# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> This file is the bounded **resume pointer**. It describes *now*, not the journey.
> History lives in `git log` + `CHANGES.md`; work state lives in the task-trees
> (`docs/tasks/`); durable facts/decisions live in `docs/decisions/`. Do **not** append
> session narration here — overwrite the "Current state" block instead.
> (The prior ever-growing MEMORY.md is preserved in git history before commit `6d668eb2`.)

## How to resume (any AI, any harness)
- Read `MEMORY_ARCHITECTURE.md` (the memory system) and `README.md` (the project).
- All work is tracked in task-trees under `docs/tasks/` (index: `docs/TASK_TREE.md`);
  follow `COMMIT.md` after every slice (unit id in the commit subject).
- Non-negotiable doctrine: see `docs/decisions/0003-task-tree-and-commit-doctrine.md`
  (no code change without an owning task-tree first; signoff quality; zero
  ROADMAP↔code↔mdBook drift; push ~every 30 commits; artifact cleanup ≥ every 24h).
- Durable cross-cutting facts: `docs/decisions/` (e.g. Docling CPU device 0001,
  LLM/VLM provider default 0002).

## Current state (OVERWRITE this block each update — do not append)
- latest_commit: `NLI-INTENT-GATE.2` close commit `a7bf7b6f` (this `NLI-GATE-METRIC.1` commit pending → ~49 ahead of pushed `d92a73e3`). **PUSH THRESHOLD = 200**. CI green baseline = **1254 tests**. KM 19 facts. `subs/fsmgen` pinned `92d7036b`. **STANDING USER REMINDER (2026-06-05): regularly sync/align the mdBook with the codebase — the book is the SOLE user-facing surface; every tree's close MUST refresh its book subsection (BOOK-METHOD-DOC).**
- active_work_unit: none — **`NLI-GATE-METRIC` CLOSED `2026-06-05`** (user "→ surface an nli_* count"): `ir/nli_verify.rs` `nli_demoted_count(&[ResidualDecisionPacket])` + `NLI_RESIDUAL_PREFIX` const; `validate_intent_ir` emits `metric("nli_demoted_contracts", …)` (read-only, no LLM). `validate <intent_ir.json>` surfaces the gate's demotions. +1 test; CI green 1254. — Prior: **`NLI-INTENT-GATE` CLOSED `2026-06-05`** (`.1`–`.2`, user pick "→ (b)"): the NLI verifier made an *active* IntentIR gate (post-build pass — residuals live at IntentIR, not EvidenceIR). `ir/nli_verify.rs`: `obligation_claim_text(&ActorContract)→Option` (phrasable-only: Drive/Stable/Eventually/HandshakeBarrier/Mutex; else None=not gated), `nli_gate_contracts(contracts, verify)` (premise=`provenance.source_text`; NotEntailed→demoted to `ResidualDecisionPacket` `nli_unentailed_<id>`; Entailed/Unknown/un-phrasable kept), `apply_nli_gate(&mut IntentIr)`. **Demote-not-delete**; verifier injected→3 hermetic tests. Opt-in **`intent --nli-verify`** (`--vlm-provider`/`--model`). Book + KM `nli-intent-gate`. CI green 1253. — Earlier: `NLI-ENTAILMENT-VERIFIER` CLOSED (`.1`–`.3`): the verifier module + claim-set gate + live `nli-verify` command. Both NLI trees done. KM `nli-entailment-verifier`, `local-llm-for-text-reasoning`, `eval-gold-interannotator-kappa`.
- κ-study extended `2026-06-05`: **`actor_signal_relation` κ = 1.00** (11/11, 2nd blind agent); **Ollama IS reachable** (started `ollama serve`; `which ollama`=/opt/homebrew/bin/ollama; models pulled = `qwen3-vl:8b`, `qwen2.5vl:7b` ONLY). Cross-model run: **qwen3-vl:8b** correct on single items but thinking NOT disable-able (`think:false`/`/no_think` ignored, ~3400 tok/signal) → too slow (partial 5/6); **qwen2.5vl:7b** fast (21 s) but **κ = 0.285** — it mislabels condition/trigger signals as obligations + invents labels (a model-competence issue, NOT a gold flaw; capable Claude reviewer κ = 0.90 is the signal). Side-finding: qwen2.5vl:7b = SpecForge's DEFAULT extraction VLM → raw extraction likely over-constrains "when X …" conditions.
- LOCAL MODELS NOW (2026-06-05): removed `qwen3-vl:8b` (too-slow thinking); pulled **`qwen2.5:14b-instruct`** (text-only, 9 GB) per user. Local set = `qwen2.5:14b-instruct` (text) + `qwen2.5vl:7b` (vision). Ollama daemon started in-session (`ollama serve`). Results: qwen2.5:14b → **relation κ=1.00**, **constraint κ=0.498** (still trips on condition-vs-obligation in the LABELING framing), **NLI entailment 5/6** (got the subtle "PSEL must be asserted"→NOT-entailed RIGHT). KEY: **NLI framing > free-form labeling** for this nuance; **qwen2.5:14b-instruct is VIABLE for an NLI verifier** (the "NLI gated: needs a model" item is now UNBLOCKED). KM card `local-llm-for-text-reasoning`.
- next_action: PNT — NLI thread DONE (verifier + active intent gate + `nli_demoted_contracts` validate metric, all closed). Follow-ups: run `intent --nli-verify` on a real corpus doc to see actual demotions (needs a SemanticIR + `ollama serve` w/ `qwen2.5:14b-instruct`); OR the standing backlog: gated (TEDS/GriTS [table gold], conformal [held-out set]) or marginal (MUC near-miss; per-relation P/R/F1; unguarded-`min>1` monitor). Local models: `qwen2.5:14b-instruct` (text/NLI) + `qwen2.5vl:7b` (vision); ollama may need `ollama serve`. Reminder: grep `KNOWLEDGE_MAP.md`; **regularly sync mdBook (sole user surface)**; **~49 ahead — no push until ~200 or ask.**
- paused_work: none.
- in_flight_uncommitted: none.
- blockers: none — NLI verifier now has a viable local model (`qwen2.5:14b-instruct`).
