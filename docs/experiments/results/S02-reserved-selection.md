# Reserving one token removes duplicated discovery, but the encoded protocol remains costly

Reserving one token per selection round removes the multiplicative eligibility work caused by identical tokens. On six flat requests, source applications fall from **212 to 78**, and complete Scan time falls from **44.06 ms to 13.90 ms**. The local competitor takes **0.051 ms**: this repairs a real cost without establishing that the encoded organization earns its machinery.

The result is conditional. Small cases can pay more for the extra round opening, and most tested heap peaks increase despite lower allocation traffic. These are exploratory finite-session comparisons, not a rejection of integration or a complete architecture choice.

## The changed responsibility

The [original protocol](S02-stable-selection.md) discovers each ready request against every remaining token. The new rule opens a round only when a ready request and a token exist. It reserves one token and posts a single round marker. Candidate discovery uses that marker without a token head; the existing stable-ticket pruning selects the earliest eligible request. Commit consumes that request and renews the epoch, spending the reserved token.

The opening witness does not select the winning request. All eligible requests still enter the existing priority comparison. A younger request can therefore make an older suspended request ready for the next round. Opening requires a ready witness so a suspended store cannot be stranded with a reserved token and unfinished round.

This removes the **token-multiplicity factor**, not all repeated discovery. Six flat requests still generate 6+5+4+3+2+1 = 21 candidates across rounds. Persistent eligibility and alternative precedence structures remain distinct experiments. Reservation also depends on this fragment's exclusive token consumer and ordered protocol: other source consumers, general bodies and interleaving require their own correspondence tests.

## Independent source work explains both benefit and overhead

The table gives applications for one query. Both a/b values and both frozen trace repetitions agree. The independent scalar evaluator executes the encoded rules, checks decoded answers against ordinary source semantics, and its application totals match Scan, Indexed and inferred diagnostic execution for all corresponding queries.

| Source | Size | Original applications | Reserved applications | Original → reserved eligibility discoveries |
|---|---:|---:|---:|---:|
| Flat ready requests | 2 | 20 | 18 | 5 → 3 |
| Flat ready requests | 6 | 212 | 78 | 91 → 21 |
| Sparse readiness | 2 | 9 | 8 | 2 → 1 |
| Sparse readiness | 6 | 17 | 8 | 6 → 1 |
| Equality-enabled chain | 2 | 18 | 18 | 3 → 2 |
| Equality-enabled chain | 6 | 132 | 108 | 21 → 6 |
| Competing requests | 2 | 10 | 11 | 2 → 2 |
| Competing requests | 6 | 82 | 51 | 32 → 15 |

Flat size six absorbs **70 duplicate candidates originally and zero after reservation**, while adding six round openings. In the size-two competition case there is only one token: no duplicate discovery is avoided, and opening adds one application. The chain retains equality, ancestry and repair work, so its improvement is smaller than the flat case's.

## Full finite-session costs retain strong competitors

The [prospective registration](../registrations/S02-reserved-selection.md) covers four families, sizes 2/6, immediate/all consumers, complete/alternating-cancel sessions and eleven controls. Each session prepares once for four changing a/b queries. All **2,112 lifecycle processes finish**: 704 metered, 176 work, 176 ordinary cancellation replays, 176 excluded warmups and 880 primary timings. There are **352 exact allocation pairs**, **88 exact work pairs** and five primary repetitions of each of 176 complete cells.

All 256 original-control configurations also reproduce their prior per-phase allocation and peak measurements exactly in the [control comparison](s02-reserved-selection/control-bridge.json). The changed protocol is therefore compared against the same measured allocation controls.

The following medians are milliseconds for size six, four complete queries and retained-all consumers. They include construction, preparation, encoding, setup, execution, owned observation, consumers and disposal.

| Source | Local | Ordinary inferred | Original CHR Scan | Reserved Scan | Original CHR Indexed | Reserved Indexed |
|---|---:|---:|---:|---:|---:|---:|
| Flat | 0.051 | 0.070 | 44.06 | 13.90 | 54.68 | 15.70 |
| Sparse | 0.044* | 0.071 | 1.67 | 1.00 | 2.54 | 1.38 |
| Chain | 0.054 | 0.089 | 37.27 | 26.68 | 24.31 | 20.61 |
| Competition | 0.051* | 0.067 | 15.83 | 9.36 | 20.02 | 11.62 |

*The marked local cells are clock-sensitive; their medians are descriptive, not reliable gain/loss classifications. All 48 reserved-versus-corresponding-original comparisons clear the registered signal rule. Twenty-six median paired ratios are below 0.9; three are above 1.1, all in size-two competition. Nineteen remain between those thresholds. Five repetitions do not establish confidence intervals or justify a frozen policy.*

