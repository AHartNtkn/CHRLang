# Local compilation and solver reuse contracts

A useful fragment of the intended relational programs has unambiguous occurrence expansion without requiring ground input modes. This simplifies a local compiler's obligation, but equality, wake-up and explicit search still need their own implementations. Solver reuse can likewise be investigated through a local logical interface without making the entire language set-based.

These are proposed fragments and mathematical arguments derived from [the reference](T008-reference-and-candidates.md), not adopted language restrictions or existing CHR-to-net theorems.

## Unambiguous relation expansion

Define fragment R by the following sufficient source conditions:

- Rules are single-headed simplifications.
- Each predicate has one variable-only head, or constructor patterns that cannot both match the same occurrence.
- Guards have a specified pure entailment meaning.
- Bodies may contain conjunction, fresh variables, finite-tree equations, explicit OR, failure and calls within R.
- Matching does not instantiate store variables. An unknown constructor demand suspends.
- Variables may be shared between occurrences; source linearity is not required.

Addition fits directly. A relational SK evaluator can use a catch-all `eval` rule with explicit body alternatives and equations encoding the evaluator cases. `no_c` can use disjoint constructor heads and retain an unknown hole as a residual constraint. This establishes a way to express the evaluator pattern, not an automatic translation theorem for every rwlog construct.

R is sufficient for unambiguous expansion of a given occurrence. It does not establish termination, equation independence, absence of body choices, or eligibility for an entire interaction-net implementation.

### Commuting-application lemma

Consider two enabled Apply transitions with different tokens. Suppose neither consumes an occurrence used in the other's heads, and each freshens its variables independently. Then applying them in either order gives the same resulting state up to fresh identity renaming.

The reason is specific to the reference transition: Apply posts body equations as pending work and does not itself modify the store substitution. The other transition's matching and guard truth therefore stay unchanged. Neither consumes the other's required heads. History additions and multiset body additions commute, and fresh allocations admit consistent renaming.

Distinct single-headed simplification occurrences satisfy the resource condition even when their arguments alias. This permits batching or parallelizing these expansions under a suitable implementation. It does not permit arbitrary reordering of the subsequent unifications, choices and failures, and does not itself share work across alternatives.

## What a net backend still must establish

For R, a sufficient backend contract would include:

1. One expansion controller per source occurrence; an occurrence cannot expand twice.
2. Principal-port local interactions, possibly a finite administrative sequence, with one replacement per active-pair type and preservation of its external interface.
3. Logical-variable identity shared correctly across references. Duplicating a reference must not create an independent logical variable.
4. A finite-tree equality service supporting composition, aliases, conditional occurs failure and wake-ups.
5. Fresh identities for rule variables and dynamic choices, with choice correlation and branch-local failure.
6. Conditional resource lifetimes and suspension on unknown constructor demands.
7. A progress argument for administrative work and a schedulable unit consistent with the declared search policy.

If each expansion/service operation simulates its declared reference transition, and the interfaces preserve their invariants, their composition simulates R. This is a conditional composition claim: it identifies the premises a compiler proof needs, not a proof that a proposed backend already meets them. Preserving successful explicit alternatives additionally needs liveness assumptions.

The inspected ordinary/nested-net results support local replacement and specified pattern encodings. They do not discharge the equality, search and wake-up services above. There is no basis for deriving mandatory ground modes merely from principal-port discipline. A representation with unknown-variable cells may support them; its costs remain to be investigated. See [the net evidence record](T003-interaction-nets.md).

### Beyond single-headed rules

A sufficient multiheaded region condition supplies a routing/ownership structure through which partners can be gathered, prevents competing consumption during gathering, revalidates matching/guards at commit or proves them stable, and gives the gathered update one atomic source interpretation. Reservations must not strand resources indefinitely.

Exclusive region ownership is a simple sufficient condition, but restrictive. Shared retained facts may be permitted when competing consumption is excluded and their relevant properties remain stable. Mere equality of logical arguments does not establish a private communication channel or exclusive ownership.

