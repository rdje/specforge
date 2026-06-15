---
id: canonical-promotion-no-reingest-protocol
title: Promote a doc's constraint surface onto canonical WITHOUT re-ingest (the CANONICAL-PROMOTION-SWEEP per-doc protocol)
answers:
  - "how do I promote a document's constraint surface on canonical without re-ingesting the PDF"
  - "why can't I just run converge to land the LLM-primary promotion on a canonical artifact"
  - "does converge re-ingest the PDF every run"
  - "what is the RAM-safe per-doc protocol for the CANONICAL-PROMOTION-SWEEP"
  - "how do I measure the extraction-quality gauge before and after a canonical promotion"
  - "what was the HBM2 canonical promotion pilot result"
  - "is the LLM-primary promotion gauge improvement reproducible on canonical artifacts (not just /tmp copies)"
  - "what is promotion_status not_promoted_review_required and where does the canonical mutation live"
date: 2026-06-15
tags: [extraction-quality, llm-primary, promotion, canonical, ram-safety, converge, gauge, hbm2]
evidence: crates/specforge/src/commands/converge.rs (run_convergence — SourceIr::build + materialize always re-ingests); crates/specforge/src/commands/extract_constraints_llm.rs (promote_constraints loads evidence_ir.json from disk, rewrites in place); crates/specforge/src/commands/nli_verify.rs (measure_and_persist_gauge); docs/tasks/CANONICAL-PROMOTION-SWEEP.md (.1 Decisions)
reverify: cargo test -p specforge --lib promote_constraints_records 2>&1 | tail -2
---

`converge <source>` **always re-ingests** — `run_convergence` calls `SourceIr::build + materialize`
(Docling, RAM-heavy) every run. So when the `CANONICAL-PROMOTION-SWEEP` frontier mandates
promoting onto an *intact* evidence bundle with **no re-ingest**, you do NOT run converge. Instead
you run the **no-re-ingest equivalent** of converge's post-stability promotion sequence directly on
the persisted canonical EvidenceIR, because `extract_constraints_llm::promote_constraints` loads
`evidence_ir.json` from disk and rewrites it in place ([[llm-primary-promotion-stage]]).

**The repeatable per-doc protocol (RAM-safe; locked by `CANONICAL-PROMOTION-SWEEP.1`):**

1. **Pre-flight:** `ollama stop <model>`; confirm **≥40% RAM free** (`memory_pressure`); ensure no
   Docling/ingest is running (serialize the 14B vs Docling); `cargo build --release` — `cargo test`
   does **not** rebuild the bin and live measurement needs the fresh `target/release/specforge`.
2. **Backup** `evidence_ir.json` (+ `semantic_ir.json` / `intent_ir.json` / `adapter.json`) →
   `*.prepromote.bak` (revert capability for the review gate).
3. **BEFORE gauge:** `specforge nli-verify <evidence_ir>` (defaults `ollama` + `qwen2.5:14b-instruct`).
4. **PROMOTE (canonical mutation, no re-ingest):** `specforge extract-constraints-llm <evidence_ir>`
   — replaces `signal_constraints` with the LLM-primary grounded surface (polarity-refined, deduped),
   records manifest `constraints.llm_primary`, drops the stale gauge, writes back.
5. **Rebuild downstream (deterministic, no model):** `specforge semantic <evidence_ir>` →
   `intent <semantic_ir>` → `adapt <intent_ir> --target isf`.
6. **AFTER gauge:** `specforge nli-verify <evidence_ir>`.
7. **Gates:** `specforge kg-bench` green; provider-free byte-stability holds by construction
   (`generated/` is git-ignored — the standalone path touches only this doc, no tracked drift).
8. **RAM watchdog:** sample `memory_pressure` free% every ~3s through the 14B steps; **abort at ≤15%
   free (≥85% used)**; `ollama stop` when done. (The owner's RAM-ceiling discipline — never the 90→93% reboot zone.)
9. **Record** before/after gauge + RAM trace + `promotion_status`. Wire docs (`.2`) ALSO re-verify
   the full `WIRE-BASED-100` gold battery `1.000` on the promoted canonical artifact, else revert.

**Pilot (`2026-06-15`, HBM2 `jesd235a_2015_11_hbm2_dram`, non-wire):** BEFORE **85.7% not-entailed**
(2E/12N/0A, 14 records) → promote 14 → 20 (11 distinct sentences) → AFTER **40.0%** (12E/8N/0A, 20
records). This **reproduces the `LLM-PRIMARY-PROMOTION.4` REDIRECTED-`/tmp`-copy datum EXACTLY on the
canonical artifact** ("HBM2 grows 14→20 AND cleans 85.7%→40%") — i.e. the proven `/tmp` improvement
lands identically on canonical (oracle reproducibility). RAM stayed **≥42% free** across all three
14B steps (min 42; never near the 15%-free kill line or the 90→93% reboot zone). `kg-bench` 156/156.

`promotion_status = not_promoted_review_required`: the mutation is realized in **git-ignored
`generated/`** (so the commit landing it is docs-only), the improvement is recorded, but the
canonical surface is NOT declared owner-approved — the `.prepromote.bak` backups stand by for revert
pending review (the `R7-VALIDATION` doctrine).

Honest caveat (from [[llm-primary-promotion-stage]] `.3b`): on docs with grounded reset polarity the
gauge can trade NLI-optics for gold-gate correctness (AXI promoted reads 48% vs 91% Pattern). HBM2
has no such case, so its gauge improvement is unambiguous. Related: [[extraction-quality-gauge-standing]].
