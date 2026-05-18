# FSMGEN Feedback From SPECFORGE

## Scope update (2026-05-18) — SPECFORGE emits only `.isf`

SPECFORGE's single adapter target is now `.isf`. SPECFORGE no longer emits
`.fsm` itself and never emitted HDL; FSMGEN consumes `.isf` and owns
scheduling, `.fsm`, and HDL downstream. The relationship is therefore
strictly `SPECFORGE IntentIR → .isf → FSMGEN`.

The `.fsm`-language-feature suggestions below are retained as historical
context and may still be useful to FSMGEN, but the **active, load-bearing
asks for SPECFORGE are the ISF-facing ones**: strict-mode `.isf` acceptance,
the capability manifest, stable diagnostic codes, JSON check, normalized
semantic JSON, and reset/clock/contract metadata as they apply to `.isf`.
Wherever this document says "`.fsm` adapter", read it as historical; the
current adapter is `.isf`.

## Purpose

This file is SPECFORGE's tracked feedback for FSMGEN.
It exists so FSMGEN can read one stable document and decide which ideas, if any, belong in FSMGEN itself.

SPECFORGE uses FSMGEN as the reference implementation and documentation surface for the downstream `.fsm` adapter.
SPECFORGE does not expect FSMGEN to solve PDF extraction, `IntentIR` recovery, or chip-spec semantic arbitration.
The goal is narrower and cooperative: help `.fsm` become a natural, precise lowering format for SPECFORGE's canonical `IntentIR`, while staying aligned with FSMGEN's own active direction.

This feedback is therefore not only about validation tooling.
It is also about language features and orientation that would let FSMGEN represent more of the typed hardware intent that SPECFORGE recovers.

## What SPECFORGE Is

SPECFORGE is a Rust toolchain for recovering typed implementation intent from chip-design specifications, especially PDFs.

Its core pipeline is:

```text
SourceIR -> EvidenceIR -> SemanticIR -> IntentIR -> adapters
```

The central product is `IntentIR`.
That artifact is meant to be backend-independent, provenance-aware, and honest about uncertainty.
It captures the strongest design intent SPECFORGE can justify from source evidence:

- actors and responsibilities
- interfaces and signal inventory
- actor-relative port/connectivity facts
- clock/reset/system contracts
- reset polarity and timing semantics
- control/state behavior
- typed temporal and stability rules
- assumptions, residual decisions, and conflicts

SPECFORGE is not trying to make `.fsm` the only product boundary.
Instead, `.fsm` is one downstream adapter target, alongside future SystemVerilog, Verilog, and VHDL targets.

The reason SPECFORGE cares deeply about FSMGEN is that `.fsm` can become the most natural high-level lowering format for recovered control intent.
If FSMGEN evolves `.fsm` in ways that align with the typed facts above, SPECFORGE can emit `.fsm` that preserves more real source intent instead of flattening it into comments, lossy HDL, or blocked adapter residuals.

So this feedback is written from the perspective of a tool that wants to lower honest `IntentIR` into a strong `.fsm` language, not from the perspective of a tool asking FSMGEN to contort itself around arbitrary output text.

Last SPECFORGE submodule sync reviewed:

- FSMGEN previous baseline: `955f2bb`
- FSMGEN refreshed baseline: `32aa318`
- notable reviewed surfaces: FSMGEN live mdBook at `subs/fsmgen/docs/book/`, first bounded `--capability-manifest`, `--check --json` / `--check-json`, stable `FSMGEN_*` diagnostic-code registry, `--emit-semantic-json` / `--semantic-json`, support-accounting/report contracts, and optional generated-SystemVerilog validation through `--verify-hdl` / `--validate-hdl`

## FSMGEN Response Received

FSMGEN responded in its own tracked document:

- FSMGEN submodule path: `subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md`
- initial response commit observed by SPECFORGE: `7475f07` (`Docs: track SPECFORGE feedback response`)
- latest response baseline reviewed by SPECFORGE: `32aa318` (`Refresh README bootstrap validation notes`)

SPECFORGE's planning interpretation is:

