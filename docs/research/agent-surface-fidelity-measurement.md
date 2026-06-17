# Agent-surface fidelity — measured baseline and design (`KG-ISF-COMPLETENESS.1`)

Read-only measurement over the persisted wire-doc IR (`2026-06-16`), executed before any
code per the tree's measurement-first rule. It characterizes the agent-surface defects the
owner's north star targets (the KG/IntentIR must capture all *real* agents and connect them
to their signals), decides the disconnected-agent cause by evidence, and fixes the design for
the `.1a`/`.1b` code slices.

Method: `python3` over `generated/intent_ir/<doc>/intent_ir.json` (actors / `actor_ports` /
`actor_signal_relations`) joined to `generated/evidence_ir/<doc>/evidence_ir.json`
(`extracted_statements`) for the source text behind each relation. No binary, no model, no
mutation — RAM-safe.

## 1. Per-actor port/relation census (wire docs + AXI-Stream guardrail)

Each row is one `actor_name`; `ports` = `actor_ports` count, `rels` = `actor_signal_relations`
count for that actor in that document.

**AXI** (`ihi0022_l`, actors=24, ports=371, rels=348)
- Real, connected: `Manager` 169/168, `Subordinate` 168/166, `interconnect` 5/3.
- Precision noise (carry ports, not agents): `For` 4/2, `with write` 4/2, `exclusive` 3/1,
  `instruction` 3/1, `monitor` 3/1, `Non-volatile Memory` 3/1, `Note` 3/1, `Shareable` 3/1,
  `Then it` 3/1.
- Zero-evidence (0/0): `agent`, `channel`, `consumer`, `controller`, `device`, `producer`,
  `receiver`, `requester`, `server`, `source`, `target`, `transmitter`.

**APB** (`ihi0024_e`, actors=10, ports=83, rels=69)
- Real: `Requester` 32/30, `Completer` 32/30.
- Precision noise: `APB protocol` 4/2, `ensures` 4/2, `Exit from` 4/2, `For` 4/2,
  `three levels` 3/1.
- Zero-evidence: `controller`, `source`, `state machine`.

**AHB** (`ihi0033_c`, actors=25, ports=100, rels=66)
- Real: `Subordinate` 25/23, `Multiplexor` 9/7, `Manager` 8/6, `Exclusive Access Monitor` 4/2.
- Fragment of a real agent (consolidate): `Subordinate and decoder` 6/4 (split),
  `address decoder` 4/2 (→`decoder`), `decoder also` 4/2 (→`decoder`),
  `Exclusive Access Monitor and Subordinate` 4/2 (split), `Subordinate extends` 4/2
  (→`Subordinate`), `Subordinate then` 1/1 (→`Subordinate`).
- Precision noise (junk): `For components` 4/2, `HPROT bit` 4/2, `is recommended` 4/2 (real
  subject `Manager` is inside the that-clause), `next` 4/2, `next access` 4/2, `section` 4/2,
  `two-cycle response` 4/2, `bus` 3/1.
- Zero-evidence: `agent`, `channel`, `controller`, `decoder` (**real but stranded** — see §3),
  `device`, `receiver`, `source`.

**SWD / ADIv6** (`ihi0074_a`, actors=23, ports=23, rels=26)
- Real-ish: `debugger` 3/3, `target` 2/4, `DP` 2/2.
- Noise/fragment: `details about` 2/2, `DP Access` 2/2 (→`DP`?), `IR register` 2/2,
  `number of` 2/2, `Class x` 1/2, `DbgSwEnable flag` 1/1, `field` 1/1, `read-only field` 1/1,
  `system has` 1/1, `then` 1/1, `watcher circuit` 1/1, `serializer` 1/1.
- Zero-evidence: `agent`, `controller`, `device`, `host`, `requester`, `sink`, `source`,
  `state machine`.

**AXI-Stream** (`ihi0051_b`, the genericity guardrail) — actors=9, ports=59, rels=54
- Real, connected: `Transmitter` 22/23, `Receiver` 22/22.
- Noise/fragment: `is permitted` 7/5, `Transmitter interface` 4/2 (→`Transmitter`),
  `TREADY input` 4/2 (junk — a signal, not an agent).
- Zero-evidence: `agent`, `channel`, `controller`, `source`.

