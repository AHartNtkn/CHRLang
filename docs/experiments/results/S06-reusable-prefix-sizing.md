# Reusable preparation: lower repeated setup, an initial cost

**Retaining lowered artifacts substantially reduces repeated setup and requested allocation in the exploratory long-prefix cases.** Short one-shot and wide queries expose contrary costs. A prospective repeated comparison is still needed before classifying practical gains and losses.

The [registration](../registrations/S06-reusable-prefix-sizing.md) fixes seven source families, six controls, depths 1/8/32, one/eight changing queries and optional consuming suffixes. The corrected matrix completes all 1,524 processes: 504 ordinary timings, 1,008 allocation runs and 12 cancellation runs. All answers, exact allocation replays and requested-live restoration checks pass.

## What retention changes

Reusable mode constructs source-derived target rules for the known ordered query signatures during preparation. A single-query process prepares one artifact; a repeated-query process prepares both alternating signatures. New keys change between queries. The rebuilt control prepares the same parameterized representation for each query; per-query lowering instead embeds the particular query in newly prepared target rules. Ordinary and inferred-specialized execution retain the source rules. All controls use Global source order and scanning.

This design measures a known interface with retained artifacts. It does not simulate unexpected signatures or prove an adaptive artifact cache. Rebuilding every query is an explicit no-retention control, not a claim that every signature change must cause rebuilding when an artifact already exists.

Representative single-sample primary milliseconds for depth 32, eight queries and consuming suffixes:

| Family | Ordinary | Specialized | Per-query lowering | Specialized lowering | Reusable | Rebuilt |
|---|---:|---:|---:|---:|---:|---:|
| plain1 | 0.729 | 0.502 | 0.350 | 0.279 | 0.178 | 0.271 |
| plain4 | 2.430 | 2.015 | 0.695 | 0.617 | 0.290 | 0.660 |
| plain16 | 10.417 | 6.730 | 1.974 | 2.286 | 0.969 | 2.308 |
| choice1 | 0.819 | 0.614 | 0.303 | 1.015 | 0.238 | 0.297 |
| choice4 | 6.071 | 4.354 | 1.928 | 1.737 | 1.500 | 1.673 |
| fail1 | 0.665 | 0.494 | 0.270 | 0.284 | 0.200 | 0.295 |
| fail4 | 2.686 | 1.375 | 0.721 | 0.705 | 0.357 | 0.525 |

Plain families have one/four/sixteen independent chain calls. Choice families independently choose f/g at the chain end. Failure families supply a ground output incompatible with either constructor, yielding no complete answer. The primary endpoint includes preparation, setup, complete execution and observation, engine/answer disposal and prepared disposal. These are individual exploratory observations, not medians or confirmed rankings.

## The important attribution is preparation versus setup

For plain16 at depth32/eight queries with resources, reusable preparation costs 0.518 ms versus 0.040 ms for per-query lowering. Across the queries, setup falls from 1.530 to 0.106 ms; execution/observation changes from 0.393 to 0.335 ms. Source/input-inclusive totals are 2.037 and 1.040 ms. The larger initial cost is included, rather than hidden outside the primary endpoint.

Requested primary allocation for that case falls from 4.450 MB with per-query lowering to 1.745 MB with reusable preparation. Peak requested growth rises from 88.7 to 140.2 kB. Retention can reduce traffic while increasing live demand. The rebuilt artifact requests 4.811 MB, so parameterization alone does not explain the traffic saving.

For choice4, most remaining time is executing and observing 16 alternatives: 1.078 ms for per-query lowering and 1.050 ms for reusable preparation. Setup falls from 0.707 to 0.154 ms. Avoiding preparation does not eliminate choice execution or complete observation.

First-answer time remains separate from total cost. For plain16 it is 0.054 ms under both lowering modes in this sample; the observed benefit is primarily preparation reuse, not a claim of substantially faster first observation. Failed queries have no first answer. All individual phases, source/input-inclusive totals and allocation readings are in the [complete summary](s06-reusable-prefix-sizing-corrected/summary.json).

## Adverse cases remain visible

For a one-step, one-query plain16 source with resources, reusable execution takes 0.281 ms versus 0.090 ms ordinary and 0.109 ms per-query lowering. The corresponding plain1 times are 0.043, 0.037 and 0.039 ms. The wide case is a concrete adverse signal; the small differences require repeated measurement.

The corrected and initial individual timings vary materially in some cells. For example, specialization after lowering in choice1/depth32/eight queries measures 1.015 ms in the corrected run versus 0.376 ms initially, with identical allocation counts. No conclusion should depend on that isolated timing. The next confirmation must retain opposing cases and randomized paired repetitions, rather than treating either single sample as authoritative precision.

## Receipt quality and scope

The initial run recorded the alternating query key seed in the per-sample depth field. Source depth actually remains fixed. That reporting field is corrected, both binaries rebuilt, and the entire registered matrix repeated with the same ordering and bounds. The [initial receipt](s06-reusable-prefix-sizing/summary.json) remains available; the corrected receipt is primary. All 504 allocation totals agree between the runs as well as replaying exactly within each run. Do not combine the two matrices as an unregistered confirmation.

The [audit](s06-reusable-prefix-sizing-corrected/audit.json) verifies counts, command/configuration identity, frozen sources and binary hashes. The corrected source freeze matches the current implementation. The actual runner test covers all six modes and seven families with changed keys, both orders and interruption; scoped strict Clippy passes. Ordinary timing has counters disabled and uses the ordinary allocator. Allocation runs use the separate meter build and describe requested heap demand, not RSS. Native compilation is excluded.

## Next decision

Register confirmation at the opposing short and long source depths, preserving all seven families, both query counts, resource settings and six controls. Estimate practical total-cost differences from paired repeated runs. The principal contrasts are reusable versus ordinary, specialization and both per-query lowering variants; reusable versus rebuilt separates retention from parameterization. Do not choose workload weights or a language-wide artifact policy from this suite.

Unexpected signatures, source changes requiring new analysis, richer arguments, sustained artifact lifetimes and recursive/effectful lowering remain open. This second package since the breadth review supplies a concrete preparation tradeoff; it does not resolve T073, S06 or the architecture goal.
