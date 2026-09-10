# Choice order has a real crossover; explicit execution also has an adverse regime

Reversing support-diagram order trades one favorable arrival order for another. Conditional execution has much lower requested allocation on the independent-choice/check sources, while the low-sharing stream still favors explicit execution. These are source-dependent results, not an architecture winner.

## What the comparison means

The source first creates independent binary choices, then checks their values one at a time. Only the all-true assignment succeeds. One variant checks oldest choices first and the other checks newest choices first; both create the choices in the same order and return the same complete answer. A consuming finish rule and alternating residual query tags are included. Explicit failure after the final check has no successful answers.

The source therefore gives conditional representation an opportunity to avoid branch duplication before the checks reject alternatives. It also varies the order of support operations without changing choice birth order. This is a complete CHR contrast; it does not imply that direct solving or source lowering must enumerate those alternatives.

The six controls distinguish diagram order, chronological observation and Boolean optimization. The ascending/general versus reverse/general pair isolates physical order; ascending/direct versus ascending/general isolates observation strategy. Scan and resumable contextual execution supply independent complete paths. Ordinary ascending/direct calibrates the existing Boolean optimization. No production default changes.

## Evidence and accounting

The [registration](../registrations/S08-order-lifecycle.md) fixes 108 configurations, two allocation repetitions and five ordinary timing repetitions. The [raw evidence](s08-order-lifecycle/) contains build commands, source/binary hashes, run order, complete process outputs and summaries. The [sizing evidence](s08-order-sizing/) establishes only that the two substantive arrival variants complete under the initial bounds.

The source gate checks 32 combinations of family, depth, final failure and query tag in each of three builds: ascending/direct, ascending/general and reverse/general. Each checks analytical answers, an independent scalar evaluator, Scan, conditional, inferred conditional and resumable contextual execution. Every measured process also performs a full independent replay before measurement. The existing reference interpreter is unchanged.

Measured phases include source construction, preparation, changed input construction, setup, execution with full observation, engine disposal, consumer release and prepared disposal. First observation is contained within execution. Compilation, process startup, fixed harness buffers and correctness replay are excluded. Timing therefore describes warmed within-process lifecycles, not cold-cache execution or compilation-inclusive architecture costs. Requested allocation is not RSS.

## Results

All 756 comparative processes complete:216 allocation runs and540 ordinary timing runs. All108 allocation pairs replay exactly. The independent audit verifies frozen sources/binaries, registered order, endpoints, phase continuity and full owner restoration;216 engine-state groups are independent of consumer retention.

Four changing queries, immediate answer release, successful sources, work4/payload8/resource enabled:

| Source | Engine / support policy | Requested bytes | Peak live growth | Lifecycle median ms [min, max] |
|---|---|---:|---:|---:|
| aliases | Conditional ordinary, ascending/direct | 370,332,261 | 2,810,282 | 613.29 [597.76, 625.96] |
| aliases | Conditional combined, ascending/direct | 199,081,941 | 2,829,354 | 367.49 [334.07, 385.97] |
| aliases | Conditional combined, ascending/general | 212,867,885 | 2,833,615 | 468.20 [417.80, 491.46] |
| aliases | Conditional combined, reverse/general | 171,774,165 | 201,755 | 327.13 [306.37, 350.81] |
| aliases | Scan | 9,940,433 | 352,897 | 7.38 [6.05, 9.25] |
| aliases | Resumable contextual | 10,397,489 | 36,047 | 4.39 [3.55, 4.94] |
| oldest-first | Conditional ordinary, ascending/direct | 6,511,168 | 68,996 | 5.50 [5.31, 6.52] |
| oldest-first | Conditional combined, ascending/direct | 2,256,704 | 84,509 | 4.54 [4.20, 4.97] |
| oldest-first | Conditional combined, ascending/general | 2,272,896 | 84,701 | 4.45 [3.55, 4.90] |
| oldest-first | Conditional combined, reverse/general | 1,751,024 | 68,205 | 4.13 [3.73, 4.27] |
| oldest-first | Scan | 83,458,572 | 23,889,472 | 76.97 [73.45, 101.61] |
| oldest-first | Resumable contextual | 150,436,380 | 8,283,952 | 59.85 [56.30, 62.80] |
| newest-first | Conditional ordinary, ascending/direct | 5,636,720 | 52,500 | 4.97 [4.71, 5.08] |
| newest-first | Conditional combined, ascending/direct | 1,730,464 | 68,013 | 3.68 [3.10, 4.27] |
| newest-first | Conditional combined, ascending/general | 1,746,656 | 68,205 | 3.92 [3.60, 4.36] |
| newest-first | Conditional combined, reverse/general | 2,274,224 | 85,069 | 3.94 [3.74, 5.00] |
| newest-first | Scan | 83,458,572 | 23,889,472 | 83.90 [77.99, 96.40] |
| newest-first | Resumable contextual | 150,436,380 | 8,218,941 | 63.03 [59.94, 72.43] |

