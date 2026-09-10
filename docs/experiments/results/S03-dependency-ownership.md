# Miss reuse saves allocation on misses, but does not select demand execution

Within-turn miss reuse lowers requested allocation in all 24 miss scenarios for each successful-result policy. It adds allocation in the other 36 scenarios. All 2,880 lifecycle processes preserve independent answers and release their task-owned heap after consumer disposal.

Continue T071 into clock qualification and a prospectively registered complete-cost comparison. Demand execution has favorable allocation cases against Scan, but retains more peak heap throughout this matrix. Allocation savings alone cannot select the executor or its cache policy.

## The comparison and its controls

The source families are plain calls, known hits, known misses, delayed hits and delayed misses. Delayed families use a call output as the key of a passive resource. Sizes 8/128 and both arrival orders exercise shallow and substantial discovery. Four changing queries, alternating values a/b, reuse each preparation.

The six demand modes combine CurrentContext, StaticBirth or MatchDependencies successful-result reuse with miss reuse off/on. Compiled Scan and Indexed execution are independent complete controls. The existing [miss-work experiment](S03-miss-reuse.md) already establishes the sound within-turn cache boundary and an adverse stale-miss mutation. This experiment measures its ownership and allocation consequences without changing that cache implementation.

Each configuration uses immediate consumer release, a window retaining two answer batches, or retained-all answers. Separate cancellation configurations cancel even queries after one service turn and complete the odd queries with the same preparation. There are 960 configurations, each run twice with the allocation meter and once with the ordinary allocator. Cancellation comparisons establish ownership and reuse; their unequal completed work is excluded from the allocation rankings below.

## Saved discovery can mean far less allocation

The size-128 delayed-miss witness, original arrival order and retained-all consumer, requests the following bytes over four complete changing queries:

| Path | Requested allocation | Peak live heap above baseline |
|---|---:|---:|
| CurrentContext, misses recomputed | 2,750,621,954 | 488,023 |
| CurrentContext, misses reused | 23,374,650 | 464,816 |
| StaticBirth, misses recomputed | 2,750,622,593 | 488,023 |
| StaticBirth, misses reused | 23,375,289 | 464,816 |
| MatchDependencies, misses recomputed | 2,750,622,593 | 488,023 |
| MatchDependencies, misses reused | 23,375,289 | 464,816 |
| Scan | 9,187,451 | 319,617 |
| Indexed | 2,055,387 | 484,489 |

The large request total is cumulative traffic, not simultaneous memory or RSS. Miss reuse removes most of that traffic, but both compiled controls request fewer bytes in this witness. Indexed execution has higher peak live heap than the miss-reusing demand modes here; Scan has lower traffic and lower peak.

The successful-chain case remains consequential. At size 128 with original arrival and retained-all output, CurrentContext requests 281,436,278 bytes without miss reuse and 281,438,294 with it. Scan requests 345,590,951, while Indexed requests 12,181,031. A comparison only against Scan would conceal a much cheaper applicable allocation control. The miss cache does not solve repeated successful discovery.

## Favorable and adverse regimes both remain

Each row below compares 60 completed scenarios. The three successful-result policies have the same comparison counts, although their byte totals can differ.

| Miss-reusing candidate versus control | Lower requested bytes | Higher requested bytes | Lower peak live heap | Higher peak live heap |
|---|---:|---:|---:|---:|
| Same demand policy without miss reuse | 24 | 36 | 12 | 48 |
| Scan | 12 | 48 | 0 | 60 |
| Indexed | 24 | 36 | 52 | 8 |

All 24 allocation improvements over the same demand policy occur in known-miss or delayed-miss sources. Plain, known-hit and delayed-hit sources add allocation. Gains over Scan occur in six known-hit and six delayed-hit scenarios; neither miss family gains over Scan in this matrix. These counts describe experimental conditions, not workload frequencies.

Small overhead is also observable. The large plain CurrentContext witness rises from 1,631,966 to 1,631,998 requested bytes over four queries, with an eight-byte higher peak. That is a different regime from the billions of avoidable requested bytes in the delayed-miss witness. Their relative importance requires explicit source conditions rather than a weighted average invented for this experiment.

