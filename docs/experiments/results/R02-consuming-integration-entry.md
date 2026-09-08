# R02: integrating equality with consuming rule execution

A useful integration experiment must let equality deductions activate source rules through the same representation. The first candidate can use one interpretation and consuming rules, without explicit search. Its strongest control is a dedicated term representation with binding-aware indexed activation.

This is an analytical entry, not an implementation or cost result. It develops [R00's independent representation direction](R00-architecture-selection.md) while R01 implementation proceeds. It uses the [E18 feasibility evidence](E18-relational-gate.md) without treating that prototype's explicit equality closure as the required representation or cost baseline.

## Complete candidate path

Lower terms into value identities and constructor descriptors. Keep each source constraint's occurrence identity separate from value equality. Matching reads established descriptors and waits for unknown structure; it must not invent structure to satisfy a head.

Place equality deductions, descriptor consistency, relation-key repair and source activation on one worklist. Identifying value classes updates their incident relation keys and wakes affected matches. A consuming application claims distinct live occurrences, posts fresh body identities and relations, and records propagation history for retained occurrences where required.

The experiment must demonstrate an equality-induced application while other consistency deductions remain pending. An implementation that completes a recursive tree-unification request before exposing any change would test a different boundary. Partial deductions may justify positive matching, but no answer can be published until the whole interpretation is consistent and all source/repair work is exhausted.

Extract selected terms and residual occurrences jointly. Value classes may coincide; source occurrences and propagation tuples must remain distinct. Class representatives are representation details, not observable answer identities.

## Small discriminating source program

```text
bind(X,T)             <=> X =:= T
open(K,f(Y)),ticket(K) <=> result(K,Y)
result(K,Y)           ==> seen(K,Y)
```

Introduce unknown `open` arguments, matching tickets and bindings to `f(a)`. With two equal-valued `open` occurrences and two tickets, complete execution must leave two `result` and two `seen` occurrences. This checks that value fusion does not fuse resources or propagation history.

Vary arrival order, indirect aliases, nested constructors and fresh body variables. Include independent constructor clashes and indirect finite-tree cycles. A clash after some applications have fired must still prevent every answer from that interpretation; early partial work can be valid speculation without being useful work.

The finite-tree obligation concerns positive constructor paths modulo equality. An equality-only cycle denotes aliases and is valid. Descriptor consistency needs symbol/arity agreement and child consistency; class/index changes must revisit affected matches. An independent terminal check must detect outstanding consistency, repair or source work beyond a candidate's own queue-empty claim.

## Predictions and controls

| Explanation | Discriminating observation |
|---|---|
| Integration avoids repeated term traversal and separate wakeup discovery | Complete-path cost falls when equality repeatedly enables selective structural matches |
| Class and index repair relocates or increases the work | High-degree aliases require broad key migration and repeated activation while few source matches become useful |
| Activation accounts for the apparent gain | Comparable binding-aware indexed activation in the dedicated representation removes it |
| Partial deductions improve responsiveness but cause speculation | Applications precede eventual inconsistency and add cost without changing completed answers |

Both candidates need occurrence ownership, histories, fresh allocation, completion and observation. The integrated candidate needs classes, descriptor incidence and relation-key repair. The dedicated candidate needs dereferencing, finite-tree unification and binding-watch maintenance. Necessary state and invariants determine the complexity comparison; module-call counts do not.

The strongest adverse case is a high-degree alias merge touching many indexed columns with few useful applications. Cheap no-binding rewriting and tiny cold queries also matter because integration may have little work to save. A first cost registration should vary alias fanout and useful-match selectivity independently and charge preparation, consistency, key repair, activation, matching and observation.

## Selection boundary

The next implementation needs an executable integrated path, a competent dedicated control, independent finite-tree/resource observations and forced deduction/application interleavings. R01 is not required to design or implement those indexes. Its findings may improve the conventional control, but cannot require the integrated candidate to preserve a completed-unifier API.

The [integrated semantic gate](R02-integrated-semantic-gate.md) now implements this entry, and the [first lifecycle pilot](R02-integrated-cost-pilot.md) records bounded cost findings. Constructor-decomposition and nested-repair costs remain selected follow-up work. Language adoption remains separate.
