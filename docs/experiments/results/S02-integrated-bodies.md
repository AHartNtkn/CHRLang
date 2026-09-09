# Integrated bodies preserve source ordering and create further consuming work

The local executor now prepares and executes source-derived bodies containing constructor equations, fresh variables and posted constraints. Newly posted consuming requests run on the same handles without exporting terms to another body executor. An independent counterexample shows why this integration must preserve the source's body-completion barrier.

The [registered gate](../registrations/S02-integrated-bodies.md) passes **53 package tests**, including 23 constructor/source tests. Its 149 new finite configurations produce **745 complete-answer comparisons** across three local dependency modes and ordinary compiled Scan/Indexed, against the independent scalar evaluator. Three additional fork/owner cases pass. All 36 prior dependency-work records remain identical. Strict scoped Clippy and formatting pass; no timing or allocation comparison ran.

## What the prepared source plan now executes

A plan accepts one simplification rule with removed heads `take(Pattern, Output), token()`. Patterns retain nested constructors and nonbinding repeated-variable matching. Output is a distinct head variable. Bodies support True, Fail, nested conjunctions, finite-tree equations and posted constraints. Alternatives, guards, kept heads, other head shapes and general multiple-rule selection remain outside this gate.

Preparation flattens the body's ordered operations and stores them with the pattern in an immutable shared plan. Queries reuse that plan while building their own handle graph, named outputs and request/token occurrences. A ready application supplies its matched handles and output handle to a private body environment. A body-only variable receives one fresh handle per application; repeated uses within that application share it.

Constructor expressions create descriptors whose children are handles. Equations enter the same consistency queue used by matching. Posted `take/2` and `token/0` occurrences participate in subsequent execution, while other predicate/arity combinations remain observable residual facts. There is no conversion to owned resolved terms between matching and body effects; syntax terms are produced for complete observation.

This removes an execution handoff in the represented source fragment. It does not establish that the new organization costs less. The body environment, source-operation cursor, occurrence handling, residual storage and preparation ownership are additional responsibilities now present in the comparison.

## Body completion changes which consumer wins

The scalar evaluator completes a rule's body before selecting another application. The local executor can update readiness during the body, but it cannot consume another token until all body operations and their equations finish.

The discriminating source has an older suspended request and a ready trigger, with two tokens. The trigger consumes one token and posts a newer ready request. A later equation in that same body enables the older request. With the body barrier preserved, the older request consumes the remaining token and binds the observed winner to `older`.

An exploratory source transformation exposes the remaining body as an ordinary continuation constraint. That allows rule selection between body phases. The newer request consumes the token first, and the independently evaluated winner becomes `newer`. Parameters and fresh variables are carried into the continuation; the altered competition is observable in the complete answer. This rejects that transformation under the current source contract, not all continuation-based execution.

All three integrated dependency modes and ordinary Scan/Indexed preserve the `older` result. Equations still settle before the next body operation, and request order selects the consumer when the completed body has made several requests ready. Different interleaving semantics would require an explicit language comparison rather than an optimization claim.

## The source cases exercise effects and ownership

| Cases | Independent evidence |
|---|---|
| 140 body/query configurations | Seven body forms × five ground/partial input shapes × one/three initial calls × separate/aliased outputs. Complete outputs include named unused variables, input aliases, constructor results, failures and residual multiplicity. |
| Eight chained-source configurations | Depth zero/one/four/twelve over ground or unknown leaves. Each application posts the next request and replenishes a token; the chain runs without host intervention. Residual links preserve each application's fresh variable and repeated aliases. |
| One contested body-order source | Exact older-consumer result in all five execution controls; exposing the continuation yields the contrary newer-consumer result in independent semantics. |
| Three prepared-owner/fork cases | A query keeps its plan alive after the external owner is dropped. A successful query and a fork with a hidden cycle remain independent; the final query disposal releases the plan. |

The 140-case matrix includes output variables aliased to input variables and to other calls' outputs. These cases can force new aliases or an occurs-check failure, so agreement does not rely on every result being a fresh unrelated output. True and Fail bodies, constructor clashes, cyclic equations, duplicate opaque facts and nonmatching `take/3` or `token/1` occurrences remain observable as specified by the source.

Prepared objects are reused across changed queries. After each query is disposed, its plan's strong-owner count returns to the external prepared owner alone. Completed and failed bodies leave no active body cursor. The per-application environment is owned by that cursor and is consumed from the request when the application starts. Graph slots, request records and residual results remain query-owned; this is not a reclamation or RSS result.

## Ordinary execution is correct, but equivalent specialization is still missing

The existing compiled inferred specialization requires one removed head and no kept heads. It rejects `take/2` and `token/0` for all seven body forms: **14 checked eligibility results**. The two-head resource condition is the reason, not the broader body expressions. The current [selector implementation](../../../research/chr-compiled/src/regions.rs) and independent evaluator are unchanged.

Thus ordinary Scan/Indexed supplies correctness controls, but a timing comparison now would mix representation with selection capability. The local plan already knows that selection consists of one patterned occurrence plus a nullary consumable. The compiled generic matcher retains general tuple discovery. That difference needs an equally specialized control before interpreting a performance difference as integration's benefit.

A rejected eligibility report is a limitation of this checker, not a proof that the source cannot be specialized. No new production baseline is needed: investigate a checked two-head selector within the existing compiled prototype, reusing its kernel, source-body execution and occurrence ownership.

## Next selection

**Qualify that two-head consuming selector, then prepare the complete lifecycle comparison.** Infer or check the actual source conditions; do not privilege the fixture's predicate names. Preserve ordered matching, missing-token behavior, output aliases, fresh bodies and the body barrier. Run the same independent body cases and adverse competing consumers, and require evidence that the specialized path actually executes. Broader integration must face this stronger control.

| Strong alternative | Architectural consequence | Selection at this boundary |
|---|---|---|
| Equally specialized selection in the existing compiled engine | Separates source-plan capability from integrated representation in the next cost comparison | Selected because the present eligibility rejection is demonstrated and the new complete-source fixtures are ready. It is a control qualification, not adoption of another baseline. |
| Immediate counter-free lifecycle matrix | Measures preparation, matching, effects, observation, cancellation and disposal | Required after control qualification. Also needs counter-free local builds and allocator/owner gates; current diagnostic duration is not cost evidence. |
| Broader joins, multiple rules and propagation history | Tests substantially more of the integrated architectural proposal | Required. Reconsider after the qualified lifecycle entry; this one-rule source gate cannot settle their scheduling or ownership obligations. |
| Dynamic reunion or broader elimination | Can reduce the work that either executor needs to perform | Required under T077/S06. Include applicable source elimination in the control review and reconsider distinct mechanisms at the next package boundary. |

This is the third T072 package since the reunion boundary and the second after the last four-package breadth review. Source preparation reuse and body execution are now concrete, while general source support, continuous service, full costs, language tradeoffs, coherent architectures and held-out challenges remain unresolved. The research goal stays active.

Evidence: [initial unsupported-body failure](s02-integrated-bodies/red.log), [final tests and ordering/eligibility results](s02-integrated-bodies/tests.log), [audit and source hashes](s02-integrated-bodies/audit.json), [Clippy](s02-integrated-bodies/clippy.log) and [formatting](s02-integrated-bodies/format.log). Scoped Clippy succeeds with existing dependency build-script warnings recorded in its log. Reference-interpreter and independent-evaluator implementations are unchanged.
