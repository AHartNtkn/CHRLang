# Continuing execution: repaired coverage, measured crossover, and repeated graph traversal

**Aggregate binding coverage repairs the conditional engine's 512-answer service cutoff, but its total cost crosses over with workload size.** At 128 answers per session it saves elapsed time in every tested scenario; at 32 answers with consuming resources it adds cost. Separately, graph diagnostics identify repeated traversal of completed calls as a concrete cause of dependency execution's scaling problem. Neither result selects an architecture.

The fixtures now exercise two distinguishable continuing queries against one prepared rule set, bounded delivery, cancellation, retained answers after producer disposal, and exact-capacity output packing. Every delivered answer is checked before and after packing outside measured intervals. These are continuing prefixes, not finite-source exhaustion claims.

## What was repaired and why

The strong conditional control already includes overlap shortcuts and bounded observation lead. Its resource source still reaches the original 200-million-call service bound before the 512-answer lifecycle endpoint. Stage attribution finds that nearly every body binding probe tests a region disjoint from the stored binding. Repeatedly walking those bindings spends substantial work discovering nothing.

The experimental repair maintains the union of a variable's binding regions. When a requested region has no overlap with that union, equality can skip the entire binding population. Nonempty and partially overlapping regions retain ordinary binding traversal. Union maintenance belongs to the existing serial writer and completes with binding publication; dependency touches and stale-job checks remain intact. Constructor matching also uses the coverage test, and its cost is included.

An independent finite-equality oracle, matching/resource tests, interrupted-writer tests and a targeted disjoint-region regression check the repair. The regression first fails with 32 binding visits, then passes with zero while preserving both old and new region meanings. The coverage variant completes all 12 lifecycle qualifications, including both ordinary and metered resource sessions at 512 under the original service bound.

## Work attribution explains the improvement—and the remaining expense

Single-session diagnostic counts at 512 answers:

| Work | Pure control → coverage | Consuming-resource control → coverage |
|---|---:|---:|
| Total service calls | 117,555,307 → 50,059,401 | 320,076,169 → 190,328,657 |
| Body stage calls | 68,835,357 → 1,602,136 | 132,899,231 → 2,400,407 |
| Application stage calls | 805,436 → 805,438 | 138,858,160 → 139,627,035 |
| Body binding probes | 131,744 → 0 | 263,586 → 0 |

Application includes matching and application management. Its continuing dominance is a reason to investigate plans and partner ordering, not evidence that all of those calls are matcher work. The matching coverage check does not reduce this stage on these sources. Coverage also changes support ownership and can raise the requested-heap peak.

## Complete lifecycle costs have a crossover

The registered pilot runs 320 processes: 32 warmups, 160 primary timing runs, 64 allocation runs and 64 separate RSS runs. Each process includes preparation, two query setups, production and first observation, queue pauses, delivery, optional packing, consumer ownership, cancellation and disposal. Primary timing uses an ordinary allocator with counters disabled. Allocation diagnostics measure requested heap, not resident memory. All allocation repetitions reproduce exactly and restore the initial requested live heap.

Each row below summarizes four separately measured retention/packing scenarios. Ratios are the range of those scenarios' five-pair median coverage/control ratios, not pooled workload weights or confidence intervals.

| Source and answers per session | Total elapsed ratio | Requested allocation traffic | Interpretation |
|---|---:|---:|---|
| Pure, 32 | 0.54–0.75 | about 7.68 → 4.30 MB | Lower pilot medians; one individual paired ratio is adverse. |
| Pure, 128 | 0.44–0.48 | about 223.6 → 44.6 MB | Large favorable effect throughout the sampled repetitions. |
| Consuming resource, 32 | 1.11–1.25 | about 14.8 → 15.2 MB | Maintenance and changed execution cost more than they save here. |
| Consuming resource, 128 | 0.84–0.89 | about 604.6 → 453.5 MB | Favorable pilot medians, smaller than the pure-source gain. |

Every scenario's median absolute elapsed difference exceeds the registered 1-ms signal floor. Five repetitions are descriptive evidence; they are not a statistical confirmation. The exact per-scenario medians, paired ranges and ownership totals are in [the audited cost table](s08-binding-coverage-cost/audit.json). Coverage remains an experimental comparison, not a default policy. The adverse resource result calls for separating equality and matching coverage costs and testing nonempty/partial-region cost witnesses before broader adoption.

Packing all retained answers reduces post-producer requested ownership from 123,136 to 86,272 bytes at 128 answers per query: 36,864 bytes saved across 256 answers, or 144 bytes per answer. It adds 12,288 requested bytes over the lifecycle. At immediate release it saves no final consumer ownership. Its small elapsed contribution cannot be selected from these noisy total-time differences. Coverage's peak is not uniformly lower: pure immediate-release at 32 rises from 275,642 to 380,566 bytes despite lower allocation traffic.

Compilation economics are a separate required comparison; these sums cover the runtime lifecycle, not compiler costs. RSS receipts remain process-residency observations and are not substituted for exact ownership accounting.

## The graph obstruction is repeated completed-call traversal

The original nine-mode gate has 108 processes: 98 pass, two conditional resource service cutoffs, and eight dependency/dependency-reclaim 512-answer wall cutoffs. These failures remain in the receipts. Some remaining qualification work overlapped diagnostics/builds, so those wall cutoffs are not quiet comparative timing results. A separately registered quiet graph probe completes all 20 processes, including single-session 512-answer endpoints.

At 512 pure-source answers, dependency execution makes 45,885,521 force entries and 45,884,495 finite-validation visits. Template execution makes 706,548 and 576,242 respectively. The resource case has the same broad pattern. At 128, the passive query marker adds just 128 force entries and no validation visits. Changing that fixture detail therefore does not address the main work.

Dependency execution retains many more completed results and obligations. Its answer and force paths repeatedly walk their common completed chains. The next graph experiment must share this work under a sound graph/context validity condition, while preserving resource updates, recursive probing, late constructor-cycle failures and unobserved result validation. Omitting those checks would invalidate the comparison.

## Decision and next experiments

The [portfolio review](S08-continuing-selection-review.md) retains completed-graph traversal as the leading architectural follow-up: it could determine how much of templates' continuing advantage is avoidable repeated work. Before that larger implementation, the observed small-resource regression warrants a bounded equality-only versus equality-plus-matching coverage ablation; it can identify whether an added boundary check is paying for itself. Broader integration, partner plans and coarser recognition remain required alternatives. One package has completed since the full review; review the full portfolio within four packages. T074 and the research goal remain active.

Registrations: [lifecycle entry](../registrations/S08-continuing-lifecycle-entry.md), [resource attribution](../registrations/S08-continuing-resource-attribution.md), [coverage qualification](../registrations/S08-binding-coverage.md), [graph attribution](../registrations/S08-continuing-graph-attribution.md), [cost pilot](../registrations/S08-binding-coverage-cost.md). Frozen sources, binaries' hashes, raw outcomes and validation logs are in [the lifecycle directory](s08-continuing-lifecycle/) and [cost directory](s08-binding-coverage-cost/). Reproduce the receipt audit with `PYTHONDONTWRITEBYTECODE=1 python3 research/chr-reuse/experiments/audit_continuing.py`.

The committed runner uses an iterator spelling accepted by Clippy and explicit Cargo feature gates for diagnostic examples. Frozen measured source archives retain the exact runner and manifests used for the pilot; these hygiene edits do not retrospectively change those measurements.
