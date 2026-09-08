# Next: restore or recompute search state instead of preserving whole branches

S04 will compare mutable state restoration and replay with credible copying/persistent controls. The architecture question is whether preserving state through mutation logs or recomputation earns its machinery across different search shapes. The matching and finite-compilation results do not answer that ownership question.

## The boundary to investigate

Existing persistent/compiled execution already provides explicit branch ownership, immutable term structures and fork controls. R04 also uses a trailed finite solver, but its state is a finite-domain relation, not general CHR execution. Neither supplies a complete CHR restoration/replay comparison under the renewed sequence.

The candidate must restore bindings, constructor information, consumed occurrence liveness, propagation history, fresh identities and pending source effects. Successful rollback of equality alone is insufficient. Compare complete answers and residual multiplicity with an independent oracle, including a failed branch followed by a sibling that needs the original resources and aliases.

Fair service is part of the comparison. A single depth-first trail cannot be treated as an equivalent control if it starves a finite sibling beside ongoing work. Assess reversible changes, checkpoint/replay or separate trails as actual organizations, charging their switching and retention costs. Do not impose the existing copied-branch API if it excludes a competent alternative.

## Entry work and gates

First inspect the current fork and mutation surfaces and register a bounded source contract. Design the smallest complete restoration and replay paths that preserve that contract, with copying over equivalent primitives where possible. Independent tests must expose a missing undo of each semantically relevant state component. Only after restoration and finite-sibling service pass should a comparative lifecycle matrix run.

Vary depth, frontier width, mutation density, failure placement and useful work between choices independently. Count output retention separately from saved search state. Include cancellation, preparation reuse and final disposal. A complete restoration candidate may still lose because replay or switching costs dominate; those are results to measure rather than premises for rejecting it.

The strongest ready alternative is S05's stable-identity equation/failure cache. It could distinguish direct sharing from economical recognition of repeated work, and existing equality controls provide a starting point. S04 is selected first because search-state ownership participates in both low- and high-reuse programs and its fair-service requirements can alter the whole engine organization. This is a prioritization judgment, not evidence against caching. S05 remains required and must be reassessed after the bounded restoration trial.

T066 owns the source/restoration gate and ensuing prospective comparison. Broader adaptive splitting and temporary separation/reunion remain S04 obligations even after an initial trail/replay matrix. No architecture is selected in advance.
