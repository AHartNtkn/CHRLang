# Direct long-reuse test of native compilation amortization

The first cost pilot measured native runtime gains but no compilation-inclusive
benefit. Its ordinary-link models predict break-even near 2,114 chain and 4,751
payload queries. Test those predictions directly, without extrapolating a result.

Use ordinary linking, chain size 32 at 1,024/8,192/16,384 queries and payload size 64
at 1,024/8,192 queries. Compare planned and native execution, alternating the same
query inputs as the pilot. Raise only the runner's query-count ceiling from 1,024
to 16,384; source, engine, oracle and phase semantics remain unchanged. ThinLTO's
larger modeled break-even is outside this bounded contrast and remains unresolved.

Rebuild ordinary and diagnostic runtimes separately. Compile generic, chain and
payload artifacts with edition 2024, opt-level 3, codegen-units 1 and lto=off. Use five
fresh ordinary artifacts/emissions per family and one diagnostic artifact. Measure
emission, compilation wall/child CPU, source/binary sizes and artifact filesystem
disposal. Generic compilation is reusable across rulesets and is not a per-ruleset
charge; native compilation is. Fixed runtime construction remains separate.

Use one warmup block then seven measured blocks, all ten runtime cells in each.
Shuffle within blocks using seed7200+block, and pin runtime children to the smallest
allowed CPU. Run each cell twice diagnostically after primary timing, requiring
exact allocation replay and query/prepared live-byte restoration. All completed
answers must match the independent scalar oracle outside runtime timing. This is
80 ordinary and 20 diagnostic processes, 696,320 complete query checks if all finish.

Each process/build/compile has a 60-second timeout; compiler address space 4 GiB and
runtime 1 GiB. Per-query engine/oracle bound remains 2 million steps. The larger query
count is justified by the measured crossover models, not an architecture change.
Any cutoff is preserved and diagnosed; do not replace it with a completed cost.

Report seven paired runtime ratios, and compilation-inclusive ratios using the
median of five measured native artifact costs, including filesystem disposal. A
practical gain requires median<=0.90 and all seven ratios<1; practical loss requires
median>=1.10 and all seven>1. Otherwise leave the practical comparison unresolved.
Separately report whether all observed native runtime+artifact totals are below
all observed planned runtime totals; that supports observed amortization even when
the gain is below 10%. State the measured query count, never a universal crossover.
Keep the lower-reuse contrary case visible. No cross-family averaging or weights.

Process wall/CPU, primary phase sums, requested allocation and absolute process
requested-live peaks remain separate. Input/oracle construction, validation and
stdout are excluded from runtime phase sums. Compiler artifacts include the shared
experimental runner/oracle; these costs are not minimal production lower bounds.
Filesystem disposal is syscall accounting, not storage synchronization. This tests
released-answer sequential queries, not streams retaining outputs or arbitrary
cross-query caches.

After this bounded comparison, reassess direct pull-tabbing/derivation reuse against
any still consequential attribution. A native win, loss or narrow overlap cannot
resolve the broader architecture. This experiment receives priority because it can
reverse the present compilation-inclusive observation with an existing correct path.
