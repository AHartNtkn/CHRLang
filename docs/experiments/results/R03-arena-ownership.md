# Arena ownership lifecycle evidence (T052)

Retain lookup-before-detachment arena sharing as a bounded serial ownership
option. It resolves a complete lifecycle advantage in all eight read-preserving
choice pairs. All 16 immediate-insertion pairs overlap; one no-choice pair resolves
in favor of cloning. This supports an applicability boundary, not a universal
replacement or automatic routing policy.

## Validated comparison

The candidate, source gate, runner and [registration](../registrations/R03-arena-ownership.md)
were committed at `028525f` before execution. All 512 planned processes completed
with 21,440 validated answers across 32 paired cells. Every source-work record
agrees between ownership configurations. Allocation baselines restore, and no
cutoff, missing job, malformed result, source change or binary change occurs.
Maximum process duration was 0.939 seconds within the 30-second/1-GiB bound.

The root and independent reviewer checked the exact manifest, command/configuration
correspondence, complete observations, primary medians, baselines and frozen
hashes. [Raw evidence](r03-arena-ownership/),
[audit](r03-arena-ownership/audit.json),
[complete phase/memory summary](r03-arena-ownership/summary.json) and
[semantic/runner gates](r03-arena-ownership-gate/) retain reproducible receipts.
The workspace gate has 323 passing tests and one ignored test. Adverse source
replay covers construction before failure, sibling isolation, aliases and
re-fork after insertion. Predicate hits and insertions test the ownership boundary
separately.

Primary builds use the ordinary allocator with counters disabled. Allocation and
source-work diagnostics are separate. Preparation, query construction, setup,
service, full observation and all disposal are charged. Validation occurs outside
measured intervals; answer disposal follows validation in both configurations.
No timing ratio below uses the earlier diagnostic freeze. Compilation cost is not
credibly isolated in this comparison.

## Findings and contrary evidence

For n512/a64/q1 mostly-fail read, median complete lifecycle falls from 20.454 to
9.359 ms per query, with separated ranges 19.994–27.876 and 9.336–9.698 ms.
Requested traffic falls from 44,898,961 to 29,892,690 bytes and peak requested heap
from 1,427,036 to 1,188,707. Service accounts for the change: its medians are 20.114
and 9.042 ms; preparation is 0.036 and 0.038 ms. First full observation likewise
falls from 19.986 to 8.882 ms of service. This is a complete measured benefit,
including repeated branch disposal, rather than an estimate from clone traffic.

The all-success read control also benefits: n512/a64/q1 medians are 179.222 versus
143.497 ms, and requested peak is 41,075,680 versus 26,059,465 bytes. Cleanup and
ordinary mutable state remain substantial, so avoiding arena copies does not
eliminate the complete source work. At size zero, alternative syntax still gives
an inherited arena; those a64 read cases also have separated benefits.

Immediate insertion largely restores the copying cost. All 16 insertion pairs have
overlapping timing ranges; there is no resolved insertion advantage. For large
mostly-fail cold insertion, medians are 20.834 versus 21.631 ms and requested traffic
51,405,326 versus51,416,166 bytes. Large all-success insertion has slightly lower
traffic with COW but slightly higher peak: 44,239,264 versus 44,264,490 bytes cold.
These results preclude a universal time or memory recommendation.

The n512/a1/mostly-fail/q1 read pair favors cloning: 3.749 versus 3.922 ms, with
ranges 3.433–3.769 and 3.818–4.009. Other no-choice pairs overlap. Across all no-choice
cells, COW adds 208 requested bytes to a cold session, or 70 bytes per query at q4,
and 24 peak bytes. Ownership and lookup overhead therefore remains real even when
no copying is avoided. Do not repeat the small cases solely to obtain a preferred
numerical ordering.

## Complete paired matrix

Each entry is median[min,max] milliseconds per query, including amortized
preparation and disposal. Ranges use all five primary repetitions and are a
conservative within-session screen, not confidence intervals.

