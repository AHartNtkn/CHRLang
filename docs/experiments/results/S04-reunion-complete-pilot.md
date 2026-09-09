# Elimination changes the reunion comparison; reunion retains conditional value

Checked countdown elimination makes the long private-work examples much cheaper. Reunion still improves the measured plain and equality-join cases after elimination, while payload-heavy and small cases expose costs. These are exploratory results for checked finite sources, not a general architecture choice.

The [registered pilot](../registrations/S04-reunion-complete-pilot.md) completed **4,032 processes**: five ordinary timing samples and two allocation samples for each of 576 configurations. All complete-answer checks, 576 exact allocation pairs, phase continuity and query/prepared disposal checks pass. An [independent audit](s04-reunion-complete/independent-audit.json) reconstructs commands, phases, totals, memory pairs and source/binary hashes from the receipts.

## Eliminating work is a stronger control than executing it faster

These are median complete costs in milliseconds for four owners and four changing queries. Preparation, input, query checking/transformation, setup, execution/observation and disposal count. Base depth zero includes effective depths zero and one across changed queries; depth 48 includes 48 and 49.

| Source | Base depth | Copy | Reunion | Shortened Copy | Shortened reunion |
|---|---:|---:|---:|---:|---:|
| Plain | 0 | 0.605 | 0.411 | 0.555 | 0.375 |
| Plain | 48 | 35.427 | 9.561 | 0.603 | 0.472 |
| Equal join | 0 | 0.605 | 0.388 | 0.591 | 0.403 |
| Equal join | 48 | 35.371 | 9.500 | 0.607 | 0.473 |
| Late binding | 0 | 1.250 | 0.948 | 1.102 | 0.943 |
| Late binding | 48 | 39.304 | 10.613 | 1.200 | 0.997 |
| Payload | 0 | 16.279 | 16.900 | 16.736 | 17.759 |
| Payload | 48 | 54.764 | 26.497 | 16.865 | 17.777 |

On the long plain source, original reunion ranges from 9.392–9.615 ms, shortened Copy from 0.588–0.615 ms and shortened reunion from 0.461–0.479 ms. The large effect of elimination survives the observed variability. The residual benefit of reunion concerns the choices and consuming joins still executed after countdown removal. It does not demonstrate that a combined architecture is universally necessary.

The long payload contrast is less decisive in time: shortened Copy ranges from 16.568–18.748 ms and shortened reunion from 17.576–22.561 ms. Their ranges overlap. Its allocation result is exact and adverse to reunion, as shown below. Five samples do not support formal win/loss classification, even when ranges separate.

## The other qualified controls remain part of the comparison

This table shows all twelve controls on the same four-owner, four-query depth-48 sources. It prevents a comparison only against ordinary Copy from standing in for the qualified alternatives. Full per-cell ranges and phase summaries are in [the machine-readable summary](s04-reunion-complete/summary.json).

| Control | Plain median ms | Payload median ms |
|---|---:|---:|
| Copy | 35.427 | 54.764 |
| Reunion | 9.561 | 26.497 |
| Generic Scan | 17.486 | 33.243 |
| Generic Indexed | 18.568 | 34.523 |
| Specialized Scan | 11.369 | 27.815 |
| Specialized Indexed | 12.879 | 31.033 |
| Shortened Copy | 0.603 | 16.865 |
| Shortened reunion | 0.472 | 17.777 |
| Shortened specialized Scan | 0.893 | 18.646 |
| Shortened specialized Indexed | 1.203 | 18.971 |
| Permanent factoring | 9.671 | 37.285 |
| Generic Indexed with arena COW | 17.266 | 33.468 |

Specialization executes in the independently qualified paths, rather than merely receiving an eligibility label. Permanent factoring retains its current preparation-at-setup API; that cost is charged, not attributed to an intrinsic architectural requirement. These finite measured sources have distinct raw answers, so the complete-answer gate succeeds despite the factored engine's unique-answer publication contract. That does not make the contracts interchangeable for arbitrary sources.

## Requested allocation and peak demand give different answers

These are complete requested heap traffic in MiB and peak live growth in KiB, excluding the separate cancellation scenario. They are not RSS. Every pair reproduces exactly.

