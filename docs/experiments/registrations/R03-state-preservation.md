# State preservation across mostly failing alternatives

Prospective T050 cost screen. [Selection](../results/R07-sufficiency-audit.md) identifies substantial live state with cheap failed alternatives and a small final answer as missing evidence for the explicit-search representation. [Source gate and measurement boundary](../results/R03-state-preservation-design.md) establish source-built payload, query-dependent failure, cleanup, independent complete observations and attribution limits. This screen is more valuable now than carrier contraction because it can change general state-preservation ownership with existing source semantics. No alternative snapshot implementation is selected in advance.

## Hypotheses and controls

H1: preserving large live state makes split-producing service a consequential lifecycle cost even when most alternatives fail promptly. Its predicted requested traffic per split grows with payload state at fixed alternative count. H2: ordinary build/cleanup, equality/access or disposal dominates instead, making snapshot replacement lower value. H3: all-success cleanup and publication change the balance; that control prevents attributing every cost that scales with alternatives to storage. The zero-state and no-choice controls expose alternative-spine and fixed costs.

One source builds n keyed payload occurrences with a shared unknown and fresh bound cells. A right-recursive choice chain has a-1 ORs. Each candidate tests its key against a query target: final key for mostly-failing, a fresh variable for all-success. Successful source cleanup consumes the real payload and emits a constant-size full answer containing duplicate residuals and aliases. Expected success count is1 or a; failures a-1 or0; splits a-1. Expected source applications are n+1+a+(n+1)*successes. These are fixture predictions and independent gate checks, not architectural lower bounds. A compiler could exploit further source properties; this screen does not establish necessary copying or necessary choice creation.

Exact matrix: n in {0,64,512}, a in {1,8,64}, target mode in {mostly-fail,all-success}, query count in {1,4}:36 cells. Reuse one prepared ruleset; query i uses n+i%2 with the same a and target mode. The q4 n0 cells alternate zero and one payload; only q1 n0 is literally zero-state throughout. One warmup, five primary repetitions, one allocation diagnostic and one work run per cell:288 fresh processes. Random(50050) shuffles each mode's jobs, in warmup/primary/allocation/work order. No post-result enlargement or favorable-cell selection.

## Measurement and validation

Primary release: ordinary allocator, `--no-default-features --features experiment`; source, kernel and observer counters absent. Allocation diagnostic: `--no-default-features --features alloc-meter`, with counters absent. Work: `--features experiment`, ordinary allocator with counters. Separate target directories and binary hashes. No cross-mode time ratio is a primary speedup measurement.

Measure preparation including source ownership, each query's construction/cloning and setup, complete service, search disposal, retained answer disposal and final prepared disposal. Report first observation with its precise start boundary. Independent full-answer validation occurs outside measured intervals; later disposal is validation-warmed. Keep fixture and fixed reporting storage live consistently across allocation baselines; final measured-owner disposal must restore baseline. A cutoff is incomplete even if some valid answers were produced.

Primary service uses one clock interval, continuing after failed branches through exhaustion. Allocation diagnostics use nonnested windows for ticks grouped by returned event, observation, answer retention, terminal branch disposal and split-metadata disposal. Split-producing service includes the ordinary source step and scheduler/lineage operations as well as cloning. It is not isolated clone timing. Per-tick clocks may distort tiny operations; report diagnostic time as attribution evidence only. Sum source work once over retired prefixes and terminal segments. Requested traffic, peak and live heap are not RSS. Record maximum frontier and exact event counts where available without adding primary engine counters.

Validate complete residual/output aliases and raw counts outside timing; retain all process results. The source gate independently proves payload existed before the first split and missing payload or broken aliases cannot pass the terminal checker. Cost-run endpoint checks do not replace that source-stage evidence. Compile, host, commands, exact job order, source and binary hashes must be frozen before measurements and verified afterward.

## Bounds and interpretation

Serial processes on the minimum inherited CPU. Per query20million ticks. Per process30seconds and1GiB address space. Each build180seconds; locked offline Cargo. Execution budget20minutes after builds and meter-check. Preserve build errors, process failures, cutoffs, missing jobs and outliers. Do not restart a live process or overwrite evidence destinations. Compilation is readiness cost, not isolated architectural compilation evidence.

Report every complete cell's median/range lifecycle per query, phases, first observation, diagnostic event costs, source work, requested traffic and peak ownership. Incomplete cells are not timing competitors. Overlapping ranges remain inconclusive; do not add repetitions to obtain separation. Fixed a comparisons across n distinguish per-split ownership growth from more split events. No-choice controls include state construction/cleanup; all-success controls explicitly include repeated cleanup.

A prospective prioritization trigger is split-producing service contributing at least25% of diagnostic measured requested traffic or diagnostic measured lifecycle in a complete n512 mostly-failing cell with a>1, together with increasing per-split cost versus n0 at the same a. This warrants inspecting cloned owners before selecting one credible matched snapshot intervention. It does not predict a25% primary speedup: split service includes other work and instrumentation. If this screen does not trigger, assess the measured dominant owner and whether the observed limit makes storage work lower value; retain surprising conflicting evidence rather than applying the threshold as an automatic rejection. No universal copying, persistence or trailing ranking follows.

## Reproduction

After runner validation and commit, use a fresh destination:

```sh
python3 research/chr-compiled/experiments/state_preservation_pilot.py --output docs/experiments/results/r03-state-preservation --registration docs/experiments/registrations/R03-state-preservation.md --seed 50050
```

Analyze all planned and missing outcomes with:

```sh
python3 research/chr-compiled/experiments/analyze_state_preservation.py --output docs/experiments/results/r03-state-preservation
```

The analyzer and exact JSON schema must pass the runner gate before comparative execution. No comparative runs have started at registration creation.
