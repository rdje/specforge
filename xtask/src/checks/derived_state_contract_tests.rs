//! The 47 derived-state contract checks, native since `RUSTDOC-AND-XTASK.15.l`: all four field classes,
//! generic core and adapter verifiers, secondary copies, registry bounds and every fail-closed mutation.

use std::fs;
use std::path::{Path, PathBuf};

use crate::pyjson::{self, Py};

use super::Output;
use super::derived_state_contracts;

struct Fixture {
    root: PathBuf,
    surface_meta: Py,
    surfaces: Vec<Py>,
    derived_meta: Py,
    contracts: Vec<Py>,
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn text(value: &str) -> Py {
    Py::Str(pyjson::py_str(value))
}

fn integer(value: usize) -> Py {
    Py::Int(value.to_string())
}

fn object(pairs: Vec<(&str, Py)>) -> Py {
    Py::Dict(
        pairs
            .into_iter()
            .map(|(key, value)| (pyjson::py_str(key), value))
            .collect(),
    )
}

fn field<'a>(value: &'a Py, key: &str) -> Option<&'a Py> {
    pyjson::get(value, key)
}

fn field_mut<'a>(value: &'a mut Py, key: &str) -> Option<&'a mut Py> {
    if let Py::Dict(pairs) = value {
        pairs
            .iter_mut()
            .find(|(candidate, _)| *candidate == pyjson::py_str(key))
            .map(|(_, item)| item)
    } else {
        None
    }
}

fn set_field(value: &mut Py, key: &str, item: Py) {
    if let Py::Dict(pairs) = value {
        let key = pyjson::py_str(key);
        if let Some((_, current)) = pairs.iter_mut().find(|(candidate, _)| *candidate == key) {
            *current = item;
        } else {
            pairs.push((key, item));
        }
    }
}

fn remove_field(value: &mut Py, key: &str) {
    if let Py::Dict(pairs) = value {
        let key = pyjson::py_str(key);
        pairs.retain(|(candidate, _)| *candidate != key);
    }
}

fn write_text(root: &Path, relative: &str, content: &str) -> Result<(), String> {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create {}: {error}", parent.display()))?;
    }
    fs::write(&path, content)
        .map_err(|error| format!("cannot write fixture '{}': {error}", path.display()))
}

fn read_text(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read fixture '{relative}': {error}"))
}

fn encode(value: &Py) -> Result<String, String> {
    pyjson::dumps(
        value,
        &pyjson::Encoder {
            indent: None,
            sort_keys: true,
            ensure_ascii: false,
            item_separator: ",",
            key_separator: ":",
        },
    )
    .ok_or_else(|| "cannot encode fixture JSON".to_string())
}

fn save_jsonl(root: &Path, relative: &str, meta: &Py, records: &[Py]) -> Result<(), String> {
    let mut content = encode(meta)? + "\n";
    for record in records {
        content.push_str(&(encode(record)? + "\n"));
    }
    write_text(root, relative, &content)
}

fn save_fixture(fixture: &Fixture) -> Result<(), String> {
    save_jsonl(
        &fixture.root,
        "control/surfaces.jsonl",
        &fixture.surface_meta,
        &fixture.surfaces,
    )?;
    save_jsonl(
        &fixture.root,
        "control/derived.jsonl",
        &fixture.derived_meta,
        &fixture.contracts,
    )
}

fn contract_mut<'a>(fixture: &'a mut Fixture, id: &str) -> &'a mut Py {
    fixture
        .contracts
        .iter_mut()
        .find(|candidate| {
            field(candidate, "contract_id")
                .and_then(|value| match value {
                    Py::Str(text) => Some(pyjson::to_text(text)),
                    _ => None,
                })
                .is_some_and(|candidate| candidate == id)
        })
        .unwrap_or_else(|| panic!("fixture contract '{id}' does not exist"))
}

fn secondary_copy_mut<'a>(fixture: &'a mut Fixture, role: &str) -> &'a mut Py {
    let copies = match field_mut(contract_mut(fixture, "adapter_copy"), "secondary_copies") {
        Some(Py::List(copies)) => copies,
        _ => panic!("fixture secondary copies do not exist"),
    };
    copies
        .iter_mut()
        .find(|copy| {
            field(copy, "role")
                .and_then(|value| match value {
                    Py::Str(text) => Some(pyjson::to_text(text)),
                    _ => None,
                })
                .is_some_and(|candidate| candidate == role)
        })
        .unwrap_or_else(|| panic!("fixture secondary copy '{role}' does not exist"))
}

