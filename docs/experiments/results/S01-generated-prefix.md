# Generated code reduces bookkeeping; activation still repeats deep matches

Actual generated Rust preserves the structural matching work of the generic executor on these sources. Active indexed access reduces that work substantially, but leaves a meaningful deep-prefix contrast to investigate. It also changes the answer in the early-consumption case, so it cannot serve as a same-answer control there.

**Next, qualify and measure complete lifecycles for retained matching against these stronger controls.** No runtime or memory advantage follows from this work screen. The research remains active.

## What was compared

The [registration](../registrations/S01-generated-prefix.md) fixes the existing structural-prefix sources: widths four/eight, depths zero/eight/32, two changing queries per preparation, and sparse, keyed, early-consumption and mismatch families. The same source constructor now feeds the earlier intermediate test and this generated test. Six rule programs are compiled using the existing general emitters; no pattern-specific runtime algorithm is added.

Three execution forms run each source: generic, generated rule callbacks, and generated access continuations. Each uses Global/Active policy and Scan/Indexed access. Two diagnostic and two counters-off repetitions complete 2,304 executions, each preceded by cancellation and preparation reuse. Answers are checked again after producer/preparation disposal. Of these, 2,016 agree with independent scalar source execution; 288 are the independently checked Active-policy counterexample described below.

The [audit](s01-generated-prefix/audit.json) verifies every configuration, exact repeated work rows, frozen source/binary hashes and zero counters in the plain build. These are finite source/work and ownership checks. They measure neither time nor heap allocation, compilation economics, nor sustained memory behavior.

## Activation saves searches, but deep matching remains

For width eight, the following structural-test counts apply identically to all three execution forms. Columns vary constructor depth. They count operations, not equal-cost time units.

| Source and access | Depth 0 | Depth 8 | Depth 32 |
|---|---:|---:|---:|
| Sparse, Global indexed | 672 | 5,856 | 21,408 |
| Sparse, Active indexed | 276 | 2,124 | 7,668 |
| Keyed, Global indexed | 392 | 3,336 | 12,168 |
| Keyed, Active indexed | 276 | 2,292 | 8,340 |
| Mismatch, Global indexed | 8 | 584 | 2,312 |
| Mismatch, Active indexed | 40 | 744 | 2,856 |

**The stronger access control narrows the retained-matching opportunity.** Sparse Active indexed adds 231 structural tests per added constructor, compared with 648 under Global. The [existing proper-intermediate screen](S01-structural-prefix.md) adds 72 local pattern visits per constructor on this source. The definitions differ across engines: this supports investigating retained traversal, not a 231/72 speed ratio. At depth 32, local proper intermediates perform 4,920 pattern visits, versus 7,668 structural tests for Active indexed.

**Activation also has adverse cases.** It increases matching in the deep-mismatch family, where no join fires. At depth zero that family omits middle facts; treat it as an absence control, not a matched zero-depth structural cost. Early consumption wastes intermediate preparation under Global, as the prior study established. Both adverse cases remain necessary in lifecycle measurement.

## Native generation removes different work

**Generated callbacks remove syntax traversal; generated access additionally removes frame copying and candidate-pool construction.** In the sparse width-eight/depth-32 Active indexed case, generic execution records 7,692 syntax visits, 762 binding-slot copies, 231 candidate-pool entries and 1,175 key-template visits. Generated callbacks eliminate the syntax visits. Generated access records zero in all four categories, while retaining the same 7,668 structural tests and 246 candidate visits.

These differences qualify generated access as an essential cost competitor. An intermediate implementation might save structural walks and still lose to cheaper generated execution once preparation, terminal matching, invalidation and disposal count. Build-time generation here demonstrates executable native code; it does not isolate user-program compilation or justify compilation amortization claims.

## Active policy changes consuming-rule competition

**Global priority kills every left occurrence before any join; FIFO activation allows earlier rights to fire first.** The source inserts left, middle and right occurrences in groups, followed by the kill fact. Global selection repeatedly chooses the higher-priority kill rule. Active selection instead restricts each search to the next queued occurrence: a left activation kills that left, while a following right can still use the last left/middle pair until the last left is activated and consumed.

At width four, Active produces three hit outputs; Global produces zero. At width eight, Active produces seven hits; Global again produces zero. The independent endpoint construction requires all middle facts, the final unconsumed right, the kill fact and the final unbound output. An independently specified exact occurrence trace requires alternating kill/join applications followed by the final kill. Generic, generated callbacks and generated access all satisfy both checks, at all depths and query seeds.

The first qualification failure is preserved in [qualification-3.log](s01-generated-prefix/qualification-3.log). The registration records the finding before the frozen screen. The source is unchanged. This is a scheduling-contract distinction, not evidence that native generation corrupts execution. Future same-answer comparisons exclude this Active/Global pair explicitly; the counterexample remains language-design evidence about observable rule competition.

## Why lifecycle measurement is next

**Completing the retained-matching cost contrast now has a credible path to changing a decision.** The missing native and activation controls execute, and activation does not erase repeated deep traversal on qualifying sources. Use generic and generated access, retain Global's adverse consumption case, and scope Active comparisons to sources with independently equal answers. Reuse the existing lifecycle harness and independent endpoints; include cheap prefixes, deep useful prefixes, keyed access, mismatches, preparation reuse and releasing/retaining consumers. Register exact costs only after the measurement adapter passes its gates.

**Compact runtime confirmation is the strongest ready alternative.** The single-traversal comparison already settles its allocation disadvantage on the tested families, while a sampled runtime advantage remains consequential. Completing the distinct retained-matching contrast first can establish whether another architecture needs maintained state at all; further compact precision does not answer that. Compact confirmation remains required when comparing complete paths or if the lifecycle adapter encounters an obstruction.

Demand discovery/dependency costs and direct integration costs also remain unfinished. At the lifecycle qualification result or obstruction, compare those studies and compact confirmation again. This is package two since the last full portfolio review; no more than two further packages may precede another full review. No design is rejected by this scheduling choice.

## Validation and limits

The frozen matrix passes four runs with 576 complete answers and cancellation/reuse pairs per run. Strict Clippy passes for the new diagnostic test. The existing structural-prefix test passes after source consolidation, and its historical evidence audit still passes against its frozen archive. All 22 existing access, generated-access and semantic regression tests pass. Initial compile-time test failures and the policy counterexample remain in the receipts.

Source scope is deterministic kept-prefix/consumed-terminal rules over ground nested constructors. This does not settle broader partner ordering, subscriptions, delayed bindings, dynamic rules or continuing lifetimes. Complete architectural comparisons, language alternatives and held-out challenges remain required.