With `p(X), q(Y) <=> r(X,Y)` and store `p(a), q(b), q(c)`, one declaration still has two instances competing for p. This refutes a direct conflict-free mapping based only on declaration uniqueness. It does not prove the program cannot be compiled to ordinary nets: administrative machinery may implement a chosen committed source schedule. A strongly confluent target is not automatically incompatible with a source compiler that deliberately selects one permitted schedule.

## A local logical solver interface

Consider a candidate solver region with public variables U, freshly scoped private variables W, declarative theory D and accumulated formula Φ(U,W). Only this region has logical/idempotent semantics; ordinary host occurrences retain multiset identity and consumption.

Its operations have these obligations:

- `add(ψ)` conjoins ψ with Φ.
- An emitted consequence χ requires `D ⊨ (∃W. Φ) → χ`, with χ over the appropriate public variables.
- Failure requires inconsistency of `D ∧ Φ`.
- Exact projection returns Π(U) satisfying `D ⊨ Π(U) ↔ ∃W. Φ(U,W)`.
- An unknown result preserves the obligation/residual information. It is neither failure nor permission to publish an incomplete answer.
- Reuse freshens private variables and reinstates the original caller constraints.
- Hidden reads of host resources are forbidden by the interface. If a consequence depends on a consumable occurrence, that dependency must be explicit and remain valid wherever reused.

This is a candidate local semantic extension. It is not a soundness argument for treating arbitrary existing CHR predicates as a logical solver just because they resemble constraints.

### Calls, answers and learning have different authority

The full-TCLP theory distinguishes overapproximating a call from overapproximating its answer. Exact answer projection and consumer filtering permit broader calls under that definite-program framework; broadening answers can invent solutions. Call generalization can also lose termination. These results do not directly cover consumable CHR histories or duplicate alternatives. [Arias and Carro, §§2–4, especially Table 2](https://arxiv.org/pdf/2009.14430)

A reusable table summary may either replace an obligation with an exact equivalent projected formula, or add a certified consequence while retaining the obligation. These are different operations. A formula table is not an execution snapshot and does not preserve a host continuation or its history by itself.

A learned clause L can carry a scoped validity certificate `D ⊨ A → L`. It is reusable where assumptions A are established. Branch-private variable names must be freshened or represented schematically; they cannot become accidental global identities. Divergence supplies no inconsistency certificate.

If a solver summary contains a disjunction, keeping it as residual logical information and enumerating its alternatives are different interfaces. The latter must follow the host's explicit-search contract. It cannot silently introduce search in a program that used no explicit disjunction.

## Can the supplied pruning constraints fit?

For finite terms, define N(t) to mean that t contains no `c` constructor and is built from the permitted SK constructors. The supplied `no_c` rules structurally decompose application, accept k/s and reject c. On a known finite constructor skeleton, this decomposition terminates; holes remain constrained. A later binding may expose further structure and must wake the relevant constraint.

This gives a plausible logical interpretation for `no_c`, including residual `N(H)` on an unused hole. Certifying a region requires checking every interaction with the predicate, not only these four rules. A host rule observing two `no_c` occurrences could distinguish multiplicity; that prevents silently treating the whole program as set-based.

Even when two copies of N(H) have the same logical solutions as one, replacing residual multisets by formulas is a semantic interface choice, not automatically the baseline's structural answer equality. A declared solver region makes that choice explicit. Its cost is an interface and certification obligation; its potential benefit is reusable consequences and summaries. Disequality and the notebook's normal-form constraints require separate analyses of their definitions and variable representation.

## What is settled and what needs an experiment

The commuting lemma removes a false requirement for alias independence during pure expansion. R shows that useful relational modes can survive a local compilation discipline. The solver interface separates exact answer meaning from call generalization and from learning. These are substantive semantic reductions in the design problem.

The remaining net question is a concrete equality/choice service design and its administrative costs. The remaining solver question is whether certified fragments capture enough of the synthesis workload, and whether reuse saves more than abstraction, reinstatement and entailment cost. These require candidate algorithms before timing; a literature-only claim of superiority would be unsupported.

No owner restriction decision is needed merely to investigate these contracts. Adopting R as the entire language, or exposing a logical-region boundary as a required programming construct, would require an explicit recommendation and owner decision after its costs are assessed.
