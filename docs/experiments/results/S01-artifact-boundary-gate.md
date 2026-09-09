# Native rulesets compile separately; cost superiority remains untested

The independent artifact boundary works for four source families and seven execution configurations. All 168 queries agree with independent scalar execution. Generated continuations still make cross-crate runtime calls, so compiler configuration needs a controlled comparison before interpreting native costs.

This is T070 correctness and accounting evidence under the [prospective gate](../registrations/S01-artifact-boundary-gate.md). It is one exploratory repetition, with no performance ranking or amortization claim.

## What the experiment establishes

The emitter receives source rules without query inputs. One generic executable and four separately compiled native executables link the same frozen runtime. Hash checks confirm that compiling a ruleset does not rebuild that runtime. Each executable processes changing queries while reusing prepared rules.

| Source | Purpose | Emitted Rust bytes |
|---|---|---:|
| Chain | Recursive multihead execution | 8,461 |
| Private payload | Projected update maintenance and late binding | 6,696 |
| Subscription | Live demands, repeated requests and retirement | 52,189 |
| Sixteen competing rules | Source rule count and dispatch position | 53,160 |

Each source runs at base sizes zero and five, with three queries alternating size and input arrangement. The controls are generic execution, a prepared update plan, and that plan with eligible inferred specialization. Native variants use generated repair, generic repair, prepared repair, or generated repair with inferred specialization. All use Global/Indexed execution; this gate does not compare Active scheduling.

All 56 query processes succeed. Complete answers, including aliases and residual multiplicity, are checked against the independent scalar source executor outside the runtime intervals. A native artifact paired with another ruleset rejects the mismatch before query output. The [receipt audit](s01-artifact-boundary/receipt-audit.json) also verifies artifact hashes and detects four deliberate accounting faults: missing disposal, an incorrect total, false validation and incomplete output.

## What is included in the accounting

The runner separates source construction, preparation, query setup, execution, full observation, engine disposal, answer disposal and prepared-state disposal. The totals reconcile exactly. Preparation consumes the source AST, so its disposal belongs to that joint interval. The emitter separately records generation, source-file writing and final source-buffer disposal.

The timing build disables engine, kernel and observer counters and uses ordinary allocation. This runner does not measure requested heap traffic or RSS. The meter belongs to a separate executable, and the registered runtime build does not enable it.

These sums are not process wall time. They exclude input construction, independent expected-answer construction, validation, stdout and startup. Validation can affect caches and allocator state between queries. Full observation is synchronous, so there is no separately measured earlier answer event. Artifact-file disposal and sustained cancellation/reclamation costs are not measured here.

The runtime build reused partial compilation outputs after a source type error was corrected. Its receipt therefore cannot establish cold runtime bootstrap cost. The shared library includes experimental source builders and oracle support; executable sizes are not estimates of a minimal production deployment. The [raw records](s01-artifact-boundary/artifacts.json) retain exact commands, toolchain, sizes and hashes.

## Compiler configuration is a consequential remaining control

The first artifacts use optimized Rust compilation without link-time optimization. Resolving ELF relative relocations in the two generated chain continuation bodies identifies calls to runtime argument access, binding, eligibility, term construction and access-range operations. An initial text search missed these symbols because their demangled names use `<chr_compiled::Core>::method`; the corrected [machine-code inspection](s01-artifact-boundary/code-inspection.json) maps actual call targets.

This establishes remaining call boundaries, not their runtime cost. The next compiler gate should compare this configuration with a runtime carrying LLVM bitcode and link-time optimization, using the same source and query controls. Charge runtime preparation separately from each user artifact's compilation/linking. Check semantics and inspect which boundaries remain before selecting configurations for the cost pilot.

## Decision and next selection boundary

The gate makes independent per-ruleset compilation measurable without charging a complete runtime rebuild to every ruleset. It does not show that native generation repays its compilation, code size or runtime ownership. The prepared data-plan control remains essential: earlier work found equal maintenance savings without native generation.

Complete the bounded compiler-configuration gate, then return to selection before expanding the matrix. If it succeeds, register lifecycle sizing and confirmation across changed-query reuse, rule count and favorable/adverse work placement, with allocation diagnostics and applicable retained controls. Direct pull-tabbing and derivation reuse remain the strongest distinct ready alternative; they must not wait for every possible access-plan refinement. T070 and the broader architecture goal remain active.

Validation for this change: 16 focused counter-free source/artifact tests pass; the emitter/library Clippy check passes. The saved 56-process gate and artifact hashes pass independent receipt revalidation. Source freezes preserve the runtime, emitter, generator, oracle and checker inputs. No comparative native timing matrix is claimed.
