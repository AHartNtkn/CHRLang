# Continue sole replay branches without repeated reconstruction

Test a specific avoidable replay cost before expanding restoration-policy measurements. Current replay reconstructs its saved prefix at every public service step, including deterministic work with a sole remaining branch. The candidate owns that sole branch as live state; when it forks, it saves a shared checkpoint and queues child recipes in the existing FIFO order. Competing saved recipes still obey the selected checkpoint interval.

The invariant is that a live recipe is the only pending recipe. Promotion after progress discards unnecessary checkpoint/choice history. A fork saves the complete state immediately before its pending choice, including bindings, live occurrence identities, history, fresh counters and remaining work. No source step or branch may be skipped, batched or reprioritized.

Require: a failing deterministic-work diagnostic; exact public event/answer agreement with the copying control on branching, failure and late-consumption sources; independent complete-answer tests already covering aliases, history, pending tails and fresh values; a finite sibling beside ongoing work; and weak-owner checks after the frontier becomes sole. Preserve independent reference/scalar implementations.

Use 100 public service calls on a continuing deterministic source for the first diagnostic and require exactly 100 actual source steps after the change. Before changing the source, the current replay formula predicts 1+...+100=5050 steps. This is a work-count attribution, not a timing measurement. Use the separate replay-diagnostic build; ordinary execution retains no diagnostic increments.

Run package tests with and without diagnostics, strict scoped Clippy, and the existing lifecycle work diagnostic after updating its independent copied-state work projection to the new ownership rule. Bound finite source tests at 100000 service calls and ongoing-sibling tests at their explicit finite observation windows. Allocation/timing and temporary reunion remain required separate work.

At this gate boundary compare broader checkpoint/replay cost measurement and actual temporary reconnection with integrated dependency repair. No broad restoration decision follows from eliminating this isolated repeated-work cost.
