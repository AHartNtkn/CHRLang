# E16 cold equation-worker costs

For these seventeen queries, the shared-arena scalar engine has lower observed
cold-time and first-answer ranges than either worker mode. Its metered heap peak is
also lower in every case. The current owned-equation worker interface is therefore
not supported as a cold-query optimization for this workload set.

There is real contrary evidence to a blanket rejection of parallel work: two workers
have lower cold-time ranges than one worker on mixed failures and distinct-hole wide
depths 6 and 8. Increasing admission from one to four requests improves both large
wide queries with two workers. Those gains do not overcome the shared-arena control.
Parallelism, representation transfer, warm-pool reuse and coarse regions remain open.

## Evidence and scope

The [registration](../registrations/E16-costs.md), [raw children](E16-costs.jsonl),
[manifest](E16-costs-manifest.json) and [independent audit](E16-costs-audit.json)
cover all 1,092 children: 182 warm-ups, 637 measured timing runs and 273 separate
allocation runs. All 91 configuration keys, repetition blocks and the seeded shuffled
order match the independently constructed matrix. All semantic/work and complete
shutdown checks pass. Source and binary hashes remain fixed throughout the batch.
The longest child lasts 0.038 seconds against the 30-second bound; none fails or times out.

Each timing cell has seven observations; each allocation cell has three. A separated
comparison means the full observed ranges do not overlap. These are local observed
ranges, not confidence intervals or cross-machine guarantees. Warm-ups are retained
but excluded, and metered timings never enter timing summaries. RSS summaries are
separate for System and metered processes. CPU affinity is 0–15 on the recorded host;
this does not prove homogeneous cores or unrestricted CPU quota.

Cold time includes input cloning, construction, search, accepted-work shutdown and
search release, with answers retained. Output release is measured separately after
external validation. The table gives medians in milliseconds; the audit retains every
range and pairwise disposition. All new drivers below use K=4 and lookahead eight.

| Case | Shared | Owned | Inline | One worker | Two workers |
|---|---:|---:|---:|---:|---:|
| wide-cheap | 0.057 | 0.060 | 0.073 | 0.317 | 0.287 |
| wide-medium | 0.371 | 0.881 | 0.880 | 1.422 | 1.090 |
| wide-large | 3.526 | 8.484 | 7.875 | 13.383 | 9.143 |
| wide-identity | 4.614 | 5.542 | 5.267 | 7.935 | 7.994 |
| chain-large | 2.417 | 7.769 | 7.511 | 15.337 | 15.965 |
| skew-first | 0.685 | 1.753 | 2.235 | 2.701 | 2.899 |
| skew-last | 0.685 | 2.184 | 2.259 | 2.620 | 2.918 |
| mixed | 0.286 | 0.685 | 0.725 | 1.111 | 0.861 |
| prefix-drain | 0.041 | 0.061 | 0.074 | 0.344 | 0.301 |
| app-sk-duplication | 0.565 | 0.978 | 1.067 | 2.459 | 2.670 |
| app-type-synthesis-prefix | 0.106 | 0.117 | 0.122 | 0.322 | 0.282 |
| distinct-wide-d-4 | 0.173 | 0.367 | 0.382 | 0.630 | 0.467 |
| distinct-chain-d-4 | 0.246 | 0.393 | 0.410 | 0.719 | 0.784 |
| distinct-wide-d-6 | 0.427 | 1.537 | 1.524 | 1.737 | 1.293 |
| distinct-chain-d-6 | 0.869 | 1.753 | 1.749 | 2.408 | 2.505 |
| distinct-wide-d-8 | 2.282 | 5.402 | 6.099 | 7.829 | 4.713 |
| distinct-chain-d-8 | 3.608 | 5.988 | 6.254 | 9.506 | 10.473 |

## What the controls distinguish

Shared beats one and two workers on cold time, first answer and metered peak in all
17 matched cases. Against Owned, two-worker cold ranges are worse in ten cases,
overlap in six, and are better only on distinct-wide-d-6. Against inline, two-worker
cold ranges are worse in eleven and overlap in six; none is separated in favor of
workers. Two versus one worker gives three lower cold ranges and fourteen overlaps.
Do not interpret the other median reductions as established wins.

