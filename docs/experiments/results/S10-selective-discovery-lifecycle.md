# Filtered discovery improves conditional costs, but stronger controls still matter

Filtering substantially reduces conditional allocation on recursive and branch-specific sources. It adds allocation on dense heads and after counting eliminates common traversal. The improvement does not remove the large remaining gap to Scan on branch-specific and dense sources, and it does not establish a universal ordering of traffic, peak and time.

The [registered pilot](../registrations/S10-selective-discovery-lifecycle.md) completed 2100 isolated processes: 600 allocation runs forming 300 exact replay pairs, then 1500 ordinary-allocator timings. All complete observations, optimizer admission counts, build identities, phase order and owner restoration checks pass. [Independent audit](s10-selective-lifecycle/audit.log), [all cells](s10-selective-lifecycle/summary.csv), [frozen sources/binaries](s10-selective-lifecycle/freeze.json), [toolchain](s10-selective-lifecycle/toolchain.txt), [hardware](s10-selective-lifecycle/hardware.txt).

## Complete measured allocation

Except the final row, traversal rows use three choices, depth sixteen and four changed queries with permits. Dense uses six occurrences and four changed queries, producing all thirty ordered pair observations. The shallow row uses no choices, depth zero and one query. Figures include inference where enabled, preparation, inputs, setup, complete delivery, one-service cancellation and disposal. Traffic is requested allocation bytes; peak is requested live heap above the initial baseline, not RSS.

| Source / transformation | Existing conditional traffic | Filtered traffic | Scan traffic | Existing peak | Filtered peak | Scan peak |
|---|---:|---:|---:|---:|---:|---:|
| Common / original | 2546543 | 1731073 | 1240946 | 54964 | 43732 | 93976 |
| Independent suffixes / original | 21994808 | 14436750 | 1354296 | 323486 | 209558 | 97442 |
| Early failure / original | 2897430 | 2743976 | 686656 | 52303 | 51135 | 50519 |
| Common / counted | 465235 | 465457 | 403610 | 24385 | 24545 | 54017 |
| Independent suffixes / counted | 2505803 | 2144033 | 518179 | 53328 | 51136 | 63660 |
| Dense / original | 5141246 | 5148676 | 297049 | 24165 | 24485 | 17303 |
| Shallow common / original | 55207 | 54532 | 24283 | 11907 | 12067 | 8938 |

The common original source illustrates a real tradeoff: filtered conditional uses more traffic than Scan but less peak memory. After counting, both conditional modes again retain less peak than Scan, with higher traffic. In contrast, uncounted independent work has substantially more traffic and higher peak than Scan even after filtering. No workload weighting is assigned to these regimes.

Counting and filtering are not additive savings. Counting removes common execution and most of the opportunity for this filter; the counted common cell adds 222 traffic bytes and 160 peak bytes. Independent suffixes retain a useful opportunity, with filtered traffic dropping from 2.51 MB to 2.14 MB. Scan with the same counting transformation remains at 0.52 MB in that cell.

## Where costs change

On original independent work, aggregate first-delivery allocation falls from 21039220 to 13992072 bytes, and remaining-delivery allocation from 835244 to 313468. Preparation is unchanged at 11376. Main query setup increases from 75836 to 85436, and cancellation setup from 18959 to 21359. The full total charges these increases; the result does not hide filtered discovery state outside execution.

The dense control has the same thirty candidate tuples in both modes. Main setup rises by 1920 bytes and first-delivery work by 5136; the complete total rises by 7430. Most of its allocation still occurs after setup, exceeding five million bytes in either conditional mode. Small list-construction changes alone cannot account for the difference from Scan's 297049 total. Broader matching, history and supported-execution costs remain a question, not an intrinsic rejection of conditional representation.

The early-failure source had slightly more service ticks after filtering in the [work gate](S10-selective-discovery-gate.md), but its complete allocation decreases here. Conversely, the shallow source has lower traffic with higher peak. These observations demonstrate why candidate counts and service ticks cannot select the architecture by themselves.

## Timing and evidence limits

Five ordinary timing samples per cell are exploratory. Original independent work has existing median 17248757 ns (15704777–18072683) and filtered median 12300647 (11330196–12616761). Scan's median is 1335627 (1178789–2068963). This sampled regime supports investigating the improved implementation as a stronger conditional control, while preserving the much cheaper Scan result.

Other ranges overlap substantially. Counted common has existing median 353183 ns (351092–363975), filtered 599019 (356561–660649) and Scan 504923 (255212–635240). These samples do not support a precise ranking of small differences or a formal practical-win classification. No subtraction of timer overhead or weighted aggregation is performed.

The matrix also includes alternating missing-permit queries. Counting declines those queries, and execution preserves their full suspended terms, fuel, unknowns and marker absence. Dense sources have no counting certificate and are tested only in original form. Every warm and measured finite observation is checked against independently constructed expectations; the independent scalar agrees outside intervals.

Every query and cancellation restores the prepared baseline; preparation and inference disposal restore their respective prior baselines. The runner includes filtered-list construction, retained candidate state and actual disposal. It retains the original input alongside a transformed query where applicable. Native compilation, process startup, original source AST construction, expected/scalar validation and preallocated bookkeeping are excluded. Delivery couples execution and observation. Sustained retention and large source/linking regimes remain unmeasured.

The prior 120-test gate in three configurations remains the semantic basis for the engine change. Strict Clippy passes for the [allocation runner](s10-selective-validation/clippy-meter.log) and [ordinary runner](s10-selective-validation/clippy-time.log). [Runner](../../../research/chr-direct-conditional/examples/selective_cost.rs), [dense/source expectations](../../../research/chr-direct-conditional/examples/support/selective_source.rs), [driver](../../../research/chr-direct-conditional/experiments/selective_lifecycle.py), [auditor](../../../research/chr-direct-conditional/experiments/audit_selective_lifecycle.py).

## Next investigation

Select [resumable contextual discovery](S10-resumable-contextual-entry.md) within T078. The filtering pilot now supplies a stronger conditional control and identifies both favorable and adverse regimes. Another small filter refinement has less immediate decision value than testing whether retained matching progress can avoid the repeated history work in contextual demand discovery without storing every candidate.

The strongest alternative is support-aware conditional joining or more precise discovery retention. It remains required: individually live occurrences can still have incompatible supports, and repeated-variable matching, guards and history are not handled by this filter. The present results do not resolve those mechanisms. Reconsider them after the first resumable source/work gate.

No default changes in this package. The broad coherent-architecture comparison, language properties, sustained lifetime, connected parallel work and held-out challenges remain required. This bounded cost pilot does not finish T078 or the goal.
