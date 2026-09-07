# R04: exact finite consistency compilation and independent oracle

A finite consistency fragment can be compiled directly to a relation without representing CHR execution steps. Every variable must receive its value through a source choice or supplied equation. An independent exhaustive oracle now checks complete observations for this fragment; the subsequent [source gate](R04-source-correspondence-gate.md) also passes. No solver performance comparison has run.

This advances [R00's selected direct-solving question](R00-architecture-selection.md): can eliminating rule execution repay compilation and answer-recovery costs? The fragment provides a concrete correspondence and a correctness control. It neither selects a solver architecture nor establishes that arbitrary CHR execution can be eliminated.

## Source fragment and observation

Use domain atoms `a0`, `a1`, `a2` and exactly these rules:

```text
choose(X) <=> (X =:= a0) | (X =:= a1) | (X =:= a2)
given(X,A) <=> X =:= A
forbid(X,Y,A,B) ==> Equal(X,A), Equal(Y,B) | fail
```

`Equal` is the existing nonbinding equality-entailment guard. The `|` after the guards is the CHR guard/body separator; parentheses in `choose` separate explicit alternatives. A forbidden occurrence remains residual unless its guard becomes true, in which case the branch fails.

An eligible query contains only these constraints, with variable or domain-atom endpoints. Supplied values and forbidden values must be ground domain atoms. Every variable mentioned anywhere, including selected outputs, must occur in a `choose` or `given`. Selected outputs have unique names and variable endpoints; distinct names may select the same variable. Repeated occurrences are allowed. No additional rules or structured endpoints are covered.

An uncovered variable makes a query unsupported by this compilation. It does not establish source failure: the source may have a legitimate nonground residual answer. Requiring this coverage is an experimental compiler eligibility condition, not a proposed language restriction.

The observation contains the ordered selected values and every original forbidden occurrence after substitution. Residuals form a multiset: sorting is allowed; suppressing repeated occurrences is not. Repeated output aliases retain their selected positions. Variables absent from both outputs and residuals still contribute explicit alternatives to the raw completion count.

## Direct relation and correspondence

Associate one domain variable with each distinct query variable. Conjoin each supplied equality and each condition `not (value(X)=A and value(Y)=B)`. Domain values are authorized by the source's explicit choices or fixed by supplied equations. No solver variable selects a CHR schedule.

Each satisfying assignment determines exactly one compatible arm of each chooser. All supplied equations succeed. No forbidden guard can become true, since established bindings agree with the final assignment. Consuming the producers therefore yields a successful terminal observation with the substituted forbidden multiset. Repeated choosers have one compatible arm each, rather than multiplying successful completions for the same assignment.

Conversely, a successful terminal source state must have consumed all choosers and supplied equations. Coverage makes every variable ground in the domain. No forbidden tuple can hold: its rule would otherwise remain enabled and fail. The resulting assignment consequently satisfies the relation.

The fragment terminates under any legal rule-selection order: each producer is consumed once, its body creates only finitely many equations and alternatives, and a forbidden firing immediately fails. There are no recursive births or repeated successful propagation effects. Thus the correspondence does not rely on enumerating scheduling alternatives or imposing a textual winner.

Raw successful completions equal satisfying assignments. Distinct returned observations can be fewer: `choose(X)` with no selected output or residual has three raw completions and one empty observation. Selecting every variable makes the observation map injective. These counts describe logical source alternatives, not diagnostic counts of solver operations.

## Executable independent control

[oracle.py](../../../research/chr-direct-solver/oracle.py) enumerates the Cartesian product of domain values, checks supplied values and forbidden tuples directly, and constructs complete observations. It imports no CHR matcher, unifier, candidate engine or solver. A Python string denotes a variable; integer `0`, `1` or `2` denotes a domain atom. Query fields are `choose`, `given`, `forbid` and `outputs`; unsupported fields are rejected.

The oracle returns variable order, satisfying assignment tuples, raw count and deduplicated observations. Its default eight-variable resource bound raises `OracleLimit`, separately from `Ineligible`. A bound is a limit of this exhaustive control, not evidence that a query fails or that larger instances cannot be compiled. This module is a controlled experimental API, not a general source parser.

[Twelve focused tests](../../../research/chr-direct-solver/test_oracle.py) pass. They check analytically derived graph counts, full selected values, residual multiplicity, repeated choosers, unobserved alternatives, conflicting givens, self-edges, ground queries, supplied queries without OR, unsupported input, source output eligibility and the oracle bound. The initial stub failed the substantive semantic expectations before implementation. A further test exposed silent acceptance of unsupported constraints; the oracle now rejects those inputs. Independent review confirmed the correspondence and identified output-eligibility mismatches; constant output endpoints and duplicate labels are now rejected, matching the source query API.

For an inequality edge `(X,Y)`, post the three forbidden equal-value pairs. The four-variable cases have independently derived counts:

| Network | Forbidden occurrences | Satisfying assignments | Reason |
|---|---:|---:|---|
| Path 0–1–2–3 | 9 | 24 | Three first values and two choices for each successor |
| Self-loop at 0, edges 1–2–3 | 9 | 0 | The loop excludes every value at 0 |
| Complete graph except edge 0–1 | 15 | 6 | Vertices 2 and 3 differ; 0 and 1 take the remaining value |
| Complete graph | 18 | 0 | Four mutually different vertices require four values |

Reproduce the focused checks from the repository root:

```sh
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-direct-solver -v
```

These focused checks validate the oracle. The subsequent [registered source gate](R04-source-correspondence-gate.md) compares complete observations against the reference executor. Timing must exclude this exhaustive oracle.

## Next decision and limitations

A prospective comparison should contrast competent incremental execution and direct constraint solving on sparse/dense, satisfiable/refuted and supplied/choice queries. Both may fold givens, index forbidden pairs, propagate domain support, simplify impossible tests, decompose components and retain a template for residual reconstruction. Any difference in those facilities needs attribution; a solver must not receive free preprocessing denied to its control.

A direct solver can internally suppress redundant checks while preserving the original residual template. It must charge preparation, model enumeration, duplicate recognition, reconstruction and storage. Sparse deterministic supplied queries are an adverse regime for global solving; tightly coupled choices are a plausible favorable regime. Neither prediction is a measurement.

The source comparison is recorded separately; a native solver/control comparison still requires selection and its own frozen registration. R01 need not succeed for eligibility or source correspondence to proceed. Ranking against compiled execution does require a relevant compiled control, or an explicit limitation on that claim.

This fragment does not answer questions about recursive derivations, consuming resource conflicts, nonground solver answers, general guards or search fairness with infinite branches. Extending it is justified only where that extension could change the architecture decision. The current bounded result is exact finite eligibility and an executable independent oracle; R04 remains open.
