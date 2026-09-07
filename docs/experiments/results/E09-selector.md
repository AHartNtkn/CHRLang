# E09 predicate-filtered selection

All 624 registered configurations pass and replay exactly: 192 source-transition
checks over all 64 E00 cases, 324 mixed scheduling configurations and 108 opposed
matching-predicate configurations. Source gate work is invariant across quanta.
The preceding 324-row full-scan matrix also replays exactly. Seventeen tests pass,
including root aliases, unknown roots, distinct occurrences and selected tuple order.

The filter rebuilds head-specific occurrence pools on each sealed source step,
resolving root aliases without binding. Pools retain store order. Cartesian-product
order with repeated IDs excluded is the same subsequence of the full ordered
occurrence tuples that can match. Thus the first enabled rule/tuple stays unchanged;
independent transition replay checks this on the source gate. Full head matching,
guards and propagation history remain authoritative after filtering. This is not
an incrementally maintained index or a source-language change.

## Opposed results

At size 12, quantum 8, ungrouped batch 8:

| Workload | Policy | Scan actions | Filter actions |
|---|---|---:|---:|
| Incompatible predicates | round | 11,421 | 906 |
| Incompatible predicates | async | 11,449 | 934 |
| Matching q predicates, rejecting equality guard | round | 48,546 | 54,555 |
| Matching q predicates, rejecting equality guard | async | 48,574 | 54,583 |
| Matching edge predicates, acyclic join | round | 43,037 | 48,966 |
| Matching edge predicates, acyclic join | async | 43,065 | 48,994 |

Filtering eliminates the large incompatible-predicate scan. Async still returns
`a` at action 792 versus round at 906, but the large preceding gap is substantially
reduced. Round returns `b` first at 869. This confirms that the earlier large delay
was mostly avoidable source selection, while the barrier effect remains at a
smaller scale. These are action indices, not measured wall-time differences.

In the opposed cases, head predicates match actual stored constraints. The q guard
compares the first two selected values; distinct occurrence values make it false
on this dataset, rather than a literal constant-false source guard. The edge case
seeks a three-edge cycle in an acyclic path. Both have independent hand-derived
complete observations checked against the tree control.

The filter cannot prune these large pools and incurs product/ID checking work.
For the asynchronous acyclic-join case, tuple candidates rise from 1,347 to 1,730;
there are 5,042 distinct-ID checking actions and 66 pool visits. Matching and
resolution fall only slightly (12,471 to 12,446 and 22,312 to 22,116 respectively).
This attributes the adverse result to this enumeration strategy, not to an
unavoidable predicate-index cost.

## Next comparison and boundaries

Implement yielding distinct-prefix enumeration with head matching as each prefix
is extended. It can reject a reused occurrence or inconsistent shared variable
before extending the rest of a tuple. Compare it with both existing services,
charging prefix bindings, copies and candidate discovery. Preserve source order
and validate partial-match rollback independently. Guard placement is a further
factor: an early guard needs a certificate that its required variables are already
bound; it cannot silently change nonbinding guard semantics.

This is useful work because the adverse cases expose precisely the wasted suffix
search it may avoid. Maintained indexes and binding-sensitive wakeup dependencies,
heterogeneous operation sharing, longer synthesis and second-service/net composition
remain open. The current filter establishes neither efficient general joins nor a
production scheduler recommendation.

All costs include explicit pool/root/ID actions alongside source, grouping and
observer work. Host collection allocation and iterator setup remain outside a hard
real-time bound. No new runtime or byte claims follow from this matrix.

Evidence: [raw results](E09-selector.jsonl), [replay](E09-selector-replay.jsonl),
[source and registration hashes](E09-selector-manifest.json).

```sh
cargo run -q -p chr-symbolic-fixtures --example export_cases > /tmp/chr-e09-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-scheduling -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/run_selector.py < /tmp/chr-e09-cases.jsonl
```
