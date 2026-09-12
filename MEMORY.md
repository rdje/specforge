# MEMORY — resume pointer (layer A of `MEMORY_ARCHITECTURE.md`; overwrite-only, keep small)

> The bounded **resume pointer**: it describes *now*, not the journey. Overwrite the "Current state"
> block below; never append session narration. How to resume — bootstrap order, doctrine, gates, and
> retrieval — is `AGENTS.md`; the layer contract is `MEMORY_ARCHITECTURE.md`. Everything else is derived
> on read: revision from `git rev-parse HEAD`, work state from `docs/tasks/`, history from `git log`.

## Current state (OVERWRITE this block each update — do not append)

- Active unit: **`PRODUCTION-GRAPH-CENSUS-PIN.2a` — a CORRECTION to `.2`, committed the same day (`2026-09-12`).** `.2` published "12 of 16 checks
  cannot detect a change in their own self-test coverage" and named `$passed/$total` as the remedy. **Both halves were wrong.** Hand-re-derived: **3
  guarded, 13 not**, and `$passed/$total` is NOT a coverage guard — `$total` is incremented in the same case loop as `$passed`, so deleting a case drops
  both and the ratio stays `N/N`. The only guarded shape is a **literal** expected total (`$passed != 13`, `-ne 22`), which three checks already use and
  `.2` had filed as unguarded because it judged the PRINT line instead of looking for the guard.
  Open: **`PRODUCTION-GRAPH-CENSUS-PIN.3`** (13 remediations; brief corrected — declare a literal, not `$passed/$total`; the true population is 31 report
  lines, not 16); **`EXTRACTION-QUALITY-GAUGE.3k`** (scope first) / **`.3j`**; `INVARIANT-SHAPE-ADMISSION.4`; `PROSE-NAME-CELL-DECLARATION.3`;
  `SIGNAL-DECLARATION-ROW-DROP` `.2c`/`.2d`; `RETAINED-BUNDLE-POPULATION-FROZEN` `.1`-`.3`; `KG-ISF-COMPLETENESS` beyond `.5`;
  `LIVE-DOCUMENT-PRESSURE-HEADROOM` `.1`/`.3`/`.4d.ii`/`.6`/`.8`-`.13`/`.14b`/`.14c`/`.15`-`.18`; `SOURCE-IR-REPRODUCIBILITY`
  `.3`/`.4`/`.6`/`.7`/`.9a`/`.10`/`.13`; `CLAIM-VERIFICATION-ADOPTION` `.8`/`.9`/`.12`/`.13`; `SCRATCH-RESIDUE-CONTAINMENT.1`/`.4`;
  `STATUS-LEDGER-ROLLOVER.2`; `PROVIDER-MODEL-STORE-LOCALITY.1`; `TASK-PART-SEAL-REACHABILITY.0`; `CHANGES-LEDGER-ROLLOVER.4`; `WIRE-BASED-100`
  `.10d`/`.10f`/`.2`/`.3`/`.5` (SWD `13/29`).
- Current state: **the census producer is now forbidden to classify.** Two regex classifiers were written for the coverage-guard property and both were
  wrong — the first judged the print format, the second gave false positives (`$lines != $parts`) and a false negative on a hand-verified case. The
  producer emits report lines + candidate guards as EVIDENCE; the verdict lives in the task leaf. That is `.2`'s own acceptance criterion, which `.2`
  itself violated by letting a regex do the adjudicating. 15 registered doctrines, 13 at gate tier. Census 27 measurable / 51 legacy.
- Next action: **`PRODUCTION-GRAPH-CENSUS-PIN.3`** — remediate the 13, starting with `check_persisted_artifact_paths.pl`, the one remaining
  literal-print check, which prints `15/15` and runs **18** named self-tests (12 `@cases` + 6 standalone). The remedy is the one `.2a` demonstrates on
  `check_corpus_kb_currentness.pl`: count in the assertion helpers, compare against a declared literal beside the suite, observed RED at exit 2. **Do not
  copy `$passed/$total`.** Also enumerate the rest of the 31 report lines: the 16 `.2` adjudicated were not the whole surface.
- In-flight uncommitted: none after this commit.
- Blockers: none. Standing hazards: **an obligation binds to the token immediately before its modal** — `HBURST_WIDTH must be …` is not about `HBURST`.
  **A subjectless clause defeats a subject scan by having no subject to find**: `collect_subject_signal_tokens(" Must be valid")` returns
  `Must`/`be`/`valid`, all identifiers, which suppresses the full-text fallback. **`specforge evidence` says only `path does not exist:
  …/normalized/<key>.md` — the bundles are in `generated/preserved/WIRE-BASED-100.10/`** (`[[evidence-rule-field-content-stales-every-proof]]`); `.3`
  wrongly concluded the chain was unrebuildable before grepping the question shards for it. **Grep `KNOWLEDGE_MAP.md` for the wall, not just for the
  feature.** **A leaf's own stated bar can be falsified by its population** — measure before trusting it. **A table row's subject lives in its header,
  which serialization throws away.** **A number that justifies a rule must be counted over the rule's whole population.** **A check that prints a
  number is not a check** (`PRODUCTION-GRAPH-CENSUS-PIN`). **The doctrine driver runs no cargo gate**
  (`[[doctrine-driver-runs-no-cargo-gate]]`) and `check_chain_currency.sh` is CI-tier, not gate-tier. An evidence-stage change stales every persisted
  proof whose content moves (`[[evidence-rule-field-content-stales-every-proof]]`); a semantic-stage change needs no bundle
  (`[[retained-chain-rebuild-order]]`). Never run the fixture suite with the locality gate (`SCRATCH-RESIDUE-CONTAINMENT.4`). A live-surface edit sets
  off `[[live-surface-edit-bookkeeping-chain]]`; this file's cap is 50 lines.
