# Rejecting incompatible tuples earlier has mixed work costs

Checking choice compatibility before candidate registration avoids impossible candidates and reduces service work on an early-failure source. It adds work on independent choices, even when it rejects some candidates. Partial-join pruning remains a distinct, unimplemented comparison; no lifecycle advantage is established yet.

This implements the direct tuple control selected in the [support-join entry](S10-support-join-entry.md). The experimental `support-join` feature implies the existing per-head `selective-discovery` filter. It is not a default-policy change.

## Why rejection is safe at this boundary

An occurrence's live condition only shrinks, and its identity is never reused. Resource consumption updates it by Boolean difference. New posts get fresh identities and their own discovery work. Therefore, if the conjunction of a tuple's live conditions is false, those particular occurrences can never coexist later.

Discovery drains before rule execution resumes, so the conditions remain stable while the new check runs. Boolean operations use the existing resumable support jobs; discovery performs one operation step per service call. The check retains the tuple, current conjunction and job while pending. It registers a candidate only after proving the conjunction nonempty.

The check does not use current bindings, guards, remaining scheduling scope or temporary busy resources to reject a tuple. Compatible tuples retain the original anchor activation scope and go through existing matching/dependency registration and execution-time validation. Late information can still enable them. This is a necessary compatibility test, not an eligibility certificate.

The implementation avoids Boolean jobs for an unconditional occurrence or one with the current condition. When every occurrence is unconditional or has the anchor's nonempty condition, it registers directly. This includes unary rules and the dense unconditional control. The earlier version performed these redundant jobs; correcting that issue reduced overhead before the reported comparison.

## What the work counters show

“Candidates” means distinct tuples registered for later processing. “Steps” means public engine service calls, not elapsed time or CPU instructions. The left control already filters dead occurrences and immutable head mismatches.

| Source | Existing candidates | New candidates | Existing steps | New steps |
|---|---:|---:|---:|---:|
| Two mutually exclusive head occurrences | 2 | 1 | 146 | 142 |
| Two correlated pairs | 5 | 3 | 432 | 423 |
| Two independent binary choices | 5 | 5 | 883 | 927 |
| Dense six-occurrence propagation | 30 | 30 | 8,556 | 8,556 |
| Common source, choices=3/depth=16 | 139 | 139 | 4,301 | 4,301 |
| Independent source, choices=3/depth=16 | 1,629 | 1,512 | 34,960 | 35,295 |
| Early-failure source, choices=3/depth=16 | 259 | 139 | 5,845 | 5,125 |

**Fewer candidate records do not guarantee less total work.** The independent source avoids 117 registered tuples but needs 335 more service calls. Compatible tuples still pass the existing execution-time live check; early checking adds a responsibility and can duplicate support work. A later lifecycle comparison must charge Boolean job construction, arena nodes, temporary tuples and disposal.

**The favorable case removes work earlier in the pipeline.** Early failure avoids 120 registered tuples and 720 service calls. Matching/selection already checked live support; this experiment moves rejection ahead of candidate and pending registration. It does not prove that a new kind of matching result becomes available.

**Direct tuple rejection still enumerates full combinations.** It constructs each candidate tuple before testing compatibility. It may also recheck an impossible tuple through another anchor because rejected tuples are not retained in the known-candidate set. Retaining rejection records would trade repeated checks for storage; no cache is assumed beneficial. Partial-prefix pruning could avoid whole suffix products, so the present result cannot settle that alternative.

## Correctness and verification

The new source test constructs complete expected residuals for mutually exclusive, correlated and independent choices. Each agrees with the independent scalar, conventional Scan and conditional execution. It preserves occurrence multiplicity and choice correlation rather than comparing only answer counts. The initial test failed on the expected impossible-candidate count before implementation.

All 123 crate tests pass with support joining. They include consuming resources, repeated variables, supported equality, late activation, source priority, mixed mechanisms and publication beside continuing work. A separate counter-free run passes 23 selected source/resource/broad/progress tests. The existing filtered control passes the new source witnesses, and strict scoped Clippy and explicit formatting pass. [Logs and source hashes](s10-support-join-gate/) record the checks.

These are bounded executable witnesses plus the monotone-support argument. They do not constitute a general theorem about CHR transformations, a sustained-lifetime study or allocation/timing evidence. No reference interpreter code changes.

## Next comparison

Implement a bounded partial-prefix traversal control before registering lifecycle measurements. Use at least three heads and an independently varying suffix population so an incompatible prefix can eliminate many combinations. Include fully compatible dense products, independent/correlated conditions, late bindings, consumed occurrences and multiple discovery anchors. Preserve candidate order among surviving tuples and the existing dependency/scheduling contract.

Compare direct tuple checking with prefix pruning explicitly; do not turn them into one undifferentiated “support-aware” result. Measure retained traversal/support state and cancellation ownership if the source/work gate succeeds. Broader equality relevance is still a ready alternative, but prefix pruning answers the remaining mechanism distinction in this selected package before another refinement cycle. T078 and the full architecture goal remain active.
