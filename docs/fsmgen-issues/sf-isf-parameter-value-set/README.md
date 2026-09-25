# FSMGen Feature Request: declared legal values and a recommended range for actor parameters

Title: sf-isf-parameter-value-set — an actor parameter can declare only a default, not the values it may take (`8, 16, … 1024`, `1..64`) or the range the specification recommends
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/value-set.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/value-set.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 354844fe1019d2db1e413dd54f8c4a5ae9eb398b (upstream HEAD, 2026-09-25); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note" below
SPECFORGE version/commit: specforge c6cab043
Expected behavior: an actor scalar parameter may declare the values it may take, as a list (values 8 16 ... 1024) or an inclusive range (range 8 1024), and a recommended range (recommended (range 32 256)); --strict --check --json reports success:true for sources/fsmgen-input/value-set.isf, and a default or an override outside the declared values fails closed.
Observed behavior: --strict --check --json reports success:false with "actor 'bus_demo' params entries require '(NAME value)'" for every spelling of a declared value set or range.
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); JSON diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `EXTRACTION-REPRESENTATION-GAPS.5` (bundle); SPECFORGE already carries the values
(`KG-ISF-COMPLETENESS.2a.vii.c.ii`, `.c.iii`) and will emit them once this ships.
Local status: see `request.json` (`forwarded` by the owner on `2026-09-25`).

## 1. What SPECFORGE needs

A specification that describes a generic block says which values each of its parameters may take, and often
which it recommends:

- AXI (IHI0022 L) parameter tables: `DATA_WIDTH` takes `8, 16, 32, 64, 128, 256, 512, 1024`; `ADDR_WIDTH`
  takes `1..64`; `ACT_R_WIDTH` takes `0, 16..32`.
- AHB (IHI0033 C): *"DATA_WIDTH can be 8, 16, 32, 64, 128, 256, 512, or 1024. However, any value smaller than 32
  or larger than 256 is not recommended."*; *"ADDR_WIDTH is recommended to be between 10 and 64."*

SPECFORGE reads all of these into typed values. An `.isf` actor parameter can carry only its default, so the
legal set stops at SPECFORGE's IR: nothing checks that an integrator's override is a width the specification
allows, and nothing tells them which the specification recommends.

## 2. Measured need (SPECFORGE corpus, 2026-09-25)

**36 parameters** carry a legal set or a recommended range across the three AMBA interface specifications
SPECFORGE rebuilds: AXI 28 (28 legal sets), AHB 7 (3 legal sets, 4 recommended ranges), APB 5 1 (legal set).
Re-derive in a SPECFORGE checkout: count the `param` entries of `generated/intent_ir/*/intent_ir.json`
`symbol_definitions` that carry `legal_values` or `recommended_values`.

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13k-isf-feature-support-matrix.md`, row *Actor report metadata and params*: actor parameter
  defaults may use literals, declared actor constants, earlier scalar parameter defaults, enum members,
  qualified package constants and compatible aggregate/list defaults; they are reported in `actor_params[]`.
  There is no declared value set.
- The observed diagnostic, *"params entries require '(NAME value)'"*, is FSMGen enforcing that shape.

**Precedent inside FSMGen.** Several surfaces already fail closed on an out-of-range value: `(set-field …)`
refuses a value that does not fit the field, `(for (i from A to B …))` refuses a non-upward range, and a storage
`(reset V)` refuses a value wider than the register (13k *Data manipulation*, *Transaction control flow*,
*Actor-owned scalar storage*). This request asks for the same discipline on a parameter's own values.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`. Each input differs from `baseline-good.isf` only in the
`DATA_WIDTH` entry.

| Input | `DATA_WIDTH` entry | `--strict --check` | Result |
| --- | --- | --- | --- |
| `baseline-good.isf` | `(DATA_WIDTH 32)` | exit 0 | accepted |
| `value-set.isf` (primary) | `(DATA_WIDTH 32 (values 8 16 32 64 128 256 512 1024))` | exit 1 | *params entries require '(NAME value)'* |
| `range.isf` | `(DATA_WIDTH 32 (range 8 1024))` | exit 1 | same |
| `recommended.isf` | `(DATA_WIDTH 32 (recommended (range 32 256)))` | exit 1 | same |
| `illegal-default.isf` | `(DATA_WIDTH 24 (values 8 16 … 1024))` | exit 1 | same today; **must stay refused** once supported, because 24 is not a declared value |

## 5. Requested capability

- **R1 (required) — a declared value set.** `(NAME DEFAULT (values V…))` and `(NAME DEFAULT (range LOW HIGH))`
  on an actor scalar parameter (several `values`/`range` items may combine: `0, 16..32`). The default must be in
  the set. Failure cases: a default outside the set, `LOW > HIGH`, a non-integer member, or a set on a non-scalar
  parameter fails closed.
- **R2 (required) — checked where the value changes.** A library use-site override, an activation override, or a
  generated-top override outside the set fails closed with a diagnostic naming the parameter, the value and the
  set.
- **R3 (required) — reported.** `actor_params[]` carries the declared set, so downstream reports and tools can
  show the legal configurations.
- **R4 (optional) — a recommended range.** `(recommended (range LOW HIGH))` (or `values`) is advisory: a default
  or override outside it produces a warning, never an error, and is reported.
- **R5 (optional) — in the HDL.** Where the generated module exposes the parameter, an elaboration-time check
  (for example a SystemVerilog assertion on the parameter) rejects an illegal instantiation.

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `value-set.isf`, `range.isf` and `recommended.isf` pass `--strict --check`, and their generated module equals
   the one for `baseline-good.isf`.
2. `illegal-default.isf` fails under `--strict` with a diagnostic naming `DATA_WIDTH`, `24` and the set.
3. An override of `DATA_WIDTH` to `24` at a library use-site fails closed; to `64`, it passes.
4. `actor_params[]` in the schedule/semantic JSON carries the declared set for `DATA_WIDTH`.
5. `recommended.isf` with default `16` passes with a warning naming the recommended range.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **Comments or a side file**: nothing checks them, and they drift from the actor.
- **An enum of widths**: a width is an integer used in arithmetic; an enum type cannot size a port.
- **Keep only the default** (what SPECFORGE does today): the specification's constraint on integrators is lost.

## 7a. What SPECFORGE does until this ships

SPECFORGE keeps the legal values and recommended ranges in its IR (`SymbolDefinitionRecord.legal_values`,
`recommended_values`) and uses them to choose each parameter's default (the smallest positive legal value inside
a recommended range, …), naming that choice in the adapter residual `isf_signal_width_parameter_chosen_defaults`.

## 8. Compatibility

Additive: `(NAME value)` keeps its meaning; only entries FSMGen refuses today gain one.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `354844fe1019d2db1e413dd54f8c4a5ae9eb398b` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same five inputs: identical exit codes). FSMGen's
feature matrix is byte-identical at the two revisions.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* is corrected by hand: the helper recorded SPECFORGE's commit, the defect
  reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop at its end.
