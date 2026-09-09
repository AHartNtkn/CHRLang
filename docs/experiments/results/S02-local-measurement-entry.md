# Local execution has a counter-free path and tested cancellation ownership

The same local executor now runs with compile-time diagnostics enabled or disabled. Both variants preserve the qualified source answers, and isolated requested-allocation runs restore every query and prepared-plan baseline after completion or cancellation. This removes two obstacles to lifecycle measurement; no comparative timing matrix ran.

## Diagnostics and semantics

`Run<false>` omits counter updates and consumed-token trace recording. Resource removal remains unconditional. `Run<true>` retains the existing diagnostics; all 36 prior dependency-work records replay exactly. The source plan and operational code are shared between the two instantiations.

All **149 body configurations** now check three additional counter-free local controls, alongside the existing seven controls: **1,490 complete-answer comparisons** per constructor-suite run. The counter-free cases resume one operation at a time, including failed bodies, aliases and the contested older-consumer source. Their counters stay zero and the consumed-token vector never allocates. Successful quiescence has no pending equations.

Release LLVM evidence marks the counter-free recursive equality and pattern functions' receiver as read-only; diagnostic versions can write it. The [IR excerpts and hashes](s02-local-measurement-entry/ir-evidence.json) preserve that observation. This corroborates source-level compile-time guards and the runtime diagnostic checks; it is not a general speed measurement.

Both `Run` instantiations occupy **416 bytes**, including **128 bytes of diagnostic fields** that remain inert in counter-free execution. This storage has not been optimized away from the type. A future measurement must charge actual representation size rather than assume that disabling updates removes all instrumentation-related footprint.

## Cancellation and output ownership

`advance(budget)` returns false when its budget ends before quiescence or failure. Returning preserves the pending body and equations; it does not allow another rule to compete midway through a body. The existing `settle` operation retains its 200,000-operation bound. A service cutoff remains unfinished work.

The isolated ownership executable uses two sources, three dependency modes, three changing queries and four stopping points: immediately after setup, with an equation pending, with a body pending, and complete execution. One source recursively creates equations, consuming requests and fresh residual aliases; the other creates an equation and then fails. Independent scalar answers are prepared before allocation intervals.

Each diagnostic configuration has **72 query cases and six prepared-plan lifetimes**. Two counter-free and two diagnostic processes give **288 query restoration checks**, including cancellation at actual pending states, and **24 prepared-plan restoration checks**. The two runs within each configuration have identical JSON allocation records. Successful answers remain independently valid after query disposal, and releasing them restores the plan-only baseline.

Preparation, setup, execution, observation, query disposal, answer release and plan disposal have separate allocation readings. Independent answer validation and output serialization occur outside those phase intervals. The final restoration reading spans validation and nested meter resets: it is explicitly an ownership check, **not a lifecycle cost total or peak**. Requested heap bytes are not RSS. The meter's own allocation self-check runs first in each isolated process.

Diagnostic token recording adds 32–224 requested bytes during completed execution in these small witnesses; counter-free recording allocates zero bytes. These figures explain an instrumentation difference, not the relative cost of integrated and compiled architectures.

The [registration](../registrations/S02-local-measurement-entry.md), [validation audit](s02-local-measurement-entry/audit.json), [source gate](s02-local-measurement-entry/source-gate.log) and [scoped strict Clippy](s02-local-measurement-entry/clippy.log) preserve commands and evidence. The ordinary relational suite passes 53 tests; the diagnostic constructor suite passes 23. Dependency build-script warnings remain visible in the logs. Independent scalar and reference-interpreter code are unchanged.

## Four-package breadth review and next selection

This completes the fourth mechanism package since the previous breadth review: equality dependency comparison, source bodies, equivalent compiled selection, and this measurement/ownership entry. Together they establish a local representation contrast and credible source controls; they do not resolve general integrated execution.

**Select the ordinary-allocator lifecycle runner and bounded comparative sizing next.** It must reuse the qualified source plans, include the compiled generic and specialized controls, separate first/full observation where meaningful, and account for cancellation and disposal without charging oracle work. Register exact sources, sizes, repetitions, resource limits and interpretation before comparative runs. The allocation ownership executable here is not a substitute for that complete measurement path.

Repeated dynamic reunion is the strongest ready distinct ownership alternative: it could recover economical explicit search where independence changes repeatedly. Direct resource-aware lowering is another substantive alternative because it could eliminate the execution being measured. Both require additional source/implementation qualification. The selected lifecycle comparison has lower prerequisite cost now and can determine whether the existing integrated representation changes total costs, including adverse repair and low-yield cases. More selector or dependency refinement before that comparison would lack this evidence.

After sizing, reconsider those distinct mechanisms before another local refinement. Attribute a consequential loss, challenge a gain, and resolve uncertainty when it could change the choice; otherwise advance to the broader sequence. General heads, propagation history, multiple-rule scheduling, sustained lifetime and whole architectures remain required. Compilation costs are still outside any complete architectural superiority claim.
