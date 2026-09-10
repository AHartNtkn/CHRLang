# Resource-phase composition saves time on the larger tested batches

The composed resource phase is faster in all 16 larger/reused scenarios against all four whole-source controls. This includes output-heavy complete queries where it requests more heap bytes. The extra allocation therefore does not establish a runtime loss. Zero/small queries mostly favor whole-source execution in their medians, but remain uncertain under the registered criterion.

All **1,920 processes** completed with valid independent answers:240 warmups and1,680 measured runs. This is a bounded, counter-free lifecycle pilot on the qualified source fragment. Compilation, sustained lifetime and complete architecture selection remain open.

## What the pilot measures

The candidate solves the initial capacity phase, constructs ordered continuations and transports private bindings to an ordinary specialized scanned caller. Controls execute the whole source using generic scanned/indexed or inferred-specialized scanned/indexed execution. Six query regimes cross both consuming head orders and both callers, giving24 scenarios and120 configurations.

The endpoint sums disjoint source construction, preparation, changing-query setup, execution, observation, cancellation and disposal intervals. Consumers retain all delivered answers through prepared disposal. Compilation, process startup and independent validation are outside this endpoint. It is neither process latency nor isolated first-answer latency.

Each configuration has seven paired measured repetitions on each of logical CPUs0 and1, with randomized order and one warmup block. A gain requires a median candidate/control ratio at most0.90 and every pair below1 on both CPUs. A loss requires a median at least1.10 and every pair above1 on both CPUs. Everything else is unresolved. These are descriptive pilot criteria, not statistical confidence claims or independent hardware replications.

## The result by regime

Ratios below compare the phase plus caller with specialized whole-source scanning. Each range covers the two head orders, two callers and both CPU placements; lower is faster.

| Regime | Paired median ratio range | Registered result |
|---|---:|---|
| Zero requests, one query | 1.195–1.404 | All four scenarios unresolved |
| Two requests, tight supply, one query | 1.047–1.232 | All four scenarios unresolved |
| Four requests, tight supply, four queries | 0.555–0.619 | All four gain |
| Four requests, empty supply, four queries | 0.208–0.268 | All four gain |
| Four requests, spare supply, weight two, four complete queries | 0.690–0.820 | All four gain |
| Same output-heavy batch, first query cancelled after one answer | 0.486–0.571 | All four gain |

The four larger/reused regimes also gain against every other whole-source control. Across96 scenario/control contrasts there are64 gains,31 unresolved comparisons and one loss. The loss is zero-request, need-first, take-caller execution versus generic scanning, with paired median ratios1.350 and1.490. These counts describe this matrix; they do not weight workloads or identify a universal winner.

For a concrete complete output-heavy case, need-first consumption with later private work takes a pooled median **13.95ms** for the candidate and **18.92ms** for specialized scanning. CPU-specific paired ratios are0.745 and0.717. The same configuration's [separate allocation measurement](S06-capacity-phase-ownership.md) requests19.36MB versus17.73MB while reaching a2.18MB versus4.34MB live-heap peak. More requested allocation, lower live memory and lower elapsed time coexist here.

## What explains the contrast with allocation

The resumed caller interval is substantially shorter than whole-source execution in the output-heavy example: pooled phase medians are11.94ms versus16.96ms. Phase solving and continuation construction add1.20ms. Thus the candidate avoids enough execution time to repay its added phase work on this example. The intervals also differ in allocator and retained-state behavior; this measurement does not isolate an individual instruction or data structure as the cause. Medians of phase intervals need not sum to the median total.

The zero-request loss has a different shape. Against generic scanning, pooled preparation medians are36.48µs versus21.44µs, with another10.21µs for phase solving. Caller and ordinary execution intervals are similar at15.76µs and16.45µs. A phase with no private requests offers little source work to eliminate, yet still pays preparation and transport. This supports an overhead explanation for that bounded loss; it does not establish that every direct-solving implementation must incur the same overhead.

Cancellation reduces total batch cost under the tested endpoint, even though phase solving still eagerly constructs all bounded continuations for the first query. The remaining three queries complete on the same preparation. Do not interpret the batch result as lower first-answer latency, identical answer order, or interruptibility inside the solver.

