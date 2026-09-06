# A conditional kernel: representation and local preservation arguments

A condition-labelled graph can represent finite-tree substitutions and CHR occurrence lifetimes without copying the whole store at a choice. The local operations below have pointwise preservation arguments. This establishes a semantic construction to investigate; it does not establish economical matching, condition manipulation, scheduling or answer enumeration.

This is original project analysis, not a theorem attributed to the surveyed systems. The target is [the proposed reference model](T008-reference-and-candidates.md). The arguments are written mathematics, not mechanically checked proofs or an implemented evaluator.

## Finite frontier and projection

At any finite execution prefix, let Ω be the finite collection of current frontier alternatives with distinct identities, with Live ⊆ Ω marking survivors. Failed leaves may remain in Ω for bookkeeping but have no live interpretation. A condition is a subset of Ω, represented abstractly by Boolean operations. An implementation may use formulas, decision diagrams, explicit sets or a hybrid; this construction does not select one. Equal subsets can have very different representation costs.

Splitting frontier leaf a replaces it by fresh children aL and aR. Let π map both children to a and every other leaf to itself. Every existing condition C is inherited as its inverse image π⁻¹(C). The chosen body arms receive the respective child conditions. A label not executed in a leaf does not create more alternatives there. Thus nested dynamic choices do not manufacture duplicates by enumerating irrelevant assignments.

For a runtime object R, projection `R|a` means retain precisely the nodes, bindings, occurrences, tokens and goals whose conditions contain a. The invariant is that each live projection denotes a well-formed reference state, up to fresh identity renaming. Runtime identities need only be injective within a projection; shared objects may denote the corresponding objects in several projections.

Maintain a separate birth condition for occurrence IDs and logical variables. Consuming an occurrence changes its live condition, not its birth condition. An identity cannot be reused where it has previously been born, even when its occurrence is no longer live: histories can still refer to it.

## Conditional finite-tree bindings

Constructor nodes have a symbol and ordered child edges. Every edge support must be contained in the birth supports of both endpoints; constructor child edges inherit the constructor birth support, and each child must exist throughout that support. Logical variable nodes have zero or more outgoing binding edges `(x, t, C)`, with pairwise disjoint conditions for distinct targets. In any live projection, a variable is either unbound or has one target. A projected binding graph must be acyclic. Its recursive interpretation denotes a solved finite-tree substitution; the graph itself may keep indirections rather than eagerly expanding every term.

The union graph need not be acyclic. Under A one may have `X = s(Y)`, and under the disjoint condition B one may have `Y = s(X)`. Each projection is finite. An unconditional graph cycle test would reject legitimate states.

### Lemma 1: conditional reachability gives the occurs condition

Give each graph edge its activation condition; constructor edges have their constructor birth condition. For a path, intersect its edge conditions. For all paths from term node t to variable x, take the union of those intersections. Call the result `Reach(t,x)`; the zero-length path contributes the birth condition of x when t is x.

Then `a ∈ Reach(t,x)` exactly when the projected graph contains a path from t to x. The forward direction follows by selecting the witnessing path whose edges all contain a. For the reverse direction, the projected path's original edge conditions all contain a, so their intersection and the union contain a. Cycles in the union do not invalidate this argument.

This relation can be computed as Boolean-labelled reachability on the finite graph. Transitive-closure methods use finitely many Boolean operations; the sizes and costs of their operands are not bounded by this observation. Reachability is a semantic construction, not a recommendation to run a cubic closure algorithm for every equation.

### Lemma 2: binding an unbound representative preserves finite trees

Suppose x is an unbound representative throughout condition C, after conditional dereferencing. First handle the subset where the other side dereferences to x as the trivial equation. For the remaining term target t, partition C into `F = C ∩ Reach(t,x)` and `E = C \\ F`.

Fail F and install `x → t` only under E. In each a in F, t contains x after the current substitution, so the nontrivial equation has no finite-tree solution. In each a in E, adding the edge cannot create a projected cycle: a new cycle would require an existing path t to x. Since x was unbound there, no previous binding is overwritten. Outside C the projection is unchanged.

The interpreted substitution after installation is the composition required by the reference, including all indirect occurrences of x. Physical indirection is not literal extension of an already-solved mathematical substitution. This construction must not mutate a binding used outside E.

### Lifting the remaining unification operations

Conditionally dereference both sides of a pending equation and partition its support only where their exposed cases differ. Equal representatives discharge the equation; constructors with different symbols or arities fail; equal constructors generate corresponding child equations under the same support; an unbound variable invokes Lemma 2. Orient two unbound variables consistently to avoid representation churn. Partitioning bookkeeping is not a language disjunction and must not create extra alternative identities.

