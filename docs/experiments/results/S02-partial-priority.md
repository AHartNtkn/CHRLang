# Partial equality preserves permitted answers, but can change consumer priority

**Stable positive guards do not make eager partial equality preserve a fixed consumer priority.** In all 12 successful source configurations, the eager relational executor selects `newer`; settling equality first selects `older`. Both results correspond to an independent atomic execution under the respective consumer order. This is a scheduling-contract distinction, not an equality error or an architectural winner.

The repository already establishes [useful early failure and its adverse control](S02-useful-interleaving-control.md), complete contextual execution, and [integrated bodies with a body-completion boundary](S02-integrated-bodies.md). This experiment carries those results forward and tests the remaining connection between successful partial execution and priority. It does not present the earlier callback or early-failure witnesses as new mechanisms.

## The source distinction

A bind rule posts a constructor equation. Its first useful consequence is `X=a`; settling a nested component later establishes `Y=b`. Two rules compete for the same request and token:

```text
older: use(X,Y,W), token <=> X=a and Y=b | W=older
newer: use(X,Y,W), token <=> X=a         | W=newer
```

Here the expressions before the bar are nonbinding, positive equality guards. The fixed-priority control considers `older` first after atomic settlement. Both guards then hold, so it consumes the request and token. Eager execution reaches `X=a` while the leaf deduction is pending; only `newer` is currently enabled, so it claims those same resources. Later establishing `Y=b` does not restore the consumed request or change the winner.

The test observes the actual live-occurrence index at the claim: every eager case has pending equality work then. Its initial instrumentation checked the allocated-identity set, which includes retired occurrences; switching to the authoritative live-location index makes the measurement reflect consumption. The source and winner assertions are unchanged.

## What the gate verifies

The matrix crosses nested depths 4/16/64, both component orders, a successful or late-clashing tail, and one/two duplicate tokens: 24 configurations. The two schedules use the same existing relational implementation and the existing test-local settlement barrier. Independent atomic evaluation checks the fixed-priority result and the alternative consumer order. Complete outputs include the winner and both logical variables; duplicate-token residual multiplicity is exact.

| Endpoint | Eager partial execution | Settlement barrier |
|---|---|---|
| 12 successful cases | `newer` | `older` |
| 12 late-clashing cases | No answer | No answer |
| Successful equality deductions at depth 4/16/64 | 8 / 20 / 68 | 8 / 20 / 68 |
| Late-clashing deductions at depth 4/16/64 | 8 / 20 / 68 | 7 / 19 / 67 |

The successful cases save no equality deductions. The failed cases service one additional deduction for the speculative winner binding before discovering the pending contradiction. No speculative result is published.

Every configuration also cancels after the request has been consumed while equality is pending, then starts a fresh query using the same prepared rules. Its complete result matches independent settled evaluation. This checks query-state isolation after cancellation; it is not an allocation or RSS claim.

Two frozen, bounded confirmations pass all seven relational library tests. All 24 new work rows and all 12 established early-failure/leaf-dependent control rows reproduce exactly. Scoped Clippy passes. Reference and scalar implementations are unchanged. There is no timing comparison.

## What this means for architecture and language design

The [successful serialization analysis](R02-partial-equality-serialization.md) assumes no mandatory priority that prevents selecting the recorded live tuple. The new result agrees with that argument: the eager successful answer has an atomic execution when the later consumer may be chosen first. It does not agree with the more specific policy that settles equations and gives the earlier consumer precedence.

Consequently, two different comparisons are legitimate:

- Under permissive source choice, compare eager and settled execution using independently allowed answers and complete resource/failure semantics.
- Under fixed consuming priority, compare the settled control with an integrated executor that also validates the priority of a ready application. Stable guards alone are insufficient; an earlier currently blocked contender may become enabled by pending equality.

Neither policy is adopted for a future language here. Their observable difference must be explicit before comparing costs. Treating the eager winner as a same-policy optimization would weaken the experimental contract; treating the discrepancy as a rejection of all interleaving would overstate the evidence.

## Next investigation

T072 next qualifies priority-aware readiness against the full settlement control. Distinguish a higher-priority contender that is definitely impossible from one that is unresolved, and include shared versus disjoint resources, later contradiction and freshly enabled requests. Test whether unrelated equality work can remain pending while a safe source application proceeds. The candidate must preserve independent fixed-priority results, and must show whether it avoids useful work or only adds readiness checks.

This is more informative now than another replication of the established early-failure family: it tests the extra responsibility exposed by the new successful witness. Broader guard contracts, general joins, contextual consequence reuse, CHR-expressed and strategic graph organizations retain their separate obligations. Compare the next gate with partner-plan work and the ready graph memo/lifecycle costs; conduct a full portfolio review within four packages. This is package one after the latest review. T072 and the goal remain active.

[Registration](../registrations/S02-partial-priority.md), [source tests](../../../research/chr-relational/src/interleaving_tests.rs), [frozen confirmations and audit](s02-partial-priority/), [repeat driver](../../../research/chr-relational/experiments/partial_priority.py).
