# Body reuse preserves interleaved caller priorities

Rule-body reuse matches 90,792 one-step checkpoints across 864 sources with
observers before, between and after private rules. Repeated complete runs execute
no additional body-trace steps. Matching, consuming heads and propagation history
remain in the live caller.

**Reuse no longer needs a whole higher-priority private phase in this experiment.**
It follows the smaller boundary where a selected rule's pending body runs before
the next rule selection. Whether that finer granularity is efficient remains the
next question; no comparative timing was run here.

## What executes where

| Responsibility | Execution owner |
|---|---|
| Rule order, matching, guards and head consumption | Original live cursor |
| Propagation history and allocation of rule-local variables | Original live cursor |
| Selected pending body: equations, conjunctions, choices and posted facts | Reusable body trace |
| Binding and posted-fact restoration before the next rule selection | Body-to-caller transfer |
| Raw alternative scheduling, cancellation and answer delivery | Caller FIFO frontier |

The body key contains resolved terms and their variable-alias structure. It does
not contain unrelated caller facts: the caller has already selected the rule,
and no further matching occurs until its pending body finishes. The unchanged
source interpreter supplies that boundary; the experiment does not impose a new
rule-priority restriction.

An isolated entry rule lets the existing trace engine execute the body without
executing rules for its posted predicates. Entry insertion/application are
processed when attaching the job; subsequent trace progress corresponds to the
pending source body. Completion restores results and selects the next live rule
in the corresponding source step. Entry processing, key construction and transfer
are actual CPU/allocation costs even though they add no scheduler tick.

The entry predicate is chosen outside the body's posted names. Source programs
can use names such as $body and $body_; these do not become reserved language
names. The shared syntax type gains structural ordering for body keys, while the
reference interpreter implementation remains independent and unchanged.

## Evidence and challenges

The matrix varies recursion depth 0/2/4, four ordinary/failing/fresh-alias/continued-
work families, reversed alternatives, three observer positions, propagating or
consuming observers, token counts 0/1/2 and two variable namespaces. Every case is
cancelled at 0/1/5 steps and restarted with the same prepared caller. Direct checks
every ordered answer/exhaustion event; an independent scalar interpreter checks
each complete raw bag after both query handles are dropped.

The metrics build requires warm complete runs to execute zero additional body
steps, not merely report a cache hit. Higher-priority consuming observers must
actually intercept private work and produce the observed output. Focused sources
also check:

- Fresh variables shared between output constructors and multiple posted facts.
- Existing same-predicate caller occurrences and two consuming tokens.
- Source predicates matching candidate internal-entry names.
- Guards that accept a, reject b and accept a again with cached bodies present.
- Earlier body bindings that make a later equation cyclic, versus finite success.
- Kept heads, consumed heads, duplicate results and foreign-run ownership.

All five new tests pass in default and counter-free configurations. The 42 relevant
counter-free tests and 22 persistent-engine tests pass; scoped Clippy passes. The initial 288-case entry matched 29,232 checkpoints; token variation
expanded it to the final 864 cases and 90,792 checkpoints.

## What the next comparison must charge

A body-specific trace is prepared on its first use. Every selected application
exports and canonicalizes its body, performs lookup and later restores bindings
and posts. This may save too little work for short bodies, even with perfect reuse.
The implementation also retains body keys and traces across queries.

Next, register complete costs against Direct and credible matched execution/reuse
controls, using both short bodies and substantive repeated body work. Include
cold/warm preparation, changed query inputs, high-priority observers, failed
alternatives, cancellation, held answers and full disposal. Inspect existing
compiled and full-state reuse controls before selecting the exact matrix. Any
speed advantage must survive the strongest applicable simpler control.

This is more consequential now than another trace-retention policy: it determines
whether the newly validated granularity can earn its lookup and transfer costs.
Selective retention, broader source planning and whole-query reuse remain required.
T075 stays active, package count two since portfolio review; the goal is active.

[Registration](../registrations/S05-body-reuse.md) ·
[Executable experiment](../../../research/chr-reuse/tests/body_reuse.rs) ·
[Phase-boundary observers](S05-live-observers.md)
