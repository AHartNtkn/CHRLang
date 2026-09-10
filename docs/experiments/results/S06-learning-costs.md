# Learning pays in measured reuse regimes; the checking policy matters

Covered-state learning reduces complete lifecycle cost in the tested deep, reused all-failure and mixed-success batches. Eager subtraction gains on repeated whole-query failure but does not establish a mixed-success gain. These results compare the qualified finite-learning policies with recomputation; they do not select a general cache or a whole architecture.

## The distinction tested

The source chooses two finite values, executes a deterministic consuming prefix and then accepts or rejects their pair. A separate caller consumes an output request and token, produces the result and preserves unrelated residual noise. Mask 0 rejects all pairs, mask 484 accepts pairs containing c, and mask 511 accepts all pairs. An a/b seed precedes changing a/b/c and a-only queries. Variable identities change between queries; prepared rules and caller state are reused.

Eager learning subtracts known failed regions before execution, potentially repeating common work across complements. Covered-state learning checks whether a refined state is wholly covered before continuing it. Both retain only completed whole-query failures under the current contract. Neither implements general conflict learning from arbitrary failed branches.

The [pilot registration](../registrations/S06-learning-cost-pilot.md) crosses three policies, three masks, depths 0/16/64 and 1/4/16 follow-ups, using weight one and capacity four. There are 81 configurations and 54 learning-versus-recomputation contrasts. Zero capacity, weighted answers, aliases and cancellation have separate [entry evidence](S06-learning-prefix-entry.md).

## Confirmation resolves the consequential mixed-success uncertainty

The pilot's covered-state mixed-success depth-64, 16-follow-up medians improve by about 18%, but extrema overlap. Its phase records locate the plausible saving in finite service rather than the caller. A separate [prospective confirmation](../registrations/S06-learning-cost-confirmation.md) therefore targets that case, deep all-failure and no-failure controls, and short-use mixed success. Pilot timings are not pooled with confirmation.

Confirmation uses 15 paired blocks on each CPU. A practical gain requires median paired ratio at most 0.90 and at least 13 of 15 ratios below one on both CPUs; a loss uses the reverse direction and a 1.10 threshold. This descriptive screen was fixed before confirmation. It tolerates isolated excursions and differs from the pilot's strict extrema rule; it is not a significance test.

| Mask / depth / follow-ups | Policy | CPU 0 median ratio | CPU 1 median ratio | Pairs faster, CPU 0 / 1 | Result |
|---|---|---:|---:|---:|---|
| 0 / 64 / 16 | eager | 0.327 | 0.345 | 15 / 14 | gain |
| 0 / 64 / 16 | covered | 0.294 | 0.288 | 15 / 15 | gain |
| 484 / 64 / 16 | eager | 0.994 | 1.025 | 8 / 7 | unresolved |
| 484 / 64 / 16 | covered | 0.806 | 0.806 | 14 / 14 | gain |
| 511 / 64 / 16 | eager | 1.006 | 1.007 | 7 / 5 | unresolved |
| 511 / 64 / 16 | covered | 0.998 | 1.000 | 9 / 8 | unresolved |
| 484 / 64 / 1 | eager | 1.095 | 1.116 | 2 / 1 | unresolved |
| 484 / 64 / 1 | covered | 0.990 | 0.969 | 8 / 9 | unresolved |

Covered-state mixed-success gains remain sensitive to an extrema-based criterion: candidate and recomputation ranges overlap on both CPUs. The confirmation establishes a repeatable median benefit under its registered paired criterion, not faster execution on every run. Covered-state deep all-failure ranges are fully separated on both CPUs; eager all-failure ranges overlap on one CPU despite its paired gain. The short-use eager case is unresolved because one CPU's median ratio, 1.095, does not cross the practical-loss threshold. Its 13/15 and 14/15 slower pairs still constitute adverse evidence; unresolved does not mean equal cost.

## The phase evidence explains the useful distinction

In the confirmed depth-64 mixed-success batch with 16 follow-ups, pooled median lifecycle totals are 3.115 ms for recomputation, 3.131 ms for eager learning and 2.522 ms for covered-state learning. Finite-service medians are 1.454, 1.430 and 0.847 ms respectively. This identifies the service interval as the consequential saving; caller execution and complete outputs remain charged. Medians of separate phases need not sum.

With only one follow-up on that source, finite service is 0.181 ms for recomputation, 0.258 ms for eager learning and 0.181 ms for covered-state learning. The current eager strategy has a concrete adverse common-work regime. On deep all-failure with 16 follow-ups, lifecycle medians fall from 1.528 ms to 0.523 ms eager and 0.444 ms covered-state. Avoiding repeated whole-query work can repay setup and retention.

The no-failure confirmation resolves no practical gain or loss. The pilot likewise contains substantial uncertainty in small and near-parity cases. These observations do not establish a generally free learning policy or a precise break-even depth/reuse threshold.

## All pilot contrasts remain visible

Each entry below is pooled candidate/recomputation median ratio and the original pilot classification. That classification requires a 10% median difference and nonoverlapping five-sample ranges on each CPU. Later confirmation applies only to its four named scenarios.

