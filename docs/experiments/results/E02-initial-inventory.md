# E02 initial source-property inventory

The supplied evaluator relations offer closed single-headed dispatch opportunities, but their structural constraints still need general CHR or a separately justified solver. No whole-language restriction follows from this inventory. This is source analysis of the executable programs, not a runtime benefit measurement or a finished E02 experiment.

## Arithmetic and explicit case declarations

`add/3` and `sub/3` each have one linear catchall head. They can use occurrence-local expansion without choosing among CHR rules. `add` retains explicit OR in its body; its unknown-input and repeated-variable queries in E00 exercise the modes a constructor-head-only rewrite could lose. A closed relational case declaration can elaborate to the existing body without a semantic change. A global requirement that each call be constructor-instantiated would exclude those queries unless explicit case generation is retained.

Compiler comparison: direct catchall OR, inferred case specialization, and an explicit declared-case source form with the same elaboration. Count dispatch/equality work and compilation separately. Source-order arbitration between ordinary rules is not needed for this closed relation, but that fact does not establish source-order semantics globally.

## SK evaluation and typing

`eval/2`, `fold/2` and `infer/2` each have one linear catchall head. Closed local expansion is possible even with unknown program and output arguments. `eval` calls `fold`, and both return through shared logical variables; local expansion does not certify independent AND components. `no_c/1` reads the same program handles and must wake after bindings.

S repeats the argument handle in its evaluation body. Affine use of writable logical-variable cells is therefore not an automatic source property. Sharing immutable constructors is different from giving two occurrences unrelated writable copies. A proposed ownership discipline needs a concrete representation of that repeated handle and tests preserving correlation.

`no_c` has four constructor-headed rules. In the declared SK signature their root cases discriminate, but an unknown argument suspends. Other constructors remain residual under the actual rules. A closed-domain logical solver must declare its signature; a compiler must not infer rejection of every unmentioned constructor merely from these rules.

## Literal lambda relation and structural theory

`step/2` and `lamEq/2` have one linear catchall rule each and retain explicit alternatives. The rules call `neq` and `norm`; a relation region may export those obligations through a common equality/effect interface. It is not an independent factor merely because dispatch is local.

`norm` uses two-headed joins with `var`. A global single-head restriction would exclude this source theory unless it is reformulated or moved behind a specified solver interface. The registration includes `norm(x)` with and without a companion `var(x)` to expose that semantic difference. Posting an inferred var occurrence from a logical consequence would change which CHR rules can fire.

Inspection of the pinned notebook confirms that both two-headed norm rules consume their var partner and post a new var occurrence. Replacing those rules with simpagation is not generally occurrence-preserving: the `norm_reintroduction_gives_propagation_a_new_occurrence` test prepends an observer propagation rule and derives two `seen(x)` occurrences. This is a concrete boundary example for region closure, propagation histories and source reformulation. The literal translation keeps the original consumption/reintroduction behavior.

The `neq` rejection rules can overlap, for example when equal arguments also have an app root. That overlap differs from the unique-dispatch relation region. All those applications fail on that particular input, but a general compiler cannot infer that every overlap in the whole program is harmless.

## Further experiments and remaining decisions

E04/E06 must compare global restrictions against local inferred/declared regions on these same programs, including an added external consumer that invalidates closure. E05/E06 must measure any uniqueness or disjointness certificate instead of assuming S is affine. E10 must distinguish retaining exact residual occurrences from adopting formula observations. E08 must account for future head interactions and shared aliases before certifying independent factors.

Still open: an executable eligibility analysis and its precision/linking cost; equivalent restricted reformulations where they exist; source and annotation burden; actual eliminated service costs; and owner adoption of any beneficial restriction. This inventory enables those comparisons and does not close them.
