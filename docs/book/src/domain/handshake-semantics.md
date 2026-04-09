# Handshake And Semantic Roles

Many chip protocols express transfer progress through handshake-like signals.

The most familiar pattern is ready/valid:

- one side says information is valid
- the other side says it can accept or is ready
- the transfer completes when the relevant handshake conditions hold together

`specforge` does not want to recover that meaning only from signal spelling.
Names like `AWVALID` and `AWREADY` are useful clues, but spelling alone is too weak for canonical intent.

## Current semantic roles

The current canonical semantic-role surface includes:

- `handshake_valid_like`
- `handshake_ready_like`

These names are intentionally role-like, not protocol-family-specific.
Different specs may call the same concept request, valid, present, accept, ready, acknowledge, or receive.

The goal is to recover the semantic role, not just match one naming convention.

## Where role evidence comes from

Role evidence can come from several source kinds:

- signal-description tables
- direct prose
- alias-grounded prose
- visual captions
- VLM timing-diagram annotations
- prior-guided local phrase matches

Those sources do not all have the same strength.
For example, direct table/prose evidence is usually stronger than alias-only evidence, and cross-modality support is stronger than repeated support from one modality.

## Observations before winners

`specforge` first preserves observations.

A semantic observation records:

- the tags found
- the source kind
- the source text
- supporting statement ids
- supporting table ids
- supporting visual evidence ids
- automation confidence

This is the first safety boundary.
The project should not jump from a phrase directly to a final canonical role if the evidence is still weak or ambiguous.

## Candidates

Observations are grouped into semantic candidates.

A candidate represents one possible role for a signal, with fields such as:

- role
- grounding strength
- supporting source kinds
- supporting observation count
- automation confidence
- evidence weight
- prior reliability adjustment
- arbitration weight
- alias-dependent flag

If a signal has one strong role, it may have one candidate.
If evidence disagrees, it may have multiple candidates.

That is expected.
Multiple candidates are not a failure by themselves; they are how uncertainty stays visible.

## Grounding strength

The canonical grounding strengths are:

- `single_source`
- `multi_source`
- `cross_modality`

They mean:

- `single_source`: one source supports the role
- `multi_source`: more than one source supports the role, but not across distinct modalities
- `cross_modality`: distinct modalities support the same role, such as table plus visual evidence

This distinction matters because repeated evidence from one source type should not be overclaimed as strongly as table-plus-visual agreement.

## Arbitration

Semantic arbitration is the typed record of how role candidates compare.

It answers:

- how many candidates exist?
- which role is currently leading?
- which role is the runner-up?
- how much evidence weight supports each side?
- did prior reliability adjust the comparison?
- what is the margin?
- is the decision actually decisive?

The current decision bases are:

- `single_candidate`
- `prior_guided_margin`
- `contested`

This is not "pick a winner no matter what".
It is "make the competition inspectable, and only resolve when it is safe".

## Consensus

Consensus is the canonical summary for a role that survived arbitration.

A consensus record says:

- which role was resolved
- how strongly it was grounded
- which source kinds supported it
- how many observations supported it
- whether it was prior-guided
- whether it was alias-dependent

Consensus is important because downstream logic should prefer observation-backed meaning over fallback-only role guesses.

## Alias-dependent meaning

Alias-grounded evidence is useful, but weaker than direct signal evidence.

For example, if a document first maps "request phase" to `XREQ`, later prose about "request phase" can help recover a role for `XREQ`.

That should remain visible as alias-dependent grounding.
It can be good enough to preserve an accepted semantic path, but it should not look identical to direct evidence that names the signal explicitly.

## Prior-guided meaning

Cross-document prior memory can help with semantic roles too.

For example, if earlier validated specs taught that a phrase like "can receive the transfer" is ready-like, a later document can use that prior to interpret the same local phrase.

The safety rule is the same as elsewhere:

- the phrase must still appear in the current document
- the prior only guides interpretation
- the prior does not invent a signal, role, or fact by itself

## Handshake completion

Typed temporal rules can carry a `HandshakeComplete` predicate.

That predicate should be derived from grounded role meaning when possible.
For example, a rule guarded by a valid-like signal and a ready-like signal can express transfer completion without relying only on literal `VALID` / `READY` spelling.

This is why role truthfulness matters.
If a role is contested, handshake completion should not be promoted just because the signal name looks convenient.

## Blocked name fallback

Literal name fallback is intentionally blocked when preserved role evidence is contested or only provisional.

For example, a signal named `XVALID` might look valid-like by spelling.
But if the document evidence also supports a competing ready-like interpretation, the pipeline should preserve the contest instead of silently accepting the name.

That can surface as a blocked handshake-name fallback residual.

This is a feature, not a failure.
It prevents a clean-looking but unsafe semantic shortcut.

## What users should inspect

When debugging handshake recovery, inspect:

- semantic observations
- semantic candidates
- semantic arbitration
- semantic consensus
- alias-dependent flags
- prior-guided flags
- `HandshakeComplete` temporal predicates
- validation findings around semantic-role arbitration and blocked fallback

The goal is not to make every signal resolve immediately.
The goal is to make the evidence trail strong enough that accepted handshake semantics are trustworthy.

