// ISF IR — typed intermediate representation for Intent Scheduling Format
// =========================================================================
// Flow:  IntentIR  →  IsfIr::from_intent_ir()  →  IsfIr  →  IsfIr::render()  →  .isf text
//
// This IR eliminates string-bashing bugs by design:
//   - BTreeSet<IsfSignal>        → dedup by construction (no duplicate clock/reset)
//   - IsfReset (non-optional)    → compiler enforces presence (strict mode requires it)
//   - Typed IsfRule / IsfPriority → cannot emit invalid S-expression syntax
//   - Recursive tree walk        → parentheses match by construction

use std::collections::BTreeSet;

use crate::ir::intent::{IntentIr, TransactionStep};
use crate::ir::semantic::{
    ControlActionRecord, ControlBinaryOperator, ControlBranchRecord,
    ControlCompoundUpdateOperation, ControlExpressionRecord, ControlReferenceSuffix,
    ControlUnaryOperator, InterfaceSignalDirection, SymbolDefinitionKind, SystemResetKind,
    SystemResetPolarity,
};
// R16-CONTRACT-IR.3: `TemporalRuleRecord`/`TemporalPredicateRecord` are now
// referenced only by the test-only parity oracle (`classify_temporal_rule`
// + helpers) and the test module — production lowering uses ContractIR.
#[cfg(test)]
use crate::ir::semantic::{TemporalPredicateRecord, TemporalRuleRecord};
use crate::ir::source::{
    AutomationConfidence, CandidateInterpretation, RegisterFieldRecord, ResidualDecisionPacket,
    WidthHint,
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
    width: u32,
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfPriority {
    higher: String,
    over: String,
}

pub(crate) struct IsfIr {
    actor_name: String,
    clock: String,
    reset: IsfReset,
    watchdog: u64,
    signals: BTreeSet<IsfSignal>,
    constants: Vec<IsfConstant>,
    types: Vec<IsfTypeDef>,
    enums: Vec<IsfEnum>,
    storage: Vec<IsfStorageVar>,
    drives: Vec<IsfNamedDrive>,
    transactions: Vec<IsfTransaction>,
    rules: Vec<IsfRule>,
    priorities: Vec<IsfPriority>,
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

    /// The `(enums …)` families `render()` actually emits: those whose
    /// every member value is a whitespace-free scalar.
    fn emitted_enums(&self) -> Vec<&IsfEnum> {
        self.enums
            .iter()
            .filter(|e| {
                !e.members.is_empty() && e.members.iter().all(|(_, v)| is_safe_isf_scalar_value(v))
            })
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

    pub(crate) fn render(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        lines.push(format!("(actor {}", self.actor_name));

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
        if !self.types.is_empty() {
            lines.push("  (types".to_string());
            for t in &self.types {
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
                lines.push(format!(
                    "    ({} {} (width {}))",
                    sig.direction.as_str(),
                    sig.name,
                    sig.width
                ));
            }
            lines.push("  )".to_string());
        }

        if !self.storage.is_empty() {
            lines.push("  (storage".to_string());
            for v in &self.storage {
                match v.reset {
                    Some(reset) => lines.push(format!(
                        "    (var {} (width {}) (reset {}))",
                        v.name, v.width, reset
                    )),
                    None => lines.push(format!("    (var {} (width {}))", v.name, v.width)),
                }
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

        for p in &self.priorities {
            lines.push(format!("  (priority {} over {})", p.higher, p.over));
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
        let clock = if let Some(sc) = &intent_ir.system_contract {
            sc.clock_signal.clone()
        } else if let Some(infra) = intent_ir.infrastructure_signals.iter().find(|s| {
            s.signal_name.to_lowercase().contains("clk")
                || s.signal_name.to_lowercase().contains("clock")
        }) {
            infra.signal_name.clone()
        } else {
            "clk".to_string()
        };

        // --- Reset (always populated — strict mode requires it) ---
        // FSMGen strict mode rejects sync active-low on _n/_b suffixed signals;
        // those suffixes conventionally mean async, so override timing.
        let reset = if let Some(sc) = &intent_ir.system_contract {
            let is_n_suffix = sc.reset_signal.ends_with("_n") || sc.reset_signal.ends_with("_b");
            IsfReset {
                signal: sc.reset_signal.clone(),
                timing: if is_n_suffix {
                    "async".to_string()
                } else {
                    match sc.reset_kind {
                        SystemResetKind::Synchronous => "sync".to_string(),
                        SystemResetKind::Asynchronous => "async".to_string(),
                    }
                },
                polarity: match sc.reset_polarity {
                    SystemResetPolarity::ActiveHigh => "active_high".to_string(),
                    SystemResetPolarity::ActiveLow => "active_low".to_string(),
                },
            }
        } else if let Some(infra) = intent_ir.infrastructure_signals.iter().find(|s| {
            s.signal_name.to_lowercase().contains("rst")
                || s.signal_name.to_lowercase().contains("reset")
        }) {
            let is_n_suffix =
                infra.signal_name.ends_with("_n") || infra.signal_name.ends_with("_b");
            IsfReset {
                signal: infra.signal_name.clone(),
                timing: if is_n_suffix {
                    "async".to_string()
                } else {
                    "sync".to_string()
                },
                polarity: if is_n_suffix {
                    "active_low".to_string()
                } else {
                    "active_high".to_string()
                },
            }
        } else {
            IsfReset {
                signal: "rst_n".to_string(),
                timing: "async".to_string(),
                polarity: "active_low".to_string(),
            }
        };

        // --- Signals (dedup by name, clock/reset excluded) ---
        let infra_names: BTreeSet<&str> = [clock.as_str(), reset.signal.as_str()]
            .into_iter()
            .collect();
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
                let dir = match sig.direction_hint {
                    Some(InterfaceSignalDirection::Input) => IsfDirection::Input,
                    _ => IsfDirection::Output,
                };
                let width = match &sig.width_hint {
                    Some(w) => render_isf_width_hint(w).parse::<u32>().unwrap_or(1),
                    None => 1,
                };
                signals.insert(IsfSignal {
                    name: sig.signal_name.clone(),
                    direction: dir,
                    width,
                });
            }
        }

        // --- Constants ---
        let constants: Vec<IsfConstant> = intent_ir
            .symbol_definitions
            .iter()
            .filter(|s| {
                matches!(
                    s.kind,
                    SymbolDefinitionKind::Constant
                        | SymbolDefinitionKind::Define
                        | SymbolDefinitionKind::Param
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
            let bits = s.members.len().max(1).next_power_of_two().trailing_zeros();
            types.push(IsfTypeDef {
                name: s.symbol_name.clone(),
                bits: if bits < 1 { 1 } else { bits },
            });
            enums.push(IsfEnum {
                type_name: s.symbol_name.clone(),
                members: s
                    .members
                    .iter()
                    .map(|m| {
                        (
                            m.member_name.clone(),
                            render_isf_control_expression(&m.value),
                        )
                    })
                    .collect(),
            });
        }

        // --- Storage (dedup by name) ---
        // ISF-REGISTER-RESET-EMIT.2: each register lowers to a `(storage (var …))`; when the
        // register's documented per-field `reset_value`s compose to a clean in-width non-negative
        // integer we also emit `(reset V)`, else the var stays reset-less (FSMGen defaults it to
        // all-0s — byte-identical to the pre-`.2` output). The var width is unchanged here
        // (max-field-extent); reconciling it to the true register width is ISF-REGISTER-RESET-EMIT.3.
        let mut seen_storage_names: BTreeSet<String> = BTreeSet::new();
        let mut storage: Vec<IsfStorageVar> = Vec::new();
        let mut reset_not_lowerable: usize = 0;
        let mut reset_deferred_width: usize = 0;
        for r in &intent_ir.register_records {
            let name = sanitize_isf_name(&r.register_name.to_lowercase());
            if !seen_storage_names.insert(name.clone()) {
                continue;
            }
            let width = r
                .fields
                .iter()
                .filter_map(|f| match (f.bits_high, f.bits_low) {
                    (Some(hi), Some(lo)) => Some(hi.saturating_sub(lo).saturating_add(1)),
                    _ => None,
                })
                .max()
                .unwrap_or(32);
            let reset = match classify_register_reset(&r.fields, width) {
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
            };
            storage.push(IsfStorageVar { name, width, reset });
        }
        let mut storage_reset_residuals: Vec<ResidualDecisionPacket> = Vec::new();
        if reset_not_lowerable + reset_deferred_width > 0 {
            storage_reset_residuals.push(storage_reset_residual_packet(
                reset_not_lowerable,
                reset_deferred_width,
            ));
        }

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
            match classify_actor_contract(contract, &signal_names) {
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

        let mut rules: Vec<IsfRule> = Vec::new();

        for (cr_idx, cr) in intent_ir.conditional_rules.iter().enumerate() {
            let sig = cr.consequent_signal.as_deref().unwrap_or("unknown");
            if !signal_names.contains(sig) {
                continue;
            }
            let val = if cr.consequent_action.contains("not")
                || cr.consequent_action.contains("must not")
            {
                "0"
            } else {
                "1"
            };
            let condition = sanitize_rule_condition(&cr.antecedent_text);
            rules.push(IsfRule {
                name: format!("rule_{}", cr_idx),
                condition,
                drives: vec![(sig.to_string(), val.to_string())],
            });
        }

        for (sc_idx, sc) in intent_ir.signal_constraints.iter().enumerate() {
            if !signal_names.contains(&sc.subject_signal) {
                continue;
            }
            let guard = sc.condition_text.as_deref().unwrap_or("true");
            let val = if sc.negated {
                sc.target_value.as_deref().unwrap_or("0")
            } else {
                sc.target_value.as_deref().unwrap_or("1")
            };
            rules.push(IsfRule {
                name: format!("constraint_{}", sc_idx),
                condition: sanitize_rule_condition(guard),
                drives: vec![(sc.subject_signal.clone(), val.to_string())],
            });
        }

        for inv in &intent_ir.temporal_invariants {
            if inv.subject_signal.is_empty() {
                continue;
            }
            if !signal_names.contains(&inv.subject_signal) {
                continue;
            }
            let guard = inv.condition_signal.as_deref().unwrap_or("true");
            let target_val = inv.target_value.as_deref().unwrap_or("1");
            rules.push(IsfRule {
                name: sanitize_isf_name(&inv.invariant_id),
                condition: sanitize_rule_condition(guard),
                drives: vec![(inv.subject_signal.clone(), target_val.to_string())],
            });
        }

        // Temporal value/guard→drive rules (`.2.3` #3) join the rule set
        // before dedup so a temporal rule that conflicts with an existing
        // rule on the same signal+guard is dropped (FSMGen strict rejects
        // conflicting drives) rather than producing invalid `.isf`.
        rules.extend(temporal_isf_rules);

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

        // --- Priorities ---
        let mut priorities: Vec<IsfPriority> = Vec::new();
        let has_rules = !rules.is_empty();
        let has_transactions = !all_transactions.is_empty();
        if has_rules && has_transactions {
            for rule in &rules {
                for tx in &all_transactions {
                    priorities.push(IsfPriority {
                        higher: rule.name.clone(),
                        over: tx.name.clone(),
                    });
                }
            }
        }

        IsfIr {
            actor_name: actor_name.to_string(),
            clock,
            reset,
            watchdog: 65536,
            signals,
            constants,
            types,
            enums,
            storage,
            drives,
            transactions: all_transactions,
            rules,
            priorities,
            temporal_residuals,
            storage_reset_residuals,
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

/// Classify a register's reset for ISF storage lowering (ISF-REGISTER-RESET-EMIT.2). *Strictly
/// composable* iff EVERY field carries a `reset_value` that parses to a non-negative integer
/// fitting its own field width, every field is located, and no two fields overlap; the per-field
/// values are then LSB-tiled into the register value `V`. Bounded to ≤64-bit registers (`u64`);
/// anything wider stays an honest residual.
fn classify_register_reset(fields: &[RegisterFieldRecord], var_width: u32) -> RegisterResetOutcome {
    if !fields.iter().any(|f| f.reset_value.is_some()) {
        return RegisterResetOutcome::NoReset;
    }
    let mut composed: u64 = 0;
    let mut used_mask: u64 = 0;
    for f in fields {
        let Some(raw) = f.reset_value.as_deref() else {
            return RegisterResetOutcome::NotLowerable; // partial coverage
        };
        let Some((hi, lo)) = register_field_extent(f) else {
            return RegisterResetOutcome::NotLowerable; // unlocated
        };
        let field_width = hi - lo + 1;
        if hi >= 64 || field_width >= 64 {
            return RegisterResetOutcome::NotLowerable; // beyond u64-safe tiling
        }
        let Some(value) = parse_reset_literal(raw) else {
            return RegisterResetOutcome::NotLowerable; // symbolic
        };
        if value >= (1u64 << field_width) {
            return RegisterResetOutcome::NotLowerable; // value over-wide for its field
        }
        let mask = ((1u64 << field_width) - 1) << lo;
        if used_mask & mask != 0 {
            return RegisterResetOutcome::NotLowerable; // overlapping fields
        }
        used_mask |= mask;
        composed |= value << lo;
    }
    if composed == 0 {
        return RegisterResetOutcome::DefaultZero;
    }
    if var_width >= 64 || composed < (1u64 << var_width) {
        RegisterResetOutcome::Emit(composed)
    } else {
        RegisterResetOutcome::DeferredWidth
    }
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

fn sanitize_isf_name(raw: &str) -> String {
    let mut name = raw
        .replace(
            [
                ' ', '-', '.', ':', '/', '#', '+', '*', '(', ')', '\'', ',', '=', '<', '>', '?',
                '!', '@', '$', '%', '^', '&', ';', '"', '\\', '[', ']', '{', '}', '|', '~', '`',
                '…',
            ],
            "_",
        )
        .to_lowercase();
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

fn sanitize_rule_condition(raw: &str) -> String {
    // FSMGen strict mode rejects bare-token rule guards (they trigger an
    // infix-assignment check in SourceFrontend).  Multi-word text would also
    // break the parse.  Return empty string to signal "no guard" (always active).
    if raw.contains(char::is_whitespace) || raw.len() > 80 || raw.is_empty() || raw == "true" {
        String::new()
    } else {
        // Single-token guard — sanitize and wrap as equality check.
        let token = sanitize_isf_name(raw);
        format!("(== {} 1)", token)
    }
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

/// Normalize a temporal predicate's textual value to an ISF literal, or
/// `None` if it is not safely representable. Fabricating a value for a
/// non-literal is forbidden (`fsmgen-contract-authority`): such rules become
/// residual decisions instead.
fn isf_literal_value(raw: &str) -> Option<String> {
    let t = raw.trim();
    match t.to_ascii_lowercase().as_str() {
        "1" | "high" | "asserted" | "assert" | "true" | "set" | "active" => {
            return Some("1".to_string());
        }
        "0" | "low" | "deasserted" | "deassert" | "false" | "clear" | "inactive" => {
            return Some("0".to_string());
        }
        _ => {}
    }
    // Pure unsigned decimal / hex / binary integer literals only.
    let is_dec = !t.is_empty() && t.bytes().all(|b| b.is_ascii_digit());
    let is_hex = t.len() > 2
        && (t.starts_with("0x") || t.starts_with("0X"))
        && t[2..].bytes().all(|b| b.is_ascii_hexdigit());
    let is_bin = t.len() > 2
        && (t.starts_with("0b") || t.starts_with("0B"))
        && t[2..].bytes().all(|b| b == b'0' || b == b'1');
    if is_dec || is_hex || is_bin {
        Some(t.to_string())
    } else {
        None
    }
}

/// Production-side guard for an `ActorContract`, parity-matched by construction
/// to the oracle's `temporal_antecedent_condition`: the first interface-declared
/// `SignalValue` antecedent (a `guard_candidates` `Eq` with an ISF literal) →
/// `(== <signal> <literal>)`, else the empty string.
fn actor_guard_condition(
    guard_candidates: &[crate::ir::contract::Condition],
    declared_signals: &BTreeSet<String>,
) -> String {
    use crate::ir::contract::Condition;
    guard_candidates
        .iter()
        .find_map(|g| {
            let Condition::Eq {
                signal: s,
                value: v,
            } = g;
            if declared_signals.contains(s) {
                isf_literal_value(v).map(|lit| format!("(== {} {})", s, lit))
            } else {
                None
            }
        })
        .unwrap_or_default()
}

/// Build the ISF verification-family property (`(assert <prop>)`) for a windowed
/// bounded-eventually (`FSMGEN-ASSERT-LOWERING.3`):
/// - **unguarded** (`guard` empty) → the anchored monitor `(monitor (within s
///   max))` (F[0,max]; the existing `FSMGEN-ASSERT-MIGRATE` form);
/// - **guarded** (a representable boolean antecedent) → the faithful implication
///   `(=> g (within s [min] max))` — the antecedent is preserved (the monitor
///   dropped it) and `min` is emitted only when `>= 2` (`(within s max)` already
///   means `##[1:max]`).
///
/// Returns `None` for a *guarded* 0 lower bound: a `|-> ##[0:N]` consequent has
/// no ISF spelling (FSMGen rejects `(within B 0 MAX)` per `FSMGEN-MIN-WINDOW-
/// CONFIRM`), so that rule stays a residual rather than being mis-lowered.
fn windowed_eventual_prop(signal: &str, min: Option<u32>, max: u64, guard: &str) -> Option<String> {
    if guard.is_empty() {
        return Some(format!("(monitor (within {} {}))", signal, max));
    }
    match min {
        Some(0) => None,
        Some(m) if m >= 2 => Some(format!("(=> {} (within {} {} {}))", guard, signal, m, max)),
        _ => Some(format!("(=> {} (within {} {}))", guard, signal, max)),
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
) -> String {
    for p in &rule.antecedents {
        if let TemporalPredicateRecord::SignalValue {
            signal_name, value, ..
        } = p
            && declared_signals.contains(signal_name)
            && let Some(val) = isf_literal_value(value)
        {
            // Use the exact declared signal name (FSMGen is case-sensitive
            // and the guard must reference a signal in the interface) — do
            // NOT `sanitize_isf_name` it.
            return format!("(== {} {})", signal_name, val);
        }
    }
    String::new()
}

/// Classify one `temporal_rule` into its single ISF disposition.
/// `declared_signals` MUST be the exact set of signal names the emitter
/// renders into the `.isf` interface, so the classification matches what is
/// actually emitted (a rule/contract may only reference declared signals).
#[cfg(test)]
pub(crate) fn classify_temporal_rule(
    rule: &TemporalRuleRecord,
    declared_signals: &BTreeSet<String>,
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
        return match temporal_consequent_signal(rule) {
            Some(signal) if declared_signals.contains(&signal) => {
                // FSMGEN-ASSERT-LOWERING.3 (parity with classify_actor_contract):
                // guarded → `(=> g (within s [min] max))`; unguarded → monitor.
                let guard = temporal_antecedent_condition(rule, declared_signals);
                match windowed_eventual_prop(&signal, window.min_cycles, u64::from(within), &guard)
                {
                    Some(prop) => TemporalRuleDisposition::Contract {
                        name: sanitize_isf_name(&rule.rule_id),
                        prop,
                    },
                    None => TemporalRuleDisposition::Residual {
                        rule_id: rule.rule_id.clone(),
                        reason: format!(
                            "guarded bounded-eventually for '{}' has a 0-cycle lower \
                             bound; `|-> ##[0:N]` has no ISF spelling — preserved as \
                             residual (FSMGEN-MIN-WINDOW-CONFIRM)",
                            signal
                        ),
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
        let Some(val) = isf_literal_value(&value) else {
            return TemporalRuleDisposition::Residual {
                rule_id: rule.rule_id.clone(),
                reason: format!(
                    "temporal rule target value '{}' is not an ISF literal; \
                     fabricating a value is forbidden",
                    value
                ),
            };
        };
        return TemporalRuleDisposition::Rule {
            name: format!("temporal_{}", sanitize_isf_name(&rule.rule_id)),
            condition: temporal_antecedent_condition(rule, declared_signals),
            signal,
            value: val,
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
) -> TemporalRuleDisposition {
    use crate::ir::contract::{Condition, EventExpr, LoweringDisposition, Obligation, Window};

    let rule_id = contract.source_rule_id.clone().unwrap_or_else(|| {
        contract
            .contract_id
            .strip_prefix("contract_")
            .unwrap_or(&contract.contract_id)
            .to_string()
    });

    match &contract.obligation {
        // Windowed bounded_eventually candidate — produced ONLY for a
        // windowed rule with a single-signal consequent and a positive
        // bound (mirrors `classify_temporal_rule` branch 1, Contract arm).
        Obligation::Eventually {
            target: EventExpr::Level { signal, .. },
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
            } else {
                // FSMGEN-ASSERT-LOWERING.3: a guarded windowed-eventual keeps its
                // antecedent (the monitor dropped it) — `(=> g (within s [min] max))`;
                // unguarded stays the anchored monitor.
                let guard = actor_guard_condition(&contract.guard_candidates, declared_signals);
                match windowed_eventual_prop(signal, *min, u64::from(*max), &guard) {
                    Some(prop) => TemporalRuleDisposition::Contract {
                        name: sanitize_isf_name(&rule_id),
                        prop,
                    },
                    None => TemporalRuleDisposition::Residual {
                        rule_id,
                        reason: format!(
                            "guarded bounded-eventually for '{}' has a 0-cycle lower \
                             bound; `|-> ##[0:N]` has no ISF spelling — preserved as \
                             residual (FSMGEN-MIN-WINDOW-CONFIRM)",
                            signal
                        ),
                    },
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
            } else if let Some(val) = isf_literal_value(value) {
                // Reproduce `temporal_antecedent_condition` EXACTLY: the
                // first SignalValue antecedent that is interface-declared
                // with an ISF-literal value → `(== signal literal)`.
                let condition = contract
                    .guard_candidates
                    .iter()
                    .find_map(|g| {
                        let Condition::Eq {
                            signal: s,
                            value: v,
                        } = g;
                        if declared_signals.contains(s)
                            && let Some(lit) = isf_literal_value(v)
                        {
                            Some(format!("(== {} {})", s, lit))
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default();
                TemporalRuleDisposition::Rule {
                    name: format!("temporal_{}", sanitize_isf_name(&rule_id)),
                    condition,
                    signal: signal.clone(),
                    value: val,
                }
            } else {
                TemporalRuleDisposition::Residual {
                    rule_id,
                    reason: format!(
                        "temporal rule target value '{}' is not an ISF literal; \
                         fabricating a value is forbidden",
                        value
                    ),
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
            if let Some(prev_val) = seen.get(&key)
                && prev_val != val
            {
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
        for (sig, val) in &rule.drives {
            let key = (sig.clone(), rule.condition.clone());
            seen.entry(key).or_insert_with(|| val.clone());
        }
        deduped.push(rule);
    }
    (deduped, residuals)
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
            constants: vec![],
            types: vec![],
            enums: vec![],
            storage: vec![],
            drives: vec![],
            transactions: vec![],
            rules: vec![],
            priorities: vec![],
            temporal_residuals: vec![],
            storage_reset_residuals: vec![],
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
    }

    #[test]
    fn sanitize_rule_condition_only_accepts_single_token_guards() {
        assert_eq!(sanitize_rule_condition("has space"), "");
        assert_eq!(sanitize_rule_condition(""), "");
        assert_eq!(sanitize_rule_condition("true"), "");
        assert_eq!(sanitize_rule_condition(&"x".repeat(81)), "");
        assert_eq!(sanitize_rule_condition("ENABLE"), "(== enable 1)");
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
        });
        isf.signals.insert(IsfSignal {
            name: "AA".to_string(),
            direction: IsfDirection::Input,
            width: 8,
        });
        isf.signals.insert(IsfSignal {
            name: "AA".to_string(),
            direction: IsfDirection::Input,
            width: 8,
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
    fn render_emits_storage_drives_rules_and_priorities() {
        let mut isf = minimal_isf();
        isf.storage.push(IsfStorageVar {
            name: "acc".to_string(),
            width: 8,
            reset: None,
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
        isf.priorities.push(IsfPriority {
            higher: "r_guarded".to_string(),
            over: "r_uncond".to_string(),
        });

        let out = isf.render();
        assert!(out.contains("  (storage"));
        assert!(out.contains("    (var acc (width 8))"));
        assert!(out.contains("  (drive (out val) (sig 1))"));
        // Guarded rule keeps its condition; unconditional rule omits it.
        assert!(out.contains("  (rule r_guarded (== en 1)"));
        assert!(out.contains("  (rule r_uncond\n") || out.contains("  (rule r_uncond)"));
        assert!(out.contains("  (priority r_guarded over r_uncond)"));
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
        }];
        assert!(matches!(
            classify_register_reset(&unlocated, 8),
            RegisterResetOutcome::NotLowerable
        ));
    }

    #[test]
    fn render_storage_var_emits_reset_only_when_set() {
        let mut isf = minimal_isf();
        isf.storage.push(IsfStorageVar {
            name: "dpidr".to_string(),
            width: 32,
            reset: Some(0x1c01_3477),
        });
        isf.storage.push(IsfStorageVar {
            name: "plain".to_string(),
            width: 8,
            reset: None,
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
        });
        isf.signals.insert(IsfSignal {
            name: "RREADY".to_string(),
            direction: IsfDirection::Input,
            width: 1,
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

    // R16-CONTRACT-IR.3 PARITY GATE (by construction): for every rule
    // shape, `classify_actor_contract(contract_from_temporal_rule(r))`
    // must yield the SAME disposition as `classify_temporal_rule(r)` —
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
            let oracle = classify_temporal_rule(r, &sigs);
            let via = classify_actor_contract(&contract_from_temporal_rule(r), &sigs);
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
        let d = classify_temporal_rule(&rule, &declared(&["SEL", "GRANT"]));
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
        let d = classify_temporal_rule(&rule, &declared(&["GRANT"]));
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
    fn classify_windowed_signalvalue_is_contract() {
        let rule = t_rule(
            "r_win",
            vec![],
            vec![sigval("RVALID", "1")],
            Some(CycleWindowRecord {
                min_cycles: None,
                max_cycles: Some(8),
            }),
        );
        let d = classify_temporal_rule(&rule, &declared(&["RVALID"]));
        assert_eq!(
            d,
            TemporalRuleDisposition::Contract {
                name: "r_win".to_string(),
                // unguarded bounded-eventually → anchored monitor
                prop: "(monitor (within RVALID 8))".to_string(),
            }
        );
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
        let d = classify_temporal_rule(&rule, &declared(&["PENABLE", "PREADY"]));
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
        let d = classify_temporal_rule(&rule, &declared(&["PENABLE", "PREADY"]));
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
        let d = classify_temporal_rule(&rule, &declared(&["PENABLE", "PREADY"]));
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
        match classify_temporal_rule(&rule, &declared(&["RVALID"])) {
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
        match classify_temporal_rule(&rule, &declared(&["AWVALID", "AWREADY"])) {
            TemporalRuleDisposition::Residual { rule_id, .. } => assert_eq!(rule_id, "r_hs"),
            other => panic!("expected Residual, got {other:?}"),
        }
    }

    #[test]
    fn classify_undeclared_signal_and_nonliteral_value_are_residual() {
        let undeclared = t_rule("r_ud", vec![], vec![sigval("MISSING", "1")], None);
        assert!(matches!(
            classify_temporal_rule(&undeclared, &declared(&["OTHER"])),
            TemporalRuleDisposition::Residual { .. }
        ));
        let nonlit = t_rule("r_nl", vec![], vec![sigval("GRANT", "addr+4")], None);
        assert!(matches!(
            classify_temporal_rule(&nonlit, &declared(&["GRANT"])),
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
            classify_temporal_rule(&rule, &declared(&["GRANT"])),
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
        });
        isf.signals.insert(IsfSignal {
            name: "GRANT".to_string(),
            direction: IsfDirection::Output,
            width: 1,
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
            classify_actor_contract(&c, &declared(&["AWVALID", "AWREADY"])),
            TemporalRuleDisposition::Stage {
                name: "h1".to_string(),
                ready: "AWREADY".to_string(),
                valid: "AWVALID".to_string(),
            }
        );
        // Undeclared signal → still residual (never reference an
        // undeclared signal).
        assert!(matches!(
            classify_actor_contract(&c, &declared(&["AWVALID"])),
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
        });
        isf.signals.insert(IsfSignal {
            name: "AWREADY".to_string(),
            direction: IsfDirection::Input,
            width: 1,
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
