# Saved transitions do not repay this call-reuse implementation

Separated memoization remains slower than Direct across this pilot, despite avoiding hundreds of recursive transitions. In the deep inert four-query example, it takes **39.80 ms versus 1.08 ms** for Direct and **0.83 ms** for inferred execution. Recognition, replay and retained state need a different cost structure before this implementation becomes a serious competitor on these sources.

These are exploratory finite-session results, not a rejection of call reuse. Coarser recognition, stable operation identities, broader caller relevance, failure learning and sustained lifetimes remain unanswered. The [selection review](S05-inert-lifecycle-review.md) chooses integrated eligibility next while retaining those investigations.

## The comparison actually performed

The [prospective registration](../registrations/S05-inert-lifecycle.md) covers 288 scenarios and ten modes. Six source families cross depths 0/4/32/128, identical/distinct callers, one/four changing queries sharing preparation and releasing/window-one/retained-all consumers. Controls are Direct, owned whole-state AlphaLive, CompactLive, separated uncached/memo, Global Scan/Indexed, Global inferred Scan and Active Scan/Indexed. Existing baselines and source fixtures are reused.

**20,118 processes complete; 24 reproduce the known allocation failure; 18 are registered skips after repeated failure.** All 20,160 scheduled jobs have receipts. The campaign takes 528 seconds. There are five ordinary repeats and two separate metered repeats per successful cell. All **2,874 allocation pairs agree exactly** after baseline normalization. The [audit](s05-inert-lifecycle/audit.json) verifies frozen sources/binaries, complete phase structure, answer counts, mode-specific readings and final ownership restoration.

Every successful process checks complete answers against independent scalar semantics and Direct FIFO delivery outside measured intervals. Raw multiplicity is retained: two answers per query, except one for early failure. Candidate preflight warms a complete session; preparation is then recreated for measurement and reused across changing query identities. Retained outputs are checked after producer disposal. Ordinary and metered lifecycle tests also pass all six families, ten modes, three consumers and cancellation/full completion at depth four with four queries. Cancellation timing itself is not compared.

## The timing result, including its limits

Memoization's median paired time exceeds Direct by more than 10% in all 288 scenarios. **107 of those comparisons are clock-sensitive and cannot support a reliable gain/loss classification.** In the remaining **181**, median paired ratios range from **2.02× to 84.21×**; even the smallest individual paired ratio is 1.47×. Five repetitions provide exploratory sizing, not confidence intervals or a confirmed policy.

Memoization also exceeds the fastest observed nonmemo control by more than 10% in all 288 scenarios; 105 of those comparisons are clock-sensitive. That control is chosen from these same samples, so this is descriptive evidence, not an independently validated selection strategy. Every individual control remains in the [paired comparison artifact](s05-inert-lifecycle/comparisons.json). Some compiled controls lose individual comparisons to memoization; choosing only those controls would obscure the stronger Direct competitor.

The following medians are milliseconds per complete session at depth 128, four changing queries, distinct callers and retained-all consumers. Each total includes source/preparation, all queries, owned observations and disposal.

| Caller/source | Direct | CompactLive | Separated uncached | Separated memo | Global inferred |
|---|---:|---:|---:|---:|---:|
| Inert caller | 1.08 | 12.99 | 1.15 | 39.80 | 0.83 |
| Readable caller | 1.21 | 13.94 | 1.37 | 82.28 | 0.99 |
| Late-bound caller | 1.14 | 14.53 | 1.56 | 93.53 | 0.91 |
| Reader of another arity | 1.06 | 13.52 | 1.19 | 40.65 | 0.89 |
| Duplicate residuals | 15.02 | 166.21 | 19.64 | 120.50 | 15.57 |
| Early failure | 0.61 | 7.26 | 0.70 | 39.93 | 0.57 |

The inferred control is not universally fastest: the duplicate-residual case retains substantial output work. The table supplies matched examples, not workload prevalence or a whole-language ranking.

## Service and retained state both matter

In the inert memo example, median service through owned observation costs **34.40 ms**, and engine disposal costs **5.30 ms**. Direct's corresponding phases cost **0.988 ms** and **0.050 ms**. Preparation is about 0.016 ms for memoization. Individual phase medians need not sum to the median complete total.

