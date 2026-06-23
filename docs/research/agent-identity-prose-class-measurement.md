# Agent-identity precision on the dense-prose doc class — Lever E measurement

Owning leaf: `KG-ISF-COMPLETENESS.1c` (measurement-first PROBE — read-only, no code change).
Date: `2026-06-23`. Sibling to `docs/research/agent-surface-fidelity-measurement.md` (the `.1`/`.1a`/`.1b`
agent-surface study, measured on the AMBA-style wire/structured docs).

## Why this measurement exists

The `KG-ISF-COMPLETENESS.1a`/`.1b.*` agent-identity gates were designed and measured on the AMBA-style
structured/wire docs, where they are clean (they reject **zero** ≥8-port actors and hold WIRE-BASED-100 at
1.000). The `CORPUS-COVERAGE.2` re-ingest of the host-local corpus surfaced a doc class those gates were
NOT measured on: **dense descriptive-prose specs**. Re-ingest #27 (JEDEC eMMC 5.0) exploded to **153 actors
/ 349 relations** (from a stale-evidence baseline of 20/23), while the re-ingest of the structured DRAM spec
#28 (JEDEC HBM2) **consolidated** 52→38 like the AMBA/CoreSight class. That contrast spun out **Lever E —
agent-identity precision for the dense-prose doc class** (north-star bar #1: every actor a real protocol
agent). This document is the probe that characterizes the explosion structurally and scopes the gate(s),
**before** any code change, per this tree's measurement-first discipline and the owner's
"structural gates, not denylists" steer (`[[feedback_avoid_denylists_prefer_structural]]`).

The measurement is read-only over the persisted `generated/intent_ir/<key>/intent_ir.json` corpus
(78 docs) with the current binary (`target/release/specforge`, built `2026-06-22`). It changes nothing.

## 1. The explosion is a relation-subject extraction-precision problem, not an actors[] prose-mint problem

eMMC #27 has 153 actors. Splitting them by participation:

| class | count | provenance shape | examples |
|---|---|---|---|
| **CONNECTED** (≥1 port AND ≥1 relation) | **148** | minted from a single `REL-INFERRED` relation | `advantage of` (p1/r1), `basic bus` (p1/r1), `B write` (p1/r1), `bit operation` (p2/r2), `actual sector` (p1/r1) |
| pure-unconnected (0 ports / 0 relations) | 5 | `PROSE-PARA`+`SECTION-PHASE`+`TERM-SCAN` | `channel`, `controller`, `source`, `target`, `transmitter` |

The two seams behave very differently:

- The **actors[] prose-mint seam** (`build_actors` Phase-2 role-term scan) contributes only the **5**
  pure-unconnected actors — and those are the **grounded-keep** generic role terms the `.1b.iv` Class-C drop
  deliberately preserves (the completeness north star keeps genuinely-discussed-but-unwired agents). Measured:
  **0** of the 153 carry ONLY the pure-inferred marker, so `.1b.iv` correctly fires zero drops here — it is
  already doing the right thing. This seam is NOT the explosion.
