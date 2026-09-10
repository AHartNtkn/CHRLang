# Reusable diagrams reduce some allocation costs, but simple source elimination is a stronger missing control

Reusable diagrams request fewer bytes than prepared name solving in 303 of 432 matched scenarios, and more than explicit early-rejection enumeration in 345. Preparation can dominate even when the final visible relation is trivial. Before primary timing, add a source-derived simplification control and charge its analysis cost.

All 4,320 metered and 2,160 ordinary processes preserve independently checked answers. Paired allocation phases match exactly, retained answers survive producer disposal, and final measured heap returns to its initial baseline in every configuration. These are allocation and correctness findings, not elapsed-time rankings.

## What is included

The comparison crosses five paths with six source families, three/six variables, two/three names, one/16/128 changing queries, membership/full-output endpoints, and immediate/window/retained-all consumers. Windowed consumers retain at most four answers. The 432 matched scenarios are experimental coverage, not workload frequencies.

Each preparation serves a cycle of unrestricted, fixed-name, conflicting and outside-alphabet requests. Odd queries also require equal visible coordinates. The preceding source gate qualifies the three-variable grammar; independent complete-assignment enumeration validates all six-variable endpoints here. Only the projected logical-set interface is compared.

Measured phases include candidate preparation, request construction, symbolic fresh transport, exact observation, query disposal, consumer actions, identity-owner disposal and preparation disposal. Diagram preparation includes branch compilation, union, projection and their intermediate allocations. Source/oracle fixtures and fixed-capacity report/consumer containers are outside the accounting baseline. There are no clocks; the ordinary build confirms semantics with the allocation meter absent. Synchronous completion is tested, not interruptible cancellation or streaming latency.

## Allocation traffic and peak heap give different answers

| Diagram relative to control | Requested bytes: lower / higher | Peak live excess: lower / higher |
|---|---:|---:|
| Prepared names | 303 / 129 | 157 / 275 |
| Finite projection | 432 / 0 | 379 / 53 |
| Symbolic rebuilding | 417 / 15 | 369 / 63 |
| Explicit early rejection | 87 / 345 | 75 / 357 |

Against prepared names, diagrams request fewer bytes in 72 of 144 one-query scenarios, 96 of 144 sixteen-query scenarios and 135 of 144 128-query scenarios. Reuse can repay compilation in allocation traffic while preparation still sets a larger peak. Against explicit enumeration, only 36 of the 144 long-reuse scenarios request fewer bytes.

These counts cannot select an architecture. Peak heap and requested traffic are separate costs, and neither measures RSS or CPU time. The controls also perform different amounts of solving under the same observation contract.

## A preparation-heavy case changes the next experiment

For six variables, three names, hidden-star exclusions, 128 membership queries and immediate release:

| Path | Total requested bytes | Preparation requests | Peak live excess |
|---|---:|---:|---:|
| Diagram | 57,787 | 48,216 | 36,008 |
| Prepared names | 32,787 | 1,456 | 1,208 |
| Finite projection | 383,010 | 266,678 | 14,830 |
| Symbolic rebuilding | 1,566,089 | 1,478 | 33,502 |
| Explicit early rejection | 6,793 | 230 | 320 |

The diagram retains only 64 measured bytes after preparation in this case: its two terminal nodes. The source relation is universal after hiding the other variables. Each hidden name must differ from X and Y; three names always provide a choice, independently for every hidden variable. Expensive general compilation has discovered a fact that simpler source reasoning can establish.

This is not just an allocator question. The symbolic control still retains 768 identity records over the 128 queries and requests 1,258,264 transport bytes. Diagrams avoid that identity history under this finite ground interface, but their own compilation can outweigh the saving relative to stronger controls.

A favorable diagram allocation example also survives: the six-variable, three-name star/clique union with 128 full-output queries and retained-all consumers requests 175,075 bytes, versus 329,043 for prepared names and 7,081,375 for symbolic rebuilding. Explicit enumeration still requests less, at 95,857 bytes. No runtime conclusion follows from either example.

## The selected sources admit much simpler visible relations

A [registered analytical follow-up](../registrations/S06-diagram-simplification-check.md) checks all 24 family/size/alphabet combinations against independent complete-assignment enumeration. All 156 visible-pair comparisons agree. The visible relations reduce to 16 universal relations, four equal-coordinate relations, one different-coordinate relation and three contradictions.

The reasons are source properties. Free alternatives impose no condition. An overlapping branch can choose its hidden endpoint differently from X. A two-name hidden star requires X=Y, while three names make it universal. A clique with more variables than names is impossible; the selected three-name/three-variable clique leaves X!=Y. A clique implies the star constraints, so their union has the star's meaning.

These are analytical simplifications with finite confirmation, not measured compiler results. They expose a limitation of this cost matrix: its apparent graph-solving work can be eliminated by stronger preparation. A timing comparison without that control could reward a reusable solver for work that need not remain at runtime.

## Next decision and remaining work

Qualify an executable source-derived control before primary timing. It must inspect constraints and justify simplification, rather than dispatch on fixture family names or use oracle answers. Candidate mechanisms include eliminating hidden vertices with fewer neighbors than available names, two-name parity reasoning and clique contradiction certificates. Charge that detection and preparation. Preserve unresolved graph cores for an explicit eligibility result; do not pretend the selected simplifications solve arbitrary finite graphs.

Then compare complete costs on both these sources and consequential sources that resist the simplifications. The latter must be selected by what architectural question they expose, rather than merely making a diagram look favorable. Cheap one-shot work, broad output and changing queries remain adverse controls.

This completes package three of the current T076 investigation. The next package triggers the full breadth review against relevant-read repair, demand attribution/capability and checkpoint/replay. Clock qualification, primary timing, sustained ownership, broader theories and complete architectures remain required. The research goal remains active.

## Evidence

[Ownership registration](../registrations/S06-diagram-ownership.md), [harness](../../../research/chr-structural/examples/diagram_ownership.rs), [driver](../../../research/chr-structural/experiments/diagram_ownership.py), [independent allocation auditor](../../../research/chr-structural/experiments/diagram_ownership_audit.py), [source/binary freeze](s06-diagram-ownership/freeze.json), [exact configurations](s06-diagram-ownership/configurations.json), [raw receipts](s06-diagram-ownership/runs/), [per-configuration summaries](s06-diagram-ownership/summary.json), [audited contrasts](s06-diagram-ownership/review-audit.json), [simplification checker](../../../research/chr-structural/experiments/diagram_simplification_check.py) and [simplification results](s06-diagram-ownership/simplification-check.json).

Both allocator configurations pass scoped Clippy. The existing diagram and source-correspondence regressions pass. The isolated process bounds are 60 seconds wall/CPU and 1 GiB address space; no process reaches them. The driver completes the campaign in about 27 seconds. That campaign duration includes harness/oracle/process work and is not a comparative performance endpoint.