fn registry_meta(max_records: usize, max_bytes: usize) -> Py {
    object(vec![
        ("record_type", text("registry")),
        ("schema_version", integer(1)),
        ("max_records", integer(max_records)),
        ("max_bytes", integer(max_bytes)),
        ("max_record_bytes", integer(2_048)),
        ("max_array_items", integer(8)),
        ("max_scalar_bytes", integer(512)),
    ])
}

fn new_fixture(root: &Path) -> Result<Fixture, String> {
    if root.exists() {
        fs::remove_dir_all(root)
            .map_err(|error| format!("remove stale fixture '{}': {error}", root.display()))?;
    }
    let mut fixture = Fixture {
        root: root.to_path_buf(),
        surface_meta: Py::None,
        surfaces: Vec::new(),
        derived_meta: Py::None,
        contracts: Vec::new(),
    };
    write_text(
        root,
        "current.md",
        "# Current\nReader accessor: fixture-current\nVerified core: yes\nVerified adapter: yes\nAuthored intent: selected\nUndeclared date 2099-01-01 and hash deadbeef remain ordinary prose.\n",
    )?;
    write_text(
        root,
        "evidence.md",
        "# Evidence\nImmutable measurement: 7\nCaptured by fixture revision\n",
    )?;
    write_text(root, "history.md", "# Historical terminal\n")?;
    write_text(root, "secondary.md", "# Secondary\nSurface copy: yes\n")?;
    write_text(root, "control.yml", "control_copy: yes\n")?;
    write_text(
        root,
        "scripts/core-ok.pl",
        "use strict; use warnings; open my $fh, '>', 'core-proof.txt' or die $!; print {$fh} \"ok\\n\"; close $fh; exit 0;\n",
    )?;
    write_text(root, "scripts/core-fail.pl", "exit 9;\n")?;
    write_text(
        root,
        "scripts/adapter-ok.pl",
        "use strict; use warnings; open my $fh, '>', 'adapter-proof.txt' or die $!; print {$fh} join(' ', \\@ARGV), \"\\n\"; close $fh; exit 0;\n",
    )?;
    write_text(root, "scripts/adapter-fail.pl", "exit 7;\n")?;
    fixture.surface_meta = registry_meta(8, 8_192);
    fixture.surfaces = vec![
        object(vec![
            ("surface_id", text("current_surface")),
            ("targets", Py::List(vec![text("current.md")])),
            ("lifecycle", text("bounded_snapshot")),
        ]),
        object(vec![
            ("surface_id", text("evidence_surface")),
            ("targets", Py::List(vec![text("evidence.md")])),
            ("lifecycle", text("partitioned_canonical")),
        ]),
        object(vec![
            ("surface_id", text("historical_surface")),
            ("targets", Py::List(vec![text("history.md")])),
            ("lifecycle", text("archive_terminal")),
        ]),
        object(vec![
            ("surface_id", text("secondary_surface")),
            ("targets", Py::List(vec![text("secondary.md")])),
            ("lifecycle", text("maintained_reference")),
        ]),
    ];
    fixture.derived_meta = registry_meta(12, 16_384);
    fixture.contracts = vec![
        object(vec![
            ("record_type", text("contract")),
            ("schema_version", integer(1)),
            ("contract_id", text("revision")),
            ("surface_id", text("current_surface")),
            ("path", text("current.md")),
            ("field_id", text("revision")),
            ("classification", text("derive_on_read")),
            ("field_marker", text("Reader accessor: fixture-current")),
            ("authority", text("fixture revision authority")),
            ("accessor", text("fixture-current")),
            ("forbidden_storage_marker", text("Stored revision:")),
            ("verifier", text("builtin:derive_on_read")),
        ]),
        object(vec![
            ("record_type", text("contract")),
            ("schema_version", integer(1)),
            ("contract_id", text("core_copy")),
            ("surface_id", text("current_surface")),
            ("path", text("current.md")),
            ("field_id", text("core_copy")),
            ("classification", text("verified_copy")),
            ("field_marker", text("Verified core: yes")),
            ("authority", text("fixture core authority")),
            ("accessor", text("scripts/core-ok.pl")),
            ("verifier", text("core:scripts/core-ok.pl")),
        ]),
        object(vec![
            ("record_type", text("contract")),
            ("schema_version", integer(1)),
            ("contract_id", text("adapter_copy")),
            ("surface_id", text("current_surface")),
            ("path", text("current.md")),
            ("field_id", text("adapter_copy")),
            ("classification", text("verified_copy")),
            ("field_marker", text("Verified adapter: yes")),
            ("authority", text("fixture adapter authority")),
            ("accessor", text("scripts/adapter-ok.pl")),
            ("verifier", text("adapter:scripts/adapter-ok.pl")),
            (
                "secondary_copies",
                Py::List(vec![
                    object(vec![
                        ("role", text("surface_copy")),
                        ("ownership", text("surface")),
                        ("surface_id", text("secondary_surface")),
                        ("path", text("secondary.md")),
                        ("field_marker", text("Surface copy: yes")),
                    ]),
                    object(vec![
                        ("role", text("control_copy")),
                        ("ownership", text("control")),
                        ("path", text("control.yml")),
                        ("field_marker", text("control_copy: yes")),
                    ]),
                ]),
            ),
        ]),
        object(vec![
            ("record_type", text("contract")),
            ("schema_version", integer(1)),
            ("contract_id", text("intent")),
            ("surface_id", text("current_surface")),
            ("path", text("current.md")),
            ("field_id", text("intent")),
            ("classification", text("authored_intent")),
            ("field_marker", text("Authored intent: selected")),
            ("authority", text("fixture author judgement")),
        ]),
        object(vec![
            ("record_type", text("contract")),
            ("schema_version", integer(1)),
            ("contract_id", text("evidence")),
            ("surface_id", text("evidence_surface")),
            ("path", text("evidence.md")),
            ("field_id", text("measurement")),
            ("classification", text("immutable_evidence")),
            ("field_marker", text("Immutable measurement: 7")),
            ("authority", text("fixture capture record")),
            ("capture_boundary", text("Captured by fixture revision")),
        ]),
    ];
    save_fixture(&fixture)?;
    Ok(fixture)
}

