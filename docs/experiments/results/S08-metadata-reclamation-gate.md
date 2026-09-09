# Selective metadata reclamation preserves the tested graph executions

The graph can now release cached-result entries and consumption claims that no pending task can select. Paired tests preserve every service boundary and complete answer, including a claim needed by an unfinished answer. Allocation and timing benefits remain unmeasured.

**Next size complete costs with retaining and periodically reclaiming controls.** The [prospective registration](../registrations/S08-reclamation-lifecycle-sizing.md) charges scanning, vector resizing, continued execution, observation and disposal. The operation's existence does not establish that frequent reclamation is economical or that remaining graph retention is necessary.

## The operation and its boundary

**Reclamation filters only permanently incompatible metadata.** `Run::reclaim_incompatible_supports` retains a result or consumption claim when its condition agrees with at least one pending task on shared choice labels. It preserves the order of retained entries and shrinks an entry vector only when entries were removed. Task contexts, graph nodes, choice labels, resource occurrences and obligation positions are unchanged.

**The argument relies on monotone task contexts.** A conflict with every pending context remains a conflict in every descendant. Result selection and resource liveness read an entry only when all its assignments hold in the current context. Thus an incompatible entry cannot affect later selection. The [support inventory](S08-future-support-inventory.md) establishes this premise and checks all 729 partial-context pairs against exhaustive extensions.

**The operation is called between service steps, not during a match or resource claim.** There is no new default collection policy or hot-path counter. A caller chooses the maintenance boundary. Full scans of retained nodes, resource claims and pending contexts, plus vector shrinking, are real costs that the next experiment must include.

## What the gate proves

**The stream comparison covers 3,456 candidate/control pairs per build.** There are 288 source configurations: repeated/distinct/alias values, resource and failing-terminal flags, depth 0/4/16, work 0/3, payload 0/4 and both query orders. Each uses three result-validity policies, templates on/off, and reclamation after every service step or after each answer. Every candidate remains in lockstep with its retaining graph control: progress, answer and exhaustion events agree, and each complete raw answer agrees independently with scalar source semantics.

**Identity and useful-history checks go beyond final answer counts.** At every maintenance call, node/call/choice counts and birth/obligation slots remain unchanged. The real-stream witness checks exact removal counts, preservation of compatible metadata and idempotence. At answer 64 of a template stream, one still-needed consumption claim survives while 64 incompatible claims are released; the final answer and residual observation remain correct.

**Additional tests cover competing resources and lifted argument choices.** Seventy-two paired cases vary zero/one/two tokens, query order, validity policy, templates and maintenance cadence. Twenty-four paired cases exercise direct argument lifting with consuming constructor matches. Reclamation preserves the graph schedule at every service boundary. The uncontested cases and lifted-choice cases also agree with the independent scalar control.

**The full gate passes in default and counter-free builds.** All seven stream tests and all 22 graph-package tests pass; scoped strict Clippy and formatting pass. [Validation receipts](s08-metadata-reclamation-gate/audit.json) identify the checked source and commands. These are correctness and source-contract results, not measured memory savings or speed results.

## The contested case exposes a language-policy boundary

**With one token, the graph and fixed scalar policy can choose different consumers.** The source offers `choose(X)` with alternatives a/b, `take(X,First)`, and `take(b,Second)`. Each `take` consumes the sole token and returns `taken(...)`; the loser remains as a residual occurrence. The graph gives the token to Second in both query orders. The scalar policy gives it to First in forward order and Second in reversed order. Tests assert the complete outputs and residuals of each policy, rather than treating either winner as an equivalent answer.

**The difference already occurs in the retaining graph control.** Graph service advances its obligation cursor before an interrupted consumer splits on a choice. Child tasks inherit the advanced cursor, allowing the next consumer to claim the token before the interrupted consumer is revisited. Reclamation leaves every event of this execution unchanged. An initial comparison against the fixed scalar policy exposed the difference; the current tests preserve it as an explicit counterexample.

**Both outcomes have an ordinary committed source derivation, but they do not implement the same fixed policy.** After resolving the choice, either enabled `take` occurrence can consume the token under an unconstrained committed selection. Its loser remains unmatched, and the token is consumed exactly once. This is the [existing distinction between permitted committed execution and fixed source order](S00-contracts-and-candidates.md), also exposed by the [derivation source gate](S03-fresh-derivation-source.md). No language policy is selected here. The contested case is excluded from equivalent-source cost claims; its source-policy compatibility remains a required S07/whole-architecture question.

## What remains

**Selective metadata removal does not reclaim the graph itself.** Result targets may have other live references; obligation indices and task cursors may still be meaningful; choice birth information participates in validity. Larger changes require separate reachability and identity arguments. The current operation cannot justify clearing those owners indiscriminately.

**The next cost package is the fourth package in this lifetime cycle.** Source/ownership measurement, future-support inventory and this reclamation gate are the first three. At the sizing boundary, review breadth against broader call-level reuse and distinct integrated S02 execution before extending local reclamation work. T074 and the architecture goal remain active.
