# Ordered projection does not yet beat the simpler complete path

Projection wins 36 of 64 comparisons against full source execution, but only 3 against enumeration. The dense lazy-enumeration challenge finds 0 projection gains, 2 losses and 6 uncertain comparisons. Current evidence does not justify choosing projection over the simpler path.

The comparison now includes complete consuming execution. Projected assignments
enter the same prepared Direct caller as enumerated assignments. A third path
runs the full source through Direct. Reference answers are checked outside timing;
the timed source carries only the variables and outputs it needs.

## Complete lifecycle results

| Projection compared with | Gains | Losses | Uncertain |
|---|---:|---:|---:|
| Enumeration | 3 | 25 | 36 |
| Full source through Direct | 36 | 8 | 20 |

A gain requires at least a 10% median paired improvement and all five blocks in
the same direction. Other measurements remain uncertain. Every final comparison
clears the recorded clock floor; there are no workload weights or aggregate score.

The primary matrix covers 64 scenarios: two/four coordinates, sparse/dense
admissibility, uniform/unequal branch lengths, one/eight queries, immediate/held
answers and full/first-answer consumption. The domain contains duplicate a choices.
All modes pay for source construction, preparation, setup, first observation,
remaining execution, consumer growth, cancellation and disposal. Prepared rules
are reused; each delivered assignment runs its own branch-local caller.

## The dense challenge separates solving from lazy delivery

A dense region imposes no hidden constraints. Its candidate set needs no solver,
but eager enumeration still constructs and sorts it before returning an answer.
Lazy enumeration uses the same frontier as projection, with domain multiplicities
computed directly. It filters sparse candidates after admission when needed.

| Projection compared with | Gains | Losses | Uncertain |
|---|---:|---:|---:|
| Enumeration | 2 | 1 | 5 |
| Full source through Direct | 4 | 2 | 2 |
| Lazy enumeration | 0 | 2 | 6 |

This challenge covers all four dense padding/retention combinations at four
coordinates and eight queries, both full and cancelling consumers. It includes
every cancellation case that qualified during the pilot and validation reruns.
The changing individual gain locations are evidence against treating a single
case as a settled crossover.

## Ownership and where the work remains

Requested bytes: projection is lower than enumeration in 12/64 cases and higher in 52. Against Direct it is lower in 48/64 and higher in 16. In the lazy-enumeration challenge it is lower in 0/8, equal in 0 and higher in 8.

Peak requested-live ownership: projection is lower than enumeration in 10/64 cases and higher in 54. Against Direct it is lower in 64/64 and higher in 0. In the lazy-enumeration challenge it is lower in 0/8, equal in 8 and higher in 0.

Ordering remains real work: each accepted coordinate tuple can create neighboring
frontier entries and visited-position records. Complete output also starts the
same consuming caller repeatedly for hidden derivations with the same visible
value. These responsibilities deserve investigation before more relation-building
tuning. The combined execution phases do not by themselves separate caller time
from ordering time.

## Validation and interpretation

The final datasets contain 448 allocation and 1120 ordinary processes across the
main matrix and dense challenge. Two allocation repetitions agree, retained
consumer ownership agrees across modes, and every measured lifecycle returns to
its initial requested-live allocation. Every run validates the complete expected
answer sequence outside timing, or its first-answer prefix when cancelling.
Held answers are checked after prepared state disposal. Consumer buffers grow
dynamically; their sizes do not come from oracle answers.

The 192-case ordered-source test passes after sharing its implementation with
the runner. All 21 relevant counter-free tests, scoped allocator-build Clippy,
and 16 sparse/dense lazy-enumeration smoke cases pass.

Counter-free builds use the ordinary allocator for timing. Requested allocation
uses a separate allocator-instrumented build and is not RSS. The reference
interpreter is unchanged and excluded from measured intervals. Compiler costs
are outside this pilot. Source-order correctness comes from the preceding
192-case experiment and each runner invocation; this does not establish equal
interpreter-step-budget observations or a general source-plan compiler.

## Next: repeated consuming continuations

Investigate avoiding repeated execution of an identical branch-local caller when
multiple hidden derivations deliver the same visible value. Keep raw multiplicity,
ordered publication, caller context, fresh variables, retained results and
cancellation explicit. Compare reuse on both projection and enumeration so the
result cannot attribute a common improvement to the solver. Separate caller work
from ordering, then measure the combined lifecycle against the existing controls.

This could change the complete-path comparison more than another projection-only
optimization. It also connects directly to the ready question of broader call
reuse. Select it under T076, with T075 caller-context evidence as a control;
selective retention remains required. Package count three since portfolio review;
review the full portfolio after the next package or a consequential obstruction.
The research goal remains active.

[Registration](../registrations/S06-ordered-cost.md) ·
[Main timing data](S06-ordered-cost.json) · [Main allocation](S06-ordered-allocation.json) ·
[Challenge timing](S06-ordered-challenge.json) · [Challenge allocation](S06-ordered-challenge-allocation.json)
