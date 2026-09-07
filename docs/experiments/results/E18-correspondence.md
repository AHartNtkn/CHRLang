# E18 finite relational correspondence argument

This is a written argument for the finite equation/descriptor fragment implemented
by the first gate, not a mechanized proof of CHR execution. Kernel tests and the
generated oracle comparison check its implementation separately. Let C be the
finite explicit context set and U the finite ID universe. Each fact has support
in C; projection retains facts whose support contains the selected context.

Generic conjunctive rule application intersects premise supports. Multiple
proofs union conclusion supports. Therefore projection commutes with every
positive rule deduction: a supported conclusion is derived exactly in contexts
where a corresponding ground rule instance has all premises. Fair finite
fixpoint iteration yields the same least closure as separate scalar deductions.
This is a semantic sharing construction, not a cost claim; scanning all joins may
cost more than independent specialized algorithms.

For a projected equation-only input, assign a finite first-order term to every ID.
A constructor fact F(r,x1,...,xn) asserts value(r)=F(value(x1),...,value(xn)); eq
asserts equal values. The supported input equations are lowered to these facts.

Every generated equality rule preserves all such assignments: equivalence rules
are valid for equality, injectivity follows from free constructors, congruence
from substitution, and transport preserves a relation's value columns. Different
constructor symbols or arities cannot describe the same term. Every reach fact
contains at least one constructor edge; equality only moves its endpoints and
transitive composition concatenates positive paths. Thus a reach(x,x) fact would
make a finite term its own proper subterm. Each generated bad fact is consequently
sound evidence of no finite solution for that context.

Conversely, suppose closure completes without bad. Quotient U by eq. Reflexivity,
symmetry and transitivity establish equivalence classes. Injectivity and clash
rules ensure all descriptors of a class have the same symbol/arity and equivalent
corresponding children. Congruence also covers nullary constructors. Positive
reachability rules ensure that the quotient constructor graph is acyclic.
Associate each undescribed class with its own distinct variable. In reverse
acyclic order, interpret each described class as its constructor applied to its
children's interpretations. This satisfies every input descriptor and equation.

For any other satisfying assignment, map each variable of an undescribed class
to that class's assigned value. Induction over the acyclic graph shows that applying
this substitution to the constructed interpretation gives that assignment on
all IDs. Thus the interpretation is most general; joint reification of selected
roots may alpha-rename those variables without changing their relationships.
Failure soundness plus this construction establishes the equation fragment's
success/failure completeness, assuming the implemented closure reaches its least
fixed point and faithfully implements the rules.

Application relations use explicit value-column schemas. Equality transport on
an occurrence-identity column would conflate source resources and is forbidden
by that schema; the initial kernel does not infer column responsibilities.
Ordinary monotone application rules can participate in the same closure and
introduce equations/descriptors. The preceding interpretation applies to the
resulting accumulated equation store. It is not a claim that every grounding of
an unbound residual application fact will remain successful: later bindings can
enable a rejecting rule.

The generated gate does not prove that destructive CHR steps commute with partial
consistency deductions, that propagation fires exactly once, or that recursive
freshness/choice births preserve this projection. Those require a separate
source-level correspondence and adversarial execution gate. Neither a solved
substitution at every microstep nor a complete-unifier call is assumed here.
