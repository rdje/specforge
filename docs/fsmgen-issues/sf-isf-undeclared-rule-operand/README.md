# FSMGen Bug Report: an undeclared rule operand becomes an implicit module input port

Title: sf-isf-undeclared-rule-operand — `--strict --check` accepts a rule whose RHS names an undeclared identifier, and generation adds it as an input port the author never declared
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/undeclared-width1.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/undeclared-width1.isf
Failure class: accepted-invalid-input
FSMGen commit or capability-manifest producer commit: 5a44ad496c852b168c31d4021cfdb158001a47c7 (upstream HEAD, 2026-09-24); also reproduced on SPECFORGE's pins c0d8b668db2527108d1c23c20d184400c51efea6 and 54ac1c22d5314943027494f4c270f9c612e62704 (published origin/main) — see "Revision note"
SPECFORGE version/commit: specforge 343bd7fa
Expected behavior: --strict --check --json rejects undeclared-width1.isf with one diagnostic naming the undeclared rule operand `VALID` (in rules r_a and r_b), per book 10 "Direct Generation Errors: undeclared internal operands"; generation never adds a module port the actor's (interface ...) does not declare.
Observed behavior: --strict --check --json exits 0 with success:true; --emit-semantic-json lists `VALID` as a module input of width 1; the generated SystemVerilog declares `input wire VALID`. When one driven output is widened to 8 bits (undeclared-mixed.isf) the same implicit input is inferred 8 bits wide and the failure is reported as a width mismatch on rule r_b, not as an undeclared operand.
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 0 — the defect is an acceptance (observed/command-logs/original.exit)
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **bug report** (FSMGen's public contract says the input is an error; FSMGen accepts it).
SPECFORGE owner: `FSMGEN-REQUESTS.2`. The SPECFORGE-side defect that exposed it — SPECFORGE emitting the
prose word `VALID` as a rule value — is owned separately by `KG-ISF-COMPLETENESS.2a.vii.e`; it is not
part of this report.
Local status: see `request.json`.

## 1. The defect in one paragraph

An `.isf` actor declares its ports in `(interface …)`. A rule `(rule r_a (A VALID))` whose right-hand side
names an identifier declared nowhere — not a port, storage variable, constant, parameter or enum member —
is accepted by `--strict --check`. FSMGen does not reject the name: it silently promotes it to a new
top-level **input** port of the generated module, with its width taken from the first signal it is
assigned to. The author's interface no longer describes the module, a typo in a rule value becomes a
new pin, and the width of that invented pin depends on declaration order.

## 2. Reproduction

From the FSMGen repository root, with this directory unpacked there as
`fsmgen-issue-sf-isf-undeclared-rule-operand/`:

```bash
bash fsmgen-issue-sf-isf-undeclared-rule-operand/commands.sh
```

The three inputs are standalone, hand-written actors of 15-16 lines and differ in one place:

| File | Difference | `--strict --check --json` exit | Result |
| --- | --- | ---: | --- |
| `declared-baseline.isf` | `(input VALID)` declared | 0 | `success:true` — correct |
| `undeclared-width1.isf` (primary) | `VALID` declared nowhere; `A`, `B` 1 bit | 0 | `success:true` — **should be rejected** |
| `undeclared-mixed.isf` | as primary, but `(output A (width 8))` | 1 | width-mismatch diagnostic on `B` (below) |

`undeclared-width1.isf`:

```lisp
(actor valid_demo
  (clock clk)
  (reset (rst_n async active_low))
  (interface
    (input start)
    (output A (width 1))
    (output B (width 1))
    (output done))
  (rule r_a
    (A VALID))
  (rule r_b
    (B VALID))
  (transaction tx
    (on start)
    (complete done)))
```

## 3. Evidence (all captured by `commands.sh`)

- **Strict accepts it.** `observed/command-logs/original.exit` and `check-strict.exit` are `0`;
  `observed/stdout/original.stdout` is `success:true` with no diagnostic.
- **An undeclared port is invented.** `observed/json/semantic-default.json`,
  `signal_analysis.inputs` contains `{"direction": "input", "name": "VALID", "width": 1}` — identical to
  the entry the declared baseline produces (`observed/stdout/baseline-semantic.stdout`). FSMGen has
  synthesised exactly the declaration the author omitted.
- **It reaches HDL.** `observed/generated/output.sv` line 16: `input  wire VALID,`; lines 121 and 137:
  `A_next = VALID;`, `B_next = VALID;`.
- **The invented width depends on the first assignee.** `observed/stdout/mixed-check-strict.stdout`:
  `assignment to 'B' uses RHS 'VALID' with incompatible width 8 for LHS width 1; the current contract
  blocks implicit truncation and requires an explicit width-aligned source expression before generation`.
  `VALID` was inferred 8 bits wide from rule `r_a`, and the error is reported against `r_b`, which is
  correct for a declared 1-bit `VALID`. Nothing in the diagnostic says `VALID` is undeclared.

## 4. Why this contradicts the public contract

- `docs/book/src/10-errors-strict-mode-and-troubleshooting.md`, *Direct Generation Errors*, lists
  "undeclared internal operands" among the typical causes the pre-generation validation path "exists
  specifically to catch … before emission".
- `docs/book/src/13a-actor-interface.md`, *Interface*: ports are declared in the singleton `(interface …)`
  clause; nothing documents implicit port creation from a rule operand.
- `docs/book/src/13g-rules.md`: a rule action's RHS is "a scalar RHS or one list expression using the same
  `.fsm` RHS expression domain as transaction `(set var expr)`"; the chapter documents no implicit
  declaration of RHS names. (`13m-local-variables.md` does describe `(set tmp expr)` to an undeclared
  **target** inside a transaction, which is a different construct: a local assignment target, not an
  operand read from outside the actor.)

If implicit input promotion is intended, the report becomes a documentation request: please state it,
state how the width is chosen, and make `--strict` either reject it or name it in a diagnostic.

## 5. Expected behavior

1. `undeclared-width1.isf` under `--strict --check --json`: non-zero exit, `success:false`, a diagnostic
   whose message names `VALID` as an undeclared operand and the rules (`r_a`, `r_b`) that read it.
2. `undeclared-mixed.isf`: the same undeclared-operand diagnostic, reported before any width inference
   (so no width-mismatch diagnostic on `r_b`).
3. `declared-baseline.isf`: unchanged (`success:true`).
4. No generation path adds a port absent from `(interface …)`.

## 6. Secondary observation (not the defect; for triage)

The `undeclared-mixed.isf` diagnostic begins `Source file: '<FSMGEN_ROOT>/.artifacts/tmp/ial1-lowering/
fsmgen-*.fsm'` and names `[OperandContractValidationSupport.pm][validate_pre_generation_operand_contract()]`.
Book 10 states that the combinational self-dependency diagnostics "do not expose parser implementation
filenames, parser routine names, or Perl call-stack frames"; if the same standard is meant to apply to
operand-contract diagnostics, this one does not meet it.

## 7. Impact on SPECFORGE

SPECFORGE relies on `--strict --check` as the contract canary for every emitted `.isf`. Two legacy
SPECFORGE outputs (APB `IHI0024D`, 14 rules; AXI `IHI0022H`, 2 rules) lower the obligation *"X must be
valid"* to `(X VALID)`. They pass strict today only because every signal is 1 bit wide, so the canary
reported them healthy while each generated module carried an invented `VALID` input. SPECFORGE fixes its
own emission under `KG-ISF-COMPLETENESS.2a.vii.e`; this report asks that strict mode stop accepting the
pattern, so the canary catches the next instance.

## 8. Revision note

Captured with `bin/fsmgen-issue-bundle` from a `git archive` export of FSMGen `5a44ad49` inside the
SPECFORGE checkout. The helper recorded the enclosing SPECFORGE commit as FSMGen's head; `env.txt` and
`observed/command-logs/fsmgen-git-head.txt` were corrected by hand to `5a44ad49` (reported separately as
`sf-tool-issue-bundle-foreign-git-head`). The three strict-check results in section 2 were also
reproduced on the pinned `c0d8b668` binary with identical exit codes and messages.

## 9. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>`, `<SPECFORGE_ROOT>`,
  `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `commands.sh` is the helper's script plus a SPECFORGE-added block at its end that runs the two companion
  inputs.

## Re-verification `2026-09-24` on the published tip

SPECFORGE moved its FSMGen pin to `54ac1c22d5314943027494f4c270f9c612e62704`, the published `origin/main`
(`FSMGEN-REFRESH-INTEGRATE-9`). The result above reproduces unchanged there. Note that `5a44ad49` and
`67c8b36d`, named above as upstream HEAD, were commits in an FSMGen checkout ahead of `origin` at capture time.

## FSMGen response (`2026-09-24`, relayed by the owner)

FSMGen has acknowledged this report and is working on it. SPECFORGE's pin `54ac1c22` was pushed before the
report was logged, so the re-verification on it (above) shows the behaviour this report describes, as
expected. SPECFORGE re-runs `commands.sh` on the first pin that contains FSMGen's change, then records
`RESOLUTION.md` and closes the request. Status: `acknowledged`.
