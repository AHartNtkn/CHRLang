# Fork allocation attribution before snapshot selection

T050 shows material split-producing traffic but cannot identify which copied structures account for it. T051 will attribute actual owner clones and subsequent interning behavior before selecting a snapshot intervention. This design preserves ordinary source scheduling and introduces no shared mutable arena or new restoration policy.

A diagnostic-only feature in compiled may forward a narrow feature to persistent. Allocator accounting remains owned by compiled. Persistent exposes a generic diagnostic clone observer, so actual Arena field cloning can be measured without depending on compiled or defining another allocator. Ordinary builds retain their existing clone path with observer hooks and diagnostic fields absent.

Measure the arena's node vector, closedness metadata, interning table, predicate vector and predicate lookup separately. Measure Core's remaining actual owned containers and Engine's trace/audit/search data in their existing evaluation order. Prepared Arc and binding persistent-tree clones remain controls, including zero-request outcomes. Arena/Core totals are sums, not additional owner intervals that double-count their fields. Fixed-size records must avoid diagnostic allocations inside owner measurements.

The existing meter's `begin` resets global peak state. It must not be called inside a split interval. Add read-only cumulative checkpoints for allocation-call/byte/free deltas; they do not reset peak or allocate. Owner intervals are mutually disjoint. Their traffic sums must reconcile against inclusive split-producing service, with an explicit remainder for source stepping, lineage, frontier and pending-arm work. Absolute per-owner peak is unavailable under this method and must not be reported. Timing remains diagnostic attribution, never a primary speedup prediction.

After each actual fork, mark both surviving arenas with their inherited node prefix. Count actual subsequent interning requests as inherited hits, branch-local hits or misses. Preparation is outside this count; operations performed in branches that later fail are included. Count operations once rather than copying cumulative diagnostic prefixes with branch state. A node created before a later fork becomes inherited at that fork; the boundary is the latest snapshot, not the query's original arena.

Before a diagnostic run, validate complete observations, failures and source work against the independent state-preservation gate. Check allocation reconciliation, zero-allocation shared-owner controls, sibling independence and all three interning classifications. Verify that feature-off builds contain no extra diagnostic state or hooks. Register exact diagnostic cells and resource bounds before collecting comparison evidence.

If arena cloning is material and subsequent requests mostly hit inherited nodes, lookup-before-detachment arena copy-on-write becomes a concrete candidate. If immediate misses force copying, or other owner groups dominate, that candidate may not earn its complexity. Whole-state copy-on-write can simply postpone copying because continuations mutate occurrence/index state; trailing and replay retain different restoration and scheduling obligations. No candidate is selected solely from source inspection.

Independent measurement review supports this dependency direction and checkpoint design. Implementation and measured owner results remain outstanding. The closed-subtree occurs correction is a separate common construction-cost correction; it does not explain the additional copied bytes found by T050.

## Implemented measurement gate

The `fork-diagnostics` feature now records 23 actual owner windows. The allocator
stays in the compiled experiment harness; persistent arenas expose generic clone
and segment callbacks. Fixed records use read-only cumulative checkpoints,
including a self-check that peak state survives an inner checkpoint. Ordinary
builds have neither diagnostic arena fields nor observer calls.

Both children start a segment at each actual fork. Endpoints distinguish split,
failure and completion, with inherited node totals, mutation-free totals, first
miss totals, predicate insertions and the number of requests before first miss.
The last measure is necessary because node count at first miss alone equals the
inherited count in an append-only arena. Interning counters drain after each
service step and include failed work. Active segments at a cutoff are partial;
the registered experiment requires completion before interpreting classifications.

Independent review found no blocking owner-window or segment-accounting issue.
Semantic tests exercise failed work, sibling divergence, preparation reuse, local
and inherited hits, misses and predicate insertion. The root auditor checks
owner/remainder reconciliation, exact continuation conservation, complete answers,
shared-handle controls, command correspondence and full non-time repetition.
Adverse smoke checks reject accounting corruption and retain malformed results.
Receipts are in [r03-fork-owner-gate](r03-fork-owner-gate/).
The prospective matrix is [registered here](../registrations/R03-fork-owner.md).
