# Demand discovery removes the fused explosion, but repeats history work

Much of the contextual engine's fused allocation cost is avoidable. Selecting matches on demand reduces one complete lifecycle from 2.29 MB to 0.15 MB. Restarting that traversal is costly on propagation-heavy sources, so neither eager enumeration nor this demand policy represents a universal architectural choice.

## What changed and what stayed accountable

The [registered source gate](../registrations/S10-demand-discovery-gate.md) adds a separate contextual execution configuration. It traverses live occurrences in ordered head-tuple order and stops at the first candidate that passes guards and propagation history. It carries all matching environments for each prefix, preserving alternative partial constructor descriptions and complete environment ordering. It does not bind unknown query variables during matching.

The eager configuration retains candidate lists. The demand configuration reconstructs traversal for each selection, including history rejection. Both use the same equality store, branch frontier, body effects, observation and ownership model. The source contract and reference interpreter are unchanged. This is an experimental competing policy, not a default replacement.

On the four-head witness, eager matching constructs 270 full matches; demand offers one full candidate when the first is eligible. Tests also reject the first 1, 269 and all 270 candidates and verify exact selection order. An adverse witness with 16 kept-head propagation occurrences requires 152 demand candidate offers, including final exhaustion, against 16 matches constructed by eager enumeration. These diagnostic counts exclude prefix work and are not complete efficiency measurements.

The [implementation](../../../research/chr-relational/src/contextual.rs) is integrated through [contextual execution](../../../research/chr-relational/src/contextual_execute.rs). [Selection and source tests](../../../research/chr-relational/tests/demand_discovery.rs) cover partial constructors before/after equality settlement, guard rejection, history and late guard activation. The full mixed-source and progress/fusion gates now include demand execution. Guards, resource multiplicity, fresh aliases, choice multiplicity and finite-sibling publication all pass.

## Paired lifecycle evidence

The [registered attribution](../registrations/S10-demand-lifecycle.md) runs fresh eager and demand controls with the same current state layout. It completes 1008 processes: 288 allocation runs forming 144 exact pairs, then 720 ordinary timing runs. Every query, cancellation, prepared owner and inference owner restores. All warm and measured complete answers match the independent scalar outside measured intervals. [Audit](s10-demand-lifecycle/audit.log), [all cells](s10-demand-lifecycle/summary.csv), [frozen inputs](s10-demand-lifecycle/freeze.json).

Six families vary counts 0/1/3 and reuse 1/4. History adds 16 distinct propagation items per producer, giving 48 items at count three. The table shows three producers, four queries per preparation and one cancellation probe. Bytes are total requested allocation across inference, certification, preparation, input, execution plus owned-answer delivery, and disposal; they are not RSS.

| Source/form | Eager traffic, bytes | Demand traffic, bytes | Eager peak above baseline | Demand peak above baseline |
|---|---:|---:|---:|---:|
| Plain original | 409466 | 168082 | 22054 | 10631 |
| Plain fused | 2290673 | 152129 | 118594 | 13283 |
| Independent choices, fused | 3236588 | 404172 | 172461 | 29022 |
| Shared payload, fused | 3095772 | 264444 | 172461 | 17720 |
| History original | 779753 | 4923297 | 48036 | 44580 |
| History fused | 2668743 | 4858295 | 131488 | 46829 |

Demand substantially reduces both retained state and traffic in the fused resource families. The history source separates those effects: demand retains less memory yet allocates much more over time. Repeated traversal and history testing are therefore material costs, not merely speculative objections to demand execution.

Timing remains exploratory. Plain fused median phase sum falls from 1198178 ns (882102–1255054) to 133277 ns (120319–156144). History original instead changes from 841794 ns (507229–846066) to 1632402 ns (1415238–2354538). Five samples do not establish formal practical-win classifications or workload weights. These current paired controls are authoritative for this attribution; changes in state layout and measurement conditions prohibit treating an earlier run as the paired control.

## Validation and limits

The interface test initially fails because demand selection is absent, then all four new tests pass. The relational library/test suite passes 65 tests. Expanded mixed and progress/lowering gates pass in default and feature-off builds: per build, 32 finite mixed configurations, 60 lowering-query configurations across seven candidates, and 28 ongoing progress executions. [RED](s10-demand-discovery/red.log), [new tests](s10-demand-discovery/gate.log), [relational suite](s10-demand-discovery/relational.log), [default composition](s10-demand-discovery/composition-default.log), [feature-off composition](s10-demand-discovery/composition-off.log).

Strict Clippy passes for the [library and new test](s10-demand-discovery/clippy.log), [ordinary runner](s10-demand-lifecycle/clippy-time.log) and [allocation runner](s10-demand-lifecycle/clippy-meter.log). The integrated build script now explicitly allows unused members of its imported shared generator module, which it uses only for one emitter. That fixes a dependency lint obstacle; existing generated-code hashes remain unchanged.

Native compilation, process startup and oracle costs remain outside the measured lifecycle. Observation is included in delivery. The benchmark uses a finite batch consumer and all queries satisfy fusion eligibility. Sustained lifetimes, rejected-query switching costs and broader source mechanisms remain required. Conditional discovery has different support/completion obligations; this branch-local implementation does not repair or resolve it.

## Next architectural decision

Return to broader mixed-source lifecycle qualification under T078. Include both eager and demand contextual controls, conventional execution and direct conditional execution; sources must combine guards/history, alias changes, search, early failure and output retention with independently known complete observations. These controls now expose a credible contextual alternative without assuming that restarting discovery suits every rule.

The strongest ready alternative is resumable or selective discovery, including direct conditional discovery. It remains consequential: retained traversal may avoid both repeated history work and full-tuple materialization, while conditional completion may require a different design. Broader composition takes priority now because it can show which responsibilities dominate beyond the resource-fusion family and whether that further discovery work changes complete architectural choices. Reconsider it at the next mixed-source package boundary. The research remains active; no organization is selected or rejected by this attribution alone.
