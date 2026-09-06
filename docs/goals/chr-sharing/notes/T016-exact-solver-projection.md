# Exact projection needs formulas as well as tree languages

Finite-tree equality, disequality and regular structural properties have a source-backed route to exact logical summaries. A summary may still require existential variables and Boolean structure. Compressing every projected relation into one regular tree language would be unsound.

## The source result and its boundary

Comon and Delor, *Equational Formulae with Membership Constraints*, §2 Definitions 1 and admissibility, §2.2 Lemma 1, and §7.1 Theorem 4, gives terminating reduction to equivalent finite disjunctions of solved forms. The conditions include a finite ranked constructor signature, effective decomposition of membership by constructor, and decidable finiteness with enumeration of finite sorts. Regular tree languages supply the required interpretation. Solved forms can retain existential variables; “quantifier elimination” must not be read as a promise of one quantifier-free formula in our current surface syntax. [Author abstract and paper link](http://lsv.ens-paris-saclay.fr/~comon/membership.html)

The full author PostScript was retrieved from its linked `ftp.articles/membership.ps`, converted locally, and the sections above inspected in `/tmp/chr-membership.txt`. This resolves the narrow availability/conditions question raised during T015's solver analysis. It does not transfer a performance claim or a CHR observation theorem.

## A concrete candidate interface

Represent a solver region's state by a finite formula over constructor equations, disequations, and memberships in declared regular tree languages. Keep public variables U and freshly scoped private variables W explicit. Return an equivalent solved formula for `exists W. Phi(U,W)`. An answer table stores that formula, its domain signature and variable interface, not a collection of guessed ground instances.

N for the declared SK domain is a regular language. On the declared lambda syntax, normal and neutral terms are mutually recursive regular languages, and the name property is regular. The notebook's neq interpretation additionally uses disequality. Their certified logical versions therefore fit this candidate once their signature assumptions and closed-region boundary are satisfied. This statement is about the interpretations in T015-solver-certificates.md; the original CHR residual multiset is not thereby replaced.

With exact formulas, reuse can be checked as follows: freshen private variables, conjoin the caller's own restrictions, and ask whether the combined formula is inconsistent or entails a proposed consequence. Equivalence of two summaries can be checked by testing unsatisfiability of their symmetric difference in the same declared theory. This proves only logical summary equivalence, not equality of source alternative multiplicities or of host continuations.

## Three projection examples

For `exists H. N(H)` over SK, the result is true because k is a witness. For `exists H. P = a(k,H) and N(H)`, the result denotes applications with k on the left and an N-constrained right child; a sort expression can represent it.

For `exists H. P = a(H,H) and N(H)`, a regular language for P alone is insufficient when N is infinite. Suppose a deterministic finite tree automaton recognized exactly those terms. There are infinitely many distinct N-terms but finitely many states, so distinct t and u reach the same state. Acceptance of a(t,t) then implies acceptance of a(t,u), contradicting the required equality of children. The summary must retain the shared witness/equality or use a richer relational constraint representation.

This third example is directly relevant to synthesized programs with repeated holes. A tree automaton can summarize allowed structures, while equality preserves correlations between repeated uses. Replacing one by the other loses information even though both deal with finite trees.

## Domain decisions are material

The theorem's finite signature does not mean a finite set of terms: recursive constructors already generate infinitely many finite terms, including unary naturals. Thus this route does not require bounding arithmetic values or synthesis depth.

An unbounded supply of atomic names needs a declared treatment. One option is a finite backend encoding of names as tagged strings or naturals, preserving source atom equality through an injective representation. Another is a separate name theory with a proved combination interface. Neither can be assumed merely from the notebook's use of letters. The regular-language transfer must use the actual encoded domain, including any validity restrictions on name codes.

Unknown extra constructors also matter. A no_c rule set that leaves them residual is not equivalent to rejecting them. A solver region can declare its domain explicitly; an inferred certificate needs evidence that callers stay in that domain. These options should be explained before an owner adopts the region semantics.

## No new source search is required

A Boolean formula inside the solver records a relation. Its normalization may distribute or combine logical disjunctions without creating source search alternatives. It may rule out existing alternatives using a valid inconsistency certificate. Enumerating assignments from the formula is a distinct operation that must be explicitly authorized by the source language.

Likewise, a summary's logical idempotence does not erase repeated host constraint occurrences. A baseline-preserving memo service can cache internal proof work and return the exact source operation's effects. A declared logical region can instead choose formula semantics for its own obligations. The owner-visible difference is residual meaning and permitted interactions with host rules, not merely an implementation switch.

## Disposition

The existence of a complete logical summary procedure for this declared finite-tree/regular fragment is sufficiently supported by the inspected source conditions and the translations above. There is no need to wait for a conditional-store experiment to answer that question. The regular-language-only proposal is refuted by the repeated-child example.

Implementing the full formula solver may cost more than it saves. Relevant measurements are the size of solved formulas, entailment/projection effort, cache reuse, and retained constraints on synthesis workloads. Before that experiment, research still needs to choose a concrete host-region boundary and compare exact summaries with baseline-preserving operation memoization. No global replacement of CHR by this solver has been recommended.
