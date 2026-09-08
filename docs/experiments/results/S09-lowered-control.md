# Serial contraction changes the resource tradeoff, but factoring still matters

The existing contraction substantially improves the specialized serial path, yet it does not uniformly beat four factored workers. It uses less process CPU in all ten comparisons and fewer requested allocation bytes, while its unpartitioned search has a substantially higher peak on multi-region sources. This comparison exposes a remaining architectural interaction between lowering, factoring and branch ownership.

## Registered evidence

The [prospective pilot](../registrations/S09-lowered-control-pilot.md) compares persistent factored inline, four workers at quantum128, unpartitioned Specialized, and Specialized with carrier contraction. It reuses the balanced/skewed depths64/256 and tiny source, with one/sixteen changing queries. All organizations validate the same complete products and raw multiplicity.

All440 processes complete:40 discarded warmups,280 measured timing processes and120 separate allocation processes. Every allocation run restores its baseline. The [raw receipts](s09-lowered-control/), [source/binary freeze](s09-lowered-control/freeze.json), [analysis](s09-lowered-control/analyze.py) and [full paired summary](s09-lowered-control/summary.json) retain the evidence. Counter-free release builds and Clippy pass. No process reaches the60-second or1GiB address-space bound.

| Lifecycle comparison | Practical benefits | Practical losses | Unresolved |
|---|---:|---:|---:|
| Contracted serial versus four workers | 2 | 2 | 6 |
| Specialized serial versus four workers | 2 | 8 | 0 |
| Contracted serial versus persistent inline | 1 | 1 | 8 |
| Contracted versus Specialized serial | 8 | 0 | 2 |

These use the registered paired-ratio screen, not confidence intervals or workload weights. The two contracted-versus-worker benefits are tiny queries; the two losses are the largest skewed source, both cold and reused. The other six comparisons do not establish an ordering at the specified precision.

## Representative resource ratios

The following are median paired ratios for **contracted serial divided by four workers**. Values below1 favor contracted serial for that resource. Peak ratios use the three independent allocation observations; concurrent peaks are descriptive rather than forced to replay.

| Source and query count | Lifecycle wall | Process CPU | Requested bytes | Incremental peak |
|---|---:|---:|---:|---:|
| Balanced depth64, sixteen queries | 0.831 | 0.469 | 0.638 | 1.985 |
| Balanced depth256, sixteen queries | 1.071 | 0.435 | 0.517 | 2.311 |
| Skewed depth256, one query | 1.497 | 0.741 | 0.750 | 4.463 |
| Skewed depth256, sixteen queries | 1.255 | 0.609 | 0.749 | 4.438 |
| Tiny, sixteen queries | 0.176 | 0.501 | 0.494 | 0.405 |

Contraction earns practical CPU benefits against four workers in all ten cells. Both serial variants have exact requested-traffic replay in their allocation triples. Lower cumulative allocation and higher simultaneous retention are compatible: the metrics describe different costs.

Preparation includes source analysis, specialization, contraction and disposal of intermediate prepared objects. The serial executor drops terminal branch engines during execution; the worker organization retains regional answer caches until query closure/disposal. Both ownership costs participate in lifecycle totals, but their phase labels cannot be interpreted as identical internal operations. Process CPU also includes validation/reporting, as prospectively declared.

## What the comparison can and cannot attribute

**Contraction is useful on these sources.** Its eight practical benefits over otherwise specialized serial execution directly challenge the idea that improving dispatch alone represents the strongest available serial path. The certificate's accepted source and known-prefix behavior remain part of the result; no language-wide restriction is selected.

**The worker opportunity survives this particular whole-organization control in two large skewed cells.** It remains a CPU/wall-time tradeoff there. The result does not establish that a serial engine combining contraction with useful independence analysis would lose.

**Branch organization is a material uncontrolled distinction in that broader claim.** Source inspection shows the compiled SearchEngine creates complete branch engines through `Engine::fork_clone` at each OR. The contracted control starts one whole query. The persistent inline/worker controls first use the existing independence certificate and combine regional answers. They therefore differ in which independent state is copied and which source work may be revisited after another region branches. Their prepared representations differ as well.

This identifies a plausible explanation for the high unpartitioned peak, not an isolated byte attribution. The measured peak alone cannot say how much comes from branch copies, immutable arena ownership or other representation costs. Do not infer an intrinsic memory penalty for contraction from this comparison.

## Next discriminating investigation

T069 remains active for contraction with certified factoring, plus an ownership control where necessary to explain remaining cost. Reuse the existing conservative certificate and complete-product semantics. The candidate must handle aliases, multiplicity, cancellation and finite publication correctly; it must not exploit the closed-form answers of this source as a hand-written shortcut. Compare complete source-to-answer lifecycle, including certification and product construction.

This is more valuable than repeating the six overlapping timing cells: it can change the organization being compared and test whether an available combination removes the remaining apparent parallel opportunity. The current whole-organization comparison remains valid within its scope. A factored serial win would not reject parallel work that remains substantial after lowering; a worker win would strengthen that bounded opportunity.

Generated multihead access remains the strongest distinct ready investigation and follows the bounded resolution. Connected-work parallelism, larger rulesets/output products, language properties, complete architectural composition and held-out challenges remain required by the governing sequence. No architecture goal is closed by this pilot.
