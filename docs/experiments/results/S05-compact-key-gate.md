# Compact keys preserve reuse and reduce allocation, with remaining costs

**Canonical closed-term handles preserve the tested key equivalence and reduce allocation in nearly every measured case.** They also reduce retained key memory. They do not yet establish faster execution or make continuation tables the preferred architecture.

This responds to the [key-cost attribution](S05-history-order-attribution.md), which identified repeated owned term export as a major construction cost. The candidate changes key representation while retaining the same pending-work, resource, output, variable-renaming and history responsibilities.

## Representation and ownership

Owned and compact keys share the state traversal and canonicalization implementation. A compact ground term stores its canonical arena node ID. Nonground terms retain explicit constructor structure and variable identity. If bindings make an open term ground, key construction interns its resolved form, making it equal to the corresponding directly constructed ground term. This can add arena nodes; it is part of the candidate's cost, not a free optimization.

Keys carry machine identity with a retained owner token. IDs from different arenas cannot compare equal, and owner lifetime prevents identity reuse after a machine is dropped. Keys are local to one machine, not portable serialized values. Diagnostic expansion rejects a key owned by another machine.

The existing exact, Alpha and AlphaLive policies each have a compact counterpart. This retains controls for the representation comparison. Complete raw output, logical derivation multiplicity, pending order and live resource/history obligations remain unchanged. Call-level identity transport and generalized caller projection are not implemented by this change.

## Equivalence and source evidence

Four focused tests check expansion to owned keys through bindings/branches, equality relations, a late-grounding versus direct-ground-construction collision, and machine ownership. The late-grounding test matters: simply representing one `f(z)` as a handle and another as expanded syntax would miss valid reuse despite producing the same expanded terms.

All seven continuation modes agree with independent complete raw semantics across 384 configurations: 2,688 executions covering aliases, resources, residual multiplicity, propagation, failures and fresh variables. The lifecycle source gate covers 600 mode/source combinations plus cancellation. All 30 checked owned/compact work signatures agree exactly across the five measurement families and resource absence/presence. Thus the allocation experiment compares the same reuse opportunities, not fewer accepted programs or different answers.

The full persistent/reuse suite passes 69 tests, eight continuation tests pass with metrics disabled, and scoped strict Clippy passes. Reference-interpreter code is unchanged.

## Requested allocation and disposal

The [prospective allocation gate](../registrations/S05-compact-key-allocation-gate.md) completes 410 successful processes and 200 exact allocation replays. All query and prepared requested-live baselines are restored after disposal. No ordinary timing comparison has run; meter elapsed times are not speed evidence.

Compact Alpha and CompactLive reduce requested primary allocation in all 20 configurations. CompactExact reduces it in 19; renamed futures at depth0/resources has a 396-byte increase, from 143,512 to 143,908 bytes. That adverse case remains in the evidence. Compact representation and owner bookkeeping are not automatically cheaper on small states.

Representative depth64/two changed queries/resources/forward:

| Source and policy | Owned requested bytes | Compact requested bytes | Owned peak growth | Compact peak growth |
|---|---:|---:|---:|---:|
| Renamed, Exact | 7,597,784 | 5,864,660 | 2,521,026 | 1,640,002 |
| Renamed, Alpha | 2,867,530 | 2,371,019 | 676,398 | 434,196 |
| Renamed, AlphaLive | 2,224,002 | 1,727,491 | 636,646 | 394,444 |
| Early history, Exact | 7,967,790 | 6,095,512 | 2,634,844 | 1,685,128 |
| Early history, Alpha | 11,475,750 | 9,564,640 | 2,656,395 | 1,687,975 |
| Early history, AlphaLive | 2,474,686 | 1,841,000 | 736,765 | 427,222 |

For the early-history example, direct execution requests 851,822 bytes and specialization 632,924. CompactLive still retains more memory and allocates more than those controls. The representation change addresses a real cost without establishing total efficiency superiority.

Primary allocation includes preparation, setup, execution/complete raw observation and engine/answer/prepared disposal. Source/input construction is separate. Requested heap bytes are not RSS; native compilation is excluded. The shared table representation and generic key traversal are compiled into all compared modes, so this gate is not a standalone feature-off layout comparison.

## Next investigation

The [breadth review](S05-compact-key-breadth-review.md) selects bounded total-cost work with varied reuse opportunity, retaining the owned counterparts and competent execution controls. Four alternative futures limit the amortization opportunity in the current measurements. Test that dimension explicitly before deciding whether these costs reject a mechanism or identify a regime. Do not automatically add more key micro-optimizations.

[Source/binary provenance](s05-compact-key-allocation-gate/freeze.json), [allocation results](s05-compact-key-allocation-gate/summary.json), work logs and the [audit](s05-compact-key-allocation-gate/audit.json) retain evidence. A post-measurement test-output label update names all seven modes; its current source and passing check are recorded separately. Runtime and measured binaries are unchanged. T075 and the architecture goal remain active.
