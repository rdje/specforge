# Cat-3 topology-capture recall measurement — `DOC-INTENT-TAXONOMY.4c.i`

- Tree leaf: `DOC-INTENT-TAXONOMY.4c.i`
- Date: `2026-06-23`
- Type: measurement, **read-only** (profiled persisted `generated/intent_ir/*` JSONs; no `validate`/`adapt`
  write → zero canonical mutation)
- Reproducer: `scripts/measure_cat3_topology_recall.py` (tracked; run from the repo root)
- Reinforces: `[[project_kg_isf_completeness]]`, `[[project_doc_intent_taxonomy]]`,
  `[[feedback_verify_fsmgen_before_fr]]`, `[[feedback_isf_no_hacks]]`, `[[feedback_scoring_rigor]]`

## The question (spun out of `.4c`)

`.4c` decided cat-3's distinctive intent — component **topology / connectivity / clock-reset distribution** —
is *captured* as a typed surface (`signal_connectivity`, `infrastructure_signals`) but ISF has **no
declarative static-topology construct** to lower it into, and it filed **no FSMGen FR yet** for two reasons:
the capture looked **sparse and noisy**, and ISF is a per-actor format where static topology may be the
integrator's concern above per-module synthesis. `.4c` deferred the construct decision to this measurement:

> Is cat-3's topology *capture* faithful enough — dense enough, fully-connected enough, clean enough — to be
> worth a new ISF static-topology construct (a verified FSMGen FR + a multi-actor emit)? Or is the bottleneck
> upstream capture-recall, making a construct FR premature?

`.4c` profiled 3 representative cat-3 docs by hand. This leaf measures the **full 15-doc cat-3 set** with a
structural faithfulness gauge, against a **cat-1 wire reference baseline** that proves the same surface is
*capable* of dense, clean topology — isolating exactly what is missing.

## Method (read-only, reproducible)

For each doc, read the two typed topology surfaces off `intent_ir.json` and compute faithfulness metrics. A
real interconnect netlist is **dense** (many connections per component) and **fully connected** (every edge has
a driver and at least one sink); the gauge measures the distance from that:

| Metric | Definition | Why it gauges faithfulness |
|---|---|---|
| **edges / actor** | `len(signal_connectivity) / len(actors)` | density of the captured component graph |
| **both-endpoint %** | edges with `producer_actor_names` **and** `consumer_actor_names` non-empty | a half edge (missing a driver or sink) is an unusable topology stub |
| **clean-endpoint %** | endpoint names that are non-empty, non-`None`, no `\`-escape artifact | name quality (the `.4c` "noisy" claim) |
| **infra w/ distribution** | `infrastructure_signals` with a non-empty `distributed_to_actor_ids` | is the clock/reset *fan-out* captured |
| **infra w/ resolved source** | `infrastructure_signals` with a non-empty `recovered_source_actor_ids` | is the clock/reset *origin* captured (a tree needs a root) |

The **cat-3 set** is the 15-doc `.1`-census reconstruction (listed in the reproducer; the census never
persisted the per-doc labels, so this leaf also closes that gap by enumerating them). Four docs the `.1` census
honest-caveats flag as borderline are annotated, and the verdict is shown **robust to dropping all four**. The
labeling is a one-off measurement label (exactly as `.1` did), **not** runtime code — the runtime recognizer
`.3` is structural per ADR 0006.

## Measured evidence

### Per-doc cat-3 topology capture

```
  sc both esc inf idist isrc  act  e/act  label
  66    4   5   2     2    0   55   1.20  GIC-600 TRM
   0    0   0   2     0    0    9   0.00  Cortex-A76 TRM [borderline cat-4]
   4    4   0   0     0    0   44   0.09  CoreSight SoC-600 TRM v1
   4    4   2   0     0    0   46   0.09  CoreSight SoC-600 TRM v2
   6    4   1   0     0    0   60   0.10  CoreSight SoC-600 TRM v7
   0    0   0   2     0    0    6   0.00  CoreSight SDC-600 TRM
   5    2   3   2     2    0   29   0.17  MMU-700 TRM
  24    4   0   0     0    0   31   0.77  CoreSight TMC TRM
   8    0   0   2     2    0    6   1.33  GIC-400 TRM
   0    0   0   0     0    0    5   0.00  CoreSight Base System Arch
   3    2   0   0     0    0   15   0.20  CoreSight Architecture
   2    2   1   0     0    0   31   0.06  GIC Architecture [borderline cat-2]
   1    0   0   0     0    0   13   0.08  SMMU Architecture [borderline cat-2]
  12    7   0   0     0    0   23   0.52  ARM Debug Interface v6 [borderline cat-1]
   0    0   0   0     0    0    7   0.00  Advanced Comms Channel Arch
