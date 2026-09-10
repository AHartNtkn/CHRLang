# Bound producer lead after the observed reverse-order regression

The read-only backlog probe reproduces every earlier count and finds30 pending observations at64 answers with the reverse shortcut, versus1 without it. Test whether bounded producer lead prevents this regression while retaining source service whenever only one observation remains.

Add an experimental feature: when two observation objects are pending, service observations until fewer than two remain; otherwise retain the existing alternating producer/observer policy. Do not drain all publication unconditionally. Observations are finite frozen-history jobs; this is a bounded policy experiment, not a chosen default or a proof of arbitrary latency fairness.

First test that the independently checked continuing source reaches64 answers with at most two pending observations, under the shortcut and reverse order. With the feature declared but the policy absent, this must fail. Then implement the policy and run the same source diagnostic forward/reverse with shortcut enabled, twice each. Target128,6M calls,60-second wall/CPU,1GiB address space. Keep all prior adverse receipts. Record calls/stages/supports/variables and pending observations at1/8/32/64/128.

Repeat independent equality, runtime and maintenance tests with the policy enabled, including finite failure and overlapping contexts already covered there. If the bound introduces a correctness or progress defect, repair it before drawing a favorable conclusion. No CPU/wall performance or full lifecycle claim. This is the fourth package since the inert ownership breadth review; a full breadth review is required at its result boundary.
