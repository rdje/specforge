# Verb-coverage corpus mine — candidate report (VERB-COVERAGE-CORPUS.1)

> **For owner review.** Nothing here is wired into the engine yet. This is the curated output of a
> seedless, model-mined pass over the real corpus, for a quick sanity pass — and especially a glance
> at the **Rejected** pile, where a curator's mistake would show.

## How it was produced (no seed)

- Corpus: `<owner local chip-doc corpus>` — **82 PDFs, cross-vendor** (I²C, Wishbone,
  CCIX, OpenCAPI, Intel VT-d/SDM, AMD IOMMU, Avalon, USB, …).
- `pdftotext` → broad **signal-presence** sentence pre-filter (a sentence with an uppercase
  signal-like token; *not* a verb seed) → up to 150 evenly-spaced sentences/spec.
- **qwen2.5:14b-instruct** read those sentences and listed the verbs describing *signal behavior* —
  with **no `must`/`shall` seeding**. ~39 min, all 82 specs.
- **Grounded:** every returned verb was verified to actually occur in the corpus text (no
  hallucinations kept). Raw result: **640 distinct grounded behavioral verbs.**
- **Curated by Claude** (a *different* model from qwen — genuine cross-model review): morphological
  variants collapsed, classified, diffed against the engine. Verbs are grammar, never names (ADR
  0006).

## The verbs are actor → signal relationships (KG edges)

A signal never behaves on its own — **something is always *done to* it, and the doer is an actor.**
So every behavioral verb is really an edge `actor —verb→ signal`, which is exactly what the
**Knowledge Graph** represents (that is *why* the KG exists). The signal-vs-actor split is not the
useful axis; the useful one is the **relation kind** — does the actor *source/change* the signal
(**Drives**) or *observe* it (**Reads**)? The verb is the edge label, so richer verb coverage = a
more complete KG. These go into the engine's relation extraction (`ACTIVE_DRIVES_VERBS` /
`ACTIVE_READS_VERBS`), not a separate signal table.

## KEEP — Drives edges (actor sources / changes / controls a signal)

`set` · `clear` · `reset` · `toggle` · `negate` · `release` · `enable` · `disable` · `mask` ·
`gate` · `pull` · `load` · `store` · `write` · `send` · `transmit` · `forward` · `respond` ·
`request` · `acknowledge` · `grant` · `issue` · `initiate` · `control` · `determine` · `activate` ·
`switch` · `generate` · `invalidate` · `increment`/`decrement` · `transition`
*(already engine drive-verbs: `drive`, `assert`, `provide`, `apply`, `set`/`sets`, `source`,
`output`, `supply`, `produce`, `present`, `place`, `return`)*

## KEEP — Reads edges (actor observes / samples a signal)

`sample` · `poll` · `check` · `receive` · `capture` · `detect` · `read` · `access`
*(already engine read-verbs: `read`, `sample`, `monitor`, `accept`, `receive`, `capture`, `observe`,
`detect`, `check`, `latch`)*

> These were structurally invisible to a `must`/`shall` regex — specs state them in the active voice
> (*"the master **drives** …"*, *"the completer **samples** …"*). `start`/`stop` are kept as
> transaction-boundary actor verbs (a coarser edge than Drives/Reads).

## REJECT — verbs that do NOT establish an actor → signal edge

These describe the *spec or device*, not an actor acting on a signal — **skim for a mistake**:
`use`/`used`, `support`, `describe`, `conform`, `define`, `ensure`, `contain`, `include`, `perform`,
`take`, `execute`, `consider`, `follow`, `cause`, `allow`, `require`, `mean`, `count` (counter-only),
and the modals/copulas `is`/`be`/`can`/`must`/`do`.

**Rescued from an earlier wrong reject (they ARE actor→signal edges, → Drives):** `provide`,
`apply`, `control`, `determine`, `report`. **Borderline, left out for now:** `ignore` (a *don't-care*
edge), `connect` (a structural/connectivity relation, not Drives/Reads).

## Next (`.2`, after your nod)

The extraction path is one model: **recognize actors + signals + normative verbs, and capture the
`(actor —verb→ signal)` relations between them** — which *is* the Knowledge Graph. So integration is:

- **Add each verb to the relation extraction** (`ACTIVE_DRIVES_VERBS` / `ACTIVE_READS_VERBS`) — the
  KG edge labels. **Done for the clear ones in this slice** (`set`/`clear`/`reset`/`toggle`/`send`/
  `transmit`/`forward`/`respond`/`request`/`acknowledge`/`grant`/`control`/`determine`/`mask`/`gate`/
  `pull`/`enable`/`disable`/`release`/`load`/`store`/`write`/… → Drives; `poll`/… → Reads).
- Where a verb also pins a **value** (`set`→1, `clear`→0), feed the signal-constraint side too.
- **Centralize** the now-much-larger normative-verb vocabulary in one maintainable place; re-verify
  with the extraction suite.

Grammar only; no names (ADR 0006). Enough coverage is the bar, not 100% precision — fine-tune later.
