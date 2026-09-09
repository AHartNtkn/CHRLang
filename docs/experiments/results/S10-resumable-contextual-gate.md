# Resumable matching avoids repeated history checks while its cursor remains valid

The new contextual cursor offers each candidate once on stable history and dense propagation sources, preserving complete answers. Equality invalidation can erase that advantage: a redundant equality after every application makes all three controls repeat the same history checks. Allocation, time and retained-state costs remain unmeasured.

This is an executable T078 source/work gate, not a new baseline or an architectural selection. It adds `Prepared::start_resumable` alongside the existing eager and restarting-demand controls. Reference interpreter code is unchanged.

## The mechanism and its validity boundary

The cursor retains a depth-first occurrence prefix, the next occurrence position for each head, and the possible matching environments for each prefix. It returns complete candidates in occurrence-tuple then environment order. It does not first materialize the complete tuple set. Partial constructor descriptions retain all their alternative environments, rather than selecting only the first description.

After consumption, the cursor prunes any saved subtree whose prefix contains an occurrence no longer live. Removing occurrences cannot create an earlier matching tuple. The saved parent position then continues with the next live occurrence. Tests cover both a consumed first prefix and a consumed suffix with a kept prefix, preserving distinct occurrence identities.

Matching-head insertion resets cursors for rules that read that predicate/arity. Any processed equality work resets all cursors, using the existing eager-cache invalidation boundary. This includes redundant deductions. A source choice clones the context and its cursor; each resulting context owns its own consumption and subsequent invalidation. The cursor is private to the crate so arbitrary callers cannot resume it against unrelated stores without the executor's invalidation discipline.

The implementation introduces retained cursor frames, prefix environments and owned copies of head patterns. Branch cloning and invalidation can copy or reconstruct that state. It still retains propagation history and the existing scheduler; neither responsibility disappears merely because matching can resume.

## Direct source-level work evidence

Counts below are complete candidates offered to history/guard checking across the actual source execution, including the final search for another application. Diagnostic updates compile out when `local-work` is absent. All rows preserve independently constructed full observations checked by the scalar and each of the three contextual paths.

| Source | Eager offers | Restarting demand offers | Resumable offers |
|---|---:|---:|---:|
| One history item | 1 | 2 | 1 |
| Sixteen history items | 16 | 152 | 16 |
| Forty-eight history items | 48 | 1224 | 48 |
| Six identical occurrences, all thirty ordered pairs | 30 | 495 | 30 |
| Sixteen history items, redundant equality after each application | 152 | 152 | 152 |

For stable single-head propagation, restarting demand offers `n(n+1)/2 + n` candidates while eager and resumable offer `n`. The tests assert these exact counts. The dense duplicate source requires thirty observations despite identical argument values, so value deduplication cannot manufacture the apparent improvement.

The equality row is an explicit adverse control. Its equation changes no logical answer, but the existing conservative policy invalidates progress whenever an equality step is processed. A more precise change test might improve this case; the result is not evidence that resumable matching intrinsically needs those restarts. Any such refinement must preserve the newly enabled earlier matches demonstrated by the guard-activation test.

[Diagnostic results](s10-resumable-gate/relational-work.log), [source/work tests](../../../research/chr-relational/tests/resumable_discovery.rs).

## Correctness beyond the favorable history source

The gate checks a newly posted head reopening an exhausted earlier rule, late guard activation through equality, consumed prefixes, duplicate occurrence multiplicity and complete fresh-result observations. A unit test compares every cursor result with the established tuple/environment order both before and after partial constructor equalities settle.

The broader mixed-source gate now includes the resumable path. It exercises guards, aliases, consuming joins, history, choices, failure and fresh outputs. The composition/progress gate adds resumable execution to both certified-fusion and ineligible-query comparisons and to finite siblings beside ongoing work. The latter requires complete finite observations and continued service without spurious exhaustion.

Six relational unit tests, four demand-boundary tests and six resumable source/work tests pass in diagnostic and counter-free builds. The broad mixed test and both composition/progress tests pass in default and counter-free configurations. Strict Clippy passes for the affected relational library/tests and composition tests. [Counter-free](s10-resumable-gate/counter-free.log), [composition](s10-resumable-gate/composition.log), [counter-free composition](s10-resumable-gate/composition-counter-free.log), [Clippy](s10-resumable-gate/clippy.log), [composition Clippy](s10-resumable-gate/composition-clippy.log).

These are bounded semantic checks and work attribution. They do not measure memory, timing, cancellation latency or sustained retention, and do not prove general runtime equivalence. The cursor still reconstructs after equality and relevant insertion; it is not an incremental equality-aware join network.

## Next comparison and priority

Register a bounded lifecycle comparison of eager, restarting-demand and resumable contextual execution, with Scan and filtered conditional controls on applicable complete sources. Include stable history, dense productive joins, redundant equality resets, meaningful changing bindings, new heads, shallow sources, branch cloning, changed queries and cancellation. Charge preparation, cursor/head/environment construction, invalidation, complete delivery, retained owners and disposal. Keep work diagnostics separate from ordinary timings.

The strongest ready alternative remains support-aware conditional joining. A cost pilot for the new cursor comes first because this gate establishes a distinct mechanism with both stable and invalidated regimes, but has not determined whether its retained state repays its cost. The equality-reset result may justify a precise invalidation experiment if its complete cost is consequential; it does not justify assuming a free repair. Reconsider conditional joining and broader architecture breadth after the pilot.

T078 remains unfinished for coherent architecture comparison. Broader language properties, richer matching organizations, sustained lifetime, connected parallel work and held-out challenges remain required.
