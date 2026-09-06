# Cost model and research boundary

The construction now identifies what work is saved and what work may replace it. Actual representations determine the balance, so a bounded implementation is the next source of evidence. These cost expressions are accounting models, not measured bounds on a production evaluator.

## Separate semantic work from bookkeeping

Let B be live frontier alternatives, N current occurrences, h a rule's head count, W the common expansion chain length, G the represented term/binding graph size, and K the Boolean-condition graph size. The meaning of each quantity must be tied to an instrumented event or state count in the experiment manifest.

Independent execution of the registered common chain performs B×W expansions; a correctly grouped symbolic execution can perform W. Both may still enumerate B answers and process B scheduling tickets. Thus W symbolic expansions do not establish runtime or memory independent of B.

A useful total-work decomposition is:

```text
matching + unification + source expansion
  + condition manipulation + dependency maintenance
  + scheduling/commit validation + answer processing
```

Compare the actual sum against the independent controls. Do not call condition or wake-up work free because it is not a rule firing. Conversely, a condition operation shared across alternatives must not be charged B times merely because its projection has B meanings.

## Adverse dimensions

Coarse matching may inspect a product of predicate occurrence counts for an h-headed rule. Selectivity and shared retained heads determine how much survives. A source restriction to single-headed expansion removes multiheaded gathering obligations, but a good general compiler can already specialize some such rules. The R experiment must not presume an advantage over a control that performs equivalent optimization.

Conditional structural matching visits node pairs under supports. Alias changes can invalidate those visits and create subscription traffic. Long shared paths can save term storage while increasing invalidation fan-out. Finite terms prevent projected cycles; they do not limit the number of distinct conditional environments or guarantee small occurs-check supports.

Boolean Apply costs depend on its operand graphs, not merely the number of source choices. For reduced ordered decision diagrams, the classic binary-operation construction has a product-of-operand-sizes bound, while represented graphs can be exponential and sensitive to variable order. The experiment fixes one order and reports graph sizes; it does not silently retune order after observing favorable cases. [Bryant, §§2–4](https://www.cs.cmu.edu/~bryant/pubdir/ieeetc86.pdf)

Per-frontier tickets have at least B records in the specified correctness scheduler. A representation that merges jobs can reduce source work while still paying for those tickets. Compact ticket scheduling is a separate research/implementation question and must not be inferred from the grouped-operation result.

Answer output has unavoidable size cost in the number and size of answers actually requested. Incremental equivalence checking can also grow with prior answers; structural hashing is only a candidate index, not proof of semantic equivalence. Synthesis may have infinitely many distinct program terms with the requested behavior, so neither deduplication nor output normalization can assume finite completion of the search.

## Decision-relevant outcomes

The proposed probe can show a missed wake-up, incorrect projection, duplicated common expansion, unexpectedly large condition graphs, or excessive subscription traffic. Each result changes which invariant or operation needs work. It can also show the intended symbolic operation count while exposing overhead that prevents a useful runtime claim.

The probe cannot rank production languages, decide that conditional stores are preferable to nets, or establish representative speed from synthetic chains. A favorable operation count justifies investigating this candidate's costs further. An unfavorable result may motivate local source properties, alternative support representations, or another mechanism; it does not imply that the owner's desired semantics is impossible.

## Remaining source and candidate disposition

The survey includes token-aware CHR, conditional contexts/ATMS, named superpositions and net calculi, memoized functional-logic graphs, storage/recomputation, tabling and learned consequences. Source records distinguish primary definitions, proof assumptions, experiments, implementation inspection and proposed transfers. No family is excluded merely because it differs from the baseline.

The recovered CW447 definition resolves the reference-composition dependency. Full TCLP resolves the projection/consumer-filtering dependency for the stated solver interface. Historical 1997/1998 access gaps and incomplete broad citation inventories remain documented. They do not supply an unmet premise of the registered probe.

Béchet's uninspected total-correctness conditions prohibit relying on those behavioral transformations; the probe uses none. Deferred resource/static-analysis sources become required if a future backend uses their specific certificates. Their existence does not establish that more general searching will settle this experiment's subscription counts, Boolean graph sizes or scheduling overhead.

Relation-graph and net backends remain live alternatives, with equality/dependency/service engineering explicitly identified. The scalar/conditional services and R-controller probe provide reusable correctness questions, but their results will not be called net compilation evidence. Tabled and learned solver interfaces likewise have an independent scope and must preserve their declared residual meaning.

## Why implementation is now needed

The pending question is no longer just whether applicability or fair work discovery can be defined. The dossier now supplies finite tuple scans, conditional witness construction, race-safe dependency registration, supported commits, a resumable FIFO scheduler and quiescence certificates. Their costs and implementation fidelity depend on actual data structures and workloads.

Further theoretical refinements are possible, but none identified by the review is a prerequisite for challenging this specified construction. The appropriate research stopping point is an implementation gate with a frozen experiment contract, not a claim that every research direction is exhausted. The broader language project continues through experiment review and later owner decisions.
