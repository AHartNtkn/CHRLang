# Known-prefix complete-cost comparison (T055)

Known-prefix contraction makes the long unknown-tail timing comparison overlap
Conditional at both registered reuse counts. Ground controls favor Carrier with
separated ranges; zero/short unknown prefixes favor Conditional. Pure-carrier
execution therefore supports a bounded lowering option, not a blanket preference
for either sharing or explicit execution.

All192 processes,7680 complete answers and24cells pass the
[raw audit](r05-carrier-prefix-cost/audit.json), including exact source-work
predictions, exhaustion, allocation restoration and source/binary freeze at
`11bbb45`. The [registration](../registrations/R05-carrier-prefix-cost.md) fixes
all configurations, cells, repetitions and resource bounds. Complete-path primary
measurements use ordinary allocation with counters disabled; work/allocation are
separate runs. The [semantic gate](R05-carrier-prefix-gate.md) independently proves
the relevant tails, identities and arbitration behavior.

## Complete cost

Milliseconds per query: median [minimum,maximum] of five repetitions, including
amortized preparation and all measured query/service/disposal phases. Unknown q4
alternates pre0/1,1/2 or64/65; ground q4 alternates(64,0)/(65,1).

| [pre,post,queries,tail] | Conditional | Specialized | Carrier |
|---|---:|---:|---:|
| [0, 0, 1, 'unknown'] | 0.203 [0.200, 0.206] | 0.330 [0.303, 0.360] | 0.389 [0.349, 0.422] |
| [0, 0, 4, 'unknown'] | 0.141 [0.133, 0.160] | 0.274 [0.260, 0.286] | 0.259 [0.252, 0.291] |
| [1, 0, 1, 'unknown'] | 0.211 [0.190, 0.218] | 0.413 [0.366, 0.428] | 0.419 [0.367, 0.437] |
| [1, 0, 4, 'unknown'] | 0.151 [0.146, 0.165] | 0.285 [0.282, 0.298] | 0.323 [0.292, 0.336] |
| [64, 0, 1, 'unknown'] | 0.769 [0.708, 0.841] | 4.508 [4.350, 4.739] | 0.851 [0.782, 0.900] |
| [64, 0, 4, 'unknown'] | 0.699 [0.695, 0.740] | 4.521 [4.508, 4.607] | 0.699 [0.679, 0.762] |
| [64, 0, 1, 'ground'] | 1.809 [1.724, 1.971] | 2.227 [2.208, 2.289] | 0.798 [0.783, 0.897] |
| [64, 0, 4, 'ground'] | 2.011 [1.820, 3.664] | 2.142 [2.061, 2.285] | 0.722 [0.712, 0.800] |

Carrier beats uncontracted Specialized in all four long-prefix cells. All four
short-prefix pairs overlap between those explicit configurations. Conditional
beats both explicit configurations in all four zero/short unknown cells. The two
long unknown Carrier/Conditional pairs overlap; they do not establish equivalence.
Ground Carrier/Conditional pairs separate in favor of Carrier. Retain the ground
reuse Conditional outlier in the reported range.

Allocation gives a different consideration. Long unknown cold requested traffic is
741,959 bytes Conditional,5,067,062 Specialized and814,034 Carrier. Peak above the
held baseline is97,789/438,789/438,207 bytes respectively. Contraction reduces
traffic substantially against Specialized but retains its higher peak ownership.
For long ground cold, traffic is1,507,849/3,539,760/850,396, while peak is
123,891/428,540/427,958. Requested heap is not RSS, and lower traffic is not itself
a timing conclusion. [Summary](r05-carrier-prefix-cost/summary.json) records all
phases, preparation, first observation, allocation and work fields.

## Architecture disposition

The pure countdown's repeated dispatch is avoidable under a source-derived
certificate. Prefix contraction requires only known unary steps, preserving the
actual tail, source identity and ordinary terminal arbitration. It has less runtime
state than whole-spine admission, but still adds source inference, job ownership,
resumable inspection and identity accounting. Feature-off controls carry none of
those candidate responsibilities. This evidence supports the opt-in bounded
lowering path. It does not mandate a source-language restriction.

Conditional remains favorable for the short residual endpoint and has lower peak
heap throughout this registered contrast. Its long unknown timing advantage is
unresolved against the competent prefix control. There is no reason to repeat
these timings merely to force an ordering. No cross-freeze timing ratios,
compilation payback or universal lifecycle ranking follows.

Substantive noncontractible equation/constructor pre-discrimination work remains
a distinct question: the pure-carrier certificate does not represent such work.
The next selection must assess that fixture against a current maintained-join
bottleneck screen, with a competent lowering alternative and observable correlation.
The goal remains active; this package completes only its bounded comparison.
