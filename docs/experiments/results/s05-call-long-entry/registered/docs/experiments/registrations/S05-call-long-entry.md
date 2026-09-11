# Longer prepared sessions: source and ownership entry

Register before runner changes and runs. The current runner's four-query bound and
fixed diagnostic arrays are fixture choices, not language limits. Extend query
counts and phase recording while keeping the same measured phases and exact
consumer accounting. Show the existing runner rejecting a 64-query case before
repair, then verify actual completion and disposal beyond four queries.

Keep policy 0 (repeated depths, changing variable namespaces) and policy 1 (depths
grow with each query). Add explicitly named policy 2: cycle four depth pairs,
changing namespaces on every query. This models a bounded reusable query working
set; it does not replace or relabel the growing-distinct policy. Exercise growing
distinct at 16 queries separately, because its computation also grows with depth.
The runner accepts positive query counts through 32,768 and depths through 2,048;
these are execution resource bounds, not language claims.

Reuse the shared runner for trace, Direct, scan, indexed, inferred, generated and
generated-inferred modes. Fresh frozen ordinary/meter binaries only. Complete
answer checks and independent scalar evaluation remain outside measured intervals
for every query. Prepared state is reused, outputs can survive its disposal, and
first-answer cancellation is checked. No comparative timing verdicts in this gate.

Main qualification: all seven modes, four families, depths 32/128, 64 queries,
output retention 0/all, complete/cancel, and policies 0/2: 448 cells. Add all modes
and families at depth32, 16 growing-distinct queries, drop outputs, complete:
28 cells. Scaling sentinels: all modes, family0/depth128, 8,192 queries, drop
outputs, complete, policies0/2: fourteen cells. Total 490 cells; two meter and one
ordinary repetition =1,470 processes. Run smaller cells before sentinels.

Each process has 1 GiB address space, 120 CPU seconds and 180 wall seconds, and
100,000 ticks per query. Validate exact repeated requested/peak/consumer bytes,
phase continuity and final root restoration. Require equivalent counts and
consumer bytes across modes. Preserve failures and repair consequential causes;
never record cutoffs as completed costs. Compiler builds are offline, at most two
Cargo jobs, with 120 seconds per build invocation. Freeze sources and binaries.

After qualification, use these actual session policies and scaling observations
to register replicated user-program compilation and amortization, including the
stronger Direct/scan/inferred controls and favorable/adverse source cases. Treat
compiler costs and the fourteen exploratory inferred-generated opportunities as
unresolved until that comparison runs.
