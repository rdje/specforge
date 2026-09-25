# FSMGen Feature Request: a known-value predicate for "X must be valid when Y"

Title: sf-isf-known-value-predicate — the verification property language has `stable`, `changed`, `rose`, `fell` and `past`, but no way to say a signal holds a known value (no `$isunknown`), so a specification's validity obligations cannot be stated
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/known.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/known.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 354844fe1019d2db1e413dd54f8c4a5ae9eb398b (upstream HEAD, 2026-09-25); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note" below
SPECFORGE version/commit: specforge b255e083
Expected behavior: the verification property language has a known-value predicate, (known SIG), true when every bit of SIG is 0 or 1 and lowered to !$isunknown(SIG); --strict --check --json reports success:true for sources/fsmgen-input/known.isf, and the generated module asserts (PSEL & PENABLE) |-> !$isunknown(PADDR).
Observed behavior: --strict --check --json reports success:false with "Unsupported expression operator 'known'" (upstream; the pin says "Unsupported expression"); (! (isunknown PADDR)) is refused the same way, while (stable PADDR) in the same position is accepted.
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `EXTRACTION-REPRESENTATION-GAPS.16` (bundle); SPECFORGE's half, never writing validity as the
value `VALID`, is `KG-ISF-COMPLETENESS.2a.vii.e`.
Local status: see `request.json` (`signed_off`: ready for the owner to forward).

## 1. What SPECFORGE needs

Interface specifications state **validity obligations**: a signal must carry a meaningful value whenever a
qualifying condition holds, and is don't-care otherwise. APB 5 (IHI0024 E): *"The select signal, PSEL, is
asserted, which means that PADDR, PWRITE, and PWDATA must be valid."* AXI and AHB say the same of many payload
signals (*"must be valid when … is asserted"*). In verification terms this is `cond |-> !$isunknown(sig)`: the
property checks that no X or Z reaches a qualified payload. It is not a value to drive, and FSMGen's property
language cannot express it today.

## 2. Measured need (SPECFORGE corpus, 2026-09-25)

Over the three AMBA interface specifications SPECFORGE rebuilds: **28 statements** say *"must be valid"*, and
**17** of them are typed validity obligations on declared signals (APB 5 8, AXI 5, AHB 4). SPECFORGE holds back 11
more temporal rules whose target would otherwise be the word `VALID`/`INVALID`. Re-derive in a SPECFORGE checkout:
`python3 scripts/measure_intent_to_isf_reach.py residuals` (the `isf_temporal_unrepresentable_*` rows) and the
census in `docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (`.16`).

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13k-isf-feature-support-matrix.md`, row *Immediate verification checks*: the property language
  includes *"the SystemVerilog sampled-value predicates `(stable SIG)` / `(changed SIG)` / `(rose SIG)` /
  `(fell SIG)` → `$stable`/`$changed`/`$rose`/`$fell(SIG)`, and value-returning `(past SIG [N])`"*. There is no
  known-value predicate, and the observed diagnostic is FSMGen refusing an operator it does not document.

**Precedent inside FSMGen.** The predicates it already ships are SystemVerilog system functions exposed one to one;
`$isunknown` belongs to the same family, and emitting it under the same `` `ifndef SYNTHESIS `` guard keeps
generated hardware unchanged.

**Relation.** `sf-isf-undeclared-rule-operand` (acknowledged) reported strict mode accepting an undeclared `VALID`
used as a value; SPECFORGE's half of that finding is never to write validity as a value. This request is the
construct that lets SPECFORGE state validity correctly instead.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`. Each input is the same transaction-level assertion with a
different predicate on `PADDR` while `PSEL` and `PENABLE` are asserted.

| Input | Predicate | `--strict --check` | Result |
| --- | --- | --- | --- |
| `baseline-good.isf` | `(stable PADDR)` | exit 0 | accepted |
| `known.isf` (primary) | `(known PADDR)` | exit 1 | *Unsupported expression operator 'known'* (pin: *Unsupported expression*) |
| `isunknown.isf` | `(! (isunknown PADDR))` | exit 1 | same, for `isunknown` |

## 5. Requested capability

- **R1 (required) — `(known SIG)`.** True when every bit of `SIG` is 0 or 1; lowered to `!$isunknown(SIG)` inside
  the verification property, verification-only like the other predicates. Failure cases: an undeclared signal, or
  use outside the verification family (a rule guard or a drive value), fails closed.
- **R2 (optional) — `(isunknown SIG)`.** The direct spelling of `$isunknown`, for authors who state the negative.
- **R3 (optional) — several signals.** `(known A B C)` as sugar for their conjunction, since specifications
  qualify payload groups together (*"PADDR, PWRITE, and PWDATA must be valid"*).

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `known.isf` passes `--strict --check`, and its generated module contains, under `` `ifndef SYNTHESIS ``, an
   `assert property` over `((PSEL & PENABLE)) |-> (!$isunknown(PADDR))` or an equivalent.
2. `isunknown.isf` (if R2 ships) generates the equivalent property.
3. `(known …)` in a rule guard or as a drive value fails under `--strict` with a diagnostic.
4. `baseline-good.isf` is unchanged.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **Lower validity as a value** (`(PADDR VALID)`): wrong; `VALID` is not a value (and strict mode accepting it is
  the reported `sf-isf-undeclared-rule-operand`).
- **Approximate with `(stable …)`**: stability is a different obligation; a signal can be stable and unknown.
- **Drop validity obligations** (what SPECFORGE does today): the specification's checkable obligation never
  reaches generated verification.

## 7a. What SPECFORGE does until this ships

SPECFORGE keeps validity obligations in its IR and lists them in adapter residuals (the target word `VALID` is
refused as not an ISF literal); it never lowers them as values.

## 8. Compatibility

Additive: a new predicate in the verification family; no accepted input changes meaning.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `354844fe1019d2db1e413dd54f8c4a5ae9eb398b` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same three inputs: identical exit codes; the pin's
diagnostic is the less specific *Unsupported expression*). FSMGen's feature matrix is byte-identical at the two
revisions.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* is corrected by hand: the helper recorded SPECFORGE's commit, the defect
  reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop at its end.
