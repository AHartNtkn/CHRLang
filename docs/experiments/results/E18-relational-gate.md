# E18 integrated relational semantic entry

The generic relational prototype passes432finite equation recipes against a
separate tree-unification oracle:170successes and262failures. All results and
work counters replay exactly across216fresh isolated children. Sixteen focused
tests pass, including constructor/application queries enabled by equality,
correlated swapped bindings, and equality-only cycles that must remain valid.

Evidence: [registration](../registrations/E18-relational-gate.md),
[v3 raw records](E18-relational-gate-v3.jsonl),
[pre-run manifest](E18-relational-gate-v3-manifest.json),
[audit](E18-relational-gate-v3-audit.json),
[written correspondence](E18-correspondence.md), and
[primary-source assessment](E18-source-assessment.md).
Code is in `research/chr-relational/`. The v3 records are authoritative; v2 records
are diagnostic with post-run source hashing. Two additional hand tests were added
after v3 without changing any of its frozen input files.

## What changed architecturally

The kernel operates only on flat predicates, ordered ID ports, rule joins and
finite bitset supports. Equality equivalence, constructor descriptions,
injectivity, congruence, disjointness and positive-path cycle detection are
ordinary generated rules. Application rules can introduce and inspect those
same facts during the same closure. There is no complete-unifier call, privileged
constructor tag dispatch or solved substitution in the kernel. Syntax lowering
and trusted final extraction are separate from that generic execution substrate.

This establishes a finite monotone denotational entry for A1. It is more than
encoding a unifier behind the existing request boundary. It does not establish
full CHR resource semantics, efficient physical fusion, general branch creation
or better sharing/performance. The existing fine-grained conditional kernel and
shared arena remain necessary controls for those claims.

## Discriminating findings

Equation-only conformance was insufficient for integration. From f(1,2), eq(0,1)
and eq(2,3), an ordinary rule with premise f(0,3) must become applicable. That
regression required constructor-column equality transport, alongside transport
for ordinary application values. It now passes using ordinary rules. Value
transport is currently an explicit schema obligation: identity columns must be
excluded and application value columns included. The kernel itself does not
validate an automatic source compiler's schema.

The context checks preserve (a,b) or (b,a) without producing cross combinations.
A cyclic union graph with acyclic projections succeeds in both contexts; a real
projected constructor cycle fails only its support. An equality-only cycle is
valid. Unknown constructor queries do not manufacture descriptors. Extraction
requires completed closure and checks bad support before producing joint terms.

Maximum universe size is20IDs, maximum fact count271, and maximum closure depth
four rounds. The largest batch performs1,162,190join attempts. This exposes the
cost of a transparent explicit equality/transport closure even at small sizes;
it is not a comparison with a competent equality-class implementation. Maximum
child wall time is0.426seconds, below30seconds; no child hit the1GiB address-space
bound or failed. Pre-run hashes remain unchanged and the independent audit checks
all108batch identities,432recipes, failure masks and exact replay.

## Remaining work and selection

A1 remains open. Next full-source obligations are supported consumption,
propagation-once, fresh locals, dynamic explicit choices, residual extraction and
legal interleaving of source effects with unfinished consistency. They must not
be implemented by requiring a solved unifier result at every step. Efficient
class/index representations, same-representation service versus integrated
controls, partially overlapping operations and intended applications remain
feasible follow-ups.

A3 maintained joins and A6 observation-before-export have independent concrete
entries in [the preparation note](../../goals/chr-experiments/notes/T018-independent-entries.md).
They need no positive result from A1. The architecture register stays active;
this semantic entry does not discharge its other assumptions or select an engine.
