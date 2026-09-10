# Projection pays for some reused hidden work; streaming wins on dense output

Finite projection has a useful measured regime, but the strongest control depends on the work required. Leaf-first elimination beats streaming enumeration on the larger reused connected weighted request. A much simpler separability control handles independent hidden choices more cheaply, while dense output and early cancellation favor streaming.

These are exploratory results for explicitly defined finite endpoints. They do not select a CHR architecture, establish source-ordered derivation equivalence, or choose a general elimination policy. The next selected investigation is adaptive search ownership and costs, under T077.

## The comparison gave each mechanism a real opportunity

The pilot uses three families at four and ten Boolean coordinates: independent hidden choices, connected star constraints, and dense visible output. It crosses one/four queries per preparation, weighted/expanded delivery, and exhaustion/cancellation after eight records. Changing queries restrict the first visible coordinate to zero, then one, before returning to an unrestricted query.

Three projection orders compete with streaming backtracking, a separability control and the existing finite structural solver. Backtracking assigns visible coordinates first and tests each relation as soon as its final coordinate is assigned. The separability control verifies that there are no relational filters, multiplies hidden choice counts and enumerates only visible values. It explicitly excludes the star. The structural solver uses its existing grammar and search implementation; preparation, full tuple construction, projection and output-map collection are charged.

All 272 applicable cells have five ordinary-allocator, metrics-off release samples and two separate allocation samples: **1,904 completed processes**. Every process independently validates the complete ordered record sequence before timing; the audit separately reconstructs all expected measured full/prefix digests. Allocation pairs agree exactly and every measured lifecycle returns live heap to baseline. No resource cutoff occurred.

## Connected hidden work can repay elimination and preparation reuse

At ten coordinates and four weighted queries, putting hidden leaves before the center reduces the median total lifecycle from **87.54 μs for enumeration to 35.58 μs**. The five-sample ranges are 83.79–134.05 μs and 31.60–38.46 μs respectively. This satisfies the registered exploratory separation rule, even after preparation and disposal are charged.

The scope-greedy policy also separates favorably on that request, at 55.75 μs median and 51.37–61.70 μs range. It finds the small-intermediate strategy automatically, but its planning cost matters. The fixed leaf-first order's preparation median is 27.38 μs; greedy preparation is 47.00 μs. A favorable hand-chosen order is a control, not a general policy that has earned adoption.

Center-first elimination takes 783.29 μs median on the same request. Its preparation accounts for almost all the time. The comparison therefore exercises both a useful mechanism and a consequential ordering failure; it does not attribute that failure to every projection organization.

Reuse changes the conclusion. With one weighted query at this size, leaf-first and enumeration ranges overlap: 30.36–39.48 μs versus 32.40–47.26 μs. Four-query evidence cannot establish the exact reuse threshold or guarantee a single-query gain. First delivery is another tradeoff: on the four-query request, median time from source construction to first record is 32.33 μs for leaf-first projection and 14.74 μs for enumeration.

Projection also buys speed with more allocation here. Leaf-first requests 41,282 bytes over the four-query lifecycle, against enumeration's 11,518 bytes. Its maximum observed live heap above baseline is 14,360 bytes versus 5,200 bytes. More allocation did not imply slower complete execution, and the timing gain did not imply lower memory cost.

## Independent choices do not justify general factor machinery

For ten independent coordinates and four weighted queries, ascending projection takes 19.13 μs median versus enumeration's 133.07 μs. That is real work elimination. But the separability control takes **5.33 μs**, with a 3.70–5.81 μs range, and requests 832 bytes versus projection's 30,300 bytes.

The architectural consequence is that recognizing independence can be more valuable than choosing between two ways to enumerate it. Projection's gain over ordinary backtracking alone would overstate the need for a general solver. Across the 32 applicable scenarios, no projection order has a qualified pilot gain against separability; most separate as losses and the remaining comparisons are unresolved.

This control's admission rule is deliberately explicit: no relational filters. It does not demonstrate economical inference of general CHR independence, privacy or effect freedom. Nor does it settle connected solving by silently selecting another engine for that family.

## Dense output and cancellation expose materialization costs

All ten coordinates are visible in the dense family, so there is no hidden product to eliminate. For four fully expanded queries, greedy projection takes 1,580.14 μs median versus enumeration's 197.87 μs. The ranges are 889.56–1,774.84 μs and 163.56–233.81 μs; this is a qualified exploratory loss despite substantial timing variability.

Cancellation makes the contrast sharper. After eight expanded records per query, greedy projection still takes **990.52 μs** median, whereas enumeration takes **6.93 μs**. Projection's request allocation remains 316,156 bytes with or without cancellation: it has already materialized each weighted map before the first record. Enumeration requests 5,084 bytes when cancelled, compared with 35,484 bytes on full delivery.

Median first delivery including preparation is 324.33 μs for cancelled greedy projection and 3.29 μs for enumeration. This follows the ownership gate's warning: a small iterator does not make its preceding materialization incremental. A future lazy projection endpoint could change that cost, but has not been measured by this implementation.

## Summary of exploratory separations

