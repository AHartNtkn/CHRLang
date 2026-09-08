# Publication priority: exact answers with a sibling-delay tradeoff

The bounded T041 gate confirms that strict global observation priority can preserve exact answers and eventual exhaustion while delaying a distinct finite sibling until an exponentially sized answer region drains. It does not establish a memory improvement. End this bounded scheduler intervention and select mixed-phase generalization of the current engines.

[Prospective registration](../registrations/R06-publication-flow-gate.md), [default-mode observations](r06-publication-flow-gate/default.log), [counter-free observations](r06-publication-flow-gate/counter-free.log). The source witness and private controller are in `research/chr-direct-conditional/src/publication_flow_tests.rs`. Production scheduling and public APIs are unchanged.

## Executable witness

The root source OR has a bulk arm with n independent equal-arm choices and one nonground `bulk(X,X)` residual. Its sibling arm performs a16-step countdown and emits `sibling()`. The independent complete result is2^n duplicate bulk answers, each preserving the repeated-variable alias, plus one sibling answer, with no other residuals or selected outputs.

The test controller forces the existing Engine observer turn while certified observations are queued; otherwise it calls ordinary Engine.tick. It neither copies execution logic nor injects a synthetic answer/state. At the first source-generated bulk observation, the test establishes a live countdown, pending source entries and no sibling residual. Application traces remain available in both counter configurations. Selected source-state snapshots check equality version, occurrence liveness, dirty work, births and trace state throughout forced observer service.

| Independent choices | Bulk answers | Bulk delivered before sibling: interleaved | Bulk delivered before sibling: strict priority | Source applications at initial bulk drain: interleaved / strict |
|---|---:|---:|---:|---:|
|0|1|1|1|2 / 2|
|2|4|4|4|4 / 2|
|4|16|16|16|11 / 2|
|8|256|28|256|18 / 2|

All cases have two source applications and two pending entries at initial bulk certification. Under strict priority, source state stays unchanged while that observer is owned, then resumes to18 applications and exact exhaustion. Under ordinary interleaving at n8, the sibling is discovered while the initial bulk observer is still owned and is delivered after28 bulk answers. Under strict priority, it cannot be discovered until all256 have been delivered.

The complete cases finish within the registered5million-tick cap. A separate cancellation witness retains the first alias-bearing bulk answer while its observer remains owned, disposes the engine under each controller, and checks the retained answer afterward. Both tests pass in default and counter-free experiment builds. Strict Clippy and formatting pass. Independent review confirms the controller, source-generated boundary, full-answer oracle and narrow conclusion.

## Architectural disposition

The general lower bound follows from the policy: if every source opportunity is suspended until a finite region's2^n raw answers drain, another runnable source continuation waits for at least that many answer deliveries. This is an answer-position/service tradeoff, not measured elapsed time or a violation of eventual fairness. The baseline's precise tick ratio is not a language requirement. Strict priority may be suitable for a deliberately bulk-first contract, but no such product policy is adopted here.

Bounding a queue also does not bound the engine's historical support state. Avoiding the witnessed delay while throttling just the overproducing continuation would need support-local producer attribution or another scheduling organization. That is more than a queue-length adjustment. Neither the present evidence nor this gate selects that expansion or broad reclamation ahead of a new architectural interaction.

The current bounded conclusion is therefore retained: conditional sharing has positive finite opaque evidence and consequential low-sharing streaming costs; specialized explicit is the usable stream control. The supported history-pruning change remains validated, with its limits. Optional explicit lineage and conditional lifetime redesign stay open with their recorded costs and dependencies, not as an automatic implementation queue.

## Next selection: mixed-phase generalization

T042 compares current conditional and inferred-specialized explicit execution on one finite source pipeline: four binary choices, opaque common work carrying the correlated tuple, constructor discrimination, then another common computation. Vary where substantial work occurs and include zero-stage controls, keeping complete answers small. The question is whether sharing survives the transition and whether separately created occurrences prevent later common work from sharing. Independent residual/alias/multiplicity/exhaustion expectations precede a cost registration; no new backend or hybrid policy is assumed.

This has greater immediate decision value than further scheduler refinement because it tests a missing interaction between the two viable execution organizations. Homogeneous R03 cases and the separate R05 control comparison do not establish it. A useful result is a bounded applicability distinction, even if neither engine wins consistently. Do not automatically implement reunion or execution switching afterward.

Repeated coarse-region parallelism is the strongest broader alternative. Its positive W256/Q64 sizing signal and adverse overhead/imbalance controls deserve a repeated comparison, but primarily address transport amortization for already certified disconnected regions. The mixed pipeline addresses correlated computation in the current candidate/control directly and needs fixtures/oracles and runner extension rather than another runtime. E16's current counters and lifecycle boundaries also need review before new primary measurements. Parallelism, native compilation and eager-region eligibility remain explicit open questions; no architecture-goal closure follows.