| Four-owner, four-query depth-48 source | Control | Traffic MiB | Peak KiB |
|---|---|---:|---:|
| Plain | Copy | 35.586 | 81.1 |
| Plain | Reunion | 9.935 | 50.8 |
| Plain | Shortened Copy | 0.805 | 42.6 |
| Plain | Shortened reunion | 0.637 | 46.7 |
| Payload | Copy | 43.016 | 1920.5 |
| Payload | Reunion | 18.296 | 1928.3 |
| Payload | Shortened Copy | 8.643 | 1920.6 |
| Payload | Shortened reunion | 9.407 | 1928.4 |

Even in the favorable shortened plain case, reunion's peak is higher than shortened Copy's. With payloads, its traffic is also higher. Complete retained answers are included; consumer ownership is part of these numbers. Sustained immediate-release or bounded-window consumers remain a separate investigation.

Short work also charges the checker. For two owners, one plain depth-zero query, Copy requests 40,048 bytes and shortened Copy 53,944 bytes; their median complete times are 28.8 and 37.8 microseconds. Reunion requests 40,898 bytes and shortened reunion 54,794 bytes, with medians 30.5 and 41.6 microseconds. There is no countdown to eliminate in that seed-zero query. This is a useful adverse control for the transformation's real preparation cost.

## Observation and cancellation are visible costs

First-answer latency is recorded within execution; it is not added again to total time. On the long plain source above, shortened Copy's first-query median is 119.4 microseconds and shortened reunion's is 38.5. The separate seed-zero input-to-first-answer-and-disposal scenario has sums of phase medians of approximately 133.0 and 51.7 microseconds, excluding shared preparation. Those phase-median sums describe the components; they are not medians of per-process sums.

Payload changes the balance. In the long payload case, shortened Copy and shortened reunion have about 6.963 and 6.777 ms of median aggregate execution/observation across four queries, while corresponding disposal phase medians sum to about 3.267 and 3.597 ms. Query construction/setup and transformation also count in the total. Looking only at execution would conceal obligations that matter to this contrast. Per-phase medians need not add to the median total.

All original queries remain caller-owned until their query ends. Temporary lowered queries are released immediately after backend setup. The checker transfers verified source into preparation and retains only query-checking metadata, so it does not keep an unnecessary second source AST. All engine, input and consumer owners return to the recorded baselines at their specified disposal boundaries.

## Evidence limits and necessary complexity

Reunion adds a checked private-phase boundary, saved local alternatives, fresh-identity/resource transport, Cartesian-product scheduling and resumed source execution. Shortening adds exact-source checking, per-query ownership/ground-counter validation and query transformation. Both still use the ordinary source rules for choices, consuming joins and outputs. The combined design must pay for both sets of responsibilities; the finite plain result shows a regime worth retaining, not permission to combine mechanisms without further evidence.

The source families and owner counts are supplied by the experiment. Broader source inference, repeated dynamic separation/reunion, unknown tails, competing effects and sustained lifetime are unmeasured here. A family checker rejecting a source is not evidence that the source cannot benefit from elimination or decomposition.

The harness measures a fresh prepared lifecycle after one complete warm lifecycle. Source-rule generation, independent oracle work and Rust compilation are outside its intervals. Execution and complete observation are measured jointly. Allocation diagnostics use separate binaries and their elapsed times are excluded from timing claims. The original/lowered inputs and retained answers impose explicit ownership assumptions. No complete architectural lifecycle superiority follows.

All 23 ordinary and 24 diagnostic package tests, strict all-target/all-feature Clippy and formatting pass. The runner's allocation phase precedes ordinary sampling and validates every measured path. Reference-interpreter and independent-evaluator implementations are unchanged. Four initial build receipts and exact runner/driver/registration copies are preserved under `build-preflight/`; they produced no comparative samples. The final binaries use their own directories and source freeze.

## Next investigation

[Return to integrated dependency repair](S04-reunion-next-investigation.md). T077 remains required for broader restoration, adaptive splitting, dynamic reunion, inference and lifetime. This seventh bounded T077 package supplies the selected complete-cost evidence; it does not complete S04 or the research goal.

Evidence: [registration](../registrations/S04-reunion-complete-pilot.md), [source/binary freeze](s04-reunion-complete/freeze.json), [driver audit](s04-reunion-complete/audit.json), [independent audit](s04-reunion-complete/independent-audit.json), [summary](s04-reunion-complete/summary.json), [run log](s04-reunion-complete/run.log), [ordinary tests](s04-reunion-complete/tests.log), [diagnostic tests](s04-reunion-complete/diagnostic-tests.log) and [Clippy](s04-reunion-complete/clippy.log). Individual ordinary and allocation receipts are retained in the same directory, named by repetition, family, owners, depth, reuse and mode.
