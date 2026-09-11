# Connected projection loses this pilot mainly in preparation

**Early-filtered enumeration wins 188 of 192 complete-session comparisons.** Four
are unresolved under the registered rule; projection wins none. Projection also
requests more heap allocation and reaches a higher retained peak in every case.
This result favors enumeration for these bounded sources and reuse counts, while
identifying preparation as the next concrete optimization question.

## The comparison

The [registration](../registrations/S06-connected-cost-pilot.md) covers 384 cells:
two engines, three connected families, four/six coordinates, plain/duplicate
choices, ordinary/aliased outputs, one/four queries, retain-none/all and complete/
first-tuple cancellation. Names are a/b/c. The unequal-name star admits many
hidden witnesses; equality stars and cliques permit strong early filtering.

The existing enumeration implementation combines duplicate domain choices into
weights and checks each relation as soon as its variables are bound. It is a
credible optimized control, not full Cartesian enumeration. Both engines start
from the same term region, with encoding and relation construction inside their
preparation. Independent full choice enumeration validates every full result
before measurement; measured complete or prefix records are checked outside clocks.

The phase sum includes source construction, preparation, source disposal, changing
query input, setup, first observation, remaining observations, producer/input
teardown, prepared disposal and consumer disposal. Retained outputs survive producer
and prepared-state teardown. Query count is explicit; no workload weights or
unmeasured compilation benefit enter the comparison.

## Complete time and requested allocation

All 192 comparisons pass the registered clock floor. Ratios below one favor
projection. A loss requires median >=1.10 and all ten paired block ratios >1;
a gain requires median <=0.90 and all ten <1.

| Family, 64 comparisons each | Gains / losses / unresolved | Median time ratio range | Requested-byte ratio range | Peak-owned-byte ratio range |
|---|---:|---:|---:|---:|
| Equality star | 0 / 64 / 0 | 2.40–5.95 | 5.13–7.87 | 1.77–2.34 |
| Equality clique | 0 / 61 / 3 | 2.99–11.72 | 6.53–11.24 | 2.16–2.23 |
| Unequal-name star | 0 / 63 / 1 | 3.18–11.99 | 9.78–18.88 | 1.20–2.10 |

Allocation evidence comes from **768 separate processes**, with exact repetition
of every cell. Consumer-owned bytes match across engines and every session returns
to its original live allocation. Peaks are reconstructed across phase readings;
requested bytes are allocator requests, not RSS. Ordinary timing uses **3,840
processes** with metrics and the allocation meter disabled.

Four contrary observations remain in the data. Their scenario median ratios are
5.18, 8.73, 7.72 and 5.70, but each has a block ratio below one, so all four remain
unresolved. Scoped Clippy ran briefly while the matrix was live; there is no
per-process scheduler evidence attributing any outlier to it. No run was excluded
or replaced. The broad loss does not establish a universal architecture ranking.

## What could change this result?

Preparation takes 70–85% of projection session time on equality stars, 78–95% on
cliques and 78–92% on unequal-name stars. This includes domain dictionaries, local
predicate evaluation, ordering and elimination; the pilot does not separate those
four costs yet.

A post-pilot bound sets projection preparation time to zero while leaving every
other measured phase and the entire enumeration session charged. The resulting
median ratios span 0.54–1.18, 0.41–1.13 and 0.42–1.57 respectively. These are
analytical bounds, not achieved optimizations or new timing verdicts. They show
that preparation work could change some comparisons, but cannot by itself repair
every case.

The setup-plus-observation ratio on unequal-name stars ranges from 0.38 to 4.08.
Thus eliminating witnesses sometimes helps query execution even though complete
cost loses. A source-specific preparation optimization merits investigation;
merely increasing reuse until a favorable result appears would not settle the
architecture question.

## Repairs, verification and next action

A preliminary ownership check found spare vector capacity in the enumeration
adapter's aliased output. Explicit slot capacity repaired it before the registered
matrix. The matrix now checks equal consumer ownership for every pair. Both runner
builds pass Clippy and every process validates its answers. The audit verifies all
4,608 terminal results, phase sums, ownership and frozen inputs.

The audit's final JSON comparison initially compared tuples to decoded lists.
Canonical comparison repairs that check. The exact registered driver is preserved
and hash-checked separately; the run outputs and measured binaries are unchanged.
[Raw evidence and analysis](s06-connected-cost-pilot/analysis.json) ·
[Reproducible attribution](s06-connected-cost-pilot/diagnosis.json).

T076 next attributes preparation to dictionary construction, relation generation,
ordering and elimination in a separate instrumentation build, using matched
ordinary controls. This decides whether sparse factor traversal, direct predicate
lowering or equality normalization can remove the dominant work. Ordered transport
with source provenance, prefixes and cancellation remains required; this weighted
endpoint does not substitute for it. Whole-call recognition remains a competing
investigation at the next gate. This is package two since the last breadth review;
the full research goal remains active.
