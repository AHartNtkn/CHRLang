# S04: restoration and replay preserve complete source answers

> This records an earlier gate or checkpoint. The [complete lifecycle report](S04-lifecycle-pilot.md) and [matcher correction gate](S04-matcher-copy-gate.md) give the subsequent evidence.

Copying, reversible changes and replay now pass the same independent source tests, including a finite answer beside ongoing work. Periodic checkpoints also pass. This establishes viable alternatives for a cost comparison; it does not establish which state organization is more efficient.

## What was compared

The [source-gate registration](../registrations/S04-restoration-source-gate.md) specifies complete CHR restoration before comparative timing. The implementation in `research/chr-restoration` uses one set of source execution primitives with six configurations:

| Configuration | How it services a queued interpretation | What it retains |
|---|---|---|
| Copy | Move state during ordinary progress; copy at explicit choice | Separate map containers with shared immutable term, occurrence and pending-goal payloads |
| Trail | Undo to a common ancestor, then redo the target interpretation's changes | Reversible changes needed by the frontier; lone-branch history is reset |
| Root replay | Reconstruct the initial state and recompute the recorded prefix | Initial state, choice decisions and source-step positions |
| Checkpoint replay, intervals 1, 4, 16 | Recompute from a saved state and periodically replace that checkpoint | Shared checkpoint state and the decisions/progress since it |

Checkpoint replacement occurs after a non-fork progress step at or beyond the interval. Consecutive choice steps can exceed that interval until the next ordinary progress step. This avoids copying both child states merely to checkpoint a fork; it is not a strict bound on replay length.

All configurations service interpretations through the same FIFO policy. A source step can contain a size-dependent join or equality operation, and switching/replay can grow with its prefix. The gate establishes source-service fairness on its witnesses, not a uniform wall-time guarantee.

The existing compiled Global Scan and Global Indexed executors provide separate architectural controls on the finite cases. They use the existing persistent engine. The new copying configuration isolates state organization over matched primitives; it is not a proposal for another production baseline.

## Independent evidence

The finite gate has 28 source/query configurations: eighteen directed configurations and ten nested-choice configurations. The eighteen directed configurations are also checked with reversed rule order, producing 46 configuration/order runs per executor. Some reversed one-rule sources are identical; they are not additional unique programs.

Each of the six restoration configurations passes all 46 runs: **276 candidate comparisons**. Both compiled controls pass the same runs: **92 control comparisons**. Expected answers come from the independent owned-syntax scalar evaluator, which imports no restoration implementation. The ten nested-choice configurations also have hand-derived full answers and exact raw multiplicities.

The directed cases exercise partial failed unification, occurs-check failure, repeated aliases, branch-local consumption, kept resources, propagation history, fresh locals, pending conjunction tails, stable nonbinding guards and duplicate alternatives. Comparing joint outputs and residuals preserves alias relationships and occurrence multiplicity. Finite runs must exhaust within 100,000 scheduler advances.

Each restoration configuration publishes exactly one expected finite answer during 100 advances beside a recursively continuing sibling. After cancellation, the same prepared rules execute a fresh query without the prior query's state. Empty-head rules and exhausted initial variable identity are explicitly rejected.

Two internal ownership tests complement the answer comparisons. One undoes and redoes a complete sequence containing bindings, live resources, propagation history, pending work and both fresh counters, checking exact state restoration. The other uses weak references to verify that obsolete lone-branch checkpoints and trail roots are released; it also verifies checkpoint-prefix reset. This is bounded ownership evidence, not a measured long-stream memory bound.

## The tests detect broken restoration

Four diagnostic builds deliberately skip inverse edits. All compile and fail the **release-mode source answer tests**:

| Omitted restoration | Observed failure |
|---|---|
| Bindings | Incorrect raw answer multiplicity |
| Live source occurrences | Incorrect complete answer |
| Propagation history | Incorrect complete answer |
| Pending source effects | Incorrect complete answers, including nested choices and finite-sibling behavior |

The [mutation script](../../../research/chr-restoration/experiments/check_mutations.py) restores the exact source in a `finally` block. Its [receipt](s04-source-gate/mutations.json) records the source hashes and runtime failures. These failures establish that the relevant rollback responsibilities affect the semantic gate; successful calls to an undo API alone would not establish that.

## What this changes, and what remains open

A trailed organization can meet the tested fair-service contract by restoring between retained paths. It does not need to adopt the copied-branch interface or abandon finite siblings. Replay with periodic checkpoints is also a working competitor, so a subsequent root-replay loss cannot be generalized to all replay.

There are substantial unmeasured costs. The matched source implementation resolves owned structural terms and scans rule tuples. Trails retain ancestor paths while siblings need them; checkpoints copy map containers when reconstructing state. Their comparison can isolate restoration behavior within this representation, but architectural conclusions also need the existing controls. General checkpoint policies, other trail organizations, deep destruction and sustained frontier retention remain open.

No comparative timing or allocation matrix has run. The next T066 work is to register and run a lifecycle pilot that varies retained state, mutation, depth, width, failure placement and useful work between choices. Charge preparation, setup, execution/observation, cancellation and disposal; compare root and checkpoint replay separately. Answer construction currently occurs inside the candidate's `advance`, so the runner must either isolate it consistently or report a joint execution/observation interval.

The strongest ready alternative remains S05 stable-identity operation/failure reuse. Completing the initial S04 comparison first is justified because the gated implementations now permit a direct test of a state-ownership boundary used across search regimes. This is an ordering judgment. S05, adaptive splitting, temporary separation/reunion and the broader architecture decision remain unresolved.

## Validation receipt

[Validation commands and source hashes](s04-source-gate/validation.json) record debug and release tests, strict package Clippy and formatting. Each test process is bounded at 60 seconds. Logs and mutation failures are in [the receipt directory](s04-source-gate/). The reference interpreter is unchanged.
