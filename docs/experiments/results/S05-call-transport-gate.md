# Call-level reuse can cross caller differences, with an explicit entry boundary

A call table now reuses computation across callers whose complete states differ, while preserving fresh result identities and complete raw answers. The whole-state table gets no reuse hit in the corresponding witness. A scheduling counterexample also shows that a private call family is insufficient permission to move a call ahead of other source work.

## The mechanism and its scope

The [call table](../../../research/chr-reuse/src/calls.rs) prepares a fixed private rule family once. Each rule must simplify one family constraint; posted constraints must stay within the family. Equations, explicit alternatives and failure are supported. Multihead resource access, kept heads and calls outside the family are rejected. The family check is a syntactic prerequisite, not a proof that arbitrary caller work can be projected away.

The caller must supply arguments resolved in its current environment. The key contains the selected predicate and all argument terms, with consistent variable renaming. Repeated variables remain repeated; different aliases, constructors, bindings and arities remain distinct. Caller-only constraints and output names are absent from the key. The prepared family owns the cache, so results cannot cross into another ruleset through this API.

A miss runs the isolated call using the persistent execution machine. Every interface variable is observed, not only a nominated return value. All successful alternatives must complete without residual source work before the result is cached. Suspension and service cutoff return distinct errors and create no cache entry. Complete failure can be cached as no successful alternatives.

Replay maps interface variables back to the caller and allocates every internal result variable freshly for each replayed alternative. A single map preserves aliases within each result. The allocator starts beyond variables in the entire caller query, including unrelated constraints and named outputs; it must remain the caller's allocator as more fresh work is added. The table returns equations for application by the caller, not a replacement caller state.

The prototype is an explicit call-boundary API. The tests apply replay equations through ordinary CHR rules and compare complete caller answers with independent scalar source execution. It does not yet locate safe call boundaries inside an arbitrary continuation, transport resource effects, or prove general relevance projection.

## Independent evidence

| Question | Experiment and result |
|---|---|
| Can caller-only state prevent whole-state reuse while calls remain reusable? | Two source alternatives retain different `caller(a, U)` and `caller(b, U)` constraints but perform the same deterministic eight-step recursive call. CompactLive completes with zero table hits. The call table computes once and replays once; both complete caller answers agree independently. |
| Is substantive call work avoided? | The matched isolated-call control executes 40 source transitions for the two calls; memoization executes 20. The complete whole-state path executes 47 transitions. Caller construction, replay and observation are outside the isolated-call count, so 47 versus 20 is not a total-efficiency ratio. |
| Are fresh results independent across replays? | Two calls in one caller each return a term containing a fresh variable twice. Replays preserve each internal alias without identifying the fresh variables across calls or with an unrelated high-numbered caller variable. Four raw alternatives remain four. |
| Do keys preserve relevant distinctions? | 216 executions vary Direct/Memo, three variable offsets, three recursion depths, six input/alias shapes and two caller tags. Complete outputs, aliases, residuals and raw multiplicity agree with independent source semantics, including the cyclic result/input case that fails. |
| Can unsupported access or unfinished work become success? | Multihead consumption, kept facts and calls outside the private family are rejected. An unknown recursive input reports suspension; a one-step limit reports cutoff. A subsequent sufficiently bounded call computes successfully without a hit from either incomplete attempt. |

The [tests](../../../research/chr-reuse/tests/call_reuse.rs) compare the candidate's persistent-machine execution with an independent owned-syntax evaluator using recursive substitution. The expected evaluator does not use this table or its renaming implementation. Direct isolated execution and whole-state reuse remain separate controls because they perform different amounts of caller work.

## Why call entry is a semantic obligation

Consider a family whose first rule returns `known` for input `f(a)` and whose fallback returns `unknown` for an unknown input. An earlier-priority caller rule supplies `f(a)` before the ordinary source selects the call. Ordinary execution therefore returns `known`.

Expanding the private call before that supply commits to the fallback and returns `unknown`. Applying the same later supply does not undo that committed choice. The executable counterexample asserts these distinct outputs and their inequality. There is no resource access outside the family, so the family check alone cannot establish commutation with caller work.

The favorable tests use a stated call-entry arrangement with complete independent answers. They do not authorize this speculative early extraction. An integrated optimizer must either invoke reuse at an already selected source call, or establish a sufficient independence/commutation condition for moving it. A different permitted scheduling policy must remain an explicit architectural/language comparison, never an unnoticed change in a timing control.

## Validation and limitations

The full reuse package passes 60 tests. All six call tests pass with metrics on and off; strict scoped Clippy passes. The final call target was rerun after strengthening the direct-control observation check. [Receipts](s05-call-transport-gate/) preserve the original unimplemented failure, package run, final call diagnostics, counter-free semantic run and lint result. Reference-interpreter source is unchanged.

Timing has not run. Metrics updates are feature-gated, but counter-free semantics alone do not establish timing isolation. Keys and cached terms are owned allocations; computation exports complete results; replay clones and renames them. These costs, cache retention/eviction, prepared-family lifetime and cancellation must be measured separately from source-transition savings. The bounded call API currently computes its successful alternatives before returning them; streaming and interrupted replay remain open.

## Next investigation and competing priorities

Continue T075 with integration at a source-valid call boundary and sharper checks of what caller state can affect that boundary. The new scheduling counterexample makes this more valuable than immediately timing the current API. Include shared live resources and history as negative cases, and identify which genuinely isolated calls a conservative admission rule misses. Broader effectful reuse is required work; rejecting it in this initial family checker does not resolve it.

Precise S02 subscriptions are the strongest ready local refinement: their broad alias overhead has an exact witness and independent matching checks. Structural solving and restoration/reconnection remain distinct alternatives. Reconsider them after the call-entry gate; do not extend cache key tuning automatically. This is the first package since the integrated breadth review. T075 and the overall architecture goal remain active.
