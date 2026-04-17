# FSMGEN Feedback From SPECFORGE

## Purpose

This file is SPECFORGE's tracked feedback for FSMGEN.
It exists so FSMGEN can read one stable document and decide which ideas, if any, belong in FSMGEN itself.

SPECFORGE uses FSMGEN as the reference implementation and documentation surface for the downstream `.fsm` adapter.
SPECFORGE does not expect FSMGEN to solve PDF extraction, `IntentIR` recovery, or chip-spec semantic arbitration.
The goal is narrower: make the boundary between SPECFORGE's canonical `IntentIR` and FSMGEN's `.fsm` language as precise, validated, and machine-checkable as possible.

Last SPECFORGE submodule sync reviewed:

- FSMGEN old baseline: `57f00e5`
- FSMGEN refreshed baseline: `955f2bb`
- notable new surface: FSMGEN live mdBook at `subs/fsmgen/docs/book/`

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

## What Is Already Helpful In Current FSMGEN

- The live mdBook gives SPECFORGE a progressive human-facing map for `.fsm` syntax and support boundaries.
- Strict-mode and support-accounting work help distinguish canonical language from tolerated legacy or compatibility forms.
- Typed failure diagnostics are the right direction for adapter validation and automated residual mapping.
- The expanding aggregate, package, type, parameter, and structural actual support points toward richer future `.fsm` lowering once SPECFORGE's own semantic facts are strong enough.
- Composition/toplink typing gives useful reference shapes for explicit top-root lowering.
- The emerging forward IR split in FSMGEN is valuable as a reference for keeping authored-source intent, lowered RTL, and structural connectivity separate.

## Requested FSMGEN Features

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

## How SPECFORGE Would Use These Features

SPECFORGE would not use FSMGEN features to mutate canonical `IntentIR`.

Instead, it would use them downstream:

- capability manifest decides whether an `IntentIR` shape is renderable as `.fsm`
- JSON check validates emitted `.fsm`
- normalized AST/IR export verifies target semantics after parsing
- diagnostic codes map FSMGEN failures into SPECFORGE adapter residual decisions
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
