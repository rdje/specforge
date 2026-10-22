# What SpecForge needs FSMGen to support, from PDF to `.isf`

Owner: `EXTRACTION-REPRESENTATION-GAPS.36`. Classified on `2026-09-26` against FSMGen's published book at
SpecForge's pin `54ac1c22` (chiefly §13k *ISF Shipped Feature Matrix* and its *Explicit Non-Claims*, and §14
*Feature Backlog*), the request packets under [`docs/upstream/fsmgen/`](../upstream/fsmgen/PROCESS.md) and
[`docs/FSMGEN_FEEDBACK.md`](../FSMGEN_FEEDBACK.md). Re-check every rung on each FSMGen pin refresh.

The rungs follow the path a specification takes: first the actor and its boundary, then the obligations on its
signals, then time, transactions, storage, and finally systems of several actors. A rung is only as useful as the
SpecForge side that feeds it, so each names both.

| Status | Meaning |
| --- | --- |
| **supported** | Shipped at the pin; SpecForge can emit it today. |
| **fixed upstream** | Fixed in FSMGen's development line, not yet published; SpecForge waits to refresh its pin. |
| **requested** | A signed-off packet has been sent (`forwarded`) or answered (`acknowledged`); FSMGen holds it. |
| **anticipated** | SpecForge will need it, and FSMGen's book says it is not shipped, but no request exists yet: `PROCESS.md` files one only after a source-grounded IntentIR value reproduces the gap (ADR 0033). |
| **question** | FSMGen's contract is silent; an answer is awaited. |

## A. The actor and its boundary

