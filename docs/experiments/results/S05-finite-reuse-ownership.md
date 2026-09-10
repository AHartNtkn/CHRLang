# Result reuse and failure learning have different allocation advantages

Result reuse reduces requested allocation on long, repeated successful computations when the table holds the recurring keys. Covered failure learning does better on the all-failing source because it can reuse a broader failure for a new, narrower query. All 900 qualification processes preserve complete answers; all 600 diagnostic runs restore task-owned heap to baseline. Runtime superiority is not established.

## The comparison now includes all retained owners

The same runner compares recomputation, eager failure learning, covered-state failure learning and exact private-result reuse using the same direct finite preparation and ordinary caller bridge. It charges source construction, preparation, query setup, finite service, transport, owned answers and disposal. Consumers retain all answers through prepared-state disposal. Source generation and compiler/artifact lifetimes are outside this experiment.

The [registration](../registrations/S05-finite-reuse-ownership.md) fixes 288 ordinary configurations and twelve cancellation configurations. Each runs once with the ordinary allocator and twice with diagnostic allocation measurement: 900 processes total. The [pre-run manifest](s05-finite-reuse-ownership/manifest.json) freezes sources, binaries and configurations. All 300 diagnostic pairs match exactly, ordinary/diagnostic outcomes agree, phase ownership is continuous, validation restores its heap, and final measured heap returns to baseline. Meter checks and both runner Clippy configurations pass.

The query sequence starts with `{a,b}` and then alternates `{a,b,c}` and `{a}`, changing variable identities every time. Capacity four retains the three exact result keys; capacity one forces their repeated eviction. Capacity zero and one-follow-up batches expose overhead with little or no useful retention. Weighted successful results expose output traffic. These are controlled regimes, not workload weights or a sustained-stream study.

## Representative complete-lifecycle allocations

Values below are total requested heap bytes. They include preparation, the caller and consumer output allocations; they are not RSS or elapsed time. “Long reuse” means a depth-64 prefix and sixteen follow-ups after the seed query.

| Scenario | Recompute | Eager learning | Covered learning | Result reuse |
|---|---:|---:|---:|---:|
| Mixed success, long reuse, capacity4, weight1 | 4,237,446 | 4,233,175 | 3,497,775 | **2,885,380** |
| All success, long reuse, capacity4, weight2 | 6,887,782 | 6,903,983 | 6,903,983 | **5,493,919** |
| All failure, long reuse, capacity4, weight1 | 1,930,521 | 706,423 | **612,889** | 710,301 |
| Mixed success, long reuse, capacity1, weight1 | 4,237,446 | 4,262,869 | **3,507,456** | 4,326,540 |
| Mixed success, depth0, one follow-up, capacity4, weight1 | **158,454** | 159,579 | 159,221 | 169,546 |

Across the 72 non-cancellation scenarios, result reuse requests fewer bytes than recomputation in ten and more in 62; fewer than eager learning in 24 and more in 48; fewer than covered learning in eight and more in 64. These counts describe this matrix only. They do not select a policy by voting across arbitrarily enumerated scenarios.

**Successful reuse can lower traffic while raising peak memory.** In the all-success weighted case, peak task-owned live heap is 321,533 bytes for result reuse versus 315,430 for recomputation. The result owner releases 6,391 bytes at disposal. In the mixed-success capacity-four case it releases 3,877 bytes, versus 1,808 for the retained failure regions. Preparation dominates the recorded 191,554-byte peak in all four modes there. An unchanged whole-run peak does not mean the retained results are free.

## The all-failure difference is attributable

The [per-query phase attribution](s05-finite-reuse-ownership/failure-attribution.json) identifies the first narrow query, at index2, as the main difference. Covered learning requests zero bytes during finite service; result reuse requests 89,159. Subsequent repeated keys request zero finite-service bytes in both. Their query setup still allocates: at index2, 3,427 bytes for covered learning and 4,002 for result reuse.

The source and key rules explain the distinction. The retained failed `{a,b}` region covers `{a}`, so learning can reject the new query before executing its prefix. The result table recognizes exact normalized private syntax; the `{a}` producer is a different key and must compute its first result. This is a difference in reuse generality, not an accidentally duplicated result-table computation. It does not establish that a combination of region learning and result caching would repay the additional machinery.

In the mixed-success capacity-four case, finite-service traffic is 357,719 bytes for result reuse versus 977,919 for covered learning. Caller transport/execution/observation requests the same 2,072,080 bytes in all modes. Thus the allocation reduction occurs inside the finite phase and survives charging the complete caller. With capacity one, eviction prevents those exact-key savings; that adverse case remains part of the comparison.

## Evidence and present limits

The [launcher](../../../research/chr-hvm/finite_reuse/ownership.py), [analysis](../../../research/chr-hvm/finite_reuse/analyze_ownership.py), [raw runs](s05-finite-reuse-ownership/runs.jsonl) and [audit](s05-finite-reuse-ownership/audit.json) reproduce the accounting. The runner's `retained_regions` field counts learned failure regions; its separate `retained_results` field counts completed-result entries. Both owners are charged within the existing learner setup/drop intervals. The initial run in [red.log](s05-finite-reuse-ownership/red.log) demonstrates that the runner did not yet admit the result-reuse mode; no solver defect is inferred from that assertion.

Requested allocation measures cumulative heap requests. Peak live heap includes retained outputs under this consumer. The experiment does not measure RSS, long-term allocator residency, arbitrary distinct query shapes or output release windows. Cancellation occurs after the first progress step on a new key; it qualifies disposal and subsequent reuse, not interruption latency inside a hit's transport operation. No recorded clock values are used to rank architectures.

## Next decision

The matched control changes the interpretation of earlier learning evidence: successful exact reuse is now a credible competing mechanism, while failure-region generalization retains a distinct favorable case. Both survive the allocation comparison, and neither earns a universal default.

Register a bounded primary cost comparison on these opposing regimes, including short/no-hit and forced-eviction cases, using the qualified ordinary-allocator build and independent validation. Runtime matters because key comparisons, containment checks and transport need not track allocated bytes. Broader effectful capacity solving is the strongest ready distinct alternative; it requires a new resource correspondence. A small paired timing comparison can decide whether the allocation tradeoffs also change complete runtime choices, so it is the next package.

This is package three since the sequence revision. At the next completed package or an obstruction, perform the four-package breadth review, explicitly comparing still-untried mechanisms with any further reuse refinement. Broader capacity phases, compilation and sustained lifetime remain required. The architecture goal remains active.
