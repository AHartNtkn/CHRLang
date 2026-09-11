# Compressed private calls now resume the live caller on schedule

**The integrated caller matches every tested delivered prefix and exhaustion point
across 28,608 service checkpoints.** This includes competing consumers, surviving
propagation history, unequal branch lengths, fresh output aliases and failure.
Suspended private work can receive caller input and resume correctly.

## The boundary that makes this work

The [registered integration](../registrations/S05-call-trace-caller.md) checks a
private single-head priority prefix. At a settled cursor, it admits a call only
when exactly one private occurrence is live and a private rule is applicable.
The remaining caller stays in its original persistent cursor, retaining its
bindings, live occurrence order and history.

The isolated trace's initial insertion is skipped because the occurrence is
already live. Its completion event imports bindings and any suspended private
facts, then services the caller in that same tick. Thus neither splice inserts a
source-service delay. A split clones the remaining caller cursor and schedules
the trace children separately in the same FIFO queue. It does not run either
branch atomically.

The suspension protocol carries residual private facts as well as results. A
caller rule can supply a missing binding; a later settled cursor then re-enters
the private trace. Result and residual terms share one renaming map, preserving
fresh aliases across that boundary. Completed trace machines are still released.

## What the tests establish

The main matrix has 256 sources: two depths independently chosen from 0/1/4/8,
both branch orders, four output/failure modes and two variable namespaces. Modes
include a binding followed by further private work, so output binding and phase
completion are distinct events. Caller propagation runs before entry and retains
its history; two rules compete for a token after the private phase.

Each source is interrupted at 0, 1 and 5 steps and restarted with the same table.
The 768 complete runs make 28,608 one-step comparisons of delivered answers and
exhaustion against Direct. Every complete answer bag also matches the independent
scalar interpreter. The metrics build requires reuse hits in every matrix case,
so a pass through Direct alone cannot satisfy the gate.

Additional tests cover:

- A private call suspended until a caller rule supplies its input, including reuse.
- A fresh variable shared between a returned constructor and a suspended private
  fact, then bound by the caller under a changed variable namespace.
- Foreign caller-run rejection and external access to the private head family.

All four caller tests pass with metrics enabled and disabled. The 23 isolated-call,
result-table and effectful-boundary regressions also pass in both configurations.
Scoped Clippy passes. [Metrics receipt](s05-call-trace-caller/metrics.log) ·
[Counter-free receipt](s05-call-trace-caller/counter-free.log).
No reference interpreter code changed.

## Architectural consequence and next experiment

This establishes a live-caller use of the compressed protocol under a checked
source property. It goes beyond independent call-job scheduling: caller effects,
history and resumed input participate in the checked whole source. It does not
establish arbitrary higher-priority observer interleaving. Such observers require
an additional protocol; the private-prefix premise is not a language restriction.

The wrapper currently performs real admission work, exports call arguments,
imports results and retains query/prepared state. Those costs must be measured.
T075 next qualifies complete lifecycle ownership against Direct and existing
transition reuse, with repeated and unique calls, changed queries, early failure,
retained outputs and cancellation. Then measure ordinary counter-free time.
Include the caller cursor, partially built traces and prepared-table disposal;
do not infer heap savings from compressed node counts.

Sparse projection remains the strongest pending alternative, but the live-caller
gate now gives call reuse a concrete end-to-end cost question. This is package two
since the portfolio review. The research goal remains active.
