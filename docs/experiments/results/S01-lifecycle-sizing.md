# The first native lifecycle sizes fit the resource bounds

All 280 sizing processes completed, validating 2,380 changed queries without a cutoff. The registered workload sizes are feasible for a comparative pilot. This single-repetition screen does not establish a runtime winner or a compilation crossover.

The [prospective sizing registration](../registrations/S01-lifecycle-sizing.md) covers five source families, seven execution modes, two link configurations, base sizes eight and 32, and one or sixteen queries per prepared ruleset. The matrix runs in seeded shuffled order. Every complete answer is checked against independent scalar source execution, and every phase total reconciles.

## What the sizes mean

Chain size counts edges; payload size counts opaque variables; subscription size counts joined paths. In dispatch16 and dispatch64, size instead selects the matching rule modulo rule count. Increasing that value changes competition position, not the amount of query state. These are distinct causal axes, not interchangeable workload weights.

The new dispatch64 artifact was generated from rules without query inputs and separately compiled under ordinary linking and ThinLTO. Both reuse the fixed runtime from the preceding gate. Its source and executable hashes are recorded alongside the [matrix summary](s01-lifecycle-sizing/summary.json). The existing runtime and executable hashes were verified before the matrix; runtime hashes were checked again afterward.

## Feasibility findings

The largest recorded execution interval was 1.655 ms. The largest completed-query sum was 1.874 ms, and the largest sixteen-query lifecycle sum was 24.852 ms. Source construction reached 0.125 ms and preparation 0.358 ms. These are maxima across exploratory observations, not representative estimates or comparisons between modes.

No cell reached its 60-second process limit, 1 GiB runtime address-space ceiling or two-million-step service limit. Both dispatch64 compilations also completed within their registered compiler limits. Process wall time and child CPU are retained separately; they include harness work excluded from the runtime sums.

The short intervals make single observations inadequate for ranking. Confirmation needs prospectively chosen repetition and a variability check, preserving phase timing and the distinction between process duration and runtime work. Query batches can increase measured work, but cannot silently erase the cold-query endpoint or include validation as engine cost.

## What is still required before the complete cost contrast

The current seven modes isolate generated execution and update planning, with eligible specialization controls. They do not include the existing source-equivalent subscription/retained implementations. Those controls must participate on subscription sources before a recommendation about matching organization; a generated-versus-generic result alone would leave that question unanswered.

Requested-allocation diagnostics must run separately from ordinary-allocator timing and charge the installed prepared/native update plans. The runner also needs cancellation accounting and artifact-lifetime treatment for the corresponding lifecycle claims. Existing completion and prepared-state disposal checks do not prove those endpoints.

The next implementation should establish those shared source and accounting boundaries, reusing the existing meter and retained engines where credible. Then register the comparative pilot, including compiler repetition, changing-query reuse sensitivity, practical thresholds and contrary cases. The sizing evidence supplies feasible starting cells; it does not justify restricting reuse to sixteen queries when a larger regime could change compilation amortization.

Direct pull-tabbing and derivation reuse remain the strongest distinct ready alternative. The four-package breadth review's next selection boundary is the first complete T070 cost contrast. These missing controls are prerequisites for that contrast, not a reason to start an unrelated access-policy refinement. T070 and the architecture goal remain active.

The [validation receipt](s01-lifecycle-sizing/validation.json) independently rereads every process record and verifies current source and new artifact hashes. The engine, generator and independent reference sources are unchanged by this sizing experiment. No measurements from this matrix are promoted to confirmatory results.
