# Next: compile finite source relations into joins

The next investigation tests direct compilation of finite relational source programs, rather than another refinement of rule execution. A compiler that derives joins from source could avoid choice enumeration and equality-failure branches. Existing R04 evidence covers a particular finite consistency schema; it does not establish this broader source translation or its costs.

## The proposed contrast

Start with a sealed finite source whose request consumes one occurrence and introduces several table-choice constraints. Each table-choice rule selects a tuple through explicit OR and equates its arguments with the tuple's ground fields. Multiple tables share variables, including repeated arguments and given values. The direct path compiles those tables and shared argument positions into joins instead of encoding interpreter transitions.

Preserve duplicate table rows as distinct derivations and raw answer multiplicity. Repeated arguments restrict the relation; they do not authorize nonbinding head matching to bind arbitrary inputs. Include disconnected factors, overlapping versus selective joins, contradictory givens, repeated calls, off-output failure, and an unknown carried outside the selected relation. This makes the source/certificate obligations explicit before choosing a solver representation.

The source certificate must inspect the actual complete ruleset, reject external observers and competing consumers where correspondence is not established, and retain the original behavior for unsupported sources only as a comparison control. It must not silently relabel unsupported programs as eligible. Preparation, certificate checking, relation/index construction, changing queries, output multiplicity and disposal all count in a later registered comparison.

## Why this now

S02 now has complete ordinary execution, favorable and adverse lifecycle results, and a measured correction of repeated candidate discovery. A further local improvement could change bounded rankings, but it would continue refining an execution organization that has received a direct trial.

S01's maintained-join comparison is the strongest ready alternative. It can decide how much discovery state should survive updates, especially in selective and consuming cases still missing from that stage. Direct source relation compilation asks a different question: whether that discovery/update loop is needed at all on an admitted source region. Its initial independent correspondence gate is feasible with the existing scalar oracle and can expose invalid lowering assumptions before solver implementation.

This is a research-order judgment, not a positive result for direct compilation or a resolution of maintained joins. Compare at least generic source execution and a direct join organization after the gate; reconsider R04's trailed finite solver where the translated relation admits it. Do not infer a universal join strategy or native-compilation benefit from the source gate. S06's recursive/contextual lowering and structural-space mechanisms remain separate obligations.

## Immediate work and exit

Inspect existing finite source certificates and relation controls. Then register an exact source schema, eligibility boundary and independent bag-relation oracle. Validate complete source correspondence, including contrary cases and multiplicity, before choosing the direct join algorithm and prospective lifecycle matrix. If source scheduling or observation invalidates the proposed translation, resolve the certificate or narrow the stated theorem explicitly; do not weaken full-answer checking.
