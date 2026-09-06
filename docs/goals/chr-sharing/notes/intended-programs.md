# Intended programs: relational arithmetic and program synthesis

The language should express standard logic programs early on, followed by evaluators that can also synthesize programs. These owner-supplied examples establish programming goals and application workloads. They do not establish where computation can be shared or how much an implementation would save.

The owner supplied these examples on 2026-09-06 and confirmed their interpretation: evaluate and synthesize through the same relational definition, combine recursive relations with constraints, return constrained partial programs, and request further solutions. Syntax may differ from rwlog. These examples inform the research without bounding the design space.

## Relational addition and subtraction

Using the owner's illustrative notation, with the recursive call made explicit:

```text
add(X, Y, Z) <=>
    (X =:= z & Y =:= Z)
  | (X =:= s(X1) & Z =:= s(Z1) & add(X1, Y, Z1)).

sub(X, Y, Z) <=> add(Y, Z, X).
```

The supplied successor arm omitted `add(X1, Y, Z1)`; the version above records the intended recursive addition, not an adopted surface syntax. The unification equations are in the body, and the alternatives are explicit. Constructor discrimination occurs through those equations, so this example does not require competing rule heads to introduce search.

The same addition relation should support forward addition, finding a missing operand, and enumerating decompositions of a specified unary natural. Subtraction demonstrates reuse by rearranging relation arguments. For example, `add(s(z), s(s(z)), Z)` has `Z = s(s(s(z)))`; `add(X, Y, s(s(z)))` has the three natural-number decompositions. These are mathematical expectations, not executed tests.

The relation does not independently enforce that every argument is a natural: the base arm equates Y and Z without constraining their constructors. Arithmetic examples therefore use unary-natural inputs; adding domain predicates would be a separate program choice. This does not block their use as the initial unification/disjunction demonstration.

## SK evaluation and synthesis

The owner supplied rwlog `skEval`, `skFold`, and the constraint theory `no_c`. The representation uses `a` for application, `k` and `s` for combinators, `c` for opaque test symbols, and a spine during evaluation.

The forward demonstration evaluates `S K K` applied to a test symbol and returns that symbol. The synthesis demonstration seeks a term D satisfying the displayed behavior `D x y = x y y`, while `no_c` prevents the synthesized term from containing the test symbols. The supplied first result is `S S (S K)`.

Further supplied results contain unbound holes with residual `no_c` constraints. Those holes are part of successful answers, not evidence that the search must finish constructing a ground program before answering. For example, a K wrapper can make an argument irrelevant to the result while the residual constraint still limits what may later fill it.

This workload exercises recursive term construction, variable sharing, constraints on partial terms, explicit alternatives, failure, and incremental answer enumeration. Candidate terms may diverge during evaluation. Search must therefore be assessed for progress on other alternatives, rather than assuming each candidate evaluation terminates.

The synthesized programs are distinct answers even when they exhibit the same requested behavior. Deduplication must not identify two different returned program terms merely because they compute the same function. The existing requirement to recognize equivalent answers, including residual constraints and variable renaming, remains controlling.

Potential sharing questions include reuse of evaluator work across related candidate programs and propagation of common constraints. These are hypotheses to investigate. Similar-looking answers, compact term graphs, or repeated source expressions alone do not show that runtime computation is shared.

## Lambda rewriting

The owner also identified [the rwLog-Rust Lambda notebook](https://github.com/AHartNtkn/rwLog-Rust/blob/master/examples/Lambda.ipynb) as an intended translatable example. Its source cells define `neq`, `var`, and `norm` constraints, a one-step relation `lamRW`, and a recursive relation `lamEq`. It includes queries with partially specified terms and requests for further results.

The notebook adds recursive rewriting, disequality and normal-form constraints to the intended workload coverage. A future translation must preserve the source program's meaning, including its treatment of variables and substitution. No defect in the source evaluator has been established, and this is not an immediate owner decision.

Retrieval: the GitHub page and raw URL failed through the web tool; the public [raw notebook](https://raw.githubusercontent.com/AHartNtkn/rwLog-Rust/master/examples/Lambda.ipynb) was successfully downloaded and its source cells inspected on 2026-09-06. The notebook was not executed. The mutable `master` link must be pinned before using it as a reproducible experimental input.

## Consequences for candidate comparisons

These are research assessments of programming cost, not newly adopted language restrictions:

- **Modes and demand analysis:** compare inferred specialization with compulsory input/output modes. Requiring a separate hand-written inverse relation would materially affect the demonstrated reuse; compiler-generated specializations may preserve it.
- **Fresh variables and range restriction:** addition's successor arm introduces fresh variables, and synthesis constructs unknown program structure. A restriction must explain how these programs are expressed or what is lost. A restricted solver fragment need not imply a restriction on the entire language.
- **Nonoverlap and local compilation:** ordinary logic-program clauses can be represented as explicit alternatives within a relation rule, as addition illustrates. Investigate whether the resulting program satisfies the precise compiler condition; neither rule-head uniqueness nor the example alone proves eligibility.
- **Groundness and residual answers:** requiring fully ground results would exclude the supplied partial-program answers. A proposal must account for that cost; answer extraction cannot discard a constraint merely because its variable does not affect the demonstrated evaluation result.
- **Fairness and primitive operations:** the evaluator itself can be expressed recursively in the language. Assess whether sharing and scheduling allow other alternatives to progress while one diverges, including work performed between choice points.
- **Persistent facts and consumable resources:** the pruning constraints motivate examining a certified persistent fragment. They do not establish that all constraints in the language can use set semantics.

The next semantic comparison should apply these application requirements alongside the conditional-consumption challenge in the research dossier. Restrictions may still be proposed, but their reformulation and expressiveness costs must be explained in terms of these programs as well as abstract examples. Neither implementation nor benchmark design is authorized by this workload record alone.
