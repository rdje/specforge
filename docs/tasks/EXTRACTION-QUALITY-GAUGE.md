# EXTRACTION-QUALITY-GAUGE: measure the extraction-quality gap — and CHI's is large

## Metadata

- Tree ID: `EXTRACTION-QUALITY-GAUGE`
- Status: `active` (gauge established + CHI measured; the fix leaves are open R-lane work)
- Roadmap lane: `R15e`/`R16` (extraction quality / production-readiness)
- Created: `2026-06-06`
- Last updated: `2026-09-18`
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
  judge it; that is `.3j.1` and its `.3j.1.a`/`.3j.1.b` split. Beside it sit `.3j.3` (the LLM pass cannot
  see a span the deterministic paths missed — a recall ceiling), the untyped-default successors
  `.3k.2h`/`.3k.2i`, the re-owned `.3k.9`, and the containers `.3`, `.3k`, `.3k.2` and `.FIELD`.
- The persisted 149-record LLM-constraint corpus measures a producer that no longer exists. `.3j.2` ran
  the real membership test over it — 111 exact / 2 case-folded / 0 field / **36 ungrounded** — and dated
  the artifacts to 78 minutes before `declared_signal_catalog` was written, when a subject was typed by
  an LLM judgment stubbed to answer `Signal` and no catalog was consulted at all. Every census over that
  population, `.3j`'s included, therefore measures the pre-catalog rule.
- `.3j.2.a` adjudicated the largest resolver-shaped class and answered **NO to widening, except once**.
  The 16 carried-name refusals are `6 qualifier-only / 3 full-width alias / 1 proper sub-slice / 6 slice
  whose signal states no width`; read against their sources, a general widening would make 4 of 16 records
  correct — below the 3/7 `.3j` already refused — while **strengthening** one obligation and **inventing**
  one subject. Only a slice spanning the whole stated width from bit 0 is an alias for its signal, and
  that is `.3j.2.a.i`'s to wire. `.3j.2.b` (the `PSELx` declaration template) and `.3j.2.c` (a row-keyed
  table obligation that drops the key scoping it) both closed the same way — measure, then answer YES to
  exactly one direction — and their wiring leaves are `.3j.2.b.i` (done) and `.3j.2.c.i` (open).

## Current Frontier

No eligible extraction-quality frontier: every open leaf needs a resource no session can supply.

`.3j.2.c` closed on `2026-09-18` and moved its own diagnosis. It was opened as a within-span scoping
loss: a matrix row's obligations drop the row key that scopes them, and the key is in the span. Measured
over the whole corpus, **109 spans** are matrix rows whose first cell binds a configuration — and **5 of
them bind on a second axis the span cannot see**, because the column header is a different statement. 3 of
LTI's 4 are that shape, so the six records the leaf was opened from are in the class it explicitly ruled
out. The realised defect is **7 records in one cell of one historical document** and nowhere else in 78;
the model carried the row key **0 of 7** times, including the one record that did emit a condition, which
holds the sentence's own predicate instead.
That decided the remedy rather than merely describing it. The measured stratum holds **9** key-scoped rows
and has produced **zero** constraints from them, and cannot produce one through `.3j.4.a` either, so a rule
that *reconstructs* the scope would have no publishable population — the ground on which `.3j` refused
three of five gates. `.3j.2.c.i` therefore wired the other direction and closed the same day: an obligation
is **refused** when its span is a key-scoped matrix row — regardless of any condition, because the
condition test is literal occurrence and the key occurs in the row along with everything else. Reach is
7 of 7, re-derived through the shipped predicate; the A/B observed RED in both directions, refusal and leak.

`.3j.3` then measured the ceiling that bounds this whole family, and it is the most consequential number
the tree has produced since the gauge itself. The LLM pass's universe is the distinct `source_text` of
constraints the deterministic paths already emitted, so it can never see a span they missed. Across the 5
measured documents that declare signals, **326** statements state an obligation about a declared signal and
the model is shown **60** — a ceiling of **18.4%**, with **266** spans that never reach a prompt. Every
`.3j` precision result therefore describes 18% of the population, and perfect model precision cannot move
the other four-fifths: the bottleneck is the deterministic extractors' recall, upstream of anything the
promotion does.

**This tree is now exhausted for in-session work, and that is the state rather than a pause.** Every open
leaf needs a resource a session cannot supply: `.3j.4.a` a running provider (`.3j.1.b` behind it),
`.3k.2h`/`.3k.2i` a legacy re-ingest that `CORPUS-COVERAGE` owns with five documents remaining in its
declared frontier, and `.3k.9` two detached runs — `replay-constraints` plus a full
`scripts/check_doctrines.sh --all`, neither of which finishes inside a session. Nothing is blocked on a
decision. The contract declares no eligible frontier for the same reason, and it also refused to let
`.3k.9` be re-routed into an active part: a legacy leaf's primary detail must stay with its migrated
source, which is the archived capsule's authority working as designed.

`.3k.9` is where the next real work is: a Markdown escape fragments an identifier and the fragment is then
DECLARED as a signal, which corrupts the catalog and so bounds recall — the bottleneck `.3j.3` measured.
`.3k.9.a` discharged the two preconditions that need no corpus rebuild, in the new active
`kind-span-family` part. **All 18 contaminated names adjudicate the same way — remove** — and the risk the
leaf flagged is the evidence for it, not against: `POWER` is declared a signal of eMMC on the strength of
`POWER\_CLASS`, while the document writes the word *power* 361 times and the identifier `POWER` never.
**The circularity control now runs the classification and was observed RED on the real corpus**: with the
synthesized-declaration exclusion disabled the census reports `0 declared name(s) across 0 document(s)` —
the exact wrong answer this leaf's first re-derivation reached. What remains is (a) and (b).
`.3j.4` closed the same day by reading the artifacts its own plan was written about. The measured stratum's
promotable population is **5 documents / 62 provider calls**, not 27 — the recall universe is the distinct
persisted `source_text` and 22 of the 27 have none — and the hazard the leaf opened with is absent: no
measured document carries a quality gauge, and the only seven that do are the seven already-promoted
historical ones. `.3j.4.a` holds the frozen five and is blocked on a provider; it is now `.3j.1.b`'s
prerequisite. Only **2 of the 7** historical documents have a measured counterpart, which is why `.3j.2.c`
was measured on the historical stratum and labelled as dated evidence under `ADR 0048` §6.

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
- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.a`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.a.i`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.a.ii`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.b`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.b.i`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.c`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.2.c.i`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.3`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.4`
- ID: `EXTRACTION-QUALITY-GAUGE.3j.4.a`
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
- ID: `EXTRACTION-QUALITY-GAUGE.3k.9.a`
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
