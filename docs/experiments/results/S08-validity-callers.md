# Two output passes repeat the same validity work

**The second output-obligation pass accounts for 15–21% of visited support entries in the dependency configurations. Its visited-entry total exactly matches the first pass in every measured configuration.** This identifies a bounded reuse candidate: remember validity decisions within one answer, while checking any newly appended obligations normally. It does not establish a speedup or justify caching across answers or changing contexts.

The campaign completes **768 processes, 384 exact repetitions and 192 diagnostic/control ownership pairs**. All complete answers agree with independent scalar execution; all owners are disposed. Inclusion checks request zero heap allocations. Their cost is checking existing state; graph allocation occurs outside those predicate calls.

## What was measured

The [registration](../registrations/S08-validity-callers.md) adds optional caller diagnostics at all twelve demand-engine inclusion sites. The disabled macro expands to the existing predicate call. Instrumentation counts calls, accepted results, support/context sizes, support entries visited, cursor steps and range constructions; callbacks measure requested allocation around each predicate. These are algorithm operations, not B-tree comparison counts or CPU percentages.

Ninety-six source configurations cross dependency/template/direct execution, repeated/distinct calls, successful/failed tails, pure/consuming resources, depths8/32 and forward/reverse constraint insertion. Each preparation serves two queries with distinct marker facts; answers remain owned until after producer/preparation disposal. Lookup and seeking each run diagnostics off/on twice. The finite stream source helper is reused, with independent complete raw-answer validation and complete exhaustion. The earlier [continuing profiles](S08-fresh-graph-profile.md) retain cancellation and consumer-pressure evidence; this finite campaign measures the failed-tail and finite-resource contrasts needed for caller attribution.

The counter gate distinguishes success from rejection at the first or last support entry. Both context builds also pass the independent assignment-truth and graph/store regressions: 27 tests each. Three separately registered native experiment tests are intentionally not run as unit tests. Scoped Clippy passes. A symbol inspection finds no validity-profile symbols in the disabled release binary.

## Which callers perform the work?

Representative seeking configurations: repeated calls, depth32, forward insertion and successful tail. Counts include both changing queries. The table reports **visited support entries**, preserving the distinction from wall time.

| Caller | Dependency, pure | Dependency, consuming | Template, pure | Template, consuming |
|---|---:|---:|---:|---:|
| Resource birth | 0 | 0 | 0 | 0 |
| Prior consumption | 0 | 25,056 | 0 | 25,056 |
| Recursive result following | 0 | 0 | 0 | 0 |
| Result selection | 69,088 | 115,080 | 1,088 | 19,560 |
| Choice birth | 0 | 1,922 | 0 | 6,832 |
| Pattern matching | 0 | 0 | 0 | 0 |
| Lifting | 0 | 0 | 0 | 0 |
| Finite validation | 69,088 | 96,714 | 1,088 | 18,812 |
| First output-obligation pass | 57,378 | 58,432 | 1,700 | 1,700 |
| Second output-obligation pass | 57,378 | 58,432 | 1,700 | 1,700 |
| Output result selection | 15,328 | 28,416 | 578 | 13,666 |
| Tick eligibility | 7,810 | 8,372 | 1,768 | 1,696 |

Zero visited entries does not imply zero calls. Resource-birth supports are empty in these witnesses; the predicate still executes. Recursive-following, pattern and lifting sites have zero calls throughout this finite matrix and are not credited with costs or savings. Any change that affects them still needs their own source gates.

All 96 lookup/seeking pairs preserve requested ownership, caller decisions and visited support counts. Seeking changes cursor/range work, not which source work completes. Every inclusion scope requests and frees zero bytes, and the diagnostic/control lifecycle allocations match exactly, phase by phase. This validates allocation transparency of the instrumentation; its runtime overhead is not used for timing claims.

## What the adverse cases change

Failed tails exercise exhaustion through rejection while preserving earlier answers. In depth32 consuming dependency execution, the failed-tail session performs 365,340 support visits; 53,700 (14.7%) belong to the second output-obligation pass. Prior-consumption checks reject 1,984 of 2,048 requests. This would be missed by a success-only inclusion microbenchmark.

The smaller template cases supply an overhead control: eight of its 32 seeking configurations visit no support entries at all, despite executing predicate calls. In templates with nonzero visits, the second-pass share ranges from zero to 21.5%. An added per-answer validity vector could therefore cost memory and time without saving a scan in those cases. Those cases must stay in the intervention comparison.

## The next experiment and its decision value

**Qualify reuse of obligation validity within one answer under T074.** The first answer pass checks obligations before finite validation and force; the residual pass checks them again. Current source inspection shows obligations are appended with owned supports, and the context parameter is immutable during `answer`. That suggests earlier decisions can be reused for the original prefix, but it is not yet a completed correctness argument.

Register the intervention before implementation and cost runs. Its source gate must force new obligations during answer construction, preserve early failure and constructor-cycle detection, distinguish incompatible choices and consuming claims, and check retained answers across changing queries. Compare the original prefix's decisions to an independently recomputed path; newly appended obligations must receive fresh checks. Keep empty-support, short-query and immediate-failure cases to expose allocation/setup overhead. Then separately compare work/requested ownership and ordinary complete lifecycle time against unchanged graph and direct controls. Do not extrapolate support-visit savings to time.

This has a narrower ownership and validity obligation than a general cross-context memo table, and directly targets duplicated work established by the campaign. Result selection, finite traversal and consuming-template validity remain substantial; their costs are not resolved by this choice. Broader integration, solving, generated plans, sustained and parallel ownership, and language alternatives remain required in the [question map](../question-to-experiment-map.md). Package count two; full portfolio review within two more packages, earlier if the architectural comparison changes. The goal remains active.

Evidence: [analysis and every caller row](s08-validity-callers/analysis.json), [frozen commands/sources](s08-validity-callers/freeze.json), raw runs, test logs and the counter gate's failing/passing evidence are retained together. Reproduce with `python3 research/chr-reuse/experiments/validity_callers.py --audit`.
