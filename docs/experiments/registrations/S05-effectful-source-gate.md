# Effectful caller reuse: dependency and scheduling source gate

T075 tests two independent assumptions before designing a broader reusable call boundary: whether call arguments identify resource-dependent results, and whether collecting an isolated phase preserves whole-source effects. This is the first package after the adaptive cost breadth review. Experimental implementation and runs are authorized; no language restriction or architecture is selected.

## Candidates and controls

Use a bounded experimental contraction probe, outside the production library. It executes selected source rules over a projected initial query with the existing persistent machine, collects complete residuals and interface bindings, freshens them, and resumes the complete caller. Compare uncached contraction, call-only keys and complete ordered projected-region keys. Include every initial query variable in the exported interface so caller aliases cannot disappear; cached outputs and residuals share one fresh import namespace. Preserve resource multiplicity and order in region keys. Separate these mechanisms from the independent scalar evaluator and direct persistent whole-source control.

H1: call-only keys can replay stale resource values or multiplicities. H2: complete region keys can qualify finite rule-priority phases with consuming/kept heads and fresh outputs across different callers. H3: region keys alone cannot establish scheduling, occurrence order or propagation-history validity. An uncached contraction mismatch diagnoses the phase boundary independently of memo recognition. A region-only mismatch absent in uncached contraction diagnoses key/import validity.

## Source matrix and observations

The favorable/adverse resource matrix uses recursive work depths 0, 1 and 4; one or two identical result alternatives; three caller variable namespaces; four token conditions (a, b, absent, a then b); present or absent kept facts; two caller tags. This gives 288 queries per candidate, using one table per depth/alternative configuration across changed resources and caller identities. Check every raw answer, alias and residual multiplicity independently. Run direct persistent and all three contractions; classify call-only mismatches rather than assuming their count. Require uncached and region agreement in this source matrix or diagnose the failure before any cost proposal.

Separately execute explicit challenges for an intervening competing consumer, an intermediate-effect observer, a late supplied binding that changes rule eligibility, surviving propagation history, and relative occurrence order at the caller boundary. These are hypotheses about unchecked contraction, not admitted programs of an asserted general optimizer. Record actual independent outputs and rule traces; do not disguise a mismatch as an eligibility rejection. Use source variants with and without the interference to isolate causes.

For fresh identity and cancellation, reuse a prepared table after dropping an unfinished caller and hold prior owned answers while subsequent queries run. Test absent resource followed by supplied resource and changed aliases. A complete independent oracle comparison outside any timing must establish the outputs; finite checks do not prove arbitrary continuation validity.

## Bounds, registration and interpretation

Freeze this registration before implementation/runs. Test first meaningful transport/dependency requirements before implementing the probe. Run deterministic semantic tests once in default metrics and once without default features, with 200,000 service calls per run and 60 seconds wall/CPU per test executable, 1 GiB address space after compilation. No comparative timing or RSS claims. Preserve build/test outputs and source hashes, including failed assumptions and repairs. Counts/hits are diagnostic only; report memo hits only when measured with metrics enabled.

A counterexample rejects only its explicit transformation. A qualified complete phase supports a candidate eligible fragment, not general inference. Investigate failures with source traces and distinctions between keys, ordering, history and caller effects. At the gate, compare a checked/resumable effect implementation with targeted adaptive timing attribution, native local ownership, conditional lifetime repair and incremental projection. Wider dependency inference and whole-architecture efficiency remain required. No costs run until the semantic boundary and its actual owner obligations qualify.
