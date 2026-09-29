# FSMGen Question: which `.isf` form states "the specification names no reset pin"?

Title: sf-isf-unstated-reset — an omitted `(reset …)` becomes `(rst_n async active_low)` and is reported exactly like a stated one, and a clock-only domain is accepted with no reset; the book does not say which form states an unspecified reset honestly
FSMGen command or public API entrypoint: ./bin/fsmgen --strict --check --json sources/fsmgen-input/omitted-reset.isf
FSMGen-facing primary artifact path: sources/fsmgen-input/omitted-reset.isf
Failure class: unknown
FSMGen commit or capability-manifest producer commit: 8513cef4254db4572ab8dd3c94f3953df358f0a9 (upstream HEAD, 2026-09-28); also reproduced on SPECFORGE's pin 54ac1c22d5314943027494f4c270f9c612e62704 — see "Revision note"
SPECFORGE version/commit: specforge 6d2eb6e60
Expected behavior: (this is a question) the book states which .isf form SPECFORGE should write for a specification that names no reset pin, and what reset semantics and report FSMGen gives each accepted form
Observed behavior: --strict --check --json accepts an actor with no (reset ...) and its semantic JSON reports explicit_system_contract reset rst_n, reset_kind async, reset_active_level 0, identical to the input that states (reset (rst_n async active_low)) apart from the input path; a single (clock-domains (domain link (clock clk))) with no reset is accepted too and reports reset null
First failing command or API call: none; every input is accepted (observed/command-logs/*-check-strict.exit)
Exit status or exception: 0 — accepted (observed/command-logs/original.exit)
Is the attached artifact bundle minimized: yes
Does the minimized/redacted bundle still reproduce: yes

Request kind: **question** (FSMGen's contract is silent on the case; SPECFORGE needs the answer before it can choose
a form). SPECFORGE owner: `FSMGEN-REQUESTS.9`; ledger record `unstated-reset`.
Local status: see `request.json` (`signed_off`; waiting for the owner to forward it).

## 1. The question

SPECFORGE lowers only what a specification states. Some specifications state no reset pin at all: USB 3.2
Revision 1.0's link layer has no reset signal, its resets are protocol states (`Warm Reset`, `Hot Reset`), and its
power-on behaviour is a state of its link-training state machine. A controller built from it will have a reset,
but its name, kind and polarity are the implementer's choice, not the specification's.

Two `.isf` forms are accepted for that case, and they mean different things:

- **Omit `(reset …)`.** Book 13a, *Reset*: *"If a legacy single-clock actor omits `(reset ...)`, the parser
  normalizes its reset to asynchronous active-low `rst_n`."* The semantic JSON then reports
  `explicit_system_contract` `reset: rst_n`, `reset_kind: async`, `reset_active_level: 0`, **byte-identical** to an
  input that states `(reset (rst_n async active_low))` apart from the input path. A downstream reader cannot tell
  a default from a stated reset.
- **A clock-only domain.** Book 13a, *Multi-Clock Domains*, accepts a domain without a reset ("clock-only no-reset
  domains"), and a single-domain block lowers through the single-clock path. Its semantic JSON reports `reset:
  null`. The book does not say what state such an actor's generated FSM starts in, or whether this form is meant
  for anything beyond CDC endpoints.

**Our questions:**

1. Which form should SPECFORGE write when the specification states no reset pin?
2. If it is the clock-only form: what is the generated FSM's initial state and its registers' initial values
   without a reset, and is a single-domain `(clock-domains …)` block supported for that purpose, or only for CDC?
3. If it is the omitted form: could the report distinguish a defaulted reset from a stated one, for example with
   a provenance field in `explicit_system_contract`? Without it, a stated `rst_n async active_low` and an unstated
   reset are the same artifact.
4. Or is a form missing, one that says "reset kind and polarity are chosen at instantiation" (for example
   parameters for kind and polarity)? If so, SPECFORGE would file it as a feature request.

## 2. Why this is a question, not a bug

FSMGen does what its book says in both cases: the default is documented, and the clock-only domain is documented
for CDC. The book does not address a specification that names no reset, which is the case SPECFORGE needs decided
before it can lower one faithfully.

## 3. Reproduction

From the FSMGen root: `bash <bundle>/commands.sh`. Each input differs from `baseline-good.isf` in its reset only;
`commands.sh` strict-checks each one and captures its semantic JSON (`observed/stdout/*-semantic.stdout`).

| Input | Reset | `--strict --check` | Semantic JSON `explicit_system_contract` |
| --- | --- | --- | --- |
| `baseline-good.isf` | `(reset (rst_n async active_low))` | exit 0 | `rst_n`, `async`, active level 0 |
| `omitted-reset.isf` (primary) | none | exit 0 | the same, byte-identical apart from the input path |
| `clock-domain-no-reset.isf` | `(clock-domains (domain link (clock clk)))`, no reset | exit 0 | `reset: null` |

## 4. Impact on SPECFORGE

SPECFORGE's `.isf` adapter refuses to lower a document whose clock and reset it cannot ground in the
specification's own sentences (`no source-grounded system clock/reset contract`). USB 3.2's adapter is blocked by
that rule. SPECFORGE deliberately does not rely on the omitted form's default, because the emitted artifact would
then state an asynchronous active-low reset the specification never states. The answer decides whether SPECFORGE
can lower such a document with the reset left open, and how.

## 5. Revision note

Reproduced on FSMGen upstream HEAD `8513cef4254db4572ab8dd3c94f3953df358f0a9` (a read-only `git archive` export,
where `commands.sh` ran and generated `observed/`) and on SPECFORGE's pin
`54ac1c22d5314943027494f4c270f9c612e62704` (strict checks of the same three inputs: all accepted). The book text
cited in §1 is identical at the two revisions. Each installation passed a known-good strict check before anything
was compared.

## 6. Redactions and simplifications

- Absolute paths in captured outputs are replaced by `<FSMGEN_ROOT>` (the FSMGen export the helper ran in),
  `<SPECFORGE_ROOT>`, `<FSMGEN_CHECKOUT>` and `<HOME>`. No other byte of any captured output changed.
- `env.txt`'s *FSMGen git HEAD* and `observed/command-logs/fsmgen-git-head.txt` are corrected by hand: the helper
  recorded SPECFORGE's commit, the defect reported as `sf-tool-issue-bundle-foreign-git-head`.
- `commands.sh` is the helper's script plus the SPECFORGE-added companion loop at its end.
