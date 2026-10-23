//! The `PROOF-SEAL-CURRENCY` doctrine (`SOURCE-IR-REPRODUCIBILITY.16`), native since `RUSTDOC-AND-XTASK.14.e`: every
//! persisted artifact in the proof-carrying stratum records a ruleset seal the current build's canonical loader
//! still accepts, so the seal debt an ordinary edit to a stage root creates is visible the moment it is created. The
//! census is total; the probe is per stage (`CORPUS-CHAIN-CURRENCY.6`): every document at `semantic` and `intent`,
//! one per distinct seal at the extraction-replaying `source-ir` and `evidence` stages, which the check says in its
//! own output, and every document everywhere under `--total` (CI tier). It asks the product's own loader through the
//! consuming stage's `--dry-run`, never `specforge validate`, which mutates the artifact it reads. It does not
//! prove content currency; that is `CHAIN-CURRENCY`'s.
//!
//! The seal read is defined here once (`.11`, `SOURCE-IR-REPRODUCIBILITY.16`): `scripts/lib/proof_seal_scan.sh`, which
//! `scripts/rebuild_stage_cascade.sh` sources, calls `--proof-ledger-candidates`, `--scan-proof-ledger` and
//! `--sealed-artifacts`. The chain stage table and the read-only probe stay defined in that library, and this check
//! reads them from it; which build answers stays `scripts/lib/corpus_replay_binary.sh`.
//!
//! `scripts/check_proof_seal_currency.sh`, the registered script, is a shim that keeps its argument loop, its knobs
//! (`SPECFORGE_PROOF_SEAL_GENERATED_ROOT`, `SPECFORGE_PROOF_SEAL_BIN`, `SPECFORGE_PROOF_SEAL_TOTAL_STAGES`) and the
//! activation.

use std::collections::HashSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::project_data::{self, Environment};

/// The cases the self-test runs; `scripts/report_self_test_totals.pl` reads this declaration.
const EXPECTED_SELF_TEST_CASES: usize = 21;
const DEFAULT_GENERATED_ROOT: &str = "generated";
/// The stages probed document by document unless the knob says otherwise (`CORPUS-CHAIN-CURRENCY.9`).
const DEFAULT_TOTAL_STAGES: &str = "semantic intent";
const LEDGER_KEY: &[u8] = b"\"proof_ledger\"";
/// The diagnostic the product emits for a ledger proved under a ruleset the current build no longer issues.
const STALE_SEAL: &str = "proof ledger ruleset hash is stale";
/// The diagnostic a probe emits when its own input is missing: no verdict, never a pass or a breach.
const ABSENT_PRECONDITION: &str = "path does not exist";
const LIBRARY: &str = "scripts/lib/proof_seal_scan.sh";

fn note(message: &str) {
    println!("[proof-seal] {message}");
}

fn fail_note(message: &str) {
    eprintln!("[proof-seal] FAIL: {message}");
}

/// `--check`, `--total` or `--self-test`, as the shim passes the mode its argument loop chose; or one of the shared
/// seal readers, as `scripts/lib/proof_seal_scan.sh` calls them.
pub fn run(repository: &Path, arguments: &[String]) -> i32 {
    let mut arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    // The library passes the directory it was called from, so relative paths resolve, and print, as given.
    if let ["--caller-dir", directory, ..] = arguments.as_slice() {
        if let Err(error) = std::env::set_current_dir(directory) {
            eprintln!("[proof-seal] cannot return to the calling directory {directory}: {error}");
            return 2;
        }
        arguments.drain(..2);
    }
    match arguments.as_slice() {
        [] | ["--check"] => gate(repository, false),
        ["--total"] => gate(repository, true),
        ["--self-test"] => self_test(repository),
        ["--proof-ledger-candidates", paths @ ..] => {
            for path in proof_ledger_candidates(paths) {
                println!("{path}");
            }
            0
        }
        ["--scan-proof-ledger", path] => match scan_proof_ledger(Path::new(path)) {
            Some(seal) => {
                println!("{}", seal.unwrap_or_else(|| "none".to_string()));
                0
            }
            None => 1,
        },
        ["--sealed-artifacts", paths @ ..] => {
            for (path, seal) in sealed_artifacts(paths) {
                println!("{path}\t{seal}");
            }
            0
        }
        _ => {
            eprintln!(
                "Usage: cargo xtask check proof-seal-currency [--check|--total|--self-test] | --proof-ledger-candidates \
                 PATH... | --scan-proof-ledger PATH | --sealed-artifacts PATH..."
            );
            2
        }
    }
}

// The seal read.

/// The inputs, in argument order, whose bytes contain the `"proof_ledger"` key: a necessary-condition prefilter,
/// never an authority. An unreadable or absent input is not a candidate.
fn proof_ledger_candidates(paths: &[&str]) -> Vec<String> {
    paths
        .iter()
        .filter(|path| contains_ledger_key(Path::new(path)))
        .map(|path| (*path).to_string())
        .collect()
}

/// Read in blocks with the needle's length less one carried over, so a needle cannot straddle a read boundary.
fn contains_ledger_key(path: &Path) -> bool {
    let Ok(mut file) = fs::File::open(path) else {
        return false;
    };
    let keep = LEDGER_KEY.len() - 1;
    let mut window: Vec<u8> = Vec::new();
    let mut block = vec![0u8; 1 << 20];
    loop {
        let read = match file.read(&mut block) {
            Ok(0) | Err(_) => return false,
            Ok(read) => read,
        };
        window.extend_from_slice(&block[..read]);
        if window
            .windows(LEDGER_KEY.len())
            .any(|candidate| candidate == LEDGER_KEY)
        {
            return true;
        }
        let tail = window.len().saturating_sub(keep);
        window.drain(..tail);
    }
}

