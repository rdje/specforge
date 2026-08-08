# Documentation Scope And Continuity

The `mdBook` is the public-facing documentation product for `specforge`.
It is what the outside world should read to understand:

- what `specforge` does
- how the user-facing commands and workflows operate
- how the IR pipeline works
- why the tool is structured the way it is
- what the current truthfulness boundaries and limitations are

## Book contract

The book is not just onboarding material.
It should grow until every meaningful user-facing aspect of the project has a clear home here, ideally as its own section or chapter.

That means the book should cover:

- operational workflows
- runtime setup
- stage semantics
- validation surfaces
- cross-document learning behavior
- rationale for major architectural decisions that affect how users should understand the tool

## Separate continuity plane

The repo root markdown docs serve a different job.
They are live engineering and continuity infrastructure for:

- crash recovery
- session handoff
- roadmap steering
- validation projection
- implementation-state tracking

They are not the primary public product docs, even when they contain valuable detail.

Four long-standing root filenames are narrower still: `USER_GUIDE.md`,
`EXTRACTION_ARCHITECTURE.md`, `KNOWLEDGE_GRAPH_ARCHITECTURE.md`, and `INTENTIR_SPEC.md` are bounded
compatibility pointers into this book. They retain stable external routes, but they do not mirror
commands, current implementation state, validation, bugs, or plans. The unique durable extraction
and `IntentIR` rules they once carried now live in the directly indexed
[Extraction Architecture Contract](extraction-architecture.md) and
[IntentIR Product Contract](intentir-contract.md).

## Practical rule

When a change affects what users should understand about `specforge`, it belongs in the book.

When a change mainly preserves live state, sequencing, recovery context, or session continuity, it belongs in the root continuity docs.

When an old root user/architecture filename must remain resolvable, update its direct route only;
put the actual public contract in the book.

## Closed task trees — how each was implemented and verified

### `AUDIT-DOC-RECONCILE` — the doctrine that keeps the docs honest

When you read a SpecForge guide, a ROADMAP entry, or a
README sentence, you should be able to trust that what it
says is what the code actually does. That sounds obvious. It
isn't free, and `AUDIT-DOC-RECONCILE` is the tree that paid
for it.

#### The user-facing guarantee

> **Every claim in the book / ROADMAP / README either
> describes what the code does, or it's recorded as wrong
> and gets fixed. The text is never allowed to silently lie
> about the code. When in doubt: the code is the truth, the
> text is the suspect.**

That's the doctrine. It's load-bearing for every later
tree's close-leaf — including every R16 subsection you just
read in this book.

#### Why this isn't free

Doc drift is the gentlest kind of bug: it doesn't break a
test, it doesn't crash a build, and it survives every
refactor that doesn't happen to touch the documented
surface. A typical drift case looks like this:

> *The ROADMAP says: "the adapter computes actor-relative
> direction from the flat `direction_hint` field."*
>
> *Six months earlier, R15-GRAPH-DIRECTION-MIGRATION moved
> direction-computation into the canonical KG. The adapter
> stopped consulting `direction_hint` for that purpose.*
>
> *The ROADMAP sentence stayed. Anyone reading the ROADMAP
> today gets a clean, plausible explanation of how the
> adapter works — except the explanation is wrong, by a
> migration nobody remembered to also write about.*

Multiply that by every active subsystem, every long-running
program, every README that promised an architecture before
the code arrived, and you get a documentation lane that
quietly drifts ahead of (or, more often, behind) the code
that's actually shipping.

#### What this tree fixed (concretely)

Two specific drifts were identified by a doc-audit pass and
reconciled in this tree:

