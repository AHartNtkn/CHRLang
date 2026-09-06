# CHR language experiments

A research project exploring CHR with finite-tree unification, explicit disjunction and shared execution. The first executable component is a **reference interpreter**: a direct implementation intended to make the specified behavior easy to inspect and compare. It copies branch state and repeats work. It is not the chosen architecture for an optimized engine.

The next investigations are organized in the [experimental sequence](docs/experiments/sequence.md), with a [direction-by-direction coverage audit](docs/experiments/coverage.md). The plan separates semantic checks, credible controls, independent mechanism probes, and complete application comparisons.

## Run the demonstrations

Use Rust with edition 2024 support. There are no external crate dependencies.

```sh
cargo run -p chr-reference --example explore -- add
cargo run -p chr-reference --example explore -- sub
cargo run -p chr-reference --example explore -- sk-identity
cargo run -p chr-reference --example explore -- sk-duplication
cargo run -p chr-reference --example explore -- infer
cargo run -p chr-reference --example explore -- type-synthesize --answers 5 --steps 10000
```

These demonstrate unary arithmetic, SK evaluation, SK type inference and type-directed synthesis. `sk-synthesize` asks the evaluator to synthesize a duplication combinator with an unconstrained program variable. It can need substantial search; the reference is deliberately unoptimized. `--steps` bounds reference work and `--answers` limits printed answers. A spent budget reports that search remains open. It never means that no further answer exists.

Terms print as `a(s, k)` and unbound variables as `$0`. Answers include the full residual constraint multiset. The CLI constructs programs directly from language data; its demo names are not a language parser or a notebook interface.

## Organization and enforced boundaries

- **[chr-syntax](crates/chr-syntax/src/lib.rs)** owns terms, constraints, rules, queries and answer data. It has no dependencies and contains no execution or answer-normalization algorithms.
- **[chr-programs](crates/chr-programs/src/lib.rs)** constructs reusable arithmetic, SK evaluator and typing rulesets. Its only dependency is `chr-syntax`.
- **[chr-reference](crates/chr-reference/src/lib.rs)** owns the copying interpreter. Its only normal dependency is `chr-syntax`; demos/tests use `chr-programs` as a development dependency. Substitution, matching, branch state and canonicalization modules are private.

Cargo's crate boundaries prevent the syntax/program layers from importing the engine, and Rust privacy prevents consumers from borrowing its internal algorithms. Future optimized engines should depend on the language data and programs independently. They should implement their own state, unification, scheduling and answer handling. Cross-engine comparisons belong outside either implementation.

The reference's results are evidence, not correctness by definition. The [semantic specification](docs/goals/chr-sharing/notes/T008-reference-and-candidates.md), independently expected examples and tests govern its behavior. Comparisons of nonconfluent programs must account for the committed rule policy; the language does not search alternative CHR schedules.

## Construct and resume a query

```rust
use chr_programs::{arithmetic, unary};
use chr_reference::Search;
use chr_syntax::{c, v, Query, Var};

let query = Query {
    constraints: vec![c("add", [unary(2), unary(3), v(0)])],
    outputs: vec![("sum".into(), Var(0))],
};
let mut search = Search::new(arithmetic(), query).unwrap();
let batch = search.advance(100);
assert!(batch.exhausted);
assert_eq!(batch.answers[0].outputs[0].1, unary(5));
```

The same `Search` can receive further `advance` calls. Each batch contains newly returned, deduplicated answers. `exhausted` is true only when no alternative remains. `stats()` exposes operation counts; `pending_alternatives()` reports the current frontier. Returned answer variable IDs are canonical names local to that answer, not original query IDs or resumable execution handles.

## Reference execution policy

Each queue turn advances one alternative. Pending goals are processed FIFO, with conjunction flattened in place. Once pending work is empty, rules are scanned in declaration order and ordered head tuples in occurrence order; the first enabled unused application commits. This reproducible order is a policy of the reference interpreter, not a source-language guarantee.

Head matching binds only rule-local variables. Constructor demands and pure guards can suspend on unknown query variables. The initial guard language is equality entailment: `Guard::Equal` tests equality already established in the store. Binding equations are a distinct `Goal::Unify` variant available only in rule bodies; queries contain ordinary constraints and selected variables. Guards cannot contain binding goals or host-language callbacks.

Every application allocates fresh body variables, retains/consumes the appropriate occurrences and records an ordered occurrence token. Equality uses transactional finite-tree unification with an occurs check. Explicit OR clones the whole remaining branch state. A failing branch cannot affect its sibling. Complete store scans recheck matching and guards after bindings; no wake-up index is assumed correct.

An answer is emitted only when no pending work or enabled unused application remains. It can contain unknown variables and residual constraints. Deduplication compares the output tuple and entire residual multiset under consistent variable renaming. It preserves aliases and multiplicity and does not equate different programs just because they have the same behavior.

## Deliberate limits

A reference step may contain a complete finite unification or exhaustive matching scan. FIFO service prevents an infinite sequence of steps in one branch from starving a finite derivation in another, assuming each step completes and resources suffice. A step budget is not a wall-clock deadline. Recursive term traversal, exhaustive matching, full branch copies, retained history and answer keys can consume substantial time and memory. Canonicalization tries all permutations of residual-only variables, which can be factorial.

`Stats::steps` includes pending-goal handling, applications and completion checks. The other counters distinguish introductions, rule applications, attempted head/occurrence matches, equations/worklist pairs, explicit splits, full branch copies, failed/completed branches and duplicate answers. They do not estimate bytes or claim a performance comparison.

## Validate

```sh
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo tree --workspace --edges normal
```

Tests cover CHR resource handling, freshening, guards, finite-tree equality, isolated alternatives, propagation history, fairness, residual answers and deduplication, plus the demonstration programs. See the [implementation record](docs/implementation/reference-interpreter.md) and [research direction audit](docs/goals/chr-sharing/notes/T018-final-direction-audit.md) for their distinct scopes.
