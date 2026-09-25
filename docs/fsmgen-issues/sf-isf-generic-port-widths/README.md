# FSMGen Feature Request: a parameter-sized port stays generic in the generated HDL

Title: sf-isf-generic-port-widths — the generated module declares the actor parameter (`parameter DATA_WIDTH = 32`) but sizes the port with its resolved default (`output reg [31:0] WDATA`), so the parameter controls nothing and an instantiation override silently has no effect
FSMGen command or public API entrypoint: ./bin/fsmgen --outdir DIR -l sv -o OUT.sv sources/fsmgen-input/parameter-width.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/parameter-width.isf
Failure class: wrong-output
FSMGen commit or capability-manifest producer commit: 354844fe1019d2db1e413dd54f8c4a5ae9eb398b (upstream HEAD, 2026-09-25); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note" below
SPECFORGE version/commit: specforge b11272ff
Expected behavior: the generated module keeps a parameter-sized port generic: WDATA is declared [DATA_WIDTH-1:0], its internal register and reset value follow DATA_WIDTH, so instantiating bus_demo #(.DATA_WIDTH(64)) yields a 64-bit port; a library use-site or activation override of DATA_WIDTH re-specializes the port the same way.
Observed behavior: --strict --check passes and the generated SystemVerilog declares 'parameter DATA_WIDTH = 32' but sizes the port 'output reg [31:0] WDATA' (register 'reg [31:0] WDATA_next', reset 32'h00000000); with the default changed to 64 the port becomes [63:0]. The width follows the default, never the parameter.
First failing command or API call: parameter-width-generate-sv (see observed/command-logs/parameter-width-declarations.txt)
Exit status or exception: 0 — the output is valid SystemVerilog; the gap is in what it declares
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
The emitted-but-unused module parameter may also be a hazard FSMGen classifies as a defect; SPECFORGE leaves
that call to FSMGen, since the published contract does not say what the emitted parameter means.
SPECFORGE owner: `EXTRACTION-REPRESENTATION-GAPS.6`.
Local status: see `request.json` (`signed_off`: ready for the owner to forward).

## 1. What SPECFORGE needs

Interface specifications describe **generic blocks**: AHB says `HRDATA` is `DATA_WIDTH` bits wide and lists the
values `DATA_WIDTH` may take; the engineer who instantiates the block chooses one. SPECFORGE lowers such a signal
to `(output HRDATA (width DATA_WIDTH))` with a `(params (DATA_WIDTH D))` declaration so the `.isf` stays generic.
The genericity has to survive into the HDL, or the generated block is one fixed configuration that only looks
configurable.

## 2. Measured need (SPECFORGE corpus, 2026-09-25)

**62 ports** in SPECFORGE's `.isf` for the three AMBA interface specifications it rebuilds are sized by a
parameter: AXI 46, AHB 9, APB 5 7. Re-derive in a SPECFORGE checkout: count the `(width NAME)` port declarations
in the `isf.source_text` of `generated/adapters/isf/*/adapter.json`.

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13k-isf-feature-support-matrix.md`, row *Interface ports*: *"Ports become scheduled `.fsm`
  `+size` entries and module ports with resolved positive integer widths."* The generated widths are exactly
  that.
- Row *Reusable ISF libraries*: use-site parameter overrides are accepted, but *"parameter-driven shape
  elaboration and nested library imports remain backlog"*; the FIFO fixture *"does not claim use-site
  parameter-driven FIFO interface shape, … generated-top respecialization"*.
- The book does not say what the emitted `parameter DATA_WIDTH = 32` means for an HDL user. Declared but unused,
  it invites an override that does nothing.

**Precedent and relation.** SPECFORGE's `sf-isf-derived-width-parameter` (forwarded and acknowledged) asks for a
*derived* parameter to stay symbolic in *"the generated module's parameter list … so overriding `DATA_WIDTH` at
instantiation re-derives `STRB_WIDTH`"*. That presumes a plain parameter-sized port already follows its
parameter; this bundle shows it does not. This request is that base case, not a duplicate.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`. Each input is generated to SystemVerilog, and the module
parameter and `WDATA` declarations are extracted to `observed/command-logs/<input>-declarations.txt`.

| Input | Port declared as | Module parameter | Generated `WDATA` | Register / reset |
| --- | --- | --- | --- | --- |
| `baseline-good.isf` | `(width 32)` | none | `output reg [31:0] WDATA` | `reg [31:0]`, `32'h00000000` |
| `parameter-width.isf` (primary) | `(width DATA_WIDTH)`, `DATA_WIDTH` 32 | `parameter DATA_WIDTH = 32` | `output reg [31:0] WDATA` | `reg [31:0]`, `32'h00000000` |
| `parameter-width-64.isf` | same, `DATA_WIDTH` 64 | `parameter DATA_WIDTH = 64` | `output reg [63:0] WDATA` | `reg [63:0]`, `64'h0…0` |

All three pass `--strict --check`. The primary's port, register and reset are identical to the fixed-width
baseline; only the default changes the width.

## 5. Requested capability

- **R1 (required) — ports follow the parameter.** A port whose width is an actor parameter is declared with that
  parameter in the generated module (`[DATA_WIDTH-1:0]`), and so is every internal signal whose width derives from
  it; reset and constant values are sized from it (`'0`, or a sized expression), not from the default.
  Failure cases: an instantiation value that is not a positive integer fails elaboration (an elaboration-time
  check, together with `sf-isf-parameter-value-set` R5 where a value set is declared).
- **R2 (required) — overrides re-specialize.** A library use-site, activation, or generated-top override of the
  parameter yields ports of the overridden width (the backlog item *parameter-driven shape elaboration*).
- **R3 (required if R1 is not taken) — no misleading parameter.** If ports stay resolved, the generated module
  should not declare a parameter that controls nothing, or should document it as report-only.
- **R4 (optional) — the scheduled review artifact states the dependency.** `+size` records the parameter a width
  came from, so a reviewer can see which widths are generic.

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. For `parameter-width.isf` the generated module declares `WDATA` as `[DATA_WIDTH-1:0]` (or an equivalent
   parameter-dependent range), and no `[31:0]` literal remains for it, its register, or its reset value.
2. Instantiating the generated module with `#(.DATA_WIDTH(64))` in a testbench elaborates a 64-bit `WDATA` whose
   behaviour matches `parameter-width-64.isf`'s module.
3. `baseline-good.isf`'s output is unchanged.
4. A use-site override of `DATA_WIDTH` in a library use generates a child with the overridden port width.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **Emit one actor per width**: a fixed configuration per file loses the generic block the specification
  describes, and the number of configurations multiplies with every parameter.
- **Post-process the generated SystemVerilog**: SPECFORGE does not own HDL generation, and a textual rewrite of
  `[31:0]` would be fragile and wrong wherever an unrelated signal happens to have the same width.
- **Accept the fixed width**: the integrator's configuration choice, which the specification leaves to them, is
  silently removed.

## 7a. What SPECFORGE does until this ships

SPECFORGE emits `(width P)` with a `(params (P D))` default taken from the specification (a stated default, or a
named choice), and its book says the generated HDL is fixed at that default.

## 8. Compatibility

For an unoverridden instantiation the generated behaviour is unchanged; only instantiations that override the
parameter (which today silently get the default width) behave differently.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `354844fe1019d2db1e413dd54f8c4a5ae9eb398b` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the three inputs: identical exit codes; generation
was run on upstream only, because SPECFORGE writes no files inside its pinned checkout). FSMGen's feature matrix is
byte-identical at the two revisions.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* is corrected by hand: the helper recorded SPECFORGE's commit, the defect
  reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added generation loop at its end.