- FSMGEN accepts the shared high-level direction: `.fsm` should remain precise, strict mode is the canonical future-facing surface, compatibility syntax must stay labeled as compatibility residue, and machine-readable contracts should complement the mdBook.
- FSMGEN accepts the near-term integration sequence: capability manifest, stable diagnostic codes, check-only JSON diagnostics, normalized semantic JSON export, and first-class reset/clock contract metadata.
- FSMGEN accepts actor-relative ports, interface/channel grouping, semantic signal roles, temporal/stability contracts, assumptions/residual/provenance metadata, contract-aware composition, and a possible canonical direct-module root as directionally valuable longer-term language features.
- FSMGEN explicitly does not want unchecked annotations. Future language additions should be parsed, validated, represented in normalized semantics, documented in the mdBook, support-accounted by fixtures, and either lowered honestly to HDL or preserved honestly as checked metadata.
- SPECFORGE should target strict-mode canonical `.fsm`, treat compatibility syntax as adapter-blocked unless FSMGEN explicitly marks a compatibility lane safe for generated output, and consult FSMGEN's mdBook plus the bounded machine-readable capability/check/semantic/support-accounting surfaces that are now present in the pinned submodule.
- SPECFORGE can keep FSMGEN as a pinned downstream dependency/reference, but FSMGEN does not need a reciprocal SPECFORGE dependency unless a concrete cross-project conformance workflow later justifies it.

## Core Adapter Stance

The `.fsm` adapter in SPECFORGE should emit target text only when canonical facts are explicit enough to map into real FSMGEN-supported syntax.

When the canonical facts are incomplete, contradictory, or too target-specific to justify, SPECFORGE should produce a blocked adapter artifact with residual decisions instead of inventing `.fsm` text.

That means FSMGEN can help SPECFORGE most by making target-language truth machine-checkable:

- what syntax is canonical
- what syntax is compatibility-only residue
- what root kinds are supported
- what expression/control/reset/composition forms are accepted
- what diagnostics mean
- what normalized target semantics FSMGEN recovered from a file

It can also help by making `.fsm` expressive enough to carry common `IntentIR` facts directly, instead of forcing SPECFORGE to choose between lossy lowering and blocked output.

## What Is Already Helpful In Current FSMGEN

- The live mdBook gives SPECFORGE a progressive human-facing map for `.fsm` syntax and support boundaries.
- Strict-mode and support-accounting work help distinguish canonical language from tolerated legacy or compatibility forms.
- Typed failure diagnostics are the right direction for adapter validation and automated residual mapping.
- The expanding aggregate, package, type, parameter, and structural actual support points toward richer future `.fsm` lowering once SPECFORGE's own semantic facts are strong enough.
- Composition/toplink typing gives useful reference shapes for explicit top-root lowering.
- The emerging forward IR split in FSMGEN is valuable as a reference for keeping authored-source intent, lowered RTL, and structural connectivity separate.

## IntentIR-Aligned `.fsm` Feature Suggestions

These suggestions are about the `.fsm` language itself.
Exact syntax is FSMGEN's choice.
The important point is the semantic shape.

### 1. First-Class System Contract

SPECFORGE's `IntentIR` carries clock/reset meaning as hardware intent, not merely as ordinary signals.

A natural `.fsm` lowering target would be able to express:

- clock signal identity
- reset signal identity
- reset polarity
- synchronous versus asynchronous reset behavior
- asynchronous assertion and synchronous release intent
- reset target registers or state elements
- reset/source/distribution caveats where representable

This would make `.fsm` a much better target for real chip-spec intent because resets and clocks are special hardware infrastructure, not just inputs named `clk` and `rst_n`.

If FSMGEN intentionally keeps some of those facts out of generated HDL, it would still be useful to preserve them as checked metadata or normalized contract data.

### 2. Actor-Relative Port Semantics

SPECFORGE often knows that a signal is an input or output only relative to a specific actor:

- Manager drives `AWVALID`
- Subordinate drives `AWREADY`
- Requester drives `PSEL`
- Completer drives `PREADY`
- controller reads `DATA_IN` and drives `DATA_OUT`

