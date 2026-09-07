# E06b: pinned backend continuation boundary

The finite data-controller gate returns the expected states in both normalization
modes. A constructor or lambda around a diverging call does not establish a return
boundary. This is evidence for explicit data continuations in the encoded-machine
path, not yet an executable general CHR compiler or a native-choice result.

## Reproduction and observations

[Build manifest](E06-hvm-build.json) records the clean pinned source revision
`6defdfc7dae2a3cca5dd6e74ed0612385b5646a8`, source/binary hashes, Clang 14.0.0 and
`clang -O2` command. The temporary source checkout is the one inspected in the
research dossier. No upstream behavior beyond that revision is assumed.

```
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-hvm -v
python research/chr-hvm/continuation_gate.py /tmp/chr-hvm4-e06 /tmp/chr-hvm4-research-20260906 docs/experiments/results/E06-hvm-boundary-v2.jsonl
python research/chr-hvm/continuation_gate.py /tmp/chr-hvm4-e06 /tmp/chr-hvm4-research-20260906 docs/experiments/results/E06-hvm-boundary-replay.jsonl
```

Each gate has 36 observations: 32 completed positive cases and four adverse
no-return observations at 0.5 seconds. Complete parsed states, outcomes, source
hashes and backend interaction/heap counters replay exactly. Raw stdout/statistics
and each generated source program are retained in JSONL. The two decoder tests
check complete constructor structure and rejection of incomplete/extra results.

Fuel 0/1/2/4/16 advances the encoded Loop, Emit and Choice states as registered.
At fuel 16 the looping continuation returns More(Loop), with 82 interactions and
408 reported heap nodes. The sibling round returns both More(Loop) and
More(Done(A)), with 57 interactions and 286 reported heap nodes. These are tiny
controller states; the numbers do not predict whole-engine costs.

The initial v1 record contains a harness-format mismatch: actual nullary
constructors include empty braces and collapse adds an interaction annotation.
The corrected decoder parses full constructor trees and validates the trailing
annotation separately. It does not approximate state equality, ignore pending
work, or accept arbitrary extra output. All initially completed positive terms
had the expected constructor state; the v2 rerun validates that explicitly.

## Why the boundary matters

The adverse programs put `spin(Loop)` in a More field or under a lambda; spin's
only rule calls itself without a base case. Neither returns within the observation
bound in either mode. A timeout alone does not establish divergence. Here the
recurrence supplies the source-level explanation, and the inspected backend paths
explain why the wrapper cannot protect it:

- `cnf_at` first calls weak normalization and recursively visits lambda bodies
  and constructor children (lines 5776, 5795–5800 and 5849–5858).
- `eval_collapse_process` calls `cnf` synchronously before handling the resulting
  task (line 6164).
- Ordinary normalization visits the term's child locations after weak reduction
  (`eval_normalize_go`, lines 5969–5993).

[Pinned primary source](https://github.com/HigherOrderCO/HVM4/blob/6defdfc7dae2a3cca5dd6e74ed0612385b5646a8/src/hvm.c)

Advance instead returns a finite first-order description of work. Loop is a data
tag, not a suspended recursive backend call. Fuel recursion has a base case and
each toy controller transition terminates, so normalizing the returned finite data
does not execute future source steps. Choice remains a constructor describing a
private source-search boundary; collapse does not turn it into native alternatives.

## Remaining work

Generate the actual source machine, including finite-tree bindings, matching,
residual constraints, source choice births and selected-variable observation.
Compare transition traces and finite answers against independent expectations and
the reference. Measure compilation, continuation construction/normalization,
resumption, full state storage and answer extraction against an equivalent direct
controller. Investigate whether serialization or retained graph boundaries can
avoid rebuilding a backend program per return. All are feasible implementation
questions, not grounds for closing E06b.

The sibling example verifies one finite round only. It does not prove unbounded
search fairness, bounded normalization cost independent of state size, or dynamic
native-label correspondence. E06c and E09 retain those separate obligations.
