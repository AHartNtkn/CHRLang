# A sole replay branch now continues without rebuilding its prefix

The replay engine now keeps the sole remaining branch as live state instead of reconstructing it at every service call. A continuing deterministic source drops from **5,050 actual source steps to 100 for 100 public service calls**, with service behavior preserved. This corrects a repeated-work cost; it does not establish replay's total efficiency or resolve temporary reunion.

## What changed and why it is sound

The earlier path cloned its checkpoint and replayed the recorded prefix before every new source step. On a continuing deterministic branch, the prefixes grow by one each time: 100 public calls perform `1 + … + 100 = 5,050` actual source steps. The registered failing diagnostic observed exactly that count.

A branch with no siblings now lives in the engine's working-state slot. That slot and the saved-recipe queue are mutually exclusive. Continuing the sole branch needs neither a new snapshot nor a checkpoint prefix. When a saved branch becomes sole after progress, its restored state moves into that slot and its unnecessary history is released.

When the resident branch forks, the engine captures the complete state immediately before its pending choice and queues the two child recipes in the existing order. Each recipe reconstructs the selected alternative before its next new source step. Competing saved recipes still use the configured checkpoint interval. Bindings, occurrence identities, propagation history, fresh counters and pending work belong to the saved state; none are inferred from output equality alone.

The working state lives outside the recipe queue, so saved recipes do not each need space for a possible full resident state. The existing Copy and Trail paths are unchanged. No reference interpreter or scalar oracle code was modified.

## The source and ownership gates pass

The [registration](../registrations/S04-resident-replay-source-gate.md) fixes the ownership rule, failing diagnostic, service contract and validation requirements. The current package passes **nine ordinary tests and ten diagnostic-feature tests**, with strict all-target/all-feature Clippy.

The finite source suite now compares **exact public event traces on 48 cases across six modes**: Copy, Trail, Replay and checkpoint intervals 1, 4 and 16. This checks progress events and answer order as well as complete answers. The cases include failed speculative bindings, finite-tree cycles, duplicate alternatives, propagation history, consuming/kept occurrences, fresh variables, guards, nested choices and pending tails. Complete answers also agree with the independent scalar evaluator and applicable compiled controls.

The ongoing-sibling test compares the first **100 public events** against copying and still delivers the finite sibling's answer without reporting false exhaustion. Cancelled queries do not contaminate subsequent queries over the prepared rules.

Weak-owner checks observe a shared checkpoint at a fork, then prove that it is released after a failing sibling leaves a sole continuing branch. Separate checks require deterministic checkpoint execution to retain a working state with no pending saved recipes. Existing reversible-state tests continue to check undo/redo of all mutable fields. These are semantic ownership checks, not requested-heap or RSS measurements.

## Competing branches still expose real restoration work

The existing lifecycle diagnostic completes all **72 cells: twelve source families and six restoration modes**. Copy and Trail actual source-step counts agree with service counts; Replay agrees with an independently copied-state projection of its reconstruction policy. The projection now accounts for resident continuation and saved prefixes at branch points. Diagnostic increments are confined to the separate `replay-diagnostic` build.

| Source family | Copy source steps | Replay source steps | Checkpoint 1 | Checkpoint 16 |
|---|---:|---:|---:|---:|
| Linear | 37 | 37 | 37 | 37 |
| Retained store | 164 | 3,780 | 178 | 1,412 |
| Work between choices | 1,004 | 139,320 | 1,018 | 8,376 |
| Deep choices | 1,452 | 76,492 | 1,578 | 12,428 |

The linear case now performs no reconstruction. Branching cases still reconstruct saved prefixes under FIFO service. Frequent checkpoints reduce that work but require snapshot construction and retention. Source-step counts alone cannot decide whether those snapshots repay their cost; neither allocation nor ordinary timing has been measured for this change.

The correction does not batch source work, alter the public scheduler or claim a constant wall-time service quantum. A saved replay service can still reconstruct a long prefix. The fair-sibling result is about the declared source-service contract and its explicit finite observation window.

## Next investigation

Keep T077 active for a bounded complete-cost comparison using the existing replay/checkpoint controls, including intervals beyond one, read-heavy and mutation-heavy state, prepared-rule reuse, first/full observation and cancellation/disposal. Register ordinary timing and separate requested-allocation diagnostics before interpreting costs. The code and harness already exist, so this can determine whether the avoided work changes total cost without another executor baseline.

Actual temporary separation and reunion remains required independently. Reusing a sole state does not separate components or reconnect their resources. Integrated dependency repair remains the strongest separate alternative after the restoration-cost boundary; its representation and consuming-source questions are not answered here.

This is T077's first bounded package. It supplies a more credible restoration control for later architectural comparison, while leaving adaptive scheduling, reconnection, lifetime and whole-architecture obligations open.

## Reproduce the evidence

Run `cargo test -p chr-restoration`, repeat with `--features replay-diagnostic`, and run `cargo clippy -p chr-restoration --all-targets --all-features --no-deps -- -D warnings`. The broader work command is `cargo run -p chr-restoration --release --features replay-diagnostic --example lifecycle -- work`; its diagnostic build rejects timing mode.

[Failing work test](s04-resident-replay-source-gate/red.log), [ordinary tests](s04-resident-replay-source-gate/tests.log), [diagnostic tests](s04-resident-replay-source-gate/tests-diagnostic.log), [work cells](s04-resident-replay-source-gate/work.jsonl), [audit](s04-resident-replay-source-gate/audit.json), [source hashes](s04-resident-replay-source-gate/freeze.sha256), [patch](s04-resident-replay-source-gate/source.patch) and [Clippy](s04-resident-replay-source-gate/clippy.log) preserve the result and its scope.
