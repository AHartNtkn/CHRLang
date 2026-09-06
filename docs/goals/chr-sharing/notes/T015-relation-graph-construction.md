# A relation graph with contextual logical variables

A restricted relation graph can share an occurrence's expansion and common descendants after an explicit choice. Constructor demand and logical-variable bindings require context-sensitive results, while active constraints remain scheduled independently. This construction is a concrete competitor to a general conditional multiset engine, with a narrower dispatch mechanism.

The construction is original. Memoized pull-tabbing supplies the starting idea of ownership and task-specific results; the CHR occurrence, equality and wake-up rules below are additional obligations. T015-source-expansion.md records that source boundary. This is a paper algorithm and local simulation argument, not an implemented compiler or a global preservation proof.

## Source fragment and observable behavior

Use fragment R from T011-local-compilation-and-solvers.md: single-headed simplifications with unambiguous constructor discrimination, pure guards, arbitrary body conjunction, finite equations and explicit OR. Calls may be recursive and variables may alias. Unknown constructor demands suspend. Predicates in the region are closed: no external rule reads or consumes their occurrences. Communication with other regions is through declared logical variables or posted constraints.

A source occurrence is not a function whose only observable role is its return value. It is an active obligation. A query containing `answer(a), reject(c(z))` cannot succeed merely because answer(a) does not demand reject's argument. All obligations participate in failure and quiescence. The graph therefore has a forest of active occurrence roots, not only the selected output-term roots.

The most favorable expansion case has a single linear variable-only head and a true guard. For example, `p(X,Y) <=> ...` need not inspect X or Y. A repeated head variable such as `p(X,X)` is an equality demand and does not qualify for this opaque case. Constructor heads and guards qualify only in contexts where their demanded observations have been established.

## Runtime objects

1. A persistent search-context tree records only executed source ORs. Descendants inherit ancestor data; local bindings override it. A context can represent an entire still-undistinguished family. Static source locations alone are not dynamic choice identities.
2. An occurrence node holds provenance, predicate, argument references, and contextual expansion results. Two identical source occurrences in the same branch remain distinct nodes or distinct occurrence handles. Provenance includes the parent application and body position, not just the printed term.
3. Constructor nodes are immutable. A logical-variable handle denotes the same variable at every aliasing reference within a context. Its binding map is contextual. Reading a binding uses the nearest applicable entry, with descendants overriding ancestors.
4. An expansion event records the chosen rule, matching witness, demanded-input certificate, fresh body graph, and the contexts in which the source application has committed. A cached body is not automatically a committed source effect.
5. Work roots include pending equality, occurrence introduction/expansion, source OR, failure, wake-up, and final observation jobs. A suspended occurrence subscribes to every traversed logical-variable binding dependency.

Context membership can be represented by ancestor regions with exceptions or a decision structure. This is conditional information, even though it is attached to nodes instead of a general constraint-store relation. Calling it a graph does not eliminate the information needed to distinguish alternatives.

## Expansion without inspecting an argument

For a live occurrence p with a linear variable-only head and true guard, compute the body graph once using p's argument handles. Each fresh body variable is identified by `(event, local-index)`; each new body occurrence by `(event, body-position)`. The handles' interpretation remains local to an alternative. A later binding in one child must not alter another child's interpretation of the same handle.

An event can be cached before all inheriting alternatives select that application. When an alternative selects it, commit the source effects there: consume p and activate its body goals exactly once. If a whole supported family selects it together, one event and one contextual membership update can perform that activation for the family. The cache alone is not a reason to skip source-policy selection or count one source transition as completed everywhere.

This preserves fresh-variable relationships. Two references to a local variable from the same event alias within a branch. Two distinct invocation events get different handles. Descendants of the same shared event may use the same symbolic handle because their binding environments are mutually exclusive. A returned answer freshens non-output variables under its own context.

For `carry(Tag,s(N)) <=> carry(Tag,N)`, constructor demand concerns the second argument. If that argument is the same immutable skeleton across a family, each successor event can be computed once and inherited. Tag is passed as an opaque handle even when its contextual bindings differ. W common recursive reductions then require W event constructions, plus context lookup/commit and scheduling work. This is sharing after choice, not merely reuse of the original program text.

