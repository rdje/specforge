//! EXTRACTOR-ARCHITECTURE.2 — the unified extractor framework.
//!
//! The EvidenceIR producer layer grew as a flat bank of ~60 free functions wired imperatively inside one
//! ~500-line `EvidenceIr::build()`, with hand-rolled per-surface merge/dedup and heterogeneous applicability
//! gates (the audit: `docs/tasks/EXTRACTOR-ARCHITECTURE.md` `.1`; KM `extractor-path-architecture`). The FSM
//! cluster — four sibling functions glued by four separate inline dedup-by-name loops at the call site — was
//! the canonical "grows erratically in every direction" failure mode.
//!
//! Per the owner directive (`2026-06-09`, "consolidate, unify, as much as possible") this module promotes the
//! implicit *strategy → gate → merge* shape into ONE coherent frame:
//!
//! - [`Extractor`] — a single extractor unit (one grammar / strategy) for a surface record type `R`. It
//!   declares its stable [`Extractor::name`], its provenance [`Extractor::tier`], an explicit
//!   [`Extractor::applies_to`] gate, and the grammar [`Extractor::run`].
//! - [`ExtractionContext`] — the immutable inputs every extractor reads, built ONCE per EvidenceIR build.
//! - [`run_surface`] — the single driver: for each registered extractor, check the gate, run it, merge the
//!   results into one surface inventory (first-wins dedup by a per-surface key), and record an inspectable
//!   [`SurfaceRun`] manifest of exactly what each extractor was eligible for and contributed.
//!
//! A new PDF family then grows by *adding one [`Extractor`] to a surface's registry list in one place* — never
//! by editing a god-function or sprinkling a new inline dedup loop. The bounded LLM/VLM tiers are the same
//! frame ([`Extractor::tier`] = [`ExtractorTier::Nlp`] / [`ExtractorTier::Vlm`], with an `applies_to` that
//! checks provider availability). This is a refactor, not a rewrite: the grammars and the typed IR target are
//! sound and unchanged; only the structure that wires them becomes first-class.
//!
//! This `.2` slice lands the frame and its tests; it migrates ZERO extractors (no behavior change). `.3`
//! migrates the FSM cluster onto it, proving byte-identical output.

use crate::ir::evidence::{ExtractedStatement, ExtractorTier};
use std::collections::BTreeSet;

/// The immutable inputs shared by every extractor, assembled once per `EvidenceIR` build.
///
/// It carries exactly what the migrated clusters need and **grows by one field per migrated cluster** (the
/// next clusters add e.g. the normalized `SourceIr`, known signal/actor names, the alias map, visual
/// evidence, prior guidance) — never a grab bag of speculative fields. Everything is borrowed: an extractor
/// reads, it does not own the build. The first migrated cluster (FSM, `.3`) reads only `statements`.
pub struct ExtractionContext<'a> {
    /// The extracted statements produced earlier in the build (prose + table-synthesized facts).
    pub statements: &'a [ExtractedStatement],
}

/// One extractor unit: a single grammar / strategy that produces records of surface type `R`.
///
/// Implementors are normally zero-sized structs (the grammar is in [`Extractor::run`]); the trait is generic
/// over the record type so each surface keeps its own typed records (no lossy `enum`-of-everything). A
/// surface's extractors form a homogeneous `&[&dyn Extractor<R>]` registry that [`run_surface`] drives.
pub trait Extractor<R> {
    /// Stable, dotted identifier, e.g. `"fsm.transition_bound"`. Surfaced in the run manifest and (as
    /// migration reaches them) in uniform fact provenance, so "which extractor produced this" is answerable.
    fn name(&self) -> &'static str;

    /// Which independent extractor family this belongs to (uniform provenance across every surface).
    /// Defaults to [`ExtractorTier::Pattern`]; LLM/VLM extractors override it.
    fn tier(&self) -> ExtractorTier {
        ExtractorTier::Pattern
    }

    /// The explicit, inspectable applicability gate. Defaults to `true`: a self-gating extractor (one that
    /// returns `[]` when its grammar does not match the document) needs no separate gate. Extractors with a
    /// cheap document-level precondition (a provider being available, a keyword present) override this so the
    /// driver can skip `run` entirely and record the skip in the manifest.
    fn applies_to(&self, _cx: &ExtractionContext<'_>) -> bool {
        true
    }

