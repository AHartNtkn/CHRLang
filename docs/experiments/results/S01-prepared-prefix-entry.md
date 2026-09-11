# Reusing query tables preserves the tested consuming choices

**The existing prepared-query API passes the source gate.** Fresh and reused execution produce the same independently expected complete residuals and exact chosen occurrence tuples across selective, neutral, duplicate and broad joins. This makes a lifecycle cost comparison meaningful; it does not yet show an efficiency gain.

| What was tested | Result |
|---|---|
| 4 families × 3 widths × 2 scheduling policies × 2 access methods | 96 configurations per build |
| Two changing endpoint queries, fresh and reused | 384 completed searches per build |
| Cancellation followed by reuse | 96 abandoned searches per build; subsequent queries pass |
| Answers retained beyond template and rules disposal | All complete residual multisets still match |
| Counter-free ordinary and selective-probe builds | Both pass; 768 completed and 192 abandoned searches total |

Every completed search must return exactly one raw answer, no output bindings, the full independently constructed ground residual multiset, and the expected source occurrence tuple. Duplicate and broad cases offer multiple possible consuming matches, so matching only the final count would not pass. Both paths receive identical table-first source order. These results do not assert equivalence to a differently ordered source program.

Selective and neutral queries reuse both tables. Duplicate and broad queries reuse the left table and supply changing right rows in each suffix. The fixture therefore exercises actual invalidation boundaries rather than silently retaining changed input. The existing nonground, generated/generic query-template regression also passes in both builds.

## What required correction

The initial command applied the 1 GiB runtime address-space cap to compilation and failed in the compiler/linker. Compilation now precedes bounded execution. The first source assertion compared an insertion-ordered answer with a sorted expectation; the checker now sorts the observed residual, retaining multiplicity and checking occurrence traces without sorting them. Both failed attempts are retained in the evidence directory. Neither failure establishes an engine defect.

## Architectural consequence and next experiment

Proceed to the separately registered requested-allocation and ownership comparison. Charge reusable prefix construction, store cloning, changing suffix admission, execution, observation, cancellation, retained answers and disposal. The current representation clones an engine at each start; avoiding initial admission may still cost more overall. Measure that tradeoff before interpreting ordinary timings.

This gate required no production API change. Ownership accounting is now a cheaper prerequisite than a new integrated admission representation, fresh graph attribution or broader direct solving, while resolving a directly measured repeated-query cost. Those alternatives remain open and must be reconsidered at the ownership result. Package count is one since the full portfolio review; T082 and the research goal remain active.

[Registration](../registrations/S01-prepared-prefix-entry.md) · [Runnable source gate](../../../research/chr-compiled/tests/query_template.rs) · [Evidence](s01-prepared-prefix-entry/)
