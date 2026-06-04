# EVAL-GOLD-INTERANNOTATOR-AGREEMENT: is the eval answer-key trustworthy? (Cohen's κ)

## Metadata

- Tree ID: `EVAL-GOLD-INTERANNOTATOR-AGREEMENT`
- Status: `done` (CLOSED `2026-06-05` — κ = 0.90, gold reliable; one ambiguity surfaced)
- Roadmap lane: `R15e` (extraction evaluation / honesty)
- Created: `2026-06-05`
- Owner: repo-local workflow
- Parent context: the eval gold (`test_data/llm_eval/seed_apb.json`) is **single-source**
  (`label_status: agent_drafted`) — we grade extraction against it but have never checked it is
  *reliable* (not one annotator's idiosyncrasies). The grounding (`extraction-evaluation.md`;
  Cohen κ 1960 / Krippendorff α / Artstein-Poesio CL'08) says: measure **inter-annotator
  agreement** with a second independent annotator. User chose to unblock this ("just handle it").

## Method

- **Unit of agreement:** each *(statement, grounding-signal)* pair (18 over the 8
  `signal_constraint` items). **Label:** the `constraint_kind` assigned to that signal, or `NONE`
  (the signal carries no obligation in that statement). Label set seen in the gold:
  `must_be_stable / must_be_low / must_be_asserted / must_be_value / NONE`.
- **Second annotator:** a fresh agent given *only* the statement text + the listed signals + the
  label scheme — **blind** to the gold facts and the gold's `label_note`s.
- **Metric:** Cohen's κ between the second annotator's per-unit labels and the gold's, plus raw
  agreement. Interpretation (Landis-Koch): κ ≥ 0.81 almost perfect, 0.61–0.80 substantial,
  0.41–0.60 moderate, < 0.41 fair/poor → the gold (and our whole eval) would be shakier than it
  looks.
- Honest caveat: 18 units is small → κ is a *signal*, not a precise estimate; the methodology +
  the capability are the durable deliverable.

## Acceptance Criteria

- `.1`: second blind annotation obtained; Cohen's κ + raw agreement computed; per-unit
  disagreements listed; result recorded (a finding doc + KM card if durable); the eval gold
  optionally gains a recorded `interannotator_kappa`. Honest interpretation reported. CLOSE.

## Task Tree

- ID: `EVAL-GOLD-INTERANNOTATOR-AGREEMENT`
  Status: `active`
  Children: `.1` (second annotation + κ + report + close)

- ID: `EVAL-GOLD-INTERANNOTATOR-AGREEMENT.1`
  Status: `done`
  Goal: run the blind second annotation (independent agent), compute Cohen's κ vs the gold over
    the 18 constraint units, list disagreements, report + record.
  Acceptance: κ computed; disagreements surfaced; honest interpretation; recorded.
  Verification: passed (`2026-06-05`) — a blind independent agent re-annotated the 8
    `signal_constraint` statements (text + signals only, no gold/notes). Cohen's **κ = 0.90**,
    raw agreement **17/18 = 0.944** ("almost perfect", Landis-Koch) → the gold is **reliable**.
    The one disagreement (`statement_0202` / **PENABLE**: gold `NONE` vs reviewer `must_be_stable`)
    is a genuine interpretive ambiguity over the "and any other control signals" clause — recorded
    as a known ambiguity, not a gold fix (changing it would shift eval scoring on a debatable
    call). KM card `eval-gold-interannotator-kappa`. Honest caveat: 18 units → κ is a strong
    signal, not a precise estimate. Cross-model follow-up (local Ollama qwen as a 2nd rater) noted
    — not reachable from the sandboxed shell.

## Current Frontier

| Order | Leaf | Status | Why next |
| --- | --- | --- | --- |
| 1 | `EVAL-GOLD-INTERANNOTATOR-AGREEMENT.1` | `done` | κ = 0.90 computed; gold reliable → **tree CLOSED** |

**Tree CLOSED `2026-06-05`.** The agent-drafted eval gold is empirically reliable (κ = 0.90);
the lone disagreement is a documented ambiguity, not an error. Cross-model (qwen) and
relation-task agreement are extensible follow-ups.

## Decisions

- `2026-06-05`: scope to the `signal_constraint` task (cleanest per-signal label); the
  `actor_signal_relation` task is an extensible follow-up. Second annotator is a blind agent (no
  gold, no label-notes) for genuine independence.

## Changelog

- `2026-06-05`: Created — measure whether the agent-drafted eval gold is reliable via Cohen's κ
  against an independent blind second annotation (Cohen/Krippendorff/Artstein-Poesio).
- `2026-06-05`: **CLOSED.** κ = 0.90 (17/18, almost-perfect) → gold reliable; one documented
  ambiguity (`0202`/PENABLE, the "any other control signals" clause); KM card
  `eval-gold-interannotator-kappa`. User's qwen-as-2nd-rater suggestion adopted as the cross-model
  follow-up (Ollama not reachable from the sandboxed shell).
- `2026-06-05` (addendum): extended to the **`actor_signal_relation`** task — a 2nd blind agent →
  **κ = 1.00** (11/11). And ran the **local Ollama qwen** raters (started `ollama serve`):
  **qwen3-vl:8b** reachable + correct on single items but its thinking can't be disabled
  (`think:false`/`/no_think` ignored) → too slow for the batch (partial 5/6 on completed units);
  **qwen2.5vl:7b** fast (21 s) but **κ = 0.285** because it systematically mislabels
  condition/trigger signals as obligations + invents labels — a statement about the model's
  competence, not the gold (the capable Claude reviewer's κ = 0.90 is the reliability signal).
  Side-finding: qwen2.5vl:7b is SpecForge's default extraction VLM → its raw extraction likely
  over-constrains "when X …" conditions (what the deterministic backbone + grounding catch).

## Commit Log

| Leaf | Commit subject or reference | Notes |
| --- | --- | --- |
| `EVAL-GOLD-INTERANNOTATOR-AGREEMENT.1` | `EVAL-GOLD-INTERANNOTATOR-AGREEMENT.1 — Cohen's kappa 0.90 on the constraint gold (reliable); close` | analysis; KM card |
