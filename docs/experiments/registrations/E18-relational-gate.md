# E18 integrated constructor-relation semantic gate

Registered before the generated gate. This is an architectural semantic entry,
not a comparative timing run or a claim of complete CHR implementation.
[Assumption register](../architecture-assumptions.md),
[source assessment](../results/E18-source-assessment.md).

## Question and candidate

Can constructor consistency, equality and application deductions inhabit one
flat relational substrate while preserving finite-tree denotation and correlated
finite contexts? The kernel must not contain a constructor tag dispatcher,
recursive term unifier, or complete-equation request/response interface.
Constructor/equality axioms are ordinary generated rules alongside application
rules. Frontend lowering and final term extraction may inspect syntax.

Facts have a predicate, ordered integer-ID ports and a finite bitset support.
Rules join premises, intersect their supports and union supported conclusions.
The initial universe is finite. Only explicitly supplied finite source-choice
recipes introduce contexts; internal matching does not invent search alternatives.
The gate uses equality as an ordinary relation with equivalence axioms, not
physical ID fusion. This is a transparent denotational baseline; efficient
union-find, contextual overlays and specialized joins remain open controls.

Generate equality substitutivity rules for every application value column as
well as constructor rules for injectivity, congruence, different
symbols/arities, and positive reachability modulo equality. Preserve descriptors.
A cycle in one context fails that context; incompatible-context union cycles do
not. Unknown IDs remain unconstrained variables. Ordinary application rules must
be able to introduce descriptors/equalities and query their consequences during
the same fixpoint computation. No separate equality saturation call is required
between application steps.

## Independent semantic obligations

Hand cases cover repeated holes, indirect aliases, constructor/arity clashes,
post-identification cycles, descriptor retention, context-local failure,
correlated swapped bindings, acyclic projections of a cyclic union graph,
constructor queries that do not invent structure, and gradual application-driven
consistency, and application joins newly enabled by equality. Occurrence identities
in later source extensions must not be quotiented as value columns. Extraction preserves joint nonground relationships and refuses
unfinished or inconsistent contexts. Compare complete equations against a
separate tree-unification oracle, not another use of the generic rules.

A finite saturated consistent equation/descriptor quotient should interpret as a
most-general finite solution (not a universal model of all future instances of
residual application constraints): one constructor shape per described equivalence class, no positive
quotient cycles, and distinct fresh variables for undescribed classes. Tests must
check joint substitution relationships, not merely ground output equality.

The frozen generated grid uses these12terms in this order:
`X`, `Y`, `Z`, `a`, `b`, `f(X)`, `f(Y)`, `f(a)`, `g(X)`,
`pair(X,X)`, `pair(X,Y)`, `f(f(X))`.
For each ordered left/right pair, test three environments in order: no extra
equation, `X=Y`, and `X=f(Y)`. This gives432recipes. Group consecutive recipes
in at most four explicitly selected contexts per engine (108batches). X/Y/Z IDs
refer to the same source variables interpreted separately under each context;
constructor occurrence nodes are distinct unless the same encoded occurrence is
reused. The recipe alternatives are explicit finite ORs, not choices created by
rule competition. Hand TDD checks may run during implementation.

Bound each generated batch to30seconds and1000closure rounds. Preserve errors,
timeouts and inconsistent oracle results; a bound hit is unresolved and requires
investigation. Retain fixture, kernel, axiom, oracle and runner hashes plus raw
per-recipe outcomes and replay. Count generic rule scans, joins and support work
for reproducibility, without interpreting them as shared-DAG comparison costs.

## Scope and next contrasts

The first fragment is monotone and finite, with no dynamic allocation or source
resource consumption. It does not establish general CHR propagation histories,
recursive explicit-choice births, full source matching or fairness. Those remain
required subsequent tests; a successful gate cannot close A1.

Before performance or partial-sharing claims, add competent shared-node/worklist
controls and a same-representation transactional control. Compare partial overlap,
high-degree alias updates, branch divergence and complete applications. Source
correspondence must allow unsolved relational states and interleaved deductions;
requiring each internal step to return a solved substitution would exclude the
architecture under investigation. A change to source semantics remains separate
from this internal representation experiment.
