//! EXTRACTION-GAP-FIX.4a — `specforge recover-register-bits <evidence-ir>`.
//!
//! Recover register-field bit positions that live ONLY in a register-layout GRAPHIC (not in the
//! field-definition table) by reading field NAMES + per-field WIDTHS in MSB→LSB order off the
//! diagram image with a VLM, then reconstructing the absolute bit ranges by cumulative LSB tiling.
//! All the trust lives in the pure, gated core ([`crate::ir::register_bits`]): the VLM's absolute
//! positions are discarded, and a reconstruction is accepted only when the widths tile a standard
//! register width AND the names match the register's own field table — else an honest residual.
//!
//! Architecture (mirrors `nlp-enrich`: EvidenceIR in → EvidenceIR out):
//!   Input:  an EvidenceIR whose `register_records` carry named fields with EMPTY bit positions.
//!   Action: for each such register, resolve its bit-layout diagram image (a `RegisterBitfield`
//!           visual asset on the register's page in the sibling SourceIR), ask the VLM for
//!           `(field_name, width)` MSB→LSB, and run the gated tiling core.
//!   Output: bit positions attached to the register's fields when (and only when) both gates pass;
//!           the updated EvidenceIR JSON is written back to disk.
//!
//! `--provider skip` (the default) makes the command a CI-safe no-op: nothing is read and nothing
//! is written. The live VLM is therefore never a CI dependency; hermetic tests drive the proposer
//! through the shared `SPECFORGE_VLM_HELPER` hook.

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::cli::{RecoverRegisterBitsArgs, VlmProviderArg};
use crate::commands::llm_text::VLM_HELPER_ENV;
use crate::error::{AppError, Result};
use crate::ir::evidence::EvidenceIr;
use crate::ir::register_bits::{
    RegisterBitRecoveryOutcome, RegisterDiagramFieldProposal, recover_bits_for_register,
};
use crate::ir::source::{DiagramKind, RegisterRecord, SourceIr};
use crate::persisted_path::{PersistedPathOrigin, resolve_existing};

/// The synthetic prefix `synthesize_register_field_tables` puts on a field-table register's id,
/// from which the source `StructuredTableRecord.table_id` (and thus its page) is recoverable.
const FIELD_TABLE_REGISTER_PREFIX: &str = "regfld_";

