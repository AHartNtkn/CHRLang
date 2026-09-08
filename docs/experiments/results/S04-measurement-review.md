# S04 measurement review while the frozen pilot runs

> This records an earlier gate or checkpoint. The [complete lifecycle report](S04-lifecycle-pilot.md) and [matcher correction gate](S04-matcher-copy-gate.md) give the subsequent evidence.

The total-lifecycle endpoint includes the costs needed to compare these implementations. Individual execution phases do not have identical setup boundaries. A matcher cost in the restoration implementation also warrants a causal check before interpreting an architectural loss.

This is a source and measurement review, not the final pilot result. The matrix continues using its frozen binaries and registration.

## Phase boundaries and ownership

The restoration `Prepared::start` populates its live source store during setup. The existing indexed engine's `start_search` lowers initial constraints into pending insertion work; execution then posts them. Thus an execution-only ratio would mix initial-store construction with runtime work differently. The primary endpoint sums preparation, setup, joint execution/observation and disposal on both sides, so it includes both organizations' costs.

Both prepared-rule owners are immutable across queries. Restoration shares source rules through an `Arc`; existing indexed execution shares prepared rules, dispatch and predicate metadata. Query state is separate. The additional cancellation query therefore does not mutate a reusable query cache in either implementation. Its costs are reported separately, with prepared-rule disposal charged once after that last use.

The first-observation clock measures when a full answer is returned to the runner. This includes terminal-state disposal performed before that return. It is not a measurement of only the final term export, and it must not be added to total runtime a second time.

The allocation meter restarts its peak for every phase. Subtracting the pre-preparation live baseline from the maximum measured phase peak reports incremental requested heap for runtime ownership and retained answers. It excludes temporary independent-oracle/validation allocations outside those phases. It is not RSS or whole-process peak memory. Exact end-of-lifecycle restoration is checked separately.

The audit script now checks exact commands against frozen binary/configuration assignments, one occurrence of every registered repetition, primary versus diagnostic memory fields and the first-observation interval bounds. Its final summary must wait for all registered receipts; partial rows do not establish completion.

## A concrete matcher responsibility to investigate

In `State::tuple`, after predicate/arity checks, the restoration matcher clones the complete slot environment for each candidate partner. Only afterward does it test whether that partner agrees with values established by previous heads. In the mutation source, the first head binds the cell key, remaining edit list and continuation. Wrong-key cells can therefore cause copies of the list and continuation before being rejected.

This is a concrete implementation operation, independent of copying versus trailing versus replay. Replay also repeats that operation while reconstructing prefixes. The inspected indexed control instead represents bound slots with internal term identities and uses indexed partner access. Its cost difference cannot automatically be attributed to state restoration.

A causal experiment should count rejected-partner environment copies, then test a conservative precheck of bindings already established by previous heads. Clone the environment only after that precheck passes; retain the full nonbinding matcher as the final authority. A precheck must not infer a new binding or reject a candidate based on an unbound rule variable.

The correctness obligation is one-sided: whenever the full matcher can accept a candidate under the entry environment, the precheck must also accept it. Existing slot bindings are not overwritten during matching; new bindings impose additional constraints. A check consulting only entry bindings can therefore reject their mismatches without inventing information about new variables. Constructor paths and repeated variables still need executable adversarial tests.

The contrary control must include compatible partners, including large equal bound structures, where a precheck may duplicate useful traversal and save no copies. Measure this overhead as well as selective rejection. The diagnosis must distinguish fewer environment copies from fewer candidate visits; a precheck need not implement an index.

## What would justify the next work

If the completed pilot and diagnostic counts show this operation materially drives a loss, its small semantic surface makes a paired correction valuable before ranking the state organizations. Preserve the original binaries and source freeze, gate the correction independently and register a bounded paired comparison. If it saves little or adds comparable checking cost, retain that contrary evidence rather than assuming indexing will rescue the candidate.

S05 stable-identity operation/failure reuse remains the strongest ready architectural alternative. This matcher investigation takes priority only if the current loss leaves the restoration comparison materially confounded. Other possible improvements—working-state reuse between consecutive services, different service quanta, richer checkpoint placement and large immutable term payloads—remain distinct investigations. The current cell-count sweep does not settle all of them.
