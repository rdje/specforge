//! `audit-extraction` — PDF-VARIANT-DIGESTION.4b.1: a proposer/verifier VLM AUDIT of the broadened,
//! table-driven extraction.
//!
//! The `.2`/`.2b`/`.2b'`/`.2c` work let SpecForge extract registers, fields, and signals from many more
//! table shapes across the corpus — but only four wire-based specs have gold-verified precision. This
//! command estimates the precision of the *rest* without authoring a gold for every document: it samples a
//! bounded set of intent-bearing data tables from a `SourceIR`, re-reads each against its rendered table
//! IMAGE with the VLM (the `.2b` consistency gate run as an AUDIT), and reports a precision ESTIMATE plus a
//! flagged-mismatch list for human review. The VLM is an imperfect oracle, so the number is an *estimate*
//! and every disagreement is surfaced by name — never hidden ([[feedback_scoring_rigor]]).
//!
//! Agnostic by construction (ADR 0006, [[feedback_no_hardcoded_chip_spec_names]]): tables are selected AND
//! judged purely by STRUCTURE / table-kind grammar (register / bit-field / signal / encoding / timing —
//! universal digital-design vocabulary, the same "how" class blessed by `LOGIC-LEVEL-BOUNDARY`); the VLM
//! prompt carries no chip / vendor / protocol names; sampling is structural, never keyed on document
//! identity. Nothing here can memorize a particular spec.
//!
//! The VLM is a TARGETED / SAMPLED tool, not a full-doc pass (the `.2b` scaling finding), so the audit
//! operates on a bounded sample whose size and seed are explicit and reproducible.

use crate::cli::{AuditExtractionArgs, VlmProviderArg};
use crate::error::{AppError, Result};
use crate::ir::evidence::{is_register_field_header, table_is_noise};
use crate::ir::source::{SourceIr, StructuredTableRecord, TableKind};

/// The kind of intent-bearing extraction a sampled table drives. The audit question is always
/// "does the table IMAGE agree this is a <AuditKind> table?". Structural categories only — no names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AuditKind {
    /// A register-map row-per-register table or a register bit-field definition table → `RegisterRecord`s.
    Register,
    /// A signal-description table → interface signal declarations.
    Signal,
    /// A value/encoding table.
    Encoding,
    /// A timing-parameter (min/typ/max) table.
    Timing,
}

impl AuditKind {
    /// Short, agnostic label for CLI output.
    fn label(self) -> &'static str {
        match self {
            AuditKind::Register => "register",
            AuditKind::Signal => "signal",
            AuditKind::Encoding => "encoding",
            AuditKind::Timing => "timing",
        }
    }

    /// Generic human description embedded in the VLM prompt — universal digital-design vocabulary only.
    fn description(self) -> &'static str {
        match self {
            AuditKind::Register => {
                "a register / bit-field definition table (it names a register's bit-fields with their bit \
positions, access, or reset values)"
            }
            AuditKind::Signal => {
                "an interface-signal table (it names hardware signals/pins with a width, direction, or \
source/destination)"
            }
            AuditKind::Encoding => {
                "a value-encoding table (it maps encoded values/codes to their meaning)"
            }
            AuditKind::Timing => {
                "a timing-parameter table (it lists parameters with min/typ/max values and units)"
            }
        }
    }
}

/// Classify the extraction a table drives, by the SAME structural predicates the extractors use, so the
/// audited set matches what the pipeline actually consumes. Returns `None` when the table is not
/// intent-bearing (then it is not audited). Header GRAMMAR only (ADR 0006).
fn audited_kind(table: &StructuredTableRecord) -> Option<AuditKind> {
    match table.table_kind {
        TableKind::RegisterMap => return Some(AuditKind::Register),
        TableKind::SignalDescription => return Some(AuditKind::Signal),
        TableKind::Encoding => return Some(AuditKind::Encoding),
        TableKind::TimingParameter => return Some(AuditKind::Timing),
        _ => {}
    }
    // Register-FIELD tables stay `Unknown` and are recovered by the `.2` header grammar — mirror its
    // header resolution (a Docling-marked header row, else `body_rows[0]`) so the audit covers them too.
    let header: Vec<String> = table
        .header_rows
        .first()
        .or_else(|| table.body_rows.first())
        .map(|r| {
            r.iter()
                .map(|c| c.text.trim().to_ascii_lowercase())
                .collect()
        })
        .unwrap_or_default();
    if is_register_field_header(&header) {
        return Some(AuditKind::Register);
    }
    None
}

