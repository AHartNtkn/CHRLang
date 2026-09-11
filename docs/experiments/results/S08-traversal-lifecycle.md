# Traversal reuse speeds dependency execution; the direct control remains much cheaper

**Completed-traversal reuse delivers a large lifecycle gain for continuing dependency execution. It does not close the gap to the existing direct engine on these sources.** Templates show little consistent timing benefit, and repeated flat-source sessions do not establish a small-case speed advantage.

The pilot completes 1,008 timing, allocation and residency processes. Every one of the 112 allocation pairs reproduces the earlier ownership gate after normalizing resident reporting buffers, with complete final disposal. A targeted follow-up runs 11,000 complete flat lifecycles; four diagnostic profiles identify context-support checking as a more specific next target than speculative memo-storage replacement.

## What was measured

The experiment retains the [qualified source and ownership comparison](S08-traversal-ownership.md): dependency and template graph execution with traversal reuse off/on, optional support reclamation for continuing queries, and an existing direct control. Sources are pure or consuming. Each process reuses preparation across two marked queries with immediate or all-answer retention.

Continuing queries deliver 32 or 128 answers through a capacity-four queue, verify blocked-producer behavior, then cancel with work pending. Flat queries contain 1 or 32 alternatives and run to exhaustion. Complete aliases, multiplicity, residuals and retained answers are validated outside measured intervals, including after producer disposal.

The lifecycle sum charges preparation, query setup, production/observation, delivery/export, consumer storage/release, cancellation/exhaustion and all producer/prepared/consumer disposal. Five ordinary timing blocks are separate from two allocation blocks and two ordinary residency blocks. Compilation and process startup are outside this endpoint. The initial continuing production batch fills four queue slots; its reported first-batch cost is not an isolated one-answer latency.

## The major gain and its architectural significance

Representative pure continuing sessions: 128 delivered answers per query, two queries, immediate consumer release, no reclamation.

| Execution | Median measured lifecycle | Requested bytes | Peak requested heap above resident inputs |
|---|---:|---:|---:|
| Dependency, uncached | 974.601 ms | 76,294,067 | 678,058 |
| Dependency, traversal reuse | 66.048 ms | 9,689,899 | 682,842 |
| Template, uncached | 6.547 ms | 5,673,647 | 979,184 |
| Template, traversal reuse | 6.120 ms | 6,464,623 | 979,336 |
| Direct | 0.510 ms | 766,053 | 50,788 |

**The dependency work saving survives complete lifecycle accounting.** Fifteen of sixteen continuing dependency comparisons, including reclamation, satisfy the registered timing criterion. Median paired cached/uncached ratios range from 0.068 to 0.390. The remaining small pure/immediate cell has median 0.267 but one paired ratio above one; its individual criterion remains unmet.

**The strongest applicable competitor still changes the conclusion.** Every continuing cached graph comparison qualifies as slower than direct execution. Median paired graph/direct ratios span 9.28–157.50 for dependency execution and 3.09–57.38 for templates, including reclamation. These sources therefore support traversal reuse within dependency execution, not selecting that execution architecture.

**The template result is much less favorable.** Across sixteen continuing template comparisons, three qualify as slower with reuse and thirteen remain unresolved at the prospective 10% criterion. No continuing template case establishes a practical timing gain. Their higher requested traffic remains an exact adverse result; overlapping timings do not turn that allocation cost into a benefit.

## Heap ownership and residency answer different questions

Requested allocation reproduces all earlier favorable and adverse cases. Continuing dependency traffic falls with traversal reuse; continuing template traffic and flat-source traffic rise. Peak requested heap rises in every matched cache comparison. Consumer ownership stays equivalent, so these differences belong to execution and memo maintenance.

Sampled RSS does not track those small heap differences monotonically. In the representative pure dependency case, uncached root-relative sampled maxima are 1,036 and 1,100 KiB; cached maxima are 1,040 and 1,044 KiB. Direct uses 408 KiB above its resident baseline in both runs. Absolute roots vary with process layout, so both absolute and root-relative readings are retained.

The same dependency RSS excess remains at the disposed milestone although all requested live ownership returns exactly to baseline. This is allocator-held residency, not evidence of a retained engine owner. The measurements sample named lifecycle milestones; they are not continuous RSS peaks. Exact requested-heap peaks and sampled process residency must remain separate quantities.

