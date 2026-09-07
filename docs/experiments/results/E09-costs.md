# E09 integrated time and storage

All 225 configurations complete within the registered limits: 25 workloads,
nine policy/grouping variants, quantum 8. The confirmatory batch contains 225
warm-ups followed by 1,125 fresh-process untraced timing runs; the separate memory
batch contains 450 traced runs. Every run preserves expected/reference observations
and raw multiplicity, or the declared finite prefix. Actions and publication action
indices agree across all eight executions of each configuration. Fifteen local
protocol/service tests pass.

The timing protocol places all warm-ups before measured repetitions. Its ordering
is tested. The first 1,350-row batch is retained as protocol diagnostics because it
interleaved warm-up labels; it supports no timing conclusion here.

## Measured consequences

Milliseconds below are internal totals including JSON-to-AST/query preparation,
search and complete output serialization, with median [minimum, maximum] over five
untraced runs. Traced peak is the maximum across preparation, search and serialization;
the two memory repetitions give identical peaks in these rows.

| Workload | Configuration | Total ms | Traced peak bytes |
|---|---|---:|---:|
| 6 duplicate choices | async, ungrouped | 7.493 [6.907, 7.895] | 148,504 |
| 6 duplicate choices | async, identity grouping | 0.898 [0.759, 0.932] | 28,723 |
| 6 duplicate choices | round, ungrouped | 5.933 [5.412, 6.199] | 130,760 |
| 6 duplicate choices | round, identity grouping | 0.866 [0.809, 0.918] | 28,723 |
| Opaque depth 64 | async, ungrouped | 5.139 [4.932, 5.468] | 216,189 |
| Opaque depth 64 | async, reverse grouping | 7.228 [6.797, 7.362] | 216,189 |
| Opaque depth 64 | async, forward grouping | 4.938 [4.765, 5.412] | 216,189 |
| SK duplication | async, ungrouped | 30.960 [28.414, 33.469] | 111,830 |
| SK duplication | async, reverse grouping | 85.398 [80.564, 90.131] | 123,410 |
| SK duplication | async, identity grouping | 33.136 [30.576, 35.017] | 112,065 |

Duplicate-choice grouping produces a separated timing improvement and lower traced
peak while retaining 64 raw alternatives and one unique answer. Its usefulness is
supported in this high-reuse probe. Ungrouped round/async queue organization also
changes peak storage, so queue counts alone would not quantify memory adequately.

The reverse opaque comparator has a separated timing penalty. Forward grouping
and ungrouped timing ranges overlap: the measured comparison does not establish a
speed advantage for either. Preparation determines this workload's peak (216,189
bytes), while async search peaks around 176,648 bytes ungrouped. Optimizing just
scheduler storage would miss that larger setup allocation.

SK duplication exposes a substantial adverse case for structural grouping. Identity
checks mitigate it, but async identity/ungrouped ranges overlap. For round scheduling,
identity total 35.966 [34.317, 38.148] ms is slower than ungrouped 31.215
[27.848, 32.668] ms with separated ranges. No grouped variant changes source-job
counts on any of the 13 exhausted application workloads. Grouping costs here are
recognition/bookkeeping without avoided source execution. This does not refute
operation-level or conditional sharing across different states.

The depth-64 large-equation probe confirms the latency effect separately from total
time. Ungrouped async first answer is 0.883 [0.828, 0.899] ms, while round is
1.227 [1.083, 1.247] ms. Those ranges separate. Total-time ranges overlap. A round
barrier can therefore worsen interactive response without materially changing
completed source work or demonstrating a total-throughput difference.

## What the measurements include

Preparation parses equivalent JSON anew, builds rule objects and initializes the
query; this adapter has no source-specific compilation phase. Imported engine code
and serialized request storage predate internal timing/tracing. Outer process time
is recorded separately and includes runtime loading, protocol and validation. The
largest confirmatory untraced process duration is 0.356 seconds rounded upward;
no deadline, address-space or action-cap failure occurs.

Search includes source/observer services, counters, grouping, scheduler dispatch
and commits. It is uniformly instrumented Python, not an optimized native-engine
benchmark. Tracemalloc is disabled in the timing batch. All memory phase current
and peak values, process RSS, output serialization size and post-release traced
current are retained in the raw records and summary.

Post-release current includes the retained answers and serialized output **plus
measurement metadata**; it is not an isolated answer-graph byte count. For example,
SK duplication async ungrouped falls from 83,844 traced current bytes after search
to 6,570 after releasing evaluator/program state and collecting. This describes
the measured lifecycle, not an attribution of all 6,570 bytes to answers. RSS
includes the runtime and is not interchangeable with traced allocations. Prefix
runs retain live evaluator work before release; they are not exhaustion results.

## Disposition and next work

These results support testing asynchronous service as a latency-preserving baseline
and retaining grouping as a workload-dependent optimization. They do not justify
always grouping, choosing a production architecture, or imposing source restrictions.
Longer intended synthesis, mixed tuple scans/wake-ups, batch-size sensitivity,
key construction/retention, heterogeneous support subdivision and net/second-service
composition remain feasible and relevant. The application no-hit result makes
operation-level sharing especially important; more duplicate-only timing repetitions
are unlikely to change that decision.

Evidence: [all summary statistics](E09-cost-summary.jsonl),
[confirmatory timings](E09-time-v2.jsonl), [memory](E09-memory.jsonl),
[protocol diagnostics](E09-timing-protocol-diagnostic.jsonl),
[source hashes and host](E09-cost-manifest.json).

```sh
cargo run -q -p chr-symbolic-fixtures --example export_cases > /tmp/chr-e09-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/run_measurements.py /tmp/chr-e09-cases.jsonl time > /tmp/e09-time.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/run_measurements.py /tmp/chr-e09-cases.jsonl memory > /tmp/e09-memory.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/summarize_measurements.py /tmp/e09-time.jsonl /tmp/e09-memory.jsonl
```
