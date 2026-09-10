# Continuing service: what obstructed the ownership comparison

The ownership comparison is not ready to rank architectures. Conditional execution exceeds its service budget, while the graph trial needs separate validation and measurement budgets. Its current reclamation control also needs a source on which it actually reclaims state.

These results narrow the next experiments. They do not justify excluding conditional execution, graph execution or reclamation from the architecture investigation.

## What ran

The [ownership registration](../registrations/S08-continuing-ownership.md) specifies nine modes, three answer demands, three consumer retention policies and two queue capacities. The intended comparison has 162 configurations and two allocation runs per configuration. **None of those 324 comparative runs has executed.**

Six of nine largest-demand preflights ran. Four passed: direct continuations, compact live continuations, scanning and specialized scanning. Each delivered 512 independently checked answers, preserved retained answers through producer disposal, and restored the final requested-heap baseline. Conditional execution reached its two-million-service-call limit; the dependency graph reached its 60-second wall limit. The other three preflights have no result.

The source repeatedly chooses between emitting a fresh aliased pair and recurring. It therefore exercises continuing execution with a fixed source and fixed-size answers. It does not cover stable-size changing finite queries, growing finite queries, general resource effects or every ongoing source accepted by the language. Those remain required comparisons.

The four successful preflights have task-owned requested-heap peaks of 280,395, 99,241, 439,455 and 518,097 bytes respectively. These are individual qualification observations with retained-all consumers, not paired estimates or a general memory ranking.

## Conditional execution: equality work needs investigation

**Known optimizations do not resolve this cutoff.** The [registered service diagnostics](../registrations/S08-continuing-service-probe.md) raise the diagnostic limit to 20 million calls while preserving the source. They test existing serial body accounting, precise equality invalidation, support identities and cached support results; reverse support order is a separate control. Optional inferred head dispatch receives its own probe.

| Configuration | Calls by answer 128 | Answers reached at 20 million calls |
|---|---:|---:|
| Initial conditional configuration | 3,198,373 | 240 |
| Existing serial/equality/support improvements | 3,198,239 | 240 |
| Those improvements with reversed support order | 4,606,799 | 211 |
| Those improvements with optional inferred head dispatch | 3,197,719 | 240 |

**Body execution is the largest measured contributor.** A read-only stage diagnostic preserves the improved configuration's exact prefix counts. At answer 128, body execution accounts for 2,242,262 calls, about 70%; observation accounts for 818,043, about 26%. The inferred-dispatch probe separately reports 2,189,877 equality-job ticks. Publication priority alone would leave the larger equality cost unresolved.

**The retained support population grows sharply over the observed prefixes.** It rises from 1,688 nodes at 32 answers to 6,440 at 64 and 25,160 at 128. Variable and occurrence populations grow much more slowly. Source inspection identifies iteration over contextual bindings and associated support operations as a causal hypothesis to test. These observations do not establish an asymptotic bound or show that the cost is intrinsic to contextual equality.

The next discriminating experiment must attribute that binding/support work and test a credible way to avoid it, while preserving answers in overlapping and late-binding cases. A loss after that comparison would have a stronger architectural meaning than merely increasing the service limit again.

## Graph execution: separate the harness limit from retained state

**Both graph configurations reach 512 answers in direct progress probes.** Dependency execution takes 1,536 public service calls and template execution 1,032. Those call counts describe progress within each implementation; their tick bodies do different amounts of work, so the counts are not speed comparisons.

**The requested-allocation meter does not itself prevent this prefix from completing.** A separate metered dependency probe reaches 516 answers, including more lookahead than the ownership queue needs. Live requested heap is 635,881 bytes at 128 answers, 2,301,321 at 256, 8,615,113 at 512 and 8,753,809 at 516. This probe immediately releases consumer answers. Its live readings include process bookkeeping and preparation; they are not peaks, RSS or separated owner measurements. The trajectory warrants investigating retained engine state.

**The full ownership process times out after its validation replay completes.** The phase diagnostic reports `validation-started` and `validation-complete`, then reaches the same 60-second wall limit before final owner disposal. The original timeout had no phase markers. This establishes an obstruction in the combined replay/measurement budget, not that either phase individually requires more than 60 seconds. The next version should give independent validation and measurement explicit separate budgets, preserve both receipts, and qualify every configuration again before comparison.

## Reclamation: this source does not exercise the proposed benefit

**Existing incompatible-support reclamation removes zero result records at every tested graph prefix.** Both dependency and template probes call it after each produced answer and still reach 512 correct answers. This is a valid no-opportunity control. It cannot serve as the favorable reclamation case required by the ownership registration.

The existing [finite reclamation study](S08-reclamation-lifecycle-sizing.md) already exercises useful reclamation. Carry that evidence forward and add a continuing source with genuinely incompatible retained records. Compare reclamation against retention on both sources, then attribute any remaining owners. Do not equate this particular collector with general reachability collection or assume all observed retention is necessary.

## The next selection boundary

**Continue T074 through one bounded qualification package, then conduct the full breadth review before further refinement.** Consumer-pressure qualification, this incomplete ownership trial and service attribution count as three packages after the adaptive review. The qualification package must address separate process budgets and real reclamation opportunity; conditional equality remains an explicit investigation dependency, not an omitted mode or a recorded architectural loss. If it requires a substantial independent repair, reconsider the research order immediately.

Adaptive-search cost qualification remains the strongest ready alternative: its source gate already exists, and measuring policy costs could change the explicit-search recommendation. Resolving the current ownership obstruction first can expose growing history across several candidate paths and make their lifetime comparison credible. Local resource claims, richer structural theories, broader reuse and complete architectures remain required at the breadth review. None depends on a conditional-engine optimization winning.

The [sequence](../sequence.md), [next cycle](../next-cycle.md) and [57-question map](../question-to-experiment-map.md) retain the broader obligations: credible distinctive mechanisms, favorable and adverse sources, total costs, language consequences, complete alternatives and held-out challenges. Completing this entry does not complete the research goal.

## Evidence and validation

The [raw directory](s08-continuing-ownership/) contains the frozen ownership manifest, six preflights, diagnostic progress receipts, build logs and retained failures. The [partial audit](s08-continuing-ownership/partial-audit.json) checks the successful ownership contracts, cutoffs, stage-count correspondence and graph progress. It explicitly records zero comparative runs. Diagnostic inventory hashes are retrospective; the original ownership manifest was frozen before its preflights.

The ownership runner and backend sources still match that manifest. Its original Cargo configuration is preserved in `Cargo.preflight.toml`; the current configuration additionally gates diagnostic examples behind their required features. The metered graph probe now compiles without unrelated conditional diagnostic APIs. Scoped Clippy passes for the ownership and diagnostic examples; the intentional runtime build guard uses a documented constant-assertion lint allowance. Formatting is checked. The independent reference interpreter is unchanged.
