# One call key can preserve service while compressing linear progress

**The new call-owned trace preserves each tested source service event without a
keyed state at every transition.** A 22-step linear call occupies one trace node;
a completed replay executes no source transitions. This establishes a resumable
operation protocol. It does not yet establish a mixed-caller substitution or a
cost advantage.

## What the implementation does

The [registered gate](../registrations/S05-call-trace-entry.md) reuses the existing
private single-head admission and canonical argument key. A call entry looks up
that key once. Each trace node records a count of uninterrupted Continue events,
then an explicit split, failure or owned answer. A split creates two independently
schedulable child jobs. A job replays exactly one event per service request.

Unfinished traces keep their live cursor; another matching job can advance that
same trace as needed. Cancellation drops the job without forcing completion.
When all branches finish, the trace releases its execution machine and arena.
Only compressed events and owned result terms remain. Every answer transport
preserves interface identities and allocates fresh internal variables through the
caller's coordinated allocator.

The existing atomic result table remains a distinct experimental control. Shared
admission and canonicalization now have one implementation used by both tables.
No reference interpreter or persistent execution semantics changed.

## Evidence

The deterministic matrix covers 64 cases: depths 0/1/4/8, both branch orders,
success/failure alternatives, repeated/distinct alias keys and two variable
namespaces. Jobs from two callers share one FIFO queue. For every step, the test
compares Continue, Split, Failed or Answer with the direct persistent stepper;
every answer is compared immediately. Completed bags also match the independent
scalar interpreter.

Across complete runs and restarts, **9,088 service events agree**. Each case is
interrupted at 0, 1 and 5 events and restarted using the same partially populated
table. Completed replay is also checked. Additional cases verify fresh existential
identities across callers, foreign-table rejection, private-family admission and
repeatable errors for suspended calls.

| Linear-call witness | Observation |
|---|---|
| Eight recursive waits, complete call | 22 source steps, one trace node |
| Same completed call again | No additional executed source transitions |
| Distinct four-wait key | Additional execution, second trace node |
| Three entries across that sequence | Three lookups, one hit |
| Aggregate work | 36 executed transitions, 22 replayed events |
| Completed first trace | No retained execution machine |

All four new tests pass with metrics enabled and disabled. The 22 existing call,
finite-entry and effectful-boundary tests also pass in both configurations. Scoped
Clippy passes. [Metrics receipt](s05-call-trace-entry/metrics.log) ·
[Counter-free receipt](s05-call-trace-entry/counter-free.log).
Counters establish work, not time or heap savings; retained answer terms and trace
ownership still need complete measurement.

## The next gate

The tested scheduler interleaves separate isolated call jobs. It does **not** yet
extract a call from a running mixed caller, publish intermediate bindings into
that caller or arbitrate its external consumables. The existing private-family
check cannot establish those responsibilities. Treating this as a complete caller
optimization would repeat the atomic-boundary error identified by earlier tests.

T075 next integrates the protocol at an explicit source boundary and checks the
entire caller. Use unequal-length branches, intermediate output observation,
competing consumables, fresh aliases, failure and cancellation/restart; compare
complete source answers and every observable prefix. Repair the boundary and
fixtures if those tests expose a discrepancy. Charge any replay/import work in
subsequent costs rather than hiding it in setup.

This integration remains more discriminating than sparse projection immediately:
the new protocol actually removes per-transition keys and linear trace nodes while
preserving isolated service. Whether those savings survive a real caller is the
next architectural question. Sparse elimination remains a strong pending
alternative. Package count is one since the portfolio review; the goal is active.