```

### Cat-3 aggregate vs the cat-1 wire baseline (the decisive contrast)

| Surface metric | **Cat-3 (15 platform docs)** | Cat-3 core (4 borderlines dropped) | **Cat-1 wire baseline (4 docs)** |
|---|---:|---:|---:|
| actors | 380 | 304 | 65 |
| signal_connectivity edges | 135 | 120 | 267 |
| **edges / actor** | **0.355** | 0.395 | **4.108** |
| **both-endpoint edges** | **33 (24%)** | 24 (20%) | **227 (85%)** |
| clean endpoints | 457/480 (95%) | 407/429 (94%) | 607/607 (100%) |
| infrastructure_signals | 10 | 8 | 8 |
| with distribution (fan-out) | 6 | 6 | 8 |
| **with resolved source (root)** | **0** | 0 | 1 |

Three structural facts fall straight out, and **all three are robust** to the borderline-doc choice:

1. **The cat-3 topology graph is ~12× too sparse.** 380 components produce only 135 captured edges
   (**0.355 edges/actor**) — a 60-component subsystem like CoreSight SoC-600 yields **6 edges**. The *same
   surface* on cat-1 wire docs is **4.108 edges/actor** (267 edges over 65 actors), because there the
   "topology" IS the protocol's small, fully-declared signal graph. The shortfall is not the surface; it is
   how much of a TRM's prose interconnect the extractor recovers.

2. **Three-quarters of the captured cat-3 edges are half-connected stubs.** Only **24%** of cat-3 edges carry
   *both* a producer and a consumer (vs **85%** on wire docs) — the rest name a driver with no sink or a sink
   with no driver, so they cannot be lowered into a connection. GIC-600's 66 edges collapse to **4** usable
   ones; GIC-400's 8 edges to **0**.

3. **The clock/reset distribution tree has no captured root.** `infrastructure_signals` is a near-fixed
   2-per-doc surface (system clock + reset); **0 of 10** cat-3 infra signals carry a resolved *source* actor,
   and only 6 carry a fan-out list — a distribution tree with no root and partial leaves is not a synthesizable
   topology either.

The **name-quality** worry from `.4c` ("`None`/escaped actor names") is the *smaller* problem: endpoints are
**95% clean** (only 12 escaped edges like `pmu\_int` corpus-wide). The dominant faithfulness gap is **density +
half-connectedness + unresolved infra source**, not noise. This refines `.4c`'s "sparse **and noisy**" to
**sparse + half-connected + rootless-infra, with minor name noise** — an honest sharpening of the same call.

## Decision

The cat-3 topology *capture* is **not faithful enough to be worth a new ISF static-topology construct** today —
the bottleneck is **upstream capture-recall, not the missing ISF abstraction**. Lowering 0.355 edges/actor with
76% of edges missing an endpoint and 0% of clock/reset sources resolved would synthesize an **unfaithful sliver
of the interconnect** — precisely the fabrication `.4c` and the honest-residual doctrine forbid.

Therefore, confirming and tightening `.4c`:

1. **No FSMGen FR is filed** for a declarative static-topology / connectivity ISF construct. An FR would be
   premature: the construct is moot until there is a faithful graph to lower into it, and a construct requested
   on a 12×-too-sparse, three-quarters-broken capture would be unfalsifiable
   (`[[feedback_verify_fsmgen_before_fr]]`).

2. **Cat-3 topology stays an honest residual.** Cat-3's *register* half already lowers (register maps +
   bit-fields via `.4a.ii` — CoreSight SoC-600 ~3,250 fields; GIC-600 33 registers); its distinctive topology
   half is captured-but-unlowered and stays so, surfaced honestly rather than fabricated.

3. **The buildable lever — if cat-3 topology is ever pursued — is upstream extraction-recall, owned OUTSIDE the
   `.4` ISF-lowering program**, exactly like `.4d.i` (cat-4 CSR bit-position recovery) and the cat-2 structure
   recall frontier. It would need: (a) denser, fully-connected `signal_connectivity` capture from TRM
   integration prose / connection diagrams (raise edges/actor and both-endpoint%), and (b) clock/reset *source*
   resolution (`recovered_source_actor_ids`). Recorded as a **cross-reference, not minted as a `.4` gap** — the
   `.4` program is ISF-lowering completeness, and there is no ISF-lowering lever here.

4. **Even with faithful capture, lowering topology remains a larger architectural question** to resolve *with*
   FSMGen, not assume: ISF is a per-actor format (one `.isf` → one FSMGen module) and SpecForge's emit is
   single-initiator-actor (`KG-ISF-COMPLETENESS.2a.ii`), so a declarative cross-component netlist would need a
   multi-actor emit + a construct FSMGen's behavioral ATL frontier does not subsume (verified in `.4c`). That
   decision only becomes real once capture-recall clears the bar measured here — it does not today.

**Net:** `.4c.i` closes the cat-3 topology question for now — **capture-recall-gated, not abstraction-gated;
honest non-target until extraction recall improves; no FR, no code, no fabrication.** This mirrors the cat-4
finding (`[[cat4-isa-csr-lowering-decision]]`: the lowerable register half is built, the distinctive half is an
extraction-recall leaf, not an FR) and completes the per-category `.4` triage of where the ISF-lowering gaps
genuinely are.

## Genericity (ADR 0006)

Every metric is a structural count/shape (edge density, endpoint presence, name-escape detection, infra
source/fan-out presence) computed uniformly over all docs. The only per-document input is the one-off cat-3 /
cat-1 measurement labeling in the reproducer — a measurement label like `.1`, not runtime code. No
chip/vendor/protocol-instance name list drives any metric. The decision rests on the measured capture
faithfulness plus FSMGen's published composition grammar (re-verified in `.4c`).

## Gates (this leaf)

- No Rust code, no canonical-artifact mutation → the wire golds / `kg-bench` / emitted `.isf` are
  byte-identical by construction (WIRE-BASED-100 orthogonal).
- `scripts/check_doctrines.sh` green (memory-arch + knowledge-map + task-acceptance); `mdbook build` green;
  knowledge-map derive-and-diff in sync after adding the `[[cat3-topology-capture-recall]]` fact card.
- Objectively measured, per-item demonstrated (`[[feedback_scoring_rigor]]`): every count is read directly off
  the persisted corpus and reproducible via `scripts/measure_cat3_topology_recall.py`.