A “gain” below means every one of five lifecycle samples was at least 10% faster than every corresponding control sample; “loss” is the converse. This is the preregistered screening rule, not a confidence interval. Each count is a source/request cell, not a workload weight. Weighted cancellation cells with fewer than eight records repeat the full endpoint; they are not additional independent regimes.

| Projection order | Against enumeration: gains / losses / unresolved (48 cells) | Against separability: gains / losses / unresolved (32 cells) | Against existing structural path (48 cells) |
|---|---:|---:|---:|
| Ascending | 5 / 37 / 6 | 0 / 30 / 2 | 48 gains |
| Descending | 6 / 32 / 10 | 0 / 31 / 1 | 48 gains |
| Greedy | 4 / 35 / 9 | 0 / 30 / 2 | 48 gains |

The existing structural path loses these comparisons while remaining semantically correct. For the ten-coordinate reused connected weighted request, it takes 8,920.54 μs median and requests 16,466,667 bytes. Its charged path constructs complete terms, maintains search/deduplication state and then collects projected output. These results concern that adapter and algorithm on these finite sources. They are not evidence against all compact structural solving or richer theories.

## Complexity and language consequences

| Organization | Responsibilities exercised in this pilot | Capability boundary |
|---|---|---|
| Backtracking | Assignment cursor, reversible traversal, relation-readiness plan and optional grouping of hidden witnesses | Enumerates hidden assignments, but can stream expanded output and stop early |
| General projection | Factor tables, scope elimination, checked weights, order selection and complete visible-map production | Eliminates hidden work; intermediate width and materialization can dominate |
| Separability control | Check absence of relational filters, multiply hidden counts, enumerate visible values | Much less machinery for admitted inputs; excludes coupled relations |
| Existing structural path | Grammar preparation, constructor requirements, search, exact term deduplication and projected observation | Supports structural obligations beyond these Boolean requests; this path still constructs full hidden terms |

These are responsibilities and restrictions, not a complexity score. No source-line count or weighted average selects an architecture. The measured endpoints deliver counted finite tuples or expanded multisets in tuple order. They do not reconstruct hidden provenance or arbitrary CHR source order, and they assume declared visible coordinates rather than proving that hidden identities cannot escape.

The lifecycle includes source construction, candidate preparation, source disposal, query start, first and remaining observations, query disposal and preparation disposal. Immediate consumer release is charged during observation. Some query-start phases include all weighted computation and materialization; the report does not treat that interval as cheap setup. Processes preflight before measuring, so these are warmed in-process results. Operational service/output limits remain enabled; diagnostic engine counters and allocation instrumentation are absent from primary builds. Compilation, process startup, artifact costs, RSS and ongoing-source lifetime are not established by this pilot.

## Next investigate adaptive search costs, keeping projection's open questions required

Select T077 adaptive search ownership qualification next. Its existing source gate demonstrates both reduced checking and changed execution opportunities; full costs could decide whether economical explicit search preserves common work without a shared graph. It is the strongest ready distinct alternative and requires less new semantic implementation than extending native local ownership or general caller relevance.

The strongest immediate projection follow-up is a cheaper input-derived ordering policy or an incremental visible-output endpoint. Both could change bounded costs. However, this pilot already isolates a useful connected/reused regime, a credible simpler independence control, and an output-materialization loss. Further precision on these same stars would not establish broader source eligibility or overturn those distinctions. Additional projection work should challenge different graph shapes, source correspondence and lazy output rather than merely refine the favorable fixed order.

T076 remains required for those questions, arbitrary structural paths and the separate names/disequality/normal-neutral theories. Native local ownership, conditional equality/lifetime repair and broader reuse also remain required. The comparator qualification and timing pilot count separately, completing the fourth package after the descriptor breadth review. The [full breadth review](S06-projection-breadth-review.md) selects adaptive ownership next and preserves every unresolved direction. That qualification begins a new cycle. The architecture research goal remains active.

## Reproducible evidence

[Prospective registration](../registrations/S06-projection-cost-pilot.md), [control implementations](../../../research/chr-structural/examples/support/projection_cost.rs), [independent control tests](../../../research/chr-structural/tests/projection_cost_controls.rs), [phase runner](../../../research/chr-structural/examples/projection_cost.rs), [bounded driver](../../../research/chr-structural/experiments/projection_cost_pilot.py), [independent audit](../../../research/chr-structural/experiments/audit_projection_cost.py), and [qualified raw evidence](s06-projection-cost-pilot/qualified/) contain the freezes, exact schedule and all observations. The [audit data](s06-projection-cost-pilot/qualified/audit.json) include each cell's ranges, medians, phase costs, requested allocation and comparison status.

The bounded control suite passes both tests, covering 272 complete source/query/endpoint combinations and their prefixes. All 17 sizing processes completed below the registered ten-second sizing ceiling; all 1,904 pilot processes satisfy the 60-second CPU/wall and 1 GiB address-space limits. The audit verifies source/binary hashes, schedule completeness, independently reconstructed observed digests and exact allocation pairs. Primary and metered Clippy pass; formatting passes. The reference interpreter and existing structural solver are unchanged.

The initial qualification receipt and driver snapshots are retained and hash-checked. The [qualification note](s06-projection-cost-pilot/qualification-note.md) records the expected-panic capture issue and its harness correction before sizing or comparative runs. No semantic assertion or experimental cell was weakened.
