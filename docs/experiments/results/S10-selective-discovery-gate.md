# Filtering conditional candidates saves tuples, but can add discovery work

Selective discovery preserves the tested complete answers while reducing candidate tuples on recursive and branch-specific sources. It adds service ticks on the dense and early-failure controls. The next question is whether reduced retention and matching repay the construction of filtered candidate lists across the full lifecycle.

This T078 gate implements a bounded mechanism in the conditional engine. It does not establish a time or memory improvement, choose a default, or settle broader conditional execution. The existing enumeration remains the control; the `selective-discovery` build feature enables the experimental comparison.

## What changes

The existing engine forms distinct-occurrence tuples from predicate/arity buckets, including occurrences that have been fully consumed and immutable constructor structures that cannot match a rule head. The new mode builds filtered lists for each anchored head before forming tuples. It inspects at most one bucket occurrence per discovery call; each structural check is bounded by the prepared head pattern. It retains the existing ordered candidate keys, supported matching, dependency notifications and round scheduler.

The filter uses two facts. An occurrence whose live support is exactly false cannot become live again: resource commits only subtract consumed support, and a new post receives a new occurrence identity. An immutable constructor mismatch cannot become a match through variable binding. Unknown terms are retained even when their present conditional bindings might suggest rejection, so future equality changes still have candidate dependencies to notify.

Discovery drains before source execution resumes. Thus the occurrence lists cannot change while a discovery's filtered lists are being constructed. Later posts anchor their own discovery. The filter does not use transient guard failure, current variable bindings or overlapping choice contexts to discard tuples.

This is per-head filtering, not general selective joining. Distinctness is still checked on complete tuples. Incompatible supports across otherwise individually live heads, repeated-variable relationships, guards and propagation-history exclusion still require later work. Snapshot-list construction and retention are new costs, not free preprocessing.

## Measured work, including contrary cases

The following are diagnostic build counts. They are not elapsed-time or allocation measurements. Each source is checked against independently constructed full answers and the independent scalar semantics.

| Source | Existing tuples | Filtered tuples | Existing ticks | Filtered ticks |
|---|---:|---:|---:|---:|
| Common traversal, depth 1, no choices | 6 | 4 | 436 | 433 |
| Common traversal, depth 4, no choices | 27 | 13 | 853 | 778 |
| Common traversal, depth 16, no choices | 291 | 139 | 4141 | 3058 |
| Common traversal, 3 choices, depth 16 | 291 | 139 | 5384 | 4301 |
| Independent branch suffixes, 3 choices, depth 16 | 2962 | 1629 | 44700 | 34960 |
| Early failure, 3 choices, depth 16 | 291 | 259 | 5808 | 5845 |
| Dense two-head propagation over 6 occurrences | 30 | 30 | 8460 | 8556 |

The no-choice count has an independent explanation. Filtering keeps the entry and permit-history candidates, one terminal candidate, and the remaining fuel alternatives for each step: `3 + d(d+1)/2`. The test asserts that count rather than merely requiring a decrease. The dense case independently requires all 30 ordered pairs of distinct occurrences and all 30 residual observations.

Early failure is already contrary evidence to treating fewer tuples as an efficiency result: 32 fewer tuples coexist with 37 more ticks. Likewise, the dense control offers nothing to filter and incurs 96 additional ticks. A tick's internal work differs between the algorithms, so even a tick reduction cannot substitute for ordinary timing.

[Existing diagnostic receipt](s10-selective-discovery-gate/baseline-work.log), [filtered diagnostic receipt](s10-selective-discovery-gate/selective-work.log), [source/work tests](../../../research/chr-direct-conditional/tests/selective_discovery.rs).

## Correctness and implementation validation

The complete package suite passes in the existing mode, filtered diagnostic mode and filtered counter-free mode: 120 tests in 21 suites each. It includes partial liveness, history, consuming resources, equality-enabled matches, shared and independent choices, full source composition, and finite-answer progress beside ongoing work. [Existing](s10-selective-discovery-gate/baseline-tests.log), [filtered](s10-selective-discovery-gate/selective-tests.log), [counter-free](s10-selective-discovery-gate/counter-free-tests.log).

The countdown maintenance test now asserts the correct exact count for each mode: two discovered constructor candidates per occurrence for the existing enumeration and one for filtered discovery. Its full-answer and linear maintenance bounds remain checked. The initial selective RED test failed with six candidates where the depth-one mechanism requires four; the implemented filter satisfies that assertion.

Strict counter-free Clippy passes for the [package library/tests](s10-selective-discovery-gate/clippy.log) and [final work tests](s10-selective-discovery-gate/clippy-work.log). The implementation is confined to conditional discovery and its feature selection. No reference-interpreter code changes. This finite test evidence and the stated monotonicity argument are not a general proof of the whole runtime.

## Next comparison and priority

Register a paired lifecycle pilot for existing and filtered conditional discovery, using Scan and source-derived counting as stronger controls where applicable. Include recursive common work, branch-specific work, early failure, dense heads, shallow inputs, changed queries and cancellation. Measure allocation separately from ordinary counter-free time. Charge filtered-list construction and disposal, candidate/history retention, preparation and complete answer delivery; verify ownership restoration and all observations.

The strongest ready alternative is resumable contextual discovery. It could avoid the repeated history probing exposed by the restarting-demand implementation. A bounded cost pilot for the implemented filter comes first because the current work counts point in both directions and cannot establish whether the representation becomes a more credible complete competitor. Reconsider contextual discovery after that pilot; another filter refinement must identify a consequential measured cost rather than follow tuple counts alone.

Support-aware joining, broader access plans, conditional representation costs, language properties and sustained lifetime remain required. This gate supports a cost investigation, not architectural selection or goal completion.
