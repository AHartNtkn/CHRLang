# Whole-evaluator symbolic compilation as a competing direction

ECTAs and finite decision diagrams need not be limited to structural solver regions. A more ambitious alternative compiles bounded *symbolic execution witnesses* for the entire evaluator. This investigates that route without assuming that the original program denotes one regular tree language or that synthesis answers must be ground.

## A finite witness construction

Fix the finite ruleset/query, a computable committed rule-selection policy, and the equality-only reference machine. Choose bounds on the number of source transitions, term/variable nodes, live and historical occurrence identities, and choice events in a witness. Canonically number fresh entities by allocation order. Encode variables as symbolic handles and finite terms as acyclic graphs; do not instantiate every unbound variable to a ground term.

There are finitely many encoded states at fixed bounds when constructors come from the finite ruleset/query signature. Builtins generating additional atoms or external values require their own bounded finite-domain encoding and are outside this construction's premise. Pending goals, occurrence multisets, substitutions, history and choice lineage are explicit state fields.

A witness is a bounded sequence of states with a source-action tag at each step. Require each adjacent pair to satisfy the reference transition relation, the chosen committed policy, finite-tree validity and fresh-allocation discipline. The endpoint must be quiescent and nonfailed. Only explicit OR permits distinct alternative selections; choosing a different ordinary rule schedule is not another synthesized answer.

At fixed bounds, validity is a finite relation. It can be represented by a Boolean formula, finite decision diagram or finite constrained term space. A direct exhaustive table is a constructive existence proof, not an efficient implementation recommendation. A useful compiler factors the transition predicates and equalities instead of listing all tuples. ECTA path equalities can connect corresponding fields across states; general finite conditions can be represented by finite unions, potentially at large size.

## Correctness and coverage

Every satisfying witness is a reference execution by induction over its transitions. Decoding its endpoint yields a symbolic answer with aliases and residual constraints intact. Conversely, every finite reference derivation uses finite state and identities, so some bound includes its canonically encoded witness. Fairly advancing a sequence of dominating bounds therefore eventually covers every finite successful derivation under the fixed policy, provided each finite instance is serviced to completion or dovetailed fairly.

The same derivation can fit many bounds, and different derivations can return the same answer. Canonical witness allocation reduces representation duplicates; final structural answer dedup handles repeated observations. Failure to find a witness at one bound means only bounded absence. It never establishes global failure or termination of an unbounded search.

Symbolic endpoints matter. For example, an ignored synthesis hole constrained by no_c remains a variable handle with its residual occurrence; the witness does not have to enumerate every ground inhabitant. Ground-program-only bounded synthesis would fail to preserve this required behavior.

## What this direction buys and costs

The potential gain is global factorization, constraint propagation and reuse over many bounded derivations, including correlated subterms that a plain regular grammar cannot describe. It may let a solver rule out whole trace families before concrete source expansion. Its costs include encoding entire operational states, absence/enabledness tests for quiescence, occurrence identities, policy constraints, formula construction and repeated larger bounds.

The constraints generated repeatedly by recursive execution need not satisfy the finitely-constrained cyclic ECTA restriction. Compiling a fresh finite instance at each bound avoids invoking a theorem outside its premises, at the cost of repeated compilation. Reusing compatible compiled subrelations across bounds is possible only with their exact interfaces and identity scopes preserved. Whether that reuse beats an incremental CHR evaluator is empirical.

This is a whole-evaluator alternative with baseline symbolic-answer coverage, not merely a structural solver. It is also distinct from a ground SK grammar search. Its finite construction and unbounded dovetail argument resolve expressibility at the paper level; they do not imply that the construction is small or competitive.

## Disposition

The analytical gap about a full symbolic-compilation direction is closed by this construction. Useful next results require a concrete factored transition encoding and measurements: formula/automaton size, compilation time, solver work, incremental reuse, time to nonground answers, and comparison with direct execution at equal bounds and policies. Term-language emptiness theorems alone cannot determine these costs. No global bound or ground-output restriction is proposed for the source language.
