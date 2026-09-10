# Native host preparation now accepts the common source boundary

The native host frontend reconstructs the same source syntax used by the Rust runner and reproduces all 26 qualified native programs. All 479 prepared queries replay exact native answers and work signatures. This establishes a missing preparation boundary, not an architecture ranking.

## What was tested

The gate decodes common source text into rules and changing queries, emits each ruleset once through the existing native compiler, and extends predicate/atom dictionaries while encoding each query. It compares the reconstructed syntax with independently stored source ASTs, programs byte-for-byte with frozen artifacts, and the common query protocols with their qualified counterparts.

Freshly emitted programs then run through the existing ordinary native wire runner. The 22 common sessions contain 383 queries, including new symbols and cancellation. Four substantive sessions add 96 queries. Every answer byte sequence, service count, pending flag, unsupported flag and dynamic graph-word count matches the prior native record. Complete cases also match the independent source expectations.

The changed-symbol cases use their recorded reference inputs; those describe the changed query and match its stored AST. The syntax equality check enforces this distinction before native execution. Three parser tests cover nested choices, variable identity, empty bodies and malformed structure; the full corpus supplies the broader syntax correspondence check.

## Accounting now exposed

| Frontend interval | Work inside it |
|---|---|
| Decode | Construct rule and query syntax from common source text |
| Emit | Read the native kernel text, generate source-specific code and construct source dictionaries |
| Encode each query | Extend query-only symbols and build its numeric protocol |
| Assemble protocol | Construct the complete prepared-session input |
| Release | Release frontend syntax, emitted program, protocols and dictionaries after validation snapshots no longer retain them |

Each interval has one diagnostic observation per session or query. These are explicitly not comparative samples. Python reference-count release is not proof of RSS restoration or complete allocator reclamation.

The native program is generated from rules with an empty initial query, then reused across changing queries. New query symbols extend host dictionaries without renumbering source symbols. The native backend itself is unchanged; its existing preparation, service and disposal records remain separately available.

## What remains before a fair cost comparison

Input reading, Python startup/imports, artifact writing, host/native transport, external output publication and the combined consumer lifetime are outside these frontend intervals. The run validates fresh artifact execution but does not time artifact writing as part of preparation. Those costs must be included or explicitly separated in the combined lifecycle runner. The frontend and native intervals cannot simply be summed and called process elapsed time.

Separate allocation diagnostics must account for both Python host and native owners, including mappings and allocator scope. Clock overhead remains uncalibrated. Independent user-program compilation and artifact lifetime also remain required for compilation claims. No requested-byte, RSS, speed or total-efficiency conclusion follows from this qualification.

## Architectural consequence and next selection

The native candidate can now enter a comparison from the same admitted source-text boundary as Rust. This removes dependence on prebuilt native programs and preencoded queries as uncharged inputs to that comparison. It does not settle whether this host/runtime split is a desirable architecture.

T078 next integrates those host costs with native transport and output ownership, qualifies allocation scope and calibrates clocks. At that boundary, compare one mixed-source pilot with broader direct source analysis, which could eliminate work the current executors retain. The [experimental sequence](../next-cycle.md) keeps that alternative and all other consequential mechanisms required. The research goal remains active.

## Evidence and reproduction

The [registration](../registrations/S10-host-frontend.md) fixes the corpus, limits, boundaries and interpretation. The [qualification scripts](../../../research/chr-hvm/host_frontend/) provide parser tests, the native replay gate and an archive audit. Run `test_frontend.py`, `gate.py` and `audit.py` from that directory using Python 3.

The [evidence directory](s10-host-frontend/) contains generated artifacts, raw frontend/native observations, parser test results and source/input/binary hashes. Native children retain their registered 15-second wall and 10-second CPU bounds and 96-GiB virtual mapping allowance. Common and substantive service bounds remain 65,536 and 1,048,576 respectively; pending work remains explicit.