Each projected operation is an ordinary finite-tree unification transformation. For a fixed finite equation batch and fixed frontier, with no additional CHR work, provided all outstanding equation work is eventually processed, the successful projected result is a most general unifier and the failing projections are exactly those with no unifier. This lifting depends on complete case partitioning and faithful conditional dereference; it does not assume that a particular optimized graph algorithm has implemented them correctly.

The scalar foundation is Martelli and Montanari's equation transformations and solved-form result: [An Efficient Unification Algorithm, §2, Theorems 2.1–2.3](https://courses.grainger.illinois.edu/cs576/sp2017/readings/01-jan-19/martelli-montanari-unif.pdf). The conditional reachability and lifting above are our extension. Their source's performance results are not transferred.

A finite family of finite scalar problems can be processed by a worklist whose useful steps each advance at least one projected problem. Splitting an existing support is finite, and equivalent idle work must not be repeatedly reintroduced. This gives a terminating construction for a fixed finite batch of equations. Recursive CHR execution can create further equations and alternatives indefinitely, and efficient symbolic implementations need a stronger operational accounting.

## Conditional rule application

For each occurrence i, let `M_i` be its live condition. For each rule/head tuple τ, let `H_τ` be the condition where its token is present. Choose a tuple of distinct occurrence IDs for the retained and consumed heads. Let D be the exact condition where matching and guard entailment hold under the projected substitutions, without binding store variables. Also require a conditional matching substitution μ: for each a in D, μ projected at a supplies the matching values of rule-head variables. Applicability alone does not supply those values; for `p(f(X)) <=> q(X)`, X can differ between matching alternatives.

The applicability condition is:

```text
C = Live ∩ D ∩ (intersection of M_i for selected heads) \\ H_τ
```

A scheduler may choose any E contained in C on which it commits to this application. Choosing E commits work in those alternatives; it does not choose their other possible CHR schedules as search alternatives.

Perform these abstract updates:

- For each consumed occurrence i, set `M_i := M_i \\ E`. Retained heads keep their membership.
- Set `H_τ := H_τ ∪ E`.
- Freshen the rule consistently and give new body-local variables birth condition E. Instantiate matched head variables using μ and preserve its conditional argument values; do not replace distinct fresh invocations by a common logical variable without justification.
- Introduce pending body goals with support E. Body equations, choices and failures are subsequently processed under that support.

### Lemma 3: this update simulates one application per participating alternative

For a in E, all selected occurrences are live and distinct, the head/guard test holds with the projected witness μ and the token is absent. The projected updates therefore retain, consume, freshen, add goals and record history exactly as the reference application. For a outside E, every membership and token projection is unchanged, and newly born objects are absent. Thus one symbolic application represents one permitted rule step in every participating alternative, with no change elsewhere.

The lemma does not require the participating stores or argument values to be identical. For example, `carry(X,N) <=> carry(X,s(N))` can construct the same outer symbolic node while X has different projected values, because the rule does not inspect X. A finite chain of such operations can represent common post-choice work. Whether actual execution does so once must still be measured at the operation level.

The hard premise is obtaining D, its witness μ and the participating tuple without spending more on matching, equality, dependencies and conditions than separate execution would spend. Defining D pointwise proves what must be computed; it is not an efficient matching algorithm. Compiled structural matching can preserve opaque arguments, but nonlinear heads and arbitrary supported guards may require more discrimination.

## Failure, histories and quiescence

Failing F restricts Live to its complement; shared objects remain available in surviving projections. Inherited tokens are pulled back through Split, while post-choice tokens carry only their firing support. The two history traces in [worked obligations](T008-worked-obligations.md) follow directly from these rules.

A quiescence certificate for E requires no pending goal support intersecting E and no enabled unused rule/head tuple anywhere in E. Testing only selected output groundness or an empty local queue is insufficient. A complete index or wake-up discipline must justify that no enabled tuple has been missed, including after conditional alias changes.

Exact matching, complete wake-up and fair servicing of pending work remain algorithmic obligations. These local lemmas do not establish a whole-system liveness theorem. In particular, sharing must not make a branch wait for a result whose divergence depends on a different branch's bindings.

## Consequences for architecture and language research

This construction demonstrates that finite-tree unification, conditional consumption and history can coexist semantically. A whole-store copy is not required by the reference transition definition. It does not demonstrate the desired net speed or a simple implementation: conditions, reachability, matching and quiescence can all dominate costs.

Consequently, a global restriction is not needed merely to make conditional lifetimes definable. Restrictions may still make their computation cheaper. Locality, stable discrimination, limited aliasing or resource ownership deserve evaluation by how they simplify D, occurs checks, wake-up and memory reuse—not by asserting that unrestricted semantics is impossible.

The next implementation-dependent questions are the cost and precision of those mechanisms, but prototype scope should first state a concrete condition representation and matching strategy. A minimal proof-oriented kernel experiment would test these lemmas' implementation, not yet compare full architectures or claim representative performance.
