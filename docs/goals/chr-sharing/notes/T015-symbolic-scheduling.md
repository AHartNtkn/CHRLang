# Fair scheduling does not require a ticket for every alternative

A finite-round symbolic scheduler can preserve eventual progress without allocating one scheduler object per search alternative. The construction establishes possibility, not low latency or a useful implementation cost. Its central requirement is that each round operates on a sealed finite snapshot.

This is an original construction extending the reference and the finite-service argument in T012-scheduler.md. It assumes a finite current representation, finite-tree built-ins, and a fixed permitted committed policy. Arbitrary nonterminating host guards do not meet its premises.

## Represent control by conditions

Maintain a finite symbolic description of the live alternatives, their pending work, stores, substitutions and histories. A control map associates conditions with source-operation descriptions or resumable administrative work. Equal descriptions can share one entry. Conditions have exact, terminating Boolean operations; the existence construction permits a decision diagram or finite formula with exhaustive valuation as its slow fallback algorithm.

Alternative identity is retained independently of equal control entries. Merging two equal operation descriptions does not identify the alternatives they service. Dynamically inactive choice labels must have one canonical assignment, or be omitted by the frontier grammar. Enumerating both values of an inactive label would introduce spurious alternatives. Each genuinely executed OR creates two children under its birth condition.

The representation may be large. The claim is that fairness does not itself demand an additional object for every leaf; it is not a bound on arbitrary stores or Boolean functions. In the regular case of k common binary splits followed by a common continuation, the choices and one common operation description can be represented without 2^k individual scheduler records. Branch-dependent bindings may independently dominate memory.

## Sealed rounds

At the beginning of round r, freeze the current finite state as S_r. No newly produced source goal enlarges the source work selected for that round. The following steps are finite algorithms on S_r:

1. Symbolically select at most one source transition for each live alternative according to the fixed policy. Branches with no enabled transition are candidates for quiescence certification. Selection can split conditions when answers to constructor, membership, guard or history tests differ.
2. Put selected operations and certification jobs in a FIFO queue with immutable inputs and finite supports. Resume each after a bounded number of graph/worklist actions. New administrative subjobs are finite and enter fairly.
3. Commit an operation only on the portion of its support where its entire reference transition has completed. Distinct selections have disjoint branch supports; no second source operation changes that branch in this round. Physical sharing still requires version-safe data structures or a single-threaded commit sequencer.
4. Publish completed successful answers through separate finite, fairly serviced extraction/deduplication jobs. A completed support portion does not wait for another portion's computation before entering that queue.
5. Once the round's finite work is exhausted, begin r+1 with the resulting state. New children of an OR first receive source service in that next round.

A source operation here is Introduce, Unify, Apply, Split, Fail, or a finite quiescence check. It is never “normalize this branch.” Matching, finite equation solving and finite tuple coverage may be enormous, but each terminates on the frozen finite data. A recursive source rule contributes one transition in a round, not an unbounded recursive call hidden in one job.

The round barrier is deliberate. It provides a simple existence proof with no retries caused by unbounded new arrivals. It may be unacceptable for practical latency. Removing it requires another progress argument; the construction does not present a global barrier as the desired scheduler.

## Progress proof

**Finite-round lemma.** Each S_r has finitely many alternatives and finite data, even when those alternatives are represented compactly. A finite reference transition requires finite work under the stated primitive assumptions. Therefore an extensional algorithm selecting/executing one transition per alternative terminates. A symbolic implementation that simulates that finite computation using terminating condition operations also terminates, though it may have exponential cost. Finite resumable service prevents an individual administrative action from monopolizing the worker.

**No starvation across source rounds.** Fix an alternative whose selected committed derivation succeeds after d additional transitions. The current round finishes in finite time. If the alternative survives, each successive round either advances its selected transition or certifies it when no transition is enabled. The d relevant rounds and certification therefore finish in finite time. Indefinitely recursive siblings execute only one transition per round and cannot make any individual round infinite. Fresh descendants cannot join an already sealed round.

**Answer progress.** Extraction and duplicate comparison for a fixed answer examine finite data and finitely many preceding accepted answers at its admission point. Later answers cannot continually enlarge that job's comparison set. FIFO finite service eventually publishes the answer or proves it equal to an earlier one. This assumes a draining consumer and available memory.

These claims are relative to the selected committed CHR policy. They do not search competing rule schedules, prove a wall-clock bound, guarantee finite space for infinite search, or establish every possible answer of a nonconfluent rewrite system.

## Counterexamples to stronger claims

A representation-independent low-latency claim is false for this construction. An unrelated branch can have a finite but enormous store scan, delaying the next source round of a small sibling. Eventual fairness holds; bounded response time does not. This separates the owner's nonstarvation ideal from an engineering claim about comfortable interactive enumeration.

Replacing the sealed round by “keep adding all newly found work to this batch” invalidates the finite-round proof. A recursive sibling can keep the batch open forever. Replacing each finite source transition by full normalization similarly invalidates it. Treating a blocked constructor demand as an occupying worker prevents its producer from running.

A compact control map can also lose sharing through its implementation. If it eagerly enumerates every valuation before recognizing identical opaque continuations, it spends at least proportional work in the number of alternatives even when the continuation executes once. This is not a correctness failure, but it can erase the intended benefit. The representation and the selector must be charged separately in experiments.

## An asynchronous improvement to investigate

A region can start its next source step before unrelated regions finish when all operations capable of changing its projected state are accounted for, and no older conflicting operation is starved. An immutable version and complete relevant dependency coverage can establish that boundary. This permits local rounds with different progress rates, but only if producer admission, overlap reservations and region subdivision preserve queue age.

The important remaining question is the cost of computing those dependency boundaries without visiting every alternative. It is not whether compact scheduling is logically compatible with fairness: the sealed-round construction answers that narrower question. A fully asynchronous symbolic scheduler still needs a complete algorithm and proof before its implementation cost can become the only unresolved issue.

## Consequence for candidate evaluation

Keep three distinct claims in the comparison: finite-answer fairness, latency isolation between alternatives, and compact control overhead. Per-leaf tickets are one correctness control. Symbolic sealed rounds are another, with different latency costs. An asynchronous condition-based scheduler must be compared against both rather than inheriting either one's guarantees from its queue data structure.
