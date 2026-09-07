# E12 structural failure certificates: projection control

The corrected 200-configuration matrix and exact non-time replay pass. Structural
certificates avoid repeated equation work, but full operand projection makes the
long mechanism probes more expensive in total. Shared-arena path checking is the
next registered test of that representation cost.

Code `9386e62` supplies the certificate/checker/source driver; `5f6fa83` corrects
workload placement for the persistent unifier's rightmost-first stack. The initial
[v1](E12-failure-v1.jsonl) and [replay](E12-failure-replay.jsonl) are placement
diagnostics: they do not exercise the intended late-prefix control. The corrected
[v2](E12-failure-v2.jsonl) and [replay](E12-failure-v2-replay.jsonl) check 2 early
pairs, D+3 late pairs, and D+3 occurs visits per derivation before comparison.

At K8 D64, all three families retain 1,024 logical jobs and 256 failed derivations;
learning substitutes 255 checked failures for ordinary equation execution.

| Family | Direct / Learn work | Requested bytes: Direct / Learn | Elapsed microseconds, two runs: Direct / Learn |
| --- | --- | ---: | --- |
| Early clash | 512 / 2 pairs | 161,201 / 1,828,681 | 262, 255 / 3,642, 4,154 |
| Late clash | 17,152 / 67 pairs | 205,564 / 1,806,276 | 409, 409 / 3,217, 3,634 |
| Occurs | 17,152 / 67 occurs visits | 749,693 / 947,473 | 761, 1,225 / 1,844, 1,843 |

The checker makes 512 path-node checks per learned configuration, while operands
are exported 256 times. Miss discovery is also charged. These counters motivate
selective arena inspection; they do not by themselves isolate every runtime cost.
Peak requested live storage remains similar because exports are short-lived.

Application results differ from full-state reconvergence. SK duplication has 113
certificate hits, reducing pairs from 475 to 115 and occurs visits from 424 to
184. Requested allocation rises from 404,366 to 452,510 bytes, and the two elapsed
comparisons disagree in direction (606/729 and 601/460 microseconds). This is work
avoidance with inconclusive timing, not a speed claim. Other arithmetic, SK/lambda
and inference cases also have hits; the requested type-synthesis prefix has none.

Every emitted answer and failure/prefix count agrees with the independent expected
observations. The proof gate checks 91 path templates against 576 operand pairs
and independently executed equations. A committed-consumer example verifies that
learning does not prune the branch where the equation is never posted. The
checker validates current structural assumptions; it does not reuse unchecked
original caller names or occurrence assumptions.

Certificates expose direct structural clashes and occurs paths. Intermediate
binding explanations, richer context learning, cache indexing/lifetime and larger
synthesis applications remain open. No semantic or production decision follows.
The matrix uses the registered release/fresh-process allocator harness; raw rows
retain both repetitions and all cost counters. Reproduce with `failure_probe`
and `research/chr-reuse/run_failure.py` at the corresponding commit.
