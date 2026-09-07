# E06a: preserving-prefix lookup ablation

All 180 v2 configurations pass and replay exactly outside timing. The preserving
protocol reduces active-pair work on every nonempty request (36 of 45); empty
requests tie (nine). Both modes return the same preserved table and option result.
No source-language restriction is needed for this service transformation.

Prospective harness/registration commit: `0e0aac5`. Reproduce with:

```
python research/chr-nets/run_comparison.py docs/experiments/results/E06-lookup-v2.jsonl --seed 603
python research/chr-nets/run_comparison.py docs/experiments/results/E06-lookup-v2-replay.jsonl --seed 604
```

Nine conformance tests pass, including the existing 18,818 comparisons and new
physical payload preservation, duplicate-key and finite-yield checks. Both net
modes compile the same 168-rule/35-agent-type system. The earlier three-mode data
remain in the v1 files; comparisons below use the common v2 compiler.

## Distinguishing the cause

At V=64,D=64:

| Query | Copying interactions | Preserving interactions | Copying Dup/Erase | Preserving Dup/Erase | Copying slots | Preserving slots |
|---|---:|---:|---:|---:|---:|---:|
| First | 16,836 | 75 | 8,450 / 8,381 | 68 / 1 | 42,190 | 8,668 |
| Last | 25,089 | 12,801 | 12,545 / 8,192 | 6,305 / 2,080 | 58,759 | 34,183 |
| Absent | 25,347 | 12,803 | 12,609 / 8,385 | 6,240 / 2,145 | 59,146 | 34,058 |

Final live-agent counts agree between protocols: 8,518, 8,581 and 8,452,
respectively. Output size is unchanged. The saving is intermediate service work
and storage, not a weaker observation contract.

For first-entry lookup, D=0,8,64 gives preserving interaction counts 11,19,75.
The increase is exactly the selected value's copy size, instead of copying and
erasing unrelated payloads across the table. Misses retain encoded-key copying,
comparison and reconstruction work. Unary key size remains a material confound
for attributing V scaling purely to entry count; a binary-key probe can separate it.

In one batch the large first-entry execute phase drops from 1,108 to 5.2 ms;
maximum traced peak drops from 4.36 to 2.98 MB. The large miss drops from 1,556 to
857 ms and 7.96 to 4.23 MB. These are instrumented Python observations. The generic
net interpreter still has different dispatch overhead from the direct controls;
full encoding, rule generation and extraction remain charged. No native speedup
claim follows. Empty cases use three interactions in either protocol, so observed
small timing differences there are not evidence of reduced work.

## Correspondence and next work

Keep returns Pair(table, option). It copies a key to test it while retaining its
original encoded value for reconstruction. On a hit it routes the tail unchanged,
rebuilds the current entry and duplicates only the returned value. On a miss,
Restore holds that entry while querying the tail, then rebuilds the prefix around
the recursively preserved table. Induction on finite list length preserves first
match and the entire table. Every controller transition remains a finite local
rule and yields through the same interface.

The result supports preserving traversal as the better of these two tested
protocols, not as an adopted production representation. Encoded key alternatives,
slot reclamation, complete transactional unification and regional source services
remain feasible follow-ups. Encoded-backend continuation and native-label work
proceed independently rather than waiting for further lookup refinements.
