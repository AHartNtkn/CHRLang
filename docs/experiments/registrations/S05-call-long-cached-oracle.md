# Cache validation expectations by exact fixture query class

Amendment after the long-entry run completed all 1,428 smaller processes and
multiple successful 8,192-query processes taking roughly 51–53 seconds each.
Execution was deliberately interrupted with terminal status 130 to repair
redundant validation work, not because a sample failed or an observation timed
out. Preserve every terminal receipt and compare its owned bytes with the new run.

This fixture's query varies only by depth pair and the numeric name of its single
variable. Within one ruleset, cache independently validated expected answers by
depth. This is a validation-side cache, never an engine result cache. Direct and
the independent scalar evaluator must agree for each distinct depth pair. Check
every delivered answer against that expectation using the existing exact
alpha-equivalence observation, including multiplicity, residuals and aliases.
Growing-distinct queries still get distinct oracle evaluations. Warm each query
class once rather than repeating all queries before the measured session.

Add a runnable gate that compares cached expectations with independent evaluation
at each actual query for all families and policies, including distant variable
namespaces and fresh outputs. Record oracle-class counts: one for repeated,
min(queries,4) for cycles, queries for growing-distinct. Metadata/expected answers
and phase recorder storage remain outside architectural intervals and heap roots,
as before; all application state and consumer capacity remain inside.

Rerun the same 490 cells and three repetitions from S05-call-long-entry in fresh
artifacts and receipts, with the same bounds and complete ownership checks.
Require exact application allocation agreement with every corresponding terminal
pre-amendment receipt. Losslessly gzip verbose receipts with both compressed and
original byte hashes; retain decoded contents exactly. This gate changes no
language semantics and supplies no comparative timing verdict.
