# Call reuse retains scoped gains against compiled execution

**Eight repeated-query cases are faster than every control tested here.** Their
median trace/control time ratios range from 0.479 to 0.756 across Direct, compiled
scan, compiled indexed access and inferred specialization. They request fewer
bytes in total, but keep more memory live at the peak. Reuse is therefore a real
candidate for these repeated computations; these results do not select a general
architecture.

## Complete costs

All ratios divide call-trace cost by the named control. Smaller is better.
“Uncertain” means the ten repetitions did not meet the registered directional and
10% magnitude rule; it does not establish equality.

| Control | Faster / slower / uncertain, of 144 | Median time-ratio range | Requested-byte ratio | Peak-live-byte ratio |
|---|---:|---:|---:|---:|
| Direct | 10 / 20 / 114 | 0.629–1.791 | 0.402–1.262 | 1.324–4.448 |
| Compiled scan | 13 / 11 / 120 | 0.471–1.557 | 0.658–1.699 | 0.840–3.124 |
| Compiled indexed | 39 / 0 / 105 | 0.420–1.140 | 0.468–1.174 | 0.679–3.015 |
| Inferred specialization | 17 / 8 / 119 | 0.564–1.836 | 0.559–1.415 | 0.664–3.622 |

The controls use the same caller sources and repaired branch scheduling. They
reuse preparation across changed query variables. No source family or failed
measurement was omitted. Indexed access is not presumed cheaper on these small
stores; its own preparation and maintained state are charged.

Repeated queries account for every qualified gain against Direct, scan and
specialization, and 37 of 39 indexed gains. The other two indexed gains are
single-query cases. Four distinct queries produce no qualified gain against any
control. Thus repeated computation is the strongest current opportunity, while
changed-query economics still need improvement.

## The gains that survive every control

All eight cases use depth 128 and four repeated queries. They span ordinary
success, a failed alternative, fresh output structure, and binding before private
work finishes. Both full consumption and first-answer cancellation occur among
them. [The diagnosis](s05-call-compiled-cost/diagnosis.json) identifies each exact
case, including output retention and per-control ratios.

Across these eight cases and four controls, requested-byte ratios are
0.402–0.712. Peak ownership is 1.998–2.135 times Direct, 1.350–1.517 times scan,
1.301–1.451 times indexed, and 1.527–1.726 times specialization. Fewer allocations
in total do not imply a smaller live working set.

The phase records support the expected reuse mechanism. Within these selected
cases, the median ratio of later-query service time to first-query service time
is 0.206 for traces, versus 0.978–1.005 for recomputation controls. Service plus
observation accounts for a median 63% of trace cost and 72–79% of control cost.
These are post-measurement diagnostics of the selected cases, not additional
qualified comparisons or population estimates.

## Timing variability remains visible

The preceding campaign qualified eleven Direct gains. In this independent
campaign four of those exact cases qualify again; seven are uncertain because
individual repetitions cross ratio one. Their medians are still 0.629–0.821.
This campaign also qualifies other cases. Both campaigns and every repetition
remain available; there is no pooling or selection of favorable runs.

This supports a scoped repeated-computation opportunity, with case-level
classification sensitive to variability. Further precision should target a
concrete architecture choice after the stronger generated control, rather than
turn uncertain cells into assumed wins.

## What the measurements include

The 720 cells combine five executors, four families, depths 4/32/128, one query /
four repeated / four distinct queries, retained outputs 0/all, and complete /
first-answer cancellation. Two meter runs and ten ordinary runs per cell produce
**8,640 terminal processes**. Ownership agrees exactly across repetitions;
consumer output bytes and counts agree across executors; all disposal sequences
restore the initial heap root.

The timed sum includes source construction, preparation, input, setup, service
and observation, output consumption, and every source, query, preparation and
consumer disposal phase. Complete semantic checks occur outside those intervals.
The three clock checks set a 3,000 ns qualification floor; all 576 paired
comparisons exceed it. Ordinary binaries use no engine or persistent counters and
the ordinary allocator. Allocation diagnostics run separately and measure
requested heap bytes, not RSS.

Process startup and Rust compilation are not part of this sum. The next generated
artifact comparison must charge its emission/build costs separately and show
amortization; this campaign cannot establish full compiler lifecycle superiority.

## Decision and next experiment

**Keep T075 active and use the existing generated-access emitter next.** Generic
matcher service still accounts for most control time, and generated matching
could change whether reuse is worth its retained state. `generate::emit_access`
and the artifact-compilation workflow already exist; no new baseline executor
is required. Preserve this same-source order/cancellation gate for the artifact.

Bounded retention and recomputation remain the strongest alternative: retained
completed keys and unfinished calls have measured costs. They follow the stronger
execution comparison unless the generated source gate reveals a more consequential
problem. Broader observer interleaving and sparse finite projection remain required
independent investigations. These results impose no language restriction.

This is package two since the last full portfolio review. The 57-question map and
active goal remain open.

[Registration](../registrations/S05-call-compiled-cost.md) ·
[Source-order repair](S05-call-compiled-entry.md) ·
[Complete receipts and frozen inputs](s05-call-compiled-cost/) ·
[Analysis](s05-call-compiled-cost/analysis.json).

Reproduce checks with `python3 research/chr-reuse/experiments/call_compiled_cost.py audit`
and `python3 research/chr-reuse/experiments/call_compiled_diagnosis.py audit`.
