# Verb-coverage corpus mine — candidate report (VERB-COVERAGE-CORPUS.1)

> **For owner review.** Nothing here is wired into the engine yet. This is the curated output of a
> seedless, model-mined pass over the real corpus, for a quick sanity pass — and especially a glance
> at the **Rejected** pile, where a curator's mistake would show.

## How it was produced (no seed)

- Corpus: `/Users/richarddje/Documents/livework/chipdoc` — **82 PDFs, cross-vendor** (I²C, Wishbone,
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

## Two verb sets — signals AND actors

SpecForge extracts behavior for **both** signals/pins/ports **and** actors, so the vocabulary has
two halves: verbs for what a *signal* does (below) and verbs for what an *actor* does (the next
section). Both are in scope.

## KEEP — signal-level behaviors (not yet in the engine)

The engine today recognizes: `asserted/deasserted`, `high/low`, `stable`, `change`, `hold`,
`driven`, `tied`, `indicate`, `valid`. The corpus leans heavily on these too (good) — and on **these
behaviors it does *not* yet cover** (number = how many of 82 specs use it):

| verb (family) | specs | what it constrains |
| --- | --- | --- |
| `set` / `clear` | 58 / 33 | a signal/bit set to 1 / cleared to 0 |
| `reset` | 37 | a signal returns to its reset value |
| `sample` / `sampled` | 28 | a signal is sampled on an edge (timing) |
| `release` / `released` | 23 | a held/driven signal is released |
| `mask` / `masked` | 13 | a signal/interrupt is masked |
| `gate` / `gated` | 12 | a signal is gated (e.g. clock gating) |
| `enable` / `disable` | 11 / 9 | a signal/feature enabled or disabled |
| `toggle` / `toggled` | 9 | a signal toggles |
| `latch` / `latched` | — | a value is latched |
| `pull` (high/low) | 5 | a line is pulled high/low |
| `capture` / `load` / `store` | 5 / 3 / 1 | a value captured/loaded/stored |
| `invalidate` / `increment` / `decrement` | 5 / 4 / — | value/entry ops |
| `activate` / `switch` (on/off) | 4 / 4 | a signal activated / switched |
| `negate` | — | a signal negated (= deasserted) |

These are the clear wins — behavioral, recurrent, and structurally invisible to a `must`/`shall`
regex because specs state them in the active voice (*"the master **drives**…"*, *"the value is
**sampled**…"*).

## KEEP — actor-level behaviors

SpecForge models **actors** (Requester/Completer/Manager/Subordinate…), not just signals — so verbs
for what an *actor does* are equally in scope. **(Owner correction, 2026-06-05 — the first pass was
too signal-centric and wrongly rejected these.)** Rescued + kept:

| verb | specs | role |
| --- | --- | --- |
| `send` / `receive` / `transmit` / `forward` | 31 / 6 / 6 / 4 | an actor sends/receives/forwards a transfer |
| `respond` | 6 | an actor responds (e.g. with a completion) |
| `check` | 6 | an actor checks/observes a signal |
| `start` / `stop` | 9 / 5 | an actor starts/stops a transaction |
| `poll` | 6 | an actor polls a signal |
| `request` / `acknowledge` / `grant` / `issue` / `initiate` | — | handshake / transaction actor verbs |

Also kept (mixed signal/actor, from the earlier borderline): `transition`, `generate`, `detect`,
`program`/`configure`, `read`/`write`/`access`.

## REJECT — not behavioral (device requirements, generic, copulas)

Dropped as device/system requirements, observations, or non-verbs — **skim for a mistake**:
`use`/`used`, `support`, `describe`, `conform`, `define`, `provide`, `ensure`, `determine`,
`contain`, `include`, `report`, `return`, `perform`, `take`, `control`, `execute`, `connect`,
`treat`, `consider`, `follow`, `apply`, `cause`, `allow`, `require`, `mean`, `ignore` (a *don't-care*,
not a constraint), `count` (too generic), and the modals/copulas `is`/`be`/`can`/`must`/`do`.

## Next (`.2`, after your nod)

Map each accepted verb to its target — **two destinations, matching the two sets**:
- **signal-level verbs** → signal constraint kinds (e.g. `set`→drive-to-1, `clear`→drive-to-0,
  `sample`→a sampling-timing fact, `gate`/`mask`/`enable`/`disable`→assertion-style,
  `hold`/`maintain`→stable);
- **actor-level verbs** → the actor–signal relation / contract layer (e.g. `send`/`drive`→*drives*,
  `receive`/`sample`→*observes/reads*, `respond`→*responds*, `start`/`stop`→transaction bounds).

Then **centralize** the now-much-larger normative vocabulary in one maintainable place and re-verify
with the extraction suite. Grammar only; no names. (We don't need 100% precision here — enough
coverage of real chip-spec verbs is the bar.)
