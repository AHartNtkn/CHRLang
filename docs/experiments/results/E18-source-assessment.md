# E18 constructor relations and architecture sources

Constructor relationalization has direct precedents. It is a meaningful contrast
to a graph-backed transactional unifier, but graph layout alone supplies no
performance or conditional-sharing result. This assessment selects semantic
probes, not a production backend. Sources accessed 2026-09-07.

## Relevant primary results

[Relational E-Matching, §§3–4](https://arxiv.org/pdf/2108.02290) represents operators
as root/child relations and structural matching as conjunctive queries. Repeated
variables and functional dependencies inform join optimization. This supports a
flat relational matching alternative. Matching modulo congruence is not by itself
finite-tree unification or destructive CHR execution.

[egglog, §3 and full-version Appendix A.3](https://arxiv.org/pdf/2304.04332v4)
implements type unification with relational injectivity rules over fresh IDs,
including occurs-check demands and incompatible type checks. Constructors and
application analyses participate in one relational system. The paper explicitly
excludes backtracking, allowing a nonpersistent union-find. Thus it is a direct
precedent for eliminating an external term-unification service, not evidence that
conditional fusion or branch failure is already solved for this project.

[Eqlog's evaluation algorithm](https://www.mbid.me/eqlog-algorithm/) describes
relations over numeric identities, equality-class merging, normalization and
matching. The [project implementation description](https://github.com/eqlog/eqlog#data-model-and-algorithms)
explains functionality axioms and canonical-row/index maintenance after merges.
This provides an integrated relational implementation control and highlights
incidence-update cost. Distinguished low-level equality machinery is compatible
with eliminating a higher-level unifier boundary. Monotone closure does not supply
consumable CHR resources or explicit search automatically.

[Colored E-Graphs, §§III–IV](https://arxiv.org/pdf/2305.19203) shares base data with
contextual equality overlays. Its premise that merely containing a term asserts
no judgment does not transfer wholesale to source constraint occurrences.
[Towards Relational Contextual Equality Saturation, §4](https://arxiv.org/pdf/2507.11897)
studies context-indexed equivalence relations and discusses representation
questions as ongoing work. These motivate shared conditioned equalities and
relational matching; neither is an established complete CHR implementation.

[Optimal Union-Find in CHR](https://arxiv.org/pdf/cs/0501073) implements rank and
path-compression techniques through CHR rules, with execution/index assumptions
for its complexity account. It is a serious alternative to explicit equality
closure, not a reason to presume all relational implementations inexpensive.
[Graph transformation embedded in CHR](https://arxiv.org/pdf/1006.1497) proves
correspondence for typed graph transformations subject to its graph invariant.
The translation direction and graph conditions matter; it is not a ready proof
of this project's compiler.

[Strategic Port Graph Rewriting, §2](https://arxiv.org/pdf/1407.7929) makes ports,
matching and reconnection explicit. It is relevant to local incidence and argument
roles; equality, finite-tree consistency and source-choice correlation require
additional rules or arguments.
[Wolfram's multiway definition](https://www.wolframphysics.org/technical-introduction/the-updating-process-in-our-models/multiway-systems-for-our-models/)
represents possible update paths and merges hypergraphs by isomorphism. Directly
turning every competing CHR match into a multiway branch would change this
project's explicit-only search contract. Event/graph representations remain worth
investigating, but whole-state isomorphism does not establish sharing between
states that remain different.

## Project-derived distinctions and obligations

The following analysis is ours. Separate four contrasts:

1. Representation: a transactional graph solver versus the existing shared arena.
2. Boundary: the same relational data used through a solver call versus integrated
   consistency/application rules and matching dependencies.
3. Conditional sharing: separate persistent contexts versus supported facts or
   contextual equality classes.
4. Compilation: general joins versus specialized incidence or port rewrites.

The first E18 semantic entry uses ordinary equality facts and relational axioms in
a finite generic closure kernel. It deliberately does not need a privileged
union primitive to establish denotation. Explicit closure can be expensive;
union-find, conditional overlays and specialized joins remain required competent
controls before an efficiency conclusion. Existing T011 elementary conditional
worklists remain relevant: the previous portfolio was not limited to whole-term
reuse.

Interpret F(root, children...) as a constructor description. Root equality plus
the same symbol entails corresponding child equality; equal children imply equal
root values; differing symbols/arities at equal roots refute. Positive constructor
paths must be acyclic after quotienting by equality in each live context.
Unspecified classes denote unbound variables. Descriptors survive consistency
propagation. Equality must affect every application value column and matching dependency,
not only constructor relations. Head/guard queries inspect established descriptors and equality;
they must not post structure to make a match succeed.

For a finite saturated equation-only database, the proposed correspondence is:
equality classes have either no descriptor or one constructor shape modulo child
equality; absence of a positive quotient cycle permits recursive finite-tree
interpretation. Give each undescribed class a distinct free variable. This yields
a solution. Generated equalities follow necessary free-constructor consequences,
so no unsupported variable identifications are introduced. A complete proof and
executable adversarial checks must justify the most-general interpretation;
agreement on ground outputs alone is insufficient.

The first finite monotone fragment does not yet prove correspondence for CHR
consumption, propagation histories, dynamic fresh variables, recursive explicit
choice births or general fairness. Those are subsequent executable obligations,
not grounds to abandon the direction. User-constraint occurrence identity must
remain distinct from value identity. Trusted extraction must await relevant
consistency and source obligations; an intermediate descriptor is not an answer.
