# Research checkpoint review

The two research passes are reviewable checkpoints; M1 is not complete. The dossier establishes concrete mechanisms and language tradeoffs, with unresolved semantic and coverage work explicitly recorded.

## Review performed

On 2026-09-06, a separate research agent reviewed both findings documents against the charter and investigation protocol. The agent had authored the functional-logic source findings, so review of that evidence checks synthesis and scope rather than independently verifying its sources. The PM checked source-record links and incorporated the correction below. No runtime or performance validation was performed.

The review found no other blocking substantive overclaim. It checked active language-design discovery, distinctions between published evidence and proposed transfers, finite-step fairness, residual consistency, and avoidance of premature architecture selection.

## Correction incorporated

The common conditional-lifetime case must test expressibility as well as correctness. A candidate can explain its execution under its own semantics, or state the precise language restriction that excludes it and assess feasible reformulation. The next specification must include candidate semantic changes explicitly. Requiring every candidate to simulate every baseline program would contradict the joint language-design investigation.

## Next research tasks

T008 specifies comparable contracts: adapt the token-aware disjunction reference, state candidate language differences and enabling properties, formulate shared-work correspondence and liveness obligations, and assess the common case. This is justified by the gap between mechanism precedents and an actual correctness target. It does not authorize prototypes or select a representation.

T009 continues citation coverage with attributable searches. Forward retrieval, access gaps and the two-round stopping criterion remain open. The discovery of conflict learning in this pass prevents counting it as a round with no new family.

## Evidence

- [Initial findings](T003-research-findings.md)
- [Second-pass findings](T006-transfer-findings.md)
- [Second-pass source record](T006-source-record.md)
- [Charter](../goal.md) and [protocol](investigation-protocol.md)

The task board records both completed checkpoints and queued research. Completion of these documents does not establish C01 correctness, C02 space savings, C03 shared work in this CHR variant, or C04 net performance benefit.
