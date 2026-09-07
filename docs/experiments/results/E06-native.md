# E06c: native choice outcomes and label provenance

The pinned runtime confirms concrete obligations for native integration. Copy
labels can select rather than copy, coexisting births can become correlated when
labels collide, administrative superpositions are not source choice births, and
disconnected failure can disappear. These probes identify faulty translations and
finite repairs; they do not establish a general native CHR compiler.

## Reproduction and validation

```
python research/chr-hvm/native/build.py /tmp/chr-hvm4-research-20260906 /tmp/chr-hvm-native-e06
PYTHONDONTWRITEBYTECODE=1 python research/chr-hvm/native/run.py /tmp/chr-hvm-native-e06 docs/experiments/results/E06-native-v2.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-hvm/native/run.py /tmp/chr-hvm-native-e06 docs/experiments/results/E06-native-replay.jsonl
```

[Build manifest](E06-native-build.json) records the pinned clean source, compiler,
source hashes and exact instrumentation insertions. The unmodified baseline and
instrumented executable have identical answer multisets on finite probes. Each
of 21 probes runs both collapse executables; 19 also run ordinary -D graph tracing.
That is 61 executions per gate: 59 finite returns and two adverse no-return
observations at 0.5 seconds. Observations, numeric events, graph traces and backend
interaction/heap counters replay exactly. Timing is not compared.

Thirteen probe encodings match their declared observation sets, seven deliberately
faulty encodings do not, and the divergent-first case has no completed observation.
All measured runtime predictions hold. These counts are not a claim that seven
source failures have been accepted as correct: they are recorded counterexamples
to the tested mappings. Expected sets are independently enumerated correlated or
Cartesian data observations, not output copied from the backend.

The initial v1 generator used `*` for failure. Inspection of the pinned parser
shows that it constructs ANY, whereas `&{}` constructs ERA. The decoder correctly
rejected wildcard-containing output. The v2 generator uses actual erasure and
retains the original failure obligations. Both records remain available; wildcard
output is not treated as an acceptable first-order result.

## Results that distinguish mappings

| Probe | Intended observation | Measured native behavior |
|---|---|---|
| Copy choice 1 using label 1000 | (A,A), (B,B) | Matches |
| Copy choice 1 using label 1 | (A,A), (B,B) | Only (A,B) |
| Two births using labels 1 and 2 | Four pairs | Matches |
| Two births reusing label 1 | Four pairs | Only equal pairs |
| Birth labels 1 and 16,777,217 | Four pairs | Only equal pairs; both effective labels are 1 |
| Copy label 16,777,217 on choice 1 | Correlated copies | Only (A,B); effective copy label is 1 |
| Label reuse after passing a choice through a function | Four sequential-choice pairs | Only equal pairs |
| Two depth-two recursive calls, overlapping label ranges | Sixteen pairs of lists | Four correlated pairs |
| Same recursive calls, disjoint ranges | Sixteen pairs of lists | Matches |
| Unused failing computation plus output A | No answer | A is returned |
| Explicit ERA joined into result structure | No answer | No result |
| Branch filter with explicit ERA for rejected branch | Only (A,K) | Matches |

Recursive depth 1/2/3 probes with distinct coexisting labels return 2/4/8 complete
lists. The duplicate-answer probe retains two equal raw results; final exact
deduplication remains an observer responsibility in this experiment.

The finite range allocation is generated for these bounded calls. It is not a
runtime freshness allocator, a reclamation proof, or a proposed fixed depth limit
for the language. The sequential reuse counterexample demonstrates why a source
operation having fired is insufficient evidence that its backend label is dead.

## Why provenance requires more than label numbers

The lambda-copy control has no DSU source primitive, yet its trace records
DUP-LAM introducing a superposition with label 1000. The graph trace shows that
administrative superposition in the duplicated binder before subsequent reduction
eliminates it. It is therefore false that every observed SUP is a source OR birth.

The guarded-branch probe is stronger: it records source DSU label 1 and an
administrative DUP-LAM superposition also labelled 1. The final observation is
correct. An invariant merely partitioning numeric labels into permanently
source-only and administration-only namespaces would not describe this runtime
execution. The compiler needs role/lineage information for uses of labels and
must distinguish copying, restriction and fresh source birth.

Same-label DUP-SUP events select opposite alternatives; distinct-label events
commute copying through a choice. Dynamic input labels are masked to 24 bits.
The explicit collision probes show requested labels 1 and 16,777,217 becoming the
same effective label, avoiding an unnecessary millions-of-births stress run.
These observations agree with the inspected `wnf_dup_lam`, `wnf_dup_sup`,
`wnf_dsu_num` and term packing paths in the
[pinned primary source](https://github.com/HigherOrderCO/HVM4/blob/6defdfc7dae2a3cca5dd6e74ed0612385b5646a8/src/hvm.c).

Instrumentation logs primitive events, not a proved one-to-one source birth
ledger. Generated programs declare their intended birth/copy labels; repeated
primitive execution under different supports must still be explained by a future
compiler invariant. Ordinary -D traces cannot be combined with -C on this revision;
collapse events and ordinary graph traces are distinct executions and are reported
as such.

## Failure and scheduling boundaries

Erasure is a usable local result protocol when explicitly joined to the result,
but pure dead-code elimination can discard a disconnected failing computation.
That is not the source CHR meaning of an active failing constraint. A complete
translation must retain and discharge every active obligation and residual
constraint before publishing selected outputs. The tiny branch filter is not yet
that store/history mechanism, and joining inert constraint data alone would not
execute source rules or establish quiescence.

With a divergent first alternative and finite second alternative, collapse -C1
does not return within the bound. Reversing them returns the finite result. Spin's
only transition calls itself, and the previously inspected collapse worker calls
normalization synchronously. Together these support the starvation counterexample;
a timeout alone would be insufficient. Native collapse is not the intended finite
service scheduler for this encoding.

The data-continuation variant returns both More(Loop) and Done(A). More is private
protocol data, not a trusted user answer. This validates a finite boundary only;
resuming supported source states and ensuring global quiescence still require an
actual engine/scheduler interface.

## Open, feasible investigations

Implement a birth allocator with explicit live-scope accounting; test retained
DUPs, closures and supported states before reusing physical labels. Investigate
capacity handling without silently wrapping labels or restricting the source to
bounded search. Derive a compiler invariant that follows administrative binder
superpositions and source restrictions, rather than classifying by numeric label.

Integrate branch-local unification, occurrence membership, propagation histories,
and off-output failure into a finite native-state prototype. Compare projections
against independent reference executions, including nonground outputs. Implement
resumption/collapse yields and record source-step/label traces on recursive
applications. These are feasible implementation questions, not external blockers.

The general encoded machine and ordinary/richer net protocols remain independent
alternatives. This evidence rejects particular transparent mappings, not the
possibility of native integration. No language contract or production architecture
has been adopted.
