# Reusing a computation is different from instantiating another rule application

**Equal inputs do not justify sharing a CHR application's concrete result.** Even a deterministic rule can introduce fresh unknowns that must remain independent between applications. The experiment separates sharing within one application from reusing a derivation across applications; both remain candidates, with different ownership costs.

This is a semantic result, not a performance result. It does not reject memoized pull-tabbing or derivation reuse. It determines what their next source implementation must represent.

## What was tested

The [prospective registration](../registrations/S03-derivation-identity-gate.md) fixes six inputs across three small source programs. Each has hand-derived complete answers, checked against the independent scalar source oracle, compiled global scanned and indexed execution, Conditional execution, and the direct named-choice graph. All six agree across all five paths, with default features and with this package's default metrics disabled. No counters or timings are used as comparative evidence.

| Source distinction | Complete result | What an incorrect reuse rule would change |
|---|---|---|
| One deterministic `make(k,R)` used twice, returning `box(Z)` with fresh Z | Both uses refer to the same unknown | Instantiating on every demand would break the alias |
| Two independent deterministic calls on k | Two distinct unknowns inside the two boxes | Returning one cached result node would introduce an alias |
| One nondeterministic call used twice | Two raw answers: `(a,a)` and `(b,b)` | Fresh choice on each demand would admit mixed pairs |
| Two independent nondeterministic calls on k | Four raw answers, including mixed pairs | Reusing one choice birth would correlate independent applications |
| Two equal-valued asks, one token | One output; both asks remain | A second body replay without a resource claim would invent an output |
| Two equal-valued asks, two tokens | Two outputs; both asks remain | Collapsing equal-valued token occurrences would lose an output |

The tests check joint aliases and raw multiplicity, not just displayed ground values. The incorrect predictions are explicit countermodels compared with expected answers; no cache implementation was injected or benchmarked. This distinction limits the claim to the proposed reuse rules.

Raw receipts: [default](s03-derivation-identity/default.log), [metrics disabled](s03-derivation-identity/metrics-off.log). Executable witnesses are the three `derivation_identity` tests in [the source gate](../../../research/chr-direct-conditional/tests/direct_graph_entry.rs). All processes completed inside the registered 60-second bound. The reference implementation was unchanged.

## Why this changes the implementation choice

**The unit of reuse must be stated before choosing a cache key.** A concrete application result can preserve its own unknowns and choices when the same application is demanded again. A reusable template for another application must instantiate its fresh unknowns and choice births. Determinism alone does not make that instantiation unnecessary: the first two cases contain no disjunction.

**A cached value is not authority to apply an effect.** A consuming application's justified transition includes the exact occurrence claims in its applicable context. Another demand on the same committed event must not consume again. A distinct application must acquire its own valid claims even if its value computation was reused. Equal printed arguments cannot identify these resources.

These are necessary conditions, not a sufficient cache-validity proof. Changes to bindings, guards, rule competition, propagation history and context support can invalidate a candidate application without changing its source text or occurrence tuple. A complete implementation must represent those dependencies or establish a fragment in which they cannot change.

## What pull-tabbing adds beyond the existing graph

Memoized pull-tabbing moves a demanded choice through a function application and retains task-specific results of shared calls. Choice identifiers preserve correlated selection; task ancestry can permit reuse of ancestor results. These are operations on suspended computations, not merely shared constructor storage. [Hanus and Teegen, sections 3–4](https://www.michaelhanus.de/papers/WFLP20.pdf).

The CHR comparison requires a separate source argument. In the current syntax, constructor terms do not themselves execute function bodies. The direct graph already passes unknown arguments opaquely, exposes constructor alternatives during matching, and creates named values for pure assignment choices. Its `Pending` queue executes source goals; its `applications` method rediscovers enabled head tuples. It has no persistent suspended-call result map. Thus adding pull-tab transformations to constructor terms alone would not establish the missing source-computation mechanism.

A concrete trace is `make(k,R), pair(R,R)`. The direct graph fires `make`, creates its result identities once, and leaves two references in `pair`. A demand-driven application node must likewise have one birth and one committed effect, even if both argument positions demand it. For `make(k,R), make(k,S), pair(R,S)`, there are two births; a template may reuse analysis or derivation structure, but cannot simply return the first result graph. The experiment verifies precisely this difference.

## Next implementation and selection decision

**Continue T071 with suspended source applications, distinguishing application-result reuse from template instantiation.** The candidate must make matching demand an explicit operation, preserve each application's context and effects, and resume unresolved demands after relevant binding or occurrence changes. It must also service active source obligations outside the output graph before claiming a complete answer. These are candidate responsibilities to implement and measure, not new language requirements or a selected production interface.

Use the existing opaque-work source family to demonstrate avoided computation, then repeated demands on one application and distinct equal-input applications to separate the two kinds of reuse. Preserve the existing adverse immediate-discrimination, disconnected-failure and finite-sibling tests. Compare retained application results with reevaluation under the same semantics, as well as the corrected direct graph and explicit controls. Charge instantiation and validity costs instead of counting a reused template as a free application.

This remains more decision-relevant than another context-allocation refinement: it tests computation retention that the current graph does not implement. Contextual equality/local consuming rewrites remain the strongest following alternative. Reconsider that ordering when the source mechanism works or exposes a consequential obstruction; do not add another isolated identity gate without a newly discovered dependency.

Full source implementation, benefit/adverse measurements, sustained lifetime and complete architecture comparisons remain open. The six countermodels supply no runtime ranking and do not complete T071 or S03.
