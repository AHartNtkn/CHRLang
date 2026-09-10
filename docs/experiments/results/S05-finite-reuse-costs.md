# Successful result reuse and broader failure learning both earn runtime advantages

Exact result reuse is faster than all three controls on the two registered long, successful workloads. Covered failure learning is faster than result reuse on all-failing inputs and when a one-entry result table repeatedly evicts the next needed key. Short-batch timing remains unresolved. These are conditional runtime and memory tradeoffs, not a universal reuse policy.

## What the matched experiment establishes

All four modes use the same direct finite preparation and ordinary caller bridge. The [registration](../registrations/S05-finite-reuse-costs.md) fixes five scenarios, two CPUs, fifteen randomized paired blocks and one excluded warmup per scenario/mode/CPU. All 640 processes complete: 40 warmups and 600 measured runs, each independently validating full answers outside measured intervals. Frozen source and binary hashes remain unchanged from allocation qualification.

The primary total sums disjoint phases from source construction through preparation, changed-query setup, finite execution, caller transport/execution/owned observation and disposal. It excludes outer bookkeeping and validation. Process startup, external publication, native user-program compilation and sustained lifetime are not established by this endpoint.

Each cell below gives result-reuse/control median paired ratios on CPUs0/1. Less than one favors result reuse. Gain or loss requires at least a 10% median difference and thirteen of fifteen pairs in that direction on both CPUs. These are prospective descriptive criteria, not confidence intervals or significance claims.

| Scenario | Versus recomputation | Versus eager learning | Versus covered learning |
|---|---|---|---|
| Mixed success, depth64, sixteen follow-ups, capacity4, weight1 | **Gain: 0.679 / 0.658** | **Gain: 0.659 / 0.637** | **Gain: 0.817 / 0.793** |
| All success, depth64, sixteen follow-ups, capacity4, weight2 | **Gain: 0.745 / 0.775** | **Gain: 0.764 / 0.754** | **Gain: 0.762 / 0.753** |
| All failure, depth64, sixteen follow-ups, capacity4, weight1 | **Gain: 0.341 / 0.349** | Unresolved: 0.990 / 1.019 | **Loss: 1.128 / 1.159** |
| Mixed success, depth64, sixteen follow-ups, capacity1, weight1 | Unresolved: 1.046 / 1.014 | Unresolved: 1.025 / 0.986 | **Loss: 1.283 / 1.218** |
| Mixed success, depth0, one follow-up, capacity4, weight1 | Unresolved: 1.040 / 0.961 | Unresolved: 1.009 / 0.960 | Unresolved: 1.060 / 0.999 |

The [full analysis](s05-finite-reuse-costs/analysis.json) reports all thirty registered pairwise comparisons, every paired ratio and observed ranges. No comparison is inferred by transitively combining labels from different pairs.

## The differences survive preparation and caller costs

Pooled lifecycle medians, in milliseconds, describe scale; the per-CPU paired criteria above determine classification.

| Scenario | Recompute | Eager | Covered | Result reuse |
|---|---:|---:|---:|---:|
| Mixed successful reuse | 3.253 | 3.316 | 2.699 | 2.171 |
| Weighted all-success reuse | 5.312 | 5.381 | 5.404 | 4.046 |
| All failure | 1.605 | 0.550 | 0.472 | 0.544 |
| Forced eviction | 3.292 | 3.366 | 2.737 | 3.376 |
| Short batch | 0.185 | 0.183 | 0.187 | 0.191 |

In mixed successful reuse, finite-service medians are 1.504ms for recomputation, 0.889ms for covered learning and 0.307ms for result reuse. In the all-failure case, they are 1.307ms, 0.176ms and 0.235ms respectively. These phases locate the saved work; separate medians do not form an additive identity.

The [allocation comparison](S05-finite-reuse-ownership.md) explains a consequential distinction: covered learning reuses a failed larger domain for a new narrower query, whereas exact result reuse must compute that key once. Exact reuse can instead retain successful results that failure learning cannot reuse. Its lower traffic on weighted successful queries comes with a higher measured live-heap peak and retained successful-result memory. Runtime alone does not settle that ownership tradeoff.

## Uncertainty remains explicit

Covered learning versus recomputation in the mixed-success capacity-four case is unresolved in this run: median ratios are 0.847/0.814, but only eleven of fifteen CPU0 pairs favor covered learning. CPU1 has fourteen favorable pairs and one ratio of1.794. All rows remain in the analysis. The earlier [learning confirmation](S06-learning-costs.md) supplies its own bounded positive result; this run does not inherit that classification.

More repetition of that contrast or the short batch would not change the current decision to retain both mechanisms for further architecture comparison. Result reuse already beats both controls in the successful regime, while covered learning has separate qualified favorable comparisons under failure and eviction. No short-query deployment threshold or automatic policy is being selected. Those timing gaps remain unresolved and must return if such a policy is proposed.

The [runner](../../../research/chr-hvm/finite_reuse/costs.py), [manifest](s05-finite-reuse-costs/manifest.json), [raw rows](s05-finite-reuse-costs/runs.jsonl) and [validation hashes](s05-finite-reuse-costs/validation.json) preserve the evidence. `costs.py --analyze` reproduces the analysis without running new timings.

## Disposition

Carry exact result reuse and covered failure learning as different viable mechanisms. A combined implementation would need to earn its extra recognition, retention and policy machinery; this experiment does not justify one automatically. General continuation projection, broader effects, sustained memory and held-out policy tests remain required.

This completes the fourth package since the sequence revision. The [breadth review](S05-finite-reuse-breadth-review.md) selects T073's resource-phase composition gate next, comparing it explicitly with the other unresolved directions. T075 remains unfinished for its broader scope. The architecture goal remains active.
