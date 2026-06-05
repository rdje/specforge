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

## KEEP — behavioral verbs NOT yet in the engine (the shortlist)

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

## BORDERLINE — my calls (please eyeball)

| verb | specs | my call | why |
| --- | --- | --- | --- |
| `transition` | 13 | **keep** | a signal/state transition is edge behavior |
| `generate` / `generated` | 10 | **keep** | a signal/pulse is generated |
| `detect` / `detected` | 8 | **keep** | an edge/condition detected on a signal |
| `indicate(s)` | 12 | keep (already in engine) | signal semantics |
| `poll` | 6 | **keep** | a signal is polled |
| `program` / `configure` | 8 / 4 | **keep** | a register/signal configured to a value |
| `read` / `write` / `access` | 11 / 9 / 4 | **keep (weak)** | register-level value behavior |
| `ignore` / `ignored` | 16 | **lean reject** | "input ignored" is a *don't-care*, not a constraint |
| `send` / `receive` / `transmit` / `forward` | 31 / 6 / 6 / 4 | **reject** | message/data-transfer level, not signal value/timing |
| `start` / `stop` / `count` | 9 / 5 / 4 | **reject** (too generic) | except in a clock/counter context |

## REJECT — not signal behavior (sanity-check these)

Device/system requirements, observations, or non-verbs — **deliberately dropped**:
`use`/`used`, `support`, `describe`, `conform`, `define`, `provide`, `ensure`, `determine`,
`contain`, `include`, `report`, `respond`, `check`, `return`, `perform`, `take`, `control`,
`execute`, `connect`, `treat`, `consider`, `follow`, `apply`, `cause`, `allow`, `require`, `mean`,
and the modals/copulas `is`/`be`/`can`/`must`/`do`.

## Next (`.2`, after your nod)

Map each accepted verb to a constraint kind (e.g. `set`→drive-to-1, `clear`→drive-to-0,
`sample`→a sampling-timing fact, `gate`/`mask`/`enable`/`disable`→assertion-style, `hold`/`maintain`
→stable), **centralize** the now-much-larger normative vocabulary in one maintainable place, and
re-verify with the extraction suite. Grammar only; no names.
