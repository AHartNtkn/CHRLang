# E16 distinct-hole pilot

All 60 children pass: six workloads, five execution modes and separate timing/allocation
runs. Distinct holes produce the predicted growth in actual solver work, so these sizes
are suitable for a repeated comparison alongside the first pilot's repeated subtrees,
identity equations, skew, failures and applications.

The [registration](../registrations/E16-distinct-pilot.md), [raw data](E16-distinct-pilot.jsonl),
[manifest](E16-distinct-pilot-manifest.json) and [audit](E16-distinct-pilot-audit.json)
record the full run. Wide depths 4/6/8 require exactly 256/1,024/4,096 owned pair steps;
matched chains require 249/1,017/4,089 because they have one tag equation instead of
eight. Shared and all owned modes agree on committed source execution and observations.
Owned operation totals agree across inline/workers and timing/memory at matched limits.
No bound or execution failure occurs. Maximum process RSS is 5,120 KiB and maximum
metered heap peak is 1,674,678 bytes, including the prepared fixture baseline.

The single-observation depth-8 wide cold times through search release are 2.239 ms
Shared, 4.710 ms Owned, 7.132 ms inline, 7.792 ms one worker and 5.794 ms two workers.
The corresponding heap peaks are 1,035,590 / 1,172,513 / 1,173,562 / 1,247,102 /
1,277,470 bytes. These exploratory values motivate repeated measurements; they do not
establish speed rankings. In particular, recovering owned-service time would not show
an improvement over the shared arena.

Distinct holes also grow complete substitutions and owner installation. This is a
whole-interface workload, not an isolated solver benchmark. A service-only timing
would omit costs central to the project decision. The earlier repeated-subtree controls
remain relevant: both representation-friendly and solver-heavy shapes must appear in
the repeated comparison. Coarse regions, warm-pool reuse and representation transfer
remain separate feasible investigations.

Independent reference workload tests, actual prefix-drain checks, workspace Clippy and
formatting pass. Reproduce after release builds with:

```sh
PYTHONDONTWRITEBYTECODE=1 python3 research/chr-reuse/scripts/parallel_pilot.py --distinct
PYTHONDONTWRITEBYTECODE=1 python3 research/chr-reuse/scripts/audit_parallel_pilot.py --distinct
```
