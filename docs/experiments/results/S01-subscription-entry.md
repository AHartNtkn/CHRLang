# Next matching question: retain only the joins that live demands can use

Investigate demand-driven matching subscriptions against eager retention and competent indexed rediscovery. The decision is whether intermediate matches should be materialized for every possible request, maintained only for active demands, or recomputed. This is distinct from choosing a faster equation cache.

## What earlier matching already tested

The current `selective_join` executor stores left/right pairs awaiting a request and maintains incidence lists to invalidate consumed partners. Those pairs are already a partial materialization of the complete source rule. Calling the same table a “partial join” would not establish a new experiment.

The unresolved contrast is which demands justify retention, how updates reach those demands, and whether retaining one intermediate stage avoids work without materializing later combinations. Extend beyond the measured two-data-relation/request pattern only where the additional stage changes that mechanism. Compare actual retained obligations, not different names for equivalent tables.

## Candidate source and correctness gate

Use repeated requests over a three-relation chain, with a selective final relation and controlled updates to earlier relations. Compare eager intermediate retention, demand-driven subscriptions and indexed rediscovery from the most selective available anchor. Each must produce the same complete source observations under a stated source schedule.

Vary demand repetition separately from data update density, final selectivity and intermediate fanout. Favor subscriptions with a small active demand set and repeated nontrivial joins. Challenge them with short-lived or unique demands, broad invalidation, consumed partners, growing demand sets and low selectivity. Include mostly unique/cheap joins so subscription bookkeeping has a real contrary case.

The first source gate must expose stale matches after consumption, binding changes that enable a previously impossible match, aliases connecting multiple rows, repeated occurrences with equal values, propagation history and demand retirement. Use independent source execution and explicit expected tuples/aliases. Verify that a selective indexed control uses the available key information before ranking it.

Record each retained owner: data indexes, partial tuples, subscription registrations, invalidation links, history and output. Preparation, query updates, observation, cancellation and disposal participate in the later cost comparison. Reuse the earlier script-ownership lesson; input handling must not hide the matching contrast behind avoidable whole-script copying.

## Selection rationale and remaining alternatives

The S05 operation crossover supplies a bounded runtime opportunity for equation reuse as well as strong distinct-request counterpressure. Further invalidation-width and cache-lifetime work remains required. Matching subscriptions are selected next in the breadth cycle because they could remove repeated discovery or unnecessary retention across ordinary multihead programs, a distinct architectural responsibility.

The strongest ready alternatives are reusable parallel workers and corrected restoration/replay policies. Both could change the architecture; neither is rejected. This matching contrast has existing selective controls and a small source-language surface for an independent gate, while testing a retention policy not discharged by the earlier full-pair crossover. Reassess that ordering after the gate reveals actual implementation and proof costs.

T068 owns the mechanism/source gate and ensuing prospective comparison. This does not close broader S01, S05 or the research goal, and it does not authorize treating all intermediate-retention designs as equivalent.
