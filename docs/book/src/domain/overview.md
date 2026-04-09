# Domain Model

This section explains hardware and protocol concepts that deserve their own public documentation.

The pipeline chapters explain where facts move through the IR stages.
The domain-model chapters explain how `specforge` thinks about the meaning of those facts.

Start here when you want to understand concepts such as:

- clock and reset infrastructure
- protocol signal roles
- actor-relative connectivity
- timing and handshake semantics
- polarity-aware interpretation

The section begins with clock and reset infrastructure because that boundary is especially important.
Clocks and resets can appear syntactically like ordinary signals, but they should not be modeled as ordinary payload or handshake edges.

The next chapter covers handshake and semantic-role recovery, because ready/valid-like transfer progress is one of the most important protocol motifs `specforge` needs to recover without over-trusting signal names.

Actor connectivity is the next boundary after that: `specforge` should prefer graph facts like `(Requester, Drives, PSEL)` and `(Completer, Reads, PSEL)` over flat direction hints whenever the document supports them.