/// The seal a top-level `proof_ledger` object records as its direct `ruleset_sha256` member: `Some(Some(seal))`,
/// `Some(None)` when the ledger records none, or `None` when there is no top-level ledger (or the file cannot be
/// read). A streaming, depth-aware scan, so a digest-shaped field elsewhere cannot impersonate the seal; string
/// literals are taken raw, escapes and all, and a literal left unterminated at the end of the file ends the scan.
fn scan_proof_ledger(path: &Path) -> Option<Option<String>> {
    let bytes = fs::read(path).ok()?;
    // Signed, as Perl's counter is, so a stray closer in a malformed file moves the depth the same way.
    let mut depth = 0isize;
    let mut pending_key: Option<&[u8]> = None;
    let mut in_ledger = 0isize;
    let mut found_ledger = false;
    let mut expect_value = false;
    let mut seal: Option<&[u8]> = None;
    let mut at = 0;
    let structural = |byte: u8| matches!(byte, b'"' | b'{' | b'}' | b'[' | b']' | b',' | b':');
    'scan: loop {
        while at < bytes.len() && !structural(bytes[at]) {
            at += 1;
        }
        let Some(&byte) = bytes.get(at) else {
            break;
        };
        if byte == b'"' {
            let start = at + 1;
            let mut end = start;
            loop {
                match bytes.get(end) {
                    None => break 'scan,
                    Some(b'"') => break,
                    Some(b'\\') => {
                        if end + 1 >= bytes.len() {
                            break 'scan;
                        }
                        end += 2;
                    }
                    Some(_) => end += 1,
                }
            }
            let literal = &bytes[start..end];
            at = end + 1;
            if expect_value {
                seal = Some(literal);
                break;
            }
            pending_key = Some(literal);
            continue;
        }
        at += 1;
        match byte {
            b'{' | b'[' => {
                depth += 1;
                if !found_ledger
                    && byte == b'{'
                    && depth == 2
                    && pending_key == Some(b"proof_ledger")
                {
                    found_ledger = true;
                    in_ledger = depth;
                }
                pending_key = None;
            }
            b'}' | b']' => {
                if in_ledger != 0 && depth == in_ledger {
                    break;
                }
                depth -= 1;
                pending_key = None;
            }
            b':' => {
                expect_value =
                    in_ledger != 0 && depth == in_ledger && pending_key == Some(b"ruleset_sha256");
            }
            _ => pending_key = None,
        }
    }
    found_ledger.then(|| seal.map(|seal| String::from_utf8_lossy(seal).into_owned()))
}

/// The composition of the two readers, and the only place they are composed: `(path, seal)` for each input that
/// carries a top-level ledger, in argument order, `none` for a ledger without a seal. An empty seal is no row, as
/// the script's `[ -n "$seal" ]` dropped it.
fn sealed_artifacts(paths: &[&str]) -> Vec<(String, String)> {
    proof_ledger_candidates(paths)
        .into_iter()
        .filter_map(|path| {
            let seal = scan_proof_ledger(Path::new(&path))?.unwrap_or_else(|| "none".to_string());
            // `seal="$(scan_proof_ledger …)" && [ -n "$seal" ]`: trailing newlines stripped, an empty seal dropped.
            let seal = seal.trim_end_matches('\n').to_string();
            (!seal.is_empty()).then_some((path, seal))
        })
        .collect()
}

// The shared library: the stage table and the read-only probe stay defined there.

/// One chain stage as `scripts/lib/proof_seal_scan.sh` defines it: its id and its artifact path with `@KEY@` for
/// the document key, resolved against the corpus root.
struct Stage {
    id: String,
    path: String,
}

/// A command with the activated environment and the script's `LC_ALL=C`.
fn command(
    program: impl AsRef<std::ffi::OsStr>,
    root: &Path,
    environment: &Environment,
) -> Command {
    let mut command = Command::new(program);
    command.current_dir(root).env("LC_ALL", "C");
    for (name, value) in environment {
        command.env(name, value);
    }
    command
}

/// The chain stage table, read from the library in its order.
fn stage_table(
    root: &Path,
    environment: &Environment,
    generated: &str,
) -> Result<Vec<Stage>, String> {
    let output = command("bash", root, environment)
        .args([
            "-c",
            ". \"$1\"; for stage in $(chain_stages); do printf '%s\\t%s\\n' \"$stage\" \"$(chain_stage_artifact_path \
             \"$stage\" @KEY@)\"; done",
            "stage-table",
            LIBRARY,
        ])
        .env("GENERATED_ROOT", generated)
        .output()
        .map_err(|error| format!("cannot read the stage table from {LIBRARY}: {error}"))?;
    if !output.status.success() {
        return Err(format!("cannot read the stage table from {LIBRARY}"));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .map(|(id, path)| Stage {
            id: id.to_string(),
            path: path.to_string(),
        })
        .collect())
}

