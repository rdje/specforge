# Clock And Reset Contract Scope

Signal ACLK is input width 1.

Signal ARESETN is input width 1.

Clock ACLK.

Reset ARESETN is asynchronous active low.

This protocol PDF defines the boundary-visible clock and reset contract for RTL and verification IP (VIP).

The final chip physical clock tree is built by the integrating SoC team.

The final chip physical reset tree is built by the integrating SoC team.

Clock-tree construction depends on project-local PLLs, clock generators, power domains, floorplan, and methodology.

Reset-tree construction depends on project-local reset controllers, DFT constraints, scan constraints, and chip integration policy.

Those physical implementation trees are not specified by this protocol PDF.
