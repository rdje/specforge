# FSMGen Feature Request: optional interface ports — absent by default, present by a parameter, tied off when absent

Title: sf-isf-optional-signal — an `.isf` interface port cannot be absent: a width parameter defaulting to 0 is refused, and there is no way to make a port depend on a parameter or give an absent input its tie-off value
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/absent-by-default.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/absent-by-default.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 354844fe1019d2db1e413dd54f8c4a5ae9eb398b (upstream HEAD, 2026-09-25); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note" below
SPECFORGE version/commit: specforge 4cd3d3ac
Expected behavior: an actor width parameter may default to 0, meaning the port is absent from the generated module; --strict --check --json reports success:true for sources/fsmgen-input/absent-by-default.isf. A port may also be declared present only when a parameter enables it, and an absent input may declare the value it reads.
Observed behavior: --strict --check --json reports success:false with "actor 'mecid_demo' interface port 'AWMECID' width parameter 'MECID_WIDTH' must resolve to a positive integer"; a literal (width 0) is refused too; a (present PARAM) port clause passes --strict and is silently ignored, so the port is still generated.
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); JSON diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `EXTRACTION-REPRESENTATION-GAPS.4` (bundle); SPECFORGE's half is `KG-ISF-COMPLETENESS.2a.vii.c.vi`
(reading the presence properties) and the adapter change once shipped.
Local status: see `request.json` (`forwarded` by the owner on `2026-09-25`; `acknowledged` by FSMGen on `2026-09-26`, owner `SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.5`).

## 1. What SPECFORGE needs

Interface specifications describe **configurable** interfaces in which some signals exist only in some
configurations. The AMBA AXI specification (IHI0022 L) says it three ways, and SPECFORGE extracts all three:

- **A width parameter whose default is 0, meaning absent.** `MECID_WIDTH` takes the values `0, 16` with default
  `0`: the `AWMECID`/`ARMECID` signals do not exist unless an integrator enables them.
- **A yes/no property that makes signals exist.** `PROT_Present` (`True, False`) decides whether the `AxPROT`
  signals are part of the interface.
- **The value an absent input takes.** *"A Manager that only issues requests of Length 1 can omit the AxLEN
  outputs from its interface. An attached Subordinate must …"* use the default; *"an attached Subordinate must
  have its AWSNOOP input tied LOW"* when the Manager omits `AWSNOOP`.

`.isf` can say none of these. A generic block that SPECFORGE lowers must therefore either declare every optional
signal present (with a width the specification says is not the default) or drop it (and lose it when an
integrator enables it). Both misstate the specification.

## 2. Measured need (SPECFORGE corpus, 2026-09-25)

Over the three AMBA interface specifications SPECFORGE rebuilds today (AXI IHI0022 L, AHB IHI0033 C, APB 5
IHI0024 E):

- AXI: **14 width parameters default to 0, sizing 21 signals** (from the adapter's own residuals; SPECFORGE
  currently declares each at its smallest positive legal value and says so).
- **18 statements** across the three state a tie-off or a default for an omitted signal (*"… inputs tied LOW"*,
  *"… can omit the AxLEN outputs"*, *"BUSY inputs are tied to the default value"*).
- AXI's parameter tables carry yes/no presence properties (`PROT_Present`, `INSTPRIV_Present`).

Re-derive (in a SPECFORGE checkout with its generated corpus): `python3 scripts/measure_intent_to_isf_reach.py
residuals` (the `isf_signal_width_parameter_*` rows name each parameter and the signals it sizes) and the census
in `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.4`).

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13a-actor-interface.md`, *Interface*: a width may be a positive integer, an actor-local scalar
  parameter default, or a declared actor constant that resolves to a positive integer; *"zero-valued or
  non-scalar actor parameters, zero-valued actor constants, and arbitrary expressions fail closed."*
- `docs/book/src/13k-isf-feature-support-matrix.md`, row *Interface ports*: `(input NAME [(width …)|(type T)]
  [(domain D)])`; ports become module ports *"with resolved positive integer widths"*.

FSMGen does what its contract says. The contract has no optional port.

**Precedent inside FSMGen.** IAL2 already has an explicit zero-width absence form for AXI ID families
(`docs/book/src/14d-ial2-foundations.md`: *"The zero-width absence form is explicit: (id-families (write (width
0)) (read (width 0)))"*), and the CDC surface generates domain modules *"without absent reset ports"* (13a). This
request asks for the same idea on ordinary `.isf` interface ports.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`. Every input differs from `baseline-good.isf` in one place.