Flat port direction is still needed for generated HDL, but `IntentIR` also benefits from preserving actor responsibility.

Potential `.fsm` support:

- optional actor/role annotations on ports
- explicit producer/consumer metadata
- module-local target-actor declaration for standalone roots
- normalized export of actor-relative port facts

This would reduce impedance between SPECFORGE's graph-first actor model and FSMGEN's emitted module boundary.

### 3. Interface Or Channel Grouping

Chip specs often describe related signals as protocol channels rather than isolated scalar ports.

IntentIR can capture that shape:

- ready/valid pairs
- address channels
- data channels
- response channels
- setup/access phases
- sideband groups
- payload plus qualifier/control relationships

A natural `.fsm` target would allow optional grouping metadata for signals that belong to the same logical interface or channel.

This does not require FSMGEN to hardcode AXI/APB/AHB.
The more general feature would be protocol-neutral grouping with semantic roles such as:

- valid-like
- ready-like
- select-like
- enable-like
- address
- data
- response
- sideband
- last/terminal marker

FSMGEN could choose whether these groups affect HDL, generate assertions, or remain normalized metadata.

### 4. Temporal And Stability Contracts

SPECFORGE increasingly recovers temporal rules from specs:

- signal must remain stable while a wait condition holds
- transfer completes on a ready/valid handshake
- a response follows an accepted request after a bounded cycle window
- certain fields remain stable through a transaction phase

Today SPECFORGE should not invent `.fsm` behavior if the target language cannot express the temporal fact.
Longer term, `.fsm` would be a more natural lowering target if it could carry optional checked temporal contracts.

Useful contract families:

- stable-while predicates
- handshake-complete predicates
- next-cycle or bounded-cycle obligations
- actor-grounded drive/stability obligations
- named phase conditions
- optional assertion-generation hooks

Even if FSMGEN initially treats these as metadata or optional generated assertions, preserving them would keep `.fsm` closer to the captured `IntentIR`.

### 5. Semantic Signal Roles

IntentIR may know that a signal is not merely `input wire`.
It may know that the signal is:

- a clock
- a reset
- a valid qualifier
- a ready qualifier
- a payload
- a select
- an enable
- an error/response
- a state/control signal

If `.fsm` can carry those semantic roles, SPECFORGE can lower richer intent without encoding meaning only in names or comments.

FSMGEN could use the roles for:

- stricter diagnostics
- better generated comments
- optional assertion generation
- support-accounted examples
- normalized AST/IR export

### 6. Assumptions, Residuals, And Provenance Metadata

SPECFORGE will often lower a partially known design.
When it does, it should preserve why the target is safe enough or why something was intentionally omitted.

Useful `.fsm` metadata would include:

- assumptions
- residual decisions
- source provenance IDs
- confidence/caveat markers
- unsupported-source-intent notes

This should not pollute normal hand-authored `.fsm`.
It could live behind a generated-metadata section, structured comments, or a strict machine-readable annotation surface.

The value is round-trip honesty: generated `.fsm` remains inspectable, and downstream tools can see what SPECFORGE knew versus what it could actually express.

### 7. Explicit Direct-Module Root Shape

SPECFORGE currently keeps compatibility-level `?mod:name` / `?module:name` outside its canonical root-kind model until it has a backend-neutral direct-module distinction.

If FSMGEN wants direct-module roots to become a canonical language feature rather than compatibility residue, SPECFORGE would benefit from a documented strict-mode root shape for them.

The useful contract would define:

- how a direct module differs from `?dt`, `?fsm`, and `?top`
- what control/body forms it may contain
- how ports/system/reset/init sections behave
- whether it can carry actor/channel/temporal metadata
- how it normalizes in exported AST/IR

That would give SPECFORGE a safe future lowering lane for `IntentIR` cases that are module-like but not naturally a pure decision tree or explicit state graph.

### 8. Contract-Aware Composition

FSMGEN's current composition/toplink direction is already useful.
SPECFORGE would benefit if composition could also preserve contract facts across child boundaries:

- child actor roles
- top/child channel grouping
- reset/clock distribution
- explicit width/type compatibility
- link provenance
- interface-level direction and role consistency

