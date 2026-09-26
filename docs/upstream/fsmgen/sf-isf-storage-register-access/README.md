# FSMGen Feature Request: a storage register's own access, as checked metadata

Title: sf-isf-storage-register-access — a scalar storage `var` cannot state its own access (`RO`, `RW`, `WO`); only a field inside `(fields …)` can
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/register-access.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/register-access.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 5be6a3c086ae626ebb2b3acec572876fd33e9344 (upstream HEAD, 2026-09-26); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note" below
SPECFORGE version/commit: specforge cd95e66b
Expected behavior: a scalar storage var may state its own access as metadata, (var status (width 8) (reset 0) (access ro)), with the token set fields already accept; --strict --check --json reports success:true for sources/fsmgen-input/register-access.isf, the schedule report's inferred_storage entry for status carries "access": "ro", generated .fsm and HDL are unchanged, and an unknown token fails closed.
Observed behavior: --strict --check --json reports success:false with "actor 'csr_access' storage 'status' has unsupported option 'access'"; only a (field …) inside (fields …) may carry (access …).
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); JSON diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `FSMGEN-REQUESTS.5` (bundle, and lowering SPECFORGE's register access once this ships).
Local status: see `request.json` (`forwarded` by the owner on `2026-09-26`; `acknowledged` by FSMGen the same day).

## 1. What SPECFORGE needs

A register summary table states the access of each register as a whole, not only of its fields. CoreSight TMC
(Arm DDI 0461B), register summary:

| Offset | Name | Type | Reset |
| --- | --- | --- | --- |
| `0x00C` | `STS` | `RO` | `0x0000000C` |
| `0x020` | `CTL` | `RW` | `0x00000000` |
| `0x024` | `RWD` | `WO` | `-` |

ADIv6 (Arm IHI 0074A) states the same `Type` column for every access-port register (`CSW` `RW`, `IDR` `RO`,
`LAR` `WO`, …). SPECFORGE reads this into a register-level access, kept apart from each field's own access. It
lowers each register to a storage `var`, and each located field to `(fields (field …))`. A field's access
reaches the `.isf`; the register's own access has nowhere to go, so it stops at SPECFORGE's IR.

## 2. Measured need (SPECFORGE corpus, 2026-09-26)

**101 registers** carry a register-level access in the two current SPECFORGE chains that state one:
TMC 50 of 50 registers, ADIv6 51 of 82. 98 of the 101 use a token FSMGen's field access already accepts
(`RO`, `RW`, `WO`); 3 carry a source footnote (`RO,RW a`, `WO b`). Of the 101, **61 have no field at all**, so
the register access is the only access fact about them, and 40 have fields, none of which states an access.
Re-derive in a SPECFORGE checkout:

```bash
python3 -c 'import json
for k in sorted(json.load(open("doctrine/chain_currency/retained_bundles.json"))["retained"]):
    regs = json.load(open(f"generated/intent_ir/{k}/intent_ir.json")).get("register_records") or []
    acc = [r for r in regs if r.get("access_type")]
    if acc: print(k, len(regs), len(acc), sum(not r.get("fields") for r in acc))'
```

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13a-actor-interface.md`, *Declarative Storage Fields*: *"Optional
  `(access ro|rw|wo|w1c|w0c|rc|rs|warl|wpri|reserved)` is metadata only"* — on a `(field …)`. The parent `var`
  takes `width`, `reset`, `fields` and, with clock domains, `domain`.
- `docs/book/src/13k-isf-feature-support-matrix.md`, row *Actor-owned scalar storage*:
  `(storage (var NAME (width …) [(reset V)] [(fields FIELD...)]) ...)`; *"Field metadata is checked and
  report-only through `inferred_storage[].fields`"*.
- The observed diagnostic, *"storage 'status' has unsupported option 'access'"*, is FSMGen enforcing that shape.

Both sections are identical at the pin and at upstream HEAD.

**Precedent inside FSMGen.** FSMGen's `2026-06-22` answer to SPECFORGE's field-structured storage request
accepted *"optional access metadata such as `ro`, `rw`, `w1c`, or `warl`"* on fields, checked and report-only, and
shipped it the same day (`docs/SPECFORGE_FEEDBACK_RESPONSE.md`, *Declarative Field-Structured Storage*). This
asks for the same metadata one level up, on the word the fields partition.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`. Each input differs from `baseline-good.isf` in one
storage entry.

| Input | Storage entry | `--strict --check` | Result |
| --- | --- | --- | --- |
| `baseline-good.isf` | `(var status (width 8) (reset 0))`; `control` has a field with `(access rw)` | exit 0 | accepted |
| `register-access.isf` (primary) | `(var status (width 8) (reset 0) (access ro))` | exit 1 | *storage 'status' has unsupported option 'access'* |
| `register-access-with-fields.isf` | `(var control (width 8) (reset 0) (access rw) (fields …))` | exit 1 | *storage 'control' has unsupported option 'access'* |
| `unknown-token.isf` | `(var status (width 8) (reset 0) (access sometimes))` | exit 1 | same today; **must stay refused** once supported, because `sometimes` is not an access token |
| `whole-word-field.isf` | `status` given one field, `(field status (bits 7 0) (access ro))` | exit 0 | accepted: the workaround in §7 |

`commands.sh` also captures the schedule report of `baseline-good.isf` and `whole-word-field.isf`
(`observed/stdout/*-schedule.stdout`): `inferred_storage[]` reports access only inside `fields`, and the
workaround reports a field named `status` that no specification states.

## 5. Requested capability

- **R1 (required) — word access on scalar storage.** `(access TOKEN)` on a width-based storage `(var …)` or
  `(variable …)`, with exactly the token set field access accepts. Metadata only: it changes no scheduled `.fsm`,
  HDL or reset behavior, as field access does not. Failure cases: an unknown token or a repeated `(access …)` on
  one entry fails closed with a diagnostic naming the storage entry; until R4, so does an `(access …)` on a bank.
- **R2 (required) — independent of fields.** A `var` may carry `(access …)` with or without `(fields …)`, and
  neither is derived from the other: the word's access is not copied into its fields, and fields do not imply a
  word access. Specifications state both independently (a read-write control register routinely holds read-only
  or reserved bits), so no consistency rule between them is requested.
- **R3 (required) — reported.** The schedule report's `inferred_storage[]` entry carries `"access"` beside
  `width` and `fields`, wherever field access is reported today.
- **R4 (optional) — banks.** The same metadata on a `(bank …)` entry, for a memory region a specification
  declares read-only as a whole.

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `register-access.isf` passes `--strict --check`; its `inferred_storage[]` entry `status` carries
   `"access": "ro"`; its scheduled `.fsm` and generated HDL equal those of `baseline-good.isf`.
2. `register-access-with-fields.isf` passes; the `control` entry carries `"access": "rw"` and the same `fields`
   as in `baseline-good.isf`.
3. `unknown-token.isf` fails under `--strict` with a diagnostic naming storage `status` and the token `sometimes`.
4. `baseline-good.isf` and `whole-word-field.isf` keep their current results and reports.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **A whole-word field** (`whole-word-field.isf`, accepted today): declares a field the specification does not
  state, and it cannot coexist with the register's real fields, since overlapping ranges fail closed.
- **Copy the register's access into each field**: invents per-field facts; a `RW` register with `RO` or
  reserved bits would be misreported.
- **Comments**: nothing checks or reports them.
- **Keep it in SPECFORGE's IR** (what SPECFORGE does today): the fact never reaches FSMGen's report or any
  downstream tool.

## 7a. What SPECFORGE does until this ships

SPECFORGE keeps each register's access in its IR (`RegisterRecord.access_type`, separate from each field's
`access_type`) and lowers only field access to the `.isf`. It never copies a register's access into an invented
field.

## 8. Compatibility

Additive: every currently accepted input keeps its meaning; only a `var` option FSMGen refuses today gains one.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `5be6a3c086ae626ebb2b3acec572876fd33e9344` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same five inputs: identical exit codes). The
contract text cited in §3 is identical at the two revisions. Each export passed a known-good strict check
(`isf/storage_fields.isf`: exit 0, no stderr) before anything was compared.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* and `observed/command-logs/fsmgen-git-head.txt` are corrected by hand: the helper
  recorded SPECFORGE's commit, the defect reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loops at its end.
- The TMC rows in §1 are copied from SPECFORGE's SourceIR of the register summary table; no corpus content is
  needed to reproduce.
