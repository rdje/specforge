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
    SystemResetPolarity, TemporalPredicateRecord, TemporalRuleRecord,
};
use crate::ir::source::{
    AutomationConfidence, CandidateInterpretation, ResidualDecisionPacket, WidthHint,
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
}

// `(contract <name> (eventually <signal> (within <N>)))` — FSMGen ISF
// spec §11.8 shipped kind `bounded_eventually`. NOTE: `--strict --check`
// requires the nested `(within N)` subclause (the spec's flat
// `within N` prose is rejected). `(stage … (ready)(valid))` from §11.8
// is also strict-rejected and is intentionally not modelled here
// (recorded in docs/FSMGEN_FEEDBACK.md).
#[derive(Debug, Clone, PartialEq, Eq)]
struct IsfContract {
    name: String,
    signal: String,
    within: u64,
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
    #[allow(dead_code)]
    constants: Vec<IsfConstant>,
    #[allow(dead_code)]
    types: Vec<IsfTypeDef>,
    #[allow(dead_code)]
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

    pub(crate) fn render(&self) -> String {
        let mut lines: Vec<String> = Vec::new();

        lines.push(format!("(actor {}", self.actor_name));
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
                lines.push(format!("    (var {} (width {}))", v.name, v.width));
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
            // FSMGen `--strict --check` requires the nested `(within N)`
            // subclause; the spec §11.8 prose form `eventually s within N`
            // is rejected. `(stage … (ready)(valid))` is also strict-
            // rejected despite §11.8, so it is intentionally NOT emitted
            // (see docs/FSMGEN_FEEDBACK.md and ISF-TEMPORAL-LOWERING.1).
            lines.push(format!(
                "    (contract {} (eventually {} (within {})))",
                contract.name, contract.signal, contract.within
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
                lines.push(format!(
                    "{}(spawn {} {})",
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
                lines.push(format!("{}(shift-left {} {})", indent, reg, bit));
            }
            IsfTxnStep::ShiftRight { reg, bit, width } => {
                if let Some(w) = width {
                    lines.push(format!(
                        "{}(shift-right {} {} (width {}))",
                        indent, reg, bit, w
                    ));
                } else {
                    lines.push(format!("{}(shift-right {} {})", indent, reg, bit));
                }
            }
            IsfTxnStep::Complete { port } => {
                lines.push(format!("{}(complete {})", indent, port));
            }
            IsfTxnStep::AwaitAll { done_port } => {
                lines.push(format!("{}(await-all {})", indent, done_port));
            }
            IsfTxnStep::AwaitAny { done_port } => {
                lines.push(format!("{}(await-any {})", indent, done_port));
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
        let mut seen_storage_names: BTreeSet<String> = BTreeSet::new();
        let storage: Vec<IsfStorageVar> = intent_ir
            .register_records
            .iter()
            .filter_map(|r| {
                let name = sanitize_isf_name(&r.register_name.to_lowercase());
                if !seen_storage_names.insert(name.clone()) {
                    return None;
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
                Some(IsfStorageVar { name, width })
            })
            .collect();

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

        // --- ISF-TEMPORAL-LOWERING.2.2/.2.3: lower temporal_rules ---
        // Each `temporal_rule` is classified into exactly one disposition
        // (`.1` mapping #1/#3/#4). Windowed `bounded_eventually` → a
        // synthetic transaction carrying the FSMGen-strict-verified nested
        // `(contract <id> (eventually <signal> (within <N>)))`. Non-windowed
        // value/guard→drive → an actor `(rule …)`. Anything with no
        // representable supported ISF construct (HandshakeComplete,
        // `(within 0)`, no concrete value, undeclared signal, …) is
        // preserved as an explicit residual decision — never fabricated
        // (`fsmgen-contract-authority`).
        let mut temporal_isf_rules: Vec<IsfRule> = Vec::new();
        let mut temporal_residuals: Vec<ResidualDecisionPacket> = Vec::new();
        for rule in &intent_ir.temporal_rules {
            match classify_temporal_rule(rule, &signal_names) {
                TemporalRuleDisposition::Contract {
                    name,
                    signal,
                    within,
                } => {
                    all_transactions.push(IsfTransaction {
                        name: format!("txn_temporal_{}", name),
                        on_trigger: None,
                        on_steps: vec![],
                        steps: vec![],
                        complete: "done".to_string(),
                        latency_min: None,
                        latency_max: None,
                        contracts: vec![IsfContract {
                            name,
                            signal,
                            within,
                        }],
                    });
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
                        &rule.source_text,
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
        // different values, FSMGen rejects the ISF.  Keep only the first.
        {
            let mut seen: std::collections::BTreeMap<(String, String), String> =
                std::collections::BTreeMap::new();
            let mut deduped: Vec<IsfRule> = Vec::new();
            'outer: for rule in rules {
                let mut conflict = false;
                for (sig, val) in &rule.drives {
                    let key = (sig.clone(), rule.condition.clone());
                    if let Some(prev_val) = seen.get(&key)
                        && prev_val != val
                    {
                        conflict = true;
                        break;
                    }
                }
                if conflict {
                    continue 'outer;
                }
                for (sig, val) in &rule.drives {
                    let key = (sig.clone(), rule.condition.clone());
                    seen.entry(key).or_insert_with(|| val.clone());
                }
                deduped.push(rule);
            }
            rules = deduped;
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
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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
    /// Windowed `bounded_eventually` → synthetic `(transaction …
    /// (contract <name> (eventually <signal> (within <within>))))` (`.2.2`).
    Contract {
        name: String,
        signal: String,
        within: u64,
    },
    /// Non-windowed value/guard→drive → actor
    /// `(rule <name> [<condition>] (<signal> <value>))` (`.2.3` #3).
    Rule {
        name: String,
        condition: String,
        signal: String,
        value: String,
    },
    /// No representable supported ISF construct → explicit residual
    /// decision; syntax is never fabricated (`.2.3` #4,
    /// `fsmgen-contract-authority`).
    Residual { rule_id: String, reason: String },
}

/// First consequent that is a concrete `SignalValue` (a signal name *and* a
/// value to drive). The actor/stability/sample predicates name a signal but
/// carry no concrete value, so they are not a representable `(rule …)` drive.
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

/// Derive a strict-safe `(rule …)` guard from the rule's antecedents: the
/// first `SignalValue` antecedent whose signal is declared and whose value is
/// an ISF literal yields `(== <signal> <value>)`. Otherwise the empty string
/// (an unconditional rule — strict-verified accepted; the real corpus emits
/// conditionless `(rule name (sig val))`). A bare non-`==` token guard is
/// never produced (FSMGen strict rejects it — `sanitize_rule_condition`).
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
                TemporalRuleDisposition::Contract {
                    name: sanitize_isf_name(&rule.rule_id),
                    signal,
                    within: u64::from(within),
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

/// Build the explicit residual-decision packet for a temporal rule that has
/// no representable supported ISF construct (mapping #4).
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
                name: "c_resp".to_string(),
                signal: "RVALID".to_string(),
                within: 4,
            }],
        });
        isf
    }

    #[test]
    fn render_emits_spec_shaped_bounded_contract() {
        let out = isf_with_temporal_contract_transaction().render();
        // FSMGen `--strict` requires the nested `(within N)` subclause.
        assert!(
            out.contains("    (contract c_resp (eventually RVALID (within 4)))"),
            "contract shape:\n{out}"
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
            "FSMGen strict rejected the spec-shaped (stage …)/(contract …) forms"
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
                signal: "RVALID".to_string(),
                within: 8,
            }
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
}
