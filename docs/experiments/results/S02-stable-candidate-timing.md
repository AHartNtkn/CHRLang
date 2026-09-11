# Candidate caching helps broad execution; readiness remains expensive

**Inferred candidate caching cuts complete lifecycle time by 45–48% in the two width-128 full-settlement cases. The compiled control still wins both.** Bounded readiness gains 15–16% from caching in those cases, but remains much slower than compiled execution. Keep the inference as a qualified experimental mechanism; test readiness traversal next. No architecture is selected.

The ordinary-allocator campaign completed **1,200 measured processes, 12,000 complete sessions and 120 paired contrasts**. Every session prepares rules, runs two changing queries, retains and validates complete answers, and disposes all measured owners. The audit verifies frozen sources/binaries, configuration order, phase coverage and all 20 blocks. Sixty warmup processes are excluded. See the [registration](../registrations/S02-stable-candidate-timing.md), [raw results and freeze](s02-stable-candidate-timing/) and [all contrasts](s02-stable-candidate-timing/analysis.json).

## What the comparisons establish

Ratios below are candidate time / reference time, using the registered median of 20 paired process-mean ratios. Smaller is faster. **Bold** means the registered gain gate passed: median at most 0.90 and all 20 blocks below 1. Plain numbers remain unresolved, including apparently large median differences. No registered cache comparison qualifies as a loss.

“Inferred” preserves candidates only for guard-free heads with globally distinct variable arguments, inferred from prepared rules. “Conservative” invalidates them on equality changes. Both use the same qualified borrowed cycle traversal and vector incidence representation. Full settlement and bounded readiness are separate schedules; the compiled engine is the existing independent control.

| Source case | Inferred / conservative, full | Inferred / conservative, ready | Compiled / inferred, full | Compiled / inferred, ready |
|---|---:|---:|---:|---:|
| Depth 4, separate, success | 0.969 | 0.985 | **0.504** | **0.484** |
| Broad 64, insert forward, cancel | 0.987 | 1.016 | **0.386** | **0.386** |
| Broad 64, insert forward, forward | 0.698 | 0.863 | 0.681 | **0.208** |
| Broad 64, insert forward, reverse | 0.701 | 0.853 | **0.613** | **0.217** |
| Depth 64, separate, fail | 0.996 | 0.977 | **0.125** | **0.451** |
| Broad 64, insert reverse, cancel | 0.985 | 0.995 | **0.350** | **0.348** |
| Broad 64, insert reverse, forward | 0.701 | 0.824 | **0.597** | **0.213** |
| Broad 64, insert reverse, reverse | 0.696 | 0.843 | 0.648 | **0.200** |
| Depth 64, shared, clash | 0.997 | 1.106 | **0.099** | **0.036** |
| Depth 64, shared, success | 0.987 | 1.007 | **0.125** | **0.047** |
| Broad 128, insert forward, forward | **0.550** | **0.839** | **0.641** | **0.119** |
| Broad 128, insert reverse, reverse | **0.524** | **0.850** | **0.659** | **0.114** |

Caching qualifies in four of 24 within-schedule comparisons, all width 128. The compiled control qualifies against inferred full settlement in ten of twelve cases and against inferred readiness in all twelve. These are source-specific findings, not workload weights or a universal ranking.

## Why the width-64 verdicts remain unresolved

Inspecting the retained sessions shows that the contrary blocks cannot all be attributed to one extreme sample. In forward-insert/forward-bind block 6, inferred/full averages 1.241 times conservative/full, and its median session is also slower (1.027). In reverse-insert/reverse-bind block 3, the corresponding ratios are 1.278 and 1.276. For compiled versus inferred/full, the contrary blocks include both a long first session and multiple slower subsequent sessions: forward/forward block 17 has mean ratio 1.468 and median-session ratio 1.200; reverse/reverse block 9 has 1.391 and 1.175. Blocks are zero-based as in the freeze.

This inspection does not change the endpoint or discard samples. It establishes that selecting a different summary after seeing the results would not justify a stable ordering. No adoption decision depends on those uncertain width-64 contrasts: both width-128 comparisons establish a cache benefit and a remaining compiled advantage under the original gate. A policy that depends on the width-64 crossover still requires a separately registered timing investigation.

## What remains costly after the improvement

For width 128 with reversed insertion and binding, the median process-mean phase times are:

| Complete two-query session phase | Conservative full | Inferred full | Inferred ready | Compiled |
|---|---:|---:|---:|---:|
| Rule preparation | 4.9 µs | 5.0 µs | 8.2 µs | 7.9 µs |
| Query setup | 719.0 µs | 707.7 µs | 741.2 µs | 259.8 µs |
| Execution and observation | 5,606.7 µs | 2,574.8 µs | 18,665.9 µs | 1,896.1 µs |
| Disposal | 69.8 µs | 75.6 µs | 77.7 µs | 49.5 µs |

Phase medians explain where time lies; their sum is not the registered paired endpoint. Backend disposal boundaries also differ, so complete totals control the comparison. Compilation and source construction are not isolated by this runner; architectural lifecycle comparison still requires those experiments.

The separate [allocation experiment](S02-stable-candidates.md) explains the cache mechanism: broad discovery falls from 261 to 4 calls and full-session requested bytes from 9.88 MB to 2.27 MB. Eight of 24 relational configurations request more memory and twenty peak higher. Those ownership costs remain part of the decision even where time improves. Requested allocation is not RSS.

## Next experiment and its architectural purpose

**Qualify a direct first-equation readiness shortcut under T072.** The current readiness path rebuilds equation/constructor reachability before selecting a relevant equation. When the first queued equation directly touches a matcher-read value, relevance already follows and there is no earlier equation whose priority could be bypassed. Test that narrow inference against the complete current path. If there is no direct hit, the existing closure is still needed.

Before comparative measurements, add an adversary where an earlier equation is indirectly relevant and a later equation directly touches a read value: a shortcut must not reorder that pair. Also test aliases, redundant equations, constructor reachability, failure priority, consuming claims, branch isolation and complete raw answers. Extend the source fixture whenever it fails to exercise an obligation. Then separately measure closure work/requested ownership and ordinary complete lifecycle time with cache-on/off and compiled controls, including broad merges and beneficial early failure. Register exact configurations and gates before running.

This is more discriminating now than another general discovery optimization: caching already identifies a large part of discovery cost, whereas the readiness path still consumes 18.67 ms in this session and previously attributed 7.72 MB to dependency traversal. A selection-preserving shortcut has a concrete, small correctness gate. Graph memo/context attribution (T074), coarser reuse and direct solving remain required independent investigations; this serial result does not answer them. The next package must include the full 57-question portfolio review and reassess priorities against those alternatives. Package count is three; T072 and the research goal remain active.

Reproduce the audit with `python3 research/chr-relational/experiments/stable_candidate_timing.py --audit`. No engine code changed in this timing package; the [candidate qualification](S02-stable-candidates.md) applies to the frozen source.
