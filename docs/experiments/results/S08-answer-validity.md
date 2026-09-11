# Within-answer validity reuse trades scans for temporary ownership

**The candidate avoids all measured second-pass validity checks, while increasing requested allocation in every graph configuration.** It preserves complete answers and all other caller work. Ordinary lifecycle timing is the next decision gate; work savings alone do not establish that the cache is economical.

## The implementation and correctness argument

The optional `answer-validity` feature records each first-pass obligation decision in a temporary boolean vector. The residual pass reuses a recorded decision for the original prefix and checks any new obligation normally. The vector is owned by one `answer` attempt and is disposed on success, failure, Progress or Split. It cannot carry validity between contexts, answers or queries.

The governing invariant is local: the context is immutable during the attempt, and existing obligations retain their owned support maps. Calls append obligations rather than altering earlier supports. Source inspection also shows that ordinary call expansion returns Progress, so an attempt that creates a child typically stops and retries with a newly built vector.

The new gate switches incompatible contexts false/true/false and checks exactly which residual survives. It then forces a child-producing call during answer construction, verifies new obligations were appended, verifies both expansion retries, and checks the final ground output and empty residual. The work assertion fails before implementation because the original path rechecks two prefix obligations; it passes with reuse. The same semantic gate also passes in the uncached control.

Both lookup and seeking pass 31 graph/context tests, including completed-result constructor cycles, incompatible choices and resource claims; three separately registered native experiment tests are not part of the unit run. Scoped Clippy passes. No source restriction or reference-interpreter change is introduced.

## Complete work and requested ownership

The [registered campaign](../registrations/S08-answer-validity.md) completes **768 processes, 384 exact repetitions and 192 diagnostic/control ownership pairs**. It retains repeated/distinct finite streams, successful/failed tails, pure/consuming resources, depths8/32, reversed insertion, two changing queries per preparation and answers retained through producer disposal. Independent scalar execution validates complete raw answers.

Against the frozen caller-attribution control:

- All 128 graph configurations perform zero second-pass checks for their captured prefixes. Every other caller row—including calls, accepted results, visited entries and cursor/range operations—is unchanged.
- All 128 graph configurations request more memory: **16–10,938 additional bytes per two-query session**.
- Peak requested ownership rises in 64 configurations, by at most130 bytes; it is unchanged in the other64.
- All64 direct-control configurations preserve complete requested bytes, peaks, answers and diagnostic rows.

Representative seeking sessions: repeated calls, depth32, successful tail, forward insertion. The recorded savings count support entries, not time.

| Execution | Second-pass support visits avoided | Complete requested bytes, control → reuse | Peak, control → reuse |
|---|---:|---:|---:|
| dependencies, pure | 57,378 | 1,598,314 → 1,604,618 | 239,180 → 239,180 |
| dependencies, consuming | 58,432 | 2,103,990 → 2,114,842 | 294,076 → 294,076 |
| templates, pure | 1,700 | 632,716 → 632,770 | 127,627 → 127,627 |
| templates, consuming | 1,700 | 833,172 → 833,852 | 159,739 → 159,739 |

Thirty-two graph configurations previously visited zero support entries in the residual predicate. Reuse still avoids those predicate calls but adds16–56 requested bytes. They are important timing controls: an empty predicate is cheap, and avoiding it need not repay a temporary owner. Failed answer attempts also pay for the vector and are included in these lifecycle totals.

Inclusion itself still allocates nothing; the extra requested bytes belong to the newly introduced temporary storage. Requested heap ownership is distinct from RSS. The campaign uses separate diagnostic and metered control builds and makes no runtime-speed claim.

The first post-run audit exposed a tuple/list comparison in the auditor: a JSON case list was compared to the equivalent internal tuple. Cases were already matched through normalized dictionary keys. The audit now compares all result fields after that key match; no raw samples or engine behavior changed. All ownership/control checks pass. The execution-time source snapshot retains the initial auditor, while the current auditor contains this correction.

## Next decision: does saved checking repay the vector?

Keep T074 active. Enable ordinary-allocator measurement in the same finite source runner, preserving the existing complete-session endpoint and independent checks outside intervals. Register interleaved cache-off/on dependency and template runs with the direct control, lookup/seeking, changing queries, failed tails, consuming cases and the zero-support controls. Charge preparation, execution, observation and all disposal; isolate diagnostic builds from timing. Use repeated complete sessions per process and investigate consequential contrary blocks without changing the endpoint afterward.

This is more informative now than another cache refinement: the mechanism and ownership costs are qualified, and timing could support retaining it, removing it from a prospective configuration, or narrowing its useful regime. Broader result selection and consuming-template validity remain separate costs. The next package must include the full portfolio review against other graph work, integration, solving and generated execution. Package count three; the research goal remains active.

Evidence: [comparisons](s08-answer-validity/comparison.json), [all caller rows](s08-answer-validity/analysis.json), [freeze](s08-answer-validity/freeze.json), raw runs and source/test logs. Reproduce with `python3 research/chr-reuse/experiments/validity_callers.py --answer-validity --audit`.
