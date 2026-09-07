# E11 initial symbolic conformance and construction diagnosis

Eight equality/guard cases now match E00 completely at the registered finite bounds, with no resource-cutoff or still-active paths. Successful models pass independent per-state witness replay. This establishes a small conformance result; it does not complete E11 or compare architecture performance.

## Reproduction

Source freezes: initial harness `76bd779`, static guard shortcut `fe798d6`, solver-assisted construction pruning `f8ab0bd`. Python 3.11 with pinned z3-solver 5.1.0.0. Generate the manifest with `cargo run --quiet -p chr-symbolic-fixtures --example export_cases`, then run `research/chr-symbolic/run_conformance.py MANIFEST OUTPUT` using the isolated Python environment. Each of eight fresh processes has a 60-second limit; bounds are T=8 transitions, N=6 heap nodes, O=3 historical occurrences, P=3 pending goals and U=4 equation microsteps.

[Initial data](E11-exploratory-v1.jsonl), [static shortcut data](E11-exploratory-v2.jsonl), [reachability-pruned data](E11-exploratory-v3.jsonl), [replay](E11-exploratory-v3-replay.jsonl).

The initial and static-shortcut runs each completed six cases with full agreement and timed out on U05/U06. The reachability-pruned run completes all eight. U01/U04 correctly have no answers; U06 has two; each other case has one. All non-time fields replay exactly across eight further runs. Every resource-cutoff and transition-cutoff predicate is unsatisfiable at these bounds. Empty output alone would not establish that result.

## What changed the next action

A cProfile diagnostic of U01 at four transitions recorded 63,819,631 Python calls and 19.514 profiled seconds. The compiler constructed 18 unification services for a path containing one source equation. Unification construction consumed 17.846 seconds, including 14.868 seconds in occurs checks. Reproduce with `python -m cProfile -o PROFILE research/chr-symbolic/probe.py MANIFEST U01-clash --transitions 4` at the initial source freeze.

Skipping statically false services makes U01–U04 construct in under one diagnostic second. It does not resolve U05/U06. Solver-assisted pruning then asks whether source-action activation predicates are possible under the current prefix formula. Only unsat permits omission; unknown retains the guarded relation. This preserves every reachable source alternative. It also prevents construction of repeated effects after a prefix has already made their guards impossible.

In the pruned run, U05/U06 each perform 14 pruning checks and prove 11 guards unreachable. G02 performs 41 checks and proves 33 unreachable. Construction takes roughly 0.09–0.78 seconds and observation roughly 0.22–0.37 seconds across this matrix. These are exploratory diagnostics, not confirmatory speed ratios: parts of the initial run overlapped test/profile processes. The profile attribution and passing fixed-bound conformance are the relevant evidence.

## Scope and remaining work

The independent direct checker separately agrees with all 64 E00 cases over 2,771 transitions. That broader check does not transfer automatically to the symbolic compiler. The symbolic machine currently has four dedicated scenario tests plus this eight-case expansion. Full registry/application coverage, exact resource attribution, increasing bounds, compatible-prefix reuse, a credible equal-bound direct control and registered comparative measurements remain active.

Construction pruning is part of compilation and must be charged, including its solver calls. Deterministic datatype projection naming remains a possible secondary cost. These small results do not establish scaling, fair unbounded enumeration, faster synthesis or a production architecture preference. Broader conformance and application probes are the next useful investigation; neither initial timeouts nor their resolution closes the direction.
