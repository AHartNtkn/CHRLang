# Indexed structural maintenance: immutable suffixes are revisited

At baseline commit `a852a82`, the [registered causal gate](../registrations/R01-indexed-maintenance-gate.md) confirms quadratic dependency/key traversal with linear source applications in both ordinary and inferred-specialized Indexed execution. All complete residual answers and exact source rule traces agree with independent expectations. [Counter output](r01-structural-maintenance/baseline-gate.log), [metrics-off semantic gate](r01-structural-maintenance/baseline-off.log), [Clippy](r01-structural-maintenance/baseline-clippy.log).

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


## Validated correction

The immutable-node property is now enforced by private node storage and read-only accessors. Constructor insertion computes closedness from immediate child metadata only for a new interned node; arena cloning preserves it. Indexed ground-key normalization returns a closed node's existing ID, and dependency discovery skips its descendants while still registering variable roots and following open bindings.

The same source gate now records 18, 34 and 66 dependency/key visits at depths 8, 16 and 32, exactly two argument roots per occurrence. Normalization requests are zero, with unchanged complete residuals, rule traces, index insertions and removals. [Failing bound](r01-structural-maintenance/bound-red.log), [corrected counts](r01-structural-maintenance/bound-green.log). Open-parent, nested alias, divergent branch binding, unknown-alias and failed speculative-equation witnesses pass; structurally open nodes retain that property even when bindings ground their observations.

All 298 workspace all-target tests, workspace strict Clippy and formatting pass. Compiled and persistent suites pass in both default and metrics-off builds, as does affected metrics-off Clippy. The persistent suite retains its existing isolated registry case outside the ordinary test invocation. [Workspace evidence](r01-structural-maintenance/workspace-final.log) and adjacent feature logs preserve validation. Independent review checked the immutable-property boundary and contrary witnesses.

The correction adds a closedness vector to each arena, computed at insertion and copied with the arena. Its preparation and retained-memory cost must be counted alongside avoided traversal. [The prospective 288-process paired lifecycle comparison](../registrations/R01-closed-subtree-lifecycle.md) includes baseline/corrected Inline and Specialized for all eight finite R08 families, plus the balanced worker controls. [The completed paired evidence](R01-closed-subtree-lifecycle.md) validates all 288 processes and resolves favorable Specialized timing ranges in four carry-heavy families. Inline timing and current worker ordering remain unresolved. T044 is complete; T045 selects finite recursive-lowering eligibility and correspondence.
