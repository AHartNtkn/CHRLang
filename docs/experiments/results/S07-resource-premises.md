# Resource counting needs more than privacy and ground inputs

The current counting certificate rejects some programs that can safely be shortened. Conversely, private fuel and a ground depth do not guarantee enough resources to finish. A language comparison must distinguish the properties users declare from the stronger conditions this particular optimization checks.

The first T079 investigation tests those distinctions using executable sources. It does not adopt a restriction, extend the compiler's accepted fragment, or measure declaration costs. The next comparison must implement concrete checked declarations and query boundaries using the distinctions below.

## What the executable examples establish

Each full answer is checked against an independently constructed expectation by the scalar semantics and existing scanned and indexed execution. Output names, residual multiplicity, shared unknowns and fresh results participate in comparison. The tests require exhaustion within 200,000 calls; hitting that bound fails the test.

| Question | Experiment and result | Consequence for language design |
|---|---|---|
| Must nobody else read fuel? | Add a propagation rule `fuel ==> true`. The checker rejects the source. Original and shortened executions nevertheless have identical full answers across six choice/depth settings. | Syntactic privacy is sufficient for this part of the certificate, but is stronger than necessary for these examples. A mandatory privacy rule would exclude a harmless reader. Its history and execution cost still differ. |
| Must every other entry effect be inert? | Entry posts `ping`; a rule consumes it and posts `tagged`, without sharing a variable or resource with the traversal. The checker rejects it. Both executions preserve the extra residual across six settings. | Independence or commutation could admit more programs than the current blanket prohibition. These examples do not establish a general effect analysis. |
| Must the depth be ground at query submission? | Submit an unknown depth plus an independent rule that supplies its value. Counting rejects the query, but ordinary execution reaches the expected full answers in six settings. Supplying that same value during query construction also preserves the answers and permits counting. | Mandatory ground entry changes the client contract. This particular producer can move to query construction; arbitrary producers with effects, failure or choices cannot be assumed movable. |
| Can shortening discard an observed depth? | Entry posts its depth as a residual. The original answer contains depth four; unchecked shortening reports zero. Both answers are independently verified. | Preserving this observation is a real semantic obligation. Rejecting this source is justified for the current transformation; a different transformation could preserve the original depth separately. |
| Do groundness and privacy imply successful completion? | Give a ground traversal one fewer fuel occurrence than required, at depths one and four. Ordinary execution returns an unbound result and a suspended `run(s(z), R)` residual. The certificate rejects the query. | Resource sufficiency is a separate condition. A property-restricted language must either preserve this behavior or explicitly exclude these inputs; it cannot silently classify them as impossible. |

The existing [source gate](S10-resource-count-gate.md) supplies the opposing effect examples: observing fuel changes markers, and an entry effect sharing the result can race the shortened terminal and change which consuming rule fires. Therefore the successful independent-effect examples justify investigating a more precise checker, not simply dropping the interference check.

The same gate also admits sufficient common fuel with insufficient branch-specific fuel. Counting preserves completed and suspended branches together. Even successful certification does not mean every branch's remaining resource execution disappears.

## Compare the contracts at their actual boundaries

**Inferred eligibility preserves ordinary source and query behavior outside the certificate.** The optimizer can decline a source or query without excluding it from the language. It pays analysis and query checking; the [lifecycle pilot](S10-resource-count-lifecycle.md) already shows that successful eligibility is not sufficient evidence of profitability.

**A checked declaration must state a property, not merely repeat “the optimizer accepted this.”** Resource privacy concerns which rules can access a predicate. Ground entry concerns an argument at an invocation boundary. Neither statement includes sufficient fuel, one entry, head-pattern agreement, commuting effects or the other conditions imposed by this certificate. The upcoming executable declaration path must check the asserted property and separately establish any extra optimization premises.

**Making those properties mandatory changes admission but does not automatically simplify execution.** Unknown-depth callers need a reformulation or rejection. A harmless reader may be excluded by syntactic privacy. Yet ground, private inputs can still suspend, and eligible sources still have choices, output unknowns, propagation history and branch-specific consumption. The current counting transformation retains those runtime responsibilities. A different restricted-language compiler must demonstrate their removal before claiming that benefit.

**Mandatory acceptance by the entire counting certificate would be a different and stronger language restriction.** It would also constrain entry cardinality, resource amounts, other active constraints and source effects. That option must be named and evaluated separately; it cannot stand in for mandatory privacy and groundness.

**Declarations cannot make external validation free.** A source property can be checked when the complete rule set is available. Adding a rule that observes fuel changes the obligation at linking or preparation. A ground-input declaration still needs trustworthy query construction or checking at an external boundary. If inference and declarations establish the same fact at the same boundary using the same checker, any efficiency difference needs evidence beyond their spelling.

These contract comparisons are analytical consequences of the stated interfaces and executable examples. Declaration syntax, modular checking, validated query ownership and their costs have not yet been implemented or measured in this package.

## Validation and its limits

[Five tests](../../../research/chr-compiled/tests/resource_properties.rs) cover 21 source/query configurations and 46 original, transformed or reformulated executions. Each runs through the independent scalar and two candidate execution paths: 138 full-answer checks per build, 276 across default and counter-free builds. The four existing resource-count boundary tests also pass in each build. [Default log](s07-resource-premises/default.log), [counter-free log](s07-resource-premises/counter-free.log), [strict Clippy](s07-resource-premises/clippy.log).

The evidence concerns finite complete observations under the existing global scheduling policy. It does not establish contextual equivalence under every extension, identical traces or service counts, equal cancellation latency, or performance. The silent reader in particular changes propagation history and work even though these finite answers agree. No reference interpreter code or runtime implementation was changed.

## Next experiment and priority

Continue T079 by implementing a checked source declaration for resource privacy and an explicit ground-entry boundary. Test rule-set extension, invalid declarations, changed queries and the rejected-but-valid witnesses above. Compare optional declarations and mandatory property admission while keeping optimization eligibility separate. Record the exact responsibilities moved to the client, linker, compiler and executor; then register their costs once those paths exist.

This is still the first bounded S07 gate, not completion of the language comparison. Its immediate value is preventing a trial from equating two properties with a much stronger optimizer contract. Reconsider selective conditional discovery and resumable contextual matching after the executable contract gate, as scheduled. Broader S07 properties, sustained lifetime, complete architecture comparison and held-out challenges remain required.
