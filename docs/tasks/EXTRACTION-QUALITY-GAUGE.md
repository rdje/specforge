# EXTRACTION-QUALITY-GAUGE: measure the extraction-quality gap — and CHI's is large

## Metadata

- Tree ID: `EXTRACTION-QUALITY-GAUGE`
- Status: `active` (gauge established + CHI measured; the fix leaves are open R-lane work)
- Roadmap lane: `R15e`/`R16` (extraction quality / production-readiness)
- Created: `2026-06-06`
- Last updated: `2026-09-17`
- Parent context: `TABLE-GRITS-CONFORMAL.4` ran the NLI-oracle conformal on CHI and it would not
  calibrate. The diagnosis turned out to be a real extraction-quality gap, not a metric problem.

## Goal

Measure how far SpecForge's extracted constraints are from what the source actually says, and close the
gap where it is mechanically closable. The reusable capability is the gauge itself: the fraction of
extracted constraints the source does NOT entail, per document, cheap enough to stand in CI. The gauge,
the CHI measurement it produced, the root causes, every per-leaf contract, each enforced acceptance
checklist, and the dated changelog live in the task-evidence parts; this root carries the bounded current
summary and the executable owner registry only.

## Current Program State

- The gauge discriminates sharply and is hand-validated: APB 14 constraints at ~29% not-entailed against
  CHI's 162 at ~83%, with 18 of 18 sampled CHI errors confirmed genuine by reading them against source.
  It is a standing, persisted per-document measurement, wired through `nli-verify`, `converge` and
  `validate` by `.0`.
- Two root causes hold, and neither is a metric artefact. **Entity discrimination is broken**: the KG is
  `(actor -verb-> signal)` and is only as good as its node typing, yet 64% of CHI's 162 constraint
  subjects appear in no declared or hinted signal set, while the real signal `TXSACTIVE` appears in
  none of them. **The constraint unit does not match the unit of meaning**: conditional and temporal
  scope is dropped, permissions become obligations, and relations flatten to value constraints.
- Closed and measured: the gauge `.gauge`; the `.1`/`.2` prototypes and the pivot they forced; the `.5`
  LLM-primary thesis; `.6`, `.7`, `.8`; the field ontology `.FIELD.1`-`.FIELD.4`, which gave packet and
  flit fields a typed home and a parallel constraint surface; the deterministic precision gates
  `.3a`-`.3i`; the dedup `.4`; and the standing gauge `.0`. The `.3k` kind-span programme closed
  `.3k.1`-`.3k.8`, `.3k.10`-`.3k.13` and `.3k.2a`-`.3k.2g`/`.3k.2j`-`.3k.2k`.
- A standing rule this tree earned the hard way: a gate is adjudicated before it is wired. `.3j` asked
  what the five positional spurious-subject gates would cost on the LLM path, measured 7 refusals of 149
  records, read all seven, found 4 of them CORRECT, and answered NO. Three of the five have no
  population on that path at all.
- The open work is the LLM path's own shape, not another deterministic micro-gate. An `llm_sigcon_*`
  record does not carry the obligation clause it was minted from, so no positional subject gate can
  judge it; that is `.3j.1` and its `.3j.1.a`/`.3j.1.b` split. Beside it sit `.3j.2` (a bare common noun
  passed catalog grounding), `.3j.3` (the LLM pass cannot see a span the deterministic paths missed — a
  recall ceiling), the untyped-default successors `.3k.2h`/`.3k.2i`, the re-owned `.3k.9`, and the
  containers `.3`, `.3k`, `.3k.2` and `.FIELD`.

## Current Frontier

Active extraction-quality frontier: `EXTRACTION-QUALITY-GAUGE.3j.1.a`.

`.3j.1.a` carries the minting clause on the LLM proposal — `source_text` IS the span the record was
minted from — which is the prerequisite for judging an `llm_sigcon_*` record positionally at all;
`.3j.1.b` re-censuses the five gates once it does. `.3j.2` must be sized with the real
`ground_constraint`/`ground_constraint_typed` membership test rather than a proxy over the seven
signal-bearing EvidenceIR surfaces: a proxy census flagged 36 of 149 subjects and survived reduction at
20, but those survivors include APB's `PSEL`, which is unquestionably declared — so the proxy is wrong
and its numbers are not findings.

## Detailed task evidence

[Open the task-evidence index](extraction-quality-gauge/INDEX.md) for every detailed task contract,
acceptance checklist, decision, verification entry, commit record, semantic evidence part, and the exact
pre-migration source capsule.

## Executable Owner Registry

These compact declarations preserve stable-path task-owner lookup. The task-evidence index is the primary
detail-routing authority, and the route catalog carries every leaf with its lifecycle and its detail part.

- ID: `EXTRACTION-QUALITY-GAUGE`
- ID: `EXTRACTION-QUALITY-GAUGE.0`
- ID: `EXTRACTION-QUALITY-GAUGE.1`
- ID: `EXTRACTION-QUALITY-GAUGE.2`
- ID: `EXTRACTION-QUALITY-GAUGE.3`
- ID: `EXTRACTION-QUALITY-GAUGE.3a`
- ID: `EXTRACTION-QUALITY-GAUGE.3b`
- ID: `EXTRACTION-QUALITY-GAUGE.3c`
- ID: `EXTRACTION-QUALITY-GAUGE.3d`
- ID: `EXTRACTION-QUALITY-GAUGE.3e`
- ID: `EXTRACTION-QUALITY-GAUGE.3f`
- ID: `EXTRACTION-QUALITY-GAUGE.3g`
- ID: `EXTRACTION-QUALITY-GAUGE.3h`
- ID: `EXTRACTION-QUALITY-GAUGE.3i`
- ID: `EXTRACTION-QUALITY-GAUGE.3j`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.1`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.1.a`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.1.b`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.2`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.3`
- ID: `EXTRACTION-QUALITY-GAUGE.3k`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.1`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.10`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.11`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.12`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.13`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2a`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2b`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2c`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2d`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2e`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2f`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2g`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2h`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2i`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2j`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.2k`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.3`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.4`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.5`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.6`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.7`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.8`
- ID: `EXTRACTION-QUALITY-GAUGE.3k.9`
- ID: `EXTRACTION-QUALITY-GAUGE.4`
- ID: `EXTRACTION-QUALITY-GAUGE.5`
- ID: `EXTRACTION-QUALITY-GAUGE.6`
- ID: `EXTRACTION-QUALITY-GAUGE.7`
- ID: `EXTRACTION-QUALITY-GAUGE.8`
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD`
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.1`
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.2`
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.3`
- ID: `EXTRACTION-QUALITY-GAUGE.FIELD.4`
- ID: `EXTRACTION-QUALITY-GAUGE.gauge`

## Changelog

The complete dated changelog is in the changelog part, which is sealed against the archived capsule. New
dated entries belong in the active part beside the leaf that earns them.