The stronger competitors change the interpretation. For the 24 signal-qualified comparisons against ordinary inferred execution, reserved median paired ratios range from **12.68× to 329.20×**. Other comparisons, including signal-limited ones, remain in the [audit](s02-reserved-selection/audit.json). Counts are unweighted experimental coverage, not workload prevalence.

## Less traffic does not guarantee less retained state

Across the 48 complete reserved configurations, requested bytes fall in **36** and rise in **12** relative to the corresponding original protocol. The increases are size-two chain and competition cases. Peak excess falls in **18** and rises in **30**. The extra rule and round state have overhead even where discovery is cheaper.

In the flat size-six retained-all Indexed session, requested allocation falls from **31.04 MB to 9.56 MB**, while peak excess falls from **133.4 kB to 118.2 kB**. Ordinary inferred execution requests **57.45 kB** and peaks at **9.90 kB**. Sparse Indexed traffic falls from **1.81 MB to 0.98 MB**, but peak excess rises slightly, from **103,489 to 103,601 bytes**. Requested traffic, live requested heap and RSS are different quantities; this experiment measures the first two.

Even eliminating every reserved execution allocation leaves **4.70×–8.73×** the lowest-allocation ordinary/local control's complete total, across all 48 scenarios. This [optimistic arithmetic bound](s02-reserved-selection/bounds.json) does not charge a replacement executor. It shows that an execution-only allocation repair cannot reverse these comparisons. It does not settle runtime, larger preparation reuse or a representation that also changes encoding, setup and observation.

The encoded organization still owns constructor/representative facts, repair rules, ancestry, source tickets, precedence, epochs/rounds, candidate pruning, host discovery/history and source translation. Local and ordinary controls implement the source without this full protocol. Further architectural investigation must distinguish which responsibilities are necessary from which follow from this encoding; fewer source applications alone cannot answer that question.

## Correctness, failure and ownership

The reserved source gate passes **784 compiled Scan/Indexed cases**, with independent ordinary scalar, encoded scalar and local agreement. The cases cover all request orders, descriptor order, repair, partial readiness, token counts and later activation of an older request. All **29 containing constructor tests** pass.

An explicit test cancels after the round marker is present under Scan, Indexed and inferred Scan, then reuses preparation for a changed query and checks retained answers after producer disposal. A mutation that opens without a readiness witness strands a round and is detected. The unchanged decoder rejects unfinished protocol state; there is no special successful decoding path for a stranded reservation.

Additional post-campaign checks exercise constructor clashes, a later consuming clash and a hidden occurs cycle after commit. Both protocols fail as the independent ordinary and encoded scalar evaluators require, then reuse preparation successfully. These give 18 compiled failures and 18 subsequent successful queries across Scan, Indexed and inferred Scan. Their [test source receipt](s02-reserved-selection/failure-test-source.json) is separate from the frozen cost campaign. No implementation change follows that campaign.

Every metered lifecycle run restores its final requested-live heap; consecutive measured heap phases agree. Retained answers are checked after engine, input, preparation and source disposal. Alternating cancellation in the cost harness occurs after one adapter call, so it is an ownership check rather than matched semantic progress. Sustained lifetimes and other source consumers remain unqualified.

## Evidence and next decision

Primary builds have no allocation or work counters. Meter/work clocks are not used as primary evidence. Three calibrations each contain 10,000 empty intervals; the largest wall median is 13 ns. The signal rule is 100 times that median times 37 measured phases. Independent oracle storage, validation, recording buffers, process startup, compilation/artifacts and sustained service are outside totals. CPU clock reads outside wall intervals are instrumentation, not charged source work. External warmups use separate processes and do not warm the measured process's allocator.

The [frozen source/binary manifest](s02-reserved-selection/freeze.json), [archive](s02-reserved-selection/sources.zip), [job order](s02-reserved-selection/jobs.json), [raw receipts](s02-reserved-selection/runs.jsonl.gz), [auditor](../../../research/chr-relational/experiments/audit_reserved_selection.py) and [bound reconstruction](../../../research/chr-relational/experiments/summarize_reserved_selection.py) preserve the result. Strict Clippy and the source tests pass; the independent reference interpreter is unchanged.

The [selection review](S02-reserved-selection-review.md) chooses demand discovery/copy repair next. Cheaper precedence/ancestry, persistent eligibility, different integrated representations and broader source contracts remain required. This is package two after the full call-key allocation review; the research remains active.
