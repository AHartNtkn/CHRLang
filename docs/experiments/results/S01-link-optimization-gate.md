# ThinLTO is viable and changes the native call boundaries

ThinLTO preserves the checked source behavior and changes the generated continuation code. It also adds per-artifact compilation work. Both compiler configurations therefore remain relevant to the lifecycle comparison; this gate selects neither a runtime winner nor a language architecture.

The [prospective registration](../registrations/S01-link-optimization-gate.md) compares ordinary optimized linking and ThinLTO over identical source files and one fixed runtime containing LLVM bitcode. The experiment completed within every registered resource bound.

## Correctness and compilation boundary

All 112 query processes completed, checking 336 changed queries against independent scalar source execution. Each compiler configuration also rejected a mismatched ruleset before query output. Prepared rules are reused across queries, and runtime phase totals reconcile under the preceding artifact gate's accounting rules.

The shared runtime was built in a target directory that did not previously exist. Its rlib hashes stayed unchanged through all ten artifact compilations. Source hashes match the preceding gate. This establishes independent ruleset compilation with ThinLTO; the linker can optimize the runtime's bitcode without rebuilding its Rust source for each ruleset.

The [validation receipt](s01-link-optimization/validation.json) rechecks all query records, runtime/executable hashes and both machine-code inspections. The [source freeze](s01-link-optimization/source-freeze.json) identifies the unchanged runtime and oracle plus the new experiment driver. No engine behavior changes were needed.

## Code changes, not a runtime speed claim

Both chain executables expose two inspectable generated continuation bodies. Without LTO, their resolved call targets include Core argument access, binding, eligibility, term creation, variable creation and access-range operations. With ThinLTO, several of those direct boundaries disappear from the sampled bodies; lower-level arena, ground-key and B-tree operations remain, as does access-next.

The static number of call sites increases from 103 to 121. This is consistent with expanding helper bodies and exposing their internal calls, but is not a dynamic work count. Fewer named high-level calls cannot establish faster execution. The recorded inspections resolve ELF relative relocations and direct calls; register-indirect and unresolved targets are identified explicitly. See [ordinary linking](s01-link-optimization/inspection-off.json) and [ThinLTO](s01-link-optimization/inspection-thin.json).

## Exploratory compilation costs

These are single observations in a fixed run order. They establish the scale of this gate, not repeatable cost estimates, confidence intervals or amortization thresholds.

| Artifact | Ordinary compilation, seconds | ThinLTO compilation, seconds | Ordinary executable, bytes | ThinLTO executable, bytes |
|---|---:|---:|---:|---:|
| Generic wrapper | 0.064 | 2.729 | 5,050,208 | 2,885,304 |
| Chain | 0.173 | 2.746 | 5,069,224 | 2,907,560 |
| Private payload | 0.168 | 2.992 | 5,067,560 | 2,904,240 |
| Subscription | 0.498 | 3.040 | 5,138,528 | 2,993,840 |
| Sixteen competing rules | 0.531 | 3.192 | 5,126,312 | 2,987,736 |

The shared runtime build took 5.659 seconds. This is a fresh Cargo target, not a cold operating-system filesystem cache. The executable includes experimental input/oracle support, so its size is not a minimal production deployment estimate. The generic executable's compilation is reusable across rulesets; native source generation and compilation are ruleset-specific. A fair amortization comparison must preserve that distinction.

Raw compilation receipts record wall time and child CPU. Runtime receipts remain accounting/correctness evidence only. Allocation traffic, sustained retention, cancellation, source generation and artifact disposal need their applicable measurements in the lifecycle study. No compilation ratio or runtime timing from this one-repetition gate selects a compiler policy.

## Breadth review and next investigation

T070 has now completed four bounded packages: native continuations, projected updates, independent artifacts and cross-crate optimization. They have established a viable execution contrast and credible preparation controls. They have not measured whether those mechanisms earn their total costs. Further code inspection alone is less valuable than a complete cost contrast.

The strongest distinct ready alternative is direct pull-tabbing and derivation reuse under S03-A. That could replace explicit search organization and remains required. T070's next package receives priority because the native and prepared alternatives now work on the same source, and their missing compilation/runtime tradeoff can be measured without implementing another engine. A gain could justify generated execution; a loss could favor prepared plans despite equal maintenance savings; opposing regimes would require explicit reuse and source-property conditions.

Proceed to bounded lifecycle sizing, with both link configurations, source-equivalent prepared/specialized and retained controls, allocation diagnostics and changed-query reuse. Register confirmation after sizing. Reconsider selection after the first complete cost contrast; do not wait for every join-order or discrimination policy. A semantic failure requires repair, a consequential cutoff requires diagnosis, and overlapping evidence requires more precision only when it could change the decision.

Broader access policies, pull-tabbing, contextual equality, broader solving/lowering, language comparisons, sustained lifetime, connected parallelism and whole-architecture challenges remain required. This breadth checkpoint authorizes the next cost contrast, not resolution of those directions. T070 and the goal remain active.
