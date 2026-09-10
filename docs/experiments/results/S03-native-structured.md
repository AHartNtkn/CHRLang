# Retained native observation preserves structured choice correlation

The native observer now returns complete structured answers while preserving repeated-choice correlation and raw multiplicity. Explicit completion obligations prevent off-output failure from disappearing. Finite structured alternatives remain observable beside continuing native recursion; consuming CHR effects and general source correspondence are still untested.

## The mechanism that made the difference

The [leaf service gate](S03-native-service.md) retained native reduction but could not observe constructors containing unresolved choices. The pinned runtime's ordinary collapse visits every child before lifting a choice, so a looping child can prevent access to a finite alternative.

The new observer retains constructor traversal frames for each queued alternative. Each service action reduces one location, enters or completes one constructor frame, or lifts an exposed choice through one parent. Lifting copies the other fields through the native labelled-copy operation. This preserves correlation when the same source choice occurs elsewhere in the structure. Pending alternatives rotate through the queue; the graph remains in the same native heap.

A child failure propagates through its constructor parents. An answer is published only when its entire admitted first-order structure has completed. The observer does not call blocking CNF between service returns. Constructor traversal retains its position, unlike the traversal-restart counterexample in the preceding gate.

## What the experiments establish

| Question | Independently expected observation | Result |
|---|---|---|
| Reuse one choice in two fields | `(A,A)` and `(B,B)`, without cross-pairs | Both answers, including nested fields |
| Use two independent choices | All four pairs | All four, with raw counts preserved |
| Preserve equal-valued alternatives | Two identical wrapped answers | Two raw answers |
| Vary choice-reference order | Every assignment agrees across two oppositely ordered lists | All permutations of1/2/3 choices, with and without enclosing constructors, pass |
| Keep an off-output failure relevant | A joined failing obligation prevents an answer | No answer |
| Preserve an off-output birth | Equal successful completion arms retain two histories | Two identical answers; replacing one arm with failure leaves one |
| Reject only one correlated alternative | The output and checking obligation use the same choice | Only the accepted structured answer |
| Expose a finite structured sibling beside recursion | One finite answer and unfinished work | Preserved in both sibling positions, under nested constructors and with reference-only recursion |
| Discard a failing computation without joining it | The intended source has no answer | The faulty encoding still emits an answer; this remains a negative correspondence result |

The last row is deliberately not a qualified source translation. Matching its predicted native behavior demonstrates that the observer cannot recover an obligation absent from the graph. A compiler must carry source failure and choice-birth obligations into the completion protocol.

## Validation and cancellation

The final matrix contains33 cases at transition quotas1/2/8, each run twice. All99 paired records replay exactly. Another99 runs disable interaction diagnostics and preserve answers, scheduler transitions, pending roots and native stack restoration. The33 cases include one deliberately faulty disconnected-failure encoding; its predicted result is checked separately from the intended source observation.

All132 cancellation runs match the exact service-trace prefix of uninterrupted execution at0/1/4/16 calls. Their pending-root counts agree, and emitted answers are prefixes of the completed run's observations. Twenty-nine terminating programs agree with the unmodified native collapse control. A higher-order result is explicitly unsupported; a260-deep constructor fails the256-frame bound without partial output. The final gate comprises460 processes, including these two expected admission/boundary outcomes.

The largest terminating case finishes within663 of the1024 allowed service calls. Each continuing case returns through all1024 calls, emits its expected finite structured sibling and retains one pending root. These observations establish progress for the tested programs, not arbitrary-source fairness.

The initial test correctly failed because the leaf observer reported the plain structured answer as unsupported. The subsequent13-case gate passed before the permutation, stronger cancellation and off-output-history challenges were added. Initial and final records remain available.

## Architectural consequence and limits

A native implementation can retain both reduction work and constructor-observation progress. The earlier blocking-wrapper result therefore does not establish a need to serialize the entire source machine at each yield. This path instead needs per-alternative traversal frames, a completion protocol, native labelled copying and an explicit scheduler.

The tests exercise bounded, explicitly allocated choice labels. They do not supply a dynamic freshness allocator or justify reusing a label while related graph nodes remain live. Nor do they establish source-level unknown identities, finite-tree equality, propagation history, competing consumption or local claim/commit ownership.

The operational quota counts reference unfolding and rewrites; constructor traversal has separate service actions. Internal weak-reducer descent, reference instantiation, native copy helpers, frame rebuilding and final printing can still perform size-dependent work. The fixed256-frame limit is an experimental resource bound with an explicit error, not a proposed language restriction. Uniform service-cost bounds and reusable-session memory ownership remain open.

No timing, requested-allocation, RSS or complete-lifecycle advantage is claimed. This is native graph observation evidence, not a generated CHR compiler or a whole-architecture comparison. The reference interpreter is unchanged.

## Next: consuming source correspondence

T080 remains active. The next gate must compile actual consuming source operations into this native path, preserving occurrence identity, source-priority effects, branch-local resource availability and complete residual observations. Compare against independent source semantics and a qualified ordinary executor. Include shared values with incompatible consumption, repeated equal-valued births, linked failure and cancellation with pending effects.

Start with an explicit serial effect owner and compare the obligations of local claims/commits on the same sources before connected-parallel timing. A constructor receipt alone cannot prove a resource was consumed exactly once. Dynamic label allocation, source admission and equality boundaries must be stated and tested wherever the translation uses them.

That source gate is more decision-relevant than timing the observer in isolation: it determines whether retained native execution can become a credible complete competitor. Broader finite solving, reuse, language choices and coherent architecture comparison remain required under the [current cycle](../next-cycle.md).

## Evidence

The [registration](../registrations/S03-native-structured.md) records the original gate and prospective challenges. The [driver](../../../research/chr-hvm/structured/gate.py) reproduces the preceding pinned reducer hooks without changing their source; the [observer](../../../research/chr-hvm/structured/observer.c) implements retained traversal and lifting. The pinned upstream source hash is checked before compilation. Clang builds the derivative with `-O2`; process bounds remain2 CPU seconds,3 wall seconds and96GiB virtual address space for the runtime's reservations.

[Final validation and hashes](s03-native-structured/validation.json), [complete runs](s03-native-structured/runs.json), [diagnostics-off controls](s03-native-structured/counter-off.json), [cancellation prefixes](s03-native-structured/cancellation.json), [unmodified finite controls](s03-native-structured/controls.json) and [explicit rejection outcomes](s03-native-structured/rejections.json) retain the evidence. [Initial validation](s03-native-structured/initial-validation.json) and the [initial source map](s03-native-structured/initial-source-snapshots.json) preserve the earlier13-case gate.
