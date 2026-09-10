# Names entry: literal rejection and declared atomic membership

T076 package one after the conditional overlap breadth review. The recorded proposal in T015-solver-certificates and T016-exact-solver-projection concerns represented names. Current authoritative source is `crates/chr-programs/src/lib.rs::lambda`, with existing literal-substitution tests. It supplies no nominal binder quotient, automatic freshness or capture-avoiding substitution. Preserve those distinctions.

## Question and candidate

Can a compact conjunction of name restrictions preserve the intended ground solutions and caller-variable dependencies, and what differs from literal CHR residual behavior? Implement two explicit experimental contracts: literal rejection of app/2 and lam/2 roots, and membership in atomic names (all nullary constructors). Under the latter a structured other/1 term fails; under the former it remains permitted. Neither contract classifies a currently unknown logical variable as a known name.

The candidate checks known roots and retains a set of restricted logical variables. This is a logical formula: repeated obligations are idempotent there. The source control retains all var occurrences and identities. Do not substitute the formula for raw source answers or claim fewer CHR derivations. A refinement rechecks a variable after a supplied binding; transport maps logical variables into a fresh caller and requires an injective mapping. Fixed constructor names are never renamed.

Independent oracle: enumerate finite ground assignments and evaluate membership directly on substituted trees. Compare accepted complete assignments, not candidate summary shape. Actual source control: run the unchanged reference with the full literal lambda program and compare independently expected failed/successful residual answers. Include a host observer to expose the consequence of discharging a ground var occurrence.

## Frozen finite matrix and bounds

Use 13 templates: variables 0 and 1; nullary x, y, app, lam and other; and app/1, app/2, lam/1, lam/2, other/1 and other/2 with child variable 0 and, for binary forms, variable 1. Obligation lists have lengths 0, 1 and 2 with ordered repetitions: 183 lists. Assign both variables from seven ground terms: x, y, app/0, lam/0, other(x), app(x,y), lam(x,y). This gives 8,967 assignment/list cases per contract and 17,934 across both contracts. Each compares direct denotation with the compiled summary followed by refinement. Run the actual reference on all 8,967 ground literal cases, validating exact residual multiplicity or failure. Also run all 183 partial lists against independently expected residuals/failure, retaining unknown aliases.

Additional named tests: late binding changes validity; two unknowns may later alias; fresh-caller transport retains restrictions and rejects noninjective transport; fixed represented names remain observable; a host var observer sees a residual ground occurrence; finite logical projection with alphabet sizes 1,2,3 and 1..6 restricted unknowns has one compact satisfiable formula but respectively n^k complete assignments. A zero-name domain has no assignment for a nonempty restriction. This is an explicit logical enumeration endpoint, not implicit source search.

Write tests before implementation and preserve the expected missing-implementation or semantic failing build. Then implement and run the names test executable twice in default and metrics-off builds. Each executable has 60-second wall/CPU and 1 GiB address-space limits; reference calls have a 1,000-step bound and must exhaust. No timing or memory ranking. Freeze source/test/registration hashes and binary identities before these confirming runs. Existing lambda and structural regression tests remain required.

## Interpretation and next boundary

A mismatch must distinguish a candidate defect from the different domain or residual contract. Correct consequential candidate defects. Agreement establishes only the finite entry plus the stated algebraic reasoning; it does not implement disequality, nominal abstraction, complete projection or a general solver. Source-domain certification, retained owners, output enumeration and full costs remain required before an optimization claim.

At this gate compare further name representation/cost work with delayed disequality (the next scheduled theory), integrated execution and conditional ownership. Plain name membership has no substantive recursive work to remove in its known-root case; do not manufacture a runtime win by comparing one formula with all raw answers. Disequality can make finite versus unbounded name domains and hidden-variable projection consequential and is the intended next contrast if the entry qualifies.
