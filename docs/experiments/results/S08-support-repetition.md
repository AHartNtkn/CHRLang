# Repeated Boolean roots justify a bounded reuse comparison

The depth64 stream repeats substantial Boolean work:8,544 later evaluations account for576,600 serviced frames. The depth-zero source instead uses only cheap identities. Compare allocation-free handling of cheap roots against bounded completed-result reuse before choosing either optimization.

## Evidence and limits

The [registration](../registrations/S08-support-repetition.md) covers conditional/inferred dispatch, alias/distinct outputs and depths0/1/16/64. All32 processes complete with16 exact trace replays. The eight cells with prior allocation evidence retain exactly the same engine tick counts. Full raw answers agree with the independent scalar evaluator and analytical expectations;13 library tests and strict scoped runner Clippy pass. The complete counter-free crate test run also passes; diagnostic examples explicitly require their instrumentation features.

Each arena records canonical operation/operand identities, whether the root has an existing constant/equality shortcut, serviced frames and result. All traced jobs finish in these finite cells; completed repeated roots agree on result. The trace is arena-local: the same numerical handle in another query is not the same Boolean object. Diagnostic storage is intentionally additional allocation; neither allocation nor timing was measured here.

Both dispatch modes and both output families have the following substantive-root counts:

| Stream depth | Substantive jobs | Distinct substantive roots | Repeated substantive jobs | Frames on first evaluations | Frames on later evaluations |
|---|---:|---:|---:|---:|---:|
| 0 | 0 | 0 | 0 | 0 | 0 |
| 1 | 21 | 5 | 16 | 20 | 64 |
| 16 | 1,220 | 596 | 624 | 14,129 | 11,856 |
| 64 | 14,132 | 5,588 | 8,544 | 421,337 | 576,600 |

Cheap-job counts differ by output shape. At depth64, aliases use6,916 cheap jobs while distinct outputs use15,236. At depth zero both use185 cheap jobs and no substantive jobs. These jobs already resolve in one serviced frame; the current implementation nevertheless constructs a continuation and memo-table entry. A direct-result path could avoid that allocation without requiring retained cross-job state.

The repeated-frame count is an opportunity estimate, not achievable cache savings. Trace order records job creation, not the time at which an earlier result becomes available. Interleaved jobs may overlap. Lookup, insertion, retention, eviction and cold misses are uncharged. A bounded cache may retain only a fraction of the5,588 distinct substantive roots, and a whole-query result does not establish useful cross-query reuse.

[Raw traces, summary and audit](s08-support-repetition/) preserve all cells. Trace row fields are operation tag (0 Not,1 And,2 Or,3 Difference), canonical operand IDs, cheap-root flag, serviced frames and result ID. The unfinished sentinel is the maximum usize on the recorded64-bit host. This gate measures top-level apply roots; it does not count potential sharing of internal subproblems across different roots.

## Four-package breadth review

The current S08 cycle has completed cross-query ownership, stage allocation, support-domain allocation and operation repetition. Together they locate a consequential complete-path cost: equal owned outputs, much larger conditional execution allocation, overwhelmingly inside Boolean jobs, with substantial repeated roots. They do not show that the cost is intrinsic or that caching wins.

**The next package compares two independently useful mechanisms:** avoid temporary allocation for cheap root identities, and reuse completed substantive results within one arena under a bounded retention policy. Include ordinary execution, each mechanism alone and their combination where applicable. First prove Boolean meaning, append-only arena validity, independent arena ownership, eviction/recomputation and source correctness. Then register cold/sparse, repeated, changing-context and cancellation/lifetime costs. Do not assume the stream's favorable repetition represents all CHR work.

The strongest ready distinct alternative remains non-overlap/effect certification: it could remove coordination or change language admission, but still needs a sound property and an additional executable beneficiary. The measured Boolean repetition makes a bounded support comparison more immediately discriminating for the credibility of an entire conditional architecture. This is a prioritization judgment, not evidence against that language direction. Reconsider it after the next correctness/mechanism gate, before extending a cache design.

A cache must justify its state and lookup machinery against the simpler identity path and strong Scan/resumable controls. A loss requires checking capacity, reuse opportunity and consequential defects; it must not reject all support representations. A gain requires complete lifecycle and adverse-source confirmation. Broader reclamation, publication, residual outputs, non-overlap, native compilation, coherent architectures and held-out challenges remain required. The goal remains active.
