# Fusion exposes a costly interaction with eager matching

Fusion saves interpreter applications but can greatly increase candidate discovery in contextual and conditional execution. The first whole-path pilot therefore challenges combining component optimizations without qualifying their interaction. It does not reject either architecture: the discovery cost may be avoidable.

The [registered pilot](../registrations/S10-composition-lifecycle.md) completes 2520 isolated processes across 360 cells: 720 allocation processes with 360 exact pairs, followed by 1800 ordinary-allocator timing processes. All complete answers agree with the independent scalar; all measured query, cancellation, prepared and inference owners restore. The [independent audit](s10-composition-lifecycle/audit.log) verifies exact phase order, allocation replay, ownership and frozen inputs. [Every cell](s10-composition-lifecycle/summary.csv), [raw results and freeze](s10-composition-lifecycle/freeze.json).

## What the full path costs in this pilot

The figures below cover three deterministic firings per query, four changing queries per preparation, and one additional cancellation probe. Traffic is requested allocation, not resident memory. Peak is requested live memory above the pre-inference baseline. Both include inference, certification, preparation, inputs, answer delivery and disposal.

| Execution | Original traffic, bytes | Fused traffic, bytes | Original peak, bytes | Fused peak, bytes |
|---|---:|---:|---:|---:|
| Compiled Scan | 103815 | 104281 | 12133 | 14030 |
| Ordinary contextual | 409426 | 2290633 | 22046 | 118586 |
| Persistent shared contextual | 414342 | 2295549 | 21966 | 118506 |
| Direct conditional | 912449 | 3444260 | 27360 | 85738 |

Compiled Scan's delivery traffic falls from 47336 to 37096 bytes, but inference and certification offset that saving. Contextual delivery traffic instead rises from 358536 to 1857552 bytes; its cancellation service operation also rises from 12370 to 383040 bytes. Conditional delivery traffic rises from 848300 to 3367996 bytes. These phase readings locate the increase in runtime work, rather than source inference alone.

Choice and shared-alias cases preserve this contrary effect. At the same count/reuse, independent-choice traffic is 371199→359276 bytes for Scan, 841494→3236164 for contextual, and 1284993→3826447 for conditional. Shared-payload traffic is 232751→228380, 682806→3095604 and 1205841→3755359 respectively. The full matrix also includes equal alternatives, spare resources, zero work, single firings, Indexed and inferred-specialized controls.

There is no uniform ordering even of preparation overhead. With zero firings and one query, ordinary contextual traffic is 5334 bytes versus Scan's 14230 and conditional's 14099. Adding fusion increases all three. Workload weights cannot be inferred from these examples.

## Why fusion creates this cost

The source transformation combines a two-head producer with a consumer that needs two permits. The emitted rule has four consuming heads. With n seeds, n fuel occurrences and 2n indistinguishable permits, there are n²(2n)(2n−1) distinct initial resource tuples for that rule. Resource occurrences remain distinct even when their values agree.

The [registered attribution probe](../registrations/S10-fusion-discovery-attribution.md) independently checks this formula against the contextual store. At n=3 it finds 270 fused tuples, compared with nine original producer tuples and no initially enabled consumer. It then runs the conditional engine with diagnostics enabled, independently checking its answers. Conditional discovered tuples change from 99 to 270 at n=3, but from three to two at n=1. [Probe output](s10-composition-lifecycle/discovery.log), [probe source](../../../research/chr-direct-conditional/examples/composition_discovery.rs).

The contextual store currently enumerates all matches into a collection before source execution selects one. Conditional execution discovers tuples and registers their support-sensitive obligations. Those are actual implementation responsibilities. The tuple count is not an unavoidable language cost: a committed computation may consume a resource before many competing tuples ever need representation. The probe supports this mechanism as a cause, but does not attribute every allocated byte to it. A controlled discovery implementation comparison is the next necessary test.

## What the timings can and cannot establish

Five ordinary samples per cell remain exploratory. For the deterministic count-three/reuse-four cell, Scan's original median is 134522 ns (130869–143365), versus fused 128470 ns (73989–140283). Contextual original is 231277 ns (229719–254014), versus fused 929451 ns (580233–958268). Conditional original is 675418 ns (541644–988190), versus fused 2114807 ns (2088844–2728001).

These are complete measured phase sums, including one cancellation probe. They do not establish isolated execution speed or formal practical-win classifications. The public APIs deliver answers at different internal boundaries, so the runner measures first delivery/exhaustion and remaining delivery as execution plus observation. Every candidate exports owned answers as they arrive and retains them through finite query completion. This differs from the earlier compiled-only pilot that retained completed engines before exporting; compare modes within this matrix.

Runtime preparation is charged; native compilation, process startup, scalar validation and source construction before inference are not. All measured queries qualify for fusion. The prior gate checks rejected-query semantics, but switching and dual prepared-artifact retention still need costs. Sustained consumers, early failure, broader mixed histories/guards and held-out sources remain required.

## Next decision

Keep T078 active and investigate demand-sensitive discovery before using the costly fused configurations to judge contextual or conditional architecture. The first implementation comparison must preserve guards, consumption, ordered occurrence history, source choices and finite sibling service. It must also test cases where eager retention helps, so a local repair cannot silently assume one source distribution.

The strongest ready alternative is broader mixed-source lifecycle measurement. Discovery attribution takes priority because it can invalidate a current competitor's cost profile and exposes a concrete interaction between compilation and runtime organization. Limit the next package to an independently qualified discovery comparison, then reconsider breadth. Do not require this mechanism to win before continuing the complete-architecture investigation.

The [runner](../../../research/chr-direct-conditional/examples/composition_cost.rs), [driver](../../../research/chr-direct-conditional/experiments/composition_lifecycle.py) and [auditor](../../../research/chr-direct-conditional/experiments/audit_composition_lifecycle.py) are retained. Strict Clippy passes for [ordinary timing](s10-composition-lifecycle/clippy-time.log), [allocation](s10-composition-lifecycle/clippy-meter.log) and [discovery diagnostics](s10-composition-lifecycle/clippy-discovery.log). Primary matrix binaries and source hashes remain unchanged after the separate diagnostic probe.
