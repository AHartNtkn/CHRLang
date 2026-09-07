# E16 certified-region worker gate

The regional worker protocol passes all 721 finite case/configuration observations
and replay. Q1 reproduces the original E08 per-turn product execution; Q8 workers
reproduce their inline Q8 scheduling control. Thirteen additional focused tests cover
protocol ordering, regional semantics, real worker overlap, failures and cancellation.
This establishes a second experimental parallel boundary without per-equation state
transfer. It supplies no speedup claim.

## Evidence

The [registration](../registrations/E16-regions.md), [v2 matrix](E16-regions-gate-v2.tsv),
[replay](E16-regions-gate-v2-replay.tsv), [manifest](E16-regions-gate-v2-manifest.json)
and [audit](E16-regions-gate-v2-audit.json) cover 103 finite fixtures: 62 E00 cases,
40 controlled products and mixed-add-infer. Each runs baseline E08 plus inline/one-worker/
two-worker execution at Q1 and Q8. The 62 E00 fixtures also run directly through the
independent reference. Explicit Cartesian-product expectations check the larger cases.
The shared fixture module serves both E08 and this probe.

Every finite answer set, raw product count and same-schedule ordered answer sequence
agrees. Per-owner-turn answer counts, cumulative source applications, product jobs,
products, renaming, duplicate and refutation counters agree with the appropriate control.
All deterministic fields replay exactly. Owner-buffered peaks differ in 19 rows;
physical cancellation/work fields do not differ in this finite matrix. The maximum
outstanding count is four. Every child matrix finishes within 300 seconds and 1 GiB.
The initial matrix is retained separately; v2 adds source-snapshot accounting assertions.

All finite matrix cases finish without unaccepted requests at shutdown. That result
cannot establish cancellation behavior. Separate deterministic tests establish:

- A real worker held before service acknowledges cancellation with zero source steps,
  eight unused quantum slots and no logical admission/refutation.
- Cancellation immediately after one real source step performs exactly one step and
  leaves seven slots unused. Its source/observation counters match one-step scalar work.

Both tests would fail if the loop ignored cancellation. Other focused tests force
reverse receipt before logical admission, retain credit after receipt, reject duplicate/
unknown replies, demonstrate two real regional workers entering service together, and
report worker panic as an execution error. They check freshness across residuals and
outputs, repeated output aliases, regional duplicate/raw multiplicity, certificate
coupling, empty-region refutation beside a loop, unfinished regions, and finite product
prefixes from an infinite producer. Prefix/refutation shutdown checks also compare
transport step totals with sums of accepted and actual regional snapshots.

The full chr-factors suite passes 20 tests: seven existing and thirteen regional tests.
Workspace Clippy with warnings denied and formatting pass. The independent reference
algorithm is unchanged; its dependency is used only for experimental validation.

## Ownership and scheduling boundary

Each fixed worker constructs and retains its assigned Rc searches. The owner retains
logical regional completion, cached answers, product jobs, renaming and exact observation.
One request per region and total K bound all unaccepted requests, including received
results. Each owner turn performs one deterministic prefetch pass; receiving replies
does not issue new work. Only the logically selected region can enter product admission.

Request queues have capacity K per worker, with total reservations at most K, and the
shared reply channel has one slot. This bounds submission independently of workers
blocked on replies. Cleanup closes producers and drains before joining. Refutation
cancels worker service immediately while retaining readable exhausted logical state;
explicit shutdown is idempotent and forbids further advance. Workers check cancellation
between atomic source steps. This does not establish a wall-time response bound for
an expensive single source step.

Q8 is an explicit coarse regional schedule. It can change when regional answers become
available and therefore product prefixes. Its worker comparisons use the same inline
schedule; Q1 is the exact E08 correspondence check. Neither quantum is an adopted
language-order rule. The conservative permanent certificate still retains coupled
predicates and shared query variables together; no broader independence is presumed.

## Remaining work and reproduction

Cost measurement remains feasible and necessary. Compare original Factored execution
with matched inline/worker quanta, keeping one-region controls, substantial regional
work, asymmetric work and owner-heavy output products. Measure startup, complete worker
lifecycle, accepted versus actual work, product observation, cancellation and retained
storage. Keep allocator measurements separate from timing. This direction does not
wait for a more efficient owned-equation representation.

```sh
cargo test -p chr-factors
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo build --release -p chr-factors --example parallel_regions_gate
PYTHONDONTWRITEBYTECODE=1 python3 research/chr-factors/scripts/parallel_regions_gate.py
```

The runner preserves existing evidence and requires a separately recorded retry prefix.
It hashes sources and binary, enforces subprocess bounds, retains stdout/stderr, and
checks exact case keys against the earlier E08 fixture inventory. Remaining unknowns
include larger quanta, worker assignment, finer certificates, longer streams and useful
speedup. None is closed by a passing semantic gate.
