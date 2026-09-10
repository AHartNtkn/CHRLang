# Different callers can share execution while retaining their own outputs

Separating provably inert ground residual data enables reuse across different callers without changing the tested FIFO deliveries. Active resources, pending effects, variable dependencies and propagation history remain in resumable execution. The mechanism now has source evidence; its total costs remain unmeasured.

Here, **inert ground residual data** means a constraint containing no unresolved variables whose name and arity occur in no rule head of the fixed prepared program. It cannot participate in future source execution, but its value and multiplicity still belong in that derivation's answer.

## What the implementation changes

**The candidate separates output ownership from reusable active state.** After each logical transition, the persistent machine extracts eligible constraints using its own source rules. Each FIFO derivation retains the extracted data separately. A cached edge carries only newly extracted data, which is added to the current derivation's own output. Splitting copies that ownership; failure discards it; successful observation reattaches it.

The continuation table still keys active resources, pending goals, bindings reachable through its key, named outputs and live propagation history. It reuses one transition at a time. It does not finish one caller atomically before servicing its sibling. The extraction API checks that its cursor belongs to the machine and does not let a caller declare an arbitrary signature unreadable.

This is source analysis for an experimental implementation, not a new source-language restriction. Groundness is checked after resolving current bindings. Variable-bearing constraints remain active while those bindings can affect their observations. A readable signature remains active even when its arguments are ground; arity is part of the signature.

## Measured source results

**All 72 matrix sources preserve every tested answer and exhaustion step.** The matrix varies six source cases, three work depths, two caller namespaces and same/different tags. Existing Direct, existing AlphaLive whole-state tables, separated execution without memoization and separated memo execution receive identical sources. The three candidates pass 5,712 per-step comparisons against Direct per build. Complete raw-answer multisets independently agree with the unchanged scalar evaluator.

**The candidate obtains reuse where inert tags blocked the whole-state table.** Aggregate transition hits rise from 380 to 562 in the metrics matrix. Representative distinct-caller depth-four observations follow; each row uses offset zero.

| Caller/source case | Whole-state hits | Separated hits | Logical steps, unchanged |
|---|---:|---:|---:|
| Unreadable ground caller tag | 0 | 16 | 39 |
| Caller tag read by a propagation rule | 0 | 0 | 43 |
| Caller variable shared with an output, then bound | 0 | 0 | 45 |
| Unreadable tag sharing a name with a different readable arity | 0 | 16 | 39 |
| Duplicate inert data emitted during recursive execution | 0 | 24 | 55 |

The readable and late-binding controls establish that the successful case is not obtained by indiscriminately dropping caller state. Dynamic duplicate outputs preserve both multiplicity and the current caller's data when cached edges are reused. Early failed branches discard their own detached data; they do not publish it with the surviving branch.

**Active consumption and propagation history also qualify.** Two additional sources combine distinct callers with propagation, a token and optional competing consumption. All four paths preserve complete independent answers and each FIFO delivery. Separation permits reuse in both cases while whole-state tables obtain no hits across the distinct tags. These are bounded source witnesses, not proof for every CHR program.

## Ownership checks and a repaired control

Prepared programs pass 444 cancellation/restart pairs per build across the matrix and the two active-effect sources. Every tested cutoff interrupts an unfinished search. Restarted queries agree with independent complete semantics, and retained answers remain valid after execution and preparation disposal. This establishes finite semantic ownership behavior; it does not measure live heap, RSS or sustained cache growth.

The uncached separated control initially retained an empty node slot for every past transition. A failing test exposed that unnecessary retention before lifecycle comparisons. The control now recycles completed slots, and the regression checks slot capacity against frontier size. The memoized candidate intentionally retains reusable nodes and edges; measuring whether that state earns its retention remains necessary.

The first behavioral test also demonstrates the motivating difference: it fails on the existing whole-state table's zero hits for distinct inert tags, then passes with separated execution and independently correct outputs. The [receipts](s05-inert-residual-gate/) retain these failures and subsequent validation.

## Validation and provenance

The [registration](../registrations/S05-inert-residual-gate.md) predates implementation and source runs. The [runner](../../../research/chr-reuse/experiments/inert_residual_gate.py) builds both configurations, runs each test executable with 60-second wall/CPU and 1 GiB address-space bounds, and freezes source hashes, binaries and receipts. Each full query has a 200,000-logical-step bound. It refuses receipt replacement.

Both default and metrics-off builds pass 42 tests each across the new gate, prior effectful source/boundary gates, generalized continuation tests and persistent-engine regression suites. There are 26 test executable runs in total, including feature-gated executables with zero enabled tests; those do not add semantic coverage. Four strict Clippy checks pass. The [manifest](s05-inert-residual-gate/final/manifest.json) identifies the actual executable targets and commands.

Prior persistent state/continuation sources and the reuse library root are preserved in [source snapshots](s05-inert-residual-gate/before/) with their hashes in the manifest. These resolve the inputs of earlier frozen measurements. The independent scalar evaluator and reference interpreter are unchanged. The new extraction operation is invoked only by the new separated search path.

## What remains necessary before choosing it

**More hits do not establish total efficiency.** The implementation scans live constraints, checks rule heads, exports prospective ground terms, removes eligible entries, retains edge payloads and carries per-derivation output vectors. Key reduction can save execution and still lose once this work and its owners count. Branch splits and retained consumers may make output ownership more expensive than the work saved.

The next package should qualify and measure separate heap ownership for preparation, active table nodes/keys, cached edge data, per-derivation residuals and consumer answers. Include changing queries, immediate/window/all retention, cancellation and no-reuse/readable/variable-bearing controls. Preserve the recycling uncached control and the existing Direct/whole-state competitors. Ordinary-allocator counter-free timing must follow qualified endpoints and a prospective registration, not be inferred from this source gate.

Broader variable-dependent observations and call-local relevance remain required. This candidate leaves those dependencies in active state; that is a tested limitation of this implementation, not their resolution or a claim that they must remain coupled in the language. Cross-query memo reuse, more general effect dependency transport and coherent-architecture costs are also unresolved.

## Why ownership next?

The source gate identifies both saved transitions and new ownership duties in an actual implementation. An owner/allocation comparison can determine whether this representation warrants timing or needs consequential repair. It has greater immediate decision value than adding more successful tag examples.

Adaptive timing attribution remains a ready competing investigation, while native local ownership, conditional equality/lifetime repair and incremental projection retain their independent value. Compare those alternatives at the owner boundary or any obstruction. Three packages have completed since the adaptive breadth review; the next independently counted package must end with a full breadth review. The research goal remains active.
