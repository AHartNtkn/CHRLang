# S05: lifecycle cost of stable equation reuse against explicit and shared execution

Test whether equation reuse over stable handles changes the total cost comparison with direct sharing. Exact-context identity and dependency validity have passed kernel and complete-source correctness gates. This pilot prices their validity and retention costs; it does not select a universal execution policy.

## Decisions and hypotheses

Dependency validity should admit repeated substantive operations across branches with different choice bindings, whereas exact-context identity may miss. Avoided unification may or may not repay lookup, certificate construction, replay and retained state. Shared execution may still avoid branch service that an operation cache preserves. Trivial equality and distinct requests should expose cache overhead. Before/after discrimination tests the sensitivity of sharing without assuming cache reuse is equally affected.

The strongest ready alternative is corrected S04 replay policies. This pilot continues S05 because a complete correctness path is now available and the comparison can directly change whether operation recognition substitutes economically for sharing. Broader replay and generalized continuation tables remain required.

## Exact sources and configurations

Use `research/chr-reuse/examples/lifecycle.rs`, drawing the original common-equation sources from the R05 equation fixture. Four sequential binary choices lead to sixteen raw successful answers or complete failure. Nontrivial terms have 64 nested row constructors with repeated unknown arguments and a terminal success or clash. A query-tag residual changes across the batch while the prepared rules stay fixed.

| Family | Operation placement and contrary mechanism |
|---|---|
| before-success | Common substantive success before constructor discrimination |
| after-success | Same operation after source discrimination |
| before-clash | Common deep clash before discrimination |
| after-clash | Same clash after discrimination |
| trivial | After discrimination, identical closed atomic operands; ordinary identity equality already succeeds cheaply |
| unique | After discrimination, each of sixteen gate rules wraps operands with its own literal choice tuple, producing distinct operation keys |

The unique case retains the substantive inner equality and its successful answer. The trivial case leaves the original output unknowns independent. Independent scalar source execution and explicit full expected answers check these differences, joint aliases, residuals, query tags and all sixteen distinct outcomes. Failed cases have zero answers, not a fabricated first-answer latency.

Seven configurations: ordinary scalar `Machine::step`; cache-intercepted Direct; Exact with capacity32; Dependencies with capacity32; compiled Global/Indexed; direct Conditional; direct choice graph. Both sharing organizations are included because neither has automatic priority. All support the same source gate. The Direct interception control prices cache-free service through the same boundary; ordinary scalar measures overhead introduced by that boundary.

Reuse counts **1 and 8 changing queries** per prepared ruleset. All scalar configurations use shared prepared rule ownership with fresh query arena, frontier and cache. Cross-query cache reuse is not implemented or assumed. Compiled and sharing configurations use their existing prepared interfaces.

Six families × seven modes × two reuse counts = **84 cells**. Five primary repetitions and two separate allocation repetitions = **588 processes**. Separate work diagnostics run twice; they validate complete answers and report cache hits, source equations and kernel pair work. No work counters appear in primary timing builds.

## Lifecycle and controls

Use two warmup queries before measured preparation. Time prepared rule construction, each query setup, execution jointly with complete observation, engine disposal, answer disposal and final prepared disposal. Also report first-answer latency and a separate run stopping at first answer or exhaustion followed by cancellation/disposal. For failure cases, that separate run measures exhaustion; its absence of an answer must remain explicit. Charge prepared disposal once in the primary batch, excluding the separately reported cancellation phases.

Prepared construction includes source cloning and rule validation/plan construction. Query setup includes input cloning and the configuration's actual query initialization. Scalar preparation now shares the validated source rules; it does not precompile source execution. Different setup/execution boundaries must not be compared as isolated equivalent work. Whole runtime batches include both.

Source/fixture generation, expected-answer construction, oracle evaluation and validation are outside primary intervals. Native compilation, process startup and artifact disposal are excluded. These measurements do not establish full architectural lifecycle superiority. Prepared and query disposal must restore requested live allocation to the starting level after the entire session.

Freeze source, lockfile, toolchain, feature commands and binary hashes before the matrix. Use ordinary-allocator counter-free release timing; separate requested-heap allocation builds are not RSS measurements. Record cumulative requested bytes and incremental peak separately. Source effects, matching and complete observation remain included for all configurations.

## Bounds and interpretation

Run serially on the lowest available CPU, shuffled cell order in each repetition using seed20260910. Each process has a 60-second wall limit and 1 GiB address-space limit; each source run has a two-million service-step bound. A cutoff stops the matrix for diagnosis; no timed retry or changed configuration is silently substituted. These are bounded pilot limits, not architectural rejection criteria.

Report per-cell medians and all five within-block runtime ratios against ordinary scalar, Direct, Indexed, Conditional and graph controls. A practical change requires a median at least 20% from one and every paired ratio on the same side of one. Smaller effects and overlapping ranges remain reported without claims of equivalence. Five repetitions are pilot evidence, not population confidence intervals. Require exact two-run allocation replay and two-run work replay. Do not pool workload weights.

A repeated-work benefit must survive its preparation/reuse and contrary controls before a bounded cache recommendation. If a cache loses, distinguish low hit rate, validity cost, retention, source-service overhead and a defective control. If a sharing candidate loses, distinguish discrimination and representation costs from intrinsic sharing obligations. Investigate consequential defects or uncertainty before changing architectural dispositions. This pilot leaves cross-query cache lifetime, indexed wake-up integration, generalized tables and whole-architecture comparison open.
