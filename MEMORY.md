# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`SIGNAL-DECLARATION-ROW-DROP.1c` CLOSED `2026-09-11`** — the proof-seal probe asked one document per distinct seal and called that
  "not a sample". It is one: `--total` now probes every in-scope artifact at every non-terminal stage (108 probes, **19m02s**), registered CI-tier as
  `PROOF-SEAL-TOTAL`; the gate keeps sampling and now says so. Open in this tree: `.2c`/`.2d`/`.3`. Also open: `RETAINED-BUNDLE-POPULATION-FROZEN`
  `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`; `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`;
  `SOURCE-IR-REPRODUCIBILITY` `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`;
  `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`; `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`;
  `CHANGES-LEDGER-ROLLOVER.4`; `WIRE-BASED-100` `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD `13/29`).
- Current state: **five slices; the largest finding is that two independent gates were reporting green over real breakage.** `.1` had left 4 of 27
  proof-carrying documents unloadable — every wire-bearing one — so `eval-extraction` refused every gold for three commits (`.1b` repaired it; every
  published wire number now re-derives, chain-currency green, kg-bench 156/156). `.1a` fixed a clippy failure the doctrine driver cannot see; `.1c`
  fixed the sampled probe that hid `.1b`. `.2a`/`.2b` shipped two extraction rules, each decided by adjudicating its own selection first. **15
  registered doctrines** now, 13 executed at gate tier. Census 27 measurable / 51 legacy.
- Next action: `[[SIGNAL-DECLARATION-ROW-DROP]]`.2c — the enumerated legal-width set (`8, 16, 32, 64, …`), **7 cells in 2 documents**. Decide the
  representation in the leaf before writing code: `WidthHint` is `Numeric(u32)` or `Parametric(String)` and a set is neither — picking a member
  fabricates, and carrying the verbatim text puts commas into a declaration sentence SemanticIR parses. Payoff is width fidelity, not recall: after
  `.2b` these rows already carry a direction.
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **a green score is evidence only about the facts its gold names**; **a green GATE can be evidence about one
  sampled document** (`[[evidence-rule-field-content-stales-every-proof]]`) — read what a check actually asked before reading its PASS. **Measure a
  candidate rule corpus-wide before shipping, and look at what it selects, not how many** (`[[base-name-template-table-is-not-a-catalogue]]`); every
  cheap rule this session over-fired until a second condition was added. **Changing the CONTENT of an already-registered EvidenceIR rule field stales
  every persisted proof without moving the ruleset seal**, and `rebuild_stage_cascade.sh` refuses a deliberate content delta by design — run the
  stages directly, one validate per artifact, upstream-to-downstream, bundles restored from `generated/preserved/` and returned byte-identical
  (`[[retained-bundle-population-is-frozen]]`). **The doctrine driver runs no cargo gate** (`[[doctrine-driver-runs-no-cargo-gate]]`): run
  fmt/clippy/test in-session or the claim is unbacked. **A relation can become a declaration** (`[[alpha-variant-placeholder-is-not-a-wire]]`). **An
  identifier may bind to another only when the document says so twice** — ADR 0037 forbids the spelling guess
  (`[[document-stated-identifier-coreference]]`). **The book can carry a false current-behaviour claim for a month**; no gate covers behavioural
  prose. **A SourceIR classification rule may not read its neighbours** (`[[sourceir-classification-is-per-record]]`); two readers of one datum must
  tokenize identically. **Do not predict a re-derivation's verdict.** Never infer ownership from a mention; attribute a regression from producer
  history, never a diff; never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). A live-surface edit sets off
  `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines; a new fact card needs `check_fact_card_catalog.pl --print-plan` folded into
  the contract before `--write` runs.
