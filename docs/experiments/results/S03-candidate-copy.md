# Candidate copies dominate misses but do not explain the architecture gap

Candidate argument and environment copies account for 9.93 MB of the large miss case's 12.26 MB requested heap traffic. Even eliminating those copies entirely leaves 2.33 MB, versus 1.00 MB for specialized explicit execution. Copy avoidance alone cannot reverse that allocation comparison.

**Direct integration is the next investigation.** Copy avoidance remains a concrete opportunity, especially for misses, but the evidence does not justify another general demand matrix now. This is a scheduling decision, not a rejection of demand execution or a runtime conclusion.

## What was measured

The [registration](../registrations/S03-candidate-copy.md) reuses the [nonground-post ownership experiment](S03-post-ownership.md), its existing runner, independent scalar answers and frozen explicit controls. Six source families, two sizes, both arrival orders, three demand policies, retained-all consumers and full/alternating-cancel runs give 144 configurations. Two isolated profiling processes per configuration give 288 processes.

Callbacks surround only candidate argument-vector copies, environment copies, and selected-occurrence-vector copying/appending. They use cumulative allocation checkpoints without resetting peaks. The argument/environment scopes count candidates reaching actual matching after preliminary filtering; they are not counts of every resource visited. The values below sum four changing queries using one prepared ruleset.

All 144 configurations reproduce the frozen parent's allocation phases, normalized live heap and peaks, and complete/cancelled answer endpoints exactly. Both attribution repeats agree exactly, and task-owned live heap returns to its initial value. Independent answer validation and retained-answer checks remain outside measurement intervals. No timing values are interpreted.

## The large miss loss needs more than copy avoidance

For size32, original arrival, within-turn miss reuse and four complete queries:

| Source | Total requested bytes | Bytes in the three copy scopes | Optimistic remainder | Lowest-allocation explicit control |
|---|---:|---:|---:|---:|
| Input post | 1,614,575 | 56,512 | 1,558,063 | 1,048,256 |
| Output post | 1,614,631 | 56,512 | 1,558,119 | 1,176,128 |
| Forward producer | 2,619,431 | 282,816 | 2,336,615 | 1,658,262 |
| Unsuccessful match | 12,259,743 | 9,930,752 | 2,328,991 | 998,792 |
| Duplicate resources | 4,285,842 | 88,064 | 4,197,778 | 1,753,898 |
| Repeated template input | 2,214,015 | 30,720 | 2,183,295 | 685,080 |

The optimistic remainder subtracts every attributed byte without charging replacement machinery. It is a sensitivity bound, not an implemented optimization. It does not bound savings from a different matching algorithm that also avoids candidate visits and their other work.

The miss case copies 47,744 candidate argument vectors and 47,744 environments across four queries. Arguments request 763,904 bytes; environments request 9,166,848. No occurrence tuple is selected. These copies account for approximately 85% of execution/observation allocation traffic, but the remaining traffic still exceeds the best explicit control's entire lifecycle. Environment copies are therefore a real local cost without being a sufficient explanation of the complete loss.

The other sources have much smaller copy fractions. In particular, eliminating copies leaves most of the duplicate-resource and repeated-template traffic intact. These results do not support making a universal match-copy fix the prerequisite for investigating a distinct architecture.

## The narrow crossover remains worth retaining

Among the 72 complete demand configurations, 57 request more bytes than the least-allocating applicable explicit control. Eliminating all three copy costs still cannot reverse 54 of those losses. The remaining three use the size8 forward source in original order, once for each demand policy:

| Demand policy | Observed bytes | Optimistic remainder | Specialized explicit bytes |
|---|---:|---:|---:|
| Birth reuse | 367,095 | 336,951 | 366,750 |
| With miss reuse | 372,423 | 342,279 | 366,750 |
| With miss and template reuse | 384,191 | 354,047 | 366,750 |

These are unweighted scenario counts. They establish neither workload prevalence nor a universal competitor. A concrete follow-up would borrow already-bound match information or delay copies until a candidate extends it, preserving partial bindings, late aliases, consuming claims and cancellation. It must measure the replacement's overhead and complete lifecycle before claiming these possible crossovers.

## Next decision and remaining obligations

This completes the second package after the nonground-post breadth review: ownership/work, then causal attribution. Direct integration takes priority because it can change equality representation, discovery and the handoff from matching to consuming bodies. The copy repair can reverse only a narrow part of the measured allocation ordering by itself; runtime consequences remain unknown. Intermediate joins are the next distinct alternative because they may avoid candidate discovery itself. Compact solving retains its separate entry priority.

T072 should first reconcile the existing CHR-constructor, direct-handle and integrated-body gates with their later lifecycle evidence, then qualify the missing common consuming-source contrast. Reuse existing implementations and independent controls. Demonstrate which service boundary disappears and compare the obligations of flat relations, CHR-expressed merging and local handle repair separately. A new representation must not receive credit merely for reimplementing an already measured equality service.

T071 remains unfinished. Its next consequential alternatives are the narrow copy repair, avoiding repeated candidate discovery rather than merely copying, primary timing of credible favorable paths, writable resource-head dependencies, broader dynamic choices and sustained ownership. Revisit these after the next integration package or sooner if that comparison requires demand as a competent control. No language restriction or architecture is selected here.

## Validation and receipts

The profiling and unprofiled allocation builds pass the complete-source/prepared-reuse example test. Strict scoped Clippy passes for the profiling build. The meter's cumulative-checkpoint self-check passes; checkpoints are now available independently of fork diagnostics. The initial compilation exposed that feature coupling before any workload ran, as recorded in the qualification note.

The [audit](s03-candidate-copy/audit.json) reconstructs all 288 raw processes, compares every phase with the frozen parent, checks exact repeats and derives the bounds above. The [freeze](s03-candidate-copy/freeze.json) records source archive and binary hashes, exact cells, parent evidence hashes and affinity. Raw commands, outputs, resource limits and build/test receipts are in [the evidence directory](s03-candidate-copy/). The earlier recorded-read, specialization and nonground-post audits also pass using their preserved sources and binaries.

Requested heap traffic is not RSS or a speed measurement. The subtraction establishes an allocation bound for these specific copy operations; it supplies no bound on elapsed time, compiler economics, sustained memory or a different resource-discovery algorithm.
