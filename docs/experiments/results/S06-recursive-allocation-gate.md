# Recursive-update lifecycle accounting passes

**The recursive-update runner now validates complete answers, exact allocation replay and disposal across favorable and adverse sources.** Constructor-update examples reduce requested allocation, while competing calls can add overhead. No ordinary timing comparison has run.

The [registration](../registrations/S06-recursive-allocation-gate.md) covers ten families, depths 0/1/16, one/four changing queries and optional resource consumption. Five configurations separate original and specialized execution with the feature off, the same two controls with it on, and contraction. All use Global+Indexed execution.

## What was validated

All 1,300 processes succeed: 1,200 allocation runs replay 600 cells exactly; 100 cancellation runs stop the first query at zero/one ticks and require the next to complete. Every query and prepared state restores its starting requested-live bytes. Complete answers are checked against the independent scalar evaluator outside measured intervals, including joint unknown aliases and residual occurrences.

The source families distinguish unchanged passthrough, unary/nested accumulator updates, unknown tails, later-bound tails, malformed residual tails, terminal alternatives, multiple competing calls, and contradictory terminal equations. A second call has one additional recursive step. Repeated queries alternate depth and occurrence order while retaining preparation.

A fixture cardinality test caught a zero-depth error before the allocation matrix: an unknown seed could unify with the proposed ground clash. The failure family now supplies an incompatible ground seed, and must produce zero answers at every tested depth. The counterexample and passing runner gates are retained with the [audit](s06-recursive-allocation-gate/audit.json). This correction changes the test source, not the language semantics or reference evaluator.

## Allocation observations to retain in the timing comparison

Primary requested bytes at first-call depth16, four queries and consuming effects. Primary includes preparation, setup, complete execution/observation and engine/answer/prepared disposal; source/input construction is reported separately.

| Family | Feature off, original | Feature off, specialized | Feature on, original | Feature on, specialized | Contracted |
|---|---:|---:|---:|---:|---:|
| pass | 161,351 | 139,951 | 161,863 | 140,463 | 81,070 |
| unary | 236,228 | 214,828 | 236,740 | 215,340 | 133,873 |
| nested | 315,097 | 293,697 | 315,609 | 294,209 | 166,948 |
| open | 249,818 | 229,474 | 250,330 | 229,986 | 121,509 |
| late | 282,277 | 258,805 | 282,789 | 259,317 | 155,161 |
| malformed | 217,948 | 197,604 | 218,460 | 198,116 | 116,649 |
| choice | 313,541 | 286,493 | 315,013 | 287,965 | 206,741 |
| multi | 392,728 | 348,960 | 393,240 | 349,472 | 351,553 |
| multi-choice | 726,457 | 651,809 | 729,849 | 655,201 | 657,525 |
| fail | 172,056 | 150,944 | 172,568 | 151,456 | 104,289 |

For unary updates, contraction requests 133,873 bytes versus 215,340 for same-build specialization. Peak requested growth is slightly higher: 18,902 versus 18,828 bytes. For nested updates, requested traffic is 166,948 versus 294,209 bytes, while peaks are 23,840 versus 23,663 bytes. Traffic and live demand are different endpoints.

The multi-choice case requests 657,525 bytes with contraction versus 655,201 for same-build specialization; peak growth is 67,409 versus 67,335 bytes. This supplies a contrary control where the singleton-admission condition limits useful contraction. Separate work diagnostics should quantify admission and skipped steps before attributing a timing result.

Feature-on ordinary controls also differ from feature-off controls—for example, unary specialized traffic rises from 214,828 to 215,340 bytes. The timing experiment must retain the feature-off control rather than charging all shared feature costs only to an artificial baseline. No runtime ranking follows from these requested-byte differences.

## Measurement responsibility and limits

The runner measures source construction, preparation, input construction, setup, complete execution/observation, engine and answer disposal, and final preparation disposal. Source rules and actual fixtures are regenerated inside their corresponding intervals. Independent expectations are constructed beforehand. The actual input must equal its validated fixture. First observation is separately recorded; contradictory cases have no first answer.

The [freeze](s06-recursive-allocation-gate/freeze.json) records sources, manifests, driver, compiler and both release binaries. Hashes match current files. The [full summary](s06-recursive-allocation-gate/summary.json) preserves every cell, allocation totals and peaks. The runner passes source/interruption tests with the feature on and off and strict scoped Clippy with contraction and metering. Existing recursive semantic gates remain applicable because runtime source is unchanged in this package.

These runs use an allocation-meter build with engine/kernel/observer counters disabled. Its elapsed times are not speed evidence; the meter counts requested heap bytes, not RSS. Short query restoration does not establish sustained consumers, publication or cache lifetime. Native compilation is not included.

## Next step

Register ordinary-allocator lifecycle sizing using the five build/mode controls and these contrasting families. Include separate work diagnostics to establish where contraction actually occurs. Larger input sizes require renewed independent-answer and allocation checks before their timings. Compare the cost of required constructor and observation work with avoided source bookkeeping, rather than assuming allocation savings imply time savings.

This completes the measurement gate within the bounded recursive cost package selected by the [breadth review](S06-recursive-breadth-review.md). T073, broader source lowering and the architecture goal remain active.
