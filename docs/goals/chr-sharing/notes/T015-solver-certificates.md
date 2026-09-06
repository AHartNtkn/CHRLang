# What the supplied pruning predicates can certify

The supplied predicates admit useful logical interpretations, but those interpretations do not authorize arbitrary rewrites of the host CHR store. Reusing a failure certificate, adding a logical formula, adding a CHR occurrence, and replacing a residual answer are four different operations.

This note derives certificates directly from the supplied SK rules and the lambda notebook. It does not assert equivalence between the complete rwlog runtime and a future CHR translation. The notebook source inspected is [Lambda.ipynb](https://github.com/AHartNtkn/rwLog-Rust/blob/master/examples/Lambda.ipynb), downloaded earlier as `/tmp/chr-lambda.ipynb`; its mutable revision must be pinned for any reproducible translation experiment.

## no_c: a finite structural constraint

Fix the intended SK term signature `k`, `s`, `a/2`, `c/1`. Define N by N(k), N(s), N(a(x,y)) iff N(x) and N(y), and not N(c(x)). A residual N(X) on an unbound variable constrains its later finite instantiation. This is not a generator or an instruction to choose a constructor.

Each supplied no_c rule preserves this formula exactly: k and s discharge true obligations, application decomposes a conjunction, and c makes it false. For any fixed finite constructor skeleton, every decomposition traverses proper children. Counting the remaining constructor nodes below the pending obligations proves termination of that decomposition. Repeated holes can yield repeated obligations; treating them as one is logical idempotence, not multiset-preserving CHR execution.

If arbitrary additional constructors are admitted, the rule set does not reject them. For example, `no_c(other)` remains residual. Therefore the definition above needs the declared SK domain. A compiler cannot infer a closed signature from these four rules alone and silently classify every other constructor as failure.

Bindings must wake residual obligations. `no_c(X)` with X unknown does not justify failure or success with that obligation absent. After `X =:= a(k,H)`, the residual restriction is N(H); after `H =:= c(z)`, the conjunction is inconsistent. The witness of failure consists of the constructor path ending at c and the equalities along that path. It does not depend on choices outside those equalities.

## var and neq: constraints on represented names

The notebook's var predicate has only two rejection rules: root `app` and root `lam` fail. Let V(t) mean that t has neither of these root constructors. Within an explicitly declared lambda signature whose remaining constructors are atomic names, V is a name constraint. Without that domain declaration it also allows any other root constructor, even one with children. It is not the host-language test “currently unbound logical variable.”

The neq rules reject equal arguments and reject app/lam at either argument's root. A corresponding declarative formula is Q(x,y) = V(x) and V(y) and x ≠ y, where inequality means inequality of eventual finite terms. Every displayed rejection is sound for Q. Unequal unknowns retain a restriction: distinct logical variable identities do not prove their future values unequal. Distinct ground names satisfy Q, but the displayed rules have no rule discharging that residual occurrence.

Under this interpretation Q implies V at both arguments. This is a useful learned formula inside a solver. It is not permission to post ordinary host `var` occurrences. From `norm(app(X,Z)), neq(X,Y)` the notebook has no var occurrence to use as a partner. Posting one would enable a norm rule and change the residual multiset. The issue persists even when the added formula is logically redundant.

## norm: validity and incompleteness are separate

For this analysis, assume the declared lambda syntax consists of atomic names, app/2 and lam/2. Define a neutral term as a name or an application of a neutral term to a normal term. Define a normal term as a neutral term or a lambda whose body is normal. Binder validity and capture-avoiding substitution are separate from this syntactic normal-form predicate.

The notebook's five norm rules preserve this interpretation when their companion var constraints have interpretation V:

1. An application with a lambda at its left root is not normal, so rejection is valid.
2. For `app(app(X,Y),Z)`, normality is equivalent to normality of `app(X,Y)` and of Z: a normal application at the left is neutral.
3. Given V(X), normality of `app(X,Z)` reduces to normality of Z, retaining V(X).
4. Normality of `lam(X,Y)` reduces to normality of Y.
5. Given V(X), normality of X is true, so norm(X) may be consumed while var(X) is retained.

These equations establish rule validity, not a complete ground normal-form recognizer. `norm(a)` for an atomic name a remains residual unless there is a companion var occurrence. Likewise `norm(app(a,b))` need not decompose. This residual behavior can be translated faithfully; silently supplying a stronger normalizer would require a different observation or solver interface.

For a fixed ground finite term and fixed companion var occurrences, the structural rules terminate. Decomposition replaces a norm obligation by obligations on proper subterms; the last rule consumes it. A finite multiset extension of proper-subterm order decreases. This does not prove termination of an enclosing synthesizer that keeps constructing new terms or adding fresh norm obligations.

The pair of regular tree languages for normal and neutral terms suggests a bottom-up finite-state analysis for ground skeletons. Holes carry admissible states and repeated holes must intersect their state requirements. This can certify failure without generating terms. It can also recognize redundant logical obligations in a declared solver region. It does not establish baseline residual equality or permit rewriting host occurrences from logical entailment alone.

## Conditions for a reusable certificate

For a region whose only operations strengthen a conjunction of these formulas and finite-tree equalities, each rule above preserves the represented ground solutions. A failure witness valid under assumptions A remains valid under stronger assumptions. Thus a cache can return `A implies false`, with local variables freshened and A checked in each consumer. A source choice need not be in A unless its selected equalities or facts are used.

The region must control all uses of its predicates. Adding a competing host rule `no_c(c(X)) <=> true` permits a committed source execution that consumes the obligation before the rejection rule. Eagerly failing every such store would lose that allowed execution. A mere inspection of the original four predicate rules cannot certify an extensible program that later adds this interaction.

A sufficient closed-region contract is: no external rule reads or consumes the region's internal occurrences; imports are explicit formulas/equalities; exported consequences remain in the logical interface; and the host observes only the declared consistency/projection interface. The host may keep its own multiset resources. This is a candidate language feature or compiler-established module property, not an adopted global set semantics.

A more conservative baseline optimization can memoize the outcome of the exact same rule/occurrence operation under the same relevant inputs, while preserving every posted and residual occurrence. It gains operation reuse without claiming formula-level answer compression. Its key must include companion occurrence availability for the norm rules and relevant binding versions. Equality of argument terms alone is insufficient.

## Projection and search boundaries

For no_c over the declared SK domain, a disconnected existential `exists X. N(X)` is satisfiable, witnessed by k. Its removal preserves the projected logical solution set. For finitely many name disequalities, an infinite supply of names can satisfy distinct unconstrained components; a finite name alphabet may not. The alphabet assumption is therefore part of any projection certificate, not an implementation detail.

Existential removal does not preserve the baseline's full residual observation, even when it preserves solutions. `answer(a)` accompanied by a disconnected residual no_c(X) differs structurally from `answer(a)` alone. The owner must choose a formula-projection interface before that becomes answer equality. Until then, certificates may assist execution while the declared residual observation is preserved.

None of these predicates needs implicit branching. An unresolved formula can remain residual. If a solver chooses to enumerate assignments or disjunctive summaries, that enumeration must correspond to explicit source alternatives or an explicitly adopted solver-search construct. In particular, a finite-state domain analysis is allowed to narrow possible states internally; it is not allowed to invent a CHR search tree.

## What remains open

The rule interpretations, fixed-skeleton termination arguments, and counterexamples above can be assessed without implementation. They establish candidate certificates for these pruning predicates, not a complete solver or a quantitative benefit.

Further analytical work should construct exact projected summaries for combinations of regular term properties and finite equality/disequality, and check whether an interface retaining ordinary residual occurrences can reuse enough work to matter. Empirical questions are table lookup, certificate construction, reuse frequency, and memory on SK/lambda synthesis. A baseline-equivalent memo service and a declared logical region are distinct candidates for those measurements.
