# Adopt / defer ledger — what SpecForge takes from each author, and what it leaves out

*Tree: `SPEC-MINING-PROVENANCE`. This sharpens the per-aspect grounding
(`docs/research/grounding/*.md`) into per-**author** provenance. For each leveraged
author/work: **Take** (the abstraction SpecForge adopts), **Leave out (+why)** (what of their
work SpecForge deliberately does not use, and the reason — so it is a recorded decision, not
an oversight), and **Instantiated at** (where the adopted idea lives in SpecForge). Every
citation is reused from the verified per-aspect grounding docs; none is invented.*

## SpecForge is *specification mining* — run forward

**Specification mining** (Ammons, Bodík & Larus, *Mining Specifications*, POPL 2002,
DOI [10.1145/503272.503275](https://doi.org/10.1145/503272.503275)) is the discipline of
**automatically discovering the formal specification a system obeys**, motivated by the fact
that engineers rarely write formal specs by hand. The literature runs it **backward**:
recover a spec *from an implementation* — execution traces, RTL, or source code.

**SpecForge runs it forward.** Its input is the *human-authored specification document*
(PDF prose + tables + figures); it mines typed design **intent** *from the spec itself*,
before any implementation exists. So SpecForge is **forward specification mining** —
spec → intent, not implementation → spec. That direction is the genuine novelty; the
abstractions below are what SpecForge borrows from the (mostly backward) spec-mining
literature and recombines for the forward case.

---

## Temporal trio — the `G(antecedent → consequent)` property template

### Pnueli — *The Temporal Logic of Programs*, 18th IEEE FOCS, 1977 (DOI [10.1109/SFCS.1977.32](https://doi.org/10.1109/SFCS.1977.32))

- **Take.** **LTL** as the formalism for temporal/protocol behaviour — the operators `G`
  (globally/always), `F` (eventually), `X` (next), and the `G(antecedent → consequent)`
  invariant shape. SpecForge's temporal contracts *are* LTL/MTL-shaped properties.
- **Leave out (+why).** Full LTL with arbitrary operator nesting (`U` until, `W` weak-until,
  `R` release) and, above all, temporal-logic **model checking / satisfiability**. SpecForge
  uses a *restricted* `G(ante → X / F[min,max] cons)` template and does **not** check or prove
  anything. *Why:* the restricted template covers the dominant chip-spec timing-obligation
  shape and stays grounded in what the spec actually states; proving properties is the
  downstream verifier's (FSMGen / sim) job, not the miner's.
- **Instantiated at.** `TemporalRuleRecord` (`crates/specforge/src/ir/semantic.rs`); the LTL
  renderer `crates/specforge/src/ir/temporal_ltl.rs` (`G` / `X` / `F[min,max]`);
  `TemporalPredicateRecord`. See `protocol-temporal-semantics.md`, `temporal-rule-ltl-rendering`.

### Vasudevan, Sheridan, Patel, Tcheng, Tuohy & Johnson — *GoldMine*, DATE 2010

- **Take.** The `G(antecedent → consequent)` LTL **assertion template with holes** as the unit
  of mined behaviour; automatic high-coverage assertion generation; and **precision/recall
  evaluation of mined assertions** against a reference.
- **Leave out (+why).** GoldMine mines from **RTL + simulation traces** via *data mining*
  (decision trees) + static analysis of the design. SpecForge mines from the **spec**
  (forward), with no RTL/traces and no machine-learning-over-runs — deterministic patterns +
  a bounded LLM over prose. *Why:* at SpecForge's stage no implementation/traces exist; the
  novelty is forward (spec → property).
- **Instantiated at.** `temporal_rules` as `G(ante→cons)` templates; the precision/recall
  methodology is realized as the `TEMPORAL-RULE-EVAL` tree (supervised P/R/F1 of the mined
  temporal rules).

### Lemieux, Park & Beschastnikh — *Texada: General LTL Specification Mining*, ASE 2015

- **Take.** **LTL property templates with "holes"** instantiated from a corpus — mining
  *general* LTL properties via templates rather than a handful of fixed patterns.
- **Leave out (+why).** Texada mines from execution **traces/logs** with **user-supplied,
  arbitrary** LTL templates + support/confidence thresholds. SpecForge uses a **fixed,
  domain-typed** template set (drive / stable / valid / sample / handshake) over spec
  **prose** — not arbitrary user templates over traces, and no support/confidence over a
  trace corpus. *Why:* forward (spec, not traces); a fixed protocol-domain ontology keeps the
  mined properties typed and grounded; the "corpus" is one document's prose, not a log set.
- **Instantiated at.** The typed `TemporalPredicateRecord` template set + the
  `G(ante→cons)`-with-holes shape; `ir/temporal_ltl.rs`.

---

*Remaining swept authors (Ammons-method/Daikon; Docling/TableFormer/DocLayNet/PubTables-1M;
OpenIE/ReVerb/Mintz/Hogan; LayoutLM/Donut/Dempster; GCD/Outlines/RAG/SNLI/SelfCheckGPT/
Garcez-Lamb; Yarowsky/Riloff-Jones/NELL/Snorkel; van Rijsbergen/MUC/Cohen/Chao;
Chow/El-Yaniv/Vovk/Guo/Scheirer; IEEE PSL/SVA/AssertLLM/HLS) are folded into this ledger in
`SPEC-MINING-PROVENANCE.3`, each with the same three fields + its verified citation, followed
by a synthesis.*

## Links

- Task-tree: `SPEC-MINING-PROVENANCE`. Per-aspect grounding: `docs/research/grounding/*.md`
  (esp. `protocol-temporal-semantics.md`). KM: `spec-mining-framing`,
  `temporal-rule-ltl-rendering`.
