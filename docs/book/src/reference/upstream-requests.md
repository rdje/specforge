# Requests To Upstream Projects

SpecForge depends on upstream projects; today there is one, FSMGen, which turns SpecForge's `.isf` into state
machines and HDL. Whatever SpecForge sends upstream travels the same way: a **bug report** (the project breaks its
own published contract), a **feature request** (the contract lacks something a specification needs) or a
**question** (the contract is silent or ambiguous). Each is a **packet**, `docs/upstream/<project>/<id>/`, that the
project can reproduce on its own machine with one command where a probe applies. The owner forwards it; SpecForge
tracks it until the change or the answer is in the revision SpecForge uses, or it is closed another way.

The full procedure for FSMGen is [`docs/upstream/fsmgen/PROCESS.md`](../../../upstream/fsmgen/PROCESS.md). This
page explains it and lists the packets SpecForge is tracking. Which gaps are FSMGen's to close, and which are
SpecForge's, is kept in [Extraction And Representation Gaps](representation-gaps.md).

**One channel since `2026-09-25`** (ADR 0054). Before, bug reports and feature requests were packets but a question
was only a record in `docs/FSMGEN_FEEDBACK.md`, with no sign-off and no checked forwarding date. Now every kind is a
packet with the same lifecycle, and the ledger is an index of the packets.

## Bug, feature, or our own defect?

Every finding is classified against FSMGen's current documentation before anything is built:

| What the evidence shows | What SpecForge files |
| --- | --- |
| FSMGen rejects what its documentation says is supported, accepts what it says is an error, or produces wrong output | a **bug report** |
| FSMGen does what its documentation says, but SpecForge needs something it does not offer | a **feature request** |
| FSMGen is right and SpecForge produced a wrong `.isf` | nothing to FSMGen — a SpecForge task fixes it |
| The documentation does not say | a **question**, as a packet like the others |

A finding can be two things. When SpecForge wrote the prose word `VALID` as a rule value, that was
SpecForge's defect; FSMGen *accepting* it, against its own rule that undeclared operands are errors, was
FSMGen's. Each half has its own owner.

## What makes a packet ready

A packet of any kind is signed off only when all nine points of its `SIGNOFF.md` hold, and a check that runs on every
SpecForge commit refuses the commit otherwise:

1. **Classified**, with the FSMGen documentation text that decides it quoted.
2. **Reproduced twice**: on the FSMGen revision SpecForge is pinned to, *and* on FSMGen's current
   upstream revision, because a request against an old revision may ask for something that already
   exists.
3. **Standalone**: its `commands.sh` runs from FSMGen's root using only files in the bundle, and every
   captured output came from that script.
4. **Minimal**: the inputs are small hand-written examples, each differing from its companion in one
   place, with the accepted counterpart included.
5. **Testable expectation**: `expected/expected-notes.md` says what a regression test should assert.
6. **Not a duplicate** of an earlier request or of an answer FSMGen already gave.
7. **Portable**: no path from this machine appears anywhere in it.
8. **Owned** by a task leaf and by an open record in `docs/FSMGEN_FEEDBACK.md`.
9. **SpecForge's own part separated**, with its own owner, so FSMGen is never asked to fix SpecForge.

## Lifecycle

`drafted` → `signed_off` (ready to forward) → `forwarded` → `acknowledged` → `resolved_upstream` →
`integrated`; or `rejected` / `withdrawn` with the reason written down. For a question, `resolved_upstream`
means answered and `integrated` means SpecForge acted on the answer. Each packet's `request.json` holds the
current state, and the project's ledger (`docs/FSMGEN_FEEDBACK.md` for FSMGen) indexes it: every outgoing record
has exactly one packet, cites its README, and agrees with it on kind and status, which the check enforces. A
request is closed only after SpecForge has moved to an FSMGen revision containing the change and re-run the
packet against it.

**When you forward one:** send its directory, then record the date and channel in its `request.json`
`forwarded` field and set its `status` to `forwarded`. The check then requires those details, and
the feedback record moves to `awaiting_response` in the same commit.

## Current packets (FSMGen)

The first three were forwarded and acknowledged by FSMGen on `2026-09-24`. FSMGen has since fixed the undeclared
rule operand in its development line (`c6b8bc783`, checked by SpecForge on `2026-09-25`); it is not published yet,
and the pin refresh `FSMGEN-REFRESH-INTEGRATE-10` brings it in once it is. The requests below them come from the gap ledger
([Extraction And Representation Gaps](representation-gaps.md)); you forwarded them on `2026-09-25`, and FSMGen
answered on `2026-09-26`. It accepted all six feature requests, each owned by a leaf of its
`SEMANTIC-INTROSPECTION-MCP-FRONTIER` tree (`.33.3`-`.33.8`), with every optional item as a separate child. It
answered the question too: silently accepting an unknown port option is a defect, which it will fix (`.33.2`).
Each closes when a published FSMGen revision containing its change is pinned and its bundle re-run.
`sf-isf-storage-register-access` (`FSMGEN-REQUESTS.5`) came from the register work on `2026-09-26`; you forwarded
it the same day, and FSMGen is reproducing it, with scalar storage tracked apart from the optional bank extension.

