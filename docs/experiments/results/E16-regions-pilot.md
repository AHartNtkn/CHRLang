# E16 regional lifecycle pilot results

All 158 fresh-process observations pass the frozen 79-cell matrix and independent
accounting audit. This is exploratory sizing evidence, not a performance ranking.
[Registration](../registrations/E16-regions-pilot.md),
[raw observations](E16-regions-pilot.jsonl),
[manifest](E16-regions-pilot-manifest.json),
[audit](E16-regions-pilot-audit.json).

Twenty-one chr-factors tests pass, including the cost workload preflight, alongside
workspace Clippy and formatting. The preflight checks mathematical stream
membership, finite full residual/output sets, exact raw multiplicity, and matched
ordered answers at each quantum. Reference code is unchanged. Source statistics
are now exposed by nonallocating iterators so measurement snapshots do not add
heap buffers; original factor semantics are unchanged.

No child errors, timeouts or mismatches occurred. The longest child took 0.01755
seconds against a 30-second cap. Maximum process RSS was 5,248 KiB; maximum
construct-through-join requested-heap peak was 1,507,794 bytes. Every frozen source,
binary, registration, runner and auditor hash remained unchanged. Memory runs use
a separate allocator-instrumented executable; their timings are not performance
evidence.

The balanced two-region W256 case at Q64/K4 is the useful sizing signal: cold wall
was 1.043 ms with two workers, 1.816 ms with one, and 1.562 ms Inline. At Q1, two
workers took 12.809 ms versus Inline 1.190 ms. The one-region Q64 control took
1.024 ms with two workers versus Inline 0.782 ms. These single observations suggest
that service granularity and available independent work deserve a repeated
comparison. They do not establish a speedup or attribute the difference to one
cause. W256 is sufficient for the next confirmatory entry; a larger size sweep
remains feasible if the repeated comparison is inconclusive.

The infinite prefix exposes scheduling and cancellation costs. Q1 accepts 35
source steps; Q8 accepts 71 to produce eight unique global answers. In the System
run, Q8 Inline physically executes 79 steps, one worker 71 and two workers 72.
Refutation at Q8 accepts three steps but physically executes 11 Inline, nine with
one worker and 11 with two. The exact physical surplus can depend on timing;
accepted work and answers agree with the matched controls. Unused quantum slots
are not avoided-computation counts. Stop-time actual snapshots describe received
replies; joined snapshots supply complete physical accounting.

A proposed follow-up is a separately registered randomized repeated comparison of
all 79 cells, preserving overhead, imbalance, product-heavy, duplicate, prefix,
refutation and application controls. Primary cold wall includes the reported
nonallocating diagnostic gaps; operational phase sums are descriptive. These
results do not settle warm pools, larger worker counts, finer certificates,
connected synthesis, temporary reunion, or representation/reclamation costs.

A [supplementary lifecycle audit](E16-regions-pilot-lifecycle-audit.json) also passes all158records, checking complete owner-state preservation, transport/quantum bounds and allocation phase continuity. It is post-run validation, not another cost batch. The architectural assumption audit now takes priority over the queued repeated comparison.