## 2. The defect taxonomy (three structural classes)

Resolving each noise actor's relations to source text shows the noise is not one phenomenon:

- **Class A — Junk** (no agent: the captured "subject" is a clause / function-word /
  descriptor). `For components` ← "*For components that support…*"; `HPROT bit` ← "*each HPROT
  bit*"; `is recommended` ← "*It is recommended that a Manager sets HPROT[0] HIGH*" (the real
  subject `Manager` is in the that-clause); `section`, `next`, `next access`, `bus`,
  `two-cycle response`, `TREADY input`, `is permitted`, `Note`, `Then it`. → **reject**
  (precision).
- **Class B — Fragment of a real agent** (subject + trailing verb/adverb, or a coordinated
  "X and Y"). `Subordinate extends`/`Subordinate then` → `Subordinate`; `address decoder`/
  `decoder also` → `decoder` ("*An address decoder provides HSELx*" is a **genuine** AHB
  fact); `Transmitter interface` → `Transmitter`; `Subordinate and decoder` → split. →
  **consolidate** (completeness).
- **Class C — Zero-evidence role-term** (0 ports AND 0 rels; minted by the SemanticIR Phase-2
  role-term scan, `build_actors`, semantic.rs:2727). Generic vocabulary the document mentions
  but never connects to a wire. → see §3.

## 3. Decisive measurement: the disconnected agents are NOT recoverable (no source evidence)

The tree's `.0` hypothesis was *"real agents `producer`/`consumer`/`receiver`/`transmitter`
are named but never connected — a relation gap to fill."* Measured and **corrected**: across
all four wire docs, a Class-C actor appears as the subject of a drive/read verb next to a
**known signal** essentially **zero** times:

- The only AXI "hit" is `source` in "*Even if a **Subordinate** has only one source of read
  data, it must assert the RVALID signal*" — `source` is the noun "source of read data"; the
  verb's real subject is `Subordinate`. False match.
- The only SWD "hit" is `state machine` in "*this state machine is in either the
  Test-Logic-Reset state…*" — it is *in a state*, not driving a wire. False match.
- `transmitter` (AXI) has 8 subject-of-verb mentions but **0** beside any AXI signal — the AXI
  doc discusses it generically (coherency/DVM prose), never as a port owner.

So for the wire docs, Class-C terms are **generic-vocabulary false positives**, not
disconnected real agents. "Connecting" them to signals would be **fabrication** (no grounding;
violates ADR-0006 honesty). The genuine completeness win is **consolidation** (Class B): AHB
`decoder` reads 0/0 only because its real relations (`drives HSELx`) are stranded under the
fragment forms `address decoder` and `decoder also`.

## 4. Genericity guardrail (proven): the rule must be per-doc evidence-keyed, never a name list

The same token flips status by document: `transmitter`/`receiver` are zero-evidence phantoms
in **AXI** (0/0) but fully-connected real agents in **AXI-Stream** (`Transmitter` 22/23,
`Receiver` 22/22). Therefore a Class-C drop rule MUST be **"0 ports AND 0 relations in *this*
document"** — purely structural/evidential, with **no global term list** (ADR 0006). That rule
drops AXI's phantom `transmitter` while keeping AXI-Stream's real `Transmitter`.

## 5. Design for the code slices (each measurement-first, WIRE-BASED-100 a hard gate)

- **`.1a` — precision structural agent-identity gate.** At the relation-subject extraction
  point (evidence.rs `extract_subject_phrase` / `extract_actor_phrase` /
  `normalize_relation_actor_name`), reject Class-A subjects by *structure*: function-word /
  verb / adverb-led, or a clause fragment, or a descriptor that is not an agent-shaped noun
  (`For`, `Then it`, `is recommended`, `next`, `section`, `two-cycle response`, `HPROT bit`,
  `TREADY input`, `is permitted`). Universal grammar only — characterize the shared structure
  (leading non-agent token classes) and gate on it, not on names.
