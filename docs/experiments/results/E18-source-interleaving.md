# E18 source interleaving: a local eligibility argument

For the current equality-entailment guard fragment, exposing partial equality
deductions need not require a complete-unifier call boundary. A local reordering
argument is available. It is not yet global CHR correspondence, confluence or
fairness evidence.

Let B be established bindings and E an already-posted finite equation in one
context. Suppose some deductions from E enable application A and completing E
consistently extends B to B'. A nonbinding constructor match remains valid under
this extension: substitute the extension into its rule-variable assignment.
Positive equality entailment also persists. Completing E does not consume head
occurrences or create a propagation token for A. The same live distinct tuple
therefore remains eligible after E completes, and its body is the corresponding
substituted instance modulo fresh-variable renaming.

This permits the local order “complete E, then A” under a source semantics that
allows A's selection. A newly enabled competing rule does not make A ineligible.
The argument does not require confluence or identical reference FIFO traces.
It establishes a permitted local execution, not equal final outcomes for every
schedule. No implementation obligation to materialize B' or call a unifier follows
from using B' in the proof.

## Why nonbinding purity alone is insufficient

Consider a hypothetical read-only var guard and atomic complete-equation source
semantics:

```text
query: p(X,Y), go(X,Y)
go(X,Y) <=> pair(X,Y) =:= pair(a,b)
p(a,Y)  <=> var(Y) | hit
```

Before equality, the constructor head cannot match unknown X. After full equality,
Y=b makes var(Y) false. An implementation exposing X=a while Y is still unknown
can fire p and produce hit, which this atomic source execution cannot produce.
The failing premise is substitution stability, not absence of side effects.
[Christiansen and Kirkeby's confluence paper](https://arxiv.org/pdf/1611.03628)
explicitly discusses the failure of logical substitution/subsumption reasoning
for nonlogical built-ins such as var/1.

Current `chr_syntax::Guard` contains only finite-tree equality entailment; this
counterexample is outside that implemented fragment. It does not silently extend
or restrict the source language. If state-inspection guards are investigated,
compare certified stable regions or suitable observation/completion protocols
with an explicit operational contract. Even then a guard's stability requirement
need not impose a callable whole-term solver on every operation. Guard-domain
adoption remains an owner decision; current positive-fragment work can proceed.

## Remaining compositional obligations

E must already have been posted in the context. It cannot move before its producer
or before its branch exists. Overlapping applications still need legal occurrence
consumption and propagation-once handling; equality monotonicity does not settle
those conflicts. Fresh allocation must be related by renaming rather than numeric
sequence equality. Inconsistent contexts must fail, with no trusted answer or
effect on incompatible contexts. Finally, a finite successful reordering argument
does not establish that consistency/failure work progresses or that answers are
covered fairly.

The next integrated source gate should therefore combine a structural head
initially blocked on an unknown argument, explicit choices introducing finite
constructor relations, a simpagation consumer, propagation with repeated equality
wakeups, fresh rule-local variables, and one cyclic failing context. Compare
permitted outputs and complete residual multiplicities under forced internal
interleavings. Keep the independent reference unchanged. This is an executable
next obligation, with no blocker requiring a new owner decision.
