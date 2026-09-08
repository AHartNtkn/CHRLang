# Carrier complete-cost registration (T054)

This comparison follows the semantic gate at `10bed 6e`. Run only after the
feature-isolated incumbent, runner, independent unknown-tail oracle and audit
checks are validated and committed. The decision is whether valid source lowering
changes the remaining bounded Conditional advantage after complete preparation,
query, service and disposal costs.

## Hypotheses and priority

Carrier contraction can avoid intermediate occurrence/index maintenance and
selection. Contrary explanations are that certificate preparation, resumable
inspection and job ownership cost more than the saved work, particularly for
short or rejected controls. Arena sharing is a separate saving and must not be
credited to contraction. Conditional can still benefit from sharing physical work
across assignments, but pure countdown work need not be intrinsically necessary.

The successful semantic gate and precise remaining positive placement give this
comparison higher decision value than more storage tuning, a broader contextual
certificate, or additional repetitions of a historical comparison. No workload
weights or universal winner are inferred.

## Configurations and source cells

Build each configuration separately, avoiding Cargo feature unification across
controls:

| configuration | backend | extra features |
|---|---|---|
| conditional | Conditional | none |
| specialized | Specialized | none |
| specialized-cow | Specialized | arena-cow |
| carrier | Specialized with inferred contraction | carrier-contraction |
| carrier-cow | Specialized with inferred contraction | carrier-contraction,arena-cow |

Feature-off builds contain no carrier job/plan fields, statistics, APIs or service
hooks. Compiler type/member and IR evidence validates that boundary. Candidate
preparation charges inference and ownership of the source plans. Every explicit
configuration uses inferred single-head specialization, Global source selection
and indexed access. All preserve raw answers and terminal arbitration.

Ground cells use the existing independently validated mixed source with placements
(pre,post)=(0,0),(0,16),(8,8),(16,0),(0,64),(32,32),(64,0), each with q=1,4.
Query i increments both depths by i%2. It makes four binary assignments, performs
pure pre-work, discriminates the tuple, performs post-work and publishes the exact
two aliased residuals. There are 16 raw answers for each query.

Two additional unknown-tail cells use pre 64/post 0 with q=1,4. Query i starts with
`s^(64+i%2)(U)` and keeps post zero. The full expected answer is one
`pre(U,z,tuple,H)` residual per binary tuple, with U and H distinct unknowns and no
outputs. This checks rejected admission without assuming a ground base. All 16
keys occur exactly once; neither binding the control nor identifying the unknowns
is acceptable. The independent structural oracle has adverse mutation checks.

These are 16 source/reuse cells ×5 configurations =80 cells. Run one warmup, five
primary repetitions, one allocation and one work process per cell:640 processes.
Use Python Random seed 54054 to shuffle each full mode batch, modes warmup,
primary, allocation, work in that order. Keep every registered placement and
outcome; do not resize or repeat based on favorable timing results.

## Measurement and resource controls

Build 15 locked offline release artifacts in separate target directories. Primary
uses `--no-default-features --features experiment` plus the configuration's extra
features; allocation uses `--no-default-features --features alloc-meter` plus
extras; work enables defaults with `--features experiment` plus extras. Engine,
kernel and observer flags are reported and must agree. Primary uses ordinary
allocation, no counters and no trace/audit expansion. Diagnostic builds/runs are
separate; requested heap is not RSS.

Charge source cloning, specialization/certificate preparation, per-query source
construction, setup, execution with full observation and terminal-branch disposal,
engine disposal, retained-answer disposal and prepared disposal. Report these
phases and first full observation separately. First service observation starts at
execution; first request observation also includes query construction and setup. Validation is outside measured
intervals and warms subsequent disposal. Fixed fixture/record storage is the
baseline, not a prebuilt query cache. Query construction now enters the measured
lifecycle, so no timing comparison to the T048 freeze is valid. Compilation costs
are not credibly isolated here; no complete architectural superiority claim.

Pin each process to the minimum inherited available CPU; impose a 30-second wall
timeout and 1-GiB address-space limit. Each query permits 20 million ticks. Each
build has 180 seconds; execution has 1200 seconds after builds. Record planned jobs,
commands, host/toolchain, build states and source/registration/binary hashes before
execution; verify hashes afterward. Run allocator self-check first. Retain failed,
malformed, timed-out and incomplete rows without relabeling them successful.

## Acceptance and interpretation

Require complete 16-answer raw multisets, full aliases/residuals, exhaustion,
configuration/command correspondence and allocation baseline restoration. Check
logical applications independently: ground Conditional pre+16*post+34, ground
explicit 1+16*(pre+post+3); unknown Conditional pre+1, unknown explicit 1+16*pre.
Candidate logical application counts remain those of the corresponding source.
They are not physical dispatch counts.

Ground carrier configurations have 16*(max(pre-1,0)+max(post-1,0)) certified omitted
steps; unknown controls have zero. Carrier inspection counts are 16*(pre+post), with
post zero in unknown cells. Feature-off configurations report zero carrier work.
These predictions apply to each changed query and distinguish successful
contraction from merely selecting a plan.

Report all 80 cells with five-repetition medians and full ranges, allocation traffic,
peak requested heap, source work, first observation and phase costs. A timing
advantage is resolved only when full repetition ranges separate; overlap stays
unresolved. Ranges are a conservative within-session screen, not confidence
intervals. Compare contraction against each matching arena control, and compare
complete explicit candidates against Conditional. Do not multiply component ratios.

If contraction changes the positive pre-heavy comparison, update the interpretation
of that sharing evidence and retain any contrary small, post-heavy or rejected
cases. If its complete benefit is absent, retain the negative result rather than
broadening the certificate to seek a winner. After this evidence, reassess overall
sufficiency and every remaining direction against concrete decision value.
