# Suspended execution now preserves residual source occurrences

**Unmatched calls now appear in complete answers with their aliases and multiplicity intact.** Source-call identity and result identity are represented separately, so a tail call can replace an occurrence while preserving its output variable. This advances the demand evaluator's source semantics; it provides no performance ranking.

Current extension: [shared-resource claims](S03-demand-resource.md) now have bounded source evidence; the next selection is recorded in the [breadth review](S03-first-breadth-review.md).

## What changed and why

Every query, producer and tail call is registered once with its activation context. A recorded application result justifies consumption of that call in compatible contexts. Without such a result, the occurrence remains available for the final residual store. An unknown input or a known constructor nonmatch does not mean failure.

A call's logical output has a separate node from the reference used to demand its result. This permits a residual occurrence to mention its own output without forming a demand cycle. Tail calls inherit that logical output; independent producers get distinct outputs. Full observation resolves query roots and residual arguments together.

A publication attempt retries whenever it discovers another source application or choice. Misses are not cached as permanently disabled calls. Within the current acyclic, single-writer fragment, demanding dependencies and then scanning all active call obligations establishes the tested quiescent answers. General mutable bindings and competing consumers will require a stronger activation and validity argument.

## Evidence

The [registered gate](../registrations/S03-demand-residual-gate.md) adds eight finite source inputs: two equal-input residual cases, tail replacement, one matched/one residual choice, later producer activation, independent failure, a producer's residual, and inactive choice-arm calls. They preserve joint output/residual aliases, distinct occurrences and raw answer multiplicity.

All new cases agree with hand-derived answers and five execution paths: the independent scalar oracle, compiled global scanned and indexed controls, direct named-choice graph, and suspended evaluator. The shared harness also applies those controls to the preceding finite suspended-source cases. The complete suite has **14 passing tests** with default features and with this test package's default metrics disabled. Clippy passes with warnings denied. The scalar reference was unchanged.

The [default log](s03-demand-residual/default.log), [metrics-disabled log](s03-demand-residual/metrics-off.log) and [executable witnesses](../../../research/chr-direct-conditional/tests/suspended_source.rs) preserve the results. Processes completed within the registered 60-second bound. These are correctness runs, not timing samples.

## A consequential scheduling correction

Registering tail calls exposed a failure in the growing obligation scan. A continuing computation could append another call just before the scan reached its current endpoint, preventing it from returning to an earlier failure. The existing independent-failure and constructor-refutation witnesses caught this regression.

Each service round now fixes its endpoint when the round begins. Newly created calls participate in a subsequent round, so extending the obligation list cannot indefinitely extend the current round. The finite-sibling, independent-failure, nested-failure and constructor-refutation witnesses all pass. This establishes those bounded service cases; recursive traversal and growing rounds still need cost and sustained-service measurements.

A producer-residual witness also caught duplicate occurrence registration during implementation. Call creation now owns registration, and producer construction owns its fresh logical output. This directly checks the responsibility boundary rather than relying only on ground output values.

## Architectural consequence and next work

The checked demand fragment now has complete residual answers as well as completed value answers. It retains occurrence records and context-specific application results; result caching alone is insufficient to establish which source occurrences survive. This is necessary complexity to charge in a later comparison, not evidence that it is prohibitively expensive.

T071 next needs shared-resource claims and source-rule competition around these suspended applications. Current eligibility still excludes overlapping clauses, multihead/kept rules, guards, multiple output writers and cyclic producer dependencies. General residual constraints outside the checked call fragment are also not admitted. These are implementation/certificate boundaries, not selected language restrictions.

Local pull-tab transformation or a relevant equivalence argument, reusable derivation templates across distinct applications, full lifecycle costs and sustained retention remain unresolved. Contextual/local-rewrite integration remains the strongest distinct following alternative. Another pure-value refinement should not displace the effect integration needed to make this candidate an architectural competitor.