- **`.1b` — consolidation + zero-evidence honesty.** (i) Normalize Class-B fragments to the
  canonical agent token (strip a trailing verb/adverb; map "X interface"→"X"); (ii) split a
  coordinated "X and Y" subject; (iii) drop Class-C zero-evidence actors **per this doc's
  evidence** (0 ports AND 0 rels) — first re-checking their provenance (`responsibilities`,
  any non-drive/read "defined-as-agent" grounding) so a genuinely-declared-but-unwired agent
  is preserved. Explicitly **do not** synthesize relations for Class-C terms (measured
  fabrication risk).

## 6. What this does NOT claim

This measured only the four wire docs + AXI-Stream (the deeply-extracted protocol set the bar
gates on). The broader protocol corpus is scoped for `.2+`. The taxonomy and the per-doc
evidence-keyed rule are expected to generalize, but each code slice re-measures on its own
gold before landing.

## 7. `.1b` consolidation/honesty measurement (`2026-06-16`, read-only over all 36 persisted IntentIR docs)

Executed before the `.1b` code, per the tree's measurement-first rule. `python3` over
`generated/intent_ir/<doc>/intent_ir.json` (actors / `actor_ports` / `actor_signal_relations`),
modelling each `.1b` transform corpus-wide. **The persisted corpus is the PRE-`.1a` baseline** —
the `.1a` rebuild materialized into a temp evidence-root, not the canonical `generated/` tree
(the WRITE-PATH GOTCHA), so the pre-`.1a` fragment actors are still visible here, which is exactly
what makes the `.1b` transforms measurable.

### 7.1 The original `.1b` (i) is two different risk profiles

Modelling the trailing-strip + `"X interface"` strip across the corpus separates them:

- **Trailing verb/adverb strip — grammatically unambiguous, the clean win.** A trailing token in
  `NON_ACTOR_LEADING_VERBS` (`extends`/`describes`/`contains`/…) or an adverb/discourse-marker
  (`then`/`also`/`next`/`where`/`only`/…) is never part of an agent name; stripping it keeps the
  leading noun. Ordered net effect (consolidate → `.1a` reject) over the corpus: **ZERO real agents
  (≥8 ports) vanish**; the wire-doc completeness wins land — AHB `Subordinate extends` 4/2 +
  `Subordinate then` 1/1 → `Subordinate` (25/23 → merges); AHB `decoder also` 4/2 → `decoder`;
  AXI-and-ACE `Subordinate interface` only via the interface rule. Residual junk consolidates to a
  function-word head the subsequent `.1a` reject removes (`does not`→`does`, `It also`→`It`,
  `is used`→`is` — net-better). Descriptor-noun residue (`chapter describes`→`chapter`,
  `section describes`→`section`, `channel(s specified`→`channel(s`) stays the deferred descriptor-noun
  class — no worse than before. → **`.1b.i`** (this slice).
- **`"X interface"→"X"` strip — named-block conflation risk.** On the wire set it is a clean merge
  (`Transmitter interface`→`Transmitter` 22/23; `Subordinate interface`→`Subordinate` 51/49). But on
  GIC `CPU interface` (the GICC, a distinct architectural block) → generic `CPU` (0/0) is WRONG, as is
  `Q-Channel interface`→`Q-Channel`, `AXI interface`→`AXI`. The safe rule needs an "only when the
  leading token is already a real connected agent in this doc" sub-gate — actor-set context the
  relation-subject seam (`normalize_relation_actor_name`, a pure string fn) does not have. → **`.1b.ii`**,
  deferred until that context is wired in.

### 7.2 Coordinated "X and Y" subjects — `.1b.iii`

AHB `Subordinate and decoder` 6/4 and `Exclusive Access Monitor and Subordinate` 4/2 are coordinated
subjects whose relations belong to BOTH agents ("*the Subordinate and decoder drive HSELx*" ⇒ both).
A split must duplicate (not move) the relations to each conjunct, then dedup against existing per-agent
relations. Distinct enough from the trailing strip (and conjunction-led, which `.1b.i` deliberately
excludes from the trailing set) → its own leaf.

### 7.3 Class-C zero-evidence drop is far broader than the wire-doc scope — `.1b.iv`, deferred

