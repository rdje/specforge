# FSMGen Feature Request: a record-typed interface port, for packets and headers

Title: sf-isf-aggregate-interface-ports — a declared record type is accepted on actor-owned storage only; an interface port that carries a packet header cannot be typed, and its fields cannot be read by name
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/record-port.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/record-port.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: 8513cef4254db4572ab8dd3c94f3953df358f0a9 (upstream HEAD, 2026-09-28); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note"
SPECFORGE version/commit: specforge 6d2eb6e60
Expected behavior: an interface port may carry a declared record type, (input lmp_in (type lmp_t)); a transaction reads its fields by name (lmp_in.subtype) in guards and assignments, may copy it whole into record storage, and the generated HDL flattens it to the record's packed width
Observed behavior: --strict --check --json refuses both inputs with "interface port 'lmp_in' references aggregate type 'lmp_t'; this ISF slice accepts aggregate type aliases only on actor-owned storage variables"
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 1 (observed/command-logs/original.exit); JSON diagnostic in observed/stdout/original.stdout
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **feature request** (FSMGen follows its current public contract; the contract lacks a capability).
SPECFORGE owner: `FSMGEN-REQUESTS.10`; ledger record `aggregate-interface-ports`.
Local status: see `request.json` (`signed_off`; waiting for the owner to forward it).

## 1. What SPECFORGE needs

A link-layer or message protocol exchanges packets, not wires. What its specification states about an interface is
the layout of each packet and header and the behaviour keyed on their fields. USB 3.2 Revision 1.0 defines Link
Management Packets, Transaction Packets, Data Packet headers and Isochronous Timestamp Packets as field layouts
(a type field, a subtype field, payload fields, CRCs), and its link layer acts on those fields: an LMP's subtype
selects a port capability, a power-management request or a timing exchange. A faithful `.isf` port for such an
interface carries the header as a typed value whose fields the behaviour reads by name.

## 2. Measured need (SPECFORGE corpus, 2026-09-29)

On SPECFORGE's 31 current chains, **191 untyped tables** are structure or message layouts that no SPECFORGE record
carries yet; **75** of them are USB 3.2's, the rest from USB4 Inter-Domain (29), CHI C2C (22), OpenCAPI Data Link
Layer (15), AXI (13) and others. Re-derive in a SPECFORGE checkout:

```bash
python3 scripts/measure_untyped_table_families.py --json families.json > /dev/null
python3 -c 'import json
d = json.load(open("families.json"))["documents"]["structure_or_message_layout"]
print(sum(d.values()), sorted(d.items(), key=lambda kv: -kv[1])[:5])'
```

## 3. Current public contract (why this is a feature request, not a bug)

- `docs/book/src/13j-type-enum-aggregate.md`: packed `record` and `list` aliases *"are currently accepted only as
  actor-owned storage"*; *"Aggregate interface ports, transaction-local ports, storage banks, and other aggregate
  carriers are not shipped yet. They require separate ownership …"*.
- `docs/book/src/13k-isf-feature-support-matrix.md`, row *Interface ports*: `(type T)` on a port is shipped for
  scalar aliases; the residual list names *"Aggregate interface ports, transaction-local aggregate ports, aggregate
  …"* as not shipped.
- The observed diagnostic is FSMGen enforcing that boundary.

Both are identical at the pin and at upstream HEAD. **Precedent:** FSMGen's `2026-06-22` answer to SPECFORGE's
field-structured storage request (`docs/SPECFORGE_FEEDBACK_RESPONSE.md`, *Declarative Field-Structured Storage*)
named *"whether packet/structure layout generalization ships with scalar storage fields or follows as a sibling
feature"* as an open question, and shipped storage fields only. This request asks for the port half, and brings a
consumer for it.

## 4. Reproduction

Run from the FSMGen root: `bash <bundle>/commands.sh`. Each input declares `(type lmp_t (record (subtype (bits 4))
(payload (bits 12))))`.

| Input | Difference | `--strict --check` | Result |
| --- | --- | --- | --- |
| `baseline-good.isf` | the record on storage only, fed from a scalar port (`(set last_lmp.subtype subtype_in)`) | exit 0 | accepted |
| `record-port.isf` (primary) | `(input lmp_in (type lmp_t))`, copied whole into record storage | exit 1 | *interface port 'lmp_in' references aggregate type 'lmp_t'; this ISF slice accepts aggregate type aliases only on actor-owned storage variables* |
| `record-port-field-guard.isf` | the same port, one field read in a guard (`(when (== lmp_in.subtype 1) …)`) | exit 1 | the same diagnostic |

## 5. Requested capability

- **R1 (required) — record-typed ports.** An interface input or output may declare a record type from the actor's
  `(types …)` or an imported package. Failure cases: an undeclared type, or a type that is not packed, fails closed
  naming the port.
- **R2 (required) — field access.** A transaction reads an input record port's fields by name wherever it reads a
  scalar (guards, `set`, `update`, rules), drives an output record port's fields by name, and copies a whole record
  between a port and record storage of the same type.
- **R3 (required) — lowering.** The generated `.fsm` and HDL carry the port at the record's packed width with a
  documented field order, and the schedule and semantic reports name the port's type and field layout.
- **R4 (optional) — list-typed ports** by the same rules, for multi-beat or multi-lane payloads.
- **R5 (optional) — variant records.** A header whose layout depends on one of its fields (a type or subtype field
  selecting the rest). SPECFORGE lists this as optional because it may belong to a later layer.

## 6. Acceptance criteria (proposed, for FSMGen's regression tests)

1. `record-port.isf` passes `--strict --check`; the generated HDL declares `lmp_in` 16 bits wide, and the
   semantic report names its type `lmp_t` and fields `subtype` and `payload`.
2. `record-port-field-guard.isf` passes; the generated guard tests the port's `subtype` bits.
3. A port typed with an undeclared record fails under `--strict` naming the port and the type.
4. `baseline-good.isf` keeps its current result.

## 7. Alternatives SPECFORGE considered and why they are not faithful

- **One scalar port per field**: splits one packet into unrelated wires the specification never names, and loses
  which fields belong to which packet.
- **One wide scalar port plus bit slicing**: the layout survives only as magic bit ranges that nothing checks.
- **Record storage fed from scalar ports** (the baseline): the port is still untyped, and the field layout exists
  only inside the actor.
- **Keep layouts in SPECFORGE's IR** (what SPECFORGE does today, once it reads them): no FSMGen artifact carries
  them.

## 7a. What SPECFORGE does until this ships

SPECFORGE does not yet read these layout tables into records (`DOC-INTENT-TAXONOMY.4b` owns that), and it lowers
no link-layer interface. USB 3.2 is classed `physical-link` in SPECFORGE's document categories, whose default is not
to generate executable output; its owner asked on `2026-09-29` for these link-layer requests.

## 8. Compatibility

Additive: every currently accepted input keeps its meaning; only a port type FSMGen refuses today gains one.

## 9. Revision note

Reproduced on FSMGen upstream HEAD `8513cef4254db4572ab8dd3c94f3953df358f0a9` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same three inputs: identical exit codes and
diagnostics). The contract text cited in §3 is identical at the two revisions. Each installation passed a
known-good strict check before anything was compared.

## 10. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* and `observed/command-logs/fsmgen-git-head.txt` are corrected by hand: the helper
  recorded SPECFORGE's commit, the defect reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop at its end.
- The USB 3.2 description in §1 summarises its packet chapters; no corpus content is needed to reproduce.
