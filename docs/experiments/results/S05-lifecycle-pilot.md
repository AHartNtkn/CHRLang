# Stable reuse reduces allocation, but the first pilot does not establish a practical runtime gain

The 588-process pilot completes with exact allocation replay in all 84 cells. Dependency validity avoids most repeated equation work, while distinct substantive requests expose a practical overhead. The architecture comparison still needs the inferred-specialization control used in R05; generic Indexed execution is insufficient to represent that alternative.

## Material results

For eight-query batches, dependency/ordinary median runtime ratios are 0.944 before success, 0.936 after success, 0.962 before clash and 0.996 after clash. None reaches the registered 20% practical improvement threshold. After-success is consistently smaller than one, while the other repeated-operation ranges cross one. These are bounded pilot observations, not evidence of equivalence.

Dependency caching reduces requested heap traffic from 2.718 to 1.953 MiB before success and from 1.850 to 1.058 MiB before clash. Peak rises: respectively 79.4 to 83.4 KiB and 73.8 to 79.0 KiB. Avoiding equation work does not remove source matching, branch service, observation or validity costs.

The distinct-request source gives a clear adverse result: dependency/ordinary ratio 1.541 [1.489, 1.613], with traffic rising from 2.605 to 3.869 MiB and peak from 93.3 to 103.7 KiB. Exact-context caching gets no hits in any family. Its eight-query median overhead is 6–9% in most families, below the registered practical threshold; retaining whole binding contexts does not create reuse across changed choice bindings.

Work diagnostics repeat exactly. On substantive success, dependency caching reduces kernel pairs from 2,142 to 140; on clash, from 1,086 to 74. Source equations remain 46. Of 37 hits, 22 concern repeated choice equations and 15 concern the substantive request. The distinct-request source keeps only the 22 choice hits and reduces 2,174 pairs to 2,152, preserving almost all substantive work. These work counts do not imply corresponding wall-time savings.

The direct choice graph remains a relevant competitor on common failure before discrimination: its eight-query median is 0.539 ms versus ordinary scalar's 0.624 ms, with graph/ordinary ratio 0.853 [0.760, 0.977] and lower peak. It is much more expensive after discrimination. Conditional also suffers strongly after discrimination. Neither sharing implementation is declared universally preferable or inferior from these families.

## Correcting the control coverage

R05 used `specialize_inferred()` on the compiled ruleset. This pilot's Indexed configuration used generic preparation. The resulting generic Indexed losses therefore do not establish losses for the strongest known compiled control. A prospectively registered follow-up must compare inferred specialization with ordinary scalar, dependency reuse and the choice graph on these same sources and lifecycle boundaries.

The original source and binaries remain frozen. No cross-freeze timing ratio should substitute for that follow-up. Broader favorable payloads, binding invalidation sensitivity, eviction, cross-query cache ownership and profiling of remaining runtime costs remain S05 obligations.

## Measurement and validation

The [registration](../registrations/S05-lifecycle-pilot.md) fixes the six sources, seven modes, reuse1/8, five timing repetitions and two allocation repetitions. [Summary](s05-lifecycle/summary.json), [audit](s05-lifecycle/audit.json), [phase table](s05-lifecycle/phases.csv), [freeze](s05-lifecycle/freeze.json) and raw receipts preserve results. Independent complete-answer gates also check explicit expected aliases and all sixteen source outcomes; failure has zero answers and no first-answer latency.

Scalar source preparation now validates and retains shared rules once. Every query still gets a fresh arena and cache. A changed-query regression confirms that prepared-rule reuse does not reuse another query's cache identity. Ordinary source execution is included separately from intercepted Direct to charge the latter's interface work. Preparation, setup, execution/observation, cancellation and disposal are recorded; native compilation and fixture/oracle work are excluded. Requested allocation is not RSS.

No universal cache or architecture is selected. T067 remains active for the strong-control follow-up and consequential attribution.
