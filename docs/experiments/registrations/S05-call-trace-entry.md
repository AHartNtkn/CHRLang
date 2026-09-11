# Call-entry recognition with compressed source progress

Registered before implementation and runs. Test a call-owned transition tree keyed
once at call entry. Store uninterrupted Continue steps as a count; retain explicit
split, failure and answer endpoints. Unfinished traces retain their live cursor.
Each service request advances or replays exactly one original source transition.
A split returns two separately schedulable child jobs. No call runs to completion
inside one step, and cancellation does not force trace completion.

Reuse the existing isolated single-private-head admission and canonical variable
transport. Test repeated and unique calls, fresh output variables, interface alias
near misses, failed alternatives and unequal branch lengths. Compare every event
and answer against the persistent direct stepper under one shared FIFO job queue;
check full bags against the existing independent scalar interpreter. Cancel at
0/1/5 steps and restart using the same partially populated table. Check a foreign
job cannot be stepped with another table. Resource-bearing host integration remains
a required next gate; this first gate exposes a resumable call protocol rather than
claiming the existing atomic caller adapter is sound.

Use recursive depths 0/1/4/8 and both branch orders, duplicate/distinct callers and
two variable namespaces. Bound each run at 100,000 service steps. Require exact
work evidence: one lookup per call, fewer stored trace nodes than source steps on
linear recursion, no execution on complete replay, and actual work for unique keys.
Run default and metrics-off semantic tests and scoped Clippy. No timing verdict
from test duration. At the gate choose actual caller integration versus sparse
projection using the demonstrated protocol and remaining correctness obligations.