1. **`ROADMAP` R15** still described deleted `.fsm` paths and
   adapter behavior that did not match that revision. The text
   was rewritten then. A later delivered slice superseded its
   direction detail: today's `.isf` adapter selects an initiator
   from `IntentIR.actor_ports`, lowers unambiguous graph
   directions, and falls back safely only for unresolved signals.
   The current contract lives in [Actor Connectivity And Graph
   Direction](../domain/actor-connectivity.md#adapter-use), not in
   this historical audit narrative.
2. **Stale references to multi-target HDL adapters**
   (SystemVerilog / Verilog / VHDL) lingered in the book and
   ROADMAP after `ISF-ONLY-CONSOLIDATION` made `.isf` the
   sole adapter target. The references were removed.

Both fixes followed the doctrine: the code was the truth;
the text was rewritten to describe it.

#### How the doctrine is enforced now

The doctrine became **structural** through the
`BOOK-METHOD-DOC` close-rule (see `Required Commit
Workflow` in `COMMIT.md` and `Completion Rules` in
`docs/TASK_TREE.md`):

- Every tree's closing leaf MUST add or refresh that tree's
  method-doc subsection in the topically-correct mdBook
  chapter — describing what the code does now.
- A close leaf whose book section is missing or stale is, by
  definition, **incomplete**.
- The CI step (`scripts/run_docs_ci.sh`) builds the book on
  every leaf, so the book is exercised continuously.

In other words: this doctrine + that close-rule mean drift
becomes a per-leaf bug instead of a six-month-later audit
finding. The book section you're reading right now exists
because the close-leaf that delivered the R7-VALIDATION tree
was required to refresh it.

#### What this buys you, as a SpecForge user

- **You can read this book and trust it.** If a section
  claims SpecForge does X, the code does X. If the section
  is wrong, that's a bug — file it, and the next close-leaf
  to touch the affected area will reconcile it.
- **You don't have to read the source to be sure.** The
  task-tree files remain the machine-tracked authority for
  what was delivered; the book is the topically-placed
  explanation that mirrors them. If you read either and
  reach the same conclusion, the doctrine is doing its job.
- **You can spot drift cheaply.** If you find a sentence
  here that doesn't match the code, you've found a real bug
  (and a small one — drifts are usually one paragraph in
  one chapter, not a tangled refactor).

*Authoritative tracking:*
`docs/tasks/AUDIT-DOC-RECONCILE.md`. The doctrine is
recorded as a standing rule in `docs/TASK_TREE.md`'s
Completion Rules and `COMMIT.md`'s Required Commit
Workflow.

### `AUDIT-PROVIDER-FRAMING-RECONCILE` — the doctrine, caught in the act

`AUDIT-DOC-RECONCILE` is the doctrine; this tree is what it
looks like when the doctrine actually catches something.

#### What drifted

Several live docs — `ROADMAP.md`'s "Immediate next
milestone," the `docs/TASK_TREE.md` R16 row, and
`RUST_CODEBASE_ANALYSIS.md` — described the R16
design-intent-capture *crux* as *"blocked on upstream
capability: it needs a prose LLM/VLM provider."* Read
literally, that implied SpecForge had **no** LLM/VLM
provider yet.

It does. Ollama + Qwen2.5VL is the **production default**
for `converge` (`--vlm-provider` / `--nlp-provider` default
to `ollama`; `DEFAULT_LOCAL_MODEL = "qwen2.5vl:7b"`),
powering the `enrich` VLM diagram-reading pass and the
`nlp_enrich` NLP-Level-3 prose relation extraction, and it
has been validated end-to-end on AMBA specs. The docs had
conflated *"the provider doesn't exist"* with *"the
provider's output isn't yet wired into the newer R16 CVE
producer `parse_constrained_contract`."* Only the second is
true.

#### What this tree fixed

Following the doctrine (the code is the truth, the text is
the suspect), the live docs were rewritten to describe what
the code does:

- the prose LLM/VLM provider is production-default;
- the remaining R16 CVE work is **wiring** that existing
  provider into `parse_constrained_contract` (an edit), plus
  adding a `signal_relation` extraction type (R14) — not
  building a provider;
- the one genuinely upstream-absent piece is a typed
  PDF→`FigureRegion` raster/vector decoder (the VLM already
  reads diagram images — just to text, not the typed
  geometric path);
- stale `.fsm` example blocks in `INTENTIR_SPEC.md` were
  corrected to `.isf`, and `RUST_CODEBASE_ANALYSIS.md` gained
  a dated currency note (provider + full 17-command surface).

#### What this buys you

The "what's left to build" picture is now honest in both
directions: it doesn't **under**-claim (the extraction
pipeline is live, default, and validated) and it doesn't
**over**-claim (the typed R16 honesty surfaces are still
dormant until wired). If you're choosing what to work on
next, the docs now point you at *wiring* and a *figure
decoder* — not at re-inventing a provider that already
ships.

*Authoritative tracking:*
`docs/tasks/AUDIT-PROVIDER-FRAMING-RECONCILE.md`.