/// The library's read-only canonical probe: 0 accepted, 1 refused (its diagnostic in `error`), 2 a terminal stage
/// with no probe, 3 no probe resolvable.
fn readonly_probe(
    root: &Path,
    environment: &Environment,
    binary: &str,
    stage: &str,
    artifact: &str,
    error: &Path,
) -> i32 {
    command("bash", root, environment)
        .args([
            "-c",
            ". \"$1\"; chain_stage_readonly_probe \"$2\" \"$3\" \"$4\"",
            "readonly-probe",
            LIBRARY,
            stage,
            artifact,
        ])
        .arg(error)
        .env("BIN", binary)
        .stdin(Stdio::inherit())
        .status()
        .ok()
        .and_then(|status| status.code())
        .unwrap_or(3)
}

/// Ask `scripts/lib/corpus_replay_binary.sh` for the build that answers.
fn corpus_replay_build(root: &Path, environment: &Environment, log: &Path) -> Option<String> {
    let output = command("bash", root, environment)
        .args([
            "-c",
            "ROOT=\"$1\"; . \"$ROOT/scripts/lib/corpus_replay_binary.sh\"; corpus_replay_build \"$2\"",
            "corpus-replay-build",
        ])
        .arg(root)
        .arg(log)
        .output()
        .ok()?;
    output.status.success().then(|| {
        String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_string()
    })
}

// The gate.

/// The task-owned command that re-earns a stale seal at this stage.
fn remedy_for(stage: &str) -> &'static str {
    if stage == "source-ir" {
        "cargo run --manifest-path Cargo.toml --features source-proof-migration --example source_proof_migrate -- \
         --write --retained-manifest (proof-only re-seal; SOURCE-IR-REPRODUCIBILITY.14)"
    } else {
        "bash scripts/rebuild_stage_cascade.sh --check, then --write (stage rebuild; SOURCE-IR-REPRODUCIBILITY.15) — \
         re-sealing SourceIR moves every downstream cumulative digest, so run it upstream-first"
    }
}

/// Whether a stage is probed document by document: always under `--total`, otherwise when the stage set names it
/// (`case " $TOTAL_PROBE_STAGES " in *" $stage "*)`).
fn probes_totally(total: bool, total_stages: &str, stage: &str) -> bool {
    total || format!(" {total_stages} ").contains(&format!(" {stage} "))
}

/// `tr '\n' ' ' | sed 's/  */ /g' | cut -c1-200` under `LC_ALL=C`: one line, runs of spaces squeezed, 200 bytes.
fn diagnostic(error: &[u8]) -> String {
    let mut out: Vec<u8> = Vec::new();
    for byte in error
        .iter()
        .map(|byte| if *byte == b'\n' { b' ' } else { *byte })
    {
        if byte == b' ' && out.last() == Some(&b' ') {
            continue;
        }
        out.push(byte);
    }
    out.truncate(200);
    String::from_utf8_lossy(&out).into_owned()
}

fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path).is_ok_and(|metadata| metadata.permissions().mode() & 0o111 != 0)
}

/// Removes the gate's workspace however it returns, as the script's `trap … EXIT` did.
struct Scratch(PathBuf);

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn scratch(environment: &Environment, kind: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    project_data::path(environment, "TMPDIR")
        .join(format!("proof-seal-{kind}.{}.{nanos}", std::process::id()))
}

/// The persisted SourceIRs, in the order the script's glob gave them: sorted by whole path.
fn source_irs(generated: &str) -> Vec<String> {
    let Ok(entries) = fs::read_dir(format!("{generated}/source_ir")) else {
        return Vec::new();
    };
    let mut paths: Vec<String> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|key| !key.starts_with('.'))
        .map(|key| format!("{generated}/source_ir/{key}/source_ir.json"))
        .filter(|path| Path::new(path).is_file())
        .collect();
    paths.sort();
    paths
}

