# R04 source correspondence gate

Before implementing a direct solver, test whether the finite relation in [the entry analysis](../results/R04-finite-consistency-entry.md) agrees with actual source execution. A discrepancy invalidates the proposed lowering or exposes an interpreter defect; it must be explained before solver comparisons. Agreement establishes eligibility on these instances, not a performance ranking or general CHR compilation theorem.

## Prospective protocol

Use the independent Cartesian assignment oracle in `research/chr-direct-solver/oracle.py` and the existing copying reference interpreter through a syntax-only Rust bridge in that directory. The oracle does not call the reference. The bridge constructs exactly the three analyzed rules and decodes complete ground answers; it contains no constraint solver. Source rule order is varied across all six permutations to test the correspondence's order independence without introducing source scheduling choices.

Cases are the four four-variable networks in the entry analysis, each with explicit choosers; all 81 supplied assignments for each network without choosers; and ten semantic boundary queries: repeated chooser, repeated forbidden residual, impossible off-diagonal self-edge, output aliases, unobserved choice, conflicting givens, empty query, ground chooser, true ground forbidden tuple, false ground forbidden tuple. All graph variables are selected. Boundary queries intentionally vary observation and residual content. This is 338 queries per source rule order.

Compare exhaustion, raw successful branch count and the entire set of selected outputs plus sorted residual multisets. Never accept a count-only match. Decode source atoms independently into domain integers; reject nonground/unknown source output rather than approximating it. The oracle's variable mapping only serializes query identities and does not provide inferred bindings to the reference.

The bridge must construct supplied queries without OR-producing constraints, and the gate checks zero reference splits on them. The oracle rejects unsupported queries before bridge execution; its focused tests cover uncovered holes, constant outputs and duplicate labels. Also validate an impossible ground forbidden query and an empty query directly against analytic expectations to expose a bridge that silently ignores constraints or always returns no answers.

## Bounds, evidence and interpretation

Each reference query receives at most 1,000,000 steps. One full bridge invocation receives a 60-second process timeout. Any unexhausted result or timeout fails this gate and is recorded as incomplete, never as refutation. These bounds are generous for at most four variables and finite producers; raise them only after diagnosing unexpected growth.

Run the source gate once after building the bridge and validating its basic behavior. Record source revisions/hashes, command, case count, semantic results and any defect found. This is a correctness comparison, with no time or memory ranking; no repeated timing runs, solver speed claim or amortization estimate follows from it. Compilation and oracle work are deliberately outside any future execution timing boundary.

A complete pass permits selecting a solver/native comparison only after specifying its competing algorithms, equivalent preprocessing, full lifecycle costs and strongest alternative use of effort. R01 maturity is not a dependency of this gate. R02 integration remains independent. Preserve reference code and algorithms without candidate imports.
