# Native graph service can retain work, but its quota needs an explicit contract

A small scheduler over the pinned native heap returns finite answers beside looping alternatives without serializing source-machine states. The original interaction quota misses reference-only cycles; a quota covering reference unfolding and rewrites passes the revised gate. This qualifies a native service mechanism, not a complete CHR architecture.

## What was tested

The [registration](../registrations/S03-native-service.md) selects a concrete unresolved boundary from the earlier [native correspondence probes](E06-native.md) and [recursive-wrapper experiment](E06-hvm-boundary.md). The locally available HVM4 revision is `6defdfc7dae2a3cca5dd6e74ed0612385b5646a8`; its source hash is checked before every build. No claim concerns another runtime version.

The scheduler owns a queue of native heap locations. A service call weakly reduces one location, then either publishes a nullary value, queues native superposition children, records erasure, or retains unfinished reduction. The same heap survives all calls. Existing frame reconstruction reconnects pending work to heap nodes; it can allocate administrative nodes and is not zero-cost suspension.

Only nullary constructors and numbers are admitted as completed observations. A structured result is explicitly unsupported. Consequently, this gate does not test repeated choice references inside structured output, finite-tree equality, off-output failure or consuming CHR rules.

## Findings that changed the implementation

| Challenge | Observed result | Consequence |
|---|---|---|
| Unmodified collapse, looping first alternative | CPU limit ends the process without an answer or service return | Ordinary collapse remains an inadequate service boundary for this witness |
| Existing interaction quota, lambda recursion |24 configurations replay exactly; finite siblings appear and looping work remains | Retaining the native graph is feasible on these terms |
| Direct reference recursion, `@loop = @loop` | First call splits the root; second call does not return before the CPU limit | Reference unfolding must participate in the service quota |
| Quota on every traversal visit | At quota1, duplicate-choice work returns64 times without reaching a rewrite | Rebuilding traversal frames can restart the same prefix forever; return alone is not progress |
| Quota on reference unfolding and rewrites | All30 revised configurations pass and replay exactly | This transition boundary preserves the tested progress while keeping graph work resident |

The reference-cycle and traversal-restart findings are distinct. Increasing the budget would hide the latter on these small terms without establishing progress for deeper ones. The selected correction charges semantic reducer transitions rather than pretending that reconstructed traversal has retained its position. General traversal cost remains an obligation.

One initial harness check assumed an empty stack had position0. The pinned runtime initializes a sentinel at position1. The corrected check verifies restoration to that actual baseline; the initial record is retained.

## Complete gate results

Ten cases at quotas1/2/8 run twice: a finite atom, equal-valued alternatives, independent nested alternatives, lambda recursion in both sibling positions, two looping arms, three finite siblings beside a loop, reference recursion in both positions, and an unsupported structured result. All60 runs agree with independently specified raw leaf multisets and their expected pending/unsupported states. All30 paired records replay exactly.

Another30 runs disable interaction diagnostics. Answers, transition counts, stack restoration and final pending states are identical, while interaction increments are zero. Scheduling therefore has its own operational state rather than depending on measurement counters. Ten zero-call cancellations retain the initial task without reducer execution. Three terminating cases also agree with the unmodified native collapse control. The revised gate comprises103 successful processes.

At quota1, the finite atom completes in two service calls, duplicate equal answers in four, and four nested alternatives in eight. Continuing cases return through all64 calls, publish their expected finite siblings and retain one or two tasks as appropriate. Cancellation releases the scheduler queue and the enclosing runtime subsequently releases its global heap. This checks process-local control flow, not measured allocation restoration, reusable sessions or fine-grained reclamation.

## What the evidence does—and does not—support

The earlier wrapper failure does not force whole-state serialization. A retained native heap with a separate scheduler is now executable evidence for another path. It adds responsibilities: continuation reconstruction, work scheduling, admitted observation boundaries and explicit progress accounting.

The quota bounds counted transitions in these cases. It does **not** bound all CPU work: reference instantiation, structural descent, helper operations and frame rebuilding can depend on term or program size. The tests do not establish arbitrary-source fairness. No timing, RSS, requested-allocation advantage or architectural lifecycle superiority is claimed.

This does not select HVM over another native substrate. It establishes a viable local mechanism at low investigative cost using pinned source with an existing rebuild operation. The Rust graph controls remain useful source-level competitors, but their results cannot discharge this native mapping boundary.

## Next experiment and remaining obligations

T080 remains active. Next implement retained, incremental structured observation with explicit native choice correlation and off-output obligations. Compare its complete finite observations against the independent source/control semantics; include recurring work beside a finite structured answer. A root-only scheduler must not be promoted to a general source executor.

Then connect consuming source occurrences and compare serial effect ownership with local claims/commits, including abandoned claims and incompatible alternatives. Register costs only after those source and service gates pass. General traversal bounds, native-label freshness, memory lifetime and cancellation ownership must remain explicit. Broader finite solving, reuse and language investigations retain their required status.

This priority follows the concrete native service finding: the next missing observation boundary can determine whether this implementation reaches a credible CHR competitor. Additional leaf timing would not answer that question. The [current cycle](../next-cycle.md) retains the stronger independent alternatives and the eventual coherent-architecture comparison.

## Reproduction and retained evidence

The [driver](../../../research/chr-hvm/service/gate.py) builds the pinned unmodified control and the native scheduler derivative with Clang `-O2`. The [scheduler](../../../research/chr-hvm/service/scheduler.c) contains the queue operations; the driver records exact reducer instrumentation. Every child has2 CPU seconds,3 wall seconds and96GiB virtual-address allowance for the runtime's reservations. Compilation has a60-second bound. No physical-memory interpretation follows from the virtual allowance.

[Validation and hashes](s03-native-service/validation.json), [revised runs](s03-native-service/runs.json), [diagnostics-off runs](s03-native-service/counter-off.json) and [unmodified finite controls](s03-native-service/controls.json) preserve the successful evidence. [CPU-limit confirmation](s03-native-service/cutoff-confirmation.json), [reference-cycle failure](s03-native-service/reference-cycle-initial.json), [traversal restart](s03-native-service/visit-restart-failure.json) and [initial interaction-budget validation](s03-native-service/interaction-validation.json) preserve the limits. The [snapshot map](s03-native-service/interaction-source-snapshots.json) resolves the initial source hashes. The test sources are retained beside the records.