For distinct-wide-d-6, two workers take 1.254–1.429 ms versus Owned's 1.483–1.794 ms.
This is a supported interface-level gain, but Shared's median is 0.427 ms. At depth 8,
two workers take 4.520–5.456 ms versus one worker's 5.972–8.503 ms; Shared takes
1.273–2.553 ms. Owned and inline ranges overlap the two-worker range there.

The admission controls strengthen the concurrency interpretation. For wide-large,
two-worker K1 takes 12.775–18.286 ms, K4 takes 9.036–11.982 ms. For distinct-wide-d-8,
K1 takes 8.250–10.004 ms and K4 takes 4.520–5.456 ms. Both also have separated
first-answer improvements. Inline and one-worker K1/K4 timing ranges overlap in both
queries. K4 increases metered peak in all six admission comparisons. These observations
concern a fixed width of eight, not general width scaling.

The chain controls permit only one outstanding equation. Their stores and output
counts differ from wide queries, so chain-versus-wide timing is not a pure concurrency
ablation. Within-case comparisons are the relevant controls. SK duplication exposes
up to three outstanding equations but gains no separated two-worker improvement;
its Shared cold range is 0.528–0.600 ms versus 2.441–3.067 ms with two workers.

## Costs and investigation of the losses

Whole-query losses are not explained solely by thread startup. For distinct-wide-d-8,
the measured search-phase medians are 1.618 ms Shared, 4.795 ms Owned, 5.417 ms inline,
6.989 ms one worker and 3.926 ms two workers. These phase medians are descriptive and
must not be summed as if they came from a single observation. The search phase itself
includes projection, service, installation and observation; it does not isolate solver
CPU time. A warm pool may help, but startup amortization alone does not explain this gap.

Actual work and source inspection identify a concrete representation follow-up. On
wide-large, the owned service performs 176 pair steps but 106,136 recursive resolve
visits. At distinct-wide-d-8 it performs 4,096 pairs and 69,680 resolve visits. The
[owned solver](../../../research/chr-reuse/src/lib.rs) clones the initial operands and
recursively constructs resolved terms for each pending pair, then returns complete
input-hole bindings. The shared arena retains compact interned terms. Pair counters
from different algorithms are not equal-cost units: the shared wide-large path actually
records more pair visits while remaining faster. This is evidence to investigate a
less costly transferable representation or coarser work boundary, not a quantitative
attribution of all elapsed loss to a single operation.

At distinct-wide-d-8, absolute construct-through-join heap peaks are 1,035,590 bytes
Shared, 1,172,513 Owned, 1,173,562 inline, 1,246,810–1,264,217 one worker and
1,273,900–1,324,146 two workers. Across all 17 cases, two-worker peak ranges exceed
Shared, Owned and inline; they exceed one worker in fifteen and overlap in two.
Peaks include fixture baselines, which the audit reports. Request bounds do not bound
payload bytes. Process RSS also includes thread/runtime storage and application fixture
setup; it cannot identify search-store bytes by itself.

The synthetic prefix drains four accepted equations and the type prefix one, without
committing additional source transitions. All actual work is counted after join.
Receipt-time operation counters can differ before shutdown because they count only
received replies; the final totals agree. This accounts for speculative costs instead
of treating first answer as the end of execution.

## Bounded recommendation and remaining investigations

Retain Shared as the competent cold-query control. Do not adopt this worker interface
as a project speed optimization on the present evidence. Preserve the worker protocol
as an experimental vehicle: useful parallel service is observable, and its adverse
representation and lifecycle costs are now measured.

Next investigate certified coarse-region workers under a matched owner/product
schedule. Their independent regional state can remain on its worker, avoiding
per-equation export and installation. This direction does not require fixing the owned
interface first. Separately, transferable shared representation, reduced clone/resolve
work, warm worker reuse, wider frontiers, more workers and resumable services could
change this comparison. Their costs and semantic boundaries require actual experiments;
none is closed by this result. The full research goal remains active.

Reproduce with the registered release binaries and unchanged inputs:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 research/chr-reuse/scripts/parallel_costs.py
PYTHONDONTWRITEBYTECODE=1 python3 research/chr-reuse/scripts/audit_parallel_costs.py
```

The runner preserves existing evidence and requires a separately recorded retry path
for another batch. Workload tests, workspace Clippy and formatting passed before the
frozen binaries were used. No experimental builds or other batches ran concurrently.
