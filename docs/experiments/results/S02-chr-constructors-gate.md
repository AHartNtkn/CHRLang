# CHR-expressed constructors enable consuming rules and expose a scheduling boundary

CHR rules now repair constructor information after merges, reject finite-tree inconsistency, and execute a consuming application whose equality enables another application. The independently checked source examples pass. Adding a kept descriptor changes which competing consumer wins under the existing match-tuple policy, so this encoding is not a transparent replacement for that fixed policy.

## The actual mechanism

The [kernel rules](../../../research/chr-relational/tests/support/chr_constructors.rs) extend the [ground-identifier forest](S02-chr-forest-gate.md). Constructors are `d_a(node)`, `d_b(node)` and `d_f(node, child)` constraints. A parent edge causes source rules to repair every exposed node column in descriptors, ancestry facts and consuming requests. Two same-class `f` descriptors generate a child union; incompatible constructors fail.

Constructor edges generate `below(parent, child)` facts. CHR propagation computes transitive ancestry, duplicate facts are absorbed, and `below(x, x)` fails. The closure is explicit and eager in this experiment. Its work and retention must be compared with a competent local cycle check before any cost conclusion about CHR integration.

The consuming rule keeps `d_f(x, child)`, consumes `take(x, result)` and one `token()`, and posts `union(result, child)`. The next match can therefore depend on a merge caused by the previous consuming application. No host equality service merges represented class identifiers; the host only transports the forest's temporary find results. Host CHR execution still owns discovery, source occurrences, propagation history and branch copying.

This is an executable bounded source encoding, not a general compiler. The signature is two constants and one unary constructor. Node identifiers are preallocated; dynamic class creation, arbitrary source rules, higher arity, richer guards and a general observation protocol remain required. Decoding is a checked test observation outside any measured interval; no timings have run.

## What was tested

The [tests](../../../research/chr-relational/tests/chr_constructors.rs) use the independent scalar evaluator's recursive substitution and finite-tree checks on ordinary equations and consuming source rules. The expected evaluator does not use the forest or descriptor implementation. Complete decoded outputs and residual multisets are compared jointly, preserving aliases and raw alternative multiplicity.

| Experiment | Configurations and evidence | Result |
|---|---|---|
| Constructor consistency | Three nodes, each unknown, `a`, `b`, or `f` of any of the three nodes: 216 descriptions. Each has no union or one of nine directed unions, giving 2,160 equation configurations. Two fact orders and Scan/Indexed access give 8,640 compiled runs. | Complete terms, aliases and failure agree with independent substitution. Includes direct and indirect cycles, constructor clashes, decomposition and disconnected unknowns. |
| Equality/consumption chain | `take(X,Y), take(Y,Z)` with `X=f(f(a))`; zero through three tokens; both access modes. | Zero tokens leave both requests; one permits the first; two permit both; three leave one token. Complete outputs and residuals agree. Scalar source traces of the encoded rules require a root merge and repair of the next request between the two consuming firings. |
| Branch-local consumption | A choice between `f(a)` and `b`, or between `f(a)` and the inconsistent equation `X=f(X)`; zero through two tokens; both fact orders and access modes. | Unmatched requests, remaining resources, successful multiplicity and failed alternatives agree independently. |
| Hidden cycle | A consistent branch and a branch with an unobserved self-cycle, with a retained token; both access modes. | Exactly the consistent sibling publishes. An off-output contradiction cannot escape through decoding or affect its sibling. |
| Competing consumers | Two ready requests for `f(a)` and `f(b)`, one token; both request orders and access modes. | Ordinary request order and encoded descriptor order can choose different consumers. The exact discrepancy is asserted, not treated as equivalent work. |

The decoder additionally requires one root or outgoing edge per represented node, finite parent walks, at most one reconciled descriptor per root, and no stale references in exposed descriptor, ancestry or request columns. Constructor cycles and unfinished internal operations cannot pass as ordinary answers.

## The resource-order counterexample

Start with `take(f(a), A)`, `take(f(b), B)` and one token. Ordinary source matching gives the token to the first request: reversing requests changes the bound output from `A=a` to `B=b`, and changes the remaining request.

The encoded rule has a kept constructor descriptor before its removed request and token. Under the current match-tuple ordering, descriptor order selects the winner. With the `a` descriptor first, the encoding produces `A=a` in both request orders, for both Scan and Indexed access. The [initial fixed-policy comparison](s02-chr-constructors-gate/fixed-policy-counterexample.log) records the failure. The final test independently asserts both complete ordinary outcomes and the exact encoded outcome, including residual consumption.

The [S00 contract](S00-contracts-and-candidates.md) permits independently validated committed source executions in an architectural comparison. Both observed winners have such a derivation. This does not prove arbitrary encodings correct, and it does not justify a speed comparison between these different answers. If preserving ordinary request priority is part of a candidate's contract, that candidate needs explicit source-application selection rather than inheriting descriptor tuple order. No language scheduling policy is adopted here.

## Validation and interpretation

All 35 relational-package tests pass, including five new tests. Scoped strict Clippy passes for the new target. The [receipts](s02-chr-constructors-gate/) contain the failing preimplementation gate, resource-order counterexample, final package tests and lint result. Commands are `cargo test -p chr-relational` and `cargo clippy -p chr-relational --test chr_constructors --no-deps -- -D warnings`. Dependency build-script warnings remain outside the scoped lint target. Reference-interpreter code is unchanged.

The experiment establishes that the represented equality, constructor and consuming transitions can execute as CHR rules. It also establishes actual alternation of source consumption and equality repair. It does not establish useful-work savings: the host executor still services the rules, and ancestry closure, descriptor repair and repeated discovery have not been costed. This is correctness and operational evidence, not lifecycle or architectural performance evidence.

## What changes next

T072 remains active. Compare this concrete encoding with strategic local incidence/port rewriting on the same equality-enabled consuming source. The competing representation must state where descriptors live, which incident applications a merge revisits, how it commits consumption and which source scheduling contract it implements. A common ability to represent equality is not operational equivalence.

Before comparative costs, establish a competent merge strategy, broader source correspondence and independent complete observation. Include fresh identities, higher-arity structure, suspension/progress and contested effects. Charge eager ancestry and repair only as costs of this implementation until an attribution or stronger alternative determines which obligations are necessary. If a fixed source policy is selected for an ablation, match that policy explicitly.

Call-level reuse remains the strongest ready alternative. This gate is the second package after the lifetime breadth review, following the forest gate. The distinct local rewrite comparison is valuable next because it can show whether the repair/discovery responsibilities exposed here disappear or merely move. Review priority after that gate and at the four-package checkpoint; further refinement of the CHR encoding is not automatically preferred. The goal remains active.
