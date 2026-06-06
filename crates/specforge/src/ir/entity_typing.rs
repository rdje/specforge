//! `EXTRACTION-QUALITY-GAUGE.1` — **entity typing**: the "human-SpecForge in Rust" structure.
//!
//! CHI exposed that SpecForge cannot discriminate a *signal* from an *actor* / *transaction* /
//! *feature* / *boilerplate* — and that pattern-matching (Rust) cannot, because the judgment needs
//! world knowledge. The answer is not to compile the LLM's brain into Rust; it is to structure Rust
//! as **the expert's method + the grounding discipline**, and call the LLM at the judgment point:
//!
//! 1. **Method (Rust):** the questions an expert asks to type a token — *where is it declared? what
//!    is its grammatical role? how is it cross-referenced?* These are universal across specs, so they
//!    are what gets frozen into code (the *procedure*, never the *names* — ADR 0006).
//! 2. **Evidence (Rust):** [`gather_entity_evidence`] answers those questions from *this* document
//!    deterministically.
//! 3. **Judgment (LLM):** given that evidence, decide the type using world knowledge ([`classify_entity`]'s
//!    injected `propose`).
//! 4. **Grounding (Rust):** [`classify_entity`] *overrides* the LLM where the document is
//!    unambiguous, and *defers* where it is silent. The document grounds everything → the same code
//!    works on any chip-spec PDF.

use crate::cli::VlmProviderArg;
use crate::commands::llm_text::{api_url, call_text_provider};
use crate::ir::evidence::EvidenceIr;

/// The entity classes a chip-spec token can be. A constraint/relation may only take a `Signal`
/// subject; the rest are exactly the things CHI mis-typed *as* signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Signal,
    Actor,
    Transaction,
    Feature,
    State,
    /// A reference to a Table/Figure/Section (`Table B13.25` → `B13`).
    StructuralRef,
    /// Legal / front-matter boilerplate (`LICENSEE`, `AMBA` trademark).
    Boilerplate,
    Value,
    Unknown,
}

impl EntityType {
    pub fn as_str(self) -> &'static str {
        match self {
            EntityType::Signal => "signal",
            EntityType::Actor => "actor",
            EntityType::Transaction => "transaction",
            EntityType::Feature => "feature",
            EntityType::State => "state",
            EntityType::StructuralRef => "structural_ref",
            EntityType::Boilerplate => "boilerplate",
            EntityType::Value => "value",
            EntityType::Unknown => "unknown",
        }
    }
    /// Parse an LLM's answer (a word, possibly with extra prose) — fail-safe → `Unknown`.
    pub fn parse(word: &str) -> EntityType {
        for w in word
            .to_ascii_lowercase()
            .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        {
            let t = match w {
                "signal" => Some(EntityType::Signal),
                "actor" | "component" | "agent" => Some(EntityType::Actor),
                "transaction" => Some(EntityType::Transaction),
                "feature" => Some(EntityType::Feature),
                "state" => Some(EntityType::State),
                "structural_ref" | "structuralref" | "reference" | "ref" | "table" | "figure" => {
                    Some(EntityType::StructuralRef)
                }
                "boilerplate" | "legal" => Some(EntityType::Boilerplate),
                "value" => Some(EntityType::Value),
                _ => None,
            };
            if let Some(t) = t {
                return t;
            }
        }
        EntityType::Unknown
    }
}

/// The document's own typing evidence for one token — gathered by Rust, fed to the LLM, and used to
/// ground its answer. Every field is something a human would *look up in the document*.
#[derive(Debug, Clone, Default)]
pub struct EntityEvidence {
    pub token: String,
    /// Declared in a pin/port/signal table (`table_signal_declaration_provenance`) — authoritative.
    pub declared_in_signal_table: bool,
    /// Semantic tags the document attached to it (`signal_semantic_hints`).
    pub semantic_hint_tags: Vec<String>,
    /// Appears as the ACTOR (subject of a normative verb) in a relation.
    pub appears_as_actor: bool,
    /// Appears as the SIGNAL (object of a normative verb) in a relation.
    pub appears_as_signal: bool,
    /// Appears as a structural reference (`Table <tok>` / `Figure <tok>`) in its context.
    pub structural_ref_context: bool,
    /// A few sentences where the token appears — handed to the LLM as grounding.
    pub context_snippets: Vec<String>,
}

/// Gather the evidence for `token` from the document (deterministic; no LLM). `context_texts` are the
/// sentences the token appears in (e.g. its constraints' `source_text`).
pub fn gather_entity_evidence(
    token: &str,
    ir: &EvidenceIr,
    context_texts: &[String],
) -> EntityEvidence {
    let up = token.trim().to_ascii_uppercase();
    let declared = ir
        .table_signal_declaration_provenance
        .iter()
        .any(|d| d.signal_name.trim().to_ascii_uppercase() == up);
    let semantic_hint_tags: Vec<String> = ir
        .signal_semantic_hints
        .iter()
        .filter(|h| h.signal_name.trim().to_ascii_uppercase() == up)
        .flat_map(|h| h.semantic_tags.iter().map(|t| format!("{t:?}")))
        .collect();
    let appears_as_actor = ir
        .actor_signal_relations
        .iter()
        .any(|r| r.actor_name.trim().to_ascii_uppercase() == up);
    let appears_as_signal = ir
        .actor_signal_relations
        .iter()
        .any(|r| r.signal_name.trim().to_ascii_uppercase() == up);
    let tl = token.trim().to_ascii_lowercase();
    let structural_ref_context = context_texts.iter().any(|t| {
        let l = t.to_ascii_lowercase();
        l.contains(&format!("table {tl}")) || l.contains(&format!("figure {tl}"))
    });
    EntityEvidence {
        token: token.to_string(),
        declared_in_signal_table: declared,
        semantic_hint_tags,
        appears_as_actor,
        appears_as_signal,
        structural_ref_context,
        context_snippets: context_texts.iter().take(3).cloned().collect(),
    }
}

