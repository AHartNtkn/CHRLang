# A6 randomized observation lifecycle costs

Status: registered before confirmatory runs. Reuse the exact four-policy driver,
16source workloads and185-column lifecycle harness validated by the pilot. No
algorithm change is included. This is a bounded comparison of these workloads,
not production architecture selection.

## Hypotheses and contrasts

Graph comparison should preserve the pilot's duplicate-export allocation savings
and distinct-stream overhead. Whether those savings reduce runtime, and whether
extra retention raises peak memory, remains empirical. Match the comparator
algorithm when attributing representation/export costs. On repeated symmetric
residuals, legacy mapping-copy costs may explain benefits independent of export.
First-answer latency has no duplicate-export opportunity yet and may expose capture
or preparation overhead. Tiny-payload workloads expose common source preparation;
they do not isolate differential cross-query arena retention.

For every workload, report four candidate/control contrasts: eager_compare /
eager_clone (clone ownership); eager_graph_compare / eager_compare (mapping
strategy); graph_compare / eager_graph_compare (matched comparator, representation
and export policy); graph_compare / eager_compare (whole policies). Never attribute
the last contrast to export avoidance alone.

## Exact sequence and bounds

Use all16pilot IDs and all four policies. With Python Random seed140907, shuffle
the64cells independently within each complete block. Run two System warm-up
blocks, seven System timing blocks, then three separate Meter blocks:576System
and192Meter children,768total. These are fresh processes and fresh engines;
warm-up does not mean a retained engine, compiled query cache or warm session.
Record the exact resulting order before the first child.

Each child has30seconds,1GiB address space and100,000source steps. No concurrent
builds, tests, profiling or independent experimental work during timing. Compile
both release binaries first, then freeze source/binary hashes, compiler/host and
all exact commands. Reserve versioned records exclusively. Preserve every failure
and investigate an unsuccessful version before interpretation; do not filter runs
for favorable timing or drop adverse workloads.

Require full answer/exhaustion/raw/export/key checks, normalized source counters,
matched rollback-comparator counters and System/Meter semantic/work agreement on
all768children. Allocation readings must release measured ownership and exclude
validation by the pilot's interval arithmetic. Require repeated allocation metrics
to agree or investigate differences; never average unexplained instrumentation
variation. All raw timing values remain visible, including warm-ups.

## Interpretation fixed before data

Primary timing is cold exhausted delivery. Also report first delivery, preparation,
and synchronous cold-plus-disposal intervals. For each control/candidate pair, use
seven same-block ratios and report median plus observed min/max. Mark ranges
separated only when the candidate's maximum is below the control's minimum (or
conversely); otherwise call the comparison overlapping. This is a descriptive
finite-sample criterion, not a population confidence interval or multiplicity-
adjusted significance test. Do not collapse workloads into one speed score.

Report allocation traffic, calls, prevalidation peak above baseline, index-drop
release, engine-drop release and delivered-output retention separately. Lower
traffic is not necessarily lower peak or shorter latency. Preserve contrary
controls and inconclusive timings. If interpretation depends on allocator/host
variance, larger graphs or different duplication rates, register the follow-up;
these outcomes do not justify closing those questions.
