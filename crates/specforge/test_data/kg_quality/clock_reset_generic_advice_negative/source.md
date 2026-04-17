# Clock And Reset Generic Advice

Signal ACLK is input width 1.

Signal ARESETN is input width 1.

Clock ACLK.

Reset ARESETN is asynchronous active low.

The ACLK clock gate policy should avoid glitches.

The ARESETN reset tree should avoid glue logic.

ARESETN may use a synchronizer in some implementations.

Reset assertion should be asynchronous and reset release should be synchronous to ACLK.