pub fn run(args: RecoverRegisterBitsArgs) -> Result<()> {
    let evidence_ir_path =
        resolve_existing(&args.evidence_ir, PersistedPathOrigin::RepositoryOwned)?;
    let mut evidence_ir = EvidenceIr::load_from_path(&evidence_ir_path)?;

    println!("command: recover-register-bits");
    println!("mode: {}", if args.dry_run { "dry-run" } else { "execute" });
    println!("evidence_ir_path: {}", evidence_ir_path.display());
    println!(
        "document_key: {}",
        evidence_ir.document_identity.document_key
    );
    let candidate_registers = evidence_ir
        .register_records
        .iter()
        .filter(|r| register_needs_bit_recovery(r))
        .count();
    println!("registers_missing_bits: {candidate_registers}");

    if matches!(args.vlm_provider, VlmProviderArg::Skip) {
        println!("vlm_provider: skip");
        println!("bit_recovery: skipped");
        println!(
            "hint: re-run with --vlm-provider ollama --vlm-model qwen2.5vl:7b to recover bit positions \
from {candidate_registers} register diagram(s)"
        );
        return Ok(());
    }

    let provider = args.vlm_provider;
    let provider_name = crate::commands::llm_text::provider_name(provider);
    let model = args
        .vlm_model
        .clone()
        .unwrap_or_else(|| crate::commands::llm_text::default_model(provider));
    let api_url = crate::commands::llm_text::api_url(provider).to_string();
    println!("vlm_provider: {provider_name}");
    println!("vlm_model: {model}");

    // The diagram images live in the sibling SourceIR referenced by the EvidenceIR.
    let source_ir = SourceIr::load_from_path(&evidence_ir.source_ir_path)?;

    // Phase 1 (immutable): decide, per candidate register, which diagram image to read.
    let plan: Vec<RegisterPlan> = evidence_ir
        .register_records
        .iter()
        .enumerate()
        .filter(|(_, r)| register_needs_bit_recovery(r))
        .map(|(index, register)| RegisterPlan {
            index,
            register_name: register.register_name.clone(),
            image: resolve_diagram_image_for_register(register, &source_ir),
        })
        .collect();

    // Phase 2 (mutable): read each diagram and apply the gated tiling core.
    let mut registers_recovered = 0usize;
    let mut fields_recovered = 0usize;
    let mut residuals = 0usize;
    for entry in plan {
        let Some((asset_id, image_path)) = entry.image else {
            println!(
                "register: {} outcome: residual_no_diagram",
                entry.register_name
            );
            residuals += 1;
            continue;
        };
        let proposals = match propose_register_diagram_fields(
            &asset_id,
            &image_path,
            &model,
            &api_url,
            provider,
        ) {
            Ok(proposals) => proposals,
            Err(error) => {
                eprintln!(
                    "warning: register {} diagram read failed ({asset_id}): {error}",
                    entry.register_name
                );
                residuals += 1;
                continue;
            }
        };
        let outcome =
            recover_bits_for_register(&mut evidence_ir.register_records[entry.index], &proposals);
        match outcome {
            RegisterBitRecoveryOutcome::Recovered { count } => {
                registers_recovered += 1;
                fields_recovered += count;
                println!(
                    "register: {} outcome: recovered fields: {count}",
                    entry.register_name
                );
            }
            other => {
                residuals += 1;
                println!(
                    "register: {} outcome: {} (honest residual — no bits fabricated)",
                    entry.register_name,
                    other.as_str()
                );
            }
        }
    }

    println!("registers_recovered: {registers_recovered}");
    println!("fields_recovered: {fields_recovered}");
    println!("residuals: {residuals}");

    if registers_recovered > 0 && !args.dry_run {
        evidence_ir.write_to_disk()?;
        println!("evidence_ir_written: {}", args.evidence_ir.display());
    } else if args.dry_run {
        println!("evidence_ir_written: skipped (dry-run)");
    } else {
        println!("evidence_ir_written: skipped (no register recovered)");
    }

    Ok(())
}

/// A per-register decision from phase 1: which diagram image (if any) to read for this register.
struct RegisterPlan {
    index: usize,
    register_name: String,
    image: Option<(String, PathBuf)>,
}

/// A register is a recovery candidate when it has ≥1 field and EVERY field is missing its bit
/// position — the "bits live in the graphic" shape. Registers whose bits came from a table are
/// already complete and are left untouched (no override of deterministic extraction).
fn register_needs_bit_recovery(register: &RegisterRecord) -> bool {
    !register.fields.is_empty()
        && register
            .fields
            .iter()
            .all(|f| f.bits_high.is_none() && f.bits_low.is_none())
}

/// Parse the trailing `_<n>` of a page id (e.g. `page_0020` → 20), matching the SourceIR convention.
fn page_number(page_id: &str) -> Option<u32> {
    page_id.rsplit('_').next()?.parse().ok()
}

