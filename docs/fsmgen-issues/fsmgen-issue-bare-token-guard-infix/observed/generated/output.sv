//=============================================================================
// Flattened Decision Tree FSM: demo
// Generated using Enable-based Methodology with WEN/EN Signals
// Date: Sun May 17 22:22:53 2026
// 
// This implementation uses:
// - Flattened decision tree approach
// - Enable-based logic with assign statements
// - Write Enable (WEN) and Enable (EN) signals for each LHS
// - Flat Boolean expressions from DT traversal
//=============================================================================

module demo (
  input  wire clk,
  input  wire rst_n,
  input  wire true,
  output reg  sig_a
);

  // State encoding

  // No state registers needed - FSM contains only decision trees

  // Internal signal declarations
  // Internal mux helper registers
  reg sig_a_next;
  // Generated enable wires
  wire sig_a_1_en;
  wire unconditional_rule_en;
  wire unconditional_rule_sig_a_1_en;

  // State and DT Enable Conditions
  assign unconditional_rule_en = true;

  // Unified WEN/EN Signal Generation from Phase 1 Analysis
  // DT-Specific Enable Signals from Unified Analysis


  // === DT: -unconditional_rule ===
  // sig_a
  assign unconditional_rule_sig_a_1_en = unconditional_rule_en & 1'b1;  // sig_a <- 1

  // LHS-Level Enable Signals from Unified Analysis

  // LHS-level enables for: sig_a
  assign sig_a_1_en = unconditional_rule_sig_a_1_en;

  // Unified Multiplexer Logic from Phase 1 Analysis

  // Unified Multiplexer for LHS: sig_a
  // Unified flop with mux for: sig_a (register_out assignment)
  always_comb begin
    sig_a_next = sig_a;  // Default value
    if (sig_a_1_en) begin
      sig_a_next = 1;
    end
  end
  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      sig_a <= 1'b0;
    end else begin
      sig_a <= sig_a_next;
    end
  end
endmodule
