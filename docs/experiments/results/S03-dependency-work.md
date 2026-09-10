# Repeated unsuccessful dependency discovery dominates this work screen

The delayed-key miss case repeatedly searches the same unresolved dependencies. At size 128 it performs 6,432,964 force entries and 3,253,118 match entries, with only 132 completed-equation validation visits. This identifies a consequential implementation question before fair lifecycle comparison; it does not establish a runtime ranking.

## What was compared

The [registration](../registrations/S03-dependency-work.md) fixes 30 source configurations: sizes 8/32/128, two constraint orders, and five families. Those families are plain independent identity calls, known-key success/failure-to-match, and delayed-key success/failure-to-match. Each of the three demand reuse policies reuses preparation across two changed atom-valued queries: 90 demand preparations and 180 sessions per execution.

Both diagnostic executions complete and produce identical rows. Complete raw answers agree with independent owned-syntax scalar semantics and compiled Scan/Indexed controls. The source has no competing consumers or implicit choice; a miss leaves its unresolved calls and resources in the answer. “Miss” means unsuccessful nonbinding matching, not whole-query failure.

The counters record force entries, match entries, validation passes and validation node visits separately. These are different operations; they must not be added into a synthetic cost or equated with compiled service ticks. This screen measures neither time nor allocation.

## The discriminating results

The table uses CurrentContext and the first changed query. All three reuse policies and both changed query values have identical work counts within each source/order configuration.

| Size-128 source | Force entries | Match entries | Validation visits |
|---|---:|---:|---:|
| Plain independent calls | 647 | 129 | 389 |
| Known keys, successful matches | 1,159 | 513 | 389 |
| Known keys, misses, original order | 100,619 | 50,567 | 261 |
| Delayed keys, success, original order | 660,327 | 339,169 | 517 |
| Delayed keys, success, reversed order | 1,287 | 513 | 389 |
| Delayed keys, misses, original order | 6,432,964 | 3,253,118 | 132 |
| Delayed keys, misses, reversed order | 6,358,665 | 3,227,972 | 132 |

**Order changes the successful dependency workload dramatically.** Reversing the query presents the terminal producer first, making useful keys available before earlier requests need them. It scarcely improves the unsuccessful chain: the same unresolved work remains discoverable repeatedly. This is a controlled source-order result, not a recommendation to reorder arbitrary competing CHR rules.

**The unsuccessful chain grows rapidly with size.** Original-order force entries are 2,224 at size 8, 107,572 at size 32 and 6,432,964 at size 128. Validation visits are 12, 36 and 132 respectively. Three sizes do not prove an asymptotic bound, and small visit counts do not prove cheap validation in elapsed time. They do identify repeated forcing/matching as work that a credible implementation must explain.

The current evaluator retains completed call results but does not retain an unsuccessful match result. During resource matching, revisiting an unresolved call therefore re-enters discovery; residual observation can also force those calls again. This source inspection is consistent with the counts. A causal repair comparison is still needed to isolate how much work can be avoided soundly.

## The cutoff and its resolution

The initial 200,000-service-turn limit stopped compiled Scan on the size-128 original-order successful delayed chain. Both attempted runs preserve their partial rows and the same cutoff. Before rerunning, the registration raised the finite service bound to 2,000,000, retaining the 60-second wall/CPU and 1-GiB process limits.

With the larger bound, Scan completes that case in 724,296 ticks; Indexed completes in 25,288. Reversed order takes 904 ticks in either compiled control. Every registered source then completes in both repetitions. These results resolve this cutoff as insufficient sizing, not nontermination or evidence that the architecture loses. Compiled ticks and demand force entries are not comparable cost units.

## Correctness and measurement limits

Five resource-dependency tests and all 31 suspended-source tests pass in both the diagnostic-enabled and metrics-off builds. Scoped Clippy passes with warnings denied. Counter fields and their increments are conditional on `work-diagnostics`; the independent oracle and reference interpreter are unchanged. The new counters distinguish the completed-equation check from the existing force/match counts without changing the validation algorithm.

The largest recorded validation count is 517 visits. This does not settle validation cost for large constructor graphs, long completed alias chains, shared subgraphs or sustained contexts: the plain control here contains independent calls. Allocation, retention, first/full observation, cancellation and preparation economics remain required before any efficiency conclusion.

## Next experiment and priority

Register and test the validity of retaining an unsuccessful probe until a relevant state change. This is a candidate repair, not an adopted cache policy. Producer completion, changed bindings, resource posts/consumption, selected choices and recursive re-entry can invalidate a miss; a sound design must not hide an enabled rule, confuse unfinished recursion with a fixed result, or change committed policy. Retain the current no-miss-reuse path as the causal control and compare the same completed outcomes and work rows.

This repair investigation precedes timing because millions of repeat probes could make the new capability an unrepresentative competitor. It is more decision-relevant than refining the precision of the small validation counts. Contextual integration remains the strongest scheduled distinct alternative and follows this bounded validity/attribution gate unless the full breadth review selects another direction. Joint symbolic runtime/union remains required.

This completes package two after the joint-ownership breadth review. Count the repair as package three, not an uncounted extension; full breadth review remains due by package four. Lifetime and comparative timing are not complete, and neither T071 nor the architectural research goal is complete.

## Receipts

- [Source/configuration harness](../../../research/chr-direct-conditional/tests/dependency_work.rs) and [freeze/run driver](../../../research/chr-direct-conditional/experiments/dependency_work_confirmation.py).
- [Qualified freeze](s03-dependency-work/qualified/freeze.json), [six process outcomes](s03-dependency-work/qualified/results.json), and [180 diagnostic rows](s03-dependency-work/qualified/rows.csv).
- [Complete diagnostic/control log](s03-dependency-work/qualified/on-dependency_work-0.log) and its second execution in the same directory.
- [Initial cutoff receipts](s03-dependency-work/confirmation/results.json), with exact earlier-source recovery beside the freeze.
- [Scoped Clippy](s03-dependency-work/clippy-qualified.log).

Separate target directories preserve the initial and qualified binaries. A repeated driver run must select a fresh receipt destination; it refuses to overwrite the recorded directory.
