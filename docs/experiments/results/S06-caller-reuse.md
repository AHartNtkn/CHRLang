# Caller reuse helps both projection and enumeration

Reusing completed callers qualifies faster in 39/64 projection cases and 37/64
cases for each enumeration strategy. Projection has no case that qualifies faster
than both enumeration strategies with reuse. The improvement belongs to reuse of
caller execution; it does not establish a preference for projection.

## What is reused

The cache owns one prepared caller program. Its key includes the complete ordered
query: constraints, variable identifiers, output names and aliases. A visible
value alone is insufficient because different token counts and caller facts can
change the result. Hits clone complete raw answer vectors, preserving duplicates,
residual occurrences and the existing local namespace of each Answer object.

This is reuse of a completed, branch-local query. It does not substitute a cached
result into an unfinished live store. Fresh variables retain their relationships
within each answer; attaching an answer to a different live scope remains a
separate transport operation. The 576-context test covers changed facts, token
counts, variable identifiers, aliases, program priorities, fresh output structures,
duplicate alternatives and failure. Independent scalar/reference comparisons
respect variable renaming. Separate owners cannot share entries, and a step-bound
failure cannot install partial results.

## Complete lifecycle comparison

| Reuse compared with | Gains | Losses | Uncertain |
|---|---:|---:|---:|
| Projection without reuse |39|0|25|
| Eager enumeration without reuse |37|1|26|
| Lazy enumeration without reuse |37|0|27|

| Projection with reuse compared with | Gains | Losses | Uncertain |
|---|---:|---:|---:|
| Eager enumeration with reuse |3|37|24|
| Lazy enumeration with reuse |5|26|33|
| Full Direct source execution |52|0|12|

All 384 comparisons clear the recorded clock floor. A qualified gain requires a
median improvement of at least 10% and the same direction in all five blocks.
The three gains against eager enumeration are dense cancelling cases; the five
against lazy enumeration are sparse cases. There is no joint winner against both.
The full Direct source control does not use this per-assignment completed-caller
cache, so that column does not attribute its whole difference to projection.

For a four-coordinate sparse star with unequal branch lengths, eight complete
queries and immediate consumer release:

| Path | Median session time | Requested bytes | Peak requested-live bytes |
|---|---:|---:|---:|
| Projection |2,620.90µs|5,096,641|44,956|
| Projection with reuse |983.40µs|739,526|46,216|
| Eager enumeration |3,252.86µs|5,018,081|38,953|
| Eager enumeration with reuse |541.13µs|660,966|40,213|
| Lazy enumeration with reuse |1,288.80µs|1,174,906|68,030|
| Full Direct source |13,155.06µs|18,303,391|213,518|

These medians illustrate complete costs; they do not replace the paired criteria
or define workload weights. Keys, cloned delivery, cache retention and disposal
are charged. All paths use the same dynamically growing consumer.

## The one-answer cost is fixed

Unconditional caching requests more bytes in every one-query, first-answer case:
there can be no cache hit. One eager-enumeration case also qualifies slower.
The runner now bypasses caching when the consumer's explicit session bound allows
at most one answer. This is known from the request, not predicted from workload
statistics. Sessions that can reuse a caller retain the measured cache mechanism.

The repair checks all 16 such scenarios across the three memo/control pairs:
192 allocation and 480 ordinary runs. Requested bytes, peak ownership and retained
consumer ownership are exactly equal in every pair. Runtime produces 47 uncertain
comparisons and one nominal gain. Both execute the same non-caching path; that
nominal gain is not attributed to the repair. This control also shows why the
five-block criterion alone cannot prove a small performance difference.

## Reuse saves allocation while retaining more state

Before the known-one-answer repair, reuse lowers requested bytes in 48/64 cases
for each ordering path and raises them in the other 16. Peak ownership rises in
44 projection cases and 56 cases for each enumeration strategy; it falls in eight
cases for each, and ties in twelve projection cases. The one-answer repair makes
its 16 affected pairs equal rather than paying for unused entries.

Default caller/source tests, all 23 relevant counter-free tests and scoped
allocator-build Clippy pass.

The initial matrix contains 896 allocation and 2240 ordinary runs; the repair adds
672 runs. Every run checks ordered answers outside timing, repeated allocation
agrees, held answers survive prepared-state disposal, and all measured allocations
are released. Timing uses counter-free engines and the ordinary allocator;
requested allocation uses a separate build and is not RSS. Compiler costs are not
included. No general cache eviction or retention policy is selected.

## Next: observations during an unfinished call

The four-package review selects live caller observers under T075. Completed-query
reuse now has benefits in both competing ordered paths. Whether similar reuse can
preserve intermediate facts, binding changes and competing consumption could
change where it belongs in a complete architecture. Implement those observations
in the trace-reuse experiment and compare actual execution; the current private
call interface is an implementation to change, not a language restriction.

Broader connected solving, cheaper ordering, whole-query caching and selective
retention remain unresolved. The [portfolio review](S06-caller-reuse-review.md)
explains their priority. The research goal remains active.

[Registration](../registrations/S06-caller-reuse.md) ·
[Timing](S06-caller-reuse-cost.json) · [Allocation](S06-caller-reuse-allocation.json) ·
[One-answer timing](S06-caller-reuse-once.json) · [One-answer allocation](S06-caller-reuse-once-allocation.json)
