# S05: add the known inferred-specialization control

The initial S05 pilot included generic Indexed execution but omitted the inferred single-head specialization used in R05. Test that stronger control before interpreting the architecture ranking. This follow-up is registered before its timings; the original pilot remains frozen.

Use the exact six source families and query-tag variation of [the initial registration](S05-lifecycle-pilot.md). Compare four configurations: ordinary scalar, dependency cache capacity32, direct choice graph, and compiled Global/Indexed with `specialize_inferred()`. Verify every source predicate offered for specialization is eligible and run independent full-answer gates for all families. Prepared timing includes both initial plan construction and specialization; the intermediate prepared value is dropped within that phase.

Six families × four configurations × reuse1/8 = **48 cells**. Five ordinary-allocator counter-free timing repetitions and two separate allocation repetitions = **336 processes**. Seed20260911, shuffled cells in each repetition, serial pinned execution, 60-second wall and 1 GiB address-space limits, two-million engine-service bound. Stop and investigate any failure or cutoff. Repeated work diagnostics remain separate.

All lifecycle phases, warmups, explicit failure handling, disposal checks, exact diagnostic replay and exclusions are unchanged from the initial registration. Compare five within-block ratios against ordinary, Dependencies, graph and specialized execution. The practical threshold remains a median at least 20% from one with every pair on the same side. Report smaller differences and uncertainty without declaring equivalence. No cross-freeze timing ratio or workload-weighted score.

This can establish whether specialization changes the apparent scalar/cache/sharing ordering on the admitted sources. It cannot establish language-wide superiority or reject broader cache validity/lifetime policies. If specialized and sharing controls expose a consequential contrary regime, retain it and investigate its mechanism rather than selecting a universal incumbent.
