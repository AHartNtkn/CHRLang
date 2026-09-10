# Whole-query failure checks avoid unnecessary subtraction

Eager learning can construct complements that a later retained failure immediately makes useless. Checking complete coverage first removes most of the excess allocation in the affected long-reuse case. The correction adds a set-containment scan, whose time cost remains unmeasured.

## Evidence for the cause

The regression first learns failure over a/b, then over a/b/c. A repeated wider query is already known to fail completely. Before the correction, the older partial region nevertheless creates complements and causes a zero-partition-budget error. The failing test records that error; the corrected implementation answers the same query with zero partitions and independently verified empty results.

The learner now checks whether any retained region contains the entire incoming normalized query region before subtracting partial failures. Coverage requires the same ordered private goals and coordinate-wise domain containment under the same immutable prepared source. The proof is the existing completed failure; no new language restriction or successful-result cache is introduced. Partial coverage still follows eager subtraction. Later covered-state pruning is unchanged.

## Paired allocation result

The frozen ownership matrix repeats 216 configurations in 432 processes. All complete outcomes, deterministic paired phase readings and final requested-heap restoration pass. Both feature builds pass 33 relevant tests, including the new budget regression; strict scoped Clippy passes.

Only four eager configurations allocate less; the other 68 are unchanged. The four have an all-failing relation, capacity four, four or sixteen follow-ups, and either duplicate weight. All 144 recomputation and covered-state configuration summaries are identical to the prior run.

| All-failing relation, weight one, capacity four | Previous eager bytes | Corrected eager bytes | Recompute bytes |
|---|---:|---:|---:|
| Four follow-ups plus seed | 78,414 | 73,815 | 67,233 |
| Sixteen follow-ups plus seed | 155,868 | 123,675 | 123,501 |

These are requested allocations across the recorded ownership phases, including reused preparation and complete caller observation. They are not RSS, timings or full compilation costs. The [ownership report](S06-learning-ownership.md) defines the excluded diagnostic storage and coupled phase boundaries.

Eager allocation remains above recomputation in all 72 configurations. Covered-state checking still allocates less in six and more in 66. The correction therefore changes attribution more than selection: most of the long-reuse excess in this particular case was avoidable, while recognition overhead and the broader time tradeoff remain.

## Breadth review and next decision

The last four learning packages established failed-region validity, compared eager and later pruning, qualified ownership, and attributed the consequential setup cost. They answer a bounded mechanism question. They do not settle general conflict learning, direct resource derivations, independent compilation or coherent architectures.

The next bounded package is counter-free timing qualification and a prospective paired cost pilot covering common-prefix depth as well as reuse and capacity. This could show that extra checks erase the allocation/work benefit, or that one learning policy earns its added machinery. The known semantic and ownership controls make that comparison relatively inexpensive now. Further allocation tuning without a consequential contrary result is not the next priority.

The strongest ready alternative remains T078's mixed-source Rust/native comparison, which can change a broader architecture decision. Its ownership and endpoint accounting must remain explicit; this learning runner does not qualify native allocation or compilation. Reconsider that pilot at the timing qualification boundary or an obstruction, and record the cost/value choice before extending learning runs. Preserve the distinct investigations in the [sequence](../next-cycle.md), including resource derivations and native compilation, regardless of this pilot's outcome. The research goal remains active.

## Evidence

- [Registration](../registrations/S06-learning-setup.md)
- [Failing regression](s06-learning-setup/red.log), [metrics-off tests](s06-learning-setup/gate-off.log), [default tests](s06-learning-setup/gate-default.log), [Clippy](s06-learning-setup/clippy.log)
- [432 raw processes](s06-learning-setup/runs.jsonl), [allocation audit and hashes](s06-learning-setup/audit.json)
- [Paired comparison](s06-learning-setup/comparison.json), [comparison script](s06-learning-setup/compare.py), [frozen process gate](s06-learning-setup/run_gate.py)
