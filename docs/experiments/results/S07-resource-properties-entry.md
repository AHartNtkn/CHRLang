# Next: compare the language contracts behind resource counting

The resource-counting experiments establish a useful optimization with real applicability boundaries. The next question is whether inference, explicit checked properties or mandatory restrictions change the language and runtime responsibilities enough to matter. This is an investigation of alternatives, not a language adoption decision.

T079 owns a bounded S07 comparison of resource privacy and ground entry modes. Use the actual resource-count compiler and its independent source witnesses, including fuel observers, active entry effects, unknown depth, insufficient resources and branch-specific suspension. Existing interpreter and compiler controls remain authoritative; do not introduce another baseline engine.

First separate semantic necessities from this checker's conservatism. For each accepted and rejected witness, state the property needed for the transformation and whether a different checker or valid reformulation could establish it. A rejected program is not automatically incompatible with optimization or with a proposed language.

Compare three concrete contracts where they differ: inferred eligibility with ordinary execution outside it; optional declarations whose premises are independently checked; and a mandatory-property variant that rejects programs or inputs outside its contract. Declarations are not trusted assertions. Identify whether validation moves to source checking, linking, query construction or execution, and charge that work rather than treating it as free. If two variants have identical obligations, establish that instead of inventing a difference.

Implement the smallest real checking/composition path needed to expose those consequences. Validate accepted examples, invalid declarations, near misses, full observations and any claimed reformulations. Record the programs each option excludes and the source changes needed to recover their observable behavior. No language restriction is adopted without the eventual evidence-backed owner decision.

The first gate must also inventory necessary implementation responsibilities: source analysis, query checking, representation of resources, runtime matching/history, ordinary-path boundaries, preparation and retained artifacts. A declaration can improve predictability without removing a service; distinguish that from actual architectural simplification. Register checking/linking and boundary costs once the alternatives are concrete.

After this bounded gate, reconsider selective conditional discovery, resumable contextual matching and the broader S07/S08 obligations. T078 remains unfinished for complete architecture comparison beyond its bounded pilots. Every other consequential unresolved direction and held-out closure remains required.

The [executable premise audit](S07-resource-premises.md) now establishes checker-conservative examples, a late-ground reformulation, and distinct observation/resource-sufficiency obligations. The [executable contract gate](S07-resource-contract.md) now implements the declarations and owned submission boundary. Their [bounded lifecycle costs](S07-resource-contract-lifecycle.md) are now measured; broader language contracts remain required.
