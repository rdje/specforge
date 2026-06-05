//! Centralized actor→signal relation verb vocabulary — the Knowledge-Graph edge labels.
//!
//! The extraction path recovers triples `actor —verb→ signal`; the collection of
//! those edges is the Knowledge Graph. These lists are the **edge labels**, typed
//! by relation kind:
//! - **Drives** — the actor *sources / changes / controls* the signal,
//! - **Reads** — the actor *observes / samples* the signal,
//!
//! each in a **passive** (`{signal} is {verb} by {actor}`) and an **active**
//! (`{actor} {verb} {signal}`) voice.
//!
//! The behavioral verbs are **corpus-mined** (`docs/tasks/VERB-COVERAGE-CORPUS.md`)
//! across dozens of vendors' specs — *a verb missing here is an edge the extractor
//! cannot draw.* Verbs are **grammar, not names** (ADR 0006): these lists may grow
//! freely without ever tying the tool to one spec. This is the single place to add
//! a relation verb.

/// Passive drive: `{signal} is {verb} by/from {actor}` → `Drives`.
pub(crate) const PASSIVE_DRIVES_VERBS: &[&str] = &[
    "driven",
    "asserted",
    "provided",
    "returned",
    "sent",
    "sourced",
    "generated",
    "issued",
    "set",
    "produced",
    "supplied",
    "output",
    "outputted",
    "activated",
    "presented",
    "placed",
    "applied",
    // Corpus-mined passive drive forms (VERB-COVERAGE-CORPUS).
    "transmitted",
    "forwarded",
    "written",
    "cleared",
    "toggled",
    "released",
    "loaded",
    "stored",
    "enabled",
    "disabled",
    "requested",
    "acknowledged",
    "granted",
    "masked",
    "gated",
    "pulled",
    "initiated",
    "switched",
    "invalidated",
    "controlled",
];

/// Active drive: `{actor} {verb} {signal}` → `Drives`.
pub(crate) const ACTIVE_DRIVES_VERBS: &[&str] = &[
    "drives",
    "asserts",
    "provides",
    "returns",
    "sources",
    "generates",
    "issues",
    "sets",
    "produces",
    "supplies",
    "outputs",
    "sends",
    "activates",
    "presents",
    "applies",
    "places",
    // Corpus-mined: an actor that sources/changes a signal.
    "transmits",
    "forwards",
    "responds",
    "writes",
    "clears",
    "resets",
    "toggles",
    "releases",
    "loads",
    "stores",
    "enables",
    "disables",
    "requests",
    "acknowledges",
    "grants",
    "negates",
    "controls",
    "determines",
    "masks",
    "gates",
    "pulls",
    "initiates",
    "switches",
    "invalidates",
    "drive",
];

/// Passive read: `{signal} is {verb} by {actor}` → `Reads`.
pub(crate) const PASSIVE_READS_VERBS: &[&str] = &[
    "read",
    "sampled",
    "monitored",
    "accepted",
    "received",
    "captured",
    "observed",
    "detected",
    "checked",
    "latched",
    // Corpus-mined passive read forms.
    "polled",
    "accessed",
];

/// Active read: `{actor} {verb} {signal}` → `Reads`.
pub(crate) const ACTIVE_READS_VERBS: &[&str] = &[
    "reads", "samples", "monitors", "accepts", "receives", "captures", "observes", "detects",
    "checks", "latches", "read", "sample", "monitor", "accept", "receive",
    // Corpus-mined: an actor that observes a signal.
    "polls", "poll", "accesses", "access",
];

// --- Logic-level vocabulary (universal binary-logic "how", deliberately retained) ---
//
// Unlike signal/value names (derived from the document; ADR 0006), a *logic level* is not
// any one spec's vocabulary: every digital specification assumes "high" = logic 1 and
// "low" = logic 0. These are the common ways specs and HDL spell those two levels. They are
// the irreducible **"how"** — `specforge` cannot lower `"X must be high"` to the correct
// assertion without knowing high = 1, and a spec never *defines* that (it is assumed). So
// this vocabulary is retained on purpose (ADR 0006's "universal how" boundary) and lives
// here as the single authority — the only place logic-level spellings are recognized.

/// Spellings that mean logic-high (1).
pub(crate) const LOGIC_HIGH_VALUES: &[&str] = &["1", "1'b1", "high", "hi", "true"];

/// Spellings that mean logic-low (0).
pub(crate) const LOGIC_LOW_VALUES: &[&str] = &["0", "1'b0", "low", "lo", "false"];
