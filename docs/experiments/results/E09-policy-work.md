# E09 first integrated policy results

All 930 policy/quantum configurations over the 62 exhausted E00 cases pass, with
byte-identical replay. They preserve exact full-residual observations and raw
multiplicity against hand expectations and the independent Rust reference. The two
nonexhausted reference prefixes are outside that exhaustive gate: different fair
schedulers can return different prefixes. The controlled recursive-sibling probes
instead check an independently known finite answer while search remains live.

The registered four-family matrix has 180 passing configurations and exact replay.
Eleven source, observer and scheduler tests pass. Candidate evaluation imports
neither reference interpreter. The comparison fixes source rule/occurrence policy;
only search-service scheduling and exact-state grouping vary.

## Findings

At service quantum 8:

| Workload | Policy | Grouping | Total actions | Source jobs | First answer action |
|---|---|---|---:|---:|---:|
| Large equation, depth 64 | FIFO | off | 1,195 | 11 | 633 |
| Large equation, depth 64 | round | off | 1,187 | 11 | 1,172 |
| Large equation, depth 64 | async | off | 1,193 | 11 | 622 |
| 6 duplicate choices | round | off | 4,627 | 382 | 3,471 |
| 6 duplicate choices | round | on | 829 | 22 | 829 |
| 6 duplicate choices | async | on | 829 | 22 | 829 |
| Different outputs, opaque depth 64 work | round | off | 6,659 | 16 | 6,644 |
| Different outputs, opaque depth 64 work | round | on | 10,304 | 16 | 10,289 |
| Different outputs, opaque depth 64 work | async | off | 6,669 | 16 | 6,654 |
| Different outputs, opaque depth 64 work | async | on | 10,224 | 16 | 10,209 |

The small `a` answer is first in each large-equation run. Without grouping, source
work is identical (987 actions); the round barrier delays its next source step
behind the unrelated large equality. Async releases it earlier without reducing
the total source work. This supports the latency-isolation mechanism under this
service, not a universal response-time claim.

Duplicate-choice grouping executes 22 source jobs representing 382 source
transitions. It preserves all 64 raw alternatives and publishes one exact unique
answer. Source work falls from 1,856 to 152 actions. Persistent arm/union supports
encode paths without enumerating those 64 leaves. Peak queued source/group jobs
is one in the grouped case; this does not establish byte usage or a general
compact representation guarantee.

Different output bindings prevent complete-state equality even when subsequent
operations are identical and independent. All variants perform 5,827 source actions
and 16 source jobs. Grouping therefore adds work without sharing those operations.
The implementation's tuple comparator visits later tuple elements first, which
can inspect a large common pending suffix before finding an earlier difference.
Traversal order, discriminating metadata and cached keys are concrete next
ablations; this overhead is not an unavoidable cost of grouping. Complete-state
keys still cannot share different states regardless of comparison speed. Operation
or conditional-family sharing remains independently relevant to the project.

All recursive probes return the trusted finite sibling answer and retain live
recursive work. For depth 64, quantum 8, publication takes 104–133 actions across
variants. This finite prefix does not measure long-running queue growth or prove
fairness by itself.

## Ownership and service boundary

Each state/support entry has one owner. A sealed batch groups at most eight ready
entries; existing jobs never absorb arriving work. Exact complete-state equality
lets one source step serve a disjoint union of paths. OR appends separate bits;
continuations preserve support, and failure retires it. Round successors wait for
the finite evaluator round; completed observations need not wait. Async successors
enter at the tail. A serial observer alternates service with evaluation and admits
one exact comparison sequence at a time, preventing duplicate-acceptance races.

Under finite immutable state/service, finite predecessors, available storage and
a draining observer, each admitted job finishes with FIFO service. Round finiteness
then gives next-round progress; async successor admission gives progress by
induction along a finite derivation. This is relative to the fixed source selector.
Arbitrary host allocation, hashing, integer arithmetic and collection finalization
are not hard real-time primitives.

## Evidence and limits

- [Conformance](E09-policy-gate.jsonl), [replay](E09-policy-gate-replay.jsonl).
- [Controlled work](E09-policy-work.jsonl), [replay](E09-policy-work-replay.jsonl).
- [Source/registration hashes and host](E09-policy-manifest.json).

Reproduce:

```sh
cargo run -q -p chr-symbolic-fixtures --example export_cases > /tmp/chr-e09-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python -m unittest discover -s research/chr-scheduling -v
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/check_scheduler.py < /tmp/chr-e09-cases.jsonl
PYTHONDONTWRITEBYTECODE=1 python research/chr-scheduling/run_probes.py
```

Counters include group comparisons, source work, observer work and admission/
commit/dispatch events. A resume's quantum bounds service actions; extra bounded
batch bookkeeping is separately counted. Query preparation, Python instrumentation
and allocation/collection costs are not wall-time or byte measurements here.
No native performance or production-policy selection follows.

Required follow-ups remain: comparator/key ablations with early/late mismatches;
charged preparation and measured storage/runtime; large tuple scans and wake-ups
under mixed load; growing synthesis prefixes; support fragmentation/subdivision
inside heterogeneous families; second-service/net integration. These are feasible
investigations and the E09 direction remains open.
