# SemanticIR

`SemanticIR` is where evidence is assembled into typed protocol meaning.

## What belongs in `SemanticIR`

- actors
- interfaces
- actor-relative ports
- signal connectivity
- phases
- invariants
- gates
- timing constraints
- temporal rules
- semantic candidates
- semantic arbitration
- residual decisions

## Why this stage exists

`EvidenceIR` can tell you what the document said and where it came from.
`SemanticIR` is where the tool starts asking:

- who drives this signal?
- who reads it?
- what role does it play?
- what timing behavior is being asserted?
- is the meaning decisive or contested?

## Important semantic principles

### Graph-first direction

Direction should come from structural actor-signal relations when possible, not from flat heuristics alone.

### Meaning before spelling

Handshake recovery should prefer grounded role evidence over raw `*VALID*` / `*READY*` name shape.

### Explicit conflict surface

Competing semantic candidates should stay visible.
Unsafe forced winners are worse than honest contestation.

### Infrastructure is special

Clocks and resets are treated as infrastructure semantics, not ordinary protocol edges.

## Residual decisions

Residual packets exist because the project would rather preserve unresolved ambiguity than fabricate a clean but wrong canonical answer.

