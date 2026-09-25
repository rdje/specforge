# FSMGen Issue Bundle: sf-isf-contract-eventually-flat

Title: sf-isf-contract-eventually-flat
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/f1-contract-eventually-flat.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/f1-contract-eventually-flat.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: see observed/command-logs/fsmgen-git-head.txt and observed/json/fsmgen-capability-manifest.json
SPECFORGE version/commit: specforge 1c3a5d37
Expected behavior: Per ISF_DOWNSTREAM_INTEGRATION_SPEC.md section 11.8 the temporal contract is documented as (contract name (eventually signal within N)) using the flat 'within N'; this documented shipped bounded_eventually form should pass --strict --check --json.
Observed behavior: fsmgen --strict --check --json exits 255 with empty stdout (no JSON despite --json) and stderr: Transaction 'txn_demo': contract 'c_demo' supports only '(eventually signal (within cycles))'. The nested (eventually signal (within N)) form is accepted.
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 255, empty stdout (no JSON emitted despite --json); diagnostic on stderr
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

## Minimization note

The case is derived from a real SPECFORGE-emitted `.isf`
(`generated/adapters/isf/ihi0082_a_2019_03_15_amba_adaptive_traffic_profiles_specification/agent.isf`),
which on its own passes `./bin/fsmgen --strict --check --json` cleanly
(`success: true`, `diagnostic_count: 0`). One transaction was appended whose
only deviation from a strict-accepted form is the contract clause:

- accepted (nested): `(contract c_demo (eventually ADDRESS (within 4)))`
- rejected (this case, spec §11.8 flat form): `(contract c_demo (eventually ADDRESS within 4))`

`expected/baseline-good.isf` is that strict-accepted nested counterpart;
`expected/baseline-good.strict-check.json` is its captured `success: true`
result. The two inputs differ by exactly that one line, so the rejection is
provably caused by the flat `within N` form and nothing else. No proprietary
identifiers are present; signal names come from the public AMBA ATP corpus
artifact already in this repository.

## Triage pointer

Spec `ISF_DOWNSTREAM_INTEGRATION_SPEC.md` §11.8 documents the temporal
contract as `(contract name (eventually signal within N))` (flat `within N`)
and names `bounded_eventually` the "Current shipped temporal contract kind".
The shipped `--strict --check` rejects that exact documented form and accepts
only the nested `(eventually signal (within N))`. Per
`DOWNSTREAM_ISSUE_REPORTING.md` §9 this is either a strict-checker bug or a
spec-doc bug; the bundle is the starting point for that triage. A secondary
observation: the strict rejection exits 255 with no JSON on stdout even though
`--json` was requested, so downstream tools cannot map this failure through the
documented JSON check surface.
