# R01: bound-key access and reusable preparation

The indexed and scanned paths now preserve full observations and selected applications under the same policy. Ruleset preparation can be reused across independent queries, and execution can be measured separately from first answer export. These are validated controls for the upcoming cost comparison; no timing result selects an architecture yet.

## Implemented boundaries

`PreparedRuleset` owns immutable lowered rules, dispatch, predicate metadata and optional native selectors. Each `start` owns a fresh term arena, bindings, occurrences, histories, queues, indexes and watchers. Query engines retain their shared preparation if the caller releases its handle. Each query's predicate dictionary initialization remains explicit setup work.

Indexed access derives keys only from established ground values. It follows bindings and normalizes constructor structure into the query's arena, so `X=f(Y)` followed by `Y=a` obtains the same key as `f(a)`. Unknown patterns use predicate scans until a justified key exists. Both generic and native paths use the same access plan; buckets retain ordered distinct occurrence IDs.

Committed bindings refresh affected keys and dependencies. Consumption removes reverse index entries. Predicates absent from all prepared heads retain their source data but need no matcher pools, indexes, watches or activations. Their eventual alias relationships are reconstructed from the bindings when observed.

Trace and detailed audit collection are opt-in. `advance(budget)` returns completion/failure status; `observe()` exports only a completed successful state. A cutoff retains resumable execution without publishing an intermediate ground-looking output. Repeated observations inspect the completed state; they are not new stream deliveries. Engine and prepared-program destruction remain explicit lifetime boundaries.

## Independent checks and review

All fifteen focused tests pass: seven access/lifetime tests and eight source semantic tests. Access checks compare scan/indexed traces within each policy, analytic complete observations, independently reconstructed application commits and independently enumerated terminal enabledness. They also interleave different queries sharing preparation, drop the preparer before its engines, and preserve fresh-variable/residual relationships.

Flat-key chains distinguish store size from term depth. Delayed chains require binding-aware maintenance. The equal-key case actually performs a keyed lookup with a large compatible bucket, including a new occurrence after earlier propagation. The repair case grounds nested arguments to values that enable no application. These remain hand-sized correctness cases rather than a performance sample.

Review exposed a fixture that populated equal values without querying a bound key; the current keyed program tests the intended operation and asserts bucket materialization. Root review also identified needless maintenance for predicates absent from every head. A regression now preserves residual-only aliases through nested bindings and fresh bodies while matcher retention remains empty.

Root validation passed `cargo test -p chr-compiled`, workspace Clippy with warnings denied, workspace formatting and whitespace checks. The reference remains independent. Generated selectors contain fourteen programs and twenty-nine rule selectors; artifact SHA256 is `4ae67e92d604470e93a815969d31ae5f5c714aeb81edbe94bd9485d81f52214e`.

```sh
cargo test -p chr-compiled
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

## Cost entry and remaining uncertainty

The [measurement design](../registrations/R01-measurement-design.md) separates preparation, query setup, execution, export and disposal. Hot-path counters still need a compile-time off configuration before primary timing. Counter runs and allocation-metered runs will explain physical work and storage separately; neither should silently become the ordinary runtime measurement.

This indexed implementation normalizes keys in an immutable term arena and maintains ground-argument buckets. It does not establish that this is the best index representation or that indexing always pays. Low selectivity, widespread repairs, key normalization and cold setup are essential counterpressure. Additional index tuning is justified only if measured evidence makes it consequential.

The next task is measurement configuration and a frozen bounded cost pilot. R02's integrated class/index alternative remains eligible and is not required to adopt these boundaries. The [partial-equality analysis](R02-partial-equality-serialization.md) advances its correctness question independently.