**The arrival reversal survives complete-source allocation accounting.** With the same general observer, reverse order lowers oldest-first traffic from2,272,896 to1,751,024 bytes, but raises newest-first traffic from1,746,656 to2,274,224. The timing ranges for these small order comparisons overlap; the allocation crossover does not establish a corresponding speed policy.

**Compact supports materially lower stream memory without closing its execution gap.** Against ascending/direct combined, reverse/general lowers alias traffic from199.1MB to171.8MB and peak growth from2.83MB to0.20MB. Its median remains327ms, versus7.38ms Scan and4.39ms resumable. First/full observation, preparation and disposal are included in their stated phases; no compilation cost is included.

**The choice/check source supplies the contrary complete-path regime.** Conditional traffic is roughly1.7–2.3MB with combined policies, versus83.5MB Scan and150.4MB resumable. Conditional medians are roughly3.7–4.5ms, versus60–84ms for the explicit controls. This is a bounded exploratory result; it does not rank stronger source-derived elimination or broader graph organizations.

**Consumer layout differs even for semantically equal answers.** For four retained arrival answers, conditional execution owns6,278 bytes and the explicit controls5,702. An independent layout probe finds residual-vector capacity4 versus1 with one live item, at48 bytes per slot:4 answers ×3 spare slots ×48 bytes =576. Outputs themselves have equal capacities. The audit checks this exact attribution before comparing18 semantic-output groups; actual reported traffic, peaks and retained costs remain unadjusted. Empty-residual streams have equal retained-answer bytes across the compared engines. This is output capacity, not unreleased engine state.

The [layout probe](../../../research/chr-direct-conditional/examples/order_layout.rs), its [observed output](s08-order-lifecycle/layout.log), and the [audit](../../../research/chr-direct-conditional/experiments/audit_order_lifecycle.py) make that post-run investigation reproducible. The discrepancy is too small to explain the substantive ordering. The complete summary retains distinct-stream, tiny, failure, windowed and cancellation cells as well as the displayed cases.


## Limits that matter to the architecture

The arrival source has one successful answer despite many explicit choices. It supplies a favorable sharing regime, whereas streams with many demanded answers expose different work and retention. Neither supplies workload weights. A source-derived solver might avoid much of the choice/check computation; that path has not been qualified in this matrix.

Five timing samples support exploratory medians and ranges. They do not establish a general policy or resolve close comparisons. The matrix tests combined Boolean optimization with each observer/order, not every cache-capacity/order interaction. Sustained intra-query reclamation and more general symbolic representations remain open; final retained node populations do not prove which nodes were reclaimable earlier.

## Four-package breadth review and next experiment

This is the fourth package since the repetition review: Boolean optimization correctness/work, its lifecycle pilot, the order/observer gate, and this lifecycle comparison. Another support refinement could improve constants or remove observation overhead. It would not settle whether a different organization can eliminate the favorable source's work altogether.

**Select complete-source qualification of stronger lowering/solving controls under T078 before more support tuning.** Reuse existing prepared specialization, pure-prefix/recursive lowering and resource-aware candidates where their contracts apply. Record actual eligibility and the work each removes; a rejected source is a checker/capability result, not an architectural loss. Validate complete answers, failures, consumption and fresh query identities before registering any timing. Include both arrival orders and an adverse low-sharing source, then compare costs only for genuinely executable paths.

The strongest ready alternative is T079 non-overlap/effect certification. Its witnesses are already available, but its next experiment still needs a sound property and an additional executable beneficiary beyond unrestricted serial accounting. The new conditional favorable regime makes a stronger elimination control a more immediate challenge to an architectural inference. This is a prioritization judgment, not negative evidence about certificates. Reconsider T079 at the stronger-control qualification boundary; it remains required.

T074 remains unfinished for broader observation, reclamation and support representations. T078 remains unfinished for coherent architecture comparison beyond these sources. No reviewed direction or research goal closes from this pilot.
