# Complete caller trace ownership

Register before comparative runs. Reuse the validated caller source generator,
moving depth arguments from fixed launch bodies into query inputs so one prepared
program serves both repeated and changing calls. Revalidate its whole-source gate.
Compare live call traces, Direct, separated stride-one reuse and stride-sixteen
reuse. All start from the same rules and queries; do not compare atomic batching.

Matrix: four modes × four source modes (ordinary, failed alternative, fresh paired
output, binding followed by private work) × depths 4/32/128 × query policies
(one query, four repeated-shape queries, four distinct-depth queries) × retain
none/all × complete/first-answer cancellation: 576 configurations. Alpha-variable
names change on each query; branch depths are n and n+1, or n+2q and n+2q+1 for
distinct requests. Two exact allocation repetitions and one ordinary semantic run
per configuration: 1,728 processes. Seed 607094; 1 GiB address space, 30 CPU seconds,
45 wall seconds per process, 100,000 source steps per query.

Validate complete answers independently before measurement, use Direct for exact
source ordering, check measured prefixes outside measured intervals. Charge source,
preparation, query input/setup, service/observation, consumption, engine and input
disposal, prepared-table disposal and consumer disposal. Allow cross-query table
retention: it must be owned and released, not mistaken for a query leak. Retain
answers beyond all producer disposal before validating them again.

Timing samples from this qualification do not establish speed. Compare exact
requested bytes, reconstructed peak live ownership, query-to-query retention and
consumer bytes. Require identical consumer bytes across all modes and final root
ownership restoration. Any discrepancy blocks later timing and must be repaired.
