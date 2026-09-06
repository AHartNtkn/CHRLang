# Cross-region composition and failure learning

## Shared variables across different execution regions

A closed relation region need not be an independent AND component. The former forbids external rules from intercepting its occurrence predicates; the latter additionally requires freedom from future cross-region binding and resource interactions. Conflating them would wrongly exclude useful relation/solver hybrids or wrongly permit independent answer products.

A sufficient region interface contains:

- globally meaningful logical-variable handles, interpreted under support;
- fresh event/occurrence allocation with birth support;
- proposed source effects: posted equations, inserted/consumed occurrence identities, propagation-token updates and failure;
- dependency subscriptions for every variable or occurrence fact used by a suspended demand;
- a finite advance operation and branch-local quiescence evidence.

A relation service can own eval/fold occurrences and a structural solver can own no_c occurrences while both refer to P. When equality binds P, one global binding publication wakes both subscribers. Neither service silently substitutes a private copy of P. Every effect is committed under a validated support and snapshot; local completion is not global answer completion.

For a finite collection of regions, let the global source store be the union of their owned occurrences and pending goals, with one contextual equality/history interpretation. Each region's step must project to a reference transition on its owned occurrences and the common binding environment. The global coordinator serializes conflicting effects or commits a proved commuting batch. Induction then yields the same composed simulation as T016. Quiescence requires all relevant regions to be stable at a jointly valid branch version, including unresolved cross-region wake-ups.

At ruleset linking time, adding a rule with a head spanning two previously closed predicate families invalidates separate local dispatch. The compiler must enlarge the region or use the general matcher. This is an eligibility computation, not a semantic fallback: the authoritative source model is the same CHR machine, with different proved implementations for different regions. A mandatory globally restricted language could eliminate this linking analysis, but would impose the expressiveness cost documented in T016.

## Proof-producing unification nogoods

Finite-tree unification failure can yield a reusable explanation. Give every asserted equation and dereference edge a provenance condition under which it holds. A constructor clash certificate consists of the original equation plus the finite dereference/equality path leading to distinct constructors. An occurs certificate consists of the proposed variable binding and a finite path back to that variable. The conjunction of their provenance conditions is sufficient for the contradiction.

The certificate's condition must include the committed application events responsible for the equation, not merely printed argument values. If the equation was posted only after one consumer won a race, it cannot justify pruning a context where a different consumer won. Explicit choice assignments can express the condition only when they actually determine those events under the fixed policy; otherwise retain the stronger event/state dependency.

Once a projected source Unify has failed, every descendant of that failed alternative is excluded. Reusing a certificate elsewhere is sound only when its assumptions are entailed there. A minimal first implementation stores the exact conjunction of all witness conditions. Generalization can attempt to omit assumptions and recheck the finite contradiction, but is optional and must not replace proof with statistical similarity.

This permits a learned exclusion over contexts without creating implicit search or adding equations to guards. A conditional engine can subtract its certified failure support from Live. A decision-graph implementation can retain the exclusion as a Boolean constraint over existing labels and event conditions. An unsatisfiable support means no alternative; it is not a source term.

## Logical solver nogoods and limits

In a certified monotone logical region, an exact solver's unsatisfiability certificate can use the same scheme: attach assumptions, prove contradiction, and reuse it only when those assumptions hold. The finite-tree/regular formula procedure provides one such domain. Arbitrary CHR residuals are not automatically monotone logical facts: consumable obligations can disappear under another valid committed execution. The counterexample with competing no_c rules in T015 demonstrates the danger of importing an eager logical failure test globally.

Subsumption and entailment can strengthen table reuse within an exact logical interface. They require caller filtering and exact answer projection, already analyzed in T011 and T016. Keeping only a consequence that could admit additional answers is unsound for answer replacement even if it is useful for pruning. Tabling keys for full CHR must include enough continuation state and histories; equality of outputs is not such a key.

## Disposition

Cross-region semantics and conservative learned-failure reuse have sufficient constructive contracts. Actual benefits depend on witness size, entailment costs, hit rate, binding overlap, wake-up traffic and source-event provenance retention. These require implementations and representative workloads. Minimal certificates and full-state structural keys provide correct controls; more aggressive minimization or generalized tables should be justified by those costs rather than assumed necessary for basic correctness.
