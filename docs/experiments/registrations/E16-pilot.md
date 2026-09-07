# E16 exploratory resource pilot

This pilot selects feasible cost-comparison sizes and detects lifecycle or accounting
problems. Its one-observation timings do not rank architectures. The semantic worker
boundary has passed the [E16 gate](../results/E16-gate.md).

## Questions and controls

Can bounded independent equation tasks grow enough to repay owned projection and
worker overhead? Which costs dominate when equations are cheap, structurally shared,
sequential, or uneven? Does accepted speculative work materially affect prefix cleanup?

Use Shared and Owned scalar execution, inline lookahead, one worker and two workers.
Use outstanding limit 4 and lookahead 8 for the three new drivers; scalar limit is 0.
Additionally run all three new drivers at outstanding 1 on wide-large. The reply
channel stays at one slot. Do not infer general scaling from two workers or equal
worker performance from equal CPU affinity. Worker stacks use the current standard
library default with RUST_MIN_STACK recorded; physical stack reservation is not a
Rust heap-request counter.

## Workloads and expected observations

The shared fixture module constructs eleven cases. Balanced binary trees have depth
0, 6 or 10, avoiding a confound between larger work and deep recursive stack usage.
An eight-way balanced explicit disjunction supplies independent branch equations.
Each successful alternative returns its distinct tag and its checked hole value.

- wide-cheap/medium/large: eight successful alternatives at depth 0/6/10.
- wide-identity: eight depth-10 equations with identical operands and nonground outputs;
  this deliberately preserves the competent shared-arena shortcut.
- chain-large: eight depth-10 equations with separate fresh holes on one continuation;
  no branch parallelism and no grounding of later tasks by an earlier task.
- skew-first/skew-last: one depth-10 alternative and seven depth-0 alternatives, with
  the expensive alternative at opposite FIFO positions.
- mixed: three depth-6 successes, three constructor mismatches and two occurs failures.
- prefix-drain: eight cheap alternatives, with one extra dependent equation in each
  later alternative; stop after the first trusted answer. A focused test requires
  accepted uncommitted service work at this boundary.
- unchanged E00 SK duplication evaluation and type-synthesis first-answer prefix.

All fixtures have hand-derived expected full answers and raw counts. A separate test
runs the independent reference against them, including five failed branches for mixed.
Every cost child checks full observations outside measured intervals. Fixed source
step budgets are errors if they fail to reach the expected exhausted/prefix stop.
The application fixtures are selected through the E00 registry before measurement;
this setup can influence process-wide RSS and allocator state. Synthetic cases are
constructed individually. No pure search-RSS claim follows from process high-water marks.

## Measurements and bounds

Build two release executables using a common harness. Timing explicitly uses System
allocation and no worker hook, trace vector or atomic allocation counters. The separate
memory executable meters allocations; its timing fields are diagnostic only.

Start before input cloning and Search construction. Record constructor return, first
trusted answer collected, expected search stop, shutdown/join, and search drop.
Capture numeric source/broker/work counters at stop and after join; accepted work must
finish even on prefix stops. Keep outputs through search release. Validate them outside
timed intervals, then measure output drop separately. Compare shutdown plus search drop
across engines: parallel shutdown clears parked state while scalar shutdown is a no-op.
These are cold per-query worker lifecycle measurements; the current API has no reusable
warm pool.

Reset/read allocation peaks only before construction and after joined shutdown, then
at search/output drop boundaries. The combined construct-through-join interval includes
worker startup and speculative completion. No worker-quiescence barrier exists at
constructor return or prefix stop, so allocation subphase peaks there are not claimed.
Record baseline, requested bytes, calls, peak and retained live bytes. Concurrent heap
accounting is not a synchronized physical-memory trace and excludes OS stacks and
allocator overhead. Record whole-process maximum RSS separately.

Run one fresh subprocess per cell: 11 cases × 5 controls + 3 limit-one controls =
58 timing cells, then 58 separately metered cells. Before execution record source and
binary hashes, release build/toolchain, host, CPU affinity, available cgroup information,
RUST_MIN_STACK, child order, 30-second wall limit and 1-GiB address-space limit. Retain
partial stdout/stderr and timeout/error outcomes. No concurrent builds, tests, profiles
or other experimental batches during the pilot.

## Interpretation

Check semantic status and deterministic source/operation work across modes at matched
limits, accounting for speculative issuance and shutdown. Inspect whether intended
parallel cases actually have multiple outstanding operations. A single observation is
only a sizing/phase diagnostic. Treat a timeout, no exposed concurrency, implausible
counter or unexpectedly large lifecycle cost as a question to investigate. Do not
select favorable cases or close parallelism because this granularity loses. Freeze
repetition/order and an expanded size/width matrix only after interpreting the pilot.
