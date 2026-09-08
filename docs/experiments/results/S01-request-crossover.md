# Retention pays for stable reuse and loses sharply under dense consumption

The focused comparison finds a practical retention benefit at 32 rows and 64 stable selective requests. Dense consumption instead makes retention substantially more expensive, with the loss growing at the larger row count. This supports an explicit tradeoff between retained eligibility and direct indexed discovery, not one universal matching organization.

All 288 processes completed with correct full answers, including ten timing repetitions per cell. All 24 allocation cells replay exactly. The comparison includes fresh preparation, query setup, execution, observation and disposal; engine code is unchanged from the validated script-ownership correction.

## The measured crossover

Ratios divide Retained by Direct total lifecycle within the same repetition. Values below one favor retention. Each cell is one complete query, so request counts are not hidden inside an averaged session mixture.

| Family | Rows per side | Requests | Paired median ratio | Ten-repetition range |
|---|---:|---:|---:|---:|
| Stable selective | 8 | 1 | 1.048 | 0.817–1.191 |
| Stable selective | 8 | 8 | 0.846 | 0.754–0.939 |
| Stable selective | 8 | 64 | 0.804 | 0.707–0.904 |
| Stable selective | 32 | 1 | 1.181 | 1.102–1.480 |
| Stable selective | 32 | 8 | 0.905 | 0.854–0.936 |
| Stable selective | 32 | 64 | 0.779 | 0.736–0.827 |
| Dense consumption | 8 | 1 | 1.414 | 1.219–6.721 |
| Dense consumption | 8 | 8 | 1.505 | 1.407–1.602 |
| Dense consumption | 8 | 64 | 1.473 | 1.418–1.541 |
| Dense consumption | 32 | 1 | 4.115 | 2.677–4.451 |
| Dense consumption | 32 | 8 | 4.914 | 4.721–7.800 |
| Dense consumption | 32 | 64 | 5.619 | 5.397–6.710 |

The stable N=32/R=64 cell clears the registered 20% practical threshold with every repetition in the same direction. N=8/R=64 is close but does not clear that threshold; it must not be rounded into a stronger claim. Stable N=32 changes direction between one and eight requests, and reaches the practical-benefit threshold somewhere between the measured eight- and 64-request points. The exact crossing point is not established.

Every dense-consuming cell clears the practical threshold against retention. Some timing outliers widen the ranges, especially at one request, but none changes that direction. Each request consumes N right resources and replenishes them before the next request; no resource is silently reused.

## What pays for the difference

At N=32/R=64, stable Direct has a median lifecycle of 2.422 ms and Retained 1.934 ms. Retained setup costs about 0.078 ms versus 0.057 ms, but execution falls from 0.982 to 0.436 ms. Observation is nearly unchanged, about 0.62 ms. Retention repays its setup through repeated eligible-pair access, rather than by omitting output work. Requested traffic is 1.776 MiB versus Direct's 1.967 MiB.

Dense consumption at that size reverses the tradeoff: Direct takes 4.692 ms and Retained 26.260 ms. Retained execution alone is about 23.235 ms versus 2.179 ms. Requested traffic is 10.387 versus 4.304 MiB. The retained relation contains many pairs per right resource, and all those incident pairs must be invalidated when that resource is consumed. Replenishment pays their construction again.

The larger request count does not make all retention beneficial: it amplifies whichever work the source repeats. Stable reuse repeats access to a maintained relation; consumption repeats construction and invalidation. This mechanism distinction is more useful than assuming retention will eventually amortize under any workload.

## Scope of the supported choice

For these checked sources, retention is justified in the measured stable high-reuse regime, while direct composite indexing is favored under dense consumption. This is not an instruction to build automatic hybrid routing, require a source declaration, or choose a mandatory runtime. Such choices must account for eligibility, routing and shared ownership costs in S07/S10.

The test uses two row counts and three request counts; it does not establish a universal switching formula or a workload distribution. Larger tables, different output consumers, partial joins, subscriptions, generated access and more complex binding updates remain separate questions. A finer estimate of the crossing point would not change the present bounded conclusion that both favorable and adverse retention regimes exist. Held-out validation and a concrete routing decision may require that precision later.

T065's bounded selective/consuming comparison is complete. Broader S01 obligations remain open. The next investigation is [S04 state restoration and replay](S04-restoration-entry.md), which can change ownership across search architectures and has not received the renewed sequence's direct trial.

## Evidence

The [registration](../registrations/S01-request-crossover.md), [freeze](s01-request-crossover/freeze.json), [raw processes](s01-request-crossover/raw.jsonl), [cell and phase results](s01-request-crossover/summary.csv), [paired ratios](s01-request-crossover/summary.json) and [runner](../../../research/chr-compiled/examples/s01_request_crossover.rs) preserve exact inputs and bounds. Full mathematical observations validate every size; six scalar-source checks validate the oracle on smaller endpoints. Counter-free release timing uses the ordinary allocator; allocation runs use a separate meter. Requested bytes are not RSS. Native compilation, process startup, input construction/cloning and validation are outside the measured lifecycle.
