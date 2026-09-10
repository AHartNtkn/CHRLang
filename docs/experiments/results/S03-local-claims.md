# Local claims preserve atomic effects, but not source priority

Occurrence-local acquisition is sufficient for the tested unordered atomic effects. It is not sufficient to replace the source-priority owner: it permits different observable winners. A global priority-admission control preserves the modeled source outcomes, while retaining global selection.

This is a bounded protocol result. Commit visibility is assumed atomic in the model; physical publication, native source integration and parallel efficiency remain unimplemented investigations.

## What the experiment establishes

The [registration](../registrations/S03-local-claims.md) fixes six scenarios: disjoint consumption, contention for one identity, overlapping multihead consumption, a kept-head reader, late equality enabling an earlier contender, and initially enabled equality. Each runs with no cancellation or one designated cancellation target, under local unordered acquisition and global priority admission.

All 40 configurations pass. Exhaustive traversal covers 414 reachable states and 581 transitions, including 51 transitions that cancel an application while it holds claims. Every terminal observation matches an independent atomic model, with equality of outcome sets rather than merely containment. No reachable deadlock or cycle exists in this finite action model.

The independent model contains no acquisition phases or locks. It applies consuming effects, binding changes and cancellation atomically. Separate direct expectations check the disjoint and single-resource cases. Observations include remaining identities, the binding bit, committed effect labels and cancelled applications. This does not yet validate ordered CHR residual insertion, fresh variables, branch correlation or arbitrary guards.

## Why safe consumption is not enough

| Source shape | What unordered local claims admit | Source-priority consequence |
|---|---|---|
| Disjoint consumers | Both effects, whichever claims first | The modeled final effect bag agrees; ordered downstream observation is not tested |
| Two consumers of one identity | Either consumer wins, with no double consumption | A later rule may win instead of the first rule |
| Overlapping two-head consumers | Either application consumes its complete head set | Different remaining resources and effects |
| Kept-head reader versus consumer | The consumer can remove the kept occurrence first | A reader effect required by the ordered execution can be absent |
| Equality enables an earlier contender | The binder and later consumer can commit without the earlier contender | A different winner despite safe occurrence ownership |

The native source path explicitly restarts at the first rule after each body, as recorded in the [identity source gate](S03-native-identity-source.md). The model's priority control follows that selection discipline for its admitted one-shot effects. Unordered results therefore describe a distinct execution contract; they are not a faster implementation of the current source contract.

These witnesses do not prove that every distributed protocol needs a central owner. They establish that exclusive occurrence claims alone do not encode the required scheduling dependencies. A dependency-aware protocol must also account for kept reads and bindings that change which earlier rule is enabled.

## What cancellation and progress mean here

An application acquires all kept and consumed identities exclusively in identity order. Cancelling before commit releases every partial claim; after commit cancellation cannot undo the effect. If another application consumes a still-needed head, the losing contender releases any other partial claims. Tests include both cancellation races and successful competing commits.

All modeled maximal executions terminate because the explored transition graphs are acyclic and have no nonterminal dead ends. Disabled polling, retry loops, worker failure and repeated rule discovery are not actions in this model. Thus this result is not starvation freedom for an arbitrary scheduler or crash recovery. Exclusive locking of kept reads is also a conservative implementation choice whose contention cost remains unmeasured.

## The implementation boundary that still matters

Commit currently makes consumption, one labelled body effect and a monotone binding update visible in one abstract transition. A physical implementation must realize that visibility through a descriptor or another concrete mechanism while different owners can be inspected separately. Treating several ordinary writes as that atomic transition would not implement the model.

The next package should compare dependency-aware admission with global source selection and explicitly staged publication. Include an earlier contender enabled by a disjoint binder, cancellation during partial publication, and observations between owner updates. Then connect the surviving protocol to real source execution and the qualified serial native control. General constructor matching, dynamic alternatives and complete lifecycle costs remain required.

This is package one after the continuing-service breadth review. T080 remains active because the next gate could determine whether global selection can be replaced, rather than merely validating the same source owner on another substrate. Adaptive costs remain the strongest ready alternative; reconsider them, conditional equality, richer theories and broader reuse at the physical/source gate or a consequential obstruction. No architecture has been selected and the research goal remains active.

## Evidence

[Protocol implementation](../../../research/chr-local-claims/model.py), [independent atomic model](../../../research/chr-local-claims/oracle.py), [bounded runner](../../../research/chr-local-claims/run.py), [frozen manifest](s03-local-claims/manifest.json), and [all results with shortest counterexample traces](s03-local-claims/results.json) retain the experiment. The independent replay audit checks the frozen files and recomputes every configuration and outcome comparison. No timing or allocation comparison ran. The reference interpreter and native source compiler are unchanged.
