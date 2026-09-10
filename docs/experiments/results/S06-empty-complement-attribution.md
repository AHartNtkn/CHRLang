# Checking for an empty alternative avoids a real state copy

A matching finite domain with only one value has no unmatched alternative. The solver now checks this before cloning the entire state. Requested allocation falls in all 12 tested nonempty finite cases; the four zero-choice cases are unchanged. No speed improvement is established.

## The isolated change

The matching code already proves that the required value belongs to the domain. With cardinality one, removing that value leaves nothing. The current implementation avoids constructing that empty branch. For larger domains it makes the same copy, removes the same value and queues the same unmatched region. Selection, multiplicity, source priority and service quotas are unchanged.

This removes an allocation responsibility in the current implementation; it does not prove that every other solver organization should use this representation. The extra cardinality lookup can have a CPU cost, and broader sources with little opportunity to avoid a copy remain a timing question. The current paired timings do not settle that question.

## Paired evidence

The [registration](../registrations/S06-empty-complement-attribution.md) specifies 18 cases across baseline/corrected binaries: 16 finite cases and two unchanged Scan calibrations. Two allocation repetitions and five ordinary-allocator repetitions per version produce 252 processes.

| Four changing queries, immediate release | Initial requested bytes | Corrected requested bytes |
|---|---:|---:|
| Selective, either arrival order | 868,762 | 795,100 |
| All choices accepted | 16,392,746 | 15,772,090 |
| Duplicate successful choices | 1,108,089 | 1,034,427 |

All 36 allocation pairs replay exactly. All 18 work signatures agree across versions and builds. Only `private_solve` allocation changes; the Scan calibrations have identical allocation and peak growth. Phase continuity and disposal roots pass. The selective peak falls slightly, while the shown unselective peak remains 447,361 bytes.

Timing ranges overlap. On the oldest-first selective batch, baseline and corrected medians are 0.601 and 0.550 ms, with ranges 0.434–0.804 and 0.392–0.741 ms. On the unselective batch the medians are 9.076 and 9.171 ms. These measurements support no speed-adoption claim.

The unselective allocation disadvantage against specialization remains. The [lifecycle report and breadth review](S06-finite-lifecycle.md) retain that result and select native graph correspondence next. Further finite representation and timing questions remain required.

## Validation and provenance

The source change is in [finite_phase.rs](../../../research/chr-compiled/experiments/finite_phase.rs). Independent phase and bridge checks preserve complete answers, raw multiplicity, caller effects, later choices and fresh output aliases. Both default and counter-free package runs pass 154 tests; scoped strict Clippy passes.

[Summary](s06-empty-complement-attribution/summary.json), [completion](s06-empty-complement-attribution/completion.json), [source/binary hashes](s06-empty-complement-attribution/freeze.json), [default validation](s06-empty-complement-validation/package-default.log), [counter-free validation](s06-empty-complement-validation/package-counter-free.log), [Clippy](s06-empty-complement-validation/clippy.log) and the [audit](../../../research/chr-direct-conditional/experiments/audit_empty_complement.py) preserve the evidence. Earlier source bytes have explicit snapshot mappings in each affected experiment directory.