- The **relation-subject seam** (`normalize_relation_actor_name` / `actor_signal_relation_surface`, where
  `.1a`/`.1b.i`/`.1b.iii` live) contributes the **148** connected phantoms. Each is a noun-phrase fragment
  pulled as the subject of a single who-acts-on-what relation — e.g. `"advantage of" reads BACKGROUND`
  (from *"take **advantage of** the BACKGROUND operation"*), `"basic bus"`, `"B write"`. Because the
  **leading token is a noun**, the `.1a` gate (which judges only the first token's part of speech) rightly
  keeps it; because it carries a relation (hence a port), `.1b.iv` (0/0-only) cannot touch it. So it survives
  as a connected phantom actor.

**Conclusion.** Lever E is a **relation-subject extraction-precision** problem on dense prose: the subject
extractor over-captures mid-sentence noun phrases as "the agent acting on signal X". On AMBA's terse
declarative prose the subject IS the agent; on eMMC's descriptive prose it is often just a noun near the
signal mention.

## 2. Structural name-SHAPE classes do NOT separate phantom from real (shape-only drop is DISPROVEN unsafe)

Classifying every actor name by structural shape, across the exploding docs and the AMBA gold docs:

| class | eMMC#27 | HBM2#28 | AHB-gold | AXI-gold |
|---|---|---|---|---|
| `single-lowercase-noun` | 34 (`adapter`,`address`,`bit`,`cache`) | 15 | 10 (`agent`,`bus`,`decoder`,`device`) | 16 (`agent`,`consumer`,`controller`) |
| `other-multiword` (noun-phrase) | 78 (`actual sector`,`B write`,`basic bus`) | 17 | 8 (`address decoder`,`Exclusive Access Monitor`) | 1 (`Non-volatile Memory`) |
| `TRAIL-preposition` (`X of/to/in`) | 25 (`advantage of`,`cache in`,`CMD to`) | 2 | 1 | 0 |
| `TRAIL-auxiliary` (`X is/has`) | 4 (`host has`,`host is`,`field is`) | 1 | 0 | 0 |
| `single-Titlecase` | 10 (`Any`,`Note`,`No`) | 1 | 3 (`Manager`,`Subordinate`,`Multiplexor`) | 4 (`Manager`,`Subordinate`,`Note`) |
| `LEAD-verb` (`.1a` should catch) | 2 | 2 | 2 | 0 |

The decisive observation: **AMBA's real agents occupy the very same shape classes as eMMC's phantoms.**
AHB/AXI `agent`/`controller`/`decoder`/`device` are real single-lowercase-noun agents; `address decoder` /
`Exclusive Access Monitor` are real `other-multiword` agents. So a gate that **drops** an actor on
name-shape alone (e.g. "drop single-lowercase-noun" or "drop multiword") would **destroy real AMBA agents
and fail WIRE-BASED-100**. A shape-only drop is the wrong tool — exactly the over-aggressive structural
move the genericity guardrail forbids. The differentiator must be **grammatical normalization** (rewrite,
not drop) or **participation/grounding**, never shape alone.

## 3. The clean, corpus-safe first gate: extend the `.1b.i` trailing-strip to auxiliaries/prepositions

The `.1b.i` consolidation already strips a TRAILING universal verb or discourse-adverb off a relation
subject (`Subordinate extends`→`Subordinate`, `decoder also`→`decoder`) as a **normalization** (the
relation re-attributes onto the head noun; the fragment disappears). The dense-prose explosion adds two
trailing closed-class tails `.1b.i` does NOT yet strip:

- **trailing preposition** — `advantage of`→`advantage`, `cache in`→`cache`, `CMD to`→`CMD`, `host to`→`host`;
- **trailing auxiliary** — `host has`→`host`, `host is`→`host`, `field is`→`field`, `cache is`→`cache`.

This is the same proven pattern (rewrite, not drop), keyed off universal English grammar (a closed class of
prepositions + auxiliaries), **never a chip/fragment name list** (ADR 0006). Two corpus-wide measurements
make it the clean first gate:

- **Safety (the genericity guardrail) — CLEAN.** Across all **78** persisted IntentIR docs, **zero** actors
  with ≥8 ports are `X <aux/prep>` shaped. So a trailing aux/prep strip **never renames a real
  high-participation agent** anywhere in the corpus — the exact bar `.1a`/`.1b.i` passed ("rejects ZERO
  ≥8-port actors"). The 4 WIRE-BASED-100 gold docs carry no `X <aux/prep>` actor, so the wire gold relation
  surface is structurally untouched.
- **Reach — corpus-wide, not eMMC-specific.** 138 actor names corpus-wide end in a closed-class aux/prep
  (`...of` 39, `...to` 28, `...is` 23, `...in` 7, `...has` 7, `...on` 5, `...with` 5, `...does` 4, …),
  spanning ~17 docs. Notably the **dense AXI+ACE spec `ihi0022_h_c`** (189 actors / 34 trailing candidates)
  and **CHI `ihi0050_g`** (87 / 11) explode too — so the doc class is **dense-prose specs**, AMBA or not,
  not "non-AMBA". (The WIRE-BASED-100 AXI gold is the *separate* terse `ihi0022_l` spec, 21 actors, clean.)

The trailing-strip is a **strict improvement** but **necessary-not-sufficient**: eMMC has only 29 of 153
trailing candidates. For a real-agent head (`host has`→`host`) it is a pure win (relation merges onto the
real agent); for a junk head (`advantage of`→`advantage`) it shrinks/merges the fragment but the residue may
still be a non-agent — which the next sub-lever must address.

## 4. The harder class: single-relation noun-phrase phantoms (deferred — needs its own measurement)

The bulk of the explosion (≈120 of eMMC's 153) is connected noun-phrase fragments (`basic bus`, `B write`,
`actual sector`, `bit operation`) minted from a **single** weak `REL-INFERRED` relation, with a leading
noun that passes `.1a`. Section 2 disproves a shape-only drop. A defensible discriminator must combine
**low participation** (appears in exactly one relation, never an independently-connected agent in the doc)
with **weak grounding** (no declared-signal-table provenance, not in the document's own defined-agent
vocabulary) — analogous to how `.1b.iv` keyed the Class-C drop on a precise provenance marker rather than a
name. But the safe boundary is non-trivial: a genuinely rare-but-real agent can also appear in exactly one
relation, and the completeness north star forbids dropping it. This needs its **own** read-only measurement
(what fraction of single-relation subjects are phantom vs. real rare agents, by provenance shape, across the
dense-prose docs) before any gate. **Deferred-with-trigger** to `.1c.ii`, exactly as `.1b.ii`/`.1b.iv` were
deferred until their safe gate was measured.

## 5. Scoping outcome (what `.1c` hands to its sub-leaves)

- **`.1c.i` — trailing aux/prep strip (clean, landable).** Extend `consolidate_trailing_fragment` /
  `NON_ACTOR_TRAILING_*` to a closed class of prepositions + auxiliaries, same seam/ordering/safety standard
  as `.1b.i`. Corpus-wide safety already measured (0 ≥8-port actors renamed; wire gold untouched). The
  landing slice still owns the live rebuild + WIRE-BASED-100 + `run_ci.sh` + `kg-bench` gating.
- **`.1c.ii` — single-relation noun-phrase phantom precision (deferred).** The larger, harder class; a
  shape-only drop is disproven unsafe (§2); needs a participation+grounding discriminator and its own
  measurement before a gate.

## 6. Honesty note — the book overclaims

`docs/book/src/pipeline/evidenceir.md` currently states the agent-definition capture "can widen across many
documents without minting noise." The eMMC measurement contradicts that for the dense-prose class. The book
is corrected in this same slice with a measured, scoped caveat (the wire/structured reference specs ARE
clean; dense descriptive-prose specs can still mint relation-subject phantoms — Lever E), so the book stays
honest about current shipped behavior even before the gate lands.

## 7. `.1c.i` LANDED (`2026-06-23`) — outcome

The trailing aux/prep strip landed as a new `NON_ACTOR_TRAILING_FUNCTION_WORDS` const in
`consolidate_trailing_fragment`. Verified live: new-binary `evidence→semantic→intent` cascade on eMMC took
**actors 153 → 138** (the four aux/prep `host` variants `host has`/`host is`/`host to`/`host with` fold onto
`host`; 29 phantom names removed); `host.isf` strict-clean. WIRE-BASED-100 = 1.000 on fresh-Pattern
new-binary evidence; `kg-bench` 156/156; `run_ci.sh` GREEN (lib 1704, +2). The eight `host *` variants
collapse to four real residuals (`host selects`/`host stops`/`host tries`/`host wants`) that are trailing
*common verbs* not in the closed `NON_ACTOR_LEADING_VERBS` list — a possible future closed-verb extension,
but risky (a verb that doubles as a device noun must not be admitted), so it is left as a measured residual.

## 8. `.1c.ii` MEASURED (`2026-06-23`) — no clean within-document structural gate; bounded residual

The bulk of the explosion (eMMC: 109 multi-word actors) cannot be gated by a within-document structural rule.
Measured the most promising candidate — a **connectivity fold** extending the `.1b.ii` idea ("fold onto an
agent independently connected in this doc"): rewrite a multi-word relation subject `A … Z` onto its last
content token when that token is an independently-connected single-word agent. The candidate **mishandles the
real cases**, proven on the dense AXI+ACE spec `ihi0022_h_c`:

- **Correct folds (the token IS the head):** `caching Manager`/`initiating Manager`/`snooped Manager`/
  `originating Manager` → `Manager` — these are descriptive references to the real Manager.
- **WRONG folds (the agent is the MODIFIER, not the head):** `Manager component` → `component`,
  `intermediate component` → `component`, `participating component` → `component` — the agent is `Manager`,
  not the noun-phrase head `component`. A fold-on-last-token rule loses the real agent; a fold-on-first-token
  rule would instead break `caching Manager` → `caching`. The agent token's POSITION varies (modifier vs.
  head), so no fixed structural position is safe.
- **Junk either way:** `full AXI` → `AXI`, `AxDOMAIN signal` → `signal`, `read barrier` → `barrier` — folds
  onto a non-agent tail.

The deeper reason: within one document the real descriptive reference (`caching Manager`) and the phantom
fragment (`basic bus`) are **structurally indistinguishable** — same single-`REL-INFERRED` participation,
same `SECTION-PHASE`/`PROSE-PARA` provenance markers (a relation's statement always sits in some section, so
the phantom inherits section-phase grounding too), same noun-phrase shape. eMMC carries no first-class
`ProtocolActorRecord` agent-definition surface to lean on (its EvidenceIR actor surface is *only*
`actor_signal_relations`), so there is no grounding signal that separates them.

**Conclusion — `.1c.ii` is a bounded residual, not a downstream gate.** A name-shape or connectivity drop is
disproven unsafe (it would lose real agents like `Manager component`'s `Manager`); the completeness north
star forbids dropping a rare-but-real agent. The genuine fix is **upstream relation-subject extraction
precision on descriptive prose** — reading the actual grammatical subject of a who-acts-on-what sentence
rather than a noun phrase near the signal — which is the owner-directed in-Rust shallow-parse direction
(`[[project_nlp_shallow_parse_direction]]`), not a downstream actor-surface rule. Until that lands, the
multi-word prose phantoms stay an honest residual; they never reach the emitted `.isf` (the adapter lowers
the renderable initiator's signals/behaviours, not the raw `actors[]`), so the cost is IntentIR `actors[]`
precision (bar #1) on dense-prose specs, explicitly bounded here. **Lever E is thereby fully scoped: the
clean structural win (`.1c.i`) is landed; the remainder is upstream-NLP-gated and recorded as a residual.**
