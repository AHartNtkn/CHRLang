# Sparse elimination helps cliques, but does not yet beat the stronger controls

Sparse traversal improves many equality-clique comparisons against Cartesian projection. Enumeration still wins every clique comparison, and the apparent gains on fully populated relations disappear against the existing separable solver. Unequal-name relations merit a longer prepared-session experiment because their measured query work is cheaper after projection.

## Complete costs change the work-count conclusion

Both projection modes use the same greedy elimination order. The matrix includes equality stars, equality cliques, unequal-name stars and a fully populated star relation. Each varies coordinate count, duplicate choices, output aliases, one or four queries, retained outputs and cancellation. These are weighted finite-answer comparisons: tuple multiplicities are preserved, but expanding every raw source derivation is not the measured output protocol.

The 256 scenarios produce these ordinary timing verdicts for sparse traversal. A gain requires at least a 10% paired median improvement and all five repetitions faster; losses use the symmetric rule.

| Relation family | Versus Cartesian: gain / loss / uncertain | Versus enumeration: gain / loss / uncertain |
|---|---:|---:|
| Equality star | 2 / 1 / 61 | 0 / 63 / 1 |
| Equality clique | 36 / 1 / 27 | 0 / 64 / 0 |
| Unequal-name star | 1 / 3 / 60 | 0 / 64 / 0 |
| Fully populated star | 0 / 0 / 64 | 3 / 52 / 9 |

The clique improvements establish that sparse traversal can matter even when both solvers choose the same order. Its median runtime ratios against Cartesian projection span 0.331–1.433 across the clique scenarios. Against enumeration they span 2.515–7.485, so the internal improvement does not establish complete competitiveness.

Allocation does not supply a general advantage either. Sparse traversal requests fewer bytes than Cartesian projection in 32 scenarios—all six-coordinate clique cases—and more in the other 224. Its peak is equal in 161 scenarios and higher in 95. Against enumeration, both requested allocation and peak ownership are higher throughout.

## A stronger control overturns the dense gains

The fully populated relation compares a two-coordinate expression with itself. That is useful for testing a traversal with no pruning, but a source-aware implementation can recognize the predicate as true. The follow-up uses the existing separable solver after checking that fact, charging the inspection, temporary preparation and disposal.

A fresh comparison across all 64 dense scenarios gives sparse projection 64 losses against separable solving. It requests 5.28–11.87 times as many bytes and peaks at 1.64–2.88 times the live ownership. The repeated enumeration contrast qualifies one different gain; none of the initial three gains is confirmed under the same settings. Those weak-control gains provide no reason to prefer sparse projection for this source.

For six coordinates, duplicate domain choices, four queries, immediate consumption and full exhaustion, the fresh complete runtime medians are:

| Executor | Runtime | Requested allocation | Peak ownership |
|---|---:|---:|---:|
| Sparse projection | 82.74 µs | 87,820 bytes | 14,906 bytes |
| Cartesian projection | 64.42 µs | 86,900 bytes | 14,906 bytes |
| Enumeration | 117.55 µs | 21,460 bytes | 8,948 bytes |
| Separable solving | 16.55 µs | 11,576 bytes | 5,720 bytes |

This uses an existing solver and a source fact, not another baseline implementation. It illustrates why avoiding unnecessary work can dominate choosing a better traversal.

## Preparation and query work suggest different next experiments

In the corresponding six-coordinate clique case, ordinary median complete time falls from 171.62 µs for Cartesian projection to 90.76 µs for sparse projection; enumeration takes 19.78 µs. Sparse preparation alone takes 77.92 µs. Separate instrumented samples place about 46% of sparse preparation in relation construction and 33% in elimination. The unchanged stages vary between diagnostic runs, so those measurements locate costs rather than establish precise speedup ratios.

Unequal-name relations have a more promising reuse pattern. In the same four-query configuration, sparse preparation takes 37.10 µs versus enumeration's 6.56 µs. But query setup, first observation and remaining output together take median phase sums of about 7.68 µs for sparse projection versus 23.60 µs for enumeration. The complete session still loses, 49.95 versus 36.20 µs.

That is a reason to measure longer changing-query sessions, not to extrapolate a crossover. Extend the fixture's current one/four-query choices and preserve explicit preparation, consumer and disposal costs. Include stars and cliques as adverse controls, aliases, retained answers and cancellation. A measured crossover could change whether prepared solving belongs in a coherent architecture; another short-batch precision run is less valuable.

## Evidence and next decision

The [registration](../registrations/S06-sparse-cost.md) precedes both timing matrices and the dense-control challenge. The initial matrix completes 1,536 allocation and 3,840 ordinary processes; the challenge adds 512 allocation and 1,280 ordinary processes. Repeated allocation agrees, consumer ownership matches across controls and final ownership returns to its starting value. Every actual weighted output is checked outside measured phases. One initial comparison fails the clock qualification and remains uncertain.

The dense fixture also passes an analytical multiplicity test for all four executors. Existing projection and connected-source tests, the fixture test and scoped Clippy pass. Twenty-four separate preparation-clock runs provide the stage attribution.

[Initial allocation](S06-sparse-allocation.json), [initial timing](S06-sparse-cost.json), [dense allocation](S06-sparse-dense-allocation.json), [dense timing](S06-sparse-dense-cost.json) and [preparation diagnostics](S06-sparse-preparation.json) contain the compact numerical results. `research/chr-structural/experiments/sparse_cost.py` runs the main matrix; `dense` selects the follow-up and `profile` selects preparation diagnostics.

Keep T076 active for observed prepared-session amortization. Cheaper relation construction remains a possible subsequent repair; broader caller observers and selective retention remain ready alternatives. This is package two since the portfolio review. The research goal remains active.
