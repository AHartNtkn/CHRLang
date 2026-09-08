# Regional lifecycle: capacity helps one control, but serial execution remains decisive

All 400 registered processes complete their endpoints, validating 10,064 full observations. All 50 cells have five primary repetitions; there are no cutoffs, timeouts, semantic failures or missing runs. Source and binary hashes remain frozen, and matched-quantum accepted work agrees across regional modes. [Registration](../registrations/R08-regional-lifecycle.md), [raw outcomes](r08-regional-lifecycle/runs.jsonl), [manifest](r08-regional-lifecycle/metadata.json), [audit](r08-regional-lifecycle/audit.json), [all summaries](r08-regional-lifecycle/summary.tsv).

## Cold query lifecycle

Primary timing disables diagnostics across all participating crates and uses the ordinary allocator. It includes source cloning, combined preparation/query/worker construction, service through full unique observation, worker shutdown and engine/prepared release, plus output release after separately timed validation. The owner and workers share three verified distinct physical cores. Source-fixture creation, executable compilation, process startup and owner-thread/process teardown remain outside this query interval.

Finite-case medians in milliseconds, regional Q64/K4:

| Case | Inline | One worker | Two workers | Specialized whole source |
|---|---:|---:|---:|---:|
| One region, zero carry | 0.085 | 0.189 | 0.209 | 0.108 |
| One region, carry256 | 0.509 | 0.726 | 0.720 | 8.170 |
| Two regions, carry256 each | 1.017 | 1.212 | 0.819 | 32.623 |
| Asymmetric, first region heavy | 0.512 | 0.845 | 0.844 | 67.262 |
| Asymmetric, last region heavy | 0.499 | 0.839 | 0.832 | 64.511 |
| Owner product | 1.934 | 2.445 | 2.452 | 5.357 |
| Duplicate product | 0.165 | 0.331 | 0.371 | 4.754 |
| Addition plus type inference | 0.094 | 0.310 | 0.222 | 0.223 |

Balanced two-region Q64 is the only favorable two-worker median against Inline. Its observed ranges narrowly overlap: two workers 0.768–0.906 ms, Inline 0.903–1.046 ms. Under the prospective interpretation rule, this ordering remains unresolved. Two workers clearly beat one worker, whose range is 1.172–1.379 ms. That is evidence for capacity within the worker organization, without a resolved complete benefit against Inline.

Every other same-quantum regional comparison favors Inline over both worker modes with separated ranges. At Q1, balanced two-region execution takes 7.286 ms with two workers versus 1.150 ms Inline. One-region, asymmetric, application and publication controls expose overhead or insufficient parallel work. The combined application is deliberately retained as contrary evidence. Five repetitions are limited evidence, and no workload weights are inferred.

Inline also has separated ranges below Specialized in every finite case except one-zero, whose ranges overlap. The application’s two-worker and Specialized ranges overlap as well. These are complete current-path comparisons; they do not establish that the persistent executor is intrinsically superior to all specialization strategies.

## Work, ownership and the consequential control discrepancy

Separate diagnostic runs validate identical accepted applications, source work and products across matched regional modes. One-region carry executes 513 source applications in both Inline and Specialized, yet Specialized requests 4,106,957 heap bytes versus 807,158 for Inline and takes much longer. Cross-region product duplication cannot explain this one-region discrepancy.

Factoring is separately visible in larger cases: balanced two-region execution performs 1,026 applications in the regional organization versus 2,051 in whole-source Specialized. The asymmetric first-heavy case performs 516 versus 4,111. Source organization and physical per-application overhead both matter; neither application count nor the worker count alone explains the complete results.

Read-only inspection identifies a concrete causal candidate in the compiled Indexed path. Every new carry occurrence invokes `Core::refresh`; dependency discovery visits the remaining unary suffix, and `ground_key` reconstructs a canonical key by recursively visiting its nodes. Immutable ground suffixes recur at decreasing depths. This can introduce quadratic maintenance despite linear source applications. Specialized head binding itself passes child handles rather than rebuilding the syntax tree. This is source evidence for the next causal gate, not a measured attribution of all excess time or a validated optimization.

All 22 nonthreaded allocation cells restore their preconstruction baseline. All 28 threaded cells retain 48 requested bytes after query, workers and outputs are disposed. The [isolated channel probe](r08-regional-runner/channel-probe.log) independently reproduces receiving-thread lifetime retention of that size; the CHR observations are consistent with that ownership. Every delta is retained in the summaries. Requested heap is not RSS, and owner-thread teardown is not timed.

Prefix/refutation endpoints preserve accepted work while actual speculative service varies. For the stream, Q1 accepts 35 source steps and executes 35–36 across modes; Q8 accepts 71 and executes 72–79. Refutation Q1 accepts five and executes five–six; Q8 accepts three and executes seven–11. Joined diagnostic accounting includes unaccepted service. These outcomes concern unique-answer regional progress and cancellation, not the raw ongoing-stream contract used in R06.

## Architecture disposition and next investigation

Do not expand the worker interface from this result. Its best complete comparison against Inline remains unresolved, and all contrary regional controls favor Inline. Permanent-region certification, product ownership and source independence remain useful separately from parallel workers. The certificate covers disconnected predicate families, not arbitrary independent calls to the same predicate, temporary independence or connected synthesis. A stronger serial factorized executor has not been compared.

T044 selects one bounded causal gate for repeated structural maintenance in the current Indexed serial control. Establish its actual dependency/key work on a deterministic unary-suffix witness and a late-binding contrary case before choosing a repair or any prospective timing comparison. This can change which costs belong to indexing and representation, and how current execution controls should be interpreted. It has greater immediate value than another worker sweep or automatically extending the serial executor.

The proposed causal witness uses only `carry(X,s(N)) <=> carry(X,N)` with `carry(a,s^n(z))`, at depths 8, 16 and 32. It predicts exactly n applications but `(n+1)(n+4)/2` dependency/key visits, with no newly allocated normalization nodes despite repeated lookup-key construction. Existing counters can test this without adding instrumentation. A late-binding/alias witness must preserve open-term dependency repair. These are prospective predictions, not additional measured results.

Static-lowering eligibility and compilation lifecycle remain the strongest broader alternative. They can eliminate computation outright, but require a reusable fragment, independent correspondence and charged preparation/toolchain/loading. If the causal gate does not expose avoidable maintenance, or correction requires substantial redesign, reconsider that broader direction. No automatic implementation queue or universal architectural winner follows.

The runner’s feature-mode semantic gates, explicit cutoff tests, strict Clippy, formatting and meter self-check passed before registration. Independent review checked the prospective design and subsequent interpretation. The goal remains active; this receipt completes T043’s bounded pilot.
