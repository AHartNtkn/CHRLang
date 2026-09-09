# CHR rules can own a forest, but this is not yet integrated CHR execution

CHR rules compute the tested equality partitions and perform actual path compression over ground node identifiers. This establishes a distinct operator implementation from the existing Rust-owned union-find. It does not establish constructor consistency, a complete consuming executor, or a performance advantage.

## What ran

The [source rules](../../../research/chr-relational/tests/support/chr_forest.rs) represent nodes with `root(node)` or `edge(node, parent)` constraints. `union` requests generate finds and a link; source rules follow edges, merge roots and redirect stale representatives. An optional source rule compresses a traversed path. The sole native equality operation binds a temporary find result to `found(ground-node-id)`; it does not unify represented classes. Distinct ground node names remain distinct host terms.

The implementation uses the existing compiled CHR executor to run these rules. This isolates the question of expressing the operator through CHR. It does not demonstrate that a complete integrated runtime eliminates the compiled executor's discovery machinery. No second production baseline is introduced.

## Independent checks and findings

The [gate](../../../research/chr-relational/tests/chr_forest.rs) enumerates every ordered pair of directed union requests on four nodes: 256 request sequences, two input orders and two compression modes, giving 1,024 source configurations. Self unions, duplicate requests, opposite directions and disconnected components are included.

Each configuration runs with compiled Scan and Indexed access and agrees with an independent scalar source evaluator. A separate boolean transitive-closure oracle computes undirected connectivity without using union-find. Both the residual forest and subsequent finds must reproduce that partition. The two compiled phases give 4,096 compiled executions. The scalar evaluator supplies 3,072 corresponding runs; these counts describe validation, not timing samples.

The residual check independently requires exactly one root or outgoing edge for every input node, in-domain ground parents, no self edges, no directed cycles and no unfinished requests. Checking the residual matters: equal find outputs alone would not establish that the represented forest is valid.

| Witness | Observed result | Consequence |
|---|---|---|
| Chains of 2, 7 and 23 edges | The first lookup traverses every edge in both modes. Compression fires once per traversed edge. A repeated lookup traverses one edge with compression and the original path without it. | The compression mechanism actually executes and changes subsequent source work. No elapsed-time conclusion follows. |
| Stale left or right link representative, both compression modes | The corresponding source redirect fires and produces one connected forest. Both compiled access modes agree with the scalar evaluator. | Root identifiers returned earlier cannot be assumed to stay roots after another merge. |
| Same stale-link witness with its necessary redirect rule omitted | The scalar result retains an unfinished link request. | The adverse mutation is detected; merely retaining ground find results is insufficient. |

Find results name the root at lookup time. They are not immutable canonical class names across subsequent unions. Clients must either follow the forest again or participate in a repair protocol. The redirect rules implement that obligation for linking; constructor descriptors and suspended source applications will need their own treatment.

## Validation scope

The original unimplemented rule provider failed its connectivity test; the implemented and strengthened gate passes all three tests. All 30 relational-package tests pass, and scoped strict Clippy passes. The runs are recorded in [the receipts](s02-chr-forest-gate/). The package includes independent source and equality checks beyond this forest gate. No reference-interpreter source was changed.

Commands: `cargo test -p chr-relational` and `cargo clippy -p chr-relational --test chr_forest --no-deps -- -D warnings`. An initial unscoped lint invocation also checked an existing dependency build script and reported unused code-generator helpers; the scoped invocation checks the task-owned target without promoting dependency warnings to errors. The final target has no lint suppression.

This is a correctness and source-operation experiment. There is no comparative timing, allocation or lifecycle result, and no claim of asymptotically optimal union-find. Root linking is unranked; a credible cost comparison must establish an appropriate merge strategy rather than attribute avoidable long paths to CHR itself.

## Next decision and strongest alternative

Continue T072 with constructor descriptors and equality-enabled consuming applications, alongside the strategic local rewrite organization specified in the [sequence](../remaining-investigations.md#next-discriminating-comparisons). The forest gate makes the CHR operator concrete; it leaves the architecture's most consequential responsibility untested: propagating a merge into constructor consistency, matching and resource effects without repeating whole-store work.

The next gate needs decomposition, clashes, finite-tree cycles, late information, fresh values, propagation and contested occurrence consumption. It must expose a chain where equality enables a consuming application whose body contributes further equality. Independent complete-source answers are required; partition correctness cannot stand in for them. Descriptor repair and stale application references are explicit candidate costs to compare against direct incidence/port rewrites.

Call-level reuse remains the strongest ready alternative. This small operator gate supports proceeding to the integrated source comparison because it resolves a representation feasibility question but gives no evidence about the combined responsibilities. Reconsider selection after that source gate, before a broad timing matrix. Count this as one correctness package toward the four-package breadth review. T072 and the overall research goal remain active.
