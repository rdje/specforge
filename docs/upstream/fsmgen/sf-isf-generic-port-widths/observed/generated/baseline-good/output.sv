//=============================================================================
// Flattened Decision Tree FSM: bus_demo
// Generated using Enable-based Methodology with WEN/EN Signals
// Date: Fri Sep 25 06:57:31 2026
// 
// This implementation uses:
// - Flattened decision tree approach
// - Enable-based logic with assign statements
// - Write Enable (WEN) and Enable (EN) signals for each LHS
// - Flat Boolean expressions from DT traversal
//=============================================================================

module bus_demo (
  input  wire clk,
  input  wire rst_n,
  input  wire start,
  output reg  [31:0] WDATA,
  output reg  done
);

  // State encoding
  localparam TX_IDLE_0 = 2'd0;
  localparam TX_SET_1 = 2'd1;
  localparam TX_DONE_2 = 2'd2;

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
  reg [31:0] WDATA_next;
  reg done_pulse_delay_pipe;
  // Generated enable wires
  wire can_accept_1_en;
  wire done_1_en;
  wire next_state_tx_done_2_en;
  wire next_state_tx_idle_0_en;
  wire next_state_tx_set_1_en;
  wire tx_done_2_done_1_en;
  wire tx_done_2_en;
  wire tx_done_2_next_state_tx_idle_0_en;
  wire tx_idle_0_can_accept_1_en;
  wire tx_idle_0_en;
  wire tx_idle_0_next_state_tx_set_1_en;
  wire tx_set_1_en;
  wire tx_set_1_next_state_tx_done_2_en;
  wire tx_set_1_wdata__7_en;
  wire wdata__7_en;

  // State and DT Enable Conditions
  assign tx_done_2_en = current_state == TX_DONE_2;
  assign tx_idle_0_en = current_state == TX_IDLE_0;
  assign tx_set_1_en = current_state == TX_SET_1;

  // Unified WEN/EN Signal Generation from Phase 1 Analysis
  // DT-Specific Enable Signals from Unified Analysis


  // === DT: tx_done_2 ===
  // done
  assign tx_done_2_done_1_en = tx_done_2_en;  // done <- 1
  // next_state
  assign tx_done_2_next_state_tx_idle_0_en = tx_done_2_en;  // next_state <- TX_IDLE_0


  // === DT: tx_idle_0 ===
  // can_accept
  assign tx_idle_0_can_accept_1_en = tx_idle_0_en;  // can_accept <- 1
  // next_state
  assign tx_idle_0_next_state_tx_set_1_en = tx_idle_0_en & start;  // next_state <- TX_SET_1


  // === DT: tx_set_1 ===
  // WDATA
  assign tx_set_1_wdata__7_en = tx_set_1_en;  // WDATA <- 7
  // next_state
  assign tx_set_1_next_state_tx_done_2_en = tx_set_1_en;  // next_state <- TX_DONE_2

  // LHS-Level Enable Signals from Unified Analysis

  // LHS-level enables for: WDATA
  assign wdata__7_en = tx_set_1_wdata__7_en;

  // LHS-level enables for: can_accept
  assign can_accept_1_en = tx_idle_0_can_accept_1_en;

  // LHS-level enables for: done
  assign done_1_en = tx_done_2_done_1_en;

  // LHS-level enables for: next_state
  assign next_state_tx_done_2_en = tx_set_1_next_state_tx_done_2_en;
  assign next_state_tx_idle_0_en = tx_done_2_next_state_tx_idle_0_en;
  assign next_state_tx_set_1_en = tx_idle_0_next_state_tx_set_1_en;

  // Unified Multiplexer Logic from Phase 1 Analysis

  // Unified Multiplexer for LHS: WDATA
  // Unified flop with mux for: WDATA (register_out assignment)
  always_comb begin
    WDATA_next = WDATA;  // Default value
    if (wdata__7_en) begin
      WDATA_next = 7;
    end
  end
  always_ff @(posedge clk or negedge rst_n) begin
    if (!rst_n) begin
      WDATA <= 32'h00000000;
    end else begin
      WDATA <= WDATA_next;
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
    if (next_state_tx_done_2_en) begin
      next_state = TX_DONE_2;
    end
    if (next_state_tx_idle_0_en) begin
      next_state = TX_IDLE_0;
    end
    if (next_state_tx_set_1_en) begin
      next_state = TX_SET_1;
    end
  end

  `ifndef SYNTHESIS
  always_comb begin
    assert ($onehot0({next_state_tx_done_2_en, next_state_tx_idle_0_en, next_state_tx_set_1_en})) else $error("selector multi-value conflict: next_state");
  end
  `endif
endmodule