This would let SPECFORGE lower more of `IntentIR` topologies without flattening away why the links are semantically correct.

## Requested Support And Tooling Features

These suggestions are about making the language contract executable for tool-to-tool integration.
As of the `32aa318` submodule sync, FSMGEN has first bounded implementations for the capability manifest, JSON check diagnostics, stable diagnostic-code registry, normalized semantic JSON export, generated-SystemVerilog validation, and several public support/report contract owners. SPECFORGE should treat those as regression-backed first slices to consume carefully, not as permission to infer target-language semantics outside FSMGEN's published contract.

### 1. Machine-Readable Capability Manifest

Please consider publishing a versioned capability manifest, preferably generated from the same support-accounting source that drives tests and docs.

Useful fields would include:

- FSMGEN version or commit hash
- supported root kinds
- strict-mode canonical syntax families
- compatibility-only syntax families
- supported assignment forms
- supported reset/system forms
- supported expression families
- supported aggregate/type/package features
- supported composition/toplink forms
- unsupported or intentionally blocked forms
- links to mdBook chapters or test fixtures

SPECFORGE would use this manifest to gate adapter renderability before emitting `.fsm`.

### 2. JSON Check And Diagnostic Mode

Please consider a stable check-only command such as:

```bash
fsmgen --strict --check --json path/to/file.fsm
```

The ideal output would avoid HDL generation and return structured diagnostics:

- stable diagnostic code
- severity
- source span
- root/module/context
- concise reason
- strict-vs-compatibility classification
- suggested migration when available

SPECFORGE would use this to validate emitted `.fsm` artifacts and map failures back into adapter residual packets.

### 3. Normalized AST Or IR Export

Please consider a parse/normalize/export command such as:

```bash
fsmgen --strict --emit-normalized-json path/to/file.fsm
```

The goal is not to expose every internal detail.
The useful surface would be a stable target-language semantic projection:

- root kind and name
- system/reset declarations
- ports/signals/types/packages
- state graph or decision-tree control
- assignments and guards
- composition children and links
- normalized expression forms
- compatibility residue, if any

SPECFORGE could then compare generated `.fsm` against FSMGEN's recovered normalized semantics instead of relying only on text snapshots or HDL shape.

### 4. Stable Diagnostic Codes

Typed diagnostics become much more useful if codes are stable across wording changes.

For example, a machine-readable code like `FSMGEN_STRICT_INFIX_ASSIGNMENT` is easier for SPECFORGE to consume than prose that may improve over time.

The exact names are FSMGEN's choice.
The key request is stable identity plus structured fields.

### 5. Reset And Clock Metadata

SPECFORGE cares a lot about clock/reset truth:

- clock source/distribution
- reset source/distribution
- reset polarity
- asynchronous assertion
- synchronous release
- reset target registers/flops
- absence of unsafe glue logic on reset/clock trees

FSMGEN does not need to become a CDC/RDC tool, but richer reset/clock metadata or diagnostics would make `.fsm` adapter validation much safer.

Useful surfaces could include:

- explicit reset polarity metadata where the language can represent it
- diagnostics when reset syntax implies less than the source intent requires
- normalized system-contract JSON for clock/reset declarations
- clear mdBook guidance on what `.fsm` can and cannot express about reset polarity and release semantics

This overlaps with the language-feature request above.
The language request is about making reset/clock facts expressible.
The tooling request is about making those facts visible and checkable.

### 6. Adapter-Facing Example Corpus

Please consider maintaining a small canonical corpus specifically for tools that emit `.fsm`.

Each case could include:

- input `.fsm`
- expected normalized AST/IR JSON
- expected strict/check result
- expected HDL-shape snippets where relevant
- short mdBook cross-reference

Useful families:

- standalone combinational decision tree
- standalone sequential decision tree
- explicit FSM root
- explicit top composition
- reset/system declarations
- aggregate/type/package use
- selector/test-node control
- compound update
- blocked strict-mode examples

This would give SPECFORGE a high-quality target conformance suite.

### 7. Keep Strict Mode First

