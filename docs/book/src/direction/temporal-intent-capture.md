# SOTA Temporal-Intent Capture (R16)

This chapter is the human-facing capture of the active forward program.
The authoritative, machine-tracked source is the task-tree umbrella
`docs/tasks/R16-INTENT-CAPTURE.md` and ROADMAP lane `R16`; this page
must stay consistent with them.

## Thesis

A digital-design PDF (protocol or component) encodes design intent as the
**temporal behavior of actors observed at their boundary — pins and
ports**. The single hardest, highest-value problem is **accurate and
reliable extraction of that temporal behavior from prose *and* from
timing diagrams into a typed knowledge graph**. Once the typed KG holds
accurate temporal behavior for every involved signal/port, the rest
(KG → `IntentIR` → `.isf` → FSMGen) is **almost mechanical**.

Consequence for sequencing:

1. the typed target the extraction lands in must exist first and be
   shaped like a *timed contract over actor boundaries* (mechanical to
   lower from);
2. capture fidelity must be **objectively measurable early** — the
   spec's own figures/waveforms are near-ground-truth conformance
   vectors;
3. then the bulk of effort concentrates on **prose + timing-diagram →
   typed KG extraction fidelity**, measured against that metric.

The residual-honesty doctrine is enforced throughout: temporal intent
the source does not license is preserved as an explicit residual, never
fabricated.

## The six points, in program order

| Order | Tree | Point | What it delivers |
| --- | --- | --- | --- |
| 1 | `R16-CONTRACT-IR` | #1 | Typed timed-contract IR — per-actor `assume`/`guarantee`, operators `rose/fell`, `stable … throughout`, `s ##[m:n] t`, `s until t`, `eventually within N`, `mutex`, `ordered_before`. One bound obligation = one object. The mechanical-to-lower target. (DAG root) |
| 2 | `R16-KG-PROTOCOL-ONTOLOGY` | #2 | First-class `Channel` / `Transaction` / `Phase` / `HandshakePair` KG nodes + edges; `IntentIR` becomes a systematic projection of protocol structure. |
| 3 | `R16-CAPTURE-FIDELITY-GATES` | #5 | Realizability/consistency check + replay of the spec's own figures as conformance vectors → the **objective capture-fidelity metric** and residual/repair driver. Pulled early — it is the objective function for the hard problem. |
| 4 | `R16-MULTIMODAL-CONTRACT-FUSION` | #3 | Cluster cross-modal evidence (prose + table + figure + state diagram) keyed by (actor, channel/group, phase) into one contract with provenance + typed merge + explicit disagreement surface. |
| 5 | `R16-WAVEFORM-CONTRACT-MINING` | #4 | Timing diagram → structured partial trace → generalized contract; cross-check vs prose. **The crux extraction thrust** (highest ceiling, research-grade). |
| 6 | `R16-CONSTRAINED-VERIFIED-EXTRACTION` | #6 | Schema-constrained LLM/VLM extraction directly into ContractIR + entailment verifier + protocol-pattern template library + uncertainty-driven converge. **The crux, continuous.** |

Dependency DAG: `1 → 2`; `1 → 3`; `{1,2,3} → 4`; `{1,3} → 5`;
`{1,3} → 6`. `5` and `6` produce/clean contract candidates that `4`
fuses; `3` measures `4`/`5`/`6`. `1` is the spine; `3` is the objective
function.

## Why this order

Points #1/#2 are not the hard part — they are the *typed target* so
extraction has somewhere accurate to land and so lowering stays
mechanical. Point #5 is pulled ahead of the extraction work because you
cannot improve what you cannot measure, and the spec ships its own
near-ground-truth vectors (its figures). Points #3/#4/#6 are the
extraction-fidelity thrust — the crux the thesis names. Each sub-tree is
`proposed` until promoted in DAG order under
`R16-INTENT-CAPTURE`; all code lands under `COMMIT.md`-tracked leaves
with `scripts/run_ci.sh` green per leaf.
