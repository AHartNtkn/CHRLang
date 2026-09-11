# Projected values need a plan for entering the caller

Binding projected values before the caller runs changes which rule consumes a
token in 120 of 216 tested cases. Keeping the binding point restores complete
answer bags, but sorted alternatives still change delivery order in 24 cases.
An experimental transport that retains source choices and binding steps matches
all 4,560 reference checkpoints.

**Keep projection as a candidate component. Its weighted tuples are insufficient
to specify ordinary consuming execution.** The experiment supplies the missing
choice and binding plan and checks it. Retaining that plan also retains execution
work; these results establish correctness, not a speed improvement.

## What was compared

The caller chooses a or b, records a propagation-history observation and has one
token per alternative. A name-specific consumer and a competing consumer can
claim it. The experiment varies their priority, duplicate choices, alternative
order, output aliases, variable identifiers and a delayed binding through zero,
one or four recursive steps. The b branch binds immediately.

| Delivery method | Complete raw answer bags | Source delivery |
|---|---|---|
| Bind projected values before starting each branch-local caller | Differs in 120/216 cases | Already fails complete answers |
| Expand weighted tuples inside the original choice rule, in sorted order | Matches 216/216 | Ordered unique answers differ in 24/216 |
| Expand using the original choice order, retaining binding steps and recursive waits | Matches 216/216 | Matches 4,560 one-step answer/exhaustion checkpoints |

The projection supplies values and multiplicities. The repaired experimental
transport consumes those counts in the source alternative order and retains the
same nested choices and delayed binding operations. It checks that every count
is consumed exactly. This uses additional source information; it does not recover
execution order from a weighted relation.

## Why the differences matter

**A value arriving early can change an answer, not just its order.** With the
name-specific rule before the competing consumer, an already-bound a wins the
token. If the source has not yet bound that variable, the competitor can win.
Recursive waits create more opportunities for this difference: early binding
differs in 24/72 immediate cases and 48/72 cases at each nonzero delay.

**Equal bags do not guarantee equal cancellation results.** For immediate
branches, expanding [b,a] as sorted [a,b] can change the first delivered answer.
Duplicate alternatives also belong to the execution plan. The delayed cases in
this matrix have no sorted unique-order differences; their branch lengths govern
which distinct result arrives first. That does not establish general order
independence.

**A correct transport exists for these callers, but it has not accelerated them.**
The repaired experiment retains the source choice and binding operations. It
therefore cannot carry over the earlier logical-query timing gains as caller
speedups. No comparative timing was run on this fixture.

## Checks and the next experiment

The independent scalar interpreter checks complete raw answer multiplicities,
including residual history and token-consumer results. The reference interpreter
checks ordered unique answers and every one-step delivery/exhaustion checkpoint.
Each case also cancels at 0/1/5 steps, restarts, and checks held answers after the
producer and prepared relation are disposed. All 20 caller, projection and connected-source tests pass in both default and
counter-free configurations. Counter-free Clippy passes for the new experiment.

The initial priority arrangement produced no early-binding difference because its
name-specific consumer followed launch. Moving that consumer before launch made
the relevant competition observable. The registration records this fixture
correction and the subsequent delayed-branch extension.

Next, apply projection to connected hidden-variable searches within the caller.
Compare keeping every choice step with contracting hidden work while preserving
the observed binding and delivery events. Implement and test the scheduling work
needed by contraction, then measure the complete path against enumeration. This
can determine whether projection's measured repeated-query benefit survives the
caller; it has higher immediate decision value than further local allocation
changes. Broader call observers and selective retention remain ready alternatives.
T076 stays active; this is package one since the portfolio review. The research
goal remains active.

[Registration](../registrations/S06-projected-caller.md) ·
[Executable experiment](../../../research/chr-structural/tests/projected_caller.rs) ·
[Previous cost results](S06-lazy-weighted.md)
