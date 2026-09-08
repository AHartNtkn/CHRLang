# Native worker sizing, not confirmation

Use the ordinary-allocator, counter-free `worker_cost` executable to size the next pilot. Start with inline and 1/2/4 workers at depths16/64/256, four changing queries per prepared runtime, quantum16 and window4, on four balanced independent recursive regions. Each region ends in two distinct values; all sixteen complete products and raw multiplicity are checked outside primary timing. Queries vary depth by query-index modulo three.

Run each of the twelve cells once, sequentially, bounded at 60 seconds and 1 GiB address space per process. Record process CPU usage as well as the executable's wall phases. These single observations select safe sizes and expose measurement defects; they do not establish a worker ordering. No comparative confirmation has been registered yet.

The runner measures ruleset construction/preparation, query construction/certification/setup, joint execution and observation, first complete observation relative to execution start, close/drain, query/answer disposal, shutdown and runtime disposal. The current service API does not isolate source execution from product observation, so report their joint interval. Independent answer comparison and result serialization are outside the wall intervals. Process CPU usage includes process startup, validation and reporting; it is a broader endpoint and must be labeled accordingly. Native compilation of this experimental executable is not a measured user-program compilation phase.

A cutoff requires diagnosis before selecting confirmatory sizes. Use this sizing with the concurrent allocation findings to register exact hypotheses, controls, repetitions, cold/reuse levels, CPU/wall endpoints and interpretation before the pilot.
