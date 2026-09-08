# R03 conditional matching and resource commits

Conditional matching and validated source-resource commits now pass independent projection checks. First-use variable slots remain opaque, and consumption is confined to the support proved eligible. T035 remains active: automatic rule selection, body execution and trustworthy publication are not implemented.

## Matching and eligibility

A prepared matcher accepts conditional terms without projecting a machine for each interpretation. A first-use slot retains its original term handle and whole requested support. Constructor tests follow guarded bindings only when structure is demanded; repeated slots use nonbinding equality demand. Joint slot environments retain variable identity. Matching does not mutate equality or allocate bindings to enable a head.

Resource authorization owns the actual prepared source rule and ordered occurrence tuple. It excludes failed and busy support, intersects occurrence liveness, subtracts propagation history when appropriate, and checks heads and equality guards. A caller may restrict the requested region but cannot supply a prevalidated eligibility region. Guard-local variables have distinct rigid identities: reflexive equality can hold without a head binding, while distinct locals and query variables cannot become equal through matching.

An application job yields one eligible symbolic frame. `Finished` means that job emitted its frame; it is not a negative eligibility certificate. Further frames require fresh preparation against current resources after the prior application is serviced. `Ineligible` is reserved for an attempt that found no eligible frame. The source-selection and completion layers must preserve this distinction.

## Atomic resource boundary

Occurrences retain stable identities even when their payloads are equal. A private token records the issuing resource/equality stores and their versions, matched support, slot environment and prepared effects. Commit validates those identities and versions before publishing consumption, propagation history, busy support and a body obligation together under one serial owner. The final commit performs selected tuple/frame work; support operations were completed resumably during authorization.

Only propagation rules use the tuple-history support. Simplification and simpagation consume the declared removed occurrences; kept occurrences retain their liveness. Ordered occurrence tuples remain distinct history keys. One occurrence cannot fill two required head positions.

Pending body supports are disjoint because authorization excludes existing busy support. Acknowledging a body therefore subtracts its support without releasing another pending body. Acknowledgment is explicitly a runtime-owner assertion that the body and its equality obligations finished. The present tests acknowledge only `True` bodies; they do not simulate nontrivial body execution by acknowledgment.

Stale or foreign authorization is rejected before resource effects. Interrupted preparation has no source effects. Equality jobs now also check issuing-store identity, protecting against coincident handles and versions from another store. Raw support/term handles still carry their documented originating-arena/store preconditions.

## Independent checks

The root-owned matcher oracle independently projects input terms and runs an owned-tree nonbinding pattern matcher. It compares both eligibility and every assigned slot, preserving joint unknown-variable identity. Across six guarded states, seven patterns and seven actual terms in two positions, it checks 14,406 matches and 28,812 world/environment projections. A separate witness requires one unpartitioned frame for an opaque slot with conditional values.

The resource oracle exhausts 1,024 combinations of one-birth masks for both head lives, guard matching, failure and prior propagation. Expected effects use integer Boolean operations, independently checking exact history, liveness and body support. Another 256 pairs of two-birth masks check crossing support consumption. These cover 3,072 resource-world projections. History setup and acknowledgment use actual validated applications with `True` bodies.

Focused tests cover foreign/stale tokens, duplicate-valued occurrence identity, distinct body identities, ordered propagation keys, simpagation, rigid guard locals, canceled preparation, disjoint busy support, acknowledgment retaining other pending work, continuation after a rejected guard frame, and fresh preparation after a single emitted frame. The terminal-state distinction was made explicit following independent review so it cannot be mistaken for exhaustive ineligibility.

All 35 package tests pass with default features and counters disabled. Workspace all-target tests, Clippy with warnings denied and formatting pass. [Receipts](R03-conditional-resources/) preserve RED, final package/workspace results and counter-free validation. Independent matcher/resource reviews found no blocking commit or matching defect. No comparative measurements were run.

## Remaining architecture gate

The implementation still needs source activation, rule-order selection, fresh body-local instantiation, supported conjunction/equality/OR execution and completion. Version rejection is conservative and global within these components; it is not a proof that disjoint recursive activity preserves a finite sibling's progress. That support-local property must be established by runtime integration.

A body record is an outstanding obligation, not an executed result. Candidate source traces and full observations must still be checked against an independent scalar oracle before cost registration. The [protocol](R03-conditional-protocol.md) governs those remaining checks. Preparation, symbolic frame copying, guarded support operations, histories, body records and query-scoped retention remain costs to measure; these gates do not establish an efficiency advantage.