fn run_checker(fixture: &Fixture) -> (i32, String) {
    let mut output = super::Captured::default();
    let code = derived_state_contracts::run_to(
        &fixture.root,
        &[
            "--root".to_string(),
            fixture.root.to_string_lossy().into_owned(),
            "--registry".to_string(),
            "control/derived.jsonl".to_string(),
            "--surfaces".to_string(),
            "control/surfaces.jsonl".to_string(),
        ],
        &mut output,
    );
    (code, output.text)
}

#[derive(Clone, Copy)]
enum Mutation {
    Lookalikes,
    InvalidClassification,
    UnknownContractField,
    DuplicateContract,
    DuplicateField,
    DuplicateMarker,
    SecondaryUnknownField,
    NonCopySecondary,
    EmptySecondary,
    DuplicateSecondaryRole,
    InvalidSecondaryRole,
    InvalidOwnership,
    SecondaryTraversal,
    SecondaryMissing,
    SecondarySymlink,
    SurfaceMissingId,
    UnknownSecondarySurface,
    HistoricalSecondary,
    OffSurfaceSecondary,
    ControlSurfaceId,
    ControlMarkdown,
    SecondaryMissingMarker,
    SecondaryDuplicateMarker,
    PrimarySecondaryDuplicateMarker,
    UnsafePrimaryPath,
    UnknownSurface,
    HistoricalPrimary,
    OffSurfacePrimary,
    MissingPrimaryFile,
    MissingPrimaryMarker,
    DuplicatePrimaryMarker,
    StoredRevision,
    MissingAccessor,
    WrongDeriveVerifier,
    InvalidVerifierScheme,
    MissingCore,
    FailingCore,
    FailingAdapter,
    IntentVerifier,
    MissingCaptureBoundary,
    RecordCountBound,
    TotalByteBound,
    RawRecordBound,
    ScalarBound,
    SchemaVersion,
}

struct Case {
    name: &'static str,
    success: bool,
    patterns: &'static [&'static str],
    mutation: Mutation,
}