The claim fails if an implementation constructs every successor independently per leaf before noticing identical events. Provenance inheritance and common argument references must survive the split. Record expansion events and context-management work separately when evaluating the candidate.

## Demand and contextual results

When expansion must inspect a constructor, dereference under the requesting context. An unbound logical variable produces suspension; it is never replaced by a constructor generator. A constructor mismatch means that head does not match. It is not failure of the entire branch unless the source program has an enabled rule producing failure.

An expansion learned after a branch-specific binding is stored with that binding's context and relevant version information. It must not overwrite an unconditional ancestor entry. For instance, after a choice binds X to k in one branch and c(z) in another, the result of `no_c(X)` cannot become the common result of the occurrence merely because the occurrence was born before the choice.

A demanded-input certificate records alias traversals as well as terminal values. Reading X through X→Y subscribes to both links as appropriate for the binding representation. The read-subscribe-validate discipline in T012-matching-and-wakeup.md applies: a binding change between reading and subscribing must trigger a recheck. A cached suspended result is invalidated or refined when its dependency changes.

Shared results are safe on a larger context only when all demanded observations agree and occurrence eligibility holds there. The runtime may prove this by region containment and identical immutable references; it need not solve arbitrary expression equivalence. Recombining separated results with merely equal printed terms is insufficient if their variables have different aliases or their source effects have different provenance.

## Finite-tree equality service

Post body equations as active work. For a requesting context, run a finite equation worklist over the contextual bindings: dereference; erase identical representatives; compare constructor roots; decompose equal constructors; and bind an unbound representative only after checking that it is absent from the other term's contextual reachable graph. Failure removes only that context's alternatives.

The occurs check must use one consistent projected context. A cycle in the union of two incompatible binding environments does not prove a cycle in either one. Conversely, omitting a branch-local edge can miss a real cycle. Updates remain private until the whole source Unify transition succeeds. The service then publishes the composed bindings and wake-ups under the same context.

This equality implementation can initially operate one context at a time while expansion remains shared. That is a meaningful hybrid, but it may spend branch-proportional equality work. To share equality itself, add compatible-context partitioning and common equation-worklist results. At that point it inherits much of the conditional-kernel proof obligation; do not count the two candidates as independent solutions to that subproblem.

## Local simulation argument

Project a runtime context to a multiset of committed live occurrence handles, its active body goals, and its contextual substitution. In R, consuming the sole head prevents repeated firing, so an additional propagation token mechanism is unnecessary inside the region. Occurrence identity and single-commit status still matter.

A committed expansion has a live sole head, a valid nonbinding matching witness and entailed pure guard. Replacing that occurrence with the fresh body therefore projects to one permitted Apply. Event reuse changes neither the witness nor the branch-local interpretation of fresh handles. A cache-only update projects to no source step. Constructor suspension projects to an unchanged ineligible application. The equality service projects to a finite-tree MGU or to failure.

These facts establish the local safety of the listed operations under their invariants. Whole-machine preservation still requires invariant closure across concurrent refinement, introduction, OR and wake-up. Finite-answer progress additionally requires fair service of every active obligation, stable quiescence certification, and no unbounded normalization hidden inside graph evaluation. T015-symbolic-scheduling.md offers one conservative scheduling construction.

## General CHR and programming cost

A multiheaded rule adds partners whose availability can change independently of the relation node. A propagation rule adds token history that survives argument instantiation. A competing consumer adds an atomic selection obligation. These cannot be inferred from an expression node's owner or argument demands. A graph representation can store these dependencies, but then it needs a store index, occurrence consumption conditions and history services comparable to the general candidate.

The useful language proposal is therefore a closed relation region, either inferred or declared, with full CHR outside it. Addition and subtraction fit the region without ground input modes. An evaluator using one catch-all rule with explicit body alternatives also fits; its pruning constraints remain independently active. A language-wide R restriction would disallow ordinary multiheaded CHR and propagation unless encoded through additional services. Those are substantial costs for the owner to assess, not an assumed requirement for sharing.

The construction identifies a real specialization: common relation expansion can use occurrence-local memoization without general partner lookup. It does not establish which representation is faster, whether closed regions are common enough in realistic synthesizers, or whether equality will dominate saved expansion work. Those measurements require implementations. The remaining analytical tasks are whole-machine simulation and a precise cross-region interface; neither depends on the conditional-store experiment running first.
