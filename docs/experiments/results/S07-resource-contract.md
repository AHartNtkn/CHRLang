# Checked declarations enforce a boundary without promising counting eligibility

The experimental declaration path now checks resource access when preparing a rule set and ground inputs when starting a search. Optional and required admission enforce the same facts when a declaration is supplied. Requiring a declaration changes admission; this implementation supplies no evidence that it removes the remaining CHR executor.

This is the first executable contract gate for T079. It builds on the [semantic premise audit](S07-resource-premises.md) and uses the existing prepared engine. It introduces no alternative baseline or reference-interpreter dependency.

## The exact contract being compared

A declaration names a resource predicate by name and arity, the rule indices allowed to read, consume or produce it, an entry predicate, and one argument that must be ground in every initial occurrence of that entry. Resource access outside the declared region rejects preparation. Non-ground input rejects search admission.

The region is explicit. It is not inferred ownership, a linearity proof, or a guarantee that there is only one consumer. Initial resource occurrences remain allowed. The declaration does not prohibit other public predicates. The tested declaration limits `fuel/0` access to the traversal step and requires argument zero of `start/3` to be ground.

Groundness applies at query submission. It does not constrain fresh results or entries subsequently produced by rule bodies. No initial entry satisfies this universal property vacuously, and multiple ground entries satisfy it. Resource sufficiency, the traversal's constructor grammar and single-entry cardinality remain separate optimizer checks.

| Admission choice | No declaration | Invalid declaration | Valid declaration with non-ground entry | Valid ground query outside counting eligibility |
|---|---|---|---|---|
| Ordinary inferred optimization, existing path | Ordinary execution remains available when inference or lowering declines | No declaration interface | Ordinary execution remains available | Ordinary execution remains available |
| Optional checked declaration | Uses ordinary prepared execution | Reject preparation | Reject this search | Uses ordinary execution |
| Required checked declaration at this interface | Reject preparation | Reject preparation | Reject this search | Uses ordinary execution |

The last row is a bounded policy for the nominated interface, not a complete language-wide ownership or mode system. Requiring groundness at every internal call or requiring every predicate to have a private region would be different restrictions needing their own comparison.

## What the implementation enforces

[CheckedSource](../../../research/chr-compiled/src/resource_contract.rs) checks the complete source and borrows that exact immutable rule slice. It validates declaration targets and region indices, checks both kept and removed heads, and traverses conjunctions and both choice arms for resource-producing posts. Predicates with the same name at a different arity remain distinct.

`PreparedContract` owns the prepared source and declaration. It checks a declaration before preparation and validates the actual owned query immediately before creating its search. Prepared queries are not authorized by a reusable Boolean from an earlier query check. Subsequent searches through the same prepared contract are checked separately. The ordinary public engine remains available for the explicitly unrestricted experimental path.

A linked extension is a new source to check. Tests add both an observing rule and a resource producer hidden in a choice/conjunction; each invalidates the previous access declaration for the extended source. The original checked source remains usable with its original rules. Separate compilation, stable exported region identities and incremental link checking are not implemented: region indices deliberately refer to a complete rule set.

## Executable distinctions

The added tests establish the following behaviors with complete observations, not only acceptance labels:

- Ground queries with insufficient fuel pass the declaration, fail counting eligibility and return the expected suspended traversal and unbound result.
- A ground depth using a different constructor passes groundness, fails counting eligibility and preserves its suspended term and fuel.
- A certified independent-choice source with only enough common fuel retains one completed and three suspended branches. Both original and counted executions preserve the exact branch results and residuals.
- Optional and required prepared contracts reuse their source across depths zero, one and four and produce the expected answers. Both reject a subsequent unknown-depth query. Optional admission without a declaration executes that query and preserves its unknowns and suspended residuals.

Admission checks also cover invalid target names, invalid argument positions, empty or out-of-range access regions, nested unknowns, multiple entries and no entry. The earlier harmless-reader, independent-effect, late-ground reformulation and observable-depth counterexamples continue to pass.

## Responsibilities and architectural consequences

| Responsibility | What this gate establishes | What it does not establish |
|---|---|---|
| Source checking | Explicit access declarations can be checked without discovering a counting transformation. Complete rule-set extensions must respect the declared region. | Automatic region inference, precise effect commutation or incremental checking. |
| Input validation | Groundness can be enforced at an owned submission boundary while results remain logical unknowns. | Free validation of external inputs, or a proof that internally generated calls are ground. |
| Optimization analysis | Counting remains a separate analysis with stronger premises. Valid declarations do not imply eligibility or profitability. | That source annotations eliminate counting analysis or permit more lowering. |
| Matching and consumption | Accepted queries still exhibit resource-sensitive suspension and branch-specific execution. The existing engine supplies those services. | That every alternative compiler for this contract must retain the same engine organization. |
| History, choices and observation | These remain in the source contract and complete answers; declaration checking does not remove them. | A language or compiler that eliminates these responsibilities without changing accepted behavior. |
| Prepared ownership | The checked source and each submitted query are tied to the execution boundary. | Sustained lifetime costs, binary-size effects or compilation savings. |

Optional and required admission share the exact declaration-present implementation: the required flag is inspected only when a declaration is absent, and is not retained during execution. Consequently this implementation provides no separate execution mechanism to benchmark for those two admitted cases. That is a code-path equivalence, not an empirical claim that all optional and mandatory language designs cost the same.

Required admission could improve predictability about accepted inputs, but the present facts do not guarantee completion or counting. Claims of broader simplification require a stronger property or a compiler that actually exploits the existing one. The next cost experiment must charge checking and preparation rather than crediting unimplemented savings.

## Validation and next decision

Nine property tests and four counting-boundary tests pass in both default and counter-free builds. Full-answer checks use the independent scalar semantics and scanned/indexed candidates; the admission test also exercises the owned prepared boundary directly. Strict counter-free Clippy passes. [Default](s07-resource-contract/default.log), [counter-free](s07-resource-contract/counter-free.log), [Clippy](s07-resource-contract/clippy.log), [tests](../../../research/chr-compiled/tests/resource_properties.rs).

The tests concern finite observations under the existing global schedule, not universal equivalence, cancellation latency or timing. No comparative cost run was performed. The declaration implementation clones its small declaration during source validation and retains it with the prepared rules; those actual allocations must count in a cost trial.

At this gate boundary, selective conditional discovery is still the strongest ready alternative: it could reduce the measured branch-specific costs that counting leaves behind. A bounded declaration-cost pilot comes first because the source and admission paths now exist, and charging their checks completes the immediately testable contract comparison. Compare inferred counting with independently checked declarations plus counting, include shallow/deep inputs, changing queries and invalid submissions, and avoid duplicating optional/required declaration-present timings. Register exact bounds and repetitions before running. Reconsider discovery after that pilot; broader language properties and sustained lifetime remain required.
