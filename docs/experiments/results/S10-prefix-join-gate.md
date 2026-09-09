# Partial joins avoid suffix work, with a cost on compatible products

Retaining compatible prefixes reduces service work when an incompatible prefix can exclude several suffix combinations. It also saves repeated conjunctions on independent choices. Fully compatible products and small inputs expose traversal overhead; lifecycle efficiency remains unmeasured.

This is the distinct prefix control required by the [direct support-join gate](S10-support-join-gate.md), not a replacement of its evidence. The experimental `prefix-join` feature selects the traversal while retaining existing per-head filtering. No default architecture changes.

## The responsibility changes

Direct tuple checking constructs a complete combination before checking its conditions. The new traversal builds a prefix in head order, retaining selected occurrence IDs, the conjunction at each depth and the next bucket position. An empty conjunction rejects that prefix before visiting its suffix. Repeated occurrence IDs are rejected as soon as encountered.

The cursor uses the existing resumable Boolean jobs. Each service call extends, backtracks, emits a tuple or advances one Boolean operation step. Completed tuples retain the original anchor activation scope and enter the existing candidate/dependency pipeline. Execution-time checks remain required because resource conditions can shrink after discovery.

The safety argument is unchanged: occurrence lives only shrink, identities are never reused, and discovery drains before execution resumes. Bindings, guards and temporary scheduling eligibility do not participate in prefix rejection. Surviving tuples keep their lexicographic bucket order. The direct tuple-check state is compiled out of prefix builds; retaining both state machines is not charged as a necessary prefix cost.

## Sources distinguish pruning from overhead

New three-head rules join `p(X), q(Y), r(Z)`. The suffix has 0, 1 or 8 `r` occurrences. Sources make `p` and `q` mutually exclusive, correlated, independently chosen or unconditionally compatible. Independent expected answers preserve every residual occurrence and choice combination. Scalar, conventional Scan and conditional execution agree in all 12 configurations.

At suffix size eight:

| Source | Registered candidates, both controls | Direct tuple steps | Prefix steps |
|---|---:|---:|---:|
| Mutually exclusive prefix | 1 | 387 | 339 |
| Correlated pairs | 17 | 3,440 | 3,361 |
| Independent choices | 33 | 9,150 | 8,850 |
| Fully compatible dense product | 9 | 1,623 | 1,638 |

**The savings precede candidate registration.** Both controls register the same candidates; prefix traversal avoids repeatedly constructing/checking combinations that direct checking later rejects. Independent choices also benefit from reusing the prefix conjunction across suffixes. Predicate-distinct heads in these witnesses exclude duplicate-occurrence pruning as the explanation.

**Small inputs do not repay traversal.** At suffix size one, prefix steps are 171 versus 170 for mutually exclusive heads, 505 versus 493 for correlated pairs, 1,059 versus 1,051 for independent choices, and 259 versus 251 for the dense product. Empty-suffix cases add two steps in each family.

Existing mixed sources also retain contrary evidence. At choices=3/depth=16, prefix traversal reduces early-failure steps from 5,125 to 4,867, but increases independent-source steps from 35,295 to 37,049 and common-source steps from 4,301 to 4,523. Six-occurrence dense propagation rises from 8,556 to 8,712. A favorable suffix product does not establish a general join strategy.

## Validation and limits

An independent cursor test enumerates all three-position selections from four occurrences with conditions `a`, `not a`, `b` and `true`, under three anchor conditions. Ordinary Boolean truth assignments determine expected tuples without invoking the production conjunction or support evaluator. The cursor must produce exactly the ordered sequence of distinct compatible IDs, and terminate within 10,000 steps.

All 125 crate tests pass with prefix traversal. A selected counter-free suite passes 37 tests, including resources, late dependencies, broad sources and finite publication beside ongoing work. The strengthened independent truth test passes separately; strict Clippy passes for prefix and direct controls. [Evidence logs and source hashes](s10-prefix-join-gate/) include the paired work measurements.

The reported steps are service calls, not instructions or elapsed time. No allocation or timing matrix has run for these join controls. The cursor retains vectors and intermediate Boolean supports; the support arena can retain constructed nodes after a traversal ends. Cancellation and sustained ownership costs must be measured, not inferred from Rust ownership alone.

## Next decision

Register a bounded lifecycle comparison of existing filtered discovery, direct tuple checking and prefix traversal, with conventional Scan and the current contextual control. Reuse prepared rules across changing queries; include incompatible and compatible suffix products, mixed sources, small inputs, retained answers and cancellation during discovery. Charge pool creation, cursor state, Boolean jobs/arena retention, preparation, delivery and every owner’s disposal. Keep ordinary timing and allocation diagnostics separate.

This comparison has priority over another implementation refinement: work counts now identify both a favorable regime and adverse controls for two credible competing mechanisms. Broader equality relevance, language properties, sustained lifetime and whole-architecture/held-out obligations remain required. Neither T078 nor the research goal is complete.
