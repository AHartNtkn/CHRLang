# Live-state preservation screen: measurement boundary

T050 will test whether copying substantial live state across cheap, mostly failing alternatives warrants a different explicit-search representation. Source inspection establishes copying, but does not establish its share of total cost. The first gate therefore needs meaningful source state, independent observations and a measurement boundary that separates fork service from ordinary execution and disposal.

The [sufficiency audit](R07-sufficiency-audit.md) selects this question over the concrete carrier-contraction alternative. No new snapshot representation is selected. A matched representation comparison becomes useful only if the screen establishes consequential storage costs after preparation, failure, cleanup and observation are included.

## Attribution boundary

`SearchEngine::tick` pops a runnable branch, advances its ordinary engine, handles an explicit OR by cloning state and adding branch lineages, then returns an event. A diagnostic interval around a tick returning `Split` therefore measures **split-producing service**, including the source step, state copying, pending-arm setup, lineage and frontier bookkeeping. It is not isolated cloning time.

Primary timing must use one clock interval around complete query service, with the ordinary allocator and source/kernel diagnostics disabled. It must continue after failed branches until global exhaustion. Retain complete successful answers through service, validate afterward, then dispose them. Preparation, query setup, search and terminal-owner disposal must be charged with truthful ownership boundaries.

A separate diagnostic process may wrap ticks and classify their intervals by returned event. Returned failed/completed branches still own their engines; their disposal lies outside the tick and must be measured separately. Successful full observation likewise occurs after the complete event. Split events can own a retired-work box in metrics builds and a lineage vector in every build; releasing them is a distinct cost rather than part of the recorded tick. A returned event's allocation record must not be interpreted as its eventual live ownership.

Requested allocation and per-tick timing instrumentation belong to separate diagnostic configurations. Per-tick clocks can overwhelm cheap source steps, so those times cannot supply the primary architecture ranking. Requested heap is not RSS. Source work counts describe actual work and must count retired fork prefixes once. Long-lived source syntax and fixed recording storage require explicit allocation baselines.

[Earlier source-search diagnosis](R03-search-diagnosis.md) already attributes a large branch explosion to delayed failure service and supplies a competent global control. T050 keeps that global source policy and fixes the number of alternatives independently of state size. It is not a repeat of the failure-scheduling intervention: it asks about the cost of each legitimate state-preserving split when failure is already cheap and prompt.

## Selected source contract

The fixture builds `n` keyed payload occurrences before choice. Each payload retains a structured cell and a repeated shared unknown; fresh per-cell variables are bound during construction. A right-recursive chain supplies `a-1` binary ORs for `a` alternatives. Each arm checks its distinct candidate key against a query-supplied target through ordinary finite-tree equality. A target equal to the final key makes earlier arms fail; a fresh unknown target allows every arm to bind independently and enter cleanup. These are query configurations of one source, not literal failing OR arms that syntax folding could discard. `a=1` is the no-choice control.

Successful cleanup consumes each keyed payload using its full cell and shared-variable pattern, then produces a constant-size answer with duplicate residuals, joint aliases and a distinct unconstrained output variable. Expected raw successes are one or `a`; failures are `a-1` or zero. Construction and successful cleanup each scale with state size and remain charged. This source shape is a right-recursive alternative chain, not a wide balanced frontier. The number of alternatives does not by itself describe simultaneously retained branch count.

The terminal observation has deliberately small size, but its exact alias and duplicate structure remains part of validation. A source-stage inspection before the first split must prove the payload was present; a skipped cleanup must leave residuals that the checker rejects.

## Controls needed before registration

State size and alternatives vary independently. A no-choice control exposes state construction and successful cleanup without snapshots. An all-success control exposes repeated cleanup and full publication. A mostly-failing case with one small answer exposes state copied into branches that do not publish it. Payload state must be consumed by source rules in successful executions; simply suppressing residuals would invalidate the experiment.

