# Language choices that enable different optimizations

The investigation supports several independent design opportunities. They should be evaluated separately: local dispatch, variable ownership, logical solver semantics and bounded service execution remove different costs. No proposal below has been adopted as a language rule.

## Closed relational declarations with explicit alternative cases

A possible source construct explicitly declares that a collection of relational cases forms a disjunction, then elaborates it into a catch-all CHR rule with RHS equations and OR. Ordinary CHR rules retain committed choice. The declaration itself is the opt-in to search; competing ordinary rules never acquire implicit alternatives.

This can make familiar logic-program definitions ergonomic while giving the compiler a closed case set. In particular, the compiler can reject impossible cases when a constructor is known, yet retain explicit alternatives when the argument is unknown. Replacing the definition by disjoint constructor heads that merely suspend on unknowns would lose backward synthesis.

Addition illustrates the distinction. Its catch-all rule explicitly chooses either X=z with Y=Z, or X=s(X1), Z=s(Z1) and recursive addition. A query with unknown X can take the explicit choice. A head-only rule `add(s(X),Y,s(Z)) <=> ...` cannot instantiate an unknown query argument through matching. Any ergonomic relational syntax must preserve that difference in its elaboration.

The opportunity is case specialization, fewer failed dynamic matches and occurrence-local expansion. The cost is an additional declared form and a specified elaboration, not necessarily a loss of relational modes. The alternative is to write the catch-all RHS disjunction directly and have the compiler infer the same closed cases. Surface syntax remains an owner/interface decision after the semantics are understood.

## Region closure instead of a whole-language nonoverlap rule

A region containing only unambiguous single-headed simplifications can use the relation graph or specialized net controllers. Predicates must have no undeclared external head interactions. Shared logical variables may cross the boundary through a binding service; source occurrences cannot be invisibly consumed across it.

Declaring that restriction globally would reduce general CHR expressiveness. Inferring it locally preserves multiheaded solvers elsewhere but adds a compiler and interface obligation. Explicit region annotations make the boundary reviewable and can support separate compilation. A whole-program analysis can be less burdensome for small isolated programs but may need to reconsider eligibility when rulesets are linked.

This is a real tradeoff, not a presumption that maximal compatibility or maximal restriction is best. The early arithmetic/evaluator relations look eligible, while the notebook's norm/var interactions require a region containing their joins or a separate solver interpretation. A restriction selected only from addition would miss that cost.

## Distinguish immutable terms from writable logical-variable identity

Immutable constructor graphs are safe to share widely. Logical-variable bindings and constraint occurrences have different update and lifetime rules. A type or ownership discipline distinguishing these categories can avoid copying known immutable structure while preventing unsafe duplication of an unknown variable cell.

A strong global affine discipline would require explicit treatment of repeated uses, including S's duplicated argument. That need not mean copying the whole argument: a shared immutable reference or a logical handle can be explicitly duplicated while its binding remains centralized. An inferred uniqueness certificate can optimize selected paths without forbidding other aliases.

Finite-tree cycle checks can also be elided where disjointness, current freshness or another proved property establishes safety. These certificates preserve finite-tree semantics. They should not be tied to a mandatory forward-only calling mode unless the owner accepts the corresponding loss of general relational use.

## Logical constraint regions with formula observations

A declared logical region can treat its obligations as conjunctions of formulas, permitting idempotence, entailment caching, exact projection and learned inconsistency certificates. The no_c and name/normal-form interpretations give concrete candidate contents. Ordinary CHR outside the region can retain consumable multiset occurrences.

The programming cost is that external rules cannot count or consume internal solver obligations as ordinary resources. Observing a projected formula also differs from observing every residual CHR occurrence. A baseline-preserving alternative keeps those occurrences and memoizes exact operation results. It may save less formula manipulation but requires no new residual semantics.

The complete finite-tree/regular formula procedure in T016-exact-solver-projection.md makes this a concrete language/execution tradeoff. It does not justify adding all solver consequences as host constraints or enumerating solver formulas as implicit source choices.

## Certificates for finite services, without bounding the whole search

A declaration or inferred proof that a primitive service terminates on each finite snapshot can support fair scheduling. It need not prove that recursive source execution terminates or that a synthesizer has finitely many answers. Structural recursion on finite terms is one certificate; sized/resource types can express stronger quantitative information where useful.

Requiring a finite constructor domain or global synthesis-depth bound would make many analyses and tables simpler but change the target: unary arithmetic and program enumeration would become bounded. Local bounds on one solver call or on an experiment do not impose that language-wide cost. Distinguish service termination from whole-program termination in both syntax and diagnostics.

## Explicit scheduling regions for stronger local guarantees

An owner could choose a construct that promises a deterministic local rule policy or a finite transaction boundary. This can simplify batching and repeatable traces in that region. It also makes control part of the source interface and can constrain optimizations, so the current unordered CHR baseline does not require it.

Alternatively, the compiler can prove that selected Apply transitions commute and choose their order freely. That certificate has no source-order cost but may be unavailable for competing consumers. A general runtime can still choose a permitted schedule without searching those conflicts. Neither strategy justifies forcing explicit disjunction on programmers merely because a program is nonconfluent.

## Future streams with finite terms

Finite terms do not prevent a request-driven stream protocol. A finite stream-state constraint can react to an explicit request, return the next element and a next-state handle, then suspend without another request. A finite consumer can reach quiescence after making finitely many requests. This differs from an eagerly active infinite producer, which would prevent whole-branch quiescence.

For the natural-number stream, the finite state can carry the current unary number and advance it on each request. No finite cyclic term need represent the entire infinite sequence. A future continuation/query interface would determine how a user requests later elements. This remains a future application rather than a reason to alter the initial finite-tree decision or to delay the current research.

## Decision readiness

These proposals establish distinct mechanisms, affected programs and semantic costs. Adoption of a global restriction, explicit module boundary, relational syntax, or projected-answer contract belongs to the owner. Research can still compare inferred versions and concrete implementations without committing to a surface design.

The evidence currently supports retaining these alternatives for evaluation. It does not support selecting a mandatory bundle of restrictions or declaring the most-developed implementation candidate the default architecture.
