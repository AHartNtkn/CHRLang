# S05 full-caller cost pilot

This prospective pilot estimates full-lifecycle tradeoffs for checked private-phase reuse. It is exploratory, not a confirmatory architecture ranking. Its outcome determines which cost or mechanism is worth investigating next.

## Questions and hypotheses

Memoized calls should save private execution when keys repeat, but must repay admission, key construction, retained results and fresh replay. Distinct calls and one-query lifetimes expose that overhead. Recursive result construction makes output traffic consequential; pure carrier recursion exposes work a stronger source transformation can eliminate. Substantial caller work may dilute any private savings.

Compare seven complete paths: ordinary persistent execution (`direct`); compact whole-state reuse (`whole`); compiled scanning (`scan`); inferred compiled specialization with Scan (`sealed`); uncached checked call phases (`call-direct`); memoized phases (`call-memo`); checked exact-source elimination (`lowered`). The last is a hand-derived source-specific control, not a general compiler. It validates natural-number control inputs and ground payloads, eliminates carrier/tail loops, constructs required recursive results and emits complete raw answers.

## Fixed sources and matrix

`examples/support/call_cost.rs` defines the source. Work either carries a result through recursion or constructs `wrap` at each step; its base has two explicit successful alternatives, each returning `pair(payload, Fresh, Fresh)`. The caller performs its own tail recursion and returns `seen(Result, Result)`, retaining a distinct `marker(caller-i)` per query. Complete joint aliases and both raw alternatives matter.

Cross build/carrier (2), private depth 0/32, caller depth 0/8, payload depth 0/8, one/eight queries per prepared owner, and repeated/distinct private keys. There are 64 configurations and seven modes. Distinct keys change the payload seed on every query; caller markers change in both families, so complete caller states differ even when private keys repeat.

Run five fresh ordinary-allocator processes per cell (2,240 processes) and two allocation-meter processes (896). Use seed 751903 to shuffle each ordinary block; use successive seeds for the five blocks. Shuffle allocation blocks with seeds 751913 and 751914. Each process first performs an unmeasured full semantic validation/preparation/query pass, disposing it before measured preparation. This warms code and allocator state; process startup is not an endpoint.

Add cancellation after the first observed answer for build/carrier, repeated/distinct keys, depth 32, caller depth 8, payload depth 8 and eight queries. Run all seven modes once ordinarily and twice with allocation diagnostics: 84 processes. Total planned processes: 3,220. No cancellation timing classification is intended.

## Endpoints and accounting

Record preparation, each query's input construction and setup, execution plus full owned observation, caller disposal, consumer release and final prepared disposal. Primary total excludes the separately reported query-input construction; also report the source-input-inclusive total. First observation includes query setup and execution through the first answer; report preparation separately and avoid treating first-answer latency as complete-query throughput.

Counter-free release builds with the ordinary allocator provide primary timings. The allocation build measures requested allocation traffic, live bytes and phase peaks; its elapsed times are never speed evidence. Work-count diagnostics, if added, use another build. The allocation runner must restore its owner baseline after final disposal. Both input construction and its later deallocation are disclosed; source-input-inclusive totals cover that lifecycle.

All complete answers are independently validated outside measured intervals, before the measured pass; deterministic answer counts and exhaustion/cancellation endpoints are checked during it. Every measured output is owned and subsequently disposed. This pilot does not isolate native compilation, parser input, long-lived eviction, asynchronous cancellation within private expansion, or first-answer interleaving between private alternatives.

## Bounds and interpretation

Freeze source changes and both binaries before execution, recording hashes, toolchain and host details. Bound each process at 60 seconds and 1 GiB address space; engine/source validation bounds are 2,000,000 service steps. Cutoffs and errors block the affected comparison and require diagnosis. Never classify unfinished work as a slow completed answer.

Require exact replay of allocation phase readings and complete query outcomes across the two diagnostic blocks. Report ordinary medians and ranges by configuration, retaining all five samples. Do not assign practical gain/loss labels from this pilot or aggregate configurations with invented weights. Compare memoization first with its matched uncached phase control, then with every stronger complete path.

If a consequential loss has an avoidable cost, attribute it before broad confirmation. If an advantage survives complete controls, investigate adverse placement, output growth and longer lifetime. If exact-source elimination dominates a contractible family, retain that bounded finding and do not infer that arbitrary call reuse is useless. This fourth package triggers a breadth review against S02 dependency repair, structural solving and restoration/reconnection before another refinement or confirmation.