const CASES: &[Case] = &[
    Case {
        name: "undeclared date, number, and hash lookalikes are not guessed",
        success: true,
        patterns: &["5 explicit field contracts"],
        mutation: Mutation::Lookalikes,
    },
    Case {
        name: "invalid classification fails closed",
        success: false,
        patterns: &["invalid classification"],
        mutation: Mutation::InvalidClassification,
    },
    Case {
        name: "unknown contract fields fail closed",
        success: false,
        patterns: &["unknown field 'heuristic'"],
        mutation: Mutation::UnknownContractField,
    },
    Case {
        name: "duplicate contract identifiers fail closed",
        success: false,
        patterns: &["declared more than once"],
        mutation: Mutation::DuplicateContract,
    },
    Case {
        name: "duplicate surface path field declarations fail closed",
        success: false,
        patterns: &["field is declared more than once"],
        mutation: Mutation::DuplicateField,
    },
    Case {
        name: "duplicate exact markers fail closed",
        success: false,
        patterns: &["marker is declared more than once"],
        mutation: Mutation::DuplicateMarker,
    },
    Case {
        name: "secondary copies reject unknown fields",
        success: false,
        patterns: &["unknown field 'fallback_path'"],
        mutation: Mutation::SecondaryUnknownField,
    },
    Case {
        name: "non-copy classes reject secondary declarations",
        success: false,
        patterns: &["unknown field 'secondary_copies'"],
        mutation: Mutation::NonCopySecondary,
    },
    Case {
        name: "secondary copies must be a non-empty array",
        success: false,
        patterns: &["secondary_copies must be a non-empty array"],
        mutation: Mutation::EmptySecondary,
    },
    Case {
        name: "secondary copy roles must be unique",
        success: false,
        patterns: &["secondary role 'surface_copy' is declared more than once"],
        mutation: Mutation::DuplicateSecondaryRole,
    },
    Case {
        name: "secondary copy roles use bounded identifiers",
        success: false,
        patterns: &["role 'Surface Copy' has an invalid identifier shape"],
        mutation: Mutation::InvalidSecondaryRole,
    },
    Case {
        name: "secondary copy ownership is closed",
        success: false,
        patterns: &["invalid ownership 'external'"],
        mutation: Mutation::InvalidOwnership,
    },
    Case {
        name: "secondary copy paths reject traversal",
        success: false,
        patterns: &["secondary_copies[1] path is absolute, escaping, or malformed"],
        mutation: Mutation::SecondaryTraversal,
    },
    Case {
        name: "secondary copy paths require regular files",
        success: false,
        patterns: &["path is missing or not a regular file: missing.yml"],
        mutation: Mutation::SecondaryMissing,
    },
    Case {
        name: "secondary copy paths reject symlinks",
        success: false,
        patterns: &["path is missing or not a regular file: linked.yml"],
        mutation: Mutation::SecondarySymlink,
    },
    Case {
        name: "surface copies require a surface identifier",
        success: false,
        patterns: &["surface ownership requires surface_id"],
        mutation: Mutation::SurfaceMissingId,
    },
    Case {
        name: "surface copies reject unknown surfaces",
        success: false,
        patterns: &["names unknown surface 'missing_surface'"],
        mutation: Mutation::UnknownSecondarySurface,
    },
    Case {
        name: "surface copies reject historical surfaces",
        success: false,
        patterns: &["must govern a current maintained surface"],
        mutation: Mutation::HistoricalSecondary,
    },
    Case {
        name: "surface copies reject off-surface paths",
        success: false,
        patterns: &["path is outside surface 'secondary_surface'"],
        mutation: Mutation::OffSurfaceSecondary,
    },
    Case {
        name: "control copies reject surface identifiers",
        success: false,
        patterns: &["control ownership must not declare surface_id"],
        mutation: Mutation::ControlSurfaceId,
    },
    Case {
        name: "Markdown secondary copies require surface ownership",
        success: false,
        patterns: &["Markdown path must use surface ownership"],
        mutation: Mutation::ControlMarkdown,
    },
    Case {
        name: "secondary copies reject missing exact markers",
        success: false,
        patterns: &[
            "secondary_copies[1] field marker must occur exactly once",
            "found 0",
        ],
        mutation: Mutation::SecondaryMissingMarker,
    },
    Case {
        name: "secondary copies reject duplicated exact markers",
        success: false,
        patterns: &[
            "secondary_copies[1] field marker must occur exactly once",
            "found 2",
        ],
        mutation: Mutation::SecondaryDuplicateMarker,
    },
    Case {
        name: "secondary and primary locations cannot duplicate exact markers",
        success: false,
        patterns: &["marker is declared more than once"],
        mutation: Mutation::PrimarySecondaryDuplicateMarker,
    },
    Case {
        name: "unsafe paths fail closed",
        success: false,
        patterns: &["path is absolute, escaping, or malformed"],
        mutation: Mutation::UnsafePrimaryPath,
    },
    Case {
        name: "unknown surfaces fail closed",
        success: false,
        patterns: &["names unknown surface"],
        mutation: Mutation::UnknownSurface,
    },
    Case {
        name: "historical surfaces reject current-state contracts",
        success: false,
        patterns: &["must govern a current maintained surface"],
        mutation: Mutation::HistoricalPrimary,
    },
    Case {
        name: "off-surface paths fail closed",
        success: false,
        patterns: &["path is outside surface"],
        mutation: Mutation::OffSurfacePrimary,
    },
    Case {
        name: "missing contract files fail closed",
        success: false,
        patterns: &["path is missing or not a regular file"],
        mutation: Mutation::MissingPrimaryFile,
    },
    Case {
        name: "missing exact field markers fail closed",
        success: false,
        patterns: &["field marker must occur exactly once", "found 0"],
        mutation: Mutation::MissingPrimaryMarker,
    },
    Case {
        name: "duplicated exact field markers fail closed",
        success: false,
        patterns: &["field marker must occur exactly once", "found 2"],
        mutation: Mutation::DuplicatePrimaryMarker,
    },
    Case {
        name: "derive-on-read rejects a stored current value",
        success: false,
        patterns: &["still stores forbidden current-state marker"],
        mutation: Mutation::StoredRevision,
    },
    Case {
        name: "derive-on-read rejects a missing reader accessor",
        success: false,
        patterns: &["accessor is absent"],
        mutation: Mutation::MissingAccessor,
    },
    Case {
        name: "derive-on-read rejects the wrong verifier class",
        success: false,
        patterns: &["must use builtin:derive_on_read"],
        mutation: Mutation::WrongDeriveVerifier,
    },
    Case {
        name: "verified copies reject undeclared verifier schemes",
        success: false,
        patterns: &["must use core: or adapter:"],
        mutation: Mutation::InvalidVerifierScheme,
    },
    Case {
        name: "verified copies reject missing core verifiers",
        success: false,
        patterns: &["verifier is missing"],
        mutation: Mutation::MissingCore,
    },
    Case {
        name: "verified copies reject failing core verifiers",
        success: false,
        patterns: &["verifier failed ", "core-fail.pl"],
        mutation: Mutation::FailingCore,
    },
    Case {
        name: "verified copies reject failing adapters",
        success: false,
        patterns: &["verifier failed ", "adapter-fail.pl"],
        mutation: Mutation::FailingAdapter,
    },
    Case {
        name: "authored intent rejects mechanical verifier fields",
        success: false,
        patterns: &["unknown field 'verifier'"],
        mutation: Mutation::IntentVerifier,
    },
    Case {
        name: "immutable evidence rejects a missing capture boundary",
        success: false,
        patterns: &["capture boundary must occur exactly once", "found 0"],
        mutation: Mutation::MissingCaptureBoundary,
    },
    Case {
        name: "registry record-count bounds fail closed",
        success: false,
        patterns: &["more records than its declared max_records"],
        mutation: Mutation::RecordCountBound,
    },
    Case {
        name: "registry total-byte bounds fail closed",
        success: false,
        patterns: &["exceeds its declared max_bytes"],
        mutation: Mutation::TotalByteBound,
    },
    Case {
        name: "registry raw-record bounds fail closed",
        success: false,
        patterns: &["record above its declared max_record_bytes"],
        mutation: Mutation::RawRecordBound,
    },
    Case {
        name: "registry scalar bounds fail closed",
        success: false,
        patterns: &["exceeds the declared scalar byte limit 8"],
        mutation: Mutation::ScalarBound,
    },
    Case {
        name: "registry schema version fails closed",
        success: false,
        patterns: &["schema_version must be 1"],
        mutation: Mutation::SchemaVersion,
    },
];

