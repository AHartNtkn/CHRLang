# E16 owned-equation worker gate

The bounded worker protocol preserves the tested source execution: all 512
case/configuration observations and their replay pass. This establishes a usable
experimental boundary for measuring parallel equations. It supplies no speedup or
general fairness claim.

The [registration](../registrations/E16.md) fixes eight configurations over the 64
E00 cases: Shared, Owned, and inline/one-worker/two-worker lookahead with outstanding
limits 1 and 4. Source state and FIFO commits stay on one owner. Workers solve owned
finite-tree equations; completed replies retain their reservation until source commit.
The reply channel has one slot. Explicit shutdown drains accepted work before joining
and does not publish further answers.

## Evidence

The [audited matrix](E16-gate-v2.tsv), [replay](E16-gate-v2-replay.tsv),
[manifest](E16-gate-v2-manifest.json) and [audit](E16-gate-v2-audit.json) check full
answers/residuals, raw completions, failed branches and exhaustion against the
independent reference and fixture expectations. Every source-step trace and FIFO
answer sequence agrees with Owned. At each matched case and outstanding limit,
inline and worker drivers also perform identical aggregate equation work.

All deterministic fields replay exactly. Owner-buffered peaks vary in 9 rows and
worker completion/assignment logs in 43, as allowed by registration. The largest
outstanding count is 4. The matrix leaves at most one accepted request at prefix
shutdown; a focused test separately leaves two, exceeding the reply channel capacity.
Both subprocesses finish within the registered 300-second and 1-GiB address-space
bounds. These debug runs are semantic checks, not performance measurements.

Nine focused tests establish obligations that matrix agreement alone cannot:
reverse receipt preserves FIFO commits; buffered replies consume reservations;
repeated preparation is inert; unknown/duplicate replies are execution errors;
aliases, dependent equations, wakeups and finite-tree failures preserve answers;
prefix cleanup does not commit; two real workers enter the solver before either is
released; bounded-channel shutdown drains; and worker panic reports an execution
error while another worker is alive. The overlap test uses synchronization rather
than elapsed-time ranking.

Independent review identified a missing reference failed-branch assertion. The v2
probe includes it and reruns the entire matrix and replay. The initial diagnostic
matrix and audit remain available as E16-gate files; the v2 receipt is authoritative
for the registered independent-failure check. All 30 chr-reuse integration tests,
crate Clippy with warnings denied, and workspace formatting pass.

## Bounded conclusion and next work

The ownership boundary supports out-of-order equation completion while preserving
this FIFO comparison selector on the tested programs. The selector is not an adopted
language rule. The reference interpreter remains independent.

The protocol retains atomic equation solving, FIFO head-of-line blocking, owned
operand projection and substitution installation. Its outstanding bound counts
requests, not bytes. Owner-buffered peaks omit channel-held or blocked-send results;
those still consume outstanding reservations. Worker hooks used for diagnostic logs
must be absent from timing runs.

Useful follow-ups remain feasible and potentially decision-changing: measure startup,
projection, transport, solving, observation and complete cleanup against both Shared
and Owned; vary task size, available branch width and skew; compare a single chain
with genuine independent alternatives; and investigate certified coarse regions
separately. The current allocator cannot supply reliable phase peaks while workers
are live. Separate uninstrumented System timing from metered construct-through-join
runs and measure subsequent search/output release only after worker quiescence.
This direction remains open and requires no owner decision to continue.

## Reproduction

From the repository root:

```sh
cargo test -p chr-reuse
cargo build -p chr-reuse --example parallel_gate
PYTHONDONTWRITEBYTECODE=1 python3 research/chr-reuse/scripts/parallel_gate.py
cargo clippy -p chr-reuse --all-targets -- -D warnings
cargo fmt --all -- --check
```

The runner records source/toolchain hashes before execution, enforces subprocess
bounds, preserves stdout/stderr, and audits configuration coverage, replay and
matched-driver work. Exact variable representatives need not agree: answers use the
independently validated exact structural observer.
