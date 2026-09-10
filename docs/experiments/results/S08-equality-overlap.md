# Less equality work can increase the observation backlog

Skipping redundant support differences reduces work in the continuing conditional emitter. With reversed support order, however, the faster producer accumulates observations and delivers fewer answers within the bound. Limiting producer lead restores progress on this source; neither change establishes sustainable memory or a timing advantage.

## What the comparison establishes

The source repeatedly chooses between returning a fresh aliased pair and continuing to emit. Every delivered answer is independently checked as `pair(X,X)` with no residual constraints. The source is continuing: 128 answers are a measured prefix, not exhaustion.

| Configuration | Calls to deliver 128 answers | Body calls | Observation calls | Support nodes at that prefix |
|---|---:|---:|---:|---:|
| Forward order, existing improvements | 3,198,239 | 2,242,262 | 818,043 | 25,160 |
| Forward order, overlap shortcut | 2,116,705 | 1,160,725 | 818,043 | 25,160 |
| Forward order, shortcut and bounded lead | 2,116,705 | 1,160,725 | 818,043 | 25,160 |
| Reverse order, existing improvements | 4,606,799 | 2,276,305 | 2,240,953 | 518 |
| Reverse order, overlap shortcut | **Unfinished: 116 answers at 6,000,000 calls** | — | — | — |
| Reverse order, shortcut and bounded lead | 3,488,701 | 1,145,427 | 2,252,809 | 519 |

Both repeats reproduce each configuration exactly. Forward order saves approximately 34% of service calls and 48% of body calls with the shortcut. Reverse order with bounded lead saves approximately 24% of calls against its original control. Reverse order still has more observation work and far fewer support nodes than forward order. Node counts are not byte measurements.

**The shortcut avoids a known redundant operation.** Before it, all 8,328 completed binding overlaps in the forward 128-answer prefix are empty; support jobs account for 2,179,097 walker ticks in body service. Once intersection establishes that two regions are disjoint, subtracting the binding region cannot change the active region. Similarly, full containment makes the remainder empty. The experimental feature handles those exact cases directly; partial overlap keeps the original split path. The isolated disjoint-binding writer takes 17 ticks instead of 21 while preserving independently checked context answers.

**The reverse regression is an interaction with scheduling.** At 64 answers the original reverse control has one pending observation. The shortcut has 30, along with 96 equality variables versus 67 in the control. The read-only backlog probe reproduces every earlier service count, so the added diagnostic does not explain the regression. Faster production has advanced further while observations compete for service.

**Bounded lead tests that explanation directly.** When at least two observations are pending, the experimental policy services observations until fewer than two remain. Otherwise it retains alternating producer/observer service. Reverse order now has two pending observations and 67 variables at 64 answers, reaches 128 within the registered bound, and uses 519 support nodes and 131 variables there. Forward prefix counts are exactly unchanged. This supports backlog control on this source; it does not prove arbitrary fairness or choose a default policy.

## Controls and correctness

The first attribution build omitted selective discovery and differed from the historical forward receipt by 130 discovery calls at 128 answers. The registered configuration control restored selective discovery and reproduced all historical prefix stage counts exactly. The original generic-discovery receipts remain part of the evidence. Comparisons in the table use matching selective-discovery configurations.

All measured configurations disable default features and enable `equality-probe`, `alloc-meter`, `serial-body-accounting`, `equality-invalidation`, `support-identities` and `support-result-cache`. Matched controls additionally enable `selective-discovery`; reverse variants add `support-reverse-order`; the two experimental switches are `equality-overlap-shortcut` and `observation-backpressure`. These are diagnostic builds. No ordinary-allocator timing comparison is claimed.

The service matrix contains 18 processes: 14 reach 128 answers and four reproduce the expected 116-answer cutoff, including the read-only backlog repeats. Each process has a six-million-call limit, 60-second wall/CPU limits and a 1 GiB address-space limit. Counts are recorded at answers 1, 8, 32, 64 and 128 when reached. A delivered-prefix boundary can contain different amounts of pending work across policies; binding-probe totals therefore need not agree.

The independent equality suite checks finite-tree substitution, nonbinding demand, late alias notifications, stale writers, interrupted commitments and context-local occurs/clash failure. Runtime tests cover consuming effects, dynamic births, prepared-query independence, finite siblings beside ongoing work and adversarial publication/dependency cases. Maintenance tests include partial liveness across choice regions. These existing tests exercise more semantics than the continuing emitter alone.

Selected regression executables pass in three configurations: 33 tests with the shortcut and default features off, 33 with default features, and 39 with the full reverse-order bounded-lead configuration. The latter includes a 64-answer test checking every service step's backlog stays at most two and every delivered answer has the independent expected meaning. Its recorded failing run before policy implementation and passing run afterwards establish that it detects the intended change. Both experimental Clippy checks and formatting checks pass. The reference interpreter and independent finite-tree oracle are unchanged.

## Architectural consequence and remaining work

**The current cost is partly avoidable; the underlying retention question remains open.** Forward support population is unchanged, and both representations retain growing variable histories. Work saved inside equality can move pressure into observation, so independently optimizing the components is insufficient. This comparison justifies carrying both the exact shortcut and bounded scheduling as experimental candidates into a future continuing lifecycle comparison.

Required follow-ups include larger completed prefixes, stable-size changing queries, cancellation/reuse, engine versus consumer ownership, reclamation versus regeneration, ordinary-allocator time and sampled RSS. Broader publication workloads must challenge the lead bound, including expensive observations and continuing siblings. Compilation is outside this comparison. No conditional architecture is selected or rejected.

Four packages—overlap attribution, shortcut qualification, backlog attribution and bounded-lead qualification—now reach their boundary. The [breadth review](S08-equality-overlap-breadth-review.md) selects the separate names-theory entry next, in accordance with the [execution sequence](../next-cycle.md).

## Evidence

Prospective registrations: [overlap attribution and shortcut](../registrations/S08-equality-overlap.md), [matched discovery control](../registrations/S08-equality-overlap-discovery-control.md), [backlog attribution](../registrations/S08-equality-backlog.md), and [bounded lead](../registrations/S08-bounded-producer-lead.md).

[Raw receipts and validation](s08-equality-overlap/), [audit summary](s08-equality-overlap/audit.json), and [resolved frozen inputs](s08-equality-overlap/resolved-inputs.json) preserve all nine configuration manifests and their 131 inputs, including earlier source snapshots. The executable [audit](../../../research/chr-direct-conditional/experiments/audit_equality_overlap.py) verifies hashes, repeat equality, cutoff classification, matched controls, prefix counts and test receipts. Those checks establish provenance and consistency; the semantic tests and answer checks supply correctness evidence.
