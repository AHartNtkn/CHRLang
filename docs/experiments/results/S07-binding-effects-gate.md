# An effect certificate removes equality wake-up bookkeeping without requiring groundness or non-overlap

A ruleset whose variable bindings cannot change does not need a persistent index for waking old matches after equality updates. The experimental certificate now removes that index and its per-dependency service calls. It accepts shared unknowns and consuming rules; it does not claim their applications commute.

## The actual beneficiary

The conditional engine normally copies each match's variable dependencies into a persistent map from variables to candidate tuples. Later binding changes consult this map to reactivate matching. If the immutable prepared ruleset cannot write a binding, those changes cannot occur.

The `effect-contract` feature adds a checked source property and omits persistent dependency registration for certified rulesets. It composes with the existing unary-head dispatch plan and serial body accounting. The feature is off by default; ordinary preparation does not infer the property automatically. A caller explicitly selects inference, a checked declaration or required declaration through the prepared ruleset.

The implementation still builds the match job's temporary dependency information. It preserves candidate discovery for new occurrences, consumption supports, propagation history, failure filtering, body scheduling and complete observation. Thus the demonstrated simplification is specific: persistent equality wake-up indexing and its drain service disappear. It is not a general removal of matching, conflict handling or scheduling.

## Why the certificate is sound within its stated contract

The prepared source is immutable and includes every rule that can run. The checker recursively examines every body, including both choice arms and every conjunct, and certifies only when there is no body equation. In this engine, matching and positive equality guards read bindings; fresh variables and constructors allocate identities without assigning existing variables. Failure changes failed support, not variable bindings. Consequently there is no binding change that could require the omitted wake-up index.

This argument depends on the current source language and closed prepared-rule interface. It would need revision for external binding updates, dynamically installed rules or new binding operations. Failure and consumption still change which contexts and occurrences are live; the optimization retains their ordinary checks.

A body may post a new constraint containing shared variables. Its new candidate tuples still enter through ordinary discovery, so useful new linking constraints remain effective without an equality notification. The source gate tests both shared and distinct-variable linking cases.

## The measured mechanism

With unary dispatch and serial accounting enabled, use one rule that requires `p(a)` and supply permanently unknown `p(X)` occurrences. Both engines return the same complete residuals; the ordinary engine records dependencies that can never fire.

| Unknown occurrences | Control service calls | Certified service calls | Control retained dependencies | Certified retained dependencies |
|---:|---:|---:|---:|---:|
| 1 | 50 | 49 | 1 | 0 |
| 4 | 146 | 142 | 4 | 0 |
| 16 | 530 | 514 | 16 | 0 |
| 64 | 2,066 | 2,002 | 64 | 0 |

One drain service call per dependency disappears. These are deterministic mechanism counts, not equal-cost instruction units or timing measurements. The saved index entries establish a real owner/operation difference; requested bytes, checking costs and lifecycle payback have not yet been measured.

## Language distinctions and adverse checks

| Case | Certificate behavior and independently checked result |
|---|---|
| Shared unknown inputs | Accepted. Full output aliases and residual variable relationships are preserved; groundness is unnecessary. |
| Positive equality guards | Accepted when bodies have no equations. Once-per-tuple propagation and duplicate residual multiplicity remain correct. |
| Choices and explicit failure | Accepted. Complete raw alternatives and failure results match independent expectations; failure is not treated as a binding update. |
| Kept-head interference | Both source orders are accepted and retain their different results: `p \\ q` can interfere with consuming `p`. No commutation conclusion follows from immutable bindings. |
| New linking constraints | Accepted. Posting `q(X)` can enable a tuple with an existing `p(X)`; a distinct unknown does not satisfy the equality guard. Discovery remains necessary. |
| Actual late binder | Uncertified under inference; ordinary dependencies remain and the later equation activates the blocked match. Checked/required no-binding admission rejects the writer. |
| Nested, unreachable or reflexive body equations | Conservatively uncertified, including an equation after failure and `X=X`. These illustrate checker false negatives; a rejection is not proof that a binding write will occur. |
| Continuing recursion beside a finite sibling | Accepted witness publishes the finite answer within its service bound. This is a progress gate, not a general throughput result. |

Inferred, checked and required successful admissions produce the same source-application traces and answers in the finite acceptance suite. Required admission also rejects an absent declaration. A declaration earns no extra runtime shortcut beyond the same verified property. No mandatory language restriction is adopted.

## Validation and reproducibility

The first test run failed because the effect API and dependency-retention operation were absent. The implementation then passes the beneficiary test and the adverse source gates. Complete observations are checked against hand-written expectations and the independent owned-syntax scalar evaluator; accepted finite cases also retain the ordinary conditional source-application trace. Accepted runs check that the store records no binding changes.

The full feature-enabled package passes 169 tests; the default counter-free package passes 136. The focused counter-free effect suite passes eight tests, including the stronger unary/serial control. Scoped Clippy and package formatting pass. [Logs and hashes](s07-binding-effects-gate/), the [source gate](../../../research/chr-direct-conditional/tests/effect_contract.rs), and [implementation](../../../research/chr-direct-conditional/src/engine.rs) provide the evidence. The reference interpreter is unchanged.

Frozen copies preserve the measured engine and manifest for preceding experiments. Their current order/lifecycle auditors verify the original hashes through explicit snapshot mappings, so this experimental feature does not change the basis of those results.

## Next: measure admission and owner costs against a credible control

This gate supplies T079 with an additional executable beneficiary beyond unary dispatch and serial accounting. A bounded lifecycle comparison is now justified: feature-off control, feature-on ordinary preparation, inference, checked declaration and required declaration. Include useful blocked/read-heavy matching, substantive constructor rewriting, a tiny overhead case and a real writer that inference cannot certify. Charge source checking, preparation, changed queries, retained answers, cancellation and disposal. Confirm that declared and inferred admissions obtain the same runtime owner benefit before comparing their language burden.

The strongest alternative is direct source-derived solving of the choice/check source. That still needs an eligibility and effect argument for eliminating alternatives while preserving consuming or externally observed behavior. The present certificate does not supply that stronger argument: those sources contain binding equations and are outside this deliberately conservative proof. Its inexpensive lifecycle comparison can decide whether even the demonstrated bookkeeping removal earns its checker before expanding the analysis. Reconsider the direct-solving proposal and broader non-overlap/ownership beneficiaries after that comparison; they remain required.

T079 remains active. This is the third package since the order-lifecycle breadth review, after stronger-control qualification and its lifecycle pilot. The next cost package triggers the four-package breadth review. Neither this certificate nor that pilot can complete the broader language-design investigation or research goal.
