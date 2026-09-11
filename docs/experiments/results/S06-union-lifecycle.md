# Union saves output work, but preparation outweighs it at the tested reuse

At one or eight queries, union requests more bytes and reaches a higher peak than simplified or explicit execution in all 96 matched allocation scenarios. The overlapping full-output source nevertheless saves enough execution allocation to make higher reuse consequential. This pilot does not reject compact union or establish a runtime winner.

The [registration](../registrations/S06-union-lifecycle.md) fixes six modes, four source families, two widths, two reuse levels, membership/full-output endpoints and three consumer policies. All 1,152 allocation processes and 960 exploratory timing processes complete with independently checked answers. Allocation repeats agree exactly in all 576 cells.

## Preparation reverses the attractive component result

For eight-coordinate overlap, eight full-output queries and retained-all consumers:

| Mode | Requested bytes | Peak excess bytes | Sampled median total time |
|---|---:|---:|---:|
| Union | 1,016,448 | 514,288 | 1.821 ms |
| Separate diagrams | 1,110,592 | 609,848 | 1.936 ms |
| Reduced disjunction | 861,320 | 460,904 | 1.643 ms |
| Explicit | 851,048 | 460,072 | 1.641 ms |
| Deduplicated explicit | 855,528 | 464,552 | 1.665 ms |
| Prepared names | 52,531,492 | 493,372 | 25.456 ms |

**Union reduces the measured output allocation but adds substantial preparation.** It requests 489,480 bytes during execution/output versus explicit execution's 848,232. Its preparation requests 525,240 bytes versus 1,088. Saving about 359 KB later does not repay about 524 KB more up front at this reuse level.

**A higher-reuse crossover is plausible and testable.** Full-output requests repeat a five-caller cycle. Its measured execution traffic is 251,920 bytes for union and 433,984 for deduplicated explicit execution. If those per-cycle costs remain unchanged, three cycles plus preparation/source construction give 1,282,728 versus 1,304,768 requested bytes. That predicts a small traffic reversal at 15 queries. It is an analytical extrapolation under repeated-query and container assumptions, not a measured crossover, memory advantage or timing result. The next experiment must verify it and test a longer-reuse point.

**Membership gives simpler controls a much stronger position.** On the same source with eight membership requests, explicit and deduplicated explicit execution request 3,328 bytes in total and allocate nothing during the membership checks. Union also allocates nothing during those checks, but its total is 527,480 bytes. There is no execution-allocation saving to amortize here. A time crossover would require measured lookup savings over enough requests; no such result follows from this pilot.

## Adverse controls remain consequential

**Source deduplication handles redundant alternatives cheaply.** At width eight/eight full-output queries/all retention, union requests 589,168 bytes versus deduplicated explicit execution's 98,160. Simplification requests 108,176. Repeatedly compiling identical operands and importing their graphs is an avoidable construction choice, not an intrinsic obligation of union. A future repair must preserve the simple deduplicated competitor.

**Disjoint and single-branch cases charge union without an overlap benefit.** For disjoint alternatives at the same size/reuse, union requests 162,368 bytes versus deduplicated explicit execution's 82,488; peaks are 89,144 versus 80,584. For a single branch, union requests 140,720 bytes versus 96,032. These remain part of any higher-reuse or construction-repair comparison.

**Prepared name solving's full-output cost belongs to this adapter and API.** It repeatedly constructs partial caller maps and asks the existing finite-feasibility solver during traversal. That can reject branches early, but creates substantial traffic. This is not evidence against a direct name-solver enumerator or cheaper caller transport. The other strong explicit/reduced controls prevent this costly path from being the sole comparator.

Across all 96 matched allocation scenarios, union exceeds reduced, explicit and deduplicated explicit execution in both traffic and peak. Against separate diagrams it uses less traffic in six, more in 66 and the same in 24; its peak is lower in 27, higher in 52 and equal in 17. These are coverage counts, not workload weights.

## Runtime remains exploratory

Five repetitions size each of the 192 retained-all timing cells. The overlap union/explicit total-time ratio ranges from 0.740 to 2.115 despite a median ratio of 1.110. The observed variation does not support a directional timing conclusion there. No outlier is excluded.

One hundred of the 160 matched timing contrasts have an insufficient-signal cell under the registered clock rule. The maximum median empty-clock cost is 18 ns; the rule accounts for the number of phase intervals. Faster-looking microsecond membership results therefore are not reported as confirmed speedups. Other contrasts have larger signals but still only sizing repetitions. Further timing precision must target a decision-relevant regime after ownership/reuse attribution.

## What the comparison includes

Every measured session charges selected-source construction, preparation, source disposal, changing requests, execution/complete logical output, request disposal, consumer handling, prepared disposal and retained-output disposal. Allocation builds have no phase clocks; ordinary timing builds have no allocation meter and engine metrics are disabled. Requested heap allocation and peak excess are not RSS.

Execution and observation are fused. First-row latency is not separately measured. Native program compilation, process startup, oracle fixtures and fixed preallocated harness recording/consumer containers are excluded. This is a bounded logical-set lifecycle comparison, not complete CHR architectural lifecycle superiority. Identical result-container types are charged in every mode; insertion order and resulting tree occupancy can still affect their memory costs.

The diagram now visits accepted assignments directly, expanding skipped coordinates and pruning caller-rejected prefixes. Explicit and reduced traversal reject invalid partial assignments; explicit membership short-circuits. A visitor can stop after its first result, and a fresh query reuses the same preparation. Full-output preflight checks that prefix against the independent answer and verifies ownership restoration. Retained outputs are checked after preparation disposal; every measured allocation session returns exactly to its initial owned-heap baseline.

## Responsibilities and language limits

Union owns a decision graph, canonical-node table and temporary import/application maps. Some sharing is useful; repeated operand construction, duplicate key storage and retained construction intermediates are implementation choices to investigate. The simpler disjunction owns constraints and performs checks at request time. Its lack of a retained decision graph is a real responsibility difference, not a proof that it is always preferable.

Both paths still need caller validation, exact logical output and consumer ownership. The new traversal's prefix predicate must soundly reject all extensions of a rejected prefix; the tested fixed-name, alias and contradictory callers satisfy that contract. Budget exhaustion and visitor stopping remain distinct from exhaustive output.

The [source gate](S06-useful-union.md) limits the inference: three-name logical assignment sets do not preserve raw successful-branch counts or residual CHR occurrences by themselves. Hidden projection, richer structural theories, asynchronous interruption during union construction and sustained service remain separate investigations.

## Verification and next decision

The [audit](s06-union-lifecycle/audit.json) reconstructs every phase, exact allocation pair and sampled timing contrast from [raw receipts](s06-union-lifecycle/). [Frozen sources and binaries](s06-union-lifecycle/freeze.json), the [driver](../../../research/chr-structural/experiments/union_lifecycle.py) and [auditor](../../../research/chr-structural/experiments/audit_union_lifecycle.py) make the comparison reproducible. The full structural suite passes 78 tests across 21 targets in each feature build; strict Clippy and six historical evidence audits pass. Historical audit snapshots are byte-checked against their original source hashes.

Next register a reuse extension around 15 queries and at longer reuse, retaining disjoint, redundant, single-branch and membership controls. Confirm whether the predicted traffic reversal occurs and whether peak memory or runtime reverses with it. Compare that result with preparation attribution before implementing an optimization. This is more immediately discriminating than another capability gate because the existing measurements predict a concrete reversal; structural-prefix generated/access controls and unfinished demand/integration costs remain the strongest distinct alternatives. Reconsider them at that result or obstruction. This pilot is package two since the full portfolio review; the goal remains active.
