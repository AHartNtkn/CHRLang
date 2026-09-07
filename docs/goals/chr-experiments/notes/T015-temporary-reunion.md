# A first temporary-reunion gate must preserve its comparison selector

Temporary regions remain a feasible independent experiment. The first gate should
retain complete live regional states and reunite them before final observation.
The permanent-factor implementation's answer products do not implement this boundary.
This note sharpens the [existing protocol](../../chr-sharing/notes/T017-temporary-decomposition.md);
it records an analytical proposal, not executable evidence or a production restriction.

## Head validity alone is insufficient for the fixed control

Consider these experimentally ordered rules and query `p, r`:

```text
p <=> q.
q, r <=> hit.
r <=> miss.
```

Two snapshot proposals can consume disjoint old occurrences, p and r. After the
first proposal posts q, the r occurrence still exists, but the earlier join now
wins under the fixed pending-first scalar selector. Committing the old r proposal
can produce q and miss instead of hit. Rechecking its heads, guards and propagation
token alone does not establish selector fidelity. That alternative schedule may be
permitted ordinary CHR; this counterexample concerns the experimental comparison.

An interface marker alone is also insufficient. Consider:

```text
startA(X) <=> readyA(X), a(X).
readyA(X), readyB <=> X =:= b.
a(b) <=> changed.
a(X) <=> single.
startB <=> readyB.
```

With query `startA(X), startB`, pending-first scalar execution consumes a(X) to
single before startB runs. Stopping A when readyA appears and reuniting immediately
can bind X to b first and produce changed. There is no competing consumer of
readyA, so a sole-consumer condition does not resolve this case.

## Sufficient premises for an initial restricted gate

Classify pre-reunion rules as private to one region or as the single joint interface
rule. Private heads, bindings and fresh variables are region-local; there are no
shared writable aliases. For this matched scalar control, place every private rule
before the joint rule, retaining ordinary pending-goal priority. Accept a regional
boundary only when its pending queue is empty and no private rule applies anywhere
in its complete store, with ordinary guard and propagation-history checks.

Keep the full state, including suspended constraints, substitutions, occurrence
identities, history and freshness. Those constraints can wake after the join. Form
compatible combinations of boundary states, rename private identities apart, and
resume ordinary scalar execution. Boundary states are not answers. Continue the
regional producer streams and distinguish exhaustion from a producer that has not
yet produced a state.

Under these premises, the joint rule cannot preempt enabled private work in the
matched scalar execution. Other regions cannot disturb private quiescence before
reunion because they cannot change that region's bindings or resources. Private
transitions commute across regions. The implementation still needs an independent
full-answer and continuation gate; this argument is not a fairness or cost result.
Supporting other priorities requires a stronger dynamic boundary or an applicable
confluence argument. The rule ordering here is an experimental control, not a
proposed language requirement.

## What to measure and try to falsify

Use explicit alternatives in both private regions, followed by a cross-region join.
Compare ordinary scalar search with private evaluation plus reunion. Vary private
work, alternative counts and join selectivity; include zero private work and an
early join as negative controls. Charge boundary detection, retained state, variable
and occurrence renaming, product enumeration, join execution and exact observation,
as well as private work avoided.

Correctness cases should cover both counterexamples above as rejected or safely
handled inputs, a second interface consumer, shared writable aliases, failing pairs,
equal outputs with different residual occurrences, duplicate lineage, propagation
history and fresh variables. Check a finite result alongside an infinite regional
producer separately from exhausted finite-answer coverage. Permanent factoring,
this restricted gate, and general dynamic reunion remain distinct questions.
