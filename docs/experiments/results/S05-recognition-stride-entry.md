# Less frequent recognition saves keys but misses reuse

**Recognizing every fourth or sixteenth source state preserves tested delivery and reduces actual key construction.** It also misses reuse and retains more intermediate states on the repeated-work witness. The next experiment must measure that tradeoff against Direct; fewer keys alone do not establish an improvement.

The [registered mechanism](../registrations/S05-recognition-stride-entry.md) extends the existing separated-continuation executor. Every source transition still consumes one service step. A positive stride selects when a full canonical state key is constructed; the root is recognized, and both split children inherit the next recognition distance. Keys still contain active resources, history, pending work, aliases and outputs. Ground observations remain owned by each derivation.

Intermediate nodes remain retained because cached edges can replay them. This experiment changes recognition frequency, not source scheduling or eviction. No whole call runs synchronously in place of one source step.

## Evidence

All six source tests pass with metrics enabled and disabled. The 72-source matrix makes 9,520 per-step comparisons against existing Direct, whole-state and separated controls; each step checks ordered answers and exhaustion. Its 864 cancellation/restart comparisons also pass. Separate resource-consumption and propagation-history sources use the same strengthened helper. Fresh aliases, changed inputs, readable observations, arity near misses, duplicate residuals and failed branches remain covered. Zero stride is rejected explicitly.

For a depth-16 repeated-work source with distinct caller tags:

| Recognition stride | Key requests | Reused transitions | Logical steps | Retained states |
|---|---:|---:|---:|---:|
| 1 | 60 | 52 | 111 | 59 |
| 4 | 17 | 49 | 111 | 62 |
| 16 | 5 | 41 | 111 | 70 |

The logical schedule is unchanged. Wider spacing performs more fresh transitions because recognition happens later. The retained-state increase is a real cost, not a reason to omit those states from accounting.

A single-branch control has no reuse at any stride. Key requests fall from 56 to 14 and 4; all variants take 56 logical steps and retain 56 states. This separates saving useless recognition from gaining reuse.

Scoped Clippy passes. The [metrics receipt](s05-recognition-stride-entry/metrics.log) retains the source matrix and work counts. No test duration is interpreted as performance.

## Next decision

Keep T075 active for requested ownership and complete lifecycle costs of strides 1, 4 and 16 against nonmemoized separation, Direct and existing compiled controls. Include repeated work, no reuse, failed branches, active resource/history differences, large inert observations, changed queries and cancellation. Charge key construction, retained states, edge replay, preparation and disposal. Use separate counter-free timing after ownership qualification.

This is more informative now than immediately implementing a whole-call cache: the existing call table requires a justified complete phase or commutation boundary, while this candidate preserves service on the tested general continuation sources. Direct solving remains a strong alternative for eliminating operations and should be reconsidered at the cost result. The stride experiment does not resolve call-boundary recognition, variable-dependent relevance, failed futures, eviction or sustained ownership.

Package count one after the full portfolio review. The full architecture research goal remains active.

Reproduce:

```
cargo test -p chr-reuse --test inert_residuals -- --nocapture
cargo test -p chr-reuse --no-default-features --test inert_residuals
cargo clippy -p chr-reuse --lib --test inert_residuals -- -D warnings
```
