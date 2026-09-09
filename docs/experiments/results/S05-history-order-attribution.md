# Favorable history projection helps, but owned keys still dominate

**Consuming temporary resources before common work gives dead-history projection a real opportunity.** It reduces executed work and substantially improves the table's complete costs, yet this implementation loses to direct execution, specialization and demand graphs in all 32 registered configurations. Separate allocation attribution identifies key export as a consequential implementation cost.

The [prospective comparison](../registrations/S05-history-order-attribution.md) completes 2,044 lifecycle processes, 224 exact allocation replays, and two allocation-diagnostic processes with 90 identical cell payloads each. All complete independent raw answers, cancellation and query/prepared requested-live restoration checks pass. No observations were excluded.

## The source ordering matters

The `history-early` variant changes only the priority of the temporary-consumption rule, placing it before recursion. The former `history` source remains as a control with live temporary occurrences during the common countdown. Both sources have independently checked complete outcomes under all seven engines; this is an experimental source contrast, not a proposal to reorder arbitrary CHR rules.

For depth64 with a consuming finish, Direct, ExactIds and Alpha each execute 565 logical source steps in both orderings. AlphaLive executes 550 in the late-consumption source but 160 in the early-consumption source. The latter replays 405 transitions. Thus early consumption exposes reusable complete states; renaming alone still retains distinct dead history and misses this opportunity.

## Complete paired costs

The table uses seven paired blocks and the prospectively fixed 10% practical threshold. These are pointwise classifications over selected source configurations, without workload weights or population-wide claims.

| Numerator / denominator | Gains | Losses | Unresolved |
|---|---:|---:|---:|
| live/alpha | 18 | 0 | 14 |
| live/exact | 12 | 7 | 13 |
| live/direct | 0 | 32 | 0 |
| live/sealed | 0 | 32 | 0 |
| live/dependencies | 0 | 32 | 0 |
| sealed/direct | 3 | 16 | 13 |

At depth64/four queries/resources/forward, median AlphaLive total is 27.322 ms for late consumption and 5.914 ms for early consumption. Early-consumption direct execution is 0.912 ms, specialization 0.793 ms, and demand graph execution 3.332 ms. The policy contrast makes the mechanism useful without making this table representation competitive.

Early AlphaLive requests 4,939,724 primary bytes with peak requested growth 734,477 bytes. Early Direct requests 1,703,500 with peak growth 46,570; specialization requests 1,252,311 with peak growth 88,200. Lower executed work does not remove key construction, lookup, replay or retained-state responsibilities.

## Allocation attribution isolates construction work

A separate diagnostic traverses direct-execution cursors, builds and drops a key at every visited state, and measures requested bytes for key export, canonicalization and source stepping separately. It does not intern keys or replay work, and it does not include queue operations in the source-step component. Its totals must not be substituted for the sharing engine's lifecycle totals.

| Depth64/resources source | Key export | Alpha canonicalization | AlphaLive canonicalization | Source steps |
|---|---:|---:|---:|---:|
| Renamed futures | 2,956,479 | 1,636,640 | 366,816 | 383,778 |
| Early history | 3,160,415 | 1,711,328 | 382,992 | 390,524 |

The exact key has zero canonicalization allocation but pays the same export cost. Both diagnostic repetitions agree exactly. These figures substantiate that repeatedly materializing keys is expensive; they do not establish an asymptotic law or show that every exported field can be omitted.

`State::key` currently exports owned syntax for pending terms, live arguments and outputs, and copies history before canonicalization. Long closed constructor spines are expanded again at successive states. The underlying arena already interns constructor nodes and records immutable structural closedness. Using a machine-local canonical handle for a closed term can potentially avoid those copies without changing its equality meaning. Nonground terms, aliases, bindings, occurrence order and live history still require explicit treatment.

## Next decision

Test a key representation that preserves closed terms by canonical arena identity, with independent equivalence checks against the owned key and explicit machine ownership. Do not use process addresses as portable keys or assume an open node becomes permanently closed after binding. Investigate exporting only relevant history where sound, but keep attribution separate from the closed-term representation change. Retain current exact, Alpha, AlphaLive, direct, specialized and graph controls.

This is a concrete representation alternative supported by measured construction cost, not a reason to tune the table indefinitely. The next gate must show that the key equivalence and reuse opportunities are preserved; only then register paired costs. Review breadth after that gate against sustained S08 consumers and remaining S02 integration. Call-level reuse, broader relevance projection, eviction and whole-architecture composition remain required.

Primary timings include preparation, changed-query setup, complete raw observation, and engine/answer/prepared disposal. Source/input-inclusive and first-observation costs remain in raw receipts. Requested allocations are not RSS; native compilation is excluded. Reference-interpreter code is unchanged.

The [freeze](s05-history-order-attribution/freeze.json), [full paired results](s05-history-order-attribution/summary.json), [work diagnostics](s05-history-order-attribution/work.log), allocation receipts and [audit](s05-history-order-attribution/audit.json) retain provenance. The expanded runner source gate checks 420 complete combinations plus cancellation, and scoped strict Clippy passes. T075 and the architecture goal remain active.