/// Resolve the register's bit-layout diagram image: a visual asset on the SAME page as the
/// register's source field table. Prefer an asset whose caption names the register; otherwise fall
/// back to the unique `RegisterBitfield` asset on that page. Returns `None` (honest residual) when
/// the page or a single unambiguous image cannot be determined — never a guess among several.
fn resolve_diagram_image_for_register(
    register: &RegisterRecord,
    source_ir: &SourceIr,
) -> Option<(String, PathBuf)> {
    // Only field-table registers carry a recoverable source-table id (and thus a page).
    let table_id = register
        .register_id
        .strip_prefix(FIELD_TABLE_REGISTER_PREFIX)?;
    let table = source_ir
        .structured_tables
        .iter()
        .find(|t| t.table_id == table_id)?;
    let page = table.page_id.as_deref().and_then(page_number)?;

    // Candidate assets: on the same page, with a usable image file.
    let candidates: Vec<&crate::ir::source::VisualAsset> = source_ir
        .visual_assets
        .iter()
        .filter(|a| {
            a.page_id.as_deref().and_then(page_number) == Some(page)
                && a.image_path.as_deref().is_some_and(Path::exists)
        })
        .collect();

    // Most specific: a caption that names this register.
    let register_name_lower = register.register_name.to_ascii_lowercase();
    if let Some(asset) = candidates.iter().find(|a| {
        a.caption_text
            .as_deref()
            .map(|c| c.to_ascii_lowercase().contains(&register_name_lower))
            .unwrap_or(false)
    }) {
        return Some((
            asset.asset_id.clone(),
            asset.image_path.clone().expect("image_path filtered above"),
        ));
    }

    // Next: the unique register-bitfield diagram on the page (ambiguity → residual).
    let mut bitfields = candidates
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::RegisterBitfield));
    if let Some(first) = bitfields.next() {
        if bitfields.next().is_some() {
            return None; // more than one RegisterBitfield — do not guess
        }
        return Some((
            first.asset_id.clone(),
            first.image_path.clone().expect("image_path filtered above"),
        ));
    }

    // EXTRACTION-GAP-FIX.4d — gap #1: register bit-layout diagrams are routinely left
    // `diagram_kind=unknown` by the ingest classifier, so the `RegisterBitfield`-only fallback above
    // misses them. When the register's page carries EXACTLY ONE diagram image, it is unambiguously
    // that register's layout, so use it regardless of `diagram_kind`. This never fabricates: the
    // downstream tiling-width gate (a) and the field-name multiset gate (b) reject any wrong read,
    // so a mis-resolved figure simply produces an honest residual. More than one image on the page
    // stays a residual — we never guess which.
    if let [only] = candidates.as_slice() {
        return Some((
            only.asset_id.clone(),
            only.image_path.clone().expect("image_path filtered above"),
        ));
    }
    None
}

