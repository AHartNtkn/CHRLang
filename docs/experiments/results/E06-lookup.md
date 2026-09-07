# E06a: preserving lookup with explicit copying

All 135 registered configurations complete with correct full outputs; the second
batch reproduces every non-time field, including traced allocation. The current
net protocol's table copy and erasure dominate even a first-entry lookup. A
preserving traversal is a concrete next ablation; neither native net performance
nor full CHR execution is settled here.

## Reproduction

Harness and prospective registration commit: `34bd227`.

```
python research/chr-nets/run_comparison.py docs/experiments/results/E06-lookup-v1.jsonl --seed 601
python research/chr-nets/run_comparison.py docs/experiments/results/E06-lookup-replay.jsonl --seed 602
```

The runner pins PYTHONHASHSEED=0. The [registration](../registrations/E06.md)
defines 45 requests and three modes, 60 seconds/process and 200,000 interactions.
No process times out or spends its interaction bound. The eight-test
[conformance gate](E06-data-gate.md) passes. Both direct controls materialize the
complete preserved table and result during extraction. The copied control also
copies inputs; the borrowed control retains immutable input references.

## Counter evidence

For V=64 and payload depth D=64:

| Query | Entries inspected | Net interactions | Dup interactions | Erase interactions | Live agents after execution | Retained slots |
|---|---:|---:|---:|---:|---:|---:|
| First | 1 | 16,836 | 8,450 | 8,381 | 8,518 | 42,190 |
| Last | 64 | 25,089 | 12,545 | 8,192 | 8,581 | 58,759 |
| Absent | 64 | 25,347 | 12,609 | 8,385 | 8,452 | 59,146 |

The first-entry case performs one Eq root inspection. Dup and Erase account for
16,831 of its 16,836 interactions. Holding V=64 and first-entry lookup fixed,
increasing D from 0 to 8 to 64 increases interactions 8,708 → 9,724 → 16,836,
without inspecting another entry. This attributes service traffic to the explicit
preservation/cleanup protocol, not to a more difficult key comparison.

Unary IDs also grow the data representation. Even depth-zero payloads retain
Ref(i), so V growth is not a constant-size-key experiment. Binary IDs and shared
handles remain distinct feasible representation ablations.

## Instrumented costs and limits

On the first-entry V=64,D=64 request, one batch records net compilation,
initialization, execution and extraction at approximately 4.8, 48.3, 1,044.7 and
61.4 milliseconds. Borrowed direct execution takes about 0.014 ms and extraction
32.3 ms; copied initialization adds 30.3 ms. Maximum traced phase peaks are about
4.35 MB for nets, 0.95 MB borrowed and 1.83 MB copied. Raw files preserve both
batches and every phase. These are instrumented Python timings, not forecasts of
a specialized native runtime. The interpreter's generic graph replacement and
tracing contribute substantial overhead.

Source payload construction is outside measurement for every mode. Net input
encoding is charged; borrowed access needs none. Rule generation is charged per
process, while amortized backend compilation remains a later question. Traced
memory is neither RSS nor cumulative allocation traffic. Live agent reclamation
does not reclaim the current node-slot vector, whose metadata retention is
separately visible. Full output extraction is a material cost in every mode.

## Next discriminating work

Implement a preserving traversal that reconstructs only the consumed prefix and
returns the untouched tail, copying keys needed for comparison and only the
selected value needed in both outputs. Compare it against the current protocol
and both direct controls with identical returned table/result observations. This
isolates avoidable whole-table fans/erasers without changing source semantics.
It must handle misses, duplicate keys, finite yields and both reduction orders.

The complete finite-tree binding service, regional dispatch, multiport inspection,
routed alias cells, encoded backend and native choice invariants remain feasible
independent work. They are not blocked by improving this first lookup protocol.
