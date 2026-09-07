# E16 regional lifecycle pilot

Registered before comparative runs. This exploratory sizing pilot follows the
[E16 regional semantic gate](../results/E16-regions-gate.md). It does not select a
production scheduler, certificate, or worker architecture.

Question: does retaining complete certified independent source regions on workers
make service granularity a useful alternative to transferring owned equations?
The equation results motivate this interface comparison; they do not predict a
speedup. We test whether larger quanta amortize transport, whether owner product
and observation work limits gains, and whether imbalance, duplicate products,
refutation, and prefix stopping cause speculative overhead.

## Frozen cells and controls

The authoritative constructors are
`research/chr-factors/examples/support/region_cost_cases.rs`. Ten cases:

- `one-zero`: one binary region, zero carry steps; overhead control.
- `one-work`: one binary region with 256 unary carry steps; no parallel source capacity.
- `two-work`: two such independent regions; balanced capacity.
- `owner-product`: eight cheap binary regions, 256 distinct final products.
- `asym-first`, `asym-last`: four binary regions, one with 256 carry steps at either end.
- `duplicate-eight`: eight a/a regions, 256 raw products and one unique answer.
- `refute-loop`: a looping region beside an exhausted empty region; no answers.
- `stream-prefix`: unary naturals times a/b, stop at eight unique answers without exhaustion.
- `mixed-add-infer`: the existing independent arithmetic/type application product.

For each: original Factored Baseline Q1, plus Inline/Threads1/Threads2 at Q1 and Q8,
K4 (70 cells). Add those three new modes at Q64/K4 on one-work and two-work (6).
Add Q8/K1 on two-work (3). Thus 79 cells per executable, 158 fresh child processes.
System timing and allocation-meter runs are separate and sequential. No other
builds, tests, profiling, or experiments run during this batch.

Baseline is the original regional control. Same-Q Inline is the transport control
for each worker mode. Q changes source/product scheduling; comparing Q8 workers
only to Baseline Q1 would confound scheduling with concurrency. Two versus one
worker is a capacity comparison, and the one-region case controls for useful
parallel capacity. K1 versus K4 measures admission on a balanced workload.

## Correctness and measurements

Preflight checks finite full output/residual sets, raw multiplicity and exhaustion
against hand Cartesian expectations or existing application expectations. Q1
ordered answers match original Baseline; workers match their same-Q Inline
ordered answers. The infinite prefix is independently checked for eight distinct
natural×a/b tuples with empty residuals, not a Q-independent prefix order. A finite
prefix does not establish fairness. Refutation must not await an infinite sibling.
Shutdown checks complete replies, released reservations, unchanged accepted
observations, and actual source steps at least accepted steps.

Input fixtures are built before measurement; cloning rules/query, certificate and
worker construction are charged. Record construction, first trusted answer,
search stop, cancellation/drain/join, search release, and separate output release.
Outputs remain retained through search release. Validation occurs outside timing.
Primary cold elapsed time is `wall_through_drop_including_diagnostics_ns`.
Fixed numeric, nonallocating snapshots introduce an explicitly reported
`diagnostic_gap_ns`; background work can proceed in that gap, so subtracting it
cannot produce a valid parallel critical-path time. The operational phase sum is
only descriptive. Differences near diagnostic overhead are inconclusive.

Record accepted and post-join actual source/observer work, owner product and global
observer work, certificate work, cached answers/jobs, raw availability, transport
admission, buffered replies and cancellation. Actual work after prefix/refutation
may vary with worker timing and is not required to equal Inline work. Unserved
quantum slots are not a count of computation avoided.

System is explicitly selected for timing; the separate meter charges requested
allocation bytes/calls/live peak from construction through join, then search and
output release at quiescent boundaries. No live-worker reset/read race is allowed.
Heap measurements do not include allocator-reserved space. Process peak RSS also
includes fixture setup, runtime and validation; report it separately.

## Bounds and interpretation

Each child has 30 seconds wall time and 1 GiB address space. Source owner-turn
budgets are fixture-specific. Retain errors and timeouts as observations; neither
supports closure. Freeze binary, source, registration, runner and exact cell order
hashes. Do not overwrite a recorded batch. Audit all expected keys and invariants.

One observation per cell is exploratory, not a performance ranking. Use it to
choose repetitions, sizes and resource bounds for a separately registered
comparison. If W256 is too cheap or inconclusive, larger service workloads remain
feasible follow-ups. If overhead persists, retained regional ownership still does
not isolate channel, source representation, allocator or lifecycle causes. Warm
pools, larger region/worker counts, connected applications and finer certificates
remain separate questions. The reference interpreter remains independent.