Independent checks must establish exact success/failure counts, raw multiplicity, complete residual/output structure, aliases and exhaustion. Prepared rules should support changed queries. At least one adverse mutation or source-observation check must make missing payload or skipped consumption detectable. A complete finite answer checker alone cannot establish that the required live state actually existed before choices; inspect that source-stage invariant independently outside measured intervals.

Any timing matrix, repetitions, sizes and resource caps will be registered after the executable source gate and actual runner are reviewed. This design records no timing evidence and makes no claim that copying dominates. Source cleanup or access may dominate instead, which is a consequential possible result of the screen.

## Executable entry validation

The [fixture](../../../research/chr-compiled/tests/state_preservation_support/mod.rs) and [five tests](../../../research/chr-compiled/tests/state_preservation.rs) establish the selected source contract. The main 24-cell matrix varies state sizes 0/1/4/16, alternatives 1/2/5 and bound/free target. Every terminal branch is checked by the independent owned-syntax scalar source replay; complete answers additionally pass the allocation-free structural oracle. Branch lineage conservation and exact success/failure counts are checked through global exhaustion.

Eight pre-split controls inspect the full resolved payload store for sizes 0/1/8/32, then cancel and reuse preparation. Failed branches retain their full payload. Six wanted-key cases select early, middle, final or no successful arm without changing rules. Source mutations that omit a payload or break its shared alias cannot yield the accepted small answer. Separate oracle checks reject wrong aliases and residual multiplicity.

The source application prediction is `n + 1 + a + (n + 1) * successes`: build, choices and successful cleanup respectively. Equality service and split ownership are additional work. The alternative spine itself contributes source/query state proportional to `a`; controls must distinguish that from the `n` payload occurrences. These counts are analytical attribution, not timing evidence or a mandatory trace across different architectures.

[Default tests](r03-state-preservation-gate/t050-state-default.log), [counter-free tests](r03-state-preservation-gate/t050-state-off.log) and [strict counter-free Clippy](r03-state-preservation-gate/t050-state-clippy.log) pass; workspace formatting also passes. Root inspected the fixture, complete checker, scalar replay and pre-split assertions. Independent measurement review confirms event-grouped diagnostics are sufficient for the first screen, provided their inclusive ownership and source-step costs are disclosed.

T050 remains active. The next work is the measurement runner, its counter/allocator ownership gate, then exact prospective cost registration and bounded runs. No source engine, snapshot strategy, scheduling policy or reference algorithm changed in this entry.

## Measurement runner entry

`chr-state-preservation-cost` now measures reusable preparation, query construction/setup, whole service, search disposal and retained-output disposal. First observation starts at service entry and ends after full export; add query/setup and preparation for other latency boundaries. Engine disposal precedes validation; retained-output disposal follows it. Allocation diagnostics partition event service and returned-owner disposal into nonnested windows. Frontier maxima are diagnostic-only. On cutoff, work counts cover retired prefixes and terminal segments only, omitting unfinished frontier work; those rows are incomplete.

[Primary](r03-state-preservation-runner/t050-runner-primary.json), [work](r03-state-preservation-runner/t050-runner-work.json) and [allocation](r03-state-preservation-runner/t050-runner-allocation.json) readiness runs use n2/a3/two mostly-failing queries. All validate complete answers and counts; allocation baseline/final live bytes agree. These are small correctness runs, not comparative timing evidence. The existing compiled experiment owns the single allocator; the runner accesses its meter with visibility changes only.

Default/counter-free runner tests cover both outcomes, no-choice and cutoff. Strict Clippy passes in default/counter-free/allocation configurations and formatting passes. Root row-audits all three smoke records and verifies rejection of altered counter flags, raw counts and allocation sums. An injected build failure preserves its attempted command and all288 prospective jobs; the analyzer reports all missing runs and failed/missing builds. [Failure-check receipt](r03-state-preservation-runner/launcher-failure-check.json) is a validator exercise, not an experiment outcome.

Independent final review finds no blocking ownership, instrumentation or cutoff defect. [The prospective registration](../registrations/R03-state-preservation.md) fixes the36 cells,288 processes, bounds and interpretation. Source/binary freeze and complete outcomes remain required before interpreting the cost screen.
