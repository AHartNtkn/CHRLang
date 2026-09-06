# Worked obligations for shared execution

Sharing must preserve each alternative's constraint lifetimes and propagation permissions, even when alternatives reuse the same terms and occurrence identities. The following traces distinguish those obligations. They are hand-derived analyses under [the proposed reference](T008-reference-and-candidates.md), not executed tests or performance evidence.

## Consumption and inherited history

Illustrative rules:

```text
r @ p(X), q(K,Y) ==> seen(K,X,Y).

c @ seen(old,X,Y) <=>
      (take(X,Y) & Y =:= a)
    | (keep(Y)   & Y =:= b).

d @ take(X,Y), p(X) <=> after(Y).
k @ keep(Y)          <=> after(Y).
n @ after(Y)         <=> q(new,Y).
```

Initial query: `p(U), q(old,V)`. Select the following permitted execution; the notation does not grant source order priority.

1. Introduce `p(U)#1` and `q(old,V)#2`.
2. Apply r, introducing `seen(old,U,V)` and recording token `r[1,2]`.
3. Apply c and split its body. Both children inherit the initial occurrences and token.
4. In A, unify `V=a` and use d to consume `p(U)#1`. In B, unify `V=b` and use k, retaining `p(U)#1`.
5. Each child applies n and introduces a fresh `q(new,V)` occurrence.
6. A cannot apply r: its p occurrence is absent. B can apply r with p and the new q, yielding `seen(new,U,b)` and a new token.
7. B cannot apply r again with old q: binding V does not invalidate `r[1,2]`.

The resulting quiescent stores, omitting runtime IDs, are:

```text
A: q(old,a), q(new,a)
B: p(U), q(old,b), q(new,b), seen(new,U,b)
```

U remains unbound. In B, c cannot match `seen(new,...)`; the relevant r tokens are present. A has no p, take, keep or after; B has no take, keep or after. Thus neither branch has a remaining enabled rule.

Globally consuming p would lose B's result. Globally retaining p would admit an application in A that its state does not permit. Recomputing history from current argument values could repeat r on old q. Equal logical terms never suffice to identify distinct occurrence tokens.

## History acquired after a choice

```text
split(V) <=> (V =:= a) | later(V).
later(V) <=> V =:= a.
p(X), q(a) ==> mark(X).
```

Initial query: `p(U), q(V), split(V)`. Introduce p and q before splitting. Choose a legal interleaving in which A binds V and propagates before B processes later. When B subsequently binds V, the same inherited p/q occurrence tuple must still be allowed to propagate in B. A's token cannot become an unconditional prohibition shared with B.

Both alternatives eventually have `p(U), q(a), mark(U)`. They can be recognized as equivalent final answers under the conservative equality criterion, but that does not justify suppressing B's enabled propagation while its mark is absent. An implementation may share the actual propagation operation if it accounts for each branch's prerequisites and effects; the example excludes only an unconditional global token.

## Conditional aliases and occurs checks

```text
start(X,Y) <=>
    (X =:= Y & Y =:= s(X))
  | (X =:= z & Y =:= s(z)).
```

The first arm fails the finite-tree occurs check; the second succeeds. Combining the first arm's alias X=Y with the second arm's constructor values would corrupt both meanings. Detecting the cycle must exclude only the first arm. This challenges conditional union/find or substitution structures without prescribing their implementation.

There is a complementary case: `(X =:= s(Y)) | (Y =:= s(X))`. Each alternative permits finite trees. Forgetting the choice conditions combines the dependencies into an apparent cycle. A condition-sensitive representation must not reject both merely because the union of its edges is cyclic; finite-tree consistency is checked per represented alternative.

## Context and fresh variables

`job(K), permit(K) <=> done(K)` demonstrates why equal visible job arguments do not identify a reusable computation: partner availability matters. A context key based only on shared logical variables also misses a partner connected by a ground K.

For `new_box(Tag,B) <=> B =:= box(X)`, two invocations introduce distinct fresh X variables even if their arguments and rule text look alike. Caching a template can be sound; reinstating the same live logical variable without justification is not. This distinguishes immutable syntax sharing from accidental aliasing of fresh variables.

For `coin, coin <=> pair`, two occurrences are required. A set-based language may intentionally give this program a different meaning or require explicit resource identities. That is a declared expressiveness cost, not a speedup on the same program.

## Application checks

For relational addition, constructor discrimination must come from the explicit body equations. Head matching must not implicitly choose constructors for an unknown query variable. Reusing one definition in several query modes is an application requirement; independent mode-specialized compiled code is compatible with that requirement if it preserves the relation and its successful alternatives.

For synthesis, an answer containing an unused hole H with `no_c(H)` retains that restriction. It need not enumerate H to groundness, and cannot erase the restriction merely because H did not affect evaluation. Conversely, a disconnected active constraint that can still fail prevents quiescence even if the selected output is already ground.

## How candidates use these cases

Conditional execution must show the membership, binding and token conditions for these transitions. Memoized graph evaluation must show the dependency distinctions in the cache or graph. Net implementations must identify physical ownership and any administrative communication preserving the transition. Tabled or learned-constraint candidates must state what histories, assumptions and residual conditions survive reuse.

A proposed restricted language can instead identify exactly which case it excludes, demonstrate a feasible reformulation if one exists, and assess its programming cost against the arithmetic and synthesis workloads. These cases do not require every candidate to support all baseline programs. They prevent an implementation from claiming baseline fidelity while silently changing their outcomes.