| # | Capability | Why SpecForge needs it | FSMGen | Request | SpecForge owner |
| --- | --- | --- | --- | --- | --- |
| F1 | One actor, one clock, a reset with its kind and polarity | every emitted `.isf` (AXI, AHB, APB-E, Wishbone) | **supported** (13k *Actor envelope*, *Single-clock timing*) | — | `EXTRACTION-REPRESENTATION-GAPS.7` |
| F2 | A reset keeps the polarity it states, whatever its name | the pin keys polarity on the name (`…n` → low) | **fixed upstream** (`f16130a4f`, unpublished) | — | `EXTRACTION-REPRESENTATION-GAPS.21` |
| F3 | Ports with a direction and a fixed or parameter width | every port | **supported** (13k *Interface ports*) | — | `KG-ISF-COMPLETENESS.2a.vii` |
| F4 | A width derived from parameters (`DATA_WIDTH/8`, `clog2`) | AXI `WSTRB`, AHB `HWSTRB` | **requested** (acknowledged) | [`sf-isf-derived-width-parameter`](../upstream/fsmgen/sf-isf-derived-width-parameter/README.md) | `FSMGEN-REQUESTS.1` |
| F5 | A generated block stays generic in its widths | one `.isf` for every width the document allows | **requested** (forwarded) | [`sf-isf-generic-port-widths`](../upstream/fsmgen/sf-isf-generic-port-widths/README.md) | `EXTRACTION-REPRESENTATION-GAPS.6` |
| F6 | A parameter's legal values and recommended range | tables of legal widths and property values | **requested** (forwarded) | [`sf-isf-parameter-value-set`](../upstream/fsmgen/sf-isf-parameter-value-set/README.md) | `EXTRACTION-REPRESENTATION-GAPS.5` |
| F7 | An optional port: absent by default, present by a parameter, tied off when absent | *If present, …* (14 of AXI's 25 listed obligations), tie-offs, and the value a stated-absent signal takes on its presence record (`.3.d.ii.a`) | **requested** (forwarded) | [`sf-isf-optional-signal`](../upstream/fsmgen/sf-isf-optional-signal/README.md) | `EXTRACTION-REPRESENTATION-GAPS.4` |
| F8 | Strict mode refuses an interface clause it does not know | a strict pass must prove each clause was understood | **fixed upstream** (`c51e22712`, unpublished; answered as a defect) | [`sf-isf-strict-unknown-port-clause`](../upstream/fsmgen/sf-isf-strict-unknown-port-clause/README.md) | `FSMGEN-REFRESH-INTEGRATE-10.1` |
| F9 | One select per target (`PSELx` per Completer) | per-target families | **supported** through composition (13a, 14h, 14i) | — | `EXTRACTION-REPRESENTATION-GAPS.14` |
| F29 | A named sub-field layout on a port (`HPROT[1]` *Privileged*; `AxMPAM[9:1]` *PARTID*), over a parameter-sized width or chosen by a parameter | AHB *3-6*, APB `PPROT`, AXI *A12.3*-*A12.6* | **anticipated**: port options are a closed set; `(fields …)` is storage-only and a packed record has a fixed width (13a, 13j, ISF spec 02 §5.1.1) | — (bundle drafted, `2026-10-22`) | `EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii` |

## B. Obligations on levels

| # | Capability | Why SpecForge needs it | FSMGen | Request | SpecForge owner |
| --- | --- | --- | --- | --- | --- |
| F10 | Guarded rules that drive a level, with conflicts and priorities | every lowered obligation | **supported** (13k *Rules and trigger fan-in*, *Rule conflicts and priorities*); the guard addendum is **fixed upstream** (`c6b8bc783`) | — | `EXTRACTION-REPRESENTATION-GAPS.10` |
| F11 | An undeclared rule operand is refused, not added as an input | a wrong artifact must fail closed | **requested** (acknowledged bug) | [`sf-isf-undeclared-rule-operand`](../upstream/fsmgen/sf-isf-undeclared-rule-operand/README.md) | `FSMGEN-REQUESTS.2` |
| F12 | An always-on `assert`/`assume`/`cover` outside a transaction | stability (*HSIZE must not change*), same-cycle exclusion, implications between outputs (*DVM Complete* rows) | only inside a transaction (13k *Immediate verification checks*); actor-level **requested** (forwarded) | [`sf-isf-actor-level-assertion`](../upstream/fsmgen/sf-isf-actor-level-assertion/README.md) | `EXTRACTION-REPRESENTATION-GAPS.15`, `EXTRACTION-REPRESENTATION-GAPS.11` |
| F13 | A known-value predicate (`known SIG`) | *X must be valid when Y* (28 statements) | **requested** (forwarded) | [`sf-isf-known-value-predicate`](../upstream/fsmgen/sf-isf-known-value-predicate/README.md) | `EXTRACTION-REPRESENTATION-GAPS.16` |

## C. Time

| # | Capability | Why SpecForge needs it | FSMGen | Request | SpecForge owner |
| --- | --- | --- | --- | --- | --- |
| F14 | *Eventually within N cycles*, inside a transaction | bounded response obligations | **supported** (13k *Temporal contracts*); the flat form was resolved | [`sf-isf-contract-eventually-flat`](../upstream/fsmgen/sf-isf-contract-eventually-flat/RESOLUTION.md) | — |
| F15 | A window counted from an event, a minimum duration, an order between two events, a global implication | *for at least one complete clock cycle*, *no later than the edge that qualifies STB_O*, unguarded windows | **anticipated**: not shipped (13k *Explicit Non-Claims*: windows, global implication, dynamic bounds, several outstanding obligations) | — | `EXTRACTION-REPRESENTATION-GAPS.37` |

## D. Transactions and handshakes

| # | Capability | Why SpecForge needs it | FSMGen | Request | SpecForge owner |
| --- | --- | --- | --- | --- | --- |
| F16 | Transactions: entry, drives, waits, control flow | a transaction the document spells out step by step | **supported** (13k *Transaction entry* … *Transaction control flow*) | — | `KG-ISF-TRANSACTIONS` |
| F17 | A ready/valid stage | handshake completion | **supported** bounded (13k *Transaction stages*); nested stages, several endpoints and skid buffers are not shipped | [`sf-isf-stage-ready-valid`](../upstream/fsmgen/sf-isf-stage-ready-valid/RESOLUTION.md) (resolved) | delivered by `R16-CONTRACT-IR` |
| F18 | Checked phase-group metadata on a transaction | AXI channel membership of each transaction | **requested** (forwarded) | [`sf-isf-transaction-phase-groups`](../upstream/fsmgen/sf-isf-transaction-phase-groups/README.md) | `EXTRACTION-REPRESENTATION-GAPS.20` |
| F19 | A transaction identified by opcode values (`AWSNOOP` with `AWDOMAIN`) as a guard | *for a WriteZero transaction* | **supported** (guards, `switch`) | — | `EXTRACTION-REPRESENTATION-GAPS.10.g.iii` |
| F20 | A protocol state machine | named states and transitions | **supported** (transactions and control flow; an explicit FSM form was withdrawn as already expressible) | [`sf-isf-explicit-fsm-declaration`](../upstream/fsmgen/sf-isf-explicit-fsm-declaration/README.md) (withdrawn) | `EXTRACTION-REPRESENTATION-GAPS.12` |

## E. Storage, registers and structures

| # | Capability | Why SpecForge needs it | FSMGen | Request | SpecForge owner |
| --- | --- | --- | --- | --- | --- |
| F21 | Registers with bit-fields (bits, access, reset, enum) | register maps (6,570 fields lower today) | **supported** (13k *Actor-owned scalar storage*) | — | `EXTRACTION-REPRESENTATION-GAPS.13` |
| F22 | Memory banks (depth, indexed load and store) | register files, queues | **supported** bounded (13k *Actor-owned banks*) | — | `EXTRACTION-REPRESENTATION-GAPS.3` |
| F23 | Structured records as ports and storage: packet, flit, descriptor, queue and page-table layouts | 1,527 message and structure fields stop before IntentIR | **anticipated**: aggregate storage aliases ship, but *aggregate interface ports, transaction-local aggregate ports, aggregate banks* remain backlog (13k *Explicit Non-Claims*) | — | `DOC-INTENT-TAXONOMY.4b` |
| F24 | Enums and types | encodings | **supported** bounded (13k *Type aliases and package imports*, *Enum member values*) | — | `KG-ISF-COMPLETENESS.5` |

## F. Systems of actors

| # | Capability | Why SpecForge needs it | FSMGen | Request | SpecForge owner |
| --- | --- | --- | --- | --- | --- |
| F25 | Several actors: static instances, groups, reusable libraries | platform topology, per-target fan-out | **supported** bounded (13k *Static actor-network metadata*, *Reusable ISF libraries*); route fan-in/fan-out and payload protocols are not shipped | — | `DOC-INTENT-TAXONOMY.4c.ii` |
| F26 | Several clock domains with data crossing between them | platform clock trees | **supported** bounded (13k *Multi-clock domains*, *Acknowledged event CDC*); **anticipated** for multi-bit payload and FIFO crossings (not shipped) | — | `EXTRACTION-REPRESENTATION-GAPS.3` |
| F27 | A reset-less actor, or one whose clock the document does not name | I²C, I²S, ADIv6's SWD | **supported** (a clock-only domain exists; no request, `.7.e`) | — | `EXTRACTION-REPRESENTATION-GAPS.7.e.i` |

## G. Tooling

| # | Capability | Why SpecForge needs it | FSMGen | Request | SpecForge owner |
| --- | --- | --- | --- | --- | --- |
| F28 | The issue-bundle tool records FSMGen's own revision | every packet must name the FSMGen it reproduces on | **requested** (acknowledged bug) | [`sf-tool-issue-bundle-foreign-git-head`](../upstream/fsmgen/sf-tool-issue-bundle-foreign-git-head/README.md) | `FSMGEN-REQUESTS.3` |

## Where the ladder stands

On `2026-09-26`, of 28 rungs: 15 supported (F26 only in part), 1 fixed upstream and awaiting publication (F2; F10's
guard addendum likewise), 9 requested (3 acknowledged, 6 forwarded), 1 question and 2 anticipated (F15, temporal
properties beyond *eventually*; F23, structured records), with F26's multi-bit clock crossings anticipated too. The
requested rungs gate the most today: F7 alone holds 14 of AXI's 25 listed obligations. The anticipated rungs wait on
SpecForge first: each needs a typed, source-grounded IntentIR value before a request can reproduce it.

Since `2026-10-22` F8 is fixed upstream too: FSMGen answered the question as a defect, and `c51e22712` refuses an
unknown port clause. That leaves no open question. F29, a named sub-field layout on a port, joins as
*anticipated*: ADR 0033 §4 files its request only once the layouts are in IntentIR and an executable consumer needs
them (`EXTRACTION-REPRESENTATION-GAPS.3.c.iv.b.ii`).