Census of 0/0 actors corpus-wide: **320 total**, split by responsibility provenance into **21
PURE-INFERRED** (only "semantic role inferred around `X` evidence" — the unambiguous Phase-2 role-term
phantom), **76 SECTION+INFERRED**, and **223 PROSE-GROUNDED** (a real prose sentence attached as a
responsibility, e.g. GIC `arbiter`). The `.1`/`.1a` measurement validated "drop 0/0 per-doc" only on the
4 wire docs; a blanket corpus-wide 0/0 drop would delete 320 actors including ones the documents
genuinely discuss (TileLink `sender`/`sink`, NVMe `host`, GIC `arbiter`), against the owner's
completeness north star. "PROSE-GROUNDED" is NOT a clean "genuinely-declared agent" discriminator
(it can be a generic-vocabulary noun mention, exactly the wire-doc `source`="source of read data"
case). So `.1b.iv` must first design a defensible re-check — most likely drop only the PURE-INFERRED
phantoms, or keep the drop wire-doc-scoped — and re-measure on its own gold. Not forced into `.1b.i`.

## 8. `.1b.iv` decision measurement (`2026-06-17`, fresh post-`.1a`/`.1b` wire IR + persisted corpus)

Executed before the `.1b.iv` code, per the measurement-first rule. Because the canonical
`generated/` corpus is still the PRE-`.1a` baseline (the WRITE-PATH GOTCHA — every prior
agent-surface rebuild went to a temp evidence-root), the 4 wire docs (AXI/APB/AHB/SWD) were
rebuilt `evidence → semantic → intent` into a temp evidence-root with the CURRENT
post-`.1a`/`.1b` release binary (canonical `source_ir` symlinked in; `--prior-memory` pointed at
the real store), so the measurement sees the agent surface AS IT IS TODAY. `python3` over the
fresh `intent_ir.json` categorised every 0/0 actor by IntentIR responsibility provenance.

### 8.1 The defensible discriminator (research §7.3's recommended "drop only PURE-INFERRED")

An actor is a PURE-INFERRED phantom iff its `responsibilities` is **exactly the single term-scan
marker** ``"semantic role inferred around `X` evidence"`` (the string `build_actors`,
semantic.rs:3190, emits for a generic role term it saw mentioned). This is structurally distinct
from:

- the relation-evidence summary a CONNECTED actor carries —
  ``"semantic role inferred from actor-signal relation evidence around `X`"`` (has `from … relation
  evidence`, ends `around `X``, not `evidence`) — so a connected actor never matches; and
- a grounded 0/0 actor, which `build_intent_actors` (intent.rs) gives an extra responsibility — a
  `participate in …` phase (SECTION+INFERRED) or a normalised contract sentence (PROSE-GROUNDED) —
  so its responsibility set has length > 1 and never matches.

Keyed on the marker SHAPE, never a name list (ADR 0006).

### 8.2 Measured result — wire docs (fresh) and corpus (persisted proxy)

| doc | actors | 0/0 actors | PURE-INFERRED drop | kept-0/0 (grounded) |
|---|---|---|---|---|
| AXI `ihi0022_l` | 21 | 12 | **0** | all 12 (incl. `transmitter`, `producer`, `consumer`) |
| APB `ihi0024_e` | 7 | 3 | **1** — `controller` | `source`, `state machine` |
| AHB `ihi0033_c` | 17 | 6 | **1** — `agent` | `channel`, `controller`, `device`, `receiver`, `source` |
| SWD `ihi0074_a` | 22 | 8 | **0** | all 8 (incl. `host`, `sink`, `state machine`) |

Corpus-wide (precise rule over the persisted 36-doc `intent_ir`): **21 PURE-INFERRED across 16
docs**, exactly the research §7.3 count — e.g. CHI `consumer`/`producer`, DTI `client`/`host`,
GIC-600 `receiver`, VT-d `initiator`, CCIX `state machine`, CXS `controller`, I2S `consumer`,
Wishbone `responder`. **Invariant proven (both fresh wire IR and the full corpus): ZERO connected
actors (≥1 port or ≥1 relation) are ever caught** — the marker + length-1 gate is exact.

### 8.3 Decision = GO (a clean precision win, not the marginal `.2b` situation)

