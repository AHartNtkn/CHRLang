# R03 fork-owner allocation registration (T051)

This diagnostic investigation follows the state-preservation screen and the
validated immutable closed-subtree occurs correction at `8f7c347`. It attributes
actual fork allocation and tests whether continuations would preserve shared
arena ownership. It does not compare architecture timings or estimate a speedup.
The prospective run starts only after implementation, semantic gates, analyzer
checks and this registration are committed.

## Decision and competing explanations

The screen's inclusive split service includes cloning and other work. Arena
nodes, interning keys and predicate dictionaries may dominate requested copying;
alternatively occurrence storage, indexes, scheduling or the inclusive remainder
may dominate. Attribution determines which owner, if any, merits a replacement.

If arena traffic is substantial and failed continuations reach their endpoint
without mutating its proposed shared owners, lookup-before-detachment arena
copy-on-write earns one bounded lifecycle comparison. If continuations insert
nodes or predicates immediately, sharing may merely defer copying. High inherited
hit fractions alone do not select it. Successful and subsequent-split segments
remain separate contrary evidence. Other owners may justify a different narrow
candidate; no candidate is chosen solely because it exists.

This investigation is more valuable now than the ready pure-carrier contraction
experiment because the completed screen identified a large unresolved allocation
cost in ordinary explicit choice. A short diagnostic run can select or reject a
storage intervention; further storage work requires that evidence.

## Configuration, cells and bounds

Use the existing source-validated state-preservation fixture and its inferred
specialized indexed engine. Sizes `n = 0, 64, 512`; alternatives `a = 1, 8, 64`;
outcomes `mostly-fail, all-success`; prepared reuse `q = 1, 4`. Within each process,
query i uses `n + i % 2`. These are 36 cells, each run twice in a fresh process:
72 processes total, shuffled together with Python Random seed `51051`.

Build one locked offline release binary with `--no-default-features --features
alloc-meter,fork-diagnostics`, in a separate target directory. Engine, kernel and
observer work counters are disabled. The allocator measures requested heap
traffic and live requested bytes, not RSS. There is no primary timing build or
warmup. Diagnostic durations are retained only as accounting data.

The process is pinned to the minimum inherited available CPU, limited to 30
seconds and 1 GiB address space. Each query permits 20 million service ticks.
The build has 180 seconds; execution has 600 seconds after the build. Retain every
failed, timed-out, malformed or incomplete result. Do not resize or extend this
matrix in response to favorable results. Sources, registration and binary hashes,
commands, environment and complete planned job order are recorded before runs;
hashes are checked afterward.

## Measurements and controls

Reuse the runner's complete lifecycle intervals: preparation, query construction,
setup, service, first full observation, engine disposal, answer disposal and
prepared disposal. Validate complete answers outside measured intervals using the
independent observation gate. All-success and mostly-fail cases have identical
source construction; the no-choice case must have no clone-owner events and no
post-fork segment activity. Size zero controls state growth, not literal empty
arena ownership; alternative syntax itself occupies space.

At each actual clone, record disjoint owner allocation-call, requested-byte and
deallocation-call differences from read-only cumulative allocator checkpoints.
Never reset peak state inside a measured interval. Reconcile the owner sum plus
an explicit remainder to inclusive split-producing service. Shared rule, region,
dispatch and binding handles supply zero-request controls. Do not interpret owner
sums as inclusive whole-lifecycle savings or report per-owner peaks.

Count actual post-fork inherited/local node hits, node misses and predicate
insertions. Both parent and sibling segments start at the latest fork and end at
a split, failure or completion. Retain failed work and never copy accumulated
prefix counters. Record inherited node count and the node count at first miss,
the number of prior requests before the first miss, and distinguish mutation-free
from mutating segments by endpoint. Preparation is
excluded. These classifications test whether the proposed shared owner would
remain shared through useful branch work.

## Acceptance and interpretation

Require complete expected raw observations, failures and exhaustion in every
cell, exact allocator baseline restoration, owner counts equal split counts,
nonnegative reconciled remainders, zero allocation for shared-handle controls,
and exact repeat agreement for non-time counts and traffic. Segment accounting
must include both fork descendants without duplicating the initial prefix, and
must distinguish predicate insertion from node insertion. Investigate any
reconciliation or repeat discrepancy before interpreting evidence.

Report all cells, owner shares and continuation classifications. A substantial
owner is one contributing at least 25% of inclusive split requested bytes in a
complete n512, a>1 mostly-fail cell; this is a screening threshold, not a benefit
claim. Consider the same-owner n0 growth and all-success controls. A storage
candidate additionally needs a concrete path to avoiding those copies under the
observed continuation behavior. A subsequent comparison must charge detachment,
mutation, observation and disposal and include an ordinary-allocator counter-free
baseline at the then-current source. Compilation costs remain outside a complete
architectural superiority claim. No workload weighting or universal winner is
inferred.