#[cfg(unix)]
fn symlink_file(source: &Path, target: &Path) -> std::io::Result<()> {
    std::os::unix::fs::symlink(source, target)
}

#[cfg(windows)]
fn symlink_file(source: &Path, target: &Path) -> std::io::Result<()> {
    std::os::windows::fs::symlink_file(source, target)
}

fn mutate(fixture: &mut Fixture, mutation: Mutation) -> Result<(), String> {
    match mutation {
        Mutation::Lookalikes => write_text(
            &fixture.root,
            "current.md",
            &(read_text(&fixture.root, "current.md")?
                + "Build 999 at 2099-12-31 has token 0123456789abcdef.\n"),
        )?,
        Mutation::InvalidClassification => set_field(
            contract_mut(fixture, "intent"),
            "classification",
            text("ambient_guess"),
        ),
        Mutation::UnknownContractField => {
            set_field(contract_mut(fixture, "intent"), "heuristic", text("date"))
        }
        Mutation::DuplicateContract => {
            let copy = contract_mut(fixture, "intent").clone();
            fixture.contracts.push(copy);
        }
        Mutation::DuplicateField => {
            let mut copy = contract_mut(fixture, "intent").clone();
            set_field(&mut copy, "contract_id", text("intent_duplicate_field"));
            fixture.contracts.push(copy);
        }
        Mutation::DuplicateMarker => {
            let mut copy = contract_mut(fixture, "intent").clone();
            set_field(&mut copy, "contract_id", text("intent_duplicate_marker"));
            set_field(&mut copy, "field_id", text("intent_other_field"));
            fixture.contracts.push(copy);
        }
        Mutation::SecondaryUnknownField => set_field(
            secondary_copy_mut(fixture, "control_copy"),
            "fallback_path",
            text("hidden.yml"),
        ),
        Mutation::NonCopySecondary => set_field(
            contract_mut(fixture, "intent"),
            "secondary_copies",
            Py::List(Vec::new()),
        ),
        Mutation::EmptySecondary => set_field(
            contract_mut(fixture, "adapter_copy"),
            "secondary_copies",
            Py::List(Vec::new()),
        ),
        Mutation::DuplicateSecondaryRole => set_field(
            secondary_copy_mut(fixture, "control_copy"),
            "role",
            text("surface_copy"),
        ),
        Mutation::InvalidSecondaryRole => set_field(
            secondary_copy_mut(fixture, "surface_copy"),
            "role",
            text("Surface Copy"),
        ),
        Mutation::InvalidOwnership => set_field(
            secondary_copy_mut(fixture, "control_copy"),
            "ownership",
            text("external"),
        ),
        Mutation::SecondaryTraversal => set_field(
            secondary_copy_mut(fixture, "control_copy"),
            "path",
            text("../control.yml"),
        ),
        Mutation::SecondaryMissing => set_field(
            secondary_copy_mut(fixture, "control_copy"),
            "path",
            text("missing.yml"),
        ),
        Mutation::SecondarySymlink => {
            symlink_file(
                &fixture.root.join("control.yml"),
                &fixture.root.join("linked.yml"),
            )
            .map_err(|error| format!("cannot create fixture symlink: {error}"))?;
            set_field(
                secondary_copy_mut(fixture, "control_copy"),
                "path",
                text("linked.yml"),
            );
        }
        Mutation::SurfaceMissingId => {
            remove_field(secondary_copy_mut(fixture, "surface_copy"), "surface_id")
        }
        Mutation::UnknownSecondarySurface => set_field(
            secondary_copy_mut(fixture, "surface_copy"),
            "surface_id",
            text("missing_surface"),
        ),
        Mutation::HistoricalSecondary => {
            let copy = secondary_copy_mut(fixture, "surface_copy");
            set_field(copy, "surface_id", text("historical_surface"));
            set_field(copy, "path", text("history.md"));
            set_field(copy, "field_marker", text("# Historical terminal"));
        }
        Mutation::OffSurfaceSecondary => {
            let copy = secondary_copy_mut(fixture, "surface_copy");
            set_field(copy, "path", text("current.md"));
            set_field(copy, "field_marker", text("# Current"));
        }
        Mutation::ControlSurfaceId => set_field(
            secondary_copy_mut(fixture, "control_copy"),
            "surface_id",
            text("secondary_surface"),
        ),
        Mutation::ControlMarkdown => {
            let copy = secondary_copy_mut(fixture, "control_copy");
            set_field(copy, "path", text("secondary.md"));
            set_field(copy, "field_marker", text("Surface copy: yes"));
        }
        Mutation::SecondaryMissingMarker => set_field(
            secondary_copy_mut(fixture, "control_copy"),
            "field_marker",
            text("control_copy: missing"),
        ),
        Mutation::SecondaryDuplicateMarker => write_text(
            &fixture.root,
            "control.yml",
            "control_copy: yes\ncontrol_copy: yes\n",
        )?,
        Mutation::PrimarySecondaryDuplicateMarker => {
            let copy = secondary_copy_mut(fixture, "control_copy");
            set_field(copy, "path", text("current.md"));
            set_field(copy, "field_marker", text("Verified adapter: yes"));
        }
        Mutation::UnsafePrimaryPath => set_field(
            contract_mut(fixture, "intent"),
            "path",
            text("../escape.md"),
        ),
        Mutation::UnknownSurface => set_field(
            contract_mut(fixture, "intent"),
            "surface_id",
            text("unknown_surface"),
        ),
        Mutation::HistoricalPrimary => {
            let record = contract_mut(fixture, "intent");
            set_field(record, "surface_id", text("historical_surface"));
            set_field(record, "path", text("history.md"));
            set_field(record, "field_marker", text("# Historical terminal"));
        }
        Mutation::OffSurfacePrimary => {
            let record = contract_mut(fixture, "intent");
            set_field(record, "path", text("evidence.md"));
            set_field(record, "field_marker", text("# Evidence"));
        }
        Mutation::MissingPrimaryFile => {
            set_field(
                &mut fixture.surfaces[0],
                "targets",
                Py::List(vec![text("missing.md")]),
            );
            set_field(contract_mut(fixture, "intent"), "path", text("missing.md"));
        }
        Mutation::MissingPrimaryMarker => set_field(
            contract_mut(fixture, "intent"),
            "field_marker",
            text("Missing intent marker"),
        ),
        Mutation::DuplicatePrimaryMarker => write_text(
            &fixture.root,
            "current.md",
            &(read_text(&fixture.root, "current.md")? + "Authored intent: selected\n"),
        )?,
        Mutation::StoredRevision => write_text(
            &fixture.root,
            "current.md",
            &(read_text(&fixture.root, "current.md")? + "Stored revision: abc123\n"),
        )?,
        Mutation::MissingAccessor => set_field(
            contract_mut(fixture, "revision"),
            "accessor",
            text("fixture-missing-accessor"),
        ),
        Mutation::WrongDeriveVerifier => set_field(
            contract_mut(fixture, "revision"),
            "verifier",
            text("core:scripts/core-ok.pl"),
        ),
        Mutation::InvalidVerifierScheme => set_field(
            contract_mut(fixture, "core_copy"),
            "verifier",
            text("external:trust-me"),
        ),
        Mutation::MissingCore => set_field(
            contract_mut(fixture, "core_copy"),
            "verifier",
            text("core:scripts/missing.pl"),
        ),
        Mutation::FailingCore => set_field(
            contract_mut(fixture, "core_copy"),
            "verifier",
            text("core:scripts/core-fail.pl"),
        ),
        Mutation::FailingAdapter => set_field(
            contract_mut(fixture, "adapter_copy"),
            "verifier",
            text("adapter:scripts/adapter-fail.pl"),
        ),
        Mutation::IntentVerifier => set_field(
            contract_mut(fixture, "intent"),
            "verifier",
            text("builtin:none"),
        ),
        Mutation::MissingCaptureBoundary => set_field(
            contract_mut(fixture, "evidence"),
            "capture_boundary",
            text("Missing capture boundary"),
        ),
        Mutation::RecordCountBound => {
            set_field(&mut fixture.derived_meta, "max_records", integer(4))
        }
        Mutation::TotalByteBound => set_field(&mut fixture.derived_meta, "max_bytes", integer(512)),
        Mutation::RawRecordBound => {
            set_field(&mut fixture.derived_meta, "max_record_bytes", integer(128))
        }
        Mutation::ScalarBound => {
            set_field(&mut fixture.derived_meta, "max_scalar_bytes", integer(8))
        }
        Mutation::SchemaVersion => {
            set_field(&mut fixture.derived_meta, "schema_version", integer(2))
        }
    }
    Ok(())
}