/// FNV-1a 64-bit hash — a small, dependency-free, deterministic mixer used to seed the reproducible sample.
fn fnv1a64(s: &str) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in s.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/// Deterministically pick up to `n` of the candidate table ids, reproducibly across runs without an RNG
/// dependency: order the candidates by `FNV-1a(seed \0 table_id)` and take the first `n`. Same `(ids, n,
/// seed)` always yields the same selection; a different `seed` reshuffles. Returns indices into `ids`.
fn select_sample(ids: &[&str], n: usize, seed: u64) -> Vec<usize> {
    let mut order: Vec<usize> = (0..ids.len()).collect();
    order.sort_by_key(|&i| (fnv1a64(&format!("{seed}\u{0}{}", ids[i])), i));
    order.truncate(n);
    order
}

/// PDF-VARIANT-DIGESTION.4b.1 — the kind-aware audit prompt. Generic, STRICT-JSON, no chip names.
fn build_audit_prompt(kind: AuditKind) -> String {
    format!(
        "This is a table image from a hardware specification PDF. An automated extractor read it as {}. \
Looking ONLY at the image, is that classification correct? Reply with STRICT JSON only, no prose: \
{{\"consistent\": <true|false>, \"reason\": <short string>}}.",
        kind.description()
    )
}

/// The VLM's verdict on one audited table.
#[derive(Debug, Clone, PartialEq, Eq)]
struct AuditVerdict {
    consistent: bool,
    reason: String,
}

