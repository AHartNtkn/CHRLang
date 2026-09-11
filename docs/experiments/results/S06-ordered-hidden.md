# Hidden search can shrink while preserving ordered caller answers

The experiment generates 9,600 successful assignments from 26,112 possible source
assignments, preserving every raw caller answer in order across 192 cases.
Unequal branch lengths break a merge based only on choice order. A merge ordered
by branch work and then choice indices passes the challenge.

**This is a working alternative to replaying every hidden choice.** It establishes
correct ordered answers for finite unequal-name stars, including duplicate choices
and token competition. Whether its ordering machinery is cheaper than enumeration
is the next measurement, not a conclusion from assignment counts.

## What the implementation does

Conditioning on a visible name makes each hidden arm of an unequal-name star
independent: its choices must differ from that name. The existing sparse projection
supplies each visible tuple and its multiplicity. The experimental iterator builds
allowed occurrence-index lists for each arm and checks their product against that
multiplicity. Duplicate values keep separate occurrence indices.

The iterator merges these Cartesian streams using a standard priority queue.
Each queued assignment is ranked by its extra source work, then its original
choice indices. Increasing one coordinate produces neighboring assignments;
a set prevents different paths from queueing the same assignment twice. Failed
assignments never enter this frontier. This implementation is specialized to the
star relation; it is not a general join or source-plan compiler.

Each delivered value enters the caller at the original launch rule. The caller
still runs its propagation-history rule and competing token consumers. A scalar
interpreter independently checks complete raw bags. The reference interpreter
supplies the raw source order by observing occurrence indices, which distinguish
duplicate branches without introducing consuming rules.

## Results and the consequential repair

| Experiment | Cases | Source assignments | Admitted assignments | Result |
|---|---:|---:|---:|---|
| Uniform branch lengths | 96 | 13,056 | 4,800 | Choice-index merge matches ordered caller answers |
| Uniform and unequal lengths together | 192 | 26,112 | 9,600 | Work-first merge matches ordered caller answers |

The unequal-length challenge adds three pending operations to odd-index choices
at each coordinate. The index-only merge fails immediately: for two coordinates,
it starts with [0,1], while the source first completes [0,2]. Equal answer bags
would miss this change to the first answer and cancellation result.

The repaired key accounts for the extra work along each assignment. Every
successful branch otherwise has equal work in these fixtures. FIFO completion
therefore agrees with that key in all tested sources; the comparison uses actual
reference execution, rather than treating this scheduling argument as a test.
The source-plan calculation must be extended and challenged again for other
control flow, failures inside reusable subcalls, or intermediate caller effects.

## What this establishes—and what it costs

The matrix covers two/four coordinates, distinct/duplicate four-entry domains,
forward/reverse choices, first/last visible coordinate, three token priorities,
plain/aliased outputs and uniform/unequal branch lengths. All 192 complete raw
answer sequences match, including residual history and consumer results. Every
answer-boundary cancellation prefix and restart matches. The new experiment passes in default and counter-free builds; all 21 relevant
counter-free tests and scoped Clippy pass. The iterator owns its
ordering data; projected preparation is disposed before caller delivery begins.

Admitted assignments are 36.8% of the Cartesian source space. That is a search-work
count, not a runtime ratio. The implementation also pays for projection, allowed
lists, a priority queue, neighbor generation and retained visited positions.
It executes a caller per raw occurrence, so output multiplicity still costs work.
The current checks do not establish delivery after equal interpreter-step budgets;
that observation contract differs from cancellation after an answer. These
experiments do not select a language contract.

## Next: complete costs against a competent enumeration control

Measure the entire ordered caller path, including preparation, query setup,
ordering, consuming execution, observation, cancellation and disposal. Compare
against enumeration using the same branch-work ordering and caller execution,
as well as source execution. Include favorable sparse stars, small cases and
dense admissibility; charge retained visited positions and held consumer answers.
Use separate ordinary-allocation timing and allocation diagnostics.

This is more useful now than further local projection tuning: it can show whether
avoiding failed assignments compensates for the ordering machinery. Broader call
observers and selective retention remain the strongest ready alternatives.
T076 remains active, package count two since the portfolio review. No architecture
or language restriction is selected by this result.

[Registration](../registrations/S06-ordered-hidden.md) ·
[Executable experiment](../../../research/chr-structural/tests/ordered_hidden.rs) ·
[Previous caller experiment](S06-projected-caller.md)
