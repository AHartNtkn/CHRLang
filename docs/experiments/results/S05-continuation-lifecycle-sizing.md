# Continuation reuse saves executions but exposes large key and retention costs

**The sizing pilot shows that fewer executed steps are insufficient to choose continuation tables.** Owned keys and retained transitions add substantial allocation and memory, even when renaming recognizes shared futures. The history source also exposes a missing favorable condition: its supposedly temporary occurrences remain live during the common work.

The [prospective pilot](../registrations/S05-continuation-lifecycle-sizing.md) completes 2,044 processes with 672 exact allocation replays. Independent complete raw answers, cancellation and query/prepared ownership checks pass for all seven engines. Timing samples are exploratory; no practical gain/loss or statistical ranking follows from them.

## Measurement responsibilities are explicit

The continuation executor now publishes one owned answer per derivation. It does not perform unique-answer presentation or retain a presentation set. Consumers that need uniqueness own that work. Raw-multiset tests replace checks that could pass while multiplicity was hidden. All measured competitors use the complete raw-answer contract.

Prepared continuation rules reuse the existing `PreparedMachine` across changing queries. Each query owns its frontier, canonical keys and cached transitions. Tests establish that a new query does not reuse old answers or identities, and allocator checks establish full query and prepared disposal. Diagnostic updates follow the existing metrics feature; counter-free tests verify zero wrapper and source counters while preserving raw answers.

Primary cost includes preparation, setup, execution with complete observation, engine/answer disposal and prepared disposal. Source/input construction and first observation remain separately recorded. Meter elapsed times are excluded from speed claims; requested allocation is not RSS. Native compilation is excluded.

## Representative complete costs

The table shows depth64, four changed queries, consuming finish and forward starting order. Each time is a single exploratory sample. Allocation measurements replay exactly.

| Family | Mode | Primary ms | Requested primary bytes | Peak requested growth |
|---|---|---:|---:|---:|
| exact | direct | 0.824 | 1,678,056 | 44,588 |
| exact | exact | 3.899 | 4,054,852 | 671,767 |
| exact | alpha | 4.918 | 5,726,820 | 674,374 |
| exact | live | 5.640 | 4,439,764 | 634,622 |
| exact | scan | 1.037 | 1,383,941 | 102,398 |
| exact | sealed | 0.651 | 983,481 | 86,248 |
| exact | dependencies | 3.192 | 2,463,909 | 167,487 |
| rename | direct | 0.789 | 1,678,284 | 44,702 |
| rename | exact | 20.147 | 15,164,448 | 2,513,194 |
| rename | alpha | 4.961 | 5,726,820 | 674,374 |
| rename | live | 5.242 | 4,439,764 | 634,622 |
| rename | scan | 1.104 | 1,436,405 | 102,430 |
| rename | sealed | 0.757 | 1,035,753 | 86,248 |
| rename | dependencies | 3.434 | 2,463,909 | 167,487 |
| history | direct | 0.876 | 1,863,236 | 46,872 |
| history | exact | 21.284 | 16,713,508 | 2,739,784 |
| history | alpha | 25.335 | 23,867,652 | 2,741,199 |
| history | live | 26.394 | 17,825,428 | 2,515,531 |
| history | scan | 1.265 | 1,634,755 | 105,142 |
| history | sealed | 0.692 | 1,220,183 | 87,560 |
| history | dependencies | 3.383 | 2,506,215 | 170,982 |
| distinct | direct | 0.788 | 1,676,812 | 44,750 |
| distinct | exact | 19.344 | 15,171,808 | 2,515,454 |
| distinct | alpha | 22.611 | 21,804,896 | 2,517,865 |
| distinct | live | 25.242 | 16,646,944 | 2,365,641 |
| distinct | scan | 1.068 | 1,372,437 | 103,718 |
| distinct | sealed | 0.627 | 972,041 | 87,568 |
| distinct | dependencies | 3.282 | 2,463,925 | 167,531 |

In the renamed-future case, Alpha requests about 5.73 MB versus exact-ID tables' 15.16 MB, but direct execution requests 1.68 MB and specialization 1.04 MB. Alpha retains roughly 674 KB peak requested growth versus direct execution's 45 KB. Recognition provides useful reuse but has costs beyond the steps it saves.

The separate metrics build confirms the mechanism on the actual depth64/resource source: Direct and ExactIds execute 543 steps; Alpha and AlphaLive execute 140 and replay 403 transitions. These are one-query work counts, not counts inferred from the earlier source witness or a combined cost score.

## Why the history family barely reuses common work

The key trace shows live `trash` occurrences while the countdown remains pending. Source rule order gives recursive work priority over consuming these occurrences. The branches therefore have different live stores throughout the substantial common computation. AlphaLive correctly retains those differences; projecting dead history does not authorize ignoring live resources.

On the actual resource source, Direct, ExactIds and Alpha execute 565 steps. AlphaLive executes 550, with 15 replay hits, mostly after the substantial work. The earlier small history witness had a different source and scheduling opportunity. Its favorable work counts cannot characterize this pilot.

Keep the current history source as an adverse control. Add a prospectively specified source with temporary consumption ordered before the common computation, validate full source correspondence and inspect actual keys/hits before timing. This is a scheduling/source-property contrast, not grounds to weaken key validity or silently ignore live occurrences.

## Evidence and next action

The [freeze](s05-continuation-lifecycle-sizing/freeze.json), [complete results](s05-continuation-lifecycle-sizing/summary.json), [work counts](s05-continuation-lifecycle-sizing/work.log), [key trace](s05-continuation-lifecycle-sizing/key-probe.log) and [audit](s05-continuation-lifecycle-sizing/audit.json) preserve provenance. The source runner checks 336 complete mode/source combinations plus cancellation. All 47 reuse package tests, eight counter-free continuation tests and strict Clippy pass. Reference-interpreter code is unchanged.

Before confirmation, establish the favorable dead-history scheduling control and attribute key costs. `State::key` exports pending terms, the live store and outputs; canonicalization then traverses owned terms and history. Long constructor spines and accumulated history can be revisited repeatedly, while cached transitions retain old keys. This is an identified implementation-cost hypothesis, not a measured asymptotic law or proof that recognition must be expensive. A credible alternative should avoid unnecessary export/retention where it can preserve the same equivalence and source contract.

Do not run a larger confirmation merely to certify these exploratory losses. First determine whether a stable key representation or a more appropriate reuse boundary can materially change the comparison, preserving exact-ID, direct, specialized and graph controls. Call-level reuse and broader relevance remain distinct obligations. Sustained S08 remains the strongest ready alternative at the next selection review; broader S02 integration remains required. T075 and the architecture goal remain active.