Unlike `.2b` (noise-dominated, would mislead), `.1b.iv` is unambiguous: the 21 phantoms are pure
generic-vocabulary noise (the word appeared, nothing else), exactly bar #1's "every actor is a
real protocol agent, zero noise". The owner's completeness north star is honoured by KEEPING every
grounded 0/0 agent (AXI `transmitter`, SWD `host`, GIC `arbiter`) — pruning only the unambiguous
phantoms, never the broader PROSE-GROUNDED set (which §7.3 showed has no clean
genuinely-declared discriminator). Implemented at `build_intent_actors` (the single place
responsibilities are assembled); `.isf` byte-identical (the emitter lowers signals/behaviors, never
the raw `actors[]`); WIRE-BASED-100 gold fields byte-identical → held 1.000 on the fresh eval. See
KM card `[[agent-pure-inferred-phantom-drop]]` and `docs/tasks/KG-ISF-COMPLETENESS.md` (`.1b.iv`).

## 9. `.1b.ii` decision measurement (`2026-06-17`, persisted corpus census + fresh wire IR)

The `.1b` split (§7.1) deferred the `"X interface"→"X"` strip because a safe rule needs an "only when the
leading token is already a real connected agent in this doc" sub-gate — actor-set context the pure-string
relation-subject seam (`normalize_relation_actor_name`) does not have. `.1b.ii` resolves that.

### 9.1 Census — prevalence and the safe/risk split (read-only over the persisted 36-doc corpus)

`python3` over `intent_ir.json`: an actor whose name ends with `" interface"`, classified by whether the
stripped lead `X` is itself a CONNECTED actor (≥1 port or relation) in the same document.

| doc | `"X interface"` actor | lead `X` | class |
|---|---|---|---|
| CoreSight `100806_0701` | `AXI interface` | `AXI` (connected) | **SAFE-MERGE** |
| AXI+ACE `ihi0022_h_c` | `Subordinate interface` | `Subordinate` (connected) | **SAFE-MERGE** |
| AXI-Stream `ihi0051_b` | `Transmitter interface` | `Transmitter` (connected) | **SAFE-MERGE** |
| GIC `100336_0106` | `CPU interface` / `Q-Channel interface` / `AXI4-Stream interface` | not connected | **CONFLATION-RISK** |
| CoreSight `100806_0100` / `100806_0200` | `AXI interface` | `AXI` (not connected) | **CONFLATION-RISK** |

Only **6 docs** carry an `"* interface"` actor at all. The decisive datum: `"AXI interface"` is SAFE in
CoreSight `100806_0701` (where `AXI` is a connected agent) but a RISK in `100806_0100`/`0200` (where it is
not) — the **same token, opposite status by document**, so the gate MUST be per-doc evidence-keyed, never a
name list (ADR 0006) — exactly the genericity guardrail `transmitter` proved in §4. **None of the 4
WIRE-BASED-100 gold docs (APB/AHB/AXI/SWD) carry an `"* interface"` actor**, so the gold relation surface is
structurally untouched by any version of this rule.

### 9.2 The rule + seam, and why it is now buildable

Strip `"X interface"`→`X` **iff `X` is, in this document, an independent connected relation subject** (a
relation subject that is not itself an `"* interface"` form). Implemented as a post-pass
`consolidate_interface_actor_relations` over the assembled relation list — the same
`actor_signal_relation_surface` slot the `.1b.iii` coordinated split uses (after the split, before dedup),
where the full relation list, and therefore the connected-agent set, IS available (the deferral's missing
context). The rewritten relation merges with `X`'s existing relations at dedup; an unstripped subject passes
through byte-identical.

### 9.3 Result = GO, `.1b` umbrella complete

Live on a fresh post-`.1a`/`.1b` rebuild: AXI-Stream `Transmitter interface`→`Transmitter` (23 rels),
AXI+ACE `Subordinate interface`→`Subordinate` (49 rels); the GIC `CPU interface` named block and the
unconnected CoreSight `AXI interface` stay intact (the latter two not live-rebuildable — reclaimed/missing
source — so the conflation guard is locked by the `CPU interface` unit test). The 4 wire docs carry no
`"* interface"` actor → relation surface byte-identical → WIRE-BASED-100 held 1.000. With `.1b.ii` the
`.1b` umbrella (`.1b.i`/`.1b.ii`/`.1b.iii`/`.1b.iv`) is complete, and with `.1a` the agent-surface fidelity
goal (`KG-ISF-COMPLETENESS.1`) is fully built. See KM card `[[agent-interface-block-consolidation]]`.