/// Type a token: Rust **grounds** where the document is authoritative, the injected LLM `propose`
/// **judges** where it is silent. This is the bounded-LLM contract — the document overrides the
/// model, never the reverse. `propose` is injected so the policy is unit-testable with no provider.
pub fn classify_entity(
    ev: &EntityEvidence,
    propose: impl Fn(&EntityEvidence) -> EntityType,
) -> EntityType {
    // POSITIVE ground: declared in a signal table → authoritatively a Signal (the doc says so).
    if ev.declared_in_signal_table {
        return EntityType::Signal;
    }
    // NEGATIVE ground: appears only as a structural reference and never as a signal → not a signal.
    if ev.structural_ref_context && !ev.appears_as_signal {
        return EntityType::StructuralRef;
    }
    // The document is silent/ambiguous → defer to the LLM, which sees the same gathered evidence.
    propose(ev)
}

/// The enforcement gate: a constraint/relation subject is valid only if it types as a `Signal`.
pub fn is_valid_signal_subject(t: EntityType) -> bool {
    matches!(t, EntityType::Signal)
}

/// Default text model for entity typing (the local model the NLI gate also uses).
pub const DEFAULT_ENTITY_MODEL: &str = "qwen2.5:14b-instruct";

/// The LLM judgment prompt — the token + the Rust-gathered evidence + the closed type set.
pub fn entity_prompt(ev: &EntityEvidence) -> String {
    let snippets = ev
        .context_snippets
        .iter()
        .map(|s| format!("  - {}", s.chars().take(180).collect::<String>()))
        .collect::<Vec<_>>()
        .join("\n");
    format!(
        "Classify a token from a chip-design specification as EXACTLY ONE of: signal, actor, \
         transaction, feature, state, structural_ref, boilerplate, value.\n\
         signal = a wire/pin/field carrying a value; actor = a component that acts \
         (manager/requester, completer, node); transaction = a named protocol operation; feature = \
         a capability; state = a protocol state; structural_ref = a Table/Figure/Section reference; \
         boilerplate = a legal/front-matter term; value = a literal value.\n\n\
         Token: {}\n\
         Declared in a signal table: {}\n\
         Document semantic tags: {:?}\n\
         Used as the actor (subject) of a requirement: {}\n\
         Used as the signal (object) of a requirement: {}\n\
         Example sentences:\n{}\n\n\
         Answer with ONE word.",
        ev.token,
        ev.declared_in_signal_table,
        ev.semantic_hint_tags,
        ev.appears_as_actor,
        ev.appears_as_signal,
        snippets,
    )
}

/// Production `propose` for [`classify_entity`]: the LLM judges given the gathered evidence.
/// Fail-safe → `Unknown` on any provider error (the caller's Rust grounding still applies).
pub fn propose_entity_type_llm(
    ev: &EntityEvidence,
    provider: VlmProviderArg,
    model: &str,
) -> EntityType {
    match call_text_provider(
        provider,
        model,
        api_url(provider),
        "",
        &ev.token,
        &entity_prompt(ev),
        16,
    ) {
        Ok(resp) => EntityType::parse(&resp),
        Err(_) => EntityType::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(token: &str) -> EntityEvidence {
        EntityEvidence {
            token: token.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn declared_signal_is_grounded_to_signal_without_asking_the_llm() {
        let mut e = ev("PADDR");
        e.declared_in_signal_table = true;
        // The LLM would (wrongly) say boilerplate — the document overrides it.
        let t = classify_entity(&e, |_| EntityType::Boilerplate);
        assert_eq!(t, EntityType::Signal);
    }

    #[test]
    fn structural_reference_is_grounded_out_even_if_the_llm_guesses_signal() {
        let mut e = ev("B13");
        e.structural_ref_context = true; // "Table B13.25"
        let t = classify_entity(&e, |_| EntityType::Signal);
        assert_eq!(
            t,
            EntityType::StructuralRef,
            "a table ref is never a signal"
        );
    }

    #[test]
    fn silent_document_defers_to_the_llm() {
        // TXSACTIVE: a real signal the noisy catalogs missed — no Rust ground fires, the LLM's
        // world knowledge carries it.
        let t = classify_entity(&ev("TXSACTIVE"), |_| EntityType::Signal);
        assert_eq!(t, EntityType::Signal);
        // And a token the LLM knows is boilerplate stays boilerplate when ungrounded.
        let t = classify_entity(&ev("LICENSEE"), |_| EntityType::Boilerplate);
        assert_eq!(t, EntityType::Boilerplate);
        assert!(!is_valid_signal_subject(t));
    }

    #[test]
    fn parse_is_fail_safe() {
        assert_eq!(EntityType::parse("Signal."), EntityType::Signal);
        assert_eq!(
            EntityType::parse("a transaction type"),
            EntityType::Transaction
        );
        assert_eq!(EntityType::parse("???"), EntityType::Unknown);
    }
}