**A disposal-only optimization cannot reverse the qualified pilot comparisons.** Subtracting every measured memo disposal interval, while leaving Direct unchanged, still gives a median paired ratio of at least **1.66×** across the 181 clock-qualified scenarios. This [arithmetic sensitivity bound](s05-inert-lifecycle/disposal-bound.json) is optimistic about disposal; it does not establish the cost of a replacement implementation or settle improvements inside service.

The [earlier allocation attribution](S05-key-allocation.md) identifies expensive key export/normalization and additional edge/residual transport. Current source inspection confirms that memoization constructs a normalized active-state key when interning each transition, retains keys and edges for the query, and clones cached edges during replay. The uncached path reuses node slots instead. This explains concrete responsibilities to investigate, but does **not** assign a measured CPU fraction to key construction. Dedicated CPU attribution or a sound changed recognition mechanism remains necessary before claiming a general limitation of reuse.

Allocation volume does not predict the timing order. In the inert example, memoization requests **17.71 MB**, less than CompactLive's **26.60 MB**, yet takes roughly three times as long. Direct requests **1.76 MB** and inferred execution **0.95 MB**. Memo's peak excess is **2.76 MB**, versus Direct's **71.3 kB**. In the duplicate-residual example, consumer-owned output alone retains **6.68 MB** in each of these modes. Consumer retention and engine history must therefore remain separate architectural obligations.

## The resource failures have a precise scope

All 24 failures are allocation aborts in owned whole-state mode on depth-128 duplicate residuals with distinct callers. They cover both query-reuse counts and all three consumer policies, twice in each build. Their full preflight cannot complete under the registered 1-GiB address-space limit; they supply no measured completed lifecycle or exact peak. Three subsequent ordinary repetitions per failed cell are explicitly skipped as registered.

All other cells complete, including CompactLive and separated execution on those same sources. This reproduces the representation-specific obstruction already investigated in the [deep-work study](S05-inert-depth.md). It does not show six separate consumer-caused failures, reject whole-state reuse abstractly, or establish an RSS requirement.

## What this contributes to architecture selection

The measured implementation adds normalized-state validity, a retained transition table, cached-edge transport and separately owned inert observations. Those responsibilities preserve source behavior, but their cost outweighs the tested saved execution. Direct remains a serious simple competitor; inferred execution is another strong control whose source analysis and preparation must be charged.

A credible reuse alternative must reduce a consequential responsibility—such as recognizing a larger operation, maintaining a sound cheaper identity, or avoiding repeated transport—not merely report fewer machine transitions. No result here establishes that those alternatives cannot pay. Nor can these component measurements select a complete architecture before interactions, language boundaries and sustained costs are measured.

## Measurement scope and receipts

Primary release builds have no engine/kernel metrics, allocation meter or stage profiler. Three calibrations each measure 10,000 empty intervals; the largest median is 15 ns. The registered signal rule flags totals below 100 times that median times the number of measured intervals. All ordinary timings use the ordinary allocator; requested heap is measured in separate processes. Neither requested bytes nor peak excess is RSS.

Source creation includes the fixture helper's construction and disposal of its unused query; preparation includes the rule clone, and query setup includes the query clone. Service and owned observation are inseparable in this runner. Validation and preflight are outside intervals but can affect caches; record storage and oracle storage are also excluded. The totals exclude process startup, user-program compilation/artifacts and sustained service. Caches are query-owned, not shared across changing queries.

The [frozen manifest](s05-inert-lifecycle/freeze.json), [source archive](s05-inert-lifecycle/sources.zip), [ordered jobs](s05-inert-lifecycle/jobs.json), [raw receipts](s05-inert-lifecycle/runs.jsonl.gz), [audit](s05-inert-lifecycle/audit.json) and [comparison script](../../../research/chr-reuse/experiments/summarize_inert_lifecycle.py) support reconstruction. The allocation evidence is newly measured over complete sessions; it does not borrow a shorter diagnostic interval as a lifecycle total. Reference-interpreter code remains unchanged.
