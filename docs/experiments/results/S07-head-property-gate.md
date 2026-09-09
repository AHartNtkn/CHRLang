# Single heads remove a join boundary, not every execution obligation

Single-head rules can still compete, wait for bindings, preserve multiplicity and require propagation history. Bundling resources into one state term reproduces a small closed program with unary rules, but changes the contract for inputs and linked updates. These witnesses define the properties the next compiler comparison must test; they do not justify a mandatory restriction.

This is the source/correctness stage selected by the [breadth review](S10-join-breadth-review.md). No property checker, new restricted execution engine or comparative cost matrix is claimed here.

## Properties that must remain separate

| Property or proposed simplification | Executable finding | Architectural consequence |
|---|---|---|
| Every rule has one head | `p(X)` and `p(a)` both consume the same `p(a)` occurrence. Swapping source order changes the winner. | Head count does not establish freedom from resource competition or remove source priority. |
| Unary constructor cases are disjoint | Two `p(a)` occurrences both complete; a separate unknown `p(X)` completes after a later binding to `b`. | Disjoint cases still need occurrence multiplicity and late activation. |
| There is only one rule | A two-head rule over three `p` occurrences has six ordered matches with intersecting consumed IDs. | Rule count cannot certify non-overlap between applications of the same rule. |
| One unary propagation rule | Two equal-valued occurrences each produce exactly one mark and remain residual. | Once-per-occurrence propagation history is independent of head count and cross-rule ambiguity. |
| One unary constructor head | A partially decomposed `f(X)=f(Y)` gives two matching environments on one occurrence before the children merge. | Head count does not bound intermediate matching environments to one. This is not evidence of two completed answers. |
| Reserve the first resource, then finish a join | With both resources, the tested variant agrees. With the second absent, reservation blocks the rival and leaves a waiting resource. | Sequential acquisition is not automatically an equivalent implementation of atomic multi-head consumption. The tested reservation variant still has a two-head finishing rule. |

The partial-equality probe checks the two stored environments and their common consumed occurrence directly. It also verifies that the children are distinct before the next equality step and become aliases afterward. Any future overlap analysis must state whether it reasons about intermediate matching, settled states or completed observations.

## A unary reformulation works within an explicit boundary

The original program atomically consumes `p` and `q` to produce `join_won`; otherwise a competing rule can consume `p` to produce `rival_won`. A hand-written unary encoding represents the three tested input states as `state(both)`, `state(only_p)` and `state(only_q)`. It produces the same complete observations for those inputs. Original and bundled source alternatives also preserve both expected outcomes.

This is a real change of runtime responsibility: the encoded rules match one occurrence instead of acquiring separate `p` and `q` occurrences. The caller has already combined the resources into a state description. It is a bounded source reformulation, not a general compiler for arbitrary multisets, duplicate resources or concurrent updates.

Linking exposes the transferred responsibility. In the original program, a higher-priority producer of `q` enables the atomic join before the rival consumes `p`. Adding that same producer to the bundled program leaves `q` separate from `state(only_p)`; the rival wins and `q` remains residual. The initial-state encoding alone therefore does not preserve composition.

A coherent unary producer instead owns a combined state-and-command occurrence and updates it to `state(both)`. That tested source restores `join_won`. This demonstrates that a broader unary representation can express the operation, while making coordinated ownership of state and commands part of its interface. The experiments reject the unchanged linked translation, not unary languages in general.

## Evidence and limits

Seven tests pass with diagnostics enabled and disabled, with scoped strict Clippy and formatting. There are 20 finite source configurations per build, each checked against independently constructed complete answers, the scalar evaluator, conventional Scan, contextual eager, conditional execution and contextual resumable execution. Two direct store probes additionally establish conflicting application instances and intermediate environment multiplicity. [Logs and source hashes](s07-head-property-gate/) record the checks.

The source checks preserve resource winners, residual occurrences, multiplicity and source alternatives. They establish concrete semantic distinctions. They do not measure checking or preparation costs, prove a general encoding theorem, or establish that a required property saves more than the same inferred property.

## Next comparison

Build the executable property/beneficiary comparison before drawing a language-policy conclusion. Keep these three propositions distinct: one head per rule; no competing applications for a consumed occurrence; and at most one matching environment. A checker must conservatively account for instances of the same rule, repeated variables, guards, partial information and linked extensions rather than treating a rule-name list as the property.

Compare inference, optional checked declarations and required admission against the same immutable prepared source and actual removed execution responsibilities. Show which join, dispatch, history or dependency machinery can disappear under each accepted property—and which must remain. Include the competing unary, late-binding, propagation, atomic-resource and linked-update witnesses above. Charge any caller-side encoding or coordinated-update machinery when measuring reformulations.

The strongest ready alternative remains broader equality relevance, but this bounded source gate exposes prerequisites for a fair head/overlap comparison. Continue to the first real compiler beneficiary, then reconsider priority before extending the restriction family. T079 and the architecture goal remain active; no language restriction is adopted.
