# Arena sharing under prepared-query reuse

Registered before source qualification and comparative runs. T082, package three since portfolio review.

**Question:** does sharing the prepared term arena reduce the measured ownership premium after changing suffixes and all disposal count? Existing `arena-cow` shares the entire arena data owner and copies on the first new node or predicate. Stores, pools and indexes still clone. Hypothesis: unchanged-table cases with only intern hits can benefit; new right-table terms can force detachment and erase the benefit. Treat this as a component intervention, not a language requirement or a general sharing result.

First run the existing independent `query_template` source tests in counter-free arena-cow and arena-cow+selective-probe builds, including nonground/generated regression, exact ground tuples, cancellation and retained outputs. Also run the existing arena hit/detachment unit test. Failures require investigation before costs.

Then repeat the entire prepared-prefix ownership matrix with arena-cow: 512 cells, two isolated shuffled blocks (seed8211), 1024 processes. Same four families, widths16/128, policies, access, lifetime/cancellation controls and fresh/reuse modes. Use the same runner, one-tick cancellation, allocation meter and outside-interval full-answer oracle. Keep counters off, 1GiB address space,60 CPU seconds,60 wall seconds and2,000,000 ticks; compile before bounds. Freeze sources and copied binaries before execution. No timing conclusions from metered durations.

Match each cell with the previously frozen ordinary-arena matrix, whose engine source is unchanged. Require exact heap repeats, full phase coverage and restoration. Report requested bytes and peak above root for both fresh and reuse, not merely clone cost. A consistent difference can establish a scoped ownership effect. Inspect the insertion path to distinguish shared hits from forced copies; run a focused diagnostic if results contradict that mechanism. Reconsider integrated admission, graph attribution and direct solving at the result; full portfolio review within four packages.
