# Is a persisted artifact what the current reader produces? — the legacy stratum, measured

Owning leaf: `CORPUS-CHAIN-CURRENCY.11` (MEASURE + DECIDE; no production rule changed).
Producer: `cargo test -p specforge-core --lib corpus_chain_currency_11 -- --ignored --nocapture`.

## Why this census exists

`scripts/check_chain_currency.sh` is honest about the stratum it cannot replay: **27 replayed, 27
current, 0 stale, 51 UNMEASURABLE**. It never claimed the 51 were current.

The problem is what happens downstream of that honesty. A persisted `evidence_ir.json` exists for
**all 78** documents, and censuses read them: `SIGNAL-DECLARATION-ROW-DROP.2j.1`'s drift census takes
its `declared` column out of exactly these files. `.2h.2` then found one artifact that demonstrably
differs from the current binary — CoreSight TMC `table_0074` records the declaration name `DATA`
where the reader emits `Data` — which turns a general worry into a specific question: **how far has
the legacy stratum drifted from the reader whose output it is taken to describe?**

## Why a whole-artifact comparison is impossible, and why that is a finding

`build_unproved_from_source_ir` reaches the declaration seed only after
`assemble_evidence_statements`, which needs the document's **normalized markdown bundle**. For the
51 legacy documents that bundle has been reclaimed — `check_chain_currency.sh` says so in as many
words: *"normalized bundle reclaimed — needs re-ingest"*. Two of the three declaration producers read
those base statements:

- the sparse-catalog **prose fallback**, gated on the table catalog's size;
- the **trapped-row** pass, whose inventory gate is keyed on the already-declared universe.

Neither can be replayed. So there is no honest whole-artifact answer for the legacy stratum, and
saying so is part of the result.

**One producer can be replayed.** `synthesize_declarations_from_tables` reads
`source_ir.structured_tables` and nothing else, and it is the producer whose output the censuses
actually quote. This census replays that one, per table, against the persisted artifact's own
records for the same table. Tables the body-row pass never reaches are excluded and counted.

## The control fired once, and that is why the number can be trusted

The 27 proof-carrying documents are certified current by the chain-currency oracle. If the
comparison is sound they must agree; a disagreement there means the **method** is wrong.

It disagreed. AXI `ihi0022_l` `table_0011` showed six reader-only declarations — `VALID`, `PENDING`,
`RP`, `CRDT`, `CRDTSH`, `SHAREDCRD`. Those are the base-name **template** table that
`WIRE-BASED-100.10b` withholds: the artifact is what the pass *publishes*, so a comparison that skips
the withholding measures a different thing. With `withhold_base_name_template_declarations` applied
the control reads **0 divergent of 27**.

A control that fires and is then explained is the method being validated. One that fires and is waved
through is the method being asserted.

## The population

```bash
cargo test -p specforge-core --lib corpus_chain_currency_11 -- --ignored --nocapture
```

| | proof-carrying | legacy |
| --- | ---: | ---: |
| documents with both artifacts | 27 | 51 |
| **documents that DIVERGE** | **0** | **22** |

| tables | count |
| --- | ---: |
| compared | 415 |
| identical | 272 |
| **differing** | **143** |
| excluded (not reached by the body-row pass) | 6 |

## The difference, characterised rather than counted

| class | size |
| --- | --- |
| identical but for **name CASE** | **73 tables, 527 declarations** |
| in the artifact, not the reader | 304 declarations |
| in the reader, not the artifact | 163 declarations |
| same name, different sentence | 24 declarations |

**The largest class is one normalisation change, not lost wires.** GIC-600's artifact declares
`CHIP_ID`, `ERR_INT`, `SAMPLE_REQ`; the reader emits `chip_id`, `err_int`, `sample_req`. Counting
those 527 as differences would overstate the drift by the width of several documents, which is why
the class is separated before anything is counted.

The remaining 70 tables carry real differences, and reading the sample they fall into recognisable
shapes:

- **the reader now finds wires the artifact has none of** — GIC-600 `table_0160` (`dftcgen`,
  `dftramhold`, `dftrstdisable`, `dftse`), `table_0162` (the eight P-Channel/Q-Channel wires),
  CoreSight SoC-600 `table_0041` (`traceclk`, `tracectl`): artifact empty, reader populated;
- **the artifact carries phantoms the reader has since refused** — GIC-600 `table_0170` publishes
  `ALLOW`, `ERROR`, `FAULT`, `ID`, `PMU`, `REQUEST`: description prose read as names, where the
  reader now emits the real `err_int`/`fault_int`/`pmu_int` set;
- **a direction the reader no longer asserts** — Cortex-A76 `table_0063`'s sixteen CP15 registers
  read `is input width 32` in the artifact and `is width 32` now: the reader stopped claiming a port
  sense for a register row;
- **this session's own change** — SDC-600 `table_0048`/`table_0059`, from
  `SIGNAL-DECLARATION-ROW-DROP.2h.2`.

## A CURRENT-reader defect this comparison surfaced

Reading the sample turned up something that is not history. GIC-600 `table_0163` and `table_0164`
publish `Signal Input is input.` and `Signal Output is output.` — the reader minted the **direction
word itself** as a signal name. Counted corpus-wide by the same producer: **14 declarations across 5
table/document pairs**, all in GIC-600.

It is a property of the current reader, so it is **owned, not mentioned**:
`SIGNAL-DECLARATION-ROW-DROP.5`.

## The decision this leaf owed

**A persisted legacy `evidence_ir.json` may be cited as evidence about ITSELF, never about the
reader.** On the one producer that can be checked, 22 of 51 documents disagree with the current
binary; the other two producers cannot be checked at all, because their input no longer exists on
disk. Any census that quotes a count out of a legacy artifact must say it is describing the artifact
— or replay the reader, which this module shows how to do for the one pass where that is possible.

**This is deliberately not gated.** A check would have to recognise "a census reading a legacy
`evidence_ir.json`", which is a property of intent rather than of a file, and a rule that cheap would
fire on every legitimate read of the artifact stratum. The proportionate controls are the producer
above, this record, `[[legacy-artifact-declaration-drift]]`, and the caveat now carried in
`docs/research/direction-column-drift-census.md`, whose `declared` column is the known instance.

## Re-derivation

```bash
cargo test -p specforge-core --lib corpus_chain_currency_11 -- --ignored --nocapture
bash scripts/check_chain_currency.sh      # the oracle whose 51 UNMEASURABLE this explains
```