## Why the small cases remain unresolved

Every specialized-scan small-case paired median favors the control, but several individual pairs reverse direction. In the zero-request, need-first, later-work case on CPU1, one control observation is512.73µs while that configuration’s pooled control median is74.90µs; that pair's ratio is0.179. The opposite candidate/control ordering also occurs without such a large outlier. All raw observations remain in the registered analysis.

No cutoff, invalid answer, mode mismatch or source/binary change accounts for these reversals. This pilot does not identify their machine-level cause. Short intervals, process-to-process variation and different phase-clock counts limit interpretation; no observation is discarded as noise and no loss is declared from medians alone.

Further small-case precision is not needed to decide the next investigation. The larger gains already justify retaining the composed phase as a candidate; the overhead and uncertainty already prevent universal adoption. No automatic routing rule or small-query threshold is being selected. If such a policy is proposed, these regimes require prospective confirmation with suitable resolution and a frozen policy. Their uncertainty remains explicit.

## Move next to adaptive separation and reunion

**Select T077's adaptive search entry next.** The current resource-phase question has credible selective and output-heavy opportunities, adverse overhead evidence and qualified ownership. Another local optimization or broader timing sweep would refine this fragment. Adaptive separation can instead change whether explicit search needs to retain shared state, and its existing repeated-reunion results expose a concrete problem: late links can incur many boundary checks without avoiding source steps.

The current implementation already exposes repeated separation, coupled boundaries, private/coupled work and inherited state in `research/chr-restoration/src/reunion.rs`. These provide an entry for deciding when to attempt separation using work already observed. The next package must specify and independently qualify an observable policy against eager repeated reunion, initial-phase reunion and ordinary execution. Include useful private work, frequent/late links, early failure, inherited history, cancellation and a finite answer beside ongoing work. Do not use future outcomes or source-generator labels to select behavior.

This comparison addresses adaptive separation attempts; it does not automatically answer delayed choice splitting, checkpoint intervals or every restoration policy. Its source/progress gate comes before primary cost claims. Count policy bookkeeping as operational work even when diagnostic counters are off.

The strongest alternative is sustained lifetime using existing composed and whole-source candidates: it could overturn finite-batch advantages through retention or consumer pressure. It needs an explicit stream/publication endpoint. Local resource claims still need a conflict/cancellation protocol, richer theories need independent denotations, and broader reuse needs dependencies beyond repeated private-query shapes. Those are required investigations, not rejected designs. The [concrete sequence](../next-cycle.md#concrete-entry-experiments-for-the-next-breadth-review) specifies their entry cases.

This is package three after the matched-reuse breadth review. The adaptive entry is the next package; conduct the full breadth review at that boundary, or sooner if an obstruction changes readiness. Reconsider sustained lifetime and the other distinct directions then. Broader resource-phase sources, general eligibility, compilation, coherent architectures and held-out challenges remain required. The research goal remains active.

## Evidence

- [Prospective registration](../registrations/S06-capacity-phase-costs.md), [launcher/analyzer](../../../research/chr-hvm/resource_capacity/phase_costs.py).
- [Pre-run manifest](s06-capacity-phase-costs/manifest.json), [all process receipts](s06-capacity-phase-costs/runs.jsonl), [paired ratios, ranges and phase analysis](s06-capacity-phase-costs/analysis.json).
- The frozen binary, source and allocation receipts match the [ownership qualification](S06-capacity-phase-ownership.md). Every timing process performs independent complete-answer validation outside measurement; the parent checks mathematical multiplicity counts, configuration, counter/allocator mode and exact phase order.
- Every process finished within the registered60-second wall/CPU and1GiB address-space limits; the1920-process launcher finished within600 seconds. No failed process or resource cutoff occurred. Reanalysis checks all hashes, run order and counts and reproduces the result.

The source fragment still has a finite initial capacity phase and an ordinary caller. This evidence establishes neither a general compiler nor a parallel ownership protocol. Its gains must participate in complete-path and sustained-lifetime comparisons before architecture selection.
