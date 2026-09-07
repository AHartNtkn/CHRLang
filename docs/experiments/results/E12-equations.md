# E12 integrated exact equation tables

The alpha-canonical table improves repeated deep equations over uncached owned-tree
unification, but that advantage does not beat the shared-arena control on the
principal repeated workload. Exact table hits alone are insufficient evidence for
an execution-engine benefit.

Code `8f80edb`. The [336-run matrix](E12-equations-v1.jsonl) and
[replay](E12-equations-replay.jsonl) cover 64 E00 cases and 48 registered workloads
in Shared, Owned and Memo modes. Every run passes and every non-time field repeats
exactly. Source steps, raw answers, failures, unique answers and maximum frontier
agree across the three modes. The generated workloads also pass independent
reference checks, including exact expected table hit counts.

| R256 D64 workload | Requested bytes: Shared / Owned / Memo | Peak live requested bytes: Shared / Owned / Memo | Memo hits / calls |
| --- | ---: | ---: | ---: |
| Repeated equation | 16,462,610 / 74,044,178 / 20,195,113 | 8,384,671 / 8,384,670 / 8,392,576 | 255 / 256 |
| Changing ground counter | 16,495,836 / 87,055,580 / 92,318,796 | 8,399,867 / 8,399,866 / 13,363,601 | 0 / 256 |
| Identity equation | 8,844,792 / 13,880,824 / 12,308,440 | 4,275,965 / 4,275,964 / 4,283,619 | 255 / 256 |

For the repeated workload, elapsed microseconds are Shared 17,579/14,653, Owned
115,431/122,091 and Memo 20,840/26,341. Memo computes one equation instead of 256,
reducing owned pair work from 17,152 to 67, but visits 34,304 key nodes and performs
512 replay-node visits. Full operand projection and result installation remain
charged. Shared also executes 17,152 pairs, with a different representation and
without the owned service's repeated whole-tree resolution. Matching pair counts
are not matching instruction costs.

The identity workload needs only 256 cheap same-term pairs in Shared. Memo has
255 hits but still performs 33,280 key-node visits, exports all operands and
replays results. In the unique workload it retains 256 keys without any hits;
final live requested storage is 13,318,577 bytes versus Shared's 8,328,771. Cold
R1 cases likewise have no reuse opportunity. These controls distinguish avoided
operation work from key/interface and retention costs.

## Intended applications

SK duplication evaluation makes 139 owned-service calls and has 12 exact hits.
Memo allocates 984,756 requested bytes versus 808,756 for Owned and 404,366 for
Shared; its peak is 306,408 versus Shared's 174,133. SK ignored-hole evaluation
has 12 hits in 166 calls and similarly higher allocation/retention. The requested
type-synthesis prefix has no hits in four calls. These are bounded workload
observations, not evidence that all equation reuse or call-level tabling is
unhelpful. Prefix agreement does not prove exhaustive synthesis.

## Semantic integration and measurement limits

Owned and Memo receive only the equation actually next in the pending queue,
resolved under the current environment. They return a most-general substitution
for the same input holes or failure. Installation consumes that equation, retains
global hole IDs, interns constructor results and uses the source engine's ordinary
subsequent matching. A focused reference-checked test verifies that a reused
binding awakens a suspended constraint while preserving aliases elsewhere.
All 64 E00 cases preserve full residual answers, raw multiplicity and finite
prefix/exhaustion status; reference implementation algorithms remain independent.

Shared is the existing persistent term-arena unifier. Owned and Memo share the
same exported-term algorithm; their difference isolates the cache. The unifiers
have different internal pair order and variable representative choices, while
source observations agree. Raw `calls` counts describe the external owned service;
zero in Shared does not mean no equations were executed.

The release/fresh-process harness includes initialization, input cloning, operand
projection, key construction/lookup, unification, replay/interning, binding
publication, matching, full outputs/residuals and deduplication. Input workload
construction and expected-answer checking are outside the interval. Requested
live bytes are not RSS, and two repetitions are not precise timing confidence
intervals. Full raw timing and operation/storage counters are retained.

## Disposition

E12's first operation-table comparison is complete; the direction remains open.
Feasible follow-ups include shared-representation keys, identity-based suboperation
reuse, table bounds/reclamation, closed call-level tables with caller filtering,
and longer relational/synthesis sessions. The comparison supports avoiding this
owned-term interface as an assumed performance improvement; it does not rule out
those distinct mechanisms. Continuation and failure-reuse evidence must remain
separate because they save different work and have different hit conditions.

Reproduce with `cargo build --release -p chr-reuse --example equation_probe`, then
`research/chr-reuse/run_equations.py OUTPUT --seed 1231` and another output with
seed 1232. Workspace tests, Clippy and formatting pass.
