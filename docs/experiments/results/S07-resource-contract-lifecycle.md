# Declarations add a small fixed allocation cost in this bounded interface

Checked declarations add 34 requested allocation bytes and 17 peak live bytes in every equivalent-work comparison in this pilot. They add no query-phase allocations. This establishes the heap cost of the implemented checks on these sources, not a reason to require declarations or evidence that they simplify the remaining executor.

The [registered pilot](../registrations/S07-resource-contract-lifecycle.md) completed 672 processes: 192 allocation runs forming 96 exact replay pairs, then 480 ordinary-allocator timing runs. Complete observations, rejection counts, phase order and all owner restoration checks pass. [Audit](s07-resource-contract-lifecycle/audit.log), [all cells](s07-resource-contract-lifecycle/summary.csv), [source/binary freeze](s07-resource-contract-lifecycle/freeze.json), [toolchain](s07-resource-contract-lifecycle/toolchain.txt).

## Equivalent work and its cost

Both modes infer counting from the same immutable source, prepare the existing Scan executor once and reuse it across changed queries. The declared mode additionally checks the resource-access region and ground submitted depth. The table includes preparation, inputs, admission/lowering/setup, complete delivery, cancellation and disposal. Requested bytes are allocation traffic; peak is live requested heap above the initial baseline, not RSS.

| Family and schedule | Choices / depth / reuse | Inferred traffic | Declared traffic | Inferred peak | Declared peak |
|---|---|---:|---:|---:|---:|
| Common, eligible | 0 / 0 / 1 | 28315 | 28349 | 9494 | 9511 |
| Common, eligible | 3 / 16 / 4 | 411690 | 411724 | 52604 | 52621 |
| Independent suffixes, eligible | 3 / 16 / 4 | 526259 | 526293 | 62091 | 62108 |
| Common, alternating missing permit | 3 / 16 / 4 | 462483 | 462517 | 82469 | 82486 |

Across all 32 equivalent-work mode pairs, additional traffic occurs only in preparation. The retained declaration allocates four bytes for `fuel`, five for `start` and eight for its one rule index. Source validation clones that declaration temporarily, accounting for another 17 requested bytes. Ground checking traverses the submitted term without allocating. This explains the measured 34-byte traffic and 17-byte retention increments; longer names or larger access regions would change them.

For common choices three/depth sixteen/reuse four, preparation traffic changes from 12345 to 12379 bytes and retained preparation memory from 4817 to 4834. Every later phase has identical allocation traffic across modes. Missing permits fail counting eligibility while still satisfying the declaration; their ordinary suspended answers remain part of the equivalent-work comparison.

## Rejection is a different outcome

Unknown-depth submissions are valid ordinary queries but violate the declared boundary. The inferred path produces their complete suspended answers; the declared path returns admission errors before counting and search. With reuse four, two ordinary queries and the cancellation submission are rejected by the declared mode. These counts are audited independently in every cell.

For common choices three/depth sixteen/reuse four, total traffic is 412310 bytes for inferred execution and 217252 for declared admission. This is not a speed or efficiency gain on equivalent work. It includes fewer executed queries and fewer produced answers. The [contract gate](S07-resource-contract.md) and [premise audit](S07-resource-premises.md) describe the expressiveness difference and one particular late-ground reformulation.

## Composition and measurement limits

The prepared contract now owns its inferred counting program as well as its rules and declaration. It validates the original submitted query before shortening. A counting certificate supplied for another source cannot enter this path. Source inference can decline without rejecting a valid declared source; query lowering can decline without rejecting a valid declared input. The [new composition test](../../../research/chr-compiled/tests/resource_properties.rs) checks complete changed-query answers and unknown-input admission with and without declarations.

The runner charges cloning the original input into the owned submission boundary; the outer input remains retained until its disposal phase. Successful lowering also creates its actual transformed query. These costs are common to both modes and included. This experiment attributes declaration costs in that concrete lifecycle; it does not prove this is the cheapest possible query-ownership implementation.

Five timing samples per cell are exploratory and noisy. Common eligible choices three/depth sixteen/reuse four has inferred median 459816 ns (262698–525223) and declared median 463017 (265047–470666). Independent suffixes at the same dimensions have medians 601034 and 408212 with substantially overlapping ranges. These samples do not establish that declaration checking speeds execution or precisely quantify its time overhead. No practical-win classifications or workload-weighted averages are assigned.

Source construction before preparation, native compilation, process startup, expected/scalar checks and preallocated bookkeeping are excluded. Execution and observation remain coupled in delivery. The pilot uses small complete rule sets, one declared region and short terms; large-module linking, stronger effect analysis, sustained retention and language-wide modes remain unmeasured.

Both timing and allocation builds disable engine/kernel counters; only allocation runs use the requested-allocation meter. Ten property tests plus four counting-boundary tests pass in [default](s07-resource-contract-validation/tests-default.log) and [counter-free](s07-resource-contract-validation/tests.log) builds. Strict Clippy passes for [allocation](s07-resource-contract-validation/clippy-meter.log) and [ordinary timing](s07-resource-contract-validation/clippy-time.log). [Runner](../../../research/chr-direct-conditional/examples/resource_contract_cost.rs), [driver](../../../research/chr-direct-conditional/experiments/resource_contract_lifecycle.py), [auditor](../../../research/chr-direct-conditional/experiments/audit_resource_contract_lifecycle.py).

## Selection after the bounded contract comparison

Resume T078 with [selective conditional discovery](S10-selective-discovery-entry.md). The declared interface now has executable semantic and bounded cost evidence. Its cheap heap checks neither remove generic execution nor justify excluding valid unknown-input programs. Repeating small-check timings would currently have less architectural consequence than investigating the large remaining discovery costs on branch-specific work.

The strongest alternative is a stronger language/compiler contract that actually removes a runtime responsibility. That remains required under T079/S07, but needs a concrete property and compiler organization beyond making the same declaration mandatory. Other modes, ownership, effect commutation, source inference and sustained lifetime are unresolved. This pilot closes only this bounded comparison; the broader language and architecture goal remain active.
