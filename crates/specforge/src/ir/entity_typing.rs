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
//! 3. **Judgment (LLM):** given that evidence with the document-owned identifier redacted, decide
//!    the type from typed declarations and grammatical context ([`classify_entity`]'s injected `propose`).
//! 4. **Grounding (Rust):** [`classify_entity`] *overrides* the LLM where the document is
//!    unambiguous, and *defers* where it is silent. The document grounds everything → the same code
//!    works on any chip-spec PDF.

use crate::ir::evidence::{EvidenceIr, collect_known_signal_names};
use crate::llm_text::{api_url, call_text_provider};
use crate::provider::VlmProviderArg;

/// Resolve a proposed identifier against a current-document catalog. Exact spelling wins;
/// case-insensitive recovery is accepted only when it identifies one unique opaque name.
/// This permits presentation recovery without making case folding part of identifier identity.
pub fn resolve_unique_document_identifier<'a>(
    proposed: &str,
    identities: impl IntoIterator<Item = &'a str>,
) -> Option<&'a str> {
    let identities = identities.into_iter().collect::<Vec<_>>();
    if let Some(exact) = identities
        .iter()
        .find(|identity| identity.trim() == proposed.trim())
    {
        return Some(exact.trim());
    }

    let mut folded = identities
        .iter()
        .map(|identity| identity.trim())
        .filter(|identity| identity.eq_ignore_ascii_case(proposed.trim()))
        .collect::<Vec<_>>();
    folded.sort_unstable();
    folded.dedup();
    (folded.len() == 1).then(|| folded[0])
}

/// `EXTRACTION-QUALITY-GAUGE.3j.2.a.i` — resolve a subject spelled as a **full-width bit slice** to
/// the signal it names: `X[w-1:0]`, where `w` is the width the document states for `X`, denotes `X`
/// exactly, so resolving it loses nothing.
///
/// This is deliberately NOT part of [`resolve_unique_document_identifier`], whose contract is opaque
/// identity and which other surfaces depend on; slice grammar is a separate question asked separately.
///
/// **What it refuses is the point** (`.3j.2.a` adjudicated all 16 carried-name refusals):
/// - a **proper sub-slice** — `AWSNOOP[3]` of a stated width 4 — because `AWSNOOP must be LOW` is a
///   strictly stronger obligation than `AWSNOOP[3] must be tied LOW`, and one the document never made;
/// - a slice whose signal **states no width**, because the comparison cannot be evaluated and an
///   unanswerable question is not a licence to resolve. Only 209 of 353 declared names state a width.
/// - a bare **qualifier** (`WTAG bits`, `Subordinate LAPM`): measured 1 correct of 6, so it is refused.
pub fn resolve_full_width_slice_alias<'a>(
    proposed: &str,
    identities: impl IntoIterator<Item = &'a str>,
    stated_width: impl Fn(&str) -> Option<u64>,
) -> Option<&'a str> {
    let proposed = proposed.trim();
    let (base, high, low) = full_width_candidate(proposed)?;
    if low != 0 {
        return None;
    }
    let identity = resolve_unique_document_identifier(base, identities)?;
    (stated_width(identity) == Some(u64::from(high) + 1)).then_some(identity)
}

/// Split `X[hi:lo]` or `X[n]` into its base name and inclusive span. A subject with no bracket, a
/// trailing remainder after the bracket, or a span this grammar cannot read as plain numerals is not
/// a slice claim at all and yields `None` — which keeps a qualifier (`WTAG bits`) and a sub-field
/// spelling (`LRMPAM .PARTID[11:9]`, whose base does not resolve) out of the rule by construction.
fn full_width_candidate(proposed: &str) -> Option<(&str, u32, u32)> {
    let open = proposed.find('[')?;
    let close = proposed.rfind(']')?;
    if close + 1 != proposed.len() || close < open {
        return None;
    }
    let inner = proposed.get(open + 1..close)?.trim();
    let (high, low) = match inner.split_once(':') {
        Some((high, low)) => (high.trim(), low.trim()),
        None => (inner, inner),
    };
    let (high, low) = (high.parse::<u32>().ok()?, low.parse::<u32>().ok()?);
    (high >= low).then(|| (proposed.get(..open).unwrap_or_default().trim(), high, low))
}