fn document_key(path: &str) -> String {
    Path::new(path)
        .parent()
        .and_then(Path::file_name)
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn gate(root: &Path, total: bool) -> i32 {
    let environment = match project_data::activate(root) {
        Ok(environment) => environment,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if let Err(error) = std::env::set_current_dir(root) {
        fail_note(&format!("cannot enter {}: {error}", root.display()));
        return 1;
    }
    let generated = std::env::var("SPECFORGE_PROOF_SEAL_GENERATED_ROOT")
        .unwrap_or_else(|_| DEFAULT_GENERATED_ROOT.to_string());
    // `${SPECFORGE_PROOF_SEAL_TOTAL_STAGES-semantic intent}`: the default only when unset, so an explicit empty
    // value turns the per-stage total tier off for one run.
    let total_stages = std::env::var("SPECFORGE_PROOF_SEAL_TOTAL_STAGES")
        .unwrap_or_else(|_| DEFAULT_TOTAL_STAGES.to_string());
    if !Path::new(&format!("{generated}/source_ir")).is_dir() {
        note(&format!(
            "SKIP: no corpus at {generated}/source_ir — `generated/` is untracked, so a fresh clone"
        ));
        note("SKIP: and a hosted CI runner have no persisted seal to check. Nothing is claimed.");
        return 0;
    }
    let work = scratch(&environment, "work");
    if fs::create_dir_all(&work).is_err() {
        fail_note("cannot create a repository-local workspace");
        return 1;
    }
    let _cleanup = Scratch(work.clone());
    let sources = source_irs(&generated);
    let source_refs: Vec<&str> = sources.iter().map(String::as_str).collect();
    let keys: Vec<String> = sealed_artifacts(&source_refs)
        .iter()
        .map(|(path, _)| document_key(path))
        .collect();
    let in_scope = keys.len();
    note(&format!(
        "corpus root: {generated} — {in_scope} proof-carrying document(s) in scope, {} legacy proofless document(s) \
         explicitly out of scope",
        sources.len() - in_scope
    ));
    if in_scope == 0 {
        note("SKIP: no persisted document carries a current seal, so there is none to be stale.");
        return 0;
    }
    let binary = match std::env::var("SPECFORGE_PROOF_SEAL_BIN") {
        Ok(binary) if !binary.is_empty() => binary,
        _ => {
            let log = work.join("build.log");
            match corpus_replay_build(root, &environment, &log) {
                Some(binary) => binary,
                None => {
                    fail_note(
                        "cargo could not build the specforge binary, so the persisted seal cannot be asked of",
                    );
                    fail_note(
                        "the current build. Fix the build first; this doctrine has no verdict without it.",
                    );
                    eprint!(
                        "{}",
                        String::from_utf8_lossy(&fs::read(&log).unwrap_or_default())
                    );
                    return 1;
                }
            }
        }
    };
    if !is_executable(Path::new(&binary)) {
        fail_note(&format!("{binary} is not an executable specforge binary"));
        return 1;
    }
    let stages = match stage_table(root, &environment, &generated) {
        Ok(stages) => stages,
        Err(error) => {
            fail_note(&error);
            return 1;
        }
    };
    let mut fail = false;
    let probe_error = work.join("probe.err");
    for stage in &stages {
        let artifacts: Vec<String> = keys
            .iter()
            .map(|key| stage.path.replace("@KEY@", key))
            .filter(|artifact| Path::new(artifact).is_file())
            .collect();
        let present = artifacts.len();
        let id = stage.id.as_str();
        if present == 0 {
            note(&format!(
                "{id} — 0/{in_scope} persisted; the chain does not reach this stage, so there is no seal to check"
            ));
            continue;
        }
        let artifact_refs: Vec<&str> = artifacts.iter().map(String::as_str).collect();
        let seals = sealed_artifacts(&artifact_refs);
        let sealed = seals.len();
        if sealed != present {
            fail_note(&format!(
                "{id} — {} of {present} persisted artifact(s) carry NO proof ledger while",
                present - sealed
            ));
            fail_note(&format!(
                "{id} — their SourceIR does. A stage that lost its seal cannot be verified; rebuild it:"
            ));
            fail_note(&format!("{id} — {}", remedy_for(id)));
            fail = true;
            continue;
        }
        let mut seen = HashSet::new();
        let first_per_seal: Vec<&(String, String)> = seals
            .iter()
            .filter(|(_, seal)| seen.insert(seal.clone()))
            .collect();
        let distinct = first_per_seal.len();
        let stage_total = probes_totally(total, &total_stages, id);
        let representatives: Vec<&(String, String)> = if stage_total {
            seals.iter().collect()
        } else {
            first_per_seal
        };
        let probes = representatives.len();
        let summary =
            format!("{id} — {sealed}/{in_scope} persisted and sealed, {distinct} distinct seal(s)");
        let (mut accepted, mut rejected, mut unprobed, mut precondition_absent) = (0, 0, 0, 0);
        for (representative, seal) in representatives {
            let _ = fs::write(&probe_error, b"");
            match readonly_probe(
                root,
                &environment,
                &binary,
                id,
                representative,
                &probe_error,
            ) {
                0 => accepted += 1,
                2 => unprobed += 1,
                3 => {
                    fail_note(&format!(
                        "{id} — no read-only probe could be resolved for this stage; the check has no verdict"
                    ));
                    fail = true;
                }
                _ => {
                    let diagnostic = diagnostic(&fs::read(&probe_error).unwrap_or_default());
                    if diagnostic.contains(ABSENT_PRECONDITION) {
                        precondition_absent += 1;
                        note(&format!(
                            "{id} — NO VERDICT for {representative}: the probe's own input is absent"
                        ));
                        note(&format!(
                            "{id} — ({diagnostic}). Censused and sealed, but unprobeable here."
                        ));
                        continue;
                    }
                    rejected += 1;
                    let short: String =
                        String::from_utf8_lossy(&seal.as_bytes()[..seal.len().min(8)]).into_owned();
                    fail_note(&format!(
                        "{id} — the current build REFUSES the persisted seal {short}… carried by"
                    ));
                    fail_note(&format!("{id} — {representative}"));
                    fail_note(&format!("{id} — loader: {diagnostic}"));
                    if diagnostic.contains(STALE_SEAL) {
                        fail_note(&format!(
                            "{id} — this is a STALE SEAL, not a content problem: the ruleset digest is over the"
                        ));
                        fail_note(&format!(
                            "{id} — implementation, so an ordinary edit to a stage root un-seals every persisted"
                        ));
                        fail_note(&format!(
                            "{id} — artifact while its content stays exactly right. Remedy (task-owned):"
                        ));
                        fail_note(&format!("{id} —   {}", remedy_for(id)));
                    } else {
                        fail_note(&format!(
                            "{id} — this is NOT the stale-seal diagnostic, so the remedy above may not apply."
                        ));
                        fail_note(&format!(
                            "{id} — Do not re-seal past it; find out why the loader refuses this artifact."
                        ));
                    }
                    fail = true;
                }
            }
        }
        if unprobed > 0 {
            note(&format!(
                "{summary}; TERMINAL stage — censused but NOT probed: it has no consumer, so no read-only"
            ));
            note(&format!(
                "{id} — canonical probe exists, and CHAIN-CURRENCY does not close the gap either (its"
            ));
            note(&format!(
                "{id} — content comparison excludes the proof surface). The only loader that would answer"
            ));
            note(&format!(
                "{id} — for it is `specforge validate`, which mutates the artifact and the chain above it."
            ));
        } else if stage_total {
            note(&format!(
                "{summary}; TOTAL probe: {accepted} of {sealed} accepted, {rejected} refused, {precondition_absent} with \
                 no verdict"
            ));
        } else {
            note(&format!(
                "{summary}; SAMPLED probe ({probes} of {sealed} documents, one per distinct seal): {accepted} accepted, \
                 {rejected} refused, {precondition_absent} with no verdict"
            ));
        }
    }
    if fail {
        fail_note(
            "the persisted corpus is out of seal with the current build. Re-seal it under its owning",
        );
        fail_note(
            "task leaf with before/after evidence — never as a side effect of an unrelated slice.",
        );
        return 1;
    }
    if total {
        note(
            "every probeable persisted artifact is accepted by the current build, asked one document at a",
        );
        note(
            "time. CHAIN-CURRENCY (CI tier) still owns whether those artifacts are the CONTENT the",
        );
        note("current binary reproduces.");
    } else if !total_stages.is_empty() {
        note(&format!(
            "every persisted artifact at [{total_stages}] is accepted by the current build, asked one"
        ));
        note(
            "document at a time; the other stages are SAMPLED, one document per distinct seal, and cannot",
        );
        note(
            "see a per-document replay-topology divergence there — measured once at 23 accepted / 4",
        );
        note(
            "refused under a single seal. Their probes replay extraction and are the expensive ones, which",
        );
        note(
            "is why the tier is per stage (CORPUS-CHAIN-CURRENCY.5/.6). Run --total for the per-document",
        );
        note("verdict at every stage, and CHAIN-CURRENCY for whether the content reproduces.");
    } else {
        note(
            "the sampled persisted artifacts carry seals the current build accepts. THIS IS A SAMPLE: one",
        );
        note(
            "document per distinct seal, so it cannot see a per-document replay-topology divergence —",
        );
        note(
            "measured once at 23 accepted / 4 refused under a single seal, and the corpus currently",
        );
        note(
            "carries ONE seal per stage across 27 artifacts, so the sample is 1 in 27. The per-stage",
        );
        note("TOTAL tier that closes this at semantic and intent is ACTIVE BY DEFAULT since");
        note(
            "CORPUS-CHAIN-CURRENCY.9, so seeing this message means the stage set was emptied for this run",
        );
        note(
            "(SPECFORGE_PROOF_SEAL_TOTAL_STAGES) and the check is knowingly blind here. Run --total (CI",
        );
        note(
            "tier) for the per-document verdict, and CHAIN-CURRENCY for whether the content reproduces.",
        );
    }
    0
}

// The self-test.

/// Prove the controls fail-closed before trusting a pass: 21 cases, the end-to-end ones through the registered
/// script with recording stubs in place of the binary.
fn self_test(root: &Path) -> i32 {
    let environment = match project_data::activate(root) {
        Ok(environment) => environment,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let work = scratch(&environment, "self-test");
    if fs::create_dir_all(&work).is_err() {
        fail_note("cannot create a repository-local self-test workspace");
        return 1;
    }
    let passed = self_test_cases(root, &environment, &work);
    let _ = fs::remove_dir_all(&work);
    if passed == EXPECTED_SELF_TEST_CASES {
        note(&format!(
            "self-test {EXPECTED_SELF_TEST_CASES}/{EXPECTED_SELF_TEST_CASES} passed."
        ));
        return 0;
    }
    fail_note(&format!(
        "self-test {passed}/{EXPECTED_SELF_TEST_CASES} passed"
    ));
    1
}

/// Write an executable stub.
fn stub(path: &Path, lines: &[&str]) {
    use std::os::unix::fs::PermissionsExt;
    let _ = fs::write(path, format!("{}\n", lines.join("\n")));
    let _ = fs::set_permissions(path, fs::Permissions::from_mode(0o755));
}

/// Every regular file under `root`, digested and sorted, as the script's `find … shasum` pipeline compared them.
fn tree_digest(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn walk(directory: &Path, out: &mut Vec<(PathBuf, Vec<u8>)>) {
        let Ok(entries) = fs::read_dir(directory) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            match entry.file_type() {
                Ok(kind) if kind.is_dir() => walk(&path, out),
                Ok(kind) if kind.is_file() => {
                    let bytes = fs::read(&path).unwrap_or_default();
                    out.push((path, bytes));
                }
                _ => {}
            }
        }
    }
    let mut out = Vec::new();
    walk(root, &mut out);
    out.sort();
    out
}

fn self_test_cases(root: &Path, environment: &Environment, work: &Path) -> usize {
    let write = |name: &str, text: &str| {
        let path = work.join(name);
        let _ = fs::write(&path, text);
        path.to_string_lossy().into_owned()
    };
    let recorded_ruleset = |path: &str| {
        sealed_artifacts(&[path])
            .first()
            .map_or_else(|| "none".to_string(), |(_, seal)| seal.clone())
    };
    let carries = |path: &str| !sealed_artifacts(&[path]).is_empty();
    let mut passed = 0;
    let mut check = |ok: bool, failure: String| {
        if ok {
            passed += 1;
        } else {
            fail_note(&failure);
        }
    };
    let (hex_a, hex_b, hex_c) = ("a".repeat(64), "b".repeat(64), "c".repeat(64));

    let compact = write(
        "compact.json",
        &format!(
            r#"{{"stage":"source_ir","proof_ledger":{{"schema_version":1,"ruleset_sha256":"{hex_a}","claims":[]}}}}"#
        ),
    );
    let output = recorded_ruleset(&compact);
    check(
        output == hex_a && carries(&compact),
        format!("self-test 1: a compactly serialized ledger was misread (got '{output}')"),
    );
    let proofless = write(
        "proofless.json",
        "{\n  \"stage\": \"source_ir\",\n  \"content_elements\": []\n}\n",
    );
    let output = recorded_ruleset(&proofless);
    check(
        output == "none" && !carries(&proofless),
        format!(
            "self-test 2: a proofless artifact was selected or reported a seal (got '{output}')"
        ),
    );
    let nested = write(
        "nested.json",
        &format!(
            r#"{{"stage":"source_ir","proof_ledger":{{"claims":[{{"ruleset_sha256":"{hex_b}"}}],"ruleset_sha256":"{hex_c}"}}}}"#
        ),
    );
    let output = recorded_ruleset(&nested);
    check(
        output == hex_c,
        format!(
            "self-test 3: a nested claim digest was mistaken for the ledger seal (got '{output}')"
        ),
    );
    let nested_ledger = write(
        "nested_ledger.json",
        &format!(
            r#"{{"stage":"source_ir","upstream":{{"proof_ledger":{{"ruleset_sha256":"{hex_b}"}}}}}}"#
        ),
    );
    check(
        !proof_ledger_candidates(&[&nested_ledger]).is_empty()
            && !carries(&nested_ledger)
            && recorded_ruleset(&nested_ledger) == "none",
        "self-test 4: a nested proof_ledger was accepted as a top-level seal, or the prefilter missed it".to_string(),
    );
    let mixed = [
        proofless.as_str(),
        compact.as_str(),
        nested.as_str(),
        nested_ledger.as_str(),
    ];
    let (candidates, sealed) = (
        proof_ledger_candidates(&mixed).len(),
        sealed_artifacts(&mixed).len(),
    );
    check(
        candidates == 3 && sealed == 2,
        format!(
            "self-test 5: the batched census disagreed with the per-file predicate ({candidates} candidates, {sealed} \
             sealed; expected 3 and 2)"
        ),
    );

    // 6-8: the stage table and the probe, read from the library that defines them.
    let library = |script: &str, extra: &[(&str, &str)]| {
        let mut shell = command("bash", root, environment);
        shell.args(["-c", script, "self-test", LIBRARY]);
        for (name, value) in extra {
            shell.env(name, value);
        }
        shell.output().ok()
    };
    let chain = library(
        ". \"$1\"; previous=''; ok=1; for this in $(chain_stages); do if producer=\"$(chain_stage_producer \"$this\")\"; \
         then [ \"$(chain_stage_artifact_path \"$producer\" k)\" = \"$previous\" ] || ok=0; else [ -z \"$previous\" ] || \
         ok=0; fi; previous=\"$(chain_stage_artifact_path \"$this\" k)\" || ok=0; done; [ \"$ok\" -eq 1 ]",
        &[],
    );
    check(
        chain.is_some_and(|output| output.status.success()),
        "self-test 6: the shared stage table is not a chain".to_string(),
    );
    let unknown = library(". \"$1\"; chain_stage_artifact_path bogus k", &[]);
    check(
        unknown.is_some_and(|output| !output.status.success()),
        "self-test 7: an unknown stage resolved to an artifact path".to_string(),
    );
    let probe_error = work.join("probe.err");
    let terminal = library(
        &format!(
            ". \"$1\"; chain_stage_readonly_probe isf-adapter '{compact}' '{}'",
            probe_error.display()
        ),
        &[("BIN", "/nonexistent")],
    )
    .and_then(|output| output.status.code());
    match terminal {
        Some(0) => check(
            false,
            "self-test 8: the terminal stage reported a canonical pass it cannot have".to_string(),
        ),
        Some(2) => check(true, String::new()),
        status => check(
            false,
            format!(
                "self-test 8: the terminal stage did not report an absent probe (status {})",
                status.map_or_else(String::new, |status| status.to_string())
            ),
        ),
    }

    // A miniature corpus: three sealed documents, doc_c sharing doc_a's seal, and the same three at semantic.
    let mini = work.join("mini");
    for (stage, file, sources) in [
        ("source_ir", "source_ir.json", [&compact, &nested, &compact]),
        (
            "semantic_ir",
            "semantic_ir.json",
            [&compact, &compact, &compact],
        ),
    ] {
        for (document, source) in ["doc_a", "doc_b", "doc_c"].iter().zip(sources) {
            let directory = mini.join(stage).join(document);
            let _ = fs::create_dir_all(&directory);
            let _ = fs::copy(source, directory.join(file));
        }
    }
    let script = root.join("scripts/check_proof_seal_currency.sh");
    let run_gate = |mode: &str, extra: &[(&str, String)]| {
        let mut gate = command(&script, root, environment);
        gate.arg(mode)
            .env("SPECFORGE_PROOF_SEAL_GENERATED_ROOT", &mini);
        for (name, value) in extra {
            gate.env(name, value);
        }
        match gate.output() {
            Ok(output) => {
                let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
                text.push_str(&String::from_utf8_lossy(&output.stderr));
                (
                    output.status.code().unwrap_or(1),
                    text.trim_end_matches('\n').to_string(),
                )
            }
            Err(error) => (1, error.to_string()),
        }
    };
    let accept_stub = work.join("accepting-specforge");
    stub(
        &accept_stub,
        &[
            "#!/usr/bin/env bash",
            "printf \"%s\\n\" \"$*\" >> \"$SPECFORGE_STUB_LOG\"",
            "exit 0",
        ],
    );
    let accept_log = work.join("accept.log");
    let _ = fs::write(&accept_log, b"");
    let accepting = [
        (
            "SPECFORGE_STUB_LOG",
            accept_log.to_string_lossy().into_owned(),
        ),
        (
            "SPECFORGE_PROOF_SEAL_BIN",
            accept_stub.to_string_lossy().into_owned(),
        ),
    ];
    let (status, output) = run_gate("--check", &accepting);
    check(
        status == 0,
        format!(
            "self-test 9: an accepted corpus did not pass (status {status}, output '{output}')"
        ),
    );
    check(
        output.contains("2 distinct seal"),
        format!("self-test 10: two divergent seals were not both censused (output '{output}')"),
    );
    let log = fs::read_to_string(&accept_log).unwrap_or_default();
    let probes = log
        .lines()
        .filter(|line| line.contains("--dry-run"))
        .count();
    check(
        probes >= 2,
        format!(
            "self-test 11: divergent seals did not each earn a probe ({probes} dry-run probes)"
        ),
    );
    check(
        !log.lines().any(|line| line.starts_with("validate ")),
        "self-test 12: the gate invoked 'specforge validate', which mutates the artifact it claims only to read".to_string(),
    );
    let before = tree_digest(&mini);
    let _ = run_gate("--check", &accepting);
    check(
        before == tree_digest(&mini),
        "self-test 13: --check modified the corpus root".to_string(),
    );

    let reject_stub = work.join("rejecting-specforge");
    stub(
        &reject_stub,
        &[
            "#!/usr/bin/env bash",
            "printf \"SourceIR proof verification failed: proof ledger ruleset hash is stale\\n\" >&2",
            "exit 1",
        ],
    );
    let (status, output) = run_gate(
        "--check",
        &[(
            "SPECFORGE_PROOF_SEAL_BIN",
            reject_stub.to_string_lossy().into_owned(),
        )],
    );
    check(
        status != 0,
        "self-test 14: a rejected persisted seal did not fail the gate".to_string(),
    );
    check(
        output.contains("source_proof_migrate"),
        format!(
            "self-test 15: a stale seal was not classified with its remedy (output '{output}')"
        ),
    );
    let absent = {
        let mut gate = command(&script, root, environment);
        gate.arg("--check").env(
            "SPECFORGE_PROOF_SEAL_GENERATED_ROOT",
            work.join("absent-corpus"),
        );
        match gate.output() {
            Ok(output) => {
                let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
                text.push_str(&String::from_utf8_lossy(&output.stderr));
                (
                    output.status.code().unwrap_or(1),
                    text.trim_end_matches('\n').to_string(),
                )
            }
            Err(error) => (1, error.to_string()),
        }
    };
    let status = if absent.1.contains("SKIP") {
        absent.0
    } else {
        99
    };
    check(
        status == 0,
        format!(
            "self-test 16: an absent corpus root did not skip loudly (status {status}, output '{}')",
            absent.1
        ),
    );
    let selective_stub = work.join("selective-specforge");
    stub(
        &selective_stub,
        &[
            "#!/usr/bin/env bash",
            "case \"$*\" in",
            "  *source_ir/doc_c*) printf \"EvidenceIR proof verification failed: registered derivation output or input topology is stale\\n\" >&2; exit 1 ;;",
            "esac",
            "exit 0",
        ],
    );
    let selective = selective_stub.to_string_lossy().into_owned();
    let (status, _) = run_gate(
        "--check",
        &[("SPECFORGE_PROOF_SEAL_BIN", selective.clone())],
    );
    check(
        status == 0,
        format!(
            "self-test 17: the SAMPLED stage was expected to MISS a divergent same-seal document (status {status})"
        ),
    );
    let semantic_stub = work.join("semantic-selective-specforge");
    stub(
        &semantic_stub,
        &[
            "#!/usr/bin/env bash",
            "case \"$*\" in",
            "  *semantic_ir/doc_c*) printf \"SemanticIR proof verification failed: registered derivation output or input topology is stale\\n\" >&2; exit 1 ;;",
            "esac",
            "exit 0",
        ],
    );
    let semantic = semantic_stub.to_string_lossy().into_owned();
    let caught = |status: i32,
                  output: &str,
                  case: &str,
                  passed_over: &str,
                  unnamed: &str|
     -> (bool, String) {
        if status == 0 {
            (false, format!("self-test {case}: {passed_over}"))
        } else if output.contains("doc_c") {
            (true, String::new())
        } else {
            (
                false,
                format!("self-test {case}: {unnamed} (output '{output}')"),
            )
        }
    };
    let (status, output) = run_gate(
        "--check",
        &[
            ("SPECFORGE_PROOF_SEAL_BIN", semantic.clone()),
            (
                "SPECFORGE_PROOF_SEAL_TOTAL_STAGES",
                "semantic intent".to_string(),
            ),
        ],
    );
    let (ok, failure) = caught(
        status,
        &output,
        "17b",
        "a TOTAL stage passed over a document its own loader refuses",
        "a TOTAL stage failed without naming the divergent document",
    );
    check(ok, failure);
    let (status, output) = run_gate("--check", &[("SPECFORGE_PROOF_SEAL_BIN", semantic)]);
    let (ok, failure) = caught(
        status,
        &output,
        "21",
        "the DEFAULT stage set passed over a document its own loader refuses",
        "the default failed without naming the divergent document",
    );
    check(ok, failure);
    let (status, output) = run_gate("--total", &[("SPECFORGE_PROOF_SEAL_BIN", selective)]);
    let (ok, failure) = caught(
        status,
        &output,
        "18",
        "--total passed over a document its own loader refuses",
        "--total failed without naming the divergent document",
    );
    check(ok, failure);
    let absent_stub = work.join("absent-input-specforge");
    stub(
        &absent_stub,
        &[
            "#!/usr/bin/env bash",
            "printf \"error: path does not exist: /nope/normalized/doc.md\\n\" >&2",
            "exit 1",
        ],
    );
    let (status, output) = run_gate(
        "--total",
        &[(
            "SPECFORGE_PROOF_SEAL_BIN",
            absent_stub.to_string_lossy().into_owned(),
        )],
    );
    if status == 0 && output.contains("with no verdict") {
        check(true, String::new());
    } else if status == 0 {
        check(
            false,
            format!(
                "self-test 19: an absent probe input passed WITHOUT being reported as no-verdict (output '{output}')"
            ),
        );
    } else {
        check(
            false,
            format!(
                "self-test 19: an absent probe input was reported as a seal breach (status {status})"
            ),
        );
    }
    passed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_seal_reader_takes_only_the_top_level_ledgers_direct_seal() {
        let work = crate::repository_root().join(".project-data/tmp/xtask-proof-seal-test");
        let _ = fs::remove_dir_all(&work);
        fs::create_dir_all(&work).expect("the fixture's directory");
        let (a, b) = ("a".repeat(64), "b".repeat(64));
        let pad = "x".repeat((1 << 20) - 15);
        let cases: Vec<(&str, String, Option<Option<String>>)> = vec![
            (
                "compact",
                format!(r#"{{"proof_ledger":{{"ruleset_sha256":"{a}"}}}}"#),
                Some(Some(a.clone())),
            ),
            (
                "nested-claim",
                format!(
                    r#"{{"proof_ledger":{{"claims":[{{"ruleset_sha256":"{b}"}}],"ruleset_sha256":"{a}"}}}}"#
                ),
                Some(Some(a.clone())),
            ),
            (
                "nested-ledger",
                format!(r#"{{"up":{{"proof_ledger":{{"ruleset_sha256":"{b}"}}}}}}"#),
                None,
            ),
            (
                "no-seal",
                r#"{"proof_ledger":{"claims":[]}}"#.to_string(),
                Some(None),
            ),
            (
                "braces-in-strings",
                format!(
                    r#"{{"t":"{{[}}]:,","proof_ledger":{{"k":"}}}}","ruleset_sha256":"{a}"}}}}"#
                ),
                Some(Some(a.clone())),
            ),
            (
                "unterminated",
                r#"{"proof_ledger":{"ruleset_sha256":"abc"#.to_string(),
                Some(None),
            ),
            (
                "straddles",
                format!(r#"{{"pad":"{pad}","proof_ledger":{{"ruleset_sha256":"{a}"}}}}"#),
                Some(Some(a.clone())),
            ),
        ];
        let mut verdicts = Vec::new();
        for (name, text, _) in &cases {
            let path = work.join(format!("{name}.json"));
            fs::write(&path, text).expect("an artifact");
            let path = path.to_string_lossy().into_owned();
            verdicts.push((
                scan_proof_ledger(Path::new(&path)),
                proof_ledger_candidates(&[&path]).len(),
            ));
        }
        let empty = work.join("empty-seal.json");
        fs::write(&empty, r#"{"proof_ledger":{"ruleset_sha256":""}}"#).expect("an artifact");
        let empty_rows = sealed_artifacts(&[&empty.to_string_lossy()]).len();
        fs::remove_dir_all(&work).expect("cleanup");
        for ((name, _, expected), (verdict, candidates)) in cases.iter().zip(verdicts) {
            assert_eq!(&verdict, expected, "{name}");
            assert_eq!(candidates, 1, "{name}: the prefilter must see the key");
        }
        assert_eq!(empty_rows, 0);
    }

    #[test]
    fn the_probe_scope_and_the_diagnostic_follow_the_script() {
        assert!(probes_totally(false, DEFAULT_TOTAL_STAGES, "semantic"));
        assert!(probes_totally(false, DEFAULT_TOTAL_STAGES, "intent"));
        assert!(!probes_totally(false, DEFAULT_TOTAL_STAGES, "evidence"));
        assert!(!probes_totally(false, "", "semantic"));
        assert!(probes_totally(true, "", "source-ir"));
        assert_eq!(
            diagnostic(b"line one\nfailed:   twice  \n"),
            "line one failed: twice "
        );
        assert_eq!(diagnostic(&[b'x'; 300]).len(), 200);
    }

    #[test]
    fn the_shared_library_reaches_each_reader_by_its_own_mode() {
        let library =
            fs::read_to_string(crate::repository_root().join(LIBRARY)).expect("the shared library");
        let body = |function: &str| {
            let start = library
                .find(&format!("{function}() {{"))
                .expect("the function");
            library[start..]
                .split("\n}")
                .next()
                .expect("its body")
                .to_string()
        };
        assert!(body("proof_ledger_candidates").contains("--proof-ledger-candidates \"$@\""));
        assert!(body("scan_proof_ledger").contains("--scan-proof-ledger \"$1\""));
        assert!(body("sealed_artifacts").contains("--sealed-artifacts \"$@\""));
    }
}
