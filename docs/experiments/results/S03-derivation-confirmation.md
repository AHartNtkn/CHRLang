# Repaired derivations earn gains, but direct source elimination remains stronger here

**Fresh derivation templates now improve substantial recursive execution, including independently born choices.** They still add overhead to some shallow queries and lose to compiled execution on the growing-output family. The exact-source control beats templates in every tested configuration; this remains a source-specific result, not evidence that a general compiler can obtain it.

## What the confirmation establishes

The [prospective comparison](../registrations/S03-derivation-confirmation.md) covers 80 source configurations and six execution paths. It completed **3,360 ordinary timing processes, 960 allocation processes and 24 cancellation processes**. All complete answers passed the independent scalar check, allocation pairs replay exactly, and query/prepared disposal restored requested-allocation baselines. The experiment used the validated repaired binaries, checked against their source and binary hashes.

Seven same-block measurements per configuration support the registered pointwise comparison. A practical gain requires the 95% bootstrap interval of the paired primary-cost ratio below 0.90; a practical loss requires it above 1.10. Other comparisons remain unresolved under that rule.

| Numerator / denominator | Gains | Losses | Unresolved |
|---|---:|---:|---:|
| Templates / ordinary demand | 32 | 16 | 32 |
| Templates / compiled scan | 48 | 8 | 24 |
| Templates / compiled indexing | 64 | 8 | 8 |
| Templates / inferred specialization | 42 | 8 | 30 |
| Exact-source control / templates | 80 | 0 | 0 |
| Exact-source control / inferred specialization | 76 | 0 | 4 |

These counts describe the registered configurations. They are neither workload frequencies nor simultaneous population-wide claims. No aggregation selects an architecture.

## The favorable and adverse regimes differ

All 32 substantive single-call, repeated-call, distinct-call and independent-choice configurations gain against ordinary demand. Repeated, distinct and choice configurations also all gain against inferred specialization. Substantive single calls have four gains and four unresolved comparisons against specialization.

The independent-choice source creates four fresh choices and must produce all 16 joint alternatives. It exercises fresh derivation instantiation rather than incorrectly correlating separate applications. At depth32/33 over eight queries with resources and forward starting order:

| Family | Demand | Templates | Scan | Indexed | Specialized | Exact-source control |
|---|---:|---:|---:|---:|---:|---:|
| One call | 0.537 | 0.194 | 0.403 | 0.464 | 0.266 | 0.054 |
| Four repeated calls | 1.848 | 0.419 | 1.521 | 1.629 | 1.059 | 0.137 |
| Four distinct calls | 1.813 | 0.659 | 1.426 | 1.632 | 1.067 | 0.134 |
| Four independent choices | 14.589 | 1.791 | 3.756 | 4.349 | 2.785 | 0.236 |
| Growing output, depth12/13 | 11.366 | 12.465 | 7.100 | 6.962 | 6.832 | 6.299 |

Values are median primary milliseconds, including preparation, changing-query setup, execution with complete observation, and disposal. Ratios and classification use paired measurements, not ratios of these marginal medians. Source/input-inclusive medians preserve the ordering of these representative rows.

Gains on single and distinct calls matter: template hits cannot explain them. Ground-input specialization also eliminates recursive source execution within a new request. The result supports both specialization and reuse opportunities; it does not isolate the marginal benefit of caching from every other cost.

Shallow sources retain adverse evidence. All eight shallow distinct-request configurations lose against ordinary demand, with eight additional shallow losses across the other families. Substantive growing-output configurations are unresolved against demand and all eight lose against scanned, indexed and specialized execution. The source family therefore changes the supported choice.

## Total memory and observation costs constrain the result

For the substantive independent-choice row, templates request 2.654 MB versus 12.603 MB for demand and 4.360 MB for specialization. Their peak requested growth is about 100 kB, versus 100 kB and 215 kB respectively. The exact-source control requests 0.269 MB and peaks at 19 kB. Lower traffic and lower peak are distinct findings; neither alone measures sustainable lifetime.

Growing-output templates request 12.003 MB and peak at 1.653 MB, while specialization requests 5.320 MB and peaks at 0.866 MB. Template execution with observation takes about 8.230 ms versus 2.736 ms for specialization. Setup and disposal also differ, so the execution interval is not the complete explanation of the lifecycle gap.

Inspection identifies a concrete remaining observation hypothesis: demand normalization clones constructor nodes while traversing, and forcing also examines cloned nodes. Those temporary child-vector copies can add traffic when a compact internal graph is exported as a large owned tree. The confirmation does not isolate that cost or prove it intrinsic to graph execution. Exact observation/export attribution remains required under S08; the growing-family loss applies to these implementations.

All timings use counters disabled and ordinary allocation. Allocation diagnostics are separate and measure requested heap demand, not RSS. Complete owned answers are part of this comparison's contract. A compact-output representation is a separate observation comparison. Native compilation is outside these intervals.

## What changes in the architecture decision

Retain fresh derivations as a supported optimization opportunity for substantive eligible source work. Their benefit survives a stronger conventional specialization control in several regimes and preserves independent dynamic identities. The source/identity gate and bounded continuation tests supply correctness evidence; the repaired compiler adds shared expression ownership, a key/cache, fresh instantiation and ground-node lookup.

Do not infer that all programs should use templates. Ground keys, bounded private derivation and ordinary effect continuations define the tested eligibility. Shallow overhead and growing-output costs remain real counterpressure. Neither these restrictions nor the source gate adopts a language-wide policy. The contested-resource scheduling distinction remains outside the timed same-answer sources.

The exact-source control demonstrates that both template and conventional execution retain avoidable work on these schemas. It validates complete source/query structure and generates the exact answer relation. Its strong result motivates general source-derived compilation, but cannot prove an inference algorithm, broader eligibility or compilation amortization.

Further precision on the unresolved local rankings cannot overturn the exact-control advantage within these 80 configurations. It may refine a component choice, but is lower value now than testing a distinct integrated mechanism or a general lowering transformation. The [breadth review](S03-derivation-breadth-review.md) selects the next investigation and preserves the remaining observation and derivation questions.

[Every configuration and interval](s03-derivation-confirmation/summary.json) · [Source/binary freeze and raw receipts](s03-derivation-confirmation/freeze.json) · [Analysis script](../../../research/chr-direct-conditional/experiments/derivation_confirmation_analysis.py) · [Applicable source validation](s03-shared-template-bounded/tests.log)