| n | a | outcome | q | mutation | clone ms | COW ms | range disposition |
|---:|---:|---|---:|---|---|---|---|
| 0 | 1 | mostly-fail | 1 | read | 0.058 [0.051, 0.079] | 0.076 [0.059, 0.092] | overlap |
| 0 | 1 | mostly-fail | 1 | insert | 0.069 [0.059, 0.080] | 0.080 [0.072, 0.086] | overlap |
| 0 | 1 | mostly-fail | 4 | read | 0.029 [0.025, 0.033] | 0.028 [0.026, 0.030] | overlap |
| 0 | 1 | mostly-fail | 4 | insert | 0.030 [0.028, 0.035] | 0.035 [0.031, 0.038] | overlap |
| 0 | 1 | all-success | 1 | read | 0.069 [0.056, 0.093] | 0.066 [0.054, 0.071] | overlap |
| 0 | 1 | all-success | 1 | insert | 0.058 [0.053, 0.069] | 0.079 [0.067, 0.103] | overlap |
| 0 | 1 | all-success | 4 | read | 0.031 [0.028, 0.032] | 0.030 [0.025, 0.033] | overlap |
| 0 | 1 | all-success | 4 | insert | 0.033 [0.032, 0.036] | 0.035 [0.029, 0.039] | overlap |
| 0 | 64 | mostly-fail | 1 | read | 0.697 [0.640, 0.701] | 0.193 [0.178, 0.216] | cow-lower |
| 0 | 64 | mostly-fail | 1 | insert | 0.864 [0.856, 0.939] | 0.875 [0.792, 0.882] | overlap |
| 0 | 64 | mostly-fail | 4 | read | 0.675 [0.662, 0.699] | 0.163 [0.151, 0.166] | cow-lower |
| 0 | 64 | mostly-fail | 4 | insert | 0.909 [0.858, 0.923] | 0.859 [0.827, 0.903] | overlap |
| 0 | 64 | all-success | 1 | read | 0.957 [0.907, 1.024] | 0.418 [0.386, 0.454] | cow-lower |
| 0 | 64 | all-success | 1 | insert | 1.388 [1.348, 1.604] | 1.349 [1.227, 1.370] | overlap |
| 0 | 64 | all-success | 4 | read | 0.992 [0.973, 1.050] | 0.440 [0.435, 0.470] | cow-lower |
| 0 | 64 | all-success | 4 | insert | 1.364 [1.281, 1.411] | 1.369 [1.354, 1.432] | overlap |
| 512 | 1 | mostly-fail | 1 | read | 3.749 [3.433, 3.769] | 3.922 [3.818, 4.009] | clone-lower |
| 512 | 1 | mostly-fail | 1 | insert | 3.863 [3.471, 4.245] | 3.626 [3.472, 3.889] | overlap |
| 512 | 1 | mostly-fail | 4 | read | 3.679 [3.550, 3.781] | 3.741 [3.696, 3.841] | overlap |
| 512 | 1 | mostly-fail | 4 | insert | 3.724 [3.543, 3.750] | 3.736 [3.694, 4.171] | overlap |
| 512 | 1 | all-success | 1 | read | 3.688 [3.628, 4.021] | 3.655 [3.577, 3.892] | overlap |
| 512 | 1 | all-success | 1 | insert | 3.658 [3.313, 4.031] | 3.630 [3.536, 3.901] | overlap |
| 512 | 1 | all-success | 4 | read | 3.802 [3.682, 5.221] | 3.689 [3.606, 3.748] | overlap |
| 512 | 1 | all-success | 4 | insert | 3.644 [3.495, 3.773] | 3.788 [3.735, 3.949] | overlap |
| 512 | 64 | mostly-fail | 1 | read | 20.454 [19.994, 27.876] | 9.359 [9.336, 9.698] | cow-lower |
| 512 | 64 | mostly-fail | 1 | insert | 20.834 [20.534, 21.018] | 21.631 [20.961, 24.699] | overlap |
| 512 | 64 | mostly-fail | 4 | read | 20.682 [20.401, 21.979] | 9.517 [9.311, 9.600] | cow-lower |
| 512 | 64 | mostly-fail | 4 | insert | 20.698 [20.640, 21.934] | 21.856 [21.782, 22.309] | overlap |
| 512 | 64 | all-success | 1 | read | 179.222 [178.300, 186.049] | 143.497 [142.602, 146.087] | cow-lower |
| 512 | 64 | all-success | 1 | insert | 184.323 [183.595, 190.941] | 188.715 [185.677, 194.802] | overlap |
| 512 | 64 | all-success | 4 | read | 172.681 [171.713, 175.928] | 142.504 [140.090, 143.094] | cow-lower |
| 512 | 64 | all-success | 4 | insert | 177.803 [175.594, 179.623] | 179.460 [178.883, 183.206] | overlap |

## Architecture and next decision

The candidate adds one reference-counted arena owner and a controlled mutation
boundary. Nodes, immutable closedness and dictionaries remain together; term IDs
stay stable and variable bindings remain branch-local. Lookup precedes detachment.
Source scheduling, occurrences, indexing and output semantics do not change.
This is simpler than introducing a trail/replay lifecycle across all engine state,
but it retains indirection and copying on insertion. Rc supplies serial sharing;
this experiment establishes no concurrent ownership advantage.

Keep the experimental feature available and retain the cloning control. The
results show that copying is an implementation choice with a useful bounded
alternative; they do not require a language restriction on term construction.
They also do not rank persistent occurrence maps, trails or replay. Further
storage refinement lacks a comparably specific unanswered decision now that the
arena option has both a measured useful region and adverse controls.

Select the pure-carrier eligibility gate next. The remaining positive conditional
regime includes an equation-free countdown; valid source-derived contraction might
eliminate that repeated work instead of sharing it. Its certificate must establish
ordinary source priority, one live carrier, a finite control spine, unchanged
opaque payloads, absence of interior observers and safe suspension/identity
accounting. Existing contextual counterexamples remain requirements. A gate may
reject this proposed scope; success must not be presumed from syntactic purity.

Independent review supports this ordering over further storage tuning. T052 is
complete; the broader goal remains active with T053 examining this semantic
boundary. No workload weights, universal winner or production policy is inferred.