## The small flat comparison was challenged

The first pilot suggested some one-alternative graph/direct gains, but many small comparisons crossed equality. The [follow-up](../registrations/S08-traversal-flat-sessions.md) runs 50 fully prepared, executed and disposed lifecycles per process across all twenty one-alternative configurations, nine ordinary blocks and two metered blocks. No prepared artifact survives between sessions.

All 11,000 lifecycles validate, including 2,000 exact metered lifecycles. None of the twenty-four graph/direct or cache-on/off contrasts satisfies the registered practical timing criterion in the follow-up. Its ranges cross equality, or the median difference is below 10%. The initial small-case gains therefore do not survive this precision check as decision-ready timing evidence. The adverse requested-heap costs remain exact.

This does not show that the designs have equal time, or that direct wins every tiny query. It prevents using these measurements to choose a small-case routing policy. The next paired intervention must retain the flat controls and use closely interleaved measurements if its selection depends on a small timing difference; repeating broad process blocks alone has not resolved that question.

## Profiling changes the next intervention

Production dominates the continuing cost. In the representative cached dependency session, median production is 65.741 ms out of 66.048 ms total. Memo teardown or prepared disposal cannot explain that gap by themselves.

Four CPU-pinned user-cycle profiles cover 240 validated complete child processes. Finite-result validation accounts for approximately 21–31% of sampled cycles; force accounts for 12–32%; source ticking accounts for 11–30%. These diagnostic proportions include the complete process and are not primary timing measurements. Each profile has at least 695 samples and reports no lost samples.

The [annotated validation assembly](s08-traversal-profile/dependencies-false-visit-assembly.txt), read with the source, identifies repeated context-support lookups as a substantial cost. A completed result carries a set of choice assignments; validation repeatedly checks each assignment by searching the current context's ordered map. The active-path cycle scan exists too, but its sampled loop is much smaller in this profile. The data do not justify treating dense-ID traversal memo storage as the main remedy.

**Next, qualify an ordered context-inclusion comparison against the current per-key lookups.** Both maps already have sorted keys, so an ordered walk may avoid repeated searches when supports are dense. Sparse supports and early misses can favor the current lookup method and must be explicit controls. Preserve incompatible choices, context extension, changed supports, constructor versus unfinished-call cycles, resource claims, cancellation and preparation reuse. This is a candidate algorithm change, not an established optimization.

This bounded intervention has a concrete sampled responsibility and can improve existing graph paths without adding a new retained owner. It currently has higher value relative to its implementation cost than replacing memo storage without evidence, or starting a new partner-plan implementation. Compare those alternatives again at its source/work gate or obstruction. T074 remains active; this is package one since the [readiness portfolio review](S02-readiness-portfolio-review.md).

## Evidence

- [Lifecycle registration](../registrations/S08-traversal-lifecycle.md), [profile registration](../registrations/S08-traversal-profile.md), [flat-session registration](../registrations/S08-traversal-flat-sessions.md)
- [Lifecycle freeze](s08-traversal-lifecycle/freeze.json), [raw runs](s08-traversal-lifecycle/runs.jsonl.gz), [analysis](s08-traversal-lifecycle/analysis.json)
- [Flat-session freeze](s08-traversal-flat-sessions/freeze.json), [raw sessions](s08-traversal-flat-sessions/runs.jsonl.gz), [analysis](s08-traversal-flat-sessions/analysis.json)
- [Profile commands and hashes](s08-traversal-profile/receipts.json), [profile analysis](s08-traversal-profile/analysis.json)
- [Lifecycle auditor](../../../research/chr-reuse/experiments/audit_traversal_lifecycle.py), [flat-session auditor](../../../research/chr-reuse/experiments/audit_traversal_flat_sessions.py), [profile auditor](../../../research/chr-reuse/experiments/audit_traversal_profile.py)
- [Ordinary Clippy](s08-traversal-lifecycle/clippy-ordinary.log), [metered Clippy](s08-traversal-lifecycle/clippy-meter.log)

All comparative runs are serial, CPU-pinned and bounded at 60 seconds and 1 GiB, with the established source-service cutoffs. No cutoff occurred. The flat runner's residency snapshots and complete-session loop preserve all source outcomes and measured allocation phases. No language restriction, workload weighting or universal architecture winner follows from this package.