| Input | Differs from the baseline by | `--strict --check` | Result |
| --- | --- | --- | --- |
| `baseline-good.isf` | — | exit 0 | accepted |
| `absent-by-default.isf` (primary) | `(MECID_WIDTH 0)` | exit 1 | *width parameter 'MECID_WIDTH' must resolve to a positive integer* |
| `literal-zero-width.isf` | `(output AWMECID (width 0))` | exit 1 | *width must be a positive integer, actor constant, …* |
| `tie-off.isf` | an input whose width parameter is 0, with `(absent-value 0)` | exit 1 | *width parameter 'AWSNOOP_WIDTH' must resolve to a positive integer* |
| `present-when.isf` | `(output AWPROT (width 3) (present PROT_Present))`, `PROT_Present` 0 | exit 0 | accepted, and the clause is **ignored**: the generated SystemVerilog still declares and drives `AWPROT` (11 lines, `observed/command-logs/present-when-awprot-line-count.txt`), exactly as without the clause |

The last row is also a separate observation, kept apart from this request: strict mode accepts an unknown port
clause without a diagnostic. SPECFORGE records it as a question in its feedback ledger, because FSMGen's contract
does not say whether an unknown clause is an error.

## 5. Requested capability

- **R1 (required) — a zero width means absent.** A port whose width resolves to `0` (a literal, a parameter
  default, or an override) is absent from the generated module. Reading an absent input yields its tie-off
  value (R3); driving an absent output is a no-op. Failure cases: a zero width on a port with no tie-off that is
  read in a condition or expression fails closed with a diagnostic naming the port; a negative or non-integer
  width still fails closed.
- **R2 (required) — presence by parameter.** `(present P)` on a port makes it present exactly when the actor
  scalar parameter `P` is non-zero. Failure cases: an undeclared `P`, a non-scalar `P`, or a `(present …)` that
  names something other than an actor parameter fails closed.
- **R3 (required) — the value an absent input reads.** `(absent-value V)` on an input gives the value reads of
  that input take when it is absent (R1 or R2). Failure cases: `V` wider than the declared width, a non-literal
  `V`, or `(absent-value …)` on an output fails closed.
- **R4 (optional) — keep presence across instantiation.** A library use-site or activation override of the
  width or presence parameter re-decides presence for that instance (ties to SPECFORGE's generic-block request
  `sf-isf-generic-port-widths`).

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `absent-by-default.isf` passes `--strict --check`, and its generated module has no `AWMECID` port.
2. The same file with `(MECID_WIDTH 16)` generates a module whose port list declares a 16-bit `AWMECID`
   output. *Corrected `2026-09-26`:* this criterion first read "byte-identical to `baseline-good.isf`". FSMGen
   found that the `baseline-good.isf` module has no `AWMECID` port either: the output is never driven, and an
   undriven declared output is dropped from the HDL (FSMGen `SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.9`). SPECFORGE
   confirmed this on `54ac1c22` (the semantic export names no `AWMECID`) and on `5be6a3c08` (no `AWMECID` in the
   generated module). Comparing two modules that both omit the port proves nothing about presence, so presence
   is checked by inventorying the realized ports. `baseline-good.isf` is left as forwarded.
3. `present-when.isf` with `PROT_Present` 0 generates no `AWPROT` port and no drive of it; with `PROT_Present` 1,
   the same module as without the clause.
4. `tie-off.isf` passes; a guard that reads `AWSNOOP` behaves as if it read `0`.
5. Each R1-R3 failure case above produces a diagnostic and a non-zero exit under `--strict`.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **Declare every optional port present** at its smallest non-zero legal width (what SPECFORGE does today):
  elaborates, but the default configuration of the specification has no such port, and a Subordinate that
  should read a tie-off reads a live input.
- **Drop optional ports**: loses them for every integrator who enables them.
- **Emit one actor per configuration**: combinatorial in the number of presence properties, and it loses the
  generic block the specification describes.

## 7a. What SPECFORGE does until this ships

SPECFORGE declares each zero-default width parameter at its smallest positive legal value and names that choice
in the adapter residual `isf_signal_width_parameter_chosen_defaults`; presence properties and tie-offs remain in
its IR and are not emitted.

## 8. Compatibility

Additive: every currently accepted input keeps its meaning. Only inputs FSMGen refuses today (zero widths) or
silently ignores today (an unknown port clause) change.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `354844fe1019d2db1e413dd54f8c4a5ae9eb398b` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same five inputs: identical exit codes; the
generation probe was run on upstream only, because SPECFORGE writes no files inside its pinned checkout. FSMGen's
check itself uses and cleans its own ignored `.artifacts/tmp/` scratch there, and the checkout stays clean). FSMGen's
feature matrix (`13k-isf-feature-support-matrix.md`) is byte-identical at the two revisions.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* is corrected by hand: the helper recorded SPECFORGE's commit, the defect
  reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop and generation probe at its end.
