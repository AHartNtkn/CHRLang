# R01 anchor-information follow-up

Test whether making an active occurrence's known pattern available before partner enumeration changes the dedicated execution control. The initial pilot exposed substantial unavailable-key scans; this comparison measures a specific correction rather than a new language restriction.

## Design and semantic gate

Both generic and generated selectors pre-match a later active head into a local frame before enumerating earlier heads. Pattern matching remains nonbinding with respect to query variables. Earlier heads retain their enumeration order; IDs and kept/consumed positions remain in source order. A failed pre-match ends that anchored search, not the query. Successful pre-matching yields before another candidate is visited. First-head anchors need no pre-match.

Before timing require default and counter-free tests, including bounded resume, independent application/terminal checks, full observations, duplicate resources, later-anchor unknown structure and repeated slots. The focused partner-discovery test failed on the initial implementation (318 visits at n=8 against a 270 upper bound) and must pass after correction. It checks meaningful discovery work and full answers, not emitted text. An independent reviewer checks the change before comparative runs.

Hypothesis: active indexed flat/delayed chains avoid most earlier-head scans when supplied with the later anchor's key. Active scanning may save failed later-head checks but still enumerate earlier pools. Global execution is an unchanged-algorithm control. Collision may pay additional pre-matching without pruning equal-key partners. Single-head build and repair should show no material algorithmic improvement; adverse overhead remains evidence. If the large active indexed scan burden survives, inspect its source before considering the control competent. If it falls substantially, reassess R02 representation integration against access-maintenance refinements.

## Exact run registry

Use the same 80 cells and four-query n,n+1,n,n+1 sequence as the [initial pilot](R01-native-pilot.md): five families, sizes 8 and 64, two execution forms, two policies, two access modes. Compare two independently launched variants: `before` is the preserved d73efe0 source-freeze binary, `after` is the new frozen source. Five primary timing repetitions, one work-count repetition and one allocation repetition per variant/cell: 1,120 isolated processes. Shuffle every job with `random.Random(20260909)` and save the complete order. Do not reuse the first pilot's timings as the before control.

Use the initial pilot's phase definitions, ordinary allocator for time/work, counter-free time/memory builds, installed toolchain and default release profile. New builds use `/tmp/chr-r01-anchor-{time,work,memory}` and copied binaries `/tmp/chr-r01-anchor-bins/{time,work,memory}`. Preserved before binaries remain `/tmp/chr-r01-bins/{time,work,memory}` with hashes verified against the initial freeze. Record both hashes and the new source/generated freeze before running. Native compilation remains excluded; the extra adverse semantic fixture changes the bundled compilation unit and code layout, so small differences in unchanged controls are not optimization effects.

Run `python3 research/chr-compiled/experiments/anchor_pilot.py`. Child affinity is the minimum allowed CPU; 2 GiB address-space, 20 CPU-second, 30 wall-second limits, no core dumps; 15-minute whole-batch bound. Run children serially without concurrent builds/experiments. Preserve all failures/timeouts/cutoffs; stop on nonzero exits or answer/feature/retention validation errors. Four successful answers and equal post-answer live bytes are required for a completed memory cell. No fixed overhead subtraction or discarded warm-up.

## Interpretation and disposition

Report five-process distributions and before/after lifecycle and execution ratios per cell, not a cross-family average. Differences below 10% or overlapping min/max ranges remain timing-inconclusive. Compare work counts to attribute eliminated traversal, and requested allocations to test whether that reduction also saves physical work. No universal architecture, isolated compilation break-even, RSS, OR/search or parallel conclusion is supported.

This follow-up has greater immediate value than R02 implementation because it resolves a discovered avoidable cost in the conventional control with a bounded semantic change to the matcher. It does not require R02 to use that matcher. Afterward select the strongest next architecture-changing question; do not extend this pilot merely because additional sizes or an index refinement are feasible.
