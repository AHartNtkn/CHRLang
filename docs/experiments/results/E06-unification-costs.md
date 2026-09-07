# E06a: full unification service costs

All 384 registered configurations pass and reproduce every non-time field in a
second batch. The controls distinguish lookup organization, unary representation,
and explicit graph operations. The present ordinary-net protocol performs much
more work than the direct controls; this is a result about the measured encoding
and interpreter, not a rejection of local nets or a native-backend speed estimate.

## Reproduction and coverage

Prospective registration, validated controls and harness: commit `6d35dbf`.

```
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-nets -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-nets/run_unification.py docs/experiments/results/E06-unification-costs-v1.jsonl --seed 605
PYTHONDONTWRITEBYTECODE=1 python research/chr-nets/run_unification.py docs/experiments/results/E06-unification-costs-replay.jsonl --seed 606
```

Sixteen tests pass. Each direct mode independently matches the 432 exported
reference requests; all four modes match the 96 registered cost requests before
measurement. The 96 comprise 12 predetermined reference requests and 84 controlled
requests. Original tables, complete resulting bindings, and projected values for
all selected variables are extracted. Failed requests expose no private partial
substitution. No process times out or spends the 200,000-interaction bound.

Each fresh process charges compilation, initialization, execution and extraction.
The map, native-table and encoded-table modes use the same direct worklist
algorithm. The encoded table retains immutable references; the net protocol
consumes/copies data explicitly. Thus encoded-direct versus net is not an ablation
of dispatch alone. Rule generation is charged per request; amortized compilation
and retained executable state remain separate questions.

## Separating explanations

For alias-chain binding with N=16,D=16, every direct control processes one equation,
17 dereferences and 17 occurs nodes. Native table lookup visits 152 entries;
map lookup performs 17 probes. Encoded-table lookup also visits 152 entries, but
unary ID comparison visits 952 nodes rather than 152 native ID comparisons.
These counters separate environment search from key representation. Map probes
and table entries are different units despite sharing the raw `entries` field.

The net performs 7,326 interactions, including 3,494 Dup and 1,171 Erase
interactions. Its preserving lookup inspects the same 152 entries. It retains
15,447 node slots after finishing with 795 live agents; peak live agents are 879.
This identifies copying, cleanup and slot metadata as additional costs beyond
linear lookup. It does not attribute all host allocation to those counters.

For indirect occurs failure at N=16,D=16, direct controls process one equation,
18 dereferences and 18 occurs nodes. Map lookup uses 18 probes; linear lookup
visits 168 entries. Encoded IDs require 1,105 node comparisons. Net execution
uses 8,765 interactions, 3,941 Dup, 1,786 Erase, and the same 168 entry inspections.
Failure ends with the original table and no partial result, not an early result
that avoids the required cleanup/observation contract.

For identity with no padding, the net uses 27 interactions. Adding 16 unrelated
depth-16 bindings raises that to 2,611, including 2,125 Dup and 276 Erase, although
there is still just one identity equation and no occurs traversal. Preserving
lookup avoids copying the table on every lookup; the transactional request still
preserves a caller copy, which remains a distinct measurable cost.

## Instrumented timing and storage

For N=16,D=16, execute times in microseconds across the two batches are:

| Request | Map | Native table | Encoded table | Net |
|---|---:|---:|---:|---:|
| Alias-chain binding | 41 / 36 | 69 / 62 | 339 / 340 | 495,193 / 487,197 |
| Indirect occurs failure | 43 / 39 | 62 / 63 | 376 / 325 | 519,929 / 530,655 |
| Identity with padding | 5 / 5 | 14 / 17 | 14 / 17 | 185,546 / 181,100 |

Net total time exceeds each direct total on all 96 requests in both batches.
These are traced Python executions with a generic graph rewriter and a deliberately
explicit data encoding. They are not forecasts of optimized C/Rust/HVM execution.
Both the direct algorithm and net implementation would need credible native
controls before comparing native speed.

Full output costs also matter: alias-chain binding totals are about 1.0–1.1 ms for
the two native direct controls despite execute phases of only tens of microseconds.
For indirect failure, total map time is 116–129 microseconds and net time
531,642–542,564 microseconds. One batch's maximum traced peaks are 11,432 bytes
for map and 697,177 for net. Traced peaks are not RSS or cumulative allocation,
and the raw files separate phases rather than treating those totals as service
operation counts.

## Resulting investigation choices

The present protocol is a correctness/control implementation, with no evidence
supporting it as an efficient production service. The evidence supports testing
routed variable cells or a shared immutable store to avoid encoded linear scans
and whole-request copying. Such a protocol must still preserve aliases, private
failure and finite yields; the direct map is a useful comparison, not an implemented
net routing protocol. Binary IDs isolate key representation. Reusing inactive node
slots isolates metadata retention. These are concrete feasible follow-ups, not
reasons to close E06a.

Nested/multiport operand inspection can remove intermediate controller steps, but
must be measured separately from the dominant Dup/Erase and table costs. Source
freshness/ownership certificates may avoid particular services; no global mode or
linearity restriction is inferred from these results. Regions, wake-ups, branch
support and intended synthesis still require integration checks.

General encoded-machine compilation, native-label correspondence and scheduling
remain independent. None needs this ordinary-net protocol to become fast first.
