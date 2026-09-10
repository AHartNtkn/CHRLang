# Preserving effects also requires preserving how callers resume

A non-propagating priority phase can preserve complete answers while changing the first answer delivered. Existing resumable transition reuse preserves the tested delivery schedule and effects, but misses reuse when callers differ only in inert tags. The next implementation should test separating that inert data without making active execution atomic.

This is source and scheduling evidence. No timing, memory advantage or complete architecture choice follows.

## What the checked phase still gets wrong

**The tested syntactic conditions do not establish ordered caller equivalence.** The selected rules form an exact initial priority prefix, each consumes at least one head, and the initial projected region includes all selected head names. In 64 sources, two branch lengths vary independently over zero, one, four and eight recursive steps, with reversed branch order and two variable namespaces. Both uncached and region-keyed contraction preserve the complete independent raw-answer multiset in every source.

Delivery order nevertheless changes in 24 of those 64 sources, or 48 of the 128 candidate comparisons. Caching is not the cause: uncached contraction changes the same order. Completing one resumed caller before servicing the other makes phase completion order decide output order.

**The consequence is visible after just one answer.** One branch needs eight further consuming steps before producing `a`; the other can produce `b` immediately after its shared prefix. Direct FIFO execution delivers `b`, then `a`. Contracted execution delivers `a`, then `b`. The independent scalar evaluator also delivers `b` first in this unequal-length witness. Cancellation after one answer therefore selects different outputs, even though full multisets agree.

This refutes the tested complete-phase batching transformation under the direct FIFO delivery contract. It does not prove that phase reuse must change order, nor that all source consumers require this contract. A different observation contract needs an explicit language comparison. A scheduling-preserving implementation remains a required alternative.

## What resumable reuse establishes

**Existing whole-state transition tables preserve the source schedule on these challenges.** AlphaLive and CompactLive each match Direct one logical service step at a time on 24 sources. The matrix includes competing consumption, intermediate observation, earlier bindings, surviving propagation history, resource absence and early failure, with same/different caller tags and two namespaces.

Across the two table representations, 968 step comparisons agree on every delivered answer and exhaustion flag. Complete raw-answer multisets also agree with the unchanged independent scalar evaluator. Both representations additionally preserve Direct's ordered answers on all 64 priority-phase sources. These are existing controls; the experiment does not present them as new call-local implementations.

**Reuse survives effects when complete relevant states reconverge.** In the metrics build, the two representations together obtain 192 transition hits on same-caller sources. Representative counts below are for one AlphaLive run at offset zero; CompactLive has the same counts.

| Source | Direct executed transitions | Reuse executed transitions | Reused transitions | Logical service steps on both |
|---|---:|---:|---:|---:|
| Competing consumption | 27 | 15 | 12 | 27 |
| Intermediate observer | 27 | 15 | 12 | 27 |
| Earlier binding | 23 | 13 | 10 | 23 |
| Surviving history | 23 | 13 | 10 | 23 |
| Absent resource | 11 | 7 | 4 | 11 |
| Early failed branch | 10 | 10 | 0 | 10 |

These counts exclude the cost of recognizing and retaining states; fewer executed transitions do not imply lower total cost. The early-failure case has no duplicated future to reuse.

**Changing an inert caller tag prevents all measured hits.** Distinct-tag sources obtain zero hits in both table representations. In these closed programs, no rule head mentions the ground `caller` tag, so it cannot affect matching or bindings. It must still appear in the final residual answer. Ignoring it in a state key without separately retaining the caller's own tag would return the wrong answer.

This yields a concrete missing comparison: preserve the full active resource/history computation and its FIFO service, while moving provably inert ground residual data into a separate per-derivation owner. The experiment has identified the opportunity; it has not implemented or priced that separation.

## Cancellation, ownership and validation limits

Prepared programs support 192 disposal/restart trials per build; 176 disposals interrupt unfinished searches, while 16 reach exhaustion before the chosen cutoff. After each disposal, a fresh query over the same preparation matches complete independent semantics. Held answers remain valid after searches and preparations are disposed. Transition tables are query-owned, so this does not demonstrate cross-query memo reuse or a bounded sustained heap.

The independent scalar evaluator uses a different branch queue convention from persistent execution in tie cases. It supplies complete multiset checks and the explicit unequal-length first-answer witness. General ordered comparisons use Direct persistent FIFO as their control; the report does not claim independent ordered equivalence for every tied source.

The [registration](../registrations/S05-effectful-boundary-gate.md) precedes implementation and runs. Three deterministic tests pass in each default and metrics-off build, and strict Clippy passes in both. The [manifest](s05-effectful-boundary-gate/final/manifest.json) freezes sources, binaries, commands and receipts. [Metrics observations](s05-effectful-boundary-gate/final/metrics-run.log) preserve every source row and work count; [metrics-off observations](s05-effectful-boundary-gate/final/primary-run.log) repeat semantic checks. Zero diagnostic counts in the metrics-off log mean counters are disabled, not absence of reuse.

Each executable is bounded by 60 seconds wall/CPU and 1 GiB address space; each full query has a 200,000-step bound. No production engine, prior frozen probe, reference interpreter or independent evaluator was changed. This gate tests a hypothesis and existing controls; the syntactic predicate is not an admission checker for an adopted optimizer.

## Next comparison and its priority

**Implement source-derived separation of inert ground residuals within resumable execution.** Infer inertness from absence of the constraint signature from every rule head in the fixed prepared program. Preserve each derivation's actual residual data. Keep active resources, pending effects, variable-bearing residuals and history in the execution state unless a separately justified dependency rule permits their separation. Include new inert data emitted during execution, different caller tags, multiple occurrences and first-answer cancellation.

Compare the new candidate with existing whole-state tables and Direct on the same interfering sources, then challenge near misses where a rule can read the tag or the residual shares a variable that later binds. Those cases must remain connected to execution or receive an independently justified transport mechanism; they cannot be silently treated as inert. Validate every FIFO delivery and retained output before owner and cost measurement.

This is a bounded next implementation, not the full resolution of effectful call reuse. Variable-bearing observations, general relevance, nontrivial resumable call dependencies and cross-query cache lifetime remain required. Its immediate value is that it could retain source interleaving while overcoming a measured cause of missed reuse; neither another atomic-phase restriction nor a full-state cache already answers that question.

Adaptive timing attribution remains the strongest ready alternative with less implementation work. It can resolve existing policy contrasts, whereas this source-derived separation could change what state an execution architecture must retain together. Native ownership, conditional lifetime repair and incremental projection remain independent alternatives. Reconsider these at the separation source gate or a concrete obstruction. Two packages have completed since the adaptive breadth review; the next full review is due within two further packages.
