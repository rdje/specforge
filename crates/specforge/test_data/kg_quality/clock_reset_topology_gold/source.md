# Clock And Reset Topology

Signal ACLK is input width 1.

Signal ARESETN is input width 1.

Clock ACLK.

Reset ARESETN is asynchronous active low.

The ACLK clock gate CGATE0 feeds the Requester branch.

The two-stage reset synchronizer RSTSYNC0 feeds ARESETN to the Requester.

The ARESETN reset tree targets the Requester registers and Completer registers.

The ACLK clock gate policy should avoid glitches.

ARESETN may use a synchronizer in some implementations.
