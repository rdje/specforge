# EVAL-DOCUMENT-RECALL: attribution-agnostic fact recall (the per-statement scorer under-counts)

## Metadata

- Tree ID: `EVAL-DOCUMENT-RECALL`
- Status: `done` (CLOSED `2026-06-05`; `.1`)
- Roadmap lane: `R16`/`R15e` (eval quality)
- Created: `2026-06-05`
- Parent context: discovered while running the relation A/B (`eval-extraction` on `seed_apb.json`)
  with the per-kind P/R/F1 from `EVAL-RELATION-GRANULARITY`.

## The finding

Running the A/B on the real APB EvidenceIR, BOTH arms (pattern `--provider skip` and LLM `ollama`)
scored **relations R=0** and **constraints R=0.333**. Root cause: the scorer
([`score_dataset`]) is **closed-world per labeled statement** — a gold fact must be predicted on the
*exact* gold statement. But the extractor finds the facts and attributes them to *different*
sentences: a check showed **all 6 gold relations present in the EvidenceIR** (exact match), **none on
the gold's own statement**. So `R=0` was a *measurement artifact*, not a recall failure.

## What

- `crate::eval::score_fact_recall(items, predicted) -> BTreeMap<EvalTask, (found, total)>` —
  **document-level fact recall**: a gold fact counts as found if predicted on ANY statement
  (attribution-agnostic; union the predicted keys per task). Precision is intentionally omitted (the
  gold is a small labeled subset → document-level precision is not meaningful).
- Surfaced in the `eval-extraction` report under "document-level fact recall".

## Result on the real APB (the truth the per-statement scorer hid)

| task | per-statement R | document-level recall |
| --- | --- | --- |
| `signal_constraint` | 0.333 (2/6) | **0.833 (5/6)** |
| `actor_signal_relation` | 0.000 (0/6) | **1.000 (6/6)** |

The deterministic extractor's *true* recall is strong — all 6 gold relations, 5/6 constraints — the
per-statement metric just mis-credited attribution. (Useful corollary: it also explains why the LLM
A/B arm looked identical to baseline — its new facts dedupe against facts the patterns already found.)

## Verification

Passed (`2026-06-05`) — +1 unit test (`document_level_recall_finds_facts_attributed_to_another_statement`:
per-statement tp=0, document-level (1,1)) + the real-APB run above. Full `scripts/run_ci.sh` GREEN
(1270→1271).

## Task Tree

- ID: `EVAL-DOCUMENT-RECALL` · Status: `done` · Children: `.1`
- ID: `EVAL-DOCUMENT-RECALL.1` · Status: `done` · Goal: document-level fact recall + report wiring +
  test, found via the relation A/B. Verification above.

## Follow-up

- Consider authoring future relation/constraint gold against the statement the extractor *uses* (or
  accept document-level recall as the primary recall metric for facts whose attribution is fluid).

## Changelog

- `2026-06-05`: Created + CLOSED — `score_fact_recall` (document-level), surfaced in `eval-extraction`;
  found the per-statement scorer was under-counting (APB: relations 0→6/6, constraints 2/6→5/6).
