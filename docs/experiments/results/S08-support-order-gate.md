# Reverse diagram order saves nodes but adds observation work

Reverse variable order reduces retained support nodes from 16,986 to 613 on the depth 64 stream. With identities and caching, total engine calls nevertheless rise from 779,212 to 827,092 because chronological observation needs a more general traversal. The experiment establishes a representation/observation tradeoff, not a winning order.

## The comparison includes the observation contract

The current ascending diagram order permits a shortcut: after assigning earlier source births, restricting the next birth needs at most the current diagram root. Reverse diagram order does not preserve that invariant. Changing only Boolean apply order would give an incorrect history enumerator.

The experimental general enumerator instead checks whether any diagram path agrees with the assigned chronological prefix. It services one node per call, using a visited set within that prefix to avoid repeated traversal of shared nodes. Unassigned variables are existentially explored; each birth's causal guard still determines whether it can branch, and inactive births retain canonical false. Frozen birth prefixes and ordered complete histories are preserved.

Three controls isolate the costs: ascending order with the existing direct cofactor method; ascending order with general feasibility; reverse order with general feasibility. Each runs ordinary Boolean jobs and the combined identity/cache policy. Physical diagram order changes neither variable identities nor source birth order. Both new features remain off by default.

## Correctness evidence

Both general ascending and reverse builds pass 144 complete counter-free crate tests with the combined Boolean policy. These include independent Boolean truth tables, canonical results, restricted causal histories, consuming resources, cancellation, frozen birth prefixes and source observations. Focused baseline-order tests also pass. Fixture diagrams are constructed in each physical order while preserving the same integer truth-table expectations.

The single-cube service test preserves its linear bound for the original method. General feasibility has a separately stated conservative polynomial bound: it may revisit diagram nodes for successive assigned prefixes, but must not enumerate all incompatible complete histories. This added observation obligation is part of the experimental cost, not hidden by an equal-service claim. Scoped strict Clippy and formatting pass; [validation logs](s08-support-order-gate/validation/) record the checks.

## Work and retained node counts

All 36 [registered diagnostic runs](../registrations/S08-support-order-gate.md) complete with independent scalar and analytical full-answer agreement. They cross the three controls, two Boolean policies, aliases/distinct outputs and depths 0/16/64. The [audit](s08-support-order-gate/audit.json) checks raw summaries and frozen inputs. All 12 ascending-order observation-control pairs preserve identical support nodes, job counts and Boolean frames, isolating the extra enumeration calls.

Depth64 aliases:

| Diagram / observation | Boolean policy | Engine calls | Boolean frames | Retained support nodes |
|---|---|---:|---:|---:|
| Ascending / direct | Ordinary | 1,257,853 | 1,004,853 | 16,986 |
| Ascending / general | Ordinary | 1,457,560 | 1,004,853 | 16,986 |
| Reverse / general | Ordinary | 1,309,240 | 914,448 | 613 |
| Ascending / direct | Combined | 779,212 | 511,729 | 16,986 |
| Ascending / general | Combined | 978,928 | 511,729 | 16,986 |
| Reverse / general | Combined | 827,092 | 417,808 | 613 |

Reverse order reduces both Boolean frames and retained nodes relative to ascending order under the same general observation method. The original direct-observation control still uses fewer total calls. Node counts include all nodes retained in the append-only arena, not only those reachable from a final result. They are neither allocated-byte measurements nor proof of safe earlier reclamation. [Complete summaries](s08-support-order-gate/summary.json) retain the other sources and depths. No timing or allocation comparison ran.

## An opposite-direction witness

A separate independent conjunction construction reverses the favorable order. With 64 variables arriving oldest first, ascending diagrams retain 2,082 construction nodes and reverse diagrams 129. With operands arriving newest first, those counts swap. Both yield the same conjunction, verified with all-true and each single-false assignment. Widths 1/4/16/64 obey the corresponding construction-count formulas in both builds.

This witness measures construction retention, including intermediate nodes; it does not claim different minimum sizes for the final Boolean function. It establishes a concrete adverse arrival shape for reverse order. The stream's favorable arrival pattern cannot justify a universal ordering policy.

## Next decision

A bounded lifecycle comparison should now charge the general enumerator's visited sets, repeated traversal, disposal and retained memory alongside the lower support-node population. Include both physical orders under the same observer and the original direct-observer control. Carry the opposite-direction construction witness into a source-level adverse case where possible; do not select an order from the favorable stream alone.

This comparison could materially change the conditional architecture's retention and necessary observation machinery. It is more discriminating than another cache-capacity sweep while the order/observation interaction is unmeasured. Non-overlap/effect certification remains required and returns at the next result boundary. This is package three since the repetition breadth review; review breadth after the next lifecycle package. Sparse substantive reuse, sound reclamation, publication, residual layouts, compilation, coherent architecture comparisons and held-out challenges remain unfinished. The goal remains active.