The most useful orientation for SPECFORGE is not broader acceptance at all costs.
It is precise acceptance.

Please keep pushing canonical behavior through strict-mode support accounting, positive fixtures, negative fixtures, and mdBook documentation.

Compatibility syntax can remain useful, but it should stay labeled as compatibility residue so adapter authors do not accidentally target it as the future language.

### 8. Preserve The Public Book As A Live Contract

The FSMGEN mdBook is especially valuable if it stays in sync with shipped behavior.

SPECFORGE will treat the book as a public human-facing language contract, while treating machine-readable manifests/check output as executable contracts.

The best long-term shape is both:

- book chapters for readers
- machine-readable support metadata for tools

## Priority From SPECFORGE's Side

If FSMGEN wants an order of attack, the most leverage for SPECFORGE would be:

- first-class reset/clock contract metadata
- continued stabilization and widening of the strict-mode capability manifest
- continued stabilization and widening of JSON check diagnostics with stable codes
- continued stabilization and widening of normalized semantic JSON export
- actor-relative port and semantic-role annotations
- temporal/stability contract metadata
- adapter-facing examples

The first four already have useful first slices in the pinned FSMGEN baseline, and widening them from regression-backed support-accounting truth would make the current adapter safer.
The later ones make `.fsm` a more natural target for future `IntentIR` richness.

## How SPECFORGE Would Use These Features

SPECFORGE would not use FSMGEN features to mutate canonical `IntentIR`.

Instead, it would use them downstream:

- capability manifest decides whether an `IntentIR` shape is renderable as `.fsm`
- IntentIR-aligned `.fsm` features decide whether richer canonical facts can be preserved instead of becoming adapter residuals
- JSON check validates emitted `.fsm`
- normalized AST/IR export verifies target semantics after parsing
- diagnostic codes map FSMGEN failures into SPECFORGE adapter residual decisions
- actor/channel/reset/temporal metadata keeps generated `.fsm` closer to source intent
- example corpus becomes adapter conformance coverage
- mdBook remains the human reference for why the adapter accepts or blocks a case

## Non-Goals

This feedback is not asking FSMGEN to:

- parse chip PDF specifications
- infer hardware intent from ambiguous prose
- become SPECFORGE's canonical IR
- accept unsafe compatibility syntax just to make adapter output easier
- hide target-language limitations behind permissive parsing

SPECFORGE's side of the bargain is to keep its adapter honest.
FSMGEN's most useful side of the bargain is to keep `.fsm` behavior precise, documented, and machine-checkable.

## Tracked finding (2026-05-18) — `ISF_DOWNSTREAM_INTEGRATION_SPEC.md` §11.8 doc-vs-strict mismatches

While implementing `ISF-TEMPORAL-LOWERING.2.1`, SPECFORGE verified §11.8
constructs against the pinned `subs/fsmgen/bin/fsmgen --strict --check
--json`. Two precise mismatches between the handoff doc and the shipped
strict checker:

1. **`(contract name (eventually signal within N))`** — §11.8 prints the
   flat form `eventually signal within N`, but `--strict --check`
   rejects it: *"contract '<n>' supports only '(eventually signal
   (within cycles))'"*. The accepted shape is the **nested**
   `(eventually <signal> (within <N>))`. SPECFORGE now emits the nested
   form; the doc prose should be corrected to match the strict grammar.
2. **`(stage phase (ready r) (valid v))`** — §11.8 presents this as the
   shipped `ready_valid_barrier`, but `--strict --check` rejects it:
   *"stage '<p>' has unsupported subclause 'ready'"*. Because the
   handoff doc explicitly lists it as supported yet strict rejects it
   (the documented escalation bar), SPECFORGE does **not** emit
   `(stage …)`; ISF temporal `HandshakeComplete` obligations are
   preserved as explicit SPECFORGE residual decisions instead, and the
   `(stage …)` source shape is reported here for FSMGEN to either fix in
   the checker or correct in the spec.

SPECFORGE has not patched the submodule (per the standing rule); this is
a forward bug report. The `(contract … (eventually s (within N)))` form
is confirmed strict-valid and is what SPECFORGE emits.
