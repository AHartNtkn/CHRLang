# Compiled controls now preserve the tested call-answer order

**The matched control exposed a scheduling defect, and the shared scheduler is
repaired.** All 768 source/query combinations now agree in delivered order across
scan, indexed access and inferred specialization. They also agree with independent
scalar complete-answer bags. The same preparation survives 2,304 cancellation
restarts per build; delivered outputs remain valid after preparation disposal.

## What failed and why it matters

The smallest encountered witness uses the ordinary caller, forward branch order,
left recursion depth one and right depth zero. Compiled scan delivered `out=a`
first; Direct delivered `out=b` first. Their residual resources also differ:
`a` selects `winner(a)`, while `b` leaves `watch(b)` and selects `winner(other)`.
This is observable when a consumer takes one answer and cancels.

The compiled search rotated branches after individual matcher microsteps.
Consequently, the amount of matching bookkeeping helped choose which answer
arrived first. The repair keeps a branch's turn while its matcher is unfinished.
Each tick still returns after bounded matcher work, so external cancellation can
interrupt selection. Once selection finishes, ordinary branch rotation resumes.
No fixture was excluded and no comparison was weakened to answer bags.

This is a scheduler repair shared by every SearchEngine caller, rather than a
special case in the cost runner. It changes scheduling and can change old
first-answer timings; earlier frozen cost receipts remain measurements of their
recorded binaries. The new campaign rebuilds every participating control.

## Validation and next comparison

The gate covers four caller families, both branch orientations, independent
recursion depths 0/1/4/8, two variable namespaces and three compiled modes. It
checks complete order, first-answer cancellation, abandoned searches at ticks
0/1/5, changed-query reuse and retained outputs. The gate passes with reuse
metrics enabled and disabled. The full compiled test suite passes with compiled
metrics enabled and disabled; scoped Clippy also passes.

[Registration](../registrations/S05-call-compiled-entry.md) ·
[Failure and validation receipts](raw/s05-call-compiled-entry/) ·
[Executable source gate](../../../research/chr-reuse/tests/call_compiled.rs).

The next experiment uses these controls in the existing complete lifecycle runner.
It measures whether the call-trace gains survive stronger execution and accounts
for their heap costs. Generated execution, broader observer interleaving and
bounded retention remain concrete investigations; this source gate resolves none
of their cost questions by itself.
