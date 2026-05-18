# FSMGen Issue Bundle: sf-isf-stage-ready-valid

Title: sf-isf-stage-ready-valid
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/f2-stage-ready-valid.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/f2-stage-ready-valid.isf
Failure class: rejected-input
FSMGen commit or capability-manifest producer commit: see observed/command-logs/fsmgen-git-head.txt and observed/json/fsmgen-capability-manifest.json
SPECFORGE version/commit: specforge 1c3a5d37
Expected behavior: Per ISF_DOWNSTREAM_INTEGRATION_SPEC.md section 11.8 the stage is documented as (stage phase_name (ready ready_signal) (valid valid_signal)) and ready_valid_barrier is named the 'Current shipped stage kind'; this documented shipped form should pass --strict --check --json.
Observed behavior: fsmgen --strict --check --json exits 255 with empty stdout (no JSON despite --json) and stderr: Transaction 'txn_demo': stage 's_demo' has unsupported subclause 'ready'.
First failing command or API call: original (./bin/fsmgen --strict --check --json CASE)
Exit status or exception: 255, empty stdout (no JSON despite --json); diagnostic on stderr
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes


## Minimization note

Derived from a real SPECFORGE-emitted `.isf` (AMBA ATP corpus) reduced so
the only deviation from a strict-accepted transaction is the
`(stage s_demo (ready SIGNAL_NAME) (valid ADDRESS))` clause (spec §11.8
shipped `ready_valid_barrier`). `expected/baseline-good.isf` is the
strict-accepted counterpart (`success:true`, see
`expected/baseline-good.strict-check.json`); the two inputs differ by exactly
that one line, so the rejection is provably caused by the stage clause. No
proprietary identifiers; signal names come from a public corpus artifact
already in this repository.

## Triage pointer

Spec §11.8 prints `(stage phase_name (ready ready_signal) (valid
valid_signal))` and names `ready_valid_barrier` the "Current shipped stage
kind". The shipped `--strict --check` rejects that exact documented form with
"stage '<p>' has unsupported subclause 'ready'". Per
`DOWNSTREAM_ISSUE_REPORTING.md` §9 this is either a strict-checker bug or a
spec-doc bug. Secondary: the strict rejection exits 255 with no JSON on
stdout despite `--json`.
