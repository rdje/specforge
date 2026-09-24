//=============================================================================
// Flattened Decision Tree FSM: valid_demo
// Generated using Enable-based Methodology with WEN/EN Signals
// Date: Thu Sep 24 13:18:21 2026
// 
// This implementation uses:
// - Flattened decision tree approach
// - Enable-based logic with assign statements
// - Write Enable (WEN) and Enable (EN) signals for each LHS
// - Flat Boolean expressions from DT traversal
//=============================================================================

module valid_demo (
  input  wire clk,
  input  wire rst_n,
  input  wire VALID,
  input  wire start,
  output reg  A,
  output reg  B,
  output reg  done
);

  // State encoding
  localparam TX_IDLE_0 = 2'd0;
  localparam TX_DONE_1 = 2'd1;

  // State registers
  reg [1:0] current_state, next_state;

  // State sequential logic
  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      current_state <= TX_IDLE_0;
    end else begin
      current_state <= next_state;
    end
  end

  // Internal signal declarations
  reg can_accept;
  // Internal mux helper registers
  reg A_next;
  reg B_next;
  reg done_pulse_delay_pipe;
  // Generated enable wires
  wire a_valid_en;
  wire b_valid_en;
  wire can_accept_1_en;
  wire done_1_en;
  wire next_state_tx_done_1_en;
  wire next_state_tx_idle_0_en;
  wire r_a_a_valid_en;
  wire r_a_en;
  wire r_b_b_valid_en;
  wire r_b_en;
  wire tx_done_1_done_1_en;
  wire tx_done_1_en;
  wire tx_done_1_next_state_tx_idle_0_en;
  wire tx_idle_0_can_accept_1_en;
  wire tx_idle_0_en;
  wire tx_idle_0_next_state_tx_done_1_en;

  // State and DT Enable Conditions
  assign tx_done_1_en = current_state == TX_DONE_1;
  assign tx_idle_0_en = current_state == TX_IDLE_0;
  assign r_a_en = 1'b1;
  assign r_b_en = 1'b1;

  // Unified WEN/EN Signal Generation from Phase 1 Analysis
  // DT-Specific Enable Signals from Unified Analysis


  // === DT: -r_a ===
  // A
  assign r_a_a_valid_en = r_a_en;  // A <- VALID


  // === DT: -r_b ===
  // B
  assign r_b_b_valid_en = r_b_en;  // B <- VALID


  // === DT: tx_done_1 ===
  // done
  assign tx_done_1_done_1_en = tx_done_1_en;  // done <- 1
  // next_state
  assign tx_done_1_next_state_tx_idle_0_en = tx_done_1_en;  // next_state <- TX_IDLE_0


  // === DT: tx_idle_0 ===
  // can_accept
  assign tx_idle_0_can_accept_1_en = tx_idle_0_en;  // can_accept <- 1
  // next_state
  assign tx_idle_0_next_state_tx_done_1_en = tx_idle_0_en & start;  // next_state <- TX_DONE_1

  // LHS-Level Enable Signals from Unified Analysis

  // LHS-level enables for: A
  assign a_valid_en = r_a_a_valid_en;

  // LHS-level enables for: B
  assign b_valid_en = r_b_b_valid_en;

  // LHS-level enables for: can_accept
  assign can_accept_1_en = tx_idle_0_can_accept_1_en;

  // LHS-level enables for: done
  assign done_1_en = tx_done_1_done_1_en;

  // LHS-level enables for: next_state
  assign next_state_tx_done_1_en = tx_idle_0_next_state_tx_done_1_en;
  assign next_state_tx_idle_0_en = tx_done_1_next_state_tx_idle_0_en;

  // Unified Multiplexer Logic from Phase 1 Analysis

  // Unified Multiplexer for LHS: A
  // Unified flop with mux for: A (register_out assignment)
  always_comb begin
    A_next = A;  // Default value
    if (a_valid_en) begin
      A_next = VALID;
    end
  end
  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      A <= 1'b0;
    end else begin
      A <= A_next;
    end
  end

  // Unified Multiplexer for LHS: B
  // Unified flop with mux for: B (register_out assignment)
  always_comb begin
    B_next = B;  // Default value
    if (b_valid_en) begin
      B_next = VALID;
    end
  end
  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      B <= 1'b0;
    end else begin
      B <= B_next;
    end
  end

  // Unified Multiplexer for LHS: can_accept
  // Unified combinational mux for: can_accept
  always_comb begin
    can_accept = 1'b0;  // Default value
    if (can_accept_1_en) begin
      can_accept = 1;
    end
  end

  // Unified Multiplexer for LHS: done
  // Delayed pulse logic for: done (<1, exact Q+1)
  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      done <= 1'b0;
      done_pulse_delay_pipe <= 1'b0;
    end else begin
      done <= 1'b0;
      if (done_pulse_delay_pipe) begin
        done <= 1'b1;
      end
      done_pulse_delay_pipe <= done_1_en;
    end
  end

  // Unified Multiplexer for LHS: next_state
  // Unified combinational mux for: next_state
  always_comb begin
    next_state = current_state;  // Default value
    if (next_state_tx_done_1_en) begin
      next_state = TX_DONE_1;
    end
    if (next_state_tx_idle_0_en) begin
      next_state = TX_IDLE_0;
    end
  end

  `ifndef SYNTHESIS
  always_comb begin
    assert ($onehot0({next_state_tx_done_1_en, next_state_tx_idle_0_en})) else $error("selector multi-value conflict: next_state");
  end
  `endif
endmodule