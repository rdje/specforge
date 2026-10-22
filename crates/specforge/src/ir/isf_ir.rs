// ISF IR — typed intermediate representation for Intent Scheduling Format
// =========================================================================
// Flow:  IntentIR  →  IsfIr::from_intent_ir()  →  IsfIr  →  IsfIr::render()  →  .isf text
//
// This IR eliminates string-bashing bugs by design:
//   - BTreeSet<IsfSignal>        → dedup by construction (no duplicate clock/reset)
//   - IsfReset (non-optional)    → compiler enforces presence (strict mode requires it)
//   - Typed IsfRule → cannot emit invalid S-expression syntax
//   - Recursive tree walk        → parentheses match by construction

use std::collections::{BTreeMap, BTreeSet};

use crate::ir::evidence::{CheckSignalRecord, ParitySense, SignalPolarity, SignalPolarityRecord};
use crate::ir::intent::{
    ConstraintLevel, IntentIr, TemporalInvariantKind, TransactionStep, signal_constraint_level,
};
use crate::ir::semantic::{
    ActorPortRecord, ActorRelativeDirection, ControlActionRecord, ControlBinaryOperator,
    ControlBranchRecord, ControlCompoundUpdateOperation, ControlExpressionRecord,
    ControlReferenceSuffix, ControlUnaryOperator, InfrastructureSignalKind,
    InterfaceSignalDirection, OperationKindRecord, ParameterValueRange, SignalFamilyRecord,
    SymbolDefinitionKind, SymbolDefinitionRecord, SystemResetKind, SystemResetPolarity,
    operation_kind_condition,
};
// R16-CONTRACT-IR.3: `TemporalRuleRecord`/`TemporalPredicateRecord` are now
// referenced only by the test-only parity oracle (`classify_temporal_rule`
// + helpers) and the test module — production lowering uses ContractIR.
#[cfg(test)]
use crate::ir::semantic::{TemporalPredicateRecord, TemporalRuleRecord};
use crate::ir::source::{
    AutomationConfidence, CandidateInterpretation, RegisterFieldRecord, RegisterRecord,
    ResidualDecisionPacket, WidthHint,
};

// ---------------------------------------------------------------------------
// ISF-IR data types
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum IsfDirection {
    Input,
    Output,
}

impl IsfDirection {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Output => "output",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct IsfSignal {
    name: String,
    direction: IsfDirection,
    /// The width FSMGen elaborates: a concrete grounded width, or the declared default of
    /// `width_param` when the document writes the width as a parameter.
    width: u32,
    /// KG-ISF-COMPLETENESS.2a.vii.b: the actor parameter that sizes this signal, rendered as
    /// `(width P)` and declared in the actor's `(params …)` block. `None` renders `(width N)`.
    width_param: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfReset {
    signal: String,
    timing: String,   // "sync" | "async"
    polarity: String, // "active_high" | "active_low"
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfConstant {
    name: String,
    value: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfTypeDef {
    name: String,
    bits: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfEnum {
    type_name: String,
    members: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfStorageVar {
    name: String,
    width: u32,
    /// The register's documented hardware reset value, lowered to FSMGen's optional
    /// `(reset V)` ONLY when it is a clean in-width non-negative integer composed from the
    /// register's per-field `reset_value`s (ISF-REGISTER-RESET-EMIT.2). `None` leaves the var
    /// reset-less — FSMGen then defaults it to all-0s, byte-identical to the pre-`.2` output.
    reset: Option<u64>,
    /// The register's named bit-fields, lowered to FSMGen's declarative
    /// `(fields (field …))` storage construct (DOC-INTENT-TAXONOMY.4a.ii; FSMGen pin
    /// `d327129b7`). EMPTY leaves the var an opaque `(var NAME (width N) [(reset V)])`,
    /// byte-identical to the pre-`.4a.ii` output. The field block is metadata-only /
    /// schedule-safe: FSMGen's scheduled `.fsm` is byte-identical with vs without it.
    fields: Vec<IsfStorageField>,
}

/// One named bit-field within an [`IsfStorageVar`], lowered to FSMGen's
/// `(field NAME (bits HI LO) [(access …)] [(reset V)] [(enum (M V)…)])`
/// (DOC-INTENT-TAXONOMY.4a.ii). Only fields admitted by `register_storage_fields`
/// (located, unique-named, non-overlapping, in-width) are represented here.
#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfStorageField {
    /// Sanitized HDL identifier, unique within the parent var.
    name: String,
    /// Most-significant bit position (inclusive), `< parent width`.
    msb: u32,
    /// Least-significant bit position (inclusive), `<= msb`.
    lsb: u32,
    /// Access policy normalized to FSMGen's token set
    /// (`ro|rw|wo|w1c|w0c|rc|rs|warl|wpri|reserved`); `None` (omit) when the
    /// document's access notation does not map — honest, never guessed.
    access: Option<String>,
    /// Field reset, emitted ONLY when the parent var carries a composed `(reset V)`
    /// (FSMGen requires a field `(reset)` to match the parent reset bit slice); the
    /// value IS that slice, so the match holds by construction.
    reset: Option<u64>,
    /// Enumerated `(MEMBER VALUE)` encodings whose value fits the field width;
    /// member names are sanitized + deduped. Empty → no `(enum …)` clause.
    enum_members: Vec<(String, u64)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfNamedDrive {
    name: String,
    body: Vec<(String, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfTransaction {
    name: String,
    on_trigger: Option<String>,
    on_steps: Vec<IsfTxnStep>,
    steps: Vec<IsfTxnStep>,
    complete: String,
    latency_min: Option<u64>,
    latency_max: Option<u64>,
    // Spec §11.8 transaction-internal bounded-eventually contracts.
    contracts: Vec<IsfContract>,
    // Spec §11.8 transaction-internal ready/valid stages
    // (`ready_valid_barrier`). FSMGen ACCEPTS `(stage p (ready r)(valid
    // v))` as of pin `9bfb9a20` (verified `FSMGEN-SUBMODULE-BUMP.1`);
    // `R16-CONTRACT-IR.4` lowers `HandshakeBarrier` here.
    stages: Vec<IsfStage>,
}

// A transaction-level FSMGen verification-family property, rendered as
// `(assert <prop>)`. `prop` is pre-built by the temporal classifier
// (`windowed_eventual_prop`): the anchored monitor `(monitor (within s N))` for
// an unguarded bounded-eventually, or the guarded implication
// `(=> g (within s [min] max))` (FSMGEN-ASSERT-LOWERING). The `(contract …
// (eventually …))` clause it replaced was removed upstream (FSMGen 0008/0009).
#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfContract {
    prop: String,
}

// `(stage <name> (ready <ready>) (valid <valid>))` — FSMGen ISF spec
// §11.8 shipped kind `ready_valid_barrier`. Strict-REJECTED at the old
// pin `effe591d` (logged in docs/FSMGEN_FEEDBACK.md); FSMGen FIXED it
// (`d4d6dfab`) and SPECFORGE verified acceptance at pin `9bfb9a20`
// (`FSMGEN-SUBMODULE-BUMP.1`). `R16-CONTRACT-IR.4` lowers a
// `HandshakeComplete`/`HandshakeBarrier` obligation to this.
#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfStage {
    name: String,
    ready: String,
    valid: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum IsfTxnStep {
    Drive {
        name: String,
        actuals: Vec<String>,
    },
    When {
        condition: String,
        body: Vec<IsfTxnStep>,
    },
    Switch {
        selector: String,
        branches: Vec<(String, Vec<IsfTxnStep>)>,
    },
    While {
        condition: String,
        body: Vec<IsfTxnStep>,
    },
    Until {
        condition: String,
        body: Vec<IsfTxnStep>,
    },
    Repeat {
        count: String,
        body: Vec<IsfTxnStep>,
    },
    Await {
        port: String,
        watchdog: Option<u64>,
    },
    Wait {
        count: String,
    },
    Sample {
        port: String,
        as_name: String,
    },
    Do {
        child_transaction: String,
    },
    Spawn {
        child_transaction: String,
        instance: String,
    },
    Set {
        target: String,
        expr: String,
    },
    Update {
        target: String,
        expr: String,
    },
    ShiftLeft {
        reg: String,
        bit: String,
    },
    ShiftRight {
        reg: String,
        bit: String,
        width: Option<u8>,
    },
    Complete {
        port: String,
    },
    AwaitAll {
        done_port: String,
    },
    AwaitAny {
        done_port: String,
    },
    Latency {
        min: u64,
        max: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfRule {
    name: String,
    condition: String,
    drives: Vec<(String, String)>,
}

pub(crate) struct IsfIr {
    actor_name: String,
    clock: String,
    reset: IsfReset,
    watchdog: u64,
    signals: BTreeSet<IsfSignal>,
    /// KG-ISF-COMPLETENESS.2a.vii.b: the actor `(params (P default))` that size parameterised
    /// signals, keyed by parameter name.
    width_params: BTreeMap<String, u32>,
    constants: Vec<IsfConstant>,
    types: Vec<IsfTypeDef>,
    enums: Vec<IsfEnum>,
    storage: Vec<IsfStorageVar>,
    drives: Vec<IsfNamedDrive>,
    transactions: Vec<IsfTransaction>,
    rules: Vec<IsfRule>,
    /// Temporal rules that have no representable supported ISF construct
    /// (ISF-TEMPORAL-LOWERING.2.3 mapping #4). They are NOT rendered into
    /// `.isf`; they are preserved here so the adapter artifact records the
    /// dropped obligation as an explicit residual decision instead of
    /// fabricating unsupported syntax.
    temporal_residuals: Vec<ResidualDecisionPacket>,
    /// Register resets that were NOT lowered to a storage `(reset V)`
    /// (ISF-REGISTER-RESET-EMIT.2): a single proportionate summary packet when ≥1
    /// register carried a documented field reset that is symbolic / partially covered
    /// / over-wide-for-its-var-width, so the adapter artifact records the honest gap
    /// instead of fabricating a power-up value.
    storage_reset_residuals: Vec<ResidualDecisionPacket>,
    /// Register bit-fields that were NOT lowered to the storage `(fields …)` block
    /// (DOC-INTENT-TAXONOMY.4a.ii): a single proportionate summary packet when ≥1
    /// captured field is an honest residual — unlocated (no bit range), ambiguous
    /// (a sanitized name shared by ≥2 fields), or in an overlap-failed-closed
    /// register — so the adapter artifact records the gap instead of fabricating a
    /// bit position the document never states.
    storage_field_residuals: Vec<ResidualDecisionPacket>,
    /// Signal widths the document writes symbolically (KG-ISF-COMPLETENESS.2a.vii.b): a summary of
    /// the parameter defaults that are placeholders, and of the widths that could not be lowered to
    /// a parameter (a derived expression such as `DATA_WIDTH/8`, or a name that collides with a
    /// declared one). Neither is rendered as a fact; both are surfaced as residual decisions.
    signal_width_residuals: Vec<ResidualDecisionPacket>,
    /// EXTRACTION-REPRESENTATION-GAPS.27.e — interface signals left out of the lowered actor because the document
    /// grounds no direction for them on it.
    interface_residuals: Vec<ResidualDecisionPacket>,
}

// ---------------------------------------------------------------------------
// ISF-IR emitter — renders the typed IR to valid ISF S-expression text
// ---------------------------------------------------------------------------

impl IsfIr {
    /// Temporal rules that could not be lowered to a supported ISF construct,
    /// preserved as explicit residual decisions
    /// (ISF-TEMPORAL-LOWERING.2.3 mapping #4). The adapter artifact appends
    /// these to its residual-decision set so a dropped temporal obligation is
    /// visible rather than silently lost.
    pub(crate) fn temporal_residuals(&self) -> &[ResidualDecisionPacket] {
        &self.temporal_residuals
    }

    /// Register resets that could not be lowered to a storage `(reset V)`
    /// (ISF-REGISTER-RESET-EMIT.2). The adapter artifact appends these to its
    /// residual-decision set so a dropped reset is visible rather than silently lost.
    pub(crate) fn storage_reset_residuals(&self) -> &[ResidualDecisionPacket] {
        &self.storage_reset_residuals
    }

    /// Register bit-fields that could not be lowered to the storage `(fields …)` block
    /// (DOC-INTENT-TAXONOMY.4a.ii). The adapter artifact appends these to its
    /// residual-decision set so a dropped field map is visible rather than silently lost.
    pub(crate) fn storage_field_residuals(&self) -> &[ResidualDecisionPacket] {
        &self.storage_field_residuals
    }

    /// Symbolic signal widths whose default is a placeholder, or that stay `(width 1)` because no
    /// parameter can state them (KG-ISF-COMPLETENESS.2a.vii.b). The adapter artifact appends these
    /// to its residual-decision set.
    pub(crate) fn signal_width_residuals(&self) -> &[ResidualDecisionPacket] {
        &self.signal_width_residuals
    }

    /// EXTRACTION-REPRESENTATION-GAPS.27.e — the interface signals the lowered actor does not declare, because the
    /// document grounds no direction for them on it. The adapter artifact appends these to its residual-decision set.
    pub(crate) fn interface_residuals(&self) -> &[ResidualDecisionPacket] {
        &self.interface_residuals
    }

    /// Number of transactions the emitter actually renders (includes the
    /// temporal-synthesized `(contract …)` transactions). The adapter
    /// artifact MUST report this, not a blind IntentIR-derived guess
    /// (ISF-TEMPORAL-LOWERING.2.4 — no metric counts a surface the emitter
    /// ignores).
    pub(crate) fn emitted_transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Number of actor `(rule …)` forms the emitter actually renders,
    /// post-dedup — including temporal value/guard→drive rules
    /// (ISF-TEMPORAL-LOWERING.2.3/.2.4).
    pub(crate) fn emitted_rule_count(&self) -> usize {
        self.rules.len()
    }

    /// The `(constants …)` entries `render()` actually emits: those whose
    /// value is a whitespace-free scalar (operator-expression values are
    /// excluded — see `is_safe_isf_scalar_value`). Shared with `render()`
    /// and `emitted_constant_count()` so the metric == emitted content.
    fn emitted_constants(&self) -> Vec<&IsfConstant> {
        self.constants
            .iter()
            .filter(|c| is_safe_isf_scalar_value(&c.value))
            .collect()
    }

    /// The `(enums …)` families `render()` actually emits (see [`isf_enum_is_emittable`]).
    fn emitted_enums(&self) -> Vec<&IsfEnum> {
        self.enums
            .iter()
            .filter(|e| isf_enum_is_emittable(e))
            .collect()
    }

    /// Number of `(constants …)` entries the emitter actually renders
    /// (metric == emitted content; mirrors `emitted_rule_count`).
    pub(crate) fn emitted_constant_count(&self) -> usize {
        self.emitted_constants().len()
    }

    /// Number of `(enums …)` families the emitter actually renders.
    pub(crate) fn emitted_enum_count(&self) -> usize {
        self.emitted_enums().len()
    }

    /// Residual packets for enums DROPPED by the value-width gate (KG-ISF-COMPLETENESS.2a.iv):
    /// an enum every member of which is a safe scalar (so the pre-`.2a.iv` emitter WOULD have
    /// emitted it) but at least one bare-decimal member overflows the enum's declared width —
    /// i.e. FSMGen would reject the literal. The whole enum is held out of the `.isf` and
    /// recorded here so the dropped surface is explicit, not silently lost. Enums dropped for the
    /// pre-existing reason (an operator-expression member value) keep their prior silent-exclusion
    /// behaviour, so those docs' adapter residual surface is byte-identical.
    pub(crate) fn enum_residuals(&self) -> Vec<ResidualDecisionPacket> {
        self.enums
            .iter()
            .filter(|e| {
                !e.members.is_empty()
                    && e.members.iter().all(|(_, v)| is_safe_isf_scalar_value(v))
                    && !isf_enum_is_emittable(e)
            })
            .map(enum_value_literal_residual_packet)
            .collect()
    }

    pub(crate) fn render(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        lines.push(format!("(actor {}", self.actor_name));

        // KG-ISF-COMPLETENESS.2a.vii.b: actor scalar parameters that size parameterised signals
        // (FSMGen book 13a, *Interface*: `(params (DATA_W 32))` with `(width DATA_W)`). Declared
        // first, as the book's actor shape does; absent when no signal is parameterised, so every
        // other document's `.isf` is byte-identical.
        if !self.width_params.is_empty() {
            lines.push("  (params".to_string());
            for (name, default) in &self.width_params {
                lines.push(format!("    ({name} {default})"));
            }
            lines.push("  )".to_string());
        }

        // Actor-local symbol surface (FSMGen ISF book 13j / public contract):
        // `(types (type NAME (bits k)))`, `(enums (NAME (M V)…))`,
        // `(constants (NAME VALUE))`. Per FSMGen's 2026-05-29 clarity reply
        // (`c0b7eaa7`, locked by `t/1378`): an enum name is NOT a type alias,
        // so a recovered enum co-declares a backing `(type NAME (bits k))`
        // (k = ceil(log2(members))) AND its `(enums …)` family — both are
        // accepted/required. Declared before `(clock …)` to match the book's
        // actor-body shape. Values that are not whitespace-free scalars
        // (operator expressions) are excluded rather than emitted as
        // strict-invalid (residual-honesty).
        // KG-ISF-COMPLETENESS.5.i — a backing `(type NAME (bits k))` is co-declared 1:1 with each enum
        // (both pushed together in `from_intent_ir`). Emit a type ONLY when its enum is actually
        // emitted: an enum dropped by `isf_enum_is_emittable` (the `.2a.iv` value-width gate, or empty
        // members) must NOT leave an orphan `(type …)` line referencing no `(enums …)` family. The
        // type/enum pair share `name`/`type_name`, so a type is kept iff some emitted enum carries it.
        let emitted_enum_names: BTreeSet<&str> = self
            .emitted_enums()
            .iter()
            .map(|e| e.type_name.as_str())
            .collect();
        let emitted_types: Vec<&IsfTypeDef> = self
            .types
            .iter()
            .filter(|t| emitted_enum_names.contains(t.name.as_str()))
            .collect();
        if !emitted_types.is_empty() {
            lines.push("  (types".to_string());
            for t in &emitted_types {
                lines.push(format!("    (type {} (bits {}))", t.name, t.bits));
            }
            lines.push("  )".to_string());
        }
        let safe_enums = self.emitted_enums();
        if !safe_enums.is_empty() {
            lines.push("  (enums".to_string());
            for e in &safe_enums {
                let members: Vec<String> = e
                    .members
                    .iter()
                    .map(|(n, v)| format!("({} {})", n, v))
                    .collect();
                lines.push(format!("    ({} {})", e.type_name, members.join(" ")));
            }
            lines.push("  )".to_string());
        }
        let safe_constants = self.emitted_constants();
        if !safe_constants.is_empty() {
            lines.push("  (constants".to_string());
            for c in &safe_constants {
                lines.push(format!("    ({} {})", c.name, c.value));
            }
            lines.push("  )".to_string());
        }

        lines.push(format!("  (clock {})", self.clock));
        lines.push(format!(
            "  (reset ({} {} {}))",
            self.reset.signal, self.reset.timing, self.reset.polarity
        ));
        lines.push(format!("  (watchdog {})", self.watchdog));

        if !self.signals.is_empty() {
            lines.push("  (interface".to_string());
            for sig in &self.signals {
                let width = match &sig.width_param {
                    Some(param) => param.clone(),
                    None => sig.width.to_string(),
                };
                lines.push(format!(
                    "    ({} {} (width {}))",
                    sig.direction.as_str(),
                    sig.name,
                    width
                ));
            }
            lines.push("  )".to_string());
        }

        if !self.storage.is_empty() {
            lines.push("  (storage".to_string());
            for v in &self.storage {
                // Opaque-var head (no fields): byte-identical to the pre-`.4a.ii` output.
                let head = match v.reset {
                    Some(reset) => {
                        format!("    (var {} (width {}) (reset {})", v.name, v.width, reset)
                    }
                    None => format!("    (var {} (width {})", v.name, v.width),
                };
                if v.fields.is_empty() {
                    lines.push(format!("{head})"));
                    continue;
                }
                // DOC-INTENT-TAXONOMY.4a.ii: declarative field-structured storage. The
                // var stays open so the nested `(fields …)` block is its child.
                lines.push(head);
                lines.push("      (fields".to_string());
                for f in &v.fields {
                    let mut field = format!("        (field {} (bits {} {})", f.name, f.msb, f.lsb);
                    if let Some(access) = &f.access {
                        field.push_str(&format!(" (access {})", access));
                    }
                    if let Some(reset) = f.reset {
                        field.push_str(&format!(" (reset {})", reset));
                    }
                    if !f.enum_members.is_empty() {
                        let members: Vec<String> = f
                            .enum_members
                            .iter()
                            .map(|(m, val)| format!("({} {})", m, val))
                            .collect();
                        field.push_str(&format!(" (enum {})", members.join(" ")));
                    }
                    field.push(')');
                    lines.push(field);
                }
                lines.push("      )".to_string());
                lines.push("    )".to_string());
            }
            lines.push("  )".to_string());
        }

        for d in &self.drives {
            let body_str: Vec<String> = d
                .body
                .iter()
                .map(|(sig, val)| format!("({} {})", sig, val))
                .collect();
            lines.push(format!("  (drive ({} val) {})", d.name, body_str.join(" ")));
        }

        for tx in &self.transactions {
            self.render_transaction(&mut lines, tx);
        }

        for rule in &self.rules {
            let drive_str: Vec<String> = rule
                .drives
                .iter()
                .map(|(sig, val)| format!("    ({} {})", sig, val))
                .collect();
            if rule.condition.is_empty() {
                lines.push(format!("  (rule {}", rule.name));
            } else {
                lines.push(format!("  (rule {} {}", rule.name, rule.condition));
            }
            for d in &drive_str {
                lines.push(d.clone());
            }
            lines.push("  )".to_string());
        }

        lines.push(")".to_string());
        lines.join("\n")
    }

    fn render_transaction(&self, lines: &mut Vec<String>, tx: &IsfTransaction) {
        lines.push(format!("  (transaction {}", tx.name));

        if let Some(trigger) = &tx.on_trigger {
            if tx.on_steps.is_empty() {
                lines.push(format!("    (on {})", trigger));
            } else {
                lines.push(format!("    (on {}", trigger));
                for step in &tx.on_steps {
                    self.render_txn_step(lines, step, "      ");
                }
                lines.push("    )".to_string());
            }
        } else if !tx.on_steps.is_empty() {
            lines.push("    (on start".to_string());
            for step in &tx.on_steps {
                self.render_txn_step(lines, step, "      ");
            }
            lines.push("    )".to_string());
        } else {
            lines.push("    (on start)".to_string());
        }

        for step in &tx.steps {
            self.render_txn_step(lines, step, "    ");
        }

        for contract in &tx.contracts {
            // A transaction-level FSMGen verification-family property
            // `(assert <prop>)`. `prop` is the anchored monitor
            // `(monitor (within s N))` (unguarded bounded-eventually) or the
            // guarded implication `(=> g (within s [min] max))`
            // (FSMGEN-ASSERT-LOWERING), pre-built by the temporal classifier.
            lines.push(format!("    (assert {})", contract.prop));
        }

        for stage in &tx.stages {
            // FSMGen `ready_valid_barrier`, accepted as of pin `9bfb9a20`
            // (R16-CONTRACT-IR.4 / FSMGEN-SUBMODULE-BUMP.1).
            lines.push(format!(
                "    (stage {} (ready {}) (valid {}))",
                stage.name, stage.ready, stage.valid
            ));
        }

        lines.push(format!("    (complete {})", tx.complete));

        if let (Some(min), Some(max)) = (tx.latency_min, tx.latency_max) {
            lines.push(format!("    (latency (min {}) (max {}))", min, max));
        }

        lines.push("  )".to_string());
    }

    fn render_txn_step(&self, lines: &mut Vec<String>, step: &IsfTxnStep, indent: &str) {
        let next = format!("{}  ", indent);
        match step {
            IsfTxnStep::Drive { name, actuals } => {
                let args = actuals.join(" ");
                if args.is_empty() {
                    lines.push(format!("{}(drive {})", indent, name));
                } else {
                    lines.push(format!("{}(drive {} {})", indent, name, args));
                }
            }
            IsfTxnStep::When { condition, body } => {
                lines.push(format!("{}(when {}", indent, condition));
                for s in body {
                    self.render_txn_step(lines, s, &next);
                }
                lines.push(format!("{})", indent));
            }
            IsfTxnStep::Switch { selector, branches } => {
                lines.push(format!("{}(switch {}", indent, selector));
                for (val, body) in branches {
                    lines.push(format!("{}  ({})", indent, val));
                    for s in body {
                        self.render_txn_step(lines, s, &format!("{}    ", indent));
                    }
                }
                lines.push(format!("{})", indent));
            }
            IsfTxnStep::While { condition, body } => {
                lines.push(format!("{}(while {}", indent, condition));
                for s in body {
                    self.render_txn_step(lines, s, &next);
                }
                lines.push(format!("{})", indent));
            }
            IsfTxnStep::Until { condition, body } => {
                lines.push(format!("{}(until {}", indent, condition));
                for s in body {
                    self.render_txn_step(lines, s, &next);
                }
                lines.push(format!("{})", indent));
            }
            IsfTxnStep::Repeat { count, body } => {
                lines.push(format!("{}(repeat {}", indent, count));
                for s in body {
                    self.render_txn_step(lines, s, &next);
                }
                lines.push(format!("{})", indent));
            }
            IsfTxnStep::Await { port, watchdog } => {
                if let Some(wd) = watchdog {
                    lines.push(format!("{}(await {} (watchdog {}))", indent, port, wd));
                } else {
                    lines.push(format!("{}(await {})", indent, port));
                }
            }
            IsfTxnStep::Wait { count } => {
                lines.push(format!("{}(wait {})", indent, count));
            }
            IsfTxnStep::Sample { port, as_name } => {
                lines.push(format!("{}(sample {} as {})", indent, port, as_name));
            }
            IsfTxnStep::Do { child_transaction } => {
                lines.push(format!("{}(do {})", indent, child_transaction));
            }
            IsfTxnStep::Spawn {
                child_transaction,
                instance,
            } => {
                // FSMGen ISF grammar (book 13f-composition.md): the `as`
                // keyword is mandatory — `(spawn child as name)`.
                lines.push(format!(
                    "{}(spawn {} as {})",
                    indent, child_transaction, instance
                ));
            }
            IsfTxnStep::Set { target, expr } => {
                lines.push(format!("{}(set {} {})", indent, target, expr));
            }
            IsfTxnStep::Update { target, expr } => {
                lines.push(format!("{}(update {} {})", indent, target, expr));
            }
            IsfTxnStep::ShiftLeft { reg, bit } => {
                lines.push(format!("{}(shift_left {} {})", indent, reg, bit));
            }
            IsfTxnStep::ShiftRight { reg, bit, width } => {
                if let Some(w) = width {
                    lines.push(format!(
                        "{}(shift_right {} {} (width {}))",
                        indent, reg, bit, w
                    ));
                } else {
                    lines.push(format!("{}(shift_right {} {})", indent, reg, bit));
                }
            }
            IsfTxnStep::Complete { port } => {
                lines.push(format!("{}(complete {})", indent, port));
            }
            IsfTxnStep::AwaitAll { done_port } => {
                lines.push(format!("{}(await_all {})", indent, done_port));
            }
            IsfTxnStep::AwaitAny { done_port } => {
                lines.push(format!("{}(await_any {})", indent, done_port));
            }
            IsfTxnStep::Latency { min, max } => {
                lines.push(format!("{}(latency (min {}) (max {}))", indent, min, max));
            }
        }
    }
}

// ---------------------------------------------------------------------------
// ISF-IR adapter — populates the typed IR from IntentIR
// ---------------------------------------------------------------------------

impl IsfIr {
    pub(crate) fn from_intent_ir(intent_ir: &IntentIr, actor_name: &str) -> Self {
        // --- Clock ---
        let clock = intent_ir
            .system_contract
            .as_ref()
            .map(|contract| contract.clock_signal.clone())
            .or_else(|| {
                intent_ir
                    .infrastructure_signals
                    .iter()
                    .find(|signal| matches!(signal.kind, InfrastructureSignalKind::SystemClock))
                    .map(|signal| signal.signal_name.clone())
            })
            .unwrap_or_else(|| "__specforge_unresolved_clock".to_string());

        // --- Reset (always populated — strict mode requires it) ---
        // Only the typed upstream contract may supply timing or polarity. A stable internal
        // placeholder keeps the diagnostic model deterministic when grounding is incomplete;
        // adapter renderability rejects that model below, so it is never emitted as source intent.
        let reset = if let Some(sc) = &intent_ir.system_contract {
            IsfReset {
                signal: sc.reset_signal.clone(),
                timing: match (sc.reset_kind, sc.reset_polarity) {
                    (SystemResetKind::Synchronous, _) => "sync".to_string(),
                    (SystemResetKind::Asynchronous, _) => "async".to_string(),
                    // EXTRACTION-REPRESENTATION-GAPS.7.d — no kind stated: the one that keeps the stated polarity at
                    // FSMGen's pin, where an `async` reset always comes out active-low and a `sync` one active-high
                    // (EXTRACTION-REPRESENTATION-GAPS.21). An implementation choice, listed as one.
                    (SystemResetKind::NotStated, SystemResetPolarity::ActiveHigh) => {
                        "sync".to_string()
                    }
                    (SystemResetKind::NotStated, _) => "async".to_string(),
                },
                polarity: match sc.reset_polarity {
                    SystemResetPolarity::ActiveHigh => "active_high".to_string(),
                    SystemResetPolarity::ActiveLow => "active_low".to_string(),
                    SystemResetPolarity::Unknown => "unknown".to_string(),
                },
            }
        } else {
            let signal = intent_ir
                .infrastructure_signals
                .iter()
                .find(|signal| matches!(signal.kind, InfrastructureSignalKind::SystemReset))
                .map(|signal| signal.signal_name.clone())
                .unwrap_or_else(|| "__specforge_unresolved_reset".to_string());
            IsfReset {
                signal,
                timing: "unknown".to_string(),
                polarity: "unknown".to_string(),
            }
        };

        // --- Signals (dedup by name, clock/reset excluded) ---
        let infra_names: BTreeSet<&str> = [clock.as_str(), reset.signal.as_str()]
            .into_iter()
            .collect();
        // KG-ISF-COMPLETENESS.2a.i: grounded concrete widths recovered from the
        // actor-port graph, used below as a fallback when the flat signal hint
        // defaults to width 1.
        let port_widths = actor_port_concrete_widths(&intent_ir.actor_ports);
        // ISF-VALUE-WIDTH-EMIT.2: a signal's grounded width can live in a NON-FIRST interface
        // `signal_record` — the first-seen dedup below keeps only the first record's hint, so a
        // concrete width declared in a later record (e.g. trace-bus `ATID` width 7 in its 3rd record,
        // which has NO actor-port so `.2a.i` cannot recover it) is otherwise dropped to the width-1
        // default. Aggregate a single unambiguous concrete (`Numeric` > 1) width across ALL of a
        // signal's records; a conflict keeps width-1 (never a guess), mirroring `actor_port_concrete_widths`.
        let interface_widths: BTreeMap<String, u32> = {
            let mut by_signal: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
            for iface in &intent_ir.interfaces {
                for sig in &iface.signal_records {
                    if let Some(WidthHint::Numeric(n)) = &sig.width_hint
                        && *n > 1
                    {
                        by_signal
                            .entry(sig.signal_name.clone())
                            .or_default()
                            .insert(*n);
                    }
                }
            }
            by_signal
                .into_iter()
                .filter_map(|(signal, widths)| match widths.len() {
                    1 => widths.into_iter().next().map(|w| (signal, w)),
                    _ => None,
                })
                .collect()
        };
        // KG-ISF-COMPLETENESS.2a.ii: the grounded actor-relative direction for the protocol's
        // INITIATOR actor (owner-chosen perspective, `2026-06-17`). Empty when there is no net-producer
        // initiator, so the interface stays byte-identical to the pre-`.2a.ii` default-`output` behavior.
        let initiator = select_initiator_for(intent_ir);
        let initiator_dirs = match &initiator {
            Some(initiator) => initiator_perspective_directions(&intent_ir.actor_ports, initiator),
            None => BTreeMap::new(),
        };
        let mut ports_not_on_actor: BTreeMap<String, &'static str> = BTreeMap::new();
        // EXTRACTION-REPRESENTATION-GAPS.27.e — a signal can sit in several interface records, and only some state a
        // direction: the declared direction is the one they state, when they state exactly one.
        let declared_directions = declared_signal_directions(&intent_ir.interfaces);
        let mut signals: BTreeSet<IsfSignal> = BTreeSet::new();
        let mut seen_signal_names: BTreeSet<String> = BTreeSet::new();
        for iface in &intent_ir.interfaces {
            for sig in &iface.signal_records {
                if infra_names.contains(sig.signal_name.as_str()) {
                    continue;
                }
                if seen_signal_names.contains(&sig.signal_name) {
                    continue;
                }
                seen_signal_names.insert(sig.signal_name.clone());
                // KG-ISF-COMPLETENESS.2a.ii: prefer the grounded INITIATOR-perspective direction from
                // the actor-port graph (the owner-chosen perspective) — a signal the initiator drives is
                // `(output)`, one it reads is `(input)`. The flat `direction_hint` is `None`/`Output` for
                // ~98% of signals (the grounded direction lives on the actor-port graph since
                // `R15-GRAPH-DIRECTION-MIGRATION`), so without this they default to `(output)`. A signal
                // the initiator does not unambiguously touch falls back to the flat hint, then to the
                // honest `(output)` default — never a guess. Direction is FSMGen-strict-neutral
                // (`fsmgen-ignores-signal-direction`), and a signal flipped to `(input)` is automatically
                // excluded from the per-output named-drive block below, so this stays strict-safe.
                // EXTRACTION-REPRESENTATION-GAPS.27.e — a signal the lowered actor neither drives nor reads is not
                // its port: declaring it `(output)` with a drive stated that the actor drives a signal the document
                // gives to another actor (AHB's `HSELx`, driven by the address decoder). It is listed instead, by
                // what the document does relate it to. A signal related to no actor keeps the direction its own
                // declaration states (AXI's `BROADCASTATOMIC` is declared an input), since nothing contradicts it.
                let not_on_actor = initiator
                    .as_deref()
                    .filter(|_| !initiator_dirs.contains_key(&sig.signal_name))
                    .map(|initiator| {
                        port_not_on_actor_reason(
                            &intent_ir.actor_ports,
                            initiator,
                            &sig.signal_name,
                        )
                    })
                    .filter(|reason| {
                        !declared_directions.contains_key(&sig.signal_name)
                            || *reason != PORT_RELATED_TO_NO_ACTOR
                    });
                if let Some(reason) = not_on_actor {
                    ports_not_on_actor.insert(sig.signal_name.clone(), reason);
                    continue;
                }
                let dir = match initiator_dirs.get(&sig.signal_name) {
                    Some(grounded) => grounded.clone(),
                    None => match declared_directions.get(&sig.signal_name) {
                        Some(InterfaceSignalDirection::Input) => IsfDirection::Input,
                        _ => IsfDirection::Output,
                    },
                };
                let width = match &sig.width_hint {
                    Some(w) => render_isf_width_hint(w).parse::<u32>().unwrap_or(1),
                    None => 1,
                };
                // KG-ISF-COMPLETENESS.2a.i: the flat `signal_records[].width_hint` is
                // `None`/symbolic for ~96% of signals (since R15-GRAPH-DIRECTION-MIGRATION
                // the grounded width lives on the actor-port graph), so the match above
                // defaults them to 1. When the graph grounds a single unambiguous concrete
                // width > 1 for this signal, prefer it — a faithful, FSMGen-safe improvement
                // (FSMGen accepts a concrete `(width N)`; it does not validate signal
                // direction — see the `fsmgen-ignores-signal-direction` fact card). A signal
                // with conflicting graph widths keeps the honest width-1 default — never a guess.
                let width = if width == 1 {
                    // ISF-VALUE-WIDTH-EMIT.2: prefer a concrete width grounded in ANY of this
                    // signal's interface records (covers a width in a non-first record), then the
                    // actor-port graph (`.2a.i`), then the honest width-1 default.
                    interface_widths
                        .get(&sig.signal_name)
                        .copied()
                        .or_else(|| port_widths.get(&sig.signal_name).copied())
                        .unwrap_or(1)
                } else {
                    width
                };
                signals.insert(IsfSignal {
                    name: sig.signal_name.clone(),
                    direction: dir,
                    width,
                    width_param: None,
                });
            }
        }

        // --- Constants ---
        // A `Param` symbol is a parameter, not a constant: it is declared in the actor's `(params …)`
        // block when it sizes a signal (KG-ISF-COMPLETENESS.2a.vii.c.i). Declaring it here as well
        // would make its name collide with that declaration.
        let constants: Vec<IsfConstant> = intent_ir
            .symbol_definitions
            .iter()
            .filter(|s| {
                matches!(
                    s.kind,
                    SymbolDefinitionKind::Constant | SymbolDefinitionKind::Define
                )
            })
            .map(|s| IsfConstant {
                name: s.symbol_name.clone(),
                value: match &s.value {
                    Some(expr) => render_isf_control_expression(expr),
                    None => "0".to_string(),
                },
            })
            .collect();

        // --- Types & Enums ---
        let mut types: Vec<IsfTypeDef> = Vec::new();
        let mut enums: Vec<IsfEnum> = Vec::new();
        for s in intent_ir
            .symbol_definitions
            .iter()
            .filter(|s| matches!(s.kind, SymbolDefinitionKind::Enum))
        {
            let members: Vec<(String, String)> = s
                .members
                .iter()
                .map(|m| {
                    (
                        m.member_name.clone(),
                        render_isf_control_expression(&m.value),
                    )
                })
                .collect();
            types.push(IsfTypeDef {
                name: s.symbol_name.clone(),
                bits: enum_type_bits(&members),
            });
            enums.push(IsfEnum {
                type_name: s.symbol_name.clone(),
                members,
            });
        }

        // --- Storage (dedup by name) ---
        // ISF-REGISTER-RESET-EMIT.2/.3: each register lowers to a `(storage (var …))` at its TRUE
        // register width (`register_var_width` — `size_bits ⊔ max(bits_high)+1`, the `.3` fix for
        // the prior max-field-extent mis-sizing); when the register's documented per-field
        // `reset_value`s compose to a clean in-width non-negative integer we also emit `(reset V)`,
        // else the var stays reset-less (FSMGen defaults it to all-0s).
        let mut seen_storage_names: BTreeSet<String> = BTreeSet::new();
        let mut storage: Vec<IsfStorageVar> = Vec::new();
        let mut reset_not_lowerable: usize = 0;
        let mut reset_deferred_width: usize = 0;
        let mut stated_reset_not_lowered: usize = 0;
        let mut fields_not_lowered: usize = 0;
        let mut width_chosen: Vec<String> = Vec::new();
        for r in &intent_ir.register_records {
            let name = sanitize_isf_name(&r.register_name.to_lowercase());
            if !seen_storage_names.insert(name.clone()) {
                continue;
            }
            let width = register_var_width(r);
            // EXTRACTION-REPRESENTATION-GAPS.41.b — a width the document does not state is the adapter's choice.
            if r.size_bits.is_none() {
                width_chosen.push(r.register_name.clone());
            }
            // EXTRACTION-GAP-FIX.8: a reset the register states as a whole takes part; its misses are
            // counted apart so the field-reset residual keeps meaning what it says.
            let reset = match classify_stated_register_reset(r, width) {
                Some(RegisterResetOutcome::Emit(value)) => Some(value),
                Some(RegisterResetOutcome::DeferredWidth | RegisterResetOutcome::NotLowerable) => {
                    stated_reset_not_lowered += 1;
                    None
                }
                Some(RegisterResetOutcome::DefaultZero | RegisterResetOutcome::NoReset) => None,
                None => match classify_register_reset(&r.fields, width) {
                    RegisterResetOutcome::Emit(value) => Some(value),
                    RegisterResetOutcome::DeferredWidth => {
                        reset_deferred_width += 1;
                        None
                    }
                    RegisterResetOutcome::NotLowerable => {
                        reset_not_lowerable += 1;
                        None
                    }
                    RegisterResetOutcome::DefaultZero | RegisterResetOutcome::NoReset => None,
                },
            };
            // DOC-INTENT-TAXONOMY.4a.ii: lower the register's bit-field map into FSMGen's
            // declarative `(fields …)` storage block. The field `(reset)`s are gated on the
            // parent reset `reset` so they always have an explicit parent slice to match.
            let (fields, field_residual) = register_storage_fields(r, width, reset);
            fields_not_lowered += field_residual;
            storage.push(IsfStorageVar {
                name,
                width,
                reset,
                fields,
            });
        }
        let mut storage_reset_residuals: Vec<ResidualDecisionPacket> = Vec::new();
        if reset_not_lowerable + reset_deferred_width > 0 {
            storage_reset_residuals.push(storage_reset_residual_packet(
                reset_not_lowerable,
                reset_deferred_width,
            ));
        }
        if stated_reset_not_lowered > 0 {
            storage_reset_residuals.push(stated_register_reset_residual_packet(
                stated_reset_not_lowered,
            ));
        }
        let mut storage_field_residuals: Vec<ResidualDecisionPacket> = Vec::new();
        if fields_not_lowered > 0 {
            storage_field_residuals.push(storage_field_residual_packet(fields_not_lowered));
        }
        if !width_chosen.is_empty() {
            storage_field_residuals.push(register_width_chosen_residual_packet(&width_chosen));
        }

        // KG-ISF-COMPLETENESS.2a.vii.b: a width the document writes as ONE parameter
        // (`HRDATA | DATA_WIDTH`) describes a generic block, so it lowers to `(width DATA_WIDTH)`
        // with an actor `(params (DATA_WIDTH D))` declaration instead of a bare `(width 1)`. `D` is
        // the document's stated default when it is positive (`.2a.vii.c.i`), else a placeholder `1`
        // surfaced as a residual, never as a fact. Runs once every other actor name is declared, so
        // the parameter name can be checked against them, and before the rules, whose value literals
        // are aligned against the widths this sets.
        let reserved_names = declared_isf_names(
            &signals,
            &clock,
            &reset.signal,
            &constants,
            &enums,
            &storage,
        );
        let width_hints = intent_ir
            .interfaces
            .iter()
            .flat_map(|iface| iface.signal_records.iter())
            .map(|sig| (sig.signal_name.as_str(), sig.width_hint.as_ref()))
            .chain(
                intent_ir
                    .actor_ports
                    .iter()
                    .map(|port| (port.signal_name.as_str(), port.width_hint.as_ref())),
            );
        let (signals, width_params, signal_width_residuals) = lower_parameter_widths(
            signals,
            &symbolic_signal_widths(width_hints),
            &stated_parameter_defaults(&intent_ir.symbol_definitions),
            &chosen_parameter_defaults(&intent_ir.symbol_definitions),
            &reserved_names,
        );

        // --- Named drives (one per output signal) ---
        let drives: Vec<IsfNamedDrive> = signals
            .iter()
            .filter(|s| s.direction == IsfDirection::Output)
            .map(|s| IsfNamedDrive {
                name: s.name.clone(),
                body: vec![(s.name.clone(), "val".to_string())],
            })
            .collect();

        // --- Transactions (from IntentIR transactions) ---
        let transactions: Vec<IsfTransaction> = intent_ir
            .transactions
            .iter()
            .filter(|tx| !tx.steps.is_empty())
            .map(|tx| {
                let (on_steps, body_steps) = partition_txn_steps(&tx.steps);
                IsfTransaction {
                    name: sanitize_isf_name(&tx.transaction_name),
                    on_trigger: tx.activation_port.clone(),
                    on_steps: convert_txn_steps(&on_steps),
                    steps: convert_txn_steps(&body_steps),
                    complete: "done".to_string(),
                    contracts: Vec::new(),
                    stages: Vec::new(),
                    latency_min: None,
                    latency_max: None,
                }
            })
            .collect();

        // --- Fallback: control_blocks → transactions ---
        let mut fallback_txns: Vec<IsfTransaction> = Vec::new();
        if transactions.is_empty() {
            for cb in &intent_ir.control_blocks {
                if cb.branches.is_empty() {
                    continue;
                }
                let block_name = sanitize_isf_name(&cb.block_name);
                let tx_name = format!("{}_tx", block_name);
                match &cb.selector {
                    None => {
                        let actions = collect_branch_actions(&cb.branches);
                        let steps: Vec<IsfTxnStep> =
                            actions.iter().map(convert_action_to_txn_step).collect();
                        fallback_txns.push(IsfTransaction {
                            name: tx_name,
                            on_trigger: None,
                            on_steps: vec![],
                            steps,
                            complete: "done".to_string(),
                            contracts: Vec::new(),
                            stages: Vec::new(),
                            latency_min: None,
                            latency_max: None,
                        });
                    }
                    Some(selector_expr) => {
                        let sel_text = render_isf_control_expression(selector_expr);
                        if cb.branches.len() == 1 {
                            let branch = &cb.branches[0];
                            let guard = branch_predicate_guard(branch, &sel_text);
                            let steps: Vec<IsfTxnStep> = branch
                                .actions
                                .iter()
                                .map(convert_action_to_txn_step)
                                .collect();
                            fallback_txns.push(IsfTransaction {
                                name: tx_name,
                                on_trigger: None,
                                on_steps: vec![],
                                steps: vec![IsfTxnStep::When {
                                    condition: guard,
                                    body: steps,
                                }],
                                complete: "done".to_string(),
                                contracts: Vec::new(),
                                stages: Vec::new(),
                                latency_min: None,
                                latency_max: None,
                            });
                        } else {
                            let branches: Vec<(String, Vec<IsfTxnStep>)> = cb
                                .branches
                                .iter()
                                .map(|b| {
                                    let val = match &b.predicate {
                                        Some(pred) => render_isf_control_expression(pred),
                                        None => "default".to_string(),
                                    };
                                    let body: Vec<IsfTxnStep> =
                                        b.actions.iter().map(convert_action_to_txn_step).collect();
                                    (val, body)
                                })
                                .collect();
                            fallback_txns.push(IsfTransaction {
                                name: tx_name,
                                on_trigger: None,
                                on_steps: vec![],
                                steps: vec![IsfTxnStep::Switch {
                                    selector: sel_text,
                                    branches,
                                }],
                                complete: "done".to_string(),
                                contracts: Vec::new(),
                                stages: Vec::new(),
                                latency_min: None,
                                latency_max: None,
                            });
                        }
                    }
                }
            }
        }

        let mut all_transactions = if transactions.is_empty() {
            fallback_txns
        } else {
            transactions
        };

        // --- Rules ---
        let signal_names: BTreeSet<String> = signals.iter().map(|s| s.name.clone()).collect();
        // EXTRACTION-REPRESENTATION-GAPS.8 — the polarity each signal's own records state, the only thing
        // that turns `asserted` into `1` or `0` (ADR 0037 §5: never the signal's spelling).
        let polarities = with_reset_polarity(
            signal_polarity_map(&intent_ir.signal_polarities),
            intent_ir.system_contract.as_ref(),
        );
        // EXTRACTION-REPRESENTATION-GAPS.7.b — the signals a temporal rule may name: the interface and the
        // actor's reset. A contract takes the reset out of the interface, and without it here a guard such
        // as *when ARESETn is asserted* would be skipped and the rule would drive unconditionally.
        let rule_signals: BTreeSet<String> = signal_names
            .iter()
            .cloned()
            .chain(
                intent_ir
                    .system_contract
                    .as_ref()
                    .map(|contract| contract.reset_signal.clone()),
            )
            .collect();
        // FSMGen `ready_valid_barrier` requires the stage `ready` operand
        // to be an actor INPUT ("stage … input '<r>' is not an actor
        // input"); only emit `(stage …)` when that holds, else residual
        // (R16-CONTRACT-IR.4 — never fabricate a strict-invalid stage).
        let input_signal_names: BTreeSet<String> = signals
            .iter()
            .filter(|s| s.direction == IsfDirection::Input)
            .map(|s| s.name.clone())
            .collect();

        // --- ISF-TEMPORAL-LOWERING.2.2/.2.3: lower temporal_rules ---
        // Each `temporal_rule` is classified into exactly one disposition
        // (`.1` mapping #1/#3/#4). Windowed `bounded_eventually` → a
        // synthetic transaction carrying the FSMGen verification-family
        // monitor property `(assert (monitor (within <signal> <N>)))` (the
        // `(contract … (eventually …))` clause was removed at pin 43b29f5c —
        // FSMGEN-ASSERT-MIGRATE). Non-windowed
        // value/guard→drive → an actor `(rule …)`. Anything with no
        // representable supported ISF construct (HandshakeComplete,
        // `(within 0)`, no concrete value, undeclared signal, …) is
        // preserved as an explicit residual decision — never fabricated
        // (`fsmgen-contract-authority`).
        let mut temporal_isf_rules: Vec<IsfRule> = Vec::new();
        let mut temporal_residuals: Vec<ResidualDecisionPacket> = Vec::new();
        // R16-CONTRACT-IR.3: lowering consumes the typed ContractIR
        // (`actor_contracts`) via `classify_actor_contract`, which
        // reproduces the exact `classify_temporal_rule` decision
        // (parity gate: emitted `.isf` byte-identical on the real corpus).
        // Back-compat: an `IntentIR` persisted before ContractIR has
        // `temporal_rules` but no `actor_contracts` — project on the fly
        // so adapting pre-existing artifacts stays parity-identical (the
        // projection is the same lossless `contract_from_temporal_rule`).
        let temporal_contracts: Vec<crate::ir::contract::ActorContract> =
            if intent_ir.actor_contracts.is_empty() {
                intent_ir
                    .temporal_rules
                    .iter()
                    .map(crate::ir::contract::contract_from_temporal_rule)
                    .collect()
            } else {
                intent_ir.actor_contracts.clone()
            };
        for contract in &temporal_contracts {
            let rule_id = || {
                contract.source_rule_id.clone().unwrap_or_else(|| {
                    contract
                        .contract_id
                        .strip_prefix("contract_")
                        .unwrap_or(&contract.contract_id)
                        .to_string()
                })
            };
            let unread = unread_stated_condition(
                contract,
                &intent_ir.signal_constraints,
                &intent_ir.conditional_rules,
            );
            let unread_release = unread_hold_release(contract, &intent_ir.signal_constraints);
            let disposition = match classify_actor_contract(contract, &rule_signals, &polarities) {
                // The reset is an input the actor tests, never one it drives or hands a barrier.
                TemporalRuleDisposition::Rule { signal, .. }
                | TemporalRuleDisposition::Stage { valid: signal, .. }
                    if !signal_names.contains(&signal) =>
                {
                    TemporalRuleDisposition::Residual {
                        rule_id: rule_id(),
                        reason: format!(
                            "temporal rule drives '{signal}', the actor's reset, which the actor \
                             never drives"
                        ),
                    }
                }
                // EXTRACTION-REPRESENTATION-GAPS.10.a — a rule on the actor's own input is another actor's
                // obligation: from this actor's side an assumption, never a drive.
                TemporalRuleDisposition::Rule { signal, .. }
                    if input_signal_names.contains(&signal) =>
                {
                    TemporalRuleDisposition::Residual {
                        rule_id: rule_id(),
                        reason: format!(
                            "temporal rule drives '{signal}', an input of the actor: {}",
                            LevelRefusal::DrivesAnInput.describe()
                        ),
                    }
                }
                // EXTRACTION-REPRESENTATION-GAPS.25.a — a stated condition none of which became a guard.
                TemporalRuleDisposition::Rule { .. }
                | TemporalRuleDisposition::Contract { .. }
                | TemporalRuleDisposition::Stage { .. }
                    if unread.is_some() =>
                {
                    TemporalRuleDisposition::Residual {
                        rule_id: rule_id(),
                        reason: format!(
                            "the rule's stated condition `{}` was not read into a guard; lowered without \
                             it, the rule would apply always",
                            unread.unwrap_or_default()
                        ),
                    }
                }
                // EXTRACTION-REPRESENTATION-GAPS.25.c — a hold whose release became no part of its guard.
                TemporalRuleDisposition::Rule { .. }
                | TemporalRuleDisposition::Contract { .. }
                | TemporalRuleDisposition::Stage { .. }
                    if unread_release.is_some() =>
                {
                    TemporalRuleDisposition::Residual {
                        rule_id: rule_id(),
                        reason: format!(
                            "the hold's release `{}` was not read into its guard; lowered without it, the hold \
                             would never release",
                            unread_release.unwrap_or_default()
                        ),
                    }
                }
                disposition => disposition,
            };
            match disposition {
                TemporalRuleDisposition::Contract { name, prop } => {
                    all_transactions.push(IsfTransaction {
                        name: format!("txn_temporal_{}", name),
                        on_trigger: None,
                        on_steps: vec![],
                        steps: vec![],
                        complete: "done".to_string(),
                        latency_min: None,
                        latency_max: None,
                        contracts: vec![IsfContract { prop }],
                        stages: vec![],
                    });
                }
                TemporalRuleDisposition::Stage { name, ready, valid } => {
                    // R16-CONTRACT-IR.4: ready/valid barrier → synthetic
                    // transaction carrying a FSMGen `ready_valid_barrier`
                    // `(stage …)` (accepted at pin `9bfb9a20`). FSMGen
                    // requires the `ready` operand to be an actor INPUT;
                    // otherwise emit no stage and preserve a residual —
                    // never fabricate a strict-invalid `(stage …)`.
                    if input_signal_names.contains(&ready) {
                        all_transactions.push(IsfTransaction {
                            name: format!("txn_temporal_{}", name),
                            on_trigger: None,
                            on_steps: vec![],
                            steps: vec![],
                            complete: "done".to_string(),
                            latency_min: None,
                            latency_max: None,
                            contracts: vec![],
                            stages: vec![IsfStage {
                                name: format!("stage_{}", name),
                                ready,
                                valid,
                            }],
                        });
                    } else {
                        let rid = contract.source_rule_id.clone().unwrap_or_else(|| {
                            contract
                                .contract_id
                                .strip_prefix("contract_")
                                .unwrap_or(&contract.contract_id)
                                .to_string()
                        });
                        temporal_residuals.push(temporal_residual_packet(
                            &rid,
                            &format!(
                                "ready/valid barrier ready signal '{}' is not an \
                                 actor input; FSMGen `ready_valid_barrier` requires \
                                 it — preserved as residual (not fabricated)",
                                ready
                            ),
                            &contract.provenance.source_text,
                        ));
                    }
                }
                TemporalRuleDisposition::Rule {
                    name,
                    condition,
                    signal,
                    value,
                } => {
                    temporal_isf_rules.push(IsfRule {
                        name,
                        condition,
                        drives: vec![(signal, value)],
                    });
                }
                TemporalRuleDisposition::Residual { rule_id, reason } => {
                    temporal_residuals.push(temporal_residual_packet(
                        &rule_id,
                        &reason,
                        &contract.provenance.source_text,
                    ));
                }
            }
        }

        let (mut rules, refused) = lower_obligations(
            &intent_ir.conditional_rules,
            &intent_ir.signal_constraints,
            &intent_ir.temporal_invariants,
            &signal_names,
            &input_signal_names,
            intent_ir
                .system_contract
                .as_ref()
                .map(|contract| contract.reset_signal.as_str()),
            &polarities,
            &intent_ir.operation_kinds,
        );
        if !refused.is_empty() {
            temporal_residuals.push(refused_obligations_residual_packet(&refused));
        }

        // Temporal value/guard→drive rules (`.2.3` #3) join the rule set
        // before dedup so a temporal rule that conflicts with an existing
        // rule on the same signal+guard is dropped (FSMGen strict rejects
        // conflicting drives) rather than producing invalid `.isf`.
        rules.extend(temporal_isf_rules);

        // --- Rule drive-value validity gate (KG-ISF-COMPLETENESS.2a.vi) ---
        // FSMGen requires a rule assignment action's RHS to be a renderable value expression
        // (`(port expr)`); a constraint whose extracted value is free PROSE (the AMBA AXI+ACE loopback
        // `(RLOOP the value that was presented on the ARLOOP signal)`) would emit a multi-word value
        // FSMGen rejects, breaking the whole `.isf`. Drop a rule whose any drive value is not a renderable
        // scalar (non-empty, whitespace-free) and record an honest residual — the prose value is
        // unrecoverable (a temporal loopback, not the current `(port ARLOOP)`), so never fabricate one.
        // Runs before the width/dedup passes so a prose rule never reaches value processing.
        {
            let (kept, value_residuals) = drop_unrenderable_rule_values(rules);
            rules = kept;
            temporal_residuals.extend(value_residuals);
        }

        // --- Value width-alignment (ISF-VALUE-WIDTH-EMIT.2) ---
        // A rule drive whose value literal's notation width differs from the target signal's emitted
        // width is FSMGen-strict-invalid: the OperandContract blocks implicit truncation and requires
        // an "explicit width-aligned source expression" (measured DTI `ATST`, trace-bus `ATID`). Re-render
        // such a literal as a width-aligned `W'd<v>` cast WHEN the value fits the signal width, else DROP
        // the rule with an honest residual — never truncate or fabricate a value. Runs before dedup so a
        // re-rendered value participates in conflict detection on its final form.
        {
            let signal_widths: BTreeMap<String, u32> =
                signals.iter().map(|s| (s.name.clone(), s.width)).collect();
            let (aligned, width_residuals) = align_rule_drive_widths(rules, &signal_widths);
            rules = aligned;
            temporal_residuals.extend(width_residuals);
        }

        // --- Parity checks (EXTRACTION-REPRESENTATION-GAPS.46.b) ---
        // A check signal keeps the parity its document states over the signals it covers. Its value is an
        // expression over those signals, not a level, so it joins after the value gates above, which admit only
        // a renderable scalar level; dedup below still sees it.
        {
            let (parity_rules, refused) = lower_parity_checks(
                &intent_ir.check_signal_records,
                &signals,
                &stated_numeric_widths(intent_ir),
                intent_ir
                    .system_contract
                    .as_ref()
                    .map(|contract| contract.reset_signal.as_str()),
            );
            rules.extend(parity_rules);
            if !refused.is_empty() {
                temporal_residuals.push(parity_checks_not_lowered_packet(&refused));
            }
        }

        // --- Dedup: remove rules that conflict on the same signal+guard ---
        // When two rules share the same guard but drive the same signal to
        // different values, FSMGen rejects the ISF. Keep the first and record
        // every dropped conflict as an explicit residual (never a silent loss
        // — ISF-RULE-CONFLICT-RESIDUAL).
        {
            let (deduped, conflict_residuals) = dedup_conflicting_rules(rules);
            rules = deduped;
            temporal_residuals.extend(conflict_residuals);
        }

        // --- Cross-guard overlap dedup (KG-ISF-COMPLETENESS.2a.v, "Lever C") ---
        // The same-guard dedup above keys on (signal, guard), so it misses a conflict between an
        // UNCONDITIONAL rule (empty guard, always active) and a GUARDED rule on the same signal with a
        // different value — yet FSMGen rejects it (`isf_conflicting_rule_writes`): an unconditional rule's
        // firing set ⊇ every guard, and FSMGen's `_condition_terms_prove_disjoint` can never prove an
        // absent condition disjoint (the AMBA LPI `PREQ`/`PACCEPT` case). Drop each such conflicting rule
        // with an honest residual (the unconditional value wins), never a fabricated precedence.
        {
            let (deduped, overlap_residuals) = drop_unconditional_overlap_conflicts(rules);
            rules = deduped;
            temporal_residuals.extend(overlap_residuals);
        }

        // --- Rule/transaction write conflicts (FSMGEN-REFRESH-INTEGRATE-6.1) ---
        // IntentIR carries rules and transactions, but it carries no precedence relation between
        // them. The former emitter hid that missing authority by asserting EVERY rule over EVERY
        // transaction. Current FSMGen correctly fails closed when such a priority targets a named
        // drive with multiple callers, and requires an actor-level priority when one local caller
        // and a rule can both write the same target. Never fabricate that winner. Remove only the
        // rule side of the proven single-caller conflict and preserve it as an explicit residual;
        // the richer source-grounded transaction remains executable.
        {
            let (kept, transaction_residuals) =
                drop_ungrounded_rule_transaction_conflicts(rules, &all_transactions, &drives);
            rules = kept;
            temporal_residuals.extend(transaction_residuals);
        }

        let reset_kind_not_stated = intent_ir
            .system_contract
            .as_ref()
            .is_some_and(|contract| matches!(contract.reset_kind, SystemResetKind::NotStated))
            .then(|| reset_kind_not_stated_residual_packet(&reset));
        // EXTRACTION-REPRESENTATION-GAPS.24.b — an output the document gives one instance per actor of a role is
        // declared once: this `.isf` drives one instance, and says so.
        let driven_families: Vec<&SignalFamilyRecord> = intent_ir
            .signal_families
            .iter()
            .filter(|family| {
                signals.iter().any(|signal| {
                    signal.name == family.signal_name
                        && matches!(signal.direction, IsfDirection::Output)
                })
            })
            .collect();
        let family_driven_once = (!driven_families.is_empty())
            .then(|| family_driven_once_residual_packet(actor_name, &driven_families));

        IsfIr {
            actor_name: actor_name.to_string(),
            clock,
            reset,
            watchdog: 65536,
            signals,
            width_params,
            constants,
            types,
            enums,
            storage,
            drives,
            transactions: all_transactions,
            rules,
            temporal_residuals,
            storage_reset_residuals,
            storage_field_residuals,
            signal_width_residuals,
            interface_residuals: (!ports_not_on_actor.is_empty())
                .then(|| {
                    ports_not_on_actor_residual_packet(
                        initiator.as_deref().unwrap_or_default(),
                        &ports_not_on_actor,
                    )
                })
                .into_iter()
                .chain(reset_kind_not_stated)
                .chain(family_driven_once)
                .collect(),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Outcome of attempting to lower a register's documented field resets to FSMGen's optional
/// storage `(reset V)` (ISF-REGISTER-RESET-EMIT.2). Composition LSB-tiles the per-field
/// `reset_value`s — `V = OR(parse(reset_i) << bits_low_i)` (the `recover_register_bits` discipline)
/// — and only a clean, in-width, non-negative integer is ever emitted; everything else is honest.
enum RegisterResetOutcome {
    /// Strictly composable, composed `V > 0`, and `V` fits the emitted var width → emit `(reset V)`.
    Emit(u64),
    /// Strictly composable and `V == 0` — faithfully the FSMGen all-0s default → omit (no residual).
    DefaultZero,
    /// Strictly composable and `V > 0` but `V` does not fit the current (max-field-extent) var width
    /// → omit; deferred to ISF-REGISTER-RESET-EMIT.3 (var-width reconciliation). Counts as residual.
    DeferredWidth,
    /// At least one field carries a `reset_value` but the register is not strictly composable
    /// (symbolic/unparseable value, unlocated field, partial coverage, over-wide field value, or
    /// overlapping fields) → omit; values remain in IntentIR `register_records`. Counts as residual.
    NotLowerable,
    /// No field carries a `reset_value` → nothing to lower, no residual.
    NoReset,
}

/// Parse a register-field reset literal as a non-negative integer, accepting only the universal
/// numeric notations (decimal, `0x…` hex, `0b…` binary, `…h` hex-suffix). Returns `None` for
/// symbolic values (`-`, `X`, `IMPLEMENTATION DEFINED`, Verilog `8'h1F`, …) — ADR-0006: numeric
/// parsing only, no chip-name list, never a guess.
fn parse_reset_literal(raw: &str) -> Option<u64> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        return u64::from_str_radix(hex, 16).ok();
    }
    if let Some(bin) = s.strip_prefix("0b").or_else(|| s.strip_prefix("0B")) {
        return u64::from_str_radix(bin, 2).ok();
    }
    if let Some(hex) = s.strip_suffix('h').or_else(|| s.strip_suffix('H'))
        && !hex.is_empty()
        && hex.bytes().all(|b| b.is_ascii_hexdigit())
    {
        return u64::from_str_radix(hex, 16).ok();
    }
    if s.bytes().all(|b| b.is_ascii_digit()) {
        return s.parse::<u64>().ok();
    }
    None
}

/// The ISF storage-var width for a register: its TRUE register width (ISF-REGISTER-RESET-EMIT.3).
/// Prefer the document's declared `size_bits` (a doc may declare a 32-bit register whose top bits
/// are reserved/unparsed), but never below one-past-the-highest-located-field-bit
/// (`max(bits_high)+1`) so no field is ever truncated — hence `size_bits ⊔ field_top`. Falls back
/// to 32 only when neither a declared width nor any located field is available. This replaces the
/// prior max-single-field-extent width, which mis-sized multi-field registers (e.g. a 32-bit
/// register of two 16-bit fields was emitted at `(width 16)`, truncating any reset above bit 15).
fn register_var_width(r: &RegisterRecord) -> u32 {
    let field_top = r
        .fields
        .iter()
        .filter_map(register_field_extent)
        .map(|(hi, _)| hi.saturating_add(1))
        .max();
    match (r.size_bits, field_top) {
        (Some(size), Some(top)) => size.max(top),
        (Some(size), None) => size,
        (None, Some(top)) => top,
        (None, None) => 32,
    }
}

/// The register-bit extent `(high, low)` of a field, from `bits_high`/`bits_low`, or
/// `bits_low`+`bit_width`, or a single `bits_low` bit. `None` when the field is unlocated.
fn register_field_extent(f: &RegisterFieldRecord) -> Option<(u32, u32)> {
    let lo = f.bits_low?;
    if let Some(hi) = f.bits_high {
        if hi < lo {
            return None;
        }
        Some((hi, lo))
    } else if let Some(w) = f.bit_width {
        if w == 0 {
            return None;
        }
        Some((lo + w - 1, lo))
    } else {
        Some((lo, lo))
    }
}

/// What a register's documented per-field resets compose to (ISF-REGISTER-RESET-EMIT.2).
enum FieldResetComposition {
    /// No field carries a `reset_value`.
    Unstated,
    /// Every field's value LSB-tiled into the register value.
    Composed(u64),
    /// Some field states a reset, but the set does not compose (symbolic or unparseable value,
    /// unlocated field, partial coverage, over-wide field value, or overlapping fields).
    NotComposable,
}

/// Compose a register's field resets. *Strictly composable* iff EVERY field carries a `reset_value`
/// that parses to a non-negative integer fitting its own field width, every field is located, and no
/// two fields overlap; the per-field values are then LSB-tiled into the register value. Bounded to
/// ≤64-bit registers (`u64`); anything wider stays an honest residual.
fn compose_field_resets(fields: &[RegisterFieldRecord]) -> FieldResetComposition {
    if !fields.iter().any(|f| f.reset_value.is_some()) {
        return FieldResetComposition::Unstated;
    }
    let mut composed: u64 = 0;
    let mut used_mask: u64 = 0;
    for f in fields {
        let Some(raw) = f.reset_value.as_deref() else {
            return FieldResetComposition::NotComposable; // partial coverage
        };
        let Some((hi, lo)) = register_field_extent(f) else {
            return FieldResetComposition::NotComposable; // unlocated
        };
        let field_width = hi - lo + 1;
        if hi >= 64 || field_width >= 64 {
            return FieldResetComposition::NotComposable; // beyond u64-safe tiling
        }
        let Some(value) = parse_reset_literal(raw) else {
            return FieldResetComposition::NotComposable; // symbolic
        };
        if value >= (1u64 << field_width) {
            return FieldResetComposition::NotComposable; // value over-wide for its field
        }
        let mask = ((1u64 << field_width) - 1) << lo;
        if used_mask & mask != 0 {
            return FieldResetComposition::NotComposable; // overlapping fields
        }
        used_mask |= mask;
        composed |= value << lo;
    }
    FieldResetComposition::Composed(composed)
}

/// The lowering outcome of a register reset value `V` at storage width `var_width`: all-0s is FSMGen's
/// default and is omitted, a value that fits is emitted, a wider one is deferred.
fn register_reset_value_outcome(value: u64, var_width: u32) -> RegisterResetOutcome {
    if value == 0 {
        RegisterResetOutcome::DefaultZero
    } else if var_width >= 64 || value < (1u64 << var_width) {
        RegisterResetOutcome::Emit(value)
    } else {
        RegisterResetOutcome::DeferredWidth
    }
}

/// Classify a register's reset from its fields alone for ISF storage lowering
/// (ISF-REGISTER-RESET-EMIT.2): see [`compose_field_resets`].
fn classify_register_reset(fields: &[RegisterFieldRecord], var_width: u32) -> RegisterResetOutcome {
    match compose_field_resets(fields) {
        FieldResetComposition::Unstated => RegisterResetOutcome::NoReset,
        FieldResetComposition::NotComposable => RegisterResetOutcome::NotLowerable,
        FieldResetComposition::Composed(value) => register_reset_value_outcome(value, var_width),
    }
}

/// EXTRACTION-GAP-FIX.8 — the reset outcome of a register that states a reset of its own (a register
/// summary's `Reset` column), or `None` when it states none and its fields alone decide. The stated
/// value decides when no field states a reset, and must equal what the fields compose when they do; a
/// field set that does not compose, a disagreement, or a stated value that is not a plain integer
/// (`0x00000080 b`, whose footnote qualifies it, or `UNKNOWN`) is not lowered.
fn classify_stated_register_reset(
    register: &RegisterRecord,
    var_width: u32,
) -> Option<RegisterResetOutcome> {
    let stated = register.reset_value.as_deref()?;
    let outcome = match (
        parse_reset_literal(stated),
        compose_field_resets(&register.fields),
    ) {
        (Some(value), FieldResetComposition::Unstated) => {
            register_reset_value_outcome(value, var_width)
        }
        (Some(value), FieldResetComposition::Composed(composed)) if value == composed => {
            register_reset_value_outcome(value, var_width)
        }
        _ => RegisterResetOutcome::NotLowerable,
    };
    Some(outcome)
}

/// A single proportionate honesty packet recording that some register resets were not lowered to
/// `(reset V)` (ISF-REGISTER-RESET-EMIT.2 bar #2). One summary per adapter — not one per register.
fn storage_reset_residual_packet(
    not_lowerable: usize,
    deferred_width: usize,
) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: "isf_storage_reset_not_lowered".to_string(),
        question: format!(
            "{} register(s) carry a documented field reset that was not lowered to an ISF `(reset V)`",
            not_lowerable + deferred_width
        ),
        why_unresolved: format!(
            "{not_lowerable} not integer-lowerable (symbolic/implementation-defined value, partial \
             field coverage, over-wide field value, or overlapping fields — the values remain in \
             IntentIR register_records); {deferred_width} composable but over the storage-var width \
             (deferred to ISF-REGISTER-RESET-EMIT.3 var-width reconciliation). FSMGen defaults an \
             omitted `(reset V)` to all-0s; no value is fabricated."
        ),
        automation_confidence: AutomationConfidence::Medium,
        candidate_interpretations: Vec::new(),
    }
}

/// EXTRACTION-GAP-FIX.8 — one packet recording that some registers state a reset of their own that was
/// not lowered to `(reset V)`; kept apart from [`storage_reset_residual_packet`], which counts field resets.
fn stated_register_reset_residual_packet(not_lowered: usize) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: "isf_stated_register_reset_not_lowered".to_string(),
        question: format!(
            "{not_lowered} register(s) state a reset of their own that was not lowered to an ISF `(reset V)`"
        ),
        why_unresolved: "A register's stated reset is lowered only when it is a plain integer that fits \
             the storage width and equals any reset its fields compose. A qualified value (a footnote \
             mark, `UNKNOWN`, `IMPLEMENTATION DEFINED`), a disagreement with the fields, or a value over \
             the width stays in IntentIR register_records. FSMGen defaults an omitted `(reset V)` to \
             all-0s; no value is fabricated."
            .to_string(),
        automation_confidence: AutomationConfidence::Medium,
        candidate_interpretations: Vec::new(),
    }
}

/// Normalize a register-field access notation to FSMGen's storage-field `(access …)` token set
/// (`ro|rw|wo|w1c|w0c|rc|rs|warl|wpri|reserved`, DOC-INTENT-TAXONOMY.4a.ii). Accepts an exact
/// (case-insensitive) token or an UNAMBIGUOUS universal synonym; returns `None` for anything
/// else so the field emits without an `(access …)` clause — honest, never a guess (ADR-0006:
/// universal RTL access vocabulary, no chip-name list). An unsupported token would fail closed
/// in FSMGen, so omission is the safe faithful choice (access is optional metadata).
fn normalize_field_access(access: Option<&str>) -> Option<String> {
    let raw = access?.trim().to_lowercase();
    let token = match raw.as_str() {
        "ro" | "rw" | "wo" | "w1c" | "w0c" | "rc" | "rs" | "warl" | "wpri" | "reserved" => {
            raw.as_str()
        }
        "r" | "read-only" | "readonly" | "read only" => "ro",
        "w" | "write-only" | "writeonly" | "write only" => "wo",
        "r/w" | "read-write" | "readwrite" | "read/write" | "read write" => "rw",
        "rw1c" => "w1c",
        "rw0c" => "w0c",
        _ => return None,
    };
    Some(token.to_string())
}

/// Lower a register's bit-fields into FSMGen's declarative storage `(fields …)` block
/// (DOC-INTENT-TAXONOMY.4a.ii). Returns the admissible [`IsfStorageField`]s plus the count of
/// captured `RegisterFieldRecord`s NOT lowered (an honest residual). Admission is structural
/// (ADR-0006, no name list) and fail-closed, matching FSMGen's own field validation:
///   - only LOCATED fields (`register_field_extent` = `Some`) can carry `(bits HI LO)`; an
///     unlocated field is an honest gap (the `(fields …)` block allows gaps) → counted residual.
///   - a sanitized field name shared by ≥ 2 located fields is AMBIGUOUS (reserved-gap repeats
///     like `res0`, or a flattened mis-extraction) — every member of the colliding group is
///     dropped, since FSMGen fails closed on duplicate field names and renaming would fabricate.
///   - the survivors must be mutually NON-OVERLAPPING; any residual overlap fails the WHOLE
///     register's field block closed (overlap is an ambiguous extraction — picking one would
///     guess). In-width is guaranteed by `register_var_width` (≥ max `bits_high` + 1).
///   - `(access)` is normalized (`normalize_field_access`), omitted when unmapped.
///   - a field `(reset)` is emitted ONLY when the parent var carries a composed reset
///     (`parent_reset = Some(v)`), as that value's own bit slice — so it always matches the
///     parent reset slice (FSMGen's field-reset-must-match-parent rule) by construction.
///   - `(enum)` keeps `(MEMBER VALUE)` members whose numeric value fits the field width
///     (`meaning` → sanitized member name, deduped); non-numeric / over-wide members are dropped.
fn register_storage_fields(
    r: &RegisterRecord,
    var_width: u32,
    parent_reset: Option<u64>,
) -> (Vec<IsfStorageField>, usize) {
    let total = r.fields.len();
    // Located fields only (carry a concrete bit range), each with its sanitized name. The bound
    // mirrors `classify_register_reset`: u64-safe tiling (`hi`/width < 64) and inside the parent
    // var width; anything else is an honest residual (the field map stays in IntentIR).
    let located: Vec<(&RegisterFieldRecord, (u32, u32), String)> = r
        .fields
        .iter()
        .filter_map(|f| {
            let (hi, lo) = register_field_extent(f)?;
            if hi >= 64 || (hi - lo + 1) >= 64 || hi >= var_width {
                return None;
            }
            Some((f, (hi, lo), sanitize_isf_name(&f.field_name)))
        })
        .collect();
    // Drop every field whose sanitized name collides (count ≥ 2) — structurally ambiguous.
    let mut name_counts: BTreeMap<String, usize> = BTreeMap::new();
    for (_, _, name) in &located {
        *name_counts.entry(name.clone()).or_default() += 1;
    }
    let kept: Vec<(&RegisterFieldRecord, (u32, u32), String)> = located
        .into_iter()
        .filter(|(_, _, name)| name_counts[name.as_str()] == 1)
        .collect();
    if kept.is_empty() {
        return (Vec::new(), total);
    }
    // Reject the whole register's field block on any residual bit overlap (fail-closed). Every
    // kept field has `hi < 64` and `width < 64`, so the mask shifts are always u64-safe.
    let mut used_mask: u64 = 0;
    for (_, (hi, lo), _) in &kept {
        let mask = ((1u64 << (hi - lo + 1)) - 1) << lo;
        if used_mask & mask != 0 {
            return (Vec::new(), total);
        }
        used_mask |= mask;
    }
    let mut fields: Vec<IsfStorageField> = Vec::with_capacity(kept.len());
    for (f, (hi, lo), name) in &kept {
        let (hi, lo) = (*hi, *lo);
        let field_width = hi - lo + 1;
        // Field reset = the parent reset's own slice for [hi:lo] (matches the parent slice by
        // construction); only when the field documents a reset AND the parent reset is composed.
        let reset = match parent_reset {
            Some(v) if f.reset_value.is_some() => Some(if field_width >= 64 {
                v >> lo
            } else {
                (v >> lo) & ((1u64 << field_width) - 1)
            }),
            _ => None,
        };
        // Enum members that fit the field width; meaning → unique sanitized member name.
        let mut enum_members: Vec<(String, u64)> = Vec::new();
        let mut seen_members: BTreeSet<String> = BTreeSet::new();
        for e in &f.enumerated_values {
            let Some(value) = parse_reset_literal(&e.value) else {
                continue;
            };
            if field_width < 64 && value >= (1u64 << field_width) {
                continue;
            }
            let member = sanitize_isf_name(&e.meaning);
            if !seen_members.insert(member.clone()) {
                continue;
            }
            enum_members.push((member, value));
        }
        fields.push(IsfStorageField {
            name: name.clone(),
            msb: hi,
            lsb: lo,
            access: normalize_field_access(f.access_type.as_deref()),
            reset,
            enum_members,
        });
    }
    let not_lowered = total - fields.len();
    (fields, not_lowered)
}

/// A single proportionate honesty packet recording that some register bit-fields were not
/// lowered to the storage `(fields …)` block (DOC-INTENT-TAXONOMY.4a.ii). One summary per
/// adapter — not one per register.
fn storage_field_residual_packet(not_lowered: usize) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: "isf_register_fields_not_lowered".to_string(),
        question: format!(
            "{not_lowered} register bit-field(s) were not lowered to the ISF storage `(fields …)` block"
        ),
        why_unresolved: "A field is lowered only when it carries a concrete bit range, its \
             sanitized name is unique within the register, and the register's located fields do \
             not overlap. Unlocated fields (no bit range) are honest gaps, an ambiguous (shared) \
             field name is dropped rather than guessed, and an overlapping extraction fails the \
             register's field block closed — the full field map always remains in IntentIR \
             register_records; no bit position is fabricated."
            .to_string(),
        automation_confidence: AutomationConfidence::Medium,
        candidate_interpretations: Vec::new(),
    }
}

/// EXTRACTION-REPRESENTATION-GAPS.41.b — the registers whose storage width the document does not state. A `(var …)`
/// needs a width, so the adapter chooses the narrowest one that holds every located field (32 with none), and says so:
/// ADIv6's `ABORT`, whose fields end at bit 4, is emitted 5 bits wide.
fn register_width_chosen_residual_packet(registers: &[String]) -> ResidualDecisionPacket {
    let shown = registers
        .iter()
        .take(12)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");
    let more = registers.len().saturating_sub(12);
    let tail = if more > 0 {
        format!(" and {more} more")
    } else {
        String::new()
    };
    ResidualDecisionPacket {
        packet_id: "isf_register_width_not_stated".to_string(),
        question: format!(
            "{} register storage width(s) are not stated by the document",
            registers.len()
        ),
        why_unresolved: format!(
            "The document states no width for {shown}{tail}. Each is emitted as wide as its highest located field \
             (32 bits with no located field), the narrowest width that holds what the document places in it; a \
             register whose top bits are reserved is wider than that. The width is the adapter's choice, not the \
             document's."
        ),
        automation_confidence: AutomationConfidence::Medium,
        candidate_interpretations: Vec::new(),
    }
}

fn partition_txn_steps(steps: &[TransactionStep]) -> (Vec<TransactionStep>, Vec<TransactionStep>) {
    let mut on_steps: Vec<TransactionStep> = Vec::new();
    let mut body_steps: Vec<TransactionStep> = Vec::new();
    let mut seen_non_sample = false;

    for step in steps {
        match step {
            TransactionStep::Sample { .. } if !seen_non_sample => {
                on_steps.push(step.clone());
            }
            _ => {
                seen_non_sample = true;
                body_steps.push(step.clone());
            }
        }
    }

    (on_steps, body_steps)
}

fn convert_txn_steps(steps: &[TransactionStep]) -> Vec<IsfTxnStep> {
    steps.iter().map(convert_txn_step).collect()
}

fn convert_txn_step(step: &TransactionStep) -> IsfTxnStep {
    match step {
        TransactionStep::Drive {
            drive_name,
            actuals,
        } => IsfTxnStep::Drive {
            name: drive_name.clone(),
            actuals: actuals.clone(),
        },
        TransactionStep::When { condition, body } => IsfTxnStep::When {
            condition: condition.clone(),
            body: convert_txn_steps(body),
        },
        TransactionStep::Switch { selector, branches } => IsfTxnStep::Switch {
            selector: selector.clone(),
            branches: branches
                .iter()
                .map(|b| (b.value.clone(), convert_txn_steps(&b.body)))
                .collect(),
        },
        TransactionStep::While { condition, body } => IsfTxnStep::While {
            condition: condition.clone(),
            body: convert_txn_steps(body),
        },
        TransactionStep::Until { condition, body } => IsfTxnStep::Until {
            condition: condition.clone(),
            body: convert_txn_steps(body),
        },
        TransactionStep::Repeat { count, body } => IsfTxnStep::Repeat {
            count: count.clone(),
            body: convert_txn_steps(body),
        },
        TransactionStep::Await { port, watchdog } => IsfTxnStep::Await {
            port: port.clone(),
            watchdog: watchdog.map(|w| w as u64),
        },
        TransactionStep::Wait { count } => IsfTxnStep::Wait {
            count: count.clone(),
        },
        TransactionStep::Sample { port, as_name } => IsfTxnStep::Sample {
            port: port.clone(),
            as_name: as_name.clone(),
        },
        TransactionStep::Do {
            child_transaction,
            bindings: _,
        } => IsfTxnStep::Do {
            child_transaction: child_transaction.clone(),
        },
        TransactionStep::Spawn {
            child_transaction,
            instance,
            bindings: _,
        } => IsfTxnStep::Spawn {
            child_transaction: child_transaction.clone(),
            instance: instance.clone(),
        },
        TransactionStep::Set { target, expr } => IsfTxnStep::Set {
            target: target.clone(),
            expr: expr.clone(),
        },
        TransactionStep::Update { target, expr } => IsfTxnStep::Update {
            target: target.clone(),
            expr: expr.clone(),
        },
        TransactionStep::ShiftLeft { reg, bit } => IsfTxnStep::ShiftLeft {
            reg: reg.clone(),
            bit: bit.clone(),
        },
        TransactionStep::ShiftRight { reg, bit, width } => IsfTxnStep::ShiftRight {
            reg: reg.clone(),
            bit: bit.clone(),
            width: width.map(|w| w as u8),
        },
        TransactionStep::Complete { port } => IsfTxnStep::Complete { port: port.clone() },
        TransactionStep::AwaitAll { done_port } => IsfTxnStep::AwaitAll {
            done_port: done_port.clone(),
        },
        TransactionStep::AwaitAny { done_port } => IsfTxnStep::AwaitAny {
            done_port: done_port.clone(),
        },
        TransactionStep::Latency { min, max } => IsfTxnStep::Latency {
            min: *min as u64,
            max: *max as u64,
        },
    }
}

fn convert_action_to_txn_step(action: &ControlActionRecord) -> IsfTxnStep {
    match action {
        ControlActionRecord::Assign { target, value, .. } => IsfTxnStep::Drive {
            name: target.signal_name.clone(),
            actuals: vec![render_isf_control_expression(value)],
        },
        ControlActionRecord::Transition { target_state } => IsfTxnStep::Set {
            target: "state".to_string(),
            expr: target_state.clone(),
        },
        ControlActionRecord::DelayedPulse { target, value, .. } => IsfTxnStep::Drive {
            name: target.signal_name.clone(),
            actuals: vec![render_isf_control_expression(value)],
        },
        ControlActionRecord::CompoundUpdate {
            target, operation, ..
        } => {
            let expr = match operation {
                ControlCompoundUpdateOperation::Increment => "+1".to_string(),
                ControlCompoundUpdateOperation::Decrement => "-1".to_string(),
            };
            IsfTxnStep::Update {
                target: target.signal_name.clone(),
                expr,
            }
        }
    }
}

/// True when a rendered control-expression value is safe to place directly
/// as a `(constants (NAME VALUE))` / enum-member scalar: a single
/// whitespace-free token — a literal (`0`, `0x3`), a reference
/// (`mode.BUSY`, `bus[3]`), or a width-cast (`(8'd5)`). Operator
/// expressions (`(| a b)`, `(! x)`) render with internal whitespace and are
/// NOT valid in scalar position (FSMGen `--strict` rejects them), so they
/// are excluded — emit nothing rather than strict-invalid `.isf`
/// (residual-honesty).
fn is_safe_isf_scalar_value(value: &str) -> bool {
    !value.is_empty() && !value.chars().any(char::is_whitespace)
}

/// True when an enum-member value is emittable to FSMGen exactly as the emitter renders it
/// (KG-ISF-COMPLETENESS.2a.iv). FSMGen's package-symbol parser (the pinned `subs/fsmgen`) treats a
/// BARE token of only binary digits (`0`/`1`) with length >= 4 as a binary-style literal and REJECTS
/// it un-qualified — it must be width/radix-qualified (`4'b1000`). Verified by a value sweep against
/// the real FSMGen: `1000`/`1010`/`1111`/`10000` fail, while `0`/`1`/`10`/`111` (<=3 binary digits)
/// and `999`/`1020`/`69152` (any value containing a 2-9 digit) all pass at ANY magnitude. Such a
/// value is a BINARY CODE the extractor mis-read as a bare decimal (HBM2's `TABLE.REPAIR_LANE`
/// `1000`/`1111`); the emitter renders values verbatim and cannot recover the true radix without
/// fabricating, so an enum carrying one is dropped to a residual rather than emitted invalid. This
/// predicate is FALSE ONLY for that un-emittable binary-token shape — every legitimate decimal value
/// passes, so a currently-clean enum stays byte-identical. Universal token grammar, no name list (ADR 0006).
fn isf_enum_value_is_emittable_literal(value: &str) -> bool {
    !(value.len() >= 4 && value.bytes().all(|b| b == b'0' || b == b'1'))
}

/// `EXTRACTION-REPRESENTATION-GAPS.59` — the width of an enum's co-declared type: wide enough for its largest member
/// code (at least 1 bit). Sizing it from the member count left AXI's `AWATOP` 4 bits wide for code 49 and `ARCACHE` 3
/// bits for 15. FSMGen gives an enum's values and its type alias distinct roles and checks neither against the other,
/// so the width must be right where it is written. A member whose value is no integer literal leaves the count-based
/// width, the only bound left to state.
fn enum_type_bits(members: &[(String, String)]) -> u32 {
    let by_count = members
        .len()
        .max(1)
        .next_power_of_two()
        .trailing_zeros()
        .max(1);
    let codes: Option<Vec<u128>> = members
        .iter()
        .map(|(_, value)| {
            let value = value.trim();
            if let Some(hex) = value
                .strip_prefix("0x")
                .or_else(|| value.strip_prefix("0X"))
            {
                u128::from_str_radix(hex, 16).ok()
            } else if let Some(binary) = value
                .strip_prefix("0b")
                .or_else(|| value.strip_prefix("0B"))
            {
                u128::from_str_radix(binary, 2).ok()
            } else {
                value.parse::<u128>().ok()
            }
        })
        .collect();
    match codes {
        Some(codes) if !codes.is_empty() => {
            let largest = codes.into_iter().max().unwrap_or(0);
            (u128::BITS - largest.leading_zeros()).max(1)
        }
        _ => by_count,
    }
}

/// An enum is EMITTABLE iff it is non-empty AND every member value is a whitespace-free scalar that
/// FSMGen accepts as the emitter renders it (see [`isf_enum_value_is_emittable_literal`]). The gate
/// (KG-ISF-COMPLETENESS.2a.iv) holds out a mega-conflated enum that carries a bare binary-looking
/// token — the HBM2 `TABLE` (binary codes the extractor mis-read as decimals) — rather than emitting
/// a literal FSMGen's package-symbol contract rejects. A well-formed enum (every value already
/// FSMGen-emittable, as every currently-clean doc's enums are) is byte-identical to the pre-`.2a.iv`
/// output.
fn isf_enum_is_emittable(e: &IsfEnum) -> bool {
    !e.members.is_empty()
        && e.members
            .iter()
            .all(|(_, v)| is_safe_isf_scalar_value(v) && isf_enum_value_is_emittable_literal(v))
}

fn render_isf_control_expression(expr: &ControlExpressionRecord) -> String {
    match expr {
        ControlExpressionRecord::Literal { literal } => literal.clone(),
        ControlExpressionRecord::Reference { reference } => {
            let mut s = reference.base_name.clone();
            for suffix in &reference.suffixes {
                match suffix {
                    ControlReferenceSuffix::Member { member_name } => {
                        s = format!("{}.{}", s, member_name);
                    }
                    ControlReferenceSuffix::BitIndex { index } => {
                        s = format!("{}[{}]", s, index);
                    }
                    ControlReferenceSuffix::Slice { msb, lsb } => {
                        s = format!("{}[{}:{}]", s, msb, lsb);
                    }
                    ControlReferenceSuffix::WidthCast { width } => {
                        s = format!("({}'d{})", width, s);
                    }
                }
            }
            s
        }
        ControlExpressionRecord::Unary { operator, operand } => {
            let op_str = match operator {
                ControlUnaryOperator::Not => "!",
            };
            format!("({} {})", op_str, render_isf_control_expression(operand))
        }
        ControlExpressionRecord::Binary {
            operator,
            left,
            right,
        } => {
            let op_str = render_isf_binary_operator(*operator);
            format!(
                "({} {} {})",
                op_str,
                render_isf_control_expression(left),
                render_isf_control_expression(right)
            )
        }
    }
}

fn render_isf_binary_operator(op: ControlBinaryOperator) -> &'static str {
    match op {
        ControlBinaryOperator::Eq => "==",
        ControlBinaryOperator::NotEq => "!=",
        ControlBinaryOperator::Lt => "<",
        ControlBinaryOperator::Le => "<=",
        ControlBinaryOperator::Gt => ">",
        ControlBinaryOperator::Ge => ">=",
        ControlBinaryOperator::BitAnd => "&",
        ControlBinaryOperator::BitOr => "|",
        ControlBinaryOperator::BitXor => "^",
        ControlBinaryOperator::Add => "+",
        ControlBinaryOperator::Sub => "-",
        ControlBinaryOperator::Mul => "*",
        ControlBinaryOperator::Div => "/",
        ControlBinaryOperator::Mod => "%",
    }
}

fn render_isf_width_hint(width: &WidthHint) -> String {
    match width {
        WidthHint::Numeric(n) => n.to_string(),
        WidthHint::Parametric(p) => p.clone(),
    }
}

/// KG-ISF-COMPLETENESS.2a.i: a per-signal concrete width recovered from the actor-port graph.
///
/// The flat `signal_records[].width_hint` is `None`/symbolic for ~96% of declared signals — since
/// `R15-GRAPH-DIRECTION-MIGRATION` the grounded width lives on `actor_ports`. This maps each signal
/// to its single unambiguous concrete (`Numeric`, `> 1`) graph width so the signal lowering can
/// prefer it over the width-1 default. A signal whose graph widths disagree is omitted, so the
/// caller keeps the honest width-1 default rather than guessing. Measured `2026-06-17`: 69 signals
/// corpus-wide gain a width this way, 0 conflicts (`isf-lowering-fidelity-gauge`).
fn actor_port_concrete_widths(actor_ports: &[ActorPortRecord]) -> BTreeMap<String, u32> {
    let mut by_signal: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    for port in actor_ports {
        if let Some(WidthHint::Numeric(n)) = &port.width_hint
            && *n > 1
        {
            by_signal
                .entry(port.signal_name.clone())
                .or_default()
                .insert(*n);
        }
    }
    by_signal
        .into_iter()
        .filter_map(|(signal, widths)| match widths.len() {
            1 => widths.into_iter().next().map(|w| (signal, w)),
            _ => None,
        })
        .collect()
}

/// KG-ISF-COMPLETENESS.2a.vii.b: the placeholder default of a width parameter whose value the IR
/// does not carry. It is the width the adapter elaborated before parameters were emitted, so the
/// design behaves exactly as before while the width stays a named, generic parameter.
const PLACEHOLDER_WIDTH_PARAM_DEFAULT: u32 = 1;

/// KG-ISF-COMPLETENESS.2a.vii.b: the signals whose width the document writes ONLY symbolically,
/// mapped to that one symbolic width.
///
/// `hints` is every width hint the IR holds, as `(signal, hint)` — each interface record and each
/// actor port — because a signal is emitted once but described in several places. A signal with
/// any numeric hint is left out, so it keeps the concrete-width path (`.2a.i`,
/// `ISF-VALUE-WIDTH-EMIT.2`) unchanged; so is one whose symbolic hints disagree, which is never
/// guessed between.
fn symbolic_signal_widths<'a>(
    hints: impl IntoIterator<Item = (&'a str, Option<&'a WidthHint>)>,
) -> BTreeMap<String, String> {
    #[derive(Default)]
    struct Hints<'a> {
        symbolic: BTreeSet<&'a str>,
        numeric: bool,
    }
    let mut by_signal: BTreeMap<&str, Hints> = BTreeMap::new();
    for (signal, hint) in hints {
        let hints = by_signal.entry(signal).or_default();
        match hint {
            Some(WidthHint::Numeric(_)) => hints.numeric = true,
            Some(WidthHint::Parametric(text)) => {
                hints.symbolic.insert(text.trim());
            }
            None => {}
        }
    }
    by_signal
        .into_iter()
        .filter(|(_, hints)| !hints.numeric && hints.symbolic.len() == 1)
        .filter_map(|(signal, hints)| {
            let width = hints.symbolic.into_iter().next()?;
            Some((signal.to_string(), width.to_string()))
        })
        .collect()
}

/// KG-ISF-COMPLETENESS.2a.vii.c.i: the default each `Param` symbol states, when it is a bare
/// non-negative integer. A parameter stated with two different values is left out, never chosen
/// between; so is one whose value is an expression.
fn stated_parameter_defaults(symbols: &[SymbolDefinitionRecord]) -> BTreeMap<String, u64> {
    let mut values: BTreeMap<&str, BTreeSet<u64>> = BTreeMap::new();
    for symbol in symbols
        .iter()
        .filter(|s| matches!(s.kind, SymbolDefinitionKind::Param))
    {
        let Some(ControlExpressionRecord::Literal { literal }) = &symbol.value else {
            continue;
        };
        if let Ok(value) = literal.trim().parse::<u64>() {
            values
                .entry(symbol.symbol_name.as_str())
                .or_default()
                .insert(value);
        }
    }
    values
        .into_iter()
        .filter_map(|(name, stated)| match stated.len() {
            1 => stated.into_iter().next().map(|v| (name.to_string(), v)),
            _ => None,
        })
        .collect()
}

/// Which of a parameter's stated value sets a chosen width-parameter default was taken from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ChosenDefaultBasis {
    /// The smallest positive legal value inside a recommended range (`.c.iii`).
    LegalAndRecommended,
    /// The smallest positive legal value (`.c.ii`).
    Legal,
    /// The smallest positive recommended value, when no legal set is stated (`.c.iii`).
    Recommended,
}

impl ChosenDefaultBasis {
    fn describe(self) -> &'static str {
        match self {
            Self::LegalAndRecommended => {
                "the smallest positive legal value the document recommends"
            }
            Self::Legal => "the smallest positive legal value",
            Self::Recommended => "the smallest positive recommended value",
        }
    }
}

/// A width-parameter default chosen from the values the document states, and what it was chosen from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct ChosenDefault {
    value: u64,
    basis: ChosenDefaultBasis,
}

/// KG-ISF-COMPLETENESS.2a.vii.c.ii/.c.iii: the default each `Param` symbol's stated values give it — the
/// smallest positive legal value inside a recommended range, else the smallest positive legal value, else
/// the smallest positive recommended value (`0..32` gives 1, `0, 16..32` gives 16, legal `8, 16, 32` with
/// a recommended `16..64` gives 16). Zero is left out on purpose: it means the signal is absent, and
/// FSMGen refuses it as a width. A parameter whose symbols disagree is left out, never chosen between.
fn chosen_parameter_defaults(
    symbols: &[SymbolDefinitionRecord],
) -> BTreeMap<String, ChosenDefault> {
    let mut chosen: BTreeMap<&str, BTreeSet<ChosenDefault>> = BTreeMap::new();
    for symbol in symbols
        .iter()
        .filter(|s| matches!(s.kind, SymbolDefinitionKind::Param))
    {
        let legal = &symbol.legal_values;
        let recommended = &symbol.recommended_values;
        let within_recommendation = legal
            .iter()
            .flat_map(|l| {
                recommended.iter().map(move |r| ParameterValueRange {
                    low: l.low.max(r.low),
                    high: l.high.min(r.high),
                })
            })
            .filter(|range| range.low <= range.high)
            .collect::<Vec<_>>();
        let choice = [
            (
                within_recommendation.as_slice(),
                ChosenDefaultBasis::LegalAndRecommended,
            ),
            (legal.as_slice(), ChosenDefaultBasis::Legal),
            (recommended.as_slice(), ChosenDefaultBasis::Recommended),
        ]
        .into_iter()
        .find_map(|(ranges, basis)| {
            smallest_positive_value(ranges).map(|value| ChosenDefault { value, basis })
        });
        if let Some(choice) = choice {
            chosen
                .entry(symbol.symbol_name.as_str())
                .or_default()
                .insert(choice);
        }
    }
    chosen
        .into_iter()
        .filter_map(|(name, choices)| match choices.len() {
            1 => choices.into_iter().next().map(|c| (name.to_string(), c)),
            _ => None,
        })
        .collect()
}

/// The smallest positive value any of `ranges` contains.
fn smallest_positive_value(ranges: &[ParameterValueRange]) -> Option<u64> {
    ranges
        .iter()
        .filter(|range| range.high > 0)
        .map(|range| range.low.max(1))
        .min()
}

/// True for a name FSMGen accepts as an actor parameter: `[A-Za-z_][A-Za-z0-9_]*`.
fn is_isf_identifier(text: &str) -> bool {
    let mut chars = text.chars();
    matches!(chars.next(), Some(c) if c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Every name the actor already declares, which a width parameter must not reuse.
fn declared_isf_names(
    signals: &BTreeSet<IsfSignal>,
    clock: &str,
    reset: &str,
    constants: &[IsfConstant],
    enums: &[IsfEnum],
    storage: &[IsfStorageVar],
) -> BTreeSet<String> {
    let enum_names = enums.iter().flat_map(|e| {
        std::iter::once(e.type_name.clone())
            .chain(e.members.iter().map(|(member, _)| member.clone()))
    });
    signals
        .iter()
        .map(|s| s.name.clone())
        .chain([clock.to_string(), reset.to_string()])
        .chain(constants.iter().map(|c| c.name.clone()))
        .chain(enum_names)
        .chain(storage.iter().map(|v| v.name.clone()))
        .collect()
}

/// A symbolic signal width that stays `(width 1)`, and why no parameter can state it.
struct UnloweredSignalWidth {
    signal: String,
    width: String,
    reason: &'static str,
}

/// KG-ISF-COMPLETENESS.2a.vii.b: lower each signal whose width is exactly one parameter `P` to
/// `(width P)`, and declare every such `P` once with a default: the positive default the document
/// states (`.c.i`), else the smallest positive value it allows or recommends (`.c.ii`/`.c.iii`, a
/// choice named in its own residual), else `(P 1)` — a placeholder.
///
/// FSMGen's published contract (book 13a, *Interface*) resolves `(width P)` against an actor scalar
/// parameter and fails closed on an undeclared name, a zero default, or any expression, so:
/// a width that is not a bare identifier (`DATA_WIDTH/8`, `ceil(ADDR_WIDTH/8)`) cannot be stated
/// and keeps `(width 1)`; so does a name the actor already declares. Returns the rewritten
/// signals, the parameters to declare, and at most three summary residuals — one naming every
/// placeholder default, one every chosen default, one every width that stayed `(width 1)` — so
/// neither a placeholder, a choice nor a loss is ever presented as a fact.
fn lower_parameter_widths(
    signals: BTreeSet<IsfSignal>,
    symbolic_widths: &BTreeMap<String, String>,
    stated_defaults: &BTreeMap<String, u64>,
    chosen_defaults: &BTreeMap<String, ChosenDefault>,
    reserved_names: &BTreeSet<String>,
) -> (
    BTreeSet<IsfSignal>,
    BTreeMap<String, u32>,
    Vec<ResidualDecisionPacket>,
) {
    let mut params: BTreeMap<String, u32> = BTreeMap::new();
    let mut placeholder_signals: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut chosen_signals: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut unlowered: Vec<UnloweredSignalWidth> = Vec::new();
    let signals = signals
        .into_iter()
        .map(|mut sig| {
            let Some(width) = symbolic_widths.get(&sig.name) else {
                return sig;
            };
            let reason = if !is_isf_identifier(width) {
                Some("An expression, which no FSMGen actor parameter can state")
            } else if reserved_names.contains(width) {
                Some("A name this actor already declares")
            } else {
                None
            };
            match reason {
                Some(reason) => unlowered.push(UnloweredSignalWidth {
                    signal: sig.name.clone(),
                    width: width.clone(),
                    reason,
                }),
                None => {
                    let positive_width = |value: Option<&u64>| {
                        value
                            .copied()
                            .filter(|&value| value > 0)
                            .and_then(|value| u32::try_from(value).ok())
                    };
                    let stated = positive_width(stated_defaults.get(width));
                    let chosen =
                        positive_width(chosen_defaults.get(width).map(|chosen| &chosen.value));
                    let default = match (stated, chosen) {
                        (Some(stated), _) => stated,
                        (None, Some(chosen)) => {
                            chosen_signals
                                .entry(width.clone())
                                .or_default()
                                .push(sig.name.clone());
                            chosen
                        }
                        (None, None) => {
                            placeholder_signals
                                .entry(width.clone())
                                .or_default()
                                .push(sig.name.clone());
                            PLACEHOLDER_WIDTH_PARAM_DEFAULT
                        }
                    };
                    params.insert(width.clone(), default);
                    sig.width = default;
                    sig.width_param = Some(width.clone());
                }
            }
            sig
        })
        .collect();
    let mut residuals = Vec::new();
    if !placeholder_signals.is_empty() {
        residuals.push(width_parameter_default_residual_packet(
            &placeholder_signals,
            stated_defaults,
        ));
    }
    if !chosen_signals.is_empty() {
        residuals.push(width_parameter_chosen_default_residual_packet(
            &chosen_signals,
            chosen_defaults,
            stated_defaults,
        ));
    }
    if !unlowered.is_empty() {
        residuals.push(unlowered_signal_width_residual_packet(&unlowered));
    }
    (signals, params, residuals)
}

/// The residual naming every width parameter whose default is `.2a.vii.b`'s placeholder.
fn width_parameter_default_residual_packet(
    sized_signals: &BTreeMap<String, Vec<String>>,
    stated_defaults: &BTreeMap<String, u64>,
) -> ResidualDecisionPacket {
    let listed = sized_signals
        .iter()
        .map(|(param, signals)| {
            format!(
                "`{param}` sizes {} ({})",
                signals.join(", "),
                stated_default_reason(stated_defaults.get(param))
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    ResidualDecisionPacket {
        packet_id: "isf_signal_width_parameter_defaults".to_string(),
        question: "Which default should each signal-width parameter declare?".to_string(),
        why_unresolved: format!(
            "{} parameter(s) size {} signal(s) whose width the document writes as a parameter; each \
             signal lowers to `(width P)` with an actor `(params (P {PLACEHOLDER_WIDTH_PARAM_DEFAULT}))` \
             declaration: {listed}. \
             None of these states a positive default, legal value or recommended value, so each \
             default is a placeholder that elaborates the width emitted before parameters were; it is \
             not a value the document states. FSMGen elaborates the generated HDL at the declared \
             default, so another width means regenerating with a different one.",
            sized_signals.len(),
            sized_signals.values().map(Vec::len).sum::<usize>()
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "document_stated_default".to_string(),
                description: "Declare the default the document states for the parameter, else its \
                              smallest positive legal value."
                    .to_string(),
                downstream_impact: "Needs the document's parameter declarations read into the IR \
                                    (KG-ISF-COMPLETENESS.2a.vii.c); the parameter stays generic."
                    .to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "specialise_at_instantiation".to_string(),
                description: "Keep the placeholder; the integrator sets every parameter when the \
                              block is instantiated."
                    .to_string(),
                downstream_impact:
                    "The block elaborates at the placeholder, which is not a recommended width."
                        .to_string(),
            },
        ],
    }
}

/// Why a parameter's stated default is not the width it is declared with.
fn stated_default_reason(stated: Option<&u64>) -> &'static str {
    match stated {
        Some(0) => "the document's default is 0: the signal is absent by default",
        Some(_) => "the document's default does not fit a width",
        None => "the document states no default",
    }
}

/// KG-ISF-COMPLETENESS.2a.vii.c.ii/.c.iii: the residual naming every width parameter whose default is
/// the smallest positive value the document allows or recommends, and which. The value is the
/// document's; choosing it as the default is the adapter's, so it is said here and never presented as a
/// stated default.
fn width_parameter_chosen_default_residual_packet(
    sized_signals: &BTreeMap<String, Vec<String>>,
    chosen_defaults: &BTreeMap<String, ChosenDefault>,
    stated_defaults: &BTreeMap<String, u64>,
) -> ResidualDecisionPacket {
    let listed = sized_signals
        .iter()
        .map(|(param, signals)| {
            let chosen = chosen_defaults[param];
            format!(
                "`{param}` = {} sizes {} ({}; {})",
                chosen.value,
                signals.join(", "),
                chosen.basis.describe(),
                stated_default_reason(stated_defaults.get(param))
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    ResidualDecisionPacket {
        packet_id: "isf_signal_width_parameter_chosen_defaults".to_string(),
        question: "Is the smallest positive stated value the right default for each signal-width \
                   parameter the document leaves without one?"
            .to_string(),
        why_unresolved: format!(
            "{} parameter(s) size {} signal(s) and state no positive default, so each is declared \
             with the smallest positive value the document allows or recommends for it: {listed}. \
             The value is one the document states; choosing it as the default is this adapter's \
             rule, not the document's. FSMGen elaborates the generated HDL at the declared default, \
             so another width means regenerating with a different one.",
            sized_signals.len(),
            sized_signals.values().map(Vec::len).sum::<usize>()
        ),
        automation_confidence: AutomationConfidence::Medium,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "smallest_positive_legal_value".to_string(),
                description: "Keep the smallest positive stated value: the narrowest block the \
                              document allows (or recommends) with the signal present."
                    .to_string(),
                downstream_impact: "The block elaborates at a width the document permits; a wider \
                                    configuration needs a different default until FSMGen elaborates \
                                    parameter-driven port shapes."
                    .to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "specialise_at_instantiation".to_string(),
                description: "Treat the default as the integrator's: the block is only ever \
                              elaborated with every parameter set."
                    .to_string(),
                downstream_impact: "No default would be needed, but FSMGen requires one for the \
                                    block to elaborate on its own."
                    .to_string(),
            },
        ],
    }
}

/// EXTRACTION-REPRESENTATION-GAPS.27.e — each signal's declared direction across every interface record that states
/// one; a signal whose records state two directions has none.
fn declared_signal_directions(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> BTreeMap<String, InterfaceSignalDirection> {
    let mut stated: BTreeMap<String, Option<InterfaceSignalDirection>> = BTreeMap::new();
    for record in interfaces.iter().flat_map(|iface| &iface.signal_records) {
        let Some(direction) = record.direction_hint else {
            continue;
        };
        stated
            .entry(record.signal_name.clone())
            .and_modify(|known| {
                if *known != Some(direction) {
                    *known = None;
                }
            })
            .or_insert(Some(direction));
    }
    stated
        .into_iter()
        .filter_map(|(signal, direction)| direction.map(|direction| (signal, direction)))
        .collect()
}

/// EXTRACTION-REPRESENTATION-GAPS.27.e — the reason given for a signal no actor relation names.
const PORT_RELATED_TO_NO_ACTOR: &str =
    "the document relates it to no actor, and its declaration states no direction";

/// EXTRACTION-REPRESENTATION-GAPS.27.e — why the document grounds no direction for `signal` on `initiator`.
fn port_not_on_actor_reason(
    actor_ports: &[ActorPortRecord],
    initiator: &str,
    signal: &str,
) -> &'static str {
    let related: Vec<&ActorPortRecord> = actor_ports
        .iter()
        .filter(|port| port.signal_name == signal)
        .collect();
    if related.iter().any(|port| port.actor_name == initiator) {
        "the document relates it to the actor in more than one direction"
    } else if related.is_empty() {
        PORT_RELATED_TO_NO_ACTOR
    } else {
        "the document relates it only to other actors"
    }
}

/// EXTRACTION-REPRESENTATION-GAPS.27.e — the residual naming every interface signal the lowered actor leaves out.
/// EXTRACTION-REPRESENTATION-GAPS.7.d — the reset kind the emitted `.isf` states when the document states none.
fn reset_kind_not_stated_residual_packet(reset: &IsfReset) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: "isf_reset_kind_not_stated".to_string(),
        question: format!(
            "Is `{}` a synchronous or an asynchronous reset?",
            reset.signal
        ),
        why_unresolved: format!(
            "The document names `{signal}` as the reset and states its polarity, but not whether it is synchronous \
             or asynchronous, and FSMGen's reset clause has no unstated kind. The emitted `{timing}` is an \
             implementation choice, not the document's: it is the kind that keeps the stated `{polarity}` at \
             SpecForge's FSMGen pin, where an `async` reset always comes out active-low and a `sync` one \
             active-high (EXTRACTION-REPRESENTATION-GAPS.21).",
            signal = reset.signal,
            timing = reset.timing,
            polarity = reset.polarity,
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "read_the_stated_kind".to_string(),
                description:
                    "Read the sentence that states the reset's kind, if the document has one."
                        .to_string(),
                downstream_impact: "The reset clause states the document's kind.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "state_an_unstated_kind".to_string(),
                description: "FSMGen states a reset whose kind the document does not give \
                              (EXTRACTION-REPRESENTATION-GAPS.7.d)."
                    .to_string(),
                downstream_impact: "No kind is chosen for the document.".to_string(),
            },
        ],
    }
}

/// EXTRACTION-REPRESENTATION-GAPS.24.b — the actor drives a signal the document gives one instance per actor of a
/// role, and the `.isf` declares it once. Every instance follows the declared signal's contract (`.24.a`); how
/// many instances there are, the document does not state, and fanning one out to each target is composition
/// (`.14`).
fn family_driven_once_residual_packet(
    actor_name: &str,
    families: &[&SignalFamilyRecord],
) -> ResidualDecisionPacket {
    let listed = families
        .iter()
        .map(|family| {
            format!(
                "`{}` (one per {}; {})",
                family.signal_name,
                family.per_actor,
                family.supporting_statement_ids.join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join(", ");
    ResidualDecisionPacket {
        packet_id: "isf_family_driven_once".to_string(),
        question: format!("How many instances of each per-target output does `{actor_name}` drive?"),
        why_unresolved: format!(
            "The document gives these outputs one instance per actor of a role: {listed}. Each instance follows \
             the signal's contract, and the document does not state how many there are. This `.isf` declares one \
             instance; driving one per target is composition (EXTRACTION-REPRESENTATION-GAPS.24, .14)."
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "one_instance_per_module".to_string(),
                description: "Each target's instance is its own port, fanned out by the composition that \
                              instantiates this actor once per target, as FSMGen's interconnect examples do."
                    .to_string(),
                downstream_impact: "This `.isf` stays as emitted; the fan-out is the composition's.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "state_the_instance_count".to_string(),
                description: "A configuration states the number of targets, and the actor declares one port \
                              per target."
                    .to_string(),
                downstream_impact: "The port becomes a vector or a family of ports sized by that number."
                    .to_string(),
            },
        ],
    }
}

fn ports_not_on_actor_residual_packet(
    initiator: &str,
    ports: &BTreeMap<String, &'static str>,
) -> ResidualDecisionPacket {
    let mut by_reason: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (signal, reason) in ports {
        by_reason.entry(reason).or_default().push(signal);
    }
    let listed = by_reason
        .iter()
        .map(|(reason, signals)| format!("{reason}: {}", signals.join(", ")))
        .collect::<Vec<_>>()
        .join("; ");
    ResidualDecisionPacket {
        packet_id: "isf_ports_not_on_the_lowered_actor".to_string(),
        question: format!(
            "Which of these interface signals are ports of `{initiator}`, and in which direction?"
        ),
        why_unresolved: format!(
            "{} interface signal(s) are not declared on `{initiator}`, because the document grounds no \
             direction for them on it. {listed}. Declaring such a signal as the actor's output with a \
             drive would state that it drives a signal the document gives to another actor, or to none.",
            ports.len()
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "ground_the_relation".to_string(),
                description:
                    "Read the sentence or table cell that relates the signal to this actor, \
                              if the document states one."
                        .to_string(),
                downstream_impact: "The port returns with the direction the document gives it."
                    .to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "compose_the_other_actor".to_string(),
                description: "Lower the actor the document relates the signal to, and compose it \
                              (EXTRACTION-REPRESENTATION-GAPS.14)."
                    .to_string(),
                downstream_impact: "The signal becomes that actor's port, connected to this one \
                                    where the document states a connection."
                    .to_string(),
            },
        ],
    }
}

/// The residual naming every symbolic signal width that stayed `(width 1)`.
fn unlowered_signal_width_residual_packet(
    unlowered: &[UnloweredSignalWidth],
) -> ResidualDecisionPacket {
    let mut by_reason: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for u in unlowered {
        by_reason
            .entry(u.reason)
            .or_default()
            .push(format!("{} `{}`", u.signal, u.width));
    }
    let listed = by_reason
        .iter()
        .map(|(reason, signals)| format!("{reason}: {}", signals.join(", ")))
        .collect::<Vec<_>>()
        .join("; ");
    ResidualDecisionPacket {
        packet_id: "isf_signal_width_unlowered".to_string(),
        question: "How should a signal width that no actor parameter can state be lowered?"
            .to_string(),
        why_unresolved: format!(
            "{} signal(s) keep `(width 1)` although the document states a symbolic width. {listed}. \
             FSMGen resolves an interface width only against a positive integer, a declared actor \
             constant, or a scalar actor parameter, and refuses arbitrary expressions, so emitting \
             the expression would fail closed and computing a number would drop the dependency.",
            unlowered.len()
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "derived_actor_parameter".to_string(),
                description: "FSMGen gains a spelling for a parameter derived from another \
                              (`(STRB_W (/ DATA_W 8))`)."
                    .to_string(),
                downstream_impact:
                    "An upstream contract change (KG-ISF-COMPLETENESS.2a.vii.d); exact and generic."
                        .to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "rename_colliding_parameter".to_string(),
                description: "For a colliding name, declare the parameter under a name the actor \
                              does not already use."
                    .to_string(),
                downstream_impact: "The emitted name would differ from the document's; only worth \
                                    it if a collision is ever observed on a live chain."
                    .to_string(),
            },
        ],
    }
}

/// KG-ISF-COMPLETENESS.2a.ii: select the protocol's primary/INITIATOR actor from the actor-port
/// graph, structurally and universally (ADR 0006 — no `Manager`/`Requester`/`Host` name list).
///
/// Direction in a protocol is actor-relative (a signal one actor drives, another reads), but the
/// emitted `.isf` is a single flat module, so it must be lowered from ONE actor's perspective. The
/// owner's choice (`2026-06-17`) is the INITIATOR's perspective. The initiator is the actor that
/// DRIVES the request and reads back the response, so structurally it is a **net producer** — its
/// output (`Drives`) ports strictly exceed its input (`Reads`) ports — with the largest driving
/// footprint. We therefore restrict to net producers (`out > in`, which by construction excludes a
/// balanced prose-fragment actor at out=in and an input-dominant completer) and pick the one maximizing
/// `(out, in)` lexicographically: most
/// driven signals first, then most read signals (a real initiator also reads responses, which breaks
/// a tie against an output-only register/fragment). Exact `(out, in)` ties resolve to the
/// lexicographically last actor name because `BTreeMap` iteration is ascending and `max_by_key`
/// replaces an earlier equal maximum. Returns the raw actor name (the caller sanitizes it), or
/// `None` when no actor is a net producer (e.g. a
/// register/command doc with no wire actors) — the honest residual that keeps the current behavior.
///
/// Retained wire-document controls confirm that this structural ranking selects the reviewed initiator.
/// EXTRACTION-REPRESENTATION-GAPS.32 — the initiator of a document with a system contract, counted without the
/// system clock and reset: every actor reads them, so they are no part of which actor issues requests. Wishbone's
/// MASTER drives 7 signals and reads 5 besides `CLK_I` and `RST_I`; counted with them it tied at 7 and no actor
/// was chosen, and the emitted module declared both sides of every wire.
pub(crate) fn select_initiator_for(intent_ir: &IntentIr) -> Option<String> {
    let infrastructure: BTreeSet<&str> = intent_ir
        .system_contract
        .iter()
        .flat_map(|contract| {
            [
                contract.clock_signal.as_str(),
                contract.reset_signal.as_str(),
            ]
        })
        .collect();
    let protocol_ports: Vec<ActorPortRecord> = intent_ir
        .actor_ports
        .iter()
        .filter(|port| !infrastructure.contains(port.signal_name.as_str()))
        .cloned()
        .collect();
    select_initiator_actor(&protocol_ports)
}

pub(crate) fn select_initiator_actor(actor_ports: &[ActorPortRecord]) -> Option<String> {
    let mut by_actor: BTreeMap<String, (u32, u32)> = BTreeMap::new();
    for port in actor_ports {
        let entry = by_actor.entry(port.actor_name.clone()).or_insert((0, 0));
        match port.direction {
            ActorRelativeDirection::Output => entry.0 += 1,
            ActorRelativeDirection::Input => entry.1 += 1,
            ActorRelativeDirection::InOut | ActorRelativeDirection::Unknown => {}
        }
    }
    by_actor
        .into_iter()
        .filter(|(_, (out, inp))| out > inp)
        // Pick max (out, in); BTreeMap iteration is name-ascending and `Iterator::max_by_key`
        // replaces an earlier equal maximum, so the lexicographically last name wins exact ties.
        .max_by_key(|(_, (out, inp))| (*out, *inp))
        .map(|(name, _)| name)
}

/// KG-ISF-COMPLETENESS.2a.ii: the grounded per-signal direction the INITIATOR actor has for each
/// signal, from the actor-port graph — `Drives` → `(output)`, `Reads` → `(input)`. A signal the
/// initiator both drives and reads (or whose graph rows disagree), or one it touches only as
/// `InOut`/`Unknown`, is OMITTED so the caller keeps the honest default rather than guess. Empty when
/// there is no initiator (`select_initiator_actor` → `None`), so the emitted interface is byte-identical
/// to the pre-`.2a.ii` default-`output` behavior in that case.
fn initiator_perspective_directions(
    actor_ports: &[ActorPortRecord],
    initiator: &str,
) -> BTreeMap<String, IsfDirection> {
    let mut by_signal: BTreeMap<String, BTreeSet<IsfDirection>> = BTreeMap::new();
    for port in actor_ports {
        if port.actor_name != initiator {
            continue;
        }
        let dir = match port.direction {
            ActorRelativeDirection::Output => IsfDirection::Output,
            ActorRelativeDirection::Input => IsfDirection::Input,
            ActorRelativeDirection::InOut | ActorRelativeDirection::Unknown => continue,
        };
        by_signal
            .entry(port.signal_name.clone())
            .or_default()
            .insert(dir);
    }
    by_signal
        .into_iter()
        .filter_map(|(signal, dirs)| match dirs.len() {
            1 => dirs.into_iter().next().map(|d| (signal, d)),
            _ => None,
        })
        .collect()
}

/// Map an arbitrary label to a valid HDL identifier (`[A-Za-z_]\w*`) — the contract FSMGen's strict
/// frontend enforces for the top-level module name and for target identifiers. Uses an ALLOWLIST (keep
/// `[A-Za-z0-9_]`, map every other char to `_`) rather than a denylist of "bad" punctuation: a denylist
/// can never enumerate every offender, and in fact missed the unicode arrow `→` that a prose-fragment
/// initiator actor name carries — which can malform the whole emitted `.isf`. The allowlist is
/// byte-identical to the prior denylist on
/// every ASCII-punctuation input the denylist already covered (so the wire golds — whose names are pure
/// alphanumeric — are unaffected); it only ever changes a name that was already broken. Universal, no
/// name list (ADR 0006; [[feedback_avoid_denylists_prefer_structural]]).
pub(crate) fn sanitize_isf_name(raw: &str) -> String {
    let mut name: String = raw
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect();
    // Collapse consecutive underscores and strip leading/trailing.
    while name.contains("__") {
        name = name.replace("__", "_");
    }
    name = name.trim_matches('_').to_string();
    if name.is_empty() {
        name = "unnamed".to_string();
    }
    // HDL identifiers must start with a letter or underscore.
    if name.starts_with(|c: char| c.is_ascii_digit()) {
        name.insert_str(0, "reg_");
    }
    name
}

fn collect_branch_actions(branches: &[ControlBranchRecord]) -> Vec<ControlActionRecord> {
    let mut actions = Vec::new();
    for branch in branches {
        actions.extend(branch.actions.clone());
    }
    actions
}

// First consequent that names a single signal, for the `bounded_eventually`
// contract target. `HandshakeComplete` names two signals and no single
// eventual target, so it is not represented here (ISF-TEMPORAL-LOWERING.2.3
// maps it to a residual decision instead of fabricating syntax).
// R16-CONTRACT-IR.3: production lowering now uses `classify_actor_contract`.
// `classify_temporal_rule` + these helpers are retained test-only as the
// parity ORACLE (`classify_actor_contract` must equal it pointwise).
#[cfg(test)]
fn temporal_consequent_signal(rule: &TemporalRuleRecord) -> Option<String> {
    rule.consequents.iter().find_map(|p| match p {
        TemporalPredicateRecord::SignalValue { signal_name, .. }
        | TemporalPredicateRecord::ActorDrivesSignal { signal_name, .. }
        | TemporalPredicateRecord::ActorMaintainsSignalStable { signal_name, .. }
        | TemporalPredicateRecord::SignalStable { signal_name, .. }
        | TemporalPredicateRecord::ActorSamplesSignal { signal_name, .. }
        | TemporalPredicateRecord::SignalSampled { signal_name, .. } => Some(signal_name.clone()),
        TemporalPredicateRecord::HandshakeComplete { .. } => None,
    })
}

/// The single ISF disposition of one `temporal_rule`
/// (ISF-TEMPORAL-LOWERING `.1` mapping #1/#3/#4). The three arms are
/// mutually exclusive so a rule is lowered exactly one way (or not at all).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TemporalRuleDisposition {
    /// Windowed bounded-eventually → synthetic transaction carrying a FSMGen
    /// verification-family property `(assert <prop>)`. `prop` is the anchored
    /// monitor `(monitor (within s N))` (unguarded) or the guarded implication
    /// `(=> g (within s [min] max))` (`FSMGEN-ASSERT-LOWERING`). `name` labels
    /// the synthetic transaction (`txn_temporal_<name>`).
    Contract { name: String, prop: String },
    /// Non-windowed value/guard→drive → actor
    /// `(rule <name> [<condition>] (<signal> <value>))` (`.2.3` #3).
    Rule {
        name: String,
        condition: String,
        signal: String,
        value: String,
    },
    /// Ready/valid handshake barrier → synthetic `(transaction …
    /// (stage <name> (ready <ready>)(valid <valid>)))`
    /// (`R16-CONTRACT-IR.4`; FSMGen `ready_valid_barrier`, accepted at
    /// pin `9bfb9a20`).
    Stage {
        name: String,
        ready: String,
        valid: String,
    },
    /// No representable supported ISF construct → explicit residual
    /// decision; syntax is never fabricated (`.2.3` #4,
    /// `fsmgen-contract-authority`).
    Residual { rule_id: String, reason: String },
}

/// First consequent that is a concrete `SignalValue` (a signal name *and* a
/// value to drive). The actor/stability/sample predicates name a signal but
/// carry no concrete value, so they are not a representable `(rule …)` drive.
#[cfg(test)]
fn temporal_drive_consequent(rule: &TemporalRuleRecord) -> Option<(String, String)> {
    rule.consequents.iter().find_map(|p| match p {
        TemporalPredicateRecord::SignalValue {
            signal_name, value, ..
        } => Some((signal_name.clone(), value.clone())),
        _ => None,
    })
}

/// EXTRACTION-REPRESENTATION-GAPS.8 — lower the three obligation families that name a signal and a level:
/// conditional rules, signal constraints and signal invariants. A drive's value is a level the document
/// states, resolved through the signal's own polarity; an obligation that states none is returned as
/// refused, never given a default (before this, all three defaulted a missing value to `1`). Only
/// obligations on a declared signal are considered, as before.
///
/// EXTRACTION-REPRESENTATION-GAPS.22 — and a rule keeps the condition it states, or is refused: before, a
/// condition of several words was dropped (the rule drove unconditionally) and a one-word condition became
/// `(== <word> 1)` on whatever the word was. `reset` is the actor's reset (the system contract's), which a
/// guard may test and no rule drives.
#[expect(
    clippy::too_many_arguments,
    reason = "obligation lowering consumes the three obligation families and the document's own readings of a condition"
)]
fn lower_obligations(
    conditional_rules: &[crate::ir::source::ConditionalRuleRecord],
    signal_constraints: &[crate::ir::source::SignalConstraintRecord],
    temporal_invariants: &[crate::ir::intent::TemporalInvariantRecord],
    signal_names: &BTreeSet<String>,
    input_signal_names: &BTreeSet<String>,
    reset: Option<&str>,
    polarities: &BTreeMap<String, SignalPolarity>,
    operation_kinds: &[OperationKindRecord],
) -> (Vec<IsfRule>, Vec<RefusedObligation>) {
    let guard_signals: BTreeSet<String> = signal_names
        .iter()
        .cloned()
        .chain(reset.map(str::to_string))
        .collect();
    let mut rules = Vec::new();
    let mut refused = Vec::new();
    let mut lower = |name: String,
                     condition: Result<String, LevelRefusal>,
                     signal: &str,
                     level: Result<String, LevelRefusal>| {
        // EXTRACTION-REPRESENTATION-GAPS.10.a — the actor never drives its own input: an obligation on one is the
        // driving actor's, and lowered as this actor's `(rule …)` it states that this actor drives it.
        let level = if input_signal_names.contains(signal) {
            Err(LevelRefusal::DrivesAnInput)
        } else {
            level
        };
        let lowered = level
            .and_then(|level| resolve_level(signal, &level, polarities))
            .and_then(|value| condition.map(|condition| (condition, value)));
        match lowered {
            Ok((condition, value)) => rules.push(IsfRule {
                name,
                condition,
                drives: vec![(signal.to_string(), value)],
            }),
            Err(refusal) => refused.push(RefusedObligation {
                name,
                signal: signal.to_string(),
                refusal,
            }),
        }
    };

    // EXTRACTION-REPRESENTATION-GAPS.10.g.ii — a condition naming a kind of operation reads as the level stating it.
    let condition = |text: &str| {
        operation_kind_condition(text, operation_kinds).unwrap_or_else(|| text.to_string())
    };
    for (cr_idx, cr) in conditional_rules.iter().enumerate() {
        let sig = cr.consequent_signal.as_deref().unwrap_or("unknown");
        if !signal_names.contains(sig) {
            continue;
        }
        lower(
            format!("rule_{}", cr_idx),
            condition_guard(
                &condition(&cr.antecedent_text),
                &guard_signals,
                reset,
                polarities,
            ),
            sig,
            action_stated_level(&cr.consequent_action),
        );
    }

    for (sc_idx, sc) in signal_constraints.iter().enumerate() {
        if !signal_names.contains(&sc.subject_signal) {
            continue;
        }
        let level = match signal_constraint_level(&sc.constraint_kind, sc.negated) {
            ConstraintLevel::Level(level) => Ok(level),
            ConstraintLevel::NegatedValue | ConstraintLevel::NotALevel => {
                Err(LevelRefusal::NoLevelStated)
            }
            ConstraintLevel::TieOff => Err(LevelRefusal::TieOff),
            ConstraintLevel::Hold => Err(LevelRefusal::Hold),
        };
        let stated = condition(sc.condition_text.as_deref().unwrap_or(""));
        lower(
            format!("constraint_{}", sc_idx),
            condition_guard(&stated, &guard_signals, reset, polarities),
            &sc.subject_signal,
            level,
        );
    }

    for inv in temporal_invariants {
        if inv.subject_signal.is_empty() || !signal_names.contains(&inv.subject_signal) {
            continue;
        }
        // Only a level obligation drives; stability, matching and bounds are properties (`.11`).
        let level = match (inv.invariant_kind, inv.target_value.as_deref()) {
            (TemporalInvariantKind::MustBeAsserted, Some(level)) => Ok(level.to_string()),
            _ => Err(LevelRefusal::NoLevelStated),
        };
        let condition = match (
            inv.condition_signal.as_deref(),
            inv.condition_value.as_deref(),
        ) {
            (None, _) => Ok(String::new()),
            (Some(signal), Some(value)) if guard_signals.contains(signal) => {
                resolve_level(signal, value, polarities)
                    .map(|level| format!("(== {signal} {level})"))
            }
            (Some(_), _) => Err(LevelRefusal::ConditionNotLowered),
        };
        lower(
            sanitize_isf_name(&inv.invariant_id),
            condition,
            &inv.subject_signal,
            level,
        );
    }
    (rules, refused)
}

/// EXTRACTION-REPRESENTATION-GAPS.22 — the guard a stated condition gives, or why it gives none. No condition
/// is no guard. The whole condition `SIGNAL is LEVEL`, on a signal a rule may test, is `(== SIGNAL LEVEL)` with
/// the level resolved through the signal's stated polarity; *reset*, *in reset* or *during reset* is the actor's
/// reset at its asserted level. Anything else is refused: dropping it would make the rule unconditional, and a
/// guard on a word that is not a declared signal makes FSMGen invent a port for it.
fn condition_guard(
    text: &str,
    guard_signals: &BTreeSet<String>,
    reset: Option<&str>,
    polarities: &BTreeMap<String, SignalPolarity>,
) -> Result<String, LevelRefusal> {
    let text = text.trim().trim_end_matches(['.', ',', ';']).trim();
    if text.is_empty() || text == "true" {
        return Ok(String::new());
    }
    let (signal, level) = match text.split_whitespace().collect::<Vec<_>>().as_slice() {
        ["reset"] | ["in" | "during", "reset"] => {
            (reset.ok_or(LevelRefusal::ConditionNotLowered)?, "asserted")
        }
        [signal, is, level] if is.eq_ignore_ascii_case("is") && guard_signals.contains(*signal) => {
            (*signal, *level)
        }
        _ => return Err(LevelRefusal::ConditionNotLowered),
    };
    resolve_level(signal, level, polarities).map(|level| format!("(== {signal} {level})"))
}

/// EXTRACTION-REPRESENTATION-GAPS.8 — why a stated obligation did not become a drive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LevelRefusal {
    /// `asserted`/`deasserted` on a signal whose polarity the document does not state (ADR 0037 §5).
    PolarityUnknown,
    /// The obligation names no level: a stability, match or validity property, a `must not be <value>`,
    /// or an action whose text is just `must`.
    NoLevelStated,
    /// The stated value is a word, not a level (`VALID`, an enum member name).
    NotALevel,
    /// EXTRACTION-REPRESENTATION-GAPS.22 — the rule states a condition that is not `SIGNAL is LEVEL` on a
    /// signal a rule may test, nor the reset.
    ConditionNotLowered,
    /// EXTRACTION-REPRESENTATION-GAPS.10.a — the obligation is on a signal the actor declares as an input, so
    /// another actor meets it; from this actor's side it is an assumption, not a drive.
    DrivesAnInput,
    /// EXTRACTION-REPRESENTATION-GAPS.10.e — a tie-off: the level an input is tied to when its driver omits the
    /// signal. No actor drives it; `.isf` states it once FSMGen ships an absent-input value (`.4`).
    TieOff,
    /// EXTRACTION-REPRESENTATION-GAPS.25.c — a hold (*must remain LEVEL until RELEASE*): not a constant level. Its
    /// temporal rule carries it, with the release in the guard, or lists it.
    Hold,
}

impl LevelRefusal {
    fn describe(self) -> &'static str {
        match self {
            Self::PolarityUnknown => {
                "states `asserted`/`deasserted`, but the document never states the signal's polarity"
            }
            Self::NoLevelStated => {
                "states no level to drive (a property, an exclusion, or a bare `must`)"
            }
            Self::NotALevel => "states a word that is not a level or number",
            Self::ConditionNotLowered => {
                "states a condition that is not `SIGNAL is LEVEL` on a declared signal, nor the reset"
            }
            Self::DrivesAnInput => {
                "is an obligation on a signal the actor reads, which another actor meets: from this actor's \
                 side an assumption, which `.isf` states only inside a transaction \
                 (EXTRACTION-REPRESENTATION-GAPS.15)"
            }
            Self::TieOff => {
                "is a tie-off: the level an input is tied to when its driver omits the signal, which no actor \
                 drives; `.isf` states it once FSMGen ships an absent-input value (EXTRACTION-REPRESENTATION-GAPS.4)"
            }
            Self::Hold => {
                "is a hold (*must remain … until …*), not a constant level: its temporal rule lowers it with the \
                 release in the guard, or lists it"
            }
        }
    }
}

/// The polarity each signal's own records state. A signal recorded with both polarities is left out:
/// two answers are never chosen between.
/// EXTRACTION-REPRESENTATION-GAPS.25.a — the condition a temporal rule's source constraint states, when none of
/// it became the rule's guard. The IR keeps such a rule (*valid during the address phase* has no signal
/// predicate), but lowered with no guard it would apply always, which the document never says.
/// EXTRACTION-REPRESENTATION-GAPS.26 — a conditional rule's antecedent is a stated condition too, and a condition
/// read only in part now yields no guard at all (`parse_temporal_condition`), so both sources are checked.
fn unread_stated_condition<'a>(
    contract: &crate::ir::contract::ActorContract,
    constraints: &'a [crate::ir::source::SignalConstraintRecord],
    conditional_rules: &'a [crate::ir::source::ConditionalRuleRecord],
) -> Option<&'a str> {
    if !contract.guard_candidates.is_empty() {
        return None;
    }
    let source = contract.source_rule_id.as_deref()?;
    let stated = if let Some(id) = source.strip_prefix("temporal_signal_constraint_") {
        constraints
            .iter()
            .find(|constraint| constraint.constraint_id == id)?
            .condition_text
            .as_deref()?
    } else {
        let id = source.strip_prefix("temporal_conditional_rule_")?;
        conditional_rules
            .iter()
            .find(|rule| rule.rule_id == id)?
            .antecedent_text
            .as_str()
    };
    Some(stated.trim()).filter(|condition| !condition.is_empty())
}

/// EXTRACTION-REPRESENTATION-GAPS.25.c — the release of a hold (*must remain LEVEL until RELEASE*) whose temporal
/// rule carries no exception. SemanticIR reads the release into the rule's exceptions, or leaves it none when the
/// release cannot be read, so a hold with no negated guard candidate has lost its release.
fn unread_hold_release<'a>(
    contract: &crate::ir::contract::ActorContract,
    constraints: &'a [crate::ir::source::SignalConstraintRecord],
) -> Option<&'a str> {
    if contract
        .guard_candidates
        .iter()
        .any(|candidate| matches!(candidate, crate::ir::contract::Condition::Ne { .. }))
    {
        return None;
    }
    let id = contract
        .source_rule_id
        .as_deref()?
        .strip_prefix("temporal_signal_constraint_")?;
    match &constraints
        .iter()
        .find(|constraint| constraint.constraint_id == id)?
        .constraint_kind
    {
        crate::ir::source::SignalConstraintKind::MustRemainUntil { release, .. } => {
            Some(release.as_str())
        }
        _ => None,
    }
}

/// EXTRACTION-REPRESENTATION-GAPS.7.b — the system reset's polarity, which the contract states from the
/// document, joins the stated polarities. A polarity record that disagrees removes it: refused, never chosen.
fn with_reset_polarity(
    mut polarities: BTreeMap<String, SignalPolarity>,
    contract: Option<&crate::ir::semantic::SystemContractRecord>,
) -> BTreeMap<String, SignalPolarity> {
    let Some(contract) = contract else {
        return polarities;
    };
    let stated = match contract.reset_polarity {
        SystemResetPolarity::ActiveHigh => SignalPolarity::ActiveHigh,
        SystemResetPolarity::ActiveLow => SignalPolarity::ActiveLow,
        SystemResetPolarity::Unknown => return polarities,
    };
    match polarities.get(&contract.reset_signal) {
        Some(recorded) if *recorded != stated => {
            polarities.remove(&contract.reset_signal);
        }
        _ => {
            polarities.insert(contract.reset_signal.clone(), stated);
        }
    }
    polarities
}

fn signal_polarity_map(records: &[SignalPolarityRecord]) -> BTreeMap<String, SignalPolarity> {
    let mut stated: BTreeMap<&str, BTreeSet<SignalPolarity>> = BTreeMap::new();
    for record in records {
        stated
            .entry(record.signal_name.as_str())
            .or_default()
            .insert(record.polarity);
    }
    stated
        .into_iter()
        .filter_map(|(signal, polarities)| match polarities.len() {
            1 => polarities
                .into_iter()
                .next()
                .map(|p| (signal.to_string(), p)),
            _ => None,
        })
        .collect()
}

/// The ISF value of `raw` as a level of `signal`. `HIGH`/`LOW` and numbers are levels on their own;
/// `asserted`/`deasserted` are levels only through the signal's stated polarity, so on a signal of
/// unknown polarity they refuse rather than assume active-high. Anything else is not a level.
fn resolve_level(
    signal: &str,
    raw: &str,
    polarities: &BTreeMap<String, SignalPolarity>,
) -> Result<String, LevelRefusal> {
    let t = raw.trim();
    let asserted = match t.to_ascii_lowercase().as_str() {
        "1" | "high" | "true" | "set" => return Ok("1".to_string()),
        "0" | "low" | "false" | "clear" => return Ok("0".to_string()),
        "asserted" | "assert" | "active" => true,
        "deasserted" | "deassert" | "inactive" => false,
        _ => {
            // Pure unsigned decimal / hex / binary integer literals only.
            let is_dec = !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit());
            let is_hex = t.len() > 2
                && (t.starts_with("0x") || t.starts_with("0X"))
                && t[2..].bytes().all(|b| b.is_ascii_hexdigit());
            let is_bin = t.len() > 2
                && (t.starts_with("0b") || t.starts_with("0B"))
                && t[2..].bytes().all(|b| b == b'0' || b == b'1');
            return if is_dec || is_hex || is_bin {
                Ok(t.to_string())
            } else {
                Err(LevelRefusal::NotALevel)
            };
        }
    };
    match polarities.get(signal) {
        Some(SignalPolarity::ActiveHigh) => Ok(if asserted { "1" } else { "0" }.to_string()),
        Some(SignalPolarity::ActiveLow) => Ok(if asserted { "0" } else { "1" }.to_string()),
        None => Err(LevelRefusal::PolarityUnknown),
    }
}

/// The level a conditional rule's action states (`must be LOW`, `must not be asserted`, `must be
/// 0b000`), with `not` applied to a level word; `must not be <value>` and an action with no level
/// (`must`, `must remain stable`) state none.
fn action_stated_level(action: &str) -> Result<String, LevelRefusal> {
    let words: Vec<String> = action
        .split_whitespace()
        .map(|w| {
            w.trim_matches(|c: char| c == '.' || c == ',')
                .to_ascii_lowercase()
        })
        .collect();
    let Some(be) = words.iter().position(|w| w == "be") else {
        return Err(LevelRefusal::NoLevelStated);
    };
    let negated = words[..be].iter().any(|w| w == "not");
    let Some(word) = words.get(be + 1) else {
        return Err(LevelRefusal::NoLevelStated);
    };
    let complement = match word.as_str() {
        "high" => "low",
        "low" => "high",
        "asserted" => "deasserted",
        "deasserted" => "asserted",
        _ if negated => return Err(LevelRefusal::NoLevelStated),
        _ => return Ok(word.clone()),
    };
    Ok(if negated { complement } else { word.as_str() }.to_string())
}

/// An obligation on a declared signal that stated no drivable level.
struct RefusedObligation {
    name: String,
    signal: String,
    refusal: LevelRefusal,
}

/// The one residual naming every obligation that did not become a drive, grouped by why.
fn refused_obligations_residual_packet(refused: &[RefusedObligation]) -> ResidualDecisionPacket {
    let mut by_refusal: BTreeMap<LevelRefusal, Vec<String>> = BTreeMap::new();
    for obligation in refused {
        by_refusal
            .entry(obligation.refusal)
            .or_default()
            .push(format!("{} ({})", obligation.name, obligation.signal));
    }
    let listed = by_refusal
        .iter()
        .map(|(refusal, names)| {
            format!(
                "{} {}: {}",
                names.len(),
                refusal.describe(),
                names.join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    ResidualDecisionPacket {
        packet_id: "isf_obligations_not_lowered".to_string(),
        question: "Which level should each of these obligations drive, if any?".to_string(),
        why_unresolved: format!(
            "{} obligation(s) on declared signals were not lowered to a drive, because a drive's value \
             must be a level the document states and its guard the condition the document states: {listed}. \
             Giving them a default level, or dropping the condition, would state what the document never did.",
            refused.len()
        ),
        automation_confidence: AutomationConfidence::High,
        candidate_interpretations: vec![CandidateInterpretation {
            interpretation_id: "state_the_level".to_string(),
            description: "Read the level from the document: the signal's polarity for `asserted`, the \
                          obligation's condition and value for the rest; lower a property (stability, \
                          validity) as an assertion instead of a drive; read a prose condition as a guard \
                          on declared signals."
                .to_string(),
            downstream_impact: "Owned by EXTRACTION-REPRESENTATION-GAPS (.17 polarity extraction, .10 and .22 \
                                conditions, .11 assertions)."
                .to_string(),
        }],
    }
}

/// EXTRACTION-REPRESENTATION-GAPS.46.b — the widths a document states as a number for each signal, over its interface
/// records and actor ports, one included; a signal whose stated numbers disagree has none. The adapter's own width of
/// 1 is also its default for a width the document writes symbolically (`DATA_WIDTH/8`), so a parity over every bit
/// uses only a width stated here.
fn stated_numeric_widths(intent_ir: &IntentIr) -> BTreeMap<String, u32> {
    let mut by_signal: BTreeMap<String, BTreeSet<u32>> = BTreeMap::new();
    let records = intent_ir
        .interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .map(|record| (record.signal_name.as_str(), record.width_hint.as_ref()));
    let ports = intent_ir
        .actor_ports
        .iter()
        .map(|port| (port.signal_name.as_str(), port.width_hint.as_ref()));
    for (name, hint) in records.chain(ports) {
        if let Some(WidthHint::Numeric(n)) = hint {
            by_signal.entry(name.to_string()).or_default().insert(*n);
        }
    }
    by_signal
        .into_iter()
        .filter_map(
            |(name, widths)| match widths.into_iter().collect::<Vec<_>>().as_slice() {
                [width] => Some((name, *width)),
                _ => None,
            },
        )
        .collect()
}

/// Why a check signal's parity was not lowered (`.46.b`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ParityRefusal {
    NoStatedSense,
    ByteGranular,
    NotDrivenHere,
    CoveredNotDrivenHere,
    CoveredWidthNotStated,
    EnableNotSignals,
}

impl ParityRefusal {
    fn describe(self) -> &'static str {
        match self {
            Self::NoStatedSense => "the document states no parity sense",
            Self::ByteGranular => {
                "a check wider than one bit, one per byte of a parameterised width (`.46.c`)"
            }
            Self::NotDrivenHere => "the check signal is not an output of this actor",
            Self::CoveredNotDrivenHere => "a covered signal is not an output of this actor",
            Self::CoveredWidthNotStated => "a covered signal's width is not stated as a number",
            Self::EnableNotSignals => {
                "the enable is not a declared signal or a conjunction of them"
            }
        }
    }
}

/// EXTRACTION-REPRESENTATION-GAPS.46.b — the parity rule each one-bit check signal states over the signals it
/// covers, in the actor that drives them: `(! (^ …))` over every covered bit for odd parity, the chain itself for
/// even, guarded by its enable read as declared signals or the actor's reset, each `1` (*AWVALID*; *PSELx &
/// PWRITE*; *ARESETn*, which a guard may test as `lower_obligations` lets it). A check wider than
/// one bit, one whose covered widths the document does not state as numbers, one this actor does not drive, or one
/// whose enable is a phase or an expression (*Write data phase*; *HTRANS != IDLE*) is refused, never approximated.
fn lower_parity_checks(
    records: &[CheckSignalRecord],
    signals: &BTreeSet<IsfSignal>,
    stated_widths: &BTreeMap<String, u32>,
    reset: Option<&str>,
) -> (Vec<IsfRule>, Vec<(String, ParityRefusal)>) {
    let declared: BTreeMap<&str, &IsfSignal> = signals
        .iter()
        .map(|signal| (signal.name.as_str(), signal))
        .collect();
    let is_output = |name: &str| {
        declared
            .get(name)
            .is_some_and(|signal| signal.direction == IsfDirection::Output)
    };
    let mut rules = Vec::new();
    let mut refused = Vec::new();
    for record in records {
        let lowered = (|| {
            let sense = record.parity_sense.ok_or(ParityRefusal::NoStatedSense)?;
            if record.width.as_deref().map(str::trim) != Some("1") {
                return Err(ParityRefusal::ByteGranular);
            }
            if !is_output(&record.check_signal) {
                return Err(ParityRefusal::NotDrivenHere);
            }
            let mut bits: Vec<String> = Vec::new();
            for covered in &record.covered_signals {
                if !is_output(covered) {
                    return Err(ParityRefusal::CoveredNotDrivenHere);
                }
                let signal = declared[covered.as_str()];
                match stated_widths.get(covered) {
                    Some(&width) if width == signal.width && signal.width_param.is_none() => {
                        if width == 1 {
                            bits.push(covered.clone());
                        } else {
                            bits.extend((0..width).map(|bit| format!("{covered}[{bit}]")));
                        }
                    }
                    _ => return Err(ParityRefusal::CoveredWidthNotStated),
                }
            }
            let enable = record.check_enable.as_deref().unwrap_or("");
            let terms: Vec<&str> = enable.split('&').map(str::trim).collect();
            if terms.iter().any(|term| {
                term.is_empty()
                    || !term.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    || !(declared.contains_key(term) || reset == Some(*term))
            }) {
                return Err(ParityRefusal::EnableNotSignals);
            }
            let condition = terms
                .iter()
                .map(|term| format!("(== {term} 1)"))
                .reduce(|left, right| format!("(& {left} {right})"))
                .ok_or(ParityRefusal::EnableNotSignals)?;
            let chain = bits
                .into_iter()
                .reduce(|left, right| format!("(^ {left} {right})"))
                .ok_or(ParityRefusal::CoveredWidthNotStated)?;
            let value = match sense {
                ParitySense::Odd => format!("(! {chain})"),
                ParitySense::Even => chain,
            };
            Ok(IsfRule {
                name: format!("parity_{}", record.check_signal),
                condition,
                drives: vec![(record.check_signal.clone(), value)],
            })
        })();
        match lowered {
            Ok(rule) => rules.push(rule),
            Err(refusal) => refused.push((record.check_signal.clone(), refusal)),
        }
    }
    (rules, refused)
}

/// EXTRACTION-REPRESENTATION-GAPS.46.b — one residual naming every check signal whose parity was not lowered, by
/// reason.
fn parity_checks_not_lowered_packet(refused: &[(String, ParityRefusal)]) -> ResidualDecisionPacket {
    let mut by_refusal: BTreeMap<ParityRefusal, Vec<&str>> = BTreeMap::new();
    for (signal, refusal) in refused {
        by_refusal.entry(*refusal).or_default().push(signal);
    }
    let listed = by_refusal
        .iter()
        .map(|(refusal, signals)| {
            format!(
                "{} {}: {}",
                signals.len(),
                refusal.describe(),
                signals.join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    ResidualDecisionPacket {
        packet_id: "isf_parity_checks_not_lowered".to_string(),
        question: "How should these check signals' parity be stated in `.isf`?".to_string(),
        why_unresolved: format!(
            "{} check signal(s) were not lowered to a parity rule, because a rule's value must be the parity the \
             document states over every bit it covers and its guard the enable the document states: {listed}. \
             Approximating a width, an enable or a driver would state what the document never did.",
            refused.len()
        ),
        automation_confidence: AutomationConfidence::High,
        candidate_interpretations: vec![CandidateInterpretation {
            interpretation_id: "state_the_parity_form".to_string(),
            description: "Lower a byte-granular check once `.isf` states a per-byte reduction over a parameterised \
                          width; read a phase or an expression enable as a guard on declared signals."
                .to_string(),
            downstream_impact: "Owned by EXTRACTION-REPRESENTATION-GAPS (.46.c the FSMGen request).".to_string(),
        }],
    }
}

/// Production-side guard for an `ActorContract`, parity-matched by construction
/// to the oracle's `temporal_antecedent_condition` (both call `guard_from_candidates`).
fn actor_guard_condition(
    guard_candidates: &[crate::ir::contract::Condition],
    declared_signals: &BTreeSet<String>,
    polarities: &BTreeMap<String, SignalPolarity>,
) -> Result<String, String> {
    use crate::ir::contract::Condition;
    guard_from_candidates(
        guard_candidates.iter().filter_map(|g| match g {
            Condition::Eq { signal, value } => Some((signal.as_str(), value.as_str())),
            Condition::Ne { .. } => None,
        }),
        guard_candidates.iter().filter_map(|g| match g {
            Condition::Ne { signal, value } => Some((signal.as_str(), value.as_str())),
            Condition::Eq { .. } => None,
        }),
        declared_signals,
        polarities,
    )
}

/// A temporal rule's guard: its candidates `(signal, value)` are the rule's conditions, all of which hold, so
/// each becomes `(== <signal> <level>)` and several are joined as FSMGen's `(& …)`; none gives the empty string
/// (an unconditional rule). EXTRACTION-REPRESENTATION-GAPS.22: a candidate on a signal the rule may not test,
/// or one whose value is no level, refuses the WHOLE rule. Keeping only the first usable candidate, as before,
/// silently dropped the rest; EXTRACTION-REPRESENTATION-GAPS.8 already refused a level that hangs on an unstated
/// polarity, since lowering it as `1` would assume active-high.
fn guard_from_candidates<'a>(
    candidates: impl Iterator<Item = (&'a str, &'a str)>,
    exceptions: impl Iterator<Item = (&'a str, &'a str)>,
    declared_signals: &BTreeSet<String>,
    polarities: &BTreeMap<String, SignalPolarity>,
) -> Result<String, String> {
    let level_of = |signal: &str, value: &str| -> Result<String, String> {
        if !declared_signals.contains(signal) {
            return Err(format!(
                "guard `{signal} {value}` names a signal the emitted `.isf` does not declare"
            ));
        }
        resolve_level(signal, value, polarities)
            .map_err(|refusal| format!("guard `{signal} {value}` {}", refusal.describe()))
    };
    let mut levels: Vec<(&str, String)> = Vec::new();
    for (signal, value) in candidates {
        let level = level_of(signal, value)?;
        // The same condition twice is one condition; the same signal at two levels is a contradiction,
        // which as a guard would never hold: the rule's conditions were mixed upstream, so it is refused.
        match levels.iter().find(|(known, _)| *known == signal) {
            Some((_, known)) if *known == level => {}
            Some((_, known)) => {
                return Err(format!(
                    "guard conditions contradict: `{signal}` is required at both {known} and {level}"
                ));
            }
            None => levels.push((signal, level)),
        }
    }
    // EXTRACTION-REPRESENTATION-GAPS.26 — an exception (*unless HRESP is ERROR*) holds as its negation. Excepting
    // the very level the guard requires leaves a guard that never holds, so that rule is refused too.
    let mut excepted: Vec<(&str, String)> = Vec::new();
    for (signal, value) in exceptions {
        let level = level_of(signal, value)?;
        if levels
            .iter()
            .any(|(known, known_level)| *known == signal && *known_level == level)
        {
            return Err(format!(
                "guard conditions contradict: `{signal}` is required at {level} and excepted at it"
            ));
        }
        // EXTRACTION-REPRESENTATION-GAPS.25.c — a guard that requires the signal at another level already excludes
        // this one (*… SYSCOACK deasserted …, until SYSCOACK is asserted*), so the negation adds nothing.
        if levels.iter().any(|(known, _)| *known == signal) {
            continue;
        }
        if !excepted
            .iter()
            .any(|(known, known_level)| *known == signal && *known_level == level)
        {
            excepted.push((signal, level));
        }
    }
    // Use the exact declared signal name (FSMGen is case-sensitive and the guard must
    // reference a signal in the interface) — do NOT `sanitize_isf_name` it.
    let terms: Vec<String> = levels
        .iter()
        .map(|(signal, level)| format!("(== {signal} {level})"))
        .chain(
            excepted
                .iter()
                .map(|(signal, level)| format!("(! (== {signal} {level}))")),
        )
        .collect();
    Ok(match terms.as_slice() {
        [] => String::new(),
        [term] => term.clone(),
        _ => format!("(& {})", terms.join(" ")),
    })
}

/// The drive a rule's `value` states for `signal`, or the residual reason it states none.
fn drive_level(
    signal: &str,
    value: &str,
    polarities: &BTreeMap<String, SignalPolarity>,
) -> Result<String, String> {
    resolve_level(signal, value, polarities).map_err(|refusal| match refusal {
        LevelRefusal::PolarityUnknown => {
            format!("temporal rule target value '{value}' on '{signal}' {}", refusal.describe())
        }
        _ => format!(
            "temporal rule target value '{value}' is not an ISF literal; fabricating a value is forbidden"
        ),
    })
}

/// Build the ISF verification-family property (`(assert <prop>)`) for a windowed
/// bounded-eventually (`FSMGEN-ASSERT-LOWERING.3`):
/// - **unguarded** (`guard` empty) → refused (EXTRACTION-REPRESENTATION-GAPS.29): the monitor `(monitor (within
///   s max))` arms when its synthetic transaction's `(on start)` fires, so its window would count from the start of
///   the run, an event the document never names (APB-E *occurs at T1 in Figure 3-1* counts from the diagram's T0);
/// - **guarded** (a representable boolean antecedent) → the faithful implication
///   `(=> g (within s [min] max))` — the antecedent is preserved (the monitor
///   dropped it) and `min` is emitted only when `>= 2` (`(within s max)` already
///   means `##[1:max]`).
///
/// Refuses a *guarded* 0 lower bound too: a `|-> ##[0:N]` consequent has
/// no ISF spelling (FSMGen rejects `(within B 0 MAX)` per `FSMGEN-MIN-WINDOW-
/// CONFIRM`), so that rule stays a residual rather than being mis-lowered.
/// EXTRACTION-REPRESENTATION-GAPS.7.d — `(within SIGNAL …)` states only that SIGNAL becomes true. A windowed event
/// whose value resolves to another level, or to no level at all (APB-E *PADDR, PWRITE, and PWDATA must be valid*), is
/// not that property: lowered, it became `(monitor (within PADDR 1))`, *PADDR is true within a cycle*.
fn windowed_event_refusal(
    signal: &str,
    value: &str,
    polarities: &BTreeMap<String, SignalPolarity>,
) -> Option<String> {
    match drive_level(signal, value, polarities) {
        Ok(level) if level == "1" => None,
        Ok(level) => Some(format!(
            "windowed temporal rule waits for '{signal}' at {level}; `(within {signal} …)` states only that it \
             becomes true"
        )),
        Err(reason) => Some(reason),
    }
}

fn windowed_eventual_prop(
    signal: &str,
    min: Option<u32>,
    max: u64,
    guard: &str,
) -> Result<String, String> {
    if guard.is_empty() {
        return Err(format!(
            "windowed temporal rule for '{signal}' has no antecedent to count its window from; `(on start)` would \
             count it from the start of the run, which the document does not state"
        ));
    }
    match min {
        Some(0) => Err(format!(
            "guarded bounded-eventually for '{signal}' has a 0-cycle lower bound; `|-> ##[0:N]` has no ISF \
             spelling — preserved as residual (FSMGEN-MIN-WINDOW-CONFIRM)"
        )),
        Some(m) if m >= 2 => Ok(format!("(=> {guard} (within {signal} {m} {max}))")),
        _ => Ok(format!("(=> {guard} (within {signal} {max}))")),
    }
}

/// Derive a strict-safe `(rule …)` guard from the rule's antecedents: the
/// first `SignalValue` antecedent whose signal is declared and whose value is
/// an ISF literal yields `(== <signal> <value>)`. Otherwise the empty string
/// (an unconditional rule — strict-verified accepted; the real corpus emits
/// conditionless `(rule name (sig val))`). A bare non-`==` token guard is
/// never produced (FSMGen strict rejects it — `sanitize_rule_condition`).
#[cfg(test)]
fn temporal_antecedent_condition(
    rule: &TemporalRuleRecord,
    declared_signals: &BTreeSet<String>,
    polarities: &BTreeMap<String, SignalPolarity>,
) -> Result<String, String> {
    fn signal_values(
        predicates: &[TemporalPredicateRecord],
    ) -> impl Iterator<Item = (&str, &str)> + '_ {
        predicates.iter().filter_map(|p| match p {
            TemporalPredicateRecord::SignalValue {
                signal_name, value, ..
            } => Some((signal_name.as_str(), value.as_str())),
            _ => None,
        })
    }
    guard_from_candidates(
        signal_values(&rule.antecedents),
        signal_values(&rule.exceptions),
        declared_signals,
        polarities,
    )
}

/// Classify one `temporal_rule` into its single ISF disposition.
/// `declared_signals` MUST be the exact set of signal names the emitter
/// renders into the `.isf` interface, so the classification matches what is
/// actually emitted (a rule/contract may only reference declared signals).
#[cfg(test)]
pub(crate) fn classify_temporal_rule(
    rule: &TemporalRuleRecord,
    declared_signals: &BTreeSet<String>,
    polarities: &BTreeMap<String, SignalPolarity>,
) -> TemporalRuleDisposition {
    // (1) Windowed `bounded_eventually` (mapping #1, wired by `.2.2`).
    if let Some(window) = &rule.cycle_window {
        let bad_window = match window.max_cycles {
            None => Some(
                "windowed temporal rule has no max cycle bound; FSMGen \
                 `(within N)` requires a positive N"
                    .to_string(),
            ),
            Some(0) => Some(
                "windowed temporal rule has a 0-cycle window; FSMGen strict \
                 rejects `(within 0)` — a same-cycle obligation is not a \
                 `bounded_eventually` contract"
                    .to_string(),
            ),
            Some(_) => None,
        };
        if let Some(reason) = bad_window {
            return TemporalRuleDisposition::Residual {
                rule_id: rule.rule_id.clone(),
                reason,
            };
        }
        let within = window
            .max_cycles
            .expect("max_cycles is Some(>=1) — None/0 returned above");
        // EXTRACTION-REPRESENTATION-GAPS.7.d — the event's value, as the contract carries it (`consequent_level_value`).
        let event_refusal = temporal_consequent_signal(rule)
            .filter(|signal| declared_signals.contains(signal))
            .and_then(|signal| match temporal_drive_consequent(rule) {
                Some((_, value)) => windowed_event_refusal(&signal, &value, polarities),
                // EXTRACTION-REPRESENTATION-GAPS.31 — no value, so no eventuality (`contract_from_temporal_rule`).
                None => Some(format!(
                    "windowed temporal rule states no value for '{signal}': `(within {signal} …)` would state that \
                     it becomes true"
                )),
            });
        if let Some(reason) = event_refusal {
            return TemporalRuleDisposition::Residual {
                rule_id: rule.rule_id.clone(),
                reason,
            };
        }
        return match temporal_consequent_signal(rule) {
            Some(signal) if declared_signals.contains(&signal) => {
                // FSMGEN-ASSERT-LOWERING.3 (parity with classify_actor_contract):
                // guarded → `(=> g (within s [min] max))`; unguarded → monitor.
                let guard = match temporal_antecedent_condition(rule, declared_signals, polarities)
                {
                    Ok(guard) => guard,
                    Err(reason) => {
                        return TemporalRuleDisposition::Residual {
                            rule_id: rule.rule_id.clone(),
                            reason,
                        };
                    }
                };
                match windowed_eventual_prop(&signal, window.min_cycles, u64::from(within), &guard)
                {
                    Ok(prop) => TemporalRuleDisposition::Contract {
                        name: sanitize_isf_name(&rule.rule_id),
                        prop,
                    },
                    Err(reason) => TemporalRuleDisposition::Residual {
                        rule_id: rule.rule_id.clone(),
                        reason,
                    },
                }
            }
            Some(signal) => TemporalRuleDisposition::Residual {
                rule_id: rule.rule_id.clone(),
                reason: format!(
                    "windowed temporal rule targets signal '{}' which is not \
                     in the emitted `.isf` interface",
                    signal
                ),
            },
            None => TemporalRuleDisposition::Residual {
                rule_id: rule.rule_id.clone(),
                reason: "windowed temporal rule has no single-signal \
                         consequent (e.g. HandshakeComplete); FSMGen strict \
                         rejects the `(stage …)` ready/valid form"
                    .to_string(),
            },
        };
    }

    // (2) Non-windowed value/guard→drive (mapping #3): a `SignalValue`
    //     consequent naming a declared signal with an ISF-literal value.
    if let Some((signal, value)) = temporal_drive_consequent(rule) {
        if !declared_signals.contains(&signal) {
            return TemporalRuleDisposition::Residual {
                rule_id: rule.rule_id.clone(),
                reason: format!(
                    "temporal rule drives signal '{}' which is not in the \
                     emitted `.isf` interface",
                    signal
                ),
            };
        }
        let resolved = drive_level(&signal, &value, polarities).and_then(|val| {
            temporal_antecedent_condition(rule, declared_signals, polarities)
                .map(|condition| (val, condition))
        });
        return match resolved {
            Ok((val, condition)) => TemporalRuleDisposition::Rule {
                name: format!("temporal_{}", sanitize_isf_name(&rule.rule_id)),
                condition,
                signal,
                value: val,
            },
            Err(reason) => TemporalRuleDisposition::Residual {
                rule_id: rule.rule_id.clone(),
                reason,
            },
        };
    }

    // (3) Everything else (HandshakeComplete-only; stability/sample/actor-
    //     drive consequents that name a signal but carry no concrete value):
    //     no representable supported ISF construct → residual (mapping #4).
    TemporalRuleDisposition::Residual {
        rule_id: rule.rule_id.clone(),
        reason: "temporal rule has no representable supported ISF construct \
                 (no positive bounded window and no concrete signal value to \
                 drive); preserved as a residual decision rather than \
                 fabricating unsupported syntax"
            .to_string(),
    }
}

/// `R16-CONTRACT-IR.3` parity re-point: classify an `ActorContract` into
/// the exact same `TemporalRuleDisposition` that `classify_temporal_rule`
/// produces for its originating rule. The `contract_from_temporal_rule`
/// conversion is window-first and structurally parallel to
/// `classify_temporal_rule`, so the obligation, `guard_candidates`,
/// `source_rule_id` and `declared_signals` together fully determine the
/// FSMGen-facing decision. Per the recorded `.3` parity definition the
/// emitted `.isf` and the Contract/Rule/residual `rule_id` sets are
/// identical; residual reason wording is internal `adapter.json`
/// metadata taken from the contract's recorded lowering reason.
pub(crate) fn classify_actor_contract(
    contract: &crate::ir::contract::ActorContract,
    declared_signals: &BTreeSet<String>,
    polarities: &BTreeMap<String, SignalPolarity>,
) -> TemporalRuleDisposition {
    use crate::ir::contract::{EventExpr, LoweringDisposition, Obligation, Window};

    let rule_id = contract.source_rule_id.clone().unwrap_or_else(|| {
        contract
            .contract_id
            .strip_prefix("contract_")
            .unwrap_or(&contract.contract_id)
            .to_string()
    });

    // EXTRACTION-REPRESENTATION-GAPS.30 — `merge_cluster` marks a fusion whose members disagree residual (AXI `AWBURST`
    // *must be INCR* and *must be WRAP*, both unconditioned). Lowered through its obligation arm it became its first
    // member's drive: a winner the document never names. Only a fusion is refused here, because the converter marks a
    // handshake barrier residual too and it is lowered as a stage on purpose (R16-CONTRACT-IR.4).
    if contract.contract_id.starts_with("fused:")
        && let LoweringDisposition::Residual { reason } = &contract.lowering
    {
        return TemporalRuleDisposition::Residual {
            rule_id,
            reason: format!(
                "the fused sentences disagree ({reason}); lowering one of them would pick a winner the document \
                 does not name"
            ),
        };
    }

    match &contract.obligation {
        // Windowed bounded_eventually candidate — produced ONLY for a
        // windowed rule with a single-signal consequent and a positive
        // bound (mirrors `classify_temporal_rule` branch 1, Contract arm).
        Obligation::Eventually {
            target: EventExpr::Level { signal, value },
            window: Window::Within { min, max },
        } => {
            if !declared_signals.contains(signal) {
                TemporalRuleDisposition::Residual {
                    rule_id,
                    reason: format!(
                        "windowed temporal rule targets signal '{}' which is not \
                         in the emitted `.isf` interface",
                        signal
                    ),
                }
            } else if let Some(reason) = windowed_event_refusal(signal, value, polarities) {
                TemporalRuleDisposition::Residual { rule_id, reason }
            } else {
                // FSMGEN-ASSERT-LOWERING.3: a guarded windowed-eventual keeps its
                // antecedent (the monitor dropped it) — `(=> g (within s [min] max))`;
                // an unguarded one has no event to count from (EXTRACTION-REPRESENTATION-GAPS.29).
                let guard = match actor_guard_condition(
                    &contract.guard_candidates,
                    declared_signals,
                    polarities,
                ) {
                    Ok(guard) => guard,
                    Err(reason) => return TemporalRuleDisposition::Residual { rule_id, reason },
                };
                match windowed_eventual_prop(signal, *min, u64::from(*max), &guard) {
                    Ok(prop) => TemporalRuleDisposition::Contract {
                        name: sanitize_isf_name(&rule_id),
                        prop,
                    },
                    Err(reason) => TemporalRuleDisposition::Residual { rule_id, reason },
                }
            }
        }

        // Non-windowed value drive — produced ONLY for a non-windowed
        // `SignalValue` consequent (mirrors branch 2, Rule arm + its
        // declared / ISF-literal gates).
        Obligation::Drive { signal, value } => {
            if !declared_signals.contains(signal) {
                TemporalRuleDisposition::Residual {
                    rule_id,
                    reason: format!(
                        "temporal rule drives signal '{}' which is not in the \
                         emitted `.isf` interface",
                        signal
                    ),
                }
            } else {
                // Reproduce `classify_temporal_rule` EXACTLY: the stated drive level, then the guard
                // `guard_from_candidates` derives (EXTRACTION-REPRESENTATION-GAPS.8 for both).
                let resolved = drive_level(signal, value, polarities).and_then(|val| {
                    actor_guard_condition(&contract.guard_candidates, declared_signals, polarities)
                        .map(|condition| (val, condition))
                });
                match resolved {
                    Ok((val, condition)) => TemporalRuleDisposition::Rule {
                        name: format!("temporal_{}", sanitize_isf_name(&rule_id)),
                        condition,
                        signal: signal.clone(),
                        value: val,
                    },
                    Err(reason) => TemporalRuleDisposition::Residual { rule_id, reason },
                }
            }
        }

        // R16-CONTRACT-IR.4 — deliberate post-parity behaviour change:
        // a ready/valid handshake barrier lowers to a `(stage …)`
        // transaction (FSMGen `ready_valid_barrier`, accepted at pin
        // `9bfb9a20`) instead of residual, when both signals are in the
        // emitted interface. Undeclared signals → residual (never
        // fabricate a reference to an undeclared signal).
        Obligation::HandshakeBarrier { valid, ready } => {
            if declared_signals.contains(valid) && declared_signals.contains(ready) {
                TemporalRuleDisposition::Stage {
                    name: sanitize_isf_name(&rule_id),
                    ready: ready.clone(),
                    valid: valid.clone(),
                }
            } else {
                TemporalRuleDisposition::Residual {
                    rule_id,
                    reason: format!(
                        "handshake barrier references signal(s) not in the \
                         emitted `.isf` interface (valid '{}', ready '{}')",
                        valid, ready
                    ),
                }
            }
        }

        // Everything else is residual (same residual SET as
        // `classify_temporal_rule`); the reason is the contract's
        // recorded lowering reason (internal `adapter.json` metadata per
        // the `.3` parity definition).
        _ => {
            let reason = match &contract.lowering {
                LoweringDisposition::Residual { reason } => reason.clone(),
                LoweringDisposition::Lowerable => {
                    "temporal rule has no representable supported ISF construct \
                     (no positive bounded window and no concrete signal value to \
                     drive); preserved as a residual decision rather than \
                     fabricating unsupported syntax"
                        .to_string()
                }
            };
            TemporalRuleDisposition::Residual { rule_id, reason }
        }
    }
}

/// Build the explicit residual-decision packet for a temporal rule that has
/// no representable supported ISF construct (mapping #4).
/// Remove rules that conflict on the same signal+guard (FSMGen strict rejects
/// conflicting drives). The FIRST rule for a given (signal, guard) is kept and
/// emitted; every later rule that drives the same signal to a *different* value
/// under the same guard is dropped from the `.isf` AND recorded as an explicit
/// `ResidualDecisionPacket` — so the conflict is surfaced, never silently lost
/// (ISF-RULE-CONFLICT-RESIDUAL). Emitted `.isf` is unchanged vs the prior
/// silent-drop behavior; only the residual record is new.
fn dedup_conflicting_rules(rules: Vec<IsfRule>) -> (Vec<IsfRule>, Vec<ResidualDecisionPacket>) {
    let mut seen: std::collections::BTreeMap<(String, String), String> =
        std::collections::BTreeMap::new();
    let mut deduped: Vec<IsfRule> = Vec::new();
    let mut residuals: Vec<ResidualDecisionPacket> = Vec::new();
    'outer: for rule in rules {
        for (sig, val) in &rule.drives {
            let key = (sig.clone(), rule.condition.clone());
            let Some(prev_val) = seen.get(&key) else {
                continue;
            };
            match drive_agreement(prev_val, val) {
                DriveAgreement::Same => {}
                DriveAgreement::SameLevelOtherSpelling => {
                    residuals.push(same_level_duplicate_residual_packet(
                        &rule.name, sig, val, prev_val,
                    ));
                    continue 'outer;
                }
                DriveAgreement::Different => {
                    residuals.push(rule_conflict_residual_packet(
                        &rule.name,
                        sig,
                        &rule.condition,
                        val,
                        prev_val,
                    ));
                    continue 'outer;
                }
            }
        }
        for (sig, val) in &rule.drives {
            let key = (sig.clone(), rule.condition.clone());
            seen.entry(key).or_insert_with(|| val.clone());
        }
        deduped.push(rule);
    }
    (deduped, residuals)
}

/// EXTRACTION-REPRESENTATION-GAPS.10.f — how a second drive of a signal relates to the one already kept.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DriveAgreement {
    /// The same value, written the same way: both rules are emitted, as FSMGen accepts.
    Same,
    /// The same number written differently (AXI `ARLOCK`: *Must be 0b0* and *tied LOW*, `0B0` and `0`). Not a
    /// conflict, but FSMGen compares the writes as written and refuses the pair, so the second is not emitted.
    SameLevelOtherSpelling,
    /// A different value: a conflict.
    Different,
}

/// A value that is no number (an enum member, a sized cast) is compared as written.
fn drive_agreement(kept: &str, new: &str) -> DriveAgreement {
    if kept == new {
        return DriveAgreement::Same;
    }
    match (parse_reset_literal(kept), parse_reset_literal(new)) {
        (Some(kept), Some(new)) if kept == new => DriveAgreement::SameLevelOtherSpelling,
        _ => DriveAgreement::Different,
    }
}

/// EXTRACTION-REPRESENTATION-GAPS.10.f — a rule not emitted because an emitted rule already drives the same level.
fn same_level_duplicate_residual_packet(
    rule_name: &str,
    signal: &str,
    value: &str,
    kept_value: &str,
) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: format!("isf_rule_same_level_{}", sanitize_isf_name(rule_name)),
        question: format!("Rule `{rule_name}` restates a drive of `{signal}`: keep one spelling?"),
        why_unresolved: format!(
            "Rule `{rule_name}` drives `{signal}` to `{value}`, the level an emitted rule already drives as \
             `{kept_value}`. The two agree, so nothing is lost; FSMGen compares rule writes as written and \
             refuses two spellings of one value, so this rule is not emitted twice."
        ),
        automation_confidence: AutomationConfidence::High,
        candidate_interpretations: vec![CandidateInterpretation {
            interpretation_id: "one_spelling".to_string(),
            description: "Emit the level once, in the spelling of the first rule.".to_string(),
            downstream_impact: "None: the obligation is emitted.".to_string(),
        }],
    }
}

/// Residual for a value-conflicting rule dropped at dedup (kept honest instead
/// of silently discarded). Mirrors `temporal_residual_packet`'s shape.
fn rule_conflict_residual_packet(
    rule_name: &str,
    signal: &str,
    condition: &str,
    dropped_value: &str,
    kept_value: &str,
) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: format!("isf_rule_conflict_{}", sanitize_isf_name(rule_name)),
        question: format!(
            "Conflicting drive for `{signal}` under guard `{condition}`: which value is correct?"
        ),
        why_unresolved: format!(
            "Rule `{rule_name}` drives `{signal}` to `{dropped_value}` under guard `{condition}`, \
             but an earlier rule already drives it to `{kept_value}` under the same guard. FSMGen \
             strict rejects conflicting drives, so this rule was DROPPED from the emitted `.isf` \
             (the earlier `{kept_value}` is kept) rather than fabricating an invalid contradiction. \
             Recorded here so the conflict is explicit, not silently lost."
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "keep_earlier_rule".to_string(),
                description: format!(
                    "Keep `{signal}` = `{kept_value}` (the earlier rule, currently emitted)."
                ),
                downstream_impact:
                    "Emitted `.isf` drives this value; the conflicting rule is omitted.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "keep_conflicting_rule".to_string(),
                description: format!(
                    "Keep `{signal}` = `{dropped_value}` (rule `{rule_name}`, currently dropped)."
                ),
                downstream_impact:
                    "Would require dropping the earlier rule instead — a human must decide."
                        .to_string(),
            },
        ],
    }
}

/// KG-ISF-COMPLETENESS.2a.v ("Lever C"): drop rules that conflict with an UNCONDITIONAL driver on the
/// same signal. An unconditional rule (`condition == ""`) drives its target on every cycle, so its firing
/// set ⊇ every guard; FSMGen's `_condition_terms_prove_disjoint` can never prove an absent condition
/// disjoint, so its `isf_conflicting_rule_writes` check rejects any other rule driving the same target to a
/// different value. The same-guard `dedup_conflicting_rules` keys on
/// `(signal, guard)` and so MISSES this cross-guard overlap. This runs AFTER it, so there is at most one
/// unconditional value per signal (two unconditional rules on one signal share key `(S, "")` and the second
/// is already gone). For each signal `S` with a kept unconditional driver value `V`, drop every other rule
/// driving `S` to a value `≠ V`, recording an honest residual (never a fabricated precedence — the
/// `(priority …)` hatch would require asserting an ungrounded winner over EVERY same-value unconditional
/// rule). It is precise: it drops exactly FSMGen's flagged overlap, so a strict-clean doc — which by
/// construction cannot contain such a config — re-emits byte-identical. Order is preserved.
fn drop_unconditional_overlap_conflicts(
    rules: Vec<IsfRule>,
) -> (Vec<IsfRule>, Vec<ResidualDecisionPacket>) {
    // The (unique, post-same-guard-dedup) unconditional driver value per signal.
    let mut unconditional_value: std::collections::BTreeMap<String, String> =
        std::collections::BTreeMap::new();
    for rule in &rules {
        if rule.condition.is_empty() {
            for (sig, val) in &rule.drives {
                unconditional_value
                    .entry(sig.clone())
                    .or_insert_with(|| val.clone());
            }
        }
    }
    let mut kept: Vec<IsfRule> = Vec::new();
    let mut residuals: Vec<ResidualDecisionPacket> = Vec::new();
    'outer: for rule in rules {
        for (sig, val) in &rule.drives {
            let Some(uncond) = unconditional_value.get(sig) else {
                continue;
            };
            match drive_agreement(uncond, val) {
                DriveAgreement::Same => {}
                DriveAgreement::SameLevelOtherSpelling => {
                    residuals.push(same_level_duplicate_residual_packet(
                        &rule.name, sig, val, uncond,
                    ));
                    continue 'outer;
                }
                DriveAgreement::Different => {
                    residuals.push(unconditional_overlap_residual_packet(
                        &rule.name,
                        sig,
                        &rule.condition,
                        val,
                        uncond,
                    ));
                    continue 'outer;
                }
            }
        }
        kept.push(rule);
    }
    (kept, residuals)
}

/// Residual for a rule dropped because it conflicts with an UNCONDITIONAL driver on the same signal
/// (KG-ISF-COMPLETENESS.2a.v). FSMGen rejects the overlap; the dropped obligation is recorded, not lost.
fn unconditional_overlap_residual_packet(
    rule_name: &str,
    signal: &str,
    condition: &str,
    dropped_value: &str,
    unconditional_value: &str,
) -> ResidualDecisionPacket {
    let guard = if condition.is_empty() {
        "(always)"
    } else {
        condition
    };
    ResidualDecisionPacket {
        packet_id: format!(
            "isf_unconditional_overlap_{}",
            sanitize_isf_name(rule_name)
        ),
        question: format!(
            "Conflicting drive for `{signal}`: an unconditional rule already drives it to \
             `{unconditional_value}`; what governs `{dropped_value}`?"
        ),
        why_unresolved: format!(
            "Rule `{rule_name}` drives `{signal}` to `{dropped_value}` under guard `{guard}`, but an \
             unconditional rule already drives `{signal}` to `{unconditional_value}` on every cycle. An \
             unconditional rule overlaps every guard, so FSMGen strict rejects the differing-value overlap \
             (`isf_conflicting_rule_writes`); this rule was DROPPED from the emitted `.isf` (the \
             unconditional `{unconditional_value}` is kept) rather than fabricating a precedence the \
             document does not ground. Recorded here so the conflict is explicit, not silently lost."
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "keep_unconditional_rule".to_string(),
                description: format!(
                    "Keep `{signal}` = `{unconditional_value}` (the unconditional rule, currently emitted)."
                ),
                downstream_impact:
                    "Emitted `.isf` drives the unconditional value; the guarded rule is omitted."
                        .to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "narrow_unconditional_rule".to_string(),
                description: format!(
                    "Treat rule `{rule_name}` (`{signal}` = `{dropped_value}` under `{guard}`) as an \
                     exception to the unconditional default — would require narrowing the unconditional \
                     rule's guard; a human must decide."
                ),
                downstream_impact:
                    "Would keep both obligations with an explicit precedence the document does not state."
                        .to_string(),
            },
        ],
    }
}

/// Retire rule/transaction precedence that the source never states
/// (`FSMGEN-REFRESH-INTEGRATE-6.1`). A transaction step calls a named drive; when exactly one
/// distinct local transaction calls that drive, FSMGen can prove that transaction owns the drive
/// and rejects a rule writing the same target unless the actor declares a priority. IntentIR has no
/// such priority carrier, so emitting one would invent behavior. Keep the richer transaction and
/// drop each overlapping rule as an explicit residual. Multiple-caller drives are intentionally
/// left alone: FSMGen cannot assign them a unique transaction owner, and the current no-priority
/// contract accepts that shape without diagnostics.
fn drop_ungrounded_rule_transaction_conflicts(
    rules: Vec<IsfRule>,
    transactions: &[IsfTransaction],
    drives: &[IsfNamedDrive],
) -> (Vec<IsfRule>, Vec<ResidualDecisionPacket>) {
    let drive_targets: BTreeMap<String, BTreeSet<String>> = drives
        .iter()
        .map(|drive| {
            (
                drive.name.clone(),
                drive
                    .body
                    .iter()
                    .map(|(target, _)| target.clone())
                    .collect(),
            )
        })
        .collect();

    let mut callers_by_target: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for transaction in transactions {
        let mut called_drives = BTreeSet::new();
        collect_called_drives(&transaction.on_steps, &mut called_drives);
        collect_called_drives(&transaction.steps, &mut called_drives);
        for drive_name in called_drives {
            let Some(targets) = drive_targets.get(&drive_name) else {
                continue;
            };
            for target in targets {
                callers_by_target
                    .entry(target.clone())
                    .or_default()
                    .insert(transaction.name.clone());
            }
        }
    }

    let uniquely_owned_targets: BTreeMap<String, String> = callers_by_target
        .into_iter()
        .filter_map(|(target, callers)| {
            if callers.len() == 1 {
                callers.into_iter().next().map(|caller| (target, caller))
            } else {
                None
            }
        })
        .collect();

    let mut kept = Vec::new();
    let mut residuals = Vec::new();
    'rule: for rule in rules {
        for (target, value) in &rule.drives {
            if let Some(transaction) = uniquely_owned_targets.get(target) {
                residuals.push(rule_transaction_conflict_residual_packet(
                    &rule.name,
                    target,
                    value,
                    transaction,
                ));
                continue 'rule;
            }
        }
        kept.push(rule);
    }
    (kept, residuals)
}

/// Recursively collect named-drive calls from every control-flow shape inside a transaction.
fn collect_called_drives(steps: &[IsfTxnStep], out: &mut BTreeSet<String>) {
    for step in steps {
        match step {
            IsfTxnStep::Drive { name, .. } => {
                out.insert(name.clone());
            }
            IsfTxnStep::When { body, .. }
            | IsfTxnStep::While { body, .. }
            | IsfTxnStep::Until { body, .. }
            | IsfTxnStep::Repeat { body, .. } => collect_called_drives(body, out),
            IsfTxnStep::Switch { branches, .. } => {
                for (_, body) in branches {
                    collect_called_drives(body, out);
                }
            }
            IsfTxnStep::Await { .. }
            | IsfTxnStep::Wait { .. }
            | IsfTxnStep::Sample { .. }
            | IsfTxnStep::Do { .. }
            | IsfTxnStep::Spawn { .. }
            | IsfTxnStep::Set { .. }
            | IsfTxnStep::Update { .. }
            | IsfTxnStep::ShiftLeft { .. }
            | IsfTxnStep::ShiftRight { .. }
            | IsfTxnStep::Complete { .. }
            | IsfTxnStep::AwaitAll { .. }
            | IsfTxnStep::AwaitAny { .. }
            | IsfTxnStep::Latency { .. } => {}
        }
    }
}

fn rule_transaction_conflict_residual_packet(
    rule_name: &str,
    target: &str,
    value: &str,
    transaction: &str,
) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: format!(
            "isf_rule_transaction_conflict_{}",
            sanitize_isf_name(rule_name)
        ),
        question: format!(
            "Rule `{rule_name}` and transaction `{transaction}` can both write `{target}`; which one wins?"
        ),
        why_unresolved: format!(
            "Rule `{rule_name}` drives `{target}` to `{value}`, while transaction `{transaction}` is the \
             single local caller of a named drive that writes the same target. FSMGen requires an explicit \
             actor-level priority for this overlap, but IntentIR carries no source-grounded precedence \
             relation. The rule was DROPPED from emitted `.isf` and preserved here instead of fabricating \
             a winner; the source-grounded transaction remains executable."
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "keep_transaction".to_string(),
                description: format!(
                    "Keep transaction `{transaction}` as the executable writer and preserve rule \
                     `{rule_name}` as a residual (current behavior)."
                ),
                downstream_impact: format!(
                    "The emitted `.isf` keeps `{transaction}` and does not assert `{target}` = `{value}` \
                     through rule `{rule_name}`."
                ),
            },
            CandidateInterpretation {
                interpretation_id: "establish_source_priority".to_string(),
                description: format!(
                    "Establish from authoritative source evidence whether `{rule_name}` or \
                     `{transaction}` has precedence, then represent that relation in canonical IR."
                ),
                downstream_impact:
                    "Would permit an actor-level priority only after the canonical model carries the \
                     evidence-backed winner; no adapter-only guess is allowed."
                        .to_string(),
            },
        ],
    }
}

/// KG-ISF-COMPLETENESS.2a.vi: drop a rule whose any drive VALUE is not a renderable ISF value. FSMGen
/// requires a rule assignment action's RHS to be a value expression (`(port expr)`); a constraint whose
/// extracted value is free PROSE (for example, a loopback described as `the value presented on the
/// peer signal`) would emit a multi-word value FSMGen rejects, breaking the whole `.isf`. The gate
/// reuses `is_safe_isf_scalar_value` (non-empty, whitespace-free) — every legitimate rule drive (a scalar
/// literal `0`/`1`/`0b01`, a width-cast `7'd125`, an enum symbol `VALID`) passes, so this is byte-identical
/// on every doc without a prose-valued rule (measured: only `ihi0022_h_c` carries any). The dropped
/// obligation is recorded as an `isf_rule_value_<name>` residual — the prose value is unrecoverable (a
/// temporal loopback, not the current `(port ARLOOP)`), so it is never fabricated into a `(port expr)`.
fn drop_unrenderable_rule_values(
    rules: Vec<IsfRule>,
) -> (Vec<IsfRule>, Vec<ResidualDecisionPacket>) {
    let mut kept: Vec<IsfRule> = Vec::new();
    let mut residuals: Vec<ResidualDecisionPacket> = Vec::new();
    for rule in rules {
        if let Some((sig, val)) = rule
            .drives
            .iter()
            .find(|(_, v)| !is_safe_isf_scalar_value(v))
        {
            residuals.push(unrenderable_rule_value_residual_packet(
                &rule.name, sig, val,
            ));
            continue;
        }
        kept.push(rule);
    }
    (kept, residuals)
}

/// Residual for a rule dropped because a drive value is not a renderable ISF value expression
/// (KG-ISF-COMPLETENESS.2a.vi). FSMGen rejects the non-`(port expr)` RHS; the dropped obligation is
/// recorded, not lost, and the prose value is preserved verbatim for a human to interpret.
fn unrenderable_rule_value_residual_packet(
    rule_name: &str,
    signal: &str,
    value: &str,
) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: format!("isf_rule_value_{}", sanitize_isf_name(rule_name)),
        question: format!(
            "Rule `{rule_name}` drives `{signal}` to a non-renderable value `{value}`: what value \
             expression does the document mean?"
        ),
        why_unresolved: format!(
            "Rule `{rule_name}` drives `{signal}` to `{value}`, which is not a renderable ISF value \
             expression (FSMGen requires a rule assignment action RHS to be a `(port expr)` — a literal, \
             port reference, or expression, not free prose). The value was captured as descriptive text \
             (e.g. a temporal loopback such as \"the value that was presented on another signal\") whose \
             exact expression the emitter cannot recover without fabricating, so the rule was DROPPED from \
             the emitted `.isf` rather than emitting a value FSMGen rejects. Recorded here so the \
             obligation is explicit, not silently lost."
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![CandidateInterpretation {
            interpretation_id: "interpret_value_expression".to_string(),
            description: format!(
                "Translate the prose value `{value}` for `{signal}` into a concrete ISF value expression \
                 (a literal or `(port …)` reference) — a human must decide the intended expression."
            ),
            downstream_impact:
                "Would let the rule emit; until then `{signal}` carries no rule-driven value from this \
                 obligation."
                    .to_string(),
        }],
    }
}

/// ISF-VALUE-WIDTH-EMIT.2: outcome of reconciling a rule drive's value literal against the target
/// signal's emitted width.
enum ValueAlign {
    /// Leave the literal byte-identical — a bare/unsized decimal, an enum symbol/reference, or a
    /// based literal whose notation width already equals the signal width.
    Keep,
    /// Re-render as an explicit width-aligned cast (`W'd<v>`): the value fits the signal width but
    /// its current notation width does not.
    Replace(String),
    /// The value does not fit the signal width; FSMGen blocks implicit truncation, so the clause is
    /// dropped with an honest residual rather than fabricating a truncated value.
    Residualize,
}

/// Parse an ISF value literal into `(value, notation_width_bits)` IFF it is a width-bearing numeric
/// based literal — `0b…` (notation width = binary-digit count) or `0x…` (notation width = hex-digit
/// count × 4, the count FSMGen uses for its operand-width contract). Returns `None` for a bare/unsized
/// decimal, an enum symbol, a reference, an expression, or an already-cast `W'…` literal — all left
/// untouched (a bare decimal is unsized in FSMGen and fits any width; a symbol is not a literal).
fn parse_sized_literal(s: &str) -> Option<(u128, u32)> {
    let t = s.trim();
    if let Some(rest) = t.strip_prefix("0b").or_else(|| t.strip_prefix("0B")) {
        if !rest.is_empty() && rest.bytes().all(|b| b == b'0' || b == b'1') {
            return u128::from_str_radix(rest, 2)
                .ok()
                .map(|v| (v, rest.len() as u32));
        }
        return None;
    }
    if let Some(rest) = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")) {
        if !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_hexdigit()) {
            return u128::from_str_radix(rest, 16)
                .ok()
                .map(|v| (v, rest.len() as u32 * 4));
        }
        return None;
    }
    None
}

/// ISF-VALUE-WIDTH-EMIT.2: decide how a single drive value literal reconciles with `width`. A based
/// literal whose notation width already matches stays byte-identical; one that overshoots but whose
/// VALUE fits is re-rendered as a width-aligned decimal cast; one whose value cannot fit is
/// residualized (never truncated). Bare decimals / symbols are kept (FSMGen sizes them in context).
fn align_value_to_width(val: &str, width: u32) -> ValueAlign {
    match parse_sized_literal(val) {
        None => ValueAlign::Keep,
        Some((v, notation_width)) => {
            if notation_width == width {
                ValueAlign::Keep
            } else if width >= 128 || v < (1u128 << width) {
                ValueAlign::Replace(format!("{width}'d{v}"))
            } else {
                ValueAlign::Residualize
            }
        }
    }
}

/// ISF-VALUE-WIDTH-EMIT.2: width-align every rule's drive value literals to their target signal's
/// emitted width. A drive value that cannot be aligned (its value exceeds the signal width) drops the
/// whole rule and yields an honest residual; an over-wide-but-fitting literal is re-rendered in place;
/// everything else (bare decimals, symbols, already-matching literals, signals of unknown width) is
/// left byte-identical. Order is preserved (the downstream dedup keeps the first rule).
fn align_rule_drive_widths(
    rules: Vec<IsfRule>,
    signal_widths: &BTreeMap<String, u32>,
) -> (Vec<IsfRule>, Vec<ResidualDecisionPacket>) {
    let mut out: Vec<IsfRule> = Vec::new();
    let mut residuals: Vec<ResidualDecisionPacket> = Vec::new();
    'rule: for mut rule in rules {
        for (sig, val) in &rule.drives {
            if let Some(&w) = signal_widths.get(sig)
                && matches!(align_value_to_width(val, w), ValueAlign::Residualize)
            {
                residuals.push(value_width_residual_packet(&rule.name, sig, val, w));
                continue 'rule;
            }
        }
        for (sig, val) in rule.drives.iter_mut() {
            if let Some(&w) = signal_widths.get(sig)
                && let ValueAlign::Replace(new_val) = align_value_to_width(val, w)
            {
                *val = new_val;
            }
        }
        out.push(rule);
    }
    (out, residuals)
}

/// Residual for a rule dropped because a drive value cannot fit its target signal width
/// (ISF-VALUE-WIDTH-EMIT.2). Mirrors `rule_conflict_residual_packet` — the dropped obligation is
/// surfaced, never silently lost or fabricated by truncation.
fn value_width_residual_packet(
    rule_name: &str,
    signal: &str,
    value: &str,
    width: u32,
) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: format!("isf_value_width_{}", sanitize_isf_name(rule_name)),
        question: format!(
            "Value `{value}` does not fit signal `{signal}` (width {width}): how should it lower?"
        ),
        why_unresolved: format!(
            "Rule `{rule_name}` drives `{signal}` (width {width}) to `{value}`, whose value exceeds \
             2^{width}. FSMGen's operand contract blocks implicit truncation, so this rule was DROPPED \
             from the emitted `.isf` rather than truncating the value or fabricating a width-aligned \
             one. Recorded here so the dropped obligation is explicit, not silently lost."
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "widen_signal".to_string(),
                description: format!(
                    "Declare `{signal}` wide enough to hold `{value}` (only if the document grounds a wider width)."
                ),
                downstream_impact:
                    "The value would then lower as a width-aligned literal — needs current-document width evidence."
                        .to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "correct_value".to_string(),
                description: format!(
                    "Treat `{value}` as mis-attributed / over-wide for `{signal}` and resolve it upstream."
                ),
                downstream_impact:
                    "The clause stays out of the `.isf` until the value or its target signal is corrected."
                        .to_string(),
            },
        ],
    }
}

/// Residual for an enum HELD OUT of the `.isf` by the value-literal gate (KG-ISF-COMPLETENESS.2a.iv):
/// every member is a safe scalar but ≥1 value is a bare binary-looking token (only `0`/`1` digits,
/// length >= 4), which FSMGen's package-symbol parser rejects un-qualified (it must be
/// width/radix-qualified). The emitter cannot qualify it without fabricating a radix (the value is a
/// binary code mis-read as a decimal in a mega-conflated enum), so the whole enum is dropped (honest
/// residual over fabrication) and surfaced here.
fn enum_value_literal_residual_packet(e: &IsfEnum) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: format!("isf_enum_value_literal_{}", sanitize_isf_name(&e.type_name)),
        question: format!(
            "Enum `{}` has a member value FSMGen cannot accept un-qualified: how should it lower?",
            e.type_name
        ),
        why_unresolved: format!(
            "Enum `{}` carries {} member(s) but at least one value is a bare binary-looking token \
             (only 0/1 digits, length >= 4 — e.g. `1000`/`1111`), which FSMGen's package-symbol parser \
             rejects un-qualified (it must be width/radix-qualified such as `4'b1000`). Such a value is \
             a binary code mis-read as a decimal in a mega-conflated enum, so the emitter cannot \
             qualify it without fabricating a radix; the whole enum was DROPPED from the emitted `.isf` \
             rather than emitting an invalid literal. Recorded here so the dropped surface is explicit, not silently lost.",
            e.type_name,
            e.members.len()
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "split_conflated_enum".to_string(),
                description:
                    "Split the conflated enum upstream so each source table is its own enum with values fitting its true width."
                        .to_string(),
                downstream_impact:
                    "Each well-formed enum would then lower — needs extraction-side enum/table separation."
                        .to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "preserve_radix".to_string(),
                description:
                    "Preserve the document's value radix (e.g. binary codes) at extraction so members lower as width-qualified literals."
                        .to_string(),
                downstream_impact:
                    "The members would lower as `W'bXXXX` literals — needs the radix captured upstream, never guessed in the emitter."
                        .to_string(),
            },
        ],
    }
}

fn temporal_residual_packet(
    rule_id: &str,
    reason: &str,
    source_text: &str,
) -> ResidualDecisionPacket {
    ResidualDecisionPacket {
        packet_id: format!(
            "isf_temporal_unrepresentable_{}",
            sanitize_isf_name(rule_id)
        ),
        question: format!(
            "How should temporal rule '{}' be represented downstream of `.isf`?",
            rule_id
        ),
        why_unresolved: format!(
            "{}. Source: {}",
            reason,
            if source_text.is_empty() {
                "(no source text)"
            } else {
                source_text
            }
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "preserve_as_residual".to_string(),
                description: "Keep the temporal obligation as a residual \
                              decision; do not emit any `.isf` construct for \
                              it (current behavior — honest and lossless to \
                              the IntentIR)."
                    .to_string(),
                downstream_impact: "FSMGen never sees this obligation; a human \
                                    or a future supported ISF construct must \
                                    carry it."
                    .to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "future_isf_construct".to_string(),
                description: "Lower it once FSMGen ships a supported ISF \
                              construct for this temporal shape (e.g. a \
                              non-rejected stage/stability form)."
                    .to_string(),
                downstream_impact: "Requires a confirmed FSMGen-strict-valid \
                                    construct; tracked via \
                                    docs/FSMGEN_FEEDBACK.md."
                    .to_string(),
            },
        ],
    }
}

fn branch_predicate_guard(branch: &ControlBranchRecord, selector_text: &str) -> String {
    match &branch.predicate {
        Some(pred) => render_isf_control_expression(pred),
        None => selector_text.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paren_balance(s: &str) -> i64 {
        let mut depth: i64 = 0;
        for c in s.chars() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ => {}
            }
            assert!(depth >= 0, "closing paren before matching open in:\n{s}");
        }
        depth
    }

    fn minimal_isf() -> IsfIr {
        IsfIr {
            actor_name: "a".to_string(),
            clock: "clk".to_string(),
            reset: IsfReset {
                signal: "rst_n".to_string(),
                timing: "async".to_string(),
                polarity: "active_low".to_string(),
            },
            watchdog: 16,
            signals: BTreeSet::new(),
            width_params: BTreeMap::new(),
            constants: vec![],
            types: vec![],
            enums: vec![],
            storage: vec![],
            drives: vec![],
            transactions: vec![],
            rules: vec![],
            temporal_residuals: vec![],
            storage_reset_residuals: vec![],
            storage_field_residuals: vec![],
            signal_width_residuals: vec![],
            interface_residuals: vec![],
        }
    }

    // --- pure helpers -----------------------------------------------------

    #[test]
    fn binary_operator_rendering_is_exhaustive_and_exact() {
        use ControlBinaryOperator::*;
        let cases = [
            (Eq, "=="),
            (NotEq, "!="),
            (Lt, "<"),
            (Le, "<="),
            (Gt, ">"),
            (Ge, ">="),
            (BitAnd, "&"),
            (BitOr, "|"),
            (BitXor, "^"),
            (Add, "+"),
            (Sub, "-"),
            (Mul, "*"),
            (Div, "/"),
            (Mod, "%"),
        ];
        for (op, expected) in cases {
            assert_eq!(render_isf_binary_operator(op), expected);
        }
    }

    #[test]
    fn width_hint_rendering_distinguishes_numeric_and_parametric() {
        assert_eq!(render_isf_width_hint(&WidthHint::Numeric(8)), "8");
        assert_eq!(render_isf_width_hint(&WidthHint::Numeric(1)), "1");
        assert_eq!(
            render_isf_width_hint(&WidthHint::Parametric("DATA_W".to_string())),
            "DATA_W"
        );
    }

    #[test]
    fn sanitize_isf_name_replaces_specials_collapses_and_guards_leading_digit() {
        assert_eq!(sanitize_isf_name("AW VALID"), "aw_valid");
        assert_eq!(sanitize_isf_name("a--b..c"), "a_b_c");
        assert_eq!(sanitize_isf_name("__lead_trail__"), "lead_trail");
        assert_eq!(sanitize_isf_name("***"), "unnamed");
        assert_eq!(sanitize_isf_name("3state"), "reg_3state");
        assert_eq!(sanitize_isf_name("Mixed/Case#1"), "mixed_case_1");
        // KG-ISF-COMPLETENESS.2a.iii: the allowlist maps ANY non-[A-Za-z0-9_] char to `_`, including
        // unicode the prior denylist missed — the arrow `→` from a prose-fragment initiator actor name
        // (GIC-600's `redistributor → distributor`) that previously malformed the whole `.isf`.
        assert_eq!(
            sanitize_isf_name("redistributor→_distributor_distributor→_redistributor"),
            "redistributor_distributor_distributor_redistributor"
        );
        assert_eq!(sanitize_isf_name("café—naïve"), "caf_na_ve");
        // The result is always a valid HDL identifier ([A-Za-z_]\w*).
        for s in ["→", "1→2", "  ", "Σλ"] {
            let out = sanitize_isf_name(s);
            assert!(
                out.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    && out.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_'),
                "sanitized {s:?} -> {out:?} is not a valid HDL identifier"
            );
        }
    }

    #[test]
    fn a_condition_is_a_guard_on_a_declared_signal_or_it_is_refused() {
        // EXTRACTION-REPRESENTATION-GAPS.22 — the shapes measured on the live chains' lowerable records.
        let signals: BTreeSet<String> = ["ACTIVATEACK", "RVALID", "AWSNOOP", "ARESETn"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let p: BTreeMap<String, SignalPolarity> =
            BTreeMap::from([("ARESETn".to_string(), SignalPolarity::ActiveLow)]);
        let guard = |text| condition_guard(text, &signals, Some("ARESETn"), &p);
        assert_eq!(guard(""), Ok(String::new()));
        assert_eq!(guard("true"), Ok(String::new()));
        assert_eq!(
            guard("ACTIVATEACK is LOW"),
            Ok("(== ACTIVATEACK 0)".to_string())
        );
        assert_eq!(
            guard("ARESETn is asserted"),
            Ok("(== ARESETn 0)".to_string())
        );
        // *… must be LOW during reset* names the contract's reset.
        for reset in ["reset", "during reset", "in reset."] {
            assert_eq!(guard(reset), Ok("(== ARESETn 0)".to_string()), "{reset}");
        }
        assert_eq!(
            condition_guard("reset", &signals, None, &p),
            Err(LevelRefusal::ConditionNotLowered)
        );
        // `asserted` on a signal of unstated polarity, and a word that is not a level.
        assert_eq!(
            guard("RVALID is asserted"),
            Err(LevelRefusal::PolarityUnknown)
        );
        assert_eq!(guard("AWSNOOP is CMO"), Err(LevelRefusal::NotALevel));
        for prose in [
            "AWSNOOP is not CMO",
            "it is driving the final read transfer in the transaction",
            "PSEL and PWRITE are asserted",
            "HREADY signal is LOW, unless HRESP signal is ERROR",
            "ENABLE",
            "UNDECLARED is HIGH",
        ] {
            assert_eq!(
                guard(prose),
                Err(LevelRefusal::ConditionNotLowered),
                "{prose}"
            );
        }
    }

    #[test]
    fn branch_predicate_guard_falls_back_to_selector_when_no_predicate() {
        let branch = ControlBranchRecord {
            branch_id: "b0".to_string(),
            declaration_order: 0,
            predicate: None,
            actions: vec![],
            supporting_statement_ids: vec![],
            automation_confidence: crate::ir::source::AutomationConfidence::Low,
        };
        assert_eq!(branch_predicate_guard(&branch, "sel_text"), "sel_text");
    }

    // --- emitter invariants ----------------------------------------------

    #[test]
    fn render_always_emits_clock_reset_watchdog_and_is_balanced() {
        let out = minimal_isf().render();
        assert!(out.starts_with("(actor a"), "actor header missing:\n{out}");
        assert!(out.contains("  (clock clk)"), "clock missing:\n{out}");
        assert!(
            out.contains("  (reset (rst_n async active_low))"),
            "reset missing:\n{out}"
        );
        assert!(out.contains("  (watchdog 16)"), "watchdog missing:\n{out}");
        assert!(out.ends_with(")"), "must end with closing paren:\n{out}");
        // No interface block when there are no signals.
        assert!(!out.contains("(interface"), "unexpected interface:\n{out}");
        assert_eq!(paren_balance(&out), 0, "unbalanced parens:\n{out}");
    }

    #[test]
    fn render_interface_is_sorted_and_deduped_by_construction() {
        let mut isf = minimal_isf();
        // Insert out of order, plus an exact duplicate.
        isf.signals.insert(IsfSignal {
            name: "ZZ".to_string(),
            direction: IsfDirection::Output,
            width: 1,
            width_param: None,
        });
        isf.signals.insert(IsfSignal {
            name: "AA".to_string(),
            direction: IsfDirection::Input,
            width: 8,
            width_param: None,
        });
        isf.signals.insert(IsfSignal {
            name: "AA".to_string(),
            direction: IsfDirection::Input,
            width: 8,
            width_param: None,
        });
        assert_eq!(
            isf.signals.len(),
            2,
            "BTreeSet must dedup identical signals"
        );
        let out = isf.render();
        let aa = out.find("(input AA (width 8))").expect("AA line");
        let zz = out.find("(output ZZ (width 1))").expect("ZZ line");
        assert!(aa < zz, "BTreeSet ordering must place AA before ZZ:\n{out}");
        assert!(out.contains("  (interface"));
        assert_eq!(paren_balance(&out), 0);
    }

    #[test]
    fn render_emits_storage_drives_and_rules_without_fabricated_priorities() {
        let mut isf = minimal_isf();
        isf.storage.push(IsfStorageVar {
            name: "acc".to_string(),
            width: 8,
            reset: None,
            fields: vec![],
        });
        isf.drives.push(IsfNamedDrive {
            name: "out".to_string(),
            body: vec![("sig".to_string(), "1".to_string())],
        });
        isf.rules.push(IsfRule {
            name: "r_guarded".to_string(),
            condition: "(== en 1)".to_string(),
            drives: vec![("o".to_string(), "0".to_string())],
        });
        isf.rules.push(IsfRule {
            name: "r_uncond".to_string(),
            condition: String::new(),
            drives: vec![],
        });
        let out = isf.render();
        assert!(out.contains("  (storage"));
        assert!(out.contains("    (var acc (width 8))"));
        assert!(out.contains("  (drive (out val) (sig 1))"));
        // Guarded rule keeps its condition; unconditional rule omits it.
        assert!(out.contains("  (rule r_guarded (== en 1)"));
        assert!(out.contains("  (rule r_uncond\n") || out.contains("  (rule r_uncond)"));
        assert!(!out.contains("(priority "));
        assert_eq!(paren_balance(&out), 0);
    }

    // --- ISF-REGISTER-RESET-EMIT.2: register reset lowering ---------------

    fn reset_field(hi: u32, lo: u32, reset: Option<&str>) -> RegisterFieldRecord {
        RegisterFieldRecord {
            field_name: format!("f_{hi}_{lo}"),
            bits_high: Some(hi),
            bits_low: Some(lo),
            bit_width: None,
            access_type: None,
            reset_value: reset.map(|s| s.to_string()),
            description: None,
            enumerated_values: vec![],
            signal_bindings: Vec::new(),
        }
    }

    #[test]
    fn parse_reset_literal_accepts_numeric_rejects_symbolic() {
        assert_eq!(parse_reset_literal("0"), Some(0));
        assert_eq!(parse_reset_literal("7"), Some(7));
        assert_eq!(parse_reset_literal("0x1c"), Some(0x1c));
        assert_eq!(parse_reset_literal("0X40"), Some(0x40));
        assert_eq!(parse_reset_literal("0b101"), Some(0b101));
        assert_eq!(parse_reset_literal("1Fh"), Some(0x1f));
        assert_eq!(parse_reset_literal("  3 "), Some(3));
        // Symbolic / non-numeric: never guessed (ADR-0006).
        for s in [
            "-",
            "X",
            "Xh",
            "IMPLEMENTATION DEFINED",
            "0x--",
            "8'h1F",
            "True",
            "",
        ] {
            assert_eq!(parse_reset_literal(s), None, "should reject {s:?}");
        }
    }

    #[test]
    fn classify_register_reset_composes_tiles_and_gates() {
        // Two located fields tile into a register value, fits the var width (8) → Emit.
        // f[7:4]=0x8, f[3:0]=0xc → 0x8c.
        let fields = vec![
            reset_field(7, 4, Some("0x8")),
            reset_field(3, 0, Some("0xC")),
        ];
        assert!(matches!(
            classify_register_reset(&fields, 8),
            RegisterResetOutcome::Emit(0x8c)
        ));
        // Same composition but the (max-field-extent) var width is only 4 → over-width → DeferredWidth.
        assert!(matches!(
            classify_register_reset(&fields, 4),
            RegisterResetOutcome::DeferredWidth
        ));
        // All-zero documented reset → faithfully the FSMGen default → DefaultZero (omit).
        let zeros = vec![reset_field(7, 4, Some("0")), reset_field(3, 0, Some("0b0"))];
        assert!(matches!(
            classify_register_reset(&zeros, 8),
            RegisterResetOutcome::DefaultZero
        ));
        // A symbolic value anywhere → NotLowerable.
        let symbolic = vec![reset_field(7, 4, Some("0x8")), reset_field(3, 0, Some("-"))];
        assert!(matches!(
            classify_register_reset(&symbolic, 8),
            RegisterResetOutcome::NotLowerable
        ));
        // Partial coverage (a field without a reset_value) → NotLowerable.
        let partial = vec![reset_field(7, 4, Some("0x8")), reset_field(3, 0, None)];
        assert!(matches!(
            classify_register_reset(&partial, 8),
            RegisterResetOutcome::NotLowerable
        ));
        // A field value that overflows its own field width → NotLowerable.
        let overwide = vec![reset_field(3, 0, Some("0x1f"))]; // 0x1f needs 5 bits, field is 4
        assert!(matches!(
            classify_register_reset(&overwide, 8),
            RegisterResetOutcome::NotLowerable
        ));
        // Overlapping fields → NotLowerable.
        let overlap = vec![
            reset_field(7, 0, Some("0x1")),
            reset_field(3, 0, Some("0x1")),
        ];
        assert!(matches!(
            classify_register_reset(&overlap, 8),
            RegisterResetOutcome::NotLowerable
        ));
        // No field carries a reset → NoReset.
        let noreset = vec![reset_field(7, 0, None)];
        assert!(matches!(
            classify_register_reset(&noreset, 8),
            RegisterResetOutcome::NoReset
        ));
        // An unlocated field carrying a reset → NotLowerable (cannot place it).
        let unlocated = vec![RegisterFieldRecord {
            field_name: "u".to_string(),
            bits_high: None,
            bits_low: None,
            bit_width: None,
            access_type: None,
            reset_value: Some("1".to_string()),
            description: None,
            enumerated_values: vec![],
            signal_bindings: Vec::new(),
        }];
        assert!(matches!(
            classify_register_reset(&unlocated, 8),
            RegisterResetOutcome::NotLowerable
        ));
    }

    fn register_with(size_bits: Option<u32>, fields: Vec<RegisterFieldRecord>) -> RegisterRecord {
        RegisterRecord {
            register_id: "r".to_string(),
            register_name: "R".to_string(),
            access_type: None,
            reset_value: None,
            offset_address: None,
            size_bits,
            fields,
            supporting_table_ids: vec![],
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    fn width_port(signal: &str, width: Option<WidthHint>) -> ActorPortRecord {
        ActorPortRecord {
            actor_id: "a".to_string(),
            actor_name: "A".to_string(),
            signal_name: signal.to_string(),
            direction: ActorRelativeDirection::Output,
            relation_basis: vec![],
            width_hint: width,
            source_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn actor_port_concrete_widths_recovers_single_unambiguous_width() {
        // KG-ISF-COMPLETENESS.2a.i: a signal gains a graph width only when it is a single,
        // unambiguous, concrete (> 1) value — otherwise the caller keeps the honest width-1 default.
        let ports = vec![
            width_port("ARSIZE", Some(WidthHint::Numeric(3))),
            width_port("ARSIZE", Some(WidthHint::Numeric(3))), // a second actor agrees → unambiguous
            width_port("HBURST", Some(WidthHint::Numeric(2))),
            width_port("AWADDR", Some(WidthHint::Numeric(1))), // width 1 is not a > 1 recovery
            width_port("CONFLICT", Some(WidthHint::Numeric(2))),
            width_port("CONFLICT", Some(WidthHint::Numeric(4))), // disagree → omitted (no guess)
            width_port("SYM", Some(WidthHint::Parametric("DATA_WIDTH".to_string()))), // symbolic → omitted
            width_port("NOHINT", None),
        ];
        let widths = actor_port_concrete_widths(&ports);
        assert_eq!(widths.get("ARSIZE"), Some(&3));
        assert_eq!(widths.get("HBURST"), Some(&2));
        assert_eq!(widths.get("AWADDR"), None);
        assert_eq!(widths.get("CONFLICT"), None);
        assert_eq!(widths.get("SYM"), None);
        assert_eq!(widths.get("NOHINT"), None);
        assert_eq!(widths.len(), 2);
    }

    fn param_signal(name: &str, width_param: Option<&str>) -> IsfSignal {
        IsfSignal {
            name: name.to_string(),
            direction: IsfDirection::Output,
            width: 1,
            width_param: width_param.map(str::to_string),
        }
    }

    #[test]
    fn symbolic_signal_widths_keeps_only_a_single_symbolic_width() {
        // KG-ISF-COMPLETENESS.2a.vii.b: every hint of a signal is read, across interface records
        // and actor ports; only a signal described by ONE symbolic width and no number qualifies.
        let data = WidthHint::Parametric("DATA_WIDTH".to_string());
        let addr = WidthHint::Parametric("ADDR_WIDTH".to_string());
        let other = WidthHint::Parametric("OTHER_WIDTH".to_string());
        let strobe = WidthHint::Parametric(" DATA_WIDTH / 8 ".to_string());
        let thirty_two = WidthHint::Numeric(32);
        let one = WidthHint::Numeric(1);
        let hints = vec![
            ("HRDATA", Some(&data)),
            ("HRDATA", Some(&data)), // the port agrees with the interface record
            ("HRDATA", None),
            ("HADDR", Some(&addr)),
            ("HADDR", Some(&thirty_two)), // a number keeps the concrete-width path
            ("HSEL", Some(&one)),
            ("SPLIT", Some(&data)),
            ("SPLIT", Some(&other)), // two symbolic widths: never guessed between
            ("WSTRB", Some(&strobe)),
            ("NOHINT", None),
        ];
        let widths = symbolic_signal_widths(hints);
        assert_eq!(widths.get("HRDATA").map(String::as_str), Some("DATA_WIDTH"));
        assert_eq!(
            widths.get("WSTRB").map(String::as_str),
            Some("DATA_WIDTH / 8")
        );
        assert_eq!(widths.len(), 2, "{widths:?}");
    }

    #[test]
    fn is_isf_identifier_accepts_only_a_bare_name() {
        assert!(is_isf_identifier("DATA_WIDTH"));
        assert!(is_isf_identifier("Num_RP_AR"));
        assert!(is_isf_identifier("_w"));
        assert!(!is_isf_identifier("DATA_WIDTH/8"));
        assert!(!is_isf_identifier("ceil(ADDR_WIDTH/8)"));
        assert!(!is_isf_identifier("8BIT"));
        assert!(!is_isf_identifier(""));
    }

    #[test]
    fn lower_parameter_widths_declares_parameters_and_residualizes_the_rest() {
        // KG-ISF-COMPLETENESS.2a.vii.b: a bare parameter becomes `(width P)` with a placeholder
        // `(P 1)`; an expression, and a name the actor already declares, stay `(width 1)`. Both
        // outcomes are named in one summary residual each.
        let signals: BTreeSet<IsfSignal> = ["HRDATA", "HWDATA", "WSTRB", "HPROT", "HSEL"]
            .into_iter()
            .map(|name| param_signal(name, None))
            .collect();
        let symbolic: BTreeMap<String, String> = [
            ("HRDATA", "DATA_WIDTH"),
            ("HWDATA", "DATA_WIDTH"),
            ("WSTRB", "DATA_WIDTH/8"),
            ("HPROT", "HSEL"),
        ]
        .into_iter()
        .map(|(signal, width)| (signal.to_string(), width.to_string()))
        .collect();
        let reserved = declared_isf_names(&signals, "clk", "rst_n", &[], &[], &[]);

        let (lowered, params, residuals) = lower_parameter_widths(
            signals,
            &symbolic,
            &BTreeMap::new(),
            &BTreeMap::new(),
            &reserved,
        );

        assert_eq!(params, BTreeMap::from([("DATA_WIDTH".to_string(), 1)]));
        let by_name: BTreeMap<&str, &IsfSignal> =
            lowered.iter().map(|s| (s.name.as_str(), s)).collect();
        assert_eq!(by_name["HRDATA"].width_param.as_deref(), Some("DATA_WIDTH"));
        assert_eq!(by_name["HWDATA"].width_param.as_deref(), Some("DATA_WIDTH"));
        for kept in ["WSTRB", "HPROT", "HSEL"] {
            assert_eq!(by_name[kept].width_param, None, "{kept}");
            assert_eq!(by_name[kept].width, 1, "{kept}");
        }
        let ids: Vec<&str> = residuals.iter().map(|r| r.packet_id.as_str()).collect();
        assert_eq!(
            ids,
            [
                "isf_signal_width_parameter_defaults",
                "isf_signal_width_unlowered"
            ]
        );
        let defaults = &residuals[0].why_unresolved;
        assert!(
            defaults.starts_with("1 parameter(s) size 2 signal(s)"),
            "{defaults}"
        );
        assert!(
            defaults.contains("`DATA_WIDTH` sizes HRDATA, HWDATA"),
            "{defaults}"
        );
        let unlowered = &residuals[1].why_unresolved;
        assert!(
            unlowered.starts_with("2 signal(s) keep `(width 1)`"),
            "{unlowered}"
        );
        let expression =
            "An expression, which no FSMGen actor parameter can state: WSTRB `DATA_WIDTH/8`";
        assert!(unlowered.contains(expression), "{unlowered}");
        let collision = "A name this actor already declares: HPROT `HSEL`";
        assert!(unlowered.contains(collision), "{unlowered}");
    }

    #[test]
    fn lower_parameter_widths_changes_nothing_without_a_symbolic_width() {
        // Every document whose widths are all concrete must emit exactly what it emitted before.
        let signals: BTreeSet<IsfSignal> = ["A", "B"]
            .into_iter()
            .map(|n| param_signal(n, None))
            .collect();
        let reserved = declared_isf_names(&signals, "clk", "rst_n", &[], &[], &[]);
        let (lowered, params, residuals) = lower_parameter_widths(
            signals.clone(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            &reserved,
        );
        assert_eq!(lowered, signals);
        assert!(params.is_empty());
        assert!(residuals.is_empty());
    }

    #[test]
    fn a_positive_stated_default_replaces_the_placeholder() {
        // KG-ISF-COMPLETENESS.2a.vii.c.i: `ADDR_WIDTH` states 32, so it is declared `(ADDR_WIDTH 32)`
        // and sizes its signal at 32 with no residual; `MECID_WIDTH` states 0 (absent by default),
        // which FSMGen refuses, and `DATA_WIDTH` states nothing, so both keep the placeholder and are
        // named in the residual with their reason.
        let signals: BTreeSet<IsfSignal> = ["AWADDR", "AWMECID", "WDATA"]
            .into_iter()
            .map(|name| param_signal(name, None))
            .collect();
        let symbolic: BTreeMap<String, String> = [
            ("AWADDR", "ADDR_WIDTH"),
            ("AWMECID", "MECID_WIDTH"),
            ("WDATA", "DATA_WIDTH"),
        ]
        .into_iter()
        .map(|(signal, width)| (signal.to_string(), width.to_string()))
        .collect();
        let stated = BTreeMap::from([
            ("ADDR_WIDTH".to_string(), 32),
            ("MECID_WIDTH".to_string(), 0),
        ]);
        let reserved = declared_isf_names(&signals, "clk", "rst_n", &[], &[], &[]);

        let (lowered, params, residuals) =
            lower_parameter_widths(signals, &symbolic, &stated, &BTreeMap::new(), &reserved);

        assert_eq!(params["ADDR_WIDTH"], 32);
        assert_eq!(params["MECID_WIDTH"], 1);
        assert_eq!(params["DATA_WIDTH"], 1);
        let awaddr = lowered.iter().find(|s| s.name == "AWADDR").expect("AWADDR");
        assert_eq!(awaddr.width, 32);
        assert_eq!(residuals.len(), 1);
        let text = &residuals[0].why_unresolved;
        assert!(!text.contains("ADDR_WIDTH` sizes"), "{text}");
        assert!(
            text.starts_with("2 parameter(s) size 2 signal(s)"),
            "{text}"
        );
        assert!(
            text.contains("`MECID_WIDTH` sizes AWMECID (the document's default is 0"),
            "{text}"
        );
        assert!(
            text.contains("`DATA_WIDTH` sizes WDATA (the document states no default)"),
            "{text}"
        );
    }

    #[test]
    fn stated_parameter_defaults_keeps_one_integer_value_per_parameter() {
        let symbol = |name: &str, kind, literal: &str| SymbolDefinitionRecord {
            symbol_id: format!("symbol_{name}"),
            symbol_name: name.to_string(),
            kind,
            value: Some(ControlExpressionRecord::Literal {
                literal: literal.to_string(),
            }),
            members: vec![],
            legal_values: vec![],
            recommended_values: vec![],
            declaration_order: 0,
            supporting_statement_ids: vec![],
            supporting_table_ids: vec![],
            automation_confidence: AutomationConfidence::High,
        };
        let symbols = vec![
            symbol("ADDR_WIDTH", SymbolDefinitionKind::Param, "32"),
            symbol("SPLIT", SymbolDefinitionKind::Param, "4"),
            symbol("SPLIT", SymbolDefinitionKind::Param, "8"), // two values: never chosen between
            symbol("EXPR", SymbolDefinitionKind::Param, "DATA_WIDTH/8"),
            symbol("LIMIT", SymbolDefinitionKind::Constant, "7"), // a constant is not a parameter
        ];
        assert_eq!(
            stated_parameter_defaults(&symbols),
            BTreeMap::from([("ADDR_WIDTH".to_string(), 32)])
        );
    }

    #[test]
    fn the_smallest_positive_legal_value_is_chosen_and_named_as_a_choice() {
        // KG-ISF-COMPLETENESS.2a.vii.c.ii: a parameter with no positive stated default takes the
        // smallest positive value it allows, named in its own residual; a positive stated default
        // still wins, and a parameter with neither keeps the placeholder.
        let signals: BTreeSet<IsfSignal> = ["AWADDR", "WDATA", "RDATA", "AWMECID", "AWUSER"]
            .into_iter()
            .map(|name| param_signal(name, None))
            .collect();
        let symbolic: BTreeMap<String, String> = [
            ("AWADDR", "ADDR_WIDTH"),
            ("WDATA", "DATA_WIDTH"),
            ("RDATA", "DATA_WIDTH"),
            ("AWMECID", "MECID_WIDTH"),
            ("AWUSER", "USER_WIDTH"),
        ]
        .into_iter()
        .map(|(signal, width)| (signal.to_string(), width.to_string()))
        .collect();
        let stated = BTreeMap::from([
            ("ADDR_WIDTH".to_string(), 32),
            ("MECID_WIDTH".to_string(), 0),
        ]);
        let legal = |value| ChosenDefault {
            value,
            basis: ChosenDefaultBasis::Legal,
        };
        let smallest = BTreeMap::from([
            ("ADDR_WIDTH".to_string(), legal(1)),
            ("DATA_WIDTH".to_string(), legal(8)),
            ("MECID_WIDTH".to_string(), legal(16)),
        ]);
        let reserved = declared_isf_names(&signals, "clk", "rst_n", &[], &[], &[]);

        let (lowered, params, residuals) =
            lower_parameter_widths(signals, &symbolic, &stated, &smallest, &reserved);

        assert_eq!(
            params,
            BTreeMap::from([
                ("ADDR_WIDTH".to_string(), 32),
                ("DATA_WIDTH".to_string(), 8),
                ("MECID_WIDTH".to_string(), 16),
                ("USER_WIDTH".to_string(), 1),
            ])
        );
        let wdata = lowered.iter().find(|s| s.name == "WDATA").expect("WDATA");
        assert_eq!(wdata.width, 8);
        let ids: Vec<&str> = residuals.iter().map(|r| r.packet_id.as_str()).collect();
        assert_eq!(
            ids,
            [
                "isf_signal_width_parameter_defaults",
                "isf_signal_width_parameter_chosen_defaults"
            ]
        );
        let placeholder = &residuals[0].why_unresolved;
        assert!(
            placeholder.starts_with("1 parameter(s) size 1 signal(s)"),
            "{placeholder}"
        );
        assert!(
            placeholder.contains("`USER_WIDTH` sizes AWUSER"),
            "{placeholder}"
        );
        let chosen = &residuals[1].why_unresolved;
        assert!(
            chosen.starts_with("2 parameter(s) size 3 signal(s)"),
            "{chosen}"
        );
        assert!(
            chosen.contains(
                "`DATA_WIDTH` = 8 sizes RDATA, WDATA (the smallest positive legal value; the \
                 document states no default); `MECID_WIDTH` = 16 sizes AWMECID (the smallest \
                 positive legal value; the document's default is 0"
            ),
            "{chosen}"
        );
        assert!(!chosen.contains("ADDR_WIDTH"), "{chosen}");
    }

    #[test]
    fn chosen_parameter_defaults_prefers_a_recommended_legal_value_and_skips_zero() {
        // KG-ISF-COMPLETENESS.2a.vii.c.ii/.c.iii: legal values inside a recommended range first, then
        // legal values, then recommended ones; zero never; disagreeing symbols never chosen between.
        let ranges = |ranges: &[(u64, u64)]| {
            ranges
                .iter()
                .map(|&(low, high)| ParameterValueRange { low, high })
                .collect::<Vec<_>>()
        };
        let symbol = |name: &str, kind, legal: &[(u64, u64)], recommended: &[(u64, u64)]| {
            SymbolDefinitionRecord {
                symbol_id: format!("symbol_{name}"),
                symbol_name: name.to_string(),
                kind,
                value: None,
                members: vec![],
                legal_values: ranges(legal),
                recommended_values: ranges(recommended),
                declaration_order: 0,
                supporting_statement_ids: vec![],
                supporting_table_ids: vec![],
                automation_confidence: AutomationConfidence::High,
            }
        };
        let param = SymbolDefinitionKind::Param;
        let symbols = vec![
            symbol("DATA_WIDTH", param, &[(8, 8), (16, 16), (1024, 1024)], &[]),
            symbol("ID_WIDTH", param, &[(0, 32)], &[]),
            symbol("ACT_WIDTH", param, &[(0, 0), (16, 32)], &[]),
            symbol(
                "BUS_WIDTH",
                param,
                &[(8, 8), (32, 32), (64, 64)],
                &[(16, 64)],
            ),
            symbol("WIDE_WIDTH", param, &[(8, 8), (16, 16)], &[(32, 64)]), // no overlap
            symbol("ADDR_WIDTH", param, &[], &[(10, 64)]),
            symbol("ABSENT", param, &[(0, 0)], &[(0, 0)]), // no positive value
            symbol("SPLIT", param, &[(2, 4)], &[]),
            symbol("SPLIT", param, &[(4, 8)], &[]), // disagreeing lists
            symbol("LIMIT", SymbolDefinitionKind::Constant, &[(1, 7)], &[]), // not a parameter
        ];
        let chosen = |value, basis| ChosenDefault { value, basis };
        assert_eq!(
            chosen_parameter_defaults(&symbols),
            BTreeMap::from([
                (
                    "ACT_WIDTH".to_string(),
                    chosen(16, ChosenDefaultBasis::Legal)
                ),
                (
                    "ADDR_WIDTH".to_string(),
                    chosen(10, ChosenDefaultBasis::Recommended)
                ),
                (
                    "BUS_WIDTH".to_string(),
                    chosen(32, ChosenDefaultBasis::LegalAndRecommended)
                ),
                (
                    "DATA_WIDTH".to_string(),
                    chosen(8, ChosenDefaultBasis::Legal)
                ),
                ("ID_WIDTH".to_string(), chosen(1, ChosenDefaultBasis::Legal)),
                (
                    "WIDE_WIDTH".to_string(),
                    chosen(8, ChosenDefaultBasis::Legal)
                ),
            ])
        );
    }

    fn polarity(entries: &[(&str, SignalPolarity)]) -> BTreeMap<String, SignalPolarity> {
        entries.iter().map(|(s, p)| (s.to_string(), *p)).collect()
    }

    #[test]
    fn a_level_resolves_only_through_the_signals_stated_polarity() {
        // EXTRACTION-REPRESENTATION-GAPS.8: `asserted` is `0` on an active-low signal and has no value at
        // all on a signal whose polarity the document never states (ADR 0037 §5 — never its spelling).
        let p = polarity(&[
            ("nSRST", SignalPolarity::ActiveLow),
            ("PSEL", SignalPolarity::ActiveHigh),
        ]);
        assert_eq!(resolve_level("nSRST", "asserted", &p), Ok("0".to_string()));
        assert_eq!(
            resolve_level("nSRST", "deasserted", &p),
            Ok("1".to_string())
        );
        assert_eq!(resolve_level("PSEL", "asserted", &p), Ok("1".to_string()));
        assert_eq!(
            resolve_level("PRESETn", "asserted", &p),
            Err(LevelRefusal::PolarityUnknown)
        );
        assert_eq!(resolve_level("PRESETn", "LOW", &p), Ok("0".to_string()));
        assert_eq!(resolve_level("X", "0b000", &p), Ok("0b000".to_string()));
        assert_eq!(
            resolve_level("X", "VALID", &p),
            Err(LevelRefusal::NotALevel)
        );
    }

    #[test]
    fn an_action_states_a_level_only_when_it_names_one() {
        assert_eq!(action_stated_level("must be LOW"), Ok("low".to_string()));
        assert_eq!(
            action_stated_level("must not be asserted"),
            Ok("deasserted".to_string())
        );
        assert_eq!(
            action_stated_level("shall not be HIGH."),
            Ok("low".to_string())
        );
        assert_eq!(
            action_stated_level("must be 0b000"),
            Ok("0b000".to_string())
        );
        for none in [
            "must",
            "must not",
            "(see source_text)",
            "must remain stable",
            "must not be 0b000",
        ] {
            assert_eq!(
                action_stated_level(none),
                Err(LevelRefusal::NoLevelStated),
                "{none}"
            );
        }
    }

    #[test]
    fn an_obligation_keeps_its_stated_condition_or_is_refused() {
        // EXTRACTION-REPRESENTATION-GAPS.22 — the AXI shapes. Before, *AWSNOOP is not CMO* was dropped (the rule
        // drove unconditionally) and *… LOW during reset* became `(== reset 1)` on a signal that does not exist.
        use crate::ir::intent::TemporalInvariantRecord;
        use crate::ir::source::{
            ConditionalRuleRecord, SignalConstraintKind, SignalConstraintRecord,
        };
        let constraint = |id: &str, signal: &str, condition: Option<&str>| SignalConstraintRecord {
            constraint_id: id.to_string(),
            subject_signal: signal.to_string(),
            constraint_kind: SignalConstraintKind::MustBeLow,
            target_value: None,
            condition_text: condition.map(str::to_string),
            negated: false,
            source_text: String::new(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::High,
        };
        let rule = |antecedent: &str| ConditionalRuleRecord {
            rule_id: "r".to_string(),
            antecedent_text: antecedent.to_string(),
            consequent_signal: Some("AWCACHE".to_string()),
            consequent_action: "must be 0".to_string(),
            source_text: String::new(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::High,
        };
        let invariant =
            |id: &str, condition: Option<(&str, Option<&str>)>| TemporalInvariantRecord {
                invariant_id: id.to_string(),
                subject_signal: "SYSCOREQ".to_string(),
                invariant_kind: TemporalInvariantKind::MustBeAsserted,
                condition_signal: condition.map(|(signal, _)| signal.to_string()),
                condition_value: condition.and_then(|(_, value)| value.map(str::to_string)),
                target_value: Some("0".to_string()),
                source_text: String::new(),
                supporting_statement_ids: vec![],
                automation_confidence: AutomationConfidence::High,
            };
        let signals = declared(&["ACTIVATEREQ", "SYSCOREQ", "AWCACHE", "ACTIVATEACK"]);
        let p = polarity(&[("ARESETn", SignalPolarity::ActiveLow)]);
        let (rules, refused) = lower_obligations(
            &[rule("AWSNOOP is not CMO"), rule("ACTIVATEACK is LOW")],
            &[
                constraint("c_reset", "ACTIVATEREQ", Some("reset")),
                constraint("c_arestn", "SYSCOREQ", Some("ARESETn is asserted")),
                constraint(
                    "c_prose",
                    "SYSCOREQ",
                    Some("it is driving the final transfer"),
                ),
            ],
            &[
                invariant("tinv_guarded", Some(("ARESETn", Some("asserted")))),
                invariant("tinv_no_level", Some(("ACTIVATEACK", None))),
            ],
            &signals,
            &BTreeSet::new(),
            Some("ARESETn"),
            &p,
            &[],
        );
        let emitted: Vec<(&str, &str)> = rules
            .iter()
            .map(|r| (r.name.as_str(), r.condition.as_str()))
            .collect();
        assert_eq!(
            emitted,
            [
                ("rule_1", "(== ACTIVATEACK 0)"),
                ("constraint_0", "(== ARESETn 0)"),
                ("constraint_1", "(== ARESETn 0)"),
                ("tinv_guarded", "(== ARESETn 0)"),
            ]
        );
        let refused: Vec<(&str, LevelRefusal)> = refused
            .iter()
            .map(|r| (r.name.as_str(), r.refusal))
            .collect();
        assert_eq!(
            refused,
            [
                ("rule_0", LevelRefusal::ConditionNotLowered),
                ("constraint_2", LevelRefusal::ConditionNotLowered),
                ("tinv_no_level", LevelRefusal::ConditionNotLowered),
            ]
        );
    }

    #[test]
    fn an_obligation_on_the_actors_own_input_is_refused_not_driven() {
        // EXTRACTION-REPRESENTATION-GAPS.10.a — AXI *ASKSTOP must be LOW when ACTIVATEACK is LOW*, renamed: the
        // Subordinate drives the signal, so on the Manager, which reads it, the obligation is an assumption. As a
        // Manager `(rule …)` it stated that the Manager drives its own input.
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};
        let constraint = |id: &str, signal: &str| SignalConstraintRecord {
            constraint_id: id.to_string(),
            subject_signal: signal.to_string(),
            constraint_kind: SignalConstraintKind::MustBeLow,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: String::new(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::High,
        };
        let signals = declared(&["XSTOP", "XREQ"]);
        let (rules, refused) = lower_obligations(
            &[],
            &[
                constraint("c_input", "XSTOP"),
                constraint("c_output", "XREQ"),
            ],
            &[],
            &signals,
            &declared(&["XSTOP"]),
            None,
            &BTreeMap::new(),
            &[],
        );
        assert_eq!(
            rules
                .iter()
                .flat_map(|rule| rule.drives.iter().map(|(signal, _)| signal.as_str()))
                .collect::<Vec<_>>(),
            ["XREQ"]
        );
        assert_eq!(
            refused
                .iter()
                .map(|r| (r.signal.as_str(), r.refusal))
                .collect::<Vec<_>>(),
            [("XSTOP", LevelRefusal::DrivesAnInput)]
        );
    }

    #[test]
    fn a_tie_off_is_listed_not_driven() {
        // EXTRACTION-REPRESENTATION-GAPS.10.e — AXI *an attached Subordinate must have its AWSNOOP input tied LOW*,
        // renamed: the level the input is tied to when the Manager omits the signal. On the Manager, which declares
        // the signal as its output, it was lowered as a drive of 0 and collided with a transaction's drive.
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};
        let constraint =
            |id: &str, signal: &str, kind: SignalConstraintKind| SignalConstraintRecord {
                constraint_id: id.to_string(),
                subject_signal: signal.to_string(),
                constraint_kind: kind,
                target_value: None,
                condition_text: None,
                negated: false,
                source_text: String::new(),
                supporting_statement_ids: vec![],
                automation_confidence: AutomationConfidence::High,
            };
        let (rules, refused) = lower_obligations(
            &[],
            &[
                constraint(
                    "c_tied",
                    "XSNOOP",
                    SignalConstraintKind::TiedTo {
                        value: "LOW".to_string(),
                    },
                ),
                constraint("c_level", "XREQ", SignalConstraintKind::MustBeLow),
            ],
            &[],
            &declared(&["XSNOOP", "XREQ"]),
            &declared(&[]),
            None,
            &BTreeMap::new(),
            &[],
        );
        assert_eq!(
            rules
                .iter()
                .flat_map(|rule| rule.drives.iter().map(|(signal, _)| signal.as_str()))
                .collect::<Vec<_>>(),
            ["XREQ"]
        );
        assert_eq!(
            refused
                .iter()
                .map(|r| (r.signal.as_str(), r.refusal))
                .collect::<Vec<_>>(),
            [("XSNOOP", LevelRefusal::TieOff)]
        );
    }

    #[test]
    fn an_obligation_drives_only_the_level_the_document_states() {
        // The measured defect: `must be LOW` lowered as `1`, `must remain stable` as `1`, a bare `must` as
        // `1`, `must be 0b000` as `1`. Each now drives what it states, or is refused and counted.
        use crate::ir::intent::TemporalInvariantRecord;
        use crate::ir::source::{
            ConditionalRuleRecord, SignalConstraintKind, SignalConstraintRecord,
        };
        let constraint = |id: &str, signal: &str, kind: SignalConstraintKind, negated: bool| {
            SignalConstraintRecord {
                constraint_id: id.to_string(),
                subject_signal: signal.to_string(),
                constraint_kind: kind,
                target_value: None,
                condition_text: None,
                negated,
                source_text: String::new(),
                supporting_statement_ids: vec![],
                automation_confidence: AutomationConfidence::High,
            }
        };
        let rule = |signal: &str, action: &str| ConditionalRuleRecord {
            rule_id: "r".to_string(),
            antecedent_text: String::new(),
            consequent_signal: Some(signal.to_string()),
            consequent_action: action.to_string(),
            source_text: String::new(),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::High,
        };
        let invariant =
            |id: &str, signal: &str, kind, target: Option<&str>| TemporalInvariantRecord {
                invariant_id: id.to_string(),
                subject_signal: signal.to_string(),
                invariant_kind: kind,
                condition_signal: None,
                condition_value: None,
                target_value: target.map(str::to_string),
                source_text: String::new(),
                supporting_statement_ids: vec![],
                automation_confidence: AutomationConfidence::High,
            };
        let signals = declared(&["AWLOCK", "AWCMO", "PADDR", "nSRST", "PSEL", "HTRANS"]);
        let p = polarity(&[("nSRST", SignalPolarity::ActiveLow)]);
        let (rules, refused) = lower_obligations(
            &[rule("AWCMO", "must be 0b000"), rule("HTRANS", "must")],
            &[
                constraint("c_low", "AWLOCK", SignalConstraintKind::MustBeLow, false),
                constraint(
                    "c_not_asserted",
                    "nSRST",
                    SignalConstraintKind::MustBeAsserted,
                    true,
                ),
                constraint(
                    "c_asserted",
                    "PSEL",
                    SignalConstraintKind::MustBeAsserted,
                    false,
                ),
                constraint(
                    "c_stable",
                    "PADDR",
                    SignalConstraintKind::MustBeStable,
                    false,
                ),
            ],
            &[
                invariant(
                    "tinv_low",
                    "AWLOCK",
                    TemporalInvariantKind::MustBeAsserted,
                    Some("0"),
                ),
                invariant(
                    "tinv_legacy",
                    "AWLOCK",
                    TemporalInvariantKind::MustBeAsserted,
                    None,
                ),
                invariant(
                    "tinv_stable",
                    "PADDR",
                    TemporalInvariantKind::MustRemainStable,
                    None,
                ),
            ],
            &signals,
            &BTreeSet::new(),
            None,
            &p,
            &[],
        );
        let drives: Vec<(&str, &str, &str)> = rules
            .iter()
            .map(|r| {
                (
                    r.name.as_str(),
                    r.drives[0].0.as_str(),
                    r.drives[0].1.as_str(),
                )
            })
            .collect();
        assert_eq!(
            drives,
            [
                ("rule_0", "AWCMO", "0b000"),
                ("constraint_0", "AWLOCK", "0"),
                ("constraint_1", "nSRST", "1"),
                ("tinv_low", "AWLOCK", "0"),
            ]
        );
        let refusals: Vec<(&str, LevelRefusal)> = refused
            .iter()
            .map(|r| (r.name.as_str(), r.refusal))
            .collect();
        assert_eq!(
            refusals,
            [
                ("rule_1", LevelRefusal::NoLevelStated),
                ("constraint_2", LevelRefusal::PolarityUnknown),
                ("constraint_3", LevelRefusal::NoLevelStated),
                ("tinv_legacy", LevelRefusal::NoLevelStated),
                ("tinv_stable", LevelRefusal::NoLevelStated),
            ]
        );
        let packet = refused_obligations_residual_packet(&refused);
        assert_eq!(packet.packet_id, "isf_obligations_not_lowered");
        assert!(
            packet.why_unresolved.starts_with("5 obligation(s)"),
            "{}",
            packet.why_unresolved
        );
        assert!(
            packet.why_unresolved.contains("constraint_2 (PSEL)"),
            "{}",
            packet.why_unresolved
        );
    }

    #[test]
    fn a_guard_on_an_unknown_polarity_refuses_the_rule_instead_of_vanishing() {
        // Dropping the guard would turn `when PSEL is asserted, X` into an unconditional `X`.
        let signals = declared(&["PSEL", "PENABLE"]);
        assert_eq!(
            guard_from_candidates(
                [("PSEL", "asserted")].into_iter(),
                std::iter::empty(),
                &signals,
                &BTreeMap::new()
            )
            .map_err(|reason| reason.contains("polarity")),
            Err(true)
        );
        let p = polarity(&[("PSEL", SignalPolarity::ActiveHigh)]);
        assert_eq!(
            guard_from_candidates(
                [("PSEL", "asserted")].into_iter(),
                std::iter::empty(),
                &signals,
                &p
            ),
            Ok("(== PSEL 1)".to_string())
        );
        // EXTRACTION-REPRESENTATION-GAPS.22: the candidates are all the rule's conditions. Every one is kept,
        // joined by `(&` … `)`; one that is no level, or names an undeclared signal, refuses the whole rule
        // (skipping it, as before, dropped the condition).
        assert_eq!(
            guard_from_candidates(
                [("PSEL", "asserted"), ("PENABLE", "HIGH")].into_iter(),
                std::iter::empty(),
                &signals,
                &p
            ),
            Ok("(& (== PSEL 1) (== PENABLE 1))".to_string())
        );
        assert_eq!(
            guard_from_candidates(
                [("PSEL", "NONSEQ"), ("PENABLE", "HIGH")].into_iter(),
                std::iter::empty(),
                &signals,
                &BTreeMap::new()
            )
            .map_err(|reason| reason.contains("not a level")),
            Err(true)
        );
        assert_eq!(
            guard_from_candidates(
                [("MISSING", "HIGH"), ("PENABLE", "HIGH")].into_iter(),
                std::iter::empty(),
                &signals,
                &BTreeMap::new()
            )
            .map_err(|reason| reason.contains("does not declare")),
            Err(true)
        );
        // AXI's fused SYSCOREQ/SYSCOACK contract: the same signal at two levels would be a guard that never
        // holds, so the rule is refused; the same condition twice is one condition.
        assert_eq!(
            guard_from_candidates(
                [
                    ("PSEL", "asserted"),
                    ("PENABLE", "LOW"),
                    ("PENABLE", "HIGH")
                ]
                .into_iter(),
                std::iter::empty(),
                &signals,
                &p
            )
            .map_err(|reason| reason.contains("contradict")),
            Err(true)
        );
        assert_eq!(
            guard_from_candidates(
                [("PENABLE", "HIGH"), ("PENABLE", "1")].into_iter(),
                std::iter::empty(),
                &signals,
                &p
            ),
            Ok("(== PENABLE 1)".to_string())
        );
    }

    #[test]
    fn render_declares_width_params_first_and_sizes_signals_by_name() {
        let mut isf = minimal_isf();
        assert!(
            !isf.render().contains("(params"),
            "no parameter, no params block"
        );
        isf.width_params.insert("DATA_WIDTH".to_string(), 1);
        isf.signals
            .insert(param_signal("HRDATA", Some("DATA_WIDTH")));
        isf.signals.insert(param_signal("HSEL", None));
        let text = isf.render();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines[1..4], ["  (params", "    (DATA_WIDTH 1)", "  )"]);
        assert!(
            text.contains("    (output HRDATA (width DATA_WIDTH))"),
            "{text}"
        );
        assert!(text.contains("    (output HSEL (width 1))"), "{text}");
    }

    #[test]
    fn parse_sized_literal_reads_based_literals_and_ignores_bare_and_symbols() {
        // ISF-VALUE-WIDTH-EMIT.2: 0b… notation width = binary-digit count; 0x… = hex-digit count × 4
        // (exactly the width FSMGen's operand contract uses).
        assert_eq!(parse_sized_literal("0b00"), Some((0, 2)));
        assert_eq!(parse_sized_literal("0B01"), Some((1, 2)));
        assert_eq!(parse_sized_literal("0b000"), Some((0, 3)));
        assert_eq!(parse_sized_literal("0x7D"), Some((125, 8)));
        assert_eq!(parse_sized_literal("0XFF"), Some((255, 8)));
        // Bare decimals are unsized; symbols / references / already-cast literals are not based
        // literals → all left untouched (None).
        assert_eq!(parse_sized_literal("0"), None);
        assert_eq!(parse_sized_literal("1"), None);
        assert_eq!(parse_sized_literal("IDLE"), None);
        assert_eq!(parse_sized_literal("mode.BUSY"), None);
        assert_eq!(parse_sized_literal("7'd125"), None);
        assert_eq!(parse_sized_literal("0xZZ"), None);
    }

    #[test]
    fn align_value_to_width_keeps_aligns_and_residualizes() {
        // Notation width already matches → byte-identical Keep (the ARTAGOP 0b00-on-width-2 case).
        assert!(matches!(align_value_to_width("0b00", 2), ValueAlign::Keep));
        // Bare decimal / enum symbol → Keep (unsized / not a literal).
        assert!(matches!(align_value_to_width("0", 1), ValueAlign::Keep));
        assert!(matches!(align_value_to_width("IDLE", 4), ValueAlign::Keep));
        // Overshoots notation but the VALUE fits → width-aligned decimal cast (trace-bus ATID).
        match align_value_to_width("0x7D", 7) {
            ValueAlign::Replace(s) => assert_eq!(s, "7'd125"),
            _ => panic!("expected Replace for 0x7D on width 7"),
        }
        // 0B01 = value 1 on width 1 → fits → 1'd1 (DTI ATST; value preserved, lossless).
        match align_value_to_width("0B01", 1) {
            ValueAlign::Replace(s) => assert_eq!(s, "1'd1"),
            _ => panic!("expected Replace for 0B01 on width 1"),
        }
        // Value does not fit the signal width → Residualize (never truncate): 0b11=3 on width 1.
        assert!(matches!(
            align_value_to_width("0b11", 1),
            ValueAlign::Residualize
        ));
        assert!(matches!(
            align_value_to_width("0xFF", 4),
            ValueAlign::Residualize
        ));
    }

    #[test]
    fn align_rule_drive_widths_aligns_fitting_and_drops_overflow_with_residual() {
        let widths: BTreeMap<String, u32> = [
            ("ATID".to_string(), 7u32),
            ("ATST".to_string(), 1u32),
            ("OVR".to_string(), 1u32),
        ]
        .into_iter()
        .collect();
        let rules = vec![
            IsfRule {
                name: "r_atid".into(),
                condition: String::new(),
                drives: vec![("ATID".into(), "0x7D".into())],
            },
            IsfRule {
                name: "r_atst".into(),
                condition: String::new(),
                drives: vec![("ATST".into(), "0B01".into())],
            },
            IsfRule {
                name: "r_over".into(),
                condition: String::new(),
                drives: vec![("OVR".into(), "0b11".into())],
            },
            // A signal of unknown width is left byte-identical.
            IsfRule {
                name: "r_unk".into(),
                condition: String::new(),
                drives: vec![("UNK".into(), "0xAB".into())],
            },
        ];
        let (aligned, residuals) = align_rule_drive_widths(rules, &widths);
        let names: Vec<&str> = aligned.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["r_atid", "r_atst", "r_unk"]); // r_over dropped (3 > width 1)
        assert_eq!(aligned[0].drives[0].1, "7'd125"); // ATID width-aligned
        assert_eq!(aligned[1].drives[0].1, "1'd1"); // ATST width-aligned, value preserved
        assert_eq!(aligned[2].drives[0].1, "0xAB"); // unknown width → untouched
        assert_eq!(residuals.len(), 1);
        assert_eq!(residuals[0].packet_id, "isf_value_width_r_over");
    }

    #[test]
    fn drop_unconditional_overlap_conflicts_drops_guarded_minority_keeps_unconditional() {
        // The AMBA LPI shape: many unconditional `PREQ <- 1` rules and one guarded `PREQ <- 0`. The
        // guarded minority rule overlaps every unconditional rule (an empty guard is never disjoint), so
        // FSMGen rejects it; it must be dropped + residualized while every same-value rule survives.
        let rules = vec![
            IsfRule {
                name: "rule_5".into(),
                condition: String::new(),
                drives: vec![("PREQ".into(), "1".into())],
            },
            IsfRule {
                name: "rule_6".into(),
                condition: String::new(),
                drives: vec![("PREQ".into(), "1".into())],
            },
            IsfRule {
                name: "temporal_dyn_sigcon_0012".into(),
                condition: "(== PACCEPT 0)".into(),
                drives: vec![("PREQ".into(), "0".into())],
            },
            // A guarded rule that AGREES with the unconditional value is kept (same value = compatible).
            IsfRule {
                name: "guarded_agree".into(),
                condition: "(== PACCEPT 1)".into(),
                drives: vec![("PREQ".into(), "1".into())],
            },
            // An unrelated signal with no unconditional driver is untouched even with differing guards.
            IsfRule {
                name: "qdeny_a".into(),
                condition: "(== A 1)".into(),
                drives: vec![("QDENY".into(), "0".into())],
            },
            IsfRule {
                name: "qdeny_b".into(),
                condition: "(== B 1)".into(),
                drives: vec![("QDENY".into(), "1".into())],
            },
        ];
        let (kept, residuals) = drop_unconditional_overlap_conflicts(rules);
        let names: Vec<&str> = kept.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(
            names,
            vec!["rule_5", "rule_6", "guarded_agree", "qdeny_a", "qdeny_b"],
        );
        assert_eq!(residuals.len(), 1);
        assert_eq!(
            residuals[0].packet_id,
            "isf_unconditional_overlap_temporal_dyn_sigcon_0012"
        );
    }

    #[test]
    fn drop_unconditional_overlap_conflicts_noop_without_unconditional_driver() {
        // No unconditional driver on the signal → cross-guard pairs are FSMGen's general-overlap case,
        // out of scope for this precise pass; it must return byte-identical (no drop, no residual).
        let rules = vec![
            IsfRule {
                name: "g0".into(),
                condition: "(== X 0)".into(),
                drives: vec![("S".into(), "0".into())],
            },
            IsfRule {
                name: "g1".into(),
                condition: "(== X 1)".into(),
                drives: vec![("S".into(), "1".into())],
            },
        ];
        let (kept, residuals) = drop_unconditional_overlap_conflicts(rules);
        let names: Vec<&str> = kept.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["g0", "g1"]);
        assert!(residuals.is_empty());
    }

    #[test]
    fn drop_ungrounded_rule_transaction_conflicts_residualizes_unique_caller_overlap() {
        let drives = vec![IsfNamedDrive {
            name: "HTRANS".into(),
            body: vec![("HTRANS".into(), "val".into())],
        }];
        let transactions = vec![IsfTransaction {
            name: "idle_transfer".into(),
            on_trigger: None,
            on_steps: vec![],
            // Exercise recursive discovery rather than only a top-level drive.
            steps: vec![IsfTxnStep::When {
                condition: "(== HREADY 1)".into(),
                body: vec![IsfTxnStep::Drive {
                    name: "HTRANS".into(),
                    actuals: vec!["IDLE".into()],
                }],
            }],
            complete: "done".into(),
            latency_min: None,
            latency_max: None,
            contracts: vec![],
            stages: vec![],
        }];
        let rules = vec![
            IsfRule {
                name: "rule_3".into(),
                condition: String::new(),
                drives: vec![("HTRANS".into(), "1".into())],
            },
            IsfRule {
                name: "unrelated".into(),
                condition: String::new(),
                drives: vec![("HRESP".into(), "1".into())],
            },
        ];

        let (kept, residuals) =
            drop_ungrounded_rule_transaction_conflicts(rules, &transactions, &drives);
        assert_eq!(
            kept.iter()
                .map(|rule| rule.name.as_str())
                .collect::<Vec<_>>(),
            vec!["unrelated"]
        );
        assert_eq!(residuals.len(), 1);
        assert_eq!(
            residuals[0].packet_id,
            "isf_rule_transaction_conflict_rule_3"
        );
        assert!(residuals[0].why_unresolved.contains("idle_transfer"));
        assert!(
            residuals[0]
                .why_unresolved
                .contains("no source-grounded precedence")
        );
    }

    #[test]
    fn drop_ungrounded_rule_transaction_conflicts_keeps_multi_caller_drive_without_priority() {
        let drives = vec![IsfNamedDrive {
            name: "AWSNOOP".into(),
            body: vec![("AWSNOOP".into(), "val".into())],
        }];
        let transaction = |name: &str| IsfTransaction {
            name: name.into(),
            on_trigger: None,
            on_steps: vec![],
            steps: vec![IsfTxnStep::Drive {
                name: "AWSNOOP".into(),
                actuals: vec!["CMO".into()],
            }],
            complete: "done".into(),
            latency_min: None,
            latency_max: None,
            contracts: vec![],
            stages: vec![],
        };
        let transactions = vec![
            transaction("cmo_transaction"),
            transaction("evict_transaction"),
        ];
        let rules = vec![IsfRule {
            name: "rule_143".into(),
            condition: String::new(),
            drives: vec![("AWSNOOP".into(), "1".into())],
        }];

        let (kept, residuals) =
            drop_ungrounded_rule_transaction_conflicts(rules, &transactions, &drives);
        assert_eq!(kept.len(), 1);
        assert!(residuals.is_empty());

        let mut isf = minimal_isf();
        isf.drives = drives;
        isf.transactions = transactions;
        isf.rules = kept;
        assert!(!isf.render().contains("(priority "));
    }

    #[test]
    fn drop_unrenderable_rule_values_drops_prose_keeps_scalars() {
        // The AMBA AXI+ACE shape: a loopback constraint whose value is free prose FSMGen rejects, beside
        // legitimate scalar / based-literal / enum-symbol drives that must survive byte-identical.
        let rules = vec![
            IsfRule {
                name: "constraint_48".into(),
                condition: String::new(),
                drives: vec![(
                    "RLOOP".into(),
                    "the value that was presented on the ARLOOP signal".into(),
                )],
            },
            IsfRule {
                name: "ok_scalar".into(),
                condition: String::new(),
                drives: vec![("BTAGMATCH".into(), "0b01".into())],
            },
            IsfRule {
                name: "ok_enum".into(),
                condition: "(== X 1)".into(),
                drives: vec![("STATE".into(), "VALID".into())],
            },
        ];
        let (kept, residuals) = drop_unrenderable_rule_values(rules);
        let names: Vec<&str> = kept.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(names, vec!["ok_scalar", "ok_enum"]); // prose rule dropped, scalars kept
        assert_eq!(residuals.len(), 1);
        assert_eq!(residuals[0].packet_id, "isf_rule_value_constraint_48");
    }

    fn dir_port(actor: &str, signal: &str, direction: ActorRelativeDirection) -> ActorPortRecord {
        ActorPortRecord {
            actor_id: actor.to_string(),
            actor_name: actor.to_string(),
            signal_name: signal.to_string(),
            direction,
            relation_basis: vec![],
            width_hint: None,
            source_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn select_initiator_actor_picks_the_net_producer() {
        use ActorRelativeDirection::{Input, Output};
        // KG-ISF-COMPLETENESS.2a.ii: the initiator is the net producer (out > in) maximizing
        // (out, in). Synthetic AMBA-shaped graph (no chip-name dependence — these are test fixtures):
        // an initiator that drives the request + reads the response, a completer that mirrors it, a
        // balanced prose-fragment actor (excluded by out > in), and an output-only decoy that ties on
        // output count but loses the (out, in) tiebreak because it reads nothing.
        let ports = vec![
            dir_port("Init", "REQA", Output),
            dir_port("Init", "REQB", Output),
            dir_port("Init", "REQC", Output),
            dir_port("Init", "RESP", Input), // initiator also reads the response
            dir_port("Comp", "REQA", Input),
            dir_port("Comp", "REQB", Input),
            dir_port("Comp", "REQC", Input),
            dir_port("Comp", "RESP", Output), // completer: input-dominant, not a net producer
            dir_port("fragment", "REQA", Output), // balanced (out=in=1) → excluded
            dir_port("fragment", "REQA", Input),
            dir_port("Decoy", "REQA", Output), // out=3/in=0 → net producer, ties Init on out
            dir_port("Decoy", "REQB", Output),
            dir_port("Decoy", "REQC", Output),
        ];
        // Init (3,1) beats Decoy (3,0) on the (out, in) tiebreak; Comp/fragment are not net producers.
        assert_eq!(select_initiator_actor(&ports).as_deref(), Some("Init"));
        let exact_tie = vec![
            dir_port("Alpha", "REQA", Output),
            dir_port("Alpha", "REQB", Output),
            dir_port("Zulu", "REQC", Output),
            dir_port("Zulu", "REQD", Output),
        ];
        assert_eq!(
            select_initiator_actor(&exact_tie).as_deref(),
            Some("Zulu"),
            "equal producer maxima resolve to the lexicographically last actor"
        );
        // No net producer → None (honest residual: the interface keeps its default-output behavior).
        let no_producer = vec![
            dir_port("Reader", "RESP", Input),
            dir_port("Balanced", "X", Output),
            dir_port("Balanced", "X", Input),
        ];
        assert_eq!(select_initiator_actor(&no_producer), None);
    }

    #[test]
    fn initiator_perspective_directions_are_grounded_and_residual_safe() {
        use ActorRelativeDirection::{InOut, Input, Output};
        // The initiator's grounded direction per signal: Drives → output, Reads → input. A signal it
        // both drives and reads is OMITTED (the caller keeps the honest default, never a guess); an
        // InOut/Unknown touch is OMITTED; another actor's ports are ignored.
        let ports = vec![
            dir_port("Init", "REQ", Output),
            dir_port("Init", "RESP", Input),
            dir_port("Init", "AMBIG", Output),
            dir_port("Init", "AMBIG", Input), // conflicting → omitted
            dir_port("Init", "SIDE", InOut),  // InOut → omitted
            dir_port("Comp", "REQ", Input),   // other actor → ignored
        ];
        let dirs = initiator_perspective_directions(&ports, "Init");
        assert_eq!(dirs.get("REQ"), Some(&IsfDirection::Output));
        assert_eq!(dirs.get("RESP"), Some(&IsfDirection::Input));
        assert_eq!(dirs.get("AMBIG"), None);
        assert_eq!(dirs.get("SIDE"), None);
        assert_eq!(dirs.len(), 2);
    }

    #[test]
    fn a_register_width_the_document_does_not_state_is_listed_as_the_adapters_choice() {
        // EXTRACTION-REPRESENTATION-GAPS.41.b — ADIv6's `ABORT` (fields to bit 4) and twelve more.
        let names = std::iter::once("ABORT".to_string())
            .chain((1..=13).map(|index| format!("REG{index}")))
            .collect::<Vec<_>>();
        let packet = register_width_chosen_residual_packet(&names);
        assert_eq!(packet.packet_id, "isf_register_width_not_stated");
        assert!(packet.question.starts_with("14 register storage width(s)"));
        assert!(packet.why_unresolved.contains("ABORT, REG1"));
        assert!(packet.why_unresolved.contains("and 2 more"));
        assert!(
            packet
                .why_unresolved
                .contains("The width is the adapter's choice")
        );
    }

    #[test]
    fn register_var_width_uses_true_register_width() {
        // Two 16-bit fields → register is 32 bits wide, not the 16-bit max-field-extent.
        let two16 = vec![
            reset_field(31, 16, Some("0")),
            reset_field(15, 0, Some("0")),
        ];
        assert_eq!(register_var_width(&register_with(None, two16)), 32);
        // Declared size_bits wins when it is wider than the located fields (reserved top bits).
        let one_field = vec![reset_field(3, 0, Some("0"))];
        assert_eq!(register_var_width(&register_with(Some(32), one_field)), 32);
        // Never truncate a field: size_bits below the highest field bit is lifted to field_top.
        let wide_field = vec![reset_field(23, 0, Some("0"))];
        assert_eq!(register_var_width(&register_with(Some(8), wide_field)), 24);
        // No located fields, no size → the 32-bit fallback (unchanged behavior).
        assert_eq!(register_var_width(&register_with(None, vec![])), 32);
        // size_bits with no located fields → the declared width.
        assert_eq!(register_var_width(&register_with(Some(64), vec![])), 64);
    }

    #[test]
    fn register_reset_emits_once_var_width_is_the_register_width() {
        // A 32-bit register of two 16-bit fields whose composed reset (0x1234_0000) needs >16 bits:
        // under the true register width it is now emittable (was DeferredWidth under max-field-extent).
        let fields = vec![
            reset_field(31, 16, Some("0x1234")),
            reset_field(15, 0, Some("0")),
        ];
        let r = register_with(None, fields);
        let w = register_var_width(&r); // 32
        assert_eq!(w, 32);
        assert!(matches!(
            classify_register_reset(&r.fields, w),
            RegisterResetOutcome::Emit(0x1234_0000)
        ));
        // Under the OLD max-field-extent width (16) it would have been over-width.
        assert!(matches!(
            classify_register_reset(&r.fields, 16),
            RegisterResetOutcome::DeferredWidth
        ));
    }

    #[test]
    fn a_register_stated_reset_lowers_alone_or_in_agreement_with_its_fields() {
        // EXTRACTION-GAP-FIX.8 — CoreSight TMC's register summary states `STS` resets to `0x0000000C`
        // while its bit table states no field reset; `RSZ` states `0x00000080 b`, qualified by a note.
        let stated = |reset: &str, fields: Vec<RegisterFieldRecord>| {
            let mut register = register_with(Some(32), fields);
            register.reset_value = Some(reset.to_string());
            classify_stated_register_reset(&register, 32)
        };
        assert!(
            classify_stated_register_reset(&register_with(Some(32), vec![]), 32).is_none(),
            "a register that states no reset leaves its fields to decide"
        );
        assert!(matches!(
            stated("0x0000000C", vec![reset_field(2, 2, None)]),
            Some(RegisterResetOutcome::Emit(12))
        ));
        assert!(matches!(
            stated("0x00000000", vec![]),
            Some(RegisterResetOutcome::DefaultZero)
        ));
        assert!(matches!(
            stated("0x00000080 b", vec![]),
            Some(RegisterResetOutcome::NotLowerable)
        ));
        // with field resets, the stated value must be what they compose
        assert!(matches!(
            stated(
                "0x12",
                vec![reset_field(7, 4, Some("1")), reset_field(3, 0, Some("2"))]
            ),
            Some(RegisterResetOutcome::Emit(0x12))
        ));
        assert!(matches!(
            stated(
                "0x13",
                vec![reset_field(7, 4, Some("1")), reset_field(3, 0, Some("2"))]
            ),
            Some(RegisterResetOutcome::NotLowerable)
        ));
        assert!(matches!(
            stated("0x12", vec![reset_field(7, 4, Some("UNKNOWN"))]),
            Some(RegisterResetOutcome::NotLowerable)
        ));
        let mut narrow = register_with(Some(8), vec![]);
        narrow.reset_value = Some("0x100".to_string());
        assert!(matches!(
            classify_stated_register_reset(&narrow, 8),
            Some(RegisterResetOutcome::DeferredWidth)
        ));
        let packet = stated_register_reset_residual_packet(2);
        assert_eq!(packet.packet_id, "isf_stated_register_reset_not_lowered");
        assert!(packet.question.starts_with("2 register(s)"));
    }

    #[test]
    fn render_storage_var_emits_reset_only_when_set() {
        let mut isf = minimal_isf();
        isf.storage.push(IsfStorageVar {
            name: "dpidr".to_string(),
            width: 32,
            reset: Some(0x1c01_3477),
            fields: vec![],
        });
        isf.storage.push(IsfStorageVar {
            name: "plain".to_string(),
            width: 8,
            reset: None,
            fields: vec![],
        });
        let out = isf.render();
        assert!(out.contains(&format!(
            "    (var dpidr (width 32) (reset {}))",
            0x1c01_3477u64
        )));
        assert!(out.contains("    (var plain (width 8))"));
        assert!(!out.contains("(var plain (width 8) (reset"));
        assert_eq!(paren_balance(&out), 0);
    }

    #[test]
    fn storage_reset_residual_packet_summarizes_both_reasons() {
        let p = storage_reset_residual_packet(3, 2);
        assert_eq!(p.packet_id, "isf_storage_reset_not_lowered");
        assert!(p.question.contains('5'));
        assert!(p.why_unresolved.contains("ISF-REGISTER-RESET-EMIT.3"));
    }

    // --- DOC-INTENT-TAXONOMY.4a.ii: register bit-field storage lowering ----

    fn field_rec(
        name: &str,
        hi: u32,
        lo: u32,
        access: Option<&str>,
        reset: Option<&str>,
        enums: &[(&str, &str)],
    ) -> RegisterFieldRecord {
        RegisterFieldRecord {
            field_name: name.to_string(),
            bits_high: Some(hi),
            bits_low: Some(lo),
            bit_width: None,
            access_type: access.map(str::to_string),
            reset_value: reset.map(str::to_string),
            description: None,
            enumerated_values: enums
                .iter()
                .map(|(v, m)| crate::ir::source::RegisterFieldEnumRecord {
                    value: v.to_string(),
                    meaning: m.to_string(),
                })
                .collect(),
            signal_bindings: Vec::new(),
        }
    }

    #[test]
    fn normalize_field_access_maps_known_tokens_and_omits_unknown() {
        for (raw, want) in [
            ("RO", "ro"),
            ("rw", "rw"),
            ("WO", "wo"),
            ("WARL", "warl"),
            ("WPRI", "wpri"),
            ("W1C", "w1c"),
            ("RW1C", "w1c"),
            ("R", "ro"),
            ("R/W", "rw"),
            ("W", "wo"),
            ("Reserved", "reserved"),
        ] {
            assert_eq!(
                normalize_field_access(Some(raw)).as_deref(),
                Some(want),
                "raw={raw}"
            );
        }
        // Unmappable notations are omitted (None), never guessed (ADR-0006 honest residual).
        for raw in ["RsvdP", "RsvdZ", "-", "…", "HwInit", "RWS/RW", "I", "X", ""] {
            assert_eq!(normalize_field_access(Some(raw)), None, "raw={raw:?}");
        }
        assert_eq!(normalize_field_access(None), None);
    }

    #[test]
    fn register_storage_fields_admits_located_unique_nonoverlapping() {
        // Three located, unique, non-overlapping fields whose numeric resets compose to V=0xA5
        // (mode=0b101@5 | prio=0b001@2 | enable=1@0 == 1010_0101). The field (reset)s are the
        // parent V's own bit slices, so they match by construction.
        let r = register_with(
            Some(8),
            vec![
                field_rec(
                    "mode",
                    7,
                    5,
                    Some("RW"),
                    Some("0x5"),
                    &[("0", "Idle"), ("5", "Run")],
                ),
                field_rec("prio", 4, 2, Some("RW"), Some("1"), &[]),
                field_rec(
                    "enable",
                    0,
                    0,
                    Some("RW"),
                    Some("1"),
                    &[("0", "Off"), ("1", "On")],
                ),
            ],
        );
        let parent = match classify_register_reset(&r.fields, 8) {
            RegisterResetOutcome::Emit(v) => Some(v),
            _ => panic!("expected RegisterResetOutcome::Emit"),
        };
        assert_eq!(parent, Some(0xA5));
        let (fields, not_lowered) = register_storage_fields(&r, 8, parent);
        assert_eq!(not_lowered, 0);
        assert_eq!(fields.len(), 3);
        let mode = &fields[0];
        assert_eq!((mode.msb, mode.lsb), (7, 5));
        assert_eq!(mode.access.as_deref(), Some("rw"));
        assert_eq!(mode.reset, Some(5)); // slice [7:5] of 0xA5 == 0b101 == 5
        assert_eq!(
            mode.enum_members,
            vec![("idle".to_string(), 0), ("run".to_string(), 5)]
        );
    }

    #[test]
    fn register_storage_fields_drops_unlocated_collision_and_failscloses_overlap() {
        // Unlocated field → honest gap (residual); the located unique field is kept, and with no
        // parent reset no field (reset) is emitted (FSMGen requires an explicit parent reset).
        let mut unlocated = field_rec("rsvd", 0, 0, None, None, &[]);
        unlocated.bits_high = None;
        unlocated.bits_low = None;
        let r = register_with(
            Some(8),
            vec![field_rec("data", 7, 1, Some("RO"), None, &[]), unlocated],
        );
        let (fields, not_lowered) = register_storage_fields(&r, 8, None);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "data");
        assert_eq!(fields[0].reset, None);
        assert_eq!(not_lowered, 1);

        // Sanitized-name collision (two `res0`, non-overlapping) → BOTH dropped; `ctrl` survives.
        let r = register_with(
            Some(8),
            vec![
                field_rec("RES0", 7, 7, None, None, &[]),
                field_rec("ctrl", 5, 0, Some("RW"), None, &[]),
                field_rec("res0", 6, 6, None, None, &[]),
            ],
        );
        let (fields, not_lowered) = register_storage_fields(&r, 8, None);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].name, "ctrl");
        assert_eq!(not_lowered, 2);

        // Overlapping located fields → the whole register's field block fails closed.
        let r = register_with(
            Some(8),
            vec![
                field_rec("a", 5, 2, Some("RW"), None, &[]),
                field_rec("b", 4, 0, Some("RW"), None, &[]),
            ],
        );
        let (fields, not_lowered) = register_storage_fields(&r, 8, None);
        assert!(fields.is_empty());
        assert_eq!(not_lowered, 2);
    }

    #[test]
    fn register_storage_fields_drops_overwide_and_nonnumeric_enum_members() {
        // A 1-bit field: enum value 2 does not fit, "X" is non-numeric → both dropped; 1 kept.
        let r = register_with(
            Some(4),
            vec![field_rec(
                "en",
                0,
                0,
                Some("RW"),
                None,
                &[("1", "On"), ("2", "Bad"), ("X", "Sym")],
            )],
        );
        let (fields, _n) = register_storage_fields(&r, 4, None);
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].enum_members, vec![("on".to_string(), 1)]);
    }

    #[test]
    fn render_storage_var_emits_fields_block() {
        let mut isf = minimal_isf();
        isf.storage.push(IsfStorageVar {
            name: "control".to_string(),
            width: 8,
            reset: Some(0xA1),
            fields: vec![
                IsfStorageField {
                    name: "mode".to_string(),
                    msb: 7,
                    lsb: 5,
                    access: Some("rw".to_string()),
                    reset: Some(5),
                    enum_members: vec![("idle".to_string(), 0), ("run".to_string(), 5)],
                },
                IsfStorageField {
                    name: "prio".to_string(),
                    msb: 4,
                    lsb: 2,
                    access: Some("rw".to_string()),
                    reset: None,
                    enum_members: vec![],
                },
            ],
        });
        // An opaque var (no fields) stays single-line — byte-identical to pre-`.4a.ii`.
        isf.storage.push(IsfStorageVar {
            name: "plain".to_string(),
            width: 8,
            reset: None,
            fields: vec![],
        });
        let out = isf.render();
        assert!(
            out.contains("    (var control (width 8) (reset 161)"),
            "{out}"
        );
        assert!(out.contains("      (fields"), "{out}");
        assert!(
            out.contains(
                "        (field mode (bits 7 5) (access rw) (reset 5) (enum (idle 0) (run 5)))"
            ),
            "{out}"
        );
        assert!(
            out.contains("        (field prio (bits 4 2) (access rw))"),
            "{out}"
        );
        assert!(out.contains("    (var plain (width 8))"), "{out}");
        assert!(!out.contains("(var plain (width 8) (reset"), "{out}");
        assert_eq!(paren_balance(&out), 0);
    }

    #[test]
    fn register_fields_pass_fsmgen_strict_and_round_trip() {
        // DOC-INTENT-TAXONOMY.4a.ii: a fields-bearing storage var must (a) pass the real
        // fsmgen --strict --check and (b) round-trip through --emit-schedule-json
        // inferred_storage[].fields[] (name/msb/lsb/width/access/reset/enum). Built on a
        // known-fsmgen-valid base actor.
        let mut isf = isf_with_temporal_contract_transaction();
        isf.storage.push(IsfStorageVar {
            name: "control".to_string(),
            width: 8,
            reset: Some(161),
            fields: vec![
                IsfStorageField {
                    name: "mode".to_string(),
                    msb: 7,
                    lsb: 5,
                    access: Some("rw".to_string()),
                    reset: Some(5),
                    enum_members: vec![("idle".to_string(), 0), ("run".to_string(), 5)],
                },
                IsfStorageField {
                    name: "prio".to_string(),
                    msb: 4,
                    lsb: 2,
                    access: Some("rw".to_string()),
                    reset: None,
                    enum_members: vec![],
                },
                IsfStorageField {
                    name: "enable".to_string(),
                    msb: 0,
                    lsb: 0,
                    access: Some("rw".to_string()),
                    reset: Some(1),
                    enum_members: vec![("off".to_string(), 0), ("on".to_string(), 1)],
                },
            ],
        });
        let tempdir = tempfile::tempdir().expect("tempdir");
        let src = isf.render();
        let isf_path = tempdir.path().join("register_fields.isf");
        std::fs::write(&isf_path, &src).expect("write isf");
        eprintln!("=== ISF ===\n{src}\n=== END ===");

        // (a) strict check passes (the field block is accepted).
        let output = crate::ir::run_fsmgen_strict_check(&isf_path);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let check: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!("fsmgen non-JSON.\nstdout:{stdout}\nstderr:{stderr}\nerr:{e}")
        });
        if !check["diagnostic_summary"]["success"]
            .as_bool()
            .unwrap_or(false)
            && let Some(diags) = check["diagnostics"].as_array()
        {
            for d in diags {
                eprintln!(
                    "FSMGen diagnostic: {}",
                    d.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("(none)")
                );
            }
        }
        assert!(
            check["diagnostic_summary"]["success"]
                .as_bool()
                .unwrap_or(false),
            "FSMGen strict rejected the (fields …) block:\n{src}"
        );

        // (b) inferred_storage[].fields[] round-trips the emitted field map.
        let sched = crate::ir::run_fsmgen_schedule_json(&isf_path);
        let sjson: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&sched.stdout)).expect("schedule json");
        let control = sjson["inferred_storage"]
            .as_array()
            .expect("inferred_storage")
            .iter()
            .find(|s| s["name"] == "control")
            .expect("control storage entry");
        let fields = control["fields"].as_array().expect("fields");
        assert_eq!(fields.len(), 3, "{control}");
        let mode = fields
            .iter()
            .find(|f| f["name"] == "mode")
            .expect("mode field");
        assert_eq!(mode["msb"], 7);
        assert_eq!(mode["lsb"], 5);
        assert_eq!(mode["width"], 3);
        assert_eq!(mode["access"], "rw");
        assert_eq!(mode["reset"], 5);
        assert_eq!(mode["enum"].as_array().expect("enum").len(), 2);
    }

    #[test]
    fn render_transaction_nests_when_switch_and_default_on_start() {
        let mut isf = minimal_isf();
        isf.transactions.push(IsfTransaction {
            name: "t1".to_string(),
            on_trigger: None,
            on_steps: vec![],
            steps: vec![
                IsfTxnStep::When {
                    condition: "ready".to_string(),
                    body: vec![IsfTxnStep::Drive {
                        name: "out".to_string(),
                        actuals: vec!["1".to_string()],
                    }],
                },
                IsfTxnStep::Switch {
                    selector: "mode".to_string(),
                    branches: vec![(
                        "fast".to_string(),
                        vec![IsfTxnStep::Wait {
                            count: "2".to_string(),
                        }],
                    )],
                },
            ],
            complete: "done".to_string(),
            contracts: Vec::new(),
            stages: Vec::new(),
            latency_min: Some(1),
            latency_max: Some(4),
        });

        let out = isf.render();
        assert!(out.contains("  (transaction t1"));
        assert!(out.contains("    (on start)"), "default on-start:\n{out}");
        assert!(out.contains("    (when ready"));
        assert!(out.contains("      (drive out 1)"));
        assert!(out.contains("    (switch mode"));
        assert!(out.contains("      (fast)"));
        assert!(out.contains("        (wait 2)"));
        assert!(out.contains("    (complete done)"));
        assert!(out.contains("    (latency (min 1) (max 4))"));
        assert_eq!(paren_balance(&out), 0, "unbalanced:\n{out}");
    }

    // --- ISF-TEMPORAL-LOWERING.2.1: transaction-internal bounded contract ---

    fn isf_with_temporal_contract_transaction() -> IsfIr {
        let mut isf = minimal_isf();
        isf.signals.insert(IsfSignal {
            name: "RVALID".to_string(),
            direction: IsfDirection::Output,
            width: 1,
            width_param: None,
        });
        isf.signals.insert(IsfSignal {
            name: "RREADY".to_string(),
            direction: IsfDirection::Input,
            width: 1,
            width_param: None,
        });
        isf.transactions.push(IsfTransaction {
            name: "t_temporal".to_string(),
            on_trigger: None,
            on_steps: vec![],
            steps: vec![IsfTxnStep::Await {
                port: "RREADY".to_string(),
                watchdog: None,
            }],
            complete: "done".to_string(),
            latency_min: None,
            latency_max: None,
            contracts: vec![IsfContract {
                prop: "(monitor (within RVALID 4))".to_string(),
            }],
            stages: vec![],
        });
        isf
    }

    #[test]
    fn render_emits_assert_monitor_bounded_eventually() {
        let out = isf_with_temporal_contract_transaction().render();
        // FSMGen verification family (pin 43b29f5c): a bounded-eventually lowers to
        // `(assert (monitor (within s N)))`; the `(contract … (eventually …))` clause
        // was removed (FSMGEN-ASSERT-MIGRATE).
        assert!(
            out.contains("    (assert (monitor (within RVALID 4)))"),
            "assert-monitor shape:\n{out}"
        );
        assert_eq!(paren_balance(&out), 0, "unbalanced:\n{out}");
    }

    #[test]
    fn bounded_contract_passes_fsmgen_strict_validation() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let out = isf_with_temporal_contract_transaction().render();
        let isf_path = tempdir.path().join("temporal_barrier.isf");
        std::fs::write(&isf_path, &out).expect("write isf");
        eprintln!("=== ISF ===\n{out}\n=== END ===");

        let output = crate::ir::run_fsmgen_strict_check(&isf_path);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let check: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!("fsmgen non-JSON.\nstdout:{stdout}\nstderr:{stderr}\nerr:{e}")
        });
        let success = check["diagnostic_summary"]["success"]
            .as_bool()
            .unwrap_or(false);
        if !success && let Some(diags) = check["diagnostics"].as_array() {
            for d in diags {
                eprintln!(
                    "FSMGen diagnostic: {}",
                    d.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("(none)")
                );
            }
        }
        assert!(
            success,
            "FSMGen strict rejected the spec-shaped (stage …)/(assert (monitor …)) forms"
        );
    }

    // --- ISF-TEMPORAL-LOWERING.2.3: classify_temporal_rule + residual ---

    use crate::ir::semantic::{ClockEdge, CycleWindowRecord, TickPhase};

    fn t_rule(
        rule_id: &str,
        antecedents: Vec<TemporalPredicateRecord>,
        consequents: Vec<TemporalPredicateRecord>,
        cycle_window: Option<CycleWindowRecord>,
    ) -> TemporalRuleRecord {
        TemporalRuleRecord {
            rule_id: rule_id.to_string(),
            clock_signal: None,
            edge: ClockEdge::Rising,
            antecedents,
            exceptions: Vec::new(),
            consequents,
            cycle_window,
            source_text: format!("source for {rule_id}"),
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    fn sigval(name: &str, value: &str) -> TemporalPredicateRecord {
        TemporalPredicateRecord::SignalValue {
            signal_name: name.to_string(),
            value: value.to_string(),
            phase: TickPhase::PostTick,
        }
    }

    fn declared(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn the_contract_states_the_reset_polarity_unless_a_record_disagrees() {
        // EXTRACTION-REPRESENTATION-GAPS.7.b
        use crate::ir::semantic::{
            SystemContractRecord, SystemResetKind, SystemResetPolarity, SystemResetTargetKind,
            SystemResetTimingRelation,
        };
        let contract = |polarity| SystemContractRecord {
            clock_signal: "clk".to_string(),
            reset_signal: "rst".to_string(),
            reset_kind: SystemResetKind::Asynchronous,
            reset_polarity: polarity,
            assertion_timing: SystemResetTimingRelation::AsynchronousToClock,
            release_timing: SystemResetTimingRelation::SynchronousToClock,
            target_kind: SystemResetTargetKind::DedicatedResetPin,
            supporting_statement_ids: vec![],
            automation_confidence: AutomationConfidence::High,
        };
        let recorded = |polarity| BTreeMap::from([("rst".to_string(), polarity)]);
        let low = contract(SystemResetPolarity::ActiveLow);
        assert_eq!(
            with_reset_polarity(BTreeMap::new(), Some(&low)),
            recorded(SignalPolarity::ActiveLow)
        );
        assert_eq!(
            with_reset_polarity(recorded(SignalPolarity::ActiveLow), Some(&low)),
            recorded(SignalPolarity::ActiveLow)
        );
        // A record that disagrees leaves the polarity unknown: refused, never chosen.
        assert!(with_reset_polarity(recorded(SignalPolarity::ActiveHigh), Some(&low)).is_empty());
        let unknown = contract(SystemResetPolarity::Unknown);
        assert!(with_reset_polarity(BTreeMap::new(), Some(&unknown)).is_empty());
        assert!(with_reset_polarity(BTreeMap::new(), None).is_empty());
    }

    // R16-CONTRACT-IR.3 PARITY GATE (by construction): for every rule
    // shape, `classify_actor_contract(contract_from_temporal_rule(r), &BTreeMap::new())`
    // must yield the SAME disposition as `classify_temporal_rule(r, &BTreeMap::new())` —
    // Contract/Rule fully equal (drives the emitted `.isf`); Residual
    // same `rule_id` (reason wording is internal `adapter.json` metadata
    // per the recorded `.3` parity definition).
    #[test]
    fn classify_actor_contract_is_parity_equivalent_to_classify_temporal_rule() {
        use crate::ir::contract::contract_from_temporal_rule;

        let hs = |v: &str, r: &str| TemporalPredicateRecord::HandshakeComplete {
            valid_signal: v.to_string(),
            ready_signal: r.to_string(),
            phase: TickPhase::PostTick,
        };
        let stable = |s: &str| TemporalPredicateRecord::SignalStable {
            signal_name: s.to_string(),
            from_phase: TickPhase::PreTick,
            to_phase: TickPhase::PostTick,
        };
        let drives = |s: &str| TemporalPredicateRecord::ActorDrivesSignal {
            actor_name: "M".to_string(),
            signal_name: s.to_string(),
            phase: TickPhase::PostTick,
        };
        let win = |max: u32| {
            Some(CycleWindowRecord {
                min_cycles: None,
                max_cycles: Some(max),
            })
        };

        let sigs = declared(&["ACK", "GRANT", "SEL", "ADDR"]);
        let cases: Vec<TemporalRuleRecord> = vec![
            t_rule("w_sv_decl", vec![], vec![sigval("ACK", "1")], win(4)),
            t_rule("w_sv_undecl", vec![], vec![sigval("MISS", "1")], win(4)),
            t_rule("w_sv_zero", vec![], vec![sigval("ACK", "1")], win(0)),
            t_rule("w_stable_decl", vec![], vec![stable("ADDR")], win(2)),
            t_rule("w_hs", vec![], vec![hs("V", "R")], win(3)),
            t_rule(
                "nw_sv_guarded",
                vec![sigval("SEL", "1")],
                vec![sigval("GRANT", "1")],
                None,
            ),
            t_rule(
                "nw_sv_nonlit",
                vec![],
                vec![sigval("GRANT", "addr+4")],
                None,
            ),
            t_rule("nw_sv_undecl", vec![], vec![sigval("MISS", "1")], None),
            t_rule("nw_hs", vec![], vec![hs("AWVALID", "AWREADY")], None),
            t_rule("nw_stable", vec![], vec![stable("ADDR")], None),
            t_rule("nw_drives", vec![], vec![drives("GRANT")], None),
            // multi-antecedent: first SignalValue undeclared, second
            // declared+literal — guard must select the second.
            t_rule(
                "nw_multi_ante",
                vec![sigval("MISS", "1"), sigval("SEL", "1")],
                vec![sigval("GRANT", "1")],
                None,
            ),
        ];

        for r in &cases {
            let oracle = classify_temporal_rule(r, &sigs, &BTreeMap::new());
            let via =
                classify_actor_contract(&contract_from_temporal_rule(r), &sigs, &BTreeMap::new());
            match (&oracle, &via) {
                (
                    TemporalRuleDisposition::Residual { rule_id: a, .. },
                    TemporalRuleDisposition::Residual { rule_id: b, .. },
                ) => assert_eq!(a, b, "residual rule_id mismatch for {}", r.rule_id),
                _ => assert_eq!(oracle, via, "disposition mismatch for rule '{}'", r.rule_id),
            }
        }
    }

    #[test]
    fn classify_non_windowed_signalvalue_with_guard_is_rule() {
        let rule = t_rule(
            "r_guarded",
            vec![sigval("SEL", "1")],
            vec![sigval("GRANT", "1")],
            None,
        );
        let d = classify_temporal_rule(&rule, &declared(&["SEL", "GRANT"]), &BTreeMap::new());
        assert_eq!(
            d,
            TemporalRuleDisposition::Rule {
                name: "temporal_r_guarded".to_string(),
                // exact declared signal name, never lowercased
                condition: "(== SEL 1)".to_string(),
                signal: "GRANT".to_string(),
                value: "1".to_string(),
            }
        );
    }

    #[test]
    fn classify_non_windowed_no_antecedent_is_unconditional_rule() {
        let rule = t_rule("r_uncond", vec![], vec![sigval("GRANT", "low")], None);
        let d = classify_temporal_rule(&rule, &declared(&["GRANT"]), &BTreeMap::new());
        assert_eq!(
            d,
            TemporalRuleDisposition::Rule {
                name: "temporal_r_uncond".to_string(),
                condition: String::new(), // strict-verified: conditionless rule
                signal: "GRANT".to_string(),
                value: "0".to_string(), // "low" normalized
            }
        );
    }

    #[test]
    fn an_unguarded_window_has_no_event_to_count_from() {
        // EXTRACTION-REPRESENTATION-GAPS.29 — this once lowered to `(monitor (within RVALID 8))`, which arms at the
        // synthetic transaction's `(on start)`: a window counted from the start of the run, which no sentence states.
        // APB-E's windows are diagram positions (*occurs at T1 in Figure 3-1*), counted from the figure's own origin.
        use crate::ir::contract::contract_from_temporal_rule;
        let rule = t_rule(
            "r_win",
            vec![],
            vec![sigval("RVALID", "1")],
            Some(CycleWindowRecord {
                min_cycles: None,
                max_cycles: Some(8),
            }),
        );
        let declared_signals = declared(&["RVALID"]);
        for disposition in [
            classify_temporal_rule(&rule, &declared_signals, &BTreeMap::new()),
            classify_actor_contract(
                &contract_from_temporal_rule(&rule),
                &declared_signals,
                &BTreeMap::new(),
            ),
        ] {
            assert!(
                matches!(&disposition, TemporalRuleDisposition::Residual { reason, .. }
                    if reason.contains("no antecedent to count its window from")),
                "{disposition:?}"
            );
        }
    }

    // FSMGEN-ASSERT-LOWERING.3: guarded windowed-eventuals keep their antecedent.

    #[test]
    fn classify_guarded_windowed_eventual_is_implication() {
        // A windowed eventual WITH a representable antecedent → `(=> g (within s max))`
        // (the monitor would have dropped the antecedent).
        let rule = t_rule(
            "g_ack",
            vec![sigval("PENABLE", "1")],
            vec![sigval("PREADY", "1")],
            Some(CycleWindowRecord {
                min_cycles: None,
                max_cycles: Some(4),
            }),
        );
        let d = classify_temporal_rule(&rule, &declared(&["PENABLE", "PREADY"]), &BTreeMap::new());
        assert_eq!(
            d,
            TemporalRuleDisposition::Contract {
                name: "g_ack".to_string(),
                prop: "(=> (== PENABLE 1) (within PREADY 4))".to_string(),
            }
        );
    }

    #[test]
    fn classify_guarded_windowed_eventual_min_gt_1_emits_range() {
        // A lower bound > 1 emits the two-operand window `(within s MIN MAX)`.
        let rule = t_rule(
            "g_min",
            vec![sigval("PENABLE", "1")],
            vec![sigval("PREADY", "1")],
            Some(CycleWindowRecord {
                min_cycles: Some(2),
                max_cycles: Some(5),
            }),
        );
        let d = classify_temporal_rule(&rule, &declared(&["PENABLE", "PREADY"]), &BTreeMap::new());
        assert_eq!(
            d,
            TemporalRuleDisposition::Contract {
                name: "g_min".to_string(),
                prop: "(=> (== PENABLE 1) (within PREADY 2 5))".to_string(),
            }
        );
    }

    #[test]
    fn classify_guarded_windowed_eventual_min_zero_is_residual() {
        // A guarded 0 lower bound has no `|-> ##[0:N]` ISF spelling → residual
        // (FSMGEN-MIN-WINDOW-CONFIRM), never a fabricated `(within B 0 MAX)`.
        let rule = t_rule(
            "g_zero",
            vec![sigval("PENABLE", "1")],
            vec![sigval("PREADY", "1")],
            Some(CycleWindowRecord {
                min_cycles: Some(0),
                max_cycles: Some(5),
            }),
        );
        let d = classify_temporal_rule(&rule, &declared(&["PENABLE", "PREADY"]), &BTreeMap::new());
        assert!(
            matches!(d, TemporalRuleDisposition::Residual { .. }),
            "{d:?}"
        );
    }

    #[test]
    fn guarded_windowed_eventual_passes_fsmgen_strict_validation() {
        // End-to-end: the guarded implication form is strict-valid on the pinned
        // binary (RREADY/RVALID are declared by the fixture).
        let mut isf = isf_with_temporal_contract_transaction();
        isf.transactions[0].contracts = vec![IsfContract {
            prop: "(=> RREADY (within RVALID 2 5))".to_string(),
        }];
        let out = isf.render();
        let tempdir = tempfile::tempdir().expect("tempdir");
        let isf_path = tempdir.path().join("guarded_eventual.isf");
        std::fs::write(&isf_path, &out).expect("write isf");

        let output = crate::ir::run_fsmgen_strict_check(&isf_path);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let check: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!("fsmgen non-JSON.\nstdout:{stdout}\nstderr:{stderr}\nerr:{e}")
        });
        let success = check["diagnostic_summary"]["success"]
            .as_bool()
            .unwrap_or(false);
        assert!(
            success,
            "guarded `(=> g (within s MIN MAX))` rejected by fsmgen strict:\n{out}\n{stdout}"
        );
    }

    #[test]
    fn classify_window_zero_is_residual_not_within_zero() {
        let rule = t_rule(
            "r_w0",
            vec![],
            vec![sigval("RVALID", "1")],
            Some(CycleWindowRecord {
                min_cycles: None,
                max_cycles: Some(0),
            }),
        );
        match classify_temporal_rule(&rule, &declared(&["RVALID"]), &BTreeMap::new()) {
            TemporalRuleDisposition::Residual { rule_id, reason } => {
                assert_eq!(rule_id, "r_w0");
                assert!(reason.contains("(within 0)"), "reason: {reason}");
            }
            other => panic!("expected Residual, got {other:?}"),
        }
    }

    #[test]
    fn classify_handshake_complete_is_residual() {
        let rule = t_rule(
            "r_hs",
            vec![],
            vec![TemporalPredicateRecord::HandshakeComplete {
                valid_signal: "AWVALID".to_string(),
                ready_signal: "AWREADY".to_string(),
                phase: TickPhase::PostTick,
            }],
            None,
        );
        match classify_temporal_rule(&rule, &declared(&["AWVALID", "AWREADY"]), &BTreeMap::new()) {
            TemporalRuleDisposition::Residual { rule_id, .. } => assert_eq!(rule_id, "r_hs"),
            other => panic!("expected Residual, got {other:?}"),
        }
    }

    #[test]
    fn classify_undeclared_signal_and_nonliteral_value_are_residual() {
        let undeclared = t_rule("r_ud", vec![], vec![sigval("MISSING", "1")], None);
        assert!(matches!(
            classify_temporal_rule(&undeclared, &declared(&["OTHER"]), &BTreeMap::new()),
            TemporalRuleDisposition::Residual { .. }
        ));
        let nonlit = t_rule("r_nl", vec![], vec![sigval("GRANT", "addr+4")], None);
        assert!(matches!(
            classify_temporal_rule(&nonlit, &declared(&["GRANT"]), &BTreeMap::new()),
            TemporalRuleDisposition::Residual { .. }
        ));
    }

    #[test]
    fn classify_signal_naming_consequent_without_value_is_residual() {
        // ActorDrivesSignal names a signal but carries no concrete value —
        // fabricating `(GRANT 1)` would invent semantics → residual.
        let rule = t_rule(
            "r_drv",
            vec![],
            vec![TemporalPredicateRecord::ActorDrivesSignal {
                actor_name: "Manager".to_string(),
                signal_name: "GRANT".to_string(),
                phase: TickPhase::PostTick,
            }],
            None,
        );
        assert!(matches!(
            classify_temporal_rule(&rule, &declared(&["GRANT"]), &BTreeMap::new()),
            TemporalRuleDisposition::Residual { .. }
        ));
    }

    #[test]
    fn temporal_residual_packet_is_well_formed() {
        let p = temporal_residual_packet("r X", "no representable construct", "RVALID stable");
        assert_eq!(p.packet_id, "isf_temporal_unrepresentable_r_x");
        assert!(p.question.contains("r X"));
        assert!(p.why_unresolved.contains("no representable construct"));
        assert!(p.why_unresolved.contains("RVALID stable"));
        assert_eq!(p.automation_confidence, AutomationConfidence::Low);
        assert_eq!(p.candidate_interpretations.len(), 2);
    }

    fn isf_with_temporal_rule() -> IsfIr {
        let mut isf = minimal_isf();
        isf.signals.insert(IsfSignal {
            name: "SEL".to_string(),
            direction: IsfDirection::Input,
            width: 1,
            width_param: None,
        });
        isf.signals.insert(IsfSignal {
            name: "GRANT".to_string(),
            direction: IsfDirection::Output,
            width: 1,
            width_param: None,
        });
        isf.rules.push(IsfRule {
            name: "temporal_r_guarded".to_string(),
            condition: "(== SEL 1)".to_string(),
            drives: vec![("GRANT".to_string(), "1".to_string())],
        });
        isf.rules.push(IsfRule {
            name: "temporal_r_uncond".to_string(),
            condition: String::new(),
            drives: vec![("GRANT".to_string(), "1".to_string())],
        });
        isf
    }

    #[test]
    fn temporal_rule_renders_guarded_and_unconditional_forms() {
        let out = isf_with_temporal_rule().render();
        assert!(
            out.contains("  (rule temporal_r_guarded (== SEL 1)"),
            "guarded rule shape:\n{out}"
        );
        assert!(
            out.contains("  (rule temporal_r_uncond\n"),
            "unconditional rule shape:\n{out}"
        );
        assert_eq!(paren_balance(&out), 0, "unbalanced:\n{out}");
    }

    #[test]
    fn temporal_rule_isf_passes_fsmgen_strict_validation() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let out = isf_with_temporal_rule().render();
        let isf_path = tempdir.path().join("temporal_rule.isf");
        std::fs::write(&isf_path, &out).expect("write isf");
        eprintln!("=== ISF ===\n{out}\n=== END ===");

        let output = crate::ir::run_fsmgen_strict_check(&isf_path);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let check: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!("fsmgen non-JSON.\nstdout:{stdout}\nstderr:{stderr}\nerr:{e}")
        });
        let success = check["diagnostic_summary"]["success"]
            .as_bool()
            .unwrap_or(false);
        if !success && let Some(diags) = check["diagnostics"].as_array() {
            for d in diags {
                eprintln!(
                    "FSMGen diagnostic: {}",
                    d.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("(none)")
                );
            }
        }
        assert!(
            success,
            "FSMGen strict rejected the temporal `(rule …)` lowering forms"
        );
    }

    // --- R16-CONTRACT-IR.4: HandshakeBarrier → (stage …) ---

    #[test]
    fn a_disagreeing_fusion_is_listed_not_lowered_as_its_first_member() {
        // EXTRACTION-REPRESENTATION-GAPS.30 — AXI *AWBURST must be INCR* and *AWBURST must be WRAP*, renamed to
        // levels: fused under one empty guard, the contract is marked residual, and lowering it drove the first
        // value. Agreeing sentences still fuse and lower.
        use crate::ir::contract::contract_from_temporal_rule;
        use crate::ir::fusion::merge_cluster;
        let contract = |id: &str, value: &str| {
            contract_from_temporal_rule(&t_rule(id, vec![], vec![sigval("XBURST", value)], None))
        };
        let declared_signals = declared(&["XBURST"]);
        let disagreeing = merge_cluster(&[contract("r1", "HIGH"), contract("r2", "LOW")]);
        assert!(matches!(
            classify_actor_contract(&disagreeing, &declared_signals, &BTreeMap::new()),
            TemporalRuleDisposition::Residual { reason, .. } if reason.contains("disagree")
        ));
        let agreeing = merge_cluster(&[contract("r1", "HIGH"), contract("r3", "HIGH")]);
        assert!(matches!(
            classify_actor_contract(&agreeing, &declared_signals, &BTreeMap::new()),
            TemporalRuleDisposition::Rule { .. }
        ));
    }

    #[test]
    fn a_windowed_event_lowers_only_a_signal_becoming_true() {
        // EXTRACTION-REPRESENTATION-GAPS.7.d — APB-E *… PADDR, PWRITE, and PWDATA must be valid*, renamed, read with a
        // one-cycle window: `(within XADDR 1)` would state that XADDR becomes true, which is not what *valid* says.
        // A level that resolves to 1 lowers; another level, or a word, is listed. Both classifiers agree.
        use crate::ir::contract::contract_from_temporal_rule;
        let window = Some(CycleWindowRecord {
            min_cycles: Some(1),
            max_cycles: Some(1),
        });
        // Guarded, so that the window counts from a stated event (EXTRACTION-REPRESENTATION-GAPS.29).
        let declared_signals = declared(&["XADDR", "XGO"]);
        for (value, lowers) in [("HIGH", true), ("VALID", false), ("LOW", false)] {
            let rule = t_rule(
                "w1",
                vec![sigval("XGO", "HIGH")],
                vec![sigval("XADDR", value)],
                window.clone(),
            );
            let contract = classify_actor_contract(
                &contract_from_temporal_rule(&rule),
                &declared_signals,
                &BTreeMap::new(),
            );
            let temporal = classify_temporal_rule(&rule, &declared_signals, &BTreeMap::new());
            assert_eq!(contract, temporal, "{value}");
            assert_eq!(
                matches!(contract, TemporalRuleDisposition::Contract { .. }),
                lowers,
                "{value}: {contract:?}"
            );
        }
        // EXTRACTION-REPRESENTATION-GAPS.31 — a windowed consequent that states no value (*must remain stable*) is
        // no eventuality either: its value was a default `1`.
        let stable = t_rule(
            "w2",
            vec![],
            vec![TemporalPredicateRecord::SignalStable {
                signal_name: "XADDR".into(),
                from_phase: TickPhase::PreTick,
                to_phase: TickPhase::PostTick,
            }],
            Some(CycleWindowRecord {
                min_cycles: Some(2),
                max_cycles: Some(2),
            }),
        );
        for disposition in [
            classify_actor_contract(
                &contract_from_temporal_rule(&stable),
                &declared_signals,
                &BTreeMap::new(),
            ),
            classify_temporal_rule(&stable, &declared_signals, &BTreeMap::new()),
        ] {
            assert!(
                matches!(disposition, TemporalRuleDisposition::Residual { .. }),
                "{disposition:?}"
            );
        }
    }

    #[test]
    fn classify_actor_contract_declared_handshake_is_stage_undeclared_residual() {
        use crate::ir::contract::contract_from_temporal_rule;
        let hs = t_rule(
            "h1",
            vec![],
            vec![TemporalPredicateRecord::HandshakeComplete {
                valid_signal: "AWVALID".into(),
                ready_signal: "AWREADY".into(),
                phase: TickPhase::PostTick,
            }],
            None,
        );
        let c = contract_from_temporal_rule(&hs);
        // Both signals declared → the deliberate `.4` behaviour change:
        // Stage (was Residual under the `.3` parity oracle).
        assert_eq!(
            classify_actor_contract(&c, &declared(&["AWVALID", "AWREADY"]), &BTreeMap::new()),
            TemporalRuleDisposition::Stage {
                name: "h1".to_string(),
                ready: "AWREADY".to_string(),
                valid: "AWVALID".to_string(),
            }
        );
        // Undeclared signal → still residual (never reference an
        // undeclared signal).
        assert!(matches!(
            classify_actor_contract(&c, &declared(&["AWVALID"]), &BTreeMap::new()),
            TemporalRuleDisposition::Residual { .. }
        ));
    }

    fn isf_with_temporal_stage() -> IsfIr {
        let mut isf = minimal_isf();
        // FSMGen `ready_valid_barrier`: `ready` must be an actor INPUT
        // (the actor samples it); `valid` as an output is accepted
        // (FSMGEN-SUBMODULE-BUMP.1 verified ready=input / valid=output).
        isf.signals.insert(IsfSignal {
            name: "AWVALID".to_string(),
            direction: IsfDirection::Output,
            width: 1,
            width_param: None,
        });
        isf.signals.insert(IsfSignal {
            name: "AWREADY".to_string(),
            direction: IsfDirection::Input,
            width: 1,
            width_param: None,
        });
        isf.transactions.push(IsfTransaction {
            name: "txn_temporal_h1".to_string(),
            on_trigger: None,
            on_steps: vec![],
            steps: vec![],
            complete: "done".to_string(),
            latency_min: None,
            latency_max: None,
            contracts: vec![],
            stages: vec![IsfStage {
                name: "stage_h1".to_string(),
                ready: "AWREADY".to_string(),
                valid: "AWVALID".to_string(),
            }],
        });
        isf
    }

    #[test]
    fn temporal_stage_renders_ready_valid_barrier() {
        let out = isf_with_temporal_stage().render();
        assert!(
            out.contains("    (stage stage_h1 (ready AWREADY) (valid AWVALID))"),
            "stage shape:\n{out}"
        );
        assert_eq!(paren_balance(&out), 0, "unbalanced:\n{out}");
    }

    #[test]
    fn temporal_stage_isf_passes_fsmgen_strict_validation() {
        let tempdir = tempfile::tempdir().expect("tempdir");
        let out = isf_with_temporal_stage().render();
        let isf_path = tempdir.path().join("temporal_stage.isf");
        std::fs::write(&isf_path, &out).expect("write isf");
        eprintln!("=== ISF ===\n{out}\n=== END ===");

        let output = crate::ir::run_fsmgen_strict_check(&isf_path);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let check: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!("fsmgen non-JSON.\nstdout:{stdout}\nstderr:{stderr}\nerr:{e}")
        });
        let success = check["diagnostic_summary"]["success"]
            .as_bool()
            .unwrap_or(false);
        if !success && let Some(diags) = check["diagnostics"].as_array() {
            for d in diags {
                eprintln!(
                    "FSMGen diagnostic: {}",
                    d.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("(none)")
                );
            }
        }
        assert!(
            success,
            "FSMGen strict rejected the `(stage …)` ready_valid_barrier \
             (expected accepted at pin 9bfb9a20)"
        );
    }

    #[test]
    fn transaction_steps_use_fsmgen_contract_grammar() {
        // FSMGen ISF grammar (book 13e-data-manipulation / 13f-composition):
        // shift_left / shift_right / await_all / await_any (underscore) and
        // `(spawn child as name)` — NOT the old hyphen / missing-`as` forms.
        let mut isf = minimal_isf();
        isf.transactions.push(IsfTransaction {
            name: "t_grammar".into(),
            on_trigger: None,
            on_steps: vec![],
            steps: vec![
                IsfTxnStep::ShiftLeft {
                    reg: "rdata".into(),
                    bit: "sda".into(),
                },
                IsfTxnStep::ShiftRight {
                    reg: "rdata".into(),
                    bit: "sda".into(),
                    width: Some(8),
                },
                IsfTxnStep::ShiftRight {
                    reg: "rdata".into(),
                    bit: "sda".into(),
                    width: None,
                },
                IsfTxnStep::AwaitAll {
                    done_port: "done".into(),
                },
                IsfTxnStep::AwaitAny {
                    done_port: "done".into(),
                },
                IsfTxnStep::Spawn {
                    child_transaction: "worker".into(),
                    instance: "w0".into(),
                },
            ],
            complete: "done".into(),
            latency_min: None,
            latency_max: None,
            contracts: vec![],
            stages: vec![],
        });
        let out = isf.render();
        // Contract-exact forms present:
        assert!(out.contains("(shift_left rdata sda)"), "{out}");
        assert!(out.contains("(shift_right rdata sda (width 8))"), "{out}");
        assert!(out.contains("(shift_right rdata sda)"), "{out}");
        assert!(out.contains("(await_all done)"), "{out}");
        assert!(out.contains("(await_any done)"), "{out}");
        assert!(out.contains("(spawn worker as w0)"), "{out}");
        // Old buggy hyphen / missing-`as` forms must be gone:
        assert!(!out.contains("shift-left"), "{out}");
        assert!(!out.contains("shift-right"), "{out}");
        assert!(!out.contains("await-all"), "{out}");
        assert!(!out.contains("await-any"), "{out}");
        assert!(!out.contains("(spawn worker w0)"), "{out}");
    }

    #[test]
    fn render_emits_symbol_surface_and_skips_expression_values() {
        let mut isf = minimal_isf();
        isf.types = vec![IsfTypeDef {
            name: "mode".into(),
            bits: 1,
        }];
        isf.enums = vec![IsfEnum {
            type_name: "mode".into(),
            members: vec![("IDLE".into(), "0".into()), ("BUSY".into(), "1".into())],
        }];
        isf.constants = vec![
            IsfConstant {
                name: "DEFAULT".into(),
                value: "5".into(),
            },
            IsfConstant {
                name: "ALIASED".into(),
                value: "mode.BUSY".into(),
            },
            IsfConstant {
                name: "EXPR_VALUED".into(),
                value: "(| a b)".into(),
            },
        ];
        let out = isf.render();
        assert!(out.contains("(type mode (bits 1))"), "{out}");
        assert!(out.contains("(mode (IDLE 0) (BUSY 1))"), "{out}");
        assert!(out.contains("(DEFAULT 5)"), "{out}");
        assert!(out.contains("(ALIASED mode.BUSY)"), "{out}");
        // The expression-valued constant is excluded (would be strict-invalid):
        assert!(!out.contains("EXPR_VALUED"), "{out}");
        assert!(!out.contains("(| a b)"), "{out}");
    }

    #[test]
    fn symbol_surface_passes_fsmgen_strict_validation() {
        // End-to-end: a recovered enum co-declares (type NAME (bits k)) +
        // (enums (NAME …)) (FSMGen's c0b7eaa7 contract), plus a literal
        // (constants …); the emitted .isf must pass the real fsmgen --strict.
        let mut isf = isf_with_temporal_contract_transaction();
        isf.types = vec![IsfTypeDef {
            name: "mode".into(),
            bits: 1,
        }];
        isf.enums = vec![IsfEnum {
            type_name: "mode".into(),
            members: vec![("IDLE".into(), "0".into()), ("BUSY".into(), "1".into())],
        }];
        isf.constants = vec![IsfConstant {
            name: "DEFAULT".into(),
            value: "1".into(),
        }];
        let tempdir = tempfile::tempdir().expect("tempdir");
        let out = isf.render();
        let isf_path = tempdir.path().join("symbol_surface.isf");
        std::fs::write(&isf_path, &out).expect("write isf");
        eprintln!("=== ISF ===\n{out}\n=== END ===");
        let output = crate::ir::run_fsmgen_strict_check(&isf_path);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let check: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!("fsmgen non-JSON.\nstdout:{stdout}\nstderr:{stderr}\nerr:{e}")
        });
        let success = check["diagnostic_summary"]["success"]
            .as_bool()
            .unwrap_or(false);
        if !success && let Some(diags) = check["diagnostics"].as_array() {
            for d in diags {
                eprintln!(
                    "FSMGen diagnostic: {}",
                    d.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("(none)")
                );
            }
        }
        assert!(
            success,
            "FSMGen strict rejected the emitted (types)/(enums)/(constants) symbol surface"
        );
    }

    #[test]
    fn emitted_symbol_counts_reflect_safe_emitted_subset() {
        // The artifact's constant_count/enum_count derive from these, so they
        // must equal what render() actually emits (metric == emitted content).
        let mut isf = minimal_isf();
        isf.types = vec![IsfTypeDef {
            name: "mode".into(),
            bits: 1,
        }];
        isf.enums = vec![
            IsfEnum {
                type_name: "mode".into(),
                members: vec![("IDLE".into(), "0".into()), ("BUSY".into(), "1".into())],
            },
            // Excluded: a member value that is an operator expression.
            IsfEnum {
                type_name: "bad".into(),
                members: vec![("X".into(), "(| a b)".into())],
            },
        ];
        isf.constants = vec![
            IsfConstant {
                name: "OK".into(),
                value: "5".into(),
            },
            IsfConstant {
                name: "SKIP".into(),
                value: "(! x)".into(),
            },
        ];
        assert_eq!(isf.emitted_constant_count(), 1);
        assert_eq!(isf.emitted_enum_count(), 1);
        // render() agrees — the unsafe entries are absent from the output.
        let out = isf.render();
        assert!(out.contains("(OK 5)"), "{out}");
        assert!(out.contains("(mode (IDLE 0) (BUSY 1))"), "{out}");
        assert!(!out.contains("SKIP"), "{out}");
        assert!(!out.contains("(bad"), "{out}");
    }

    /// `EXTRACTION-REPRESENTATION-GAPS.59` — an enum type is as wide as its largest code, not its member count.
    #[test]
    fn an_enum_type_is_as_wide_as_its_largest_code() {
        let members = |values: &[&str]| -> Vec<(String, String)> {
            values
                .iter()
                .enumerate()
                .map(|(i, v)| (format!("M{i}"), v.to_string()))
                .collect()
        };
        assert_eq!(
            super::enum_type_bits(&members(&["48", "49", "1", "2"])),
            6,
            "AWATOP's 49"
        );
        assert_eq!(
            super::enum_type_bits(&members(&["0", "1", "2", "3", "14", "15"])),
            4,
            "ARCACHE's 15"
        );
        assert_eq!(
            super::enum_type_bits(&members(&["0", "3"])),
            2,
            "AxDOMAIN's 3, two members"
        );
        assert_eq!(
            super::enum_type_bits(&members(&["0", "1", "2", "3", "4", "5", "6", "7"])),
            3,
            "a dense code set keeps its width"
        );
        assert_eq!(
            super::enum_type_bits(&members(&["0"])),
            1,
            "at least one bit"
        );
        assert_eq!(super::enum_type_bits(&members(&["0x1F", "0b11"])), 5);
        assert_eq!(
            super::enum_type_bits(&members(&["0", "WIDTH", "2"])),
            2,
            "a non-literal value leaves the count-based width"
        );
    }

    #[test]
    fn binary_looking_enum_value_is_excluded_and_recorded_as_residual() {
        // KG-ISF-COMPLETENESS.2a.iv: FSMGen rejects a BARE token of only 0/1 digits with
        // length >= 4 (a binary literal un-qualified). An enum carrying such a value — HBM2's
        // mega-conflated `TABLE` whose binary codes (1000/1111) were mis-read as decimals — is
        // held out of the `.isf` and recorded as a residual; a well-formed enum (999 has a 9,
        // 0 is short) is emitted byte-identically.
        let mut isf = minimal_isf();
        isf.types = vec![
            IsfTypeDef {
                name: "mode".into(),
                bits: 2,
            },
            IsfTypeDef {
                name: "table".into(),
                bits: 6,
            },
        ];
        isf.enums = vec![
            // Well-formed: small bare decimals FSMGen accepts -> emitted, byte-identical.
            IsfEnum {
                type_name: "mode".into(),
                members: vec![("IDLE".into(), "0".into()), ("BUSY".into(), "999".into())],
            },
            // Un-emittable: a bare decimal >= 1000 (binary code mis-read as decimal) -> dropped.
            IsfEnum {
                type_name: "table".into(),
                members: vec![
                    ("A".into(), "0".into()),
                    ("B".into(), "1000".into()),
                    ("C".into(), "1111".into()),
                ],
            },
        ];
        assert_eq!(isf.emitted_enum_count(), 1);
        let out = isf.render();
        assert!(out.contains("(mode (IDLE 0) (BUSY 999))"), "{out}");
        assert!(!out.contains("(table"), "{out}");
        // KG-ISF-COMPLETENESS.5.i — the emittable enum keeps its backing type, but the dropped
        // `table` enum must NOT leave an orphan `(type table (bits 6))` line (the emitter now gates
        // the types block by the emitted-enum set).
        assert!(out.contains("(type mode (bits 2))"), "{out}");
        assert!(!out.contains("(type table"), "{out}");
        let residuals = isf.enum_residuals();
        assert_eq!(residuals.len(), 1);
        assert!(
            residuals[0].packet_id.contains("enum_value_literal"),
            "{}",
            residuals[0].packet_id
        );
        assert!(
            residuals[0].why_unresolved.contains("DROPPED"),
            "{}",
            residuals[0].why_unresolved
        );
        assert_eq!(residuals[0].candidate_interpretations.len(), 2);
    }

    #[test]
    fn well_formed_enum_boundary_and_radix_tokens_still_emit() {
        // Guard the byte-identical invariant: a bare decimal AT the boundary (999) stays
        // emitted; a width/radix-qualified token is accepted at ANY magnitude (16'd1000) and
        // is never dropped; no residual recorded.
        let mut isf = minimal_isf();
        isf.types = vec![IsfTypeDef {
            name: "k".into(),
            bits: 16,
        }];
        isf.enums = vec![IsfEnum {
            type_name: "k".into(),
            members: vec![
                ("LIM".into(), "999".into()),
                ("BIG".into(), "16'd1000".into()),
            ],
        }];
        assert_eq!(isf.emitted_enum_count(), 1);
        assert!(isf.enum_residuals().is_empty());
        let out = isf.render();
        assert!(out.contains("(k (LIM 999) (BIG 16'd1000))"), "{out}");
    }

    #[test]
    fn dedup_records_conflicting_rule_as_residual_not_silent_drop() {
        let rules = vec![
            IsfRule {
                name: "r_keep".to_string(),
                condition: "SEL == 1".to_string(),
                drives: vec![("GRANT".to_string(), "1".to_string())],
            },
            IsfRule {
                name: "r_conflict".to_string(),
                condition: "SEL == 1".to_string(),
                drives: vec![("GRANT".to_string(), "0".to_string())],
            },
            IsfRule {
                name: "r_other".to_string(),
                condition: "EN == 1".to_string(),
                drives: vec![("BUSY".to_string(), "1".to_string())],
            },
        ];
        let (deduped, residuals) = dedup_conflicting_rules(rules);
        // First GRANT rule + the non-conflicting BUSY rule survive; the
        // conflicting second GRANT rule is dropped from emission...
        let kept: Vec<&str> = deduped.iter().map(|r| r.name.as_str()).collect();
        assert_eq!(kept, vec!["r_keep", "r_other"]);
        // ...but recorded as an explicit residual instead of silently lost.
        assert_eq!(residuals.len(), 1);
        let p = &residuals[0];
        assert!(p.packet_id.contains("rule_conflict"), "{}", p.packet_id);
        assert!(p.why_unresolved.contains("GRANT"), "{}", p.why_unresolved);
        assert!(p.why_unresolved.contains("DROPPED"), "{}", p.why_unresolved);
        assert_eq!(p.candidate_interpretations.len(), 2);
    }

    #[test]
    fn the_same_level_in_two_spellings_is_not_a_conflict() {
        // EXTRACTION-REPRESENTATION-GAPS.10.f — AXI `ARLOCK`: *Must be 0b0* and *tied LOW* drive the same level. It is
        // emitted once and the restatement listed as such, never as a conflict; FSMGen refuses the two spellings
        // together. A different number still conflicts, and a word is compared as written.
        let rule = |name: &str, value: &str| IsfRule {
            name: name.to_string(),
            condition: String::new(),
            drives: vec![("XLOCK".to_string(), value.to_string())],
        };
        fn same_level(residuals: &[ResidualDecisionPacket]) -> Vec<&str> {
            residuals
                .iter()
                .map(|packet| packet.packet_id.as_str())
                .collect()
        }
        let (kept, residuals) =
            dedup_conflicting_rules(vec![rule("binary", "0B0"), rule("decimal", "0")]);
        assert_eq!(kept.len(), 1);
        assert_eq!(same_level(&residuals), ["isf_rule_same_level_decimal"]);
        let (kept, residuals) =
            drop_unconditional_overlap_conflicts(vec![rule("binary", "0B0"), rule("decimal", "0")]);
        assert_eq!(kept.len(), 1);
        assert_eq!(same_level(&residuals), ["isf_rule_same_level_decimal"]);
        let (_, residuals) =
            dedup_conflicting_rules(vec![rule("fourteen", "0B1110"), rule("zero", "0")]);
        assert_eq!(same_level(&residuals), ["isf_rule_conflict_zero"]);
        let (_, residuals) =
            dedup_conflicting_rules(vec![rule("member", "INCR"), rule("other", "WRAP")]);
        assert_eq!(same_level(&residuals), ["isf_rule_conflict_other"]);
        let (kept, residuals) =
            dedup_conflicting_rules(vec![rule("first", "0"), rule("again", "0")]);
        assert_eq!((kept.len(), residuals.len()), (2, 0));
    }

    #[test]
    fn dedup_without_conflict_keeps_all_rules_and_records_no_residual() {
        // Same signal but DIFFERENT guards is not a conflict — both kept.
        let rules = vec![
            IsfRule {
                name: "a".to_string(),
                condition: "SEL == 1".to_string(),
                drives: vec![("X".to_string(), "1".to_string())],
            },
            IsfRule {
                name: "b".to_string(),
                condition: "SEL == 0".to_string(),
                drives: vec![("X".to_string(), "0".to_string())],
            },
        ];
        let (deduped, residuals) = dedup_conflicting_rules(rules);
        assert_eq!(deduped.len(), 2);
        assert!(residuals.is_empty());
    }
}

#[cfg(test)]
mod parity_check_tests {
    use super::{
        CheckSignalRecord, IsfDirection, IsfSignal, ParityRefusal, ParitySense, lower_parity_checks,
    };
    use std::collections::{BTreeMap, BTreeSet};

    fn signal(name: &str, direction: IsfDirection, width: u32) -> IsfSignal {
        IsfSignal {
            name: name.to_string(),
            direction,
            width,
            width_param: None,
        }
    }

    fn check(
        name: &str,
        covered: &[&str],
        width: &str,
        enable: &str,
        sense: Option<ParitySense>,
    ) -> CheckSignalRecord {
        CheckSignalRecord {
            check_id: format!("check_{name}"),
            check_signal: name.to_string(),
            covered_signals: covered.iter().map(|s| s.to_string()).collect(),
            width: Some(width.to_string()),
            granularity: None,
            check_enable: Some(enable.to_string()),
            table_id: "table_0001".to_string(),
            parity_sense: sense,
            parity_statement_ids: Vec::new(),
        }
    }

    fn actor() -> (BTreeSet<IsfSignal>, BTreeMap<String, u32>) {
        use IsfDirection::{Input, Output};
        let signals = [
            signal("AWVALID", Output, 1),
            signal("AWVALIDCHK", Output, 1),
            signal("PSELx", Output, 1),
            signal("PWRITE", Output, 1),
            signal("PNSE", Output, 1),
            signal("PPROT", Output, 3),
            signal("PCTRLCHK", Output, 1),
            signal("PSTRB", Output, 1),
            signal("PSTRBCHK", Output, 1),
            signal("PREADY", Input, 1),
            signal("PREADYCHK", Input, 1),
            signal("PRDATACHK", Output, 1),
        ]
        .into_iter()
        .collect();
        let stated = [
            ("AWVALID", 1),
            ("PSELx", 1),
            ("PWRITE", 1),
            ("PNSE", 1),
            ("PPROT", 3),
            ("PREADY", 1),
        ]
        .into_iter()
        .map(|(name, width)| (name.to_string(), width))
        .collect();
        (signals, stated)
    }

    /// EXTRACTION-REPRESENTATION-GAPS.46.b — a one-bit odd parity over one signal is its inversion, over several the
    /// inverted XOR of every bit, under the enable read as declared signals; even parity is the chain itself.
    #[test]
    fn a_one_bit_check_over_stated_widths_lowers_as_one_parity_rule() {
        let (signals, stated) = actor();
        let (rules, refused) = lower_parity_checks(
            &[
                check(
                    "AWVALIDCHK",
                    &["AWVALID"],
                    "1",
                    "ARESETn",
                    Some(ParitySense::Odd),
                ),
                check(
                    "PCTRLCHK",
                    &["PPROT", "PWRITE", "PNSE"],
                    "1",
                    "PSELx & PWRITE",
                    Some(ParitySense::Even),
                ),
            ],
            &signals,
            &stated,
            Some("ARESETn"),
        );
        assert!(refused.is_empty(), "{refused:?}");
        let read: Vec<(&str, &str, &str, &str)> = rules
            .iter()
            .map(|rule| {
                (
                    rule.name.as_str(),
                    rule.condition.as_str(),
                    rule.drives[0].0.as_str(),
                    rule.drives[0].1.as_str(),
                )
            })
            .collect();
        assert_eq!(
            read,
            vec![
                (
                    "parity_AWVALIDCHK",
                    "(== ARESETn 1)",
                    "AWVALIDCHK",
                    "(! AWVALID)"
                ),
                (
                    "parity_PCTRLCHK",
                    "(& (== PSELx 1) (== PWRITE 1))",
                    "PCTRLCHK",
                    "(^ (^ (^ (^ PPROT[0] PPROT[1]) PPROT[2]) PWRITE) PNSE)"
                ),
            ]
        );
    }

    #[test]
    fn a_check_this_actor_cannot_state_is_refused_with_its_reason() {
        let (signals, stated) = actor();
        let odd = Some(ParitySense::Odd);
        let (rules, refused) = lower_parity_checks(
            &[
                check("AWVALIDCHK", &["AWVALID"], "1", "ARESETn", None),
                check("PRDATACHK", &["PSELx"], "ceil(DATA_WIDTH/8)", "PSELx", odd),
                check("PREADYCHK", &["PREADY"], "1", "PSELx", odd),
                check("PRDATACHK", &["PREADY"], "1", "PSELx", odd),
                check("PSTRBCHK", &["PSTRB"], "1", "PSELx & PWRITE", odd),
                check("PCTRLCHK", &["PWRITE"], "1", "Write data phase", odd),
                check("PCTRLCHK", &["PWRITE"], "1", "PSEL", odd),
            ],
            &signals,
            &stated,
            Some("ARESETn"),
        );
        assert!(rules.is_empty());
        assert_eq!(
            refused
                .iter()
                .map(|(_, refusal)| *refusal)
                .collect::<Vec<_>>(),
            vec![
                ParityRefusal::NoStatedSense,
                ParityRefusal::ByteGranular,
                ParityRefusal::NotDrivenHere,
                ParityRefusal::CoveredNotDrivenHere,
                ParityRefusal::CoveredWidthNotStated,
                ParityRefusal::EnableNotSignals,
                ParityRefusal::EnableNotSignals,
            ]
        );
    }
}
