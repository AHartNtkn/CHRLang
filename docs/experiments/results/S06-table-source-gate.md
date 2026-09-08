# Finite table choices can compile into joins while preserving source answers

The new compiler extracts finite relations from a sealed CHR ruleset and executes their bag join directly. It preserves the checked source answers, including duplicate derivations, repeated arguments and unknown aliases. This establishes a source-derived alternative to rule execution; its preparation and execution costs remain unmeasured.

## What the compiler accepts

A request rule consumes one occurrence and posts finite table-choice calls. Each table rule chooses among ground tuples through explicit OR and binds its arguments through equality. Calls may share variables, repeat arguments, contain structured unknowns or givens, and introduce fresh request-local variables. Query constraints without a matching source rule remain passive residuals.

The compiler checks the whole supplied ruleset. Its current certificate requires distinct variable head arguments, one consuming head per rule, no guards, no competing predicate definitions, no recursive request calls and no other source effects. Table rows assign each head argument exactly once to a ground finite term. Empty tables and zero-arity rows are supported.

These are the implemented certificate's premises, not a proposal to require finite domains or ground inputs throughout the language. A harmless guard or a more general safe source may still be rejected. Broader eligibility and the cost of boundaries around compiled regions remain S06/S07 questions.

## Why direct joins preserve this fragment

Every table-call occurrence chooses a row occurrence, so its semantics is a bag rather than a set. Duplicate equal rows create separate successful derivations. A successful source execution is exactly a combination of row occurrences whose ground assignments agree across all shared arguments. Incompatible assignments fail, including assignments to variables absent from selected outputs.

The admitted source has no competing consumers, observers or recursion. Request expansion introduces fresh local variables, and table calls add only finite equalities. Consequently the order of these calls can change failed intermediate work but cannot change the successful bag of complete answers. The direct path may choose the most selective available table before another call while retaining every compatible row occurrence.

The implementation binds variables only to ground row terms and recursively checks structured arguments. Shared query variables therefore constrain multiple positions jointly; arbitrary unification is not substituted for source head matching. Unselected unknowns and passive residuals use the same query-variable identities in the answer. This reasoning is specific to the accepted fragment; it does not justify reordering general consuming CHR regions.

## Independent evidence

The main gate exhausts both binary tables' subsets over four possible rows, optionally duplicates a left row, and varies whether the first and third request arguments are aliased. That gives **1,024 table/query configurations**. Each is checked under all six source rule orders: **6,144 comparisons per executor**.

An independent mathematical oracle enumerates row-index products and filters agreement, retaining duplicate products. The owned-syntax scalar evaluator, direct join engine, generic dedicated scanned engine and generic dedicated indexed engine all match its complete raw observations. The same prepared direct rules serve changed alias queries. No reference code or evaluator implementation was changed.

Ten directed queries additionally cover constructor-valued rows, structured unknown arguments, contradictory givens, fresh locals, independent and aliased repeated requests, direct table calls, off-output failure, unmatched signatures and empty queries. A zero-arity example confirms four raw answers from two independent binary choices despite identical empty observations.

A separate lazy-enumeration test publishes the first answer of a ten-factor, twenty-row product within 25 advances, then confirms the search remains live. It does not enumerate the entire 20^10 product or claim that output production is free. The engine retains a depth-first continuation for each selected table rather than materializing every combination before the first answer.

Eleven source near misses and a missing entry are rejected. Deliberately collapsing duplicate rows or ignoring an existing variable binding causes the mathematical/source gate to fail. These checks protect actual multiplicity and alias semantics, rather than only compiler labels. Tests, replay, strict Clippy and formatting pass within the registered process bound.

## What this organization costs and avoids

Preparation owns source inspection, table extraction and column indexes. Query setup expands request locals and separates table calls from passive residuals. Execution owns partial ground substitutions and row continuations. Observation reconstructs outputs and residuals from those substitutions.

This path has no CHR occurrence store, activation queue, propagation history or interpreter-trace encoding. It still clones partial bindings, materializes candidate row lists while selecting the next table, retains duplicate rows and stores enough continuation state to preserve raw enumeration. Those are actual implementation costs for the next comparison, not evidence of an efficiency advantage.

R04's finite consistency solver is a relevant control with a different representation: three-valued variables and forbidden binary pairs. It does not directly accept arbitrary-arity tables, constructor-valued fields or duplicate row derivations. A comparison must account for translation and multiplicity instead of treating that solver as an interchangeable engine. The generic dedicated controls already pass the full main gate and provide immediately usable source execution comparisons.

## Next decision

T064 remains active for a prospective lifecycle comparison. Include selective and overlapping relations, duplicate-heavy output, contradictory givens, disconnected products, tiny queries and reuse of source preparation. Charge certificate checking, relation construction, changing queries, first/full observation and disposal. Investigate any consequential cutoff or preparation crossover; native compilation is a separate cost question.

The [registration](../registrations/S06-table-source-gate.md), [compiler and join engine](../../../research/chr-direct-relation/src/lib.rs), [independent gate](../../../research/chr-direct-relation/tests/source.rs) and [validation receipts](s06-table-source/validation.json) provide the exact scope and commands. This gate establishes correctness on the registered fragment. It neither selects an architecture nor resolves broader relational compilation, contextual recursion, structural spaces or necessary language restrictions.
