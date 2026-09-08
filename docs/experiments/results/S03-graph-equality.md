# Contextual graph equality passes independent finite-tree checks

The direct graph now supports contextual variable bindings, finite-tree equality and nonbinding equality queries. Its results agree with an independent scalar substitution oracle on 324 ordered equation pairs across 1,296 complete choice assignments. **This advances the graph toward source execution; it does not establish a complete CHR evaluator or a cost advantage.**

The [prospective registration](../registrations/S03-graph-equality.md) fixes the semantic questions. [Equality implementation](../../../research/chr-direct-choice/src/equality.rs), [directed tests](../../../research/chr-direct-choice/tests/equality.rs) and [independent scalar oracle](../../../research/chr-direct-choice/tests/equality_oracle.rs) are available for review. The implementation uses no reference-interpreter or competing executor code.

## Results that matter for the architecture

**A binding can retain a choice-valued graph reference without choosing its value.** Binding X to choice(A,B) returns one undemanded region when acyclic. Constructor demand and equality queries select only the relevant alternatives later. The representation keeps contextual redirects for unknowns; it does not create a complete substitution map for every history.

**Cycles and clashes fail only their affected contexts.** Tests cover immediate cycles, cycles discovered through earlier aliases and constructor bindings, constructor-name and arity mismatches, and a cyclic arm beside a successful arm. Failure also suppresses observations of unrelated roots. Thus an output-only traversal cannot overlook a contradiction recorded elsewhere in the graph.

**A self alternative is different from a constructor cycle.** X = choice(X,A) leaves X unknown on its reflexive arm and binds it to A on the other. X = choice(f(X),A) rejects the cyclic arm. An occurs check that rejected both indiscriminately would be incorrect; the implementation demands the outer choice when necessary to distinguish these cases.

**Nonbinding equality does not solve for unknowns.** Two independent unknowns do not match merely because they could be unified. Existing aliases and structurally equal values do match, including the same logical unknown allocated as separate nodes. The query takes an immutable graph reference and neither adds bindings nor records failure.

## Independent evidence and limits

The oracle uses owned terms and a separate scalar substitution algorithm. It instantiates two unconditional source-choice labels to each complete assignment, executes each ordered pair from an 18-equation catalogue, then compares survival and all observed roots. Joint alpha normalization preserves alias relationships without requiring the candidate to choose the same representative variable. Successful assignments also check nonbinding equality against the oracle's existing substituted terms.

The catalogue combines aliases, contextual updates, immediate and indirect cycles, choices beneath constructors, correlated and independent choices, equal arms and arity clashes. The directed tests additionally cover disconnected explicit failure and the reflexive-arm distinction. This is exhaustive over the registered catalogue and assignments, not over all finite terms or equation sequences.

The five initial directed tests failed on unimplemented equality operations. The first implementation exposed an observation-order assumption in one test: its two correct labeled alternatives arrived in the reverse order. The final test compares the labeled contexts and values; the API does not promise observation order. Final validation passes all twelve package tests, replay, Clippy with warnings denied and formatting. Every process stayed within its 60-second bound. [Commands and source hashes](s03-graph-equality/validation.json), [final tests](s03-graph-equality/tests.log), [oracle diagnostic](s03-graph-equality/oracle.log).

## Responsibilities introduced and remaining

Contextual binding introduces region intersection and subtraction when following redirects. Occurs checking traverses descendants, including relevant choice arms, before committing a binding. It may therefore inspect substantial structure even when the resulting binding remains undemanded. These are costs to measure; an undemanded result alone is not proof that equality work was avoided.

Failure regions and bindings remain retained until graph disposal. Equality and observation are synchronous recursive operations with no constant service bound. Source callers must respect dynamic birth activation and node ownership. No reclamation, cancellation or source-completion protocol is supplied by this result.

T062 remains active for general source matching, propagation history, rule-body execution and safe publication. The next step is a source executor using these graph operations and contextual occurrence ownership, followed by the [compound source gate](../registrations/S03-dynamic-resource-source-gate.md) and finite-sibling progress checks. A completed source gate is still required before a graph cost comparison. S01's selective/consuming and lifecycle comparisons, S02 integration and S06 direct compilation remain unresolved alternatives; this equality work supplies a necessary prerequisite for the already selected direct-graph trial.
