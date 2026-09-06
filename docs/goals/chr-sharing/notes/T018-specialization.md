# Specialization as an independent optimization direction

Specialization can remove repeated interpretation and known-case work before execution. This is distinct from sharing dynamic work between alternatives, and the two can combine. The relevant question is which substitutions and control transformations preserve the relational program, including failure and residual constraints.

Alpuente et al., [Specialization of Functional Logic Programs Based on Needed Narrowing](https://arxiv.org/pdf/cs/0403011), defines a partial-evaluation scheme for inductively sequential programs. Its strong correctness theorem requires a finite specialization set, independent renaming and closedness of the residual program and goal; computed answers coincide up to renaming. The paper does not give that theorem for arbitrary multiheaded CHR, propagation histories or this project's explicit-OR relation fragment.

## A conservative CHR construction

Start from a single linear catchall relation such as the translated `eval(I,O)`. Specialize it for a known outer constructor skeleton, retaining all free logical variables. Unfold a finite number of its unique catchall expansions, freshening body variables and preserving every explicit OR. Reject an arm only when its posted finite-tree equations are definitely inconsistent with the specialization assumptions. Retain unresolved equations, source constraints and recursive calls. Memoize residual predicate variants by a finite set of structural patterns; when the chosen unfolding allowance is exhausted, residualize the original call rather than continuing compilation indefinitely.

Each retained arm is a finite source derivation prefix followed by its original continuation. Each rejected arm has a finite unification failure proof. Hence the specialization preserves finite successful derivations within the stated input assumptions, with fresh renaming and explicit alternative provenance. No unknown constructor may be treated as a known mismatch. This is a direct construction argument, not an invocation of the needed-narrowing theorem for CHR.

For example, specializing `add(s(A),Y,Z)` removes the zero arm by constructor clash and yields `Z =:= s(B) & add(A,Y,B)`. A variant for `add(X,Y,s(B))` cannot remove the zero arm: it still has `X =:= z & Y =:= s(B)`. Specializing on output shape therefore differs from imposing an input-only mode, and must preserve all relational solutions.

## Limits of inlining active constraints

Inlining is safe only for the closed relation expansion whose intermediate occurrences cannot be intercepted by another rule or observed through source effects. A second rule consuming `eval` would invalidate the unique-expansion premise. Likewise, merging multiple CHR steps into one atomic macro can suppress interleavings with external consumers. Preserve the finite source-step interface, or prove a closed-region/commutation certificate before removing the intermediate boundary.

Unused outputs do not justify eliminating active calls: they may fail or diverge. A compiler can eliminate an operation only after proving the required observational and liveness property under the region contract. Purity alone does not imply totality. Compile-time evaluation must itself be bounded or justified by a termination argument, even for deterministic source rules.

## Candidate costs and language implications

Finite specialization requires no new source semantics. Optional declarations of static arguments or closed relation regions can improve eligibility and diagnostic clarity. A global mandatory mode system would restrict the all-directions addition and synthesis use cases; it is not required by this construction. More general needed-narrowing specialization is credible inside a separately certified functional-logic fragment, with its source assumptions checked explicitly.

Measure compilation work, residual code size, number of variants, runtime rule/matcher work, and interaction with event sharing. A highly specialized evaluator can reduce the apparent benefit of a shared runtime, so every candidate comparison should use equivalent specialization opportunities or report the asymmetry. Aggressive compilation can also inflate code and duplicate common continuations; source size is not evidence of runtime efficiency.

## Disposition

A bounded, semantics-preserving specialization route and its restrictions are established at the paper level. Its profitable depth, variant policy and effect on SK/lambda synthesis require generated residual code and execution measurements. Unrestricted partial evaluation of arbitrary CHR is not needed to exploit this direction; adopting a stronger language fragment would require a separate expressiveness decision.