/// Parse the VLM's `{"consistent": bool, "reason": str}` reply, tolerating ```json fences and surrounding
/// prose. Accepts a boolean or the strings "true"/"false" for `consistent`. `None` when no usable verdict
/// is present (treated as an audit error, never as agreement).
fn parse_audit_verdict(content: &str) -> Option<AuditVerdict> {
    let start = content.find('{')?;
    let end = content.rfind('}')?;
    if end < start {
        return None;
    }
    let v: serde_json::Value = serde_json::from_str(&content[start..=end]).ok()?;
    let consistent = match v.get("consistent")? {
        serde_json::Value::Bool(b) => *b,
        serde_json::Value::String(s) => match s.trim().to_ascii_lowercase().as_str() {
            "true" => true,
            "false" => false,
            _ => return None,
        },
        _ => return None,
    };
    let reason = v
        .get("reason")
        .and_then(|r| r.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    Some(AuditVerdict { consistent, reason })
}

/// The outcome of auditing one sampled table.
#[derive(Debug, Clone)]
struct AuditOutcome {
    table_id: String,
    page_id: Option<String>,
    kind: AuditKind,
    /// `None` when the VLM call failed or its reply could not be parsed into a verdict.
    verdict: Option<AuditVerdict>,
}

/// A flagged mismatch surfaced for human review.
#[derive(Debug, Clone, PartialEq, Eq)]
struct FlaggedMismatch {
    table_id: String,
    page_id: Option<String>,
    kind: AuditKind,
    reason: String,
}

/// The aggregated audit result. `judged` excludes VLM/parse errors, so the precision estimate is over
/// tables the oracle actually ruled on.
#[derive(Debug, Clone, PartialEq)]
struct AuditReport {
    sampled: usize,
    judged: usize,
    consistent: usize,
    errors: usize,
    flagged: Vec<FlaggedMismatch>,
}

impl AuditReport {
    /// `consistent / judged` — the table-kind precision ESTIMATE, or `None` when no table was judged
    /// (never fabricate a number from zero evidence — [[feedback_scoring_rigor]]).
    fn precision_estimate(&self) -> Option<f64> {
        if self.judged == 0 {
            None
        } else {
            Some(self.consistent as f64 / self.judged as f64)
        }
    }
}

/// Fold per-table outcomes into the report: count judged/consistent/errors and collect the inconsistent
/// ones (the flagged mismatches) for review.
fn aggregate(outcomes: &[AuditOutcome]) -> AuditReport {
    let mut report = AuditReport {
        sampled: outcomes.len(),
        judged: 0,
        consistent: 0,
        errors: 0,
        flagged: Vec::new(),
    };
    for o in outcomes {
        match &o.verdict {
            None => report.errors += 1,
            Some(v) => {
                report.judged += 1;
                if v.consistent {
                    report.consistent += 1;
                } else {
                    report.flagged.push(FlaggedMismatch {
                        table_id: o.table_id.clone(),
                        page_id: o.page_id.clone(),
                        kind: o.kind,
                        reason: v.reason.clone(),
                    });
                }
            }
        }
    }
    report
}

/// Resolve the live VLM model/endpoint for a non-skip provider (mirrors `enrich`).
fn resolve_provider(
    provider: VlmProviderArg,
    model_override: Option<String>,
) -> (&'static str, String, String) {
    let name = match provider {
        VlmProviderArg::Ollama => "ollama",
        VlmProviderArg::OpenAi => "openai",
        VlmProviderArg::LmStudio => "lmstudio",
        VlmProviderArg::Skip => "skip",
    };
    let model = model_override.unwrap_or_else(|| match provider {
        VlmProviderArg::OpenAi => "gpt-4o".to_string(),
        _ => "qwen2.5vl:7b".to_string(),
    });
    let api_url = match provider {
        VlmProviderArg::OpenAi => "https://api.openai.com/v1/chat/completions".to_string(),
        VlmProviderArg::LmStudio => "http://localhost:1234/v1/chat/completions".to_string(),
        _ => "http://localhost:11434/v1/chat/completions".to_string(),
    };
    (name, model, api_url)
}

/// `audit-extraction` entrypoint.
pub fn run(args: AuditExtractionArgs) -> Result<()> {
    if !args.source_ir.exists() {
        return Err(AppError::MissingPath(args.source_ir));
    }
    let source_ir = SourceIr::load_from_path(&args.source_ir)?;

    // Image lookup by asset id — only tables with a rendered image can be re-read by the VLM.
    let image_by_asset: std::collections::HashMap<&str, &std::path::Path> = source_ir
        .visual_assets
        .iter()
        .filter_map(|a| a.image_path.as_deref().map(|p| (a.asset_id.as_str(), p)))
        .collect();

    // Candidate = intent-bearing (by structure), not noise (TOC/revision/index), and has a table image.
    let candidates: Vec<(usize, AuditKind)> = source_ir
        .structured_tables
        .iter()
        .enumerate()
        .filter(|(_, t)| !table_is_noise(t) && image_by_asset.contains_key(t.asset_id.as_str()))
        .filter_map(|(i, t)| audited_kind(t).map(|k| (i, k)))
        .collect();

    let candidate_ids: Vec<&str> = candidates
        .iter()
        .map(|(i, _)| source_ir.structured_tables[*i].table_id.as_str())
        .collect();
    let picked = select_sample(&candidate_ids, args.sample, args.seed);

    let (provider_name, model, api_url) = resolve_provider(args.provider, args.model.clone());

    println!("command: audit-extraction");
    println!(
        "mode: {}",
        if matches!(args.provider, VlmProviderArg::Skip) {
            "plan-only"
        } else {
            "execute"
        }
    );
    println!("source_ir_path: {}", args.source_ir.display());
    println!("document_key: {}", source_ir.document_identity.document_key);
    println!("intent_bearing_tables: {}", candidates.len());
    println!("sample_size: {}", picked.len());
    println!("seed: {}", args.seed);
    println!("provider: {provider_name}");

    if matches!(args.provider, VlmProviderArg::Skip) {
        // Plan-only: show exactly which tables WOULD be audited (no VLM calls). This is the hermetic /
        // CI-safe path; the precision estimate requires a live VLM run (`.4b.2`).
        println!(
            "audit_mode: plan-only (no VLM calls; re-run with --provider ollama to estimate precision)"
        );
        for &ci in &picked {
            let (ti, kind) = candidates[ci];
            let t = &source_ir.structured_tables[ti];
            println!(
                "audit_candidate: {} page={} kind={}",
                t.table_id,
                t.page_id.as_deref().unwrap_or("?"),
                kind.label()
            );
        }
        return Ok(());
    }

    println!("vlm_model: {model}");
    let mut outcomes: Vec<AuditOutcome> = Vec::new();
    for &ci in &picked {
        let (ti, kind) = candidates[ci];
        let table = &source_ir.structured_tables[ti];
        let Some(image_path) = image_by_asset.get(table.asset_id.as_str()) else {
            // Should not happen (candidates were filtered on image presence), but stay defensive.
            outcomes.push(AuditOutcome {
                table_id: table.table_id.clone(),
                page_id: table.page_id.clone(),
                kind,
                verdict: None,
            });
            continue;
        };
        let prompt = build_audit_prompt(kind);
        let verdict = match crate::commands::enrich::vlm_image_query(
            image_path,
            &prompt,
            &model,
            &api_url,
            args.provider,
        ) {
            Ok(content) => parse_audit_verdict(&content),
            Err(_) => None,
        };
        outcomes.push(AuditOutcome {
            table_id: table.table_id.clone(),
            page_id: table.page_id.clone(),
            kind,
            verdict,
        });
    }

    let report = aggregate(&outcomes);
    println!("audited: {}", report.sampled);
    println!("judged: {}", report.judged);
    println!("consistent: {}", report.consistent);
    println!("vlm_errors: {}", report.errors);
    match report.precision_estimate() {
        Some(p) => println!("table_kind_precision_estimate: {p:.3}"),
        None => println!("table_kind_precision_estimate: n/a (no table judged)"),
    }
    println!("flagged_mismatches: {}", report.flagged.len());
    for f in &report.flagged {
        println!(
            "flagged: {} page={} kind={} reason=\"{}\"",
            f.table_id,
            f.page_id.as_deref().unwrap_or("?"),
            f.kind.label(),
            f.reason
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::source::{StructuredTableCellRecord, StructuredTableRecord, TableKind};

    fn cell(text: &str, is_header: bool) -> StructuredTableCellRecord {
        StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        }
    }

    fn table(table_id: &str, kind: TableKind, header: &[&str]) -> StructuredTableRecord {
        StructuredTableRecord {
            table_id: table_id.to_string(),
            asset_id: format!("asset_{table_id}"),
            page_id: Some("p1".to_string()),
            caption_text: None,
            source_ref: None,
            table_kind: kind,
            header_rows: vec![header.iter().map(|h| cell(h, true)).collect()],
            body_rows: vec![],
            row_count: 1,
            col_count: header.len() as u32,
        }
    }

    #[test]
    fn audited_kind_classifies_intent_bearing_tables_by_structure() {
        // Confident data kinds map directly.
        assert_eq!(
            audited_kind(&table(
                "t1",
                TableKind::RegisterMap,
                &["Offset", "Name", "Access"]
            )),
            Some(AuditKind::Register)
        );
        assert_eq!(
            audited_kind(&table(
                "t2",
                TableKind::SignalDescription,
                &["Signal", "Width", "Direction"]
            )),
            Some(AuditKind::Signal)
        );
        assert_eq!(
            audited_kind(&table("t3", TableKind::Encoding, &["Value", "Meaning"])),
            Some(AuditKind::Encoding)
        );
        assert_eq!(
            audited_kind(&table(
                "t4",
                TableKind::TimingParameter,
                &["Param", "Min", "Max"]
            )),
            Some(AuditKind::Timing)
        );
        // A register-FIELD table stays Unknown but the `.2` grammar recovers it → audited as register.
        assert_eq!(
            audited_kind(&table(
                "t5",
                TableKind::Unknown,
                &["Field", "Description", "Access", "Reset"]
            )),
            Some(AuditKind::Register)
        );
        // A header-in-body register-field table (no Docling header row).
        let mut t6 = table("t6", TableKind::Unknown, &[]);
        t6.header_rows = vec![];
        t6.body_rows = vec![
            vec![
                cell("Bits", false),
                cell("Name", false),
                cell("Reset", false),
            ],
            vec![cell("3:0", false), cell("MODE", false), cell("0", false)],
        ];
        assert_eq!(audited_kind(&t6), Some(AuditKind::Register));
        // A plain unknown table with no recognizable structure is NOT audited.
        assert_eq!(
            audited_kind(&table("t7", TableKind::Unknown, &["Lorem", "Ipsum"])),
            None
        );
    }

    #[test]
    fn select_sample_is_deterministic_bounded_and_seed_sensitive() {
        let ids = ["a", "b", "c", "d", "e", "f", "g", "h"];
        let s0 = select_sample(&ids, 3, 0);
        // Reproducible for the same (ids, n, seed).
        assert_eq!(s0, select_sample(&ids, 3, 0));
        // Bounded.
        assert_eq!(s0.len(), 3);
        // n >= len returns every index exactly once.
        let all = select_sample(&ids, 99, 0);
        assert_eq!(all.len(), ids.len());
        let mut sorted = all.clone();
        sorted.sort_unstable();
        assert_eq!(sorted, (0..ids.len()).collect::<Vec<_>>());
        // A different seed generally reshuffles the order (different selection here).
        assert_ne!(select_sample(&ids, 3, 1), s0);
        // Empty input is safe.
        assert!(select_sample(&[], 3, 0).is_empty());
    }

    #[test]
    fn parse_audit_verdict_is_tolerant() {
        assert_eq!(
            parse_audit_verdict(
                "```json\n{\"consistent\": true, \"reason\": \"clear register table\"}\n```"
            ),
            Some(AuditVerdict {
                consistent: true,
                reason: "clear register table".to_string()
            })
        );
        // prose-wrapped + string boolean + missing reason
        assert_eq!(
            parse_audit_verdict("Sure! {\"consistent\": \"false\"} hope that helps"),
            Some(AuditVerdict {
                consistent: false,
                reason: String::new()
            })
        );
        // missing the field, or no JSON at all → no verdict (an audit error, never agreement)
        assert_eq!(parse_audit_verdict("{\"reason\": \"hmm\"}"), None);
        assert_eq!(parse_audit_verdict("no json here"), None);
        assert_eq!(parse_audit_verdict("{\"consistent\": 7}"), None);
    }

    #[test]
    fn build_audit_prompt_is_agnostic_and_strict_json() {
        for kind in [
            AuditKind::Register,
            AuditKind::Signal,
            AuditKind::Encoding,
            AuditKind::Timing,
        ] {
            let p = build_audit_prompt(kind);
            assert!(p.contains("STRICT JSON"));
            assert!(p.contains("\"consistent\""));
            // No chip / vendor / protocol names may leak into the runtime prompt (ADR 0006).
            let lower = p.to_ascii_lowercase();
            for forbidden in [
                "amba", "axi", "apb", "ahb", "riscv", "risc-v", "nvme", "arm", "i2c",
            ] {
                assert!(!lower.contains(forbidden), "prompt leaked `{forbidden}`");
            }
        }
    }

    #[test]
    fn aggregate_computes_precision_and_flags_mismatches() {
        let outcomes = vec![
            AuditOutcome {
                table_id: "t1".into(),
                page_id: Some("p1".into()),
                kind: AuditKind::Register,
                verdict: Some(AuditVerdict {
                    consistent: true,
                    reason: String::new(),
                }),
            },
            AuditOutcome {
                table_id: "t2".into(),
                page_id: Some("p2".into()),
                kind: AuditKind::Signal,
                verdict: Some(AuditVerdict {
                    consistent: false,
                    reason: "looks like an example table".into(),
                }),
            },
            AuditOutcome {
                table_id: "t3".into(),
                page_id: None,
                kind: AuditKind::Encoding,
                verdict: None, // VLM error — excluded from the denominator
            },
        ];
        let report = aggregate(&outcomes);
        assert_eq!(report.sampled, 3);
        assert_eq!(report.judged, 2);
        assert_eq!(report.consistent, 1);
        assert_eq!(report.errors, 1);
        assert_eq!(report.flagged.len(), 1);
        assert_eq!(report.flagged[0].table_id, "t2");
        assert_eq!(report.precision_estimate(), Some(0.5));

        // No judged tables → no fabricated number.
        let empty = aggregate(&[]);
        assert_eq!(empty.precision_estimate(), None);
    }
}
