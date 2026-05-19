//! Protocol-structured knowledge-graph nodes (R16-KG-PROTOCOL-ONTOLOGY).
//!
//! Per the `R16-KG-PROTOCOL-ONTOLOGY.1` design (see
//! `docs/tasks/R16-KG-PROTOCOL-ONTOLOGY.md`): first-class `Channel` /
//! `ProtocolPhase` / `Transaction` / `HandshakePair` records so
//! `IntentIR` is a systematic projection of protocol structure, not just
//! a flat actor/signal graph. Edges are modelled as typed references
//! (matching the existing actor graph), not a raw edge soup.
//!
//! `.2` scope (this module): the typed model + serde + count helpers +
//! unit tests. The `ProtocolGraph` field is added additively to
//! `SemanticIr`/`IntentIr` and is **empty and unpopulated** (serde-
//! skipped while empty ⇒ zero artifact churn, parity-preserving — the
//! same discipline as `R16-CONTRACT-IR.2`). Wiring the projection is
//! `.3`; recovering channels/phases/transactions from the PDF is the
//! extraction trees' job (`#3`/`#4`/`#6`), an explicit Non-Goal here.
//!
//! NOTE: `ProtocolPhase` is **protocol-stage** granularity (APB
//! setup/access; address/data/response) and is distinct from the
//! temporal model's `TickPhase` (clock-edge granularity); a
//! `ProtocolPhase` may span many clock ticks.

use serde::{Deserialize, Serialize};

/// Semantic role of a protocol channel (protocol-neutral).
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChannelRole {
    Address,
    Data,
    Response,
    Request,
    Sideband,
    Mixed,
}

/// A named protocol channel grouping related boundary signals
/// (e.g. AXI `AW`/`W`/`B`/`AR`/`R`; APB transfer; TileLink `A`..`E`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Channel {
    pub channel_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(default)]
    pub signal_names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<ChannelRole>,
}

/// A protocol phase (protocol-stage granularity — NOT a clock
/// `TickPhase`). `order` is the intra-channel/transaction sequence.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolPhase {
    pub phase_id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
    pub order: u32,
}

/// A protocol transaction binding channels + phases + ordering.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub transaction_id: String,
    pub name: String,
    #[serde(default)]
    pub channels: Vec<String>,
    #[serde(default)]
    pub phases: Vec<String>,
    /// `ordered-before` edges: transaction ids that must precede this one.
    #[serde(default)]
    pub ordered_before: Vec<String>,
}

/// A ready/valid handshake pair — the canonical transfer point; ties to
/// a ContractIR `HandshakeBarrier` obligation / `EventExpr::HandshakeFire`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandshakePair {
    pub pair_id: String,
    pub valid_signal: String,
    pub ready_signal: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel: Option<String>,
}

/// The protocol-structure projection of the KG, carried additively on
/// `SemanticIr`/`IntentIr`. Empty until the extraction trees populate it.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolGraph {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<Channel>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub phases: Vec<ProtocolPhase>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<Transaction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub handshakes: Vec<HandshakePair>,
}

impl ProtocolGraph {
    /// True when no protocol structure has been recovered yet. Used by
    /// `#[serde(skip_serializing_if = …)]` so an empty graph adds zero
    /// bytes to any artifact (parity-preserving).
    pub fn is_empty(&self) -> bool {
        self.channels.is_empty()
            && self.phases.is_empty()
            && self.transactions.is_empty()
            && self.handshakes.is_empty()
    }

    /// `(channels, phases, transactions, handshakes)` counts — the
    /// validation-count surface (wired into `specforge validate` at
    /// `.4`; meaningless while empty so not reported in `.2`).
    pub fn counts(&self) -> (usize, usize, usize, usize) {
        (
            self.channels.len(),
            self.phases.len(),
            self.transactions.len(),
            self.handshakes.len(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_graph_is_empty_and_zero_counts() {
        let g = ProtocolGraph::default();
        assert!(g.is_empty());
        assert_eq!(g.counts(), (0, 0, 0, 0));
    }

    #[test]
    fn populated_graph_is_not_empty_and_counts() {
        let g = ProtocolGraph {
            channels: vec![Channel {
                channel_id: "ch_aw".into(),
                name: "AW".into(),
                actor: Some("Manager".into()),
                signal_names: vec!["AWVALID".into(), "AWREADY".into()],
                role: Some(ChannelRole::Address),
            }],
            phases: vec![ProtocolPhase {
                phase_id: "ph_setup".into(),
                name: "setup".into(),
                channel: Some("ch_aw".into()),
                order: 0,
            }],
            transactions: vec![Transaction {
                transaction_id: "txn_write".into(),
                name: "write".into(),
                channels: vec!["ch_aw".into()],
                phases: vec!["ph_setup".into()],
                ordered_before: vec![],
            }],
            handshakes: vec![HandshakePair {
                pair_id: "hs_aw".into(),
                valid_signal: "AWVALID".into(),
                ready_signal: "AWREADY".into(),
                channel: Some("ch_aw".into()),
            }],
        };
        assert!(!g.is_empty());
        assert_eq!(g.counts(), (1, 1, 1, 1));
    }

    #[test]
    fn serde_round_trips_and_empty_skips() {
        let empty = ProtocolGraph::default();
        // Empty graph serialises to `{}` (all fields skip_if_empty).
        assert_eq!(serde_json::to_string(&empty).unwrap(), "{}");
        let back: ProtocolGraph = serde_json::from_str("{}").unwrap();
        assert_eq!(empty, back);

        let g = ProtocolGraph {
            channels: vec![Channel {
                channel_id: "c".into(),
                name: "C".into(),
                actor: None,
                signal_names: vec!["S".into()],
                role: Some(ChannelRole::Data),
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&g).unwrap();
        let back: ProtocolGraph = serde_json::from_str(&json).unwrap();
        assert_eq!(g, back);
    }
}
