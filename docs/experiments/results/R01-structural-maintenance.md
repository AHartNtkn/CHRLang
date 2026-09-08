# Indexed structural maintenance: immutable suffixes are revisited

The [registered causal gate](../registrations/R01-indexed-maintenance-gate.md) confirms quadratic dependency/key traversal with linear source applications in both ordinary and inferred-specialized Indexed execution. All complete residual answers and exact source rule traces agree with independent expectations. [Counter output](r01-structural-maintenance/baseline-gate.log), [metrics-off semantic gate](r01-structural-maintenance/baseline-off.log), [Clippy](r01-structural-maintenance/baseline-clippy.log).

| Unary depth | Applications | Dependency visits | Key visits/normalization requests | New normalization nodes |
|---:|---:|---:|---:|---:|
| 8 | 8 | 54 | 54 | 0 |
| 16 | 16 | 170 | 170 | 0 |
| 32 | 32 | 594 | 594 | 0 |

Each indexed occurrence visits the full remaining suffix twice, for dependencies and canonical keys. Counts follow `(n+1)(n+4)/2` for each traversal. Existing canonical nodes are found again, so zero new normalization nodes does not imply zero temporary allocation or traversal. Global/Scan performs neither kind of maintenance; it is an attribution control, not a proposed replacement for indexed access. This establishes avoidable repeated discovery of immutable structure, not how much of R08 elapsed time it explains.

The first run corrected two entry assumptions: Global/Scan intentionally omits dependency tracking, and sealed specialization rejects Active policy. [The original outcome](r01-structural-maintenance/first-gate.log) remains recorded. Ordinary Active and Global, and specialized Global, pass the pending constructor/alias/late-binding witness with complete nonground residual aliases and rule order `alias, bind, match`. Unsupported specialized Active configurations are checked for rejection.

## Selected correction boundary

An interned node can carry the immutable property “contains no syntactic variables.” It is true exactly when every child is a node with that property. A structurally closed subtree has no binding dependencies, and its existing arena-local ID is already its canonical ground key. Computing this property once at node construction allows dependency and key traversal to skip that subtree.

This property must be maintained at the actual mutation boundary: node contents and their table cannot remain externally mutable while cached metadata assumes immutability. Read consumers must use an immutable view. Forks must preserve metadata with their node tables. A variable root still registers a dependency before following its binding, even if currently bound to a closed value. A syntactically open node does not become structurally closed when one branch grounds it.

T044 continues with this bounded correction and contrary tests for nested alias binding, open parents beside closed subtrees, branch-local binding differences, canonical identity, and failed speculative equality. Independent review supports this boundary. No elapsed improvement is claimed before a separately registered lifecycle comparison; metadata preparation and retention costs must be charged there. Static-lowering eligibility remains the stronger broader alternative if the correction requires substantial redesign.
