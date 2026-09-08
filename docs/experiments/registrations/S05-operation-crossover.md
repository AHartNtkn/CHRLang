# S05: operation-size and within-query request crossover

Determine whether stable dependency reuse produces a practical runtime benefit when equations become larger or occur more often. The depth64/16-request pilot established large work/traffic savings but small runtime changes, with a distinct-request loss. Increasing query batches alone would not create more cache reuse because caches are query-owned.

## Sources and contrasts

Use `experiments/crossover_source.rs` through the existing lifecycle harness. Independent knobs are constructor depth D in {64,256,1024} and binary-choice depth B in {2,4,6}, yielding R=2^B requests in {4,16,64}. Unknown and repeated arguments, consumption, explicit choice and full residual witnesses retain the previous source meaning. Query seed0 is the measured query; gates also check seed7. All successful alternatives have unique full witnesses; clash has zero answers. Ruleset size grows with request count, so preparation and source selection costs are included rather than attributing the entire change to unification.

The primary grid contains before-discrimination success and clash for every D/B combination: **18 source configurations**. Four contrary configurations add after-discrimination success and clash at D1024/B6, and distinct successful requests at D64/B6 and D1024/B6. Total **22 sources**. The largest configurations first receive noncomparative complete-answer sizing checks under the same runtime resource limits; those checks do not rank performance.

Use ordinary scalar, dependency cache capacity32, direct choice graph and inferred-specialized compiled execution. Cache capacity is held fixed. At 64 distinct substantive requests it forces eviction, whereas common-request keys fit; report this joint pressure explicitly rather than calling it a capacity-controlled estimate of miss overhead. Exact-context caching is excluded from this crossover because it had zero hits across changed choice bindings; broader exact-context opportunities remain open.

Each cell contains **one measured query per prepared ruleset**. This is a cold runtime lifecycle crossover, not a claim about long-lived deployment frequency. Preparation and execution remain separately available. 22 sources × four configurations = **88 cells**; five timing repetitions and two separate allocation repetitions = **616 processes**.

## Prospective work checks

For common success, ordinary kernel work is R(2D+4)+(2R−2) pairs. For common clash it is R(D+2)+(2R−2). The second term counts source choice equations. Dependency reuse should reduce this to one substantive request plus 2B distinct choice equations: 2D+4+2B or D+2+2B. All policies still execute 3R−2 source equations. These predictions must agree with complete-answer-validated work diagnostics run twice outside timing.

Distinct-request wrappers add two pairs per substantive operation; they prevent substantive cache hits. Capacity32 can evict choice keys in the 64-request case, so no exact hit/pair prediction is imposed for that diagnostic. Report actual work and investigate unexpected semantic or work differences before timing.

## Measurement and limits

Preserve the initial S05 harness's two warmups, preparation, setup, joint execution/observation, engine/answer disposal, separate first-answer-or-exhaustion cancellation run, and prepared disposal. Validate complete answers outside timing against explicit full expected answers and the independent source evaluator. Zero-answer cases have no first-answer latency. Fixture/oracle work, native compilation and process startup are excluded. Runtime sums charge prepared disposal once; cancellation is separately reported.

Use serial pinned counter-free ordinary-allocator timing, randomized blocks with seed20260912, and separately built requested-heap diagnostics. Initial limits are 60 seconds and 1 GiB per process, with two million service steps per source run. Cutoffs stop for diagnosis, including whether validation or execution hit the bound. Preserve completed receipts and do not silently restart or shrink a live run. Larger limits require a prospective amendment supported by sizing evidence.

Require exact allocation replay across the two diagnostics and live-allocation restoration after disposal. Requested bytes are not RSS. Report all five within-block ratios and medians/ranges against each control. A practical runtime change requires a median at least 20% from one and all pairs on the same side. Five repetitions remain bounded pilot evidence, not population confidence. No pooled workload weights.

If larger common operations produce a practical gain, locate the supported regime without claiming a universal policy; retain distinct-request, discrimination and peak-memory costs. If not, inspect phase/work sensitivity before concluding a meaningful bound. A graph advantage or a compiled advantage must remain in the decision map. Further cache tuning must be compared with the strongest ready alternative, including corrected S04 replay, partial matching/subscriptions and reusable parallel execution. This matrix does not discharge general invalidation, cache lifetime, all compilation/storage variants, S05 as a whole or the research goal.