FSMGen also found a flaw in the optional-signal packet: its positive control never drives `AWMECID`, and an
undriven output is dropped from the generated module (`.33.9`), so "byte-identical to the control" could not
show a port is present. That criterion now asks for the port to appear in the module's port list. The same
class of loss reaches SpecForge's own output on the pinned FSMGen `54ac1c22`: an input that no transaction reads
is dropped from the generated module, and 123 of the 126 inputs declared by the five emitted `.isf` files are
missing from their HDL, although each file passes the strict check. FSMGen's development line keeps them all
(`5be6a3c08`, 0 of 126 missing; the input repair is `a7bf398ac`). It is not published yet; the pin refresh
`FSMGEN-REFRESH-INTEGRATE-10` brings it in and must show every declared port in the generated module.

| Bundle | Kind | Status | What it asks |
| --- | --- | --- | --- |
| [`sf-isf-derived-width-parameter`](../../../upstream/fsmgen/sf-isf-derived-width-parameter/README.md) | feature request | acknowledged | Let an `.isf` port width be derived from a parameter (`DATA_WIDTH/8`, `ceil(ADDR_WIDTH/8)`, `clog2(N)`) and stay generic when the block is instantiated. FSMGen already evaluates such expressions one level down, in `.fsm`. |
| [`sf-isf-undeclared-rule-operand`](../../../upstream/fsmgen/sf-isf-undeclared-rule-operand/README.md) | bug report | acknowledged; fixed, not yet published | Strict mode accepts a rule that reads a name nobody declared, and the generated module gains an input port for it. |
| [`sf-tool-issue-bundle-foreign-git-head`](../../../upstream/fsmgen/sf-tool-issue-bundle-foreign-git-head/README.md) | bug report | acknowledged | FSMGen's own bundle tool records the wrong project's revision when run from an exported copy of FSMGen. |
| [`sf-isf-optional-signal`](../../../upstream/fsmgen/sf-isf-optional-signal/README.md) | feature request | acknowledged | Let an interface port be absent: a zero width means absent, `(present P)` makes a port depend on a parameter, and `(absent-value V)` says what an absent input reads. AXI alone has 14 width parameters defaulting to 0, and the AMBA specifications state 18 tie-offs. |
| [`sf-isf-parameter-value-set`](../../../upstream/fsmgen/sf-isf-parameter-value-set/README.md) | feature request | acknowledged | Let a parameter declare the values it may take (`(values 8 16 … 1024)`, `(range 1 64)`) and a recommended range, checked on every default and override. 36 AMBA parameters carry one. |
| [`sf-isf-generic-port-widths`](../../../upstream/fsmgen/sf-isf-generic-port-widths/README.md) | feature request | acknowledged | Keep a parameter-sized port generic in the generated HDL. Today the module declares `parameter DATA_WIDTH = 32` but sizes the port `[31:0]`, so overriding the parameter when instantiating the block silently changes nothing. 62 SpecForge ports are sized by a parameter. |
| [`sf-isf-actor-level-assertion`](../../../upstream/fsmgen/sf-isf-actor-level-assertion/README.md) | feature request | acknowledged | Let an always-on `assert`/`assume`/`cover` be written at actor level. Today it can only sit inside a transaction (where it is still always-on), so an actor with no transaction cannot state an invariant at all. |
| [`sf-isf-known-value-predicate`](../../../upstream/fsmgen/sf-isf-known-value-predicate/README.md) | feature request | acknowledged | Add `(known SIG)` (SystemVerilog `!$isunknown`) to the property language, so *"PADDR must be valid when PSEL is asserted"* can be checked. 28 AMBA statements say *must be valid*. |
| [`sf-isf-transaction-phase-groups`](../../../upstream/fsmgen/sf-isf-transaction-phase-groups/README.md) | feature request | acknowledged | Let a transaction say which signals each protocol phase involves, as checked metadata with no implied behaviour: the shape FSMGen agreed on in June, now needed by 18 SpecForge transactions. |
| [`sf-isf-strict-unknown-port-clause`](../../../upstream/fsmgen/sf-isf-strict-unknown-port-clause/README.md) | question | acknowledged: a defect FSMGen will fix | Should strict mode refuse an interface-port clause it does not recognise? `(bogus-clause xyz)` and `(present P)` on a port both pass the strict check and change nothing in the generated module, and FSMGen's book does not say whether that is intended. |
| [`sf-isf-storage-register-access`](../../../upstream/fsmgen/sf-isf-storage-register-access/README.md) | feature request | acknowledged | Let a storage register state its own access (`(var STS (width 32) (access ro))`), as a field already can. Today only a field may carry `(access …)`, so a register summary's `RO`/`RW`/`WO` stops at SpecForge's IR: 101 registers across TMC and ADIv6, 61 of them with no field at all. |

Until the first one ships, every derived-width signal is emitted at `(width 1)` and listed, with its
expression, in the adapter residual `isf_signal_width_unlowered` (see
[How wide is each signal?](../pipeline/isf-adapter.md#how-wide-is-each-signal)).

Authoritative tracking: `docs/tasks/FSMGEN-REQUESTS.md` (the first three) and
`docs/tasks/EXTRACTION-REPRESENTATION-GAPS.md` (those from the gap ledger).
