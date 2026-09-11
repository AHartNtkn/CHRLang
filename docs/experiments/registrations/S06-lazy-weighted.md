# Lazy weighted projected output

Replace eager query-result materialization experimentally with an iterator over
existing visible-coordinate assignments and prepared factors. Keep eager output
as a control. Preserve lexicographic weighted tuples, aliases, restrictions,
retained results, cancellation and full exhaustion. Do not expand multiplicities
or change the observation contract.

Validate restriction and assignment bounds at construction as before. Establish
that factor-weight products fit u128 using an upper bound; if that cannot prove
safety, inspect actual products before returning the iterator. Thus overflow still
fails before any output, while ordinary bounded cases need no output-map creation.
Count this check in query setup. Empty domains and annihilating zero factors must
remain correct. Validate both traversals against independent finite assignments
and connected source results, including complete/first output and held tuples.

After correctness, reuse the preceding192 scenarios (three families, six
coordinates, duplicate/plain domains, aliased/plain outputs,4/16/64/256 queries,
immediate/retained answers, complete/cancel). Compare sparse eager, sparse lazy,
Cartesian lazy and enumeration:768 cells. Two allocation and five randomized
ordinary blocks, seed607104:1536 allocation and3840 timing runs. Same full phase
accounting, process bounds and >=10%/all-five-direction rule. Compare sparse lazy
against the other three modes:576 comparisons. Run a portfolio review afterward.
