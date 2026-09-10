# Source simplification reduces repeated allocation, while explicit execution retains peak-memory advantages

The constraint-based simplifier requests fewer bytes and reaches lower peak heap than prepared names, finite projection and symbolic rebuilding in all 432 matched scenarios. Against explicit early-rejection enumeration, it requests fewer bytes in 264 scenarios but reaches higher peaks in 317. Preparation and reuse still matter; this does not establish a runtime winner.

All 5,184 metered and 2,592 ordinary processes preserve independent complete answers. Paired allocation phases match exactly, retained answers survive producer disposal, and every final measured heap returns to baseline. The five original controls reproduce their previous normalized summaries exactly, isolating the new control's contribution.

## The comparison includes the analysis cost

The new path calls the qualified graph simplifier on actual constraints. It checks that all hidden obligations were eliminated before serving direct visible queries. It does not select answers from workload names or oracle results. Graph construction, parity/component analysis, elimination, retained constraints and disposal are included in preparation and final ownership phases.

The same six families, variable/alphabet sizes, one/16/128-query streams, membership/full-output endpoints and immediate/window/all consumers are retained. Request construction, transport where applicable, exact output and consumer actions remain charged. Source/oracle fixtures and fixed-capacity reporting/consumer slots remain outside the baseline. No clocks run in either allocator configuration.

The 2,592 configurations each run twice with allocation instrumentation and once with the ordinary allocator. Independent enumeration checks complete answers outside measured phases. The raw phase audit verifies continuous live ownership, not just a final zero balance. There are no cutoffs under the registered 60-second process and 1-GiB address-space limits.

## Stronger controls change the allocation comparison

| Simplifier relative to control | Requested bytes: lower / higher | Peak excess: lower / equal / higher |
|---|---:|---:|
| Diagram | 408 / 24 | 294 / 43 / 95 |
| Prepared names | 432 / 0 | 432 / 0 / 0 |
| Finite projection | 432 / 0 | 432 / 0 / 0 |
| Symbolic rebuilding | 432 / 0 | 432 / 0 / 0 |
| Explicit early rejection | 264 / 168 | 115 / 0 / 317 |

The simplifier's 24 traffic losses against diagrams all occur in one-query scenarios. Against explicit enumeration it requests fewer bytes in 39 of 144 one-query scenarios, 87 of 144 sixteen-query scenarios and 138 of 144 128-query scenarios. These are coverage counts, not workload weights or a threshold for real applications.

## Preparation savings are now measured

For six variables, three names, hidden-star exclusions, 128 membership queries and immediate release:

| Path | Session requested bytes | Preparation requests | Peak live excess |
|---|---:|---:|---:|
| Simplifier | 4,385 | 958 | 864 |
| Diagram | 57,787 | 48,216 | 36,008 |
| Prepared names | 32,787 | 1,456 | 1,208 |
| Explicit early rejection | 6,793 | 230 | 320 |

The simplifier establishes that the visible relation is universal without constructing a general decision diagram. Membership observation requests no heap allocation; the remaining session traffic includes changing requests and preparation. Explicit enumeration still has the smaller peak and cheaper preparation, despite greater repeated traffic.

The small adverse case remains visible. A single membership query on the free three-variable/two-name source requests 187 bytes for simplification, 160 for the diagram and 124 for explicit enumeration. Avoiding later solving need not repay analysis for a trivial one-shot request.

Full output also carries unavoidable consumer ownership. The six-variable/three-name star-clique union with 128 full-output queries and retained-all answers requests 67,201 bytes with simplification, versus 175,075 for diagrams, 329,043 for prepared names and 95,857 for explicit enumeration. Its simplifier peak is 48,098 bytes, close to explicit enumeration's 48,456: retained output dominates this case more than the prepared relation does.

## What this supports—and what remains unanswered

This supplies the missing measured preparation control. It shows that some allocation savings attributed to reusable formulas can instead come from eliminating source constraints. It also preserves a real traffic-versus-peak tradeoff with explicit search. Requested heap bytes are not RSS, elapsed time or energy, and zero-allocation membership still performs validation and relation checks.

The sources are precisely those admitted by the finite logical-set protocol. The preceding source tests and unresolved K3,3 witness remain the eligibility evidence. The current control does not solve every graph, preserve arbitrary residual effects, or handle broader structural theories by this method. Synchronous complete observations do not establish interruptible cancellation or streaming progress.

Next qualify clocks and diagnostic-counter removal for the matched runner, then prospectively register primary timing with adequate signal and complete lifecycle accounting. Tiny phases may be dominated by measurement overhead; qualification must determine which timing endpoints are credible before interpreting ratios. Preserve operational work bounds and validate complete answers outside measured intervals.

This completes qualification package one under the [bounded breadth-review decision](S06-graph-simplification-breadth-review.md). Clock qualification is the remaining selected qualification package, followed by timing confirmation and the T077 restoration handoff. No further graph-rule expansion is selected. The goal remains active.

## Evidence

[Registration](../registrations/S06-simplified-ownership.md), [integrated harness](../../../research/chr-structural/examples/diagram_ownership.rs), [driver](../../../research/chr-structural/experiments/simplified_ownership.py), [independent auditor](../../../research/chr-structural/experiments/simplified_ownership_audit.py), [frozen sources/binaries](s06-simplified-ownership/freeze.json), [all raw runs](s06-simplified-ownership/runs/), [matched summaries](s06-simplified-ownership/summary.json), [audited contrasts](s06-simplified-ownership/review-audit.json), [meter-build Clippy](s06-simplified-ownership/clippy-meter.log) and [ordinary-build Clippy](s06-simplified-ownership/clippy-ordinary.log).