    /// Produce this extractor's records from the shared context. This is the grammar — the only place a
    /// migrated free function's body moves to. Must not fabricate: it returns only what the document grounds.
    fn run(&self, cx: &ExtractionContext<'_>) -> Vec<R>;
}

/// What one extractor was and did in a [`run_surface`] pass — the unit of the inspectable manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtractorRunEntry {
    /// The extractor's stable id ([`Extractor::name`]).
    pub name: &'static str,
    /// Its provenance tier ([`Extractor::tier`]).
    pub tier: ExtractorTier,
    /// Whether its [`Extractor::applies_to`] gate passed (a skipped extractor recorded `eligible: false`).
    pub eligible: bool,
    /// How many records it produced (before cross-extractor dedup).
    pub produced: usize,
    /// How many of those survived the cross-extractor first-wins dedup (the rest were duplicates of records
    /// an earlier extractor in the registry already contributed).
    pub kept: usize,
}

/// The result of running ONE surface's extractor registry through [`run_surface`]: the merged record
/// inventory plus a per-extractor manifest. `R`-typed; call [`SurfaceRun::manifest`] for the `R`-free view
/// the build aggregates across surfaces.
#[derive(Debug, Clone)]
pub struct SurfaceRun<R> {
    /// The surface label, e.g. `"protocol_states"`.
    pub surface: &'static str,
    /// The merged, deduped records — the surface inventory the build consumes.
    pub records: Vec<R>,
    /// How many registered extractors were eligible (passed their gate).
    pub eligible: usize,
    /// One entry per registered extractor, in registry order.
    pub entries: Vec<ExtractorRunEntry>,
}

impl<R> SurfaceRun<R> {
    /// The `R`-free manifest view (surface label + eligible count + per-extractor entries) the build collects
    /// across every surface into one [`ExtractionManifest`] for `validate` / `audit-extraction` to surface.
    pub fn manifest(&self) -> SurfaceManifest {
        SurfaceManifest {
            surface: self.surface,
            eligible: self.eligible,
            entries: self.entries.clone(),
        }
    }
}

/// The `R`-free manifest for a single surface (see [`SurfaceRun::manifest`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SurfaceManifest {
    /// The surface label.
    pub surface: &'static str,
    /// How many registered extractors were eligible.
    pub eligible: usize,
    /// One entry per registered extractor, in registry order.
    pub entries: Vec<ExtractorRunEntry>,
}

/// The whole-build manifest: one [`SurfaceManifest`] per surface, in build order. This is the inspectable
/// "which extractors fired and what each contributed" view the audit found missing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ExtractionManifest {
    /// Per-surface manifests, in the order the build ran the surfaces.
    pub surfaces: Vec<SurfaceManifest>,
}

impl ExtractionManifest {
    /// Append one surface's manifest (called by the build after each [`run_surface`]).
    pub fn record<R>(&mut self, run: &SurfaceRun<R>) {
        self.surfaces.push(run.manifest());
    }
}

/// THE driver — the single place a surface's extractors are wired, gated, run, and merged.
///
/// For each extractor in `extractors` (in order): if [`Extractor::applies_to`] fails, record an ineligible
/// entry and skip; otherwise run it and fold its records into the surface inventory with **first-wins dedup**
/// keyed by `key` (a record whose key was already contributed by an earlier extractor is dropped). The order
/// of `extractors` therefore defines precedence — list the authoritative/structural extractors first. Returns
/// the merged inventory plus the per-extractor manifest.
///
/// This replaces the per-surface hand-rolled merge loops (e.g. the four inline FSM dedup loops): a surface's
/// merge policy is now just its `key` function passed here, in one call.
pub fn run_surface<R, K, KF>(
    surface: &'static str,
    cx: &ExtractionContext<'_>,
    extractors: &[&dyn Extractor<R>],
    key: KF,
) -> SurfaceRun<R>
where
    K: Ord,
    KF: Fn(&R) -> K,
{
    let mut records: Vec<R> = Vec::new();
    let mut seen: BTreeSet<K> = BTreeSet::new();
    let mut entries: Vec<ExtractorRunEntry> = Vec::with_capacity(extractors.len());
    let mut eligible = 0usize;
    for extractor in extractors {
        let tier = extractor.tier();
        let name = extractor.name();
        if !extractor.applies_to(cx) {
            entries.push(ExtractorRunEntry {
                name,
                tier,
                eligible: false,
                produced: 0,
                kept: 0,
            });
            continue;
        }
        eligible += 1;
        let produced = extractor.run(cx);
        let produced_count = produced.len();
        let mut kept = 0usize;
        for record in produced {
            if seen.insert(key(&record)) {
                records.push(record);
                kept += 1;
            }
        }
        entries.push(ExtractorRunEntry {
            name,
            tier,
            eligible: true,
            produced: produced_count,
            kept,
        });
    }
    SurfaceRun {
        surface,
        records,
        eligible,
        entries,
    }
}