/// Current-document signal declarations in source/provenance order. Prompt builders must preserve
/// this order: sorting by the opaque spelling would let alpha-renaming perturb model policy.
pub fn declared_signal_catalog(ir: &EvidenceIr) -> Vec<String> {
    let mut catalog = Vec::new();
    for statement in &ir.extracted_statements {
        let mut names = collect_known_signal_names(std::slice::from_ref(statement))
            .into_iter()
            .collect::<Vec<_>>();
        let folded_statement = statement.text.to_ascii_lowercase();
        names.sort_by_key(|name| {
            folded_statement
                .find(&name.to_ascii_lowercase())
                .unwrap_or(usize::MAX)
        });
        for name in names {
            if !catalog.contains(&name) {
                catalog.push(name);
            }
        }
    }
    for declaration in &ir.table_signal_declaration_provenance {
        if !catalog.contains(&declaration.signal_name) {
            catalog.push(declaration.signal_name.clone());
        }
    }
    catalog
}

/// The entity classes a chip-spec token can be. A constraint/relation may only take a `Signal`
/// subject; the rest are exactly the things CHI mis-typed *as* signals.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Signal,
    /// A MESSAGE FIELD — a named portion of a packet/flit/message payload (`TxnID`, `DBID`).
    /// Protocol intent, but NOT a signal: no wire, no direction (EXTRACTION-QUALITY-GAUGE.FIELD.3).
    Field,
    Actor,
    Transaction,
    Feature,
    State,
    /// A reference to a Table/Figure/Section (`Table B13.25` → `B13`).
    StructuralRef,
    /// Legal / front-matter boilerplate (licensee names, marks, and notices).
    Boilerplate,
    Value,
    Unknown,
}