/// Ask the diagram reader for the register's fields in MSB→LSB order. Honors the
/// `SPECFORGE_VLM_HELPER` hook (so tests are hermetic); otherwise calls the shared VLM image
/// transport. The reply is parsed leniently; the gated core decides what to trust.
fn propose_register_diagram_fields(
    asset_id: &str,
    image_path: &Path,
    model: &str,
    api_url: &str,
    provider: VlmProviderArg,
) -> Result<Vec<RegisterDiagramFieldProposal>> {
    let reply = if let Some(helper_path) = std::env::var_os(VLM_HELPER_ENV) {
        let mut command = Command::new(&helper_path);
        crate::project_data::configure_command(&mut command)?;
        let output = command
            .arg("--asset-id")
            .arg(asset_id)
            .arg("--diagram-type")
            .arg("register_bitfield")
            .output()?;
        if !output.status.success() {
            return Err(AppError::ExternalCommandFailed {
                program: helper_path.display().to_string(),
                exit_code: output.status.code(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        let prompt = build_register_diagram_prompt();
        crate::commands::enrich::vlm_image_query(image_path, &prompt, model, api_url, provider)?
    };
    Ok(parse_diagram_field_proposals(&reply))
}

/// The diagram-reading prompt. Asks ONLY for what the VLM reads reliably — field names, their
/// order, and per-field widths — never absolute bit positions (those are reconstructed by tiling).
fn build_register_diagram_prompt() -> String {
    "This is a register bit-field layout diagram from a chip-specification PDF. Read the named \
fields from left (most-significant bit) to right (least-significant bit). For EACH field cell, \
report its field name and its WIDTH in bits (how many bit positions the cell spans). Include \
reserved/unnamed cells too, naming them \"reserved\". Do NOT report absolute bit positions. Reply \
with STRICT JSON only, no prose: {\"fields\": [{\"name\": \"<field>\", \"width\": <bits>}, ...]} in \
most-significant-first order."
        .to_string()
}

/// Lenient parse of the diagram reader's reply into `(name, width)` proposals, in reply order.
/// Tolerates ```json fences and surrounding prose by slicing from the first `{` to the last `}`.
/// A width that cannot be read becomes `0`, which the tiling gate rejects (honest residual) rather
/// than guessing.
fn parse_diagram_field_proposals(content: &str) -> Vec<RegisterDiagramFieldProposal> {
    let Some(start) = content.find('{') else {
        return Vec::new();
    };
    let Some(end) = content.rfind('}') else {
        return Vec::new();
    };
    if end < start {
        return Vec::new();
    }
    let json = &content[start..=end];

    #[derive(serde::Deserialize)]
    struct DiagramReply {
        #[serde(default)]
        fields: Vec<DiagramFieldReply>,
    }
    #[derive(serde::Deserialize)]
    struct DiagramFieldReply {
        #[serde(default)]
        name: String,
        #[serde(default)]
        width: serde_json::Value,
    }

    let Ok(reply) = serde_json::from_str::<DiagramReply>(json) else {
        return Vec::new();
    };
    reply
        .fields
        .into_iter()
        .filter(|f| !f.name.trim().is_empty())
        .map(|f| RegisterDiagramFieldProposal {
            field_name: f.name.trim().to_string(),
            width: coerce_width(&f.width),
        })
        .collect()
}

/// Coerce a JSON width (number or numeric string like `"10"` / `"10 bits"`) to bits; `0` on failure.
fn coerce_width(value: &serde_json::Value) -> u32 {
    match value {
        serde_json::Value::Number(n) => n
            .as_u64()
            .map(|v| v.min(u64::from(u32::MAX)) as u32)
            .unwrap_or(0),
        serde_json::Value::String(s) => s
            .trim()
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect::<String>()
            .parse()
            .unwrap_or(0),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;
    use crate::ir::source::{
        AutomationConfidence, RegisterFieldRecord, SourceIr, StructuredTableRecord, VisualAsset,
        VisualAssetKind,
    };
    use crate::test_support::env_var_lock;

    const DMCONTROL_REPLY: &str = "{\"fields\":[\
{\"name\":\"haltreq\",\"width\":1},{\"name\":\"resumereq\",\"width\":1},\
{\"name\":\"hartreset\",\"width\":1},{\"name\":\"ackhavereset\",\"width\":1},\
{\"name\":\"ackunavail\",\"width\":1},{\"name\":\"hasel\",\"width\":1},\
{\"name\":\"hartsello\",\"width\":10},{\"name\":\"hartselhi\",\"width\":10},\
{\"name\":\"setkeepalive\",\"width\":1},{\"name\":\"clrkeepalive\",\"width\":1},\
{\"name\":\"setresethaltreq\",\"width\":1},{\"name\":\"clrresethaltreq\",\"width\":1},\
{\"name\":\"ndmreset\",\"width\":1},{\"name\":\"dmactive\",\"width\":1}]}";

    fn named_field(name: &str) -> RegisterFieldRecord {
        RegisterFieldRecord {
            field_name: name.to_string(),
            bits_high: None,
            bits_low: None,
            bit_width: None,
            access_type: None,
            reset_value: None,
            description: None,
            enumerated_values: Vec::new(),
        }
    }

    fn dmcontrol_field_names() -> Vec<&'static str> {
        vec![
            "dmactive",
            "ndmreset",
            "clrresethaltreq",
            "setresethaltreq",
            "clrkeepalive",
            "setkeepalive",
            "hartselhi",
            "hartsello",
            "hasel",
            "ackunavail",
            "ackhavereset",
            "hartreset",
            "resumereq",
            "haltreq",
        ]
    }

    /// Write a mock VLM helper that echoes a fixed reply regardless of args (the command always
    /// passes `--asset-id`/`--diagram-type`). Mirrors the `nlp-enrich` test helper convention.
    fn write_mock_helper(dir: &Path, reply: &str) -> PathBuf {
        let script_path = dir.join("mock_diagram_helper.sh");
        let script = format!("#!/bin/bash\ncat <<'JSON'\n{reply}\nJSON\n");
        fs::write(&script_path, script).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755)).unwrap();
        }
        script_path
    }

    #[test]
    fn parses_fenced_diagram_reply() {
        let fenced = format!("Here are the fields:\n```json\n{DMCONTROL_REPLY}\n```\nDone.");
        let proposals = parse_diagram_field_proposals(&fenced);
        assert_eq!(proposals.len(), 14);
        assert_eq!(proposals[0].field_name, "haltreq");
        assert_eq!(proposals[6].field_name, "hartsello");
        assert_eq!(proposals[6].width, 10);
        assert_eq!(proposals.iter().map(|p| p.width).sum::<u32>(), 32);
    }

    #[test]
    fn parses_string_widths_and_skips_blank_names() {
        let reply = "{\"fields\":[{\"name\":\"a\",\"width\":\"8 bits\"},{\"name\":\"  \",\"width\":4},{\"name\":\"b\",\"width\":\"24\"}]}";
        let proposals = parse_diagram_field_proposals(reply);
        assert_eq!(proposals.len(), 2);
        assert_eq!(proposals[0].width, 8);
        assert_eq!(proposals[1].width, 24);
    }

    #[test]
    fn parse_tolerates_garbage() {
        assert!(parse_diagram_field_proposals("not json at all").is_empty());
        assert!(parse_diagram_field_proposals("").is_empty());
    }

    #[test]
    fn proposer_reads_through_helper_hook() {
        let _lock = env_var_lock();
        let tempdir = tempdir().unwrap();
        let helper = write_mock_helper(tempdir.path(), DMCONTROL_REPLY);
        let image = tempdir.path().join("diagram.png");
        fs::write(&image, b"not-a-real-png").unwrap();

        unsafe { std::env::set_var(VLM_HELPER_ENV, &helper) };
        let proposals = propose_register_diagram_fields(
            "picture_0020",
            &image,
            "qwen2.5vl:7b",
            "http://localhost:11434/v1/chat/completions",
            VlmProviderArg::Ollama,
        )
        .unwrap();
        unsafe { std::env::remove_var(VLM_HELPER_ENV) };

        assert_eq!(proposals.len(), 14);
        assert_eq!(proposals.iter().map(|p| p.width).sum::<u32>(), 32);
    }

    /// Build an on-disk EvidenceIR (+ sibling SourceIR) carrying a `dmcontrol` field-table register
    /// with empty bits, the source field table on page 5, and a `RegisterBitfield` diagram image on
    /// the same page. Returns `(tempdir, evidence_ir_path)`.
    fn build_fixture(field_names: &[&str], register_name: &str) -> (tempfile::TempDir, PathBuf) {
        let tempdir = tempdir().unwrap();
        let source = tempdir.path().join("spec.md");
        fs::write(&source, "# Spec\nRegister definitions.\n").unwrap();
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        let mut source_ir = SourceIr::build(&source, &source_artifact_base).unwrap();
        // Inject the register's source field table (page 5) and its bit-layout diagram image.
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "T1".to_string(),
            asset_id: "asset_table".to_string(),
            page_id: Some("page_0005".to_string()),
            caption_text: None,
            source_ref: None,
            table_kind: crate::ir::source::TableKind::Unknown,
            header_rows: Vec::new(),
            body_rows: Vec::new(),
            row_count: 0,
            col_count: 0,
        });
        let image_path = tempdir.path().join("picture_0005.png");
        fs::write(&image_path, b"not-a-real-png").unwrap();
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0005".to_string(),
            asset_kind: VisualAssetKind::Figure,
            page_id: Some("page_0005".to_string()),
            image_path: Some(image_path),
            caption_text: None,
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: None,
            diagram_kind: DiagramKind::RegisterBitfield,
        });
        source_ir.write_to_disk().unwrap();

        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )
        .unwrap();
        evidence_ir.register_records.push(RegisterRecord {
            register_id: "regfld_T1".to_string(),
            register_name: register_name.to_string(),
            access_type: None,
            offset_address: None,
            size_bits: None,
            fields: field_names.iter().map(|n| named_field(n)).collect(),
            supporting_table_ids: vec!["T1".to_string()],
            supporting_statement_ids: Vec::new(),
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk().unwrap();
        let evidence_ir_path = evidence_ir.artifact_layout.evidence_ir_path.clone();
        (tempdir, evidence_ir_path)
    }

    /// Build a SourceIR with the register's source field table on page 5 and `image_count` diagram
    /// images on that page, each of `kind`. Returns the SourceIR and a matching field-table register
    /// (`regfld_T1` → table `T1`). Used to exercise `resolve_diagram_image_for_register` directly.
    fn source_with_page_images(
        kind: DiagramKind,
        image_count: usize,
    ) -> (tempfile::TempDir, SourceIr, RegisterRecord) {
        let tempdir = tempdir().unwrap();
        let source = tempdir.path().join("spec.md");
        fs::write(&source, "# Spec\n").unwrap();
        let base = tempdir.path().join("generated").join("source_ir");
        let mut source_ir = SourceIr::build(&source, &base).unwrap();
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "T1".to_string(),
            asset_id: "asset_table".to_string(),
            page_id: Some("page_0005".to_string()),
            caption_text: None,
            source_ref: None,
            table_kind: crate::ir::source::TableKind::Unknown,
            header_rows: Vec::new(),
            body_rows: Vec::new(),
            row_count: 0,
            col_count: 0,
        });
        for i in 0..image_count {
            let img = tempdir.path().join(format!("pic_{i}.png"));
            fs::write(&img, b"x").unwrap();
            source_ir.visual_assets.push(VisualAsset {
                asset_id: format!("pic_{i}"),
                asset_kind: VisualAssetKind::Figure,
                page_id: Some("page_0005".to_string()),
                image_path: Some(img),
                caption_text: None,
                caption_source_path: None,
                source_ref: None,
                placeholder_text: None,
                note: None,
                diagram_kind: kind,
            });
        }
        let register = RegisterRecord {
            register_id: "regfld_T1".to_string(),
            register_name: "dmcontrol".to_string(),
            access_type: None,
            offset_address: None,
            size_bits: None,
            fields: vec![named_field("version")],
            supporting_table_ids: vec!["T1".to_string()],
            supporting_statement_ids: Vec::new(),
            automation_confidence: AutomationConfidence::Medium,
        };
        (tempdir, source_ir, register)
    }

    #[test]
    fn resolves_a_single_unknown_diagram_on_the_page() {
        // EXTRACTION-GAP-FIX.4d — a register page with exactly ONE diagram image left `unknown` by the
        // ingest classifier resolves to it, so the bit reader is not blocked by mis-classification.
        let (_t, source_ir, register) = source_with_page_images(DiagramKind::Unknown, 1);
        let resolved = resolve_diagram_image_for_register(&register, &source_ir);
        assert_eq!(
            resolved.map(|(id, _)| id),
            Some("pic_0".to_string()),
            "a single unknown-kind page image must resolve"
        );
    }

    #[test]
    fn two_unknown_images_on_the_page_stay_a_residual() {
        // Never guess among several: two images on the page → honest residual, not a guess.
        let (_t, source_ir, register) = source_with_page_images(DiagramKind::Unknown, 2);
        assert!(resolve_diagram_image_for_register(&register, &source_ir).is_none());
    }

    #[test]
    fn command_recovers_dmcontrol_bits_end_to_end() {
        let _lock = env_var_lock();
        let (tempdir, evidence_ir_path) = build_fixture(&dmcontrol_field_names(), "dmcontrol");
        let helper = write_mock_helper(tempdir.path(), DMCONTROL_REPLY);

        unsafe { std::env::set_var(VLM_HELPER_ENV, &helper) };
        run(RecoverRegisterBitsArgs {
            evidence_ir: evidence_ir_path.clone(),
            vlm_provider: VlmProviderArg::Ollama,
            vlm_model: None,
            dry_run: false,
        })
        .unwrap();
        unsafe { std::env::remove_var(VLM_HELPER_ENV) };

        let reloaded = EvidenceIr::load_from_path(&evidence_ir_path).unwrap();
        let reg = &reloaded.register_records[0];
        assert!(reg.fields.iter().all(|f| f.bits_high.is_some()));
        let by_name: std::collections::BTreeMap<&str, &RegisterFieldRecord> = reg
            .fields
            .iter()
            .map(|f| (f.field_name.as_str(), f))
            .collect();
        assert_eq!(
            (
                by_name["hartsello"].bits_high,
                by_name["hartsello"].bits_low
            ),
            (Some(25), Some(16))
        );
        assert_eq!(
            (
                by_name["hartselhi"].bits_high,
                by_name["hartselhi"].bits_low
            ),
            (Some(15), Some(6))
        );
        assert_eq!(reg.size_bits, Some(32));
    }

    #[test]
    fn command_leaves_residual_on_name_mismatch_end_to_end() {
        let _lock = env_var_lock();
        // The field table names a field the diagram never reports → gate (b) rejects → no bits.
        let mut names = dmcontrol_field_names();
        names[0] = "totallydifferent";
        let (tempdir, evidence_ir_path) = build_fixture(&names, "dmcontrol");
        let helper = write_mock_helper(tempdir.path(), DMCONTROL_REPLY);

        unsafe { std::env::set_var(VLM_HELPER_ENV, &helper) };
        run(RecoverRegisterBitsArgs {
            evidence_ir: evidence_ir_path.clone(),
            vlm_provider: VlmProviderArg::Ollama,
            vlm_model: None,
            dry_run: false,
        })
        .unwrap();
        unsafe { std::env::remove_var(VLM_HELPER_ENV) };

        let reloaded = EvidenceIr::load_from_path(&evidence_ir_path).unwrap();
        let reg = &reloaded.register_records[0];
        assert!(
            reg.fields
                .iter()
                .all(|f| f.bits_high.is_none() && f.bits_low.is_none()),
            "no bits may be fabricated when names disagree"
        );
    }

    #[test]
    fn skip_provider_is_a_noop() {
        let _lock = env_var_lock();
        let (_tempdir, evidence_ir_path) = build_fixture(&dmcontrol_field_names(), "dmcontrol");
        run(RecoverRegisterBitsArgs {
            evidence_ir: evidence_ir_path.clone(),
            vlm_provider: VlmProviderArg::Skip,
            vlm_model: None,
            dry_run: false,
        })
        .unwrap();
        let reloaded = EvidenceIr::load_from_path(&evidence_ir_path).unwrap();
        assert!(
            reloaded.register_records[0]
                .fields
                .iter()
                .all(|f| f.bits_high.is_none())
        );
    }

    #[test]
    fn skip_provider_accepts_a_legacy_repository_artifact_path() {
        let _lock = env_var_lock();
        let (tempdir, evidence_ir_path) = build_fixture(&dmcontrol_field_names(), "dmcontrol");
        let argument_path = tempdir.path().join("legacy_argument/evidence_ir.json");
        fs::create_dir_all(argument_path.parent().expect("legacy argument parent")).unwrap();
        fs::copy(evidence_ir_path, &argument_path).unwrap();
        let repository = crate::project_data::repository_root().unwrap();
        let relative = argument_path
            .strip_prefix(repository)
            .expect("fixture is repository-local");
        let legacy = Path::new("/retired/specforge").join(relative);

        run(RecoverRegisterBitsArgs {
            evidence_ir: legacy,
            vlm_provider: VlmProviderArg::Skip,
            vlm_model: None,
            dry_run: false,
        })
        .unwrap();
    }
}
