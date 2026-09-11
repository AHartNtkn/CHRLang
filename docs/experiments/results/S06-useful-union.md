# Symbolic union preserves substantive alternatives; its economic value remains open

The existing diagram union correctly combines overlapping relations that remain nontrivial after graph simplification. It preserves correlations in a compact representation and avoids constructing an explicit table of accepted assignments. Competent explicit membership checks also avoid constructing that table, so compactness alone does not establish an architectural advantage.

The [registered screen](../registrations/S06-useful-union.md) uses three atomic names and four/six/eight visible coordinates. Families are overlapping cycles with a different missing edge, disjoint alternatives, repeated identical alternatives, and one cycle. No new solver or baseline was implemented. Hidden projection, arbitrary terms and consuming effects are not represented by this operation.

## The useful operation and its costs to investigate

**Overlapping alternatives remain correlated after union.** At eight coordinates, eight alternatives admit 3,072 successful branch assignments but only 1,266 distinct assignments. Every coordinate individually admits every name, yet the all-zero assignment is rejected. Replacing the relation with independent coordinate domains would therefore be incorrect.

**The combined representation is smaller than the separately allocated operand diagrams in these sources.** The following are observed representation and deterministic work-budget counts at eight coordinates. Node counts include intermediate allocated nodes and both terminals; they are not live bytes or reachable-node counts.

| Source | Distinct assignments | Successful branch assignments | Separate operand nodes | Combined allocated nodes |
|---|---:|---:|---:|---:|
| Overlap | 1,266 | 3,072 | 967 | 152 |
| Disjoint | 192 | 192 | 239 | 35 |
| Redundant | 258 | 2,064 | 1,192 | 54 |
| Single | 258 | 258 | 149 | 149 |

**Construction remains real work.** The overlapping case requires 4,760 charged budget ticks to compile its operands and 6,225 for sequential union operations. These are exact minimum accepted limits, verified by success at each limit and failure one tick below. They count the implementation's recursive and node-construction operations; they are not CPU instructions, timing samples or allocation measurements. Searching for the minimum is diagnostic work and must not enter a primary cost measurement.

**Short-circuit explicit checks are a serious competitor.** Across all 6,561 complete membership requests at width eight, the overlap source's direct control performs 47,154 branch probes and 125,022 disequality probes. Each request rejects a branch at its first failing edge and accepts the union at its first successful branch. A diagram walks at most one node per coordinate for a complete membership request, by its ordered construction, but its steps have different costs and require preparation. This suggests a measurable reuse crossover; it does not locate one.

All coordinates are visible in this screen. Exhaustively requesting them supplies a correctness oracle, not a workload distribution. A one-request client, changed restrictions and a full-output client may favor different organizations. Full-output comparisons need early-rejecting generation, not an explicit control forced to construct and reject every complete assignment.

## Strong controls and adverse interpretations

**Existing simplification remains applicable without eliminating the relation.** Every reduced source retains nonempty constraints. Its direct membership result agrees with union, separate diagrams, prepared name formulas and independent branch checks. The finding therefore does not establish that a general symbolic runtime is necessary: a prepared disjunction of reduced constraints is already a correct competitor.

**Redundancy does not require a general union engine.** The simplifier deduplicates identical reduced branches. The diagram union also imports only reachable operand nodes; a smaller combined allocation can consequently include compaction of construction intermediates, not only a saving from overlapping alternatives. Charging separate compilation of identical branches as unavoidable would exaggerate union's preparation advantage or disadvantage. The cost study must include source deduplication as an applicable control.

**Disjoint alternatives distinguish combination from duplicate elimination.** Their successful branch count equals their distinct assignment count. Union still builds a shared representation, but it has no overlapping successful derivations to collapse. The single-branch source isolates preparation and membership overhead without a union opportunity. These remain adverse contrasts even if a repeated-overlap regime proves favorable.

**The logical result does not preserve all CHR observations.** Twenty width-four source cases exercise unrestricted callers, two fixed-name callers, an aliasing caller and contradictory bindings. Independent enumeration agrees with the unchanged reference on completed successful branches and full output/residual observations. Logical union agrees on the assignment set, but it does not retain branch multiplicity or residual `different` occurrences. A language or source-region contract must justify discarding those observations; otherwise this diagram is not a complete replacement for source execution.

## Evidence and limits

The [source and control test](../../../research/chr-structural/tests/useful_union.rs) checks 29,484 complete assignments across five membership paths per repeat, 60 caller result sets and 20 reference source cases. Two repeats in each of the default and metrics-off library builds agree exactly. Failed bounded union leaves operands usable; results survive operand disposal. These checks do not measure memory restoration, cancellation during union construction or streaming service.

The [source/binary freeze](s06-useful-union/freeze.json), [raw runs](s06-useful-union/), [driver](../../../research/chr-structural/experiments/useful_union.py) and [independent audit](s06-useful-union/audit.json) preserve all twelve case rows. The auditor independently reconstructs relation cardinalities and explicit branch/edge probes in Python. The full structural suite passes 76 tests across 21 targets in each feature build; strict Clippy passes. Those receipts accompany the runs. No production engine or reference code changed, and no comparative timings ran.

## Next: price the qualified operation against its simpler controls

Select a bounded lifecycle comparison for these substantive relations. Reuse prepared union, separate diagrams, simplified disjunctions and applicable name solving across changing requests. Keep direct short-circuit membership and early-rejecting full-output generation competent. Charge source preparation, operand compilation, union, input construction, observation, retained consumers and disposal; measure allocation separately from ordinary counter-free timing. Do not mix logical-set costs with an unsupported claim about raw CHR replacement.

This follow-through is preferred over another capability gate because the operation and competing answers now exist, while construction and reuse could reverse the apparent representation benefit. Structural-prefix generated/access controls and their costs are the strongest ready distinct alternative. They remain required and will be reconsidered at the ownership/cost result or an obstruction, along with unfinished demand/integration costs. This is the first package after the structural-prefix portfolio review; the research remains active.