impl EntityType {
    pub fn as_str(self) -> &'static str {
        match self {
            EntityType::Signal => "signal",
            EntityType::Field => "field",
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
                "field" => Some(EntityType::Field),
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
    /// Declared in a message-field table (`message_field_records`) — authoritative for `Field`
    /// when the signal tables are silent (EXTRACTION-QUALITY-GAUGE.FIELD.3).
    pub declared_in_field_table: bool,
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
    let declared_signal = resolve_unique_document_identifier(
        token,
        ir.table_signal_declaration_provenance
            .iter()
            .map(|declaration| declaration.signal_name.as_str()),
    );
    let declared_field_name = resolve_unique_document_identifier(
        token,
        ir.message_field_records
            .iter()
            .map(|field| field.name.as_str()),
    );
    let semantic_hint_signal = resolve_unique_document_identifier(
        token,
        ir.signal_semantic_hints
            .iter()
            .map(|hint| hint.signal_name.as_str()),
    );
    let semantic_hint_tags: Vec<String> = semantic_hint_signal
        .into_iter()
        .flat_map(|resolved| {
            ir.signal_semantic_hints
                .iter()
                .filter(move |hint| hint.signal_name.trim() == resolved)
        })
        .flat_map(|hint| hint.semantic_tags.iter().map(|tag| format!("{tag:?}")))
        .collect();
    let actor_identity = resolve_unique_document_identifier(
        token,
        ir.actor_signal_relations
            .iter()
            .map(|relation| relation.actor_name.as_str()),
    );
    let signal_identity = resolve_unique_document_identifier(
        token,
        ir.actor_signal_relations
            .iter()
            .map(|relation| relation.signal_name.as_str()),
    );
    let tl = token.trim().to_ascii_lowercase();
    let structural_ref_context = context_texts.iter().any(|t| {
        let l = t.to_ascii_lowercase();
        l.contains(&format!("table {tl}")) || l.contains(&format!("figure {tl}"))
    });
    EntityEvidence {
        token: token.to_string(),
        declared_in_signal_table: declared_signal.is_some(),
        declared_in_field_table: declared_field_name.is_some(),
        semantic_hint_tags,
        appears_as_actor: actor_identity.is_some(),
        appears_as_signal: signal_identity.is_some(),
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
    // A signal-table declaration outranks a field-table one when a name appears in both.
    if ev.declared_in_signal_table {
        return EntityType::Signal;
    }
    // POSITIVE ground: declared in a message-field table (and in no signal table) →
    // authoritatively a Field — flit/message content, not a wire. This grounds the CHI-class
    // `DBID`/`TxnID` vocabulary OUT of signal subjects without consulting the LLM at all
    // (EXTRACTION-QUALITY-GAUGE.FIELD.3).
    if ev.declared_in_field_table {
        return EntityType::Field;
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

const OPAQUE_IDENTIFIER_PLACEHOLDER: &str = "<OPAQUE_IDENTIFIER>";

/// Hide one document-owned identifier without damaging unrelated words that merely contain the
/// same characters. ASCII case variants are also hidden because presentation recovery may have
/// changed case before this prompt is built.
fn redact_opaque_identifier(text: &str, identifier: &str) -> String {
    let identifier = identifier.trim();
    if identifier.is_empty() {
        return text.to_string();
    }
    let folded_text = text.to_ascii_lowercase();
    let folded_identifier = identifier.to_ascii_lowercase();
    let require_word_boundaries = identifier.chars().count() == 1;
    let mut output = String::with_capacity(text.len());
    let mut cursor = 0usize;

    while let Some(relative) = folded_text[cursor..].find(&folded_identifier) {
        let start = cursor + relative;
        let end = start + folded_identifier.len();
        let left_is_word = text[..start]
            .chars()
            .next_back()
            .is_some_and(|c| c.is_alphanumeric() || c == '_');
        let right_is_word = text[end..]
            .chars()
            .next()
            .is_some_and(|c| c.is_alphanumeric() || c == '_');
        if require_word_boundaries && (left_is_word || right_is_word) {
            let next = text[start..]
                .char_indices()
                .nth(1)
                .map_or(text.len(), |(offset, _)| start + offset);
            output.push_str(&text[cursor..next]);
            cursor = next;
            continue;
        }
        output.push_str(&text[cursor..start]);
        output.push_str(OPAQUE_IDENTIFIER_PLACEHOLDER);
        cursor = end;
    }
    output.push_str(&text[cursor..]);
    output
}

/// The LLM judgment prompt — the token + the Rust-gathered evidence + the closed type set.
pub fn entity_prompt(ev: &EntityEvidence) -> String {
    let snippets = ev
        .context_snippets
        .iter()
        .map(|s| redact_opaque_identifier(s, &ev.token))
        .map(|s| format!("  - {}", s.chars().take(180).collect::<String>()))
        .collect::<Vec<_>>()
        .join("\n");
    let semantic_hint_tags =
        redact_opaque_identifier(&format!("{:?}", ev.semantic_hint_tags), &ev.token);
    format!(
        "Classify an opaque identifier from a digital-hardware specification as EXACTLY ONE of: signal, field, \
         actor, transaction, feature, state, structural_ref, boilerplate, value.\n\
         signal = a physical wire/pin carrying a value; field = a named portion of a \
         packet/flit/message payload (not a wire); actor = a component that acts; transaction = a \
         named operation; feature = a capability; state = a named behavioral state; structural_ref = \
         a Table/Figure/Section reference; boilerplate = a legal/front-matter term; value = a literal value.\n\
         The identifier spelling is intentionally hidden. Use only the typed evidence and grammatical \
         context below; do not reconstruct or guess the identifier.\n\n\
         Token: {}\n\
         Declared in a signal table: {}\n\
         Declared in a message-field table: {}\n\
         Document semantic tags: {}\n\
         Used as the actor (subject) of a requirement: {}\n\
         Used as the signal (object) of a requirement: {}\n\
         Current-document context:\n{}\n\n\
         Answer with ONE word.",
        OPAQUE_IDENTIFIER_PLACEHOLDER,
        ev.declared_in_signal_table,
        ev.declared_in_field_table,
        semantic_hint_tags,
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
        OPAQUE_IDENTIFIER_PLACEHOLDER,
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
    fn declared_field_is_grounded_to_field_without_asking_the_llm() {
        // EXTRACTION-QUALITY-GAUGE.FIELD.3 — the CHI-class fix: DBID is declared in a
        // message-field table, so it is authoritatively a Field even when the LLM (or the old
        // conflated prompt) would call it a signal — and a Field is never a valid signal subject.
        let mut e = ev("DBID");
        e.declared_in_field_table = true;
        let t = classify_entity(&e, |_| EntityType::Signal);
        assert_eq!(t, EntityType::Field);
        assert!(
            !is_valid_signal_subject(t),
            "a field is not a signal subject"
        );
    }

    #[test]
    fn signal_table_declaration_outranks_field_table_declaration() {
        // A name declared in BOTH a signal table and a field table stays a Signal — the
        // signal-table declaration is the stronger, wire-level authority.
        let mut e = ev("XDATA");
        e.declared_in_signal_table = true;
        e.declared_in_field_table = true;
        assert_eq!(
            classify_entity(&e, |_| EntityType::Boilerplate),
            EntityType::Signal
        );
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
    fn silent_document_defers_to_contextual_llm_judgment() {
        // The injected decision may use grammatical context, but the production prompt never
        // exposes this spelling to the provider.
        let t = classify_entity(&ev("opaque_atom"), |_| EntityType::Signal);
        assert_eq!(t, EntityType::Signal);
        let t = classify_entity(&ev("other_atom"), |_| EntityType::Boilerplate);
        assert_eq!(t, EntityType::Boilerplate);
        assert!(!is_valid_signal_subject(t));
    }

    #[test]
    fn entity_prompt_is_alpha_equivariant_and_hides_identifier_spelling() {
        let mut first = ev("orchid");
        first.context_snippets =
            vec!["The ORCHID output is sampled here; orchid stays high.".into()];
        first.semantic_hint_tags = vec!["orchid_role".into()];
        let mut renamed = ev("juniper");
        renamed.context_snippets =
            vec!["The JUNIPER output is sampled here; juniper stays high.".into()];
        renamed.semantic_hint_tags = vec!["juniper_role".into()];

        let first_prompt = entity_prompt(&first);
        assert_eq!(first_prompt, entity_prompt(&renamed));
        assert!(!first_prompt.to_ascii_lowercase().contains("orchid"));
        assert!(first_prompt.contains(OPAQUE_IDENTIFIER_PLACEHOLDER));
        assert!(first_prompt.contains("Use only the typed evidence and grammatical context"));
    }

    #[test]
    fn identifier_redaction_respects_unrelated_word_boundaries() {
        assert_eq!(
            redact_opaque_identifier("A is distinct from DATA and a.", "A"),
            "<OPAQUE_IDENTIFIER> is distinct from DATA and <OPAQUE_IDENTIFIER>."
        );
    }

    #[test]
    fn parse_is_fail_safe() {
        assert_eq!(EntityType::parse("Signal."), EntityType::Signal);
        assert_eq!(EntityType::parse("field"), EntityType::Field);
        assert_eq!(
            EntityType::parse("a transaction type"),
            EntityType::Transaction
        );
        assert_eq!(EntityType::parse("???"), EntityType::Unknown);
    }

    #[test]
    fn document_identifier_resolution_is_exact_first_and_ambiguity_failing() {
        let catalog = ["sig", "SIG", "mixedCase"];
        assert_eq!(
            resolve_unique_document_identifier("SIG", catalog),
            Some("SIG")
        );
        assert_eq!(
            resolve_unique_document_identifier("MIXEDCASE", catalog),
            Some("mixedCase")
        );
        assert_eq!(resolve_unique_document_identifier("SiG", catalog), None);
    }
    /// `EXTRACTION-QUALITY-GAUGE.3j.2.a.i` — the wired rule, pinned in every direction it must
    /// separate. The catalog is a set of opaque tokens and the rule never reads one (ADR 0006); the
    /// spellings are `.3j.2.a`'s own measured instances.
    #[test]
    fn a_full_width_slice_resolves_and_nothing_else_does() {
        let catalog = ["ARLEN", "ARCACHE", "AWCMO", "AWSNOOP", "LAPAS"];
        let width = |name: &str| match name {
            "ARLEN" => Some(8),
            "ARCACHE" => Some(4),
            "AWCMO" => Some(2),
            "AWSNOOP" => Some(4),
            // LAPAS enters the catalog through the table-declaration surface, which states no width.
            _ => None,
        };

        // GREEN — the three measured full-width instances resolve to the signal they name.
        for (spelling, identity) in [
            ("ARLEN[7:0]", "ARLEN"),
            ("ARCACHE[3:0]", "ARCACHE"),
            ("AWCMO[1:0]", "AWCMO"),
        ] {
            assert_eq!(
                resolve_full_width_slice_alias(spelling, catalog, width),
                Some(identity),
                "a slice spanning the whole stated width denotes the signal itself"
            );
        }

        // RED — the case the adjudication turned on. One bit of four is not the signal, and
        // resolving it would assert a stronger obligation than the document ever stated.
        assert_eq!(
            resolve_full_width_slice_alias("AWSNOOP[3]", catalog, width),
            None,
            "a proper sub-slice must never resolve"
        );
        // A span reaching the top but not bit 0 is still partial.
        assert_eq!(
            resolve_full_width_slice_alias("ARCACHE[3:1]", catalog, width),
            None
        );
        // A width the document does not state leaves the question unanswerable, not answered.
        assert_eq!(
            resolve_full_width_slice_alias("LAPAS[2:1]", catalog, width),
            None
        );
        assert_eq!(
            resolve_full_width_slice_alias("ARLEN[7:0]", catalog, |_| None),
            None,
            "the same spelling must refuse once its width is unknown"
        );
        // A bare qualifier is a different question, and `.3j.2.a` answered it NO.
        assert_eq!(
            resolve_full_width_slice_alias("ARLEN bits", catalog, width),
            None
        );
        // A base the catalog does not declare cannot be recovered by its slice.
        assert_eq!(
            resolve_full_width_slice_alias("LRMPAM .PARTID[11:9]", catalog, width),
            None
        );
        // Trailing text after the bracket is not a slice claim.
        assert_eq!(
            resolve_full_width_slice_alias("ARLEN[7:0] output", catalog, width),
            None
        );
        // A signal named without any slice is not this rule's business.
        assert_eq!(
            resolve_full_width_slice_alias("ARLEN", catalog, width),
            None
        );
    }
}
