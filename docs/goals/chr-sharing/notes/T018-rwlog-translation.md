# Relational translation of the supplied rwlog demonstrations

The demonstrations establish intended expressiveness: one evaluator used forward and backward, nonground synthesized programs with residual restrictions, and incremental enumeration. They are also realistic workloads on which to investigate sharing. Their existence does not establish that a particular shared evaluator will be efficient.

## Source and scope

The read-only source checkout is `/tmp/chr-rwlog-research-20260906`, revision `6e45ef1d62672fda8fd63bc1cb896c99f9224e3f`. Sources are [Rel IR](https://github.com/AHartNtkn/rwLog-Rust/blob/6e45ef1d62672fda8fd63bc1cb896c99f9224e3f/src/rel.rs), [meet](https://github.com/AHartNtkn/rwLog-Rust/blob/6e45ef1d62672fda8fd63bc1cb896c99f9224e3f/src/kernel/meet.rs), [composition](https://github.com/AHartNtkn/rwLog-Rust/blob/6e45ef1d62672fda8fd63bc1cb896c99f9224e3f/src/kernel/compose.rs), and the [lambda notebook](https://github.com/AHartNtkn/rwLog-Rust/blob/6e45ef1d62672fda8fd63bc1cb896c99f9224e3f/examples/Lambda.ipynb). The IR distinguishes union, intersection and composition. Meet freshens the two operands and matches both their inputs and their outputs; composition matches the first output to the second input. This justifies the relational interpretation used below. No rwlog binary or CHR prototype was executed.

## Compositional translation

Write T(E,I,O) for the constraint goal saying expression E relates I to O. E is a compiler expression here, not a new source term constructor. For the one-input/one-output subset in the demonstrations:

- An atomic span `l {C} -> r` becomes fresh copies of its local variables, `I =:= l & O =:= r & C`.
- `@t` becomes `I =:= t & O =:= t`, with the span's variable scope preserved.
- `A ; B` becomes `T(A,I,M) & T(B,M,O)` with fresh M and separately fresh local span variables.
- `A & B` becomes `T(A,I,O) & T(B,I,O)`, sharing the interface variables and freshening the two operands' local variables.
- `A | B` becomes an explicit body disjunction `T(A,I,O) | T(B,I,O)`.
- A named relation R becomes `R(I,O)` with a single linear catchall rule that expands its defining expression. Recursive calls refer to that predicate.

These clauses can be compiled away to first-order CHR predicates; no higher-order source term or impure guard is required. In particular, different atomic spans do not share variables merely because they reuse the same spelling. That detail matters in the supplied conjunctions.

Induction on finite relational derivations proves correspondence for pure spans, union, intersection and composition: endpoint unifications enforce exactly their relational interfaces; freshening preserves existential scopes; conjunction requires both derivations; disjunction supplies exactly the source alternatives. Recursion uses finite unfolding derivations. A fair scheduler can enumerate finite successful derivations without assuming that every recursive branch terminates.

Embedded CHR theories require an additional boundary argument. The original implementation normalizes constraints during composition and intersection; unrestricted nonconfluent theories cannot automatically move across these boundaries into one global store without changing permitted behavior. The no_c structural theory and the lambda predicates have the specific certificates and limitations in T015/T016. This note does not claim a translation theorem for every rwlog feature or arbitrary embedded theory. The relation algebra translation itself is concrete.

## SK evaluator as explicit relational CHR

Use `eval(I,O)` and `fold(I,O)`. All variables not in a rule head are fresh per application. Each list below is one explicit OR in the body of one catchall rule; the head remains `eval(I,O)` or `fold(I,O)`. No implicit alternatives arise from overlapping CHR heads.

For `eval(I,O)`, the arms are:

```
(I =:= p(c(N),Sp) & fold(p(c(N),Sp),O))
| (I =:= p(k,nil) & O =:= k)
| (I =:= p(k,cons(X,nil)) & eval(p(X,nil),NX) & O =:= a(k,NX))
| (I =:= p(s,nil) & O =:= s)
| (I =:= p(s,cons(X,nil)) & eval(p(X,nil),NX) & O =:= a(s,NX))
| (I =:= p(s,cons(X,cons(Y,nil))) & eval(p(X,nil),NX)
   & eval(p(Y,nil),NY) & O =:= a(a(s,NX),NY))
| (I =:= p(a(X,Y),Sp) & eval(p(X,cons(Y,Sp)),O))
| (I =:= p(k,cons(X,cons(Y,Sp))) & eval(p(X,Sp),O))
| (I =:= p(s,cons(F,cons(G,cons(X,Sp))))
   & eval(p(F,cons(X,cons(a(G,X),Sp))),O))
```

For `fold(I,O)`:

```
(I =:= p(T,nil) & O =:= T)
| (I =:= p(T,cons(X,Sp)) & eval(p(X,nil),NX)
   & fold(p(a(T,NX),Sp),O))
```

The two evaluations in the underapplied S arm are exactly the intersection-based reconstruction in the supplied example. In fold's recursive arm, the second intersection operand retains the original T and Sp while allowing X to be replaced by its evaluation result. The direct equations make that interface explicit.

Keep the supplied no_c CHR rules. Their role is to reject a synthesized program containing the test constants, while allowing residual holes constrained by no_c. A duplication query is `no_c(P), eval(p(a(a(P,c(z)),c(s(z))),nil), a(a(c(z),c(s(z))),c(s(z))))`, selecting P. An identity query fixes P to `a(a(s,k),k)` and requests the evaluator result.

The advertised ground duplication candidate is P = `a(a(s,s),a(s,k))`. On paper, `S S (S K) x y` reduces to `S x ((S K) x) y`, then `x y (((S K) x) y)`, and `(S K) x y` reduces to `K y (x y) = y`. Thus the result is `x y y`. This validates the intended combinator algebra; it is not a measured enumerator result or a claim about answer order.

Partially applied K wrappers can introduce ignored holes. Such holes remain restricted by no_c even when evaluation ignores them. Therefore all active constraints must participate in final-answer certification; output-demand evaluation alone would be insufficient. The translation is eligible for the single-headed catchall relation fragment, while no_c is a separate active structural constraint family.

## Lambda relation and binding check

The notebook's lamRW relation can likewise become one `step(I,O)` catchall rule with explicit OR. Its five beta-oriented arms translate to:

```
(I =:= app(lam(X,X),Z) & O =:= Z)
| (I =:= app(lam(X,Y),Z) & neq(X,Y) & O =:= Y)
| (I =:= app(lam(X,lam(X,Y)),Z) & O =:= lam(X,Y))
| (I =:= app(lam(X,lam(Y,Z)),W) & neq(X,Y)
   & O =:= lam(Y,app(lam(X,Z),W)))
| (I =:= app(lam(X,app(Y,Z)),W)
   & O =:= app(app(lam(X,Y),W),app(lam(X,Z),W)))
```

The three congruence arms are:

```
(I =:= lam(X,Y) & step(Y,Z) & O =:= lam(X,Z))
| (I =:= app(X,Y) & step(X,Z) & O =:= app(Z,Y))
| (I =:= app(X,Y) & norm(X) & step(Y,Z) & O =:= app(X,Z))
```

`lamEq(I,O)` becomes `(I =:= O & norm(I)) | (step(I,M) & lamEq(M,O))`. Keep the notebook's neq, var and norm theories, including their residual behavior. In particular, norm is not a complete ground normal-form decision procedure without the accompanying var occurrences described in T015.

The binding/substitution check has a precise scope: does this literal relation implement conventional capture-avoiding beta reduction, or a different syntactic rewriting relation? Take distinct ground names a and b. The displayed lambda-under-lambda arm admits

```
app(lam(a,lam(b,a)),b)
  -> lam(b,app(lam(a,a),b))
  -> lam(b,b)
```

The side condition neq(a,b) does not state that b is absent from the substituting term. Conventional capture avoidance would rename the inner binder and retain the free b. Therefore the literal notebook relation should not be labeled a capture-avoiding evaluator on arbitrary named terms. This observation does not prevent faithful translation or diminish its use as a relational synthesis workload.

If conventional lambda semantics is desired as an additional demo, choose a representation and define substitution with the required freshness or index-shifting relation. Named terms with explicit freshness and de Bruijn terms are alternative source designs; neither requires higher-order terms or rational trees. Selecting that demo's binding convention is an owner decision only if adoption is requested. It does not block the faithful notebook translation above or the remaining engine research.

## Disposition

The supplied examples now have concrete relational translations and identified semantic boundaries. Implementation is needed to validate the actual engine's answer stream, residuals, fairness and sharing costs. The next-ten ordering from rwlog is not a baseline guarantee: search scheduling remains deliberately open. A claim of full rwlog equivalence would require a separately scoped operational treatment of arbitrary embedded theories and additional language features; the intended demonstration does not require that broader compiler.