## What ownership qualification establishes

Independent scalar source execution constructs complete expected answers before measurement. Every completed query yields one raw answer; comparisons preserve bindings, residual resources and aliases. Canceled output is checked as a raw submultiset. All retained output is checked again after both its engine and preparation have been disposed. Window eviction releases the oldest batch in the measured consumer phase.

The measured phases are source construction, preparation, fresh query construction, query setup, execution with owned observation, engine disposal, consumer retention/release, preparation disposal and final consumer disposal. Independent fixtures and fixed-capacity reporting/consumer slots precede the root baseline. Peak values are maxima of measured phase peaks above that baseline; validation allocation is excluded. The final live heap returns to the root baseline in every metered process.

Both metered repetitions agree exactly in non-clock results, and all 960 ordinary-allocator runs have identical endpoints. An independent auditor checks all raw files, exact matrix coverage, both frozen binaries, 546 frozen source inputs, aggregate traffic and peak calculations. The meter self-check passes. No process reaches the registered 60-second wall/CPU, 1-GiB address-space or 2,000,000-turn limits.

The dependency source constructors now have one shared fixture module used by the work test and the ownership runner. Their source definitions are unchanged; the work test and all seven resource-dependency tests pass. The scalar interpreter remains independent, and no engine or reference implementation changes in this package.

Strict Clippy flags the intentional constant feature assertion in the runner. The current source gives that assertion a scoped lint allowance: feature-rich builds remain compilable, while their measurements are rejected at runtime. The frozen runner is preserved, and the auditor verifies that this single lint annotation is the entire source difference. Final Clippy passes, and a metrics-enabled execution is rejected before any measurement start event. This adjustment does not change the measured program or allocation behavior.

## Next decision

This completes package three after the relevant-deduction breadth review. T071 remains active to qualify the clock and register paired complete-lifecycle costs. Include the cheap overhead sources, substantial delayed misses, successful-chain discovery and both compiled controls. Each of these can change the interpretation of the work and allocation gains.

T076 reusable symbolic solving is the strongest distinct alternative and follows this comparison under the [sequence](../next-cycle.md). It could change evaluation itself, whereas more allocation precision would not price the demonstrated demand work. Completing the missing timing comparison is justified now because the same source, ordinary execution and ownership endpoints are qualified. At the next qualification boundary, the four-package breadth review must explicitly compare finishing that cost result with T076 and T072's relevant-key repair.

This finite qualification does not establish sustainable memory under long streams, general demand eligibility, nonground body posts, writable heads or an architectural winner. Parsing and native compilation are outside its accounting. The recorded clock values are diagnostic observations, not a registered timing result. First-observation clock qualification and independent confirmatory repetitions remain required before speed claims. The goal remains active.

## Evidence

[Registration](../registrations/S03-dependency-ownership.md), [runner](../../../research/chr-direct-conditional/examples/dependency_ownership.rs), [shared source fixture](../../../research/chr-direct-conditional/tests/runtime_support/dependency_source.rs), [matrix driver](../../../research/chr-direct-conditional/experiments/dependency_ownership.py), [freeze](s03-dependency-ownership/freeze.json), [960 configurations](s03-dependency-ownership/results.json), [raw runs](s03-dependency-ownership/runs/), [driver audit](s03-dependency-ownership/audit.json), [independent audit](s03-dependency-ownership/review-audit.json), [source tests](s03-dependency-ownership/source-tests.log) and [final Clippy](s03-dependency-ownership/clippy-final.log) and [counter rejection](s03-dependency-ownership/counter-rejection-final.log).

Run `python research/chr-direct-conditional/experiments/dependency_ownership_audit.py` to audit the stored results. The matrix driver refuses to overwrite its runs. The [preserved runner](s03-dependency-ownership/before/dependency_ownership.rs) supplies the exact frozen input; its relation to the current lint-annotated source is checked by the auditor.
