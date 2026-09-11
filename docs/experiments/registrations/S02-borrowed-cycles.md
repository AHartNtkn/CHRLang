# Borrow constructor children during finite-cycle traversal

Registered before runs. T072, first package since portfolio review.

Replace only descriptor ownership inside `Store::reaches`: borrow child slices from constructor rows instead of constructing owned names and child vectors. Preserve incidence iteration order, stack order, target checks and the visited BTreeSet. Keep the owned-descriptor path as a feature-controlled experiment reference. No cycle memoization, closedness inference or language change is included.

First run source/store/library tests with borrowed traversal and each incidence representation. Add explicit deep indirect cycles and shared-child DAGs with independently expected success/failure and fork isolation. Existing complete broad/readiness source oracles remain controlling; investigate any mismatch before costs.

Repeat the execution-attribution matrix with borrowing:144 processes across36 configurations, meter/profile builds and two repetitions, same exact allocation/advance/ownership gates and resource bounds. Compare with the prior frozen owned traversal. Require every non-cycle exclusive allocation scope and count to match; compiled controls remain identical. Diagnostic wall clocks do not establish speed gains.

Then repeat the earlier complete-source ordinary timing pilot using borrowed traversal, both incidence representations and compiled controls:72 entry processes,60 excluded warmups,300 primary processes across the same12 cases and five execution variants, five rotated blocks, seed7213. Match its paired ownership interpretation with the previous owned pilot, but do not treat separate campaigns as closely paired timing. The new pilot compares bucket/schedule/compiled choices within borrowed traversal; it establishes current complete cost against compiled. Any direct borrowed-versus-owned timing claim requires a separately registered interleaved confirmation. Both campaigns freeze sources and binaries before execution, use60second CPU/wall and1GiB limits and100,000 advances per query. Input construction and independent validation remain outside the established lifecycle endpoint, with preparation, execution/observation and all disposal included.

Use results to decide whether borrowing removes a consequential ownership burden and whether it changes the compiled gap. Reassess discovery invalidation, readiness traversal, graph attribution and direct solving at the result; no default adoption or goal completion follows automatically.
