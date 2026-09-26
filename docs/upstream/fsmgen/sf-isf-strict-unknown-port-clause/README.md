# FSMGen Question: should strict mode reject an interface-port clause it does not recognise?

Title: sf-isf-strict-unknown-port-clause — `--strict` accepts an interface-port clause it does not recognise and ignores it; the book does not say whether that is intended
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/bogus-clause.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/bogus-clause.isf
Failure class: unknown
FSMGen commit or capability-manifest producer commit: 354844fe1019d2db1e413dd54f8c4a5ae9eb398b (upstream HEAD, 2026-09-25); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note"
SPECFORGE version/commit: specforge 27e9a39e
Expected behavior: (this is a question) either --strict --check --json refuses an interface-port clause it does not recognise, such as (bogus-clause xyz), with a diagnostic naming it, or FSMGen's book documents that strict mode accepts and ignores unrecognised port clauses
Observed behavior: --strict --check --json accepts (output AWPROT (width 3) (bogus-clause xyz)) with success:true and no diagnostic, and the generated SystemVerilog declares and drives AWPROT exactly as the port without the clause does; the plausible but undocumented (present PROT_Present) behaves the same
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE); it is an acceptance, so nothing fails
Exit status or exception: 0 — accepted (observed/command-logs/original.exit)
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **question** (FSMGen's contract is silent; SPECFORGE needs the answer before it can tell a bug from
intended behaviour). SPECFORGE owner: `EXTRACTION-REPRESENTATION-GAPS.19`; ledger record
`strict-unknown-port-clause`.
Local status: see `request.json` (`forwarded` by the owner on `2026-09-25`; `acknowledged` by FSMGen on `2026-09-26`, owner `SEMANTIC-INTROSPECTION-MCP-FRONTIER.33.2`).

## 1. The question

An `.isf` interface port accepts `(width …)`/`(type …)` and `(domain …)` (book 13k, row *Interface ports*). A
port that also carries a clause FSMGen does not recognise, `(output AWPROT (width 3) (bogus-clause xyz))`, passes
`--strict --check` with no diagnostic, and the generated SystemVerilog is identical to the port without the
clause. A plausible clause fares the same: `(present PROT_Present)`, which SPECFORGE asks for in
`sf-isf-optional-signal`, is accepted and ignored, so the port is still generated. **Is silent acceptance of an
unrecognised port clause intended?** If it is not, SPECFORGE would file it as a bug. Either way we ask that the
answer be documented, because a strict pass cannot otherwise show that a clause SPECFORGE emitted was understood.

## 2. Why this is a question, not a bug

Book 10 (*Strict Mode*) says strict mode *"exists to narrow the language toward the supported intentional
surface"*, and 13k lists the port grammar and what fails closed (unknown symbolic width names, zero-valued
parameters, arbitrary width expressions, …). Neither says an unrecognised port clause is an error. Unknown
*transaction* and *actor* clauses are refused (seen while building `sf-isf-transaction-phase-groups`), so ports may
simply be the one place this was not decided. Both passages are identical at the pin and at upstream HEAD.

## 3. Reproduction

From the FSMGen repository root, with this directory unpacked there as
`fsmgen-issue-sf-isf-strict-unknown-port-clause/`:

```bash
bash fsmgen-issue-sf-isf-strict-unknown-port-clause/commands.sh
```

| Input | The port | `--strict --check` | Generated SystemVerilog |
| --- | --- | --- | --- |
| `baseline-good.isf` | `(output AWPROT (width 3))` | exit 0 | the reference |
| `present-when.isf` | `… (present PROT_Present)` | exit 0, no diagnostic | identical to the reference apart from the `// Date:` line (`observed/command-logs/present-when-vs-baseline.txt`) |
| `bogus-clause.isf` (primary) | `… (bogus-clause xyz)` | exit 0, no diagnostic | identical likewise (`bogus-clause-vs-baseline.txt`) |

The three inputs are 13-line actors and differ only in that clause.

## 4. What SPECFORGE does until it is answered

SPECFORGE emits no port clause beyond the documented ones, and treats a strict pass as proof only of what the
book documents.

## 5. Revision note

Reproduced on SPECFORGE's pin `54ac1c22` and on FSMGen's upstream HEAD `354844fe1`, a complete `git archive`
export; both accept all three inputs with `success:true`. The question was first forwarded on `2026-09-25` as a
ledger record; this packet was made afterwards under SPECFORGE's ADR 0054, which gives every question the same
packet as a request, and it adds the `bogus-clause` probe and the generated-module comparison.

## 6. Redactions and simplifications

Paths are normalised to `<FSMGEN_ROOT>`, `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. `env.txt`'s FSMGen
revision is corrected by hand, because the helper, run in an export that is not a git repository, records
SPECFORGE's commit (`sf-tool-issue-bundle-foreign-git-head`).