fn matches_in_order(output: &str, patterns: &[&str]) -> bool {
    let mut remainder = output;
    for pattern in patterns {
        let Some(index) = remainder.find(pattern) else {
            return false;
        };
        remainder = &remainder[index + pattern.len()..];
    }
    true
}

fn report_result(
    name: &str,
    number: usize,
    passed: bool,
    output: &str,
    quiet: bool,
    output_sink: &mut dyn Output,
) {
    if passed {
        if !quiet {
            output_sink.out(&format!("ok {number} - {name}\n"));
        }
    } else {
        output_sink.out(&format!("not ok {number} - {name}\n"));
        for line in output.lines() {
            output_sink.out(&format!("# {line}\n"));
        }
    }
}

fn fixture_root(root: &Path, number: usize) -> PathBuf {
    root.join("generated").join(format!(
        "derived-state-tests.{}.{number}",
        std::process::id()
    ))
}

/// Run all 47 checks with the original `--quiet` or verbose output contract.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn Output) -> i32 {
    let quiet = match arguments {
        [] => false,
        [argument] if argument == "--quiet" => true,
        _ => {
            output.err("Usage: cargo xtask check derived-state-contract-tests [--quiet]\n");
            return 2;
        }
    };
    if let Err(error) = fs::create_dir_all(root.join("generated")) {
        output.err(&format!("cannot create generated fixture root: {error}\n"));
        return 1;
    }
    let mut number = 0;
    let mut failures = 0;
    let source = match fs::read_to_string(root.join("scripts/check_derived_state_contracts.pl")) {
        Ok(source) => source,
        Err(error) => {
            output.err(&format!("cannot read checker source: {error}\n"));
            return 1;
        }
    };
    let project_terms: Vec<&str> = [
        "rust_prerequisite_copies",
        "fsmgen_gitlink_copies",
        "rust_book",
        "rust_ci",
        "feedback_contract_json",
    ]
    .into_iter()
    .filter(|term| source.contains(term))
    .collect();
    number += 1;
    if !project_terms.is_empty() {
        failures += 1;
    }
    report_result(
        "neutral checker contains no project contract identifiers or adapter roles",
        number,
        project_terms.is_empty(),
        &format!("project terms: {}\n", project_terms.join(" ")),
        quiet,
        output,
    );

    let fixture = match new_fixture(&fixture_root(root, 2)) {
        Ok(fixture) => fixture,
        Err(error) => {
            output.err(&format!("{error}\n"));
            return 1;
        }
    };
    let (status, checker_output) = run_checker(&fixture);
    let passed = status == 0
        && fixture.root.join("core-proof.txt").is_file()
        && fixture.root.join("adapter-proof.txt").is_file();
    number += 1;
    if !passed {
        failures += 1;
    }
    report_result(
        "all four classifications and both verifier kinds pass",
        number,
        passed,
        &checker_output,
        quiet,
        output,
    );
    drop(fixture);

    for case in CASES {
        let mut fixture = match new_fixture(&fixture_root(root, number + 1)) {
            Ok(fixture) => fixture,
            Err(error) => {
                output.err(&format!("{error}\n"));
                return 1;
            }
        };
        if let Err(error) = mutate(&mut fixture, case.mutation) {
            output.err(&format!("{error}\n"));
            return 1;
        }
        if let Err(error) = save_fixture(&fixture) {
            output.err(&format!("{error}\n"));
            return 1;
        }
        let (status, checker_output) = run_checker(&fixture);
        let mut passed = if case.success {
            status == 0
        } else {
            status != 0
        };
        passed &= matches_in_order(&checker_output, case.patterns);
        number += 1;
        if !passed {
            failures += 1;
        }
        report_result(case.name, number, passed, &checker_output, quiet, output);
    }
    if !quiet {
        output.out(&format!("1..{number}\n"));
    }
    if failures > 0 {
        output.err(&format!(
            "derived-state-tests: {failures} of {number} checks failed\n"
        ));
        return 1;
    }
    if number != 47 {
        output.err(&format!(
            "derived-state-tests: ran {number} checks, declaration expects 47 — re-derive the declaration beside the suite\n"
        ));
        return 1;
    }
    output.err("derived-state-tests: 47/47 classification and authority-control checks pass\n");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_forty_seven_checks_pass_without_fixture_residue() {
        let root = crate::repository_root();
        let prefix = format!("derived-state-tests.{}.", std::process::id());
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--quiet".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert_eq!(
            output.text,
            "derived-state-tests: 47/47 classification and authority-control checks pass\n"
        );
        let residue = fs::read_dir(root.join("generated"))
            .expect("generated fixture root")
            .filter_map(Result::ok)
            .any(|entry| entry.file_name().to_string_lossy().starts_with(&prefix));
        assert!(!residue, "native derived-state fixtures were not cleaned");
    }
}