#[cfg(test)]
mod tests {
    //! EXTRACTOR-ARCHITECTURE.2 — the framework drives gate/run/merge/manifest correctly. Uses a toy record
    //! plus toy extractors (no real grammar) so the DRIVER is tested in isolation; cluster migrations
    //! (`.3` onward) verify each real grammar's output byte-identically against the corpus.
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct Toy {
        name: String,
    }

    fn toy(name: &str) -> Toy {
        Toy {
            name: name.to_string(),
        }
    }

    /// An extractor that always applies and emits a fixed list (its grammar is "return these").
    struct Fixed {
        id: &'static str,
        out: Vec<Toy>,
    }
    impl Extractor<Toy> for Fixed {
        fn name(&self) -> &'static str {
            self.id
        }
        fn run(&self, _cx: &ExtractionContext<'_>) -> Vec<Toy> {
            self.out.clone()
        }
    }

    /// An extractor gated OFF — the driver must skip `run` and record it ineligible.
    struct Gated {
        id: &'static str,
    }
    impl Extractor<Toy> for Gated {
        fn name(&self) -> &'static str {
            self.id
        }
        fn applies_to(&self, _cx: &ExtractionContext<'_>) -> bool {
            false
        }
        fn tier(&self) -> ExtractorTier {
            ExtractorTier::Vlm
        }
        fn run(&self, _cx: &ExtractionContext<'_>) -> Vec<Toy> {
            panic!("gated extractor must not be run");
        }
    }

    fn ctx(statements: &[ExtractedStatement]) -> ExtractionContext<'_> {
        ExtractionContext { statements }
    }

    fn key(t: &Toy) -> String {
        t.name.to_ascii_uppercase()
    }

    #[test]
    fn merges_and_dedups_first_wins_across_extractors() {
        let stmts: Vec<ExtractedStatement> = vec![];
        let cx = ctx(&stmts);
        let a = Fixed {
            id: "a",
            out: vec![toy("X"), toy("Y")],
        };
        // "x" duplicates a's "X" (case-insensitive key) → dropped; "Z" is new → kept.
        let b = Fixed {
            id: "b",
            out: vec![toy("x"), toy("Z")],
        };
        let run = run_surface("toy", &cx, &[&a, &b], key);
        assert_eq!(
            run.records
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>(),
            vec!["X", "Y", "Z"],
            "first-wins dedup preserves a's X over b's x"
        );
        assert_eq!(run.eligible, 2);
        assert_eq!(run.entries[0].produced, 2);
        assert_eq!(run.entries[0].kept, 2);
        assert_eq!(run.entries[1].produced, 2);
        assert_eq!(run.entries[1].kept, 1, "b's duplicate dropped");
    }

    #[test]
    fn gate_skips_run_and_records_ineligible() {
        let stmts: Vec<ExtractedStatement> = vec![];
        let cx = ctx(&stmts);
        let a = Fixed {
            id: "a",
            out: vec![toy("X")],
        };
        let gated = Gated { id: "g" }; // its run() panics if called
        let run = run_surface("toy", &cx, &[&a, &gated], key);
        assert_eq!(run.eligible, 1);
        assert_eq!(run.records.len(), 1);
        let g = run.entries.iter().find(|e| e.name == "g").unwrap();
        assert!(!g.eligible);
        assert_eq!(g.produced, 0);
        assert_eq!(g.tier, ExtractorTier::Vlm);
    }

    #[test]
    fn registry_order_defines_precedence_in_manifest() {
        let stmts: Vec<ExtractedStatement> = vec![];
        let cx = ctx(&stmts);
        let a = Fixed {
            id: "first",
            out: vec![toy("DUP")],
        };
        let b = Fixed {
            id: "second",
            out: vec![toy("dup")],
        };
        let run = run_surface("toy", &cx, &[&a, &b], key);
        assert_eq!(
            run.records.len(),
            1,
            "the duplicate from `second` is dropped"
        );
        assert_eq!(run.entries[0].name, "first");
        assert_eq!(run.entries[0].kept, 1);
        assert_eq!(run.entries[1].name, "second");
        assert_eq!(run.entries[1].kept, 0);
    }

    #[test]
    fn aggregate_manifest_collects_every_surface() {
        let stmts: Vec<ExtractedStatement> = vec![];
        let cx = ctx(&stmts);
        let a = Fixed {
            id: "a",
            out: vec![toy("X")],
        };
        let run = run_surface("toy", &cx, &[&a], key);
        let mut manifest = ExtractionManifest::default();
        manifest.record(&run);
        assert_eq!(manifest.surfaces.len(), 1);
        assert_eq!(manifest.surfaces[0].surface, "toy");
        assert_eq!(manifest.surfaces[0].eligible, 1);
        assert_eq!(manifest.surfaces[0].entries[0].name, "a");
    }
}
