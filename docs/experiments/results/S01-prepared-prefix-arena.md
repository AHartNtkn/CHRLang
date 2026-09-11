# Arena sharing helps when queries reuse existing terms

**Sharing the prepared arena reduces allocation in unchanged-table queries, but does not remove the overall peak-memory premium of prepared execution.** Changing right-table terms force a copy and erase the benefit. These results support a conditional mechanism, not a general preference for shared state.

The [registered intervention](../registrations/S01-prepared-prefix-arena.md) ran1,024 isolated allocation processes, with512 exact repetitions and complete ownership restoration. Both independent query-template source gates pass with arena sharing, as does the existing hit/detachment unit test. The source archives confirm unchanged engine and runner code between the ordinary and shared arena measurements; the feature selects the representation.

## Where sharing changes the result

Compared with the same mode using an ordinary arena:

| Mode and family | Comparisons | Requested bytes | Peak live bytes |
|---|---:|---|---|
| Fresh execution, all families |256|Higher in every case|Higher in every case|
| Reused selective/neutral tables |128|Lower in every case|Lower in every case|
| Reused duplicate/broad, changing right table |128|Higher in every case|Higher in every case|

The fresh overhead is232 requested bytes and24 peak bytes per complete two-query lifecycle. In reused duplicate/broad cases the overhead is416 requested bytes and208 peak bytes. Counts span both access methods, both policies, both widths, both probe settings, cancellation and retained outputs; they are not workload frequencies.

For width128, Global scheduling, Indexed access, ordinary partner execution, two complete queries and immediately released answers:

| Reused query | Ordinary requested | Shared requested | Ordinary peak | Shared peak |
|---|---:|---:|---:|---:|
| Selective |996,706|894,778|543,010|492,046|
| Neutral |991,421|889,577|541,350|490,428|
| Duplicate |699,620|700,036|310,719|310,927|
| Broad |906,758|907,174|421,156|421,364|

All quantities are requested heap bytes; peak is above the process root, not RSS. Even with sharing, reused execution has a higher peak than matched fresh execution in all256 comparisons. The selective shared peak492,046 remains well above the ordinary fresh peak321,014.

## Why the result splits this way

`Arena` shares one `Rc` owner containing nodes, interning and predicate dictionaries. Existing-term and predicate hits do not detach it. A new term or predicate invokes `Rc::make_mut`, copying that owner. The unit test directly verifies both paths.

Selective/neutral prefixes contain both tables, including the endpoint terms subsequently requested. Their query suffixes and receipt arguments reuse those terms. At width128 the selective setup phase requests102,296 fewer bytes with sharing; preparation requests368 more, leaving a lifecycle saving of101,928. Execution and observation allocation are unchanged.

Duplicate/broad prefixes contain only the left table. The changing right table introduces endpoint terms absent from the prefix, so setup must detach the arena. The measured setup allocation is48bytes higher over two queries, in addition to368bytes of preparation overhead. The intervention moves the copy rather than avoiding it. This explanation follows the measured phase difference and the actual insertion path; no timing benefit is inferred.

Stores, indexes, pools and queued work still clone at each prepared start. Arena sharing therefore addresses only part of the remaining ownership. Its `Rc` representation also supplies no cross-thread sharing contract; that is a separate implementation choice, not a source-language restriction.

## Decision and next investigation

Retain fresh execution as a serious control. Prepared execution now has a measured allocation benefit on complete queries and a demonstrated cancellation/peak tradeoff. Arena sharing improves the stable-term region and slightly worsens the changing-term region. Neither prepared execution nor sharing is selected as a universal policy.

The next discriminating question is whether these responsibilities change complete ordinary-allocator time. Register a bounded paired, batched lifecycle comparison of fresh/reuse with ordinary/shared arenas, keeping adverse term-changing and cancellation controls. Setup-only timing cannot answer it. This has greater immediate decision value than another copying representation: allocation effects are established, while total speed could reverse their practical interpretation. Integrated admission, fresh graph attribution and direct solving remain required; conduct the full portfolio review after that package, regardless of whether timing separates the variants. Current package count is three; T082 and the goal remain active.

[Per-cell arena comparison](s01-prepared-prefix-arena/arena-comparison.json) · [Source and binary freeze](s01-prepared-prefix-arena/freeze.json) · [Driver and auditor](../../../research/chr-compiled/experiments/prepared_prefix_ownership.py)

The audit additionally verifies full phase order, one sample in each block and unchanged measured source across the two archives. Its post-run cross-archive comparison was added to the current auditor; the execution archive preserves the exact driver used for the run.
