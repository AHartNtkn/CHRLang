# E16 cold equation-worker cost comparison

The [semantic gate](../results/E16-gate.md), [lifecycle pilot](../results/E16-pilot.md)
and [distinct-hole pilot](../results/E16-distinct-pilot.md) establish the implementation
boundary and feasible workload sizes. This registration tests the cold per-query cost
of parallel owned equations; it does not select a production architecture.

## Hypotheses and comparisons

1. Cheap equations and identity/repeated structures do not reliably repay projection,
   transport and worker startup against Shared. Compare each worker mode with both
   Shared and Owned, not only with another worker count.
2. At fixed source query and K=4, two workers may reduce search or total elapsed time relative
   to one worker on distinct-hole wide tasks. Whole cold cost includes construction,
   installation, observation and release; a search-phase benefit alone is insufficient.
3. K=1 removes equation overlap even with two workers. Comparing the same wide query
   at K=1 and K=4 tests admission effects without substituting a different program.
4. Prefix stopping leaves accepted work to finish; report first answer separately
   from cold completion-through-release and charge all speculative work.
5. Workers may increase peak storage even with bounded requests. Compare separately
   metered peaks and requested bytes, keeping byte and request bounds distinct.

Chains supply no-branch-concurrency controls. Do not use wide-minus-chain time as an
isolated parallelism estimate: chains have a growing caller substitution and one answer,
whereas wide queries have separate stores and eight answers. Same-query mode and K
comparisons are the decisive controls. No claim about width scaling or warm pools is
supported by this matrix.

## Frozen matrix and procedure

Use all seventeen pilot cases: eleven initial cases and six distinct-hole cases.
For each run Shared, Owned, Inline K4, Threads1 K4 and Threads2 K4. Add Inline,
Threads1 and Threads2 at K1 for wide-large and distinct-wide-d-8. This gives 91 cells.
Lookahead remains eight and the reply channel one slot; source FIFO selection is the
comparison control, not a language decision.

Use two warm-up repetitions and seven measured timing repetitions, each with one fresh
child per cell, followed by three separate metered repetitions. Independently shuffle
all 91 cells within each repetition using a deterministic generator seeded 160916;
record the exact order before execution. There are 819 time children (182 warm-up,
637 measured) and 273 memory children, 1,092 total. Warm-up records are retained and
excluded from comparative ranges. Fresh children do not provide warm-pool reuse.

Use the existing two release binaries: explicit System for timing, Meter for allocation.
Freeze binary/source/build-input/registration/runner hashes and record toolchain, host,
CPU affinity, available cgroup information and stack environment. Run without concurrent
assistant builds, tests, profiles or experimental batches. External host activity is
not controlled; independent shuffle reduces order confounding but does not eliminate it.
Keep each child's 30-second wall and 1-GiB address-space limits. Preserve malformed
output, errors and timeouts before stopping the affected batch for investigation.

## Measurements and checks

Use the pilot's exact lifecycle boundaries. Primary elapsed metric is cold construction
through search drop, retaining answers; output drop is a separately timed phase after
external validation. Also retain first-answer latency, construction, search, shutdown,
search drop and output drop. Compare shutdown plus search drop for release across modes.
Do not pool metered timing with System measurements.

Require independent fixture observations, equal source counts across modes, unchanged
source state during shutdown, and complete accepted-request accounting. At matched query
and K, require the new drivers' issued requests and aggregate final solver work to agree
across repetitions and measurement kinds. Allow receive timing and owner-buffered peaks
to vary. Preserve scalar work counts as controls; Shared uses its own arena operations.
Require distinct-hole pair totals predicted by its pilot registration. Prefix-drain must
finish four uncommitted requests at K4; type synthesis must retain its finite-prefix
contract. Any failure invalidates the affected comparison pending repair.

Allocation comparisons use construct-through-join absolute peak, baseline, requested
bytes/calls and post-search/output retained bytes. All meter boundaries are quiescent.
Peaks include fixture baseline; report it. Process RSS includes runtime/setup and thread
storage, with application registry setup contamination as documented in the pilot.
Neither heap accounting nor RSS alone identifies source-store memory or bandwidth.

## Interpretation criteria and follow-ups

For each cell report measured median and full min/max range, without trimming outliers.
For same-case comparisons, call a directional difference separated only when the full
observed ranges do not overlap. Overlap is inconclusive for that direction at this
measurement resolution; a median ratio alone does not establish a win. These are
observed local ranges, not confidence intervals or guarantees on another host. Apply
the same rule to first-answer latency and the three observed allocation peaks.

Report all cases, contrary evidence and represented-operation counts. A two-worker
improvement over one worker does not establish a project improvement if Shared remains
better. A cold loss does not close coarse regions, warm-pool reuse, different transfer
representations, resumable service, width scaling or another worker-count regime.
Use stable cost patterns to prioritize the next mechanism/control, and investigate
inconsistent or unexpected patterns before issuing a bounded recommendation.