| Mask | Depth | Follow-ups | Eager ratio / result | Covered ratio / result |
|---|---:|---:|---|---|
| 0 | 0 | 1 | 1.041 / unresolved | 1.001 / unresolved |
| 0 | 0 | 4 | 1.153 / unresolved | 1.118 / unresolved |
| 0 | 0 | 16 | 1.078 / unresolved | 1.084 / unresolved |
| 0 | 16 | 1 | 1.077 / unresolved | 0.990 / unresolved |
| 0 | 16 | 4 | 0.985 / unresolved | 0.913 / unresolved |
| 0 | 16 | 16 | 0.702 / gain | 0.641 / gain |
| 0 | 64 | 1 | 1.141 / unresolved | 1.015 / unresolved |
| 0 | 64 | 4 | 0.758 / unresolved | 0.635 / gain |
| 0 | 64 | 16 | 0.341 / gain | 0.297 / gain |
| 484 | 0 | 1 | 1.032 / unresolved | 1.038 / unresolved |
| 484 | 0 | 4 | 1.043 / unresolved | 1.092 / unresolved |
| 484 | 0 | 16 | 1.017 / unresolved | 1.069 / unresolved |
| 484 | 16 | 1 | 1.017 / unresolved | 1.005 / unresolved |
| 484 | 16 | 4 | 1.029 / unresolved | 0.987 / unresolved |
| 484 | 16 | 16 | 1.005 / unresolved | 0.925 / unresolved |
| 484 | 64 | 1 | 1.146 / unresolved | 1.037 / unresolved |
| 484 | 64 | 4 | 1.014 / unresolved | 0.850 / unresolved |
| 484 | 64 | 16 | 1.005 / unresolved | 0.817 / unresolved |
| 511 | 0 | 1 | 1.041 / unresolved | 1.000 / unresolved |
| 511 | 0 | 4 | 1.052 / unresolved | 0.991 / unresolved |
| 511 | 0 | 16 | 1.046 / unresolved | 1.007 / unresolved |
| 511 | 16 | 1 | 1.039 / unresolved | 1.079 / unresolved |
| 511 | 16 | 4 | 1.002 / unresolved | 1.014 / unresolved |
| 511 | 16 | 16 | 1.002 / unresolved | 1.054 / unresolved |
| 511 | 64 | 1 | 1.026 / unresolved | 0.982 / unresolved |
| 511 | 64 | 4 | 0.987 / unresolved | 0.987 / unresolved |
| 511 | 64 | 16 | 1.044 / unresolved | 1.029 / unresolved |

## Validation and limits

The pilot first runs 243 excluded qualification processes: every configuration once primary and twice diagnostic. Complete independently checked answers, analytical multiplicity and retained-region counts agree; diagnostic phases repeat exactly and restore task-owned heap. It then runs 162 excluded warmups and 810 measured primary processes. Confirmation adds 24 warmups and 360 measured processes. All 1,599 processes complete without reported cutoff or disagreement.

Primary builds use ordinary allocation and disabled diagnostic engine/learning counters. The endpoint sums source construction, preparation, learner/query setup, finite service, caller transport/execution/observation and all recorded disposal phases. Internal complete answers survive preparation disposal. Full scalar and compiled-control validation occurs outside those intervals in every process. Preallocated outer output/phase storage is excluded; process wall includes validation and is not an engine comparison. Clock calls and validation can affect allocator/cache context. External publication, compilation and sustained consumer lifetimes remain separate obligations.

Renamed query variables defeat the existing exact-debug-query cache, but a cache recognizing queries up to renaming is a credible stronger competitor. These batches repeat a small set of normalized shapes. That cache's recognition, answer remapping, retention and invalidation costs have not been measured here. The result establishes benefits over recomputation, not that failed-region learning is the best reuse organization. Call-level reuse remains a required S05 comparison, including genuinely different domains where a cached exact answer does not apply.

## Next investigation and architectural consequence

Retain covered-state learning as a measured candidate for deep compatible-query reuse; retain eager subtraction's favorable whole-failure and adverse common-work regimes. Do not impose learning globally or infer workload weights. Required machinery includes source admission, failure-region validity, recognition, retention/eviction, cancellation safety and answer transport; the measured savings must be weighed against those responsibilities.

Select a direct consuming-resource derivation entry next under T073. It could eliminate execution responsibilities retained by both these learning policies and current compiled plans. Start by inventorying existing resource-fusion, count and finite-solving evidence, then name an additional source-derived mechanism and its independent oracle before implementation. A previously measured transformation must remain a control.

The strongest ready alternative is a bounded comparison with renaming-aware call reuse, followed by broader compatible domains and retention. It could overturn a preference for failed-region learning, but needs a valid answer-remapping and lifetime implementation rather than the existing exact-key cache. Direct derivations have broader potential to change runtime responsibilities; select their bounded entry first, with the call-reuse comparison reviewed at that semantic entry or an obstruction. Tightening the short-use 10% threshold would not choose between those architectures, so it does not take priority now. Source/language tradeoffs, sustained lifetime and complete-architecture challenges remain required. The research goal is active.

## Reproducible evidence

[Pilot manifest and hashes](s06-learning-cost-pilot/manifest.json), [qualification](s06-learning-cost-pilot/qualification.jsonl), [primary observations](s06-learning-cost-pilot/runs.jsonl), [analysis](s06-learning-cost-pilot/analysis.json), [validation receipt](s06-learning-cost-pilot/validation.json); [confirmation manifest](s06-learning-cost-confirmation/manifest.json), [observations](s06-learning-cost-confirmation/runs.jsonl), [analysis](s06-learning-cost-confirmation/analysis.json), [validation hashes](s06-learning-cost-confirmation/validation.json).

The [pilot runner](../../../research/chr-hvm/learned_regions/cost_pilot.py) and [confirmation runner](../../../research/chr-hvm/learned_regions/cost_confirmation.py) refuse to overwrite observations. Reanalyze the pilot without rerunning timings using `python research/chr-hvm/learned_regions/cost_analysis.py`. Exact commands, frozen configurations, randomized order and raw phases are preserved. Both registrations specify CPU affinity and resource bounds before execution.
